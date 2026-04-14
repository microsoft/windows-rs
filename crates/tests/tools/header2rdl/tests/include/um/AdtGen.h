/*+-----------------------------------------------------------------------

   Microsoft Windows

   Copyright (c) Microsoft Corporation 2000

   File:        A D T G E N . H

   Contents:    definitions of types/functions required for 
                generating generic audits.

                !!!WARNING!!!
                This file is included by lsarpc.idl, therefore, if you
                change it, make sure to clean build the entire DS depot.


   History:     
     07-January-2000  kumarp        created

------------------------------------------------------------------------*/

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4001) /* nonstandard extension: single line comment */
#pragma warning(disable:4201) /* nonstandard extension: nameless struct/union */
#pragma warning(disable:4820) /* padding added after data member */
#endif

#ifndef _ADTGEN_H
#define _ADTGEN_H
#include <winapifamily.h>

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)



//
// type of audit 
//
// AUDIT_TYPE_LEGACY 
//     In this case the audit event schema is stored in a .mc file.
//
// AUDIT_TYPE_WMI    
//     The schema is stored in WMI. (currently not supported)
//

#define AUDIT_TYPE_LEGACY 1
#define AUDIT_TYPE_WMI    2

//
// Type of parameters passed in the AUDIT_PARAMS.Parameters array
// 
// Use the AdtInitParams function to initialize and prepare 
// an array of audit parameters.
//

typedef enum _AUDIT_PARAM_TYPE
{
    //
    // do we need this?
    //

    APT_None = 1,

    //
    // NULL terminated string 
    //

    APT_String,

    //
    // unsigned long
    //

    APT_Ulong,

    //
    // a pointer. use for specifying handles/pointers
    // (32 bit on 32 bit systems and 64 bit on 64 bit systems)
    // Note that the memory to which the pointer points to
    // is not marshalled when using this type. Use this when you
    // are interested in the absolute value of the pointer.
    // A good example of this is when specifying HANDLE values.
    //

    APT_Pointer,

    //
    // SID
    //

    APT_Sid,

    //
    // Logon ID (LUID), this results in 3 output parameters.
    // 1. Account Name 2. Authority Name 3. LogonID
    //

    APT_LogonId,

    //
    // Object Type List
    //

    APT_ObjectTypeList,
    
    //
    // Luid (not translated to LogonId)
    //
    
    APT_Luid,
    
    //
    // Guid
    //
     
    APT_Guid,

    //
    // Time (FILETIME)
    //

    APT_Time,

    //
    // ULONGLONG
    //

    APT_Int64,
    
    //
    // IP Addess (IPv4 and IPv6). This logs the address as the 
    // first parameter and the port as the second. So ensure 
    // that 2 entries are added in the event message file, one
    // for the address and the immediate next entry as the port
    //

    APT_IpAddress,
    
    //
    // Logon ID with SID (LUID) results in 4 output parameters.
    // 1. SID 2. Account Name 3. Authority Name 4. LogonID
    //

    APT_LogonIdWithSid

} AUDIT_PARAM_TYPE;

// 
// There are two types of flags that can be used with a parameter.
//
// - formatting flag
//   This defines the appearance of a parameter when
//   written to the eventlog. Such flags may become obsolete
//   when we move to WMI auditing.
//
// - control flag
//   This causes a specified action to be taken that affects 
//   a parameter value.
//
//   For example:
//   If you use the AP_PrimaryLogonId/AP_ClientLogonId flag,
//   the system will capture the logon-id from the process/thread token.
//

#define AP_ParamTypeBits  8
#define AP_ParamTypeMask  0x000000ffL

//
// the flags values below have overlapping values. this is ok since
// the scope of each flag is limited to the type to which it applies.
//

//
// APT_Ulong : format flag : causes a number to appear in hex
//

#define AP_FormatHex      (0x0001L << AP_ParamTypeBits)

//
// APT_Ulong : format flag : causes a number to be treated as access-mask.
//                           The meaning of each bit depends on the associated
//                           object type.
//

#define AP_AccessMask     (0x0002L << AP_ParamTypeBits)

                                                       
//
// APT_String : format flag : causes a string to be treated as a file-path
//

#define AP_Filespec       (0x0001L << AP_ParamTypeBits)
                                                       
//
// APT_Sid : format flag : causes a sid to be treated as a logon-id
//

#define AP_SidAsLogonId   (0x0001L << AP_ParamTypeBits)

//
// APT_LogonId : control flag : logon-id is captured from the process token
//

#define AP_PrimaryLogonId (0x0001L << AP_ParamTypeBits)

//
// APT_LogonId : control flag : logon-id is captured from the thread token
//

#define AP_ClientLogonId  (0x0002L << AP_ParamTypeBits)


//
// internal helper macros
//

#define ApExtractType(TypeFlags)  ((AUDIT_PARAM_TYPE)(TypeFlags & AP_ParamTypeMask))
#define ApExtractFlags(TypeFlags) ((TypeFlags & ~AP_ParamTypeMask))

//
// Element of an object-type-list
//
// The AUDIT_OBJECT_TYPES structure identifies an object type element 
// in a hierarchy of object types. The AccessCheckByType functions use 
// an array of such structures to define a hierarchy of an object and 
// its subobjects, such as property sets and properties.
//

typedef struct _AUDIT_OBJECT_TYPE
{
    GUID        ObjectType;     // guid of the (sub)object
    USHORT      Flags;          // currently not defined
    USHORT      Level;          // level within the hierarchy. 
                                // 0 is the root level
    ACCESS_MASK AccessMask;     // access-mask for this (sub)object
} AUDIT_OBJECT_TYPE, *PAUDIT_OBJECT_TYPE;

typedef struct _AUDIT_OBJECT_TYPES
{
    USHORT Count;               // number of object-types in pObjectTypes
    USHORT Flags;               // currently not defined
#ifdef MIDL_PASS
    [size_is(Count)]
#endif
    AUDIT_OBJECT_TYPE* pObjectTypes; // array of object-types
} AUDIT_OBJECT_TYPES, *PAUDIT_OBJECT_TYPES;


//
// Maximum size of the SOCKADDR_STORAGE structure
//

#define _AUTHZ_SS_MAXSIZE 128                 // Maximum size


typedef struct _AUDIT_IP_ADDRESS
{
	BYTE pIpAddress[_AUTHZ_SS_MAXSIZE];
} AUDIT_IP_ADDRESS, *PAUDIT_IP_ADDRESS;

//
// Structure that defines a single audit parameter.
//
// LsaGenAuditEvent accepts an array of such elements to
// represent the parameters of the audit to be generated.
//
// It is best to initialize this structure using AdtInitParams function.
// This will ensure compatibility with any future changes to this
// structure.
//

typedef struct _AUDIT_PARAM
{
    AUDIT_PARAM_TYPE Type;      // type
    ULONG Length;               // currently unused
    DWORD Flags;                // currently unused

#ifdef MIDL_PASS
    [switch_type(AUDIT_PARAM_TYPE),switch_is(Type)]
#endif
    union 
    {
#ifdef MIDL_PASS
        [default]
#endif
        ULONG_PTR Data0;

#ifdef MIDL_PASS
        [case(APT_String)]
        [string]
#endif
        PWSTR  String;

        
#ifdef MIDL_PASS
        [case(APT_Ulong,
              APT_Pointer)]
#endif
        ULONG_PTR u;
        
#ifdef MIDL_PASS
        [case(APT_Sid)]
#endif
        SID* psid;
        
#ifdef MIDL_PASS
        [case(APT_Guid)]
#endif
        GUID* pguid;

#ifdef MIDL_PASS
        [case(APT_LogonId)]
#endif
        ULONG LogonId_LowPart;

#ifdef MIDL_PASS
        [case(APT_ObjectTypeList)]
#endif
        AUDIT_OBJECT_TYPES* pObjectTypes;
        
#ifdef MIDL_PASS
        [case(APT_IpAddress)]
#endif
        AUDIT_IP_ADDRESS* pIpAddress;
    } DUMMYUNIONNAME;

#ifdef MIDL_PASS
    [switch_type(AUDIT_PARAM_TYPE),switch_is(Type)]
#endif
    union 
    {
#ifdef MIDL_PASS
        [default]
#endif
        ULONG_PTR Data1;

#ifdef MIDL_PASS
        [case(APT_LogonId)]
#endif
        LONG LogonId_HighPart;
    } DUMMYUNIONNAME2;
    
} AUDIT_PARAM, *PAUDIT_PARAM;

