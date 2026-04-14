/*=============================================================================

    Copyright (c) 1998  Microsoft Corporation

    Module Name:

        clfs.h

    Abstract:

        Header file containing all publicly defined data structures for the
        common log file system.

    Author:

        Dexter Bradshaw    [DexterB]   09-Dec-1998


    Revision History:

=============================================================================*/

#include <winapifamily.h>

#pragma region Desktop Family or BootableSku Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_BOOTABLESKU)

// begin_wdm
#ifndef _CLFS_PUBLIC_H_
#define _CLFS_PUBLIC_H_
// end_wdm

// begin_wdm
#if !defined(CLFSUSER_API)

#if defined(__CLFSUSER_EXPORTS__)
#define CLFSUSER_API
#else
#define CLFSUSER_API __declspec(dllimport)
#endif

#endif
// end_wdm

#ifndef CLFS_KERNEL_MODE

#include <stdio.h>

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// PFILE
//
// Define PFILE to be a pointer to _iobuf *
//

typedef FILE *PFILE, **PPFILE;
typedef DWORD CLFSSTATUS;

#   define ClfsLsnEqual             LsnEqual
#   define ClfsLsnLess              LsnLess
#   define ClfsLsnGreater           LsnGreater
#   define ClfsLsnNull              LsnNull
#   define ClfsLsnCreate            LsnCreate
#   define ClfsLsnContainer         LsnContainer
#   define ClfsLsnBlockOffset       LsnBlockOffset
#   define ClfsLsnRecordSequence    LsnRecordSequence
#   define ClfsLsnInvalid           LsnInvalid
#   define ClfsLsnIncrement         LsnIncrement

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* CLFS_KERNEL_MODE */

// begin_wdm

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)

//
// FILE_ATTRIBUTE_DEDICATED is defined as FILE_ATTRIBUTE_TEMPORARY.
//

#define FILE_ATTRIBUTE_DEDICATED    FILE_ATTRIBUTE_TEMPORARY

//
// Container name and container size extended attribute entry names.
//

#define EA_CONTAINER_NAME           "ContainerName"
#define EA_CONTAINER_SIZE           "ContainerSize"

//
// Base log file name 3-letter extension.
//

#define CLFS_BASELOG_EXTENSION      L".blf"

//
// Common log file system public flags and constants.
//

#define CLFS_FLAG_NO_FLAGS              0x00000000      // No flags.
#define CLFS_FLAG_FORCE_APPEND          0x00000001      // Flag to force an append to log queue
#define CLFS_FLAG_FORCE_FLUSH           0x00000002      // Flag to force a log flush
#define CLFS_FLAG_USE_RESERVATION       0x00000004      // Flag to charge a data append to reservation
#define CLFS_FLAG_REENTRANT_FILE_SYSTEM 0x00000008      // Kernel mode create flag indicating a re-entrant file system.
#define CLFS_FLAG_NON_REENTRANT_FILTER  0x00000010      // Kernel mode create flag indicating non-reentrant filter.
#define CLFS_FLAG_REENTRANT_FILTER      0x00000020      // Kernel mode create flag indicating reentrant filter.
#define CLFS_FLAG_IGNORE_SHARE_ACCESS   0x00000040      // Kernel mode create flag indicating IO_IGNORE_SHARE_ACCESS_CHECK semantics.
#define CLFS_FLAG_READ_IN_PROGRESS      0x00000080      // Flag indicating read in progress and not completed.
#define CLFS_FLAG_MINIFILTER_LEVEL      0x00000100      // Kernel mode create flag indicating mini-filter target.
#define CLFS_FLAG_HIDDEN_SYSTEM_LOG     0x00000200      // Kernel mode create flag indicating the log and containers should be marked hidden & system.
//      CLFS_FLAG_MAX_VALUE             0x00800000      // Highest value allowed for public flags.
/* NOTE that the uppper 8 bits of CLFS_FLAG values are reserved for internal use. */
// 
// Marshalling Context Flag
//
#define CLFS_MARSHALLING_FLAG_NONE                0x00000000     // No flags
#define CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT   0x00000001     // Flag to disable mashalling buffer intialization

//
// Flag indicating all CLFS I/O will be targeted to an intermediate level of the I/O stack
//

#define CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL CLFS_FLAG_NON_REENTRANT_FILTER
    
//
// Flag indicating all CLFS I/O will be targeted to the top level of the I/O stack
//

#define CLFS_FLAG_FILTER_TOP_LEVEL          CLFS_FLAG_REENTRANT_FILTER

//
// CLFS_CONTAINER_INDEX
//
// Index into the container table.
//

