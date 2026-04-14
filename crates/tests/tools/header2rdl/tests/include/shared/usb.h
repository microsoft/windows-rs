/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

        USB.H

Abstract:

   structures and APIs for USB drivers.

Environment:

    Kernel & user mode

Revision History:

    09-29-95 : created
    02-10-04 : Updated to include header versioning

--*/

#ifndef   __USB_H__
#define   __USB_H__
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef PHYSICAL_ADDRESS
typedef LARGE_INTEGER PHYSICAL_ADDRESS;
#endif

#ifndef _NTDDK_
#ifndef _WDMDDK_
typedef PVOID PIRP;
typedef PVOID PMDL;
#endif
#endif

#define USBDI_VERSION    0x00000600

#include "usbspec.h"
#ifdef _WDMDDK_
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4201) // named type definition in parentheses
#pragma warning(disable:4214) // named type definition in parentheses

#define USB_PORTATTR_NO_CONNECTOR       0x00000001
#define USB_PORTATTR_SHARED_USB2        0x00000002
#define USB_PORTATTR_MINI_CONNECTOR     0x00000004
#define USB_PORTATTR_OEM_CONNECTOR      0x00000008

#define USB_PORTATTR_OWNED_BY_CC        0x01000000
#define USB_PORTATTR_NO_OVERCURRENT_UI  0x02000000

typedef enum _USB_CONTROLLER_FLAVOR {

    USB_HcGeneric   = 0,

    OHCI_Generic    = 100,
    OHCI_Hydra,
    OHCI_NEC,

    UHCI_Generic    = 200,
    UHCI_Piix4      = 201,
    UHCI_Piix3      = 202,
    UHCI_Ich2       = 203,
    UHCI_Reserved204   = 204,
    UHCI_Ich1       = 205,
    UHCI_Ich3m      = 206,
    UHCI_Ich4       = 207,
    UHCI_Ich5       = 208,
    UHCI_Ich6       = 209,

    UHCI_Intel      = 249,

    UHCI_VIA          = 250,
    UHCI_VIA_x01      = 251,
    UHCI_VIA_x02      = 252,
    UHCI_VIA_x03      = 253,
    UHCI_VIA_x04      = 254,

    UHCI_VIA_x0E_FIFO = 264,

    EHCI_Generic    = 1000,
    EHCI_NEC        = 2000,
    EHCI_Lucent     = 3000,
    EHCI_NVIDIA_Tegra2 = 4000,
    EHCI_NVIDIA_Tegra3 = 4001,
    EHCI_Intel_Medfield = 5001

} USB_CONTROLLER_FLAVOR;

//
// USB defined structures and constants
// (see chapter 9 of USB specification)
//

#define USB_DEFAULT_DEVICE_ADDRESS     0
#define USB_DEFAULT_ENDPOINT_ADDRESS   0

//
// max packet size (bytes) for default endpoint
// until SET_ADDRESS command is received.
//

#define USB_DEFAULT_MAX_PACKET         64

//
// USBD interface structures and constants
//

#define URB_FROM_IRP(Irp) ((IoGetCurrentIrpStackLocation(Irp))->Parameters.Others.Argument1)

//
//  URB request codes
//