//
// Audit control flags. To be used with AUDIT_PARAMS.Flags
//

#define APF_AuditFailure 0x00000000  // generate a failure audit
#define APF_AuditSuccess 0x00000001  // generate a success audit when set,
                                     // a failure audit otherwise. 

//
// set of valid audit control flags 
//

#define APF_ValidFlags   (APF_AuditSuccess)

//
// Audit parameters passed to LsaGenAuditEvent
//

typedef struct _AUDIT_PARAMS
{
    ULONG  Length;              // size in bytes
    DWORD  Flags;               // currently unused
    USHORT Count;               // number of parameters
#ifdef MIDL_PASS
    [size_is(Count)]
#endif    
    AUDIT_PARAM* Parameters;    // array of parameters
} AUDIT_PARAMS, *PAUDIT_PARAMS;

//
// Defines the elements of a legacy audit event.
//

typedef struct _AUTHZ_AUDIT_EVENT_TYPE_LEGACY
{
    //
    // Audit category ID
    //

    USHORT CategoryId;

    //
    // Audit event ID
    //

    USHORT AuditId;

    //
    // Parameter count
    //

    USHORT ParameterCount;
    
} AUTHZ_AUDIT_EVENT_TYPE_LEGACY, *PAUTHZ_AUDIT_EVENT_TYPE_LEGACY;

typedef
#ifdef MIDL_PASS
[switch_type(BYTE)]
#endif
union _AUTHZ_AUDIT_EVENT_TYPE_UNION
{
#ifdef MIDL_PASS
        [case(AUDIT_TYPE_LEGACY)]
#endif
        AUTHZ_AUDIT_EVENT_TYPE_LEGACY Legacy;
} AUTHZ_AUDIT_EVENT_TYPE_UNION, *PAUTHZ_AUDIT_EVENT_TYPE_UNION;

//
// description of an audit event
//

typedef
struct _AUTHZ_AUDIT_EVENT_TYPE_OLD
{
    // version number

    ULONG Version;
    DWORD dwFlags;
    LONG  RefCount;
    ULONG_PTR hAudit;
    LUID  LinkId;
#ifdef MIDL_PASS
    [switch_is(Version)] 
#endif
    AUTHZ_AUDIT_EVENT_TYPE_UNION u;

} AUTHZ_AUDIT_EVENT_TYPE_OLD;

typedef
#ifdef MIDL_PASS
[handle]
#endif
AUTHZ_AUDIT_EVENT_TYPE_OLD* PAUTHZ_AUDIT_EVENT_TYPE_OLD;

//
// Flags for AUTHZ_AUDIT_EVENT_TYPE_OLD
//
#define AUTHZP_WPD_EVENT 0x10

typedef
#ifdef MIDL_PASS
[context_handle]
#endif
PVOID AUDIT_HANDLE, *PAUDIT_HANDLE;

//
// Begin support for extensible auditing.
//

//
// Registration Flags.
//

#define AUTHZ_ALLOW_MULTIPLE_SOURCE_INSTANCES 0x1
#define AUTHZ_MIGRATED_LEGACY_PUBLISHER       0x2

//
// Audit Generation Flags.
//

#define AUTHZ_AUDIT_INSTANCE_INFORMATION 0x2


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif //_ADTGEN_H

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
