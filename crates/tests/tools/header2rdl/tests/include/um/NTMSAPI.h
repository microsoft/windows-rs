/*++

Copyright (c) Microsoft Corporation. All rights reserved

Module Name:

    ntmsapi.h

Abstract:

    This module contains the RSM API prototypes
    and public definitions supported in the API
    The RSM API provides a "C" interface to RSM
    This file can be included in C and C++ code

    This file's  name is historical.  RSM began
    life as NTMS

--*/

#ifndef _INCL_NTMSAPI_H_
#define _INCL_NTMSAPI_H_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) /* padding added after data member */
#endif

#pragma pack(8)

#ifdef __cplusplus
extern "C" {
#endif

typedef GUID  NTMS_GUID;
typedef GUID* LPNTMS_GUID;

#define NTMS_NULLGUID  {0,0,0,{0,0,0,0,0,0,0,0}}
#define NTMS_IS_NULLGUID(id) ((id.Data1==0)&&(id.Data2==0)&&(id.Data3==0)&&\
                                                          (id.Data4[0]==0)&&(id.Data4[1]==0)&&(id.Data4[2]==0)&&\
                                                          (id.Data4[3]==0)&&(id.Data4[4]==0)&&(id.Data4[5]==0)&&\
                                                          (id.Data4[6]==0)&&(id.Data4[7]==0))

//
// Define unicode/ascii specific functions
// This include functions that get strings
//

#ifdef UNICODE
#define OpenNtmsSession OpenNtmsSessionW
#define GetNtmsDeviceName GetNtmsDeviceNameW
#define GetNtmsObjectInformation GetNtmsObjectInformationW
#define SetNtmsObjectInformation SetNtmsObjectInformationW
#define CreateNtmsMediaPool CreateNtmsMediaPoolW
#define GetNtmsMediaPoolName GetNtmsMediaPoolNameW
#define GetNtmsObjectAttribute GetNtmsObjectAttributeW
#define SetNtmsObjectAttribute SetNtmsObjectAttributeW
#define GetNtmsUIOptions GetNtmsUIOptionsW
#define SetNtmsUIOptions SetNtmsUIOptionsW
#define SubmitNtmsOperatorRequest SubmitNtmsOperatorRequestW
#define CreateNtmsMedia CreateNtmsMediaW
#define EjectDiskFromSADrive EjectDiskFromSADriveW
#define GetVolumesFromDrive GetVolumesFromDriveW
#else
#define OpenNtmsSession OpenNtmsSessionA
#define GetNtmsDeviceName GetNtmsDeviceNameA
#define GetNtmsObjectInformation GetNtmsObjectInformationA
#define SetNtmsObjectInformation SetNtmsObjectInformationA
#define CreateNtmsMediaPool CreateNtmsMediaPoolA
#define GetNtmsMediaPoolName GetNtmsMediaPoolNameA
#define GetNtmsObjectAttribute GetNtmsObjectAttributeA
#define SetNtmsObjectAttribute SetNtmsObjectAttributeA
#define GetNtmsUIOptions GetNtmsUIOptionsA
#define SetNtmsUIOptions SetNtmsUIOptionsA
#define SubmitNtmsOperatorRequest SubmitNtmsOperatorRequestA
#define CreateNtmsMedia CreateNtmsMediaA
#define EjectDiskFromSADrive EjectDiskFromSADriveA
#define GetVolumesFromDrive GetVolumesFromDriveA
#endif

#ifndef NTMS_NOREDEF

// WARNING : The object types are provided as an ordered list of NTMS
// database objects.  Do NOT modify without also modifying the object
// metadata table in dbobmeta.cpp

enum NtmsObjectsTypes {
    NTMS_UNKNOWN                    = 0,
    NTMS_OBJECT,
    NTMS_CHANGER,
    NTMS_CHANGER_TYPE,
    NTMS_COMPUTER,
    NTMS_DRIVE,
    NTMS_DRIVE_TYPE,
    NTMS_IEDOOR,
    NTMS_IEPORT,
    NTMS_LIBRARY,
    NTMS_LIBREQUEST,
    NTMS_LOGICAL_MEDIA,
    NTMS_MEDIA_POOL,
    NTMS_MEDIA_TYPE,
    NTMS_PARTITION,
    NTMS_PHYSICAL_MEDIA,
    NTMS_STORAGESLOT,
    NTMS_OPREQUEST,
    NTMS_UI_DESTINATION,

    NTMS_NUMBER_OF_OBJECT_TYPES
};

// async i/o defined for future implementation
typedef struct _NTMS_ASYNC_IO {
    NTMS_GUID   OperationId;                // reserved - do not use
    NTMS_GUID   EventId;                    // reserved - do not use
    DWORD       dwOperationType;            // reserved - do not use
    DWORD       dwResult;                   // completion status
    DWORD       dwAsyncState;               // current state of request
#ifdef MIDL_PASS
    NTMS_HANDLE hEvent;
#else
    HANDLE      hEvent;                     // event handle to signal on status change
#endif                                      // or request completion
    BOOL        bOnStateChange;             // indicate to signal on status change also
} NTMS_ASYNC_IO, *LPNTMS_ASYNC_IO;

enum NtmsAsyncStatus {
    NTMS_ASYNCSTATE_QUEUED          = 0,    // initial state is queued
    NTMS_ASYNCSTATE_WAIT_RESOURCE,
    NTMS_ASYNCSTATE_WAIT_OPERATOR,
    NTMS_ASYNCSTATE_INPROCESS,
    NTMS_ASYNCSTATE_COMPLETE                // complete, check result code
};

enum NtmsAsyncOperations {
    NTMS_ASYNCOP_MOUNT              = 1     // mount operation
};

#endif  // NTMS_NOREDEF

//=======================================================================
// SESSION ESTABLISHMENT
//=======================================================================

#ifndef MIDL_PASS

enum NtmsSessionOptions {
    NTMS_SESSION_QUERYEXPEDITE      = 0x1
};

HANDLE WINAPI OpenNtmsSessionW(
    LPCWSTR lpServer,
    LPCWSTR lpApplication,
    DWORD   dwOptions
    );

HANDLE WINAPI OpenNtmsSessionA(
    LPCSTR lpServer,
    LPCSTR lpApplication,
    DWORD   dwOptions
    );

DWORD WINAPI CloseNtmsSession(
    HANDLE hSession
    );

#endif  // MIDL_PASS

#ifndef NTMS_NOREDEF

//=======================================================================
// MOUNT DEFINITIONS
//=======================================================================

enum NtmsMountOptions {
    NTMS_MOUNT_READ                 = 0x0001,
    NTMS_MOUNT_WRITE                = 0x0002,
    NTMS_MOUNT_ERROR_NOT_AVAILABLE  = 0x0004,
    NTMS_MOUNT_ERROR_IF_UNAVAILABLE = 0x0004,
    NTMS_MOUNT_ERROR_OFFLINE        = 0x0008,
    NTMS_MOUNT_ERROR_IF_OFFLINE     = 0x0008,
    NTMS_MOUNT_SPECIFIC_DRIVE       = 0x0010,
    NTMS_MOUNT_NOWAIT               = 0x0020
};

enum NtmsDismountOptions {
    NTMS_DISMOUNT_DEFERRED          = 0x0001,
    NTMS_DISMOUNT_IMMEDIATE         = 0x0002
};

enum NtmsMountPriority {
    NTMS_PRIORITY_DEFAULT           = 0,
    NTMS_PRIORITY_HIGHEST           = 15,
    NTMS_PRIORITY_HIGH              = 7,
    NTMS_PRIORITY_NORMAL            = 0,
    NTMS_PRIORITY_LOW               = -7,
    NTMS_PRIORITY_LOWEST            = -15
};

typedef struct _NTMS_MOUNT_INFORMATION
{
    DWORD  dwSize;                  // size of structure
#ifdef MIDL_PASS
    [ptr]  LPNTMS_ASYNC_IO lpReserved;
#else
    LPVOID lpReserved;
#endif
} NTMS_MOUNT_INFORMATION, *LPNTMS_MOUNT_INFORMATION;

#endif  // NTMS_NOREDEF

//=======================================================================
// MOUNT AND DISMOUNT APIS
//=======================================================================

#ifndef MIDL_PASS

DWORD WINAPI MountNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    LPNTMS_GUID lpDriveId,
    DWORD dwCount,
    DWORD dwOptions,
    int   dwPriority,
    DWORD dwTimeout,
    LPNTMS_MOUNT_INFORMATION lpMountInformation
    );

DWORD WINAPI DismountNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    DWORD dwCount,
    DWORD dwOptions
    );

#endif  // MIDL_PASS

#ifndef NTMS_NOREDEF

//=======================================================================
// ALLOCATE DEFINITIONS
//=======================================================================

enum NtmsAllocateOptions {
    NTMS_ALLOCATE_NEW               = 0x0001,
    NTMS_ALLOCATE_NEXT              = 0x0002,
    NTMS_ALLOCATE_ERROR_IF_UNAVAILABLE = 0x0004
};

typedef struct _NTMS_ALLOCATION_INFORMATION
{
    DWORD  dwSize;                      // size of structure
#ifdef MIDL_PASS
    [ptr]  LPNTMS_ASYNC_IO lpReserved;  // future async io control
#else
    LPVOID lpReserved;
#endif
    NTMS_GUID AllocatedFrom;            // original source of media (scratch pool, app pool, import etc).
} NTMS_ALLOCATION_INFORMATION, *LPNTMS_ALLOCATION_INFORMATION;

#endif

//=======================================================================
// ALLOCATE AND DEALLOCATE APIS
//=======================================================================

#ifndef MIDL_PASS

DWORD WINAPI AllocateNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaPool,
    LPNTMS_GUID lpPartition,        // optional
    LPNTMS_GUID lpMediaId,          // OUTPUT, media id or operator request id
    DWORD dwOptions,
    DWORD dwTimeout,
    LPNTMS_ALLOCATION_INFORMATION lpAllocateInformation
    );

DWORD WINAPI DeallocateNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    DWORD dwOptions
    );

DWORD WINAPI SwapNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId1,
    LPNTMS_GUID lpMediaId2
    );

DWORD WINAPI AddNtmsMediaType(
    HANDLE hSession,
    LPNTMS_GUID lpMediaTypeId,
    LPNTMS_GUID lpLibId
    );

DWORD WINAPI DeleteNtmsMediaType(
    HANDLE hSession,
    LPNTMS_GUID lpMediaTypeId,
    LPNTMS_GUID lpLibId
    );

DWORD WINAPI ChangeNtmsMediaType(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    LPNTMS_GUID lpPoolId
    );

//=======================================================================
// MEDIA STATES
//=======================================================================

DWORD WINAPI DecommissionNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId
    );

DWORD WINAPI SetNtmsMediaComplete(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId
    );

DWORD WINAPI DeleteNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId
    );

#endif  // MIDL_PASS

#ifndef NTMS_NOREDEF

//=======================================================================
// MEDIA POOLS
//=======================================================================

enum NtmsCreateOptions {
    NTMS_OPEN_EXISTING              = 0x0001,
    NTMS_CREATE_NEW                 = 0x0002,
    NTMS_OPEN_ALWAYS                = 0x0003
};

#endif  // NTMS_NOREDEF

#ifndef MIDL_PASS
#ifdef PRE_SEVIL

DWORD WINAPI CreateNtmsMediaPool(
    HANDLE hSession,
    LPCTSTR lpPoolName,
    LPNTMS_GUID lpMediaType,
    DWORD dwAction,
    LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    LPNTMS_GUID lpPoolId            // OUT
    );

#endif

// SEVILIA
DWORD WINAPI CreateNtmsMediaPoolA(
    HANDLE hSession,
    LPCSTR lpPoolName,
    LPNTMS_GUID lpMediaType,
    DWORD dwAction,
    LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    LPNTMS_GUID lpPoolId            // OUT
    );

DWORD WINAPI CreateNtmsMediaPoolW(
    HANDLE hSession,
    LPCWSTR lpPoolName,
    LPNTMS_GUID lpMediaType,
    DWORD dwAction,
    LPSECURITY_ATTRIBUTES lpSecurityAttributes,
    LPNTMS_GUID lpPoolId            // OUT
    );

DWORD
WINAPI
GetNtmsMediaPoolNameA(
    _In_ HANDLE hSession,
    _In_ LPNTMS_GUID lpPoolId,
    _Out_writes_to_(*lpdwBufSize, *lpdwBufSize) LPSTR lpNameBuf,
    _Inout_ LPDWORD lpdwBufSize
    );

DWORD
WINAPI
GetNtmsMediaPoolNameW(
    _In_ HANDLE hSession,
    _In_ LPNTMS_GUID lpPoolId,
    _Out_writes_to_(*lpdwBufSize, *lpdwBufSize) LPWSTR lpNameBuf,
    _Inout_ LPDWORD lpdwBufSize
    );

DWORD WINAPI MoveToNtmsMediaPool(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    LPNTMS_GUID lpPoolId
    );

DWORD WINAPI DeleteNtmsMediaPool(
    HANDLE hSession,
    LPNTMS_GUID lpPoolId
    );

DWORD WINAPI DeleteNtmsLibrary(
    HANDLE hSession,
    LPNTMS_GUID lpLibraryId
    );

DWORD WINAPI DeleteNtmsDrive(
    HANDLE hSession,
    LPNTMS_GUID lpDriveId
    );

#endif  // MIDL_PASS

#define NTMS_OBJECTNAME_LENGTH      64
#define NTMS_DESCRIPTION_LENGTH     127
#define NTMS_DEVICENAME_LENGTH      64
#define NTMS_SERIALNUMBER_LENGTH    32
#define NTMS_REVISION_LENGTH        32
#define NTMS_BARCODE_LENGTH         64
#define NTMS_SEQUENCE_LENGTH        32
#define NTMS_VENDORNAME_LENGTH      128
#define NTMS_PRODUCTNAME_LENGTH     128
#define NTMS_USERNAME_LENGTH        64
#define NTMS_APPLICATIONNAME_LENGTH 64
#define NTMS_COMPUTERNAME_LENGTH    64
#define NTMS_I1_MESSAGE_LENGTH      127
#define NTMS_MESSAGE_LENGTH         256
#define NTMS_POOLHIERARCHY_LENGTH   512
#define NTMS_OMIDLABELID_LENGTH     255
#define NTMS_OMIDLABELTYPE_LENGTH   64
#define NTMS_OMIDLABELINFO_LENGTH   256

#ifndef NTMS_NOREDEF

//======================================================================
// OBJECT INFORMATION STRUCTURES
//======================================================================

enum NtmsDriveState {
    NTMS_DRIVESTATE_DISMOUNTED      = 0,
    NTMS_DRIVESTATE_MOUNTED         = 1,
    NTMS_DRIVESTATE_LOADED          = 2,
    NTMS_DRIVESTATE_UNLOADED        = 5,
    NTMS_DRIVESTATE_BEING_CLEANED   = 6,
    NTMS_DRIVESTATE_DISMOUNTABLE    = 7
};

#ifdef UNICODE
#define _NTMS_DRIVEINFORMATION _NTMS_DRIVEINFORMATIONW
#define NTMS_DRIVEINFORMATION NTMS_DRIVEINFORMATIONW
#else
#define _NTMS_DRIVEINFORMATION _NTMS_DRIVEINFORMATIONA
#define NTMS_DRIVEINFORMATION NTMS_DRIVEINFORMATIONA
#endif

typedef struct _NTMS_DRIVEINFORMATIONA {
    DWORD       Number;
    DWORD       State;
    NTMS_GUID   DriveType;
    CHAR        szDeviceName[NTMS_DEVICENAME_LENGTH];
    CHAR        szSerialNumber[NTMS_SERIALNUMBER_LENGTH];
    CHAR        szRevision[NTMS_REVISION_LENGTH];
    WORD        ScsiPort;
    WORD        ScsiBus;
    WORD        ScsiTarget;
    WORD        ScsiLun;
    DWORD       dwMountCount;
    SYSTEMTIME  LastCleanedTs;
    NTMS_GUID   SavedPartitionId;
    NTMS_GUID   Library;
    GUID        Reserved;
    DWORD       dwDeferDismountDelay;
} NTMS_DRIVEINFORMATIONA;

typedef struct _NTMS_DRIVEINFORMATIONW {
    DWORD       Number;
    DWORD       State;
    NTMS_GUID   DriveType;
    WCHAR       szDeviceName[NTMS_DEVICENAME_LENGTH];
    WCHAR       szSerialNumber[NTMS_SERIALNUMBER_LENGTH];
    WCHAR       szRevision[NTMS_REVISION_LENGTH];
    WORD        ScsiPort;
    WORD        ScsiBus;
    WORD        ScsiTarget;
    WORD        ScsiLun;
    DWORD       dwMountCount;
    SYSTEMTIME  LastCleanedTs;
    NTMS_GUID   SavedPartitionId;
    NTMS_GUID   Library;
    GUID        Reserved;
    DWORD       dwDeferDismountDelay;
} NTMS_DRIVEINFORMATIONW;

enum NtmsLibraryType {
    NTMS_LIBRARYTYPE_UNKNOWN        = 0,
    NTMS_LIBRARYTYPE_OFFLINE        = 1,
    NTMS_LIBRARYTYPE_ONLINE         = 2,
    NTMS_LIBRARYTYPE_STANDALONE     = 3
};

// library flags
enum NtmsLibraryFlags {
    NTMS_LIBRARYFLAG_FIXEDOFFLINE     = 0x01,
    NTMS_LIBRARYFLAG_CLEANERPRESENT   = 0x02,
    NTMS_LIBRARYFLAG_AUTODETECTCHANGE = 0x04,
    NTMS_LIBRARYFLAG_IGNORECLEANERUSESREMAINING = 0x08,
    NTMS_LIBRARYFLAG_RECOGNIZECLEANERBARCODE = 0x10
};

enum NtmsInventoryMethod {
    NTMS_INVENTORY_NONE             = 0,
    NTMS_INVENTORY_FAST             = 1,
    NTMS_INVENTORY_OMID             = 2,
    NTMS_INVENTORY_DEFAULT          = 3,
    NTMS_INVENTORY_SLOT             = 4,
    NTMS_INVENTORY_STOP             = 5,
    NTMS_INVENTORY_MAX
};

typedef struct _NTMS_LIBRARYINFORMATION {
    DWORD       LibraryType;
    NTMS_GUID   CleanerSlot;
    NTMS_GUID   CleanerSlotDefault;
    BOOL        LibrarySupportsDriveCleaning;
    BOOL        BarCodeReaderInstalled;
    DWORD       InventoryMethod;
    DWORD       dwCleanerUsesRemaining;
    DWORD       FirstDriveNumber;
    DWORD       dwNumberOfDrives;
    DWORD       FirstSlotNumber;
    DWORD       dwNumberOfSlots;
    DWORD       FirstDoorNumber;
    DWORD       dwNumberOfDoors;
    DWORD       FirstPortNumber;
    DWORD       dwNumberOfPorts;
    DWORD       FirstChangerNumber;
    DWORD       dwNumberOfChangers;
    DWORD       dwNumberOfMedia;
    DWORD       dwNumberOfMediaTypes;
    DWORD       dwNumberOfLibRequests;
    GUID        Reserved;
    BOOL        AutoRecovery;
    DWORD       dwFlags;
} NTMS_LIBRARYINFORMATION;

#ifdef UNICODE
#define _NTMS_CHANGERINFORMATION _NTMS_CHANGERINFORMATIONW
#define NTMS_CHANGERINFORMATION NTMS_CHANGERINFORMATIONW
#else
#define _NTMS_CHANGERINFORMATION _NTMS_CHANGERINFORMATIONA
#define NTMS_CHANGERINFORMATION NTMS_CHANGERINFORMATIONA
#endif

typedef struct _NTMS_CHANGERINFORMATIONA {
    DWORD       Number;
    NTMS_GUID   ChangerType;
    CHAR        szSerialNumber[NTMS_SERIALNUMBER_LENGTH];
    CHAR        szRevision[NTMS_REVISION_LENGTH];
    CHAR        szDeviceName[NTMS_DEVICENAME_LENGTH];
    WORD        ScsiPort;
    WORD        ScsiBus;
    WORD        ScsiTarget;
    WORD        ScsiLun;
    NTMS_GUID   Library;
} NTMS_CHANGERINFORMATIONA;

typedef struct _NTMS_CHANGERINFORMATIONW {
    DWORD       Number;
    NTMS_GUID   ChangerType;
    WCHAR       szSerialNumber[NTMS_SERIALNUMBER_LENGTH];
    WCHAR       szRevision[NTMS_REVISION_LENGTH];
    WCHAR       szDeviceName[NTMS_DEVICENAME_LENGTH];
    WORD        ScsiPort;
    WORD        ScsiBus;
    WORD        ScsiTarget;
    WORD        ScsiLun;
    NTMS_GUID   Library;
} NTMS_CHANGERINFORMATIONW;

enum NtmsSlotState {
    NTMS_SLOTSTATE_UNKNOWN          = 0,
    NTMS_SLOTSTATE_FULL             = 1,
    NTMS_SLOTSTATE_EMPTY            = 2,
    NTMS_SLOTSTATE_NOTPRESENT       = 3,
    NTMS_SLOTSTATE_NEEDSINVENTORY   = 4
};

typedef struct _NTMS_STORAGESLOTINFORMATION {
    DWORD       Number;
    DWORD       State;
    NTMS_GUID   Library;
} NTMS_STORAGESLOTINFORMATION;

enum NtmsDoorState {
    NTMS_DOORSTATE_UNKNOWN          = 0,
    NTMS_DOORSTATE_CLOSED           = 1,
    NTMS_DOORSTATE_OPEN             = 2
};

typedef struct _NTMS_IEDOORINFORMATION {
    DWORD       Number;
    DWORD       State;
    WORD        MaxOpenSecs;
    NTMS_GUID   Library;
} NTMS_IEDOORINFORMATION;

enum NtmsPortPosition {
    NTMS_PORTPOSITION_UNKNOWN       = 0,
    NTMS_PORTPOSITION_EXTENDED      = 1,
    NTMS_PORTPOSITION_RETRACTED     = 2
};

enum NtmsPortContent {
    NTMS_PORTCONTENT_UNKNOWN        = 0,
    NTMS_PORTCONTENT_FULL           = 1,
    NTMS_PORTCONTENT_EMPTY          = 2
};

typedef struct _NTMS_IEPORTINFORMATION {
    DWORD       Number;
    DWORD       Content;
    DWORD       Position;
    WORD        MaxExtendSecs;
    NTMS_GUID   Library;
} NTMS_IEPORTINFORMATION;

enum NtmsBarCodeState {
    NTMS_BARCODESTATE_OK            = 1,
    NTMS_BARCODESTATE_UNREADABLE    = 2
};

enum NtmsMediaState {
    NTMS_MEDIASTATE_IDLE            = 0,
    NTMS_MEDIASTATE_INUSE           = 1,
    NTMS_MEDIASTATE_MOUNTED         = 2,
    NTMS_MEDIASTATE_LOADED          = 3,
    NTMS_MEDIASTATE_UNLOADED        = 4,
    NTMS_MEDIASTATE_OPERROR         = 5,
    // media is waiting for operator request servicing
    NTMS_MEDIASTATE_OPREQ           = 6
};

#ifdef UNICODE
#define _NTMS_PMIDINFORMATION _NTMS_PMIDINFORMATIONW
#define NTMS_PMIDINFORMATION NTMS_PMIDINFORMATIONW
#else
#define _NTMS_PMIDINFORMATION _NTMS_PMIDINFORMATIONA
#define NTMS_PMIDINFORMATION NTMS_PMIDINFORMATIONA
#endif

typedef struct _NTMS_PMIDINFORMATIONA {
    NTMS_GUID   CurrentLibrary;                 // the current library
    NTMS_GUID   MediaPool;                      // media pool that the media belongs to
    NTMS_GUID   Location;                       // actual location of the media
    DWORD       LocationType;
    NTMS_GUID   MediaType;
    NTMS_GUID   HomeSlot;                       // home slot for online media
    CHAR        szBarCode[NTMS_BARCODE_LENGTH]; // bar code string
    DWORD       BarCodeState;                   // current state of the bar code
    CHAR        szSequenceNumber[NTMS_SEQUENCE_LENGTH];
    DWORD       MediaState;                     // current media state
    DWORD       dwNumberOfPartitions;
    DWORD       dwMediaTypeCode;
    DWORD       dwDensityCode;
    NTMS_GUID   MountedPartition;
} NTMS_PMIDINFORMATIONA;

typedef struct _NTMS_PMIDINFORMATIONW {
    NTMS_GUID   CurrentLibrary;                 // the current library
    NTMS_GUID   MediaPool;                      // media pool that the media belongs to
    NTMS_GUID   Location;                       // actual location of the media
    DWORD       LocationType;
    NTMS_GUID   MediaType;
    NTMS_GUID   HomeSlot;                       // home slot for online media
    WCHAR       szBarCode[NTMS_BARCODE_LENGTH]; // bar code string
    DWORD       BarCodeState;                   // current state of the bar code
    WCHAR       szSequenceNumber[NTMS_SEQUENCE_LENGTH];
    DWORD       MediaState;                     // current media state
    DWORD       dwNumberOfPartitions;
    DWORD       dwMediaTypeCode;
    DWORD       dwDensityCode;
    NTMS_GUID   MountedPartition;
} NTMS_PMIDINFORMATIONW;

typedef struct _NTMS_LMIDINFORMATION {
    NTMS_GUID   MediaPool;
    DWORD       dwNumberOfPartitions;
} NTMS_LMIDINFORMATION;

enum NtmsPartitionState {
    NTMS_PARTSTATE_UNKNOWN          = 0,
    NTMS_PARTSTATE_UNPREPARED       = 1,
    NTMS_PARTSTATE_INCOMPATIBLE     = 2,
    NTMS_PARTSTATE_DECOMMISSIONED   = 3,
    NTMS_PARTSTATE_AVAILABLE        = 4,
    NTMS_PARTSTATE_ALLOCATED        = 5,
    NTMS_PARTSTATE_COMPLETE         = 6,
    NTMS_PARTSTATE_FOREIGN          = 7,
    NTMS_PARTSTATE_IMPORT           = 8,
    NTMS_PARTSTATE_RESERVED         = 9
};

// define the new state as the unknown state for now.
#define NTMS_PARTSTATE_NEW NTMS_PARTSTATE_UNKNOWN

#ifdef UNICODE
#define _NTMS_PARTITIONINFORMATION _NTMS_PARTITIONINFORMATIONW
#define NTMS_PARTITIONINFORMATION NTMS_PARTITIONINFORMATIONW
#else
#define _NTMS_PARTITIONINFORMATION _NTMS_PARTITIONINFORMATIONA
#define NTMS_PARTITIONINFORMATION NTMS_PARTITIONINFORMATIONA
#endif

typedef struct _NTMS_PARTITIONINFORMATIONA {
    NTMS_GUID   PhysicalMedia;
    NTMS_GUID   LogicalMedia;
    DWORD       State;
    WORD        Side;
    DWORD       dwOmidLabelIdLength;    // binary id
    BYTE        OmidLabelId[NTMS_OMIDLABELID_LENGTH];
    CHAR        szOmidLabelType[NTMS_OMIDLABELTYPE_LENGTH];     // type string
    CHAR        szOmidLabelInfo[NTMS_OMIDLABELINFO_LENGTH];     // info string
    DWORD       dwMountCount;
    DWORD       dwAllocateCount;
    LARGE_INTEGER Capacity;
} NTMS_PARTITIONINFORMATIONA;

typedef struct _NTMS_PARTITIONINFORMATIONW {
    NTMS_GUID   PhysicalMedia;
    NTMS_GUID   LogicalMedia;
    DWORD       State;
    WORD        Side;
    DWORD       dwOmidLabelIdLength;    // binary id
    BYTE        OmidLabelId[NTMS_OMIDLABELID_LENGTH];
    WCHAR       szOmidLabelType[NTMS_OMIDLABELTYPE_LENGTH];     // type string
    WCHAR       szOmidLabelInfo[NTMS_OMIDLABELINFO_LENGTH];     // info string
    DWORD       dwMountCount;
    DWORD       dwAllocateCount;
    LARGE_INTEGER Capacity;
} NTMS_PARTITIONINFORMATIONW;

enum NtmsPoolType {
    NTMS_POOLTYPE_UNKNOWN           = 0,
    NTMS_POOLTYPE_SCRATCH           = 1,
    NTMS_POOLTYPE_FOREIGN           = 2,
    NTMS_POOLTYPE_IMPORT            = 3,
    NTMS_POOLTYPE_APPLICATION       = 1000
};

enum NtmsAllocationPolicy {
    NTMS_ALLOCATE_FROMSCRATCH       = 1
};

enum NtmsDeallocationPolicy {
    NTMS_DEALLOCATE_TOSCRATCH       = 1
};

typedef struct _NTMS_MEDIAPOOLINFORMATION {
     DWORD      PoolType;
     NTMS_GUID  MediaType;
     NTMS_GUID  Parent;
     DWORD      AllocationPolicy;
     DWORD      DeallocationPolicy;
     DWORD      dwMaxAllocates;
     DWORD      dwNumberOfPhysicalMedia;
     DWORD      dwNumberOfLogicalMedia;
     DWORD      dwNumberOfMediaPools;
} NTMS_MEDIAPOOLINFORMATION;

enum NtmsReadWriteCharacteristics {
    NTMS_MEDIARW_UNKNOWN            = 0,
    NTMS_MEDIARW_REWRITABLE         = 1,
    NTMS_MEDIARW_WRITEONCE          = 2,
    NTMS_MEDIARW_READONLY           = 3
};

typedef struct _NTMS_MEDIATYPEINFORMATION {
    DWORD       MediaType;      // MediaTypeCodes
    DWORD       NumberOfSides;
    DWORD       ReadWriteCharacteristics;
    DWORD       DeviceType;
} NTMS_MEDIATYPEINFORMATION;

#ifdef UNICODE
#define _NTMS_DRIVETYPEINFORMATION _NTMS_DRIVETYPEINFORMATIONW
#define NTMS_DRIVETYPEINFORMATION NTMS_DRIVETYPEINFORMATIONW
#else
#define _NTMS_DRIVETYPEINFORMATION _NTMS_DRIVETYPEINFORMATIONA
#define NTMS_DRIVETYPEINFORMATION NTMS_DRIVETYPEINFORMATIONA
#endif

typedef struct _NTMS_DRIVETYPEINFORMATIONA {
    CHAR        szVendor[NTMS_VENDORNAME_LENGTH];
    CHAR        szProduct[NTMS_PRODUCTNAME_LENGTH];
    DWORD       NumberOfHeads;
    DWORD       DeviceType;
} NTMS_DRIVETYPEINFORMATIONA;

typedef struct _NTMS_DRIVETYPEINFORMATIONW {
    WCHAR       szVendor[NTMS_VENDORNAME_LENGTH];
    WCHAR       szProduct[NTMS_PRODUCTNAME_LENGTH];
    DWORD       NumberOfHeads;
    DWORD       DeviceType;
} NTMS_DRIVETYPEINFORMATIONW;

#ifdef UNICODE
#define _NTMS_CHANGERTYPEINFORMATION _NTMS_CHANGERTYPEINFORMATIONW
#define NTMS_CHANGERTYPEINFORMATION NTMS_CHANGERTYPEINFORMATIONW
#else
#define _NTMS_CHANGERTYPEINFORMATION _NTMS_CHANGERTYPEINFORMATIONA
#define NTMS_CHANGERTYPEINFORMATION NTMS_CHANGERTYPEINFORMATIONA
#endif

typedef struct _NTMS_CHANGERTYPEINFORMATIONA {
    CHAR        szVendor[NTMS_VENDORNAME_LENGTH];
    CHAR        szProduct[NTMS_PRODUCTNAME_LENGTH];
    DWORD       DeviceType;
} NTMS_CHANGERTYPEINFORMATIONA;

typedef struct _NTMS_CHANGERTYPEINFORMATIONW {
    WCHAR       szVendor[NTMS_VENDORNAME_LENGTH];
    WCHAR       szProduct[NTMS_PRODUCTNAME_LENGTH];
    DWORD       DeviceType;
} NTMS_CHANGERTYPEINFORMATIONW;

//
// Support both CHANGER & LIBRARY for (Dis/Enable) - backwards compatibility
//
enum NtmsLmOperation {
    NTMS_LM_REMOVE                  = 0,
    NTMS_LM_DISABLECHANGER          = 1,
    NTMS_LM_DISABLELIBRARY          = 1,
    NTMS_LM_ENABLECHANGER           = 2,
    NTMS_LM_ENABLELIBRARY           = 2,
    NTMS_LM_DISABLEDRIVE            = 3,
    NTMS_LM_ENABLEDRIVE             = 4,
    NTMS_LM_DISABLEMEDIA            = 5,
    NTMS_LM_ENABLEMEDIA             = 6,
    NTMS_LM_UPDATEOMID              = 7,
    NTMS_LM_INVENTORY               = 8,
    NTMS_LM_DOORACCESS              = 9,
    NTMS_LM_EJECT                   = 10,
    NTMS_LM_EJECTCLEANER            = 11,
    NTMS_LM_INJECT                  = 12,
    NTMS_LM_INJECTCLEANER           = 13,
    NTMS_LM_PROCESSOMID             = 14,
    NTMS_LM_CLEANDRIVE              = 15,
    NTMS_LM_DISMOUNT                = 16,
    NTMS_LM_MOUNT                   = 17,
    NTMS_LM_WRITESCRATCH            = 18,
    NTMS_LM_CLASSIFY                = 19,
    NTMS_LM_RESERVECLEANER          = 20,
    NTMS_LM_RELEASECLEANER          = 21,
    NTMS_LM_MAXWORKITEM
};

enum NtmsLmState {
    NTMS_LM_QUEUED                  = 0,
    NTMS_LM_INPROCESS               = 1,
    NTMS_LM_PASSED                  = 2,
    NTMS_LM_FAILED                  = 3,
    NTMS_LM_INVALID                 = 4,
    NTMS_LM_WAITING                 = 5,
    NTMS_LM_DEFERRED                = 6,
    NTMS_LM_DEFFERED                = 6,  // DO NOT USE.  Maintained for backward compatiblity
    NTMS_LM_CANCELLED               = 7,
    NTMS_LM_STOPPED                 = 8
};

#ifdef UNICODE
#define _NTMS_LIBREQUESTINFORMATION _NTMS_LIBREQUESTINFORMATIONW
#define NTMS_LIBREQUESTINFORMATION NTMS_LIBREQUESTINFORMATIONW
#else
#define _NTMS_LIBREQUESTINFORMATION _NTMS_LIBREQUESTINFORMATIONA
#define NTMS_LIBREQUESTINFORMATION NTMS_LIBREQUESTINFORMATIONA
#endif

typedef struct _NTMS_LIBREQUESTINFORMATIONA {
    DWORD       OperationCode;
    DWORD       OperationOption;
    DWORD       State;
    NTMS_GUID   PartitionId;
    NTMS_GUID   DriveId;
    NTMS_GUID   PhysMediaId;
    NTMS_GUID   Library;
    NTMS_GUID   SlotId;
    SYSTEMTIME  TimeQueued;
    SYSTEMTIME  TimeCompleted;
    CHAR        szApplication[NTMS_APPLICATIONNAME_LENGTH];
    CHAR        szUser[NTMS_USERNAME_LENGTH];
    CHAR        szComputer[NTMS_COMPUTERNAME_LENGTH];
    DWORD       dwErrorCode;                    // error value if state = FAILED
    NTMS_GUID   WorkItemId;                     // work item id for cancel and other requests that
    DWORD       dwPriority;                     // require and associated work item
} NTMS_LIBREQUESTINFORMATIONA;

typedef struct _NTMS_LIBREQUESTINFORMATIONW {
    DWORD       OperationCode;
    DWORD       OperationOption;
    DWORD       State;
    NTMS_GUID   PartitionId;
    NTMS_GUID   DriveId;
    NTMS_GUID   PhysMediaId;
    NTMS_GUID   Library;
    NTMS_GUID   SlotId;
    SYSTEMTIME  TimeQueued;
    SYSTEMTIME  TimeCompleted;
    WCHAR       szApplication[NTMS_APPLICATIONNAME_LENGTH];
    WCHAR       szUser[NTMS_USERNAME_LENGTH];
    WCHAR       szComputer[NTMS_COMPUTERNAME_LENGTH];
    DWORD       dwErrorCode;                    // error value if state = FAILED
    NTMS_GUID   WorkItemId;                     // work item id for cancel and other requests that
    DWORD       dwPriority;                     // require and associated work item
} NTMS_LIBREQUESTINFORMATIONW;

enum NtmsOpreqCommand {
    NTMS_OPREQ_UNKNOWN              = 0,
    NTMS_OPREQ_NEWMEDIA             = 1,
    NTMS_OPREQ_CLEANER              = 2,
    NTMS_OPREQ_DEVICESERVICE        = 3,
    NTMS_OPREQ_MOVEMEDIA            = 4,
    NTMS_OPREQ_MESSAGE              = 5
};

enum NtmsOpreqState {
    NTMS_OPSTATE_UNKNOWN            = 0,
    NTMS_OPSTATE_SUBMITTED          = 1,
    NTMS_OPSTATE_ACTIVE             = 2,
    NTMS_OPSTATE_INPROGRESS         = 3,
    NTMS_OPSTATE_REFUSED            = 4,
    NTMS_OPSTATE_COMPLETE           = 5
};

#ifdef UNICODE
#define _NTMS_OPREQUESTINFORMATION _NTMS_OPREQUESTINFORMATIONW
#define NTMS_OPREQUESTINFORMATION NTMS_OPREQUESTINFORMATIONW
#else
#define _NTMS_OPREQUESTINFORMATION _NTMS_OPREQUESTINFORMATIONA
#define NTMS_OPREQUESTINFORMATION NTMS_OPREQUESTINFORMATIONA
#endif

typedef struct _NTMS_OPREQUESTINFORMATIONA {
    DWORD       Request;
    SYSTEMTIME  Submitted;
    DWORD       State;
    CHAR        szMessage[NTMS_MESSAGE_LENGTH];
    DWORD       Arg1Type;
    NTMS_GUID   Arg1;
    DWORD       Arg2Type;
    NTMS_GUID   Arg2;
    CHAR        szApplication[NTMS_APPLICATIONNAME_LENGTH];
    CHAR        szUser[NTMS_USERNAME_LENGTH];
    CHAR        szComputer[NTMS_COMPUTERNAME_LENGTH];
} NTMS_OPREQUESTINFORMATIONA;

typedef struct _NTMS_OPREQUESTINFORMATIONW {
    DWORD       Request;
    SYSTEMTIME  Submitted;
    DWORD       State;
    WCHAR       szMessage[NTMS_MESSAGE_LENGTH];
    DWORD       Arg1Type;
    NTMS_GUID   Arg1;
    DWORD       Arg2Type;
    NTMS_GUID   Arg2;
    WCHAR       szApplication[NTMS_APPLICATIONNAME_LENGTH];
    WCHAR       szUser[NTMS_USERNAME_LENGTH];
    WCHAR       szComputer[NTMS_COMPUTERNAME_LENGTH];
} NTMS_OPREQUESTINFORMATIONW;

typedef struct _NTMS_COMPUTERINFORMATION {
    DWORD       dwLibRequestPurgeTime;
    DWORD       dwOpRequestPurgeTime;
    DWORD       dwLibRequestFlags;
    DWORD       dwOpRequestFlags;
    DWORD       dwMediaPoolPolicy;
} NTMS_COMPUTERINFORMATION;

// library request option flags
enum NtmsLibRequestFlags {
    NTMS_LIBREQFLAGS_NOAUTOPURGE    = 0x01,
    NTMS_LIBREQFLAGS_NOFAILEDPURGE  = 0x02
};

// operator request option flags
enum NtmsOpRequestFlags {
    NTMS_OPREQFLAGS_NOAUTOPURGE     = 0x01,
    NTMS_OPREQFLAGS_NOFAILEDPURGE   = 0x02,
    NTMS_OPREQFLAGS_NOALERTS        = 0x10,
    NTMS_OPREQFLAGS_NOTRAYICON      = 0x20
};

// media pool policy flags
enum NtmsMediaPoolPolicy {
    // purge offline scratch media.  By default we keep offline scratch media
    // in the fixed offline media pool
    NTMS_POOLPOLICY_PURGEOFFLINESCRATCH = 0x01,
    NTMS_POOLPOLICY_KEEPOFFLINEIMPORT   = 0x02
};

#ifdef UNICODE
#define _NTMS_OBJECTINFORMATION _NTMS_OBJECTINFORMATIONW
#define NTMS_OBJECTINFORMATION NTMS_OBJECTINFORMATIONW
#define LPNTMS_OBJECTINFORMATION LPNTMS_OBJECTINFORMATIONW
#else
#define _NTMS_OBJECTINFORMATION _NTMS_OBJECTINFORMATIONA
#define NTMS_OBJECTINFORMATION NTMS_OBJECTINFORMATIONA
#define LPNTMS_OBJECTINFORMATION LPNTMS_OBJECTINFORMATIONA
#endif

enum NtmsOperationalState {
    NTMS_READY                      = 0,
    NTMS_INITIALIZING               = 10,
    NTMS_NEEDS_SERVICE              = 20,
    NTMS_NOT_PRESENT                = 21
};

typedef struct _NTMS_OBJECTINFORMATIONA {
    DWORD       dwSize;
    DWORD       dwType;
    SYSTEMTIME  Created;
    SYSTEMTIME  Modified;
    NTMS_GUID   ObjectGuid;
    BOOL        Enabled;
    DWORD       dwOperationalState;
    CHAR        szName[NTMS_OBJECTNAME_LENGTH];
    CHAR        szDescription[NTMS_DESCRIPTION_LENGTH];
#ifdef MIDL_PASS
    [switch_is(dwType)] union {
        [case(NTMS_DRIVE)]              NTMS_DRIVEINFORMATIONA Drive;
        [case(NTMS_DRIVE_TYPE)]         NTMS_DRIVETYPEINFORMATIONA DriveType;
        [case(NTMS_LIBRARY)]            NTMS_LIBRARYINFORMATION Library;
        [case(NTMS_CHANGER)]            NTMS_CHANGERINFORMATIONA Changer;
        [case(NTMS_CHANGER_TYPE)]       NTMS_CHANGERTYPEINFORMATIONA ChangerType;
        [case(NTMS_STORAGESLOT)]        NTMS_STORAGESLOTINFORMATION StorageSlot;
        [case(NTMS_IEDOOR)]             NTMS_IEDOORINFORMATION IEDoor;
        [case(NTMS_IEPORT)]             NTMS_IEPORTINFORMATION IEPort;
        [case(NTMS_PHYSICAL_MEDIA)]     NTMS_PMIDINFORMATIONA PhysicalMedia;
        [case(NTMS_LOGICAL_MEDIA)]      NTMS_LMIDINFORMATION LogicalMedia;
        [case(NTMS_PARTITION)]          NTMS_PARTITIONINFORMATIONA Partition;
        [case(NTMS_MEDIA_POOL)]         NTMS_MEDIAPOOLINFORMATION MediaPool;
        [case(NTMS_MEDIA_TYPE)]         NTMS_MEDIATYPEINFORMATION MediaType;
        [case(NTMS_LIBREQUEST)]         NTMS_LIBREQUESTINFORMATIONA LibRequest;
        [case(NTMS_OPREQUEST)]          NTMS_OPREQUESTINFORMATIONA OpRequest;
        [case(NTMS_COMPUTER)]           NTMS_COMPUTERINFORMATION Computer;
        [default]                                       ;
    } Info;
#else
    union {
        NTMS_DRIVEINFORMATIONA      Drive;
        NTMS_DRIVETYPEINFORMATIONA  DriveType;
        NTMS_LIBRARYINFORMATION     Library;
        NTMS_CHANGERINFORMATIONA    Changer;
        NTMS_CHANGERTYPEINFORMATIONA ChangerType;
        NTMS_STORAGESLOTINFORMATION StorageSlot;
        NTMS_IEDOORINFORMATION      IEDoor;
        NTMS_IEPORTINFORMATION      IEPort;
        NTMS_PMIDINFORMATIONA       PhysicalMedia;
        NTMS_LMIDINFORMATION        LogicalMedia;
        NTMS_PARTITIONINFORMATIONA  Partition;
        NTMS_MEDIAPOOLINFORMATION   MediaPool;
        NTMS_MEDIATYPEINFORMATION   MediaType;
        NTMS_LIBREQUESTINFORMATIONA LibRequest;
        NTMS_OPREQUESTINFORMATIONA  OpRequest;
        NTMS_COMPUTERINFORMATION    Computer;
    } Info;
#endif  // MIDL_PASS
} NTMS_OBJECTINFORMATIONA, *LPNTMS_OBJECTINFORMATIONA;

typedef struct _NTMS_OBJECTINFORMATIONW {
    DWORD       dwSize;
    DWORD       dwType;
    SYSTEMTIME  Created;
    SYSTEMTIME  Modified;
    NTMS_GUID   ObjectGuid;
    BOOL        Enabled;
    DWORD       dwOperationalState;
    WCHAR       szName[NTMS_OBJECTNAME_LENGTH];
    WCHAR       szDescription[NTMS_DESCRIPTION_LENGTH];
#ifdef MIDL_PASS
    [switch_is(dwType)] union {
        [case(NTMS_DRIVE)]              NTMS_DRIVEINFORMATIONW Drive;
        [case(NTMS_DRIVE_TYPE)]         NTMS_DRIVETYPEINFORMATIONW DriveType;
        [case(NTMS_LIBRARY)]            NTMS_LIBRARYINFORMATION Library;
        [case(NTMS_CHANGER)]            NTMS_CHANGERINFORMATIONW Changer;
        [case(NTMS_CHANGER_TYPE)]       NTMS_CHANGERTYPEINFORMATIONW ChangerType;
        [case(NTMS_STORAGESLOT)]        NTMS_STORAGESLOTINFORMATION StorageSlot;
        [case(NTMS_IEDOOR)]             NTMS_IEDOORINFORMATION IEDoor;
        [case(NTMS_IEPORT)]             NTMS_IEPORTINFORMATION IEPort;
        [case(NTMS_PHYSICAL_MEDIA)]     NTMS_PMIDINFORMATIONW PhysicalMedia;
        [case(NTMS_LOGICAL_MEDIA)]      NTMS_LMIDINFORMATION LogicalMedia;
        [case(NTMS_PARTITION)]          NTMS_PARTITIONINFORMATIONW Partition;
        [case(NTMS_MEDIA_POOL)]         NTMS_MEDIAPOOLINFORMATION MediaPool;
        [case(NTMS_MEDIA_TYPE)]         NTMS_MEDIATYPEINFORMATION MediaType;
        [case(NTMS_LIBREQUEST)]         NTMS_LIBREQUESTINFORMATIONW LibRequest;
        [case(NTMS_OPREQUEST)]          NTMS_OPREQUESTINFORMATIONW OpRequest;
        [case(NTMS_COMPUTER)]           NTMS_COMPUTERINFORMATION Computer;
        [default]                                       ;
    } Info;
#else
    union {
        NTMS_DRIVEINFORMATIONW Drive;
        NTMS_DRIVETYPEINFORMATIONW DriveType;
        NTMS_LIBRARYINFORMATION Library;
        NTMS_CHANGERINFORMATIONW Changer;
        NTMS_CHANGERTYPEINFORMATIONW ChangerType;
        NTMS_STORAGESLOTINFORMATION StorageSlot;
        NTMS_IEDOORINFORMATION IEDoor;
        NTMS_IEPORTINFORMATION IEPort;
        NTMS_PMIDINFORMATIONW PhysicalMedia;
        NTMS_LMIDINFORMATION LogicalMedia;
        NTMS_PARTITIONINFORMATIONW Partition;
        NTMS_MEDIAPOOLINFORMATION MediaPool;
        NTMS_MEDIATYPEINFORMATION MediaType;
        NTMS_LIBREQUESTINFORMATIONW LibRequest;
        NTMS_OPREQUESTINFORMATIONW OpRequest;
        NTMS_COMPUTERINFORMATION Computer;
    } Info;
#endif  // MIDL_PASS
} NTMS_OBJECTINFORMATIONW, *LPNTMS_OBJECTINFORMATIONW;

//********************************************************************************
// NT 5.0 BETA1 variations of object information.  Required to keep here for
// backward compatibility in DCOM interface.
// Different data structures add the _I1_ to the structure name, denoting
// used for Interface 1 only.
// DO NOT MODIFY IN FUTURE VERSIONS !!
//********************************************************************************

#ifdef UNICODE
#define NTMS_I1_LIBREQUESTINFORMATION   NTMS_I1_LIBREQUESTINFORMATIONW
#define NTMS_I1_PARTITIONINFORMATION    NTMS_I1_PARTITIONINFORMATIONW
#define NTMS_I1_PMIDINFORMATION         NTMS_I1_PMIDINFORMATIONW
#define NTMS_I1_OPREQUESTINFORMATION    NTMS_I1_OPREQUESTINFORMATIONW
#define NTMS_I1_OBJECTINFORMATION       NTMS_I1_OBJECTINFORMATIONW
#else
#define NTMS_I1_LIBREQUESTINFORMATION   NTMS_I1_LIBREQUESTINFORMATIONA
#define NTMS_I1_PARTITIONINFORMATION    NTMS_I1_PARTITIONINFORMATIONA
#define NTMS_I1_PMIDINFORMATION         NTMS_I1_PMIDINFORMATIONA
#define NTMS_I1_OPREQUESTINFORMATION    NTMS_I1_OPREQUESTINFORMATIONA
#define NTMS_I1_OBJECTINFORMATION       NTMS_I1_OBJECTINFORMATIONA
#endif

typedef struct _NTMS_I1_LIBRARYINFORMATION {
    DWORD       LibraryType;
    NTMS_GUID   CleanerSlot;
    NTMS_GUID   CleanerSlotDefault;
    BOOL        LibrarySupportsDriveCleaning;
    BOOL        BarCodeReaderInstalled;
    DWORD       InventoryMethod;
    DWORD       dwCleanerUsesRemaining;
    DWORD       FirstDriveNumber;
    DWORD       dwNumberOfDrives;
    DWORD       FirstSlotNumber;
    DWORD       dwNumberOfSlots;
    DWORD       FirstDoorNumber;
    DWORD       dwNumberOfDoors;
    DWORD       FirstPortNumber;
    DWORD       dwNumberOfPorts;
    DWORD       FirstChangerNumber;
    DWORD       dwNumberOfChangers;
    DWORD       dwNumberOfMedia;
    DWORD       dwNumberOfMediaTypes;
    DWORD       dwNumberOfLibRequests;
    GUID        Reserved;
} NTMS_I1_LIBRARYINFORMATION;

typedef struct _NTMS_I1_LIBREQUESTINFORMATIONA {
    DWORD       OperationCode;
    DWORD       OperationOption;
    DWORD       State;
    NTMS_GUID   PartitionId;
    NTMS_GUID   DriveId;
    NTMS_GUID   PhysMediaId;
    NTMS_GUID   Library;
    NTMS_GUID   SlotId;
    SYSTEMTIME  TimeQueued;
    SYSTEMTIME  TimeCompleted;
    CHAR        szApplication[NTMS_APPLICATIONNAME_LENGTH];
    CHAR        szUser[NTMS_USERNAME_LENGTH];
    CHAR        szComputer[NTMS_COMPUTERNAME_LENGTH];
} NTMS_I1_LIBREQUESTINFORMATIONA;

typedef struct _NTMS_I1_LIBREQUESTINFORMATIONW {
    DWORD       OperationCode;
    DWORD       OperationOption;
    DWORD       State;
    NTMS_GUID   PartitionId;
    NTMS_GUID   DriveId;
    NTMS_GUID   PhysMediaId;
    NTMS_GUID   Library;
    NTMS_GUID   SlotId;
    SYSTEMTIME  TimeQueued;
    SYSTEMTIME  TimeCompleted;
    WCHAR       szApplication[NTMS_APPLICATIONNAME_LENGTH];
    WCHAR       szUser[NTMS_USERNAME_LENGTH];
    WCHAR       szComputer[NTMS_COMPUTERNAME_LENGTH];
} NTMS_I1_LIBREQUESTINFORMATIONW;

typedef struct _NTMS_I1_PMIDINFORMATIONA {
    NTMS_GUID   CurrentLibrary;                 // the current library
    NTMS_GUID   MediaPool;                      // media pool that the media belongs to
    NTMS_GUID   Location;                       // actual location of the media
    DWORD       LocationType;
    NTMS_GUID   MediaType;
    NTMS_GUID   HomeSlot;                       // home slot for online media
    CHAR        szBarCode[NTMS_BARCODE_LENGTH]; // bar code string
    DWORD       BarCodeState;                   // current state of the bar code
    CHAR        szSequenceNumber[NTMS_SEQUENCE_LENGTH];
    DWORD       MediaState;                     // current media state
    DWORD       dwNumberOfPartitions;
} NTMS_I1_PMIDINFORMATIONA;

typedef struct _NTMS_I1_PMIDINFORMATIONW {
    NTMS_GUID   CurrentLibrary;                 // the current library
    NTMS_GUID   MediaPool;                      // media pool that the media belongs to
    NTMS_GUID   Location;                       // actual location of the media
    DWORD       LocationType;
    NTMS_GUID   MediaType;
    NTMS_GUID   HomeSlot;                       // home slot for online media
    WCHAR       szBarCode[NTMS_BARCODE_LENGTH]; // bar code string
    DWORD       BarCodeState;                   // current state of the bar code
    WCHAR       szSequenceNumber[NTMS_SEQUENCE_LENGTH];
    DWORD       MediaState;                     // current media state
    DWORD       dwNumberOfPartitions;
} NTMS_I1_PMIDINFORMATIONW;

typedef struct _NTMS_I1_PARTITIONINFORMATIONA {
    NTMS_GUID   PhysicalMedia;
    NTMS_GUID   LogicalMedia;
    DWORD       State;
    WORD        Side;
    DWORD       dwOmidLabelIdLength;    // binary id
    BYTE        OmidLabelId[255];
    CHAR        szOmidLabelType[64];    // type string
    CHAR        szOmidLabelInfo[256];   // info string
    DWORD       dwMountCount;
    DWORD       dwAllocateCount;
} NTMS_I1_PARTITIONINFORMATIONA;

typedef struct _NTMS_I1_PARTITIONINFORMATIONW {
    NTMS_GUID   PhysicalMedia;
    NTMS_GUID   LogicalMedia;
    DWORD       State;
    WORD        Side;
    DWORD       dwOmidLabelIdLength;    // binary id
    BYTE        OmidLabelId[255];
    WCHAR       szOmidLabelType[64];    // type string
    WCHAR       szOmidLabelInfo[256];   // info string
    DWORD       dwMountCount;
    DWORD       dwAllocateCount;
} NTMS_I1_PARTITIONINFORMATIONW;

typedef struct _NTMS_I1_OPREQUESTINFORMATIONA {
    DWORD       Request;
    SYSTEMTIME  Submitted;
    DWORD       State;
    CHAR        szMessage[NTMS_I1_MESSAGE_LENGTH];
    DWORD       Arg1Type;
    NTMS_GUID   Arg1;
    DWORD       Arg2Type;
    NTMS_GUID   Arg2;
    CHAR        szApplication[NTMS_APPLICATIONNAME_LENGTH];
    CHAR        szUser[NTMS_USERNAME_LENGTH];
    CHAR        szComputer[NTMS_COMPUTERNAME_LENGTH];
} NTMS_I1_OPREQUESTINFORMATIONA;

typedef struct _NTMS_I1_OPREQUESTINFORMATIONW {
    DWORD       Request;
    SYSTEMTIME  Submitted;
    DWORD       State;
    WCHAR       szMessage[NTMS_I1_MESSAGE_LENGTH];
    DWORD       Arg1Type;
    NTMS_GUID   Arg1;
    DWORD       Arg2Type;
    NTMS_GUID   Arg2;
    WCHAR       szApplication[NTMS_APPLICATIONNAME_LENGTH];
    WCHAR       szUser[NTMS_USERNAME_LENGTH];
    WCHAR       szComputer[NTMS_COMPUTERNAME_LENGTH];
} NTMS_I1_OPREQUESTINFORMATIONW;

typedef struct _NTMS_I1_OBJECTINFORMATIONA {
    DWORD       dwSize;
    DWORD       dwType;
    SYSTEMTIME  Created;
    SYSTEMTIME  Modified;
    NTMS_GUID   ObjectGuid;
    BOOL        Enabled;
    DWORD       dwOperationalState;
    CHAR        szName[NTMS_OBJECTNAME_LENGTH];
    CHAR        szDescription[NTMS_DESCRIPTION_LENGTH];
#ifdef MIDL_PASS
    [switch_is(dwType)] union {
        [case(NTMS_DRIVE)]              NTMS_DRIVEINFORMATIONA Drive;
        [case(NTMS_DRIVE_TYPE)]         NTMS_DRIVETYPEINFORMATIONA DriveType;
        [case(NTMS_LIBRARY)]            NTMS_I1_LIBRARYINFORMATION Library;
        [case(NTMS_CHANGER)]            NTMS_CHANGERINFORMATIONA Changer;
        [case(NTMS_CHANGER_TYPE)]       NTMS_CHANGERTYPEINFORMATIONA ChangerType;
        [case(NTMS_STORAGESLOT)]        NTMS_STORAGESLOTINFORMATION StorageSlot;
        [case(NTMS_IEDOOR)]             NTMS_IEDOORINFORMATION IEDoor;
        [case(NTMS_IEPORT)]             NTMS_IEPORTINFORMATION IEPort;
        [case(NTMS_PHYSICAL_MEDIA)]     NTMS_I1_PMIDINFORMATIONA PhysicalMedia;
        [case(NTMS_LOGICAL_MEDIA)]      NTMS_LMIDINFORMATION LogicalMedia;
        [case(NTMS_PARTITION)]          NTMS_I1_PARTITIONINFORMATIONA Partition;
        [case(NTMS_MEDIA_POOL)]         NTMS_MEDIAPOOLINFORMATION MediaPool;
        [case(NTMS_MEDIA_TYPE)]         NTMS_MEDIATYPEINFORMATION MediaType;
        [case(NTMS_LIBREQUEST)]         NTMS_I1_LIBREQUESTINFORMATIONA LibRequest;
        [case(NTMS_OPREQUEST)]          NTMS_I1_OPREQUESTINFORMATIONA OpRequest;
        [default]                                       ;
    } Info;
#else
    union {
        NTMS_DRIVEINFORMATIONA Drive;
        NTMS_DRIVETYPEINFORMATIONA DriveType;
        NTMS_I1_LIBRARYINFORMATION Library;
        NTMS_CHANGERINFORMATIONA Changer;
        NTMS_CHANGERTYPEINFORMATIONA ChangerType;
        NTMS_STORAGESLOTINFORMATION StorageSlot;
        NTMS_IEDOORINFORMATION IEDoor;
        NTMS_IEPORTINFORMATION IEPort;
        NTMS_I1_PMIDINFORMATIONA PhysicalMedia;
        NTMS_LMIDINFORMATION LogicalMedia;
        NTMS_I1_PARTITIONINFORMATIONA Partition;
        NTMS_MEDIAPOOLINFORMATION MediaPool;
        NTMS_MEDIATYPEINFORMATION MediaType;
        NTMS_I1_LIBREQUESTINFORMATIONA LibRequest;
        NTMS_I1_OPREQUESTINFORMATIONA OpRequest;
    } Info;
#endif  // MIDL_PASS
} NTMS_I1_OBJECTINFORMATIONA, *LPNTMS_I1_OBJECTINFORMATIONA;

