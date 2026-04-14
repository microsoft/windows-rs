/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

        USBUSER.H

Abstract:

        This file contains USER Mode IOCTLS supported by
        the USB PORT or (HC - Host Controller) driver.

Environment:

    user mode

Revision History:


--*/

#ifndef   __USBUSER_H__
#define   __USBUSER_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


/*
    User IOCTLS defined for Windows XP and later only
*/
#if (_WIN32_WINNT >= 0x0501)

#include "usbiodef.h"

#include <PSHPACK1.H>

#define USBUSER_VERSION     0x00000004

#define IOCTL_USB_USER_REQUEST          USB_CTL(HCD_USER_REQUEST)
/*
    The following were used by test applications and are no
    longer supported (they have been replaced with USBUSER Opcodes):


#define IOCTL_USB_HCD_GET_STATS_1   USB_CTL(HCD_GET_STATS_1)
#define IOCTL_USB_HCD_GET_STATS_2   USB_CTL(HCD_GET_STATS_2)
#define IOCTL_USB_HCD_DISABLE_PORT  USB_CTL(HCD_DISABLE_PORT)
#define IOCTL_USB_HCD_ENABLE_PORT   USB_CTL(HCD_ENABLE_PORT)
*/

/*
   The following are used by the 'USBDIAG' driver
*/
#ifndef IOCTL_USB_DIAGNOSTIC_MODE_ON
#define IOCTL_USB_DIAGNOSTIC_MODE_ON    USB_CTL(HCD_DIAGNOSTIC_MODE_ON)
#endif
#ifndef IOCTL_USB_DIAGNOSTIC_MODE_OFF
#define IOCTL_USB_DIAGNOSTIC_MODE_OFF   USB_CTL(HCD_DIAGNOSTIC_MODE_OFF)
#endif

#ifndef IOCTL_USB_GET_ROOT_HUB_NAME
#define IOCTL_USB_GET_ROOT_HUB_NAME     USB_CTL(HCD_GET_ROOT_HUB_NAME)
#endif
#ifndef IOCTL_GET_HCD_DRIVERKEY_NAME
#define IOCTL_GET_HCD_DRIVERKEY_NAME    USB_CTL(HCD_GET_DRIVERKEY_NAME)
#endif

/*
    define error codes
*/
typedef enum _USB_USER_ERROR_CODE {

    UsbUserSuccess = 0,
    UsbUserNotSupported,
    UsbUserInvalidRequestCode,
    UsbUserFeatureDisabled,
    UsbUserInvalidHeaderParameter,
    UsbUserInvalidParameter,
    UsbUserMiniportError,
    UsbUserBufferTooSmall,
    UsbUserErrorNotMapped,
    UsbUserDeviceNotStarted,
    UsbUserNoDeviceConnected

} USB_USER_ERROR_CODE;

/*
    define USB USER request Codes
*/

/*
    The following APIS are enabled always
*/
#define USBUSER_GET_CONTROLLER_INFO_0           0x00000001
#define USBUSER_GET_CONTROLLER_DRIVER_KEY       0x00000002
#define USBUSER_PASS_THRU                       0x00000003
#define USBUSER_GET_POWER_STATE_MAP             0x00000004
#define USBUSER_GET_BANDWIDTH_INFORMATION       0x00000005
#define USBUSER_GET_BUS_STATISTICS_0            0x00000006
#define USBUSER_GET_ROOTHUB_SYMBOLIC_NAME       0x00000007
#define USBUSER_GET_USB_DRIVER_VERSION          0x00000008
#define USBUSER_GET_USB2_HW_VERSION             0x00000009
#define USBUSER_USB_REFRESH_HCT_REG             0x0000000a

/*
    The following APIs are only enabled when the
    devlopr key is set in the registry.
*/
#define USBUSER_OP_SEND_ONE_PACKET              0x10000001

/*
    The following APIs are only enabled when the
    root hub is disabled.
*/

#define USBUSER_OP_RAW_RESET_PORT               0x20000001
#define USBUSER_OP_OPEN_RAW_DEVICE              0x20000002
#define USBUSER_OP_CLOSE_RAW_DEVICE             0x20000003
#define USBUSER_OP_SEND_RAW_COMMAND             0x20000004

#define USBUSER_SET_ROOTPORT_FEATURE            0x20000005
#define USBUSER_CLEAR_ROOTPORT_FEATURE          0x20000006
#define USBUSER_GET_ROOTPORT_STATUS             0x20000007

