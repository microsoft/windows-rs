/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

        USBIOCTL.H

Abstract:

   This file defines both kernel and user mode IOCTL
   codes supported by the USB core stack.

   These APIs are the APIS supported by th USB hub driver and the USB bus
   driver AKA USBPORT.

   Typically only user mode applications (usbui) or the hub driver include this
   file, USB drivers should use usbdrivr.h usb bus drivers should include
   usbkern.h

Environment:

    Kernel & user mode

Revision History:

    09-29-95 : created
    01-06-97 : added user mode hub ioctls
    10-31-99 : cleanup and document, jdunn
    1-25-03  : more cleanup and documentation
    2-10-04  : header versioning

--*/

#ifndef   __USBIOCTL_H__
#define   __USBIOCTL_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)



#include "usb200.h"

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) // named type definition in parentheses
#pragma warning(disable:4214) // named type definition in parentheses
#pragma warning(disable:4200) // named type definition in parentheses


#ifndef FAR
#define FAR
#endif

#include "usbiodef.h"

/*
    IOCTL definitions
*/

/*
   USB kernel Mode IOCTLS
*/

/*
Routine Description:
Define the standard USB 'URB' IOCTL

IOCTL_INTERNAL_USB_SUBMIT_URB

This IOCTL is used by client drivers to submit URB (USB Request Blocks)

Parameters.Others.Argument1 = pointer to URB

*/

#define IOCTL_INTERNAL_USB_SUBMIT_URB  \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_SUBMIT_URB,  \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)

/* IOCTL_INTERNAL_USB_RESET_PORT

    This IOCTL is used by kernel mode drivers to reset their
    upstream port.

    After a successful reset the device is re-configured to the
    same configuration it was in before the reset.  All pipe
    handles, configuration handles and interface handles remain
    valid.

*/

#define IOCTL_INTERNAL_USB_RESET_PORT  \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_RESET_PORT, \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)

/*  IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO

Routine Description:
This function is used to by hubs to get the top of the physical USB stack.
All IRPs passed to a hub PDO are either serviced by the hub or forwarded
directly to the top of the bus driver stack i.e. the root hub PDO.  A filter
driver interested only in bus traffic (AKA Urbs) can see such traffic by
attaching to the top of the root hub PDO, see section 9.

Parameters:
ioStackLocation->Parameters.Others.Argument1
>>RootHubPhysicalDeviceObject
This parameter contains a pointer that is filled in with the root hub PDO.
This is the actual PDO created by the USBPORT driver for the root hub.

ioStackLocation->Parameters.Others.Argument2
>> HcdTopOfStack
This parameter contains a pointer that is filled in with the top of the bus
driver stack is the device object returned when the hub driver attached to
top of the device stack associated with the root hub PDO.
*/

#define IOCTL_INTERNAL_USB_GET_ROOTHUB_PDO  \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_GET_ROOTHUB_PDO, \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)



#define  USBD_PORT_ENABLED      0x00000001
#define  USBD_PORT_CONNECTED    0x00000002

/* IOCTL_INTERNAL_USB_GET_PORT_STATUS

Routine Description:
This function returns the current 'live' status of the port.  It can be used
by client drivers to determine the current state of their device because
certain hardware errors on the bus can result in a device port being disabled
.  The hub driver must communicate with the hub to get this information, if
it cannot for some reason a failure status is returned.

This API will fail if called at raised IRQL.

IRQL: Passive

Parameters:
ioStackLocation->Parameters.Others.Argument1
A pointer to a ULONG that is filled in with the port status bits defined below:

*/

#define IOCTL_INTERNAL_USB_GET_PORT_STATUS  \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_GET_PORT_STATUS, \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)

/*
    IOCTL_INTERNAL_USB_ENABLE_PORT is obsolete, drivers should use
    IOCTL_INTERNAL_USB_RESET_PORT
*/

#define IOCTL_INTERNAL_USB_ENABLE_PORT      \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_ENABLE_PORT, \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)


/* IOCTL_INTERNAL_USB_GET_HUB_COUNT

Routine Description:
This function is used to count the number of hubs in the stack.  It is used
to enforce the electrical limitation of no more that 5 hubs plus the root
being chained together.

Parameters:
ioStackLocation->Parameters.Others.Argument1
>>Count
This parameter contains a pointer that is filled in with the current count of
hubs in the stack. Each instance of the hub driver that receives the IRP
increments the counter and passes the irp on to its' PDO.  When the Irp
reaches the hub that is the root the IRP is completed.  This will result in
the count value being equal to the number of hubs (including the root).


*/

#define IOCTL_INTERNAL_USB_GET_HUB_COUNT    \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_GET_HUB_COUNT, \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)

/* IOCTL_INTERNAL_USB_CYCLE_PORT

    This IOCTL will simulate a plug/unplug on the drivers upstream
    port.  The device will be removed and re-added by PnP.
*/

#define IOCTL_INTERNAL_USB_CYCLE_PORT  \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_CYCLE_PORT, \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)

/* IOCTL_INTERNAL_USB_GET_HUB_NAME

Routine Description:
This API returns the UNICODE symbolic name for the PDO if the PDO is for a
usbhub, otherwise a NULL string is returned.  The symbolic name can be used
to retrieve additional information about the hub through user mode ioctl apis
and WMI calls.

Parameters:
ioStackLocation->Parameters.DeviceIoControl.OutputBufferLength
Length of buffer passed bytes.

Irp->AssociatedIrp.SystemBuffer
A pointer to a structure (USB_HUB_NAME) to receive the symbolic name.

 USB_BUS_NOTIFICATION.
ActualLength - The structure size in bytes necessary to hold the NULL
   terminated symbolic link name.  This includes the entire structure, not
   just the name.

  HubName - The UNICODE NULL terminated symbolic link name of the external
hub attached to the port.  If there is no external hub attached to the port a
single NULL is returned.

*/

#define IOCTL_INTERNAL_USB_GET_HUB_NAME  \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_GET_HUB_NAME,  \
                                            METHOD_BUFFERED,  \
                                            FILE_ANY_ACCESS)

/*
    IOCTL_INTERNAL_USB_GET_BUS_INFO is obsolete -- it has been replaced by the
    USB_BUSIFFN_QUERY_BUS_INFORMATION service available thru the usb stack bus
    interface. Drivers should use the bus interface function instead
*/
#define IOCTL_INTERNAL_USB_GET_BUS_INFO         CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_GET_BUS_INFO,  \
                                                    METHOD_BUFFERED,  \
                                                    FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_GET_CONTROLLER_NAME  CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_GET_CONTROLLER_NAME,  \
                                                    METHOD_BUFFERED,  \
                                                    FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_GET_BUSGUID_INFO     CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_GET_BUSGUID_INFO,  \
                                                    METHOD_BUFFERED,  \
                                                    FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_GET_PARENT_HUB_INFO  CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_GET_PARENT_HUB_INFO,  \
                                                    METHOD_BUFFERED,  \
                                                    FILE_ANY_ACCESS)

/*
   USB kernel Mode IOCTLS defined for windows XP and later
*/

#if (_WIN32_WINNT >= 0x0501)

/* IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION

This ioctl registers a device to receive notification when a specific
timeout has expired and it should now be suspended in order to conserve
power. If all devices on a hub are suspended, then the actual hub
can be suspended.

Routine Description:
This function is part of the hub drivers selective suspend feature (see
section 2 usbhub.doc).   This API is serviced only by hubs it is not supported
by a USB Port driver.  The client USB drivers use this API to register an idle
callback request with the parent hub.  The details on how this API is handled
can be found in section 2.

Parameters:
ioStackLocation->Parameters.Others.Argument1
>>IdeCallbackInfo
A pointer to a structure containing the callback routine and a context value.

_IRQL_requires_max_(PASSIVE_LEVEL)
typedef
VOID
(*USB_IDLE_CALLBACK)(
    _In_ PVOID Context
    );

typedef struct _USB_IDLE_CALLBACK_INFO {
    USB_IDLE_CALLBACK IdleCallback;
    PVOID IdleContext;
} USB_IDLE_CALLBACK_INFO, *PUSB_IDLE_CALLBACK_INFO;

*/

#define IOCTL_INTERNAL_USB_SUBMIT_IDLE_NOTIFICATION   \
                                            CTL_CODE(FILE_DEVICE_USB,  \
                                            USB_IDLE_NOTIFICATION,  \
                                            METHOD_NEITHER,  \
                                            FILE_ANY_ACCESS)

/* IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE

Routine Description:
This function returns the device handle associated with the callers PDO. The
device handle is an opaque structure that is used as a parameter for other
APIs/

Parameters:
ioStackLocation->Parameters.Others.Argument1
A pointer to a device handle (pointer to pointer).
*/

#define IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE     CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_GET_DEVICE_HANDLE, \
                                                    METHOD_NEITHER,  \
                                                    FILE_ANY_ACCESS)

#endif

/*
   USB kernel Mode IOCTLS defined for windows Longhorn and later
*/

#if (_WIN32_WINNT >= 0x0600)


#define IOCTL_INTERNAL_USB_NOTIFY_IDLE_READY     CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_IDLE_NOTIFICATION_EX, \
                                                    METHOD_NEITHER,  \
                                                    FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_REQ_GLOBAL_SUSPEND    CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_REQ_GLOBAL_SUSPEND, \
                                                    METHOD_NEITHER,  \
                                                    FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_REQ_GLOBAL_RESUME     CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_REQ_GLOBAL_RESUME, \
                                                    METHOD_NEITHER,  \
                                                    FILE_ANY_ACCESS)

/* IOCTL_INTERNAL_USB_RECORD_FAILURE

*/

#ifdef USB20_API
typedef struct _USB_START_FAILDATA {
    ULONG LengthInBytes;
    NTSTATUS NtStatus;
    USBD_STATUS UsbdStatus;
    ULONG ConnectStatus;
    UCHAR DriverData[4];
} USB_START_FAILDATA, *PUSB_START_FAILDATA;
#endif

#define IOCTL_INTERNAL_USB_RECORD_FAILURE        CTL_CODE(FILE_DEVICE_USB,  \
                                                    USB_RECORD_FAILURE, \
                                                    METHOD_NEITHER,  \
                                                    FILE_ANY_ACCESS)


#define IOCTL_INTERNAL_USB_GET_DEVICE_HANDLE_EX   CTL_CODE(FILE_DEVICE_USB,  \
                                                      USB_GET_DEVICE_HANDLE_EX, \
                                                      METHOD_NEITHER,  \
                                                      FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_GET_TT_DEVICE_HANDLE   CTL_CODE(FILE_DEVICE_USB,  \
                                                      USB_GET_TT_DEVICE_HANDLE, \
                                                      METHOD_NEITHER,  \
                                                      FILE_ANY_ACCESS)

/*  IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS  */

typedef struct _USB_TOPOLOGY_ADDRESS {
        ULONG PciBusNumber;
        ULONG PciDeviceNumber;
        ULONG PciFunctionNumber;
        ULONG Reserved;
        USHORT RootHubPortNumber;
        USHORT HubPortNumber[5];
        USHORT Reserved2;
} USB_TOPOLOGY_ADDRESS, *PUSB_TOPOLOGY_ADDRESS;

#define IOCTL_INTERNAL_USB_GET_TOPOLOGY_ADDRESS   CTL_CODE(FILE_DEVICE_USB,  \
                                                      USB_GET_TOPOLOGY_ADDRESS, \
                                                      METHOD_NEITHER,  \
                                                      FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO    CTL_CODE(FILE_DEVICE_USB,  \
                                                      USB_GET_HUB_CONFIG_INFO, \
                                                      METHOD_NEITHER,  \
                                                      FILE_ANY_ACCESS)

#endif


#if (NTDDI_VERSION >= NTDDI_WIN8)
/*
   USB kernel Mode IOCTLS defined for Win8 and later.
   Pivot on NTDDI_VERSION instead of _WIN32_WINNT
   Note: These ioctls use FILE_DEVICE_USBEX.
   All previously defined USB Ioctls use FILE_DEVICE_USB (aka FILE_DEVICE_UNKNOWN).
*/

#define IOCTL_INTERNAL_USB_REGISTER_COMPOSITE_DEVICE CTL_CODE(FILE_DEVICE_USBEX,\
                                                      USB_REGISTER_COMPOSITE_DEVICE, \
                                                      METHOD_NEITHER, \
                                                      FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_UNREGISTER_COMPOSITE_DEVICE CTL_CODE(FILE_DEVICE_USBEX,\
                                                        USB_UNREGISTER_COMPOSITE_DEVICE, \
                                                        METHOD_NEITHER, \
                                                        FILE_ANY_ACCESS)

#define IOCTL_INTERNAL_USB_REQUEST_REMOTE_WAKE_NOTIFICATION CTL_CODE(FILE_DEVICE_USBEX,\
                                                             USB_REQUEST_REMOTE_WAKE_NOTIFICATION, \
                                                             METHOD_NEITHER, \
                                                             FILE_ANY_ACCESS)

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10)

/* IOCTL_INTERNAL_USB_FAIL_GET_STATUS_FROM_DEVICE

    This IOCTL is used by kernel mode drivers to inform HUB driver
    that device doesn't support IOCTL_INTERNAL_USB_GET_STATUS_FROM_DEVICE
*/

#define IOCTL_INTERNAL_USB_FAIL_GET_STATUS_FROM_DEVICE CTL_CODE(FILE_DEVICE_USB,  \
                                                                USB_FAIL_GET_STATUS, \
                                                                METHOD_NEITHER,  \
                                                                FILE_ANY_ACCESS)

#endif

/*
   USB user mode IOCTLS
*/
#ifndef USB_KERNEL_IOCTL

/**************************************************************************
The following IOCTLS are always sent to the HCD symbolic name
***************************************************************************/

/* IOCTL_USB_HCD_GET_STATS_1 (OPTIONAL)

    The following IOCTL is used to return internal statictics
    for HCDs

*/

#define IOCTL_USB_HCD_GET_STATS_1           CTL_CODE(FILE_DEVICE_USB,  \
                                                HCD_GET_STATS_1,  \
                                                METHOD_BUFFERED,  \
                                                FILE_ANY_ACCESS)

/* IOCTL_USB_HCD_GET_STATS_2 (OPTIONAL)

    The following IOCTL is used to return internal statictics
    for HCDs

*/

#define IOCTL_USB_HCD_GET_STATS_2           CTL_CODE(FILE_DEVICE_USB,  \
                                                HCD_GET_STATS_2,  \
                                                METHOD_BUFFERED,  \
                                                FILE_ANY_ACCESS)

#define IOCTL_USB_HCD_DISABLE_PORT          CTL_CODE(FILE_DEVICE_USB, \
                                                HCD_DISABLE_PORT, \
                                                METHOD_BUFFERED, \
                                                FILE_ANY_ACCESS)