typedef struct _NTMS_I1_OBJECTINFORMATIONW {
    DWORD       dwSize;
    DWORD       dwType;
    SYSTEMTIME  Created;
    SYSTEMTIME  Modified;
    NTMS_GUID   ObjectGuid;
    BOOL        Enabled;
    DWORD       dwOperationalState;
    WCHAR       szName[NTMS_OBJECTNAME_LENGTH];
    WCHAR       szDescription[NTMS_DESCRIPTION_LENGTH];
#ifdef MIDL_PASS
    [switch_is(dwType)] union {
        [case(NTMS_DRIVE)]              NTMS_DRIVEINFORMATIONW Drive;
        [case(NTMS_DRIVE_TYPE)]         NTMS_DRIVETYPEINFORMATIONW DriveType;
        [case(NTMS_LIBRARY)]            NTMS_I1_LIBRARYINFORMATION Library;
        [case(NTMS_CHANGER)]            NTMS_CHANGERINFORMATIONW Changer;
        [case(NTMS_CHANGER_TYPE)]       NTMS_CHANGERTYPEINFORMATIONW ChangerType;
        [case(NTMS_STORAGESLOT)]        NTMS_STORAGESLOTINFORMATION StorageSlot;
        [case(NTMS_IEDOOR)]             NTMS_IEDOORINFORMATION IEDoor;
        [case(NTMS_IEPORT)]             NTMS_IEPORTINFORMATION IEPort;
        [case(NTMS_PHYSICAL_MEDIA)]     NTMS_I1_PMIDINFORMATIONW PhysicalMedia;
        [case(NTMS_LOGICAL_MEDIA)]      NTMS_LMIDINFORMATION LogicalMedia;
        [case(NTMS_PARTITION)]          NTMS_I1_PARTITIONINFORMATIONW Partition;
        [case(NTMS_MEDIA_POOL)]         NTMS_MEDIAPOOLINFORMATION MediaPool;
        [case(NTMS_MEDIA_TYPE)]         NTMS_MEDIATYPEINFORMATION MediaType;
        [case(NTMS_LIBREQUEST)]         NTMS_I1_LIBREQUESTINFORMATIONW LibRequest;
        [case(NTMS_OPREQUEST)]          NTMS_I1_OPREQUESTINFORMATIONW OpRequest;
        [default]                                       ;
    } Info;
#else
    union {
        NTMS_DRIVEINFORMATIONW Drive;
        NTMS_DRIVETYPEINFORMATIONW DriveType;
        NTMS_I1_LIBRARYINFORMATION Library;
        NTMS_CHANGERINFORMATIONW Changer;
        NTMS_CHANGERTYPEINFORMATIONW ChangerType;
        NTMS_STORAGESLOTINFORMATION StorageSlot;
        NTMS_IEDOORINFORMATION IEDoor;
        NTMS_IEPORTINFORMATION IEPort;
        NTMS_I1_PMIDINFORMATIONW PhysicalMedia;
        NTMS_LMIDINFORMATION LogicalMedia;
        NTMS_I1_PARTITIONINFORMATIONW Partition;
        NTMS_MEDIAPOOLINFORMATION MediaPool;
        NTMS_MEDIATYPEINFORMATION MediaType;
        NTMS_I1_LIBREQUESTINFORMATIONW LibRequest;
        NTMS_I1_OPREQUESTINFORMATIONW OpRequest;
    } Info;
#endif  // MIDL_PASS
} NTMS_I1_OBJECTINFORMATIONW, *LPNTMS_I1_OBJECTINFORMATIONW;

#endif  // NTMS_NOREDEF

#ifndef NTMS_NOREDEF

//=======================================================================
// CREATENTMSMEDIA DEFINITIONS
//=======================================================================

enum NtmsCreateNtmsMediaOptions {
    NTMS_ERROR_ON_DUPLICATE         = 0x0001
};

#endif

#ifndef MIDL_PASS

//======================================================================
// OBJECT MANAGEMENT APIS
//======================================================================

#ifdef PRE_SEVIL

DWORD WINAPI GetNtmsObjectInformation(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    LPNTMS_OBJECTINFORMATION lpInfo
    );

DWORD WINAPI SetNtmsObjectInformation(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    LPNTMS_OBJECTINFORMATION lpInfo
    );

#endif

// Added by SEvilia
DWORD WINAPI GetNtmsObjectInformationA(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    LPNTMS_OBJECTINFORMATIONA lpInfo
    );

DWORD WINAPI GetNtmsObjectInformationW(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    LPNTMS_OBJECTINFORMATIONW lpInfo
    );

DWORD WINAPI SetNtmsObjectInformationA(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    LPNTMS_OBJECTINFORMATIONA lpInfo
    );

DWORD WINAPI SetNtmsObjectInformationW(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    LPNTMS_OBJECTINFORMATIONW lpInfo
    );
// END of SEVILIA

DWORD WINAPI CreateNtmsMediaA(
    HANDLE hSession,
    LPNTMS_OBJECTINFORMATIONA lpMedia,
    LPNTMS_OBJECTINFORMATIONA lpList,
    DWORD dwOptions
    ) ;

DWORD WINAPI CreateNtmsMediaW(
    HANDLE hSession,
    LPNTMS_OBJECTINFORMATIONW lpMedia,
    LPNTMS_OBJECTINFORMATIONW lpList,
    DWORD dwOptions
    ) ;

enum NtmsEnumerateOption {
    NTMS_ENUM_DEFAULT                   = 0,
    NTMS_ENUM_ROOTPOOL                  = 1
    };

DWORD WINAPI EnumerateNtmsObject(
    HANDLE hSession,
    const LPNTMS_GUID lpContainerId,
    LPNTMS_GUID lpList,
    LPDWORD lpdwListSize,
    DWORD dwType,
    DWORD dwOptions
    );

DWORD WINAPI DisableNtmsObject(
    HANDLE hSession,
    DWORD dwType,
    LPNTMS_GUID lpObjectId
    );

DWORD WINAPI EnableNtmsObject(
    HANDLE hSession,
    DWORD dwType,
    LPNTMS_GUID lpObjectId
    );

enum NtmsEjectOperation {
    NTMS_EJECT_START                = 0,
    NTMS_EJECT_STOP                 = 1,
    NTMS_EJECT_QUEUE                = 2,
    NTMS_EJECT_FORCE                = 3,
    NTMS_EJECT_IMMEDIATE            = 4,
    NTMS_EJECT_ASK_USER             = 5
};

DWORD WINAPI EjectNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    LPNTMS_GUID lpEjectOperation,
    DWORD dwAction
    );

enum NtmsInjectOperation {
    NTMS_INJECT_START               = 0,
    NTMS_INJECT_STOP                = 1,
    NTMS_INJECT_RETRACT             = 2,
    NTMS_INJECT_STARTMANY           = 3
};

DWORD WINAPI InjectNtmsMedia(
    HANDLE hSession,
    LPNTMS_GUID lpLibraryId,
    LPNTMS_GUID lpInjectOperation,
    DWORD dwAction
    );

DWORD WINAPI AccessNtmsLibraryDoor(
    HANDLE hSession,
    LPNTMS_GUID lpLibraryId,
    DWORD dwAction
    );

DWORD WINAPI CleanNtmsDrive(
    HANDLE hSession,
    LPNTMS_GUID lpDriveId
    );

DWORD WINAPI DismountNtmsDrive(
    HANDLE hSession,
    LPNTMS_GUID lpDriveId
    );

DWORD WINAPI InventoryNtmsLibrary(
    HANDLE hSession,
    LPNTMS_GUID lpLibraryId,
    DWORD dwAction
    );

DWORD WINAPI IdentifyNtmsSlot(
    HANDLE hSession,
    LPNTMS_GUID lpSlotId,
    DWORD dwOption
    );

// definitions for the UpdateNtmsOmidInfo ...
#define NTMS_OMID_TYPE_RAW_LABEL        0x01
#define NTMS_OMID_TYPE_FILESYSTEM_INFO  0x02

typedef struct
{
    WCHAR FileSystemType[64] ;
    WCHAR VolumeName[256] ;
    DWORD SerialNumber ;
} NTMS_FILESYSTEM_INFO ;

DWORD WINAPI UpdateNtmsOmidInfo(
    HANDLE hSession,
    LPNTMS_GUID lpMediaId,
    DWORD labelType,
    DWORD numberOfBytes,
    LPVOID lpBuffer
    );

DWORD WINAPI CancelNtmsLibraryRequest(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId
    );

DWORD WINAPI GetNtmsRequestOrder(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId,
    LPDWORD lpdwOrderNumber
    );

DWORD WINAPI SetNtmsRequestOrder(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId,
    DWORD dwOrderNumber
    );

DWORD WINAPI DeleteNtmsRequests(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId,
    DWORD dwType,
    DWORD dwCount
    );

DWORD WINAPI ReserveNtmsCleanerSlot (
    HANDLE hSession,
    LPNTMS_GUID lpLibrary,
    LPNTMS_GUID lpSlot
    );