typedef ULONG                       CLFS_CONTAINER_ID;
typedef CLFS_CONTAINER_ID           *PCLFS_CONTAINER_ID;
typedef CLFS_CONTAINER_ID           **PPCLFS_CONTAINER_ID;

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#ifdef __CLFS_PRIVATE_LSN__

#include <clfslsn.h>

#else

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)

//
// CLS_LSN
//

typedef struct _CLS_LSN
{

    ULONGLONG               Internal;

} CLS_LSN, *PCLS_LSN, **PPCLS_LSN;

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* __CLFS_PRIVATE_LSN__ */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)

//
// Alias CLS prefixed types with CLFS prefixes.
//

typedef CLS_LSN CLFS_LSN;
typedef CLFS_LSN *PCLFS_LSN, **PPCLFS_LSN;

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#ifdef __cplusplus
extern "C"
{
#endif /* __cplusplus */

// end_wdm

//
// Definition of special LSN's: CLFS_LSN_INVALID and CLFS_LSN_NULL. Note that
// [CLFS_LSN_NULL, CLFS_LSN_INVALID) define the only valid LSN range.  LSN values
// are strictly monotonic increasing.
//
#ifdef CLFS_KERNEL_MODE

#if defined __CLFS_SUPPORT_LIBRARY__

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
extern const CLFS_LSN CLFS_LSN_INVALID;
extern const CLFS_LSN CLFS_LSN_NULL;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#elif defined __CLFSUSER_EXPORTS__

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
extern __declspec(dllexport) const CLFS_LSN CLFS_LSN_INVALID;
extern __declspec(dllexport) const CLFS_LSN CLFS_LSN_NULL;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#   else

// begin_wdm
#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
extern __declspec(dllimport) const CLFS_LSN CLFS_LSN_INVALID;
extern __declspec(dllimport) const CLFS_LSN CLFS_LSN_NULL;
#endif /* NTDDI_VERSION || _WIN32_WINNT */
// end_wdm 

#   endif /* __CLFSUSER_EXPORTS__ */

#else

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
extern CLFSUSER_API const CLFS_LSN CLFS_LSN_INVALID;
extern CLFSUSER_API const CLFS_LSN CLFS_LSN_NULL;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* CLFS_KERNEL_MODE */

// begin_wdm

#ifdef __cplusplus
}
#endif /* __cplusplus */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)

//
// CLS_RECORD_TYPE
//
// Definition of record types.
//

#ifdef __cplusplus

const UCHAR  ClfsNullRecord          =   0x00;           // Null record type.        
const UCHAR  ClfsDataRecord          =   0x01;           // Client data record.
const UCHAR  ClfsRestartRecord       =   0x02;           // Restart record.


// Valid client records are restart and data records.

const UCHAR  ClfsClientRecord        =   0x03; 

#else

#define ClfsNullRecord                  0x00            // Null record type.        
#define ClfsDataRecord                  0x01            // Client data record.
#define ClfsRestartRecord               0x02            // Restart record.


// Valid client records are restart and data records.

#define ClfsClientRecord (ClfsDataRecord|ClfsRestartRecord) 

#endif /* _cplusplus */

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Log container path prefix indicating the log container's location is
// actually a stream inside of the BLF.
//

#ifdef _cplusplus

const LPCWSTR CLFS_CONTAINER_STREAM_PREFIX     = L"%BLF%:"

#else

#define CLFS_CONTAINER_STREAM_PREFIX             L"%BLF%:"

#endif /* _cplusplus */

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Log container path prefix indicating the log container's location is
// relative to the base log file (BLF) and not an absolute path.
// Paths which do not being with said prefix are absolute paths.
//

#ifdef _cplusplus

const LPCWSTR CLFS_CONTAINER_RELATIVE_PREFIX    = L"%BLF%\\"

#else

#define CLFS_CONTAINER_RELATIVE_PREFIX            L"%BLF%\\"

#endif /* _cplusplus */

#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Alias CLS prefix with CLFS prefixes.
//

typedef UCHAR CLS_RECORD_TYPE, *PCLS_RECORD_TYPE, **PPCLS_RECORD_TYPE;
typedef CLS_RECORD_TYPE CLFS_RECORD_TYPE, *PCLFS_RECORD_TYPE, **PPCLFS_RECORD_TYPE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLS_CONTEXT_MODE
//
// The context mode specifies the direction and access methods used to scan the
// log file. 
//

typedef enum _CLS_CONTEXT_MODE
{
    ClsContextNone = 0x00,
    ClsContextUndoNext,
    ClsContextPrevious,
    ClsContextForward

} CLS_CONTEXT_MODE, *PCLS_CONTEXT_MODE, **PPCLS_CONTEXT_MODE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef enum _CLFS_CONTEXT_MODE
{
    ClfsContextNone = 0x00,
    ClfsContextUndoNext,
    ClfsContextPrevious,
    ClfsContextForward

} CLFS_CONTEXT_MODE, *PCLFS_CONTEXT_MODE, **PPCLFS_CONTEXT_MODE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFSD_NODE_ID
//
// Common log file system node identifier.  Every CLFS file system
// structure has a node identity and type.  The node type is a signature
// field while the size is used in for consistency checking.
//

typedef struct _CLFS_NODE_ID
{
    ULONG   cType;                                      // CLFS node type.
    ULONG   cbNode;                                     // CLFS node size.

} CLFS_NODE_ID, *PCLFS_NODE_ID;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
//  CLS_WRITE_ENTRY
//
// Write entry specifying the contents of a user buffer and length that are
// marshaled in the space reservation and append interface of the CLS API.
//

typedef struct _CLS_WRITE_ENTRY
{
    PVOID Buffer;
    ULONG ByteLength;
} CLS_WRITE_ENTRY, *PCLS_WRITE_ENTRY, **PPCLS_WRITE_ENTRY;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef CLS_WRITE_ENTRY CLFS_WRITE_ENTRY;
typedef CLFS_WRITE_ENTRY *PCLFS_WRITE_ENTRY, **PPCLFS_WRITE_ENTRY;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_LOG_ID
// 
// A log identifier is a GUID that describes uniquely a physical log file.
//

typedef GUID CLFS_LOG_ID;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_INFORMATION
//
// Logical log file information structure describing either virtual or physical log
// file data, depending on the type of information queried.
//

typedef struct _CLS_INFORMATION
{
    LONGLONG TotalAvailable;                            // Total log data space available.
    LONGLONG CurrentAvailable;                          // Usable space in the log file.
    LONGLONG TotalReservation;                       // Space reserved for UNDO's (aggregate for physical log)
    ULONGLONG BaseFileSize;                             // Size of the base log file.
    ULONGLONG ContainerSize;                            // Uniform size of log containers.
    ULONG TotalContainers;                              // Total number of containers.
    ULONG FreeContainers;                               // Number of containers not in active log.
    ULONG TotalClients;                                 // Total number of clients.
    ULONG Attributes;                                   // Log file attributes.
    ULONG FlushThreshold;                               // Log file flush threshold.
    ULONG SectorSize;                                   // Underlying container sector size.
    CLS_LSN MinArchiveTailLsn;                          // Marks the global archive tail.
    CLS_LSN BaseLsn;                                    // Start of the active log region.
    CLS_LSN LastFlushedLsn;                             // Last flushed LSN in active log.
    CLS_LSN LastLsn;                                    // End of active log region.
    CLS_LSN RestartLsn;                                 // Location of restart record.
    GUID Identity;                                      // Unique identifier for the log.
} CLS_INFORMATION, *PCLS_INFORMATION, *PPCLS_INFORMATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Alias CLS prefixes with CLS prefixes.
//

typedef CLS_INFORMATION CLFS_INFORMATION;
typedef CLFS_INFORMATION *PCLFS_INFORMATION, *PPCLFS_INFORMATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */
/*
//
// CLFS_CLIENT_INFORMATION
// 
// The client information structure maintains client-based log metadata.
//

typedef struct _CLS_CLIENT_INFORMATION
{
    CLS_INFORMATION ClfsInfo;                           // Contains base log file information.
    ULONG ClientAttributes;                             // Virtual log file attributes.
    LONGLONG ClientUndoCommitment;                      // Max. undo commitment for client.
    CLS_LSN ClientArchiveTailLsn;                       // Marks the client archive tail.
    CLS_LSN ClientBaseLsn;                              // Min. client LSN in active log region.
    CLS_LSN ClientLastLsn;                              // Max. client LSN in active log region.
    CLS_LSN ClientRestartLsn;                           // Location of restart record.

} CLS_CLIENT_INFORMATION, *PCLS_CLIENT_INFORMATION, **PPCLS_CLIENT_INFORMATION;

//
// Alias CLS prefixes with CLS prefixes.
//

typedef CLS_CLIENT_INFORMATION CLFS_CLIENT_INFORMATION;
typedef CLFS_CLIENT_INFORMATION *PCLFS_CLIENT_INFORMATION, *PPCLFS_CLIENT_INFORMATION;
*/

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_LOG_NAME_INFORMATION
// 
// The client information structure stores the name of a log.  It is used
// to communicate ClfsLogNameInformation and ClfsLogPhysicalNameInformation.
//

typedef struct _CLFS_LOG_NAME_INFORMATION
{

    USHORT NameLengthInBytes;
    WCHAR  Name[1];

} CLFS_LOG_NAME_INFORMATION, *PCLFS_LOG_NAME_INFORMATION, **PPCLFS_LOG_NAME_INFORMATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_STREAM_ID_INFORMATION
// 
// The client information structure provides a permanent identifier unique
// to the log for the stream in question.
//

typedef struct _CLFS_STREAM_ID_INFORMATION
{

    UCHAR StreamIdentifier;

} CLFS_STREAM_ID_INFORMATION, *PCLFS_STREAM_ID_INFORMATION, **PPCLFS_STREAM_ID_INFORMATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
//
// CLFS_PHYSICAL_LSN_INFORMATION
// 
// An information structure that describes a virtual:physical LSN pairing 
// for the stream identified in the structure.
//
#pragma pack(push,8)
typedef struct _CLFS_PHYSICAL_LSN_INFORMATION
{
    UCHAR          StreamIdentifier;
    CLFS_LSN       VirtualLsn;
    CLFS_LSN       PhysicalLsn;

} CLFS_PHYSICAL_LSN_INFORMATION, *PCLFS_PHYSICAL_LSN_INFORMATION;
#pragma pack(pop)
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLS_CONTAINER_STATE
//
// At any point in time a container could be inactive or uninitialized, active,
// pending deletion from the list of free containers, pending archival, or 
// pending deletion while waiting to be archived.
//

typedef UINT32 CLS_CONTAINER_STATE, *PCLS_CONTAINER_STATE, *PPCLS_CONTAINER_STATE;
typedef CLS_CONTAINER_STATE  CLFS_CONTAINER_STATE, *PCLFS_CONTAINER_STATE, *PPCLFS_CONTAINER_STATE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#ifdef __cplusplus

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
const CLFS_CONTAINER_STATE  ClsContainerInitializing            = 0x01;
const CLFS_CONTAINER_STATE  ClsContainerInactive                = 0x02;
const CLFS_CONTAINER_STATE  ClsContainerActive                  = 0x04;
const CLFS_CONTAINER_STATE  ClsContainerActivePendingDelete     = 0x08;
const CLFS_CONTAINER_STATE  ClsContainerPendingArchive          = 0x10;
const CLFS_CONTAINER_STATE  ClsContainerPendingArchiveAndDelete = 0x20;

const CLFS_CONTAINER_STATE  ClfsContainerInitializing           = 0x01;
const CLFS_CONTAINER_STATE  ClfsContainerInactive               = 0x02;
const CLFS_CONTAINER_STATE  ClfsContainerActive                 = 0x04;
const CLFS_CONTAINER_STATE  ClfsContainerActivePendingDelete    = 0x08;
const CLFS_CONTAINER_STATE  ClfsContainerPendingArchive         = 0x10;
const CLFS_CONTAINER_STATE  ClfsContainerPendingArchiveAndDelete= 0x20;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#else

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
#define ClsContainerInitializing                                  0x01
#define ClsContainerInactive                                      0x02
#define ClsContainerActive                                        0x04
#define ClsContainerActivePendingDelete                           0x08
#define ClsContainerPendingArchive                                0x10
#define ClsContainerPendingArchiveAndDelete                       0x20

#define ClfsContainerInitializing                                 0x01
#define ClfsContainerInactive                                     0x02
#define ClfsContainerActive                                       0x04
#define ClfsContainerActivePendingDelete                          0x08
#define ClfsContainerPendingArchive                               0x10
#define ClfsContainerPendingArchiveAndDelete                      0x20
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* __cplusplus */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_MAX_CONTAINER_INFO
//
// The maximum length, in bytes, of the FileName field in the CLFS
// container information structure.
//

#ifdef __cplusplus

const ULONG CLFS_MAX_CONTAINER_INFO = (256);

#else

#define CLFS_MAX_CONTAINER_INFO       (256)

#endif /* __cplusplus */

#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLS_CONTAINER_INFORMATION
//
// This structure defines a container descriptor.  The descriptor specifies the
// container's creation and access times, size, file system name, file system
// attributes, state, minimum, and maximum LSNs.
//

typedef struct _CLS_CONTAINER_INFORMATION
{
    ULONG FileAttributes;                    // File system attribute flag.
    ULONGLONG CreationTime;                  // File creation time.
    ULONGLONG LastAccessTime;                // Last time container was read/written.
    ULONGLONG LastWriteTime;                 // Last time container was written.
    LONGLONG ContainerSize;                  // Size of container in bytes.
    ULONG FileNameActualLength;              // Length of the actual file name.
    ULONG FileNameLength;                    // Length of file name in buffer
    WCHAR FileName [CLFS_MAX_CONTAINER_INFO];// File system name for container.
    CLFS_CONTAINER_STATE State;              // Current state of the container.
    CLFS_CONTAINER_ID PhysicalContainerId;   // Physical container identifier.
    CLFS_CONTAINER_ID LogicalContainerId;    // Logical container identifier.

} CLS_CONTAINER_INFORMATION, *PCLS_CONTAINER_INFORMATION, **PPCLS_CONTAINER_INFORMATION;

//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef CLS_CONTAINER_INFORMATION CLFS_CONTAINER_INFORMATION;
typedef CLFS_CONTAINER_INFORMATION *PCLFS_CONTAINER_INFORMATION, **PPCLFS_CONTAINER_INFORMATION;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_LOG_INFORMATION_CLASS
//
// The information class specifies the kind of information a caller
// wishes to query or set on a log file.
//

typedef enum _CLS_LOG_INFORMATION_CLASS
{

    ClfsLogBasicInformation = 0x00,         // For virtual or physical logs, indicates the respective basic information.
    ClfsLogBasicInformationPhysical,        // Always indicates physical log basic information.
    ClfsLogPhysicalNameInformation,         // Always indicates physical name information.
    ClfsLogStreamIdentifierInformation,     // Virtual/physical log agnostic.
#if (NTDDI_VERSION >= NTDDI_VISTA) || (_WIN32_WINNT >= _WIN32_WINNT_LONGHORN)
    ClfsLogSystemMarkingInformation,        // Count of system marking references.
    ClfsLogPhysicalLsnInformation           // Maps virtual LSNs to physical LSNs; only valid for physical logs.
#endif /* NTDDI_VERSION || _WIN32_WINNT */

} CLS_LOG_INFORMATION_CLASS, *PCLS_LOG_INFORMATION_CLASS, **PPCLS_LOG_INFORMATION_CLASS;

//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef CLS_LOG_INFORMATION_CLASS CLFS_LOG_INFORMATION_CLASS;
typedef CLFS_LOG_INFORMATION_CLASS *PCLFS_LOG_INFORMATION_CLASS, **PPCLFS_LOG_INFORMATION_CLASS;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLS_IOSTATS_CLASS
//
// Enumerated type defining the class of I/O statistics.
//

typedef enum _CLS_IOSTATS_CLASS
{
    ClsIoStatsDefault = 0x0000,
    ClsIoStatsMax     = 0xFFFF

} CLS_IOSTATS_CLASS, *PCLS_IOSTATS_CLASS, **PPCLS_IOSTATS_CLASS;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_IOSTATS_CLASS
//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef enum _CLFS_IOSTATS_CLASS
{
    ClfsIoStatsDefault = 0x0000,
    ClfsIoStatsMax     = 0xFFFF

} CLFS_IOSTATS_CLASS, *PCLFS_IOSTATS_CLASS, **PPCLFS_IOSTATS_CLASS;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLS_IO_STATISTICS
//
// This structure defines I/O performance counters particular to a log file.  It consists
// of a header followed by the I/O statistics counters.  The header is being ignored for
// now.
//

typedef struct _CLS_IO_STATISTICS_HEADER
{
    UCHAR                ubMajorVersion;     // Major version of the statistics buffer.
    UCHAR                ubMinorVersion;     // Minor version of the statistics buffer.
    CLFS_IOSTATS_CLASS  eStatsClass;        // I/O statistics class.
    USHORT              cbLength;           // Length of the statistics buffer.                     
    ULONG               coffData;           // Offset of statistics counters.

} CLS_IO_STATISTICS_HEADER, *PCLS_IO_STATISTICS_HEADER, **PPCLS_IO_STATISTICS_HEADER;

//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef CLS_IO_STATISTICS_HEADER CLFS_IO_STATISTICS_HEADER;
typedef CLFS_IO_STATISTICS_HEADER *PCLFS_IO_STATISTICS_HEADER, **PPCLFS_IO_STATISTICS_HEADER;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
typedef struct _CLS_IO_STATISTICS
{
    CLS_IO_STATISTICS_HEADER hdrIoStats;    // Statistics buffer header.
    ULONGLONG cFlush;                       // Flush count.
    ULONGLONG cbFlush;                      // Cumulative number of bytes flushed.
    ULONGLONG cMetaFlush;                   // Metadata flush count.
    ULONGLONG cbMetaFlush;                  // Cumulative number of metadata bytes flushed.

} CLS_IO_STATISTICS, *PCLS_IO_STATISTICS, **PPCLS_IO_STATISTICS;

//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef CLS_IO_STATISTICS CLFS_IO_STATISTICS;
typedef CLFS_IO_STATISTICS *PCLFS_IO_STATISTICS, **PPCLFS_IO_STATISTICS;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_SCAN_MODE
//
// Container scan mode flags.
//

#ifdef __cplusplus

const   UCHAR CLFS_SCAN_INIT         =   0x01;
const   UCHAR CLFS_SCAN_FORWARD      =   0x02;
const   UCHAR CLFS_SCAN_BACKWARD     =   0x04;
const   UCHAR CLFS_SCAN_CLOSE        =   0x08;
const   UCHAR CLFS_SCAN_INITIALIZED  =   0x10;
const   UCHAR CLFS_SCAN_BUFFERED     =   0x20;

#else

#define CLFS_SCAN_INIT                  0x01
#define CLFS_SCAN_FORWARD               0x02
#define CLFS_SCAN_BACKWARD              0x04
#define CLFS_SCAN_CLOSE                 0x08
#define CLFS_SCAN_INITIALIZED           0x10
#define CLFS_SCAN_BUFFERED              0x20

#endif

typedef UCHAR CLFS_SCAN_MODE, *PCLFS_SCAN_MODE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


// end_wdm

#ifdef CLFS_KERNEL_MODE

// begin_wdm
#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)

//
// CLFS_SCAN_CONTEXT
//
// Container scan context for scanning all containers in a given physical log
// file.
//

//
// The log file object wraps an NT file object and the size of the structure. 
// The log file object may be modified in the near future and there should be no
// dependencies on the size of the structure itself.
//

typedef FILE_OBJECT LOG_FILE_OBJECT, *PLOG_FILE_OBJECT, **PPLOG_FILE_OBJECT;

#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(push)
#pragma warning(disable:4324) // structure padded due to __declspec(align())
#endif
#endif

typedef struct _CLS_SCAN_CONTEXT
{
    CLFS_NODE_ID cidNode;
    PLOG_FILE_OBJECT plfoLog;
    __declspec(align(8)) ULONG cIndex;
    __declspec(align(8)) ULONG cContainers;
    __declspec(align(8)) ULONG cContainersReturned;
    __declspec(align(8)) CLFS_SCAN_MODE eScanMode;
    __declspec(align(8)) PCLS_CONTAINER_INFORMATION pinfoContainer;
    
} CLS_SCAN_CONTEXT, *PCLS_SCAN_CONTEXT, **PPCLS_SCAN_CONTEXT;

#if defined(_MSC_VER)
#if (_MSC_VER >= 1200)
#pragma warning(pop)
#endif
#endif

#endif /* NTDDI_VERSION || _WIN32_WINNT */

// end_wdm

#else

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
typedef struct _CLS_SCAN_CONTEXT
{
    CLFS_NODE_ID cidNode;
    HANDLE hLog;
    __declspec(align(8)) ULONG cIndex;
    __declspec(align(8)) ULONG cContainers;
    __declspec(align(8)) ULONG cContainersReturned;
    __declspec(align(8)) CLFS_SCAN_MODE eScanMode;
    __declspec(align(8)) PCLS_CONTAINER_INFORMATION pinfoContainer;
    
} CLS_SCAN_CONTEXT, *PCLS_SCAN_CONTEXT, **PPCLS_SCAN_CONTEXT;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* CLFS_KERNEL_MODE */

// begin_wdm

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// Alias all CLS prefixes with CLFS prefixes.
//

typedef CLS_SCAN_CONTEXT CLFS_SCAN_CONTEXT;
typedef CLFS_SCAN_CONTEXT *PCLFS_SCAN_CONTEXT, **PPCLFS_SCAN_CONTEXT;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_ARCHIVE_DESCRIPTOR
//
// Log archive descriptors describe the set of discrete but logically
// contiguous disk extents comprising a snapshot of the active log when
// preparing for archival.  Log archive descriptors specify enough information
// for log archive clients directly access the relevant contents of containers
// for archiving and restoring a snapshot of the log.
//

typedef struct _CLS_ARCHIVE_DESCRIPTOR
{
    ULONGLONG coffLow;
    ULONGLONG coffHigh;
    CLS_CONTAINER_INFORMATION infoContainer;

} CLS_ARCHIVE_DESCRIPTOR, *PCLS_ARCHIVE_DESCRIPTOR, **PPCLS_ARCHIVE_DESCRIPTOR;

//
// Alias CLS prefixes with CLFS prefixes.
//

typedef CLS_ARCHIVE_DESCRIPTOR CLFS_ARCHIVE_DESCRIPTOR;
typedef CLFS_ARCHIVE_DESCRIPTOR *PCLFS_ARCHIVE_DESCRIPTOR, **PPCLFS_ARCHIVE_DESCRIPTOR;
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_ALLOCATION_ROUTINE
//
// Allocate a blocks for marshaled reads or writes
//

typedef PVOID (* CLFS_BLOCK_ALLOCATION) (ULONG cbBufferLength, PVOID pvUserContext);

//
// CLFS_DEALLOCATION_ROUTINE
//
// Deallocate buffers allocated by the CLFS_ALLOCATION_ROUTINE.
//

typedef void (* CLFS_BLOCK_DEALLOCATION) (PVOID pvBuffer, PVOID pvUserContext);
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// CLFS_LOG_ARCHIVE_MODE
//
// Describes the archive support behavior for the log.
//

typedef enum _CLFS_LOG_ARCHIVE_MODE
{

    ClfsLogArchiveEnabled = 0x01,
    ClfsLogArchiveDisabled = 0x02

} CLFS_LOG_ARCHIVE_MODE, *PCLFS_LOG_ARCHIVE_MODE;
#endif /* NTDDI_VERSION || _WIN32_WINNT */


//-----------------------------------------------------------------------------
// LSN OPERATORS
//-----------------------------------------------------------------------------

#ifdef __cplusplus
extern "C"
{
#endif


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnEqual
//
//      Method Description:
//
//          Check for the equivalence of LSNs.
//
//      Arguments:
//
//          plsn1   -- first LSN comparator
//          plsn2   -- second LSN comparator
//          
//
//      Return Value:
//
//          TRUE if LSN values are equivalent and FALSE otherwise.
//
//-----------------------------------------------------------------------------

CLFSUSER_API BOOLEAN NTAPI
ClfsLsnEqual
(
    _In_ const CLFS_LSN* plsn1,
    _In_ const CLFS_LSN* plsn2
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnLess
//
//      Method Description:
//
//          Check if LSN1 is less than LSN2.
//
//      Arguments:
//
//          plsn1   -- first LSN comparator
//          plsn2   -- second LSN comparator
//          
//
//      Return Value:
//
//          TRUE if LSN1 is less than LSN2 and FALSE otherwise.
//
//-----------------------------------------------------------------------------

CLFSUSER_API BOOLEAN NTAPI
ClfsLsnLess
(
    _In_ const CLFS_LSN* plsn1,
    _In_ const CLFS_LSN* plsn2
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnGreater
//
//      Method Description:
//
//          Check if LSN1 is  greater than LSN2.
//
//      Arguments:
//
//          plsn1   -- first LSN comparator
//          plsn2   -- second LSN comparator
//          
//
//      Return Value:
//
//          TRUE if LSN1 is greater than LSN2 and FALSE otherwise.
//
//-----------------------------------------------------------------------------

CLFSUSER_API BOOLEAN NTAPI
ClfsLsnGreater
(
    _In_ const CLFS_LSN* plsn1,
    _In_ const CLFS_LSN* plsn2
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnNull (Inline)
//
//      Method Description:
//
//          Check whether or not an LSN is CLFS_LSN_NULL.
//
//      Arguments:
//
//          plsn    -- reference to LSN tested against the NULL value.
//          
//
//      Return Value:
//
//          TRUE if and only if an LSN is equivalent to CLFS_LSN_NULL.  
//          LSNs with the value CLFS_LSN_INVALID will return FALSE.
//
//-----------------------------------------------------------------------------

CLFSUSER_API BOOLEAN NTAPI
ClfsLsnNull
(
    _In_ const CLFS_LSN* plsn
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnContainer (Inline)
//
//      Routine Description:
//
//      Extract the container identifier from the LSN.
//
//      Arguments:
//
//          plsn -- get block offset from this LSN
//
//      Return Value:
//
//          Returns the container identifier for the LSN.
//
//-----------------------------------------------------------------------------

CLFSUSER_API CLFS_CONTAINER_ID NTAPI
ClfsLsnContainer
(
  _In_ const CLFS_LSN* plsn
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnCreate (Inline)
//
//      Routine Description:
//
//      Create an LSN given a log identifier, a container identifier, a block
//      offset and a bucket identifier.  Caller must test for invalid LSN after
//      making this call.
//
//      Arguments:
//
//          cidContainer    -- container identifier
//          offBlock        -- block offset
//          cRecord         -- ordinal number of the record in block
//
//      Return Value:
//
//          Returns a valid LSN if successful, otherwise it returns
//          CLFS_LSN_INVALID
//
//-----------------------------------------------------------------------------

CLFSUSER_API CLFS_LSN NTAPI
ClfsLsnCreate
(
    _In_ CLFS_CONTAINER_ID    cidContainer,
    _In_ ULONG                offBlock,
    _In_ ULONG                cRecord
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnBlockOffset (Inline)
//
//      Routine Description:
//
//      Extract the block offset from the LSN.
//
//      Arguments:
//
//          plsn -- get block offset from this LSN
//
//      Return Value:
//
//          Returns the block offset for the LSN.
//
//-----------------------------------------------------------------------------

CLFSUSER_API ULONG NTAPI
ClfsLsnBlockOffset
(
  _In_ const CLFS_LSN* plsn
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnRecordSequence (Inline)
//
//      Routine Description:
//
//          Extract the bucket identifier from the LSN.
//
//      Arguments:
//
//          plsn    -- get block offset from this LSN
//
//      Return Value:
//
//          Returns the bucket identifier for the LSN.
//
//-----------------------------------------------------------------------------

CLFSUSER_API ULONG NTAPI
ClfsLsnRecordSequence
(
    _In_ const CLFS_LSN* plsn
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnInvalid
//
//      Method Description:
//
//          Check whether or not an LSN is CLFS_LSN_INVALID.
//
//      Arguments:
//
//          plsn    -- reference to LSN tested against CLFS_LSN_INVALID.
//          
//
//      Return Value:
//
//          TRUE if and only if an LSN is equivalent to CLFS_LSN_INVALID.  
//          LSNs with the value CLFS_LSN_NULL will return FALSE.
//
//-----------------------------------------------------------------------------

CLFSUSER_API BOOLEAN NTAPI
ClfsLsnInvalid
(
    _In_ const CLFS_LSN* plsn
);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//-----------------------------------------------------------------------------
// ClfsLsnIncrement
//
//      Method Description:
//
//          Increment and LSN by 1
//
//      Arguments:
//
//          plsn -- LSN to be incremented.
//          
//
//      Return Value:
//
//          A valid LSN next in sequence to the input LSN, if successful.
//          Otherwise, this function returns CLFS_LSN_INVALID.
//
//-----------------------------------------------------------------------------

CLFSUSER_API CLFS_LSN NTAPI
ClfsLsnIncrement (_In_ PCLFS_LSN  plsn);
#endif /* NTDDI_VERSION || _WIN32_WINNT */


#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

#ifdef CLFS_OPERATORS

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// LSN arithmetic increment operator.
//

inline CLFS_LSN
operator++
(
    _Inout_ CLFS_LSN& refLsn
)
{
    //
    // Prefix increment operator.
    //

    refLsn = ClfsLsnIncrement (&refLsn);
    return refLsn;
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
//
// BOOLEAN LSN operators.
//

inline BOOLEAN      
operator<
(
    _In_ const CLFS_LSN& refLsn1, 
    _In_ const CLFS_LSN& refLsn2
)
{
    return (ClfsLsnLess ((PCLFS_LSN) &refLsn1, (PCLFS_LSN) &refLsn2));
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
inline BOOLEAN  
operator>
(
    _In_ const CLFS_LSN& refLsn1, 
    _In_ const CLFS_LSN& refLsn2
)
{
    return (ClfsLsnGreater ((PCLFS_LSN) &refLsn1, (PCLFS_LSN) &refLsn2));
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
inline BOOLEAN  
operator==
(
    _In_ const CLFS_LSN& refLsn1, 
    _In_ const CLFS_LSN& refLsn2
)
{
    return (ClfsLsnEqual ((PCLFS_LSN) &refLsn1, (PCLFS_LSN) &refLsn2));
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
inline BOOLEAN
operator!=
(
    _In_ const CLFS_LSN& refLsn1,
    _In_ const CLFS_LSN& refLsn2
)
{
    return (!ClfsLsnEqual ((PCLFS_LSN) &refLsn1, (PCLFS_LSN) &refLsn2));
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
inline BOOLEAN      
operator<=
(
    _In_ const CLFS_LSN& refLsn1, 
    _In_ const CLFS_LSN& refLsn2
)
{
    return (!ClfsLsnGreater ((PCLFS_LSN) &refLsn1, (PCLFS_LSN) &refLsn2));
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#if (NTDDI_VERSION >= NTDDI_WS03SP1) || (_WIN32_WINNT >= _WIN32_WINNT_WS03)
inline BOOLEAN  
operator>=
(
    _In_ const CLFS_LSN& refLsn1, 
    _In_ const CLFS_LSN& refLsn2
)
{
    return (!ClfsLsnLess ((PCLFS_LSN) &refLsn1, (PCLFS_LSN) &refLsn2));
}
#endif /* NTDDI_VERSION || _WIN32_WINNT */

#endif /* CLFS_OPERATORS */

#endif /* __cplusplus */


#endif /* _CLFS_PUBLIC_H_ */
// end_wdm


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_BOOTABLESKU) */
#pragma endregion

//-----------------------------------------------------------------------------
//                                      END OF FILE
//-----------------------------------------------------------------------------


