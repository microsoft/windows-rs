/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    nvme.h

Abstract:

    NVMe command protocol related definitions

Revision:

    Aug. 2018 - Align to NVMe spec version 1.3.

--*/

#ifndef NVME_INCLUDED
#define NVME_INCLUDED

#include <winapifamily.h>

#pragma region Desktop Family or Storage Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_STORAGE)

#if _MSC_VER >= 1200
#pragma once
#pragma warning(push)
#endif

#pragma warning(disable:4214)   // bit field types other than int
#pragma warning(disable:4201)   // nameless struct/union
#pragma warning(disable:4200)   // zero-sized array in struct/union

////////////////////////////////////////////////////////////////////////
//
// NVMe definitions based on 2.0 spec
//

#define NVME_ADMINQ_ID                   0

#define NVME_NQN_MAX_LEN                 256
#define NVME_NQN_NAME_MAX_LEN            223

#define NVME_CONTROLLER_ID_MIN           0x0000
#define NVME_CONTROLLER_ID_MAX           0xFFEF
#define NVME_CONTROLLER_ID_STAT_PERSIST  0xFFFE
#define NVME_CONTROLLER_ID_DYN           0xFFFF

//
// NVMe Controller types reported in
// CNTRLTYPE field of Identify Controller
//
typedef enum _NVME_CONTROLLER_TYPE {

    NvmeCtrlNotReported = 0x00, // Reserved (controller type not reported)
    NvmeCtrlIO          = 0x01, // I/O controller
    NvmeCtrlDiscovery   = 0x02, // Discovery controller
    NvmeCtrlAdmin       = 0x03, // Administrative controller
    NvmeCtrlReservedMin = 0x04,
    NvmeCtrlReservedMax = 0xFF

} NVME_CONTROLLER_TYPE;

//
// NVMe Property Offset defined in
// the Controller Properties section
//
typedef enum _NVME_PROPERTY_OFFSET {

    NvmePropCAP     = 0x0000, // Controller Capabilities
    NvmePropVS      = 0x0008, // Version
    NvmePropINTMS   = 0x000C, // Interrupt Mask Set
    NvmePropINTMC   = 0x0010, // Interrupt Mask Clear  # Note: This is incorrect in the spec.
    NvmePropCC      = 0x0014, // Controller Configuration
    NvmePropCSTS    = 0x001C, // Controller Status
    NvmePropNSSR    = 0x0020, // NVM Subsystem Reset
    NvmePropAQA     = 0x0024, // Admin Queue Attributes
    NvmePropASQ     = 0x0028, // Admin Submission Queue Base Address
    NvmePropACQ     = 0x0030, // Admin Completion Queue Base Address
    NvmePropCMBLOC  = 0x0038, // Controller Memory Buffer Location
    NvmePropCMBSZ   = 0x003C, // Controller Memory Buffer Size
    NvmePropBPINFO  = 0x0040, // Boot Partition Information
    NvmePropBPRSEL  = 0x0044, // Boot Partition Read Select
    NvmePropBPMBL   = 0x0048, // Boot Partition Memory Buffer Location
    NvmePropCMBMSC  = 0x0050, // Controller Memory Buffer Memory Space Control
    NvmePropCMBSTS  = 0x0058, // Controller Memory Buffer Status
    NvmePropCMBEBS  = 0x005C, // Controller Memory Buffer Elasticity Buffer Size
    NvmePropCMBSWTP = 0x0060, // Controller Memory Buffer Sustained Write Throughput
    NvmePropNSSD    = 0x0064, // NVM Subsystem Shutdown
    NvmePropCRTO    = 0x0068, // Controller Ready Timeouts
    NvmePropPMRCAP  = 0x0E00, // Persistent Memory Capabilities
    NvmePropPMRCTL  = 0x0E04, // Persistent Memory Region Control
    NvmePropPMRSTS  = 0x0E08, // Persistent Memory Region Status
    NvmePropPMREBS  = 0x0E0C, // Persistent Memory Region Elasticity Buffer Size
    NvmePropPMRSWTP = 0x0E10, // Persistent Memory Region Sustained Write Throughput
    NvmePropPMRMSCL = 0x0E14, // Persistent Memory Region Controller Memory Space Control Lower
    NvmePropPMRMSCU = 0x0E18  // Persistent Memory Region Controller Memory Space Control Upper

} NVME_PROPERTY_OFFSET;

//
// Offset 0000h: NvmePropCAP (Controller Capabilities)
//
typedef enum {

    NVME_AMS_ROUND_ROBIN                    = 0,
    NVME_AMS_WEIGHTED_ROUND_ROBIN_URGENT    = 1,

} NVME_AMS_OPTION;

typedef enum {

    NVME_CPS_NOT_REPORTED        = 0,
    NVME_CPS_CONTROLLER_SCOPE    = 1,
    NVME_CPS_DOMAIN_SCOPE        = 2,
    NVME_CPS_SUBSYSTEM_SCOPE     = 3

} NVME_CPS_VALUE;

typedef union {

    struct {
        //LSB

        ULONGLONG MQES      : 16;   // RO - Bit 00-15: Maximum Queue Entries Supported (MQES)
        ULONGLONG CQR       : 1;    // RO - Bit 16: Contiguous Queues Required (CQR)

                                    // Bit 17, 18 - AMS; RO - Arbitration Mechanism Supported (AMS)
        ULONGLONG AMS_WeightedRoundRobinWithUrgent : 1; // Bit 17: Weighted Round Robin with Urgent;
        ULONGLONG AMS_VendorSpecific               : 1; // Bit 18: Vendor Specific.

        ULONGLONG Reserved0 : 5;    // RO - Bit 19-23
        ULONGLONG TO        : 8;    // RO - Bit 24-31: Timeout (TO)
        ULONGLONG DSTRD     : 4;    // RO - Bit 32-35: Doorbell Stride (DSTRD)
        ULONGLONG NSSRS     : 1;    // RO - Bit 36: NVM Subsystem Reset Supported (NSSRS)

                                    // Bit 37 ~ 44 - CSS; RO - Command Sets Supported (CSS)
        ULONGLONG CSS_NVM               : 1;  // Bit 37: NVM command set
        ULONGLONG CSS_Reserved0         : 1;  // Bit 38: Reserved
        ULONGLONG CSS_Reserved1         : 1;  // Bit 39: Reserved
        ULONGLONG CSS_Reserved2         : 1;  // Bit 40: Reserved
        ULONGLONG CSS_Reserved3         : 1;  // Bit 41: Reserved
        ULONGLONG CSS_Reserved4         : 1;  // Bit 42: Reserved
        ULONGLONG CSS_MultipleIo        : 1;  // Bit 43: One or more IO command sets
        ULONGLONG CSS_AdminOnly         : 1;  // Bit 44: Only Admin command set (no IO command set)

        ULONGLONG BPS : 1;          // RO - Bit 45: Boot Partition Support
        ULONGLONG CPS : 2;          // RO - Bit 46-47: Controller Power Scope

        ULONGLONG MPSMIN    : 4;    // RO - Bit 48-51: Memory Page Size Minimum (MPSMIN)
        ULONGLONG MPSMAX    : 4;    // RO - Bit 52-55: Memory Page Size Maximum (MPSMAX)
        ULONGLONG PMRS      : 1;    // RO - Bit 56: Persistent Memory Region Supported (PMRS)
        ULONGLONG CMBS      : 1;    // RO - Bit 57: Controller Memory Buffer Supported (CMBS)
        ULONGLONG NSSS      : 1;    // RO - Bit 58: NVM Subsystem Shutdown Supported (NSSS)
        ULONGLONG CRWMS     : 1;    // RO - Bit 59: Controller Ready With Media Support (CRWMS)
        ULONGLONG CRIMS     : 1;    // RO - Bit 60: Controller Ready Independent of Media Support (CRIMS)

        ULONGLONG Reserved2 : 3;    // RO - Bit 61 ~ 63

        //MSB
    } DUMMYSTRUCTNAME;

    ULONGLONG AsUlonglong;

} NVME_CONTROLLER_CAPABILITIES, *PNVME_CONTROLLER_CAPABILITIES;

//
// Offset 0008h: NvmePropVS (Version)
//
typedef union {

    struct {
        //LSB
        ULONG TER       : 8;      // Tertiary Version Number (TER)
        ULONG MNR       : 8;      // Minor Version Number (MNR)
        ULONG MJR       : 16;     // Major Version Number (MJR)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_VERSION, *PNVME_VERSION;

//
// Offset 0014h: NvmePropCC (Controller Configuration)
//
typedef enum {

    NVME_CC_SHN_NO_NOTIFICATION     = 0,
    NVME_CC_SHN_NORMAL_SHUTDOWN     = 1,
    NVME_CC_SHN_ABRUPT_SHUTDOWN     = 2,

} NVME_CC_SHN_SHUTDOWN_NOTIFICATIONS;

typedef enum {

    NVME_CSS_NVM_COMMAND_SET                = 0,
    NVME_CSS_ALL_SUPPORTED_IO_COMMAND_SET   = 6,
    NVME_CSS_ADMIN_COMMAND_SET_ONLY         = 7,

} NVME_CSS_COMMAND_SETS;

typedef union {

    struct {
        //LSB
        ULONG EN        : 1;        // RW - Enable (EN)
        ULONG Reserved0 : 3;        // RO
        ULONG CSS       : 3;        // RW - I/O  Command Set Selected (CSS)
        ULONG MPS       : 4;        // RW - Memory Page Size (MPS)
        ULONG AMS       : 3;        // RW - Arbitration Mechanism Selected (AMS)
        ULONG SHN       : 2;        // RW - Shutdown Notification (SHN)
        ULONG IOSQES    : 4;        // RW - I/O  Submission Queue Entry Size (IOSQES)
        ULONG IOCQES    : 4;        // RW - I/O  Completion Queue Entry Size (IOCQES)
        ULONG CRIME     : 1;        // RO/RW - Controller Ready Independent of Media Enable (CRIME).
                                    //         RO/RW depends on CAP.CRMS
        ULONG Reserved1 : 7;        // RO
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CONTROLLER_CONFIGURATION, *PNVME_CONTROLLER_CONFIGURATION;

//
// Offset 001Ch: NvmePropCSTS (Controller Status)
//
typedef enum {

    NVME_CSTS_SHST_NO_SHUTDOWN          = 0,
    NVME_CSTS_SHST_SHUTDOWN_IN_PROCESS  = 1,
    NVME_CSTS_SHST_SHUTDOWN_COMPLETED   = 2,

} NVME_CSTS_SHST_SHUTDOWN_STATUS;

typedef union {

    struct {
        ULONG RDY       : 1;        // RO - Ready (RDY)
        ULONG CFS       : 1;        // RO - Controller Fatal Status (CFS)
        ULONG SHST      : 2;        // RO - Shutdown Status (SHST)
        ULONG NSSRO     : 1;        // RW1C - NVM Subsystem Reset Occurred (NSSRO)
        ULONG PP        : 1;        // RO - Processing Paused (PP)
        ULONG ST        : 1;        // RO - Shutdown Type (ST)

        ULONG Reserved0 : 25;       // RO
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CONTROLLER_STATUS, *PNVME_CONTROLLER_STATUS;

//
// Offset 0020h: NvmePropNSSR (NVM Subsystem Reset)
//
typedef struct _NVME_NVM_SUBSYSTEM_RESET {

    ULONG   NSSRC;                  // RW - NVM Subsystem Reset Control (NSSRC)

} NVME_NVM_SUBSYSTEM_RESET, *PNVME_NVM_SUBSYSTEM_RESET;

//
// Offset 0024h: NvmePropAQA (Admin Queue Attributes)
//
typedef union {

    struct {
        //LSB
        ULONG ASQS      : 12;       // RW - Admin  Submission Queue Size (ASQS)
        ULONG Reserved0 : 4;        // RO
        ULONG ACQS      : 12;       // RW - Admin  Completion Queue Size (ACQS)
        ULONG Reserved1 : 4;        // RO
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_ADMIN_QUEUE_ATTRIBUTES, *PNVME_ADMIN_QUEUE_ATTRIBUTES;

//
// Offset 0028h: NvmePropASQ (Admin Submission Queue Base Address)
//
typedef union {

    struct {
        //LSB
        ULONGLONG Reserved0 : 12;       // RO
        ULONGLONG ASQB      : 52;       // RW - Admin Submission Queue Base (ASQB)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONGLONG AsUlonglong;

} NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS, *PNVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS;

//
// Offset 0030h: NvmePropACQ (Admin Completion Queue Base Address)
//
typedef union {

    struct {
        //LSB
        ULONGLONG Reserved0 : 12;       // RO
        ULONGLONG ACQB      : 52;       // RW - Admin Completion Queue Base (ACQB)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONGLONG AsUlonglong;

} NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS, *PNVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS;

//
// Offset 0038h: NvmePropCMBLOC (Controller Memory Buffer Location)
//
typedef union {

    struct {
        //LSB
        ULONG   BIR         : 3;    // RO - Base Indicator Register (BIR)
        ULONG   Reserved    : 9;    // RO
        ULONG   OFST        : 20;   // RO - Offset (OFST)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CONTROLLER_MEMORY_BUFFER_LOCATION, *PNVME_CONTROLLER_MEMORY_BUFFER_LOCATION;

//
// Offset 003Ch: NvmePropCMBSZ (Controller Memory Buffer Size)
//
typedef enum {

    NVME_CMBSZ_SIZE_UNITS_4KB       = 0,
    NVME_CMBSZ_SIZE_UNITS_64KB      = 1,
    NVME_CMBSZ_SIZE_UNITS_1MB       = 2,
    NVME_CMBSZ_SIZE_UNITS_16MB      = 3,
    NVME_CMBSZ_SIZE_UNITS_256MB     = 4,
    NVME_CMBSZ_SIZE_UNITS_4GB       = 5,
    NVME_CMBSZ_SIZE_UNITS_64GB      = 6,

} NVME_CMBSZ_SIZE_UNITS;

typedef union {

    struct {
        //LSB
        ULONG   SQS         : 1;    // RO - Submission Queue Support (SQS)
        ULONG   CQS         : 1;    // RO - Completion Queue Support (CQS)
        ULONG   LISTS       : 1;    // RO - PRP SGL List Support (LISTS)
        ULONG   RDS         : 1;    // RO - Read Data Support (RDS)
        ULONG   WDS         : 1;    // RO - Write Data Support (WDS)
        ULONG   Reserved    : 3;    // RO
        ULONG   SZU         : 4;    // RO - Size Units (SZU)
        ULONG   SZ          : 20;   // RO - Size (SZ)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CONTROLLER_MEMORY_BUFFER_SIZE, *PNVME_CONTROLLER_MEMORY_BUFFER_SIZE;


//
// Offset 0064h: NvmePropNSSD (NVM Subsystem Shutdown)
//

#define NVM_SUBSYSTEM_SHUTDOWN_NORMAL       0x4E726D6C
#define NVM_SUBSYSTEM_SHUTDOWN_ABRUPT       0x41627074

typedef struct _NVME_NVM_SUBSYSTEM_SHUTDOWN {

    ULONG   NSSC;                  // RW - NVM Subsystem Shutdown Control (NSSRC)

} NVME_NVM_SUBSYSTEM_SHUTDOWN, *PNVME_NVM_SUBSYSTEM_SHUTDOWN;

//
// Offset 0068h: NvmePropCRTO (Controller Ready Timeouts)
//

typedef union _NVME_CONTROLLER_READY_TIMEOUTS {

    struct {

        USHORT CRWMT;              // RO: Controller Ready With Media Timeout
        USHORT CRIMT;              // RO: Controller Ready Independent of Media Timeout

    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CONTROLLER_READY_TIMEOUTS, *PNVME_CONTROLLER_READY_TIMEOUTS;


//
// Offset (1000h + ((2y) * (4 << CAP.DSTRD))): SQyTDBL (Submission Queue y Tail Doorbell)
//
typedef union {

    struct {
        //LSB
        ULONG SQT       : 16;       // RW - Submission Queue Tail (SQT)
        ULONG Reserved0 : 16;       // RO
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_SUBMISSION_QUEUE_TAIL_DOORBELL, *PNVME_SUBMISSION_QUEUE_TAIL_DOORBELL;

//
// Offset  (1000h + ((2y + 1) * (4 << CAP.DSTRD))): CQyHDBL (Completion Queue y Head Doorbell)
//
typedef union {

    struct {
        //LSB
        ULONG CQH       : 16;       // RW - Completion Queue Head (CQH)
        ULONG Reserved0 : 16;       // RO
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_COMPLETION_QUEUE_HEAD_DOORBELL, *PNVME_COMPLETION_QUEUE_HEAD_DOORBELL;

typedef struct {

    NVME_CONTROLLER_CAPABILITIES    CAP;                // Controller Capabilities; 8 bytes
    NVME_VERSION                    VS;                 // Version
    ULONG                           INTMS;              // Interrupt Mask Set
    ULONG                           INTMC;              // Interrupt Mask Clear
    NVME_CONTROLLER_CONFIGURATION   CC;                 // Controller Configuration
    ULONG                           Reserved0;
    NVME_CONTROLLER_STATUS          CSTS;               // Controller Status
    NVME_NVM_SUBSYSTEM_RESET        NSSR;               // NVM Subsystem Reset (Optional)

    NVME_ADMIN_QUEUE_ATTRIBUTES                 AQA;    // Admin Queue Attributes
    NVME_ADMIN_SUBMISSION_QUEUE_BASE_ADDRESS    ASQ;    // Admin Submission Queue Base Address; 8 bytes
    NVME_ADMIN_COMPLETION_QUEUE_BASE_ADDRESS    ACQ;    // Admin Completion Queue Base Address; 8 bytes

    NVME_CONTROLLER_MEMORY_BUFFER_LOCATION      CMBLOC; // Controller Memory Buffer Location (Optional)
    NVME_CONTROLLER_MEMORY_BUFFER_SIZE          CMBSZ;  // Controller Memory Buffer Size (Optional)

    ULONG                           Reserved1[9];       // 40h ~ 64h

    NVME_NVM_SUBSYSTEM_SHUTDOWN     NSSD;               // NVM Subsystem Shutdown
    NVME_CONTROLLER_READY_TIMEOUTS  CRTO;               // Controller Ready Timeouts

    ULONG                           Reserved2[933];     // 6Ch ~ EFFh
    ULONG                           Reserved3[64];      // F00h ~ FFFh, Command Set Specific

    ULONG                           Doorbells[0];       // Start of the first Doorbell register. (Admin SQ Tail Doorbell)

} NVME_CONTROLLER_REGISTERS, *PNVME_CONTROLLER_REGISTERS;


//
// Command completion status
//
typedef union _NVME_COMMAND_STATUS {

    struct {
        USHORT  P           : 1;        // Phase Tag (P)
        USHORT  SC          : 8;        // Status Code (SC)
        USHORT  SCT         : 3;        // Status Code Type (SCT)
        USHORT  CRD         : 2;        // Command Retry Delay (CRD)
        USHORT  M           : 1;        // More (M)
        USHORT  DNR         : 1;        // Do Not Retry (DNR)
    } DUMMYSTRUCTNAME;

    USHORT AsUshort;

} NVME_COMMAND_STATUS, *PNVME_COMMAND_STATUS;

//
// Command completion entry
//
typedef struct {

    ULONG   DW0;
    ULONG   DW1;

    union {
        struct {
            USHORT  SQHD;               // SQ Head Pointer (SQHD)
            USHORT  SQID;               // SQ Identifier (SQID)
        } DUMMYSTRUCTNAME;

        ULONG   AsUlong;
    } DW2;

    union {
        struct {
            USHORT              CID;    // Command Identifier (CID)
            NVME_COMMAND_STATUS Status;
        } DUMMYSTRUCTNAME;

        ULONG   AsUlong;
    } DW3;

} NVME_COMPLETION_ENTRY, *PNVME_COMPLETION_ENTRY;

//
// Completion entry DW0 for NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST
//

typedef enum {

    NVME_ASYNC_EVENT_TYPE_ERROR_STATUS              = 0,
    NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS             = 1,
    NVME_ASYNC_EVENT_TYPE_NOTICE                    = 2,
    NVME_ASYNC_EVENT_TYPE_IMMEDIATE                 = 3,
    NVME_ASYNC_EVENT_TYPE_IO_COMMAND_SET_STATUS     = 6,
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC           = 7,

} NVME_ASYNC_EVENT_TYPES;

//
// Error Status: NVME_ASYNC_EVENT_TYPE_ERROR_STATUS
//
typedef enum {

    NVME_ASYNC_ERROR_WRITE_TO_INVALID_DOORBELL_REGISTER = 0,
    NVME_ASYNC_ERROR_INVALID_DOORBELL_WRITE_VALUE       = 1,
    NVME_ASYNC_ERROR_DIAG_FAILURE                       = 2,
    NVME_ASYNC_ERROR_PERSISTENT_INTERNAL_DEVICE_ERROR   = 3,
    NVME_ASYNC_ERROR_TRANSIENT_INTERNAL_DEVICE_ERROR    = 4,
    NVME_ASYNC_ERROR_FIRMWARE_IMAGE_LOAD_ERROR          = 5,

} NVME_ASYNC_EVENT_ERROR_STATUS_CODES;

//
// SMART/Health Status: NVME_ASYNC_EVENT_TYPE_HEALTH_STATUS
//
typedef enum {

    NVME_ASYNC_HEALTH_NVM_SUBSYSTEM_RELIABILITY         = 0,
    NVME_ASYNC_HEALTH_TEMPERATURE_THRESHOLD             = 1,
    NVME_ASYNC_HEALTH_SPARE_BELOW_THRESHOLD             = 2,

} NVME_ASYNC_EVENT_HEALTH_STATUS_CODES;

//
// Notice Status: NVME_ASYNC_EVENT_TYPE_NOTICE
//
typedef enum {

    NVME_ASYNC_NOTICE_NAMESPACE_ATTRIBUTE_CHANGED                       = 0,
    NVME_ASYNC_NOTICE_FIRMWARE_ACTIVATION_STARTING                      = 1,
    NVME_ASYNC_NOTICE_TELEMETRY_LOG_CHANGED                             = 2,
    NVME_ASYNC_NOTICE_ASYMMETRIC_ACCESS_CHANGE                          = 3,
    NVME_ASYNC_NOTICE_PREDICTABLE_LATENCY_EVENT_AGGREGATE_LOG_CHANGE    = 4,
    NVME_ASYNC_NOTICE_LBA_STATUS_INFORMATION_ALERT                      = 5,
    NVME_ASYNC_NOTICE_ENDURANCE_GROUP_EVENT_AGGREGATE_LOG_CHANGE        = 6,

    NVME_ASYNC_NOTICE_ZONE_DESCRIPTOR_CHANGED                           = 0xEF,
    NVME_ASYNC_NOTICE_DISCOVERY_LOG_PAGE_CHANGED                        = 0xF0,

} NVME_ASYNC_EVENT_NOTICE_CODES;

//
// Immediate Status: NVME_ASYNC_EVENT_TYPE_IMMEDIATE
//
typedef enum {

    NVME_ASYNC_IMMEDIATE_NVM_SUBSYSTEM_NORMAL_SHUTDOWN      = 0,

} NVME_ASYNC_EVENT_IMMEDIATE_STATUS_CODES;

//
// NVM Command Set Status: NVME_ASYNC_EVENT_TYPE_IO_COMMAND_SET_STATUS
//
typedef enum {

    NVME_ASYNC_IO_CMD_SET_RESERVATION_LOG_PAGE_AVAILABLE                                = 0,
    NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED                                      = 1,
    NVME_ASYNC_IO_CMD_SANITIZE_OPERATION_COMPLETED_WITH_UNEXPECTED_DEALLOCATION         = 2,

} NVME_ASYNC_EVENT_IO_COMMAND_SET_STATUS_CODES;


typedef struct {

    ULONG   AsyncEventType  : 3;
    ULONG   Reserved0       : 5;
    ULONG   AsyncEventInfo  : 8;
    ULONG   LogPage         : 8;
    ULONG   Reserved1       : 8;

} NVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST, *PNVME_COMPLETION_DW0_ASYNC_EVENT_REQUEST;



//
//  Status Code Type (SCT)
//
typedef enum {

    NVME_STATUS_TYPE_GENERIC_COMMAND    = 0,
    NVME_STATUS_TYPE_COMMAND_SPECIFIC   = 1,
    NVME_STATUS_TYPE_MEDIA_ERROR        = 2,
    NVME_STATUS_TYPE_PATH_RELATED       = 3,
    NVME_STATUS_TYPE_VENDOR_SPECIFIC    = 7,

} NVME_STATUS_TYPES;

//
//  Status Code (SC) of NVME_STATUS_TYPE_GENERIC_COMMAND
//
typedef enum {

    NVME_STATUS_SUCCESS_COMPLETION                              = 0x00,
    NVME_STATUS_INVALID_COMMAND_OPCODE                          = 0x01,
    NVME_STATUS_INVALID_FIELD_IN_COMMAND                        = 0x02,
    NVME_STATUS_COMMAND_ID_CONFLICT                             = 0x03,
    NVME_STATUS_DATA_TRANSFER_ERROR                             = 0x04,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_POWER_LOSS_NOTIFICATION  = 0x05,
    NVME_STATUS_INTERNAL_DEVICE_ERROR                           = 0x06,
    NVME_STATUS_COMMAND_ABORT_REQUESTED                         = 0x07,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_SQ_DELETION              = 0x08,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_FUSED_COMMAND     = 0x09,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_FAILED_MISSING_COMMAND   = 0x0A,
    NVME_STATUS_INVALID_NAMESPACE_OR_FORMAT                     = 0x0B,
    NVME_STATUS_COMMAND_SEQUENCE_ERROR                          = 0x0C,
    NVME_STATUS_INVALID_SGL_LAST_SEGMENT_DESCR                  = 0x0D,
    NVME_STATUS_INVALID_NUMBER_OF_SGL_DESCR                     = 0x0E,
    NVME_STATUS_DATA_SGL_LENGTH_INVALID                         = 0x0F,
    NVME_STATUS_METADATA_SGL_LENGTH_INVALID                     = 0x10,
    NVME_STATUS_SGL_DESCR_TYPE_INVALID                          = 0x11,
    NVME_STATUS_INVALID_USE_OF_CONTROLLER_MEMORY_BUFFER         = 0x12,
    NVME_STATUS_PRP_OFFSET_INVALID                              = 0x13,
    NVME_STATUS_ATOMIC_WRITE_UNIT_EXCEEDED                      = 0x14,
    NVME_STATUS_OPERATION_DENIED                                = 0x15,
    NVME_STATUS_SGL_OFFSET_INVALID                              = 0x16,
    NVME_STATUS_RESERVED                                        = 0x17,
    NVME_STATUS_HOST_IDENTIFIER_INCONSISTENT_FORMAT             = 0x18,
    NVME_STATUS_KEEP_ALIVE_TIMEOUT_EXPIRED                      = 0x19,
    NVME_STATUS_KEEP_ALIVE_TIMEOUT_INVALID                      = 0x1A,
    NVME_STATUS_COMMAND_ABORTED_DUE_TO_PREEMPT_ABORT            = 0x1B,
    NVME_STATUS_SANITIZE_FAILED                                 = 0x1C,
    NVME_STATUS_SANITIZE_IN_PROGRESS                            = 0x1D,
    NVME_STATUS_SGL_DATA_BLOCK_GRANULARITY_INVALID              = 0x1E,
    NVME_STATUS_COMMAND_NOT_SUPPORTED_FOR_QUEUE_IN_CMB          = 0x1F,
    NVME_STATUS_NAMESPACE_IS_WRITE_PROTECTED                    = 0x20,
    NVME_STATUS_COMMAND_INTERRUPTED                             = 0x21,
    NVME_STATUS_TRANSIENT_TRANSPORT_ERROR                       = 0x22,
    NVME_STATUS_COMMAND_PROHIBITED_BY_LOCKDOWN                  = 0x23,
    NVME_STATUS_ADMIN_COMMAND_MEDIA_NOT_READY                   = 0x24,

    NVME_STATUS_DIRECTIVE_TYPE_INVALID                          = 0x70,
    NVME_STATUS_DIRECTIVE_ID_INVALID                            = 0x71,

    NVME_STATUS_NVM_LBA_OUT_OF_RANGE                            = 0x80,
    NVME_STATUS_NVM_CAPACITY_EXCEEDED                           = 0x81,
    NVME_STATUS_NVM_NAMESPACE_NOT_READY                         = 0x82,
    NVME_STATUS_NVM_RESERVATION_CONFLICT                        = 0x83,
    NVME_STATUS_FORMAT_IN_PROGRESS                              = 0x84,

} NVME_STATUS_GENERIC_COMMAND_CODES;

//
//  Status Code (SC) of NVME_STATUS_TYPE_COMMAND_SPECIFIC
//
typedef enum {

    NVME_STATUS_COMPLETION_QUEUE_INVALID                            = 0x00,         // Create I/O Submission Queue
    NVME_STATUS_INVALID_QUEUE_IDENTIFIER                            = 0x01,         // Create I/O Submission Queue, Create I/O Completion Queue, Delete I/O Completion Queue, Delete I/O Submission Queue
    NVME_STATUS_MAX_QUEUE_SIZE_EXCEEDED                             = 0x02,         // Create I/O Submission Queue, Create I/O Completion Queue
    NVME_STATUS_ABORT_COMMAND_LIMIT_EXCEEDED                        = 0x03,         // Abort
    NVME_STATUS_ASYNC_EVENT_REQUEST_LIMIT_EXCEEDED                  = 0x05,         // Asynchronous Event Request
    NVME_STATUS_INVALID_FIRMWARE_SLOT                               = 0x06,         // Firmware Commit
    NVME_STATUS_INVALID_FIRMWARE_IMAGE                              = 0x07,         // Firmware Commit
    NVME_STATUS_INVALID_INTERRUPT_VECTOR                            = 0x08,         // Create I/O Completion Queue
    NVME_STATUS_INVALID_LOG_PAGE                                    = 0x09,         // Get Log Page
    NVME_STATUS_INVALID_FORMAT                                      = 0x0A,         // Format NVM, Namespace Management
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_CONVENTIONAL_RESET     = 0x0B,         // Firmware Commit, Sanitize
    NVME_STATUS_INVALID_QUEUE_DELETION                              = 0x0C,         // Delete I/O Completion Queue
    NVME_STATUS_FEATURE_ID_NOT_SAVEABLE                             = 0x0D,         // Set Features
    NVME_STATUS_FEATURE_NOT_CHANGEABLE                              = 0x0E,         // Set Features
    NVME_STATUS_FEATURE_NOT_NAMESPACE_SPECIFIC                      = 0x0F,         // Set Features
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_NVM_SUBSYSTEM_RESET    = 0x10,         // Firmware Commit, Sanitize
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_RESET                  = 0x11,         // Firmware Commit, Sanitize
    NVME_STATUS_FIRMWARE_ACTIVATION_REQUIRES_MAX_TIME_VIOLATION     = 0x12,         // Firmware Commit
    NVME_STATUS_FIRMWARE_ACTIVATION_PROHIBITED                      = 0x13,         // Firmware Commit
    NVME_STATUS_OVERLAPPING_RANGE                                   = 0x14,         // Firmware Commit, Firmware Image Download, Set Features

    NVME_STATUS_NAMESPACE_INSUFFICIENT_CAPACITY                     = 0x15,         // Namespace Management
    NVME_STATUS_NAMESPACE_IDENTIFIER_UNAVAILABLE                    = 0x16,         // Namespace Management
    NVME_STATUS_NAMESPACE_ALREADY_ATTACHED                          = 0x18,         // Namespace Attachment
    NVME_STATUS_NAMESPACE_IS_PRIVATE                                = 0x19,         // Namespace Attachment
    NVME_STATUS_NAMESPACE_NOT_ATTACHED                              = 0x1A,         // Namespace Attachment
    NVME_STATUS_NAMESPACE_THIN_PROVISIONING_NOT_SUPPORTED           = 0x1B,         // Namespace Management
    NVME_STATUS_CONTROLLER_LIST_INVALID                             = 0x1C,         // Namespace Attachment

    NVME_STATUS_DEVICE_SELF_TEST_IN_PROGRESS                        = 0x1D,         // Device Self-test

    NVME_STATUS_BOOT_PARTITION_WRITE_PROHIBITED                     = 0x1E,         // Firmware Commit

    NVME_STATUS_INVALID_CONTROLLER_IDENTIFIER                       = 0x1F,         // Virtualization Management
    NVME_STATUS_INVALID_SECONDARY_CONTROLLER_STATE                  = 0x20,         // Virtualization Management
    NVME_STATUS_INVALID_NUMBER_OF_CONTROLLER_RESOURCES              = 0x21,         // Virtualization Management
    NVME_STATUS_INVALID_RESOURCE_IDENTIFIER                         = 0x22,         // Virtualization Management

    NVME_STATUS_SANITIZE_PROHIBITED_ON_PERSISTENT_MEMORY            = 0x23,         // Sanitize

    NVME_STATUS_INVALID_ANA_GROUP_IDENTIFIER                        = 0x24,         // Namespace Management
    NVME_STATUS_ANA_ATTACH_FAILED                                   = 0x25,         // Namespace Attachment
    NVME_STATUS_INSUFFICIENT_CAPACITY                               = 0x26,         // Capacity Management
    NVME_STATUS_NAMESPACE_ATTACHMENT_LIMIT_EXCEEDED                 = 0x27,         // Namespace Attachment
    NVME_STATUS_PROHIBITION_NOT_SUPPORTED                           = 0x28,         // Lockdown

    NVME_IO_COMMAND_SET_NOT_SUPPORTED                               = 0x29,         // Namespace Attachment/Management, Get Log Page
    NVME_IO_COMMAND_SET_NOT_ENABLED                                 = 0x2A,         // Namespace Attachment
    NVME_IO_COMMAND_SET_COMBINATION_REJECTED                        = 0x2B,         // Set Features
    NVME_IO_COMMAND_SET_INVALID                                     = 0x2C,         // Identify, Namespace Management

    NVME_STATUS_INVALID_DISCOVERY_INFORMATION                       = 0x2F,         // Discovery Information Management
    NVME_STATUS_INSUFFICIENT_DISCOVERY_RESOURCES                    = 0x32,         // Discovery Information Management

    NVME_STATUS_STREAM_RESOURCE_ALLOCATION_FAILED                   = 0x7F,         // Streams Directive
    NVME_STATUS_ZONE_INVALID_FORMAT                                 = 0x7F,         // Namespace Management

    NVME_STATUS_NVM_CONFLICTING_ATTRIBUTES                          = 0x80,         // Dataset Management, Read, Write
    NVME_STATUS_NVM_INVALID_PROTECTION_INFORMATION                  = 0x81,         // Compare, Read, Write, Write Zeroes, Verify
    NVME_STATUS_NVM_ATTEMPTED_WRITE_TO_READ_ONLY_RANGE              = 0x82,         // Dataset Management, Write, Write Uncorrectable, Write Zeroes
    NVME_STATUS_NVM_COMMAND_SIZE_LIMIT_EXCEEDED                     = 0x83,         // Dataset Management

    NVME_STATUS_ZONE_BOUNDARY_ERROR                                 = 0xB8,         // Compare, Read, Verify, Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_FULL                                           = 0xB9,         // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_READ_ONLY                                      = 0xBA,         // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_OFFLINE                                        = 0xBB,         // Compare, Read, Verify, Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append
    NVME_STATUS_ZONE_INVALID_WRITE                                  = 0xBC,         // Write, Write Uncorrectable, Write Zeroes, Copy
    NVME_STATUS_ZONE_TOO_MANY_ACTIVE                                = 0xBD,         // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append, Zone Management Send
    NVME_STATUS_ZONE_TOO_MANY_OPEN                                  = 0xBE,         // Write, Write Uncorrectable, Write Zeroes, Copy, Zone Append, Zone Management Send
    NVME_STATUS_ZONE_INVALID_STATE_TRANSITION                       = 0xBF,         // Zone Management Send

} NVME_STATUS_COMMAND_SPECIFIC_CODES;

//
//  Status Code (SC) of NVME_STATUS_TYPE_COMMAND_SPECIFIC, Fabrics Commands (NVME_ADMIN_COMMAND_FABRICS)
//
typedef enum {

    NVME_STATUS_INCOMPATIBLE_FORMAT                                 = 0x80,         // Connect, Disconnect
    NVME_STATUS_CONTROLLER_BUSY                                     = 0x81,         // Connect, Disconnect
    NVME_STATUS_CONNECT_INVALID_PARAMETERS                          = 0x82,         // Connect
    NVME_STATUS_CONNECT_RESTART_DISCOVERY                           = 0x83,         // Connect
    NVME_STATUS_CONNECT_INVALID_HOST                                = 0x84,         // Connect
    NVME_STATUS_INVALID_QUEUE_TYPE                                  = 0x85,         // Disconnect

    NVME_STATUS_DISCOVER_RESTART                                    = 0x90,         // Get Log Page
    NVME_STATUS_AUTHENTICATION_REQUIRED                             = 0x91,         // Queue has not yet been authenticated, in-band authentication is required

} NVME_STATUS_FABRIC_COMMAND_CODES;

//
//  Status Code (SC) of NVME_STATUS_TYPE_PATH_RELATED
//
typedef enum {

    NVME_STATUS_INTERNAL_PATH_ERROR                         = 0x00,
    NVME_STATUS_ASYMMETRIC_ACCESS_PERSISTENT_LOSS           = 0x01,
    NVME_STATUS_ASYMMETRIC_ACCESS_INACCESSIBLE              = 0x02,
    NVME_STATUS_ASYMMETRIC_ACCESS_TRANSITION                = 0x03,

    NVME_STATUS_CONTROLLER_PATHING_ERROR                    = 0x60,

    NVME_STATUS_HOST_PATHING_ERROR                          = 0x70,
    NVME_STATUS_COMMAND_ABORTED_BY_HOST                     = 0x71,

} NVME_STATUS_PATH_ERROR_CODES;

//
//  Status Code (SC) of NVME_STATUS_TYPE_MEDIA_ERROR
//
typedef enum {

    NVME_STATUS_NVM_WRITE_FAULT                             = 0x80,
    NVME_STATUS_NVM_UNRECOVERED_READ_ERROR                  = 0x81,
    NVME_STATUS_NVM_END_TO_END_GUARD_CHECK_ERROR            = 0x82,
    NVME_STATUS_NVM_END_TO_END_APPLICATION_TAG_CHECK_ERROR  = 0x83,
    NVME_STATUS_NVM_END_TO_END_REFERENCE_TAG_CHECK_ERROR    = 0x84,
    NVME_STATUS_NVM_COMPARE_FAILURE                         = 0x85,
    NVME_STATUS_NVM_ACCESS_DENIED                           = 0x86,
    NVME_STATUS_NVM_DEALLOCATED_OR_UNWRITTEN_LOGICAL_BLOCK  = 0x87,

} NVME_STATUS_MEDIA_ERROR_CODES;

//
// Admin Command Set
//
typedef enum {

    NVME_ADMIN_COMMAND_DELETE_IO_SQ             = 0x00,
    NVME_ADMIN_COMMAND_CREATE_IO_SQ             = 0x01,
    NVME_ADMIN_COMMAND_GET_LOG_PAGE             = 0x02,
    NVME_ADMIN_COMMAND_DELETE_IO_CQ             = 0x04,
    NVME_ADMIN_COMMAND_CREATE_IO_CQ             = 0x05,
    NVME_ADMIN_COMMAND_IDENTIFY                 = 0x06,
    NVME_ADMIN_COMMAND_ABORT                    = 0x08,
    NVME_ADMIN_COMMAND_SET_FEATURES             = 0x09,
    NVME_ADMIN_COMMAND_GET_FEATURES             = 0x0A,
    NVME_ADMIN_COMMAND_ASYNC_EVENT_REQUEST      = 0x0C,
    NVME_ADMIN_COMMAND_NAMESPACE_MANAGEMENT     = 0x0D,

    NVME_ADMIN_COMMAND_FIRMWARE_ACTIVATE        = 0x10,
    NVME_ADMIN_COMMAND_FIRMWARE_COMMIT          = 0x10,         // "Firmware Activate" command has been renamed to "Firmware Commit" command in spec v1.2
    NVME_ADMIN_COMMAND_FIRMWARE_IMAGE_DOWNLOAD  = 0x11,
    NVME_ADMIN_COMMAND_DEVICE_SELF_TEST         = 0x14,
    NVME_ADMIN_COMMAND_NAMESPACE_ATTACHMENT     = 0x15,

    NVME_ADMIN_COMMAND_KEEP_ALIVE               = 0x18,
    NVME_ADMIN_COMMAND_DIRECTIVE_SEND           = 0x19,
    NVME_ADMIN_COMMAND_DIRECTIVE_RECEIVE        = 0x1A,
    NVME_ADMIN_COMMAND_VIRTUALIZATION_MANAGEMENT= 0x1C,
    NVME_ADMIN_COMMAND_NVME_MI_SEND             = 0x1D,
    NVME_ADMIN_COMMAND_NVME_MI_RECEIVE          = 0x1E,

    NVME_ADMIN_COMMAND_DISCOVERY_INFO_MGMT      = 0x21,

    NVME_ADMIN_COMMAND_DOORBELL_BUFFER_CONFIG   = 0x7C,

    NVME_ADMIN_COMMAND_FABRICS                  = 0x7F,

    NVME_ADMIN_COMMAND_FORMAT_NVM               = 0x80,
    NVME_ADMIN_COMMAND_SECURITY_SEND            = 0x81,
    NVME_ADMIN_COMMAND_SECURITY_RECEIVE         = 0x82,
    NVME_ADMIN_COMMAND_SANITIZE                 = 0x84,
    NVME_ADMIN_COMMAND_GET_LBA_STATUS           = 0x86,

} NVME_ADMIN_COMMANDS;

//
// Features for Get/Set Features command
//
typedef enum {

    NVME_FEATURE_ARBITRATION                            = 0x01,
    NVME_FEATURE_POWER_MANAGEMENT                       = 0x02,
    NVME_FEATURE_LBA_RANGE_TYPE                         = 0x03,
    NVME_FEATURE_TEMPERATURE_THRESHOLD                  = 0x04,
    NVME_FEATURE_ERROR_RECOVERY                         = 0x05,
    NVME_FEATURE_VOLATILE_WRITE_CACHE                   = 0x06,
    NVME_FEATURE_NUMBER_OF_QUEUES                       = 0x07,
    NVME_FEATURE_INTERRUPT_COALESCING                   = 0x08,
    NVME_FEATURE_INTERRUPT_VECTOR_CONFIG                = 0x09,
    NVME_FEATURE_WRITE_ATOMICITY                        = 0x0A,
    NVME_FEATURE_ASYNC_EVENT_CONFIG                     = 0x0B,
    NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION      = 0x0C,
    NVME_FEATURE_HOST_MEMORY_BUFFER                     = 0x0D,
    NVME_FEATURE_TIMESTAMP                              = 0x0E,
    NVME_FEATURE_KEEP_ALIVE                             = 0x0F,
    NVME_FEATURE_HOST_CONTROLLED_THERMAL_MANAGEMENT     = 0x10,
    NVME_FEATURE_NONOPERATIONAL_POWER_STATE             = 0x11,
    NVME_FEATURE_READ_RECOVERY_LEVEL_CONFIG             = 0x12,
    NVME_FEATURE_PREDICTABLE_LATENCY_MODE_CONFIG        = 0x13,
    NVME_FEATURE_PREDICTABLE_LATENCY_MODE_WINDOW        = 0x14,
    NVME_FEATURE_LBA_STATUS_INFORMATION_REPORT_INTERVAL = 0x15,
    NVME_FEATURE_HOST_BEHAVIOR_SUPPORT                  = 0x16,
    NVME_FEATURE_SANITIZE_CONFIG                        = 0x17,
    NVME_FEATURE_ENDURANCE_GROUP_EVENT_CONFIG           = 0x18,
    NVME_FEATURE_IO_COMMAND_SET_PROFILE                 = 0x19,

    NVME_FEATURE_ENHANCED_CONTROLLER_METADATA           = 0x7D,
    NVME_FEATURE_CONTROLLER_METADATA                    = 0x7E,
    NVME_FEATURE_NAMESPACE_METADATA                     = 0x7F,

    NVME_FEATURE_NVM_SOFTWARE_PROGRESS_MARKER           = 0x80,
    NVME_FEATURE_NVM_HOST_IDENTIFIER                    = 0x81,
    NVME_FEATURE_NVM_RESERVATION_NOTIFICATION_MASK      = 0x82,
    NVME_FEATURE_NVM_RESERVATION_PERSISTANCE            = 0x83,
    NVME_FEATURE_NVM_NAMESPACE_WRITE_PROTECTION_CONFIG  = 0x84,

    NVME_FEATURE_ERROR_INJECTION                        = 0xC0, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY                = 0xC1, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_READONLY_WRITETHROUGH_MODE             = 0xC2, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS  = 0xC3, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_ENABLE_IEEE1667_SILO                   = 0xC4, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_LATENCY_MONITOR                        = 0xC5, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_PLP_HEALTH_CHECK_INTERVAL              = 0xC6, // This is from OCP NVMe Cloud SSD spec.
    NVME_FEATURE_DSSD_POWER_STATE                       = 0xC7, // This is from OCP NVMe Cloud SSD spec.

} NVME_FEATURES;


//
// Abort command: parameter
//
typedef union {

    struct {
        ULONG   SQID    : 8;        // Submission Queue Identifier (SQID)
        ULONG   CID     : 16;       // Command Identifier (CID)
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW10_ABORT, *PNVME_CDW10_ABORT;

//
// Identify Command of Controller or Namespace Structure (CNS)
//
typedef enum {

    NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE                    = 0x0,
    NVME_IDENTIFY_CNS_CONTROLLER                            = 0x1,
    NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES                     = 0x2,       // A list of up to 1024 active namespace IDs is returned to the host containing active namespaces with a namespace identifier greater than the value specified in the Namespace Identifier (CDW1.NSID) field.
    NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE                  = 0x3,
    NVME_IDENTIFY_CNS_NVM_SET                               = 0x4,

    NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE_IO_COMMAND_SET     = 0x5,
    NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET    = 0x6,
    NVME_IDENTIFY_CNS_ACTIVE_NAMESPACE_LIST_IO_COMMAND_SET  = 0x7,

    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_LIST              = 0x10,
    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE                   = 0x11,
    NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID               = 0x12,
    NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM      = 0x13,
    NVME_IDENTIFY_CNS_PRIMARY_CONTROLLER_CAPABILITIES       = 0x14,
    NVME_IDENTIFY_CNS_SECONDARY_CONTROLLER_LIST             = 0x15,
    NVME_IDENTIFY_CNS_NAMESPACE_GRANULARITY_LIST            = 0x16,
    NVME_IDENTIFY_CNS_UUID_LIST                             = 0x17,
    NVME_IDENTIFY_CNS_DOMAIN_LIST                           = 0x18,
    NVME_IDENTIFY_CNS_ENDURANCE_GROUP_LIST                  = 0x19,

    NVME_IDENTIFY_CNS_ALLOCATED_NAMSPACE_LIST_IO_COMMAND_SET= 0x1A,
    NVME_IDENTIFY_CNS_ALLOCATED_NAMESPACE_IO_COMMAND_SET    = 0x1B,
    NVME_IDENTIFY_CNS_IO_COMMAND_SET                        = 0x1C,

} NVME_IDENTIFY_CNS_CODES;

//
// Identify Command Set Identifiers (CSI)
//
typedef enum {
    NVME_COMMAND_SET_NVM                                = 0x0,
    NVME_COMMAND_SET_KEY_VALUE                          = 0x1,
    NVME_COMMAND_SET_ZONED_NAMESPACE                    = 0x2,
} NVME_COMMAND_SET_IDENTIFIERS;

typedef union {

    struct {
        ULONG   CNS         : 8;         // Controller or Namespace Structure (CNS, Defined in NVME_IDENTIFY_CNS_CODES)
        ULONG   Reserved    : 8;
        ULONG   CNTID       : 16;        // Controller Identifier (CNTID)
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW10_IDENTIFY, *PNVME_CDW10_IDENTIFY;

typedef union {

    struct {
        USHORT  NVMSETID;               // NVM Set Identifier
        USHORT  Reserved;
    } DUMMYSTRUCTNAME;

    struct {
        ULONG   CNSID       : 16;       // CNS Specific Identifier (NVM Set ID/Domain ID/Endurance Group ID)
        ULONG   Reserved2   : 8;
        ULONG   CSI         : 8;        // Command Set Identifier (CSI, Defined in NVME_COMMAND_SET_IDENTIFIERS)
    } DUMMYSTRUCTNAME2;

    ULONG AsUlong;

} NVME_CDW11_IDENTIFY, *PNVME_CDW11_IDENTIFY;

typedef union {

    struct {
        ULONG   UUIDIndex   : 7;        // Bits 06:00
        ULONG   Reserved    : 25;       // Bits 31:07
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW14_IDENTIFY, *PNVME_CDW14_IDENTIFY;

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE (0x0)
//

typedef enum {
    NVME_READ_BEHAVIOR_NOT_REPORTED     = 0x0,
    NVME_READ_BEHAVIOR_RETURN_ZERO      = 0x1,
    NVME_READ_BEHAVIOR_RETURN_ONES      = 0x2
} NVME_DEALLOCATE_READ_BEHAVIOR;

typedef union {

    struct {
        USHORT  MS;                 // bit 0:15     Metadata Size (MS)
        UCHAR   LBADS;              // bit 16:23    LBA  Data  Size (LBADS)

        UCHAR   RP          : 2;    // bit 24:25    Relative Performance (RP)
        UCHAR   Reserved0   : 6;    // bit 26:31
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_LBA_FORMAT, *PNVME_LBA_FORMAT;

typedef union {

    struct {
        UCHAR   PersistThroughPowerLoss                     : 1;
        UCHAR   WriteExclusiveReservation                   : 1;
        UCHAR   ExclusiveAccessReservation                  : 1;
        UCHAR   WriteExclusiveRegistrantsOnlyReservation    : 1;
        UCHAR   ExclusiveAccessRegistrantsOnlyReservation   : 1;
        UCHAR   WriteExclusiveAllRegistrantsReservation     : 1;
        UCHAR   ExclusiveAccessAllRegistrantsReservation    : 1;
        UCHAR   Reserved                                    : 1;
    } DUMMYSTRUCTNAME;

    UCHAR AsUchar;
} NVM_RESERVATION_CAPABILITIES, *PNVME_RESERVATION_CAPABILITIES;

typedef struct {

    ULONGLONG   NSZE;                   // byte 0:7.    M - Namespace Size (NSZE)
    ULONGLONG   NCAP;                   // byte 8:15    M - Namespace Capacity (NCAP)
    ULONGLONG   NUSE;                   // byte 16:23   M - Namespace Utilization (NUSE)

    struct {
        UCHAR   ThinProvisioning            : 1;
        UCHAR   NameSpaceAtomicWriteUnit    : 1;
        UCHAR   DeallocatedOrUnwrittenError : 1;
        UCHAR   SkipReuseUI                 : 1;
        UCHAR   NameSpaceIoOptimization     : 1;
        UCHAR   Reserved                    : 3;
    } NSFEAT;                           // byte 24      M - Namespace Features (NSFEAT)

    UCHAR   NLBAF;                      // byte 25      M - Number of LBA Formats (NLBAF)

    struct {
        UCHAR   LbaFormatIndex              : 4;
        UCHAR   MetadataInExtendedDataLBA   : 1;
        UCHAR   LbaFormatIndexMS            : 2;
        UCHAR   Reserved                    : 1;
    } FLBAS;                            // byte 26      M - Formatted LBA Size (FLBAS)

    struct {
        UCHAR   MetadataInExtendedDataLBA   : 1;
        UCHAR   MetadataInSeparateBuffer    : 1;
        UCHAR   Reserved                    : 6;
    } MC;                               // byte 27      M - Metadata Capabilities (MC)

    struct {
        UCHAR   ProtectionInfoType1         : 1;
        UCHAR   ProtectionInfoType2         : 1;
        UCHAR   ProtectionInfoType3         : 1;
        UCHAR   InfoAtBeginningOfMetadata   : 1;
        UCHAR   InfoAtEndOfMetadata         : 1;
        UCHAR   Reserved                    : 3;
    } DPC;                              // byte 28      M - End-to-end Data Protection Capabilities (DPC)

    struct {
        UCHAR   ProtectionInfoTypeEnabled   : 3;    // 0 - not enabled; 1 ~ 3: enabled type; 4 ~ 7: reserved
        UCHAR   InfoAtBeginningOfMetadata   : 1;
        UCHAR   Reserved                    : 4;
    } DPS;                              // byte 29      M - End-to-end Data Protection Type Settings (DPS)

    struct {
        UCHAR   SharedNameSpace     : 1;
        UCHAR   Reserved            : 7;
    } NMIC;                             // byte 30      O - Namespace Multi-path I/O and Namespace Sharing Capabilities (NMIC)

    NVM_RESERVATION_CAPABILITIES RESCAP;    // byte 31      O - Reservation Capabilities (RESCAP)

    struct {
        UCHAR   PercentageRemained  : 7;// Bits 6:0: indicate the percentage of the namespace that remains to be formatted
        UCHAR   Supported           : 1;// Bit 7: if set to 1 indicates that the namespace supports the Format Progress Indicator.
    } FPI;                              // byte 32      O - Format Progress Indicator (FPI)

    struct {
        UCHAR  ReadBehavior         : 3;// Bits 2:0: indicate deallocated logical block read behavior (NVME_DEALLOCATE_READ_BEHAVIOR)
        UCHAR  WriteZeroes          : 1;// Bit 3: indicate controller supports the deallocate bit in Write Zeroes
        UCHAR  GuardFieldWithCRC    : 1;// Bit 4: indicate guard field for deallocated logical blocks is set to CRC
        UCHAR  Reserved             : 3;
    } DLFEAT;                                  // byte 33

    USHORT          NAWUN;              // byte 34:35 O - Namespace Atomic Write Unit Normal (NAWUN)
    USHORT          NAWUPF;             // byte 36:37 O - Namespace Atomic Write Unit Power Fail (NAWUPF)
    USHORT          NACWU;              // byte 38:39 O - Namespace Atomic Compare & Write Unit (NACWU)
    USHORT          NABSN;              // byte 40:41 O - Namespace Atomic Boundary Size Normal (NABSN)
    USHORT          NABO;               // byte 42:43 O - Namespace Atomic Boundary Offset (NABO)
    USHORT          NABSPF;             // byte 44:45 O - Namespace Atomic Boundary Size Power Fail (NABSPF)
    USHORT          NOIOB;              // byte 46:47 O - Namespace Optimal IO Boundary (NOIOB)

    UCHAR           NVMCAP[16];         // byte 48:63 O - NVM Capacity (NVMCAP)

    USHORT          NPWG;               // byte 64:65 O - Namespace Preferred Write Granularity (NPWG)
    USHORT          NPWA;               // byte 66:67 O - Namespace Preferred Write Alignment (NPWA)
    USHORT          NPDG;               // byte 68:69 O - Namespace Preferred Deallocate Granularity (NPDG)
    USHORT          NPDA;               // byte 70:71 O - Namespace Preferred Deallocate Alignment (NPDA)
    USHORT          NOWS;               // byte 72:73 O - Namespace Optimal Write Size (NOWS)

    USHORT          MSSRL;              // byte 74:75 O - Maximum Single Source Range Length(MSSRL)
    ULONG           MCL;                // byte 76:79 O - Maximum Copy Length(MCL)
    UCHAR           MSRC;               // byte 80 O - Maximum Source Range Count(MSRC)
    UCHAR           Reserved2[11];      // byte 81:91

    ULONG           ANAGRPID;           // byte 92:95 O - ANA Group Identifier (ANAGRPID)

    UCHAR           Reserved3[3];       // byte 96:98

    struct {
        UCHAR   WriteProtected      : 1;// Write Protected
        UCHAR   Reserved            : 7;
    } NSATTR;                           // byte 99 O - Namespace Attributes

    USHORT          NVMSETID;           // byte 100:101 O - Associated NVM Set Identifier: 1-based value specifying NVM Set of this namespace.

    USHORT          ENDGID;             // byte 102:103 O - Associated Endurance Group Identier

    UCHAR           NGUID[16];          // byte 104:119 O - Namespace Globally Unique Identifier (NGUID)

    UCHAR           EUI64[8];           // byte 120:127 M - IEEE Extended Unique Identifier (EUI64)

    NVME_LBA_FORMAT LBAF[64];           // byte 128:131 M - LBA Format 0 Support (LBAF0)
                                        // byte 132:135 O - LBA Format 1 Support (LBAF1)
                                        // byte 136:139 O - LBA Format 2 Support (LBAF2)
                                        // byte 140:143 O - LBA Format 3 Support (LBAF3)
                                        // ...
                                        // byte 380:383 O - LBA Format 63 Support (LBAF63)

    UCHAR           VS[3712];           // byte 384:4095 O - Vendor Specific (VS): This range of bytes is allocated for vendor specific usage.

} NVME_IDENTIFY_NAMESPACE_DATA, *PNVME_IDENTIFY_NAMESPACE_DATA;

//
// Output of NVME_IDENTIFY_CNS_CONTROLLER (0x01)
//

//
// Discovery Controller Type
//
typedef enum _NVME_DISC_CTRL_TYPE {

    NvmeDiscCtrlTypeUnspecified = 0,
    NvmeDiscCtrlTypeDDC         = 1,
    NvmeDiscCtrlTypeCDC         = 2,
    NvmeDiscCtrlTypeReserved1   = 3,
    NvmeDiscCtrlTypeReservedMax = 255,

} NVME_DISC_CTRL_TYPE;


typedef struct {
    USHORT  MP;                 // bit 0:15.    Maximum  Power (MP)

    UCHAR   Reserved0;          // bit 16:23

    UCHAR   MPS         : 1;    // bit 24: Max Power Scale (MPS)
    UCHAR   NOPS        : 1;    // bit 25: Non-Operational State (NOPS)
    UCHAR   Reserved1   : 6;    // bit 26:31

    ULONG   ENLAT;              // bit 32:63.   Entry Latency (ENLAT)
    ULONG   EXLAT;              // bit 64:95.   Exit Latency (EXLAT)

    UCHAR   RRT         : 5;    // bit 96:100.  Relative Read Throughput (RRT)
    UCHAR   Reserved2   : 3;    // bit 101:103

    UCHAR   RRL         : 5;    // bit 104:108  Relative Read Latency (RRL)
    UCHAR   Reserved3   : 3;    // bit 109:111

    UCHAR   RWT         : 5;    // bit 112:116  Relative Write Throughput (RWT)
    UCHAR   Reserved4   : 3;    // bit 117:119

    UCHAR   RWL         : 5;    // bit 120:124  Relative Write Latency (RWL)
    UCHAR   Reserved5   : 3;    // bit 125:127

    USHORT  IDLP;               // bit 128:143  Idle Power (IDLP)

    UCHAR   Reserved6   : 6;    // bit 144:149
    UCHAR   IPS         : 2;    // bit 150:151  Idle Power Scale (IPS)

    UCHAR   Reserved7;          // bit 152:159

    USHORT  ACTP;               // bit 160:175  Active Power (ACTP)

    UCHAR   APW         : 3;    // bit 176:178  Active Power Workload (APW)
    UCHAR   Reserved8   : 3;    // bit 179:181
    UCHAR   APS         : 2;    // bit 182:183  Active Power Scale (APS)


    UCHAR   Reserved9[9];       // bit 184:255.

} NVME_POWER_STATE_DESC, *PNVME_POWER_STATE_DESC;

typedef struct {
    //
    // byte 0 : 255, Controller Capabilities and Features
    //
    USHORT  VID;                // byte 0:1.    M - PCI Vendor ID (VID)
    USHORT  SSVID;              // byte 2:3.    M - PCI Subsystem Vendor ID (SSVID)
    UCHAR   SN[20];             // byte 4: 23.  M - Serial Number (SN)
    UCHAR   MN[40];             // byte 24:63.  M - Model Number (MN)
    UCHAR   FR[8];              // byte 64:71.  M - Firmware Revision (FR)
    UCHAR   RAB;                // byte 72.     M - Recommended Arbitration Burst (RAB)
    UCHAR   IEEE[3];            // byte 73:75.  M - IEEE OUI Identifier (IEEE). Controller Vendor code.

    struct {
        UCHAR   MultiPorts          : 1;
        UCHAR   MultiControllers    : 1;
        UCHAR   SRIOV               : 1;
        UCHAR   ANAR                : 1;
        UCHAR   Reserved            : 4;
    } CMIC;                     // byte 76.     O - Controller Multi-Path I/O and Namespace Sharing Capabilities (CMIC)

    UCHAR   MDTS;               // byte 77.     M - Maximum Data Transfer Size (MDTS)
    USHORT  CNTLID;             // byte 78:79.   M - Controller ID (CNTLID)
    ULONG   VER;                // byte 80:83.   M - Version (VER)
    ULONG   RTD3R;              // byte 84:87.   M - RTD3 Resume Latency (RTD3R)
    ULONG   RTD3E;              // byte 88:91.   M - RTD3 Entry Latency (RTD3E)

    struct {
        ULONG   Reserved0                   : 8;
        ULONG   NamespaceAttributeChanged   : 1;
        ULONG   FirmwareActivation          : 1;
        ULONG   Reserved1                   : 1;
        ULONG   AsymmetricAccessChanged     : 1;
        ULONG   PredictableLatencyAggregateLogChanged   : 1;
        ULONG   LbaStatusChanged            : 1;
        ULONG   EnduranceGroupAggregateLogChanged       : 1;
        ULONG   NormalNvmSubsystemShutdown  : 1;
        ULONG   Reserved2                   : 11;
        ULONG   ZoneInformation             : 1;
        ULONG   Reserved3                   : 3;
        ULONG   DiscoveryLogChanged         : 1;
    } OAES;                     // byte 92:95.   M - Optional Asynchronous Events Supported (OAES)

   struct {
        ULONG   HostIdentifier128Bit        : 1;
        ULONG   NOPSPMode                   : 1;
        ULONG   NVMSets                     : 1;
        ULONG   ReadRecoveryLevels          : 1;
        ULONG   EnduranceGroups             : 1;
        ULONG   PredictableLatencyMode      : 1;
        ULONG   TBKAS                       : 1;    // Traffic Based Keep Alive Support
        ULONG   NamespaceGranularity        : 1;
        ULONG   SQAssociations              : 1;
        ULONG   UUIDList                    : 1;
        ULONG   MultiDomainSubsystem        : 1;
        ULONG   FixedCapacityManagement     : 1;
        ULONG   VariableCapacityManagement  : 1;
        ULONG   DeleteEnduranceGroup        : 1;
        ULONG   DeleteNVMSet                : 1;
        ULONG   ELBAS                       : 1;    // Extended LBA Formats Supported
        ULONG   Reserved0                   : 16;
    } CTRATT;                   // byte 96:99.   M - Controller Attributes (CTRATT)

    struct {
        USHORT  ReadRecoveryLevel0          : 1;
        USHORT  ReadRecoveryLevel1          : 1;
        USHORT  ReadRecoveryLevel2          : 1;
        USHORT  ReadRecoveryLevel3          : 1;
        USHORT  ReadRecoveryLevel4          : 1;
        USHORT  ReadRecoveryLevel5          : 1;
        USHORT  ReadRecoveryLevel6          : 1;
        USHORT  ReadRecoveryLevel7          : 1;
        USHORT  ReadRecoveryLevel8          : 1;
        USHORT  ReadRecoveryLevel9          : 1;
        USHORT  ReadRecoveryLevel10         : 1;
        USHORT  ReadRecoveryLevel11         : 1;
        USHORT  ReadRecoveryLevel12         : 1;
        USHORT  ReadRecoveryLevel13         : 1;
        USHORT  ReadRecoveryLevel14         : 1;
        USHORT  ReadRecoveryLevel15         : 1;
    } RRLS;                     // byte 100:101. O - Read Recovery Levels Supported (RRLS)

    UCHAR   Reserved0[9];       // byte 102:110.

    UCHAR   CNTRLTYPE;          // byte 111.     M - Controller Type
    UCHAR   FGUID[16];          // byte 112:127. O - FRU Globally Unique Identifier (FGUID)

    USHORT  CRDT1;              // byte 128:129. O - Command Retry Delay Time 1
    USHORT  CRDT2;              // byte 130:131. O - Command Retry Delay Time 1
    USHORT  CRDT3;              // byte 132:133. O - Command Retry Delay Time 1

    UCHAR   Reserved1[106];      // byte 134:239.
    UCHAR   ReservedForManagement[13];     // byte 240:252.  Refer to the NVMe Management Interface Specification for definition.
    UCHAR   NVMSR;              // byte 253. M - NVM Subsystem Report
    UCHAR   VWCI;               // byte 254. M - VPD Write Cycle Information
    UCHAR   MEC;                // byte 255. M - Management Endpoint Capabilities

    //
    // byte 256 : 511, Admin Command Set Attributes
    //
    struct {
        USHORT  SecurityCommands       : 1;
        USHORT  FormatNVM              : 1;
        USHORT  FirmwareCommands       : 1;
        USHORT  NamespaceCommands      : 1;
        USHORT  DeviceSelfTest         : 1;
        USHORT  Directives             : 1;
        USHORT  NVMeMICommands         : 1;
        USHORT  VirtualizationMgmt     : 1;
        USHORT  DoorBellBufferConfig   : 1;
        USHORT  GetLBAStatus           : 1;
        USHORT  CommandFeatureLockdown : 1;
        USHORT  Reserved               : 5;
    } OACS;                     // byte 256:257. M - Optional Admin Command Support (OACS)

    UCHAR   ACL;                // byte 258.    M - Abort Command Limit (ACL)
    UCHAR   AERL;               // byte 259.    M - Asynchronous Event Request Limit (AERL)

    struct {
        UCHAR   Slot1ReadOnly   : 1;
        UCHAR   SlotCount       : 3;
        UCHAR   ActivationWithoutReset  : 1;
        UCHAR   Reserved        : 3;
    } FRMW;                     // byte 260.    M - Firmware Updates (FRMW)

    struct {
        UCHAR   SmartPagePerNamespace   : 1;
        UCHAR   CommandEffectsLog       : 1;
        UCHAR   LogPageExtendedData     : 1;
        UCHAR   TelemetrySupport        : 1;
        UCHAR   PersistentEventLog      : 1;
        UCHAR   SupportedLogPages       : 1;
        UCHAR   TelemetryDataArea4      : 1;
        UCHAR   Reserved1               : 1;
    } LPA;                      // byte 261.    M - Log Page Attributes (LPA)

    UCHAR   ELPE;               // byte 262.    M - Error Log Page Entries (ELPE)
    UCHAR   NPSS;               // byte 263.    M - Number of Power States Support (NPSS)

    struct {
        UCHAR   CommandFormatInSpec : 1;
        UCHAR   Reserved            : 7;
    } AVSCC;                    // byte 264.    M - Admin Vendor Specific Command Configuration (AVSCC)

    struct {
        UCHAR   Supported       : 1;
        UCHAR   Reserved        : 7;
    } APSTA;                    // byte 265.     O - Autonomous Power State Transition Attributes (APSTA)

    USHORT  WCTEMP;             // byte 266:267. M - Warning Composite Temperature Threshold (WCTEMP)
    USHORT  CCTEMP;             // byte 268:269. M - Critical Composite Temperature Threshold (CCTEMP)
    USHORT  MTFA;               // byte 270:271. O - Maximum Time for Firmware Activation (MTFA)
    ULONG   HMPRE;              // byte 272:275. O - Host Memory Buffer Preferred Size (HMPRE)
    ULONG   HMMIN;              // byte 276:279. O - Host Memory Buffer Minimum Size (HMMIN)
    UCHAR   TNVMCAP[16];        // byte 280:295. O - Total NVM Capacity (TNVMCAP)
    UCHAR   UNVMCAP[16];        // byte 296:311. O - Unallocated NVM Capacity (UNVMCAP)

    struct {
        ULONG   RPMBUnitCount           : 3;    // Number of RPMB Units
        ULONG   AuthenticationMethod    : 3;    // Authentication Method
        ULONG   Reserved0               : 10;
        ULONG   TotalSize               : 8;    // Total Size: in 128KB units.
        ULONG   AccessSize              : 8;    // Access Size: in 512B units.
    } RPMBS;                    // byte 312:315. O - Replay Protected Memory Block Support (RPMBS)

    USHORT  EDSTT;              // byte 316:317. O - Extended Device Self-test Time (EDSTT)
    UCHAR   DSTO;               // byte 318.     O - Device Self-test Options (DSTO)
    UCHAR   FWUG;               // byte 319.     M - Firmware Update Granularity (FWUG)
    USHORT  KAS;                // byte 320:321  M - Keep Alive Support (KAS)

    struct {
        USHORT  Supported       : 1;
        USHORT  Reserved        : 15;
    } HCTMA;                    // byte 322:323  O - Host Controlled Thermal Management Attributes (HCTMA)

    USHORT  MNTMT;              // byte 324:325  O - Minimum Thermal Management Temperature (MNTMT)
    USHORT  MXTMT;              // byte 326:327  O - Maximum Thermal Management Temperature (MXTMT)

    struct {
        ULONG   CryptoErase             : 1;     // Controller supports Crypto Erase Sanitize
        ULONG   BlockErase              : 1;     // Controller supports Block Erase Sanitize
        ULONG   Overwrite               : 1;     // Controller supports Overwrite Santize
        ULONG   Reserved                : 26;
        ULONG   NDI                     : 1;     // No-Deallocate Inhibited (NDI)
        ULONG   NODMMAS                 : 2;     // No-Deallocate Modifies Media After Sanitize (NODMMAS)
    } SANICAP;                  // byte 328:331  O - Sanitize Capabilities (SANICAP)

    ULONG   HMMINDS;            // byte 332:335  O - Host Memory Buffer Minimum Descriptor Entry Size (HMMINDS)
    USHORT  HMMAXD;             // byte 336:337  O - Host Memory Maxiumum Descriptors Entries (HMMAXD)

    USHORT  NSETIDMAX;          // byte 338:339  O - NVM Set Identifier Maximum: 1-based maximum value of a valid NVM Set Identifier.
    USHORT  ENDGIDMAX;          // byte 340:341  O - Endurance Group Identifier Maximum (ENDGIDMAX)

    UCHAR   ANATT;              // byte 342      O - ANA Transition Time (ANATT)

    struct {
        UCHAR   OptimizedState          : 1;     // Report ANA Optimized State
        UCHAR   NonOptimizedState       : 1;     // Report ANA Non-Optimized State
        UCHAR   InaccessibleState       : 1;     // Report ANA Inaccessible State
        UCHAR   PersistentLossState     : 1;     // Report ANA Persistent Loss State
        UCHAR   ChangeState             : 1;     // Report ANA Change State
        UCHAR   Reserved                : 1;
        UCHAR   StaticANAGRPID          : 1;     // If set, ANAGRPID in Identify Namespace doesn't change
        UCHAR   SupportNonZeroANAGRPID  : 1;     // If set, Controller supports a non-zero value in ANAGRPID field of Namespace Mgmt Command
    } ANACAP;                   // byte 343      O - Asymmetric Namespace Access Capabilities (ANACAP)

    ULONG  ANAGRPMAX;           // byte 344:347  O - ANA Group Identifier Maximum (ANAGRPMAX)
    ULONG  NANAGRPID;           // byte 348:351  O - Number of ANA Group Identifiers (NANAGRPID)

    ULONG   PELS;               // byte 352:355  O - Persistent Event Log Size (PELS)

    USHORT  DomainId;           // byte 356:357  O - Domain Identifier

    UCHAR   Reserved2[10];      // byte 358:367

    UCHAR   MEGCAP[16];         // byte 368:383  O - Max Endurance Group Capacity

    UCHAR   TMPTHHA;            // byte 384      O - Temperature Threshold Hysteresis Attributes

    UCHAR   Reserved3;          // byte 385

    USHORT  CQT;                // byte 386:387  M - Command Quiesce Time (milliseconds)

    UCHAR   Reserved4[124];     // byte 388:511

    //
    // byte 512 : 703, NVM Command Set Attributes
    //
    struct {
        UCHAR   RequiredEntrySize   : 4;    // The value is in bytes and is reported as a power of two (2^n).
        UCHAR   MaxEntrySize        : 4;    // This value is larger than or equal to the required SQ entry size.  The value is in bytes and is reported as a power of two (2^n).
    } SQES;                     // byte 512.    M - Submission Queue Entry Size (SQES)

    struct {
        UCHAR   RequiredEntrySize   : 4;    // The value is in bytes and is reported as a power of two (2^n).
        UCHAR   MaxEntrySize        : 4;    // This value is larger than or equal to the required CQ entry size. The value is in bytes and is reported as a power of two (2^n).
    } CQES;                     // byte 513.    M - Completion Queue Entry Size (CQES)

    USHORT  MAXCMD;             // byte 514:515. M - Maximum Outstanding Commands (MAXCMD)

    ULONG   NN;                 // byte 516:519. M - Number of Namespaces (NN)

    struct {
        USHORT  Compare             : 1;
        USHORT  WriteUncorrectable  : 1;
        USHORT  DatasetManagement   : 1;
        USHORT  WriteZeroes         : 1;
        USHORT  FeatureField        : 1;
        USHORT  Reservations        : 1;
        USHORT  Timestamp           : 1;
        USHORT  Verify              : 1;

        USHORT  Reserved            : 8;
    } ONCS;                     // byte 520:521. M - Optional NVM Command Support (ONCS)

    struct {
        USHORT  CompareAndWrite             : 1;
        USHORT  Reserved                    : 15;
    } FUSES;                    // byte 522:523. M - Fused Operation Support (FUSES)

    struct {
        UCHAR   FormatApplyToAll                : 1;
        UCHAR   SecureEraseApplyToAll           : 1;
        UCHAR   CryptographicEraseSupported     : 1;
        UCHAR   FormatSupportNSIDAllF           : 1;
        UCHAR   Reserved                        : 4;
    } FNA;                      // byte 524.     M - Format NVM Attributes (FNA)

    struct {
        UCHAR   Present         : 1;
        UCHAR   FlushBehavior   : 2;
        UCHAR   Reserved        : 5;
    } VWC;                      // byte 525.     M - Volatile Write Cache (VWC)

    USHORT  AWUN;               // byte 526:527. M - Atomic Write Unit Normal (AWUN)
    USHORT  AWUPF;              // byte 528:529. M - Atomic Write Unit Power Fail (AWUPF)

    struct {
        UCHAR   CommandFormatInSpec : 1;
        UCHAR   Reserved            : 7;
    } NVSCC;                    // byte 530.     M - NVM Vendor Specific Command Configuration (NVSCC)

    struct {
        UCHAR   WriteProtect        : 1;
        UCHAR   UntilPowerCycle     : 1;
        UCHAR   Permanent           : 1;
        UCHAR   Reserved            : 5;
    } NWPC;                     // byte 531.     M - Namespace Write Protection Capabilities (NWPC)

    USHORT  ACWU;               // byte 532:533  O - Atomic Compare & Write Unit (ACWU)

    USHORT  CopyDescFormats;    // byte 534:535  M - Copy Descriptor Formats Supported

    struct {
        ULONG   SGLSupported            : 2;
        ULONG   KeyedSGLData            : 1;
        ULONG   Reserved0               : 13;
        ULONG   BitBucketDescrSupported : 1;
        ULONG   ByteAlignedContiguousPhysicalBuffer : 1;
        ULONG   SGLLengthLargerThanDataLength       : 1;
        ULONG   MPTRSGLDescriptor       : 1;
        ULONG   AddressFieldSGLDataBlock: 1;
        ULONG   TransportSGLData        : 1;
        ULONG   Reserved1               : 10;
    } SGLS;                     // byte 536:539. O - SGL Support (SGLS)

    ULONG   MNAN;               // byte 540:543. O - Maximum Number of Allowed Namespace (MNAN)

    UCHAR   MAXDNA[16];         // byte 544:559. O - Maximum Domain Namespace Attachments (MAXDNA)

    ULONG   MAXCNA;             // byte 560:563. O - Maximum I/O Controller Namespace Attachments (MAXCNA)

    UCHAR   Reserved6[204];     // byte 564:767.

    UCHAR   SUBNQN[256];        // byte 768:1023. M - NVM Subsystem NVMe Qualified Name (SUBNQN)

    UCHAR   Reserved7[768];     // byte 1024:1791

    //
    // byte 1792:2047. Fabric specific
    //
    ULONG   IOCCSZ;             // byte 1792:1795. M - I/O Queue Command Capsule Supported Size (IOCCSZ)

    ULONG   IORCSZ;             // byte 1796:1799. M - I/O Queue Response Capsule Supported Size (IORCSZ)

    USHORT  ICDOFF;             // byte 1800:1801. M - In Capsule Data Offset (ICDOFF)

    struct {
        UCHAR   StaticControllerModel : 1;
        UCHAR   Reserved              : 7;
    } FCATT;                    // byte 1802. M - Fabrics Controller Attributes (FCATT)

    UCHAR  MSDBD;               // byte 1803. M - Maximum SGL Data Block Descriptors (MSDBD)

    struct {
        USHORT   IOQueueDeletion : 1;
        USHORT   Reserved        : 15;
    } OFCS;                    // byte 1804:1805. M - Optional Fabric Commands Support (OFCS)

    UCHAR DCTYPE;              // byte 1806. Discovery Controller Type (NVME_DISC_CTRL_TYPE)

    UCHAR   Reserved8[241];    // byte 1807:2047

    //
    // byte 2048 : 3071, Power State Descriptors
    //
    NVME_POWER_STATE_DESC   PDS[32];    // byte 2048:2079. M - Power State 0 Descriptor (PSD0):  This field indicates the characteristics of power state 0
                                        // byte 2080:2111. O - Power State 1 Descriptor (PSD1):  This field indicates the characteristics of power state 1
                                        // byte 2112:2143. O - Power State 2 Descriptor (PSD1):  This field indicates the characteristics of power state 2
                                        // byte 2144:2175. O - Power State 3 Descriptor (PSD1):  This field indicates the characteristics of power state 3
                                        // byte 2176:2207. O - Power State 4 Descriptor (PSD1):  This field indicates the characteristics of power state 4
                                        // byte 2208:2239. O - Power State 5 Descriptor (PSD1):  This field indicates the characteristics of power state 5
                                        // byte 2240:2271. O - Power State 6 Descriptor (PSD1):  This field indicates the characteristics of power state 6
                                        // byte 2272:2303. O - Power State 7 Descriptor (PSD1):  This field indicates the characteristics of power state 7
                                        // byte 2304:2335. O - Power State 8 Descriptor (PSD1):  This field indicates the characteristics of power state 8
                                        // byte 2336:2367. O - Power State 9 Descriptor (PSD1):  This field indicates the characteristics of power state 9
                                        // byte 2368:2399. O - Power State 10 Descriptor (PSD1):  This field indicates the characteristics of power state 10
                                        // byte 2400:2431. O - Power State 11 Descriptor (PSD1):  This field indicates the characteristics of power state 11
                                        // byte 2432:2463. O - Power State 12 Descriptor (PSD1):  This field indicates the characteristics of power state 12
                                        // byte 2464:2495. O - Power State 13 Descriptor (PSD1):  This field indicates the characteristics of power state 13
                                        // byte 2496:2527. O - Power State 14 Descriptor (PSD1):  This field indicates the characteristics of power state 14
                                        // byte 2528:2559. O - Power State 15 Descriptor (PSD1):  This field indicates the characteristics of power state 15
                                        // byte 2560:2591. O - Power State 16 Descriptor (PSD1):  This field indicates the characteristics of power state 16
                                        // byte 2592:2623. O - Power State 17 Descriptor (PSD1):  This field indicates the characteristics of power state 17
                                        // byte 2624:2655. O - Power State 18 Descriptor (PSD1):  This field indicates the characteristics of power state 18
                                        // byte 2656:2687. O - Power State 19 Descriptor (PSD1):  This field indicates the characteristics of power state 19
                                        // byte 2688:2719. O - Power State 20 Descriptor (PSD1):  This field indicates the characteristics of power state 20
                                        // byte 2720:2751. O - Power State 21 Descriptor (PSD1):  This field indicates the characteristics of power state 21
                                        // byte 2752:2783. O - Power State 22 Descriptor (PSD1):  This field indicates the characteristics of power state 22
                                        // byte 2784:2815. O - Power State 23 Descriptor (PSD1):  This field indicates the characteristics of power state 23
                                        // byte 2816:2847. O - Power State 24 Descriptor (PSD1):  This field indicates the characteristics of power state 24
                                        // byte 2848:2879. O - Power State 25 Descriptor (PSD1):  This field indicates the characteristics of power state 25
                                        // byte 2880:2911. O - Power State 26 Descriptor (PSD1):  This field indicates the characteristics of power state 26
                                        // byte 2912:2943. O - Power State 27 Descriptor (PSD1):  This field indicates the characteristics of power state 27
                                        // byte 2944:2975. O - Power State 28 Descriptor (PSD1):  This field indicates the characteristics of power state 28
                                        // byte 2976:3007. O - Power State 29 Descriptor (PSD1):  This field indicates the characteristics of power state 29
                                        // byte 3008:3039. O - Power State 30 Descriptor (PSD1):  This field indicates the characteristics of power state 30
                                        // byte 3040:3071. O - Power State 31 Descriptor (PSD1):  This field indicates the characteristics of power state 31

    //
    // byte 3072 : 4095, Vendor Specific
    //
    UCHAR   VS[1024];           // byte 3072 : 4095.

} NVME_IDENTIFY_CONTROLLER_DATA, *PNVME_IDENTIFY_CONTROLLER_DATA;

//
// Namespace Identfier Type (NIDT)
//
typedef enum {
    NVME_IDENTIFIER_TYPE_EUI64      = 0x1,
    NVME_IDENTIFIER_TYPE_NGUID      = 0x2,
    NVME_IDENTIFIER_TYPE_UUID       = 0x3,
    NVME_IDENTIFIER_TYPE_CSI        = 0x4,

} NVME_IDENTIFIER_TYPE;

//
// Namespace Identfier Length (NIDL) for a given type defined by NVME_IDENTIFIER_TYPE
//
typedef enum {
    NVME_IDENTIFIER_TYPE_EUI64_LENGTH      = 0x8,
    NVME_IDENTIFIER_TYPE_NGUID_LENGTH      = 0x10,
    NVME_IDENTIFIER_TYPE_UUID_LENGTH       = 0x10,
    NVME_IDENTIFIER_TYPE_CSI_LENGTH        = 0x1,

} NVME_IDENTIFIER_TYPE_LENGTH;

//
// Output of NVME_IDENTIFY_CNS_ACTIVE_NAMESPACES (0x02)
//

typedef struct {

    ULONG   NSID[1024];  // List of Namespace ID upto 1024 entries

} NVME_ACTIVE_NAMESPACE_ID_LIST, *PNVME_ACTIVE_NAMESPACE_ID_LIST;

//
// Output of NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE (0x03)
//

#define NVME_IDENTIFY_CNS_DESCRIPTOR_NAMESPACE_SIZE 0x1000

typedef struct {
    UCHAR           NIDT;                           // Namespace Identifier Type as defined in NVME_IDENTIFIER_TYPE
    UCHAR           NIDL;                           // Namespace Identifier Length
    UCHAR           Reserved[2];
    UCHAR           NID[ANYSIZE_ARRAY];             // Namespace Identifier (Based on NVME_IDENTIFIER_TYPE)
} NVME_IDENTIFY_NAMESPACE_DESCRIPTOR, *PNVME_IDENTIFY_NAMESPACE_DESCRIPTOR;

//
// Output of NVME_IDENTIFY_CNS_NVM_SET (0x04)
//
typedef struct {

    USHORT      Identifier;
    USHORT      ENDGID;

    ULONG       Reserved1;

    ULONG       Random4KBReadTypical;
    ULONG       OptimalWriteSize;
    UCHAR       TotalCapacity[16];
    UCHAR       UnallocatedCapacity[16];

    UCHAR       Reserved2[80];
} NVME_SET_ATTRIBUTES_ENTRY, *PNVME_SET_ATTRIBUTES_ENTRY;

typedef struct {

    UCHAR       IdentifierCount;

    UCHAR       Reserved[127];

    NVME_SET_ATTRIBUTES_ENTRY       Entry[ANYSIZE_ARRAY];

} NVM_SET_LIST, *PNVM_SET_LIST;

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_NAMESPACE_IO_COMMAND_SET (0x05)
//
typedef struct {

    ULONGLONG   ZoneSize;           // bit 0:63     Zone Size (MS)
    UCHAR       ZDES;               // bit 64:71    Zone Descriptor Extension Size (ZDES)
    UCHAR       Reserved[7];

} NVME_LBA_ZONE_FORMAT, *PNVME_LBA_ZONE_FORMAT;

typedef struct {

    struct {
        USHORT  VariableZoneCapacity                        : 1;
        USHORT  ZoneExcursions                              : 1;
        USHORT  Reserved                                    : 14;
    } ZOC;                  // Zone Operation Characteristics

    struct {
        USHORT  ReadAcrossZoneBoundaries                    : 1;
        USHORT  Reserved                                    : 15;
    } OZCS;                 // Optional Zoned Command Support

    ULONG   MAR;            // Maximum Active Resources (MAR)
    ULONG   MOR;            // Maximum Open Resources (MOR)
    ULONG   RRL;            // Reset Recommended Limit (RRL)
    ULONG   FRL;            // Finish Recommended Limit (FRL)

    UCHAR   Reserved0[2796];

    NVME_LBA_ZONE_FORMAT LBAEF[16];     // byte 2816:2831 M - LBA Format 0 Extension (LBAFE0)
                                        // byte 2832:2847 O - LBA Format 1 Extension (LBAFE1)
                                        // byte 2848:2863 O - LBA Format 2 Extension (LBAFE2)
                                        // byte 2864:2879 O - LBA Format 3 Extension (LBAFE3)
                                        // byte 2880:2895 O - LBA Format 4 Extension (LBAFE4)
                                        // byte 2896:2911 O - LBA Format 5 Extension (LBAFE5)
                                        // byte 2912:2927 O - LBA Format 6 Extension (LBAFE6)
                                        // byte 2928:2943 O - LBA Format 7 Extension (LBAFE7)
                                        // byte 2944:2959 O - LBA Format 8 Extension (LBAFE8)
                                        // byte 2960:2975 O - LBA Format 9 Extension (LBAFE9)
                                        // byte 2976:2991 O - LBA Format 10 Extension (LBAFE10)
                                        // byte 2992:3007 O - LBA Format 11 Extension (LBAFE11)
                                        // byte 3008:3023 O - LBA Format 12 Extension (LBAFE12)
                                        // byte 3024:3039 O - LBA Format 13 Extension (LBAFE13)
                                        // byte 3040:3055 O - LBA Format 14 Extension (LBAFE14)
                                        // byte 3056:3971 O - LBA Format 15 Extension (LBAFE15)

    UCHAR           Reserved1[768];     // byte 3072:3839

    UCHAR           VS[256];            // byte 3840:4095 O - Vendor Specific (VS): This range of bytes is allocated for vendor specific usage.

} NVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET, *PNVME_IDENTIFY_SPECIFIC_NAMESPACE_IO_COMMAND_SET;

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET (0x06) with Command Set Identifier (0x00)
//
typedef struct {
    UCHAR           VSL;                // byte 0       O - Verify Size Limit (VZL)
    UCHAR           WZSL;               // byte 1       O - Write Zeroes Size Limit (WZSL)
    UCHAR           WUSL;               // byte 2       O - Write Uncorrectable Size Limit (WUSL)

    UCHAR           DMRL;               // byte 3       O - Dataset Management Ranges Limit (DMRL)
    ULONG           DMRSL;              // byte 4:7     O - Dataset Management Range Size Limit (DMRSL)
    ULONGLONG       DMSL;               // byte 8:15    O - Dataset Management Size Limit (DMSL)

    UCHAR           Reserved[4080];     // byte 16:4095

} NVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET, *PNVME_IDENTIFY_NVM_SPECIFIC_CONTROLLER_IO_COMMAND_SET;

//
// Output of NVME_IDENTIFY_CNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET (0x06) with Command Set Identifier (0x02)
//
typedef struct {
    UCHAR           ZASL;               // byte 0.          O - Zone Append Size Limit (ZASL)

    UCHAR           Reserved[4095];     // byte 1:4095

} NVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET, *PNVME_IDENTIFY_ZNS_SPECIFIC_CONTROLLER_IO_COMMAND_SET;

//
// Output of NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NSID (0x12)/NVME_IDENTIFY_CNS_CONTROLLER_LIST_OF_NVM_SUBSYSTEM (0x13)
//
typedef struct {

    USHORT  NumberOfIdentifiers;
    USHORT  ControllerID[2047];

} NVME_CONTROLLER_LIST, *PNVME_CONTROLLER_LIST;

//
// Output of NVME_IDENTIFY_CNS_UUID_LIST (CNS 0x17)
// The UUID List is 4096 bytes.
//

#define NVME_UUID_ASSOCIATION_NONE              0       // No association of UUID reported
#define NVME_UUID_ASSOCIATION_PCI_VID           1       // UUID is associated with the vendor reported in the PCI Vendor ID field of Identify Controller data
#define NVME_UUID_ASSOCIATION_PCI_SUBSYSTEM_VID 2       // UUID is associated with vendor reported in the PCI subsystem vendor ID field of Identify Controller data
#define NVME_UUID_ASSOCIATION_RESERVED          3

typedef struct {

    UCHAR      IdentifierAssociation   :  2;    // Byte 0, indicates whether the UUID is associated with a vendor.
    UCHAR      Reserved                :  6;
    UCHAR      Reserved1[15];                   // Bytes 1-15
    UCHAR      UUID[16];                        // Bytes 16-31, 128-bit UUID (per RFC 4122)

} NVME_UUID_LIST_ENTRY, *PNVME_UUID_LIST_ENTRY;

#define NVME_NUM_UUID_LIST_ENTRIES  128
#define NVME_MAX_UUID_INDEX         127     // Index for last entry in a UUID List.  Note, NVM Express
                                            // requires UUID List Entry at index 127 shall be cleared to 0h.

typedef struct {

    NVME_UUID_LIST_ENTRY       UUID[NVME_NUM_UUID_LIST_ENTRIES];      // Note, UUID[0] is reserved.

} NVME_UUID_LIST, *PNVME_UUID_LIST;

//
// Output of NVME_IDENTIFY_CNS_IO_COMMAND_SET (0x1C)
//

typedef struct {

    ULONGLONG NVMCommandSet : 1;  // Bit 0, indicates whether this vector supports NVM command set.
    ULONGLONG KVCommandSet  : 1;  // Bit 1, indicates whether this vector supports Key Value command set.
    ULONGLONG ZNCommandSet  : 1;  // Bit 2, indicates whether this vector supports Zoned Namespace command set.
    ULONGLONG Reserved      : 61; // Bits 3-63, reserved

} IO_COMMAND_SET_VECTOR, *PIO_COMMAND_SET_VECTOR;

typedef struct {

    IO_COMMAND_SET_VECTOR IOCommandSetVector[512];

} NVME_IDENTIFY_IO_COMMAND_SET, *PNVME_IDENTIFY_IO_COMMAND_SET;

//
// Data Structure of LBA Range Type entry
//
typedef enum {

    NVME_LBA_RANGE_TYPE_RESERVED            = 0,
    NVME_LBA_RANGE_TYPE_FILESYSTEM          = 1,
    NVME_LBA_RANGE_TYPE_RAID                = 2,
    NVME_LBA_RANGE_TYPE_CACHE               = 3,
    NVME_LBA_RANGE_TYPE_PAGE_SWAP_FILE      = 4,

} NVME_LBA_RANGE_TYPES;

typedef struct {
    UCHAR       Type;                   // Type (Type): Specifies the Type of the LBA range.
    struct {
        UCHAR   MayOverwritten : 1;
        UCHAR   Hidden         : 1;
        UCHAR   Reserved       : 6;
    } Attributes;                       // Attributes: Specifies attributes of the LBA range. Each bit defines an attribute.

    UCHAR       Reserved0[14];
    ULONGLONG   SLBA;                   // Starting LBA (SLBA): This field specifies the 64-bit address of the first logical block that is part of this LBA range.
    ULONGLONG   NLB;                    // Number of Logical Blocks (NLB): This field specifies the number of logical blocks that are part of this LBA range. This is a 0s based value.
    UCHAR       GUID[16];               // Unique Identifier (GUID): This field is a global unique identifier that uniquely specifies the type of this LBA range. Well known Types may be defined and are published on the NVM Express website.
    UCHAR       Reserved1[16];
} NVME_LBA_RANGET_TYPE_ENTRY, *PNVME_LBA_RANGET_TYPE_ENTRY;

//
// Vendor defined log pages
//
typedef enum {
    NVME_LOG_PAGE_OCP_DEVICE_SMART_INFORMATION    = 0xC0,          // OCP device SMART Information log page
    NVME_LOG_PAGE_OCP_DEVICE_ERROR_RECOVERY       = 0xC1,          // OCP device Error Recovery log page
    NVME_LOG_PAGE_OCP_FIRMWARE_ACTIVATION_HISTORY = 0xC2,          // OCP device Firmware Activation History log page
    NVME_LOG_PAGE_OCP_LATENCY_MONITOR             = 0xC3,          // OCP device Latency Monitor log page
    NVME_LOG_PAGE_OCP_DEVICE_CAPABILITIES         = 0xC4,          // OCP device Device Capabilities log page
    NVME_LOG_PAGE_OCP_UNSUPPORTED_REQUIREMENTS    = 0xC5,          // OCP device Unsupported Requirements log page
    NVME_LOG_PAGE_OCP_TCG_CONFIGURATION           = 0xC8,          // OCP device TCG Configuration log page
    NVME_LOG_PAGE_OCP_TCG_HISTORY                 = 0xC9           // OCP device TCG History log page
} NVME_VENDOR_LOG_PAGES;

#define NVME_VENDOR_SPECIFIC_LOG_PAGE_MIN_IDENTIFIER                 0xC0
#define NVME_VENDOR_SPECIFIC_LOG_PAGE_MAX_IDENTIFIER                 0xFF

#define NVME_LOG_PAGE_WCS_DEVICE_SMART_ATTRIBUTES    NVME_LOG_PAGE_OCP_DEVICE_SMART_INFORMATION // WCS device SMART Attributes log page
#define NVME_LOG_PAGE_WCS_DEVICE_ERROR_RECOVERY      NVME_LOG_PAGE_OCP_DEVICE_ERROR_RECOVERY // WCS device Error Recovery log page

//
// SMART Attributes Log Page GUID is defined in spec as byte stream: 0xAFD514C97C6F4F9CA4F2BFEA2810AFC5
// which is converted to GUID format as: {2810AFC5-BFEA-A4F2-9C4F-6F7CC914D5AF}
//
#define GUID_OCP_DEVICE_SMART_INFORMATIONGuid { 0x2810AFC5, 0xBFEA, 0xA4F2, { 0x9C, 0x4F, 0x6F, 0x7C, 0xC9, 0x14, 0xD5, 0xAF} }
DEFINE_GUID(GUID_OCP_DEVICE_SMART_INFORMATION, 0x2810AFC5, 0xBFEA, 0xA4F2, 0x9C, 0x4F, 0x6F, 0x7C, 0xC9, 0x14, 0xD5, 0xAF);

#define GUID_WCS_DEVICE_SMART_ATTRIBUTESGuid { 0x2810AFC5, 0xBFEA, 0xA4F2, { 0x9C, 0x4F, 0x6F, 0x7C, 0xC9, 0x14, 0xD5, 0xAF} }
DEFINE_GUID(GUID_WCS_DEVICE_SMART_ATTRIBUTES, 0x2810AFC5, 0xBFEA, 0xA4F2, 0x9C, 0x4F, 0x6F, 0x7C, 0xC9, 0x14, 0xD5, 0xAF);

//
// Error Recovery Log Page GUID is defined in spec as byte stream: 0x5A1983BA3DFD4DABAE3430FE2131D944
// which is converted to GUID format as: {2131D944-30FE-AE34-AB4D-FD3DBA83195A}
//
#define GUID_OCP_DEVICE_ERROR_RECOVERYGuid { 0x2131D944, 0x30FE, 0xAE34, {0xAB, 0x4D, 0xFD, 0x3D, 0xBA, 0x83, 0x19, 0x5A} }
DEFINE_GUID(GUID_OCP_DEVICE_ERROR_RECOVERY, 0x2131D944, 0x30FE, 0xAE34, 0xAB, 0x4D, 0xFD, 0x3D, 0xBA, 0x83, 0x19, 0x5A);

#define GUID_WCS_DEVICE_ERROR_RECOVERYGuid { 0x2131D944, 0x30FE, 0xAE34, {0xAB, 0x4D, 0xFD, 0x3D, 0xBA, 0x83, 0x19, 0x5A} }
DEFINE_GUID(GUID_WCS_DEVICE_ERROR_RECOVERY, 0x2131D944, 0x30FE, 0xAE34, 0xAB, 0x4D, 0xFD, 0x3D, 0xBA, 0x83, 0x19, 0x5A);

//
// Firmware Activation History Log Page GUID is defined in spec as byte stream: 0xD11CF3AC8AB24DE2A3F6DAB4769A796D
// which is converted to GUID format as: {769A796D-DAB4-A3F6-E24D-B28AACF31CD1}
//
#define GUID_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORYGuid { 0x769A796D, 0xDAB4, 0xA3F6, { 0xE2, 0x4D, 0xB2, 0x8A, 0xAC, 0xF3, 0x1C, 0xD1} }
DEFINE_GUID(GUID_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY, 0x769A796D, 0xDAB4, 0xA3F6, 0xE2, 0x4D, 0xB2, 0x8A, 0xAC, 0xF3, 0x1C, 0xD1);

//
// Latency Monitor Log Page GUID is defined in spec as byte stream: 0x85D45E58D4E643709C6C84D08CC07A92
// which is converted to GUID format as: {8CC07A92-84D0-9C6C-7043-E6D4585ED485}
//
#define GUID_OCP_DEVICE_LATENCY_MONITORGuid { 0x8CC07A92, 0x84D0, 0x9C6C, { 0x70, 0x43, 0xE6, 0xD4, 0x58, 0x5E, 0xD4, 0x85} }
DEFINE_GUID(GUID_OCP_DEVICE_LATENCY_MONITOR, 0x8CC07A92, 0x84D0, 0x9C6C, 0x70, 0x43, 0xE6, 0xD4, 0x58, 0x5E, 0xD4, 0x85);

//
// Device Capabilities Log Page GUID is defined in spec as byte stream: 0xB7053C914B58495D98C9E1D10D054297
// which is converted to GUID format as: {0D054297-E1D1-98C9-5D49-584B913C05B7}
//
#define GUID_OCP_DEVICE_DEVICE_CAPABILITIESGuid { 0x0D054297, 0xE1D1, 0x98C9, { 0x5D, 0x49, 0x58, 0x4B, 0x91, 0x3C, 0x05, 0xB7} }
DEFINE_GUID(GUID_OCP_DEVICE_DEVICE_CAPABILITIES, 0x0D054297, 0xE1D1, 0x98C9, 0x5D, 0x49, 0x58, 0x4B, 0x91, 0x3C, 0x05, 0xB7);

//
// Unsupported Requirements Log Page GUID is defined in spec as byte stream: 0xC7BB98B7D0324863BB2C23990E9C722F
// which is converted to GUID format as: {0E9C722F-2399-BB2C-6348-32D0B798BBC7}
//
#define GUID_OCP_DEVICE_UNSUPPORTED_REQUIREMENTSGuid { 0x0E9C722F, 0x2399, 0xBB2C, { 0x63, 0x48, 0x32, 0xD0, 0xB7, 0x98, 0xBB, 0xC7} }
DEFINE_GUID(GUID_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS, 0x0E9C722F, 0x2399, 0xBB2C, 0x63, 0x48, 0x32, 0xD0, 0xB7, 0x98, 0xBB, 0xC7);

//
// TCG Configuration Log Page GUID is defined in spec as byte stream: 0x54E02A9DFA5447C083E6E07EBD244006
// which is converted to GUID format as: {BD244006-E07E-83E6-C047-54FA9D2AE054}
//
#define GUID_OCP_DEVICE_TCG_CONFIGURATIONGuid { 0xBD244006, 0xE07E, 0x83E6, { 0xC0, 0x47, 0x54, 0xFA, 0x9D, 0x2A, 0xE0, 0x54} }
DEFINE_GUID(GUID_OCP_DEVICE_TCG_CONFIGURATION, 0xBD244006, 0xE07E, 0x83E6, 0xC0, 0x47, 0x54, 0xFA, 0x9D, 0x2A, 0xE0, 0x54);

//
// TCG History Log Page GUID is defined in spec as byte stream: 0x88D7909696D04E27949009C6704b513E
// which is converted to GUID format as: {704B513E-09C6-9490-274E-D0969690D788}
//
#define GUID_OCP_DEVICE_TCG_HISTORYGuid { 0x704B513E, 0x09C6, 0x9490, { 0x27, 0x4E, 0xD0, 0x96, 0x96, 0x90, 0xD7, 0x88} }
DEFINE_GUID(GUID_OCP_DEVICE_TCG_HISTORY, 0x704B513E, 0x09C6, 0x9490, 0x27, 0x4E, 0xD0, 0x96, 0x96, 0x90, 0xD7, 0x88);

//
// MFND child controller event Log Page GUID is defined in spec as byte stream: 0x9C669D257FD944A5BF35A5F098BCCE18
// which is converted to GUID format as: {98BCCE18-A5F0-BF35-A544-D97F259D669C}
//
#define GUID_MFND_CHILD_CONTROLLER_EVENT_LOG_PAGEGuid { 0x98BCCE18, 0xA5F0, 0xBF35, {0xA5, 0x44, 0xD9, 0x7F, 0x25, 0x9D, 0x66, 0x9C} }
DEFINE_GUID(GUID_MFND_CHILD_CONTROLLER_EVENT_LOG_PAGE, 0x98BCCE18, 0xA5F0, 0xBF35, 0xA5, 0x44, 0xD9, 0x7F, 0x25, 0x9D, 0x66, 0x9C);

//
// MFND child controller QoS statistics Log Page GUID is defined in spec as byte stream: 0x0F5F578400403E87464406529CB5FA26
// which is converted to GUID format as: {9CB5FA26-0652-4644-873E-400084575F0F}
//
#define GUID_MFND_CHILD_CONTROLLER_QOS_STAT_LOG_PAGEGuid { 0x9CB5FA26, 0x0652, 0x4644, {0x87, 0x3E, 0x40, 0x00, 0x84, 0x57, 0x5F, 0x0F} }
DEFINE_GUID(GUID_MFND_CHILD_CONTROLLER_QOS_STAT_LOG_PAGE, 0x9CB5FA26, 0x0652, 0x4644, 0x87, 0x3E, 0x40, 0x00, 0x84, 0x57, 0x5F, 0x0F);

//
// Notice Status: NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC
//
typedef enum {

    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_RESERVED = 0,
    NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_DEVICE_PANIC = 1,

} NVME_ASYNC_EVENT_TYPE_VENDOR_SPECIFIC_CODES;

//
// Device recommended reset action on firmware assert for Windows Cloud Server Devices
//
typedef struct _NVME_WCS_DEVICE_RESET_ACTION {
    union {
        struct {
            UCHAR ControllerReset : 1;
            UCHAR NVMeSubsystemReset : 1;
            UCHAR PCIeFLR : 1;
            UCHAR PERST : 1;
            UCHAR PowerCycle : 1;
            UCHAR PCIeConventionalHotReset : 1;
            UCHAR Reserved : 2;
        };

        UCHAR AsUCHAR;
    };

} NVME_WCS_DEVICE_RESET_ACTION, * PNVME_WCS_DEVICE_RESET_ACTION;

//
// Windows Cloud Server device capabilities
//
typedef struct _NVME_WCS_DEVICE_CAPABILITIES {
    union {
        struct {
            ULONG PanicAEN : 1;
            ULONG PanicCFS : 1;
            ULONG Reserved : 30;
        };

        ULONG AsULONG;
    };

} NVME_WCS_DEVICE_CAPABILITIES, * PNVME_WCS_DEVICE_CAPABILITIES;

//
// Device recovery action on device panic
//
typedef enum _NVME_WCS_DEVICE_RECOVERY_ACTION1
{
    NVMeDeviceRecoveryNoAction = 0,          // Requires no action
    NVMeDeviceRecoveryFormatNVM,             // Requires Format NVM
    NVMeDeviceRecoveryVendorSpecificCommand, // Requires Vendor Specific Command
    NVMeDeviceRecoveryVendorAnalysis,        // Requires Vendor Analysis
    NVMeDeviceRecoveryDeviceReplacement,     // Requires Device Replacement
    NVMeDeviceRecoverySanitize,              // Requires Sanitize
    NVMeDeviceRecovery1Max = 15               // Not an actual action, denotes max action.
} NVME_WCS_DEVICE_RECOVERY_ACTION1, * PNVME_WCS_DEVICE_RECOVERY_ACTION1;

typedef enum _NVME_WCS_DEVICE_RECOVERY_ACTION2
{
    NVMeDeviceRecoveryControllerReset = 0,   // Requires controller reset
    NVMeDeviceRecoverySubsystemReset,        // Requires NVM subsystem reset
    NVMeDeviceRecoveryPcieFunctionReset,     // Requires PCIe Function Level Reset
    NVMeDeviceRecoveryPERST,                 // Requires PERST#
    NVMeDeviceRecoveryPowerCycle,            // Requires Main Power Cycle
    NVMeDeviceRecoveryPcieHotReset,          // Requires PCIe Conventional Hot Reset
    NVMeDeviceRecovery2Max = 15              // Not an actual action, denotes max action.
} NVME_WCS_DEVICE_RECOVERY_ACTION2, * PNVME_WCS_DEVICE_RECOVERY_ACTION2;

#pragma pack(push, 1)

//
// Log page definition of NVME_LOG_PAGE_WCS_DEVICE_SMART_ATTRIBUTES/NVME_LOG_PAGE_OCP_DEVICE_SMART_INFORMATION. Size 512 bytes
//

// Version independent structure to perform basic validation

typedef struct _NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG {

    UCHAR VersionSpecificData[494];

    USHORT LogPageVersionNumber;

    GUID LogPageGUID;           // Shall be set to GUID_WCS_DEVICE_SMART_ATTRIBUTES / GUID_OCP_DEVICE_SMART_INFORMATION

} NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG, *PNVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG;

C_ASSERT(sizeof(NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG) == 512);

// Version 2

#define NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_VERSION_2        0x0002

typedef struct _NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2 {

    UCHAR MediaUnitsWritten[16];
    UCHAR MediaUnitsRead[16];

    struct {
        UCHAR RawCount[6];
        UCHAR Normalized[2];
    } BadUserNANDBlockCount;

    struct {
        UCHAR RawCount[6];
        UCHAR Normalized[2];
    } BadSystemNANDBlockCount;

    ULONGLONG XORRecoveryCount;
    ULONGLONG UnrecoverableReadErrorCount;
    ULONGLONG SoftECCErrorCount;

    struct {
        ULONG DetectedCounts;
        ULONG CorrectedCounts;
    } EndToEndCorrectionCounts;

    UCHAR PercentageSystemDataUsed;
    UCHAR RefreshCount[7];

    struct {
        ULONG MaximumCount;
        ULONG MinimumCount;
    } UserDataEraseCounts;

    struct {
        UCHAR EventCount;
        UCHAR Status;
    } ThermalThrottling;

    UCHAR Reserved0[6];

    ULONGLONG PCIeCorrectableErrorCount;
    ULONG IncompleteShutdownCount;

    ULONG Reserved1;

    UCHAR PercentageFreeBlocks;

    UCHAR Reserved2[7];

    USHORT CapacitorHealth;

    UCHAR Reserved3[6];

    ULONGLONG UnalignedIOCount;
    ULONGLONG SecurityVersionNumber;
    ULONGLONG NUSE;

    UCHAR PLPStartCount[16];
    UCHAR EnduranceEstimate[16];

    UCHAR Reserved4[302];

    USHORT LogPageVersionNumber; // Shall be set to NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_VERSION_2

    GUID LogPageGUID;           // Shall be set to GUID_WCS_DEVICE_SMART_ATTRIBUTES

} NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2, *PNVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2;

C_ASSERT(sizeof(NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG_V2) == sizeof(NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG));

// Version 3

#define NVME_OCP_DEVICE_SMART_INFORMATION_LOG_VERSION_3        0x0003

#define NVME_OCP_DEVICE_DSSD_SPEC_MAJOR_VERSION_0              0x00 // For OCP 1.0 compliance
#define NVME_OCP_DEVICE_DSSD_SPEC_MAJOR_VERSION_2              0x02 // For OCP 2.0 compliance

typedef struct _NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3 {

    UCHAR MediaUnitsWritten[16];
    UCHAR MediaUnitsRead[16];

    struct {
        UCHAR RawCount[6];
        UCHAR Normalized[2];
    } BadUserNANDBlockCount;

    struct {
        UCHAR RawCount[6];
        UCHAR Normalized[2];
    } BadSystemNANDBlockCount;

    ULONGLONG XORRecoveryCount;
    ULONGLONG UnrecoverableReadErrorCount;
    ULONGLONG SoftECCErrorCount;

    struct {
        ULONG DetectedCounts;
        ULONG CorrectedCounts;
    } EndToEndCorrectionCounts;

    UCHAR PercentageSystemDataUsed;
    UCHAR RefreshCount[7];

    struct {
        ULONG MaximumCount;
        ULONG MinimumCount;
    } UserDataEraseCounts;

    struct {
        UCHAR EventCount;
        UCHAR Status;
    } ThermalThrottling;

    struct {
        UCHAR Errata;
        USHORT PointVersion;
        USHORT MinorVersion;
        UCHAR MajorVersion;
    } DSSDSpecVersion;

    ULONGLONG PCIeCorrectableErrorCount;
    ULONG IncompleteShutdownCount;

    ULONG Reserved1;

    UCHAR PercentageFreeBlocks;

    UCHAR Reserved2[7];

    USHORT CapacitorHealth;

    UCHAR NvmeErrata;

    UCHAR Reserved3[5];

    ULONGLONG UnalignedIOCount;
    ULONGLONG SecurityVersionNumber;
    ULONGLONG NUSE;

    UCHAR PLPStartCount[16];
    UCHAR EnduranceEstimate[16];

    ULONGLONG PCIeLinkRetrainingCount;
    ULONGLONG PowerStateChangeCount;

    UCHAR Reserved4[286];

    USHORT LogPageVersionNumber;

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_SMART_INFORMATION

} NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3, *PNVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3;

C_ASSERT(sizeof(NVME_OCP_DEVICE_SMART_INFORMATION_LOG_V3) == sizeof(NVME_WCS_DEVICE_SMART_ATTRIBUTES_LOG));

//
// Log page definition of NVME_LOG_PAGE_WCS_DEVICE_ERROR_RECOVERY. Size 512 bytes
//

// Version independent structure to perform basic validation

typedef struct _NVME_WCS_DEVICE_ERROR_RECOVERY_LOG {

    // Amount of time to wait for device panic workflow to complete in msec. Delay the reset accordingly
    USHORT PanicResetWaitTime;

    // Reset actions on firmware assert, multiple options could be set
    NVME_WCS_DEVICE_RESET_ACTION PanicResetAction;

    //Recovery action for device panic condition
    UCHAR DriveRecoveryAction;

    // Id to identify the panic condition
    ULONGLONG  PanicId;

    // Device capabilities
    NVME_WCS_DEVICE_CAPABILITIES DeviceCapabilities;

    // Vendor specific command opcode to recover device from panic condition
    UCHAR VendorSpecificRecoveryCode;

    UCHAR Reserved0[3];

    // CDW12 value for the Vendor Specific command to recover device from panic condition
    ULONG VendorSpecificCommandCDW12;

    // CDW13 value for the Vendor Specific command to recover device from panic condition
    ULONG VendorSpecificCommandCDW13;

    UCHAR Reserved1[466];

    USHORT LogPageVersionNumber;

    GUID LogPageGUID;           // Shall be set to GUID_WCS_DEVICE_ERROR_RECOVERY / GUID_OCP_DEVICE_ERROR_RECOVERY

} NVME_WCS_DEVICE_ERROR_RECOVERY_LOG, *PNVME_WCS_DEVICE_ERROR_RECOVERY_LOG;

C_ASSERT(sizeof(NVME_WCS_DEVICE_ERROR_RECOVERY_LOG) == 512);

// Version 1

#define NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_VERSION_1        0x0001

typedef struct _NVME_WCS_DEVICE_ERROR_RECOVERY_LOG NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1, *PNVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1;

C_ASSERT(sizeof(NVME_WCS_DEVICE_ERROR_RECOVERY_LOG_V1) == sizeof(NVME_WCS_DEVICE_ERROR_RECOVERY_LOG));

// Version 2

#define NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_VERSION_2        0x0002

typedef struct _NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2 {

    // Amount of time to wait for device panic workflow to complete in msec. Delay the reset accordingly
    USHORT PanicResetWaitTime;

    // Reset actions on firmware assert, multiple options could be set
    NVME_WCS_DEVICE_RESET_ACTION PanicResetAction;

    //Recovery action for device panic condition
    UCHAR DeviceRecoveryAction1;

    // Id to identify the panic condition
    ULONGLONG  PanicId;

    // Device capabilities
    NVME_WCS_DEVICE_CAPABILITIES DeviceCapabilities;

    // Vendor specific command opcode to recover device from panic condition
    UCHAR VendorSpecificRecoveryCode;

    UCHAR Reserved0[3];

    // CDW12 value for the Vendor Specific command to recover device from panic condition
    ULONG VendorSpecificCommandCDW12;

    // CDW13 value for the Vendor Specific command to recover device from panic condition
    ULONG VendorSpecificCommandCDW13;

    UCHAR VendorSpecificCommandTimeout;

    UCHAR DeviceRecoveryAction2;

    UCHAR DeviceRecoveryAction2Timeout;

    UCHAR Reserved1[463];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_VERSION_2

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_ERROR_RECOVERY

} NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2, *PNVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2;

C_ASSERT(sizeof(NVME_OCP_DEVICE_ERROR_RECOVERY_LOG_V2) == sizeof(NVME_WCS_DEVICE_ERROR_RECOVERY_LOG));

//
// Log page definition of NVME_LOG_PAGE_OCP_FIRMWARE_ACTIVATION_HISTORY. Size 4096 bytes
//

#define FIRMWARE_ACTIVATION_HISTORY_ENTRY_VERSION_1        0x01

typedef struct _FIRMWARE_ACTIVATION_HISTORY_ENTRY {

    UCHAR VersionNumber;
    UCHAR Length;
    USHORT Reserved0;
    USHORT ActivationCount;
    ULONGLONG Timestamp;
    ULONGLONG Reserved1;
    ULONGLONG PowerCycleCount;
    ULONGLONG PreviousFirmware;
    ULONGLONG NewFirmware;
    UCHAR SlotNumber;
    UCHAR CommitActionType;
    USHORT Result;
    UCHAR Reserved2[14];

} FIRMWARE_ACTIVATION_HISTORY_ENTRY, *PFIRMWARE_ACTIVATION_HISTORY_ENTRY;

C_ASSERT(sizeof(FIRMWARE_ACTIVATION_HISTORY_ENTRY) == 64);

#define NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG_VERSION_1        0x0001

typedef struct _NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG {

    UCHAR LID; // Shall be set to NVME_LOG_PAGE_OCP_FIRMWARE_ACTIVATION_HISTORY

    UCHAR Reserved0[3];

    ULONG ValidNumberOfEntries;

    FIRMWARE_ACTIVATION_HISTORY_ENTRY Entries[20];

    UCHAR Reserved1[2790];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG_VERSION_1

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY

} NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG, *PNVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG;

C_ASSERT(sizeof(NVME_OCP_DEVICE_FIRMWARE_ACTIVATION_HISTORY_LOG) == 4096);

//
// Log page definition of NVME_LOG_PAGE_OCP_LATENCY_MONITOR. Size 512 bytes
//

typedef struct _LATENCY_MONITOR_FEATURE_STATUS {
    union {
        struct {
            UCHAR FeatureEnabled : 1;
            UCHAR ActiveLatencyMode : 1;
            UCHAR ActiveMeasuredLatency : 1;
            UCHAR Reserved : 5;
        };

        UCHAR AsUchar;
    };

} LATENCY_MONITOR_FEATURE_STATUS, * PLATENCY_MONITOR_FEATURE_STATUS;

typedef struct _ACTIVE_LATENCY_CONFIGURATION {
    union {
        struct {
            USHORT Read0 : 1;
            USHORT Write0 : 1;
            USHORT Trim0 : 1;
            USHORT Read1 : 1;
            USHORT Write1 : 1;
            USHORT Trim1 : 1;
            USHORT Read2 : 1;
            USHORT Write2 : 1;
            USHORT Trim2 : 1;
            USHORT Read3 : 1;
            USHORT Write3 : 1;
            USHORT Trim3 : 1;
            USHORT Reserved : 4;
        };

        USHORT AsUshort;
    };

} ACTIVE_LATENCY_CONFIGURATION, * PACTIVE_LATENCY_CONFIGURATION;

typedef struct _BUCKET_COUNTER {

    ULONG Reserved;
    ULONG Trim;
    ULONG Write;
    ULONG Read;

} BUCKET_COUNTER, * PBUCKET_COUNTER;

C_ASSERT(sizeof(BUCKET_COUNTER) == 16);

typedef struct _LATENCY_STAMP {

    ULONGLONG Trim3;
    ULONGLONG Write3;
    ULONGLONG Read3;

    ULONGLONG Trim2;
    ULONGLONG Write2;
    ULONGLONG Read2;

    ULONGLONG Trim1;
    ULONGLONG Write1;
    ULONGLONG Read1;

    ULONGLONG Trim0;
    ULONGLONG Write0;
    ULONGLONG Read0;

} LATENCY_STAMP, * PLATENCY_STAMP;

C_ASSERT(sizeof(LATENCY_STAMP) == 96);

typedef struct _MEASURED_LATENCY {

    USHORT Trim3;
    USHORT Write3;
    USHORT Read3;

    USHORT Trim2;
    USHORT Write2;
    USHORT Read2;

    USHORT Trim1;
    USHORT Write1;
    USHORT Read1;

    USHORT Trim0;
    USHORT Write0;
    USHORT Read0;

} MEASURED_LATENCY, * PMEASURED_LATENCY;

C_ASSERT(sizeof(MEASURED_LATENCY) == 24);

typedef struct _LATENCY_STAMP_UNITS {

    USHORT Read0 : 1;
    USHORT Write0 : 1;
    USHORT Trim0 : 1;

    USHORT Read1 : 1;
    USHORT Write1 : 1;
    USHORT Trim1 : 1;

    USHORT Read2 : 1;
    USHORT Write2 : 1;
    USHORT Trim2 : 1;

    USHORT Read3 : 1;
    USHORT Write3 : 1;
    USHORT Trim3 : 1;

    USHORT Reserved : 4;

} LATENCY_STAMP_UNITS, * PLATENCY_STAMP_UNITS;

C_ASSERT(sizeof(LATENCY_STAMP_UNITS) == 2);

typedef struct _DEBUG_BIT_FIELD {

    USHORT Read0 : 1;
    USHORT Write0 : 1;
    USHORT Trim0 : 1;

    USHORT Read1 : 1;
    USHORT Write1 : 1;
    USHORT Trim1 : 1;

    USHORT Read2 : 1;
    USHORT Write2 : 1;
    USHORT Trim2 : 1;

    USHORT Read3 : 1;
    USHORT Write3 : 1;
    USHORT Trim3 : 1;

    USHORT Reserved : 4;

} DEBUG_BIT_FIELD, * PDEBUG_BIT_FIELD;

C_ASSERT(sizeof(DEBUG_BIT_FIELD) == 2);

#define NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_VERSION_1        0x0001

typedef struct _NVME_OCP_DEVICE_LATENCY_MONITOR_LOG {

    LATENCY_MONITOR_FEATURE_STATUS FeatureStatus;

    UCHAR Reserved0;

    USHORT ActiveBucketTimer;
    USHORT ActiveBucketTimerThreshold;

    UCHAR ActiveThresholdA;
    UCHAR ActiveThresholdB;
    UCHAR ActiveThresholdC;
    UCHAR ActiveThresholdD;

    ACTIVE_LATENCY_CONFIGURATION ActiveLatencyConfig;
    UCHAR ActiveLatencyMinimumWindow;

    UCHAR Reserved1[19];

    BUCKET_COUNTER ActiveBucketCounter0;
    BUCKET_COUNTER ActiveBucketCounter1;
    BUCKET_COUNTER ActiveBucketCounter2;
    BUCKET_COUNTER ActiveBucketCounter3;

    LATENCY_STAMP ActiveLatencyStamp;
    MEASURED_LATENCY ActiveMeasuredLatency;
    LATENCY_STAMP_UNITS ActiveLatencyStampUnits;

    UCHAR Reserved2[22];

    BUCKET_COUNTER StaticBucketCounter0;
    BUCKET_COUNTER StaticBucketCounter1;
    BUCKET_COUNTER StaticBucketCounter2;
    BUCKET_COUNTER StaticBucketCounter3;

    LATENCY_STAMP StaticLatencyStamp;
    MEASURED_LATENCY StaticMeasuredLatency;
    LATENCY_STAMP_UNITS StaticLatencyStampUnits;

    UCHAR Reserved3[22];

    DEBUG_BIT_FIELD DebugLogTriggerEnable;
    USHORT DebugLogMeasuredLatency;
    ULONGLONG DebugLogLatencyStamp;
    USHORT DebugLogPointer;
    DEBUG_BIT_FIELD DebugCounterTriggerSource;

    union {
        struct {
            UCHAR BasedOnTimestamp : 1;
            UCHAR Reserved : 7;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;
    } DebugLogStampUnits;

    UCHAR Reserved4[29];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_LATENCY_MONITOR_LOG_VERSION_1

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_LATENCY_MONITOR

} NVME_OCP_DEVICE_LATENCY_MONITOR_LOG, *PNVME_OCP_DEVICE_LATENCY_MONITOR_LOG;

C_ASSERT(sizeof(NVME_OCP_DEVICE_LATENCY_MONITOR_LOG) == 512);

//
// Log page definition of NVME_LOG_PAGE_OCP_DEVICE_CAPABILITIES. Size 4096 bytes
//

typedef struct _DSSD_POWER_STATE_DESCRIPTOR {

    UCHAR NvmePowerState : 5;
    UCHAR Reserved : 2;
    UCHAR ValidDSSDPowerState : 1;

} DSSD_POWER_STATE_DESCRIPTOR, *PDSSD_POWER_STATE_DESCRIPTOR;

C_ASSERT(sizeof(DSSD_POWER_STATE_DESCRIPTOR) == 1);

#define NVME_OCP_DEVICE_CAPABILITIES_LOG_VERSION_1        0x0001

typedef struct _NVME_OCP_DEVICE_CAPABILITIES_LOG {

    USHORT PciePorts;

    union {
        struct {
            USHORT MctpOverSMBusSupported : 1;
            USHORT MctpOverPcieVDMSupported : 1;
            USHORT BasicMgmtCommandSupported : 1;
            USHORT Reserved : 12;
            USHORT CompliesWithSpec : 1;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;
    } OobMgmtSupport;

    union {
        struct {
            USHORT Supported : 1;
            USHORT DEACBitSupported : 1;
            USHORT FUABitSupported : 1;
            USHORT NvmeIo5Met : 1;
            USHORT NvmeIo6Met : 1;
            USHORT Reserved : 10;
            USHORT CompliesWithSpec : 1;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;
    } WriteZeroesCommand;

    union {
        struct {
            USHORT Supported : 1;
            USHORT CryptoEraseSupported : 1;
            USHORT BlockEraseSupported : 1;
            USHORT OverwriteSupported : 1;
            USHORT DeallocateLbaSupported : 1;
            USHORT Reserved : 10;
            USHORT CompliesWithSpec : 1;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;
    } SanitizeCommand;

    union {
        struct {
            USHORT Supported : 1;
            USHORT AttribDeallocateSupported : 1;
            USHORT Reserved : 13;
            USHORT CompliesWithSpec : 1;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;
    } DatasetMgmtCommand;

    union {
        struct {
            USHORT Supported : 1;
            USHORT SingleLBASupported : 1;
            USHORT MaxLBASupported : 1;
            USHORT NvmeIo14Met : 1;
            USHORT Reserved : 11;
            USHORT CompliesWithSpec : 1;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;
    } WriteUncorrectableCommand;

    union {
        struct {
            USHORT CWFusedSupported : 1;
            USHORT Reserved : 14;
            USHORT CompliesWithSpec : 1;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;
    } FusedCommand;

    USHORT MinimumValidDSSDPowerState;

    UCHAR Reserved0;

    DSSD_POWER_STATE_DESCRIPTOR DssdDescriptors[127];

    UCHAR Reserved1[3934];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_CAPABILITIES_LOG_VERSION_1

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_DEVICE_CAPABILITIES

} NVME_OCP_DEVICE_CAPABILITIES_LOG, *PNVME_OCP_DEVICE_CAPABILITIES_LOG;

C_ASSERT(sizeof(NVME_OCP_DEVICE_CAPABILITIES_LOG) == 4096);

//
// Log page definition of NVME_LOG_PAGE_OCP_UNSUPPORTED_REQUIREMENTS. Size 4096 bytes
//

typedef struct _UNSUPPORTED_REQUIREMENT {

    UCHAR ReqId[16]; // Zero padded ASCII string of the requirement id not supported

} UNSUPPORTED_REQUIREMENT, *PUNSUPPORTED_REQUIREMENT;

#define NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG_VERSION_1        0x0001

typedef struct _NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG {

    USHORT UnsupportedCount;

    UCHAR Reserved0[14];

    UNSUPPORTED_REQUIREMENT UnsupportedReqList[253];

    UCHAR Reserved1[14];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG_VERSION_1

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS

} NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG, *PNVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG;

C_ASSERT(sizeof(NVME_OCP_DEVICE_UNSUPPORTED_REQUIREMENTS_LOG) == 4096);

//
// Log page definition of NVME_LOG_PAGE_OCP_TCG_CONFIGURATION. Size 512 bytes
//

#define NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_VERSION_1        0x0001

typedef struct _NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG {

    union {

        struct {

            UCHAR CPINSIDValue : 1;
            UCHAR CPINSIDBlocked : 1;
            UCHAR LockingEnabled : 1;
            UCHAR SUMOwner : 1;
            UCHAR Reserved : 4;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;
    } State;

    UCHAR Reserved0[3];

    // Locking SP Activation count
    UCHAR LSPActivationCount;

    // TPer Revert count
    UCHAR TPRevertCount;

    // Locking SP Revert count
    UCHAR LSPRevertCount;

    // Locking Object Count in Locking Table
    UCHAR LOCount;

    // Single User Mode Locking Object count
    UCHAR SUMLOCount;

    // Range Provisioned Locking Object count
    UCHAR RPLOCount;

    // Namespace Provisioned Locking Object count
    UCHAR NPLOCount;

    // Read Locked Locking Object count
    UCHAR RLLOCount;

    // Write Locked Locking Object count
    UCHAR WLLOCount;

    // Read Unlocked Locking Object count
    UCHAR RULOCount;

    // Write Unlocked Locking Object count
    UCHAR WULOCount;

    UCHAR Reserved1;

    // SID Authentication Try (failed) count
    ULONG SIDAuthTryCount;

    // SID Authentication Try (failed) limit
    ULONG SIDAuthTryLimit;

    // Programmatic TCG Reset count
    ULONG ResetCount;

    // Count of Locking Objects transitioned to locked state
    // due to Programmatic TCG Reset
    ULONG ResetLockCount;

    UCHAR Reserved2[462];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG_VERSION_1

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_TCG_CONFIGURATION

} NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG, *PNVME_OCP_DEVICE_TCG_CONFIGURATION_LOG;

C_ASSERT(sizeof(NVME_OCP_DEVICE_TCG_CONFIGURATION_LOG) == 512);

//
// Log page definition of NVME_LOG_PAGE_OCP_TCG_HISTORY. Size 4096 bytes
//

#define TCG_HISTORY_ENTRY_VERSION_1        0x01

typedef struct _TCG_HISTORY_ENTRY {

    UCHAR VersionNumber;
    UCHAR EntryLength;
    USHORT PowerCycleCount;
    ULONG TcgCommandCount;
    ULONGLONG TcgCommandCompletionTS;
    ULONGLONG InvokingId;
    ULONGLONG MethodId;
    USHORT ComId;
    UCHAR ProtocolId;
    UCHAR TcgStatus;
    USHORT ProcessTime;
    UCHAR CommandSpecific[10];

} TCG_HISTORY_ENTRY, *PTCG_HISTORY_ENTRY;

C_ASSERT(sizeof(TCG_HISTORY_ENTRY) == 48);

typedef struct _TCG_AUTH_METHOD_SPECIFIC {

    ULONGLONG AuthorityId;
    UCHAR TriesCount;

} TCG_AUTH_METHOD_SPECIFIC, *PTCG_AUTH_METHOD_SPECIFIC;

typedef struct _TCG_ACTIVATE_METHOD_SPECIFIC {

    UCHAR RangeStartLengthPolicy;

} TCG_ACTIVATE_METHOD_SPECIFIC, *PTCG_ACTIVATE_METHOD_SPECIFIC;

typedef struct _TCG_REACTIVATE_METHOD_SPECIFIC {

    UCHAR RangeStartLengthPolicy;

} TCG_REACTIVATE_METHOD_SPECIFIC, *PTCG_REACTIVATE_METHOD_SPECIFIC;

typedef struct _TCG_ASSIGN_METHOD_SPECIFIC {

    ULONG NamespaceId;

} TCG_ASSIGN_METHOD_SPECIFIC, *PTCG_ASSIGN_METHOD_SPECIFIC;

typedef struct _TCG_BLOCKSID_METHOD_SPECIFIC {

    UCHAR ClearEvents;

} TCG_BLOCKSID_METHOD_SPECIFIC, *PTCG_BLOCKSID_METHOD_SPECIFIC;

#define NVME_OCP_DEVICE_TCG_HISTORY_LOG_VERSION_1        0x0001

typedef struct _NVME_OCP_DEVICE_TCG_HISTORY_LOG {

    UCHAR LID; // Shall be set to NVME_LOG_PAGE_OCP_TCG_HISTORY

    UCHAR Reserved0[3];

    ULONG HistoryEntryCount;

    TCG_HISTORY_ENTRY HistoryEntries[84];

    UCHAR Reserved1[38];

    USHORT LogPageVersionNumber; // Shall be set to NVME_OCP_DEVICE_TCG_HISTORY_LOG_VERSION_1

    GUID LogPageGUID;           // Shall be set to GUID_OCP_DEVICE_TCG_HISTORY

} NVME_OCP_DEVICE_TCG_HISTORY_LOG, *PNVME_OCP_DEVICE_TCG_HISTORY_LOG;

C_ASSERT(sizeof(NVME_OCP_DEVICE_TCG_HISTORY_LOG) == 4096);

#pragma pack(pop)

//
// Parameters for NVME_ADMIN_COMMAND_CREATE_IO_CQ
//
typedef union {

    struct {
        ULONG   QID         : 16;       // Queue Identifier (QID)
        ULONG   QSIZE       : 16;       // Queue Size (QSIZE)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_CREATE_IO_QUEUE, *PNVME_CDW10_CREATE_IO_QUEUE;

typedef union {

    struct {
        ULONG   PC          : 1;        // Physically Contiguous (PC)
        ULONG   IEN         : 1;        // Interrupts Enabled (IEN)
        ULONG   Reserved0   : 14;
        ULONG   IV          : 16;       // Interrupt Vector (IV)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_CREATE_IO_CQ, *PNVME_CDW11_CREATE_IO_CQ;

//
// Parameters for NVME_ADMIN_COMMAND_DELETE_IO_CQ and NVME_ADMIN_COMMAND_DELETE_IO_SQ
//
typedef union {

    struct {
        ULONG   QID         : 16;       // Queue Identifier (QID)
        ULONG   Reserved    : 16;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_DELETE_IO_QUEUE, *PNVME_CDW10_DELETE_IO_QUEUE;

//
// Parameters for NVME_ADMIN_COMMAND_CREATE_IO_SQ
//
typedef enum {

    NVME_NVM_QUEUE_PRIORITY_URGENT          = 0,
    NVME_NVM_QUEUE_PRIORITY_HIGH            = 1,
    NVME_NVM_QUEUE_PRIORITY_MEDIUM          = 2,
    NVME_NVM_QUEUE_PRIORITY_LOW             = 3,

} NVME_NVM_QUEUE_PRIORITIES;


typedef union {

    struct {
        ULONG   PC          : 1;        // Physically Contiguous (PC)
        ULONG   QPRIO       : 2;        // Queue Priority (QPRIO)
        ULONG   Reserved0   : 13;
        ULONG   CQID        : 16;       // Completion Queue Identifier (CQID)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_CREATE_IO_SQ, *PNVME_CDW11_CREATE_IO_SQ;

//
// Output and Parameters for NVME_ADMIN_COMMAND_GET_FEATURES or NVME_ADMIN_COMMAND_SET_FEATURES
//
typedef enum {

    NVME_FEATURE_VALUE_CURRENT                      = 0,
    NVME_FEATURE_VALUE_DEFAULT                      = 1,
    NVME_FEATURE_VALUE_SAVED                        = 2,
    NVME_FEATURE_VALUE_SUPPORTED_CAPABILITIES       = 3,

} NVME_FEATURE_VALUE_CODES;

typedef union {

    struct {
        ULONG   FID         : 8;        // Feature Identifier (FID)
        ULONG   SEL         : 3;        // Select (SEL): This field specifies which value of the attributes to return in the provided data.
        ULONG   Reserved0   : 21;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_GET_FEATURES, *PNVME_CDW10_GET_FEATURES;

typedef union {

    struct {
        ULONG   FID         : 8;        // Feature Identifier (FID)
        ULONG   Reserved0   : 23;
        ULONG   SV          : 1;        // Save (SV)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_SET_FEATURES, *PNVME_CDW10_SET_FEATURES;

typedef struct {

    struct {
        ULONGLONG   Timestamp   : 48;  // Timestamp in ms since Jan 1, 1970 UTC
        ULONGLONG   Synch       : 1;   // 0 = controller counted time continuously, 1 = controller may have stopped
        ULONGLONG   Origin      : 3;   // 000 = controller set timestamp to 0 at reset, 001 = timestamp set via Set Features
        ULONGLONG   Reserved    : 12;
    } DUMMYSTRUCTNAME;

    ULONGLONG   AsUlonglong;

} NVME_GET_FEATURE_TIMESTAMP, *PNVME_GET_FEATURE_TIMESTAMP;

typedef union {

    struct {
        ULONG   NSQ     : 16;           // Number of IO Submission Queues.
        ULONG   NCQ     : 16;           // Number of IO Completion Queues.
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_NUMBER_OF_QUEUES, *PNVME_CDW11_FEATURE_NUMBER_OF_QUEUES;

typedef union {

    struct {
        ULONG   THR         : 8;        // Aggregation Threshold (THR)
        ULONG   TIME        : 8;        // Aggregation Time (TIME)
        ULONG   Reserved0   : 16;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_INTERRUPT_COALESCING, *PNVME_CDW11_FEATURE_INTERRUPT_COALESCING;

typedef union {

    struct {
        ULONG   IV          : 16;       // Interrupt Vector (IV)
        ULONG   CD          : 1;        // Coalescing Disabled (CD)
        ULONG   Reserved0   : 15;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG, *PNVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG;

typedef union {

    struct {
        ULONG   DN          : 1;        // Disable Normal (DN)
        ULONG   Reserved0   : 31;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL, *PNVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL;

typedef union {

    struct {
        ULONG   NOPPME      : 1;        // Non-Operational Power State Permissive Mode Enable (NOPPME)
        ULONG   Reserved0   : 31;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE, *PNVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE;

typedef union {

    struct {
        ULONG   NUM         : 6;        // Number of LBA Ranges (NUM)
        ULONG   Reserved0   : 26;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_LBA_RANGE_TYPE, *PNVME_CDW11_FEATURE_LBA_RANGE_TYPE;

typedef union {

    struct {
        ULONG   AB          : 3;        // Arbitration Burst (AB)
        ULONG   Reserved0   : 5;
        ULONG   LPW         : 8;        // Low Priority Weight (LPW)
        ULONG   MPW         : 8;        // Medium Priority Weight (MPW)
        ULONG   HPW         : 8;        // High Priority Weight (HPW)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_ARBITRATION, *PNVME_CDW11_FEATURE_ARBITRATION;

typedef union {

    struct {
        ULONG   WCE         : 1;        // Volatile Write Cache Enable (WCE)
        ULONG   Reserved0   : 31;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE, *PNVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE;

typedef union {

    struct {
        ULONG   SAVE         : 1;        // Save supported
        ULONG   NSS          : 1;        // Namespace specific
        ULONG   MOD          : 1;        // Changeable
        ULONG   Reserved0    : 29;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_SUPPORTED_CAPABILITY, *PNVME_CDW11_FEATURE_SUPPORTED_CAPABILITY;

typedef union {

    struct {
        ULONG   CriticalWarnings            : 8;  // SMART / Health Critical Warnings
        ULONG   NsAttributeNotices          : 1;  // Namespace Attributes Notices
        ULONG   FwActivationNotices         : 1;  // Firmware Activation Notices
        ULONG   TelemetryLogNotices         : 1;  // Telemetry Log Notices
        ULONG   ANAChangeNotices            : 1;  // Asymmetric Namespace Access Change Notices
        ULONG   PredictableLogChangeNotices : 1;  // Predictable Latency Event Aggregate Log Change Notices
        ULONG   LBAStatusNotices            : 1;  // LBA Status Information Notices
        ULONG   EnduranceEventNotices       : 1;  // Endurance Group Event Aggregate Log Change Notices
        ULONG   NormalNVMSubsystemShutdown  : 1;  // Normal NVM Subsystem shutdown
        ULONG   Reserved0                   : 11;
        ULONG   ZoneDescriptorNotices       : 1;  // Zone Descriptor Changed Notices
        ULONG   Reserved1                   : 3;
        ULONG   DiscoveryLogPageChange      : 1;  // Discovery Log Page Change Notification
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG, *PNVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG;

//
// Parameter for NVME_FEATURE_POWER_MANAGEMENT
//
typedef union {

    struct {
        ULONG   PS          : 5;        // Power State (PS)
        ULONG   Reserved0   : 27;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_POWER_MANAGEMENT, *PNVME_CDW11_FEATURE_POWER_MANAGEMENT;

//
// Parameter for NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION
//
typedef union {
    struct {
        ULONG APSTE     : 1;    // Autonomous Power State Transition Enable (APSTE)
        ULONG Reserved0 : 31;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION, *PNVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION;

//
// Parameter for NVME_FEATURE_AUTONOMOUS_POWER_STATE_TRANSITION
// There is an array of 32 of these (one for each power state) in the data buffer.
//
typedef struct {
    ULONG Reserved0                 : 3;    // Bits 0-2 are reserved.
    ULONG IdleTransitionPowerState  : 5;    // Bits 3-7 - (ITPS) The non-operational power state for the controller to autonomously transition to after there is a continuous period of idle time in the current power state that exceeds time specified in the ITPT field.
    ULONG IdleTimePriorToTransition : 24;   // Bits 8-31 - (ITPT) The amount of idle time (in ms) that occurs in this power state prior to transitioning to the Idle Transition Power State.  A value of 0 disables APST for this power state.
    ULONG Reserved1;                        // Bits 32-63 are reserved.
} NVME_AUTO_POWER_STATE_TRANSITION_ENTRY, *PNVME_AUTO_POWER_STATE_TRANSITION_ENTRY;

//
// Parameter for NVME_FEATURE_TEMPERATURE_THRESHOLD
//

//
// Following definitions are used in "THSEL" field.
//
typedef enum {

    NVME_TEMPERATURE_OVER_THRESHOLD         = 0,
    NVME_TEMPERATURE_UNDER_THRESHOLD        = 1,

} NVME_TEMPERATURE_THRESHOLD_TYPES;

typedef union {

    struct {
        ULONG   TMPTH       : 16;       // Temperature Threshold (TMPTH):  Indicates the threshold for the temperature of the overall device (controller and NVM included) in units of Kelvin.
        ULONG   TMPSEL      : 4;        // Threshold Temperature Select (TMPSEL)
        ULONG   THSEL       : 2;        // Threshold Type Select (THSEL)
        ULONG   Reserved0   : 10;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD, *PNVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD;

//
// Parameter for NVME_FEATURE_ERROR_RECOVERY
//
typedef union {
    struct {
        ULONG   TLER        : 16;       // Time limited error recovery (TLER)
        ULONG   DULBE       : 1;        // Deallocated or unwritten logical block error enable (DULBE)
        ULONG   Reserved0   : 15;
    } DUMMYSTRUCTNAME;
    ULONG AsUlong;
} NVME_CDW11_FEATURE_ERROR_RECOVERY, *PNVME_CDW11_FEATURE_ERROR_RECOVERY;

//
// Parameters for NVME_FEATURE_HOST_MEMORY_BUFFER
//
typedef union {
    struct {
        ULONG EHM : 1;  // Enable Host Memory (EHM) - Enables the host memory buffer.
        ULONG MR : 1;   // Memory Return (MR) - Indicates if the host is returning previously allocated memory to the controller.
        ULONG Reserved : 30;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER, *PNVME_CDW11_FEATURE_HOST_MEMORY_BUFFER;

typedef union {
    struct {
        ULONG HSIZE; // Host Memory Buffer Size (HSIZE) - The size of the host memory buffer in memory page size (CC.MPS) units.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER, *PNVME_CDW12_FEATURE_HOST_MEMORY_BUFFER;

typedef union {
    struct {
        ULONG Reserved : 4;
        ULONG HMDLLA : 28; // Host Memory Descriptor List Lower Address (HMDLLA) - 16-byte aligned, lower 32 bits of the physical location of the Host Memory Descriptor List.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER, *PNVME_CDW13_FEATURE_HOST_MEMORY_BUFFER;

typedef union {
    struct {
        ULONG HMDLUA; // Host Memory Descriptor List Upper Address (HMDLLA) - Upper 32 bits of the physical location of the Host Memory Descriptor List.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER, *PNVME_CDW14_FEATURE_HOST_MEMORY_BUFFER;

typedef union {
    struct {
        ULONG HMDLEC; // Host Memory Descriptor List Entry Count (HMDLEC) - Number of entries in the Host Memory Descriptor List.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER, *PNVME_CDW15_FEATURE_HOST_MEMORY_BUFFER;

//
// This structure is a single entry in the host memory descriptor list.
//
typedef struct {
    ULONGLONG BADD; // Buffer Address (BADD) - Physical host memory address aligned to the memory page size (CC.MPS)
    ULONG BSIZE;    // Buffer Size (BSIZE) - The number of contiguous memory page size (CC.MPS) units for this entry.
    ULONG Reserved;
} NVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY, *PNVME_HOST_MEMORY_BUFFER_DESCRIPTOR_ENTRY;

//
// Data structure for NVME_FEATURE_HOST_BEHAVIOR_SUPPORT
//
typedef struct {

    UCHAR ACRE;                   // byte 0 Advanced Command Retry Enable
    UCHAR ETDAS;                  // byte 1 Extended Telemetry Data Area 4 Supported
    UCHAR LBAFEE;                 // byte 2 LBA Format Extension Enable
    UCHAR Reserved[509];          // byte 3:511

} NVME_HOST_BEHAVIOR_SUPPORT_DATA, *PNVME_HOST_BEHAVIOR_SUPPORT_DATA;

//
// Parameters for NVME_FEATURE_IO_COMMAND_SET_PROFILE
//
typedef union {

    struct {
        ULONG   IOCSCI     : 8;           // I/O command Set Profile
        ULONG   Reserved   : 24;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE, *PNVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE;

//
// Parameters for NVME_FEATURE_ENHANDED_CONTROLLER_METADATA, NVME_FEATURE_CONTROLLER_METADATA, NVME_FEATURE_NAMESPACE_METADATA
//
typedef union {

    struct {
        ULONG GDHM      : 1;    // Generate Default Host Metadata (GDHM)
        ULONG Reserved  : 31;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_GET_HOST_METADATA, *PNVME_CDW11_FEATURE_GET_HOST_METADATA;

typedef enum {

    NVME_HOST_METADATA_ADD_REPLACE_ENTRY        = 0,
    NVME_HOST_METADATA_DELETE_ENTRY_MULTIPLE    = 1,
    NVME_HOST_METADATA_ADD_ENTRY_MULTIPLE       = 2,

} NVME_HOST_METADATA_ELEMENT_ACTIONS;

typedef union {

    struct {
        ULONG Reserved0 : 13;

        ULONG EA        : 2;    // Element Action (EA), value defined in enum NVME_HOST_METADATA_ELEMENT_ACTIONS

        ULONG Reserved1 : 17;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_SET_HOST_METADATA, *PNVME_CDW11_FEATURE_SET_HOST_METADATA;

typedef enum {

    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_CONTROLLER_NAME   = 0x1,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_NAME       = 0x2,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_VERSION    = 0x3,
    NVME_CONTROLLER_METADATA_PREBOOT_CONTROLLER_NAME            = 0x4,
    NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_NAME                = 0x5,
    NVME_CONTROLLER_METADATA_PREBOOT_DRIVER_VERSION             = 0x6,
    NVME_CONTROLLER_METADATA_SYSTEM_PROCESSOR_MODEL             = 0x7,
    NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_NAME                = 0x8,
    NVME_CONTROLLER_METADATA_CHIPSET_DRIVER_VERSION             = 0x9,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_NAME_AND_BUILD    = 0xA,
    NVME_CONTROLLER_METADATA_SYSTEM_PRODUCT_NAME                = 0xB,
    NVME_CONTROLLER_METADATA_FIRMWARE_VERSION                   = 0xC,
    NVME_CONTROLLER_METADATA_OPERATING_SYSTEM_DRIVER_FILENAME   = 0xD,
    NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_NAME                = 0xE,
    NVME_CONTROLLER_METADATA_DISPLAY_DRIVER_VERSION             = 0xF,
    NVME_CONTROLLER_METADATA_HOST_DETERMINED_FAILURE_RECORD     = 0x10,

} NVME_CONTROLLER_METADATA_ELEMENT_TYPES;

typedef enum {

    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME             = 0x1,
    NVME_NAMESPACE_METADATA_PREBOOT_NAMESPACE_NAME                      = 0x2,
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_1 = 0x3,
    NVME_NAMESPACE_METADATA_OPERATING_SYSTEM_NAMESPACE_NAME_QUALIFIER_2 = 0x4,

} NVME_NAMESPACE_METADATA_ELEMENT_TYPES;

typedef struct {

    ULONG ET        : 6;        // Element Type (ET), value defined in enum NVME_CONTROLLER_METADATA_ELEMENT_TYPES, NVME_NAMESPACE_METADATA_ELEMENT_TYPES

    ULONG Reserved0 : 2;

    ULONG ER        : 4;        // Element Revision (ER)

    ULONG Reserved1 : 4;

    ULONG ELEN      : 16;       // Element Length (ELEN), element value length in bytes

    UCHAR EVAL[ANYSIZE_ARRAY];  // Element Value (EVAL), UTF-8 string

} NVME_HOST_METADATA_ELEMENT_DESCRIPTOR, *PNVME_HOST_METADATA_ELEMENT_DESCRIPTOR;

typedef struct {

    UCHAR NumberOfMetadataElementDescriptors;

    UCHAR Reserved0;

    UCHAR MetadataElementDescriptors[4094]; // Use NVME_HOST_METADATA_ELEMENT_DESCRIPTOR to access this list.

} NVME_FEATURE_HOST_METADATA_DATA, *PNVME_FEATURE_HOST_METADATA_DATA;

//
// Parameter for NVME_FEATURE_ERROR_INJECTION
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG NUM       : 7;    // Number of Error Injections.
        ULONG Reserved0 : 25;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_ERROR_INJECTION, *PNVME_CDW11_FEATURE_ERROR_INJECTION;

//
// DWORD 0 for get feature command (Error Injection) shares the same format with DWORD 11 for set feature command (Error Injection).
//
typedef NVME_CDW11_FEATURE_ERROR_INJECTION NVME_CDW0_FEATURE_ERROR_INJECTION, *PNVME_CDW0_FEATURE_ERROR_INJECTION;

typedef struct {

    union {

        struct {

            UCHAR Enable         : 1; // A value of 0 indicates error injection is not enabled. A value of 1 indicates error injection is enabled.

            UCHAR SingleInstance : 1; // A value of 0 indicates error injection is enabled until disable.
                                      // A value of 1 indicates a single instance error injection where a single error is injected.
                                      // After a single instance error has been created, the value of the Enable field shall be 0 in the results from Get Features command.
            UCHAR Reserved0      : 6;

        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;

    } Flags;

    UCHAR  Reserved1;
    USHORT ErrorInjectionType;             // Specifies the Type of Error Injection.
    UCHAR  ErrorInjectionTypeSpecific[28]; // Error Injection Type specific definition.

} NVME_ERROR_INJECTION_ENTRY, *PNVME_ERROR_INJECTION_ENTRY;

//
// Definitions are used in "Error Injection Type" field.
//
typedef enum {

    NVME_ERROR_INJECTION_TYPE_RESERVED0 = 0,
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_CPU_CONTROLLER_HANG,         // 0x1
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_HANG,                   // 0x2
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_PLP_DEFECT,                  // 0x3
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_LOGICAL_FW_ERROR,            // 0x4
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_CRITICAL,    // 0x5
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_DRAM_CORRUPTION_NONCRITICAL, // 0x6
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_NAND_CORRUPTION,             // 0x7
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_SRAM_CORRUPTION,             // 0x8
    NVME_ERROR_INJECTION_TYPE_DEVICE_PANIC_HW_MALFUNCTION,              // 0x9

    NVME_ERROR_INJECTION_TYPE_RESERVED1,                                // 0xA

    NVME_ERROR_INJECTION_TYPE_MAX = 0xFFFF

} NVME_ERROR_INJECTION_TYPES;

//
// Parameter for set feature NVME_FEATURE_CLEAR_FW_UPDATE_HISTORY
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG Reserved0 : 31;
        ULONG Clear     : 1;    // Clear Firmware Update History Log.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY, *PNVME_CDW11_FEATURE_CLEAR_FW_UPDATE_HISTORY;

//
// Parameter for set feature NVME_FEATURE_READONLY_WRITETHROUGH_MODE
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG Reserved0   : 30;
        ULONG EOLBehavior : 2;    // End of Life Behavior.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE, *PNVME_CDW11_FEATURE_READONLY_WRITETHROUGH_MODE;

//
// Output for get feature NVME_FEATURE_READONLY_WRITETHROUGH_MODE
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG EOLBehavior : 3;    // End of Life Behavior.
        ULONG Reserved0   : 29;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE, *PNVME_CDW0_FEATURE_READONLY_WRITETHROUGH_MODE;

//
// Parameter for set feature NVME_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG Reserved0 : 31;
        ULONG Clear     : 1;    // Clear PCIe Error Counters.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS, *PNVME_CDW11_FEATURE_CLEAR_PCIE_CORRECTABLE_ERROR_COUNTERS;

//
// Parameter for set feature NVME_FEATURE_ENABLE_IEEE1667_SILO
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG Reserved0 : 31;
        ULONG Enable    : 1;    // Enable IEEE1667 Silo.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO, *PNVME_CDW11_FEATURE_ENABLE_IEEE1667_SILO;

//
// Output for get feature NVME_FEATURE_ENABLE_IEEE1667_SILO
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG Enabled   : 3;    // IEEE1667 Silo Enabled.
        ULONG Reserved0 : 29;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO, *PNVME_CDW0_FEATURE_ENABLE_IEEE1667_SILO;

//
// Parameter for set feature NVME_FEATURE_LATENCY_MONITOR.
// This is from OCP NVMe Cloud SSD spec.
//

#pragma pack(push, 1)
typedef struct {

    USHORT ActiveBucketTimerThreshold;

    UCHAR ActiveThresholdA;

    UCHAR ActiveThresholdB;

    UCHAR ActiveThresholdC;

    UCHAR ActiveThresholdD;

    USHORT ActiveLatencyConfig;

    UCHAR ActiveLatencyMinimumWindow;

    USHORT DebugLogTriggerEnable;

    UCHAR DiscardDebugLog;

    UCHAR LatencyMonitorFeatureEnable;

    UCHAR Reserved0[4083];

} NVME_LATENCY_MONITORING_ENTRY, *PNVME_LATENCY_MONITORING_ENTRY;
#pragma pack(pop)

C_ASSERT(sizeof(NVME_LATENCY_MONITORING_ENTRY) == 4096);

//
// Output for get feature NVME_DSSD_POWER_STATE
// This is from OCP NVMe Cloud SSD spec.
//
typedef union {

    struct {
        ULONG DSSDPowerState  : 7;
        ULONG Reserved0       : 25;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW0_FEATURE_DSSD_POWER_STATE, *PNVME_CDW0_FEATURE_DSSD_POWER_STATE;

//
// Parameters for NVME_FEATURE_NVM_HOST_IDENTIFIER
//
#define NVME_MAX_HOST_IDENTIFIER_SIZE       16  // 16 Bytes, 128 Bits
#define NVME_HOST_IDENTIFIER_SIZE           8   // 8 Bytes, 64 Bits
#define NVME_EXTENDED_HOST_IDENTIFIER_SIZE  16  // 16 Bytes, 128 Bits

typedef struct {

    ULONG EXHID     : 1;                // Enable Extended Host Identifier (EXHID)
    ULONG Reserved  : 31;

} NVME_CDW11_FEATURE_HOST_IDENTIFIER, *PNVME_CDW11_FEATURE_HOST_IDENTIFIER;

typedef struct {

    UCHAR HOSTID[NVME_MAX_HOST_IDENTIFIER_SIZE];    // Host Identifier (HOSTID)

} NVME_FEATURE_HOST_IDENTIFIER_DATA, *PNVME_FEATURE_HOST_IDENTIFIER_DATA;

typedef struct {

    ULONG PTPL      : 1;                // Persist Through Power Loss (PTPL)
    ULONG Reserved  : 31;

} NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE, *PNVME_CDW11_FEATURE_RESERVATION_PERSISTENCE;

typedef struct {

    ULONG Reserved      : 1;

    ULONG REGPRE        : 1;            // Mask Registration Preempted Notification (REGPRE)
    ULONG RESREL        : 1;            // Mask Reservation Released Notification (RESREL)
    ULONG RESPRE        : 1;            // Mast Reservation Preempted Notification (RESPRE)

    ULONG Reserved1     : 28;

} NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK, *PNVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK;

typedef union {
    NVME_CDW11_FEATURE_NUMBER_OF_QUEUES                 NumberOfQueues;
    NVME_CDW11_FEATURE_INTERRUPT_COALESCING             InterruptCoalescing;
    NVME_CDW11_FEATURE_INTERRUPT_VECTOR_CONFIG          InterruptVectorConfig;
    NVME_CDW11_FEATURE_LBA_RANGE_TYPE                   LbaRangeType;
    NVME_CDW11_FEATURE_ARBITRATION                      Arbitration;
    NVME_CDW11_FEATURE_VOLATILE_WRITE_CACHE             VolatileWriteCache;
    NVME_CDW11_FEATURE_ASYNC_EVENT_CONFIG               AsyncEventConfig;
    NVME_CDW11_FEATURE_POWER_MANAGEMENT                 PowerManagement;
    NVME_CDW11_FEATURE_AUTO_POWER_STATE_TRANSITION      AutoPowerStateTransition;
    NVME_CDW11_FEATURE_TEMPERATURE_THRESHOLD            TemperatureThreshold;
    NVME_CDW11_FEATURE_ERROR_RECOVERY                   ErrorRecovery;
    NVME_CDW11_FEATURE_HOST_MEMORY_BUFFER               HostMemoryBuffer;
    NVME_CDW11_FEATURE_WRITE_ATOMICITY_NORMAL           WriteAtomicityNormal;
    NVME_CDW11_FEATURE_NON_OPERATIONAL_POWER_STATE      NonOperationalPowerState;
    NVME_CDW11_FEATURE_IO_COMMAND_SET_PROFILE           IoCommandSetProfile;
    NVME_CDW11_FEATURE_ERROR_INJECTION                  ErrorInjection;
    NVME_CDW11_FEATURE_HOST_IDENTIFIER                  HostIdentifier;
    NVME_CDW11_FEATURE_RESERVATION_PERSISTENCE          ReservationPersistence;
    NVME_CDW11_FEATURE_RESERVATION_NOTIFICATION_MASK    ReservationNotificationMask;
    NVME_CDW11_FEATURE_GET_HOST_METADATA                GetHostMetadata;
    NVME_CDW11_FEATURE_SET_HOST_METADATA                SetHostMetadata;

    ULONG   AsUlong;
} NVME_CDW11_FEATURES, *PNVME_CDW11_FEATURES;

typedef union {
    NVME_CDW12_FEATURE_HOST_MEMORY_BUFFER   HostMemoryBuffer;

    ULONG   AsUlong;
} NVME_CDW12_FEATURES, *PNVME_CDW12_FEATURES;

typedef union {
    NVME_CDW13_FEATURE_HOST_MEMORY_BUFFER   HostMemoryBuffer;

    ULONG   AsUlong;
} NVME_CDW13_FEATURES, *PNVME_CDW13_FEATURES;

typedef union {
    NVME_CDW14_FEATURE_HOST_MEMORY_BUFFER   HostMemoryBuffer;

    ULONG   AsUlong;
} NVME_CDW14_FEATURES, *PNVME_CDW14_FEATURES;

typedef union {
    NVME_CDW15_FEATURE_HOST_MEMORY_BUFFER   HostMemoryBuffer;

    ULONG   AsUlong;
} NVME_CDW15_FEATURES, *PNVME_CDW15_FEATURES;

//
// NVMe Maximum log size
//
#define NVME_MAX_LOG_SIZE               0x1000

//
// NVM Express Log Page Identifier values for NVME_ADMIN_COMMAND_GET_LOG_PAGE Command
//
typedef enum {

    NVME_LOG_PAGE_SUPPORTED_LOG_PAGES                       = 0x00,
    NVME_LOG_PAGE_ERROR_INFO                                = 0x01,
    NVME_LOG_PAGE_HEALTH_INFO                               = 0x02,
    NVME_LOG_PAGE_FIRMWARE_SLOT_INFO                        = 0x03,
    NVME_LOG_PAGE_CHANGED_NAMESPACE_LIST                    = 0x04,
    NVME_LOG_PAGE_COMMAND_EFFECTS                           = 0x05,
    NVME_LOG_PAGE_DEVICE_SELF_TEST                          = 0x06,
    NVME_LOG_PAGE_TELEMETRY_HOST_INITIATED                  = 0x07,
    NVME_LOG_PAGE_TELEMETRY_CTLR_INITIATED                  = 0x08,
    NVME_LOG_PAGE_ENDURANCE_GROUP_INFORMATION               = 0x09,
    NVME_LOG_PAGE_PREDICTABLE_LATENCY_NVM_SET               = 0x0A,
    NVME_LOG_PAGE_PREDICTABLE_LATENCY_EVENT_AGGREGATE       = 0x0B,
    NVME_LOG_PAGE_ASYMMETRIC_NAMESPACE_ACCESS               = 0x0C,
    NVME_LOG_PAGE_PERSISTENT_EVENT_LOG                      = 0x0D,
    NVME_LOG_PAGE_LBA_STATUS_INFORMATION                    = 0x0E,     // NVM Express NVM Command Set
    NVME_LOG_PAGE_ENDURANCE_GROUP_EVENT_AGGREGATE           = 0x0F,
    NVME_LOG_PAGE_MEDIA_UNIT_STATUS                         = 0x10,
    NVME_LOG_PAGE_SUPPORTED_CAPACITY_CONFIGURATION_LIST     = 0X11,
    NVME_LOG_PAGE_FEATURE_IDENTIFIERS_SUPPORTED_AND_EFFECTS = 0x12,
    NVME_LOG_PAGE_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS    = 0x13,
    NVME_LOG_PAGE_COMMAND_AND_FEATURE_LOCKDOWN              = 0x14,
    NVME_LOG_PAGE_BOOT_PARTITON                             = 0x15,
    NVME_LOG_PAGE_ROTATIONAL_MEDIA_INFORMATION              = 0x16,
    NVME_LOG_PAGE_DISCOVERY                                 = 0x70,
    NVME_LOG_PAGE_RESERVATION_NOTIFICATION                  = 0x80,
    NVME_LOG_PAGE_SANITIZE_STATUS                           = 0x81,
    NVME_LOG_PAGE_CHANGED_ZONE_LIST                         = 0xBF,     // NVM Express Zoned Namespace Command Set

} NVME_LOG_PAGES;

//
// Get LOG PAGE format which conforms to < 1.2.1 NVMe Specification
//
typedef union {

    struct {
        ULONG   LID         : 8;        // Log Page Identifier (LID)
        ULONG   Reserved0   : 8;
        ULONG   NUMD        : 12;       // Number of Dwords (NUMD)
        ULONG   Reserved1   : 4;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_GET_LOG_PAGE, *PNVME_CDW10_GET_LOG_PAGE;

//
// Get LOG PAGE format which conforms to 1.2.1 NVMe Specification
//
typedef union {

    struct {
        ULONG   LID         : 8;        // Log Page Identifier (LID)
        ULONG   Reserved0   : 8;
        ULONG   NUMDL       : 16;       // Number of Dwords Lower (NUMDL)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_GET_LOG_PAGE_V121, *PNVME_CDW10_GET_LOG_PAGE_V121;

//
// Get Log Page CDW10 format for NVM Express specification revisions 1.3a thru 1.4, inclusive.
//
typedef union {

    struct {
        ULONG   LID         : 8;        // Log Page Identifier (LID)
        ULONG   LSP         : 4;        // Log Specific Field (LSP)
        ULONG   Reserved0   : 3;
        ULONG   RAE         : 1;        // Retain Asynchronous Event (RAE)
        ULONG   NUMDL       : 16;       // Number of Lower Dwords (NUMDL)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_GET_LOG_PAGE_V13, *PNVME_CDW10_GET_LOG_PAGE_V13;

//
// Get Log Page CDW10 format for NVM Express Base Specification revision 2.0a and above.
//
typedef union {

    struct {
        ULONG   LID         : 8;        // Log Page Identifier (LID)
        ULONG   LSP         : 7;        // Log Specific Field (LSP)
        ULONG   RAE         : 1;        // Retain Asynchronous Event (RAE)
        ULONG   NUMDL       : 16;       // Number of Lower Dwords (NUMDL)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_GET_LOG_PAGE_V20, *PNVME_CDW10_GET_LOG_PAGE_V20;

//
// Defined values for bits 09:08 of Log Specific Field (LSP) in CDW10 for Get Log Page, Persistent Event Log (Log Identifier 0Dh)
// NVM Express Base Specification, revision >= 2.0a.
//

#define NVME_CDW10_LSP_ACTION_READ_LOG_DATA                                    0x0
#define NVME_CDW10_LSP_ACTION_ESTABLISH_CONTEXT_AND_READ_LOG_DATA              0x1
#define NVME_CDW10_LSP_ACTION_RELEASE_CONTEXT                                  0x2
#define NVME_CDW10_LSP_ACTION_ESTABLISH_CONTEXT_AND_READ_512_BYTES_OF_HEADER   0x3

typedef union {

    struct {
        ULONG   NUMDU                   : 16;       // Number of Upper Dwords (NUMDU)
        ULONG   LogSpecificIdentifier   : 16;       // Log Specific Identifier  (Reserved prior to NVM Express 1.4 specification)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_GET_LOG_PAGE, *PNVME_CDW11_GET_LOG_PAGE;

typedef union {

    ULONG   LPOL;                       // Log Page Offset Lower (LPOL)

    ULONG   AsUlong;

} NVME_CDW12_GET_LOG_PAGE, *PNVME_CDW12_GET_LOG_PAGE;

typedef union {

    ULONG   LPOU;                       // Log Page Offset Upper (LPOU)

    ULONG   AsUlong;

} NVME_CDW13_GET_LOG_PAGE, *PNVME_CDW13_GET_LOG_PAGE;

typedef union {

    struct {
        ULONG   UUIDIndex               : 7;       // UUID Index
        ULONG   Reserved                : 16;
        ULONG   OT                      : 1;       // Offset Type
        ULONG   CommandSetIdentifier    : 8;       // Command Set Identifier (CSI)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW14_GET_LOG_PAGE, *PNVME_CDW14_GET_LOG_PAGE;

typedef union {

    struct {
        ULONG   UUIDIndex               : 7;       // UUID Index
        ULONG   Reserved                : 16;
        ULONG   OffsetType              : 1;       // Offset Type (OT)
        ULONG   CommandSetIdentifier    : 8;       // Command Set Identifier (CSI)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW14_GET_LOG_PAGE_V20, *PNVME_CDW14_GET_LOG_PAGE_V20;

//
// Information of log: NVME_LOG_PAGE_SUPPORTED_LOG_PAGES.  Size: 1024 bytes
//

//
// LID Supported and Effects Data Structure - LID Specific Field for Log Page Identifier 0Dh (NVME_LOG_PAGE_PERSISTENT_EVENT_LOG)
//

typedef struct {

    USHORT     EstablishContextAndRead512BytesOfHeaderSupported :  1;
    USHORT     Reserved                                         : 15;

} NVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG, *PNVME_LID_SPECIFIC_PERSISTENT_EVENT_LOG;

//
// Get Log Page - LID Supported and Effects Data Structure (Log Identifier 00h)
//

typedef struct {

    ULONG      LSUPP       : 1;        // LID Supported
    ULONG      IOS         : 1;        // Index Offset Supported
    ULONG      Reserved    : 14;       // Note, 2.0a spec typo shows as bits 31:2
    ULONG      LIDSpecific : 16;       // LID Specific Field (bits 31:16)

} NVME_LID_SUPPORTED_AND_EFFECTS, *PNVME_LID_SUPPORTED_AND_EFFECTS;

#define NVME_NUM_LOG_PAGE_IDENTIFIERS  256
#define NVME_MAX_LOG_PAGE_IDENTIFIER   0xFF     // Highest legal Log Page Identifier value.

//
// Get Log Pages - Supported Log Pages - Log (Log Identifier 00h). Size: 1024 bytes.
//

typedef struct {

    NVME_LID_SUPPORTED_AND_EFFECTS     LogPageIdentifierSupported[NVME_NUM_LOG_PAGE_IDENTIFIERS];

} NVME_SUPPORTED_LOG_PAGES_LOG, *PNVME_SUPPORTED_LOG_PAGES_LOG;

//
// Information of log: NVME_LOG_PAGE_ERROR_INFO. Size: 64 bytes
//
typedef struct {

    ULONGLONG           ErrorCount;
    USHORT              SQID;           // Submission Queue ID
    USHORT              CMDID;          // Command ID
    NVME_COMMAND_STATUS Status;         // Status Field: This field indicates the Status Field for the command  that completed.  The Status
                                        // Field is located in bits 15:01, bit 00 corresponds to the Phase Tag posted for the command.

    struct {
        USHORT  Byte        : 8;        // Byte in command that contained the error.
        USHORT  Bit         : 3;        // Bit in command that contained the error.
        USHORT  Reserved    : 5;
    } ParameterErrorLocation;

    ULONGLONG           Lba;            // LBA: This field indicates the first LBA that experienced the error condition, if applicable.
    ULONG               NameSpace;      // Namespace: This field indicates the namespace that the error is associated with, if applicable.

    UCHAR               VendorInfoAvailable;    // Vendor Specific Information Available

    UCHAR               TRTYPE;         // NVMEOF_TRANSPORT_TYPE

    UCHAR               Reserved0[2];

    ULONGLONG           CommandSpecificInfo;    // This field contains command specific information. If used, the command definition specifies the information returned.

    USHORT              TransportTypeSpecificInfo; // This field contains the transport type specific error information.

    UCHAR               Reserved1[22];

} NVME_ERROR_INFO_LOG, *PNVME_ERROR_INFO_LOG;

//
// Information of log: NVME_LOG_PAGE_HEALTH_INFO. Size: 512 bytes
//
typedef struct {

    union {

        struct {
            UCHAR   AvailableSpaceLow   : 1;                    // If set to 1, then the available spare space has fallen below the threshold.
            UCHAR   TemperatureThreshold : 1;                   // If set to 1, then a temperature is above an over temperature threshold or below an under temperature threshold.
            UCHAR   ReliabilityDegraded : 1;                    // If set to 1, then the device reliability has been degraded due to significant media related  errors or any internal error that degrades device reliability.
            UCHAR   ReadOnly            : 1;                    // If set to 1, then the media has been placed in read only mode
            UCHAR   VolatileMemoryBackupDeviceFailed    : 1;    // If set to 1, then the volatile memory backup device has failed. This field is only valid if the controller has a volatile memory backup solution.
            UCHAR   Reserved                            : 3;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;

    } CriticalWarning;    // This field indicates critical warnings for the state of the  controller. Each bit corresponds to a critical warning type; multiple bits may be set.

    UCHAR   Temperature[2];                 // Temperature: Contains the temperature of the overall device (controller and NVM included) in units of Kelvin. If the temperature exceeds the temperature threshold, refer to section 5.12.1.4, then an asynchronous event completion may occur
    UCHAR   AvailableSpare;                 // Available Spare:  Contains a normalized percentage (0 to 100%) of the remaining spare capacity available
    UCHAR   AvailableSpareThreshold;        // Available Spare Threshold:  When the Available Spare falls below the threshold indicated in this field, an asynchronous event  completion may occur. The value is indicated as a normalized percentage (0 to 100%).
    UCHAR   PercentageUsed;                 // Percentage Used
    UCHAR   Reserved0[26];

    UCHAR   DataUnitRead[16];               // Data Units Read:  Contains the number of 512 byte data units the host has read from the controller; this value does not include metadata. This value is reported in thousands (i.e., a value of 1 corresponds to 1000 units of 512 bytes read)  and is rounded up.  When the LBA size is a value other than 512 bytes, the controller shall convert the amount of data read to 512 byte units. For the NVM command set, logical blocks read as part of Compare and Read operations shall be included in this value
    UCHAR   DataUnitWritten[16];            // Data Units Written: Contains the number of 512 byte data units the host has written to the controller; this value does not include metadata. This value is reported in thousands (i.e., a value of 1 corresponds to 1000 units of 512 bytes written)  and is rounded up.  When the LBA size is a value other than 512 bytes, the controller shall convert the amount of data written to 512 byte units. For the NVM command set, logical blocks written as part of Write operations shall be included in this value. Write Uncorrectable commands shall not impact this value.
    UCHAR   HostReadCommands[16];           // Host Read Commands:  Contains the number of read commands  completed by  the controller. For the NVM command set, this is the number of Compare and Read commands.
    UCHAR   HostWrittenCommands[16];        // Host Write Commands:  Contains the number of write commands  completed by  the controller. For the NVM command set, this is the number of Write commands.
    UCHAR   ControllerBusyTime[16];         // Controller Busy Time:  Contains the amount of time the controller is busy with I/O commands. The controller is busy when there is a command outstanding to an I/O Queue (specifically, a command was issued via an I/O Submission Queue Tail doorbell write and the corresponding  completion queue entry  has not been posted yet to the associated I/O Completion Queue). This value is reported in minutes.
    UCHAR   PowerCycle[16];                 // Power Cycles: Contains the number of power cycles.
    UCHAR   PowerOnHours[16];               // Power On Hours: Contains the number of power-on hours. This does not include time that the controller was powered and in a low power state condition.
    UCHAR   UnsafeShutdowns[16];            // Unsafe Shutdowns: Contains the number of unsafe shutdowns. This count is incremented when a shutdown notification (CC.SHN) is not received prior to loss of power.
    UCHAR   MediaErrors[16];                // Media Errors:  Contains the number of occurrences where the controller detected an unrecovered data integrity error. Errors such as uncorrectable ECC, CRC checksum failure, or LBA tag mismatch are included in this field.
    UCHAR   ErrorInfoLogEntryCount[16];     // Number of Error Information Log Entries:  Contains the number of Error Information log entries over the life of the controller
    ULONG   WarningCompositeTemperatureTime;     // Warning Composite Temperature Time: Contains the amount of time in minutes that the controller is operational and the Composite Temperature is greater than or equal to the Warning Composite Temperature Threshold (WCTEMP) field and less than the Critical Composite Temperature Threshold (CCTEMP) field in the Identify Controller data structure
    ULONG   CriticalCompositeTemperatureTime;    // Critical Composite Temperature Time: Contains the amount of time in minutes that the controller is operational and the Composite Temperature is greater the Critical Composite Temperature Threshold (CCTEMP) field in the Identify Controller data structure
    USHORT  TemperatureSensor1;          // Contains the current temperature reported by temperature sensor 1.
    USHORT  TemperatureSensor2;          // Contains the current temperature reported by temperature sensor 2.
    USHORT  TemperatureSensor3;          // Contains the current temperature reported by temperature sensor 3.
    USHORT  TemperatureSensor4;          // Contains the current temperature reported by temperature sensor 4.
    USHORT  TemperatureSensor5;          // Contains the current temperature reported by temperature sensor 5.
    USHORT  TemperatureSensor6;          // Contains the current temperature reported by temperature sensor 6.
    USHORT  TemperatureSensor7;          // Contains the current temperature reported by temperature sensor 7.
    USHORT  TemperatureSensor8;          // Contains the current temperature reported by temperature sensor 8.
    UCHAR   Reserved1[296];

} NVME_HEALTH_INFO_LOG, *PNVME_HEALTH_INFO_LOG;

//
// "Telemetry Host-Initiated Log" structure definition.
//

#define NVME_TELEMETRY_DATA_BLOCK_SIZE                  0x200 // All NVMe Telemetry Data Blocks are 512 bytes in size.

typedef struct _NVME_TELEMETRY_HOST_INITIATED_LOG {

    UCHAR   LogIdentifier;                      // Byte 0
    UCHAR   Reserved0[4];                       // Bytes 1-4
    UCHAR   OrganizationID[3];                  // Bytes 5-7 - IEEE OUI Identifier
    USHORT  Area1LastBlock;                     // Bytes 8-9
    USHORT  Area2LastBlock;                     // Bytes 10-11
    USHORT  Area3LastBlock;                     // Bytes 12-13
    UCHAR   Reserved1[2];                       // Bytes 14-15
    ULONG   Area4LastBlock;                     // Bytes 16-19
    UCHAR   Reserved2[361];                     // Bytes 20-380
    UCHAR   HostInitiatedDataGenerationNumber;  // Byte 381
    UCHAR   ControllerInitiatedDataAvailable;   // Byte 382
    UCHAR   ControllerInitiatedDataGenerationNumber; // Byte 383
    UCHAR   ReasonIdentifier[128];              // Bytes 384-511

} NVME_TELEMETRY_HOST_INITIATED_LOG, *PNVME_TELEMETRY_HOST_INITIATED_LOG;

//
// "Telemetry Controller-Initiated Log" structure definition.
//
typedef struct _NVME_TELEMETRY_CONTROLLER_INITIATED_LOG {

    UCHAR   LogIdentifier;                      // Byte 0
    UCHAR   Reserved0[4];                       // Bytes 1-4
    UCHAR   OrganizationID[3];                  // Bytes 5-7 - IEEE OUI Identifier
    USHORT  Area1LastBlock;                     // Bytes 8-9
    USHORT  Area2LastBlock;                     // Bytes 10-11
    USHORT  Area3LastBlock;                     // Bytes 12-13
    UCHAR   Reserved1[2];                       // Bytes 14-15
    ULONG   Area4LastBlock;                     // Bytes 16-19
    UCHAR   Reserved2[362];                     // Bytes 20-381
    UCHAR   ControllerInitiatedDataAvailable;   // Byte 382
    UCHAR   ControllerInitiatedDataGenerationNumber; // Byte 383
    UCHAR   ReasonIdentifier[128];              // Bytes 384-511

} NVME_TELEMETRY_CONTROLLER_INITIATED_LOG, *PNVME_TELEMETRY_CONTROLLER_INITIATED_LOG;

//
// Information of log: NVME_LOG_PAGE_FIRMWARE_SLOT_INFO. Size: 512 bytes
//
typedef struct {

    struct {
        UCHAR   ActiveSlot          : 3;        // Bits 2:0 indicates the firmware slot that contains the actively running firmware revision.
        UCHAR   Reserved0           : 1;
        UCHAR   PendingActivateSlot : 3;        // Bits 6:4 indicates the firmware slot that is going to be activated at the next controller reset.
        UCHAR   Reserved1           : 1;
    } AFI;    // Active Firmware Info (AFI)

    UCHAR       Reserved0[7];

    ULONGLONG   FRS[7];            // Firmware Revision for Slot 1 - 7(FRS1 - FRS7):  Contains the revision of the firmware downloaded to firmware slot 1 - 7.

    UCHAR       Reserved1[448];

} NVME_FIRMWARE_SLOT_INFO_LOG, *PNVME_FIRMWARE_SLOT_INFO_LOG;

//
// Information of log: NVME_LOG_PAGE_CHANGED_NAMESPACE_LIST. Size: 4096 bytes
//
typedef struct {

    ULONG   NSID[1024];                        // List of Namespace ID upto 1024 entries

} NVME_CHANGED_NAMESPACE_LIST_LOG, *PNVME_CHANGED_NAMESPACE_LIST_LOG;

//
// Information of log: NVME_LOG_PAGE_CHANGED_ZONE_LIST. Size: 4096 bytes
//
typedef struct {

    USHORT  ZoneIdentifiersCount;               // Number of Zone Identifiers
    UCHAR   Reserved[6];

    ULONGLONG   ZoneIdentifier[511];            // List of Zone Identifiers upto 511 entries. Identifier contains Zone Start Logical Block Address(ZSLBA)

} NVME_CHANGED_ZONE_LIST_LOG, *PNVME_CHANGED_ZONE_LIST_LOG;


//
// Information of log: NVME_LOG_PAGE_COMMAND_EFFECTS. Size: 4096 bytes
//
typedef enum {

    NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_NONE                     = 0,
    NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_SINGLE_PER_NAMESPACE     = 1,
    NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMIT_SINGLE_PER_CONTROLLER    = 2,

} NVME_COMMAND_EFFECT_SBUMISSION_EXECUTION_LIMITS;

typedef union {

    struct {
        //LSB
        ULONG   CSUPP                   : 1;        // Command Supported (CSUPP)
        ULONG   LBCC                    : 1;        // Logical Block Content Change (LBCC)
        ULONG   NCC                     : 1;        // Namespace Capability Change (NCC)
        ULONG   NIC                     : 1;        // Namespace Inventory Change (NIC)
        ULONG   CCC                     : 1;        // Controller Capability Change (CCC)
        ULONG   Reserved                : 11;
        ULONG   CSE                     : 3;        // Command Submission and Execution (CSE)
        ULONG   UUIDSelectionSupported  : 1;        // UUID Selection Supported
        ULONG   CSPNamespace            : 1;        // Namespace Scope
        ULONG   CSPController           : 1;        // Controller Scope
        ULONG   CSPNVMSet               : 1;        // NVM Set Scope
        ULONG   CSPEnduranceGroup       : 1;        // Endurance Group Scope
        ULONG   CSPDomain               : 1;        // Domain Scope
        ULONG   CSPNVMSubsystem         : 1;        // NVM Subsystem Scope
        ULONG   CSPReserved             : 6;
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_COMMAND_EFFECTS_DATA, *PNVME_COMMAND_EFFECTS_DATA;


typedef struct {

    NVME_COMMAND_EFFECTS_DATA   ACS[256];       // Admin Command Supported
    NVME_COMMAND_EFFECTS_DATA   IOCS[256];      // I/O Command Supported

    UCHAR                       Reserved[2048];

} NVME_COMMAND_EFFECTS_LOG, *PNVME_COMMAND_EFFECTS_LOG;

#define NVME_VENDOR_SPECIFIC_ADMIN_COMMAND_MIN_OPCODE                 0xC0
#define NVME_VENDOR_SPECIFIC_ADMIN_COMMAND_MAX_OPCODE                 0xFF

#define NVME_VENDOR_SPECIFIC_NVM_COMMAND_MIN_OPCODE                   0x80
#define NVME_VENDOR_SPECIFIC_NVM_COMMAND_MAX_OPCODE                   0xFF


#pragma pack(push, 1)
typedef struct {

    struct {
        UCHAR   Result              : 4;        // Result of Device Self-Test operation of this particular result data
        UCHAR   CodeValue           : 4;        // Self-Test code value that was specified in command
    } Status;

    UCHAR       SegmentNumber;                  // Indicates the first segment that failure occured

    struct {
        UCHAR   NSIDValid           : 1;        // If set to 1, the contents of Namespace Identifier field is valid
        UCHAR   FLBAValid           : 1;        // If set to 1, the contents of Failing LBA field is valid
        UCHAR   SCTValid            : 1;        // If set to 1, the contents of Status Code Type field is valid
        UCHAR   SCValid             : 1;        // If set to 1, the contents of Status Code field is valid
        UCHAR   Reserved            : 4;
    } ValidDiagnostics;

    UCHAR       Reserved;

    ULONGLONG   POH;                            // Power On Hours, when test operation was completed/aborted
    ULONG       NSID;                           // Namespace Identifier. Only valid if NSIDValid is set
    ULONGLONG   FailingLBA;                     // Failed LBA which caused test to fail. Only valid if FLBAValid is set

    struct {
        UCHAR   AdditionalInfo      : 3;        // Additional information related to errors/conditions. Only valid if SCTValid is set
        UCHAR   Reserved            : 5;
    } StatusCodeType;

    UCHAR       StatusCode;                     // Additional information related to errors/conditons. Only valid if SCValid is set
    USHORT      VendorSpecific;

} NVME_DEVICE_SELF_TEST_RESULT_DATA, *PNVME_DEVICE_SELF_TEST_RESULT_DATA;

//
// Information of log: NVME_LOG_PAGE_DEVICE_SELF_TEST. Size: 564 bytes
//
typedef struct {

     struct {
        UCHAR   Status              : 4;        // Status of current Device Self-Test operation
        UCHAR   Reserved            : 4;
     } CurrentOperation;

     struct {
        UCHAR   CompletePercent     : 7;        // Percentage of completion of Device Self-Test operation. Valid if Status field is non-zero.
        UCHAR   Reserved            : 1;
     } CurrentCompletion;

     UCHAR      Reserved[2];

     NVME_DEVICE_SELF_TEST_RESULT_DATA       ResultData[20];    // Last 20 Self-Test Result Data, latest to oldest available in sorted order

} NVME_DEVICE_SELF_TEST_LOG, *PNVME_DEVICE_SELF_TEST_LOG;

//
// Information of log: NVME_LOG_PAGE_ENDURANCE_GROUP_INFORMATION. Size: 512 bytes
//
typedef struct {

    ULONG       Reserved0;
    UCHAR       AvailableSpareThreshold;    // Available spare indicated as normalized percentage (0-100)
    UCHAR       PercentageUsed;             // Vendor specific estimate of percentage of life used for the NVM set(s) of Endurance Group (Billion Unit)

    UCHAR       Reserved1[26];
    UCHAR       EnduranceEstimate[16];      // Estimate of total number of data bytes written to NVM set(s) of Endurance Group (Billion Unit)
    UCHAR       DataUnitsRead[16];          // Total number of data bytes read from NVM set(s) of Endurance Group (Billion Unit)
    UCHAR       DataUnitsWritten[16];       // Total number of data bytes written to NVM sets(s) of Endurance Group (Billion Unit)
                                            // Includes only host writes

    UCHAR       MediaUnitsWritten[16];      // Total number of data bytes written to NVM sets(s) of Endurance Group (Billion Unit)
                                            // Includes both host and controller writes.

    UCHAR       Reserved2[416];

} NVME_ENDURANCE_GROUP_LOG, *PNVME_ENDURANCE_GROUP_LOG;

//
// Information of log: NVME_LOG_PAGE_PERSISTENT_EVENT_LOG. Header Size: 512 bytes
//
typedef struct {

    UCHAR     LogIdentifier;                      // Byte 0      - Shall be set to 0x0D
    UCHAR     Reserved0[3];                       // Bytes 1-3
    ULONG     TotalNumberOfEvents;                // Bytes 4-7   - Contains the number of event entries in the log.
    ULONGLONG TotalLogLength;                     // Bytes 8-15  - Contains the total number of bytes of persistent event log page data available, including the header.
    UCHAR     LogRevision;                        // Bytes 16    - Contains a number indicating the revision of the Get Log Page data structure that this log page data complies with.
    UCHAR     Reserved1;                          // Bytes 17
    USHORT    LogHeaderLength;                    // Bytes 18-19 - Contains the length in bytes of the log header information that follows. The total length of the log header in bytes is the value in this field plus 20.
    ULONGLONG Timestamp;                          // Bytes 20-27
    UCHAR     PowerOnHours[16];                   // Bytes 28-43 - Indicates the number of power-on hours at the time the Persistent Event log was retrieved.
    ULONGLONG PowerCycleCount;                    // Bytes 44-51 - Contains the number of power cycles for the controller.
    USHORT    PciVendorId;                        // Bytes 52-53 - Same value as reported in the Identify Controller data PCI Vendor ID field.
    USHORT    PciSubsystemVendorId;               // Bytes 54-55 - Same value as reported in the Identify Controller data PCI Subsystem Vendor ID field.
    UCHAR     SerialNumber[20];                   // Bytes 56-75 - Same value as reported in the Identify Controller data Serial Number field.
    UCHAR     ModelNumber[40];                    // Bytes 76-115 - Same value as reported in the Identify Controller data Model Number field.
    UCHAR     NVMSubsystemNVMeQualifiedName[256]; // Bytes 116-371 - Same value as reported in the Identify Controller data.
    UCHAR     Reserved[108];                      // Bytes 372-479
    UCHAR     SupportedEventsBitmap[32];          // Bytes 480-511 - Contains a bitmap indicating support for the persistent event log events.

} NVME_PERSISTENT_EVENT_LOG_HEADER, *PNVME_PERSISTENT_EVENT_LOG_HEADER;

typedef struct {

    UCHAR     EventType;                          // Byte 0      - Indicates the event type for this entry.
    UCHAR     EventTypeRevision;                  // Byte 1      - Contains a number indicating the revision of the event data.
    UCHAR     EventHeaderLength;                  // Byte 2      - Contains the length in bytes of the event header information that follows.
    UCHAR     Reserved0;                          // Byte 3
    USHORT    ControllerIdentifier;               // Bytes 4-5   - Contains the NVM subsystem unique controller identifier for the controller that created this event.
    ULONGLONG EventTimestamp;                     // Bytes 6-13
    UCHAR     Reserved1[6];                       // Bytes 14-19
    USHORT    VendorSpecificInformationLength;    // Bytes 20-21 - Indicates the length in bytes of the Vendor Specific Information.
    USHORT    EventLength;                        // Bytes 22-23 - Indicates the length in bytes of the Vendor Specific Information.

} NVME_PERSISTENT_EVENT_LOG_EVENT_HEADER, *PNVME_PERSISTENT_EVENT_LOG_EVENT_HEADER;

typedef enum {

    NVME_PERSISTENT_EVENT_TYPE_RESERVED0                    = 0x00,

    NVME_PERSISTENT_EVENT_TYPE_SMART_HEALTH_LOG_SNAPSHOT    = 0x01,
    NVME_PERSISTENT_EVENT_TYPE_FIRMWARE_COMMIT              = 0x02,
    NVME_PERSISTENT_EVENT_TYPE_TIMESTAMP_CHANGE             = 0x03,
    NVME_PERSISTENT_EVENT_TYPE_POWER_ON_OR_RESET            = 0x04,
    NVME_PERSISTENT_EVENT_TYPE_NVM_SUBSYSTEM_HARDWARE_ERROR = 0x05,
    NVME_PERSISTENT_EVENT_TYPE_CHANGE_NAMESPACE             = 0x06,
    NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_START             = 0x07,
    NVME_PERSISTENT_EVENT_TYPE_FORMAT_NVM_COMPLETION        = 0x08,
    NVME_PERSISTENT_EVENT_TYPE_SANITIZE_START               = 0x09,
    NVME_PERSISTENT_EVENT_TYPE_SANITIZE_COMPLETION          = 0x0A,
    NVME_PERSISTENT_EVENT_TYPE_SET_FEATURE                  = 0x0B,
    NVME_PERSISTENT_EVENT_TYPE_TELEMETRY_LOG_CREATED        = 0x0C,
    NVME_PERSISTENT_EVENT_TYPE_THERMAL_EXCURSION            = 0x0D,

    NVME_PERSISTENT_EVENT_TYPE_RESERVED1_BEGIN              = 0x0E,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED1_END                = 0xDD,

    NVME_PERSISTENT_EVENT_TYPE_VENDOR_SPECIFIC_EVENT        = 0xDE,
    NVME_PERSISTENT_EVENT_TYPE_TCG_DEFINED                  = 0xDF,

    NVME_PERSISTENT_EVENT_TYPE_RESERVED2_BEGIN              = 0xE0,
    NVME_PERSISTENT_EVENT_TYPE_RESERVED2_END                = 0xFF,

    NVME_PERSISTENT_EVENT_TYPE_MAX                          = 0xFF,

} NVME_PERSISTENT_EVENT_LOG_EVENT_TYPES;

#pragma pack(pop)

//
// Information of log: NVME_LOG_PAGE_FEATURE_IDENTIFIERS_SUPPORTED_AND_EFFECTS (Log Identifier: 12h). Size: 1024 bytes
// Note: This log page describes the FIDs that are supported on the interface to which the Get Log Page command was
// submitted and the effects of those features on the state of the NVM subsystem.
//

typedef struct {

    ULONG  FSUPP                    :  1;       // FID Supported
    ULONG  UDCC                     :  1;       // User Data Content Change
    ULONG  NCC                      :  1;       // Namespace Capability Change
    ULONG  NIC                      :  1;       // Namespace Inventory Change
    ULONG  CCC                      :  1;       // Controller Capability Change
    ULONG  Reserved                 : 14;       // Reserved
    ULONG  UUIDSelectionSupported   :  1;       // UUID Selection Supported
    ULONG  FSPNamespace             :  1;       // Namespace Scope
    ULONG  FSPController            :  1;       // Controller Scope
    ULONG  FSPNVMSet                :  1;       // NVM Set Scope
    ULONG  FSPEnduranceGroup        :  1;       // Endurance Group Scope
    ULONG  FSPDomain                :  1;       // Domain Scope
    ULONG  FSPNVMSubsystem          :  1;       // NVM Subsystem Scope
    ULONG  FSPReserved              :  6;

} NVME_FID_SUPPORTED_AND_EFFECTS, *PNVME_FID_SUPPORTED_AND_EFFECTS;

#define NVME_NUM_FID_SUPPORTED      256

typedef struct {

    NVME_FID_SUPPORTED_AND_EFFECTS     FeatureIdentifierSupported[NVME_NUM_FID_SUPPORTED];

} NVME_FEATURE_IDENTIFIERS_EFFECTS_LOG, *PNVME_FEATURE_IDENTIFIERS_EFFECTS_LOG;

#define NVME_VENDOR_SPECIFIC_FEATURE_MIN_IDENTIFIER                 0xC0
#define NVME_VENDOR_SPECIFIC_FEATURE_MAX_IDENTIFIER                 0xFF


//
// Log Page: NVME_LOG_PAGE_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS (Log Identifier: 13h). Size: 4096 bytes
//

typedef struct {

    ULONG  CSUPP                :  1;       // Command Supported
    ULONG  UDCC                 :  1;       // User Data Content Change
    ULONG  NCC                  :  1;       // Namespace Capability Change
    ULONG  NIC                  :  1;       // Namespace Inventory Change
    ULONG  CCC                  :  1;       // Controller Capability Change
    ULONG  Reserved             : 15;       // Reserved
    ULONG  CSPNamespace         :  1;       // Namespace Scope
    ULONG  CSPController        :  1;       // Controller Scope
    ULONG  CSPNVMSet            :  1;       // NVM Set Scope
    ULONG  CSPEnduranceGroup    :  1;       // Endurance Group Scope
    ULONG  CSPDomain            :  1;       // Domain Scope
    ULONG  CSPNVMSubsystem      :  1;       // NVM Subsystem Scope
    ULONG  CSPReserved          :  6;

} NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS, *PNVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS;

#define NVME_NUM_NVME_MI_COMMANDS_SUPPORTED         256

typedef struct {

    NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS     ManagementInterfaceCommandSupported[NVME_NUM_NVME_MI_COMMANDS_SUPPORTED];
    UCHAR                                           Reserved[3 * 1024];

} NVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG, *PNVME_NVME_MI_COMMANDS_SUPPORTED_AND_EFFECTS_LOG;

//
// Information of log: NVME_LOG_PAGE_RESERVATION_NOTIFICATION. Size: 64 bytes
//
typedef enum {

    NVME_RESERVATION_NOTIFICATION_TYPE_EMPTY_LOG_PAGE           = 0,
    NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_PREEMPTED   = 1,
    NVME_RESERVATION_NOTIFICATION_TYPE_REGISTRATION_RELEASED    = 2,
    NVME_RESERVATION_NOTIFICATION_TYPE_RESERVATION_PREEPMPTED   = 3,

} NVME_RESERVATION_NOTIFICATION_TYPES;

typedef struct {

    ULONGLONG   LogPageCount;           // Log Page Count
    UCHAR       LogPageType;            // Reservation Notification Log Page Type.
    UCHAR       AvailableLogPageCount;  // Number of Available Log Pages
    UCHAR       Reserved0[2];
    ULONG       NameSpaceId;            // Namespace ID
    UCHAR       Reserved1[48];

} NVME_RESERVATION_NOTIFICATION_LOG, *PNVME_RESERVATION_NOTIFICATION_LOG;

//
// Information of log: NVME_SANITIZE_STATUS_LOG. Size: 512 bytes
//
typedef enum {

    //
    // The NVM subsystem has never been sanitized.
    //
    NVME_SANITIZE_OPERATION_NONE                                = 0,

    //
    // The most recent sanitize operation completed successfully including
    // any additional media modification.
    //
    NVME_SANITIZE_OPERATION_SUCCEEDED                           = 1,

    //
    // A sanitize operation is currently in progress.
    //
    NVME_SANITIZE_OPERATION_IN_PROGRESS                         = 2,

    //
    // The most recent sanitize operation failed.
    //
    NVME_SANITIZE_OPERATION_FAILED                              = 3,

    //
    // The most recent sanitize operation for which No-Deallocate After Sanitize was Requested
    // has completed successfully with deallocation of all LBAs.
    //
    NVME_SANITIZE_OPERATION_SUCCEEDED_WITH_FORCED_DEALLOCATION  = 4

} NVME_SANITIZE_OPERATION_STATUS, *PNVME_SANITIZE_OPERATION_STATUS;

typedef struct {

    //
    // This contains the status of the most recent sanitize operation.
    // The value of this field is defined in enum NVME_SANITIZE_OPERATION_STATUS.
    //
    USHORT MostRecentSanitizeOperationStatus    : 3;

    //
    // This contains the number of completed passes if the most recent sanitize operation
    // was an Overwrite.
    //
    USHORT NumberCompletedPassesOfOverwrite     : 4;

    //
    // If set to 1, then no namespace logical block in the NVM subsystem has been written to
    // and no Persistent Memory Region in the NVM subsystem has been enabled since manufactured
    // or most recent successfully sanitized operation.
    //
    // If set to 0, then a namespace logical block in the NVM subsystem has been written to
    // or a Persistent Memory Region in the NVM subsystem has been enabled since manufactured
    // or most recent successfully sanitized operation.
    //
    USHORT GlobalDataErased                     : 1;

    USHORT Reserved                             : 8;

} NVME_SANITIZE_STATUS, *PNVME_SANITIZE_STATUS;

typedef struct {

    //
    // Sanitize Progress (SPROG)
    // This field indicates the fraction complete of the sanitize operation. The value is a numerator
    // of the fraction complete that has 65536 (10000h) as its denominator. This value shall be set to
    // FFFFh if bits 2:0 of the SSTAT field are not set to 10b.
    //
    USHORT                  SPROG;

    //
    // Sanitize Status (SSTAT)
    // This field indicates the status associated with the most recent sanitize operation.
    //
    NVME_SANITIZE_STATUS    SSTAT;

    //
    // Sanitize Command Dword 10 Information (SCDW10)
    // This field contains the value of the Command Dword 10 field of the Sanitize command that started
    // the sanitize operation whose status is reported in the SSTAT field.
    //
    ULONG                   SCDW10;

    //
    // These fields below indicates the number of seconds required to complete the sanitize operation
    // of Overwrite/Block Erase/Crypto Erase methods when the No-Deallocate Modifies Media After Sanitize
    // field is not set to 10b. A value of 0 indicates that the sanitize operation is expected to be
    // completed in the background when the Sanitize command that started that operation is completed.
    // A value of FFFFFFFFh indicates that no time period is reported.
    //
    ULONG                   EstimatedTimeForOverwrite;
    ULONG                   EstimatedTimeForBlockErase;
    ULONG                   EstimatedTimeForCryptoErase;

    //
    // These fields below indicates the number of seconds required to complete the sanitize operation
    // of Overwrite/Block Erase/CryptoErase methods and the associated additional media modification
    // after the sanitize operation. A value of 0 indicates that the sanitize operation is expected
    // to be completed in the background when the Sanitize command that started that operation is completed.
    // A value of FFFFFFFFh indicates that no time period is reported.
    //
    ULONG                   EstimatedTimeForOverwriteWithNoDeallocateMediaModification;
    ULONG                   EstimatedTimeForBlockEraseWithNoDeallocateMediaModification;
    ULONG                   EstimatedTimeForCryptoEraseWithNoDeallocateMediaModification;

    UCHAR                   Reserved[480];

} NVME_SANITIZE_STATUS_LOG, *PNVME_SANITIZE_STATUS_LOG;


//
// Parameters for FIRMWARE IMAGE DOWNLOAD Command
//
typedef struct {

    ULONG   NUMD;                               // Number of Dwords (NUMD)

} NVME_CDW10_FIRMWARE_DOWNLOAD, *PNVME_CDW10_FIRMWARE_DOWNLOAD;

typedef struct {

    ULONG   OFST;                               // Offset (OFST)

} NVME_CDW11_FIRMWARE_DOWNLOAD, *PNVME_CDW11_FIRMWARE_DOWNLOAD;

//
// Parameters for FIRMWARE ACTIVATE/COMMIT Commands
//
typedef enum {


    //
    // Downloaded image replaces the existing image, if any, in the specified Firmware Slot.
    // The newly placed image is not activated.
    //
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT                                       = 0,

    //
    // Downloaded image replaces the existing image, if any, in the specified Firmware Slot.
    // The newly placed image is activated at the next Controller Level Reset.
    //
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE                          = 1,

    //
    // The existing image in the specified Firmware Slot is activated at the next Controller
    // Level Reset
    //
    NVME_FIRMWARE_ACTIVATE_ACTION_ACTIVATE                                               = 2,

    //
    // Downloaded image replaces the existing image, if any, in the specified Firmware Slot
    // and is then activated immediately. If there is not a newly downloaded image, then the
    // existing image in the specified firmware slot is activated immediately.
    //
    NVME_FIRMWARE_ACTIVATE_ACTION_DOWNLOAD_TO_SLOT_AND_ACTIVATE_IMMEDIATE                = 3,

} NVME_FIRMWARE_ACTIVATE_ACTIONS;

typedef union {

    struct {
        ULONG   FS          : 3;            // Firmware Slot (FS)
        ULONG   AA          : 2;            // Activate Action (AA)
        ULONG   Reserved    : 27;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_FIRMWARE_ACTIVATE, *PNVME_CDW10_FIRMWARE_ACTIVATE;

//
// Parameters for FORMAT NVM Commands
//
typedef enum {

    NVME_PROTECTION_INFORMATION_NOT_ENABLED                 = 0,
    NVME_PROTECTION_INFORMATION_TYPE1                       = 1,
    NVME_PROTECTION_INFORMATION_TYPE2                       = 2,
    NVME_PROTECTION_INFORMATION_TYPE3                       = 3,

} NVME_PROTECTION_INFORMATION_TYPES;

typedef enum {

    NVME_SECURE_ERASE_NONE                                  = 0,
    NVME_SECURE_ERASE_USER_DATA                             = 1,
    NVME_SECURE_ERASE_CRYPTOGRAPHIC                         = 2,

} NVME_SECURE_ERASE_SETTINGS;

typedef union {

    struct {
        ULONG   LBAF        : 4;                // LBA Format (LBAF)
        ULONG   MS          : 1;                // Metadata Settings (MS)
        ULONG   PI          : 3;                // Protection Information (PI)
        ULONG   PIL         : 1;                // Protection Information Location (PIL)
        ULONG   SES         : 3;                // Secure Erase Settings (SES)
        ULONG   ZF          : 2;                // Zone Format (ZF)

        ULONG   Reserved    : 18;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_FORMAT_NVM, *PNVME_CDW10_FORMAT_NVM;

typedef enum {

    //
    // Additional media modification after sanitize is not defined.
    //
    NVME_MEDIA_ADDITIONALLY_MODIFIED_AFTER_SANITIZE_NOT_DEFINED = 0,

    //
    // Media is not additionally modified after sanitize completes successfully.
    //
    NVME_MEDIA_NOT_ADDITIONALLY_MODIFIED_AFTER_SANITIZE         = 1,

    //
    // Media is additionally modified after sanitize completes sucessfully. The Sanitize Operation Completed event
    // does not occur until the additional media modification associated with this field has completed.
    //
    NVME_MEDIA_ADDITIONALLY_MOFIDIED_AFTER_SANITIZE             = 2

} NVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE, *PNVME_NO_DEALLOCATE_MODIFIES_MEDIA_AFTER_SANITIZE;

//
// Parameters for Sanitize.
//

typedef enum {

    NVME_SANITIZE_ACTION_RESERVED                       = 0,
    NVME_SANITIZE_ACTION_EXIT_FAILURE_MODE              = 1,
    NVME_SANITIZE_ACTION_START_BLOCK_ERASE_SANITIZE     = 2,
    NVME_SANITIZE_ACTION_START_OVERWRITE_SANITIZE       = 3,
    NVME_SANITIZE_ACTION_START_CRYPTO_ERASE_SANITIZE    = 4

} NVME_SANITIZE_ACTION, *PNVME_SANITIZE_ACTION;

typedef union {

    struct {

        //
        // Sanitize Action (SANACT)
        // The value of this field is defined in enum NVME_SANTIZE_ACTION.
        //
        ULONG   SANACT      : 3;    // Sanitize Action (SANACT)

        //
        // Allow Unrestricted Sanitize Exit (AUSE)
        // This bit is ignored if Sanitize Action is in Exit Failure Mode (001b).
        //
        ULONG   AUSE        : 1;    // Allow Unrestricted Sanitize Exit (AUSE)

        //
        // Overwrite Pass Count (OWPASS)
        // This field specifies the number of overwrite passes using the data from Overwrite Pattern.
        // A value of 0h specified 16 overwrite passes. This is ignored unless Sanitize Action is Overwrite (011b).
        //
        ULONG   OWPASS      : 4;    // Overwrite Pass Count (OWPASS)

        //
        // Overwrite Invert Pattern Between Passes (OIPBP)
        // This field indicates if Overwrite Pattern shall be inverted between passes.
        // This is ignored unless Sanitize Action is Overwrite (011b).
        //
        ULONG   OIPBP       : 1;    // Overwrite Invert Pattern Between Passes (OIPBP)

        //
        // No Deallocate After Sanitize
        // If set to 1 and No-Deallocate Inhibited bit is 0,
        //     controller shall not deallocate any logical blocks after sanitize completed successfully.
        // If set to 1 and No-Deallocate Inhibited bit is 1, or if set to 0,
        //     controller shall deallocate logical blocks after sanitize completed successfully.
        // This bit is ignored if Sanitize Action is Exit Failure Mode (001b).
        //
        ULONG   NDAS        : 1;    // No Deallocate After Sanitize

        ULONG   Reserved    : 22;

    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW10_SANITIZE, *PNVME_CDW10_SANITIZE;

typedef union {

    struct {

        //
        // Overwrite Pattern
        // This field is ignored unless the Sanitize Action field in Command Dword 10 is set to 011b (Overwrite).
        // This field specifies a 32-bit pattern that is used for the Overwrite sanitize operation.
        //
        ULONG OVRPAT;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW11_SANITIZE;

//
// Parameters for RESERVATION Commands
//
typedef enum {

    NVME_RESERVATION_TYPE_RESERVED                              = 0,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE                       = 1,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS                      = 2,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_REGISTRANTS_ONLY      = 3,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_REGISTRANTS_ONLY     = 4,
    NVME_RESERVATION_TYPE_WRITE_EXCLUSIVE_ALL_REGISTRANTS       = 5,
    NVME_RESERVATION_TYPE_EXCLUSIVE_ACCESS_ALL_REGISTRANTS      = 6,

} NVME_RESERVATION_TYPES;

//
// Parameters for RESERVATION ACQUIRE Commands
//
typedef enum {

    NVME_RESERVATION_ACQUIRE_ACTION_ACQUIRE             = 0,
    NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT             = 1,
    NVME_RESERVATION_ACQUIRE_ACTION_PREEMPT_AND_ABORT   = 2,

} NVME_RESERVATION_ACQUIRE_ACTIONS;

typedef struct {

    ULONG PTPL          : 1;        // Persist Through Power Loss (PTPL)
    ULONG Reserved      : 31;

} NVME_CDW0_RESERVATION_PERSISTENCE, *PNVME_CDW0_RESERVATION_PERSISTENCE;

typedef union {

    struct {
        ULONG RACQA         : 3;    // Reservation Acquire Action (RACQA)
        ULONG IEKEY         : 1;    // Ignore Existing Key (IEKEY)
        ULONG Reserved      : 4;
        ULONG RTYPE         : 8;    // Reservation Type (RTYPE)

        ULONG Reserved1     : 16;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW10_RESERVATION_ACQUIRE, *PNVME_CDW10_RESERVATION_ACQUIRE;


//
// Reservation Acquire Data Structure
//
typedef struct {

    ULONGLONG CRKEY;    // Current Reservation Key (CRKEY)

    //
    // If the Reseravation Acquire Action is set to 001b (Preempt) or 010b (Preempt and Abort),
    // then this field specifies the reservation key to be unregistered from the namespace.
    // For all other Reservation Acquire Action values, this field is reserved.
    //
    ULONGLONG PRKEY;    // Preempt Reservation Key (PRKEY)

} NVME_RESERVATION_ACQUIRE_DATA_STRUCTURE, *PNVME_RESERVATION_ACQUIRE_DATA_STRUCTURE;

//
// Parameters for RESERVATION REGISTER Commands
//
typedef enum {

    NVME_RESERVATION_REGISTER_ACTION_REGISTER       = 0,
    NVME_RESERVATION_REGISTER_ACTION_UNREGISTER     = 1,
    NVME_RESERVATION_REGISTER_ACTION_REPLACE        = 2,

} NVME_RESERVATION_REGISTER_ACTIONS;

typedef enum {

    NVME_RESERVATION_REGISTER_PTPL_STATE_NO_CHANGE  = 0,
    NVME_RESERVATION_REGISTER_PTPL_STATE_RESERVED   = 1,
    NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_0   = 2,    // Reservations are released and registrants are cleared on a power on.
    NVME_RESERVATION_REGISTER_PTPL_STATE_SET_TO_1   = 3,    // Reservations and registrants persist across a power loss.

} NVME_RESERVATION_REGISTER_PTPL_STATE_CHANGES;

typedef union {

    struct {
        ULONG RREGA         : 3;        // Reservation Register Action (RREGA)
        ULONG IEKEY         : 1;        // Ignore Existing Key (IEKEY)
        ULONG Reserved      : 26;
        ULONG CPTPL         : 2;        // Change Persist Through Power Loss State (CPTPL)
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;
} NVME_CDW10_RESERVATION_REGISTER, *PNVME_CDW10_RESERVATION_REGISTER;

//
// Reservation Register Data Structure
//
typedef struct {

    ULONGLONG CRKEY;    // Current Reservation Key (CRKEY)

    //
    // If the Reseravation Acquire Action is set to 001b (Preempt) or 010b (Preempt and Abort),
    // then this field specifies the reservation key to be unregistered from the namespace.
    // For all other Reservation Acquire Action values, this field is reserved.
    //
    ULONGLONG NRKEY;    // New Reservation Key (NRKEY)

} NVME_RESERVATION_REGISTER_DATA_STRUCTURE, *PNVME_RESERVATION_REGISTER_DATA_STRUCTURE;

//
// Parameters for RESERVATION RELEASE Commands
//
typedef enum {

    NVME_RESERVATION_RELEASE_ACTION_RELEASE     = 0,
    NVME_RESERVATION_RELEASE_ACTION_CLEAR       = 1,

} NVME_RESERVATION_RELEASE_ACTIONS;

typedef union {

    struct {
        ULONG RRELA         : 3;        // Reservation Release Action (RRELA)
        ULONG IEKEY         : 1;        // IgnoreExistingKey (IEKEY)
        ULONG Reserved      : 4;
        ULONG RTYPE         : 8;        // Reservation Type (RTYPE)

        ULONG Reserved1     : 16;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW10_RESERVATION_RELEASE, *PNVME_CDW10_RESERVATION_RELEASE;

//
// Reservation Release Data Structure
//
typedef struct {

    ULONGLONG CRKEY;    // Current Reservation Key (CRKEY)

} NVME_RESERVATION_RELEASE_DATA_STRUCTURE, *PNVME_RESERVATION_RELEASE_DATA_STRUCTURE;

//
// Parameters for RESERVATION REPORT Commands
//

typedef union {

    struct {
        ULONG NUMD;     // Number of Dwords (NUMD), NOTE: 0's based value.
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW10_RESERVATION_REPORT, *PNVME_CDW10_RESERVATION_REPORT;

typedef union {

    struct {
        ULONG EDS           : 1;    // Extended Data Structure (EDS)

        ULONG Reserved      : 31;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW11_RESERVATION_REPORT, *PNVME_CDW11_RESERVATION_REPORT;

#pragma pack(push, 1)
typedef struct {

    //
    // This field is a counter that increments any time the below occurs:
    //      a. a Reservation Register command completes successfully on any controller associated with the namespace;
    //      b. a Reservation Release command with Reservation Release Action set to 001b (Clear) completes successfully
    //         on any controller associated with the name space;
    //      c. a Reservation Acquire command with Reservation Acquire Action set to 001b (Preempt) or 010b (Preempt and Abort)
    //         completes successfully on any controller associated with the namespace.
    // If the value of this field is FFFFFFFFh, then the field shall be cleared to 0b when incremented.
    //
    ULONG GEN;          // Generation (Gen)

    UCHAR RTYPE;        // Reservation Type (RTYPE)

    USHORT REGCTL;      // Number of Registered Controllers (REGCTL)

    UCHAR Reserved[2];

    UCHAR PTPLS;        // Persist Through Power Loss State (PTPLS)

    UCHAR Reserved1[14];

} NVME_RESERVATION_REPORT_STATUS_HEADER, *PNVME_RESERVATION_REPORT_STATUS_HEADER;
#pragma pack(pop)

C_ASSERT(sizeof(NVME_RESERVATION_REPORT_STATUS_HEADER) == 24);

typedef struct {

    USHORT CNTLID;                  // Controller ID (CNTLID)

    struct {
        UCHAR HoldReservation : 1;

        UCHAR Reserved        : 7;
    } RCSTS;                        // Reservation Status (RCSTS)

    UCHAR Reserved[5];

    UCHAR HOSTID[8];                // Host Identifier (HOSTID)
    ULONGLONG RKEY;                 // Reservation Key (RKEY)

} NVME_REGISTERED_CONTROLLER_DATA, *PNVME_REGISTERED_CONTROLLER_DATA;

C_ASSERT(sizeof(NVME_REGISTERED_CONTROLLER_DATA) == 24);

typedef struct {

    NVME_RESERVATION_REPORT_STATUS_HEADER Header;

    NVME_REGISTERED_CONTROLLER_DATA RegisteredControllersData[ANYSIZE_ARRAY];

} NVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE, *PNVME_RESERVATION_REPORT_STATUS_DATA_STRUCTURE;

typedef struct {

    USHORT CNTLID;                  // Controller ID (CNTLID)

    struct {
        UCHAR HoldReservation : 1;

        UCHAR Reserved        : 7;
    } RCSTS;                        // Reservation Status (RCSTS)

    UCHAR Reserved[5];

    ULONGLONG RKEY;             // Reservation Key (RKEY)
    UCHAR HOSTID[16];           // 128-bit Host Identifier (HOSTID)

    UCHAR Reserved1[32];

} NVME_REGISTERED_CONTROLLER_EXTENDED_DATA, *PNVME_REGISTERED_CONTROLLER_EXTENDED_DATA;

C_ASSERT(sizeof(NVME_REGISTERED_CONTROLLER_EXTENDED_DATA) == 64);

typedef struct {

    NVME_RESERVATION_REPORT_STATUS_HEADER Header;

    UCHAR Reserved1[40];

    NVME_REGISTERED_CONTROLLER_EXTENDED_DATA RegisteredControllersExtendedData[ANYSIZE_ARRAY];

} NVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE, *PNVME_RESERVATION_REPORT_STATUS_EXTENDED_DATA_STRUCTURE;

//
// Parameters for Directives.
//

typedef enum {

    NVME_DIRECTIVE_TYPE_IDENTIFY                    = 0x00,
    NVME_DIRECTIVE_TYPE_STREAMS                     = 0x01

} NVME_DIRECTIVE_TYPES;

#define NVME_STREAMS_ID_MIN 1
#define NVME_STREAMS_ID_MAX 0xFFFF

//
// General parameters for Directive Receive.
//

typedef struct {

    ULONG   NUMD; // Number of Dwords (NUMD)

} NVME_CDW10_DIRECTIVE_RECEIVE, *PNVME_CDW10_DIRECTIVE_RECEIVE;

typedef union {

    struct {
        ULONG   DOPER : 8;    // Directive Operation
        ULONG   DTYPE : 8;    // Directive Type
        ULONG   DSPEC : 16;   // Directive Specific
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_DIRECTIVE_RECEIVE, *PNVME_CDW11_DIRECTIVE_RECEIVE;


//
// General parameters for Directive Send.
//

typedef struct {

    ULONG   NUMD; // Number of Dwords (NUMD)

} NVME_CDW10_DIRECTIVE_SEND, *PNVME_CDW10_DIRECTIVE_SEND;

typedef union {

    struct {
        ULONG   DOPER : 8;    // Directive Operation
        ULONG   DTYPE : 8;    // Directive Type
        ULONG   DSPEC : 16;   // Directive Specific
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_DIRECTIVE_SEND, *PNVME_CDW11_DIRECTIVE_SEND;


//
// Parameters for the Identify Directive Type.
//

typedef enum {

    NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATION_RETURN_PARAMETERS = 1

} NVME_DIRECTIVE_RECEIVE_IDENTIFY_OPERATIONS;

typedef enum {

    NVME_DIRECTIVE_SEND_IDENTIFY_OPERATION_ENABLE_DIRECTIVE = 1

} NVME_DIRECTIVE_SEND_IDENTIFY_OPERATIONS;

typedef struct {

    UCHAR   Identify : 1;
    UCHAR   Streams : 1;
    UCHAR   Reserved0 : 6;

    UCHAR   Reserved1[31];

} NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR, *PNVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR;

typedef struct {

    NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR     DirectivesSupported;
    NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS_DESCRIPTOR     DirectivesEnabled;

    //
    // This data structure is 4KB in size.  The reserved space is commented out
    // so that this data structure can be safely allocated on the stack.
    //
    //UCHAR   Reserved[4032]; // 4096 - 32 - 32 = 4032

} NVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS, *PNVME_DIRECTIVE_IDENTIFY_RETURN_PARAMETERS;

typedef union {

    struct {
        ULONG   ENDIR       : 1;    // Enable Directive
        ULONG   Reserved0   : 7;
        ULONG   DTYPE       : 8;    // Directive Type
        ULONG   Reserved1   : 16;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE, *PNVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE;

//
// Parameters for the Streams Directive Type
//
typedef enum {

    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_RETURN_PARAMETERS = 1,
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_GET_STATUS = 2,
    NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATION_ALLOCATE_RESOURCES = 3

} NVME_DIRECTIVE_RECEIVE_STREAMS_OPERATIONS;

typedef enum {

    NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_IDENTIFIER = 1,
    NVME_DIRECTIVE_SEND_STREAMS_OPERATION_RELEASE_RESOURCES = 2

} NVME_DIRECTIVE_SEND_STREAMS_OPERATIONS;

typedef struct {

    USHORT  MSL;    // Max Streams Limit
    USHORT  NSSA;   // NVM Subsystem Streams Available
    USHORT  NSSO;   // NVM Subsystem Streams Open
    UCHAR   Reserved0[10];

    ULONG   SWS;    // Stream Write Size
    USHORT  SGS;    // Stream Granularity Size
    USHORT  NSA;    // Namespace Streams Allocated
    USHORT  NSO;    // Namespace Streams Open
    UCHAR   Reserved1[6];

} NVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS, *PNVME_DIRECTIVE_STREAMS_RETURN_PARAMETERS;

#define NVME_STREAMS_GET_STATUS_MAX_IDS 65535

typedef struct {

    USHORT  OpenStreamCount;            // Number of currently open streams.
    USHORT  StreamIdentifiers[NVME_STREAMS_GET_STATUS_MAX_IDS];   // Array of stream IDs that are currently open.

} NVME_DIRECTIVE_STREAMS_GET_STATUS_DATA, *PNVME_DIRECTIVE_STREAMS_GET_STATUS_DATA;

typedef union {

    struct {
        ULONG   NSR         : 16;   // Namespace Streams Requested
        ULONG   Reserved    : 16;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES, *PNVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES;

typedef struct {

    struct {
        ULONG   NSA         : 16;   // Namespace Streams Allocated
        ULONG   Reserved    : 16;
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES, *PNVME_COMPLETION_DW0_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES;

typedef union {
    NVME_CDW12_DIRECTIVE_SEND_IDENTIFY_ENABLE_DIRECTIVE EnableDirective;

    ULONG AsUlong;
} NVME_CDW12_DIRECTIVE_SEND;

typedef union {
    NVME_CDW12_DIRECTIVE_RECEIVE_STREAMS_ALLOCATE_RESOURCES AllocateResources;

    ULONG AsUlong;
} NVME_CDW12_DIRECTIVE_RECEIVE;

//
// Parameters for SECURITY SEND / RECEIVE Commands
//
typedef union {

    struct {
        ULONG   Reserved0   : 8;
        ULONG   SPSP        : 16;       // SP Specific (SPSP)
        ULONG   SECP        : 8;        // Security Protocol (SECP)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_SECURITY_SEND_RECEIVE, *PNVME_CDW10_SECURITY_SEND_RECEIVE;

typedef struct {

    ULONG   TL;                         // Transfer Length  (TL):

} NVME_CDW11_SECURITY_SEND, *PNVME_CDW11_SECURITY_SEND;

typedef struct {

    ULONG   AL;                         // Transfer Length  (AL)

} NVME_CDW11_SECURITY_RECEIVE, *PNVME_CDW11_SECURITY_RECEIVE;

//
// NVM Command Set
//
typedef enum {

    NVME_NVM_COMMAND_FLUSH                  = 0x00,
    NVME_NVM_COMMAND_WRITE                  = 0x01,
    NVME_NVM_COMMAND_READ                   = 0x02,

    NVME_NVM_COMMAND_WRITE_UNCORRECTABLE    = 0x04,
    NVME_NVM_COMMAND_COMPARE                = 0x05,
    NVME_NVM_COMMAND_WRITE_ZEROES           = 0x08,
    NVME_NVM_COMMAND_DATASET_MANAGEMENT     = 0x09,
    NVME_NVM_COMMAND_VERIFY                 = 0x0C,
    NVME_NVM_COMMAND_RESERVATION_REGISTER   = 0x0D,
    NVME_NVM_COMMAND_RESERVATION_REPORT     = 0x0E,
    NVME_NVM_COMMAND_RESERVATION_ACQUIRE    = 0x11,
    NVME_NVM_COMMAND_RESERVATION_RELEASE    = 0x15,
    NVME_NVM_COMMAND_COPY                   = 0x19,

    NVME_NVM_COMMAND_ZONE_MANAGEMENT_SEND       = 0x79,
    NVME_NVM_COMMAND_ZONE_MANAGEMENT_RECEIVE    = 0x7A,
    NVME_NVM_COMMAND_ZONE_APPEND                = 0x7D,

} NVME_NVM_COMMANDS;

//
// Data structure of CDW12 for Read/Write command
//
typedef union {

    struct {
        ULONG   NLB         : 16;       // Number of Logical Blocks (NLB)
        ULONG   Reserved0   : 4;
        ULONG   DTYPE       : 4;        // Directive Type (DTYPE)
        ULONG   Reserved1   : 2;
        ULONG   PRINFO      : 4;        // Protection Information Field (PRINFO)
        ULONG   FUA         : 1;        // Force Unit Access (FUA)
        ULONG   LR          : 1;        // Limited Retry (LR)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW12_READ_WRITE, *PNVME_CDW12_READ_WRITE;

//
// Data structure of CDW13 for Read/Write command
//
typedef enum {

    NVME_ACCESS_FREQUENCY_NONE                  = 0,    // No frequency information provided.
    NVME_ACCESS_FREQUENCY_TYPICAL               = 1,    // Typical number of reads and writes expected for this LBA range.
    NVME_ACCESS_FREQUENCY_INFR_WRITE_INFR_READ  = 2,    // Infrequent writes and infrequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_INFR_WRITE_FR_READ    = 3,    // Infrequent writes and frequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_FR_WRITE_INFR_READ    = 4,    // Frequent writes and infrequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_FR_WRITE_FR_READ      = 5,    // Frequent writes and frequent reads to the LBA range indicated.
    NVME_ACCESS_FREQUENCY_ONE_TIME_READ         = 6,    // One time read. E.g. command is due to virus scan, backup, file copy, or archive.
    NVME_ACCESS_FREQUENCY_SPECULATIVE_READ      = 7,    // Speculative read. The command is part of a prefetch operation.
    NVME_ACCESS_FREQUENCY_WILL_BE_OVERWRITTEN   = 8,    // The LBA range is going to be overwritten in the near future.

} NVME_ACCESS_FREQUENCIES;


typedef enum {

    NVME_ACCESS_LATENCY_NONE                = 0,        // None.  No latency information provided.
    NVME_ACCESS_LATENCY_IDLE                = 1,        // Idle. Longer latency acceptable
    NVME_ACCESS_LATENCY_NORMAL              = 2,        // Normal. Typical latency.
    NVME_ACCESS_LATENCY_LOW                 = 3,        // Low. Smallest possible latency

} NVME_ACCESS_LATENCIES;

typedef union {

    struct {
        struct {
            UCHAR   AccessFrequency     : 4;
            UCHAR   AccessLatency       : 2;
            UCHAR   SequentialRequest   : 1;
            UCHAR   Incompressible      : 1;
        } DSM;                      // Dataset Management (DSM)

        UCHAR   Reserved;
        USHORT  DSPEC;                // Directive Specific Value

    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW13_READ_WRITE, *PNVME_CDW13_READ_WRITE;

//
// Data structure of CDW15 for Read/Write command
//
typedef union {

    struct {
        ULONG   ELBAT       : 16;       // Expected Logical Block Application Tag (ELBAT)
        ULONG   ELBATM      : 16;       // Expected Logical Block Application Tag Mask (ELBATM)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW15_READ_WRITE, *PNVME_CDW15_READ_WRITE;


//
// Dataset Management - Range Definition
//
typedef union {

    struct {
        ULONG   AccessFrequency         : 4;
        ULONG   AccessLatency           : 2;
        ULONG   Reserved0               : 2;
        ULONG   SequentialReadRange     : 1;
        ULONG   SequentialWriteRange    : 1;
        ULONG   WritePrepare            : 1;
        ULONG   Reserved1               : 13;
        ULONG   CommandAccessSize       : 8;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CONTEXT_ATTRIBUTES, *PNVME_CONTEXT_ATTRIBUTES;

typedef struct {

    NVME_CONTEXT_ATTRIBUTES Attributes;             // The use of this information is optional and the controller is not required to perform any specific action.
    ULONG                   LogicalBlockCount;
    ULONGLONG               StartingLBA;

} NVME_LBA_RANGE, *PNVME_LBA_RANGE;

typedef union {

    struct {
        ULONG   NR          : 8;        // Number of Ranges (NR)
        ULONG   Reserved    : 24;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_DATASET_MANAGEMENT, *PNVME_CDW10_DATASET_MANAGEMENT;

typedef union {

    struct {
        ULONG   IDR         : 1;        // Integral Dataset for Read (IDR)
        ULONG   IDW         : 1;        // Integral Dataset for Write (IDW)
        ULONG   AD          : 1;        // Deallocate (AD)
        ULONG   Reserved    : 29;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW11_DATASET_MANAGEMENT, *PNVME_CDW11_DATASET_MANAGEMENT;

//
// Data structure of CDW12 for Verify command
//
typedef union {

    struct {
        ULONG   NLB         : 16;       // Number of Logical Blocks (NLB). Zero based. 
        ULONG   Reserved    : 10;
        ULONG   PRINFO      : 4;        // Protection Information Field (PRINFO)
        ULONG   FUA         : 1;        // Force Unit Access (FUA)
        ULONG   LR          : 1;        // Limited Retry (LR)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW12_VERIFYCOMMAND, *PNVME_CDW12_VERIFYCOMMAND;

//
// Data structure of CDW15 for Verify command
//
typedef union {

    struct {
        ULONG   ELBAT       : 16;       // Expected Logical Block Application Tag (ELBAT)
        ULONG   ELBATM      : 16;       // Expected Logical Block Application Tag Mask (ELBATM)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW15_VERIFY_COMMAND, *PNVME_CDW15_VERIFY_COMMAND;

//
// Zone Descriptor
//
typedef struct {

    struct {
        UCHAR       ZT          : 4;    // Zone Type
        UCHAR       Reserved1   : 4;
    } DUMMYSTRUCTNAME;

    struct {
        UCHAR       Reserved2   : 4;
        UCHAR       ZS          : 4;    // Zone State
    } DUMMYSTRUCTNAME;

    struct {
        UCHAR       ZFC         : 1;    // Zone Finished by Controller (ZFC)
        UCHAR       FZR         : 1;    // Finish Zone Recommended (FZR)
        UCHAR       RZR         : 1;    // Reset Zone Recommended (RZR)
        UCHAR       Reserved    : 4;
        UCHAR       ZDEV        : 1;    // Zone Descriptor Extension Valid (ZDEV)
    } ZA;                           // Zone Attribute

    UCHAR       Reserved3[5];

    ULONGLONG   ZCAP;       // Zone Capacity

    ULONGLONG   ZSLBA;              // Zone Start Logical Block Address

    ULONGLONG   WritePointer;       // Current Write pointer of the Zone

    UCHAR       Reserved4[32];

} NVME_ZONE_DESCRIPTOR, *PNVME_ZONE_DESCRIPTOR;


//
// Zone States
//
typedef enum {
    NVME_STATE_ZSE      = 0x1,    // Zone State Empty
    NVME_STATE_ZSIO     = 0x2,    // Zone State Implicitly Opened
    NVME_STATE_ZSEO     = 0x3,    // Zone State Explicitly Opened
    NVME_STATE_ZSC      = 0x4,    // Zone State Closed

    NVME_STATE_ZSRO     = 0xD,    // Zone State Read-Only
    NVME_STATE_ZSF      = 0xE,    // Zone State Full
    NVME_STATE_ZSO      = 0xF,    // Zone State Offline

} ZONE_STATE;

//
// Data structure of CDW13 for Zone Management Send command
//
typedef enum {
    NVME_ZONE_SEND_CLOSE        = 1,    // Close one or more zones
    NVME_ZONE_SEND_FINISH       = 2,    // Finish one or more zones
    NVME_ZONE_SEND_OPEN         = 3,    // Open one or more zones
    NVME_ZONE_SEND_RESET        = 4,    // Reset one or more zones
    NVME_ZONE_SEND_OFFLINE      = 5,    // Offline one or more zones

    NVME_ZONE_SEND_SET_ZONE_DESCRIPTOR      = 0x10,    // Attach Zone Descriptor Extension data to a zone in the Empty state and
                                                       // transition the zone to the Closed state
} NVME_ZONE_SEND_ACTION;

typedef struct {

    ULONGLONG               SLBA;       // Starting LBA (SLBA)

} NVME_CDW10_ZONE_MANAGEMENT_SEND, *PNVME_CDW10_ZONE_MANAGEMENT_SEND;

typedef union {

    struct {
        ULONG   ZSA         : 8;        // Zone Send Action, as defined in NVME_ZONE_SEND_ACTION
        ULONG   SelectAll   : 1;        // Select all the zones. SLBA is ignored if set
        ULONG   Reserved    : 23;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW13_ZONE_MANAGEMENT_SEND, *PNVME_CDW13_ZONE_MANAGEMENT_SEND;

//
// Report Zone Data Structure
//
typedef struct {

    ULONGLONG               ZoneCount;      //Number of Zones
    ULONGLONG               Reserved[7];

    NVME_ZONE_DESCRIPTOR    ZoneDescriptor[ANYSIZE_ARRAY];
} NVME_REPORT_ZONE_INFO, *PNVME_REPORT_ZONE_INFO;

//
// Extended Report Zone Data Structure and related defines
//
typedef struct{

    UCHAR   ZoneDescriptorExtensionInfo[64];

} NVME_ZONE_DESCRIPTOR_EXTENSION, *PNVME_ZONE_DESCRIPTOR_EXTENSION;

#define ZDES_SIZE_MULTIPLIER_IN_BYTES   64

typedef struct {

    NVME_ZONE_DESCRIPTOR            ZoneDescriptor;
    NVME_ZONE_DESCRIPTOR_EXTENSION  ZoneDescriptorExtension[ANYSIZE_ARRAY];

} NVME_ZONE_EXTENDED_REPORT_ZONE_DESC, *PNVME_ZONE_EXTENDED_REPORT_ZONE_DESC;

typedef struct {

    ULONGLONG               ZoneCount;      //Number of Zones
    ULONGLONG               Reserved[7];

    NVME_ZONE_EXTENDED_REPORT_ZONE_DESC Desc[ANYSIZE_ARRAY];
} NVME_EXTENDED_REPORT_ZONE_INFO, *PNVME_EXTENDED_REPORT_ZONE_INFO;

//
// Data structure of CDW13 for Zone Management Receive command and related defines
//
typedef enum {
    NVME_ZONE_RECEIVE_REPORT_ZONES          = 0,    // Returns report zone Descriptors
    NVME_ZONE_RECEIVE_EXTENDED_REPORT_ZONES = 1,    // Returns report zone descriptors with extended report zone information

} NVME_ZONE_RECEIVE_ACTION;

typedef enum {
    NVME_ZRA_ALL_ZONES              = 0,    // List all zones
    NVME_ZRA_EMPTY_STATE_ZONES      = 1,    // List zones with state Zone State Empty
    NVME_ZRA_IO_STATE_ZONES         = 2,    // List zones with state Zone State Implicitly Opened
    NVME_ZRA_EO_STATE_ZONES         = 3,    // List zones with state Zone State Explicitly Opened
    NVME_ZRA_CLOSED_STATE_ZONES     = 4,    // List zones with state Zone State Closed
    NVME_ZRA_FULL_STATE_ZONES       = 5,    // List zones with state Zone State Full
    NVME_ZRA_RO_STATE_ZONES         = 6,    // List zones with state Zone State Read-Only
    NVME_ZRA_OFFLINE_STATE_ZONES    = 7,    // List zones with state Zone State Offline

} NVME_ZONE_RECEIVE_ACTION_SPECIFIC;

typedef struct {

    ULONGLONG               SLBA;       // Starting LBA (SLBA)

} NVME_CDW10_ZONE_MANAGEMENT_RECEIVE, *PNVME_CDW10_ZONE_MANAGEMENT_RECEIVE;

typedef union {

    struct {
        ULONG   ZRA             : 8;        // Zone Receive Action, as defined in NVME_ZONE_RECEIVE_ACTION
        ULONG   ZRASpecific     : 8;        // Zone Receive Action Specific field, as defined in NVME_ZONE_RECEIVE_ACTION_SPECIFIC
        ULONG   Partial         : 1;        // Report Zones and Extended Report Zones: Partial Report
        ULONG   Reserved        : 15;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW13_ZONE_MANAGEMENT_RECEIVE, *PNVME_CDW13_ZONE_MANAGEMENT_RECEIVE;

typedef struct {

    ULONGLONG               SLBA;       // Starting LBA (SLBA)

} NVME_CDW10_ZONE_APPEND, *PNVME_CDW10_ZONE_APPEND;

typedef union {

    struct {
        ULONG   NLB             : 16;       // Number of Logical Blocks (NLB)
        ULONG   Reserved        : 9;
        ULONG   PIREMAP         : 1;        // Protection Information Remap (PIREMAP)
        ULONG   PRINFO          : 4;        // Protection Information Field (PRINFO)
        ULONG   FUA             : 1;        // Force Unit Access (FUA)
        ULONG   LR              : 1;        // Limited Retry(LR);
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW12_ZONE_APPEND, *PNVME_CDW12_ZONE_APPEND;

typedef union {

    struct {
        ULONG   LBAT            : 16;       // Logical Block Application Tag
        ULONG   LBATM           : 16;       // Logical Block Application Tag Mask (LBATM)
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW15_ZONE_APPEND, *PNVME_CDW15_ZONE_APPEND;

typedef union {

    struct {
        ULONG   STC         : 4;        // Self-test Code (STC)
        ULONG   Reserved    : 28;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_DEVICE_SELF_TEST, *PNVME_CDW10_DEVICE_SELF_TEST;

//
// Parameters for NVME_ADMIN_COMMAND_DISCOVERY_INFO_MGMT
//
typedef enum {

    NVME_DISCOVERY_INFO_MGMT_TASK_REGISTER = 0,
    NVME_DISCOVERY_INFO_MGMT_TASK_DEREGISTER = 1,
    NVME_DISCOVERY_INFO_MGMT_TASK_UPDATE = 2

} NVME_DISCOVERY_INFO_MGMT_TASK;

typedef union {

    struct {
        ULONG   TAS         : 4;       // Task (TAS) - NVME_DISCOVERY_INFO_MGMT_TASK
        ULONG   Reserved    : 28;
    } DUMMYSTRUCTNAME;

    ULONG   AsUlong;

} NVME_CDW10_DISCOVERY_INFO_MGMT, *PNVME_CDW10_DISCOVERY_INFO_MGMT;

//
// Command Dword 0
//

#define NVME_PSDT_XFER_PRP          0       // PRPs are used for this transfer.
#define NVME_PSDT_XFER_SGL_BYTE     1       // SGLs are used for this transfer. If used, Metadata Pointer (MPTR) contains an address
                                            //  of a single contiguous physical buffer that is byte aligned. Required for all
                                            //  Admin and I/O commands for NVMe over Fabrics implementations.
#define NVME_PSDT_XFER_SGL_QWORD    2       // SGLs are used for this transfer. If used, Metadata Pointer (MPTR) contains an address
                                            //  of an SGL segment containing exactly one SGL Descriptor that is qword aligned.
#define NVME_PSDT_XFER_RESERVED     3

typedef union {

    struct {
        //LSB
        ULONG OPC       : 8;        // Opcode (OPC)
        ULONG FUSE      : 2;        // Fused Operation (FUSE)
        ULONG Reserved0 : 4;
        ULONG PSDT      : 2;        // PRP or SGL for Data Transfer (PSDT)
        ULONG CID       : 16;       // Command Identifier (CID)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONG AsUlong;

} NVME_COMMAND_DWORD0, *PNVME_COMMAND_DWORD0;

//
// Fused Operation code
//
typedef enum {

    NVME_FUSED_OPERATION_NORMAL         = 0,
    NVME_FUSED_OPERATION_FIRST_CMD      = 1,
    NVME_FUSED_OPERATION_SECOND_CMD     = 2,

} NVME_FUSED_OPERATION_CODES;


typedef union {

    struct {
        //LSB
        ULONGLONG Reserved0 : 2;
        ULONGLONG PBAO      : 62;       // Page Base Address and Offset (PBAO)
        //MSB
    } DUMMYSTRUCTNAME;

    ULONGLONG AsUlonglong;

} NVME_PRP_ENTRY, *PNVME_PRP_ENTRY;

//
// If the namespace is not used for the command, then 'NSID' field shall be cleared to 0h.
// If a command shall be applied to all namespaces on the device, then 'NSID' field shall be set to FFFFFFFFh.
//
#define NVME_NAMESPACE_ALL              0xFFFFFFFF

//
// NVMe command data structure
//
typedef struct {
    //
    // Common fields for all commands
    //
    NVME_COMMAND_DWORD0 CDW0;
    ULONG               NSID;
    ULONG               Reserved0[2];
    ULONGLONG           MPTR;

    union {

        struct {
            ULONGLONG PRP1;
            ULONGLONG PRP2;
        };

        ULONGLONG SGL1[2];
    };

    //
    // Command independent fields from CDW10 to CDW15
    //
    union {

        //
        // General Command data fields
        //
        struct {
            ULONG   CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } GENERAL;

        //
        // Admin Command: Identify
        //
        struct {
            NVME_CDW10_IDENTIFY CDW10;
            NVME_CDW11_IDENTIFY CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            union {
                ULONG                   CDW14;
                NVME_CDW14_IDENTIFY     CDW14_V20;
            };
            ULONG   CDW15;
        } IDENTIFY;

        //
        // Admin Command: Abort
        //
        struct {
            NVME_CDW10_ABORT CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } ABORT;

        //
        // Admin Command: Get/Set Features
        //
        struct {
            NVME_CDW10_GET_FEATURES CDW10;
            NVME_CDW11_FEATURES     CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } GETFEATURES;

        struct {
            NVME_CDW10_SET_FEATURES CDW10;
            NVME_CDW11_FEATURES     CDW11;
            NVME_CDW12_FEATURES     CDW12;
            NVME_CDW13_FEATURES     CDW13;
            NVME_CDW14_FEATURES     CDW14;
            NVME_CDW15_FEATURES     CDW15;
        } SETFEATURES;

        //
        // Admin Command: Get Log Page
        //
        struct {
            union {
                NVME_CDW10_GET_LOG_PAGE      CDW10;
                NVME_CDW10_GET_LOG_PAGE_V121 CDW10_V121;
                NVME_CDW10_GET_LOG_PAGE_V13  CDW10_V13;
                NVME_CDW10_GET_LOG_PAGE_V20  CDW10_V20;
            };

            NVME_CDW11_GET_LOG_PAGE CDW11;
            NVME_CDW12_GET_LOG_PAGE CDW12;
            NVME_CDW13_GET_LOG_PAGE CDW13;
            union {
                NVME_CDW14_GET_LOG_PAGE         CDW14;
                NVME_CDW14_GET_LOG_PAGE_V20     CDW14_V20;
            };
            ULONG                   CDW15;
        } GETLOGPAGE;

        //
        // Admin Command: Create IO Completion Queue
        //
        struct {
            NVME_CDW10_CREATE_IO_QUEUE CDW10;
            NVME_CDW11_CREATE_IO_CQ    CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } CREATEIOCQ;

        //
        // Admin Command: Create IO Submission Queue
        //
        struct {
            NVME_CDW10_CREATE_IO_QUEUE CDW10;
            NVME_CDW11_CREATE_IO_SQ    CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } CREATEIOSQ;

        //
        // Admin Command: Delete IO Queue (Submission or Completion)
        //
        struct {
            NVME_CDW10_DELETE_IO_QUEUE CDW10;
        } DELETEIOQUEUE;

        //
        // NVM Command: Dataset Management
        //
        struct {
            NVME_CDW10_DATASET_MANAGEMENT   CDW10;
            NVME_CDW11_DATASET_MANAGEMENT   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } DATASETMANAGEMENT;

        //
        // Admin Command: SECURITY SEND
        //
        struct {
            NVME_CDW10_SECURITY_SEND_RECEIVE    CDW10;
            NVME_CDW11_SECURITY_SEND            CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } SECURITYSEND;

        //
        // Admin Command: SECURITY RECEIVE
        //
        struct {
            NVME_CDW10_SECURITY_SEND_RECEIVE    CDW10;
            NVME_CDW11_SECURITY_RECEIVE         CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } SECURITYRECEIVE;

        //
        // Admin Command: FIRMWARE IMAGE DOWNLOAD
        //
        struct {
            NVME_CDW10_FIRMWARE_DOWNLOAD        CDW10;
            NVME_CDW11_FIRMWARE_DOWNLOAD        CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } FIRMWAREDOWNLOAD;

        //
        // Admin Command: FIRMWARE ACTIVATE
        //
        struct {
            NVME_CDW10_FIRMWARE_ACTIVATE        CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } FIRMWAREACTIVATE;

        //
        // Admin Command: FORMAT NVM
        //
        struct {
            NVME_CDW10_FORMAT_NVM               CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } FORMATNVM;

        //
        // Admin Command: DIRECTIVE RECEIVE
        //
        struct {
            NVME_CDW10_DIRECTIVE_RECEIVE        CDW10;
            NVME_CDW11_DIRECTIVE_RECEIVE        CDW11;
            NVME_CDW12_DIRECTIVE_RECEIVE        CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } DIRECTIVERECEIVE;

        //
        // Admin Command: DIRECTIVE SEND
        //
        struct {
            NVME_CDW10_DIRECTIVE_SEND           CDW10;
            NVME_CDW11_DIRECTIVE_SEND           CDW11;
            NVME_CDW12_DIRECTIVE_SEND           CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } DIRECTIVESEND;

        //
        // Admin Command: SANITIZE
        //
        struct {
            NVME_CDW10_SANITIZE                 CDW10;
            NVME_CDW11_SANITIZE                 CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } SANITIZE;

        //
        // NVM Command: Read/Write
        //
        struct {
            ULONG                   LBALOW;
            ULONG                   LBAHIGH;
            NVME_CDW12_READ_WRITE   CDW12;
            NVME_CDW13_READ_WRITE   CDW13;
            ULONG                   CDW14;
            NVME_CDW15_READ_WRITE   CDW15;
        } READWRITE;

        //
        // NVM Command: RESERVATION ACQUIRE
        //
        struct {
            NVME_CDW10_RESERVATION_ACQUIRE      CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } RESERVATIONACQUIRE;

        //
        // NVM Command: RESERVATION REGISTER
        //
        struct {
            NVME_CDW10_RESERVATION_REGISTER     CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } RESERVATIONREGISTER;

        //
        // NVM Command: RESERVATION RELEASE
        //
        struct {
            NVME_CDW10_RESERVATION_RELEASE      CDW10;
            ULONG   CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } RESERVATIONRELEASE;

        //
        // NVM Command: RESERVATION REPORT
        //
        struct {
            NVME_CDW10_RESERVATION_REPORT       CDW10;
            NVME_CDW11_RESERVATION_REPORT       CDW11;
            ULONG   CDW12;
            ULONG   CDW13;
            ULONG   CDW14;
            ULONG   CDW15;
        } RESERVATIONREPORT;

        //
        // NVM Command: Zone Management Send
        //
        struct {
            NVME_CDW10_ZONE_MANAGEMENT_SEND CDW1011;
            ULONG                           CDW12;
            NVME_CDW13_ZONE_MANAGEMENT_SEND CDW13;
            ULONG                           CDW14;
            ULONG                           CDW15;
        } ZONEMANAGEMENTSEND;

        //
        // NVM Command: Zone Management Receive
        //
        struct {
            NVME_CDW10_ZONE_MANAGEMENT_RECEIVE  CDW1011;
            ULONG                               DWORDCOUNT;
            NVME_CDW13_ZONE_MANAGEMENT_RECEIVE  CDW13;
            ULONG                               CDW14;
            ULONG                               CDW15;
        } ZONEMANAGEMENTRECEIVE;

        //
        // NVM Command: Zone Append
        //
        struct {
            NVME_CDW10_ZONE_APPEND              CDW1011;
            NVME_CDW12_ZONE_APPEND              CDW12;
            ULONG                               CDW13;
            ULONG                               ILBRT;
            NVME_CDW15_ZONE_APPEND              CDW15;
        } ZONEAPPEND;

        //
        // NVM Command: Device Self Test
        //
        struct {
            NVME_CDW10_DEVICE_SELF_TEST CDW10;
            ULONG                       CDW11;
            ULONG                       CDW12;
            ULONG                       CDW13;
            ULONG                       CDW14;
            ULONG                       CDW15;
        } DEVICESELFTEST;

        //
        //
        //
        struct {
            NVME_CDW10_DISCOVERY_INFO_MGMT CDW10;
            ULONG                          CDW11;
            ULONG                          CDW12;
            ULONG                          CDW13;
            ULONG                          CDW14;
            ULONG                          CDW15;
        } DISCOVERYINFOMGMT;

        //
        // Admin or NVM Command: Vendor Specific Common Format
        //
        struct {
            ULONG                       NDT;
            ULONG                       NDM;
            ULONG                       CDW12;
            ULONG                       CDW13;
            ULONG                       CDW14;
            ULONG                       CDW15;
        } VENDORSPECIFIC;

        //
        // NVM Command: Verify command
        //
        struct {
            ULONG                       LBALOW;
            ULONG                       LBAHIGH;
            NVME_CDW12_VERIFYCOMMAND    CDW12;
            ULONG                       CDW13;
            ULONG                       EILBRT;
            NVME_CDW15_VERIFY_COMMAND   CDW15;
        } VERIFYCOMMAND;

    } u;

} NVME_COMMAND, *PNVME_COMMAND;

C_ASSERT(sizeof(NVME_COMMAND) == 64); // NVMe commands are always 64 bytes
                                      // (defined by constant STORAGE_PROTOCOL_COMMAND_LENGTH_NVME)

//
// The SCSI name string identifier used for the page 83 descriptor in NVMe to SCSI translation
// For NVMe devices compliant with revision 1.0.
//
typedef struct {

    CHAR PCIVendorID[4];
    CHAR ModelNumber[40];
    CHAR NamespaceID[4];
    CHAR SerialNumber[20];

} NVME_SCSI_NAME_STRING, *PNVME_SCSI_NAME_STRING;

////////////////////////////////////////////////////////////////////////
//
// NVMe SGL definitions based on 2.0 spec
//

//
// SGL Descriptor Type defined in
// the Scatter Gather List section
//
typedef enum _NVME_SGL_DESC_TYPE {

    NvmeSglDescTypeDataBlock          = 0x0,
    NvmeSglDescTypeBitBucket          = 0x1,
    NvmeSglDescTypeSegment            = 0x2,
    NvmeSglDescTypeLastSegment        = 0x3,
    NvmeSglDescTypeKeyedDataBlock     = 0x4,
    NvmeSglDescTypeTransportDataBlock = 0x5,
    NvmeSglDescTypeMax                = 0xF

} NVME_SGL_DESC_TYPE;

//
// SGL Descriptor Sub Type defined in
// the Scatter Gather List section
//
typedef enum _NVME_SGL_DESC_SUBTYPE {

    NvmeSglDescSubtypeAddress    = 0x0,
    NvmeSglDescSubtypeOffset     = 0x1,
    NvmeSglDescSubtypeTransportA = 0xA,
    NvmeSglDescSubtypeTransportB = 0xB,
    NvmeSglDescSubtypeTransportC = 0xC,
    NvmeSglDescSubtypeTransportD = 0xD,
    NvmeSglDescSubtypeTransportE = 0xE,
    NvmeSglDescSubtypeTransportF = 0xF

} NVME_SGL_DESC_SUBTYPE;

//
// General NVMe SGL Descriptor
//
typedef struct _NVME_SGL_DESC {

    UCHAR Reserved0[15];

    union {
        struct {

            UCHAR SubType : 4;
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_DESC, *PNVME_SGL_DESC;

C_ASSERT(sizeof(NVME_SGL_DESC) == 2 * sizeof(ULONGLONG));

//
// NVMe SGL Data Block descriptor
//
typedef struct _NVME_SGL_DATABLOCK_DESC {

    ULONGLONG Address;
    ULONG Length;
    UCHAR Reserved0[3];

    union {
        struct {

            UCHAR SubType : 4;

            _Field_range_(NvmeSglDescTypeDataBlock, NvmeSglDescTypeDataBlock)
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_DATABLOCK_DESC, *PNVME_SGL_DATABLOCK_DESC;

C_ASSERT(sizeof(NVME_SGL_DATABLOCK_DESC) == 2 * sizeof(ULONGLONG));

//
// NVMe SGL Bit Bucket descriptor
//
typedef struct _NVME_SGL_BITBUCKET_DESC {

    ULONGLONG Reserved0;
    ULONG Length;
    UCHAR Reserved1[3];

    union {
        struct {

            UCHAR SubType : 4;

            _Field_range_(NvmeSglDescTypeBitBucket, NvmeSglDescTypeBitBucket)
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_BITBUCKET_DESC, *PNVME_SGL_BITBUCKET_DESC;

C_ASSERT(sizeof(NVME_SGL_BITBUCKET_DESC) == 2 * sizeof(ULONGLONG));

//
// NVMe SGL Segment descriptor
//
typedef struct _NVME_SGL_SEGMENT_DESC {

    ULONGLONG Address;
    ULONG Length;
    UCHAR Reserved0[3];

    union {
        struct {

            UCHAR SubType : 4;

            _Field_range_(NvmeSglDescTypeSegment, NvmeSglDescTypeSegment)
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_SEGMENT_DESC, *PNVME_SGL_SEGMENT_DESC;

C_ASSERT(sizeof(NVME_SGL_SEGMENT_DESC) == 2 * sizeof(ULONGLONG));

//
// NVMe SGL Last Segment descriptor
//
typedef struct _NVME_SGL_LASTSEG_DESC {

    ULONGLONG Address;
    ULONG Length;
    UCHAR Reserved0[3];

    union {
        struct {

            UCHAR SubType : 4;

            _Field_range_(NvmeSglDescTypeLastSegment, NvmeSglDescTypeLastSegment)
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_LASTSEG_DESC, *PNVME_SGL_LASTSEG_DESC;

C_ASSERT(sizeof(NVME_SGL_LASTSEG_DESC) == 2 * sizeof(ULONGLONG));

//
// SGL Descriptor Sub Type defined in
// the Scatter Gather List section
//
typedef enum _NVME_RDMA_KEYED_SGL_DESC_SUBTYPE {

    NvmeRdmaKeyedSglDescSubtypeInvalidate = 0xF

} NVME_RDMA_KEYED_SGL_DESC_SUBTYPE;

//
// NVMe SGL Keyed Data Block descriptor
//
typedef struct _NVME_SGL_KEYDATABLOCK_DESC {

    ULONGLONG Address;
    UCHAR Length[3];
    UCHAR Key[4];

    union {
        struct {

            UCHAR SubType : 4;

            _Field_range_(NvmeSglDescTypeKeyedDataBlock, NvmeSglDescTypeKeyedDataBlock)
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_KEYDATABLOCK_DESC, *PNVME_SGL_KEYDATABLOCK_DESC;

C_ASSERT(sizeof(NVME_SGL_KEYDATABLOCK_DESC) == 2 * sizeof(ULONGLONG));

//
// NVMe SGL Transport Data Block descriptor
//
typedef struct _NVME_SGL_TRANSPORTDATA_DESC {

    ULONGLONG Reserved0;
    ULONG Length;
    UCHAR Reserved1[3];

    union {
        struct {

            UCHAR SubType : 4;

            _Field_range_(NvmeSglDescTypeTransportDataBlock, NvmeSglDescTypeTransportDataBlock)
            UCHAR Type    : 4;
        };

        UCHAR AsUchar;

    } Identifier;

} NVME_SGL_TRANSPORTDATA_DESC, *PNVME_SGL_TRANSPORTDATA_DESC;

C_ASSERT(sizeof(NVME_SGL_TRANSPORTDATA_DESC) == 2 * sizeof(ULONGLONG));

////////////////////////////////////////////////////////////////////////
//
// NVMe over Fabrics definitions based on 2.0 spec
//

#define NVMEOF_TRANSPORT_ADDR_MAX_LEN      256
#define NVMEOF_TRANSPORT_SERVID_MAX_LEN    32
#define NVMEOF_TRANSPORT_SAS_MAX_LEN       256

#define NVMEOF_DISCOVERY_NQN               "nqn.2014-08.org.nvmexpress.discovery"
#define NVMEOF_DISCOVERY_LOG_VERSION_0     0

#define NVMEOF_ADMINQ_MIN_DEPTH            32
#define NVMEOF_ADMINQ_MAX_DEPTH            4096

#define NVMEOF_IOQ_MIN_DEPTH               2
#define NVMEOF_IOQ_MAX_DEPTH               65536

#define NVMEOF_PROPERTY_SIZE_4Bytes        0x00
#define NVMEOF_PROPERTY_SIZE_8Bytes        0x01

//
// Fabrics Command Types
//
typedef enum {

    NVME_FABRICS_COMMAND_PROPERTY_SET    = 0x00,
    NVME_FABRICS_COMMAND_CONNECT         = 0x01,
    NVME_FABRICS_COMMAND_PROPERTY_GET    = 0x04,
    NVME_FABRICS_COMMAND_AUTH_SEND       = 0x05,
    NVME_FABRICS_COMMAND_AUTH_RECV       = 0x06,
    NVME_FABRICS_COMMAND_DISCONNECT      = 0x08

} NVME_FABRICS_COMMAND_TYPE;

//
// Transport type (TRTYPE) defined in the
// Get Log Page - Discovery Log Page Entry
//
typedef enum _NVMEOF_TRANSPORT_TYPE {

    NvmeofTransportUnknown  = 0,
    NvmeofTransportRdma     = 1,
    NvmeofTransportFC       = 2,
    NvmeofTransportTcp      = 3,
    NvmeofTransportLoopback = 254,
    NvmeofTransportMax      = 255

} NVMEOF_TRANSPORT_TYPE;

//
// Address family (ADRFAM) defined in the
// Get Log Page - Discovery Log Page Entry
//
typedef enum _NVMEOF_ADDRESS_FAMILY {

    NvmeofAddressUnknown  = 0,
    NvmeofAddressIPv4     = 1,
    NvmeofAddressIPv6     = 2,
    NvmeofAddressIB       = 3,
    NvmeofAddressFC       = 4,
    NvmeofAddressLoopback = 254,
    NvmeofAddressMax      = 255

} NVMEOF_ADDRESS_FAMILY;

//
// Subsystem type (SUBTYPE) defined in the
// Get Log Page - Discovery Log Page Entry
//
typedef enum _NVMEOF_SUBSYSTEM_TYPE {

    NvmeofSubsysTypeUnknown      = 0,
    NvmeofSubsysTypeDiscReferral = 1,
    NvmeofSubsysTypeIo           = 2,
    NvmeofSubsysTypeDiscCurrent  = 3,
    NvmeofSubsysTypeMax          = 255

} NVMEOF_SUBSYSTEM_TYPE;

//
// Fabric secure channel requirement in the
// Transport requirements (TREQ) field in the
// Get Log Page - Discovery Log Page Entry
//
typedef enum _NVMEOF_SECURE_CHANNEL {

    NvmeofSCUnspecified = 0,
    NvmeofSCRequired    = 1,
    NvmeofSCNotRequired = 2,
    NvmeofSCReserved    = 3

} NVMEOF_SECURE_CHANNEL;

//
// Fabric authentication and secure channel
// requirement in the Transport requirements
// (TREQ) field in the Get Log Page -
// Discovery Log Page Entry
//
typedef enum _NVMEOF_AUTH_SECURE_CHANNEL {

    NvmeofAuthSCUnspecified          = 0,
    NvmeofAuthSCAuthRequired         = 1,
    NvmeofAuthSCAuthConcatSCRequired = 2,
    NvmeofAuthSCReserved             = 3

} NVMEOF_AUTH_SECURE_CHANNEL;

//
// NVMe Fabrics Command Capsule
//
typedef struct _NVMEOF_FABRICS_COMMAND {

    UCHAR OPC;          // Opcode (7Fh)
    UCHAR PSDT;         // PRP or SGL for data transfer (only Bits 7:6 are to be used, rest are reserved)
    USHORT CID;         // Command Identifier
    UCHAR FCTYPE;       // Fabrics Command Type
    UCHAR Reserved[35]; // Byte 5:39
    UCHAR Specific[24]; // Byte 40:63 - Command Type Specific

} NVMEOF_FABRICS_COMMAND, *PNVMEOF_FABRICS_COMMAND;

C_ASSERT(sizeof(NVMEOF_FABRICS_COMMAND) == 64);

//
// NVMe Fabrics Response Capsule
//
typedef struct _NVMEOF_FABRICS_RESPONSE {

    UCHAR Specific[8]; // Byte 0:7 - Response Type Specific
    USHORT SQHD;       // SQ Head Pointer
    USHORT Reserved;   // Byte 10:11
    USHORT CID;        // Command Identifier
    USHORT STS;        // Status - NVME_COMMAND_STATUS

} NVMEOF_FABRICS_RESPONSE, *PNVMEOF_FABRICS_RESPONSE;

C_ASSERT(sizeof(NVMEOF_FABRICS_RESPONSE) == 16);

//
// NVMeoF Connect Command
//
typedef struct _NVMEOF_CONNECT_COMMAND {

    UCHAR OPC; // Opcode (7Fh)
    UCHAR Reserved0; // Maps to PSDT in NVMEOF_FABRICS_COMMAND
    USHORT CID; // Command Identifier
    UCHAR FCTYPE; // Fabrics Command Type, set to 01h
    UCHAR Reserved1[19];
    NVME_SGL_DESC SGL1; // SGL descriptor that describes the entire data transfer
    USHORT RECFMT; // Format of connect command capsule
    USHORT QID; // Queue Identifier for the Admin Queue or I/O Queue to be created
    USHORT SQSIZE; // Size of submission queue to be created

    union {

        struct {

            UCHAR PriorityClass : 2;
            UCHAR SqFlowControlDisable : 1;
            UCHAR IoQueueDeletion : 1;
            UCHAR Reserved : 4;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;

    } CATTR; // Connection attributes

    UCHAR Reserved2;
    ULONG KATO; // Keep Alive Timeout, valid only for Admin Queue, reserved for IO Queue
    UCHAR Reserved3[12];

} NVMEOF_CONNECT_COMMAND, *PNVMEOF_CONNECT_COMMAND;

C_ASSERT(sizeof(NVMEOF_CONNECT_COMMAND) == 64);

//
// NVMeoF Connect Command Data
//
typedef struct _NVMEOF_CONNECT_DATA {

    UCHAR HOSTID[NVME_EXTENDED_HOST_IDENTIFIER_SIZE];
    USHORT CNTLID;
    UCHAR Reserved0[238];
    UCHAR SUBNQN[NVME_NQN_MAX_LEN];
    UCHAR HOSTNQN[NVME_NQN_MAX_LEN];
    UCHAR Reserved1[256];

} NVMEOF_CONNECT_DATA, *PNVMEOF_CONNECT_DATA;

C_ASSERT(sizeof(NVMEOF_CONNECT_DATA) == 1024);

//
// NVMeoF Connect Response
//
typedef struct _NVMEOF_CONNECT_RESPONSE {

    union {

        struct {

            USHORT CNTLID;

            union {

                struct {

                    USHORT Obsolete : 1;
                    USHORT ATR : 1;
                    USHORT ASCR : 1;
                    USHORT Reserved : 13;
                };

                USHORT AsUshort;

            } AUTHREQ;

        } Success;

        ULONG AsUlong;

    } SCSpecific; // Status Code Specific

    ULONG Reserved0;
    USHORT SQHD; // Current Submission Queue Head pointer if SQ flow control is enabled
    USHORT Reserved1;
    USHORT CID; // Command Identifier
    USHORT STS; // Status

} NVMEOF_CONNECT_RESPONSE, *PNVMEOF_CONNECT_RESPONSE;

C_ASSERT(sizeof(NVMEOF_CONNECT_RESPONSE) == 16);

//
// NVMeoF Disconnect Command
//
typedef struct _NVMEOF_DISCONNECT_COMMAND {

    UCHAR OPC; // Opcode (7Fh)
    UCHAR Reserved0; // Maps to PSDT in NVMEOF_FABRICS_COMMAND
    USHORT CID; // Command Identifier
    UCHAR FCTYPE; // Fabrics Command Type, set to 08h
    UCHAR Reserved1[19]; // Byte 5:23
    NVME_SGL_DATABLOCK_DESC SGL1;
    USHORT RECFMT; // Format of disconnect command capsule
    UCHAR Reserved2[22];

} NVMEOF_DISCONNECT_COMMAND, *PNVMEOF_DISCONNECT_COMMAND;

C_ASSERT(sizeof(NVMEOF_DISCONNECT_COMMAND) == 64);

//
// NVMeoF Disconnect Response
//
typedef struct _NVMEOF_DISCONNECT_RESPONSE {

    ULONGLONG Reserved0;
    USHORT SQHD; // Current Submission Queue Head pointer for the associated Submission Queue
    USHORT Reserved1;
    USHORT CID; // Command Identifier
    USHORT STS; // Status

} NVMEOF_DISCONNECT_RESPONSE, *PNVMEOF_DISCONNECT_RESPONSE;

C_ASSERT(sizeof(NVMEOF_DISCONNECT_RESPONSE) == 16);

//
// NVMeoF Property Get Command
//

typedef struct _NVMEOF_PROPERTY_GET_COMMAND {

    UCHAR OPC; // Opcode (7Fh)
    UCHAR Reserved0; // Maps to PSDT in NVMEOF_FABRICS_COMMAND
    USHORT CID; // Command Identifier
    UCHAR FCTYPE; // Fabrics Command Type, set to 04h
    UCHAR Reserved1[35]; // Byte 5:39

    struct {

        UCHAR PropertySize : 3;
        UCHAR Reserved : 5;
    } ATTRIB; // Attributes for the Property Get command

    UCHAR Reserved2[3];
    ULONG OFST; // Offset to the property to get
    UCHAR Reserved3[16];

} NVMEOF_PROPERTY_GET_COMMAND, *PNVMEOF_PROPERTY_GET_COMMAND;

C_ASSERT(sizeof(NVMEOF_PROPERTY_GET_COMMAND) == 64);

//
// NVMeoF Property Get Response
//
typedef struct _NVMEOF_PROPERTY_GET_RESPONSE {

    union {

        struct {

            ULONG Value;
            ULONG Reserved;
        } FourBytes;

        ULONGLONG EightBytes;

    } VALUE; // Value returned for the property

    USHORT SQHD; // Current Submission Queue Head pointer for the associated Submission Queue
    USHORT Reserved0;
    USHORT CID; // Command Identifier
    USHORT STS; // Status

} NVMEOF_PROPERTY_GET_RESPONSE, *PNVMEOF_PROPERTY_GET_RESPONSE;

C_ASSERT(sizeof(NVMEOF_PROPERTY_GET_RESPONSE) == 16);

//
// NVMeoF Property Set Command
//
typedef struct _NVMEOF_PROPERTY_SET_COMMAND {

    UCHAR OPC; // Opcode (7Fh)
    UCHAR Reserved0; // Maps to PSDT in NVMEOF_FABRICS_COMMAND
    USHORT CID; // Command Identifier
    UCHAR FCTYPE; // Fabrics Command Type, set to 00h
    UCHAR Reserved1[35]; // Byte 5:39

    struct {

        UCHAR PropertySize : 3;
        UCHAR Reserved : 5;
    } ATTRIB; // Attributes for the Property Set command

    UCHAR Reserved2[3];
    ULONG OFST; // Offset to the property to get

    union {

        struct {

            ULONG Value;
            ULONG Reserved;
        } FourBytes;

        ULONGLONG EightBytes;

    } VALUE; // Value to set for the property

    UCHAR Reserved3[8];

} NVMEOF_PROPERTY_SET_COMMAND, *PNVMEOF_PROPERTY_SET_COMMAND;

C_ASSERT(sizeof(NVMEOF_PROPERTY_SET_COMMAND) == 64);

//
// NVMeoF Property Set Response
//
typedef struct _NVMEOF_PROPERTY_SET_RESPONSE {

    ULONGLONG Reserved0;
    USHORT SQHD; // Current Submission Queue Head pointer for the associated Submission Queue
    USHORT Reserved1;
    USHORT CID; // Command Identifier
    USHORT STS; // Status

} NVMEOF_PROPERTY_SET_RESPONSE, *PNVMEOF_PROPERTY_SET_RESPONSE;

C_ASSERT(sizeof(NVMEOF_PROPERTY_SET_RESPONSE) == 16);

//
// NVMeoF Authenticate Receive Command
//
typedef struct _NVMEOF_AUTH_RECEIVE_COMMAND {

    UCHAR OPC; // Opcode (7Fh)
    UCHAR Reserved0; // Maps to PSDT in NVMEOF_FABRICS_COMMAND
    USHORT CID; // Command Identifier
    UCHAR FCTYPE; // Fabrics Command Type, set to 06h
    UCHAR Reserved1[19]; // Byte 5:23
    NVME_SGL_DESC SGL1; // SGL descriptor that describes the entire data transfer
    UCHAR Reserved2;
    UCHAR SPSP0; // Bits 07:00 of Security Protocol Specific field as defined in SPC-5
    UCHAR SPSP1; // Bits 15:08 of Security Protocol Specific field as defined in SPC-5
    UCHAR SECP; // Security protocol as defined in SPC-5
    ULONG AL; // Allocation Length, specific to the Security Protocol as defined in SPC-5 where INC_512 is cleared to '0'

    UCHAR Reserved3[16];

} NVMEOF_AUTH_RECEIVE_COMMAND, *PNVMEOF_AUTH_RECEIVE_COMMAND;

C_ASSERT(sizeof(NVMEOF_AUTH_RECEIVE_COMMAND) == 64);

//
// NVMeoF Authenticate Receive Response
//
typedef struct _NVMEOF_AUTH_RECEIVE_RESPONSE {

    ULONGLONG Reserved0;
    USHORT SQHD; // Current Submission Queue Head pointer for the associated Submission Queue
    USHORT Reserved1;
    USHORT CID; // Command Identifier
    USHORT STS; // Status

} NVMEOF_AUTH_RECEIVE_RESPONSE, *PNVMEOF_AUTH_RECEIVE_RESPONSE;

C_ASSERT(sizeof(NVMEOF_AUTH_RECEIVE_RESPONSE) == 16);

//
// NVMeoF Authenticate Send Command
//
typedef struct _NVMEOF_AUTH_SEND_COMMAND {

    UCHAR OPC; // Opcode (7Fh)
    UCHAR Reserved0; // Maps to PSDT in NVMEOF_FABRICS_COMMAND
    USHORT CID; // Command Identifier
    UCHAR FCTYPE; // Fabrics Command Type, set to 05h
    UCHAR Reserved1[19]; // Byte 5:23
    NVME_SGL_DESC SGL1; // SGL descriptor that describes the entire data transfer
    UCHAR Reserved2;
    UCHAR SPSP0; // Bits 07:00 of Security Protocol Specific field as defined in SPC-5
    UCHAR SPSP1; // Bits 15:08 of Security Protocol Specific field as defined in SPC-5
    UCHAR SECP; // Security protocol as defined in SPC-5
    ULONG TL; // Transfer Length, specific to the Security Protocol as defined in SPC-5 where INC_512 is cleared to '0'

    UCHAR Reserved3[16];

} NVMEOF_AUTH_SEND_COMMAND, *PNVMEOF_AUTH_SEND_COMMAND;

C_ASSERT(sizeof(NVMEOF_AUTH_SEND_COMMAND) == 64);

//
// NVMeoF Authenticate Send Response
//
typedef struct _NVMEOF_AUTH_SEND_RESPONSE {

    ULONGLONG Reserved0;
    USHORT SQHD; // Current Submission Queue Head pointer for the associated Submission Queue
    USHORT Reserved1;
    USHORT CID; // Command Identifier
    USHORT STS; // Status

} NVMEOF_AUTH_SEND_RESPONSE, *PNVMEOF_AUTH_SEND_RESPONSE;

C_ASSERT(sizeof(NVMEOF_AUTH_SEND_RESPONSE) == 16);

//
// NVMe Discovery and Extended Discovery Entry
// for Subsystem and Host information.
//
typedef struct _NVME_DISCOVERY_ENTRY {

    UCHAR TRTYPE;  // Transport Type - NVMEOF_TRANSPORT_TYPE
    UCHAR ADRFAM;  // Address Family - NVMEOF_ADDRESS_FAMILY
    UCHAR SUBTYPE; // Subsystem Type - NVMEOF_SUBSYSTEM_TYPE

    union {

        struct {

            UCHAR SecureChannel : 2; // NVMEOF_SECURE_CHANNEL
            UCHAR SqFlowControlDisable : 1;
            UCHAR ZeroHostIdSupport : 1;
            UCHAR AuthAndSecureChannel : 2; //NVMEOF_AUTH_SECURE_CHANNEL
            UCHAR Reserved : 2;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;

    } TREQ;        // Transport Requirements

    USHORT PORTID; // Subsystem Port Id
    USHORT CNTLID; // Controller Id
                   // If subsystem supports dynamic controller model the value will be FFFFh.
                   // If subsystem supports static controller model and value is FFFEh, the
                   //   host should remember the controller Id returned by Connect command.
                   // If subsystem supports static controller model and value is between
                   // 0h and FFEFh, then a specific controller is specified.
    USHORT ASQSZ;  // Maximum size of an Admin Submission Queue, minimum value of 32

    union {

        struct {

            USHORT DuplicateReturnedInfo : 1;
            USHORT ExplicitPersistentConnectionSupport : 1;
            USHORT NoCDCConnectivity : 1;
            USHORT Reserved : 13;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;

    } EFLAGS;        // Entry Flags

    UCHAR Reserved0[20];
    UCHAR TRSVCID[NVMEOF_TRANSPORT_SERVID_MAX_LEN]; // NVMe Transport service identifier
    UCHAR Reserved1[192];
    UCHAR NQN[NVME_NQN_MAX_LEN]; // NQN that uniquely identifies the NVM entity (Subsystem/Host)

    UCHAR TRADDR[NVMEOF_TRANSPORT_ADDR_MAX_LEN]; // Address of the fabric interface on the NVM entity (Subsystem/Host)
    UCHAR TSAS[NVMEOF_TRANSPORT_SAS_MAX_LEN]; // Transport specific information of the address

} NVME_DISCOVERY_ENTRY, *PNVME_DISCOVERY_ENTRY;

C_ASSERT(sizeof(NVME_DISCOVERY_ENTRY) == 1024);

typedef struct _NVME_EXTENDED_DISCOVERY_ENTRY {

    UCHAR TRTYPE;  // Transport Type - NVMEOF_TRANSPORT_TYPE
    UCHAR ADRFAM;  // Address Family - NVMEOF_ADDRESS_FAMILY
    UCHAR SUBTYPE; // Subsystem Type - NVMEOF_SUBSYSTEM_TYPE

    union {

        struct {

            UCHAR SecureChannel : 2; // NVMEOF_SECURE_CHANNEL
            UCHAR SqFlowControlDisable : 1;
            UCHAR ZeroHostIdSupport : 1;
            UCHAR AuthAndSecureChannel : 2; //NVMEOF_AUTH_SECURE_CHANNEL
            UCHAR Reserved : 2;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;

    } TREQ;        // Transport Requirements

    USHORT PORTID; // Subsystem Port Id
    USHORT CNTLID; // Controller Id
                   // If subsystem supports dynamic controller model the value will be FFFFh.
                   // If subsystem supports static controller model and value is FFFEh, the
                   //   host should remember the controller Id returned by Connect command.
                   // If subsystem supports static controller model and value is between
                   // 0h and FFEFh, then a specific controller is specified.
    USHORT ASQSZ;  // Maximum size of an Admin Submission Queue, minimum value of 32

    union {

        struct {

            USHORT DuplicateReturnedInfo : 1;
            USHORT ExplicitPersistentConnectionSupport : 1;
            USHORT NoCDCConnectivity : 1;
            USHORT Reserved : 13;
        } DUMMYSTRUCTNAME;

        USHORT AsUshort;

    } EFLAGS;        // Entry Flags

    UCHAR Reserved0[20];
    UCHAR TRSVCID[NVMEOF_TRANSPORT_SERVID_MAX_LEN]; // NVMe Transport service identifier
    UCHAR Reserved1[192];
    UCHAR NQN[NVME_NQN_MAX_LEN]; // NQN that uniquely identifies the NVM entity (Subsystem/Host)

    UCHAR TRADDR[NVMEOF_TRANSPORT_ADDR_MAX_LEN]; // Address of the fabric interface on the NVM entity (Subsystem/Host)
    UCHAR TSAS[NVMEOF_TRANSPORT_SAS_MAX_LEN]; // Transport specific information of the address

    ULONG TEL; // Length in bytes of the entire extended discovery information entry

    USHORT NUMEXAT; // Number of Extended Attributes

    USHORT Reserved2;

} NVME_EXTENDED_DISCOVERY_ENTRY, *PNVME_EXTENDED_DISCOVERY_ENTRY;

C_ASSERT(sizeof(NVME_EXTENDED_DISCOVERY_ENTRY) == 1032);

typedef enum _NVME_EXTENDED_ATTR_TYPE {

    NvmeExtAttrReserved0 = 0x0,
    NvmeExtAttrHostId = 0x1,
    NvmeExtAttrAdminLabelAscii = 0x2,
    NvmeExtAttrAdminLabelUtf8 = 0x3,
    NvmeExtAttrReservedStart = 0x4,
    NvmeExtAttrReservedEnd = 0xFEFF,
    NvmeExtAttrVendorStart = 0xFF00,
    NvmeExtAttrVendorEnd = 0xFFFF

} NVME_EXTENDED_ATTR_TYPE;

typedef struct _NVME_EXTENDED_ATTR {

    USHORT EXATTYPE; // Extended Attribute Type - NVME_EXTENDED_ATTR_TYPE

    USHORT EXATLEN; // Extended Attribute Length

    _Field_size_(EXATLEN)
    UCHAR EXATVAL[0]; // Extended Attribute Value

} NVME_EXTENDED_ATTR, *PNVME_EXTENDED_ATTR;

//
// NVMe Discovery Log Page Header
//
typedef struct _NVME_DISCOVERY_HEADER {

    ULONGLONG GENCTR; // Version of the discovery information starting at 0h and incrementing
    ULONGLONG NUMREC; // Number of records contained in the log
    USHORT RECFMT; // Format of the Discovery Log Page

    union {

        struct {

            UCHAR Extended : 1;
            UCHAR PortLocal : 1;
            UCHAR AllSubsystems : 1;
            UCHAR Reserved : 5;
        } DUMMYSTRUCTNAME;

        UCHAR AsUchar;

    } DLPF;        // Entry Flags

    UCHAR Reserved0;

    ULONG TDLPL;

    UCHAR Reserved1[1000];

} NVME_DISCOVERY_HEADER, *PNVME_DISCOVERY_HEADER;

C_ASSERT(sizeof(NVME_DISCOVERY_HEADER) == 1024);

//
// Data for NVME_ADMIN_COMMAND_DISCOVERY_INFO_MGMT
//
typedef enum {

    NVME_DISCOVERY_INFO_ENTRY_FORMAT_RESERVED = 0,
    NVME_DISCOVERY_INFO_ENTRY_FORMAT_BASIC = 1,
    NVME_DISCOVERY_INFO_ENTRY_FORMAT_EXTENDED = 2

} NVME_DISCOVERY_INFO_ENTRY_FORMATS;

typedef enum {

    NVME_DISCOVERY_INFO_ENTITY_TYPE_RESERVED = 0,
    NVME_DISCOVERY_INFO_ENTITY_TYPE_HOST = 1,
    NVME_DISCOVERY_INFO_ENTITY_TYPE_DDC = 2,
    NVME_DISCOVERY_INFO_ENTITY_TYPE_CDC = 3

} NVME_DISCOVERY_INFO_ENTITY_TYPES;

#define NVME_DISCOVERY_INFO_MGMT_EKTYPE_PORTID      0x003F   // EKTYPE - Port ID Based (For Subsystem)
#define NVME_DISCOVERY_INFO_MGMT_EKTYPE_TRADDR      0x005F   // EKTYPE - TRADDR Based (For Subsystem and Host)

typedef struct _NVME_DISCOVERY_INFO_MGMT_HEADER {

    ULONG       TDL;               // Total Data Length (TDL)
    ULONG       Reserved0;

    ULONGLONG   NUMENT;            // Number of Entries (NUMENT)

    USHORT      ENTFMT;            // Entry Format (ENTFMT)
    USHORT      ETYPE;             // Entity Type (ETYPE)

    UCHAR       PORTLCL;           // Port Local (PORTLCL)
    UCHAR       Reserved1;

    union {

        struct {

            USHORT  NQN          : 1;
            USHORT  TSAS         : 1;
            USHORT  TRSVCID      : 1;
            USHORT  ADRFAM       : 1;
            USHORT  TRTYPE       : 1;
            USHORT  PORTID       : 1;
            USHORT  TRADDR       : 1;
            USHORT  Reserved     : 9;

        } DUMMYSTRUCTNAME;

        USHORT AsUshort;

    } EKTYPE;                      // Entry Key Type (EKTYPE)

    UCHAR       EID[256];          // Entity Identifier (EID)
    UCHAR       ENAME[256];        // Entity Name (ENAME)
    UCHAR       EVER[64];          // Entity Version (EVER)

    UCHAR       Reserved2[424];

} NVME_DISCOVERY_INFO_MGMT_HEADER, *PNVME_DISCOVERY_INFO_MGMT_HEADER;

C_ASSERT(sizeof(NVME_DISCOVERY_INFO_MGMT_HEADER) == 1024);

//
// NVMeof Authentication and Secure Channel definitions
//

//
// Authentication Security Protocols (SECP)
//
typedef enum _NVMEOF_AUTH_PROTOCOL {

    NvmeofAuthProtocolDHCHAP = 0xe9

} NVMEOF_AUTH_PROTOCOL;

//
// Authentication Types (AUTH_TYPE)
//
typedef enum _NVMEOF_AUTH_TYPE {

    NvmeofAuthTypeCommonMessages = 0x00,
    NvmeofAuthTypeDHCHAPMessages = 0x01

} NVMEOF_AUTH_TYPE;

//
// Authentication IDs (AUTH_ID)
//
typedef enum _NVMEOF_AUTH_ID {

    NvmeofAuthIdNegotiate = 0x00,
    NvmeofAuthIdChallenge = 0x01,
    NvmeofAuthIdReply     = 0x02,
    NvmeofAuthIdSuccess1  = 0x03,
    NvmeofAuthIdSuccess2  = 0x04,
    NvmeofAuthIdFailure2  = 0xf0,
    NvmeofAuthIdFailure1  = 0xf1

} NVMEOF_AUTH_ID;

//
// Secure Channel Protocol Identifiers (SC_C)
//
typedef enum _NVMEOF_SECURE_CHANNEL_PROTOCOL {

    NvmeofSecureChannelConcatNone    = 0x00,
    NvmeofSecureChannelConcatWithTLS = 0x01,
    NvmeofSecureChannelNewTLSPSK     = 0x02,
    NvmeofSecureChannelReplaceTLSPSK = 0x02,

} NVMEOF_SECURE_CHANNEL_PROTOCOL;

//
// Authentication Failure Reason Codes (RCODE)
//
typedef enum _NVMEOF_AUTH_FAIL_REASON_CODE {

    NvmeofAuthFailureReasonFailed = 0x01,

} NVMEOF_AUTH_FAIL_REASON_CODE;

//
// Authentication Failure Reason Explanations (RCODEEX)
//
typedef enum _NVMEOF_AUTH_FAIL_REASON_EXPLANATION {

    NvmeofAuthFailed                      = 0x01,
    NvmeofAuthProtocolNotUsable           = 0x02,
    NvmeofAuthSecureChannelConcatMismatch = 0x03,
    NvmeofAuthHashFunctionNotUsable       = 0x04,
    NvmeofAuthDHGroupNotUsable            = 0x05,
    NvmeofAuthIncorrectPayload            = 0x06,
    NvmeofAuthIncorrectProtocolMessage    = 0x07

} NVMEOF_AUTH_FAIL_REASON_EXPLANATION;

//
// Authentication Negotiate message
//
typedef struct _NVMEOF_AUTH_NEGOTIATE {

    UCHAR AUTH_TYPE;  // NVMEOF_AUTH_TYPE: NvmeofAuthTypeCommonMessages
    UCHAR AUTH_ID;    // NVMEOF_AUTH_ID: NvmeofAuthIdNegotiate

    USHORT Reserved0;
    USHORT T_ID;      // Transaction identifier

    UCHAR SC_C;       // NVMEOF_SECURE_CHANNEL_PROTOCOL
    UCHAR NAPD;       // Number of authentication protocol descriptors

    //
    // Followed by one or more authentication
    // protocol descriptors
    //

} NVMEOF_AUTH_NEGOTIATE, *PNVMEOF_AUTH_NEGOTIATE;

//
// Authentication Failure message
//
typedef struct _NVMEOF_AUTH_FAILURE {

    UCHAR AUTH_TYPE;  // NVMEOF_AUTH_TYPE: NvmeofAuthTypeCommonMessages
    UCHAR AUTH_ID;    // NVMEOF_AUTH_ID: NvmeofAuthIdFailure1, NvmeofAuthIdFailure2

    USHORT Reserved0;
    USHORT T_ID;      // Transaction identifier

    UCHAR ReasonCode;        // NVMEOF_AUTH_FAIL_REASON_CODE
    UCHAR ReasonExplanation; // NVMEOF_AUTH_FAIL_REASON_EXPLANATION

} NVMEOF_AUTH_FAILURE, *PNVMEOF_AUTH_FAILURE;

C_ASSERT(sizeof(NVMEOF_AUTH_FAILURE) == 8);

//
// DH-HMAC-CHAP Protocol definitions
//
#define NVMEOF_DHCHAP_PROTOCOL_ID           0x01

#define NVMEOF_DHCHAP_PREFIX_V1             "DHHC-1:"

typedef enum _NVMEOF_AUTH_DHCHAP_HASH_ID {

    NvmeofAuthDHCHAPHashReserved = 0x00,
    NvmeofAuthDHCHAPHashSha256   = 0x01,
    NvmeofAuthDHCHAPHashSha384   = 0x02,
    NvmeofAuthDHCHAPHashSha512   = 0x03,
    NvmeofAuthDHCHAPHashMax      = 0xFF

} NVMEOF_AUTH_DHCHAP_HASH_ID;

typedef enum _NVMEOF_AUTH_DHCHAP_GROUP_ID {

    NvmeofAuthDHCHAPGroupNull    = 0x00,
    NvmeofAuthDHCHAPGroup2048    = 0x01,
    NvmeofAuthDHCHAPGroup3072    = 0x02,
    NvmeofAuthDHCHAPGroup4096    = 0x03,
    NvmeofAuthDHCHAPGroup6144    = 0x04,
    NvmeofAuthDHCHAPGroup8192    = 0x05,
    NvmeofAuthDHCHAPGroupMax     = 0xFF

} NVMEOF_AUTH_DHCHAP_GROUP_ID;

UCHAR
FORCEINLINE
NVMEOF_AUTH_GET_HASH_LENGTH (
    _In_ UCHAR HashId
    )
{
    UCHAR Length = 0;

    switch(HashId) {

        case NvmeofAuthDHCHAPHashSha256:
            Length = 32;
            break;

        case NvmeofAuthDHCHAPHashSha384:
            Length = 48;
            break;

        case NvmeofAuthDHCHAPHashSha512:
            Length = 64;
            break;
    }

    return Length;
}

typedef struct _NVMEOF_AUTH_DHCHAP_DESCRIPTOR {

    UCHAR AuthId; // Authentication protocol identifier (NVMEOF_DHCHAP_PROTOCOL_ID)
    UCHAR Reserved0;

    UCHAR HALEN; // HashIDList Length: Number of hash function identifiers (1 to 30)
    UCHAR DHLEN; // DHgIDList Length: Number of Diffie-Hellman group identifiers (1 to 30)

    UCHAR IdList[60]; // List of HashIDList (NVMEOF_AUTH_DHCHAP_HASH_ID)
                      // and DHgIDList (NVMEOF_AUTH_DHCHAP_GROUP_ID)

} NVMEOF_AUTH_DHCHAP_DESCRIPTOR, *PNVMEOF_AUTH_DHCHAP_DESCRIPTOR;

C_ASSERT(sizeof(NVMEOF_AUTH_DHCHAP_DESCRIPTOR) == 64);

typedef struct _NVMEOF_AUTH_DHCHAP_CHALLENGE {

    UCHAR AUTH_TYPE;  // NVMEOF_AUTH_TYPE: NvmeofAuthTypeDHCHAPMessages
    UCHAR AUTH_ID;    // NVMEOF_AUTH_ID: NvmeofAuthIdChallenge

    USHORT Reserved0;
    USHORT T_ID;      // Transaction identifier

    UCHAR HL; // Hash Length, Length in bytes of the selected hash function
    UCHAR Reserved1;

    UCHAR HashID; // Identifier of selected hash function
    UCHAR DHgID; // Identifier of selected Diffie-Hellman group

    USHORT DHVLEN; // DH Value Length, Length in bytes of DH value.
                   // If no DH value is included in the message,
                   // then this field is cleared to 0h.
                   // This should be a multiple of 4.

    ULONG SEQNUM; // Sequence Number

    //
    // Followed by Challenge Value (CVAL) bytes
    // of HL length
    //

    //
    // Followed by DH Value (DHV) bytes
    // of DHVLEN length
    //

} NVMEOF_AUTH_DHCHAP_CHALLENGE, *PNVMEOF_AUTH_DHCHAP_CHALLENGE;

#define NVMEOF_DHCHAP_REPLY_CVAL_NOTVALID         0x00
#define NVMEOF_DHCHAP_REPLY_CVAL_VALID            0x01

typedef struct _NVMEOF_AUTH_DHCHAP_REPLY {

    UCHAR AUTH_TYPE;  // NVMEOF_AUTH_TYPE: NvmeofAuthTypeDHCHAPMessages
    UCHAR AUTH_ID;    // NVMEOF_AUTH_ID: NvmeofAuthIdReply

    USHORT Reserved0;
    USHORT T_ID;      // Transaction identifier

    UCHAR HL; // Hash Length, Length in bytes of the selected hash function
    UCHAR Reserved1;

    UCHAR CVALID; // Challenge Valid
    UCHAR Reserved2;

    USHORT DHVLEN; // DH Value Length, Length in bytes of DH value.
                   // If no DH value is included in the message,
                   // then this field is cleared to 0h.
                   // This should be a multiple of 4.

    ULONG SEQNUM; // Sequence Number

    //
    // Followed by Response Value (RVAL) bytes
    // of HL length
    //

    //
    // Followed by Challenge Value (CVAL) bytes
    // of HL length
    //

    //
    // Followed by DH Value (DHV) bytes
    // of DHVLEN length
    //

} NVMEOF_AUTH_DHCHAP_REPLY, *PNVMEOF_AUTH_DHCHAP_REPLY;

#define NVMEOF_DHCHAP_SUCCESS1_RVAL_NOTVALID      0x00
#define NVMEOF_DHCHAP_SUCCESS1_RVAL_VALID         0x01

typedef struct _NVMEOF_AUTH_DHCHAP_SUCCESS1 {

    UCHAR AUTH_TYPE;  // NVMEOF_AUTH_TYPE: NvmeofAuthTypeDHCHAPMessages
    UCHAR AUTH_ID;    // NVMEOF_AUTH_ID: NvmeofAuthIdSuccess1

    USHORT Reserved0;
    USHORT T_ID;      // Transaction identifier

    UCHAR HL; // Hash Length, Length in bytes of the selected hash function
    UCHAR Reserved1;

    UCHAR RVALID; // Response Valid
    UCHAR Reserved2[7];

    //
    // Followed by Response Value (RVAL) bytes
    // of HL length
    //

} NVMEOF_AUTH_DHCHAP_SUCCESS1, *PNVMEOF_AUTH_DHCHAP_SUCCESS1;

typedef struct _NVMEOF_AUTH_DHCHAP_SUCCESS2 {

    UCHAR AUTH_TYPE;  // NVMEOF_AUTH_TYPE: NvmeofAuthTypeDHCHAPMessages
    UCHAR AUTH_ID;    // NVMEOF_AUTH_ID: NvmeofAuthIdSuccess2

    USHORT Reserved0;
    USHORT T_ID;      // Transaction identifier

    UCHAR Reserved1[10];

} NVMEOF_AUTH_DHCHAP_SUCCESS2, *PNVMEOF_AUTH_DHCHAP_SUCCESS2;

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4214)
#pragma warning(default:4201)
#pragma warning(default:4200)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_STORAGE) */
#pragma endregion

#endif //NVME_INCLUDED