#define USBUSER_INVALID_REQUEST                 0xFFFFFFF0

#define USBUSER_OP_MASK_DEVONLY_API             0x10000000
#define USBUSER_OP_MASK_HCTEST_API              0x20000000


/*
    Common Header used by all USBUSER APIS
*/

typedef struct _USBUSER_REQUEST_HEADER {
    /*
        API Requested
    */
    ULONG UsbUserRequest;
    /*
        status code returned by port driver
    */
    USB_USER_ERROR_CODE UsbUserStatusCode;
    /*
        size of client input/output buffer
        we always use the same buffer for input
        and output
    */
    ULONG RequestBufferLength;
    /*
        size of buffer required to get all of the data
    */
    ULONG ActualBufferLength;

} USBUSER_REQUEST_HEADER, *PUSBUSER_REQUEST_HEADER;


/*****************************************************
    API - Send a single usb packet on the bus

    USBUSER_OP_SEND_ONE_PACKET

    This API is used to implement the 'single step'
    USB transaction development tool.

*******************************************************/

/*
  transaction speed
*/
#define USB_PACKETFLAG_LOW_SPEED            0x00000001
#define USB_PACKETFLAG_FULL_SPEED           0x00000002
#define USB_PACKETFLAG_HIGH_SPEED           0x00000004

/*
  transaction type async(bulk, control, interrupt) or iso
*/
#define USB_PACKETFLAG_ASYNC_IN             0x00000008
#define USB_PACKETFLAG_ASYNC_OUT            0x00000010
#define USB_PACKETFLAG_ISO_IN               0x00000020
#define USB_PACKETFLAG_ISO_OUT              0x00000040
#define USB_PACKETFLAG_SETUP                0x00000080

/*
  transaction data toggle
*/
#define USB_PACKETFLAG_TOGGLE0              0x00000100
#define USB_PACKETFLAG_TOGGLE1              0x00000200

typedef struct _PACKET_PARAMETERS {

    UCHAR DeviceAddress;
    UCHAR EndpointAddress;
    USHORT MaximumPacketSize;
    /* timeout in ms, zero means default */
    /* default timeout is 10 ms */
    ULONG Timeout;
    ULONG Flags;
    ULONG DataLength;
    /* for 2.0 hubs */
    USHORT HubDeviceAddress;
    USHORT PortTTNumber;

    UCHAR ErrorCount;
    UCHAR Pad[3];

    USBD_STATUS UsbdStatusCode;
    UCHAR Data[4];

} PACKET_PARAMETERS, *PPACKET_PARAMETERS;

typedef struct _USBUSER_SEND_ONE_PACKET {

    USBUSER_REQUEST_HEADER Header;
    PACKET_PARAMETERS PacketParameters;

} USBUSER_SEND_ONE_PACKET, *PUSBUSER_SEND_ONE_PACKET;

/*****************************************************
    API - Test Reset Root Port

    USBUSER_OP_RAW_RESET_PORT
******************************************************/

typedef struct _RAW_RESET_PORT_PARAMETERS {

    USHORT PortNumber;
    USHORT PortStatus;

} RAW_RESET_PORT_PARAMETERS, *PRAW_RESET_PORT_PARAMETERS;

typedef struct _USBUSER_RAW_RESET_ROOT_PORT {

    USBUSER_REQUEST_HEADER Header;
    RAW_RESET_PORT_PARAMETERS Parameters;

} USBUSER_RAW_RESET_ROOT_PORT, *PUSBUSER_RAW_RESET_ROOT_PORT;

/*****************************************************
    API - Test Set/Clear Root Port Feature

    USBUSER_SET_ROOTPORT_FEATURE
    USBUSER_CLEAR_ROOTPORT_FEATURE
******************************************************/

typedef struct _RAW_ROOTPORT_FEATURE {

    USHORT PortNumber;
    USHORT PortFeature;
    USHORT PortStatus;

} RAW_ROOTPORT_FEATURE, *PRAW_ROOTPORT_FEATURE;

typedef struct _USBUSER_ROOTPORT_FEATURE_REQUEST {

    USBUSER_REQUEST_HEADER Header;
    RAW_ROOTPORT_FEATURE Parameters;

} USBUSER_ROOTPORT_FEATURE_REQUEST, *PUSBUSER_ROOTPORT_FEATURE_REQUEST;

/*****************************************************
    API - Get RootPort Status

    USBUSER_GET_ROOTPORT_STATUS

******************************************************/

