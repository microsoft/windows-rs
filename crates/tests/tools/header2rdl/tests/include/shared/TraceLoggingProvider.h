/* ++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    TraceLoggingProvider.h

Abstract:

    Event Provider API for C/C++.

Environment:

    User mode or kernel mode.

--*/

/*
Quick start:

#include <windows.h> // or <wdm.h> for kernel-mode.
#include <TraceLoggingProvider.h>

TRACELOGGING_DEFINE_PROVIDER( // defines g_hProvider
    g_hProvider,  // Name of the provider variable
    "MyProvider", // Human-readable name of the provider
    (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71)); // Provider GUID

int main(int argc, char* argv[]) // or DriverEntry for kernel-mode.
{
    TraceLoggingRegister(g_hProvider);
    TraceLoggingWrite(
        g_hProvider,
        "MyEvent1",
        TraceLoggingString(argv[0], "arg0"), // field name is "arg0"
        TraceLoggingInt32(argc)); // field name is implicitly "argc"
    TraceLoggingUnregister(g_hProvider);
    return 0;
}

Usage note:

Symbols starting with T are part of the public interface of this header.
Symbols starting with _tlg or _Tlg are private internal implementation
details that are not supported for use outside this header and may change in
future releases.

Compatibility note:

TraceLoggingProvider.h needs to use the EventSetInformation/EtwSetInformation
API on systems where it is available. Depending on its configuration,
TraceLoggingProvider.h can access this API via GetProcAddress (sub-optimal,
but compatible with Vista or later), or can link directly against the needed
API (optimal but only works on OS versions that have the API).
TraceLoggingProvider.h uses WINVER (user mode) and NTDDI_VERSION (kernel mode)
to determine whether it should be compatible with earlier OS versions or be
optimized for newer OS versions.

For user-mode, if you include <windows.h> before setting WINVER, <windows.h>
will set WINVER to the SDK's default target OS version. If WINVER is set to
0x602 or higher, TraceLoggingProvider.h will optimize its behavior for Windows
8 and your program will not run on earlier versions of Windows. If you need
your program to run on Vista or Windows 7, be sure to set WINVER to the
appropriate value before including <windows.h>.

For kernel-mode, if you include <wdm.h> before setting NTDDI_VERSION, <wdm.h>
will set NTDDI_VERSION to a default value. If NTDDI_VERSION is set to
0x06040000 or higher, TraceLoggingProvider.h will optimize its behavior for
Windows 10 and your driver will not work on earlier versions of Windows.

If you need to adjust this behavior, refer to the comments for the
TLG_HAVE_EVENT_SET_INFORMATION macro.
*/

#ifndef _traceloggingprovider_
#define _traceloggingprovider_
#pragma once

#include <evntprov.h>

#if !defined(RC_INVOKED)

#pragma warning(push)
#pragma warning(disable:4055)      // Allow cast from a PVOID to a PFN
#pragma warning(disable:4626)      // assignment operator deleted
#pragma warning(disable:4995 4996) // strlen/wcslen marked as deprecated
#pragma warning(disable:25033)     // Nonconst psz parameter
#if defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC)
#pragma warning(disable:4714) // __forceinline not inlined
#endif

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#pragma region Configuration macros and typedefs

#ifdef _ETW_KM_
typedef PETWENABLECALLBACK     TLG_PENABLECALLBACK;
typedef NTSTATUS               TLG_STATUS;
#else // _ETW_KM_
typedef PENABLECALLBACK        TLG_PENABLECALLBACK;
typedef HRESULT                TLG_STATUS;
#endif // _ETW_KM_

/*
The TLG_HAVE_EVENT_SET_INFORMATION can be set before including this header to
control how the TraceLoggingSetInformation function should behave.
TraceLoggingSetInformation is complex because the underlying Windows API
(EventSetInformation or EtwSetInformation) is not available on all versions of
Windows supported by TraceLoggingProvider. While TraceLoggingProvider.h works
correctly without the SetInformation API, it works better when the
SetInformation API is present. If TLG_HAVE_EVENT_SET_INFORMATION is left
unset, TraceLoggingSetInformation will use a default behavior that works for
most situations.

Note that TraceLoggingSetInformation is called during TraceLoggingRegister.

If both TLG_HAVE_EVENT_SET_INFORMATION and TLG_EVENT_SET_INFORMATION are
undefined, TraceLoggingSetInformation(...) behavior will depend on the Windows
version targeted at compile time, as determined by WINVER (user-mode) or
NTDDI_VERSION (kernel-mode). If the EventSetInformation API is available on
the targeted version of Windows, it will be called directly. Otherwise,
TraceLoggingSetInformation will use GetProcAddress (user-mode) or
MmGetSystemRoutineAddress (kernel-mode) in an attempt to locate the API.

If TLG_HAVE_EVENT_SET_INFORMATION is undefined but TLG_EVENT_SET_INFORMATION
is defined, TraceLoggingSetInformation(...) will invoke
TLG_EVENT_SET_INFORMATION(...).

If TLG_HAVE_EVENT_SET_INFORMATION is set to 0, TraceLoggingSetInformation(...)
will do nothing and will always return a NOT_SUPPORTED error.

If TLG_HAVE_EVENT_SET_INFORMATION is set to 1 and the
TLG_EVENT_SET_INFORMATION macro is not defined,
TraceLoggingSetInformation(...) will directly invoke EventSetInformation(...)
for user-mode or EtwSetInformation(...) for kernel-mode.

If TLG_HAVE_EVENT_SET_INFORMATION is set to 1 and the
TLG_EVENT_SET_INFORMATION macro is defined,
TraceLoggingSetInformation(...) will invoke TLG_EVENT_SET_INFORMATION(...).

If TLG_HAVE_EVENT_SET_INFORMATION is set to 2, TraceLoggingSetInformation(...)
will attempt to load EventSetInformation via GetProcAddress (user-mode) or
will attempt to load EtwSetInformation via MmGetSystemRoutineAddress
(kernel-mode). If the load is successful, it will be invoked. Otherwise,
TraceLoggingSetInformation will return an error.
*/
#ifndef TLG_HAVE_EVENT_SET_INFORMATION
  #if defined(TLG_EVENT_SET_INFORMATION)
    #define TLG_HAVE_EVENT_SET_INFORMATION 1
  #elif defined(_ETW_KM_)
    #if NTDDI_VERSION < 0x06040000
      #define TLG_HAVE_EVENT_SET_INFORMATION 2 // Find "EtwSetInformation" via MmGetSystemRoutineAddress
    #else
      #define TLG_HAVE_EVENT_SET_INFORMATION 1 // Directly invoke TLG_EVENT_SET_INFORMATION(...)
    #endif
  #else // User-mode
    #if WINVER < 0x0602 || !defined(EVENT_FILTER_TYPE_SCHEMATIZED)
      #define TLG_HAVE_EVENT_SET_INFORMATION 2 // Find "EventSetInformation" via GetModuleHandleExW/GetProcAddress
    #else
      #define TLG_HAVE_EVENT_SET_INFORMATION 1 // Directly invoke TLG_EVENT_SET_INFORMATION(...)
    #endif
  #endif // TLG_EVENT_SET_INFORMATION
#endif // TLG_HAVE_EVENT_SET_INFORMATION

/*
Determine whether TraceLoggingWriteEx should be defined.
- If TLG_EVENT_WRITE_EX is defined, TraceLoggingWriteEx will be defined.
- Otherwise, if target is kernel-mode with NTDDI_VERSION >= 0x06010000,
  TraceLoggingWriteEx will be defined.
- Otherwise, if target is user-mode with WINVER >= 0x0601, TraceLoggingWriteEx
  will be defined.
- Otherwise, TraceLoggingWriteEx will not be defined by this header.
*/
#ifdef TLG_EVENT_WRITE_EX
  #define _tlg_ENABLE_TraceLoggingWriteEx 1
#elif defined(_ETW_KM_)
  #define _tlg_ENABLE_TraceLoggingWriteEx NTDDI_VERSION >= 0x06010000
#else
  #define _tlg_ENABLE_TraceLoggingWriteEx WINVER >= 0x0601
#endif

/*
The following override macros can be defined before including this header to
control the APIs used by this header:

- TLG_EVENT_REGISTER
- TLG_EVENT_UNREGISTER
- TLG_EVENT_SET_INFORMATION
- TLG_EVENT_WRITE_TRANSFER
- TLG_EVENT_WRITE_EX

If the the macro is undefined, the TraceLoggingProvider.h implementation will
default to the corresponding ETW APIs. For example, if the TLG_EVENT_REGISTER
macro is undefined, TraceLoggingRegister will use EventRegister in user mode
and will use EtwRegister in kernel mode.

To prevent issues from conflicting definitions of these macros, the value of
the override macro will be used as a suffix in certain internal function names.
Because of this, the override macros must follow certain rules:

- The macro must be defined before TraceLoggingProvider.h is included and must
  not be undefined or redefined after TraceLoggingProvider.h is included.
  Different translation units (i.e. different .c or .cpp files) may set the
  macros to different values, but within a translation unit (within a single
  .c or .cpp file), the macro must be set once and not changed.
- The override must be an object-like macro, not a function-like
  macro (i.e. the override macro must not have a parameter list).
- The override macro's value must be a simple identifier, i.e. must be
  something that starts with a letter or '_' and contains only letters,
  numbers, and '_' characters.
- If the override macro's value is the name of a second object-like macro, the
  second object-like macro must follow the same rules. (The override macro's
  value can also be the name of a function-like macro, in which case the
  function-like macro does not need to follow the same rules.)

For example, the following will cause compile errors:

  #define TLG_EVENT_WRITE_TRANSFER MyNamespace::MyClass::MyFunction // Value has non-identifier characters (colon).
  #define TLG_EVENT_WRITE_TRANSFER GetEventWriteFunctionPointer(7)  // Value has non-identifier characters (parentheses).
  #define TLG_EVENT_WRITE_TRANSFER(h,e,a,r,c,d) EventWrite(h,e,c,d) // Override is defined as a function-like macro.
  #define MY_OBJECT_LIKE_MACRO     MyNamespace::MyClass::MyEventWriteFunction
  #define TLG_EVENT_WRITE_TRANSFER MY_OBJECT_LIKE_MACRO // Evaluates to something with non-identifier characters (colon).

The following would be ok:

  #define TLG_EVENT_WRITE_TRANSFER  MyEventWriteFunction1  // OK, suffix will be "MyEventWriteFunction1".
  #define MY_OBJECT_LIKE_MACRO      MyEventWriteFunction2
  #define TLG_EVENT_WRITE_TRANSFER  MY_OBJECT_LIKE_MACRO   // OK, suffix will be "MyEventWriteFunction2".
  #define MY_FUNCTION_LIKE_MACRO(h,e,a,r,c,d) MyNamespace::MyClass::MyEventWriteFunction3(h,e,c,d)
  #define TLG_EVENT_WRITE_TRANSFER  MY_FUNCTION_LIKE_MACRO // OK, suffix will be "MY_FUNCTION_LIKE_MACRO".
*/
#ifdef _ETW_KM_
#ifndef   TLG_EVENT_REGISTER
  #define TLG_EVENT_REGISTER        EtwRegister
#endif
#ifndef   TLG_EVENT_UNREGISTER
  #define TLG_EVENT_UNREGISTER      EtwUnregister
#endif
#ifndef   TLG_EVENT_SET_INFORMATION
  #define TLG_EVENT_SET_INFORMATION EtwSetInformation
#endif
#ifndef   TLG_EVENT_WRITE_TRANSFER
  #define TLG_EVENT_WRITE_TRANSFER  EtwWriteTransfer
#endif
#ifndef   TLG_EVENT_WRITE_EX
  #define TLG_EVENT_WRITE_EX        EtwWriteEx
#endif
#else // _ETW_KM_
#ifndef   TLG_EVENT_REGISTER
  #define TLG_EVENT_REGISTER        EventRegister
#endif
#ifndef   TLG_EVENT_UNREGISTER
  #define TLG_EVENT_UNREGISTER      EventUnregister
#endif
#ifndef   TLG_EVENT_SET_INFORMATION
  #define TLG_EVENT_SET_INFORMATION EventSetInformation
#endif
#ifndef   TLG_EVENT_WRITE_TRANSFER
  #define TLG_EVENT_WRITE_TRANSFER  EventWriteTransfer
#endif
#ifndef   TLG_EVENT_WRITE_EX
  #define TLG_EVENT_WRITE_EX        EventWriteEx
#endif
#endif // _ETW_KM_

#ifndef TLG_PAGED_CODE
  #ifdef _ETW_KM_
    #define TLG_PAGED_CODE()   PAGED_CODE()
  #else // _ETW_KM_
    #define TLG_PAGED_CODE()   ((void)0)
  #endif // _ETW_KM_
#endif // TLG_PAGED_CODE

#ifndef TLG_INLINE
  #ifndef __cplusplus
    #define TLG_INLINE __inline
  #else
    #define TLG_INLINE inline
  #endif // __cplusplus
#endif

#ifndef TLG_NULL
  #ifndef __cplusplus
    #define TLG_NULL NULL
  #else
    #define TLG_NULL nullptr
  #endif // __cplusplus
#endif // TLG_NULL

#ifndef TLG_DEBUG
  #if (DBG || defined(DEBUG) || defined(_DEBUG)) && !defined(NDEBUG)
    #define TLG_DEBUG 1
  #else // DBG
    #define TLG_DEBUG 0
  #endif // DBG
#endif // TLG_DEBUG

#ifndef TLG_RAISEASSERTIONFAILURE
  #if defined(__clang__) || defined(_M_CEE)
    #define TLG_RAISEASSERTIONFAILURE() __debugbreak()
  #else
    #define TLG_RAISEASSERTIONFAILURE() DbgRaiseAssertionFailure()
  #endif
#endif // TLG_RAISEASSERTIONFAILURE

#ifndef TLG_PFORCEINLINE
  #if defined(PFORCEINLINE)
    #define TLG_PFORCEINLINE PFORCEINLINE
  #elif defined(FORCEINLINE)
    #define TLG_PFORCEINLINE FORCEINLINE
  #else
    #define TLG_PFORCEINLINE __forceinline
  #endif
#endif // TLG_PFORCEINLINE

#ifdef __cplusplus
  #define _tlg_EXTERN_C       extern "C"
  #define _tlg_EXTERN_C_CONST extern "C" const
#else
  #define _tlg_EXTERN_C       extern // In C, linkage is always "C".
  #define _tlg_EXTERN_C_CONST const  // In C, extern with initializer is wrong.
#endif // __cplusplus

#define _tlg_CALL __stdcall

/*
Internal implementation detail: Handler macros.
Wrapper macros (e.g. TraceLoggingInt32) are defined in terms of a handler.
The following macros are the first stage in unwrapping the handler parameters.
The unwrapping performed here is somewhat trivial and could be pushed into the
wrapper macro, but defining these handler macros as a separate layer provides
some IntelliSense and code documentation benefits.
NOTE: Do not call these directly. These macros may change or be renamed in
future versions of this header. Instead, call a public wrapper macro such as
TraceLoggingInt32 or TraceLoggingString.
*/
#define _tlgArgAuto(              value,                               ...) /* for TraceLoggingValue. */ \
          (_tlgAuto,              value,                                        _tlgNDT(value, __VA_ARGS__))
#define _tlgArgScalarVal(  ctype, value,              inType, outType, ...) /* for by-value scalar. */ \
          (_tlgScalarVal,  ctype, value,              inType, _tlgExpandType(outType), _tlgNDT(value, __VA_ARGS__))
#define _tlgArgScalarRef(  ctype, value,              inType, outType, ...) /* for by-ref scalar (in C, must be an lvalue). */ \
          (_tlgScalarRef,  ctype, value,              inType, _tlgExpandType(outType), _tlgNDT(value, __VA_ARGS__))
#define _tlgArgSid(        ctype, pValue,                              ...) /* for SID*. */ \
          (_tlgSid,        ctype, pValue,                                       _tlgNDT(pValue, __VA_ARGS__))
#define _tlgArgPsz(        ctype, pszValue,           inType, outType, ...) /* for nul-terminated string. */ \
          (_tlgPsz,        ctype, pszValue,           inType, _tlgExpandType(outType), _tlgNDT(pszValue, __VA_ARGS__))
#define _tlgArgBuffer(     ctype, pValue,             inType, outType, ...) /* for UNICODE_STRING*, STRING*, etc. */ \
          (_tlgBuffer,     ctype, pValue,             inType, _tlgExpandType(outType), _tlgNDT(pValue, __VA_ARGS__))
#define _tlgArgArray(      ctype, pValues,  cValues,  inType, outType, ...) /* for variable-length array with count of elements. */ \
          (_tlgArray,      ctype, pValues,  cValues,  inType, _tlgExpandType(outType), _tlgNDT(pValues, __VA_ARGS__))
#define _tlgArgBinary(     ctype, pValue,   cbValue,  inType, outType, ...) /* for variable-length array with count of bytes. */ \
          (_tlgBinary,     ctype, pValue,   cbValue,  inType, _tlgExpandType(outType), _tlgNDT(pValue, __VA_ARGS__))
#define _tlgArgCounted(    ctype, pchValue, cchValue, inType, outType, ...) /* for counted strings. */ \
          (_tlgCounted,    ctype, pchValue, cchValue, inType, _tlgExpandType(outType), _tlgNDT(pchValue, __VA_ARGS__))
#define _tlgArgFixedArray( ctype, pValues,  cValues,  inType, outType, ...) /* for fixed-length array with count of elements. */ \
          (_tlgFixedArray, ctype, pValues,  cValues,  inType, _tlgExpandType(outType), _tlgNDT(pValues, __VA_ARGS__))
#define _tlgArgPackedField(ctype, pValue,   cbValue,  inType, outType, ...) /* for user-marshalled data and metadata. */ \
          (_tlgPackedField,ctype, pValue,   cbValue,  inType, _tlgExpandType(outType), _tlgNDT(pValue, __VA_ARGS__))
#define _tlgArgPackedMeta(        name,               inType, outType, ...) /* for user-marshalled metadata. */ \
          (_tlgPackedMeta,                            inType, _tlgExpandType(outType), _tlgDT(name, __VA_ARGS__))
#define _tlgArgPackedData( ctype, pValue,   cbValue, dataDescType         ) /* for user-marshalled data. */ \
          (_tlgPackedData, ctype, pValue,   cbValue, _tlgExpandType(dataDescType))
#define _tlgArgCustom(     ctype, pValue,   cbValue,  protocol,bSchema,cbSchema,...) /* for user-serialized data and schema. */ \
          (_tlgCustom,     ctype, pValue,   cbValue,  protocol,bSchema,cbSchema,_tlgNDT(pValue, __VA_ARGS__))
#define _tlgArgStruct(     fieldCount, name,          inType,          ...) /* for struct and array of struct. */ \
          (_tlgStruct,     fieldCount,                inType,                   _tlgDT(name, __VA_ARGS__))
#define _tlgArgChannel(    eventChannel)                                    /* for TraceLoggingChannel. */ \
          (_tlgChannel,    eventChannel)
#define _tlgArgLevel(      eventLevel)                                      /* for TraceLoggingLevel. */ \
          (_tlgLevel,      eventLevel)
#define _tlgArgOpcode(     eventOpcode)                                     /* for TraceLoggingOpcode. */ \
          (_tlgOpcode,     eventOpcode)
#define _tlgArgKeyword(    eventKeyword)                                    /* for TraceLoggingKeyword. */ \
          (_tlgKeyword,    eventKeyword)
#define _tlgArgTag(        eventTag)                                        /* for TraceLoggingEventTag. */ \
          (_tlgTag,        eventTag)
#define _tlgArgDesc(       eventDescription)                                /* for TraceLoggingDescription. */ \
          (_tlgDesc,       eventDescription)
#define _tlgArgCustomAnnot(key, value)                                      /* for TraceLoggingCustomAttribute. */ \
          (_tlgCustomAnnot,key, value)

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_PASTE2(a, b)        _tlg_PASTE2_imp(a, b)
#define _tlg_PASTE2_imp(a, b)    a##b

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_PASTE3(a, b, c)     _tlg_PASTE3_imp(a, b, c)
#define _tlg_PASTE3_imp(a, b, c) a##b##_##c

// Helpers for override macro validation.
typedef void TLG_EVENT_REGISTER_must_not_be_a_functionLike_macro_TLG_EVENT_REGISTER;
typedef void TLG_EVENT_UNREGISTER_must_not_be_a_functionLike_macro_TLG_EVENT_UNREGISTER;
typedef void TLG_EVENT_SET_INFORMATION_must_not_be_a_functionLike_macro_TLG_EVENT_SET_INFORMATION;
typedef void TLG_EVENT_WRITE_TRANSFER_must_not_be_a_functionLike_macro_TLG_EVENT_WRITE_TRANSFER;
typedef void TLG_EVENT_WRITE_EX_must_not_be_a_functionLike_macro_TLG_EVENT_WRITE_EX;

/*
Validate override macros - trigger compile errors if the override macros do
not follow the rules. (The full list of rules and some examples are given near
the top of this header.)
*/

// Trigger an error if TLG_EVENT_REGISTER is not an unqualified (simple) identifier:
struct _tlg_PASTE2(TLG_EVENT_REGISTER_definition_must_be_an_unqualified_identifier_, TLG_EVENT_REGISTER);

// Trigger an error if TLG_EVENT_REGISTER is defined as a function-like macro:
typedef int _tlg_PASTE2(TLG_EVENT_REGISTER_must_not_be_a_functionLike_macro_, TLG_EVENT_REGISTER);

// Trigger an error if TLG_EVENT_UNREGISTER is not an unqualified (simple) identifier:
struct _tlg_PASTE2(TLG_EVENT_UNREGISTER_definition_must_be_an_unqualified_identifier_, TLG_EVENT_UNREGISTER);

// Trigger an error if TLG_EVENT_UNREGISTER is defined as a function-like macro:
typedef int _tlg_PASTE2(TLG_EVENT_UNREGISTER_must_not_be_a_functionLike_macro_, TLG_EVENT_UNREGISTER);

// Trigger an error if TLG_EVENT_SET_INFORMATION is not an unqualified (simple) identifier:
struct _tlg_PASTE2(TLG_EVENT_SET_INFORMATION_definition_must_be_an_unqualified_identifier_, TLG_EVENT_SET_INFORMATION);

// Trigger an error if TLG_EVENT_SET_INFORMATION is defined as a function-like macro:
typedef int _tlg_PASTE2(TLG_EVENT_SET_INFORMATION_must_not_be_a_functionLike_macro_, TLG_EVENT_SET_INFORMATION);

// Trigger an error if TLG_EVENT_WRITE_TRANSFER is not an unqualified (simple) identifier:
struct _tlg_PASTE2(TLG_EVENT_WRITE_TRANSFER_definition_must_be_an_unqualified_identifier_, TLG_EVENT_WRITE_TRANSFER);

// Trigger an error if TLG_EVENT_WRITE_TRANSFER is defined as a function-like macro:
typedef int _tlg_PASTE2(TLG_EVENT_WRITE_TRANSFER_must_not_be_a_functionLike_macro_, TLG_EVENT_WRITE_TRANSFER);

// Trigger an error if TLG_EVENT_WRITE_EX is not an unqualified (simple) identifier:
struct _tlg_PASTE2(TLG_EVENT_WRITE_EX_definition_must_be_an_unqualified_identifier_, TLG_EVENT_WRITE_EX);

// Trigger an error if TLG_EVENT_WRITE_EX is defined as a function-like macro:
typedef int _tlg_PASTE2(TLG_EVENT_WRITE_EX_must_not_be_a_functionLike_macro_, TLG_EVENT_WRITE_EX);

#if TLG_HAVE_EVENT_SET_INFORMATION == 1
  #define _tlg_EVENT_SET_INFORMATION_suffix   TLG_EVENT_SET_INFORMATION
#elif TLG_HAVE_EVENT_SET_INFORMATION == 2
  #ifdef _ETW_KM_
    #define _tlg_EVENT_SET_INFORMATION_suffix 2K
  #else // _ETW_KM_
    #define _tlg_EVENT_SET_INFORMATION_suffix 2U
  #endif // _ETW_KM_
#else // TLG_HAVE_EVENT_SET_INFORMATION == 0
  #define _tlg_EVENT_SET_INFORMATION_suffix   0
#endif // TLG_HAVE_EVENT_SET_INFORMATION

/*
Determine the full (ODR-safe) name for tracelogging functions that are affected
by macro overrides.
*/
#define TraceLoggingUnregister     _tlg_PASTE2(TraceLoggingUnregister_,  TLG_EVENT_UNREGISTER)
#define TraceLoggingRegister       _tlg_PASTE3(TraceLoggingRegister_,    TLG_EVENT_REGISTER, _tlg_EVENT_SET_INFORMATION_suffix)
#define TraceLoggingRegisterEx     _tlg_PASTE3(TraceLoggingRegisterEx_,  TLG_EVENT_REGISTER, _tlg_EVENT_SET_INFORMATION_suffix)
#define TraceLoggingSetInformation _tlg_PASTE2(TraceLoggingSetInformation_, _tlg_EVENT_SET_INFORMATION_suffix)
#define _tlgWriteTransfer          _tlg_PASTE2(_tlgWriteTransfer_,       TLG_EVENT_WRITE_TRANSFER)
#define _tlgWriteEx                _tlg_PASTE2(_tlgWriteEx_,             TLG_EVENT_WRITE_EX)

#pragma endregion

#pragma region Public interface

/*
Typedef TraceLoggingHProvider:
The type of a provider handle.
To define a provider handle, use TRACELOGGING_DEFINE_PROVIDER.
To forward-declare a provider handle, use TRACELOGGING_DECLARE_PROVIDER.

Notes:

The _tlgProvider_t type is an internal implementation detail. Do not use the
_tlgProvider_t type directly, as it may be changed or renamed in future
versions of this header. Use the TraceLoggingHProvider type and the provided
accessor functions instead of reading members directly.

The scope of a TraceLogging provider handle is strictly limited to the module
(the DLL or EXE) in which it is defined. The provider handle from one module
must not be shared with code in other modules. In other words, for any handle
g_hProvider, all instances of TraceLoggingWrite(g_hProvider, ...) must reside
in the same EXE or DLL as the TRACELOGGING_DEFINE_PROVIDER(g_hProvider, ...)
definition. If this rule is not followed, the behavior of TraceLoggingWrite is
unpredictable.

The TraceLoggingHProvider type should almost never need to be used directly.
In most cases, the provider handle should be a global variable that is defined
and declared using TRACELOGGING macros. In most cases, all uses of the
provider handle will refer directly to this global variable (no copies of the
global variable are made), so user-defined variables of type
TraceLoggingHProvider should usually be unnecessary. (Using the global variable
directly also results in the most efficient code and automatically avoids the
scoping problem described above.)
*/
typedef struct _tlgProvider_t const* TraceLoggingHProvider;

/*
Macro TRACELOGGING_DEFINE_PROVIDER(hProviderVariableName, "ProviderName", providerId, [option]):
Invoke this macro to allocate static storage for a provider and define a
TraceLoggingHProvider variable that references the storage.

An invocation of

    TRACELOGGING_DEFINE_PROVIDER(
        g_hMyProvider,
        "MyProvider",
        (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71));

can be thought of as expanding to something like this:

    static [ProviderStateType] uniqueVarName = { ... }; // stores provider state
    extern "C" const TraceLoggingHProvider g_hMyProvider = &uniqueVarName; // defines a handle

The "ProviderName" specifies a name that identifies the provider in the logged
events. It must be a char string literal (not a variable) and must not contain
any '\0' characters. It must be specific enough to distinguish your provider
from other providers on the system. Typically, provider names use a hierarchy
for unique identification, e.g. "MyCompany.MyOrganization.MyComponent".

The providerId specifies the ETW provider GUID. This GUID must be linked with
the "ProviderName", i.e. the name should never be used with any other GUID, and
the GUID should never be used with any other name. While it is acceptable to
generate the GUID using guidgen or another UUID generation tool, we recommend
that the GUID be generated as a hash of the name using the algorithm shown in
this C# code:

    static Guid ProviderIdFromName(string name)
    {
        var signature = new byte[] {
            0x48, 0x2C, 0x2D, 0xB2, 0xC3, 0x90, 0x47, 0xC8,
            0x87, 0xF8, 0x1A, 0x15, 0xBF, 0xC1, 0x30, 0xFB };
        var nameBytes = Encoding.BigEndianUnicode.GetBytes(name.ToUpperInvariant());
        using (var sha1 = new SHA1Managed())
        {
            sha1.TransformBlock(signature, 0, signature.Length, null, 0);
            sha1.TransformFinalBlock(nameBytes, 0, nameBytes.Length);
            var hash = sha1.Hash;
            Array.Resize(ref hash, 16);
            hash[7] = (byte)((hash[7] & 0x0F) | 0x50);
            return new Guid(hash);
        }
    }

The providerId parameter must be a parenthesized list of 11 compile-time
constant integers e.g.  (g1, g2, g3, ... g11). The integers will be used to
initialize the provider ID as follows:

    GUID providerId = { g1, g2, g3, g4, g5, g6, g7, g8, g9, g10, g11 };

After the providerId GUID, you may optionally specify a TraceLoggingOption...
macro to configure your provider, e.g.

    TRACELOGGING_DEFINE_PROVIDER(g_hMyProvider, "MyProvider",
        (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71),
        TraceLoggingOptionGroup(0xfaaf2f61, 0x9b26, 0x4591, 0x9b, 0xb1, 0xb9, 0xb8, 0xba, 0xe2, 0xd3, 0x4c));

Note that the provider handle is created in the "unregistered" state. Calls to
TraceLoggingWrite on unregistered handles are no-ops. Call
TraceLoggingRegister to register the handle.

For kernel-mode, be aware that while provider metadata is explicitly stored
into TLG_METADATA_SEGMENT, the uniqueVarName and g_hMyProvider variables are
not explicitly assigned a segment and will use the current implicit segments.
The TRACELOGGING_DEFINE_PROVIDER macro expects the implicit const and data
segments to be configured appropriately (they usually need to be nonpaged). If
this is not the case already, the caller must set the data segment via #pragma
data_seg (for uniqueVarName) and/or the const segment via #pragma const_seg
(for g_hMyProvider) before invoking the TRACELOGGING_DEFINE_PROVIDER macro.
*/
#define TRACELOGGING_DEFINE_PROVIDER(handleVariable, providerName, providerId, ...) \
    _tlg_DefineProvider_annotation(handleVariable, _Tlg##handleVariable##Prov, 1, providerName); \
    _tlgProviderStorage_imp(_Tlg##handleVariable##Prov, providerName, providerId, 1, __VA_ARGS__); \
    _tlg_EXTERN_C_CONST TraceLoggingHProvider handleVariable = &_Tlg##handleVariable##Prov

/*
Macro TRACELOGGING_DEFINE_PROVIDER_STORAGE(storageVariable, "ProviderName", providerId, [option]):
Advanced scenarios only: Use this macro to help create a provider handle
when additional control is needed over the storage of the provider's data.
The provider name must be a string literal (not a variable) and must not
contain any '\0' characters.

Typical usage:

    TRACELOGGING_DEFINE_PROVIDER_STORAGE(myProviderStorage, "MyProvider",
        (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71));
    const TraceLoggingHProvider g_hMyProvider = &myProviderStorage;

The above example can be thought of as expanding to something like this:

    static [ProviderStateType] myProviderStorage = { ... };
    const TraceLoggingHProvider g_hMyProvider = &myProviderStorage;

Refer to the documentation for TRACELOGGING_DEFINE_PROVIDER for the syntax of
the providerId and option parameters.

For kernel-mode, be aware that while provider metadata is explicitly stored
into TLG_METADATA_SEGMENT, the myProviderStorage variable is not explicitly
assigned a segment and will use the current implicit segment. The
TRACELOGGING_DEFINE_PROVIDER_STORAGE macro expects the implicit data segment to
be configured appropriately (it usually needs to be nonpaged). If this is not
the case already, the caller must set the data segment via #pragma data_seg for
myProviderStorage before invoking the TRACELOGGING_DEFINE_PROVIDER_STORAGE
macro.
*/
#define TRACELOGGING_DEFINE_PROVIDER_STORAGE(storageVariable, providerName, providerId, ...) \
    _tlgProviderStorage_imp(storageVariable, providerName, providerId, 0, __VA_ARGS__)

/*
Macro TRACELOGGING_DECLARE_PROVIDER(hProviderVariableName):
Invoke this macro to forward-declare a TraceLoggingHProvider variable that
will be defined by TRACELOGGING_DEFINE_PROVIDER. This is for declaring a global
provider handle variable in a header file.

An invocation of

    TRACELOGGING_DECLARE_PROVIDER(g_hMyProvider);

can be thought of as expanding to something like this:

    extern "C" const TraceLoggingHProvider g_hMyProvider;

A variable declared by TRACELOGGING_DECLARE_PROVIDER must later be defined
using the TRACELOGGING_DEFINE_PROVIDER macro.
*/
#define TRACELOGGING_DECLARE_PROVIDER(handleVariable) \
    _tlg_EXTERN_C const TraceLoggingHProvider handleVariable

/*
Macro TraceLoggingOptionGroup(g1, g2, g3, g4, g5, g6, g7, g8, g9, g10, g11):
Advanced scenarios: Wrapper macro for use in TRACELOGGING_DEFINE_PROVIDER that
declares the provider's membership in a provider group (see
https://learn.microsoft.com/windows/win32/etw/provider-traits#provider-groups).
A provider can be a member of no more than one group. The semantics of group
membership are determined by ETW controllers that make use of the group
membership information to configure providers via their group.

The parameters to this macro are 11 compile-time constant integers that will
form the group GUID. The GUID will be initialized as follows:

    GUID groupId = { g1, g2, g3, g4, g5, g6, g7, g8, g9, g10, g11 };

Example:

    TRACELOGGING_DEFINE_PROVIDER(
        g_hMyProvider,
        "MyProvider",
        (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71),
        TraceLoggingOptionGroup(0xfaaf2f61, 0x9b26, 0x4591, 0x9b, 0xb1, 0xb9, 0xb8, 0xba, 0xe2, 0xd3, 0x4c));
*/
#define TraceLoggingOptionGroup(g1, g2, g3, g4, g5, g6, g7, g8, g9, g10, g11) \
    TraceLoggingOptionTrait(_TlgOptionGroup, GUID, ({ g1, g2, g3, { g4, g5, g6, g7, g8, g9, g10, g11 } }))

/*
Macro TraceLoggingOptionTrait(traitType, ctype, value):
Advanced scenarios: Wrapper macro for use in TRACELOGGING_DEFINE_PROVIDER that
adds a custom provider trait to a provider definition (see
https://learn.microsoft.com/en-us/windows/win32/etw/provider-traits#custom-traits).

Parameters:

- traitType: Compile-time constant integer value from 128-255 indicating the
  user-defined trait type to use for the trait.

- ctype: C/C++ type to be used for the trait value, e.g. UINT32 or GUID.

- value: Compile-time constant expression that initializes the trait value,
  enclosed in parenthesis, e.g. (32) or ({ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11}).

Example:

    TRACELOGGING_DEFINE_PROVIDER(
        g_hMyProvider,
        "MyProvider",
        (0xb3864c38, 0x4273, 0x58c5, 0x54, 0x5b, 0x8b, 0x36, 0x08, 0x34, 0x34, 0x71),
        TraceLoggingOptionTrait(128, UINT32, ( 45 ));
*/
#define TraceLoggingOptionTrait(traitType, ctype, value) \
    (_tlgOption, traitType, ctype, value)

/*
Function TraceLoggingUnregister(hProvider):
Call this function to unregister your provider. Normally you will register at
component startup and unregister at component stop.

Note that unregistration is important, especially in the case of a DLL (or
driver) that might be dynamically unloaded before the process ends (or the
system shuts down). TraceLoggingRegister sets up an ETW callback. If the DLL
unloads without calling TraceLoggingUnregister, the ETW callback will cause a
crash.

Thread safety: TraceLoggingRegister/TraceLoggingUnregister are NOT thread-safe
and registration is NOT reference counted.
- Registering an already-registered handle is an error and can cause crashes.
- Calling TraceLoggingRegister(h1) or TraceLoggingUnregister(h1) on one thread
  while another thread is in the middle of a call to TraceLoggingRegister(h1),
  TraceLoggingUnregister(h1), or TraceLoggingSetInformation(h1) is an error
  and can cause crashes.

Calling TraceLoggingUnregister on an unregistered handle is a safe no-op.

After unregistering a provider, it is ok to register it again. In other words,
the following is ok:

    TRACELOGGING_DECLARE_PROVIDER(hProvider);
    ...
    TraceLoggingRegister(hProvider);
    ...
    TraceLoggingUnregister(hProvider);
    ...
    TraceLoggingRegister(hProvider);
    ...
    TraceLoggingUnregister(hProvider);

However, re-registering a provider should only happen because a component
has been uninitialized and then reinitialized. You should not register and
unregister a provider each time you need to write a few events.
*/
_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
void _tlg_CALL
TraceLoggingUnregister(
    TraceLoggingHProvider _Inout_ hProvider);

/*
Function TraceLoggingRegister(hProvider):
Call this function to register your provider with ETW.

The provider handle must be in the "unregistered" state (i.e. registration is
not reference counted, and registering an already-registered handle is a
serious error).

In user-mode, returns an HRESULT code. Use SUCCEEDED to check success.
In kernel-mode, returns an NTSTATUS code. Use NT_SUCCESS to check success.

Calling TraceLoggingRegister(hProvider) is the same as calling
TraceLoggingRegisterEx(hProvider, NULL, NULL).

Refer to the documentation in TraceLoggingUnregister for additional information
about registration, especially regarding thread safety.

Note that it is ok to ignore failure - if TraceLoggingRegister fails,
TraceLoggingWrite and TraceLoggingUnregister will be safe no-ops. The returned
error code is typically only useful during development (i.e. for detection and
diagnosis of programming errors). In production, errors from
TraceLoggingRegister should usually be ignored unless event logging is
security-critical.
*/
_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
TraceLoggingRegister(
    TraceLoggingHProvider _Inout_ hProvider);

/*
Function TraceLoggingRegisterEx(hProvider, pEnableCallback, pCallbackContext):
Call this function to register your provider with ETW if you need callbacks
(advanced scenarios only).

The provider handle must be in the "unregistered" state (i.e. registration is
not reference counted, and registering an already-registered handle is a
serious error).

In user-mode, returns an HRESULT code. Use SUCCEEDED to check success.
In kernel-mode, returns an NTSTATUS code. Use NT_SUCCESS to check success.

Refer to the documentation in TraceLoggingUnregister for additional information
about registration, especially regarding thread safety.

Note that it is ok to ignore failure - if TraceLoggingRegisterEx fails,
TraceLoggingWrite and TraceLoggingUnregister will be safe no-ops. The returned
error code is typically only useful during development (i.e. for detection and
diagnosis of programming errors). In production, errors from
TraceLoggingRegister should usually be ignored unless event logging is
security-critical.
*/
_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
TraceLoggingRegisterEx(
    TraceLoggingHProvider _Inout_ hProvider,
    _In_opt_ TLG_PENABLECALLBACK pEnableCallback,
    _In_opt_ PVOID pCallbackContext);

/*
Function TraceLoggingSetInformation(hProvider, informationClass, pvInformation, cbInformation):
Call this function to provide special-purpose information to the ETW runtime.
For details, see the corresponding ETW API: EventSetInformation.

Note that the EventSetInformation API is not available on all versions of
Windows. This wrapper has configurable behavior for how it should access the
EventSetInformation API. The default behavior of this function depends on the
value of WINVER (user-mode) or NTDDI_VERSION (kernel-mode). Use the
TLG_HAVE_EVENT_SET_INFORMATION macro to override the default behavior.
*/
_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
TraceLoggingSetInformation(
    TraceLoggingHProvider _Inout_ hProvider,
    _In_ EVENT_INFO_CLASS informationClass,
    _In_reads_bytes_opt_(cbInformation) PVOID pvInformation,
    _In_ ULONG cbInformation);

/*
Function TraceLoggingProviderEnabled(hProvider, eventLevel, eventKeyword):
Call this function to determine whether the given provider is enabled for
events with the specified level and keyword. Use 0 for eventLevel to determine
whether the provider is enabled for any level. Use 0 for eventKeyword to
determine whether the provider is enabled for any keyword.

This function can be used to avoid doing extra work before calling
TraceLoggingWrite. For example, you might use TraceLoggingProviderEnabled for
something like this:

    if (TraceLoggingProviderEnabled(provider, level, keyword))
    {
        string expensiveValue = ComputeExpensiveValue();
        TraceLoggingWriteImpl(provider, name, level, keyword, expensiveValue);
    }

Do not use TraceLoggingProviderEnabled simply to avoid calling
TraceLoggingWrite. TraceLoggingWrite performs its own test, i.e. a
TraceLoggingWrite(provider, name, level, keyword, args...) already includes a
check, approximately like:

    if (TraceLoggingProviderEnabled(provider, level, keyword))
    {
        TraceLoggingWriteImpl(provider, name, level, keyword, args...);
    }

In this case, making your own call to TraceLoggingProviderEnabled would be
redundant.
*/
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
BOOLEAN _tlg_CALL
TraceLoggingProviderEnabled(
    TraceLoggingHProvider _In_ hProvider,
    UCHAR eventLevel,
    ULONGLONG eventKeyword);

/*
Function TraceLoggingProviderId(hProvider):
Returns the provider ID (GUID) that was specified in
TRACELOGGING_DEFINE_PROVIDER.
*/
_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
GUID _tlg_CALL
TraceLoggingProviderId(
    TraceLoggingHProvider _In_ hProvider);

/*
Macro TraceLoggingWrite(hProvider, "EventName", args...):
Invoke this macro to log an event.

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingInt32(myIntVar),
        TraceLoggingWideString(myString));

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Supports up to 99 args (subject to compiler limitations). Each arg must be a
wrapper macro such as TraceLoggingLevel, TraceLoggingKeyword, TraceLoggingInt32,
TraceLoggingString, etc.

Note that TraceLoggingWrite(hProvider, eventName, args...) is a macro that
checks whether the provider is enabled before evaluating args... and before
writing the data to ETW. In other words, an event like
TraceLoggingWrite(hProvider, eventName, TraceLoggingLevel(level), args...)
expands to an implementation that works as follows:

    if (TraceLoggingProviderEnabled(hProvider, level, keyword))
    {
        TraceLoggingWriteImpl(hProvider, eventName,
            TraceLoggingLevel(level), args...);
    }

Note that TraceLoggingWrite(hProvider, eventName, args...) is equivalent to
TraceLoggingWriteActivity(hProvider, eventName, NULL, NULL, args...).
- The resulting event will use the current thread's implicit activity ID.
- The resulting event will not include a "related activity ID" attribute.
*/
#define TraceLoggingWrite(hProvider, eventName, ...) \
    _tlgWrite_imp(_tlgWriteTransfer, \
    hProvider, eventName, \
    (LPCGUID, LPCGUID), \
    (TLG_NULL, TLG_NULL), \
    __VA_ARGS__)

/*
Macro TraceLoggingWriteActivity(hProvider, "EventName", pActivityId, pRelatedId, args...):
Invoke this macro to log an event with explicitly-specified activity GUIDs.

Note that pActivityId can be NULL, which will cause the resulting event to use
the current thread's implicit activity ID.

Note that pRelatedActivityId can be NULL, which will cause the resulting event
to not include a "related activity ID" attribute. In practice, only
activity-start events (Opcode=START) should set the "related activity ID" to
a non-NULL ID.

Example:

    TraceLoggingWriteActivity(
        g_hProvider,
        "MyEventName",
        pActivityId,
        pRelatedActivityId,
        TraceLoggingInt32(myIntVar),
        TraceLoggingWideString(myString));

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Supports up to 99 args (subject to compiler limitations). Each arg must be a
wrapper macro such as TraceLoggingLevel, TraceLoggingKeyword, TraceLoggingInt32,
TraceLoggingString, etc.
*/
#define TraceLoggingWriteActivity(hProvider, eventName, pActivityId, pRelatedActivityId, ...) \
    _tlgWrite_imp(_tlgWriteTransfer, \
    hProvider, eventName, \
    (LPCGUID, LPCGUID), \
    ((pActivityId), (pRelatedActivityId)), \
    __VA_ARGS__)

#if _tlg_ENABLE_TraceLoggingWriteEx

/*
Macro TraceLoggingWriteEx(hProvider, "EventName", filter, flags, pActivityId, pRelatedId, args...):
Advanced scenarios: Invoke this macro to log an event with explicitly-specified
filter, flags, and activity GUIDs. Use this macro when you need to set the
filter or flags parameters (see the documentation of EventWriteEx for details).
Otherwise, use the TraceLoggingWrite and TraceLoggingWriteActivity macros.

This macro is only available if WINVER is set to Windows 7 or later (or if
TLG_EVENT_WRITE_EX has been overridden).

Example:

    TraceLoggingWriteEx(
        g_hProvider,
        "MyEventName",
        filter,             // normally 0 (no filter)
        flags,              // normally 0 (no flags)
        pActivityId,        // normally NULL (implicit thread activity ID)
        pRelatedActivityId, // normally NULL (no related activity ID)
        TraceLoggingInt32(myIntVar),
        TraceLoggingWideString(myString));

The event name must be a string literal (not a variable) and must not contain
any '\0' characters.

Supports up to 99 args (subject to compiler limitations). Each arg must be a
wrapper macro such as TraceLoggingLevel, TraceLoggingKeyword, TraceLoggingInt32,
TraceLoggingString, etc.
*/
#define TraceLoggingWriteEx(hProvider, eventName, filter, flags, pActivityId, pRelatedActivityId, ...) \
    _tlgWrite_imp(_tlgWriteEx, \
    hProvider, eventName, \
    (ULONG64, ULONG, LPCGUID, LPCGUID), \
    ((filter), (flags), (pActivityId), (pRelatedActivityId)), \
    __VA_ARGS__)

#endif // _tlg_ENABLE_TraceLoggingWriteEx

/*
Macro TraceLoggingLevel(eventLevel)
Wrapper macro for setting the event's level.

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingLevel(TRACE_LEVEL_WARNING),
        TraceLoggingWideString(myString));

The eventLevel parameter must be a compile-time constant 0 to 255 (typically
a TRACE_LEVEL_??? constant from evntrace.h or a WINEVENT_LEVEL_??? constant
from winmeta.h). If no TraceLoggingLevel(n) arg is provided, the event will
use logging level 5 (TRACE_LEVEL_VERBOSE). If multiple TraceLoggingLevel(n)
args are provided, the value from the last TraceLoggingLevel(n) will be used.

ETW event level is stored in each event as an 8-bit value from 0 to 255
indicating event severity, with lower values being more severe. Levels 0..5 are
defined by Microsoft as log-always=0, critical-error=1, error=2, warning=3,
info=4, and verbose=5. Levels 6..15 are reserved for future definition by
Microsoft. Levels and keywords are heavily used by the ETW runtime for event
filtering and routing.
*/
#define TraceLoggingLevel(eventLevel) \
    _tlgArgLevel(eventLevel)

/*
Macro TraceLoggingKeyword(eventKeyword):
Wrapper macro for setting the event's keyword(s).

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingKeyword(WINEVENT_KEYWORD_RESPONSE_TIME),
        TraceLoggingWideString(myString));

The eventKeyword parameter must be a compile-time constant 0 to UINT64_MAX. If
no TraceLoggingKeyword(n) arg is provided, the event will use keyword 0
(meaning "log-always"). If multiple TraceLoggingKeyword(n) args are provided,
the parameters are OR'ed together.

ETW event keyword is stored in each event as a 64-bit value. Each bit in the
keyword value corresponds to an event category, with a "1" bit indicating that
the event is included in the category. The 16 most-significant bits of the
keyword (bits 48..63) correspond to Microsoft-defined categories. The 48
least-significant bits of the keyword (bits 0..47) are user-defined categories,
i.e. the developer might decide that within a particular component, keyword
bit 0 (keyword value 0x1) indicates "networking", and keyword bit 3 (keyword
value 0x8) indicates "UI", so keyword value 0x9 would mean the event is in both
"networking" and "UI" categories. As a special exception, the keyword value 0
means "log-always", i.e. that the event cannot be disabled by keyword. (Because
of this rule, it is best practice to always assign a non-zero keyword to every
event.)

Levels and keywords are heavily used by the ETW runtime for event filtering and
routing.
*/
#define TraceLoggingKeyword(eventKeyword) \
    _tlgArgKeyword(eventKeyword)

/*
Macro TraceLoggingOpcode(eventOpcode):
Wrapper macro for setting the event's opcode.

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingOpcode(EVENT_TRACE_TYPE_STOP),
        TraceLoggingWideString(myString));

The eventOpcode parameter must be a compile-time constant 0 to 255 (typically
a EVENT_TRACE_TYPE_??? constant from evntrace.h or a WINEVENT_OPCODE_???
constant from winmeta.h). If no TraceLoggingOpcode(n) arg is provided, the
event will use opcode 0 (EVENT_TRACE_TYPE_INFO). If multiple
TraceLoggingOpcode(n) args are provided, the value from the last
TraceLoggingOpcode(n) will be used.

ETW event opcode is stored in each event as an 8-bit value from 0 to 255
indicating event semantics. Most events use opcode info=0, indicating an
informational event (no special semantics). The most commonly-used opcodes are
start=1 (indicating the start of an activity) and stop=2 (activity stop). The
opcode is not used by the ETW runtime (i.e. opcode does not usually affect
event filtering or routing), but the opcode is important during decoding
(i.e. in construction of activity sequences).
*/
#define TraceLoggingOpcode(eventOpcode) \
    _tlgArgOpcode(eventOpcode)

/*
Macro TraceLoggingChannel(eventChannel)
Wrapper macro for setting the event's channel. (Advanced scenarios.)

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingChannel(WINEVENT_CHANNEL_GLOBAL_APPLICATION),
        TraceLoggingWideString(myString));

The eventChannel parameter must be a compile-time constant 0 to 255. If no
TraceLoggingChannel(n) arg is provided, the default channel is 11
(WINEVENT_CHANNEL_TRACELOGGING), indicating that this is a normal
TraceLogging event. If multiple TraceLoggingChannel(n) args are provided, the
value from the last TraceLoggingChannel(n) is used.

ETW event channel is stored in each event as an 8-bit value from 0 to 255
indicating event semantics. Channel value 0 indicates a normal event.

Channel values 1..15 are reserved by Microsoft and may trigger special
behavior in the ETW runtime or during event decoding. Channel versions 16 and
higher are defined by specific event consumers.

The primary use of channels is with the Windows Event Log. When using channels
with the Event Log, a manifest must be registered with the system to define
the provider and its channels. A manifest for a TraceLogging provider should
define the provider and the channels but should omit the event definitions,
since they are managed by TraceLogging.

NOTE: Do not use TraceLoggingChannel in code that needs to run on older
versions of Windows. The default channel 11 (WINEVENT_CHANNEL_TRACELOGGING)
marks the event as using TraceLogging-based decoding. On Windows 10, or on
Windows 7 with the necessary updates, the ETW runtime will mark the event as
TraceLogging regardless of channel, but on versions of Windows where the ETW
runtime has not been updated with TraceLogging support, the channel is the
only way for the decoder to know that the event was a TraceLogging event. As a
result, events that use a channel other than 11 and are captured on an older
version of Windows will not decode properly.
*/
#define TraceLoggingChannel(eventChannel) \
    _tlgArgChannel(eventChannel)

/*
Macro TraceLoggingEventTag(eventTag):
Wrapper macro for setting the event's tag(s).

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingEventTag(0x200000),
        TraceLoggingWideString(myString));

Tags are a 28-bit user-defined metadata field. The semantics of the tags are
defined by the event consumer. During event processing, this tag can be
retrieved from the TRACE_EVENT_INFO Tags field.

The eventTag parameter must be a compile-time constant, 0x0FFFFFFF or less.
(In C, the low 14 bits of eventTag must all be 0, as only the top 14 bits are
supported.)

If no TraceLoggingEventTag(eventTag) arg is provided, the event's tag value
will be 0. If multiple TraceLoggingEventTag(eventTag) args are provided, the
tag values will be OR'ed together.

Tag encoding is optimized in favor of the high bits, so you should generally
start assigning tag values at 0x08000000 (the highest supported tag bit), then
assign subsequent values to lower bits.

C specific:

The C implementation always uses a 2-byte encoding for tags, supporting up to
14 bits of tag data. The value parameter must be a compile-time constant
covered by mask 0x0FFFC000 (i.e. only tag bits 15-28 are supported).

C++ specific:

The C++ implementation uses a 1-byte, 2-byte, or 4-byte encoding for tags,
depending on tag's value. Tags that leave low bits unset can be encoded more
optimally. For example:

- 0x00000000 uses a 1-byte encoding.
- 0x0FE00000 uses a 1-byte encoding.
- 0x0FFFC000 uses a 2-byte encoding.
- 0x0FFFFFFF uses a 4-byte encoding.

Technical details:

The TraceLogging schema convention encodes tags as 28-bit fields by using a
chain of up to four bytes with the upper-most bit used as a 'chain' bit
(4 bytes * 7 bits per byte = 28 bits supported). Data is encoded
most-significant byte first. The encoding always uses at least 1 byte.
*/
#define TraceLoggingEventTag(eventTag) \
    _tlgArgTag(eventTag)

/*
Macro TraceLoggingDescription(eventDescription):
Wrapper macro for setting a description for an event.

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingDescription("My event's detailed description"),
        TraceLoggingWideString(myString));

The value parameter must be a string literal. If no
TraceLoggingDescription(description) parameter is provided, the default is
empty. If multiple TraceLoggingDescriptions(description) args are provided,
they are concatenated together into a single string.

Descriptions are stored in the debug symbols (PDB file). They are not
available at runtime.
*/
#define TraceLoggingDescription(eventDescription) \
    _tlgArgDesc(eventDescription)

/*
Macro TraceLoggingCustomAttribute(key, value):
Wrapper macro for adding custom information about an event to the debug
symbols (PDB file).

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingCustomAttribute("Key", "Value"),
        TraceLoggingWideString(myString));

Both parameters must be string literals. Multiple custom attributes can be
specified per event.

Custom attributes are stored in the debug symbols (PDB file). They are not
available at runtime.
*/
#define TraceLoggingCustomAttribute(key, value) \
    _tlgArgCustomAnnot(key, value)

/*
Macro TraceLoggingStruct(fieldCount, "structName", "description", tags):
Wrapper macro for defining a group of related fields in an event.

The description and tags parameters are optional.

The fieldCount parameter must be a compile-time constant. It indicates the
number of fields that will be considered to be part of the struct. A struct and
all of its contained fields count as a single field in any parent structs.

The name parameter must be a string literal (not a variable) and must not
contain any '\0' characters.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Example:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingStruct(2, "Name"),
            TraceLoggingWideString(szLast),
            TraceLoggingWideString(szFirst));
*/
#define TraceLoggingStruct(fieldCount, name, ...) \
    _tlgArgStruct(fieldCount, name, _TlgInSTRUCT, __VA_ARGS__)

#if defined(__cplusplus)
/*
Macro TraceLoggingValue(value, "name", "description", tags):
Wrapper macro for event fields. Automatically deduces value type. C++ only.

The name, description, and tags parameters are optional.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the value
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingValue(val1)                      // field name = "val1", description = unset,  tags = 0.
- TraceLoggingValue(val1, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingValue(val1, "name", "desc"       // field name = "name", description = "desc", tags = 0.
- TraceLoggingValue(val1, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.

Based on the type of val, TraceLoggingValue(val, ...) is equivalent to:
- bool               --> TraceLoggingBoolean(val, ...)
- char               --> TraceLoggingChar(val, ...)
- char16_t           --> TraceLoggingChar16(val, ...)
- wchar_t            --> TraceLoggingWChar(val, ...) // only for native wchar_t type, not for USHORT
- intNN_t            --> TraceLoggingIntNN(val, ...)
- uintNN_t           --> TraceLoggingUIntNN(val, ...)
- float              --> TraceLoggingFloat32(val, ...)
- double             --> TraceLoggingFloat64(val, ...)
- GUID               --> TraceLoggingGuid(val, ...)
- FILETIME           --> TraceLoggingFileTime(val, ...)
- SYSTEMTIME         --> TraceLoggingSystemTime(val, ...)
- SID*               --> TraceLoggingSid(val, ...)        // Requires non-NULL, valid SID.
- void*              --> TraceLoggingPointer(val, ...)    // Logs the pointer's value, not the pointed-at data.
- char*              --> TraceLoggingString(val, ...)     // Assumes nul-terminated ANSI string. NULL is the same as "".
- char16_t*          --> TraceLoggingString16(val, ...)   // Assumes nul-terminated utf-16 string. NULL is the same as u"".
- wchar_t*           --> TraceLoggingWideString(val, ...) // Assumes nul-terminated utf-16 string. NULL is the same as L"".
*/
#define TraceLoggingValue(value, ...) \
    _tlgArgAuto(value, __VA_ARGS__)
#endif // __cplusplus

/*
Wrapper macros for event fields with simple scalar values.
Usage: TraceLoggingInt32(value, "name", "description", tags).

In C, a GUID, FILETIME, or SYSTEMTIME value parameter must be an lvalue.

The name, description, and tags parameters are optional.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the value
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Notes:
- TraceLoggingBool is for 32-bit boolean values.
- TraceLoggingBoolean is for 8-bit boolean values.

Examples:
- TraceLoggingInt32(val1)                      // field name = "val1", description = unset,  tags = 0.
- TraceLoggingInt32(val1, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingInt32(val1, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingInt32(val1, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingInt8(value, ...)          _tlgArgScalarVal(INT8,              value, TlgInINT8,      (),                  __VA_ARGS__)
#define TraceLoggingUInt8(value, ...)         _tlgArgScalarVal(UINT8,             value, TlgInUINT8,     (),                  __VA_ARGS__)
#define TraceLoggingInt16(value, ...)         _tlgArgScalarVal(INT16,             value, TlgInINT16,     (),                  __VA_ARGS__)
#define TraceLoggingUInt16(value, ...)        _tlgArgScalarVal(UINT16,            value, TlgInUINT16,    (),                  __VA_ARGS__)
#define TraceLoggingInt32(value, ...)         _tlgArgScalarVal(INT32,             value, TlgInINT32,     (),                  __VA_ARGS__)
#define TraceLoggingUInt32(value, ...)        _tlgArgScalarVal(UINT32,            value, TlgInUINT32,    (),                  __VA_ARGS__)
#define TraceLoggingInt64(value, ...)         _tlgArgScalarVal(INT64,             value, TlgInINT64,     (),                  __VA_ARGS__)
#define TraceLoggingUInt64(value, ...)        _tlgArgScalarVal(UINT64,            value, TlgInUINT64,    (),                  __VA_ARGS__)
#define TraceLoggingIntPtr(value, ...)        _tlgArgScalarVal(INT_PTR,           value, TlgInINTPTR,    (),                  __VA_ARGS__)
#define TraceLoggingUIntPtr(value, ...)       _tlgArgScalarVal(UINT_PTR,          value, TlgInUINTPTR,   (),                  __VA_ARGS__)
#define TraceLoggingLong(value, ...)          _tlgArgScalarVal(LONG,              value, TlgInLONG,      (),                  __VA_ARGS__)
#define TraceLoggingULong(value, ...)         _tlgArgScalarVal(ULONG,             value, TlgInULONG,     (),                  __VA_ARGS__)
#define TraceLoggingHexInt8(value, ...)       _tlgArgScalarVal(INT8,              value, TlgInUINT8,     (TlgOutHEX),         __VA_ARGS__)
#define TraceLoggingHexUInt8(value, ...)      _tlgArgScalarVal(UINT8,             value, TlgInUINT8,     (TlgOutHEX),         __VA_ARGS__)
#define TraceLoggingHexInt16(value, ...)      _tlgArgScalarVal(INT16,             value, TlgInUINT16,    (TlgOutHEX),         __VA_ARGS__)
#define TraceLoggingHexUInt16(value, ...)     _tlgArgScalarVal(UINT16,            value, TlgInUINT16,    (TlgOutHEX),         __VA_ARGS__)
#define TraceLoggingHexInt32(value, ...)      _tlgArgScalarVal(INT32,             value, TlgInHEXINT32,  (),                  __VA_ARGS__)
#define TraceLoggingHexUInt32(value, ...)     _tlgArgScalarVal(UINT32,            value, TlgInHEXINT32,  (),                  __VA_ARGS__)
#define TraceLoggingHexInt64(value, ...)      _tlgArgScalarVal(INT64,             value, TlgInHEXINT64,  (),                  __VA_ARGS__)
#define TraceLoggingHexUInt64(value, ...)     _tlgArgScalarVal(UINT64,            value, TlgInHEXINT64,  (),                  __VA_ARGS__)
#define TraceLoggingHexIntPtr(value, ...)     _tlgArgScalarVal(INT_PTR,           value, TlgInPOINTER,   (),                  __VA_ARGS__)
#define TraceLoggingHexUIntPtr(value, ...)    _tlgArgScalarVal(UINT_PTR,          value, TlgInPOINTER,   (),                  __VA_ARGS__)
#define TraceLoggingHexLong(value, ...)       _tlgArgScalarVal(LONG,              value, TlgInHEXLONG,   (),                  __VA_ARGS__)
#define TraceLoggingHexULong(value, ...)      _tlgArgScalarVal(ULONG,             value, TlgInHEXLONG,   (),                  __VA_ARGS__)
#define TraceLoggingFloat32(value, ...)       _tlgArgScalarVal(float,             value, TlgInFLOAT,     (),                  __VA_ARGS__)
#define TraceLoggingFloat64(value, ...)       _tlgArgScalarVal(double,            value, TlgInDOUBLE,    (),                  __VA_ARGS__)
#define TraceLoggingBoolean(value, ...)       _tlgArgScalarVal(BOOLEAN,           value, TlgInUINT8,     (TlgOutBOOLEAN),     __VA_ARGS__)
#define TraceLoggingBool(value, ...)          _tlgArgScalarVal(INT32,             value, TlgInBOOL32,    (),                  __VA_ARGS__)
#define TraceLoggingChar(value, ...)          _tlgArgScalarVal(char,              value, TlgInUINT8,     (TlgOutSTRING),      __VA_ARGS__)
#define TraceLoggingChar16(value, ...)        _tlgArgScalarVal(char16_t,          value, TlgInUINT16,    (TlgOutSTRING),      __VA_ARGS__)
#define TraceLoggingWChar(value, ...)         _tlgArgScalarVal(wchar_t,           value, TlgInUINT16,    (TlgOutSTRING),      __VA_ARGS__)
#define TraceLoggingPointer(value, ...)       _tlgArgScalarVal(void const*,       value, TlgInPOINTER,   (),                  __VA_ARGS__)
#define TraceLoggingCodePointer(value, ...)   _tlgArgScalarVal(void const*,       value, TlgInPOINTER,   (TlgOutCODE_POINTER),__VA_ARGS__)
#define TraceLoggingPid(value, ...)           _tlgArgScalarVal(UINT32,            value, TlgInUINT32,    (TlgOutPID),         __VA_ARGS__)
#define TraceLoggingTid(value, ...)           _tlgArgScalarVal(UINT32,            value, TlgInUINT32,    (TlgOutTID),         __VA_ARGS__)
#define TraceLoggingPort(value, ...)          _tlgArgScalarVal(UINT16,            value, TlgInUINT16,    (TlgOutPORT),        __VA_ARGS__)
#define TraceLoggingWinError(value, ...)      _tlgArgScalarVal(UINT32,            value, TlgInUINT32,    (TlgOutWIN32ERROR),  __VA_ARGS__)
#define TraceLoggingNTStatus(value, ...)      _tlgArgScalarVal(NTSTATUS,          value, TlgInUINT32,    (TlgOutNTSTATUS),    __VA_ARGS__)
#define TraceLoggingHResult(value, ...)       _tlgArgScalarVal(HRESULT,           value, TlgInINT32,     (TlgOutHRESULT),     __VA_ARGS__)
#define TraceLoggingFileTime(value, ...)      _tlgArgScalarRef(struct _FILETIME,  value, TlgInFILETIME,  (),                  __VA_ARGS__)
#define TraceLoggingFileTimeUtc(value, ...)   _tlgArgScalarRef(struct _FILETIME,  value, TlgInFILETIME,  (TlgOutDATETIME_UTC),__VA_ARGS__)
#define TraceLoggingSystemTime(value, ...)    _tlgArgScalarRef(struct _SYSTEMTIME,value, TlgInSYSTEMTIME,(),                  __VA_ARGS__)
#define TraceLoggingSystemTimeUtc(value, ...) _tlgArgScalarRef(struct _SYSTEMTIME,value, TlgInSYSTEMTIME,(TlgOutDATETIME_UTC),__VA_ARGS__)
#define TraceLoggingGuid(value, ...)          _tlgArgScalarRef(GUID,              value, TlgInGUID,      (),                  __VA_ARGS__)

/*
Wrapper macros for event fields with zero-terminated or counted string values.
Usage: TraceLoggingString(pszValue, "name", "description", tags)     // NUL-terminated char*, uses decoding system's default ANSI code page.
Usage: TraceLoggingUtf8String(pszValue, "name", "description", tags) // NUL-terminated char*, uses UTF-8.
Usage: TraceLoggingString16(pszValue, "name", "description", tags)   // NUL-terminated char16_t*, uses UTF-16LE.
Usage: TraceLoggingWideString(pszValue, "name", "description", tags) // NUL-terminated wchar_t*, uses UTF-16LE.
Usage: TraceLoggingCountedString(pchValue, cchValue, "name", "description", tags)     // Counted char*, uses decoding system's default ANSI code page.
Usage: TraceLoggingCountedUtf8String(pchValue, cchValue, "name", "description", tags) // Counted char*, uses UTF-8.
Usage: TraceLoggingCountedString16(pchValue, cchValue, "name", "description", tags)   // Counted char16_t*, uses UTF-16LE.
Usage: TraceLoggingCountedWideString(pchValue, cchValue, "name", "description", tags) // Counted wchar_t*, uses UTF-16LE.

The name, description, and tags parameters are optional.

For TraceLoggingString, TraceLoggingUtf8String, TraceLoggingString16, and
TraceLoggingWideString, the pszValue parameter is treated as a NUL-terminated
string. If pszValue is NULL, it is treated as an empty (zero-length) string.

For TraceLoggingCountedString, TraceLoggingCountedUtf8String,
TraceLoggingCountedString16, and TraceLoggingCountedWideString, the pchValue
parameter is treated as a counted string, with length cchValue given in
characters. The pchValue parameter may be NULL only if cchValue is 0.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pszValue
or pchValue parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingString(psz1)                      // field name = "psz1", description = unset,  tags = 0.
- TraceLoggingString(psz1, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingString(psz1, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingString(psz1, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingString(pszValue, ...)                      _tlgArgPsz(    char,     pszValue,           TlgInANSISTRING,        (),           __VA_ARGS__)
#define TraceLoggingUtf8String(pszValue, ...)                  _tlgArgPsz(    char,     pszValue,           TlgInANSISTRING,        (TlgOutUTF8), __VA_ARGS__)
#define TraceLoggingString16(pszValue, ...)                    _tlgArgPsz(    char16_t, pszValue,           TlgInUNICODESTRING,     (),           __VA_ARGS__)
#define TraceLoggingWideString(pszValue, ...)                  _tlgArgPsz(    wchar_t,  pszValue,           TlgInUNICODESTRING,     (),           __VA_ARGS__)
#define TraceLoggingCountedString(pchValue, cchValue, ...)     _tlgArgCounted(char,     pchValue, cchValue, TlgInCOUNTEDANSISTRING, (),           __VA_ARGS__)
#define TraceLoggingCountedUtf8String(pchValue, cchValue, ...) _tlgArgCounted(char,     pchValue, cchValue, TlgInCOUNTEDANSISTRING, (TlgOutUTF8), __VA_ARGS__)
#define TraceLoggingCountedString16(pchValue, cchValue, ...)   _tlgArgCounted(char16_t, pchValue, cchValue, TlgInCOUNTEDSTRING,     (),           __VA_ARGS__)
#define TraceLoggingCountedWideString(pchValue, cchValue, ...) _tlgArgCounted(wchar_t,  pchValue, cchValue, TlgInCOUNTEDSTRING,     (),           __VA_ARGS__)

/*
Wrapper macros for event fields with PANSI_STRING, PUNICODE_STRING, etc. values.
Usage: TraceLoggingUnicodeString(pValue, "name", "description", tags).

Use TraceLoggingUnicodeString for PUNICODE_STRING or PCUNICODE_STRING.
Use TraceLoggingAnsiString for PSTRING, PANSI_STRING, PCANSI_STRING, POEM_STRING, etc.

The name, description, and tags parameters are optional.

The pString parameter must not be NULL.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pString
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingUnicodeString(pStr)                      // field name = "pStr", description = unset,  tags = 0.
- TraceLoggingUnicodeString(pStr, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingUnicodeString(pStr, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingUnicodeString(pStr, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingAnsiString(pValue, ...)     _tlgArgBuffer(struct _STRING,         pValue, TlgInCOUNTEDANSISTRING, (), __VA_ARGS__)
#define TraceLoggingUnicodeString(pValue, ...)  _tlgArgBuffer(struct _UNICODE_STRING, pValue, TlgInCOUNTEDSTRING,     (), __VA_ARGS__)

/*
Wrapper macros for raw binary data.
Usage: TraceLoggingBinary(pValue, cbValue, "name", "description", tags).
Usage: TraceLoggingBinaryEx(pValue, cbValue, TlgOutTYPE, "name", "description", tags).

Use TraceLoggingBinary for normal binary data.
Use TraceLoggingBinaryEx to specify a custom value for OutType.

The pValue parameter is treated as a const void* so that any kind of data can
be provided. The cbValue parameter is the size of the data, in bytes.

The name, description, and tags parameters are optional.

The pValue parameter may be NULL only if cbValue is 0.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pValue
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available
at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingBinary(pObj, sizeof(*pObj))                      // field name = "pObj", description = unset,  tags = 0.
- TraceLoggingBinary(pObj, sizeof(*pObj), "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingBinary(pObj, sizeof(*pObj), "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingBinary(pObj, sizeof(*pObj), "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingBinary(pValue, cbValue, ...)             _tlgArgBinary(void, pValue, cbValue, TlgInBINARY, (),        __VA_ARGS__)
#define TraceLoggingBinaryEx(pValue, cbValue, outType, ...)  _tlgArgBinary(void, pValue, cbValue, TlgInBINARY, (outType), __VA_ARGS__)

/*
Wrapper macro for event fields with PSOCKADDR, PSOCKADDR_IN, etc. values.
Usage: TraceLoggingSocketAddress(pValue, cbValue, "name", "description", tags).

Note that the amount of data needed for a SOCKADDR field varies depending on
the type of address. If the data is stored in a union variable, be sure to
set the cbValue parameter to the size of the correct union member or the
data might be truncated.

The name, description, and tags parameters are optional.

The pValue parameter may be NULL only if cbValue is 0.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pValue
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingSocketAddress(pSock, sizeof(*pSock))                      // field name = "pSock", description = unset,  tags = 0.
- TraceLoggingSocketAddress(pSock, sizeof(*pSock), "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingSocketAddress(pSock, sizeof(*pSock), "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingSocketAddress(pSock, sizeof(*pSock), "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingSocketAddress(pValue, cbValue, ...) _tlgArgBinary(void, pValue, cbValue, TlgInBINARY, (TlgOutSOCKETADDRESS), __VA_ARGS__)

/*
Wrapper macro for event fields with IPv4 address values.
Usage: TraceLoggingIPv4Address(value, "name", "description", tags).

The name, description, and tags parameters are optional.

The value parameter must be a UINT32-encoded IPv4 address
(e.g. pSock->sin_addr.s_addr).

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the value
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingIPv4Address(sin_addr.s_addr)                      // field name = "sin_addr.s_addr", description = unset,  tags = 0.
- TraceLoggingIPv4Address(sin_addr.s_addr, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingIPv4Address(sin_addr.s_addr, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingIPv4Address(sin_addr.s_addr, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingIPv4Address(value, ...) _tlgArgScalarVal(UINT32, value, TlgInUINT32, (TlgOutIPV4),  __VA_ARGS__)

/*
Wrapper macro for event fields with IPv6 address values.
Usage: TraceLoggingIPv6Address(pValue, "name", "description", tags).

The name, description, and tags parameters are optional.

The pValue parameter must not be NULL and must point at a 16-byte buffer
(e.g. &pSock->sin6_addr).

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pValue
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingIPv6Address(&pSock->sin6_addr)                      // field name = "&pSock->sin6_addr", description = unset,  tags = 0.
- TraceLoggingIPv6Address(&pSock->sin6_addr, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingIPv6Address(&pSock->sin6_addr, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingIPv6Address(&pSock->sin6_addr, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingIPv6Address(pValue, ...) _tlgArgBinary(void, pValue, 16u, TlgInBINARY, (TlgOutIPV6), __VA_ARGS__)

#ifdef SID_DEFINED
/*
Wrapper macros for event fields with PSID values.
Usage: TraceLoggingSid(pValue, "name", "description", tags).

Note that the amount of data needed for a SID field varies depending on
the number of subauthorities. TraceLogging assumes the SID structure is valid
and will send the amount of data indicated by the subauthority count.

The name, description, and tags parameters are optional.

The pValue parameter must not be NULL and must point at a valid SID
(SubAuthorityCount must be initialized).

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pValue
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingSid(pSid)                      // field name = "pSid", description = unset,  tags = 0.
- TraceLoggingSid(pSid, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingSid(pSid, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingSid(pSid, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingSid(pValue, ...) _tlgArgSid(SID, pValue, __VA_ARGS__)
#endif

/*
Wrapper macros for event fields with values that are fixed-length arrays.
Usage: TraceLoggingInt32FixedArray(pValues, cValues, "name", "description", tags).

The cValues parameter must be a compile-time constant value. It must not be 0.

The name, description, and tags parameters are optional.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pbData
or pValues parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingUInt8FixedArray(pbX1, 32)                      // field name = "pbX1", description = unset,  tags = 0.
- TraceLoggingUInt8FixedArray(pbX1, 32, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingUInt8FixedArray(pbX1, 32, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingUInt8FixedArray(pbX1, 32, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingInt8FixedArray(pValues, cValues, ...)          _tlgArgFixedArray(INT8,               pValues, cValues, TlgInINT8,       (),                   __VA_ARGS__)
#define TraceLoggingUInt8FixedArray(pValues, cValues, ...)         _tlgArgFixedArray(UINT8,              pValues, cValues, TlgInUINT8,      (),                   __VA_ARGS__)
#define TraceLoggingInt16FixedArray(pValues, cValues, ...)         _tlgArgFixedArray(INT16,              pValues, cValues, TlgInINT16,      (),                   __VA_ARGS__)
#define TraceLoggingUInt16FixedArray(pValues, cValues, ...)        _tlgArgFixedArray(UINT16,             pValues, cValues, TlgInUINT16,     (),                   __VA_ARGS__)
#define TraceLoggingInt32FixedArray(pValues, cValues, ...)         _tlgArgFixedArray(INT32,              pValues, cValues, TlgInINT32,      (),                   __VA_ARGS__)
#define TraceLoggingUInt32FixedArray(pValues, cValues, ...)        _tlgArgFixedArray(UINT32,             pValues, cValues, TlgInUINT32,     (),                   __VA_ARGS__)
#define TraceLoggingInt64FixedArray(pValues, cValues, ...)         _tlgArgFixedArray(INT64,              pValues, cValues, TlgInINT64,      (),                   __VA_ARGS__)
#define TraceLoggingUInt64FixedArray(pValues, cValues, ...)        _tlgArgFixedArray(UINT64,             pValues, cValues, TlgInUINT64,     (),                   __VA_ARGS__)
#define TraceLoggingIntPtrFixedArray(pValues, cValues, ...)        _tlgArgFixedArray(INT_PTR,            pValues, cValues, TlgInINTPTR,     (),                   __VA_ARGS__)
#define TraceLoggingUIntPtrFixedArray(pValues, cValues, ...)       _tlgArgFixedArray(UINT_PTR,           pValues, cValues, TlgInUINTPTR,    (),                   __VA_ARGS__)
#define TraceLoggingLongFixedArray(pValues, cValues, ...)          _tlgArgFixedArray(LONG,               pValues, cValues, TlgInLONG,       (),                   __VA_ARGS__)
#define TraceLoggingULongFixedArray(pValues, cValues, ...)         _tlgArgFixedArray(ULONG,              pValues, cValues, TlgInULONG,      (),                   __VA_ARGS__)
#define TraceLoggingHexInt8FixedArray(pValues, cValues, ...)       _tlgArgFixedArray(INT8,               pValues, cValues, TlgInUINT8,      (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexUInt8FixedArray(pValues, cValues, ...)      _tlgArgFixedArray(UINT8,              pValues, cValues, TlgInUINT8,      (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexInt16FixedArray(pValues, cValues, ...)      _tlgArgFixedArray(INT16,              pValues, cValues, TlgInUINT16,     (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexUInt16FixedArray(pValues, cValues, ...)     _tlgArgFixedArray(UINT16,             pValues, cValues, TlgInUINT16,     (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexInt32FixedArray(pValues, cValues, ...)      _tlgArgFixedArray(INT32,              pValues, cValues, TlgInHEXINT32,   (),                   __VA_ARGS__)
#define TraceLoggingHexUInt32FixedArray(pValues, cValues, ...)     _tlgArgFixedArray(UINT32,             pValues, cValues, TlgInHEXINT32,   (),                   __VA_ARGS__)
#define TraceLoggingHexInt64FixedArray(pValues, cValues, ...)      _tlgArgFixedArray(INT64,              pValues, cValues, TlgInHEXINT64,   (),                   __VA_ARGS__)
#define TraceLoggingHexUInt64FixedArray(pValues, cValues, ...)     _tlgArgFixedArray(UINT64,             pValues, cValues, TlgInHEXINT64,   (),                   __VA_ARGS__)
#define TraceLoggingHexIntPtrFixedArray(pValues, cValues, ...)     _tlgArgFixedArray(INT_PTR,            pValues, cValues, TlgInPOINTER,    (),                   __VA_ARGS__)
#define TraceLoggingHexUIntPtrFixedArray(pValues, cValues, ...)    _tlgArgFixedArray(UINT_PTR,           pValues, cValues, TlgInPOINTER,    (),                   __VA_ARGS__)
#define TraceLoggingHexLongFixedArray(pValues, cValues, ...)       _tlgArgFixedArray(LONG,               pValues, cValues, TlgInHEXLONG,    (),                   __VA_ARGS__)
#define TraceLoggingHexULongFixedArray(pValues, cValues, ...)      _tlgArgFixedArray(ULONG,              pValues, cValues, TlgInHEXLONG,    (),                   __VA_ARGS__)
#define TraceLoggingFloat32FixedArray(pValues, cValues, ...)       _tlgArgFixedArray(float,              pValues, cValues, TlgInFLOAT,      (),                   __VA_ARGS__)
#define TraceLoggingFloat64FixedArray(pValues, cValues, ...)       _tlgArgFixedArray(double,             pValues, cValues, TlgInDOUBLE,     (),                   __VA_ARGS__)
#define TraceLoggingBooleanFixedArray(pValues, cValues, ...)       _tlgArgFixedArray(BOOLEAN,            pValues, cValues, TlgInUINT8,      (TlgOutBOOLEAN),      __VA_ARGS__)
#define TraceLoggingBoolFixedArray(pValues, cValues, ...)          _tlgArgFixedArray(INT32,              pValues, cValues, TlgInBOOL32,     (),                   __VA_ARGS__)
#define TraceLoggingCharFixedArray(pValues, cValues, ...)          _tlgArgFixedArray(char,               pValues, cValues, TlgInUINT8,      (TlgOutSTRING),       __VA_ARGS__)
#define TraceLoggingChar16FixedArray(pValues, cValues, ...)        _tlgArgFixedArray(char16_t,           pValues, cValues, TlgInUINT16,     (TlgOutSTRING),       __VA_ARGS__)
#define TraceLoggingWCharFixedArray(pValues, cValues, ...)         _tlgArgFixedArray(wchar_t,            pValues, cValues, TlgInUINT16,     (TlgOutSTRING),       __VA_ARGS__)
#define TraceLoggingPointerFixedArray(pValues, cValues, ...)       _tlgArgFixedArray(LPVOID,             pValues, cValues, TlgInPOINTER,    (),                   __VA_ARGS__)
#define TraceLoggingCodePointerFixedArray(pValues, cValues, ...)   _tlgArgFixedArray(LPVOID,             pValues, cValues, TlgInPOINTER,    (TlgOutCODE_POINTER), __VA_ARGS__)
#define TraceLoggingFileTimeFixedArray(pValues, cValues, ...)      _tlgArgFixedArray(struct _FILETIME,   pValues, cValues, TlgInFILETIME,   (),                   __VA_ARGS__)
#define TraceLoggingFileTimeUtcFixedArray(pValues, cValues, ...)   _tlgArgFixedArray(struct _FILETIME,   pValues, cValues, TlgInFILETIME,   (TlgOutDATETIME_UTC), __VA_ARGS__)
#define TraceLoggingSystemTimeFixedArray(pValues, cValues, ...)    _tlgArgFixedArray(struct _SYSTEMTIME, pValues, cValues, TlgInSYSTEMTIME, (),                   __VA_ARGS__)
#define TraceLoggingSystemTimeUtcFixedArray(pValues, cValues, ...) _tlgArgFixedArray(struct _SYSTEMTIME, pValues, cValues, TlgInSYSTEMTIME, (TlgOutDATETIME_UTC), __VA_ARGS__)
#define TraceLoggingGuidFixedArray(pValues, cValues, ...)          _tlgArgFixedArray(GUID,               pValues, cValues, TlgInGUID,       (),                   __VA_ARGS__)

/*
Wrapper macros for event fields with values that are variable-length arrays.
Usage: TraceLoggingInt32Array(pValues, cValues, "name", "description", tags).

The name, description, and tags parameters are optional.

The pointer parameter may be NULL only if the count parameter is 0.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pbData
or pValues parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Examples:
- TraceLoggingUInt8Array(pbX1, cbX1)                      // field name = "pbX1", description = unset,  tags = 0.
- TraceLoggingUInt8Array(pbX1, cbX1, "name")              // field name = "name", description = unset,  tags = 0.
- TraceLoggingUInt8Array(pbX1, cbX1, "name", "desc")      // field name = "name", description = "desc", tags = 0.
- TraceLoggingUInt8Array(pbX1, cbX1, "name", "desc", 0x4) // field name = "name", description = "desc", tags = 0x4.
*/
#define TraceLoggingInt8Array(pValues, cValues, ...)          _tlgArgArray(INT8,               pValues, cValues, TlgInINT8,       (),                   __VA_ARGS__)
#define TraceLoggingUInt8Array(pValues, cValues, ...)         _tlgArgArray(UINT8,              pValues, cValues, TlgInUINT8,      (),                   __VA_ARGS__)
#define TraceLoggingInt16Array(pValues, cValues, ...)         _tlgArgArray(INT16,              pValues, cValues, TlgInINT16,      (),                   __VA_ARGS__)
#define TraceLoggingUInt16Array(pValues, cValues, ...)        _tlgArgArray(UINT16,             pValues, cValues, TlgInUINT16,     (),                   __VA_ARGS__)
#define TraceLoggingInt32Array(pValues, cValues, ...)         _tlgArgArray(INT32,              pValues, cValues, TlgInINT32,      (),                   __VA_ARGS__)
#define TraceLoggingUInt32Array(pValues, cValues, ...)        _tlgArgArray(UINT32,             pValues, cValues, TlgInUINT32,     (),                   __VA_ARGS__)
#define TraceLoggingInt64Array(pValues, cValues, ...)         _tlgArgArray(INT64,              pValues, cValues, TlgInINT64,      (),                   __VA_ARGS__)
#define TraceLoggingUInt64Array(pValues, cValues, ...)        _tlgArgArray(UINT64,             pValues, cValues, TlgInUINT64,     (),                   __VA_ARGS__)
#define TraceLoggingIntPtrArray(pValues, cValues, ...)        _tlgArgArray(INT_PTR,            pValues, cValues, TlgInINTPTR,     (),                   __VA_ARGS__)
#define TraceLoggingUIntPtrArray(pValues, cValues, ...)       _tlgArgArray(UINT_PTR,           pValues, cValues, TlgInUINTPTR,    (),                   __VA_ARGS__)
#define TraceLoggingLongArray(pValues, cValues, ...)          _tlgArgArray(LONG,               pValues, cValues, TlgInLONG,       (),                   __VA_ARGS__)
#define TraceLoggingULongArray(pValues, cValues, ...)         _tlgArgArray(ULONG,              pValues, cValues, TlgInULONG,      (),                   __VA_ARGS__)
#define TraceLoggingHexInt8Array(pValues, cValues, ...)       _tlgArgArray(INT8,               pValues, cValues, TlgInUINT8,      (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexUInt8Array(pValues, cValues, ...)      _tlgArgArray(UINT8,              pValues, cValues, TlgInUINT8,      (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexInt16Array(pValues, cValues, ...)      _tlgArgArray(INT16,              pValues, cValues, TlgInUINT16,     (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexUInt16Array(pValues, cValues, ...)     _tlgArgArray(UINT16,             pValues, cValues, TlgInUINT16,     (TlgOutHEX),          __VA_ARGS__)
#define TraceLoggingHexInt32Array(pValues, cValues, ...)      _tlgArgArray(INT32,              pValues, cValues, TlgInHEXINT32,   (),                   __VA_ARGS__)
#define TraceLoggingHexUInt32Array(pValues, cValues, ...)     _tlgArgArray(UINT32,             pValues, cValues, TlgInHEXINT32,   (),                   __VA_ARGS__)
#define TraceLoggingHexInt64Array(pValues, cValues, ...)      _tlgArgArray(INT64,              pValues, cValues, TlgInHEXINT64,   (),                   __VA_ARGS__)
#define TraceLoggingHexUInt64Array(pValues, cValues, ...)     _tlgArgArray(UINT64,             pValues, cValues, TlgInHEXINT64,   (),                   __VA_ARGS__)
#define TraceLoggingHexIntPtrArray(pValues, cValues, ...)     _tlgArgArray(INT_PTR,            pValues, cValues, TlgInPOINTER,    (),                   __VA_ARGS__)
#define TraceLoggingHexUIntPtrArray(pValues, cValues, ...)    _tlgArgArray(UINT_PTR,           pValues, cValues, TlgInPOINTER,    (),                   __VA_ARGS__)
#define TraceLoggingHexLongArray(pValues, cValues, ...)       _tlgArgArray(LONG,               pValues, cValues, TlgInHEXLONG,    (),                   __VA_ARGS__)
#define TraceLoggingHexULongArray(pValues, cValues, ...)      _tlgArgArray(ULONG,              pValues, cValues, TlgInHEXLONG,    (),                   __VA_ARGS__)
#define TraceLoggingFloat32Array(pValues, cValues, ...)       _tlgArgArray(float,              pValues, cValues, TlgInFLOAT,      (),                   __VA_ARGS__)
#define TraceLoggingFloat64Array(pValues, cValues, ...)       _tlgArgArray(double,             pValues, cValues, TlgInDOUBLE,     (),                   __VA_ARGS__)
#define TraceLoggingBooleanArray(pValues, cValues, ...)       _tlgArgArray(BOOLEAN,            pValues, cValues, TlgInUINT8,      (TlgOutBOOLEAN),      __VA_ARGS__)
#define TraceLoggingBoolArray(pValues, cValues, ...)          _tlgArgArray(INT32,              pValues, cValues, TlgInBOOL32,     (),                   __VA_ARGS__)
#define TraceLoggingCharArray(pValues, cValues, ...)          _tlgArgArray(char,               pValues, cValues, TlgInUINT8,      (TlgOutSTRING),       __VA_ARGS__)
#define TraceLoggingChar16Array(pValues, cValues, ...)        _tlgArgArray(char16_t,           pValues, cValues, TlgInUINT16,     (TlgOutSTRING),       __VA_ARGS__)
#define TraceLoggingWCharArray(pValues, cValues, ...)         _tlgArgArray(wchar_t,            pValues, cValues, TlgInUINT16,     (TlgOutSTRING),       __VA_ARGS__)
#define TraceLoggingPointerArray(pValues, cValues, ...)       _tlgArgArray(LPVOID,             pValues, cValues, TlgInPOINTER,    (),                   __VA_ARGS__)
#define TraceLoggingCodePointerArray(pValues, cValues, ...)   _tlgArgArray(LPVOID,             pValues, cValues, TlgInPOINTER,    (TlgOutCODE_POINTER), __VA_ARGS__)
#define TraceLoggingFileTimeArray(pValues, cValues, ...)      _tlgArgArray(struct _FILETIME,   pValues, cValues, TlgInFILETIME,   (),                   __VA_ARGS__)
#define TraceLoggingFileTimeUtcArray(pValues, cValues, ...)   _tlgArgArray(struct _FILETIME,   pValues, cValues, TlgInFILETIME,   (TlgOutDATETIME_UTC), __VA_ARGS__)
#define TraceLoggingSystemTimeArray(pValues, cValues, ...)    _tlgArgArray(struct _SYSTEMTIME, pValues, cValues, TlgInSYSTEMTIME, (),                   __VA_ARGS__)
#define TraceLoggingSystemTimeUtcArray(pValues, cValues, ...) _tlgArgArray(struct _SYSTEMTIME, pValues, cValues, TlgInSYSTEMTIME, (TlgOutDATETIME_UTC), __VA_ARGS__)
#define TraceLoggingGuidArray(pValues, cValues, ...)          _tlgArgArray(GUID,               pValues, cValues, TlgInGUID,       (),                   __VA_ARGS__)

/*
Wrapper macros for manually-packed fields (advanced scenarios).
These macros support custom serialization of fields for use in creating events
that would otherwise be inexpressible through TraceLoggingProvider.h. For
example, these macros can be used to write fields containing arrays of strings
or arrays of structures. Note that the correct use of these macros requires an
understanding of how TraceLogging encodes events. If used incorrectly, these
macros can generate events that are impossible to decode. Note that to write
arrays of strings or arrays of structures, you will usually need to do
additional work such as manually buffering or marshaling the data before
invoking TraceLoggingWrite.

TraceLoggingPackedField(pValue, cbValue, inType, "name", "description", tags)
TraceLoggingPackedFieldEx(pValue, cbValue, inType, outType, "name", "description", tags)
TraceLoggingPackedMetadata(inType, "name", "description", tags)
TraceLoggingPackedMetadataEx(inType, outType, "name", "description", tags)
TraceLoggingPackedStruct(fieldCount, "name", "description", tags)
TraceLoggingPackedStructArray(fieldCount, "name", "description", tags)
TraceLoggingPackedData(pValue, cbValue)
TraceLoggingPackedDataEx(pValue, cbValue, dataDescType)

The name parameter must be a string literal (not a variable) and must not
contain any '\0' characters. For TraceLoggingPackedField and
TraceLoggingPackedFieldEx, the name parameter is optional. If the name is not
provided, the TraceLoggingPackedField and TraceLoggingPackedFieldEx macros will
use the pValue parameter to automatically generate a field name.

The description parameter is optional. If provided, the description parameter
must be a string literal. The description will be included in the debug
symbols (PDB file).

The tags parameter is optional. If provided, the tags parameter must be an
integer value. The low 28 bits of the value will be included in the field's
metadata. The semantics of the tags are defined by the event consumer. During
event processing, this tag can be retrieved from the EVENT_PROPERTY_INFO Tags
field.

A TraceLogging event contains metadata and data. The metadata is the list of
fields, each with a name and a type. The data is the payload - an array of
raw bytes that contains the values of the event fields. The metadata is
composed of compile-time-constant data, while the data can be different each
time the event is generated. The metadata is used to decode the data, so the
metadata and the data need to be coordinated. The other wrapper macros
(TraceLoggingInt32, TraceLoggingString, etc.) automatically keep the metadata
and data coordinated, but the TraceLoggingPacked macros allow direct control
over the metadata and data so incorrect use of them can result in events that
do not decode correctly.

The TraceLoggingPackedField macro adds both metadata and data. It adds an
arbitrary field to the event's type, and adds arbitrary data to the event's
payload. The TraceLoggingPackedFieldEx macro does the same, but includes a byte
for the field's OutType in the field descriptor.

The TraceLoggingPackedMetadata macro adds only metadata. It adds a field to the
event's type without adding any data to the event's payload. The
TraceLoggingPackedMetadataEx macro does the same, but includes a byte for the
field's OutType in the field descriptor.

The TraceLoggingPackedStruct macro adds only metadata (a struct declaration
never contains data -- the struct's data is provided by its fields). It begins
a structure in the event: the <fieldCount> logical fields that follow the start
of the structure are considered to be part of the structure, and they will form
one logical field. (Structures can nest, and a nested structure counts as one
logical field in the parent structure.) The TraceLoggingPackedStructArray does
the same, but it begins an array of structures (which also counts as one
logical field).

The TraceLoggingPackedData macro adds data directly into the event payload
without adding a field to the event's type. The TraceLoggingPackedDataEx macro
is the same, but it allows you to specify a non-default value to use in the
EVENT_DATA_DESCRIPTOR Type field (for custom communication with low-level ETW
processing).

These macros can be combined in various ways to express TraceLogging field
structures not otherwise possible. Possible scenarios include:

* Write a simple field with a specific InType/OutType combination that is
  not supported by the core TraceLogging macros.

  For example, to write a nul-terminated wide string that is tagged as
  containing JSON data:

    TraceLoggingWrite(
        g_hProvider,
        "MyEventWithJsonData",
        TraceLoggingInt32(otherData1),
        TraceLoggingPackedFieldEx(
            szJson,
            (wcslen(szJson) + 1) * sizeof(wchar_t),
            TlgInUNICODESTRING,
            TlgOutJSON,
            "MyJsonFieldName"),
        TraceLoggingInt32(otherData2));

* Write a complex field that requires marshaling data into a temporary
  buffer.

  For example, to write an array of nul-terminated ANSI strings:

    // This scenario requires manually marshaling data.
    // Don't spend time marshaling data if the event is disabled.
    if (TraceLoggingProviderEnabled(g_hProvider, level, keyword))
    {
        // This example assumes that the strings will fit into 100 bytes.
        // Your production code will need to do additional error checking, or
        // perhaps use std::vector<BYTE> and do a buf.push_back(val) instead of
        // buf[iBuf++] = val.
        BYTE buf[100];
        unsigned iBuf = 0;

        // Packed arrays start with a UINT16 value indicating the number of
        // elements in the array.
        buf[iBuf++] = (BYTE)cStrings;        // Low byte of the element count
        buf[iBuf++] = (BYTE)(cStrings >> 8); // High byte of the element count

        // Then we need to add the content of each array element.
        for (UINT i = 0; i != cStrings; i++)
        {
            for (LPCSTR pString = pStrings[i]; *pString != 0; pString++)
            {
                buf[iBuf++] = *pString;
            }
            buf[iBuf++] = 0; // nul-terminate
        }

        TraceLoggingWrite(
            g_hProvider,
            "MyEventWithArrayOfStrings",
            TraceLoggingLevel(level),
            TraceLoggingKeyword(keyword),
            TraceLoggingInt32(otherData1),
            TraceLoggingPackedField(
                buf,
                iBuf,
                TlgInANSISTRING | TlgInVcount, // TlgInVcount means this is an array.
                "MyArrayOfStringsFieldName"),
            TraceLoggingInt32(otherData2));
    }

* Write a structure directly as a single entity instead of as a series of
  fields.

  This can be a minor performance optimization in some cases (it can reduce
  per-event CPU and reduce stack usage), since it reduces the number of
  EVENT_DATA_DESCRIPTOR that need to be created and initialized when generating
  the event. Note that structures can only be written directly if the structure
  contains no internal padding or non-blittable fields. If the structure
  contains padding or non-blittable fields, you would need to buffer and repack
  the data before using this technique, in which case it would have been more
  efficient to use the normal methods for logging structures (i.e. using a
  normal TraceLoggingStruct followed by the appropriate TraceLoggingValue for
  each field).

  Overview: provide the data for the struct using TraceLoggingPackedData;
  provide the number of fields and the name of the structure with
  TraceLoggingPackedStruct; provide the names and types of the fields using
  TraceLoggingPackedMetadata.

  Note that while the order of metadata is important and the ordering of data is
  important, the ordering between metadata and data is not important. In the
  example below, the TraceLoggingPackedData macro could appear anywhere between
  otherData1 and otherData2 without changing the result. However, it could not
  appear before otherData1 or after otherData2, since each of those also emit
  data, and the data from TraceLoggingPackedData must appear after otherData1
  and before otherData2.

    TraceLoggingWrite(
        g_hProvider,
        "MyEventWithRect",
        TraceLoggingInt32(otherData1),
        TraceLoggingPackedData(&rect, sizeof(RECT)), // Data for all 4 fields
        TraceLoggingPackedStruct(4, "RectangleFieldName"), // Structure with 4 fields
            TraceLoggingPackedMetadata(TlgInINT32, "left"),
            TraceLoggingPackedMetadata(TlgInINT32, "top"),
            TraceLoggingPackedMetadata(TlgInINT32, "right"),
            TraceLoggingPackedMetadata(TlgInINT32, "bottom"),
        TraceLoggingInt32(otherData2));

* Write an array of structures.

  Overview: Provide the data for the array (the array count and the array
  content) using TraceLoggingPackedData; provide the number of fields and the
  name of the structure with TraceLoggingPackedStructArray; provide the names
  and types of the fields using TraceLoggingPackedMetadata.

  In the example below, the array contains no padding and no non-blittable data
  (i.e. variable-length data, out-of-line data like pointers to strings, etc.),
  so we can provide a pointer directly to the array content. If the array
  contained padding or contained non-blittable data, you would need to allocate
  a buffer and re-pack the data, inlining any non-blittable elements and
  omitting any padding. The example below needs to provide the array element
  count (UINT16) as well as the array content, so it uses
  TraceLoggingPackedData twice.

    TraceLoggingWrite(
        g_hProvider,
        "MyEventWithArrayOfRectangles",
        TraceLoggingInt32(otherData1),
        TraceLoggingPackedData(&cRectangles, sizeof(UINT16)), // Data for the array count
        TraceLoggingPackedData(pRectangles, cRectangles * sizeof(RECT)), // Data for the array content
        TraceLoggingPackedStructArray(4, "RectangleArrayFieldName"), // Structure with 4 fields
            TraceLoggingPackedMetadata(TlgInINT32, "left"),
            TraceLoggingPackedMetadata(TlgInINT32, "top"),
            TraceLoggingPackedMetadata(TlgInINT32, "right"),
            TraceLoggingPackedMetadata(TlgInINT32, "bottom"),
        TraceLoggingInt32(otherData2));

Notes on serializing data:

- When the decoder receives the event, it sees the event payload as a single
  block of bytes. It does not see any boundaries between chunks of data in the
  payload. If I use TraceLoggingPackedMetadata to add an Int32 field but
  provide 5 bytes of data, the decoder will not be able to correctly decode the
  remaining fields of the event. The developer must take care that the data
  written matches up with the field definitions. On the other hand, this allows
  flexibility in the way the data is encoded. For example, I might write the
  data for several fields using a single TraceLoggingPackedData macro (more
  efficient if the data is already contiguous in memory), or I might use
  multiple TraceLoggingPackedData macros to gather bits of a single field's
  value from multiple locations in memory (more efficient than recopying the
  data to make it contiguous).
- Encoding/decoding behavior only uses the inType. The outType is only a
  formatting hint and may be ignored by the decoder.
- TlgInUNICODESTRING means "nul-terminated UTF-16 wchar_t string".
- TlgInANSISTRING means "nul-terminated MBCS char string".
- TlgInCOUNTEDSTRING means "size-prefixed UTF-16 wchar_t string".
- TlgInCOUNTEDANSISTRING means "size-prefixed MBCS char string".
- TlgInBINARY and TlgInCOUNTEDBINARY mean "size-prefixed binary data". Both
  types are encoded identically, but they go through different decoding paths.
  TlgInBINARY cannot be used for arrays and results in an implicit Length
  property. TlgInCOUNTEDBINARY can be used for arrays but is a newer InType
  that may not be supported by all decoding tools.
- TlgInBINARY, TlgInCOUNTEDBINARY, TlgInCOUNTEDSTRING, and
  TlgInCOUNTEDANSISTRING are encoded as a little-endian UINT16 byte-count (not
  a char-count) followed by the data.
- Form an array by adding the TlgInVcount flag to the inType. For example,
  an inType of TlgInANSISTRING will result in a field that stores a single
  string, but an inType of TlgInANSISTRING|TlgInVcount will result in a field
  that stores an array of strings.
- Arrays are serialized as a little-endian UINT16 element-count followed by the
  elements. The elements in an array are serialized exactly as if they were not
  in an array, even if the element has a variable length. For example, the
  payload corrsponding to the 3-element array { "ABC", "DE", "F" } would be:
  BYTE a[] = { '\3', '\0', 'A', 'B', 'C', '\0', 'D', 'E', '\0', 'F', '\0' };
- Array of TlgInBINARY should not be used because TDH cannot decode it.
  As an alternative, you can create an array of structures and put a field of
  type TlgInBINARY inside it, or if your decoding tool supports the
  TDH_INTYPE_MANIFEST_COUNTEDBINARY type, you can use TlgInCOUNTEDBINARY.
- All simple data types other than those used with TlgOutPORT and TlgOutIPV4
  are encoded as little-endian.
*/
#define TraceLoggingPackedField(pValue, cbValue, inType, ...)            _tlgArgPackedField(void, pValue, cbValue, inType, (),        __VA_ARGS__)
#define TraceLoggingPackedFieldEx(pValue, cbValue, inType, outType, ...) _tlgArgPackedField(void, pValue, cbValue, inType, (outType), __VA_ARGS__)
#define TraceLoggingPackedMetadata(inType, name, ...)                    _tlgArgPackedMeta(       name,            inType, (),        __VA_ARGS__)
#define TraceLoggingPackedMetadataEx(inType, outType, name, ...)         _tlgArgPackedMeta(       name,            inType, (outType), __VA_ARGS__)
#define TraceLoggingPackedStruct(fieldCount, name, ...)                  _tlgArgStruct(fieldCount,name, _TlgInSTRUCT,                 __VA_ARGS__)
#define TraceLoggingPackedStructArray(fieldCount, name, ...)             _tlgArgStruct(fieldCount,name, _TlgInSTRUCT|TlgInVcount,     __VA_ARGS__)
#define TraceLoggingPackedData(pValue, cbValue)                          _tlgArgPackedData( void, pValue, cbValue, ())
#define TraceLoggingPackedDataEx(pValue, cbValue, dataDescType)          _tlgArgPackedData( void, pValue, cbValue, (dataDescType))

/*
Wrapper macros for binary data referenced by a structure (advanced scenarios).
Usage: TraceLoggingBinaryBuffer(pMyStruct, MyStructType, "name", "description", tags).
Usage: TraceLoggingBinaryBufferEx(pMyStruct, MyStructType, TlgOutTYPE, "name", "description", tags).

These macros support serialization of structures that have fields "Buffer" and
"Length", where the Buffer field points to the data to be transmitted and the
Length field contains the number of bytes to be transmitted.

Use TraceLoggingBinaryBuffer for normal binary data.
Use TraceLoggingBinaryBufferEx to specify a custom value for OutType.

The name, description, and tags parameters are optional.

The pMyStruct parameter must not be NULL.

If provided, the name parameter must be a string literal (not a variable) and
must not contain any '\0' characters. If the name is not provided, the pValue
parameter is used to automatically generate a name.

If provided, the description parameter must be a string literal, and will be
included in the debug symbols (PDB file). The description will not be
available at runtime.

If provided, the tags parameter must be an integer value. The low 28 bits of
the value will be included in the field's metadata. The semantics of the tags
are defined by the event consumer. During event processing, this tag can be
retrieved from the EVENT_PROPERTY_INFO Tags field.

Requirements:

- The pMyStruct parameter must be a non-null pointer to dataType (or const
  dataType).
- The dataType must have fields "Length" and "Buffer".
  - The Length field must contain the size of the data (in bytes).
  - The Buffer field must contain a pointer to the data. (Buffer may be null if
    Length is 0.)
*/
#define TraceLoggingBinaryBuffer(pValue, StructType, ...)            _tlgArgBuffer(StructType, pValue, TlgInBINARY, (),        __VA_ARGS__)
#define TraceLoggingBinaryBufferEx(pValue, StructType, outType, ...) _tlgArgBuffer(StructType, pValue, TlgInBINARY, (outType), __VA_ARGS__)

/*
Macro TraceLoggingCustom(pValue, cbValue, protocol, bSchema, cbSchema, name, ...):
Wrapper macro for an event field packed using a custom serializer.

The pValue/cbValue are the serialized data that has been generated at runtime
by a serializer from the specified protocol family.

The protocol family may be specified as a value from 1 to 31
with reserved values defined below (TRACELOGGING_PROTOCOL_*).

The bSchema/cbSchema need to be compile-time literals (not variables).

Example:

    BYTE rgValue[] = {...};

    TraceLoggingWrite(
        g_hProvider,
        "MyEventName",
        TraceLoggingCustom(
            rgValue,
            sizeof(rgValue),
            TRACELOGGING_PROTOCOL_MYPROTOCOL,
            ( 0x0, 0x1, 0x2 ), // schema bytes
            3, // number of schema bytes
            "MyCustomField"
            ));
*/
#define TraceLoggingCustom(pValue, cbValue, protocol, bSchema, cbSchema, ...) \
    _tlgArgCustom(void, pValue, cbValue, protocol, bSchema, cbSchema, __VA_ARGS__)

#define TRACELOGGING_PROTOCOL_MICROSOFT_RESERVED0   0
#define TRACELOGGING_PROTOCOL_MICROSOFT_RESERVED1   1
#define TRACELOGGING_PROTOCOL_MICROSOFT_RESERVED2   2
#define TRACELOGGING_PROTOCOL_MICROSOFT_RESERVED3   3
#define TRACELOGGING_PROTOCOL_MICROSOFT_RESERVED4   4

/*
By default, non-pageable TraceLogging inline functions go into the ".text"
segment. This can be overridden if necessary by setting the
TLG_NONPAGED_SEGMENT macro before including <TraceLoggingProvider.h>.
*/
#ifndef TLG_NONPAGED_SEGMENT
#define TLG_NONPAGED_SEGMENT ".text"
#endif

/*
By default, pageable TraceLogging inline functions go into the ".text" segment
(user-mode) or the "PAGE" segment (kernel-mode). This can be overridden if
necessary by setting the TLG_PAGED_SEGMENT macro before including
<TraceLoggingProvider.h>.
*/
#ifndef TLG_PAGED_SEGMENT
#ifdef _ETW_KM_
#define TLG_PAGED_SEGMENT "PAGE"
#else
#define TLG_PAGED_SEGMENT ".text"
#endif
#endif

/*
By default, event metadata is stored in a blob in the ".rdata" segment. This
can be overridden if necessary by setting the TLG_METADATA_SEGMENT_BASE macro
before including <TraceLoggingProvider.h>. If overridden, this value must be
set consistently for all .obj files within the same module.

Note that for kernel-mode code, the metadata usually needs to be in non-paged
memory.
*/
#ifndef TLG_METADATA_SEGMENT_BASE
#define TLG_METADATA_SEGMENT_BASE .rdata
#endif

#pragma endregion

#pragma region Utility macros (for use only within TraceLoggingProvider.h)

#if TLG_DEBUG
  // Not for use outside of TraceLoggingProvider.h.
  #define _tlg_ASSERT(exp, str) ((void)(!(exp) ? (__annotation(L"Debug", L"AssertFail", L"TraceLogging: " _tlg_LSTRINGIZE(exp) L" : " str), TLG_RAISEASSERTIONFAILURE(), 0) : 0))
#else // TLG_DEBUG
  // Not for use outside of TraceLoggingProvider.h.
  #define _tlg_ASSERT(exp, str) ((void)0)
#endif // TLG_DEBUG

#ifndef _tlg_FASTFAIL
  #if defined(_M_CEE)
    // Not for use outside of TraceLoggingProvider.h.
    #define _tlg_FASTFAIL(code) __debugbreak()
  #else
    // Not for use outside of TraceLoggingProvider.h.
    #define _tlg_FASTFAIL(code) __fastfail(code)
  #endif
#endif // _tlg_FASTFAIL

#ifndef _tlg_CASSERT
  #if defined(__cplusplus)
    // Not for use outside of TraceLoggingProvider.h.
    #define _tlg_CASSERT(exp, str) static_assert(exp, str)
  #elif defined(__clang__)
    // Not for use outside of TraceLoggingProvider.h.
    #define _tlg_CASSERT(exp, str) _Static_assert(exp, str)
  #else
    // Not for use outside of TraceLoggingProvider.h.
    #define _tlg_CASSERT(exp, str) C_ASSERT(exp)
  #endif
#endif // _tlg_CASSERT

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_FLATTEN(...)       __VA_ARGS__
#define _tlg_PARENTHESIZE(...) (__VA_ARGS__)

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_STRINGIZE(x) _tlg_STRINGIZE_imp(x)
#define _tlg_STRINGIZE_imp(x) #x

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_LSTRINGIZE(x) _tlg_LSTRINGIZE_imp(x)
#define _tlg_LSTRINGIZE_imp(x) _tlg_PASTE2(L, #x)

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_CAT(a, ...)     _tlg_CAT_imp(a, __VA_ARGS__)
#define _tlg_CAT_imp(a, ...) a##__VA_ARGS__

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_SPLIT(cond, ...) _tlg_SPLIT_imp(cond, (__VA_ARGS__))
#define _tlg_SPLIT_imp(cond, args) _tlg_PASTE2(_tlg_SPLIT_imp, cond) args
#define _tlg_SPLIT_imp0(false_val, ...) false_val
#define _tlg_SPLIT_imp1(false_val, ...) __VA_ARGS__

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_IS_PARENTHESIZED(...) \
    _tlg_SPLIT(0, _tlg_CAT(_tlg_IS_PARENTHESIZED_imp, _tlg_IS_PARENTHESIZED_imp0 __VA_ARGS__))
#define _tlg_IS_PARENTHESIZED_imp0(...) 1
#define _tlg_IS_PARENTHESIZED_imp1      1,
#define _tlg_IS_PARENTHESIZED_imp_tlg_IS_PARENTHESIZED_imp0 0,

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_IS_EMPTY(...) _tlg_SPLIT( \
    _tlg_IS_PARENTHESIZED(__VA_ARGS__), \
    _tlg_IS_PARENTHESIZED(_tlg_PARENTHESIZE __VA_ARGS__()), \
    0)

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_NARGS(...) _tlg_NARGS_imp(_tlg_IS_EMPTY(__VA_ARGS__), (__VA_ARGS__))
#define _tlg_NARGS_imp(is_empty, args) _tlg_PASTE2(_tlg_NARGS_imp, is_empty) args
#define _tlg_NARGS_imp0(...) _tlg_PASTE2(_tlg_NARGS_imp2(__VA_ARGS__, \
    99,98,97,96,95,94,93,92,91,90, \
    89,88,87,86,85,84,83,82,81,80, \
    79,78,77,76,75,74,73,72,71,70, \
    69,68,67,66,65,64,63,62,61,60, \
    59,58,57,56,55,54,53,52,51,50, \
    49,48,47,46,45,44,43,42,41,40, \
    39,38,37,36,35,34,33,32,31,30, \
    29,28,27,26,25,24,23,22,21,20, \
    19,18,17,16,15,14,13,12,11,10, \
     9, 8, 7, 6, 5, 4, 3, 2, 1,),)
