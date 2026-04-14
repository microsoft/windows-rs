/*++
Copyright (C) Microsoft Corporation.  All rights reserved. 


Module Name:

    evcoll.h

Abstract:

    Event Collector API 

--*/


#ifndef __EVCOLL_H__
#define __EVCOLL_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C"
{
#endif


typedef HANDLE EC_HANDLE;
typedef HANDLE EC_OBJECT_ARRAY_PROPERTY_HANDLE;


// 
// Access Control Permissions
//

/*--------------------------------------------------------------------------
  Subscription Definitions
  --------------------------------------------------------------------------*/

typedef enum _EC_SUBSCRIPTION_PROPERTY_ID
{
    EcSubscriptionEnabled = 0,           // EcVarTypeBoolean
    EcSubscriptionEventSources,          // EcVarObjectArrayPropertyHandle
    EcSubscriptionEventSourceAddress,    // EcVarTypeString
    EcSubscriptionEventSourceEnabled,    // EcVarTypeBoolean
    EcSubscriptionEventSourceUserName,   // EcVarTypeString
    EcSubscriptionEventSourcePassword,   // EcVarTypeString
    EcSubscriptionDescription,           // EcVarTypeString
    EcSubscriptionURI,                   // EcVarTypeString
    EcSubscriptionConfigurationMode,     // EcVarTypeUInt32, EC_SUBSCRIPTION_CONFIGURATION_MODE
    EcSubscriptionExpires,               // EcVarTypeDateTime
    EcSubscriptionQuery,                 // EcVarTypeString
    EcSubscriptionTransportName,         // EcVarTypeString
    EcSubscriptionTransportPort,         // EcVarTypeUInt32        
    EcSubscriptionDeliveryMode,          // EcVarTypeUInt32, EC_SUBSCRIPTION_DELIVERY_MODE
    EcSubscriptionDeliveryMaxItems,      // EcVarTypeUInt32
    EcSubscriptionDeliveryMaxLatencyTime, // EcVarTypeUInt32
    EcSubscriptionHeartbeatInterval,     // EcVarTypeUInt32
    EcSubscriptionLocale,                // EcVarTypeString
    EcSubscriptionContentFormat,         // EcVarTypeUInt32, EC_SUBSCRIPTION_CONTENT_FORMAT
    EcSubscriptionLogFile,               // EcVarTypeString
    EcSubscriptionPublisherName,         // EcVarTypeString
    EcSubscriptionCredentialsType,       // EcVarTypeUInt32, EC_SUBSCRIPTION_CREDENTIALS_TYPE
    EcSubscriptionCommonUserName,        // EcVarTypeString
    EcSubscriptionCommonPassword,        // EcVarTypeString
    EcSubscriptionHostName,              // EcVarTypeString    
    EcSubscriptionReadExistingEvents,    // EcVarTypeBoolean
    EcSubscriptionDialect,               // EcVarTypeString
    EcSubscriptionType,              // EcVarTypeUInt32, EC_SUBSCRIPTION_TYPE
    EcSubscriptionAllowedIssuerCAs,  // EcVarTypeString  | EcArrayBitMask
    EcSubscriptionAllowedSubjects,   // EcVarTypeString  | EcArrayBitMask
    EcSubscriptionDeniedSubjects,   // EcVarTypeString  | EcArrayBitMask
    EcSubscriptionAllowedSourceDomainComputers,	// EcVarTypeString SDDL
    EcSubscriptionPropertyIdEND
} EC_SUBSCRIPTION_PROPERTY_ID;

typedef enum _EC_SUBSCRIPTION_CREDENTIALS_TYPE {
    EcSubscriptionCredDefault = 0,
    EcSubscriptionCredNegotiate,
    EcSubscriptionCredDigest,
    EcSubscriptionCredBasic,
    EcSubscriptionCredLocalMachine
} EC_SUBSCRIPTION_CREDENTIALS_TYPE;

typedef enum _EC_SUBSCRIPTION_TYPE {
    EcSubscriptionTypeSourceInitiated = 0,
    EcSubscriptionTypeCollectorInitiated = 1
} EC_SUBSCRIPTION_TYPE;

typedef enum _EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID
{
    EcSubscriptionRunTimeStatusActive = 0, // EcVarTypeUInt32, EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS
    EcSubscriptionRunTimeStatusLastError,  // EcVarTypeUInt32
    EcSubscriptionRunTimeStatusLastErrorMessage,  // EcVarTypeString
    EcSubscriptionRunTimeStatusLastErrorTime,  // EcVarTypeDateTime
    EcSubscriptionRunTimeStatusNextRetryTime,  // EcVarTypeDateTime
    EcSubscriptionRunTimeStatusEventSources, // EcVarTypeString | ArrayBitMask
    EcSubscriptionRunTimeStatusLastHeartbeatTime, // EcVarTypeDateTime
    EcSubscriptionRunTimeStatusInfoIdEND
} EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID;

typedef enum _EC_VARIANT_TYPE
{
    EcVarTypeNull = 0,
    EcVarTypeBoolean,
    EcVarTypeUInt32,
    EcVarTypeDateTime,
    EcVarTypeString,
    EcVarObjectArrayPropertyHandle
} EC_VARIANT_TYPE;


#define EC_VARIANT_TYPE_MASK 0x7f
#define EC_VARIANT_TYPE_ARRAY 128


typedef struct _EC_VARIANT
{
    union 
    {
        BOOL       BooleanVal;
        UINT32     UInt32Val;
        ULONGLONG  DateTimeVal;
        LPCWSTR    StringVal;
        PBYTE      BinaryVal; 
        BOOL*      BooleanArr;
        INT32*     Int32Arr;
        LPWSTR*    StringArr;
        EC_OBJECT_ARRAY_PROPERTY_HANDLE PropertyHandleVal;
    };

    DWORD Count;   // number of elements not length in bytes.  Used for arrays
                   // and binary/string types. 
    DWORD Type;

} EC_VARIANT, *PEC_VARIANT;



#define EC_READ_ACCESS    1
#define EC_WRITE_ACCESS   2

#define EC_OPEN_ALWAYS 0
#define EC_CREATE_NEW 1
#define EC_OPEN_EXISTING 2

typedef enum _EC_SUBSCRIPTION_CONFIGURATION_MODE
{
    EcConfigurationModeNormal = 0,
    EcConfigurationModeCustom,        
    EcConfigurationModeMinLatency,
    EcConfigurationModeMinBandwidth

} EC_SUBSCRIPTION_CONFIGURATION_MODE;

typedef enum _EC_SUBSCRIPTION_DELIVERY_MODE
{
    EcDeliveryModePull = 1,
    EcDeliveryModePush

} EC_SUBSCRIPTION_DELIVERY_MODE;


typedef enum _EC_SUBSCRIPTION_CONTENT_FORMAT
{
    EcContentFormatEvents = 1,
    EcContentFormatRenderedText
} EC_SUBSCRIPTION_CONTENT_FORMAT;


typedef enum _EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS
{
    EcRuntimeStatusActiveStatusDisabled = 1,
    EcRuntimeStatusActiveStatusActive,
    EcRuntimeStatusActiveStatusInactive,
    EcRuntimeStatusActiveStatusTrying
} EC_SUBSCRIPTION_RUNTIME_STATUS_ACTIVE_STATUS;


EC_HANDLE WINAPI EcOpenSubscriptionEnum(
    DWORD Flags 
    );

BOOL WINAPI EcEnumNextSubscription( 
    EC_HANDLE SubscriptionEnum,
    DWORD SubscriptionNameBufferSize,
    _Out_writes_to_opt_(SubscriptionNameBufferSize,*SubscriptionNameBufferUsed)
    LPWSTR SubscriptionNameBuffer,
    _Out_ PDWORD SubscriptionNameBufferUsed
    );

EC_HANDLE WINAPI EcOpenSubscription(
    LPCWSTR SubscriptionName,
    DWORD AccessMask,
    DWORD Flags 
    );

BOOL WINAPI EcSetSubscriptionProperty( 
    EC_HANDLE Subscription,
    EC_SUBSCRIPTION_PROPERTY_ID PropertyId,
    DWORD Flags,
    PEC_VARIANT PropertyValue
    );

BOOL WINAPI EcGetSubscriptionProperty( 
    EC_HANDLE Subscription,
    EC_SUBSCRIPTION_PROPERTY_ID PropertyId,
    DWORD Flags,
    DWORD PropertyValueBufferSize,
    PEC_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed 
    );

BOOL WINAPI EcSaveSubscription( 
    EC_HANDLE Subscription,
    DWORD Flags
    );

BOOL WINAPI EcDeleteSubscription(
    LPCWSTR SubscriptionName,
    DWORD   Flags 
    );

BOOL WINAPI EcGetObjectArraySize( 
    EC_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray,
    _Out_ PDWORD ObjectArraySize 
    );

BOOL WINAPI EcSetObjectArrayProperty( 
    EC_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray, 
    EC_SUBSCRIPTION_PROPERTY_ID PropertyId,
    DWORD ArrayIndex,
    DWORD Flags,
    PEC_VARIANT PropertyValue
    );
 
BOOL WINAPI EcGetObjectArrayProperty( 
    EC_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray, 
    EC_SUBSCRIPTION_PROPERTY_ID PropertyId,
    DWORD ArrayIndex,
    DWORD Flags,
    DWORD PropertyValueBufferSize,
    PEC_VARIANT PropertyValueBuffer,
    _Out_ PDWORD PropertyValueBufferUsed 
    );

BOOL WINAPI EcInsertObjectArrayElement( 
    EC_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray, 
    DWORD ArrayIndex 
    );

BOOL WINAPI EcRemoveObjectArrayElement( 
    EC_OBJECT_ARRAY_PROPERTY_HANDLE ObjectArray, 
    DWORD ArrayIndex 
    );

BOOL WINAPI EcGetSubscriptionRunTimeStatus( 
    LPCWSTR SubscriptionName,
    EC_SUBSCRIPTION_RUNTIME_STATUS_INFO_ID StatusInfoId,
    LPCWSTR EventSourceName,
    DWORD Flags,
    DWORD StatusValueBufferSize,
    PEC_VARIANT StatusValueBuffer,
    _Out_ PDWORD StatusValueBufferUsed 
    );

BOOL WINAPI EcRetrySubscription(
    LPCWSTR SubscriptionName,
    LPCWSTR EventSourceName,
    DWORD Flags 
    );

BOOL WINAPI EcClose( 
    EC_HANDLE Object
    );


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __EC_H__