typedef struct _RAW_ROOTPORT_PARAMETERS {

    USHORT PortNumber;
    USHORT PortStatus;

} RAW_ROOTPORT_PARAMETERS, *PRAW_ROOTPORT_PARAMETERS;

typedef struct _USBUSER_ROOTPORT_PARAMETERS {

    USBUSER_REQUEST_HEADER Header;
    RAW_ROOTPORT_PARAMETERS Parameters;

} USBUSER_ROOTPORT_PARAMETERS, *PUSBUSER_ROOTPORT_PARAMETERS;

/****************************************************
    API - Get Controller Information

    Return some information about the controller

    USBUSER_GET_CONTROLLER_INFO_0
****************************************************/

/* these flags indicate features of the HC */

#define USB_HC_FEATURE_FLAG_PORT_POWER_SWITCHING    0x00000001
#define USB_HC_FEATURE_FLAG_SEL_SUSPEND             0x00000002
#define USB_HC_FEATURE_LEGACY_BIOS                  0x00000004
#define USB_HC_FEATURE_TIME_SYNC_API                0x00000008

typedef struct _USB_CONTROLLER_INFO_0 {

    ULONG PciVendorId;
    ULONG PciDeviceId;
    ULONG PciRevision;

    ULONG NumberOfRootPorts;

    USB_CONTROLLER_FLAVOR ControllerFlavor;

    ULONG HcFeatureFlags;

} USB_CONTROLLER_INFO_0 , *PUSB_CONTROLLER_INFO_0;

typedef struct _USBUSER_CONTROLLER_INFO_0 {

    USBUSER_REQUEST_HEADER Header;
    USB_CONTROLLER_INFO_0 Info0;

} USBUSER_CONTROLLER_INFO_0, *PUSBUSER_CONTROLLER_INFO_0;

/****************************************************
    API - Get Controller Driver Key

    Returns the driver key in the registry associated
    with this controller.

    The key is returned NULL terminated, KeyLength
    is the length of the key in bytes including the
    UNICODE_NULL

    USBUSER_GET_CONTROLLER_DRIVER_KEY

    API - Get Root Hub Name

    Returns the symbolic name for the root hub on the
    host controller. Length is the length of the name
    in bytes including the NULL.

    USBUSER_GET_ROOTHUB_SYMBOLIC_NAME

----------------------------------------------------

    The following structure is used to return unicode
    names from the port driver for both of these APIs.

****************************************************/

typedef struct _USB_UNICODE_NAME {

    ULONG Length;
    WCHAR String[1];

} USB_UNICODE_NAME, *PUSB_UNICODE_NAME;

typedef struct _USBUSER_CONTROLLER_UNICODE_NAME {

    USBUSER_REQUEST_HEADER Header;
    USB_UNICODE_NAME UnicodeName;

} USBUSER_CONTROLLER_UNICODE_NAME, *PUSBUSER_CONTROLLER_UNICODE_NAME;

/****************************************************
    API - PassThru

    allows for vendor specific APIs to be passed to
    Host Controller Miniport Driver

    The vendors must pass a guid that is recognized
    by the miniport , this enures that the parameters
    are not miss-interpreted

    USBUSER_PASS_THRU
****************************************************/

typedef struct _USB_PASS_THRU_PARAMETERS {

    GUID FunctionGUID;
    ULONG ParameterLength;
    UCHAR Parameters[4];

} USB_PASS_THRU_PARAMETERS, *PUSB_PASS_THRU_PARAMETERS;

typedef struct _USBUSER_PASS_THRU_REQUEST {

    USBUSER_REQUEST_HEADER Header;
    USB_PASS_THRU_PARAMETERS PassThru;

} USBUSER_PASS_THRU_REQUEST, *PUSBUSER_PASS_THRU_REQUEST;


/****************************************************
    API - GetPowerStateMap

    Returns specific information about a controller
    and root hubs power state given a specific
    system state.

    USBUSER_GET_POWER_STATE_MAP
****************************************************/

typedef enum _WDMUSB_POWER_STATE {

    WdmUsbPowerNotMapped = 0,

    WdmUsbPowerSystemUnspecified = 100,
    WdmUsbPowerSystemWorking,
    WdmUsbPowerSystemSleeping1,
    WdmUsbPowerSystemSleeping2,
    WdmUsbPowerSystemSleeping3,
    WdmUsbPowerSystemHibernate,
    WdmUsbPowerSystemShutdown,

    WdmUsbPowerDeviceUnspecified = 200,
    WdmUsbPowerDeviceD0,
    WdmUsbPowerDeviceD1,
    WdmUsbPowerDeviceD2,
    WdmUsbPowerDeviceD3

} WDMUSB_POWER_STATE;

typedef struct _USB_POWER_INFO {

    /* input */
    WDMUSB_POWER_STATE SystemState;
    /* output */
    WDMUSB_POWER_STATE HcDevicePowerState;
    WDMUSB_POWER_STATE HcDeviceWake;
    WDMUSB_POWER_STATE HcSystemWake;

    WDMUSB_POWER_STATE RhDevicePowerState;
    WDMUSB_POWER_STATE RhDeviceWake;
    WDMUSB_POWER_STATE RhSystemWake;

    WDMUSB_POWER_STATE LastSystemSleepState;

    BOOLEAN CanWakeup;
    BOOLEAN IsPowered;

} USB_POWER_INFO, *PUSB_POWER_INFO;

typedef struct _USBUSER_POWER_INFO_REQUEST {

    USBUSER_REQUEST_HEADER Header;
    USB_POWER_INFO PowerInformation;

} USBUSER_POWER_INFO_REQUEST, *PUSBUSER_POWER_INFO_REQUEST;


/****************************************************
    API - Open Raw Device access on the bus

    USBUSER_OP_OPEN_RAW_DEVICE
****************************************************/

typedef struct _USB_OPEN_RAW_DEVICE_PARAMETERS {

    USHORT PortStatus;
    USHORT MaxPacketEp0;

} USB_OPEN_RAW_DEVICE_PARAMETERS , *PUSB_OPEN_RAW_DEVICE_PARAMETERS;

typedef struct _USBUSER_OPEN_RAW_DEVICE {

    USBUSER_REQUEST_HEADER Header;
    USB_OPEN_RAW_DEVICE_PARAMETERS Parameters;

} USBUSER_OPEN_RAW_DEVICE, *PUSBUSER_OPEN_RAW_DEVICE;

/****************************************************
    API - Close Raw Device access on the bus

    USBUSER_OP_CLOSE_RAW_DEVICE
****************************************************/

typedef struct _USB_CLOSE_RAW_DEVICE_PARAMETERS {

    ULONG xxx;

} USB_CLOSE_RAW_DEVICE_PARAMETERS , *PUSB_CLOSE_RAW_DEVICE_PARAMETERS;

typedef struct _USBUSER_CLOSE_RAW_DEVICE {

    USBUSER_REQUEST_HEADER Header;
    USB_CLOSE_RAW_DEVICE_PARAMETERS Parameters;

} USBUSER_CLOSE_RAW_DEVICE, *PUSBUSER_CLOSE_RAW_DEVICE;


/****************************************************
    API - Send control command via raw device handle

    USBUSER_OP_SEND_RAW_COMMAND
****************************************************/

typedef struct _USB_SEND_RAW_COMMAND_PARAMETERS {

    /* setup packet */
    UCHAR Usb_bmRequest;
    UCHAR Usb_bRequest;
    USHORT Usb_wVlaue;
    USHORT Usb_wIndex;
    USHORT Usb_wLength;

    /* other parameters */
    USHORT DeviceAddress;
    USHORT MaximumPacketSize;
    ULONG Timeout;
    ULONG DataLength;
    USBD_STATUS UsbdStatusCode;
    UCHAR Data[4];

} USB_SEND_RAW_COMMAND_PARAMETERS, *PUSB_SEND_RAW_COMMAND_PARAMETERS;

typedef struct _USBUSER_SEND_RAW_COMMAND {

    USBUSER_REQUEST_HEADER Header;
    USB_SEND_RAW_COMMAND_PARAMETERS Parameters;

} USBUSER_SEND_RAW_COMMAND, *PUSBUSER_SEND_RAW_COMMAND;


/****************************************************
    API - return information about allocated
        bandwidth

    USBUSER_GET_BANDWIDTH_INFORMATION
****************************************************/

typedef struct _USB_BANDWIDTH_INFO {

    ULONG DeviceCount;
    // total bandith in bits/sec
    ULONG TotalBusBandwidth;

    // allocated bandwidth based on a 32 sec
    // slice of bus time ie bits/32 sec
    ULONG Total32secBandwidth;

    ULONG AllocedBulkAndControl;
    ULONG AllocedIso;
    ULONG AllocedInterrupt_1ms;
    ULONG AllocedInterrupt_2ms;
    ULONG AllocedInterrupt_4ms;
    ULONG AllocedInterrupt_8ms;
    ULONG AllocedInterrupt_16ms;
    ULONG AllocedInterrupt_32ms;

} USB_BANDWIDTH_INFO, *PUSB_BANDWIDTH_INFO;

typedef struct _USBUSER_BANDWIDTH_INFO_REQUEST {

    USBUSER_REQUEST_HEADER Header;
    USB_BANDWIDTH_INFO BandwidthInformation;

} USBUSER_BANDWIDTH_INFO_REQUEST, *PUSBUSER_BANDWIDTH_INFO_REQUEST;


/****************************************************
    API - return information data transferred on the
        bus

    USBUSER_BUS_STATISTICS_0
****************************************************/

typedef struct _USB_BUS_STATISTICS_0 {

    ULONG DeviceCount;

    LARGE_INTEGER CurrentSystemTime;

    ULONG CurrentUsbFrame;

    ULONG BulkBytes;
    ULONG IsoBytes;
    ULONG InterruptBytes;
    ULONG ControlDataBytes;

    ULONG PciInterruptCount;
    ULONG HardResetCount;
    ULONG WorkerSignalCount;
    ULONG CommonBufferBytes;
    ULONG WorkerIdleTimeMs;

    BOOLEAN RootHubEnabled;
     // 0=D0, 1=D1, 2=D2, 3=D3
    UCHAR RootHubDevicePowerState;
    // 1 = active 0 = idle
    UCHAR Unused;
    // used to generate legacy name HCDn
    UCHAR NameIndex;

} USB_BUS_STATISTICS_0, *PUSB_BUS_STATISTICS_0;

typedef struct _USBUSER_BUS_STATISTICS_0_REQUEST {

    USBUSER_REQUEST_HEADER Header;
    USB_BUS_STATISTICS_0 BusStatistics0;

} USBUSER_BUS_STATISTICS_0_REQUEST, *PUSBUSER_BUS_STATISTICS_0_REQUEST;


/****************************************************
    API - Get USB DRIVER Version

    USBUSER_GET_USB_DRIVER_VERSION
****************************************************/

typedef struct _USB_DRIVER_VERSION_PARAMETERS {

    /* goat code for this rev of the stack */
    ULONG DriverTrackingCode;
    /* USBDI Api set supported */
    ULONG USBDI_Version;
    /* USB USER Api Set supported */
    ULONG USBUSER_Version;

    /* set to true if checked vesrion(s) on
       the stack are loaded
    */
    BOOLEAN CheckedPortDriver;
    BOOLEAN CheckedMiniportDriver;

    /* BCD usb version 0x0110 (1.1) 0x0200 (2.0) */
    USHORT USB_Version;

} USB_DRIVER_VERSION_PARAMETERS , *PUSB_DRIVER_VERSION_PARAMETERS;

typedef struct _USBUSER_GET_DRIVER_VERSION {

    USBUSER_REQUEST_HEADER Header;
    USB_DRIVER_VERSION_PARAMETERS Parameters;

} USBUSER_GET_DRIVER_VERSION, *PUSBUSER_GET_DRIVER_VERSION;

/****************************************************
    API - Get USB 2 Hardware Revision

    USBUSER_GET_USB2HW_VERSION
*****************************************************/

//#define USB2HW_UNKNOWN  0x00
//#define USB2HW_A0       0xA0
//#define USB2HW_A1       0xA1
//#define USB2HW_B0       0xB0

typedef struct _USB_USB2HW_VERSION_PARAMETERS {

    UCHAR Usb2HwRevision;

} USB_USB2HW_VERSION_PARAMETERS, *PUSB_USB2HW_VERSION_PARAMETERS;

typedef struct _USBUSER_GET_USB2HW_VERSION {

    USBUSER_REQUEST_HEADER Header;
    USB_USB2HW_VERSION_PARAMETERS Parameters;

} USBUSER_GET_USB2HW_VERSION, *PUSBUSER_GET_USB2HW_VERSION;

/****************************************************
    API - Get USB refresh global keys

    USBUSER_REFRESH_G_REG
*****************************************************/



typedef struct _USBUSER_REFRESH_HCT_REG {

    USBUSER_REQUEST_HEADER Header;
    ULONG Flags;

} USBUSER_REFRESH_HCT_REG, *PUSBUSER_REFRESH_HCT_REG;


#include <POPPACK.H>

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif //__USBUSER_H__