#define _tlg_NARGS_imp1() 0
#define _tlg_NARGS_imp2( \
         a1, a2, a3, a4, a5, a6, a7, a8, a9, \
    a10,a11,a12,a13,a14,a15,a16,a17,a18,a19, \
    a20,a21,a22,a23,a24,a25,a26,a27,a28,a29, \
    a30,a31,a32,a33,a34,a35,a36,a37,a38,a39, \
    a40,a41,a42,a43,a44,a45,a46,a47,a48,a49, \
    a50,a51,a52,a53,a54,a55,a56,a57,a58,a59, \
    a60,a61,a62,a63,a64,a65,a66,a67,a68,a69, \
    a70,a71,a72,a73,a74,a75,a76,a77,a78,a79, \
    a80,a81,a82,a83,a84,a85,a86,a87,a88,a89, \
    a90,a91,a92,a93,a94,a95,a96,a97,a98,a99, \
    size, ...) size

// Not for use outside of TraceLoggingProvider.h.
#define _tlg_FOREACH(fn, ...) _tlg_FOR_imp(_tlg_NARGS(__VA_ARGS__), (fn, __VA_ARGS__))
#define _tlg_FOR_imp(n, fnAndArgs)  _tlg_PASTE2(_tlg_FOR_imp, n) fnAndArgs
#define _tlg_FOR_imp0( f,...)
#define _tlg_FOR_imp1( f,a0) f(0,a0)
#define _tlg_FOR_imp2( f,a0,a1) f(0,a0) f(1,a1)
#define _tlg_FOR_imp3( f,a0,a1,a2) f(0,a0) f(1,a1) f(2,a2)
#define _tlg_FOR_imp4( f,a0,a1,a2,a3) f(0,a0) f(1,a1) f(2,a2) f(3,a3)
#define _tlg_FOR_imp5( f,a0,a1,a2,a3,a4) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4)
#define _tlg_FOR_imp6( f,a0,a1,a2,a3,a4,a5) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5)
#define _tlg_FOR_imp7( f,a0,a1,a2,a3,a4,a5,a6) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6)
#define _tlg_FOR_imp8( f,a0,a1,a2,a3,a4,a5,a6,a7) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7)
#define _tlg_FOR_imp9( f,a0,a1,a2,a3,a4,a5,a6,a7,a8) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8)
#define _tlg_FOR_imp10(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9)
#define _tlg_FOR_imp11(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10)
#define _tlg_FOR_imp12(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11)
#define _tlg_FOR_imp13(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12)
#define _tlg_FOR_imp14(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13)
#define _tlg_FOR_imp15(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14)
#define _tlg_FOR_imp16(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15)
#define _tlg_FOR_imp17(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16)
#define _tlg_FOR_imp18(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17)
#define _tlg_FOR_imp19(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18)
#define _tlg_FOR_imp20(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19)
#define _tlg_FOR_imp21(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20)
#define _tlg_FOR_imp22(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21)
#define _tlg_FOR_imp23(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22)
#define _tlg_FOR_imp24(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23)
#define _tlg_FOR_imp25(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24)
#define _tlg_FOR_imp26(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25)
#define _tlg_FOR_imp27(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26)
#define _tlg_FOR_imp28(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27)
#define _tlg_FOR_imp29(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28)
#define _tlg_FOR_imp30(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29)
#define _tlg_FOR_imp31(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30)
#define _tlg_FOR_imp32(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31)
#define _tlg_FOR_imp33(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32)
#define _tlg_FOR_imp34(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33)
#define _tlg_FOR_imp35(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34)
#define _tlg_FOR_imp36(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35)
#define _tlg_FOR_imp37(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36)
#define _tlg_FOR_imp38(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37)
#define _tlg_FOR_imp39(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38)
#define _tlg_FOR_imp40(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39)
#define _tlg_FOR_imp41(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40)
#define _tlg_FOR_imp42(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41)
#define _tlg_FOR_imp43(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42)
#define _tlg_FOR_imp44(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43)
#define _tlg_FOR_imp45(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44)
#define _tlg_FOR_imp46(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45)
#define _tlg_FOR_imp47(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46)
#define _tlg_FOR_imp48(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47)
#define _tlg_FOR_imp49(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48)
#define _tlg_FOR_imp50(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49)
#define _tlg_FOR_imp51(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50)
#define _tlg_FOR_imp52(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51)
#define _tlg_FOR_imp53(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52)
#define _tlg_FOR_imp54(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53)
#define _tlg_FOR_imp55(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54)
#define _tlg_FOR_imp56(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55)
#define _tlg_FOR_imp57(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56)
#define _tlg_FOR_imp58(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57)
#define _tlg_FOR_imp59(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58)
#define _tlg_FOR_imp60(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59)
#define _tlg_FOR_imp61(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60)
#define _tlg_FOR_imp62(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61)
#define _tlg_FOR_imp63(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62)
#define _tlg_FOR_imp64(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63)
#define _tlg_FOR_imp65(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64)
#define _tlg_FOR_imp66(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65)
#define _tlg_FOR_imp67(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66)
#define _tlg_FOR_imp68(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67)
#define _tlg_FOR_imp69(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68)
#define _tlg_FOR_imp70(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69)
#define _tlg_FOR_imp71(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70)
#define _tlg_FOR_imp72(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71)
#define _tlg_FOR_imp73(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72)
#define _tlg_FOR_imp74(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73)
#define _tlg_FOR_imp75(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74)
#define _tlg_FOR_imp76(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75)
#define _tlg_FOR_imp77(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76)
#define _tlg_FOR_imp78(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77)
#define _tlg_FOR_imp79(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78)
#define _tlg_FOR_imp80(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79)
#define _tlg_FOR_imp81(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80)
#define _tlg_FOR_imp82(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81)
#define _tlg_FOR_imp83(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82)
#define _tlg_FOR_imp84(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83)
#define _tlg_FOR_imp85(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84)
#define _tlg_FOR_imp86(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85)
#define _tlg_FOR_imp87(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86)
#define _tlg_FOR_imp88(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87)
#define _tlg_FOR_imp89(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88)
#define _tlg_FOR_imp90(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89)
#define _tlg_FOR_imp91(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90)
#define _tlg_FOR_imp92(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91)
#define _tlg_FOR_imp93(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92)
#define _tlg_FOR_imp94(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92,a93) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92) f(93,a93)
#define _tlg_FOR_imp95(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92,a93,a94) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92) f(93,a93) f(94,a94)
#define _tlg_FOR_imp96(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92,a93,a94,a95) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92) f(93,a93) f(94,a94) f(95,a95)
#define _tlg_FOR_imp97(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92,a93,a94,a95,a96) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92) f(93,a93) f(94,a94) f(95,a95) f(96,a96)
#define _tlg_FOR_imp98(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92,a93,a94,a95,a96,a97) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92) f(93,a93) f(94,a94) f(95,a95) f(96,a96) f(97,a97)
#define _tlg_FOR_imp99(f,a0,a1,a2,a3,a4,a5,a6,a7,a8,a9,a10,a11,a12,a13,a14,a15,a16,a17,a18,a19,a20,a21,a22,a23,a24,a25,a26,a27,a28,a29,a30,a31,a32,a33,a34,a35,a36,a37,a38,a39,a40,a41,a42,a43,a44,a45,a46,a47,a48,a49,a50,a51,a52,a53,a54,a55,a56,a57,a58,a59,a60,a61,a62,a63,a64,a65,a66,a67,a68,a69,a70,a71,a72,a73,a74,a75,a76,a77,a78,a79,a80,a81,a82,a83,a84,a85,a86,a87,a88,a89,a90,a91,a92,a93,a94,a95,a96,a97,a98) f(0,a0) f(1,a1) f(2,a2) f(3,a3) f(4,a4) f(5,a5) f(6,a6) f(7,a7) f(8,a8) f(9,a9) f(10,a10) f(11,a11) f(12,a12) f(13,a13) f(14,a14) f(15,a15) f(16,a16) f(17,a17) f(18,a18) f(19,a19) f(20,a20) f(21,a21) f(22,a22) f(23,a23) f(24,a24) f(25,a25) f(26,a26) f(27,a27) f(28,a28) f(29,a29) f(30,a30) f(31,a31) f(32,a32) f(33,a33) f(34,a34) f(35,a35) f(36,a36) f(37,a37) f(38,a38) f(39,a39) f(40,a40) f(41,a41) f(42,a42) f(43,a43) f(44,a44) f(45,a45) f(46,a46) f(47,a47) f(48,a48) f(49,a49) f(50,a50) f(51,a51) f(52,a52) f(53,a53) f(54,a54) f(55,a55) f(56,a56) f(57,a57) f(58,a58) f(59,a59) f(60,a60) f(61,a61) f(62,a62) f(63,a63) f(64,a64) f(65,a65) f(66,a66) f(67,a67) f(68,a68) f(69,a69) f(70,a70) f(71,a71) f(72,a72) f(73,a73) f(74,a74) f(75,a75) f(76,a76) f(77,a77) f(78,a78) f(79,a79) f(80,a80) f(81,a81) f(82,a82) f(83,a83) f(84,a84) f(85,a85) f(86,a86) f(87,a87) f(88,a88) f(89,a89) f(90,a90) f(91,a91) f(92,a92) f(93,a93) f(94,a94) f(95,a95) f(96,a96) f(97,a97) f(98,a98)