#define URB_FUNCTION_SELECT_CONFIGURATION            0x0000
#define URB_FUNCTION_SELECT_INTERFACE                0x0001
#define URB_FUNCTION_ABORT_PIPE                      0x0002
#define URB_FUNCTION_TAKE_FRAME_LENGTH_CONTROL       0x0003
#define URB_FUNCTION_RELEASE_FRAME_LENGTH_CONTROL    0x0004
#define URB_FUNCTION_GET_FRAME_LENGTH                0x0005
#define URB_FUNCTION_SET_FRAME_LENGTH                0x0006
#define URB_FUNCTION_GET_CURRENT_FRAME_NUMBER        0x0007
#define URB_FUNCTION_CONTROL_TRANSFER                0x0008
#define URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER      0x0009
#define URB_FUNCTION_ISOCH_TRANSFER                  0x000A
#define URB_FUNCTION_GET_DESCRIPTOR_FROM_DEVICE      0x000B
#define URB_FUNCTION_SET_DESCRIPTOR_TO_DEVICE        0x000C
#define URB_FUNCTION_SET_FEATURE_TO_DEVICE           0x000D
#define URB_FUNCTION_SET_FEATURE_TO_INTERFACE        0x000E
#define URB_FUNCTION_SET_FEATURE_TO_ENDPOINT         0x000F
#define URB_FUNCTION_CLEAR_FEATURE_TO_DEVICE         0x0010
#define URB_FUNCTION_CLEAR_FEATURE_TO_INTERFACE      0x0011
#define URB_FUNCTION_CLEAR_FEATURE_TO_ENDPOINT       0x0012
#define URB_FUNCTION_GET_STATUS_FROM_DEVICE          0x0013
#define URB_FUNCTION_GET_STATUS_FROM_INTERFACE       0x0014
#define URB_FUNCTION_GET_STATUS_FROM_ENDPOINT        0x0015
#define URB_FUNCTION_RESERVED_0X0016                 0x0016
#define URB_FUNCTION_VENDOR_DEVICE                   0x0017
#define URB_FUNCTION_VENDOR_INTERFACE                0x0018
#define URB_FUNCTION_VENDOR_ENDPOINT                 0x0019
#define URB_FUNCTION_CLASS_DEVICE                    0x001A
#define URB_FUNCTION_CLASS_INTERFACE                 0x001B
#define URB_FUNCTION_CLASS_ENDPOINT                  0x001C
#define URB_FUNCTION_RESERVE_0X001D                  0x001D
// previously URB_FUNCTION_RESET_PIPE
#define URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL 0x001E
#define URB_FUNCTION_CLASS_OTHER                     0x001F
#define URB_FUNCTION_VENDOR_OTHER                    0x0020
#define URB_FUNCTION_GET_STATUS_FROM_OTHER           0x0021
#define URB_FUNCTION_CLEAR_FEATURE_TO_OTHER          0x0022
#define URB_FUNCTION_SET_FEATURE_TO_OTHER            0x0023
#define URB_FUNCTION_GET_DESCRIPTOR_FROM_ENDPOINT    0x0024
#define URB_FUNCTION_SET_DESCRIPTOR_TO_ENDPOINT      0x0025
#define URB_FUNCTION_GET_CONFIGURATION               0x0026
#define URB_FUNCTION_GET_INTERFACE                   0x0027
#define URB_FUNCTION_GET_DESCRIPTOR_FROM_INTERFACE   0x0028
#define URB_FUNCTION_SET_DESCRIPTOR_TO_INTERFACE     0x0029

// Reserve 0x002B-0x002F
#define URB_FUNCTION_RESERVE_0X002B                  0x002B
#define URB_FUNCTION_RESERVE_0X002C                  0x002C
#define URB_FUNCTION_RESERVE_0X002D                  0x002D
#define URB_FUNCTION_RESERVE_0X002E                  0x002E
#define URB_FUNCTION_RESERVE_0X002F                  0x002F

// USB 2.0 calls start at 0x0030

#if (_WIN32_WINNT >= 0x0501)

#define URB_FUNCTION_GET_MS_FEATURE_DESCRIPTOR       0x002A
#define URB_FUNCTION_SYNC_RESET_PIPE                 0x0030
#define URB_FUNCTION_SYNC_CLEAR_STALL                0x0031

#endif

#if (_WIN32_WINNT >= 0x0600)

#define URB_FUNCTION_CONTROL_TRANSFER_EX             0x0032
#define URB_FUNCTION_RESERVE_0X0033                  0x0033
#define URB_FUNCTION_RESERVE_0X0034                  0x0034

#endif

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define URB_FUNCTION_OPEN_STATIC_STREAMS                          0x0035
#define URB_FUNCTION_CLOSE_STATIC_STREAMS                         0x0036
#define URB_FUNCTION_BULK_OR_INTERRUPT_TRANSFER_USING_CHAINED_MDL 0x0037
#define URB_FUNCTION_ISOCH_TRANSFER_USING_CHAINED_MDL             0x0038
#endif


#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)


#define URB_FUNCTION_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS                0x003D

#endif


// for backward drivers
#define URB_FUNCTION_RESET_PIPE     \
    URB_FUNCTION_SYNC_RESET_PIPE_AND_CLEAR_STALL

//
// Values for URB TransferFlags Field
//

#define USBD_SHORT_TRANSFER_OK                0x00000002
#define USBD_START_ISO_TRANSFER_ASAP          0x00000004
#define USBD_DEFAULT_PIPE_TRANSFER            0x00000008

#define USBD_TRANSFER_DIRECTION_OUT           0
#define USBD_TRANSFER_DIRECTION_IN            1
#define USBD_TRANSFER_DIRECTION               USBD_TRANSFER_DIRECTION_IN

#define USBD_TRANSFER_DIRECTION_FLAG(flags)  ((flags) & USBD_TRANSFER_DIRECTION)

#define VALID_TRANSFER_FLAGS_MASK             (USBD_SHORT_TRANSFER_OK | \
                                               USBD_TRANSFER_DIRECTION | \
                                               USBD_START_ISO_TRANSFER_ASAP | \
                                               USBD_DEFAULT_PIPE_TRANSFER)

#define USBD_ISO_START_FRAME_RANGE            1024

typedef LONG USBD_STATUS;

//
// USBD status codes
//
//  Status values are 32 bit values layed out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+---------------------------+-------------------------------+
//  | S |               Status Code                                 |
//  +---+---------------------------+-------------------------------+
//
//  where
//
//      S - is the state code
//
//          00 - completed with success
//          01 - request is pending
//          11, 10 - completed with error
//
//
//      Code - is the status code
//

//
// Generic test for success on any status value (non-negative numbers
// indicate success).
//

#define USBD_SUCCESS(Status) ((USBD_STATUS)(Status) >= 0)

//
// Generic test for pending status value.
//

#define USBD_PENDING(Status) ((ULONG)(Status) >> 30 == 1)

//
// Generic test for error on any status value.
//

#define USBD_ERROR(Status) ((USBD_STATUS)(Status) < 0)

#define USBD_STATUS_SUCCESS                  ((USBD_STATUS)0x00000000L)
#define USBD_STATUS_PORT_OPERATION_PENDING   ((USBD_STATUS)0x00000001L)
#define USBD_STATUS_PENDING                  ((USBD_STATUS)0x40000000L)

//
// HC (Hardware) status codes range 0x00000001 - 0x000000FF
//

#define USBD_STATUS_CRC                      ((USBD_STATUS)0xC0000001L)
#define USBD_STATUS_BTSTUFF                  ((USBD_STATUS)0xC0000002L)
#define USBD_STATUS_DATA_TOGGLE_MISMATCH     ((USBD_STATUS)0xC0000003L)
#define USBD_STATUS_STALL_PID                ((USBD_STATUS)0xC0000004L)
#define USBD_STATUS_DEV_NOT_RESPONDING       ((USBD_STATUS)0xC0000005L)
#define USBD_STATUS_PID_CHECK_FAILURE        ((USBD_STATUS)0xC0000006L)
#define USBD_STATUS_UNEXPECTED_PID           ((USBD_STATUS)0xC0000007L)
#define USBD_STATUS_DATA_OVERRUN             ((USBD_STATUS)0xC0000008L)
#define USBD_STATUS_DATA_UNDERRUN            ((USBD_STATUS)0xC0000009L)
#define USBD_STATUS_RESERVED1                ((USBD_STATUS)0xC000000AL)
#define USBD_STATUS_RESERVED2                ((USBD_STATUS)0xC000000BL)
#define USBD_STATUS_BUFFER_OVERRUN           ((USBD_STATUS)0xC000000CL)
#define USBD_STATUS_BUFFER_UNDERRUN          ((USBD_STATUS)0xC000000DL)
#define USBD_STATUS_NOT_ACCESSED             ((USBD_STATUS)0xC000000FL)
#define USBD_STATUS_FIFO                     ((USBD_STATUS)0xC0000010L)
#define USBD_STATUS_XACT_ERROR               ((USBD_STATUS)0xC0000011L)
#define USBD_STATUS_BABBLE_DETECTED          ((USBD_STATUS)0xC0000012L)
#define USBD_STATUS_DATA_BUFFER_ERROR        ((USBD_STATUS)0xC0000013L)
#define USBD_STATUS_NO_PING_RESPONSE         ((USBD_STATUS)0xC0000014L)
#define USBD_STATUS_INVALID_STREAM_TYPE      ((USBD_STATUS)0xC0000015L)
#define USBD_STATUS_INVALID_STREAM_ID        ((USBD_STATUS)0xC0000016L)
#define USBD_STATUS_ENDPOINT_HALTED          ((USBD_STATUS)0xC0000030L)

//
// Software status codes
//
#define USBD_STATUS_INVALID_URB_FUNCTION     ((USBD_STATUS)0x80000200L)
#define USBD_STATUS_INVALID_PARAMETER        ((USBD_STATUS)0x80000300L)
#define USBD_STATUS_ERROR_BUSY               ((USBD_STATUS)0x80000400L)
#define USBD_STATUS_INVALID_PIPE_HANDLE      ((USBD_STATUS)0x80000600L)
#define USBD_STATUS_NO_BANDWIDTH             ((USBD_STATUS)0x80000700L)
#define USBD_STATUS_INTERNAL_HC_ERROR        ((USBD_STATUS)0x80000800L)
#define USBD_STATUS_ERROR_SHORT_TRANSFER     ((USBD_STATUS)0x80000900L)
#define USBD_STATUS_BAD_START_FRAME          ((USBD_STATUS)0xC0000A00L)
#define USBD_STATUS_ISOCH_REQUEST_FAILED     ((USBD_STATUS)0xC0000B00L)
#define USBD_STATUS_FRAME_CONTROL_OWNED      ((USBD_STATUS)0xC0000C00L)
#define USBD_STATUS_FRAME_CONTROL_NOT_OWNED  ((USBD_STATUS)0xC0000D00L)
#define USBD_STATUS_NOT_SUPPORTED            ((USBD_STATUS)0xC0000E00L)
#define USBD_STATUS_INAVLID_CONFIGURATION_DESCRIPTOR \
                                             ((USBD_STATUS)0xC0000F00L)
#define USBD_STATUS_INVALID_CONFIGURATION_DESCRIPTOR \
                                             ((USBD_STATUS)0xC0000F00L)
#define USBD_STATUS_INSUFFICIENT_RESOURCES   ((USBD_STATUS)0xC0001000L)
#define USBD_STATUS_SET_CONFIG_FAILED        ((USBD_STATUS)0xC0002000L)
#define USBD_STATUS_BUFFER_TOO_SMALL         ((USBD_STATUS)0xC0003000L)
#define USBD_STATUS_INTERFACE_NOT_FOUND      ((USBD_STATUS)0xC0004000L)
#define USBD_STATUS_INAVLID_PIPE_FLAGS       ((USBD_STATUS)0xC0005000L)
#define USBD_STATUS_TIMEOUT                  ((USBD_STATUS)0xC0006000L)
#define USBD_STATUS_DEVICE_GONE              ((USBD_STATUS)0xC0007000L)
#define USBD_STATUS_STATUS_NOT_MAPPED        ((USBD_STATUS)0xC0008000L)
#define USBD_STATUS_HUB_INTERNAL_ERROR       ((USBD_STATUS)0xC0009000L)
#define USBD_STATUS_CANCELED                 ((USBD_STATUS)0xC0010000L)
#define USBD_STATUS_ISO_NOT_ACCESSED_BY_HW   ((USBD_STATUS)0xC0020000L)
#define USBD_STATUS_ISO_TD_ERROR             ((USBD_STATUS)0xC0030000L)
#define USBD_STATUS_ISO_NA_LATE_USBPORT      ((USBD_STATUS)0xC0040000L)
#define USBD_STATUS_ISO_NOT_ACCESSED_LATE    ((USBD_STATUS)0xC0050000L)
#define USBD_STATUS_BAD_DESCRIPTOR           ((USBD_STATUS)0xC0100000L)
#define USBD_STATUS_BAD_DESCRIPTOR_BLEN      ((USBD_STATUS)0xC0100001L)
#define USBD_STATUS_BAD_DESCRIPTOR_TYPE      ((USBD_STATUS)0xC0100002L)
#define USBD_STATUS_BAD_INTERFACE_DESCRIPTOR ((USBD_STATUS)0xC0100003L)
#define USBD_STATUS_BAD_ENDPOINT_DESCRIPTOR  ((USBD_STATUS)0xC0100004L)
#define USBD_STATUS_BAD_INTERFACE_ASSOC_DESCRIPTOR \
                                             ((USBD_STATUS)0xC0100005L)
#define USBD_STATUS_BAD_CONFIG_DESC_LENGTH   ((USBD_STATUS)0xC0100006L)
#define USBD_STATUS_BAD_NUMBER_OF_INTERFACES ((USBD_STATUS)0xC0100007L)
#define USBD_STATUS_BAD_NUMBER_OF_ENDPOINTS  ((USBD_STATUS)0xC0100008L)
#define USBD_STATUS_BAD_ENDPOINT_ADDRESS     ((USBD_STATUS)0xC0100009L)

typedef PVOID USBD_PIPE_HANDLE;
typedef PVOID USBD_CONFIGURATION_HANDLE;
typedef PVOID USBD_INTERFACE_HANDLE;

#if (_WIN32_WINNT >= 0x0501)

#define USBD_DEFAULT_MAXIMUM_TRANSFER_SIZE  0xFFFFFFFF

#else

#define USBD_DEFAULT_MAXIMUM_TRANSFER_SIZE  PAGE_SIZE

#endif

typedef struct _USBD_VERSION_INFORMATION {
    ULONG USBDI_Version;
    ULONG Supported_USB_Version;
} USBD_VERSION_INFORMATION, *PUSBD_VERSION_INFORMATION;

typedef enum _USBD_PIPE_TYPE {
    UsbdPipeTypeControl,
    UsbdPipeTypeIsochronous,
    UsbdPipeTypeBulk,
    UsbdPipeTypeInterrupt
} USBD_PIPE_TYPE;

#define USBD_PIPE_DIRECTION_IN(pipeInformation) ((pipeInformation)->EndpointAddress & \
                                                  USB_ENDPOINT_DIRECTION_MASK)

typedef struct _USBD_DEVICE_INFORMATION {
    ULONG OffsetNext;
    PVOID UsbdDeviceHandle;
    USB_DEVICE_DESCRIPTOR DeviceDescriptor;
} USBD_DEVICE_INFORMATION, *PUSBD_DEVICE_INFORMATION;

typedef struct _USBD_PIPE_INFORMATION {
    USHORT MaximumPacketSize;
    UCHAR EndpointAddress;
    UCHAR Interval;
    USBD_PIPE_TYPE PipeType;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG MaximumTransferSize;
    ULONG PipeFlags;
} USBD_PIPE_INFORMATION, *PUSBD_PIPE_INFORMATION;

//
// values for PipeFlags field in USBD_PIPE_INFORMATION struct
//

//
// override the enpoint max_packet size with the value in pipe_information
// field
//
#define USBD_PF_CHANGE_MAX_PACKET             0x00000001

//
// optimize for short packets 'bulk optimization #1'
//
#define USBD_PF_SHORT_PACKET_OPT              0x00000002

//
// optimize transfers for use with 'real time threads
//
#define USBD_PF_ENABLE_RT_THREAD_ACCESS       0x00000004

//
// causes the driver to allocate map more transfers in the queue.
//
#define USBD_PF_MAP_ADD_TRANSFERS             0x00000008

#define USBD_PF_VIDEO_PRIORITY                0x00000010
#define USBD_PF_VOICE_PRIORITY                0x00000020
#define USBD_PF_INTERACTIVE_PRIORITY          0x00000030

#define USBD_PF_PRIORITY_MASK                 0x000000F0


#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

//
// Inform the USB hub driver that the client is enlightened to handle
// SuperSpeedPlus high bandwidth Isochronous endpoints and expects the
// bytes per interval to be captured in the MaximumTransferSize field
// of the pipe information instead of the MaximumPacketSize field.
// The USB hub driver will set the USBD_PF_SSP_HIGH_BANDWIDTH_ISOCH
// flag for any such pipes if the client opts in by first setting this flag.
//
#define USBD_PF_HANDLES_SSP_HIGH_BANDWIDTH_ISOCH      0x00000100

//
// Inform the client that the pipe is a high bandwidth SuperSpeedPlus
// Isochronous endpoint and the bytes per interval are captured in
// the MaximumTransferSize field of the pipe information instead
// of the MaximumPacketSize field. Note: this flag communicates pipe
// properties from the host to the client, whereas other pipe flags
// have historically been used to communicate intent from the client
// to the host. As such, this flag is not included in the USBD_PF_VALID_MASK
// and is only meaningful if the client has set the
// USBD_PF_HANDLES_SSP_HIGH_BANDWIDTH_ISOCH flag.
//
#define USBD_PF_SSP_HIGH_BANDWIDTH_ISOCH      0x00010000

#define USBD_PF_VALID_MASK    (USBD_PF_CHANGE_MAX_PACKET | \
                               USBD_PF_SHORT_PACKET_OPT | \
                               USBD_PF_ENABLE_RT_THREAD_ACCESS | \
                               USBD_PF_MAP_ADD_TRANSFERS | \
                               USBD_PF_VIDEO_PRIORITY | \
                               USBD_PF_VOICE_PRIORITY | \
                               USBD_PF_INTERACTIVE_PRIORITY | \
                               USBD_PF_HANDLES_SSP_HIGH_BANDWIDTH_ISOCH)

#else // (NTDDI_VERSION < NTDDI_WIN10_VB)

#define USBD_PF_VALID_MASK    (USBD_PF_CHANGE_MAX_PACKET | \
                               USBD_PF_SHORT_PACKET_OPT | \
                               USBD_PF_ENABLE_RT_THREAD_ACCESS | \
                               USBD_PF_MAP_ADD_TRANSFERS | \
                               USBD_PF_VIDEO_PRIORITY | \
                               USBD_PF_VOICE_PRIORITY | \
                               USBD_PF_INTERACTIVE_PRIORITY)

#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

typedef enum _USBD_ENDPOINT_OFFLOAD_MODE {

    UsbdEndpointOffloadModeNotSupported = 0,
    UsbdEndpointOffloadSoftwareAssisted,
    UsbdEndpointOffloadHardwareAssisted

} USBD_ENDPOINT_OFFLOAD_MODE;

#pragma pack(push)
#pragma pack(1)

typedef struct _USBD_ENDPOINT_OFFLOAD_INFORMATION_V1 {

    ULONG Size;

    //
    // Input field to be filled in by USB client driver
    //

    USHORT EndpointAddress;

    ULONG ResourceId; // Valid only in hardware assisted offload mode.

    //
    // Output fields returned by the USB host controller driver if the specified
    // endpoint (in EndpointAddress) was successfully setup for offload mode.
    //

    USBD_ENDPOINT_OFFLOAD_MODE Mode;

    //
    // Fields from SLOT_CONTEXT describing the device.
    // Valid for both hardware and software assisted endpoint
    // offload modes.
    //

    ULONG RootHubPortNumber:8;
    ULONG RouteString:20;
    ULONG Speed:4;

    ULONG UsbDeviceAddress:8;
    ULONG SlotId:8;
    ULONG MultiTT:1;

    ULONG LSOrFSDeviceConnectedToTTHub:1;
    ULONG Reserved0:14;

    //
    // OUTPUT - Transfer ring information.
    // Valid only in software assisted offload mode.
    //

    PHYSICAL_ADDRESS TransferSegmentLA;
    PVOID TransferSegmentVA;
    size_t TransferRingSize;
    ULONG TransferRingInitialCycleBit;

    //
    // OUTPUT - Secondary event ring information
    // Valid only in software assisted offload mode.
    //

    ULONG MessageNumber;
    PHYSICAL_ADDRESS EventRingSegmentLA;
    PVOID EventRingSegmentVA;
    size_t EventRingSize;
    ULONG EventRingInitialCycleBit;

} USBD_ENDPOINT_OFFLOAD_INFORMATION_V1, *PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V1;

typedef struct _USBD_ENDPOINT_OFFLOAD_INFORMATION {

    ULONG Size;

    //
    // Input field to be filled in by USB client driver
    //

    USHORT EndpointAddress;

    ULONG ResourceId; // Valid only in hardware assisted offload mode.

    //
    // Output fields returned by the USB host controller driver if the specified
    // endpoint (in EndpointAddress) was successfully setup for offload mode.
    //

    USBD_ENDPOINT_OFFLOAD_MODE Mode;

    //
    // Fields from SLOT_CONTEXT describing the device.
    // Valid for both hardware and software assisted endpoint
    // offload modes.
    //

    ULONG RootHubPortNumber:8;
    ULONG RouteString:20;
    ULONG Speed:4;

    ULONG UsbDeviceAddress:8;
    ULONG SlotId:8;
    ULONG MultiTT:1;

    ULONG LSOrFSDeviceConnectedToTTHub:1;
    ULONG Reserved0:14;

    //
    // OUTPUT - Transfer ring information.
    // Valid only in software assisted offload mode.
    //

    PHYSICAL_ADDRESS TransferSegmentLA;
    PVOID TransferSegmentVA;
    size_t TransferRingSize;
    ULONG TransferRingInitialCycleBit;

    //
    // OUTPUT - Secondary event ring information
    // Valid only in software assisted offload mode.
    //

    ULONG MessageNumber;
    PHYSICAL_ADDRESS EventRingSegmentLA;
    PVOID EventRingSegmentVA;
    size_t EventRingSize;
    ULONG EventRingInitialCycleBit;

    //
    // +++
    // New fields added in _V2 version follow here.
    // ---
    //

    //
    // INPUT - Transfer ring segments provided by the sideband component.
    // Output remains in the "Transfer Ring Information" above".
    // Valid only in software assisted endpoint offload mode and
    // must belong to the IO/device address space. It should be page
    // aligned and size must be a multiple of PAGE_SIZE.
    //

    PHYSICAL_ADDRESS ClientTransferRingSegmentPAIn;
    size_t ClientTransferRingSizeIn;

    //
    // INPUT - Data buffers provided by the sideband component.
    // Valid only in software assisted endpoint offload mode and
    // must belong to the IO/device address space. It should be page
    // aligned and size must be a multiple of PAGE_SIZE.
    //

    PHYSICAL_ADDRESS ClientDataBufferPAIn;
    size_t ClientDataBufferSizeIn;

    //
    // OUTPUT - fields describing mapped data buffer
    // Valid only in software assisted offload mode.
    //

    PHYSICAL_ADDRESS ClientDataBufferLAOut;
    PVOID ClientDataBufferVAOut;

} USBD_ENDPOINT_OFFLOAD_INFORMATION, *PUSBD_ENDPOINT_OFFLOAD_INFORMATION,
  USBD_ENDPOINT_OFFLOAD_INFORMATION_V2, *PUSBD_ENDPOINT_OFFLOAD_INFORMATION_V2;

C_ASSERT(sizeof(USBD_ENDPOINT_OFFLOAD_INFORMATION_V2) > sizeof(USBD_ENDPOINT_OFFLOAD_INFORMATION_V1));
C_ASSERT(sizeof(USBD_ENDPOINT_OFFLOAD_INFORMATION) > sizeof(USBD_ENDPOINT_OFFLOAD_INFORMATION_V1));

#pragma pack(pop)

#endif

typedef struct _USBD_INTERFACE_INFORMATION {
    USHORT Length;
    UCHAR InterfaceNumber;
    UCHAR AlternateSetting;
    UCHAR Class;
    UCHAR SubClass;
    UCHAR Protocol;
    UCHAR Reserved;
    USBD_INTERFACE_HANDLE InterfaceHandle;
    ULONG NumberOfPipes;
    USBD_PIPE_INFORMATION Pipes[1];
} USBD_INTERFACE_INFORMATION, *PUSBD_INTERFACE_INFORMATION;

struct _URB_HCD_AREA {
    PVOID Reserved8[8];
};

struct _URB_HEADER {
    USHORT Length;
    USHORT Function;
    USBD_STATUS Status;
    PVOID UsbdDeviceHandle; // Reserved
    ULONG UsbdFlags; // Reserved
};

struct _URB_SELECT_INTERFACE {
    struct _URB_HEADER Hdr;
    USBD_CONFIGURATION_HANDLE ConfigurationHandle;
    USBD_INTERFACE_INFORMATION Interface;
};


struct _URB_SELECT_CONFIGURATION {
    struct _URB_HEADER Hdr;
    PUSB_CONFIGURATION_DESCRIPTOR ConfigurationDescriptor;
    USBD_CONFIGURATION_HANDLE ConfigurationHandle;
    USBD_INTERFACE_INFORMATION Interface;
};

struct _URB_PIPE_REQUEST {
    struct _URB_HEADER Hdr;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG Reserved;
};

struct _URB_FRAME_LENGTH_CONTROL {
    struct _URB_HEADER Hdr;
};

struct _URB_GET_FRAME_LENGTH {
    struct _URB_HEADER Hdr;
    ULONG FrameLength;
    ULONG FrameNumber;
};

struct _URB_SET_FRAME_LENGTH {
    struct _URB_HEADER Hdr;
    LONG FrameLengthDelta;
};

struct _URB_GET_CURRENT_FRAME_NUMBER {
    struct _URB_HEADER Hdr;
    ULONG FrameNumber;
};

struct _URB_CONTROL_DESCRIPTOR_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG Reserved0;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    USHORT Reserved1;
    UCHAR Index;
    UCHAR DescriptorType;
    USHORT LanguageId;
    USHORT Reserved2;
};

struct _URB_CONTROL_GET_STATUS_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG Reserved0;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR Reserved1[4];
    USHORT Index;
    USHORT Reserved2;
};

struct _URB_CONTROL_FEATURE_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG Reserved2;
    ULONG Reserved3;
    PVOID Reserved4;
    PMDL Reserved5;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    USHORT Reserved0;
    USHORT FeatureSelector;
    USHORT Index;
    USHORT Reserved1;
};

struct _URB_CONTROL_VENDOR_OR_CLASS_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG TransferFlags;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR RequestTypeReservedBits;
    UCHAR Request;
    USHORT Value;
    USHORT Index;
    USHORT Reserved1;
};

struct _URB_CONTROL_GET_INTERFACE_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG Reserved0;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR Reserved1[4];
    USHORT Interface;
    USHORT Reserved2;
};

struct _URB_CONTROL_GET_CONFIGURATION_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG Reserved0;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink;  //Reserved
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR Reserved1[8];
};

#if (_WIN32_WINNT >= 0x0501)

#define OS_STRING_DESCRIPTOR_INDEX                  0xEE

#define MS_GENRE_DESCRIPTOR_INDEX                   0x0001
#define MS_POWER_DESCRIPTOR_INDEX                   0x0002

#define MS_OS_STRING_SIGNATURE                      L"MSFT100"

#define MS_OS_FLAGS_CONTAINERID                     0x02

typedef struct _OS_STRING {
    UCHAR bLength;
    UCHAR bDescriptorType;
    WCHAR MicrosoftString[7];
    UCHAR bVendorCode;
    union {
        UCHAR bPad;
        UCHAR bFlags;
    };
} OS_STRING, *POS_STRING;

struct _URB_OS_FEATURE_DESCRIPTOR_REQUEST {
    struct _URB_HEADER Hdr;
    PVOID Reserved;
    ULONG Reserved0;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR   Recipient:5;
    UCHAR   Reserved1:3;
    UCHAR   Reserved2;
    UCHAR   InterfaceNumber;
    UCHAR   MS_PageIndex;
    USHORT  MS_FeatureDescriptorIndex;
    USHORT  Reserved3;
};

#endif

struct _URB_CONTROL_TRANSFER {
    struct _URB_HEADER Hdr;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG TransferFlags;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR SetupPacket[8];
};

#if (_WIN32_WINNT >= 0x0600)

struct _URB_CONTROL_TRANSFER_EX {
    struct _URB_HEADER Hdr;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG TransferFlags;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    ULONG Timeout;
#ifdef WIN64
    ULONG Pad;
#endif
    struct _URB_HCD_AREA hca; // Reserved
    UCHAR SetupPacket[8];
};

