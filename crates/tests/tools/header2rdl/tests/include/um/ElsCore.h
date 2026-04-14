/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    ElsCore.h

Abstract:

    This file contains macros/typedef/function prototypes that are common for ELS.

--*/

#ifndef __ELS_CORE__
#define __ELS_CORE__

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

struct _MAPPING_PROPERTY_BAG;

typedef void (CALLBACK* PFN_MAPPINGCALLBACKPROC)(struct _MAPPING_PROPERTY_BAG *pBag, LPVOID data, DWORD dwDataSize, HRESULT Result);

//
// Service types constants used in MAPPING_ENUM_OPTIONS (ServiceType field)
//

#define ALL_SERVICE_TYPES           0
#define HIGHLEVEL_SERVICE_TYPES     1
#define LOWLEVEL_SERVICE_TYPES      2

//
// Online service constants used in MAPPING_ENUM_OPTIONS (OnlineService field)
//
#define ALL_SERVICES                0
#define ONLINE_SERVICES             1
#define OFFLINE_SERVICES            2

//
// Public ELS structures
//

typedef struct _MAPPING_SERVICE_INFO
{
    size_t      Size;
    LPWSTR      pszCopyright;
    WORD        wMajorVersion;
    WORD        wMinorVersion;
    WORD        wBuildVersion;
    WORD        wStepVersion;
    DWORD       dwInputContentTypesCount;
    LPWSTR      *prgInputContentTypes;
    DWORD       dwOutputContentTypesCount;
    LPWSTR      *prgOutputContentTypes;
    DWORD       dwInputLanguagesCount;
    LPWSTR      *prgInputLanguages;
    DWORD       dwOutputLanguagesCount;
    LPWSTR      *prgOutputLanguages;
    DWORD       dwInputScriptsCount;
    LPWSTR      *prgInputScripts;
    DWORD       dwOutputScriptsCount;
    LPWSTR      *prgOutputScripts;
    GUID        guid;
    LPWSTR      pszCategory;
    LPWSTR      pszDescription;
    DWORD       dwPrivateDataSize;
    LPVOID      pPrivateData;
    LPVOID      pContext;
    unsigned    IsOneToOneLanguageMapping : 1;
    unsigned    HasSubservices            : 1;
    unsigned    OnlineOnly                : 1;
    unsigned    ServiceType               : 2;
} MAPPING_SERVICE_INFO, *PMAPPING_SERVICE_INFO;


typedef struct _MAPPING_ENUM_OPTIONS
{
    size_t                  Size;
    LPWSTR                  pszCategory;
    LPWSTR                  pszInputLanguage;
    LPWSTR                  pszOutputLanguage;
    LPWSTR                  pszInputScript;
    LPWSTR                  pszOutputScript;
    LPWSTR                  pszInputContentType;
    LPWSTR                  pszOutputContentType;
    GUID                    *pGuid;
    unsigned                OnlineService : 2;
    unsigned                ServiceType   : 2;
} MAPPING_ENUM_OPTIONS, *PMAPPING_ENUM_OPTIONS;


typedef struct _MAPPING_OPTIONS
{
    size_t                  Size;
    LPWSTR                  pszInputLanguage;
    LPWSTR                  pszOutputLanguage;
    LPWSTR                  pszInputScript;
    LPWSTR                  pszOutputScript;
    LPWSTR                  pszInputContentType;
    LPWSTR                  pszOutputContentType;
    LPWSTR                  pszUILanguage;
    PFN_MAPPINGCALLBACKPROC pfnRecognizeCallback;
    LPVOID                  pRecognizeCallerData;
    DWORD                   dwRecognizeCallerDataSize;
    PFN_MAPPINGCALLBACKPROC pfnActionCallback;
    LPVOID                  pActionCallerData;
    DWORD                   dwActionCallerDataSize;
    DWORD                   dwServiceFlag;
    unsigned                GetActionDisplayName : 1;
} MAPPING_OPTIONS, *PMAPPING_OPTIONS;


typedef struct _MAPPING_DATA_RANGE
{
    DWORD    dwStartIndex;
    DWORD    dwEndIndex;
    LPWSTR   pszDescription;
    DWORD    dwDescriptionLength;
    LPVOID   pData;
    DWORD    dwDataSize;
    LPWSTR   pszContentType;
    LPWSTR   *prgActionIds;
    DWORD    dwActionsCount;
    LPWSTR   *prgActionDisplayNames;
} MAPPING_DATA_RANGE, *PMAPPING_DATA_RANGE;


typedef struct _MAPPING_PROPERTY_BAG
{
    size_t               Size;
    PMAPPING_DATA_RANGE  prgResultRanges;
    DWORD                dwRangesCount;
    LPVOID               pServiceData;
    DWORD                dwServiceDataSize;
    LPVOID               pCallerData;
    DWORD                dwCallerDataSize;
    LPVOID               pContext;
} MAPPING_PROPERTY_BAG, *PMAPPING_PROPERTY_BAG;


//
// Public ELS APIs
//

HRESULT
WINAPI
MappingGetServices(
    _In_opt_  PMAPPING_ENUM_OPTIONS  pOptions,
    _Out_     PMAPPING_SERVICE_INFO *prgServices,
    _Out_     DWORD                 *pdwServicesCount
    );


HRESULT
WINAPI
MappingFreeServices(
    _In_ PMAPPING_SERVICE_INFO pServiceInfo
    );


HRESULT
WINAPI
MappingRecognizeText(
    _In_                  PMAPPING_SERVICE_INFO  pServiceInfo,
    _In_reads_(dwLength) LPCWSTR                pszText,
    _In_                  DWORD                  dwLength,
    _In_                  DWORD                  dwIndex,
    _In_opt_              PMAPPING_OPTIONS       pOptions,
    _Inout_               PMAPPING_PROPERTY_BAG  pbag
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

HRESULT
WINAPI
MappingDoAction(
    _Inout_ PMAPPING_PROPERTY_BAG pBag,
    _In_    DWORD                 dwRangeIndex,
    _In_    LPCWSTR               pszActionId
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

HRESULT
WINAPI
MappingFreePropertyBag(
    _In_ PMAPPING_PROPERTY_BAG pBag
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#ifdef __cplusplus
};
#endif

#endif // __ELS_CORE__