#pragma endregion

#pragma region Declarations

/*
Core field types and flags.
*/
enum TlgIn_t
{
    TlgInNULL, // Invalid type.
    TlgInUNICODESTRING,
    TlgInANSISTRING,
    TlgInINT8,
    TlgInUINT8,
    TlgInINT16,
    TlgInUINT16,
    TlgInINT32,
    TlgInUINT32,
    TlgInINT64,
    TlgInUINT64,
    TlgInFLOAT,
    TlgInDOUBLE,
    TlgInBOOL32,
    TlgInBINARY,
    TlgInGUID,
    _TlgInPOINTER_unsupported,
    TlgInFILETIME,
    TlgInSYSTEMTIME,
    TlgInSID,
    TlgInHEXINT32,
    TlgInHEXINT64,
    TlgInCOUNTEDSTRING,     // TDH_INTYPE_MANIFEST_COUNTEDSTRING
    TlgInCOUNTEDANSISTRING, // TDH_INTYPE_MANIFEST_COUNTEDANSISTRING
    _TlgInSTRUCT,           // TDH_INTYPE_RESERVED24
    TlgInCOUNTEDBINARY,     // TDH_INTYPE_MANIFEST_COUNTEDBINARY
    // New values go above this line, but _TlgInMax must not exceed 32.
    _TlgInMax,
    TlgInINTPTR  = sizeof(void*) == 8 ? TlgInINT64    : TlgInINT32,
    TlgInUINTPTR = sizeof(void*) == 8 ? TlgInUINT64   : TlgInUINT32,
    TlgInPOINTER = sizeof(void*) == 8 ? TlgInHEXINT64 : TlgInHEXINT32,
    TlgInLONG    = sizeof(LONG)  == 8 ? TlgInINT64    : TlgInINT32,
    TlgInULONG   = sizeof(ULONG) == 8 ? TlgInUINT64   : TlgInUINT32,
    TlgInHEXLONG = sizeof(ULONG) == 8 ? TlgInHEXINT64 : TlgInHEXINT32,
    _TlgInCcount = 32, // Indicates that field metadata contains a const-array-count tag.
    TlgInVcount = 64,  // Indicates that field data contains variable-array-count tag.
    _TlgInChain = 128, // Indicates that field metadata contains a TlgOut tag.
    _TlgInCustom = TlgInVcount | _TlgInCcount, // Indicates that the field uses a custom serializer.
    _TlgInTypeMask = 31,
    _TlgInCountMask = TlgInVcount | _TlgInCcount,
    _TlgInFlagMask = _TlgInChain | TlgInVcount | _TlgInCcount
};
_tlg_CASSERT(_TlgInMax <= _TlgInCcount, "Too many TlgIn_t values.");
#define _tlg_AssertValidPackedMetadataInType(inType) \
    _tlg_CASSERT( \
        ((inType)&_TlgInTypeMask) != _TlgInPOINTER_unsupported && \
        ((inType)&_TlgInTypeMask) != _TlgInSTRUCT && \
        ((inType)|_TlgInTypeMask|TlgInVcount) == (_TlgInTypeMask|TlgInVcount) && \
        (inType) != (TlgInBINARY|TlgInVcount), \
        "Invalid packed metadata in_type: " #inType)

/*
Extended field formatting and flags.
*/
enum TlgOut_t
{
    TlgOutNULL,
    TlgOutNOPRINT,
    TlgOutSTRING,
    TlgOutBOOLEAN, // Used with InType = uint8
    TlgOutHEX, // Used with InType = uint8, uint16, uint32, uint64, binary
    TlgOutPID, // Used with InType = uint32
    TlgOutTID, // Used with InType = uint32
    TlgOutPORT, // Used with InType = uint16, encoded as big-endian
    TlgOutIPV4, // Used with InType = uint32
    TlgOutIPV6, // Used with InType = binary
    TlgOutSOCKETADDRESS, // Used with InType = binary
    TlgOutXML, // Used with InType = unicodestring, ansistring (counted and nul-terminated)
    TlgOutJSON, // Used with InType = unicodestring, ansistring (counted and nul-terminated)
    TlgOutWIN32ERROR, // Used with InType = uint32
    TlgOutNTSTATUS, // Used with InType = uint32
    TlgOutHRESULT, // Used with InType = int32
    TlgOutFILETIME,
    TlgOutSIGNED,
    TlgOutUNSIGNED,
    // RS1+: Values 32 and higher are the same as the corresponding TDH_OUTTYPE values.
    TlgOutUTF8 = 35, // Used with InType = ansistring (counted and nul-terminated)
    TlgOutPKCS7_WITH_TYPE_INFO = 36, // Used with InType = binary
    TlgOutCODE_POINTER = 37, // Used with InType = pointer
    TlgOutDATETIME_UTC = 38, // Used with InType = FILETIME or SYSTEMTIME.
    _TlgOutMax,
    _TlgOutChain = 128,
    _TlgOutTypeMask = _TlgOutChain - 1
};
_tlg_CASSERT(_TlgOutMax <= _TlgOutChain, "Too many TlgOut_t values.");

/*
This comment defines the TraceLogging encoding format.
TraceLogging events are sent to ETW as described in this section.

*** General concepts:

- Each event has metadata and data.
- The metadata describes invariant characteristics of the event -- information
  that is the same for each instance of the event.
- The data describes variable characteristics of the event -- information that
  is different each time the event is written.
- The metadata includes provider characteristics, event characteristics, and
  field characteristics.
- Each provider has a name and may optionally have other characteristics.
- Each event has a name and may optionally have fields and tags.
- Each field has a name, a type, and may optionally have tags.
- Field types can be considered as a combination of InType, OutType, and arity.
  - InType defines how a field size is determined, and also defines a default
    formatting behavior that will be used if no OutType is given. For example,
    TlgInINT32 indicates that field size is 4 and that if no OutType is
    given, the default formatting should be "signed decimal".
  - If provided, OutType provides additional formatting information. For
    example, TlgOutHEX would set formatting to "unsigned hexadecimal".
  - If provided, arity indicates whether the field is a scalar (one value),
    a fixed-length array (the same number of values for all events), or a
    variable-length array (different events may have a different number of
    values).
- TraceLogging supports grouping fields into structures.
  - Structures can contain other structures (i.e. nesting is supported).
  - Structures can contain arrays, and array-of-structure is supported.
- Events and fields can optionally set tags. Tags is a 28-bit metadata value
  with user-defined semantics. For example, event tag 0x01000000 might be
  defined as indicating a high-priority event, or field tag 0x02000000 might be
  defined as indicating a field that contains potentially-private information.

*** How to initialize a TraceLogging provider:

- Call EventRegister.
- Call EventSetInformation to set your provider's characteristics and enable
  the use of the Reserved field in EVENT_DATA_DESCRIPTOR.

Example:

EventRegister(&ProviderId, NULL, NULL, &RegHandle);
EventSetInformation(RegHandle, EventProviderSetTraits, &ProviderMetadata, ProviderMetadata.Size);

*** How to send a TraceLogging event to ETW:

- Determine N = the number of EVENT_DATA_DESCRIPTORs you need for your data.
- Allocate N + 2 EVENT_DATA_DESCRIPTORs.
- Use the first EVENT_DATA_DESCRIPTOR for provider metadata.
- Use the second EVENT_DATA_DESCRIPTOR for event metadata.
- Use the remaining EVENT_DATA_DESCRIPTORs for your data.

Example:

EVENT_DATA_DESCRIPTOR DataDescriptors[N + 2];
DataDescriptors[0].Ptr = (ULONGLONG)(ULONG_PTR)&ProviderMetadata; // Defined below
DataDescriptors[0].Size = ProviderMetadata.Size;
DataDescriptors[0].Reserved = EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA;
DataDescriptors[1].Ptr = (ULONGLONG)(ULONG_PTR)&EventMetadata; // Defined below
DataDescriptors[1].Size = EventMetadata.Size;
DataDescriptors[1].Reserved = EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA;
// ...
// Use EventDataDescCreate(...) to populate the remaining N DataDescriptors
// with the data for your event, e.g.
// EventDataDescCreate(&DataDescriptors[2], &myData, sizeof(myData));
// Note that the data layout in the remaining N DataDescriptors must match up
// with the fields defined in EventMetadata.
// ...
EventWrite(RegHandle, &EventDescriptor, N + 2, DataDescriptors);

*** How to encode metadata:

// ProviderMetadata:
// This pseudo-structure is the layout of the "provider traits" referenced by
// EventProviderSetTraits, EtwGetTraitFromProviderTraits,
// EVENT_HEADER_EXT_TYPE_PROV_TRAITS, and
// EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA.
// It provides the provider's name, plus other optional information such as group ID.
struct ProviderMetadata // Variable-length pseudo-structure, byte-aligned, tightly-packed.
{
    UINT16 Size; // = sizeof(ProviderMetadata)
    char Name[]; // UTF-8 nul-terminated provider name
    ProviderMetadataChunk AdditionalProviderInfo[]; // 0 or more chunks of data.
};

// ProviderMetadataChunk:
struct ProviderMetadataChunk // Variable-length pseudo-structure, byte-aligned, tightly-packed.
{
    UINT16 Size; // = sizeof(ProviderMetadataChunk)
    UINT8 Type; // Value from the ETW_PROVIDER_TRAIT_TYPE enumeration.
    AnyType Data;
};

// EventMetadata:
// This pseudo-structure is the layout of the "event metadata" referenced by
// EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA.
// It provides the event's name, event tags, and field information.
struct EventMetadata // Variable-length pseudo-structure, byte-aligned, tightly-packed.
{
    UINT16 Size; // = sizeof(EventMetadata)
    UINT8 Extension[]; // 1 or more bytes. Read until you hit a byte with high bit unset.
    char Name[]; // UTF-8 nul-terminated event name
    FieldMetadata Fields[]; // 0 or more field definitions.
};

// FieldMetadata:
struct FieldMetadata // Variable-length pseudo-structure, byte-aligned, tightly-packed.
{
    char Name[]; // UTF-8 nul-terminated field name
    UINT8 InType; // Values from the TlgIn enumeration.
    UINT8 OutType; // TlgOut enumeration. Only present if (InType & 128) == 128.
    UINT8 Extension[]; // Only present if OutType is present and (OutType & 128) == 128. Read until you hit a byte with high bit unset.
    UINT16 ValueCount;  // Only present if (InType & CountMask) == Ccount.
    UINT16 TypeInfoSize; // Only present if (InType & CountMask) == Custom.
    char TypeInfo[TypeInfoSize]; // Only present if (InType & CountMask) == Custom.
};

Each field has InType and OutType. The InType controls how the field's size is
determined and also implies a default formatting behavior that should be used
if OutType is 0 or not present. For example, TlgInINT32 indicates that the
field's size is 4 and that if no OutType is given (or if the decoder does not
understand the given OutType), the field formatting should be "signed decimal".

Some InType values have special behaviors or limitations:

- TlgInNULL: Invalid field type.
- TlgInBINARY: Array of BINARY should not be used. Array of BINARY is legal,
  but TDH cannot decode a field with type array of BINARY.
- _TlgInPOINTER_unsupported: Do not use. This value is reserved because the
  corresponding TDH value is TDH_INTYPE_POINTER. TraceLogging does not directly
  support a POINTER type. Instead, TraceLogging defines TlgInPOINTER as
  TlgInHEXINT32 for 32-bit binaries or TlgInHEXINT64 for 64-bit binaries.
- A struct has no data for itself, but it groups the following N logical fields
  together into a single logical field. The value N is encoded in the OutType.
  Arrays of struct are allowed, and structs can contain other structs (or
  arrays of structs).

The EventMetadata and FieldMetadata structures contain an Extension field. The
size of the Extension field is variable and unlimited. The decoder should
consume bytes until it has consumed a byte with the high bit set to 0.

At present, only the first 4 bytes of Extension are defined (the remaining
bytes of the Extension field are reserved for future use and should be
ignored). The first 4 bytes (if present) each contribute 7 bits of "Tag" data
to the event or field, for up to 28 bits of "Tag" data. Tags are encoded
high-bits first. For example, if the first byte of Extension is 0x12, that
means the Extension field is 1 byte in length (high bit is unset), and value of
the Tag data is 0x02400000 (0x12 low 7 bits are 0010010, used as bits 21..27 of
the Tag data).

*** Encoding data:

Each scalar field's data is appended end-to-end with no alignment or padding.

A fixed-length array is encoded by appending the data for N values end-to-end
with no alignment or padding, where N is the number of values as encoded in
the field's metadata. This rule applies even if the data is variable-length or
complex (e.g. it applies even if the field is a structure). Note that an array
length of 0 for a fixed-length array should not be used.

A variable-length array is encoded as a UINT16 containing the number of values
followed by the data for those values, end-to-end.

Most value types are fixed size (e.g. sizeof(INT32) == 4), but the following
value types are variable-size:

- SID: size is 8 + 4*SubAuthorityCount.
- ANSISTRING: size is strlen(value) + 1. (Use strnlen for safe parsing.)
- UNICODESTRING: size is wcslen(value)*2 + 2. (Use wcsnlen for safe parsing.)
- BINARY, COUNTEDSTRING, COUNTEDANSISTRING: first two bytes are UINT16
  bytecount.
*/

/*
The following information is not part of the TraceLogging protocol - it
describes how the information needed by this header is stored in memory.
The following information is internal implementation information and may
change in future revisions of this header.
*/

/*
Type codes for metadata blobs in the binary.
*/
enum _TlgBlob_t
{
    _TlgBlobNone,
    _TlgBlobEnd,
    _TlgBlobProvider,
    _TlgBlobEvent3,
    _TlgBlobProvider3,
    _TlgBlobEvent2,
    _TlgBlobEvent4, // Same as _TlgBlobEvent3 but event id is always 0.
    _TlgBlobMax
};

enum _TlgFlags_t
{
    _TlgFlagsNone = 0,
    _TlgFlagsPtr64 = 1
};

enum _TlgOptions_t
{
    _TlgOptionNone = 0,
    _TlgOptionGroup = 1 // = EtwProviderTraitTypeGroup
};

/*
A module's ETW metadata consists of a _TraceLoggingMetadata_t (8-byte aligned)
followed by metadata blobs (1-byte aligned). Extra zero-byte padding may be
present before or after blobs and must be ignored. (The extra padding is most
obvious in unoptimized builds.)
*/
struct _TraceLoggingMetadata_t
{
    UINT32 Signature; // = _tlg_MetadataSignature = "ETW0"
    UINT16 Size;      // = sizeof(_TraceLoggingMetadata_t)
    UINT8  Version;   // = _tlg_MetadataVersion
    UINT8  Flags;     // = _tlg_MetadataFlags
    UINT64 Magic;     // = _tlg_MetadataMagic
};

#pragma pack(push,1) // Tight packing for metadata.

/*
This is the data stored in the binary to describe a TraceLogging provider.
This structure may change in future revisions of this header.
The current design has the structure start with information needed by the
functions in this header (Type, ProviderId), and the structure ends with
information that will be passed on to ETW (provider traits).
Variable-length structure, byte-aligned, tightly-packed.
Actual size is: sizeof(Type) + _tlg_PROVIDER_METADATA_PREAMBLE + RemainingSize.
*/
struct _tlgProviderMetadata_t
{
    UINT8 Type; // = _TlgBlobProvider3
    GUID ProviderId;
#define _tlg_PROVIDER_METADATA_PREAMBLE 16 // = sizeof(ProviderId)
    UINT16 RemainingSize; // = sizeof(RemainingSize + ProviderName)
    /*
    char ProviderName[sizeof("providerName")]; // UTF-8 nul-terminated provider name
    for each additional chunk of metadata {
        UINT16 ChunkSize;
        UINT8 ChunkType;
        UINT8 ChunkData[ChunkSize - 3];
    }
    */
};

/*
This is the data stored in the binary to describe a TraceLogging event.
This structure may change in future revisions of this header.
The current design has the structure start with information needed by the
functions in this header (Type, Channel, etc.), and the structure ends with
information that will be passed on to ETW (event traits).
Variable-length structure, byte-aligned, tightly-packed.
Actual size is: sizeof(Type) + _tlg_EVENT_METADATA_PREAMBLE + RemainingSize
*/
struct _tlgEventMetadata_t
{
    UINT8 Type; // = _TlgBlobEvent4
    UCHAR Channel;
    UCHAR Level;
    UCHAR Opcode;
    ULONGLONG Keyword;
#define _tlg_EVENT_METADATA_PREAMBLE 11 // sizeof(Channel + Level + Opcode + Keyword)
    UINT16 RemainingSize; // = sizeof(RemainingSize + Tags + EventName + Fields)
    /*
    UINT8 Tags[]; // 1 or more bytes. Read until you hit a byte with high bit unset.
    char EventName[sizeof("eventName")]; // UTF-8 nul-terminated event name
    for each field {
        char FieldName[sizeof("fieldName")]; // UTF-8 nul-terminated field name
        UINT8 InType; // TlgIn_t
        UINT8 OutType; // TlgOut_t, only present if (InType & Chain) == Chain.
        UINT8 Tags[]; // Only present if OutType is present and (OutType & Chain) == Chain. Read until you hit a byte with high bit unset.
        UINT16 ValueCount;  // Only present if (InType & CountMask) == Ccount.
        UINT16 TypeInfoSize; // Only present if (InType & CountMask) == Custom.
        char TypeInfo[TypeInfoSize]; // Only present if (InType & CountMask) == Custom.
    }
    */
};

#pragma pack(8) // Default packing for other structures.

/*
Data for TraceLoggingHProvider.
Note: The _tlgProvider_t type is an internal implementation detail. Do
not use the _tlgProvider_t type directly, as it may be changed or renamed in
future versions of this header. Use the TraceLoggingHProvider type and the
provided accessor functions instead of reading members directly.
*/
struct _tlgProvider_t
{
    UINT32 LevelPlus1;
    UINT16 const UNALIGNED* ProviderMetadataPtr; // Points to the RemainingSize member of provider metadata.
    ULONGLONG KeywordAny;
    ULONGLONG KeywordAll;
    REGHANDLE RegHandle;
    TLG_PENABLECALLBACK EnableCallback;
    PVOID CallbackContext;
};

/*
Tag type. Has no effect on code. Can be used for detection of wrapper type for
a provider (such as TraceLoggingActivity). The tag indicates that the first
template parameter of the wrapper type is a reference to the provider handle
variable.

Example:
template<
    TraceLoggingHProvider const& provider,
    typename TlgReflectorTag = _TlgReflectorTag_Param0IsHProvider>
class Activity
{
    ...
};
*/
struct _TlgReflectorTag_Param0IsHProvider;

/*
Tag type. Has no effect on code. Can be used for detection of a wrapper type
for a provider (such as TraceLoggingActivity). The tag indicates that the first
template parameter of the wrapper type is a type T that wraps a provider, such
that T::Provider() returns an HProvider.

Example:
template<
    typename ProviderType,
    typename TlgReflectorTag = _TlgReflectorTag_Param0IsProviderType>
class Activity
{
    static TraceLoggingHProvider Provider()
    {
        return ProviderType::Provider();
    }
};
*/
struct _TlgReflectorTag_Param0IsProviderType;

#pragma pack(pop)

#pragma endregion

#pragma region Helper functions

#define _tlg_MetadataSignature   '0WTE' // "ETW0"
#define _tlg_MetadataVersion     0
#define _tlg_MetadataFlags       (sizeof(void*)==8?1:0)
#define _tlg_MetadataMagic       0xBB8A052B88040E86

#define _tlg_MetaDataDescCount   2

#define _tlgSegMetadataBegin     _tlg_STRINGIZE(_tlg_PASTE2(TLG_METADATA_SEGMENT_BASE, $zETW0))
#define _tlgSegMetadataEvents    _tlg_STRINGIZE(_tlg_PASTE2(TLG_METADATA_SEGMENT_BASE, $zETW1))
#define _tlgSegMetadataProviders _tlg_STRINGIZE(_tlg_PASTE2(TLG_METADATA_SEGMENT_BASE, $zETW2))
#define _tlgSegMetadataEnd       _tlg_STRINGIZE(_tlg_PASTE2(TLG_METADATA_SEGMENT_BASE, $zETW9))

#ifdef __cplusplus
#pragma detect_mismatch("TLG_METADATA_SEGMENT_BASE", _tlg_STRINGIZE(TLG_METADATA_SEGMENT_BASE))
#endif // __cplusplus

#pragma section(_tlgSegMetadataBegin, read)
#pragma section(_tlgSegMetadataEvents, read)
#pragma section(_tlgSegMetadataProviders, read)
#pragma section(_tlgSegMetadataEnd, read)

#define _tlgGetLengthSid(pSid) (8u + (pSid)->SubAuthorityCount * 4u)

// ********** NO FUNCTION DEFINITIONS ABOVE THIS POINT ***********************
#pragma code_seg(push)

// ********** PAGED CODE ONLY ************************************************
#pragma code_seg(TLG_PAGED_SEGMENT)

_IRQL_requires_max_(PASSIVE_LEVEL)
_IRQL_requires_same_
TLG_INLINE
void NTAPI
_tlgEnableCallback(
    _In_ LPCGUID pSourceId,
    _In_ ULONG callbackType,
    _In_ UCHAR level,
    _In_ ULONGLONG keywordAny,
    _In_ ULONGLONG keywordAll,
    _In_opt_ PEVENT_FILTER_DESCRIPTOR pFilterData,
    _Inout_opt_ PVOID pCallbackContext)
{
    TLG_PAGED_CODE();
    if (pCallbackContext)
    {
        struct _tlgProvider_t* pProvider = (struct _tlgProvider_t*)pCallbackContext;
        switch (callbackType)
        {
        case 0: // EVENT_CONTROL_CODE_DISABLE_PROVIDER
            pProvider->LevelPlus1 = 0;
            break;
        case 1: // EVENT_CONTROL_CODE_ENABLE_PROVIDER
            pProvider->LevelPlus1 = level != 0 ? (UINT32)level + 1u : 256u;
            pProvider->KeywordAny = keywordAny;
            pProvider->KeywordAll = keywordAll;
            break;
        }

        if (pProvider->EnableCallback)
        {
            pProvider->EnableCallback(
                pSourceId,
                callbackType,
                level,
                keywordAny,
                keywordAll,
                pFilterData,
                pProvider->CallbackContext);
        }
    }
}

_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
void _tlg_CALL
TraceLoggingUnregister(TraceLoggingHProvider _Inout_ hProvider)
{
    struct _tlgProvider_t* pProvider = (struct _tlgProvider_t*)hProvider;
    REGHANDLE RegHandle;
    TLG_PAGED_CODE();
    RegHandle = hProvider->RegHandle;
    pProvider->LevelPlus1 = 0;
    pProvider->RegHandle = 0;
    TLG_EVENT_UNREGISTER(RegHandle);
}

_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
TraceLoggingRegister(
    TraceLoggingHProvider _Inout_ hProvider)
{
    TLG_PAGED_CODE();
    return TraceLoggingRegisterEx(hProvider, TLG_NULL, TLG_NULL);
}

_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
TraceLoggingRegisterEx(
    TraceLoggingHProvider _Inout_ hProvider,
    _In_opt_ TLG_PENABLECALLBACK pEnableCallback,
    _In_opt_ PVOID pCallbackContext)
{
    TLG_STATUS status;
    struct _tlgProvider_t* pProvider = (struct _tlgProvider_t*)hProvider;
    GUID const providerId = TraceLoggingProviderId(pProvider);
    TLG_PAGED_CODE();

    if (pProvider->RegHandle != 0)
    {
        // TraceLoggingRegister[Ex] was called with an hProvider that is
        // currently registered. This leaks the provider registration and can
        // cause memory corruption during provider callback. This is a serious
        // bug in the code that called TraceLoggingRegister[Ex].
        _tlg_FASTFAIL(5); // 5 = FAST_FAIL_INVALID_ARG
    }

    pProvider->EnableCallback = pEnableCallback;
    pProvider->CallbackContext = pCallbackContext;
    status = TLG_EVENT_REGISTER(
        &providerId,
        &_tlgEnableCallback,
        pProvider,
        &pProvider->RegHandle);
    if (status != 0)
    {
#ifndef _ETW_KM_
        status = HRESULT_FROM_WIN32(status);
#endif
    }
    else
    {
        UINT16 const cbMetadata = *pProvider->ProviderMetadataPtr;
        (void)TraceLoggingSetInformation(
            pProvider,
            (EVENT_INFO_CLASS)2, // EventProviderSetTraits
            (PVOID)pProvider->ProviderMetadataPtr,
            cbMetadata);
    }
    return status;
}

_IRQL_requires_max_(PASSIVE_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
TraceLoggingSetInformation(
    TraceLoggingHProvider _Inout_ hProvider,
    _In_ EVENT_INFO_CLASS informationClass,
    _In_reads_bytes_opt_(cbInformation) PVOID pvInformation,
    _In_ ULONG cbInformation)
{
    TLG_STATUS status;

#if TLG_HAVE_EVENT_SET_INFORMATION == 1

    TLG_PAGED_CODE();
#pragma warning(suppress: 6387) // It's ok for pvInformation to be null if cbInformation is 0.
    status = TLG_EVENT_SET_INFORMATION(
        hProvider->RegHandle,
        informationClass,
        pvInformation,
        cbInformation);
#ifndef _ETW_KM_
    status = HRESULT_FROM_WIN32(status);
#endif

#elif TLG_HAVE_EVENT_SET_INFORMATION == 2

#ifdef _ETW_KM_
    typedef NTSTATUS(NTAPI* PFEtwSetInformation)(
        _In_ REGHANDLE RegHandle,
        _In_ EVENT_INFO_CLASS InformationClass,
        _In_reads_bytes_opt_(InformationLength) PVOID EventInformation,
        _In_ ULONG InformationLength);
    static UNICODE_STRING strEtwSetInformation = {
        sizeof(L"EtwSetInformation") - 2,
        sizeof(L"EtwSetInformation") - 2,
        (PWCH)L"EtwSetInformation"
    };
    PFEtwSetInformation pfEtwSetInformation;
    TLG_PAGED_CODE();
    status = STATUS_NOT_SUPPORTED;
    pfEtwSetInformation = (PFEtwSetInformation)MmGetSystemRoutineAddress(&strEtwSetInformation);
    if (pfEtwSetInformation)
    {
        status = pfEtwSetInformation(
            hProvider->RegHandle,
            informationClass,
            pvInformation,
            cbInformation);
    }
#else // _ETW_KM_
    HMODULE hEventing;
    status = ERROR_NOT_SUPPORTED;
    if (GetModuleHandleExW(0, L"api-ms-win-eventing-provider-l1-1-0.dll", &hEventing) ||
        GetModuleHandleExW(0, L"advapi32.dll", &hEventing))
    {
        typedef ULONG(WINAPI* PFEventSetInformation)(
            _In_ REGHANDLE RegHandle,
            _In_ EVENT_INFO_CLASS InformationClass,
            _In_reads_bytes_opt_(InformationLength) PVOID EventInformation,
            _In_ ULONG InformationLength);
        PFEventSetInformation pfEventSetInformation =
            (PFEventSetInformation)GetProcAddress(hEventing, "EventSetInformation");
        if (pfEventSetInformation)
        {
            status = pfEventSetInformation(
                hProvider->RegHandle,
                informationClass,
                pvInformation,
                cbInformation);
        }

        FreeLibrary(hEventing);
    }
    status = HRESULT_FROM_WIN32(status);
#endif // _ETW_KM_

#else // TLG_HAVE_EVENT_SET_INFORMATION == 0

    (void)hProvider;
    (void)informationClass;
    (void)pvInformation;
    (void)cbInformation;

  #ifdef _ETW_KM_
    TLG_PAGED_CODE();
    status = STATUS_ENTRYPOINT_NOT_FOUND;
  #else // _ETW_KM_
    status = HRESULT_FROM_WIN32(ERROR_NOT_SUPPORTED);
  #endif // _ETW_KM_

#endif // TLG_HAVE_EVENT_SET_INFORMATION

    return status;
}

// ********** NON-PAGED CODE ONLY ********************************************
#pragma code_seg(TLG_NONPAGED_SEGMENT)

extern struct _TraceLoggingMetadata_t const
    __declspec(selectany allocate(_tlgSegMetadataBegin))
    _TraceLoggingMetadata = {
    _tlg_MetadataSignature,
    sizeof(_TraceLoggingMetadata),
    _tlg_MetadataVersion,
    _tlg_MetadataFlags,
    _tlg_MetadataMagic
};

extern UINT8 const
    __declspec(selectany allocate(_tlgSegMetadataEnd))
    _TraceLoggingMetadataEnd = _TlgBlobEnd;

/*
Marking _tlgWriteCommon as __forceinline helps ensure minimal code size.
In most cases, there is no difference between __inline or __forceinline.
In a few cases __inline results in larger code. This is most common when both
_tlgWriteTransfer and _tlgWriteEx are used in the same module.

Usually, _tlgWriteCommon gets inlined into _tlgWriteTransfer, and _tlgWrite
does not get inlined into call sites. This is generally the best result.

Without __forceinline, _tlgWriteTransfer sometimes gets inlined into call
sites, and _tlgWriteCommon does not get inlined. The result is slightly larger
code at the call site with little or no benefit in performance.
*/
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
void _tlg_CALL
_tlgWriteCommon(
    _In_ TraceLoggingHProvider hProvider,
    void const* pEventMetadata, // Buffer size determined from buffer content
    _Out_writes_(2) ULONGLONG* pDesc,
    _Out_writes_(2) EVENT_DATA_DESCRIPTOR* pData)
{
    _tlg_CASSERT(2 == _tlg_MetaDataDescCount, "Caller out of sync with _tlg_MetaDataDescCount.");
    _tlg_CASSERT(_tlg_EVENT_METADATA_PREAMBLE == 1 + 2 + 4 + 4, "Caller out of sync with _tlg_EVENT_METADATA_PREAMBLE.");
    UINT8 const* p = (UINT8 const*)pEventMetadata;

    ((UINT32*)pDesc)[0] = *p << 24;  p += 1; // Id=0, Version=0, Channel=*p
    ((UINT32*)pDesc)[1] = *(UINT16 UNALIGNED const*)p; p += 2; // Level, Opcode, Task
    pDesc[1] = *(ULONGLONG UNALIGNED const*)p; p += 8; // Keyword
    pData[0].Ptr = (ULONGLONG)(ULONG_PTR)hProvider->ProviderMetadataPtr;
    pData[0].Size = *hProvider->ProviderMetadataPtr;
    pData[0].Reserved = 2; // = EVENT_DATA_DESCRIPTOR_TYPE_PROVIDER_METADATA
    pData[1].Ptr = (ULONGLONG)(ULONG_PTR)p;
    pData[1].Size = *(UINT16 UNALIGNED const*)p;
    pData[1].Reserved = 1; // = EVENT_DATA_DESCRIPTOR_TYPE_EVENT_METADATA
    _tlg_ASSERT((void const*)&_TraceLoggingMetadata < (void const*)pEventMetadata, "Event metadata not in metadata segment.");
    _tlg_ASSERT((void const*)&_TraceLoggingMetadataEnd > (void const*)pEventMetadata, "Event metadata not in metadata segment.");

    /*
    Passing provider handles from one module to another is illegal.
    Please fix your code to avoid doing this.
    */
    _tlg_ASSERT(
        (void const*)&_TraceLoggingMetadata < (void const*)hProvider->ProviderMetadataPtr,
        "Provider handles must not be used outside of the module in which it was declared. Please don't share provider handles with other DLLs.");
    _tlg_ASSERT(
        (void const*)&_TraceLoggingMetadataEnd > (void const*)hProvider->ProviderMetadataPtr,
        "Provider handles must not be used outside of the module in which it was declared. Please don't share provider handles with other DLLs.");

    /*
    Force references to _TraceLoggingMetadata and _TraceLoggingMetadataEnd:
    */
    {
        int cbMetadata;
        int volatile volatileVar;
        cbMetadata = (int)((LPCCH)&_TraceLoggingMetadataEnd - (LPCCH)&_TraceLoggingMetadata);
#if defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC)
        __iso_volatile_store32(&volatileVar, cbMetadata);
#else
#pragma warning(suppress: 28931) // Unused assignment
        volatileVar = cbMetadata;
        UNREFERENCED_PARAMETER(volatileVar);
#endif
    }
}

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
_tlgWriteTransfer(
    _In_ TraceLoggingHProvider hProvider,
    void const* pEventMetadata, // Buffer size determined from buffer content
    _In_opt_ LPCGUID pActivityId,
    _In_opt_ LPCGUID pRelatedActivityId,
    _In_range_(2, 128) UINT32 cData,
    _Inout_cap_(cData) EVENT_DATA_DESCRIPTOR* pData)
{
    TLG_STATUS status;
    ULONGLONG desc[2];
    _tlgWriteCommon(hProvider, pEventMetadata, desc, pData);
    _tlg_ASSERT(cData <= 128, "Too many data values.");
    status = TLG_EVENT_WRITE_TRANSFER(hProvider->RegHandle, (EVENT_DESCRIPTOR const*)desc, pActivityId, pRelatedActivityId, cData, pData);
    return status;
}

#if _tlg_ENABLE_TraceLoggingWriteEx

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
TLG_STATUS _tlg_CALL
_tlgWriteEx(
    _In_ TraceLoggingHProvider hProvider,
    void const* pEventMetadata, // Buffer size determined from buffer content
    ULONG64 filter,
    ULONG flags,
    _In_opt_ LPCGUID pActivityId,
    _In_opt_ LPCGUID pRelatedActivityId,
    _In_range_(2, 128) UINT32 cData,
    _Inout_cap_(cData) EVENT_DATA_DESCRIPTOR* pData)
{
    TLG_STATUS status;
    ULONGLONG desc[2];
    _tlgWriteCommon(hProvider, pEventMetadata, desc, pData);
    _tlg_ASSERT(cData <= 128, "Too many data values.");
    status = TLG_EVENT_WRITE_EX(hProvider->RegHandle, (EVENT_DESCRIPTOR const*)desc, filter, flags, pActivityId, pRelatedActivityId, cData, pData);
    return status;
}

#endif // _tlg_ENABLE_TraceLoggingWriteEx

/*
_tlgKeywordOn: Internal helper function. Do not call directly. This function
may change or be renamed in future versions of this header.
Use TraceLoggingProviderEnabled instead.
*/
_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
BOOLEAN _tlg_CALL
_tlgKeywordOn(
    TraceLoggingHProvider _In_ hProvider,
    ULONGLONG keyword)
{
    return keyword == 0 || (
        (keyword & hProvider->KeywordAny) &&
        (keyword & hProvider->KeywordAll) == hProvider->KeywordAll);
}

_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
BOOLEAN _tlg_CALL
TraceLoggingProviderEnabled(
    TraceLoggingHProvider _In_ hProvider,
    UCHAR eventLevel,
    ULONGLONG eventKeyword)
{
    return eventLevel < hProvider->LevelPlus1 && _tlgKeywordOn(hProvider, eventKeyword);
}

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
GUID _tlg_CALL
TraceLoggingProviderId(
    TraceLoggingHProvider _In_ hProvider)
{
    GUID const UNALIGNED* pProviderId =
        &CONTAINING_RECORD(hProvider->ProviderMetadataPtr, struct _tlgProviderMetadata_t, RemainingSize)->ProviderId;
    return *pProviderId;
}

#if !defined(__cplusplus)

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
void _tlg_CALL
_tlgCreate1Sz_char(
    _Out_ PEVENT_DATA_DESCRIPTOR pDesc,
    _In_opt_z_ char const* psz)
{
    char const* pch;
    unsigned cb;
    if (psz)
    {
        pch = psz;
        cb = ((unsigned)strlen(pch) + 1) * sizeof(pch[0]);
    }
    else
    {
        pch = "";
        cb = sizeof(pch[0]);
    }
    EventDataDescCreate(pDesc, pch, cb);
}

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
void _tlg_CALL
_tlgCreate1Sz_wchar_t(
    _Out_ PEVENT_DATA_DESCRIPTOR pDesc,
    _In_opt_z_ wchar_t const* psz)
{
    wchar_t const* pch;
    unsigned cb;
    if (psz)
    {
        pch = psz;
        cb = ((unsigned)wcslen(pch) + 1) * sizeof(pch[0]);
    }
    else
    {
        pch = L"";
        cb = sizeof(pch[0]);
    }
    EventDataDescCreate(pDesc, pch, cb);
}

/*
This is a macro instead of a function so that we don't have a dependency on
the char16_t type until the user actually uses a TraceLoggingChar16 or
TraceLoggingString16 macro, i.e. so that the user can include and use
TraceLoggingProvider.h without including uchar.h, and only needs to include
uchar.h if actually using char16_t.
Note: assumes that wchar_t and char16_t are the same type (i.e. both uint16).
*/
#define _tlgCreate1Sz_char16_t(pDesc, psz) _tlgCreate1Sz_wchar_t(pDesc, psz)

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
void _tlg_CALL
_tlgCreate2Binary(
    _Out_writes_(2) PEVENT_DATA_DESCRIPTOR pDesc,
    _In_reads_bytes_(cbValue) void const* pValue,
    _In_ ULONG cbValue)
{
    EventDataDescCreate(&pDesc[0], &pDesc[1].Size, 2);
    EventDataDescCreate(&pDesc[1], pValue, cbValue);
}

#else // !defined(__cplusplus)
} // extern "C"

template<class CH> struct _tlgCharTraits;
template<> struct _tlgCharTraits<char>
{
    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_INLINE
    _Check_return_ _Ret_z_ char const* _tlg_CALL
    empty()
    { return ""; }

    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_INLINE
    _Check_return_ size_t _tlg_CALL
    length(_In_z_ char const* psz)
    { return strlen(psz); }
};
template<> struct _tlgCharTraits<wchar_t>
{
    static TLG_INLINE
    _Check_return_ _Ret_z_ wchar_t const* _tlg_CALL
    empty()
    { return L""; }

    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_INLINE
    _Check_return_ size_t _tlg_CALL
    length(_In_z_ wchar_t const* psz)
    { return wcslen(psz); }
};
template<> struct _tlgCharTraits<char16_t>
{
    static_assert(sizeof(wchar_t) == sizeof(char16_t), "Unexpected sizeof(wchar_t)");

    static TLG_INLINE
    _Check_return_ _Ret_z_ char16_t const* _tlg_CALL
    empty()
    { return u""; }

    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_INLINE
    _Check_return_ size_t _tlg_CALL
    length(_In_z_ char16_t const* psz)
    { return wcslen(reinterpret_cast<wchar_t const*>(psz)); }
};

template<class CH>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
void _tlg_CALL
_tlgCreate1Sz(
    _Out_ PEVENT_DATA_DESCRIPTOR pDesc,
    _In_opt_z_ CH const* psz)
{
    typedef _tlgCharTraits<CH> _traits;
    CH const* pch;
    unsigned cb;
    if (psz)
    {
        pch = psz;
        cb = ((unsigned)_traits::length(pch) + 1) * sizeof(CH);
    }
    else
    {
        pch = _traits::empty();
        cb = sizeof(CH);
    }
#pragma warning(suppress: 26018) // Analysis doesn't know that _traits::length(pch) returns strlen.
    EventDataDescCreate(pDesc, pch, cb);
}

/*
_tlgIndexSequence: stores the size of the EVENT_DATA_DESC array needed and the
indices at which each value will go. The Indices are mostly sequential, e.g.
(0, 1, 2, 3, Size=4), but some indices might be skipped since some values need
two descriptors, e.g. (0, 2, 4, 5, Size=7).
*/
template<unsigned Size, unsigned... Indices>
struct _tlgIndexSequence
{
    static unsigned const size = Size;
};

/*
_tlgExtendIndexSequence: recursively determines the _tlgIndexSequence
corresponding to a list of value types.
*/
template<class BaseSequence, class... Types>
struct _tlgExtendIndexSequence;

/*
_tlgExtendIndexSequence with no types: the sequence is complete.
*/
template<unsigned Size, unsigned... Indices>
struct _tlgExtendIndexSequence<_tlgIndexSequence<Size, Indices...>>
{
    using type = _tlgIndexSequence<Size, Indices...>;
};

/*
_tlgExtendIndexSequence with at least one type:
Extend the given base sequence with the DataDescCount from the next type.
Recurse to obtain the final sequence.
*/
template<unsigned BaseSize, unsigned... BaseIndices, class NextType, class... RestTypes>
struct _tlgExtendIndexSequence<_tlgIndexSequence<BaseSize, BaseIndices...>, NextType, RestTypes...>
{
private:

    using NextSequence = _tlgIndexSequence<BaseSize + NextType::DataDescCount, BaseIndices..., BaseSize>;

public:

    using type = typename _tlgExtendIndexSequence<NextSequence, RestTypes...>::type;
};

/*
_tlgDataDescInit is a helper type that fills in the EVENT_DATA_DESCRIPTOR
array. The helper is used to extract the Indices from _tlgIndexSequence.
This helper will be unnecessary once the compiler supports fold expressions.
*/
template<class IndexSequence>
struct _tlgDataDescInit;

template<unsigned Size, unsigned... Indices>
struct _tlgDataDescInit<_tlgIndexSequence<Size, Indices...>>
{
    /*
    Fills in the EVENT_DATA_DESCRIPTOR array using the data from the wrapped
    arguments.
    */
    template<class... WrappedTys>
    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_PFORCEINLINE
    void _tlg_CALL
    Fill(
        EVENT_DATA_DESCRIPTOR* pDescriptors,
        WrappedTys const&... wrappedArgs)
    {
        /*
        warning C4100 (unreferenced parameter pDescriptors) under VC2015
        */
        (void)pDescriptors;

        /*
        Uses both the wrappedArgs and Indices parameter packs. For each pair of
        entries (one arg from wrappedArgs, one index from Indices), calls
        arg.Fill(&pDesc[index]). We do this as a function call because
        parameter pack expansions are only allowed in certain places, and the
        function call seems to be the syntax that the compiler is able to
        optimize most effectively.
        */
        EvaluateArgs(wrappedArgs.Fill(&pDescriptors[Indices])...);
    }

private:

    /*
    Does nothing. Used to create the syntax in which a parameter pack
    expansion may occur.
    */
    template<class... WrappedTys>
    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_PFORCEINLINE
    void _tlg_CALL
    EvaluateArgs(WrappedTys...) {}
};

/*
_tlgWriteTemplate: puts argument data into an EVENT_DATA_DESCRIPTOR array and
calls the event write API. This is used by _tlgWrite_imp.

The struct is templated on the event write API and the extra parameters it
needs. The Write function accepts the standard arguments, any extra parameters
needed by the event write API, then wrapped values corresponding to the data
to be included in the event. For EventWriteTransfer with an INT32 value and a
char* value, you would use something like:

_tlgWriteTemplate<
    decltype(EventWriteTransfer), EventWriteTransfer, // The API to call.
    LPCGUID, LPCGUID                                  // The API's extra args.
>::Write(
    hProvider, pEventMetadata,       // Standard arguments.
    pActivityId, pRelatedActivityId, // The API's extra args.
    _tlgWrapScalar<INT32>(value1),   // The wrapped INT32 value.
    _tlgWrapSz<char>(value2));       // The wrapped char* value.
*/
template<class WriterTy, WriterTy writer, class... WriterArgTys>
struct _tlgWriteTemplate
{
    template<class... WrappedTys>
    _IRQL_requires_max_(HIGH_LEVEL)
    static TLG_INLINE
    TLG_STATUS _tlg_CALL
    Write(
        _In_ TraceLoggingHProvider hProvider,
        void const* pEventMetadata, // Buffer size inexpressible
        WriterArgTys... writerArgs,
        WrappedTys const&... wrappedArgs)
    {
        using IndexSequence = typename _tlgExtendIndexSequence<_tlgIndexSequence<0>, WrappedTys...>::type;
        unsigned const cData = IndexSequence::size + 2;
        static_assert(cData <= 128, "Too many data values.");
        EVENT_DATA_DESCRIPTOR data[cData];
        _tlgDataDescInit<IndexSequence>::Fill(data + 2, wrappedArgs...);
        return writer(hProvider, pEventMetadata, writerArgs..., cData, data);
    }
};

/*
_tlgStorageTypeForSize: Given a value size (1, 2, 4, 8), returns a suitable
type in which the value's bits can be stored (UINT8, UINT16, UINT32, UINT64).
*/
template<unsigned Size> struct _tlgStorageTypeForSize;
template<> struct _tlgStorageTypeForSize<1> { typedef UINT8 type; };
template<> struct _tlgStorageTypeForSize<2> { typedef UINT16 type; };
template<> struct _tlgStorageTypeForSize<4> { typedef UINT32 type; };
template<> struct _tlgStorageTypeForSize<8> { typedef UINT64 type; };

/*
_tlgWrapperByVal: Type-erased handler for fixed-size by-val scalar
(max 8 bytes).
Usage: _tlgWrapScalar<ctype>(value)
*/
template<unsigned Size>
struct _tlgWrapperByVal
{
    static const unsigned DataDescCount = 1; // Consumes 1 EVENT_DATA_DESCRIPTOR.
    typedef typename _tlgStorageTypeForSize<Size>::type StorageType;

    StorageType const Value;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    explicit _tlgWrapperByVal(_In_reads_bytes_(Size) void const* ptr)
        : Value(*static_cast<StorageType const*>(ptr)) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_ EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(pDesc, &Value, Size);
        return pDesc;
    }
};

/*
_tlgWrapperByRef: Type-erased handler for fixed-size by-ref scalar (GUID,
SYSTEMTIME, FILETIME).
Usage: _tlgWrapScalar<ctype>(value)
*/
template<unsigned Size>
struct _tlgWrapperByRef
{
    static const unsigned DataDescCount = 1; // Consumes 1 EVENT_DATA_DESCRIPTOR.

    void const* const Ptr;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    explicit _tlgWrapperByRef(_In_reads_bytes_(Size) void const* ptr)
        : Ptr(ptr) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_ EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(pDesc, Ptr, Size);
        return pDesc;
    }
};

/*
_tlgWrapScalar: Type-checks the value, then wraps it (performing type erasure)
in either _tlgWrapperByVal or _tlgWrapperByRef (depending on type size).
Usage: _tlgWrapScalar<ctype>(value)
*/
template<unsigned Size, bool ByRef = (sizeof(void*) < Size)>
struct _tlgWrapperTypeForScalar { typedef _tlgWrapperByVal<Size> type; };
template<unsigned Size>
struct _tlgWrapperTypeForScalar<Size, true> { typedef _tlgWrapperByRef<Size> type; };
template<class T>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
typename _tlgWrapperTypeForScalar<sizeof(T)>::type _tlg_CALL
_tlgWrapScalar(T const& value)
{ return typename _tlgWrapperTypeForScalar<sizeof(T)>::type(&value); }

/*
_tlgWrapSid: Typed handler for SID.
Usage: _tlgWrapSid<ctype>(pValue)
*/
template<class T>
struct _tlgWrapSid
{
    static const unsigned DataDescCount = 1; // Consumes 1 EVENT_DATA_DESCRIPTOR.

    T const* const Sid;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    explicit _tlgWrapSid(T const* sid)
        : Sid(sid) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_ EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(pDesc, Sid, _tlgGetLengthSid(Sid));
        return pDesc;
    }
};

/*
_tlgWrapSz: Typed handler for char* string.
Usage: _tlgWrapSz<ctype>(pszValue)
*/
template<class CH> struct _tlgWrapSz
{
    static const unsigned DataDescCount = 1; // Consumes 1 EVENT_DATA_DESCRIPTOR.
    _Field_z_ CH const* const Psz;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    explicit _tlgWrapSz(_In_z_ CH const* psz)
        : Psz(psz) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_ EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        _tlgCreate1Sz<CH>(pDesc, Psz);
        return pDesc;
    }
};

/*
_tlgWrapperPtrSize: Type-erased handler for fixed array and
TraceLoggingPackedData.
Usage: _tlgWrapPtrSize<ctype>(pValue, cbValue)
*/
struct _tlgWrapperPtrSize
{
    static const unsigned DataDescCount = 1; // Consumes 1 EVENT_DATA_DESCRIPTOR.

    void const* const Ptr;
    ULONG const Size;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _tlgWrapperPtrSize(_In_reads_bytes_(size) void const* ptr, ULONG size)
        : Ptr(ptr), Size(size) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_ EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(pDesc, Ptr, Size);
        return pDesc;
    }
};
template<class T>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
_tlgWrapperPtrSize _tlg_CALL
_tlgWrapPtrSize(_In_reads_bytes_(size) T const* ptr, ULONG size)
{ return _tlgWrapperPtrSize(ptr, size); }

/*
_tlgWrapperPtrSizeType: Type-erased handler for TraceLoggingPackedDataEx.
Usage: _tlgWrapPtrSizeType<ctype>(pValue, cbValue, reserved)
*/
struct _tlgWrapperPtrSizeType
{
    static const unsigned DataDescCount = 1; // Consumes 1 EVENT_DATA_DESCRIPTOR.

    void const* const Ptr;
    ULONG const Size;
    ULONG const Reserved;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _tlgWrapperPtrSizeType(_In_reads_bytes_(size) void const* ptr, ULONG size, ULONG reserved)
        : Ptr(ptr), Size(size), Reserved(reserved) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_ EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        pDesc->Ptr = (UINT_PTR)Ptr;
        pDesc->Size = Size;
        pDesc->Reserved = Reserved;
        return pDesc;
    }
};
template<class T>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
_tlgWrapperPtrSizeType _tlg_CALL
_tlgWrapPtrSizeType(_In_reads_bytes_(size) T const* ptr, ULONG size, ULONG reserved)
{ return _tlgWrapperPtrSizeType(ptr, size, reserved); }

/*
_tlgWrapperBinary: Type-erased handler for size-prefixed types (counted
binary, counted string).
Usage: _tlgWrapBinary<ctype>(pValue, cbValue)
*/
struct _tlgWrapperBinary
{
    static const unsigned DataDescCount = 2; // Consumes 2 EVENT_DATA_DESCRIPTORs.

    void const* const Ptr;
    UINT16 const Size;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _tlgWrapperBinary(_In_reads_bytes_(size) void const* ptr, UINT16 size)
        : Ptr(ptr), Size(size) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_writes_(2) EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(&pDesc[0], &pDesc[1].Size, 2);
        EventDataDescCreate(&pDesc[1], Ptr, Size);
        return pDesc;
    }
};
template<class T>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
_tlgWrapperBinary _tlg_CALL
_tlgWrapBinary(_In_reads_bytes_(size) T const* ptr, ULONG size)
{ return _tlgWrapperBinary(ptr, static_cast<UINT16>(size)); } // Type checking before erasure.

/*
_tlgWrapBuffer: Typed handler for Buffer types (e.g. ANSI_STRING,
UNICODE_STRING).
Usage: _tlgWrapBuffer<ctype>(pValue)
*/
template<class T>
struct _tlgWrapBuffer
{
    static const unsigned DataDescCount = 2; // Consumes 2 EVENT_DATA_DESCRIPTORs.

    T const* const Ptr;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    explicit _tlgWrapBuffer(_In_ T const* ptr)
        : Ptr(ptr) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_writes_(2) EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(&pDesc[0], &pDesc[1].Size, 2);
#pragma warning(suppress: 26018) // Buffer length bounded by Ptr->Length.
        EventDataDescCreate(&pDesc[1], Ptr->Buffer, static_cast<UINT16>(Ptr->Length));
        return pDesc;
    }
};

/*
_tlgWrapperArray: Type-erased handler for count-prefixed types (arrays).
Usage: _tlgWrapArray<ctype>(pValues, cValues)
*/
template<unsigned Size>
struct _tlgWrapperArray
{
    static const unsigned DataDescCount = 2; // Consumes 2 EVENT_DATA_DESCRIPTORs.

    void const* const Ptr;
    UINT16 const Length;

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _tlgWrapperArray(_In_reads_bytes_(Size * length) void const* ptr, UINT16 length)
        : Ptr(ptr), Length(length) {}

    _IRQL_requires_max_(HIGH_LEVEL)
    TLG_INLINE
    _Ret_ void*
    Fill(_Out_writes_(2) EVENT_DATA_DESCRIPTOR* pDesc) const
    {
        EventDataDescCreate(&pDesc[0], &Length, 2);
        EventDataDescCreate(&pDesc[1], Ptr, Length * Size);
        return pDesc;
    }
};
template<class T>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_PFORCEINLINE
_tlgWrapperArray<sizeof(T)> _tlg_CALL
_tlgWrapArray(T const* ptr, UINT16 count)
{ return _tlgWrapperArray<sizeof(T)>(ptr, count); }

template<unsigned fieldCount>
struct _tlgCheckStructFieldCount
{
    static_assert(fieldCount < 128, "TraceLoggingStruct fieldCount must be less than 128.");
    static unsigned const value = 0;
};

// remove reference from non-reference type (no-op).
template<class T>
struct _tlgRemoveReference
{
    typedef T type;
};

// remove reference from lvalue-reference type.
template<class T>
struct _tlgRemoveReference<T&>
{
    typedef T type;
};

// remove reference from rvalue-reference type.
template<class T>
struct _tlgRemoveReference<T&&>
{
    typedef T type;
};

template<class T> struct _tlgDecay_cv { typedef T type; };
template<class T> struct _tlgDecay_cv<T const> { typedef T type; };
template<class T> struct _tlgDecay_cv<T volatile> { typedef T type; };
template<class T> struct _tlgDecay_cv<T const volatile> { typedef T type; };

// Given non-ref non-array type, remove const/volatile.
template<class T>
struct _tlgDecay_noref
{
    typedef typename _tlgDecay_cv<T>::type type;
};

// Given array type, decay to pointer.
template<class T>
struct _tlgDecay_noref<T[]>
{
    typedef T* type;
};

// Given array type, decay to pointer.
template<class T, SIZE_T N>
struct _tlgDecay_noref<T[N]>
{
    typedef T* type;
};

// Remove reference, remove const/volatile, arrays decay to pointers.
template<class T>
struct _tlgDecay
{
    typedef typename _tlgDecay_noref<typename _tlgRemoveReference<T>::type>::type type;
};

/*
Convert a type into an TlgIn_t value.
*/
template<class T> struct _tlgTypeMapBase
{
    static_assert(sizeof(T) == 0, "The type is not supported by TraceLoggingValue.");
};

template<class T> struct _tlgTypeMap
    : _tlgTypeMapBase<typename _tlgDecay<T>::type> { };

// _tlgInTypeBaseDecl: outType is 0.
#define _tlgInTypeBaseDecl(simple, ctype, inType) \
    template<> struct _tlgTypeMapBase<ctype> \
    { \
        typedef UINT8  _tlgTypeType0; /* No field tags: Don't need to store outtype. */ \
        typedef UINT16 _tlgTypeType1; /* Yes field tags: Need to store outtype = 0. */ \
        static bool const _tlgIsSimple = simple; \
        static _tlgTypeType0 const _tlgType0 = inType | 0x0000; \
        static _tlgTypeType1 const _tlgType1 = inType | 0x8080; \
    }

// _tlgInTypeBaseDeclOut: outType is not 0.
#define _tlgInTypeBaseDeclOut(simple, ctype, inType, outType) \
    template<> struct _tlgTypeMapBase<ctype> \
    { \
        typedef UINT16 _tlgTypeType0; /* Need to store intype+outtype. */ \
        typedef UINT16 _tlgTypeType1; /* Need to store intype+outtype. */ \
        static bool const _tlgIsSimple = simple; \
        static _tlgTypeType0 const _tlgType0 = inType | 0x0080 | (outType << 8); \
        static _tlgTypeType1 const _tlgType1 = inType | 0x8080 | (outType << 8); \
    }

// _tlgWrapAuto normal case (where we want to write sizeof(T) bytes of data):

template<class T>
_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
typename _tlgWrapperTypeForScalar<sizeof(T)>::type _tlg_CALL
_tlgWrapAuto(T const& value)
{
    static_assert(_tlgTypeMap<T>::_tlgIsSimple, "Missing _tlgWrapAuto overload");
    return typename _tlgWrapperTypeForScalar<sizeof(T)>::type(&value);
}

#define _tlgInTypeBaseDecl0(ctype, inType         ) _tlgInTypeBaseDecl(   true, ctype, inType)
#define _tlgInTypeBaseDecl1(ctype, inType, outType) _tlgInTypeBaseDeclOut(true, ctype, inType, outType)
_tlgInTypeBaseDecl1(bool, TlgInUINT8, TlgOutBOOLEAN);
_tlgInTypeBaseDecl1(char, TlgInUINT8, TlgOutSTRING);
_tlgInTypeBaseDecl1(char16_t, TlgInUINT16, TlgOutSTRING);
_tlgInTypeBaseDecl1(__wchar_t, TlgInUINT16, TlgOutSTRING);
_tlgInTypeBaseDecl0(signed   char, TlgInINT8);
_tlgInTypeBaseDecl0(unsigned char, TlgInUINT8);
_tlgInTypeBaseDecl0(signed   short, TlgInINT16);
_tlgInTypeBaseDecl0(unsigned short, TlgInUINT16);
_tlgInTypeBaseDecl0(signed   int, TlgInINT32);
_tlgInTypeBaseDecl0(unsigned int, TlgInUINT32);
_tlgInTypeBaseDecl0(signed   long, sizeof(signed long) == 8 ? TlgInINT64 : TlgInINT32);
_tlgInTypeBaseDecl0(unsigned long, sizeof(unsigned long) == 8 ? TlgInUINT64 : TlgInUINT32);
_tlgInTypeBaseDecl0(signed   long long, TlgInINT64);
_tlgInTypeBaseDecl0(unsigned long long, TlgInUINT64);
_tlgInTypeBaseDecl0(float, TlgInFLOAT);
_tlgInTypeBaseDecl0(double, TlgInDOUBLE);
_tlgInTypeBaseDecl0(GUID, TlgInGUID);
_tlgInTypeBaseDecl0(struct _FILETIME, TlgInFILETIME);
_tlgInTypeBaseDecl0(struct _SYSTEMTIME, TlgInSYSTEMTIME);
_tlgInTypeBaseDecl0(void*, TlgInPOINTER);
_tlgInTypeBaseDecl0(void const*, TlgInPOINTER);
#undef _tlgInTypeBaseDecl0
#undef _tlgInTypeBaseDecl1

// _tlgWrapAuto special cases:

#ifdef SID_DEFINED

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSid<SID> _tlg_CALL
_tlgWrapAuto(_In_ SID* pVal)
{
    return _tlgWrapSid<SID>(pVal);
}
_tlgInTypeBaseDecl(false, SID*, TlgInSID);

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSid<SID> _tlg_CALL
_tlgWrapAuto(_In_ SID const* pVal)
{
    return _tlgWrapSid<SID>(pVal);
}
_tlgInTypeBaseDecl(false, SID const*, TlgInSID);

#endif // SID_DEFINED

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSz<char> _tlg_CALL
_tlgWrapAuto(_In_z_ char* psz)
{
    return _tlgWrapSz<char>(psz);
}
_tlgInTypeBaseDecl(false, char*, TlgInANSISTRING);

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSz<char> _tlg_CALL
_tlgWrapAuto(_In_z_ char const* psz)
{
    return _tlgWrapSz<char>(psz);
}
_tlgInTypeBaseDecl(false, char const*, TlgInANSISTRING);

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSz<char16_t> _tlg_CALL
_tlgWrapAuto(_In_z_ char16_t* psz)
{
    return _tlgWrapSz<char16_t>(psz);
}
_tlgInTypeBaseDecl(false, char16_t*, TlgInUNICODESTRING);

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSz<char16_t> _tlg_CALL
_tlgWrapAuto(_In_z_ char16_t const* psz)
{
    return _tlgWrapSz<char16_t>(psz);
}
_tlgInTypeBaseDecl(false, char16_t const*, TlgInUNICODESTRING);

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSz<wchar_t> _tlg_CALL
_tlgWrapAuto(_In_z_ wchar_t* psz)
{
    return _tlgWrapSz<wchar_t>(psz);
}
_tlgInTypeBaseDecl(false, wchar_t*, TlgInUNICODESTRING);

_IRQL_requires_max_(HIGH_LEVEL)
TLG_INLINE
_tlgWrapSz<wchar_t> _tlg_CALL
_tlgWrapAuto(_In_z_ wchar_t const* psz)
{
    return _tlgWrapSz<wchar_t>(psz);
}
_tlgInTypeBaseDecl(false, wchar_t const*, TlgInUNICODESTRING);

#undef _tlgInTypeBaseDecl
#undef _tlgInTypeBaseDeclOut

#endif // __cplusplus

// ********** NO FUNCTION DEFINITIONS BELOW THIS POINT ***********************
#pragma code_seg(pop)

#pragma endregion

#pragma region Implementation macros

#if defined(_MSC_VER) && !defined(__clang__)
#define _tlgPragmaUtf8Begin __pragma(execution_character_set(push, "UTF-8"))
#define _tlgPragmaUtf8End   __pragma(execution_character_set(pop))
#else
#define _tlgPragmaUtf8Begin
#define _tlgPragmaUtf8End
#endif

#define _tlgValidateProviderId(...) \
    _tlgValidateProviderId_presentN(_tlg_NARGS(__VA_ARGS__), (__VA_ARGS__))
#define _tlgValidateProviderId_presentN(n, args) \
    _tlgValidateProviderId_presentCALL(_tlg_PASTE2(_tlgValidateProviderId_present, n), args)
#define _tlgValidateProviderId_presentCALL(macro, args) \
    macro args
#define _tlgValidateProviderId_present0(...) /* parameter not provided - error case */ \
    _tracelogging_SyntaxError_ProviderIdMustBeSpecified _tracelogging_SyntaxError_ProviderIdMustBeSpecified;
#define _tlgValidateProviderId_present1(providerId) \
    _tlgValidateProviderId_parenthesisN(_tlg_IS_PARENTHESIZED(providerId), providerId)
#define _tlgValidateProviderId_parenthesisN(n, providerID) \
    _tlgValidateProviderId_parenthesisCALL(_tlg_PASTE2(_tlgValidateProviderId_parenthesis, n), providerID)
#define _tlgValidateProviderId_parenthesisCALL(macro, args) \
    macro args
#define _tlgValidateProviderId_parenthesis0 /* parameter not parenthesized - error case */ \
    _tracelogging_SyntaxError_ProviderIdMustBeInParenthesis _tracelogging_SyntaxError_ProviderIdMustBeInParenthesis;

#ifdef __cplusplus
#define _tlgValidateProviderId_parenthesis1(...) \
    static_assert(11 == _tlg_NARGS(__VA_ARGS__), "TRACELOGGING_DEFINE_PROVIDER providerId must be eleven integers, e.g. (1,2,3,4,5,6,7,8,9,10,11)."); \
    static_assert(1 _tlg_FOREACH(_tlgParseProviderId_CheckInt, __VA_ARGS__), "TRACELOGGING_DEFINE_PROVIDER providerId must be eleven integers, e.g. (1,2,3,4,5,6,7,8,9,10,11).");
#define _tlgParseProviderId_CheckInt(n, val) +(val)
#else
#define _tlgValidateProviderId_parenthesis1(...)
#endif

#define _tlgExpandProviderId(a, b, c, d, e, f, g, h, i, j, k) \
    { a, b, c, { d, e, f, g, h, i, j , k } }

#define _tlgProviderStorage_imp(storageVariable, providerName, providerId, annotationFunc, ...) \
    _tlgPragmaUtf8Begin \
    __pragma(pack(push, 1)) \
    _tlgValidateProviderId(providerId) \
    static struct { \
        struct _tlgProviderMetadata_t _tlgProv; \
        char _tlgName[sizeof(providerName)]; \
        _tlg_FOREACH(_tlgProvVar, __VA_ARGS__) \
    } const __declspec(allocate(_tlgSegMetadataProviders)) __declspec(align(1)) storageVariable##_Meta = { \
        { _TlgBlobProvider3, _tlgExpandProviderId providerId, sizeof(storageVariable##_Meta) - 1 - _tlg_PROVIDER_METADATA_PREAMBLE }, \
        (providerName) \
        _tlg_FOREACH(_tlgProvVal, __VA_ARGS__) \
    }; \
    __pragma(pack(pop)) \
    _tlgPragmaUtf8End \
    static struct _tlgProvider_t storageVariable = { \
        0, &storageVariable##_Meta._tlgProv.RemainingSize, \
        0, 0, 0, 0, 0 \
        _tlgAnnotationFunc_imp(annotationFunc, storageVariable) \
    }

#define _tlgProvVar(n, args) _tlgApplyArgsN(_tlgProvVar, n, args)
#define _tlgProvVar_tlgOption(n, traitType, ctype, value) \
    UINT16 _tlgOptionSize##n; UINT8 _tlgTraitType##n; ctype _tlgOptionVal##n;

#define _tlgProvVal(n, args) _tlgApplyArgs(_tlgProvVal, args)
#define _tlgProvVal_tlgOption(traitType, ctype, value) \
    , 3 + sizeof(ctype), traitType, _tlg_FLATTEN value

#define _tlgAnnotationFunc_imp(use_annotationFunc, storageVariable) _tlg_PASTE2(_tlgAnnotationFunc_imp, use_annotationFunc) (storageVariable)
#define _tlgAnnotationFunc_imp0(storageVariable)

/*
Note: Using pragma(comment /include AnnotationFunction) to ensure the
annotation is included into the PDB. Alternative would be taking the
function's address, but that interferes with control flow graph data.
*/
#if defined(_M_HYBRID) || defined(_M_ARM64EC)
#define _tlgAnnotationFunc_imp1(storageVariable) __pragma(comment(linker, "/include:#" _tlg_STRINGIZE(_tlg_PASTE2(_tlgDefineProvider_annotation_, storageVariable)))) //
#elif defined(_M_IX86) || defined(_X86_) // x86 requires leading underscore
#define _tlgAnnotationFunc_imp1(storageVariable) __pragma(comment(linker, "/include:_" _tlg_STRINGIZE(_tlg_PASTE2(_tlgDefineProvider_annotation_, storageVariable))))
#else
#define _tlgAnnotationFunc_imp1(storageVariable) __pragma(comment(linker,  "/include:" _tlg_STRINGIZE(_tlg_PASTE2(_tlgDefineProvider_annotation_, storageVariable))))
#endif

/*
_tlgExpandType(typeParam)  --> typeParam, hasType // typeParam should be a parenthesized value
_tlgExpandType(())         --> (0),       0
_tlgExpandType((typeVal))  --> (typeVal), 1
*/
#define _tlgExpandType_imp0()             (0),       0
#define _tlgExpandType_imp1(typeVal)      (typeVal), 1
#define _tlgExpandType_impB(macro, args)  macro args
#define _tlgExpandType_impA(n, typeParam) _tlgExpandType_impB(_tlg_PASTE2(_tlgExpandType_imp, n), typeParam)
#define _tlgExpandType(typeParam)         _tlgExpandType_impA(_tlg_NARGS typeParam, typeParam)

/*
_tlgNDT: Extracts Name/Description/Tags from varargs of wrapper macro with optional name.
_tlgNDT(value, __VA_ARGS__) --> "fieldName", L"description", tags, hasTags
*/
#define _tlgNDT_imp0(value, ...)               #value,      ,     , 0
#define _tlgNDT_imp1(value, name)              name,        ,     , 0
#define _tlgNDT_imp2(value, name, desc)        name, L"" desc,    , 0
#define _tlgNDT_imp3(value, name, desc, tags)  name, L"" desc, tags, 1
#define _tlgNDT_impB(macro, args)              macro args
#define _tlgNDT_impA(n, args)                  _tlgNDT_impB(_tlg_PASTE2(_tlgNDT_imp, n), args)
#define _tlgNDT(value, ...)                    _tlgNDT_impA(_tlg_NARGS(__VA_ARGS__), (value, __VA_ARGS__))

/*
_tlgDT: Extracts Name/Description/Tags from varargs of wrapper macro with required name.
_tlgDT(name, __VA_ARGS__) --> "fieldName", L"description", tags, hasTags
*/
#define _tlgDT_imp0(name, ...)                 name,        ,     , 0
#define _tlgDT_imp1(name, desc)                name, L"" desc,     , 0
#define _tlgDT_imp2(name, desc, tags)          name, L"" desc, tags, 1
#define _tlgDT_impB(macro, args)               macro args
#define _tlgDT_impA(n, args)                   _tlgDT_impB(_tlg_PASTE2(_tlgDT_imp, n), args)
#define _tlgDT(name, ...)                      _tlgDT_impA(_tlg_NARGS(__VA_ARGS__), (name, __VA_ARGS__))

/*
_tlgApplyArgs and _tlgApplyArgsN: Macro dispatchers.
_tlgApplyArgs( macro,    (handler, ...)) --> macro##handler(...)
_tlgApplyArgsN(macro, n, (handler, ...)) --> macro##handler(n, ...)
*/
#define _tlgApplyArgs_UNWRAP(...)                   __VA_ARGS__
#define _tlgApplyArgs_CALL(macro, handler, args)    macro##handler args
#define _tlgApplyArgs_impB(macro, handler, ...)     _tlgApplyArgs_CALL(macro, handler, (__VA_ARGS__))
#define _tlgApplyArgs_impA(args)                    _tlgApplyArgs_impB args
#define _tlgApplyArgs(macro, args)                  _tlgApplyArgs_impA((macro, _tlgApplyArgs_UNWRAP args))
#define _tlgApplyArgsN_impB(macro, n, handler, ...) _tlgApplyArgs_CALL(macro, handler, (n, __VA_ARGS__))
#define _tlgApplyArgsN_impA(args)                   _tlgApplyArgsN_impB args
#define _tlgApplyArgsN(macro, n, args)              _tlgApplyArgsN_impA((macro, n, _tlgApplyArgs_UNWRAP args))

/*
Dispatch macros: generate a part of the TraceLoggingWrite macro.
*/

#define _tlgInfoVars(n, args) _tlgApplyArgsN(_tlgInfoVars, n, args)
#define _tlgInfoVars_tlgAuto(       n,        value,                                       name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; typename _tlgTypeMap<decltype(value)>::_tlgTypeType##hasTags _tlgTy##n; _tlgInfoVars_##hasTags(n)
#define _tlgInfoVars_tlgScalarVal(  n, ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgScalarRef(  n, ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgSid(        n, ctype, pValue,                                      name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_0       ##hasTags(n)
#define _tlgInfoVars_tlgPsz(        n, ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgBuffer(     n, ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgArray(      n, ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgBinary(     n, ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgCounted(    n, ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n)
#define _tlgInfoVars_tlgFixedArray( n, ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n) UINT16 _tlgValCount##n;
#define _tlgInfoVars_tlgPackedField(n, ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n) _tlg_AssertValidPackedMetadataInTypeCppOnly(inType)
#define _tlgInfoVars_tlgPackedMeta( n,                            inType, outType, hasOut, name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_##hasOut##hasTags(n) _tlg_AssertValidPackedMetadataInTypeCppOnly(inType)
#define _tlgInfoVars_tlgPackedData( n, ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgInfoVars_tlgCustom(     n, ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_0       ##hasTags(n) UINT16 _tlgCustLen##n; UINT8 _tlgCustDat##n[cbSchema];
#define _tlgInfoVars_tlgStruct(     n,                  fieldCount,inType,                 name, desc, tags, hasTags) char _tlgName##n[sizeof(name)]; UINT8 _tlgIn##n; _tlgInfoVars_1       ##hasTags(n)
#define _tlgInfoVars_tlgChannel(    n, eventChannel                                                                 )
#define _tlgInfoVars_tlgLevel(      n, eventLevel                                                                   )
#define _tlgInfoVars_tlgOpcode(     n, eventOpcode                                                                  )
#define _tlgInfoVars_tlgKeyword(    n, eventKeyword                                                                 )
#define _tlgInfoVars_tlgTag(        n, eventTag                                                                     )
#define _tlgInfoVars_tlgDesc(       n, eventDescription                                                             )
#define _tlgInfoVars_tlgCustomAnnot(n, key, value                                                                   )
#define _tlgInfoVars_0(n)
#define _tlgInfoVars_1(n) UINT32 _tlgTags##n;
#define _tlgInfoVars_00(n)
#define _tlgInfoVars_10(n) UINT8 _tlgOut##n;
#define _tlgInfoVars_01(n) UINT8 _tlgOut##n;UINT32 _tlgTags##n;
#define _tlgInfoVars_11(n) UINT8 _tlgOut##n;UINT32 _tlgTags##n;

#define _tlgInfoVals(n, args) _tlgApplyArgsN(_tlgInfoVals, n, args)
#define _tlgInfoVals_tlgAuto(       n,        value,                                       name, desc, tags, hasTags)  , (name), _tlgTypeMap<decltype(value)>::_tlgType##hasTags _tlgInfoVals_##hasTags(tags)
#define _tlgInfoVals_tlgScalarVal(  n, ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgScalarRef(  n, ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgSid(        n, ctype, pValue,                                      name, desc, tags, hasTags)  , (name), TlgInSID  _tlgInfoVals_0       ##hasTags(outType, tags)
#define _tlgInfoVals_tlgPsz(        n, ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgBuffer(     n, ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgArray(      n, ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType|64 _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgBinary(     n, ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgCounted(    n, ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_##hasOut##hasTags(outType, tags)
#define _tlgInfoVals_tlgFixedArray( n, ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)  , (name), inType|32 _tlgInfoVals_##hasOut##hasTags(outType, tags), (cValues)
#define _tlgInfoVals_tlgPackedField(n, ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)  , (name), (inType)  _tlgInfoVals_##hasOut##hasTags((outType&_TlgOutTypeMask), tags)
#define _tlgInfoVals_tlgPackedMeta( n,                            inType, outType, hasOut, name, desc, tags, hasTags)  , (name), (inType)  _tlgInfoVals_##hasOut##hasTags((outType&_TlgOutTypeMask), tags)
#define _tlgInfoVals_tlgPackedData( n, ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgInfoVals_tlgCustom(     n, ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)  , (name), ((protocol)&31)|96 _tlgInfoVals_0##hasTags(0, tags), cbSchema, {_tlg_FLATTEN bSchema}
#define _tlgInfoVals_tlgStruct(     n,                  fieldCount,inType,                 name, desc, tags, hasTags)  , (name), inType    _tlgInfoVals_1       ##hasTags(fieldCount, tags)
#define _tlgInfoVals_tlgChannel(    n, eventChannel                                                                 )
#define _tlgInfoVals_tlgLevel(      n, eventLevel                                                                   )
#define _tlgInfoVals_tlgOpcode(     n, eventOpcode                                                                  )
#define _tlgInfoVals_tlgKeyword(    n, eventKeyword                                                                 )
#define _tlgInfoVals_tlgTag(        n, eventTag                                                                     )
#define _tlgInfoVals_tlgDesc(       n, eventDescription                                                             )
#define _tlgInfoVals_tlgCustomAnnot(n, key, value                                                                   )
#define _tlgInfoVals_0(...)
#define _tlgInfoVals_1(tags) ,_tlgInfoVals_Tags(tags)
#define _tlgInfoVals_00(outType, tags)
#define _tlgInfoVals_10(outType, tags) |128,outType
#define _tlgInfoVals_01(outType, tags) |128,128,_tlgInfoVals_Tags(tags)
#define _tlgInfoVals_11(outType, tags) |128,outType|128,_tlgInfoVals_Tags(tags)
#define _tlgInfoVals_Tags(tags) 0x808080|(((tags)&0x7f)<<24)|(((tags)&0x3f80)<<9)|(((tags)&0x1fc000)>>6)|(((tags)&0xfe00000)>>21)

#ifdef __cplusplus

#define _tlgWrap(n, args) _tlgApplyArgs(_tlgWrap, args)
#define _tlgWrap_tlgAuto(              value,                                       name, desc, tags, hasTags) , _tlgWrapAuto(value)
#define _tlgWrap_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapScalar<ctype>(value)
#define _tlgWrap_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapScalar<ctype>(value)
#define _tlgWrap_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags) , _tlgWrapSid<ctype>(pValue)
#define _tlgWrap_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapSz<ctype>(pszValue)
#define _tlgWrap_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapBuffer<ctype>(pValue)
#define _tlgWrap_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapArray<ctype>(pValues, cValues)
#define _tlgWrap_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapBinary<ctype>(pValue,  cbValue)
#define _tlgWrap_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapBinary<ctype>(pchValue, (cchValue)*sizeof(ctype))
#define _tlgWrap_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapPtrSize<ctype>(pValues, (cValues)*sizeof(ctype))
#define _tlgWrap_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) , _tlgWrapPtrSize<ctype>(pValue,  cbValue)
#define _tlgWrap_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgWrap_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              ) , _tlgWrap_tlgPackedData##hasType(ctype, pValue, cbValue, dataDescType)
#define _tlgWrap_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags) , _tlgWrapBinary<ctype>(pValue, cbValue)
#define _tlgWrap_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgWrap_tlgChannel(    eventChannel                                                                 )
#define _tlgWrap_tlgLevel(      eventLevel                                                                   )
#define _tlgWrap_tlgOpcode(     eventOpcode                                                                  )
#define _tlgWrap_tlgKeyword(    eventKeyword                                                                 )
#define _tlgWrap_tlgTag(        eventTag                                                                     )
#define _tlgWrap_tlgDesc(       eventDescription                                                             )
#define _tlgWrap_tlgCustomAnnot(key, value                                                                   )
#define _tlgWrap_tlgPackedData0(ctype, pValue,   cbValue, dataDescType) _tlgWrapPtrSize<ctype>(pValue,  cbValue)
#define _tlgWrap_tlgPackedData1(ctype, pValue,   cbValue, dataDescType) _tlgWrapPtrSizeType<ctype>(pValue,  cbValue, dataDescType)

#else // __cplusplus

/*
Note: The _tlgTemp variable serves up to 3 purposes.
1. Type checking: by assigning the provided value to a ctype temporary, we get
   a compile error or warning if the provided value is incompatible with
   ctype. This is needed in every case except Psz. Psz is ok because the
   _tlgCreate1Sz_ctype function is strongly-typed.
2. Single evaluation: in the Sid and Buffer cases, we don't want to repeatedly
   evaluate the value pValue expression, so we need a temporary.
3. Storage: in the ScalarVal case, the value may be an rvalue, and we need to
   take a pointer to it, so we store it in _tlgTemp.
Because of this, we store a temporary in all cases except Psz.
We also need a temporary for the Count in the Array case.
We can avoid the Count temporary for the Binary case because count == size, so
we can use &dataDescriptor[1].Size as the pointer to the count.
*/
#define _tlgDataDescCreate(n, args) _tlgApplyArgsN(_tlgDataDescCreate, n, args)
#define _tlgDataDescCreate_tlgScalarVal(  n, ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const _tlgTemp##n = (value); \
    EventDataDescCreate(&_tlgData[_tlgIdx], &_tlgTemp##n, sizeof(ctype)); \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgScalarRef(  n, ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = &(value); \
    EventDataDescCreate(&_tlgData[_tlgIdx], _tlgTemp##n, sizeof(ctype)); \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgSid(        n, ctype, pValue,                                      name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValue); \
    EventDataDescCreate(&_tlgData[_tlgIdx], _tlgTemp##n, _tlgGetLengthSid(_tlgTemp##n)); \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgPsz(        n, ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags) \
    _tlgCreate1Sz_##ctype(&_tlgData[_tlgIdx], (pszValue)); \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgBuffer(     n, ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValue); \
    _tlgCreate2Binary(&_tlgData[_tlgIdx], _tlgTemp##n->Buffer, _tlgTemp##n->Length); \
    _tlgIdx += 2;
#define _tlgDataDescCreate_tlgArray(      n, ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValues); \
    UINT16 const _tlgCount##n = (cValues); \
    EventDataDescCreate(&_tlgData[_tlgIdx+0], &_tlgCount##n, 2); \
    EventDataDescCreate(&_tlgData[_tlgIdx+1], _tlgTemp##n, _tlgCount##n * sizeof(ctype)); \
    _tlgIdx += 2;
#define _tlgDataDescCreate_tlgBinary(     n, ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValue); \
    _tlgCreate2Binary(&_tlgData[_tlgIdx], _tlgTemp##n, (cbValue)); \
    _tlgIdx += 2;
#define _tlgDataDescCreate_tlgCounted(    n, ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pchValue); \
    _tlgCreate2Binary(&_tlgData[_tlgIdx], _tlgTemp##n, sizeof(ctype)*(cchValue)); \
    _tlgIdx += 2;
#define _tlgDataDescCreate_tlgFixedArray( n, ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValues); \
    EventDataDescCreate(&_tlgData[_tlgIdx], _tlgTemp##n, (cValues)*sizeof(ctype)); \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgPackedField(n, ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValue); \
    _tlg_AssertValidPackedMetadataInType(inType); \
    EventDataDescCreate(&_tlgData[_tlgIdx], _tlgTemp##n, (cbValue)); \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgPackedMeta( n,                            inType, outType, hasOut, name, desc, tags, hasTags) \
    _tlg_AssertValidPackedMetadataInType(inType);
#define _tlgDataDescCreate_tlgPackedData( n, ctype, pValue,   cbValue, dataDescType, hasType                              )  \
    ctype const* const _tlgTemp##n = (pValue); \
    _tlgData[_tlgIdx].Ptr = (ULONGLONG)(ULONG_PTR)_tlgTemp##n; \
    _tlgData[_tlgIdx].Size = (cbValue); \
    _tlgData[_tlgIdx].Reserved = dataDescType; \
    _tlgIdx += 1;
#define _tlgDataDescCreate_tlgCustom(     n, ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags) \
    ctype const* const _tlgTemp##n = (pValue); \
    _tlgCreate2Binary(&_tlgData[_tlgIdx], _tlgTemp##n, (cbValue)); \
    _tlgIdx += 2;
#define _tlgDataDescCreate_tlgStruct(     n,                  fieldCount,inType,                 name, desc, tags, hasTags) \
    _tlg_CASSERT((fieldCount)<128, "TraceLoggingStruct fieldCount must be less than 128.");
#define _tlgDataDescCreate_tlgChannel(    n, eventChannel                                                                 )
#define _tlgDataDescCreate_tlgLevel(      n, eventLevel                                                                   )
#define _tlgDataDescCreate_tlgOpcode(     n, eventOpcode                                                                  )
#define _tlgDataDescCreate_tlgKeyword(    n, eventKeyword                                                                 )
#define _tlgDataDescCreate_tlgTag(        n, eventTag                                                                     )
#define _tlgDataDescCreate_tlgDesc(       n, eventDescription                                                             )
#define _tlgDataDescCreate_tlgCustomAnnot(n, key, value                                                                   )

#endif // __cplusplus

#define _tlgDataDescCount(n, args) _tlgApplyArgs(_tlgDataDescCount, args)
#define _tlgDataDescCount_tlgAuto(              value,                                       name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)  +2
#define _tlgDataDescCount_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)  +2
#define _tlgDataDescCount_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)  +2
#define _tlgDataDescCount_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)  +2
#define _tlgDataDescCount_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)  +1
#define _tlgDataDescCount_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDataDescCount_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )  +1
#define _tlgDataDescCount_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)  +2
#define _tlgDataDescCount_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)  +_tlgCheckStructFieldCountCppOnly(fieldCount)
#define _tlgDataDescCount_tlgChannel(    eventChannel                                                                 )
#define _tlgDataDescCount_tlgLevel(      eventLevel                                                                   )
#define _tlgDataDescCount_tlgOpcode(     eventOpcode                                                                  )
#define _tlgDataDescCount_tlgKeyword(    eventKeyword                                                                 )
#define _tlgDataDescCount_tlgTag(        eventTag                                                                     )
#define _tlgDataDescCount_tlgDesc(       eventDescription                                                             )
#define _tlgDataDescCount_tlgCustomAnnot(key, value                                                                   )

#define _tlgChannelVal(n, args) _tlgApplyArgs(_tlgChannelVal, args)
#define _tlgChannelVal_tlgAuto(              value,                                       name, desc, tags, hasTags)
#define _tlgChannelVal_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)
#define _tlgChannelVal_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgChannelVal_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgChannelVal_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)
#define _tlgChannelVal_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgChannelVal_tlgChannel(    eventChannel                                                                 ) &0|(eventChannel)
#define _tlgChannelVal_tlgLevel(      eventLevel                                                                   )
#define _tlgChannelVal_tlgOpcode(     eventOpcode                                                                  )
#define _tlgChannelVal_tlgKeyword(    eventKeyword                                                                 )
#define _tlgChannelVal_tlgTag(        eventTag                                                                     )
#define _tlgChannelVal_tlgDesc(       eventDescription                                                             )
#define _tlgChannelVal_tlgCustomAnnot(key, value                                                                   )

#define _tlgLevelVal(n, args) _tlgApplyArgs(_tlgLevelVal, args)
#define _tlgLevelVal_tlgAuto(              value,                                       name, desc, tags, hasTags)
#define _tlgLevelVal_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)
#define _tlgLevelVal_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgLevelVal_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgLevelVal_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)
#define _tlgLevelVal_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgLevelVal_tlgChannel(    eventChannel                                                                 )
#define _tlgLevelVal_tlgLevel(      eventLevel                                                                   ) &0|(eventLevel)
#define _tlgLevelVal_tlgOpcode(     eventOpcode                                                                  )
#define _tlgLevelVal_tlgKeyword(    eventKeyword                                                                 )
#define _tlgLevelVal_tlgTag(        eventTag                                                                     )
#define _tlgLevelVal_tlgDesc(       eventDescription                                                             )
#define _tlgLevelVal_tlgCustomAnnot(key, value                                                                   )

#define _tlgOpcodeVal(n, args) _tlgApplyArgs(_tlgOpcodeVal, args)
#define _tlgOpcodeVal_tlgAuto(              value,                                       name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgOpcodeVal_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgOpcodeVal_tlgChannel(    eventChannel                                                                 )
#define _tlgOpcodeVal_tlgLevel(      eventLevel                                                                   )
#define _tlgOpcodeVal_tlgOpcode(     eventOpcode                                                                  )  &0|(eventOpcode)
#define _tlgOpcodeVal_tlgKeyword(    eventKeyword                                                                 )
#define _tlgOpcodeVal_tlgTag(        eventTag                                                                     )
#define _tlgOpcodeVal_tlgDesc(       eventDescription                                                             )
#define _tlgOpcodeVal_tlgCustomAnnot(key, value                                                                   )

#define _tlgKeywordVal(n, args) _tlgApplyArgs(_tlgKeywordVal, args)
#define _tlgKeywordVal_tlgAuto(              value,                                       name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgKeywordVal_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgKeywordVal_tlgChannel(    eventChannel                                                                 )
#define _tlgKeywordVal_tlgLevel(      eventLevel                                                                   )
#define _tlgKeywordVal_tlgOpcode(     eventOpcode                                                                  )
#define _tlgKeywordVal_tlgKeyword(    eventKeyword                                                                 ) |(eventKeyword)
#define _tlgKeywordVal_tlgTag(        eventTag                                                                     )
#define _tlgKeywordVal_tlgDesc(       eventDescription                                                             )
#define _tlgKeywordVal_tlgCustomAnnot(key, value                                                                   )

#define _tlgEvtTagVal(n, args) _tlgApplyArgs(_tlgEvtTagVal, args)
#define _tlgEvtTagVal_tlgAuto(              value,                                       name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgEvtTagVal_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgEvtTagVal_tlgChannel(    eventChannel                                                                 )
#define _tlgEvtTagVal_tlgLevel(      eventLevel                                                                   )
#define _tlgEvtTagVal_tlgOpcode(     eventOpcode                                                                  )
#define _tlgEvtTagVal_tlgKeyword(    eventKeyword                                                                 )
#define _tlgEvtTagVal_tlgTag(        eventTag                                                                     ) |(eventTag)
#define _tlgEvtTagVal_tlgDesc(       eventDescription                                                             )
#define _tlgEvtTagVal_tlgCustomAnnot(key, value                                                                   )

#define _tlgAnnotes(n, args) _tlgApplyArgs(_tlgAnnotes, args)
#define _tlgAnnotes_tlgAuto(              value,                                       name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgAnnotes_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags) L"|" _tlg_LSTRINGIZE(name) L"=" desc
#define _tlgAnnotes_tlgChannel(    eventChannel                                                                 )
#define _tlgAnnotes_tlgLevel(      eventLevel                                                                   )
#define _tlgAnnotes_tlgOpcode(     eventOpcode                                                                  )
#define _tlgAnnotes_tlgKeyword(    eventKeyword                                                                 )
#define _tlgAnnotes_tlgTag(        eventTag                                                                     )
#define _tlgAnnotes_tlgDesc(       eventDescription                                                             )
#define _tlgAnnotes_tlgCustomAnnot(key, value                                                                   ) L"|" key L"=" value

#define _tlgDescVal(n, args) _tlgApplyArgs(_tlgDescVal, args)
#define _tlgDescVal_tlgAuto(              value,                                       name, desc, tags, hasTags)
#define _tlgDescVal_tlgScalarVal(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgScalarRef(  ctype, value,              inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgSid(        ctype, pValue,                                      name, desc, tags, hasTags)
#define _tlgDescVal_tlgPsz(        ctype, pszValue,           inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgBuffer(     ctype, pValue,             inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgArray(      ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgBinary(     ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgCounted(    ctype, pchValue, cchValue, inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgFixedArray( ctype, pValues,  cValues,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgPackedField(ctype, pValue,   cbValue,  inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgPackedMeta(                            inType, outType, hasOut, name, desc, tags, hasTags)
#define _tlgDescVal_tlgPackedData( ctype, pValue,   cbValue, dataDescType, hasType                              )
#define _tlgDescVal_tlgCustom(     ctype, pValue,   cbValue, protocol,bSchema,cbSchema,name, desc, tags, hasTags)
#define _tlgDescVal_tlgStruct(                      fieldCount,inType,                 name, desc, tags, hasTags)
#define _tlgDescVal_tlgChannel(    eventChannel                                                                 )
#define _tlgDescVal_tlgLevel(      eventLevel                                                                   )
#define _tlgDescVal_tlgOpcode(     eventOpcode                                                                  )
#define _tlgDescVal_tlgKeyword(    eventKeyword                                                                 )
#define _tlgDescVal_tlgTag(        eventTag                                                                     )
#define _tlgDescVal_tlgDesc(       eventDescription                                                             ) eventDescription
#define _tlgDescVal_tlgCustomAnnot(key, value                                                                   )

#ifdef __cplusplus

template<ULONGLONG n>
struct _traceLoggingKeyword { static ULONGLONG const value = n; };
#define _tlgKeywordConst(n) _traceLoggingKeyword<n>::value

template<
    UINT32 eventTag,
    unsigned length
    = (eventTag & 0x0FE00000) == eventTag ? 1
    : (eventTag & 0x0FFFC000) == eventTag ? 2
    : (eventTag & 0x0FFFFFFF) == eventTag ? 4
    : 0>
    struct _tlgTagEnc
{
    static_assert(
        (eventTag & 0x0FFFFFFF) == eventTag,
        "Invalid Tag value. Tag must be a 28-bit compile-time constant integer.");
};

template<UINT32 eventTag>
struct _tlgTagEnc<eventTag, 1>
{
    typedef UINT8 type;
    static const UINT8 value =
        ((eventTag & 0xfe00000) >> 21);
};

template<UINT32 eventTag>
struct _tlgTagEnc<eventTag, 2>
{
    typedef UINT16 type;
    static const UINT16 value = 0x80
        | ((eventTag & 0xfe00000) >> 21)
        | ((eventTag & 0x01fc000) >> 6);
};

template<UINT32 eventTag>
struct _tlgTagEnc<eventTag, 4>
{
    typedef UINT32 type;
    static const UINT32 value = 0x808080
        | ((eventTag & 0xfe00000) >> 21)
        | ((eventTag & 0x01fc000) >> 6)
        | ((eventTag & 0x0003f80) << 9)
        | ((eventTag & 0x000007f) << 24);
};

#define _tlgEvtTagDecl(eventTag)   typedef _tlgTagEnc<(eventTag)> _tlgTagTy
#define _tlgEvtTagType             _tlgTagTy::type
#define _tlgEvtTagInit             _tlgTagTy::value

#define _tlg_AssertValidPackedMetadataInTypeCppOnly(inType) _tlg_AssertValidPackedMetadataInType(inType);
#define _tlgCheckStructFieldCountCppOnly(fieldCount) _tlgCheckStructFieldCount<fieldCount>::value

#define _tlgPackAndEmitEvent(tlgWriteFunc, tlgWriteArgTypes, tlgWriteArgs, ...) \
	_tlgWriteTemplate<decltype(tlgWriteFunc), tlgWriteFunc, _tlg_FLATTEN tlgWriteArgTypes>::Write( \
		_tlgProv, &_tlgEvent._tlgChannel, \
		_tlg_FLATTEN tlgWriteArgs \
		_tlg_FOREACH(_tlgWrap, __VA_ARGS__))

#else // __cplusplus

#define _tlgKeywordConst(n) n

#define _tlgEvtTagDecl(eventTag)   enum { _tlgTagConst = (eventTag) }
#define _tlgEvtTagType             UINT16
#define _tlgEvtTagInit \
    128 /* Set the high bit of the low byte to indicate 2 bytes of tags. */ \
    |((0xfe00000&(UINT32)_tlgTagConst)>>21) /* Encode 0x0fe00000 bits into UINT16  */ \
    |((0x01fc000&(UINT32)_tlgTagConst)>>6)  /* Encode 0x001fc000 bits into UINT16  */ \
    |((0x0003fff&(UINT32)_tlgTagConst)<<16) /* Trigger warning for 0x00003fff bits */ \
    |(~(0x0FFFFFFF|~(UINT32)_tlgTagConst))  /* Trigger warning for any other bits  */ \

#define _tlg_AssertValidPackedMetadataInTypeCppOnly(inType)
#define _tlgCheckStructFieldCountCppOnly(fieldCount) 0

#define _tlgPackAndEmitEvent(tlgWriteFunc, tlgWriteArgTypes, tlgWriteArgs, ...) \
    EVENT_DATA_DESCRIPTOR _tlgData[_tlg_MetaDataDescCount _tlg_FOREACH(_tlgDataDescCount, __VA_ARGS__)]; \
    UINT32 _tlgIdx = _tlg_MetaDataDescCount; \
    _tlg_FOREACH(_tlgDataDescCreate, __VA_ARGS__) \
    tlgWriteFunc(_tlgProv, &_tlgEvent._tlgChannel, _tlg_FLATTEN tlgWriteArgs, _tlgIdx, _tlgData)

#endif // __cplusplus

#define _tlgWrite_imp(tlgWriteFunc, hProvider, eventName, tlgWriteArgTypes, tlgWriteArgs, ...) \
    do { \
        __pragma(warning(push)) \
        __pragma(warning(disable:4127 4132 6001)) \
        __pragma(warning(error:4047)) \
        __pragma(pack(push, 1)) \
        _tlgPragmaUtf8Begin \
        _tlgEvtTagDecl(0 _tlg_FOREACH(_tlgEvtTagVal, __VA_ARGS__)); \
        enum { _tlgChannelConst = (11u _tlg_FOREACH(_tlgChannelVal, __VA_ARGS__)) }; \
        enum { _tlgLevelConst = (5u _tlg_FOREACH(_tlgLevelVal, __VA_ARGS__)) }; \
        enum { _tlgOpcodeConst = (0u _tlg_FOREACH(_tlgOpcodeVal, __VA_ARGS__)) }; \
        static struct { \
            UCHAR _tlgBlobTyp; UCHAR _tlgChannel; UCHAR _tlgLevel; UCHAR _tlgOpcode; ULONGLONG _tlgKeyword; \
            UINT16 _tlgEvtMetaSize; \
            _tlgEvtTagType _tlgEvtTag; \
            char _tlgName[sizeof(eventName)]; \
            _tlg_FOREACH(_tlgInfoVars, __VA_ARGS__) \
        } __declspec(allocate(_tlgSegMetadataEvents)) __declspec(align(1)) const _tlgEvent = { \
            _TlgBlobEvent4, \
            _tlgChannelConst, \
            _tlgLevelConst, \
            _tlgOpcodeConst, \
            _tlgKeywordConst(0u _tlg_FOREACH(_tlgKeywordVal, __VA_ARGS__)), \
            sizeof(_tlgEvent)-_tlg_EVENT_METADATA_PREAMBLE-1, \
            _tlgEvtTagInit, \
            (eventName) \
            _tlg_FOREACH(_tlgInfoVals, __VA_ARGS__) \
        }; \
        TraceLoggingHProvider const _tlgProv = (hProvider); \
        if ((UCHAR)_tlgLevelConst < _tlgProv->LevelPlus1 && _tlgKeywordOn(_tlgProv, _tlgEvent._tlgKeyword)) { \
            __annotation( \
                L"_TlgWrite:|" _tlg_LSTRINGIZE(__LINE__) L"|" _tlg_LSTRINGIZE(hProvider) L"|" \
                _tlg_LSTRINGIZE(eventName) L"=" _tlg_FOREACH(_tlgDescVal, __VA_ARGS__) \
                _tlg_FOREACH(_tlgAnnotes, __VA_ARGS__) ); \
            _tlgPackAndEmitEvent(tlgWriteFunc, tlgWriteArgTypes, tlgWriteArgs, __VA_ARGS__); \
        } \
        _tlgPragmaUtf8End \
        __pragma(pack(pop)) \
        __pragma(warning(pop)) \
    } while (0) \

#define _tlg_DefineProvider_annotation(hProvider, functionPostfix, requiresWrapper, providerName) \
    _tlgDefineProvider_functionWrapperBegin##requiresWrapper(functionPostfix) \
        __annotation( \
            L"_TlgDefineProvider:|" _tlg_LSTRINGIZE(__LINE__) L"|" _tlg_LSTRINGIZE(hProvider) L"|" \
            providerName \
        ); \
    _tlgDefineProvider_functionWrapperEnd##requiresWrapper

#define _tlgDefineProvider_functionWrapperBegin0(functionPostfix)

// These functions exist only to contain annotations. They are never executed.
// They are extern "C" so that they can be named in pragma(linker, /include).
#define _tlgDefineProvider_functionWrapperBegin1(functionPostfix) \
    _tlg_EXTERN_C \
    void __cdecl \
    _tlg_PASTE2(_tlgDefineProvider_annotation_, functionPostfix)(void) \
    {
#define _tlgDefineProvider_functionWrapperEnd0
#define _tlgDefineProvider_functionWrapperEnd1  }

#pragma endregion

#pragma warning(pop)
#endif // RC_INVOKED
#endif // _traceloggingprovider_