#endif

struct _URB_BULK_OR_INTERRUPT_TRANSFER {
    struct _URB_HEADER Hdr;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG TransferFlags;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
};

typedef struct _USBD_ISO_PACKET_DESCRIPTOR {
    ULONG Offset;
    ULONG Length;
    USBD_STATUS Status;
} USBD_ISO_PACKET_DESCRIPTOR, *PUSBD_ISO_PACKET_DESCRIPTOR;

struct _URB_ISOCH_TRANSFER {
    struct _URB_HEADER Hdr;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG TransferFlags;
    ULONG TransferBufferLength;
    PVOID TransferBuffer;
    PMDL TransferBufferMDL;
    struct _URB *UrbLink; // Reserved
    struct _URB_HCD_AREA hca; // Reserved
    ULONG StartFrame;
    ULONG NumberOfPackets;
    ULONG ErrorCount;
    USBD_ISO_PACKET_DESCRIPTOR IsoPacket[1];
};

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define URB_OPEN_STATIC_STREAMS_VERSION_100 0x100

typedef struct _USBD_STREAM_INFORMATION {
    USBD_PIPE_HANDLE   PipeHandle;
    ULONG              StreamID;
    ULONG              MaximumTransferSize;
    ULONG              PipeFlags;
} USBD_STREAM_INFORMATION, *PUSBD_STREAM_INFORMATION;

struct _URB_OPEN_STATIC_STREAMS {
    struct _URB_HEADER       Hdr;
    USBD_PIPE_HANDLE         PipeHandle;
    ULONG                    NumberOfStreams;
    USHORT                   StreamInfoVersion;
    USHORT                   StreamInfoSize;
    PUSBD_STREAM_INFORMATION Streams;
};

#endif


#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)

struct _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS {
    struct _URB_HEADER Hdr;
    USBD_PIPE_HANDLE PipeHandle;
    ULONG MaximumSendPathDelayInMilliSeconds;
    ULONG MaximumCompletionPathDelayInMilliSeconds;
};

#endif

typedef _Struct_size_bytes_(UrbHeader.Length) struct _URB {
    union {
        struct _URB_HEADER
            UrbHeader;
        struct _URB_SELECT_INTERFACE
            UrbSelectInterface;
        struct _URB_SELECT_CONFIGURATION
            UrbSelectConfiguration;
        struct _URB_PIPE_REQUEST
            UrbPipeRequest;
        struct _URB_FRAME_LENGTH_CONTROL
            UrbFrameLengthControl;
        struct _URB_GET_FRAME_LENGTH
            UrbGetFrameLength;
        struct _URB_SET_FRAME_LENGTH
            UrbSetFrameLength;
        struct _URB_GET_CURRENT_FRAME_NUMBER
            UrbGetCurrentFrameNumber;
        struct _URB_CONTROL_TRANSFER
            UrbControlTransfer;

    #if (_WIN32_WINNT >= 0x0600)

        struct _URB_CONTROL_TRANSFER_EX
            UrbControlTransferEx;

    #endif

        struct _URB_BULK_OR_INTERRUPT_TRANSFER
            UrbBulkOrInterruptTransfer;
        struct _URB_ISOCH_TRANSFER
            UrbIsochronousTransfer;
        struct _URB_CONTROL_DESCRIPTOR_REQUEST
            UrbControlDescriptorRequest;
        struct _URB_CONTROL_GET_STATUS_REQUEST
            UrbControlGetStatusRequest;
        struct _URB_CONTROL_FEATURE_REQUEST
            UrbControlFeatureRequest;
        struct _URB_CONTROL_VENDOR_OR_CLASS_REQUEST
            UrbControlVendorClassRequest;
        struct _URB_CONTROL_GET_INTERFACE_REQUEST
            UrbControlGetInterfaceRequest;
        struct _URB_CONTROL_GET_CONFIGURATION_REQUEST
            UrbControlGetConfigurationRequest;

    #if (_WIN32_WINNT >= 0x0501)

        struct _URB_OS_FEATURE_DESCRIPTOR_REQUEST
            UrbOSFeatureDescriptorRequest;
    #endif

    #if (NTDDI_VERSION >= NTDDI_WIN8)

        struct _URB_OPEN_STATIC_STREAMS
            UrbOpenStaticStreams;

    #endif


    #if (NTDDI_VERSION >= NTDDI_WIN10_RS3)


        struct _URB_GET_ISOCH_PIPE_TRANSFER_PATH_DELAYS
            UrbGetIsochPipeTransferPathDelays;

    #endif

    };

} URB, *PURB;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /*  __USB_H__ */

