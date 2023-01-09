impl ::core::default::Default for ALTERNATE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ALTERNATE_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceNumber == other.InterfaceNumber && self.AlternateInterfaceNumber == other.AlternateInterfaceNumber
    }
}
impl ::core::cmp::Eq for ALTERNATE_INTERFACE {}
impl ::core::fmt::Debug for ALTERNATE_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ALTERNATE_INTERFACE").field("InterfaceNumber", &self.InterfaceNumber).field("AlternateInterfaceNumber", &self.AlternateInterfaceNumber).finish()
    }
}
impl ::core::default::Default for BM_REQUEST_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CHANNEL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANNEL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EventChannelSize == other.EventChannelSize && self.uReadDataAlignment == other.uReadDataAlignment && self.uWriteDataAlignment == other.uWriteDataAlignment
    }
}
impl ::core::cmp::Eq for CHANNEL_INFO {}
impl ::core::fmt::Debug for CHANNEL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANNEL_INFO").field("EventChannelSize", &self.EventChannelSize).field("uReadDataAlignment", &self.uReadDataAlignment).field("uWriteDataAlignment", &self.uWriteDataAlignment).finish()
    }
}
impl ::core::default::Default for DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.usVendorId == other.usVendorId && self.usProductId == other.usProductId && self.usBcdDevice == other.usBcdDevice && self.usLanguageId == other.usLanguageId
    }
}
impl ::core::cmp::Eq for DEVICE_DESCRIPTOR {}
impl ::core::fmt::Debug for DEVICE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DESCRIPTOR").field("usVendorId", &self.usVendorId).field("usProductId", &self.usProductId).field("usBcdDevice", &self.usBcdDevice).field("usLanguageId", &self.usLanguageId).finish()
    }
}
impl ::core::default::Default for DRV_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRV_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.internal == other.internal
    }
}
impl ::core::cmp::Eq for DRV_VERSION {}
impl ::core::fmt::Debug for DRV_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRV_VERSION").field("major", &self.major).field("minor", &self.minor).field("internal", &self.internal).finish()
    }
}
impl ::core::default::Default for IO_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IO_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        self.uOffset == other.uOffset && self.uLength == other.uLength && self.pbyData == other.pbyData && self.uIndex == other.uIndex
    }
}
impl ::core::cmp::Eq for IO_BLOCK {}
impl ::core::fmt::Debug for IO_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_BLOCK").field("uOffset", &self.uOffset).field("uLength", &self.uLength).field("pbyData", &self.pbyData).field("uIndex", &self.uIndex).finish()
    }
}
impl ::core::default::Default for IO_BLOCK_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IO_BLOCK_EX {
    fn eq(&self, other: &Self) -> bool {
        self.uOffset == other.uOffset && self.uLength == other.uLength && self.pbyData == other.pbyData && self.uIndex == other.uIndex && self.bRequest == other.bRequest && self.bmRequestType == other.bmRequestType && self.fTransferDirectionIn == other.fTransferDirectionIn
    }
}
impl ::core::cmp::Eq for IO_BLOCK_EX {}
impl ::core::fmt::Debug for IO_BLOCK_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_BLOCK_EX").field("uOffset", &self.uOffset).field("uLength", &self.uLength).field("pbyData", &self.pbyData).field("uIndex", &self.uIndex).field("bRequest", &self.bRequest).field("bmRequestType", &self.bmRequestType).field("fTransferDirectionIn", &self.fTransferDirectionIn).finish()
    }
}
impl ::core::default::Default for OS_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PACKET_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PIPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PIPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PIPE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RAW_PIPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RAW_PIPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAW_PIPE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RAW_RESET_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RAW_ROOTPORT_FEATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for RAW_ROOTPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for URB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBD_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBD_ENDPOINT_OFFLOAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBD_ENDPOINT_OFFLOAD_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBD_ENDPOINT_OFFLOAD_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBD_ENDPOINT_OFFLOAD_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USBD_INTERFACE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBD_INTERFACE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceNumber == other.InterfaceNumber && self.AlternateSetting == other.AlternateSetting && self.Class == other.Class && self.SubClass == other.SubClass && self.Protocol == other.Protocol && self.Reserved == other.Reserved && self.InterfaceHandle == other.InterfaceHandle && self.NumberOfPipes == other.NumberOfPipes && self.Pipes == other.Pipes
    }
}
impl ::core::cmp::Eq for USBD_INTERFACE_INFORMATION {}
impl ::core::fmt::Debug for USBD_INTERFACE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBD_INTERFACE_INFORMATION").field("Length", &self.Length).field("InterfaceNumber", &self.InterfaceNumber).field("AlternateSetting", &self.AlternateSetting).field("Class", &self.Class).field("SubClass", &self.SubClass).field("Protocol", &self.Protocol).field("Reserved", &self.Reserved).field("InterfaceHandle", &self.InterfaceHandle).field("NumberOfPipes", &self.NumberOfPipes).field("Pipes", &self.Pipes).finish()
    }
}
impl ::core::default::Default for USBD_ISO_PACKET_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBD_ISO_PACKET_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Length == other.Length && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for USBD_ISO_PACKET_DESCRIPTOR {}
impl ::core::fmt::Debug for USBD_ISO_PACKET_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBD_ISO_PACKET_DESCRIPTOR").field("Offset", &self.Offset).field("Length", &self.Length).field("Status", &self.Status).finish()
    }
}
impl ::core::default::Default for USBD_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBD_PIPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumPacketSize == other.MaximumPacketSize && self.EndpointAddress == other.EndpointAddress && self.Interval == other.Interval && self.PipeType == other.PipeType && self.PipeHandle == other.PipeHandle && self.MaximumTransferSize == other.MaximumTransferSize && self.PipeFlags == other.PipeFlags
    }
}
impl ::core::cmp::Eq for USBD_PIPE_INFORMATION {}
impl ::core::fmt::Debug for USBD_PIPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBD_PIPE_INFORMATION").field("MaximumPacketSize", &self.MaximumPacketSize).field("EndpointAddress", &self.EndpointAddress).field("Interval", &self.Interval).field("PipeType", &self.PipeType).field("PipeHandle", &self.PipeHandle).field("MaximumTransferSize", &self.MaximumTransferSize).field("PipeFlags", &self.PipeFlags).finish()
    }
}
impl ::core::default::Default for USBD_PIPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBD_PIPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBD_PIPE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USBD_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBD_STREAM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PipeHandle == other.PipeHandle && self.StreamID == other.StreamID && self.MaximumTransferSize == other.MaximumTransferSize && self.PipeFlags == other.PipeFlags
    }
}
impl ::core::cmp::Eq for USBD_STREAM_INFORMATION {}
impl ::core::fmt::Debug for USBD_STREAM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBD_STREAM_INFORMATION").field("PipeHandle", &self.PipeHandle).field("StreamID", &self.StreamID).field("MaximumTransferSize", &self.MaximumTransferSize).field("PipeFlags", &self.PipeFlags).finish()
    }
}
impl ::core::default::Default for USBD_VERSION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBD_VERSION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.USBDI_Version == other.USBDI_Version && self.Supported_USB_Version == other.Supported_USB_Version
    }
}
impl ::core::cmp::Eq for USBD_VERSION_INFORMATION {}
impl ::core::fmt::Debug for USBD_VERSION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBD_VERSION_INFORMATION").field("USBDI_Version", &self.USBDI_Version).field("Supported_USB_Version", &self.Supported_USB_Version).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USBFN_BUS_CONFIGURATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for USBFN_BUS_CONFIGURATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ConfigurationName == other.ConfigurationName && self.IsCurrent == other.IsCurrent && self.IsActive == other.IsActive
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for USBFN_BUS_CONFIGURATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for USBFN_BUS_CONFIGURATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBFN_BUS_CONFIGURATION_INFO").field("ConfigurationName", &self.ConfigurationName).field("IsCurrent", &self.IsCurrent).field("IsActive", &self.IsActive).finish()
    }
}
impl ::core::default::Default for USBFN_BUS_SPEED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBFN_BUS_SPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBFN_BUS_SPEED").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USBFN_CLASS_INFORMATION_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USBFN_CLASS_INFORMATION_PACKET_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBFN_CLASS_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBFN_CLASS_INTERFACE_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBFN_DEVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBFN_DEVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBFN_DEVICE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USBFN_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBFN_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBFN_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for USBFN_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBFN_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBFN_EVENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for USBFN_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBFN_INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceNumber == other.InterfaceNumber && self.Speed == other.Speed && self.Size == other.Size && self.InterfaceDescriptorSet == other.InterfaceDescriptorSet
    }
}
impl ::core::cmp::Eq for USBFN_INTERFACE_INFO {}
impl ::core::fmt::Debug for USBFN_INTERFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBFN_INTERFACE_INFO").field("InterfaceNumber", &self.InterfaceNumber).field("Speed", &self.Speed).field("Size", &self.Size).field("InterfaceDescriptorSet", &self.InterfaceDescriptorSet).finish()
    }
}
impl ::core::default::Default for USBFN_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBFN_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBFN_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USBFN_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USBFN_PORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for USBFN_USB_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBFN_USB_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.StringIndex == other.StringIndex && self.UsbString == other.UsbString
    }
}
impl ::core::cmp::Eq for USBFN_USB_STRING {}
impl ::core::fmt::Debug for USBFN_USB_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBFN_USB_STRING").field("StringIndex", &self.StringIndex).field("UsbString", &self.UsbString).finish()
    }
}
impl ::core::default::Default for USBSCAN_GET_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBSCAN_GET_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.DescriptorType == other.DescriptorType && self.Index == other.Index && self.LanguageId == other.LanguageId
    }
}
impl ::core::cmp::Eq for USBSCAN_GET_DESCRIPTOR {}
impl ::core::fmt::Debug for USBSCAN_GET_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBSCAN_GET_DESCRIPTOR").field("DescriptorType", &self.DescriptorType).field("Index", &self.Index).field("LanguageId", &self.LanguageId).finish()
    }
}
impl ::core::default::Default for USBSCAN_PIPE_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBSCAN_PIPE_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfPipes == other.NumberOfPipes && self.PipeInfo == other.PipeInfo
    }
}
impl ::core::cmp::Eq for USBSCAN_PIPE_CONFIGURATION {}
impl ::core::fmt::Debug for USBSCAN_PIPE_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBSCAN_PIPE_CONFIGURATION").field("NumberOfPipes", &self.NumberOfPipes).field("PipeInfo", &self.PipeInfo).finish()
    }
}
impl ::core::default::Default for USBSCAN_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBSCAN_PIPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumPacketSize == other.MaximumPacketSize && self.EndpointAddress == other.EndpointAddress && self.Interval == other.Interval && self.PipeType == other.PipeType
    }
}
impl ::core::cmp::Eq for USBSCAN_PIPE_INFORMATION {}
impl ::core::fmt::Debug for USBSCAN_PIPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBSCAN_PIPE_INFORMATION").field("MaximumPacketSize", &self.MaximumPacketSize).field("EndpointAddress", &self.EndpointAddress).field("Interval", &self.Interval).field("PipeType", &self.PipeType).finish()
    }
}
impl ::core::default::Default for USBSCAN_TIMEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USBSCAN_TIMEOUT {
    fn eq(&self, other: &Self) -> bool {
        self.TimeoutRead == other.TimeoutRead && self.TimeoutWrite == other.TimeoutWrite && self.TimeoutEvent == other.TimeoutEvent
    }
}
impl ::core::cmp::Eq for USBSCAN_TIMEOUT {}
impl ::core::fmt::Debug for USBSCAN_TIMEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USBSCAN_TIMEOUT").field("TimeoutRead", &self.TimeoutRead).field("TimeoutWrite", &self.TimeoutWrite).field("TimeoutEvent", &self.TimeoutEvent).finish()
    }
}
impl ::core::default::Default for USBUSER_BANDWIDTH_INFO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USBUSER_BUS_STATISTICS_0_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_CLOSE_RAW_DEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_CONTROLLER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_CONTROLLER_UNICODE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USBUSER_GET_DRIVER_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_GET_USB2HW_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_OPEN_RAW_DEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_PASS_THRU_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USBUSER_POWER_INFO_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_RAW_RESET_ROOT_PORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_REFRESH_HCT_REG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_REQUEST_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_ROOTPORT_FEATURE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_ROOTPORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_SEND_ONE_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USBUSER_SEND_RAW_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_20_PORT_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_20_PORT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_30_HUB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_30_PORT_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_30_PORT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_BANDWIDTH_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_BOS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USB_BUS_STATISTICS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_CLOSE_RAW_DEVICE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_COMMON_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USB_COMMON_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.bLength == other.bLength && self.bDescriptorType == other.bDescriptorType
    }
}
impl ::core::cmp::Eq for USB_COMMON_DESCRIPTOR {}
impl ::core::fmt::Debug for USB_COMMON_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USB_COMMON_DESCRIPTOR").field("bLength", &self.bLength).field("bDescriptorType", &self.bDescriptorType).finish()
    }
}
impl ::core::default::Default for USB_CONFIGURATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_CONFIGURATION_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_CONTROLLER_FLAVOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USB_CONTROLLER_FLAVOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USB_CONTROLLER_FLAVOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for USB_CONTROLLER_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEFAULT_PIPE_SETUP_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_BILLBOARD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.bLength == other.bLength && self.bDescriptorType == other.bDescriptorType && self.bDevCapabilityType == other.bDevCapabilityType && self.bReserved == other.bReserved && self.ContainerID == other.ContainerID
    }
}
impl ::core::cmp::Eq for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {}
impl ::core::fmt::Debug for USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USB_DEVICE_CAPABILITY_CONTAINER_ID_DESCRIPTOR").field("bLength", &self.bLength).field("bDescriptorType", &self.bDescriptorType).field("bDevCapabilityType", &self.bDevCapabilityType).field("bReserved", &self.bReserved).field("ContainerID", &self.ContainerID).finish()
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USB_DEVICE_CAPABILITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.bLength == other.bLength && self.bDescriptorType == other.bDescriptorType && self.bDevCapabilityType == other.bDevCapabilityType
    }
}
impl ::core::cmp::Eq for USB_DEVICE_CAPABILITY_DESCRIPTOR {}
impl ::core::fmt::Debug for USB_DEVICE_CAPABILITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USB_DEVICE_CAPABILITY_DESCRIPTOR").field("bLength", &self.bLength).field("bDescriptorType", &self.bDescriptorType).field("bDevCapabilityType", &self.bDevCapabilityType).finish()
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_FIRMWARE_STATUS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_PD_CONSUMER_PORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_PLATFORM_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_POWER_DELIVERY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_USB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_SUPERSPEED_USB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_CAPABILITY_USB20_EXTENSION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_QUALIFIER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_SPEED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USB_DEVICE_SPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USB_DEVICE_SPEED").field(&self.0).finish()
    }
}
impl ::core::default::Default for USB_DEVICE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USB_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USB_DEVICE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USB_DRIVER_VERSION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_ENDPOINT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_ENDPOINT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_FUNCTION_SUSPEND_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_HIGH_SPEED_MAXPACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_HUB_30_PORT_REMOTE_WAKE_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_HUB_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_HUB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_HUB_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_HUB_STATUS_AND_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_IDLE_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.bLength == other.bLength && self.bDescriptorType == other.bDescriptorType && self.bFirstInterface == other.bFirstInterface && self.bInterfaceCount == other.bInterfaceCount && self.bFunctionClass == other.bFunctionClass && self.bFunctionSubClass == other.bFunctionSubClass && self.bFunctionProtocol == other.bFunctionProtocol && self.iFunction == other.iFunction
    }
}
impl ::core::cmp::Eq for USB_INTERFACE_ASSOCIATION_DESCRIPTOR {}
impl ::core::fmt::Debug for USB_INTERFACE_ASSOCIATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USB_INTERFACE_ASSOCIATION_DESCRIPTOR").field("bLength", &self.bLength).field("bDescriptorType", &self.bDescriptorType).field("bFirstInterface", &self.bFirstInterface).field("bInterfaceCount", &self.bInterfaceCount).field("bFunctionClass", &self.bFunctionClass).field("bFunctionSubClass", &self.bFunctionSubClass).field("bFunctionProtocol", &self.bFunctionProtocol).field("iFunction", &self.iFunction).finish()
    }
}
impl ::core::default::Default for USB_INTERFACE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USB_INTERFACE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.bLength == other.bLength && self.bDescriptorType == other.bDescriptorType && self.bInterfaceNumber == other.bInterfaceNumber && self.bAlternateSetting == other.bAlternateSetting && self.bNumEndpoints == other.bNumEndpoints && self.bInterfaceClass == other.bInterfaceClass && self.bInterfaceSubClass == other.bInterfaceSubClass && self.bInterfaceProtocol == other.bInterfaceProtocol && self.iInterface == other.iInterface
    }
}
impl ::core::cmp::Eq for USB_INTERFACE_DESCRIPTOR {}
impl ::core::fmt::Debug for USB_INTERFACE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USB_INTERFACE_DESCRIPTOR").field("bLength", &self.bLength).field("bDescriptorType", &self.bDescriptorType).field("bInterfaceNumber", &self.bInterfaceNumber).field("bAlternateSetting", &self.bAlternateSetting).field("bNumEndpoints", &self.bNumEndpoints).field("bInterfaceClass", &self.bInterfaceClass).field("bInterfaceSubClass", &self.bInterfaceSubClass).field("bInterfaceProtocol", &self.bInterfaceProtocol).field("iInterface", &self.iInterface).finish()
    }
}
impl ::core::default::Default for USB_INTERFACE_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_INTERFACE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_OPEN_RAW_DEVICE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_PASS_THRU_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_PORT_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_PORT_EXT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_PORT_EXT_STATUS_AND_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_PORT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_PORT_STATUS_AND_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USB_POWER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_SEND_RAW_COMMAND_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_STRING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_SUPERSPEEDPLUS_ISOCH_ENDPOINT_COMPANION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_SUPERSPEED_ENDPOINT_COMPANION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_UNICODE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_USB2HW_VERSION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USB_USER_ERROR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USB_USER_ERROR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USB_USER_ERROR_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WDMUSB_POWER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WDMUSB_POWER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WDMUSB_POWER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WINUSB_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINUSB_PIPE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PipeType == other.PipeType && self.PipeId == other.PipeId && self.MaximumPacketSize == other.MaximumPacketSize && self.Interval == other.Interval
    }
}
impl ::core::cmp::Eq for WINUSB_PIPE_INFORMATION {}
impl ::core::fmt::Debug for WINUSB_PIPE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINUSB_PIPE_INFORMATION").field("PipeType", &self.PipeType).field("PipeId", &self.PipeId).field("MaximumPacketSize", &self.MaximumPacketSize).field("Interval", &self.Interval).finish()
    }
}
impl ::core::default::Default for WINUSB_PIPE_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WINUSB_PIPE_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PipeType == other.PipeType && self.PipeId == other.PipeId && self.MaximumPacketSize == other.MaximumPacketSize && self.Interval == other.Interval && self.MaximumBytesPerInterval == other.MaximumBytesPerInterval
    }
}
impl ::core::cmp::Eq for WINUSB_PIPE_INFORMATION_EX {}
impl ::core::fmt::Debug for WINUSB_PIPE_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WINUSB_PIPE_INFORMATION_EX").field("PipeType", &self.PipeType).field("PipeId", &self.PipeId).field("MaximumPacketSize", &self.MaximumPacketSize).field("Interval", &self.Interval).field("MaximumBytesPerInterval", &self.MaximumBytesPerInterval).finish()
    }
}
impl ::core::default::Default for WINUSB_SETUP_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for _URB_BULK_OR_INTERRUPT_TRANSFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_BULK_OR_INTERRUPT_TRANSFER {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.TransferFlags == other.TransferFlags && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca
    }
}
impl ::core::cmp::Eq for _URB_BULK_OR_INTERRUPT_TRANSFER {}
impl ::core::fmt::Debug for _URB_BULK_OR_INTERRUPT_TRANSFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_BULK_OR_INTERRUPT_TRANSFER").field("Hdr", &self.Hdr).field("PipeHandle", &self.PipeHandle).field("TransferFlags", &self.TransferFlags).field("TransferBufferLength", &self.TransferBufferLength).field("TransferBuffer", &self.TransferBuffer).field("TransferBufferMDL", &self.TransferBufferMDL).field("UrbLink", &self.UrbLink).field("hca", &self.hca).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_DESCRIPTOR_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.Reserved0 == other.Reserved0 && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.Reserved1 == other.Reserved1 && self.Index == other.Index && self.DescriptorType == other.DescriptorType && self.LanguageId == other.LanguageId && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_DESCRIPTOR_REQUEST {}
impl ::core::fmt::Debug for _URB_CONTROL_DESCRIPTOR_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_DESCRIPTOR_REQUEST")
            .field("Hdr", &self.Hdr)
            .field("Reserved", &self.Reserved)
            .field("Reserved0", &self.Reserved0)
            .field("TransferBufferLength", &self.TransferBufferLength)
            .field("TransferBuffer", &self.TransferBuffer)
            .field("TransferBufferMDL", &self.TransferBufferMDL)
            .field("UrbLink", &self.UrbLink)
            .field("hca", &self.hca)
            .field("Reserved1", &self.Reserved1)
            .field("Index", &self.Index)
            .field("DescriptorType", &self.DescriptorType)
            .field("LanguageId", &self.LanguageId)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_FEATURE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_FEATURE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.Reserved4 == other.Reserved4 && self.Reserved5 == other.Reserved5 && self.UrbLink == other.UrbLink && self.hca == other.hca && self.Reserved0 == other.Reserved0 && self.FeatureSelector == other.FeatureSelector && self.Index == other.Index && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_FEATURE_REQUEST {}
impl ::core::fmt::Debug for _URB_CONTROL_FEATURE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_FEATURE_REQUEST").field("Hdr", &self.Hdr).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).field("Reserved5", &self.Reserved5).field("UrbLink", &self.UrbLink).field("hca", &self.hca).field("Reserved0", &self.Reserved0).field("FeatureSelector", &self.FeatureSelector).field("Index", &self.Index).field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.Reserved0 == other.Reserved0 && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_GET_CONFIGURATION_REQUEST {}
impl ::core::fmt::Debug for _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_GET_CONFIGURATION_REQUEST").field("Hdr", &self.Hdr).field("Reserved", &self.Reserved).field("Reserved0", &self.Reserved0).field("TransferBufferLength", &self.TransferBufferLength).field("TransferBuffer", &self.TransferBuffer).field("TransferBufferMDL", &self.TransferBufferMDL).field("UrbLink", &self.UrbLink).field("hca", &self.hca).field("Reserved1", &self.Reserved1).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_GET_INTERFACE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_GET_INTERFACE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.Reserved0 == other.Reserved0 && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.Reserved1 == other.Reserved1 && self.Interface == other.Interface && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_GET_INTERFACE_REQUEST {}
impl ::core::fmt::Debug for _URB_CONTROL_GET_INTERFACE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_GET_INTERFACE_REQUEST").field("Hdr", &self.Hdr).field("Reserved", &self.Reserved).field("Reserved0", &self.Reserved0).field("TransferBufferLength", &self.TransferBufferLength).field("TransferBuffer", &self.TransferBuffer).field("TransferBufferMDL", &self.TransferBufferMDL).field("UrbLink", &self.UrbLink).field("hca", &self.hca).field("Reserved1", &self.Reserved1).field("Interface", &self.Interface).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_GET_STATUS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_GET_STATUS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.Reserved0 == other.Reserved0 && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.Reserved1 == other.Reserved1 && self.Index == other.Index && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_GET_STATUS_REQUEST {}
impl ::core::fmt::Debug for _URB_CONTROL_GET_STATUS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_GET_STATUS_REQUEST").field("Hdr", &self.Hdr).field("Reserved", &self.Reserved).field("Reserved0", &self.Reserved0).field("TransferBufferLength", &self.TransferBufferLength).field("TransferBuffer", &self.TransferBuffer).field("TransferBufferMDL", &self.TransferBufferMDL).field("UrbLink", &self.UrbLink).field("hca", &self.hca).field("Reserved1", &self.Reserved1).field("Index", &self.Index).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_TRANSFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_TRANSFER {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.TransferFlags == other.TransferFlags && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.SetupPacket == other.SetupPacket
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_TRANSFER {}
impl ::core::fmt::Debug for _URB_CONTROL_TRANSFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_TRANSFER").field("Hdr", &self.Hdr).field("PipeHandle", &self.PipeHandle).field("TransferFlags", &self.TransferFlags).field("TransferBufferLength", &self.TransferBufferLength).field("TransferBuffer", &self.TransferBuffer).field("TransferBufferMDL", &self.TransferBufferMDL).field("UrbLink", &self.UrbLink).field("hca", &self.hca).field("SetupPacket", &self.SetupPacket).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_TRANSFER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_TRANSFER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.TransferFlags == other.TransferFlags && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.Timeout == other.Timeout && self.hca == other.hca && self.SetupPacket == other.SetupPacket
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_TRANSFER_EX {}
impl ::core::fmt::Debug for _URB_CONTROL_TRANSFER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_TRANSFER_EX").field("Hdr", &self.Hdr).field("PipeHandle", &self.PipeHandle).field("TransferFlags", &self.TransferFlags).field("TransferBufferLength", &self.TransferBufferLength).field("TransferBuffer", &self.TransferBuffer).field("TransferBufferMDL", &self.TransferBufferMDL).field("Timeout", &self.Timeout).field("hca", &self.hca).field("SetupPacket", &self.SetupPacket).finish()
    }
}
impl ::core::default::Default for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.TransferFlags == other.TransferFlags && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.RequestTypeReservedBits == other.RequestTypeReservedBits && self.Request == other.Request && self.Value == other.Value && self.Index == other.Index && self.Reserved1 == other.Reserved1
    }
}
impl ::core::cmp::Eq for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {}
impl ::core::fmt::Debug for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_CONTROL_VENDOR_OR_CLASS_REQUEST")
            .field("Hdr", &self.Hdr)
            .field("Reserved", &self.Reserved)
            .field("TransferFlags", &self.TransferFlags)
            .field("TransferBufferLength", &self.TransferBufferLength)
            .field("TransferBuffer", &self.TransferBuffer)
            .field("TransferBufferMDL", &self.TransferBufferMDL)
            .field("UrbLink", &self.UrbLink)
            .field("hca", &self.hca)
            .field("RequestTypeReservedBits", &self.RequestTypeReservedBits)
            .field("Request", &self.Request)
            .field("Value", &self.Value)
            .field("Index", &self.Index)
            .field("Reserved1", &self.Reserved1)
            .finish()
    }
}
impl ::core::default::Default for _URB_FRAME_LENGTH_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_FRAME_LENGTH_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr
    }
}
impl ::core::cmp::Eq for _URB_FRAME_LENGTH_CONTROL {}
impl ::core::fmt::Debug for _URB_FRAME_LENGTH_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_FRAME_LENGTH_CONTROL").field("Hdr", &self.Hdr).finish()
    }
}
impl ::core::default::Default for _URB_GET_CURRENT_FRAME_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_GET_CURRENT_FRAME_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.FrameNumber == other.FrameNumber
    }
}
impl ::core::cmp::Eq for _URB_GET_CURRENT_FRAME_NUMBER {}
impl ::core::fmt::Debug for _URB_GET_CURRENT_FRAME_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_GET_CURRENT_FRAME_NUMBER").field("Hdr", &self.Hdr).field("FrameNumber", &self.FrameNumber).finish()
    }
}
impl ::core::default::Default for _URB_GET_FRAME_LENGTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_GET_FRAME_LENGTH {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.FrameLength == other.FrameLength && self.FrameNumber == other.FrameNumber
    }
}
impl ::core::cmp::Eq for _URB_GET_FRAME_LENGTH {}
impl ::core::fmt::Debug for _URB_GET_FRAME_LENGTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_GET_FRAME_LENGTH").field("Hdr", &self.Hdr).field("FrameLength", &self.FrameLength).field("FrameNumber", &self.FrameNumber).finish()
    }
}
impl ::core::default::Default for _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.MaximumSendPathDelayInMilliSeconds == other.MaximumSendPathDelayInMilliSeconds && self.MaximumCompletionPathDelayInMilliSeconds == other.MaximumCompletionPathDelayInMilliSeconds
    }
}
impl ::core::cmp::Eq for _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {}
impl ::core::fmt::Debug for _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS").field("Hdr", &self.Hdr).field("PipeHandle", &self.PipeHandle).field("MaximumSendPathDelayInMilliSeconds", &self.MaximumSendPathDelayInMilliSeconds).field("MaximumCompletionPathDelayInMilliSeconds", &self.MaximumCompletionPathDelayInMilliSeconds).finish()
    }
}
impl ::core::default::Default for _URB_HCD_AREA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_HCD_AREA {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved8 == other.Reserved8
    }
}
impl ::core::cmp::Eq for _URB_HCD_AREA {}
impl ::core::fmt::Debug for _URB_HCD_AREA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_HCD_AREA").field("Reserved8", &self.Reserved8).finish()
    }
}
impl ::core::default::Default for _URB_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Function == other.Function && self.Status == other.Status && self.UsbdDeviceHandle == other.UsbdDeviceHandle && self.UsbdFlags == other.UsbdFlags
    }
}
impl ::core::cmp::Eq for _URB_HEADER {}
impl ::core::fmt::Debug for _URB_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_HEADER").field("Length", &self.Length).field("Function", &self.Function).field("Status", &self.Status).field("UsbdDeviceHandle", &self.UsbdDeviceHandle).field("UsbdFlags", &self.UsbdFlags).finish()
    }
}
impl ::core::default::Default for _URB_ISOCH_TRANSFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_ISOCH_TRANSFER {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.TransferFlags == other.TransferFlags && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self.StartFrame == other.StartFrame && self.NumberOfPackets == other.NumberOfPackets && self.ErrorCount == other.ErrorCount && self.IsoPacket == other.IsoPacket
    }
}
impl ::core::cmp::Eq for _URB_ISOCH_TRANSFER {}
impl ::core::fmt::Debug for _URB_ISOCH_TRANSFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_ISOCH_TRANSFER")
            .field("Hdr", &self.Hdr)
            .field("PipeHandle", &self.PipeHandle)
            .field("TransferFlags", &self.TransferFlags)
            .field("TransferBufferLength", &self.TransferBufferLength)
            .field("TransferBuffer", &self.TransferBuffer)
            .field("TransferBufferMDL", &self.TransferBufferMDL)
            .field("UrbLink", &self.UrbLink)
            .field("hca", &self.hca)
            .field("StartFrame", &self.StartFrame)
            .field("NumberOfPackets", &self.NumberOfPackets)
            .field("ErrorCount", &self.ErrorCount)
            .field("IsoPacket", &self.IsoPacket)
            .finish()
    }
}
impl ::core::default::Default for _URB_OPEN_STATIC_STREAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_OPEN_STATIC_STREAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.NumberOfStreams == other.NumberOfStreams && self.StreamInfoVersion == other.StreamInfoVersion && self.StreamInfoSize == other.StreamInfoSize && self.Streams == other.Streams
    }
}
impl ::core::cmp::Eq for _URB_OPEN_STATIC_STREAMS {}
impl ::core::fmt::Debug for _URB_OPEN_STATIC_STREAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_OPEN_STATIC_STREAMS").field("Hdr", &self.Hdr).field("PipeHandle", &self.PipeHandle).field("NumberOfStreams", &self.NumberOfStreams).field("StreamInfoVersion", &self.StreamInfoVersion).field("StreamInfoSize", &self.StreamInfoSize).field("Streams", &self.Streams).finish()
    }
}
impl ::core::default::Default for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.Reserved == other.Reserved && self.Reserved0 == other.Reserved0 && self.TransferBufferLength == other.TransferBufferLength && self.TransferBuffer == other.TransferBuffer && self.TransferBufferMDL == other.TransferBufferMDL && self.UrbLink == other.UrbLink && self.hca == other.hca && self._bitfield == other._bitfield && self.Reserved2 == other.Reserved2 && self.InterfaceNumber == other.InterfaceNumber && self.MS_PageIndex == other.MS_PageIndex && self.MS_FeatureDescriptorIndex == other.MS_FeatureDescriptorIndex && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {}
impl ::core::fmt::Debug for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_OS_FEATURE_DESCRIPTOR_REQUEST")
            .field("Hdr", &self.Hdr)
            .field("Reserved", &self.Reserved)
            .field("Reserved0", &self.Reserved0)
            .field("TransferBufferLength", &self.TransferBufferLength)
            .field("TransferBuffer", &self.TransferBuffer)
            .field("TransferBufferMDL", &self.TransferBufferMDL)
            .field("UrbLink", &self.UrbLink)
            .field("hca", &self.hca)
            .field("_bitfield", &self._bitfield)
            .field("Reserved2", &self.Reserved2)
            .field("InterfaceNumber", &self.InterfaceNumber)
            .field("MS_PageIndex", &self.MS_PageIndex)
            .field("MS_FeatureDescriptorIndex", &self.MS_FeatureDescriptorIndex)
            .field("Reserved3", &self.Reserved3)
            .finish()
    }
}
impl ::core::default::Default for _URB_PIPE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_PIPE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.PipeHandle == other.PipeHandle && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for _URB_PIPE_REQUEST {}
impl ::core::fmt::Debug for _URB_PIPE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_PIPE_REQUEST").field("Hdr", &self.Hdr).field("PipeHandle", &self.PipeHandle).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for _URB_SELECT_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_SELECT_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.ConfigurationDescriptor == other.ConfigurationDescriptor && self.ConfigurationHandle == other.ConfigurationHandle && self.Interface == other.Interface
    }
}
impl ::core::cmp::Eq for _URB_SELECT_CONFIGURATION {}
impl ::core::fmt::Debug for _URB_SELECT_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_SELECT_CONFIGURATION").field("Hdr", &self.Hdr).field("ConfigurationDescriptor", &self.ConfigurationDescriptor).field("ConfigurationHandle", &self.ConfigurationHandle).field("Interface", &self.Interface).finish()
    }
}
impl ::core::default::Default for _URB_SELECT_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_SELECT_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.ConfigurationHandle == other.ConfigurationHandle && self.Interface == other.Interface
    }
}
impl ::core::cmp::Eq for _URB_SELECT_INTERFACE {}
impl ::core::fmt::Debug for _URB_SELECT_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_SELECT_INTERFACE").field("Hdr", &self.Hdr).field("ConfigurationHandle", &self.ConfigurationHandle).field("Interface", &self.Interface).finish()
    }
}
impl ::core::default::Default for _URB_SET_FRAME_LENGTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for _URB_SET_FRAME_LENGTH {
    fn eq(&self, other: &Self) -> bool {
        self.Hdr == other.Hdr && self.FrameLengthDelta == other.FrameLengthDelta
    }
}
impl ::core::cmp::Eq for _URB_SET_FRAME_LENGTH {}
impl ::core::fmt::Debug for _URB_SET_FRAME_LENGTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_URB_SET_FRAME_LENGTH").field("Hdr", &self.Hdr).field("FrameLengthDelta", &self.FrameLengthDelta).finish()
    }
}
