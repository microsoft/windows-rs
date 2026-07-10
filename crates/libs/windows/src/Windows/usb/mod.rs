pub const EHCI_Generic: USB_CONTROLLER_FLAVOR = 1000;
pub const EHCI_Intel_Medfield: USB_CONTROLLER_FLAVOR = 5001;
pub const EHCI_Lucent: USB_CONTROLLER_FLAVOR = 3000;
pub const EHCI_NEC: USB_CONTROLLER_FLAVOR = 2000;
pub const EHCI_NVIDIA_Tegra2: USB_CONTROLLER_FLAVOR = 4000;
pub const EHCI_NVIDIA_Tegra3: USB_CONTROLLER_FLAVOR = 4001;
pub const MS_GENRE_DESCRIPTOR_INDEX: u32 = 1;
pub const MS_OS_FLAGS_CONTAINERID: u32 = 2;
pub const MS_OS_STRING_SIGNATURE: windows_core::PCWSTR = windows_core::w!("MSFT100");
pub const MS_POWER_DESCRIPTOR_INDEX: u32 = 2;
pub const OHCI_Generic: USB_CONTROLLER_FLAVOR = 100;
pub const OHCI_Hydra: USB_CONTROLLER_FLAVOR = 101;
pub const OHCI_NEC: USB_CONTROLLER_FLAVOR = 102;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OS_STRING {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub MicrosoftString: [u16; 7],
    pub bVendorCode: u8,
    pub Anonymous: OS_STRING_0,
}
impl Default for OS_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OS_STRING_0 {
    pub bPad: u8,
    pub bFlags: u8,
}
impl Default for OS_STRING_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OS_STRING_DESCRIPTOR_INDEX: u32 = 238;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PHYSICAL_ADDRESS(pub i64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIRP(pub *mut core::ffi::c_void);
impl PIRP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIRP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMDL(pub *mut core::ffi::c_void);
impl PMDL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMDL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POS_STRING(pub *mut OS_STRING);
impl POS_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POS_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PURB(pub *mut URB);
#[cfg(feature = "usbspec")]
impl PURB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "usbspec")]
impl Default for PURB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "usbspec")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_DEVICE_INFORMATION(pub *mut USBD_DEVICE_INFORMATION);
#[cfg(feature = "usbspec")]
impl PUSBD_DEVICE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "usbspec")]
impl Default for PUSBD_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_ENDPOINT_OFFLOAD_INFORMATION(pub *mut USBD_ENDPOINT_OFFLOAD_INFORMATION);
impl PUSBD_ENDPOINT_OFFLOAD_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_ENDPOINT_OFFLOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V1(pub *mut USBD_ENDPOINT_OFFLOAD_INFORMATION_V1);
impl PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V2(pub *mut USBD_ENDPOINT_OFFLOAD_INFORMATION);
impl PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_INTERFACE_INFORMATION(pub *mut USBD_INTERFACE_INFORMATION);
impl PUSBD_INTERFACE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_INTERFACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_ISO_PACKET_DESCRIPTOR(pub *mut USBD_ISO_PACKET_DESCRIPTOR);
impl PUSBD_ISO_PACKET_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_ISO_PACKET_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_PIPE_INFORMATION(pub *mut USBD_PIPE_INFORMATION);
impl PUSBD_PIPE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_PIPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_STREAM_INFORMATION(pub *mut USBD_STREAM_INFORMATION);
impl PUSBD_STREAM_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_STREAM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSBD_VERSION_INFORMATION(pub *mut USBD_VERSION_INFORMATION);
impl PUSBD_VERSION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSBD_VERSION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UHCI_Generic: USB_CONTROLLER_FLAVOR = 200;
pub const UHCI_Ich1: USB_CONTROLLER_FLAVOR = 205;
pub const UHCI_Ich2: USB_CONTROLLER_FLAVOR = 203;
pub const UHCI_Ich3m: USB_CONTROLLER_FLAVOR = 206;
pub const UHCI_Ich4: USB_CONTROLLER_FLAVOR = 207;
pub const UHCI_Ich5: USB_CONTROLLER_FLAVOR = 208;
pub const UHCI_Ich6: USB_CONTROLLER_FLAVOR = 209;
pub const UHCI_Intel: USB_CONTROLLER_FLAVOR = 249;
pub const UHCI_Piix3: USB_CONTROLLER_FLAVOR = 202;
pub const UHCI_Piix4: USB_CONTROLLER_FLAVOR = 201;
pub const UHCI_Reserved204: USB_CONTROLLER_FLAVOR = 204;
pub const UHCI_VIA: USB_CONTROLLER_FLAVOR = 250;
pub const UHCI_VIA_x01: USB_CONTROLLER_FLAVOR = 251;
pub const UHCI_VIA_x02: USB_CONTROLLER_FLAVOR = 252;
pub const UHCI_VIA_x03: USB_CONTROLLER_FLAVOR = 253;
pub const UHCI_VIA_x04: USB_CONTROLLER_FLAVOR = 254;
pub const UHCI_VIA_x0E_FIFO: USB_CONTROLLER_FLAVOR = 264;
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy)]
pub struct URB {
    pub Anonymous: URB_0,
}
#[cfg(feature = "usbspec")]
impl Default for URB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy)]
pub union URB_0 {
    pub UrbHeader: _URB_HEADER,
    pub UrbSelectInterface: _URB_SELECT_INTERFACE,
    pub UrbSelectConfiguration: _URB_SELECT_CONFIGURATION,
    pub UrbPipeRequest: _URB_PIPE_REQUEST,
    pub UrbFrameLengthControl: _URB_FRAME_LENGTH_CONTROL,
    pub UrbGetFrameLength: _URB_GET_FRAME_LENGTH,
    pub UrbSetFrameLength: _URB_SET_FRAME_LENGTH,
    pub UrbGetCurrentFrameNumber: _URB_GET_CURRENT_FRAME_NUMBER,
    pub UrbControlTransfer: _URB_CONTROL_TRANSFER,
    pub UrbControlTransferEx: _URB_CONTROL_TRANSFER_EX,
    pub UrbBulkOrInterruptTransfer: _URB_BULK_OR_INTERRUPT_TRANSFER,
    pub UrbIsochronousTransfer: _URB_ISOCH_TRANSFER,
    pub UrbControlDescriptorRequest: _URB_CONTROL_DESCRIPTOR_REQUEST,
    pub UrbControlGetStatusRequest: _URB_CONTROL_GET_STATUS_REQUEST,
    pub UrbControlFeatureRequest: _URB_CONTROL_FEATURE_REQUEST,
    pub UrbControlVendorClassRequest: _URB_CONTROL_VENDOR_OR_CLASS_REQUEST,
    pub UrbControlGetInterfaceRequest: _URB_CONTROL_GET_INTERFACE_REQUEST,
    pub UrbControlGetConfigurationRequest: _URB_CONTROL_GET_CONFIGURATION_REQUEST,
    pub UrbOSFeatureDescriptorRequest: _URB_OS_FEATURE_DESCRIPTOR_REQUEST,
    pub UrbOpenStaticStreams: _URB_OPEN_STATIC_STREAMS,
    pub UrbGetIsochPipeTransferPathDelays: _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS,
}
#[cfg(feature = "usbspec")]
impl Default for URB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const URB_FUNCTION_ABORT_PIPE: u32 = 2;
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER: u32 = 9;
pub const URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER_USING_CHAINED_MDL: u32 = 55;
pub const URB_FUNCTION_CLASS_DEVICE: u32 = 26;
pub const URB_FUNCTION_CLASS_ENDPOINT: u32 = 28;
pub const URB_FUNCTION_CLASS_INTERFACE: u32 = 27;
pub const URB_FUNCTION_CLASS_OTHER: u32 = 31;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_DEVICE: u32 = 16;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_ENDPOINT: u32 = 18;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_INTERFACE: u32 = 17;
pub const URB_FUNCTION_CLEAR_FEATURE_TO_OTHER: u32 = 34;
pub const URB_FUNCTION_CLOSE_STATIC_STREAMS: u32 = 54;
pub const URB_FUNCTION_CONTROL_TRANSFER: u32 = 8;
pub const URB_FUNCTION_CONTROL_TRANSFER_EX: u32 = 50;
pub const URB_FUNCTION_GET_CONFIGURATION: u32 = 38;
pub const URB_FUNCTION_GET_CURRENT_FRAME_NUMBER: u32 = 7;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_DEVICE: u32 = 11;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_ENDPOINT: u32 = 36;
pub const URB_FUNCTION_GET_DESCRIPTOR_FROM_INTERFACE: u32 = 40;
pub const URB_FUNCTION_GET_FRAME_LENGTH: u32 = 5;
pub const URB_FUNCTION_GET_INTERFACE: u32 = 39;
pub const URB_FUNCTION_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS: u32 = 61;
pub const URB_FUNCTION_GET_MS_FEATURE_DESCRIPTOR: u32 = 42;
pub const URB_FUNCTION_GET_STATUS_FROM_DEVICE: u32 = 19;
pub const URB_FUNCTION_GET_STATUS_FROM_ENDPOINT: u32 = 21;
pub const URB_FUNCTION_GET_STATUS_FROM_INTERFACE: u32 = 20;
pub const URB_FUNCTION_GET_STATUS_FROM_OTHER: u32 = 33;
pub const URB_FUNCTION_ISOCH_TRANSFER: u32 = 10;
pub const URB_FUNCTION_ISOCH_TRANSFER_USING_CHAINED_MDL: u32 = 56;
pub const URB_FUNCTION_OPEN_STATIC_STREAMS: u32 = 53;
pub const URB_FUNCTION_RELEASE_FRAME_LENGTH_CONTROL: u32 = 4;
pub const URB_FUNCTION_RESERVED_0X0016: u32 = 22;
pub const URB_FUNCTION_RESERVE_0X001D: u32 = 29;
pub const URB_FUNCTION_RESERVE_0X002B: u32 = 43;
pub const URB_FUNCTION_RESERVE_0X002C: u32 = 44;
pub const URB_FUNCTION_RESERVE_0X002D: u32 = 45;
pub const URB_FUNCTION_RESERVE_0X002E: u32 = 46;
pub const URB_FUNCTION_RESERVE_0X002F: u32 = 47;
pub const URB_FUNCTION_RESERVE_0X0033: u32 = 51;
pub const URB_FUNCTION_RESERVE_0X0034: u32 = 52;
pub const URB_FUNCTION_RESET_PIPE: u32 = 30;
pub const URB_FUNCTION_SELECT_CONFIGURATION: u32 = 0;
pub const URB_FUNCTION_SELECT_INTERFACE: u32 = 1;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_DEVICE: u32 = 12;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_ENDPOINT: u32 = 37;
pub const URB_FUNCTION_SET_DESCRIPTOR_TO_INTERFACE: u32 = 41;
pub const URB_FUNCTION_SET_FEATURE_TO_DEVICE: u32 = 13;
pub const URB_FUNCTION_SET_FEATURE_TO_ENDPOINT: u32 = 15;
pub const URB_FUNCTION_SET_FEATURE_TO_INTERFACE: u32 = 14;
pub const URB_FUNCTION_SET_FEATURE_TO_OTHER: u32 = 35;
pub const URB_FUNCTION_SET_FRAME_LENGTH: u32 = 6;
pub const URB_FUNCTION_SYNC_CLEAR_STALL: u32 = 49;
pub const URB_FUNCTION_SYNC_RESET_PIPE: u32 = 48;
pub const URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL: u32 = 30;
pub const URB_FUNCTION_TAKE_FRAME_LENGTH_CONTROL: u32 = 3;
pub const URB_FUNCTION_VENDOR_DEVICE: u32 = 23;
pub const URB_FUNCTION_VENDOR_ENDPOINT: u32 = 25;
pub const URB_FUNCTION_VENDOR_INTERFACE: u32 = 24;
pub const URB_FUNCTION_VENDOR_OTHER: u32 = 32;
pub const URB_OPEN_STATIC_STREAMS_VERSION_100: u32 = 256;
pub const USBDI_VERSION: u32 = 1536;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USBD_CONFIGURATION_HANDLE(pub *mut core::ffi::c_void);
impl USBD_CONFIGURATION_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for USBD_CONFIGURATION_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const USBD_DEFAULT_MAXIMUM_TRANSFER_SIZE: u32 = 4294967295;
pub const USBD_DEFAULT_PIPE_TRANSFER: u32 = 8;
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy)]
pub struct USBD_DEVICE_INFORMATION {
    pub OffsetNext: u32,
    pub UsbdDeviceHandle: *mut core::ffi::c_void,
    pub DeviceDescriptor: super::usbspec::USB_DEVICE_DESCRIPTOR,
}
#[cfg(feature = "usbspec")]
impl Default for USBD_DEVICE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USBD_ENDPOINT_OFFLOAD_INFORMATION {
    pub Size: u32,
    pub EndpointAddress: u16,
    pub ResourceId: u32,
    pub Mode: USBD_ENDPOINT_OFFLOAD_MODE,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub TransferSegmentLA: PHYSICAL_ADDRESS,
    pub TransferSegmentVA: *mut core::ffi::c_void,
    pub TransferRingSize: usize,
    pub TransferRingInitialCycleBit: u32,
    pub MessageNumber: u32,
    pub EventRingSegmentLA: PHYSICAL_ADDRESS,
    pub EventRingSegmentVA: *mut core::ffi::c_void,
    pub EventRingSize: usize,
    pub EventRingInitialCycleBit: u32,
    pub ClientTransferRingSegmentPAIn: PHYSICAL_ADDRESS,
    pub ClientTransferRingSizeIn: usize,
    pub ClientDataBufferPAIn: PHYSICAL_ADDRESS,
    pub ClientDataBufferSizeIn: usize,
    pub ClientDataBufferLAOut: PHYSICAL_ADDRESS,
    pub ClientDataBufferVAOut: *mut core::ffi::c_void,
}
impl Default for USBD_ENDPOINT_OFFLOAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct USBD_ENDPOINT_OFFLOAD_INFORMATION_V1 {
    pub Size: u32,
    pub EndpointAddress: u16,
    pub ResourceId: u32,
    pub Mode: USBD_ENDPOINT_OFFLOAD_MODE,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub TransferSegmentLA: PHYSICAL_ADDRESS,
    pub TransferSegmentVA: *mut core::ffi::c_void,
    pub TransferRingSize: usize,
    pub TransferRingInitialCycleBit: u32,
    pub MessageNumber: u32,
    pub EventRingSegmentLA: PHYSICAL_ADDRESS,
    pub EventRingSegmentVA: *mut core::ffi::c_void,
    pub EventRingSize: usize,
    pub EventRingInitialCycleBit: u32,
}
impl Default for USBD_ENDPOINT_OFFLOAD_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type USBD_ENDPOINT_OFFLOAD_INFORMATION_V2 = USBD_ENDPOINT_OFFLOAD_INFORMATION;
pub type USBD_ENDPOINT_OFFLOAD_MODE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USBD_INTERFACE_HANDLE(pub *mut core::ffi::c_void);
impl USBD_INTERFACE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for USBD_INTERFACE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USBD_INTERFACE_INFORMATION {
    pub Length: u16,
    pub InterfaceNumber: u8,
    pub AlternateSetting: u8,
    pub Class: u8,
    pub SubClass: u8,
    pub Protocol: u8,
    pub Reserved: u8,
    pub InterfaceHandle: USBD_INTERFACE_HANDLE,
    pub NumberOfPipes: u32,
    pub Pipes: [USBD_PIPE_INFORMATION; 1],
}
impl Default for USBD_INTERFACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USBD_ISO_PACKET_DESCRIPTOR {
    pub Offset: u32,
    pub Length: u32,
    pub Status: USBD_STATUS,
}
pub const USBD_ISO_START_FRAME_RANGE: u32 = 1024;
pub const USBD_PF_CHANGE_MAX_PACKET: u32 = 1;
pub const USBD_PF_ENABLE_RT_THREAD_ACCESS: u32 = 4;
pub const USBD_PF_HANDLES_SSP_HIGH_BANDWIDTH_ISOCH: u32 = 256;
pub const USBD_PF_INTERACTIVE_PRIORITY: u32 = 48;
pub const USBD_PF_MAP_ADD_TRANSFERS: u32 = 8;
pub const USBD_PF_PRIORITY_MASK: u32 = 240;
pub const USBD_PF_SHORT_PACKET_OPT: u32 = 2;
pub const USBD_PF_SSP_HIGH_BANDWIDTH_ISOCH: u32 = 65536;
pub const USBD_PF_VALID_MASK: u32 = 319;
pub const USBD_PF_VIDEO_PRIORITY: u32 = 16;
pub const USBD_PF_VOICE_PRIORITY: u32 = 32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct USBD_PIPE_HANDLE(pub *mut core::ffi::c_void);
impl USBD_PIPE_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for USBD_PIPE_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USBD_PIPE_INFORMATION {
    pub MaximumPacketSize: u16,
    pub EndpointAddress: u8,
    pub Interval: u8,
    pub PipeType: USBD_PIPE_TYPE,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub MaximumTransferSize: u32,
    pub PipeFlags: u32,
}
pub type USBD_PIPE_TYPE = i32;
pub const USBD_SHORT_TRANSFER_OK: u32 = 2;
pub const USBD_START_ISO_TRANSFER_ASAP: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct USBD_STATUS(pub i32);
pub const USBD_STATUS_BABBLE_DETECTED: USBD_STATUS = USBD_STATUS(0xC0000012_u32 as _);
pub const USBD_STATUS_BAD_CONFIG_DESC_LENGTH: USBD_STATUS = USBD_STATUS(0xC0100006_u32 as _);
pub const USBD_STATUS_BAD_DESCRIPTOR: USBD_STATUS = USBD_STATUS(0xC0100000_u32 as _);
pub const USBD_STATUS_BAD_DESCRIPTOR_BLEN: USBD_STATUS = USBD_STATUS(0xC0100001_u32 as _);
pub const USBD_STATUS_BAD_DESCRIPTOR_TYPE: USBD_STATUS = USBD_STATUS(0xC0100002_u32 as _);
pub const USBD_STATUS_BAD_ENDPOINT_ADDRESS: USBD_STATUS = USBD_STATUS(0xC0100009_u32 as _);
pub const USBD_STATUS_BAD_ENDPOINT_DESCRIPTOR: USBD_STATUS = USBD_STATUS(0xC0100004_u32 as _);
pub const USBD_STATUS_BAD_INTERFACE_ASSOC_DESCRIPTOR: USBD_STATUS = USBD_STATUS(0xC0100005_u32 as _);
pub const USBD_STATUS_BAD_INTERFACE_DESCRIPTOR: USBD_STATUS = USBD_STATUS(0xC0100003_u32 as _);
pub const USBD_STATUS_BAD_NUMBER_OF_ENDPOINTS: USBD_STATUS = USBD_STATUS(0xC0100008_u32 as _);
pub const USBD_STATUS_BAD_NUMBER_OF_INTERFACES: USBD_STATUS = USBD_STATUS(0xC0100007_u32 as _);
pub const USBD_STATUS_BAD_START_FRAME: USBD_STATUS = USBD_STATUS(0xC0000A00_u32 as _);
pub const USBD_STATUS_BTSTUFF: USBD_STATUS = USBD_STATUS(0xC0000002_u32 as _);
pub const USBD_STATUS_BUFFER_OVERRUN: USBD_STATUS = USBD_STATUS(0xC000000C_u32 as _);
pub const USBD_STATUS_BUFFER_TOO_SMALL: USBD_STATUS = USBD_STATUS(0xC0003000_u32 as _);
pub const USBD_STATUS_BUFFER_UNDERRUN: USBD_STATUS = USBD_STATUS(0xC000000D_u32 as _);
pub const USBD_STATUS_CANCELED: USBD_STATUS = USBD_STATUS(0xC0010000_u32 as _);
pub const USBD_STATUS_CRC: USBD_STATUS = USBD_STATUS(0xC0000001_u32 as _);
pub const USBD_STATUS_DATA_BUFFER_ERROR: USBD_STATUS = USBD_STATUS(0xC0000013_u32 as _);
pub const USBD_STATUS_DATA_OVERRUN: USBD_STATUS = USBD_STATUS(0xC0000008_u32 as _);
pub const USBD_STATUS_DATA_TOGGLE_MISMATCH: USBD_STATUS = USBD_STATUS(0xC0000003_u32 as _);
pub const USBD_STATUS_DATA_UNDERRUN: USBD_STATUS = USBD_STATUS(0xC0000009_u32 as _);
pub const USBD_STATUS_DEVICE_GONE: USBD_STATUS = USBD_STATUS(0xC0007000_u32 as _);
pub const USBD_STATUS_DEV_NOT_RESPONDING: USBD_STATUS = USBD_STATUS(0xC0000005_u32 as _);
pub const USBD_STATUS_ENDPOINT_HALTED: USBD_STATUS = USBD_STATUS(0xC0000030_u32 as _);
pub const USBD_STATUS_ERROR_BUSY: USBD_STATUS = USBD_STATUS(0x80000400_u32 as _);
pub const USBD_STATUS_ERROR_SHORT_TRANSFER: USBD_STATUS = USBD_STATUS(0x80000900_u32 as _);
pub const USBD_STATUS_FIFO: USBD_STATUS = USBD_STATUS(0xC0000010_u32 as _);
pub const USBD_STATUS_FRAME_CONTROL_NOT_OWNED: USBD_STATUS = USBD_STATUS(0xC0000D00_u32 as _);
pub const USBD_STATUS_FRAME_CONTROL_OWNED: USBD_STATUS = USBD_STATUS(0xC0000C00_u32 as _);
pub const USBD_STATUS_HUB_INTERNAL_ERROR: USBD_STATUS = USBD_STATUS(0xC0009000_u32 as _);
pub const USBD_STATUS_INAVLID_CONFIGURATION_DESCRIPTOR: USBD_STATUS = USBD_STATUS(0xC0000F00_u32 as _);
pub const USBD_STATUS_INAVLID_PIPE_FLAGS: USBD_STATUS = USBD_STATUS(0xC0005000_u32 as _);
pub const USBD_STATUS_INSUFFICIENT_RESOURCES: USBD_STATUS = USBD_STATUS(0xC0001000_u32 as _);
pub const USBD_STATUS_INTERFACE_NOT_FOUND: USBD_STATUS = USBD_STATUS(0xC0004000_u32 as _);
pub const USBD_STATUS_INTERNAL_HC_ERROR: USBD_STATUS = USBD_STATUS(0x80000800_u32 as _);
pub const USBD_STATUS_INVALID_CONFIGURATION_DESCRIPTOR: USBD_STATUS = USBD_STATUS(0xC0000F00_u32 as _);
pub const USBD_STATUS_INVALID_PARAMETER: USBD_STATUS = USBD_STATUS(0x80000300_u32 as _);
pub const USBD_STATUS_INVALID_PIPE_HANDLE: USBD_STATUS = USBD_STATUS(0x80000600_u32 as _);
pub const USBD_STATUS_INVALID_STREAM_ID: USBD_STATUS = USBD_STATUS(0xC0000016_u32 as _);
pub const USBD_STATUS_INVALID_STREAM_TYPE: USBD_STATUS = USBD_STATUS(0xC0000015_u32 as _);
pub const USBD_STATUS_INVALID_URB_FUNCTION: USBD_STATUS = USBD_STATUS(0x80000200_u32 as _);
pub const USBD_STATUS_ISOCH_REQUEST_FAILED: USBD_STATUS = USBD_STATUS(0xC0000B00_u32 as _);
pub const USBD_STATUS_ISO_NA_LATE_USBPORT: USBD_STATUS = USBD_STATUS(0xC0040000_u32 as _);
pub const USBD_STATUS_ISO_NOT_ACCESSED_BY_HW: USBD_STATUS = USBD_STATUS(0xC0020000_u32 as _);
pub const USBD_STATUS_ISO_NOT_ACCESSED_LATE: USBD_STATUS = USBD_STATUS(0xC0050000_u32 as _);
pub const USBD_STATUS_ISO_TD_ERROR: USBD_STATUS = USBD_STATUS(0xC0030000_u32 as _);
pub const USBD_STATUS_NOT_ACCESSED: USBD_STATUS = USBD_STATUS(0xC000000F_u32 as _);
pub const USBD_STATUS_NOT_SUPPORTED: USBD_STATUS = USBD_STATUS(0xC0000E00_u32 as _);
pub const USBD_STATUS_NO_BANDWIDTH: USBD_STATUS = USBD_STATUS(0x80000700_u32 as _);
pub const USBD_STATUS_NO_PING_RESPONSE: USBD_STATUS = USBD_STATUS(0xC0000014_u32 as _);
pub const USBD_STATUS_PENDING: USBD_STATUS = USBD_STATUS(0x40000000_u32 as _);
pub const USBD_STATUS_PID_CHECK_FAILURE: USBD_STATUS = USBD_STATUS(0xC0000006_u32 as _);
pub const USBD_STATUS_PORT_OPERATION_PENDING: USBD_STATUS = USBD_STATUS(0x1_u32 as _);
pub const USBD_STATUS_RESERVED1: USBD_STATUS = USBD_STATUS(0xC000000A_u32 as _);
pub const USBD_STATUS_RESERVED2: USBD_STATUS = USBD_STATUS(0xC000000B_u32 as _);
pub const USBD_STATUS_SET_CONFIG_FAILED: USBD_STATUS = USBD_STATUS(0xC0002000_u32 as _);
pub const USBD_STATUS_STALL_PID: USBD_STATUS = USBD_STATUS(0xC0000004_u32 as _);
pub const USBD_STATUS_STATUS_NOT_MAPPED: USBD_STATUS = USBD_STATUS(0xC0008000_u32 as _);
pub const USBD_STATUS_SUCCESS: USBD_STATUS = USBD_STATUS(0x0_u32 as _);
pub const USBD_STATUS_TIMEOUT: USBD_STATUS = USBD_STATUS(0xC0006000_u32 as _);
pub const USBD_STATUS_UNEXPECTED_PID: USBD_STATUS = USBD_STATUS(0xC0000007_u32 as _);
pub const USBD_STATUS_XACT_ERROR: USBD_STATUS = USBD_STATUS(0xC0000011_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USBD_STREAM_INFORMATION {
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub StreamID: u32,
    pub MaximumTransferSize: u32,
    pub PipeFlags: u32,
}
pub const USBD_TRANSFER_DIRECTION: u32 = 1;
pub const USBD_TRANSFER_DIRECTION_IN: u32 = 1;
pub const USBD_TRANSFER_DIRECTION_OUT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USBD_VERSION_INFORMATION {
    pub USBDI_Version: u32,
    pub Supported_USB_Version: u32,
}
pub type USB_CONTROLLER_FLAVOR = i32;
pub const USB_DEFAULT_DEVICE_ADDRESS: u32 = 0;
pub const USB_DEFAULT_ENDPOINT_ADDRESS: u32 = 0;
pub const USB_DEFAULT_MAX_PACKET: u32 = 64;
pub const USB_HcGeneric: USB_CONTROLLER_FLAVOR = 0;
pub const USB_PORTATTR_MINI_CONNECTOR: u32 = 4;
pub const USB_PORTATTR_NO_CONNECTOR: u32 = 1;
pub const USB_PORTATTR_NO_OVERCURRENT_UI: u32 = 33554432;
pub const USB_PORTATTR_OEM_CONNECTOR: u32 = 8;
pub const USB_PORTATTR_OWNED_BY_CC: u32 = 16777216;
pub const USB_PORTATTR_SHARED_USB2: u32 = 2;
pub const UsbdEndpointOffloadHardwareAssisted: USBD_ENDPOINT_OFFLOAD_MODE = 2;
pub const UsbdEndpointOffloadModeNotSupported: USBD_ENDPOINT_OFFLOAD_MODE = 0;
pub const UsbdEndpointOffloadSoftwareAssisted: USBD_ENDPOINT_OFFLOAD_MODE = 1;
pub const UsbdPipeTypeBulk: USBD_PIPE_TYPE = 2;
pub const UsbdPipeTypeControl: USBD_PIPE_TYPE = 0;
pub const UsbdPipeTypeInterrupt: USBD_PIPE_TYPE = 3;
pub const UsbdPipeTypeIsochronous: USBD_PIPE_TYPE = 1;
pub const VALID_TRANSFER_FLAGS_MASK: u32 = 15;
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_BULK_OR_INTERRUPT_TRANSFER {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_BULK_OR_INTERRUPT_TRANSFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_DESCRIPTOR_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: u16,
    pub Index: u8,
    pub DescriptorType: u8,
    pub LanguageId: u16,
    pub Reserved2: u16,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_FEATURE_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub Reserved2: u32,
    pub Reserved3: u32,
    pub Reserved4: *mut core::ffi::c_void,
    pub Reserved5: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved0: u16,
    pub FeatureSelector: u16,
    pub Index: u16,
    pub Reserved1: u16,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_FEATURE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: [u8; 8],
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_GET_INTERFACE_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: [u8; 4],
    pub Interface: u16,
    pub Reserved2: u16,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_GET_INTERFACE_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_GET_STATUS_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub Reserved1: [u8; 4],
    pub Index: u16,
    pub Reserved2: u16,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_GET_STATUS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_TRANSFER {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub SetupPacket: [u8; 8],
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_TRANSFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_TRANSFER_EX {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub Timeout: u32,
    pub hca: _URB_HCD_AREA,
    pub SetupPacket: [u8; 8],
}
impl Default for _URB_CONTROL_TRANSFER_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub RequestTypeReservedBits: u8,
    pub Request: u8,
    pub Value: u16,
    pub Index: u16,
    pub Reserved1: u16,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_FRAME_LENGTH_CONTROL {
    pub Hdr: _URB_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_GET_CURRENT_FRAME_NUMBER {
    pub Hdr: _URB_HEADER,
    pub FrameNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_GET_FRAME_LENGTH {
    pub Hdr: _URB_HEADER,
    pub FrameLength: u32,
    pub FrameNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub MaximumSendPathDelayInMilliSeconds: u32,
    pub MaximumCompletionPathDelayInMilliSeconds: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_HCD_AREA {
    pub Reserved8: [*mut core::ffi::c_void; 8],
}
impl Default for _URB_HCD_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_HEADER {
    pub Length: u16,
    pub Function: u16,
    pub Status: USBD_STATUS,
    pub UsbdDeviceHandle: *mut core::ffi::c_void,
    pub UsbdFlags: u32,
}
impl Default for _URB_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_ISOCH_TRANSFER {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub TransferFlags: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub StartFrame: u32,
    pub NumberOfPackets: u32,
    pub ErrorCount: u32,
    pub IsoPacket: [USBD_ISO_PACKET_DESCRIPTOR; 1],
}
#[cfg(feature = "usbspec")]
impl Default for _URB_ISOCH_TRANSFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_OPEN_STATIC_STREAMS {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub NumberOfStreams: u32,
    pub StreamInfoVersion: u16,
    pub StreamInfoSize: u16,
    pub Streams: PUSBD_STREAM_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    pub Hdr: _URB_HEADER,
    pub Reserved: *mut core::ffi::c_void,
    pub Reserved0: u32,
    pub TransferBufferLength: u32,
    pub TransferBuffer: *mut core::ffi::c_void,
    pub TransferBufferMDL: PMDL,
    pub UrbLink: *mut URB,
    pub hca: _URB_HCD_AREA,
    pub _bitfield: u8,
    pub Reserved2: u8,
    pub InterfaceNumber: u8,
    pub MS_PageIndex: u8,
    pub MS_FeatureDescriptorIndex: u16,
    pub Reserved3: u16,
}
#[cfg(feature = "usbspec")]
impl Default for _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_PIPE_REQUEST {
    pub Hdr: _URB_HEADER,
    pub PipeHandle: USBD_PIPE_HANDLE,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "usbspec")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_SELECT_CONFIGURATION {
    pub Hdr: _URB_HEADER,
    pub ConfigurationDescriptor: super::usbspec::PUSB_CONFIGURATION_DESCRIPTOR,
    pub ConfigurationHandle: USBD_CONFIGURATION_HANDLE,
    pub Interface: USBD_INTERFACE_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_SELECT_INTERFACE {
    pub Hdr: _URB_HEADER,
    pub ConfigurationHandle: USBD_CONFIGURATION_HANDLE,
    pub Interface: USBD_INTERFACE_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _URB_SET_FRAME_LENGTH {
    pub Hdr: _URB_HEADER,
    pub FrameLengthDelta: i32,
}
