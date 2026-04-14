/*********************************************************************************
*                                                                                *
* featurestagingapi.h -- ApiSet Contract for api-ms-win-core-featurestaging-l1 *
*                                                                                *
* Copyright (c) Microsoft Corporation. All rights reserved.                      *
*                                                                                *
**********************************************************************************/

#ifndef _APISET_WIL_FEATURESTAGING_
#define _APISET_WIL_FEATURESTAGING_

#include <apiset.h>
#include <apisetcconv.h>
#ifdef _CONTRACT_GEN
#include <nt.h>
#include <ntrtl.h>
#include <nturtl.h>
#include <windows.h>
#endif

typedef enum FEATURE_CHANGE_TIME
{
    FEATURE_CHANGE_TIME_READ = 0,
    FEATURE_CHANGE_TIME_MODULE_RELOAD = 1,
    FEATURE_CHANGE_TIME_SESSION = 2,
    FEATURE_CHANGE_TIME_REBOOT = 3
} FEATURE_CHANGE_TIME;

typedef enum FEATURE_ENABLED_STATE
{
    FEATURE_ENABLED_STATE_DEFAULT = 0,
    FEATURE_ENABLED_STATE_DISABLED = 1,
    FEATURE_ENABLED_STATE_ENABLED = 2
} FEATURE_ENABLED_STATE;

typedef struct FEATURE_ERROR
{
    HRESULT hr;
    UINT16 lineNumber;
    PCSTR file;
    PCSTR process;
    PCSTR module;
    UINT32 callerReturnAddressOffset;
    PCSTR callerModule;
    PCSTR message;
    UINT16 originLineNumber;
    PCSTR originFile;
    PCSTR originModule;
    UINT32 originCallerReturnAddressOffset;
    PCSTR originCallerModule;
    PCSTR originName;
} FEATURE_ERROR;

DECLARE_HANDLE(FEATURE_STATE_CHANGE_SUBSCRIPTION);
typedef void WINAPI FEATURE_STATE_CHANGE_CALLBACK(_In_opt_ void* context);
typedef FEATURE_STATE_CHANGE_CALLBACK *PFEATURE_STATE_CHANGE_CALLBACK;

EXTERN_C
FEATURE_ENABLED_STATE
GetFeatureEnabledState(
    UINT32 featureId,
    FEATURE_CHANGE_TIME changeTime
    );

EXTERN_C
void
RecordFeatureUsage(
    UINT32 featureId,
    UINT32 kind,
    UINT32 addend,
    _In_ PCSTR originName
    );

EXTERN_C
void
RecordFeatureError(
    UINT32 featureId,
    _In_ const FEATURE_ERROR* error
    );

EXTERN_C
void
SubscribeFeatureStateChangeNotification(
    _Outptr_ FEATURE_STATE_CHANGE_SUBSCRIPTION* subscription,
    _In_ PFEATURE_STATE_CHANGE_CALLBACK callback,
    _In_opt_ void* context
    );

EXTERN_C
void
UnsubscribeFeatureStateChangeNotification(
    _In_ _Post_invalid_ FEATURE_STATE_CHANGE_SUBSCRIPTION subscription
    );

EXTERN_C
UINT32
GetFeatureVariant(
    UINT32 featureId,
    FEATURE_CHANGE_TIME changeTime,
    _Out_ UINT32* payloadId,
    _Out_ BOOL* hasNotification
    );

#endif // _APISET_WIL_FEATURESTAGING_
