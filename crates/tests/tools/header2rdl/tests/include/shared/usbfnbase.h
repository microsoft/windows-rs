/*++

Copyright (C) Microsoft Corporation. All rights reserved.

Module Name:

    usbfnbase.h

Abstract:

    This header defines all interfaces and macros that USB Function class
    drivers will require to implement class driver functionality.

Environment:

    Kernel and user mode

--*/
#pragma once

//
// Includes
//
#include <usbspec.h>

//
// Registry path for the USBFn Enumeration keys
//
#define KREGUSBFNENUMPATH L"\\Registry\\Machine\\SYSTEM\\CurrentControlSet\\Control\\USBFN\\"
#define UREGUSBFNENUMPATH L"HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\USBFN\\"

//
// Registry path for the USBFn Enumeration keys (manufacturing mode)
//
#define KREGMANUSBFNENUMPATH L"\\Registry\\Machine\\SYSTEM\\CurrentControlSet\\Control\\ManufacturingMode\\Current\\USBFN\\"
#define UREGMANUSBFNENUMPATH L"HKEY_LOCAL_MACHINE\\SYSTEM\\CurrentControlSet\\Control\\ManufacturingMode\\Current\\USBFN\\"


//
// Structure definitions
//

//
// A configuration can have upto 15 endpoints plus Endpoint 0 which belongs to the device. 
// That translates into 16 pipes, including the pipe belonging to EP0.
// 

#define MAX_NUM_USBFN_ENDPOINTS     15
#define MAX_NUM_USBFN_PIPES         (MAX_NUM_USBFN_ENDPOINTS)+1

//
// Maximum length of a configuration length supported.
//

#define MAX_CONFIGURATION_NAME_LENGTH 40

#define MAX_USB_STRING_LENGTH 255

//
// Maximum number of supported configurations
//

#define MAX_SUPPORTED_CONFIGURATIONS 12

//
// if an interrupt endpoint's wMaxPacketSize should not be updated based on  
// the current speed of the device and limit it to whatever was mentioned in  
// the registry, wMaxPacketSize's MSB should be set to 1. Or the function stack
// will update it to an optimal value based on current connection speed.
// If this mask is set to wMaxPacketSize of the endpoint descriptor, then that 
// endpoint's wMaxPacketSize will not be updated according to the current speed.
#define USBFN_INTERRUPT_ENDPOINT_SIZE_NOT_UPDATEABLE_MASK 0x80

//
// USB Electrical Test Mode selectors
//

#define USB_TEST_MODE_TEST_J              0x01
#define USB_TEST_MODE_TEST_K              0x02
#define USB_TEST_MODE_TEST_SE0_NAK        0x03
#define USB_TEST_MODE_TEST_PACKET         0x04
#define USB_TEST_MODE_TEST_FORCE_ENABLE   0x05

//
// Maximum length of an interface name.
//

#define MAX_INTERFACE_NAME_LENGTH 40
#define MAX_ALTERNATE_NAME_LENGTH 40
#define MAX_ASSOCIATION_NAME_LENGTH 40

//
// Maximum length of an interface GUID string.
//

#define MAX_INTERFACE_GUID_SIZE (sizeof(L"{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}"))
#define MAX_INTERFACE_GUID_LENGTH (MAX_INTERFACE_GUID_SIZE / sizeof(WCHAR))

//
// Notifications sent to class drivers.
//
typedef enum _USBFN_EVENT
{
    UsbfnEventMinimum = 0x0,
    //
    // VBUS is powered.
    // No action is required.
    //
    UsbfnEventAttach,
    
    //
    // USBFN has completed a USB Reset.
    // If previously configured, class drivers should
    // reset their state. Transfer requests will be cancelled.
    //
    UsbfnEventReset,
    
    //
    // VBUS is no longer powered.
    // If previously configured, class drivers should
    // reset their state. Transfer requests will be cancelled.
    // The BusSpeed field of the notification is set appropriately.
    //
    UsbfnEventDetach,
    
    //
    // There have been no SOF packets on the bus for 3ms.
    // If a class driver wishes to issue a remote wakeup,
    // please use IOCTL_INTERNAL_USBFN_SIGNAL_REMOTE_WAKEUP.
    //
    UsbfnEventSuspend,
    
    //
    // USBFN has resumed from suspend to the previous state.
    //
    UsbfnEventResume,
    
    //
    // USBFN has received a setup packet with
    // bmRequestType.Type == BMREQUEST_CLASS and
    // bmRequestType.Recipient == BMREQUEST_TO_INTERFACE.
    // USBFN has forwarded this setup packet to the class driver
    // specified in wIndex.LowByte.
    //
    // The setup packet is available in the SetupPacket field of the
    // event. If the control transfer does not require a data stage,
    // class drivers should respond with
    // IOCTL_INTERNAL_USBFN_CONTROL_STATUS_HANDSHAKE_OUT.
    // If a data stage is required, class drivers should respond with
    // one or more IOCTL_INTERNAL_USBFN_TRANSFER*, followed by
    // IOCTL_INTERNAL_USBFN_CONTROL_STATUS_HANDSHAKE* in the opposite
    // direction.
    //
    UsbfnEventSetupPacket,
    
    //
    // USBFN has received a SET_CONFIGURATION setup packet. Transfer
    // requests from class drivers are now permitted.
    // The ConfigurationValue of the notification is set to wValue.W.
    //
    UsbfnEventConfigured,
    
    //
    // USBFN has received a SET_CONFIGURATION setup packet with
    // wValue.W == 0. If previously configured, class drivers should
    // reset their state. Transfer requests will be cancelled.
    //
    UsbfnEventUnConfigured,
    
    //
    // Deprecated.
    //
    UsbfnEventPortType,
    
    //
    // Deprecated.
    //
    UsbfnEventBusTearDown,

    //
    // USBFN has received a SET_INTERFACE setup packet.  On receiving this
    // notification the class driver should query for the new endpoint set
    // for the interface.
    //
    UsbfnEventSetInterface,

    UsbfnEventMaximum
} USBFN_EVENT, *PUSBFN_EVENT;


//
// USBFN Port Types
//
typedef enum _USBFN_PORT_TYPE {
    UsbfnUnknownPort = 0,
    UsbfnStandardDownstreamPort,
    UsbfnChargingDownstreamPort,
    UsbfnDedicatedChargingPort,
    UsbfnInvalidDedicatedChargingPort,
    UsbfnProprietaryDedicatedChargingPort,
    UsbfnPortTypeMaximum
} USBFN_PORT_TYPE, *PUSBFN_PORT_TYPE;


//
// Possible USB Bus speeds
//
typedef enum _USBFN_BUS_SPEED
{
    UsbfnBusSpeedLow,
    UsbfnBusSpeedFull,
    UsbfnBusSpeedHigh,
    UsbfnBusSpeedSuper,
    UsbfnBusSpeedMaximum
} USBFN_BUS_SPEED, *PUSBFN_BUS_SPEED;

//
// Possible USB Bus transfer and endpoint directions
//

typedef enum _USBFN_DIRECTION
{
    UsbfnDirectionMinimum = 0x0,
    UsbfnDirectionIn,
    UsbfnDirectionOut,
    UsbfnDirectionTx = UsbfnDirectionIn,
    UsbfnDirectionRx = UsbfnDirectionOut,
    UsbfnDirectionMaximum
} USBFN_DIRECTION, *PUSBFN_DIRECTION;

//
// These states indicated the current state of the USB driver.
// This should be used by class-extension clients only.
//
typedef enum _USBFN_DEVICE_STATE {
    UsbfnDeviceStateMinimum = 0x0,
    //
    // VBUS is powered
    //
    UsbfnDeviceStateAttached,
    
    //
    // USBFN has received a USB Reset.
    //
    UsbfnDeviceStateDefault,
    
    //
    // VBUS is not powered.
    //
    UsbfnDeviceStateDetached,
    
    //
    // USBFN has received a SET_ADDRESS setup packet.
    //
    UsbfnDeviceStateAddressed,
    
    //
    // USBFN has received a SET_CONFIGURATION setup packet.
    //
    UsbfnDeviceStateConfigured,
    
    //
    // There have been no SOF packets on the bus for 3ms.
    //
    UsbfnDeviceStateSuspended,

    UsbfnDeviceStateStateMaximum
} USBFN_DEVICE_STATE, *PUSBFN_DEVICE_STATE;


typedef struct _ALTERNATE_INTERFACE {
    USHORT InterfaceNumber;
    USHORT AlternateInterfaceNumber;
} ALTERNATE_INTERFACE, *PALTERNATE_INTERFACE;


//
// Notification structure used by class drivers to receive information
// about USBFN. Please see the definition of USBFN_EVENT to for details
// about each event.
//
typedef struct _USBFN_NOTIFICATION
{
    USBFN_EVENT Event;

    union 
    {
        USBFN_BUS_SPEED BusSpeed;
        USB_DEFAULT_PIPE_SETUP_PACKET SetupPacket;
        USHORT ConfigurationValue;
        USBFN_PORT_TYPE PortType;
        ALTERNATE_INTERFACE AlternateInterface;
    } u;

} USBFN_NOTIFICATION, *PUSBFN_NOTIFICATION;

//
// Defines an endpoint pipe index (not the address)
//
typedef ULONG USBFNPIPEID;
typedef USBFNPIPEID *PUSBFNPIPEID;

typedef struct _USBFN_PIPE_INFORMATION {
    USB_ENDPOINT_DESCRIPTOR EpDesc;
    USBFNPIPEID PipeId;
} USBFN_PIPE_INFORMATION, *PUSBFN_PIPE_INFORMATION;

typedef struct _USBFN_CLASS_INTERFACE {
    UINT8 InterfaceNumber;
    UINT8 PipeCount;
    USBFN_PIPE_INFORMATION PipeArr[MAX_NUM_USBFN_PIPES];
} USBFN_CLASS_INTERFACE, *PUSBFN_CLASS_INTERFACE;

typedef struct _USBFN_CLASS_INFORMATION_PACKET {
    USBFN_CLASS_INTERFACE FullSpeedClassInterface;
    USBFN_CLASS_INTERFACE HighSpeedClassInterface;
    WCHAR InterfaceName[MAX_INTERFACE_NAME_LENGTH];
    WCHAR InterfaceGuid[MAX_INTERFACE_GUID_LENGTH];
    BOOLEAN HasInterfaceGuid;
    USBFN_CLASS_INTERFACE SuperSpeedClassInterface;
} USBFN_CLASS_INFORMATION_PACKET, *PUSBFN_CLASS_INFORMATION_PACKET;

typedef struct _USBFN_CLASS_INTERFACE_EX {
    UINT8 BaseInterfaceNumber;
    UINT8 InterfaceCount;
    UINT8 PipeCount;
    USBFN_PIPE_INFORMATION PipeArr[MAX_NUM_USBFN_PIPES];
} USBFN_CLASS_INTERFACE_EX, *PUSBFN_CLASS_INTERFACE_EX;

typedef struct _USBFN_CLASS_INFORMATION_PACKET_EX {
    USBFN_CLASS_INTERFACE_EX FullSpeedClassInterfaceEx;
    USBFN_CLASS_INTERFACE_EX HighSpeedClassInterfaceEx;
    USBFN_CLASS_INTERFACE_EX SuperSpeedClassInterfaceEx;
    WCHAR InterfaceName[MAX_INTERFACE_NAME_LENGTH];
    WCHAR InterfaceGuid[MAX_INTERFACE_GUID_LENGTH];
    BOOLEAN HasInterfaceGuid;
} USBFN_CLASS_INFORMATION_PACKET_EX, *PUSBFN_CLASS_INFORMATION_PACKET_EX;

typedef struct _USBFN_INTERFACE_INFO {
    UINT8 InterfaceNumber;
    USBFN_BUS_SPEED Speed;
    USHORT Size;
    UCHAR InterfaceDescriptorSet[1];
} USBFN_INTERFACE_INFO, *PUSBFN_INTERFACE_INFO;

typedef struct _USBFN_USB_STRING {
    UINT8 StringIndex;
    WCHAR UsbString[MAX_USB_STRING_LENGTH];
} USBFN_USB_STRING, *PUSBFN_USB_STRING;


//
// Configuration packet that gives information about 
// an available configuration.
//
typedef struct _USBFN_BUS_CONFIGURATION_INFO {
    //
    // NULL terminated wide char string indicating the name of a configuration
    //
    WCHAR ConfigurationName[MAX_CONFIGURATION_NAME_LENGTH];
    //
    // BOOLEAN, indicating if this configuration is / should be the 
    // considered as current configuation.
    //
    BOOLEAN IsCurrent;
    //
    // The state of this configuration - is this configuration active?
    // This is a read-only information returned by Class Extension and
    // will be ignored in requests to Class extension.
    //
    BOOLEAN IsActive;
} USBFN_BUS_CONFIGURATION_INFO, *PUSBFN_BUS_CONFIGURATION_INFO;
