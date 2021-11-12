#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct UsbControlRecipient(i32);
#[repr(transparent)]
pub struct UsbControlRequestType(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UsbControlTransferType(i32);
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
#[repr(C)]
pub struct UsbEndpointType(i32);
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
#[repr(C)]
pub struct UsbReadOptions(i32);
#[repr(transparent)]
pub struct UsbSetupPacket(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UsbTransferDirection(i32);
#[repr(C)]
pub struct UsbWriteOptions(i32);
