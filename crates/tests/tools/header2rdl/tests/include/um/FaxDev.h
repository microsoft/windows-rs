/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND, 
EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED 
WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

Module Name:

    faxdev.h

Abstract:

    This file contains the prototypes for the
    FAX device provider API.

--*/

#include <commctrl.h>


#ifndef _FAXDEV_
#define _FAXDEV_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

//
// FAX status constants
//

#define FS_INITIALIZING         0x20000000
#define FS_DIALING              0x20000001
#define FS_TRANSMITTING         0x20000002
#define FS_RECEIVING            0x20000004
#define FS_COMPLETED            0x20000008
#define FS_HANDLED              0x20000010
#define FS_LINE_UNAVAILABLE     0x20000020
#define FS_BUSY                 0x20000040
#define FS_NO_ANSWER            0x20000080
#define FS_BAD_ADDRESS          0x20000100
#define FS_NO_DIAL_TONE         0x20000200
#define FS_DISCONNECTED         0x20000400
#define FS_FATAL_ERROR          0x20000800  
#define FS_NOT_FAX_CALL         0x20001000
#define FS_CALL_DELAYED         0x20002000
#define FS_CALL_BLACKLISTED     0x20004000
#define FS_USER_ABORT           0x20200000
#define FS_ANSWERED             0x20800000


//
// data structures
//

typedef struct _FAX_SEND {
    DWORD   SizeOfStruct;
    LPWSTR  FileName;
    LPWSTR  CallerName;
    LPWSTR  CallerNumber;
    LPWSTR  ReceiverName;
    LPWSTR  ReceiverNumber;
    BOOL    Branding;
    HCALL   CallHandle;
    DWORD   Reserved[3];
} FAX_SEND, *PFAX_SEND;

typedef struct _FAX_RECEIVE {
    DWORD   SizeOfStruct;
    LPWSTR  FileName;
    LPWSTR  ReceiverName;
    LPWSTR  ReceiverNumber;
    DWORD   Reserved[4];
} FAX_RECEIVE, *PFAX_RECEIVE;

typedef _Struct_size_bytes_(SizeOfStruct) struct _FAX_DEV_STATUS {
    DWORD   SizeOfStruct;
    DWORD   StatusId;
    DWORD   StringId;
    DWORD   PageCount;
    LPWSTR  CSI;
    LPWSTR  CallerId;
    LPWSTR  RoutingInfo;
    DWORD   ErrorCode;
    DWORD   Reserved[3];
} FAX_DEV_STATUS, *PFAX_DEV_STATUS;

typedef BOOL
(CALLBACK *PFAX_SERVICE_CALLBACK)(
    IN HANDLE FaxHandle,
    IN DWORD  DeviceId,
    IN DWORD_PTR  Param1,
    IN DWORD_PTR  Param2,
    IN DWORD_PTR  Param3
    );

typedef void
(CALLBACK *PFAX_LINECALLBACK)(
    IN HANDLE FaxHandle,
    IN DWORD hDevice,
    IN DWORD dwMessage,
    IN DWORD_PTR dwInstance,
    IN DWORD_PTR dwParam1,
    IN DWORD_PTR dwParam2,
    IN DWORD_PTR dwParam3
    );

BOOL WINAPI
FaxDevInitialize(
    IN  HLINEAPP LineAppHandle,
    IN  HANDLE HeapHandle,
    OUT PFAX_LINECALLBACK *LineCallbackFunction,
    IN  PFAX_SERVICE_CALLBACK FaxServiceCallback
    );

HRESULT WINAPI FaxDevShutdown(
    void
);

BOOL WINAPI
FaxDevVirtualDeviceCreation(
    OUT LPDWORD DeviceCount,
    _Out_writes_(128) LPWSTR DeviceNamePrefix,
    OUT LPDWORD DeviceIdPrefix,
    IN  HANDLE CompletionPort,
    IN  ULONG_PTR CompletionKey
    );

BOOL WINAPI
FaxDevStartJob(
    IN  HLINE LineHandle,
    IN  DWORD DeviceId,
    OUT PHANDLE FaxHandle,
    IN  HANDLE CompletionPortHandle,
    IN  ULONG_PTR CompletionKey
    );

BOOL WINAPI
FaxDevEndJob(
    IN  HANDLE FaxHandle
    );

typedef BOOL
(CALLBACK *PFAX_SEND_CALLBACK)(
    IN HANDLE FaxHandle,
    IN HCALL CallHandle,
    IN DWORD Reserved1,
    IN DWORD Reserved2
    );

BOOL WINAPI
FaxDevSend(
    IN  HANDLE FaxHandle,
    IN  PFAX_SEND FaxSend,
    IN  PFAX_SEND_CALLBACK FaxSendCallback
    );

#define FAXDEVRECEIVE_SIZE 4096

BOOL WINAPI
FaxDevReceive(
    IN  HANDLE FaxHandle,
    IN  HCALL CallHandle,
    IN OUT PFAX_RECEIVE FaxReceive
    );

#define FAXDEVREPORTSTATUS_SIZE 4096

BOOL WINAPI
FaxDevReportStatus(
    IN  HANDLE FaxHandle OPTIONAL,
    OUT PFAX_DEV_STATUS FaxStatus,
    IN  DWORD FaxStatusSize,
    OUT LPDWORD FaxStatusSizeRequired
    );

BOOL WINAPI
FaxDevAbortOperation(
    IN  HANDLE FaxHandle
    );

BOOL WINAPI
FaxDevConfigure(
    OUT HPROPSHEETPAGE *PropSheetPage
    );

typedef BOOL (WINAPI *PFAXDEVINITIALIZE)                (HLINEAPP,HANDLE,PFAX_LINECALLBACK*,PFAX_SERVICE_CALLBACK);
typedef BOOL (WINAPI *PFAXDEVVIRTUALDEVICECREATION)     (LPDWORD DeviceCount,
                                                                                                            _Out_writes_(128) LPWSTR DeviceNamePrefix,
                                                                                                            LPDWORD DeviceIdPrefix,
                                                                                                            HANDLE CompletionPort,
                                                                                                            ULONG_PTR CompletionKey);
typedef BOOL (WINAPI *PFAXDEVSTARTJOB)                  (HLINE,DWORD,PHANDLE,HANDLE,ULONG_PTR);
typedef BOOL (WINAPI *PFAXDEVENDJOB)                    (HANDLE);
typedef BOOL (WINAPI *PFAXDEVSEND)                      (HANDLE,PFAX_SEND,PFAX_SEND_CALLBACK);
typedef BOOL (WINAPI *PFAXDEVRECEIVE)                   (HANDLE,HCALL,PFAX_RECEIVE);
typedef BOOL (WINAPI *PFAXDEVREPORTSTATUS)              (HANDLE,PFAX_DEV_STATUS,DWORD,LPDWORD);
typedef BOOL (WINAPI *PFAXDEVABORTOPERATION)            (HANDLE);
typedef BOOL (WINAPI *PFAXDEVCONFIGURE)                 (HPROPSHEETPAGE*);
typedef HRESULT (WINAPI * PFAXDEVSHUTDOWN)              (void);

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif
