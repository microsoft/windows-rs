/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ntddscsi.h

Abstract:

    This is the include file that defines all constants and types for
    accessing the SCSI port adapters.

--*/


//
// Interface GUIDs
//
// need these GUIDs outside conditional includes so that user can
//   #include <ntddscsi.h> in precompiled header
//   #include <initguid.h> in a single source file
//   #include <ntddscsi.h> in that source file a second time to instantiate the GUIDs
//
#ifdef DEFINE_GUID
//
// Make sure FAR is defined...
//
#ifndef FAR
#ifdef _WIN32
#define FAR
#else
#define FAR _far
#endif
#endif

DEFINE_GUID(ScsiRawInterfaceGuid, 0x53f56309L, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
DEFINE_GUID(WmiScsiAddressGuid,   0x53f5630fL, 0xb6bf, 0x11d0, 0x94, 0xf2, 0x00, 0xa0, 0xc9, 0x1e, 0xfb, 0x8b);
#endif

#ifndef _NTDDSCSIH_
#define _NTDDSCSIH_
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
extern "C" {
#endif

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

//
// Device Name - this string is the name of the device.  It is the name
// that should be passed to NtOpenFile when accessing the device.
//
// Note:  For devices that support multiple units, it should be suffixed
//        with the Ascii representation of the unit number.
//

#define IOCTL_SCSI_BASE                 FILE_DEVICE_CONTROLLER
#define FILE_DEVICE_SCSI                0x0000001b

#define DD_SCSI_DEVICE_NAME "\\Device\\ScsiPort"


//
// NtDeviceIoControlFile IoControlCode values for this device.
//
// Warning:  Remember that the low two bits of the code specify how the
//           buffers are passed to the driver!
//

#define IOCTL_SCSI_PASS_THROUGH         CTL_CODE(IOCTL_SCSI_BASE, 0x0401, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_SCSI_MINIPORT             CTL_CODE(IOCTL_SCSI_BASE, 0x0402, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_SCSI_GET_INQUIRY_DATA     CTL_CODE(IOCTL_SCSI_BASE, 0x0403, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCSI_GET_CAPABILITIES     CTL_CODE(IOCTL_SCSI_BASE, 0x0404, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCSI_PASS_THROUGH_DIRECT  CTL_CODE(IOCTL_SCSI_BASE, 0x0405, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_SCSI_GET_ADDRESS          CTL_CODE(IOCTL_SCSI_BASE, 0x0406, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCSI_RESCAN_BUS           CTL_CODE(IOCTL_SCSI_BASE, 0x0407, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCSI_GET_DUMP_POINTERS    CTL_CODE(IOCTL_SCSI_BASE, 0x0408, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_SCSI_FREE_DUMP_POINTERS   CTL_CODE(IOCTL_SCSI_BASE, 0x0409, METHOD_BUFFERED, FILE_ANY_ACCESS)
#define IOCTL_IDE_PASS_THROUGH          CTL_CODE(IOCTL_SCSI_BASE, 0x040a, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_ATA_PASS_THROUGH          CTL_CODE(IOCTL_SCSI_BASE, 0x040b, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_ATA_PASS_THROUGH_DIRECT   CTL_CODE(IOCTL_SCSI_BASE, 0x040c, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_ATA_MINIPORT              CTL_CODE(IOCTL_SCSI_BASE, 0x040d, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_MINIPORT_PROCESS_SERVICE_IRP CTL_CODE(IOCTL_SCSI_BASE,  0x040e, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_MPIO_PASS_THROUGH_PATH    CTL_CODE(IOCTL_SCSI_BASE, 0x040f, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT CTL_CODE(IOCTL_SCSI_BASE, 0x0410, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_SCSI_PASS_THROUGH_EX          CTL_CODE(IOCTL_SCSI_BASE, 0x0411, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_SCSI_PASS_THROUGH_DIRECT_EX   CTL_CODE(IOCTL_SCSI_BASE, 0x0412, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

#define IOCTL_MPIO_PASS_THROUGH_PATH_EX         CTL_CODE(IOCTL_SCSI_BASE, 0x0413, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)
#define IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX  CTL_CODE(IOCTL_SCSI_BASE, 0x0414, METHOD_BUFFERED, FILE_READ_ACCESS | FILE_WRITE_ACCESS)

//
// Note: Function code values of less than 0x800 are reserved for Microsoft. Values of 0x800 and higher can be used by vendors.
//       So do not use function code of 0x800 and higher to define new IOCTLs in this file.
//

//
// Non Volatile Cache support
//

#define IOCTL_SCSI_MINIPORT_NVCACHE           ((FILE_DEVICE_SCSI << 16) + 0x0600)

//
// Hybrid Device support
//
#define IOCTL_SCSI_MINIPORT_HYBRID            ((FILE_DEVICE_SCSI << 16) + 0x0620)

//
// Firmware upgrade support
//
#define IOCTL_SCSI_MINIPORT_FIRMWARE          ((FILE_DEVICE_SCSI << 16) + 0x0780)

//
// Diagnostic support
//
#define IOCTL_SCSI_MINIPORT_DIAGNOSTIC        ((FILE_DEVICE_SCSI << 16) + 0x0900)

//
// Define the SCSI pass through structure.
//

typedef struct _SCSI_PASS_THROUGH {
    USHORT Length;
    UCHAR ScsiStatus;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR CdbLength;
    UCHAR SenseInfoLength;
    UCHAR DataIn;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    ULONG_PTR DataBufferOffset;
    ULONG SenseInfoOffset;
    UCHAR Cdb[16];
}SCSI_PASS_THROUGH, *PSCSI_PASS_THROUGH;

//
// Define the SCSI pass through direct structure.
//

typedef struct _SCSI_PASS_THROUGH_DIRECT {
    USHORT Length;
    UCHAR ScsiStatus;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR CdbLength;
    UCHAR SenseInfoLength;
    UCHAR DataIn;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    PVOID DataBuffer;
    ULONG SenseInfoOffset;
    UCHAR Cdb[16];
}SCSI_PASS_THROUGH_DIRECT, *PSCSI_PASS_THROUGH_DIRECT;


//
// Define the SCSI pass through direct structure for Win64 (thunking).
//
#if defined(_WIN64)
typedef struct _SCSI_PASS_THROUGH32 {
    USHORT Length;
    UCHAR ScsiStatus;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR CdbLength;
    UCHAR SenseInfoLength;
    UCHAR DataIn;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    ULONG32 DataBufferOffset;
    ULONG SenseInfoOffset;
    UCHAR Cdb[16];
}SCSI_PASS_THROUGH32, *PSCSI_PASS_THROUGH32;

//
// Define the SCSI pass through direct structure.
//

typedef struct _SCSI_PASS_THROUGH_DIRECT32 {
    USHORT Length;
    UCHAR ScsiStatus;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR CdbLength;
    UCHAR SenseInfoLength;
    UCHAR DataIn;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    VOID * POINTER_32 DataBuffer;
    ULONG SenseInfoOffset;
    UCHAR Cdb[16];
}SCSI_PASS_THROUGH_DIRECT32, *PSCSI_PASS_THROUGH_DIRECT32;

#endif


//
// Data structure for IOCTL_SCSI_PASS_THROUGH_EX
//

typedef struct _SCSI_PASS_THROUGH_EX {
    ULONG Version;
    ULONG Length;                   // size of the structure
    ULONG CdbLength;                // non-zero value should be set by caller
    ULONG StorAddressLength;        // non-zero value should be set by caller
    UCHAR ScsiStatus;
    UCHAR SenseInfoLength;          // optional, can be zero
    UCHAR DataDirection;            // data transfer direction
    UCHAR Reserved;                 // padding
    ULONG TimeOutValue;
    ULONG StorAddressOffset;        // a value bigger than (structure size + CdbLength) should be set by caller
    ULONG SenseInfoOffset;
    ULONG DataOutTransferLength;    // optional, can be zero
    ULONG DataInTransferLength;     // optional, can be zero
    ULONG_PTR DataOutBufferOffset;
    ULONG_PTR DataInBufferOffset;
    _Field_size_bytes_full_(CdbLength) UCHAR Cdb[ANYSIZE_ARRAY];
} SCSI_PASS_THROUGH_EX, *PSCSI_PASS_THROUGH_EX;


//
// Data structure for IOCTL_SCSI_PASS_THROUGH_DIRECT_EX
//

typedef struct _SCSI_PASS_THROUGH_DIRECT_EX {
    ULONG Version;
    ULONG Length;                   // size of the structure
    ULONG CdbLength;                // non-zero value should be set by caller
    ULONG StorAddressLength;        // non-zero value should be set by caller
    UCHAR ScsiStatus;
    UCHAR SenseInfoLength;          // optional, can be zero
    UCHAR DataDirection;            // data transfer direction
    UCHAR Reserved;                 // padding
    ULONG TimeOutValue;
    ULONG StorAddressOffset;        // a value bigger than (structure size + CdbLength) should be set by caller
    ULONG SenseInfoOffset;
    ULONG DataOutTransferLength;    // optional, can be zero
    ULONG DataInTransferLength;     // optional, can be zero
    _Field_size_bytes_full_(DataOutTransferLength) VOID * DataOutBuffer;
    _Field_size_bytes_full_opt_(DataInTransferLength) VOID * DataInBuffer;
    _Field_size_bytes_full_(CdbLength) UCHAR Cdb[ANYSIZE_ARRAY];
} SCSI_PASS_THROUGH_DIRECT_EX, *PSCSI_PASS_THROUGH_DIRECT_EX;


//
// Structure needed for WOW64 support
//

#if defined(_WIN64)

//
// Data structure for IOCTL_SCSI_PASS_THROUGH_EX
//

typedef struct _SCSI_PASS_THROUGH32_EX {
    ULONG Version;
    ULONG Length;                   // size of the structure
    ULONG CdbLength;                // non-zero value should be set by caller
    ULONG StorAddressLength;        // non-zero value should be set by caller
    UCHAR ScsiStatus;
    UCHAR SenseInfoLength;          // optional, can be zero
    UCHAR DataDirection;            // data transfer direction
    UCHAR Reserved;                 // padding
    ULONG TimeOutValue;
    ULONG StorAddressOffset;        // a value bigger than (structure size + CdbLength) should be set by caller
    ULONG SenseInfoOffset;
    ULONG DataOutTransferLength;    // optional, can be zero
    ULONG DataInTransferLength;     // optional, can be zero
    ULONG32 DataOutBufferOffset;
    ULONG32 DataInBufferOffset;
    _Field_size_bytes_full_(CdbLength) UCHAR Cdb[ANYSIZE_ARRAY];
} SCSI_PASS_THROUGH32_EX, *PSCSI_PASS_THROUGH32_EX;


//
// Data structure for IOCTL_SCSI_PASS_THROUGH_DIRECT_EX
//

typedef struct _SCSI_PASS_THROUGH_DIRECT32_EX {
    ULONG Version;
    ULONG Length;                   // size of the structure
    ULONG CdbLength;                // non-zero value should be set by caller
    ULONG StorAddressLength;        // non-zero value should be set by caller
    UCHAR ScsiStatus;
    UCHAR SenseInfoLength;          // optional, can be zero
    UCHAR DataDirection;            // data transfer direction
    UCHAR Reserved;                 // padding
    ULONG TimeOutValue;
    ULONG StorAddressOffset;        // a value bigger than (structure size + CdbLength) should be set by caller
    ULONG SenseInfoOffset;
    ULONG DataOutTransferLength;    // optional, can be zero
    ULONG DataInTransferLength;     // optional, can be zero
    _Field_size_bytes_full_(DataOutTransferLength) VOID * POINTER_32 DataOutBuffer;
    _Field_size_bytes_full_opt_(DataInTransferLength) VOID * POINTER_32 DataInBuffer;
    _Field_size_bytes_full_(CdbLength) UCHAR Cdb[ANYSIZE_ARRAY];
} SCSI_PASS_THROUGH_DIRECT32_EX, *PSCSI_PASS_THROUGH_DIRECT32_EX;

#endif


//
// ATA pass through structure
//

typedef struct _ATA_PASS_THROUGH_EX {
    USHORT Length;
    USHORT AtaFlags;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR ReservedAsUchar;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    ULONG ReservedAsUlong;
    ULONG_PTR DataBufferOffset;
    UCHAR PreviousTaskFile[8];
    UCHAR CurrentTaskFile[8];
} ATA_PASS_THROUGH_EX, *PATA_PASS_THROUGH_EX;

//
// ATA pass through direct structure.
//

typedef struct _ATA_PASS_THROUGH_DIRECT {
    USHORT Length;
    USHORT AtaFlags;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR ReservedAsUchar;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    ULONG ReservedAsUlong;
    PVOID DataBuffer;
    UCHAR PreviousTaskFile[8];
    UCHAR CurrentTaskFile[8];
} ATA_PASS_THROUGH_DIRECT, *PATA_PASS_THROUGH_DIRECT;

//
// Define the ATA pass through direct structure for Win64 (thunking).
//
#if defined(_WIN64)

typedef struct _ATA_PASS_THROUGH_EX32 {
    USHORT Length;
    USHORT AtaFlags;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR ReservedAsUchar;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    ULONG ReservedAsUlong;
    ULONG32 DataBufferOffset;
    UCHAR PreviousTaskFile[8];
    UCHAR CurrentTaskFile[8];
} ATA_PASS_THROUGH_EX32, *PATA_PASS_THROUGH_EX32;

//
// ATA pass through direct structure.
//

typedef struct _ATA_PASS_THROUGH_DIRECT32 {
    USHORT Length;
    USHORT AtaFlags;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    UCHAR ReservedAsUchar;
    ULONG DataTransferLength;
    ULONG TimeOutValue;
    ULONG ReservedAsUlong;
    VOID * POINTER_32 DataBuffer;
    UCHAR PreviousTaskFile[8];
    UCHAR CurrentTaskFile[8];
} ATA_PASS_THROUGH_DIRECT32, *PATA_PASS_THROUGH_DIRECT32;
#endif

//
// ATA Pass Through Flags
//
#define ATA_FLAGS_DRDY_REQUIRED         (1 << 0)
#define ATA_FLAGS_DATA_IN               (1 << 1)
#define ATA_FLAGS_DATA_OUT              (1 << 2)
#define ATA_FLAGS_48BIT_COMMAND         (1 << 3)
#define ATA_FLAGS_USE_DMA               (1 << 4)
#define ATA_FLAGS_NO_MULTIPLE           (1 << 5)

//
// Define header for IOCTL_ATA_MINIPORT
//

typedef struct _IDE_IO_CONTROL {
        ULONG HeaderLength;
        UCHAR Signature[8];
        ULONG Timeout;
        ULONG ControlCode;
        ULONG ReturnStatus;
        ULONG DataLength;
} IDE_IO_CONTROL, *PIDE_IO_CONTROL;

//
// Define the structure for IOCTL_MPIO_PASS_THROUGH_PATH.
//

typedef struct _MPIO_PASS_THROUGH_PATH {
        SCSI_PASS_THROUGH PassThrough;
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH, *PMPIO_PASS_THROUGH_PATH;

//
// Define the structure for IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT.
//

typedef struct _MPIO_PASS_THROUGH_PATH_DIRECT {
        SCSI_PASS_THROUGH_DIRECT PassThrough;
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH_DIRECT, *PMPIO_PASS_THROUGH_PATH_DIRECT;

//
// Define the structure for IOCTL_MPIO_PASS_THROUGH_PATH_EX.
//

typedef struct _MPIO_PASS_THROUGH_PATH_EX {
        ULONG   PassThroughOffset;  // Offset to a SCSI_PASS_THROUGH_EX structure.
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH_EX, *PMPIO_PASS_THROUGH_PATH_EX;

//
// Define the structure for IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX.
//

typedef struct _MPIO_PASS_THROUGH_PATH_DIRECT_EX {
        ULONG   PassThroughOffset;  // Offset to a PSCSI_PASS_THROUGH_DIRECT_EX structure.
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH_DIRECT_EX, *PMPIO_PASS_THROUGH_PATH_DIRECT_EX;

//
// Define the IOCTL_MPIO_PASS_THROUGH_PATH structure for Win64 (thunking).
//

#if defined(_WIN64)
typedef struct _MPIO_PASS_THROUGH_PATH32 {
        SCSI_PASS_THROUGH32 PassThrough;
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH32, *PMPIO_PASS_THROUGH_PATH32;

//
// Define the IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT structure for Win64 (thunking).
//

typedef struct _MPIO_PASS_THROUGH_PATH_DIRECT32 {
        SCSI_PASS_THROUGH_DIRECT32 PassThrough;
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH_DIRECT32, *PMPIO_PASS_THROUGH_PATH_DIRECT32;

//
// Define the IOCTL_MPIO_PASS_THROUGH_PATH_EX structure for Win64 (thunking).
//

typedef struct _MPIO_PASS_THROUGH_PATH32_EX {
        ULONG   PassThroughOffset;  // Offset to a PSCSI_PASS_THROUGH32_EX structure.
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH32_EX, *PMPIO_PASS_THROUGH_PATH32_EX;

//
// Define the IOCTL_MPIO_PASS_THROUGH_PATH_DIRECT_EX structure for Win64 (thunking).
//

typedef struct _MPIO_PASS_THROUGH_PATH_DIRECT32_EX {
        ULONG   PassThroughOffset;   // Offset to a PSCSI_PASS_THROUGH_DIRECT32_EX structure.
        ULONG   Version;
        USHORT  Length;
        UCHAR   Flags;
        UCHAR   PortNumber;
        ULONGLONG MpioPathId;
} MPIO_PASS_THROUGH_PATH_DIRECT32_EX, *PMPIO_PASS_THROUGH_PATH_DIRECT32_EX;

#endif

//
// Define SCSI information.
// Used with the IOCTL_SCSI_GET_INQUIRY_DATA IOCTL.
//

typedef struct _SCSI_BUS_DATA {
    UCHAR NumberOfLogicalUnits;
    UCHAR InitiatorBusId;
    ULONG InquiryDataOffset;
}SCSI_BUS_DATA, *PSCSI_BUS_DATA;

//
// Define SCSI adapter bus information structure..
// Used with the IOCTL_SCSI_GET_INQUIRY_DATA IOCTL.
//

typedef struct _SCSI_ADAPTER_BUS_INFO {
    UCHAR NumberOfBuses;
    SCSI_BUS_DATA BusData[1];
} SCSI_ADAPTER_BUS_INFO, *PSCSI_ADAPTER_BUS_INFO;

//
// Define SCSI adapter bus information.
// Used with the IOCTL_SCSI_GET_INQUIRY_DATA IOCTL.
//

typedef struct _SCSI_INQUIRY_DATA {
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
    BOOLEAN DeviceClaimed;
    ULONG InquiryDataLength;
    ULONG NextInquiryDataOffset;
    UCHAR InquiryData[1];
}SCSI_INQUIRY_DATA, *PSCSI_INQUIRY_DATA;

//
// Define header for I/O control SRB.
//

//
// Acceptable signatures for SCSI IOCTL MINIPORT calls. Must be equal in byte size to sizeof(SrbIoctl->Signature)
//
#define IOCTL_MINIPORT_SIGNATURE_SCSIDISK           "SCSIDISK"
#define IOCTL_MINIPORT_SIGNATURE_HYBRDISK           "HYBRDISK"
#define IOCTL_MINIPORT_SIGNATURE_DSM_NOTIFICATION   "MPDSM   "
#define IOCTL_MINIPORT_SIGNATURE_DSM_GENERAL        "MPDSMGEN"

#define IOCTL_MINIPORT_SIGNATURE_FIRMWARE           "FIRMWARE"
#define IOCTL_MINIPORT_SIGNATURE_QUERY_PROTOCOL     "PROTOCOL"
#define IOCTL_MINIPORT_SIGNATURE_SET_PROTOCOL       "SETPROTO"
#define IOCTL_MINIPORT_SIGNATURE_QUERY_TEMPERATURE  "TEMPERAT"
#define IOCTL_MINIPORT_SIGNATURE_SET_TEMPERATURE_THRESHOLD  "SETTEMPT"
#define IOCTL_MINIPORT_SIGNATURE_QUERY_PHYSICAL_TOPOLOGY    "TOPOLOGY"
#define IOCTL_MINIPORT_SIGNATURE_ENDURANCE_INFO     "ENDURINF"


typedef struct _SRB_IO_CONTROL {
        ULONG HeaderLength;
        UCHAR Signature[8];
        ULONG Timeout;
        ULONG ControlCode;
        ULONG ReturnCode;
        ULONG Length;
} SRB_IO_CONTROL, *PSRB_IO_CONTROL;

typedef struct _NVCACHE_REQUEST_BLOCK {
    ULONG           NRBSize;
    USHORT          Function;
    ULONG           NRBFlags;
    ULONG           NRBStatus;
    ULONG           Count;
    ULONGLONG       LBA;
    ULONG           DataBufSize;
    ULONG           NVCacheStatus;
    ULONG           NVCacheSubStatus;
} NVCACHE_REQUEST_BLOCK, *PNVCACHE_REQUEST_BLOCK;

#define NRB_FUNCTION_NVCACHE_INFO               0xEC
#define NRB_FUNCTION_SPINDLE_STATUS                 0xE5
#define NRB_FUNCTION_NVCACHE_POWER_MODE_SET         0x00
#define NRB_FUNCTION_NVCACHE_POWER_MODE_RETURN  0x01
#define NRB_FUNCTION_FLUSH_NVCACHE              0x14
#define NRB_FUNCTION_QUERY_PINNED_SET           0x12
#define NRB_FUNCTION_QUERY_CACHE_MISS           0x13
#define NRB_FUNCTION_ADD_LBAS_PINNED_SET        0x10
#define NRB_FUNCTION_REMOVE_LBAS_PINNED_SET     0x11
#define NRB_FUNCTION_QUERY_ASCENDER_STATUS      0xD0
#define NRB_FUNCTION_QUERY_HYBRID_DISK_STATUS   0xD1

//
// The NRB function - NRB_FUNCTION_PASS_HINT_PAYLOAD is deprecated.
//
#define NRB_FUNCTION_PASS_HINT_PAYLOAD          0xE0

//
// Function set to control write back caching policy separated non volatile cache in the miniport
//
#define NRB_FUNCTION_NVSEPARATED_INFO              0xc0
#define NRB_FUNCTION_NVSEPARATED_FLUSH             0xc1
#define NRB_FUNCTION_NVSEPARATED_WB_DISABLE        0xc2
#define NRB_FUNCTION_NVSEPARATED_WB_REVERT_DEFAULT 0xc3


#define NRB_SUCCESS                             0
#define NRB_ILLEGAL_REQUEST                     1
#define NRB_INVALID_PARAMETER                   2
#define NRB_INPUT_DATA_OVERRUN                  3
#define NRB_INPUT_DATA_UNDERRUN                 4
#define NRB_OUTPUT_DATA_OVERRUN                 5
#define NRB_OUTPUT_DATA_UNDERRUN                6

typedef struct _NV_FEATURE_PARAMETER{
        USHORT NVPowerModeEnabled;
        USHORT NVParameterReserv1;
        USHORT NVCmdEnabled;
        USHORT NVParameterReserv2;
        USHORT NVPowerModeVer;
        USHORT NVCmdVer;
        ULONG  NVSize;               // in number of LBA
        USHORT NVReadSpeed;          // in MB/s
        USHORT NVWrtSpeed;
        ULONG  DeviceSpinUpTime;             // in second
} NV_FEATURE_PARAMETER, *PNV_FEATURE_PARAMETER;

#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int
#pragma warning(disable:4201) // nameless struct/unions

//
// NOTE that data structure NVCACHE_HINT_PAYLOAD is deprecated along with NRB function - NRB_FUNCTION_PASS_HINT_PAYLOAD.
//
// Parameter structure for NRB_FUNCTION_PASS_HINT_PAYLOAD request
// This is the corresponding data structure for FIS 0x27 defined in SATA-IO spec.
// Caller that prepares this data structure shall set '0' to any non-used data field.
//
#pragma pack(push, nvcachehintpayload, 1)
typedef struct _NVCACHE_HINT_PAYLOAD {
    UCHAR Command;              // 0x63 or 0x64
    UCHAR Feature7_0;           // when Commmand is 0x63, bit 0 - 3 reflects SubCommand
    UCHAR Feature15_8;
    UCHAR Count15_8;            // when Commmand is 0x64, bit 0 - 4 reflects SubCommand

    UCHAR LBA7_0;
    UCHAR LBA15_8;
    UCHAR LBA23_16;
    UCHAR LBA31_24;

    UCHAR LBA39_32;
    UCHAR LBA47_40;
    UCHAR Auxiliary7_0;
    UCHAR Auxiliary23_16;
    
    UCHAR Reserved[4];
} NVCACHE_HINT_PAYLOAD, *PNVCACHE_HINT_PAYLOAD;
#pragma pack(pop, nvcachehintpayload)

//
// Parameter structure for NRB_FUNCTION_NVSEPARATED_INFO request
//
// Describes parameters of non volatile cache, supported by the miniport (aka separated nv cache).
//
// Caching is associated with the logical unit, it is up to the miniport to implement the exact scope (adapter or unit). MP caching  is relative
// to what miniport considers as primary non volatile media for the unit. Miniport caching layer  is separate from the caching layer integrated
// on the device and managed by the specific NRB functions. Miniport may have to support both separated cache and on device cache as well
// as volatile caches. Volatile cache policy is queried through the StorageProperty interface.
//
// Note that owner of the caching  policy may be miniport or the HBA firmware, in either case miniport is the handler of the policy setting.
//

typedef struct _NV_SEP_CACHE_PARAMETER{
        ULONG   Version;                // Version of the request structure
        ULONG   Size;                   // Size of this structure in bytes

        union {

            struct {
                //
                // Capability flags, describing separated non volatile cache
                //
                UCHAR   WriteCacheEnabled   : 1;   // Is write caching enabled by the miniport as a persistent policy
                UCHAR   WriteCacheChangeable: 1;   // Does the miniport respect change in write caching policy
                UCHAR   WriteThroughIOSupported :1;// Does the miniport support WriteThrough semantics for the NV cache on individual Writes .
                UCHAR   FlushCacheSupported :1;    // Does the miniport support flushing of the NV cache
                UCHAR   ReservedBits: 4;
            }   CacheFlags;

            UCHAR   CacheFlagsSet;
        } Flags;

        //
        // Persistent (cross boot) and effective caching types
        //
        UCHAR   WriteCacheType;         // Type of the NV cache (policy) supported by the miniport - persistent setting.
        UCHAR   WriteCacheTypeEffective;// Effective type of the NV cache (policy) used by the miniport at the time of query.

        UCHAR   ParameterReserve1[3];
} NV_SEP_CACHE_PARAMETER, *PNV_SEP_CACHE_PARAMETER;

//
// Define version value for the structure
//
#define NV_SEP_CACHE_PARAMETER_VERSION_1        1
#define NV_SEP_CACHE_PARAMETER_VERSION          NV_SEP_CACHE_PARAMETER_VERSION_1

//
// Caching policy for separated NV cache.
//
// There are 2 distinct settings fields, one describing policy as used by the miniport
// after boot. The other field describes current effective policy setting, which may be impacted by NRB_FUNCTION_NVSEPARATED_WB_DISABLE
//
// Caller of NRB_FUNCTION_NVSEPARATED_INFO should expect only the effective policy setting to be modified if the query is made after
// call to NRB_FUNCTION_NVSEPARATED_WB_DISABLE (or other control codes in the future).
//
// Control of the persistent policy setting (WriteCacheType) is performed only through miniport proprietary interfaces
//

typedef enum _NV_SEP_WRITE_CACHE_TYPE    {
    NVSEPWriteCacheTypeUnknown    = 0,       // Miniport can't report the type of the write cache
    NVSEPWriteCacheTypeNone      = 1,        // Miniport doesn't support non volatile write cache
    NVSEPWriteCacheTypeWriteBack = 2,        // Miniport supports write back caching
    NVSEPWriteCacheTypeWriteThrough = 3      // Miniport supports write through caching
} NV_SEP_WRITE_CACHE_TYPE, *PNV_SEP_WRITE_CACHE_TYPE;

#pragma warning(pop)

//
// STORAGE_DIAGNOSTIC_STATUS definitions
//
#define STORAGE_DIAGNOSTIC_STATUS_SUCCESS                              0
#define STORAGE_DIAGNOSTIC_STATUS_BUFFER_TOO_SMALL                     0x1
#define STORAGE_DIAGNOSTIC_STATUS_UNSUPPORTED_VERSION                  0x2
#define STORAGE_DIAGNOSTIC_STATUS_INVALID_PARAMETER                    0x3
#define STORAGE_DIAGNOSTIC_STATUS_INVALID_SIGNATURE                    0x4
#define STORAGE_DIAGNOSTIC_STATUS_INVALID_TARGET_TYPE                  0x5
#define STORAGE_DIAGNOSTIC_STATUS_MORE_DATA                            0x6

//
// Diagnostic level allow caller to control what kinds of data the provider should return.
//
// Currently there is only default level defined, provider makes the call to return
// anything it assumes helpful for diagnostic purpose.
//
typedef enum _MP_STORAGE_DIAGNOSTIC_LEVEL {
    MpStorageDiagnosticLevelDefault = 0,
    MpStorageDiagnosticLevelMax
} MP_STORAGE_DIAGNOSTIC_LEVEL, *PMP_STORAGE_DIAGNOSTIC_LEVEL;

typedef enum _MP_STORAGE_DIAGNOSTIC_TARGET_TYPE {

    MpStorageDiagnosticTargetTypeUndefined = 0,
    MpStorageDiagnosticTargetTypeMiniport = 2,
    MpStorageDiagnosticTargetTypeHbaFirmware,
    MpStorageDiagnosticTargetTypeMax

} MP_STORAGE_DIAGNOSTIC_TARGET_TYPE, *PMP_STORAGE_DIAGNOSTIC_TARGET_TYPE;

//
// IOCTL_SCSI_MINIPORT_DIAGNOSTIC
//
// Diagnostic request to Miniport
//

//
// Parameter for STORAGE_DIAGNOSTIC_MP_REQUEST
// Input buffer should contain SRB_IO_CONTROL, STORAGE_DIAGNOSTIC_MP_REQUEST structures.
// 
// Fields in STORAGE_DIAGNOSTIC_MP_REQUEST
// - Input: Version
// - Input: TargetType
// - Input: Level
// - Output: ProviderId
// - Input/Output: BufferSize
//     As input:
//       "BufferSize" should be set to number of bytes allocated for the DataBuffer.
//     As output: 
//       If the request is failed because of buffer too short, "BufferSize" should be set to the
//       length required for DataBuffer by the diagnostic data provider;
//       If the request is successful, it should be filled with returned data size of DataBuffer;
//       For other cases, it should be cleared to 0.
// - Output: DataBuffer
//

typedef struct _STORAGE_DIAGNOSTIC_MP_REQUEST {

    // Size of this structure.
    ULONG Version;

    // Whole size of the structure and the associated data buffer.
    ULONG Size;

    // Request target type.
    MP_STORAGE_DIAGNOSTIC_TARGET_TYPE TargetType;

    // Diagnostic level.
    MP_STORAGE_DIAGNOSTIC_LEVEL Level;

    // GUID of diagnostic data provider.
    GUID ProviderId;

    // Data buffer size.
    ULONG BufferSize;

    // Reserved for future use.
    ULONG Reserved;

    // Diagnostic data buffer.
    _Field_size_(BufferSize) UCHAR DataBuffer[ANYSIZE_ARRAY];

} STORAGE_DIAGNOSTIC_MP_REQUEST, *PSTORAGE_DIAGNOSTIC_MP_REQUEST;

//
// MINIPORT_IOCTL block for data set management notifications
//
// Structure used to describe the list of ranges to process, should match the one in ntddstor.h
//

typedef struct _MP_DEVICE_DATA_SET_RANGE {
    LONGLONG    StartingOffset;         // Measured in bytes,  must align to a sector boundary
    ULONGLONG   LengthInBytes;          // Multiple of sector size.
} MP_DEVICE_DATA_SET_RANGE, *PMP_DEVICE_DATA_SET_RANGE;


typedef struct _DSM_NOTIFICATION_REQUEST_BLOCK {
        ULONG   Size;                   // Length of this structure
        ULONG   Version;                // Version of the template

        ULONG   NotifyFlags;            // Same as in DSM definitions (eg Begin, End)
        ULONG   DataSetProfile;         // Expected profile of IO access (based on OS file type : eg, hibernation, crashdump)
        ULONG   Reserved[3];            // Reserved for future use
        ULONG   DataSetRangesCount;
        MP_DEVICE_DATA_SET_RANGE DataSetRanges[ANYSIZE_ARRAY];
} DSM_NOTIFICATION_REQUEST_BLOCK,*PDSM_NOTIFICATION_REQUEST_BLOCK;

//
// Define version value for the structure
//
#define MINIPORT_DSM_NOTIFICATION_VERSION_1     1
#define MINIPORT_DSM_NOTIFICATION_VERSION       MINIPORT_DSM_NOTIFICATION_VERSION_1

//
// Define access profiles, which are passed to the miniport, as describing expected access pattern to the dataset
//
#define MINIPORT_DSM_PROFILE_UNKNOWN    0
#define MINIPORT_DSM_PROFILE_PAGE_FILE  1
#define MINIPORT_DSM_PROFILE_HIBERNATION_FILE   2
#define MINIPORT_DSM_PROFILE_CRASHDUMP_FILE     3

//
// Notification flag values  - consistent with DSM flags, defined in ntddstor.h
//
#define MINIPORT_DSM_NOTIFY_FLAG_BEGIN            0x00000001  // The given LBA range is being used as defined by the profileID
#define MINIPORT_DSM_NOTIFY_FLAG_END              0x00000002  // The given LBA range is no longer being used as defined by the proileID


//
// Data structure and definitions related to IOCTL_SCSI_MINIPORT_HYBRID
//

#define HYBRID_FUNCTION_GET_INFO                            0x01

#define HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM              0x10
#define HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM               0x11
#define HYBRID_FUNCTION_SET_DIRTY_THRESHOLD                 0x12
#define HYBRID_FUNCTION_DEMOTE_BY_SIZE                      0x13

//
// HYBRID IOCTL status
//
#define HYBRID_STATUS_SUCCESS                             0x0
#define HYBRID_STATUS_ILLEGAL_REQUEST                     0x1
#define HYBRID_STATUS_INVALID_PARAMETER                   0x2
#define HYBRID_STATUS_OUTPUT_BUFFER_TOO_SMALL             0x3

//
// A driver should keep a "RefCount" for HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM.
// e.g. it increases this value every time when it receives a HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM call.
// 
// When driver receives HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM call, it decreases "RefCount" when it's bigger than 0, 
// and then only send command to disable caching medium when the new "RefCount" value is 0.
// 
// In case of HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM is received but no command is sent to device because of decreased "RefCount" is not 0,
// the driver returns following status to inform caller.
//
#define HYBRID_STATUS_ENABLE_REFCOUNT_HOLD                0x10

//
// General input data structure.
// Any data structures in input buffer after SRB_IO_CONTROL should contain HYBRID_REQUEST_BLOCK.
//
#define HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION          0x1

typedef struct _HYBRID_REQUEST_BLOCK {
    ULONG   Version;            // HYBRID_REQUEST_BLOCK_STRUCTURE_VERSION
    ULONG   Size;               // Size of the data structure.
    ULONG   Function;           // Function code
    ULONG   Flags;

    ULONG   DataBufferOffset;   // the offset is from the beginning of buffer. e.g. from beginning of SRB_IO_CONTROL. The value should be multiple of sizeof(PVOID); Value 0 means that there is no data buffer.
    ULONG   DataBufferLength;   // length of the buffer
} HYBRID_REQUEST_BLOCK, *PHYBRID_REQUEST_BLOCK;

//
// Parameter for HYBRID_FUNCTION_GET_INFO
// Input buffer should contain SRB_IO_CONTROL and HYBRID_REQUEST_BLOCK data structures.
// Field "DataBufferOffset" of HYBRID_REQUEST_BLOCK points to output data buffer and HYBRID_INFORMATION should be returned.
//

//
// Output parameter for HYBRID_FUNCTION_GET_INFO
//
typedef enum _NVCACHE_TYPE {
    NvCacheTypeUnknown        = 0,  // Driver can't report the type of the nvcache
    NvCacheTypeNone           = 1,  // Device doesn't support non-volatile cache
    NvCacheTypeWriteBack      = 2,  // Device supports write back caching
    NvCacheTypeWriteThrough   = 3   // Device supports write through caching
} NVCACHE_TYPE;

typedef enum _NVCACHE_STATUS {
    NvCacheStatusUnknown     = 0,   // Driver can't report non-volatile cache status
    NvCacheStatusDisabling   = 1,   // non-volatile cache is in process of being disabled.
    NvCacheStatusDisabled    = 2,   // non-volatile cache has been disabled.
    NvCacheStatusEnabled     = 3    // non-volatile cache has been enabled.
} NVCACHE_STATUS;

typedef struct _NVCACHE_PRIORITY_LEVEL_DESCRIPTOR {
    UCHAR   PriorityLevel;
    UCHAR   Reserved0[3];
    ULONG   ConsumedNVMSizeFraction;
    ULONG   ConsumedMappingResourcesFraction;
    ULONG   ConsumedNVMSizeForDirtyDataFraction;
    ULONG   ConsumedMappingResourcesForDirtyDataFraction;
    ULONG   Reserved1;
} NVCACHE_PRIORITY_LEVEL_DESCRIPTOR, *PNVCACHE_PRIORITY_LEVEL_DESCRIPTOR;

#define HYBRID_REQUEST_INFO_STRUCTURE_VERSION           0x1

#pragma warning(push)
#pragma warning(disable:4214) // bit fields other than int
#pragma warning(disable:4200) // nonstandard extension used : zero-sized array in struct/union

typedef struct _HYBRID_INFORMATION {
    ULONG           Version;                // HYBRID_REQUEST_INFO_STRUCTURE_VERSION
    ULONG           Size;                   // sizeof(HYBRID_INFORMATION)

    BOOLEAN         HybridSupported;
    NVCACHE_STATUS  Status;                 // for hybrid disk, expect values can be: NvCacheStatusDisabling, NvCacheStatusDisabled or NvCacheStatusEnabled
    NVCACHE_TYPE    CacheTypeEffective;     // for hybrid disk, expect value will be: NvCacheTypeWriteBack
    NVCACHE_TYPE    CacheTypeDefault;       // for hybrid disk, expect values can be: NvCacheTypeWriteBack

    ULONG           FractionBase;           // Base value of all fraction type of fields in the data structure. For hybrid disk, value of this field will be 255.

    ULONGLONG       CacheSize;              // total size of NVCache. unit: LBA count

    struct {
        ULONG   WriteCacheChangeable    : 1;    // Does the device respect change in write caching policy
        ULONG   WriteThroughIoSupported : 1;    // Does the device support WriteThrough semantics for the NVCache on individual Writes.
        ULONG   FlushCacheSupported     : 1;    // Does the device support flushing of the NVCache
        ULONG   Removable               : 1;    // Does the nvcache can be removed.
        ULONG   ReservedBits            : 28;
    } Attributes;

    struct {
        UCHAR     PriorityLevelCount;           // A non-zero value indicates the non-volatile cache supports priority levels.
        BOOLEAN   MaxPriorityBehavior;          // If set to TRUE, the disk may fail IO sent with max priority level when it cannot find space for the IO in caching medium.
        UCHAR     OptimalWriteGranularity;      // In LBAs. Value is the power value (of 2). Value 0xFF means that Optimal Write Granularity is not indicated. 
                                                //          For example: value 0 indicates 2^0 = 1 logical sector, 1 indicates 2^1 = 2 logical sectors
        UCHAR     Reserved;

        ULONG     DirtyThresholdLow;            // fraction type of value, with base "FractionBase".
        ULONG     DirtyThresholdHigh;           // fraction type of value, with base "FractionBase".

        struct {
            ULONG   CacheDisable                : 1;    // support of disabling the caching medium
            ULONG   SetDirtyThreshold           : 1;    // support of Setting dirty threshold for the entire caching medium
            ULONG   PriorityDemoteBySize        : 1;    // support of demote by size command
            ULONG   PriorityChangeByLbaRange    : 1;    // support of change by lba command
            ULONG   Evict                       : 1;    // support of evict command
            ULONG   ReservedBits                : 27;

            ULONG   MaxEvictCommands;                   // Max outstanding Evict commands concurrently. Only value when "Evict" value is 1.

            ULONG   MaxLbaRangeCountForEvict;           // Count of LBA ranges can be associated with evict command. Only value when "Evict" value is 1.
            ULONG   MaxLbaRangeCountForChangeLba;       // Count of LBA ranges associated with PriorityChangeByLbaRange command. Only value when "PriorityChangeByLbaRange" value is 1.
        } SupportedCommands;

        NVCACHE_PRIORITY_LEVEL_DESCRIPTOR   Priority[0];

    } Priorities;

} HYBRID_INFORMATION, *PHYBRID_INFORMATION;

#pragma warning(pop)

//
// Parameter for HYBRID_FUNCTION_DISABLE_CACHING_MEDIUM and HYBRID_FUNCTION_ENABLE_CACHING_MEDIUM
// Input buffer should contain SRB_IO_CONTROL and HYBRID_REQUEST_BLOCK data structures.
// Field "DataBufferOffset" of HYBRID_REQUEST_BLOCK should be set to "0" indicating there is no other parameter associated.
// NOTE that these functions don't have output parameter.
//


//
// Parameter for HYBRID_FUNCTION_SET_DIRTY_THRESHOLD
// Input buffer should contain SRB_IO_CONTROL, HYBRID_REQUEST_BLOCK and HYBRID_DIRTY_THRESHOLDS data structures.
// Field "DataBufferOffset" of HYBRID_REQUEST_BLOCK should be set to the starting offset of HYBRID_DIRTY_THRESHOLDS from beginning of buffer.
// NOTE that these functions don't have output parameter.
//
typedef struct _HYBRID_DIRTY_THRESHOLDS {
    ULONG   Version;
    ULONG   Size;               // sizeof(HYBRID_DIRTY_THRESHOLDS)

    ULONG   DirtyLowThreshold;  //
    ULONG   DirtyHighThreshold; // >= DirtyLowThreshold 
} HYBRID_DIRTY_THRESHOLDS, *PHYBRID_DIRTY_THRESHOLDS;

//
// Parameter for HYBRID_FUNCTION_DEMOTE_BY_SIZE
// Input buffer should contain SRB_IO_CONTROL, HYBRID_REQUEST_BLOCK and HYBRID_DEMOTE_BY_SIZE data structures.
// Field "DataBufferOffset" of HYBRID_REQUEST_BLOCK should be set to the starting offset of HYBRID_DEMOTE_BY_SIZE from beginning of buffer.
// NOTE that these functions don't have output parameter.
//
typedef struct _HYBRID_DEMOTE_BY_SIZE {
    ULONG       Version;
    ULONG       Size;               // sizeof(HYBRID_DEMOTE_BY_SIZE)

    UCHAR       SourcePriority;     // 1 ~ max priority
    UCHAR       TargetPriority;     // < SourcePriority
    USHORT      Reserved0;
    ULONG       Reserved1;
    ULONGLONG   LbaCount;           // How many LBAs should be demoted 
} HYBRID_DEMOTE_BY_SIZE, *PHYBRID_DEMOTE_BY_SIZE;



//
// Data structure and definitions related to IOCTL_SCSI_MINIPORT_FIRMWARE
//

#define FIRMWARE_FUNCTION_GET_INFO                          0x01
#define FIRMWARE_FUNCTION_DOWNLOAD                          0x02
#define FIRMWARE_FUNCTION_ACTIVATE                          0x03

//
// FIRMWARE IOCTL status
//
#define FIRMWARE_STATUS_SUCCESS                             0x0
#define FIRMWARE_STATUS_ERROR                               0x1
#define FIRMWARE_STATUS_ILLEGAL_REQUEST                     0x2
#define FIRMWARE_STATUS_INVALID_PARAMETER                   0x3
#define FIRMWARE_STATUS_INPUT_BUFFER_TOO_BIG                0x4
#define FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL             0x5
#define FIRMWARE_STATUS_INVALID_SLOT                        0x6
#define FIRMWARE_STATUS_INVALID_IMAGE                       0x7
#define FIRMWARE_STATUS_CONTROLLER_ERROR                    0x10
#define FIRMWARE_STATUS_POWER_CYCLE_REQUIRED                0x20
#define FIRMWARE_STATUS_DEVICE_ERROR                        0x40
#define FIRMWARE_STATUS_INTERFACE_CRC_ERROR                 0x80
#define FIRMWARE_STATUS_UNCORRECTABLE_DATA_ERROR            0x81
#define FIRMWARE_STATUS_MEDIA_CHANGE                        0x82
#define FIRMWARE_STATUS_ID_NOT_FOUND                        0x83
#define FIRMWARE_STATUS_MEDIA_CHANGE_REQUEST                0x84
#define FIRMWARE_STATUS_COMMAND_ABORT                       0x85
#define FIRMWARE_STATUS_END_OF_MEDIA                        0x86
#define FIRMWARE_STATUS_ILLEGAL_LENGTH                      0x87

//
// For IOCTL_SCSI_MINIPORT_FIRMWARE, the data buffer should contain following structures/fields:
// 1. SRB_IO_CONTROL
//    This is the header data structure indicating which IOCTL is sent. In this case, it's IOCTL_SCSI_MINIPORT_FIRMWARE.
// 2. FIRMWARE_REQUEST_BLOCK
//    This data structure shall be right after SRB_IO_CONTROL. It indicates:
//    a: the function code of firmware request.
//    b: the data buffer location (in field - DataBufferOffset) and length (in field - DataBufferLength) for the firmware function.
//       DataBufferOffset: should have value: ALIGN_UP(sizeof(SRB_IO_CONTROL) + sizeof(FIRMWARE_REQUEST_BLOCK), PVOID). This is to make sure the buffer is pointer aligned.
// 3. Padding. In case of (sizeof(SRB_IO_CONTROL) + sizeof(FIRMWARE_REQUEST_BLOCK)) is not multiple of pointer size, there will be padding space.
// 4. STORAGE_FIRMWARE_INFO or STORAGE_FIRMWARE_DOWNLOAD or STORAGE_FIRMWARE_ACTIVATE depending on the function code of firmware request.
//
#define FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION            0x1

typedef struct _FIRMWARE_REQUEST_BLOCK {
    ULONG   Version;            // FIRMWARE_REQUEST_BLOCK_STRUCTURE_VERSION
    ULONG   Size;               // Size of the data structure.
    ULONG   Function;           // Function code
    ULONG   Flags;

    ULONG   DataBufferOffset;   // the offset is from the beginning of buffer. e.g. from beginning of SRB_IO_CONTROL. The value should be multiple of sizeof(PVOID); Value 0 means that there is no data buffer.
    ULONG   DataBufferLength;   // length of the buffer
} FIRMWARE_REQUEST_BLOCK, *PFIRMWARE_REQUEST_BLOCK;

//
// The request is for Controller if this flag is set. Otherwise, it's for Device/Unit.
//
#define FIRMWARE_REQUEST_FLAG_CONTROLLER                    0x00000001

//
// Indicate that current FW image segment is the last one.
//
#define FIRMWARE_REQUEST_FLAG_LAST_SEGMENT                  0x00000002

//
// Indicate that current FW image segment is the first one.
//
#define FIRMWARE_REQUEST_FLAG_FIRST_SEGMENT                 0x00000004

//
// Indicate that the existing firmware in slot should be activated immediately without
// controller reset.
//
#define FIRMWARE_REQUEST_FLAG_SWITCH_TO_FIRMWARE_WITHOUT_RESET   0x10000000

//
// Indicate that any existing firmware in slot should be replaced with the downloaded image,
// and activated with controller reset.
//
#define FIRMWARE_REQUEST_FLAG_REPLACE_AND_SWITCH_UPON_RESET      0x20000000

//
// Indicate that any existing firmware in slot should be replaced with the downloaded image.
//
#define FIRMWARE_REQUEST_FLAG_REPLACE_EXISTING_IMAGE             0x40000000

//
// Indicate that the existing firmware in slot should be activated. 
// This flag is only valid for fimrware_activate request. It's ignored for other requests.
//
#define FIRMWARE_REQUEST_FLAG_SWITCH_TO_EXISTING_FIRMWARE        0x80000000

//
// Parameter for FIRMWARE_FUNCTION_GET_INFO
// Input buffer should contain SRB_IO_CONTROL and FIRMWARE_REQUEST_BLOCK data structures.
// Field "DataBufferOffset" of FIRMWARE_REQUEST_BLOCK points to output data buffer and STORAGE_FIRMWARE_INFO should be returned.
//

//
// NOTE: In case of (NTDDI_VERSION >= NTDDI_WINTHRESHOLD), applications should use 
//          IOCTL_STORAGE_FIRMWARE_GET_INFO
//          IOCTL_STORAGE_FIRMWARE_DOWNLOAD
//          IOCTL_STORAGE_FIRMWARE_ACTIVATE
//      to work on storage firmware query and update.
//
//      Storage port driver uses FIRMWARE_FUNCTION_GET_INFO as corresponding interface to communicate to miniport.
//      Version 1 data structures below are kept for backwards compatibility only. It's recommended that miniport drivers support Version 2 data structures.
//

//
// Output parameter for FIRMWARE_FUNCTION_GET_INFO
// The total size of returned data is for Firmware Info is:
//   sizeof(STORAGE_FIRMWARE_INFO) + sizeof(STORAGE_FIRMWARE_SLOT_INFO) * SlotCount.
// If the buffer is not big enough, callee should set "FIRMWARE_STATUS_OUTPUT_BUFFER_TOO_SMALL" into "ReturnCode" field of SRB_IO_CONTROL,
// and the required size in "DataBufferLength" field of FIRMWARE_REQUEST_BLOCK.
//

#define STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION         0x1
#define STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2      0x2

#define STORAGE_FIRMWARE_INFO_INVALID_SLOT              0xFF

typedef struct _STORAGE_FIRMWARE_SLOT_INFO {

    UCHAR   SlotNumber;
    BOOLEAN ReadOnly;
    UCHAR   Reserved[6];

    union {
        UCHAR     Info[8];
        ULONGLONG AsUlonglong;
    } Revision;

} STORAGE_FIRMWARE_SLOT_INFO, *PSTORAGE_FIRMWARE_SLOT_INFO;

#define STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH   16

typedef struct _STORAGE_FIRMWARE_SLOT_INFO_V2 {

    UCHAR   SlotNumber;
    BOOLEAN ReadOnly;
    UCHAR   Reserved[6];

    UCHAR   Revision[STORAGE_FIRMWARE_SLOT_INFO_V2_REVISION_LENGTH];

} STORAGE_FIRMWARE_SLOT_INFO_V2, *PSTORAGE_FIRMWARE_SLOT_INFO_V2;


#pragma warning(push)
#pragma warning(disable:4200) // nonstandard extension used : zero-sized array in struct/union

typedef struct _STORAGE_FIRMWARE_INFO {

    ULONG   Version;        // STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION
    ULONG   Size;           // sizeof(STORAGE_FIRMWARE_INFO)

    BOOLEAN UpgradeSupport;
    UCHAR   SlotCount;
    UCHAR   ActiveSlot;
    UCHAR   PendingActivateSlot;

    ULONG   Reserved;

    STORAGE_FIRMWARE_SLOT_INFO Slot[0];

} STORAGE_FIRMWARE_INFO, *PSTORAGE_FIRMWARE_INFO;

typedef struct _STORAGE_FIRMWARE_INFO_V2 {

    ULONG   Version;        // STORAGE_FIRMWARE_INFO_STRUCTURE_VERSION_V2
    ULONG   Size;           // sizeof(STORAGE_FIRMWARE_INFO_V2)

    BOOLEAN UpgradeSupport;
    UCHAR   SlotCount;
    UCHAR   ActiveSlot;
    UCHAR   PendingActivateSlot;

    BOOLEAN FirmwareShared;         // The firmware applies to both device and adapter. For example: PCIe SSD.
    UCHAR   Reserved[3];

    ULONG   ImagePayloadAlignment;  // Number of bytes. Max: PAGE_SIZE. The transfer size should be multiple of this unit size. Some protocol requires at least sector size. 0 means the value is not valid.
    ULONG   ImagePayloadMaxSize;    // for a single command.

    STORAGE_FIRMWARE_SLOT_INFO_V2 Slot[0];

} STORAGE_FIRMWARE_INFO_V2, *PSTORAGE_FIRMWARE_INFO_V2;


//
// Parameter for FIRMWARE_FUNCTION_DOWNLOAD
// Input buffer should contain SRB_IO_CONTROL and FIRMWARE_REQUEST_BLOCK data structures.
// Field "DataBufferOffset" of FIRMWARE_REQUEST_BLOCK points to input data buffer that contains STORAGE_FIRMWARE_DOWNLOAD.
//
#define STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION         0x1
#define STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2      0x2

typedef struct _STORAGE_FIRMWARE_DOWNLOAD {

    ULONG       Version;            // STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION
    ULONG       Size;               // sizeof(STORAGE_FIRMWARE_DOWNLOAD)

    ULONGLONG   Offset;             // image file offset, should be aligned to value of "ImagePayloadAlignment" from STORAGE_FIRMWARE_INFO.
    ULONGLONG   BufferSize;         // should be multiple of value of "ImagePayloadAlignment" from STORAGE_FIRMWARE_INFO

    UCHAR       ImageBuffer[0];     // firmware image file. 

} STORAGE_FIRMWARE_DOWNLOAD, *PSTORAGE_FIRMWARE_DOWNLOAD; 

typedef struct _STORAGE_FIRMWARE_DOWNLOAD_V2 {

    ULONG       Version;            // STORAGE_FIRMWARE_DOWNLOAD_STRUCTURE_VERSION_V2
    ULONG       Size;               // sizeof(STORAGE_FIRMWARE_DOWNLOAD_V2)

    ULONGLONG   Offset;             // image file offset, should be aligned to value of "ImagePayloadAlignment" from STORAGE_FIRMWARE_INFO.
    ULONGLONG   BufferSize;         // should be multiple of value of "ImagePayloadAlignment" from STORAGE_FIRMWARE_INFO

    UCHAR       Slot;
    UCHAR       Reserved[3];
    ULONG       ImageSize;

    UCHAR       ImageBuffer[0];     // firmware image file. 

} STORAGE_FIRMWARE_DOWNLOAD_V2, *PSTORAGE_FIRMWARE_DOWNLOAD_V2; 

#pragma warning(pop)

//
// Parameter for FIRMWARE_FUNCTION_ACTIVATE
// Input buffer should contain SRB_IO_CONTROL and FIRMWARE_REQUEST_BLOCK data structures.
// Field "DataBufferOffset" of FIRMWARE_REQUEST_BLOCK points to input data buffer that contains STORAGE_FIRMWARE_ACTIVATE.
//
#define STORAGE_FIRMWARE_ACTIVATE_STRUCTURE_VERSION         0x1

typedef struct _STORAGE_FIRMWARE_ACTIVATE {

    ULONG   Version;
    ULONG   Size;

    UCHAR   SlotToActivate;
    UCHAR   Reserved0[3];

} STORAGE_FIRMWARE_ACTIVATE, *PSTORAGE_FIRMWARE_ACTIVATE;



//
// SCSI port driver capabilities structure.
//

typedef struct _IO_SCSI_CAPABILITIES {

    //
    // Length of this structure
    //

    ULONG Length;

    //
    // Maximum transfer size in single SRB
    //

    ULONG MaximumTransferLength;

    //
    // Maximum number of physical pages per data buffer
    //

    ULONG MaximumPhysicalPages;

    //
    // Async calls from port to class
    //

    ULONG SupportedAsynchronousEvents;

    //
    // Alignment mask for data transfers.
    //

    ULONG AlignmentMask;

    //
    // Supports tagged queuing
    //

    BOOLEAN TaggedQueuing;

    //
    // Host adapter scans down for bios devices.
    //

    BOOLEAN AdapterScansDown;

    //
    // The host adapter uses programmed I/O.
    //

    BOOLEAN AdapterUsesPio;

} IO_SCSI_CAPABILITIES, *PIO_SCSI_CAPABILITIES;

typedef struct _SCSI_ADDRESS {
    ULONG Length;
    UCHAR PortNumber;
    UCHAR PathId;
    UCHAR TargetId;
    UCHAR Lun;
} SCSI_ADDRESS, *PSCSI_ADDRESS;

//
// Define structure for returning crash dump pointers.
//

struct _ADAPTER_OBJECT;
#define DUMP_POINTERS_VERSION_1         1
#define DUMP_POINTERS_VERSION_2         2
#define DUMP_POINTERS_VERSION_3         3
#define DUMP_POINTERS_VERSION_4         4
#define DUMP_DRIVER_NAME_LENGTH         15

typedef
LONG
DUMP_DEVICE_POWERON_ROUTINE(
    _In_ PVOID Context
    );
typedef DUMP_DEVICE_POWERON_ROUTINE *PDUMP_DEVICE_POWERON_ROUTINE;

typedef struct _DUMP_POINTERS_VERSION {
    //
    // Dump pointers structure version
    //
    ULONG Version;

    //
    // Dump pointers structure size
    //
    ULONG Size;

} DUMP_POINTERS_VERSION, *PDUMP_POINTERS_VERSION;

typedef struct _DUMP_POINTERS {
    struct _ADAPTER_OBJECT *AdapterObject;
    PVOID MappedRegisterBase;
    PVOID DumpData;
    PVOID CommonBufferVa;
    LARGE_INTEGER CommonBufferPa;
    ULONG CommonBufferSize;
    BOOLEAN AllocateCommonBuffers;
#if (NTDDI_VERSION < NTDDI_WINXP)
    UCHAR Spare1[3];
#else
    BOOLEAN UseDiskDump;
    UCHAR Spare1[2];
#endif
    PVOID DeviceObject;
} DUMP_POINTERS, *PDUMP_POINTERS;

typedef struct _DUMP_POINTERS_EX {
    DUMP_POINTERS_VERSION Header;
    PVOID DumpData;
    PVOID CommonBufferVa;
    ULONG CommonBufferSize;
    BOOLEAN AllocateCommonBuffers;
    PVOID DeviceObject;
    PVOID DriverList;

#if (NTDDI_VERSION >= NTDDI_WIN8)

    //
    // Start of DUMP_POINTERS_EX_V3 specific fields
    //
    ULONG       dwPortFlags;            // Bit mask of various flags, returned by the port driver to crashdump driver

    //
    // Configuration values for device telemetry collection - obtained from either live device or from the registry.
    //
    ULONG       MaxDeviceDumpSectionSize;// Size of the buffer to hold complete telemetry section from the storage stack (header, public and private subsections)
    ULONG       MaxDeviceDumpLevel;      // Max level of device dump to request from the device at the crash time

    //
    // Maximum transfer size supported in dump stack
    //
    ULONG MaxTransferSize;

    //
    // Field needed to support DMA operations in diskdump for storport miniports.
    //
    PVOID AdapterObject;
    PVOID MappedRegisterBase;

    //
    // Pointer to stack managed 'device ready for dump'.
    //

    PBOOLEAN DeviceReady;

#endif

 
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
    //
    // Start of DUMP_POINTERS_EX_V4 specific fields
    //

    //
    // Dump device power up callback
    //
    PDUMP_DEVICE_POWERON_ROUTINE DumpDevicePowerOn;
    PVOID DumpDevicePowerOnContext;

#endif

} DUMP_POINTERS_EX, *PDUMP_POINTERS_EX;


#if (NTDDI_VERSION == NTDDI_WIN8)

//
// Size of the different DUMP_POINTERS_EX structures
//

#define DUMP_POINTERS_EX_V2_SIZE    ((ULONG)FIELD_OFFSET(DUMP_POINTERS_EX, dwPortFlags))
#define DUMP_POINTERS_EX_V3_SIZE    sizeof(DUMP_POINTERS_EX)

#elif (NTDDI_VERSION >= NTDDI_WINBLUE)

#define DUMP_POINTERS_EX_V2_SIZE    ((ULONG)FIELD_OFFSET(DUMP_POINTERS_EX, dwPortFlags))
#define DUMP_POINTERS_EX_V3_SIZE    ((ULONG)FIELD_OFFSET(DUMP_POINTERS_EX, DumpDevicePowerOn))
#define DUMP_POINTERS_EX_V4_SIZE    sizeof(DUMP_POINTERS_EX)

#endif

//
// Bit flag values, returned by the port driver in DUMP_POINTERS_EX structure
//

//
// Driver stack & controller support 64-bit physical memory
//
#define DUMP_EX_FLAG_SUPPORT_64BITMEMORY        0x00000001

//
// Driver stack is capable of producing device and driver telemetry data (individual device may not be able to return data)
//
#define DUMP_EX_FLAG_SUPPORT_DD_TELEMETRY       0x00000002

//
// Driver stack is capable of working after a system resume
//
#define DUMP_EX_FLAG_RESUME_SUPPORT             0x00000004

//
// Driver stack is capable to provide driver full path(DUMP_DRIVER_EX is allocated/used by lower stack)
//
#define DUMP_EX_FLAG_DRIVER_FULL_PATH_SUPPORT   0x00000008

typedef struct _DUMP_DRIVER {

    //
    // Dump driver list from port driver
    //
    PVOID DumpDriverList;

    //
    // Name of the driver to be loaded
    //
    WCHAR DriverName[DUMP_DRIVER_NAME_LENGTH];

    //
    // Driver base name
    //
    WCHAR BaseName[DUMP_DRIVER_NAME_LENGTH];

} DUMP_DRIVER, *PDUMP_DRIVER;

//
// Duplicate UNICODE_STRING definition here to eliminate dependancy
// with other header file.
//
// Unicode strings are counted 16-bit character strings. If they are
// NULL terminated, Length does not include trailing NULL.
//

typedef struct _NTSCSI_UNICODE_STRING {
    USHORT Length;
    USHORT MaximumLength;
#ifdef MIDL_PASS
    [size_is(MaximumLength / 2), length_is((Length) / 2) ] USHORT * Buffer;
#else // MIDL_PASS
    _Field_size_bytes_part_opt_(MaximumLength, Length) PWCH Buffer;
#endif // MIDL_PASS
} NTSCSI_UNICODE_STRING;
typedef NTSCSI_UNICODE_STRING *PNTSCSI_UNICODE_STRING;

typedef struct _DUMP_DRIVER_EX {

    //
    // Dump driver list from port driver
    //
    PVOID DumpDriverList;

    //
    // Name of the driver to be loaded
    //
    WCHAR DriverName[DUMP_DRIVER_NAME_LENGTH];

    //
    // Driver base name
    //
    WCHAR BaseName[DUMP_DRIVER_NAME_LENGTH];

    //
    // Driver full path string
    //
    NTSCSI_UNICODE_STRING DriverFullPath;

} DUMP_DRIVER_EX, *PDUMP_DRIVER_EX;


//
// Define values for pass-through DataIn field.
//

#define SCSI_IOCTL_DATA_OUT          0
#define SCSI_IOCTL_DATA_IN           1
#define SCSI_IOCTL_DATA_UNSPECIFIED  2

//
// This value specifies that there is both a data-in and data-out buffer.
// This value is only relevant when used for the DataDirection field of
// SCSI_PASS_THROUGH_EX and SCSI_PASS_THROUGH_DIRECT_EX.
//
#define SCSI_IOCTL_DATA_BIDIRECTIONAL   3

//
// Define values for MPIO-pass-through-path Flags field.
//

#define MPIO_IOCTL_FLAG_USE_PATHID      1
#define MPIO_IOCTL_FLAG_USE_SCSIADDRESS 2
#define MPIO_IOCTL_FLAG_INVOLVE_DSM     4


#pragma warning(push)
#pragma warning(disable:4214)   // bit fields other than int to disable this around the struct
#pragma warning(disable:4201)   // nameless struct/union

//
// Parameters for IOCTL_SCSI_MINIPORT/IOCTL_MINIPORT_SIGNATURE_ENDURANCE_INFO
//      The storage port driver uses these structures when communicating with the miniport.
//

typedef struct _STORAGE_ENDURANCE_INFO {
    ULONG       ValidFields;        // ValidFields represents bit mapping of valid fields of any type
                                    // Eg: Bit 0 stands for GroupId, Bit 1 stands for Flags, Bit 3 for BytesReadCount

    ULONG       GroupId;            // Set Id Eg: Set Id for NVMe sets

    struct {
        ULONG   Shared:1;           // TRUE if information is shared with multiple units/groups

        ULONG   Reserved:31;
    } Flags;

    ULONG       LifePercentage;         // Used life percentage

    UCHAR       BytesReadCount[16];     // Total bytes read from device (Billion Unit)

    UCHAR       ByteWriteCount[16];     // Total bytes written to device (Billion Unit)

} STORAGE_ENDURANCE_INFO, *PSTORAGE_ENDURANCE_INFO;

typedef struct _STORAGE_ENDURANCE_DATA_DESCRIPTOR {
    ULONG                      Version;            // Version of the endurance information structure

    ULONG                      Size;               // Size of the endurance information

    STORAGE_ENDURANCE_INFO     EnduranceInfo;      // Endurance Information of the device

} STORAGE_ENDURANCE_DATA_DESCRIPTOR, *PSTORAGE_ENDURANCE_DATA_DESCRIPTOR;

#pragma warning(pop)

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif

