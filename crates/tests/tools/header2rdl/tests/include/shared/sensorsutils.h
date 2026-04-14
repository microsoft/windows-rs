/*++

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    SensorsUtils.h

Abstract:

    This file contains utility functions for V2 Sensors

--*/

#if _MSC_VER > 1000
#pragma once
#endif

#include <SensorsStructures.h>

#ifdef __cplusplus
extern "C" {
#endif

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// TimeStamp Helpers                                                         //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

NTSTATUS
GetPerformanceTime(
    _Out_ PULONG TimeMs
    );

#define MILLISECONDS_TO_100NANOSECONDS(durationMs) ((durationMs) * 1000 * 10)
#define MILLISECONDS_FROM_100NANOSECONDS(durationNanoS) ((durationNanoS) / (1000 * 10))

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// PropVariant Helper                                                        //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

HRESULT
InitPropVariantFromFloat(
    _In_ FLOAT fltVal, 
    _Out_ PROPVARIANT *ppropvar
    );

NTSTATUS
PropKeyFindKeyGetPropVariant(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _In_ BOOLEAN TypeCheck,
    _Inout_ PROPVARIANT *pValue
    );

_At_(pList->AllocatedSizeInBytes, _Const_)
_At_(pList->Count, _Const_)
NTSTATUS
PropKeyFindKeySetPropVariant(
    _Inout_ SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _In_ BOOLEAN TypeCheck,
    _In_ PROPVARIANT *pValue
    );

NTSTATUS
PropKeyFindKeyGetFileTime(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ FILETIME *pRetValue);

NTSTATUS
PropKeyFindKeyGetGuid(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ GUID *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetBool(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ BOOL *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetUlong(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ ULONG *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetUshort(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ USHORT *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetFloat(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ FLOAT *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetDouble(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ double *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetInt32(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ INT32 *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetInt64(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _Out_ INT64 *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetNthUlong(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _In_ const UINT32 Occurrence,
    _Out_ ULONG *pRetValue
    );
    
NTSTATUS
PropKeyFindKeyGetNthUshort(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _In_ const UINT32 Occurrence,
    _Out_ USHORT *pRetValue
    );

NTSTATUS
PropKeyFindKeyGetNthInt64(
    _In_ const SENSOR_COLLECTION_LIST *pList,
    _In_ const PROPERTYKEY *pKey,
    _In_ const UINT32 Occurrence,
    _Out_ INT64 *pRetValue
    );
    
BOOLEAN
IsKeyPresentInPropertyList(
    _In_ PSENSOR_PROPERTY_LIST pList,
    _In_ const PROPERTYKEY *pKey
    );

BOOLEAN
IsKeyPresentInCollectionList(
    _In_ PSENSOR_COLLECTION_LIST pList,
    _In_ const PROPERTYKEY *pKey
    );

BOOLEAN
IsCollectionListSame(
    _In_ const PSENSOR_COLLECTION_LIST ListA,
    _In_ const PSENSOR_COLLECTION_LIST ListB
    );

NTSTATUS
PropVariantGetInformation(  
    _In_ const PROPVARIANT *PropVariantValue,
    _Out_opt_ ULONG *PropVariantOffset,
    _Out_opt_ ULONG *PropVariantSize,
    _Outptr_opt_result_bytebuffer_maybenull_(*PropVariantSize) PVOID *PropVariantPointer,
    _Out_opt_ DEVPROPTYPE *RemappedType
    );

NTSTATUS
PropertiesListCopy(
    _Inout_ PSENSOR_PROPERTY_LIST Target,
    _In_ const PSENSOR_PROPERTY_LIST Source
    );

ULONG
PropertiesListGetFillableCount(
    _In_ ULONG BufferSizeBytes
    );

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// Collection List Marshaling Legacy Helpers                                 //
// Architecture-specific - Avoid using for data across process boundaries    //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

ULONG
CollectionsListGetMarshalledSize(
    _In_ const PSENSOR_COLLECTION_LIST Collection
    );

NTSTATUS
CollectionsListCopyAndMarshall(
    _Inout_ PSENSOR_COLLECTION_LIST Target,
    _In_ const PSENSOR_COLLECTION_LIST Source
    );

NTSTATUS
CollectionsListMarshall(
    _Inout_ PSENSOR_COLLECTION_LIST Target
    );

ULONG
CollectionsListGetMarshalledSizeWithoutSerialization(
    _In_ const PSENSOR_COLLECTION_LIST Collection
    );

NTSTATUS
CollectionsListUpdateMarshalledPointer(
    _Inout_ PSENSOR_COLLECTION_LIST Collection
    );

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// Collection List Serialization Helpers                                     //
// Architecture-independent - Safe to use for data across process boundaries //
// (for example, during DeviceIoControl)                                     //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

NTSTATUS
SerializationBufferAllocate(
    _In_ ULONG SizeInBytes,
    _Outptr_result_bytebuffer_(SizeInBytes) BYTE** pBuffer
    );

VOID
SerializationBufferFree(
    _In_opt_ BYTE* Buffer
    );

ULONG
CollectionsListGetSerializedSize(
    _In_ const PSENSOR_COLLECTION_LIST Collection
    );

NTSTATUS
CollectionsListSerializeToBuffer(
    _In_ const PSENSOR_COLLECTION_LIST SourceCollection,
    _In_ ULONG TargetBufferSizeInBytes,
    _Out_writes_bytes_(TargetBufferSizeInBytes) BYTE* TargetBuffer
    );

NTSTATUS
CollectionsListAllocateBufferAndSerialize(
    _In_ const PSENSOR_COLLECTION_LIST SourceCollection,
    _Out_ ULONG* pTargetBufferSizeInBytes,
    _Outptr_result_bytebuffer_(*pTargetBufferSizeInBytes) BYTE** pTargetBuffer
    );

NTSTATUS
CollectionsListDeserializeFromBuffer(
    _In_ ULONG SourceBufferSizeInBytes,
    _In_reads_bytes_(SourceBufferSizeInBytes) const BYTE* SourceBuffer,
    _Inout_ PSENSOR_COLLECTION_LIST TargetCollection
    );

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// Collection List Helper                                                    //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////

NTSTATUS
SensorCollectionGetAt(
    _In_ UINT32 Index,
    _In_ PSENSOR_COLLECTION_LIST pSensorsList,
    _Out_opt_ PROPERTYKEY* pKey,
    _Out_opt_ PROPVARIANT* pValue
    );

ULONG
CollectionsListGetFillableCount(
    _In_ ULONG BufferSizeBytes
    );
    
BOOLEAN 
EvaluateActivityThresholds(
    _In_ PSENSOR_COLLECTION_LIST newSample,
    _In_ PSENSOR_COLLECTION_LIST oldSample,
    _In_ PSENSOR_COLLECTION_LIST thresholds
    );
    
NTSTATUS 
CollectionsListSortSubscribedActivitiesByConfidence(
    _In_ PSENSOR_COLLECTION_LIST thresholds,
    _Inout_updates_bytes_(pCollection->AllocatedSizeInBytes) PSENSOR_COLLECTION_LIST pCollection
    );

HRESULT
InitPropVariantFromCLSIDArray(
    _In_reads_(size) GUID *members,
    _In_ ULONG size,
    _Out_ PROPVARIANT *ppropvar
);

BOOLEAN
IsSensorSubscribed(
    _In_ PSENSOR_COLLECTION_LIST subscriptionList,
    _In_ GUID currentType
);

BOOLEAN
IsGUIDPresentInList(
    _In_reads_(arrayLength) const GUID *guidArray,
    _In_ const ULONG arrayLength,
    _In_ const GUID *guidElem
);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#ifdef __cplusplus
    }
#endif