#define IOCTL_USB_HCD_ENABLE_PORT           CTL_CODE(FILE_DEVICE_USB, \
                                                HCD_ENABLE_PORT, \
                                                METHOD_BUFFERED, \
                                                FILE_ANY_ACCESS)


/*
   These ioctls are used for USB diagnostic and test applications
*/
#ifndef IOCTL_USB_DIAGNOSTIC_MODE_ON
#define IOCTL_USB_DIAGNOSTIC_MODE_ON   CTL_CODE(FILE_DEVICE_USB,  \
                                                HCD_DIAGNOSTIC_MODE_ON,  \
                                                METHOD_BUFFERED,  \
                                                FILE_ANY_ACCESS)
#endif

#ifndef IOCTL_USB_DIAGNOSTIC_MODE_OFF
#define IOCTL_USB_DIAGNOSTIC_MODE_OFF  CTL_CODE(FILE_DEVICE_USB,  \
                                                HCD_DIAGNOSTIC_MODE_OFF,  \
                                                METHOD_BUFFERED,  \
                                                FILE_ANY_ACCESS)
#endif

#ifndef IOCTL_USB_GET_ROOT_HUB_NAME
#define IOCTL_USB_GET_ROOT_HUB_NAME    CTL_CODE(FILE_DEVICE_USB,  \
                                                HCD_GET_ROOT_HUB_NAME,  \
                                                METHOD_BUFFERED,  \
                                                FILE_ANY_ACCESS)
#endif

#ifndef IOCTL_GET_HCD_DRIVERKEY_NAME
#define IOCTL_GET_HCD_DRIVERKEY_NAME   CTL_CODE(FILE_DEVICE_USB,  \
                                                HCD_GET_DRIVERKEY_NAME,  \
                                                METHOD_BUFFERED,  \
                                                FILE_ANY_ACCESS)
#endif

/**************************************************************************
The following IOCTLS are always sent to symbolic names
created by usbhub
***************************************************************************/

/*
    Utility IOCTLS supported by the hub device
*/

/*
   These ioctls are supported by the hub driver for
   use by user mode USB utilities.
*/

/*
IOCTL_USB_GET_NODE_INFORMATION

Routine Description:
Returns information about the USB hub in the user buffer passed in.  If the
IOCTL is sent to the hub NodeType is set to UsbHub and USB_HUB_INFORMATION is
returned this includes the hub descriptor and a flag indicating if the hub is
bus vs self powered .
This API returns FAILURE (STATUS_UNSUCCESSFUL) if the hub is not started or
otherwise not functional.

Parameters:
Input:
Irp->AssociatedIrp.SystemBuffer - pointer to USB_NODE_INFORMATION structure

Ouput:
USB_NODE_INFORMATION filled in as appropriate
*/
#define IOCTL_USB_GET_NODE_INFORMATION   \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_NODE_INFORMATION,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)


/*
IOCTL_USB_GET_NODE_CONNECTION_INFORMATION

Exactly the same as _EX but the speed field is a Boolean
LowSpeed
-TRUE if the device is low speed.
*/
#define IOCTL_USB_GET_NODE_CONNECTION_INFORMATION  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_NODE_CONNECTION_INFORMATION,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#define IOCTL_USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION   \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_DESCRIPTOR_FROM_NODE_CONNECTION,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_NAME    \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_NODE_CONNECTION_NAME,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#define IOCTL_USB_DIAG_IGNORE_HUBS_ON   CTL_CODE(FILE_DEVICE_USB,  \
                                USB_DIAG_IGNORE_HUBS_ON,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#define IOCTL_USB_DIAG_IGNORE_HUBS_OFF  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_DIAG_IGNORE_HUBS_OFF,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_DRIVERKEY_NAME  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_NODE_CONNECTION_DRIVERKEY_NAME,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

/*
*  IOCTLS defined for Windows XP and later
*/

#if (_WIN32_WINNT >= 0x0501)

#define IOCTL_USB_GET_HUB_CAPABILITIES  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_HUB_CAPABILITIES,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#define IOCTL_USB_HUB_CYCLE_PORT  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_HUB_CYCLE_PORT,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

/*
IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES

Routine Description:
Returns the Microsoft extended port attributes for a specific port. The
caller inputs the port number as the 'ConnectionIndex'.  Microsoft extended
port attributes are defined in the Extended Port Attribute specification.
This API also returns the current connection status of the port.

This API returns FAILURE (STATUS_UNSUCCESSFUL) if the hub is not started or
otherwise not functional.

Parameters:
Input:
Irp->AssociatedIrp.SystemBuffer - pointer to
Struct  _USB_NODE_CONNECTION_INFORMATION {
    ULONG ConnectionIndex;
}
ConnectionIndex
-is the one based port number.

Ouput: (if a device is attached)
ConnectionStatus
-The current USB connection status. Indicates things like enumeration failure
or overcurrent.
PortAttributes
-Extended port attributes defined in usb.h.
*/

#define IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_NODE_CONNECTION_ATTRIBUTES,\
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

/*
IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX

Routine Description:
Returns information about a specific USB hub port (AKA connection).  If there
is a device connected to the port information about the device is also
returned. The caller inputs the port number as the 'ConnectionIndex'.

This API returns FAILURE (STATUS_UNSUCCESSFUL) if the hub is not started or
otherwise not functional.

Parameters:
Input:
Irp->AssociatedIrp.SystemBuffer - pointer to
Struct  _USB_NODE_CONNECTION_INFORMATION {
    ULONG ConnectionIndex;
    USB_DEVICE_DESCRIPTOR DeviceDescriptor;
    UCHAR CurrentConfigurationValue;
    BOOLEAN LowSpeed;
    BOOLEAN DeviceIsHub;
    USHORT DeviceAddress;
    ULONG NumberOfOpenPipes;
    USB_CONNECTION_STATUS ConnectionStatus;
    USB_PIPE_INFO PipeList[0];
}
ConnectionIndex
-is the one based port number.

Ouput: (if a device is attached)
DeviceDescriptor
-USB device descriptor.
CurrentConfigurationValue
-Currently selected configuration value.
Speed
-indicates the 'current' operating speed, note that high speed devices can
operate at full speed when necessary.
DeviceIsHub
- TRUE if the attached device is a hub
DeviceAddress
- USB assigned device address.
NumberOfOpenPipes
- Number of open USB pipes in the current configuration
ConnectionStatus
- The current USB connection status.
USB_PIPE_INFO PipeList[0];
- list of open pipes including schedule offset and endpoint descriptor.  This
information can be used to calculate bandwidthusage.
*/

#define IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                USB_GET_NODE_CONNECTION_INFORMATION_EX,  \
                                METHOD_BUFFERED,  \
                                FILE_ANY_ACCESS)

#endif

/*
* The following IOCTLS are defined for Windows Longhorn and Later
*/
#if (_WIN32_WINNT >= 0x0600)

#define IOCTL_USB_RESET_HUB  \
            CTL_CODE(FILE_DEVICE_USB,  \
                    USB_RESET_HUB,\
                    METHOD_BUFFERED,  \
                    FILE_ANY_ACCESS)

#define IOCTL_USB_GET_HUB_CAPABILITIES_EX  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_HUB_CAPABILITIES_EX,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)
#endif

/*
* The following IOCTLs are defined for Windows 8 and later
*/
#if (NTDDI_VERSION >= NTDDI_WIN8)

#define IOCTL_USB_GET_HUB_INFORMATION_EX  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                   USB_GET_HUB_INFORMATION_EX,\
                                   METHOD_BUFFERED,  \
                                   FILE_ANY_ACCESS)

#define IOCTL_USB_GET_PORT_CONNECTOR_PROPERTIES  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_PORT_CONNECTOR_PROPERTIES,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX_V2  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_NODE_CONNECTION_INFORMATION_EX_V2,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)
#endif

#endif //#ifndef USB_KERNEL_IOCTL

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#define IOCTL_USB_GET_TRANSPORT_CHARACTERISTICS  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_TRANSPORT_CHARACTERISTICS,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_REGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE   \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_NOTIFY_ON_TRANSPORT_CHARACTERISTICS_CHANGE,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE   \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_UNREGISTER_FOR_TRANSPORT_CHARACTERISTICS_CHANGE,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_START_TRACKING_FOR_TIME_SYNC  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_START_TRACKING_FOR_TIME_SYNC,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_STOP_TRACKING_FOR_TIME_SYNC  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_STOP_TRACKING_FOR_TIME_SYNC,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_GET_DEVICE_CHARACTERISTICS  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_DEVICE_CHARACTERISTICS,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#define IOCTL_USB_GET_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION  \
                                CTL_CODE(FILE_DEVICE_USB,  \
                                    USB_GET_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION,  \
                                    METHOD_BUFFERED,  \
                                    FILE_ANY_ACCESS)

#endif

#ifndef USB_KERNEL_IOCTL

/*
   structures for user mode ioctls
*/

#include <pshpack1.h>

typedef enum _USB_HUB_NODE {
    UsbHub,
    UsbMIParent
} USB_HUB_NODE;

typedef struct _USB_HUB_INFORMATION {
    /*
       copy of data from hub descriptor
    */
    USB_HUB_DESCRIPTOR HubDescriptor;

    BOOLEAN HubIsBusPowered;

} USB_HUB_INFORMATION, *PUSB_HUB_INFORMATION;

typedef struct _USB_MI_PARENT_INFORMATION {
    ULONG NumberOfInterfaces;
} USB_MI_PARENT_INFORMATION, *PUSB_MI_PARENT_INFORMATION;

typedef struct _USB_NODE_INFORMATION {
    USB_HUB_NODE NodeType;        /* hub, mi parent */
    union {
        USB_HUB_INFORMATION HubInformation;
        USB_MI_PARENT_INFORMATION MiParentInformation;
    } u;
} USB_NODE_INFORMATION, *PUSB_NODE_INFORMATION;

typedef struct _USB_PIPE_INFO {
    USB_ENDPOINT_DESCRIPTOR EndpointDescriptor;
    ULONG ScheduleOffset;
} USB_PIPE_INFO, *PUSB_PIPE_INFO;


#if (_WIN32_WINNT >= 0x0600)
/*
    For Windows Longhorn
*/

typedef enum _USB_CONNECTION_STATUS {
    NoDeviceConnected,
    DeviceConnected,

    /* failure codes, these map to fail reasons */
    DeviceFailedEnumeration,
    DeviceGeneralFailure,
    DeviceCausedOvercurrent,
    DeviceNotEnoughPower,
    DeviceNotEnoughBandwidth,
    DeviceHubNestedTooDeeply,
    DeviceInLegacyHub,
    DeviceEnumerating,
    DeviceReset
} USB_CONNECTION_STATUS, *PUSB_CONNECTION_STATUS;

#elif (_WIN32_WINNT >= 0x0501)

/*
    For Windows XP
*/

typedef enum _USB_CONNECTION_STATUS {
    NoDeviceConnected,
    DeviceConnected,

    /* failure codes, these map to fail reasons */
    DeviceFailedEnumeration,
    DeviceGeneralFailure,
    DeviceCausedOvercurrent,
    DeviceNotEnoughPower,
    DeviceNotEnoughBandwidth,
    DeviceHubNestedTooDeeply,
    DeviceInLegacyHub
} USB_CONNECTION_STATUS, *PUSB_CONNECTION_STATUS;

#else

/*
    For Windows 2000
*/

typedef enum _USB_CONNECTION_STATUS {
    NoDeviceConnected,
    DeviceConnected,

    /* failure codes, these map to fail reasons */
    DeviceFailedEnumeration,
    DeviceGeneralFailure,
    DeviceCausedOvercurrent,
    DeviceNotEnoughPower,
    DeviceNotEnoughBandwidth
} USB_CONNECTION_STATUS, *PUSB_CONNECTION_STATUS;

#endif


/** IOCTL_USB_GET_NODE_CONNECTION_INFORMATION **/
#pragma warning( disable : 4200 )
#pragma warning( disable : 4201 )
typedef struct _USB_NODE_CONNECTION_INFORMATION {
    ULONG ConnectionIndex;  /* INPUT */
    /* usb device descriptor returned by this device
       during enumeration */
    USB_DEVICE_DESCRIPTOR DeviceDescriptor; /* OUTPUT */
    UCHAR CurrentConfigurationValue;/* OUTPUT */
    BOOLEAN LowSpeed;/* OUTPUT */
    BOOLEAN DeviceIsHub;/* OUTPUT */
    USHORT DeviceAddress;/* OUTPUT */
    ULONG NumberOfOpenPipes;/* OUTPUT */
    USB_CONNECTION_STATUS ConnectionStatus;/* OUTPUT */
    USB_PIPE_INFO PipeList[0];/* OUTPUT */
} USB_NODE_CONNECTION_INFORMATION, *PUSB_NODE_CONNECTION_INFORMATION;
#pragma warning( default : 4200 )
#pragma warning( default : 4201 )

/** IOCTL_USB_GET_NODE_CONNECTION_DRIVERKEY_NAME **/
typedef struct _USB_NODE_CONNECTION_DRIVERKEY_NAME {
    ULONG ConnectionIndex;  /* INPUT */
    ULONG ActualLength;     /* OUTPUT */
    /* unicode name for the devnode */
    WCHAR DriverKeyName[1]; /* OUTPUT */
} USB_NODE_CONNECTION_DRIVERKEY_NAME, *PUSB_NODE_CONNECTION_DRIVERKEY_NAME;

/** IOCTL_USB_GET_NODE_CONNECTION_NAME **/
typedef struct _USB_NODE_CONNECTION_NAME {
    ULONG ConnectionIndex;  /* INPUT */
    ULONG ActualLength;     /* OUTPUT */
    /* unicode symbolic name for this node if it is a hub or parent driver
       null if this node is a device. */
    WCHAR NodeName[1];      /* OUTPUT */
} USB_NODE_CONNECTION_NAME, *PUSB_NODE_CONNECTION_NAME;

typedef struct _USB_HUB_NAME {
    ULONG ActualLength;     /* OUTPUT */
    /* NULL terminated unicode symbolic name for the root hub */
    WCHAR HubName[1];       /* OUTPUT */
} USB_HUB_NAME, *PUSB_HUB_NAME;

typedef struct _USB_ROOT_HUB_NAME {
    ULONG ActualLength;     /* OUTPUT */
    /* NULL terminated unicode symbolic name for the root hub */
    WCHAR RootHubName[1];   /* OUTPUT */
} USB_ROOT_HUB_NAME, *PUSB_ROOT_HUB_NAME;

typedef struct _USB_HCD_DRIVERKEY_NAME {
    ULONG ActualLength;     /* OUTPUT */
    /* NULL terminated unicode driverkeyname for hcd */
    WCHAR DriverKeyName[1]; /* OUTPUT */
} USB_HCD_DRIVERKEY_NAME, *PUSB_HCD_DRIVERKEY_NAME;

#pragma warning( disable : 4200 )
typedef struct _USB_DESCRIPTOR_REQUEST {
    ULONG ConnectionIndex;
    struct {
        UCHAR bmRequest;
        UCHAR bRequest;
        USHORT wValue;
        USHORT wIndex;
        USHORT wLength;
    } SetupPacket;
    UCHAR Data[0];
} USB_DESCRIPTOR_REQUEST, *PUSB_DESCRIPTOR_REQUEST;
#pragma warning( default : 4200 )

/*
    Structures defined for Windows XP and later only
*/

#if (_WIN32_WINNT >= 0x0501)

typedef struct _USB_HUB_CAPABILITIES {
    /*
        Unlike the USB_HUB_INFORMATION structure used by
        IOCTL_USB_GET_NODE_INFORMATION, this structure can be extended in the
        future to accomodate more data.  The IOCTL will return only as much
        data as indicated by the size of the request buffer, to maintain
        backward compatibility with older callers that don't know about the
        new data.
    */

    ULONG HubIs2xCapable:1;

} USB_HUB_CAPABILITIES, *PUSB_HUB_CAPABILITIES;


/** IOCTL_USB_GET_NODE_CONNECTION_ATTRIBUTES **/

typedef struct _USB_NODE_CONNECTION_ATTRIBUTES {
    /* one based port number */
    ULONG ConnectionIndex;  /* INPUT */
    /* current USB connect status for the port*/
    USB_CONNECTION_STATUS ConnectionStatus; /* OUTPUT */
    /* extended port attributes defined in usb.h*/
    ULONG PortAttributes; /* OUTPUT */
} USB_NODE_CONNECTION_ATTRIBUTES, *PUSB_NODE_CONNECTION_ATTRIBUTES;

/** IOCTL_USB_GET_NODE_CONNECTION_INFORMATION_EX **/
#pragma warning( disable : 4200 )
typedef struct _USB_NODE_CONNECTION_INFORMATION_EX {
    ULONG ConnectionIndex;  /* INPUT */
    /* usb device descriptor returned by this device
       during enumeration */
    USB_DEVICE_DESCRIPTOR DeviceDescriptor;/* OUTPUT */
    UCHAR CurrentConfigurationValue;/* OUTPUT */
    /* values for the speed field are defined in USB200.h */
    UCHAR Speed;/* OUTPUT */
    BOOLEAN DeviceIsHub;/* OUTPUT */
    USHORT DeviceAddress;/* OUTPUT */
    ULONG NumberOfOpenPipes;/* OUTPUT */
    USB_CONNECTION_STATUS ConnectionStatus;/* OUTPUT */
    USB_PIPE_INFO PipeList[0];/* OUTPUT */
} USB_NODE_CONNECTION_INFORMATION_EX, *PUSB_NODE_CONNECTION_INFORMATION_EX;

C_ASSERT(sizeof(USB_NODE_CONNECTION_INFORMATION_EX) == \
    sizeof(USB_NODE_CONNECTION_INFORMATION));
#pragma warning( default : 4200 )
#endif

/*
  Structures defined for windows Longhorn and later only
*/

#if (_WIN32_WINNT >= 0x0600)
#pragma warning( disable : 4201 )
typedef union _USB_HUB_CAP_FLAGS {
    ULONG ul;
    struct {
        ULONG HubIsHighSpeedCapable:1;
        ULONG HubIsHighSpeed:1;
        ULONG HubIsMultiTtCapable:1;
        ULONG HubIsMultiTt:1;
        ULONG HubIsRoot:1;
        ULONG HubIsArmedWakeOnConnect:1;
        ULONG HubIsBusPowered:1;
        ULONG ReservedMBZ:25;
    };

} USB_HUB_CAP_FLAGS, *PUSB_HUB_CAP_FLAGS;

#pragma warning( default : 4201 )

C_ASSERT(sizeof(USB_HUB_CAP_FLAGS) == sizeof(ULONG));

typedef struct _USB_HUB_CAPABILITIES_EX {

    USB_HUB_CAP_FLAGS CapabilityFlags;

} USB_HUB_CAPABILITIES_EX, *PUSB_HUB_CAPABILITIES_EX;

typedef struct _USB_CYCLE_PORT_PARAMS {
    /* one based port number */
    ULONG ConnectionIndex;  /* INPUT */

    ULONG StatusReturned; /* OUTPUT */
} USB_CYCLE_PORT_PARAMS, *PUSB_CYCLE_PORT_PARAMS;


/************************************************************************
*     Structures used for IOCTL_INTERNAL_USB_GET_DEVICE_CONFIG_INFO
**************************************************************************/

/*
    structure for storing PnP ids
    the length includes any trailing NULLs
*/

typedef struct _USB_ID_STRING {

    USHORT LanguageId;      // laguage id where apllicable
    USHORT Pad;
    ULONG LengthInBytes;    // length of <Buffer> in Bytes includes NULLs etc
    PWCHAR Buffer;

} USB_ID_STRING, *PUSB_ID_STRING;

typedef struct _USB_HUB_DEVICE_UXD_SETTINGS {
    ULONG   Version;
    GUID    PnpGuid;
    GUID    OwnerGuid;
    ULONG   DeleteOnShutdown;
    ULONG   DeleteOnReload;
    ULONG   DeleteOnDisconnect;
    ULONG   Reserved[5];
} USB_HUB_DEVICE_UXD_SETTINGS, *PUSB_HUB_DEVICE_UXD_SETTINGS;

typedef struct _HUB_DEVICE_CONFIG_INFO_V1 {
    ULONG                           Version;
    ULONG                           Length;
    USB_HUB_CAP_FLAGS               HubFlags;
    USB_ID_STRING                   HardwareIds;
    USB_ID_STRING                   CompatibleIds;
    USB_ID_STRING                   DeviceDescription;
    ULONG                           Reserved[19];
    USB_HUB_DEVICE_UXD_SETTINGS     UxdSettings;
} HUB_DEVICE_CONFIG_INFO, *PHUB_DEVICE_CONFIG_INFO;

#endif

/*
   Structure for returning HCD debug and statistic information to
   a user mode application.
*/

typedef struct _HCD_STAT_COUNTERS {
    ULONG BytesTransferred;

    USHORT IsoMissedCount;
    USHORT DataOverrunErrorCount;

    USHORT CrcErrorCount;
    USHORT ScheduleOverrunCount;

    USHORT TimeoutErrorCount;
    USHORT InternalHcErrorCount;

    USHORT BufferOverrunErrorCount;
    USHORT SWErrorCount;

    USHORT StallPidCount;
    USHORT PortDisableCount;

} HCD_STAT_COUNTERS, *PHCD_STAT_COUNTERS;


typedef struct _HCD_ISO_STAT_COUNTERS {

    USHORT  LateUrbs;
    USHORT  DoubleBufferedPackets;

    USHORT  TransfersCF_5ms;
    USHORT  TransfersCF_2ms;

    USHORT  TransfersCF_1ms;
    USHORT  MaxInterruptLatency;

    USHORT  BadStartFrame;
    USHORT  StaleUrbs;

    /* total count of packets programmed but not accessed by
       the controller either due to software scheduling
       problems or HW problems */
    USHORT  IsoPacketNotAccesed;
    USHORT  IsoPacketHWError;

    USHORT  SmallestUrbPacketCount;
    USHORT  LargestUrbPacketCount;

    USHORT IsoCRC_Error;
    USHORT IsoOVERRUN_Error;
    USHORT IsoINTERNAL_Error;
    USHORT IsoUNKNOWN_Error;

    ULONG  IsoBytesTransferred;

    /* count of packets missed due to software scheduling
       problems */
    USHORT LateMissedCount;
    /* incremented when a packet is scheduled but not
       accessed by the controller */
    USHORT HWIsoMissedCount;

    ULONG  Reserved7[8];

} HCD_ISO_STAT_COUNTERS, *PHCD_ISO_STAT_COUNTERS;



typedef struct _HCD_STAT_INFORMATION_1 {
    ULONG Reserved1;
    ULONG Reserved2;
    ULONG ResetCounters;
    LARGE_INTEGER TimeRead;
    /*
       stat registers
    */
    HCD_STAT_COUNTERS Counters;

} HCD_STAT_INFORMATION_1, *PHCD_STAT_INFORMATION_1;

typedef struct _HCD_STAT_INFORMATION_2 {
    ULONG Reserved1;
    ULONG Reserved2;
    ULONG ResetCounters;
    LARGE_INTEGER TimeRead;

    LONG LockedMemoryUsed;
    /*
       stat registers
    */
    HCD_STAT_COUNTERS Counters;
    HCD_ISO_STAT_COUNTERS IsoCounters;

} HCD_STAT_INFORMATION_2, *PHCD_STAT_INFORMATION_2;

/*
   WMI related structures
*/

/* these index in to our array of guids for the hub FDO */
#define WMI_USB_DRIVER_INFORMATION      0
#define WMI_USB_DRIVER_NOTIFICATION     1
#define WMI_USB_POWER_DEVICE_ENABLE     2
#define WMI_USB_HUB_NODE_INFORMATION    4

/* Index into array of guids for the HUB pdos */

#define WMI_USB_PERFORMANCE_INFORMATION 1
#define WMI_USB_DEVICE_NODE_INFORMATION 2

#if (_WIN32_WINNT >= 0x0501)
    /*
        Windows XP and later
    */

typedef enum _USB_NOTIFICATION_TYPE {

    /*  the following return a
        USB_CONNECTION_NOTIFICATION structure: */
    EnumerationFailure = 0,
    InsufficentBandwidth,
    InsufficentPower,
    OverCurrent,
    ResetOvercurrent,

    /* the following return a
       USB_BUS_NOTIFICATION structure:*/
    AcquireBusInfo,

    /* the following return a
      USB_ACQUIRE_INFO structure: */
    AcquireHubName,
    AcquireControllerName,

    /* the following return a
       USB_HUB_NOTIFICATION structure: */
    HubOvercurrent,
    HubPowerChange,
    HubNestedTooDeeply,
    ModernDeviceInLegacyHub

} USB_NOTIFICATION_TYPE;

#else

/*
    For Windows 2000
*/

typedef enum _USB_NOTIFICATION_TYPE {

    /*  the following return a
        USB_CONNECTION_NOTIFICATION structure: */
    EnumerationFailure = 0,
    InsufficentBandwidth,
    InsufficentPower,
    OverCurrent,
    ResetOvercurrent,

    /* the following return a
       USB_BUS_NOTIFICATION structure:*/
    AcquireBusInfo,

    /* the following return a
      USB_ACQUIRE_INFO structure: */
    AcquireHubName,
    AcquireControllerName,

    /* the following return a
       USB_HUB_NOTIFICATION structure: */
    HubOvercurrent,
    HubPowerChange

} USB_NOTIFICATION_TYPE;

#endif

typedef struct _USB_NOTIFICATION {
    /* indicates type of notification */
    USB_NOTIFICATION_TYPE NotificationType;

} USB_NOTIFICATION, *PUSB_NOTIFICATION;

/* this structure is used for connection notification
   codes */

typedef struct _USB_CONNECTION_NOTIFICATION {
    /* indicates type of notification */
    USB_NOTIFICATION_TYPE NotificationType;

    /* valid for all connection notifictaion codes,
       0 indicates global condition for hub or parent
       this value will be a port number for devices
       attached to a hub, otherwise a one based
       index if the device is a child of a composite
       parent */
    ULONG ConnectionNumber;

    /* valid for InsufficentBandwidth,
       the amount of bandwidth the device
       tried to allocate and was denied. */
    ULONG RequestedBandwidth;

    /* valid for EnumerationFailure,
       gives some indication why the device failed
       to enumerate */
    ULONG EnumerationFailReason;

    /* valid for InsufficentPower,
       the amount of power requested to configure
       this device. */
    ULONG PowerRequested;

    /* length of the UNICODE symbolic name (in bytes) for the HUB
       that this device is attached to.
       not including NULL */
    ULONG HubNameLength;

} USB_CONNECTION_NOTIFICATION, *PUSB_CONNECTION_NOTIFICATION;

/*
   This structure is used for the bus notification code 'AcquireBusInfo'
*/

typedef struct _USB_BUS_NOTIFICATION {
    /* indicates type of notification */
    USB_NOTIFICATION_TYPE NotificationType;     /* indicates type of */
                                                /* notification */
    ULONG TotalBandwidth;
    ULONG ConsumedBandwidth;

    /* length of the UNICODE symbolic name (in bytes) for the controller
       that this device is attached to.
       not including NULL */
    ULONG ControllerNameLength;

} USB_BUS_NOTIFICATION, *PUSB_BUS_NOTIFICATION;

/*
   used to acquire user mode filenames to open respective objects
*/

typedef struct _USB_ACQUIRE_INFO {
    /* indicates type of notification */
    USB_NOTIFICATION_TYPE NotificationType;
    /* TotalSize of this struct */
    ULONG TotalSize;

    WCHAR Buffer[1];
} USB_ACQUIRE_INFO, *PUSB_ACQUIRE_INFO;


/*
    Structures defined for windows Longhorn and later only
*/

#if (_WIN32_WINNT >= 0x0600)

/*
    structures used to acquire device specific info via
    GUID_USB_WMI_NODE_INFO
*/

#define USB_NODE_INFO_SIG 'USBN'

typedef enum _USB_WMI_DEVICE_NODE_TYPE {
    UsbDevice,
    HubDevice,
    CompositeDevice,
    UsbController
} USB_WMI_DEVICE_NODE_TYPE, *PUSB_WMI_DEVICE_NODE_TYPE;

typedef struct _USB_DEVICE_STATE {

    ULONG DeviceConnected:1;
    ULONG DeviceStarted:1;

} USB_DEVICE_STATE, *PUSB_DEVICE_STATE;

//
// Specific information about a hub device
//

typedef struct _USB_HUB_PORT_INFORMATION{

    USB_DEVICE_STATE DeviceState;

    USHORT PortNumber;

    USHORT DeviceAddress;

    // Legacy ConnectionIndex used with USB user IOCTLS
    ULONG ConnectionIndex;

    // Legacy ConnectionStatus
    USB_CONNECTION_STATUS ConnectionStatus;

} USB_HUB_PORT_INFORMATION, *PUSB_HUB_PORT_INFORMATION;

typedef struct _USB_HUB_DEVICE_INFO {

    // Hub Descriptor
    USB_HUB_DESCRIPTOR HubDescriptor;

    // Unique Hub number
    ULONG HubNumber;

    // Device Address
    USHORT DeviceAddress;

    // Hub power bit
    BOOLEAN HubIsSelfPowered;

    // Root hub
    BOOLEAN HubIsRootHub;

    // Hub capabilities
    USB_HUB_CAPABILITIES HubCapabilities;

    // Number of hub ports
    ULONG NumberOfHubPorts;

    // Variable length array of info about hub ports
    USB_HUB_PORT_INFORMATION PortInfo[1];

} USB_HUB_DEVICE_INFO, *PUSB_HUB_DEVICE_INFO;

//
// Specific info about a composite device
//

typedef struct _USB_COMPOSITE_FUNCTION_INFO{

    UCHAR FunctionNumber;

    UCHAR BaseInterfaceNumber;

    UCHAR NumberOfInterfaces;

    BOOLEAN FunctionIsIdle;

} USB_COMPOSITE_FUNCTION_INFO, *PUSB_COMPOSITE_FUNCTION_INFO;


typedef struct _USB_COMPOSITE_DEVICE_INFO {

    // USB Device Descriptor
    USB_DEVICE_DESCRIPTOR DeviceDescriptor;

    // Usb Configuration Descriptor
    USB_CONFIGURATION_DESCRIPTOR CurrentConfigDescriptor;

    // 0-based configuration number
    UCHAR CurrentConfigurationValue;

    // Number of composite PDOs
    UCHAR NumberOfFunctions;

    USB_COMPOSITE_FUNCTION_INFO FunctionInfo[1];

} USB_COMPOSITE_DEVICE_INFO, *PUSB_COMPOSITE_DEVICE_INFO;

//
// Specific info about a USB controller
//

typedef struct _USB_CONTROLLER_DEVICE_INFO {

    ULONG PciVendorId;
    ULONG PciDeviceId;
    ULONG PciRevision;

    ULONG NumberOfRootPorts;

    ULONG HcFeatureFlags;

} USB_CONTROLLER_DEVICE_INFO, *PUSB_CONTROLLER_DEVICE_INFO;

//
// Specific info about a connected USB device
//

typedef struct _USB_DEVICE_INFO{

    // Device State
    USB_DEVICE_STATE DeviceState;

    // Hub Port Number
    USHORT PortNumber;

    // USB Device Descriptor
    USB_DEVICE_DESCRIPTOR DeviceDescriptor;

    // Current configuration value
    UCHAR CurrentConfigurationValue;

    // Device speed
    USB_DEVICE_SPEED Speed;

    // Device Address
    USHORT DeviceAddress;

    // Legacy ConnectionIndex used with USB user IOCTLS
    ULONG ConnectionIndex;

    // Legacy ConnectionStatus
    USB_CONNECTION_STATUS ConnectionStatus;

    // PNP HardwareID in multi-string format
    WCHAR PnpHardwareId[128];

    // PNP Compatible ID in multi-string format
    WCHAR PnpCompatibleId[128];

    // USB Serial Number string if present
    WCHAR SerialNumberId[128];

    // PNP Device Description
    WCHAR PnpDeviceDescription[128];

    // Number of pipes contained in the PipeList
    ULONG NumberOfOpenPipes;

    // Variable length list of open pipes
    USB_PIPE_INFO PipeList[1];

} USB_DEVICE_INFO, *PUSB_DEVICE_INFO;

#pragma warning( disable : 4201 )
typedef struct _USB_DEVICE_NODE_INFO {

    // Structure signature
    ULONG Sig;

    // Length of structure
    ULONG LengthInBytes;

    // Device Description
    WCHAR DeviceDescription[40];

    // Device Type
    USB_WMI_DEVICE_NODE_TYPE NodeType;

    // Bus Address
    USB_TOPOLOGY_ADDRESS BusAddress;

    // device information
    union{
        USB_DEVICE_INFO UsbDeviceInfo;
        USB_HUB_DEVICE_INFO HubDeviceInfo;
        USB_COMPOSITE_DEVICE_INFO CompositeDeviceInfo;
        USB_CONTROLLER_DEVICE_INFO ControllerDeviceInfo;
        UCHAR DeviceInformation[4];
    };

} USB_DEVICE_NODE_INFO, *PUSB_DEVICE_NODE_INFO;
#pragma warning( default : 4201 )

/*
    structures used to acquire device specific performance info
    GUID_USB_WMI_DEVICE_PERF_INFO
*/

typedef struct _USB_DEVICE_PERFORMANCE_INFO {

    //total bulk bytes transfered for this device
    ULONG BulkBytes;

    //total control bytes transfered for this device
    ULONG ControlDataBytes;

    // total iso bytes transfered for this device
    ULONG IsoBytes;

    // total interrupt bytes transfered for this device
    ULONG InterruptBytes;

    // Total number of transfer URBs processed for this device
    ULONG BulkUrbCount;
    ULONG ControlUrbCount;
    ULONG IsoUrbCount;
    ULONG InterruptUrbCount;

    // BW reported in bits/32ms(32 frames)
    // Interrupt BW is reported by endpoint period in AllocedInterrupt[n] where period = 2^n
    // The total interrupt BW allocated for this device is the sum of the BW
    // for all periods
    //
    ULONG AllocedInterrupt[6];

    // Iso BW allocated for all iso endpoints on the device.
    // Reported in bits/32ms(32 frames)
    //
    ULONG AllocedIso;

    // Total USB controller BW available in bits/32ms.
    ULONG Total32secBandwidth;

    // Total USB BW available on the device's TT in bits/32ms
    ULONG TotalTtBandwidth;

    // Text description of the device
    WCHAR DeviceDescription[60];

    // operating speed of the device
    USB_DEVICE_SPEED DeviceSpeed;

    // total number of ms iso transfers have waited after being scheduled
    ULONG TotalIsoLatency;

    // Number of ISO packets that were not scheduled or processed by the controller
    ULONG DroppedIsoPackets;

    // Number of transfers completing with an error
    ULONG TransferErrors;

    //
    //  Following values are for CONTROLLER instances only.
    //

    // Number of controller interrupts
    ULONG PciInterruptCount;

    // HC Idle State -- non zero if the HC is not running
    ULONG HcIdleState;

    // Async (EHCI) Idle State -- non zero if the async segment is off
    ULONG HcAsyncIdleState;

    // Async (EHCI) Cache stats -- incremented each time we flush the async cache (doorbell).
    ULONG HcAsyncCacheFlushCount;

    // Periodic (EHCI) Idle State -- non zero if the periodic segment is off
    ULONG HcPeriodicIdleState;

    // Periodic (EHCI) Cache stats -- incremented each time we flush the periodic prefetch cache.
    ULONG HcPeriodicCacheFlushCount;

} USB_DEVICE_PERFORMANCE_INFO, *PUSB_DEVICE_PERFORMANCE_INFO;

#endif


/*
    Structures defined for Windows 8 and later only
*/
#if (NTDDI_VERSION >= NTDDI_WIN8)

typedef enum _USB_HUB_TYPE {
    UsbRootHub        = 1,
    Usb20Hub          = 2,
    Usb30Hub          = 3
} USB_HUB_TYPE;


typedef struct _USB_HUB_INFORMATION_EX {
    USB_HUB_TYPE             HubType;

    // The higest valid port number on the hub
    USHORT   HighestPortNumber;

    union {
        USB_HUB_DESCRIPTOR  UsbHubDescriptor;
        USB_30_HUB_DESCRIPTOR  Usb30HubDescriptor;
    } u;

} USB_HUB_INFORMATION_EX, *PUSB_HUB_INFORMATION_EX;


#pragma warning( disable : 4201 )

typedef union _USB_PORT_PROPERTIES {
    ULONG  ul;

    struct {
        ULONG PortIsUserConnectable  :1;
        ULONG PortIsDebugCapable  :1;
        ULONG PortHasMultipleCompanions  :1;
        ULONG PortConnectorIsTypeC  :1;
        ULONG ReservedMBZ  :28;
    };

} USB_PORT_PROPERTIES, *PUSB_PORT_PROPERTIES;

#pragma warning( default : 4201 )

typedef struct _USB_PORT_CONNECTOR_PROPERTIES {
    // one based port number
    ULONG  ConnectionIndex;

    // The number of bytes required to hold the entire USB_PORT_CONNECTOR_PROPERTIES
    // structure, including the full CompanionHubSymbolicLinkName string
    ULONG  ActualLength;

    // bitmask of flags indicating properties and capabilities of the port
    USB_PORT_PROPERTIES  UsbPortProperties;

    // Zero based index number of the companion port being queried.
    USHORT                CompanionIndex;

    // Port number of the companion port
    USHORT                CompanionPortNumber;

    // Symbolic link name for the companion hub
    WCHAR                 CompanionHubSymbolicLinkName[1];
} USB_PORT_CONNECTOR_PROPERTIES, *PUSB_PORT_CONNECTOR_PROPERTIES;

#pragma warning( disable : 4201 )

typedef union _USB_PROTOCOLS {
    ULONG  ul;

    struct {
        ULONG Usb110 :1;
        ULONG Usb200 :1;
        ULONG Usb300 :1;
        ULONG ReservedMBZ  :29;
    };

} USB_PROTOCOLS, *PUSB_PROTOCOLS;


typedef union _USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS {
    ULONG  ul;

    struct {
        ULONG DeviceIsOperatingAtSuperSpeedOrHigher  :1;
        ULONG DeviceIsSuperSpeedCapableOrHigher  :1;
        ULONG DeviceIsOperatingAtSuperSpeedPlusOrHigher  :1;
        ULONG DeviceIsSuperSpeedPlusCapableOrHigher  :1;
        ULONG ReservedMBZ  :28;
    };

} USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS, *PUSB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS;

#pragma warning( default : 4201 )


typedef struct _USB_NODE_CONNECTION_INFORMATION_EX_V2 {
    // one based port number
    ULONG  ConnectionIndex;

    // length of the structure
    ULONG  Length;

    // On input a bitmask that indicates which USB protocols are understood by the caller
    // On output a bitmask that indicates which USB signaling protocols are supported by the port
    USB_PROTOCOLS SupportedUsbProtocols;

    // A bitmask indicating properties of the connected device or port
    USB_NODE_CONNECTION_INFORMATION_EX_V2_FLAGS Flags;
} USB_NODE_CONNECTION_INFORMATION_EX_V2, *PUSB_NODE_CONNECTION_INFORMATION_EX_V2;

#endif

#include <poppack.h>

#endif //#if USB_KERNEL_IOCTL

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

#include <pshpack1.h>

#define USB_TRANSPORT_CHARACTERISTICS_VERSION_1                 0x01

#define USB_TRANSPORT_CHARACTERISTICS_LATENCY_AVAILABLE     0x1
#define USB_TRANSPORT_CHARACTERISTICS_BANDWIDTH_AVAILABLE   0x2

typedef struct _USB_TRANSPORT_CHARACTERISTICS {
    ULONG   Version;
    ULONG   TransportCharacteristicsFlags;
    ULONG64 CurrentRoundtripLatencyInMilliSeconds;
    ULONG64 MaxPotentialBandwidth;
} USB_TRANSPORT_CHARACTERISTICS, *PUSB_TRANSPORT_CHARACTERISTICS;

#define USB_REGISTER_FOR_TRANSPORT_LATENCY_CHANGE   0x1
#define USB_REGISTER_FOR_TRANSPORT_BANDWIDTH_CHANGE 0x2

DECLARE_HANDLE(USB_CHANGE_REGISTRATION_HANDLE);

typedef struct _USB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION {
    ULONG ChangeNotificationInputFlags;
    USB_CHANGE_REGISTRATION_HANDLE Handle;
    USB_TRANSPORT_CHARACTERISTICS UsbTransportCharacteristics;
} USB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION, *PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_REGISTRATION;

typedef struct _USB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION {
    USB_CHANGE_REGISTRATION_HANDLE Handle;
    USB_TRANSPORT_CHARACTERISTICS UsbTransportCharacteristics;
} USB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION, *PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_NOTIFICATION;

typedef struct _USB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION {
    USB_CHANGE_REGISTRATION_HANDLE Handle;
} USB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION, *PUSB_TRANSPORT_CHARACTERISTICS_CHANGE_UNREGISTRATION;

#define USB_DEVICE_CHARACTERISTICS_VERSION_1                    0x01

#define USB_DEVICE_CHARACTERISTICS_MAXIMUM_PATH_DELAYS_AVAILABLE 	0x1

typedef struct _USB_DEVICE_CHARACTERISTICS {
    ULONG                   Version;
    ULONG                   Reserved[2];
    ULONG                   UsbDeviceCharacteristicsFlags;
    ULONG                   MaximumSendPathDelayInMilliSeconds;
    ULONG                   MaximumCompletionPathDelayInMilliSeconds;
} USB_DEVICE_CHARACTERISTICS, *PUSB_DEVICE_CHARACTERISTICS;


#ifndef __USB_TIME_SYNC_DEFINED
#define __USB_TIME_SYNC_DEFINED

typedef struct _USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION {

    HANDLE          TimeTrackingHandle;
    BOOLEAN         IsStartupDelayTolerable;

} USB_START_TRACKING_FOR_TIME_SYNC_INFORMATION, *PUSB_START_TRACKING_FOR_TIME_SYNC_INFORMATION;

typedef struct _USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION {

    HANDLE          TimeTrackingHandle;

} USB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION, *PUSB_STOP_TRACKING_FOR_TIME_SYNC_INFORMATION;

typedef struct _USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION {

    //
    // Input
    //

    HANDLE          TimeTrackingHandle;
    ULONG           InputFrameNumber;
    ULONG           InputMicroFrameNumber;

    //
    // Output
    //

    LARGE_INTEGER   QueryPerformanceCounterAtInputFrameOrMicroFrame;
    LARGE_INTEGER   QueryPerformanceCounterFrequency;
    ULONG           PredictedAccuracyInMicroSeconds;

    ULONG           CurrentGenerationID;
    LARGE_INTEGER   CurrentQueryPerformanceCounter;
    ULONG           CurrentHardwareFrameNumber;         // 11 bits from hardware/MFINDEX
    ULONG           CurrentHardwareMicroFrameNumber;    //  3 bits from hardware/MFINDEX
    ULONG           CurrentUSBFrameNumber;              // 32 bit USB Frame Number

} USB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION, *PUSB_FRAME_NUMBER_AND_QPC_FOR_TIME_SYNC_INFORMATION;

#endif

typedef struct _USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION {
    // one based port number
    ULONG  ConnectionIndex;

    // length of the structure
    ULONG  Length;

    // Current Operating Speed for RX Lane
    USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED RxSuperSpeedPlus;

    // Note: For 'actual' LaneCount, add 1.
    ULONG RxLaneCount;

    // Current Operating Speed for TX Lane
    USB_DEVICE_CAPABILITY_SUPERSPEEDPLUS_SPEED TxSuperSpeedPlus;

    // Note: For 'actual' LaneCount, add 1.
    ULONG TxLaneCount;
} USB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION, *PUSB_NODE_CONNECTION_SUPERSPEEDPLUS_INFORMATION;


#include <poppack.h>

#endif


#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* __USBIOCTL_H__ */


