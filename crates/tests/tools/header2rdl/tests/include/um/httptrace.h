//
//    Copyright (C) Microsoft.  All rights reserved.
//
#ifndef _HTTP_TRACE_H
#define _HTTP_TRACE_H

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// HTTP_TRACE_TYPE contains type that are supported by
// the IIS trace infrastructure.
// The enum values match the equivalent VARIANT types.
//
enum HTTP_TRACE_TYPE
{ 
    HTTP_TRACE_TYPE_BYTE = 17,         // mof type "uint8"
    HTTP_TRACE_TYPE_USHORT = 18,       // mof type "uint16"
    HTTP_TRACE_TYPE_ULONG = 19,        // mof type "uint32"
    HTTP_TRACE_TYPE_ULONGLONG = 21,    // mof type "uint64"
    HTTP_TRACE_TYPE_CHAR = 16,         // mof type "int8"
    HTTP_TRACE_TYPE_SHORT = 2,         // mof type "int16"
    HTTP_TRACE_TYPE_LONG = 3,          // mof type "int32"
    HTTP_TRACE_TYPE_LONGLONG = 20,     // mof type "int64"
    HTTP_TRACE_TYPE_LPCWSTR = 31,      // mof type string "w"
    HTTP_TRACE_TYPE_LPCSTR = 30,       // mof type string "a"
    HTTP_TRACE_TYPE_LPCGUID = 72,      // mof type object "Guid"
    HTTP_TRACE_TYPE_BOOL = 11,         // mof type object "boolean"
};

// Bit values for the HTTP_TRACE_EVENT.dwFlags

//
// If HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS is set then
// consumers of the trace event can assume that
// pProviderGuid, pAreaGuid, pszEventName in the HTTP_TRACE_EVENT
// and pszName and pszDescription in each HTTP_TRACE_EVENT_ITEM
// entry are static (their memory is available throughout the 
// process lifetime). If event needs to be buffered there is no need
// to copy values for these fields
//
#define HTTP_TRACE_EVENT_FLAG_STATIC_DESCRIPTIVE_FIELDS       0x01

//
// In addition to TRACE_LEVEL_FATAL - TRACE_LEVEL_VERBOSE
//
#define HTTP_TRACE_LEVEL_START  6
#define HTTP_TRACE_LEVEL_END    7

struct HTTP_TRACE_EVENT_ITEM;

//
// Each Trace event will need to be described with the HTTP_TRACE_EVENT structure. 
// It is designed to contain all the information relevant to ETW, 
// but ETW is not the only consumer
//
//

struct HTTP_TRACE_EVENT
{
    //
    // Trace Provider (such as "WWW Server")
    //
    LPCGUID                 pProviderGuid;
    //
    // Trace Area Flag ( a bit flag that may represent area such as "Authentication")
    //
    DWORD                   dwArea;
    //
    // GUID equivalent to the Area Flag
    //
    LPCGUID                 pAreaGuid;
    //
    // ID of the event (in the ETW terminology this would be the Event Type)
    //
    DWORD                   dwEvent; 
    //
    // String matching the event ID (dwEvent)
    //
    LPCWSTR                 pszEventName;
    //
    // Event version
    //
    DWORD                   dwEventVersion;
    //
    // Verbosity of the event ( General, FatalError, Error, Warning, Info, Verbose)
    // In the ETW terminology this is the "event level"
    DWORD                   dwVerbosity;
    //
    // Activity GUID. For the http request based event
    // it may represent the request ID
    //
    LPCGUID                 pActivityGuid;
    //
    // Additional GUID to allow to associate 
    // multiple related activities
    //
    LPCGUID                 pRelatedActivityGuid;
    //
    // Optional timestamp (in GetTickCount() ticks )
    // If set to 0 then system will fill in the TickCount.
    //
    DWORD                   dwTimeStamp;
    //
    // Flags (reserved: Set to 0)
    //
    DWORD                   dwFlags;
    //
    //
    // Number of entries in the structured
    // description of the event
    // Note that the maximum allowed number of EventTraceItems
    // should match the ETW internal limit of 16.
    //
    DWORD                   cEventItems; 
    //
    // Array of individual entries describing
    // the event
    //
    _Field_size_full_(cEventItems)
    struct HTTP_TRACE_EVENT_ITEM * pEventItems;
};


