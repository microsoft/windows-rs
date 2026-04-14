/*============================================================================
 * Copyright (C) Microsoft Corporation, All rights reserved.
 *============================================================================
 */

/*
**==============================================================================
**
** Management Interface (MI)
**
**     This file defines the management interface.
**
**==============================================================================
*/

#include <winapifamily.h>

#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)

#ifndef _MI_h
#define _MI_h

#include <sal.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#ifdef _PREFAST_
 #pragma prefast(push)
 #pragma prefast(disable:6001)
 #pragma prefast(disable:6101)
 #pragma prefast(disable:28922)
#endif


/*
**==============================================================================
**
**  Use eight-byte packing for structures (on Windows)
**
**==============================================================================
*/

# pragma pack(push,8)

/*
**==============================================================================
**
** MI_CHAR_TYPE
**
**     Indicates the character type to use (1='char', 2='wchar_t'). This
**     can be overridden prior to including this file or on the compiler
**     command line (e.g., -DMI_CHAR_TYPE=1).
**
**==============================================================================
*/

#if defined(MI_CHAR_TYPE)
# if (MI_CHAR_TYPE != 1) && (MI_CHAR_TYPE != 2)
#  error "MI_CHAR_TYPE must be 1 or 2"
# endif
#else
#  define MI_CHAR_TYPE 2
#endif

#if (MI_CHAR_TYPE == 2)
# define MI_USE_WCHAR
#endif

/*
**==============================================================================
**
** MI_CONST
**
**     Provider managers and client transport implementations predefine this
**     macro to relax const checking on various structures within this file.
**
**==============================================================================
*/

#ifndef MI_CONST
# define MI_CONST const
#endif

/*
**==============================================================================
**
** MI_VERSION
**
**     Define the version number used the MI_Module.version field, which is
**     set in the MI_Main() entry point as follows:
**
**         module.version = MI_VERSION;
**
**==============================================================================
*/

#define MI_MAJOR   ((MI_Uint32)1)
#define MI_MINOR   ((MI_Uint32)0)
#define MI_REVISON ((MI_Uint32)0)
#define MI_MAKE_VERSION(MAJ, MIN, REV) ((MAJ << 16) | (MIN << 8) | REV)
#define MI_VERSION MI_MAKE_VERSION(MI_MAJOR, MI_MINOR, MI_REVISON)

#define MI_UNREFERENCED_PARAMETER(P)          (P)

/*
**==============================================================================
**
** SAL notation (Windows only)
**
**     If the SAL macros are undefined, define empty ones.
**
**==============================================================================
*/

#if !defined(_In_)
# define _In_
#endif

#if !defined(_In_opt_)
# define _In_opt_
#endif

#if !defined(_In_z_)
# define _In_z_
#endif

#if !defined(_In_opt_z_)
# define _In_opt_z_
#endif

#if !defined(_Out_)
# define _Out_
#endif

#if !defined(_Outptr_)
# define _Outptr_
#endif

#if !defined(_Out_writes_z_)
# define _Out_writes_z_(count)
#endif

#if !defined(_Outptr_result_z_)
# define _Outptr_result_z_
#endif

#if !defined(_Outptr_result_maybenull_)
# define _Outptr_result_maybenull_
#endif

#if !defined(_Outptr_opt_result_maybenull_)
# define _Outptr_opt_result_maybenull_
#endif

#if !defined(_Outptr_result_maybenull_z_)
# define _Outptr_result_maybenull_z_
#endif

#if !defined(_Outptr_opt_result_z_)
# define _Outptr_opt_result_z_
#endif

#if !defined(_Outptr_opt_result_maybenull_z_)
# define _Outptr_opt_result_maybenull_z_
#endif

#if !defined(_Return_type_success_)
# define _Return_type_success_(expr)
#endif

#if !defined(_In_reads_opt_)
# define _In_reads_opt_(expr)
#endif

#if !defined(_Out_writes_to_opt_)
# define _Out_writes_to_opt_(length, lengthwritten)
#endif

#if !defined(_Always_)
# define _Always_(annotation)
#endif

/*
**==============================================================================
**
** MI_EXPORT
**
**     This macro exports the MI_Main() entry point. For example:
**
**         MI_EXPORT MI_Module* MI_MAIN_CALL MI_Main(MI_Server* server)
**         {
**         }
**
**     The macro appears first in the definition.
**
**==============================================================================
*/

# define MI_EXPORT __declspec(dllexport)

/*
**==============================================================================
**
** MI_MAIN_CALL
**
**     This macro specifies the MI_Main() calling convention. For example:
**
**         MI_EXPORT MI_Module* MI_MAIN_CALL MI_Main(MI_Server* server)
**         {
**             ...
**         }
**
**     The macro appears directly before the function name.
**
**==============================================================================
*/

# define MI_MAIN_CALL __cdecl

/*
**==============================================================================
**
** MI_CALL
**
**     This macros specifies the calling convention for all functions other
**     than MI_Main(). For example:
**
**         MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PostResult(...);
**
**     The macro appears directly before the function name.
**
**==============================================================================
*/
# define MI_CALL __stdcall

//
// TODO: Need to change the _Managed macro to a standard well known one.
//
#ifdef _MANAGED_PURE
# define MI_INLINE_CALL
#else
# define MI_INLINE_CALL MI_CALL
#endif

/*
**==============================================================================
**
** MI_INLINE
**
**     This macro provides a platform-independent method for specifying that
**     a function is in-line. For compilers that do not support in-lining, define
**     MI_INLINE as 'static' instead.
**
**==============================================================================
*/

# define MI_INLINE static __inline

/*
**==============================================================================
**
** MI_OFFSETOF
**
**     This macro obtains the byte-offset of a field within a structure. It is
**     used in schema.c to obtain the offsets of generated structure fields.
**
**==============================================================================
*/

#define MI_OFFSETOF(STRUCT,FIELD) (((ptrdiff_t)&(((STRUCT*)1)->FIELD))-1)

/*
**==============================================================================
**
** MI_EXTERN_C
**
**     This macro forces a function to use the C function naming convention
**     (rather than C++ mangled naming convention).
**
**==============================================================================
*/

#ifdef __cplusplus
#   define  MI_EXTERN_C  extern "C"
#else
#   define  MI_EXTERN_C  extern
#endif

/*
**==============================================================================
**
** MI_COUNT()
**
**     This macro obtains the element count of an array. For example:
**
**         const char* DATA[] = { "Red", "Green", "Blue" };
**         size_t COUNT = MI_COUNT(DATA);
**
**     'COUNT' in this example is 3 (the number of array elements). This
**     macro is used by schema.c to obtain array lengths.
**
**==============================================================================
*/

#define MI_COUNT(X) (sizeof(X)/sizeof(X[0]))

/*
**==============================================================================
**
** MI_T()
**
**     This macro conditionally places the 'L' character in front of a string
**     literal (when using wide-character strings).
**
**==============================================================================
*/

#if (MI_CHAR_TYPE == 1)
# define MI_T(STR) STR
#else
# define MI_T(STR) L##STR
#endif

/*
**==============================================================================
**
** MI_LL()
** MI_ULL()
**
**     Macros for adding endings for sint64 and uint64 literals (used within
**     schema.c).
**
**==============================================================================
*/

# define MI_LL(X) X##i64
# define MI_ULL(X) X##ui64

/*
**==============================================================================
**
** Forward structure typedef declarations.
**
**==============================================================================
*/

typedef struct _MI_Server MI_Server;
typedef struct _MI_Context MI_Context;
typedef struct _MI_ClassDecl MI_ClassDecl;
typedef struct _MI_Instance MI_Instance;
typedef struct _MI_Filter MI_Filter;
typedef struct _MI_PropertySet MI_PropertySet;
typedef struct _MI_Qualifier MI_Qualifier;
typedef struct _MI_Session MI_Session;
typedef struct _MI_ServerFT MI_ServerFT;
typedef struct _MI_ProviderFT MI_ProviderFT;
typedef struct _MI_PropertySetFT MI_PropertySetFT;
typedef struct _MI_InstanceFT MI_InstanceFT;
typedef struct _MI_ContextFT MI_ContextFT;
typedef struct _MI_FilterFT MI_FilterFT;
typedef struct _MI_Class MI_Class;
typedef struct _MI_InstanceExFT MI_InstanceExFT;

/*
**==============================================================================
**
** MI_Result
**
**     This enumeration defines function return codes. These codes are
**     specified in [1].
**
**     [1] See DSP0004 (DMTF document number).
**
**==============================================================================
*/

typedef _Return_type_success_(return == MI_RESULT_OK) enum _MI_Result
{
    /* The operation was successful */
    MI_RESULT_OK = 0,

    /* A general error occurred, not covered by a more specific error code. */
    MI_RESULT_FAILED = 1,

    /* Access to a CIM resource is not available to the client. */
    MI_RESULT_ACCESS_DENIED = 2,

    /* The target namespace does not exist. */
    MI_RESULT_INVALID_NAMESPACE = 3,

    /* One or more parameter values passed to the method are not valid. */
    MI_RESULT_INVALID_PARAMETER  = 4,

    /* The specified class does not exist. */
    MI_RESULT_INVALID_CLASS = 5,

    /* The requested object cannot be found. */
    MI_RESULT_NOT_FOUND = 6,

    /* The requested operation is not supported. */
    MI_RESULT_NOT_SUPPORTED = 7,

    /* The operation cannot be invoked because the class has subclasses. */
    MI_RESULT_CLASS_HAS_CHILDREN = 8,

    /* The operation cannot be invoked because the class has instances. */
    MI_RESULT_CLASS_HAS_INSTANCES = 9,

    /* The operation cannot be invoked because the superclass does not exist. */
    MI_RESULT_INVALID_SUPERCLASS = 10,

    /* The operation cannot be invoked because an object already exists. */
    MI_RESULT_ALREADY_EXISTS = 11,

    /* The specified property does not exist. */
    MI_RESULT_NO_SUCH_PROPERTY = 12,

    /* The value supplied is not compatible with the type. */
    MI_RESULT_TYPE_MISMATCH = 13,

    /* The query language is not recognized or supported. */
    MI_RESULT_QUERY_LANGUAGE_NOT_SUPPORTED = 14,

    /* The query is not valid for the specified query language. */
    MI_RESULT_INVALID_QUERY = 15,

    /* The extrinsic method cannot be invoked. */
    MI_RESULT_METHOD_NOT_AVAILABLE = 16,

    /* The specified extrinsic method does not exist. */
    MI_RESULT_METHOD_NOT_FOUND = 17,

    /* The specified namespace is not empty. */
    MI_RESULT_NAMESPACE_NOT_EMPTY = 20,

    /* The enumeration identified by the specified context is invalid. */
    MI_RESULT_INVALID_ENUMERATION_CONTEXT = 21,

    /* The specified operation timeout is not supported by the CIM Server. */
    MI_RESULT_INVALID_OPERATION_TIMEOUT = 22,

    /* The Pull operation has been abandoned. */
    MI_RESULT_PULL_HAS_BEEN_ABANDONED = 23,

    /* The attempt to abandon a concurrent Pull operation failed. */
    MI_RESULT_PULL_CANNOT_BE_ABANDONED = 24,

    /* Using a filter in the enumeration is not supported by the CIM server. */
    MI_RESULT_FILTERED_ENUMERATION_NOT_SUPPORTED = 25,

    /* The CIM server does not support continuation on error. */
    MI_RESULT_CONTINUATION_ON_ERROR_NOT_SUPPORTED = 26,

    /* The operation failed because server limits were exceeded. */
    MI_RESULT_SERVER_LIMITS_EXCEEDED = 27,

    /* The CIM server is shutting down and cannot process the operation. */
    MI_RESULT_SERVER_IS_SHUTTING_DOWN = 28
}
MI_Result;

/*
**==============================================================================
**
** MI_ErrorCategory
**
**     This enumeration defines error categories for the CIM extensions.
**
**==============================================================================
*/

typedef enum _MI_ErrorCategory
{
    MI_ERRORCATEGORY_NOT_SPECIFIED          =  0,
    MI_ERRORCATEGORY_OPEN_ERROR             =  1,
    MI_ERRORCATEGORY_CLOS_EERROR            =  2,
    MI_ERRORCATEGORY_DEVICE_ERROR           =  3,
    MI_ERRORCATEGORY_DEADLOCK_DETECTED      =  4,
    MI_ERRORCATEGORY_INVALID_ARGUMENT       =  5,
    MI_ERRORCATEGORY_INVALID_DATA           =  6,
    MI_ERRORCATEGORY_INVALID_OPERATION      =  7,
    MI_ERRORCATEGORY_INVALID_RESULT         =  8,
    MI_ERRORCATEGORY_INVALID_TYPE           =  9,
    MI_ERRORCATEGORY_METADATA_ERROR         = 10,
    MI_ERRORCATEGORY_NOT_IMPLEMENTED        = 11,
    MI_ERRORCATEGORY_NOT_INSTALLED          = 12,
    MI_ERRORCATEGORY_OBJECT_NOT_FOUND       = 13,
    MI_ERRORCATEGORY_OPERATION_STOPPED      = 14,
    MI_ERRORCATEGORY_OPERATION_TIMEOUT      = 15,
    MI_ERRORCATEGORY_SYNTAX_ERROR           = 16,
    MI_ERRORCATEGORY_PARSER_ERROR           = 17,
    MI_ERRORCATEGORY_ACCESS_DENIED          = 18,
    MI_ERRORCATEGORY_RESOURCE_BUSY          = 19,
    MI_ERRORCATEGORY_RESOURCE_EXISTS        = 20,
    MI_ERRORCATEGORY_RESOURCE_UNAVAILABLE   = 21,
    MI_ERRORCATEGORY_READ_ERROR             = 22,
    MI_ERRORCATEGORY_WRITE_ERROR            = 23,
    MI_ERRORCATEGORY_FROM_STDERR            = 24,
    MI_ERRORCATEGORY_SECURITY_ERROR         = 25,
    MI_ERRORCATEGORY_PROTOCOL_ERROR         = 26,
    MI_ERRORCATEGORY_CONNECTION_ERROR       = 27,
    MI_ERRORCATEGORY_AUTHENTICATION_ERROR   = 28,
    MI_ERRORCATEGORY_LIMITS_EXCEEDED        = 29,
    MI_ERRORCATEGORY_QUOTA_EXCEEDED         = 30,
    MI_ERRORCATEGORY_NOT_ENABLED            = 31
}
MI_ErrorCategory;

/*
**==============================================================================
**
** MI_PromptType
**
**     This enumeration defines prompt types for the CIM extensions.
**
**==============================================================================
*/

typedef enum _MI_PromptType
{
    MI_PROMPTTYPE_NORMAL,
    MI_PROMPTTYPE_CRITICAL
}
MI_PromptType;

/*
**==============================================================================
**
** MI_CallbackMode
**
**     This enumeration defines callback mode  for the CIM extensions for WriteError and PromptUser APIs.
**
**==============================================================================
*/

typedef enum _MI_CallbackMode
{
    MI_CALLBACKMODE_REPORT,
    MI_CALLBACKMODE_INQUIRE,
    MI_CALLBACKMODE_IGNORE
}
MI_CallbackMode;

/*
**==============================================================================
**
** MI_ProviderArchitecture
**
**     This enumeration defines the provider to be used on the server.
**
**==============================================================================
*/

typedef enum _MI_ProviderArchitecture
{
    MI_PROVIDER_ARCHITECTURE_32BIT,
    MI_PROVIDER_ARCHITECTURE_64BIT,
}
MI_ProviderArchitecture;


/*
**==============================================================================
**
** Bit flags
**
**==============================================================================
*/

/* CIM meta types (or qualifier scopes) */
#define MI_FLAG_CLASS           (1 << 0)
#define MI_FLAG_METHOD          (1 << 1)
#define MI_FLAG_PROPERTY        (1 << 2)
#define MI_FLAG_PARAMETER       (1 << 3)
#define MI_FLAG_ASSOCIATION     (1 << 4)
#define MI_FLAG_INDICATION      (1 << 5)
#define MI_FLAG_REFERENCE       (1 << 6)
#define MI_FLAG_ANY             (1|2|4|8|16|32|64)

/* Qualifier flavors */
#define MI_FLAG_ENABLEOVERRIDE  (1 << 7)
#define MI_FLAG_DISABLEOVERRIDE (1 << 8)
#define MI_FLAG_RESTRICTED      (1 << 9)
#define MI_FLAG_TOSUBCLASS      (1 << 10)
#define MI_FLAG_TRANSLATABLE    (1 << 11)

/* Select boolean qualifier */
#define MI_FLAG_KEY             (1 << 12)
#define MI_FLAG_IN              (1 << 13)
#define MI_FLAG_OUT             (1 << 14)
#define MI_FLAG_REQUIRED        (1 << 15)
#define MI_FLAG_STATIC          (1 << 16)
#define MI_FLAG_ABSTRACT        (1 << 17)
#define MI_FLAG_TERMINAL        (1 << 18)
#define MI_FLAG_EXPENSIVE       (1 << 19)
#define MI_FLAG_STREAM          (1 << 20)
#define MI_FLAG_READONLY        (1 << 21)

/* Special flags */
#define MI_FLAG_EXTENDED        (1 << 12)
#define MI_FLAG_NOT_MODIFIED    (1 << 25) // indicates that the property is not modified
#define MI_FLAG_VERSION         (1<<26|1<<27|1<<28)
#define MI_FLAG_NULL            (1 << 29)
#define MI_FLAG_BORROW          (1 << 30)
#define MI_FLAG_ADOPT	        ((MI_Uint32)(1 << 31))

/*
**==============================================================================
**
** enum MI_Type
**
**     This enumeration defines type tags for the CIM data types [1]. These
**     tags specify the data type of qualifiers, properties, references,
**     parameters, and method return values. Tags ending in 'A' signify
**     arrays. All tags are within the range of 0 to 31, allowing them to be
**     stored in a single byte. The 0x10 bit of array tags is non-zero.
**
**     [1] See DSP0004 (DMTF document number).
**
**==============================================================================
*/

typedef enum _MI_Type
{
    MI_BOOLEAN = 0,
    MI_UINT8 = 1,
    MI_SINT8 = 2,
    MI_UINT16 = 3,
    MI_SINT16 = 4,
    MI_UINT32 = 5,
    MI_SINT32 = 6,
    MI_UINT64 = 7,
    MI_SINT64 = 8,
    MI_REAL32 = 9,
    MI_REAL64 = 10,
    MI_CHAR16 = 11,
    MI_DATETIME = 12,
    MI_STRING = 13,
    MI_REFERENCE = 14,
    MI_INSTANCE = 15,
    MI_BOOLEANA = 16,
    MI_UINT8A = 17,
    MI_SINT8A = 18,
    MI_UINT16A = 19,
    MI_SINT16A = 20,
    MI_UINT32A = 21,
    MI_SINT32A = 22,
    MI_UINT64A = 23,
    MI_SINT64A = 24,
    MI_REAL32A = 25,
    MI_REAL64A = 26,
    MI_CHAR16A = 27,
    MI_DATETIMEA = 28,
    MI_STRINGA = 29,
    MI_REFERENCEA = 30,
    MI_INSTANCEA = 31,

    /* MI_ARRAY is not an actual type, rather this is the bit that signifies
     * the type is an array */
    MI_ARRAY = 16
}
MI_Type;

/*
**==============================================================================
**
** MI_Uint8
** MI_Sint8
** MI_Uint16
** MI_Sint16
** MI_Uint32
** MI_Sint32
** MI_Uint64
** MI_Sint64
** MI_Real32
** MI_Real64
** MI_Char16
** MI_Char
**
**     The following represent CIM data types.
**
**==============================================================================
*/

typedef unsigned char MI_Boolean;
typedef unsigned char MI_Uint8;
typedef signed char MI_Sint8;
typedef unsigned short MI_Uint16;
typedef signed short MI_Sint16;
typedef unsigned int MI_Uint32;
typedef signed int MI_Sint32;

typedef unsigned __int64 MI_Uint64;
typedef signed __int64 MI_Sint64;

typedef float MI_Real32;
typedef double MI_Real64;
typedef unsigned short MI_Char16;

#if (MI_CHAR_TYPE == 1)
typedef char MI_Char;
#else
typedef wchar_t MI_Char;
#endif

typedef _Null_terminated_ MI_Char* MI_StringPtr;
typedef _Null_terminated_ _Const_ const MI_Char* MI_ConstStringPtr;

#define MI_TRUE ((MI_Boolean)1)
#define MI_FALSE ((MI_Boolean)0)

/*
**==============================================================================
**
** MI_Timestamp
**
**     Represents a timestamp as described in the CIM Infrastructure
**     specification
**
**     [1] MI_ee DSP0004 (http://www.dmtf.org/standards/published_documents)
**
**==============================================================================
*/

typedef struct _MI_Timestamp
{
    /* YYYYMMDDHHMMSS.MMMMMMSUTC */
    MI_Uint32 year;
    MI_Uint32 month;
    MI_Uint32 day;
    MI_Uint32 hour;
    MI_Uint32 minute;
    MI_Uint32 second;
    MI_Uint32 microseconds;
    MI_Sint32 utc;
}
MI_Timestamp;

/*
**==============================================================================
**
** struct MI_Interval
**
**     Represents an interval as described in the CIM Infrastructure
**     specification. This structure is padded to have the same length
**     as a MI_Timestamp structure.
**
**     [1] MI_ee DSP0004 (http://www.dmtf.org/standards/published_documents)
**
**==============================================================================
*/

typedef struct _MI_Interval
{
    /* DDDDDDDDHHMMSS.MMMMMM:000 */
    MI_Uint32 days;
    MI_Uint32 hours;
    MI_Uint32 minutes;
    MI_Uint32 seconds;
    MI_Uint32 microseconds;
    MI_Uint32 __padding1;
    MI_Uint32 __padding2;
    MI_Uint32 __padding3;
}
MI_Interval;

/*
**==============================================================================
**
** struct MI_Datetime
**
**     Represents a CIM datetime type as described in the CIM Infrastructure
**     specification. It contains a union of MI_Timestamp and MI_Interval.
**
**==============================================================================
*/

typedef struct _MI_Datetime
{
    MI_Uint32 isTimestamp;
    union
    {
        MI_Timestamp timestamp;
        MI_Interval interval;
    }
    u;
}
MI_Datetime;

/*
**==============================================================================
**
** struct MI_<TYPE>A
**
**     These structure represent arrays of the types introduced above.
**
**==============================================================================
*/

typedef struct _MI_BooleanA
{
    MI_Boolean* data;
    MI_Uint32 size;
}
MI_BooleanA;

typedef struct _MI_Uint8A
{
    MI_Uint8* data;
    MI_Uint32 size;
}
MI_Uint8A;

typedef struct _MI_Sint8A
{
    MI_Sint8* data;
    MI_Uint32 size;
}
MI_Sint8A;

typedef struct _MI_Uint16A
{
    MI_Uint16* data;
    MI_Uint32 size;
}
MI_Uint16A;

typedef struct _MI_Sint16A
{
    MI_Sint16* data;
    MI_Uint32 size;
}
MI_Sint16A;

typedef struct _MI_Uint32A
{
    MI_Uint32* data;
    MI_Uint32 size;
}
MI_Uint32A;

typedef struct _MI_Sint32A
{
    MI_Sint32* data;
    MI_Uint32 size;
}
MI_Sint32A;

typedef struct _MI_Uint64A
{
    MI_Uint64* data;
    MI_Uint32 size;
}
MI_Uint64A;

typedef struct _MI_Sint64A
{
    MI_Sint64* data;
    MI_Uint32 size;
}
MI_Sint64A;

typedef struct _MI_Real32A
{
    MI_Real32* data;
    MI_Uint32 size;
}
MI_Real32A;

typedef struct _MI_Real64A
{
    MI_Real64* data;
    MI_Uint32 size;
}
MI_Real64A;

typedef struct _MI_Char16A
{
    MI_Char16* data;
    MI_Uint32 size;
}
MI_Char16A;

typedef struct _MI_DatetimeA
{
    MI_Datetime* data;
    MI_Uint32 size;
}
MI_DatetimeA;

typedef struct _MI_StringA
{
    MI_Char** data;
    MI_Uint32 size;
}
MI_StringA;

typedef struct _MI_ReferenceA
{
    struct _MI_Instance** data;
    MI_Uint32 size;
}
MI_ReferenceA;

typedef struct _MI_InstanceA
{
    MI_Instance** data;
    MI_Uint32 size;
}
MI_InstanceA;

typedef struct _MI_Array
{
    void* data;
    MI_Uint32 size;
}
MI_Array;

/*
**==============================================================================
**
** struct MI_Const<TYPE>A
**
**     These structure represent arrays of the types introduced above.
**
**==============================================================================
*/

typedef struct _MI_ConstBooleanA
{
    MI_CONST MI_Boolean* data;
    MI_Uint32 size;
}
MI_ConstBooleanA;

typedef struct _MI_ConstUint8A
{
    MI_CONST MI_Uint8* data;
    MI_Uint32 size;
}
MI_ConstUint8A;

typedef struct _MI_ConstSint8A
{
    MI_CONST MI_Sint8* data;
    MI_Uint32 size;
}
MI_ConstSint8A;

typedef struct _MI_ConstUint16A
{
    MI_CONST MI_Uint16* data;
    MI_Uint32 size;
}
MI_ConstUint16A;

typedef struct _MI_ConstSint16A
{
    MI_CONST MI_Sint16* data;
    MI_Uint32 size;
}
MI_ConstSint16A;

typedef struct _MI_ConstUint32A
{
    MI_CONST MI_Uint32* data;
    MI_Uint32 size;
}
MI_ConstUint32A;

typedef struct _MI_ConstSint32A
{
    MI_CONST MI_Sint32* data;
    MI_Uint32 size;
}
MI_ConstSint32A;

typedef struct _MI_ConstUint64A
{
    MI_CONST MI_Uint64* data;
    MI_Uint32 size;
}
MI_ConstUint64A;

typedef struct _MI_ConstSint64A
{
    MI_CONST MI_Sint64* data;
    MI_Uint32 size;
}
MI_ConstSint64A;

typedef struct _MI_ConstReal32A
{
    MI_CONST MI_Real32* data;
    MI_Uint32 size;
}
MI_ConstReal32A;

typedef struct _MI_ConstReal64A
{
    MI_CONST MI_Real64* data;
    MI_Uint32 size;
}
MI_ConstReal64A;

typedef struct _MI_ConstChar16A
{
    MI_CONST MI_Char16* data;
    MI_Uint32 size;
}
MI_ConstChar16A;

typedef struct _MI_ConstDatetimeA
{
    MI_CONST MI_Datetime* data;
    MI_Uint32 size;
}
MI_ConstDatetimeA;

typedef struct _MI_ConstStringA
{
    MI_CONST MI_Char* MI_CONST* data;
    MI_Uint32 size;
}
MI_ConstStringA;

typedef struct _MI_ConstReferenceA
{
    MI_CONST MI_Instance* MI_CONST* data;
    MI_Uint32 size;
}
MI_ConstReferenceA;

typedef struct _MI_ConstInstanceA
{
    MI_CONST MI_Instance* MI_CONST* data;
    MI_Uint32 size;
}
MI_ConstInstanceA;

/*
**==============================================================================
**
** union MI_Value
**
**     This structure defines a union of all CIM data types.
**
**==============================================================================
*/

typedef union _MI_Value
{
    MI_Boolean boolean;
    MI_Uint8 uint8;
    MI_Sint8 sint8;
    MI_Uint16 uint16;
    MI_Sint16 sint16;
    MI_Uint32 uint32;
    MI_Sint32 sint32;
    MI_Uint64 uint64;
    MI_Sint64 sint64;
    MI_Real32 real32;
    MI_Real64 real64;
    MI_Char16 char16;
    MI_Datetime datetime;
    MI_Char* string;
    MI_Instance* instance;
    MI_Instance* reference;
    MI_BooleanA booleana;
    MI_Uint8A uint8a;
    MI_Sint8A sint8a;
    MI_Uint16A uint16a;
    MI_Sint16A sint16a;
    MI_Uint32A uint32a;
    MI_Sint32A sint32a;
    MI_Uint64A uint64a;
    MI_Sint64A sint64a;
    MI_Real32A real32a;
    MI_Real64A real64a;
    MI_Char16A char16a;
    MI_DatetimeA datetimea;
    MI_StringA stringa;
    MI_ReferenceA referencea;
    MI_InstanceA instancea;
    MI_Array array;
}
MI_Value;

/*
**==============================================================================
**
** struct MI_<TYPE>Field
**
**     These structures represent property or parameter fields within generated
**     structures. Each structure definition defines two fields.
**
**         value - a field of the given type.
**         exists - a flag indicating whether the field is non-null.
**
**     The flags field is used for internal use.
**
**
**==============================================================================
*/

typedef struct _MI_BooleanField
{
    MI_Boolean value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_BooleanField;

typedef struct _MI_Sint8Field
{
    MI_Sint8 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint8Field;

typedef struct _MI_Uint8Field
{
    MI_Uint8 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint8Field;

typedef struct _MI_Sint16Field
{
    MI_Sint16 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint16Field;

typedef struct _MI_Uint16Field
{
    MI_Uint16 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint16Field;

typedef struct _MI_Sint32Field
{
    MI_Sint32 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint32Field;

typedef struct _MI_Uint32Field
{
    MI_Uint32 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint32Field;

typedef struct _MI_Sint64Field
{
    MI_Sint64 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint64Field;

typedef struct _MI_Uint64Field
{
    MI_Uint64 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint64Field;

typedef struct _MI_Real32Field
{
    MI_Real32 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Real32Field;

typedef struct _MI_Real64Field
{
    MI_Real64 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Real64Field;

typedef struct _MI_Char16Field
{
    MI_Char16 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Char16Field;

typedef struct _MI_DatetimeField
{
    MI_Datetime value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_DatetimeField;

typedef struct _MI_StringField
{
    MI_Char* value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_StringField;

typedef struct _MI_ReferenceField
{
    MI_Instance* value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ReferenceField;

typedef struct _MI_InstanceField
{
    MI_Instance* value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_InstanceField;

typedef struct _MI_BooleanAField
{
    MI_BooleanA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_BooleanAField;

typedef struct _MI_Uint8AField
{
    MI_Uint8A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint8AField;

typedef struct _MI_Sint8AField
{
    MI_Sint8A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint8AField;

typedef struct _MI_Uint16AField
{
    MI_Uint16A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint16AField;

typedef struct _MI_Sint16AField
{
    MI_Sint16A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint16AField;

typedef struct _MI_Uint32AField
{
    MI_Uint32A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint32AField;

typedef struct _MI_Sint32AField
{
    MI_Sint32A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint32AField;

typedef struct _MI_Uint64AField
{
    MI_Uint64A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Uint64AField;

typedef struct _MI_Sint64AField
{
    MI_Sint64A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Sint64AField;

typedef struct _MI_Real32AField
{
    MI_Real32A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Real32AField;

typedef struct _MI_Real64AField
{
    MI_Real64A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Real64AField;

typedef struct _MI_Char16AField
{
    MI_Char16A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_Char16AField;

typedef struct _MI_DatetimeAField
{
    MI_DatetimeA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_DatetimeAField;

typedef struct _MI_StringAField
{
    MI_StringA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_StringAField;

typedef struct _MI_ReferenceAField
{
    MI_ReferenceA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ReferenceAField;

typedef struct _MI_InstanceAField
{
    MI_InstanceA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_InstanceAField;

typedef struct _MI_ArrayField
{
    MI_Array value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ArrayField;


/*
**==============================================================================
**
** struct MI_Const<TYPE>Field
**
**     These structures represent property or parameter fields within generated
**     structures. Each structure definition defines two fields.
**
**         value - a field of the given type.
**         exists - a flag indicating whether the field is non-null.
**
**     The flags field is used for internal use.
**
**==============================================================================
*/

typedef struct _MI_ConstBooleanField
{
    MI_Boolean value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstBooleanField;

typedef struct _MI_ConstSint8Field
{
    MI_Sint8 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint8Field;

typedef struct _MI_ConstUint8Field
{
    MI_Uint8 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint8Field;

typedef struct _MI_ConstSint16Field
{
    MI_Sint16 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint16Field;

typedef struct _MI_ConstUint16Field
{
    MI_Uint16 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint16Field;

typedef struct _MI_ConstSint32Field
{
    MI_Sint32 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint32Field;

typedef struct _MI_ConstUint32Field
{
    MI_Uint32 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint32Field;

typedef struct _MI_ConstSint64Field
{
    MI_Sint64 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint64Field;

typedef struct _MI_ConstUint64Field
{
    MI_Uint64 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint64Field;

typedef struct _MI_ConstReal32Field
{
    MI_Real32 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstReal32Field;

typedef struct _MI_ConstReal64Field
{
    MI_Real64 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstReal64Field;

typedef struct _MI_ConstChar16Field
{
    MI_Char16 value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstChar16Field;

typedef struct _MI_ConstDatetimeField
{
    MI_Datetime value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstDatetimeField;

typedef struct _MI_ConstStringField
{
    MI_CONST MI_Char* value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstStringField;

typedef struct _MI_ConstReferenceField
{
    MI_CONST MI_Instance* value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstReferenceField;

typedef struct _MI_ConstInstanceField
{
    MI_CONST MI_Instance* value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstInstanceField;

typedef struct _MI_ConstBooleanAField
{
    MI_ConstBooleanA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstBooleanAField;

typedef struct _MI_ConstUint8AField
{
    MI_ConstUint8A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint8AField;

typedef struct _MI_ConstSint8AField
{
    MI_ConstSint8A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint8AField;

typedef struct _MI_ConstUint16AField
{
    MI_ConstUint16A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint16AField;

typedef struct _MI_ConstSint16AField
{
    MI_ConstSint16A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint16AField;

typedef struct _MI_ConstUint32AField
{
    MI_ConstUint32A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint32AField;

typedef struct _MI_ConstSint32AField
{
    MI_ConstSint32A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint32AField;

typedef struct _MI_ConstUint64AField
{
    MI_ConstUint64A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstUint64AField;

typedef struct _MI_ConstSint64AField
{
    MI_ConstSint64A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstSint64AField;

typedef struct _MI_ConstReal32AField
{
    MI_ConstReal32A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstReal32AField;

typedef struct _MI_ConstReal64AField
{
    MI_ConstReal64A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstReal64AField;

typedef struct _MI_ConstChar16AField
{
    MI_ConstChar16A value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstChar16AField;

typedef struct _MI_ConstDatetimeAField
{
    MI_ConstDatetimeA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstDatetimeAField;

typedef struct _MI_ConstStringAField
{
    MI_ConstStringA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstStringAField;

typedef struct _MI_ConstReferenceAField
{
    MI_ConstReferenceA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstReferenceAField;

typedef struct _MI_ConstInstanceAField
{
    MI_ConstInstanceA value;
    MI_Boolean exists;
    MI_Uint8 flags;
}
MI_ConstInstanceAField;

/*
**==============================================================================
**
** MI_Server
**
**==============================================================================
*/

struct _MI_ServerFT
{
    MI_Result (MI_CALL *GetVersion)(
        MI_Uint32* version);

    MI_Result (MI_CALL *GetSystemName)(
        const MI_Char** systemName);
};

/**
 * This structure defines the global server object. It defines the interface
 * for communicating with the server. It also defines default function tables
 * for all other types (Context, Instance, PropertySet, and Filter).
 *
 */
struct _MI_Server
{
    const MI_ServerFT* serverFT;
    const MI_ContextFT* contextFT;
    const MI_InstanceFT* instanceFT;
    const MI_PropertySetFT* propertySetFT;
    const MI_FilterFT*  filterFT;
};

/**
 * Obtains the value of the MI_VERSION macro used when compiling the server.
 *
 * param: version contains the version number upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_Result MI_CALL MI_Server_GetVersion(MI_Uint32* version);

/**
 * Obtains the 'system name' for this server. The system name is used in
 * several standard CIM key properties (e.g., CIM_Fan.SystemName). The name
 * is only known by the server. The provider should never attempt to determine
 * the system name on its own. The system name is typically the hostname
 * for the system but the server may add additional qualification.
 *
 * param: systemName points to the system name upon return (remains in scope
 *        for the lifetime of the process.
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_Result MI_CALL MI_Server_GetSystemName(const MI_Char** systemName);

/*
**==============================================================================
**
** MI_Filter
**
**==============================================================================
*/

/** The MI_FilterFT function table */
struct _MI_FilterFT
{
    MI_Result (MI_CALL *Evaluate)(
        _In_ const MI_Filter* self,
        _In_ const MI_Instance* instance,
        _Out_ MI_Boolean* result);

    MI_Result (MI_CALL *GetExpression)(
        _In_ const MI_Filter* self,
        _Outptr_result_maybenull_z_ const MI_Char** queryLang,
        _Outptr_result_maybenull_z_ const MI_Char** queryExpr);
};

struct _MI_Filter
{
    /* Function table */
    const MI_FilterFT* ft;

    /* Reserved for internal use */
    ptrdiff_t reserved[3];
};

/**
 *
 * Provider calls this function to evaluate an instance against given filter.
 *
 * param: self pointer to the filter.
 * param: instance to evaluate.
 * param: on completion, result indicates whether the instance matched the filter.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Filter_Evaluate(
        _In_ const MI_Filter* self,
        _In_ const MI_Instance* instance,
        _Out_ MI_Boolean* result)
{
    if (self && self->ft)
    {
        return self->ft->Evaluate(self, instance, result);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * This function returns filter language and expression.
 *
 * param: self pointer to the filter.
 * param: queryExpr the query string upon return.
 * param: queryLang the query language upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Filter_GetExpression(
        _In_ const MI_Filter* self,
        _Outptr_result_maybenull_z_ const MI_Char** queryLang,
        _Outptr_result_maybenull_z_ const MI_Char** queryExpr)
{
    if (self && self->ft)
    {
        return self->ft->GetExpression(self, queryLang, queryExpr);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**==============================================================================
**
** The MI_PropertySet Module
**
**==============================================================================
*/


/** The MI_PropertySet function table. */
struct _MI_PropertySetFT
{
    MI_Result (MI_CALL *GetElementCount)(
        _In_ const MI_PropertySet* self,
        _Out_ MI_Uint32* count);

    MI_Result (MI_CALL *ContainsElement)(
        _In_ const MI_PropertySet* self,
        _In_z_ const MI_Char* name,
        _Out_ MI_Boolean* flag);

    MI_Result (MI_CALL *AddElement)(
        _Inout_ MI_PropertySet* self,
        _In_z_ const MI_Char* name);

    MI_Result (MI_CALL *GetElementAt)(
        _In_ const MI_PropertySet* self,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char** name);

    MI_Result (MI_CALL *Clear)(
        _Inout_ MI_PropertySet* self);

    MI_Result (MI_CALL *Destruct)(
        _Inout_ MI_PropertySet* self);

    MI_Result (MI_CALL *Delete)(
        _Inout_ MI_PropertySet* self);

    MI_Result (MI_CALL *Clone)(
        _In_ const MI_PropertySet* self,
        _Outptr_ MI_PropertySet** newPropertySet);
};

/**
 *  This type implements a set of property names. It supports building of
 *  property sets and interrogation of property sets. In general, clients
 *  build property sets and providers interrogate them.
 *
 */
struct _MI_PropertySet
{
    /* Function table */
    const MI_PropertySetFT* ft;

    /* Reserved for internal use */
    ptrdiff_t reserved[3];
};

/**
 * Gets the number of properties in the list.
 *
 * param: self the property list
 * param: count the number of properties upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_GetElementCount(
    _In_ const MI_PropertySet* self,
    _Out_ MI_Uint32* count)
{
    if (self && self->ft)
    {
        return self->ft->GetElementCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Determines whether the property list contains the given property.
 *
 * param: self the property list
 * param: name check whether this property is contained in list.
 * param: flag MI_TRUE upon return if property was found. MI_FALSE otherwise.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_ContainsElement(
    _In_ const MI_PropertySet* self,
    _In_z_ const MI_Char* name,
    _Out_ MI_Boolean* flag)
{
    if (self && self->ft)
    {
        return self->ft->ContainsElement(self, name, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Adds a name to the property list.
 *
 * param: self the property list
 * param: name add this name to the property list.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_ALREADY_EXISTS, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_AddElement(
    _Inout_ MI_PropertySet* self,
    _In_z_ const MI_Char* name)
{
    if (self && self->ft)
    {
        return self->ft->AddElement(self, name);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets the i-th name from the list.
 *
 * param: self the property list.
 * param: index get the name with this index.
 * param: name set this to point to the name (the lifetime of this string
 *        is tied to the property list).
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_GetElementAt(
    _In_ const MI_PropertySet* self,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char** name)
{
    if (self && self->ft)
    {
        return self->ft->GetElementAt(self, index, name);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Remove all names from the property list. Afterwards, the count is zero
 * This allows property lists to be reused (without having to be destructed
 * and reconstructed).
 *
 * param: self the property list
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_Clear(
    _Inout_ MI_PropertySet* self)
{
    if (self && self->ft)
    {
        return self->ft->Clear(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Destructs the property list (releasing memory resources). The property list
 * must have been constructed on the stack and not on the heap.
 *
 * param: self the property list
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_Destruct(
    _Inout_ MI_PropertySet* self)
{
    if (self && self->ft)
    {
        return self->ft->Destruct(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Deletes the property list (releasing memory resources). The property list
 * must have been constructed on the heap (not the stack).
 *
 * param: self the property list
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_Delete(
    _Inout_ MI_PropertySet* self)
{
    if (self && self->ft)
    {
        return self->ft->Delete(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/**
 * This function creates a copy of the given Property set on the heap. Upon
 * a successful return, new property set points to a newly created property set object.
 * The new property set should eventually be passed to MI_PropertySet_Delete().
 *
 * param: self pointer to the property set to be cloned.
 * param: newPropertySet a pointer to the new property set upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_PropertySet_Clone(
    _In_ const MI_PropertySet* self,
    _Outptr_ MI_PropertySet** newPropertySet)
{
    if (self && self->ft)
    {
        return self->ft->Clone(self, newPropertySet);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}



/*
**==============================================================================
**
** struct MI_ObjectDecl
**
**     A base type for MI_ClassDecl and MI_PropertyDecl, which allows functions
**     to be written that work on the common fields of these two types.
**
**==============================================================================
*/

typedef struct _MI_ObjectDecl /* extends MI_FeatureDecl */
{
    /* Fields inherited from MI_FeatureDecl */
    MI_Uint32 flags;
    MI_Uint32 code;
    MI_CONST MI_Char* name;
    MI_Qualifier MI_CONST* MI_CONST* qualifiers;
    MI_Uint32 numQualifiers;

    /* The properties or parameters of this object. Note that for methods
     * the type will be MI_ParameterDecl rather than MI_PropertyDecl.
     */
    struct _MI_PropertyDecl MI_CONST* MI_CONST* properties;
    MI_Uint32 numProperties;

    /* Size of structure described by MI_MethodDecl or MI_ClassDecl */
    MI_Uint32 size;
}
MI_ObjectDecl;

/*
**==============================================================================
**
** struct MI_ClassDecl
**
**     Represents a CIM class.
**
**     Flags:
**         MI_FLAG_CLASS
**         MI_FLAG_ASSOCIATION
**         MI_FLAG_INDICATION
**         MI_FLAG_ABSTRACT
**         MI_FLAG_TERMINAL
**
**==============================================================================
*/

struct _MI_ClassDecl /* extends MI_ObjectDecl */
{
    /* Fields inherited from MI_FeatureDecl */
    MI_Uint32 flags;
    MI_Uint32 code;
    MI_CONST MI_Char* name;
    struct _MI_Qualifier MI_CONST* MI_CONST* qualifiers;
    MI_Uint32 numQualifiers;

    /* Fields inherited from MI_ObjectDecl */
    struct _MI_PropertyDecl MI_CONST* MI_CONST* properties;
    MI_Uint32 numProperties;
    MI_Uint32 size;

    /* Name of superclass */
    MI_CONST MI_Char* superClass;

    /* Superclass declaration */
    MI_ClassDecl MI_CONST* superClassDecl;

    /* The methods of this class */
    struct _MI_MethodDecl MI_CONST* MI_CONST* methods;
    MI_Uint32 numMethods;

    /* Pointer to scema this class belongs to */
    struct _MI_SchemaDecl MI_CONST* schema;

    /* Provider functions */
    MI_CONST MI_ProviderFT* providerFT;

    /* Owning MI_Class object, if any.  NULL if static classDecl, -1 is from a dynamic instance */
    MI_Class *owningClass;
};

/*
**==============================================================================
**
** struct MI_FeatureDecl
**
**     This structure functions as a base type for these structures:
**         MI_PropertyDecl
**         MI_ParameterDecl
**         MI_MethodDecl
**
**==============================================================================
*/

typedef struct _MI_FeatureDecl
{
    /* Flags */
    MI_Uint32 flags;

    /* Hash code: (name[0] << 16) | (name[len-1] << 8) | len */
    MI_Uint32 code;

    /* Name of this feature */
    MI_CONST MI_Char* name;

    /* Qualifiers */
    MI_Qualifier MI_CONST* MI_CONST *  qualifiers;
    MI_Uint32 numQualifiers;
}
MI_FeatureDecl;

/*
**==============================================================================
**
** struct MI_ParameterDecl
**
**     Represents a CIM property (or reference)
**
**     Flags:
**         MI_FLAG_PROPERTY
**         MI_FLAG_KEY
**
**==============================================================================
*/

typedef struct _MI_ParameterDecl /* extends MI_FeatureDecl */
{
    /* Fields inherited from MI_FeatureDecl */
    MI_Uint32 flags;
    MI_Uint32 code;
    MI_CONST MI_Char* name;
    MI_Qualifier MI_CONST* MI_CONST* qualifiers;
    MI_Uint32 numQualifiers;

    /* Type of this field */
    MI_Uint32 type;

    /* Name of reference class */
    MI_CONST MI_Char* className;

    /* Array subscript */
    MI_Uint32 subscript;

    /* Offset of this field within the structure */
    MI_Uint32 offset;
}
MI_ParameterDecl;

/*
**==============================================================================
**
** struct MI_PropertyDecl
**
**     Represents a CIM property (or reference)
**
**     Flags:
**         MI_FLAG_PROPERTY
**         MI_FLAG_KEY
**
**==============================================================================
*/

typedef struct _MI_PropertyDecl /* extends MI_ParameterDecl */
{
    /* Fields inherited from MI_FeatureDecl */
    MI_Uint32 flags;
    MI_Uint32 code;
    MI_CONST MI_Char* name;
    MI_Qualifier MI_CONST* MI_CONST* qualifiers;
    MI_Uint32 numQualifiers;

    /* Fields inherited from MI_ParameterDecl */
    MI_Uint32 type;
    MI_CONST MI_Char* className;
    MI_Uint32 subscript;
    MI_Uint32 offset;

    /* Ancestor class that first defined a property with this name */
    MI_CONST MI_Char* origin;

    /* Ancestor class that last defined a property with this name */
    MI_CONST MI_Char* propagator;

    /* Value of this property */
    MI_CONST  void* value;
}
MI_PropertyDecl;

/*
**==============================================================================
**
** struct MI_MethodDecl
**
**     Represents a CIM method.
**
**     Flags:
**         MI_FLAG_METHOD
**         MI_FLAG_STATIC
**
**==============================================================================
*/

typedef void (MI_CALL *MI_MethodDecl_Invoke)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_z_ const MI_Char* methodName,
    _In_ const MI_Instance* instanceName,
    _In_ const MI_Instance* parameters);

typedef struct _MI_MethodDecl /* extends MI_ObjectDecl */
{
    /* Fields inherited from MI_FeatureDecl */
    MI_Uint32 flags;
    MI_Uint32 code;
    MI_CONST MI_Char* name;
    struct _MI_Qualifier MI_CONST* MI_CONST* qualifiers;
    MI_Uint32 numQualifiers;

    /* Fields inherited from MI_ObjectDecl */
    struct _MI_ParameterDecl MI_CONST* MI_CONST* parameters;
    MI_Uint32 numParameters;
    MI_Uint32 size;

    /* PostResult type of this method */
    MI_Uint32 returnType;

    /* Ancestor class that first defined a property with this name */
    MI_CONST MI_Char* origin;

    /* Ancestor class that last defined a property with this name */
    MI_CONST MI_Char* propagator;

    /* Pointer to scema this class belongs to */
    struct _MI_SchemaDecl MI_CONST* schema;

    /* Pointer to extrinsic method */
    MI_MethodDecl_Invoke function;
}
MI_MethodDecl;

/*
**==============================================================================
**
** struct MI_QualifierDecl
**
**     Represents a CIM qualifier declaration.
**
**==============================================================================
*/

typedef struct _MI_QualifierDecl
{
    /* Name of this qualifier */
    MI_CONST MI_Char* name;

    /* Type of this qualifier */
    MI_Uint32 type;

    /* Qualifier scope */
    MI_Uint32 scope;

    /* Qualifier flavor */
    MI_Uint32 flavor;

    /* Array subscript (for arrays only) */
    MI_Uint32 subscript;

    /* Pointer to value */
    MI_CONST void* value;
}
MI_QualifierDecl;

/*
**==============================================================================
**
** struct MI_Qualifier
**
**     Represents a CIM qualifier.
**
**==============================================================================
*/

struct _MI_Qualifier
{
    /* Qualifier name */
    MI_CONST MI_Char* name;

    /* Qualifier type */
    MI_Uint32 type;

    /* Qualifier flavor */
    MI_Uint32 flavor;

    /* Pointer to value */
    MI_CONST void* value;
};

/*
**==============================================================================
**
** struct MI_SchemaDecl
**
**     This structure represents the schema objects in a CIM schemas, which
**     include CIM classes and CIM qualifier declarations.
**
**==============================================================================
*/

typedef struct _MI_SchemaDecl
{
    /* Qualifier declarations */
    MI_QualifierDecl MI_CONST* MI_CONST* qualifierDecls;
    MI_Uint32 numQualifierDecls;

    /* Class declarations */
    MI_ClassDecl MI_CONST* MI_CONST* classDecls;
    MI_Uint32 numClassDecls;
}
MI_SchemaDecl;

/*
**==============================================================================
**
** The MI_ProviderFT Module
**
**==============================================================================
*/

/* The developer may optionally define this structure in module.c */
typedef struct _MI_Module_Self MI_Module_Self;

/**
 * The server invokes this function to initialize the provider, which
 * performs initialization activities. The provider may set the 'self'
 * parameter to refer to any provider state data (or null if no state data
 * is required). Whatever value the provider sets for 'self' is passed into
 * other calls to the provider.
 *
 * param: self the provider may set this to refer to any provider state data
 *        (or NULL if no state data is required).
 * param: selfModule the 'self' parameter obtained when loading the module.
 * param: context the current request context
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_Load)(
    _Outptr_ void** self,
    _In_opt_ MI_Module_Self* selfModule,
    _In_ MI_Context* context);

/**
 * The server invokes this function to release any resources held by the
 * provider. The provider should close any file handles and release any
 * memory associated with the execution of the provider.
 *
 * The implementation should pass MI_RESULT_OK or MI_RESULT_FAILED to
 * MI_Context_PostResult.
 *
 * param: self the provider state data.
 * param: context the request context.
 *
 * Result posting: nothing
 *
 * return: MI_RESULT_OK, MI_RESULT_DO_NOT_UNLOAD, MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_Unload)(
    _In_opt_ void* self,
    _In_ MI_Context* context);

/**
 *  The server invokes the GetInstance function to obtain a single CIM
 *  instance from the provider. The 'instanceName' property defines the
 *  name of the instance to be retrieved.
 *
 *  If the 'propertySet' parameter is not null, the elements of the set define
 *  zero or more property names. The returned instance shall not include
 *  elements for properties missing from this set. If the 'propertySet' input
 *  parameter is an empty set, no properties are included in the response. If
 *  the 'propertySet' input parameter is null, no properties shall be filtered.
 *
 *  If the provider returns MI_RESULT_NOT_SUPPORTED (via MI_Context_PostResult), the
 *  server attempts to satisfy the request by calling the provider's
 *  'EnumerateInstances' method. Do not rely on this behavior unless the
 *  number of instances is reasonably small.
 *
 *  If GetInstance is successful, the provider should pass the new instance
 *  to MI_Context_PostInstance(). If GetInstance is unsuccessful, the provider should
 *  return one of the results listed below (via MI_Context_PostResult).
 *
 *  param: self the provider state data
 *  param: context the request context
 *  param: nameSpace the namespace of the request.
 *  param: className the name of the class.
 *  param: instanceName name of the requested instance
 *  param: propertySet list of required properties or NULL for all.
 *
 *  Result posting: resulting instance.
 *
 *  return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_INVALID_CLASS
 *      MI_RESULT_NOT_FOUND
 *      MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_GetInstance)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_ const MI_Instance* instanceName,
    _In_opt_ const MI_PropertySet* propertySet);

/**
 *  The server calls EnumerateInstances to enumerate instances of a CIM class
 *  in the target namespace. Note that the enumeration is not polymoprhic; the
 *  implementaiton should provide instances of the exact class given by the
 *  'className' input parameter, and should not include instances of any
 *  derived classes.
 *
 *  The 'className' input parameter defines the exact class to be enumerated.
 *
 *  If the 'propertySet' parameter is not null, the elements of the set define
 *  zero or more property names. The returned instances shall not include
 *  elements for properties missing from this set. If the 'propertySet' input
 *  parameter is an empty set, no properties are included in the response. If
 *  the 'propertySet' input parameter is null, no properties shall be filtered.
 *
 *  If the 'keysOnly' input parameter is true, then the implementaiton should
 *  provide only key properties.
 *
 *  If not null, the 'filter' input parameter defines a query filter that all
 *  provided instances must match. If the MI_Module.flags field contains
 *  MI_MODULE_FLAG_FILTER_SUPPORT (set by the MI_Main() entry point), this
 *  filter may be non-null. Otherwise, the 'filter' input paramerter is null.
 *
 *  If EnumerateInstances is successful, the method returns zero or more
 *  instances.
 *
 *  param: self the provider state data.
 *  param: context the request context.
 *  param: nameSpace enumerate instances of this namespace.
 *  param: className enumerate instances of this class.
 *  param: propertySet list of required properties or NULL for all.
 *  param: keysOnly true if only key properties are required
 *  param: filter Used to filter instances.
 *
 *  Result posting: zero or more instances.
 *
 *  return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_INVALID_CLASS
 *      MI_RESULT_NOT_SUPPORTED
 *      MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_EnumerateInstances)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_opt_ const MI_PropertySet* propertySet,
    MI_Boolean keysOnly,
    _In_opt_ const MI_Filter* filter);

/**
 *  The server calls the CreateInstance function to create a single CIM
 *  instance in the target namespace.
 *
 *  The 'newInstance' input parameter defines the properties of the new
 *  instance. The null properties of this instance are ignored and are not
 *  part of the new instance.
 *
 *  The 'newInstance' input parameter may define some but not all of the key
 *  properties (leaving some keys null). If so, the implementation must
 *  allocate values for the undefined keys. This occurs with keys that the
 *  requestor cannot define, since their values are only known by the server.
 *  Typical examples include 'SystemName' and 'SystemCreationClassName'.
 *
 *  If CreateInstance is successful, the implementation should post the
 *  instance name of the new instance and then post MI_RESULT_OK.
 *
 *  If CreateInstance is successful, the implementation should post the
 *  result error code in the return section.
 *
 *  If an instances with the same keys already exists, the implementation
 *  should post MI_RESULT_ALREADY_EXISTS.
 *
 *  param: self the provider state data.
 *  param: context the request context.
 *  param: nameSpace enumerate instances of this namespace.
 *  param: className enumerate instances of this class.
 *  param: newInstance the instance that will be created.
 *
 *  Result posting: a single instance.
 *
 *  return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_NOT_SUPPORTED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_INVALID_CLASS
 *      MI_RESULT_ALREADY_EXISTS
 *      MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_CreateInstance)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_ const MI_Instance* newInstance);

/**
 *  The server calls the ModifyInstance function to modify an existing CIM
 *  instance in the target namespace. The instance must already exist.
 *
 *  The 'modifiedInstance' input parameter identifies the instance that shall
 *  be modified (through its key properties) and provides new property values
 *  for it.
 *
 *  The set of properties that are modified are determined as follows:
 *
 *  If the propertySet input parameter is not null, the elements of the set
 *  define zero or more property names. Only properties specified in this set
 *  are modified. Properties of the modifiedInstance that are missing from the
 *  set shall be ingored. If the set is empty, no properties are modified. If
 *  propertySet is null, the set of properties to be modified consists of those
 *  of modifiedInstance that are not null and whose values are different from
 *  the current values of the instance to be modified.
 *
 *  If propertySet contains invalid property names, the implementation shall
 *  reject the request. If a property cannot be modified because, it is a key,
 *  it is non-writable, or for any other reason, the implementation shall
 *  reject the request.
 *
 *  If ModifyInstance is successful, all properties to modified are updated
 *  in the specified instance.
 *
 *  If ModifyInstance is unsuccessful, no change is made to the specified
 *  instance and an error is returned.
 *
 *  param: self the provider state data
 *  param: context the request context
 *  param: nameSpace enumerate instances of this namespace.
 *  param: className enumerate instances of this class.
 *  param: modifiedInstance contains the new property values for the instance
 *  param: propertySet specifies which properties to modify or NULL for all.
 *
 *  return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_CLASS
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_NOT_SUPPORTED
 *      MI_RESULT_NOT_FOUND
 *      MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_ModifyInstance)(
    void* self,
    MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    const MI_Instance* modifiedInstance,
    const MI_PropertySet* propertySet);

/**
 *  The server calls the DeleteInstance function to delete a single CIM
 *  instance from the target namespace.
 *
 *  The instanceName input parameter defines the name (keys) of the instance
 *  to be deleted.
 *
 *  Deleting an instance may cause the automatic deletion of other instances,
 *  such as associations that refer to that instance.
 *
 *  If DeleteInstance is successful, the implementation removes the specified
 *  instance.
 *
 *  If DeleteInstance is unsuccessful, the implementation should return the
 *  appropriate result code.
 *
 *  param: self the provider state data
 *  param: context the request context
 *  param: nameSpace enumerate instances of this namespace.
 *  param: className enumerate instances of this class.
 *  param: instanceName the name of the instance to be deleted
 *
 * Result posting: nothing
 *
 * return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_CLASS
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_NOT_SUPPORTED
 *      MI_RESULT_NOT_FOUND
 *      MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_DeleteInstance)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_ const MI_Instance* instanceName);

/**
 *  The server calls the AssociatorInstances function to find all CIM instances
 *  associated with a particular 'source' CIM instance.
 *
 *  The instanceName input parameter defines the source CIM instance, whose
 *  associated instances shall be returned.
 *
 *  The className input parameter, if not null, is the name of an association
 *  class. It filters the returned set of instances by requiring that each
 *  returned instance is associated to the source instance through an instance
 *  of this class or one of its subclasses.
 *
 *  The resultClass input parameter, if not null, is the name of a class.
 *  It filters the returned set of instances by requiring that each returned
 *  instance is either this class or one of its subclasses. Note that the
 *  resultClass shall not refer to an association class.
 *
 *  The role input parameter, if not null, is a valid property name. It filters
 *  the returned set of instances by requiring that each returned instance be
 *  associated with the source instance through an association that contains
 *  a reference property with this name that refers to the source instance.
 *
 *  The resultRole input parameter, if not null, is a valid property name. It
 *  filters the returned set of instances by requiring that each returned
 *  instance shall be associated to the source instance through an association
 *  that contains a reference property with this name that refers to the
 *  returned instance.
 *
 *  If the propertySet input parameter is not null, the elements of the set
 *  define zero or more property names. Each returned instance shall include
 *  only properties in that set. If propertySet is empty, no properties are
 *  included in each returned instance. If propertySet is null, no additional
 *  filtering is performed.
 *
 *  If the propertySet input parameter contains invalid properties, the
 *  implementation shall reject the request.
 *
 *  If the resultClass input parameter is null, the propertySet shall be null
 *  as well (otherwise the class to which the property names refer, would be
 *  unknown).
 *
 *  If keysOnly is true, only key properties are included in the result
 *  instances.
 *
 *  If AssociatorInstances returns MI_RESULT_NOT_SUPPORTED, the server
 *  attempts to satisfy the request by calling EnumerateInstances. Unless
 *  the number of associators is very small, the AssociatorInstances operation
 *  shall be implemented.
 *
 *  If AssociatorInstances is successful, it returns zero or more CIM instances.
 *  Note that these instances may reside in a different namespace than the
 *  source instance (given by instanceName). The implementation must ensure that
 *  the namespace of the MI_Instance is set correctly.
 *
 *  If AssociatorInstances is unsuccessful, it returns the appropriate result
 *  code.
 *
 *  param: self the provider state data
 *  param: context the request context
 *  param: nameSpace the target namespace
 *  param: className the name of the association class (or NULL)
 *  param: instanceName the source class for the association.
 *  param: resultClass the name of the result class (or NULL)
 *  param: role the property name referring to the source instance.
 *  param: resultRole the property name referring to the result instances.
 *  param: propertySet names of properties to include or NULL for all.
 *  param: keysOnly true if only key properties are requested.
 *  param: filter used to filter the result instances, which could be of
 *        different types.
 *
 *  Result posting: zero or more instances
 *
 *  return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_NOT_SUPPORTED
 *      MI_RESULT_FAILED
 */
typedef void (MI_CALL *MI_ProviderFT_AssociatorInstances)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_ const MI_Instance* instanceName,
    _In_opt_z_ const MI_Char* resultClass,
    _In_opt_z_ const MI_Char* role,
    _In_opt_z_ const MI_Char* resultRole,
    _In_opt_ const MI_PropertySet* propertySet,
    MI_Boolean keysOnly,
    _In_opt_ const MI_Filter* filter);

/**
 *  The server calls the ReferenceInstances function to enumerate association
 *  instances that refer to a particular CIM instance.
 *
 *  The instanceName input parameter defines the target instance whose
 *  referring instances shall be returned.
 *
 *  The resultClass input parameter, if not null, is a CIM class name. It
 *  filters the returned set of association instances by requiring that each
 *  returned instance shall be an instance of this class or one of its
 *  subclasses.
 *
 *  The role input parameter, if not null, is a CIM property name. It filters
 *  the returned set of association instances by requiring that each returned
 *  instance refers to the target instance through a property with this name.
 *
 *  If the propertySet input parameter is not null, the elements of the set
 *  define zero or more property names. Each returned instance shall include
 *  only properties in that set. If propertySet is empty, no properties are
 *  included in each returned instance. If propertySet is null, no additional
 *  filtering is performed.
 *
 *  If the propertySet input parameter contains invalid properties, the
 *  implementation shall reject the request.
 *
 *  If the className input parameter is null, the propertySet shall be null
 *  as well (otherwise the class to which the property names refer, would be
 *  unknown).
 *
 *  If keysOnly is true, only key properties are included in the result
 *  instances.
 *
 *  If ReferenceInstances returns MI_RESULT_NOT_SUPPORTED, the server
 *  attempts to satisfy the request by calling EnumerateInstances. Unless
 *  the number of associators is very small, the ReferenceInstances operation
 *  shall be implemented.
 *
 *  If ReferenceInstances is successful, the implementation returns zero
 *  or more instances.
 *
 *  If ReferenceInstances is unsuccessful, the implementation returns the
 *  appropriate error result.
 *
 *  param: self the provider state data.
 *  param: context the request context.
 *  param: nameSpace the target namespace.
 *  param: className the name of the result class.
 *  param: instanceName find references of the instance with this name.
 *  param: role the association property name that refers to instanceName.
 *  param: propertySet get these properties or all if NULL.
 *  param: keysOnly get only key properties.
 *  param: filter use to filter instances.
 *
 *  Result posting: zero or more reference instances.
 *
 *  return:
 *      MI_RESULT_OK
 *      MI_RESULT_ACCESS_DENIED
 *      MI_RESULT_INVALID_NAMESPACE
 *      MI_RESULT_INVALID_PARAMETER
 *      MI_RESULT_NOT_SUPPORTED
 *      MI_RESULT_FAILED
 */
typedef void (MI_CALL *MI_ProviderFT_ReferenceInstances)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_ const MI_Instance* instanceName,
    _In_opt_z_ const MI_Char* role,
    _In_opt_ const MI_PropertySet* propertySet,
    MI_Boolean keysOnly,
    _In_opt_ const MI_Filter* filter);

/**
 * The server calls this function to enable indications delivery
 * from the provider. Provider must store context and use it later
 * for posting indications whenever it has new event.
 * Simple implementation of providers may ignore Subscribe and
 * Unsubscribe calls and always post new indications.
 * Advanced providers may analyze filters in subscribe to perform
 * fine filtering of the indications (mostly for performance reasons).
 * Note: that's the only function where provider does not call
 * PostResult and stores context until DisableIndications call.
 *
 * param: self the provider state data.
 * param: indicationsContext the context for indications delivery
 *
 * Result posting: zero or more indication instances.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_EnableIndications)(
    _In_opt_ void* self,
    _In_ MI_Context* indicationsContext,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className);

/**
 * The server calls this function to disable indications delivery
 * from the provider. Provider must stop emitting indications and
 * confirm operations by PostResult(OK) on given context.
 * Server provides the same context pointer as it did in
 * corresponding EnableIndication call before.
 *
 * param: self the provider state data.
 * param: indicationsContext the context for indications delivery
 *
 * Result posting: nothing
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
typedef void (MI_CALL *MI_ProviderFT_DisableIndications)(
    _In_opt_ void* self,
    _In_ MI_Context* indicationsContext,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className);

/**
 * The server invokes this function to subscribe to indications. The
 * provider may highjack the calling thread (not recommended) or create a new
 * thread in order to process indications. As events occur, the provider
 * should create indication instances and pass them to MI_Context_PostInstance(),
 * with context provided by EnableIndications call. See EnableIndications for
 * details.
 *
 * Subscribe is called between calls to EnableIndications and DisableIndications.
 *
 * param: self the provider state data.
 * param: context the request context used only for request confirmation.
 * param: filter used to filter indications.
 * param: subscriptionID unique id of the subscription.
 *
 * Result posting: nothing
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_ACCESS_DENIED,
 *     MI_RESULT_CANCELED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_Subscribe)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_opt_ const MI_Filter* filter,
    _In_opt_z_ const MI_Char* bookmark,
    MI_Uint64  subscriptionID,
    _Outptr_result_maybenull_ void** subscriptionSelf);

/**
 * The server invokes this function to unsubscribe from indications.
 * The provider can match subscribe/unsubscribe calls by subscriptionID.
 *
 * Unsubscribe is called between calls to EnableIndications and
 * DisableIndications.
 *
 * param: self the provider state data.
 * param: context the request context
 * param: subscriptionID unique id of the subscription.
 *
 * Result posting:
 *     MI_Context_PostInstance()
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_Unsubscribe)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    MI_Uint64  subscriptionID,
    _In_opt_ void* subscriptionSelf);

/**
 * The server calls this function to carry out a CIM extrinsic method
 * invocation on behalf of a requestor. The provider receives input parameters,
 * carries out the invoke request, and posts output parameters.
 *
 * For static methods, the 'instanceName' parameter is null. For non-static
 * methods, 'instanceName' defines a target instance (through its keys).
 *
 * Note: the implementation must set the 'MIReturn' output parameter.
 *
 * param: self the provider state data.
 * param: context the request context
 * param: nameSpace the namespace of the request.
 * param: className the name of the class.
 * param: methodName the name of the method.
 * param: instanceName the name of the target instance (null if static method).
 * param: inputParameters the input parameters for the method invocation.
 *
 * Result posting: the output parameters.
 *
 * return:
 *     MI_RESULT_OK
 *     MI_RESULT_ACCESS_DENIED
 *     MI_RESULT_NOT_SUPPORTED,
 *     MI_RESULT_INVALID_NAMESPACE
 *     MI_RESULT_NOT_FOUND
 *     MI_RESULT_METHOD_NOT_FOUND
 *     MI_RESULT_METHOD_NOT_AVAILABLE
 *     MI_RESULT_FAILED
 *
 */
typedef void (MI_CALL *MI_ProviderFT_Invoke)(
    _In_opt_ void* self,
    _In_ MI_Context* context,
    _In_z_ const MI_Char* nameSpace,
    _In_z_ const MI_Char* className,
    _In_z_ const MI_Char* methodName,
    _In_ const MI_Instance* instanceName,
    _In_ const MI_Instance* inputParameters);

/** Defines the function table for providers. */
struct _MI_ProviderFT
{
    MI_ProviderFT_Load Load;
    MI_ProviderFT_Unload Unload;
    MI_ProviderFT_GetInstance GetInstance;
    MI_ProviderFT_EnumerateInstances EnumerateInstances;
    MI_ProviderFT_CreateInstance CreateInstance;
    MI_ProviderFT_ModifyInstance ModifyInstance;
    MI_ProviderFT_DeleteInstance DeleteInstance;
    MI_ProviderFT_AssociatorInstances AssociatorInstances;
    MI_ProviderFT_ReferenceInstances ReferenceInstances;
    MI_ProviderFT_EnableIndications EnableIndications;
    MI_ProviderFT_DisableIndications DisableIndications;
    MI_ProviderFT_Subscribe Subscribe;
    MI_ProviderFT_Unsubscribe Unsubscribe;
    MI_ProviderFT_Invoke Invoke;
};


/*
**==============================================================================
**
** The MI_Module Module
**
**==============================================================================
*/

/** Whether standard qualifiers were generated */
#define MI_MODULE_FLAG_STANDARD_QUALIFIERS (1 << 0)

/** Whether description qualifiers were generated */
#define MI_MODULE_FLAG_DESCRIPTIONS (1 << 1)

/** Whether Values and ValueMap qualifiers were generated */
#define MI_MODULE_FLAG_VALUES (1 << 2)

/** Whether the MappingStrings qualifiers were generated */
#define MI_MODULE_FLAG_MAPPING_STRINGS (1 << 3)

/** Whether the boolean qualifiers were generated */
#define MI_MODULE_FLAG_BOOLEANS (1 << 4)

/** Whether C++ extensions were generated */
#define MI_MODULE_FLAG_CPLUSPLUS (1 << 5)

/** Whether translatable qualifiers were localized (and STRING.RC generated) */
#define MI_MODULE_FLAG_LOCALIZED (1 << 6)

/** Whether filters are supported */
#define MI_MODULE_FLAG_FILTER_SUPPORT (1 << 7)


/**
 * This function is called to load the main provider module. The implementation
 * resides in the file named module.c. The provider developer may define a
 * suitable MI_Module_Self structure in module.c.
 *
 * Note: this function is asynchronous.
 *
 * param: self the module state data.
 * param: context the invocation context.
 *
 * See also: MI_Module_Unload()
 *
 */
typedef void (MI_CALL *MI_Module_Load)(
    _Out_ MI_Module_Self** self,
    _In_ MI_Context* context);

/**
 * This function is called to unload the main provider module. The
 * implementation resides in the file named module.c.
 *
 * Note: this function is synchronous.
 *
 * param: self the module state data.
 * param: context the invocation context.
 *
 * See also: MI_Module_Load()
 *
 */
typedef void (MI_CALL *MI_Module_Unload)(
    _In_opt_ MI_Module_Self* self,
    _In_ MI_Context* context);

/** This structure is returned by the MI_Main() entry point. It contains
 *  all data needed by the provider manager to manage the providers within this
 *  module. A typical implementation of MI_Main() looks something like this.
 *
 *  The module may specify both static and dynamic providers. The provider
 *  manager first attempts to find a static provider function table through
 *  the MI_Module.schemaDecl field. If this fails (or if the field is NULL),
 *  it then uses the MI_Module.dynamicProviderFT (if non-NULL). Static
 *  providers provides only CIM instances, but dynamic providers may provider
 *  CIM instances, CIM classes, and CIM qualifier declarations.
 *
 */
typedef struct _MI_Module
{
    /** The version the provider was compiled with (MI_VERSION) */
    MI_Uint32 version;

    /** The hex value of MI_VERSION when the generator was compiled */
    MI_Uint32 generatorVersion;

    /** Module flags (see MI_MODULE_FLAG_* enumerations) */
    MI_Uint32 flags;

    /** Size of the MI_Char in bytes */
    MI_Uint32 charSize;

    /** Pointer to generated schema declarations (static providers only). */
    MI_SchemaDecl* schemaDecl;

    /** Library initializer */
    MI_Module_Load Load;

    /** Library cleanup */
    MI_Module_Unload Unload;

    /** The module may implement a single 'dynamic provider' (one that
     *  provides CIM instances, CIM classes and CIM qualifier declarations).
     *  The provider manager uses this function table when (1) it is non-null,
     *  and (2) MI_Module.schemaDecl is null or does not contain an RTTI
     *  corresponding to the given request.
     */
    const MI_ProviderFT* dynamicProviderFT;
}
MI_Module;

/*
**==============================================================================
**
** The MI_Instance Module
**
**==============================================================================
*/

/** The MI_Instance function table */
struct _MI_InstanceFT
{
    MI_Result (MI_CALL *Clone)(
        _In_ const MI_Instance* self,
        _Outptr_ MI_Instance** newInstance);

    MI_Result (MI_CALL *Destruct)(
        _Inout_ MI_Instance* self);

    MI_Result (MI_CALL *Delete)(
        _Inout_ MI_Instance* self);

    MI_Result (MI_CALL *IsA)(
        _In_ const MI_Instance* self,
        _In_ const MI_ClassDecl* classDecl,
        _Out_ MI_Boolean* flag);

    MI_Result (MI_CALL *GetClassName)(
        _In_ const MI_Instance* self,
        _Outptr_result_maybenull_z_ const MI_Char** className);

    MI_Result (MI_CALL *SetNameSpace)(
        _Inout_ MI_Instance* self,
        _In_z_ const MI_Char* nameSpace);

    MI_Result (MI_CALL *GetNameSpace)(
        _In_ const MI_Instance* self,
        _Outptr_result_maybenull_z_ const MI_Char** nameSpace);

    MI_Result (MI_CALL *GetElementCount)(
        _In_ const MI_Instance* self,
        _Out_ MI_Uint32* count);

    MI_Result (MI_CALL *AddElement)(
        _Inout_ MI_Instance* self,
        _In_z_ const MI_Char* name,
        _In_opt_ const MI_Value* value,
        MI_Type type,
        MI_Uint32 flags);

    MI_Result (MI_CALL *SetElement)(
        _Inout_ MI_Instance* self,
        _In_z_ const MI_Char* name,
        _In_opt_ const MI_Value* value,
        MI_Type type,
        MI_Uint32 flags);

    MI_Result (MI_CALL *SetElementAt)(
        _Inout_ MI_Instance* self,
        MI_Uint32 index,
        _In_reads_opt_(_Inexpressible_("varies depending on type")) const MI_Value* value,
        MI_Type type,
        MI_Uint32 flags);

    MI_Result (MI_CALL *GetElement)(
        _In_ const MI_Instance* self,
        _In_z_ const MI_Char* name,
        _Out_opt_ MI_Value* value,
        _Out_opt_ MI_Type* type,
        _Out_opt_ MI_Uint32* flags,
        _Out_opt_ MI_Uint32* index);

    MI_Result (MI_CALL *GetElementAt)(
        _In_ const MI_Instance* self,
        MI_Uint32 index,
        _Outptr_result_maybenull_z_ const MI_Char** name,
        _Out_opt_ MI_Value* value,
        _Out_opt_ MI_Type* type,
        _Out_opt_ MI_Uint32* flags);

    MI_Result (MI_CALL *ClearElement)(
        _Inout_ MI_Instance* self,
        _In_z_ const MI_Char* name);

    MI_Result (MI_CALL *ClearElementAt)(
        _Inout_ MI_Instance* self,
        MI_Uint32 index);

    MI_Result (MI_CALL *GetServerName)(
        _In_ const MI_Instance* self,
        _Outptr_result_maybenull_z_ const MI_Char** name);

    MI_Result (MI_CALL *SetServerName)(
        _Inout_ MI_Instance* self,
        _In_z_ const MI_Char* name);

    MI_Result (MI_CALL *GetClass)(
        _In_ const MI_Instance* self,
        _Outptr_ MI_Class** instanceClass);

};

struct _MI_InstanceExFT
{
    MI_InstanceFT parent;

    MI_Result (MI_CALL *Normalize)(
        _Inout_ MI_Instance** self);
};

/**
 * This structure represents a CIM instance. Both dynamic and static instances
 * implement this interface. Static instance structures include this structure
 * as the initial data field.
 *
 *     Knowledge of this structure allows provider to create its own,
 *     server-compatible (for RO operations) instances to improve performance.
 *
 *     The following restrictions are made for 'reserved' part of MI_Instance:
 *     - when server sends instance to the provider, it has to initialize
 *     ft, classDecl, serverName and namespace (last two may be null).
 *    'reserved' space is server-specific and provider may not
 *    make any assumptions about its content.
 *
 *      - when provider sends instance to the server (PostResult, PostIndication)
 *      it has to initialize classDecl, serverName (optional) and
 *      namespace (optional). ft and reserved space must be 0-initialized.
 *      Server may perform only read only operations on such instance.

 */
struct _MI_Instance
{
    /** Function table */
    const MI_InstanceFT* ft;

    /** The class declaration for this instance */
    const MI_ClassDecl* classDecl;

    /** The server name (optional) */
    const MI_Char* serverName;

    /** The namespace (optional) */
    const MI_Char* nameSpace;

    /** Reserved for internal use */
    ptrdiff_t reserved[4];
};

/**
 *
 * This function creates a copy of the given instance on the heap. Upon
 * a successful return, newInstance points to a newly created instance. The
 * new instance should eventually be passed to MI_Instance_Delete().
 *
 * param: self pointer to the instance to be cloned.
 * param: newInstance a pointer to the new instance upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 * See also: MI_Instance_Delete()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_Clone(
    _In_ const MI_Instance* self,
    _Outptr_ MI_Instance** newInstance)
{
    if (self && self->ft)
    {
        return self->ft->Clone(self, newInstance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * This function releases an instance that was created on the stack. This
 * function applies to instances constructed on the stack using generated
 * functions of the form CLASSNAME_Construct().
 *
 * param: self pointer to the instance to be destructed.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 * See also: MI_Instance_Delete()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_Destruct(
    _Inout_ MI_Instance* self)
{
    if (self && self->ft)
    {
        return self->ft->Destruct(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * This function releases an instance that was created on the heap. Functions
 * created with MI_Instance_Clone() should eventually be passed to this
 * function.
 *
 * param: self pointer to the instance to be released.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_Delete(
    _Inout_ MI_Instance* self)
{
    if (self && self->ft)
    {
        return self->ft->Delete(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function checks to see if the instance given by 'self' is an
 * instance of the class given by 'classDecl'. If so it return
 * MI_TRUE in flag. Otherwise it returns MI_FALSE in flag out parameter.
 *
 * param: self pointer to an instance.
 * param: classDecl the potential 'super' class declaration.
 * param: flag [out] boolean outcome of the operation.
 *
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_IsA(
    _In_ const MI_Instance* self,
    _In_ const MI_ClassDecl* classDecl,
    _Out_ MI_Boolean* flag)
{
    if (self && self->ft)
    {
        return self->ft->IsA(self, classDecl, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets the className of the given instance.
 *
 * param: self instance whose className will be gotten.
 * param: className the class name of the instance upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetClassName(
    _In_ const MI_Instance* self,
    _Outptr_result_maybenull_z_ const MI_Char** className)
{
    if (self && self->ft)
    {
        return self->ft->GetClassName(self, className);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Sets the namespace name of the given instance. Namespace names must conform
 * to the following productions:
 *
 * Examples:
 *     root
 *     root/cimv2
 *     aaa/bbb/ccc
 *
 * param: self the instance whose property will be gotten.
 * param: nameSpace the new namespace of the instance.
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER, MI_RESULT_FAILED
 *         MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_SetNameSpace(
    _Inout_ MI_Instance* self,
    _In_z_ const MI_Char* nameSpace)
{
    if (self && self->ft)
    {
        return self->ft->SetNameSpace(self, nameSpace);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets the namespace name of the given instance.
 *
 * param: self a pointer to an instance.
 * param: nameSpace the namespace name upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetNameSpace(
    _In_ const MI_Instance* self,
    _Outptr_result_maybenull_z_ const MI_Char** nameSpace)
{
    if (self && self->ft)
    {
        return self->ft->GetNameSpace(self, nameSpace);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Retrieves the number of properties in an instance.
 *
 * param: self the instance
 * param: count the number of properties in the instance upon return
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER, MI_RESULT_INVALID_PARAMETER
 *
 * See also: MI_Instance_GetAt(), MI_Instance_SetAt()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetElementCount(
    _In_ const MI_Instance* self,
    _Out_ MI_Uint32* count)
{
    if (self && self->ft)
    {
        return self->ft->GetElementCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * Adds a new property to a dynamic instance (supported only by dynamic
 * instances whose schema may be extended at run time).
 *
 * param: self the instance
 * param: name the name of the new property
 * param: value the value of the new property
 * param: type the type of the new property
 * param: flags the flags of the new property (MI_FLAG_KEY, MI_FLAG_IN,
 *        MI_FLAG_OUT, MI_FLAG_REQUIRED, MI_FLAG_STREAM)
 *        also indicates memory management policy
 *              (MI_FLAG_BORROW, MI_FLAG_ADOPT)
 *        null value (MI_FLAG_NULL), and
 *        CIM meta types (MI_FLAG_ANY).
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_AddElement(
    _Inout_ MI_Instance* self,
    _In_z_ const MI_Char* name,
    _In_opt_ const MI_Value* value,
    MI_Type type,
    MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->AddElement(self, name, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Set the value of the property at the given index.
 *
 * param: self a pointer to an instance.
 * param: index the integer position of the property.
 * param: value the new value for the property.
 * param: type the CIM type of the property that will be set.
 * param: flags bit flags indicating memory management policy
 *              (MI_FLAG_BORROW, MI_FLAG_ADOPT) and null value
 *              (MI_FLAG_NULL).
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_TYPE_MISMATCH
 *     MI_RESULT_INVALID_PARAMETER, MI_RESULT_NOT_FOUND, MI_RESULT_FAILED
 *
 * See also: MI_Instance_GetAt()
 * See also: MI_Instance_GetElementCount()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_SetElementAt(
    _Inout_ MI_Instance* self,
    MI_Uint32 index,
    _In_reads_opt_(_Inexpressible_("varies depending on type")) const MI_Value* value,
    MI_Type type,
    MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->SetElementAt(self, index, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Set the value of the property with the given name. By default, all memory
 * referred to by the value parameter is copied. By passing the flag
 * MI_FLAG_BORROW, memory pointers within the value structure are stored
 * directly in the instance's property. The caller must guarantee that the
 * memory outlives the instance.
 *
 * param: self a pointer to an instance.
 * param: name the name of the property that will be set.
 * param: value the new value for the property.
 * param: type the CIM type of the property that will be set.
 * param: flags bit flags indicating memory management policy
 *              (MI_FLAG_BORROW, MI_FLAG_ADOPT) and null value
 *              (MI_FLAG_NULL).
 *
 * return: MI_RESULT_O, KMI_RESULT_TYPE_MISMATCH, MI_RESULT_INVALID_PARAMETER
 *     MI_RESULT_NOT_FOUND, MI_RESULT_FAILED
 *
 * See also: MI_Instance_GetElement()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_SetElement(
    _Inout_ MI_Instance* self,
    _In_z_ const MI_Char* name,
    _In_opt_ const MI_Value* value,
    MI_Type type,
    MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->SetElement(self, name, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * Gets the value of the property with the given name.
 *
 * param: self the instance whose property will be gotten.
 * param: name the name of the property that will be gotten.
 * param: value contains the value of the property upon return.
 * param: type contains the type of the property upon return.
 * param: flags the flags associated with property (MI_FLAG_NULL, MI_FLAG_KEY,
 *        MI_FLAG_IN, MI_FLAG_OUT).
 * param: index the index of the named attribute.
 *
 * return: MI_RESULT_OK, MI_RESULT_TYPE_MISMATCH, MI_RESULT_INVALID_PARAMETER
 *     MI_RESULT_NOT_FOUND, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetElement(
    _In_ const MI_Instance* self,
    _In_z_ const MI_Char* name,
    _Out_opt_ MI_Value* value,
    _Out_opt_ MI_Type* type,
    _Out_opt_ MI_Uint32* flags,
    _Out_opt_ MI_Uint32* index)
{
    if (self && self->ft)
    {
        return self->ft->GetElement(self, name, value, type, flags, index);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * Get the value of the property at the given index.
 *
 * param: self the instance whose property will be gotten.
 * param: index the position of the property that will be set.
 * param: name the name of the property upon return.
 * param: value the value of the property upon return.
 * param: type the type of the property upon return.
 * param: flags the flags of the property upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_INVALID_PARAMETER, MI_RESULT_FAILED,
 * MI_RESULT_FAILED
 *
 * See also: MI_Instance_SetAt()
 * See also: MI_Instance_GetElementCount()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetElementAt(
    _In_ const MI_Instance* self,
    MI_Uint32 index,
    _Outptr_result_maybenull_z_ const MI_Char** name,
    _Out_opt_ MI_Value* value,
    _Out_opt_ MI_Type* type,
    _Out_opt_ MI_Uint32* flags)
{
    if (self && self->ft)
    {
        return self->ft->GetElementAt(self, index, name, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * Clears the value of the property with the given name. Afterwards, the
 * property has a null value.
 *
 * param: self the instance whose property will be set.
 * param: name the name of the property that will be cleared.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_ClearElement(
    _Inout_ MI_Instance* self,
    _In_z_ const MI_Char* name)
{
    if (self && self->ft)
    {
        return self->ft->ClearElement(self, name);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * Clears the value of the property at the given index. Afterwards, the
 * property has a null value.
 *
 * param: self the instance whose property will be cleared.
 * param: index the position of the property that will be cleared.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_ClearElementAt(
    _Inout_ MI_Instance* self,
    MI_Uint32 index)
{
    if (self && self->ft)
    {
        return self->ft->ClearElementAt(self, index);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 *
 * Gets the server name from the instance.  The resultant name
 * memory is owned by the instance and will be destroyed when
 * the instance is deleted.
 *
 * param: self the instance whose property will be cleared.
 * param: name is the outbound server name string.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetServerName(
    _In_ const MI_Instance* self,
    _Outptr_result_maybenull_z_ const MI_Char** name)
{
    if (self && self->ft)
    {
        return self->ft->GetServerName(self, name);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_SetServerName(
    _Inout_ MI_Instance* self,
    _In_z_ const MI_Char* name)
{
    if (self && self->ft)
    {
        return self->ft->SetServerName(self, name);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_GetClass(
    _In_ const MI_Instance* self,
    _Outptr_ MI_Class** instanceClass)
{
    if (self && self->ft)
    {
        return self->ft->GetClass(self, instanceClass);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_INLINE_CALL MI_Instance_Normalize(
    _Inout_ MI_Instance** self)
{
    MI_Instance* inst = *self;
    if (inst && inst->ft)
    {
        if (inst->classDecl->flags & MI_FLAG_EXTENDED)
        {
            MI_InstanceExFT* ft = (MI_InstanceExFT*)inst->ft;
            return ft->Normalize(self);
        }
        else
        {
            return MI_RESULT_OK;
        }
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**==============================================================================
**
** The MI_Context Module
**
**==============================================================================
*/

/* The maximum size of a locale string (including zero-terminator). */
#define MI_MAX_LOCALE_SIZE 128

/* Defines the support locale enum tags used by the MI_Context.GetLocale() */
typedef enum _MI_LocaleType
{
    MI_LOCALE_TYPE_REQUESTED_UI,
    MI_LOCALE_TYPE_REQUESTED_DATA,
    MI_LOCALE_TYPE_CLOSEST_UI,
    MI_LOCALE_TYPE_CLOSEST_DATA
}
MI_LocaleType;

typedef enum _MI_CancellationReason
{
    MI_REASON_NONE,
    MI_REASON_TIMEOUT,
    MI_REASON_SHUTDOWN,
    MI_REASON_SERVICESTOP
}
MI_CancellationReason;


typedef void (MI_CALL *MI_CancelCallback)(
    MI_CancellationReason reason,
    _In_opt_ void* callbackData);

/* Defines the channel numbers for WriteMessage PS semantic callback */
#define MI_WRITEMESSAGE_CHANNEL_WARNING 0
#define MI_WRITEMESSAGE_CHANNEL_VERBOSE 1
#define MI_WRITEMESSAGE_CHANNEL_DEBUG   2

/* Defines the resultType for the result code */

#define MI_RESULT_TYPE_MI MI_T("MI") /* MI Result type*/
#define MI_RESULT_TYPE_HRESULT MI_T("HRESULT") /* HRESULT Result type */
#define MI_RESULT_TYPE_WIN32 MI_T("WIN32") /* WIN32 Result type*/
#define MI_RESULT_TYPE_ERRNO MI_T("ERRNO") /* CRT ERRNO Result type*/


/** Defines the function table used by MI_Context */
struct _MI_ContextFT
{
    /*
    **--------------------------------------------------------------------------
    **
    ** Post Methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *PostResult)(
        _In_ MI_Context* context,
        MI_Result result);

    MI_Result (MI_CALL *PostInstance)(
        _In_ MI_Context* context,
        _In_ const MI_Instance* instance);

    MI_Result (MI_CALL *PostIndication)(
        _In_ MI_Context* context,
        _In_ const MI_Instance* indication,
        MI_Uint32 subscriptionIDCount,
        _In_opt_z_ const MI_Char* bookmark);

    /*
    **--------------------------------------------------------------------------
    **
    ** Factory Methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *ConstructInstance)(
        _In_ MI_Context* context,
        _In_ const MI_ClassDecl* classDecl,
        _Out_ MI_Instance* instance);

    MI_Result (MI_CALL *ConstructParameters)(
        _In_ MI_Context* context,
        _In_ const MI_MethodDecl* methodDecl,
        _Out_ MI_Instance* instance);

    MI_Result (MI_CALL *NewInstance)(
        _In_ MI_Context* context,
        _In_ const MI_ClassDecl* classDecl,
        _Outptr_ MI_Instance** instance);

    MI_Result (MI_CALL *NewDynamicInstance)(
        _In_ MI_Context* context,
        _In_ const MI_Char* className,
        MI_Uint32 flags,
        _Outptr_ MI_Instance** instance);

    MI_Result (MI_CALL *NewParameters)(
        _In_ MI_Context* context,
        _In_ const MI_MethodDecl* methodDecl,
        _Outptr_ MI_Instance** instance);

    /*
    **--------------------------------------------------------------------------
    **
    ** Misc. Methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *Canceled)(
        _In_ const MI_Context* context,
        _Out_ MI_Boolean* flag);

    MI_Result (MI_CALL *GetLocale)(
        _In_ const MI_Context* context,
        MI_LocaleType localeType,
        _Out_writes_z_(MI_MAX_LOCALE_SIZE) MI_Char locale[MI_MAX_LOCALE_SIZE]);

    MI_Result (MI_CALL *RegisterCancel)(
        _In_ MI_Context* context,
        _In_ MI_CancelCallback callback,
        _In_opt_ void* callbackData);

    MI_Result (MI_CALL *RequestUnload)(
        _In_ MI_Context* context);

    MI_Result (MI_CALL *RefuseUnload)(
        _In_ MI_Context* context);

    MI_Result (MI_CALL *GetLocalSession)(
        _In_ const MI_Context* context,
        _Out_ MI_Session* session);

    MI_Result (MI_CALL *SetStringOption)(
        _In_ MI_Context* context,
        _In_z_ const MI_Char* name,
        _In_z_ const MI_Char* value);

 /*
    **--------------------------------------------------------------------------
    **
    ** Operation Options methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *GetStringOption)(
        _In_  MI_Context* context,
        _In_z_ const MI_Char* name,
        _Outptr_result_z_  const MI_Char** value);

    MI_Result (MI_CALL *GetNumberOption)(
        _In_  MI_Context* context,
        _In_z_ const MI_Char *name,
        _Out_opt_  MI_Uint32* value);

    MI_Result (MI_CALL *GetCustomOption)(
        _In_  MI_Context* context,
        _In_z_ const MI_Char* name,
        _Out_opt_  MI_Type* valueType,
        _Out_opt_  MI_Value* value);

    MI_Result (MI_CALL *GetCustomOptionCount)(
        _In_  MI_Context* context,
        _Out_opt_ MI_Uint32* count);

    MI_Result (MI_CALL *GetCustomOptionAt)(
        _In_  MI_Context* context,
        _In_ MI_Uint32 index,
        _Outptr_opt_result_maybenull_z_  const MI_Char** name,
        _Out_opt_  MI_Type* valueType,
        _Out_opt_  MI_Value* value);

    /*
    **--------------------------------------------------------------------------
    **
    ** CIM Extension Methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *WriteMessage)(
        _In_ MI_Context* context,
        MI_Uint32 channel,
        _In_z_ const MI_Char* message);

    MI_Result (MI_CALL *WriteProgress)(
        _In_ MI_Context* context,
        _In_z_ const MI_Char* activity,
        _In_z_ const MI_Char* currentOperation,
        _In_z_ const MI_Char* statusDescription,
        MI_Uint32 percentComplete,
        MI_Uint32 secondsRemaining);

    MI_Result (MI_CALL *WriteStreamParameter)(
        _In_ MI_Context* context,
        _In_z_ const MI_Char* name,
        _In_ const MI_Value* value,
        _In_ MI_Type type,
        _In_ MI_Uint32 flags);

    MI_Result (MI_CALL *WriteCimError)(
        _In_ MI_Context* context,
        _In_ const MI_Instance *error,
        _Out_ MI_Boolean *flag);

    MI_Result (MI_CALL *PromptUser)(
        _In_ MI_Context* context,
        _In_z_ const MI_Char* message,
        MI_PromptType promptType,
        _Out_ MI_Boolean* result );



    /*
    **--------------------------------------------------------------------------
    **
    ** CIM Extension Helper Methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *ShouldProcess)(
        _In_ MI_Context* context,
        _In_z_ const MI_Char* target,
        _In_z_ const MI_Char* action,
        _Out_ MI_Boolean* result);

    MI_Result (MI_CALL *ShouldContinue)(
        _In_ MI_Context* context,
        _In_z_ const MI_Char* message,
        _Out_ MI_Boolean* result);

    /*
    **--------------------------------------------------------------------------
    **
    ** New Methods
    **
    **--------------------------------------------------------------------------
    */

    MI_Result (MI_CALL *PostError)(
        _In_ MI_Context* context,
        MI_Uint32 resultCode,
        _In_z_ const MI_Char* resultType,
        _In_z_ const MI_Char* errorMessage);

    MI_Result (MI_CALL *PostCimError)(
        _In_ MI_Context* context,
        _In_ const MI_Instance *error);

    MI_Result (MI_CALL *WriteError)(
        _In_ MI_Context* context,
        MI_Uint32 resultCode,
        _In_z_ const MI_Char* resultType,
        _In_z_ const MI_Char* errorMessage,
        _Out_ MI_Boolean *flag);
};

/** Operations are defined on this structure for (1) posting results,
 *  (2) posting instances, (3) creating new objects.
 *
 */
struct _MI_Context
{
    /* Function table */
    const MI_ContextFT* ft;

    /* Reserved for internal use */
    ptrdiff_t reserved[3];
};

/**
 * Providers call this function to post a return code to the server in
 * response to a request.
 *
 * param: context the request context
 * param: result the result code
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PostResult(
    _In_ MI_Context* context,
    MI_Result result)
{
    if (context && context->ft)
    {
        return context->ft->PostResult(context, result);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Providers call this function to post a return code and an error message
 * to the server in response to a request.
 *
 * param: context the request context. * param: error representing CIM_Error.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PostCimError(
    _In_ MI_Context* context,
    _In_ const MI_Instance *error)
{
    if (context && context->ft)
    {
        return context->ft->PostCimError(context, error);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Providers call this function to post a return code and a CIM error
 * instance to the server in response to a request.
 *
 * param: context the request context.
 * param: result the result code.
 * param: resultType of the result
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PostError(
    _In_ MI_Context* context,
    MI_Uint32 resultCode,
    _In_z_ const MI_Char* resultType,
    _In_z_ const MI_Char* errorMessage)
{
    if (context && context->ft)
    {
        return context->ft->PostError(context, resultCode, resultType, errorMessage);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/**
 * Providers call this function to post an instance to the server in
 * response to a request. The server is responsible for copying the
 * instance so the provider is free to dispose of the instance afterwards.
 *
 * param: context the request context
 * param: instance instance to be posted
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PostInstance(
    _In_ MI_Context* context,
    _In_ const MI_Instance* instance)
{
    if (context && context->ft)
    {
        return context->ft->PostInstance(context, instance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Providers call this function to post an indication to the server in
 * response to a request. The server is responsible for copying the
 * instance so the provider is free to dispose of the instance afterwards.
 *
 * param: context the request context
 * param: indication the indication to be posted
 * param: subscriptionIDCount the number of subscription identifiers.
 * param: bookmark the bookmark for this subscription.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PostIndication(
    _In_ MI_Context* context,
    _In_ const MI_Instance* indication,
    MI_Uint32 subscriptionIDCount,
    _In_opt_z_ const MI_Char* bookmark)
{
    if (context && context->ft)
    {
        return context->ft->PostIndication(
            context, indication, subscriptionIDCount, bookmark);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * A provider calls this function to initialize an instance. The caller
 * is responsible for reserving the memory for the instance (either on
 * the stack or the heap). The caller should eventually pass
 * the instance to MI_Instance_Destruct().
 *
 * param: context the request context.
 * param: instance the instance to be initialized.
 * param: classDecl the class declaration used to initialize the instance.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_ConstructInstance(
    _In_ MI_Context* context,
    _In_ const MI_ClassDecl* classDecl,
    _Out_ MI_Instance* instance)
{
    if (context && context->ft)
    {
        return context->ft->ConstructInstance(context, classDecl, instance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * A provider calls this function to initialize a parameters instance.
 * The caller is responsible for reserving the memory for the instance
 * (either on the stack or the heap). The caller should eventually pass
 * the instance to MI_Instance_Destruct().
 *
 * param: context the request context.
 * param: instance the instance to be initialized.
 * param: methodDecl the method declaration used to initialize the instance.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_ConstructParameters(
    _In_ MI_Context* context,
    _In_ const MI_MethodDecl* methodDecl,
    _Out_ MI_Instance* instance)
{
    if (context && context->ft)
    {
        return context->ft->ConstructParameters(context, methodDecl, instance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function creates a new instance of the class given by the classDecl
 * parameter. The caller should eventually pass the instance to
 * MI_Instance_Delete().
 *
 * param: context the request context
 * param: classDecl the class declaration used to initialize the instance.
 * param: instance points to a new instance upon successful return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_NewInstance(
    _In_ MI_Context* context,
    _In_ const MI_ClassDecl* classDecl,
    _Outptr_ MI_Instance** instance)
{
    if (context && context->ft)
    {
        return context->ft->NewInstance(context, classDecl, instance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function creates a new dynamic instance of the class whose name is
 * given by the className parameter. The caller should eventually pass the
 * instance to MI_Instance_Delete().
 *
 * param: context the request context
 * param: className the name of the new class.
 * param: flags create flags (include class meta type).
 * param: instance points to a new instance upon successful return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_NewDynamicInstance(
    _In_ MI_Context* context,
    _In_ const MI_Char* className,
    MI_Uint32 flags,
    _Outptr_ MI_Instance** instance)
{
    if (context && context->ft)
    {
        return context->ft->NewDynamicInstance(context, className, flags, instance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function creates a new instance of the method given by the
 * methodDecl parameter. The caller should eventually pass the instance to
 * MI_Instance_Delete().
 *
 * param: context the request context
 * param: methodDecl the method declaration used to initialize the instance.
 * param: instance points to a new instance upon successful return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_NewParameters(
    _In_ MI_Context* context,
    _In_ const MI_MethodDecl* methodDecl,
    _Outptr_ MI_Instance** instance)
{
    if (context && context->ft)
    {
        return context->ft->NewParameters(context, methodDecl, instance);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Providers call this function periodically to determine whether the
 * operation has been canceled. If so, the flag parameter is set to
 * MI_TRUE and MI_RESULT_OK is returned.
 *
 * param: context the request context
 * param: flag upon return this flag indicates whether the operation has been
 *        canceled.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_Canceled(
    _In_ const MI_Context* context,
    _Out_ MI_Boolean* flag)
{
    if (context && context->ft)
    {
        return context->ft->Canceled(context, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function returns the locale of the given type.
 *
 * param: context the request context
 * param: localeType the type of locale to be returned.
 * param: locale the locale upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetLocale(
    _In_ const MI_Context* context,
    MI_LocaleType localeType,
    _Out_writes_z_(MI_MAX_LOCALE_SIZE) MI_Char locale[MI_MAX_LOCALE_SIZE])
{
    if (locale)
    {
        locale[0] = L'\0';
    }

    if (context && context->ft)
    {
        return context->ft->GetLocale(context, localeType, locale);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function registers a callback that is called when the operation
 * is canceled.
 *
 * param: context the request context.
 * param: callback call this function on cancel.
 * param: callbackData pass this data to the callback.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_RegisterCancel(
    _In_ MI_Context* context,
    _In_ MI_CancelCallback callback,
    _In_opt_ void* callbackData)
{
    if (context && context->ft)
    {
        return context->ft->RegisterCancel(context, callback, callbackData);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function requests to unload the module or the provider (depending
 * on the location of invocation). Providers should call this function within
 * their load methods. The provider will be unloaded soon after this call.
 *
 * param: context the request context.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_RequestUnload(
    _In_ MI_Context* context)
{
    if (context && context->ft)
    {
        return context->ft->RequestUnload(context);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * By calling this, the provider prevents itself from being unloaded after
 * provider unload timeout (but it does not prevent it from being unloaded
 * during CIM server shutdown). After calling this, the provider manages
 * its own lifetime. The provider may call MI_Context_RequestUnload() to request an
 * unload at any time. This function should be called with the load method.
 *
 * param: context the request context.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_RefuseUnload(
    _In_ MI_Context* context)
{
    if (context && context->ft)
    {
        return context->ft->RefuseUnload(context);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
};

/**
 * Gets the local session (MI_Session), which allows the provider to
 * communicate with the CIM server. This session is pre-instantiated
 * and has the lifetime of the context (from which the session was
 * obtained. The provider MUST NOT destruct this session, since its
 * lifetime is bound to the context.
 *
 * param: context the request context.
 * param: session the local session handle.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetLocalSession(
    _In_ const MI_Context* context,
    _Out_ MI_Session* session)
{
    if (context && context->ft)
    {
        return context->ft->GetLocalSession(context, session);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Sets context-specific option. It allows the provider to
 * adjust server's behavior. Typically is server-specific.
 *
 * param: context the request context.
 * param: name of the option to change.
 * param: value - new value for the option.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 */

/**
 * MI_Context_SetStringOption only supports
 * "SECURITY" option, and is only used by indication
 * provider to control which users have permission to
 * subscrbe to the indication class. The API is valid if
 * and only if being called inside <Indication Class>_Load
 * function. Following example allows only adminstrators
 * group to subscribe to MyIndication class.
 *
 * void MI_CALL <MyIndication>_Load(
 *       My_Alarm_Self** self,
 *       MI_Module_Self* selfModule,
 *       MI_Context* context)
 *   {
 *       MI_Result r;
 *       *self = NULL;
 *       r = MI_Context_SetStringOption(context,
 *               MI_T("SECURITY"),
 *               MI_T("O:BAG:BAD:(A;;0x40;;;BA)"));
 *       MI_PostResult(context, r);
 *   }
 *
 *   The SDDL string "O:BAG:BAD:(A;;0x40;;;BA)" contains 0x40 flag, which means
 *   WBEM_RIGHT_SUBSCRIBE permission, the SDDL must contain this flag, please
 *   see http://msdn.microsoft.com/en-us/library/aa390415.aspx for details.
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_SetStringOption(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* name,
    _In_z_ const MI_Char* value)
{
    if (context && context->ft)
    {
        return context->ft->SetStringOption(context, name, value);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets context-specific option.
 *
 * param: context the request context.
 * param: name of the option to get.
 * param: value [out] of the option.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetStringOption(
    _In_  MI_Context* context,
    _In_z_ const MI_Char* name,
    _Outptr_result_z_  const MI_Char** value)
{
    if (context && context->ft)
    {
        return context->ft->GetStringOption(context, name, value);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets context-specific option.
 *
 * param: context the request context.
 * param: name of the option to get.
 * param: value [out] of the option.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetNumberOption(
    _In_  MI_Context* context,
    _In_z_ const MI_Char* name,
    _Out_opt_  MI_Uint32* value)
{
    if (context && context->ft)
    {
        return context->ft->GetNumberOption(context, name, value);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets context-specific option.
 *
 * param: context the request context.
 * param: name of the option to get.
 * param: value [out] of the option.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetCustomOption(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* name,
    _Out_opt_  MI_Type* valueType,
    _Out_opt_  MI_Value* value)
{
    if (context && context->ft)
    {
        return context->ft->GetCustomOption(context, name, valueType,value);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets context-specific option.
 *
 * param: context the request context.
 * param: name of the option to get.
 * param: value [out] of the option.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetCustomOptionCount(
    _In_  MI_Context* context,
    _Out_opt_ MI_Uint32* count)
{
    if (context && context->ft)
    {
        return context->ft->GetCustomOptionCount(context, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * Gets context-specific option.
 *
 * param: context the request context.
 * param: name of the option to get.
 * param: value [out] of the option.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_GetCustomOptionAt(
    _In_  MI_Context* context,
    _In_ MI_Uint32 index,
    _Outptr_opt_result_maybenull_z_  const MI_Char** name,
    _Out_opt_  MI_Type* valueType,
    _Out_opt_  MI_Value* value)
{
    if (context && context->ft)
    {
        return context->ft->GetCustomOptionAt(context, index, name, valueType,value);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/**
 * This function implements the ShouldProcess CIM Extensions operation.
 *
 * param: context the request context.
 * param: message the message.
 * param: flag MI_TRUE if for 'should process'.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_ShouldProcess(
    _In_ MI_Context* context,
    _In_z_ const MI_Char *target,
    _In_z_ const MI_Char* action,
    _Out_ MI_Boolean* flag)
{
    if (context && context->ft)
    {
        return context->ft->ShouldProcess(context, target, action , flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function implements the ShouldContinue CIM Extensions operation.
 *
 * param: context the request context.
 * param: message a message.
 * param: flag MI_TRUE if for 'should continue'.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_ShouldContinue(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* message,
    _Out_ MI_Boolean* flag)
{
    if (context && context->ft)
    {
        return context->ft->ShouldContinue(context, message, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/**
 * This function implements the MI_PromptUser CIM Extensions operation.
 *
 * param: context the request context.
 * param: message a message.
 * param: promptType prompt type (critical, normal)
 * param: flag MI_TRUE if for 'should continue'.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_PromptUser(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* message,
    MI_PromptType promptType,
    _Out_ MI_Boolean*flag )

{
    if (context && context->ft)
    {
        return context->ft->PromptUser(context, message, promptType, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/**
 * This function implements the WriteError CIM operation.
 *
 * param: context the request context.
 * param: resultCode
 * param: resultType
 * param: flag
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteError(
    _In_ MI_Context* context,
    MI_Uint32 resultCode,
    _In_z_ const MI_Char* resultType,
    _In_z_ const MI_Char* errorMessage,
    _Out_ MI_Boolean *flag)
{
    if (context && context->ft)
    {
        return context->ft->WriteError(context, resultCode, resultType, errorMessage, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function implements the WriteCimError CIM Extensions operation.
 *
 * param: context the request context.
 * param: error CIM_Error type object.
 * param: flag
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteCimError(
    _In_ MI_Context* context,
    _In_ const MI_Instance *error,
    _Out_ MI_Boolean *flag)

{
    if (context && context->ft)
    {
        return context->ft->WriteCimError(context, error, flag);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}



/**
 * This function implements the WriteMessage CIM Extensions operation.
 *
 * param: context the request context.
 * param: channel
 * param: message
 *
 * return: MI_TRUE or MI_FALSE.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteMessage(
    _In_ MI_Context* context,
    MI_Uint32 channel,
    _In_z_ const MI_Char* message)
{
    if (context && context->ft)
    {
        return context->ft->WriteMessage(context, channel, message);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function implements the WriteProgress CIM Extentions operation.
 *
 * param: context the request context.
 * param: activity
 * param: currentOperation
 * param: statusDescription
 * param: percentComplete
 * param: secondsRemaining
 *
 * return: MI_TRUE or MI_FALSE.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteProgress(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* activity,
    _In_z_ const MI_Char* currentOperation,
    _In_z_ const MI_Char* statusDescription,
    MI_Uint32 percentComplete,
    MI_Uint32 secondsRemaining)
{
    if (context && context->ft)
    {
        return context->ft->WriteProgress(context, activity, currentOperation,
            statusDescription, percentComplete, secondsRemaining);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/**
 * The provider calls this function to send streamed data to the requestor.
 * The value is an array that contains one or more elements of the specified
 * type. Call this function repeatedly to send the entire stream.
 *
 * param: self the request context
 * param: name the name of a parameter.
 * param: value an array value with at least one element.
 * param: type the type (must be an array type).
 * param: flags is reserved and the value must be 0.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteStreamParameter(
    _In_ MI_Context* self,
    _In_z_ const MI_Char* name,
    _In_ const MI_Value* value,
    _In_ MI_Type type,
    _In_ MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->WriteStreamParameter(self, name, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function implements the WriteWarning functionality of WriteMessage CIM Extensions operation.
 *
 * param: context the request context.
 * param: message
 *
 * return: MI_TRUE or MI_FALSE.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteWarning(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* message)
{
    if (context && context->ft)
    {
        return context->ft->WriteMessage(context, MI_WRITEMESSAGE_CHANNEL_WARNING, message);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function implements the WriteVerbose functionality of WriteMessage CIM Extensions operation.
 *
 * param: context the request context.
 * param: message
 *
 * return: MI_TRUE or MI_FALSE.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteVerbose(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* message)
{
    if (context && context->ft)
    {
        return context->ft->WriteMessage(context, MI_WRITEMESSAGE_CHANNEL_VERBOSE, message);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function implements the WriteDebug functionality of WriteMessage CIM Extensions operation.
 *
 * param: context the request context.
 * param: message
 *
 * return: MI_TRUE or MI_FALSE.
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_Context_WriteDebug(
    _In_ MI_Context* context,
    _In_z_ const MI_Char* message)
{
    if (context && context->ft)
    {
        return context->ft->WriteMessage(context, MI_WRITEMESSAGE_CHANNEL_DEBUG, message);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}




/*
**==============================================================================
**
**  MI_InstanceOf
**      converts pointer to concrete instance to pointer to MI_Instance
**
**==============================================================================
*/
#define MI_InstanceOf(inst)  (&(inst)->__instance)



/*
**==============================================================================
**
**  Undo eight-byte packing for structures (on Windows)
**
**==============================================================================
*/

# pragma pack(pop)

#endif /* _MI_h */

/*============================================================================
 * Copyright (C) Microsoft Corporation, All rights reserved.
 *============================================================================
 */

/*
**=============================================================================
**
** CIM Client Management Interface (MI) APIs
**
**=============================================================================
*/
#ifndef __MI_C_API_H
#define __MI_C_API_H

#ifndef MI_CALL_VERSION
/* Always default to version 1 of API */
#define MI_CALL_VERSION 1
#endif

#if (MI_CALL_VERSION > 1)
#error "Unsupported version of MI_CALL_VERSION.  This SDK only supports version 1."
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef MI_Module* (MI_MAIN_CALL *MI_MainFunction)(MI_Server* server);

typedef struct _MI_QualifierSet MI_QualifierSet;

/* make sure ordering of methods and arguments is consistent with other function tables */

typedef struct _MI_QualifierSetFT
{
    MI_Result (MI_CALL *GetQualifierCount)(
        _In_ const MI_QualifierSet *self,
        _Out_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetQualifierAt)(
        _In_ const MI_QualifierSet *self,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **name,
        _Out_ MI_Type *qualifierType,
        _Out_ MI_Uint32 *qualifierFlags,    /* scope information */
        _Out_ MI_Value *qualifierValue
        );

    MI_Result (MI_CALL *GetQualifier)(
        _In_ const MI_QualifierSet *self,
        _In_z_ const MI_Char *name,
        _Out_ MI_Type *qualifierType,
        _Out_ MI_Uint32 *qualifierFlags,    /* scope information */
        _Out_ MI_Value *qualifierValue,
        _Out_ MI_Uint32 *index
        );
} MI_QualifierSetFT;

struct _MI_QualifierSet
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;

    const MI_QualifierSetFT *ft;
};

typedef struct _MI_ParameterSet MI_ParameterSet;

typedef struct _MI_ParameterSetFT
{
    MI_Result (MI_CALL *GetMethodReturnType)(
        _In_  const MI_ParameterSet *self,
        _Out_ MI_Type *returnType,
        _Out_ MI_QualifierSet *qualifierSet);

    MI_Result (MI_CALL *GetParameterCount)(
        _In_ const MI_ParameterSet *self,
        _Out_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetParameterAt)(
        _In_ const MI_ParameterSet *self,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **name,
        MI_Type *parameterType,
        _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
        _Out_ MI_QualifierSet *qualifierSet);

    MI_Result (MI_CALL *GetParameter)(
        _In_ const MI_ParameterSet *self,
        _In_z_ const MI_Char *name,
        _Out_ MI_Type *parameterType,
        _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
        _Out_ MI_QualifierSet *qualifierSet,
        _Out_ MI_Uint32 *index);

} MI_ParameterSetFT;

struct _MI_ParameterSet
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;

    const MI_ParameterSetFT * ft;
};

typedef struct _MI_ClassFT
{
    MI_Result (MI_CALL *GetClassName)(
        _In_ const MI_Class* self,
        _Outptr_result_maybenull_z_ const MI_Char** className);

    MI_Result (MI_CALL *GetNameSpace)(
        _In_ const MI_Class* self,
        _Outptr_result_maybenull_z_ const MI_Char** nameSpace);

    MI_Result (MI_CALL *GetServerName)(
        _In_ const MI_Class* self,
        _Outptr_result_maybenull_z_ const MI_Char** serverName);

    MI_Result (MI_CALL *GetElementCount)(
        _In_ const MI_Class* self,
        _Out_ MI_Uint32* count);

    MI_Result (MI_CALL *GetElement)(
        _In_      const MI_Class* self,
        _In_z_    const MI_Char* name,
        _Out_opt_ MI_Value* value,
        _Out_opt_ MI_Boolean* valueExists,
        _Out_opt_ MI_Type* type,
        _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_Uint32* flags,
        _Out_opt_ MI_Uint32* index);

    MI_Result (MI_CALL *GetElementAt)(
        _In_ const MI_Class* self,
        MI_Uint32 index,
        _Outptr_opt_result_maybenull_z_ const MI_Char** name,
        _Out_opt_ MI_Value* value,
        _Out_opt_ MI_Boolean* valueExists,
        _Out_opt_ MI_Type* type,
        _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_Uint32* flags);

    MI_Result (MI_CALL *GetClassQualifierSet)(
        _In_ const MI_Class* self,
        _Out_opt_ MI_QualifierSet *qualifierSet
        );

    MI_Result (MI_CALL *GetMethodCount)(
        _In_ const MI_Class* self,
        _Out_ MI_Uint32* count);

    MI_Result (MI_CALL *GetMethodAt)(
        _In_ const MI_Class *self,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **name,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_ParameterSet *parameterSet
        );

    MI_Result (MI_CALL *GetMethod)(
        _In_ const MI_Class *self,
        _In_z_ const MI_Char *name,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_ParameterSet *parameterSet,
        _Out_opt_ MI_Uint32 *index
        );

    MI_Result (MI_CALL *GetParentClassName)(
        _In_ const MI_Class *self,
        _Outptr_result_maybenull_z_ const MI_Char **name);

    MI_Result (MI_CALL *GetParentClass)(
        _In_ const MI_Class *self,
        _Outptr_ MI_Class **parentClass);

    MI_Result (MI_CALL *Delete)(
        _Inout_ MI_Class* self);

    MI_Result (MI_CALL *Clone)(
        _In_ const MI_Class* self,
        _Outptr_ MI_Class** newClass);
} MI_ClassFT;

struct _MI_Class
{
    const MI_ClassFT *ft;

    MI_CONST MI_ClassDecl *classDecl;
    MI_CONST MI_Char *namespaceName;
    MI_CONST MI_Char *serverName;

    ptrdiff_t reserved[4];
};

/*
**=============================================================================
**
** typedef struct _MI_Application MI_Application
**
** The application needs to initialize the MI infrastructure.  This handle
** represents the initialized infrastructure and must be closed before
** application exit.
** The application object represents a collection of any number of remote
** sessions.
** It is expected that a single client object is created per management
** application, whereby it is created at startup and closed on shutdown.
** Having one may reduce the amount of memory used by the management
** infrastructure.
**
** This handle is created through a call to MI_Application_Initialize
** and must be closed through MI_Application_Close.
**
** See MI_Application_* functions for operations on this handle.
**
** An application must initialize a variable of this type with
** MI_APPLICATION_NULL.
**
** Members
**
**      ft          -   This is the function table for accessing the
**                      management infrastructure.  It also holds the
**                      application shutdown function.
**                      See _MI_Application for actual methods available.
**
**      reserved    -   Used internally and must not be changed.
**=============================================================================
*/
typedef struct _MI_Application MI_Application;

/*
**=============================================================================
**
** typedef  struct _MI_Session MI_Session
**
** The session object represents a destination, and any configuration
** associated with the destination.  The creation of a session generally does
** not talk to the server.  A session can have multiple operations
** running in parallel.  There is as much transport connection pooling and reuse
** as is possible, so two sequential operations will try and share the same
** connection if that makes sense for the underlying transport.  New connections
** will be made if necessary to run two operations in parallel.  This object
** holds the internal function tables for carrying out actions on the operation.
**
** This handle is created through a call to MI_Application_NewSession
** and must be closed through MI_Session_Close.
**
** See MI_Session_* functions for operations on this handle.
**
** An application must initialize a variable of this type with
** MI_SESSION_NULL.
**
** Members
**
**      ft          -   This is the function table for accessing carrying out
**                      operations on a destination machine, along with
**                      configuration of the session.  It also has the
**                      session shutdown function.
**                      See _MI_SessionFT for actual methods available.
**
**      reserved    -   Used internally and must not be changed.
**=============================================================================
*/
typedef struct _MI_Session MI_Session;

/*
**=============================================================================
**
** typedef struct _MI_Operation MI_Operation
**
** The operation object represents a single operations execution.  This object
** holds the internal function tables for carrying out actions on the
** operation.
**
** This handle is created through a call to one of the MI_Session_* operation
** functions and must be closed through MI_Operation_Close.  An operation
** can be cancelled through a call to MI_Operation_Cancel.
**
** See MI_Operation_* functions for operations on this handle.
**
** An application must initialize a variable of this type with
** MI_OPERATON_NULL.
**
** Members
**
**      ft          -   This is the function table for accessing results
**                      from the request (if synchronous), and operation
**                      cancellation and shutdown.
**
**      reserved    -   Used internally and must not be changed.
**=============================================================================
*/
typedef struct _MI_Operation MI_Operation;

/*
**=============================================================================
**
** struct _MI_HostedProvider
**
** The object represents the registration of a hosted provider with the
** server.
**
** This handle is created through a call to MI_Application_NewHostedProvider
** and must be closed through MI_HostedProvider_Close.
**
** See MI_HostedProvider_* functions for operations on this handle.
**
** An application must initialize a variable of this type with
** MI_DECOUPLEDPROVIDER_NULL.
**
** Members
**
**      ft          -   This is the function table for unregistering the
**                      hosted provder from the server.
**
**      reserved    -   Used internally and must not be changed.
**=============================================================================
*/
typedef struct _MI_HostedProvider MI_HostedProvider;

/*
**=============================================================================
**
** typedef const struct _MI_DestinationOptions MI_DestinationOptions
**
** The object represents a set of destionation options.  The options can be
** used on a session or for discovering destination capabilities.  The object
** can be used multiple times is required.
**
** This handle is created through a call to MI_Application_NewDestinationOptions
** and must be closed through MI_DestinationOptions_Delete.
**
** See MI_DestinationOptions_* functions for operations on this handle.
**
** An application must initialize a variable of this type with
** MI_DESTINATIONOPTIONS_NULL.

**** Members
**
**      ft          -   This is the function table for setting the options.
**
**      reserved    -   Used internally and must not be changed.
**=============================================================================
*/
typedef struct _MI_DestinationOptions MI_DestinationOptions;

/*
**=============================================================================
**
** typedef struct _MI_OperationOptions MI_OperationOptions
**
** The object represents a set of operation options.  The options can be
** used on operations.  The object can be used multiple times is required.
** Some options are overrides of those set in the destination options.
**
** This handle is created through a call to MI_Application_NewOperationOptions
** and must be closed through MI_OperationOptions_Delete.
**
** See MI_OperationOptions_* functions for operations on this handle.
**
** An application must initialize a variable of this type with
** MI_OPERATIONOPTIONS_NULL.
**
** Members
**
**      ft          -   This is the function table for setting the options.
**
**      reserved    -   Used internally and must not be changed.
**=============================================================================
*/
typedef struct _MI_OperationOptions MI_OperationOptions;

/*
**=============================================================================
**
** (_MI_OperationCallback_ResponseType)
**
**
**=============================================================================
*/

typedef enum _MI_OperationCallback_ResponseType
{
    MI_OperationCallback_ResponseType_No,
    MI_OperationCallback_ResponseType_Yes,
    MI_OperationCallback_ResponseType_NoToAll,
    MI_OperationCallback_ResponseType_YesToAll

} MI_OperationCallback_ResponseType;


/*
**=============================================================================
**
** (*MI_OperationCallback_PromptUser)()
**
** CIM Extension callback ask the client if the operation should process the
** request.  The callback calls the promptUserResult() method to return the
** response, either from the current thread or from a different one.
** MI_OperationCallback_ResponseType_Yes tells the operation to continue,
** MI_OperationCallback_ResponseType_No tells the operation to abort. If
** CallbackMode for MI_OperationCallback_PromptUser is
** MI_CALLBACKMODE_REPORT promptUserResult() is NULL.
** If promptUserResult() is not NULL application must call this method
** otherwise the request will not progress.
** All parameters are valid until the call into promptUserResult().
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_PromptUser)(
    _In_     MI_Operation *operation,
    _In_opt_ void *callbackContext,
    _In_z_   const MI_Char *message,
             MI_PromptType promptType,
    _In_opt_ MI_Result (MI_CALL * promptUserResult)(_In_ MI_Operation *operation,
                                                      MI_OperationCallback_ResponseType response));

/*
**=============================================================================
**
** (*MI_OperationCallback_WriteError)()
**
** CIM Extension callback reports an error and ask the user if the operation
** should continue.

** The callback calls the writeErrorResult() method to return the response,
** wither from the current thread or from a different one.  MI_TRUE tells the
** operation to continue, MI_FALSE tells the operation to abort. The
** application must call this method otherwise the request will not progress.
** All parameters are valid until the call into writeErrorResult().
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_WriteError)(
    _In_     MI_Operation *operation,
    _In_opt_ void *callbackContext,
    _In_ MI_Instance*instance,
    _In_opt_ MI_Result (MI_CALL * writeErrorResult)(_In_ MI_Operation *operation,
                                                    MI_OperationCallback_ResponseType response));


/*
**=============================================================================
**
** (*MI_OperationCallback_WriteMessage)()
**
** CIM Extension callback reports a message back to the client.  The channel
** says if it is warning, error, debug, etc.  This is purely informational and
** it does not effect the operation.
** All parameters are valid for the lifetime of the callback only.
**
** channel: Can be one of the MI_WRITEMESSAGE_CHANNEL_* defines or a custom one
**=============================================================================
*/
#define MI_WRITEMESSAGE_CHANNEL_WARNING 0
#define MI_WRITEMESSAGE_CHANNEL_VERBOSE 1
#define MI_WRITEMESSAGE_CHANNEL_DEBUG   2

typedef void (MI_CALL *MI_OperationCallback_WriteMessage)(
    _In_     MI_Operation *operation,
    _In_opt_ void *callbackContext,
             MI_Uint32 channel,
    _In_z_   const MI_Char *message);
/*
**=============================================================================
**
** (*MI_OperationCallback_WriteProgress)()
**
** CIM Extension callback indicates the progress of an operation.  This is
** informational and does not effect the operation.
** All parameters are valid for the lifetime of the callback only.
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_WriteProgress)(
    _In_     MI_Operation *operation,
    _In_opt_ void *callbackContext,
    _In_z_   const MI_Char *activity,
    _In_z_   const MI_Char *currentOperation,
    _In_z_   const MI_Char *statusDescription,
             MI_Uint32 percentageComplete,
             MI_Uint32 secondsRemaining);

/*
**=============================================================================
**
** (*MI_OperationCallback_Instance)()
**
** Registering for this callback will cause asynchronous notification of this
** method to be called for instance operation results for Get, Modify,
** Create, Delete, Enumeration and Invoke operations.  For Enumeration this
** callback will be called once for each available result.
**
** Application must call the resultAcknowledgement callback when they are
** done with the instance.  Not doing so will stop the operation from,
** completing and will cause enumerations to not progress.
**
** Not calling the resultAcknowledgement callback will stop the CloseOperaton
** from completing.
**
** For method invocations, the instance will be a property bag for each
** of the out parameters of the method.
** All parameters are valid until the call into resultAcknowledgement().
**
** Return
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_Instance)(
    _In_opt_     MI_Operation *operation,
    _In_     void *callbackContext,
    _In_opt_ const MI_Instance *instance,
             MI_Boolean moreResults,
    _In_     MI_Result resultCode,
    _In_opt_z_ const MI_Char *errorString,
    _In_opt_ const MI_Instance *errorDetails,
    _In_opt_ MI_Result (MI_CALL * resultAcknowledgement)(_In_ MI_Operation *operation));


/*
**=============================================================================
**
** (*MI_OperationCallback_StreamedParameter)()
**
** Registering this async callback is necessary if an outbound paramter is
** marked as streamed.  This callback will be called as streamed parameter data
** is available.  Streaming can only happen on array parameters.  Call the
** resultAcknowledgement to acknowledge the result.  Not doing so will stop
** more data from arriving and will stop the operation from completing.  It
** will also stop the operation from closing.
** All parameters are valid until the call into resultAcknowledgement().
**
** Return
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_StreamedParameter)(
    _In_ MI_Operation *operation,
    _In_ void *callbackContext,
    _In_z_ const MI_Char *parameterName,
    _In_ MI_Type resultType,
    _In_ const MI_Value *result,
    _In_opt_ MI_Result (MI_CALL * resultAcknowledgement)(_In_ MI_Operation *operation));

/*
**=============================================================================
**
** (*MI_OperationCallback_Indication)()
**
** Registering for this callback will cause asynchronous notification of this
** method to be called when results of a subscription are delivered.  Call
** resultAcknowledgement when done with the indication.  Not doing so will
** result in no more results being received and will stop the subscription from
** shutting down.
** All parameters are valid until the call into resultAcknowledgement().
**
** Return
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_Indication)(
    _In_opt_     MI_Operation *operation,
    _In_     void *callbackContext,
    _In_opt_ const MI_Instance *instance,
    _In_opt_z_ const MI_Char *bookmark,
    _In_opt_z_ const MI_Char *machineID,
             MI_Boolean moreResults,
    _In_     MI_Result resultCode,
    _In_opt_z_ const MI_Char *errorString,
    _In_opt_ const MI_Instance *errorDetails,
    _In_opt_ MI_Result (MI_CALL * resultAcknowledgement)(_In_ MI_Operation *operation));

/*
**=============================================================================
**
** (*MI_OperationCallback_Class)()
**
** Registering for this callback will cause asynchronous notification  when
** results of a class operations are completed.  For enumerations this callback
** is called for each result.  Call resultAcknowledgement when done with the
** class.  Not doing so will result in no more results being received and will
** stop the operation from shutting down.
** All parameters are valid until the call into resultAcknowledgement().
**
** Return
**
**=============================================================================
*/
typedef void (MI_CALL *MI_OperationCallback_Class)(
    _In_opt_     MI_Operation *operation,
    _In_     void *callbackContext,
    _In_opt_ const MI_Class *classResult,
             MI_Boolean moreResults,
    _In_     MI_Result resultCode,
    _In_opt_z_ const MI_Char *errorString,
    _In_opt_ const MI_Instance *errorDetails,
    _In_opt_ MI_Result (MI_CALL * resultAcknowledgement)(_In_ MI_Operation *operation));

/*
**=============================================================================
**
** typedef MI_OperationCallbacks
**
** Structure that holds all callback function pointers.  Fill in the ones
** you want to receive.  If the associated operation callback for the operation
** is not set the operation will be carried out synchronously.  All CIM
** extension and streamed result callbacks are optional.  The callbackContext
** is application specific data that is passed back in the callback so the
** application can correlate the callbacks with the request.
**
**=============================================================================
*/
typedef struct _MI_OperationCallbacks
{
    /* User callback context */
    void *callbackContext;

    /* CIM Extension callbacks */
    MI_OperationCallback_PromptUser promptUser;
    MI_OperationCallback_WriteError writeError;
    MI_OperationCallback_WriteMessage writeMessage;
    MI_OperationCallback_WriteProgress writeProgress;

    /* Result callbacks */
    MI_OperationCallback_Instance instanceResult;
    MI_OperationCallback_Indication indicationResult;
    MI_OperationCallback_Class classResult;

    /* Invoke streamed outbound parameter result callback */
    MI_OperationCallback_StreamedParameter streamedParameterResult;
}
MI_OperationCallbacks;


/*
**=============================================================================
**
** #define MI_OPERATIONCALLBACKS_NULL
**
** Initializer for the MI_OperationCallbacks structure
**=============================================================================
*/
#define MI_OPERATIONCALLBACKS_NULL {NULL}

/*
**=============================================================================
**
** typedef MI_SessionCallbacks
**
** Structure that holds all callback function pointers.  Fill in the ones
** you want to receive.  All callbacks are CIM extensions for tracking
** logging and error messages.
**
**=============================================================================
*/
typedef struct _MI_SessionCallbacks
{
    /* User callback context that is passed into callback */
    void *callbackContext;

    /*=========================================================================
    ** CIM Extension callback for recieving logging from the session creation.
    ** All parameters are valid only for the lifetime of the callback.
    **=========================================================================
    */
    void (MI_CALL *writeMessage)(
        _In_     MI_Application *application,
        _In_opt_ void *callbackContext,
                 MI_Uint32 channel,
        _In_z_   const MI_Char * message);

    /*=========================================================================
    ** CIM Extension callback for errors. The session version of this API is
    ** information only.  The session will fail to create and will return an
    ** error.  All parameters are valid only for the lifetime of the callback.
    **=========================================================================
    */
    void (MI_CALL *writeError)(
        _In_     MI_Application *application,
        _In_opt_ void *callbackContext,
        _In_ MI_Instance *instance);
}
MI_SessionCallbacks;

#define MI_SESSIONCALLBACKS_NULL { NULL }


/* Flags to be passed into Session operation functions.
 * One item from each group can be bit-wise combined.
 */
/*#define MI_OPERATIONFLAGS_AUTOMATIC_ACK_RESULTS            default, 0x0000 */
#define MI_OPERATIONFLAGS_MANUAL_ACK_RESULTS                 0x0001

/* RTTI bitmasks in order to specify what options are acceptable to the client */
#define MI_OPERATIONFLAGS_NO_RTTI                            0x0400 /* All instance elements are string except embedded objects and references, but their elements will be strings also */
#define MI_OPERATIONFLAGS_BASIC_RTTI                         0x0002 /* All instance elements may be strings or the correct type */
#define MI_OPERATIONFLAGS_STANDARD_RTTI                      0x0800 /* All instance elements are of the correct type, but some hierarchy information may be lost due to optimizations */
#define MI_OPERATIONFLAGS_FULL_RTTI                          0x0004 /* All instance elements at every level of the instances class hierarchy will be accurate.  This may be very expensive */

/* If no RTTI flag is specified (i.e. 0 for relevant RTTI bits) then the protocol handler itself will pick the best option */
#define MI_OPERATIONFLAGS_DEFAULT_RTTI                       0

/*#define MI_OPERATIONFLAGS_NON_LOCALIZED_QUALIFIERS         default, 0x0000 */
#define MI_OPERATIONFLAGS_LOCALIZED_QUALIFIERS               0x0008

/*#define MI_OPERATIONFLAGS_NON_EXPENSIVE_PROPERTIES_ONLY    default, 0x0040 */
#define MI_OPERATIONFLAGS_EXPENSIVE_PROPERTIES               0x0040

/*#define MI_OPERATIONFLAGS_POLYMORPHISM_DEEP                default, 0x0000 */
#define MI_OPERATIONFLAGS_POLYMORPHISM_SHALLOW               0x0080
#define MI_OPERATIONFLAGS_POLYMORPHISM_DEEP_BASE_PROPS_ONLY  0x0180

/* Report an empty result when the operation has successfully started.
 * The first result may have a NULL instance/class/indication, with
 * moreResults set to MI_TRUE.  If a result is delivered very quickly
 * the actual result will be delivered instead.
 * Not all operations or protocol handlers can achieve this.
 */
#define MI_OPERATIONFLAGS_REPORT_OPERATION_STARTED           0x0200

/*
**=============================================================================
**
** #define MI_AUTH_TYPE_*
**
** Set of defines for different types of supported authentication.  Note that
** not all protocols and transports support this list, and some protocols and
** transports may support others.
**
**=============================================================================
*/
#define MI_AUTH_TYPE_DEFAULT  MI_T("Default") /* transport picks default specific to it. Ex: winrm uses Kerberos and NegotiateWithoutCredentials as default*/
#define MI_AUTH_TYPE_NONE MI_T("None")  /* do not authenticate, most servers require some kind of authentication */
#define MI_AUTH_TYPE_DIGEST MI_T("Digest") /* Needs username/password */
#define MI_AUTH_TYPE_NEGO_WITH_CREDS MI_T("NegoWithCreds") /* needs username/password */
#define MI_AUTH_TYPE_NEGO_NO_CREDS MI_T("NegoNoCreds") /* current thread user, need to trust destination! */
#define MI_AUTH_TYPE_BASIC MI_T("Basic") /* needs username/password */
#define MI_AUTH_TYPE_KERBEROS MI_T("Kerberos") /* username/password optional */
#define MI_AUTH_TYPE_CLIENT_CERTS MI_T("ClientCerts") /* needs cert thumbprint */
#define MI_AUTH_TYPE_NTLM MI_T("Ntlmdomain") /* username/password optional */
#if (WINVER >= 0x600)
#define MI_AUTH_TYPE_CREDSSP MI_T("CredSSP") /* username/password optional */
#endif
#define MI_AUTH_TYPE_ISSUER_CERT MI_T("IssuerCert") /* Push/Source Initiated subscriptions only */

/*
**=============================================================================
**
** typedef MI_UsernamePasswordCreds
**
** A username/password combination used for subscription operations.
**
**=============================================================================
*/
typedef struct _MI_UsernamePasswordCreds
{
    const MI_Char *domain;
    const MI_Char *username;
    const MI_Char *password;
}
MI_UsernamePasswordCreds;

/*
**=============================================================================
**
** typedef MI_UserCredentials
**
** A user credential.  It includes an authentication type and either a username
** and password or a certificate thumbprint, depending on the authentication
** type.
**
**=============================================================================
*/
typedef struct _MI_UserCredentials
{
    const MI_Char *authenticationType; /* MI_AUTH_TYPE_... values */
    union
    {
        MI_UsernamePasswordCreds usernamePassword;
        const MI_Char *certificateThumbprint;
    } credentials;
}
MI_UserCredentials;

/*
**=============================================================================
**
** enum MI_SubscriptionDeliveryType
**
** Subsciption type.
**
** A Pull subscription polls the destination for indications.  If the
** subscription can get through the firewall of the destination machine then
** pulling events from the machine is more lifely to work.
**
** Push subscriptions has the destination machine push the indication to the
** client machine.  This is more efficient as it does not need to keep a
** connection to the destination open, however the firewall on the client
** machine also needs to be opened up as well as on the destination machine.
** The security around this type of subscription is a little more difficult
** to get working than Pull.
**
** Source initiated subscriptions is when something starts the subscription
** on a destination machine through some other means and starts pushing the
** events to this machine.  This subscription can collect events from a
** large number of sources if required, and is configurable based on the
** clients needs.
**=============================================================================
*/
typedef enum _MI_SubscriptionDeliveryType
{
    MI_SubscriptionDeliveryType_Pull = 1,
    MI_SubscriptionDeliveryType_Push = 2,
}
MI_SubscriptionDeliveryType;

typedef struct _MI_SubscriptionDeliveryOptions MI_SubscriptionDeliveryOptions;

typedef struct _MI_SubscriptionDeliveryOptionsFT
{
    MI_Result (MI_CALL *SetString)(
        _Inout_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_z_ const MI_Char *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *SetNumber)(
        _Inout_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   MI_Uint32 value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *SetDateTime)(
        _Inout_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   const MI_Datetime *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *SetInterval)(
        _Inout_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   const MI_Interval *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *AddCredentials)(
        _Inout_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   const MI_UserCredentials *credentials,
               MI_Uint32 flags);

    MI_Result (MI_CALL *Delete)(
        _Inout_ MI_SubscriptionDeliveryOptions* self);

    MI_Result (MI_CALL *GetString)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _Outptr_result_z_ const MI_Char **value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetNumber)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Uint32 *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetDateTime)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Datetime *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetInterval)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Interval *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetOptionCount)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _Out_opt_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetOptionAt)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_ MI_Value *value,
        _Out_ MI_Type *type,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetOption)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Value *value,
        _Out_ MI_Type *type,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetCredentialsCount)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        _Out_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetCredentialsAt)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_ MI_UserCredentials *credentials, // output credentials always has password set to '******'
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetCredentialsPasswordAt)(
        _In_   MI_SubscriptionDeliveryOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_writes_to_opt_(bufferLength, *passwordLength) MI_Char *password,
        _In_ MI_Uint32 bufferLength,
        _Out_ MI_Uint32 *passwordLength,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *Clone)(
        _In_ const MI_SubscriptionDeliveryOptions* self,
        _Out_ MI_SubscriptionDeliveryOptions* newSubscriptionDeliveryOptions);

} MI_SubscriptionDeliveryOptionsFT;
/*
**=============================================================================
**
** typedef MI_SubscriptionDeliveryOptions
**
** Subscription configuration options.
**
**=============================================================================
*/
typedef struct _MI_SubscriptionDeliveryOptions
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;

    const MI_SubscriptionDeliveryOptionsFT * ft;
}
MI_SubscriptionDeliveryOptions;

#define MI_SUBSCRIPTIONDELIVERYOPTIONS_NULL { 0, 0, NULL }

typedef struct _MI_Serializer MI_Serializer;
typedef struct _MI_SerializerFT MI_SerializerFT;
typedef struct _MI_Deserializer MI_Deserializer;
typedef struct _MI_DeserializerFT MI_DeserializerFT;

struct _MI_Serializer
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
} ;

struct _MI_Deserializer
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
} ;

struct _MI_SerializerFT
{
    MI_Result (MI_CALL *Close)(
        _Inout_ MI_Serializer *serializer);

    MI_Result (MI_CALL *SerializeClass)(
        _Inout_ MI_Serializer *serializer,
        MI_Uint32 flags,
        _In_ const MI_Class *classObject,
        _Out_writes_bytes_(clientBufferLength) MI_Uint8 *clientBuffer,
        MI_Uint32 clientBufferLength,
        _Inout_ MI_Uint32 *clientBufferNeeded);

    MI_Result (MI_CALL *SerializeInstance)(
        _Inout_ MI_Serializer *serializer,
        MI_Uint32 flags,
        _In_ const MI_Instance *instanceObject,
        _Out_writes_bytes_(clientBufferLength) MI_Uint8 *clientBuffer,
        MI_Uint32 clientBufferLength,
        _Inout_ MI_Uint32 *clientBufferNeeded);
};

/*
**=============================================================================
**
** typedef MI_Deserializer_ClassObjectNeeded
**
** Callback function used to provider requested class object during deserialization.
**
**=============================================================================
*/
typedef MI_Result (MI_CALL *MI_Deserializer_ClassObjectNeeded)(
                _In_opt_ void *context,
                _In_opt_z_ const MI_Char *serverName,
                _In_opt_z_ const MI_Char *namespaceName,
                _In_z_ const MI_Char *className,
                _Outptr_ MI_Class **requestedClassObject);

/*
**=============================================================================
**
** MI_DeserializerFT
**
** Deserialier function table.
**
**=============================================================================
*/
struct _MI_DeserializerFT
{
    MI_Result (MI_CALL *Close)(
        _Inout_ MI_Deserializer *deserializer);

    MI_Result (MI_CALL *DeserializeClass)(
        _Inout_   MI_Deserializer *deserializer,
        MI_Uint32 flags,
        _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
        MI_Uint32 serializedBufferLength,
        _In_opt_ MI_Class *parentClass,
        _In_opt_z_ const MI_Char *serverName,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_ MI_Deserializer_ClassObjectNeeded classObjectNeeded,
        _In_opt_ void *classObjectNeededContext,
        _Out_opt_ MI_Uint32 *serializedBufferRead,
        _Outptr_opt_result_maybenull_ MI_Class **classObject,
        _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails);

    MI_Result (MI_CALL *Class_GetClassName)(
        _Inout_ MI_Deserializer *deserializer,
        _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
        MI_Uint32 serializedBufferLength,
        _Out_writes_opt_(*classNameLength) MI_Char *className,
        _Inout_ MI_Uint32 *classNameLength,
        _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails);

    MI_Result (MI_CALL *Class_GetParentClassName)(
        _Inout_ MI_Deserializer *deserializer,
        _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
        MI_Uint32 serializedBufferLength,
        _Out_writes_opt_(*parentClassNameLength) MI_Char *parentClassName,
        _Inout_ MI_Uint32 *parentClassNameLength,
        _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails);

    MI_Result (MI_CALL *DeserializeInstance)(
        _Inout_ MI_Deserializer *deserializer,
        MI_Uint32 flags,
        _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
        MI_Uint32 serializedBufferLength,
        _In_reads_opt_(numberClassObjects) MI_Class **classObjects,
        MI_Uint32 numberClassObjects,
        _In_opt_ MI_Deserializer_ClassObjectNeeded classObjectNeeded,
        _In_opt_ void *classObjectNeededContext,
        _Out_opt_ MI_Uint32 *serializedBufferRead,
        _Outptr_opt_result_maybenull_ MI_Instance **instanceObject,
        _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails);

    MI_Result (MI_CALL *Instance_GetClassName)(
        _Inout_ MI_Deserializer *deserializer,
        _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
        MI_Uint32 serializedBufferLength,
        _Out_writes_opt_(*classNameLength) MI_Char *className,
        _Inout_ MI_Uint32 *classNameLength,
        _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails);

};

/*
**=============================================================================
**
** typedef  _MI_ApplicationFT MI_ApplicationFT
**
** Function table for all actions on a application object.
**
** See MI_Application_<method name> functions for details.
**
**=============================================================================
*/
typedef struct _MI_ApplicationFT
{
    MI_Result (MI_CALL *Close)(
        _Inout_     MI_Application *application);

    MI_Result (MI_CALL *NewSession)(
        _In_     MI_Application *application,
        _In_opt_z_ const MI_Char *protocol,
        _In_opt_z_ const MI_Char *destination,
        _In_opt_ MI_DestinationOptions *options,
        _In_opt_ MI_SessionCallbacks *callbacks,
        _Outptr_opt_result_maybenull_ MI_Instance **extendedError,
        _Out_    MI_Session *session);

    MI_Result (MI_CALL *NewHostedProvider)(
        _In_     MI_Application *application,
        _In_z_     const MI_Char *namespaceName,
        _In_z_     const MI_Char *providerName,
        _In_     MI_MainFunction mi_Main,
        _Outptr_opt_result_maybenull_ MI_Instance **extendedError,
        _Out_    MI_HostedProvider *provider
        );

    MI_Result (MI_CALL *NewInstance)(
        _In_     MI_Application *application,
        _In_z_ const MI_Char *className,
        _In_opt_ const MI_ClassDecl *classRTTI,
        _Outptr_    MI_Instance **instance);

    MI_Result (MI_CALL *NewDestinationOptions)(
        _In_     MI_Application *application,
        _Out_    MI_DestinationOptions *options);

    MI_Result (MI_CALL *NewOperationOptions)(
        _In_     MI_Application *application,
        _In_     MI_Boolean customOptionsMustUnderstand,
        _Out_    MI_OperationOptions *options);

    MI_Result (MI_CALL *NewSubscriptionDeliveryOptions)(
        _In_     MI_Application *application,
        _In_     MI_SubscriptionDeliveryType deliveryType,
        _Out_    MI_SubscriptionDeliveryOptions *deliveryOptions);

    MI_Result (MI_CALL *NewSerializer)(
        _Inout_ MI_Application *application,
        MI_Uint32 flags,
        _In_z_ MI_Char *format,
        _Out_ MI_Serializer *serializer);

    MI_Result (MI_CALL *NewDeserializer)(
        _Inout_ MI_Application *application,
        MI_Uint32 flags,
        _In_z_ MI_Char *format,
        _Out_ MI_Deserializer *deserializer);

    MI_Result (MI_CALL *NewInstanceFromClass)(
        _In_     MI_Application *application,
        _In_z_ const MI_Char *className,
        _In_opt_ const MI_Class *classObject,
        _Outptr_    MI_Instance **instance);

    MI_Result (MI_CALL *NewClass) (
        _In_ MI_Application *application,
        _In_ const MI_ClassDecl* classDecl,
        __in_z_opt const MI_Char *namespaceName,
        __in_z_opt const MI_Char *serverName,
        _Outptr_ MI_Class** classObject);

}
MI_ApplicationFT;

/*
**=============================================================================
**
** typedef const _MI_HostedProviderFT MI_HostedProviderFT
**
** Function table for all actions on a hosted provider object.
**
** See MI_HostedProvider_<method name> functions for details.
**
**=============================================================================
*/
typedef struct _MI_HostedProviderFT
{
    MI_Result (MI_CALL *Close)(
        _Inout_  MI_HostedProvider *hostedProvider);

    MI_Result (MI_CALL *GetApplication)(
        _In_  MI_HostedProvider *hostedProvider,
        _Out_ MI_Application *application);
}
MI_HostedProviderFT;

/*
**=============================================================================
**
** typedef const _MI_SessionFT MI_SessionFT
**
** Function table for all actions on a session object.
**
** See MI_Session_<method name> functions for details.
**
**=============================================================================
*/
typedef struct _MI_SessionFT
{
    MI_Result (MI_CALL *Close)(
        _Inout_     MI_Session *session,
        _In_opt_ void *completionContext,
        _In_opt_ void (MI_CALL *completionCallback)(_In_opt_ void *completionContext));

    MI_Result (MI_CALL *GetApplication)(
        _In_     MI_Session *session,
        _Out_    MI_Application *application);

    void (MI_CALL *GetInstance)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_     const MI_Instance *inboundInstance,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *ModifyInstance)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_     const MI_Instance *inboundInstance,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *CreateInstance)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_     const MI_Instance *inboundInstance,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *DeleteInstance)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_     const MI_Instance *inboundInstance,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *Invoke)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_z_ const MI_Char *className,
        _In_z_     const MI_Char *methodName,
        _In_opt_ const MI_Instance *inboundInstance,
        _In_opt_ const MI_Instance *inboundProperties,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *EnumerateInstances)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_z_ const MI_Char *className,
                 MI_Boolean keysOnly,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *QueryInstances)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_z_ const MI_Char *queryDialect,
        _In_opt_z_ const MI_Char *queryExpression,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *AssociatorInstances)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_     const MI_Instance *instanceKeys,
        _In_opt_z_ const MI_Char *assocClass,
        _In_opt_z_ const MI_Char *resultClass,
        _In_opt_z_ const MI_Char *role,
        _In_opt_z_ const MI_Char *resultRole,
                 MI_Boolean keysOnly,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *ReferenceInstances)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_     const MI_Instance *instanceKeys,
        _In_opt_z_ const MI_Char *resultClass,
        _In_opt_z_ const MI_Char *role,
                 MI_Boolean keysOnly,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *Subscribe)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_z_ const MI_Char *queryDialect,
        _In_opt_z_ const MI_Char *queryExpression,
        _In_opt_ const MI_SubscriptionDeliveryOptions *deliverOptions,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *GetClass)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_z_ const MI_Char *className,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *EnumerateClasses)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationOptions *options,
        _In_opt_z_ const MI_Char *namespaceName,
        _In_opt_z_ const MI_Char *className,
                 MI_Boolean classNamesOnly,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation);

    void (MI_CALL *TestConnection)(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation
        );
}
MI_SessionFT;

/*
**=============================================================================
**
** typedef const _MI_OperationFT MI_OperationFT
**
** Function table for all actions on a operation object.
**
** See MI_Operation_<method name> functions for details.
**
**=============================================================================
*/
typedef struct _MI_OperationFT
{
    MI_Result (MI_CALL *Close)(
        _Inout_      MI_Operation *operation);

    MI_Result (MI_CALL *Cancel)(
        _Inout_      MI_Operation *operation,
                  MI_CancellationReason reason);

    MI_Result (MI_CALL *GetSession)(
        _In_      MI_Operation *operation,
        _Out_     MI_Session *session);

    MI_Result (MI_CALL *GetInstance)(
        _In_      MI_Operation *operation,
        _Outptr_result_maybenull_     const MI_Instance **instance,
        _Out_opt_ MI_Boolean *moreResults,
        _Out_opt_ MI_Result *result,
        _Outptr_opt_result_maybenull_z_ const MI_Char **errorMessage,
        _Outptr_opt_result_maybenull_ const MI_Instance **completionDetails);

    MI_Result (MI_CALL *GetIndication)(
        _In_      MI_Operation *operation,
        _Outptr_result_maybenull_       const MI_Instance **instance,
        _Outptr_opt_result_maybenull_z_ const MI_Char **bookmark,
        _Outptr_opt_result_maybenull_z_ const MI_Char **machineID,
        _Out_opt_ MI_Boolean *moreResults,
        _Out_opt_ MI_Result *result,
        _Outptr_opt_result_maybenull_z_ const MI_Char **errorMessage,
        _Outptr_opt_result_maybenull_   const MI_Instance **completionDetails);

    MI_Result (MI_CALL *GetClass)(
        _In_      MI_Operation *operation,
        _Outptr_result_maybenull_     const MI_Class **classResult,
        _Out_opt_ MI_Boolean *moreResults,
        _Out_opt_ MI_Result *result,
        _Outptr_opt_result_maybenull_z_ const MI_Char **errorMessage,
        _Outptr_opt_result_maybenull_ const MI_Instance **completionDetails);

}
MI_OperationFT;

/*
**=============================================================================
**
** typedef const _MI_DestinationOptionsFT MI_DestinationOptionsFT
**
** Function table for all actions on a destination options object.
**
** See MI_DestinationOptions_<method name> functions for details.
**
**=============================================================================
*/
typedef struct _MI_DestinationOptionsFT
{
    void (MI_CALL *Delete)(
        _Inout_ MI_DestinationOptions *options);

    MI_Result (MI_CALL *SetString)(
        _Inout_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_z_ const MI_Char *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *SetNumber)(
        _Inout_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   MI_Uint32 value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *AddCredentials)(
        _Inout_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   const MI_UserCredentials *credentials,
               MI_Uint32 flags);

    MI_Result (MI_CALL *GetString)(
        _In_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Outptr_result_z_ const MI_Char **value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetNumber)(
        _In_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Uint32 *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetOptionCount)(
        _In_   MI_DestinationOptions *options,
        _Out_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetOptionAt)(
        _In_   MI_DestinationOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_ MI_Value *value,
        _Out_ MI_Type *type,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetOption)(
        _In_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Value *value,
        _Out_ MI_Type *type,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetCredentialsCount)(
        _In_   MI_DestinationOptions *options,
        _Out_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetCredentialsAt)(
        _In_   MI_DestinationOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_ MI_UserCredentials *credentials, // output credentials always has password set to '******'
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetCredentialsPasswordAt)(
        _In_   MI_DestinationOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_writes_to_opt_(bufferLength, *passwordLength) MI_Char *password,
        _In_ MI_Uint32 bufferLength,
        _Out_ MI_Uint32 *passwordLength,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *Clone)(
        _In_ const MI_DestinationOptions* self,
        _Out_ MI_DestinationOptions* newDestinationOptions);

    MI_Result (MI_CALL *SetInterval)(
        _Inout_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   const MI_Interval *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *GetInterval)(
        _In_   MI_DestinationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Interval *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);
}
MI_DestinationOptionsFT;

/*
**=============================================================================
**
** typedef const _MI_OperationOptionsFT MI_OperationOptionsFT
**
** Function table for all actions on a destination options object.
**
** See MI_DestinationOptions_<method name> functions for details.
**
**=============================================================================
*/
typedef struct _MI_OperationOptionsFT
{
    void (MI_CALL *Delete)(
        _Inout_ MI_OperationOptions *options);

    MI_Result (MI_CALL *SetString)(
        _Inout_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_z_ const MI_Char *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *SetNumber)(
        _Inout_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   MI_Uint32 value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *SetCustomOption)(
        _Inout_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   MI_Type valueType,
        _In_   const MI_Value *value,
               MI_Boolean mustComply,
               MI_Uint32 flags);

    MI_Result (MI_CALL *GetString)(
        _In_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Outptr_result_z_ const MI_Char **value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetNumber)(
        _In_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Uint32 *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetOptionCount)(
        _In_   MI_OperationOptions *options,
        _Out_ MI_Uint32 *count);

    MI_Result (MI_CALL *GetOptionAt)(
        _In_   MI_OperationOptions *options,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **optionName,
        _Out_ MI_Value *value,
        _Out_ MI_Type *type,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetOption)(
        _In_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Value *value,
        _Out_ MI_Type *type,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *GetEnabledChannels)(
        _In_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_writes_to_opt_(bufferLength, *channelCount) MI_Uint32 *channels,
        _In_ MI_Uint32 bufferLength,
        _Out_ MI_Uint32 *channelCount,
        _Out_opt_ MI_Uint32 *flags);

    MI_Result (MI_CALL *Clone)(
        _In_ const MI_OperationOptions* self,
        _Out_ MI_OperationOptions* newOperationOptions);


    MI_Result (MI_CALL *SetInterval)(
        _Inout_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _In_   const MI_Interval *value,
               MI_Uint32 flags);

    MI_Result (MI_CALL *GetInterval)(
        _In_   MI_OperationOptions *options,
        _In_z_ const MI_Char *optionName,
        _Out_ MI_Interval *value,
        _Out_opt_ MI_Uint32 *index,
        _Out_opt_ MI_Uint32 *flags);
}
MI_OperationOptionsFT;

/*
**=============================================================================
**
** struct _MI_Application
**
** Note: Use MI_Application typedef instead of this structure.
**
**=============================================================================
*/
struct _MI_Application
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
    const MI_ApplicationFT * ft;
} ;

#define MI_APPLICATION_NULL {  0, 0, NULL }

/*
**=============================================================================
**
** struct _MI_Session
**
** Note: Use MI_Session typedef instead of this structure.
**
**=============================================================================
*/
struct _MI_Session
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
    const MI_SessionFT * ft;
} ;

#define MI_SESSION_NULL { 0, 0, NULL }

/*
**=============================================================================
**
** struct _MI_Operation
**
** Note: Use MI_Operation typedef instead of this structure.
**=============================================================================
*/
struct _MI_Operation
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
    const MI_OperationFT * ft;
};

#define MI_OPERATION_NULL { 0, 0, NULL }

/*
**=============================================================================
**
** struct _MI_HostedProvider
**
** Note: Use MI_HostedProvider typedef instead of this structure.
**=============================================================================
*/
struct _MI_HostedProvider
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
    const MI_HostedProviderFT * ft;
} ;

#define MI_HOSTEDPROVIDER_NULL { 0, 0, NULL }

/*
**=============================================================================
**
** struct _MI_DestinationOptions
**
** Note: Use MI_DestinationOptions typedef instead of this structure.
**=============================================================================
*/
struct _MI_DestinationOptions
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
    const MI_DestinationOptionsFT * ft;
} ;

#define MI_DESTINATIONOPTIONS_NULL { 0, 0, NULL }

/*
**=============================================================================
**
** struct _MI_OperationOptions
**
** Note: Use MI_OperationOptions typedef instead of this structure.
**=============================================================================
*/
struct _MI_OperationOptions
{
    MI_Uint64 reserved1;
    ptrdiff_t reserved2;
    const MI_OperationOptionsFT * ft;
} ;

#define MI_OPERATIONOPTIONS_NULL { 0, 0, NULL }

typedef struct _MI_UtilitiesFT
{
    MI_ErrorCategory (MI_CALL *MapErrorToMiErrorCategory)(
        _In_z_ MI_Char *errorType,
        MI_Uint32 error);

    MI_Result (MI_CALL *CimErrorFromErrorCode)(
        MI_Uint32 error,
        _In_z_ const MI_Char *errorType,
        _In_z_ const MI_Char* errorMessage,
        _Outptr_ MI_Instance **cimError);


} MI_UtilitiesFT;

typedef struct _MI_ClientFT_V1
{
    const MI_ApplicationFT *applicationFT;
    const MI_SessionFT *sessionFT;
    const MI_OperationFT *operationFT;
    const MI_HostedProviderFT *hostedProviderFT;
    const MI_SerializerFT *serializerFT;
    const MI_DeserializerFT *deserializerFT;
    const MI_SubscriptionDeliveryOptionsFT *subscribeDeliveryOptionsFT;
    const MI_DestinationOptionsFT *destinationOptionsFT;
    const MI_OperationOptionsFT *operationOptionsFT;
    const MI_UtilitiesFT *utilitiesFT;
} MI_ClientFT_V1;

#ifndef _MANAGED_PURE
__declspec(dllimport) const MI_ClientFT_V1 *mi_clientFT_V1;
#endif

#if (MI_CALL_VERSION == 1)
#define mi_clientFT mi_clientFT_V1
#endif

/*
**=============================================================================
**
** MI_Application_InitializeV1()
**
** NOTE: Do not call this method directly, instead call through
**       MI_Application_Initialize.  Not doing so can cause API versioning
**       problems.
**
** Initialize the management infrastructure to allow calling into client-side
** management APIs.  Call MI_Application_Close() to shut down the
** management infrastructure, not doing so can cause crashes and memory leaks.
**
** This API is versioned based on the definition of MI_CALL_VERSION.  If this
** is not specified version 1 is assumed.
**
** flags:           Must be 0
** applicationID:   optional application identifier.  This can be a GUID or
**                  in the form company/product/version.  It can be used for
**                  application specific configuration.
** application:     Returned application handle that must be closed through
**                  MI_Application_Close().
**
** Return: MI_RESULT_OK success, other errors indicate specific failure
**=============================================================================
*/
MI_Result MI_MAIN_CALL MI_Application_InitializeV1(
             MI_Uint32 flags,
    _In_opt_z_ const MI_Char *applicationID,
    _Outptr_opt_result_maybenull_ MI_Instance **extendedError,
    _Out_    MI_Application *application);

#if MI_CALL_VERSION == 1
#define MI_Application_Initialize MI_Application_InitializeV1
#endif
/*
**=============================================================================
**
** MI_Application_Close()
**
** Deinitializes the management infrastructure.  This API is synchronous.
** It must not be called from within an asynchronous callback otherwise it will
** cause deadlocks.  This API will cancel all active sessions and operations.
** All operations and sessions must be closed before this API will complete.
**
** application: Handle returned from MI_Application_Initialize.
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_Close(
    _Inout_ MI_Application *application)
{
    if (application && application->ft)
    {
        return application->ft->Close(application);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_Application_NewInstance()
**
** Creates an instance.  It may be strongly typed if an MI_ClassDecl is
** specified, although it can be created dynamically through instance API calls.
** The instance must be closed through MI_Instance_Delete.
**
** application: Handle returned from MI_Application_Initialize.
** className:   class name of instance.
** classRTTI:   Run-time type information that represents a class definition
** instance:    Resultant instance
**
** Return: MI_RESULT_OK success, other errors indicate specific failure
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_NewInstance(
    _In_     MI_Application *application,
    _In_z_ const MI_Char *className,
    _In_opt_ const MI_ClassDecl *classRTTI,
    _Outptr_    MI_Instance **instance)
{
    if (application && application->ft)
    {
        return application->ft->NewInstance(application, className, classRTTI, instance);
    }
    else
    {
        if (instance)
        {
            *instance = NULL;
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}
MI_INLINE MI_Result MI_Application_NewInstanceFromClass(
    _In_     MI_Application *application,
    _In_z_ const MI_Char *className,
    _In_opt_ const MI_Class *classObject,
    _Outptr_    MI_Instance **instance)
{
    if (application && application->ft)
    {
        return application->ft->NewInstanceFromClass(application, className, classObject, instance);
    }
    else
    {
        if (instance)
        {
            *instance = NULL;
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_Application_NewClass()
**
** Creates an MI_Class from an MI_ClassDecl
** The MI_Class must be closed through MI_Class_Delete.
**
** application  : Handle returned from MI_Application_Initialize.
** classDecl    : The MI_ClassDecl for the class to create.
** namespaceName: The optional namespace name.
** serverName   : The optional server name
** classObject  : The resultant MI_Class
**
** Return: MI_RESULT_OK success, other errors indicate specific failure
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_NewClass(
        _In_ MI_Application *application,
        _In_ const MI_ClassDecl* classDecl,
        __in_z_opt const MI_Char *namespaceName,
        __in_z_opt const MI_Char *serverName,
        _Outptr_ MI_Class** classObject)
{
    if (application && application->ft)
    {
        return application->ft->NewClass(application, classDecl, namespaceName, serverName, classObject);
    }
    else
    {
        if (classObject)
        {
            *classObject = NULL;
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_Application_NewParameterSet(
    _In_     MI_Application *application,
    _In_opt_ const MI_ClassDecl *classRTTI,
    _Outptr_    MI_Instance **instance)
{
    if (application && application->ft)
    {
        return application->ft->NewInstance(application, MI_T("Parameters"), classRTTI, instance);
    }
    else
    {
        if (instance)
        {
            *instance = NULL;
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_Application_NewDestinationOptions()
**
** Creates an MI_DestinationOptions object.  It represents configuration needed
** to talk to the destination endpoint.
** The destination options must be closed through MI_DestinationOptions_Delete.
**
** application: Handle returned from MI_Application_Initialize.
** options:     Resultant options handle for which options can be set
**
** Return: MI_RESULT_OK success, other errors indicate specific failure
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_NewDestinationOptions(
    _In_  MI_Application *application,
    _Out_ MI_DestinationOptions *options)
{
    if (application && application->ft)
    {
        return application->ft->NewDestinationOptions(application, options);
    }
    else
    {
        if (options)
        {
            memset(options, 0, sizeof(MI_DestinationOptions));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_Application_NewOperationOptions()
**
** Creates an MI_OperationOptions object.  It represents configuration needed
** to carry out an operation.
** The operaton options must be closed through MI_OperationOptions_Delete.
**
** application: Handle returned from MI_Application_Initialize.
** options:     Resultant options handle for which options can be set
**
** Return: MI_RESULT_OK success, other errors indicate specific failure
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_NewOperationOptions(
    _In_  MI_Application *application,
          MI_Boolean mustUnderstand,
    _Out_ MI_OperationOptions *options)
{
    if (application && application->ft)
    {
        return application->ft->NewOperationOptions(application, mustUnderstand, options);
    }
    else
    {
        if (options)
        {
            memset(options, 0, sizeof(MI_OperationOptions));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Application_NewSubscriptionDeliveryOptions(
    _In_  MI_Application *application,
    _In_  MI_SubscriptionDeliveryType deliveryType,
    _Out_ MI_SubscriptionDeliveryOptions *deliveryOptions)
{
    if (application && application->ft)
    {
        return application->ft->NewSubscriptionDeliveryOptions(application, deliveryType, deliveryOptions);
    }
    else
    {
        if (deliveryOptions)
        {
            memset(deliveryOptions, 0, sizeof(MI_SubscriptionDeliveryOptions));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}
/*
**=============================================================================
**
** MI_Application_NewSession()
**
** Creates a session that allows a group of operations that go to the same
** destination to be grouped so they can share connections. MI_CloseSession()
** needs to be called on the outbound session handle otherwise operations may
** crash or leak memory.  Close all operations before closing the session.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_NewSession(
    _In_     MI_Application *application,
    _In_opt_z_ const MI_Char *protocol,
    _In_opt_z_ const MI_Char *destination,
    _In_opt_ MI_DestinationOptions *options,
    _In_opt_ MI_SessionCallbacks *callbacks,
    _Outptr_opt_result_maybenull_ MI_Instance **extendedError,
    _Out_    MI_Session *session)
{
    if (application && application->ft)
    {
        return application->ft->NewSession(application, protocol, destination, options, callbacks, extendedError, session);
    }
    else
    {
        if (session)
        {
            memset(session, 0, sizeof(MI_Session));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_Application_NewHostedProvider()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Application_NewHostedProvider(
    _In_  MI_Application *application,
    _In_z_  const MI_Char *namespaceName,
    _In_z_  const MI_Char *providerName,
    _In_  MI_MainFunction mi_Main,
    _Outptr_opt_result_maybenull_ MI_Instance **extendedError,
    _Out_ MI_HostedProvider *hostedProvider)
{
    if (application && application->ft)
    {
        return application->ft->NewHostedProvider(application, namespaceName, providerName, mi_Main, extendedError, hostedProvider);
    }
    else
    {
        if (hostedProvider)
        {
            memset(hostedProvider, 0, sizeof(MI_HostedProvider));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Application_NewSerializer(
    _Inout_ MI_Application *application,
    MI_Uint32 flags,
    _In_z_ MI_Char *format,
    _Out_ MI_Serializer *serializer)
{
    if (application && application->ft)
    {
        return application->ft->NewSerializer(application, flags, format, serializer);
    }
    else
    {
        if (serializer)
        {
            memset(serializer, 0, sizeof(MI_Serializer));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Application_NewDeserializer(
    _Inout_ MI_Application *application,
    MI_Uint32 flags,
    _In_z_ MI_Char *format,
    _Out_ MI_Deserializer *deserializer)
{
    if (application && application->ft)
    {
        return application->ft->NewDeserializer(application, flags, format, deserializer);
    }
    else
    {
        if (deserializer)
        {
            memset(deserializer, 0, sizeof(MI_Deserializer));
        }
        return MI_RESULT_INVALID_PARAMETER;
    }
}



/*
**=============================================================================
**
** MI_HostedProvider_Close()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_HostedProvider_Close(
    _Inout_ MI_HostedProvider *hostedProvider)
{
    if (hostedProvider && hostedProvider->ft)
    {
        return hostedProvider->ft->Close(hostedProvider);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_HostedProvider_GetApplication()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_HostedProvider_GetApplication(
    _In_  MI_HostedProvider *hostedProvider,
    _Out_ MI_Application *application)
{
    if (hostedProvider && hostedProvider->ft)
    {
        return hostedProvider->ft->GetApplication(hostedProvider, application);
    }
    else if (application)
    {
        memset(application, 0, sizeof(MI_Application));
    }
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Session_Close()
**
** Closes the session and frees up all memory associated with it.  If there
** are unfinished operations, those operatons will be cancelled.  All
** operations must have their handles closed before the session finishes closing.
**
** This can be called from inside an asynchronous callback only if the callback
** is used.  Waiting for the completion of this callback from inside the callback
** will cause a deadlock, which is the same reason that calling this API synchronously
** will cause a deadlock on an asynchronous callback.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Session_Close(
    _Inout_     MI_Session *session,
    _In_opt_ void *completionContext,
    _In_opt_ void (MI_CALL *completionCallback)(_In_opt_ void *completionContext))
{
    if (session && session->ft)
    {
        return session->ft->Close(session, completionContext, completionCallback);
    }
    else if (completionCallback)
    {
        completionCallback(completionContext);
        return MI_RESULT_OK;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_Session_GetApplication()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Session_GetApplication(
    _In_  MI_Session *session,
    _Out_ MI_Application *application)
{
    if (session && session->ft)
    {
        return session->ft->GetApplication(session, application);
    }
    if (application)
    {
        memset(application, 0, sizeof(MI_Application));
    }
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Session_GetInstance()
**
** Initiates a Get operation.  If a result callback is specified, the callback
** will be called when the operation completes.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the object and/or operation
** return code.  Even if the operation is cancelled the client must still
** call MI_Operation_GetInstance as cancellation is scynchronous.
**
**=============================================================================
*/
MI_INLINE void MI_Session_GetInstance(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_     const MI_Instance *inboundInstance,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->GetInstance(session, flags, options, namespaceName, inboundInstance, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_ModifyInstance()
**
** Initiates a Modify operation.  If a result callback is specified, the callback
** will be called when the operation completes.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the object and/or operation
** return code.  Even if the operation is cancelled the client must still
** call MI_Operation_GetInstance as cancellation is scynchronous.
**
**=============================================================================
*/
MI_INLINE void MI_Session_ModifyInstance(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_     const MI_Instance *inboundInstance,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->ModifyInstance(session, flags, options, namespaceName, inboundInstance, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_CreateInstance()
**
** Initiates a Create operation.  If a result callback is specified, the callback
** will be called when the operation completes.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the object and/or operation
** return code.  Even if the operation is cancelled the client must still
** call MI_Operation_GetInstance as cancellation is scynchronous.
**
**=============================================================================
*/
MI_INLINE void MI_Session_CreateInstance(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_     const MI_Instance *inboundInstance,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->CreateInstance(session, flags, options, namespaceName, inboundInstance, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}
/*
**=============================================================================
**
** MI_Session_DeleteInstance()
**
** Initiates a Delete operation.  If a result callback is specified, the callback
** will be called when the operation completes.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the operation return code.
** Even if the operation is cancelled the client must still call
** MI_Operation_GetInstance as cancellation is scynchronous.
**
**=============================================================================
*/
MI_INLINE void MI_Session_DeleteInstance(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_     const MI_Instance *inboundInstance,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->DeleteInstance(session, flags, options, namespaceName, inboundInstance, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_Invoke()
**
** Initiates a Delete operation.  If a result callback is specified, the callback
** will be called when the operation completes.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the object and/or operation
** return code.  Even if the operation is cancelled the client must still
** call MI_Operation_GetInstance as cancellation is scynchronous.
**
**=============================================================================
*/
MI_INLINE void MI_Session_Invoke(
    _In_       MI_Session *session,
               MI_Uint32 flags,
    _In_opt_   MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_z_ const MI_Char *className,
    _In_z_     const MI_Char *methodName,
    _In_opt_   const MI_Instance *inboundInstance,
    _In_opt_   const MI_Instance *inboundProperties,
    _In_opt_   MI_OperationCallbacks *callbacks,
    _Out_      MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->Invoke(session, flags, options, namespaceName, className, methodName, inboundInstance, inboundProperties, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_EnumerateInstances()
**
** Initiates a Enumerate operation.  If a result callback is specified, the callback
** will be called for each result in the result set and when complete gives the
** return code of the operation.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the objects, once for each object
** until the operation return code is also included.  Even if the operation is
** cancelled the client must still call MI_Operation_GetInstance until the return
** code is given.
**
**=============================================================================
*/
MI_INLINE void MI_Session_EnumerateInstances(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_z_ const MI_Char *className,
             MI_Boolean keysOnly,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->EnumerateInstances(session, flags, options, namespaceName, className, keysOnly, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

MI_INLINE void MI_Session_QueryInstances(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_z_ const MI_Char *queryDialect,
    _In_opt_z_ const MI_Char *queryExpression,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->QueryInstances(session, flags, options, namespaceName, queryDialect, queryExpression, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_Associators()
**
** Initiates a Associators operation.  If a result callback is specified, the
** callback will be called for each result in the result set and when complete
** gives the return code of the operation.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the objects, once for each object
** until the operation return code is also included.  Even if the operation is
** cancelled the client must still call MI_Operation_GetInstance until the return
** code is given.
**
**=============================================================================
*/
MI_INLINE void MI_Session_AssociatorInstances(
    _In_       MI_Session *session,
               MI_Uint32 flags,
    _In_opt_   MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_       const MI_Instance *instanceKey,
    _In_opt_z_ const MI_Char *assocClass,
    _In_opt_z_ const MI_Char *resultClass,
    _In_opt_z_ const MI_Char *role,
    _In_opt_z_ const MI_Char *resultRole,
               MI_Boolean keysOnly,
    _In_opt_   MI_OperationCallbacks *callbacks,
    _Out_      MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->AssociatorInstances(session, flags, options, namespaceName, instanceKey, assocClass, resultClass, role, resultRole, keysOnly, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_References()
**
** Initiates a References operation.  If a result callback is specified, the
** callback will be called for each result in the result set and when complete
** gives the return code of the operation.  For synchronous, the client
** must call MI_Operation_GetInstance to receive the objects, once for each object
** until the operation return code is also included.  Even if the operation is
** cancelled the client must still call MI_Operation_GetInstance until the return
** code is given.
**
**=============================================================================
*/
MI_INLINE void MI_Session_ReferenceInstances(
    _In_     MI_Session *session,
             MI_Uint32 flags,
    _In_opt_ MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_     const MI_Instance *instanceKey,
    _In_opt_z_ const MI_Char *resultClass,
    _In_opt_z_ const MI_Char *role,
             MI_Boolean keysOnly,
    _In_opt_ MI_OperationCallbacks *callbacks,
    _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->ReferenceInstances(session, flags, options, namespaceName, instanceKey, resultClass, role, keysOnly, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_Subscribe()
**
** Initiates a Subscribe operation.  If a indicate callback is specified, the
** callback will be called for each indications in the result set and when
** complete gives the return code of the operation.  For synchronous, the client
** must call MI_Operation_GetIndication to receive the objects, once for each object
** until the operation return code is also included.  Even if the operation is
** cancelled the client must still call MI_Operation_GetIndication until the return
** code is given.
**
**=============================================================================
*/

MI_INLINE void MI_Session_Subscribe(
    _In_       MI_Session *session,
               MI_Uint32 flags,
    _In_opt_   MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_z_ const MI_Char *queryDialect,
    _In_opt_z_ const MI_Char *queryExpression,
    _In_opt_   const MI_SubscriptionDeliveryOptions *deliverOptions,
    _In_opt_   MI_OperationCallbacks *callbacks,
    _Out_      MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->Subscribe(session, flags, options, namespaceName, queryDialect, queryExpression, deliverOptions, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->indicationResult)
        {
            callbacks->indicationResult(NULL, callbacks->callbackContext, NULL, NULL, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_GetClass()
**
**=============================================================================
*/
MI_INLINE void MI_Session_GetClass(
    _In_       MI_Session *session,
               MI_Uint32 flags,
    _In_opt_   MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_z_ const MI_Char *className,
    _In_opt_   MI_OperationCallbacks *callbacks,
    _Out_      MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->GetClass(session, flags, options, namespaceName, className, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->classResult)
        {
            callbacks->classResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Session_EnumerateClasses()
**
**=============================================================================
*/
MI_INLINE void MI_Session_EnumerateClasses(
    _In_       MI_Session *session,
               MI_Uint32 flags,
    _In_opt_   MI_OperationOptions *options,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_z_ const MI_Char *className,
               MI_Boolean classNamesOnly,
    _In_opt_   MI_OperationCallbacks *callbacks,
    _Out_      MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->EnumerateClasses(session, flags, options, namespaceName, className, classNamesOnly, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->classResult)
        {
            callbacks->classResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

MI_INLINE void MI_Session_TestConnection(
        _In_     MI_Session *session,
                 MI_Uint32 flags,
        _In_opt_ MI_OperationCallbacks *callbacks,
        _Out_    MI_Operation *operation)
{
    if (session && session->ft)
    {
        session->ft->TestConnection(session, flags, callbacks, operation);
    }
    else
    {
        if (operation)
        {
            memset(operation, 0, sizeof(*operation));
        }
        if (callbacks && callbacks->instanceResult)
        {
            callbacks->instanceResult(NULL, callbacks->callbackContext, NULL, MI_FALSE, MI_RESULT_INVALID_PARAMETER, NULL, NULL, NULL);
        }
    }
}

/*
**=============================================================================
**
** MI_Operation_GetInstance()
**
** This method is called to get a syncronous result for all operations except
** subscriptions, where MI_Operation_GetIndication should be used.
** It is an error to call this function if a result callback is registered.
** This method will block until a result is available.  If this is an
** enumeration operation then this function should be called until a
** returnCode is returned.
** Calls to this method for enumerations could cause many network round trips
** to happen if it is a large enumeration.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Operation_GetInstance(
    _In_        MI_Operation *operation,
    _Outptr_result_maybenull_       const MI_Instance **instance,
    _Out_opt_   MI_Boolean *moreResults,
    _Out_opt_   MI_Result *result,
    _Outptr_opt_result_maybenull_z_ const MI_Char **errorMessage,
    _Outptr_opt_result_maybenull_   const MI_Instance **completionDetails)
{
    if (operation && operation->ft)
    {
        return operation->ft->GetInstance(operation, instance, moreResults, result, errorMessage, completionDetails);
    }
    if (result) *result = MI_RESULT_INVALID_PARAMETER;
    if (moreResults) *moreResults = MI_FALSE;
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Operation_GetIndication()
**
** This method is called to get a syncronous result for a subscription.
** It is an error to call this function if a Indication callback is registered.
** This method will block until a result is available.  This function should be
** called until a returnCode object is returned.
** Calls to this method are likely to cause many network round trips
** to happen.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Operation_GetIndication(
    _In_        MI_Operation *operation,
    _Outptr_result_maybenull_       const MI_Instance **instance,
    _Outptr_opt_result_maybenull_z_ const MI_Char **bookmark,
    _Outptr_opt_result_maybenull_z_ const MI_Char **machineID,
    _Out_opt_   MI_Boolean *moreResults,
    _Out_opt_   MI_Result *result,
    _Outptr_opt_result_maybenull_z_ const MI_Char **errorMessage,
    _Outptr_opt_result_maybenull_       const MI_Instance **completionDetails)
{
    if (operation && operation->ft)
    {
        return operation->ft->GetIndication(operation, instance, bookmark, machineID, moreResults, result, errorMessage, completionDetails);
    }
    if (result) *result = MI_RESULT_INVALID_PARAMETER;
    if (moreResults) *moreResults = MI_FALSE;
    return  MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Operation_GetClass()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Operation_GetClass(
    _In_        MI_Operation *operation,
    _Outptr_result_maybenull_       const MI_Class **classResult,
    _Out_opt_   MI_Boolean *moreResults,
    _Out_opt_   MI_Result *result,
    _Outptr_opt_result_maybenull_z_ const MI_Char **errorMessage,
    _Outptr_opt_result_maybenull_   const MI_Instance **completionDetails)
{
    if (operation && operation->ft)
    {
        return operation->ft->GetClass(operation, classResult, moreResults, result, errorMessage, completionDetails);
    }
    if (result) *result = MI_RESULT_INVALID_PARAMETER;
    if (moreResults) *moreResults = MI_FALSE;
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Operation_Close()
**
** This method closes down an operation.  Normally this will happen after a
** result is returned, however this can be called to cancel a running
** operation.
** This is asynchronous if the operation is asynchronous, or synchronous and blocks
** if the operation is synchronous.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Operation_Close(
    _Inout_ MI_Operation *operation)
{
    if (operation && operation->ft)
    {
        return operation->ft->Close(operation);
    }
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Operation_Cancel()
**
** Cancels a running operation.  All results need to be consumed until moreResults
** returns MI_FALSE.  The operation also must still be closed with MI_Operation_Close.
**=============================================================================
*/
MI_INLINE MI_Result MI_Operation_Cancel(
    _Inout_ MI_Operation *operation,
         MI_CancellationReason reason)
{
    if (operation && operation->ft)
    {
        return operation->ft->Cancel(operation, reason);
    }
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_Operation_GetSession()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_Operation_GetSession(
    _In_  MI_Operation *operation,
    _Out_ MI_Session *session)
{
    if (session)
    {
        memset(session, 0, sizeof(MI_Session));
    }

    if (operation && operation->ft)
    {
        return operation->ft->GetSession(operation, session);
    }
    return MI_RESULT_INVALID_PARAMETER;
}

/*
**=============================================================================
**
** MI_DestinationOptions_Delete()
**
**=============================================================================
*/
MI_INLINE void MI_DestinationOptions_Delete(
    _Inout_ MI_DestinationOptions *options)
{
    if (options && options->ft)
    {
        options->ft->Delete(options);
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetTimeout()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetTimeout(
    _Inout_ MI_DestinationOptions *options,
    _In_ const MI_Interval *timeout)
{
    if (options && options->ft)
    {
        return options->ft->SetInterval(options, MI_T("__MI_DESTINATIONOPTIONS_TIMEOUT"), timeout, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetTimeout()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetTimeout(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Interval *timeout)
{
    if (options && options->ft)
    {
        return options->ft->GetInterval(options, MI_T("__MI_DESTINATIONOPTIONS_TIMEOUT"), timeout, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetCertCACheck()
**
**=============================================================================
*/
/* Check/skip CA check when doing SSL, default TRUE, check*/
MI_INLINE MI_Result MI_DestinationOptions_SetCertCACheck(
    _Inout_ MI_DestinationOptions *options,
         MI_Boolean check)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_CERT_CA_CHECK"), check, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetCertCACheck()
**
** Description
**  Get Check/skip CA check when doing SSL
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetCertCACheck(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Boolean *check)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_CERT_CA_CHECK"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *check = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_DestinationOptions_SetCertCNCheck()
**
**=============================================================================
*/
/* Check/skip CN check when doing SSL, default TRUE, check*/
MI_INLINE MI_Result MI_DestinationOptions_SetCertCNCheck(
    _Inout_ MI_DestinationOptions *options,
         MI_Boolean check)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_CERT_CN_CHECK"), check, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetCertCNCheck()
**
** Description:
**  Get check/skip CN check when doing SSL
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetCertCNCheck(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Boolean *check)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_CERT_CN_CHECK"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *check = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetCertRevocationCheck()
**
**=============================================================================
*/
/* Check/skip cert revocation check when doing SSL, default TRUE, check*/
MI_INLINE MI_Result MI_DestinationOptions_SetCertRevocationCheck(
    _Inout_ MI_DestinationOptions *options,
         MI_Boolean check)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_CERT_REVOCATION_CHECK"), check, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetCertRevocationCheck()
**
** Description:
**  Get Check/skip cert revocation check when doing SSL
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetCertRevocationCheck(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Boolean *check)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_CERT_REVOCATION_CHECK"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *check = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetPacketPrivacy()
**
** Ensure packet privacy (encryption), default TRUE
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetPacketPrivacy(
    _Inout_ MI_DestinationOptions *options,
         MI_Boolean privacy)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_PACKET_PRIVACY"), privacy, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetPacketPrivacy()
**
** Description:
**  Get packet privacy (encryption)
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetPacketPrivacy(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Boolean *privacy)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_PACKET_PRIVACY"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *privacy = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetPacketIntegrity()
**
**=============================================================================
*/
/* Ensure packet integrity (sign), default TRUE*/
MI_INLINE MI_Result MI_DestinationOptions_SetPacketIntegrity(
    _Inout_ MI_DestinationOptions *options,
         MI_Boolean integrity)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_PACKET_INTEGRITY"), integrity, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetPacketIntegrity()
**
** Description
**  Get packet integrity (sign)
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetPacketIntegrity(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Boolean *integrity)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_PACKET_INTEGRITY"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *integrity = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
**
**=============================================================================
*/
#define MI_DESTINATIONOPTIONS_PACKET_ENCODING_DEFAULT MI_T("default")
#define MI_DESTINATIONOPTIONS_PACKET_ENCODING_UTF8 MI_T("UTF8")
#define MI_DESTINATIONOPTIONS_PACKET_ENCODING_UTF16 MI_T("UTF16")

/*
**=============================================================================
**
** MI_DestinationOptions_SetPacketEncoding()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetPacketEncoding(
    _Inout_   MI_DestinationOptions *options,
    _In_z_ const MI_Char *encoding)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_DESTINATIONOPTIONS_PACKET_ENCODING"), encoding, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetPacketEncoding()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetPacketEncoding(
    _In_   MI_DestinationOptions *options,
    _Outptr_result_z_ const MI_Char **encoding)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_DESTINATIONOPTIONS_PACKET_ENCODING"), encoding, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetDataLocale()
**
** Description:
**      Overrides the default data locale that is used for requests.
**      By default the data locale of the calling thread is used and this
**      method will override with the specified locale.
**      Data locale is used  to determine string formats for things like
**      decimal nunbers in string format and date/time formats.
**
** Parameters
**    Option:  Valid MI_DestinationOptions created through
**             MI_Application_NewDestinationOptions.
**    locale:  Valid data locale string.  Example: en-us
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetDataLocale(
    _Inout_   MI_DestinationOptions *options,
    _In_z_ const MI_Char *locale)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_DESTINATIONOPTIONS_DATA_LOCALE"), locale, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetDataLocale()
**
** Description:
**      Gets the data locale set by user.
**
** Parameters
**    Option:  Valid MI_DestinationOptions created through
**             MI_Application_NewDestinationOptions.
**    locale:  data locale string has been set.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetDataLocale(
    _In_   MI_DestinationOptions *options,
    _Outptr_result_z_ const MI_Char **locale)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_DESTINATIONOPTIONS_DATA_LOCALE"), locale, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetUILocale()
**
** Description:
**      Overrides the default UI locale that is used for requests.
**      By default the UI locale of the calling thread is used and this
**      method will override with the specified locale.
**      UI locale is used  to determine the language used for localizable
**      strings, such as error messages.
**
** Parameters
**    Option:  Valid MI_DestinationOptions created through
**             MI_Application_NewDestinationOptions.
**    locale:  Valid UI locale string.  Example: en-us
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetUILocale(
    _Inout_   MI_DestinationOptions *options,
    _In_z_ const MI_Char *locale)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_DESTINATIONOPTIONS_UI_LOCALE"), locale, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetUILocale()
**
** Description:
**      Gets the UI locale set by user.
**
** Parameters
**    Option:  Valid MI_DestinationOptions created through
**             MI_Application_NewDestinationOptions.
**    locale:  UI locale string has been set.
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetUILocale(
    _In_   MI_DestinationOptions *options,
    _Outptr_result_z_ const MI_Char **locale)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_DESTINATIONOPTIONS_UI_LOCALE"), locale, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetMaxEnvelopeSize()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetMaxEnvelopeSize(
    _Inout_ MI_DestinationOptions *options,
         MI_Uint32 sizeInKB)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_MAX_ENVELOPE_SIZE"), sizeInKB, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetMaxEnvelopeSize()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetMaxEnvelopeSize(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Uint32 *sizeInKB)
{
    if (options && options->ft)
    {
        return options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_MAX_ENVELOPE_SIZE"), sizeInKB, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetEncodePortInSPN()
**
** default MI_FALSE = don't, MI_TRUE = do encode, WinRM transport specific
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetEncodePortInSPN(
    _Inout_ MI_DestinationOptions *options,
         MI_Boolean encodePort)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_ENCODE_PORT_IN_SPN"), encodePort, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetEncodePortInSPN()
**
** default MI_FALSE = don't, MI_TRUE = do encode, WinRM transport specific
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetEncodePortInSPN(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Boolean *encodePort)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_ENCODE_PORT_IN_SPN"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *encodePort = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetHttpUrlPrefix()
**
**Specific to HTTP/HTTP based transports, WinRM default is /wsman, other transports are transports/protocols specific
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetHttpUrlPrefix(
    _Inout_   MI_DestinationOptions *options,
    _In_z_ const MI_Char *prefix)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_DESTINATIONOPTIONS_HTTP_URL_PREFIX"), prefix, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetHttpUrlPrefix()
**
**Specific to HTTP/HTTP based transports, WinRM default is /wsman, other transports are transports/protocols specific
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetHttpUrlPrefix(
    _In_   MI_DestinationOptions *options,
    _Outptr_result_z_ const MI_Char **prefix)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_DESTINATIONOPTIONS_HTTP_URL_PREFIX"), prefix, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetDestinationPort()
**
** Transport specific, Default port for transport.
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetDestinationPort(
    _Inout_ MI_DestinationOptions *options,
         MI_Uint32 port)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_DESTINATION_PORT"), port, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetDestinationPort()
**
** Transport specific, Default port for transport.
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetDestinationPort(
    _In_ MI_DestinationOptions *options,
    _Out_ MI_Uint32 *port)
{
    if (options && options->ft)
    {
        return options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_DESTINATION_PORT"), port, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** #define
**
**
**=============================================================================
*/
/*Transports vary depending on protocol.  For WSMAN, HTTP and HTTPS are supported */
#define MI_DESTINATIONOPTIONS_TRANSPORT_HTTP MI_T("HTTP")
#define MI_DESTINATIONOPTIONS_TRANPSORT_HTTPS MI_T("HTTPS")

/*
**=============================================================================
**
** MI_DestinationOptions_SetTransport()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetTransport(
    _Inout_ MI_DestinationOptions *options,
    _In_z_ const MI_Char *transport)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_DESTINATIONOPTIONS_TRANSPORT"), transport, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetTransport()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetTransport(
    _In_ MI_DestinationOptions *options,
    _Outptr_result_z_ const MI_Char **transport)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_DESTINATIONOPTIONS_TRANSPORT"), transport, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** #define
**
**=============================================================================
*/
#define MI_DESTINATIONOPTIONS_PROXY_TYPE_IE MI_T("IE")
#define MI_DESTINATIONOPTIONS_PROXY_TYPE_WINHTTP MI_T("WinHTTP")
#define MI_DESTINATIONOPTIONS_PROXY_TYPE_AUTO MI_T("Auto")
#define MI_DESTINATIONOPTIONS_PROXY_TYPE_NONE MI_T("None")

/*
**=============================================================================
**
** MI_DestinationOptions_SetProxyType()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetProxyType(
    _Inout_ MI_DestinationOptions *options,
    _In_z_ const MI_Char *proxyType)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_DESTINATIONOPTIONS_PROXY_TYPE"), proxyType, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetProxyType()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetProxyType(
    _In_ MI_DestinationOptions *options,
    _Outptr_result_z_ const MI_Char **proxyType)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_DESTINATIONOPTIONS_PROXY_TYPE"), proxyType, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_AddProxyCredentials()
**
** Cumulative meaning if you add 2 it will be dual auth.  Not all cred types
** can be combined, not all transport handles support dual auth.
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_AddProxyCredentials(
    _Inout_ MI_DestinationOptions *options,
    _In_ const MI_UserCredentials *credentials)
{
    if (options && options->ft)
    {
        return options->ft->AddCredentials(options, MI_T("__MI_DESTINATIONOPTIONS_PROXY_CREDENTIALS"), credentials, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_AddDestinationCredentials()
**
** Cumulative meaning if you add 2 it will be dual auth.  Not all cred types
** can be combined, not all transport handles support dual auth.
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_AddDestinationCredentials(
    _Inout_     MI_DestinationOptions *options,
    _In_ const MI_UserCredentials *credentials)
{
    if (options && options->ft)
    {
        return options->ft->AddCredentials(options, MI_T("__MI_DESTINATIONOPTIONS_DESTINATION_CREDENTIALS"), credentials, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

typedef enum _MI_DestinationOptions_ImpersonationType
{
    MI_DestinationOptions_ImpersonationType_Default = 0,
    MI_DestinationOptions_ImpersonationType_None = 1,
    MI_DestinationOptions_ImpersonationType_Identify = 2,
    MI_DestinationOptions_ImpersonationType_Impersonate = 3,
    MI_DestinationOptions_ImpersonationType_Delegate = 4
} MI_DestinationOptions_ImpersonationType;

MI_INLINE MI_Result MI_DestinationOptions_SetImpersonationType(
    _Inout_ MI_DestinationOptions *options,
    _In_ MI_DestinationOptions_ImpersonationType impersonationType)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_IMPERSONATION_TYPE"), impersonationType, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_DestinationOptions_GetImpersonationType(
    _Inout_ MI_DestinationOptions *options,
    _Out_ MI_DestinationOptions_ImpersonationType * impersonationType)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_DESTINATIONOPTIONS_IMPERSONATION_TYPE"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *impersonationType = (MI_DestinationOptions_ImpersonationType) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetString()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetString(
    _Inout_ MI_DestinationOptions *options,
    _In_z_ const MI_Char *optionName,
    _In_z_ const MI_Char *optionValue)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, optionName, optionValue, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetString()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetString(
    _In_ MI_DestinationOptions *options,
    _In_z_ const MI_Char *optionName,
    _Outptr_result_z_ const MI_Char **optionValue,
    _Out_opt_ MI_Uint32 *index)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, optionName, optionValue, index, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_SetNumber()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_SetNumber(
    _Inout_ MI_DestinationOptions *options,
    _In_z_ const MI_Char *optionName,
    _In_ MI_Uint32 optionValue)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, optionName, optionValue, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetNumber()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetNumber(
    _In_ MI_DestinationOptions *options,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Uint32 *optionValue,
    _Out_opt_ MI_Uint32 *index)
{
    if (options && options->ft)
    {
        return options->ft->GetNumber(options, optionName, optionValue, index, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_DestinationOptions_GetOptionCount()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetOptionCount(
    _In_   MI_DestinationOptions *options,
    _Out_ MI_Uint32 *count)
{
    if (options && options->ft)
    {
        return options->ft->GetOptionCount(options, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_DestinationOptions_GetOptionAt()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetOptionAt(
    _In_   MI_DestinationOptions *options,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_ MI_Value *value,
    _Out_ MI_Type *type,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetOptionAt(options, index, optionName, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_GetOption()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetOption(
    _In_   MI_DestinationOptions *options,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Value *value,
    _Out_ MI_Type *type,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetOption(options, optionName, value, type, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_DestinationOptions_GetCredentialsCount()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetCredentialsCount(
    _In_   MI_DestinationOptions *options,
    _Out_ MI_Uint32 *count)
{
    if (options && options->ft)
    {
        return options->ft->GetCredentialsCount(options, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_DestinationOptions_GetCredentialsAt()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetCredentialsAt(
    _In_   MI_DestinationOptions *options,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_ MI_UserCredentials *credentials, // output credentials always has password set to '******'
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetCredentialsAt(options, index, optionName, credentials, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_DestinationOptions_GetCredentialsPasswordAt()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_DestinationOptions_GetCredentialsPasswordAt(
    _In_   MI_DestinationOptions *options,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_writes_to_opt_(bufferLength, *passwordLength) MI_Char *password,
    _In_ MI_Uint32 bufferLength,
    _Out_ MI_Uint32 *passwordLength,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetCredentialsPasswordAt(options, index, optionName, password, bufferLength, passwordLength, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_DestinationOptions_Clone()
**
** This function creates a copy of the given MI_DestinationOptions. Upon
** a successful return, new MI_DestinationOptions points to a newly
** created destination options object.
** The new destination options should eventually be passed to MI_DestinationOptions_Delete().
**
** param: self pointer to the destination options to be cloned.
** param: newDestinationOptions a pointer to the new destination options upon return.
**
** return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
**
** See also: MI_DestinationOptions_Delete()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_INLINE_CALL MI_DestinationOptions_Clone(
    _In_ const MI_DestinationOptions* self,
    _Out_ MI_DestinationOptions* newDestinationOptions)
{
    if (self && self->ft)
    {
        return self->ft->Clone(self, newDestinationOptions);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_Delete()
**
**=============================================================================
*/
MI_INLINE void MI_OperationOptions_Delete(
    _Inout_ MI_OperationOptions *options)
{
    if (options && options->ft)
    {
        options->ft->Delete(options);
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetWriteErrorMode()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetWriteErrorMode(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_CallbackMode mode)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_WRITEERRORMODE"), mode, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetWriteErrorMode()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetWriteErrorMode(
    _In_ MI_OperationOptions *options,
    _Out_ MI_CallbackMode *mode)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_WRITEERRORMODE"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *mode = (MI_CallbackMode) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetPromptUserMode()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetPromptUserMode(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_CallbackMode mode)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROMPTUSERMODE"), mode, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}




/*
**=============================================================================
**
** MI_OperationOptions_GetPromptUserMode()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetPromptUserMode(
    _In_ MI_OperationOptions *options,
    _Out_ MI_CallbackMode *mode)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROMPTUSERMODE"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *mode = (MI_CallbackMode) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetPromptUserRegularMode()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetPromptUserRegularMode(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_CallbackMode mode,
    _In_ MI_Boolean ackValue)
{
    if (options && options->ft)
    {
        MI_Result result = options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROMPTUSERMODE"), mode, 0);
        if( result == MI_RESULT_OK)
            return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROMPTUSERMODEREGULAR_ACKVALUE"), ackValue, 0);
        else
            return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetPromptUserRegularMode()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetPromptUserRegularMode(
    _Inout_ MI_OperationOptions *options,
    _Out_ MI_CallbackMode *mode,
    _Out_ MI_Boolean *ackValue)
{
    if (options && options->ft)
    {
        MI_Uint32 _mode;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROMPTUSERMODE"), &_mode, 0, 0);
        if( result == MI_RESULT_OK)
        {
            MI_Uint32 _ackValue;
            result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROMPTUSERMODEREGULAR_ACKVALUE"), &_ackValue, 0, 0);
            if( result == MI_RESULT_OK)
            {
                *mode = (MI_CallbackMode)_mode;
                *ackValue = (MI_Boolean) _ackValue;
            }
        }
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}



/*
**=============================================================================
**
** MI_OperationOptions_SetProviderArchitecture()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetProviderArchitecture(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_ProviderArchitecture architecture,
    _In_ MI_Boolean mustComply)
{
    if (options && options->ft)
    {
        MI_Result result = options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROVIDER_ARCHITECTURE"), architecture, 0);
        if(result == MI_RESULT_OK)
               return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_REQUIRED_ARCHITECTURE"), mustComply, 0);
       else
              return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetProviderArchitecture()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetProviderArchitecture(
    _In_ MI_OperationOptions *options,
    _Out_ MI_ProviderArchitecture *architecture,
    _Out_ MI_Boolean *mustComply)
{
    if (options && options->ft)
    {
        MI_Uint32 _architecture;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_PROVIDER_ARCHITECTURE"), &_architecture, 0, 0);
        if(result == MI_RESULT_OK)
        {
            MI_Uint32 _mustComply;
            result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_REQUIRED_ARCHITECTURE"), &_mustComply, 0, 0);
            if(result == MI_RESULT_OK)
            {
                *architecture = (MI_ProviderArchitecture)_architecture;
                *mustComply = (MI_Boolean)_mustComply;
            }
        }
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_EnableChannel()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_EnableChannel(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_Uint32 channel)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_CHANNEL"), channel, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_DisableChannel()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_DisableChannel(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_Uint32 channel)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_CHANNEL"), channel, 1);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetEnabledChannels()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetEnabledChannels(
    _In_   MI_OperationOptions *options,
    _Out_writes_to_opt_(bufferLength, *channelCount) MI_Uint32 *channels,
    _In_ MI_Uint32 bufferLength,
    _Out_ MI_Uint32 *channelCount,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetEnabledChannels(options, MI_T("__MI_OPERATIONOPTIONS_CHANNEL"), channels, bufferLength, channelCount, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetTimeout()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetTimeout(
    _Inout_ MI_OperationOptions *options,
    _In_ const MI_Interval *timeout)
{
    if (options && options->ft)
    {
        return options->ft->SetInterval(options, MI_T("__MI_OPERATIONOPTIONS_TIMEOUT"), timeout, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetTimeout()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetTimeout(
    _In_ MI_OperationOptions *options,
    _Out_ MI_Interval *timeout)
{
    if (options && options->ft)
    {
        return options->ft->GetInterval(options, MI_T("__MI_OPERATIONOPTIONS_TIMEOUT"), timeout, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetResourceUriPrefix()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetResourceUriPrefix(
    _Inout_   MI_OperationOptions *options,
    _In_z_ const MI_Char *ruriPrefix)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_OPERATIONOPTIONS_RESOURCE_URI_PREFIX"), ruriPrefix, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetResourceUriPrefix()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetResourceUriPrefix(
    _In_   MI_OperationOptions *options,
    _Outptr_result_z_ const MI_Char **ruriPrefix)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_OPERATIONOPTIONS_RESOURCE_URI_PREFIX"), ruriPrefix, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetResourceUri()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetResourceUri(
    _Inout_   MI_OperationOptions *options,
    _In_z_ const MI_Char *rUri)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, MI_T("__MI_OPERATIONOPTIONS_RESOURCE_URI"), rUri, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetResourceUri()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetResourceUri(
    _In_   MI_OperationOptions *options,
    _Outptr_result_z_ const MI_Char **rUri)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, MI_T("__MI_OPERATIONOPTIONS_RESOURCE_URI"), rUri, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


/*
**=============================================================================
**
** MI_OperationOptions_SetUseMachineID()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetUseMachineID(
    _Inout_ MI_OperationOptions *options,
    _In_ MI_Boolean machineID)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, MI_T("__MI_OPERATIONOPTIONS_USE_MACHINE_ID"), machineID, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetUseMachineID()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetUseMachineID(
    _In_ MI_OperationOptions *options,
    _Out_ MI_Boolean *machineID)
{
    if (options && options->ft)
    {
        MI_Uint32 value;
        MI_Result result = options->ft->GetNumber(options, MI_T("__MI_OPERATIONOPTIONS_USE_MACHINE_ID"), &value, 0, 0);
        if (result == MI_RESULT_OK)
            *machineID = (MI_Boolean) value;
        return result;
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetCustomOption()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetCustomOption(
    _Inout_   MI_OperationOptions *options,
    _In_z_ const MI_Char *optionName,
    _In_   MI_Type optionValueType,
    _In_   const MI_Value *optionValue,
           MI_Boolean mustComply)
{
    if (options && options->ft)
    {
        return options->ft->SetCustomOption(options, optionName, optionValueType, optionValue, mustComply, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetOptionCount()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetOptionCount(
    _In_   MI_OperationOptions *options,
    _Out_ MI_Uint32 *count)
{
    if (options && options->ft)
    {
        return options->ft->GetOptionCount(options, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetOptionAt()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetOptionAt(
    _In_   MI_OperationOptions *options,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_ MI_Value *value,
    _Out_ MI_Type *type,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetOptionAt(options, index, optionName, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetString()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetString(
    _Inout_   MI_OperationOptions *options,
    _In_z_ const MI_Char *optionName,
    _In_z_ const MI_Char *value,
    MI_Uint32 flags)
{
    if (options && options->ft)
    {
        return options->ft->SetString(options, optionName, value, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetString()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetString(
    _In_   MI_OperationOptions *options,
    _In_z_ const MI_Char *optionName,
    _Outptr_result_z_ const MI_Char **value,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetString(options, optionName, value, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_SetNumber()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_SetNumber(
    _Inout_   MI_OperationOptions *options,
    _In_z_ const MI_Char *optionName,
    _In_   MI_Uint32 value,
    MI_Uint32 flags)
{
    if (options && options->ft)
    {
        return options->ft->SetNumber(options, optionName, value, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetNumber()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetNumber(
    _In_   MI_OperationOptions *options,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Uint32 *value,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetNumber(options, optionName, value, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_GetOption()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_OperationOptions_GetOption(
    _In_   MI_OperationOptions *options,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Value *value,
    _Out_ MI_Type *type,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (options && options->ft)
    {
        return options->ft->GetOption(options, optionName, value, type, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/*
**=============================================================================
**
** MI_OperationOptions_Clone()
**
** This function creates a copy of the given MI_OperationOptions. Upon
** a successful return, new MI_OperationOptions points to a newly
** created operation options object.
** The new operation options should eventually be passed to MI_OperationOptions_Delete().
**
** param: self pointer to the operation options to be cloned.
** param: newOperationOptions a pointer to the new operation options upon return.
**
** return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
**
** See also: MI_OperationOptions_Delete()
**
**=============================================================================
*/
MI_INLINE MI_Result MI_INLINE_CALL MI_OperationOptions_Clone(
    _In_ const MI_OperationOptions* self,
    _Out_ MI_OperationOptions* newOperationOptions)
{
    if (self && self->ft)
    {
        return self->ft->Clone(self, newOperationOptions);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_Class_GetClassName(
        _In_              const MI_Class* self,
        _Outptr_result_maybenull_z_ const MI_Char** className)
{
    if (self && self->ft)
    {
        return self->ft->GetClassName(self, className);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetNameSpace(
        _In_              const MI_Class* self,
        _Outptr_result_maybenull_z_ const MI_Char** nameSpace)
{
    if (self && self->ft)
    {
        return self->ft->GetNameSpace(self, nameSpace);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_Class_GetServerName(
        _In_              const MI_Class* self,
        _Outptr_result_maybenull_z_ const MI_Char** serverName)
{
    if (self && self->ft)
    {
        return self->ft->GetServerName(self, serverName);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_Class_GetElementCount(
        _In_  const MI_Class* self,
        _Out_ MI_Uint32* count)
{
    if (self && self->ft)
    {
        return self->ft->GetElementCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_Class_GetElement(
        _In_      const MI_Class* self,
        _In_z_    const MI_Char* name,
        _Out_opt_ MI_Value* value,
        _Out_opt_ MI_Boolean* valueExists,
        _Out_opt_ MI_Type* type,
        _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_Uint32* flags,
        _Out_opt_ MI_Uint32* index)
{
    if (self && self->ft)
    {
        return self->ft->GetElement(self, name, value, valueExists, type, referenceClass, qualifierSet, flags, index);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_Class_GetElementAt(
        _In_              const MI_Class* self,
                          MI_Uint32 index,
        _Outptr_opt_result_maybenull_z_ const MI_Char** name,
        _Out_opt_         MI_Value* value,
        _Out_opt_         MI_Boolean* valueExists,
        _Out_opt_         MI_Type* type,
        _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_         MI_Uint32* flags)
{
    if (self && self->ft)
    {
        return self->ft->GetElementAt(self, index, name, value, valueExists, type, referenceClass, qualifierSet, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetClassQualifierSet(
        _In_            const MI_Class* self,
        _Out_opt_ MI_QualifierSet *qualifierSet
        )
{
    if (self && self->ft)
    {
        return self->ft->GetClassQualifierSet(self, qualifierSet);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetMethodCount(
        _In_  const MI_Class* self,
        _Out_ MI_Uint32* count)
{
    if (self && self->ft)
    {
        return self->ft->GetMethodCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetMethodAt(
        _In_ const MI_Class *self,
        MI_Uint32 index,
        _Outptr_result_z_ const MI_Char **name,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_ParameterSet *parameterSet)
{
    if (self && self->ft)
    {
        return self->ft->GetMethodAt(self, index, name, qualifierSet, parameterSet);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetMethod(
        _In_ const MI_Class *self,
        _In_z_ const MI_Char *name,
        _Out_opt_ MI_QualifierSet *qualifierSet,
        _Out_opt_ MI_ParameterSet *parameterSet,
        _Out_opt_ MI_Uint32 *index)
{
    if (self && self->ft)
    {
        return self->ft->GetMethod(self, name, qualifierSet, parameterSet, index);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetParentClassName(
        _In_ const MI_Class *self,
        _Outptr_result_maybenull_z_ const MI_Char **name)
{
    if (self && self->ft)
    {
        return self->ft->GetParentClassName(self, name);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_GetParentClass(
        _In_ const MI_Class *self,
        _Outptr_ MI_Class **parentClass)
{
    if (self && self->ft)
    {
        return self->ft->GetParentClass(self, parentClass);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_Class_Delete(
        _Inout_ MI_Class* self)
{
    if (self && self->ft)
    {
        return self->ft->Delete(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_INLINE_CALL MI_Class_Clone(
    _In_ const MI_Class* self,
    _Outptr_ MI_Class** newClass)
{
    if (self && self->ft)
    {
        return self->ft->Clone(self, newClass);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_ParameterSet_GetMethodReturnType(
    _In_  const MI_ParameterSet *self,
    _Out_ MI_Type *returnType,
    _Out_ MI_QualifierSet *qualifierSet)
{
    if (self && self->ft)
    {
        return self->ft->GetMethodReturnType(self, returnType, qualifierSet);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_ParameterSet_GetParameterCount(
    _In_ const MI_ParameterSet *self,
    _Out_ MI_Uint32 *count)
{
    if (self && self->ft)
    {
        return self->ft->GetParameterCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_ParameterSet_GetParameterAt(
    _In_ const MI_ParameterSet *self,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **name,
    _Out_ MI_Type *parameterType,
    _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
    _Out_ MI_QualifierSet *qualifierSet)
{
    if (self && self->ft)
    {
        return self->ft->GetParameterAt(self, index, name, parameterType, referenceClass, qualifierSet);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_ParameterSet_GetParameter(
    _In_ const MI_ParameterSet *self,
    _In_z_ const MI_Char *name,
    _Out_ MI_Type *parameterType,
    _Outptr_opt_result_maybenull_z_ MI_Char **referenceClass,
    _Out_ MI_QualifierSet *qualifierSet,
    _Out_ MI_Uint32 *index)
{
    if (self && self->ft)
    {
        return self->ft->GetParameter(self, name, parameterType, referenceClass, qualifierSet, index);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_QualifierSet_GetQualifierCount(
    _In_ const MI_QualifierSet *self,
    _Out_ MI_Uint32 *count)
{
    if (self && self->ft)
    {
        return self->ft->GetQualifierCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_QualifierSet_GetQualifierAt(
    _In_ const MI_QualifierSet *self,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **name,
    _Out_ MI_Type *qualifierType,
    _Out_ MI_Uint32 *qualifierFlags,    /* scope information */
    _Out_ MI_Value *qualifierValue)
{
    if (self && self->ft)
    {
        return self->ft->GetQualifierAt(self, index, name, qualifierType, qualifierFlags, qualifierValue);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_QualifierSet_GetQualifier(
    _In_ const MI_QualifierSet *self,
    _In_z_ const MI_Char *name,
    _Out_ MI_Type *qualifierType,
    _Out_ MI_Uint32 *qualifierFlags,    /* scope information */
    _Out_ MI_Value *qualifierValue,
    _Out_ MI_Uint32 *index)
{
    if (self && self->ft)
    {
        return self->ft->GetQualifier(self, name, qualifierType, qualifierFlags, qualifierValue, index);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetMaximumLatency(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   MI_Interval *value)
{
    if (self && self->ft)
    {
        return self->ft->SetInterval(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_MAXIMUM_LATENCY"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetMaximumLatency(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Out_ MI_Interval *value)
{
    if (self && self->ft)
    {
        return self->ft->GetInterval(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_MAXIMUM_LATENCY"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetHeartbeatInterval(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   MI_Interval *value)
{
    if (self && self->ft)
    {
        return self->ft->SetInterval(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_HEARTBEAT_INTERVAL"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetHeartbeatInterval(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Out_ MI_Interval *value)
{
    if (self && self->ft)
    {
        return self->ft->GetInterval(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_HEARTBEAT_INTERVAL"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetExpirationTime(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   MI_Datetime *value)
{
    if (self && self->ft)
    {
        return self->ft->SetDateTime(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_EXPIRATION_TIME"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetExpirationTime(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _Out_ MI_Datetime *value)
{
    if (self && self->ft)
    {
        return self->ft->GetDateTime(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_EXPIRATION_TIME"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/* If bookmarks are required for subscriptions a bookmark needs to be set.
 * To indicate you want the indication events to start delivering from the
 * oldest possible event that is available use MI_SUBSCRIBE_BOOKMARK_OLDEST.
 * To start delivering from the latest events only specify MI_SUBSCRIBE_BOOKMARK_NEWEST.
 * To start delivering from a previously sent bookmark (if possible) pass in the boookmark
 * value that was delivered with the last event.
 * If no bookmark is set then the providers may not deliver bookmarks with the events.
 */
#define MI_SUBSCRIBE_BOOKMARK_OLDEST L"MI_SUBSCRIBE_BOOKMARK_OLDEST"
#define MI_SUBSCRIBE_BOOKMARK_NEWEST L"MI_SUBSCRIBE_BOOKMARK_NEWEST"

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetBookmark(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_   const MI_Char *value)
{
    if (self && self->ft)
    {
        return self->ft->SetString(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_BOOKMARK"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetBookmark(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Outptr_result_z_ const MI_Char **value)
{
    if (self && self->ft)
    {
        return self->ft->GetString(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_BOOKMARK"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetDeliveryDestination(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *value)
{
    if (self && self->ft)
    {
        return self->ft->SetString(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_DESTINATION"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetDeliveryDestination(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Outptr_result_z_ const MI_Char **value)
{
    if (self && self->ft)
    {
        return self->ft->GetString(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_DESTINATION"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetDeliveryPortNumber(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   MI_Uint32 value)
{
    if (self && self->ft)
    {
        return self->ft->SetNumber(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_PORT_NUMBER"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetDeliveryPortNumber(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Out_   MI_Uint32 *value)
{
    if (self && self->ft)
    {
        return self->ft->GetNumber(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_PORT_NUMBER"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_AddDeliveryCredentials(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   const MI_UserCredentials *value)
{
    if (self && self->ft)
    {
        return self->ft->AddCredentials(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_ADD_DELIVERY_CREDENTIALS"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetDeliveryRetryInterval(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   const MI_Interval *value)
{
    if (self && self->ft)
    {
        return self->ft->SetInterval(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_RETRY_INTERVAL"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetDeliveryRetryInterval(
    _In_  MI_SubscriptionDeliveryOptions *self,
    _Out_ MI_Interval *value)
{
    if (self && self->ft)
    {
        return self->ft->GetInterval(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_RETRY_INTERVAL"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetDeliveryRetryAttempts(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_   MI_Uint32 value)
{
    if (self && self->ft)
    {
        return self->ft->SetNumber(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_RETRY_ATTEMPTS"), value, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetDeliveryRetryAttempts(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Out_ MI_Uint32 *value)
{
    if (self && self->ft)
    {
        return self->ft->GetNumber(self, MI_T("__MI_SUBSCRIPTIONDELIVERYOPTIONS_SET_DELIVERY_RETRY_ATTEMPTS"), value, 0, 0);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}


MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_Delete(
        _Inout_ MI_SubscriptionDeliveryOptions* self)
{
    if (self && self->ft)
    {
        return self->ft->Delete(self);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetString(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _In_z_ const MI_Char *value,
            MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->SetString(self, optionName, value, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetNumber(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _In_   MI_Uint32 value,
            MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->SetNumber(self, optionName, value, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetDateTime(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _In_   const MI_Datetime *value,
            MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->SetDateTime(self, optionName, value, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_SetInterval(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _In_   const MI_Interval *value,
            MI_Uint32 flags)
{
    if (self && self->ft)
    {
        return self->ft->SetInterval(self, optionName, value, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetString(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _Outptr_result_z_ const MI_Char **value,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetString(self, optionName, value, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetNumber(
    _Inout_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Uint32 *value,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetNumber(self, optionName, value, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetDateTime(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Datetime *value,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetDateTime(self, optionName, value, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetInterval(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Interval *value,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetInterval(self, optionName, value, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetOptionCount(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Out_opt_ MI_Uint32 *count)
{
    if (self && self->ft)
    {
        return self->ft->GetOptionCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetOptionAt(
    _In_   MI_SubscriptionDeliveryOptions *self,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_ MI_Value *value,
    _Out_ MI_Type *type,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetOptionAt(self, index, optionName, value, type, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetOption(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _In_z_ const MI_Char *optionName,
    _Out_ MI_Value *value,
    _Out_ MI_Type *type,
    _Out_opt_ MI_Uint32 *index,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetOption(self, optionName, value, type, index, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetCredentialsCount(
    _In_   MI_SubscriptionDeliveryOptions *self,
    _Out_ MI_Uint32 *count)
{
    if (self && self->ft)
    {
        return self->ft->GetCredentialsCount(self, count);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetCredentialsAt(
    _In_   MI_SubscriptionDeliveryOptions *self,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_ MI_UserCredentials *credentials, // output credentials always has password set to '******'
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetCredentialsAt(self, index, optionName, credentials, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

MI_INLINE MI_Result MI_SubscriptionDeliveryOptions_GetCredentialsPasswordAt(
    _In_   MI_SubscriptionDeliveryOptions *self,
    MI_Uint32 index,
    _Outptr_result_z_ const MI_Char **optionName,
    _Out_writes_to_opt_(bufferLength, *passwordLength) MI_Char *password,
    _In_ MI_Uint32 bufferLength,
    _Out_ MI_Uint32 *passwordLength,
    _Out_opt_ MI_Uint32 *flags)
{
    if (self && self->ft)
    {
        return self->ft->GetCredentialsPasswordAt(self, index, optionName, password, bufferLength, passwordLength, flags);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

/**
 * This function creates a copy of the given MI_SubscriptionDeliveryOptions. Upon
 * a successful return, new MI_SubscriptionDeliveryOptions points to a newly
 * created delivery options.
 * The new delivery options should eventually be passed to MI_SubscriptionDeliveryOptions_Delete().
 *
 * param: self pointer to the delivery options to be cloned.
 * param: newSubscriptionDeliveryOptions a pointer to the new delivery options upon return.
 *
 * return: MI_RESULT_OK, MI_RESULT_FAILED, MI_RESULT_INVALID_PARAMETER
 *
 * See also: MI_SubscriptionDeliveryOptions_Delete()
 *
 */
MI_INLINE MI_Result MI_INLINE_CALL MI_SubscriptionDeliveryOptions_Clone(
    _In_ const MI_SubscriptionDeliveryOptions* self,
    _Out_ MI_SubscriptionDeliveryOptions* newSubscriptionDeliveryOptions)
{
    if (self && self->ft)
    {
        return self->ft->Clone(self, newSubscriptionDeliveryOptions);
    }
    else
    {
        return MI_RESULT_INVALID_PARAMETER;
    }
}

#define MI_SERIALIZER_FLAGS_CLASS_DEEP 1
#define MI_SERIALIZER_FLAGS_INSTANCE_WITH_CLASS 1

MI_INLINE MI_Result MI_Serializer_Close(
    _Inout_ MI_Serializer *serializer)
{
    return mi_clientFT->serializerFT->Close(serializer);
}

MI_INLINE MI_Result MI_Serializer_SerializeClass(
    _Inout_ MI_Serializer *serializer,
    MI_Uint32 flags,
    _In_ const MI_Class *classObject,
   _Out_writes_bytes_(clientBufferLength) MI_Uint8 *clientBuffer,
    MI_Uint32 clientBufferLength,
   _Inout_ MI_Uint32 *clientBufferNeeded)
{
    return mi_clientFT->serializerFT->SerializeClass(serializer, flags, classObject, clientBuffer, clientBufferLength, clientBufferNeeded);
}

MI_INLINE MI_Result MI_Serializer_SerializeInstance(
   _Inout_ MI_Serializer *serializer,
   MI_Uint32 flags,
   _In_ const MI_Instance *instanceObject,
   _Out_writes_bytes_(clientBufferLength) MI_Uint8 *clientBuffer,
    MI_Uint32 clientBufferLength,
   _Inout_ MI_Uint32 *clientBufferNeeded)
{
    return mi_clientFT->serializerFT->SerializeInstance(serializer, flags, instanceObject, clientBuffer, clientBufferLength, clientBufferNeeded);
}


MI_INLINE MI_Result MI_Deserializer_Close(
    _Inout_ MI_Deserializer *deserializer)
{
    const MI_ClientFT_V1 *clientFT = mi_clientFT;
    const MI_DeserializerFT *deserializerFT = clientFT->deserializerFT;
    return deserializerFT->Close(deserializer);
}

MI_INLINE MI_Result MI_Deserializer_DeserializeClass(
    _Inout_ MI_Deserializer *deserializer,
    MI_Uint32 flags,
    _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
    MI_Uint32 serializedBufferLength,
    _In_opt_ MI_Class *parentClass,
    _In_opt_z_ const MI_Char *serverName,
    _In_opt_z_ const MI_Char *namespaceName,
    _In_opt_ MI_Deserializer_ClassObjectNeeded classObjectNeeded,
    _In_opt_ void *classObjectNeededContext,
    _Out_opt_ MI_Uint32 *serializedBufferRead,
    _Outptr_opt_result_maybenull_ MI_Class **classObject,
    _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails)
{
    return mi_clientFT->deserializerFT->DeserializeClass(deserializer, flags, serializedBuffer, serializedBufferLength, parentClass, serverName, namespaceName, classObjectNeeded, classObjectNeededContext, serializedBufferRead, classObject, cimErrorDetails);
}

MI_INLINE MI_Result MI_Deserializer_Class_GetClassName(
    _Inout_ MI_Deserializer *deserializer,
    _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
    MI_Uint32 serializedBufferLength,
    _Out_writes_opt_(*classNameLength) MI_Char *className,
    _Inout_ MI_Uint32 *classNameLength,
    _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails)
{
    return mi_clientFT->deserializerFT->Class_GetClassName(deserializer, serializedBuffer, serializedBufferLength, className, classNameLength, cimErrorDetails);
}


MI_INLINE MI_Result MI_Deserializer_Class_GetParentClassName(
    _Inout_ MI_Deserializer *deserializer,
    _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
    MI_Uint32 serializedBufferLength,
    _Out_writes_opt_(*parentClassNameLength) MI_Char *parentClassName,
    _Inout_ MI_Uint32 *parentClassNameLength,
    _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails)
{
    return mi_clientFT->deserializerFT->Class_GetParentClassName(deserializer, serializedBuffer, serializedBufferLength, parentClassName, parentClassNameLength, cimErrorDetails);
}

MI_INLINE MI_Result MI_Deserializer_DeserializeInstance(
    _Inout_ MI_Deserializer *deserializer,
    MI_Uint32 flags,
    _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
    MI_Uint32 serializedBufferLength,
    _In_reads_opt_(numberClassObjects) MI_Class **classObjects,
    MI_Uint32 numberClassObjects,
    _In_opt_ MI_Deserializer_ClassObjectNeeded classObjectNeeded,
    _In_opt_ void *classObjectNeededContext,
    _Out_opt_ MI_Uint32 *serializedBufferRead,
    _Outptr_opt_result_maybenull_ MI_Instance **instanceObject,
    _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails)
{
    return mi_clientFT->deserializerFT->DeserializeInstance(deserializer, flags, serializedBuffer, serializedBufferLength, classObjects, numberClassObjects, classObjectNeeded, classObjectNeededContext, serializedBufferRead, instanceObject, cimErrorDetails);
}


MI_INLINE MI_Result MI_Deserializer_Instance_GetClassName(
    _Inout_ MI_Deserializer *deserializer,
    _In_reads_(serializedBufferLength) MI_Uint8 *serializedBuffer,
    MI_Uint32 serializedBufferLength,
    _Out_writes_opt_(*classNameLength) MI_Char *className,
    _Inout_ MI_Uint32 *classNameLength,
    _Outptr_opt_result_maybenull_ MI_Instance **cimErrorDetails)
{
    return mi_clientFT->deserializerFT->Instance_GetClassName(deserializer, serializedBuffer, serializedBufferLength, className, classNameLength, cimErrorDetails);
}


/* Map an OS specific error code to an error category */
MI_INLINE MI_ErrorCategory MI_Utilities_MapErrorToMiErrorCategory(
        _In_z_ MI_Char *errorType,
        MI_Uint32 error)
{
    return mi_clientFT->utilitiesFT->MapErrorToMiErrorCategory(errorType, error);
}

/* Map an OS specific error code to an CIM error instance.
    * Call MI_Instance_Delete on the CIM when finished
    * using it.
    */
MI_INLINE MI_Result MI_Utilities_CimErrorFromErrorCode(
    MI_Uint32 error,
    _In_z_ const MI_Char *errorType,
    _In_z_ const MI_Char* errorMessage,
    _Outptr_ MI_Instance **cimError)
{
    return mi_clientFT->utilitiesFT->CimErrorFromErrorCode(error, errorType, errorMessage, cimError);

}

/* Mappings from obsolete names to new names */
#define MI_CancelationReason MI_CancellationReason
#define _MI_CancelationReason _MI_CancellationReason
#define MI_PostResult MI_Context_PostResult
#define MI_PostCimError MI_Context_PostCimError
#define MI_PostError MI_Context_PostError
#define MI_PostInstance MI_Context_PostInstance
#define MI_PostIndication MI_Context_PostIndication
#define MI_ConstructInstance MI_Context_ConstructInstance
#define MI_ConstructParameters MI_Context_ConstructParameters
#define MI_NewInstance MI_Context_NewInstance
#define MI_NewDynamicInstance MI_Context_NewDynamicInstance
#define MI_NewParameters MI_Context_NewParameters
#define MI_Canceled MI_Context_Canceled
#define MI_GetLocale MI_Context_GetLocale
#define MI_RegisterCancel MI_Context_RegisterCancel
#define MI_RequestUnload MI_Context_RequestUnload
#define MI_RefuseUnload MI_Context_RefuseUnload
#define MI_GetLocalSession MI_Context_GetLocalSession
#define MI_SetStringOption MI_Context_SetStringOption
#define MI_GetStringOption MI_Context_GetStringOption
#define MI_GetNumberOption MI_Context_GetNumberOption
#define MI_GetCustomOption MI_Context_GetCustomOption
#define MI_GetCustomOptionCount MI_Context_GetCustomOptionCount
#define MI_GetCustomOptionAt MI_Context_GetCustomOptionAt
#define MI_ShouldProcess MI_Context_ShouldProcess
#define MI_ShouldContinue MI_Context_ShouldContinue
#define MI_PromptUser MI_Context_PromptUser
#define MI_WriteError MI_Context_WriteError
#define MI_WriteCimError MI_Context_WriteCimError
#define MI_WriteMessage MI_Context_WriteMessage
#define MI_WriteProgress MI_Context_WriteProgress
#define MI_WriteStreamParameter MI_Context_WriteStreamParameter
#define MI_WriteWarning MI_Context_WriteWarning
#define MI_WriteVerbose MI_Context_WriteVerbose
#define MI_WriteDebug MI_Context_WriteDebug
#define MI_SubscriptionDeliveryOptions__SetExpirationTime MI_SubscriptionDeliveryOptions_SetExpirationTime
#define MI_SubscriptionDeliveryOptions__GetExpirationTime MI_SubscriptionDeliveryOptions_GetExpirationTime


#ifdef __cplusplus
} // end of extern C
#endif   // __cplusplus

#ifdef _PREFAST_
 #pragma prefast(pop)
#endif

#endif /* __MI_C_API_H */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion
