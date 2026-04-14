/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    srb.h

Abstract:

    This file defines the interface between SCSI mini-port drivers and the
    SCSI port driver.  It is also used by SCSI class drivers to talk to the
    SCSI port driver.

Revision History:

--*/

//@[contract("ntoskrnl-srb"), comment("MVI_tracked - https://osgwiki.com/wiki/Microsoft_Virus_Initiative")];

#ifndef _NTSRB_
#define _NTSRB_

#include <winapifamily.h>

#pragma warning(push)
#pragma warning(disable:4214) // nonstandard extension used : bit field types other than int
#pragma warning(disable:4201) // nonstandard extension used : nameless struct/union

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if DBG
#define DebugPrint(x) ScsiDebugPrint x
#else
#define DebugPrint(x)
#endif

// begin_storport begin_privstorport

//
// Define SCSI maximum configuration parameters.
//
// NOTE - the current SCSI_MAXIMUM_TARGETS_PER_BUS is applicable only
// on scsiport miniports. For storport miniports, the max target
// supported is 255.
//

#define SCSI_MAXIMUM_BUSES_PER_ADAPTER 255
#define SCSI_MAXIMUM_TARGETS_PER_BUS 128
#define SCSI_MAXIMUM_LUNS_PER_TARGET 255
#define SCSI_MINIMUM_PHYSICAL_BREAKS  16
#define SCSI_MAXIMUM_PHYSICAL_BREAKS 255

#define SCSI_COMBINE_BUS_TARGET( Bus, Target ) ( \
    ((((UCHAR) (Target)) & ~(0x20 - 1)) << 8) |        \
    (((UCHAR) (Bus)) << 5) |                     \
    (((UCHAR) (Target)) & (0x20 - 1)))

#define SCSI_DECODE_BUS_TARGET( Value, Bus, Target ) ( \
    Bus = (UCHAR) ((Value) >> 5),                     \
    Target = (UCHAR) ((((Value) >> 8) & ~(0x20 - 1)) | ((Value) & (0x20 - 1))))

//
// This constant is for backward compatibility.
// This use to be the maximum supported.
//

#define SCSI_MAXIMUM_BUSES 8
#define SCSI_MAXIMUM_TARGETS 8
#define SCSI_MAXIMUM_LOGICAL_UNITS 8

//end_storport end_privstorport

typedef PHYSICAL_ADDRESS SCSI_PHYSICAL_ADDRESS, *PSCSI_PHYSICAL_ADDRESS;

typedef struct _ACCESS_RANGE {
    SCSI_PHYSICAL_ADDRESS RangeStart;
    ULONG RangeLength;
    BOOLEAN RangeInMemory;
}ACCESS_RANGE, *PACCESS_RANGE;

//
// Configuration information structure.  Contains the information necessary
// to initialize the adapter. NOTE: This structure's must be a multiple of
// quadwords.
//

typedef struct _PORT_CONFIGURATION_INFORMATION {

    //
    // Length of port configuation information strucuture.
    //

    ULONG Length;

    //
    // IO bus number (0 for machines that have only 1 IO bus
    //

    ULONG SystemIoBusNumber;

    //
    // EISA, MCA or ISA
    //

    INTERFACE_TYPE  AdapterInterfaceType;

    //
    // Interrupt request level for device
    //

    ULONG BusInterruptLevel;

    //
    // Bus interrupt vector used with hardware buses which use as vector as
    // well as level, such as internal buses.
    //

    ULONG BusInterruptVector;

    //
    // Interrupt mode (level-sensitive or edge-triggered)
    //

    KINTERRUPT_MODE InterruptMode;

    //
    // Maximum number of bytes that can be transferred in a single SRB
    //

    ULONG MaximumTransferLength;

    //
    // Number of contiguous blocks of physical memory
    //

    ULONG NumberOfPhysicalBreaks;

    //
    // DMA channel for devices using system DMA
    //

    ULONG DmaChannel;
    ULONG DmaPort;
    DMA_WIDTH DmaWidth;
    DMA_SPEED DmaSpeed;

    //
    // Alignment masked required by the adapter for data transfers.
    //

    ULONG AlignmentMask;

    //
    // Number of access range elements which have been allocated.
    //

    ULONG NumberOfAccessRanges;

    //
    // Pointer to array of access range elements.
    //

    ACCESS_RANGE (*AccessRanges)[];

    //
    // Reserved field.
    //

    PVOID Reserved;

    //
    // Number of SCSI buses attached to the adapter.
    //

    UCHAR NumberOfBuses;

    //
    // SCSI bus ID for adapter
    //

    UCHAR InitiatorBusId[8];

    //
    // Indicates that the adapter does scatter/gather
    //

    BOOLEAN ScatterGather;

    //
    // Indicates that the adapter is a bus master
    //

    BOOLEAN Master;

    //
    // Host caches data or state.
    //

    BOOLEAN CachesData;

    //
    // Host adapter scans down for bios devices.
    //

    BOOLEAN AdapterScansDown;

    //
    // Primary at disk address (0x1F0) claimed.
    //

    BOOLEAN AtdiskPrimaryClaimed;

    //
    // Secondary at disk address (0x170) claimed.
    //

    BOOLEAN AtdiskSecondaryClaimed;

    //
    // The master uses 32-bit DMA addresses.
    //

    BOOLEAN Dma32BitAddresses;

    //
    // Use Demand Mode DMA rather than Single Request.
    //

    BOOLEAN DemandMode;

    //
    // Data buffers must be mapped into virtual address space.
    //

    BOOLEAN MapBuffers;

    //
    // The driver will need to tranlate virtual to physical addresses.
    //

    BOOLEAN NeedPhysicalAddresses;

    //
    // Supports tagged queuing
    //

    BOOLEAN TaggedQueuing;

    //
    // Supports auto request sense.
    //

    BOOLEAN AutoRequestSense;

    //
    // Supports multiple requests per logical unit.
    //

    BOOLEAN MultipleRequestPerLu;

    //
    // Support receive event function.
    //

    BOOLEAN ReceiveEvent;

    //
    // Indicates the real-mode driver has initialized the card.
    //

    BOOLEAN RealModeInitialized;

    //
    // Indicate that the miniport will not touch the data buffers directly.
    //

    BOOLEAN BufferAccessScsiPortControlled;

    //
    // Indicator for wide scsi.
    //

    UCHAR   MaximumNumberOfTargets;

    //
    // Ensure quadword alignment.
    //

    UCHAR   ReservedUchars[2];

    //
    // Adapter slot number
    //

    ULONG SlotNumber;

    //
    // Interrupt information for a second IRQ.
    //

    ULONG BusInterruptLevel2;
    ULONG BusInterruptVector2;
    KINTERRUPT_MODE InterruptMode2;

    //
    // DMA information for a second channel.
    //

    ULONG DmaChannel2;
    ULONG DmaPort2;
    DMA_WIDTH DmaWidth2;
    DMA_SPEED DmaSpeed2;

    //
    // Fields added to allow for the miniport
    // to update these sizes based on requirements
    // for large transfers ( > 64K);
    //

    ULONG DeviceExtensionSize;
    ULONG SpecificLuExtensionSize;
    ULONG SrbExtensionSize;

    //
    // Used to determine whether the system and/or the miniport support
    // 64-bit physical addresses.  See SCSI_DMA64_* flags below.
    //

    UCHAR  Dma64BitAddresses;        /* New */

    //
    // Indicates that the miniport can accept a SRB_FUNCTION_RESET_DEVICE
    // to clear all requests to a particular LUN.
    //

    BOOLEAN ResetTargetSupported;       /* New */

    //
    // Indicates that the miniport can support more than 8 logical units per
    // target (maximum LUN number is one less than this field).
    //

    UCHAR MaximumNumberOfLogicalUnits;  /* New */

    //
    // Supports WMI?
    //

    BOOLEAN WmiDataProvider;

} PORT_CONFIGURATION_INFORMATION, *PPORT_CONFIGURATION_INFORMATION;

//
// Version control for ConfigInfo structure.
//

#define CONFIG_INFO_VERSION_2 sizeof(PORT_CONFIGURATION_INFORMATION)


//
// Flags for controlling 64-bit DMA use (PORT_CONFIGURATION_INFORMATION field
// Dma64BitAddresses)
//

//
// Set by scsiport on entering HwFindAdapter if the system can support 64-bit
// physical addresses.  The miniport can use this information before calling
// ScsiPortGetUncachedExtension to modify the DeviceExtensionSize,
// SpecificLuExtensionSize & SrbExtensionSize fields to account for the extra
// size of the scatter gather list.
//

#define SCSI_DMA64_SYSTEM_SUPPORTED     0x80

//
// Set by the miniport before calling ScsiPortGetUncachedExtension to indicate
// that scsiport should provide it with 64-bit physical addresses.  If the
// system does not support 64-bit PA's then this bit will be ignored.
//

#define SCSI_DMA64_MINIPORT_SUPPORTED   0x01