// Each HTTP_TRACE_EVENT can have multiple items or elements that describe the event
// (for example the "GENERAL_REQUEST_START" event contains an item with the Request URL)

struct HTTP_TRACE_EVENT_ITEM
{
    //
    // Friendly name of the item (for example "RequestUrl")
    //
    LPCWSTR          pszName;
    //
    // Data type that pbData is pointed to
    //
    enum HTTP_TRACE_TYPE dwDataType;
    //
    // Pointer to the actual data (needs to be casted based on the dwDataType)
    // For example the HTTP_TRACE_EVENT_ITEM describing HTTP request URL would have
    // the pbData pointing to the actual URL string
    //
    _Field_size_bytes_full_(cbData)
    PBYTE   pbData;
    //
    // # of bytes of the actual data (it should matter only for string types)
    //
    DWORD   cbData;
    //
    // Additional description of the data - enumerations take advantage of this field
    // (always set to NULL if not used)
    //
    LPCWSTR pszDataDescription;
};


//
// HTTP_TRACE_CONFIGURATION is used by both trace providers and trace consumers
// Trace consumers declare the TraceProviders and TraceAreas of the interest.
// Trace providers can retrieve what areas at what verbosity is allowed
// for a given provider
//

struct HTTP_TRACE_CONFIGURATION
{
    LPCGUID     pProviderGuid;
    DWORD       dwAreas;
    DWORD       dwVerbosity;
    BOOL        fProviderEnabled;
};

//
// Definiton of the GUIDs for the TRACE Providers that ship with IIS
// or are related to IIS
//

DEFINE_GUID( GUID_IIS_ALL_TRACE_PROVIDERS,
             0x00000000,0x0000,0x0000,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00);

DEFINE_GUID( GUID_IIS_WWW_SERVER_TRACE_PROVIDER,
             0x3a2a4e84,0x4c21,0x4981,0xae,0x10,0x3f,0xda,0x0d,0x9b,0x0f,0x83);

// IIS V2(Crimson/Unified) trace provider
DEFINE_GUID( GUID_IIS_WWW_SERVER_V2_TRACE_PROVIDER,
             0xde4649c9,0x15e8,0x4fea,0x9d,0x85,0x1c,0xdd,0xa5,0x20,0xc3,0x34);

// ETW name for the provider: "ASP.NET Events
DEFINE_GUID( GUID_IIS_ASPNET_TRACE_PROVIDER,
             0xAFF081FE,0x0247,0x4275,0x9C,0x4E,0x02,0x1F,0x3D,0xC1,0xDA,0x35);

// ETW name for the provider: "IIS: Active Server Pages (ASP)"
DEFINE_GUID( GUID_IIS_ASP_TRACE_TRACE_PROVIDER,
             0x06b94d9a,0xb15e,0x456e,0xa4,0xef,0x37,0xc9,0x84,0xa2,0xcb,0x4b);

DEFINE_GUID( GUID_IIS_WWW_GLOBAL_TRACE_PROVIDER,
             0xd55d3bc9,0xcba9,0x44df,0x82,0x7e,0x13,0x2d,0x3a,0x45,0x96,0xc2);

// ETW name for the provider: "IIS: WWW Isapi Extension"
DEFINE_GUID( GUID_IIS_ISAPI_TRACE_PROVIDER,
             0xa1c2040e,0x8840,0x4c31,0xba,0x11,0x98,0x71,0x03,0x1a,0x19,0xea);



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif

