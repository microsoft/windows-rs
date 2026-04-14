#include <winapifamily.h>

/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    devpropdef.h

Abstract:

    Defines property types and keys for the Plug and Play Device Property API.

Environment:

    User and Kernel modes.

--*/


#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#ifndef _DEVPROPDEF_H_
#define _DEVPROPDEF_H_

//
// Type definition for property data types.  Valid DEVPROPTYPE values are
// constructed from base DEVPROP_TYPE_ values, which may be modified by a
// logical OR with DEVPROP_TYPEMOD_ values, as appropriate.
//
typedef ULONG DEVPROPTYPE, *PDEVPROPTYPE;

//
// Property type modifiers.  Used to modify base DEVPROP_TYPE_ values, as
// appropriate.  Not valid as standalone DEVPROPTYPE values.
//
#define DEVPROP_TYPEMOD_ARRAY                   0x00001000  // array of fixed-sized data elements
#define DEVPROP_TYPEMOD_LIST                    0x00002000  // list of variable-sized data elements

//
// Property data types.
//
#define DEVPROP_TYPE_EMPTY                      0x00000000  // nothing, no property data
#define DEVPROP_TYPE_NULL                       0x00000001  // null property data
#define DEVPROP_TYPE_SBYTE                      0x00000002  // 8-bit signed int (SBYTE)
#define DEVPROP_TYPE_BYTE                       0x00000003  // 8-bit unsigned int (BYTE)
#define DEVPROP_TYPE_INT16                      0x00000004  // 16-bit signed int (SHORT)
#define DEVPROP_TYPE_UINT16                     0x00000005  // 16-bit unsigned int (USHORT)
#define DEVPROP_TYPE_INT32                      0x00000006  // 32-bit signed int (LONG)
#define DEVPROP_TYPE_UINT32                     0x00000007  // 32-bit unsigned int (ULONG)
#define DEVPROP_TYPE_INT64                      0x00000008  // 64-bit signed int (LONG64)
#define DEVPROP_TYPE_UINT64                     0x00000009  // 64-bit unsigned int (ULONG64)
#define DEVPROP_TYPE_FLOAT                      0x0000000A  // 32-bit floating-point (FLOAT)
#define DEVPROP_TYPE_DOUBLE                     0x0000000B  // 64-bit floating-point (DOUBLE)
#define DEVPROP_TYPE_DECIMAL                    0x0000000C  // 128-bit data (DECIMAL)
#define DEVPROP_TYPE_GUID                       0x0000000D  // 128-bit unique identifier (GUID)
#define DEVPROP_TYPE_CURRENCY                   0x0000000E  // 64 bit signed int currency value (CURRENCY)
#define DEVPROP_TYPE_DATE                       0x0000000F  // date (DATE)
#define DEVPROP_TYPE_FILETIME                   0x00000010  // file time (FILETIME)
#define DEVPROP_TYPE_BOOLEAN                    0x00000011  // 8-bit boolean (DEVPROP_BOOLEAN)
#define DEVPROP_TYPE_STRING                     0x00000012  // null-terminated string
#define DEVPROP_TYPE_STRING_LIST (DEVPROP_TYPE_STRING|DEVPROP_TYPEMOD_LIST) // multi-sz string list
#define DEVPROP_TYPE_SECURITY_DESCRIPTOR        0x00000013  // self-relative binary SECURITY_DESCRIPTOR
#define DEVPROP_TYPE_SECURITY_DESCRIPTOR_STRING 0x00000014  // security descriptor string (SDDL format)
#define DEVPROP_TYPE_DEVPROPKEY                 0x00000015  // device property key (DEVPROPKEY)
#define DEVPROP_TYPE_DEVPROPTYPE                0x00000016  // device property type (DEVPROPTYPE)
#define DEVPROP_TYPE_BINARY      (DEVPROP_TYPE_BYTE|DEVPROP_TYPEMOD_ARRAY)  // custom binary data
#define DEVPROP_TYPE_ERROR                      0x00000017  // 32-bit Win32 system error code
#define DEVPROP_TYPE_NTSTATUS                   0x00000018  // 32-bit NTSTATUS code
#define DEVPROP_TYPE_STRING_INDIRECT            0x00000019  // string resource (@[path\]<dllname>,-<strId>)

//
// Max base DEVPROP_TYPE_ and DEVPROP_TYPEMOD_ values.
//
#define MAX_DEVPROP_TYPE                        0x00000019  // max valid DEVPROP_TYPE_ value
#define MAX_DEVPROP_TYPEMOD                     0x00002000  // max valid DEVPROP_TYPEMOD_ value

//
// Bitmasks for extracting DEVPROP_TYPE_ and DEVPROP_TYPEMOD_ values.
//
#define DEVPROP_MASK_TYPE                       0x00000FFF  // range for base DEVPROP_TYPE_ values
#define DEVPROP_MASK_TYPEMOD                    0x0000F000  // mask for DEVPROP_TYPEMOD_ type modifiers


//
// Property type specific data types.
//

// 8-bit boolean type definition for DEVPROP_TYPE_BOOLEAN (True=-1, False=0)
typedef CHAR DEVPROP_BOOLEAN, *PDEVPROP_BOOLEAN;
#define DEVPROP_TRUE  ((DEVPROP_BOOLEAN)-1)
#define DEVPROP_FALSE ((DEVPROP_BOOLEAN) 0)


//
// DEVPROPKEY structure
//

#ifndef DEVPROPKEY_DEFINED
#define DEVPROPKEY_DEFINED

typedef GUID  DEVPROPGUID, *PDEVPROPGUID;
typedef ULONG DEVPROPID,   *PDEVPROPID;

typedef struct _DEVPROPKEY {
    DEVPROPGUID fmtid;
    DEVPROPID   pid;
} DEVPROPKEY, *PDEVPROPKEY;

#endif // DEVPROPKEY_DEFINED

#ifndef IsEqualDevPropKey
#ifdef __cplusplus
#define IsEqualDevPropKey(a, b)   (((a).pid == (b).pid) && IsEqualGUID((a).fmtid, (b).fmtid))
#ifdef _SYS_GUID_OPERATOR_EQ_
extern "C++" {
inline bool operator==(const DEVPROPKEY &a, const DEVPROPKEY &b) { return ((a.pid == b.pid) && (a.fmtid == b.fmtid)); }
inline bool operator!=(const DEVPROPKEY &a, const DEVPROPKEY &b) { return !(a == b); }
}
#endif // _SYS_GUID_OPERATOR_EQ_
#else // !__cplusplus
#define IsEqualDevPropKey(a, b)   (((a).pid == (b).pid) && IsEqualGUID(&(a).fmtid, &(b).fmtid))
#endif // __cplusplus
#endif // !IsEqualDevPropKey

//
// DEVPROPSTORE Enumeration
//
// This enumeration describes where a property is stored.
//

typedef
#ifdef MIDL_PASS
[v1_enum]
#endif
enum _DEVPROPSTORE {
    DEVPROP_STORE_SYSTEM,
    DEVPROP_STORE_USER
} DEVPROPSTORE, *PDEVPROPSTORE;

//
// DEVPROPCOMPKEY structure
//
// This structure represents a compound key for a property.
//

typedef struct _DEVPROPCOMPKEY {
    DEVPROPKEY Key;
    DEVPROPSTORE Store;
#ifdef MIDL_PASS
    [string] wchar_t *LocaleName;
#else
    PCWSTR LocaleName;
#endif
} DEVPROPCOMPKEY, *PDEVPROPCOMPKEY;

#ifndef IsEqualLocaleName
#define IsEqualLocaleName(a, b) (((a) == (b)) || (((a) != NULL) && ((b) != NULL) && (_wcsicmp((a), (b)) == 0)))
#endif

#ifndef IsEqualDevPropCompKey
#define IsEqualDevPropCompKey(a, b) (IsEqualDevPropKey((a).Key, (b).Key) && ((a).Store == (b).Store) && IsEqualLocaleName((a).LocaleName, (b).LocaleName))
#ifdef __cplusplus
#ifdef _SYS_GUID_OPERATOR_EQ_
extern "C++" {
inline bool operator==(const DEVPROPCOMPKEY &a, const DEVPROPCOMPKEY &b) { return ((a.Key == b.Key) && (a.Store == b.Store) && IsEqualLocaleName(a.LocaleName, b.LocaleName)); }
inline bool operator!=(const DEVPROPCOMPKEY &a, const DEVPROPCOMPKEY &b) { return !(a == b); }
}
#endif // _SYS_GUID_OPERATOR_EQ_
#endif // __cplusplus
#endif // !IsEqualDevPropCompKey

//
// DEVPROPERTY structure
//

typedef struct _DEVPROPERTY {
    DEVPROPCOMPKEY CompKey;
    DEVPROPTYPE Type;
    ULONG BufferSize;
#ifdef MIDL_PASS
    [size_is(BufferSize)] BYTE *Buffer;
#else
    __field_bcount_opt(BufferSize) PVOID Buffer;
#endif
} DEVPROPERTY, *PDEVPROPERTY;


//
// All valid DEVPROPKEY definitions must use a PROPID that is equal to or greater
// than DEVPROPID_FIRST_USABLE.
//
#define DEVPROPID_FIRST_USABLE 2

#endif // _DEVPROPDEF_H_


#ifdef DEFINE_DEVPROPKEY
#undef DEFINE_DEVPROPKEY
#endif
#ifdef INITGUID
#define DEFINE_DEVPROPKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) EXTERN_C const DEVPROPKEY DECLSPEC_SELECTANY name = { { l, w1, w2, { b1, b2,  b3,  b4,  b5,  b6,  b7,  b8 } }, pid }
#else
#define DEFINE_DEVPROPKEY(name, l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, pid) EXTERN_C const DEVPROPKEY name
#endif // INITGUID

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion
