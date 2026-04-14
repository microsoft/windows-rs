/*++

Copyright (c) 2006 Microsoft Corporation

Module Name:

    WdsClientApi.h

Abstract:

    WDS Client API public header file.

Environment:

    User Mode

--*/
#ifndef __WDSCLIENTAPI_H__
#define __WDSCLIENTAPI_H__

#if (_MSC_VER > 1000)
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#pragma warning( push )
#pragma warning( disable : 4091 )

#define WDSCLIAPI WINAPI

#ifdef __cplusplus
extern "C"
{
#endif

//
// Structures
//


// Credentials structure.
//
typedef struct tagWDS_CLI_CRED
{
    PCWSTR pwszUserName,
           pwszDomain,
           pwszPassword;
} WDS_CLI_CRED, *PWDS_CLI_CRED, *LPWDS_CLI_CRED;


//
// General functions
//
HRESULT
WDSCLIAPI
WdsCliClose( 
    _In_ HANDLE Handle );


typedef 
VOID
(WDSCLIAPI *PFN_WdsCliTraceFunction)(
    _In_ LPCWSTR pwszFormat,
    _In_ va_list Params
);


HRESULT
WDSCLIAPI
WdsCliRegisterTrace(
    _In_opt_ PFN_WdsCliTraceFunction pfn );


HRESULT
WDSCLIAPI
WdsCliFreeStringArray(
    _Inout_updates_opt_(ulCount) PWSTR *ppwszArray,
    _In_ ULONG ulCount );

//
// Image enumeration functions
//
HRESULT
WDSCLIAPI
WdsCliFindFirstImage(
    _In_ HANDLE hSession,
    _Out_ PHANDLE phFindHandle );

HRESULT
WDSCLIAPI
WdsCliFindNextImage(
    _In_ HANDLE Handle );

HRESULT
WDSCLIAPI
WdsCliGetEnumerationFlags(
    _In_ HANDLE Handle,
    _Out_ PDWORD pdwFlags );

HRESULT
WDSCLIAPI
WdsCliGetImageHandleFromFindHandle(
    _In_ HANDLE FindHandle,
    _Out_ PHANDLE phImageHandle );

HRESULT
WDSCLIAPI
WdsCliGetImageHandleFromTransferHandle(
    _In_ HANDLE hTransfer,
    _Out_ PHANDLE phImageHandle );


//
// Image enumeration constants
//

enum
{
    WdsCliFlagEnumFilterVersion = 0x0001,
    WdsCliFlagEnumFilterFirmware = 0x0002, 
};

//
// Image type constants.
//
typedef enum _WDS_CLI_IMAGE_TYPE
{
    WDS_CLI_IMAGE_TYPE_UNKNOWN = 0,
    WDS_CLI_IMAGE_TYPE_WIM = 1,
    WDS_CLI_IMAGE_TYPE_VHD = 2,
    WDS_CLI_IMAGE_TYPE_VHDX = 3
} WDS_CLI_IMAGE_TYPE, *PWDS_CLI_IMAGE_TYPE;

//
// Firmware Types.
//
typedef enum _WDS_CLI_FIRMWARE_TYPE 
{
    WDS_CLI_FIRMWARE_UNKNOWN = 0,
    WDS_CLI_FIRMWARE_BIOS = 0x0001,
    WDS_CLI_FIRMWARE_EFI = 0x0002,
} WDS_CLI_FIRMWARE_TYPE, *PWDS_CLI_FIRMWARE_TYPE;

//
// Extended Image Parameters.
//
// WDS_CLI_IMAGE_PARAM_SPARSE_FILE is deprecated and will always return FALSE
// from WdsCliGetImageParameter.
//
typedef enum _WDS_CLI_IMAGE_PARAM_TYPE
{
    WDS_CLI_IMAGE_PARAM_UNKNOWN = 0,
    WDS_CLI_IMAGE_PARAM_SPARSE_FILE = 1,
    WDS_CLI_IMAGE_PARAM_SUPPORTED_FIRMWARES = 2,
} WDS_CLI_IMAGE_PARAM_TYPE, *PWDS_CLI_IMAGE_PARAM_TYPE;

//
// Session functions
//
HRESULT
WDSCLIAPI
WdsCliCreateSession(
    _In_ PWSTR pwszServer,
    _In_opt_ PWDS_CLI_CRED pCred,
    _Out_ PHANDLE phSession );

HRESULT
WDSCLIAPI
WdsCliAuthorizeSession(
    _Inout_ HANDLE hSession,
    _In_opt_ PWDS_CLI_CRED pCred );

//
// Client -> Server Logging functions
//
HRESULT
WDSCLIAPI
WdsCliInitializeLog(
    _In_ HANDLE hSession, 
    _In_ ULONG ulClientArchitecture,
    _In_ PWSTR pwszClientId,
    _In_ PWSTR pwszClientAddress );

HRESULT
WDSCLIAPI
WdsCliLog(
    _In_ HANDLE hSession,
    _In_ ULONG ulLogLevel,
    _In_ ULONG ulMessageCode,
    ... );

//
// Log events sent by the client to the server.
// Each one of these has a set of variables associated with it
// that must be present in the request in order for the request to be 
// considered valid.
//
enum
{
    WDS_LOG_TYPE_CLIENT_ERROR = 1,
    WDS_LOG_TYPE_CLIENT_STARTED,
    WDS_LOG_TYPE_CLIENT_FINISHED,
    WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED,    
    WDS_LOG_TYPE_CLIENT_APPLY_STARTED,
    WDS_LOG_TYPE_CLIENT_APPLY_FINISHED,
    WDS_LOG_TYPE_CLIENT_GENERIC_MESSAGE,
    WDS_LOG_TYPE_CLIENT_UNATTEND_MODE,
    WDS_LOG_TYPE_CLIENT_TRANSFER_START,
    WDS_LOG_TYPE_CLIENT_TRANSFER_END,
    WDS_LOG_TYPE_CLIENT_TRANSFER_DOWNGRADE,
    WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR,
    WDS_LOG_TYPE_CLIENT_POST_ACTIONS_START,
    WDS_LOG_TYPE_CLIENT_POST_ACTIONS_END,
    WDS_LOG_TYPE_CLIENT_APPLY_STARTED_2,
    WDS_LOG_TYPE_CLIENT_APPLY_FINISHED_2,
    WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR_2,
    WDS_LOG_TYPE_CLIENT_DRIVER_PACKAGE_NOT_ACCESSIBLE,
    WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_START,
    WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_END,
    WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_FAILURE,
    WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED2,
    WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED3,
    // Only add entries before WDS_LOG_TYPE_CLIENT_MAX_CODE.
    WDS_LOG_TYPE_CLIENT_MAX_CODE 
};

//
// Log levels.
//
enum
{
    WDS_LOG_LEVEL_DISABLED = 0,
    WDS_LOG_LEVEL_ERROR = 1,
    WDS_LOG_LEVEL_WARNING = 2,
    WDS_LOG_LEVEL_INFO = 3
};

//
// Image information functions
//

HRESULT
WDSCLIAPI
WdsCliGetImageName(
    _In_ HANDLE hIfh, 
    _Out_ PWSTR *ppwszValue );

HRESULT
WDSCLIAPI
WdsCliGetImageDescription(
    _In_ HANDLE hIfh, 
    _Out_ PWSTR *ppwszValue );

HRESULT 
WDSCLIAPI 
WdsCliGetImageType(
    _In_ HANDLE hIfh,
    _Out_ PWDS_CLI_IMAGE_TYPE pImageType );

HRESULT
WDSCLIAPI
WdsCliGetImageFiles(
    _In_ HANDLE hIfh,
    _Outptr_result_buffer_(*pdwCount) LPWSTR **pppwszFiles,
   _Out_ PDWORD pdwCount );

HRESULT
WDSCLIAPI
WdsCliGetImageLanguage(
    _In_ HANDLE hIfh, 
    _Out_ PWSTR *ppwszValue );

HRESULT
WDSCLIAPI
WdsCliGetImageLanguages(
    _In_ HANDLE hIfh,
    _Outptr_result_buffer_(*pdwNumValues) PTSTR **pppszValues,
    _Out_ PDWORD pdwNumValues );

HRESULT
WDSCLIAPI
WdsCliGetImageVersion(
    _In_ HANDLE hIfh, 
    _Out_ PWSTR *ppwszValue );

HRESULT
WDSCLIAPI
WdsCliGetImagePath(
    _In_ HANDLE hIfh,
    _Out_ PWSTR *ppwszValue );

HRESULT
WDSCLIAPI
WdsCliGetImageIndex(
    _In_ HANDLE hIfh,
    _Out_ PDWORD pdwValue );

HRESULT
WDSCLIAPI
WdsCliGetImageArchitecture(
    _In_ HANDLE hIfh,
    _Out_ PDWORD pdwValue );

HRESULT
WDSCLIAPI
WdsCliGetImageLastModifiedTime(
    _In_ HANDLE hIfh,
    _Out_ PSYSTEMTIME *ppSysTimeValue );

HRESULT
WDSCLIAPI
WdsCliGetImageSize(
    _In_ HANDLE hIfh,
    _Out_ PULONGLONG pullValue );

HRESULT
WDSCLIAPI
WdsCliGetImageHalName(
    _In_ HANDLE hIfh,
    _Out_ PWSTR *ppwszValue );

HRESULT
WDSCLIAPI
WdsCliGetImageGroup(
    _In_ HANDLE hIfh,
    _Out_ PWSTR *ppwszValue );

HRESULT
WDSCLIAPI
WdsCliGetImageNamespace(
    _In_ HANDLE hIfh,
    _Out_ PWSTR *ppwszValue );

HRESULT 
WDSCLIAPI 
WdsCliGetImageParameter(
    _In_ HANDLE hIfh,
    _In_ WDS_CLI_IMAGE_PARAM_TYPE ParamType,
    _Out_writes_bytes_(uResponseLen) PVOID pResponse,
    _In_ ULONG uResponseLen );

//
// Image transfer functions.
//

// Callback function
//

typedef
VOID
(WDSCLIAPI *PFN_WdsCliCallback)(
    DWORD dwMessageId,
    _In_opt_ WPARAM wParam,
    _In_opt_ LPARAM lParam,
    _In_opt_ PVOID pvUserData );

// Callback messages.
//
enum
{
    WDS_CLI_MSG_START = 0,
    WDS_CLI_MSG_COMPLETE,
    WDS_CLI_MSG_PROGRESS,
    WDS_CLI_MSG_TEXT,
};


HRESULT
WDSCLIAPI
WdsCliGetTransferSize(
    _In_ HANDLE hIfh,
    _Out_ PULONGLONG pullValue );

// Flags for transfer functions
//
// WDS_CLI_NO_SPARSE_FILE is deprecated but ignored for compatibility.
//
#define WDS_CLI_TRANSFER_ASYNCHRONOUS         0x00000001
#define WDS_CLI_NO_SPARSE_FILE                0x00000002

VOID
WDSCLIAPI
WdsCliSetTransferBufferSize(
    _In_ ULONG ulSizeInBytes );

HRESULT
WDSCLIAPI
WdsCliTransferImage(
    _In_ HANDLE hImage,
    _In_ PWSTR pwszLocalPath,
    _In_ DWORD dwFlags,
    _In_ DWORD dwReserved,
    _In_opt_ PFN_WdsCliCallback pfnWdsCliCallback,
    _In_opt_ PVOID pvUserData,
    _Out_ PHANDLE phTransfer );

HRESULT
WDSCLIAPI
WdsCliTransferFile(
    _In_ PCWSTR pwszServer,
    _In_ PCWSTR pwszNamespace,
    _In_ PCWSTR pwszRemoteFilePath,
    _In_ PCWSTR pwszLocalFilePath,
    _In_ DWORD dwFlags,
    _In_ DWORD dwReserved,
    _In_opt_ PFN_WdsCliCallback pfnWdsCliCallback,
    _In_opt_ PVOID pvUserData,
    _Out_ PHANDLE phTransfer );

HRESULT
WDSCLIAPI
WdsCliCancelTransfer(
    _In_ HANDLE hTransfer );

HRESULT
WDSCLIAPI
WdsCliWaitForTransfer(
    _In_ HANDLE hTransfer );

//
// Dynamic Driver Provisioning functions
//
HRESULT
WDSCLIAPI
WdsCliObtainDriverPackages(
    _In_ HANDLE hImage,
    _Outptr_ PWSTR *ppwszServerName,
    _Outptr_result_buffer_(*pulCount) PWSTR **pppwszDriverPackages,
    _Out_ ULONG *pulCount    
    );

HRESULT
WDSCLIAPI
WdsCliObtainDriverPackagesEx (
    _In_ HANDLE hSession,
    _In_ PWSTR pwszMachineInfo,
    _Outptr_ PWSTR *ppwszServerName,    
    _Outptr_result_buffer_(*pulCount) PWSTR **pppwszDriverPackages,
    _Out_ ULONG* pulCount
    );

HRESULT
WDSCLIAPI
WdsCliGetDriverQueryXml (
    _In_opt_ PWSTR pwszWinDirPath,
    _Outptr_ PWSTR *ppwszDriverQuery
    );


#ifdef __cplusplus
}
#endif

#pragma warning( pop ) 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  // __WDSCLIENTAPI_H__