#if (NTDDI_VERSION > NTDDI_WS03SP1)
//
// Set by the miniport before calling ScsiPortGetUncachedExtension to indicate
// that scsiport should provide it with 64-bit physical addresses.
// In addition to I/O requests being handled with > 4GB physical addresses,
// the uncached extension, SenseInof and Srb Extension may all lie above 4GB.
// If the system does not support 64-bit PA's then this bit will be ignored.
//

#define SCSI_DMA64_MINIPORT_FULL64BIT_SUPPORTED 0x02
#endif


//
// Command type (and parameter) definition(s) for AdapterControl requests.
//

typedef enum _SCSI_ADAPTER_CONTROL_TYPE {
    ScsiQuerySupportedControlTypes = 0,
    ScsiStopAdapter,
    ScsiRestartAdapter,
    ScsiSetBootConfig,
    ScsiSetRunningConfig,
    ScsiAdapterControlMax,
    MakeAdapterControlTypeSizeOfUlong = 0xffffffff
} SCSI_ADAPTER_CONTROL_TYPE, *PSCSI_ADAPTER_CONTROL_TYPE;

//
// Adapter control status values
//

typedef enum _SCSI_ADAPTER_CONTROL_STATUS {
    ScsiAdapterControlSuccess = 0,
    ScsiAdapterControlUnsuccessful
} SCSI_ADAPTER_CONTROL_STATUS, *PSCSI_ADAPTER_CONTROL_STATUS;

//
// Parameters for Adapter Control Functions:
//

//
// ScsiQuerySupportedControlTypes:
//

#pragma warning(disable:4200)
typedef struct _SCSI_SUPPORTED_CONTROL_TYPE_LIST {

    //
    // Specifies the number of entries in the adapter control type list.
    //

    IN ULONG MaxControlType;

    //
    // The miniport will set TRUE for each control type it supports.
    // The number of entries in this array is defined by MaxAdapterControlType
    // - the miniport must not attempt to set any AC types beyond the maximum
    // value specified.
    //

    OUT BOOLEAN SupportedTypeList[0];

} SCSI_SUPPORTED_CONTROL_TYPE_LIST, *PSCSI_SUPPORTED_CONTROL_TYPE_LIST;
#pragma warning(default:4200)

// begin_storport begin_privstorport

//
// Uninitialized flag value.
//

#define SP_UNINITIALIZED_VALUE ((ULONG) ~0)
#define SP_UNTAGGED ((UCHAR) ~0)

//
// Set asynchronous events.
//

#define SRBEV_BUS_RESET               0x0001
#define SRBEV_SCSI_ASYNC_NOTIFICATION 0x0002

// begin_ntminitape

#define MAXIMUM_CDB_SIZE 12

//
// SCSI I/O Request Block
//

//@[comment("MVI_tracked")]
typedef struct _SCSI_REQUEST_BLOCK {
    USHORT Length;                  // offset 0
    UCHAR Function;                 // offset 2
    UCHAR SrbStatus;                // offset 3
    UCHAR ScsiStatus;               // offset 4
    UCHAR PathId;                   // offset 5
    UCHAR TargetId;                 // offset 6
    UCHAR Lun;                      // offset 7
    UCHAR QueueTag;                 // offset 8
    UCHAR QueueAction;              // offset 9
    UCHAR CdbLength;                // offset a
    UCHAR SenseInfoBufferLength;    // offset b
    ULONG SrbFlags;                 // offset c
    ULONG DataTransferLength;       // offset 10
    ULONG TimeOutValue;             // offset 14
    _Field_size_bytes_(DataTransferLength)
    PVOID DataBuffer;               // offset 18
    PVOID SenseInfoBuffer;          // offset 1c
    struct _SCSI_REQUEST_BLOCK *NextSrb; // offset 20
    PVOID OriginalRequest;          // offset 24
    PVOID SrbExtension;             // offset 28
    union {
        ULONG InternalStatus;       // offset 2c
        ULONG QueueSortKey;         // offset 2c
        ULONG LinkTimeoutValue;     // offset 2c
    };

#if defined(_WIN64)

    //
    // Force PVOID alignment of Cdb
    //

    ULONG Reserved;

#endif

    UCHAR Cdb[16];                  // offset 30
} SCSI_REQUEST_BLOCK, *PSCSI_REQUEST_BLOCK;

#define SCSI_REQUEST_BLOCK_SIZE sizeof(SCSI_REQUEST_BLOCK)

//
// SCSI I/O Request Block for WMI Requests
//

typedef struct _SCSI_WMI_REQUEST_BLOCK {
    USHORT Length;
    UCHAR Function;        // SRB_FUNCTION_WMI
    UCHAR SrbStatus;
    UCHAR WMISubFunction;
    UCHAR PathId;          // If SRB_WMI_FLAGS_ADAPTER_REQUEST is set in
    UCHAR TargetId;        // WMIFlags then PathId, TargetId and Lun are
    UCHAR Lun;             // reserved fields.
    UCHAR Reserved1;
    UCHAR WMIFlags;
    UCHAR Reserved2[2];
    ULONG SrbFlags;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    PVOID DataBuffer;
    PVOID DataPath;
    PVOID Reserved3;
    PVOID OriginalRequest;
    PVOID SrbExtension;
    ULONG Reserved4;

#if (NTDDI_VERSION >= NTDDI_WS03SP1)
#if defined(_WIN64)

    //
    // Force PVOID alignment of Cdb
    //

    ULONG Reserved6;

#endif
#endif

    UCHAR Reserved5[16];
} SCSI_WMI_REQUEST_BLOCK, *PSCSI_WMI_REQUEST_BLOCK;

typedef enum _STOR_DEVICE_POWER_STATE {
    StorPowerDeviceUnspecified = 0,
    StorPowerDeviceD0,
    StorPowerDeviceD1,
    StorPowerDeviceD2,
    StorPowerDeviceD3,
    StorPowerDeviceMaximum
} STOR_DEVICE_POWER_STATE, *PSTOR_DEVICE_POWER_STATE;

typedef enum {
    StorPowerActionNone = 0,
    StorPowerActionReserved,
    StorPowerActionSleep,
    StorPowerActionHibernate,
    StorPowerActionShutdown,
    StorPowerActionShutdownReset,
    StorPowerActionShutdownOff,
    StorPowerActionWarmEject
} STOR_POWER_ACTION, *PSTOR_POWER_ACTION;

typedef struct _SCSI_POWER_REQUEST_BLOCK {
    USHORT Length;                  // offset 0
    UCHAR Function;                 // offset 2
    UCHAR SrbStatus;                // offset 3
    UCHAR SrbPowerFlags;            // offset 4
    UCHAR PathId;                   // offset 5
    UCHAR TargetId;                 // offset 6
    UCHAR Lun;                      // offset 7
    STOR_DEVICE_POWER_STATE DevicePowerState; // offset 8
    ULONG SrbFlags;                 // offset c
    ULONG DataTransferLength;       // offset 10
    ULONG TimeOutValue;             // offset 14
    PVOID DataBuffer;               // offset 18
    PVOID SenseInfoBuffer;          // offset 1c
    struct _SCSI_REQUEST_BLOCK *NextSrb; // offset 20
    PVOID OriginalRequest;          // offset 24
    PVOID SrbExtension;             // offset 28
    STOR_POWER_ACTION PowerAction;       // offset 2c

#if defined(_WIN64)

    //
    // Force PVOID alignment of Cdb
    //

    ULONG Reserved;

#endif

    UCHAR Reserved5[16];              // offset 30
} SCSI_POWER_REQUEST_BLOCK, *PSCSI_POWER_REQUEST_BLOCK;

//
// PNP minor function codes.
//
typedef enum {
    StorStartDevice = 0x0,
    StorRemoveDevice = 0x2,
    StorStopDevice  = 0x4,
    StorQueryCapabilities = 0x9,
    StorQueryResourceRequirements = 0xB,
    StorFilterResourceRequirements = 0xD,
    StorSurpriseRemoval = 0x17
} STOR_PNP_ACTION, *PSTOR_PNP_ACTION;

typedef struct _STOR_DEVICE_CAPABILITIES {
    USHORT Version;
    ULONG  DeviceD1:1;
    ULONG  DeviceD2:1;
    ULONG  LockSupported:1;
    ULONG  EjectSupported:1;
    ULONG  Removable:1;
    ULONG  DockDevice:1;
    ULONG  UniqueID:1;
    ULONG  SilentInstall:1;
    ULONG  SurpriseRemovalOK:1;
    ULONG  NoDisplayInUI:1;

} STOR_DEVICE_CAPABILITIES, *PSTOR_DEVICE_CAPABILITIES;


#define STOR_DEVICE_CAPABILITIES_EX_VERSION_1    0x1

typedef struct _STOR_DEVICE_CAPABILITIES_EX {
    USHORT Version;
    USHORT Size;
    ULONG  DeviceD1:1;
    ULONG  DeviceD2:1;
    ULONG  LockSupported:1;
    ULONG  EjectSupported:1;
    ULONG  Removable:1;
    ULONG  DockDevice:1;
    ULONG  UniqueID:1;
    ULONG  SilentInstall:1;
    ULONG  RawDeviceOK:1;
    ULONG  SurpriseRemovalOK:1;
    ULONG  NoDisplayInUI:1;
    ULONG  DefaultWriteCacheEnabled:1;
    ULONG  Reserved0:20;

    ULONG  Address;
    ULONG  UINumber;

    ULONG  Reserved1[2];
} STOR_DEVICE_CAPABILITIES_EX, *PSTOR_DEVICE_CAPABILITIES_EX;

typedef struct _SCSI_PNP_REQUEST_BLOCK {
    USHORT Length;                  // offset 0
    UCHAR Function;                 // offset 2
    UCHAR SrbStatus;                // offset 3
    UCHAR PnPSubFunction;           // offset 4
    UCHAR PathId;                   // offset 5
    UCHAR TargetId;                 // offset 6
    UCHAR Lun;                      // offset 7
    STOR_PNP_ACTION PnPAction;       // offset 8
    ULONG SrbFlags;                 // offset c
    ULONG DataTransferLength;       // offset 10
    ULONG TimeOutValue;             // offset 14
    PVOID DataBuffer;               // offset 18
    PVOID SenseInfoBuffer;          // offset 1c
    struct _SCSI_REQUEST_BLOCK *NextSrb; // offset 20
    PVOID OriginalRequest;          // offset 24
    PVOID SrbExtension;             // offset 28
    ULONG SrbPnPFlags;              // offset 2c

#if defined(_WIN64)

    //
    // Force PVOID alignment of Cdb
    //

    ULONG Reserved;

#endif
        UCHAR Reserved4[16];            // offset 30
} SCSI_PNP_REQUEST_BLOCK, *PSCSI_PNP_REQUEST_BLOCK;

//
// SRB Functions
//

#define SRB_FUNCTION_EXECUTE_SCSI           0x00
#define SRB_FUNCTION_CLAIM_DEVICE           0x01
#define SRB_FUNCTION_IO_CONTROL             0x02
#define SRB_FUNCTION_RECEIVE_EVENT          0x03
#define SRB_FUNCTION_RELEASE_QUEUE          0x04
#define SRB_FUNCTION_ATTACH_DEVICE          0x05
#define SRB_FUNCTION_RELEASE_DEVICE         0x06
#define SRB_FUNCTION_SHUTDOWN               0x07
#define SRB_FUNCTION_FLUSH                  0x08
#define SRB_FUNCTION_PROTOCOL_COMMAND       0x09
#define SRB_FUNCTION_EXECUTE_NVME           0x0A
#define SRB_FUNCTION_ABORT_COMMAND          0x10
#define SRB_FUNCTION_RELEASE_RECOVERY       0x11
#define SRB_FUNCTION_RESET_BUS              0x12
#define SRB_FUNCTION_RESET_DEVICE           0x13
#define SRB_FUNCTION_TERMINATE_IO           0x14
#define SRB_FUNCTION_FLUSH_QUEUE            0x15
#define SRB_FUNCTION_REMOVE_DEVICE          0x16
#define SRB_FUNCTION_WMI                    0x17
#define SRB_FUNCTION_LOCK_QUEUE             0x18
#define SRB_FUNCTION_UNLOCK_QUEUE           0x19
#define SRB_FUNCTION_QUIESCE_DEVICE         0x1a
#define SRB_FUNCTION_RESET_LOGICAL_UNIT     0x20
#define SRB_FUNCTION_SET_LINK_TIMEOUT       0x21
#define SRB_FUNCTION_LINK_TIMEOUT_OCCURRED  0x22
#define SRB_FUNCTION_LINK_TIMEOUT_COMPLETE  0x23
#define SRB_FUNCTION_POWER                  0x24
#define SRB_FUNCTION_PNP                    0x25
#define SRB_FUNCTION_DUMP_POINTERS          0x26
#define SRB_FUNCTION_FREE_DUMP_POINTERS     0x27


//
// Define extended SRB function that will be used to identify a new
// type of SRB that is not a SCSI_REQUEST_BLOCK. A
// SRB_FUNCTION_STORAGE_REQUEST_BLOCK will use a SRB that is of type
// STORAGE_REQUEST_BLOCK.
//
#define SRB_FUNCTION_STORAGE_REQUEST_BLOCK  0x28

#define SRB_FUNCTION_GET_DUMP_INFO          0x2a
#define SRB_FUNCTION_FREE_DUMP_INFO         0x2b

#define SRB_FUNCTION_NVMEOF_OPERATION       0x2c

#define SRB_FUNCTION_MINIPORT_PASSTHROUGH_REQUEST     0x2d

//
// SRB Status
//

#define SRB_STATUS_PENDING                  0x00
#define SRB_STATUS_SUCCESS                  0x01
#define SRB_STATUS_ABORTED                  0x02
#define SRB_STATUS_ABORT_FAILED             0x03
#define SRB_STATUS_ERROR                    0x04
#define SRB_STATUS_BUSY                     0x05
#define SRB_STATUS_INVALID_REQUEST          0x06
#define SRB_STATUS_INVALID_PATH_ID          0x07
#define SRB_STATUS_NO_DEVICE                0x08
#define SRB_STATUS_TIMEOUT                  0x09
#define SRB_STATUS_SELECTION_TIMEOUT        0x0A
#define SRB_STATUS_COMMAND_TIMEOUT          0x0B
#define SRB_STATUS_MESSAGE_REJECTED         0x0D
#define SRB_STATUS_BUS_RESET                0x0E
#define SRB_STATUS_PARITY_ERROR             0x0F
#define SRB_STATUS_REQUEST_SENSE_FAILED     0x10
#define SRB_STATUS_NO_HBA                   0x11
#define SRB_STATUS_DATA_OVERRUN             0x12
#define SRB_STATUS_UNEXPECTED_BUS_FREE      0x13
#define SRB_STATUS_PHASE_SEQUENCE_FAILURE   0x14
#define SRB_STATUS_BAD_SRB_BLOCK_LENGTH     0x15
#define SRB_STATUS_REQUEST_FLUSHED          0x16
#define SRB_STATUS_ACCESS_DENIED            0x17
#define SRB_STATUS_OPERATION_IN_PROGRESS    0x18
#define SRB_STATUS_INVALID_LUN              0x20
#define SRB_STATUS_INVALID_TARGET_ID        0x21
#define SRB_STATUS_BAD_FUNCTION             0x22
#define SRB_STATUS_ERROR_RECOVERY           0x23
#define SRB_STATUS_NOT_POWERED              0x24
#define SRB_STATUS_LINK_DOWN                0x25
#define SRB_STATUS_INSUFFICIENT_RESOURCES   0x26
#define SRB_STATUS_THROTTLED_REQUEST        0x27
#define SRB_STATUS_INVALID_PARAMETER        0x28


//
// This value is used by the port driver to indicate that a non-scsi-related
// error occured.  Miniports must never return this status.
//

#define SRB_STATUS_INTERNAL_ERROR           0x30

//
// Srb status values 0x38 through 0x3f are reserved for internal port driver
// use.
//

//
// SRB Status Masks
//

#define SRB_STATUS_QUEUE_FROZEN             0x40
#define SRB_STATUS_AUTOSENSE_VALID          0x80

#define SRB_STATUS(Status) (Status & ~(SRB_STATUS_AUTOSENSE_VALID | SRB_STATUS_QUEUE_FROZEN))

//
// SRB Flag Bits
//

#define SRB_FLAGS_QUEUE_ACTION_ENABLE       0x00000002
#define SRB_FLAGS_DISABLE_DISCONNECT        0x00000004
#define SRB_FLAGS_DISABLE_SYNCH_TRANSFER    0x00000008

#define SRB_FLAGS_BYPASS_FROZEN_QUEUE       0x00000010
#define SRB_FLAGS_DISABLE_AUTOSENSE         0x00000020
#define SRB_FLAGS_DATA_IN                   0x00000040
#define SRB_FLAGS_DATA_OUT                  0x00000080
#define SRB_FLAGS_NO_DATA_TRANSFER          0x00000000
#define SRB_FLAGS_UNSPECIFIED_DIRECTION     (SRB_FLAGS_DATA_IN | SRB_FLAGS_DATA_OUT)

#define SRB_FLAGS_NO_QUEUE_FREEZE           0x00000100
#define SRB_FLAGS_ADAPTER_CACHE_ENABLE      0x00000200
#define SRB_FLAGS_FREE_SENSE_BUFFER         0x00000400

//
// This flag indicates the request is part of the workflow for processing a D3.
//
#define SRB_FLAGS_D3_PROCESSING             0x00000800

//
// This flag indicates that LBA range falls into sequential write required zone
//
#define SRB_FLAGS_SEQUENTIAL_REQUIRED       0x00001000


#define SRB_FLAGS_IS_ACTIVE                 0x00010000
#define SRB_FLAGS_ALLOCATED_FROM_ZONE       0x00020000
#define SRB_FLAGS_SGLIST_FROM_POOL          0x00040000
#define SRB_FLAGS_BYPASS_LOCKED_QUEUE       0x00080000

#define SRB_FLAGS_NO_KEEP_AWAKE             0x00100000
#define SRB_FLAGS_PORT_DRIVER_ALLOCSENSE    0x00200000

#define SRB_FLAGS_PORT_DRIVER_SENSEHASPORT  0x00400000
#define SRB_FLAGS_DONT_START_NEXT_PACKET    0x00800000

#define SRB_FLAGS_PORT_DRIVER_RESERVED      0x0F000000
#define SRB_FLAGS_CLASS_DRIVER_RESERVED     0xF0000000

#if DBG==1
//
// A signature used to validate the scsi port number
// at the end of a sense buffer.
//
#define SCSI_PORT_SIGNATURE                 0x54524f50
#endif

//
// Queue Action
//

#define SRB_SIMPLE_TAG_REQUEST              0x20
#define SRB_HEAD_OF_QUEUE_TAG_REQUEST       0x21
#define SRB_ORDERED_QUEUE_TAG_REQUEST       0x22

#define SRB_WMI_FLAGS_ADAPTER_REQUEST       0x01
#define SRB_POWER_FLAGS_ADAPTER_REQUEST     0x01
#define SRB_PNP_FLAGS_ADAPTER_REQUEST       0x01
#define SRB_IOCTL_FLAGS_ADAPTER_REQUEST     0x01
#define SRB_PROTOCOL_FLAGS_ADAPTER_REQUEST  0x01

#if (NTDDI_VERSION >= NTDDI_WIN8)

//
// Define alignment requirements for variable length components in extended
// SRB. For Win64, need to ensure all variable length components are 8 bytes
// align so the pointer fields within the variable length components are 8
// bytes align. Also define pointer field alignment.
//
#if defined(_WIN64) || defined(_M_ALPHA)
#define SRB_ALIGN           DECLSPEC_ALIGN(8)
#define STOR_ADDRESS_ALIGN  DECLSPEC_ALIGN(8)
#define POINTER_ALIGN       DECLSPEC_ALIGN(8)
#else
#define SRB_ALIGN
#define STOR_ADDRESS_ALIGN
#define POINTER_ALIGN
#endif

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

//
// This is the STOR_ADDRESS type used by StorMQ.
// The Controller field will contain the StorMQ controller extension known to the miniport.
//

//
// N.B. The other legacy STOR_ADDRESS_TYPE_xxx values and structs are defined in scsi.h.
// Any updates to STOR_ADDRESS should be put here to not introduce new dependencies on legacy
// SCSI infrastructure.
//
#define STOR_ADDRESS_TYPE_NVME      0x2

#define STOR_ADDR_NVME_ADDRESS_LENGTH    16
typedef struct STOR_ADDRESS_ALIGN _STOR_ADDR_NVME {
    _Field_range_(STOR_ADDRESS_TYPE_NVME, STOR_ADDRESS_TYPE_NVME)
    USHORT Type;
    USHORT Port;
    _Field_range_(STOR_ADDR_NVME_ADDRESS_LENGTH, STOR_ADDR_NVME_ADDRESS_LENGTH)
    ULONG AddressLength;
    PVOID Controller;
    ULONG NamespaceId;
    ULONG Reserved;
} STOR_ADDR_NVME, *PSTOR_ADDR_NVME;

#endif

// SRB extended data types.


typedef enum _SRBEXDATATYPE {
    SrbExDataTypeUnknown = 0,
    SrbExDataTypeBidirectional,
    SrbExDataTypeScsiCdb16 = 0x40,
    SrbExDataTypeScsiCdb32,
    SrbExDataTypeScsiCdbVar,
    SrbExDataTypeNvmeCommand,
    SrbExDataTypeNvmeofOperation,
    SrbExDataTypeMiniportPassthrough,
    SrbExDataTypeWmi = 0x60,
    SrbExDataTypePower,
    SrbExDataTypePnP,
    SrbExDataTypeIoInfo = 0x80,
    SrbExDataTypePassthroughDirect = 0xa0,
    SrbExDataTypeMSReservedStart = 0xf0000000,
    SrbExDataTypeReserved = 0xffffffff
} SRBEXDATATYPE, *PSRBEXDATATYPE;

//
// Generic structure definition for accessing any SRB extended data. All SRB
// extended data must begin with a Type and Length field.
//
typedef struct SRB_ALIGN _SRBEX_DATA {
    SRBEXDATATYPE Type;
    ULONG Length;
    _Field_size_bytes_(Length) UCHAR Data[ANYSIZE_ARRAY];
} SRBEX_DATA, *PSRBEX_DATA;

//
// SRB extended data for bi-directional commands. For these SRBs, it will have both a
// SRB extended data (e.g. SRBEX_DATA_SCSI_CDB*) and a SRBEX_DATA_BIDIRECTIONAL.
//
#define SRBEX_DATA_BIDIRECTIONAL_LENGTH ((2 * sizeof(ULONG)) + sizeof(PVOID))

typedef struct SRB_ALIGN _SRBEX_DATA_BIDIRECTIONAL {
    _Field_range_(SrbExDataTypeBidirectional, SrbExDataTypeBidirectional)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_BIDIRECTIONAL_LENGTH, SRBEX_DATA_BIDIRECTIONAL_LENGTH)
    ULONG Length;
    ULONG DataInTransferLength;
    // Reserved for future to support 64-bit DataTransferInLength
    ULONG Reserved1;
    _Field_size_bytes_full_(DataInTransferLength)
    PVOID POINTER_ALIGN DataInBuffer;
} SRBEX_DATA_BIDIRECTIONAL, *PSRBEX_DATA_BIDIRECTIONAL;


// SRB_FUNCTION_EXECUTE_SCSI for up to 16 byte CDBs
#define SRBEX_DATA_SCSI_CDB16_LENGTH ((20 * sizeof(UCHAR)) + sizeof(ULONG) + sizeof(PVOID))

typedef struct SRB_ALIGN _SRBEX_DATA_SCSI_CDB16 {
    _Field_range_(SrbExDataTypeScsiCdb16, SrbExDataTypeScsiCdb16)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_SCSI_CDB16_LENGTH, SRBEX_DATA_SCSI_CDB16_LENGTH)
    ULONG Length;
    UCHAR ScsiStatus;
    UCHAR SenseInfoBufferLength;
    UCHAR CdbLength;
    UCHAR Reserved;
    ULONG Reserved1;
    _Field_size_bytes_full_(SenseInfoBufferLength)
    PVOID POINTER_ALIGN SenseInfoBuffer;
    UCHAR POINTER_ALIGN Cdb[16];
} SRBEX_DATA_SCSI_CDB16, *PSRBEX_DATA_SCSI_CDB16;

// SRB_FUNCTION_EXECUTE_SCSI for up to 32 byte CDBs
#define SRBEX_DATA_SCSI_CDB32_LENGTH ((36 * sizeof(UCHAR)) + sizeof(ULONG) + sizeof(PVOID))

typedef struct SRB_ALIGN _SRBEX_DATA_SCSI_CDB32 {
    _Field_range_(SrbExDataTypeScsiCdb32, SrbExDataTypeScsiCdb32)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_SCSI_CDB32_LENGTH, SRBEX_DATA_SCSI_CDB32_LENGTH)
    ULONG Length;
    UCHAR ScsiStatus;
    UCHAR SenseInfoBufferLength;
    UCHAR CdbLength;
    UCHAR Reserved;
    ULONG Reserved1;
    _Field_size_bytes_full_(SenseInfoBufferLength)
    PVOID POINTER_ALIGN SenseInfoBuffer;
    UCHAR POINTER_ALIGN Cdb[32];
} SRBEX_DATA_SCSI_CDB32, *PSRBEX_DATA_SCSI_CDB32;

// SRB_FUNCTION_EXECUTE_SCSI for variable length CDBs
#define SRBEX_DATA_SCSI_CDB_VAR_LENGTH_MIN ((4 * sizeof(UCHAR)) + (3 * sizeof(ULONG)) + sizeof(PVOID))

//
// NOTE - may want to cap it instead of using ULONG_MAX. Max is 65KB
// currently (XCDB)
//
#define SRBEX_DATA_SCSI_CDB_VAR_LENGTH_MAX 0xffffffffUL

typedef struct SRB_ALIGN _SRBEX_DATA_SCSI_CDB_VAR {
    _Field_range_(SrbExDataTypeScsiCdbVar, SrbExDataTypeScsiCdbVar)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_SCSI_CDB_VAR_LENGTH_MIN, SRBEX_DATA_SCSI_CDB_VAR_LENGTH_MAX)
    ULONG Length;
    UCHAR ScsiStatus;
    UCHAR SenseInfoBufferLength;
    UCHAR Reserved[2];
    ULONG CdbLength;
    ULONG Reserved1[2];
    _Field_size_bytes_full_(SenseInfoBufferLength)
    PVOID POINTER_ALIGN SenseInfoBuffer;
    _Field_size_bytes_full_(CdbLength)
    UCHAR POINTER_ALIGN Cdb[ANYSIZE_ARRAY];
} SRBEX_DATA_SCSI_CDB_VAR, *PSRBEX_DATA_SCSI_CDB_VAR;

// Used by SRB_FUNCTION_WMI
#define SRBEX_DATA_WMI_LENGTH ((4 * sizeof(UCHAR)) + sizeof(ULONG) + sizeof(PVOID))

typedef struct SRB_ALIGN _SRBEX_DATA_WMI {
    _Field_range_(SrbExDataTypeWmi, SrbExDataTypeWmi)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_WMI_LENGTH, SRBEX_DATA_WMI_LENGTH)
    ULONG Length;
    UCHAR WMISubFunction;
    UCHAR WMIFlags;
    UCHAR Reserved[2];
    ULONG Reserved1;
    PVOID POINTER_ALIGN DataPath;
} SRBEX_DATA_WMI, *PSRBEX_DATA_WMI;

// Used by SRB_FUNCTION_POWER
#define SRBEX_DATA_POWER_LENGTH ((4 * sizeof(UCHAR)) + sizeof(STOR_DEVICE_POWER_STATE) + sizeof(STOR_POWER_ACTION))

typedef struct SRB_ALIGN _SRBEX_DATA_POWER {
    _Field_range_(SrbExDataTypePower, SrbExDataTypePower)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_POWER_LENGTH, SRBEX_DATA_POWER_LENGTH)
    ULONG Length;
    UCHAR SrbPowerFlags;
    UCHAR Reserved[3];
    STOR_DEVICE_POWER_STATE DevicePowerState;
    STOR_POWER_ACTION PowerAction;
} SRBEX_DATA_POWER, *PSRBEX_DATA_POWER;

// Used by SRB_FUNCTION_PNP
#define SRBEX_DATA_PNP_LENGTH ((4 * sizeof(UCHAR)) + sizeof(STOR_PNP_ACTION) + (2 * sizeof(ULONG)))

typedef struct SRB_ALIGN _SRBEX_DATA_PNP {
    _Field_range_(SrbExDataTypePnP, SrbExDataTypePnP)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_PNP_LENGTH, SRBEX_DATA_PNP_LENGTH)
    ULONG Length;
    UCHAR PnPSubFunction;
    UCHAR Reserved[3];
    STOR_PNP_ACTION PnPAction;
    ULONG SrbPnPFlags;
    ULONG Reserved1;
} SRBEX_DATA_PNP, *PSRBEX_DATA_PNP;

// Used by SRB_FUNCTION_MINIPORT_PASSTHROUGH_REQUEST
#define SRBEX_DATA_MINIPORT_PASSTHROUGH_LENGTH ((4 * sizeof(ULONG)))

typedef struct SRB_ALIGN _SRBEX_DATA_MINIPORT_PASSTHROUGH {
    _Field_range_(SrbExDataTypeMiniportPassthrough, SrbExDataTypeMiniportPassthrough)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_MINIPORT_PASSTHROUGH_LENGTH, SRBEX_DATA_MINIPORT_PASSTHROUGH_LENGTH)
    ULONG Length;
    ULONG InputBufferLength;
    ULONG OutputBufferLength;
    ULONG OutputBufferWritten;
    ULONG Reserved;
} SRBEX_DATA_MINIPORT_PASSTHROUGH, *PSRBEX_DATA_MINIPORT_PASSTHROUGH;

// Use in read/write requests to provide additional info about the IO.
#define SRBEX_DATA_IO_INFO_LENGTH ((5 * sizeof(ULONG)) + (4 * sizeof(UCHAR)))

// Define bit flags used in Flags field
#define REQUEST_INFO_NO_CACHE_FLAG                  0x00000001
#define REQUEST_INFO_PAGING_IO_FLAG                 0x00000002
#define REQUEST_INFO_SEQUENTIAL_IO_FLAG             0x00000004
#define REQUEST_INFO_TEMPORARY_FLAG                 0x00000008
#define REQUEST_INFO_WRITE_THROUGH_FLAG             0x00000010
#define REQUEST_INFO_HYBRID_WRITE_THROUGH_FLAG      0x00000020

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define REQUEST_INFO_NO_FILE_OBJECT_FLAG            0x00000040
#define REQUEST_INFO_VOLSNAP_IO_FLAG                0x00000080
#define REQUEST_INFO_STREAM_FLAG                    0x00000100

#endif //(NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define REQUEST_INFO_CRYPTO_FLAG                    0x00000200
#define REQUEST_INFO_VALID_CACHEPRIORITY_FLAG       0x80000000

typedef struct SRB_ALIGN _SRBEX_DATA_IO_INFO {
    _Field_range_(SrbExDataTypeIoInfo, SrbExDataTypeIoInfo)
    SRBEXDATATYPE Type;
    _Field_range_(SRBEX_DATA_IO_INFO_LENGTH, SRBEX_DATA_IO_INFO_LENGTH)
    ULONG Length;
    ULONG Flags;
    ULONG Key;
    ULONG RWLength;
    BOOLEAN IsWriteRequest;
    UCHAR CachePriority;
#if (NTDDI_VERSION >= NTDDI_WIN11_GE)
    UCHAR IoPriorityLevel;
    UCHAR Reserved;
#else
    UCHAR Reserved[2];
#endif //(NTDDI_VERSION >= NTDDI_WIN11_GE)
    ULONG Reserved1[2];
} SRBEX_DATA_IO_INFO, *PSRBEX_DATA_IO_INFO;

//
// Used by SRB_FUNCTION_EXECUTE_NVME
//

#define SRBEX_DATA_NVME_COMMAND_LENGTH ((4 * sizeof(ULONGLONG)) + (14 * sizeof(ULONG)) + (5 * sizeof(USHORT)) + (2 * sizeof(UCHAR)))

typedef enum {
    SRBEX_DATA_NVME_COMMAND_TYPE_NVM     = 0,
    SRBEX_DATA_NVME_COMMAND_TYPE_ADMIN,
    SRBEX_DATA_NVME_COMMAND_TYPE_FABRICS
} SRBEX_DATA_NVME_COMMAND_TYPE, *PSRBEX_DATA_NVME_COMMAND_TYPE;

typedef enum {
    SRBEX_DATA_NVME_COMMAND_FLAG_REQUIRE_DATA_TRANSFER_IN  = 0x1,  // Data is being read in from the device.
    SRBEX_DATA_NVME_COMMAND_FLAG_REQUIRE_DATA_TRANSFER_OUT = 0x2,  // Data is being written out to the device.
    SRBEX_DATA_NVME_COMMAND_FLAG_PRP_SET_ALREADY           = 0x4,
    SRBEX_DATA_NVME_COMMAND_FLAG_SIGNATURE_ENABLED         = 0x8,
    SRBEX_DATA_NVME_COMMAND_FLAG_NO_POLLING                = 0x10  // Indicate to send the command with interrupt mode.
} SRBEX_DATA_NVME_COMMAND_FLAG, *PSRBEX_DATA_NVME_COMMAND_FLAG;

typedef enum {
    SRBEX_DATA_NVME_RESPONSE_FLAG_SQHD_VALID               = 0x1
} SRBEX_DATA_NVME_RESPONSE_FLAG, *PSRBEX_DATA_NVME_RESPONSE_FLAG;

typedef struct SRB_ALIGN _SRBEX_DATA_NVME_COMMAND {

    _Field_range_(SrbExDataTypeNvmeCommand, SrbExDataTypeNvmeCommand)
    SRBEXDATATYPE Type;

    _Field_range_(SRBEX_DATA_NVME_COMMAND_LENGTH, SRBEX_DATA_NVME_COMMAND_LENGTH)
    ULONG Length;

    union {

        //
        // Miniport's handle for the NVMe controller
        //
        PVOID ControllerHandle;
        ULONGLONG Reserved0;
    };

    //
    // NVMe command fields (Directly maps to Common Command Format in NVMe spec)
    //
    union {

        struct {

            ULONG CommandDWORD0;   // NVME_COMMAND_DWORD0 in nvme.h
            ULONG CommandNSID;
            ULONG Reserved1[2];

            ULONGLONG CommandMPTR;

            union {

                struct {
                    ULONGLONG CommandPRP1;
                    ULONGLONG CommandPRP2;
                };

                ULONGLONG CommandSGL1[2];
            };

            ULONG CommandCDW10;
            ULONG CommandCDW11;
            ULONG CommandCDW12;
            ULONG CommandCDW13;
            ULONG CommandCDW14;
            ULONG CommandCDW15;
        };                         // NVME_COMMAND in nvme.h

        struct {

            UCHAR OPC;
            UCHAR PSDT;
            USHORT CID;
            UCHAR FCTYPE;
            UCHAR Reserved[35];
            UCHAR Specific[24];

        } FabricsCommand;          // NVMEOF_FABRICS_COMMAND in nvme.h

        struct {

            ULONG OPC       : 8;        // Opcode (OPC)
            ULONG FUSE      : 2;        // Fused Operation (FUSE)
            ULONG Reserved  : 4;
            ULONG PSDT      : 2;        // PRP or SGL for Data Transfer (PSDT)
            ULONG CID       : 16;       // Command Identifier (CID)
            UCHAR TypeSpecific[60];

        } Command;                 // To reference command DW0

    };

    //
    // Additional command and response information
    //
    UCHAR CommandType;             // Defined in SRBEX_DATA_NVME_COMMAND_TYPE
    UCHAR Reserved2;
    USHORT CommandFlags;           // Defined in SRBEX_DATA_NVME_COMMAND_FLAG
    USHORT ResponseFlags;          // Defined in SRBEX_DATA_NVME_RESPONSE_FLAG

    //
    // Command status
    //
    union {
        struct {
            USHORT  P   : 1;       // Phase Tag (P)
            USHORT  SC  : 8;       // Status Code (SC)
            USHORT  SCT : 3;       // Status Code Type (SCT)
            USHORT  CRD : 2;       // Command Retry Delay (CRD)
            USHORT  M   : 1;       // More (M)
            USHORT  DNR : 1;       // Do Not Retry (DNR)
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;

    } CommandStatus;               // NVME_COMMAND_STATUS in nvme.h

    ULONG QID;                     // Choice of Queue ID, if unspecified it should be 0xFFFFFFFF
    ULONG CommandTag;              // Unique identifier for the command

    //
    // NVMe response fields
    //
    union {

        struct {

            ULONG CQEntryDW0;      // Completion queue entry DW0
            ULONG CQEntryDW1;      // Completion queue entry DW1
        };

        UCHAR Specific[8];         // Fabrics command specific response
    };

    USHORT SQHD;                   // SQ Head Pointer in completion queue entry

    //
    // NVMe command and response fields
    //
    USHORT SQID;                   // SQ Identifier

} SRBEX_DATA_NVME_COMMAND, *PSRBEX_DATA_NVME_COMMAND;


//
// Used by SRB_FUNCTION_NVMEOF_OPERATION
//

#define STOR_NVMEOF_OPERATION_V1             0x0001
#define SRBEX_DATA_NVMEOF_OPERATION_LENGTH   ((2 * sizeof(USHORT)) + (2 * sizeof(ULONG)))

typedef struct SRB_ALIGN _SRBEX_DATA_NVMEOF_OPERATION {

    _Field_range_(SrbExDataTypeNvmeofOperation, SrbExDataTypeNvmeofOperation)
    SRBEXDATATYPE Type;

    _Field_range_(SRBEX_DATA_NVMEOF_OPERATION_LENGTH, SRBEX_DATA_NVMEOF_OPERATION_LENGTH)
    ULONG Length;

    //
    // Version of this structure
    //
    _Field_range_(STOR_NVMEOF_OPERATION_V1, STOR_NVMEOF_OPERATION_V1)
    USHORT Version;

    USHORT Reserved1;

    //
    // Operation specific flags
    //
    ULONG Flags;

    //
    // STOR_NVMEOF_FUNCTION_TYPE (defined in storport.w)
    // Payload if applicable is in data buffer
    //
    ULONG FunctionType;

} SRBEX_DATA_NVMEOF_OPERATION, *PSRBEX_DATA_NVMEOF_OPERATION;


// SRB signature - "SRBX" in ASCII
#define SRB_SIGNATURE 0x53524258

// STORAGE_REQUEST_BLOCK structure version.
#define STORAGE_REQUEST_BLOCK_VERSION_1    0x1


typedef struct SRB_ALIGN _STORAGE_REQUEST_BLOCK_HEADER {
    USHORT Length;

    _Field_range_(SRB_FUNCTION_STORAGE_REQUEST_BLOCK, SRB_FUNCTION_STORAGE_REQUEST_BLOCK)
    UCHAR Function;

    UCHAR SrbStatus;
} STORAGE_REQUEST_BLOCK_HEADER, *PSTORAGE_REQUEST_BLOCK_HEADER;



// STORAGE_REQUEST_BLOCK is the SRB used for SRB_FUNCTION_STORAGE_REQUEST_BLOCK
typedef _Struct_size_bytes_(SrbLength) struct SRB_ALIGN _STORAGE_REQUEST_BLOCK {
    //
    // First 8 bytes of this structure is commom between all SRBs and should
    // have the same meaning as SCSI_REQEUST_BLOCK. To avoid compatibility
    // issues, bytes 4 to 7 will not be used and should be set to 0. These
    // bytes corresponds to the ScsiStatus/WMiSubFunction/SrbPowerFlags/
    // PnPSubFunction, PathId, TargetId and Lun fields in the existing SRBs
    // (e.g. SCSI_REQUEST_BLOCK, SCSI_WMI_REQUEST_BLOCK, etc).
    //
    USHORT Length;

    _Field_range_(SRB_FUNCTION_STORAGE_REQUEST_BLOCK, SRB_FUNCTION_STORAGE_REQUEST_BLOCK)
    UCHAR Function;

    UCHAR SrbStatus;

    // Reserved for internal use
    ULONG ReservedUlong1;

    //
    // General SRB fields. The first 6 fields should not changed between
    // versions of this structure.
    //

    // Signature field
    _Field_range_(SRB_SIGNATURE, SRB_SIGNATURE)
    ULONG Signature;

    // Version field to denote version of STORAGE_REQUEST_BLOCK structure.
    _Field_range_(STORAGE_REQUEST_BLOCK_VERSION_1, STORAGE_REQUEST_BLOCK_VERSION_1)
    ULONG Version;

    // Size of this structure including address and SRB extended data.
    ULONG SrbLength;

    ULONG SrbFunction;

    ULONG SrbFlags;

    // Reserved for future use to expand SrbStatus to 32-bit
    ULONG ReservedUlong2;

    // Equivalent to QueueTag or Task Tag in SCSI
    ULONG RequestTag;

    // Equivalent to Task Priority in SCSI
    USHORT RequestPriority;

    // Equivalent to QueueAction or Task Attributes in SCSI.
    USHORT RequestAttribute;

    // Request timeout value
    ULONG TimeOutValue;

#if (NTDDI_VERSION >= NTDDI_WIN10_CU)

    union {
        //
        // Used to store system failure status information in
        // SrbStatus failure conditions (e.g. SRB_STATUS_INTERNAL_ERROR).
        //
        ULONG SystemStatus;

        //
        // Used to store high 4 bytes of unique tag if unique tag feature is enabled.
        //
        ULONG RequestTagHigh4Bytes;

    } DUMMYUNIONNAME;

#else

    //
    // Used to store system failure status information in
    // SrbStatus failure conditions (e.g. SRB_STATUS_INTERNAL_ERROR).
    //
    ULONG SystemStatus;

#endif

    //
    // Guard page that should always be zero. Use to guard against misbehaving
    // storage filter drivers and to have defined behavior for drivers that misinterpret
    // new SRB format for old on 32-bit platforms.
    //
    ULONG ZeroGuard1;

    //
    // Offset to address from beginning of structure. Address should follow
    // immediately after STORAGE_REQUEST_BLOCK structure and be a STOR_ADDRESS
    // type address.
    //
    _Field_range_(sizeof(STORAGE_REQUEST_BLOCK), SrbLength - sizeof(STOR_ADDRESS))
    ULONG AddressOffset;

    // Number of SRB extended data
    ULONG NumSrbExData;

    // Data transfer length
    ULONG DataTransferLength;

    // Data buffer
    _Field_size_bytes_full_(DataTransferLength)
    PVOID POINTER_ALIGN DataBuffer;

    //
    // Guard page that should always be zero. Use to guard against misbehaving
    // storage filter drivers and to have defined behavior for drivers that misinterpret
    // new SRB format for old on 64-bit platforms.
    //
    PVOID POINTER_ALIGN ZeroGuard2;

    // Original IRP containing the request
    PVOID POINTER_ALIGN OriginalRequest;

    // Class driver's per-request context
    PVOID POINTER_ALIGN ClassContext;

    // Port driver's per-request context
    PVOID POINTER_ALIGN PortContext;

    // Miniport's per-request context
    PVOID POINTER_ALIGN MiniportContext;

    struct _STORAGE_REQUEST_BLOCK POINTER_ALIGN *NextSrb;

    //
    // Offsets to SRB extended data from beginning of structure.
    // Should follow Address field content and be a SRBEX_DATA type extension.
    //
    _At_buffer_(SrbExDataOffset, _Iter_, NumSrbExData, _Field_range_(0, SrbLength - sizeof(SRBEX_DATA)))
    _Field_size_(NumSrbExData)
    ULONG SrbExDataOffset[ANYSIZE_ARRAY];

} STORAGE_REQUEST_BLOCK, *PSTORAGE_REQUEST_BLOCK;

// Define SRB types supported
#define SRB_TYPE_SCSI_REQUEST_BLOCK         0
#define SRB_TYPE_STORAGE_REQUEST_BLOCK      1
#define SRB_TYPE_NVME_REQUEST_BLOCK         2

// Define address type supported
#define STORAGE_ADDRESS_TYPE_BTL8           0
#define STORAGE_ADDRESS_TYPE_NVME           1


#endif //(NTDDI_VERSION >= NTDDI_WIN8)

// end_ntminitape

// end_storport end_privstorport

//
// SCSI Adapter Dependent Routines
//

typedef
_Must_inspect_result_
BOOLEAN
(*PHW_INITIALIZE) (
    _In_ PVOID DeviceExtension
    );

typedef
_Must_inspect_result_
BOOLEAN
(*PHW_STARTIO) (
    _In_ PVOID DeviceExtension,
    _In_ PSCSI_REQUEST_BLOCK Srb
    );

typedef
_Must_inspect_result_
BOOLEAN
(*PHW_INTERRUPT) (
    _In_ PVOID DeviceExtension
    );

typedef
VOID
(*PHW_TIMER) (
    _In_ PVOID DeviceExtension
    );

typedef
VOID
(*PHW_DMA_STARTED) (
    _In_ PVOID DeviceExtension
    );

typedef
_Must_inspect_result_
ULONG
(*PHW_FIND_ADAPTER) (
    _In_ PVOID DeviceExtension,
    _In_ PVOID HwContext,
    _In_ PVOID BusInformation,
    _In_ PCHAR ArgumentString,
    _Inout_ PPORT_CONFIGURATION_INFORMATION ConfigInfo,
    _Out_ PBOOLEAN Again
    );

typedef
_Must_inspect_result_
BOOLEAN
(*PHW_RESET_BUS) (
    _In_ PVOID DeviceExtension,
    _In_ ULONG PathId
    );

typedef
_Must_inspect_result_
BOOLEAN
(*PHW_ADAPTER_STATE) (
    _In_ PVOID DeviceExtension,
    _In_ PVOID Context,
    _In_ BOOLEAN SaveState
    );

typedef
_Must_inspect_result_
SCSI_ADAPTER_CONTROL_STATUS
(*PHW_ADAPTER_CONTROL) (
    _In_ PVOID DeviceExtension,
    _In_ SCSI_ADAPTER_CONTROL_TYPE ControlType,
    _In_ PVOID Parameters
    );

//
// Port driver error codes
//

#define SP_BUS_PARITY_ERROR         0x0001
#define SP_UNEXPECTED_DISCONNECT    0x0002
#define SP_INVALID_RESELECTION      0x0003
#define SP_BUS_TIME_OUT             0x0004
#define SP_PROTOCOL_ERROR           0x0005
#define SP_INTERNAL_ADAPTER_ERROR   0x0006
#define SP_REQUEST_TIMEOUT          0x0007
#define SP_IRQ_NOT_RESPONDING       0x0008
#define SP_BAD_FW_WARNING           0x0009
#define SP_BAD_FW_ERROR             0x000a
#define SP_LOST_WMI_MINIPORT_REQUEST 0x000b

//
// Port driver version flags
//
#define SP_VER_TRACE_SUPPORT        0x0010

//
// Return values for SCSI_HW_FIND_ADAPTER.
//

#define SP_RETURN_NOT_FOUND     0
#define SP_RETURN_FOUND         1
#define SP_RETURN_ERROR         2
#define SP_RETURN_BAD_CONFIG    3

//
// Notification Event Types
//

typedef enum _SCSI_NOTIFICATION_TYPE {
    RequestComplete,
    NextRequest,
    NextLuRequest,
    ResetDetected,
    CallDisableInterrupts,
    CallEnableInterrupts,
    RequestTimerCall,
    BusChangeDetected,     /* New */
    WMIEvent,
    WMIReregister,
    LinkUp,
    LinkDown,
    QueryTickCount,
    BufferOverrunDetected,
    TraceNotification      /* New */
} SCSI_NOTIFICATION_TYPE, *PSCSI_NOTIFICATION_TYPE;

//
// Structure passed between miniport initialization
// and SCSI port initialization
//

typedef struct _HW_INITIALIZATION_DATA {

    ULONG HwInitializationDataSize;

    //
    // Adapter interface type:
    //
    // Internal
    // Isa
    // Eisa
    // MicroChannel
    // TurboChannel
    // PCIBus
    // VMEBus
    // NuBus
    // PCMCIABus
    // CBus
    // MPIBus
    // MPSABus
    //

    INTERFACE_TYPE  AdapterInterfaceType;

    //
    // Miniport driver routines
    //

    PHW_INITIALIZE HwInitialize;

    PHW_STARTIO HwStartIo;

    PHW_INTERRUPT HwInterrupt;

    PHW_FIND_ADAPTER HwFindAdapter;

    PHW_RESET_BUS HwResetBus;

    PHW_DMA_STARTED HwDmaStarted;

    PHW_ADAPTER_STATE HwAdapterState;

    //
    // Miniport driver resources
    //

    ULONG DeviceExtensionSize;

    ULONG SpecificLuExtensionSize;

    ULONG SrbExtensionSize;

    ULONG NumberOfAccessRanges;

    PVOID Reserved;

    //
    // Data buffers must be mapped into virtual address space.
    //

    BOOLEAN MapBuffers;

    //
    // The driver will need to tranlate virtual to physical addresses.
    //

    BOOLEAN NeedPhysicalAddresses;

    //
    // Supports tagged queuing
    //

    BOOLEAN TaggedQueuing;

    //
    // Supports auto request sense.
    //

    BOOLEAN AutoRequestSense;

    //
    // Supports multiple requests per logical unit.
    //

    BOOLEAN MultipleRequestPerLu;

    //
    // Support receive event function.
    //

    BOOLEAN ReceiveEvent;

    //
    // Vendor identification length
    //

    USHORT VendorIdLength;

    //
    // Vendor identification
    //

    PVOID VendorId;

    //
    // Pad for alignment and future use.
    //

    union {

        USHORT ReservedUshort;

        //
        // Flags to indicate supported features
        //
        USHORT PortVersionFlags;
    };

    //
    // Device identification length
    //

    USHORT DeviceIdLength;

    //
    // Device identification
    //

    PVOID DeviceId;

    //
    // Stop adapter routine.
    //

    PHW_ADAPTER_CONTROL HwAdapterControl;

} HW_INITIALIZATION_DATA, *PHW_INITIALIZATION_DATA;

// begin_ntminitape

#ifndef _NTDDK_
#define SCSIPORT_API DECLSPEC_IMPORT
#else
#define SCSIPORT_API
#endif

// end_ntminitape

//
// Port driver routines called by miniport driver
//
//@[comment("MVI_tracked")]
_Must_inspect_result_
_IRQL_requires_max_(PASSIVE_LEVEL)
SCSIPORT_API
ULONG
ScsiPortInitialize(
    _In_ PVOID Argument1,
    _In_ PVOID Argument2,
    _In_ struct _HW_INITIALIZATION_DATA *HwInitializationData,
    _In_ PVOID HwContext
    );

SCSIPORT_API
VOID
ScsiPortFreeDeviceBase(
    _In_ PVOID HwDeviceExtension,
    _In_ PVOID MappedAddress
    );

_Must_inspect_result_
SCSIPORT_API
ULONG
ScsiPortGetBusData(
    _In_ PVOID DeviceExtension,
    _In_ ULONG BusDataType,
    _In_ ULONG SystemIoBusNumber,
    _In_ ULONG SlotNumber,
    _In_reads_bytes_(Length) PVOID Buffer,
    _In_ ULONG Length
    );

_Must_inspect_result_
SCSIPORT_API
ULONG
ScsiPortSetBusDataByOffset(
    _In_ PVOID DeviceExtension,
    _In_ ULONG BusDataType,
    _In_ ULONG SystemIoBusNumber,
    _In_ ULONG SlotNumber,
    _In_reads_bytes_(Length) PVOID Buffer,
    _In_ ULONG Offset,
    _In_ ULONG Length
    );

_Must_inspect_result_
SCSIPORT_API
PVOID
ScsiPortGetDeviceBase(
    _In_ PVOID HwDeviceExtension,
    _In_ INTERFACE_TYPE BusType,
    _In_ ULONG SystemIoBusNumber,
    _In_ SCSI_PHYSICAL_ADDRESS IoAddress,
    _In_ ULONG NumberOfBytes,
    _In_ BOOLEAN InIoSpace
    );

_Must_inspect_result_
SCSIPORT_API
PVOID
ScsiPortGetLogicalUnit(
    _In_ PVOID HwDeviceExtension,
    _In_ UCHAR PathId,
    _In_ UCHAR TargetId,
    _In_ UCHAR Lun
    );

_Must_inspect_result_
SCSIPORT_API
PSCSI_REQUEST_BLOCK
ScsiPortGetSrb(
    _In_ PVOID DeviceExtension,
    _In_ UCHAR PathId,
    _In_ UCHAR TargetId,
    _In_ UCHAR Lun,
    _In_ LONG QueueTag
    );

_Must_inspect_result_
SCSIPORT_API
SCSI_PHYSICAL_ADDRESS
ScsiPortGetPhysicalAddress(
    _In_ PVOID HwDeviceExtension,
    _In_ PSCSI_REQUEST_BLOCK Srb,
    _In_ PVOID VirtualAddress,
    _Out_ ULONG *Length
    );

_Must_inspect_result_
SCSIPORT_API
PVOID
ScsiPortGetVirtualAddress(
    _In_ PVOID HwDeviceExtension,
    _In_ SCSI_PHYSICAL_ADDRESS PhysicalAddress
    );

_Must_inspect_result_
SCSIPORT_API
PVOID
ScsiPortGetUncachedExtension(
    _In_ PVOID HwDeviceExtension,
    _In_ PPORT_CONFIGURATION_INFORMATION ConfigInfo,
    _In_ ULONG NumberOfBytes
    );

SCSIPORT_API
VOID
ScsiPortFlushDma(
    _In_ PVOID DeviceExtension
    );

SCSIPORT_API
VOID
ScsiPortIoMapTransfer(
    _In_ PVOID HwDeviceExtension,
    _In_ PSCSI_REQUEST_BLOCK Srb,
    _In_ PVOID LogicalAddress,
    _In_ ULONG Length
    );

SCSIPORT_API
VOID
ScsiPortNotification(
    _In_ SCSI_NOTIFICATION_TYPE NotificationType,
    _In_ PVOID HwDeviceExtension,
    ...
    );

SCSIPORT_API
VOID
ScsiPortLogError(
    _In_ PVOID HwDeviceExtension,
    _In_ PSCSI_REQUEST_BLOCK Srb OPTIONAL,
    _In_ UCHAR PathId,
    _In_ UCHAR TargetId,
    _In_ UCHAR Lun,
    _In_ ULONG ErrorCode,
    _In_ ULONG UniqueId
    );

SCSIPORT_API
VOID
ScsiPortCompleteRequest(
    _In_ PVOID HwDeviceExtension,
    _In_ UCHAR PathId,
    _In_ UCHAR TargetId,
    _In_ UCHAR Lun,
    _In_ UCHAR SrbStatus
    );

SCSIPORT_API
VOID
ScsiPortStallExecution(
    _In_ ULONG Delay
    );

#if defined(_M_AMD64)

#define ScsiPortReadPortUchar READ_PORT_UCHAR
#define ScsiPortReadPortUshort READ_PORT_USHORT
#define ScsiPortReadPortUlong READ_PORT_ULONG

#define ScsiPortReadPortBufferUchar READ_PORT_BUFFER_UCHAR
#define ScsiPortReadPortBufferUshort READ_PORT_BUFFER_USHORT
#define ScsiPortReadPortBufferUlong READ_PORT_BUFFER_ULONG

#define ScsiPortReadRegisterUchar READ_REGISTER_UCHAR
#define ScsiPortReadRegisterUshort READ_REGISTER_USHORT
#define ScsiPortReadRegisterUlong READ_REGISTER_ULONG

#define ScsiPortReadRegisterBufferUchar READ_REGISTER_BUFFER_UCHAR
#define ScsiPortReadRegisterBufferUshort READ_REGISTER_BUFFER_USHORT
#define ScsiPortReadRegisterBufferUlong READ_REGISTER_BUFFER_ULONG

#define ScsiPortWritePortUchar WRITE_PORT_UCHAR
#define ScsiPortWritePortUshort WRITE_PORT_USHORT
#define ScsiPortWritePortUlong WRITE_PORT_ULONG

#define ScsiPortWritePortBufferUchar WRITE_PORT_BUFFER_UCHAR
#define ScsiPortWritePortBufferUshort WRITE_PORT_BUFFER_USHORT
#define ScsiPortWritePortBufferUlong WRITE_PORT_BUFFER_ULONG

#define ScsiPortWriteRegisterUchar WRITE_REGISTER_UCHAR
#define ScsiPortWriteRegisterUshort WRITE_REGISTER_USHORT
#define ScsiPortWriteRegisterUlong WRITE_REGISTER_ULONG

#define ScsiPortWriteRegisterBufferUchar WRITE_REGISTER_BUFFER_UCHAR
#define ScsiPortWriteRegisterBufferUshort WRITE_REGISTER_BUFFER_USHORT
#define ScsiPortWriteRegisterBufferUlong WRITE_REGISTER_BUFFER_ULONG

#define ScsiPortMoveMemory memmove

#else

_Must_inspect_result_
SCSIPORT_API
UCHAR
ScsiPortReadPortUchar(
    _In_ PUCHAR Port
    );

_Must_inspect_result_
SCSIPORT_API
USHORT
ScsiPortReadPortUshort(
    _In_ PUSHORT Port
    );

_Must_inspect_result_
SCSIPORT_API
ULONG
ScsiPortReadPortUlong(
    _In_ PULONG Port
    );

SCSIPORT_API
VOID
ScsiPortReadPortBufferUchar(
    _In_ PUCHAR Port,
    _In_ PUCHAR Buffer,
    _In_ ULONG  Count
    );

SCSIPORT_API
VOID
ScsiPortReadPortBufferUshort(
    _In_ PUSHORT Port,
    _In_ PUSHORT Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortReadPortBufferUlong(
    _In_ PULONG Port,
    _In_ PULONG Buffer,
    _In_ ULONG Count
    );

_Must_inspect_result_
SCSIPORT_API
UCHAR
ScsiPortReadRegisterUchar(
    _In_ PUCHAR Register
    );

_Must_inspect_result_
SCSIPORT_API
USHORT
ScsiPortReadRegisterUshort(
    _In_ PUSHORT Register
    );

_Must_inspect_result_
SCSIPORT_API
ULONG
ScsiPortReadRegisterUlong(
    _In_ PULONG Register
    );

SCSIPORT_API
VOID
ScsiPortReadRegisterBufferUchar(
    _In_ PUCHAR Register,
    _In_ PUCHAR Buffer,
    _In_ ULONG  Count
    );

SCSIPORT_API
VOID
ScsiPortReadRegisterBufferUshort(
    _In_ PUSHORT Register,
    _In_ PUSHORT Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortReadRegisterBufferUlong(
    _In_ PULONG Register,
    _In_ PULONG Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortWritePortUchar(
    _In_ PUCHAR Port,
    _In_ UCHAR Value
    );

SCSIPORT_API
VOID
ScsiPortWritePortUshort(
    _In_ PUSHORT Port,
    _In_ USHORT Value
    );

SCSIPORT_API
VOID
ScsiPortWritePortUlong(
    _In_ PULONG Port,
    _In_ ULONG Value
    );

SCSIPORT_API
VOID
ScsiPortWritePortBufferUchar(
    _In_ PUCHAR Port,
    _In_ PUCHAR Buffer,
    _In_ ULONG  Count
    );

SCSIPORT_API
VOID
ScsiPortWritePortBufferUshort(
    _In_ PUSHORT Port,
    _In_ PUSHORT Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortWritePortBufferUlong(
    _In_ PULONG Port,
    _In_ PULONG Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortWriteRegisterUchar(
    _In_ PUCHAR Register,
    _In_ UCHAR Value
    );

SCSIPORT_API
VOID
ScsiPortWriteRegisterUshort(
    _In_ PUSHORT Register,
    _In_ USHORT Value
    );

SCSIPORT_API
VOID
ScsiPortWriteRegisterUlong(
    _In_ PULONG Register,
    _In_ ULONG Value
    );

SCSIPORT_API
VOID
ScsiPortWriteRegisterBufferUchar(
    _In_ PUCHAR Register,
    _In_ PUCHAR Buffer,
    _In_ ULONG  Count
    );

SCSIPORT_API
VOID
ScsiPortWriteRegisterBufferUshort(
    _In_ PUSHORT Register,
    _In_ PUSHORT Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortWriteRegisterBufferUlong(
    _In_ PULONG Register,
    _In_ PULONG Buffer,
    _In_ ULONG Count
    );

SCSIPORT_API
VOID
ScsiPortMoveMemory(
    _In_ PVOID WriteBuffer,
    _In_ PVOID ReadBuffer,
    _In_ ULONG Length
    );

#endif

_Must_inspect_result_
SCSIPORT_API
SCSI_PHYSICAL_ADDRESS
ScsiPortConvertUlongToPhysicalAddress(
    _In_ ULONG_PTR UlongAddress
    );

_Must_inspect_result_
SCSIPORT_API
ULONG
ScsiPortConvertPhysicalAddressToUlong(
    _In_ SCSI_PHYSICAL_ADDRESS Address
    );

SCSIPORT_API
VOID
ScsiPortQuerySystemTime(
    _Out_ PLARGE_INTEGER CurrentTime
    );

#define ScsiPortConvertPhysicalAddressToUlong(Address) ((Address).LowPart)

//
// Sundown Note:
// For now, ScsiPortConvertPhysicalAddressToULongPtr() exists only as a macro.
//

#define ScsiPortConvertPhysicalAddressToULongPtr(Address) ((ULONG_PTR)((Address).QuadPart))

_Must_inspect_result_
SCSIPORT_API
BOOLEAN
ScsiPortValidateRange(
    _In_ PVOID HwDeviceExtension,
    _In_ INTERFACE_TYPE BusType,
    _In_ ULONG SystemIoBusNumber,
    _In_ SCSI_PHYSICAL_ADDRESS IoAddress,
    _In_ ULONG NumberOfBytes,
    _In_ BOOLEAN InIoSpace
    );

// begin_ntminitape

SCSIPORT_API
VOID
ScsiDebugPrint(
    ULONG DebugPrintLevel,
    PCCHAR DebugMessage,
    ...
    );

// end_ntminitape

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#pragma warning(pop) // un-sets any local warning changes

#endif //