DWORD WINAPI ReleaseNtmsCleanerSlot (
    HANDLE hSession,
    LPNTMS_GUID lpLibrary
    );

DWORD WINAPI InjectNtmsCleaner (
    HANDLE hSession,
    LPNTMS_GUID lpLibrary,
    LPNTMS_GUID lpInjectOperation,
    DWORD dwNumberOfCleansLeft,
    DWORD dwAction
    );

DWORD WINAPI EjectNtmsCleaner (
    HANDLE hSession,
    LPNTMS_GUID lpLibrary,
    LPNTMS_GUID lpEjectOperation,
    DWORD dwAction
    );

DWORD WINAPI BeginNtmsDeviceChangeDetection(
    HANDLE hSession,
    LPHANDLE lpDetectHandle
    );

DWORD WINAPI SetNtmsDeviceChangeDetection(
    HANDLE hSession,
    HANDLE DetectHandle,
    LPNTMS_GUID lpRequestId,
    DWORD dwType,
    DWORD dwCount
    );

DWORD WINAPI EndNtmsDeviceChangeDetection(
    HANDLE hSession,
    HANDLE DetectHandle
    );

#endif  // MIDL_PASS

#ifndef NTMS_NOREDEF
enum NtmsDriveType {
    NTMS_UNKNOWN_DRIVE              = 0
};

#endif  // NTMS_NOREDEF

#ifndef MIDL_PASS

// Security for NTMS API
DWORD WINAPI GetNtmsObjectSecurity(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    DWORD dwType,
    SECURITY_INFORMATION RequestedInformation,
    PSECURITY_DESCRIPTOR lpSecurityDescriptor,
    DWORD nLength,
    LPDWORD lpnLengthNeeded
    );

DWORD WINAPI SetNtmsObjectSecurity(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    DWORD dwType,
    SECURITY_INFORMATION SecurityInformation,
    PSECURITY_DESCRIPTOR lpSecurityDescriptor
    );

// Security Access Control Masks :
// NTMS_USE_ACCESS is required to use an NTMS object.  For example,
// you will need this access to a library in order to mount media
// within it.
// NTMS_MODIFY_ACCESS is required to make changes to an NTMS object.
// For example, you will need modify access in order to change the name
// of an object or change its attributes.
// NTMS_CONTROL_ACCESS is required to control an NTMS object.  For
// example, you will need control access to a library in order to
// inject media, eject media, clean or open the door.
//
enum NtmsAccessMask {
    NTMS_USE_ACCESS                 = 0x1,
    NTMS_MODIFY_ACCESS              = 0x2,
    NTMS_CONTROL_ACCESS             = 0x4
};

// Generic Mappings :
#define NTMS_GENERIC_READ           NTMS_USE_ACCESS
#define NTMS_GENERIC_WRITE          (NTMS_USE_ACCESS | NTMS_MODIFY_ACCESS)
#define NTMS_GENERIC_EXECUTE        (NTMS_USE_ACCESS | NTMS_MODIFY_ACCESS | NTMS_CONTROL_ACCESS)
#define NTMS_GENERIC_ALL            (NTMS_USE_ACCESS | NTMS_MODIFY_ACCESS | NTMS_CONTROL_ACCESS)

// Maximum attribute size for NTMS Object attribute API's
#define NTMS_MAXATTR_LENGTH         0x10000

// Maximum len of attribute name (# of chars.)
#define NTMS_MAXATTR_NAMELEN        32

// Object extensions for NTMS API
DWORD WINAPI GetNtmsObjectAttributeA(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    DWORD dwType,
    LPCSTR lpAttributeName,
    LPVOID lpAttributeData,
    LPDWORD lpAttributeSize
    );

DWORD WINAPI GetNtmsObjectAttributeW(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    DWORD dwType,
    LPCWSTR lpAttributeName,
    LPVOID lpAttributeData,
    LPDWORD lpAttributeSize
    );

DWORD WINAPI SetNtmsObjectAttributeA(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    DWORD dwType,
    LPCSTR lpAttributeName,
    LPVOID lpAttributeData,
    DWORD dwAttributeSize
    );

DWORD WINAPI SetNtmsObjectAttributeW(
    HANDLE hSession,
    LPNTMS_GUID lpObjectId,
    DWORD dwType,
    LPCWSTR lpAttributeName,
    LPVOID lpAttributeData,
    DWORD AttributeSize
    );

//UI Notification
enum NtmsUITypes {
    NTMS_UITYPE_INVALID             = 0,
    NTMS_UITYPE_INFO,
    NTMS_UITYPE_REQ,
    NTMS_UITYPE_ERR,
    NTMS_UITYPE_MAX
};

enum NtmsUIOperations {
    NTMS_UIDEST_ADD                 = 1,
    NTMS_UIDEST_DELETE,
    NTMS_UIDEST_DELETEALL,
    NTMS_UIOPERATION_MAX
};

DWORD
WINAPI
GetNtmsUIOptionsA(
    _In_ HANDLE hSession,
    _In_ const LPNTMS_GUID lpObjectId,
    _In_ DWORD dwType,
    _Out_writes_to_(*lpdwBufSize, *lpdwBufSize) LPSTR lpszDestination,
    _Inout_ LPDWORD lpdwBufSize
    );

_Success_ (return == ERROR_SUCCESS)
DWORD
WINAPI
GetNtmsUIOptionsW(
    _In_ HANDLE hSession,
    _In_ const LPNTMS_GUID lpObjectId,
    _In_ DWORD dwType,
    _Out_writes_to_(*lpdwBufSize, *lpdwBufSize) LPWSTR lpszDestination,
    _Inout_ LPDWORD lpdwBufSize
    );

DWORD WINAPI SetNtmsUIOptionsA(
    HANDLE hSession,
    const LPNTMS_GUID lpObjectId,
    DWORD dwType,
    DWORD dwOperation,
    LPCSTR lpszDestination
    );

DWORD WINAPI SetNtmsUIOptionsW(
    HANDLE hSession,
    const LPNTMS_GUID lpObjectId,
    DWORD dwType,
    DWORD dwOperation,
    LPCWSTR lpszDestination
    );


// Operator Requests
DWORD WINAPI SubmitNtmsOperatorRequestW(
    HANDLE hSession,
    DWORD dwRequest,
    LPCWSTR lpMessage,
    LPNTMS_GUID lpArg1Id,
    LPNTMS_GUID lpArg2Id,
    LPNTMS_GUID lpRequestId
    );

DWORD WINAPI SubmitNtmsOperatorRequestA(
    HANDLE hSession,
    DWORD dwRequest,
    LPCSTR lpMessage,
    LPNTMS_GUID lpArg1Id,
    LPNTMS_GUID lpArg2Id,
    LPNTMS_GUID lpRequestId
    );

DWORD WINAPI WaitForNtmsOperatorRequest(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId,
    DWORD dwTimeout
    );

DWORD WINAPI CancelNtmsOperatorRequest(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId
    );

DWORD WINAPI SatisfyNtmsOperatorRequest(
    HANDLE hSession,
    LPNTMS_GUID lpRequestId
    );

#endif  // MIDL_PASS

#ifndef NTMS_NOREDEF

enum NtmsNotificationOperations {
    NTMS_OBJ_UPDATE                 = 1,
    NTMS_OBJ_INSERT                 = 2,
    NTMS_OBJ_DELETE                 = 3,
    NTMS_EVENT_SIGNAL               = 4,
    NTMS_EVENT_COMPLETE             = 5
};

// object notification
typedef struct _NTMS_NOTIFICATIONINFORMATION {
    DWORD dwOperation;
    NTMS_GUID ObjectId;
} NTMS_NOTIFICATIONINFORMATION, *LPNTMS_NOTIFICATIONINFORMATION;

#endif  // NTMS_NOREDEF

#ifndef MIDL_PASS

DWORD WINAPI ImportNtmsDatabase(
    HANDLE hSession
    );

DWORD WINAPI ExportNtmsDatabase(
    HANDLE hSession
    );

#endif  // MIDL_PASS

#ifndef MIDL_PASS

DWORD WINAPI ImportNtmsDatabase(
    HANDLE hSession
    );

DWORD WINAPI ExportNtmsDatabase(
    HANDLE hSession
    );

#endif  // MIDL_PASS

#ifndef MIDL_PASS

HANDLE WINAPI OpenNtmsNotification(
    HANDLE hSession,
    DWORD dwType
    );

DWORD WINAPI WaitForNtmsNotification(
    HANDLE hNotification,
    LPNTMS_NOTIFICATIONINFORMATION lpNotificationInformation,
    DWORD dwTimeout
    );

DWORD WINAPI CloseNtmsNotification(
    HANDLE hNotification
    );

#endif  // MIDL_PASS

#ifndef MIDL_PASS

DWORD WINAPI EjectDiskFromSADriveW(
    LPCWSTR lpComputerName,
    LPCWSTR lpAppName,
    LPCWSTR lpDeviceName,
    HWND hWnd,
    LPCWSTR lpTitle,
    LPCWSTR lpMessage,
    DWORD dwOptions
    );

DWORD WINAPI EjectDiskFromSADriveA(
    LPCSTR lpComputerName,
    LPCSTR lpAppName,
    LPCSTR lpDeviceName,
    HWND hWnd,
    LPCSTR lpTitle,
    LPCSTR lpMessage,
    DWORD dwOptions
    );

#endif

#ifndef MIDL_PASS

DWORD
WINAPI
GetVolumesFromDriveW(
    _In_ LPWSTR pszDriveName,
    _Outptr_ LPWSTR *VolumeNameBufferPtr,
    _Outptr_ LPWSTR *DriveLetterBufferPtr
    );

DWORD
WINAPI
GetVolumesFromDriveA(
    _In_ LPSTR pszDriveName,
    _Outptr_ LPSTR *VolumeNameBufferPtr,
    _Outptr_ LPSTR *DriveLetterBufferPtr
    );

#endif

#ifdef __cplusplus
} // end extern "C"
#endif

#pragma pack()

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _INCL_NTMSAPI_H_
