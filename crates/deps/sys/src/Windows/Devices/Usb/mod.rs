#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IUsbBulkInEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbBulkInPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbBulkOutEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbBulkOutPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbConfigurationDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbConfigurationDescriptorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbControlRequestType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDeviceClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDeviceClasses(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDeviceClassesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDeviceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbEndpointDescriptorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterfaceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterfaceDescriptorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterfaceSetting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterruptInEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterruptInEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterruptInPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterruptOutEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbInterruptOutPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbSetupPacket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUsbSetupPacketFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbBulkInEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbBulkInPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbBulkOutEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbBulkOutPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbConfigurationDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbControlRecipient(pub i32);
impl UsbControlRecipient {
    pub const Device: Self = Self(0i32);
    pub const SpecifiedInterface: Self = Self(1i32);
    pub const Endpoint: Self = Self(2i32);
    pub const Other: Self = Self(3i32);
    pub const DefaultInterface: Self = Self(4i32);
}
impl ::core::marker::Copy for UsbControlRecipient {}
impl ::core::clone::Clone for UsbControlRecipient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UsbControlRequestType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbControlTransferType(pub i32);
impl UsbControlTransferType {
    pub const Standard: Self = Self(0i32);
    pub const Class: Self = Self(1i32);
    pub const Vendor: Self = Self(2i32);
}
impl ::core::marker::Copy for UsbControlTransferType {}
impl ::core::clone::Clone for UsbControlTransferType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UsbDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbDeviceClass(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbDeviceClasses(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbDeviceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbEndpointType(pub i32);
impl UsbEndpointType {
    pub const Control: Self = Self(0i32);
    pub const Isochronous: Self = Self(1i32);
    pub const Bulk: Self = Self(2i32);
    pub const Interrupt: Self = Self(3i32);
}
impl ::core::marker::Copy for UsbEndpointType {}
impl ::core::clone::Clone for UsbEndpointType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UsbInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterfaceDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterfaceSetting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterruptInEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterruptInEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterruptInPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterruptOutEndpointDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbInterruptOutPipe(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbReadOptions(pub u32);
impl UsbReadOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const OverrideAutomaticBufferManagement: Self = Self(2u32);
    pub const IgnoreShortPacket: Self = Self(4u32);
    pub const AllowPartialReads: Self = Self(8u32);
}
impl ::core::marker::Copy for UsbReadOptions {}
impl ::core::clone::Clone for UsbReadOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UsbSetupPacket(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UsbTransferDirection(pub i32);
impl UsbTransferDirection {
    pub const Out: Self = Self(0i32);
    pub const In: Self = Self(1i32);
}
impl ::core::marker::Copy for UsbTransferDirection {}
impl ::core::clone::Clone for UsbTransferDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UsbWriteOptions(pub u32);
impl UsbWriteOptions {
    pub const None: Self = Self(0u32);
    pub const AutoClearStall: Self = Self(1u32);
    pub const ShortPacketTerminate: Self = Self(2u32);
}
impl ::core::marker::Copy for UsbWriteOptions {}
impl ::core::clone::Clone for UsbWriteOptions {
    fn clone(&self) -> Self {
        *self
    }
}
