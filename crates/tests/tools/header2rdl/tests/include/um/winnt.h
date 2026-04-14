/*++ BUILD Version: 0072     Increment this if a change has global effects

Copyright (c) Microsoft Corporation. All rights reserved.

Module Name:

    winnt.h

Abstract:

    This module defines the 32-Bit Windows types and constants that are
    defined by NT, but exposed through the Win32 API.

Revision History:

--*/

#ifndef _WINNT_
#define _WINNT_

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#pragma warning(disable:4820) // padded added
#endif
#pragma warning(disable:4200) // nonstandard extension used : zero-sized array in struct/union
#pragma warning(disable:4201) // named type definition in parentheses
#pragma warning(disable:4214) // bit field types other than int

#ifdef __cplusplus
extern "C" {
#endif

#include <ctype.h>  
#include <winapifamily.h>  

//
// Anywhere that NOINITALL is defined, warning 4845 should be disabled. This warning
// fires whenever __declspec(no_init_all) is found but /d1initall isn't set. This isn't
// helpful since this will be done intentionally (not all components opt-in).
//

#ifndef DECLSPEC_NOINITALL
#if (_MSC_VER >= 1915) && !defined(MIDL_PASS) && !defined(SORTPP_PASS) && !defined(RC_INVOKED)
#define DECLSPEC_NOINITALL __pragma(warning(push)) __pragma(warning(disable:4845)) __declspec(no_init_all) __pragma(warning(pop))
#else
#define DECLSPEC_NOINITALL
#endif
#endif

#define ANYSIZE_ARRAY 1       

//
// For compilers that don't support nameless unions/structs
//
#ifndef DUMMYUNIONNAME
#if defined(NONAMELESSUNION) || !defined(_MSC_EXTENSIONS)
#define DUMMYUNIONNAME   u
#define DUMMYUNIONNAME2  u2
#define DUMMYUNIONNAME3  u3
#define DUMMYUNIONNAME4  u4
#define DUMMYUNIONNAME5  u5
#define DUMMYUNIONNAME6  u6
#define DUMMYUNIONNAME7  u7
#define DUMMYUNIONNAME8  u8
#define DUMMYUNIONNAME9  u9
#else
#define DUMMYUNIONNAME
#define DUMMYUNIONNAME2
#define DUMMYUNIONNAME3
#define DUMMYUNIONNAME4
#define DUMMYUNIONNAME5
#define DUMMYUNIONNAME6
#define DUMMYUNIONNAME7
#define DUMMYUNIONNAME8
#define DUMMYUNIONNAME9
#endif
#endif // DUMMYUNIONNAME

#ifndef DUMMYSTRUCTNAME
#if defined(NONAMELESSUNION) || !defined(_MSC_EXTENSIONS)
#define DUMMYSTRUCTNAME  s
#define DUMMYSTRUCTNAME2 s2
#define DUMMYSTRUCTNAME3 s3
#define DUMMYSTRUCTNAME4 s4
#define DUMMYSTRUCTNAME5 s5
#define DUMMYSTRUCTNAME6 s6
#else
#define DUMMYSTRUCTNAME
#define DUMMYSTRUCTNAME2
#define DUMMYSTRUCTNAME3
#define DUMMYSTRUCTNAME4
#define DUMMYSTRUCTNAME5
#define DUMMYSTRUCTNAME6
#endif
#endif // DUMMYSTRUCTNAME

// end_ntoshvp

#include <specstrings.h>
#include <kernelspecs.h>

#if defined(STRICT_GS_ENABLED)
#pragma strict_gs_check(push, on)
#endif

// begin_ntoshvp

#if defined(_M_MRX000) && !(defined(MIDL_PASS) || defined(RC_INVOKED)) && defined(ENABLE_RESTRICTED)
#define RESTRICTED_POINTER __restrict
#else
#define RESTRICTED_POINTER
#endif

#if defined(_M_MRX000) || defined(_M_ALPHA) || defined(_M_PPC) || defined(_M_IA64) || defined(_M_AMD64) || defined(_M_ARM) || defined(_M_ARM64)
#define ALIGNMENT_MACHINE
#define UNALIGNED __unaligned
#if defined(_WIN64)
#define UNALIGNED64 __unaligned
#else
#define UNALIGNED64
#endif
#else
#undef ALIGNMENT_MACHINE
#define UNALIGNED
#define UNALIGNED64
#endif

// end_ntoshvp


#if defined(_WIN64) || defined(_M_ALPHA)
#define MAX_NATURAL_ALIGNMENT sizeof(ULONGLONG)
#define MEMORY_ALLOCATION_ALIGNMENT 16
#else
#define MAX_NATURAL_ALIGNMENT sizeof(DWORD)
#define MEMORY_ALLOCATION_ALIGNMENT 8
#endif

//
// TYPE_ALIGNMENT will return the alignment requirements of a given type for
// the current platform.
//

#ifdef __cplusplus
#if _MSC_VER >= 1300
#define TYPE_ALIGNMENT( t ) __alignof(t)
#endif
#else
#define TYPE_ALIGNMENT( t ) \
    FIELD_OFFSET( struct { char x; t test; }, test )
#endif

//
// Note: RC_INVOKED is checked in PROBE_ALIGNMENT to maintain compatibility with previous
//       versions of the SDK which did not block inclusion in an .RC file.
//

#if defined(_AMD64_) || defined(_X86_) || defined(_ARM64EC_)
#define PROBE_ALIGNMENT( _s ) TYPE_ALIGNMENT( DWORD )
#elif defined(_IA64_) || defined(_ARM_) || defined(_ARM64_)

//
// TODO: WOWXX - Unblock ARM. Make all alignment checks DWORD for now.
//

#define PROBE_ALIGNMENT( _s ) TYPE_ALIGNMENT( DWORD )
#elif !defined(RC_INVOKED)
#error "No Target Architecture"
#endif

//
// Define PROBE_ALIGNMENT32 to be the same as PROBE_ALIGNMENT on x86, so that
// code hosting x86 under WoW can handle x86's maximum guaranteed alignment.
//

#define PROBE_ALIGNMENT32( _s ) TYPE_ALIGNMENT( DWORD )

// begin_ntoshvp

//
// C_ASSERT() can be used to perform many compile-time assertions:
//            type sizes, field offsets, etc.
//
// An assertion failure results in error C2118: negative subscript.
//

#ifndef SORTPP_PASS
#define C_ASSERT(e) typedef char __C_ASSERT__[(e)?1:-1]
#else
#define C_ASSERT(e) /* nothing */
#endif

#include <basetsd.h>


#ifndef DECLSPEC_IMPORT
#if (defined(_M_IX86) || defined(_M_IA64) || defined(_M_AMD64) || defined(_M_ARM) || defined(_M_ARM64)) && !defined(MIDL_PASS)
#define DECLSPEC_IMPORT __declspec(dllimport)
#else
#define DECLSPEC_IMPORT
#endif
#endif

#ifndef DECLSPEC_NORETURN
#if (_MSC_VER >= 1200) && !defined(MIDL_PASS)
#define DECLSPEC_NORETURN   __declspec(noreturn)
#else
#define DECLSPEC_NORETURN
#endif
#endif

#ifndef DECLSPEC_NOTHROW
#if (_MSC_VER >= 1200) && !defined(MIDL_PASS)
#define DECLSPEC_NOTHROW   __declspec(nothrow)
#else
#define DECLSPEC_NOTHROW
#endif
#endif

#ifndef DECLSPEC_RESTRICT
#if (_MSC_VER >= 1915) && !defined(MIDL_PASS)
#define DECLSPEC_RESTRICT   __declspec(restrict)
#else
#define DECLSPEC_RESTRICT
#endif
#endif

#ifndef DECLSPEC_ALIGN
#if (_MSC_VER >= 1300) && !defined(MIDL_PASS)
#define DECLSPEC_ALIGN(x)   __declspec(align(x))
#else
#define DECLSPEC_ALIGN(x)
#endif
#endif

// end_ntoshvp

#ifndef X86_CACHE_ALIGNMENT_SIZE
#define X86_CACHE_ALIGNMENT_SIZE 64
#endif

#ifndef ARM_CACHE_ALIGNMENT_SIZE
#define ARM_CACHE_ALIGNMENT_SIZE 128
#endif

#ifndef SYSTEM_CACHE_ALIGNMENT_SIZE
#if defined(_AMD64_) || defined(_X86_)
#define SYSTEM_CACHE_ALIGNMENT_SIZE X86_CACHE_ALIGNMENT_SIZE
#elif defined(_ARM64_) || defined(_ARM_)
#define SYSTEM_CACHE_ALIGNMENT_SIZE ARM_CACHE_ALIGNMENT_SIZE
#elif !defined(RC_INVOKED)
#error Must define a target architecture.
#endif
#endif // SYSTEM_CACHE_ALIGNMENT_SIZE

#ifndef DECLSPEC_CACHEALIGN
#define DECLSPEC_CACHEALIGN DECLSPEC_ALIGN(SYSTEM_CACHE_ALIGNMENT_SIZE)
#endif

#ifndef DECLSPEC_UUID
#if (_MSC_VER >= 1100) && defined (__cplusplus)
#define DECLSPEC_UUID(x)    __declspec(uuid(x))
#else
#define DECLSPEC_UUID(x)
#endif
#endif

#ifndef DECLSPEC_NOVTABLE
#if (_MSC_VER >= 1100) && defined(__cplusplus)
#define DECLSPEC_NOVTABLE   __declspec(novtable)
#else
#define DECLSPEC_NOVTABLE
#endif
#endif

#ifndef DECLSPEC_SELECTANY
#if (_MSC_VER >= 1100)
#define DECLSPEC_SELECTANY  __declspec(selectany)
#else
#define DECLSPEC_SELECTANY
#endif
#endif

#ifndef NOP_FUNCTION
#if (_MSC_VER >= 1210)
#define NOP_FUNCTION __noop
#else
#define NOP_FUNCTION (void)0
#endif
#endif

#ifndef DECLSPEC_ADDRSAFE
#if (_MSC_VER >= 1200) && (defined(_M_ALPHA) || defined(_M_AXP64))
#define DECLSPEC_ADDRSAFE  __declspec(address_safe)
#else
#define DECLSPEC_ADDRSAFE
#endif
#endif

#ifndef DECLSPEC_SAFEBUFFERS
#if (_MSC_VER >= 1600)
#define DECLSPEC_SAFEBUFFERS  __declspec(safebuffers)
#else
#define DECLSPEC_SAFEBUFFERS
#endif
#endif

#ifndef DECLSPEC_NOINLINE
#if (_MSC_VER >= 1300)
#define DECLSPEC_NOINLINE  __declspec(noinline)
#else
#define DECLSPEC_NOINLINE
#endif
#endif

#ifndef DECLSPEC_SAFEBUFFERS
#if (_MSC_VER >= 1300)
#define DECLSPEC_SAFEBUFFERS  __declspec(safebuffers)
#else
#define DECLSPEC_SAFEBUFFERS
#endif
#endif


// begin_ntoshvp

//
// When DECLSPEC_NOSANITIZEADDRESS is set, the compiler may not inline
// functions marked with __forceinline. This may result in warning 4714:
//
//     function 'xxx' marked as __forceinline not inlined
//
// Provide a way to disable this warning.
//

#ifndef DECLSPEC_NOSANITIZEADDRESS
#if defined(__SANITIZE_ADDRESS__)
#define DECLSPEC_NOSANITIZEADDRESS      __declspec(no_sanitize_address)
#define ASAN_WARNING_DISABLE_4714_PUSH  __pragma(warning(push)) __pragma(warning(disable:4714))
#define ASAN_WARNING_DISABLE_4714_POP   __pragma(warning(pop))
#else
#define DECLSPEC_NOSANITIZEADDRESS
#define ASAN_WARNING_DISABLE_4714_PUSH
#define ASAN_WARNING_DISABLE_4714_POP
#endif
#endif

#ifndef DECLSPEC_GUARDNOCF
#if (_MSC_FULL_VER >= 170065501) || defined(_D1VERSIONLKG171_)
#define DECLSPEC_GUARDNOCF  __declspec(guard(nocf))
#else
#define DECLSPEC_GUARDNOCF
#endif
#endif

#ifndef DECLSPEC_GUARD_SUPPRESS
#if (_MSC_FULL_VER >= 181040116) || defined(_D1VERSIONLKG171_)
#define DECLSPEC_GUARD_SUPPRESS  __declspec(guard(suppress))
#else
#define DECLSPEC_GUARD_SUPPRESS
#endif
#endif

#ifndef DECLSPEC_CHPE_GUEST
#if _M_HYBRID
#define DECLSPEC_CHPE_GUEST  __declspec(hybrid_guest)
#else
#define DECLSPEC_CHPE_GUEST
#endif
#endif

#ifndef DECLSPEC_CHPE_PATCHABLE
#if !defined(SORTPP_PASS)
#if defined (_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)
#define DECLSPEC_CHPE_PATCHABLE  __declspec(hybrid_patchable)
#else
#define DECLSPEC_CHPE_PATCHABLE  DECLSPEC_NOINLINE
#endif
#else
#define DECLSPEC_CHPE_PATCHABLE
#endif
#endif

#ifndef FORCEINLINE
#if (_MSC_VER >= 1200)
#define FORCEINLINE __forceinline
#else
#define FORCEINLINE __inline
#endif
#endif

//
// CFORCEINLINE: __forceinline required for correctness.  Such definitions are
//               typically required to be visible in the same translation unit
//               (i.e., so that they may still be forceinlined, even in the
//               event of non-LTCG code being encountered).
//

#define CFORCEINLINE FORCEINLINE

//
// STKFORCEINLINE: __forceinline required for correctness due to counting stack
//                 frames for a stack trace being captured.
//

#define STKFORCEINLINE FORCEINLINE

//
// PFORCEINLINE: __forceinline required for performance.
//

#ifndef PFORCEINLINE
#define PFORCEINLINE FORCEINLINE
#endif

// end_ntoshvp

#ifndef DECLSPEC_DEPRECATED
#if (_MSC_VER >= 1300) && !defined(MIDL_PASS)
#define DECLSPEC_DEPRECATED   __declspec(deprecated)
#define DEPRECATE_SUPPORTED
#else
#define DECLSPEC_DEPRECATED
#undef  DEPRECATE_SUPPORTED
#endif
#endif

#ifdef DEPRECATE_DDK_FUNCTIONS
#ifdef _NTDDK_
#define DECLSPEC_DEPRECATED_DDK DECLSPEC_DEPRECATED
#ifdef DEPRECATE_SUPPORTED
#define PRAGMA_DEPRECATED_DDK 1
#endif
#else
#define DECLSPEC_DEPRECATED_DDK
#define PRAGMA_DEPRECATED_DDK 1
#endif
#else
#define DECLSPEC_DEPRECATED_DDK
#define PRAGMA_DEPRECATED_DDK 0
#endif

// begin_ntoshvp

//
// Void
//

typedef void *PVOID;
typedef void * POINTER_64 PVOID64;


#if (_MSC_VER >= 800) || defined(_STDCALL_SUPPORTED)
#define NTAPI __stdcall
#else
#define _cdecl
#define __cdecl
#define NTAPI
#endif

// end_ntminiport end_ntminitape

#if !defined(_M_CEE_PURE)
#define NTAPI_INLINE    NTAPI
#else
#define NTAPI_INLINE
#endif

// begin_ntminiport begin_ntminitape

//
// Define API decoration for direct importing system DLL references.
//

#if !defined(_NTSYSTEM_) && !defined(_NTHALLIB_)
#define NTSYSAPI     DECLSPEC_IMPORT
#define NTSYSCALLAPI DECLSPEC_IMPORT
#else
#define NTSYSAPI
#if defined(_NTDLLBUILD_)
#define NTSYSCALLAPI
#else
#define NTSYSCALLAPI DECLSPEC_ADDRSAFE
#endif
#endif

//
// Basics
//

#ifndef VOID
#define VOID void
typedef char CHAR;
typedef short SHORT;
typedef long LONG;
#if !defined(MIDL_PASS)
typedef int INT;
#endif
#endif

//
// UNICODE (Wide Character) types
//

#ifndef _MAC
typedef wchar_t WCHAR;    // wc,   16-bit UNICODE character
#else
// some Macintosh compilers don't define wchar_t in a convenient location, or define it as a char
typedef unsigned short WCHAR;    // wc,   16-bit UNICODE character
#endif

typedef WCHAR *PWCHAR, *LPWCH, *PWCH;
typedef CONST WCHAR *LPCWCH, *PCWCH;

typedef _Null_terminated_ WCHAR *NWPSTR, *LPWSTR, *PWSTR;
typedef _Null_terminated_ PWSTR *PZPWSTR;
typedef _Null_terminated_ CONST PWSTR *PCZPWSTR;
typedef _Null_terminated_ WCHAR UNALIGNED *LPUWSTR, *PUWSTR;
typedef _Null_terminated_ CONST WCHAR *LPCWSTR, *PCWSTR;
typedef _Null_terminated_ PCWSTR *PZPCWSTR;
typedef _Null_terminated_ CONST PCWSTR *PCZPCWSTR;
typedef _Null_terminated_ CONST WCHAR UNALIGNED *LPCUWSTR, *PCUWSTR;

typedef _NullNull_terminated_ WCHAR *PZZWSTR;
typedef _NullNull_terminated_ CONST WCHAR *PCZZWSTR;
typedef _NullNull_terminated_ WCHAR UNALIGNED *PUZZWSTR;
typedef _NullNull_terminated_ CONST WCHAR UNALIGNED *PCUZZWSTR;

typedef  WCHAR *PNZWCH;
typedef  CONST WCHAR *PCNZWCH;
typedef  WCHAR UNALIGNED *PUNZWCH;
typedef  CONST WCHAR UNALIGNED *PCUNZWCH;

#if _WIN32_WINNT >= 0x0600 || (defined(__cplusplus) && defined(WINDOWS_ENABLE_CPLUSPLUS))

typedef CONST WCHAR *LPCWCHAR, *PCWCHAR;
typedef CONST WCHAR UNALIGNED *LPCUWCHAR, *PCUWCHAR;

//
//  UCS (Universal Character Set) types
//

typedef unsigned long UCSCHAR;

//
//  Even pre-Unicode agreement, UCS values are always in the
//  range U+00000000 to U+7FFFFFFF, so we'll pick an obvious
//  value.

#define UCSCHAR_INVALID_CHARACTER (0xffffffff)

#define MIN_UCSCHAR (0)

//
//  We'll assume here that the ISO-10646 / Unicode agreement
//  not to assign code points after U+0010FFFF holds so that
//  we do not have to have separate "UCSCHAR" and "UNICODECHAR"
//  types.
//

#define MAX_UCSCHAR (0x0010FFFF)

typedef UCSCHAR *PUCSCHAR;
typedef const UCSCHAR *PCUCSCHAR;

typedef UCSCHAR *PUCSSTR;
typedef UCSCHAR UNALIGNED *PUUCSSTR;

typedef const UCSCHAR *PCUCSSTR;
typedef const UCSCHAR UNALIGNED *PCUUCSSTR;

typedef UCSCHAR UNALIGNED *PUUCSCHAR;
typedef const UCSCHAR UNALIGNED *PCUUCSCHAR;

#endif // _WIN32_WINNT >= 0x0600


//
// ANSI (Multi-byte Character) types
//
typedef CHAR *PCHAR, *LPCH, *PCH;
typedef CONST CHAR *LPCCH, *PCCH;

typedef _Null_terminated_ CHAR *NPSTR, *LPSTR, *PSTR;
typedef _Null_terminated_ PSTR *PZPSTR;
typedef _Null_terminated_ CONST PSTR *PCZPSTR;
typedef _Null_terminated_ CONST CHAR *LPCSTR, *PCSTR;
typedef _Null_terminated_ PCSTR *PZPCSTR;
typedef _Null_terminated_ CONST PCSTR *PCZPCSTR;

typedef _NullNull_terminated_ CHAR *PZZSTR;
typedef _NullNull_terminated_ CONST CHAR *PCZZSTR;

typedef  CHAR *PNZCH;
typedef  CONST CHAR *PCNZCH;

//
// Neutral ANSI/UNICODE types and macros
//
#ifdef  UNICODE                     // r_winnt

#ifndef _TCHAR_DEFINED
typedef WCHAR TCHAR, *PTCHAR;
typedef WCHAR TBYTE , *PTBYTE ;
#define _TCHAR_DEFINED
#endif /* !_TCHAR_DEFINED */

typedef LPWCH LPTCH, PTCH;
typedef LPCWCH LPCTCH, PCTCH;
typedef LPWSTR PTSTR, LPTSTR;
typedef LPCWSTR PCTSTR, LPCTSTR;
typedef LPUWSTR PUTSTR, LPUTSTR;
typedef LPCUWSTR PCUTSTR, LPCUTSTR;
typedef LPWSTR LP;
typedef PZZWSTR PZZTSTR;
typedef PCZZWSTR PCZZTSTR;
typedef PUZZWSTR PUZZTSTR;
typedef PCUZZWSTR PCUZZTSTR;
typedef PZPWSTR PZPTSTR;
typedef PNZWCH PNZTCH;
typedef PCNZWCH PCNZTCH;
typedef PUNZWCH PUNZTCH;
typedef PCUNZWCH PCUNZTCH;
#define __TEXT(quote) L##quote      // r_winnt

#else   /* UNICODE */               // r_winnt

#ifndef _TCHAR_DEFINED
typedef char TCHAR, *PTCHAR;
typedef unsigned char TBYTE , *PTBYTE ;
#define _TCHAR_DEFINED
#endif /* !_TCHAR_DEFINED */

typedef LPCH LPTCH, PTCH;
typedef LPCCH LPCTCH, PCTCH;
typedef LPSTR PTSTR, LPTSTR, PUTSTR, LPUTSTR;
typedef LPCSTR PCTSTR, LPCTSTR, PCUTSTR, LPCUTSTR;
typedef PZZSTR PZZTSTR, PUZZTSTR;
typedef PCZZSTR PCZZTSTR, PCUZZTSTR;
typedef PZPSTR PZPTSTR;
typedef PNZCH PNZTCH, PUNZTCH;
typedef PCNZCH PCNZTCH, PCUNZTCH;
#define __TEXT(quote) quote         // r_winnt

#endif /* UNICODE */                // r_winnt
#define TEXT(quote) __TEXT(quote)   // r_winnt


typedef SHORT *PSHORT;  
typedef LONG *PLONG;    

#define ALL_PROCESSOR_GROUPS        0xffff

//
// Structure to represent a system wide processor number. It contains a
// group number and relative processor number within the group.
//

typedef struct _PROCESSOR_NUMBER {
    WORD   Group;
    BYTE  Number;
    BYTE  Reserved;
} PROCESSOR_NUMBER, *PPROCESSOR_NUMBER;

// begin_ntoshvp

//
// Structure to represent a group-specific affinity, such as that of a
// thread.  Specifies the group number and the affinity within that group.
//

typedef struct _GROUP_AFFINITY {
    KAFFINITY Mask;
    WORD   Group;
    WORD   Reserved[3];
} GROUP_AFFINITY, *PGROUP_AFFINITY;

typedef struct _GROUP_AFFINITY32 {
    DWORD Mask;
    WORD   Group;
    WORD   Reserved[3];
} GROUP_AFFINITY32, *PGROUP_AFFINITY32;

typedef struct _GROUP_AFFINITY64 {
    unsigned __int64 Mask;
    WORD   Group;
    WORD   Reserved[3];
} GROUP_AFFINITY64, *PGROUP_AFFINITY64;

// end_ntoshvp

#if defined(_WIN64)

#define MAXIMUM_PROC_PER_GROUP 64

#else

#define MAXIMUM_PROC_PER_GROUP 32

#endif

#define MAXIMUM_PROCESSORS          MAXIMUM_PROC_PER_GROUP

// begin_ntoshvp

//
// Handle to an Object
//

#ifdef STRICT
typedef void *HANDLE;
#if 0 && (_MSC_VER > 1000)
#define DECLARE_HANDLE(name) struct name##__; typedef struct name##__ *name
#else
#define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name
#endif
#else
typedef PVOID HANDLE;
#define DECLARE_HANDLE(name) typedef HANDLE name
#endif
typedef HANDLE *PHANDLE;

// end_ntoshvp

//
// Flag (bit) fields
//

typedef BYTE   FCHAR;
typedef WORD   FSHORT;
typedef DWORD  FLONG;

// begin_ntoshvp

// Component Object Model defines, and macros

#ifndef _HRESULT_DEFINED
#define _HRESULT_DEFINED
#ifdef __midl
typedef LONG HRESULT;
#else
typedef _Return_type_success_(return >= 0) long HRESULT;
#endif // __midl
#endif // !_HRESULT_DEFINED

// end_ntoshvp

#ifdef __cplusplus
    #define EXTERN_C       extern "C"
    #define EXTERN_C_START extern "C" {
    #define EXTERN_C_END   }

    #if _MSC_VER >= 1900
        #define WIN_NOEXCEPT noexcept
    #else
        #define WIN_NOEXCEPT throw()
    #endif

    // 'noexcept' on typedefs is invalid prior to C++17
    #if _MSVC_LANG >= 201703
        #define WIN_NOEXCEPT_PFN noexcept
    #else
        #define WIN_NOEXCEPT_PFN
    #endif
#else
    #define EXTERN_C       extern
    #define EXTERN_C_START
    #define EXTERN_C_END
    #define WIN_NOEXCEPT
    #define WIN_NOEXCEPT_PFN
#endif

#if defined(_WIN32) || defined(_MPPC_)

// Win32 doesn't support __export

#ifdef _68K_
#define STDMETHODCALLTYPE       __cdecl
#else
#define STDMETHODCALLTYPE       __stdcall
#endif
#define STDMETHODVCALLTYPE      __cdecl

#define STDAPICALLTYPE          __stdcall
#define STDAPIVCALLTYPE         __cdecl

#else

#define STDMETHODCALLTYPE       __export __stdcall
#define STDMETHODVCALLTYPE      __export __cdecl

#define STDAPICALLTYPE          __export __stdcall
#define STDAPIVCALLTYPE         __export __cdecl

#endif


#define STDAPI                  EXTERN_C HRESULT STDAPICALLTYPE
#define STDAPI_CHPE_PATCHABLE   EXTERN_C DECLSPEC_CHPE_PATCHABLE HRESULT STDAPICALLTYPE
#define STDAPI_(type)           EXTERN_C type STDAPICALLTYPE
#define STDAPI_CHPE_PATCHABLE_(type)           EXTERN_C DECLSPEC_CHPE_PATCHABLE type STDAPICALLTYPE
#define DEPRECATED_STDAPI(message) EXTERN_C __declspec(deprecated(message)) HRESULT STDAPICALLTYPE
#define DEPRECATED_NO_MESSAGE_STDAPI EXTERN_C __declspec(deprecated) HRESULT STDAPICALLTYPE
#define DEPRECATED_STDAPI_(type, message) EXTERN_C __declspec(deprecated(message)) type STDAPICALLTYPE
#define DEPRECATED_NO_MESSAGE_STDAPI_(type) EXTERN_C __declspec(deprecated) type STDAPICALLTYPE

#define STDMETHODIMP            HRESULT STDMETHODCALLTYPE
#define STDMETHODIMP_(type)     type STDMETHODCALLTYPE

#define STDOVERRIDEMETHODIMP        __override STDMETHODIMP
#define STDOVERRIDEMETHODIMP_(type) __override STDMETHODIMP_(type)

#define IFACEMETHODIMP          __override STDMETHODIMP
#define IFACEMETHODIMP_(type)   __override STDMETHODIMP_(type)

// The 'V' versions allow Variable Argument lists.

#define STDAPIV                 EXTERN_C HRESULT STDAPIVCALLTYPE
#define STDAPIV_(type)          EXTERN_C type STDAPIVCALLTYPE

#define DEPRECATED_STDAPIV(message) EXTERN_C __declspec(deprecated(message)) HRESULT STDAPIVCALLTYPE
#define DEPRECATED_NO_MESSAGE_STDAPIV EXTERN_C __declspec(deprecated) HRESULT STDAPIVCALLTYPE
#define DEPRECATED_STDAPIV_(type, message) EXTERN_C __declspec(deprecated(message)) type STDAPIVCALLTYPE
#define DEPRECATED_NO_MESSAGE_STDAPIV_(type) EXTERN_C __declspec(deprecated) type STDAPIVCALLTYPE

#define STDMETHODIMPV           HRESULT STDMETHODVCALLTYPE
#define STDMETHODIMPV_(type)    type STDMETHODVCALLTYPE

#define STDOVERRIDEMETHODIMPV        __override STDMETHODIMPV
#define STDOVERRIDEMETHODIMPV_(type) __override STDMETHODIMPV_(type)

#define IFACEMETHODIMPV          __override STDMETHODIMPV
#define IFACEMETHODIMPV_(type)   __override STDMETHODIMPV_(type)

typedef char CCHAR;          
typedef DWORD LCID;         
typedef PDWORD PLCID;       
typedef WORD   LANGID;      

#ifndef __COMPARTMENT_ID_DEFINED__
#define __COMPARTMENT_ID_DEFINED__

//
// Compartment identifier
//

typedef enum {
    UNSPECIFIED_COMPARTMENT_ID = 0,
    DEFAULT_COMPARTMENT_ID
} COMPARTMENT_ID, *PCOMPARTMENT_ID;

#endif // __COMPARTMENT_ID_DEFINED__

#define APPLICATION_ERROR_MASK       0x20000000
#define ERROR_SEVERITY_SUCCESS       0x00000000
#define ERROR_SEVERITY_INFORMATIONAL 0x40000000
#define ERROR_SEVERITY_WARNING       0x80000000
#define ERROR_SEVERITY_ERROR         0xC0000000
// begin_ntoshvp

//
// _M_IX86 included so that EM CONTEXT structure compiles with
// x86 programs. *** TBD should this be for all architectures?
//

//
// 16 byte aligned type for 128 bit floats
//

//
// For we define a 128 bit structure and use __declspec(align(16)) pragma to
// align to 128 bits.
//

#if defined(_M_IA64) && !defined(MIDL_PASS)
__declspec(align(16))
#endif
typedef struct _FLOAT128 {
    __int64 LowPart;
    __int64 HighPart;
} FLOAT128;

typedef FLOAT128 *PFLOAT128;


//
// __int64 is only supported by 2.0 and later midl.
// __midl is set by the 2.0 midl and not by 1.0 midl.
//

#define _ULONGLONG_
#if (!defined (_MAC) && (!defined(MIDL_PASS) || defined(__midl)) && (!defined(_M_IX86) || (defined(_INTEGRAL_MAX_BITS) && _INTEGRAL_MAX_BITS >= 64)))
typedef __int64 LONGLONG;
typedef unsigned __int64 ULONGLONG;

#define MAXLONGLONG                         (0x7fffffffffffffff)


#else

#if defined(_MAC) && defined(_MAC_INT_64)
typedef __int64 LONGLONG;
typedef unsigned __int64 ULONGLONG;

#define MAXLONGLONG                      (0x7fffffffffffffff)


#else
typedef double LONGLONG;
typedef double ULONGLONG;
#endif //_MAC and int64

#endif

typedef LONGLONG *PLONGLONG;
typedef ULONGLONG *PULONGLONG;

// Update Sequence Number

typedef LONGLONG USN;

#if defined(MIDL_PASS)
typedef struct _LARGE_INTEGER {
    LONGLONG QuadPart;
} LARGE_INTEGER;
#else // MIDL_PASS
typedef union _LARGE_INTEGER {
    struct {
        DWORD LowPart;
        LONG HighPart;
    } DUMMYSTRUCTNAME;
    struct {
        DWORD LowPart;
        LONG HighPart;
    } u;
    LONGLONG QuadPart;
} LARGE_INTEGER;
#endif //MIDL_PASS

typedef LARGE_INTEGER *PLARGE_INTEGER;

#if defined(MIDL_PASS)
typedef struct _ULARGE_INTEGER {
    ULONGLONG QuadPart;
} ULARGE_INTEGER;
#else // MIDL_PASS
typedef union _ULARGE_INTEGER {
    struct {
        DWORD LowPart;
        DWORD HighPart;
    } DUMMYSTRUCTNAME;
    struct {
        DWORD LowPart;
        DWORD HighPart;
    } u;
    ULONGLONG QuadPart;
} ULARGE_INTEGER;
#endif //MIDL_PASS

typedef ULARGE_INTEGER *PULARGE_INTEGER;

//
// Reference count.
//

typedef LONG_PTR RTL_REFERENCE_COUNT, *PRTL_REFERENCE_COUNT;
typedef LONG RTL_REFERENCE_COUNT32, *PRTL_REFERENCE_COUNT32;

// end_ntminiport end_ntndis end_ntminitape
// end_ntoshvp


//
// Locally Unique Identifier
//

typedef struct _LUID {
    DWORD LowPart;
    LONG HighPart;
} LUID, *PLUID;

#define _DWORDLONG_
typedef ULONGLONG  DWORDLONG;
typedef DWORDLONG *PDWORDLONG;


//
// Define operations to logically shift an int64 by 0..31 bits and to multiply
// 32-bits by 32-bits to form a 64-bit product.
//

#if defined(MIDL_PASS) || defined(RC_INVOKED) || defined(_M_CEE_PURE) \
    || defined(_68K_) || defined(_MPPC_) \
    || defined(_M_IA64) || defined(_M_AMD64) || defined(_M_ARM) || defined(_M_ARM64) \
    || defined(_M_HYBRID_X86_ARM64)

//
// Midl does not understand inline assembler. Therefore, the Rtl functions
// are used for shifts by 0..31 and multiplies of 32-bits times 32-bits to
// form a 64-bit product.
//
//
// IA64 and AMD64 have native 64-bit operations that are just as fast as their
// 32-bit counter parts. Therefore, the int64 data type is used directly to form
// shifts of 0..31 and multiplies of 32-bits times 32-bits to form a 64-bit
// product.
//

#define Int32x32To64(a, b)  (((__int64)((long)(a))) * ((__int64)((long)(b))))
#define UInt32x32To64(a, b) (((unsigned __int64)((unsigned int)(a))) * ((unsigned __int64)((unsigned int)(b))))

#define Int64ShllMod32(a, b) (((unsigned __int64)(a)) << (b))
#define Int64ShraMod32(a, b) (((__int64)(a)) >> (b))
#define Int64ShrlMod32(a, b) (((unsigned __int64)(a)) >> (b))


#elif defined(_M_IX86)

//
// The x86 C compiler understands inline assembler. Therefore, inline functions
// that employ inline assembler are used for shifts of 0..31.  The multiplies
// rely on the compiler recognizing the cast of the multiplicand to int64 to
// generate the optimal code inline.
//

#define Int32x32To64(a, b)  ((__int64)(((__int64)((long)(a))) * ((long)(b))))
#define UInt32x32To64(a, b) ((unsigned __int64)(((unsigned __int64)((unsigned int)(a))) * ((unsigned int)(b))))


ULONGLONG
NTAPI
Int64ShllMod32 (
    _In_ ULONGLONG Value,
    _In_ DWORD ShiftCount
    );

LONGLONG
NTAPI
Int64ShraMod32 (
    _In_ LONGLONG Value,
    _In_ DWORD ShiftCount
    );

ULONGLONG
NTAPI
Int64ShrlMod32 (
    _In_ ULONGLONG Value,
    _In_ DWORD ShiftCount
    );

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4035 4793)               // re-enable below

__inline ULONGLONG
NTAPI
Int64ShllMod32 (
    _In_ ULONGLONG Value,
    _In_ DWORD ShiftCount
    )
{
    __asm    {
        mov     ecx, ShiftCount
        mov     eax, dword ptr [Value]
        mov     edx, dword ptr [Value+4]
        shld    edx, eax, cl
        shl     eax, cl
    }
}

__inline LONGLONG
NTAPI
Int64ShraMod32 (
    _In_ LONGLONG Value,
    _In_ DWORD ShiftCount
    )
{
    __asm {
        mov     ecx, ShiftCount
        mov     eax, dword ptr [Value]
        mov     edx, dword ptr [Value+4]
        shrd    eax, edx, cl
        sar     edx, cl
    }
}

__inline ULONGLONG
NTAPI
Int64ShrlMod32 (
    _In_ ULONGLONG Value,
    _In_ DWORD ShiftCount
    )
{
    __asm    {
        mov     ecx, ShiftCount
        mov     eax, dword ptr [Value]
        mov     edx, dword ptr [Value+4]
        shrd    eax, edx, cl
        shr     edx, cl
    }
}

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4035 4793)
#endif

#else

#error Must define a target architecture.

#endif

//
// Define rotate intrinsics.
//

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_M_AMD64)

#define RotateLeft8 _rotl8
#define RotateLeft16 _rotl16
#define RotateRight8 _rotr8
#define RotateRight16 _rotr16

unsigned char
__cdecl
_rotl8 (
    _In_ unsigned char Value,
    _In_ unsigned char Shift
    );

unsigned short
__cdecl
_rotl16 (
    _In_ unsigned short Value,
    _In_ unsigned char Shift
    );

unsigned char
__cdecl
_rotr8 (
    _In_ unsigned char Value,
    _In_ unsigned char Shift
    );

unsigned short
__cdecl
_rotr16 (
    _In_ unsigned short Value,
    _In_ unsigned char Shift
    );

#pragma intrinsic(_rotl8)
#pragma intrinsic(_rotl16)
#pragma intrinsic(_rotr8)
#pragma intrinsic(_rotr16)

#endif /* _M_AMD64 */

#if _MSC_VER >= 1300

#define RotateLeft32 _rotl
#define RotateLeft64 _rotl64
#define RotateRight32 _rotr
#define RotateRight64 _rotr64

unsigned int
__cdecl
_rotl (
    _In_ unsigned int Value,
    _In_ int Shift
    );

unsigned __int64
__cdecl
_rotl64 (
    _In_ unsigned __int64 Value,
    _In_ int Shift
    );

unsigned int
__cdecl
_rotr (
    _In_ unsigned int Value,
    _In_ int Shift
    );

unsigned __int64
__cdecl
_rotr64 (
    _In_ unsigned __int64 Value,
    _In_ int Shift
    );

#pragma intrinsic(_rotl)
#pragma intrinsic(_rotl64)
#pragma intrinsic(_rotr)
#pragma intrinsic(_rotr64)

#endif  /* _MSC_VER >= 1300 */

#ifdef __cplusplus
}
#endif

#define ANSI_NULL ((CHAR)0)     
#define UNICODE_NULL ((WCHAR)0) 
#define UNICODE_STRING_MAX_BYTES ((WORD  ) 65534) 
#define UNICODE_STRING_MAX_CHARS (32767) 
typedef BYTE  BOOLEAN;           
typedef BOOLEAN *PBOOLEAN;       
//
//  Doubly linked list structure.  Can be used as either a list head, or
//  as link words.
//

typedef struct _LIST_ENTRY {
   struct _LIST_ENTRY *Flink;
   struct _LIST_ENTRY *Blink;
} LIST_ENTRY, *PLIST_ENTRY, *RESTRICTED_POINTER PRLIST_ENTRY;

//
//  Singly linked list structure. Can be used as either a list head, or
//  as link words.
//

typedef struct _SINGLE_LIST_ENTRY {
    struct _SINGLE_LIST_ENTRY *Next;
} SINGLE_LIST_ENTRY, *PSINGLE_LIST_ENTRY;

// end_ntoshvp
// begin_ntoshvp

//
// These are needed for portable debugger support.
//

typedef struct LIST_ENTRY32 {
    DWORD Flink;
    DWORD Blink;
} LIST_ENTRY32;
typedef LIST_ENTRY32 *PLIST_ENTRY32;

typedef struct LIST_ENTRY64 {
    ULONGLONG Flink;
    ULONGLONG Blink;
} LIST_ENTRY64;
typedef LIST_ENTRY64 *PLIST_ENTRY64;


#include <guiddef.h>

#ifndef __OBJECTID_DEFINED
#define __OBJECTID_DEFINED

typedef struct  _OBJECTID {     // size is 20
    GUID Lineage;
    DWORD Uniquifier;
} OBJECTID;
#endif // !_OBJECTID_DEFINED

#define MINCHAR     0x80        
#define MAXCHAR     0x7f        
#define MINSHORT    0x8000      
#define MAXSHORT    0x7fff      
#define MINLONG     0x80000000  
#define MAXLONG     0x7fffffff  
#define MAXBYTE     0xff        
#define MAXWORD     0xffff      
#define MAXDWORD    0xffffffff  
// begin_ntoshvp
//
// Calculate the byte offset of a field in a structure of type type.
//

#ifdef __has_builtin
#if __has_builtin(__builtin_offsetof)
#define FIELD_OFFSET(type, field)    ((LONG)__builtin_offsetof(type, field))
#define UFIELD_OFFSET(type, field)    ((DWORD)__builtin_offsetof(type, field))
#endif
#endif

#ifndef FIELD_OFFSET
#define FIELD_OFFSET(type, field)    ((LONG)(LONG_PTR)&(((type *)0)->field))
#define UFIELD_OFFSET(type, field)    ((DWORD)(LONG_PTR)&(((type *)0)->field))
#endif

//
// Calculate the size of a field in a structure of type type, without
// knowing or stating the type of the field.
//
#define RTL_FIELD_SIZE(type, field) (sizeof(((type *)0)->field))

//
// Calculate the size of a structure of type type up through and
// including a field.
//
#define RTL_SIZEOF_THROUGH_FIELD(type, field) \
    (FIELD_OFFSET(type, field) + RTL_FIELD_SIZE(type, field))

// end_ntoshvp

//
//  RTL_CONTAINS_FIELD usage:
//
//      if (RTL_CONTAINS_FIELD(pBlock, pBlock->cbSize, dwMumble))
//          // safe to use pBlock->dwMumble
//
#define RTL_CONTAINS_FIELD(Struct, Size, Field) \
    ( (((PCHAR)(&(Struct)->Field)) + sizeof((Struct)->Field)) <= (((PCHAR)(Struct))+(Size)) )

//
// Return the number of elements in a statically sized array.
//   DWORD Buffer[100];
//   RTL_NUMBER_OF(Buffer) == 100
// This is also popularly known as: NUMBER_OF, ARRSIZE, _countof, NELEM, etc.
//
#define RTL_NUMBER_OF_V1(A) (sizeof(A)/sizeof((A)[0]))

#if defined(__cplusplus) && \
    !defined(MIDL_PASS) && \
    !defined(RC_INVOKED) && \
    !defined(_PREFAST_) && \
    (_MSC_FULL_VER >= 13009466) && \
    !defined(SORTPP_PASS)
//
// RtlpNumberOf is a function that takes a reference to an array of N Ts.
//
// typedef T array_of_T[N];
// typedef array_of_T &reference_to_array_of_T;
//
// RtlpNumberOf returns a pointer to an array of N chars.
// We could return a reference instead of a pointer but older compilers do not accept that.
//
// typedef char array_of_char[N];
// typedef array_of_char *pointer_to_array_of_char;
//
// sizeof(array_of_char) == N
// sizeof(*pointer_to_array_of_char) == N
//
// pointer_to_array_of_char RtlpNumberOf(reference_to_array_of_T);
//
// We never even call RtlpNumberOf, we just take the size of dereferencing its return type.
// We do not even implement RtlpNumberOf, we just declare it.
//
// Attempts to pass pointers instead of arrays to this macro result in compile time errors.
// That is the point.
//

// end_ntndis end_ntminiport

#pragma region Application Family or OneCore Family Or Game Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

// begin_ntndis begin_ntminiport

extern "C++" // templates cannot be declared to have 'C' linkage
template <typename T, size_t N>
char (*RtlpNumberOf( UNALIGNED T (&)[N] ))[N];

// end_ntndis end_ntminiport

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

// begin_ntndis begin_ntminiport

#define RTL_NUMBER_OF_V2(A) (sizeof(*RtlpNumberOf(A)))

//
// This does not work with:
//
// void Foo()
// {
//    struct { int x; } y[2];
//    RTL_NUMBER_OF_V2(y); // illegal use of anonymous local type in template instantiation
// }
//
// You must instead do:
//
// struct Foo1 { int x; };
//
// void Foo()
// {
//    Foo1 y[2];
//    RTL_NUMBER_OF_V2(y); // ok
// }
//
// OR
//
// void Foo()
// {
//    struct { int x; } y[2];
//    RTL_NUMBER_OF_V1(y); // ok
// }
//
// OR
//
// void Foo()
// {
//    struct { int x; } y[2];
//    _ARRAYSIZE(y); // ok
// }
//

#else
#define RTL_NUMBER_OF_V2(A) RTL_NUMBER_OF_V1(A)
#endif

#ifdef ENABLE_RTL_NUMBER_OF_V2
#define RTL_NUMBER_OF(A) RTL_NUMBER_OF_V2(A)
#else
#define RTL_NUMBER_OF(A) RTL_NUMBER_OF_V1(A)
#endif

//
// ARRAYSIZE is more readable version of RTL_NUMBER_OF_V2, and uses
// it regardless of ENABLE_RTL_NUMBER_OF_V2
//
// _ARRAYSIZE is a version useful for anonymous types
//
#define ARRAYSIZE(A)    RTL_NUMBER_OF_V2(A)
#define _ARRAYSIZE(A)   RTL_NUMBER_OF_V1(A)

//
// An expression that yields the type of a field in a struct.
//
#define RTL_FIELD_TYPE(type, field) (((type*)0)->field)

// RTL_ to avoid collisions in the global namespace.
//
// Given typedef struct _FOO { BYTE Bar[123]; } FOO;
// RTL_NUMBER_OF_FIELD(FOO, Bar) == 123
//
#define RTL_NUMBER_OF_FIELD(type, field) (RTL_NUMBER_OF(RTL_FIELD_TYPE(type, field)))

//
// eg:
// typedef struct FOO {
//   DWORD Integer;
//   PVOID Pointer;
// } FOO;
//
// RTL_PADDING_BETWEEN_FIELDS(FOO, Integer, Pointer) == 0 for Win32, 4 for Win64
//
#define RTL_PADDING_BETWEEN_FIELDS(T, F1, F2) \
    ((FIELD_OFFSET(T, F2) > FIELD_OFFSET(T, F1)) \
        ? (FIELD_OFFSET(T, F2) - FIELD_OFFSET(T, F1) - RTL_FIELD_SIZE(T, F1)) \
        : (FIELD_OFFSET(T, F1) - FIELD_OFFSET(T, F2) - RTL_FIELD_SIZE(T, F2)))

// RTL_ to avoid collisions in the global namespace.
#if defined(__cplusplus)
#define RTL_CONST_CAST(type) const_cast<type>
#else
#define RTL_CONST_CAST(type) (type)
#endif


// like sizeof
// usually this would be * CHAR_BIT, but we don't necessarily have #include <limits.h>
#define RTL_BITS_OF(sizeOfArg) (sizeof(sizeOfArg) * 8)

#define RTL_BITS_OF_FIELD(type, field) (RTL_BITS_OF(RTL_FIELD_TYPE(type, field)))

// begin_ntoshvp

//
// Calculate the address of the base of the structure given its type, and an
// address of a field within the structure.
//

#define CONTAINING_RECORD(address, type, field) ((type *)( \
                                                  (PCHAR)(address) - \
                                                  (ULONG_PTR)(&((type *)0)->field)))

// end_ntoshvp
// end_ntminiport end_ntndis

//
// Exception handler routine definition.
//

#include <excpt.h>

typedef
_IRQL_requires_same_
_Function_class_(EXCEPTION_ROUTINE)
EXCEPTION_DISPOSITION
NTAPI
EXCEPTION_ROUTINE (
    _Inout_ struct _EXCEPTION_RECORD *ExceptionRecord,
    _In_ PVOID EstablisherFrame,
    _Inout_ struct _CONTEXT *ContextRecord,
    _In_ PVOID DispatcherContext
    );

typedef EXCEPTION_ROUTINE *PEXCEPTION_ROUTINE;


//
// Enclave ID definitions
//

#define ENCLAVE_SHORT_ID_LENGTH             16
#define ENCLAVE_LONG_ID_LENGTH              32


#define VER_SERVER_NT                       0x80000000
#define VER_WORKSTATION_NT                  0x40000000
#define VER_SUITE_SMALLBUSINESS             0x00000001
#define VER_SUITE_ENTERPRISE                0x00000002
#define VER_SUITE_BACKOFFICE                0x00000004
#define VER_SUITE_COMMUNICATIONS            0x00000008
#define VER_SUITE_TERMINAL                  0x00000010
#define VER_SUITE_SMALLBUSINESS_RESTRICTED  0x00000020
#define VER_SUITE_EMBEDDEDNT                0x00000040
#define VER_SUITE_DATACENTER                0x00000080
#define VER_SUITE_SINGLEUSERTS              0x00000100
#define VER_SUITE_PERSONAL                  0x00000200
#define VER_SUITE_BLADE                     0x00000400
#define VER_SUITE_EMBEDDED_RESTRICTED       0x00000800
#define VER_SUITE_SECURITY_APPLIANCE        0x00001000
#define VER_SUITE_STORAGE_SERVER            0x00002000
#define VER_SUITE_COMPUTE_SERVER            0x00004000
#define VER_SUITE_WH_SERVER                 0x00008000
#define VER_SUITE_MULTIUSERTS               0x00020000


//
// Product types
// This list grows with each OS release.
//
// There is no ordering of values to ensure callers
// do an equality test i.e. greater-than and less-than
// comparisons are not useful.
//
// NOTE: Values in this list should never be deleted.
//       When a product-type 'X' gets dropped from a
//       OS release onwards, the value of 'X' continues
//       to be used in the mapping table of GetProductInfo.
//

#define PRODUCT_UNDEFINED                           0x00000000

#define PRODUCT_ULTIMATE                            0x00000001
#define PRODUCT_HOME_BASIC                          0x00000002
#define PRODUCT_HOME_PREMIUM                        0x00000003
#define PRODUCT_ENTERPRISE                          0x00000004
#define PRODUCT_HOME_BASIC_N                        0x00000005
#define PRODUCT_BUSINESS                            0x00000006
#define PRODUCT_STANDARD_SERVER                     0x00000007
#define PRODUCT_DATACENTER_SERVER                   0x00000008
#define PRODUCT_SMALLBUSINESS_SERVER                0x00000009
#define PRODUCT_ENTERPRISE_SERVER                   0x0000000A
#define PRODUCT_STARTER                             0x0000000B
#define PRODUCT_DATACENTER_SERVER_CORE              0x0000000C
#define PRODUCT_STANDARD_SERVER_CORE                0x0000000D
#define PRODUCT_ENTERPRISE_SERVER_CORE              0x0000000E
#define PRODUCT_ENTERPRISE_SERVER_IA64              0x0000000F
#define PRODUCT_BUSINESS_N                          0x00000010
#define PRODUCT_WEB_SERVER                          0x00000011
#define PRODUCT_CLUSTER_SERVER                      0x00000012
#define PRODUCT_HOME_SERVER                         0x00000013
#define PRODUCT_STORAGE_EXPRESS_SERVER              0x00000014
#define PRODUCT_STORAGE_STANDARD_SERVER             0x00000015
#define PRODUCT_STORAGE_WORKGROUP_SERVER            0x00000016
#define PRODUCT_STORAGE_ENTERPRISE_SERVER           0x00000017
#define PRODUCT_SERVER_FOR_SMALLBUSINESS            0x00000018
#define PRODUCT_SMALLBUSINESS_SERVER_PREMIUM        0x00000019
#define PRODUCT_HOME_PREMIUM_N                      0x0000001A
#define PRODUCT_ENTERPRISE_N                        0x0000001B
#define PRODUCT_ULTIMATE_N                          0x0000001C
#define PRODUCT_WEB_SERVER_CORE                     0x0000001D
#define PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT    0x0000001E
#define PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY      0x0000001F
#define PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING     0x00000020
#define PRODUCT_SERVER_FOUNDATION                   0x00000021
#define PRODUCT_HOME_PREMIUM_SERVER                 0x00000022
#define PRODUCT_SERVER_FOR_SMALLBUSINESS_V          0x00000023
#define PRODUCT_STANDARD_SERVER_V                   0x00000024
#define PRODUCT_DATACENTER_SERVER_V                 0x00000025
#define PRODUCT_ENTERPRISE_SERVER_V                 0x00000026
#define PRODUCT_DATACENTER_SERVER_CORE_V            0x00000027
#define PRODUCT_STANDARD_SERVER_CORE_V              0x00000028
#define PRODUCT_ENTERPRISE_SERVER_CORE_V            0x00000029
#define PRODUCT_HYPERV                              0x0000002A
#define PRODUCT_STORAGE_EXPRESS_SERVER_CORE         0x0000002B
#define PRODUCT_STORAGE_STANDARD_SERVER_CORE        0x0000002C
#define PRODUCT_STORAGE_WORKGROUP_SERVER_CORE       0x0000002D
#define PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE      0x0000002E
#define PRODUCT_STARTER_N                           0x0000002F
#define PRODUCT_PROFESSIONAL                        0x00000030
#define PRODUCT_PROFESSIONAL_N                      0x00000031
#define PRODUCT_SB_SOLUTION_SERVER                  0x00000032
#define PRODUCT_SERVER_FOR_SB_SOLUTIONS             0x00000033
#define PRODUCT_STANDARD_SERVER_SOLUTIONS           0x00000034
#define PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE      0x00000035
#define PRODUCT_SB_SOLUTION_SERVER_EM               0x00000036
#define PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM          0x00000037
#define PRODUCT_SOLUTION_EMBEDDEDSERVER             0x00000038
#define PRODUCT_SOLUTION_EMBEDDEDSERVER_CORE        0x00000039
#define PRODUCT_PROFESSIONAL_EMBEDDED               0x0000003A
#define PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT       0x0000003B
#define PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL       0x0000003C
#define PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC    0x0000003D
#define PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC    0x0000003E
#define PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE   0x0000003F
#define PRODUCT_CLUSTER_SERVER_V                    0x00000040
#define PRODUCT_EMBEDDED                            0x00000041
#define PRODUCT_STARTER_E                           0x00000042
#define PRODUCT_HOME_BASIC_E                        0x00000043
#define PRODUCT_HOME_PREMIUM_E                      0x00000044
#define PRODUCT_PROFESSIONAL_E                      0x00000045
#define PRODUCT_ENTERPRISE_E                        0x00000046
#define PRODUCT_ULTIMATE_E                          0x00000047
#define PRODUCT_ENTERPRISE_EVALUATION               0x00000048
#define PRODUCT_MULTIPOINT_STANDARD_SERVER          0x0000004C
#define PRODUCT_MULTIPOINT_PREMIUM_SERVER           0x0000004D
#define PRODUCT_STANDARD_EVALUATION_SERVER          0x0000004F
#define PRODUCT_DATACENTER_EVALUATION_SERVER        0x00000050
#define PRODUCT_ENTERPRISE_N_EVALUATION             0x00000054
#define PRODUCT_EMBEDDED_AUTOMOTIVE                 0x00000055
#define PRODUCT_EMBEDDED_INDUSTRY_A                 0x00000056
#define PRODUCT_THINPC                              0x00000057
#define PRODUCT_EMBEDDED_A                          0x00000058
#define PRODUCT_EMBEDDED_INDUSTRY                   0x00000059
#define PRODUCT_EMBEDDED_E                          0x0000005A
#define PRODUCT_EMBEDDED_INDUSTRY_E                 0x0000005B
#define PRODUCT_EMBEDDED_INDUSTRY_A_E               0x0000005C
#define PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER 0x0000005F
#define PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER  0x00000060
#define PRODUCT_CORE_ARM                            0x00000061
#define PRODUCT_CORE_N                              0x00000062
#define PRODUCT_CORE_COUNTRYSPECIFIC                0x00000063
#define PRODUCT_CORE_SINGLELANGUAGE                 0x00000064
#define PRODUCT_CORE                                0x00000065
#define PRODUCT_PROFESSIONAL_WMC                    0x00000067
#define PRODUCT_EMBEDDED_INDUSTRY_EVAL              0x00000069
#define PRODUCT_EMBEDDED_INDUSTRY_E_EVAL            0x0000006A
#define PRODUCT_EMBEDDED_EVAL                       0x0000006B
#define PRODUCT_EMBEDDED_E_EVAL                     0x0000006C
#define PRODUCT_NANO_SERVER                         0x0000006D
#define PRODUCT_CLOUD_STORAGE_SERVER                0x0000006E
#define PRODUCT_CORE_CONNECTED                      0x0000006F
#define PRODUCT_PROFESSIONAL_STUDENT                0x00000070
#define PRODUCT_CORE_CONNECTED_N                    0x00000071
#define PRODUCT_PROFESSIONAL_STUDENT_N              0x00000072
#define PRODUCT_CORE_CONNECTED_SINGLELANGUAGE       0x00000073
#define PRODUCT_CORE_CONNECTED_COUNTRYSPECIFIC      0x00000074
#define PRODUCT_CONNECTED_CAR                       0x00000075
#define PRODUCT_INDUSTRY_HANDHELD                   0x00000076
#define PRODUCT_PPI_PRO                             0x00000077
#define PRODUCT_ARM64_SERVER                        0x00000078
#define PRODUCT_EDUCATION                           0x00000079
#define PRODUCT_EDUCATION_N                         0x0000007A
#define PRODUCT_IOTUAP                              0x0000007B
#define PRODUCT_CLOUD_HOST_INFRASTRUCTURE_SERVER    0x0000007C
#define PRODUCT_ENTERPRISE_S                        0x0000007D
#define PRODUCT_ENTERPRISE_S_N                      0x0000007E
#define PRODUCT_PROFESSIONAL_S                      0x0000007F
#define PRODUCT_PROFESSIONAL_S_N                    0x00000080
#define PRODUCT_ENTERPRISE_S_EVALUATION             0x00000081
#define PRODUCT_ENTERPRISE_S_N_EVALUATION           0x00000082
#define PRODUCT_HOLOGRAPHIC                         0x00000087
#define PRODUCT_HOLOGRAPHIC_BUSINESS                0x00000088
#define PRODUCT_PRO_SINGLE_LANGUAGE                 0x0000008A
#define PRODUCT_PRO_CHINA                           0x0000008B
#define PRODUCT_ENTERPRISE_SUBSCRIPTION             0x0000008C
#define PRODUCT_ENTERPRISE_SUBSCRIPTION_N           0x0000008D
#define PRODUCT_DATACENTER_NANO_SERVER              0x0000008F
#define PRODUCT_STANDARD_NANO_SERVER                0x00000090
#define PRODUCT_DATACENTER_A_SERVER_CORE            0x00000091
#define PRODUCT_STANDARD_A_SERVER_CORE              0x00000092
#define PRODUCT_DATACENTER_WS_SERVER_CORE           0x00000093
#define PRODUCT_STANDARD_WS_SERVER_CORE             0x00000094
#define PRODUCT_UTILITY_VM                          0x00000095
#define PRODUCT_DATACENTER_EVALUATION_SERVER_CORE   0x0000009F
#define PRODUCT_STANDARD_EVALUATION_SERVER_CORE     0x000000A0
#define PRODUCT_PRO_WORKSTATION                     0x000000A1
#define PRODUCT_PRO_WORKSTATION_N                   0x000000A2
#define PRODUCT_PRO_FOR_EDUCATION                   0x000000A4
#define PRODUCT_PRO_FOR_EDUCATION_N                 0x000000A5
#define PRODUCT_AZURE_SERVER_CORE                   0x000000A8
#define PRODUCT_AZURE_NANO_SERVER                   0x000000A9
#define PRODUCT_ENTERPRISEG                         0x000000AB
#define PRODUCT_ENTERPRISEGN                        0x000000AC
#define PRODUCT_SERVERRDSH                          0x000000AF
#define PRODUCT_CLOUD                               0x000000B2
#define PRODUCT_CLOUDN                              0x000000B3
#define PRODUCT_HUBOS                               0x000000B4
#define PRODUCT_ONECOREUPDATEOS                     0x000000B6
#define PRODUCT_CLOUDE                              0x000000B7
#define PRODUCT_IOTOS                               0x000000B9
#define PRODUCT_CLOUDEN                             0x000000BA
#define PRODUCT_IOTEDGEOS                           0x000000BB
#define PRODUCT_IOTENTERPRISE                       0x000000BC
#define PRODUCT_LITE                                0x000000BD
#define PRODUCT_IOTENTERPRISES                      0x000000BF
#define PRODUCT_XBOX_SYSTEMOS                       0x000000C0
#define PRODUCT_XBOX_GAMEOS                         0x000000C2
#define PRODUCT_XBOX_ERAOS                          0x000000C3
#define PRODUCT_XBOX_DURANGOHOSTOS                  0x000000C4
#define PRODUCT_XBOX_SCARLETTHOSTOS                 0x000000C5
#define PRODUCT_XBOX_KEYSTONE                       0x000000C6
#define PRODUCT_AZURE_SERVER_CLOUDHOST              0x000000C7
#define PRODUCT_AZURE_SERVER_CLOUDMOS               0x000000C8
#define PRODUCT_CLOUDEDITIONN                       0x000000CA
#define PRODUCT_CLOUDEDITION                        0x000000CB
#define PRODUCT_VALIDATION                          0x000000CC
#define PRODUCT_IOTENTERPRISESK                     0x000000CD
#define PRODUCT_IOTENTERPRISEK                      0x000000CE
#define PRODUCT_IOTENTERPRISESEVAL                  0x000000CF
#define PRODUCT_AZURE_SERVER_AGENTBRIDGE            0x000000D0
#define PRODUCT_AZURE_SERVER_NANOHOST               0x000000D1
#define PRODUCT_WNC                                 0x000000D2
#define PRODUCT_AZURESTACKHCI_SERVER_CORE           0x00000196
#define PRODUCT_DATACENTER_SERVER_AZURE_EDITION     0x00000197
#define PRODUCT_DATACENTER_SERVER_CORE_AZURE_EDITION 0x00000198
#define PRODUCT_DATACENTER_WS_SERVER_CORE_AZURE_EDITION 0x00000199

#define PRODUCT_UNLICENSED                          0xABCDABCD

#include <sdkddkver.h>

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  DEPRECATED: The Language ID  concept is deprecated, please use
//  Locale Names instead, eg: "en" instead of a LANGID like 0x09.
//  See the documentation for GetLocaleInfoEx.
//
//  Note that the named locale APIs (eg GetLocaleInfoEx) are preferred.
//
//  WARNING: Not all locales/languages have unique Language IDs
//
//  The following two combinations of primary language ID and
//  sublanguage ID have special semantics:
//
//    Primary Language ID   Sublanguage ID      Result
//    -------------------   ---------------     ------------------------
//    LANG_NEUTRAL          SUBLANG_NEUTRAL     Language neutral
//    LANG_NEUTRAL          SUBLANG_DEFAULT     User default language
//    LANG_NEUTRAL          SUBLANG_SYS_DEFAULT System default language
//    LANG_INVARIANT        SUBLANG_NEUTRAL     Invariant locale
//
//  This concept is deprecated.  It is strongly recommended that
//  applications test for locale names instead of Language IDs / LCIDs.
//
//  Primary language IDs.
//
//  WARNING: This pattern is broken and not followed for all languages.
//           Serbian, Bosnian & Croatian are a few examples.
//
//  WARNING: There are > 6000 human languages.  The PRIMARYLANGID construct
//           cannot support all languages your application may encounter.
//           Please use Language Names, such as "en".
//
//  WARNING: Some languages may have more than one PRIMARYLANGID.  Please
//           use Locale Names, such as "en-FJ".
//
//  WARNING: Some languages do not have assigned LANGIDs.  Please use
//           Locale Names, such as "tlh-Piqd".
//
//  It is recommended that applications test for locale names or actual LCIDs.
//
//  Note that the LANG, SUBLANG construction is not always consistent.
//  The named locale APIs (eg GetLocaleInfoEx) are recommended.
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define LANG_NEUTRAL                     0x00
#define LANG_INVARIANT                   0x7f

#define LANG_AFRIKAANS                   0x36
#define LANG_ALBANIAN                    0x1c
#define LANG_ALSATIAN                    0x84
#define LANG_AMHARIC                     0x5e
#define LANG_ARABIC                      0x01
#define LANG_ARMENIAN                    0x2b
#define LANG_ASSAMESE                    0x4d
#define LANG_AZERI                       0x2c   // for Azerbaijani, LANG_AZERBAIJANI is preferred
#define LANG_AZERBAIJANI                 0x2c
#define LANG_BANGLA                      0x45
#define LANG_BASHKIR                     0x6d
#define LANG_BASQUE                      0x2d
#define LANG_BELARUSIAN                  0x23
#define LANG_BENGALI                     0x45   // Some prefer to use LANG_BANGLA
#define LANG_BRETON                      0x7e
#define LANG_BOSNIAN                     0x1a   // Use with SUBLANG_BOSNIAN_* Sublanguage IDs
#define LANG_BOSNIAN_NEUTRAL           0x781a   // Use with the ConvertDefaultLocale function
#define LANG_BULGARIAN                   0x02
#define LANG_CATALAN                     0x03
#define LANG_CENTRAL_KURDISH             0x92
#define LANG_CHEROKEE                    0x5c
#define LANG_CHINESE                     0x04   // Use with SUBLANG_CHINESE_* Sublanguage IDs
#define LANG_CHINESE_SIMPLIFIED          0x04   // Use with the ConvertDefaultLocale function
#define LANG_CHINESE_TRADITIONAL       0x7c04   // Use with the ConvertDefaultLocale function
#define LANG_CORSICAN                    0x83
#define LANG_CROATIAN                    0x1a
#define LANG_CZECH                       0x05
#define LANG_DANISH                      0x06
#define LANG_DARI                        0x8c
#define LANG_DIVEHI                      0x65
#define LANG_DUTCH                       0x13
#define LANG_ENGLISH                     0x09
#define LANG_ESTONIAN                    0x25
#define LANG_FAEROESE                    0x38
#define LANG_FARSI                       0x29   // Deprecated: use LANG_PERSIAN instead
#define LANG_FILIPINO                    0x64
#define LANG_FINNISH                     0x0b
#define LANG_FRENCH                      0x0c
#define LANG_FRISIAN                     0x62
#define LANG_FULAH                       0x67
#define LANG_GALICIAN                    0x56
#define LANG_GEORGIAN                    0x37
#define LANG_GERMAN                      0x07
#define LANG_GREEK                       0x08
#define LANG_GREENLANDIC                 0x6f
#define LANG_GUJARATI                    0x47
#define LANG_HAUSA                       0x68
#define LANG_HAWAIIAN                    0x75
#define LANG_HEBREW                      0x0d
#define LANG_HINDI                       0x39
#define LANG_HUNGARIAN                   0x0e
#define LANG_ICELANDIC                   0x0f
#define LANG_IGBO                        0x70
#define LANG_INDONESIAN                  0x21
#define LANG_INUKTITUT                   0x5d
#define LANG_IRISH                       0x3c   // Use with the SUBLANG_IRISH_IRELAND Sublanguage ID
#define LANG_ITALIAN                     0x10
#define LANG_JAPANESE                    0x11
#define LANG_KANNADA                     0x4b
#define LANG_KASHMIRI                    0x60
#define LANG_KAZAK                       0x3f
#define LANG_KHMER                       0x53
#define LANG_KICHE                       0x86
#define LANG_KINYARWANDA                 0x87
#define LANG_KONKANI                     0x57
#define LANG_KOREAN                      0x12
#define LANG_KYRGYZ                      0x40
#define LANG_LAO                         0x54
#define LANG_LATVIAN                     0x26
#define LANG_LITHUANIAN                  0x27
#define LANG_LOWER_SORBIAN               0x2e
#define LANG_LUXEMBOURGISH               0x6e
#define LANG_MACEDONIAN                  0x2f   // the Former Yugoslav Republic of Macedonia
#define LANG_MALAY                       0x3e
#define LANG_MALAYALAM                   0x4c
#define LANG_MALTESE                     0x3a
#define LANG_MANIPURI                    0x58
#define LANG_MAORI                       0x81
#define LANG_MAPUDUNGUN                  0x7a
#define LANG_MARATHI                     0x4e
#define LANG_MOHAWK                      0x7c
#define LANG_MONGOLIAN                   0x50
#define LANG_NEPALI                      0x61
#define LANG_NORWEGIAN                   0x14
#define LANG_OCCITAN                     0x82
#define LANG_ODIA                        0x48
#define LANG_ORIYA                       0x48   // Deprecated: use LANG_ODIA, instead.
#define LANG_PASHTO                      0x63
#define LANG_PERSIAN                     0x29
#define LANG_POLISH                      0x15
#define LANG_PORTUGUESE                  0x16
#define LANG_PULAR                       0x67   // Deprecated: use LANG_FULAH instead
#define LANG_PUNJABI                     0x46
#define LANG_QUECHUA                     0x6b
#define LANG_ROMANIAN                    0x18
#define LANG_ROMANSH                     0x17
#define LANG_RUSSIAN                     0x19
#define LANG_SAKHA                       0x85
#define LANG_SAMI                        0x3b
#define LANG_SANSKRIT                    0x4f
#define LANG_SCOTTISH_GAELIC             0x91
#define LANG_SERBIAN                     0x1a   // Use with the SUBLANG_SERBIAN_* Sublanguage IDs
#define LANG_SERBIAN_NEUTRAL           0x7c1a   // Use with the ConvertDefaultLocale function
#define LANG_SINDHI                      0x59
#define LANG_SINHALESE                   0x5b
#define LANG_SLOVAK                      0x1b
#define LANG_SLOVENIAN                   0x24
#define LANG_SOTHO                       0x6c
#define LANG_SPANISH                     0x0a
#define LANG_SWAHILI                     0x41
#define LANG_SWEDISH                     0x1d
#define LANG_SYRIAC                      0x5a
#define LANG_TAJIK                       0x28
#define LANG_TAMAZIGHT                   0x5f
#define LANG_TAMIL                       0x49
#define LANG_TATAR                       0x44
#define LANG_TELUGU                      0x4a
#define LANG_THAI                        0x1e
#define LANG_TIBETAN                     0x51
#define LANG_TIGRIGNA                    0x73
#define LANG_TIGRINYA                    0x73   // Preferred spelling in locale
#define LANG_TSWANA                      0x32
#define LANG_TURKISH                     0x1f
#define LANG_TURKMEN                     0x42
#define LANG_UIGHUR                      0x80
#define LANG_UKRAINIAN                   0x22
#define LANG_UPPER_SORBIAN               0x2e
#define LANG_URDU                        0x20
#define LANG_UZBEK                       0x43
#define LANG_VALENCIAN                   0x03
#define LANG_VIETNAMESE                  0x2a
#define LANG_WELSH                       0x52
#define LANG_WOLOF                       0x88
#define LANG_XHOSA                       0x34
#define LANG_YAKUT                       0x85   // Deprecated: use LANG_SAKHA,instead
#define LANG_YI                          0x78
#define LANG_YORUBA                      0x6a
#define LANG_ZULU                        0x35

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  DEPRECATED: The Sublanguage ID concept is deprecated, please use
//  Locale Names instead, eg: "en-US" instead of an LCID like 0x0409.
//  See the documentation for GetLocaleInfoEx.
//
//  The name immediately following SUBLANG_ dictates which primary
//  language ID that sublanguage ID can be combined with to form a
//  valid language ID.
//
//  Note that the LANG, SUBLANG construction is not always consistent.
//  The named locale APIs (eg GetLocaleInfoEx) are recommended.
//
//  WARNING: The pattern is broken and not followed for all languages.
//           Serbian, Bosnian & Croatian are a few examples.
//
//  WARNING: The "SUBLANG" depends on the primary language and is inconsistent.
//           SUBLANG_ENGLISH_US is 0x1 and SUBLANG_SPANISH_US is 0x15, so
//           it is impossible to determine region merely by inspecting the
//           SUBLANG.  Please use Locale Names such as "en-US" instead.
//
//  WARNING: Numerous SUBLANGS are assigned the same value, so 0x01 could be
//           US, French, or many other variations.  Please use Locale Names
//           such as "en-US" instead.  If that is not possible, consider
//           testing the entire LCID, eg: 0x0409.
//
//  WARNING: There are > 6000 human languages.  The PRIMARYLANGID construct
//           cannot support all languages your application may encounter.
//           Please use Language Names, such as "en".
//
//  WARNING: There are > 200 country-regions.  The SUBLANGID construct cannot
//           represent all valid dialects of languages such as English.
//           Please use Locale Names, such as "en-US".
//
//  WARNING: Some languages may have more than one PRIMARYLANGID.  Please
//           use Locale Names, such as "en-FJ".
//
//  WARNING: Some languages do not have assigned LANGIDs.  Please use
//           Locale Names, such as "tlh-Piqd".
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define SUBLANG_NEUTRAL                             0x00    // language neutral
#define SUBLANG_DEFAULT                             0x01    // user default
#define SUBLANG_SYS_DEFAULT                         0x02    // system default
#define SUBLANG_CUSTOM_DEFAULT                      0x03    // default custom language/locale
#define SUBLANG_CUSTOM_UNSPECIFIED                  0x04    // custom language/locale
#define SUBLANG_UI_CUSTOM_DEFAULT                   0x05    // Default custom MUI language/locale


#define SUBLANG_AFRIKAANS_SOUTH_AFRICA              0x01    // Afrikaans (South Africa) 0x0436 af-ZA
#define SUBLANG_ALBANIAN_ALBANIA                    0x01    // Albanian (Albania) 0x041c sq-AL
#define SUBLANG_ALSATIAN_FRANCE                     0x01    // Alsatian (France) 0x0484
#define SUBLANG_AMHARIC_ETHIOPIA                    0x01    // Amharic (Ethiopia) 0x045e
#define SUBLANG_ARABIC_SAUDI_ARABIA                 0x01    // Arabic (Saudi Arabia)
#define SUBLANG_ARABIC_IRAQ                         0x02    // Arabic (Iraq)
#define SUBLANG_ARABIC_EGYPT                        0x03    // Arabic (Egypt)
#define SUBLANG_ARABIC_LIBYA                        0x04    // Arabic (Libya)
#define SUBLANG_ARABIC_ALGERIA                      0x05    // Arabic (Algeria)
#define SUBLANG_ARABIC_MOROCCO                      0x06    // Arabic (Morocco)
#define SUBLANG_ARABIC_TUNISIA                      0x07    // Arabic (Tunisia)
#define SUBLANG_ARABIC_OMAN                         0x08    // Arabic (Oman)
#define SUBLANG_ARABIC_YEMEN                        0x09    // Arabic (Yemen)
#define SUBLANG_ARABIC_SYRIA                        0x0a    // Arabic (Syria)
#define SUBLANG_ARABIC_JORDAN                       0x0b    // Arabic (Jordan)
#define SUBLANG_ARABIC_LEBANON                      0x0c    // Arabic (Lebanon)
#define SUBLANG_ARABIC_KUWAIT                       0x0d    // Arabic (Kuwait)
#define SUBLANG_ARABIC_UAE                          0x0e    // Arabic (U.A.E)
#define SUBLANG_ARABIC_BAHRAIN                      0x0f    // Arabic (Bahrain)
#define SUBLANG_ARABIC_QATAR                        0x10    // Arabic (Qatar)
#define SUBLANG_ARMENIAN_ARMENIA                    0x01    // Armenian (Armenia) 0x042b hy-AM
#define SUBLANG_ASSAMESE_INDIA                      0x01    // Assamese (India) 0x044d
#define SUBLANG_AZERI_LATIN                         0x01    // Azeri (Latin) - for Azerbaijani, SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN preferred
#define SUBLANG_AZERI_CYRILLIC                      0x02    // Azeri (Cyrillic) - for Azerbaijani, SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC preferred
#define SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN        0x01    // Azerbaijani (Azerbaijan, Latin)
#define SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC     0x02    // Azerbaijani (Azerbaijan, Cyrillic)
#define SUBLANG_BANGLA_INDIA                        0x01    // Bangla (India)
#define SUBLANG_BANGLA_BANGLADESH                   0x02    // Bangla (Bangladesh)
#define SUBLANG_BASHKIR_RUSSIA                      0x01    // Bashkir (Russia) 0x046d ba-RU
#define SUBLANG_BASQUE_BASQUE                       0x01    // Basque (Basque) 0x042d eu-ES
#define SUBLANG_BELARUSIAN_BELARUS                  0x01    // Belarusian (Belarus) 0x0423 be-BY
#define SUBLANG_BENGALI_INDIA                       0x01    // Bengali (India) - Note some prefer SUBLANG_BANGLA_INDIA
#define SUBLANG_BENGALI_BANGLADESH                  0x02    // Bengali (Bangladesh) - Note some prefer SUBLANG_BANGLA_BANGLADESH
#define SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_LATIN    0x05    // Bosnian (Bosnia and Herzegovina - Latin) 0x141a bs-BA-Latn
#define SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC 0x08    // Bosnian (Bosnia and Herzegovina - Cyrillic) 0x201a bs-BA-Cyrl
#define SUBLANG_BRETON_FRANCE                       0x01    // Breton (France) 0x047e
#define SUBLANG_BULGARIAN_BULGARIA                  0x01    // Bulgarian (Bulgaria) 0x0402
#define SUBLANG_CATALAN_CATALAN                     0x01    // Catalan (Catalan) 0x0403
#define SUBLANG_CENTRAL_KURDISH_IRAQ                0x01    // Central Kurdish (Iraq) 0x0492 ku-Arab-IQ
#define SUBLANG_CHEROKEE_CHEROKEE                   0x01    // Cherokee (Cherokee) 0x045c chr-Cher-US
#define SUBLANG_CHINESE_TRADITIONAL                 0x01    // Chinese (Taiwan) 0x0404 zh-TW
#define SUBLANG_CHINESE_SIMPLIFIED                  0x02    // Chinese (PR China) 0x0804 zh-CN
#define SUBLANG_CHINESE_HONGKONG                    0x03    // Chinese (Hong Kong S.A.R., P.R.C.) 0x0c04 zh-HK
#define SUBLANG_CHINESE_SINGAPORE                   0x04    // Chinese (Singapore) 0x1004 zh-SG
#define SUBLANG_CHINESE_MACAU                       0x05    // Chinese (Macau S.A.R.) 0x1404 zh-MO
#define SUBLANG_CORSICAN_FRANCE                     0x01    // Corsican (France) 0x0483
#define SUBLANG_CZECH_CZECH_REPUBLIC                0x01    // Czech (Czech Republic) 0x0405
#define SUBLANG_CROATIAN_CROATIA                    0x01    // Croatian (Croatia)
#define SUBLANG_CROATIAN_BOSNIA_HERZEGOVINA_LATIN   0x04    // Croatian (Bosnia and Herzegovina - Latin) 0x101a hr-BA
#define SUBLANG_DANISH_DENMARK                      0x01    // Danish (Denmark) 0x0406
#define SUBLANG_DARI_AFGHANISTAN                    0x01    // Dari (Afghanistan)
#define SUBLANG_DIVEHI_MALDIVES                     0x01    // Divehi (Maldives) 0x0465 div-MV
#define SUBLANG_DUTCH                               0x01    // Dutch
#define SUBLANG_DUTCH_BELGIAN                       0x02    // Dutch (Belgian)
#define SUBLANG_ENGLISH_US                          0x01    // English (USA)
#define SUBLANG_ENGLISH_UK                          0x02    // English (UK)
#define SUBLANG_ENGLISH_AUS                         0x03    // English (Australian)
#define SUBLANG_ENGLISH_CAN                         0x04    // English (Canadian)
#define SUBLANG_ENGLISH_NZ                          0x05    // English (New Zealand)
#define SUBLANG_ENGLISH_EIRE                        0x06    // English (Irish)
#define SUBLANG_ENGLISH_SOUTH_AFRICA                0x07    // English (South Africa)
#define SUBLANG_ENGLISH_JAMAICA                     0x08    // English (Jamaica)
#define SUBLANG_ENGLISH_CARIBBEAN                   0x09    // English (Caribbean)
#define SUBLANG_ENGLISH_BELIZE                      0x0a    // English (Belize)
#define SUBLANG_ENGLISH_TRINIDAD                    0x0b    // English (Trinidad)
#define SUBLANG_ENGLISH_ZIMBABWE                    0x0c    // English (Zimbabwe)
#define SUBLANG_ENGLISH_PHILIPPINES                 0x0d    // English (Philippines)
#define SUBLANG_ENGLISH_INDIA                       0x10    // English (India)
#define SUBLANG_ENGLISH_MALAYSIA                    0x11    // English (Malaysia)
#define SUBLANG_ENGLISH_SINGAPORE                   0x12    // English (Singapore)
#define SUBLANG_ESTONIAN_ESTONIA                    0x01    // Estonian (Estonia) 0x0425 et-EE
#define SUBLANG_FAEROESE_FAROE_ISLANDS              0x01    // Faroese (Faroe Islands) 0x0438 fo-FO
#define SUBLANG_FILIPINO_PHILIPPINES                0x01    // Filipino (Philippines) 0x0464 fil-PH
#define SUBLANG_FINNISH_FINLAND                     0x01    // Finnish (Finland) 0x040b
#define SUBLANG_FRENCH                              0x01    // French
#define SUBLANG_FRENCH_BELGIAN                      0x02    // French (Belgian)
#define SUBLANG_FRENCH_CANADIAN                     0x03    // French (Canadian)
#define SUBLANG_FRENCH_SWISS                        0x04    // French (Swiss)
#define SUBLANG_FRENCH_LUXEMBOURG                   0x05    // French (Luxembourg)
#define SUBLANG_FRENCH_MONACO                       0x06    // French (Monaco)
#define SUBLANG_FRISIAN_NETHERLANDS                 0x01    // Frisian (Netherlands) 0x0462 fy-NL
#define SUBLANG_FULAH_SENEGAL                       0x02    // Fulah (Senegal) 0x0867 ff-Latn-SN
#define SUBLANG_GALICIAN_GALICIAN                   0x01    // Galician (Galician) 0x0456 gl-ES
#define SUBLANG_GEORGIAN_GEORGIA                    0x01    // Georgian (Georgia) 0x0437 ka-GE
#define SUBLANG_GERMAN                              0x01    // German
#define SUBLANG_GERMAN_SWISS                        0x02    // German (Swiss)
#define SUBLANG_GERMAN_AUSTRIAN                     0x03    // German (Austrian)
#define SUBLANG_GERMAN_LUXEMBOURG                   0x04    // German (Luxembourg)
#define SUBLANG_GERMAN_LIECHTENSTEIN                0x05    // German (Liechtenstein)
#define SUBLANG_GREEK_GREECE                        0x01    // Greek (Greece)
#define SUBLANG_GREENLANDIC_GREENLAND               0x01    // Greenlandic (Greenland) 0x046f kl-GL
#define SUBLANG_GUJARATI_INDIA                      0x01    // Gujarati (India (Gujarati Script)) 0x0447 gu-IN
#define SUBLANG_HAUSA_NIGERIA_LATIN                 0x01    // Hausa (Latin, Nigeria) 0x0468 ha-NG-Latn
#define SUBLANG_HAWAIIAN_US                         0x01    // Hawiian (US) 0x0475 haw-US
#define SUBLANG_HEBREW_ISRAEL                       0x01    // Hebrew (Israel) 0x040d
#define SUBLANG_HINDI_INDIA                         0x01    // Hindi (India) 0x0439 hi-IN
#define SUBLANG_HUNGARIAN_HUNGARY                   0x01    // Hungarian (Hungary) 0x040e
#define SUBLANG_ICELANDIC_ICELAND                   0x01    // Icelandic (Iceland) 0x040f
#define SUBLANG_IGBO_NIGERIA                        0x01    // Igbo (Nigeria) 0x0470 ig-NG
#define SUBLANG_INDONESIAN_INDONESIA                0x01    // Indonesian (Indonesia) 0x0421 id-ID
#define SUBLANG_INUKTITUT_CANADA                    0x01    // Inuktitut (Syllabics) (Canada) 0x045d iu-CA-Cans
#define SUBLANG_INUKTITUT_CANADA_LATIN              0x02    // Inuktitut (Canada - Latin)
#define SUBLANG_IRISH_IRELAND                       0x02    // Irish (Ireland)
#define SUBLANG_ITALIAN                             0x01    // Italian
#define SUBLANG_ITALIAN_SWISS                       0x02    // Italian (Swiss)
#define SUBLANG_JAPANESE_JAPAN                      0x01    // Japanese (Japan) 0x0411
#define SUBLANG_KANNADA_INDIA                       0x01    // Kannada (India (Kannada Script)) 0x044b kn-IN
#define SUBLANG_KASHMIRI_SASIA                      0x02    // Kashmiri (South Asia)
#define SUBLANG_KASHMIRI_INDIA                      0x02    // For app compatibility only
#define SUBLANG_KAZAK_KAZAKHSTAN                    0x01    // Kazakh (Kazakhstan) 0x043f kk-KZ
#define SUBLANG_KHMER_CAMBODIA                      0x01    // Khmer (Cambodia) 0x0453 kh-KH
#define SUBLANG_KICHE_GUATEMALA                     0x01    // K'iche (Guatemala)
#define SUBLANG_KINYARWANDA_RWANDA                  0x01    // Kinyarwanda (Rwanda) 0x0487 rw-RW
#define SUBLANG_KONKANI_INDIA                       0x01    // Konkani (India) 0x0457 kok-IN
#define SUBLANG_KOREAN                              0x01    // Korean (Extended Wansung)
#define SUBLANG_KYRGYZ_KYRGYZSTAN                   0x01    // Kyrgyz (Kyrgyzstan) 0x0440 ky-KG
#define SUBLANG_LAO_LAO                             0x01    // Lao (Lao PDR) 0x0454 lo-LA
#define SUBLANG_LATVIAN_LATVIA                      0x01    // Latvian (Latvia) 0x0426 lv-LV
#define SUBLANG_LITHUANIAN                          0x01    // Lithuanian
#define SUBLANG_LOWER_SORBIAN_GERMANY               0x02    // Lower Sorbian (Germany) 0x082e wee-DE
#define SUBLANG_LUXEMBOURGISH_LUXEMBOURG            0x01    // Luxembourgish (Luxembourg) 0x046e lb-LU
#define SUBLANG_MACEDONIAN_MACEDONIA                0x01    // Macedonian (Macedonia (FYROM)) 0x042f mk-MK
#define SUBLANG_MALAY_MALAYSIA                      0x01    // Malay (Malaysia)
#define SUBLANG_MALAY_BRUNEI_DARUSSALAM             0x02    // Malay (Brunei Darussalam)
#define SUBLANG_MALAYALAM_INDIA                     0x01    // Malayalam (India (Malayalam Script) ) 0x044c ml-IN
#define SUBLANG_MALTESE_MALTA                       0x01    // Maltese (Malta) 0x043a mt-MT
#define SUBLANG_MAORI_NEW_ZEALAND                   0x01    // Maori (New Zealand) 0x0481 mi-NZ
#define SUBLANG_MAPUDUNGUN_CHILE                    0x01    // Mapudungun (Chile) 0x047a arn-CL
#define SUBLANG_MARATHI_INDIA                       0x01    // Marathi (India) 0x044e mr-IN
#define SUBLANG_MOHAWK_MOHAWK                       0x01    // Mohawk (Mohawk) 0x047c moh-CA
#define SUBLANG_MONGOLIAN_CYRILLIC_MONGOLIA         0x01    // Mongolian (Cyrillic, Mongolia)
#define SUBLANG_MONGOLIAN_PRC                       0x02    // Mongolian (PRC)
#define SUBLANG_NEPALI_INDIA                        0x02    // Nepali (India)
#define SUBLANG_NEPALI_NEPAL                        0x01    // Nepali (Nepal) 0x0461 ne-NP
#define SUBLANG_NORWEGIAN_BOKMAL                    0x01    // Norwegian (Bokmal)
#define SUBLANG_NORWEGIAN_NYNORSK                   0x02    // Norwegian (Nynorsk)
#define SUBLANG_OCCITAN_FRANCE                      0x01    // Occitan (France) 0x0482 oc-FR
#define SUBLANG_ODIA_INDIA                          0x01    // Odia (India (Odia Script)) 0x0448 or-IN
#define SUBLANG_ORIYA_INDIA                         0x01    // Deprecated: use SUBLANG_ODIA_INDIA instead
#define SUBLANG_PASHTO_AFGHANISTAN                  0x01    // Pashto (Afghanistan)
#define SUBLANG_PERSIAN_IRAN                        0x01    // Persian (Iran) 0x0429 fa-IR
#define SUBLANG_POLISH_POLAND                       0x01    // Polish (Poland) 0x0415
#define SUBLANG_PORTUGUESE                          0x02    // Portuguese
#define SUBLANG_PORTUGUESE_BRAZILIAN                0x01    // Portuguese (Brazil)
#define SUBLANG_PULAR_SENEGAL                       0x02    // Deprecated: Use SUBLANG_FULAH_SENEGAL instead
#define SUBLANG_PUNJABI_INDIA                       0x01    // Punjabi (India (Gurmukhi Script)) 0x0446 pa-IN
#define SUBLANG_PUNJABI_PAKISTAN                    0x02    // Punjabi (Pakistan (Arabic Script)) 0x0846 pa-Arab-PK
#define SUBLANG_QUECHUA_BOLIVIA                     0x01    // Quechua (Bolivia)
#define SUBLANG_QUECHUA_ECUADOR                     0x02    // Quechua (Ecuador)
#define SUBLANG_QUECHUA_PERU                        0x03    // Quechua (Peru)
#define SUBLANG_ROMANIAN_ROMANIA                    0x01    // Romanian (Romania) 0x0418
#define SUBLANG_ROMANSH_SWITZERLAND                 0x01    // Romansh (Switzerland) 0x0417 rm-CH
#define SUBLANG_RUSSIAN_RUSSIA                      0x01    // Russian (Russia) 0x0419
#define SUBLANG_SAKHA_RUSSIA                        0x01    // Sakha (Russia) 0x0485 sah-RU
#define SUBLANG_SAMI_NORTHERN_NORWAY                0x01    // Northern Sami (Norway)
#define SUBLANG_SAMI_NORTHERN_SWEDEN                0x02    // Northern Sami (Sweden)
#define SUBLANG_SAMI_NORTHERN_FINLAND               0x03    // Northern Sami (Finland)
#define SUBLANG_SAMI_LULE_NORWAY                    0x04    // Lule Sami (Norway)
#define SUBLANG_SAMI_LULE_SWEDEN                    0x05    // Lule Sami (Sweden)
#define SUBLANG_SAMI_SOUTHERN_NORWAY                0x06    // Southern Sami (Norway)
#define SUBLANG_SAMI_SOUTHERN_SWEDEN                0x07    // Southern Sami (Sweden)
#define SUBLANG_SAMI_SKOLT_FINLAND                  0x08    // Skolt Sami (Finland)
#define SUBLANG_SAMI_INARI_FINLAND                  0x09    // Inari Sami (Finland)
#define SUBLANG_SANSKRIT_INDIA                      0x01    // Sanskrit (India) 0x044f sa-IN
#define SUBLANG_SCOTTISH_GAELIC                     0x01    // Scottish Gaelic (United Kingdom) 0x0491 gd-GB
#define SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_LATIN    0x06    // Serbian (Bosnia and Herzegovina - Latin)
#define SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC 0x07    // Serbian (Bosnia and Herzegovina - Cyrillic)
#define SUBLANG_SERBIAN_MONTENEGRO_LATIN            0x0b    // Serbian (Montenegro - Latn)
#define SUBLANG_SERBIAN_MONTENEGRO_CYRILLIC         0x0c    // Serbian (Montenegro - Cyrillic)
#define SUBLANG_SERBIAN_SERBIA_LATIN                0x09    // Serbian (Serbia - Latin)
#define SUBLANG_SERBIAN_SERBIA_CYRILLIC             0x0a    // Serbian (Serbia - Cyrillic)
#define SUBLANG_SERBIAN_CROATIA                     0x01    // Croatian (Croatia) 0x041a hr-HR
#define SUBLANG_SERBIAN_LATIN                       0x02    // Serbian (Latin)
#define SUBLANG_SERBIAN_CYRILLIC                    0x03    // Serbian (Cyrillic)
#define SUBLANG_SINDHI_INDIA                        0x01    // Sindhi (India) reserved 0x0459
#define SUBLANG_SINDHI_PAKISTAN                     0x02    // Sindhi (Pakistan) 0x0859 sd-Arab-PK
#define SUBLANG_SINDHI_AFGHANISTAN                  0x02    // For app compatibility only
#define SUBLANG_SINHALESE_SRI_LANKA                 0x01    // Sinhalese (Sri Lanka)
#define SUBLANG_SOTHO_NORTHERN_SOUTH_AFRICA         0x01    // Northern Sotho (South Africa)
#define SUBLANG_SLOVAK_SLOVAKIA                     0x01    // Slovak (Slovakia) 0x041b sk-SK
#define SUBLANG_SLOVENIAN_SLOVENIA                  0x01    // Slovenian (Slovenia) 0x0424 sl-SI
#define SUBLANG_SPANISH                             0x01    // Spanish (Castilian)
#define SUBLANG_SPANISH_MEXICAN                     0x02    // Spanish (Mexico)
#define SUBLANG_SPANISH_MODERN                      0x03    // Spanish (Modern)
#define SUBLANG_SPANISH_GUATEMALA                   0x04    // Spanish (Guatemala)
#define SUBLANG_SPANISH_COSTA_RICA                  0x05    // Spanish (Costa Rica)
#define SUBLANG_SPANISH_PANAMA                      0x06    // Spanish (Panama)
#define SUBLANG_SPANISH_DOMINICAN_REPUBLIC          0x07    // Spanish (Dominican Republic)
#define SUBLANG_SPANISH_VENEZUELA                   0x08    // Spanish (Venezuela)
#define SUBLANG_SPANISH_COLOMBIA                    0x09    // Spanish (Colombia)
#define SUBLANG_SPANISH_PERU                        0x0a    // Spanish (Peru)
#define SUBLANG_SPANISH_ARGENTINA                   0x0b    // Spanish (Argentina)
#define SUBLANG_SPANISH_ECUADOR                     0x0c    // Spanish (Ecuador)
#define SUBLANG_SPANISH_CHILE                       0x0d    // Spanish (Chile)
#define SUBLANG_SPANISH_URUGUAY                     0x0e    // Spanish (Uruguay)
#define SUBLANG_SPANISH_PARAGUAY                    0x0f    // Spanish (Paraguay)
#define SUBLANG_SPANISH_BOLIVIA                     0x10    // Spanish (Bolivia)
#define SUBLANG_SPANISH_EL_SALVADOR                 0x11    // Spanish (El Salvador)
#define SUBLANG_SPANISH_HONDURAS                    0x12    // Spanish (Honduras)
#define SUBLANG_SPANISH_NICARAGUA                   0x13    // Spanish (Nicaragua)
#define SUBLANG_SPANISH_PUERTO_RICO                 0x14    // Spanish (Puerto Rico)
#define SUBLANG_SPANISH_US                          0x15    // Spanish (United States)
#define SUBLANG_SWAHILI_KENYA                       0x01    // Swahili (Kenya) 0x0441 sw-KE
#define SUBLANG_SWEDISH                             0x01    // Swedish
#define SUBLANG_SWEDISH_FINLAND                     0x02    // Swedish (Finland)
#define SUBLANG_SYRIAC_SYRIA                        0x01    // Syriac (Syria) 0x045a syr-SY
#define SUBLANG_TAJIK_TAJIKISTAN                    0x01    // Tajik (Tajikistan) 0x0428 tg-TJ-Cyrl
#define SUBLANG_TAMAZIGHT_ALGERIA_LATIN             0x02    // Tamazight (Latin, Algeria) 0x085f tzm-Latn-DZ
#define SUBLANG_TAMAZIGHT_MOROCCO_TIFINAGH          0x04    // Tamazight (Tifinagh) 0x105f tzm-Tfng-MA
#define SUBLANG_TAMIL_INDIA                         0x01    // Tamil (India)
#define SUBLANG_TAMIL_SRI_LANKA                     0x02    // Tamil (Sri Lanka) 0x0849 ta-LK
#define SUBLANG_TATAR_RUSSIA                        0x01    // Tatar (Russia) 0x0444 tt-RU
#define SUBLANG_TELUGU_INDIA                        0x01    // Telugu (India (Telugu Script)) 0x044a te-IN
#define SUBLANG_THAI_THAILAND                       0x01    // Thai (Thailand) 0x041e th-TH
#define SUBLANG_TIBETAN_PRC                         0x01    // Tibetan (PRC)
#define SUBLANG_TIGRIGNA_ERITREA                    0x02    // Tigrigna (Eritrea)
#define SUBLANG_TIGRINYA_ERITREA                    0x02    // Tigrinya (Eritrea) 0x0873 ti-ER (preferred spelling)
#define SUBLANG_TIGRINYA_ETHIOPIA                   0x01    // Tigrinya (Ethiopia) 0x0473 ti-ET
#define SUBLANG_TSWANA_BOTSWANA                     0x02    // Setswana / Tswana (Botswana) 0x0832 tn-BW
#define SUBLANG_TSWANA_SOUTH_AFRICA                 0x01    // Setswana / Tswana (South Africa) 0x0432 tn-ZA
#define SUBLANG_TURKISH_TURKEY                      0x01    // Turkish (Turkey) 0x041f tr-TR
#define SUBLANG_TURKMEN_TURKMENISTAN                0x01    // Turkmen (Turkmenistan) 0x0442 tk-TM
#define SUBLANG_UIGHUR_PRC                          0x01    // Uighur (PRC) 0x0480 ug-CN
#define SUBLANG_UKRAINIAN_UKRAINE                   0x01    // Ukrainian (Ukraine) 0x0422 uk-UA
#define SUBLANG_UPPER_SORBIAN_GERMANY               0x01    // Upper Sorbian (Germany) 0x042e wen-DE
#define SUBLANG_URDU_PAKISTAN                       0x01    // Urdu (Pakistan)
#define SUBLANG_URDU_INDIA                          0x02    // Urdu (India)
#define SUBLANG_UZBEK_LATIN                         0x01    // Uzbek (Latin)
#define SUBLANG_UZBEK_CYRILLIC                      0x02    // Uzbek (Cyrillic)
#define SUBLANG_VALENCIAN_VALENCIA                  0x02    // Valencian (Valencia) 0x0803 ca-ES-Valencia
#define SUBLANG_VIETNAMESE_VIETNAM                  0x01    // Vietnamese (Vietnam) 0x042a vi-VN
#define SUBLANG_WELSH_UNITED_KINGDOM                0x01    // Welsh (United Kingdom) 0x0452 cy-GB
#define SUBLANG_WOLOF_SENEGAL                       0x01    // Wolof (Senegal)
#define SUBLANG_XHOSA_SOUTH_AFRICA                  0x01    // isiXhosa / Xhosa (South Africa) 0x0434 xh-ZA
#define SUBLANG_YAKUT_RUSSIA                        0x01    // Deprecated: use SUBLANG_SAKHA_RUSSIA instead
#define SUBLANG_YI_PRC                              0x01    // Yi (PRC)) 0x0478
#define SUBLANG_YORUBA_NIGERIA                      0x01    // Yoruba (Nigeria) 046a yo-NG
#define SUBLANG_ZULU_SOUTH_AFRICA                   0x01    // isiZulu / Zulu (South Africa) 0x0435 zu-ZA




//
//  Sorting IDs.
//
//  Note that the named locale APIs (eg CompareStringExEx) are recommended.
//

#define SORT_DEFAULT                     0x0     // sorting default

#define SORT_INVARIANT_MATH              0x1     // Invariant (Mathematical Symbols)

#define SORT_JAPANESE_XJIS               0x0     // Japanese XJIS order
#define SORT_JAPANESE_UNICODE            0x1     // Japanese Unicode order (no longer supported)
#define SORT_JAPANESE_RADICALSTROKE      0x4     // Japanese radical/stroke order

#define SORT_CHINESE_BIG5                0x0     // Chinese BIG5 order
#define SORT_CHINESE_PRCP                0x0     // PRC Chinese Phonetic order
#define SORT_CHINESE_UNICODE             0x1     // Chinese Unicode order (no longer supported)
#define SORT_CHINESE_PRC                 0x2     // PRC Chinese Stroke Count order
#define SORT_CHINESE_BOPOMOFO            0x3     // Traditional Chinese Bopomofo order
#define SORT_CHINESE_RADICALSTROKE       0x4     // Traditional Chinese radical/stroke order.

#define SORT_KOREAN_KSC                  0x0     // Korean KSC order
#define SORT_KOREAN_UNICODE              0x1     // Korean Unicode order (no longer supported)

#define SORT_GERMAN_PHONE_BOOK           0x1     // German Phone Book order

#define SORT_HUNGARIAN_DEFAULT           0x0     // Hungarian Default order
#define SORT_HUNGARIAN_TECHNICAL         0x1     // Hungarian Technical order

#define SORT_GEORGIAN_TRADITIONAL        0x0     // Georgian Traditional order
#define SORT_GEORGIAN_MODERN             0x1     // Georgian Modern order

// end_r_winnt

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  DEPRECATED: The LCID/LANGID/SORTID concept is deprecated, please use
//  Locale Names instead, eg: "en-US" instead of an LCID like 0x0409.
//  See the documentation for GetLocaleInfoEx.
//
//  A language ID is a 16 bit value which is the combination of a
//  primary language ID and a secondary language ID.  The bits are
//  allocated as follows:
//
//       +-----------------------+-------------------------+
//       |     Sublanguage ID    |   Primary Language ID   |
//       +-----------------------+-------------------------+
//        15                   10 9                       0   bit
//
//  WARNING:  This pattern is broken and not followed for all languages.
//            Serbian, Bosnian & Croatian are a few examples.
//
//  WARNING:  There are > 6000 human languages.  The PRIMARYLANGID construct
//            cannot support all languages your application may encounter.
//            Please use Language Names, such as "en".
//
//  WARNING:  There are > 200 country-regions.  The SUBLANGID construct cannot
//            represent all valid dialects of languages such as English.
//            Please use Locale Names, such as "en-US".
//
//  WARNING:  Some languages may have more than one PRIMARYLANGID.  Please
//            use Locale Names, such as "en-FJ".
//
//  WARNING:  Some languages do not have assigned LANGIDs.  Please use
//            Locale Names, such as "tlh-Piqd".
//
//  It is recommended that applications test for locale names rather than
//  attempting to construct/deconstruct LANGID/PRIMARYLANGID/SUBLANGID
//
//  Language ID creation/extraction macros:
//
//    MAKELANGID    - construct language id from a primary language id and
//                    a sublanguage id.
//    PRIMARYLANGID - extract primary language id from a language id.
//    SUBLANGID     - extract sublanguage id from a language id.
//
//  Note that the LANG, SUBLANG construction is not always consistent.
//  The named locale APIs (eg GetLocaleInfoEx) are recommended.
//
//  DEPRECATED: Language IDs do not exist for all locales
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define MAKELANGID(p, s)       ((((WORD  )(s)) << 10) | (WORD  )(p))
#define PRIMARYLANGID(lgid)    ((WORD  )(lgid) & 0x3ff)
#define SUBLANGID(lgid)        ((WORD  )(lgid) >> 10)

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  DEPRECATED: The LCID/LANGID/SORTID concept is deprecated, please use
//  Locale Names instead, eg: en-US instead of an LCID like 0x0409.
//  See the documentation for GetLocaleInfoEx.
//
//  A locale ID is a 32 bit value which is the combination of a
//  language ID, a sort ID, and a reserved area.  The bits are
//  allocated as follows:
//
//       +-------------+---------+-------------------------+
//       |   Reserved  | Sort ID |      Language ID        |
//       +-------------+---------+-------------------------+
//        31         20 19     16 15                      0   bit
//
//  WARNING: This pattern isn't always followed (es-ES_tradnl vs es-ES for example)
//
//  WARNING: Some locales do not have assigned LCIDs.  Please use
//           Locale Names, such as "tlh-Piqd".
//
//  It is recommended that applications test for locale names rather than
//  attempting to rely on LCID or LANGID behavior.
//
//  DEPRECATED: Locale ID creation/extraction macros:
//
//    MAKELCID            - construct the locale id from a language id and a sort id.
//    MAKESORTLCID        - construct the locale id from a language id, sort id, and sort version.
//    LANGIDFROMLCID      - extract the language id from a locale id.
//    SORTIDFROMLCID      - extract the sort id from a locale id.
//    SORTVERSIONFROMLCID - extract the sort version from a locale id.
//
//  Note that the LANG, SUBLANG construction is not always consistent.
//  The named locale APIs (eg GetLocaleInfoEx) are recommended.
//
//  DEPRECATED: LCIDs do not exist for all locales.
//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
#define NLS_VALID_LOCALE_MASK  0x000fffff

#define MAKELCID(lgid, srtid)  ((DWORD)((((DWORD)((WORD  )(srtid))) << 16) |  \
                                         ((DWORD)((WORD  )(lgid)))))
#define MAKESORTLCID(lgid, srtid, ver)                                            \
                               ((DWORD)((MAKELCID(lgid, srtid)) |             \
                                    (((DWORD)((WORD  )(ver))) << 20)))
#define LANGIDFROMLCID(lcid)   ((WORD  )(lcid))
#define SORTIDFROMLCID(lcid)   ((WORD  )((((DWORD)(lcid)) >> 16) & 0xf))
#define SORTVERSIONFROMLCID(lcid)  ((WORD  )((((DWORD)(lcid)) >> 20) & 0xf))

// Maximum Locale Name Length in Windows
// Locale names are preferred to the deprecated LCID/LANGID concepts.
//
// Locale names should follow the BCP47 recommendations and typically
// include language, script, regional variant, and perhaps additional specifiers.
// BCP47 allows some variation, eg: en-US is preferred to en-Latn-US.
#define LOCALE_NAME_MAX_LENGTH   85

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
//  Deprecated default System and User IDs for language and locale.
//
//  Locale names such as LOCALE_NAME_SYSTEM_DEFAULT, LOCALE_NAME_USER_DEFAULT,
//  and LOCALE_NAME_INVARIANT are preferred.  See documentation for GetLocaleInfoEx.
//

#define LANG_SYSTEM_DEFAULT    (MAKELANGID(LANG_NEUTRAL, SUBLANG_SYS_DEFAULT))
#define LANG_USER_DEFAULT      (MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT))

#define LOCALE_SYSTEM_DEFAULT  (MAKELCID(LANG_SYSTEM_DEFAULT, SORT_DEFAULT))
#define LOCALE_USER_DEFAULT    (MAKELCID(LANG_USER_DEFAULT, SORT_DEFAULT))

//
//  Other special IDs for language and locale.
//
//  DEPRECATED: These identifiers are all underspecified and lose information.
//              Please use Locale Names such as "en-FJ".
//              See documentation for GetLocaleInfoEx.
//
#define LOCALE_CUSTOM_DEFAULT                                                 \
          (MAKELCID(MAKELANGID(LANG_NEUTRAL, SUBLANG_CUSTOM_DEFAULT), SORT_DEFAULT))

#define LOCALE_CUSTOM_UNSPECIFIED                                             \
          (MAKELCID(MAKELANGID(LANG_NEUTRAL, SUBLANG_CUSTOM_UNSPECIFIED), SORT_DEFAULT))

#define LOCALE_CUSTOM_UI_DEFAULT                                              \
          (MAKELCID(MAKELANGID(LANG_NEUTRAL, SUBLANG_UI_CUSTOM_DEFAULT), SORT_DEFAULT))

#define LOCALE_NEUTRAL                                                        \
          (MAKELCID(MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL), SORT_DEFAULT))

#define LOCALE_INVARIANT                                                      \
          (MAKELCID(MAKELANGID(LANG_INVARIANT, SUBLANG_NEUTRAL), SORT_DEFAULT))

//
// Transient keyboard Locale IDs (LCIDs)
// Should only be used for keyboard layout identification
//
//  DEPRECATED: These identifiers are all transient and will change, even at
//              different times on the same system.
//              Please use Locale Names such as "en-FJ".
//              See documentation for GetLocaleInfoEx.
//
#define LOCALE_TRANSIENT_KEYBOARD1  0x2000
#define LOCALE_TRANSIENT_KEYBOARD2  0x2400
#define LOCALE_TRANSIENT_KEYBOARD3  0x2800
#define LOCALE_TRANSIENT_KEYBOARD4  0x2c00

//
// Locale with an unassigned LCID
// These locales cannot be queried by LCID
// Currently same as LOCALE_CUSTOM_UNSPECIFIED
//
// DEPRECATED: Please use Locale Names; see documentation for GetLocaleInfoEx.
//
#define LOCALE_UNASSIGNED_LCID LOCALE_CUSTOM_UNSPECIFIED

// begin_ntminiport begin_ntndis begin_ntminitape

//
// Macros used to eliminate compiler warning generated when formal
// parameters or local variables are not declared.
//
// Use DBG_UNREFERENCED_PARAMETER() when a parameter is not yet
// referenced but will be once the module is completely developed.
//
// Use DBG_UNREFERENCED_LOCAL_VARIABLE() when a local variable is not yet
// referenced but will be once the module is completely developed.
//
// Use UNREFERENCED_PARAMETER() if a parameter will never be referenced.
//
// DBG_UNREFERENCED_PARAMETER and DBG_UNREFERENCED_LOCAL_VARIABLE will
// eventually be made into a null macro to help determine whether there
// is unfinished work.
//

// begin_ntoshvp

#if ! defined(lint)

#ifdef _PREFAST_

void _Prefast_unreferenced_parameter_impl_(const char*, ...);
#define UNREFERENCED_PARAMETER(P)          _Prefast_unreferenced_parameter_impl_("PREfast", ((void) (P), 0))
#define DBG_UNREFERENCED_PARAMETER(P)      _Prefast_unreferenced_parameter_impl_("PREfast", ((void) (P), 0))
#define DBG_UNREFERENCED_LOCAL_VARIABLE(V) _Prefast_unreferenced_parameter_impl_("PREfast", ((void) (V), 0))

#else // _PREFAST_

#define UNREFERENCED_PARAMETER(P)          (P)
#define DBG_UNREFERENCED_PARAMETER(P)      (P)
#define DBG_UNREFERENCED_LOCAL_VARIABLE(V) (V)

#endif // _PREFAST_

#else // lint

// Note: lint -e530 says don't complain about uninitialized variables for
// this variable.  Error 527 has to do with unreachable code.
// -restore restores checking to the -save state

#define UNREFERENCED_PARAMETER(P)          \
    /*lint -save -e527 -e530 */ \
    { \
        (P) = (P); \
    } \
    /*lint -restore */
#define DBG_UNREFERENCED_PARAMETER(P)      \
    /*lint -save -e527 -e530 */ \
    { \
        (P) = (P); \
    } \
    /*lint -restore */
#define DBG_UNREFERENCED_LOCAL_VARIABLE(V) \
    /*lint -save -e527 -e530 */ \
    { \
        (V) = (V); \
    } \
    /*lint -restore */

#endif // lint

// end_ntoshvp

//
// Macro used to eliminate compiler warning 4715 within a switch statement
// when all possible cases have already been accounted for.
//
// switch (a & 3) {
//     case 0: return 1;
//     case 1: return Foo();
//     case 2: return Bar();
//     case 3: return 1;
//     DEFAULT_UNREACHABLE;
//

#if (_MSC_VER > 1200)
#define DEFAULT_UNREACHABLE default: __assume(0)
#else

//
// Older compilers do not support __assume(), and there is no other free
// method of eliminating the warning.
//

#define DEFAULT_UNREACHABLE

#endif

#ifdef __cplusplus

// Define operator overloads to enable bit operations on enum values that are
// used to define flags. Use DEFINE_ENUM_FLAG_OPERATORS(YOUR_TYPE) to enable these
// operators on YOUR_TYPE.

// Moved here from objbase.w.

// Templates are defined here in order to avoid a dependency on C++ <type_traits> header file,
// or on compiler-specific constructs.
extern "C++" {

    template <size_t S>
    struct _ENUM_FLAG_INTEGER_FOR_SIZE;

    template <>
    struct _ENUM_FLAG_INTEGER_FOR_SIZE<1>
    {
        typedef INT8 type;
    };

    template <>
    struct _ENUM_FLAG_INTEGER_FOR_SIZE<2>
    {
        typedef INT16 type;
    };

    template <>
    struct _ENUM_FLAG_INTEGER_FOR_SIZE<4>
    {
        typedef INT32 type;
    };

    template <>
    struct _ENUM_FLAG_INTEGER_FOR_SIZE<8>
    {
        typedef INT64 type;
    };

    // used as an approximation of std::underlying_type<T>
    template <class T>
    struct _ENUM_FLAG_SIZED_INTEGER
    {
        typedef typename _ENUM_FLAG_INTEGER_FOR_SIZE<sizeof(T)>::type type;
    };

}

#if _MSC_VER >= 1900
#define _ENUM_FLAG_CONSTEXPR constexpr
#else
#define _ENUM_FLAG_CONSTEXPR
#endif

#define DEFINE_ENUM_FLAG_OPERATORS(ENUMTYPE) \
extern "C++" { \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator | (ENUMTYPE a, ENUMTYPE b) WIN_NOEXCEPT { return ENUMTYPE(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a) | ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline ENUMTYPE &operator |= (ENUMTYPE &a, ENUMTYPE b) WIN_NOEXCEPT { return (ENUMTYPE &)(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type &)a) |= ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator & (ENUMTYPE a, ENUMTYPE b) WIN_NOEXCEPT { return ENUMTYPE(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a) & ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline ENUMTYPE &operator &= (ENUMTYPE &a, ENUMTYPE b) WIN_NOEXCEPT { return (ENUMTYPE &)(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type &)a) &= ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator ~ (ENUMTYPE a) WIN_NOEXCEPT { return ENUMTYPE(~((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a)); } \
inline _ENUM_FLAG_CONSTEXPR ENUMTYPE operator ^ (ENUMTYPE a, ENUMTYPE b) WIN_NOEXCEPT { return ENUMTYPE(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)a) ^ ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
inline ENUMTYPE &operator ^= (ENUMTYPE &a, ENUMTYPE b) WIN_NOEXCEPT { return (ENUMTYPE &)(((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type &)a) ^= ((_ENUM_FLAG_SIZED_INTEGER<ENUMTYPE>::type)b)); } \
}
#else
#define DEFINE_ENUM_FLAG_OPERATORS(ENUMTYPE) // NOP, C allows these operators.
#endif

// Compile-time macros for initializing flag values in const data.
//
// When using DEFINE_ENUM_FLAG_OPERATORS for enum values you should use the macros below
// when you need to initialize global const data.  Without these macros the inline operators
// from DEFINE_ENUM_FLAG_OPERATORS force a runtime initialization rather than a
// compile time initialization.  This applies even if you have declared the data as const.
//
// This is no longer necessary for compilers that support constexpr.
#define COMPILETIME_OR_2FLAGS(a,b)          ((UINT)(a)|(UINT)(b))
#define COMPILETIME_OR_3FLAGS(a,b,c)        ((UINT)(a)|(UINT)(b)|(UINT)(c))
#define COMPILETIME_OR_4FLAGS(a,b,c,d)      ((UINT)(a)|(UINT)(b)|(UINT)(c)|(UINT)(d))
#define COMPILETIME_OR_5FLAGS(a,b,c,d,e)    ((UINT)(a)|(UINT)(b)|(UINT)(c)|(UINT)(d)|(UINT)(e))
#define COMPILETIME_OR_6FLAGS(a,b,c,d,e,f)  ((UINT)(a)|(UINT)(b)|(UINT)(c)|(UINT)(d)|(UINT)(e)|(UINT)(f))


// Much of the Windows SDK assumes the default packing of structs.
#if !defined(WINDOWS_IGNORE_PACKING_MISMATCH) && !defined(__midl) && !defined(MIDL_PASS) && !defined(SORTPP_PASS) && !defined(RC_INVOKED)
#if defined(__cplusplus) && (_MSC_VER >= 1600)
static_assert(__alignof(LARGE_INTEGER) == 8, "Windows headers require the default packing option. Changing this can lead to memory corruption."
    " This diagnostic can be disabled by building with WINDOWS_IGNORE_PACKING_MISMATCH defined.");
#elif _MSC_VER >= 1300
#pragma warning(push)
#pragma warning(disable: 4116)
C_ASSERT(TYPE_ALIGNMENT(LARGE_INTEGER) == 8);
#pragma warning(pop)
#endif
#endif


#ifndef UMDF_USING_NTSTATUS
#ifndef WIN32_NO_STATUS 
/*lint -save -e767 */  
#define STATUS_WAIT_0                           ((DWORD   )0x00000000L) 
#define STATUS_ABANDONED_WAIT_0          ((DWORD   )0x00000080L)    
#define STATUS_USER_APC                  ((DWORD   )0x000000C0L)    
#define STATUS_TIMEOUT                   ((DWORD   )0x00000102L)    
#define STATUS_PENDING                   ((DWORD   )0x00000103L)    
#define DBG_EXCEPTION_HANDLED            ((DWORD   )0x00010001L)    
#define DBG_CONTINUE                     ((DWORD   )0x00010002L)    
#define STATUS_SEGMENT_NOTIFICATION      ((DWORD   )0x40000005L)    
#define STATUS_FATAL_APP_EXIT            ((DWORD   )0x40000015L)    
#define DBG_REPLY_LATER                  ((DWORD   )0x40010001L)    
#define DBG_TERMINATE_THREAD             ((DWORD   )0x40010003L)    
#define DBG_TERMINATE_PROCESS            ((DWORD   )0x40010004L)    
#define DBG_CONTROL_C                    ((DWORD   )0x40010005L)    
#define DBG_PRINTEXCEPTION_C             ((DWORD   )0x40010006L)    
#define DBG_RIPEXCEPTION                 ((DWORD   )0x40010007L)    
#define DBG_CONTROL_BREAK                ((DWORD   )0x40010008L)    
#define DBG_COMMAND_EXCEPTION            ((DWORD   )0x40010009L)    
#define DBG_PRINTEXCEPTION_WIDE_C        ((DWORD   )0x4001000AL)    
#define STATUS_GUARD_PAGE_VIOLATION      ((DWORD   )0x80000001L)    
#define STATUS_DATATYPE_MISALIGNMENT     ((DWORD   )0x80000002L)    
#define STATUS_BREAKPOINT                ((DWORD   )0x80000003L)    
#define STATUS_SINGLE_STEP               ((DWORD   )0x80000004L)    
#define STATUS_LONGJUMP                  ((DWORD   )0x80000026L)    
#define STATUS_UNWIND_CONSOLIDATE        ((DWORD   )0x80000029L)    
#define DBG_EXCEPTION_NOT_HANDLED        ((DWORD   )0x80010001L)    
#define STATUS_ACCESS_VIOLATION          ((DWORD   )0xC0000005L)    
#define STATUS_IN_PAGE_ERROR             ((DWORD   )0xC0000006L)    
#define STATUS_INVALID_HANDLE            ((DWORD   )0xC0000008L)    
#define STATUS_INVALID_PARAMETER         ((DWORD   )0xC000000DL)    
#define STATUS_NO_MEMORY                 ((DWORD   )0xC0000017L)    
#define STATUS_ILLEGAL_INSTRUCTION       ((DWORD   )0xC000001DL)    
#define STATUS_NONCONTINUABLE_EXCEPTION  ((DWORD   )0xC0000025L)    
#define STATUS_INVALID_DISPOSITION       ((DWORD   )0xC0000026L)    
#define STATUS_ARRAY_BOUNDS_EXCEEDED     ((DWORD   )0xC000008CL)    
#define STATUS_FLOAT_DENORMAL_OPERAND    ((DWORD   )0xC000008DL)    
#define STATUS_FLOAT_DIVIDE_BY_ZERO      ((DWORD   )0xC000008EL)    
#define STATUS_FLOAT_INEXACT_RESULT      ((DWORD   )0xC000008FL)    
#define STATUS_FLOAT_INVALID_OPERATION   ((DWORD   )0xC0000090L)    
#define STATUS_FLOAT_OVERFLOW            ((DWORD   )0xC0000091L)    
#define STATUS_FLOAT_STACK_CHECK         ((DWORD   )0xC0000092L)    
#define STATUS_FLOAT_UNDERFLOW           ((DWORD   )0xC0000093L)    
#define STATUS_INTEGER_DIVIDE_BY_ZERO    ((DWORD   )0xC0000094L)    
#define STATUS_INTEGER_OVERFLOW          ((DWORD   )0xC0000095L)    
#define STATUS_PRIVILEGED_INSTRUCTION    ((DWORD   )0xC0000096L)    
#define STATUS_STACK_OVERFLOW            ((DWORD   )0xC00000FDL)    
#define STATUS_DLL_NOT_FOUND             ((DWORD   )0xC0000135L)    
#define STATUS_ORDINAL_NOT_FOUND         ((DWORD   )0xC0000138L)    
#define STATUS_ENTRYPOINT_NOT_FOUND      ((DWORD   )0xC0000139L)    
#define STATUS_CONTROL_C_EXIT            ((DWORD   )0xC000013AL)    
#define STATUS_DLL_INIT_FAILED           ((DWORD   )0xC0000142L)    
#define STATUS_CONTROL_STACK_VIOLATION   ((DWORD   )0xC00001B2L)    
#define STATUS_FLOAT_MULTIPLE_FAULTS     ((DWORD   )0xC00002B4L)    
#define STATUS_FLOAT_MULTIPLE_TRAPS      ((DWORD   )0xC00002B5L)    
#define STATUS_REG_NAT_CONSUMPTION       ((DWORD   )0xC00002C9L)    
#define STATUS_HEAP_CORRUPTION           ((DWORD   )0xC0000374L)    
#define STATUS_STACK_BUFFER_OVERRUN      ((DWORD   )0xC0000409L)    
#define STATUS_INVALID_CRUNTIME_PARAMETER ((DWORD   )0xC0000417L)    
#define STATUS_ASSERTION_FAILURE         ((DWORD   )0xC0000420L)    
#define STATUS_ENCLAVE_VIOLATION         ((DWORD   )0xC00004A2L)    
#define STATUS_INTERRUPTED               ((DWORD   )0xC0000515L)    
#define STATUS_THREAD_NOT_RUNNING        ((DWORD   )0xC0000516L)    
#define STATUS_ALREADY_REGISTERED        ((DWORD   )0xC0000718L)    
#if defined(STATUS_SUCCESS) || (_WIN32_WINNT > 0x0500) || (_WIN32_FUSION >= 0x0100) 
#define STATUS_SXS_EARLY_DEACTIVATION    ((DWORD   )0xC015000FL)    
#define STATUS_SXS_INVALID_DEACTIVATION  ((DWORD   )0xC0150010L)    
#endif 
/*lint -restore */  
#endif 
#endif /* UMDF_USING_NTSTATUS */

#define MAXIMUM_WAIT_OBJECTS 64     // Maximum number of wait objects

#define MAXIMUM_SUSPEND_COUNT MAXCHAR // Maximum times thread can be suspended

typedef ULONG_PTR KSPIN_LOCK;
typedef KSPIN_LOCK *PKSPIN_LOCK;

// begin_ntoshvp

//
// Define 128-bit 16-byte aligned xmm register type.
//

typedef struct DECLSPEC_ALIGN(16) _M128A {
    ULONGLONG Low;
    LONGLONG High;
} M128A, *PM128A;

//
// Format of data for (F)XSAVE/(F)XRSTOR instruction
//

typedef struct DECLSPEC_ALIGN(16) _XSAVE_FORMAT {
    WORD   ControlWord;
    WORD   StatusWord;
    BYTE  TagWord;
    BYTE  Reserved1;
    WORD   ErrorOpcode;
    DWORD ErrorOffset;
    WORD   ErrorSelector;
    WORD   Reserved2;
    DWORD DataOffset;
    WORD   DataSelector;
    WORD   Reserved3;
    DWORD MxCsr;
    DWORD MxCsr_Mask;
    M128A FloatRegisters[8];

#if defined(_WIN64)

    M128A XmmRegisters[16];
    BYTE  Reserved4[96];

#else

    M128A XmmRegisters[8];
    BYTE  Reserved4[224];

#endif

} XSAVE_FORMAT, *PXSAVE_FORMAT;

// end_ntoshvp

//
// Format for CET_U XSTATE component.
//

typedef struct _XSAVE_CET_U_FORMAT {
    DWORD64 Ia32CetUMsr;
    DWORD64 Ia32Pl3SspMsr;
} XSAVE_CET_U_FORMAT, *PXSAVE_CET_U_FORMAT;

//
// Header for ARM64 SVE component.
//

typedef struct _XSAVE_ARM64_SVE_HEADER {
    DWORD VectorLength;
    DWORD VectorRegisterOffset;
    DWORD PredicateRegisterOffset;
    DWORD Reserved[5];
} XSAVE_ARM64_SVE_HEADER, *PXSAVE_ARM64_SVE_HEADER;

#if !defined(__midl) && !defined(MIDL_PASS)

C_ASSERT(sizeof(XSAVE_ARM64_SVE_HEADER) == (4 * sizeof(DWORD64)));

#endif

typedef struct DECLSPEC_ALIGN(8) _XSAVE_AREA_HEADER {
    DWORD64 Mask;
    DWORD64 CompactionMask;
    DWORD64 Reserved2[6];
} XSAVE_AREA_HEADER, *PXSAVE_AREA_HEADER;

typedef struct DECLSPEC_ALIGN(16) _XSAVE_AREA {
    XSAVE_FORMAT LegacyState;
    XSAVE_AREA_HEADER Header;
} XSAVE_AREA, *PXSAVE_AREA;

#define XSTATE_CONTEXT_FLAG_LOOKASIDE    0x1

typedef struct _XSTATE_CONTEXT {
    DWORD64 Mask;
    DWORD Length;
    BYTE  Flags;
    BYTE  Reserved0[3];
    _Field_size_bytes_opt_(Length) PXSAVE_AREA Area;

#if defined(_X86_)
    DWORD Reserved2;
#endif

    PVOID Buffer;

#if defined(_X86_)
    DWORD Reserved3;
#endif

} XSTATE_CONTEXT, *PXSTATE_CONTEXT;

typedef struct _KERNEL_CET_CONTEXT {
    DWORD64 Ssp;
    DWORD64 Rip;
    WORD   SegCs;
    union {
        WORD   AllFlags;
        struct {
            WORD   UseWrss : 1;
            WORD   PopShadowStackOne : 1;
            WORD   Unused : 14;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
    WORD   Fill[2];
} KERNEL_CET_CONTEXT, *PKERNEL_CET_CONTEXT;

#if !defined(__midl) && !defined(MIDL_PASS)

C_ASSERT(sizeof(KERNEL_CET_CONTEXT) == (3 * sizeof(DWORD64)));

#endif

//

//
// Scope table structure definition.
//

typedef struct _SCOPE_TABLE_AMD64 {
    DWORD Count;
    struct {
        DWORD BeginAddress;
        DWORD EndAddress;
        DWORD HandlerAddress;
        DWORD JumpTarget;
    } ScopeRecord[1];
} SCOPE_TABLE_AMD64, *PSCOPE_TABLE_AMD64;

//
//

#if defined(_AMD64_)

//
//

#if defined(_M_AMD64) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

//
// Define bit test intrinsics.
//

#ifdef __cplusplus
extern "C" {
#endif

#define BitTest _bittest
#define BitTestAndComplement _bittestandcomplement
#define BitTestAndSet _bittestandset
#define BitTestAndReset _bittestandreset

#if !defined(_M_ARM64EC)
#define InterlockedBitTestAndSet _interlockedbittestandset
#define InterlockedBitTestAndSetAcquire _interlockedbittestandset
#define InterlockedBitTestAndSetRelease _interlockedbittestandset
#define InterlockedBitTestAndSetNoFence _interlockedbittestandset
#define InterlockedBitTestAndReset _interlockedbittestandreset
#define InterlockedBitTestAndResetAcquire _interlockedbittestandreset
#define InterlockedBitTestAndResetRelease _interlockedbittestandreset
#define InterlockedBitTestAndResetNoFence _interlockedbittestandreset
#endif // !defined(_M_ARM64EC)

#define BitTest64 _bittest64
#define BitTestAndComplement64 _bittestandcomplement64
#define BitTestAndSet64 _bittestandset64
#define BitTestAndReset64 _bittestandreset64
#if !defined(_M_ARM64EC)
#define InterlockedBitTestAndSet64 _interlockedbittestandset64
#define InterlockedBitTestAndSet64Acquire _interlockedbittestandset64
#define InterlockedBitTestAndSet64Release _interlockedbittestandset64
#define InterlockedBitTestAndSet64NoFence _interlockedbittestandset64
#define InterlockedBitTestAndReset64 _interlockedbittestandreset64
#define InterlockedBitTestAndReset64Acquire _interlockedbittestandreset64
#define InterlockedBitTestAndReset64Release _interlockedbittestandreset64
#define InterlockedBitTestAndReset64NoFence _interlockedbittestandreset64
#endif // !defined(_M_ARM64EC)

_Must_inspect_result_
BOOLEAN
_bittest (
    _In_reads_bytes_((Offset/8)+1) LONG const *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittestandcomplement (
    _Inout_updates_bytes_((Offset/8)+1) LONG *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittestandset (
    _Inout_updates_bytes_((Offset/8)+1) LONG *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittestandreset (
    _Inout_updates_bytes_((Offset/8)+1) LONG *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_interlockedbittestandset (
    _Inout_updates_bytes_((Offset/8)+1) _Interlocked_operand_ LONG volatile *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_interlockedbittestandreset (
    _Inout_updates_bytes_((Offset/8)+1) _Interlocked_operand_ LONG volatile *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittest64 (
    _In_reads_bytes_((Offset/8)+1) LONG64 const *Base,
    _In_range_(>=,0) LONG64 Offset
    );

BOOLEAN
_bittestandcomplement64 (
    _Inout_updates_bytes_((Offset/8)+1) LONG64 *Base,
    _In_range_(>=,0) LONG64 Offset
    );

BOOLEAN
_bittestandset64 (
    _Inout_updates_bytes_((Offset/8)+1) LONG64 *Base,
    _In_range_(>=,0) LONG64 Offset
    );

BOOLEAN
_bittestandreset64 (
    _Inout_updates_bytes_((Offset/8)+1) LONG64 *Base,
    _In_range_(>=,0) LONG64 Offset
    );

BOOLEAN
_interlockedbittestandset64 (
    _Inout_updates_bytes_((Offset/8)+1) _Interlocked_operand_ LONG64 volatile *Base,
    _In_range_(>=,0) LONG64 Offset
    );

BOOLEAN
_interlockedbittestandreset64 (
    _Inout_updates_bytes_((Offset/8)+1) _Interlocked_operand_ LONG64 volatile *Base,
    _In_range_(>=,0) LONG64 Offset
    );

#pragma intrinsic(_bittest)
#pragma intrinsic(_bittestandcomplement)
#pragma intrinsic(_bittestandset)
#pragma intrinsic(_bittestandreset)

#if !defined(_M_ARM64EC)
#pragma intrinsic(_interlockedbittestandset)
#pragma intrinsic(_interlockedbittestandreset)
#endif

#pragma intrinsic(_bittest64)
#pragma intrinsic(_bittestandcomplement64)
#pragma intrinsic(_bittestandset64)
#pragma intrinsic(_bittestandreset64)

#if !defined(_M_ARM64EC)
#pragma intrinsic(_interlockedbittestandset64)
#pragma intrinsic(_interlockedbittestandreset64)
#endif

//
// Define bit scan intrinsics.
//

#define BitScanForward _BitScanForward
#define BitScanReverse _BitScanReverse
#define BitScanForward64 _BitScanForward64
#define BitScanReverse64 _BitScanReverse64

_Success_(return!=0)
BOOLEAN
_BitScanForward (
    _Out_ DWORD *Index,
    _In_ DWORD Mask
    );

_Success_(return!=0)
BOOLEAN
_BitScanReverse (
    _Out_ DWORD *Index,
    _In_ DWORD Mask
    );

_Success_(return!=0)
BOOLEAN
_BitScanForward64 (
    _Out_ DWORD *Index,
    _In_ DWORD64 Mask
    );

_Success_(return!=0)
BOOLEAN
_BitScanReverse64 (
    _Out_ DWORD *Index,
    _In_ DWORD64 Mask
    );

#pragma intrinsic(_BitScanForward)
#pragma intrinsic(_BitScanReverse)
#pragma intrinsic(_BitScanForward64)
#pragma intrinsic(_BitScanReverse64)

//
// Interlocked intrinsic functions.
//

#define InterlockedIncrement16 _InterlockedIncrement16
#define InterlockedDecrement16 _InterlockedDecrement16
#define InterlockedCompareExchange16 _InterlockedCompareExchange16
#define InterlockedAnd _InterlockedAnd
#define InterlockedOr _InterlockedOr
#define InterlockedXor _InterlockedXor
#define InterlockedIncrement _InterlockedIncrement
#define InterlockedDecrement _InterlockedDecrement
#define InterlockedExchange _InterlockedExchange
#define InterlockedExchangeAdd _InterlockedExchangeAdd
#define InterlockedCompareExchange _InterlockedCompareExchange

#define InterlockedAnd64 _InterlockedAnd64
#define InterlockedOr64 _InterlockedOr64
#define InterlockedXor64 _InterlockedXor64
#define InterlockedIncrement64 _InterlockedIncrement64
#define InterlockedDecrement64 _InterlockedDecrement64
#define InterlockedExchange64 _InterlockedExchange64
#define InterlockedExchangeAdd64 _InterlockedExchangeAdd64
#define InterlockedCompareExchange64 _InterlockedCompareExchange64
#define InterlockedCompareExchange128 _InterlockedCompareExchange128
#define InterlockedExchangePointer _InterlockedExchangePointer
#define InterlockedCompareExchangePointer _InterlockedCompareExchangePointer

#if !defined(_M_ARM64EC)
#define InterlockedIncrementAcquire16 _InterlockedIncrement16
#define InterlockedIncrementRelease16 _InterlockedIncrement16
#define InterlockedIncrementNoFence16 _InterlockedIncrement16
#define InterlockedDecrement16 _InterlockedDecrement16
#define InterlockedDecrementAcquire16 _InterlockedDecrement16
#define InterlockedDecrementRelease16 _InterlockedDecrement16
#define InterlockedDecrementNoFence16 _InterlockedDecrement16
#define InterlockedCompareExchange16 _InterlockedCompareExchange16
#define InterlockedCompareExchangeAcquire16 _InterlockedCompareExchange16
#define InterlockedCompareExchangeRelease16 _InterlockedCompareExchange16
#define InterlockedCompareExchangeNoFence16 _InterlockedCompareExchange16

#define InterlockedAnd _InterlockedAnd
#define InterlockedAndAcquire _InterlockedAnd
#define InterlockedAndRelease _InterlockedAnd
#define InterlockedAndNoFence _InterlockedAnd
#define InterlockedOr _InterlockedOr
#define InterlockedOrAcquire _InterlockedOr
#define InterlockedOrRelease _InterlockedOr
#define InterlockedOrNoFence _InterlockedOr
#define InterlockedXor _InterlockedXor
#define InterlockedXorAcquire _InterlockedXor
#define InterlockedXorRelease _InterlockedXor
#define InterlockedXorNoFence _InterlockedXor
#define InterlockedIncrement _InterlockedIncrement
#define InterlockedIncrementAcquire _InterlockedIncrement
#define InterlockedIncrementRelease _InterlockedIncrement
#define InterlockedIncrementNoFence _InterlockedIncrement
#define InterlockedDecrement _InterlockedDecrement
#define InterlockedDecrementAcquire _InterlockedDecrement
#define InterlockedDecrementRelease _InterlockedDecrement
#define InterlockedDecrementNoFence _InterlockedDecrement
#define InterlockedAdd _InlineInterlockedAdd
#define InterlockedAddAcquire _InlineInterlockedAdd
#define InterlockedAddRelease _InlineInterlockedAdd
#define InterlockedAddNoFence _InlineInterlockedAdd
#define InterlockedExchange _InterlockedExchange
#define InterlockedExchangeAcquire _InterlockedExchange
#define InterlockedExchangeNoFence _InterlockedExchange
#define InterlockedExchangeAdd _InterlockedExchangeAdd
#define InterlockedExchangeAddAcquire _InterlockedExchangeAdd
#define InterlockedExchangeAddRelease _InterlockedExchangeAdd
#define InterlockedExchangeAddNoFence _InterlockedExchangeAdd
#define InterlockedCompareExchange _InterlockedCompareExchange
#define InterlockedCompareExchangeAcquire _InterlockedCompareExchange
#define InterlockedCompareExchangeRelease _InterlockedCompareExchange
#define InterlockedCompareExchangeNoFence _InterlockedCompareExchange

#define InterlockedAnd64 _InterlockedAnd64
#define InterlockedAnd64Acquire _InterlockedAnd64
#define InterlockedAnd64Release _InterlockedAnd64
#define InterlockedAnd64NoFence _InterlockedAnd64
#define InterlockedAndAffinity InterlockedAnd64
#define InterlockedOr64 _InterlockedOr64
#define InterlockedOr64Acquire _InterlockedOr64
#define InterlockedOr64Release _InterlockedOr64
#define InterlockedOr64NoFence _InterlockedOr64
#define InterlockedOrAffinity InterlockedOr64
#define InterlockedXor64 _InterlockedXor64
#define InterlockedXor64Acquire _InterlockedXor64
#define InterlockedXor64Release _InterlockedXor64
#define InterlockedXor64NoFence _InterlockedXor64
#define InterlockedIncrement64 _InterlockedIncrement64
#define InterlockedIncrementAcquire64 _InterlockedIncrement64
#define InterlockedIncrementRelease64 _InterlockedIncrement64
#define InterlockedIncrementNoFence64 _InterlockedIncrement64
#define InterlockedDecrement64 _InterlockedDecrement64
#define InterlockedDecrementAcquire64 _InterlockedDecrement64
#define InterlockedDecrementRelease64 _InterlockedDecrement64
#define InterlockedDecrementNoFence64 _InterlockedDecrement64
#define InterlockedAdd64 _InlineInterlockedAdd64
#define InterlockedAddAcquire64 _InlineInterlockedAdd64
#define InterlockedAddRelease64 _InlineInterlockedAdd64
#define InterlockedAddNoFence64 _InlineInterlockedAdd64
#define InterlockedExchange64 _InterlockedExchange64
#define InterlockedExchangeAcquire64 InterlockedExchange64
#define InterlockedExchangeNoFence64 InterlockedExchange64
#define InterlockedExchangeAdd64 _InterlockedExchangeAdd64
#define InterlockedExchangeAddAcquire64 _InterlockedExchangeAdd64
#define InterlockedExchangeAddRelease64 _InterlockedExchangeAdd64
#define InterlockedExchangeAddNoFence64 _InterlockedExchangeAdd64
#define InterlockedCompareExchange64 _InterlockedCompareExchange64
#define InterlockedCompareExchangeAcquire64 InterlockedCompareExchange64
#define InterlockedCompareExchangeRelease64 InterlockedCompareExchange64
#define InterlockedCompareExchangeNoFence64 InterlockedCompareExchange64
#define InterlockedCompareExchange128 _InterlockedCompareExchange128
#define InterlockedCompareExchangeAcquire128 _InterlockedCompareExchange128
#define InterlockedCompareExchangeRelease128 _InterlockedCompareExchange128
#define InterlockedCompareExchangeNoFence128 _InterlockedCompareExchange128

#define InterlockedExchangePointer _InterlockedExchangePointer
#define InterlockedExchangePointerNoFence _InterlockedExchangePointer
#define InterlockedExchangePointerAcquire _InterlockedExchangePointer
#define InterlockedCompareExchangePointer _InterlockedCompareExchangePointer
#define InterlockedCompareExchangePointerAcquire _InterlockedCompareExchangePointer
#define InterlockedCompareExchangePointerRelease _InterlockedCompareExchangePointer
#define InterlockedCompareExchangePointerNoFence _InterlockedCompareExchangePointer

#define InterlockedExchangeAddSizeT(a, b) InterlockedExchangeAdd64((LONG64 *)a, b)
#define InterlockedExchangeAddSizeTAcquire(a, b) InterlockedExchangeAdd64((LONG64 *)a, b)
#define InterlockedExchangeAddSizeTNoFence(a, b) InterlockedExchangeAdd64((LONG64 *)a, b)
#define InterlockedIncrementSizeT(a) InterlockedIncrement64((LONG64 *)a)
#define InterlockedIncrementSizeTNoFence(a) InterlockedIncrement64((LONG64 *)a)
#define InterlockedDecrementSizeT(a) InterlockedDecrement64((LONG64 *)a)
#define InterlockedDecrementSizeTNoFence(a) InterlockedDecrement64((LONG64 *)a)
#endif // !defined(_M_ARM64EC)

SHORT
InterlockedIncrement16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Addend
    );

SHORT
InterlockedDecrement16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Addend
    );

SHORT
InterlockedCompareExchange16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT ExChange,
    _In_ SHORT Comperand
    );

LONG
InterlockedAnd (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG
InterlockedOr (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG
InterlockedXor (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG64
InterlockedAnd64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    );

LONG64
InterlockedOr64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    );

LONG64
InterlockedXor64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    );

LONG
InterlockedIncrement (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend
    );

LONG
InterlockedDecrement (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend
    );

LONG
InterlockedExchange (
    _Inout_ _Interlocked_operand_ LONG volatile *Target,
    _In_ LONG Value
    );

LONG
InterlockedExchangeAdd (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend,
    _In_ LONG Value
    );

#if !defined(_X86AMD64_)

__forceinline
LONG
InterlockedAdd (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend,
    _In_ LONG Value
    )

{
    return InterlockedExchangeAdd(Addend, Value) + Value;
}

#endif // !defined(_X86AMD64_)

LONG
InterlockedCompareExchange (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG ExChange,
    _In_ LONG Comperand
    );

LONG64
InterlockedIncrement64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Addend
    );

LONG64
InterlockedDecrement64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Addend
    );

LONG64
InterlockedExchange64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Target,
    _In_ LONG64 Value
    );

LONG64
InterlockedExchangeAdd64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Addend,
    _In_ LONG64 Value
    );

#if !defined(_X86AMD64_)

__forceinline
LONG64
_InlineInterlockedAdd64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Addend,
    _In_ LONG64 Value
    )

{
    return InterlockedExchangeAdd64(Addend, Value) + Value;
}

#endif // !defined(_X86AMD64_)

LONG64
InterlockedCompareExchange64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 ExChange,
    _In_ LONG64 Comperand
    );

BOOLEAN
InterlockedCompareExchange128 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 ExchangeHigh,
    _In_ LONG64 ExchangeLow,
    _Inout_ LONG64 *ComparandResult
    );

_Ret_writes_(_Inexpressible_(Unknown)) PVOID
InterlockedCompareExchangePointer (
    _Inout_ _At_(*Destination,
        _Pre_writable_byte_size_(_Inexpressible_(Unknown))
        _Post_writable_byte_size_(_Inexpressible_(Unknown)))
    _Interlocked_operand_ PVOID volatile *Destination,
    _In_opt_ PVOID Exchange,
    _In_opt_ PVOID Comperand
    );

_Ret_writes_(_Inexpressible_(Unknown)) PVOID
InterlockedExchangePointer(
    _Inout_ _At_(*Target,
        _Pre_writable_byte_size_(_Inexpressible_(Unknown))
        _Post_writable_byte_size_(_Inexpressible_(Unknown)))
    _Interlocked_operand_ PVOID volatile *Target,
    _In_opt_ PVOID Value
    );

#if !defined(_M_ARM64EC)
#pragma intrinsic(_InterlockedIncrement16)
#pragma intrinsic(_InterlockedDecrement16)
#pragma intrinsic(_InterlockedCompareExchange16)
#pragma intrinsic(_InterlockedAnd)
#pragma intrinsic(_InterlockedOr)
#pragma intrinsic(_InterlockedXor)
#pragma intrinsic(_InterlockedIncrement)
#pragma intrinsic(_InterlockedDecrement)
#pragma intrinsic(_InterlockedExchange)
#pragma intrinsic(_InterlockedExchangeAdd)
#pragma intrinsic(_InterlockedCompareExchange)
#pragma intrinsic(_InterlockedAnd64)
#pragma intrinsic(_InterlockedOr64)
#pragma intrinsic(_InterlockedXor64)
#pragma intrinsic(_InterlockedIncrement64)
#pragma intrinsic(_InterlockedDecrement64)
#pragma intrinsic(_InterlockedExchange64)
#pragma intrinsic(_InterlockedExchangeAdd64)
#pragma intrinsic(_InterlockedCompareExchange64)

#if _MSC_VER >= 1500

#pragma intrinsic(_InterlockedCompareExchange128)

#endif

#pragma intrinsic(_InterlockedExchangePointer)
#pragma intrinsic(_InterlockedCompareExchangePointer)
#endif

#if !defined(_M_ARM64EC)
#if (_MSC_VER >= 1600)

#define InterlockedExchange8 _InterlockedExchange8
#define InterlockedExchangeNoFence8 InterlockedExchange8
#define InterlockedExchangeAcquire8 InterlockedExchange8

#define InterlockedExchange16 _InterlockedExchange16
#define InterlockedExchangeNoFence16 InterlockedExchange16
#define InterlockedExchangeAcquire16 InterlockedExchange16

CHAR
InterlockedExchange8 (
    _Inout_ _Interlocked_operand_ CHAR volatile *Target,
    _In_ CHAR Value
    );

SHORT
InterlockedExchange16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT ExChange
    );

#if !defined(_M_ARM64EC)
#pragma intrinsic(_InterlockedExchange8)
#pragma intrinsic(_InterlockedExchange16)
#endif

#endif /* _MSC_VER >= 1600 */

#if _MSC_FULL_VER >= 140041204

#define InterlockedExchangeAdd8 _InterlockedExchangeAdd8
#define InterlockedAnd8 _InterlockedAnd8
#define InterlockedAndAcquire8 _InterlockedAnd8
#define InterlockedAndRelease8 _InterlockedAnd8
#define InterlockedAndNoFence8 _InterlockedAnd8
#define InterlockedOr8 _InterlockedOr8
#define InterlockedOrAcquire8 _InterlockedOr8
#define InterlockedOrRelease8 _InterlockedOr8
#define InterlockedOrNoFence8 _InterlockedOr8
#define InterlockedXor8 _InterlockedXor8
#define InterlockedXorAcquire8 _InterlockedXor8
#define InterlockedXorRelease8 _InterlockedXor8
#define InterlockedXorNoFence8 _InterlockedXor8
#define InterlockedExchangeAdd16 _InterlockedExchangeAdd16
#define InterlockedAnd16 _InterlockedAnd16
#define InterlockedAndAcquire16 InterlockedAnd16
#define InterlockedAndRelease16 InterlockedAnd16
#define InterlockedAndNoFence16 InterlockedAnd16
#define InterlockedOr16 _InterlockedOr16
#define InterlockedOrAcquire16 InterlockedOr16
#define InterlockedOrRelease16 InterlockedOr16
#define InterlockedOrNoFence16 InterlockedOr16
#define InterlockedXor16 _InterlockedXor16
#define InterlockedXorAcquire16 InterlockedXor16
#define InterlockedXorRelease16 InterlockedXor16
#define InterlockedXorNoFence16 InterlockedXor16

char
InterlockedExchangeAdd8 (
    _Inout_ _Interlocked_operand_ char volatile * _Addend,
    _In_ char _Value
    );

char
InterlockedAnd8 (
    _Inout_ _Interlocked_operand_ char volatile *Destination,
    _In_ char Value
    );

char
InterlockedOr8 (
    _Inout_ _Interlocked_operand_ char volatile *Destination,
    _In_ char Value
    );

char
InterlockedXor8 (
    _Inout_ _Interlocked_operand_ char volatile *Destination,
    _In_ char Value
    );

SHORT
InterlockedAnd16(
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

SHORT
InterlockedOr16(
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

SHORT
InterlockedXor16(
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

#if !defined(_M_ARM64EC)
#pragma intrinsic (_InterlockedExchangeAdd8)
#pragma intrinsic (_InterlockedAnd8)
#pragma intrinsic (_InterlockedOr8)
#pragma intrinsic (_InterlockedXor8)
#pragma intrinsic (_InterlockedAnd16)
#pragma intrinsic (_InterlockedOr16)
#pragma intrinsic (_InterlockedXor16)
#endif

#endif
#endif

//
//

//
// Define extended CPUID intrinsic.
//

#if !defined(_M_ARM64EC)

#define CpuIdEx __cpuidex

#else

#undef __cpuidex
#define __cpuidex CPUIDEX64
#undef __cpuid
#define __cpuid __CpuId

#endif

VOID
#if defined(_M_ARM64EC)
__stdcall
#endif
__cpuidex (
    int CPUInfo[4],
    int Function,
    int SubLeaf
    );

#if !defined(_M_ARM64EC)

#pragma intrinsic(__cpuidex)

#else

__forceinline
VOID
__CpuId (
    int CPUInfo[4],
    int Function
    )
{
    __cpuidex(CPUInfo, Function, 0);
}

//
// TODO-ARM64X: Emulate __readmsr/__writemsr behavior accurately.
//

__declspec(noreturn) void __fastfail(_In_ unsigned int);

__forceinline
unsigned __int64
__readmsr(
    _In_ unsigned long Register
    )
{
    (Register); // reference to make compiler happy
    __fastfail(0);
    return 0;
}

__forceinline
void
__writemsr(
    _In_ unsigned long Register,
    _In_ unsigned __int64 Value
    )
{
    (Register); // reference to make compiler happy
    (Value);    // reference to make compiler happy
    __fastfail(0);
}

#endif // !defined(_M_ARM64EC)

//
//

#if !defined(_M_ARM64EC)

//
// Define function to flush a cache line.
//

#define CacheLineFlush(Address) _mm_clflush(Address)

VOID
_mm_clflush (
    _In_ VOID const *Address
    );

#pragma intrinsic(_mm_clflush)

#endif // !defined(_M_ARM64EC)

//
//

#if !defined(_M_ARM64EC)

VOID
_ReadWriteBarrier (
    VOID
    );

#pragma intrinsic(_ReadWriteBarrier)

//
// Define memory fence intrinsics  TODO-ARM64X: Re-express in terms of ARM64?
//

#define FastFence __faststorefence

#endif // !defined(_M_ARM64EC)

//
//

// TODO-ARM64X: Re-express in terms of ARM64?
#if !defined(_M_ARM64EC)

#define LoadFence _mm_lfence
#define MemoryFence _mm_mfence
#define StoreFence _mm_sfence
#define SpeculationFence LoadFence

#endif

//
//

// TODO-ARM64X: Re-express in terms of ARM64?
#if !defined(_M_ARM64EC)

VOID
__faststorefence (
    VOID
    );

#endif // !defined(_M_ARM64EC)

//
//

// TODO-ARM64X: Intrinsics
#if !defined(_M_ARM64EC)

VOID
_mm_lfence (
    VOID
    );

VOID
_mm_mfence (
    VOID
    );

VOID
_mm_sfence (
    VOID
    );

VOID
_mm_pause (
    VOID
    );

VOID
_mm_prefetch (
    _In_ CHAR CONST *a,
    _In_ int sel
    );

VOID
_m_prefetchw (
    _In_ volatile CONST VOID *Source
    );

#endif // !defined(_M_ARM64EC)

//
// Define constants for use with _mm_prefetch.
//

#define _MM_HINT_T0     1
#define _MM_HINT_T1     2
#define _MM_HINT_T2     3
#define _MM_HINT_NTA    0

//
//

// TODO-ARM64X: Intrisincs
#if !defined(_M_ARM64EC)

#pragma intrinsic(__faststorefence)

#endif // !defined(_M_ARM64EC)

//
//

// TODO-ARM64X: Intrisincs
#if !defined(_M_ARM64EC)

#pragma intrinsic(_mm_pause)
#pragma intrinsic(_mm_prefetch)
#pragma intrinsic(_mm_lfence)
#pragma intrinsic(_mm_mfence)
#pragma intrinsic(_mm_sfence)
#pragma intrinsic(_m_prefetchw)

#endif // !defined(_M_ARM64EC)

#if !defined(_M_ARM64EC)

#define YieldProcessor _mm_pause
#define MemoryBarrier __faststorefence
#define PreFetchCacheLine(l, a)  _mm_prefetch((CHAR CONST *) a, l)
#define PrefetchForWrite(p) _m_prefetchw(p)
#define ReadForWriteAccess(p) (_m_prefetchw(p), *(p))

//
// PreFetchCacheLine level defines.
//

#define PF_TEMPORAL_LEVEL_1 _MM_HINT_T0
#define PF_TEMPORAL_LEVEL_2 _MM_HINT_T1
#define PF_TEMPORAL_LEVEL_3 _MM_HINT_T2
#define PF_NON_TEMPORAL_LEVEL_ALL _MM_HINT_NTA

#endif // !defined(_M_ARM64EC)

//
// Define get/set MXCSR intrinsics.
//

#define ReadMxCsr _mm_getcsr
#define WriteMxCsr _mm_setcsr

unsigned int
_mm_getcsr (
    VOID
    );

VOID
_mm_setcsr (
    _In_ unsigned int MxCsr
    );

#if !defined(_M_ARM64EC)

#pragma intrinsic(_mm_getcsr)
#pragma intrinsic(_mm_setcsr)

#endif // !defined(_M_ARM64EC)

#if !defined(_M_ARM64EC)

//
// Define function to get the caller's EFLAGs value.
//

#define GetCallersEflags() __getcallerseflags()

unsigned __int32
__getcallerseflags (
    VOID
    );

#pragma intrinsic(__getcallerseflags)

//
// Define function to get segment limit.
//

#define GetSegmentLimit __segmentlimit

DWORD
__segmentlimit (
    _In_ DWORD Selector
    );

#pragma intrinsic(__segmentlimit)

//
// Define function to read the value of a performance counter.
//

#define ReadPMC __readpmc

DWORD64
__readpmc (
    _In_ DWORD Counter
    );

#pragma intrinsic(__readpmc)

//
// Define function to read the value of the time stamp counter
//

#define ReadTimeStampCounter() __rdtsc()

DWORD64
__rdtsc (
    VOID
    );

#pragma intrinsic(__rdtsc)

//
// Define functions to move strings as bytes, words, dwords, and qwords.
//

VOID
__movsb (
    _Out_writes_all_(Count) PBYTE  Destination,
    _In_reads_(Count) BYTE  const *Source,
    _In_ SIZE_T Count
    );

VOID
__movsw (
    _Out_writes_all_(Count) PWORD   Destination,
    _In_reads_(Count) WORD   const *Source,
    _In_ SIZE_T Count
    );

VOID
__movsd (
    _Out_writes_all_(Count) PDWORD Destination,
    _In_reads_(Count) DWORD const *Source,
    _In_ SIZE_T Count
    );

VOID
__movsq (
    _Out_writes_all_(Count) PDWORD64 Destination,
    _In_reads_(Count) DWORD64 const *Source,
    _In_ SIZE_T Count
    );

#pragma intrinsic(__movsb)
#pragma intrinsic(__movsw)
#pragma intrinsic(__movsd)
#pragma intrinsic(__movsq)

//
// Define functions to store strings as bytes, words, dwords, and qwords.
//

VOID
__stosb (
    _Out_writes_all_(Count) PBYTE  Destination,
    _In_ BYTE  Value,
    _In_ SIZE_T Count
    );

VOID
__stosw (
    _Out_writes_all_(Count) PWORD   Destination,
    _In_ WORD   Value,
    _In_ SIZE_T Count
    );

VOID
__stosd (
    _Out_writes_all_(Count) PDWORD Destination,
    _In_ DWORD Value,
    _In_ SIZE_T Count
    );

VOID
__stosq (
    _Out_writes_all_(Count) PDWORD64 Destination,
    _In_ DWORD64 Value,
    _In_ SIZE_T Count
    );

#pragma intrinsic(__stosb)
#pragma intrinsic(__stosw)
#pragma intrinsic(__stosd)
#pragma intrinsic(__stosq)

//
// Define functions to capture the high 64-bits of a 128-bit multiply.
//

#define MultiplyHigh __mulh
#define UnsignedMultiplyHigh __umulh

LONGLONG
MultiplyHigh (
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand
    );

ULONGLONG
UnsignedMultiplyHigh (
    _In_ DWORD64 Multiplier,
    _In_ DWORD64 Multiplicand
    );

#pragma intrinsic(__mulh)
#pragma intrinsic(__umulh)

#endif // !defined(_M_ARM64EC)

//
// Define population count intrinsic.
//

#if !defined(_M_ARM64EC)

#define PopulationCount64 __popcnt64

#else

#undef __popcnt64
#define __popcnt64 PopulationCount64

#endif // !defined(_M_ARM64EC)

DWORD64
PopulationCount64 (
    _In_ DWORD64 operand
    );

#if _MSC_VER >= 1500
#if !defined(_M_ARM64EC)

#pragma intrinsic(__popcnt64)

#endif // !defined(_M_ARM64EC)
#endif // _MSC_VER >= 1500

//
// Define functions to perform 128-bit shifts
//

#if !defined(_M_ARM64EC)

#define ShiftLeft128 __shiftleft128
#define ShiftRight128 __shiftright128

#else

#define __shiftleft128   ShiftLeft128
#define __shiftright128  ShiftRight128

#endif // !defined(_M_ARM64EC)

DWORD64
ShiftLeft128 (
    _In_ DWORD64 LowPart,
    _In_ DWORD64 HighPart,
    _In_ BYTE  Shift
    );

DWORD64
ShiftRight128 (
    _In_ DWORD64 LowPart,
    _In_ DWORD64 HighPart,
    _In_ BYTE  Shift
    );

#if !defined(_M_ARM64EC)

#pragma intrinsic(__shiftleft128)
#pragma intrinsic(__shiftright128)

#endif // !defined(_M_ARM64EC)

//
// Define functions to perform 128-bit multiplies.
//

#if !defined(_M_ARM64EC)

#define Multiply128 _mul128

#else

#undef _mul128
#define _mul128 Multiply128

#endif // !defined(_M_ARM64EC)

LONG64
Multiply128 (
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand,
    _Out_ LONG64 *HighProduct
    );

#if !defined(_M_ARM64EC)

#pragma intrinsic(_mul128)

#endif // !defined(_M_ARM64EC)

#if !defined(UnsignedMultiply128)

DWORD64
UnsignedMultiply128 (
    _In_ DWORD64 Multiplier,
    _In_ DWORD64 Multiplicand,
    _Out_ DWORD64 *HighProduct
    );

#if !defined(_M_ARM64EC)

#define UnsignedMultiply128 _umul128

#else

#undef _umul128
#define _umul128 UnsignedMultiply128

#endif // !defined(_M_ARM64EC)

DWORD64
UnsignedMultiply128 (
    _In_ DWORD64 Multiplier,
    _In_ DWORD64 Multiplicand,
    _Out_ DWORD64 *HighProduct
    );

LONG64
Multiply128 (
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand,
    _Out_ LONG64 *HighProduct
    );

#if !defined(_M_ARM64EC)

#pragma intrinsic(_umul128)

#endif // !defined(_M_ARM64EC)

#endif // !defined(UnsignedMultiply128)

__forceinline
LONG64
MultiplyExtract128 (
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand,
    _In_ BYTE  Shift
    )

{

    LONG64 extractedProduct;
    LONG64 highProduct;
    LONG64 lowProduct;
    BOOLEAN negate;
    DWORD64 uhighProduct;
    DWORD64 ulowProduct;

    lowProduct = Multiply128(Multiplier, Multiplicand, &highProduct);
    negate = FALSE;
    uhighProduct = (DWORD64)highProduct;
    ulowProduct = (DWORD64)lowProduct;
    if (highProduct < 0) {
        negate = TRUE;
        uhighProduct = (DWORD64)(-highProduct);
        ulowProduct = (DWORD64)(-lowProduct);
        if (ulowProduct != 0) {
            uhighProduct -= 1;
        }
    }

    extractedProduct = (LONG64)ShiftRight128(ulowProduct, uhighProduct, Shift);
    if (negate != FALSE) {
        extractedProduct = -extractedProduct;
    }

    return extractedProduct;
}

__forceinline
DWORD64
UnsignedMultiplyExtract128 (
    _In_ DWORD64 Multiplier,
    _In_ DWORD64 Multiplicand,
    _In_ BYTE  Shift
    )

{

    DWORD64 extractedProduct;
    DWORD64 highProduct;
    DWORD64 lowProduct;

    lowProduct = UnsignedMultiply128(Multiplier, Multiplicand, &highProduct);
    extractedProduct = ShiftRight128(lowProduct, highProduct, Shift);
    return extractedProduct;
}

#if !defined(_M_ARM64EC)

//
// Define functions to read and write the user TEB and the system PCR/PRCB.
//

BYTE 
__readgsbyte (
    _In_ DWORD Offset
    );

WORD  
__readgsword (
    _In_ DWORD Offset
    );

DWORD
__readgsdword (
    _In_ DWORD Offset
    );

DWORD64
__readgsqword (
    _In_ DWORD Offset
    );

VOID
__writegsbyte (
    _In_ DWORD Offset,
    _In_ BYTE  Data
    );

VOID
__writegsword (
    _In_ DWORD Offset,
    _In_ WORD   Data
    );

VOID
__writegsdword (
    _In_ DWORD Offset,
    _In_ DWORD Data
    );

VOID
__writegsqword (
    _In_ DWORD Offset,
    _In_ DWORD64 Data
    );

#pragma intrinsic(__readgsbyte)
#pragma intrinsic(__readgsword)
#pragma intrinsic(__readgsdword)
#pragma intrinsic(__readgsqword)
#pragma intrinsic(__writegsbyte)
#pragma intrinsic(__writegsword)
#pragma intrinsic(__writegsdword)
#pragma intrinsic(__writegsqword)

#if !defined(_MANAGED)

VOID
__incgsbyte (
    _In_ DWORD Offset
    );

VOID
__addgsbyte (
    _In_ DWORD Offset,
    _In_ BYTE  Value
    );

VOID
__incgsword (
    _In_ DWORD Offset
    );

VOID
__addgsword (
    _In_ DWORD Offset,
    _In_ WORD   Value
    );

VOID
__incgsdword (
    _In_ DWORD Offset
    );

VOID
__addgsdword (
    _In_ DWORD Offset,
    _In_ DWORD Value
    );

VOID
__incgsqword (
    _In_ DWORD Offset
    );

VOID
__addgsqword (
    _In_ DWORD Offset,
    _In_ DWORD64 Value
    );

#if 0
#pragma intrinsic(__incgsbyte)
#pragma intrinsic(__addgsbyte)
#pragma intrinsic(__incgsword)
#pragma intrinsic(__addgsword)
#pragma intrinsic(__incgsdword)
#pragma intrinsic(__addgsdword)
#pragma intrinsic(__incgsqword)
#pragma intrinsic(__addgsqword)
#endif

#endif // !defined(_MANAGED)

#endif // !defined(_M_ARM64EC)

//
//

#ifdef __cplusplus
}
#endif

#endif // defined(_M_AMD64) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

//
//

//
// The following values specify the type of access in the first parameter
// of the exception record whan the exception code specifies an access
// violation.
//

#if !defined(_ARM64EC_)

#define EXCEPTION_READ_FAULT 0          // exception caused by a read
#define EXCEPTION_WRITE_FAULT 1         // exception caused by a write
#define EXCEPTION_EXECUTE_FAULT 8       // exception caused by an instruction fetch

#endif // !defined(_ARM64EC_)

//
// The following flags control the contents of the CONTEXT structure.
//

#if !defined(RC_INVOKED)

#define CONTEXT_AMD64   0x00100000L

#define CONTEXT_CONTROL         (CONTEXT_AMD64 | 0x00000001L)
#define CONTEXT_INTEGER         (CONTEXT_AMD64 | 0x00000002L)
#define CONTEXT_SEGMENTS        (CONTEXT_AMD64 | 0x00000004L)
#define CONTEXT_FLOATING_POINT  (CONTEXT_AMD64 | 0x00000008L)
#define CONTEXT_DEBUG_REGISTERS (CONTEXT_AMD64 | 0x00000010L)

#define CONTEXT_FULL            (CONTEXT_CONTROL | CONTEXT_INTEGER | \
                                 CONTEXT_FLOATING_POINT)

#define CONTEXT_ALL             (CONTEXT_CONTROL | CONTEXT_INTEGER | \
                                 CONTEXT_SEGMENTS | CONTEXT_FLOATING_POINT | \
                                 CONTEXT_DEBUG_REGISTERS)

#define CONTEXT_XSTATE          (CONTEXT_AMD64 | 0x00000040L)
#define CONTEXT_KERNEL_CET      (CONTEXT_AMD64 | 0x00000080L)

#if defined(XBOX_SYSTEMOS)

#define CONTEXT_KERNEL_DEBUGGER     0x04000000L

#endif

#define CONTEXT_EXCEPTION_ACTIVE    0x08000000L
#define CONTEXT_SERVICE_ACTIVE      0x10000000L
#define CONTEXT_EXCEPTION_REQUEST   0x40000000L
#define CONTEXT_EXCEPTION_REPORTING 0x80000000L

//
// CONTEXT_UNWOUND_TO_CALL flag is set by the unwinder if it
// has unwound to a call site, and cleared whenever it unwinds
// through a trap frame.
//

#define CONTEXT_UNWOUND_TO_CALL     0x20000000

#endif // !defined(RC_INVOKED)

//
// Define initial MxCsr and FpCsr control.
//

#define INITIAL_MXCSR 0x1f80            // initial MXCSR value
#define INITIAL_FPCSR 0x027f            // initial FPCSR value

//
//

typedef XSAVE_FORMAT XMM_SAVE_AREA32, *PXMM_SAVE_AREA32;

//
//

//
// Context Frame
//
//  This frame has a several purposes: 1) it is used as an argument to
//  NtContinue, 2) it is used to constuct a call frame for APC delivery,
//  and 3) it is used in the user level thread creation routines.
//
//
// The flags field within this record controls the contents of a CONTEXT
// record.
//
// If the context record is used as an input parameter, then for each
// portion of the context record controlled by a flag whose value is
// set, it is assumed that that portion of the context record contains
// valid context. If the context record is being used to modify a threads
// context, then only that portion of the threads context is modified.
//
// If the context record is used as an output parameter to capture the
// context of a thread, then only those portions of the thread's context
// corresponding to set flags will be returned.
//
// CONTEXT_CONTROL specifies SegSs, Rsp, SegCs, Rip, and EFlags.
//
// CONTEXT_INTEGER specifies Rax, Rcx, Rdx, Rbx, Rbp, Rsi, Rdi, and R8-R15.
//
// CONTEXT_SEGMENTS specifies SegDs, SegEs, SegFs, and SegGs.
//
// CONTEXT_FLOATING_POINT specifies Xmm0-Xmm15.
//
// CONTEXT_DEBUG_REGISTERS specifies Dr0-Dr3 and Dr6-Dr7.
//

typedef struct DECLSPEC_ALIGN(16) DECLSPEC_NOINITALL _CONTEXT {

    //
    // Register parameter home addresses.
    //
    // N.B. These fields are for convience - they could be used to extend the
    //      context record in the future.
    //

    DWORD64 P1Home;
    DWORD64 P2Home;
    DWORD64 P3Home;
    DWORD64 P4Home;
    DWORD64 P5Home;
    DWORD64 P6Home;

    //
    // Control flags.
    //

    DWORD ContextFlags;
    DWORD MxCsr;

    //
    // Segment Registers and processor flags.
    //

    WORD   SegCs;
    WORD   SegDs;
    WORD   SegEs;
    WORD   SegFs;
    WORD   SegGs;
    WORD   SegSs;
    DWORD EFlags;

    //
    // Debug registers
    //

    DWORD64 Dr0;
    DWORD64 Dr1;
    DWORD64 Dr2;
    DWORD64 Dr3;
    DWORD64 Dr6;
    DWORD64 Dr7;

    //
    // Integer registers.
    //

    DWORD64 Rax;
    DWORD64 Rcx;
    DWORD64 Rdx;
    DWORD64 Rbx;
    DWORD64 Rsp;
    DWORD64 Rbp;
    DWORD64 Rsi;
    DWORD64 Rdi;
    DWORD64 R8;
    DWORD64 R9;
    DWORD64 R10;
    DWORD64 R11;
    DWORD64 R12;
    DWORD64 R13;
    DWORD64 R14;
    DWORD64 R15;

    //
    // Program counter.
    //

    DWORD64 Rip;

    //
    // Floating point state.
    //

    union {
        XMM_SAVE_AREA32 FltSave;
        struct {
            M128A Header[2];
            M128A Legacy[8];
            M128A Xmm0;
            M128A Xmm1;
            M128A Xmm2;
            M128A Xmm3;
            M128A Xmm4;
            M128A Xmm5;
            M128A Xmm6;
            M128A Xmm7;
            M128A Xmm8;
            M128A Xmm9;
            M128A Xmm10;
            M128A Xmm11;
            M128A Xmm12;
            M128A Xmm13;
            M128A Xmm14;
            M128A Xmm15;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    //
    // Vector registers.
    //

    M128A VectorRegister[26];
    DWORD64 VectorControl;

    //
    // Special debug control registers.
    //

    DWORD64 DebugControl;
    DWORD64 LastBranchToRip;
    DWORD64 LastBranchFromRip;
    DWORD64 LastExceptionToRip;
    DWORD64 LastExceptionFromRip;
} CONTEXT, *PCONTEXT;

//
//

//
// Select platform-specific definitions
//

typedef struct _IMAGE_RUNTIME_FUNCTION_ENTRY RUNTIME_FUNCTION, *PRUNTIME_FUNCTION;
typedef SCOPE_TABLE_AMD64 SCOPE_TABLE, *PSCOPE_TABLE;

#define RUNTIME_FUNCTION_INDIRECT 0x1

//
// Define unwind information flags.
//

#define UNW_FLAG_NHANDLER       0x0
#define UNW_FLAG_EHANDLER       0x1
#define UNW_FLAG_UHANDLER       0x2
#define UNW_FLAG_CHAININFO      0x4

#define UNW_FLAG_NO_EPILOGUE    0x80000000UL    // Software only flag

#define UNWIND_CHAIN_LIMIT      32

//
// Define dynamic function table entry.
//

typedef
_Function_class_(GET_RUNTIME_FUNCTION_CALLBACK)
PRUNTIME_FUNCTION
GET_RUNTIME_FUNCTION_CALLBACK (
    _In_ DWORD64 ControlPc,
    _In_opt_ PVOID Context
    );
typedef GET_RUNTIME_FUNCTION_CALLBACK *PGET_RUNTIME_FUNCTION_CALLBACK;

typedef
_Function_class_(OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK)
DWORD   
OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK (
    _In_ HANDLE Process,
    _In_ PVOID TableAddress,
    _Out_ PDWORD Entries,
    _Out_ PRUNTIME_FUNCTION* Functions
    );
typedef OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK *POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK;

#define OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME \
    "OutOfProcessFunctionTableCallback"

//
// Define exception dispatch context structure.
//

typedef struct _DISPATCHER_CONTEXT {
    DWORD64 ControlPc;
    DWORD64 ImageBase;
    PRUNTIME_FUNCTION FunctionEntry;
    DWORD64 EstablisherFrame;
    DWORD64 TargetIp;
    PCONTEXT ContextRecord;
    PEXCEPTION_ROUTINE LanguageHandler;
    PVOID HandlerData;
    struct _UNWIND_HISTORY_TABLE *HistoryTable;
    DWORD ScopeIndex;
    DWORD Fill0;
} DISPATCHER_CONTEXT, *PDISPATCHER_CONTEXT;

#if defined(_ARM64EC_)

typedef struct _DISPATCHER_CONTEXT_ARM64EC {
    DWORD64 ControlPc;
    DWORD64 ImageBase;
    PRUNTIME_FUNCTION FunctionEntry;
    DWORD64 EstablisherFrame;
    union {
        DWORD64 TargetIp;
        DWORD64 TargetPc;
    } DUMMYUNIONNAME;
    PCONTEXT ContextRecord;
    PEXCEPTION_ROUTINE LanguageHandler;
    PVOID HandlerData;
    struct _UNWIND_HISTORY_TABLE *HistoryTable;
    DWORD ScopeIndex;
    BOOLEAN ControlPcIsUnwound;
    PBYTE  NonVolatileRegisters;
} DISPATCHER_CONTEXT_ARM64EC, *PDISPATCHER_CONTEXT_ARM64EC;

#endif // defined(_ARM64EC_)

//
// Define exception filter and termination handler function types.
//

struct _EXCEPTION_POINTERS;
typedef
LONG
(*PEXCEPTION_FILTER) (
    struct _EXCEPTION_POINTERS *ExceptionPointers,
    PVOID EstablisherFrame
    );

typedef
VOID
(*PTERMINATION_HANDLER) (
    BOOLEAN AbnormalTermination,
    PVOID EstablisherFrame
    );

//
// Nonvolatile context pointer record.
//

typedef struct _KNONVOLATILE_CONTEXT_POINTERS {
    union {
        PM128A FloatingContext[16];
        struct {
            PM128A Xmm0;
            PM128A Xmm1;
            PM128A Xmm2;
            PM128A Xmm3;
            PM128A Xmm4;
            PM128A Xmm5;
            PM128A Xmm6;
            PM128A Xmm7;
            PM128A Xmm8;
            PM128A Xmm9;
            PM128A Xmm10;
            PM128A Xmm11;
            PM128A Xmm12;
            PM128A Xmm13;
            PM128A Xmm14;
            PM128A Xmm15;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    union {
        PDWORD64 IntegerContext[16];
        struct {
            PDWORD64 Rax;
            PDWORD64 Rcx;
            PDWORD64 Rdx;
            PDWORD64 Rbx;
            PDWORD64 Rsp;
            PDWORD64 Rbp;
            PDWORD64 Rsi;
            PDWORD64 Rdi;
            PDWORD64 R8;
            PDWORD64 R9;
            PDWORD64 R10;
            PDWORD64 R11;
            PDWORD64 R12;
            PDWORD64 R13;
            PDWORD64 R14;
            PDWORD64 R15;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME2;

} KNONVOLATILE_CONTEXT_POINTERS, *PKNONVOLATILE_CONTEXT_POINTERS;

//
//

#endif // defined(_AMD64_)

//
//
// Scope table structure definition.
//

typedef struct _SCOPE_TABLE_ARM {
    DWORD Count;
    struct
    {
        DWORD BeginAddress;
        DWORD EndAddress;
        DWORD HandlerAddress;
        DWORD JumpTarget;
    } ScopeRecord[1];
} SCOPE_TABLE_ARM, *PSCOPE_TABLE_ARM;


#ifdef _ARM_


#if defined(_M_ARM) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

#include <intrin.h>

#if !defined(_M_CEE_PURE)

#ifdef __cplusplus
extern "C" {
#endif

//
// Memory barriers and prefetch intrinsics.
//

#pragma intrinsic(__yield)
#pragma intrinsic(__prefetch)
#pragma intrinsic(__prefetchw)

#if (_MSC_FULL_VER >= 170040825)
#pragma intrinsic(__dmb)
#pragma intrinsic(__dsb)
#pragma intrinsic(__isb)
#else
#define __dmb(x) { __emit(0xF3BF); __emit(0x8F5F); }
#define __dsb(x) { __emit(0xF3BF); __emit(0x8F4F); }
#define __isb(x) { __emit(0xF3BF); __emit(0x8F6F); }
#endif

#pragma intrinsic(_ReadWriteBarrier)
#pragma intrinsic(_WriteBarrier)

#define SpeculationFence() NOP_FUNCTION

FORCEINLINE
VOID
YieldProcessor (
    VOID
    )
{
    __dmb(_ARM_BARRIER_ISHST);
    __yield();
}

#define MemoryBarrier()             __dmb(_ARM_BARRIER_SY)
#define PreFetchCacheLine(l,a)      __prefetch((const void *) (a))
#define PrefetchForWrite(p)         __prefetchw((const void *) (p))
#define ReadForWriteAccess(p)       (__prefetchw((const void *) (p)), *(p))

#define _DataSynchronizationBarrier()        __dsb(_ARM_BARRIER_SY)
#define _InstructionSynchronizationBarrier() __isb(_ARM_BARRIER_SY)

//
// Define bit test intrinsics.
//

#define BitTest _bittest
#define BitTestAndComplement _bittestandcomplement
#define BitTestAndSet _bittestandset
#define BitTestAndReset _bittestandreset
#define InterlockedBitTestAndSet _interlockedbittestandset
#define InterlockedBitTestAndSetAcquire _interlockedbittestandset_acq
#define InterlockedBitTestAndSetRelease _interlockedbittestandset_rel
#define InterlockedBitTestAndSetNoFence _interlockedbittestandset_nf
#define InterlockedBitTestAndReset _interlockedbittestandreset
#define InterlockedBitTestAndResetAcquire _interlockedbittestandreset_acq
#define InterlockedBitTestAndResetRelease _interlockedbittestandreset_rel
#define InterlockedBitTestAndResetNoFence _interlockedbittestandreset_nf

#pragma intrinsic(_bittest)
#pragma intrinsic(_bittestandcomplement)
#pragma intrinsic(_bittestandset)
#pragma intrinsic(_bittestandreset)

//
// Define bit scan functions
//

#define BitScanForward _BitScanForward
#define BitScanReverse _BitScanReverse

#pragma intrinsic(_BitScanForward)
#pragma intrinsic(_BitScanReverse)

_Success_(return != 0)
FORCEINLINE
BOOLEAN
_InlineBitScanReverse64 (
    _Out_ DWORD *Index,
    _In_ DWORD64 Mask
    )
{
    if (_BitScanReverse(Index, (DWORD)(Mask >> 32))) {
        *Index += 32;
        return 1;
    }

    if (_BitScanReverse(Index, (DWORD)Mask)) {
        return 1;
    }

    return 0;
}

#define BitScanReverse64 _InlineBitScanReverse64

#define InterlockedAnd8 _InterlockedAnd8
#define InterlockedOr8 _InterlockedOr8
#define InterlockedXor8 _InterlockedXor8
#define InterlockedExchange8 _InterlockedExchange8
#define InterlockedExchangeAdd8 _InterlockedExchangeAdd8

#define InterlockedAnd16 _InterlockedAnd16
#define InterlockedOr16 _InterlockedOr16
#define InterlockedXor16 _InterlockedXor16
#define InterlockedIncrement16 _InterlockedIncrement16
#define InterlockedDecrement16 _InterlockedDecrement16
#define InterlockedExchangeAdd16 _InterlockedExchangeAdd16
#define InterlockedCompareExchange16 _InterlockedCompareExchange16

#define InterlockedAnd _InterlockedAnd
#define InterlockedOr _InterlockedOr
#define InterlockedXor _InterlockedXor
#define InterlockedIncrement _InterlockedIncrement
#define InterlockedDecrement _InterlockedDecrement
#define InterlockedAdd _InterlockedAdd
#define InterlockedExchange _InterlockedExchange
#define InterlockedExchangeAdd _InterlockedExchangeAdd
#define InterlockedCompareExchange _InterlockedCompareExchange

#define InterlockedAnd64 _InterlockedAnd64
#define InterlockedAndAffinity InterlockedAnd64
#define InterlockedOr64 _InterlockedOr64
#define InterlockedOrAffinity InterlockedOr64
#define InterlockedXor64 _InterlockedXor64
#define InterlockedIncrement64 _InterlockedIncrement64
#define InterlockedDecrement64 _InterlockedDecrement64
#define InterlockedAdd64 _InterlockedAdd64
#define InterlockedExchange64 _InterlockedExchange64
#define InterlockedExchangeAdd64 _InterlockedExchangeAdd64
#define InterlockedCompareExchange64 _InterlockedCompareExchange64

#define InterlockedExchangePointer _InterlockedExchangePointer
#define InterlockedCompareExchangePointer _InterlockedCompareExchangePointer

#define InterlockedExchange16 _InterlockedExchange16

#define InterlockedAndAcquire8 _InterlockedAnd8_acq
#define InterlockedAndRelease8 _InterlockedAnd8_rel
#define InterlockedAndNoFence8 _InterlockedAnd8_nf
#define InterlockedOrAcquire8 _InterlockedOr8_acq
#define InterlockedOrRelease8 _InterlockedOr8_rel
#define InterlockedOrNoFence8 _InterlockedOr8_nf
#define InterlockedXorAcquire8 _InterlockedXor8_acq
#define InterlockedXorRelease8 _InterlockedXor8_rel
#define InterlockedXorNoFence8 _InterlockedXor8_nf
#define InterlockedExchangeNoFence8 _InterlockedExchange8_nf
#define InterlockedExchangeAcquire8 _InterlockedExchange8_acq

#define InterlockedAndAcquire16 _InterlockedAnd16_acq
#define InterlockedAndRelease16 _InterlockedAnd16_rel
#define InterlockedAndNoFence16 _InterlockedAnd16_nf
#define InterlockedOrAcquire16 _InterlockedOr16_acq
#define InterlockedOrRelease16 _InterlockedOr16_rel
#define InterlockedOrNoFence16 _InterlockedOr16_nf
#define InterlockedXorAcquire16 _InterlockedXor16_acq
#define InterlockedXorRelease16 _InterlockedXor16_rel
#define InterlockedXorNoFence16 _InterlockedXor16_nf
#define InterlockedIncrementAcquire16 _InterlockedIncrement16_acq
#define InterlockedIncrementRelease16 _InterlockedIncrement16_rel
#define InterlockedIncrementNoFence16 _InterlockedIncrement16_nf
#define InterlockedDecrementAcquire16 _InterlockedDecrement16_acq
#define InterlockedDecrementRelease16 _InterlockedDecrement16_rel
#define InterlockedDecrementNoFence16 _InterlockedDecrement16_nf
#define InterlockedExchangeAcquire16 _InterlockedExchange16_acq
#define InterlockedExchangeNoFence16 _InterlockedExchange16_nf
#define InterlockedCompareExchangeAcquire16 _InterlockedCompareExchange16_acq
#define InterlockedCompareExchangeRelease16 _InterlockedCompareExchange16_rel
#define InterlockedCompareExchangeNoFence16 _InterlockedCompareExchange16_nf

#define InterlockedAndAcquire _InterlockedAnd_acq
#define InterlockedAndRelease _InterlockedAnd_rel
#define InterlockedAndNoFence _InterlockedAnd_nf
#define InterlockedOrAcquire _InterlockedOr_acq
#define InterlockedOrRelease _InterlockedOr_rel
#define InterlockedOrNoFence _InterlockedOr_nf
#define InterlockedXorAcquire _InterlockedXor_acq
#define InterlockedXorRelease _InterlockedXor_rel
#define InterlockedXorNoFence _InterlockedXor_nf
#define InterlockedIncrementAcquire _InterlockedIncrement_acq
#define InterlockedIncrementRelease _InterlockedIncrement_rel
#define InterlockedIncrementNoFence _InterlockedIncrement_nf
#define InterlockedDecrementAcquire _InterlockedDecrement_acq
#define InterlockedDecrementRelease _InterlockedDecrement_rel
#define InterlockedDecrementNoFence _InterlockedDecrement_nf
#define InterlockedAddAcquire _InterlockedAdd_acq
#define InterlockedAddRelease _InterlockedAdd_rel
#define InterlockedAddNoFence _InterlockedAdd_nf
#define InterlockedExchangeAcquire _InterlockedExchange_acq
#define InterlockedExchangeNoFence _InterlockedExchange_nf
#define InterlockedExchangeAddAcquire _InterlockedExchangeAdd_acq
#define InterlockedExchangeAddRelease _InterlockedExchangeAdd_rel
#define InterlockedExchangeAddNoFence _InterlockedExchangeAdd_nf
#define InterlockedCompareExchangeAcquire _InterlockedCompareExchange_acq
#define InterlockedCompareExchangeRelease _InterlockedCompareExchange_rel
#define InterlockedCompareExchangeNoFence _InterlockedCompareExchange_nf

#define InterlockedAndAcquire64 _InterlockedAnd64_acq
#define InterlockedAndRelease64 _InterlockedAnd64_rel
#define InterlockedAndNoFence64 _InterlockedAnd64_nf
#define InterlockedOrAcquire64 _InterlockedOr64_acq
#define InterlockedOrRelease64 _InterlockedOr64_rel
#define InterlockedOrNoFence64 _InterlockedOr64_nf
#define InterlockedXorAcquire64 _InterlockedXor64_acq
#define InterlockedXorRelease64 _InterlockedXor64_rel
#define InterlockedXorNoFence64 _InterlockedXor64_nf
#define InterlockedIncrementAcquire64 _InterlockedIncrement64_acq
#define InterlockedIncrementRelease64 _InterlockedIncrement64_rel
#define InterlockedIncrementNoFence64 _InterlockedIncrement64_nf
#define InterlockedDecrementAcquire64 _InterlockedDecrement64_acq
#define InterlockedDecrementRelease64 _InterlockedDecrement64_rel
#define InterlockedDecrementNoFence64 _InterlockedDecrement64_nf
#define InterlockedAddAcquire64 _InterlockedAdd64_acq
#define InterlockedAddRelease64 _InterlockedAdd64_rel
#define InterlockedAddNoFence64 _InterlockedAdd64_nf
#define InterlockedExchangeAcquire64 _InterlockedExchange64_acq
#define InterlockedExchangeNoFence64 _InterlockedExchange64_nf
#define InterlockedExchangeAddAcquire64 _InterlockedExchangeAdd64_acq
#define InterlockedExchangeAddRelease64 _InterlockedExchangeAdd64_rel
#define InterlockedExchangeAddNoFence64 _InterlockedExchangeAdd64_nf
#define InterlockedCompareExchangeAcquire64 _InterlockedCompareExchange64_acq
#define InterlockedCompareExchangeRelease64 _InterlockedCompareExchange64_rel
#define InterlockedCompareExchangeNoFence64 _InterlockedCompareExchange64_nf

#define InterlockedExchangePointerAcquire _InterlockedExchangePointer_acq
#define InterlockedExchangePointerNoFence _InterlockedExchangePointer_nf
#define InterlockedCompareExchangePointerAcquire _InterlockedCompareExchangePointer_acq
#define InterlockedCompareExchangePointerRelease _InterlockedCompareExchangePointer_rel
#define InterlockedCompareExchangePointerNoFence _InterlockedCompareExchangePointer_nf

#define InterlockedExchangeAddSizeT(a, b) InterlockedExchangeAdd((LONG *)a, b)
#define InterlockedExchangeAddSizeTAcquire(a, b) InterlockedExchangeAddAcquire((LONG *)a, b)
#define InterlockedExchangeAddSizeTNoFence(a, b) InterlockedExchangeAddNoFence((LONG *)a, b)
#define InterlockedIncrementSizeT(a) InterlockedIncrement((LONG *)a)
#define InterlockedIncrementSizeTNoFence(a) InterlockedIncrementNoFence((LONG *)a)
#define InterlockedDecrementSizeT(a) InterlockedDecrement((LONG *)a)
#define InterlockedDecrementSizeTNoFence(a) InterlockedDecrementNoFence((LONG *)a)

//
// Define accessors for volatile loads and stores.
//

#pragma intrinsic(__iso_volatile_load8)
#pragma intrinsic(__iso_volatile_load16)
#pragma intrinsic(__iso_volatile_load32)
#pragma intrinsic(__iso_volatile_load64)
#pragma intrinsic(__iso_volatile_store8)
#pragma intrinsic(__iso_volatile_store16)
#pragma intrinsic(__iso_volatile_store32)
#pragma intrinsic(__iso_volatile_store64)

// end_wdm end_ntndis end_ntosp end_ntminiport end_ntoshvp
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// begin_wdm begin_ntndis begin_ntosp begin_ntminiport begin_ntoshvp

FORCEINLINE
CHAR
ReadAcquire8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

    Value = __iso_volatile_load8(Source);
    __dmb(_ARM_BARRIER_ISH);
    return Value;
}

FORCEINLINE
CHAR
ReadNoFence8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

    Value = __iso_volatile_load8(Source);
    return Value;
}

FORCEINLINE
VOID
WriteRelease8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

    __dmb(_ARM_BARRIER_ISH);
    __iso_volatile_store8(Destination, Value);
    return;
}

FORCEINLINE
VOID
WriteNoFence8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

    __iso_volatile_store8(Destination, Value);
    return;
}

FORCEINLINE
SHORT
ReadAcquire16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

    Value = __iso_volatile_load16(Source);
    __dmb(_ARM_BARRIER_ISH);
    return Value;
}

FORCEINLINE
SHORT
ReadNoFence16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

    Value = __iso_volatile_load16(Source);
    return Value;
}

FORCEINLINE
VOID
WriteRelease16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

    __dmb(_ARM_BARRIER_ISH);
    __iso_volatile_store16(Destination, Value);
    return;
}

FORCEINLINE
VOID
WriteNoFence16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

    __iso_volatile_store16(Destination, Value);
    return;
}

FORCEINLINE
LONG
ReadAcquire (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

    Value = __iso_volatile_load32((int *)Source);
    __dmb(_ARM_BARRIER_ISH);
    return Value;
}

FORCEINLINE
LONG
ReadNoFence (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

    Value = __iso_volatile_load32((int *)Source);
    return Value;
}

CFORCEINLINE
VOID
WriteRelease (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

    __dmb(_ARM_BARRIER_ISH);
    __iso_volatile_store32((int *)Destination, Value);
    return;
}

FORCEINLINE
VOID
WriteNoFence (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

    __iso_volatile_store32((int *)Destination, Value);
    return;
}

FORCEINLINE
LONG64
ReadAcquire64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

    Value = __iso_volatile_load64(Source);
    __dmb(_ARM_BARRIER_ISH);
    return Value;
}

FORCEINLINE
LONG64
ReadNoFence64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

    Value = __iso_volatile_load64(Source);
    return Value;
}

CFORCEINLINE
VOID
WriteRelease64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

    __dmb(_ARM_BARRIER_ISH);
    __iso_volatile_store64(Destination, Value);
    return;
}

FORCEINLINE
VOID
WriteNoFence64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

    __iso_volatile_store64(Destination, Value);
    return;
}

FORCEINLINE
VOID
BarrierAfterRead (
    VOID
    )

{
    __dmb(_ARM_BARRIER_ISH);
    return;
}

// end_wdm end_ntndis end_ntosp end_ntminiport end_ntoshvp
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// begin_wdm begin_ntndis begin_ntosp begin_ntminiport begin_ntoshvp

//
// Define coprocessor access intrinsics.  Coprocessor 15 contains
// registers for the MMU, cache, TLB, feature bits, core
// identification and performance counters.
//

#define CP15_PMSELR            15, 0,  9, 12, 5         // Event Counter Selection Register
#define CP15_PMXEVCNTR         15, 0,  9, 13, 2         // Event Count Register
#define CP15_TPIDRURW          15, 0, 13,  0, 2         // Software Thread ID Register, User Read/Write
#define CP15_TPIDRURO          15, 0, 13,  0, 3         // Software Thread ID Register, User Read Only
#define CP15_TPIDRPRW          15, 0, 13,  0, 4         // Software Thread ID Register, Privileged Only

#pragma intrinsic(_MoveToCoprocessor)
#pragma intrinsic(_MoveFromCoprocessor)

//
// Coprocessor registers for synchronization
//

#define _InvalidateBTAC() _MoveToCoprocessor(0, CP15_BPIALL)

//
// PreFetchCacheLine level defines.
//

#define PF_TEMPORAL_LEVEL_1         0
#define PF_TEMPORAL_LEVEL_2         1
#define PF_TEMPORAL_LEVEL_3         2
#define PF_NON_TEMPORAL_LEVEL_ALL   3

//
// Define function to read the value of the time stamp counter which
// ARM doesn't have.
//

// end_ntoshvp

DWORD64
ReadTimeStampCounter(
    VOID
    );

FORCEINLINE
DWORD64
ReadPMC (
    _In_ DWORD Counter
    )
{

    _MoveToCoprocessor(Counter, CP15_PMSELR);
    _DataSynchronizationBarrier();
    return (DWORD64)_MoveFromCoprocessor(CP15_PMXEVCNTR);
}

// begin_ntoshvp

#ifdef __cplusplus
}
#endif

#endif // !defined(_M_CEE_PURE)

#endif // defined(_M_ARM) && !defined(RC_INVOKED) && !defined(MIDL_PASS) && !defined(_M_CEE_PURE)

#if defined(_M_CEE_PURE)
FORCEINLINE
VOID
YieldProcessor (
    VOID
    )
{
}
#endif

//
// The following values specify the type of access in the first parameter
// of the exception record whan the exception code specifies an access
// violation.
//

#define EXCEPTION_READ_FAULT 0          // exception caused by a read
#define EXCEPTION_WRITE_FAULT 1         // exception caused by a write
#define EXCEPTION_EXECUTE_FAULT 8       // exception caused by an instruction fetch

// begin_wx86
//
// The following flags control the contents of the CONTEXT structure.
//

#if !defined(RC_INVOKED)

#define CONTEXT_ARM   0x00200000L

// end_wx86

#define CONTEXT_CONTROL (CONTEXT_ARM | 0x1L)
#define CONTEXT_INTEGER (CONTEXT_ARM | 0x2L)
#define CONTEXT_FLOATING_POINT  (CONTEXT_ARM | 0x4L)
#define CONTEXT_DEBUG_REGISTERS (CONTEXT_ARM | 0x8L)

#define CONTEXT_FULL (CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_FLOATING_POINT)

#define CONTEXT_ALL (CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_FLOATING_POINT | CONTEXT_DEBUG_REGISTERS)

#define CONTEXT_EXCEPTION_ACTIVE 0x8000000L
#define CONTEXT_SERVICE_ACTIVE 0x10000000L
#define CONTEXT_EXCEPTION_REQUEST 0x40000000L
#define CONTEXT_EXCEPTION_REPORTING 0x80000000L

//
// This flag is set by the unwinder if it has unwound to a call
// site, and cleared whenever it unwinds through a trap frame.
// It is used by language-specific exception handlers to help
// differentiate exception scopes during dispatching.
//

#define CONTEXT_UNWOUND_TO_CALL 0x20000000

// begin_wx86

#endif // !defined(RC_INVOKED)

//
// Define initial Cpsr/Fpscr value
//

#define INITIAL_CPSR 0x10
#define INITIAL_FPSCR 0

//
// Specify the number of breakpoints and watchpoints that the OS
// will track. Architecturally, ARM supports up to 16. In practice,
// however, almost no one implements more than 4 of each.
//

#define ARM_MAX_BREAKPOINTS     8
#define ARM_MAX_WATCHPOINTS     1

//
// Context Frame
//
//  This frame has a several purposes: 1) it is used as an argument to
//  NtContinue, 2) it is used to constuct a call frame for APC delivery,
//  and 3) it is used in the user level thread creation routines.
//
//
// The flags field within this record controls the contents of a CONTEXT
// record.
//
// If the context record is used as an input parameter, then for each
// portion of the context record controlled by a flag whose value is
// set, it is assumed that that portion of the context record contains
// valid context. If the context record is being used to modify a threads
// context, then only that portion of the threads context is modified.
//
// If the context record is used as an output parameter to capture the
// context of a thread, then only those portions of the thread's context
// corresponding to set flags will be returned.
//
// CONTEXT_CONTROL specifies Sp, Lr, Pc, and Cpsr
//
// CONTEXT_INTEGER specifies R0-R12
//
// CONTEXT_FLOATING_POINT specifies Q0-Q15 / D0-D31 / S0-S31
//
// CONTEXT_DEBUG_REGISTERS specifies up to 16 of DBGBVR, DBGBCR, DBGWVR,
//      DBGWCR.
//

typedef struct _NEON128 {
    ULONGLONG Low;
    LONGLONG High;
} NEON128, *PNEON128;

typedef struct DECLSPEC_ALIGN(8) DECLSPEC_NOINITALL _CONTEXT {

    //
    // Control flags.
    //

    DWORD ContextFlags;

    //
    // Integer registers
    //

    DWORD R0;
    DWORD R1;
    DWORD R2;
    DWORD R3;
    DWORD R4;
    DWORD R5;
    DWORD R6;
    DWORD R7;
    DWORD R8;
    DWORD R9;
    DWORD R10;
    DWORD R11;
    DWORD R12;

    //
    // Control Registers
    //

    DWORD Sp;
    DWORD Lr;
    DWORD Pc;
    DWORD Cpsr;

    //
    // Floating Point/NEON Registers
    //

    DWORD Fpscr;
    DWORD Padding;
    union {
        NEON128 Q[16];
        ULONGLONG D[32];
        DWORD S[32];
    } DUMMYUNIONNAME;

    //
    // Debug registers
    //

    DWORD Bvr[ARM_MAX_BREAKPOINTS];
    DWORD Bcr[ARM_MAX_BREAKPOINTS];
    DWORD Wvr[ARM_MAX_WATCHPOINTS];
    DWORD Wcr[ARM_MAX_WATCHPOINTS];

    DWORD Padding2[2];

} CONTEXT, *PCONTEXT;

//
// Select platform-specific definitions
//

typedef struct _IMAGE_ARM_RUNTIME_FUNCTION_ENTRY RUNTIME_FUNCTION, *PRUNTIME_FUNCTION;
typedef SCOPE_TABLE_ARM SCOPE_TABLE, *PSCOPE_TABLE;

//
// Define unwind information flags.
//

#define UNW_FLAG_NHANDLER               0x0             /* any handler */
#define UNW_FLAG_EHANDLER               0x1             /* filter handler */
#define UNW_FLAG_UHANDLER               0x2             /* unwind handler */

//
// Define exception dispatch context structure.
//

typedef struct _DISPATCHER_CONTEXT {
    DWORD ControlPc;
    DWORD ImageBase;
    PRUNTIME_FUNCTION FunctionEntry;
    DWORD EstablisherFrame;
    DWORD TargetPc;
    PCONTEXT ContextRecord;
    PEXCEPTION_ROUTINE LanguageHandler;
    PVOID HandlerData;
    struct _UNWIND_HISTORY_TABLE *HistoryTable;
    DWORD ScopeIndex;
    BOOLEAN ControlPcIsUnwound;
    PBYTE  NonVolatileRegisters;
    DWORD Reserved;
} DISPATCHER_CONTEXT, *PDISPATCHER_CONTEXT;

//
// Define exception filter and termination handler function types.
// N.B. These functions use a custom calling convention.
//

struct _EXCEPTION_POINTERS;
typedef
LONG
(*PEXCEPTION_FILTER) (
    struct _EXCEPTION_POINTERS *ExceptionPointers,
    DWORD EstablisherFrame
    );

typedef
VOID
(*PTERMINATION_HANDLER) (
    BOOLEAN AbnormalTermination,
    DWORD EstablisherFrame
    );

//
// Define dynamic function table entry.
//

typedef
_Function_class_(GET_RUNTIME_FUNCTION_CALLBACK)
PRUNTIME_FUNCTION
GET_RUNTIME_FUNCTION_CALLBACK (
    _In_ DWORD ControlPc,
    _In_opt_ PVOID Context
    );
typedef GET_RUNTIME_FUNCTION_CALLBACK *PGET_RUNTIME_FUNCTION_CALLBACK;

typedef
_Function_class_(OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK)
DWORD   
OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK (
    _In_ HANDLE Process,
    _In_ PVOID TableAddress,
    _Out_ PDWORD Entries,
    _Out_ PRUNTIME_FUNCTION* Functions
    );
typedef OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK *POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK;

#define OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME \
    "OutOfProcessFunctionTableCallback"


//
// Nonvolatile context pointer record.
//

typedef struct _KNONVOLATILE_CONTEXT_POINTERS {

    PDWORD R4;
    PDWORD R5;
    PDWORD R6;
    PDWORD R7;
    PDWORD R8;
    PDWORD R9;
    PDWORD R10;
    PDWORD R11;
    PDWORD Lr;

    PULONGLONG D8;
    PULONGLONG D9;
    PULONGLONG D10;
    PULONGLONG D11;
    PULONGLONG D12;
    PULONGLONG D13;
    PULONGLONG D14;
    PULONGLONG D15;

} KNONVOLATILE_CONTEXT_POINTERS, *PKNONVOLATILE_CONTEXT_POINTERS;

#endif // _ARM_

//

//
// Scope table structure definition.
//

typedef struct _SCOPE_TABLE_ARM64 {
    DWORD Count;
    struct
    {
        DWORD BeginAddress;
        DWORD EndAddress;
        DWORD HandlerAddress;
        DWORD JumpTarget;
    } ScopeRecord[1];
} SCOPE_TABLE_ARM64, *PSCOPE_TABLE_ARM64;

//
//

#if defined(_ARM64_) || defined(_CHPE_X86_ARM64_) || defined(_ARM64EC_)

//
//

#if !defined(_M_CEE_PURE)
#if !defined(RC_INVOKED) && !defined(MIDL_PASS)

#include <intrin.h>

#if defined(_M_ARM64) || defined(_M_ARM64EC)

#pragma intrinsic(__readx18byte)
#pragma intrinsic(__readx18word)
#pragma intrinsic(__readx18dword)
#pragma intrinsic(__readx18qword)

#pragma intrinsic(__writex18byte)
#pragma intrinsic(__writex18word)
#pragma intrinsic(__writex18dword)
#pragma intrinsic(__writex18qword)

#pragma intrinsic(__addx18byte)
#pragma intrinsic(__addx18word)
#pragma intrinsic(__addx18dword)
#pragma intrinsic(__addx18qword)

#pragma intrinsic(__incx18byte)
#pragma intrinsic(__incx18word)
#pragma intrinsic(__incx18dword)
#pragma intrinsic(__incx18qword)

//
// Define bit test intrinsics.
//

#define BitTest _bittest
#define BitTestAndComplement _bittestandcomplement
#define BitTestAndSet _bittestandset
#define BitTestAndReset _bittestandreset
#define InterlockedBitTestAndSet _interlockedbittestandset
#define InterlockedBitTestAndSetAcquire _interlockedbittestandset_acq
#define InterlockedBitTestAndSetRelease _interlockedbittestandset_rel
#define InterlockedBitTestAndSetNoFence _interlockedbittestandset_nf
#define InterlockedBitTestAndReset _interlockedbittestandreset
#define InterlockedBitTestAndResetAcquire _interlockedbittestandreset_acq
#define InterlockedBitTestAndResetRelease _interlockedbittestandreset_rel
#define InterlockedBitTestAndResetNoFence _interlockedbittestandreset_nf

//
// Temporary workaround for C++ bug: 64-bit bit test intrinsics are
// not honoring the full 64-bit wide index, so pre-process the index
// down to a qword base and a bit index 0-63 before calling through
// to the true intrinsic. This issue was fixed as of compiler build
// 19.28.29395.7.
//
#if defined(_MSC_FULL_VER) && (_MSC_FULL_VER < 192829395 || (_MSC_FULL_VER == 192829395 && _MSC_BUILD < 7))
#define __ARM64_COMPILER_BITTEST64_WORKAROUND
#endif

#if !defined(__ARM64_COMPILER_BITTEST64_WORKAROUND)
#define BitTest64 _bittest64
#define BitTestAndComplement64 _bittestandcomplement64
#define BitTestAndSet64 _bittestandset64
#define BitTestAndReset64 _bittestandreset64
#else
#undef BitTest64
#undef BitTestAndComplement64
#undef BitTestAndSet64
#undef BitTestAndReset64
FORCEINLINE
unsigned char
_BitTest64(__int64 const *Base, __int64 Index)
{
    return _bittest64(Base + (Index >> 6), Index & 63);
}

FORCEINLINE
unsigned char
_BitTestAndComplement64(__int64 *Base, __int64 Index)
{
    return _bittestandcomplement64(Base + (Index >> 6), Index & 63);
}

FORCEINLINE
unsigned char
_BitTestAndReset64(__int64 *Base, __int64 Index)
{
    return _bittestandreset64(Base + (Index >> 6), Index & 63);
}

FORCEINLINE
unsigned char
_BitTestAndSet64(__int64 *Base, __int64 Index)
{
    return _bittestandset64(Base + (Index >> 6), Index & 63);
}
#define BitTest64 _BitTest64
#define BitTestAndComplement64 _BitTestAndComplement64
#define BitTestAndSet64 _BitTestAndSet64
#define BitTestAndReset64 _BitTestAndReset64
#endif

//
// N.B. The above is not needed for the interlocked variants because they
//      are now generated as calls to glue code which already contain
//      fixes for this oversight.
//
#define InterlockedBitTestAndSet64 _interlockedbittestandset64
#define InterlockedBitTestAndSet64Acquire _interlockedbittestandset64_acq
#define InterlockedBitTestAndSet64Release _interlockedbittestandset64_rel
#define InterlockedBitTestAndSet64NoFence _interlockedbittestandset64_nf
#define InterlockedBitTestAndReset64 _interlockedbittestandreset64
#define InterlockedBitTestAndReset64Acquire _interlockedbittestandreset64_acq
#define InterlockedBitTestAndReset64Release _interlockedbittestandreset64_rel
#define InterlockedBitTestAndReset64NoFence _interlockedbittestandreset64_nf

#pragma intrinsic(_bittest)
#pragma intrinsic(_bittestandcomplement)
#pragma intrinsic(_bittestandset)
#pragma intrinsic(_bittestandreset)

#if !defined(__ARM64_COMPILER_BITTEST64_WORKAROUND)
#pragma intrinsic(_bittest64)
#pragma intrinsic(_bittestandcomplement64)
#pragma intrinsic(_bittestandset64)
#pragma intrinsic(_bittestandreset64)
#endif

//
// Define bit scan functions
//

#define BitScanForward _BitScanForward
#define BitScanReverse _BitScanReverse
#define BitScanForward64 _BitScanForward64
#define BitScanReverse64 _BitScanReverse64

#pragma intrinsic(_BitScanForward)
#pragma intrinsic(_BitScanReverse)
#pragma intrinsic(_BitScanForward64)
#pragma intrinsic(_BitScanReverse64)

#define InterlockedAnd8 _InterlockedAnd8
#define InterlockedOr8 _InterlockedOr8
#define InterlockedXor8 _InterlockedXor8
#define InterlockedExchangeAdd8 _InterlockedExchangeAdd8

#define InterlockedAnd16 _InterlockedAnd16
#define InterlockedOr16 _InterlockedOr16
#define InterlockedXor16 _InterlockedXor16
#define InterlockedIncrement16 _InterlockedIncrement16
#define InterlockedDecrement16 _InterlockedDecrement16
#define InterlockedExchangeAdd16 _InterlockedExchangeAdd16
#define InterlockedCompareExchange16 _InterlockedCompareExchange16

#define InterlockedAnd _InterlockedAnd
#define InterlockedOr _InterlockedOr
#define InterlockedXor _InterlockedXor
#define InterlockedIncrement _InterlockedIncrement
#define InterlockedDecrement _InterlockedDecrement
#define InterlockedAdd _InterlockedAdd
#define InterlockedExchange _InterlockedExchange
#define InterlockedExchangeAdd _InterlockedExchangeAdd
#define InterlockedCompareExchange _InterlockedCompareExchange

#define InterlockedAnd64 _InterlockedAnd64
#define InterlockedAndAffinity InterlockedAnd64
#define InterlockedOr64 _InterlockedOr64
#define InterlockedOrAffinity InterlockedOr64
#define InterlockedXor64 _InterlockedXor64
#define InterlockedIncrement64 _InterlockedIncrement64
#define InterlockedDecrement64 _InterlockedDecrement64
#define InterlockedAdd64 _InterlockedAdd64
#define InterlockedExchange64 _InterlockedExchange64
#define InterlockedExchangeAdd64 _InterlockedExchangeAdd64
#define InterlockedCompareExchange64 _InterlockedCompareExchange64

#define InterlockedExchangePointer _InterlockedExchangePointer
#define InterlockedCompareExchangePointer _InterlockedCompareExchangePointer

#define InterlockedExchange16 _InterlockedExchange16
#define InterlockedExchange8 _InterlockedExchange8

#define InterlockedAndAcquire8 _InterlockedAnd8_acq
#define InterlockedAndRelease8 _InterlockedAnd8_rel
#define InterlockedAndNoFence8 _InterlockedAnd8_nf
#define InterlockedOrAcquire8 _InterlockedOr8_acq
#define InterlockedOrRelease8 _InterlockedOr8_rel
#define InterlockedOrNoFence8 _InterlockedOr8_nf
#define InterlockedXorAcquire8 _InterlockedXor8_acq
#define InterlockedXorRelease8 _InterlockedXor8_rel
#define InterlockedXorNoFence8 _InterlockedXor8_nf
#define InterlockedExchangeNoFence8 _InterlockedExchange8_nf
#define InterlockedExchangeAcquire8 _InterlockedExchange8_acq

#define InterlockedAndAcquire16 _InterlockedAnd16_acq
#define InterlockedAndRelease16 _InterlockedAnd16_rel
#define InterlockedAndNoFence16 _InterlockedAnd16_nf
#define InterlockedOrAcquire16 _InterlockedOr16_acq
#define InterlockedOrRelease16 _InterlockedOr16_rel
#define InterlockedOrNoFence16 _InterlockedOr16_nf
#define InterlockedXorAcquire16 _InterlockedXor16_acq
#define InterlockedXorRelease16 _InterlockedXor16_rel
#define InterlockedXorNoFence16 _InterlockedXor16_nf
#define InterlockedIncrementAcquire16 _InterlockedIncrement16_acq
#define InterlockedIncrementRelease16 _InterlockedIncrement16_rel
#define InterlockedIncrementNoFence16 _InterlockedIncrement16_nf
#define InterlockedDecrementAcquire16 _InterlockedDecrement16_acq
#define InterlockedDecrementRelease16 _InterlockedDecrement16_rel
#define InterlockedDecrementNoFence16 _InterlockedDecrement16_nf
#define InterlockedExchangeAcquire16 _InterlockedExchange16_acq
#define InterlockedExchangeNoFence16 _InterlockedExchange16_nf
#define InterlockedCompareExchangeAcquire16 _InterlockedCompareExchange16_acq
#define InterlockedCompareExchangeRelease16 _InterlockedCompareExchange16_rel
#define InterlockedCompareExchangeNoFence16 _InterlockedCompareExchange16_nf

#define InterlockedAndAcquire _InterlockedAnd_acq
#define InterlockedAndRelease _InterlockedAnd_rel
#define InterlockedAndNoFence _InterlockedAnd_nf
#define InterlockedOrAcquire _InterlockedOr_acq
#define InterlockedOrRelease _InterlockedOr_rel
#define InterlockedOrNoFence _InterlockedOr_nf
#define InterlockedXorAcquire _InterlockedXor_acq
#define InterlockedXorRelease _InterlockedXor_rel
#define InterlockedXorNoFence _InterlockedXor_nf
#define InterlockedIncrementAcquire _InterlockedIncrement_acq
#define InterlockedIncrementRelease _InterlockedIncrement_rel
#define InterlockedIncrementNoFence _InterlockedIncrement_nf
#define InterlockedDecrementAcquire _InterlockedDecrement_acq
#define InterlockedDecrementRelease _InterlockedDecrement_rel
#define InterlockedDecrementNoFence _InterlockedDecrement_nf
#define InterlockedAddAcquire _InterlockedAdd_acq
#define InterlockedAddRelease _InterlockedAdd_rel
#define InterlockedAddNoFence _InterlockedAdd_nf
#define InterlockedExchangeAcquire _InterlockedExchange_acq
#define InterlockedExchangeNoFence _InterlockedExchange_nf
#define InterlockedExchangeAddAcquire _InterlockedExchangeAdd_acq
#define InterlockedExchangeAddRelease _InterlockedExchangeAdd_rel
#define InterlockedExchangeAddNoFence _InterlockedExchangeAdd_nf
#define InterlockedCompareExchangeAcquire _InterlockedCompareExchange_acq
#define InterlockedCompareExchangeRelease _InterlockedCompareExchange_rel
#define InterlockedCompareExchangeNoFence _InterlockedCompareExchange_nf

#define InterlockedAndAcquire64 _InterlockedAnd64_acq
#define InterlockedAndRelease64 _InterlockedAnd64_rel
#define InterlockedAndNoFence64 _InterlockedAnd64_nf
#define InterlockedOrAcquire64 _InterlockedOr64_acq
#define InterlockedOrRelease64 _InterlockedOr64_rel
#define InterlockedOrNoFence64 _InterlockedOr64_nf
#define InterlockedXorAcquire64 _InterlockedXor64_acq
#define InterlockedXorRelease64 _InterlockedXor64_rel
#define InterlockedXorNoFence64 _InterlockedXor64_nf
#define InterlockedIncrementAcquire64 _InterlockedIncrement64_acq
#define InterlockedIncrementRelease64 _InterlockedIncrement64_rel
#define InterlockedIncrementNoFence64 _InterlockedIncrement64_nf
#define InterlockedDecrementAcquire64 _InterlockedDecrement64_acq
#define InterlockedDecrementRelease64 _InterlockedDecrement64_rel
#define InterlockedDecrementNoFence64 _InterlockedDecrement64_nf
#define InterlockedAddAcquire64 _InterlockedAdd64_acq
#define InterlockedAddRelease64 _InterlockedAdd64_rel
#define InterlockedAddNoFence64 _InterlockedAdd64_nf
#define InterlockedExchangeAcquire64 _InterlockedExchange64_acq
#define InterlockedExchangeNoFence64 _InterlockedExchange64_nf
#define InterlockedExchangeAddAcquire64 _InterlockedExchangeAdd64_acq
#define InterlockedExchangeAddRelease64 _InterlockedExchangeAdd64_rel
#define InterlockedExchangeAddNoFence64 _InterlockedExchangeAdd64_nf
#define InterlockedCompareExchangeAcquire64 _InterlockedCompareExchange64_acq
#define InterlockedCompareExchangeRelease64 _InterlockedCompareExchange64_rel
#define InterlockedCompareExchangeNoFence64 _InterlockedCompareExchange64_nf
#define InterlockedCompareExchange128 _InterlockedCompareExchange128
#define InterlockedCompareExchangeAcquire128 _InterlockedCompareExchange128_acq
#define InterlockedCompareExchangeRelease128 _InterlockedCompareExchange128_rel
#define InterlockedCompareExchangeNoFence128 _InterlockedCompareExchange128_nf

// AMD64_WORKITEM : these are redundant but necessary for AMD64 compatibility
#define InterlockedAnd64Acquire _InterlockedAnd64_acq
#define InterlockedAnd64Release _InterlockedAnd64_rel
#define InterlockedAnd64NoFence _InterlockedAnd64_nf
#define InterlockedOr64Acquire _InterlockedOr64_acq
#define InterlockedOr64Release _InterlockedOr64_rel
#define InterlockedOr64NoFence _InterlockedOr64_nf
#define InterlockedXor64Acquire _InterlockedXor64_acq
#define InterlockedXor64Release _InterlockedXor64_rel
#define InterlockedXor64NoFence _InterlockedXor64_nf

#define InterlockedExchangePointerAcquire _InterlockedExchangePointer_acq
#define InterlockedExchangePointerNoFence _InterlockedExchangePointer_nf
#define InterlockedCompareExchangePointerAcquire _InterlockedCompareExchangePointer_acq
#define InterlockedCompareExchangePointerRelease _InterlockedCompareExchangePointer_rel
#define InterlockedCompareExchangePointerNoFence _InterlockedCompareExchangePointer_nf

#define InterlockedExchangeAddSizeT(a, b) InterlockedExchangeAdd64((LONG64 *)a, b)
#define InterlockedExchangeAddSizeTAcquire(a, b) InterlockedExchangeAddAcquire64((LONG64 *)a, b)
#define InterlockedExchangeAddSizeTNoFence(a, b) InterlockedExchangeAddNoFence64((LONG64 *)a, b)
#define InterlockedIncrementSizeT(a) InterlockedIncrement64((LONG64 *)a)
#define InterlockedIncrementSizeTNoFence(a) InterlockedIncrementNoFence64((LONG64 *)a)
#define InterlockedDecrementSizeT(a) InterlockedDecrement64((LONG64 *)a)
#define InterlockedDecrementSizeTNoFence(a) InterlockedDecrementNoFence64((LONG64 *)a)

#endif // defined(_M_ARM64) || defined(_M_ARM64EC)

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)

#pragma intrinsic(__getReg)
#pragma intrinsic(__getCallerReg)
#pragma intrinsic(__getRegFp)
#pragma intrinsic(__getCallerRegFp)

#pragma intrinsic(__setReg)
#pragma intrinsic(__setCallerReg)
#pragma intrinsic(__setRegFp)
#pragma intrinsic(__setCallerRegFp)

//
// Memory barriers and prefetch intrinsics.
//

#pragma intrinsic(__yield)
#pragma intrinsic(__prefetch)
#pragma intrinsic(__prefetch2)

#define ARM64_PREFETCH_PLD  (0 << 3)
#define ARM64_PREFETCH_PLI  (1 << 3)
#define ARM64_PREFETCH_PST  (2 << 3)

#define ARM64_PREFETCH_L1   (0 << 1)
#define ARM64_PREFETCH_L2   (1 << 1)
#define ARM64_PREFETCH_L3   (2 << 1)

#define ARM64_PREFETCH_KEEP (0 << 0)
#define ARM64_PREFETCH_STRM (1 << 0)

#define ARM64_PREFETCH(a,b,c) (ARM64_PREFETCH_##a | ARM64_PREFETCH_##b | ARM64_PREFETCH_##c)

#pragma intrinsic(__dmb)
#pragma intrinsic(__dsb)
#pragma intrinsic(__isb)

#pragma intrinsic(_ReadWriteBarrier)
#pragma intrinsic(_WriteBarrier)

#define SpeculationFence() NOP_FUNCTION

FORCEINLINE
VOID
MemoryBarrier (
    VOID
    )
{
    __dmb(_ARM64_BARRIER_SY);
}

#define PreFetchCacheLine(l,a)      __prefetch2((const void *) (a), ARM64_PREFETCH(PLD, L1, KEEP))
#define PrefetchForWrite(p)         __prefetch2((const void *) (p), ARM64_PREFETCH(PST, L1, KEEP))
#define ReadForWriteAccess(p)       (__prefetch2((const void *) (p), ARM64_PREFETCH(PST, L1, KEEP)), *(p))

#define _DataSynchronizationBarrier()        __dsb(_ARM64_BARRIER_SY)
#define _InstructionSynchronizationBarrier() __isb(_ARM64_BARRIER_SY)

FORCEINLINE
VOID
YieldProcessor (
    VOID
    )
{
    __dmb(_ARM64_BARRIER_ISHST);
    __yield();
}

//
// Define accessors for volatile loads and stores.
//

#pragma intrinsic(__iso_volatile_load8)
#pragma intrinsic(__iso_volatile_load16)
#pragma intrinsic(__iso_volatile_load32)
#pragma intrinsic(__iso_volatile_load64)
#pragma intrinsic(__iso_volatile_store8)
#pragma intrinsic(__iso_volatile_store16)
#pragma intrinsic(__iso_volatile_store32)
#pragma intrinsic(__iso_volatile_store64)
#pragma intrinsic(__ldar8)
#pragma intrinsic(__ldar16)
#pragma intrinsic(__ldar32)
#pragma intrinsic(__ldar64)
#pragma intrinsic(__stlr8)
#pragma intrinsic(__stlr16)
#pragma intrinsic(__stlr32)
#pragma intrinsic(__stlr64)

#if _MSC_FULL_VER >= 193632407
#pragma intrinsic(__load_acquire8)
#pragma intrinsic(__load_acquire16)
#pragma intrinsic(__load_acquire32)
#pragma intrinsic(__load_acquire64)
#endif // _MSC_FULL_VER >= 193632407

//
//

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
//

FORCEINLINE
CHAR
ReadAcquire8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

#if defined(__clang__) && !defined(RUST_BINDGEN)
    Value = (CHAR)__atomic_load_n((unsigned __int8 volatile*)Source, 2);
#else // defined(__clang__) && !defined(RUST_BINDGEN)
#if _MSC_FULL_VER >= 193632407
    Value = (CHAR)__load_acquire8((unsigned __int8 volatile*)Source);
#else
    Value = (CHAR)__ldar8((unsigned __int8 volatile*)Source);
#endif // defined(__clang__) && !defined(RUST_BINDGEN)
#endif

    return Value;
}

FORCEINLINE
CHAR
ReadNoFence8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

    Value = __iso_volatile_load8(Source);
    return Value;
}

FORCEINLINE
VOID
WriteRelease8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

#if defined(__clang__) && !defined(RUST_BINDGEN)
    __atomic_store_n((unsigned __int8 volatile*)Destination, (unsigned __int8)Value, 3);
#else
    __stlr8((unsigned __int8 volatile*)Destination, (unsigned __int8)Value);
#endif

    return;
}

FORCEINLINE
VOID
WriteNoFence8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

    __iso_volatile_store8(Destination, Value);
    return;
}

FORCEINLINE
SHORT
ReadAcquire16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

#if defined(__clang__) && !defined(RUST_BINDGEN)
    Value = (SHORT)__atomic_load_n((unsigned __int16 volatile*)Source, 2);
#else // defined(__clang__) && !defined(RUST_BINDGEN)
#if _MSC_FULL_VER >= 193632407
    Value = (SHORT)__load_acquire16((unsigned __int16 volatile*)Source);
#else
    Value = (SHORT)__ldar16((unsigned __int16 volatile*)Source);
#endif
#endif // defined(__clang__) && !defined(RUST_BINDGEN)

    return Value;
}

FORCEINLINE
SHORT
ReadNoFence16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

    Value = __iso_volatile_load16(Source);
    return Value;
}

FORCEINLINE
VOID
WriteRelease16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

#if defined(__clang__) && !defined(RUST_BINDGEN)
    __atomic_store_n((unsigned __int16 volatile*)Destination, (unsigned __int16)Value, 3);
#else
    __stlr16((unsigned __int16 volatile*)Destination, (unsigned __int16)Value);
#endif

    return;
}

FORCEINLINE
VOID
WriteNoFence16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

    __iso_volatile_store16(Destination, Value);
    return;
}

FORCEINLINE
LONG
ReadAcquire (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

#if defined(__clang__) && !defined(RUST_BINDGEN)
    Value = (LONG)__atomic_load_n((unsigned __int32 volatile*)Source, 2);
#else // defined(__clang__) && !defined(RUST_BINDGEN)
#if _MSC_FULL_VER >= 193632407
    Value = (LONG)__load_acquire32((unsigned __int32 volatile*)Source);
#else
    Value = (LONG)__ldar32((unsigned __int32 volatile*)Source);
#endif
#endif // defined(__clang__) && !defined(RUST_BINDGEN)

    return Value;
}

FORCEINLINE
LONG
ReadNoFence (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

    Value = __iso_volatile_load32((int *)Source);
    return Value;
}

FORCEINLINE
VOID
WriteRelease (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

#if defined(__clang__) && !defined(RUST_BINDGEN)
    __atomic_store_n((unsigned __int32 volatile*)Destination, (unsigned __int32)Value, 3);
#else
    __stlr32((unsigned __int32 volatile*)Destination, (unsigned __int32)Value);
#endif

    return;
}

FORCEINLINE
VOID
WriteNoFence (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

    __iso_volatile_store32((int *)Destination, Value);
    return;
}

FORCEINLINE
LONG64
ReadAcquire64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

#if defined(__clang__) && !defined(RUST_BINDGEN)
    Value = (LONG64)__atomic_load_n((unsigned __int64 volatile*)Source, 2);
#else // defined(__clang__) && !defined(RUST_BINDGEN)
#if _MSC_FULL_VER >= 193632407
    Value = (LONG64)__load_acquire64((unsigned __int64 volatile*)Source);
#else
    Value = (LONG64)__ldar64((unsigned __int64 volatile*)Source);
#endif
#endif // defined(__clang__) && !defined(RUST_BINDGEN)

    return Value;
}

FORCEINLINE
LONG64
ReadNoFence64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

    Value = __iso_volatile_load64(Source);
    return Value;
}

FORCEINLINE
VOID
WriteRelease64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

#if defined(__clang__) && !defined(RUST_BINDGEN)
    __atomic_store_n((unsigned __int64 volatile*)Destination, (unsigned __int64)Value, 3);
#else
    __stlr64((unsigned __int64 volatile*)Destination, (unsigned __int64)Value);
#endif

    return;
}

FORCEINLINE
VOID
WriteNoFence64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

    __iso_volatile_store64(Destination, Value);
    return;
}

FORCEINLINE
VOID
BarrierAfterRead (
    VOID
    )

{
    __dmb(_ARM64_BARRIER_ISH);
    return;
}

//
//

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
//

//
// Define coprocessor access intrinsics.  Coprocessor 15 contains
// registers for the MMU, cache, TLB, feature bits, core
// identification and performance counters.
//

// op0=2/3 encodings, use with _Read/WriteStatusReg
#define ARM64_SYSREG(op0, op1, crn, crm, op2) \
        ( ((op0 & 1) << 14) | \
          ((op1 & 7) << 11) | \
          ((crn & 15) << 7) | \
          ((crm & 15) << 3) | \
          ((op2 & 7) << 0) )

// op0=1 encodings, use with __sys
#define ARM64_SYSINSTR(op0, op1, crn, crm, op2) \
        ( ((op1 & 7) << 11) | \
          ((crn & 15) << 7) | \
          ((crm & 15) << 3) | \
          ((op2 & 7) << 0) )

#define ARM64_SYSREG_OP1(_Reg_) (((_Reg_) >> 11) & 7)
#define ARM64_SYSREG_CRN(_Reg_) (((_Reg_) >> 7) & 15)
#define ARM64_SYSREG_CRM(_Reg_) (((_Reg_) >> 3) & 15)
#define ARM64_SYSREG_OP2(_Reg_) ((_Reg_) & 7)

#define ARM64_CNTVCT            ARM64_SYSREG(3,3,14, 0,2)  // Generic Timer counter register
#define ARM64_CNTVCT_EL0        ARM64_SYSREG(3,3,14, 0,2)  // Generic Timer counter register
#define ARM64_CNTFRQ_EL0        ARM64_SYSREG(3,3,14, 0,0)  // Counter-timer Frequency register
#define ARM64_PMCCNTR_EL0       ARM64_SYSREG(3,3, 9,13,0)  // Cycle Count Register [CP15_PMCCNTR]
#define ARM64_PMSELR_EL0        ARM64_SYSREG(3,3, 9,12,5)  // Event Counter Selection Register [CP15_PMSELR]
#define ARM64_PMXEVCNTR_EL0     ARM64_SYSREG(3,3, 9,13,2)  // Event Count Register [CP15_PMXEVCNTR]
#define ARM64_PMXEVCNTRn_EL0(n) ARM64_SYSREG(3,3,14, 8+((n)/8), (n)%8)    // Direct Event Count Register [n/a]
#define ARM64_TPIDR_EL0         ARM64_SYSREG(3,3,13, 0,2)  // Thread ID Register, User Read/Write [CP15_TPIDRURW]
#define ARM64_TPIDRRO_EL0       ARM64_SYSREG(3,3,13, 0,3)  // Thread ID Register, User Read Only [CP15_TPIDRURO]
#define ARM64_TPIDR_EL1         ARM64_SYSREG(3,0,13, 0,4)  // Thread ID Register, Privileged Only [CP15_TPIDRPRW]

#pragma intrinsic(_WriteStatusReg)
#pragma intrinsic(_ReadStatusReg)

//
// PreFetchCacheLine level defines.
//

#define PF_TEMPORAL_LEVEL_1         0
#define PF_TEMPORAL_LEVEL_2         1
#define PF_TEMPORAL_LEVEL_3         2
#define PF_NON_TEMPORAL_LEVEL_ALL   3

#if defined(_M_HYBRID_X86_ARM64)

extern DWORD64 (*_os_wowa64_rdtsc) (VOID);

#endif // defined(_M_HYBRID_X86_ARM64)

//
// Define function to read the value of the time stamp counter.
//

#if defined(_M_HYBRID_X86_ARM64)

DECLSPEC_GUARDNOCF

#endif // defined(_M_HYBRID_X86_ARM64)

//
// hextract end_ntoshvp

FORCEINLINE
DWORD64
ReadTimeStampCounter(
    VOID
    )
{

#if defined(_M_HYBRID_X86_ARM64)

    //
    // Call into the emulator to return the same value as the x86 RDTSC
    // instruction.
    //

    return (*_os_wowa64_rdtsc)();

#else // defined(_M_HYBRID_X86_ARM64)

    return (DWORD64)_ReadStatusReg(ARM64_PMCCNTR_EL0);

#endif // defined(_M_HYBRID_X86_ARM64)

}

FORCEINLINE
DWORD64
ReadPMC (
    _In_ DWORD Counter
    )
{

    switch (Counter) {
    case  0: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(0));
    case  1: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(1));
    case  2: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(2));
    case  3: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(3));
    case  4: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(4));
    case  5: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(5));
    case  6: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(6));
    case  7: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(7));
    case  8: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(8));
    case  9: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(9));
    case 10: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(10));
    case 11: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(11));
    case 12: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(12));
    case 13: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(13));
    case 14: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(14));
    case 15: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(15));
    case 16: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(16));
    case 17: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(17));
    case 18: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(18));
    case 19: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(19));
    case 20: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(20));
    case 21: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(21));
    case 22: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(22));
    case 23: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(23));
    case 24: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(24));
    case 25: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(25));
    case 26: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(26));
    case 27: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(27));
    case 28: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(28));
    case 29: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(29));
    case 30: return (DWORD64)_ReadStatusReg(ARM64_PMXEVCNTRn_EL0(30));
    default: return 0;
    }
}

// hextract begin_ntoshvp
//

//
// Define functions to capture the high 64-bits of a 128-bit multiply.
//

#define MultiplyHigh __mulh
#define UnsignedMultiplyHigh __umulh

#pragma intrinsic(__mulh)
#pragma intrinsic(__umulh)

#endif // defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)


//
// Define population count intrinsic.
//

#if !defined(PopulationCount64)

__forceinline
DWORD64
PopulationCount64 (
    _In_ DWORD64 operand
    )
{
#if (defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64))
#if defined(__clang__)
    return __builtin_popcountll(operand);
#else
    return _CountOneBits64(operand);
#endif
#else

    // log(n) population count

    DWORD64 highBits = (operand & 0xAAAAAAAAAAAAAAAA) >> 1;
    DWORD64 lowBits = operand & 0x5555555555555555;
    DWORD64 bitSum = highBits + lowBits;

    highBits = (bitSum & 0xCCCCCCCCCCCCCCCC) >> 2;
    lowBits = bitSum & 0x3333333333333333;
    bitSum = highBits + lowBits;

    highBits = (bitSum & 0xF0F0F0F0F0F0F0F0) >> 4;
    lowBits = bitSum & 0x0F0F0F0F0F0F0F0F;
    bitSum = highBits + lowBits;

    highBits = (bitSum & 0xFF00FF00FF00FF00) >> 8;
    lowBits = bitSum & 0x00FF00FF00FF00FF;
    bitSum = highBits + lowBits;

    highBits = (bitSum & 0xFFFF0000FFFF0000) >> 16;
    lowBits = bitSum & 0x0000FFFF0000FFFF;
    bitSum = highBits + lowBits;

    highBits = (bitSum & 0xFFFFFFFF00000000) >> 32;
    lowBits = bitSum & 0x00000000FFFFFFFF;
    bitSum = highBits + lowBits;

    return bitSum;

#endif // (defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64))
}

#endif // !defined(PopulationCount64)

//
// Define functions to perform 128-bit shifts
//

#if !defined(ShiftLeft128)

#define __shiftleft128   ShiftLeft128
#define __shiftright128  ShiftRight128

__forceinline
DWORD64
ShiftLeft128 (
    _In_ DWORD64 LowPart,
    _In_ DWORD64 HighPart,
    _In_ BYTE  Shift
    )
{
    Shift &= 63;

    if (Shift == 0) {
        return HighPart;
    }

    return (HighPart << Shift) | (LowPart >> (64 - Shift));
}

__forceinline
DWORD64
ShiftRight128 (
    _In_ DWORD64 LowPart,
    _In_ DWORD64 HighPart,
    _In_ BYTE  Shift
    )
{
    Shift &= 63;

    if (Shift == 0) {
        return LowPart;
    }

    return (LowPart >> Shift) | (HighPart << (64 - Shift));
}

#endif // !defined(ShiftLeft128)

//
// Define functions to perform 128-bit multiplies.
//

#if !defined(_ARM64_MULT_INTRINS_SUPPORTED)
#define _ARM64_MULT_INTRINS_SUPPORTED 0
#if defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#if defined(__clang__)
#if __has_builtin(__umulh) && __has_builtin(__mulh)
#undef _ARM64_MULT_INTRINS_SUPPORTED
#define _ARM64_MULT_INTRINS_SUPPORTED 1
#endif // __has_builtin(__umulh) && __has_builtin(__mulh)
#else // defined(__clang__)
#undef _ARM64_MULT_INTRINS_SUPPORTED
#define _ARM64_MULT_INTRINS_SUPPORTED 1
#endif // defined(__clang__)
#endif // defined(_M_ARM64) || defined(_M_ARM64EC) || defined(_M_HYBRID_X86_ARM64)
#endif // !defined(_ARM64_MULT_INTRINS_SUPPORTED)

#if !defined(UnsignedMultiply128)

__forceinline
DWORD64
UnsignedMultiply128 (
    _In_ DWORD64 Multiplier,
    _In_ DWORD64 Multiplicand,
    _Out_ DWORD64 *HighProduct
    )

/*++

Routine Description:

    Calculates the 128 bit product of two 64 bit integers.

Arguments:

    Multiplier -

    Multiplicand -

    HighProduct - Receives the high 64 bits of the product.

Return Value:

    Low 64 bits of the product.

--*/

{

#if (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)) && _ARM64_MULT_INTRINS_SUPPORTED

    *HighProduct = UnsignedMultiplyHigh(Multiplier, Multiplicand);
    return Multiplier * Multiplicand;

#else

    DWORD64 HiMultiplier = Multiplier >> 32;
    DWORD64 LoMultiplier = Multiplier & 0xFFFFFFFF;
    DWORD64 HiMultiplicand = Multiplicand >> 32;
    DWORD64 LoMultiplicand = Multiplicand & 0xFFFFFFFF;
    DWORD64 CrossTerm1 = (HiMultiplier * LoMultiplicand);
    DWORD64 CrossTerm2 = (LoMultiplier * HiMultiplicand);

    DWORD64 ResultLo = (LoMultiplier * LoMultiplicand);
    DWORD64 ResultHi = (HiMultiplier * HiMultiplicand);

    // Add the cross-terms and propagate carries across all 128 bits.

    ResultLo += (CrossTerm1 << 32);
    ResultHi += (CrossTerm1 >> 32) + (ResultLo < (CrossTerm1 << 32));

    ResultLo += (CrossTerm2 << 32);
    ResultHi += (CrossTerm2 >> 32) + (ResultLo < (CrossTerm2 << 32));

    *HighProduct = ResultHi;

    return ResultLo;

#endif

}

#endif // !defined(UnsignedMultiply128)

#if !defined(Multiply128)

__forceinline
LONG64
Multiply128 (
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand,
    _Out_ LONG64 *HighProduct
    )
{
#if (defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)) && _ARM64_MULT_INTRINS_SUPPORTED

    *HighProduct = MultiplyHigh(Multiplier, Multiplicand);
    return Multiplier * Multiplicand;

#else

    LONG64 Result = (LONG64)UnsignedMultiply128((DWORD64)Multiplier, (DWORD64)Multiplicand, (DWORD64 *)HighProduct);

    *HighProduct += (Multiplier >> 63) * Multiplicand;
    *HighProduct += Multiplier * (Multiplicand >> 63);

    return Result;

#endif
}

#endif // !defined(Multiply128)

#if !defined(_M_ARM64EC)

__forceinline
LONG64
MultiplyExtract128 (
    _In_ LONG64 Multiplier,
    _In_ LONG64 Multiplicand,
    _In_ BYTE  Shift
    )

{

    LONG64 extractedProduct;
    LONG64 highProduct;
    LONG64 lowProduct;
    BOOLEAN negate;
    DWORD64 uhighProduct;
    DWORD64 ulowProduct;

    lowProduct = Multiply128(Multiplier, Multiplicand, &highProduct);
    negate = FALSE;
    uhighProduct = (DWORD64)highProduct;
    ulowProduct = (DWORD64)lowProduct;
    if (highProduct < 0) {
        negate = TRUE;
        uhighProduct = (DWORD64)(-highProduct);
        ulowProduct = (DWORD64)(-lowProduct);
        if (ulowProduct != 0) {
            uhighProduct -= 1;
        }
    }

    extractedProduct = (LONG64)ShiftRight128(ulowProduct, uhighProduct, Shift);
    if (negate != FALSE) {
        extractedProduct = -extractedProduct;
    }

    return extractedProduct;
}

__forceinline
DWORD64
UnsignedMultiplyExtract128 (
    _In_ DWORD64 Multiplier,
    _In_ DWORD64 Multiplicand,
    _In_ BYTE  Shift
    )

{

    DWORD64 extractedProduct;
    DWORD64 highProduct;
    DWORD64 lowProduct;

    lowProduct = UnsignedMultiply128(Multiplier, Multiplicand, &highProduct);
    extractedProduct = ShiftRight128(lowProduct, highProduct, Shift);
    return extractedProduct;
}

#endif // !defined(_M_ARM64EC)



#if defined(_M_ARM64EC)

unsigned int
_mm_getcsr (
    VOID
    );

VOID
_mm_setcsr (
    _In_ unsigned int MxCsr
    );

#endif // defined(_M_ARM64EC)

#endif // !defined(RC_INVOKED) && !defined(MIDL_PASS)

#else // defined(_M_CEE_PURE)

#include <intrin.h>

#endif // !defined(_M_CEE_PURE)

#if defined(_M_CEE_PURE)
FORCEINLINE
VOID
YieldProcessor (
    VOID
    )
{
}
#endif // defined(_M_CEE_PURE)

//
//

//
// The following values specify the type of access in the first parameter
// of the exception record whan the exception code specifies an access
// violation.
//

#define EXCEPTION_READ_FAULT 0          // exception caused by a read
#define EXCEPTION_WRITE_FAULT 1         // exception caused by a write
#define EXCEPTION_EXECUTE_FAULT 8       // exception caused by an instruction fetch

//
// Define initial Cpsr/Fpscr value
//

#define INITIAL_CPSR 0x10
#define INITIAL_FPSCR 0

//
//

#endif // defined(_ARM64_) || defined(_CHPE_X86_ARM64_) || defined(_ARM64EC_)

//
//

//
// The following flags control the contents of the CONTEXT structure.
//

#if !defined(RC_INVOKED)

#define CONTEXT_ARM64   0x00400000L

#define CONTEXT_ARM64_CONTROL (CONTEXT_ARM64 | 0x1L)
#define CONTEXT_ARM64_INTEGER (CONTEXT_ARM64 | 0x2L)
#define CONTEXT_ARM64_FLOATING_POINT  (CONTEXT_ARM64 | 0x4L)
#define CONTEXT_ARM64_DEBUG_REGISTERS (CONTEXT_ARM64 | 0x8L)
#define CONTEXT_ARM64_X18 (CONTEXT_ARM64 | 0x10L)
#define CONTEXT_ARM64_XSTATE (CONTEXT_ARM64 | 0x20L)
#define CONTEXT_ARM64_FLOATING_POINT_LOW (CONTEXT_ARM64 | 0x40L)
#define CONTEXT_ARM64_FLOATING_POINT_HIGH (CONTEXT_ARM64 | 0x80L)

//
// CONTEXT_ARM64_X18 is not part of CONTEXT_ARM64_FULL because in NT user-mode
// threads, x18 contains a pointer to the TEB and should generally not be set
// without intending to.
//
// CONTEXT_ARM64_FLOATING_POINT_LOW and CONTEXT_ARM64_FLOATING_POINT_HIGH are
// not part of CONTEXT_ARM64_FULL because they are only used in limited cases
// involving conversion between ARM64 and ARM64EC (AMD64) context records.
//

#define CONTEXT_ARM64_FULL (CONTEXT_ARM64_CONTROL | CONTEXT_ARM64_INTEGER | CONTEXT_ARM64_FLOATING_POINT)
#define CONTEXT_ARM64_ALL  (CONTEXT_ARM64_CONTROL | CONTEXT_ARM64_INTEGER | CONTEXT_ARM64_FLOATING_POINT | \
                            CONTEXT_ARM64_DEBUG_REGISTERS | CONTEXT_ARM64_X18 | CONTEXT_ARM64_FLOATING_POINT_LOW | \
                            CONTEXT_ARM64_FLOATING_POINT_HIGH)

#if defined(_ARM64_)

#define CONTEXT_CONTROL CONTEXT_ARM64_CONTROL
#define CONTEXT_INTEGER CONTEXT_ARM64_INTEGER
#define CONTEXT_FLOATING_POINT CONTEXT_ARM64_FLOATING_POINT
#define CONTEXT_DEBUG_REGISTERS CONTEXT_ARM64_DEBUG_REGISTERS
#define CONTEXT_XSTATE CONTEXT_ARM64_XSTATE
#define CONTEXT_FULL CONTEXT_ARM64_FULL
#define CONTEXT_ALL CONTEXT_ARM64_ALL

#define CONTEXT_EXCEPTION_ACTIVE    0x08000000L
#define CONTEXT_SERVICE_ACTIVE      0x10000000L
#define CONTEXT_EXCEPTION_REQUEST   0x40000000L
#define CONTEXT_EXCEPTION_REPORTING 0x80000000L

#endif // defined(_ARM64_)

//
// CONTEXT_UNWOUND_TO_CALL flag is set by the unwinder if it
// has unwound to a call site, and cleared whenever it unwinds
// through a trap frame. It is used by language-specific exception
// handlers to help differentiate exception scopes during dispatching.
//

#define CONTEXT_ARM64_UNWOUND_TO_CALL 0x20000000
#define CONTEXT_ARM64_RET_TO_GUEST    0x04000000

#if defined(_ARM64_) || defined(_CHPE_X86_ARM64_) || defined(_X86_)

#define CONTEXT_UNWOUND_TO_CALL CONTEXT_ARM64_UNWOUND_TO_CALL
#define CONTEXT_RET_TO_GUEST    CONTEXT_ARM64_RET_TO_GUEST

#endif // defined(_ARM64_) || defined(_CHPE_X86_ARM64_) || defined(_X86_)

#endif // !defined(RC_INVOKED)

//
//

//
// Specify the number of breakpoints and watchpoints that the OS
// will track. Architecturally, ARM64 supports up to 16. In practice,
// however, almost no one implements more than 4 of each.
//

#define ARM64_MAX_BREAKPOINTS     8
#define ARM64_MAX_WATCHPOINTS     2

//
// Context Frame
//
//  This frame has a several purposes: 1) it is used as an argument to
//  NtContinue, 2) it is used to constuct a call frame for APC delivery,
//  and 3) it is used in the user level thread creation routines.
//
//
// The flags field within this record controls the contents of a CONTEXT
// record.
//
// If the context record is used as an input parameter, then for each
// portion of the context record controlled by a flag whose value is
// set, it is assumed that that portion of the context record contains
// valid context. If the context record is being used to modify a threads
// context, then only that portion of the threads context is modified.
//
// If the context record is used as an output parameter to capture the
// context of a thread, then only those portions of the thread's context
// corresponding to set flags will be returned.
//
// CONTEXT_CONTROL specifies FP, LR, SP, PC, and CPSR
//
// CONTEXT_INTEGER specifies X0-X28
//
// CONTEXT_FLOATING_POINT specifies Fpcr, Fpsr and Q0-Q31 / D0-D31 / S0-S31
//
// CONTEXT_DEBUG_REGISTERS specifies up to 16 of DBGBVR, DBGBCR, DBGWVR,
//      DBGWCR.
//
// CONTEXT_XSTATE specifies ARM64 extended state such as SVE and SME.
//
// CONTEXT_ARM64_FLOATING_POINT_LOW specifies that only the FPCR, FPSR and
// V0-V15 should be operated on, and CONTEXT_ARM64_FLOATING_POINT_HIGH
// specifies that only V16-31 should be operated on, for use in ARM64 context
// records that have been converted from ARM64EC (AMD64) context records. Both
// flags are considered to be set if CONTEXT_FLOATING_POINT is set.
//

typedef union _ARM64_NT_NEON128 {
    struct {
        ULONGLONG Low;
        LONGLONG High;
    } DUMMYSTRUCTNAME;
    double D[2];
    float S[4];
    WORD   H[8];
    BYTE  B[16];
} ARM64_NT_NEON128, *PARM64_NT_NEON128;

#if defined(_ARM64_)

typedef ARM64_NT_NEON128 NEON128, *PNEON128;

#endif // defined(_ARM64_)

#if defined(_ARM64_)

#pragma push_macro("_ARM64_NT_CONTEXT")
#undef _ARM64_NT_CONTEXT
#define _ARM64_NT_CONTEXT _CONTEXT

#pragma push_macro("ARM64_NT_NEON128")
#undef ARM64_NT_NEON128
#define ARM64_NT_NEON128 NEON128

#endif // defined(_ARM64_)

typedef struct DECLSPEC_ALIGN(16) DECLSPEC_NOINITALL _ARM64_NT_CONTEXT {

    //
    // Control flags.
    //

    /* +0x000 */ DWORD ContextFlags;

    //
    // Integer registers
    //

    /* +0x004 */ DWORD Cpsr;       // NZVF + DAIF + CurrentEL + SPSel
    /* +0x008 */ union {
                    struct {
                        DWORD64 X0;
                        DWORD64 X1;
                        DWORD64 X2;
                        DWORD64 X3;
                        DWORD64 X4;
                        DWORD64 X5;
                        DWORD64 X6;
                        DWORD64 X7;
                        DWORD64 X8;
                        DWORD64 X9;
                        DWORD64 X10;
                        DWORD64 X11;
                        DWORD64 X12;
                        DWORD64 X13;
                        DWORD64 X14;
                        DWORD64 X15;
                        DWORD64 X16;
                        DWORD64 X17;
                        DWORD64 X18;
                        DWORD64 X19;
                        DWORD64 X20;
                        DWORD64 X21;
                        DWORD64 X22;
                        DWORD64 X23;
                        DWORD64 X24;
                        DWORD64 X25;
                        DWORD64 X26;
                        DWORD64 X27;
                        DWORD64 X28;
    /* +0x0f0 */        DWORD64 Fp;
    /* +0x0f8 */        DWORD64 Lr;
                    } DUMMYSTRUCTNAME;
                    DWORD64 X[31];
                 } DUMMYUNIONNAME;
    /* +0x100 */ DWORD64 Sp;
    /* +0x108 */ DWORD64 Pc;

    //
    // Floating Point/NEON Registers
    //

    /* +0x110 */ ARM64_NT_NEON128 V[32];
    /* +0x310 */ DWORD Fpcr;
    /* +0x314 */ DWORD Fpsr;

    //
    // Debug registers
    //

    /* +0x318 */ DWORD Bcr[ARM64_MAX_BREAKPOINTS];
    /* +0x338 */ DWORD64 Bvr[ARM64_MAX_BREAKPOINTS];
    /* +0x378 */ DWORD Wcr[ARM64_MAX_WATCHPOINTS];
    /* +0x380 */ DWORD64 Wvr[ARM64_MAX_WATCHPOINTS];
    /* +0x390 */

} ARM64_NT_CONTEXT, *PARM64_NT_CONTEXT;

#if defined(_ARM64_)

#undef ARM64_NT_NEON128
#pragma pop_macro("ARM64_NT_NEON128")

#undef _ARM64_NT_CONTEXT
#pragma pop_macro("_ARM64_NT_CONTEXT")

typedef ARM64_NT_CONTEXT CONTEXT, *PCONTEXT;

#endif // defined(_ARM64_)

typedef struct DECLSPEC_ALIGN(16) DECLSPEC_NOINITALL _ARM64EC_NT_CONTEXT {
    union {
        struct {

            //
            // AMD64 call register home space. These can't be used by ARM64EC
            //

            /* +0x000 */ DWORD64 AMD64_P1Home;
            /* +0x008 */ DWORD64 AMD64_P2Home;
            /* +0x010 */ DWORD64 AMD64_P3Home;
            /* +0x018 */ DWORD64 AMD64_P4Home;
            /* +0x020 */ DWORD64 AMD64_P5Home;
            /* +0x028 */ DWORD64 AMD64_P6Home;

            //
            // Control flags.
            //

            /* +0x030 */ DWORD ContextFlags;

            /* +0x034 */ DWORD AMD64_MxCsr_copy;

            //
            // Segment Registers and processor flags. These can't be used by
            // ARM64EC
            //

            /* +0x038 */ WORD   AMD64_SegCs;
            /* +0x03a */ WORD   AMD64_SegDs;
            /* +0x03c */ WORD   AMD64_SegEs;
            /* +0x03e */ WORD   AMD64_SegFs;
            /* +0x040 */ WORD   AMD64_SegGs;
            /* +0x042 */ WORD   AMD64_SegSs;

            //
            // General purpose flags.
            //

            /* +0x044 */ DWORD AMD64_EFlags;

            //
            // Debug registers
            //

            /* +0x048 */ DWORD64 AMD64_Dr0;
            /* +0x050 */ DWORD64 AMD64_Dr1;
            /* +0x058 */ DWORD64 AMD64_Dr2;
            /* +0x060 */ DWORD64 AMD64_Dr3;
            /* +0x068 */ DWORD64 AMD64_Dr6;
            /* +0x070 */ DWORD64 AMD64_Dr7;

            //
            // Integer registers.
            //

            /* +0x078 */ DWORD64 X8;     // AMD64_Rax
            /* +0x080 */ DWORD64 X0;     // AMD64_Rcx
            /* +0x088 */ DWORD64 X1;     // AMD64_Rdx
            /* +0x090 */ DWORD64 X27;    // AMD64_Rbx
            /* +0x098 */ DWORD64 Sp;     // AMD64_Rsp
            /* +0x0a0 */ DWORD64 Fp;     // AMD64_Rbp
            /* +0x0a8 */ DWORD64 X25;    // AMD64_Rsi
            /* +0x0b0 */ DWORD64 X26;    // AMD64_Rdi
            /* +0x0b8 */ DWORD64 X2;     // AMD64_R8
            /* +0x0c0 */ DWORD64 X3;     // AMD64_R9
            /* +0x0c8 */ DWORD64 X4;     // AMD64_R10
            /* +0x0d0 */ DWORD64 X5;     // AMD64_R11
            /* +0x0d8 */ DWORD64 X19;    // AMD64_R12
            /* +0x0e0 */ DWORD64 X20;    // AMD64_R13
            /* +0x0e8 */ DWORD64 X21;    // AMD64_R14
            /* +0x0f0 */ DWORD64 X22;    // AMD64_R15

            //
            // Program counter.
            //

            /* +0x0f8 */ DWORD64 Pc;     // AMD64_Rip

            //
            // Floating point state.
            //

            struct {
                /* +0x100 */ WORD   AMD64_ControlWord;
                /* +0x102 */ WORD   AMD64_StatusWord;
                /* +0x104 */ BYTE  AMD64_TagWord;
                /* +0x105 */ BYTE  AMD64_Reserved1;
                /* +0x106 */ WORD   AMD64_ErrorOpcode;
                /* +0x108 */ DWORD AMD64_ErrorOffset;
                /* +0x10c */ WORD   AMD64_ErrorSelector;
                /* +0x10e */ WORD   AMD64_Reserved2;
                /* +0x110 */ DWORD AMD64_DataOffset;
                /* +0x114 */ WORD   AMD64_DataSelector;
                /* +0x116 */ WORD   AMD64_Reserved3;

                /* +0x118 */ DWORD AMD64_MxCsr;
                /* +0x11c */ DWORD AMD64_MxCsr_Mask;

                /* +0x120 */ DWORD64 Lr;                 // AMD64_St0_Low
                /* +0x128 */ WORD   X16_0;               // AMD64_St0_High
                /* +0x12a */ WORD   AMD64_St0_Reserved1;
                /* +0x12c */ DWORD AMD64_St0_Reserved2;
                /* +0x130 */ DWORD64 X6;                 // AMD64_St1_Low
                /* +0x138 */ WORD   X16_1;               // AMD64_St1_High
                /* +0x13a */ WORD   AMD64_St1_Reserved1;
                /* +0x13c */ DWORD AMD64_St1_Reserved2;
                /* +0x140 */ DWORD64 X7;                 // AMD64_St2_Low
                /* +0x148 */ WORD   X16_2;               // AMD64_St2_High
                /* +0x14a */ WORD   AMD64_St2_Reserved1;
                /* +0x14c */ DWORD AMD64_St2_Reserved2;
                /* +0x150 */ DWORD64 X9;                 // AMD64_St3_Low
                /* +0x158 */ WORD   X16_3;               // AMD64_St3_High
                /* +0x15a */ WORD   AMD64_St3_Reserved1;
                /* +0x15c */ DWORD AMD64_St3_Reserved2;
                /* +0x160 */ DWORD64 X10;                // AMD64_St4_Low
                /* +0x168 */ WORD   X17_0;               // AMD64_St4_High
                /* +0x16a */ WORD   AMD64_St4_Reserved1;
                /* +0x16c */ DWORD AMD64_St4_Reserved2;
                /* +0x170 */ DWORD64 X11;                // AMD64_St5_Low
                /* +0x178 */ WORD   X17_1;               // AMD64_St5_High
                /* +0x17a */ WORD   AMD64_St5_Reserved1;
                /* +0x17c */ DWORD AMD64_St5_Reserved2;
                /* +0x180 */ DWORD64 X12;                // AMD64_St6_Low
                /* +0x188 */ WORD   X17_2;               // AMD64_St6_High
                /* +0x18a */ WORD   AMD64_St6_Reserved1;
                /* +0x18c */ DWORD AMD64_St6_Reserved2;
                /* +0x190 */ DWORD64 X15;                // AMD64_St7_Low
                /* +0x198 */ WORD   X17_3;               // AMD64_St7_High;
                /* +0x19a */ WORD   AMD64_St7_Reserved1;
                /* +0x19c */ DWORD AMD64_St7_Reserved2;

                /* +0x1a0 */ ARM64_NT_NEON128 V[16];     // AMD64_XmmRegisters[16]
                /* +0x2a0 */ BYTE  AMD64_XSAVE_FORMAT_Reserved4[96];
            } DUMMYSTRUCTNAME;

            //
            // AMD64 Vector registers.
            //

            /* +0x300 */ ARM64_NT_NEON128 AMD64_VectorRegister[26];
            /* +0x4a0 */ DWORD64 AMD64_VectorControl;

            //
            // AMD64 Special debug control registers.
            //

            /* +0x4a8 */ DWORD64 AMD64_DebugControl;
            /* +0x4b0 */ DWORD64 AMD64_LastBranchToRip;
            /* +0x4b8 */ DWORD64 AMD64_LastBranchFromRip;
            /* +0x4c0 */ DWORD64 AMD64_LastExceptionToRip;
            /* +0x4c8 */ DWORD64 AMD64_LastExceptionFromRip;
            /* +0x4d0 */

        } DUMMYSTRUCTNAME;

    #if defined(_ARM64EC_)

        CONTEXT AMD64_Context;

    #endif

    } DUMMYUNIONNAME;
} ARM64EC_NT_CONTEXT, *PARM64EC_NT_CONTEXT;

#if defined(_ARM64EC_) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

C_ASSERT(FIELD_OFFSET(ARM64EC_NT_CONTEXT, X8) == FIELD_OFFSET(CONTEXT, Rax));
C_ASSERT(FIELD_OFFSET(ARM64EC_NT_CONTEXT, Lr) == FIELD_OFFSET(CONTEXT, FltSave.FloatRegisters));
C_ASSERT(FIELD_OFFSET(ARM64EC_NT_CONTEXT, V) == FIELD_OFFSET(CONTEXT, Xmm0));

#endif // defined(_ARM64EC_) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

//
//

//
// Select platform-specific definitions
//

#if defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

typedef SCOPE_TABLE_ARM64 SCOPE_TABLE, *PSCOPE_TABLE;

#endif //  defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

typedef struct _IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY ARM64_RUNTIME_FUNCTION, *PARM64_RUNTIME_FUNCTION;

#if defined(_ARM64_)

typedef struct _IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY RUNTIME_FUNCTION, *PRUNTIME_FUNCTION;

#endif // defined(_ARM64_)

//
// Define unwind information flags.
//

#define UNW_FLAG_NHANDLER       0x0             /* any handler */
#define UNW_FLAG_EHANDLER       0x1             /* filter handler */
#define UNW_FLAG_UHANDLER       0x2             /* unwind handler */

#if defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

#pragma push_macro("_DISPATCHER_CONTEXT_ARM64")
#undef _DISPATCHER_CONTEXT_ARM64
#define _DISPATCHER_CONTEXT_ARM64 _DISPATCHER_CONTEXT

#endif // defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

//
// Define exception dispatch context structure.
//

#define NONVOL_INT_NUMREG_ARM64 (11)
#define NONVOL_FP_NUMREG_ARM64  (8)

#define NONVOL_INT_SIZE_ARM64 (NONVOL_INT_NUMREG_ARM64 * sizeof(DWORD64))
#define NONVOL_FP_SIZE_ARM64  (NONVOL_FP_NUMREG_ARM64 * sizeof(double))

typedef union _DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    BYTE  Buffer[NONVOL_INT_SIZE_ARM64 + NONVOL_FP_SIZE_ARM64];

    struct {
        DWORD64 GpNvRegs[NONVOL_INT_NUMREG_ARM64];      // [x19, x29(Fp)]
        double FpNvRegs [NONVOL_FP_NUMREG_ARM64];       // [V8d0, v15d0]
    } DUMMYSTRUCTNAME;
} DISPATCHER_CONTEXT_NONVOLREG_ARM64;

#if !defined(RC_INVOKED) && !defined(MIDL_PASS)

C_ASSERT(sizeof(DISPATCHER_CONTEXT_NONVOLREG_ARM64) == (NONVOL_INT_SIZE_ARM64 + NONVOL_FP_SIZE_ARM64));

#endif // !defined(RC_INVOKED) && !defined(MIDL_PASS)

typedef struct _DISPATCHER_CONTEXT_ARM64 {
    ULONG_PTR ControlPc;
    ULONG_PTR ImageBase;
    PARM64_RUNTIME_FUNCTION FunctionEntry;
    ULONG_PTR EstablisherFrame;
    ULONG_PTR TargetPc;
    PARM64_NT_CONTEXT ContextRecord;
    PEXCEPTION_ROUTINE LanguageHandler;
    PVOID HandlerData;
    struct _UNWIND_HISTORY_TABLE *HistoryTable;
    DWORD ScopeIndex;
    BOOLEAN ControlPcIsUnwound;
    PBYTE  NonVolatileRegisters;
} DISPATCHER_CONTEXT_ARM64, *PDISPATCHER_CONTEXT_ARM64;

#if defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

typedef DISPATCHER_CONTEXT_ARM64 DISPATCHER_CONTEXT, *PDISPATCHER_CONTEXT;

#undef _DISPATCHER_CONTEXT_ARM64
#pragma pop_macro("_DISPATCHER_CONTEXT_ARM64")

#endif // defined(_ARM64_) || defined(_CHPE_X86_ARM64_)


#if defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

//
// Define exception filter and termination handler function types.
// N.B. These functions use a custom calling convention.
//

struct _EXCEPTION_POINTERS;
typedef
LONG
(*PEXCEPTION_FILTER) (
    struct _EXCEPTION_POINTERS *ExceptionPointers,
    DWORD64 EstablisherFrame
    );

typedef
VOID
(*PTERMINATION_HANDLER) (
    BOOLEAN AbnormalTermination,
    DWORD64 EstablisherFrame
    );

#endif // defined(_ARM64_) || defined(_CHPE_X86_ARM64_)

#if defined(_ARM64_)

//
// Define dynamic function table entry.
//

typedef
_Function_class_(GET_RUNTIME_FUNCTION_CALLBACK)
PARM64_RUNTIME_FUNCTION
GET_RUNTIME_FUNCTION_CALLBACK (
    _In_ DWORD64 ControlPc,
    _In_opt_ PVOID Context
    );
typedef GET_RUNTIME_FUNCTION_CALLBACK *PGET_RUNTIME_FUNCTION_CALLBACK;

typedef
_Function_class_(OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK)
DWORD   
OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK (
    _In_ HANDLE Process,
    _In_ PVOID TableAddress,
    _Out_ PDWORD Entries,
    _Out_ PARM64_RUNTIME_FUNCTION* Functions
    );
typedef OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK *POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK;

#define OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME \
    "OutOfProcessFunctionTableCallback"

#endif // defined(_ARM64_)

//
// Nonvolatile context pointer record.
//

typedef struct _KNONVOLATILE_CONTEXT_POINTERS_ARM64 {

    PDWORD64 X19;
    PDWORD64 X20;
    PDWORD64 X21;
    PDWORD64 X22;
    PDWORD64 X23;
    PDWORD64 X24;
    PDWORD64 X25;
    PDWORD64 X26;
    PDWORD64 X27;
    PDWORD64 X28;
    PDWORD64 Fp;
    PDWORD64 Lr;

    PDWORD64 D8;
    PDWORD64 D9;
    PDWORD64 D10;
    PDWORD64 D11;
    PDWORD64 D12;
    PDWORD64 D13;
    PDWORD64 D14;
    PDWORD64 D15;

} KNONVOLATILE_CONTEXT_POINTERS_ARM64, *PKNONVOLATILE_CONTEXT_POINTERS_ARM64;

#if defined(_ARM64_)

typedef KNONVOLATILE_CONTEXT_POINTERS_ARM64 KNONVOLATILE_CONTEXT_POINTERS, *PKNONVOLATILE_CONTEXT_POINTERS;

#endif // defined(_ARM64_)

//
// begin_wudfwdm

#ifdef __cplusplus
extern "C" {
#endif

//
// Assert exception.
//

#if !defined(_DBGRAISEASSERTIONFAILURE_) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

#define _DBGRAISEASSERTIONFAILURE_

#if defined(_PREFAST_)

__analysis_noreturn
FORCEINLINE
VOID
DbgRaiseAssertionFailure (
    VOID
    );

#endif

#if defined(_AMD64_) && !defined(_ARM64EC_)

#if defined(_M_AMD64)

VOID
__int2c (
    VOID
    );

#pragma intrinsic(__int2c)

#if !defined(_PREFAST_)

#define DbgRaiseAssertionFailure() __int2c()

#endif // !defined(_PREFAST_)

#endif // defined(_M_AMD64) && !defined(_ARM64EC_)

#elif defined(_X86_) && !defined(_M_HYBRID_X86_ARM64)

#if defined(_M_IX86) && !defined(_M_HYBRID_X86_ARM64)

#if _MSC_FULL_VER >= 140030222

VOID
__int2c (
    VOID
    );

#pragma intrinsic(__int2c)

#if !defined(_PREFAST_)

#define DbgRaiseAssertionFailure() __int2c()

#endif // !defined(_PREFAST_)

#else // _MSC_FULL_VER >= 140030222

#pragma warning( push )
#pragma warning( disable : 4793 )

#if !defined(_PREFAST_)

__analysis_noreturn
FORCEINLINE
VOID
DbgRaiseAssertionFailure (
    VOID
    )

{
    __asm int 0x2c
}

#endif // !defined(_PREFAST_)

#pragma warning( pop )

#endif // _MSC_FULL_VER >= 140030222

#endif // defined(_M_IX86)

#elif defined(_IA64_)

#if defined(_M_IA64)

#pragma prefast( push )
#pragma prefast( disable: 28301 )

void
__break(
    _In_ int StIIM
    );

#pragma prefast( pop )

#pragma intrinsic (__break)

#define BREAK_DEBUG_BASE    0x080000
#define ASSERT_BREAKPOINT         (BREAK_DEBUG_BASE+3)  // Cause a STATUS_ASSERTION_FAILURE exception to be raised.

#if !defined(_PREFAST_)

#define DbgRaiseAssertionFailure() __break(ASSERT_BREAKPOINT)

#endif // !defined(_PREFAST_)

#endif // defined(_M_IA64)

#elif defined(_ARM64_) || defined(_CHPE_X86_ARM64_) || defined(_ARM64EC_)

#if defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)

#pragma prefast( push )
#pragma prefast( disable: 28301 )

void
__break(
    _In_ int Code
    );

#pragma prefast( pop )

#pragma intrinsic (__break)

#if !defined(_PREFAST_)

#define DbgRaiseAssertionFailure() __break(0xf001)

#endif // !defined(_PREFAST_)

#endif // defined(_M_ARM64) || defined(_M_HYBRID_X86_ARM64) || defined(_M_ARM64EC)

#elif defined(_ARM_)

#if defined(_M_ARM)

VOID
__emit(
    const unsigned __int32 opcode
    );

#pragma intrinsic(__emit)

#if !defined(_PREFAST_)

#define DbgRaiseAssertionFailure() __emit(0xdefc)     // THUMB_ASSERT

#endif // !defined(_PREFAST_)

#endif // defined(_M_ARM)

#endif // _AMD64_, _X86_, _IA64_, _ARM64_, _ARM_
#endif // !defined(_DBGRAISEASSERTIONFAILURE_) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

#ifdef __cplusplus
}
#endif

// begin_ntoshvp

#ifdef _X86_

//
// Some intrinsics have a redundant __cdecl calling-convention specifier when
// not compiled with /clr:pure.
//

#if defined(_M_CEE_PURE)

#define CDECL_NON_WVMPURE

#else

#define CDECL_NON_WVMPURE __cdecl

#endif

// end_ntoshvp
//
// Disable these two pragmas that evaluate to "sti" "cli" on x86 so that driver
// writers to not leave them inadvertantly in their code.
//

#if !defined(MIDL_PASS)
#if !defined(RC_INVOKED)

#if _MSC_VER >= 1200
#pragma warning(push)
#endif // _MSC_VER >= 1200
//#pragma warning(disable:4164)   // disable C4164 warning so that apps that
                                // build with /Od don't get weird errors !

#if _MSC_VER >= 1200
#pragma warning( pop )
#else
#pragma warning( default:4164 ) // reenable C4164 warning
#endif // _MSC_VER >= 1200

#endif // !defined(MIDL_PASS)
#endif // !defined(RC_INVOKED)



// end_ntddk end_nthal
// begin_ntoshvp
#if defined(_M_IX86) && !defined(RC_INVOKED) && !defined(MIDL_PASS)

#ifdef __cplusplus
extern "C" {
#endif

#if !defined(_MANAGED)

//
// Define bit test intrinsics.
//

#define BitTest _bittest
#define BitTestAndComplement _bittestandcomplement
#define BitTestAndSet _bittestandset
#define BitTestAndReset _bittestandreset
#define InterlockedBitTestAndSet _interlockedbittestandset
#define InterlockedBitTestAndSetAcquire _interlockedbittestandset
#define InterlockedBitTestAndSetRelease _interlockedbittestandset
#define InterlockedBitTestAndSetNoFence _interlockedbittestandset
#define InterlockedBitTestAndReset _interlockedbittestandreset
#define InterlockedBitTestAndResetAcquire _interlockedbittestandreset
#define InterlockedBitTestAndResetRelease _interlockedbittestandreset
#define InterlockedBitTestAndResetNoFence _interlockedbittestandreset

_Must_inspect_result_
BOOLEAN
_bittest (
    _In_reads_bytes_((Offset/8)+1) LONG const *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittestandcomplement (
    _Inout_updates_bytes_((Offset/8)+1) LONG *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittestandset (
    _Inout_updates_bytes_((Offset/8)+1) LONG *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_bittestandreset (
    _Inout_updates_bytes_((Offset/8)+1) LONG *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_interlockedbittestandset (
    _Inout_updates_bytes_((Offset/8)+1) _Interlocked_operand_ LONG volatile *Base,
    _In_range_(>=,0) LONG Offset
    );

BOOLEAN
_interlockedbittestandreset (
    _Inout_updates_bytes_((Offset/8)+1) _Interlocked_operand_ LONG volatile *Base,
    _In_range_(>=,0) LONG Offset
    );

#pragma intrinsic(_bittest)
#pragma intrinsic(_bittestandcomplement)
#pragma intrinsic(_bittestandset)
#pragma intrinsic(_bittestandreset)
#pragma intrinsic(_interlockedbittestandset)
#pragma intrinsic(_interlockedbittestandreset)

//
// Define bit scan intrinsics.
//

#define BitScanForward _BitScanForward
#define BitScanReverse _BitScanReverse

_Success_(return != 0)
BOOLEAN
_BitScanForward (
    _Out_ DWORD *Index,
    _In_ DWORD Mask
    );

_Success_(return != 0)
BOOLEAN
_BitScanReverse (
    _Out_ DWORD *Index,
    _In_ DWORD Mask
    );

#pragma intrinsic(_BitScanForward)
#pragma intrinsic(_BitScanReverse)

_Success_(return != 0)
FORCEINLINE
BOOLEAN
_InlineBitScanForward64 (
    _Out_ DWORD *Index,
    _In_ DWORD64 Mask
    )
{
    if (_BitScanForward(Index, (DWORD)Mask)) {
        return 1;
    }

    if (_BitScanForward(Index, (DWORD)(Mask >> 32))) {
        *Index += 32;
        return 1;
    }

    return 0;
}

#define BitScanForward64 _InlineBitScanForward64

_Success_(return != 0)
FORCEINLINE
BOOLEAN
_InlineBitScanReverse64 (
    _Out_ DWORD *Index,
    _In_ DWORD64 Mask
    )
{
    if (_BitScanReverse(Index, (DWORD)(Mask >> 32))) {
        *Index += 32;
        return 1;
    }

    if (_BitScanReverse(Index, (DWORD)Mask)) {
        return 1;
    }

    return 0;
}

#define BitScanReverse64 _InlineBitScanReverse64

#endif // !defined(_MANAGED)

//
// Interlocked intrinsic functions.
//

#if !defined(_MANAGED)

#define InterlockedIncrement16 _InterlockedIncrement16
#define InterlockedIncrementAcquire16 _InterlockedIncrement16
#define InterlockedIncrementRelease16 _InterlockedIncrement16
#define InterlockedIncrementNoFence16 _InterlockedIncrement16

#define InterlockedDecrement16 _InterlockedDecrement16
#define InterlockedDecrementAcquire16 _InterlockedDecrement16
#define InterlockedDecrementRelease16 _InterlockedDecrement16
#define InterlockedDecrementNoFence16 _InterlockedDecrement16

#define InterlockedCompareExchange16 _InterlockedCompareExchange16
#define InterlockedCompareExchangeAcquire16 _InterlockedCompareExchange16
#define InterlockedCompareExchangeRelease16 _InterlockedCompareExchange16
#define InterlockedCompareExchangeNoFence16 _InterlockedCompareExchange16

#define InterlockedCompareExchange64 _InterlockedCompareExchange64
#define InterlockedCompareExchangeAcquire64 _InterlockedCompareExchange64
#define InterlockedCompareExchangeRelease64 _InterlockedCompareExchange64
#define InterlockedCompareExchangeNoFence64 _InterlockedCompareExchange64

SHORT
InterlockedIncrement16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Addend
    );

SHORT
InterlockedDecrement16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Addend
    );

SHORT
InterlockedCompareExchange16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT ExChange,
    _In_ SHORT Comperand
    );

LONG64
InterlockedCompareExchange64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 ExChange,
    _In_ LONG64 Comperand
    );

#pragma intrinsic(_InterlockedIncrement16)
#pragma intrinsic(_InterlockedDecrement16)
#pragma intrinsic(_InterlockedCompareExchange16)
#pragma intrinsic(_InterlockedCompareExchange64)

#endif // !defined(_MANAGED)

#define InterlockedAnd _InterlockedAnd
#define InterlockedAndAcquire _InterlockedAnd
#define InterlockedAndRelease _InterlockedAnd
#define InterlockedAndNoFence _InterlockedAnd

#define InterlockedOr _InterlockedOr
#define InterlockedOrAcquire _InterlockedOr
#define InterlockedOrRelease _InterlockedOr
#define InterlockedOrNoFence _InterlockedOr

#define InterlockedXor _InterlockedXor
#define InterlockedXorAcquire _InterlockedXor
#define InterlockedXorRelease _InterlockedXor
#define InterlockedXorNoFence _InterlockedXor

#define InterlockedIncrement _InterlockedIncrement
#define InterlockedIncrementAcquire _InterlockedIncrement
#define InterlockedIncrementRelease _InterlockedIncrement
#define InterlockedIncrementNoFence _InterlockedIncrement

#define InterlockedDecrement _InterlockedDecrement
#define InterlockedDecrementAcquire _InterlockedDecrement
#define InterlockedDecrementRelease _InterlockedDecrement
#define InterlockedDecrementNoFence _InterlockedDecrement

#define InterlockedAdd _InlineInterlockedAdd
#define InterlockedAddAcquire _InlineInterlockedAdd
#define InterlockedAddRelease _InlineInterlockedAdd
#define InterlockedAddNoFence _InlineInterlockedAdd
#define InterlockedAddNoFence64 _InlineInterlockedAdd64

#define InterlockedExchange _InterlockedExchange
#define InterlockedExchangeAcquire _InterlockedExchange
#define InterlockedExchangeNoFence _InterlockedExchange

#define InterlockedExchangeAdd _InterlockedExchangeAdd
#define InterlockedExchangeAddAcquire _InterlockedExchangeAdd
#define InterlockedExchangeAddRelease _InterlockedExchangeAdd
#define InterlockedExchangeAddNoFence _InterlockedExchangeAdd

#define InterlockedCompareExchange _InterlockedCompareExchange
#define InterlockedCompareExchangeAcquire _InterlockedCompareExchange
#define InterlockedCompareExchangeRelease _InterlockedCompareExchange
#define InterlockedCompareExchangeNoFence _InterlockedCompareExchange

#define InterlockedExchange16 _InterlockedExchange16

LONG
InterlockedAnd (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG
InterlockedOr (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG
InterlockedXor (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG
CDECL_NON_WVMPURE
InterlockedIncrement (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend
    );

LONG
CDECL_NON_WVMPURE
InterlockedDecrement (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend
    );

LONG
__cdecl
InterlockedExchange (
    _Inout_ _Interlocked_operand_ LONG volatile *Target,
    _In_ LONG Value
    );

LONG
__cdecl
InterlockedExchangeAdd (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend,
    _In_ LONG Value
    );

FORCEINLINE
LONG
_InlineInterlockedAdd (
    _Inout_ _Interlocked_operand_ LONG volatile *Addend,
    _In_ LONG Value
    )

{

    return InterlockedExchangeAdd(Addend, Value) + Value;
}

LONG
CDECL_NON_WVMPURE
InterlockedCompareExchange (
    _Inout_ _Interlocked_operand_ LONG volatile * Destination,
    _In_ LONG ExChange,
    _In_ LONG Comperand
    );

#undef _InterlockedExchangePointer

FORCEINLINE
_Ret_writes_(_Inexpressible_(Unknown))
PVOID
_InlineInterlockedExchangePointer(
    _Inout_ _At_(*Destination,
        _Pre_writable_byte_size_(_Inexpressible_(Unknown))
        _Post_writable_byte_size_(_Inexpressible_(Unknown)))
    _Interlocked_operand_ PVOID volatile * Destination,
    _In_opt_ PVOID Value
    )
{
    return (PVOID)InterlockedExchange((LONG volatile *) Destination,
                                      (LONG) Value);
}

#define InterlockedExchangePointer _InlineInterlockedExchangePointer
#define InterlockedExchangePointerAcquire _InlineInterlockedExchangePointer
#define InterlockedExchangePointerRelease _InlineInterlockedExchangePointer
#define InterlockedExchangePointerNoFence _InlineInterlockedExchangePointer

FORCEINLINE
_Ret_writes_(_Inexpressible_(Unknown))
PVOID
_InlineInterlockedCompareExchangePointer (
    _Inout_ _At_(*Destination,
        _Pre_writable_byte_size_(_Inexpressible_(Unknown))
        _Post_writable_byte_size_(_Inexpressible_(Unknown)))
    _Interlocked_operand_ PVOID volatile * Destination,
    _In_opt_ PVOID ExChange,
    _In_opt_ PVOID Comperand
    )
{
    return (PVOID)InterlockedCompareExchange((LONG volatile *) Destination,
                                             (LONG) ExChange,
                                             (LONG) Comperand);
}

#define InterlockedCompareExchangePointer \
    _InlineInterlockedCompareExchangePointer
#define InterlockedCompareExchangePointerAcquire \
    _InlineInterlockedCompareExchangePointer
#define InterlockedCompareExchangePointerRelease \
    _InlineInterlockedCompareExchangePointer
#define InterlockedCompareExchangePointerNoFence \
    _InlineInterlockedCompareExchangePointer

#pragma intrinsic(_InterlockedAnd)
#pragma intrinsic(_InterlockedOr)
#pragma intrinsic(_InterlockedXor)
#pragma intrinsic(_InterlockedIncrement)
#pragma intrinsic(_InterlockedDecrement)
#pragma intrinsic(_InterlockedExchange)
#pragma intrinsic(_InterlockedExchangeAdd)
#pragma intrinsic(_InterlockedCompareExchange)

#if !defined(_MANAGED)

#if (_MSC_VER >= 1600)

#define InterlockedExchange8 _InterlockedExchange8
#define InterlockedExchangeNoFence8 InterlockedExchange8
#define InterlockedExchangeAcquire8 InterlockedExchange8

#define InterlockedExchange16 _InterlockedExchange16
#define InterlockedExchangeNoFence16 InterlockedExchange16
#define InterlockedExchangeAcquire16 InterlockedExchange16

CHAR
InterlockedExchange8 (
    _Inout_ _Interlocked_operand_ CHAR volatile *Target,
    _In_ CHAR Value
    );

SHORT
InterlockedExchange16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT ExChange
    );

#pragma intrinsic(_InterlockedExchange8)
#pragma intrinsic(_InterlockedExchange16)

#endif // _MSC_VER >= 1600

#if _MSC_FULL_VER >= 140041204

#define InterlockedExchangeAdd8 _InterlockedExchangeAdd8
#define InterlockedAnd8 _InterlockedAnd8
#define InterlockedAndAcquire8 _InterlockedAnd8
#define InterlockedAndRelease8 _InterlockedAnd8
#define InterlockedAndNoFence8 _InterlockedAnd8
#define InterlockedOr8 _InterlockedOr8
#define InterlockedOrAcquire8 _InterlockedOr8
#define InterlockedOrRelease8 _InterlockedOr8
#define InterlockedOrNoFence8 _InterlockedOr8
#define InterlockedXor8 _InterlockedXor8
#define InterlockedXorAcquire8 _InterlockedXor8
#define InterlockedXorRelease8 _InterlockedXor8
#define InterlockedXorNoFence8 _InterlockedXor8
#define InterlockedExchangeAdd16 _InterlockedExchangeAdd16
#define InterlockedAnd16 _InterlockedAnd16
#define InterlockedAndAcquire16 InterlockedAnd16
#define InterlockedAndRelease16 InterlockedAnd16
#define InterlockedAndNoFence16 InterlockedAnd16
#define InterlockedOr16 _InterlockedOr16
#define InterlockedOrAcquire16 InterlockedOr16
#define InterlockedOrRelease16 InterlockedOr16
#define InterlockedOrNoFence16 InterlockedOr16
#define InterlockedXor16 _InterlockedXor16
#define InterlockedXorAcquire16 InterlockedXor16
#define InterlockedXorRelease16 InterlockedXor16
#define InterlockedXorNoFence16 InterlockedXor16

char
InterlockedExchangeAdd8 (
    _Inout_ _Interlocked_operand_ char volatile * _Addend,
    _In_ char _Value
    );

char
InterlockedAnd8 (
    _Inout_ _Interlocked_operand_ char volatile *Destination,
    _In_ char Value
    );

char
InterlockedOr8 (
    _Inout_ _Interlocked_operand_ char volatile *Destination,
    _In_ char Value
    );

char
InterlockedXor8 (
    _Inout_ _Interlocked_operand_ char volatile *Destination,
    _In_ char Value
    );

SHORT
_InterlockedAnd16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

SHORT
InterlockedXor16(
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

SHORT
_InterlockedCompareExchange16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT ExChange,
    _In_ SHORT Comperand
    );

SHORT
_InterlockedOr16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

SHORT
_InterlockedIncrement16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination
    );

SHORT
_InterlockedDecrement16 (
    _Inout_ _Interlocked_operand_ SHORT volatile *Destination
    );

#pragma intrinsic (_InterlockedExchangeAdd8)
#pragma intrinsic (_InterlockedAnd8)
#pragma intrinsic (_InterlockedOr8)
#pragma intrinsic (_InterlockedXor8)
#pragma intrinsic (_InterlockedAnd16)
#pragma intrinsic (_InterlockedOr16)
#pragma intrinsic (_InterlockedXor16)
#pragma intrinsic (_InterlockedCompareExchange16)
#pragma intrinsic (_InterlockedIncrement16)
#pragma intrinsic (_InterlockedDecrement16)

#endif  /* _MSC_FULL_VER >= 140040816 */

//
// Define 64-bit operations in terms of InterlockedCompareExchange64
//

#define InterlockedCompareExchange64 _InterlockedCompareExchange64


FORCEINLINE
LONG64
_InlineInterlockedAnd64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )
{
    LONG64 Old;

    do {
        Old = *Destination;
    } while (InterlockedCompareExchange64(Destination,
                                          Old & Value,
                                          Old) != Old);

    return Old;
}

#define InterlockedAnd64 _InlineInterlockedAnd64
#define InterlockedAnd64Acquire _InlineInterlockedAnd64
#define InterlockedAnd64Release _InlineInterlockedAnd64
#define InterlockedAnd64NoFence _InlineInterlockedAnd64

FORCEINLINE
LONG64
_InlineInterlockedAdd64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Addend,
    _In_ LONG64 Value
    )
{
    LONG64 Old;

    do {
        Old = *Addend;
    } while (InterlockedCompareExchange64(Addend,
                                          Old + Value,
                                          Old) != Old);

    return Old + Value;
}

#define InterlockedAdd64 _InlineInterlockedAdd64
#define InterlockedAddAcquire64 _InlineInterlockedAdd64
#define InterlockedAddRelease64 _InlineInterlockedAdd64
#define InterlockedAddNoFence64 _InlineInterlockedAdd64


#endif // !defined(_MANAGED)

#define InterlockedExchangeAddSizeT(a, b) InterlockedExchangeAdd((LONG *)a, b)
#define InterlockedExchangeAddSizeTAcquire(a, b) InterlockedExchangeAdd((LONG *)a, b)
#define InterlockedExchangeAddSizeTNoFence(a, b) InterlockedExchangeAdd((LONG *)a, b)
#define InterlockedIncrementSizeT(a) InterlockedIncrement((LONG *)a)
#define InterlockedIncrementSizeTNoFence(a) InterlockedIncrement((LONG *)a)
#define InterlockedDecrementSizeT(a) InterlockedDecrement((LONG *)a)
#define InterlockedDecrementSizeTNoFence(a) InterlockedDecrement((LONG *)a)

//
// Definitons below
//

LONG
_InterlockedXor (
    _Inout_ _Interlocked_operand_ LONG volatile *Target,
    _In_ LONG Set
    );

#pragma intrinsic(_InterlockedXor)

#define InterlockedXor _InterlockedXor

#if !defined(_MANAGED)

LONGLONG
FORCEINLINE
_InlineInterlockedOr64 (
    _Inout_ _Interlocked_operand_ LONGLONG volatile *Destination,
    _In_ LONGLONG Value
    )
{
    LONGLONG Old;

    do {
        Old = *Destination;
    } while (InterlockedCompareExchange64(Destination,
                                          Old | Value,
                                          Old) != Old);

    return Old;
}

#define InterlockedOr64 _InlineInterlockedOr64

FORCEINLINE
LONG64
_InlineInterlockedXor64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )
{
    LONG64 Old;

    do {
        Old = *Destination;
    } while (InterlockedCompareExchange64(Destination,
                                          Old ^ Value,
                                          Old) != Old);

    return Old;
}

#define InterlockedXor64 _InlineInterlockedXor64

LONGLONG
FORCEINLINE
_InlineInterlockedIncrement64 (
    _Inout_ _Interlocked_operand_ LONGLONG volatile *Addend
    )
{
    LONGLONG Old;

    do {
        Old = *Addend;
    } while (InterlockedCompareExchange64(Addend,
                                          Old + 1,
                                          Old) != Old);

    return Old + 1;
}

#define InterlockedIncrement64 _InlineInterlockedIncrement64
#define InterlockedIncrementAcquire64 InterlockedIncrement64
#define InterlockedIncrementRelease64 InterlockedIncrement64
#define InterlockedIncrementNoFence64 InterlockedIncrement64

FORCEINLINE
LONGLONG
_InlineInterlockedDecrement64 (
    _Inout_ _Interlocked_operand_ LONGLONG volatile *Addend
    )
{
    LONGLONG Old;

    do {
        Old = *Addend;
    } while (InterlockedCompareExchange64(Addend,
                                          Old - 1,
                                          Old) != Old);

    return Old - 1;
}

#define InterlockedDecrement64 _InlineInterlockedDecrement64
#define InterlockedDecrementAcquire64 InterlockedDecrement64
#define InterlockedDecrementRelease64 InterlockedDecrement64
#define InterlockedDecrementNoFence64 InterlockedDecrement64

FORCEINLINE
LONGLONG
_InlineInterlockedExchange64 (
    _Inout_ _Interlocked_operand_ LONGLONG volatile *Target,
    _In_ LONGLONG Value
    )
{
    LONGLONG Old;

    do {
        Old = *Target;
    } while (InterlockedCompareExchange64(Target,
                                          Value,
                                          Old) != Old);

    return Old;
}

#define InterlockedExchange64 _InlineInterlockedExchange64
#define InterlockedExchangeAcquire64 InterlockedExchange64
#define InterlockedExchangeNoFence64 _InlineInterlockedExchange64


FORCEINLINE
LONGLONG
_InlineInterlockedExchangeAdd64 (
    _Inout_ _Interlocked_operand_ LONGLONG volatile *Addend,
    _In_ LONGLONG Value
    )
{
    LONGLONG Old;

    do {
        Old = *Addend;
    } while (InterlockedCompareExchange64(Addend,
                                          Old + Value,
                                          Old) != Old);

    return Old;
}

#define InterlockedExchangeAdd64 _InlineInterlockedExchangeAdd64
#define InterlockedExchangeAddNoFence64 _InlineInterlockedExchangeAdd64

//
// FS relative adds and increments.
//

VOID
__incfsbyte (
    _In_ DWORD Offset
    );

VOID
__addfsbyte (
    _In_ DWORD Offset,
    _In_ BYTE  Value
    );

VOID
__incfsword (
    _In_ DWORD Offset
    );

VOID
__addfsword (
    _In_ DWORD Offset,
    _In_ WORD   Value
    );

VOID
__incfsdword (
    _In_ DWORD Offset
    );

VOID
__addfsdword (
    _In_ DWORD Offset,
    _In_ DWORD Value
    );

#pragma intrinsic(__incfsbyte)
#pragma intrinsic(__addfsbyte)
#pragma intrinsic(__incfsword)
#pragma intrinsic(__addfsword)
#pragma intrinsic(__incfsdword)
#pragma intrinsic(__addfsdword)

#endif // !defined(_MANAGED)

#if !defined(_M_CEE_PURE)

// end_ntoshvp

#if _MSC_VER >= 1500

//
// Define extended CPUID intrinsic.
//

#define CpuIdEx __cpuidex

VOID
__cpuidex (
    int CPUInfo[4],
    int Function,
    int SubLeaf
    );

#pragma intrinsic(__cpuidex)

#endif

// begin_ntoshvp

//
// Define FS read/write intrinsics
//

BYTE 
__readfsbyte (
    _In_ DWORD Offset
    );

WORD  
__readfsword (
    _In_ DWORD Offset
    );

DWORD
__readfsdword (
    _In_ DWORD Offset
    );

VOID
__writefsbyte (
    _In_ DWORD Offset,
    _In_ BYTE  Data
    );

VOID
__writefsword (
    _In_ DWORD Offset,
    _In_ WORD   Data
    );

VOID
__writefsdword (
    _In_ DWORD Offset,
    _In_ DWORD Data
    );

#pragma intrinsic(__readfsbyte)
#pragma intrinsic(__readfsword)
#pragma intrinsic(__readfsdword)
#pragma intrinsic(__writefsbyte)
#pragma intrinsic(__writefsword)
#pragma intrinsic(__writefsdword)

#if !defined(_M_HYBRID_X86_ARM64)

VOID
_mm_lfence (
    VOID
    );

#pragma intrinsic(_mm_lfence)

#define SpeculationFence _mm_lfence

#endif // !defined(_M_HYBRID_x86_ARM64)

VOID
_ReadWriteBarrier (
    VOID
    );

#pragma intrinsic(_ReadWriteBarrier)

#endif // !defined(_M_CEE_PURE)


#if !defined(_MANAGED) && !defined(_M_HYBRID_X86_ARM64)
VOID
_mm_pause (
    VOID
    );

#pragma intrinsic(_mm_pause)

#define YieldProcessor _mm_pause

#endif // !defined(_MANAGED) && !defined(_M_HYBRID_X86_ARM64)

#ifdef __cplusplus
}
#endif

#endif  /* !defined(MIDL_PASS) || defined(_M_IX86) */

// end_ntoshvp
// begin_ntoshvp
#if !defined(MIDL_PASS) && defined(_M_IX86)

#if !defined(_M_CEE_PURE) && !defined(_M_HYBRID_X86_ARM64)

#pragma prefast(push)
#pragma warning(push)
#pragma prefast(disable: 6001 28113, "The barrier variable is accessed only to create a side effect.")
#pragma warning(disable: 4793)
FORCEINLINE
VOID
MemoryBarrier (
    VOID
    )
{
    LONG Barrier;

    InterlockedOr(&Barrier, 0);
    return;
}

#pragma warning(pop)
#pragma prefast(pop)

#endif /* !_M_CEE_PURE || !_M_HYBRID_X86_ARM64*/

#if !defined(_M_HYBRID_X86_ARM64)

//
// Define constants for use with _mm_prefetch.
//

#define _MM_HINT_T0     1
#define _MM_HINT_T1     2
#define _MM_HINT_T2     3
#define _MM_HINT_NTA    0

VOID
_mm_prefetch (
    _In_ CHAR CONST *a,
    _In_ int sel
    );

#pragma intrinsic(_mm_prefetch)

//
// PreFetchCacheLine level defines.
//

#define PF_TEMPORAL_LEVEL_1 _MM_HINT_T0
#define PF_TEMPORAL_LEVEL_2 _MM_HINT_T1
#define PF_TEMPORAL_LEVEL_3 _MM_HINT_T2
#define PF_NON_TEMPORAL_LEVEL_ALL _MM_HINT_NTA

#define PreFetchCacheLine(l, a)  _mm_prefetch((CHAR CONST *) a, l)
#define PrefetchForWrite(p)
#define ReadForWriteAccess(p) (*(p))

#if !defined(_MANAGED)

//
// Define function to read the value of a performance counter.
//

#define ReadPMC __readpmc

DWORD64
__readpmc (
    _In_ DWORD Counter
    );

#pragma intrinsic(__readpmc)

//
// Define function to read the value of the time stamp counter
//

#define ReadTimeStampCounter() __rdtsc()

DWORD64
__rdtsc (
    VOID
    );

#pragma intrinsic(__rdtsc)

#endif // !defined(_MANAGED)

#endif // !defined(_M_HYBRID_X86_ARM64)

// end_ntoshvp
// end_ntddk

#if !defined(_MANAGED)

__inline PVOID GetFiberData( void )    { return *(PVOID *) (ULONG_PTR) __readfsdword (0x10);}
__inline PVOID GetCurrentFiber( void ) { return (PVOID) (ULONG_PTR) __readfsdword (0x10);}

#endif // !defined(_MANAGED)

// begin_ntddk
// begin_ntoshvp
#endif // !defined(MIDL_PASS) && defined(_M_IX86)
// end_ntoshvp
// end_ntddk

//
// The following values specify the type of failing access when the status is
// STATUS_ACCESS_VIOLATION and the first parameter in the execpetion record.
//

#define EXCEPTION_READ_FAULT          0 // Access violation was caused by a read
#define EXCEPTION_WRITE_FAULT         1 // Access violation was caused by a write
#define EXCEPTION_EXECUTE_FAULT       8 // Access violation was caused by an instruction fetch

// begin_wx86
// begin_ntddk
// begin_ntoshvp

//
//  Define the size of the 80387 save area, which is in the context frame.
//

#define SIZE_OF_80387_REGISTERS      80

//
// The following flags control the contents of the CONTEXT structure.
//

#if !defined(RC_INVOKED)

#define CONTEXT_i386    0x00010000L    // this assumes that i386 and
#define CONTEXT_i486    0x00010000L    // i486 have identical context records

// end_wx86

#define CONTEXT_CONTROL         (CONTEXT_i386 | 0x00000001L) // SS:SP, CS:IP, FLAGS, BP
#define CONTEXT_INTEGER         (CONTEXT_i386 | 0x00000002L) // AX, BX, CX, DX, SI, DI
#define CONTEXT_SEGMENTS        (CONTEXT_i386 | 0x00000004L) // DS, ES, FS, GS
#define CONTEXT_FLOATING_POINT  (CONTEXT_i386 | 0x00000008L) // 387 state
#define CONTEXT_DEBUG_REGISTERS (CONTEXT_i386 | 0x00000010L) // DB 0-3,6,7
#define CONTEXT_EXTENDED_REGISTERS  (CONTEXT_i386 | 0x00000020L) // cpu specific extensions

#define CONTEXT_FULL (CONTEXT_CONTROL | CONTEXT_INTEGER |\
                      CONTEXT_SEGMENTS)

#define CONTEXT_ALL             (CONTEXT_CONTROL | CONTEXT_INTEGER | CONTEXT_SEGMENTS | \
                                 CONTEXT_FLOATING_POINT | CONTEXT_DEBUG_REGISTERS | \
                                 CONTEXT_EXTENDED_REGISTERS)

#define CONTEXT_XSTATE          (CONTEXT_i386 | 0x00000040L)

#define CONTEXT_EXCEPTION_ACTIVE    0x08000000L
#define CONTEXT_SERVICE_ACTIVE      0x10000000L
#define CONTEXT_EXCEPTION_REQUEST   0x40000000L
#define CONTEXT_EXCEPTION_REPORTING 0x80000000L

// begin_wx86

#endif // !defined(RC_INVOKED)

typedef struct _FLOATING_SAVE_AREA {
    DWORD   ControlWord;
    DWORD   StatusWord;
    DWORD   TagWord;
    DWORD   ErrorOffset;
    DWORD   ErrorSelector;
    DWORD   DataOffset;
    DWORD   DataSelector;
    BYTE    RegisterArea[SIZE_OF_80387_REGISTERS];
    DWORD   Spare0;
} FLOATING_SAVE_AREA;

typedef FLOATING_SAVE_AREA *PFLOATING_SAVE_AREA;


// end_ntddk
// begin_wdm begin_ntosp

#define MAXIMUM_SUPPORTED_EXTENSION     512

#if !defined(__midl) && !defined(MIDL_PASS)

C_ASSERT(sizeof(XSAVE_FORMAT) == MAXIMUM_SUPPORTED_EXTENSION);

#endif

// end_wdm end_ntosp
// begin_ntddk

#include "pshpack4.h"

//
// Context Frame
//
//  This frame has a several purposes: 1) it is used as an argument to
//  NtContinue, 2) is is used to constuct a call frame for APC delivery,
//  and 3) it is used in the user level thread creation routines.
//
//  The layout of the record conforms to a standard call frame.
//

typedef struct DECLSPEC_NOINITALL _CONTEXT {

    //
    // The flags values within this flag control the contents of
    // a CONTEXT record.
    //
    // If the context record is used as an input parameter, then
    // for each portion of the context record controlled by a flag
    // whose value is set, it is assumed that that portion of the
    // context record contains valid context. If the context record
    // is being used to modify a threads context, then only that
    // portion of the threads context will be modified.
    //
    // If the context record is used as an IN OUT parameter to capture
    // the context of a thread, then only those portions of the thread's
    // context corresponding to set flags will be returned.
    //
    // The context record is never used as an OUT only parameter.
    //

    DWORD ContextFlags;

    //
    // This section is specified/returned if CONTEXT_DEBUG_REGISTERS is
    // set in ContextFlags.  Note that CONTEXT_DEBUG_REGISTERS is NOT
    // included in CONTEXT_FULL.
    //

    DWORD   Dr0;
    DWORD   Dr1;
    DWORD   Dr2;
    DWORD   Dr3;
    DWORD   Dr6;
    DWORD   Dr7;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_FLOATING_POINT.
    //

    FLOATING_SAVE_AREA FloatSave;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_SEGMENTS.
    //

    DWORD   SegGs;
    DWORD   SegFs;
    DWORD   SegEs;
    DWORD   SegDs;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_INTEGER.
    //

    DWORD   Edi;
    DWORD   Esi;
    DWORD   Ebx;
    DWORD   Edx;
    DWORD   Ecx;
    DWORD   Eax;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_CONTROL.
    //

    DWORD   Ebp;
    DWORD   Eip;
    DWORD   SegCs;              // MUST BE SANITIZED
    DWORD   EFlags;             // MUST BE SANITIZED
    DWORD   Esp;
    DWORD   SegSs;

    //
    // This section is specified/returned if the ContextFlags word
    // contains the flag CONTEXT_EXTENDED_REGISTERS.
    // The format and contexts are processor specific
    //

    BYTE    ExtendedRegisters[MAXIMUM_SUPPORTED_EXTENSION];

} CONTEXT;

typedef CONTEXT *PCONTEXT;

#include "poppack.h"

// end_ntoshvp
// begin_ntminiport
#endif //_X86_


#ifndef _LDT_ENTRY_DEFINED
#define _LDT_ENTRY_DEFINED

typedef struct _LDT_ENTRY {
    WORD    LimitLow;
    WORD    BaseLow;
    union {
        struct {
            BYTE    BaseMid;
            BYTE    Flags1;     // Declare as bytes to avoid alignment
            BYTE    Flags2;     // Problems.
            BYTE    BaseHi;
        } Bytes;
        struct {
            DWORD   BaseMid : 8;
            DWORD   Type : 5;
            DWORD   Dpl : 2;
            DWORD   Pres : 1;
            DWORD   LimitHi : 4;
            DWORD   Sys : 1;
            DWORD   Reserved_0 : 1;
            DWORD   Default_Big : 1;
            DWORD   Granularity : 1;
            DWORD   BaseHi : 8;
        } Bits;
    } HighWord;
} LDT_ENTRY, *PLDT_ENTRY;

#endif


#if defined(_X86_)

typedef struct _KNONVOLATILE_CONTEXT_POINTERS {
    DWORD   Dummy;
} KNONVOLATILE_CONTEXT_POINTERS, *PKNONVOLATILE_CONTEXT_POINTERS;

#endif // defined(_X86_)

#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// begin_wdm begin_ntminiport

#if !defined(RC_INVOKED) && !defined(MIDL_PASS)
#if ((defined(_M_AMD64) || defined(_M_IX86)) && !defined(_M_HYBRID_X86_ARM64) && !defined(_M_ARM64EC)) || defined(_M_CEE_PURE)

#ifdef __cplusplus
extern "C" {
#endif

FORCEINLINE
CHAR
ReadAcquire8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

    Value = *Source;
    return Value;
}

FORCEINLINE
CHAR
ReadNoFence8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

    Value = *Source;
    return Value;
}

FORCEINLINE
VOID
WriteRelease8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
VOID
WriteNoFence8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
SHORT
ReadAcquire16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

    Value = *Source;
    return Value;
}

FORCEINLINE
SHORT
ReadNoFence16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

    Value = *Source;
    return Value;
}

FORCEINLINE
VOID
WriteRelease16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
VOID
WriteNoFence16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
LONG
ReadAcquire (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

    Value = *Source;
    return Value;
}

CFORCEINLINE
LONG
ReadNoFence (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

    Value = *Source;
    return Value;
}

CFORCEINLINE
VOID
WriteRelease (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
VOID
WriteNoFence (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
LONG64
ReadAcquire64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

    Value = *Source;
    return Value;
}

CFORCEINLINE
LONG64
ReadNoFence64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

    Value = *Source;
    return Value;
}

CFORCEINLINE
VOID
WriteRelease64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

    *Destination = Value;
    return;
}

FORCEINLINE
VOID
WriteNoFence64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

    *Destination = Value;
    return;
}

#if !defined(_M_CEE_PURE)

FORCEINLINE
VOID
BarrierAfterRead (
    VOID
    )

{
    _ReadWriteBarrier();
    return;
}

#endif

#ifdef __cplusplus
}
#endif

#endif // ((defined(_M_AMD64) || defined(_M_IX86)) && !defined(_M_HYBRID_X86_ARM64) && !defined(_M_ARM64EC)) || defined(_M_CEE_PURE)

//
// Define "raw" operations which have no ordering or atomicity semantics.
//

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_KERNEL_MODE) && defined(__SANITIZE_ADDRESS__) && defined(CSAN_ON_ASAN)

#define ReadRaw8        CsanRead8NoCheck
#define WriteRaw8       CsanWrite8NoCheck
#define ReadRaw16       CsanRead16NoCheck
#define WriteRaw16      CsanWrite16NoCheck
#define ReadRaw         CsanReadNoCheck
#define WriteRaw        CsanWriteNoCheck
#define ReadRaw64       CsanRead64NoCheck
#define WriteRaw64      CsanWrite64NoCheck

CHAR
ReadRaw8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    );

VOID
WriteRaw8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    );

SHORT
ReadRaw16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    );

VOID
WriteRaw16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    );

LONG
ReadRaw (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    );

VOID
WriteRaw (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    );

LONG64
ReadRaw64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    );

VOID
WriteRaw64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    );

#else

FORCEINLINE
CHAR
ReadRaw8 (
    _In_ _Interlocked_operand_ CHAR const volatile *Source
    )

{

    CHAR Value;

    Value = *(CHAR *)Source;
    return Value;
}

FORCEINLINE
VOID
WriteRaw8 (
    _Out_ _Interlocked_operand_ CHAR volatile *Destination,
    _In_ CHAR Value
    )

{

    *(CHAR *)Destination = Value;
    return;
}

FORCEINLINE
SHORT
ReadRaw16 (
    _In_ _Interlocked_operand_ SHORT const volatile *Source
    )

{

    SHORT Value;

    Value = *(SHORT *)Source;
    return Value;
}

FORCEINLINE
VOID
WriteRaw16 (
    _Out_ _Interlocked_operand_ SHORT volatile *Destination,
    _In_ SHORT Value
    )

{

    *(SHORT *)Destination = Value;
    return;
}

FORCEINLINE
LONG
ReadRaw (
    _In_ _Interlocked_operand_ LONG const volatile *Source
    )

{

    LONG Value;

    Value = *(LONG *)Source;
    return Value;
}

CFORCEINLINE
VOID
WriteRaw (
    _Out_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{

    *(LONG *)Destination = Value;
    return;
}

FORCEINLINE
LONG64
ReadRaw64 (
    _In_ _Interlocked_operand_ LONG64 const volatile *Source
    )

{

    LONG64 Value;

    Value = *(LONG64 *)Source;
    return Value;
}

FORCEINLINE
VOID
WriteRaw64 (
    _Out_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{

    *(LONG64 *)Destination = Value;
    return;
}

#endif // _KERNEL_MODE && __SANITIZE_ADDRESS__ && CSAN_ON_ASAN

#ifdef __cplusplus
}
#endif

FORCEINLINE
LONG
AddRaw (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination,
    _In_ LONG Value
    )

{
    LONG NewValue;

    NewValue = ReadRaw(Destination);
    NewValue += Value;
    WriteRaw(Destination, NewValue);

    return NewValue;
}

FORCEINLINE
DWORD
AddULongRaw (
    _Inout_ _Interlocked_operand_ DWORD volatile *Destination,
    _In_ DWORD Value
    )

{
    return (DWORD)AddRaw((PLONG)Destination, (LONG)Value);
}

FORCEINLINE
LONG
IncrementRaw (
    _Inout_ _Interlocked_operand_ LONG volatile *Destination
    )

{
    return AddRaw(Destination, 1);
}

FORCEINLINE
DWORD
IncrementULongRaw (
    _Inout_ _Interlocked_operand_ DWORD volatile *Destination
    )

{
    return (DWORD)IncrementRaw((PLONG)Destination);
}

//
// Define explicit read and write operations for derived types.
//

FORCEINLINE
BYTE 
ReadUCharAcquire (
    _In_ _Interlocked_operand_ BYTE  const volatile *Source
    )

{

    return (BYTE )ReadAcquire8((PCHAR)Source);
}

FORCEINLINE
BYTE 
ReadUCharNoFence (
    _In_ _Interlocked_operand_ BYTE  const volatile *Source
    )

{

    return (BYTE )ReadNoFence8((PCHAR)Source);
}

FORCEINLINE
BYTE 
ReadBooleanAcquire (
    _In_ _Interlocked_operand_ BOOLEAN const volatile *Source
    )

{

    return (BOOLEAN)ReadAcquire8((PCHAR)Source);
}

FORCEINLINE
BYTE 
ReadBooleanNoFence (
    _In_ _Interlocked_operand_ BOOLEAN const volatile *Source
    )

{

    return (BOOLEAN)ReadNoFence8((PCHAR)Source);
}

FORCEINLINE
BYTE 
ReadBooleanRaw (
    _In_ _Interlocked_operand_ BOOLEAN const volatile *Source
    )

{
    return (BOOLEAN)ReadRaw8((PCHAR)Source);
}

FORCEINLINE
BYTE 
ReadUCharRaw (
    _In_ _Interlocked_operand_ BYTE  const volatile *Source
    )

{

    return (BYTE )ReadRaw8((PCHAR)Source);
}

FORCEINLINE
VOID
WriteUCharRelease (
    _Out_ _Interlocked_operand_ BYTE  volatile *Destination,
    _In_ BYTE  Value
    )

{

    WriteRelease8((PCHAR)Destination, (CHAR)Value);
    return;
}

FORCEINLINE
VOID
WriteUCharNoFence (
    _Out_ _Interlocked_operand_ BYTE  volatile *Destination,
    _In_ BYTE  Value
    )

{

    WriteNoFence8((PCHAR)Destination, (CHAR)Value);
    return;
}

FORCEINLINE
VOID
WriteBooleanRelease (
    _Out_ _Interlocked_operand_ BOOLEAN volatile *Destination,
    _In_ BOOLEAN Value
    )

{

    WriteRelease8((PCHAR)Destination, (CHAR)Value);
    return;
}

FORCEINLINE
VOID
WriteBooleanNoFence (
    _Out_ _Interlocked_operand_ BOOLEAN volatile *Destination,
    _In_ BOOLEAN Value
    )

{

    WriteNoFence8((PCHAR)Destination, (CHAR)Value);
    return;
}

FORCEINLINE
VOID
WriteUCharRaw (
    _Out_ _Interlocked_operand_ BYTE  volatile *Destination,
    _In_ BYTE  Value
    )

{

    WriteRaw8((PCHAR)Destination, (CHAR)Value);
    return;
}

FORCEINLINE
WORD  
ReadUShortAcquire (
    _In_ _Interlocked_operand_ WORD   const volatile *Source
    )

{

    return (WORD  )ReadAcquire16((PSHORT)Source);
}

FORCEINLINE
WORD  
ReadUShortNoFence (
    _In_ _Interlocked_operand_ WORD   const volatile *Source
    )

{

    return (WORD  )ReadNoFence16((PSHORT)Source);
}

FORCEINLINE
WORD  
ReadUShortRaw (
    _In_ _Interlocked_operand_ WORD   const volatile *Source
    )

{

    return (WORD  )ReadRaw16((PSHORT)Source);
}

FORCEINLINE
VOID
WriteUShortRelease (
    _Out_ _Interlocked_operand_ WORD   volatile *Destination,
    _In_ WORD   Value
    )

{

    WriteRelease16((PSHORT)Destination, (SHORT)Value);
    return;
}

FORCEINLINE
VOID
WriteUShortNoFence (
    _Out_ _Interlocked_operand_ WORD   volatile *Destination,
    _In_ WORD   Value
    )

{

    WriteNoFence16((PSHORT)Destination, (SHORT)Value);
    return;
}

FORCEINLINE
VOID
WriteUShortRaw (
    _Out_ _Interlocked_operand_ WORD   volatile *Destination,
    _In_ WORD   Value
    )

{

    WriteRaw16((PSHORT)Destination, (SHORT)Value);
    return;
}

FORCEINLINE
DWORD
ReadULongAcquire (
    _In_ _Interlocked_operand_ DWORD const volatile *Source
    )

{

    return (DWORD)ReadAcquire((PLONG)Source);
}

FORCEINLINE
DWORD
ReadULongNoFence (
    _In_ _Interlocked_operand_ DWORD const volatile *Source
    )

{

    return (DWORD)ReadNoFence((PLONG)Source);
}

FORCEINLINE
DWORD
ReadULongRaw (
    _In_ _Interlocked_operand_ DWORD const volatile *Source
    )

{

    return (DWORD)ReadRaw((PLONG)Source);
}

CFORCEINLINE
VOID
WriteULongRelease (
    _Out_ _Interlocked_operand_ DWORD volatile *Destination,
    _In_ DWORD Value
    )

{

    WriteRelease((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WriteULongNoFence (
    _Out_ _Interlocked_operand_ DWORD volatile *Destination,
    _In_ DWORD Value
    )

{

    WriteNoFence((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WriteULongRaw (
    _Out_ _Interlocked_operand_ DWORD volatile *Destination,
    _In_ DWORD Value
    )

{

    WriteRaw((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
INT32
ReadInt32Acquire (
    _In_ _Interlocked_operand_ INT32 const volatile *Source
    )

{

    return (INT32)ReadAcquire((PLONG)Source);
}

FORCEINLINE
INT32
ReadInt32NoFence (
    _In_ _Interlocked_operand_ INT32 const volatile *Source
    )

{

    return (INT32)ReadNoFence((PLONG)Source);
}

FORCEINLINE
INT32
ReadInt32Raw (
    _In_ _Interlocked_operand_ INT32 const volatile *Source
    )

{

    return (INT32)ReadRaw((PLONG)Source);
}

CFORCEINLINE
VOID
WriteInt32Release (
    _Out_ _Interlocked_operand_ INT32 volatile *Destination,
    _In_ INT32 Value
    )

{

    WriteRelease((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WriteInt32NoFence (
    _Out_ _Interlocked_operand_ INT32 volatile *Destination,
    _In_ INT32 Value
    )

{

    WriteNoFence((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WriteInt32Raw (
    _Out_ _Interlocked_operand_ INT32 volatile *Destination,
    _In_ INT32 Value
    )

{

    WriteRaw((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
UINT32
ReadUInt32Acquire (
    _In_ _Interlocked_operand_ UINT32 const volatile *Source
    )

{

    return (UINT32)ReadAcquire((PLONG)Source);
}

FORCEINLINE
UINT32
ReadUInt32NoFence (
    _In_ _Interlocked_operand_ UINT32 const volatile *Source
    )

{

    return (UINT32)ReadNoFence((PLONG)Source);
}

FORCEINLINE
UINT32
ReadUInt32Raw (
    _In_ _Interlocked_operand_ UINT32 const volatile *Source
    )

{

    return (UINT32)ReadRaw((PLONG)Source);
}

CFORCEINLINE
VOID
WriteUInt32Release (
    _Out_ _Interlocked_operand_ UINT32 volatile *Destination,
    _In_ UINT32 Value
    )

{

    WriteRelease((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WriteUInt32NoFence (
    _Out_ _Interlocked_operand_ UINT32 volatile *Destination,
    _In_ UINT32 Value
    )

{

    WriteNoFence((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WriteUInt32Raw (
    _Out_ _Interlocked_operand_ UINT32 volatile *Destination,
    _In_ UINT32 Value
    )

{

    WriteRaw((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
DWORD64
ReadULong64Acquire (
    _In_ _Interlocked_operand_ DWORD64 const volatile *Source
    )

{

    return (DWORD64)ReadAcquire64((PLONG64)Source);
}

FORCEINLINE
DWORD64
ReadULong64NoFence (
    _In_ _Interlocked_operand_ DWORD64 const volatile *Source
    )

{

    return (DWORD64)ReadNoFence64((PLONG64)Source);
}

FORCEINLINE
DWORD64
ReadULong64Raw (
    _In_ _Interlocked_operand_ DWORD64 const volatile *Source
    )

{

    return (DWORD64)ReadRaw64((PLONG64)Source);
}

CFORCEINLINE
VOID
WriteULong64Release (
    _Out_ _Interlocked_operand_ DWORD64 volatile *Destination,
    _In_ DWORD64 Value
    )

{

    WriteRelease64((PLONG64)Destination, (LONG64)Value);
    return;
}

FORCEINLINE
VOID
WriteULong64NoFence (
    _Out_ _Interlocked_operand_ DWORD64 volatile *Destination,
    _In_ DWORD64 Value
    )

{

    WriteNoFence64((PLONG64)Destination, (LONG64)Value);
    return;
}

FORCEINLINE
VOID
WriteULong64Raw (
    _Out_ _Interlocked_operand_ DWORD64 volatile *Destination,
    _In_ DWORD64 Value
    )

{

    WriteRaw64((PLONG64)Destination, (LONG64)Value);
    return;
}

FORCEINLINE
LONG64
AddRaw64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination,
    _In_ LONG64 Value
    )

{
    LONG64 NewValue;

    NewValue = ReadRaw64(Destination);
    NewValue += Value;
    WriteRaw64(Destination, NewValue);

    return NewValue;
}

FORCEINLINE
DWORD64
AddULong64Raw (
    _Inout_ _Interlocked_operand_ DWORD64 volatile *Destination,
    _In_ DWORD64 Value
    )

{
    return (DWORD64)AddRaw64((PLONG64)Destination, (LONG64)Value);
}

FORCEINLINE
LONG64
IncrementRaw64 (
    _Inout_ _Interlocked_operand_ LONG64 volatile *Destination
    )

{
    return AddRaw64(Destination, 1);
}

FORCEINLINE
DWORD64
IncrementULong64Raw (
    _Inout_ _Interlocked_operand_ DWORD64 volatile *Destination
    )

{
    return (DWORD64)IncrementRaw64((PLONG64)Destination);
}

#define ReadSizeTAcquire ReadULongPtrAcquire

#define ReadSizeTNoFence ReadULongPtrNoFence

#define ReadSizeTRaw ReadULongPtrRaw

#define WriteSizeTRelease WriteULongPtrRelease

#define WriteSizeTNoFence WriteULongPtrNoFence

#define WriteSizeTRaw WriteULongPtrRaw

#if !defined(_WIN64)

FORCEINLINE
PVOID
ReadPointerAcquire (
    _In_ _Interlocked_operand_ PVOID const volatile *Source
    )

{

    return (PVOID)ReadAcquire((PLONG)Source);
}

CFORCEINLINE
PVOID
ReadPointerNoFence (
    _In_ _Interlocked_operand_ PVOID const volatile *Source
    )

{

    return (PVOID)ReadNoFence((PLONG)Source);
}

FORCEINLINE
PVOID
ReadPointerRaw (
    _In_ _Interlocked_operand_ PVOID const volatile *Source
    )

{

    return (PVOID)ReadRaw((PLONG)Source);
}

CFORCEINLINE
VOID
WritePointerRelease (
    _Out_ _Interlocked_operand_ PVOID volatile *Destination,
    _In_ PVOID Value
    )

{

    WriteRelease((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WritePointerNoFence (
    _Out_ _Interlocked_operand_ PVOID volatile *Destination,
    _In_opt_ PVOID Value
    )

{

    WriteNoFence((PLONG)Destination, (LONG)Value);
    return;
}

FORCEINLINE
VOID
WritePointerRaw (
    _Out_ _Interlocked_operand_ PVOID volatile *Destination,
    _In_opt_ PVOID Value
    )

{

    WriteRaw((PLONG)Destination, (LONG)Value);
    return;
}

#define ReadLongPtrAcquire ReadAcquire

#define ReadLongPtrNoFence ReadNoFence

#define ReadLongPtrRaw ReadRaw

#define WriteLongPtrRelease WriteRelease

#define WriteLongPtrNoFence WriteNoFence

#define WriteLongPtrRaw WriteRaw

#define ReadULongPtrAcquire ReadULongAcquire

#define ReadULongPtrNoFence ReadULongNoFence

#define ReadULongPtrRaw ReadULongRaw

#define WriteULongPtrRelease WriteULongRelease

#define WriteULongPtrNoFence WriteULongNoFence

#define WriteULongPtrRaw WriteULongRaw

#else // !defined(_WIN64)

FORCEINLINE
PVOID
ReadPointerAcquire (
    _In_ _Interlocked_operand_ PVOID const volatile *Source
    )

{

    return (PVOID)ReadAcquire64((PLONG64)Source);
}

CFORCEINLINE
PVOID
ReadPointerNoFence (
    _In_ _Interlocked_operand_ PVOID const volatile *Source
    )

{

    return (PVOID)ReadNoFence64((PLONG64)Source);
}

FORCEINLINE
PVOID
ReadPointerRaw (
    _In_ _Interlocked_operand_ PVOID const volatile *Source
    )

{

    return (PVOID)ReadRaw64((PLONG64)Source);
}

FORCEINLINE
VOID
WritePointerRelease (
    _Out_ _Interlocked_operand_ PVOID volatile *Destination,
    _In_ PVOID Value
    )

{

    WriteRelease64((PLONG64)Destination, (LONG64)Value);
    return;
}

FORCEINLINE
VOID
WritePointerNoFence (
    _Out_ _Interlocked_operand_ PVOID volatile *Destination,
    _In_ PVOID Value
    )

{

    WriteNoFence64((PLONG64)Destination, (LONG64)Value);
    return;
}

FORCEINLINE
VOID
WritePointerRaw (
    _Out_ _Interlocked_operand_ PVOID volatile *Destination,
    _In_ PVOID Value
    )

{

    WriteRaw64((PLONG64)Destination, (LONG64)Value);
    return;
}

#define ReadLongPtrAcquire ReadAcquire64

#define ReadLongPtrNoFence ReadNoFence64

#define ReadLongPtrRaw ReadRaw64

#define WriteLongPtrRelease WriteRelease64

#define WriteLongPtrNoFence WriteNoFence64

#define WriteLongPtrRaw WriteRaw64

#define ReadULongPtrAcquire ReadULong64Acquire

#define ReadULongPtrNoFence ReadULong64NoFence

#define ReadULongPtrRaw ReadULong64Raw

#define WriteULongPtrRelease WriteULong64Release

#define WriteULongPtrNoFence WriteULong64NoFence

#define WriteULongPtrRaw WriteULong64Raw

#endif // !defined(_WIN64)

#endif // !defined(RC_INVOKED) && !defined(MIDL_PASS)

// end_ntddk end_wdm end_ntminiport
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4214) // bitfields other than int
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#pragma warning(disable:4820) // padding added after data member
#endif

#if !defined(RC_INVOKED)

#define WOW64_CONTEXT_i386      0x00010000    // this assumes that i386 and
#define WOW64_CONTEXT_i486      0x00010000    // i486 have identical context records

#define WOW64_CONTEXT_CONTROL               (WOW64_CONTEXT_i386 | 0x00000001L) // SS:SP, CS:IP, FLAGS, BP
#define WOW64_CONTEXT_INTEGER               (WOW64_CONTEXT_i386 | 0x00000002L) // AX, BX, CX, DX, SI, DI
#define WOW64_CONTEXT_SEGMENTS              (WOW64_CONTEXT_i386 | 0x00000004L) // DS, ES, FS, GS
#define WOW64_CONTEXT_FLOATING_POINT        (WOW64_CONTEXT_i386 | 0x00000008L) // 387 state
#define WOW64_CONTEXT_DEBUG_REGISTERS       (WOW64_CONTEXT_i386 | 0x00000010L) // DB 0-3,6,7
#define WOW64_CONTEXT_EXTENDED_REGISTERS    (WOW64_CONTEXT_i386 | 0x00000020L) // cpu specific extensions

#define WOW64_CONTEXT_FULL      (WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER | WOW64_CONTEXT_SEGMENTS)

#define WOW64_CONTEXT_ALL       (WOW64_CONTEXT_CONTROL | WOW64_CONTEXT_INTEGER | WOW64_CONTEXT_SEGMENTS | \
                                 WOW64_CONTEXT_FLOATING_POINT | WOW64_CONTEXT_DEBUG_REGISTERS | \
                                 WOW64_CONTEXT_EXTENDED_REGISTERS)

#define WOW64_CONTEXT_XSTATE                (WOW64_CONTEXT_i386 | 0x00000040L)

#define WOW64_CONTEXT_EXCEPTION_ACTIVE      0x08000000
#define WOW64_CONTEXT_SERVICE_ACTIVE        0x10000000
#define WOW64_CONTEXT_EXCEPTION_REQUEST     0x40000000
#define WOW64_CONTEXT_EXCEPTION_REPORTING   0x80000000

#endif // !defined(RC_INVOKED)

//
//  Define the size of the 80387 save area, which is in the context frame.
//

#define WOW64_SIZE_OF_80387_REGISTERS      80

#define WOW64_MAXIMUM_SUPPORTED_EXTENSION     512

typedef struct _WOW64_FLOATING_SAVE_AREA {
    DWORD   ControlWord;
    DWORD   StatusWord;
    DWORD   TagWord;
    DWORD   ErrorOffset;
    DWORD   ErrorSelector;
    DWORD   DataOffset;
    DWORD   DataSelector;
    BYTE    RegisterArea[WOW64_SIZE_OF_80387_REGISTERS];
    DWORD   Cr0NpxState;
} WOW64_FLOATING_SAVE_AREA;

typedef WOW64_FLOATING_SAVE_AREA *PWOW64_FLOATING_SAVE_AREA;

#include "pshpack4.h"

//
// Context Frame
//
//  This frame has a several purposes: 1) it is used as an argument to
//  NtContinue, 2) is is used to constuct a call frame for APC delivery,
//  and 3) it is used in the user level thread creation routines.
//
//  The layout of the record conforms to a standard call frame.
//

typedef struct _WOW64_CONTEXT {

    //
    // The flags values within this flag control the contents of
    // a CONTEXT record.
    //
    // If the context record is used as an input parameter, then
    // for each portion of the context record controlled by a flag
    // whose value is set, it is assumed that that portion of the
    // context record contains valid context. If the context record
    // is being used to modify a threads context, then only that
    // portion of the threads context will be modified.
    //
    // If the context record is used as an IN OUT parameter to capture
    // the context of a thread, then only those portions of the thread's
    // context corresponding to set flags will be returned.
    //
    // The context record is never used as an OUT only parameter.
    //

    DWORD ContextFlags;

    //
    // This section is specified/returned if CONTEXT_DEBUG_REGISTERS is
    // set in ContextFlags.  Note that CONTEXT_DEBUG_REGISTERS is NOT
    // included in CONTEXT_FULL.
    //

    DWORD   Dr0;
    DWORD   Dr1;
    DWORD   Dr2;
    DWORD   Dr3;
    DWORD   Dr6;
    DWORD   Dr7;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_FLOATING_POINT.
    //

    WOW64_FLOATING_SAVE_AREA FloatSave;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_SEGMENTS.
    //

    DWORD   SegGs;
    DWORD   SegFs;
    DWORD   SegEs;
    DWORD   SegDs;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_INTEGER.
    //

    DWORD   Edi;
    DWORD   Esi;
    DWORD   Ebx;
    DWORD   Edx;
    DWORD   Ecx;
    DWORD   Eax;

    //
    // This section is specified/returned if the
    // ContextFlags word contians the flag CONTEXT_CONTROL.
    //

    DWORD   Ebp;
    DWORD   Eip;
    DWORD   SegCs;              // MUST BE SANITIZED
    DWORD   EFlags;             // MUST BE SANITIZED
    DWORD   Esp;
    DWORD   SegSs;

    //
    // This section is specified/returned if the ContextFlags word
    // contains the flag CONTEXT_EXTENDED_REGISTERS.
    // The format and contexts are processor specific
    //

    BYTE    ExtendedRegisters[WOW64_MAXIMUM_SUPPORTED_EXTENSION];

} WOW64_CONTEXT;

typedef WOW64_CONTEXT *PWOW64_CONTEXT;

#include "poppack.h"


typedef struct _WOW64_LDT_ENTRY {
    WORD    LimitLow;
    WORD    BaseLow;
    union {
        struct {
            BYTE    BaseMid;
            BYTE    Flags1;     // Declare as bytes to avoid alignment
            BYTE    Flags2;     // Problems.
            BYTE    BaseHi;
        } Bytes;
        struct {
            DWORD   BaseMid : 8;
            DWORD   Type : 5;
            DWORD   Dpl : 2;
            DWORD   Pres : 1;
            DWORD   LimitHi : 4;
            DWORD   Sys : 1;
            DWORD   Reserved_0 : 1;
            DWORD   Default_Big : 1;
            DWORD   Granularity : 1;
            DWORD   BaseHi : 8;
        } Bits;
    } HighWord;
} WOW64_LDT_ENTRY, *PWOW64_LDT_ENTRY;

typedef struct _WOW64_DESCRIPTOR_TABLE_ENTRY {
    DWORD Selector;
    WOW64_LDT_ENTRY Descriptor;
} WOW64_DESCRIPTOR_TABLE_ENTRY, *PWOW64_DESCRIPTOR_TABLE_ENTRY;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#define EXCEPTION_NONCONTINUABLE 0x1        // Noncontinuable exception
#define EXCEPTION_UNWINDING 0x2             // Unwind is in progress
#define EXCEPTION_EXIT_UNWIND 0x4           // Exit unwind is in progress
#define EXCEPTION_STACK_INVALID 0x8         // Stack out of limits or unaligned
#define EXCEPTION_NESTED_CALL 0x10          // Nested exception handler call
#define EXCEPTION_TARGET_UNWIND 0x20        // Target unwind in progress
#define EXCEPTION_COLLIDED_UNWIND 0x40      // Collided exception handler call
#define EXCEPTION_SOFTWARE_ORIGINATE 0x80   // Exception originated in software

#define EXCEPTION_UNWIND (EXCEPTION_UNWINDING | EXCEPTION_EXIT_UNWIND | \
                          EXCEPTION_TARGET_UNWIND | EXCEPTION_COLLIDED_UNWIND)

#define IS_UNWINDING(Flag) ((Flag & EXCEPTION_UNWIND) != 0)
#define IS_DISPATCHING(Flag) ((Flag & EXCEPTION_UNWIND) == 0)
#define IS_TARGET_UNWIND(Flag) (Flag & EXCEPTION_TARGET_UNWIND)

#define EXCEPTION_MAXIMUM_PARAMETERS 15 // maximum number of exception parameters

//
// Exception record definition.
//

typedef struct _EXCEPTION_RECORD {
    DWORD    ExceptionCode;
    DWORD ExceptionFlags;
    struct _EXCEPTION_RECORD *ExceptionRecord;
    PVOID ExceptionAddress;
    DWORD NumberParameters;
    ULONG_PTR ExceptionInformation[EXCEPTION_MAXIMUM_PARAMETERS];
    } EXCEPTION_RECORD;

typedef EXCEPTION_RECORD *PEXCEPTION_RECORD;

typedef struct _EXCEPTION_RECORD32 {
    DWORD    ExceptionCode;
    DWORD ExceptionFlags;
    DWORD ExceptionRecord;
    DWORD ExceptionAddress;
    DWORD NumberParameters;
    DWORD ExceptionInformation[EXCEPTION_MAXIMUM_PARAMETERS];
} EXCEPTION_RECORD32, *PEXCEPTION_RECORD32;

typedef struct _EXCEPTION_RECORD64 {
    DWORD    ExceptionCode;
    DWORD ExceptionFlags;
    DWORD64 ExceptionRecord;
    DWORD64 ExceptionAddress;
    DWORD NumberParameters;
    DWORD __unusedAlignment;
    DWORD64 ExceptionInformation[EXCEPTION_MAXIMUM_PARAMETERS];
} EXCEPTION_RECORD64, *PEXCEPTION_RECORD64;

//
// Typedef for pointer returned by exception_info()
//

typedef struct _EXCEPTION_POINTERS {
    PEXCEPTION_RECORD ExceptionRecord;
    PCONTEXT ContextRecord;
} EXCEPTION_POINTERS, *PEXCEPTION_POINTERS;

// end_ntoshvp
// end_wdm


#if defined(_IA64_)

NTSYSAPI
VOID
NTAPI
RtlUnwind2 (
    _In_opt_ FRAME_POINTERS TargetFrame,
    _In_opt_ PVOID TargetIp,
    _In_opt_ PEXCEPTION_RECORD ExceptionRecord,
    _In_ PVOID ReturnValue,
    _In_ PCONTEXT ContextRecord
    );

#endif

typedef PVOID PACCESS_TOKEN;            
typedef PVOID PSECURITY_DESCRIPTOR;     
typedef PVOID PSID;     
typedef PVOID PCLAIMS_BLOB;     
////////////////////////////////////////////////////////////////////////
//                                                                    //
//                             ACCESS MASK                            //
//                                                                    //
////////////////////////////////////////////////////////////////////////

//
//  Define the access mask as a longword sized structure divided up as
//  follows:
//
//       3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//       1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//      +---------------+---------------+-------------------------------+
//      |G|G|G|G|Res'd|A| StandardRights|         SpecificRights        |
//      |R|W|E|A|     |S|               |                               |
//      +-+-------------+---------------+-------------------------------+
//
//      typedef struct _ACCESS_MASK {
//          WORD   SpecificRights;
//          BYTE  StandardRights;
//          BYTE  AccessSystemAcl : 1;
//          BYTE  Reserved : 3;
//          BYTE  GenericAll : 1;
//          BYTE  GenericExecute : 1;
//          BYTE  GenericWrite : 1;
//          BYTE  GenericRead : 1;
//      } ACCESS_MASK;
//      typedef ACCESS_MASK *PACCESS_MASK;
//
//  but to make life simple for programmer's we'll allow them to specify
//  a desired access mask by simply OR'ing together mulitple single rights
//  and treat an access mask as a DWORD.  For example
//
//      DesiredAccess = DELETE | READ_CONTROL
//
//  So we'll declare ACCESS_MASK as DWORD
//

// begin_wdm
// begin_ntoshvp
typedef DWORD ACCESS_MASK;
typedef ACCESS_MASK *PACCESS_MASK;

// end_ntoshvp
// begin_access
////////////////////////////////////////////////////////////////////////
//                                                                    //
//                             ACCESS TYPES                           //
//                                                                    //
////////////////////////////////////////////////////////////////////////


// begin_wdm
//
//  The following are masks for the predefined standard access types
//

#define DELETE                           (0x00010000L)
#define READ_CONTROL                     (0x00020000L)
#define WRITE_DAC                        (0x00040000L)
#define WRITE_OWNER                      (0x00080000L)
#define SYNCHRONIZE                      (0x00100000L)

#define STANDARD_RIGHTS_REQUIRED         (0x000F0000L)

#define STANDARD_RIGHTS_READ             (READ_CONTROL)
#define STANDARD_RIGHTS_WRITE            (READ_CONTROL)
#define STANDARD_RIGHTS_EXECUTE          (READ_CONTROL)

#define STANDARD_RIGHTS_ALL              (0x001F0000L)

#define SPECIFIC_RIGHTS_ALL              (0x0000FFFFL)

//
// AccessSystemAcl access type
//

#define ACCESS_SYSTEM_SECURITY           (0x01000000L)

//
// MaximumAllowed access type
//

#define MAXIMUM_ALLOWED                  (0x02000000L)

//
//  These are the generic rights.
//

#define GENERIC_READ                     (0x80000000L)
#define GENERIC_WRITE                    (0x40000000L)
#define GENERIC_EXECUTE                  (0x20000000L)
#define GENERIC_ALL                      (0x10000000L)

//
//  Define the generic mapping array.  This is used to denote the
//  mapping of each generic access right to a specific access mask.
//

typedef struct _GENERIC_MAPPING {
    ACCESS_MASK GenericRead;
    ACCESS_MASK GenericWrite;
    ACCESS_MASK GenericExecute;
    ACCESS_MASK GenericAll;
} GENERIC_MAPPING;
typedef GENERIC_MAPPING *PGENERIC_MAPPING;



////////////////////////////////////////////////////////////////////////
//                                                                    //
//                        LUID_AND_ATTRIBUTES                         //
//                                                                    //
////////////////////////////////////////////////////////////////////////
//
//


#include <pshpack4.h>

typedef struct _LUID_AND_ATTRIBUTES {
    LUID Luid;
    DWORD Attributes;
    } LUID_AND_ATTRIBUTES, * PLUID_AND_ATTRIBUTES;
typedef LUID_AND_ATTRIBUTES LUID_AND_ATTRIBUTES_ARRAY[ANYSIZE_ARRAY];
typedef LUID_AND_ATTRIBUTES_ARRAY *PLUID_AND_ATTRIBUTES_ARRAY;

#include <poppack.h>


////////////////////////////////////////////////////////////////////////
//                                                                    //
//              Security Id     (SID)                                 //
//                                                                    //
////////////////////////////////////////////////////////////////////////
//
//
// Pictorially the structure of an SID is as follows:
//
//         1   1   1   1   1   1
//         5   4   3   2   1   0   9   8   7   6   5   4   3   2   1   0
//      +---------------------------------------------------------------+
//      |      SubAuthorityCount        |Reserved1 (SBZ)|   Revision    |
//      +---------------------------------------------------------------+
//      |                   IdentifierAuthority[0]                      |
//      +---------------------------------------------------------------+
//      |                   IdentifierAuthority[1]                      |
//      +---------------------------------------------------------------+
//      |                   IdentifierAuthority[2]                      |
//      +---------------------------------------------------------------+
//      |                                                               |
//      +- -  -  -  -  -  -  -  SubAuthority[]  -  -  -  -  -  -  -  - -+
//      |                                                               |
//      +---------------------------------------------------------------+
//
//


// begin_ntifs

#ifndef SID_IDENTIFIER_AUTHORITY_DEFINED
#define SID_IDENTIFIER_AUTHORITY_DEFINED
typedef struct _SID_IDENTIFIER_AUTHORITY {
    BYTE  Value[6];
} SID_IDENTIFIER_AUTHORITY, *PSID_IDENTIFIER_AUTHORITY;
#endif

#define SID_REVISION                     (1)    // Current revision level
#define SID_MAX_SUB_AUTHORITIES          (15)
#define SID_RECOMMENDED_SUB_AUTHORITIES  (1)    // Will change to around 6 in a future release.

#ifndef SID_DEFINED
#define SID_DEFINED
typedef struct _SID {
   BYTE  Revision;
#ifdef MIDL_PASS
   [range(0,SID_MAX_SUB_AUTHORITIES)]
#endif
   BYTE  SubAuthorityCount;
   SID_IDENTIFIER_AUTHORITY IdentifierAuthority;
#ifdef MIDL_PASS
   [size_is(SubAuthorityCount)] DWORD SubAuthority[*];
#else // MIDL_PASS
   DWORD SubAuthority[ANYSIZE_ARRAY];
#endif // MIDL_PASS
} SID, *PISID;
#endif

#ifndef MIDL_PASS
#define SECURITY_MAX_SID_SIZE  \
      (sizeof(SID) - sizeof(DWORD) + (SID_MAX_SUB_AUTHORITIES * sizeof(DWORD)))

#define SECURITY_SID_SIZE(SubAuthorityCount_) (sizeof(SID) - sizeof(DWORD) + \
                                                (SubAuthorityCount_) * sizeof(DWORD))

// 2 (S-)
// 4 (Rev(max: 255)-)
// 15 (
//      If (Auth < 2^32): Auth(max:4294967295)-
//      Else:             0xAuth(max:FFFFFFFFFFFF)-
//    )
// (11 * SID_MAX_SUB_AUTHORITIES) (SubN(max:4294967295)-)
// 1 (NULL character)
// = 187 (assuming SID_MAX_SUB_AUTHORITIES = 15)
#define SECURITY_MAX_SID_STRING_CHARACTERS \
    (2 + 4 + 15 + (11 * SID_MAX_SUB_AUTHORITIES) + 1)

//
// Union which can hold any valid sid.
//

typedef union _SE_SID {
    SID Sid;
    BYTE  Buffer[SECURITY_MAX_SID_SIZE];
} SE_SID, *PSE_SID;

#endif // MIDL_PASS


typedef enum _SID_NAME_USE {
    SidTypeUser = 1,
    SidTypeGroup,
    SidTypeDomain,
    SidTypeAlias,
    SidTypeWellKnownGroup,
    SidTypeDeletedAccount,
    SidTypeInvalid,
    SidTypeUnknown,
    SidTypeComputer,
    SidTypeLabel,
    SidTypeLogonSession
} SID_NAME_USE, *PSID_NAME_USE;

typedef struct _SID_AND_ATTRIBUTES {
#ifdef MIDL_PASS
    PISID Sid;
#else // MIDL_PASS
    PSID Sid;
#endif // MIDL_PASS
    DWORD Attributes;
    } SID_AND_ATTRIBUTES, * PSID_AND_ATTRIBUTES;

typedef SID_AND_ATTRIBUTES SID_AND_ATTRIBUTES_ARRAY[ANYSIZE_ARRAY];
typedef SID_AND_ATTRIBUTES_ARRAY *PSID_AND_ATTRIBUTES_ARRAY;

#define SID_HASH_SIZE 32
typedef ULONG_PTR SID_HASH_ENTRY, *PSID_HASH_ENTRY;

typedef struct _SID_AND_ATTRIBUTES_HASH {
    DWORD SidCount;
    PSID_AND_ATTRIBUTES SidAttr;
    SID_HASH_ENTRY Hash[SID_HASH_SIZE];
} SID_AND_ATTRIBUTES_HASH, *PSID_AND_ATTRIBUTES_HASH;

typedef struct _ATTRIBUTES_AND_SID {
    UINT32 Attributes;
    DWORD SidStart;
} ATTRIBUTES_AND_SID, *PATTRIBUTES_AND_SID;


/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// Universal well-known SIDs                                               //
//                                                                         //
//     Null SID                     S-1-0-0                                //
//     World                        S-1-1-0                                //
//     Local                        S-1-2-0                                //
//     Creator Owner ID             S-1-3-0                                //
//     Creator Group ID             S-1-3-1                                //
//     Creator Owner Server ID      S-1-3-2                                //
//     Creator Group Server ID      S-1-3-3                                //
//                                                                         //
//     (Non-unique IDs)             S-1-4                                  //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////

#define SECURITY_NULL_SID_AUTHORITY         {0,0,0,0,0,0}
#define SECURITY_WORLD_SID_AUTHORITY        {0,0,0,0,0,1}
#define SECURITY_LOCAL_SID_AUTHORITY        {0,0,0,0,0,2}
#define SECURITY_CREATOR_SID_AUTHORITY      {0,0,0,0,0,3}
#define SECURITY_NON_UNIQUE_AUTHORITY       {0,0,0,0,0,4}
#define SECURITY_RESOURCE_MANAGER_AUTHORITY {0,0,0,0,0,9}


#define SECURITY_NULL_RID                 (0x00000000L)
#define SECURITY_WORLD_RID                (0x00000000L)
#define SECURITY_LOCAL_RID                (0x00000000L)
#define SECURITY_LOCAL_LOGON_RID          (0x00000001L)

#define SECURITY_CREATOR_OWNER_RID        (0x00000000L)
#define SECURITY_CREATOR_GROUP_RID        (0x00000001L)

#define SECURITY_CREATOR_OWNER_SERVER_RID (0x00000002L)
#define SECURITY_CREATOR_GROUP_SERVER_RID (0x00000003L)

#define SECURITY_CREATOR_OWNER_RIGHTS_RID (0x00000004L)

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
// NT well-known SIDs                                                        //
//                                                                           //
//     NT Authority            S-1-5                                         //
//     Dialup                  S-1-5-1                                       //
//                                                                           //
//     Network                 S-1-5-2                                       //
//     Batch                   S-1-5-3                                       //
//     Interactive             S-1-5-4                                       //
//     (Logon IDs)             S-1-5-5-X-Y                                   //
//     Service                 S-1-5-6                                       //
//     AnonymousLogon          S-1-5-7       (aka null logon session)        //
//     Proxy                   S-1-5-8                                       //
//     Enterprise DC (EDC)     S-1-5-9       (aka domain controller account) //
//     Self                    S-1-5-10      (self RID)                      //
//     Authenticated User      S-1-5-11      (Authenticated user somewhere)  //
//     Restricted Code         S-1-5-12      (Running restricted code)       //
//     Terminal Server         S-1-5-13      (Running on Terminal Server)    //
//     Remote Logon            S-1-5-14      (Remote Interactive Logon)      //
//     This Organization       S-1-5-15                                      //
//                                                                           //
//     IUser                   S-1-5-17
//     Local System            S-1-5-18                                      //
//     Local Service           S-1-5-19                                      //
//     Network Service         S-1-5-20                                      //
//                                                                           //
//     (NT non-unique IDs)     S-1-5-0x15-... (NT Domain Sids)               //
//                                                                           //
//     (Built-in domain)       S-1-5-0x20                                    //
//                                                                           //
//     (Security Package IDs)  S-1-5-0x40                                    //
//     NTLM Authentication     S-1-5-0x40-10                                 //
//     SChannel Authentication S-1-5-0x40-14                                 //
//     Digest Authentication   S-1-5-0x40-21                                 //
//                                                                           //
//     Other Organization      S-1-5-1000    (>=1000 can not be filtered)    //
//                                                                           //
//                                                                           //
// NOTE: the relative identifier values (RIDs) determine which security      //
//       boundaries the SID is allowed to cross.  Before adding new RIDs,    //
//       a determination needs to be made regarding which range they should  //
//       be added to in order to ensure proper "SID filtering"               //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////


#define SECURITY_NT_AUTHORITY           {0,0,0,0,0,5}   // ntifs

#define SECURITY_DIALUP_RID             (0x00000001L)
#define SECURITY_NETWORK_RID            (0x00000002L)
#define SECURITY_BATCH_RID              (0x00000003L)
#define SECURITY_INTERACTIVE_RID        (0x00000004L)
#define SECURITY_LOGON_IDS_RID          (0x00000005L)
#define SECURITY_LOGON_IDS_RID_COUNT    (3L)
#define SECURITY_SERVICE_RID            (0x00000006L)
#define SECURITY_ANONYMOUS_LOGON_RID    (0x00000007L)
#define SECURITY_PROXY_RID              (0x00000008L)
#define SECURITY_ENTERPRISE_CONTROLLERS_RID (0x00000009L)
#define SECURITY_SERVER_LOGON_RID       SECURITY_ENTERPRISE_CONTROLLERS_RID
#define SECURITY_PRINCIPAL_SELF_RID     (0x0000000AL)
#define SECURITY_AUTHENTICATED_USER_RID (0x0000000BL)
#define SECURITY_RESTRICTED_CODE_RID    (0x0000000CL)
#define SECURITY_TERMINAL_SERVER_RID    (0x0000000DL)
#define SECURITY_REMOTE_LOGON_RID       (0x0000000EL)
#define SECURITY_THIS_ORGANIZATION_RID  (0x0000000FL)
#define SECURITY_IUSER_RID              (0x00000011L)
#define SECURITY_LOCAL_SYSTEM_RID       (0x00000012L)
#define SECURITY_LOCAL_SERVICE_RID      (0x00000013L)
#define SECURITY_NETWORK_SERVICE_RID    (0x00000014L)

#define SECURITY_NT_NON_UNIQUE          (0x00000015L)
#define SECURITY_NT_NON_UNIQUE_SUB_AUTH_COUNT  (3L)

#define SECURITY_ENTERPRISE_READONLY_CONTROLLERS_RID (0x00000016L)

#define SECURITY_BUILTIN_DOMAIN_RID     (0x00000020L)
#define SECURITY_WRITE_RESTRICTED_CODE_RID (0x00000021L)


#define SECURITY_PACKAGE_BASE_RID       (0x00000040L)
#define SECURITY_PACKAGE_RID_COUNT      (2L)
#define SECURITY_PACKAGE_NTLM_RID       (0x0000000AL)
#define SECURITY_PACKAGE_SCHANNEL_RID   (0x0000000EL)
#define SECURITY_PACKAGE_DIGEST_RID     (0x00000015L)

#define SECURITY_CRED_TYPE_BASE_RID             (0x00000041L)
#define SECURITY_CRED_TYPE_RID_COUNT            (2L)
#define SECURITY_CRED_TYPE_THIS_ORG_CERT_RID    (0x00000001L)

#define SECURITY_MIN_BASE_RID           (0x00000050L)

#define SECURITY_SERVICE_ID_BASE_RID    (0x00000050L)
#define SECURITY_SERVICE_ID_RID_COUNT   (6L)

#define SECURITY_RESERVED_ID_BASE_RID   (0x00000051L)

#define SECURITY_APPPOOL_ID_BASE_RID    (0x00000052L)
#define SECURITY_APPPOOL_ID_RID_COUNT   (6L)

#define SECURITY_VIRTUALSERVER_ID_BASE_RID    (0x00000053L)
#define SECURITY_VIRTUALSERVER_ID_RID_COUNT   (6L)

#define SECURITY_USERMODEDRIVERHOST_ID_BASE_RID  (0x00000054L)
#define SECURITY_USERMODEDRIVERHOST_ID_RID_COUNT (6L)

#define SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE_RID  (0x00000055L)
#define SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_RID_COUNT (6L)

#define SECURITY_WMIHOST_ID_BASE_RID  (0x00000056L)
#define SECURITY_WMIHOST_ID_RID_COUNT (6L)

#define SECURITY_TASK_ID_BASE_RID                 (0x00000057L)

#define SECURITY_NFS_ID_BASE_RID        (0x00000058L)

#define SECURITY_COM_ID_BASE_RID        (0x00000059L)

#define SECURITY_WINDOW_MANAGER_BASE_RID     (0x0000005AL)

#define SECURITY_RDV_GFX_BASE_RID       (0x0000005BL)

#define SECURITY_DASHOST_ID_BASE_RID    (0x0000005CL)
#define SECURITY_DASHOST_ID_RID_COUNT   (6L)

#define SECURITY_USERMANAGER_ID_BASE_RID    (0x0000005DL)
#define SECURITY_USERMANAGER_ID_RID_COUNT   (6L)

#define SECURITY_WINRM_ID_BASE_RID      (0x0000005EL)
#define SECURITY_WINRM_ID_RID_COUNT     (6L)

#define SECURITY_CCG_ID_BASE_RID        (0x0000005FL)
#define SECURITY_UMFD_BASE_RID          (0x00000060L)
#define SECURITY_UNIQUIFIED_SERVICE_BASE_RID (0x00000061L)

#define SECURITY_VIRTUALACCOUNT_ID_RID_COUNT   (6L)

#define SECURITY_EDGE_CLOUD_INFRASTRUCTURE_SERVICE_ID_BASE_RID (0x00000062L)

#define SECURITY_RESTRICTED_SERVICES_BASE_RID  (0x00000063L)
#define SECURITY_RESTRICTED_SERVICES_RID_COUNT (6L)

//
// Virtual account logon is not limited to inbox callers.  Reserve base RID 0x6F for application usage.
//

#define SECURITY_MAX_BASE_RID           (0x0000006FL)
#define SECURITY_MAX_ALWAYS_FILTERED    (0x000003E7L)
#define SECURITY_MIN_NEVER_FILTERED     (0x000003E8L)

#define SECURITY_OTHER_ORGANIZATION_RID (0x000003E8L)

//
//Service SID type RIDs are in the range 0x50- 0x6F.  Therefore, we are giving  the next available RID to Windows Mobile team.
//
#define SECURITY_WINDOWSMOBILE_ID_BASE_RID (0x00000070L)

//
// Installer Capability Group Sid related. Currently Base RID is same as LOCAL DOMAIN.
//
#define SECURITY_INSTALLER_GROUP_CAPABILITY_BASE (0x20)
#define SECURITY_INSTALLER_GROUP_CAPABILITY_RID_COUNT (9)

// Note: This is because the App Capability Rid is S-1-15-3-1024-...
//       whereas the service group rid is          S-1-5-32-...
//  The number of RIDs from hash (8) are the same for both
#define SECURITY_INSTALLER_CAPABILITY_RID_COUNT (10)

//
//Well-known group for local accounts
//
#define SECURITY_LOCAL_ACCOUNT_RID (0x00000071L)
#define SECURITY_LOCAL_ACCOUNT_AND_ADMIN_RID (0x00000072L)

/////////////////////////////////////////////////////////////////////////////
//                                                                         //
// well-known domain relative sub-authority values (RIDs)...               //
//                                                                         //
/////////////////////////////////////////////////////////////////////////////


#define DOMAIN_GROUP_RID_AUTHORIZATION_DATA_IS_COMPOUNDED       (0x000001F0L)
#define DOMAIN_GROUP_RID_AUTHORIZATION_DATA_CONTAINS_CLAIMS     (0x000001F1L)
#define DOMAIN_GROUP_RID_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS (0x000001F2L)

#define FOREST_USER_RID_MAX            (0x000001F3L)

// Well-known users ...

#define DOMAIN_USER_RID_ADMIN                (0x000001F4L)
#define DOMAIN_USER_RID_GUEST                (0x000001F5L)
#define DOMAIN_USER_RID_KRBTGT               (0x000001F6L)
#define DOMAIN_USER_RID_DEFAULT_ACCOUNT      (0x000001F7L)
#define DOMAIN_USER_RID_WDAG_ACCOUNT         (0x000001F8L)

#define DOMAIN_USER_RID_MAX            (0x000003E7L)


// well-known groups ...

#define DOMAIN_GROUP_RID_ADMINS        (0x00000200L)
#define DOMAIN_GROUP_RID_USERS         (0x00000201L)
#define DOMAIN_GROUP_RID_GUESTS        (0x00000202L)
#define DOMAIN_GROUP_RID_COMPUTERS     (0x00000203L)
#define DOMAIN_GROUP_RID_CONTROLLERS   (0x00000204L)
#define DOMAIN_GROUP_RID_CERT_ADMINS   (0x00000205L)
#define DOMAIN_GROUP_RID_SCHEMA_ADMINS (0x00000206L)
#define DOMAIN_GROUP_RID_ENTERPRISE_ADMINS (0x00000207L)
#define DOMAIN_GROUP_RID_POLICY_ADMINS (0x00000208L)
#define DOMAIN_GROUP_RID_READONLY_CONTROLLERS (0x00000209L)
#define DOMAIN_GROUP_RID_CLONEABLE_CONTROLLERS (0x0000020AL)
#define DOMAIN_GROUP_RID_CDC_RESERVED    (0x0000020CL)
#define DOMAIN_GROUP_RID_PROTECTED_USERS (0x0000020DL)
#define DOMAIN_GROUP_RID_KEY_ADMINS      (0x0000020EL)
#define DOMAIN_GROUP_RID_ENTERPRISE_KEY_ADMINS (0x0000020FL)
#define DOMAIN_GROUP_RID_FOREST_TRUSTS   (0x00000210L)
#define DOMAIN_GROUP_RID_EXTERNAL_TRUSTS (0x00000211L)

// well-known aliases ...

#define DOMAIN_ALIAS_RID_ADMINS                         (0x00000220L)
#define DOMAIN_ALIAS_RID_USERS                          (0x00000221L)
#define DOMAIN_ALIAS_RID_GUESTS                         (0x00000222L)
#define DOMAIN_ALIAS_RID_POWER_USERS                    (0x00000223L)

#define DOMAIN_ALIAS_RID_ACCOUNT_OPS                    (0x00000224L)
#define DOMAIN_ALIAS_RID_SYSTEM_OPS                     (0x00000225L)
#define DOMAIN_ALIAS_RID_PRINT_OPS                      (0x00000226L)
#define DOMAIN_ALIAS_RID_BACKUP_OPS                     (0x00000227L)

#define DOMAIN_ALIAS_RID_REPLICATOR                     (0x00000228L)
#define DOMAIN_ALIAS_RID_RAS_SERVERS                    (0x00000229L)
#define DOMAIN_ALIAS_RID_PREW2KCOMPACCESS               (0x0000022AL)
#define DOMAIN_ALIAS_RID_REMOTE_DESKTOP_USERS           (0x0000022BL)
#define DOMAIN_ALIAS_RID_NETWORK_CONFIGURATION_OPS      (0x0000022CL)
#define DOMAIN_ALIAS_RID_INCOMING_FOREST_TRUST_BUILDERS (0x0000022DL)

#define DOMAIN_ALIAS_RID_MONITORING_USERS               (0x0000022EL)
#define DOMAIN_ALIAS_RID_LOGGING_USERS                  (0x0000022FL)
#define DOMAIN_ALIAS_RID_AUTHORIZATIONACCESS            (0x00000230L)
#define DOMAIN_ALIAS_RID_TS_LICENSE_SERVERS             (0x00000231L)
#define DOMAIN_ALIAS_RID_DCOM_USERS                     (0x00000232L)
#define DOMAIN_ALIAS_RID_IUSERS                         (0x00000238L)
#define DOMAIN_ALIAS_RID_CRYPTO_OPERATORS               (0x00000239L)
#define DOMAIN_ALIAS_RID_CACHEABLE_PRINCIPALS_GROUP     (0x0000023BL)
#define DOMAIN_ALIAS_RID_NON_CACHEABLE_PRINCIPALS_GROUP (0x0000023CL)
#define DOMAIN_ALIAS_RID_EVENT_LOG_READERS_GROUP        (0x0000023DL)
#define DOMAIN_ALIAS_RID_CERTSVC_DCOM_ACCESS_GROUP      (0x0000023EL)
#define DOMAIN_ALIAS_RID_RDS_REMOTE_ACCESS_SERVERS      (0x0000023FL)
#define DOMAIN_ALIAS_RID_RDS_ENDPOINT_SERVERS           (0x00000240L)
#define DOMAIN_ALIAS_RID_RDS_MANAGEMENT_SERVERS         (0x00000241L)
#define DOMAIN_ALIAS_RID_HYPER_V_ADMINS                 (0x00000242L)
#define DOMAIN_ALIAS_RID_ACCESS_CONTROL_ASSISTANCE_OPS  (0x00000243L)
#define DOMAIN_ALIAS_RID_REMOTE_MANAGEMENT_USERS        (0x00000244L)
#define DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT                (0x00000245L)
#define DOMAIN_ALIAS_RID_STORAGE_REPLICA_ADMINS         (0x00000246L)
#define DOMAIN_ALIAS_RID_DEVICE_OWNERS                  (0x00000247L)
#define DOMAIN_ALIAS_RID_USER_MODE_HARDWARE_OPERATORS   (0x00000248L)
#define DOMAIN_ALIAS_RID_OPENSSH_USERS                  (0x00000249L)

//
// Application Package Authority.
//

#define SECURITY_APP_PACKAGE_AUTHORITY              {0,0,0,0,0,15}

#define SECURITY_APP_PACKAGE_BASE_RID               (0x00000002L)
#define SECURITY_BUILTIN_APP_PACKAGE_RID_COUNT      (2L)
#define SECURITY_APP_PACKAGE_RID_COUNT              (8L)
#define SECURITY_CAPABILITY_BASE_RID                (0x00000003L)
#define SECURITY_CAPABILITY_APP_RID                 (0x00000400L)
#define SECURITY_CAPABILITY_APP_SILO_RID            (0x00010000L)
#define SECURITY_BUILTIN_CAPABILITY_RID_COUNT       (2L)
#define SECURITY_CAPABILITY_RID_COUNT               (5L)
#define SECURITY_PARENT_PACKAGE_RID_COUNT           (SECURITY_APP_PACKAGE_RID_COUNT)
#define SECURITY_CHILD_PACKAGE_RID_COUNT            (12L)

//
// Built-in Packages.
//

#define SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE            (0x00000001L)
#define SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE (0x00000002L)

//
// Built-in Capabilities.
//

#define SECURITY_CAPABILITY_INTERNET_CLIENT                     (0x00000001L)
#define SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER              (0x00000002L)
#define SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER       (0x00000003L)
#define SECURITY_CAPABILITY_PICTURES_LIBRARY                    (0x00000004L)
#define SECURITY_CAPABILITY_VIDEOS_LIBRARY                      (0x00000005L)
#define SECURITY_CAPABILITY_MUSIC_LIBRARY                       (0x00000006L)
#define SECURITY_CAPABILITY_DOCUMENTS_LIBRARY                   (0x00000007L)
#define SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION           (0x00000008L)
#define SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES            (0x00000009L)
#define SECURITY_CAPABILITY_REMOVABLE_STORAGE                   (0x0000000AL)
#define SECURITY_CAPABILITY_APPOINTMENTS                        (0x0000000BL)
#define SECURITY_CAPABILITY_CONTACTS                            (0x0000000CL)

#define SECURITY_CAPABILITY_INTERNET_EXPLORER                   (0x00001000L)

//
// Mandatory Label Authority.
//

#define SECURITY_MANDATORY_LABEL_AUTHORITY          {0,0,0,0,0,16}
#define SECURITY_MANDATORY_UNTRUSTED_RID            (0x00000000L)
#define SECURITY_MANDATORY_LOW_RID                  (0x00001000L)
#define SECURITY_MANDATORY_MEDIUM_RID               (0x00002000L)
#define SECURITY_MANDATORY_MEDIUM_PLUS_RID          (SECURITY_MANDATORY_MEDIUM_RID + 0x100)
#define SECURITY_MANDATORY_HIGH_RID                 (0x00003000L)
#define SECURITY_MANDATORY_SYSTEM_RID               (0x00004000L)
#define SECURITY_MANDATORY_PROTECTED_PROCESS_RID    (0x00005000L)
#define SECURITY_MANDATORY_MEDIUM_PLUS_CREDUI_RID   (SECURITY_MANDATORY_MEDIUM_RID + 0xA)

//
// SECURITY_MANDATORY_MAXIMUM_USER_RID is the highest RID that
// can be set by a usermode caller.
//

#define SECURITY_MANDATORY_MAXIMUM_USER_RID   SECURITY_MANDATORY_SYSTEM_RID

#define MANDATORY_LEVEL_TO_MANDATORY_RID(IL) (IL * 0x1000)

#define SECURITY_SCOPED_POLICY_ID_AUTHORITY             {0,0,0,0,0,17}

//
// Authentication Authority
//

#define SECURITY_AUTHENTICATION_AUTHORITY                        {0,0,0,0,0,18}
#define SECURITY_AUTHENTICATION_AUTHORITY_RID_COUNT              (1L)
#define SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED_RID           (0x00000001L)
#define SECURITY_AUTHENTICATION_SERVICE_ASSERTED_RID             (0x00000002L)
#define SECURITY_AUTHENTICATION_FRESH_KEY_AUTH_RID               (0x00000003L)
#define SECURITY_AUTHENTICATION_KEY_TRUST_RID                    (0x00000004L)
#define SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA_RID             (0x00000005L)
#define SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION_RID     (0x00000006L)

//
// Process Trust Authority
//

#define SECURITY_PROCESS_TRUST_AUTHORITY    {0,0,0,0,0,19}
#define SECURITY_PROCESS_TRUST_AUTHORITY_RID_COUNT (2L)

#define SECURITY_PROCESS_PROTECTION_TYPE_FULL_RID           (0x00000400L)
#define SECURITY_PROCESS_PROTECTION_TYPE_LITE_RID           (0x00000200L)
#define SECURITY_PROCESS_PROTECTION_TYPE_NONE_RID           (0x00000000L)

#define SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB_RID        (0x00002000L)
#define SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS_RID       (0x00001000L)
#define SECURITY_PROCESS_PROTECTION_LEVEL_APP_RID           (0x00000800L)
#define SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE_RID   (0x00000600L)
#define SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE_RID  (0x00000400L)
#define SECURITY_PROCESS_PROTECTION_LEVEL_NONE_RID          (0x00000000L)

//
// Trusted Installer RIDs
//

#define SECURITY_TRUSTED_INSTALLER_RID1 956008885
#define SECURITY_TRUSTED_INSTALLER_RID2 3418522649
#define SECURITY_TRUSTED_INSTALLER_RID3 1831038044
#define SECURITY_TRUSTED_INSTALLER_RID4 1853292631
#define SECURITY_TRUSTED_INSTALLER_RID5 2271478464





//
// Well known SID definitions for lookup.
//

typedef enum {

    WinNullSid                                  = 0,
    WinWorldSid                                 = 1,
    WinLocalSid                                 = 2,
    WinCreatorOwnerSid                          = 3,
    WinCreatorGroupSid                          = 4,
    WinCreatorOwnerServerSid                    = 5,
    WinCreatorGroupServerSid                    = 6,
    WinNtAuthoritySid                           = 7,
    WinDialupSid                                = 8,
    WinNetworkSid                               = 9,
    WinBatchSid                                 = 10,
    WinInteractiveSid                           = 11,
    WinServiceSid                               = 12,
    WinAnonymousSid                             = 13,
    WinProxySid                                 = 14,
    WinEnterpriseControllersSid                 = 15,
    WinSelfSid                                  = 16,
    WinAuthenticatedUserSid                     = 17,
    WinRestrictedCodeSid                        = 18,
    WinTerminalServerSid                        = 19,
    WinRemoteLogonIdSid                         = 20,
    WinLogonIdsSid                              = 21,
    WinLocalSystemSid                           = 22,
    WinLocalServiceSid                          = 23,
    WinNetworkServiceSid                        = 24,
    WinBuiltinDomainSid                         = 25,
    WinBuiltinAdministratorsSid                 = 26,
    WinBuiltinUsersSid                          = 27,
    WinBuiltinGuestsSid                         = 28,
    WinBuiltinPowerUsersSid                     = 29,
    WinBuiltinAccountOperatorsSid               = 30,
    WinBuiltinSystemOperatorsSid                = 31,
    WinBuiltinPrintOperatorsSid                 = 32,
    WinBuiltinBackupOperatorsSid                = 33,
    WinBuiltinReplicatorSid                     = 34,
    WinBuiltinPreWindows2000CompatibleAccessSid = 35,
    WinBuiltinRemoteDesktopUsersSid             = 36,
    WinBuiltinNetworkConfigurationOperatorsSid  = 37,
    WinAccountAdministratorSid                  = 38,
    WinAccountGuestSid                          = 39,
    WinAccountKrbtgtSid                         = 40,
    WinAccountDomainAdminsSid                   = 41,
    WinAccountDomainUsersSid                    = 42,
    WinAccountDomainGuestsSid                   = 43,
    WinAccountComputersSid                      = 44,
    WinAccountControllersSid                    = 45,
    WinAccountCertAdminsSid                     = 46,
    WinAccountSchemaAdminsSid                   = 47,
    WinAccountEnterpriseAdminsSid               = 48,
    WinAccountPolicyAdminsSid                   = 49,
    WinAccountRasAndIasServersSid               = 50,
    WinNTLMAuthenticationSid                    = 51,
    WinDigestAuthenticationSid                  = 52,
    WinSChannelAuthenticationSid                = 53,
    WinThisOrganizationSid                      = 54,
    WinOtherOrganizationSid                     = 55,
    WinBuiltinIncomingForestTrustBuildersSid    = 56,
    WinBuiltinPerfMonitoringUsersSid            = 57,
    WinBuiltinPerfLoggingUsersSid               = 58,
    WinBuiltinAuthorizationAccessSid            = 59,
    WinBuiltinTerminalServerLicenseServersSid   = 60,
    WinBuiltinDCOMUsersSid                      = 61,
    WinBuiltinIUsersSid                         = 62,
    WinIUserSid                                 = 63,
    WinBuiltinCryptoOperatorsSid                = 64,
    WinUntrustedLabelSid                        = 65,
    WinLowLabelSid                              = 66,
    WinMediumLabelSid                           = 67,
    WinHighLabelSid                             = 68,
    WinSystemLabelSid                           = 69,
    WinWriteRestrictedCodeSid                   = 70,
    WinCreatorOwnerRightsSid                    = 71,
    WinCacheablePrincipalsGroupSid              = 72,
    WinNonCacheablePrincipalsGroupSid           = 73,
    WinEnterpriseReadonlyControllersSid         = 74,
    WinAccountReadonlyControllersSid            = 75,
    WinBuiltinEventLogReadersGroup              = 76,
    WinNewEnterpriseReadonlyControllersSid      = 77,
    WinBuiltinCertSvcDComAccessGroup            = 78,
    WinMediumPlusLabelSid                       = 79,
    WinLocalLogonSid                            = 80,
    WinConsoleLogonSid                          = 81,
    WinThisOrganizationCertificateSid           = 82,
    WinApplicationPackageAuthoritySid           = 83,
    WinBuiltinAnyPackageSid                     = 84,
    WinCapabilityInternetClientSid              = 85,
    WinCapabilityInternetClientServerSid        = 86,
    WinCapabilityPrivateNetworkClientServerSid  = 87,
    WinCapabilityPicturesLibrarySid             = 88,
    WinCapabilityVideosLibrarySid               = 89,
    WinCapabilityMusicLibrarySid                = 90,
    WinCapabilityDocumentsLibrarySid            = 91,
    WinCapabilitySharedUserCertificatesSid      = 92,
    WinCapabilityEnterpriseAuthenticationSid    = 93,
    WinCapabilityRemovableStorageSid            = 94,
    WinBuiltinRDSRemoteAccessServersSid         = 95,
    WinBuiltinRDSEndpointServersSid             = 96,
    WinBuiltinRDSManagementServersSid           = 97,
    WinUserModeDriversSid                       = 98,
    WinBuiltinHyperVAdminsSid                   = 99,
    WinAccountCloneableControllersSid           = 100,
    WinBuiltinAccessControlAssistanceOperatorsSid = 101,
    WinBuiltinRemoteManagementUsersSid          = 102,
    WinAuthenticationAuthorityAssertedSid       = 103,
    WinAuthenticationServiceAssertedSid         = 104,
    WinLocalAccountSid                          = 105,
    WinLocalAccountAndAdministratorSid          = 106,
    WinAccountProtectedUsersSid                 = 107,
    WinCapabilityAppointmentsSid                = 108,
    WinCapabilityContactsSid                    = 109,
    WinAccountDefaultSystemManagedSid           = 110,
    WinBuiltinDefaultSystemManagedGroupSid      = 111,
    WinBuiltinStorageReplicaAdminsSid           = 112,
    WinAccountKeyAdminsSid                      = 113,
    WinAccountEnterpriseKeyAdminsSid            = 114,
    WinAuthenticationKeyTrustSid                = 115,
    WinAuthenticationKeyPropertyMFASid          = 116,
    WinAuthenticationKeyPropertyAttestationSid  = 117,
    WinAuthenticationFreshKeyAuthSid            = 118,
    WinBuiltinDeviceOwnersSid                   = 119,
    WinBuiltinUserModeHardwareOperatorsSid      = 120,
    WinBuiltinOpenSSHUsersSid                   = 121,
} WELL_KNOWN_SID_TYPE;

//
// Allocate the System Luid.  The first 1000 LUIDs are reserved.
// Use #999 here (0x3e7 = 999)
//

#define SYSTEM_LUID                     { 0x3e7, 0x0 }
#define ANONYMOUS_LOGON_LUID            { 0x3e6, 0x0 }
#define LOCALSERVICE_LUID               { 0x3e5, 0x0 }
#define NETWORKSERVICE_LUID             { 0x3e4, 0x0 }
#define IUSER_LUID                      { 0x3e3, 0x0 }
#define PROTECTED_TO_SYSTEM_LUID        { 0x3e2, 0x0 }

// end_ntifs

////////////////////////////////////////////////////////////////////////
//                                                                    //
//                          User and Group related SID attributes     //
//                                                                    //
////////////////////////////////////////////////////////////////////////

//
// Group attributes
//

#define SE_GROUP_MANDATORY                 (0x00000001L)
#define SE_GROUP_ENABLED_BY_DEFAULT        (0x00000002L)
#define SE_GROUP_ENABLED                   (0x00000004L)
#define SE_GROUP_OWNER                     (0x00000008L)
#define SE_GROUP_USE_FOR_DENY_ONLY         (0x00000010L)
#define SE_GROUP_INTEGRITY                 (0x00000020L)
#define SE_GROUP_INTEGRITY_ENABLED         (0x00000040L)
#define SE_GROUP_LOGON_ID                  (0xC0000000L)
#define SE_GROUP_RESOURCE                  (0x20000000L)

#define SE_GROUP_VALID_ATTRIBUTES          (SE_GROUP_MANDATORY          | \
                                            SE_GROUP_ENABLED_BY_DEFAULT | \
                                            SE_GROUP_ENABLED            | \
                                            SE_GROUP_OWNER              | \
                                            SE_GROUP_USE_FOR_DENY_ONLY  | \
                                            SE_GROUP_LOGON_ID           | \
                                            SE_GROUP_RESOURCE           | \
                                            SE_GROUP_INTEGRITY          | \
                                            SE_GROUP_INTEGRITY_ENABLED)

//
// User attributes
//

// (None yet defined.)




////////////////////////////////////////////////////////////////////////
//                                                                    //
//                         ACL  and  ACE                              //
//                                                                    //
////////////////////////////////////////////////////////////////////////

//
//  Define an ACL and the ACE format.  The structure of an ACL header
//  followed by one or more ACEs.  Pictorally the structure of an ACL header
//  is as follows:
//
//       3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//       1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//      +-------------------------------+---------------+---------------+
//      |            AclSize            |      Sbz1     |  AclRevision  |
//      +-------------------------------+---------------+---------------+
//      |              Sbz2             |           AceCount            |
//      +-------------------------------+-------------------------------+
//
//  The current AclRevision is defined to be ACL_REVISION.
//
//  AclSize is the size, in bytes, allocated for the ACL.  This includes
//  the ACL header, ACES, and remaining free space in the buffer.
//
//  AceCount is the number of ACES in the ACL.
//

// begin_wdm
// This is the *current* ACL revision

#define ACL_REVISION     (2)
#define ACL_REVISION_DS  (4)

// This is the history of ACL revisions.  Add a new one whenever
// ACL_REVISION is updated

#define ACL_REVISION1   (1)
#define MIN_ACL_REVISION ACL_REVISION2
#define ACL_REVISION2   (2)
#define ACL_REVISION3   (3)
#define ACL_REVISION4   (4)
#define MAX_ACL_REVISION ACL_REVISION4

typedef struct _ACL {
    BYTE  AclRevision;
    BYTE  Sbz1;
    WORD   AclSize;
    WORD   AceCount;
    WORD   Sbz2;
} ACL;
typedef ACL *PACL;

// end_wdm
// begin_ntifs

//
//  The structure of an ACE is a common ace header followed by ace type
//  specific data.  Pictorally the structure of the common ace header is
//  as follows:
//
//       3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//       1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//      +---------------+-------+-------+---------------+---------------+
//      |            AceSize            |    AceFlags   |     AceType   |
//      +---------------+-------+-------+---------------+---------------+
//
//  AceType denotes the type of the ace, there are some predefined ace
//  types
//
//  AceSize is the size, in bytes, of ace.
//
//  AceFlags are the Ace flags for audit and inheritance, defined shortly.

typedef struct _ACE_HEADER {
    BYTE  AceType;
    BYTE  AceFlags;
    WORD   AceSize;
} ACE_HEADER;
typedef ACE_HEADER *PACE_HEADER;

//
//  The following are the predefined ace types that go into the AceType
//  field of an Ace header.
//

#define ACCESS_MIN_MS_ACE_TYPE                  (0x0)
#define ACCESS_ALLOWED_ACE_TYPE                 (0x0)
#define ACCESS_DENIED_ACE_TYPE                  (0x1)
#define SYSTEM_AUDIT_ACE_TYPE                   (0x2)
#define SYSTEM_ALARM_ACE_TYPE                   (0x3)
#define ACCESS_MAX_MS_V2_ACE_TYPE               (0x3)

#define ACCESS_ALLOWED_COMPOUND_ACE_TYPE        (0x4)
#define ACCESS_MAX_MS_V3_ACE_TYPE               (0x4)

#define ACCESS_MIN_MS_OBJECT_ACE_TYPE           (0x5)
#define ACCESS_ALLOWED_OBJECT_ACE_TYPE          (0x5)
#define ACCESS_DENIED_OBJECT_ACE_TYPE           (0x6)
#define SYSTEM_AUDIT_OBJECT_ACE_TYPE            (0x7)
#define SYSTEM_ALARM_OBJECT_ACE_TYPE            (0x8)
#define ACCESS_MAX_MS_OBJECT_ACE_TYPE           (0x8)

#define ACCESS_MAX_MS_V4_ACE_TYPE               (0x8)
#define ACCESS_MAX_MS_ACE_TYPE                  (0x8)

#define ACCESS_ALLOWED_CALLBACK_ACE_TYPE        (0x9)
#define ACCESS_DENIED_CALLBACK_ACE_TYPE         (0xA)
#define ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE (0xB)
#define ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE  (0xC)
#define SYSTEM_AUDIT_CALLBACK_ACE_TYPE          (0xD)
#define SYSTEM_ALARM_CALLBACK_ACE_TYPE          (0xE)
#define SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE   (0xF)
#define SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE   (0x10)

#define SYSTEM_MANDATORY_LABEL_ACE_TYPE         (0x11)
#define SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE      (0x12)
#define SYSTEM_SCOPED_POLICY_ID_ACE_TYPE        (0x13)
#define SYSTEM_PROCESS_TRUST_LABEL_ACE_TYPE     (0x14)
#define SYSTEM_ACCESS_FILTER_ACE_TYPE           (0x15)
#define ACCESS_MAX_MS_V5_ACE_TYPE               (0x15)


//
//  The following are the inherit flags that go into the AceFlags field
//  of an Ace header.
//

#define OBJECT_INHERIT_ACE                (0x1)
#define CONTAINER_INHERIT_ACE             (0x2)
#define NO_PROPAGATE_INHERIT_ACE          (0x4)
#define INHERIT_ONLY_ACE                  (0x8)
#define INHERITED_ACE                     (0x10)
#define VALID_INHERIT_FLAGS               (0x1F)


//  The following are the currently defined ACE flags that go into the
//  AceFlags field of an ACE header.  Each ACE type has its own set of
//  AceFlags.
//
//

//
// ACCESS_ALLOWED_ACE_TYPE
//
// These control whether the ACE is critical and cannot be removed.
//
// CRITICAL_ACE_FLAG - used only with access allowed ACE types to
// indicate that the ACE is critical and cannot be removed.
//

#define CRITICAL_ACE_FLAG              (0x20)

//
//  SYSTEM_AUDIT and SYSTEM_ALARM AceFlags
//
//  These control the signaling of audit and alarms for success or failure.
//
//  SUCCESSFUL_ACCESS_ACE_FLAG - used only with system audit and alarm ACE
//  types to indicate that a message is generated for successful accesses.
//
//  FAILED_ACCESS_ACE_FLAG - used only with system audit and alarm ACE types
//  to indicate that a message is generated for failed accesses.
//

#define SUCCESSFUL_ACCESS_ACE_FLAG       (0x40)
#define FAILED_ACCESS_ACE_FLAG           (0x80)

//
//  SYSTEM_ACCESS_FILTER_ACE AceFlags
//
//  These control the behaviour of SYSTEM_ACCESS_FILTER_ACE .
//
//  TRUST_PROTECTED_FILTER_ACE_FLAG - used only with SYSTEM_FILTERING_ACE_TYPE
//  ACEs to indicate that this ACE may not be deleted/modified except when the,
//  the current Trust Level dominates the one specified in the Ace SID.
//  If this flag is set then the SID in the ACE should be a valid TrustLevelSid.
//

#define TRUST_PROTECTED_FILTER_ACE_FLAG  (0x40)


//
//  We'll define the structure of the predefined ACE types.  Pictorally
//  the structure of the predefined ACE's is as follows:
//
//       3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//       1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//      +---------------+-------+-------+---------------+---------------+
//      |    AceFlags   | Resd  |Inherit|    AceSize    |     AceType   |
//      +---------------+-------+-------+---------------+---------------+
//      |                              Mask                             |
//      +---------------------------------------------------------------+
//      |                                                               |
//      +                                                               +
//      |                                                               |
//      +                              Sid                              +
//      |                                                               |
//      +                                                               +
//      |                                                               |
//      +---------------------------------------------------------------+
//
//  Mask is the access mask associated with the ACE.  This is either the
//  access allowed, access denied, audit, or alarm mask.
//
//  Sid is the Sid associated with the ACE.
//

//  The following are the four predefined ACE types.

//  Examine the AceType field in the Header to determine
//  which structure is appropriate to use for casting.


typedef struct _ACCESS_ALLOWED_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} ACCESS_ALLOWED_ACE;

typedef ACCESS_ALLOWED_ACE *PACCESS_ALLOWED_ACE;

typedef struct _ACCESS_DENIED_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} ACCESS_DENIED_ACE;
typedef ACCESS_DENIED_ACE *PACCESS_DENIED_ACE;

typedef struct _SYSTEM_AUDIT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} SYSTEM_AUDIT_ACE;
typedef SYSTEM_AUDIT_ACE *PSYSTEM_AUDIT_ACE;

typedef struct _SYSTEM_ALARM_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} SYSTEM_ALARM_ACE;
typedef SYSTEM_ALARM_ACE *PSYSTEM_ALARM_ACE;

typedef struct _SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
    // Sid followed by CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 structure
} SYSTEM_RESOURCE_ATTRIBUTE_ACE, *PSYSTEM_RESOURCE_ATTRIBUTE_ACE;

typedef struct _SYSTEM_SCOPED_POLICY_ID_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} SYSTEM_SCOPED_POLICY_ID_ACE, *PSYSTEM_SCOPED_POLICY_ID_ACE;

typedef struct _SYSTEM_MANDATORY_LABEL_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} SYSTEM_MANDATORY_LABEL_ACE, *PSYSTEM_MANDATORY_LABEL_ACE;

typedef struct _SYSTEM_PROCESS_TRUST_LABEL_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
} SYSTEM_PROCESS_TRUST_LABEL_ACE, *PSYSTEM_PROCESS_TRUST_LABEL_ACE;

typedef struct _SYSTEM_ACCESS_FILTER_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
    // Filter Condition follows the SID
} SYSTEM_ACCESS_FILTER_ACE, *PSYSTEM_ACCESS_FILTER_ACE;

#define SYSTEM_MANDATORY_LABEL_NO_WRITE_UP         0x1
#define SYSTEM_MANDATORY_LABEL_NO_READ_UP          0x2
#define SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP       0x4

#define SYSTEM_MANDATORY_LABEL_VALID_MASK (SYSTEM_MANDATORY_LABEL_NO_WRITE_UP   | \
                                           SYSTEM_MANDATORY_LABEL_NO_READ_UP    | \
                                           SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP)

// Placeholder value that allows all ranges
#define SYSTEM_PROCESS_TRUST_LABEL_VALID_MASK      0x00ffffff
#define SYSTEM_PROCESS_TRUST_NOCONSTRAINT_MASK     0xffffffff
#define SYSTEM_ACCESS_FILTER_VALID_MASK            0x00ffffff
#define SYSTEM_ACCESS_FILTER_NOCONSTRAINT_MASK     0xffffffff
// end_ntifs


typedef struct _ACCESS_ALLOWED_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
} ACCESS_ALLOWED_OBJECT_ACE, *PACCESS_ALLOWED_OBJECT_ACE;

typedef struct _ACCESS_DENIED_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
} ACCESS_DENIED_OBJECT_ACE, *PACCESS_DENIED_OBJECT_ACE;

typedef struct _SYSTEM_AUDIT_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
} SYSTEM_AUDIT_OBJECT_ACE, *PSYSTEM_AUDIT_OBJECT_ACE;

typedef struct _SYSTEM_ALARM_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
} SYSTEM_ALARM_OBJECT_ACE, *PSYSTEM_ALARM_OBJECT_ACE;

//
// Callback ace support in post Win2000.
// Resource managers can put their own data after Sidstart + Length of the sid
//

typedef struct _ACCESS_ALLOWED_CALLBACK_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
    // Opaque resource manager specific data
} ACCESS_ALLOWED_CALLBACK_ACE, *PACCESS_ALLOWED_CALLBACK_ACE;

typedef struct _ACCESS_DENIED_CALLBACK_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
    // Opaque resource manager specific data
} ACCESS_DENIED_CALLBACK_ACE, *PACCESS_DENIED_CALLBACK_ACE;

typedef struct _SYSTEM_AUDIT_CALLBACK_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
    // Opaque resource manager specific data
} SYSTEM_AUDIT_CALLBACK_ACE, *PSYSTEM_AUDIT_CALLBACK_ACE;

typedef struct _SYSTEM_ALARM_CALLBACK_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD SidStart;
    // Opaque resource manager specific data
} SYSTEM_ALARM_CALLBACK_ACE, *PSYSTEM_ALARM_CALLBACK_ACE;

typedef struct _ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
    // Opaque resource manager specific data
} ACCESS_ALLOWED_CALLBACK_OBJECT_ACE, *PACCESS_ALLOWED_CALLBACK_OBJECT_ACE;

typedef struct _ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
    // Opaque resource manager specific data
} ACCESS_DENIED_CALLBACK_OBJECT_ACE, *PACCESS_DENIED_CALLBACK_OBJECT_ACE;

typedef struct _SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
    // Opaque resource manager specific data
} SYSTEM_AUDIT_CALLBACK_OBJECT_ACE, *PSYSTEM_AUDIT_CALLBACK_OBJECT_ACE;

typedef struct _SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    ACE_HEADER Header;
    ACCESS_MASK Mask;
    DWORD Flags;
    GUID ObjectType;
    GUID InheritedObjectType;
    DWORD SidStart;
    // Opaque resource manager specific data
} SYSTEM_ALARM_CALLBACK_OBJECT_ACE, *PSYSTEM_ALARM_CALLBACK_OBJECT_ACE;

//
// Currently define Flags for "OBJECT" ACE types.
//

#define ACE_OBJECT_TYPE_PRESENT           0x1
#define ACE_INHERITED_OBJECT_TYPE_PRESENT 0x2


//
//  The following declarations are used for setting and querying information
//  about and ACL.  First are the various information classes available to
//  the user.
//

typedef enum _ACL_INFORMATION_CLASS {
    AclRevisionInformation = 1,
    AclSizeInformation
} ACL_INFORMATION_CLASS;

//
//  This record is returned/sent if the user is requesting/setting the
//  AclRevisionInformation
//

typedef struct _ACL_REVISION_INFORMATION {
    DWORD AclRevision;
} ACL_REVISION_INFORMATION;
typedef ACL_REVISION_INFORMATION *PACL_REVISION_INFORMATION;

//
//  This record is returned if the user is requesting AclSizeInformation
//

typedef struct _ACL_SIZE_INFORMATION {
    DWORD AceCount;
    DWORD AclBytesInUse;
    DWORD AclBytesFree;
} ACL_SIZE_INFORMATION;
typedef ACL_SIZE_INFORMATION *PACL_SIZE_INFORMATION;


////////////////////////////////////////////////////////////////////////
//                                                                    //
//                             SECURITY_DESCRIPTOR                    //
//                                                                    //
////////////////////////////////////////////////////////////////////////
//
//  Define the Security Descriptor and related data types.
//  This is an opaque data structure.
//

// begin_wdm
//
// Current security descriptor revision value
//

#define SECURITY_DESCRIPTOR_REVISION     (1)
#define SECURITY_DESCRIPTOR_REVISION1    (1)

// end_wdm
// begin_ntifs

#define SECURITY_DESCRIPTOR_MIN_LENGTH   (sizeof(SECURITY_DESCRIPTOR))


typedef WORD   SECURITY_DESCRIPTOR_CONTROL, *PSECURITY_DESCRIPTOR_CONTROL;

#define SE_OWNER_DEFAULTED               (0x0001)
#define SE_GROUP_DEFAULTED               (0x0002)
#define SE_DACL_PRESENT                  (0x0004)
#define SE_DACL_DEFAULTED                (0x0008)
#define SE_SACL_PRESENT                  (0x0010)
#define SE_SACL_DEFAULTED                (0x0020)
#define SE_DACL_AUTO_INHERIT_REQ         (0x0100)
#define SE_SACL_AUTO_INHERIT_REQ         (0x0200)
#define SE_DACL_AUTO_INHERITED           (0x0400)
#define SE_SACL_AUTO_INHERITED           (0x0800)
#define SE_DACL_PROTECTED                (0x1000)
#define SE_SACL_PROTECTED                (0x2000)
#define SE_RM_CONTROL_VALID              (0x4000)
#define SE_SELF_RELATIVE                 (0x8000)

//
//  Where:
//
//      SE_OWNER_DEFAULTED - This boolean flag, when set, indicates that the
//          SID pointed to by the Owner field was provided by a
//          defaulting mechanism rather than explicitly provided by the
//          original provider of the security descriptor.  This may
//          affect the treatment of the SID with respect to inheritence
//          of an owner.
//
//      SE_GROUP_DEFAULTED - This boolean flag, when set, indicates that the
//          SID in the Group field was provided by a defaulting mechanism
//          rather than explicitly provided by the original provider of
//          the security descriptor.  This may affect the treatment of
//          the SID with respect to inheritence of a primary group.
//
//      SE_DACL_PRESENT - This boolean flag, when set, indicates that the
//          security descriptor contains a discretionary ACL.  If this
//          flag is set and the Dacl field of the SECURITY_DESCRIPTOR is
//          null, then a null ACL is explicitly being specified.
//
//      SE_DACL_DEFAULTED - This boolean flag, when set, indicates that the
//          ACL pointed to by the Dacl field was provided by a defaulting
//          mechanism rather than explicitly provided by the original
//          provider of the security descriptor.  This may affect the
//          treatment of the ACL with respect to inheritence of an ACL.
//          This flag is ignored if the DaclPresent flag is not set.
//
//      SE_SACL_PRESENT - This boolean flag, when set,  indicates that the
//          security descriptor contains a system ACL pointed to by the
//          Sacl field.  If this flag is set and the Sacl field of the
//          SECURITY_DESCRIPTOR is null, then an empty (but present)
//          ACL is being specified.
//
//      SE_SACL_DEFAULTED - This boolean flag, when set, indicates that the
//          ACL pointed to by the Sacl field was provided by a defaulting
//          mechanism rather than explicitly provided by the original
//          provider of the security descriptor.  This may affect the
//          treatment of the ACL with respect to inheritence of an ACL.
//          This flag is ignored if the SaclPresent flag is not set.
//
//      SE_SELF_RELATIVE - This boolean flag, when set, indicates that the
//          security descriptor is in self-relative form.  In this form,
//          all fields of the security descriptor are contiguous in memory
//          and all pointer fields are expressed as offsets from the
//          beginning of the security descriptor.  This form is useful
//          for treating security descriptors as opaque data structures
//          for transmission in communication protocol or for storage on
//          secondary media.
//
//
//
// Pictorially the structure of a security descriptor is as follows:
//
//       3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//       1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//      +---------------------------------------------------------------+
//      |            Control            |Reserved1 (SBZ)|   Revision    |
//      +---------------------------------------------------------------+
//      |                            Owner                              |
//      +---------------------------------------------------------------+
//      |                            Group                              |
//      +---------------------------------------------------------------+
//      |                            Sacl                               |
//      +---------------------------------------------------------------+
//      |                            Dacl                               |
//      +---------------------------------------------------------------+
//
// In general, this data structure should be treated opaquely to ensure future
// compatibility.
//
//

typedef struct _SECURITY_DESCRIPTOR_RELATIVE {
    BYTE  Revision;
    BYTE  Sbz1;
    SECURITY_DESCRIPTOR_CONTROL Control;
    DWORD Owner;
    DWORD Group;
    DWORD Sacl;
    DWORD Dacl;
    } SECURITY_DESCRIPTOR_RELATIVE, *PISECURITY_DESCRIPTOR_RELATIVE;

typedef struct _SECURITY_DESCRIPTOR {
   BYTE  Revision;
   BYTE  Sbz1;
   SECURITY_DESCRIPTOR_CONTROL Control;
   PSID Owner;
   PSID Group;
   PACL Sacl;
   PACL Dacl;

   } SECURITY_DESCRIPTOR, *PISECURITY_DESCRIPTOR;


typedef struct _SECURITY_OBJECT_AI_PARAMS {
    DWORD Size;             //Set to sizeof(SECURITY_OBJECT_AI_PARAMS)
    DWORD ConstraintMask;
} SECURITY_OBJECT_AI_PARAMS, *PSECURITY_OBJECT_AI_PARAMS;


// end_ntifs

// Where:
//
//     Revision - Contains the revision level of the security
//         descriptor.  This allows this structure to be passed between
//         systems or stored on disk even though it is expected to
//         change in the future.
//
//     Control - A set of flags which qualify the meaning of the
//         security descriptor or individual fields of the security
//         descriptor.
//
//     Owner - is a pointer to an SID representing an object's owner.
//         If this field is null, then no owner SID is present in the
//         security descriptor.  If the security descriptor is in
//         self-relative form, then this field contains an offset to
//         the SID, rather than a pointer.
//
//     Group - is a pointer to an SID representing an object's primary
//         group.  If this field is null, then no primary group SID is
//         present in the security descriptor.  If the security descriptor
//         is in self-relative form, then this field contains an offset to
//         the SID, rather than a pointer.
//
//     Sacl - is a pointer to a system ACL.  This field value is only
//         valid if the DaclPresent control flag is set.  If the
//         SaclPresent flag is set and this field is null, then a null
//         ACL  is specified.  If the security descriptor is in
//         self-relative form, then this field contains an offset to
//         the ACL, rather than a pointer.
//
//     Dacl - is a pointer to a discretionary ACL.  This field value is
//         only valid if the DaclPresent control flag is set.  If the
//         DaclPresent flag is set and this field is null, then a null
//         ACL (unconditionally granting access) is specified.  If the
//         security descriptor is in self-relative form, then this field
//         contains an offset to the ACL, rather than a pointer.
//




////////////////////////////////////////////////////////////////////////
//                                                                    //
//               Object Type list for AccessCheckByType               //
//                                                                    //
////////////////////////////////////////////////////////////////////////

typedef struct _OBJECT_TYPE_LIST {
    WORD   Level;
    WORD   Sbz;
    GUID *ObjectType;
} OBJECT_TYPE_LIST, *POBJECT_TYPE_LIST;

//
// DS values for Level
//

#define ACCESS_OBJECT_GUID       0
#define ACCESS_PROPERTY_SET_GUID 1
#define ACCESS_PROPERTY_GUID     2

#define ACCESS_MAX_LEVEL         4

//
// Parameters to NtAccessCheckByTypeAndAditAlarm
//

typedef enum _AUDIT_EVENT_TYPE {
    AuditEventObjectAccess,
    AuditEventDirectoryServiceAccess
} AUDIT_EVENT_TYPE, *PAUDIT_EVENT_TYPE;

#define AUDIT_ALLOW_NO_PRIVILEGE 0x1

//
// DS values for Source and ObjectTypeName
//

#define ACCESS_DS_SOURCE_A "DS"
#define ACCESS_DS_SOURCE_W L"DS"
#define ACCESS_DS_OBJECT_TYPE_NAME_A "Directory Service Object"
#define ACCESS_DS_OBJECT_TYPE_NAME_W L"Directory Service Object"

////////////////////////////////////////////////////////////////////////
//                                                                    //
//               Privilege Related Data Structures                    //
//                                                                    //
////////////////////////////////////////////////////////////////////////

// end_ntifs
// begin_wdm
//
// Privilege attributes
//

#define SE_PRIVILEGE_ENABLED_BY_DEFAULT (0x00000001L)
#define SE_PRIVILEGE_ENABLED            (0x00000002L)
#define SE_PRIVILEGE_REMOVED            (0X00000004L)
#define SE_PRIVILEGE_USED_FOR_ACCESS    (0x80000000L)

#define SE_PRIVILEGE_VALID_ATTRIBUTES   (SE_PRIVILEGE_ENABLED_BY_DEFAULT | \
                                         SE_PRIVILEGE_ENABLED            | \
                                         SE_PRIVILEGE_REMOVED            | \
                                         SE_PRIVILEGE_USED_FOR_ACCESS)


//
// Privilege Set Control flags
//

#define PRIVILEGE_SET_ALL_NECESSARY    (1)

//
//  Privilege Set - This is defined for a privilege set of one.
//                  If more than one privilege is needed, then this structure
//                  will need to be allocated with more space.
//
//  Note: don't change this structure without fixing the INITIAL_PRIVILEGE_SET
//  structure (defined in se.h)
//

typedef struct _PRIVILEGE_SET {
    DWORD PrivilegeCount;
    DWORD Control;
    LUID_AND_ATTRIBUTES Privilege[ANYSIZE_ARRAY];
    } PRIVILEGE_SET, * PPRIVILEGE_SET;




//
// Values for different access granted\denied reasons:
// AccessReasonAceN = AccessReasonAce + N.
// AccessReasonPrivilegeN = AccessReasonPrivilege + N.
//

#define ACCESS_REASON_TYPE_MASK 0x00ff0000
#define ACCESS_REASON_DATA_MASK 0x0000ffff

#define ACCESS_REASON_STAGING_MASK  0x80000000
#define ACCESS_REASON_EXDATA_MASK   0x7f000000

typedef enum _ACCESS_REASON_TYPE{

    AccessReasonNone                    = 0x00000000,   // Indicate no reason for the bit. The bit may not be checked, or just no known reason.

    //
    // The lowest 2 bytes store the index of the ACE that grant/deny this bit.
    // If the corresponding access mask is zero, then it is deny ACE; otherwise,
    // it is allow ACE.
    //
    AccessReasonAllowedAce                  = 0x00010000,   // Granted a permission.
    AccessReasonDeniedAce                   = 0x00020000,   // Denied a permission.

    AccessReasonAllowedParentAce            = 0x00030000,   // Granted a permission from parent ACE
    AccessReasonDeniedParentAce             = 0x00040000,   // Denied a permission from parent ACE

    AccessReasonNotGrantedByCape            = 0x00050000,   // A CAPE didn't grant the permission
    AccessReasonNotGrantedByParentCape      = 0x00060000,   // A CAPE from the parent's SD didn't grant the permission

    AccessReasonNotGrantedToAppContainer    = 0x00070000,   // This is an AppContainer and no ACE granted the permission.

    AccessReasonMissingPrivilege            = 0x00100000,
    AccessReasonFromPrivilege               = 0x00200000,


    AccessReasonIntegrityLevel              = 0x00300000,

    AccessReasonOwnership                   = 0x00400000,

    AccessReasonNullDacl                    = 0x00500000,
    AccessReasonEmptyDacl                   = 0x00600000,

    AccessReasonNoSD                        = 0x00700000,
    AccessReasonNoGrant                     = 0x00800000,   // this access bit is not granted by any ACE.

    AccessReasonTrustLabel                  = 0x00900000,   // The trust label ACE did not grant this access.

    AccessReasonFilterAce                   = 0x00a00000    // The filtering ACE did not grant this access
}
ACCESS_REASON_TYPE;

 //
// Structure to hold access denied\granted reason for every bit of ACCESS_MASK.
// There are 32-bits in ACCESS_MASK and only 27-bits are actually valid on
// return from AccessCheck because MAXIMUM_ALLOWED, GENERIC_READ,
// GENERIC_WRITE, GENERIC_EXECUTE, and GENERIC_ALL are never returned.
//
// The content in Data fields depends on the Access Reason, for example,
// if the reason is AccessReasonAce, the Data will be the ACE ID.
// If there are more than one reason (more than one bit is set), the array size
// of the Data is equal to the number of bits set (or number of reasons).
// The Data could be null for a particular reason.
//

typedef DWORD ACCESS_REASON;

typedef struct _ACCESS_REASONS{
        ACCESS_REASON Data[32];
} ACCESS_REASONS, *PACCESS_REASONS;


/*
The following data structures are defined to consolidate various falvors of
access check functions. In particular for Windows 7, the new access check
function will enable security attribute check, plus returning the reason
for a access check result.

The new access check function based on these data structures will
form the foundation to reimplement other flavors of access check
functions.

*/

//
// Structure to hold pointer to security descriptor and its unique id, which
// can be used for caching access check results.
// (NOTE NOTE) The cache key can be constructed by SecurityDescriptorId, Token and
// PrincipalSelfSid. Watch how GenericMapping affects the cache results.
//
#define SE_SECURITY_DESCRIPTOR_FLAG_NO_OWNER_ACE            0x00000001
#define SE_SECURITY_DESCRIPTOR_FLAG_NO_LABEL_ACE            0x00000002
#define SE_SECURITY_DESCRIPTOR_FLAG_NO_ACCESS_FILTER_ACE    0x00000004
#define SE_SECURITY_DESCRIPTOR_VALID_FLAGS                  0x00000007

#define SE_ACCESS_CHECK_FLAG_NO_LEARNING_MODE_LOGGING       0x00000008
#define SE_ACCESS_CHECK_VALID_FLAGS                         0x00000008


typedef struct _SE_SECURITY_DESCRIPTOR
{
    DWORD Size;
    DWORD Flags;
    PSECURITY_DESCRIPTOR SecurityDescriptor;
} SE_SECURITY_DESCRIPTOR, *PSE_SECURITY_DESCRIPTOR;

typedef struct _SE_ACCESS_REQUEST
{
    DWORD Size;
    PSE_SECURITY_DESCRIPTOR SeSecurityDescriptor;
    ACCESS_MASK DesiredAccess;
    ACCESS_MASK PreviouslyGrantedAccess;
    PSID PrincipalSelfSid;      // Need to watch how this field affects the cache.
    PGENERIC_MAPPING GenericMapping;
    DWORD ObjectTypeListCount;
    POBJECT_TYPE_LIST ObjectTypeList;
} SE_ACCESS_REQUEST, *PSE_ACCESS_REQUEST;


typedef struct _SE_ACCESS_REPLY
{
    DWORD Size;
    DWORD ResultListCount;  // Indicate the array size of GrantedAccess and AccessStatus, it only can be either 1 or ObjectTypeListCount.
    PACCESS_MASK GrantedAccess;
    PDWORD    AccessStatus;
    PACCESS_REASONS AccessReason;
    PPRIVILEGE_SET* Privileges;
} SE_ACCESS_REPLY, *PSE_ACCESS_REPLY;


////////////////////////////////////////////////////////////////////////
//                                                                    //
//               NT Defined Privileges                                //
//                                                                    //
////////////////////////////////////////////////////////////////////////

#define SE_CREATE_TOKEN_NAME                         TEXT("SeCreateTokenPrivilege")
#define SE_ASSIGNPRIMARYTOKEN_NAME                   TEXT("SeAssignPrimaryTokenPrivilege")
#define SE_LOCK_MEMORY_NAME                          TEXT("SeLockMemoryPrivilege")
#define SE_INCREASE_QUOTA_NAME                       TEXT("SeIncreaseQuotaPrivilege")
#define SE_UNSOLICITED_INPUT_NAME                    TEXT("SeUnsolicitedInputPrivilege")
#define SE_MACHINE_ACCOUNT_NAME                      TEXT("SeMachineAccountPrivilege")
#define SE_TCB_NAME                                  TEXT("SeTcbPrivilege")
#define SE_SECURITY_NAME                             TEXT("SeSecurityPrivilege")
#define SE_TAKE_OWNERSHIP_NAME                       TEXT("SeTakeOwnershipPrivilege")
#define SE_LOAD_DRIVER_NAME                          TEXT("SeLoadDriverPrivilege")
#define SE_SYSTEM_PROFILE_NAME                       TEXT("SeSystemProfilePrivilege")
#define SE_SYSTEMTIME_NAME                           TEXT("SeSystemtimePrivilege")
#define SE_PROF_SINGLE_PROCESS_NAME                  TEXT("SeProfileSingleProcessPrivilege")
#define SE_INC_BASE_PRIORITY_NAME                    TEXT("SeIncreaseBasePriorityPrivilege")
#define SE_CREATE_PAGEFILE_NAME                      TEXT("SeCreatePagefilePrivilege")
#define SE_CREATE_PERMANENT_NAME                     TEXT("SeCreatePermanentPrivilege")
#define SE_BACKUP_NAME                               TEXT("SeBackupPrivilege")
#define SE_RESTORE_NAME                              TEXT("SeRestorePrivilege")
#define SE_SHUTDOWN_NAME                             TEXT("SeShutdownPrivilege")
#define SE_DEBUG_NAME                                TEXT("SeDebugPrivilege")
#define SE_AUDIT_NAME                                TEXT("SeAuditPrivilege")
#define SE_SYSTEM_ENVIRONMENT_NAME                   TEXT("SeSystemEnvironmentPrivilege")
#define SE_CHANGE_NOTIFY_NAME                        TEXT("SeChangeNotifyPrivilege")
#define SE_REMOTE_SHUTDOWN_NAME                      TEXT("SeRemoteShutdownPrivilege")
#define SE_UNDOCK_NAME                               TEXT("SeUndockPrivilege")
#define SE_SYNC_AGENT_NAME                           TEXT("SeSyncAgentPrivilege")
#define SE_ENABLE_DELEGATION_NAME                    TEXT("SeEnableDelegationPrivilege")
#define SE_MANAGE_VOLUME_NAME                        TEXT("SeManageVolumePrivilege")
#define SE_IMPERSONATE_NAME                          TEXT("SeImpersonatePrivilege")
#define SE_CREATE_GLOBAL_NAME                        TEXT("SeCreateGlobalPrivilege")
#define SE_TRUSTED_CREDMAN_ACCESS_NAME               TEXT("SeTrustedCredManAccessPrivilege")
#define SE_RELABEL_NAME                              TEXT("SeRelabelPrivilege")
#define SE_INC_WORKING_SET_NAME                      TEXT("SeIncreaseWorkingSetPrivilege")
#define SE_TIME_ZONE_NAME                            TEXT("SeTimeZonePrivilege")
#define SE_CREATE_SYMBOLIC_LINK_NAME                 TEXT("SeCreateSymbolicLinkPrivilege")
#define SE_DELEGATE_SESSION_USER_IMPERSONATE_NAME    TEXT("SeDelegateSessionUserImpersonatePrivilege")

// begin_ntosifs

//
// List Of String Capabilities.
//
#define SE_ACTIVATE_AS_USER_CAPABILITY L"activateAsUser"
#define SE_CONSTRAINED_IMPERSONATION_CAPABILITY L"constrainedImpersonation"
#define SE_SESSION_IMPERSONATION_CAPABILITY L"sessionImpersonation"
#define SE_MUMA_CAPABILITY L"muma"
#define SE_DEVELOPMENT_MODE_NETWORK_CAPABILITY L"developmentModeNetwork"
#define SE_LEARNING_MODE_LOGGING_CAPABILITY L"learningModeLogging"
#define SE_PERMISSIVE_LEARNING_MODE_CAPABILITY L"permissiveLearningMode"
#define SE_APP_SILO_VOLUME_ROOT_MINIMAL_CAPABILITY L"isolatedWin32-volumeRootMinimal"
#define SE_APP_SILO_PROFILES_ROOT_MINIMAL_CAPABILITY L"isolatedWin32-profilesRootMinimal"
#define SE_APP_SILO_USER_PROFILE_MINIMAL_CAPABILITY L"isolatedWin32-userProfileMinimal"
#define SE_APP_SILO_PROMPT_FOR_ACCESS_CAPABILITY L"isolatedWin32-promptForAccess"
#define SE_APP_SILO_ACCESS_TO_PUBLISHER_DIRECTORY_CAPABILITY L"isolatedWin32-accessToPublisherDirectory"
#define SE_APP_SILO_PRINT_CAPABILITY L"isolatedWin32-print"

// end_ntosifs



////////////////////////////////////////////////////////////////////
//                                                                //
//           Security Quality Of Service                          //
//                                                                //
//                                                                //
////////////////////////////////////////////////////////////////////

// begin_wdm
//
// Impersonation Level
//
// Impersonation level is represented by a pair of bits in Windows.
// If a new impersonation level is added or lowest value is changed from
// 0 to something else, fix the Windows CreateFile call.
//

typedef enum _SECURITY_IMPERSONATION_LEVEL {
    SecurityAnonymous,
    SecurityIdentification,
    SecurityImpersonation,
    SecurityDelegation
    } SECURITY_IMPERSONATION_LEVEL, * PSECURITY_IMPERSONATION_LEVEL;

#define SECURITY_MAX_IMPERSONATION_LEVEL SecurityDelegation
#define SECURITY_MIN_IMPERSONATION_LEVEL SecurityAnonymous
#define DEFAULT_IMPERSONATION_LEVEL SecurityImpersonation
#define VALID_IMPERSONATION_LEVEL(L) (((L) >= SECURITY_MIN_IMPERSONATION_LEVEL) && ((L) <= SECURITY_MAX_IMPERSONATION_LEVEL))


////////////////////////////////////////////////////////////////////
//                                                                //
//           Token Object Definitions                             //
//                                                                //
//                                                                //
////////////////////////////////////////////////////////////////////

// begin_access

//
// Token Specific Access Rights.
//

#define TOKEN_ASSIGN_PRIMARY    (0x0001)
#define TOKEN_DUPLICATE         (0x0002)
#define TOKEN_IMPERSONATE       (0x0004)
#define TOKEN_QUERY             (0x0008)
#define TOKEN_QUERY_SOURCE      (0x0010)
#define TOKEN_ADJUST_PRIVILEGES (0x0020)
#define TOKEN_ADJUST_GROUPS     (0x0040)
#define TOKEN_ADJUST_DEFAULT    (0x0080)
#define TOKEN_ADJUST_SESSIONID  (0x0100)

#define TOKEN_ALL_ACCESS_P (STANDARD_RIGHTS_REQUIRED  |\
                          TOKEN_ASSIGN_PRIMARY      |\
                          TOKEN_DUPLICATE           |\
                          TOKEN_IMPERSONATE         |\
                          TOKEN_QUERY               |\
                          TOKEN_QUERY_SOURCE        |\
                          TOKEN_ADJUST_PRIVILEGES   |\
                          TOKEN_ADJUST_GROUPS       |\
                          TOKEN_ADJUST_DEFAULT )

#if ((defined(_WIN32_WINNT) && (_WIN32_WINNT > 0x0400)) || (!defined(_WIN32_WINNT)))
#define TOKEN_ALL_ACCESS  (TOKEN_ALL_ACCESS_P |\
                          TOKEN_ADJUST_SESSIONID )
#else
#define TOKEN_ALL_ACCESS  (TOKEN_ALL_ACCESS_P)
#endif

#define TOKEN_READ       (STANDARD_RIGHTS_READ      |\
                          TOKEN_QUERY)


#define TOKEN_WRITE      (STANDARD_RIGHTS_WRITE     |\
                          TOKEN_ADJUST_PRIVILEGES   |\
                          TOKEN_ADJUST_GROUPS       |\
                          TOKEN_ADJUST_DEFAULT)

#define TOKEN_EXECUTE    (STANDARD_RIGHTS_EXECUTE)

#define TOKEN_TRUST_CONSTRAINT_MASK    (STANDARD_RIGHTS_READ  | \
                                       TOKEN_QUERY  |\
                                       TOKEN_QUERY_SOURCE )

#define TOKEN_TRUST_ALLOWED_MASK    (TOKEN_TRUST_CONSTRAINT_MASK |\
                                    TOKEN_DUPLICATE              |\
                                    TOKEN_IMPERSONATE)

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define TOKEN_ACCESS_PSEUDO_HANDLE_WIN8 (TOKEN_QUERY | TOKEN_QUERY_SOURCE)

#define TOKEN_ACCESS_PSEUDO_HANDLE TOKEN_ACCESS_PSEUDO_HANDLE_WIN8

#endif
//
// end_access
//
//
// Token Types
//

typedef enum _TOKEN_TYPE {
    TokenPrimary = 1,
    TokenImpersonation
    } TOKEN_TYPE;
typedef TOKEN_TYPE *PTOKEN_TYPE;

//
// Token elevation values describe the relative strength of a given token.
// A full token is a token with all groups and privileges to which the principal
// is authorized.  A limited token is one with some groups or privileges removed.
//

typedef enum _TOKEN_ELEVATION_TYPE {
    TokenElevationTypeDefault = 1,
    TokenElevationTypeFull,
    TokenElevationTypeLimited,
} TOKEN_ELEVATION_TYPE, *PTOKEN_ELEVATION_TYPE;

//
// Token Information Classes.
//


typedef enum _TOKEN_INFORMATION_CLASS {
    TokenUser = 1,
    TokenGroups,
    TokenPrivileges,
    TokenOwner,
    TokenPrimaryGroup,
    TokenDefaultDacl,
    TokenSource,
    TokenType,
    TokenImpersonationLevel,
    TokenStatistics,
    TokenRestrictedSids,
    TokenSessionId,
    TokenGroupsAndPrivileges,
    TokenSessionReference,
    TokenSandBoxInert,
    TokenAuditPolicy,
    TokenOrigin,
    TokenElevationType,
    TokenLinkedToken,
    TokenElevation,
    TokenHasRestrictions,
    TokenAccessInformation,
    TokenVirtualizationAllowed,
    TokenVirtualizationEnabled,
    TokenIntegrityLevel,
    TokenUIAccess,
    TokenMandatoryPolicy,
    TokenLogonSid,
    TokenIsAppContainer,
    TokenCapabilities,
    TokenAppContainerSid,
    TokenAppContainerNumber,
    TokenUserClaimAttributes,
    TokenDeviceClaimAttributes,
    TokenRestrictedUserClaimAttributes,
    TokenRestrictedDeviceClaimAttributes,
    TokenDeviceGroups,
    TokenRestrictedDeviceGroups,
    TokenSecurityAttributes,
    TokenIsRestricted,
    TokenProcessTrustLevel,
    TokenPrivateNameSpace,
    TokenSingletonAttributes,
    TokenBnoIsolation,
    TokenChildProcessFlags,
    TokenIsLessPrivilegedAppContainer,
    TokenIsSandboxed,
    TokenIsAppSilo,
    TokenLoggingInformation,
    TokenLearningMode,
    MaxTokenInfoClass  // MaxTokenInfoClass should always be the last enum
} TOKEN_INFORMATION_CLASS, *PTOKEN_INFORMATION_CLASS;

//
// Token information class structures
//

typedef struct _TOKEN_USER {
    SID_AND_ATTRIBUTES User;
} TOKEN_USER, *PTOKEN_USER;

#ifndef MIDL_PASS

typedef struct _SE_TOKEN_USER {
    union {
        TOKEN_USER TokenUser;
        SID_AND_ATTRIBUTES User;
    } DUMMYUNIONNAME;

    union {
        SID Sid;
        BYTE  Buffer[SECURITY_MAX_SID_SIZE];
    } DUMMYUNIONNAME2;

} SE_TOKEN_USER , PSE_TOKEN_USER;

#define TOKEN_USER_MAX_SIZE (sizeof(TOKEN_USER) + SECURITY_MAX_SID_SIZE)

#endif


typedef struct _TOKEN_GROUPS {
    DWORD GroupCount;
#ifdef MIDL_PASS
    [size_is(GroupCount)] SID_AND_ATTRIBUTES Groups[*];
#else // MIDL_PASS
    SID_AND_ATTRIBUTES Groups[ANYSIZE_ARRAY];
#endif // MIDL_PASS
} TOKEN_GROUPS, *PTOKEN_GROUPS;

typedef struct _TOKEN_PRIVILEGES {
    DWORD PrivilegeCount;
    LUID_AND_ATTRIBUTES Privileges[ANYSIZE_ARRAY];
} TOKEN_PRIVILEGES, *PTOKEN_PRIVILEGES;


typedef struct _TOKEN_OWNER {
    PSID Owner;
} TOKEN_OWNER, *PTOKEN_OWNER;

#ifndef MIDL_PASS
#define TOKEN_OWNER_MAX_SIZE (sizeof(TOKEN_OWNER) + SECURITY_MAX_SID_SIZE)
#endif

typedef struct _TOKEN_PRIMARY_GROUP {
    PSID PrimaryGroup;
} TOKEN_PRIMARY_GROUP, *PTOKEN_PRIMARY_GROUP;


typedef struct _TOKEN_DEFAULT_DACL {
    PACL DefaultDacl;
} TOKEN_DEFAULT_DACL, *PTOKEN_DEFAULT_DACL;

typedef struct _TOKEN_USER_CLAIMS {
    PCLAIMS_BLOB UserClaims;
} TOKEN_USER_CLAIMS, *PTOKEN_USER_CLAIMS;

typedef struct _TOKEN_DEVICE_CLAIMS {
    PCLAIMS_BLOB DeviceClaims;
} TOKEN_DEVICE_CLAIMS, *PTOKEN_DEVICE_CLAIMS;

typedef struct _TOKEN_GROUPS_AND_PRIVILEGES {
    DWORD SidCount;
    DWORD SidLength;
    PSID_AND_ATTRIBUTES Sids;
    DWORD RestrictedSidCount;
    DWORD RestrictedSidLength;
    PSID_AND_ATTRIBUTES RestrictedSids;
    DWORD PrivilegeCount;
    DWORD PrivilegeLength;
    PLUID_AND_ATTRIBUTES Privileges;
    LUID AuthenticationId;
} TOKEN_GROUPS_AND_PRIVILEGES, *PTOKEN_GROUPS_AND_PRIVILEGES;

typedef struct _TOKEN_LINKED_TOKEN {
    HANDLE LinkedToken;
} TOKEN_LINKED_TOKEN, *PTOKEN_LINKED_TOKEN;

typedef struct _TOKEN_ELEVATION {
    DWORD TokenIsElevated;
} TOKEN_ELEVATION, *PTOKEN_ELEVATION;

typedef struct _TOKEN_MANDATORY_LABEL {
    SID_AND_ATTRIBUTES Label;
} TOKEN_MANDATORY_LABEL, *PTOKEN_MANDATORY_LABEL;

#define TOKEN_MANDATORY_POLICY_OFF             0x0
#define TOKEN_MANDATORY_POLICY_NO_WRITE_UP     0x1
#define TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN 0x2

#define TOKEN_MANDATORY_POLICY_VALID_MASK      (TOKEN_MANDATORY_POLICY_NO_WRITE_UP | \
                                                TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN)

#ifndef MIDL_PASS
#define TOKEN_INTEGRITY_LEVEL_MAX_SIZE ((((DWORD)(sizeof(TOKEN_MANDATORY_LABEL)) + sizeof(PVOID) - 1) & ~(sizeof(PVOID)-1)) + SECURITY_MAX_SID_SIZE)
#endif

typedef struct _TOKEN_MANDATORY_POLICY {
    DWORD Policy;
} TOKEN_MANDATORY_POLICY, *PTOKEN_MANDATORY_POLICY;

typedef PVOID PSECURITY_ATTRIBUTES_OPAQUE;

typedef struct _TOKEN_ACCESS_INFORMATION {
    PSID_AND_ATTRIBUTES_HASH SidHash;
    PSID_AND_ATTRIBUTES_HASH RestrictedSidHash;
    PTOKEN_PRIVILEGES Privileges;
    LUID AuthenticationId;
    TOKEN_TYPE TokenType;
    SECURITY_IMPERSONATION_LEVEL ImpersonationLevel;
    TOKEN_MANDATORY_POLICY MandatoryPolicy;
    DWORD Flags;
    DWORD AppContainerNumber;
    PSID PackageSid;
    PSID_AND_ATTRIBUTES_HASH CapabilitiesHash;
    PSID TrustLevelSid;
    PSECURITY_ATTRIBUTES_OPAQUE SecurityAttributes;
} TOKEN_ACCESS_INFORMATION, *PTOKEN_ACCESS_INFORMATION;

typedef struct _TOKEN_LOGGING_INFORMATION {
    TOKEN_TYPE TokenType;
    TOKEN_ELEVATION TokenElevation;
    TOKEN_ELEVATION_TYPE TokenElevationType;
    SECURITY_IMPERSONATION_LEVEL ImpersonationLevel;
    DWORD IntegrityLevel;
    SID_AND_ATTRIBUTES User;
    PSID TrustLevelSid;
    DWORD SessionId;
    DWORD AppContainerNumber;
    LUID AuthenticationId;
    DWORD GroupCount;
    DWORD GroupsLength;
    PSID_AND_ATTRIBUTES Groups;
} TOKEN_LOGGING_INFORMATION, *PTOKEN_LOGGING_INFORMATION;

//
// Valid bits for each TOKEN_AUDIT_POLICY policy mask field.
//

#define POLICY_AUDIT_SUBCATEGORY_COUNT (60)

typedef struct _TOKEN_AUDIT_POLICY {
    BYTE  PerUserPolicy[((POLICY_AUDIT_SUBCATEGORY_COUNT) >> 1) + 1];
} TOKEN_AUDIT_POLICY, *PTOKEN_AUDIT_POLICY;

#define TOKEN_SOURCE_LENGTH 8

typedef struct _TOKEN_SOURCE {
    CHAR SourceName[TOKEN_SOURCE_LENGTH];
    LUID SourceIdentifier;
} TOKEN_SOURCE, *PTOKEN_SOURCE;


typedef struct _TOKEN_STATISTICS {
    LUID TokenId;
    LUID AuthenticationId;
    LARGE_INTEGER ExpirationTime;
    TOKEN_TYPE TokenType;
    SECURITY_IMPERSONATION_LEVEL ImpersonationLevel;
    DWORD DynamicCharged;
    DWORD DynamicAvailable;
    DWORD GroupCount;
    DWORD PrivilegeCount;
    LUID ModifiedId;
} TOKEN_STATISTICS, *PTOKEN_STATISTICS;



typedef struct _TOKEN_CONTROL {
    LUID TokenId;
    LUID AuthenticationId;
    LUID ModifiedId;
    TOKEN_SOURCE TokenSource;
} TOKEN_CONTROL, *PTOKEN_CONTROL;

typedef struct _TOKEN_ORIGIN {
    LUID OriginatingLogonSession ;
} TOKEN_ORIGIN, * PTOKEN_ORIGIN ;


typedef enum _MANDATORY_LEVEL {
    MandatoryLevelUntrusted = 0,
    MandatoryLevelLow,
    MandatoryLevelMedium,
    MandatoryLevelHigh,
    MandatoryLevelSystem,
    MandatoryLevelSecureProcess,
    MandatoryLevelCount
} MANDATORY_LEVEL, *PMANDATORY_LEVEL;

typedef struct _TOKEN_APPCONTAINER_INFORMATION {
    PSID TokenAppContainer;
} TOKEN_APPCONTAINER_INFORMATION, *PTOKEN_APPCONTAINER_INFORMATION;

#ifndef MIDL_PASS
#define TOKEN_APPCONTAINER_SID_MAX_SIZE (sizeof(TOKEN_APPCONTAINER_INFORMATION) + SECURITY_MAX_SID_SIZE)
#endif

typedef struct _TOKEN_SID_INFORMATION {
    PSID Sid;
} TOKEN_SID_INFORMATION, *PTOKEN_SID_INFORMATION;

typedef struct _TOKEN_BNO_ISOLATION_INFORMATION {
    PWSTR       IsolationPrefix;
    BOOLEAN     IsolationEnabled;
} TOKEN_BNO_ISOLATION_INFORMATION, *PTOKEN_BNO_ISOLATION_INFORMATION;

//
//  *** Claim Security attributes ***
//
//      These #defines and data structures (almost) exactly mirror
//      the Token_XXX definitions (except for PWSTR/PUNICODE changes)
//      in ntseapi.w as well as AUTHZ_XXX in authz.w.
//      Keep them in sync.
//
//
//  Security attribute data types ...
//

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID   0x00

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64     0x01
#define CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64    0x02

//
//  Case insensitive attribute value string by default.
//  Unless the flag CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE
//  is set indicating otherwise.
//

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING    0x03

//
//  Fully-qualified binary name.
//

typedef struct _CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    DWORD64             Version;
    PWSTR               Name;
} CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE, *PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE;

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN      0x04

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_SID       0x05

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN   0x06


typedef struct _CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    PVOID   pValue;         //  Pointer is BYTE aligned.
    DWORD   ValueLength;    //  In bytes
} CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
    *PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE;

#define CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING  0x10

//
// Attribute Flags
//

//
//  Attribute must not be inherited across process spawns.
//

#define CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE      0x0001


//
//  Attribute value is compared in a case sensitive way. It is valid with string value
//  or composite type containing string value. For other types of value, this flag
//  will be ignored. Currently, it is valid with the two types:
//  CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING and CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN.
//
#define CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE         0x0002

//
// Attribute is considered only for Deny Aces.
//

#define CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY 0x0004

//
// Attribute is disabled by default.
//

#define CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT 0x0008

//
// Attribute is disabled.
//

#define CLAIM_SECURITY_ATTRIBUTE_DISABLED 0x0010

//
// Attribute is mandatory.
//

#define CLAIM_SECURITY_ATTRIBUTE_MANDATORY 0x0020


#define CLAIM_SECURITY_ATTRIBUTE_VALID_FLAGS   (    \
                        CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE       |  \
                        CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE  |  \
                        CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY     |  \
                        CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT   |  \
                        CLAIM_SECURITY_ATTRIBUTE_DISABLED              |  \
                        CLAIM_SECURITY_ATTRIBUTE_MANDATORY )


//
// Reserve upper 16 bits for custom flags. These should be preserved but not
// validated as they do not affect security in any way.
//
#define CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS   0xFFFF0000

//
//  An individual security attribute.
//


typedef struct _CLAIM_SECURITY_ATTRIBUTE_V1 {

    //
    //  Name of the attribute.
    //  Case insensitive Unicode string.
    //

    PWSTR   Name;

    //
    //  Data type of attribute.
    //

    WORD    ValueType;

    //
    //  Pass 0 in a set operation and check for 0 in
    //  a get operation.
    //

    WORD    Reserved;

    //
    // Attribute Flags
    //

    DWORD   Flags;

    //
    //  Number of values.
    //

    DWORD   ValueCount;

    //
    //  The actual value itself.
    //

    union {
        PLONG64                                         pInt64;
        PDWORD64                                        pUint64;
        PWSTR                                           *ppString;
        PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE            pFqbn;
        PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE    pOctetString;
    } Values;
} CLAIM_SECURITY_ATTRIBUTE_V1, *PCLAIM_SECURITY_ATTRIBUTE_V1;

//
//  Relative form of the security attribute.
//


typedef struct _CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {

    //
    //  Name of the attribute.
    //  Offset from beginning of structure.
    //

    DWORD   Name;

    //
    //  Data type of attribute.
    //

    WORD    ValueType;

    //
    //  Pass 0 in a set operation and check for 0 in
    //  a get operation.
    //

    WORD    Reserved;

    //
    // Attribute Flags
    //

    DWORD   Flags;

    //
    //  Number of values.
    //

    DWORD   ValueCount;

    //
    //  The actual value itself.
    //

    union {
        DWORD pInt64[ANYSIZE_ARRAY];
        DWORD pUint64[ANYSIZE_ARRAY];
        DWORD ppString[ANYSIZE_ARRAY];
        DWORD pFqbn[ANYSIZE_ARRAY];
        DWORD pOctetString[ANYSIZE_ARRAY];
    } Values;
} CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1, *PCLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1;


//
//  Set of security attributes.
//

//
//  Versioning. The interpretation of the pointers in the
//  Attribute field below is dependent on the version field.
//
//  Get operations return the version while the set operation
//  MUST specify the version of the data structure passed in.
//

#define CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1    1

#define CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION       \
    CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1



typedef struct _CLAIM_SECURITY_ATTRIBUTES_INFORMATION {

    //
    //  MUST BE first.
    //

    WORD    Version;

    //
    //  Pass 0 in set operations and ignore on get operations.
    //

    WORD    Reserved;

    DWORD   AttributeCount;
    union {
        PCLAIM_SECURITY_ATTRIBUTE_V1    pAttributeV1;
    } Attribute;
} CLAIM_SECURITY_ATTRIBUTES_INFORMATION, *PCLAIM_SECURITY_ATTRIBUTES_INFORMATION;

//
// Security Tracking Mode
//

#define SECURITY_DYNAMIC_TRACKING      (TRUE)
#define SECURITY_STATIC_TRACKING       (FALSE)

typedef BOOLEAN SECURITY_CONTEXT_TRACKING_MODE,
                    * PSECURITY_CONTEXT_TRACKING_MODE;



//
// Quality Of Service
//

typedef struct _SECURITY_QUALITY_OF_SERVICE {
    DWORD Length;
    SECURITY_IMPERSONATION_LEVEL ImpersonationLevel;
    SECURITY_CONTEXT_TRACKING_MODE ContextTrackingMode;
    BOOLEAN EffectiveOnly;
    } SECURITY_QUALITY_OF_SERVICE, * PSECURITY_QUALITY_OF_SERVICE;


//
// Used to represent information related to a thread impersonation
//

typedef struct _SE_IMPERSONATION_STATE {
    PACCESS_TOKEN Token;
    BOOLEAN CopyOnOpen;
    BOOLEAN EffectiveOnly;
    SECURITY_IMPERSONATION_LEVEL Level;
} SE_IMPERSONATION_STATE, *PSE_IMPERSONATION_STATE;

#define DISABLE_MAX_PRIVILEGE   0x1 
#define SANDBOX_INERT           0x2 
#define LUA_TOKEN               0x4 
#define WRITE_RESTRICTED        0x8 

typedef DWORD SECURITY_INFORMATION, *PSECURITY_INFORMATION;

#define OWNER_SECURITY_INFORMATION                  (0x00000001L)
#define GROUP_SECURITY_INFORMATION                  (0x00000002L)
#define DACL_SECURITY_INFORMATION                   (0x00000004L)
#define SACL_SECURITY_INFORMATION                   (0x00000008L)
#define LABEL_SECURITY_INFORMATION                  (0x00000010L)
#define ATTRIBUTE_SECURITY_INFORMATION              (0x00000020L)
#define SCOPE_SECURITY_INFORMATION                  (0x00000040L)
#define PROCESS_TRUST_LABEL_SECURITY_INFORMATION    (0x00000080L)
#define ACCESS_FILTER_SECURITY_INFORMATION          (0x00000100L)
#define BACKUP_SECURITY_INFORMATION                 (0x00010000L)

#define PROTECTED_DACL_SECURITY_INFORMATION         (0x80000000L)
#define PROTECTED_SACL_SECURITY_INFORMATION         (0x40000000L)
#define UNPROTECTED_DACL_SECURITY_INFORMATION       (0x20000000L)
#define UNPROTECTED_SACL_SECURITY_INFORMATION       (0x10000000L)


//
// Base signing levels.
//

typedef BYTE  SE_SIGNING_LEVEL, *PSE_SIGNING_LEVEL;

#define SE_SIGNING_LEVEL_UNCHECKED         0x00000000
#define SE_SIGNING_LEVEL_UNSIGNED          0x00000001
#define SE_SIGNING_LEVEL_ENTERPRISE        0x00000002
#define SE_SIGNING_LEVEL_CUSTOM_1          0x00000003
#define SE_SIGNING_LEVEL_DEVELOPER         SE_SIGNING_LEVEL_CUSTOM_1
#define SE_SIGNING_LEVEL_AUTHENTICODE      0x00000004
#define SE_SIGNING_LEVEL_CUSTOM_2          0x00000005
#define SE_SIGNING_LEVEL_STORE             0x00000006
#define SE_SIGNING_LEVEL_CUSTOM_3          0x00000007
#define SE_SIGNING_LEVEL_ANTIMALWARE       SE_SIGNING_LEVEL_CUSTOM_3
#define SE_SIGNING_LEVEL_MICROSOFT         0x00000008
#define SE_SIGNING_LEVEL_CUSTOM_4          0x00000009
#define SE_SIGNING_LEVEL_CUSTOM_5          0x0000000A
#define SE_SIGNING_LEVEL_DYNAMIC_CODEGEN   0x0000000B
#define SE_SIGNING_LEVEL_WINDOWS           0x0000000C
#define SE_SIGNING_LEVEL_CUSTOM_7          0x0000000D
#define SE_SIGNING_LEVEL_WINDOWS_TCB       0x0000000E
#define SE_SIGNING_LEVEL_CUSTOM_6          0x0000000F

//
// Image signature types.
//

typedef enum _SE_IMAGE_SIGNATURE_TYPE
{
    SeImageSignatureNone = 0,
    SeImageSignatureEmbedded,
    SeImageSignatureCache,
    SeImageSignatureCatalogCached,
    SeImageSignatureCatalogNotCached,
    SeImageSignatureCatalogHint,
    SeImageSignaturePackageCatalog,
    SeImageSignaturePplMitigated
} SE_IMAGE_SIGNATURE_TYPE, *PSE_IMAGE_SIGNATURE_TYPE;


typedef struct _SECURITY_CAPABILITIES {
#ifdef MIDL_PASS
    PISID AppContainerSid;
    [size_is(CapabilityCount)] PSID_AND_ATTRIBUTES Capabilities;
#else // MIDL_PASS
    PSID AppContainerSid;
    PSID_AND_ATTRIBUTES Capabilities;
#endif // MIDL_PASS
    DWORD CapabilityCount;
    DWORD Reserved;
} SECURITY_CAPABILITIES, *PSECURITY_CAPABILITIES, *LPSECURITY_CAPABILITIES;


#define PROCESS_TERMINATE                  (0x0001)  
#define PROCESS_CREATE_THREAD              (0x0002)  
#define PROCESS_SET_SESSIONID              (0x0004)  
#define PROCESS_VM_OPERATION               (0x0008)  
#define PROCESS_VM_READ                    (0x0010)  
#define PROCESS_VM_WRITE                   (0x0020)  
#define PROCESS_DUP_HANDLE                 (0x0040)  
#define PROCESS_CREATE_PROCESS             (0x0080)  
#define PROCESS_SET_QUOTA                  (0x0100)  
#define PROCESS_SET_INFORMATION            (0x0200)  
#define PROCESS_QUERY_INFORMATION          (0x0400)  
#define PROCESS_SUSPEND_RESUME             (0x0800)  
#define PROCESS_QUERY_LIMITED_INFORMATION  (0x1000)  
#define PROCESS_SET_LIMITED_INFORMATION    (0x2000)  
//

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define PROCESS_ALL_ACCESS        (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | \
                                   0xFFFF)
#else
#define PROCESS_ALL_ACCESS        (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | \
                                   0xFFF)
#endif

//
#define THREAD_TERMINATE                 (0x0001)  
#define THREAD_SUSPEND_RESUME            (0x0002)  
#define THREAD_GET_CONTEXT               (0x0008)  
#define THREAD_SET_CONTEXT               (0x0010)  
#define THREAD_QUERY_INFORMATION         (0x0040)  
#define THREAD_SET_INFORMATION           (0x0020)  
#define THREAD_SET_THREAD_TOKEN          (0x0080)
#define THREAD_IMPERSONATE               (0x0100)
#define THREAD_DIRECT_IMPERSONATION      (0x0200)
#define THREAD_SET_LIMITED_INFORMATION   (0x0400)  // winnt
#define THREAD_QUERY_LIMITED_INFORMATION (0x0800)  // winnt
#define THREAD_RESUME                    (0x1000)  // winnt

#if (NTDDI_VERSION >= NTDDI_VISTA)
#define THREAD_ALL_ACCESS         (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | \
                                   0xFFFF)
#else
#define THREAD_ALL_ACCESS         (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | \
                                   0x3FF)
#endif

//
//

//
// Job Object Specific Access Rights
//

#define JOB_OBJECT_ASSIGN_PROCESS           (0x0001)
#define JOB_OBJECT_SET_ATTRIBUTES           (0x0002)
#define JOB_OBJECT_QUERY                    (0x0004)
#define JOB_OBJECT_TERMINATE                (0x0008)
#define JOB_OBJECT_SET_SECURITY_ATTRIBUTES  (0x0010)
#define JOB_OBJECT_IMPERSONATE              (0x0020)
#define JOB_OBJECT_ALL_ACCESS       (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | \
                                        0x3F )

//
//

typedef struct _JOB_SET_ARRAY {
    HANDLE JobHandle;   // Handle to job object to insert
    DWORD MemberLevel;  // Level of this job in the set. Must be > 0. Can be sparse.
    DWORD Flags;        // Unused. Must be zero
} JOB_SET_ARRAY, *PJOB_SET_ARRAY;

//
#define FLS_MAXIMUM_AVAILABLE 4080  
#define TLS_MINIMUM_AVAILABLE 64    
//

typedef struct _EXCEPTION_REGISTRATION_RECORD {
    struct _EXCEPTION_REGISTRATION_RECORD *Next;
    PEXCEPTION_ROUTINE Handler;
} EXCEPTION_REGISTRATION_RECORD;

typedef EXCEPTION_REGISTRATION_RECORD *PEXCEPTION_REGISTRATION_RECORD;

//@[comment("MVI_tracked")]
typedef struct _NT_TIB {
    struct _EXCEPTION_REGISTRATION_RECORD *ExceptionList;
    PVOID StackBase;
    PVOID StackLimit;
    PVOID SubSystemTib;
#if defined(_MSC_EXTENSIONS)
    union {
        PVOID FiberData;
        DWORD Version;
    };
#else
    PVOID FiberData;
#endif
    PVOID ArbitraryUserPointer;
    struct _NT_TIB *Self;
} NT_TIB;
typedef NT_TIB *PNT_TIB;

//
// 32 and 64 bit specific version for wow64 and the debugger
//
typedef struct _NT_TIB32 {
    DWORD ExceptionList;
    DWORD StackBase;
    DWORD StackLimit;
    DWORD SubSystemTib;

#if defined(_MSC_EXTENSIONS)
    union {
        DWORD FiberData;
        DWORD Version;
    };
#else
    DWORD FiberData;
#endif

    DWORD ArbitraryUserPointer;
    DWORD Self;
} NT_TIB32, *PNT_TIB32;

typedef struct _NT_TIB64 {
    DWORD64 ExceptionList;
    DWORD64 StackBase;
    DWORD64 StackLimit;
    DWORD64 SubSystemTib;

#if defined(_MSC_EXTENSIONS)
    union {
        DWORD64 FiberData;
        DWORD Version;
    };

#else
    DWORD64 FiberData;
#endif

    DWORD64 ArbitraryUserPointer;
    DWORD64 Self;
} NT_TIB64, *PNT_TIB64;

//
//

#define THREAD_DYNAMIC_CODE_ALLOW   1     // Opt-out of dynamic code generation.

#define THREAD_BASE_PRIORITY_LOWRT  15  // value that gets a thread to LowRealtime-1
#define THREAD_BASE_PRIORITY_MAX    2   // maximum thread base priority boost
#define THREAD_BASE_PRIORITY_MIN    (-2)  // minimum thread base priority boost
#define THREAD_BASE_PRIORITY_IDLE   (-15) // value that gets a thread to idle

//
typedef struct _UMS_CREATE_THREAD_ATTRIBUTES {  
    DWORD UmsVersion;   
        PVOID UmsContext;   
        PVOID UmsCompletionList;   
} UMS_CREATE_THREAD_ATTRIBUTES, *PUMS_CREATE_THREAD_ATTRIBUTES; 
//

//
// Disable Component Filter information
//

#define COMPONENT_KTM           0x01
#define COMPONENT_VALID_FLAGS   (COMPONENT_KTM)

typedef struct _COMPONENT_FILTER {
    DWORD ComponentFlags;
} COMPONENT_FILTER, *PCOMPONENT_FILTER;

//
//

//
// Page/memory priorities.
//

#define MEMORY_PRIORITY_LOWEST           0
#define MEMORY_PRIORITY_VERY_LOW         1
#define MEMORY_PRIORITY_LOW              2
#define MEMORY_PRIORITY_MEDIUM           3
#define MEMORY_PRIORITY_BELOW_NORMAL     4
#define MEMORY_PRIORITY_NORMAL           5

//
//

//
// Process dynamic exception handling continuation targets information.
//
// Information class - ProcessDynamicEHContinuationTargets.
//

//
// Dynamic exception handling continuation target should be added. If not set,
// the target is removed. Input flag.
//

#define DYNAMIC_EH_CONTINUATION_TARGET_ADD                               (0x00000001)

//
// Dynamic exception handling continuation target has been successfully
// processed. Used to report to the caller how much progress has been made.
// Output flag.
//

#define DYNAMIC_EH_CONTINUATION_TARGET_PROCESSED                         (0x00000002)

typedef struct _PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    ULONG_PTR TargetAddress;
    ULONG_PTR Flags;
} PROCESS_DYNAMIC_EH_CONTINUATION_TARGET, *PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET;

typedef struct _PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    WORD   NumberOfTargets;
    WORD   Reserved;
    DWORD Reserved2;
    PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET Targets;
} PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION, *PPROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION;

//
// Process dynamic enforced address ranges information, used for dynamic
// enforced CETCOMPAT ranges.
//
// Information class - ProcessDynamicEnforcedCetCompatibleRanges.
//

//
// Dynamic enforced address range should be added. If not set, the range is
// removed. Input flag.
//

#define DYNAMIC_ENFORCED_ADDRESS_RANGE_ADD                               (0x00000001)

//
// Dynamic enforced address range has been successfully processed. Used to
// report to the caller how much progress has been made. Output flag.
//

#define DYNAMIC_ENFORCED_ADDRESS_RANGE_PROCESSED                         (0x00000002)

typedef struct _PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    ULONG_PTR BaseAddress;
    SIZE_T Size;
    DWORD Flags;
} PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE, *PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE;

typedef struct _PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    WORD   NumberOfRanges;
    WORD   Reserved;
    DWORD Reserved2;
    PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE Ranges;
} PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION, *PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION;

//
//

typedef struct _QUOTA_LIMITS {
    SIZE_T PagedPoolLimit;
    SIZE_T NonPagedPoolLimit;
    SIZE_T MinimumWorkingSetSize;
    SIZE_T MaximumWorkingSetSize;
    SIZE_T PagefileLimit;
    LARGE_INTEGER TimeLimit;
} QUOTA_LIMITS, *PQUOTA_LIMITS;

#define QUOTA_LIMITS_HARDWS_MIN_ENABLE  0x00000001
#define QUOTA_LIMITS_HARDWS_MIN_DISABLE 0x00000002
#define QUOTA_LIMITS_HARDWS_MAX_ENABLE  0x00000004
#define QUOTA_LIMITS_HARDWS_MAX_DISABLE 0x00000008
#define QUOTA_LIMITS_USE_DEFAULT_LIMITS 0x00000010

typedef union _RATE_QUOTA_LIMIT {
    DWORD RateData;
    struct {
        DWORD RatePercent : 7;
        DWORD Reserved0   : 25;
    } DUMMYSTRUCTNAME;
} RATE_QUOTA_LIMIT, *PRATE_QUOTA_LIMIT;

typedef struct _QUOTA_LIMITS_EX {
    SIZE_T PagedPoolLimit;
    SIZE_T NonPagedPoolLimit;
    SIZE_T MinimumWorkingSetSize;
    SIZE_T MaximumWorkingSetSize;
    SIZE_T PagefileLimit;               // Limit expressed in pages
    LARGE_INTEGER TimeLimit;
    SIZE_T WorkingSetLimit;             // Limit expressed in pages
    SIZE_T Reserved2;
    SIZE_T Reserved3;
    SIZE_T Reserved4;
    DWORD  Flags;
    RATE_QUOTA_LIMIT CpuRateLimit;
} QUOTA_LIMITS_EX, *PQUOTA_LIMITS_EX;

//
// Process I/O Counters
//  NtQueryInformationProcess using ProcessIoCounters
//

typedef struct _IO_COUNTERS {
    ULONGLONG  ReadOperationCount;
    ULONGLONG  WriteOperationCount;
    ULONGLONG  OtherOperationCount;
    ULONGLONG ReadTransferCount;
    ULONGLONG WriteTransferCount;
    ULONGLONG OtherTransferCount;
} IO_COUNTERS;
typedef IO_COUNTERS *PIO_COUNTERS;

//
//

#define MAX_HW_COUNTERS 16
#define THREAD_PROFILING_FLAG_DISPATCH  0x00000001

typedef enum _HARDWARE_COUNTER_TYPE {
    PMCCounter,
    MaxHardwareCounterType
} HARDWARE_COUNTER_TYPE, *PHARDWARE_COUNTER_TYPE;

//
//

typedef enum _PROCESS_MITIGATION_POLICY {
    ProcessDEPPolicy,
    ProcessASLRPolicy,
    ProcessDynamicCodePolicy,
    ProcessStrictHandleCheckPolicy,
    ProcessSystemCallDisablePolicy,
    ProcessMitigationOptionsMask,
    ProcessExtensionPointDisablePolicy,
    ProcessControlFlowGuardPolicy,
    ProcessSignaturePolicy,
    ProcessFontDisablePolicy,
    ProcessImageLoadPolicy,
    ProcessSystemCallFilterPolicy,
    ProcessPayloadRestrictionPolicy,
    ProcessChildProcessPolicy,
    ProcessSideChannelIsolationPolicy,
    ProcessUserShadowStackPolicy,
    ProcessRedirectionTrustPolicy,
    ProcessUserPointerAuthPolicy,
	ProcessSEHOPPolicy,
    MaxProcessMitigationPolicy
} PROCESS_MITIGATION_POLICY, *PPROCESS_MITIGATION_POLICY;

//
// N.B.  High entropy mode is read only and can only be set at creation time
//       and not via the ProcessMitigationPolicy APIs.
//

typedef struct _PROCESS_MITIGATION_ASLR_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnableBottomUpRandomization : 1;
            DWORD EnableForceRelocateImages : 1;
            DWORD EnableHighEntropy : 1;
            DWORD DisallowStrippedImages : 1;
            DWORD ReservedFlags : 28;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_ASLR_POLICY, *PPROCESS_MITIGATION_ASLR_POLICY;

typedef struct _PROCESS_MITIGATION_DEP_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD Enable : 1;
            DWORD DisableAtlThunkEmulation : 1;
            DWORD ReservedFlags : 30;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
    BOOLEAN Permanent;
} PROCESS_MITIGATION_DEP_POLICY, *PPROCESS_MITIGATION_DEP_POLICY;

typedef struct _PROCESS_MITIGATION_SEHOP_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnableSehop : 1;
            DWORD ReservedFlags : 31;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_SEHOP_POLICY, *PPROCESS_MITIGATION_SEHOP_POLICY;

typedef struct _PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD RaiseExceptionOnInvalidHandleReference : 1;
            DWORD HandleExceptionsPermanentlyEnabled : 1;
            DWORD ReservedFlags : 30;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY, *PPROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY;

typedef struct _PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD DisallowWin32kSystemCalls : 1;
            DWORD AuditDisallowWin32kSystemCalls : 1;
            DWORD DisallowFsctlSystemCalls : 1;
            DWORD AuditDisallowFsctlSystemCalls : 1;
            DWORD ReservedFlags : 28;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY, *PPROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY;

typedef struct _PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD DisableExtensionPoints : 1;
            DWORD ReservedFlags : 31;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY, *PPROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY;

typedef struct _PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD ProhibitDynamicCode : 1;
            DWORD AllowThreadOptOut : 1;
            DWORD AllowRemoteDowngrade : 1;
            DWORD AuditProhibitDynamicCode : 1;
            DWORD ReservedFlags : 28;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_DYNAMIC_CODE_POLICY, *PPROCESS_MITIGATION_DYNAMIC_CODE_POLICY;

typedef struct _PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnableControlFlowGuard : 1;
            DWORD EnableExportSuppression : 1;
            DWORD StrictMode : 1;
            DWORD EnableXfg : 1;
            DWORD EnableXfgAuditMode : 1;
            DWORD ReservedFlags : 27;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY, *PPROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY;

typedef struct _PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD MicrosoftSignedOnly : 1;
            DWORD StoreSignedOnly : 1;
            DWORD MitigationOptIn : 1;
            DWORD AuditMicrosoftSignedOnly : 1;
            DWORD AuditStoreSignedOnly : 1;
            DWORD ReservedFlags : 27;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY, *PPROCESS_MITIGATION_BINARY_SIGNATURE_POLICY;

typedef struct _PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD DisableNonSystemFonts     : 1;
            DWORD AuditNonSystemFontLoading : 1;
            DWORD ReservedFlags             : 30;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_FONT_DISABLE_POLICY, *PPROCESS_MITIGATION_FONT_DISABLE_POLICY;

typedef struct _PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD NoRemoteImages : 1;
            DWORD NoLowMandatoryLabelImages : 1;
            DWORD PreferSystem32Images : 1;
            DWORD AuditNoRemoteImages : 1;
            DWORD AuditNoLowMandatoryLabelImages : 1;
            DWORD ReservedFlags : 27;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_IMAGE_LOAD_POLICY, *PPROCESS_MITIGATION_IMAGE_LOAD_POLICY;

typedef struct _PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD FilterId: 4;
            DWORD ReservedFlags : 28;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY, *PPROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY;

typedef struct _PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnableExportAddressFilter     : 1;
            DWORD AuditExportAddressFilter      : 1;

            DWORD EnableExportAddressFilterPlus : 1;
            DWORD AuditExportAddressFilterPlus  : 1;

            DWORD EnableImportAddressFilter     : 1;
            DWORD AuditImportAddressFilter      : 1;

            DWORD EnableRopStackPivot           : 1;
            DWORD AuditRopStackPivot            : 1;

            DWORD EnableRopCallerCheck          : 1;
            DWORD AuditRopCallerCheck           : 1;

            DWORD EnableRopSimExec              : 1;
            DWORD AuditRopSimExec               : 1;

            DWORD ReservedFlags                 : 20;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY, *PPROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY;

typedef struct _PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD NoChildProcessCreation : 1;
            DWORD AuditNoChildProcessCreation : 1;
            DWORD AllowSecureProcessCreation : 1;
            DWORD ReservedFlags : 29;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_CHILD_PROCESS_POLICY, *PPROCESS_MITIGATION_CHILD_PROCESS_POLICY;

typedef struct _PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    union {
        DWORD Flags;
        struct {

            //
            // Prevent branch target pollution cross-SMT-thread in user mode.
            //

            DWORD SmtBranchTargetIsolation : 1;

            //
            // Isolate this process into a distinct security domain, even from
            // other processes running as the same security context.  This
            // prevents branch target injection cross-process (normally such
            // branch target injection is only inhibited across different
            // security contexts).
            //
            // Page combining is limited to processes within the same security
            // domain.  This flag thus also effectively limits the process to
            // only being able to combine internally to the process itself,
            // except for common pages (unless further restricted by the
            // DisablePageCombine policy).
            //

            DWORD IsolateSecurityDomain : 1;

            //
            // Disable all page combining for this process, even internally to
            // the process itself, except for common pages (zeroes or ones).
            //

            DWORD DisablePageCombine : 1;

            //
            // Memory Disambiguation Disable.
            //

            DWORD SpeculativeStoreBypassDisable : 1;

            //
            // Prevent this process' threads from being scheduled on the same
            // core as threads outside its security domain.
            //

            DWORD RestrictCoreSharing : 1;

            DWORD ReservedFlags : 27;

        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY, *PPROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY;

typedef struct _PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnableUserShadowStack : 1;
            DWORD AuditUserShadowStack : 1;
            DWORD SetContextIpValidation : 1;
            DWORD AuditSetContextIpValidation : 1;
            DWORD EnableUserShadowStackStrictMode : 1;
            DWORD BlockNonCetBinaries : 1;
            DWORD BlockNonCetBinariesNonEhcont : 1;
            DWORD AuditBlockNonCetBinaries : 1;
            DWORD CetDynamicApisOutOfProcOnly : 1;
            DWORD SetContextIpValidationRelaxedMode : 1;
            DWORD ReservedFlags : 22;

        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY, *PPROCESS_MITIGATION_USER_SHADOW_STACK_POLICY;

typedef struct _PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnablePointerAuthUserIp : 1;
            DWORD ReservedFlags : 31;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY, *PPROCESS_MITIGATION_USER_POINTER_AUTH_POLICY;

typedef struct _PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    union {
        DWORD Flags;
        struct {
            DWORD EnforceRedirectionTrust : 1;
            DWORD AuditRedirectionTrust : 1;
            DWORD ReservedFlags : 30;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY, *PPROCESS_MITIGATION_REDIRECTION_TRUST_POLICY;

//
// Structure used for Network I/O accounting information.
//

typedef struct _PROCESS_NETWORK_COUNTERS {
    DWORD64 BytesIn;
    DWORD64 BytesOut;
} PROCESS_NETWORK_COUNTERS, *PPROCESS_NETWORK_COUNTERS;

//
//

typedef struct _JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    LARGE_INTEGER TotalUserTime;
    LARGE_INTEGER TotalKernelTime;
    LARGE_INTEGER ThisPeriodTotalUserTime;
    LARGE_INTEGER ThisPeriodTotalKernelTime;
    DWORD TotalPageFaultCount;
    DWORD TotalProcesses;
    DWORD ActiveProcesses;
    DWORD TotalTerminatedProcesses;
} JOBOBJECT_BASIC_ACCOUNTING_INFORMATION, *PJOBOBJECT_BASIC_ACCOUNTING_INFORMATION;

//@[comment("MVI_tracked")]
typedef struct _JOBOBJECT_BASIC_LIMIT_INFORMATION {
    LARGE_INTEGER PerProcessUserTimeLimit;
    LARGE_INTEGER PerJobUserTimeLimit;
    DWORD LimitFlags;
    SIZE_T MinimumWorkingSetSize;
    SIZE_T MaximumWorkingSetSize;
    DWORD ActiveProcessLimit;
    ULONG_PTR Affinity;
    DWORD PriorityClass;
    DWORD SchedulingClass;
} JOBOBJECT_BASIC_LIMIT_INFORMATION, *PJOBOBJECT_BASIC_LIMIT_INFORMATION;

//@[comment("MVI_tracked")]
typedef struct _JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    JOBOBJECT_BASIC_LIMIT_INFORMATION BasicLimitInformation;
    IO_COUNTERS IoInfo;
    SIZE_T ProcessMemoryLimit;
    SIZE_T JobMemoryLimit;
    SIZE_T PeakProcessMemoryUsed;
    SIZE_T PeakJobMemoryUsed;
} JOBOBJECT_EXTENDED_LIMIT_INFORMATION, *PJOBOBJECT_EXTENDED_LIMIT_INFORMATION;

//
//

//@[comment("MVI_tracked")]
typedef struct _JOBOBJECT_BASIC_PROCESS_ID_LIST {
    DWORD NumberOfAssignedProcesses;
    DWORD NumberOfProcessIdsInList;
    ULONG_PTR ProcessIdList[1];
} JOBOBJECT_BASIC_PROCESS_ID_LIST, *PJOBOBJECT_BASIC_PROCESS_ID_LIST;

typedef struct _JOBOBJECT_BASIC_UI_RESTRICTIONS {
    DWORD UIRestrictionsClass;
} JOBOBJECT_BASIC_UI_RESTRICTIONS, *PJOBOBJECT_BASIC_UI_RESTRICTIONS;

//
// N.B. The JOBOBJECT_SECURITY_LIMIT_INFORMATION information class is no longer supported.
//

typedef struct _JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    DWORD SecurityLimitFlags ;
    HANDLE JobToken ;
    PTOKEN_GROUPS SidsToDisable ;
    PTOKEN_PRIVILEGES PrivilegesToDelete ;
    PTOKEN_GROUPS RestrictedSids ;
} JOBOBJECT_SECURITY_LIMIT_INFORMATION, *PJOBOBJECT_SECURITY_LIMIT_INFORMATION ;

typedef struct _JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    DWORD EndOfJobTimeAction;
} JOBOBJECT_END_OF_JOB_TIME_INFORMATION, *PJOBOBJECT_END_OF_JOB_TIME_INFORMATION;

typedef struct _JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    PVOID CompletionKey;
    HANDLE CompletionPort;
} JOBOBJECT_ASSOCIATE_COMPLETION_PORT, *PJOBOBJECT_ASSOCIATE_COMPLETION_PORT;

typedef struct _JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    JOBOBJECT_BASIC_ACCOUNTING_INFORMATION BasicInfo;
    IO_COUNTERS IoInfo;
} JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION, *PJOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION;

typedef struct _JOBOBJECT_NETWORK_ACCOUNTING_INFORMATION {
    DWORD64 DataBytesIn;
    DWORD64 DataBytesOut;
} JOBOBJECT_NETWORK_ACCOUNTING_INFORMATION;

typedef struct _JOBOBJECT_JOBSET_INFORMATION {
    DWORD MemberLevel;
} JOBOBJECT_JOBSET_INFORMATION, *PJOBOBJECT_JOBSET_INFORMATION;

typedef enum _JOBOBJECT_RATE_CONTROL_TOLERANCE {
    ToleranceLow = 1,
    ToleranceMedium,
    ToleranceHigh
} JOBOBJECT_RATE_CONTROL_TOLERANCE, *PJOBOBJECT_RATE_CONTROL_TOLERANCE;

typedef enum _JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    ToleranceIntervalShort = 1,
    ToleranceIntervalMedium,
    ToleranceIntervalLong
} JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
  *PJOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL;

typedef struct _JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    DWORD64 IoReadBytesLimit;
    DWORD64 IoWriteBytesLimit;
    LARGE_INTEGER PerJobUserTimeLimit;
    DWORD64 JobMemoryLimit;
    JOBOBJECT_RATE_CONTROL_TOLERANCE RateControlTolerance;
    JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL RateControlToleranceInterval;
    DWORD LimitFlags;
} JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION, *PJOBOBJECT_NOTIFICATION_LIMIT_INFORMATION;

typedef struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    DWORD64 IoReadBytesLimit;
    DWORD64 IoWriteBytesLimit;
    LARGE_INTEGER PerJobUserTimeLimit;
    union {
        DWORD64 JobHighMemoryLimit;
        DWORD64 JobMemoryLimit;
    } DUMMYUNIONNAME;

    union {
        JOBOBJECT_RATE_CONTROL_TOLERANCE RateControlTolerance;
        JOBOBJECT_RATE_CONTROL_TOLERANCE CpuRateControlTolerance;
    } DUMMYUNIONNAME2;

    union {
        JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL RateControlToleranceInterval;
        JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL
            CpuRateControlToleranceInterval;
    } DUMMYUNIONNAME3;

    DWORD LimitFlags;
    JOBOBJECT_RATE_CONTROL_TOLERANCE IoRateControlTolerance;
    DWORD64 JobLowMemoryLimit;
    JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL IoRateControlToleranceInterval;
    JOBOBJECT_RATE_CONTROL_TOLERANCE NetRateControlTolerance;
    JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL NetRateControlToleranceInterval;
} JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2;

//
//

typedef struct _JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    DWORD LimitFlags;
    DWORD ViolationLimitFlags;
    DWORD64 IoReadBytes;
    DWORD64 IoReadBytesLimit;
    DWORD64 IoWriteBytes;
    DWORD64 IoWriteBytesLimit;
    LARGE_INTEGER PerJobUserTime;
    LARGE_INTEGER PerJobUserTimeLimit;
    DWORD64 JobMemory;
    DWORD64 JobMemoryLimit;
    JOBOBJECT_RATE_CONTROL_TOLERANCE RateControlTolerance;
    JOBOBJECT_RATE_CONTROL_TOLERANCE RateControlToleranceLimit;
} JOBOBJECT_LIMIT_VIOLATION_INFORMATION, *PJOBOBJECT_LIMIT_VIOLATION_INFORMATION;

typedef struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    DWORD LimitFlags;
    DWORD ViolationLimitFlags;
    DWORD64 IoReadBytes;
    DWORD64 IoReadBytesLimit;
    DWORD64 IoWriteBytes;
    DWORD64 IoWriteBytesLimit;
    LARGE_INTEGER PerJobUserTime;
    LARGE_INTEGER PerJobUserTimeLimit;
    DWORD64 JobMemory;
    union {
        DWORD64 JobHighMemoryLimit;
        DWORD64 JobMemoryLimit;
    } DUMMYUNIONNAME;

    union {
        JOBOBJECT_RATE_CONTROL_TOLERANCE RateControlTolerance;
        JOBOBJECT_RATE_CONTROL_TOLERANCE CpuRateControlTolerance;
    } DUMMYUNIONNAME2;

    union {
        JOBOBJECT_RATE_CONTROL_TOLERANCE RateControlToleranceLimit;
        JOBOBJECT_RATE_CONTROL_TOLERANCE CpuRateControlToleranceLimit;
    } DUMMYUNIONNAME3;

    DWORD64 JobLowMemoryLimit;
    JOBOBJECT_RATE_CONTROL_TOLERANCE IoRateControlTolerance;
    JOBOBJECT_RATE_CONTROL_TOLERANCE IoRateControlToleranceLimit;
    JOBOBJECT_RATE_CONTROL_TOLERANCE NetRateControlTolerance;
    JOBOBJECT_RATE_CONTROL_TOLERANCE NetRateControlToleranceLimit;
} JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2;

//
//

typedef struct _JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    DWORD ControlFlags;
    union {
        DWORD CpuRate;
        DWORD Weight;
        struct {
            WORD   MinRate;
            WORD   MaxRate;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} JOBOBJECT_CPU_RATE_CONTROL_INFORMATION, *PJOBOBJECT_CPU_RATE_CONTROL_INFORMATION;

//
// Control flags for network rate control.
//

typedef enum JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    JOB_OBJECT_NET_RATE_CONTROL_ENABLE = 0x1,
    JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH = 0x2,
    JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG = 0x4,
    JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS = 0x7
} JOB_OBJECT_NET_RATE_CONTROL_FLAGS;

#if !defined(SORTPP_PASS) && !defined(MIDL_PASS) && !defined(RC_INVOKED)

DEFINE_ENUM_FLAG_OPERATORS(JOB_OBJECT_NET_RATE_CONTROL_FLAGS)
C_ASSERT(JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS ==
             (JOB_OBJECT_NET_RATE_CONTROL_ENABLE +
              JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH +
              JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG));

#endif

#define JOB_OBJECT_NET_RATE_CONTROL_MAX_DSCP_TAG 64

typedef struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    DWORD64 MaxBandwidth;
    JOB_OBJECT_NET_RATE_CONTROL_FLAGS ControlFlags;
    BYTE  DscpTag;
} JOBOBJECT_NET_RATE_CONTROL_INFORMATION;

//
//

//
// Control flags for IO rate control.
//

typedef enum JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    JOB_OBJECT_IO_RATE_CONTROL_ENABLE = 0x1,
    JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME = 0x2,
    JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL = 0x4,
    JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP = 0x8,
    JOB_OBJECT_IO_RATE_CONTROL_VALID_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_ENABLE |
                                             JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME |
                                             JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL |
                                             JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP
} JOB_OBJECT_IO_RATE_CONTROL_FLAGS;

#if !defined(SORTPP_PASS) && !defined(MIDL_PASS) && !defined(RC_INVOKED)

DEFINE_ENUM_FLAG_OPERATORS(JOB_OBJECT_IO_RATE_CONTROL_FLAGS)

#endif

typedef struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    LONG64 MaxIops;
    LONG64 MaxBandwidth;
    LONG64 ReservationIops;
    PWSTR VolumeName;
    DWORD BaseIoSize;
    JOB_OBJECT_IO_RATE_CONTROL_FLAGS ControlFlags;
    WORD   VolumeNameLength;
} JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE;

typedef JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE
    JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1;

typedef struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    LONG64 MaxIops;
    LONG64 MaxBandwidth;
    LONG64 ReservationIops;
    PWSTR VolumeName;
    DWORD BaseIoSize;
    JOB_OBJECT_IO_RATE_CONTROL_FLAGS ControlFlags;
    WORD   VolumeNameLength;
    LONG64 CriticalReservationIops;
    LONG64 ReservationBandwidth;
    LONG64 CriticalReservationBandwidth;
    LONG64 MaxTimePercent;
    LONG64 ReservationTimePercent;
    LONG64 CriticalReservationTimePercent;
} JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2;

typedef struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    LONG64 MaxIops;
    LONG64 MaxBandwidth;
    LONG64 ReservationIops;
    PWSTR VolumeName;
    DWORD BaseIoSize;
    JOB_OBJECT_IO_RATE_CONTROL_FLAGS ControlFlags;
    WORD   VolumeNameLength;
    LONG64 CriticalReservationIops;
    LONG64 ReservationBandwidth;
    LONG64 CriticalReservationBandwidth;
    LONG64 MaxTimePercent;
    LONG64 ReservationTimePercent;
    LONG64 CriticalReservationTimePercent;
    LONG64 SoftMaxIops;
    LONG64 SoftMaxBandwidth;
    LONG64 SoftMaxTimePercent;
    LONG64 LimitExcessNotifyIops;
    LONG64 LimitExcessNotifyBandwidth;
    LONG64 LimitExcessNotifyTimePercent;
} JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3;

//
//

typedef enum JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    JOBOBJECT_IO_ATTRIBUTION_CONTROL_ENABLE = 0x1,
    JOBOBJECT_IO_ATTRIBUTION_CONTROL_DISABLE = 0x2,
    JOBOBJECT_IO_ATTRIBUTION_CONTROL_VALID_FLAGS = 0x3
} JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS;

typedef struct _JOBOBJECT_IO_ATTRIBUTION_STATS {

    ULONG_PTR IoCount;
    ULONGLONG TotalNonOverlappedQueueTime;
    ULONGLONG TotalNonOverlappedServiceTime;
    ULONGLONG TotalSize;

} JOBOBJECT_IO_ATTRIBUTION_STATS, *PJOBOBJECT_IO_ATTRIBUTION_STATS;

typedef struct _JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    DWORD ControlFlags;

    JOBOBJECT_IO_ATTRIBUTION_STATS ReadStats;
    JOBOBJECT_IO_ATTRIBUTION_STATS WriteStats;

} JOBOBJECT_IO_ATTRIBUTION_INFORMATION, *PJOBOBJECT_IO_ATTRIBUTION_INFORMATION;

//
//

#define JOB_OBJECT_TERMINATE_AT_END_OF_JOB  0
#define JOB_OBJECT_POST_AT_END_OF_JOB       1

//
// Completion Port Messages for job objects
//
// These values are returned via the lpNumberOfBytesTransferred parameter
//

#define JOB_OBJECT_MSG_END_OF_JOB_TIME          1
#define JOB_OBJECT_MSG_END_OF_PROCESS_TIME      2
#define JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT     3
#define JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO      4
#define JOB_OBJECT_MSG_NEW_PROCESS              6
#define JOB_OBJECT_MSG_EXIT_PROCESS             7
#define JOB_OBJECT_MSG_ABNORMAL_EXIT_PROCESS    8
#define JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT     9
#define JOB_OBJECT_MSG_JOB_MEMORY_LIMIT         10
#define JOB_OBJECT_MSG_NOTIFICATION_LIMIT       11
#define JOB_OBJECT_MSG_JOB_CYCLE_TIME_LIMIT     12
#define JOB_OBJECT_MSG_SILO_TERMINATED          13

//
// Define the valid notification filter values.
//

#define JOB_OBJECT_MSG_MINIMUM 1
#define JOB_OBJECT_MSG_MAXIMUM 13

#define JOB_OBJECT_VALID_COMPLETION_FILTER \
    (((1UL << (JOB_OBJECT_MSG_MAXIMUM + 1)) - 1) - \
     ((1UL << JOB_OBJECT_MSG_MINIMUM) - 1))

//
// Basic Limits
//
#define JOB_OBJECT_LIMIT_WORKINGSET                 0x00000001
#define JOB_OBJECT_LIMIT_PROCESS_TIME               0x00000002
#define JOB_OBJECT_LIMIT_JOB_TIME                   0x00000004
#define JOB_OBJECT_LIMIT_ACTIVE_PROCESS             0x00000008
#define JOB_OBJECT_LIMIT_AFFINITY                   0x00000010
#define JOB_OBJECT_LIMIT_PRIORITY_CLASS             0x00000020
#define JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME          0x00000040
#define JOB_OBJECT_LIMIT_SCHEDULING_CLASS           0x00000080

//
// Extended Limits
//
#define JOB_OBJECT_LIMIT_PROCESS_MEMORY             0x00000100
#define JOB_OBJECT_LIMIT_JOB_MEMORY                 0x00000200
#define JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH            JOB_OBJECT_LIMIT_JOB_MEMORY
#define JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION 0x00000400
#define JOB_OBJECT_LIMIT_BREAKAWAY_OK               0x00000800
#define JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK        0x00001000
#define JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE          0x00002000
#define JOB_OBJECT_LIMIT_SUBSET_AFFINITY            0x00004000
#define JOB_OBJECT_LIMIT_JOB_MEMORY_LOW             0x00008000

//
// Notification Limits
//

#define JOB_OBJECT_LIMIT_JOB_READ_BYTES             0x00010000
#define JOB_OBJECT_LIMIT_JOB_WRITE_BYTES            0x00020000
#define JOB_OBJECT_LIMIT_RATE_CONTROL               0x00040000
#define JOB_OBJECT_LIMIT_CPU_RATE_CONTROL           JOB_OBJECT_LIMIT_RATE_CONTROL
#define JOB_OBJECT_LIMIT_IO_RATE_CONTROL            0x00080000
#define JOB_OBJECT_LIMIT_NET_RATE_CONTROL           0x00100000

//
//

//
// Valid Job Object Limits
//

#define JOB_OBJECT_LIMIT_VALID_FLAGS                 0x0007ffff
#define JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS           0x000000ff
#define JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS        0x00007fff
#define JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS \
    (JOB_OBJECT_LIMIT_JOB_READ_BYTES | \
     JOB_OBJECT_LIMIT_JOB_WRITE_BYTES | \
     JOB_OBJECT_LIMIT_JOB_TIME | \
     JOB_OBJECT_LIMIT_JOB_MEMORY_LOW | \
     JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH | \
     JOB_OBJECT_LIMIT_CPU_RATE_CONTROL | \
     JOB_OBJECT_LIMIT_IO_RATE_CONTROL | \
     JOB_OBJECT_LIMIT_NET_RATE_CONTROL)

//
//

//
// UI restrictions for jobs
//

#define JOB_OBJECT_UILIMIT_NONE             0x00000000

#define JOB_OBJECT_UILIMIT_HANDLES          0x00000001
#define JOB_OBJECT_UILIMIT_READCLIPBOARD    0x00000002
#define JOB_OBJECT_UILIMIT_WRITECLIPBOARD   0x00000004
#define JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS 0x00000008
#define JOB_OBJECT_UILIMIT_DISPLAYSETTINGS  0x00000010
#define JOB_OBJECT_UILIMIT_GLOBALATOMS      0x00000020
#define JOB_OBJECT_UILIMIT_DESKTOP          0x00000040
#define JOB_OBJECT_UILIMIT_EXITWINDOWS      0x00000080
#define JOB_OBJECT_UILIMIT_IME              0x00000100

// TODO: New Additions
#define JOB_OBJECT_UILIMIT_INJECTION        0x00000200


#define JOB_OBJECT_UILIMIT_ALL              0x000003FF

#define JOB_OBJECT_UI_VALID_FLAGS           0x000003FF

#define JOB_OBJECT_SECURITY_NO_ADMIN            0x00000001
#define JOB_OBJECT_SECURITY_RESTRICTED_TOKEN    0x00000002
#define JOB_OBJECT_SECURITY_ONLY_TOKEN          0x00000004
#define JOB_OBJECT_SECURITY_FILTER_TOKENS       0x00000008

#define JOB_OBJECT_SECURITY_VALID_FLAGS         0x0000000f

//
// Control flags for CPU rate control.
//

#define JOB_OBJECT_CPU_RATE_CONTROL_ENABLE              0x1
#define JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED        0x2
#define JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP            0x4
#define JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY              0x8
#define JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE        0x10
#define JOB_OBJECT_CPU_RATE_CONTROL_PER_PROCESSOR_CAPS  0x20
#define JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS         0x3f

//
//

//@[comment("MVI_tracked")]
typedef enum _JOBOBJECTINFOCLASS {
    JobObjectBasicAccountingInformation = 1,
    JobObjectBasicLimitInformation,
    JobObjectBasicProcessIdList,
    JobObjectBasicUIRestrictions,
    JobObjectSecurityLimitInformation,  // deprecated
    JobObjectEndOfJobTimeInformation,
    JobObjectAssociateCompletionPortInformation,
    JobObjectBasicAndIoAccountingInformation,
    JobObjectExtendedLimitInformation,
    JobObjectJobSetInformation,
    JobObjectGroupInformation,
    JobObjectNotificationLimitInformation,
    JobObjectLimitViolationInformation,
    JobObjectGroupInformationEx,
    JobObjectCpuRateControlInformation,
    JobObjectCompletionFilter,
    JobObjectCompletionCounter,

//
//

    JobObjectReserved1Information = 18,
    JobObjectReserved2Information,
    JobObjectReserved3Information,
    JobObjectReserved4Information,
    JobObjectReserved5Information,
    JobObjectReserved6Information,
    JobObjectReserved7Information,
    JobObjectReserved8Information,
    JobObjectReserved9Information,
    JobObjectReserved10Information,
    JobObjectReserved11Information,
    JobObjectReserved12Information,
    JobObjectReserved13Information,
    JobObjectReserved14Information = 31,
    JobObjectNetRateControlInformation,
    JobObjectNotificationLimitInformation2,
    JobObjectLimitViolationInformation2,
    JobObjectCreateSilo,
    JobObjectSiloBasicInformation,
    JobObjectReserved15Information = 37,
    JobObjectReserved16Information = 38,
    JobObjectReserved17Information = 39,
    JobObjectReserved18Information = 40,
    JobObjectReserved19Information = 41,
    JobObjectReserved20Information = 42,
    JobObjectReserved21Information = 43,
    JobObjectReserved22Information = 44,
    JobObjectReserved23Information = 45,
    JobObjectReserved24Information = 46,
    JobObjectReserved25Information = 47,
    JobObjectReserved26Information = 48,
    JobObjectReserved27Information = 49,
    JobObjectReserved28Information = 50,
    JobObjectNetworkAccountingInformation,
    MaxJobObjectInfoClass
} JOBOBJECTINFOCLASS;

//
//

typedef struct _SILOOBJECT_BASIC_INFORMATION {
    DWORD SiloId;
    DWORD SiloParentId;
    DWORD NumberOfProcesses;
    BOOLEAN IsInServerSilo;
    BYTE  Reserved[3];
} SILOOBJECT_BASIC_INFORMATION, *PSILOOBJECT_BASIC_INFORMATION;

typedef enum _SERVERSILO_STATE {
    SERVERSILO_INITING = 0,
    SERVERSILO_STARTED,
    SERVERSILO_SHUTTING_DOWN,
    SERVERSILO_TERMINATING,
    SERVERSILO_TERMINATED,
} SERVERSILO_STATE, *PSERVERSILO_STATE;

typedef struct _SERVERSILO_BASIC_INFORMATION {
    DWORD ServiceSessionId;
    SERVERSILO_STATE State;
    DWORD    ExitStatus;
    BOOLEAN Reserved;
    PVOID ApiSetSchema;
    PVOID HostApiSetSchema;
    DWORD ContainerBuildNumber;
    DWORD HostBuildNumber;
} SERVERSILO_BASIC_INFORMATION, *PSERVERSILO_BASIC_INFORMATION;

typedef struct _SERVERSILO_DIAGNOSTIC_INFORMATION {
    GUID ReportId;
    DWORD    ExitStatus;
    WCHAR CriticalProcessName[15];
} SERVERSILO_DIAGNOSTIC_INFORMATION, *PSERVERSILO_DIAGNOSTIC_INFORMATION;

//
// begin_wdm

//
// Partition Specific Access Rights.
//

#define MEMORY_PARTITION_QUERY_ACCESS  0x0001
#define MEMORY_PARTITION_MODIFY_ACCESS 0x0002

#define MEMORY_PARTITION_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED |      \
                                     SYNCHRONIZE |                   \
                                     MEMORY_PARTITION_QUERY_ACCESS | \
                                     MEMORY_PARTITION_MODIFY_ACCESS)

//

typedef enum _FIRMWARE_TYPE {
    FirmwareTypeUnknown,
    FirmwareTypeBios,
    FirmwareTypeUefi,
    FirmwareTypeMax
} FIRMWARE_TYPE, *PFIRMWARE_TYPE;

//
#define EVENT_MODIFY_STATE      0x0002  
#define EVENT_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED|SYNCHRONIZE|0x3) 
//

//
// Mutant Specific Access Rights
//
#define MUTANT_QUERY_STATE      0x0001

#define MUTANT_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED|SYNCHRONIZE|\
                          MUTANT_QUERY_STATE)

//
#define SEMAPHORE_MODIFY_STATE      0x0002  
#define SEMAPHORE_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED|SYNCHRONIZE|0x3) 
//

//
// Timer Specific Access Rights.
//

#define TIMER_QUERY_STATE       0x0001
#define TIMER_MODIFY_STATE      0x0002

#define TIMER_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED|SYNCHRONIZE|\
                          TIMER_QUERY_STATE|TIMER_MODIFY_STATE)

//
//

#define TIME_ZONE_ID_UNKNOWN  0
#define TIME_ZONE_ID_STANDARD 1
#define TIME_ZONE_ID_DAYLIGHT 2

//
//

typedef enum _LOGICAL_PROCESSOR_RELATIONSHIP {
    RelationProcessorCore,
    RelationNumaNode,
    RelationCache,
    RelationProcessorPackage,
    RelationGroup,
    RelationProcessorDie,
    RelationNumaNodeEx,
    RelationProcessorModule,
    RelationAll = 0xffff
} LOGICAL_PROCESSOR_RELATIONSHIP;

#define LTP_PC_SMT 0x1

//
//

typedef enum _PROCESSOR_CACHE_TYPE {
    CacheUnified,
    CacheInstruction,
    CacheData,
    CacheTrace,
    CacheUnknown
} PROCESSOR_CACHE_TYPE, *PPROCESSOR_CACHE_TYPE;

//
//

#define CACHE_FULLY_ASSOCIATIVE 0xFF

typedef struct _CACHE_DESCRIPTOR {
    BYTE   Level;
    BYTE   Associativity;
    WORD   LineSize;
    DWORD  Size;
    PROCESSOR_CACHE_TYPE Type;
} CACHE_DESCRIPTOR, *PCACHE_DESCRIPTOR;

typedef struct _SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    ULONG_PTR   ProcessorMask;
    LOGICAL_PROCESSOR_RELATIONSHIP Relationship;
    union {
        struct {
            BYTE  Flags;
        } ProcessorCore;
        struct {
            DWORD NodeNumber;
        } NumaNode;
        CACHE_DESCRIPTOR Cache;
        ULONGLONG  Reserved[2];
    } DUMMYUNIONNAME;
} SYSTEM_LOGICAL_PROCESSOR_INFORMATION, *PSYSTEM_LOGICAL_PROCESSOR_INFORMATION;

typedef struct _PROCESSOR_RELATIONSHIP {
    BYTE  Flags;
    BYTE  EfficiencyClass;
    BYTE  Reserved[20];
    WORD   GroupCount;
    _Field_size_(GroupCount) GROUP_AFFINITY GroupMask[ANYSIZE_ARRAY];
} PROCESSOR_RELATIONSHIP, *PPROCESSOR_RELATIONSHIP;

typedef struct _NUMA_NODE_RELATIONSHIP {
    DWORD NodeNumber;
    BYTE  Reserved[18];
    WORD   GroupCount;
    union {
        GROUP_AFFINITY GroupMask;
        _Field_size_(GroupCount)
        GROUP_AFFINITY GroupMasks[ANYSIZE_ARRAY];
    } DUMMYUNIONNAME;
} NUMA_NODE_RELATIONSHIP, *PNUMA_NODE_RELATIONSHIP;

typedef struct _CACHE_RELATIONSHIP {
    BYTE  Level;
    BYTE  Associativity;
    WORD   LineSize;
    DWORD CacheSize;
    PROCESSOR_CACHE_TYPE Type;
    BYTE  Reserved[18];
    WORD   GroupCount;
    union {
        GROUP_AFFINITY GroupMask;
        _Field_size_(GroupCount)
        GROUP_AFFINITY GroupMasks[ANYSIZE_ARRAY];
    } DUMMYUNIONNAME;
} CACHE_RELATIONSHIP, *PCACHE_RELATIONSHIP;

typedef struct _PROCESSOR_GROUP_INFO {
    BYTE  MaximumProcessorCount;
    BYTE  ActiveProcessorCount;
    BYTE  Reserved[38];
    KAFFINITY ActiveProcessorMask;
} PROCESSOR_GROUP_INFO, *PPROCESSOR_GROUP_INFO;

typedef struct _GROUP_RELATIONSHIP {
    WORD   MaximumGroupCount;
    WORD   ActiveGroupCount;
    BYTE  Reserved[20];
    _Field_size_(ActiveGroupCount) PROCESSOR_GROUP_INFO GroupInfo[ANYSIZE_ARRAY];
} GROUP_RELATIONSHIP, *PGROUP_RELATIONSHIP;

_Struct_size_bytes_(Size) struct _SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    LOGICAL_PROCESSOR_RELATIONSHIP Relationship;
    DWORD Size;
    union {
        PROCESSOR_RELATIONSHIP Processor;
        NUMA_NODE_RELATIONSHIP NumaNode;
        CACHE_RELATIONSHIP Cache;
        GROUP_RELATIONSHIP Group;
    } DUMMYUNIONNAME;
};

typedef struct _SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, *PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX;

typedef enum _CPU_SET_INFORMATION_TYPE {
    CpuSetInformation
} CPU_SET_INFORMATION_TYPE, *PCPU_SET_INFORMATION_TYPE;

_Struct_size_bytes_(Size) struct _SYSTEM_CPU_SET_INFORMATION {
    DWORD Size;
    CPU_SET_INFORMATION_TYPE Type;
    union {
        struct {
            DWORD Id;
            WORD   Group;
            BYTE  LogicalProcessorIndex;
            BYTE  CoreIndex;
            BYTE  LastLevelCacheIndex;
            BYTE  NumaNodeIndex;
            BYTE  EfficiencyClass;
            union {

#define SYSTEM_CPU_SET_INFORMATION_PARKED 0x1
#define SYSTEM_CPU_SET_INFORMATION_ALLOCATED 0x2
#define SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS 0x4
#define SYSTEM_CPU_SET_INFORMATION_REALTIME 0x8

                BYTE  AllFlags;
                struct {
                    BYTE  Parked : 1;
                    BYTE  Allocated : 1;
                    BYTE  AllocatedToTargetProcess : 1;
                    BYTE  RealTime : 1;
                    BYTE  ReservedFlags : 4;
                } DUMMYSTRUCTNAME;
            } DUMMYUNIONNAME2;

            union {
                DWORD Reserved;
                BYTE  SchedulingClass;
            };

            DWORD64 AllocationTag;
        } CpuSet;
    } DUMMYUNIONNAME;
};

typedef struct _SYSTEM_CPU_SET_INFORMATION SYSTEM_CPU_SET_INFORMATION, *PSYSTEM_CPU_SET_INFORMATION;

//
//

typedef struct _SYSTEM_POOL_ZEROING_INFORMATION {
    BOOLEAN PoolZeroingSupportPresent;
} SYSTEM_POOL_ZEROING_INFORMATION, *PSYSTEM_POOL_ZEROING_INFORMATION;

//
//

typedef struct _SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    DWORD64 CycleTime;
} SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION, *PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION;

typedef struct _SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    DWORD Machine : 16;
    DWORD KernelMode : 1;
    DWORD UserMode : 1;
    DWORD Native : 1;
    DWORD Process : 1;
    DWORD WoW64Container : 1;
    DWORD ReservedZero0 : 11;
} SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION;

#define PROCESSOR_INTEL_386     386
#define PROCESSOR_INTEL_486     486
#define PROCESSOR_INTEL_PENTIUM 586
#define PROCESSOR_INTEL_IA64    2200
#define PROCESSOR_AMD_X8664     8664
#define PROCESSOR_MIPS_R4000    4000    // incl R4101 & R3910 for Windows CE
#define PROCESSOR_ALPHA_21064   21064
#define PROCESSOR_PPC_601       601
#define PROCESSOR_PPC_603       603
#define PROCESSOR_PPC_604       604
#define PROCESSOR_PPC_620       620
#define PROCESSOR_HITACHI_SH3   10003   // Windows CE
#define PROCESSOR_HITACHI_SH3E  10004   // Windows CE
#define PROCESSOR_HITACHI_SH4   10005   // Windows CE
#define PROCESSOR_MOTOROLA_821  821     // Windows CE
#define PROCESSOR_SHx_SH3       103     // Windows CE
#define PROCESSOR_SHx_SH4       104     // Windows CE
#define PROCESSOR_STRONGARM     2577    // Windows CE - 0xA11
#define PROCESSOR_ARM720        1824    // Windows CE - 0x720
#define PROCESSOR_ARM820        2080    // Windows CE - 0x820
#define PROCESSOR_ARM920        2336    // Windows CE - 0x920
#define PROCESSOR_ARM_7TDMI     70001   // Windows CE
#define PROCESSOR_OPTIL         0x494f  // MSIL

#define PROCESSOR_ARCHITECTURE_INTEL            0
#define PROCESSOR_ARCHITECTURE_MIPS             1
#define PROCESSOR_ARCHITECTURE_ALPHA            2
#define PROCESSOR_ARCHITECTURE_PPC              3
#define PROCESSOR_ARCHITECTURE_SHX              4
#define PROCESSOR_ARCHITECTURE_ARM              5
#define PROCESSOR_ARCHITECTURE_IA64             6
#define PROCESSOR_ARCHITECTURE_ALPHA64          7
#define PROCESSOR_ARCHITECTURE_MSIL             8
#define PROCESSOR_ARCHITECTURE_AMD64            9
#define PROCESSOR_ARCHITECTURE_IA32_ON_WIN64    10
#define PROCESSOR_ARCHITECTURE_NEUTRAL          11
#define PROCESSOR_ARCHITECTURE_ARM64            12
#define PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64   13
#define PROCESSOR_ARCHITECTURE_IA32_ON_ARM64    14

#define PROCESSOR_ARCHITECTURE_UNKNOWN 0xFFFF

//
#define PF_FLOATING_POINT_PRECISION_ERRATA           0   
#define PF_FLOATING_POINT_EMULATED                   1   
#define PF_COMPARE_EXCHANGE_DOUBLE                   2   
#define PF_MMX_INSTRUCTIONS_AVAILABLE                3   
#define PF_PPC_MOVEMEM_64BIT_OK                      4   
#define PF_ALPHA_BYTE_INSTRUCTIONS                   5   
#define PF_XMMI_INSTRUCTIONS_AVAILABLE               6   
#define PF_3DNOW_INSTRUCTIONS_AVAILABLE              7   
#define PF_RDTSC_INSTRUCTION_AVAILABLE               8   
#define PF_PAE_ENABLED                               9   
#define PF_XMMI64_INSTRUCTIONS_AVAILABLE            10   
#define PF_SSE_DAZ_MODE_AVAILABLE                   11   
#define PF_NX_ENABLED                               12   
#define PF_SSE3_INSTRUCTIONS_AVAILABLE              13   
#define PF_COMPARE_EXCHANGE128                      14   
#define PF_COMPARE64_EXCHANGE128                    15   
#define PF_CHANNELS_ENABLED                         16   
#define PF_XSAVE_ENABLED                            17   
#define PF_ARM_VFP_32_REGISTERS_AVAILABLE           18   
#define PF_ARM_NEON_INSTRUCTIONS_AVAILABLE          19   
#define PF_SECOND_LEVEL_ADDRESS_TRANSLATION         20   
#define PF_VIRT_FIRMWARE_ENABLED                    21   
#define PF_RDWRFSGSBASE_AVAILABLE                   22   
#define PF_FASTFAIL_AVAILABLE                       23   
#define PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE         24   
#define PF_ARM_64BIT_LOADSTORE_ATOMIC               25   
#define PF_ARM_EXTERNAL_CACHE_AVAILABLE             26   
#define PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE          27   
#define PF_RDRAND_INSTRUCTION_AVAILABLE             28   
#define PF_ARM_V8_INSTRUCTIONS_AVAILABLE            29   
#define PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE     30   
#define PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE      31   
#define PF_RDTSCP_INSTRUCTION_AVAILABLE             32   
#define PF_RDPID_INSTRUCTION_AVAILABLE              33   
#define PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE    34   
#define PF_MONITORX_INSTRUCTION_AVAILABLE           35   
#define PF_SSSE3_INSTRUCTIONS_AVAILABLE             36   
#define PF_SSE4_1_INSTRUCTIONS_AVAILABLE            37   
#define PF_SSE4_2_INSTRUCTIONS_AVAILABLE            38   
#define PF_AVX_INSTRUCTIONS_AVAILABLE               39   
#define PF_AVX2_INSTRUCTIONS_AVAILABLE              40   
#define PF_AVX512F_INSTRUCTIONS_AVAILABLE           41   
#define PF_ERMS_AVAILABLE                           42   
#define PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE        43   
#define PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE     44   
#define PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE     45   
#define PF_ARM_SVE_INSTRUCTIONS_AVAILABLE           46   
#define PF_ARM_SVE2_INSTRUCTIONS_AVAILABLE          47   
#define PF_ARM_SVE2_1_INSTRUCTIONS_AVAILABLE        48   
#define PF_ARM_SVE_AES_INSTRUCTIONS_AVAILABLE       49   
#define PF_ARM_SVE_PMULL128_INSTRUCTIONS_AVAILABLE  50   
#define PF_ARM_SVE_BITPERM_INSTRUCTIONS_AVAILABLE   51   
#define PF_ARM_SVE_BF16_INSTRUCTIONS_AVAILABLE      52   
#define PF_ARM_SVE_EBF16_INSTRUCTIONS_AVAILABLE     53   
#define PF_ARM_SVE_B16B16_INSTRUCTIONS_AVAILABLE    54   
#define PF_ARM_SVE_SHA3_INSTRUCTIONS_AVAILABLE      55   
#define PF_ARM_SVE_SM4_INSTRUCTIONS_AVAILABLE       56   
#define PF_ARM_SVE_I8MM_INSTRUCTIONS_AVAILABLE      57   
#define PF_ARM_SVE_F32MM_INSTRUCTIONS_AVAILABLE     58   
#define PF_ARM_SVE_F64MM_INSTRUCTIONS_AVAILABLE     59   
#define PF_BMI2_INSTRUCTIONS_AVAILABLE              60   
#define PF_MOVDIR64B_INSTRUCTION_AVAILABLE          61   
#define PF_ARM_LSE2_AVAILABLE                       62   
#define PF_RESERVED_FEATURE                         63   
#define PF_ARM_SHA3_INSTRUCTIONS_AVAILABLE          64   
#define PF_ARM_SHA512_INSTRUCTIONS_AVAILABLE        65   
#define PF_ARM_V82_I8MM_INSTRUCTIONS_AVAILABLE      66   
#define PF_ARM_V82_FP16_INSTRUCTIONS_AVAILABLE      67   
#define PF_ARM_V86_BF16_INSTRUCTIONS_AVAILABLE      68   
#define PF_ARM_V86_EBF16_INSTRUCTIONS_AVAILABLE     69   
#define PF_ARM_SME_INSTRUCTIONS_AVAILABLE           70   
#define PF_ARM_SME2_INSTRUCTIONS_AVAILABLE          71   
#define PF_ARM_SME2_1_INSTRUCTIONS_AVAILABLE        72   
#define PF_ARM_SME2_2_INSTRUCTIONS_AVAILABLE        73   
#define PF_ARM_SME_AES_INSTRUCTIONS_AVAILABLE       74   
#define PF_ARM_SME_SBITPERM_INSTRUCTIONS_AVAILABLE  75   
#define PF_ARM_SME_SF8MM4_INSTRUCTIONS_AVAILABLE    76   
#define PF_ARM_SME_SF8MM8_INSTRUCTIONS_AVAILABLE    77   
#define PF_ARM_SME_SF8DP2_INSTRUCTIONS_AVAILABLE    78   
#define PF_ARM_SME_SF8DP4_INSTRUCTIONS_AVAILABLE    79   
#define PF_ARM_SME_SF8FMA_INSTRUCTIONS_AVAILABLE    80   
#define PF_ARM_SME_F8F32_INSTRUCTIONS_AVAILABLE     81   
#define PF_ARM_SME_F8F16_INSTRUCTIONS_AVAILABLE     82   
#define PF_ARM_SME_F16F16_INSTRUCTIONS_AVAILABLE    83   
#define PF_ARM_SME_B16B16_INSTRUCTIONS_AVAILABLE    84   
#define PF_ARM_SME_F64F64_INSTRUCTIONS_AVAILABLE    85   
#define PF_ARM_SME_I16I64_INSTRUCTIONS_AVAILABLE    86   
#define PF_ARM_SME_LUTv2_INSTRUCTIONS_AVAILABLE     87   
#define PF_ARM_SME_FA64_INSTRUCTIONS_AVAILABLE      88   
//

//
// Known extended CPU state feature BITs (AMD64 and x86).
//
// 0    x87
// 1    SSE
// 2    AVX
// 3    BNDREGS (B0.LB-B3.LB B0.UB-B3.UB)
// 4    BNDCSR  (BNDCFGU + BNDSTATUS)       Persistent
// 5    KMASK   (KMASK [63:0][0-7])
// 6    ZMM_H   (ZMM_H[511:256][0-15])
// 7    ZMM     (ZMM[511:0][16-31])
// 8    IPT                                 Supervisor
// 10   PASID                               Supervisor
// 11   CET_U                               Supervisor
// 12   CET_S                               Supervisor (Cannot be used by NT! Only defined for SK intercept purposes!)
//
// 17   TILE_CONFIG
// 18   TILE_DATA                           XFD, Large
//
// 62   LWP                                 Persistent
//
// 63   RZ0                                 Reserved
//

#define XSTATE_LEGACY_FLOATING_POINT        (0)
#define XSTATE_LEGACY_SSE                   (1)
#define XSTATE_GSSE                         (2)
#define XSTATE_AVX                          (XSTATE_GSSE)
#define XSTATE_MPX_BNDREGS                  (3)
#define XSTATE_MPX_BNDCSR                   (4)
#define XSTATE_AVX512_KMASK                 (5)
#define XSTATE_AVX512_ZMM_H                 (6)
#define XSTATE_AVX512_ZMM                   (7)
#define XSTATE_IPT                          (8)
#define XSTATE_PASID                        (10)
#define XSTATE_CET_U                        (11)
#define XSTATE_CET_S                        (12)
#define XSTATE_AMX_TILE_CONFIG              (17)
#define XSTATE_AMX_TILE_DATA                (18)
#define XSTATE_LWP                          (62)
#define MAXIMUM_XSTATE_FEATURES             (64)

//
// Known extended CPU state feature MASKs (AMD64 and x86).
//

#define XSTATE_MASK_LEGACY_FLOATING_POINT   (1ui64 << (XSTATE_LEGACY_FLOATING_POINT))
#define XSTATE_MASK_LEGACY_SSE              (1ui64 << (XSTATE_LEGACY_SSE))

#define XSTATE_MASK_GSSE                    (1ui64 << (XSTATE_GSSE))
#define XSTATE_MASK_AVX                     (XSTATE_MASK_GSSE)
#define XSTATE_MASK_MPX                     ((1ui64 << (XSTATE_MPX_BNDREGS)) | \
                                             (1ui64 << (XSTATE_MPX_BNDCSR)))

#define XSTATE_MASK_AVX512                  ((1ui64 << (XSTATE_AVX512_KMASK)) | \
                                             (1ui64 << (XSTATE_AVX512_ZMM_H)) | \
                                             (1ui64 << (XSTATE_AVX512_ZMM)))

#define XSTATE_MASK_IPT                     (1ui64 << (XSTATE_IPT))
#define XSTATE_MASK_PASID                   (1ui64 << (XSTATE_PASID))
#define XSTATE_MASK_CET_U                   (1ui64 << (XSTATE_CET_U))
#define XSTATE_MASK_CET_S                   (1ui64 << (XSTATE_CET_S))
#define XSTATE_MASK_AMX_TILE_CONFIG         (1ui64 << (XSTATE_AMX_TILE_CONFIG))
#define XSTATE_MASK_AMX_TILE_DATA           (1ui64 << (XSTATE_AMX_TILE_DATA))
#define XSTATE_MASK_LWP                     (1ui64 << (XSTATE_LWP))

//
// Known extended CPU state feature BITs (ARM64).
//
// 0    Unused
// 1    Unused
// 2    SVE
//

#define XSTATE_ARM64_SVE                    (2)

//
// Known extended CPU state feature MASKs (ARM64).
//

#define XSTATE_MASK_ARM64_SVE               (1ui64 << (XSTATE_ARM64_SVE))

#if defined(_AMD64_)

#define XSTATE_MASK_LEGACY                  (XSTATE_MASK_LEGACY_FLOATING_POINT | \
                                             XSTATE_MASK_LEGACY_SSE)

#define XSTATE_MASK_ALLOWED                 (XSTATE_MASK_LEGACY | \
                                             XSTATE_MASK_AVX | \
                                             XSTATE_MASK_MPX | \
                                             XSTATE_MASK_AVX512 | \
                                             XSTATE_MASK_IPT | \
                                             XSTATE_MASK_PASID | \
                                             XSTATE_MASK_CET_U | \
                                             XSTATE_MASK_AMX_TILE_CONFIG | \
                                             XSTATE_MASK_AMX_TILE_DATA | \
                                             XSTATE_MASK_LWP)

#define XSTATE_MASK_PERSISTENT              ((1ui64 << (XSTATE_MPX_BNDCSR)) | \
                                             XSTATE_MASK_LWP)

#define XSTATE_MASK_USER_VISIBLE_SUPERVISOR (XSTATE_MASK_CET_U)

#define XSTATE_MASK_LARGE_FEATURES          (XSTATE_MASK_AMX_TILE_DATA)

#define XSTATE_FIRST_NON_LEGACY_FEATURE     XSTATE_AVX

#elif defined(_X86_)

#define XSTATE_MASK_LEGACY                  (XSTATE_MASK_LEGACY_FLOATING_POINT | \
                                             XSTATE_MASK_LEGACY_SSE)

#define XSTATE_MASK_ALLOWED                 (XSTATE_MASK_LEGACY | \
                                             XSTATE_MASK_AVX | \
                                             XSTATE_MASK_MPX | \
                                             XSTATE_MASK_AVX512 | \
                                             XSTATE_MASK_IPT | \
                                             XSTATE_MASK_CET_U | \
                                             XSTATE_MASK_LWP)

#define XSTATE_MASK_PERSISTENT              ((1ui64 << (XSTATE_MPX_BNDCSR)) | \
                                             XSTATE_MASK_LWP)

#define XSTATE_MASK_USER_VISIBLE_SUPERVISOR (XSTATE_MASK_CET_U)

#define XSTATE_MASK_LARGE_FEATURES          (0ui64)

#define XSTATE_FIRST_NON_LEGACY_FEATURE     XSTATE_AVX

#elif defined(_ARM64_)

#define XSTATE_MASK_LEGACY                  (0ui64)

#define XSTATE_MASK_ALLOWED                 (XSTATE_MASK_ARM64_SVE)

#define XSTATE_MASK_PERSISTENT              (0ui64)

#define XSTATE_MASK_USER_VISIBLE_SUPERVISOR (0ui64)

#define XSTATE_MASK_LARGE_FEATURES          (0ui64)

#define XSTATE_FIRST_NON_LEGACY_FEATURE     XSTATE_ARM64_SVE

#endif

#define XSTATE_MASK_AMD64_LEGACY            (XSTATE_MASK_LEGACY_FLOATING_POINT | \
                                             XSTATE_MASK_LEGACY_SSE)

//
// Large XSTATE features are not supported in x86.
//

#if defined(_X86_)

#if !defined(__midl) && !defined(MIDL_PASS)

C_ASSERT((XSTATE_MASK_ALLOWED & XSTATE_MASK_LARGE_FEATURES) == 0);

#endif

#endif

//
// Flags associated with compaction mask
//

#define XSTATE_COMPACTION_ENABLE            (63)
#define XSTATE_COMPACTION_ENABLE_MASK       (1ui64 << (XSTATE_COMPACTION_ENABLE))

#define XSTATE_ALIGN_BIT                    (1)
#define XSTATE_ALIGN_MASK                   (1ui64 << (XSTATE_ALIGN_BIT))

#define XSTATE_XFD_BIT                      (2)
#define XSTATE_XFD_MASK                     (1ui64 << (XSTATE_XFD_BIT))

#define XSTATE_CONTROLFLAG_XSAVEOPT_MASK    (1)
#define XSTATE_CONTROLFLAG_XSAVEC_MASK      (2)
#define XSTATE_CONTROLFLAG_XFD_MASK         (4)
#define XSTATE_CONTROLFLAG_VALID_MASK       (XSTATE_CONTROLFLAG_XSAVEOPT_MASK | \
                                             XSTATE_CONTROLFLAG_XSAVEC_MASK | \
                                             XSTATE_CONTROLFLAG_XFD_MASK)

//
// Extended processor state configuration
//

typedef struct _XSTATE_FEATURE {
    DWORD Offset;
    DWORD Size;
} XSTATE_FEATURE, *PXSTATE_FEATURE;

typedef struct _XSTATE_CONFIGURATION {
    // Mask of all enabled features
    DWORD64 EnabledFeatures;

    // Mask of volatile enabled features
    DWORD64 EnabledVolatileFeatures;

    // Total size of the save area for user states
    DWORD Size;

    // Control Flags
    union {
        DWORD ControlFlags;
        struct
        {
            DWORD OptimizedSave : 1;
            DWORD CompactionEnabled : 1;
            DWORD ExtendedFeatureDisable : 1;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

    // List of features
    XSTATE_FEATURE Features[MAXIMUM_XSTATE_FEATURES];

    // Mask of all supervisor features
    DWORD64 EnabledSupervisorFeatures;

    // Mask of features that require start address to be 64 byte aligned
    DWORD64 AlignedFeatures;

    // Total size of the save area for user and supervisor states
    DWORD AllFeatureSize;

    // List which holds size of each user and supervisor state supported by CPU
    DWORD AllFeatures[MAXIMUM_XSTATE_FEATURES];

    // Mask of all supervisor features that are exposed to user-mode
    DWORD64 EnabledUserVisibleSupervisorFeatures;

    // Mask of features that can be disabled via XFD
    DWORD64 ExtendedFeatureDisableFeatures;

    // Total size of the save area for non-large user and supervisor states
    DWORD AllNonLargeFeatureSize;

    // The maximum supported ARM64 SVE vector length that can be used in the
    // current environment, in bytes.
    WORD   MaxSveVectorLength;

    WORD   Spare1;

} XSTATE_CONFIGURATION, *PXSTATE_CONFIGURATION;

//
//

//
//  Runtime Report Definitions
//

//
// ===============================================
// Runtime Report Package Format:
//
// ------------------------------------- Signed part Begin
//
//     RUNTIME_REPORT_PACKAGE_HEADER
//
//     BYTE Nonce[RUNTIME_REPORT_NONCE_SIZE]
//
//     RUNTIME_REPORT_DIGEST_HEADER_A
//
//     RUNTIME_REPORT_DIGEST_HEADER_B
//     ...
//     ...
//
// ------------------------------------- Signed part End
//
//     Signature Blob
//
// ------------------------------------- Authenticated part Begin
//
//     RUNTIME_REPORT_HEADER
//     REPORT_A
//
//     RUNTIME_REPORT_HEADER
//     REPORT_B
//
// ------------------------------------- Authenticated part End
//
// ===============================================
//

#define RUNTIME_REPORT_PACKAGE_MAGIC    0x52545250  // = "RTRP"

#define RUNTIME_REPORT_PACKAGE_VERSION_CURRENT  (1)

#define RUNTIME_REPORT_NONCE_SIZE   32

#define RUNTIME_REPORT_DIGEST_MAX_SIZE  64

#define RUNTIME_REPORT_SIGNATURE_SCHEME_SHA512_RSA_PSS_SHA512   (1)

//
// Runtime Report Type Enumeration
//

typedef enum _RUNTIME_REPORT_TYPE {
    RuntimeReportTypeDriver = 0,
    RuntimeReportTypeMax
} RUNTIME_REPORT_TYPE;

//
// Macro to convert a report type enum value to a bitmap mask
//

#define RUNTIME_REPORT_TYPE_TO_MASK(type) (1ULL << (type))

//
// Bitmap mask containing all valid report types
//

#define RUNTIME_REPORT_TYPE_MASK_ALL ((1ULL << RuntimeReportTypeMax) - 1)

typedef struct _RUNTIME_REPORT_PACKAGE_HEADER {

    //
    // Set to RUNTIME_REPORT_PACKAGE_MAGIC = 0x52545250 ("RTRP")
    //

    UINT32 Magic;

    //
    // The version of the package format
    //

    UINT16 PackageVersion;

    //
    // Number of different report types contained in the package.
    //

    UINT16 NumberOfReports;

    //
    // A bitmap of all the report types in the package.
    //
    // Use RUNTIME_REPORT_TYPE_TO_MASK macro to convert enum values to bitmap masks.
    // Current valid report types:
    //      RuntimeReportTypeDriver = 0
    //

    UINT64 ReportTypesBitmap;

    //
    // The size of the total package including the package header,
    // various runtime reports, their digests, and the signature blob.
    //

    UINT32 PackageSize;

    //
    // The type of digest contained in the report digest headers.
    //
    // Current valid values:
    //      CALG_SHA_512 (see wincrypt.h)
    //

    UINT16 ReportDigestType;

    //
    // Total size of the signed runtime report digest headers
    // following the package header.
    //

    UINT16 TotalReportDigestsSize;

    //
    // Reserved field. Must be set to zero.
    //

    UINT16 Reserved;

    //
    // The signature scheme used to sign the runtime reports.
    //
    // Current valid values:
    //      RUNTIME_REPORT_SIGNATURE_SCHEME_SHA512_RSA_PSS_SHA512 = 1
    //

    UINT16 SignatureScheme;

    //
    // Size of the signature blob following the runtime report digests.
    //

    UINT32 SignatureSize;

    //
    // Total size of the authenticated (but unsigned) runtime reports
    // following the signature blob.
    //

    UINT32 TotalAuthenticatedReportsSize;

} RUNTIME_REPORT_PACKAGE_HEADER, *PRUNTIME_REPORT_PACKAGE_HEADER;

typedef struct _RUNTIME_REPORT_DIGEST_HEADER {

    //
    // Indicates the type of report that was hashed.
    //
    // Current valid values:
    //      RuntimeReportTypeDriver = 0
    //

    UINT16 ReportType;

    //
    // Reserved field.
    //

    UINT16 Reserved;

    //
    // Digest of the report including the report header.
    // This is a SHA-512 digest.
    //

    UINT8 ReportDigest[RUNTIME_REPORT_DIGEST_MAX_SIZE];

} RUNTIME_REPORT_DIGEST_HEADER, *PRUNTIME_REPORT_DIGEST_HEADER;

typedef struct _RUNTIME_REPORT_HEADER {

    //
    // Indicates the type of report.
    //
    // Current valid values:
    //      RuntimeReportTypeDriver = 0
    //

    UINT16 ReportType;

    //
    // Reserved field.
    //

    UINT16 Reserved;

    //
    // The number of bytes consumed by this report, including the header.
    //

    UINT32 ReportSize;

} RUNTIME_REPORT_HEADER, *PRUNTIME_REPORT_HEADER;

//
//  Driver Report Definitions
//

#define DRIVER_REPORT_DIGEST_MAX_SIZE   RUNTIME_REPORT_DIGEST_MAX_SIZE

#define DRIVER_REPORT_NAME_MAX_LENGTH   32

typedef struct _DRIVER_INFO_ENTRY {

    //
    // Internal name of the driver from the resource section.
    //

    CHAR InternalName[DRIVER_REPORT_NAME_MAX_LENGTH];

    //
    // Hash algorithm used to calculate the image digest.
    //

    UINT16 ImageHashAlgorithm;

    //
    // Hash algorithm used to calculate the thumbprint of the leaf certificate
    // that validates the entire image.
    //

    UINT16 PublisherThumbprintHashAlgorithm;

    //
    // Offset from the start of the driver report to a buffer containing the
    // digest of the driver image on disk.
    //

    UINT32 ImageHashOffset;

    //
    // Offset from the start of the driver report to a buffer containing the
    // thumbprint of the leaf certificate validating the entire image
    //

    UINT32 PublisherThumbprintOffset;

    //
    // Number of times that this driver image has been loaded into the system.
    //

    UINT16 LoadCount;

    //
    // Size and Offset of a string indicating the OEM name stored in the
    // authenticated OPUS block of the image digital signature.
    // There is no OEM name for inbox Windows signed drivers. The size does *NOT*
    // include the NULL terminator (even though the string is NULL-terminated).
    //

    UINT16 OemNameSize;
    UINT32 OemNameOffset;

    //
    // Flags indicating various properties of the current driver image:
    //      - Unloaded - Set to 1 in case the driver is current unloaded.
    //
    //      - BootDriver - Set to 1 in case the image is a Boot Driver;
    //           0 otherwise (the image is a Runtime driver).
    //
    //      - HotPatch - Set to 1 in case the image can be also loaded as Hotpatch;
    //
    //      - Reserved - Reserved flags bits.
    //

    union {
        struct {
            UINT16 Unloaded : 1;
            UINT16 BootDriver : 1;
            UINT16 HotPatch : 1;
            UINT16 Reserved : 13;
        };
        UINT16 AsUInt16;
    } Flags;

    UINT16 Padding;
} DRIVER_INFO_ENTRY, *PDRIVER_INFO_ENTRY;

typedef struct _DRIVER_RUNTIME_REPORT {

    //
    // The driver runtime report header.
    //

    RUNTIME_REPORT_HEADER Header;

    //
    // The current number of unique drivers in the report.
    //

    UINT16 NumberOfDrivers;

    //
    // Flags indicating various properties of the report:
    //      - ReportOverflowed - Secure Kernel places a limit on the number of
    //          drivers it can list in the report. If this is set, it indicates
    //          that some loaded drivers might be missing from the report.
    //
    //      - PartialReport - Indicates whether the report contains only a
    //          subset of NT loaded drivers.
    //
    //      - IncludeBootDrivers - Set to 1 in case the report includes
    //          boot-loaded drivers; 0 otherwise (in that case the information
    //          is stored in the TCG Log).
    //
    //      - Reserved - Reserved flags bits.
    //

    union {
        struct {
            UINT16 ReportOverflowed : 1;
            UINT16 PartialReport : 1;
            UINT16 IncludeBootDrivers : 1;
            UINT16 Reserved : 13;
        };
        UINT16 AsUInt16;
    } Flags;

    //
    // A list, of size zero up to MaximumDriversRecorded, containing driver entries.
    // Unloaded drivers are not removed from the list.
    //

    DRIVER_INFO_ENTRY DriverEntries[ANYSIZE_ARRAY];

    //
    // After the driver info array the driver runtime report store hashes,
    // strings and information that are dynamic in size.
    //
    // BYTE DynamicBuffer[ANYSIZE_ARRAY];
    //
    // The dynamic buffer, for each driver is composed off:
    // ImageHash - PublisherHash - OemName.
    //

} DRIVER_RUNTIME_REPORT, *PDRIVER_RUNTIME_REPORT;

//
// begin_ntifs

typedef struct _MEMORY_BASIC_INFORMATION {
    PVOID BaseAddress;
    PVOID AllocationBase;
    DWORD AllocationProtect;
#if defined (_WIN64)
    WORD   PartitionId;
#endif
    SIZE_T RegionSize;
    DWORD State;
    DWORD Protect;
    DWORD Type;
} MEMORY_BASIC_INFORMATION, *PMEMORY_BASIC_INFORMATION;

// end_ntifs

typedef struct _MEMORY_BASIC_INFORMATION32 {
    DWORD BaseAddress;
    DWORD AllocationBase;
    DWORD AllocationProtect;
    DWORD RegionSize;
    DWORD State;
    DWORD Protect;
    DWORD Type;
} MEMORY_BASIC_INFORMATION32, *PMEMORY_BASIC_INFORMATION32;

typedef struct DECLSPEC_ALIGN(16) _MEMORY_BASIC_INFORMATION64 {
    ULONGLONG BaseAddress;
    ULONGLONG AllocationBase;
    DWORD     AllocationProtect;
    DWORD     __alignment1;
    ULONGLONG RegionSize;
    DWORD     State;
    DWORD     Protect;
    DWORD     Type;
    DWORD     __alignment2;
} MEMORY_BASIC_INFORMATION64, *PMEMORY_BASIC_INFORMATION64;

//
// Define flags for setting process CFG valid call target entries.
//

//
// Call target should be made valid.  If not set, the call target is made
// invalid.  Input flag.
//

#define CFG_CALL_TARGET_VALID                               (0x00000001)

//
// Call target has been successfully processed.  Used to report to the caller
// how much progress has been made.  Output flag.
//

#define CFG_CALL_TARGET_PROCESSED                           (0x00000002)

//
// Call target should be made valid only if it is suppressed export.
// What this flag means is that it can *only* be used on a cell which is
// currently in the CFG export suppressed state (only considered for export
// suppressed processes and not legacy CFG processes!), and it is also
// allowed to be used even if the process is a restricted (i.e. no ACG) process.
//

#define CFG_CALL_TARGET_CONVERT_EXPORT_SUPPRESSED_TO_VALID  (0x00000004)

//
// Call target should be made into an XFG call target.
//
#define CFG_CALL_TARGET_VALID_XFG                           (0x00000008)
//
// Call target should be made valid only if it is already an XFG target
// in a process which has XFG audit mode enabled.
//
#define CFG_CALL_TARGET_CONVERT_XFG_TO_CFG                  (0x00000010)

typedef struct _CFG_CALL_TARGET_INFO {
    ULONG_PTR Offset;
    ULONG_PTR Flags;
} CFG_CALL_TARGET_INFO, *PCFG_CALL_TARGET_INFO;

#define SECTION_QUERY                0x0001
#define SECTION_MAP_WRITE            0x0002
#define SECTION_MAP_READ             0x0004
#define SECTION_MAP_EXECUTE          0x0008
#define SECTION_EXTEND_SIZE          0x0010
#define SECTION_MAP_EXECUTE_EXPLICIT 0x0020 // not included in SECTION_ALL_ACCESS

#define SECTION_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED|SECTION_QUERY|\
                            SECTION_MAP_WRITE |      \
                            SECTION_MAP_READ |       \
                            SECTION_MAP_EXECUTE |    \
                            SECTION_EXTEND_SIZE)

//
// Session Specific Access Rights.
//

#define SESSION_QUERY_ACCESS  0x0001
#define SESSION_MODIFY_ACCESS 0x0002

#define SESSION_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED |  \
                            SESSION_QUERY_ACCESS |             \
                            SESSION_MODIFY_ACCESS)

// end_access
#define PAGE_NOACCESS           0x01    
#define PAGE_READONLY           0x02    
#define PAGE_READWRITE          0x04    
#define PAGE_WRITECOPY          0x08    
#define PAGE_EXECUTE            0x10    
#define PAGE_EXECUTE_READ       0x20    
#define PAGE_EXECUTE_READWRITE  0x40    
#define PAGE_EXECUTE_WRITECOPY  0x80    
#define PAGE_GUARD             0x100    
#define PAGE_NOCACHE           0x200    
#define PAGE_WRITECOMBINE      0x400    
#define PAGE_GRAPHICS_NOACCESS           0x0800    
#define PAGE_GRAPHICS_READONLY           0x1000    
#define PAGE_GRAPHICS_READWRITE          0x2000    
#define PAGE_GRAPHICS_EXECUTE            0x4000    
#define PAGE_GRAPHICS_EXECUTE_READ       0x8000    
#define PAGE_GRAPHICS_EXECUTE_READWRITE 0x10000    
#define PAGE_GRAPHICS_COHERENT          0x20000    
#define PAGE_GRAPHICS_NOCACHE           0x40000    
#define PAGE_ENCLAVE_THREAD_CONTROL 0x80000000  
#define PAGE_REVERT_TO_FILE_MAP     0x80000000  
#define PAGE_TARGETS_NO_UPDATE      0x40000000  
#define PAGE_TARGETS_INVALID        0x40000000  
#define PAGE_ENCLAVE_UNVALIDATED    0x20000000  
#define PAGE_ENCLAVE_MASK           0x10000000  
#define PAGE_ENCLAVE_DECOMMIT       (PAGE_ENCLAVE_MASK | 0) 
#define PAGE_ENCLAVE_SS_FIRST       (PAGE_ENCLAVE_MASK | 1) 
#define PAGE_ENCLAVE_SS_REST        (PAGE_ENCLAVE_MASK | 2) 
#define MEM_COMMIT                      0x00001000  
#define MEM_RESERVE                     0x00002000  
#define MEM_REPLACE_PLACEHOLDER         0x00004000  
#define MEM_RESERVE_PLACEHOLDER         0x00040000  
#define MEM_RESET                       0x00080000  
#define MEM_TOP_DOWN                    0x00100000  
#define MEM_WRITE_WATCH                 0x00200000  
#define MEM_PHYSICAL                    0x00400000  
#define MEM_ROTATE                      0x00800000  
#define MEM_DIFFERENT_IMAGE_BASE_OK     0x00800000  
#define MEM_RESET_UNDO                  0x01000000  
#define MEM_LARGE_PAGES                 0x20000000  
#define MEM_4MB_PAGES                   0x80000000  
#define MEM_64K_PAGES                   (MEM_LARGE_PAGES | MEM_PHYSICAL)  
#define MEM_UNMAP_WITH_TRANSIENT_BOOST  0x00000001  
#define MEM_COALESCE_PLACEHOLDERS       0x00000001  
#define MEM_PRESERVE_PLACEHOLDER        0x00000002  
#define MEM_DECOMMIT                    0x00004000  
#define MEM_RELEASE                     0x00008000  
#define MEM_FREE                        0x00010000  
#define MEM_EXTENDED_PARAMETER_GRAPHICS                 0x00000001  
#define MEM_EXTENDED_PARAMETER_NONPAGED                 0x00000002  
#define MEM_EXTENDED_PARAMETER_ZERO_PAGES_OPTIONAL      0x00000004  
#define MEM_EXTENDED_PARAMETER_NONPAGED_LARGE           0x00000008  
#define MEM_EXTENDED_PARAMETER_NONPAGED_HUGE            0x00000010  
#define MEM_EXTENDED_PARAMETER_SOFT_FAULT_PAGES         0x00000020  
#define MEM_EXTENDED_PARAMETER_EC_CODE                  0x00000040  
#define MEM_EXTENDED_PARAMETER_NUMA_NODE_MANDATORY      MINLONG64	  
// begin_wdm

typedef enum MEM_EXTENDED_PARAMETER_TYPE {
    MemExtendedParameterInvalidType = 0,
    MemExtendedParameterAddressRequirements,
    MemExtendedParameterNumaNode,
    MemExtendedParameterPartitionHandle,
    MemExtendedParameterUserPhysicalHandle,
    MemExtendedParameterAttributeFlags,
    MemExtendedParameterImageMachine,
    MemExtendedParameterMax
} MEM_EXTENDED_PARAMETER_TYPE, *PMEM_EXTENDED_PARAMETER_TYPE;

#define MEM_EXTENDED_PARAMETER_TYPE_BITS    8

typedef struct DECLSPEC_ALIGN(8) MEM_EXTENDED_PARAMETER {

    struct {
        DWORD64 Type : MEM_EXTENDED_PARAMETER_TYPE_BITS;
        DWORD64 Reserved : 64 - MEM_EXTENDED_PARAMETER_TYPE_BITS;
    } DUMMYSTRUCTNAME;

    union {
        DWORD64 ULong64;
        PVOID Pointer;
        SIZE_T Size;
        HANDLE Handle;
        DWORD ULong;
    } DUMMYUNIONNAME;

} MEM_EXTENDED_PARAMETER, *PMEM_EXTENDED_PARAMETER;

typedef struct _MEM_ADDRESS_REQUIREMENTS {
    PVOID LowestStartingAddress;
    PVOID HighestEndingAddress;
    SIZE_T Alignment;
} MEM_ADDRESS_REQUIREMENTS, *PMEM_ADDRESS_REQUIREMENTS;

#define MEMORY_CURRENT_PARTITION_HANDLE         ((HANDLE) (LONG_PTR) -1)
#define MEMORY_SYSTEM_PARTITION_HANDLE          ((HANDLE) (LONG_PTR) -2)
#define MEMORY_EXISTING_VAD_PARTITION_HANDLE    ((HANDLE) (LONG_PTR) -3)

//
// Dedicated memory attributes.
//

#define MEM_DEDICATED_ATTRIBUTE_NOT_SPECIFIED   ((DWORD64) -1)

typedef enum _MEM_DEDICATED_ATTRIBUTE_TYPE {
    MemDedicatedAttributeReadBandwidth = 0,
    MemDedicatedAttributeReadLatency,
    MemDedicatedAttributeWriteBandwidth,
    MemDedicatedAttributeWriteLatency,
    MemDedicatedAttributeMax
} MEM_DEDICATED_ATTRIBUTE_TYPE, *PMEM_DEDICATED_ATTRIBUTE_TYPE;

#define SEC_HUGE_PAGES              0x00020000  
#define SEC_PARTITION_OWNER_HANDLE  0x00040000  
#define SEC_64K_PAGES               0x00080000  
#define SEC_FILE                    0x00800000  
#define SEC_IMAGE                   0x01000000  
#define SEC_PROTECTED_IMAGE         0x02000000  
#define SEC_RESERVE                 0x04000000  
#define SEC_COMMIT                  0x08000000  
#define SEC_NOCACHE                 0x10000000  
#define SEC_WRITECOMBINE            0x40000000  
#define SEC_LARGE_PAGES             0x80000000  
#define SEC_IMAGE_NO_EXECUTE (SEC_IMAGE | SEC_NOCACHE)  
// begin_wdm

typedef enum MEM_SECTION_EXTENDED_PARAMETER_TYPE {
    MemSectionExtendedParameterInvalidType = 0,
    MemSectionExtendedParameterUserPhysicalFlags,
    MemSectionExtendedParameterNumaNode,
    MemSectionExtendedParameterSigningLevel,
    MemSectionExtendedParameterMax
} MEM_SECTION_EXTENDED_PARAMETER_TYPE, *PMEM_SECTION_EXTENDED_PARAMETER_TYPE;

#define MEM_PRIVATE                 0x00020000  
#define MEM_MAPPED                  0x00040000  
#define MEM_IMAGE                   0x01000000  
#define WRITE_WATCH_FLAG_RESET  0x01    
#define VM_PREFETCH_TO_WORKING_SET        0x1  

#define ENCLAVE_TYPE_SGX            0x00000001
#define ENCLAVE_TYPE_SGX2           0x00000002

typedef struct _ENCLAVE_CREATE_INFO_SGX {
    BYTE  Secs[4096];
} ENCLAVE_CREATE_INFO_SGX, *PENCLAVE_CREATE_INFO_SGX;

typedef struct _ENCLAVE_INIT_INFO_SGX {
    BYTE  SigStruct[1808];
    BYTE  Reserved1[240];
    BYTE  EInitToken[304];
    BYTE  Reserved2[1744];
} ENCLAVE_INIT_INFO_SGX, *PENCLAVE_INIT_INFO_SGX;

#define ENCLAVE_TYPE_VBS            0x00000010

typedef struct _ENCLAVE_CREATE_INFO_VBS {
    DWORD Flags;
    BYTE  OwnerID[32];
} ENCLAVE_CREATE_INFO_VBS, *PENCLAVE_CREATE_INFO_VBS;

#define ENCLAVE_VBS_FLAG_DEBUG      0x00000001

#define ENCLAVE_TYPE_VBS_BASIC      0x00000011

typedef struct _ENCLAVE_CREATE_INFO_VBS_BASIC {
    DWORD Flags;
    BYTE  OwnerID[32];
} ENCLAVE_CREATE_INFO_VBS_BASIC, *PENCLAVE_CREATE_INFO_VBS_BASIC;

typedef struct _ENCLAVE_LOAD_DATA_VBS_BASIC {
    DWORD PageType;
} ENCLAVE_LOAD_DATA_VBS_BASIC, *PENCLAVE_LOAD_DATA_VBS_BASIC;

#define VBS_BASIC_PAGE_MEASURED_DATA        0x00000001
#define VBS_BASIC_PAGE_UNMEASURED_DATA      0x00000002
#define VBS_BASIC_PAGE_ZERO_FILL            0x00000003
#define VBS_BASIC_PAGE_THREAD_DESCRIPTOR    0x00000004
#define VBS_BASIC_PAGE_SYSTEM_CALL          0x00000005

typedef struct _ENCLAVE_INIT_INFO_VBS_BASIC {
    BYTE  FamilyId[ENCLAVE_SHORT_ID_LENGTH];
    BYTE  ImageId[ENCLAVE_SHORT_ID_LENGTH];
    ULONGLONG EnclaveSize;
    DWORD EnclaveSvn;
    DWORD Reserved;
    union {
        HANDLE SignatureInfoHandle;
        ULONGLONG Unused;
    } DUMMYUNIONNAME;
} ENCLAVE_INIT_INFO_VBS_BASIC, *PENCLAVE_INIT_INFO_VBS_BASIC;


typedef struct _ENCLAVE_INIT_INFO_VBS {
    DWORD Length;
    DWORD ThreadCount;
} ENCLAVE_INIT_INFO_VBS, *PENCLAVE_INIT_INFO_VBS;

#if !defined(SORTPP_PASS) && !defined(MIDL_PASS) && !defined(RC_INVOKED)

typedef PVOID (ENCLAVE_TARGET_FUNCTION)(PVOID);
typedef ENCLAVE_TARGET_FUNCTION (*PENCLAVE_TARGET_FUNCTION);
typedef PENCLAVE_TARGET_FUNCTION LPENCLAVE_TARGET_FUNCTION;

#endif


#define DEDICATED_MEMORY_CACHE_ELIGIBLE   0x1

typedef struct DECLSPEC_ALIGN(8) _MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    MEM_DEDICATED_ATTRIBUTE_TYPE Type;
    DWORD Reserved;

    //
    // The unit of Value is determined by the Type.
    // When Type is a latency, Value is in picoseconds.
    // When Type is a bandwidth, Value is in megabyte per second.
    //
    DWORD64 Value;
} MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE, *PMEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE;

typedef struct DECLSPEC_ALIGN(8) _MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {

    //
    // Offset of the next information entry from the beginning of this
    // information entry.
    //

    DWORD NextEntryOffset;

    //
    // Size of this information entry.
    //

    DWORD SizeOfInformation;

    //
    // Various flags.
    //

    DWORD Flags;

    //
    // Offset (from the beginning of this entry) to the set of available
    // attributes of the dedicated memory.
    //

    DWORD AttributesOffset;

    //
    // Number of available dedicated memory attributes.
    //

    DWORD AttributeCount;

    //
    // Reserved field.
    //

    DWORD Reserved;

    //
    // Type identifier for this dedicated memory used to open a handle.
    //

    DWORD64 TypeId;

} MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION, *PMEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION;


//
// Define access rights to files and directories
//

//
// The FILE_READ_DATA and FILE_WRITE_DATA constants are also defined in
// devioctl.h as FILE_READ_ACCESS and FILE_WRITE_ACCESS. The values for these
// constants *MUST* always be in sync.
// The values are redefined in devioctl.h because they must be available to
// both DOS and NT.
//

#define FILE_READ_DATA            ( 0x0001 )    // file & pipe
#define FILE_LIST_DIRECTORY       ( 0x0001 )    // directory

#define FILE_WRITE_DATA           ( 0x0002 )    // file & pipe
#define FILE_ADD_FILE             ( 0x0002 )    // directory

#define FILE_APPEND_DATA          ( 0x0004 )    // file
#define FILE_ADD_SUBDIRECTORY     ( 0x0004 )    // directory
#define FILE_CREATE_PIPE_INSTANCE ( 0x0004 )    // named pipe


#define FILE_READ_EA              ( 0x0008 )    // file & directory

#define FILE_WRITE_EA             ( 0x0010 )    // file & directory

#define FILE_EXECUTE              ( 0x0020 )    // file
#define FILE_TRAVERSE             ( 0x0020 )    // directory

#define FILE_DELETE_CHILD         ( 0x0040 )    // directory

#define FILE_READ_ATTRIBUTES      ( 0x0080 )    // all

#define FILE_WRITE_ATTRIBUTES     ( 0x0100 )    // all

#define FILE_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED | SYNCHRONIZE | 0x1FF)

#define FILE_GENERIC_READ         (STANDARD_RIGHTS_READ     |\
                                   FILE_READ_DATA           |\
                                   FILE_READ_ATTRIBUTES     |\
                                   FILE_READ_EA             |\
                                   SYNCHRONIZE)


#define FILE_GENERIC_WRITE        (STANDARD_RIGHTS_WRITE    |\
                                   FILE_WRITE_DATA          |\
                                   FILE_WRITE_ATTRIBUTES    |\
                                   FILE_WRITE_EA            |\
                                   FILE_APPEND_DATA         |\
                                   SYNCHRONIZE)


#define FILE_GENERIC_EXECUTE      (STANDARD_RIGHTS_EXECUTE  |\
                                   FILE_READ_ATTRIBUTES     |\
                                   FILE_EXECUTE             |\
                                   SYNCHRONIZE)

#define FILE_SHARE_READ                 0x00000001  
#define FILE_SHARE_WRITE                0x00000002  
#define FILE_SHARE_DELETE               0x00000004  
#define FILE_ATTRIBUTE_READONLY             0x00000001  
#define FILE_ATTRIBUTE_HIDDEN               0x00000002  
#define FILE_ATTRIBUTE_SYSTEM               0x00000004  
#define FILE_ATTRIBUTE_DIRECTORY            0x00000010  
#define FILE_ATTRIBUTE_ARCHIVE              0x00000020  
#define FILE_ATTRIBUTE_DEVICE               0x00000040  
#define FILE_ATTRIBUTE_NORMAL               0x00000080  
#define FILE_ATTRIBUTE_TEMPORARY            0x00000100  
#define FILE_ATTRIBUTE_SPARSE_FILE          0x00000200  
#define FILE_ATTRIBUTE_REPARSE_POINT        0x00000400  
#define FILE_ATTRIBUTE_COMPRESSED           0x00000800  
#define FILE_ATTRIBUTE_OFFLINE              0x00001000  
#define FILE_ATTRIBUTE_NOT_CONTENT_INDEXED  0x00002000  
#define FILE_ATTRIBUTE_ENCRYPTED            0x00004000  
#define FILE_ATTRIBUTE_INTEGRITY_STREAM     0x00008000  
#define FILE_ATTRIBUTE_VIRTUAL              0x00010000  
#define FILE_ATTRIBUTE_NO_SCRUB_DATA        0x00020000  
#define FILE_ATTRIBUTE_EA                   0x00040000  
#define FILE_ATTRIBUTE_PINNED               0x00080000  
#define FILE_ATTRIBUTE_UNPINNED             0x00100000  
#define FILE_ATTRIBUTE_RECALL_ON_OPEN       0x00040000  
#define FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS 0x00400000 
#define TREE_CONNECT_ATTRIBUTE_PRIVACY      0x00004000  
#define TREE_CONNECT_ATTRIBUTE_INTEGRITY    0x00008000  
#define TREE_CONNECT_ATTRIBUTE_GLOBAL       0x00000004  
#define TREE_CONNECT_ATTRIBUTE_PINNED       0x00000002  
#define FILE_ATTRIBUTE_STRICTLY_SEQUENTIAL  0x20000000  
#define FILE_NOTIFY_CHANGE_FILE_NAME    0x00000001   
#define FILE_NOTIFY_CHANGE_DIR_NAME     0x00000002   
#define FILE_NOTIFY_CHANGE_ATTRIBUTES   0x00000004   
#define FILE_NOTIFY_CHANGE_SIZE         0x00000008   
#define FILE_NOTIFY_CHANGE_LAST_WRITE   0x00000010   
#define FILE_NOTIFY_CHANGE_LAST_ACCESS  0x00000020   
#define FILE_NOTIFY_CHANGE_CREATION     0x00000040   
#define FILE_NOTIFY_CHANGE_SECURITY     0x00000100   
#define FILE_ACTION_ADDED                   0x00000001   
#define FILE_ACTION_REMOVED                 0x00000002   
#define FILE_ACTION_MODIFIED                0x00000003   
#define FILE_ACTION_RENAMED_OLD_NAME        0x00000004   
#define FILE_ACTION_RENAMED_NEW_NAME        0x00000005   
#define MAILSLOT_NO_MESSAGE             ((DWORD)-1) 
#define MAILSLOT_WAIT_FOREVER           ((DWORD)-1) 
#define FILE_CASE_SENSITIVE_SEARCH          0x00000001  
#define FILE_CASE_PRESERVED_NAMES           0x00000002  
#define FILE_UNICODE_ON_DISK                0x00000004  
#define FILE_PERSISTENT_ACLS                0x00000008  
#define FILE_FILE_COMPRESSION               0x00000010  
#define FILE_VOLUME_QUOTAS                  0x00000020  
#define FILE_SUPPORTS_SPARSE_FILES          0x00000040  
#define FILE_SUPPORTS_REPARSE_POINTS        0x00000080  
#define FILE_SUPPORTS_REMOTE_STORAGE        0x00000100  
#define FILE_RETURNS_CLEANUP_RESULT_INFO    0x00000200  
#define FILE_SUPPORTS_POSIX_UNLINK_RENAME   0x00000400  
#define FILE_SUPPORTS_BYPASS_IO             0x00000800  
#define FILE_SUPPORTS_STREAM_SNAPSHOTS      0x00001000  
#define FILE_SUPPORTS_CASE_SENSITIVE_DIRS   0x00002000  

#define FILE_VOLUME_IS_COMPRESSED           0x00008000  
#define FILE_SUPPORTS_OBJECT_IDS            0x00010000  
#define FILE_SUPPORTS_ENCRYPTION            0x00020000  
#define FILE_NAMED_STREAMS                  0x00040000  
#define FILE_READ_ONLY_VOLUME               0x00080000  
#define FILE_SEQUENTIAL_WRITE_ONCE          0x00100000  
#define FILE_SUPPORTS_TRANSACTIONS          0x00200000  
#define FILE_SUPPORTS_HARD_LINKS            0x00400000  
#define FILE_SUPPORTS_EXTENDED_ATTRIBUTES   0x00800000  
#define FILE_SUPPORTS_OPEN_BY_FILE_ID       0x01000000  
#define FILE_SUPPORTS_USN_JOURNAL           0x02000000  
#define FILE_SUPPORTS_INTEGRITY_STREAMS     0x04000000  
#define FILE_SUPPORTS_BLOCK_REFCOUNTING     0x08000000  
#define FILE_SUPPORTS_SPARSE_VDL            0x10000000  
#define FILE_DAX_VOLUME                     0x20000000  
#define FILE_SUPPORTS_GHOSTING              0x40000000  

#define FILE_INVALID_FILE_ID               ((LONGLONG)-1LL) 
typedef struct _FILE_ID_128 {                               
    BYTE  Identifier[16];                                   
} FILE_ID_128, *PFILE_ID_128;                               

//
// Define the file notification information structure
//

typedef struct _FILE_NOTIFY_INFORMATION {
    DWORD NextEntryOffset;
    DWORD Action;
    DWORD FileNameLength;
    WCHAR FileName[1];
} FILE_NOTIFY_INFORMATION, *PFILE_NOTIFY_INFORMATION;

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_RS3)
typedef struct _FILE_NOTIFY_EXTENDED_INFORMATION {
    DWORD NextEntryOffset;
    DWORD Action;
    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastModificationTime;
    LARGE_INTEGER LastChangeTime;
    LARGE_INTEGER LastAccessTime;
    LARGE_INTEGER AllocatedLength;
    LARGE_INTEGER FileSize;
    DWORD FileAttributes;
    union {
        DWORD ReparsePointTag;
        DWORD EaSize;
    } DUMMYUNIONNAME;
    LARGE_INTEGER FileId;
    LARGE_INTEGER ParentFileId;
    DWORD FileNameLength;
    WCHAR FileName[1];
} FILE_NOTIFY_EXTENDED_INFORMATION, *PFILE_NOTIFY_EXTENDED_INFORMATION;
#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_NI)
#define FILE_NAME_FLAG_HARDLINK      0    // not part of a name pair
#define FILE_NAME_FLAG_NTFS          0x01 // NTFS name in a name pair
#define FILE_NAME_FLAG_DOS           0x02 // DOS name in a name pair
#define FILE_NAME_FLAG_BOTH          0x03 // NTFS+DOS combined name
#define FILE_NAME_FLAGS_UNSPECIFIED  0x80 // not specified by file system (do not combine with other flags)

typedef struct _FILE_NOTIFY_FULL_INFORMATION {
    DWORD NextEntryOffset;
    DWORD Action;
    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastModificationTime;
    LARGE_INTEGER LastChangeTime;
    LARGE_INTEGER LastAccessTime;
    LARGE_INTEGER AllocatedLength;
    LARGE_INTEGER FileSize;
    DWORD FileAttributes;
    union {
        DWORD ReparsePointTag;
        DWORD EaSize;
    } DUMMYUNIONNAME;
    LARGE_INTEGER FileId;
    LARGE_INTEGER ParentFileId;
    WORD   FileNameLength;
    BYTE  FileNameFlags;
    BYTE  Reserved;
    WCHAR FileName[1];
} FILE_NOTIFY_FULL_INFORMATION, *PFILE_NOTIFY_FULL_INFORMATION;
#endif


//================ FileStatInformation ========================================

//
// Note: Fields up through EffectiveAccess are the same as in
// FILE_STAT_LX_INFORMATION, and up through NumberOfLinks are the same
// as in FILE_STAT_BASIC_INFORMATION.
//

typedef struct _FILE_STAT_INFORMATION {
    LARGE_INTEGER FileId;
    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastAccessTime;
    LARGE_INTEGER LastWriteTime;
    LARGE_INTEGER ChangeTime;
    LARGE_INTEGER AllocationSize;
    LARGE_INTEGER EndOfFile;
    DWORD FileAttributes;
    DWORD ReparseTag;
    DWORD NumberOfLinks;
    ACCESS_MASK EffectiveAccess;
} FILE_STAT_INFORMATION, *PFILE_STAT_INFORMATION;

//================ FileStatLxInformation ========================================

//
// LxFlags for FILE_STAT_LX_INFORMATION that specify which metadata fields
// were present for the file.
//

#define LX_FILE_METADATA_HAS_UID 0x1
#define LX_FILE_METADATA_HAS_GID 0x2
#define LX_FILE_METADATA_HAS_MODE 0x4
#define LX_FILE_METADATA_HAS_DEVICE_ID 0x8
#define LX_FILE_CASE_SENSITIVE_DIR 0x10

//
// Note: Fields up through EffectiveAccess are the same as in
// FILE_STAT_INFORMATION, and up through NumberOfLinks are the same
// as in FILE_STAT_BASIC_INFORMATION.
//

typedef struct _FILE_STAT_LX_INFORMATION {
    LARGE_INTEGER FileId;
    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastAccessTime;
    LARGE_INTEGER LastWriteTime;
    LARGE_INTEGER ChangeTime;
    LARGE_INTEGER AllocationSize;
    LARGE_INTEGER EndOfFile;
    DWORD FileAttributes;
    DWORD ReparseTag;
    DWORD NumberOfLinks;
    ACCESS_MASK EffectiveAccess;
    DWORD LxFlags;
    DWORD LxUid;
    DWORD LxGid;
    DWORD LxMode;
    DWORD LxDeviceIdMajor;
    DWORD LxDeviceIdMinor;
} FILE_STAT_LX_INFORMATION, *PFILE_STAT_LX_INFORMATION;

//================ FileStatBasicInformation ========================================

//
// Note: Fields up through NumberOfLinks are the same
// as in FILE_STAT_INFORMATION and FILE_STAT_LX_INFORMATION
// (associated with file info classes FileStatInformation and
// FileStatLxInformation, respectively).
//
// Fields DeviceType and DeviceCharacteristics are the same as
// fields DeviceType and Characteristics in FILE_FS_DEVICE_INFORMATION,
// respectively (assocated with file info class FileFsDeviceInformation).
//
// Field VolumeSerialNumber is the same as in FILE_ID_INFORMATION
// (associated with file info class FileIdInformation).
//
// For FileId and FileId128 fields: the distinction here is the same as
// for FILE_ID_ALL_EXTD_BOTH_DIR_INFORMATION; i.e., filesystems can
// represent file IDs in both a 64-bit form (FileId) and a 128-bit
// form (FileId128). Both fields are opaque and dependent on the
// type of filesystem; and both represent a valid way to uniquely
// identify a particular file.
//

#if (NTDDI_VERSION >= NTDDI_WIN11_ZN)

typedef struct _FILE_STAT_BASIC_INFORMATION {
    LARGE_INTEGER FileId;
    LARGE_INTEGER CreationTime;
    LARGE_INTEGER LastAccessTime;
    LARGE_INTEGER LastWriteTime;
    LARGE_INTEGER ChangeTime;
    LARGE_INTEGER AllocationSize;
    LARGE_INTEGER EndOfFile;
    DWORD FileAttributes;
    DWORD ReparseTag;
    DWORD NumberOfLinks;
    DWORD DeviceType;
    DWORD DeviceCharacteristics;
    DWORD Reserved;
    LARGE_INTEGER VolumeSerialNumber;
    FILE_ID_128 FileId128;
} FILE_STAT_BASIC_INFORMATION, *PFILE_STAT_BASIC_INFORMATION;

#endif // (NTDDI_VERSION >= NTDDI_WIN11_ZN)


//
// Flag definitions for Flags in FILE_CASE_SENSITIVE_INFO.
//

#define FILE_CS_FLAG_CASE_SENSITIVE_DIR     0x00000001

typedef struct _FILE_CASE_SENSITIVE_INFORMATION {
    DWORD Flags;
} FILE_CASE_SENSITIVE_INFORMATION, *PFILE_CASE_SENSITIVE_INFORMATION;


//
// Define segment buffer structure for scatter/gather read/write.
//

typedef union _FILE_SEGMENT_ELEMENT {
    PVOID64 Buffer;
    ULONGLONG Alignment;
}FILE_SEGMENT_ELEMENT, *PFILE_SEGMENT_ELEMENT;

#if (NTDDI_VERSION >= NTDDI_WIN8)

//
//  Flag definitions for NtFlushBuffersFileEx
//
//  If none of the below flags are specified the following will occur for a
//  given file handle:
//      - Write any modified data for the given file from the Windows in-memory
//        cache.
//      - Commit all pending metadata changes for the given file from the
//        Windows in-memory cache.
//      - Send a SYNC command to the underlying storage device to commit all
//        written data in the devices cache to persistent storage.
//
//  If a volume handle is specified:
//      - Write all modified data for all files on the volume from the Windows
//        in-memory cache.
//      - Commit all pending metadata changes for all files on the volume from
//        the Windows in-memory cache.
//      - Send a SYNC command to the underlying storage device to commit all
//        written data in the devices cache to persistent storage.
//
//  This is equivalent to how NtFlushBuffersFile has always worked.
//

//
//  If set, this operation will write the data for the given file from the
//  Windows in-memory cache.  This will NOT commit any associated metadata
//  changes.  This will NOT send a SYNC to the storage device to flush its
//  cache.  Not supported on volume handles.
//

#define FLUSH_FLAGS_FILE_DATA_ONLY                      0x00000001

//
//  If set, this operation will commit both the data and metadata changes for
//  the given file from the Windows in-memory cache.  This will NOT send a SYNC
//  to the storage device to flush its cache.  Not supported on volume handles.
//

#define FLUSH_FLAGS_NO_SYNC                             0x00000002

#endif  // (NTDDI_VERSION >= NTDDI_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)

//
//  If set, this operation will write the data for the given file from the
//  Windows in-memory cache.  It will also try to skip updating the timestamp
//  as much as possible.  This will send a SYNC to the storage device to flush its
//  cache.  Not supported on volume or directory handles.
//

#define FLUSH_FLAGS_FILE_DATA_SYNC_ONLY                 0x00000004

#endif  // (NTDDI_VERSION >= NTDDI_WIN10_RS1)

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

//
//  If set, this operation will write the data for the given file from the
//  Windows in-memory cache.  It will also try to skip updating the timestamp
//  as much as possible.  This will send a SYNC to the storage device to flush its
//  cache.  Not supported on volume or directory handles.
//

#define FLUSH_FLAGS_FLUSH_AND_PURGE                     0x00000008

#endif  // (NTDDI_VERSION >= NTDDI_WIN11_GA)


//
// The reparse GUID structure is used by all 3rd party layered drivers to
// store data in a reparse point. For non-Microsoft tags, The GUID field
// cannot be GUID_NULL.
// The constraints on reparse tags are defined below.
// Microsoft tags can also be used with this format of the reparse point buffer.
//

typedef struct _REPARSE_GUID_DATA_BUFFER {
    DWORD  ReparseTag;
    WORD   ReparseDataLength;
    WORD   Reserved;
    GUID   ReparseGuid;
    struct {
        BYTE   DataBuffer[1];
    } GenericReparseBuffer;
} REPARSE_GUID_DATA_BUFFER, *PREPARSE_GUID_DATA_BUFFER;

#define REPARSE_GUID_DATA_BUFFER_HEADER_SIZE   UFIELD_OFFSET(REPARSE_GUID_DATA_BUFFER, GenericReparseBuffer)

//
// Maximum allowed size of the reparse data.
//

#define MAXIMUM_REPARSE_DATA_BUFFER_SIZE      ( 16 * 1024 )

//
// Predefined reparse tags.
// These tags need to avoid conflicting with IO_REMOUNT defined in ntos\inc\io.h
//

#define IO_REPARSE_TAG_RESERVED_ZERO             (0)
#define IO_REPARSE_TAG_RESERVED_ONE              (1)
#define IO_REPARSE_TAG_RESERVED_TWO              (2)

//
// The value of the following constant needs to satisfy the following conditions:
//  (1) Be at least as large as the largest of the reserved tags.
//  (2) Be strictly smaller than all the tags in use.
//

#define IO_REPARSE_TAG_RESERVED_RANGE            IO_REPARSE_TAG_RESERVED_TWO

//
// The reparse tags are a DWORD. The 32 bits are laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +-+-+-+-+-----------------------+-------------------------------+
//  |M|R|N|D|     Reserved bits     |       Reparse Tag Value       |
//  +-+-+-+-+-----------------------+-------------------------------+
//
// M is the Microsoft bit. When set to 1, it denotes a tag owned by Microsoft.
//   All ISVs must use a tag with a 0 in this position.
//   Note: If a Microsoft tag is used by non-Microsoft software, the
//   behavior is not defined.
//
// R is reserved.  Must be zero for non-Microsoft tags.
//
// N is name surrogate. When set to 1, the file represents another named
//   entity in the system.
//
// D is the directory bit. When set to 1, indicates that any directory
//   with this reparse tag can have children. Has no special meaning when used
//   on a non-directory file. Not compatible with the name surrogate bit.
//
// The M and N bits are OR-able.
// The following macros check for the M and N bit values:
//

//
// Macro to determine whether a reparse point tag corresponds to a tag
// owned by Microsoft.
//

#define IsReparseTagMicrosoft(_tag) (              \
                           ((_tag) & 0x80000000)   \
                           )

//
// Macro to determine whether a reparse point tag corresponds to a reserved
// tag owned by Microsoft.
//

#define IsReparseTagReserved(_tag) (               \
                           ((_tag) & 0x40000000)   \
                           )

//
// Macro to determine whether a reparse point tag is a name surrogate
//

#define IsReparseTagNameSurrogate(_tag) (          \
                           ((_tag) & 0x20000000)   \
                           )

//
// Macro to determine whether a directory with this reparse point can have
// children.
//

#define IsReparseTagDirectory(_tag) (    \
                           ((_tag) & 0x10000000)   \
                           )

#define IO_REPARSE_TAG_RESERVED_INVALID         (0xC0008000L)       
#define IO_REPARSE_TAG_MOUNT_POINT              (0xA0000003L)       
#define IO_REPARSE_TAG_HSM                      (0xC0000004L)       
#define IO_REPARSE_TAG_HSM2                     (0x80000006L)       
#define IO_REPARSE_TAG_SIS                      (0x80000007L)       
#define IO_REPARSE_TAG_WIM                      (0x80000008L)       
#define IO_REPARSE_TAG_CSV                      (0x80000009L)       
#define IO_REPARSE_TAG_DFS                      (0x8000000AL)       
#define IO_REPARSE_TAG_SYMLINK                  (0xA000000CL)       
#define IO_REPARSE_TAG_DFSR                     (0x80000012L)       
#define IO_REPARSE_TAG_DEDUP                    (0x80000013L)       
#define IO_REPARSE_TAG_NFS                      (0x80000014L)       
#define IO_REPARSE_TAG_FILE_PLACEHOLDER         (0x80000015L)       
#define IO_REPARSE_TAG_WOF                      (0x80000017L)       
#define IO_REPARSE_TAG_WCI                      (0x80000018L)       
#define IO_REPARSE_TAG_WCI_1                    (0x90001018L)       
#define IO_REPARSE_TAG_GLOBAL_REPARSE           (0xA0000019L)       
#define IO_REPARSE_TAG_CLOUD                    (0x9000001AL)       
#define IO_REPARSE_TAG_CLOUD_1                  (0x9000101AL)       
#define IO_REPARSE_TAG_CLOUD_2                  (0x9000201AL)       
#define IO_REPARSE_TAG_CLOUD_3                  (0x9000301AL)       
#define IO_REPARSE_TAG_CLOUD_4                  (0x9000401AL)       
#define IO_REPARSE_TAG_CLOUD_5                  (0x9000501AL)       
#define IO_REPARSE_TAG_CLOUD_6                  (0x9000601AL)       
#define IO_REPARSE_TAG_CLOUD_7                  (0x9000701AL)       
#define IO_REPARSE_TAG_CLOUD_8                  (0x9000801AL)       
#define IO_REPARSE_TAG_CLOUD_9                  (0x9000901AL)       
#define IO_REPARSE_TAG_CLOUD_A                  (0x9000A01AL)       
#define IO_REPARSE_TAG_CLOUD_B                  (0x9000B01AL)       
#define IO_REPARSE_TAG_CLOUD_C                  (0x9000C01AL)       
#define IO_REPARSE_TAG_CLOUD_D                  (0x9000D01AL)       
#define IO_REPARSE_TAG_CLOUD_E                  (0x9000E01AL)       
#define IO_REPARSE_TAG_CLOUD_F                  (0x9000F01AL)       
#define IO_REPARSE_TAG_CLOUD_MASK               (0x0000F000L)       
#define IO_REPARSE_TAG_APPEXECLINK              (0x8000001BL)       
#define IO_REPARSE_TAG_PROJFS                   (0x9000001CL)       
#define IO_REPARSE_TAG_STORAGE_SYNC             (0x8000001EL)       
#define IO_REPARSE_TAG_WCI_TOMBSTONE            (0xA000001FL)       
#define IO_REPARSE_TAG_UNHANDLED                (0x80000020L)       
#define IO_REPARSE_TAG_ONEDRIVE                 (0x80000021L)       
#define IO_REPARSE_TAG_PROJFS_TOMBSTONE         (0xA0000022L)       
#define IO_REPARSE_TAG_AF_UNIX                  (0x80000023L)       
#define IO_REPARSE_TAG_STORAGE_SYNC_FOLDER      (0x90000027L)       
#define IO_REPARSE_TAG_WCI_LINK                 (0xA0000027L)       
#define IO_REPARSE_TAG_WCI_LINK_1               (0xA0001027L)       
#define IO_REPARSE_TAG_DATALESS_CIM             (0xA0000028L)       

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)

//======================= FSCTL_SCRUB_DATA =============================

#define SCRUB_DATA_INPUT_FLAG_RESUME                           0x00000001
#define SCRUB_DATA_INPUT_FLAG_SKIP_IN_SYNC                     0x00000002
#define SCRUB_DATA_INPUT_FLAG_SKIP_NON_INTEGRITY_DATA          0x00000004
#define SCRUB_DATA_INPUT_FLAG_IGNORE_REDUNDANCY                0x00000008
#define SCRUB_DATA_INPUT_FLAG_SKIP_DATA                        0x00000010
#define SCRUB_DATA_INPUT_FLAG_SCRUB_BY_OBJECT_ID               0x00000020
#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_19H2)
#define SCRUB_DATA_INPUT_FLAG_OPLOCK_NOT_ACQUIRED              0x00000040
#endif

#define SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE                      0x00000001

#define SCRUB_DATA_OUTPUT_FLAG_NON_USER_DATA_RANGE             0x00010000
#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
#define SCRUB_DATA_OUTPUT_FLAG_PARITY_EXTENT_DATA_RETURNED     0x00020000
#define SCRUB_DATA_OUTPUT_FLAG_RESUME_CONTEXT_LENGTH_SPECIFIED 0x00040000
#endif /* _WIN32_WINNT >= _WIN32_WINNT_WINBLUE */

typedef struct _SCRUB_DATA_INPUT {

    //
    // sizeof(SCRUB_DATA_INPUT)
    //

    DWORD Size;

    //
    // Zero for the initial call.
    //
    // SCRUB_DATA_INPUT_FLAG_RESUME has to be specified when
    // ResumeContext is provided from the previous call
    //

    DWORD Flags;

    //
    // Maximum number of IOs in a single call. This is a hint to a
    // file system to halt the operation with a restart context if the
    // operation takes too long.
    //

    DWORD MaximumIos;

    //
    // 16 Byte object id. Only used if SCRUB_DATA_INPUT_FLAG_SCRUB_BY_OBJECT_ID
    // is specified via FSCTL_SCRUB_UNDISCOVERABLE_ID. Array of DWORDs to
    // preserve previous alignment.
    //

    DWORD ObjectId[4];

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_FE)
    //
    // Start scrubbing at byte offset for byte count
    // Both must be cluster aligned
    //

    ULONGLONG StartingByteOffset;

    ULONGLONG ByteCount;

    //
    // Reserved
    //

    DWORD Reserved[36];

#else

    DWORD Reserved[41];

#endif

    //
    // Opaque data returned from the previous call to restart the
    // operation. Only valid when SCRUB_DATA_FLAG_RESUME is set
    // at Flags field.  This offset needs to match that of SCRUB_DATA_OUTPUT.
    //

    BYTE  ResumeContext[1040];

} SCRUB_DATA_INPUT, *PSCRUB_DATA_INPUT;

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

typedef struct _SCRUB_PARITY_EXTENT {

    LONGLONG Offset;

    ULONGLONG Length;

} SCRUB_PARITY_EXTENT, *PSCRUB_PARITY_EXTENT;

typedef struct _SCRUB_PARITY_EXTENT_DATA {

    //
    // sizeof(SCRUB_PARITY_EXTENT_DATA)
    //

    WORD   Size;

    //
    // Reserved
    //

    WORD   Flags;

    //
    // Number of parity extents
    //

    WORD   NumberOfParityExtents;

    //
    // Maximum number of parity extents in ParityExtents buffer
    //

    WORD   MaximumNumberOfParityExtents;

    //
    // Output buffer for parity extents
    //

    SCRUB_PARITY_EXTENT ParityExtents[ANYSIZE_ARRAY];

} SCRUB_PARITY_EXTENT_DATA, *PSCRUB_PARITY_EXTENT_DATA;

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

typedef struct _SCRUB_DATA_OUTPUT {

    //
    // sizeof(SCRUB_DATA_OUTPUT)
    //

    DWORD Size;

    //
    // Output Flags
    //
    // SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE will be set if there are
    // remaining ranges. ResumeContext provided for the subsequent
    // call.
    //

    DWORD Flags;

    //
    // Operational status
    //

    DWORD Status;

    //
    // Offset of the error range of the user data where the operational errors were found.
    // This value may be -1 if the error were found in non-user data area
    //

    ULONGLONG ErrorFileOffset;

    //
    // Length of the error range of the user data where the operational errors were found.
    // This value may be 0 if the error were found in non-user data area
    //

    ULONGLONG ErrorLength;

    //
    // Number of bytes successfully repaired in the operational error range
    //

    ULONGLONG NumberOfBytesRepaired;

    //
    // Number of bytes failed due to an error in the operational error range
    //

    ULONGLONG NumberOfBytesFailed;

    //
    // Reference number for the file system specific internal file
    //

    ULONGLONG InternalFileReference;

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

    //
    // Resume context length
    //
    // Only valid if SCRUB_DATA_OUTPUT_FLAG_RESUME_CONTEXT_LENGTH_SPECIFIED
    // is specified in the Flags.
    //

    WORD   ResumeContextLength;

    //
    // Offset for the parity extent data in the output buffer
    // Only valid if SCRUB_DATA_OUTPUT_FLAG_PARITY_EXTENT_DATA_RETURNED
    // is specified in the Flags.
    //

    WORD   ParityExtentDataOffset;

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_FE)

    //
    //  Next scrub starting offset in bytes
    //

    ULONGLONG NextStartingByteOffset;

    ULONGLONG ValidDataLength;

    DWORD Reserved[4];

#else

    //
    // Reserved
    //

    DWORD Reserved[9];
#endif

#else /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

    //
    // Reserved
    //

    DWORD Reserved[10];

#endif /* (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE) */

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)

    //
    // Number of bytes of metadata processed
    //

    ULONGLONG NumberOfMetadataBytesProcessed;

    //
    // Number of bytes of data to be processed
    //

    ULONGLONG NumberOfDataBytesProcessed;

    //
    // Total number of bytes of metadata in use
    //

    ULONGLONG TotalNumberOfMetadataBytesInUse;

    //
    // Total number of bytes of data in use
    //

    ULONGLONG TotalNumberOfDataBytesInUse;

#else

    ULONGLONG Reserved2[4];

#endif

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN10_FE)

    //
    //  Number of bytes skipped due to hole, ghost, and reserved
    //

    ULONGLONG DataBytesSkippedDueToNoAllocation;

    //
    //  Number of bytes skipped due allocation that haven't been written to
    //

    ULONGLONG DataBytesSkippedDueToInvalidRun;

    //
    //  Number of bytes skipped due to Integrity stream
    //

    ULONGLONG DataBytesSkippedDueToIntegrityStream;

    //
    //  Number of bytes skipped due to region not dirty (DRT mode only)
    //

    ULONGLONG DataBytesSkippedDueToRegionBeingClean;

    //
    //  Number of bytes skipped due to lock conflict
    //

    ULONGLONG DataBytesSkippedDueToLockConflict;

    //
    //  Number of bytes skipped due to stream marked as don't scrub
    //

    ULONGLONG DataBytesSkippedDueToNoScrubDataFlag;

    //
    //  Number of bytes skipped due to non Integrity stream marked as don't scrub
    //

    ULONGLONG DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag;

    //
    //  Number of bytes actually scrubbed
    //

    ULONGLONG DataBytesScrubbed;

#else

    ULONGLONG Reserved3[8];

#endif

    //
    // Opaque data that the file system returns to the user so that
    // subsequent call can use this data to resume from the previous
    // point for the operation.
    //
    // Resume operation can be requested on a different handle and
    // across the reboot. However, file system may not honor the
    // resume context if not feasible and start from the beginning.
    //
    // This field is only valid when SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE
    // is set.  It has to be last in the structure.
    //

    BYTE  ResumeContext[1040];

} SCRUB_DATA_OUTPUT, *PSCRUB_DATA_OUTPUT;

#endif /*_WIN32_WINNT >= _WIN32_WINNT_WIN8 */

#if (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)
//
//=============== FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT ====================
//

//
// Whether the file system supports shared virtual disks.
//
typedef enum _SharedVirtualDiskSupportType
{
    //
    // Shared virtual disks are not supported.
    //
    SharedVirtualDisksUnsupported = 0,

    //
    // Shared virtual disks are supported.
    //
    SharedVirtualDisksSupported = 1,

    //
    // The target device supports taking virtual disk
    // snapshots.
    //
    SharedVirtualDiskSnapshotsSupported = 3,

    //
    // The target device supports Continuous Data
    // Protection (log based) snapshots.
    //
    SharedVirtualDiskCDPSnapshotsSupported = 7
} SharedVirtualDiskSupportType;

typedef enum _SharedVirtualDiskHandleState
{
    //
    // The file handle is not related to a shared virtual disk.
    //
    SharedVirtualDiskHandleStateNone = 0,

    //
    // This handle is for the same file where at least one handle is
    // accessing the file in shared mode.
    //
    SharedVirtualDiskHandleStateFileShared = 1,

    //
    // This handle is currently being used to access a shared
    // virtual disk.
    //
    SharedVirtualDiskHandleStateHandleShared = 3
} SharedVirtualDiskHandleState;

//
// Response to FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT that indicates the level
// of support for shared virtual disks on the target file system.
//
typedef struct _SHARED_VIRTUAL_DISK_SUPPORT {
    //
    // One or more of the above SharedVirtualDiskSupportType flags that indicate the
    // level of shared virtual disk support on this file system.
    //
    SharedVirtualDiskSupportType SharedVirtualDiskSupport;

    //
    // The state of the current shared virtual disk handle.  This is one or more of the
    // above SharedVirtualDiskHandleState flags.
    //
    SharedVirtualDiskHandleState HandleState;
} SHARED_VIRTUAL_DISK_SUPPORT, *PSHARED_VIRTUAL_DISK_SUPPORT;

//
// Determines if the provided virtual disk handle state, from FSCTL_QUERY_SHARED_VIRTUAL_DISK_SUPPORT,
// indicates that the target virtual disk file is opened in shared mode.
//
#define IsVirtualDiskFileShared(HandleState) (((HandleState) & SharedVirtualDiskHandleStateFileShared) != 0)

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WINBLUE)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) || (NTDDI_VERSION >= NTDDI_WIN8) //Win8 check is for backward compatibility.

//
//=============== FSCTL_REARRANGE_FILE ====================
//

//
//  Input structure for FSCTL_REARRANGE_FILE
//

typedef struct _REARRANGE_FILE_DATA {

    //
    // Cluster-aligned byte offset of the base of the contiguous source region
    // in the source file to move to the target location of the target file.
    //
    ULONGLONG SourceStartingOffset;

    //
    // Cluster-aligned byte offset of the point to insert the source region.
    //
    ULONGLONG TargetOffset;

    //
    // Alternate file to move the source region clusters from;
    // if NULL, the source region is from the same file.
    //
    HANDLE SourceFileHandle;

    //
    // Cluster-aligned length (in bytes) of the source region.
    //
    DWORD Length;

    //
    // Flags - reserved for future definition and must be zero.
    //
    DWORD Flags;

} REARRANGE_FILE_DATA, *PREARRANGE_FILE_DATA;

#if defined(_WIN64)
//
//  32/64 Bit thunking support structure
//

typedef struct _REARRANGE_FILE_DATA32 {

    ULONGLONG SourceStartingOffset;
    ULONGLONG TargetOffset;
    UINT32 SourceFileHandle;
    DWORD Length;
    DWORD Flags;

} REARRANGE_FILE_DATA32, *PREARRANGE_FILE_DATA32;
#endif // defined(_WIN64)

#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS5) */

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
//
//=============== FSCTL_SHUFFLE_FILE ====================
//

#define SHUFFLE_FILE_FLAG_SKIP_INITIALIZING_NEW_CLUSTERS                (0x00000001)

//
//  Input structure for FSCTL_SHUFFLE_FILE
//

typedef struct _SHUFFLE_FILE_DATA {

    LONGLONG StartingOffset;
    LONGLONG Length;
    DWORD Flags;

} SHUFFLE_FILE_DATA, *PSHUFFLE_FILE_DATA;

#endif // (_WIN32_WINNT >= _WIN32_WINNT_WIN8)


//
// I/O Completion Specific Access Rights.
//

#define IO_COMPLETION_MODIFY_STATE  0x0002  
#define IO_COMPLETION_ALL_ACCESS (STANDARD_RIGHTS_REQUIRED|SYNCHRONIZE|0x3) 

#define IO_QOS_MAX_RESERVATION 1000000000ULL


//
// Some applications include both ntioapi_x.h and winioctl.h
//

#ifndef SMB_CCF_APP_INSTANCE_EA_NAME
#define SMB_CCF_APP_INSTANCE_EA_NAME   "ClusteredApplicationInstance"
#endif //SMB_CCF_APP_INSTANCE_EA_NAME

#ifndef _NETWORK_APP_INSTANCE_EA_DEFINED
#define _NETWORK_APP_INSTANCE_EA_DEFINED

#if (NTDDI_VERSION >= NTDDI_WIN10)

//
// Define the SMB Cluster Client Failover AppInstance Extended Attribute name
// newer version of input payload assumes that EA is not just a GUID,
// but instead is a structure that contains additional information
//

//
// Is used only when file is opened directly on CSVFS. This flag is ignored when file
// is opened over SMB.
// Tells CSVFS that this file open should be valid only on coordinating node.
// If open comes to CSVFS, and this node is not a coordinating then open would fail.
// If file is opened, and coordinating node is moved then file open will be invalidated
//
#ifndef NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR
#define NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR 0x00000001
#endif //NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR

typedef struct _NETWORK_APP_INSTANCE_EA {

    //
    //  The caller places a GUID that should always be unique for a single instance of
    //  the application.
    //

    GUID AppInstanceID;

    //
    //  Combination of the NETWORK_APP_INSTANCE_CSV_FLAGS_* flags
    //

    DWORD CsvFlags;

} NETWORK_APP_INSTANCE_EA, *PNETWORK_APP_INSTANCE_EA;

#endif // (NTDDI_VERSION >= NTDDI_WIN10)

#endif //_NETWORK_APP_INSTANCE_EA_DEFINED

// begin_access

//
// Object Manager Symbolic Link Specific Access Rights.
//

#define DUPLICATE_CLOSE_SOURCE      0x00000001  
#define DUPLICATE_SAME_ACCESS       0x00000002  

//
// =========================================
// Define GUIDs which represent well-known power schemes
// =========================================
//

//
// Maximum Power Savings - indicates that very aggressive power savings measures will be used to help
//                         stretch battery life.
//
// {a1841308-3541-4fab-bc81-f71556f20b4a}
//
DEFINE_GUID( GUID_MAX_POWER_SAVINGS, 0xA1841308, 0x3541, 0x4FAB, 0xBC, 0x81, 0xF7, 0x15, 0x56, 0xF2, 0x0B, 0x4A );

//
// No Power Savings - indicates that almost no power savings measures will be used.
//
// {8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c}
//
DEFINE_GUID( GUID_MIN_POWER_SAVINGS, 0x8C5E7FDA, 0xE8BF, 0x4A96, 0x9A, 0x85, 0xA6, 0xE2, 0x3A, 0x8C, 0x63, 0x5C );

//
// Typical Power Savings - indicates that fairly aggressive power savings measures will be used.
//
// {381b4222-f694-41f0-9685-ff5bb260df2e}
//
DEFINE_GUID( GUID_TYPICAL_POWER_SAVINGS, 0x381B4222, 0xF694, 0x41F0, 0x96, 0x85, 0xFF, 0x5B, 0xB2, 0x60, 0xDF, 0x2E );

//
// This is a special GUID that represents "no subgroup" of settings.  That is, it indicates
// that settings that are in the root of the power policy hierarchy as opposed to settings
// that are buried under a subgroup of settings.  This should be used when querying for
// power settings that may not fall into a subgroup.
//
DEFINE_GUID( NO_SUBGROUP_GUID, 0xFEA3413E, 0x7E05, 0x4911, 0x9A, 0x71, 0x70, 0x03, 0x31, 0xF1, 0xC2, 0x94 );

//
// This is a special GUID that represents "every power scheme".  That is, it indicates
// that any write to this power scheme should be reflected to every scheme present.
// This allows users to write a single setting once and have it apply to all schemes.  They
// can then apply custom settings to specific power schemes that they care about.
//
DEFINE_GUID( ALL_POWERSCHEMES_GUID, 0x68A1E95E, 0x13EA, 0x41E1, 0x80, 0x11, 0x0C, 0x49, 0x6C, 0xA4, 0x90, 0xB0 );

//
// This is a special GUID that represents a 'personality' that each power scheme will have.
// In other words, each power scheme will have this key indicating "I'm most like *this* base
// power scheme."  This individual setting will have one of three settings:
// GUID_MAX_POWER_SAVINGS
// GUID_MIN_POWER_SAVINGS
// GUID_TYPICAL_POWER_SAVINGS
//
// This allows several features:
// 1. Drivers and applications can register for notification of this GUID.  So when this power
//    scheme is activiated, this GUID's setting will be sent across the system and drivers/applications
//    can see "GUID_MAX_POWER_SAVINGS" which will tell them in a generic fashion "get real aggressive
//    about conserving power".
// 2. UserB may install a driver or application which creates power settings, and UserB may modify
//    those power settings.  Now UserA logs in.  How does he see those settings?  They simply don't
//    exist in his private power key.  Well they do exist over in the system power key.  When we
//    enumerate all the power settings in this system power key and don't find a corresponding entry
//    in the user's private power key, then we can go look at this "personality" key in the users
//    power scheme.  We can then go get a default value for the power setting, depending on which
//    "personality" power scheme is being operated on.  Here's an example:
//    A. UserB installs an application that creates a power setting Seetting1
//    B. UserB changes Setting1 to have a value of 50 because that's one of the possible settings
//       available for setting1.
//    C. UserB logs out
//    D. UserA logs in and his active power scheme is some custom scheme that was derived from
//       the GUID_TYPICAL_POWER_SAVINGS.  But remember that UserA has no setting1 in his
//       private power key.
//    E. When activating UserA's selected power scheme, all power settings in the system power key will
//       be enumerated (including Setting1).
//    F. The power manager will see that UserA has no Setting1 power setting in his private power scheme.
//    G. The power manager will query UserA's power scheme for its personality and retrieve
//       GUID_TYPICAL_POWER_SAVINGS.
//    H. The power manager then looks in Setting1 in the system power key and looks in its set of default
//       values for the corresponding value for GUID_TYPICAL_POWER_SAVINGS power schemes.
//    I. This derived power setting is applied.
DEFINE_GUID( GUID_POWERSCHEME_PERSONALITY, 0x245D8541, 0x3943, 0x4422, 0xB0, 0x25, 0x13, 0xA7, 0x84, 0xF6, 0x79, 0xB7 );

//
// Define a special GUID which will be used to define the active power scheme.
// User will register for this power setting GUID, and when the active power
// scheme changes, they'll get a callback where the payload is the GUID
// representing the active powerscheme.
// ( 31F9F286-5084-42FE-B720-2B0264993763 }
//
DEFINE_GUID( GUID_ACTIVE_POWERSCHEME, 0x31F9F286, 0x5084, 0x42FE, 0xB7, 0x20, 0x2B, 0x02, 0x64, 0x99, 0x37, 0x63 );

//
// =========================================
// Define GUIDs which represent Power Modes (Overlays)
// =========================================
//

//
// Efficiency Power Mode
//
// {961cc777-2547-4f9d-8174-7d86181b8a7a}
//

DEFINE_GUID( GUID_POWER_MODE_BEST_EFFICIENCY, 0x961cc777, 0x2547, 0x4f9d, 0x81, 0x74, 0x7d, 0x86, 0x18, 0x1b, 0x8a, 0x7a );

//
// Default Power Mode
//
// {00000000-0000-0000-0000-000000000000}
//

DEFINE_GUID( GUID_POWER_MODE_NONE, 0L, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 );

//
// Deprecated Performance Power Mode
//
// {3af9B8d9-7c97-431d-ad78-34a8bfea439f}
//

DEFINE_GUID( GUID_POWER_MODE_PERFORMANCE, 0x3af9b8d9, 0x7c97, 0x431d, 0xad, 0x78, 0x34, 0xa8, 0xbf, 0xea, 0x43, 0x9f );

//
// Best Performance Power Mode
//
// {ded574b5-45a0-4f42-8737-46345c09c238}
//

DEFINE_GUID( GUID_POWER_MODE_BEST_PERFORMANCE, 0xded574b5, 0x45a0, 0x4f42, 0x87, 0x37, 0x46, 0x34, 0x5c, 0x9, 0xc2, 0x38 );

//
// =========================================
// Define GUIDs which represent well-known power settings
// =========================================
//

// Idle resiliency settings
// -------------------------
//
// Specifies the subgroup which will contain all of the idle resiliency
// settings for a single policy.
//
// {2E601130-5351-4d9d-8E04-252966BAD054}
DEFINE_GUID(GUID_IDLE_RESILIENCY_SUBGROUP, 0x2e601130, 0x5351, 0x4d9d, 0x8e, 0x4, 0x25, 0x29, 0x66, 0xba, 0xd0, 0x54);

//
// Specifies the maximum clock interrupt period (in ms)
//
// N.B. This power setting is DEPRECATED.
//
// {C42B79AA-AA3A-484b-A98F-2CF32AA90A28}
DEFINE_GUID(GUID_IDLE_RESILIENCY_PERIOD, 0xc42b79aa, 0xaa3a, 0x484b, 0xa9, 0x8f, 0x2c, 0xf3, 0x2a, 0xa9, 0xa, 0x28);

//
// Specifies the deep sleep policy setting.
// This is intended to override the GUID_IDLE_RESILIENCY_PERIOD
// {d502f7ee-1dc7-4efd-a55d-f04b6f5c0545}
DEFINE_GUID(GUID_DEEP_SLEEP_ENABLED, 0xd502f7ee, 0x1dc7, 0x4efd, 0xa5, 0x5d, 0xf0, 0x4b, 0x6f, 0x5c, 0x5, 0x45);

//
// Specifies the platform idle state index associated with idle resiliency
// period.
//
// N.B. This power setting is DEPRECATED.
//
// {D23F2FB8-9536-4038-9C94-1CE02E5C2152}
DEFINE_GUID(GUID_DEEP_SLEEP_PLATFORM_STATE, 0xd23f2fb8, 0x9536, 0x4038, 0x9c, 0x94, 0x1c, 0xe0, 0x2e, 0x5c, 0x21, 0x52);

//
// Specifies (in milliseconds) how long we wait after the last disk access
// before we power off the disk in case when IO coalescing is active.
//
// {C36F0EB4-2988-4a70-8EEE-0884FC2C2433}
DEFINE_GUID(GUID_DISK_COALESCING_POWERDOWN_TIMEOUT, 0xc36f0eb4, 0x2988, 0x4a70, 0x8e, 0xee, 0x8, 0x84, 0xfc, 0x2c, 0x24, 0x33);

//
// Specifies (in seconds) how long we wait after the CS Enter before
// we deactivate execution required request.
//
//   0 : implies execution power requests are disabled and have no effect
//  -1 : implies execution power requests are never deactivated
//
// Note: Execution required power requests are mapped into system required
//      power requests on non-AoAc machines and this value has no effect.
//
// {3166BC41-7E98-4e03-B34E-EC0F5F2B218E}
DEFINE_GUID(GUID_EXECUTION_REQUIRED_REQUEST_TIMEOUT, 0x3166bc41, 0x7e98, 0x4e03, 0xb3, 0x4e, 0xec, 0xf, 0x5f, 0x2b, 0x21, 0x8e);


// Video settings
// --------------
//
// Specifies the subgroup which will contain all of the video
// settings for a single policy.
//
// {7516b95f-f776-4464-8c53-06167f40cc99}
//
DEFINE_GUID( GUID_VIDEO_SUBGROUP, 0x7516B95F, 0xF776, 0x4464, 0x8C, 0x53, 0x06, 0x16, 0x7F, 0x40, 0xCC, 0x99 );

//
// Specifies (in seconds) how long we wait after the last user input has been
// received before we power off the video.
//
// {3c0bc021-c8a8-4e07-a973-6b14cbcb2b7e}
//
DEFINE_GUID( GUID_VIDEO_POWERDOWN_TIMEOUT, 0x3C0BC021, 0xC8A8, 0x4E07, 0xA9, 0x73, 0x6B, 0x14, 0xCB, 0xCB, 0x2B, 0x7E );

//
// Specifies whether adaptive display dimming is turned on or off.
//
// N.B. This setting is DEPRECATED in Windows 8.1
//
// {82DBCF2D-CD67-40C5-BFDC-9F1A5CCD4663}
//
DEFINE_GUID( GUID_VIDEO_ANNOYANCE_TIMEOUT, 0x82DBCF2D, 0xCD67, 0x40C5, 0xBF, 0xDC, 0x9F, 0x1A, 0x5C, 0xCD, 0x46, 0x63 );

//
// Specifies how much adaptive dim time out will be increased by.
//
// N.B. This setting is DEPRECATED in Windows 8.1
//
// {EED904DF-B142-4183-B10B-5A1197A37864}
//
DEFINE_GUID( GUID_VIDEO_ADAPTIVE_PERCENT_INCREASE, 0xEED904DF, 0xB142, 0x4183, 0xB1, 0x0B, 0x5A, 0x11, 0x97, 0xA3, 0x78, 0x64 );

//
// Specifies (in seconds) how long we wait after the last user input has been
// received before we dim the video.
//
// {17aaa29b-8b43-4b94-aafe-35f64daaf1ee}
//
DEFINE_GUID( GUID_VIDEO_DIM_TIMEOUT, 0x17aaa29b, 0x8b43, 0x4b94, 0xaa, 0xfe, 0x35, 0xf6, 0x4d, 0xaa, 0xf1, 0xee);

//
// Specifies if the operating system should use adaptive timers (based on
// previous behavior) to power down the video.
//
// {90959d22-d6a1-49b9-af93-bce885ad335b}
//
DEFINE_GUID( GUID_VIDEO_ADAPTIVE_POWERDOWN, 0x90959D22, 0xD6A1, 0x49B9, 0xAF, 0x93, 0xBC, 0xE8, 0x85, 0xAD, 0x33, 0x5B );

//
// Specifies if the monitor is currently being powered or not.
//
// {02731015-4510-4526-99E6-E5A17EBD1AEA}
//
DEFINE_GUID( GUID_MONITOR_POWER_ON, 0x02731015, 0x4510, 0x4526, 0x99, 0xE6, 0xE5, 0xA1, 0x7E, 0xBD, 0x1A, 0xEA );

//
// Monitor brightness policy when in normal state.
//
// {aded5e82-b909-4619-9949-f5d71dac0bcb}
//
DEFINE_GUID(GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS, 0xaded5e82L, 0xb909, 0x4619, 0x99, 0x49, 0xf5, 0xd7, 0x1d, 0xac, 0x0b, 0xcb);

//
// Monitor brightness policy when in dim state.
//
// {f1fbfde2-a960-4165-9f88-50667911ce96}
//
DEFINE_GUID(GUID_DEVICE_POWER_POLICY_VIDEO_DIM_BRIGHTNESS, 0xf1fbfde2, 0xa960, 0x4165, 0x9f, 0x88, 0x50, 0x66, 0x79, 0x11, 0xce, 0x96);

//
// Current monitor brightness.
//
// {8ffee2c6-2d01-46be-adb9-398addc5b4ff}
//
DEFINE_GUID(GUID_VIDEO_CURRENT_MONITOR_BRIGHTNESS, 0x8ffee2c6, 0x2d01, 0x46be, 0xad, 0xb9, 0x39, 0x8a, 0xdd, 0xc5, 0xb4, 0xff);

//
// Specifies if the operating system should use ambient light sensor to change
// adaptively the display's brightness.
//
// {FBD9AA66-9553-4097-BA44-ED6E9D65EAB8}
//
DEFINE_GUID(GUID_VIDEO_ADAPTIVE_DISPLAY_BRIGHTNESS, 0xFBD9AA66, 0x9553, 0x4097, 0xBA, 0x44, 0xED, 0x6E, 0x9D, 0x65, 0xEA, 0xB8);

//
// Specifies a change in the current monitor's display state.
//
// {6fe69556-704a-47a0-8f24-c28d936fda47}
//
DEFINE_GUID(GUID_CONSOLE_DISPLAY_STATE, 0x6fe69556, 0x704a, 0x47a0, 0x8f, 0x24, 0xc2, 0x8d, 0x93, 0x6f, 0xda, 0x47);

//
// Defines a guid for enabling/disabling the ability to create display required
// power requests.
//
// {A9CEB8DA-CD46-44FB-A98B-02AF69DE4623}
//
DEFINE_GUID( GUID_ALLOW_DISPLAY_REQUIRED, 0xA9CEB8DA, 0xCD46, 0x44FB, 0xA9, 0x8B, 0x02, 0xAF, 0x69, 0xDE, 0x46, 0x23 );

//
// Specifies the video power down timeout (in seconds) after the interactive
// console is locked (and sensors indicate UserNotPresent). Value 0
// effectively disables this feature.
//
// {8EC4B3A5-6868-48c2-BE75-4F3044BE88A7}
//
DEFINE_GUID(GUID_VIDEO_CONSOLE_LOCK_TIMEOUT, 0x8ec4b3a5, 0x6868, 0x48c2, 0xbe, 0x75, 0x4f, 0x30, 0x44, 0xbe, 0x88, 0xa7);

//
// Specifies power settings which will decide whether to
// prefer visual quality or battery life for an Advanced
// Color capable display
//
// {684C3E69-A4F7-4014-8754-D45179A56167}
//
DEFINE_GUID(GUID_ADVANCED_COLOR_QUALITY_BIAS, 0x684c3e69, 0xa4f7, 0x4014, 0x87, 0x54, 0xd4, 0x51, 0x79, 0xa5, 0x61, 0x67);


// Adaptive power behavior settings
// --------------------------------
//
// {8619B916-E004-4dd8-9B66-DAE86F806698}
DEFINE_GUID(GUID_ADAPTIVE_POWER_BEHAVIOR_SUBGROUP, 0x8619b916, 0xe004, 0x4dd8, 0x9b, 0x66, 0xda, 0xe8, 0x6f, 0x80, 0x66, 0x98);

//
// Specifies the input timeout (in seconds) to be used to indicate UserUnkown.
// Value 0 effectively disables this feature.
//
// {5ADBBFBC-074E-4da1-BA38-DB8B36B2C8F3}
DEFINE_GUID(GUID_NON_ADAPTIVE_INPUT_TIMEOUT, 0x5adbbfbc, 0x74e, 0x4da1, 0xba, 0x38, 0xdb, 0x8b, 0x36, 0xb2, 0xc8, 0xf3);

//
// Specifies a change in the input controller(s) global system's state:
// e.g. enabled, suppressed, filtered.
//
// {0E98FAE9-F45A-4DE1-A757-6031F197F6EA}
DEFINE_GUID(GUID_ADAPTIVE_INPUT_CONTROLLER_STATE, 0xe98fae9, 0xf45a, 0x4de1, 0xa7, 0x57, 0x60, 0x31, 0xf1, 0x97, 0xf6, 0xea);

// Harddisk settings
// -----------------
//
// Specifies the subgroup which will contain all of the harddisk
// settings for a single policy.
//
DEFINE_GUID( GUID_DISK_SUBGROUP, 0x0012EE47, 0x9041, 0x4B5D, 0x9B, 0x77, 0x53, 0x5F, 0xBA, 0x8B, 0x14, 0x42 );

//
// Specifies a maximum power consumption level.
//
DEFINE_GUID(GUID_DISK_MAX_POWER, 0x51dea550, 0xbb38, 0x4bc4, 0x99, 0x1b, 0xea, 0xcf, 0x37, 0xbe, 0x5e, 0xc8);

//
// Specifies (in seconds) how long we wait after the last disk access
// before we power off the disk.
//
DEFINE_GUID( GUID_DISK_POWERDOWN_TIMEOUT, 0x6738E2C4, 0xE8A5, 0x4A42, 0xB1, 0x6A, 0xE0, 0x40, 0xE7, 0x69, 0x75, 0x6E );

//
// Specifies (in milliseconds) how long we wait after the last disk access
// before we power off the disk taking into account if IO coalescing is active.
//
// {58E39BA8-B8E6-4EF6-90D0-89AE32B258D6}
DEFINE_GUID( GUID_DISK_IDLE_TIMEOUT, 0x58E39BA8, 0xB8E6, 0x4EF6, 0x90, 0xD0, 0x89, 0xAE, 0x32, 0xB2, 0x58, 0xD6 );

//
// Specifies the amount of contiguous disk activity time to ignore when
// calculating disk idleness.
//
// 80e3c60e-bb94-4ad8-bbe0-0d3195efc663
//

DEFINE_GUID( GUID_DISK_BURST_IGNORE_THRESHOLD, 0x80e3c60e, 0xbb94, 0x4ad8, 0xbb, 0xe0, 0x0d, 0x31, 0x95, 0xef, 0xc6, 0x63 );

//
// Specifies if the operating system should use adaptive timers (based on
// previous behavior) to power down the disk,
//
DEFINE_GUID( GUID_DISK_ADAPTIVE_POWERDOWN, 0x396A32E1, 0x499A, 0x40B2, 0x91, 0x24, 0xA9, 0x6A, 0xFE, 0x70, 0x76, 0x67 );

//
// Specifies whether NVMe non-operational power state permissive mode is enabled.
//
DEFINE_GUID(GUID_DISK_NVME_NOPPME, 0xfc7372b6, 0xab2d, 0x43ee, 0x87, 0x97, 0x15, 0xe9, 0x84, 0x1f, 0x2c, 0xca);

// System sleep settings
// ---------------------
//
// Specifies the subgroup which will contain all of the sleep
// settings for a single policy.
// { 238C9FA8-0AAD-41ED-83F4-97BE242C8F20 }
//
DEFINE_GUID( GUID_SLEEP_SUBGROUP, 0x238C9FA8, 0x0AAD, 0x41ED, 0x83, 0xF4, 0x97, 0xBE, 0x24, 0x2C, 0x8F, 0x20 );

//
// Specifies an idle treshold percentage (0-100). The system must be this idle
// over a period of time in order to idle to sleep.
//
// N.B. DEPRECATED IN WINDOWS 6.1
//
DEFINE_GUID( GUID_SLEEP_IDLE_THRESHOLD, 0x81cd32e0, 0x7833, 0x44f3, 0x87, 0x37, 0x70, 0x81, 0xf3, 0x8d, 0x1f, 0x70 );

//
// Specifies (in seconds) how long we wait after the system is deemed
// "idle" before moving to standby (S1, S2 or S3).
//
DEFINE_GUID( GUID_STANDBY_TIMEOUT, 0x29F6C1DB, 0x86DA, 0x48C5, 0x9F, 0xDB, 0xF2, 0xB6, 0x7B, 0x1F, 0x44, 0xDA );

//
// Specifies (in seconds) how long the system should go back to sleep after
// waking unattended. 0 indicates that the standard standby/hibernate idle
// policy should be used instead.
//
// {7bc4a2f9-d8fc-4469-b07b-33eb785aaca0}
//
DEFINE_GUID( GUID_UNATTEND_SLEEP_TIMEOUT, 0x7bc4a2f9, 0xd8fc, 0x4469, 0xb0, 0x7b, 0x33, 0xeb, 0x78, 0x5a, 0xac, 0xa0 );

//
// Specifies (in seconds) how long we wait after the system is deemed
// "idle" before moving to hibernate (S4).
//
DEFINE_GUID( GUID_HIBERNATE_TIMEOUT, 0x9D7815A6, 0x7EE4, 0x497E, 0x88, 0x88, 0x51, 0x5A, 0x05, 0xF0, 0x23, 0x64 );

//
// Specifies whether or not Fast S4 should be enabled if the system supports it
// 94AC6D29-73CE-41A6-809F-6363BA21B47E
//
DEFINE_GUID( GUID_HIBERNATE_FASTS4_POLICY, 0x94AC6D29, 0x73CE, 0x41A6, 0x80, 0x9F, 0x63, 0x63, 0xBA, 0x21, 0xB4, 0x7E );

//
// Define a GUID for controlling the criticality of sleep state transitions.
// Critical sleep transitions do not query applications, services or drivers
// before transitioning the platform to a sleep state.
//
// {B7A27025-E569-46c2-A504-2B96CAD225A1}
//
DEFINE_GUID( GUID_CRITICAL_POWER_TRANSITION,  0xB7A27025, 0xE569, 0x46c2, 0xA5, 0x04, 0x2B, 0x96, 0xCA, 0xD2, 0x25, 0xA1);

//
// Specifies if the system is entering or exiting 'away mode'.
// 98A7F580-01F7-48AA-9C0F-44352C29E5C0
//
DEFINE_GUID( GUID_SYSTEM_AWAYMODE, 0x98A7F580, 0x01F7, 0x48AA, 0x9C, 0x0F, 0x44, 0x35, 0x2C, 0x29, 0xE5, 0xC0 );

//
// Specify whether away mode is allowed
//
// {25DFA149-5DD1-4736-B5AB-E8A37B5B8187}
//
DEFINE_GUID( GUID_ALLOW_AWAYMODE, 0x25dfa149, 0x5dd1, 0x4736, 0xb5, 0xab, 0xe8, 0xa3, 0x7b, 0x5b, 0x81, 0x87 );

//
// Defines a guid to control User Presence Prediction mode.
//
// {82011705-FB95-4D46-8D35-4042B1D20DEF}
//
DEFINE_GUID( GUID_USER_PRESENCE_PREDICTION, 0x82011705, 0xfb95, 0x4d46, 0x8d, 0x35, 0x40, 0x42, 0xb1, 0xd2, 0xd, 0xef );

//
// Defines a guid to control Standby Budget Grace Period.
//
// {60C07FE1-0556-45CF-9903-D56E32210242}
//
DEFINE_GUID( GUID_STANDBY_BUDGET_GRACE_PERIOD, 0x60c07fe1, 0x0556, 0x45cf, 0x99, 0x03, 0xd5, 0x6e, 0x32, 0x21, 0x2, 0x42 );

//
// Defines a guid to control Standby Budget Percent.
//
// {9FE527BE-1B70-48DA-930D-7BCF17B44990}
//
DEFINE_GUID( GUID_STANDBY_BUDGET_PERCENT, 0x9fe527be, 0x1b70, 0x48da, 0x93, 0x0d, 0x7b, 0xcf, 0x17, 0xb4, 0x49, 0x90 );

//
// Defines a guid the control the number of standby budget refreshes.
//
// {ACA8648E-C4B1-4BAA-8CCE-9390AD647F8C}
//
DEFINE_GUID( GUID_STANDBY_BUDGET_REFRESH_COUNT, 0xACA8648E, 0xC4B1, 0x4BAA, 0x8C, 0xCE, 0x93, 0x90, 0xAD, 0x64, 0x7F, 0x8C );

//
// Defines a guid the control the interval between standby budget refreshes.
//
// {61F45DFE-1919-4180-BB46-8CC70E0B38F1}
//
DEFINE_GUID( GUID_STANDBY_BUDGET_REFRESH_INTERVAL, 0x61F45DFE, 0x1919, 0x4180, 0xBB, 0x46, 0x8C, 0xC7, 0x0E, 0x0B, 0x38, 0xF1 );

//
//
// Defines a guid to control Standby Reserve Grace Period.
//
// {C763EE92-71E8-4127-84EB-F6ED043A3E3D}
//
DEFINE_GUID( GUID_STANDBY_RESERVE_GRACE_PERIOD, 0xc763ee92, 0x71e8, 0x4127, 0x84, 0xeb, 0xf6, 0xed, 0x04, 0x3a, 0x3e, 0x3d );

//
// Defines a guid to control Standby Reserve Time.
//
// {468FE7E5-1158-46EC-88BC-5B96C9E44FD0}
//
DEFINE_GUID( GUID_STANDBY_RESERVE_TIME, 0x468FE7E5, 0x1158, 0x46EC, 0x88, 0xbc, 0x5b, 0x96, 0xc9, 0xe4, 0x4f, 0xd0 );

//
// Defines a guid to control Standby Reset Percentage.
//
// {49CB11A5-56E2-4AFB-9D38-3DF47872E21B}
//
DEFINE_GUID(GUID_STANDBY_RESET_PERCENT, 0x49cb11a5, 0x56e2, 0x4afb, 0x9d, 0x38, 0x3d, 0xf4, 0x78, 0x72, 0xe2, 0x1b);

//
// Defines a guid to control Human Presence Sensor Adaptive Away Display Timeout.
//
// {0A7D6AB6-AC83-4AD1-8282-ECA5B58308F3}
//
DEFINE_GUID(GUID_HUPR_ADAPTIVE_AWAY_DISPLAY_TIMEOUT, 0x0A7D6AB6, 0xAC83, 0x4AD1, 0x82, 0x82, 0xEC, 0xA5, 0xB5, 0x83, 0x08, 0xF3);

#define GUID_HUPR_ADAPTIVE_DISPLAY_TIMEOUT GUID_HUPR_ADAPTIVE_AWAY_DISPLAY_TIMEOUT

//
// Defines a guid to control Human Presence Sensor Adaptive Inattentive Dim Timeout;
//
// {CF8C6097-12B8-4279-BBDD-44601EE5209D}
//

DEFINE_GUID(GUID_HUPR_ADAPTIVE_INATTENTIVE_DIM_TIMEOUT, 0xCF8C6097, 0x12B8, 0x4279, 0xBB, 0xDD, 0x44, 0x60, 0x1E, 0xE5, 0x20, 0x9D);

#define GUID_HUPR_ADAPTIVE_DIM_TIMEOUT GUID_HUPR_ADAPTIVE_INATTENTIVE_DIM_TIMEOUT

//
// Defines a guid to control Human Presence Sensor Adaptive Inattentive Display Timeout.
//
// {EE16691E-6AB3-4619-BB48-1C77C9357E5A}
//
DEFINE_GUID(GUID_HUPR_ADAPTIVE_INATTENTIVE_DISPLAY_TIMEOUT, 0xEE16691E, 0x6AB3, 0x4619, 0xBB, 0x48, 0x1C, 0x77, 0xC9, 0x35, 0x7E, 0x5A);

//
// Defines a guid to control Human Presence Sensor Adaptive Away Dim Timeout;
//
// {A79C8E0E-F271-482D-8F8A-5DB9A18312DE}
//

DEFINE_GUID(GUID_HUPR_ADAPTIVE_AWAY_DIM_TIMEOUT, 0xA79C8E0E, 0xF271, 0x482D, 0x8F, 0x8A, 0x5D, 0xB9, 0xA1, 0x83, 0x12, 0xDE);

//
// Defines a guid for enabling/disabling standby (S1-S3) states. This does not
// affect hibernation (S4).
//
// {abfc2519-3608-4c2a-94ea-171b0ed546ab}
//
DEFINE_GUID( GUID_ALLOW_STANDBY_STATES, 0xabfc2519, 0x3608, 0x4c2a, 0x94, 0xea, 0x17, 0x1b, 0x0e, 0xd5, 0x46, 0xab );

//
// Defines a guid for enabling/disabling the ability to wake via RTC.
//
// {BD3B718A-0680-4D9D-8AB2-E1D2B4AC806D}
//
DEFINE_GUID( GUID_ALLOW_RTC_WAKE, 0xBD3B718A, 0x0680, 0x4D9D, 0x8A, 0xB2, 0xE1, 0xD2, 0xB4, 0xAC, 0x80, 0x6D );

//
// Defines a guid for enabling/disabling legacy RTC mitigations.
//
// {1A34BDC3-7E6B-442E-A9D0-64B6EF378E84}
//
DEFINE_GUID( GUID_LEGACY_RTC_MITIGATION, 0x1A34BDC3, 0x7E6B, 0x442E, 0xA9, 0xD0, 0x64, 0xB6, 0xEF, 0x37, 0x8E, 0x84 );

//
// Defines a guid for enabling/disabling the ability to create system required
// power requests.
//
// {A4B195F5-8225-47D8-8012-9D41369786E2}
//
DEFINE_GUID( GUID_ALLOW_SYSTEM_REQUIRED, 0xA4B195F5, 0x8225, 0x47D8, 0x80, 0x12, 0x9D, 0x41, 0x36, 0x97, 0x86, 0xE2 );

// Energy Saver Settings (deprecated in Germanium)
// ---------------------
//
// ***Use GUID_ENERGY_SAVER_STATUS instead*** This power setting represents the
// state for battery saver and remains here for backwards compatibility.
//
// Indicates if Energy Saver is ON or OFF.
//
// {E00958C0-C213-4ACE-AC77-FECCED2EEEA5}
//
DEFINE_GUID( GUID_POWER_SAVING_STATUS, 0xe00958c0, 0xc213, 0x4ace, 0xac, 0x77, 0xfe, 0xcc, 0xed, 0x2e, 0xee, 0xa5);

//
// Energy Saver Settings
// ---------------------
//
// Defines a guid to indicate energy saver status (Off, Standard, or High Savings)
// from the ENERGY_SAVER_STATUS structure.
//
// This power setting is used instead of GUID_POWER_SAVING_STATUS starting
// in Germanium.
//
// {550E8400-E29B-41D4-A716-446655440000}
//
DEFINE_GUID( GUID_ENERGY_SAVER_STATUS, 0x550e8400, 0xe29b, 0x41d4, 0xa7, 0x16, 0x44, 0x66, 0x55, 0x44, 0x00, 0x00);

//
// Specifies the subgroup which will contain all of the Energy Saver settings
// for a single policy.
//
// {DE830923-A562-41AF-A086-E3A2C6BAD2DA}
//
DEFINE_GUID( GUID_ENERGY_SAVER_SUBGROUP, 0xDE830923, 0xA562, 0x41AF, 0xA0, 0x86, 0xE3, 0xA2, 0xC6, 0xBA, 0xD2, 0xDA );

//
// Defines a guid to engage Energy Saver at specific battery charge level
//
// {E69653CA-CF7F-4F05-AA73-CB833FA90AD4}
//
DEFINE_GUID( GUID_ENERGY_SAVER_BATTERY_THRESHOLD, 0xE69653CA, 0xCF7F, 0x4F05, 0xAA, 0x73, 0xCB, 0x83, 0x3F, 0xA9, 0x0A, 0xD4 );

//
// Defines a guid to specify display brightness weight when Energy Saver is engaged
//
// {13D09884-F74E-474A-A852-B6BDE8AD03A8}
//
DEFINE_GUID( GUID_ENERGY_SAVER_BRIGHTNESS, 0x13D09884, 0xF74E, 0x474A, 0xA8, 0x52, 0xB6, 0xBD, 0xE8, 0xAD, 0x03, 0xA8 );

//
// Defines a guid to specify the Energy Saver policy
//
// {5C5BB349-AD29-4ee2-9D0B-2B25270F7A81}
//
DEFINE_GUID( GUID_ENERGY_SAVER_POLICY, 0x5c5bb349, 0xad29, 0x4ee2, 0x9d, 0xb, 0x2b, 0x25, 0x27, 0xf, 0x7a, 0x81 );

// System button actions
// ---------------------
//
//
// Specifies the subgroup which will contain all of the system button
// settings for a single policy.
//
DEFINE_GUID( GUID_SYSTEM_BUTTON_SUBGROUP, 0x4F971E89, 0xEEBD, 0x4455, 0xA8, 0xDE, 0x9E, 0x59, 0x04, 0x0E, 0x73, 0x47 );

#define POWERBUTTON_ACTION_INDEX_NOTHING                0
#define POWERBUTTON_ACTION_INDEX_SLEEP                  1
#define POWERBUTTON_ACTION_INDEX_HIBERNATE              2
#define POWERBUTTON_ACTION_INDEX_SHUTDOWN               3
#define POWERBUTTON_ACTION_INDEX_TURN_OFF_THE_DISPLAY   4

//
// System button values which contain the PowerAction* value for each action.
//

#define POWERBUTTON_ACTION_VALUE_NOTHING                0
#define POWERBUTTON_ACTION_VALUE_SLEEP                  2
#define POWERBUTTON_ACTION_VALUE_HIBERNATE              3
#define POWERBUTTON_ACTION_VALUE_SHUTDOWN               6
#define POWERBUTTON_ACTION_VALUE_TURN_OFF_THE_DISPLAY   8

// Specifies (in a POWER_ACTION_POLICY structure) the appropriate action to
// take when the system power button is pressed.
//
DEFINE_GUID( GUID_POWERBUTTON_ACTION, 0x7648EFA3, 0xDD9C, 0x4E3E, 0xB5, 0x66, 0x50, 0xF9, 0x29, 0x38, 0x62, 0x80 );

//
// Specifies (in a POWER_ACTION_POLICY structure) the appropriate action to
// take when the system sleep button is pressed.
//
DEFINE_GUID( GUID_SLEEPBUTTON_ACTION, 0x96996BC0, 0xAD50, 0x47EC, 0x92, 0x3B, 0x6F, 0x41, 0x87, 0x4D, 0xD9, 0xEB );

//
// Specifies (in a POWER_ACTION_POLICY structure) the appropriate action to
// take when the system sleep button is pressed.
// { A7066653-8D6C-40A8-910E-A1F54B84C7E5 }
//
DEFINE_GUID( GUID_USERINTERFACEBUTTON_ACTION, 0xA7066653, 0x8D6C, 0x40A8, 0x91, 0x0E, 0xA1, 0xF5, 0x4B, 0x84, 0xC7, 0xE5 );

//
// Specifies (in a POWER_ACTION_POLICY structure) the appropriate action to
// take when the system lid is closed.
//
DEFINE_GUID( GUID_LIDCLOSE_ACTION, 0x5CA83367, 0x6E45, 0x459F, 0xA2, 0x7B, 0x47, 0x6B, 0x1D, 0x01, 0xC9, 0x36 );
DEFINE_GUID( GUID_LIDOPEN_POWERSTATE, 0x99FF10E7, 0x23B1, 0x4C07, 0xA9, 0xD1, 0x5C, 0x32, 0x06, 0xD7, 0x41, 0xB4 );


// Battery Discharge Settings
// --------------------------
//
// Specifies the subgroup which will contain all of the battery discharge
// settings for a single policy.
//
DEFINE_GUID( GUID_BATTERY_SUBGROUP, 0xE73A048D, 0xBF27, 0x4F12, 0x97, 0x31, 0x8B, 0x20, 0x76, 0xE8, 0x89, 0x1F );

//
// 4 battery discharge alarm settings.
//
// GUID_BATTERY_DISCHARGE_ACTION_x - This is the action to take.  It is a value
//                                   of type POWER_ACTION
// GUID_BATTERY_DISCHARGE_LEVEL_x  - This is the battery level (%)
// GUID_BATTERY_DISCHARGE_FLAGS_x  - Flags defined below:
//                                   POWER_ACTION_POLICY->EventCode flags
//                                   BATTERY_DISCHARGE_FLAGS_EVENTCODE_MASK
//                                   BATTERY_DISCHARGE_FLAGS_ENABLE
DEFINE_GUID( GUID_BATTERY_DISCHARGE_ACTION_0, 0x637EA02F, 0xBBCB, 0x4015, 0x8E, 0x2C, 0xA1, 0xC7, 0xB9, 0xC0, 0xB5, 0x46 );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_LEVEL_0, 0x9A66D8D7, 0x4FF7, 0x4EF9, 0xB5, 0xA2, 0x5A, 0x32, 0x6C, 0xA2, 0xA4, 0x69 );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_FLAGS_0, 0x5dbb7c9f, 0x38e9, 0x40d2, 0x97, 0x49, 0x4f, 0x8a, 0x0e, 0x9f, 0x64, 0x0f );

DEFINE_GUID( GUID_BATTERY_DISCHARGE_ACTION_1, 0xD8742DCB, 0x3E6A, 0x4B3C, 0xB3, 0xFE, 0x37, 0x46, 0x23, 0xCD, 0xCF, 0x06 );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_LEVEL_1, 0x8183BA9A, 0xE910, 0x48DA, 0x87, 0x69, 0x14, 0xAE, 0x6D, 0xC1, 0x17, 0x0A );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_FLAGS_1, 0xbcded951, 0x187b, 0x4d05, 0xbc, 0xcc, 0xf7, 0xe5, 0x19, 0x60, 0xc2, 0x58 );

DEFINE_GUID( GUID_BATTERY_DISCHARGE_ACTION_2, 0x421CBA38, 0x1A8E, 0x4881, 0xAC, 0x89, 0xE3, 0x3A, 0x8B, 0x04, 0xEC, 0xE4 );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_LEVEL_2, 0x07A07CA2, 0xADAF, 0x40D7, 0xB0, 0x77, 0x53, 0x3A, 0xAD, 0xED, 0x1B, 0xFA );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_FLAGS_2, 0x7fd2f0c4, 0xfeb7, 0x4da3, 0x81, 0x17, 0xe3, 0xfb, 0xed, 0xc4, 0x65, 0x82 );

DEFINE_GUID( GUID_BATTERY_DISCHARGE_ACTION_3, 0x80472613, 0x9780, 0x455E, 0xB3, 0x08, 0x72, 0xD3, 0x00, 0x3C, 0xF2, 0xF8 );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_LEVEL_3, 0x58AFD5A6, 0xC2DD, 0x47D2, 0x9F, 0xBF, 0xEF, 0x70, 0xCC, 0x5C, 0x59, 0x65 );
DEFINE_GUID( GUID_BATTERY_DISCHARGE_FLAGS_3, 0x73613ccf, 0xdbfa, 0x4279, 0x83, 0x56, 0x49, 0x35, 0xf6, 0xbf, 0x62, 0xf3 );

// Processor power settings
// ------------------------
//

// Specifies the subgroup which will contain all of the processor
// settings for a single policy.
//
// {54533251-82be-4824-96c1-47b60b740d00}
//
DEFINE_GUID( GUID_PROCESSOR_SETTINGS_SUBGROUP, 0x54533251, 0x82BE, 0x4824, 0x96, 0xC1, 0x47, 0xB6, 0x0B, 0x74, 0x0D, 0x00 );

//
// Specifies various attributes that control processor performance/throttle
// states.
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_POLICY, 0x57027304, 0x4AF6, 0x4104, 0x92, 0x60, 0xE3, 0xD9, 0x52, 0x48, 0xFC, 0x36 );

#define PERFSTATE_POLICY_CHANGE_IDEAL  0
#define PERFSTATE_POLICY_CHANGE_SINGLE 1
#define PERFSTATE_POLICY_CHANGE_ROCKET 2
#define PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE 3

#define PERFSTATE_POLICY_CHANGE_DECREASE_MAX PERFSTATE_POLICY_CHANGE_ROCKET
#define PERFSTATE_POLICY_CHANGE_INCREASE_MAX PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE

//
// Specifies a percentage (between 0 and 100) that the processor frequency
// should never go above.  For example, if this value is set to 80, then
// the processor frequency will never be throttled above 80 percent of its
// maximum frequency by the system.
//
// {bc5038f7-23e0-4960-96da-33abaf5935ec}
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_MAXIMUM, 0xBC5038F7, 0x23E0, 0x4960, 0x96, 0xDA, 0x33, 0xAB, 0xAF, 0x59, 0x35, 0xEC );

//
// Specifies a percentage (between 0 and 100) that the processor frequency
// should never go above for Processor Power Efficiency Class 1.
// For example, if this value is set to 80, then the processor frequency will
// never be throttled above 80 percent of its maximum frequency by the system.
//
// {bc5038f7-23e0-4960-96da-33abaf5935ed}
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_MAXIMUM_1, 0xBC5038F7, 0x23E0, 0x4960, 0x96, 0xDA, 0x33, 0xAB, 0xAF, 0x59, 0x35, 0xED );

//
// Specifies a percentage (between 0 and 100) that the processor frequency
// should never go above for Processor Power Efficiency Class 2.
//
// {bc5038f7-23e0-4960-96da-33abaf5935ee}
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_MAXIMUM_2, 0xBC5038F7, 0x23E0, 0x4960, 0x96, 0xDA, 0x33, 0xAB, 0xAF, 0x59, 0x35, 0xEE );

//
// Specifies a percentage (between 0 and 100) that the processor frequency
// should not drop below.  For example, if this value is set to 50, then the
// processor frequency will never be throttled below 50 percent of its
// maximum frequency by the system.
//
// {893dee8e-2bef-41e0-89c6-b55d0929964c}
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_MINIMUM, 0x893DEE8E, 0x2BEF, 0x41E0, 0x89, 0xC6, 0xB5, 0x5D, 0x09, 0x29, 0x96, 0x4C );

//
// Specifies a percentage (between 0 and 100) that the processor frequency
// should not drop below for Processor Power Efficiency Class 1.
// For example, if this value is set to 50, then the processor frequency will
// never be throttled below 50 percent of its maximum frequency by the system.
//
// {893dee8e-2bef-41e0-89c6-b55d0929964d}
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_MINIMUM_1, 0x893DEE8E, 0x2BEF, 0x41E0, 0x89, 0xC6, 0xB5, 0x5D, 0x09, 0x29, 0x96, 0x4D );

//
// Specifies a percentage (between 0 and 100) that the processor frequency
// should not drop below for Processor Power Efficiency Class 2.
//
// {893dee8e-2bef-41e0-89c6-b55d0929964e}
//
DEFINE_GUID( GUID_PROCESSOR_THROTTLE_MINIMUM_2, 0x893DEE8E, 0x2BEF, 0x41E0, 0x89, 0xC6, 0xB5, 0x5D, 0x09, 0x29, 0x96, 0x4E );

//
// Specifies the maximum processor frequency (expresssed in MHz).
//

// {75B0AE3F-BCE0-45a7-8C89-C9611C25E100}
DEFINE_GUID(GUID_PROCESSOR_FREQUENCY_LIMIT,
0x75b0ae3f, 0xbce0, 0x45a7, 0x8c, 0x89, 0xc9, 0x61, 0x1c, 0x25, 0xe1, 0x00);

// {75B0AE3F-BCE0-45a7-8C89-C9611C25E101}
DEFINE_GUID(GUID_PROCESSOR_FREQUENCY_LIMIT_1,
0x75b0ae3f, 0xbce0, 0x45a7, 0x8c, 0x89, 0xc9, 0x61, 0x1c, 0x25, 0xe1, 0x01);

// {75B0AE3F-BCE0-45a7-8C89-C9611C25E102}
DEFINE_GUID(GUID_PROCESSOR_FREQUENCY_LIMIT_2,
0x75b0ae3f, 0xbce0, 0x45a7, 0x8c, 0x89, 0xc9, 0x61, 0x1c, 0x25, 0xe1, 0x02);

//
// Specifies whether throttle states are allowed to be used even when
// performance states are available.
//
// {3b04d4fd-1cc7-4f23-ab1c-d1337819c4bb}
//
DEFINE_GUID( GUID_PROCESSOR_ALLOW_THROTTLING, 0x3b04d4fd, 0x1cc7, 0x4f23, 0xab, 0x1c, 0xd1, 0x33, 0x78, 0x19, 0xc4, 0xbb );

#define PROCESSOR_THROTTLE_DISABLED  0
#define PROCESSOR_THROTTLE_ENABLED   1
#define PROCESSOR_THROTTLE_AUTOMATIC 2

//
// Specifies processor power settings for CState policy data
// {68F262A7-F621-4069-B9A5-4874169BE23C}
//
DEFINE_GUID( GUID_PROCESSOR_IDLESTATE_POLICY, 0x68f262a7, 0xf621, 0x4069, 0xb9, 0xa5, 0x48, 0x74, 0x16, 0x9b, 0xe2, 0x3c);

//
// Specifies processor power settings for PerfState policy data
// {BBDC3814-18E9-4463-8A55-D197327C45C0}
//
DEFINE_GUID( GUID_PROCESSOR_PERFSTATE_POLICY, 0xBBDC3814, 0x18E9, 0x4463, 0x8A, 0x55, 0xD1, 0x97, 0x32, 0x7C, 0x45, 0xC0);

//
// Specifies the increase busy percentage threshold that must be met before
// increasing the processor performance state.
//
// {06cadf0e-64ed-448a-8927-ce7bf90eb35d}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_THRESHOLD, 0x06cadf0e, 0x64ed, 0x448a, 0x89, 0x27, 0xce, 0x7b, 0xf9, 0x0e, 0xb3, 0x5d );

//
// Specifies the increase busy percentage threshold that must be met before
// increasing the processor performance state for Processor Power Efficiency
// Class 1.
//
// {06cadf0e-64ed-448a-8927-ce7bf90eb35e}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_THRESHOLD_1, 0x06cadf0e, 0x64ed, 0x448a, 0x89, 0x27, 0xce, 0x7b, 0xf9, 0x0e, 0xb3, 0x5e );

//
// Specifies the decrease busy percentage threshold that must be met before
// decreasing the processor performance state.
//
// {12a0ab44-fe28-4fa9-b3bd-4b64f44960a6}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_THRESHOLD, 0x12a0ab44, 0xfe28, 0x4fa9, 0xb3, 0xbd, 0x4b, 0x64, 0xf4, 0x49, 0x60, 0xa6 );

//
// Specifies the decrease busy percentage threshold that must be met before
// decreasing the processor performance state for Processor Power Efficiency
// Class 1.
//
// {12a0ab44-fe28-4fa9-b3bd-4b64f44960a7}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_THRESHOLD_1, 0x12a0ab44, 0xfe28, 0x4fa9, 0xb3, 0xbd, 0x4b, 0x64, 0xf4, 0x49, 0x60, 0xa7 );

//
// Specifies, either as ideal, single or rocket, how aggressive performance
// states should be selected when increasing the processor performance state.
//
// {465E1F50-B610-473a-AB58-00D1077DC418}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_POLICY, 0x465e1f50, 0xb610, 0x473a, 0xab, 0x58, 0x0, 0xd1, 0x7, 0x7d, 0xc4, 0x18);

//
// Specifies, either as ideal, single or rocket, how aggressive performance
// states should be selected when increasing the processor performance state
// for Processor Power Efficiency Class 1.
//
// {465E1F50-B610-473a-AB58-00D1077DC419}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_POLICY_1, 0x465e1f50, 0xb610, 0x473a, 0xab, 0x58, 0x0, 0xd1, 0x7, 0x7d, 0xc4, 0x19);

//
// Specifies, either as ideal, single or rocket, how aggressive performance
// states should be selected when decreasing the processor performance state.
//
// {40FBEFC7-2E9D-4d25-A185-0CFD8574BAC6}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_POLICY, 0x40fbefc7, 0x2e9d, 0x4d25, 0xa1, 0x85, 0xc, 0xfd, 0x85, 0x74, 0xba, 0xc6);

//
// Specifies, either as ideal, single or rocket, how aggressive performance
// states should be selected when decreasing the processor performance state for
// Processor Power Efficiency Class 1.
//
// {40FBEFC7-2E9D-4d25-A185-0CFD8574BAC7}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_POLICY_1, 0x40fbefc7, 0x2e9d, 0x4d25, 0xa1, 0x85, 0xc, 0xfd, 0x85, 0x74, 0xba, 0xc7);

//
// Specifies, in milliseconds, the minimum amount of time that must elapse after
// the last processor performance state change before increasing the processor
// performance state.
//
// {984CF492-3BED-4488-A8F9-4286C97BF5AA}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_TIME, 0x984cf492, 0x3bed, 0x4488, 0xa8, 0xf9, 0x42, 0x86, 0xc9, 0x7b, 0xf5, 0xaa);

//
// Specifies, in milliseconds, the minimum amount of time that must elapse after
// the last processor performance state change before increasing the processor
// performance state for Processor Power Efficiency Class 1.
//
// {984CF492-3BED-4488-A8F9-4286C97BF5AB}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_TIME_1, 0x984cf492, 0x3bed, 0x4488, 0xa8, 0xf9, 0x42, 0x86, 0xc9, 0x7b, 0xf5, 0xab);

//
// Specifies, in milliseconds, the minimum amount of time that must elapse after
// the last processor performance state change before increasing the processor
// performance state.
//
// {D8EDEB9B-95CF-4f95-A73C-B061973693C8}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_TIME, 0xd8edeb9b, 0x95cf, 0x4f95, 0xa7, 0x3c, 0xb0, 0x61, 0x97, 0x36, 0x93, 0xc8);

//
// Specifies, in milliseconds, the minimum amount of time that must elapse after
// the last processor performance state change before increasing the processor
// performance state for Processor Power Efficiency Class 1.
//
// {D8EDEB9B-95CF-4f95-A73C-B061973693C9}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_TIME_1, 0xd8edeb9b, 0x95cf, 0x4f95, 0xa7, 0x3c, 0xb0, 0x61, 0x97, 0x36, 0x93, 0xc9);

//
// Specifies the time, in milliseconds, that must expire before considering
// a change in the processor performance states or parked core set.
//
// {4D2B0152-7D5C-498b-88E2-34345392A2C5}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_TIME_CHECK, 0x4d2b0152, 0x7d5c, 0x498b, 0x88, 0xe2, 0x34, 0x34, 0x53, 0x92, 0xa2, 0xc5);

//
// Specifies how the processor should manage performance and efficiency
// tradeoffs when boosting frequency above the maximum.
//
// {45BCC044-D885-43e2-8605-EE0EC6E96B59}
//
DEFINE_GUID(GUID_PROCESSOR_PERF_BOOST_POLICY,
0x45bcc044, 0xd885, 0x43e2, 0x86, 0x5, 0xee, 0xe, 0xc6, 0xe9, 0x6b, 0x59);

#define PROCESSOR_PERF_BOOST_POLICY_DISABLED 0
#define PROCESSOR_PERF_BOOST_POLICY_MAX 100

//
// Specifies how a processor opportunistically increases frequency above
// the maximum when operating contitions allow it to do so safely.
//
// {BE337238-0D82-4146-A960-4F3749D470C7}
//
DEFINE_GUID(GUID_PROCESSOR_PERF_BOOST_MODE,
0xbe337238, 0xd82, 0x4146, 0xa9, 0x60, 0x4f, 0x37, 0x49, 0xd4, 0x70, 0xc7);

#define PROCESSOR_PERF_BOOST_MODE_DISABLED 0
#define PROCESSOR_PERF_BOOST_MODE_ENABLED 1
#define PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE 2
#define PROCESSOR_PERF_BOOST_MODE_EFFICIENT_ENABLED 3
#define PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE 4
#define PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE_AT_GUARANTEED 5
#define PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE_AT_GUARANTEED 6
#define PROCESSOR_PERF_BOOST_MODE_MAX PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE_AT_GUARANTEED

//
// Specifies whether or not a procesor should autonomously select its
// operating performance state.
//
// {8BAA4A8A-14C6-4451-8E8B-14BDBD197537}
//
DEFINE_GUID(GUID_PROCESSOR_PERF_AUTONOMOUS_MODE,
0x8baa4a8a, 0x14c6, 0x4451, 0x8e, 0x8b, 0x14, 0xbd, 0xbd, 0x19, 0x75, 0x37);

#define PROCESSOR_PERF_AUTONOMOUS_MODE_DISABLED 0
#define PROCESSOR_PERF_AUTONOMOUS_MODE_ENABLED 1

//
// Specifies the tradeoff between performance and energy the processor should
// make when operating in autonomous mode.
//
// {36687F9E-E3A5-4dbf-B1DC-15EB381C6863}
DEFINE_GUID(GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE,
0x36687f9e, 0xe3a5, 0x4dbf, 0xb1, 0xdc, 0x15, 0xeb, 0x38, 0x1c, 0x68, 0x63);

//
// Specifies the tradeoff between performance and energy the processor should
// make when operating in autonomous mode for class 1 processors.
//
// {36687F9E-E3A5-4dbf-B1DC-15EB381C6864}
DEFINE_GUID(GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE_1,
0x36687f9e, 0xe3a5, 0x4dbf, 0xb1, 0xdc, 0x15, 0xeb, 0x38, 0x1c, 0x68, 0x64);

//
// Specifies the tradeoff between performance and energy the processor should
// make when operating in autonomous mode for class 2 processors.
//
// {36687F9E-E3A5-4dbf-B1DC-15EB381C6865}
DEFINE_GUID(GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE_2,
0x36687f9e, 0xe3a5, 0x4dbf, 0xb1, 0xdc, 0x15, 0xeb, 0x38, 0x1c, 0x68, 0x65);

#define PROCESSOR_PERF_PERFORMANCE_PREFERENCE 0xff
#define PROCESSOR_PERF_ENERGY_PREFERENCE         0

//
// Specifies the window over which the processor should observe utilization when
// operating in autonomous mode, in microseconds.
//
// {CFEDA3D0-7697-4566-A922-A9086CD49DFA}
DEFINE_GUID(GUID_PROCESSOR_PERF_AUTONOMOUS_ACTIVITY_WINDOW,
0xcfeda3d0, 0x7697, 0x4566, 0xa9, 0x22, 0xa9, 0x8, 0x6c, 0xd4, 0x9d, 0xfa);

#define PROCESSOR_PERF_MINIMUM_ACTIVITY_WINDOW 0
#define PROCESSOR_PERF_MAXIMUM_ACTIVITY_WINDOW 1270000000

//
// Specifies whether the processor should perform duty cycling.
//
// {4E4450B3-6179-4e91-B8F1-5BB9938F81A1}
DEFINE_GUID(GUID_PROCESSOR_DUTY_CYCLING,
0x4e4450b3, 0x6179, 0x4e91, 0xb8, 0xf1, 0x5b, 0xb9, 0x93, 0x8f, 0x81, 0xa1);

#define PROCESSOR_DUTY_CYCLING_DISABLED 0
#define PROCESSOR_DUTY_CYCLING_ENABLED 1

//
// Specifies if idle state promotion and demotion values should be scaled based
// on the current peformance state.
//
// {6C2993B0-8F48-481f-BCC6-00DD2742AA06}
//
DEFINE_GUID( GUID_PROCESSOR_IDLE_ALLOW_SCALING, 0x6c2993b0, 0x8f48, 0x481f, 0xbc, 0xc6, 0x0, 0xdd, 0x27, 0x42, 0xaa, 0x6);

//
// Specifies if idle states should be disabled.
//
// {5D76A2CA-E8C0-402f-A133-2158492D58AD}
//
DEFINE_GUID( GUID_PROCESSOR_IDLE_DISABLE, 0x5d76a2ca, 0xe8c0, 0x402f, 0xa1, 0x33, 0x21, 0x58, 0x49, 0x2d, 0x58, 0xad);

//
// Specifies the deepest idle state type that should be used. If this value is
// set to zero, this setting is ignored. Values higher than supported by the
// processor then this setting has no effect.
//
// {9943e905-9a30-4ec1-9b99-44dd3b76f7a2}
//
DEFINE_GUID( GUID_PROCESSOR_IDLE_STATE_MAXIMUM, 0x9943e905, 0x9a30, 0x4ec1, 0x9b, 0x99, 0x44, 0xdd, 0x3b, 0x76, 0xf7, 0xa2);

//
// Specifies the time that elapsed since the last idle state promotion or
// demotion before idle states may be promoted or demoted again (in
// microseconds).
//
// {C4581C31-89AB-4597-8E2B-9C9CAB440E6B}
//
DEFINE_GUID( GUID_PROCESSOR_IDLE_TIME_CHECK, 0xc4581c31, 0x89ab, 0x4597, 0x8e, 0x2b, 0x9c, 0x9c, 0xab, 0x44, 0xe, 0x6b);


//
// Specifies the upper busy threshold that must be met before demoting the
// processor to a lighter idle state (in percentage).
//
// {4B92D758-5A24-4851-A470-815D78AEE119}
//
DEFINE_GUID( GUID_PROCESSOR_IDLE_DEMOTE_THRESHOLD, 0x4b92d758, 0x5a24, 0x4851, 0xa4, 0x70, 0x81, 0x5d, 0x78, 0xae, 0xe1, 0x19);

//
// Specifies the lower busy threshold that must be met before promoting the
// processor to a deeper idle state (in percentage).
//
// {7B224883-B3CC-4d79-819F-8374152CBE7C}
//
DEFINE_GUID( GUID_PROCESSOR_IDLE_PROMOTE_THRESHOLD, 0x7b224883, 0xb3cc, 0x4d79, 0x81, 0x9f, 0x83, 0x74, 0x15, 0x2c, 0xbe, 0x7c);

//
// Specifies the utilization threshold in percent that must be crossed in order to un-park cores.
//
// N.B. This power setting is DEPRECATED.
//
// {df142941-20f3-4edf-9a4a-9c83d3d717d1}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_INCREASE_THRESHOLD, 0xdf142941, 0x20f3, 0x4edf, 0x9a, 0x4a, 0x9c, 0x83, 0xd3, 0xd7, 0x17, 0xd1 );

//
// Specifies the utilization threshold in percent that must be crossed in order to park cores.
//
// N.B. This power setting is DEPRECATED.
//
// {68dd2f27-a4ce-4e11-8487-3794e4135dfa}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_DECREASE_THRESHOLD, 0x68dd2f27, 0xa4ce, 0x4e11, 0x84, 0x87, 0x37, 0x94, 0xe4, 0x13, 0x5d, 0xfa);

//
// Specifies, either as ideal, single or rocket, how aggressive core parking is when cores must be unparked.
//
// {c7be0679-2817-4d69-9d02-519a537ed0c6}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_INCREASE_POLICY, 0xc7be0679, 0x2817, 0x4d69, 0x9d, 0x02, 0x51, 0x9a, 0x53, 0x7e, 0xd0, 0xc6);

#define CORE_PARKING_POLICY_CHANGE_IDEAL  0
#define CORE_PARKING_POLICY_CHANGE_SINGLE 1
#define CORE_PARKING_POLICY_CHANGE_ROCKET 2
#define CORE_PARKING_POLICY_CHANGE_MULTISTEP 3
#define CORE_PARKING_POLICY_CHANGE_MAX CORE_PARKING_POLICY_CHANGE_MULTISTEP

//
// Specifies, either as ideal, single or rocket, how aggressive core parking is when cores must be parked.
//
// {71021b41-c749-4d21-be74-a00f335d582b}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_DECREASE_POLICY, 0x71021b41, 0xc749, 0x4d21, 0xbe, 0x74, 0xa0, 0x0f, 0x33, 0x5d, 0x58, 0x2b);

//
// Specifies, on a per processor group basis, the maximum number of cores that can be kept unparked.
//
// {ea062031-0e34-4ff1-9b6d-eb1059334028}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_MAX_CORES, 0xea062031, 0x0e34, 0x4ff1, 0x9b, 0x6d, 0xeb, 0x10, 0x59, 0x33, 0x40, 0x28);

//
// Specifies, on a per processor group basis, the maximum number of cores that
// can be kept unparked for Processor Power Efficiency Class 1.
//
// {ea062031-0e34-4ff1-9b6d-eb1059334029}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_MAX_CORES_1, 0xea062031, 0x0e34, 0x4ff1, 0x9b, 0x6d, 0xeb, 0x10, 0x59, 0x33, 0x40, 0x29);

//
// Specifies, on a per processor group basis, the minimum number of cores that must be kept unparked.
//
// {0cc5b647-c1df-4637-891a-dec35c318583}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_MIN_CORES, 0x0cc5b647, 0xc1df, 0x4637, 0x89, 0x1a, 0xde, 0xc3, 0x5c, 0x31, 0x85, 0x83);

//
// Specifies, on a per processor group basis, the minimum number of cores that
// must be kept unparked in Processor Power Efficiency Class 1.
//
// {0cc5b647-c1df-4637-891a-dec35c318584}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_MIN_CORES_1, 0x0cc5b647, 0xc1df, 0x4637, 0x89, 0x1a, 0xde, 0xc3, 0x5c, 0x31, 0x85, 0x84);

//
// Specifies, in milliseconds, the minimum amount of time a core must be parked before it can be unparked.
//
// {2ddd5a84-5a71-437e-912a-db0b8c788732}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_INCREASE_TIME, 0x2ddd5a84, 0x5a71, 0x437e, 0x91, 0x2a, 0xdb, 0x0b, 0x8c, 0x78, 0x87, 0x32);

//
// Specifies, in milliseconds, the minimum amount of time a core must be unparked before it can be parked.
//
// {dfd10d17-d5eb-45dd-877a-9a34ddd15c82}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_DECREASE_TIME, 0xdfd10d17, 0xd5eb, 0x45dd, 0x87, 0x7a, 0x9a, 0x34, 0xdd, 0xd1, 0x5c, 0x82);

//
// Specifies the factor by which to decrease affinity history on each core after each check.
//
// {8f7b45e3-c393-480a-878c-f67ac3d07082}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_DECREASE_FACTOR, 0x8f7b45e3, 0xc393, 0x480a, 0x87, 0x8c, 0xf6, 0x7a, 0xc3, 0xd0, 0x70, 0x82);

//
// Specifies the threshold above which a core is considered to have had significant affinitized work scheduled to it while parked.
//
// {5b33697b-e89d-4d38-aa46-9e7dfb7cd2f9}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_THRESHOLD, 0x5b33697b, 0xe89d, 0x4d38, 0xaa, 0x46, 0x9e, 0x7d, 0xfb, 0x7c, 0xd2, 0xf9);

//
// Specifies the weighting given to each occurence where affinitized work was scheduled to a parked core.
//
// {e70867f1-fa2f-4f4e-aea1-4d8a0ba23b20}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_AFFINITY_WEIGHTING, 0xe70867f1, 0xfa2f, 0x4f4e, 0xae, 0xa1, 0x4d, 0x8a, 0x0b, 0xa2, 0x3b, 0x20);

//
// Specifies the factor by which to decrease the over utilization history on each core after the current performance check.
//
// {1299023c-bc28-4f0a-81ec-d3295a8d815d}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_DECREASE_FACTOR, 0x1299023c, 0xbc28, 0x4f0a, 0x81, 0xec, 0xd3, 0x29, 0x5a, 0x8d, 0x81, 0x5d);

//
// Specifies the threshold above which a core is considered to have been recently over utilized while parked.
//
// {9ac18e92-aa3c-4e27-b307-01ae37307129}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_THRESHOLD, 0x9ac18e92, 0xaa3c, 0x4e27, 0xb3, 0x07, 0x01, 0xae, 0x37, 0x30, 0x71, 0x29);

//
// Specifies the weighting given to each occurence where a parked core is found to be over utilized.
//
// {8809c2d8-b155-42d4-bcda-0d345651b1db}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_WEIGHTING, 0x8809c2d8, 0xb155, 0x42d4, 0xbc, 0xda, 0x0d, 0x34, 0x56, 0x51, 0xb1, 0xdb);

//
// Specifies, in percentage, the busy threshold that must be met before a parked core is considered over utilized.
//
// {943c8cb6-6f93-4227-ad87-e9a3feec08d1}
//
DEFINE_GUID( GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_THRESHOLD, 0x943c8cb6, 0x6f93, 0x4227, 0xad, 0x87, 0xe9, 0xa3, 0xfe, 0xec, 0x08, 0xd1);

//
// Specifies if at least one processor per core should always remain unparked.
//
// {a55612aa-f624-42c6-a443-7397d064c04f}
//

DEFINE_GUID( GUID_PROCESSOR_PARKING_CORE_OVERRIDE, 0xa55612aa, 0xf624, 0x42c6, 0xa4, 0x43, 0x73, 0x97, 0xd0, 0x64, 0xc0, 0x4f);

//
// Specifies what performance state a processor should enter when first parked.
//
// {447235c7-6a8d-4cc0-8e24-9eaf70b96e2b}
//

DEFINE_GUID( GUID_PROCESSOR_PARKING_PERF_STATE, 0x447235c7, 0x6a8d, 0x4cc0, 0x8e, 0x24, 0x9e, 0xaf, 0x70, 0xb9, 0x6e, 0x2b);

//
// Specifies what performance state a processor should enter when first parked
// for Processor Power Efficiency Class 1.
//
// {447235c7-6a8d-4cc0-8e24-9eaf70b96e2c}
//
DEFINE_GUID( GUID_PROCESSOR_PARKING_PERF_STATE_1, 0x447235c7, 0x6a8d, 0x4cc0, 0x8e, 0x24, 0x9e, 0xaf, 0x70, 0xb9, 0x6e, 0x2c);

//
// Specify the busy threshold that must be met when calculating the concurrency of a node's workload.
//
// {2430ab6f-a520-44a2-9601-f7f23b5134b1}
//

DEFINE_GUID( GUID_PROCESSOR_PARKING_CONCURRENCY_THRESHOLD, 0x2430ab6f, 0xa520, 0x44a2, 0x96, 0x01, 0xf7, 0xf2, 0x3b, 0x51, 0x34, 0xb1);

//
// Specify the busy threshold that must be met by all cores in a concurrency set to unpark an extra core.
//
// {f735a673-2066-4f80-a0c5-ddee0cf1bf5d}
//

DEFINE_GUID( GUID_PROCESSOR_PARKING_HEADROOM_THRESHOLD, 0xf735a673, 0x2066, 0x4f80, 0xa0, 0xc5, 0xdd, 0xee, 0x0c, 0xf1, 0xbf, 0x5d);

//
// Specify the percentage utilization used to calculate the distribution concurrency.
//
// {4bdaf4e9-d103-46d7-a5f0-6280121616ef}
//

DEFINE_GUID( GUID_PROCESSOR_PARKING_DISTRIBUTION_THRESHOLD, 0x4bdaf4e9, 0xd103, 0x46d7, 0xa5, 0xf0, 0x62, 0x80, 0x12, 0x16, 0x16, 0xef);

//
// Specify the anticipated execution latency at which a soft parked core can be
// used by the scheduler.
//
// {97CFAC41-2217-47eb-992D-618B1977C907}
//
DEFINE_GUID(GUID_PROCESSOR_SOFT_PARKING_LATENCY,
0x97cfac41, 0x2217, 0x47eb, 0x99, 0x2d, 0x61, 0x8b, 0x19, 0x77, 0xc9, 0x7);

//
// Specifies the number of perf time check intervals to average utility over.
//
// {7d24baa7-0b84-480f-840c-1b0743c00f5f}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_HISTORY, 0x7d24baa7, 0x0b84, 0x480f, 0x84, 0x0c, 0x1b, 0x07, 0x43, 0xc0, 0x0f, 0x5f);

//
// Specifies the number of perf time check intervals to average utility over in
// Processor Power Efficiency Class 1.
//
// {7d24baa7-0b84-480f-840c-1b0743c00f60}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_HISTORY_1, 0x7d24baa7, 0x0b84, 0x480f, 0x84, 0x0c, 0x1b, 0x07, 0x43, 0xc0, 0x0f, 0x60);

//
// Specifies the number of perf time check intervals to average utility over to
// determine performance increase.
//
// N.B. This power setting is DEPRECATED.
//
// {99B3EF01-752F-46a1-80FB-7730011F2354}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_INCREASE_HISTORY, 0x99b3ef01, 0x752f, 0x46a1, 0x80, 0xfb, 0x77, 0x30, 0x1, 0x1f, 0x23, 0x54);

//
// Specifies the number of perf time check intervals to average utility over to
// determine performance decrease.
//
// N.B. This power setting is DEPRECATED.
//
// {0300F6F8-ABD6-45a9-B74F-4908691A40B5}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_DECREASE_HISTORY, 0x300f6f8, 0xabd6, 0x45a9, 0xb7, 0x4f, 0x49, 0x8, 0x69, 0x1a, 0x40, 0xb5);

//
// Specifies the number of perf time check intervals to average utility over for
// core parking.
//
// N.B. This power setting is DEPRECATED.
//
// {77D7F282-8F1A-42cd-8537-45450A839BE8}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_CORE_PARKING_HISTORY, 0x77d7f282, 0x8f1a, 0x42cd, 0x85, 0x37, 0x45, 0x45, 0xa, 0x83, 0x9b, 0xe8);

//
// Specifies whether latency sensitivity hints should be taken into account by
// the perf state engine.
//
// N.B. This power setting is DEPRECATED.
//
// {0822df31-9c83-441c-a079-0de4cf009c7b}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT, 0x0822df31, 0x9c83, 0x441c, 0xa0, 0x79, 0x0d, 0xe4, 0xcf, 0x00, 0x9c, 0x7b);

//
// Specifies the processor performance state in response to latency sensitivity hints.
//
// {619b7505-003b-4e82-b7a6-4dd29c300971}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT_PERF, 0x619b7505, 0x3b, 0x4e82, 0xb7, 0xa6, 0x4d, 0xd2, 0x9c, 0x30, 0x9, 0x71);

//
// Specifies the processor performance state in response to latency sensitivity
// hints for Processor Power Efficiency Class 1.
//
// {619b7505-003b-4e82-b7a6-4dd29c300972}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT_PERF_1, 0x619b7505, 0x3b, 0x4e82, 0xb7, 0xa6, 0x4d, 0xd2, 0x9c, 0x30, 0x9, 0x72);

//
// Specifies the processor performance state in response to latency sensitivity
// hints for Processor Power Efficiency Class 2.
//
// {619b7505-003b-4e82-b7a6-4dd29c300973}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT_PERF_2, 0x619b7505, 0x3b, 0x4e82, 0xb7, 0xa6, 0x4d, 0xd2, 0x9c, 0x30, 0x9, 0x73);

//
// Specifies the energy/performance preference to use in response to latency
// sensitivity hints.
//
// {4B70F900-CDD9-4e66-AA26-AE8417F98173}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT_EPP, 0x4b70f900, 0xcdd9, 0x4e66, 0xaa, 0x26, 0xae, 0x84, 0x17, 0xf9, 0x81, 0x73);

//
// Specifies the energy/performance preference to use in response to latency
// sensitivity hints for Processor Power Efficiency Class 1.
//
// {4B70F900-CDD9-4e66-AA26-AE8417F98174}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT_EPP_1, 0x4b70f900, 0xcdd9, 0x4e66, 0xaa, 0x26, 0xae, 0x84, 0x17, 0xf9, 0x81, 0x74);

//
// Specifies the energy/performance preference to use in response to latency
// sensitivity hints for Processor Power Efficiency Class 2.
//
// {4B70F900-CDD9-4e66-AA26-AE8417F98175}
//
DEFINE_GUID( GUID_PROCESSOR_PERF_LATENCY_HINT_EPP_2, 0x4b70f900, 0xcdd9, 0x4e66, 0xaa, 0x26, 0xae, 0x84, 0x17, 0xf9, 0x81, 0x75);

//
// Specifies the minimum unparked processors when a latency hint is active
// (in a percentage).
//
// {616cdaa5-695e-4545-97ad-97dc2d1bdd88}
//
DEFINE_GUID( GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK, 0x616cdaa5, 0x695e, 0x4545, 0x97, 0xad, 0x97, 0xdc, 0x2d, 0x1b, 0xdd, 0x88);

//
// Specifies the minimum unparked processors when a latency hint is active
// for Processor Power Efficiency Class 1 (in a percentage).
//
// {616cdaa5-695e-4545-97ad-97dc2d1bdd89}
//
DEFINE_GUID( GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK_1, 0x616cdaa5, 0x695e, 0x4545, 0x97, 0xad, 0x97, 0xdc, 0x2d, 0x1b, 0xdd, 0x89);

//
// Specifies the module unparking policy.
//
// {b0deaf6b-59c0-4523-8a45-ca7f40244114}
//
DEFINE_GUID( GUID_PROCESSOR_MODULE_PARKING_POLICY, 0xb0deaf6b, 0x59c0, 0x4523, 0x8a, 0x45, 0xca, 0x7f, 0x40, 0x24, 0x41, 0x14);

//
// Specifies the complex llc unparking policy.
//
// {b669a5e9-7b1d-4132-baaa-49190abcfeb6}
//
DEFINE_GUID(GUID_PROCESSOR_COMPLEX_PARKING_POLICY, 0xb669a5e9, 0x7b1d, 0x4132, 0xba, 0xaa, 0x49, 0x19, 0xa, 0xbc, 0xfe, 0xb6);

//
// PO topology(module or complex) parking Policies
//

#define PARKING_TOPOLOGY_POLICY_DISABLED    0
#define PARKING_TOPOLOGY_POLICY_ROUNDROBIN  1
#define PARKING_TOPOLOGY_POLICY_SEQUENTIAL  2

//
// Specifies the Smt unparking policy.
//
// {b28a6829-c5f7-444e-8f61-10e24e85c532}
//

DEFINE_GUID(GUID_PROCESSOR_SMT_UNPARKING_POLICY, 0xb28a6829, 0xc5f7, 0x444e, 0x8f, 0x61, 0x10, 0xe2, 0x4e, 0x85, 0xc5, 0x32);

#define SMT_UNPARKING_POLICY_CORE 0
#define SMT_UNPARKING_POLICY_CORE_PER_THREAD 1
#define SMT_UNPARKING_POLICY_LP_ROUNDROBIN 2
#define SMT_UNPARKING_POLICY_LP_SEQUENTIAL 3

//
// Specifies the maximum processor count for corresponding QoS threads.
//
// {1a98ad09-af22-42ca-8e61-f0a5802c270a}
//

DEFINE_GUID(GUID_PROCESSOR_RESTRICTION_COUNT, 0x1a98ad09, 0xaf22, 0x42ca, 0x8e, 0x61, 0xf0, 0xa5, 0x80, 0x2c, 0x27, 0x0a);

//
// Specifies whether the core parking engine should distribute processor
// utility.
//
// {e0007330-f589-42ed-a401-5ddb10e785d3}
//
DEFINE_GUID( GUID_PROCESSOR_DISTRIBUTE_UTILITY, 0xe0007330, 0xf589, 0x42ed, 0xa4, 0x01, 0x5d, 0xdb, 0x10, 0xe7, 0x85, 0xd3);

//
// Specifies the processor resource priority.
//
// {603fe9ce-8d01-4b48-a968-1d706c28df5c}
//
DEFINE_GUID( GUID_PROCESSOR_RESOURCE_PRIORITY, 0x603fe9ce, 0x8d01, 0x4b48, 0xa9, 0x68, 0x1d, 0x70, 0x6c, 0x28, 0xfd, 0x5c);

//
// Specifies the processor resource priority for Processor Power Efficiency Class 1.
//
// {603fe9ce-8d01-4b48-a968-1d706c28df5d}
//
DEFINE_GUID( GUID_PROCESSOR_RESOURCE_PRIORITY_1, 0x603fe9ce, 0x8d01, 0x4b48, 0xa9, 0x68, 0x1d, 0x70, 0x6c, 0x28, 0xfd, 0x5d);

//
// Specifies the processor resource priority for Processor Power Efficiency Class 2.
//
// {603fe9ce-8d01-4b48-a968-1d706c28df5e}
//
DEFINE_GUID( GUID_PROCESSOR_RESOURCE_PRIORITY_2, 0x603fe9ce, 0x8d01, 0x4b48, 0xa9, 0x68, 0x1d, 0x70, 0x6c, 0x28, 0xfd, 0x5e);

//
// GUIDS to control PPM settings on computer system with more than one
// Processor Power Efficiency Classes (heterogeneous system).
// -----------------
//
// Specifies the current active heterogeneous policy.
//
// {7f2f5cfa-f10c-4823-b5e1-e93ae85f46b5}
//
DEFINE_GUID( GUID_PROCESSOR_HETEROGENEOUS_POLICY, 0x7f2f5cfa, 0xf10c, 0x4823, 0xb5, 0xe1, 0xe9, 0x3a, 0xe8, 0x5f, 0x46, 0xb5);

//
// Specifies the number of perf check cycles required to decrease the number of
// Processor Power Efficiency Class 1 processors.
//
// {7f2492b6-60b1-45e5-ae55-773f8cd5caec}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_DECREASE_TIME, 0x7f2492b6, 0x60b1, 0x45e5, 0xae, 0x55, 0x77, 0x3f, 0x8c, 0xd5, 0xca, 0xec);

//
// Specifies the number of perf check cycles required to increase the number of
// Processor Power Efficiency Class 1 processors.
//
// {4009efa7-e72d-4cba-9edf-91084ea8cbc3}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_INCREASE_TIME, 0x4009efa7, 0xe72d, 0x4cba, 0x9e, 0xdf, 0x91, 0x08, 0x4e, 0xa8, 0xcb, 0xc3);

//
// Specify the minimum number of perf check intervals since the last
// performance state change before the one containment zone may be decreased for
// picking cores from another containment zone.
//
// {6FF13AEB-7897-4356-9999-DD9930AF065F}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_CONTAINMENT_DECREASE_TIME, 0x6FF13AEB, 0x7897, 0x4356, 0x99, 0x99, 0xDD, 0x99, 0x30, 0xAF, 0x06, 0x5F);

//
// Specify the minimum number of perf check intervals since the last
// performance state change before the one containment zone may be increase for
// picking cores from another containment.
//
// {64FCEE6B-5B1F-45A4-A76A-19B2C36EE290}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_CONTAINMENT_INCREASE_TIME, 0x64FCEE6B, 0x5B1F, 0x45A4, 0xA7, 0x6A, 0x19, 0xB2, 0xC3, 0x6E, 0xE2, 0x90);

//
// Specify the busy threshold that must be met when calculating the containment
// crossover from efficiency to hybrid.
//
// {69439B22-221B-4830-BD34-F7BCECE24583}
DEFINE_GUID(GUID_PROCESSOR_HETERO_CONTAINMENT_EFFICIENCY_THRESHOLD, 0x69439b22, 0x221b, 0x4830, 0xbd, 0x34, 0xf7, 0xbc, 0xec, 0xe2, 0x45, 0x83);

//
// Specify the busy threshold that must be met when calculating the containment
// crossover from hybrid to no containment.
//
// {6788488B-1B90-4D11-8FA7-973E470DFF47}
DEFINE_GUID(GUID_PROCESSOR_HETERO_CONTAINMENT_HYBRID_THRESHOLD, 0x6788488b, 0x1b90, 0x4d11, 0x8f, 0xa7, 0x97, 0x3e, 0x47, 0xd, 0xff, 0x47);

//
// Specify whether containment policy should be enable or disable.
//
// {60FBE21B-EFD9-49F2-B066-8674D8E9F423}
DEFINE_GUID(GUID_PROCESSOR_HETERO_CONTAINMENT_POLICY, 0x60fbe21b, 0xefd9, 0x49f2, 0xb0, 0x66, 0x86, 0x74, 0xd8, 0xe9, 0xf4, 0x23);

//
// Specify the important utility percentage that once met, allow workload to move to hybrid containment zone.
//
// {6ece9e1f-b6dd-42bf-b1b7-5a512b10c092}
DEFINE_GUID(GUID_PROCESSOR_HETERO_CONTAINMENT_EFFICIENCY_IMP_UTIL_THRESHOLD, 0x6ece9e1f, 0xb6dd, 0x42bf, 0xb1, 0xb7, 0x5a, 0x51, 0x2b, 0x10, 0xc0, 0x92);

//
// Specify the important utility percentage that once met, allow workload to move to no containment zone.
//
// {12fd031f-53d2-4bf4-ac6d-c699fc9538c7}
DEFINE_GUID(GUID_PROCESSOR_HETERO_CONTAINMENT_HYBRID_IMP_UTIL_THRESHOLD, 0x12fd031f, 0x53d2, 0x4bf4, 0xac, 0x6d, 0xc6, 0x99, 0xfc, 0x95, 0x38, 0xc7);

//
// Specifies the minimum efficiency score for a core to be considered efficient
// or "small" for WPS systems. A value of 0 disables the policy.
//
// {5BA7419A-295C-4B02-841B-66799388D6DA}
DEFINE_GUID(GUID_PROCESSOR_WPS_MIN_EFFICIENCY_THRESHOLD, 0x5ba7419a, 0x295c, 0x4b02, 0x84, 0x1b, 0x66, 0x79, 0x93, 0x88, 0xd6, 0xda);

//
// Specifies the performance level (in units of Processor Power Efficiency
// Class 0 processor performance) at which the number of Processor Power
// Efficiency Class 1 processors is decreased.
//
// {f8861c27-95e7-475c-865b-13c0cb3f9d6b}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD, 0xf8861c27, 0x95e7, 0x475c, 0x86, 0x5b, 0x13, 0xc0, 0xcb, 0x3f, 0x9d, 0x6b);

//
// Specifies the performance level (in units of Processor Power Efficiency
// Class 1 processor performance) at which the number of Processor Power
// Efficiency Class 2 processors is decreased.
//
// {f8861c27-95e7-475c-865b-13c0cb3f9d6c}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD_1, 0xf8861c27, 0x95e7, 0x475c, 0x86, 0x5b, 0x13, 0xc0, 0xcb, 0x3f, 0x9d, 0x6c);

//
// Specifies the performance level (in units of Processor Power Efficiency
// Class 0 processor performance) at which the number of Processor Power
// Efficiency Class 1 processors is increased.
//
// {b000397d-9b0b-483d-98c9-692a6060cfbf}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD, 0xb000397d, 0x9b0b, 0x483d, 0x98, 0xc9, 0x69, 0x2a, 0x60, 0x60, 0xcf, 0xbf);

//
// Specifies the performance level (in units of Processor Power Efficiency
// Class 1 processor performance) at which the number of Processor Power
// Efficiency Class 2 processors is increased.
//
// {b000397d-9b0b-483d-98c9-692a6060cfc0}
//
DEFINE_GUID( GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD_1, 0xb000397d, 0x9b0b, 0x483d, 0x98, 0xc9, 0x69, 0x2a, 0x60, 0x60, 0xcf, 0xc0);

//
// Specifies the performance target floor of a Processor Power Efficiency
// Class 0 processor when the system unparks Processor Power Efficiency Class 1
// processor(s).
//
// {fddc842b-8364-4edc-94cf-c17f60de1c80}
//
DEFINE_GUID( GUID_PROCESSOR_CLASS0_FLOOR_PERF, 0xfddc842b, 0x8364, 0x4edc, 0x94, 0xcf, 0xc1, 0x7f, 0x60, 0xde, 0x1c, 0x80);

//
// Specifies the initial performance target of a Processor Power Efficiency
// Class 1 processor when the system makes a transition up from zero Processor
// Power Efficiency Class 1 processors.
//
// {1facfc65-a930-4bc5-9f38-504ec097bbc0}
//
DEFINE_GUID( GUID_PROCESSOR_CLASS1_INITIAL_PERF, 0x1facfc65, 0xa930, 0x4bc5, 0x9f, 0x38, 0x50, 0x4e, 0xc0, 0x97, 0xbb, 0xc0);

//
// Specifies the scheduling policy for threads in a given QoS class.
//
// {93B8B6DC-0698-4d1c-9EE4-0644E900C85D}
//
DEFINE_GUID( GUID_PROCESSOR_THREAD_SCHEDULING_POLICY,
0x93b8b6dc, 0x698, 0x4d1c, 0x9e, 0xe4, 0x6, 0x44, 0xe9, 0x0, 0xc8, 0x5d);

//
// Specifies the scheduling policy for short running threads in a given QoS
// class.
//
// {BAE08B81-2D5E-4688-AD6A-13243356654B}
//
DEFINE_GUID( GUID_PROCESSOR_SHORT_THREAD_SCHEDULING_POLICY,
0xbae08b81, 0x2d5e, 0x4688, 0xad, 0x6a, 0x13, 0x24, 0x33, 0x56, 0x65, 0x4b);

//
// Specifies the global threshold that designates which threads have a
// short versus a long runtime.
//
// {D92998C2-6A48-49CA-85D4-8CCEEC294570}
//
DEFINE_GUID( GUID_PROCESSOR_SHORT_THREAD_RUNTIME_THRESHOLD,
0xd92998c2, 0x6a48, 0x49ca, 0x85, 0xd4, 0x8c, 0xce, 0xec, 0x29, 0x45, 0x70);

//
// Specify the upper limit of architecture class for short run threads.
//
// {828423EB-8662-4344-90F7-52BF15870F5A}
//
DEFINE_GUID( GUID_PROCESSOR_SHORT_THREAD_ARCH_CLASS_UPPER_THRESHOLD,
0x828423eb, 0x8662, 0x4344, 0x90, 0xf7, 0x52, 0xbf, 0x15, 0x87, 0x0f, 0x5a);

//
// Specify the lower limit of architecture class for short run threads.
//
// {53824D46-87BD-4739-AA1B-AA793FAC36D6}
//
DEFINE_GUID( GUID_PROCESSOR_SHORT_THREAD_ARCH_CLASS_LOWER_THRESHOLD,
0x53824d46, 0x87bd, 0x4739, 0xaa, 0x1b, 0xaa, 0x79, 0x3f, 0xac, 0x36, 0xd6);

//
// Specify the upper limit of architecture class for long run threads.
//
// {BF903D33-9D24-49D3-A468-E65E0325046A}
//
DEFINE_GUID( GUID_PROCESSOR_LONG_THREAD_ARCH_CLASS_UPPER_THRESHOLD,
0xbf903d33, 0x9d24, 0x49d3, 0xa4, 0x68, 0xe6, 0x5e, 0x03, 0x25, 0x04, 0x6a);

//
// Specify the lower limit of architecture class for long run threads.
//
// {43F278BC-0F8A-46D0-8B31-9A23E615D713}
//
DEFINE_GUID( GUID_PROCESSOR_LONG_THREAD_ARCH_CLASS_LOWER_THRESHOLD,
0x43f278bc, 0x0f8a, 0x46d0, 0x8b, 0x31, 0x9a, 0x23, 0xe6, 0x15, 0xd7, 0x13);

//
// Specifies active vs passive cooling.  Although not directly related to
// processor settings, it is the processor that gets throttled if we're doing
// passive cooling, so it is fairly strongly related.
// {94D3A615-A899-4AC5-AE2B-E4D8F634367F}
//
DEFINE_GUID( GUID_SYSTEM_COOLING_POLICY,
0x94D3A615, 0xA899, 0x4AC5, 0xAE, 0x2B, 0xE4, 0xD8, 0xF6, 0x34, 0x36, 0x7F);

//
// Processor responsiveness settings
//
// Specifies the number of responsiveness events required to disable
// responsiveness policy overrides.
//
// {38B8383D-CCE0-4c79-9E3E-56A4F17CC480}
//
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD,
0x38b8383d, 0xcce0, 0x4c79, 0x9e, 0x3e, 0x56, 0xa4, 0xf1, 0x7c, 0xc4, 0x80);

//
// Specifies the number of responsiveness events required to disable
// responsiveness policy overrides for efficiency class 1 processors.
//
// {38B8383D-CCE0-4c79-9E3E-56A4F17CC481}
//
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD_1,
0x38b8383d, 0xcce0, 0x4c79, 0x9e, 0x3e, 0x56, 0xa4, 0xf1, 0x7c, 0xc4, 0x81);

//
// Specifies the number of responsiveness events required to enable
// responsiveness policy overrides.
//
// {3D44E256-7222-4415-A9ED-9C45FA3DD830}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD,
0x3d44e256, 0x7222, 0x4415, 0xa9, 0xed, 0x9c, 0x45, 0xfa, 0x3d, 0xd8, 0x30);

//
// Specifies the number of responsiveness events required to enable
// responsiveness policy overrides for efficiency class 1 processors.
//
// {3D44E256-7222-4415-A9ED-9C45FA3DD831}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD_1,
0x3d44e256, 0x7222, 0x4415, 0xa9, 0xed, 0x9c, 0x45, 0xfa, 0x3d, 0xd8, 0x31);

//
// Specifies the number of consecutive perf checks with a disable hint before
// responsivenss overrides will be disabled.
//
// {F565999F-3FB0-411a-A226-3F0198DEC130}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME,
0xf565999f, 0x3fb0, 0x411a, 0xa2, 0x26, 0x3f, 0x1, 0x98, 0xde, 0xc1, 0x30);

//
// Specifies the number of consecutive perf checks with a disable hint before
// responsivenss overrides will be disabled for efficiency class 1 processors.
//
// {F565999F-3FB0-411a-A226-3F0198DEC131}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME_1,
0xf565999f, 0x3fb0, 0x411a, 0xa2, 0x26, 0x3f, 0x1, 0x98, 0xde, 0xc1, 0x31);

//
// Specifies the number of consecutive perf checks with a enable hint before
// responsivenss overrides will be enabled.
//
// {3D915188-7830-49ae-A79A-0FB0A1E5A200}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME,
0x3d915188, 0x7830, 0x49ae, 0xa7, 0x9a, 0xf, 0xb0, 0xa1, 0xe5, 0xa2, 0x0);

//
// Specifies the number of consecutive perf checks with a enable hint before
// responsivenss overrides will be enabled for efficiency class 1 processors.
//
// {3D915188-7830-49ae-A79A-0FB0A1E5A201}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME_1,
0x3d915188, 0x7830, 0x49ae, 0xa7, 0x9a, 0xf, 0xb0, 0xa1, 0xe5, 0xa2, 0x1);

//
// Specifies the ceiling placed on EPP when responsiveness hints are enabled.
//
// {4427C73B-9756-4a5c-B84B-C7BDA79C7320}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING,
0x4427c73b, 0x9756, 0x4a5c, 0xb8, 0x4b, 0xc7, 0xbd, 0xa7, 0x9c, 0x73, 0x20);

//
// Specifies the ceiling placed on EPP when responsiveness hints are enabled
// for efficiency class 1 processors.
//
// {4427C73B-9756-4a5c-B84B-C7BDA79C7321}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING_1,
0x4427c73b, 0x9756, 0x4a5c, 0xb8, 0x4b, 0xc7, 0xbd, 0xa7, 0x9c, 0x73, 0x21);

//
// Specifies the floor placed on processor performance when responsiveness hints
// are enabled.
//
// {CE8E92EE-6A86-4572-BFE0-20C21D03CD40}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR,
0xce8e92ee, 0x6a86, 0x4572, 0xbf, 0xe0, 0x20, 0xc2, 0x1d, 0x3, 0xcd, 0x40);

//
// Specifies the floor placed on processor performance when responsiveness hints
// are enabled for efficiency class 1 processors.
//
// {CE8E92EE-6A86-4572-BFE0-20C21D03CD41}
DEFINE_GUID(GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR_1,
0xce8e92ee, 0x6a86, 0x4572, 0xbf, 0xe0, 0x20, 0xc2, 0x1d, 0x3, 0xcd, 0x41);

// Lock Console on Wake
// --------------------
//

// Specifies the behavior of the system when we wake from standby or
// hibernate.  If this is set, then we will cause the console to lock
// after we resume.
//
DEFINE_GUID( GUID_LOCK_CONSOLE_ON_WAKE, 0x0E796BDB, 0x100D, 0x47D6, 0xA2, 0xD5, 0xF7, 0xD2, 0xDA, 0xA5, 0x1F, 0x51 );

// Device idle characteristics
// ---------------------------
//
// Specifies whether to use the "performance" or "conservative" timeouts for
// device idle management.
//
// 4faab71a-92e5-4726-b531-224559672d19
//
DEFINE_GUID( GUID_DEVICE_IDLE_POLICY, 0x4faab71a, 0x92e5, 0x4726, 0xb5, 0x31, 0x22, 0x45, 0x59, 0x67, 0x2d, 0x19 );

#define POWER_DEVICE_IDLE_POLICY_PERFORMANCE  0
#define POWER_DEVICE_IDLE_POLICY_CONSERVATIVE 1

//
// Specifies standby connectivity preference.
//
// F15576E8-98B7-4186-B944-EAFA664402D9
DEFINE_GUID( GUID_CONNECTIVITY_IN_STANDBY, 0xF15576E8, 0x98B7, 0x4186, 0xB9, 0x44, 0xEA, 0xFA, 0x66, 0x44, 0x02, 0xD9 );

#define POWER_CONNECTIVITY_IN_STANDBY_DISABLED 0
#define POWER_CONNECTIVITY_IN_STANDBY_ENABLED 1
#define POWER_CONNECTIVITY_IN_STANDBY_SYSTEM_MANAGED 2

//
// Specifies the mode for disconnected standby.
//
// 68AFB2D9-EE95-47A8-8F50-4115088073B1
DEFINE_GUID( GUID_DISCONNECTED_STANDBY_MODE, 0x68AFB2D9, 0xEE95, 0x47A8, 0x8F, 0x50, 0x41, 0x15, 0x08, 0x80, 0x73, 0xB1 );

#define POWER_DISCONNECTED_STANDBY_MODE_NORMAL 0
#define POWER_DISCONNECTED_STANDBY_MODE_AGGRESSIVE 1

// AC/DC power source
// ------------------
//

// Specifies the power source for the system.  consumers may register for
// notification when the power source changes and will be notified with
// one of 3 values:
// 0 - Indicates the system is being powered by an AC power source.
// 1 - Indicates the system is being powered by a DC power source.
// 2 - Indicates the system is being powered by a short-term DC power
//     source.  For example, this would be the case if the system is
//     being powed by a short-term battery supply in a backing UPS
//     system.  When this value is recieved, the consumer should make
//     preparations for either a system hibernate or system shutdown.
//
// { 5D3E9A59-E9D5-4B00-A6BD-FF34FF516548 }
DEFINE_GUID( GUID_ACDC_POWER_SOURCE, 0x5D3E9A59, 0xE9D5, 0x4B00, 0xA6, 0xBD, 0xFF, 0x34, 0xFF, 0x51, 0x65, 0x48 );

// Lid state changes
// -----------------
//
// Specifies the current state of the lid (open or closed). The callback won't
// be called at all until a lid device is found and its current state is known.
//
// Values:
//
// 0 - closed
// 1 - opened
//
// { BA3E0F4D-B817-4094-A2D1-D56379E6A0F3 }
//

DEFINE_GUID( GUID_LIDSWITCH_STATE_CHANGE,  0xBA3E0F4D, 0xB817, 0x4094, 0xA2, 0xD1, 0xD5, 0x63, 0x79, 0xE6, 0xA0, 0xF3 );

// Lid state reliability
// -----------------
//
// Specifies the current reliability of lid state.
//
// Values:
//
// 0 - unreliable
// 1 - reliable
//
// {AE4C4FF1-D361-43F4-80AA-BBB6EB03DE94}
//

DEFINE_GUID( GUID_LIDSWITCH_STATE_RELIABILITY, 0xAE4C4FF1, 0xD361, 0x43F4, 0x80, 0xAA, 0xBB, 0xB6, 0xEB, 0x03, 0xDE, 0x94);

// Battery status changes
// ----------------------
//

// Specifies the percentage of battery life remaining.  The consumer
// may register for notification in order to track battery life in
// a fine-grained manner.
//
// Once registered, the consumer can expect to be notified as the battery
// life percentage changes.
//
// The consumer will recieve a value between 0 and 100 (inclusive) which
// indicates percent battery life remaining.
//
// { A7AD8041-B45A-4CAE-87A3-EECBB468A9E1 }
DEFINE_GUID( GUID_BATTERY_PERCENTAGE_REMAINING, 0xA7AD8041, 0xB45A, 0x4CAE, 0x87, 0xA3, 0xEE, 0xCB, 0xB4, 0x68, 0xA9, 0xE1 );

// Specifies change in number of batteries present on the system. The consumer
// may register for notification in order to track change in number of batteries
// available on a system.
//
// Once registered, the consumer can expect to be notified whenever the
// batteries are added or removed from the system.
//
// The consumer will recieve a value indicating number of batteries currently
// present on the system.
//
// {7D263F15-FCA4-49E5-854B-A9F2BFBD5C24}
DEFINE_GUID( GUID_BATTERY_COUNT, 0x7d263f15, 0xfca4, 0x49e5, 0x85, 0x4b, 0xa9, 0xf2, 0xbf, 0xbd, 0x5c, 0x24 );

//
// Global notification indicating to listeners user activity/presence accross
// all sessions in the system (Present, NotPresent, Inactive)
//
// {786E8A1D-B427-4344-9207-09E70BDCBEA9}
DEFINE_GUID( GUID_GLOBAL_USER_PRESENCE, 0x786e8a1d, 0xb427, 0x4344, 0x92, 0x7, 0x9, 0xe7, 0xb, 0xdc, 0xbe, 0xa9 );

//
// Session specific notification indicating to listeners whether or not the display
// related to the given session is on/off/dim
//
// N.B. This is a session-specific notification, sent only to interactive
//      session registrants. Session 0 and kernel mode consumers do not receive
//      this notification.
//
// {2B84C20E-AD23-4ddf-93DB-05FFBD7EFCA5}
DEFINE_GUID( GUID_SESSION_DISPLAY_STATUS, 0x2b84c20e, 0xad23, 0x4ddf, 0x93, 0xdb, 0x5, 0xff, 0xbd, 0x7e, 0xfc, 0xa5 );

//
// Session specific notification indicating to listeners user activity/presence
//(Present, NotPresent, Inactive)
//
// N.B. This is a session-specific notification, sent only to interactive
//      session registrants. Session 0 and kernel mode consumers do not receive
//      this notification.
// {3C0F4548-C03F-4c4d-B9F2-237EDE686376}
DEFINE_GUID( GUID_SESSION_USER_PRESENCE, 0x3c0f4548, 0xc03f, 0x4c4d, 0xb9, 0xf2, 0x23, 0x7e, 0xde, 0x68, 0x63, 0x76 );


// Notification to listeners that the system is fairly busy and won't be moving
// into an idle state any time soon.  This can be used as a hint to listeners
// that now might be a good time to do background tasks.
//
DEFINE_GUID( GUID_IDLE_BACKGROUND_TASK, 0x515C31D8, 0xF734, 0x163D, 0xA0, 0xFD, 0x11, 0xA0, 0x8C, 0x91, 0xE8, 0xF1 );

// Notification to listeners that the system is fairly busy and won't be moving
// into an idle state any time soon.  This can be used as a hint to listeners
// that now might be a good time to do background tasks.
//
// { CF23F240-2A54-48D8-B114-DE1518FF052E }
DEFINE_GUID( GUID_BACKGROUND_TASK_NOTIFICATION, 0xCF23F240, 0x2A54, 0x48D8, 0xB1, 0x14, 0xDE, 0x15, 0x18, 0xFF, 0x05, 0x2E );

// Define a GUID that will represent the action of a direct experience button
// on the platform.  Users will register for this DPPE setting and recieve
// notification when the h/w button is pressed.
//
// { 1A689231-7399-4E9A-8F99-B71F999DB3FA }
//
DEFINE_GUID( GUID_APPLAUNCH_BUTTON, 0x1A689231, 0x7399, 0x4E9A, 0x8F, 0x99, 0xB7, 0x1F, 0x99, 0x9D, 0xB3, 0xFA );

// PCI Express power settings
// ------------------------
//

// Specifies the subgroup which will contain all of the PCI Express
// settings for a single policy.
//
// {501a4d13-42af-4429-9fd1-a8218c268e20}
//
DEFINE_GUID( GUID_PCIEXPRESS_SETTINGS_SUBGROUP, 0x501a4d13, 0x42af,0x4429, 0x9f, 0xd1, 0xa8, 0x21, 0x8c, 0x26, 0x8e, 0x20 );

// Specifies the PCI Express ASPM power policy.
//
// {ee12f906-d277-404b-b6da-e5fa1a576df5}
//
DEFINE_GUID( GUID_PCIEXPRESS_ASPM_POLICY, 0xee12f906, 0xd277, 0x404b, 0xb6, 0xda, 0xe5, 0xfa, 0x1a, 0x57, 0x6d, 0xf5 );

// POWER Shutdown settings
// ------------------------
//

// Specifies if forced shutdown should be used for all button and lid initiated
// shutdown actions.
//
// {833a6b62-dfa4-46d1-82f8-e09e34d029d6}
//

DEFINE_GUID( GUID_ENABLE_SWITCH_FORCED_SHUTDOWN, 0x833a6b62, 0xdfa4, 0x46d1, 0x82, 0xf8, 0xe0, 0x9e, 0x34, 0xd0, 0x29, 0xd6 );

// Interrupt Steering power settings
// ------------------------
//

// {48672F38-7A9A-4bb2-8BF8-3D85BE19DE4E}
DEFINE_GUID(GUID_INTSTEER_SUBGROUP,
0x48672f38, 0x7a9a, 0x4bb2, 0x8b, 0xf8, 0x3d, 0x85, 0xbe, 0x19, 0xde, 0x4e);

// {2BFC24F9-5EA2-4801-8213-3DBAE01AA39D}
DEFINE_GUID(GUID_INTSTEER_MODE,
0x2bfc24f9, 0x5ea2, 0x4801, 0x82, 0x13, 0x3d, 0xba, 0xe0, 0x1a, 0xa3, 0x9d);

// {73CDE64D-D720-4bb2-A860-C755AFE77EF2}
DEFINE_GUID(GUID_INTSTEER_LOAD_PER_PROC_TRIGGER,
0x73cde64d, 0xd720, 0x4bb2, 0xa8, 0x60, 0xc7, 0x55, 0xaf, 0xe7, 0x7e, 0xf2);

// {D6BA4903-386F-4c2c-8ADB-5C21B3328D25}
DEFINE_GUID(GUID_INTSTEER_TIME_UNPARK_TRIGGER,
0xd6ba4903, 0x386f, 0x4c2c, 0x8a, 0xdb, 0x5c, 0x21, 0xb3, 0x32, 0x8d, 0x25);

// Graphics power settings
// ------------------------
//

// Specified the subgroup which contains all inbox graphics settings.
//
// {5FB4938D-1EE8-4b0f-9A3C-5036B0AB995C}
//
DEFINE_GUID(GUID_GRAPHICS_SUBGROUP, 0x5fb4938d, 0x1ee8, 0x4b0f, 0x9a, 0x3c, 0x50, 0x36, 0xb0, 0xab, 0x99, 0x5c);

// Specifies the GPU preference policy.
//
// {DD848B2A-8A5D-4451-9AE2-39CD41658F6C}
//
DEFINE_GUID(GUID_GPU_PREFERENCE_POLICY, 0xdd848b2a, 0x8a5d, 0x4451, 0x9a, 0xe2, 0x39, 0xcd, 0x41, 0x65, 0x8f, 0x6c);

// Other miscellaneous power notification GUIDs
// ------------------------
//

// Specifies whether mixed reality mode is engaged.
//
// {1E626B4E-CF04-4f8d-9CC7-C97C5B0F2391}
//

DEFINE_GUID(GUID_MIXED_REALITY_MODE,
0x1e626b4e, 0xcf04, 0x4f8d, 0x9c, 0xc7, 0xc9, 0x7c, 0x5b, 0xf, 0x23, 0x91);

// Specifies a change (start/end) in System Power Report's Active Session.
//
// {0E24CE38-C393-4742-BDB1-744F4B9EE08E}
//

DEFINE_GUID(GUID_SPR_ACTIVE_SESSION_CHANGE,
0xe24ce38, 0xc393, 0x4742, 0xbd, 0xb1, 0x74, 0x4f, 0x4b, 0x9e, 0xe0, 0x8e);


typedef enum _SYSTEM_POWER_STATE {
    PowerSystemUnspecified = 0,
    PowerSystemWorking     = 1,
    PowerSystemSleeping1   = 2,
    PowerSystemSleeping2   = 3,
    PowerSystemSleeping3   = 4,
    PowerSystemHibernate   = 5,
    PowerSystemShutdown    = 6,
    PowerSystemMaximum     = 7
} SYSTEM_POWER_STATE, *PSYSTEM_POWER_STATE;

#define POWER_SYSTEM_MAXIMUM 7

typedef enum {
    PowerActionNone = 0,
    PowerActionReserved,
    PowerActionSleep,
    PowerActionHibernate,
    PowerActionShutdown,
    PowerActionShutdownReset,
    PowerActionShutdownOff,
    PowerActionWarmEject,
    PowerActionDisplayOff
} POWER_ACTION, *PPOWER_ACTION;

typedef enum _DEVICE_POWER_STATE {
    PowerDeviceUnspecified = 0,
    PowerDeviceD0,
    PowerDeviceD1,
    PowerDeviceD2,
    PowerDeviceD3,
    PowerDeviceMaximum
} DEVICE_POWER_STATE, *PDEVICE_POWER_STATE;

typedef enum _MONITOR_DISPLAY_STATE {
    PowerMonitorOff = 0,
    PowerMonitorOn,
    PowerMonitorDim
} MONITOR_DISPLAY_STATE, *PMONITOR_DISPLAY_STATE;

typedef enum _USER_ACTIVITY_PRESENCE {
    PowerUserPresent = 0,
    PowerUserNotPresent,
    PowerUserInactive,
    PowerUserMaximum,
    PowerUserInvalid = PowerUserMaximum
} USER_ACTIVITY_PRESENCE, *PUSER_ACTIVITY_PRESENCE;

typedef enum _ENERGY_SAVER_STATUS {
    ENERGY_SAVER_OFF = 0,
    ENERGY_SAVER_STANDARD,
    ENERGY_SAVER_HIGH_SAVINGS
} ENERGY_SAVER_STATUS, *PENERGY_SAVER_STATUS;


#define ES_SYSTEM_REQUIRED   ((DWORD)0x00000001)
#define ES_DISPLAY_REQUIRED  ((DWORD)0x00000002)
#define ES_USER_PRESENT      ((DWORD)0x00000004)
#define ES_AWAYMODE_REQUIRED ((DWORD)0x00000040)
#define ES_CONTINUOUS        ((DWORD)0x80000000)

typedef DWORD EXECUTION_STATE, *PEXECUTION_STATE;

typedef enum {
    LT_DONT_CARE,
    LT_LOWEST_LATENCY
} LATENCY_TIME;

#define DIAGNOSTIC_REASON_VERSION    0

#define DIAGNOSTIC_REASON_SIMPLE_STRING             0x00000001
#define DIAGNOSTIC_REASON_DETAILED_STRING           0x00000002
#define DIAGNOSTIC_REASON_NOT_SPECIFIED             0x80000000
#define DIAGNOSTIC_REASON_INVALID_FLAGS             (~0x80000007)


//
// Defines for power request APIs
//

#define POWER_REQUEST_CONTEXT_VERSION               DIAGNOSTIC_REASON_VERSION

#define POWER_REQUEST_CONTEXT_SIMPLE_STRING         DIAGNOSTIC_REASON_SIMPLE_STRING
#define POWER_REQUEST_CONTEXT_DETAILED_STRING       DIAGNOSTIC_REASON_DETAILED_STRING

typedef enum _POWER_REQUEST_TYPE {
    PowerRequestDisplayRequired,
    PowerRequestSystemRequired,
    PowerRequestAwayModeRequired,
    PowerRequestExecutionRequired
} POWER_REQUEST_TYPE, *PPOWER_REQUEST_TYPE;

// end_ntminiport

#if (NTDDI_VERSION >= NTDDI_WINXP)

//-----------------------------------------------------------------------------
// Device Power Information
// Accessable via CM_Get_DevInst_Registry_Property_Ex(CM_DRP_DEVICE_POWER_DATA)
//-----------------------------------------------------------------------------

#define PDCAP_D0_SUPPORTED              0x00000001
#define PDCAP_D1_SUPPORTED              0x00000002
#define PDCAP_D2_SUPPORTED              0x00000004
#define PDCAP_D3_SUPPORTED              0x00000008
#define PDCAP_WAKE_FROM_D0_SUPPORTED    0x00000010
#define PDCAP_WAKE_FROM_D1_SUPPORTED    0x00000020
#define PDCAP_WAKE_FROM_D2_SUPPORTED    0x00000040
#define PDCAP_WAKE_FROM_D3_SUPPORTED    0x00000080
#define PDCAP_WARM_EJECT_SUPPORTED      0x00000100

typedef struct CM_Power_Data_s {
    DWORD               PD_Size;
    DEVICE_POWER_STATE  PD_MostRecentPowerState;
    DWORD               PD_Capabilities;
    DWORD               PD_D1Latency;
    DWORD               PD_D2Latency;
    DWORD               PD_D3Latency;
    DEVICE_POWER_STATE  PD_PowerStateMapping[POWER_SYSTEM_MAXIMUM];
    SYSTEM_POWER_STATE  PD_DeepestSystemWake;
} CM_POWER_DATA, *PCM_POWER_DATA;

#endif // (NTDDI_VERSION >= NTDDI_WINXP)

// begin_wdm

typedef enum {
    SystemPowerPolicyAc,
    SystemPowerPolicyDc,
    VerifySystemPolicyAc,
    VerifySystemPolicyDc,
    SystemPowerCapabilities,
    SystemBatteryState,
    SystemPowerStateHandler,
    ProcessorStateHandler,
    SystemPowerPolicyCurrent,
    AdministratorPowerPolicy,
    SystemReserveHiberFile,
    ProcessorInformation,
    SystemPowerInformation,
    ProcessorStateHandler2,
    LastWakeTime,                                   // Compare with KeQueryInterruptTime()
    LastSleepTime,                                  // Compare with KeQueryInterruptTime()
    SystemExecutionState,
    SystemPowerStateNotifyHandler,
    ProcessorPowerPolicyAc,
    ProcessorPowerPolicyDc,
    VerifyProcessorPowerPolicyAc,
    VerifyProcessorPowerPolicyDc,
    ProcessorPowerPolicyCurrent,
    SystemPowerStateLogging,
    SystemPowerLoggingEntry,
    SetPowerSettingValue,
    NotifyUserPowerSetting,
    PowerInformationLevelUnused0,
    SystemMonitorHiberBootPowerOff,
    SystemVideoState,
    TraceApplicationPowerMessage,
    TraceApplicationPowerMessageEnd,
    ProcessorPerfStates,
    ProcessorIdleStates,
    ProcessorCap,
    SystemWakeSource,
    SystemHiberFileInformation,
    TraceServicePowerMessage,
    ProcessorLoad,
    PowerShutdownNotification,
    MonitorCapabilities,
    SessionPowerInit,
    SessionDisplayState,
    PowerRequestCreate,
    PowerRequestAction,
    GetPowerRequestList,
    ProcessorInformationEx,
    NotifyUserModeLegacyPowerEvent,
    GroupPark,
    ProcessorIdleDomains,
    WakeTimerList,
    SystemHiberFileSize,
    ProcessorIdleStatesHv,
    ProcessorPerfStatesHv,
    ProcessorPerfCapHv,
    ProcessorSetIdle,
    LogicalProcessorIdling,
    UserPresence,                                   // Deprecated
    PowerSettingNotificationName,
    GetPowerSettingValue,
    IdleResiliency,
    SessionRITState,
    SessionConnectNotification,
    SessionPowerCleanup,
    SessionLockState,
    SystemHiberbootState,
    PlatformInformation,
    PdcInvocation,
    MonitorInvocation,
    FirmwareTableInformationRegistered,
    SetShutdownSelectedTime,
    SuspendResumeInvocation,                        // Deprecated
    PlmPowerRequestCreate,
    ScreenOff,
    CsDeviceNotification,
    PlatformRole,
    LastResumePerformance,
    DisplayBurst,
    ExitLatencySamplingPercentage,
    RegisterSpmPowerSettings,
    PlatformIdleStates,
    ProcessorIdleVeto,                              // Deprecated.
    PlatformIdleVeto,                               // Deprecated.
    SystemBatteryStatePrecise,
    ThermalEvent,
    PowerRequestActionInternal,
    BatteryDeviceState,
    PowerInformationInternal,
    ThermalStandby,
    SystemHiberFileType,
    PhysicalPowerButtonPress,
    QueryPotentialDripsConstraint,
    EnergyTrackerCreate,
    EnergyTrackerQuery,
    UpdateBlackBoxRecorder,
    SessionAllowExternalDmaDevices,
    SendSuspendResumeNotification,
    BlackBoxRecorderDirectAccessBuffer,
    SystemPowerSourceState,
    PowerInformationLevelMaximum
} POWER_INFORMATION_LEVEL;

//
// User Presence Values
//

typedef enum {
    UserNotPresent = 0,
    UserPresent = 1,
    UserUnknown = 0xff
} POWER_USER_PRESENCE_TYPE, *PPOWER_USER_PRESENCE_TYPE;

typedef struct _POWER_USER_PRESENCE {
    POWER_USER_PRESENCE_TYPE UserPresence;
} POWER_USER_PRESENCE, *PPOWER_USER_PRESENCE;

//
// Session Connect/Disconnect
//
typedef struct _POWER_SESSION_CONNECT {
    BOOLEAN Connected;  // TRUE - connected, FALSE - disconnected
    BOOLEAN Console;    // TRUE - console, FALSE - TS (not used for Connected = FALSE)
} POWER_SESSION_CONNECT, *PPOWER_SESSION_CONNECT;

typedef struct _POWER_SESSION_TIMEOUTS {
    DWORD InputTimeout;
    DWORD DisplayTimeout;
} POWER_SESSION_TIMEOUTS, *PPOWER_SESSION_TIMEOUTS;

//
// Session RIT State
//
typedef struct _POWER_SESSION_RIT_STATE {
    BOOLEAN Active;  // TRUE - RIT input received, FALSE - RIT timeout
    DWORD64 LastInputTime; // last input time held for this session
} POWER_SESSION_RIT_STATE, *PPOWER_SESSION_RIT_STATE;

//
// Winlogon notifications
//
typedef struct _POWER_SESSION_WINLOGON {
    DWORD SessionId; // the Win32k session identifier
    BOOLEAN Console; // TRUE - for console session, FALSE - for remote session
    BOOLEAN Locked; // TRUE - lock, FALSE - unlock
} POWER_SESSION_WINLOGON, *PPOWER_SESSION_WINLOGON;

//
// Winlogon notification to unblock external DMA devices.
//
typedef struct _POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    BOOLEAN IsAllowed;
} POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES, *PPOWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES;

//
// Idle resiliency
//
typedef struct _POWER_IDLE_RESILIENCY {
    DWORD CoalescingTimeout;
    DWORD IdleResiliencyPeriod;
} POWER_IDLE_RESILIENCY, *PPOWER_IDLE_RESILIENCY;

//
// Monitor on/off reasons
//
// N.B. Update power-event mapping when adding new events.
//
typedef enum {
    MonitorRequestReasonUnknown,
    MonitorRequestReasonPowerButton,
    MonitorRequestReasonRemoteConnection,
    MonitorRequestReasonScMonitorpower,
    MonitorRequestReasonUserInput,
    MonitorRequestReasonAcDcDisplayBurst,
    MonitorRequestReasonUserDisplayBurst,
    MonitorRequestReasonPoSetSystemState,
    MonitorRequestReasonSetThreadExecutionState,
    MonitorRequestReasonFullWake,
    MonitorRequestReasonSessionUnlock,
    MonitorRequestReasonScreenOffRequest,
    MonitorRequestReasonIdleTimeout,
    MonitorRequestReasonPolicyChange,
    MonitorRequestReasonSleepButton,
    MonitorRequestReasonLid,
    MonitorRequestReasonBatteryCountChange,
    MonitorRequestReasonGracePeriod,
    MonitorRequestReasonPnP,
    MonitorRequestReasonDP,
    MonitorRequestReasonSxTransition,
    MonitorRequestReasonSystemIdle,
    MonitorRequestReasonNearProximity,
    MonitorRequestReasonThermalStandby,
    MonitorRequestReasonResumePdc,
    MonitorRequestReasonResumeS4,
    MonitorRequestReasonTerminal,
    MonitorRequestReasonPdcSignal,
    MonitorRequestReasonAcDcDisplayBurstSuppressed,
    MonitorRequestReasonSystemStateEntered, // When CS exit happens because system
                                            // transition to S4/S5, please note this
                                            // reason is different than ReasonSxTransition.
    MonitorRequestReasonWinrt,
    MonitorRequestReasonUserInputKeyboard,
    MonitorRequestReasonUserInputMouse,
    MonitorRequestReasonUserInputTouchpad,
    MonitorRequestReasonUserInputPen,
    MonitorRequestReasonUserInputAccelerometer,
    MonitorRequestReasonUserInputHid,
    MonitorRequestReasonUserInputPoUserPresent,
    MonitorRequestReasonUserInputSessionSwitch,
    MonitorRequestReasonUserInputInitialization,
    MonitorRequestReasonPdcSignalWindowsMobilePwrNotif,         // PDC_SIGNAL_PROVIDER_PWRNOTIF_SVC
    MonitorRequestReasonPdcSignalWindowsMobileShell,            // PDC_SIGNAL_PROVIDER_UM_CS_CONTROL
    MonitorRequestReasonPdcSignalHeyCortana,                    // PDC_SIGNAL_PROVIDER_HEY_CORTANA
    MonitorRequestReasonPdcSignalHolographicShell,              // PDC_SIGNAL_PROVIDER_HOLOSI_CRITICAL_BATTERY_WAKE
    MonitorRequestReasonPdcSignalFingerprint,                   // PDC_SIGNAL_PROVIDER_WINBIO
    MonitorRequestReasonDirectedDrips,
    MonitorRequestReasonDim,
    MonitorRequestReasonBuiltinPanel,
    MonitorRequestReasonDisplayRequiredUnDim,
    MonitorRequestReasonBatteryCountChangeSuppressed,
    MonitorRequestReasonResumeModernStandby,
    MonitorRequestReasonTerminalInit,
    MonitorRequestReasonPdcSignalSensorsHumanPresence,          // PDC_SIGNAL_PROVIDER_SENSORS_HUMAN_PRESENCE_MONITOR
    MonitorRequestReasonBatteryPreCritical,
    MonitorRequestReasonUserInputTouch,
    MonitorRequestReasonAusterityBatteryDrain,
    MonitorRequestReasonDozeRestrictedStandby,
    MonitorRequestReasonSmartRestrictedStandby,
    MonitorRequestReasonMax
} POWER_MONITOR_REQUEST_REASON;

typedef enum _POWER_MONITOR_REQUEST_TYPE {
    MonitorRequestTypeOff,
    MonitorRequestTypeOnAndPresent,
    MonitorRequestTypeToggleOn
} POWER_MONITOR_REQUEST_TYPE;

//
// Monitor invocation
//
typedef struct _POWER_MONITOR_INVOCATION {
    BOOLEAN Console;
    POWER_MONITOR_REQUEST_REASON RequestReason;
} POWER_MONITOR_INVOCATION, *PPOWER_MONITOR_INVOCATION;

#if (NTDDI_VERSION >= NTDDI_WIN11_GA)

//
// Power Limit Interfaces
//

typedef enum _POWER_LIMIT_TYPES {
    PowerLimitContinuous = 0,
    PowerLimitType1 = PowerLimitContinuous,
    PowerLimitBurst,
    PowerLimitType2 = PowerLimitBurst,
    PowerLimitRapid,
    PowerLimitType3 = PowerLimitRapid,
    PowerLimitPreemptive,
    PowerLimitType4 = PowerLimitPreemptive,
    PowerLimitPreemptiveOffset,
    PowerLimitTypeMax
} POWER_LIMIT_TYPES, *PPOWER_LIMIT_TYPES;

typedef struct _POWER_LIMIT_ATTRIBUTES {

    //
    // IDs of this power limit.
    //

    POWER_LIMIT_TYPES   Type;
    DWORD               DomainId;

    //
    // Attributes of this power limit.
    //

    DWORD               MaxValue;
    DWORD               MinValue;
    DWORD               MinTimeParameter;
    DWORD               MaxTimeParameter;
    DWORD               DefaultACValue;
    DWORD               DefaultDCValue;

    union {
        struct {
            DWORD       SupportTimeParameter : 1;
            DWORD       Reserved : 31;
        };

        DWORD           AsUlong;
    } Flags;
} POWER_LIMIT_ATTRIBUTES, *PPOWER_LIMIT_ATTRIBUTES;

typedef struct _POWER_LIMIT_VALUE {
    POWER_LIMIT_TYPES Type;
    DWORD DomainId;
    DWORD TargetValue;
    DWORD TimeParameter;
} POWER_LIMIT_VALUE, *PPOWER_LIMIT_VALUE;

#define POWER_LIMIT_VALUE_NO_CONTROL            DWORD_MAX

#endif // NTDDI_VERSION >= NTDDI_WIN11_GA

//
// Last resume performance structure
//

typedef struct _RESUME_PERFORMANCE {
    DWORD PostTimeMs;
    ULONGLONG TotalResumeTimeMs;
    ULONGLONG ResumeCompleteTimestamp;
} RESUME_PERFORMANCE, *PRESUME_PERFORMANCE;

//
// Power Setting definitions
//

typedef enum {
    PoAc,
    PoDc,
    PoHot,
    PoConditionMaximum
} SYSTEM_POWER_CONDITION;

typedef struct {

    //
    // Version of this structure.  Currently should be set to
    // POWER_SETTING_VALUE_VERSION.
    //
    DWORD       Version;


    //
    // GUID representing the power setting being applied.
    //
    GUID        Guid;


    //
    // What power state should this setting be applied to?  E.g.
    // AC, DC, thermal, ...
    //
    SYSTEM_POWER_CONDITION PowerCondition;

    //
    // Length (in bytes) of the 'Data' member.
    //
    DWORD       DataLength;

    //
    // Data which contains the actual setting value.
    //
    BYTE    Data[ANYSIZE_ARRAY];
} SET_POWER_SETTING_VALUE, *PSET_POWER_SETTING_VALUE;

#define POWER_SETTING_VALUE_VERSION (0x1)

typedef struct {
    GUID Guid;
} NOTIFY_USER_POWER_SETTING, *PNOTIFY_USER_POWER_SETTING;

//
// Package definition for an experience button device notification.  When
// someone registers for GUID_EXPERIENCE_BUTTON, this is the definition of
// the setting data they'll get.
//
typedef struct _APPLICATIONLAUNCH_SETTING_VALUE {

    //
    // System time when the most recent button press ocurred.  Note that this is
    // specified in 100ns internvals since January 1, 1601.
    //
    LARGE_INTEGER       ActivationTime;

    //
    // Reserved for internal use.
    //
    DWORD               Flags;

    //
    // which instance of this device was pressed?
    //
    DWORD               ButtonInstanceID;


} APPLICATIONLAUNCH_SETTING_VALUE, *PAPPLICATIONLAUNCH_SETTING_VALUE;

//
// define platform roles
//

typedef enum _POWER_PLATFORM_ROLE {
    PlatformRoleUnspecified = 0,
    PlatformRoleDesktop,
    PlatformRoleMobile,
    PlatformRoleWorkstation,
    PlatformRoleEnterpriseServer,
    PlatformRoleSOHOServer,
    PlatformRoleAppliancePC,
    PlatformRolePerformanceServer, // v1 last supported
    PlatformRoleSlate,             // v2 last supported
    PlatformRoleMaximum
} POWER_PLATFORM_ROLE, *PPOWER_PLATFORM_ROLE;

#define POWER_PLATFORM_ROLE_V1     (0x00000001)
#define POWER_PLATFORM_ROLE_V1_MAX (PlatformRolePerformanceServer + 1)

#define POWER_PLATFORM_ROLE_V2     (0x00000002)
#define POWER_PLATFORM_ROLE_V2_MAX (PlatformRoleSlate + 1)

#if (NTDDI_VERSION >= NTDDI_WIN8)

#define POWER_PLATFORM_ROLE_VERSION     POWER_PLATFORM_ROLE_V2
#define POWER_PLATFORM_ROLE_VERSION_MAX POWER_PLATFORM_ROLE_V2_MAX

#else

#define POWER_PLATFORM_ROLE_VERSION     POWER_PLATFORM_ROLE_V1
#define POWER_PLATFORM_ROLE_VERSION_MAX POWER_PLATFORM_ROLE_V1_MAX

#endif

typedef struct _POWER_PLATFORM_INFORMATION {
    BOOLEAN AoAc;
} POWER_PLATFORM_INFORMATION, *PPOWER_PLATFORM_INFORMATION;

//
// Enum which defines the effective altitude of a power setting.
//

typedef enum POWER_SETTING_ALTITUDE {
    ALTITUDE_GROUP_POLICY,
    ALTITUDE_USER,
    ALTITUDE_RUNTIME_OVERRIDE,
    ALTITUDE_PROVISIONING,
    ALTITUDE_OEM_CUSTOMIZATION,
    ALTITUDE_INTERNAL_OVERRIDE,
    ALTITUDE_OS_DEFAULT,
} POWER_SETTING_ALTITUDE, *PPOWER_SETTING_ALTITUDE;

//
// System power manager capabilities
//

#if (NTDDI_VERSION >= NTDDI_WINXP) || !defined(_BATCLASS_)
typedef struct {
    DWORD       Granularity;
    DWORD       Capacity;
} BATTERY_REPORTING_SCALE, *PBATTERY_REPORTING_SCALE;
#endif // (NTDDI_VERSION >= NTDDI_WINXP) || !defined(_BATCLASS_)

//

typedef struct {
    DWORD   Frequency;
    DWORD   Flags;
    DWORD   PercentFrequency;
} PPM_WMI_LEGACY_PERFSTATE, *PPPM_WMI_LEGACY_PERFSTATE;

typedef struct {
    DWORD Latency;
    DWORD Power;
    DWORD TimeCheck;
    BYTE  PromotePercent;
    BYTE  DemotePercent;
    BYTE  StateType;
    BYTE  Reserved;
    DWORD StateFlags;
    DWORD Context;
    DWORD IdleHandler;
    DWORD Reserved1;            // reserved for future use
} PPM_WMI_IDLE_STATE, *PPPM_WMI_IDLE_STATE;

typedef struct {
    DWORD Type;
    DWORD Count;
    DWORD TargetState;          // current idle state
    DWORD OldState;             // previous idle state
    DWORD64 TargetProcessors;
    PPM_WMI_IDLE_STATE State[ANYSIZE_ARRAY];
} PPM_WMI_IDLE_STATES, *PPPM_WMI_IDLE_STATES;

typedef struct {
    DWORD Type;
    DWORD Count;
    DWORD TargetState;          // current idle state
    DWORD OldState;             // previous idle state
    PVOID TargetProcessors;
    PPM_WMI_IDLE_STATE State[ANYSIZE_ARRAY];
} PPM_WMI_IDLE_STATES_EX, *PPPM_WMI_IDLE_STATES_EX;

typedef struct {
    DWORD Frequency;            // in Mhz
    DWORD Power;                // in milliwatts
    BYTE  PercentFrequency;
    BYTE  IncreaseLevel;        // goto higher state
    BYTE  DecreaseLevel;        // goto lower state
    BYTE  Type;                 // performance or throttle
    DWORD IncreaseTime;         // in tick counts
    DWORD DecreaseTime;         // in tick counts
    DWORD64 Control;            // control value
    DWORD64 Status;             // control value
    DWORD HitCount;
    DWORD Reserved1;            // reserved for future use
    DWORD64 Reserved2;
    DWORD64 Reserved3;
} PPM_WMI_PERF_STATE, *PPPM_WMI_PERF_STATE;

typedef struct {
    DWORD Count;
    DWORD MaxFrequency;
    DWORD CurrentState;         // current state
    DWORD MaxPerfState;         // fastest state considering policy restrictions
    DWORD MinPerfState;         // slowest state considering policy restrictions
    DWORD LowestPerfState;      // slowest perf state, fixed, aka the "knee"
    DWORD ThermalConstraint;
    BYTE  BusyAdjThreshold;
    BYTE  PolicyType;           // domain coordination
    BYTE  Type;
    BYTE  Reserved;
    DWORD TimerInterval;
    DWORD64 TargetProcessors;   // domain affinity
    DWORD PStateHandler;
    DWORD PStateContext;
    DWORD TStateHandler;
    DWORD TStateContext;
    DWORD FeedbackHandler;
    DWORD Reserved1;
    DWORD64 Reserved2;
    PPM_WMI_PERF_STATE State[ANYSIZE_ARRAY];
} PPM_WMI_PERF_STATES, *PPPM_WMI_PERF_STATES;

typedef struct {
    DWORD Count;
    DWORD MaxFrequency;
    DWORD CurrentState;         // current state
    DWORD MaxPerfState;         // fastest state considering policy restrictions
    DWORD MinPerfState;         // slowest state considering policy restrictions
    DWORD LowestPerfState;      // slowest perf state, fixed, aka the "knee"
    DWORD ThermalConstraint;
    BYTE  BusyAdjThreshold;
    BYTE  PolicyType;           // domain coordination
    BYTE  Type;
    BYTE  Reserved;
    DWORD TimerInterval;
    PVOID TargetProcessors;     // domain affinity
    DWORD PStateHandler;
    DWORD PStateContext;
    DWORD TStateHandler;
    DWORD TStateContext;
    DWORD FeedbackHandler;
    DWORD Reserved1;
    DWORD64 Reserved2;
    PPM_WMI_PERF_STATE State[ANYSIZE_ARRAY];
} PPM_WMI_PERF_STATES_EX, *PPPM_WMI_PERF_STATES_EX;

//
// Legacy processor idle accounting.
//

#define PROC_IDLE_BUCKET_COUNT 6

typedef struct {
    DWORD IdleTransitions;
    DWORD FailedTransitions;
    DWORD InvalidBucketIndex;
    DWORD64 TotalTime;
    DWORD IdleTimeBuckets[PROC_IDLE_BUCKET_COUNT];
} PPM_IDLE_STATE_ACCOUNTING, *PPPM_IDLE_STATE_ACCOUNTING;

typedef struct {
    DWORD StateCount;
    DWORD TotalTransitions;
    DWORD ResetCount;
    DWORD64 StartTime;
    PPM_IDLE_STATE_ACCOUNTING State[ANYSIZE_ARRAY];
} PPM_IDLE_ACCOUNTING, *PPPM_IDLE_ACCOUNTING;

//
// Processor idle accounting.
//

#define PROC_IDLE_BUCKET_COUNT_EX 16

typedef struct {
    DWORD64 TotalTimeUs;
    DWORD MinTimeUs;
    DWORD MaxTimeUs;
    DWORD Count;
} PPM_IDLE_STATE_BUCKET_EX, *PPPM_IDLE_STATE_BUCKET_EX;

typedef struct {
    DWORD64 TotalTime;
    DWORD IdleTransitions;
    DWORD FailedTransitions;
    DWORD InvalidBucketIndex;
    DWORD MinTimeUs;
    DWORD MaxTimeUs;
    DWORD CancelledTransitions;
    PPM_IDLE_STATE_BUCKET_EX IdleTimeBuckets[PROC_IDLE_BUCKET_COUNT_EX];
} PPM_IDLE_STATE_ACCOUNTING_EX, *PPPM_IDLE_STATE_ACCOUNTING_EX;

typedef struct {
    DWORD StateCount;
    DWORD TotalTransitions;
    DWORD ResetCount;
    DWORD AbortCount;
    DWORD64 StartTime;
    _Field_size_(StateCount) PPM_IDLE_STATE_ACCOUNTING_EX State[ANYSIZE_ARRAY];
} PPM_IDLE_ACCOUNTING_EX, *PPPM_IDLE_ACCOUNTING_EX;

//
// Definitions of coordination types for _PSD, _TSD, and _CSD BIOS objects from
// the Acpi 3.0 specification
//

#define ACPI_PPM_SOFTWARE_ALL     0xFC
#define ACPI_PPM_SOFTWARE_ANY     0xFD
#define ACPI_PPM_HARDWARE_ALL     0xFE

//
// Definition of Microsoft PPM coordination types.
//

#define MS_PPM_SOFTWARE_ALL       0x1

//
// Processor firmware rundown feature bit definitions.
//

#define PPM_FIRMWARE_ACPI1C2      0x00000001
#define PPM_FIRMWARE_ACPI1C3      0x00000002
#define PPM_FIRMWARE_ACPI1TSTATES 0x00000004
#define PPM_FIRMWARE_CST          0x00000008
#define PPM_FIRMWARE_CSD          0x00000010
#define PPM_FIRMWARE_PCT          0x00000020
#define PPM_FIRMWARE_PSS          0x00000040
#define PPM_FIRMWARE_XPSS         0x00000080
#define PPM_FIRMWARE_PPC          0x00000100
#define PPM_FIRMWARE_PSD          0x00000200
#define PPM_FIRMWARE_PTC          0x00000400
#define PPM_FIRMWARE_TSS          0x00000800
#define PPM_FIRMWARE_TPC          0x00001000
#define PPM_FIRMWARE_TSD          0x00002000
#define PPM_FIRMWARE_PCCH         0x00004000
#define PPM_FIRMWARE_PCCP         0x00008000
#define PPM_FIRMWARE_OSC          0x00010000
#define PPM_FIRMWARE_PDC          0x00020000
#define PPM_FIRMWARE_CPC          0x00040000
#define PPM_FIRMWARE_LPI          0x00080000

//
// Processor performance and idle controls implementations.
//

#define PPM_PERFORMANCE_IMPLEMENTATION_NONE      0x00000000
#define PPM_PERFORMANCE_IMPLEMENTATION_PSTATES   0x00000001
#define PPM_PERFORMANCE_IMPLEMENTATION_PCCV1     0x00000002
#define PPM_PERFORMANCE_IMPLEMENTATION_CPPC      0x00000003
#define PPM_PERFORMANCE_IMPLEMENTATION_PEP       0x00000004

#define PPM_IDLE_IMPLEMENTATION_NONE             0x00000000
#define PPM_IDLE_IMPLEMENTATION_CSTATES          0x00000001
#define PPM_IDLE_IMPLEMENTATION_PEP              0x00000002
#define PPM_IDLE_IMPLEMENTATION_MICROPEP         0x00000003
#define PPM_IDLE_IMPLEMENTATION_LPISTATES        0x00000004

//
// Processor Power Management WMI interface.
//

// {A5B32DDD-7F39-4abc-B892-900E43B59EBB}
DEFINE_GUID(PPM_PERFSTATE_CHANGE_GUID,
0xa5b32ddd, 0x7f39, 0x4abc, 0xb8, 0x92, 0x90, 0xe, 0x43, 0xb5, 0x9e, 0xbb);

// {995e6b7f-d653-497a-b978-36a30c29bf01}
DEFINE_GUID(PPM_PERFSTATE_DOMAIN_CHANGE_GUID,
0x995e6b7f, 0xd653, 0x497a, 0xb9, 0x78, 0x36, 0xa3, 0xc, 0x29, 0xbf, 0x1);

// {4838fe4f-f71c-4e51-9ecc-8430a7ac4c6c}
DEFINE_GUID(PPM_IDLESTATE_CHANGE_GUID,
0x4838fe4f, 0xf71c, 0x4e51, 0x9e, 0xcc, 0x84, 0x30, 0xa7, 0xac, 0x4c, 0x6c);

// {5708cc20-7d40-4bf4-b4aa-2b01338d0126}
DEFINE_GUID(PPM_PERFSTATES_DATA_GUID,
0x5708cc20, 0x7d40, 0x4bf4, 0xb4, 0xaa, 0x2b, 0x01, 0x33, 0x8d, 0x01, 0x26);

// {ba138e10-e250-4ad7-8616-cf1a7ad410e7}
DEFINE_GUID(PPM_IDLESTATES_DATA_GUID,
0xba138e10, 0xe250, 0x4ad7, 0x86, 0x16, 0xcf, 0x1a, 0x7a, 0xd4, 0x10, 0xe7);

// {e2a26f78-ae07-4ee0-a30f-ce354f5a94cd}
DEFINE_GUID(PPM_IDLE_ACCOUNTING_GUID,
0xe2a26f78, 0xae07, 0x4ee0, 0xa3, 0x0f, 0xce, 0x54, 0xf5, 0x5a, 0x94, 0xcd);

// {d67abd39-81f8-4a5e-8152-72e31ec912ee}
DEFINE_GUID(PPM_IDLE_ACCOUNTING_EX_GUID,
0xd67abd39, 0x81f8, 0x4a5e, 0x81, 0x52, 0x72, 0xe3, 0x1e, 0xc9, 0x12, 0xee);

// {a852c2c8-1a4c-423b-8c2c-f30d82931a88}
DEFINE_GUID(PPM_THERMALCONSTRAINT_GUID,
0xa852c2c8, 0x1a4c, 0x423b, 0x8c, 0x2c, 0xf3, 0x0d, 0x82, 0x93, 0x1a, 0x88);

// {7fd18652-0cfe-40d2-b0a1-0b066a87759e}
DEFINE_GUID(PPM_PERFMON_PERFSTATE_GUID,
0x7fd18652, 0xcfe, 0x40d2, 0xb0, 0xa1, 0xb, 0x6, 0x6a, 0x87, 0x75, 0x9e);

// {48f377b8-6880-4c7b-8bdc-380176c6654d}
DEFINE_GUID(PPM_THERMAL_POLICY_CHANGE_GUID,
0x48f377b8, 0x6880, 0x4c7b, 0x8b, 0xdc, 0x38, 0x1, 0x76, 0xc6, 0x65, 0x4d);


typedef struct {
    DWORD State;
    DWORD Status;
    DWORD Latency;
    DWORD Speed;
    DWORD Processor;
} PPM_PERFSTATE_EVENT, *PPPM_PERFSTATE_EVENT;

typedef struct {
    DWORD State;
    DWORD Latency;
    DWORD Speed;
    DWORD64 Processors;
} PPM_PERFSTATE_DOMAIN_EVENT, *PPPM_PERFSTATE_DOMAIN_EVENT;

typedef struct {
    DWORD NewState;
    DWORD OldState;
    DWORD64 Processors;
} PPM_IDLESTATE_EVENT, *PPPM_IDLESTATE_EVENT;

typedef struct {
    DWORD ThermalConstraint;
    DWORD64 Processors;
} PPM_THERMALCHANGE_EVENT, *PPPM_THERMALCHANGE_EVENT;

#pragma warning(push)
#pragma warning(disable:4121)

typedef struct {
    BYTE  Mode;
    DWORD64 Processors;
} PPM_THERMAL_POLICY_EVENT, *PPPM_THERMAL_POLICY_EVENT;

#pragma warning(pop)

// Power Policy Management interfaces
//

typedef struct {
    POWER_ACTION    Action;
    DWORD           Flags;
    DWORD           EventCode;
} POWER_ACTION_POLICY, *PPOWER_ACTION_POLICY;

// POWER_ACTION_POLICY->Flags:
#define POWER_ACTION_QUERY_ALLOWED      0x00000001
#define POWER_ACTION_UI_ALLOWED         0x00000002
#define POWER_ACTION_OVERRIDE_APPS      0x00000004
#define POWER_ACTION_HIBERBOOT          0x00000008
#define POWER_ACTION_USER_NOTIFY        0x00000010  // Indicate User-mode of an impending action.
#define POWER_ACTION_DOZE_TO_HIBERNATE  0x00000020
#define POWER_ACTION_ACPI_CRITICAL      0x01000000
#define POWER_ACTION_ACPI_USER_NOTIFY   0x02000000
#define POWER_ACTION_DIRECTED_DRIPS     0x04000000
#define POWER_ACTION_PSEUDO_TRANSITION  0x08000000
#define POWER_ACTION_LIGHTEST_FIRST     0x10000000
#define POWER_ACTION_LOCK_CONSOLE       0x20000000
#define POWER_ACTION_DISABLE_WAKES      0x40000000
#define POWER_ACTION_CRITICAL           0x80000000

// POWER_ACTION_POLICY->EventCode flags
#define POWER_LEVEL_USER_NOTIFY_TEXT      0x00000001
#define POWER_LEVEL_USER_NOTIFY_SOUND     0x00000002
#define POWER_LEVEL_USER_NOTIFY_EXEC      0x00000004
#define POWER_USER_NOTIFY_BUTTON          0x00000008
#define POWER_USER_NOTIFY_SHUTDOWN        0x00000010 // Application and Services are intimated of shutdown.
#define POWER_USER_NOTIFY_FORCED_SHUTDOWN 0x00000020 // Immediate shutdown - Application and Services are not intimated.
#define POWER_FORCE_TRIGGER_RESET         0x80000000

// Note: for battery alarm EventCodes, the ID of the battery alarm << 16 is ORed
// into the flags.  For example: DISCHARGE_POLICY_LOW << 16

//
// The GUID_BATTERY_DISCHARGE_FLAGS_x power settings use a subset of EventCode
// flags.  The POWER_FORCE_TRIGGER_RESET flag doesn't make sense for a battery
// alarm so it is overloaded for other purposes (gerneral enable/disable).
#define BATTERY_DISCHARGE_FLAGS_EVENTCODE_MASK  0x00000007
#define BATTERY_DISCHARGE_FLAGS_ENABLE  0x80000000

// system battery drain policies
typedef struct {
    BOOLEAN                 Enable;
    BYTE                    Spare[3];
    DWORD                   BatteryLevel;
    POWER_ACTION_POLICY     PowerPolicy;
    SYSTEM_POWER_STATE      MinSystemState;
} SYSTEM_POWER_LEVEL, *PSYSTEM_POWER_LEVEL;

// Discharge policy constants
#define NUM_DISCHARGE_POLICIES      4
#define DISCHARGE_POLICY_CRITICAL   0
#define DISCHARGE_POLICY_LOW        1


// system power policies
typedef struct _SYSTEM_POWER_POLICY {
    DWORD                   Revision;       // 1

    // events
    POWER_ACTION_POLICY     PowerButton;
    POWER_ACTION_POLICY     SleepButton;
    POWER_ACTION_POLICY     LidClose;
    SYSTEM_POWER_STATE      LidOpenWake;
    DWORD                   Reserved;

    // "system idle" detection
    POWER_ACTION_POLICY     Idle;
    DWORD                   IdleTimeout;
    BYTE                    IdleSensitivity;

    BYTE                    DynamicThrottle;
    BYTE                    Spare2[2];

    // meaning of power action "sleep"
    SYSTEM_POWER_STATE      MinSleep;
    SYSTEM_POWER_STATE      MaxSleep;
    SYSTEM_POWER_STATE      ReducedLatencySleep;
    DWORD                   WinLogonFlags;

    DWORD                   Spare3;

    // parameters for dozing
    //
    DWORD                   DozeS4Timeout;

    // battery policies
    DWORD                   BroadcastCapacityResolution;
    SYSTEM_POWER_LEVEL      DischargePolicy[NUM_DISCHARGE_POLICIES];

    // video policies
    DWORD                   VideoTimeout;
    BOOLEAN                 VideoDimDisplay;
    DWORD                   VideoReserved[3];

    // hard disk policies
    DWORD                   SpindownTimeout;

    // processor policies
    BOOLEAN                 OptimizeForPower;
    BYTE                    FanThrottleTolerance;
    BYTE                    ForcedThrottle;
    BYTE                    MinThrottle;
    POWER_ACTION_POLICY     OverThrottled;

} SYSTEM_POWER_POLICY, *PSYSTEM_POWER_POLICY;


// processor power policy state

//
// Processor Idle State Policy.
//

#define PROCESSOR_IDLESTATE_POLICY_COUNT 0x3

typedef struct {
    DWORD TimeCheck;
    BYTE  DemotePercent;
    BYTE  PromotePercent;
    BYTE  Spare[2];
} PROCESSOR_IDLESTATE_INFO, *PPROCESSOR_IDLESTATE_INFO;

typedef struct {
    WORD   Revision;
    union {
        WORD   AsWORD  ;
        struct {
            WORD   AllowScaling : 1;
            WORD   Disabled : 1;
            WORD   Reserved : 14;
        } DUMMYSTRUCTNAME;
    } Flags;

    DWORD PolicyCount;
    PROCESSOR_IDLESTATE_INFO Policy[PROCESSOR_IDLESTATE_POLICY_COUNT];
} PROCESSOR_IDLESTATE_POLICY, *PPROCESSOR_IDLESTATE_POLICY;

//
// Legacy Processor Policy.  This is only provided to allow legacy
// applications to compile.  New applications must use
// PROCESSOR_IDLESTATE_POLICY.
//

#define PO_THROTTLE_NONE            0
#define PO_THROTTLE_CONSTANT        1
#define PO_THROTTLE_DEGRADE         2
#define PO_THROTTLE_ADAPTIVE        3
#define PO_THROTTLE_MAXIMUM         4   // not a policy, just a limit


typedef struct _PROCESSOR_POWER_POLICY_INFO {

    // Time based information (will be converted to kernel units)
    DWORD                   TimeCheck;                      // in US
    DWORD                   DemoteLimit;                    // in US
    DWORD                   PromoteLimit;                   // in US

    // Percentage based information
    BYTE                    DemotePercent;
    BYTE                    PromotePercent;
    BYTE                    Spare[2];

    // Flags
    DWORD                   AllowDemotion:1;
    DWORD                   AllowPromotion:1;
    DWORD                   Reserved:30;

} PROCESSOR_POWER_POLICY_INFO, *PPROCESSOR_POWER_POLICY_INFO;

// processor power policy
typedef struct _PROCESSOR_POWER_POLICY {
    DWORD                       Revision;       // 1

    // Dynamic Throttling Policy
    BYTE                        DynamicThrottle;
    BYTE                        Spare[3];

    // Flags
    DWORD                       DisableCStates:1;
    DWORD                       Reserved:31;

    // System policy information
    // The Array is last, in case it needs to be grown and the structure
    // revision incremented.
    DWORD                       PolicyCount;
    PROCESSOR_POWER_POLICY_INFO Policy[3];

} PROCESSOR_POWER_POLICY, *PPROCESSOR_POWER_POLICY;

//
// Processor Perf State Policy.
//

typedef struct {
    DWORD Revision;
    BYTE  MaxThrottle;
    BYTE  MinThrottle;
    BYTE  BusyAdjThreshold;
    union {
        BYTE  Spare;
        union {
            BYTE  AsBYTE ;
            struct {
                BYTE  NoDomainAccounting : 1;
                BYTE  IncreasePolicy: 2;
                BYTE  DecreasePolicy: 2;
                BYTE  Reserved : 3;
            } DUMMYSTRUCTNAME;
        } Flags;
    } DUMMYUNIONNAME;

    DWORD TimeCheck;
    DWORD IncreaseTime;
    DWORD DecreaseTime;
    DWORD IncreasePercent;
    DWORD DecreasePercent;
} PROCESSOR_PERFSTATE_POLICY, *PPROCESSOR_PERFSTATE_POLICY;

// administrator power policy overrides
typedef struct _ADMINISTRATOR_POWER_POLICY {

    // meaning of power action "sleep"
    SYSTEM_POWER_STATE      MinSleep;
    SYSTEM_POWER_STATE      MaxSleep;

    // video policies
    DWORD                   MinVideoTimeout;
    DWORD                   MaxVideoTimeout;

    // disk policies
    DWORD                   MinSpindownTimeout;
    DWORD                   MaxSpindownTimeout;
} ADMINISTRATOR_POWER_POLICY, *PADMINISTRATOR_POWER_POLICY;


typedef enum _HIBERFILE_BUCKET_SIZE {
    HiberFileBucket1GB = 0,
    HiberFileBucket2GB,
    HiberFileBucket4GB,
    HiberFileBucket8GB,
    HiberFileBucket16GB,
    HiberFileBucket32GB,
    HiberFileBucketUnlimited,
    HiberFileBucketMax
} HIBERFILE_BUCKET_SIZE, *PHIBERFILE_BUCKET_SIZE;

#define HIBERFILE_TYPE_NONE       0x00
#define HIBERFILE_TYPE_REDUCED    0x01
#define HIBERFILE_TYPE_FULL       0x02
#define HIBERFILE_TYPE_MAX        0x03

typedef struct _HIBERFILE_BUCKET {
    DWORD64 MaxPhysicalMemory;
    DWORD PhysicalMemoryPercent[HIBERFILE_TYPE_MAX];
} HIBERFILE_BUCKET, *PHIBERFILE_BUCKET;

typedef struct {
    // Misc supported system features
    BOOLEAN             PowerButtonPresent;
    BOOLEAN             SleepButtonPresent;
    BOOLEAN             LidPresent;
    BOOLEAN             SystemS1;
    BOOLEAN             SystemS2;
    BOOLEAN             SystemS3;
    BOOLEAN             SystemS4;           // hibernate
    BOOLEAN             SystemS5;           // off
    BOOLEAN             HiberFilePresent;
    BOOLEAN             FullWake;
    BOOLEAN             VideoDimPresent;
    BOOLEAN             ApmPresent;
    BOOLEAN             UpsPresent;

    // Processors
    BOOLEAN             ThermalControl;
    BOOLEAN             ProcessorThrottle;
    BYTE                ProcessorMinThrottle;

#if (NTDDI_VERSION < NTDDI_WINXP)
    BYTE                ProcessorThrottleScale;
    BYTE                spare2[4];
#else
    BYTE                ProcessorMaxThrottle;
    BOOLEAN             FastSystemS4;
    BOOLEAN             Hiberboot;
    BOOLEAN             WakeAlarmPresent;
    BOOLEAN             AoAc;
#endif // (NTDDI_VERSION < NTDDI_WINXP)

    // Disk
    BOOLEAN             DiskSpinDown;

#if (NTDDI_VERSION < NTDDI_WINTHRESHOLD)
    BYTE                spare3[8];
# else
    // HiberFile
    BYTE                HiberFileType;
    BOOLEAN             AoAcConnectivitySupported;
    BYTE                spare3[6];
#endif // (NTDDI_VERSION < NTDDI_WINTHRESHOLD)

    // System Battery
    BOOLEAN             SystemBatteriesPresent;
    BOOLEAN             BatteriesAreShortTerm;
    BATTERY_REPORTING_SCALE BatteryScale[3];

    // Wake
    SYSTEM_POWER_STATE  AcOnLineWake;
    SYSTEM_POWER_STATE  SoftLidWake;
    SYSTEM_POWER_STATE  RtcWake;
    SYSTEM_POWER_STATE  MinDeviceWakeState; // note this may change on driver load
    SYSTEM_POWER_STATE  DefaultLowLatencyWake;
} SYSTEM_POWER_CAPABILITIES, *PSYSTEM_POWER_CAPABILITIES;

typedef struct {
    BOOLEAN             AcOnLine;
    BOOLEAN             BatteryPresent;
    BOOLEAN             Charging;
    BOOLEAN             Discharging;
    BOOLEAN             Spare1[3];

    BYTE                Tag;

    DWORD               MaxCapacity;
    DWORD               RemainingCapacity;
    DWORD               Rate;
    DWORD               EstimatedTime;

    DWORD               DefaultAlert1;
    DWORD               DefaultAlert2;
} SYSTEM_BATTERY_STATE, *PSYSTEM_BATTERY_STATE;

typedef struct _SYSTEM_POWER_SOURCE_STATE {
    SYSTEM_BATTERY_STATE BatteryState;
    DWORD                InstantaneousPeakPower;
    DWORD                InstantaneousPeakPeriod;
    DWORD                SustainablePeakPower;
    DWORD                SustainablePeakPeriod;
    DWORD                PeakPower;
    DWORD                MaxOutputPower;
    DWORD                MaxInputPower;
    LONG                 BatteryRateInCurrent;
    DWORD                BatteryVoltage;
} SYSTEM_POWER_SOURCE_STATE, *PSYSTEM_POWER_SOURCE_STATE;

//
// N.B. SYSTEM_POWER_SOURCE_STATE extends SYSTEM_BATTERY_STATE, with BatteryState
//      positioned at the beginning of the structure. This layout ensures that unions
//      or structures referencing SYSTEM_BATTERY_STATE will correctly interpret
//      BatteryState when accessing SYSTEM_POWER_SOURCE_STATE. The assertion ensures
//      that the BatteryState field has a zero offset, confirming its position at the
//      start of SYSTEM_POWER_SOURCE_STATE.
//

#ifndef MIDL_PASS

C_ASSERT(FIELD_OFFSET(SYSTEM_POWER_SOURCE_STATE, BatteryState) == 0);

#endif



//
// Image Format
//


#ifndef _MAC

#include "pshpack4.h"                   // 4 byte packing is the default

#define IMAGE_DOS_SIGNATURE                 0x5A4D      // MZ
#define IMAGE_OS2_SIGNATURE                 0x454E      // NE
#define IMAGE_OS2_SIGNATURE_LE              0x454C      // LE
#define IMAGE_VXD_SIGNATURE                 0x454C      // LE
#define IMAGE_NT_SIGNATURE                  0x00004550  // PE00

#include "pshpack2.h"                   // 16 bit headers are 2 byte packed

#else

#include "pshpack1.h"

#define IMAGE_DOS_SIGNATURE                 0x4D5A      // MZ
#define IMAGE_OS2_SIGNATURE                 0x4E45      // NE
#define IMAGE_OS2_SIGNATURE_LE              0x4C45      // LE
#define IMAGE_NT_SIGNATURE                  0x50450000  // PE00
#endif

typedef struct _IMAGE_DOS_HEADER {      // DOS .EXE header
    WORD   e_magic;                     // Magic number
    WORD   e_cblp;                      // Bytes on last page of file
    WORD   e_cp;                        // Pages in file
    WORD   e_crlc;                      // Relocations
    WORD   e_cparhdr;                   // Size of header in paragraphs
    WORD   e_minalloc;                  // Minimum extra paragraphs needed
    WORD   e_maxalloc;                  // Maximum extra paragraphs needed
    WORD   e_ss;                        // Initial (relative) SS value
    WORD   e_sp;                        // Initial SP value
    WORD   e_csum;                      // Checksum
    WORD   e_ip;                        // Initial IP value
    WORD   e_cs;                        // Initial (relative) CS value
    WORD   e_lfarlc;                    // File address of relocation table
    WORD   e_ovno;                      // Overlay number
    WORD   e_res[4];                    // Reserved words
    WORD   e_oemid;                     // OEM identifier (for e_oeminfo)
    WORD   e_oeminfo;                   // OEM information; e_oemid specific
    WORD   e_res2[10];                  // Reserved words
    LONG   e_lfanew;                    // File address of new exe header
  } IMAGE_DOS_HEADER, *PIMAGE_DOS_HEADER;

typedef struct _IMAGE_OS2_HEADER {      // OS/2 .EXE header
    WORD   ne_magic;                    // Magic number
    CHAR   ne_ver;                      // Version number
    CHAR   ne_rev;                      // Revision number
    WORD   ne_enttab;                   // Offset of Entry Table
    WORD   ne_cbenttab;                 // Number of bytes in Entry Table
    LONG   ne_crc;                      // Checksum of whole file
    WORD   ne_flags;                    // Flag word
    WORD   ne_autodata;                 // Automatic data segment number
    WORD   ne_heap;                     // Initial heap allocation
    WORD   ne_stack;                    // Initial stack allocation
    LONG   ne_csip;                     // Initial CS:IP setting
    LONG   ne_sssp;                     // Initial SS:SP setting
    WORD   ne_cseg;                     // Count of file segments
    WORD   ne_cmod;                     // Entries in Module Reference Table
    WORD   ne_cbnrestab;                // Size of non-resident name table
    WORD   ne_segtab;                   // Offset of Segment Table
    WORD   ne_rsrctab;                  // Offset of Resource Table
    WORD   ne_restab;                   // Offset of resident name table
    WORD   ne_modtab;                   // Offset of Module Reference Table
    WORD   ne_imptab;                   // Offset of Imported Names Table
    LONG   ne_nrestab;                  // Offset of Non-resident Names Table
    WORD   ne_cmovent;                  // Count of movable entries
    WORD   ne_align;                    // Segment alignment shift count
    WORD   ne_cres;                     // Count of resource segments
    BYTE   ne_exetyp;                   // Target Operating system
    BYTE   ne_flagsothers;              // Other .EXE flags
    WORD   ne_pretthunks;               // offset to return thunks
    WORD   ne_psegrefbytes;             // offset to segment ref. bytes
    WORD   ne_swaparea;                 // Minimum code swap area size
    WORD   ne_expver;                   // Expected Windows version number
  } IMAGE_OS2_HEADER, *PIMAGE_OS2_HEADER;

typedef struct _IMAGE_VXD_HEADER {      // Windows VXD header
    WORD   e32_magic;                   // Magic number
    BYTE   e32_border;                  // The byte ordering for the VXD
    BYTE   e32_worder;                  // The word ordering for the VXD
    DWORD  e32_level;                   // The EXE format level for now = 0
    WORD   e32_cpu;                     // The CPU type
    WORD   e32_os;                      // The OS type
    DWORD  e32_ver;                     // Module version
    DWORD  e32_mflags;                  // Module flags
    DWORD  e32_mpages;                  // Module # pages
    DWORD  e32_startobj;                // Object # for instruction pointer
    DWORD  e32_eip;                     // Extended instruction pointer
    DWORD  e32_stackobj;                // Object # for stack pointer
    DWORD  e32_esp;                     // Extended stack pointer
    DWORD  e32_pagesize;                // VXD page size
    DWORD  e32_lastpagesize;            // Last page size in VXD
    DWORD  e32_fixupsize;               // Fixup section size
    DWORD  e32_fixupsum;                // Fixup section checksum
    DWORD  e32_ldrsize;                 // Loader section size
    DWORD  e32_ldrsum;                  // Loader section checksum
    DWORD  e32_objtab;                  // Object table offset
    DWORD  e32_objcnt;                  // Number of objects in module
    DWORD  e32_objmap;                  // Object page map offset
    DWORD  e32_itermap;                 // Object iterated data map offset
    DWORD  e32_rsrctab;                 // Offset of Resource Table
    DWORD  e32_rsrccnt;                 // Number of resource entries
    DWORD  e32_restab;                  // Offset of resident name table
    DWORD  e32_enttab;                  // Offset of Entry Table
    DWORD  e32_dirtab;                  // Offset of Module Directive Table
    DWORD  e32_dircnt;                  // Number of module directives
    DWORD  e32_fpagetab;                // Offset of Fixup Page Table
    DWORD  e32_frectab;                 // Offset of Fixup Record Table
    DWORD  e32_impmod;                  // Offset of Import Module Name Table
    DWORD  e32_impmodcnt;               // Number of entries in Import Module Name Table
    DWORD  e32_impproc;                 // Offset of Import Procedure Name Table
    DWORD  e32_pagesum;                 // Offset of Per-Page Checksum Table
    DWORD  e32_datapage;                // Offset of Enumerated Data Pages
    DWORD  e32_preload;                 // Number of preload pages
    DWORD  e32_nrestab;                 // Offset of Non-resident Names Table
    DWORD  e32_cbnrestab;               // Size of Non-resident Name Table
    DWORD  e32_nressum;                 // Non-resident Name Table Checksum
    DWORD  e32_autodata;                // Object # for automatic data object
    DWORD  e32_debuginfo;               // Offset of the debugging information
    DWORD  e32_debuglen;                // The length of the debugging info. in bytes
    DWORD  e32_instpreload;             // Number of instance pages in preload section of VXD file
    DWORD  e32_instdemand;              // Number of instance pages in demand load section of VXD file
    DWORD  e32_heapsize;                // Size of heap - for 16-bit apps
    BYTE   e32_res3[12];                // Reserved words
    DWORD  e32_winresoff;
    DWORD  e32_winreslen;
    WORD   e32_devid;                   // Device ID for VxD
    WORD   e32_ddkver;                  // DDK version for VxD
  } IMAGE_VXD_HEADER, *PIMAGE_VXD_HEADER;

#ifndef _MAC
#include "poppack.h"                    // Back to 4 byte packing
#endif

//
// File header format.
//

typedef struct _IMAGE_FILE_HEADER {
    WORD    Machine;
    WORD    NumberOfSections;
    DWORD   TimeDateStamp;
    DWORD   PointerToSymbolTable;
    DWORD   NumberOfSymbols;
    WORD    SizeOfOptionalHeader;
    WORD    Characteristics;
} IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;

#define IMAGE_SIZEOF_FILE_HEADER             20

#define IMAGE_FILE_RELOCS_STRIPPED           0x0001  // Relocation info stripped from file.
#define IMAGE_FILE_EXECUTABLE_IMAGE          0x0002  // File is executable  (i.e. no unresolved external references).
#define IMAGE_FILE_LINE_NUMS_STRIPPED        0x0004  // Line nunbers stripped from file.
#define IMAGE_FILE_LOCAL_SYMS_STRIPPED       0x0008  // Local symbols stripped from file.
#define IMAGE_FILE_AGGRESIVE_WS_TRIM         0x0010  // Aggressively trim working set
#define IMAGE_FILE_LARGE_ADDRESS_AWARE       0x0020  // App can handle >2gb addresses
#define IMAGE_FILE_BYTES_REVERSED_LO         0x0080  // Bytes of machine word are reversed.
#define IMAGE_FILE_32BIT_MACHINE             0x0100  // 32 bit word machine.
#define IMAGE_FILE_DEBUG_STRIPPED            0x0200  // Debugging info stripped from file in .DBG file
#define IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP   0x0400  // If Image is on removable media, copy and run from the swap file.
#define IMAGE_FILE_NET_RUN_FROM_SWAP         0x0800  // If Image is on Net, copy and run from the swap file.
#define IMAGE_FILE_SYSTEM                    0x1000  // System File.
#define IMAGE_FILE_DLL                       0x2000  // File is a DLL.
#define IMAGE_FILE_UP_SYSTEM_ONLY            0x4000  // File should only be run on a UP machine
#define IMAGE_FILE_BYTES_REVERSED_HI         0x8000  // Bytes of machine word are reversed.

#define IMAGE_FILE_MACHINE_UNKNOWN           0
#define IMAGE_FILE_MACHINE_TARGET_HOST       0x0001  // Useful for indicating we want to interact with the host and not a WoW guest.
#define IMAGE_FILE_MACHINE_I386              0x014c  // Intel 386.
#define IMAGE_FILE_MACHINE_R3000             0x0162  // MIPS little-endian, 0x160 big-endian
#define IMAGE_FILE_MACHINE_R4000             0x0166  // MIPS little-endian
#define IMAGE_FILE_MACHINE_R10000            0x0168  // MIPS little-endian
#define IMAGE_FILE_MACHINE_WCEMIPSV2         0x0169  // MIPS little-endian WCE v2
#define IMAGE_FILE_MACHINE_ALPHA             0x0184  // Alpha_AXP
#define IMAGE_FILE_MACHINE_SH3               0x01a2  // SH3 little-endian
#define IMAGE_FILE_MACHINE_SH3DSP            0x01a3
#define IMAGE_FILE_MACHINE_SH3E              0x01a4  // SH3E little-endian
#define IMAGE_FILE_MACHINE_SH4               0x01a6  // SH4 little-endian
#define IMAGE_FILE_MACHINE_SH5               0x01a8  // SH5
#define IMAGE_FILE_MACHINE_ARM               0x01c0  // ARM Little-Endian
#define IMAGE_FILE_MACHINE_THUMB             0x01c2  // ARM Thumb/Thumb-2 Little-Endian
#define IMAGE_FILE_MACHINE_ARMNT             0x01c4  // ARM Thumb-2 Little-Endian
#define IMAGE_FILE_MACHINE_AM33              0x01d3
#define IMAGE_FILE_MACHINE_POWERPC           0x01F0  // IBM PowerPC Little-Endian
#define IMAGE_FILE_MACHINE_POWERPCFP         0x01f1
#define IMAGE_FILE_MACHINE_IA64              0x0200  // Intel 64
#define IMAGE_FILE_MACHINE_MIPS16            0x0266  // MIPS
#define IMAGE_FILE_MACHINE_ALPHA64           0x0284  // ALPHA64
#define IMAGE_FILE_MACHINE_MIPSFPU           0x0366  // MIPS
#define IMAGE_FILE_MACHINE_MIPSFPU16         0x0466  // MIPS
#define IMAGE_FILE_MACHINE_AXP64             IMAGE_FILE_MACHINE_ALPHA64
#define IMAGE_FILE_MACHINE_TRICORE           0x0520  // Infineon
#define IMAGE_FILE_MACHINE_CEF               0x0CEF
#define IMAGE_FILE_MACHINE_EBC               0x0EBC  // EFI Byte Code
#define IMAGE_FILE_MACHINE_AMD64             0x8664  // AMD64 (K8)
#define IMAGE_FILE_MACHINE_M32R              0x9041  // M32R little-endian
#define IMAGE_FILE_MACHINE_ARM64             0xAA64  // ARM64 Little-Endian
#define IMAGE_FILE_MACHINE_CEE               0xC0EE


//
// Directory format.
//

typedef struct _IMAGE_DATA_DIRECTORY {
    DWORD   VirtualAddress;
    DWORD   Size;
} IMAGE_DATA_DIRECTORY, *PIMAGE_DATA_DIRECTORY;

#define IMAGE_NUMBEROF_DIRECTORY_ENTRIES    16

//
// Optional header format.
//

typedef struct _IMAGE_OPTIONAL_HEADER {
    //
    // Standard fields.
    //

    WORD    Magic;
    BYTE    MajorLinkerVersion;
    BYTE    MinorLinkerVersion;
    DWORD   SizeOfCode;
    DWORD   SizeOfInitializedData;
    DWORD   SizeOfUninitializedData;
    DWORD   AddressOfEntryPoint;
    DWORD   BaseOfCode;
    DWORD   BaseOfData;

    //
    // NT additional fields.
    //

    DWORD   ImageBase;
    DWORD   SectionAlignment;
    DWORD   FileAlignment;
    WORD    MajorOperatingSystemVersion;
    WORD    MinorOperatingSystemVersion;
    WORD    MajorImageVersion;
    WORD    MinorImageVersion;
    WORD    MajorSubsystemVersion;
    WORD    MinorSubsystemVersion;
    DWORD   Win32VersionValue;
    DWORD   SizeOfImage;
    DWORD   SizeOfHeaders;
    DWORD   CheckSum;
    WORD    Subsystem;
    WORD    DllCharacteristics;
    DWORD   SizeOfStackReserve;
    DWORD   SizeOfStackCommit;
    DWORD   SizeOfHeapReserve;
    DWORD   SizeOfHeapCommit;
    DWORD   LoaderFlags;
    DWORD   NumberOfRvaAndSizes;
    IMAGE_DATA_DIRECTORY DataDirectory[IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
} IMAGE_OPTIONAL_HEADER32, *PIMAGE_OPTIONAL_HEADER32;

typedef struct _IMAGE_ROM_OPTIONAL_HEADER {
    WORD   Magic;
    BYTE   MajorLinkerVersion;
    BYTE   MinorLinkerVersion;
    DWORD  SizeOfCode;
    DWORD  SizeOfInitializedData;
    DWORD  SizeOfUninitializedData;
    DWORD  AddressOfEntryPoint;
    DWORD  BaseOfCode;
    DWORD  BaseOfData;
    DWORD  BaseOfBss;
    DWORD  GprMask;
    DWORD  CprMask[4];
    DWORD  GpValue;
} IMAGE_ROM_OPTIONAL_HEADER, *PIMAGE_ROM_OPTIONAL_HEADER;

typedef struct _IMAGE_OPTIONAL_HEADER64 {
    WORD        Magic;
    BYTE        MajorLinkerVersion;
    BYTE        MinorLinkerVersion;
    DWORD       SizeOfCode;
    DWORD       SizeOfInitializedData;
    DWORD       SizeOfUninitializedData;
    DWORD       AddressOfEntryPoint;
    DWORD       BaseOfCode;
    ULONGLONG   ImageBase;
    DWORD       SectionAlignment;
    DWORD       FileAlignment;
    WORD        MajorOperatingSystemVersion;
    WORD        MinorOperatingSystemVersion;
    WORD        MajorImageVersion;
    WORD        MinorImageVersion;
    WORD        MajorSubsystemVersion;
    WORD        MinorSubsystemVersion;
    DWORD       Win32VersionValue;
    DWORD       SizeOfImage;
    DWORD       SizeOfHeaders;
    DWORD       CheckSum;
    WORD        Subsystem;
    WORD        DllCharacteristics;
    ULONGLONG   SizeOfStackReserve;
    ULONGLONG   SizeOfStackCommit;
    ULONGLONG   SizeOfHeapReserve;
    ULONGLONG   SizeOfHeapCommit;
    DWORD       LoaderFlags;
    DWORD       NumberOfRvaAndSizes;
    IMAGE_DATA_DIRECTORY DataDirectory[IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
} IMAGE_OPTIONAL_HEADER64, *PIMAGE_OPTIONAL_HEADER64;

#define IMAGE_NT_OPTIONAL_HDR32_MAGIC      0x10b
#define IMAGE_NT_OPTIONAL_HDR64_MAGIC      0x20b
#define IMAGE_ROM_OPTIONAL_HDR_MAGIC       0x107

#ifdef _WIN64
typedef IMAGE_OPTIONAL_HEADER64             IMAGE_OPTIONAL_HEADER;
typedef PIMAGE_OPTIONAL_HEADER64            PIMAGE_OPTIONAL_HEADER;
#define IMAGE_NT_OPTIONAL_HDR_MAGIC         IMAGE_NT_OPTIONAL_HDR64_MAGIC
#else
typedef IMAGE_OPTIONAL_HEADER32             IMAGE_OPTIONAL_HEADER;
typedef PIMAGE_OPTIONAL_HEADER32            PIMAGE_OPTIONAL_HEADER;
#define IMAGE_NT_OPTIONAL_HDR_MAGIC         IMAGE_NT_OPTIONAL_HDR32_MAGIC
#endif

typedef struct _IMAGE_NT_HEADERS64 {
    DWORD Signature;
    IMAGE_FILE_HEADER FileHeader;
    IMAGE_OPTIONAL_HEADER64 OptionalHeader;
} IMAGE_NT_HEADERS64, *PIMAGE_NT_HEADERS64;

typedef struct _IMAGE_NT_HEADERS {
    DWORD Signature;
    IMAGE_FILE_HEADER FileHeader;
    IMAGE_OPTIONAL_HEADER32 OptionalHeader;
} IMAGE_NT_HEADERS32, *PIMAGE_NT_HEADERS32;

typedef struct _IMAGE_ROM_HEADERS {
    IMAGE_FILE_HEADER FileHeader;
    IMAGE_ROM_OPTIONAL_HEADER OptionalHeader;
} IMAGE_ROM_HEADERS, *PIMAGE_ROM_HEADERS;

#ifdef _WIN64
typedef IMAGE_NT_HEADERS64                  IMAGE_NT_HEADERS;
typedef PIMAGE_NT_HEADERS64                 PIMAGE_NT_HEADERS;
#else
typedef IMAGE_NT_HEADERS32                  IMAGE_NT_HEADERS;
typedef PIMAGE_NT_HEADERS32                 PIMAGE_NT_HEADERS;
#endif

// IMAGE_FIRST_SECTION doesn't need 32/64 versions since the file header is the same either way.

#define IMAGE_FIRST_SECTION( ntheader ) ((PIMAGE_SECTION_HEADER)        \
    ((ULONG_PTR)(ntheader) +                                            \
     FIELD_OFFSET( IMAGE_NT_HEADERS, OptionalHeader ) +                 \
     ((ntheader))->FileHeader.SizeOfOptionalHeader   \
    ))

// Subsystem Values

#define IMAGE_SUBSYSTEM_UNKNOWN              0   // Unknown subsystem.
#define IMAGE_SUBSYSTEM_NATIVE               1   // Image doesn't require a subsystem.
#define IMAGE_SUBSYSTEM_WINDOWS_GUI          2   // Image runs in the Windows GUI subsystem.
#define IMAGE_SUBSYSTEM_WINDOWS_CUI          3   // Image runs in the Windows character subsystem.
#define IMAGE_SUBSYSTEM_OS2_CUI              5   // image runs in the OS/2 character subsystem.
#define IMAGE_SUBSYSTEM_POSIX_CUI            7   // image runs in the Posix character subsystem.
#define IMAGE_SUBSYSTEM_NATIVE_WINDOWS       8   // image is a native Win9x driver.
#define IMAGE_SUBSYSTEM_WINDOWS_CE_GUI       9   // Image runs in the Windows CE subsystem.
#define IMAGE_SUBSYSTEM_EFI_APPLICATION      10  //
#define IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER  11   //
#define IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER   12  //
#define IMAGE_SUBSYSTEM_EFI_ROM              13
#define IMAGE_SUBSYSTEM_XBOX                 14
#define IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION 16
#define IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG    17

// DllCharacteristics Entries

//      IMAGE_LIBRARY_PROCESS_INIT            0x0001     // Reserved.
//      IMAGE_LIBRARY_PROCESS_TERM            0x0002     // Reserved.
//      IMAGE_LIBRARY_THREAD_INIT             0x0004     // Reserved.
//      IMAGE_LIBRARY_THREAD_TERM             0x0008     // Reserved.
#define IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA    0x0020  // Image can handle a high entropy 64-bit virtual address space.
#define IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE 0x0040     // DLL can move.
#define IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY    0x0080     // Code Integrity Image
#define IMAGE_DLLCHARACTERISTICS_NX_COMPAT    0x0100     // Image is NX compatible
#define IMAGE_DLLCHARACTERISTICS_NO_ISOLATION 0x0200     // Image understands isolation and doesn't want it
#define IMAGE_DLLCHARACTERISTICS_NO_SEH       0x0400     // Image does not use SEH.  No SE handler may reside in this image
#define IMAGE_DLLCHARACTERISTICS_NO_BIND      0x0800     // Do not bind this image.
#define IMAGE_DLLCHARACTERISTICS_APPCONTAINER 0x1000     // Image should execute in an AppContainer
#define IMAGE_DLLCHARACTERISTICS_WDM_DRIVER   0x2000     // Driver uses WDM model
#define IMAGE_DLLCHARACTERISTICS_GUARD_CF     0x4000     // Image supports Control Flow Guard.
#define IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE     0x8000

// Directory Entries

#define IMAGE_DIRECTORY_ENTRY_EXPORT          0   // Export Directory
#define IMAGE_DIRECTORY_ENTRY_IMPORT          1   // Import Directory
#define IMAGE_DIRECTORY_ENTRY_RESOURCE        2   // Resource Directory
#define IMAGE_DIRECTORY_ENTRY_EXCEPTION       3   // Exception Directory
#define IMAGE_DIRECTORY_ENTRY_SECURITY        4   // Security Directory
#define IMAGE_DIRECTORY_ENTRY_BASERELOC       5   // Base Relocation Table
#define IMAGE_DIRECTORY_ENTRY_DEBUG           6   // Debug Directory
//      IMAGE_DIRECTORY_ENTRY_COPYRIGHT       7   // (X86 usage)
#define IMAGE_DIRECTORY_ENTRY_ARCHITECTURE    7   // Architecture Specific Data
#define IMAGE_DIRECTORY_ENTRY_GLOBALPTR       8   // RVA of GP
#define IMAGE_DIRECTORY_ENTRY_TLS             9   // TLS Directory
#define IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG    10   // Load Configuration Directory
#define IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT   11   // Bound Import Directory in headers
#define IMAGE_DIRECTORY_ENTRY_IAT            12   // Import Address Table
#define IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT   13   // Delay Load Import Descriptors
#define IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR 14   // COM Runtime descriptor

//
// Non-COFF Object file header
//

typedef struct ANON_OBJECT_HEADER {
    WORD    Sig1;            // Must be IMAGE_FILE_MACHINE_UNKNOWN
    WORD    Sig2;            // Must be 0xffff
    WORD    Version;         // >= 1 (implies the CLSID field is present)
    WORD    Machine;
    DWORD   TimeDateStamp;
    CLSID   ClassID;         // Used to invoke CoCreateInstance
    DWORD   SizeOfData;      // Size of data that follows the header
} ANON_OBJECT_HEADER;

typedef struct ANON_OBJECT_HEADER_V2 {
    WORD    Sig1;            // Must be IMAGE_FILE_MACHINE_UNKNOWN
    WORD    Sig2;            // Must be 0xffff
    WORD    Version;         // >= 2 (implies the Flags field is present - otherwise V1)
    WORD    Machine;
    DWORD   TimeDateStamp;
    CLSID   ClassID;         // Used to invoke CoCreateInstance
    DWORD   SizeOfData;      // Size of data that follows the header
    DWORD   Flags;           // 0x1 -> contains metadata
    DWORD   MetaDataSize;    // Size of CLR metadata
    DWORD   MetaDataOffset;  // Offset of CLR metadata
} ANON_OBJECT_HEADER_V2;

typedef struct ANON_OBJECT_HEADER_BIGOBJ {
   /* same as ANON_OBJECT_HEADER_V2 */
    WORD    Sig1;            // Must be IMAGE_FILE_MACHINE_UNKNOWN
    WORD    Sig2;            // Must be 0xffff
    WORD    Version;         // >= 2 (implies the Flags field is present)
    WORD    Machine;         // Actual machine - IMAGE_FILE_MACHINE_xxx
    DWORD   TimeDateStamp;
    CLSID   ClassID;         // {D1BAA1C7-BAEE-4ba9-AF20-FAF66AA4DCB8}
    DWORD   SizeOfData;      // Size of data that follows the header
    DWORD   Flags;           // 0x1 -> contains metadata
    DWORD   MetaDataSize;    // Size of CLR metadata
    DWORD   MetaDataOffset;  // Offset of CLR metadata

    /* bigobj specifics */
    DWORD   NumberOfSections; // extended from WORD
    DWORD   PointerToSymbolTable;
    DWORD   NumberOfSymbols;
} ANON_OBJECT_HEADER_BIGOBJ;

//
// Section header format.
//

#define IMAGE_SIZEOF_SHORT_NAME              8

typedef struct _IMAGE_SECTION_HEADER {
    BYTE    Name[IMAGE_SIZEOF_SHORT_NAME];
    union {
            DWORD   PhysicalAddress;
            DWORD   VirtualSize;
    } Misc;
    DWORD   VirtualAddress;
    DWORD   SizeOfRawData;
    DWORD   PointerToRawData;
    DWORD   PointerToRelocations;
    DWORD   PointerToLinenumbers;
    WORD    NumberOfRelocations;
    WORD    NumberOfLinenumbers;
    DWORD   Characteristics;
} IMAGE_SECTION_HEADER, *PIMAGE_SECTION_HEADER;

#define IMAGE_SIZEOF_SECTION_HEADER          40

//
// Section characteristics.
//
//      IMAGE_SCN_TYPE_REG                   0x00000000  // Reserved.
//      IMAGE_SCN_TYPE_DSECT                 0x00000001  // Reserved.
//      IMAGE_SCN_TYPE_NOLOAD                0x00000002  // Reserved.
//      IMAGE_SCN_TYPE_GROUP                 0x00000004  // Reserved.
#define IMAGE_SCN_TYPE_NO_PAD                0x00000008  // Reserved.
//      IMAGE_SCN_TYPE_COPY                  0x00000010  // Reserved.

#define IMAGE_SCN_CNT_CODE                   0x00000020  // Section contains code.
#define IMAGE_SCN_CNT_INITIALIZED_DATA       0x00000040  // Section contains initialized data.
#define IMAGE_SCN_CNT_UNINITIALIZED_DATA     0x00000080  // Section contains uninitialized data.

#define IMAGE_SCN_LNK_OTHER                  0x00000100  // Reserved.
#define IMAGE_SCN_LNK_INFO                   0x00000200  // Section contains comments or some other type of information.
//      IMAGE_SCN_TYPE_OVER                  0x00000400  // Reserved.
#define IMAGE_SCN_LNK_REMOVE                 0x00000800  // Section contents will not become part of image.
#define IMAGE_SCN_LNK_COMDAT                 0x00001000  // Section contents comdat.
//                                           0x00002000  // Reserved.
//      IMAGE_SCN_MEM_PROTECTED - Obsolete   0x00004000
#define IMAGE_SCN_NO_DEFER_SPEC_EXC          0x00004000  // Reset speculative exceptions handling bits in the TLB entries for this section.
#define IMAGE_SCN_GPREL                      0x00008000  // Section content can be accessed relative to GP
#define IMAGE_SCN_MEM_FARDATA                0x00008000
//      IMAGE_SCN_MEM_SYSHEAP  - Obsolete    0x00010000
#define IMAGE_SCN_MEM_PURGEABLE              0x00020000
#define IMAGE_SCN_MEM_16BIT                  0x00020000
#define IMAGE_SCN_MEM_LOCKED                 0x00040000
#define IMAGE_SCN_MEM_PRELOAD                0x00080000

#define IMAGE_SCN_ALIGN_1BYTES               0x00100000  //
#define IMAGE_SCN_ALIGN_2BYTES               0x00200000  //
#define IMAGE_SCN_ALIGN_4BYTES               0x00300000  //
#define IMAGE_SCN_ALIGN_8BYTES               0x00400000  //
#define IMAGE_SCN_ALIGN_16BYTES              0x00500000  // Default alignment if no others are specified.
#define IMAGE_SCN_ALIGN_32BYTES              0x00600000  //
#define IMAGE_SCN_ALIGN_64BYTES              0x00700000  //
#define IMAGE_SCN_ALIGN_128BYTES             0x00800000  //
#define IMAGE_SCN_ALIGN_256BYTES             0x00900000  //
#define IMAGE_SCN_ALIGN_512BYTES             0x00A00000  //
#define IMAGE_SCN_ALIGN_1024BYTES            0x00B00000  //
#define IMAGE_SCN_ALIGN_2048BYTES            0x00C00000  //
#define IMAGE_SCN_ALIGN_4096BYTES            0x00D00000  //
#define IMAGE_SCN_ALIGN_8192BYTES            0x00E00000  //
// Unused                                    0x00F00000
#define IMAGE_SCN_ALIGN_MASK                 0x00F00000

#define IMAGE_SCN_LNK_NRELOC_OVFL            0x01000000  // Section contains extended relocations.
#define IMAGE_SCN_MEM_DISCARDABLE            0x02000000  // Section can be discarded.
#define IMAGE_SCN_MEM_NOT_CACHED             0x04000000  // Section is not cachable.
#define IMAGE_SCN_MEM_NOT_PAGED              0x08000000  // Section is not pageable.
#define IMAGE_SCN_MEM_SHARED                 0x10000000  // Section is shareable.
#define IMAGE_SCN_MEM_EXECUTE                0x20000000  // Section is executable.
#define IMAGE_SCN_MEM_READ                   0x40000000  // Section is readable.
#define IMAGE_SCN_MEM_WRITE                  0x80000000  // Section is writeable.

//
// TLS Characteristic Flags
//
#define IMAGE_SCN_SCALE_INDEX                0x00000001  // Tls index is scaled

#ifndef _MAC
#include "pshpack2.h"                       // Symbols, relocs, and linenumbers are 2 byte packed
#endif

//
// Symbol format.
//

typedef struct _IMAGE_SYMBOL {
    union {
        BYTE    ShortName[8];
        struct {
            DWORD   Short;     // if 0, use LongName
            DWORD   Long;      // offset into string table
        } Name;
        DWORD   LongName[2];    // PBYTE [2]
    } N;
    DWORD   Value;
    SHORT   SectionNumber;
    WORD    Type;
    BYTE    StorageClass;
    BYTE    NumberOfAuxSymbols;
} IMAGE_SYMBOL;
typedef IMAGE_SYMBOL UNALIGNED *PIMAGE_SYMBOL;

#define IMAGE_SIZEOF_SYMBOL                  18

typedef struct _IMAGE_SYMBOL_EX {
    union {
        BYTE     ShortName[8];
        struct {
            DWORD   Short;     // if 0, use LongName
            DWORD   Long;      // offset into string table
        } Name;
        DWORD   LongName[2];    // PBYTE  [2]
    } N;
    DWORD   Value;
    LONG    SectionNumber;
    WORD    Type;
    BYTE    StorageClass;
    BYTE    NumberOfAuxSymbols;
} IMAGE_SYMBOL_EX;
typedef IMAGE_SYMBOL_EX UNALIGNED *PIMAGE_SYMBOL_EX;

//
// Section values.
//
// Symbols have a section number of the section in which they are
// defined. Otherwise, section numbers have the following meanings:
//

#define IMAGE_SYM_UNDEFINED           (SHORT)0          // Symbol is undefined or is common.
#define IMAGE_SYM_ABSOLUTE            (SHORT)-1         // Symbol is an absolute value.
#define IMAGE_SYM_DEBUG               (SHORT)-2         // Symbol is a special debug item.
#define IMAGE_SYM_SECTION_MAX         0xFEFF            // Values 0xFF00-0xFFFF are special
#define IMAGE_SYM_SECTION_MAX_EX      MAXLONG

//
// Type (fundamental) values.
//

#define IMAGE_SYM_TYPE_NULL                 0x0000  // no type.
#define IMAGE_SYM_TYPE_VOID                 0x0001  //
#define IMAGE_SYM_TYPE_CHAR                 0x0002  // type character.
#define IMAGE_SYM_TYPE_SHORT                0x0003  // type short integer.
#define IMAGE_SYM_TYPE_INT                  0x0004  //
#define IMAGE_SYM_TYPE_LONG                 0x0005  //
#define IMAGE_SYM_TYPE_FLOAT                0x0006  //
#define IMAGE_SYM_TYPE_DOUBLE               0x0007  //
#define IMAGE_SYM_TYPE_STRUCT               0x0008  //
#define IMAGE_SYM_TYPE_UNION                0x0009  //
#define IMAGE_SYM_TYPE_ENUM                 0x000A  // enumeration.
#define IMAGE_SYM_TYPE_MOE                  0x000B  // member of enumeration.
#define IMAGE_SYM_TYPE_BYTE                 0x000C  //
#define IMAGE_SYM_TYPE_WORD                 0x000D  //
#define IMAGE_SYM_TYPE_UINT                 0x000E  //
#define IMAGE_SYM_TYPE_DWORD                0x000F  //
#define IMAGE_SYM_TYPE_PCODE                0x8000  //
//
// Type (derived) values.
//

#define IMAGE_SYM_DTYPE_NULL                0       // no derived type.
#define IMAGE_SYM_DTYPE_POINTER             1       // pointer.
#define IMAGE_SYM_DTYPE_FUNCTION            2       // function.
#define IMAGE_SYM_DTYPE_ARRAY               3       // array.

//
// Storage classes.
//
#define IMAGE_SYM_CLASS_END_OF_FUNCTION     (BYTE )-1
#define IMAGE_SYM_CLASS_NULL                0x0000
#define IMAGE_SYM_CLASS_AUTOMATIC           0x0001
#define IMAGE_SYM_CLASS_EXTERNAL            0x0002
#define IMAGE_SYM_CLASS_STATIC              0x0003
#define IMAGE_SYM_CLASS_REGISTER            0x0004
#define IMAGE_SYM_CLASS_EXTERNAL_DEF        0x0005
#define IMAGE_SYM_CLASS_LABEL               0x0006
#define IMAGE_SYM_CLASS_UNDEFINED_LABEL     0x0007
#define IMAGE_SYM_CLASS_MEMBER_OF_STRUCT    0x0008
#define IMAGE_SYM_CLASS_ARGUMENT            0x0009
#define IMAGE_SYM_CLASS_STRUCT_TAG          0x000A
#define IMAGE_SYM_CLASS_MEMBER_OF_UNION     0x000B
#define IMAGE_SYM_CLASS_UNION_TAG           0x000C
#define IMAGE_SYM_CLASS_TYPE_DEFINITION     0x000D
#define IMAGE_SYM_CLASS_UNDEFINED_STATIC    0x000E
#define IMAGE_SYM_CLASS_ENUM_TAG            0x000F
#define IMAGE_SYM_CLASS_MEMBER_OF_ENUM      0x0010
#define IMAGE_SYM_CLASS_REGISTER_PARAM      0x0011
#define IMAGE_SYM_CLASS_BIT_FIELD           0x0012

#define IMAGE_SYM_CLASS_FAR_EXTERNAL        0x0044  //

#define IMAGE_SYM_CLASS_BLOCK               0x0064
#define IMAGE_SYM_CLASS_FUNCTION            0x0065
#define IMAGE_SYM_CLASS_END_OF_STRUCT       0x0066
#define IMAGE_SYM_CLASS_FILE                0x0067
// new
#define IMAGE_SYM_CLASS_SECTION             0x0068
#define IMAGE_SYM_CLASS_WEAK_EXTERNAL       0x0069

#define IMAGE_SYM_CLASS_CLR_TOKEN           0x006B

// type packing constants

#define N_BTMASK                            0x000F
#define N_TMASK                             0x0030
#define N_TMASK1                            0x00C0
#define N_TMASK2                            0x00F0
#define N_BTSHFT                            4
#define N_TSHIFT                            2
// MACROS

// Basic Type of  x
#define BTYPE(x) ((x) & N_BTMASK)

// Is x a pointer?
#ifndef ISPTR
#define ISPTR(x) (((x) & N_TMASK) == (IMAGE_SYM_DTYPE_POINTER << N_BTSHFT))
#endif

// Is x a function?
#ifndef ISFCN
#define ISFCN(x) (((x) & N_TMASK) == (IMAGE_SYM_DTYPE_FUNCTION << N_BTSHFT))
#endif

// Is x an array?

#ifndef ISARY
#define ISARY(x) (((x) & N_TMASK) == (IMAGE_SYM_DTYPE_ARRAY << N_BTSHFT))
#endif

// Is x a structure, union, or enumeration TAG?
#ifndef ISTAG
#define ISTAG(x) ((x)==IMAGE_SYM_CLASS_STRUCT_TAG || (x)==IMAGE_SYM_CLASS_UNION_TAG || (x)==IMAGE_SYM_CLASS_ENUM_TAG)
#endif

#ifndef INCREF
#define INCREF(x) ((((x)&~N_BTMASK)<<N_TSHIFT)|(IMAGE_SYM_DTYPE_POINTER<<N_BTSHFT)|((x)&N_BTMASK))
#endif
#ifndef DECREF
#define DECREF(x) ((((x)>>N_TSHIFT)&~N_BTMASK)|((x)&N_BTMASK))
#endif

#include <pshpack2.h>

typedef struct IMAGE_AUX_SYMBOL_TOKEN_DEF {
    BYTE  bAuxType;                  // IMAGE_AUX_SYMBOL_TYPE
    BYTE  bReserved;                 // Must be 0
    DWORD SymbolTableIndex;
    BYTE  rgbReserved[12];           // Must be 0
} IMAGE_AUX_SYMBOL_TOKEN_DEF;

typedef IMAGE_AUX_SYMBOL_TOKEN_DEF UNALIGNED *PIMAGE_AUX_SYMBOL_TOKEN_DEF;

#include <poppack.h>

//
// Auxiliary entry format.
//

typedef union _IMAGE_AUX_SYMBOL {
    struct {
        DWORD    TagIndex;                      // struct, union, or enum tag index
        union {
            struct {
                WORD    Linenumber;             // declaration line number
                WORD    Size;                   // size of struct, union, or enum
            } LnSz;
           DWORD    TotalSize;
        } Misc;
        union {
            struct {                            // if ISFCN, tag, or .bb
                DWORD    PointerToLinenumber;
                DWORD    PointerToNextFunction;
            } Function;
            struct {                            // if ISARY, up to 4 dimen.
                WORD     Dimension[4];
            } Array;
        } FcnAry;
        WORD    TvIndex;                        // tv index
    } Sym;
    struct {
        BYTE    Name[IMAGE_SIZEOF_SYMBOL];
    } File;
    struct {
        DWORD   Length;                         // section length
        WORD    NumberOfRelocations;            // number of relocation entries
        WORD    NumberOfLinenumbers;            // number of line numbers
        DWORD   CheckSum;                       // checksum for communal
        SHORT   Number;                         // section number to associate with
        BYTE    Selection;                      // communal selection type
	BYTE    bReserved;
	SHORT   HighNumber;                     // high bits of the section number
    } Section;
    IMAGE_AUX_SYMBOL_TOKEN_DEF TokenDef;
    struct {
        DWORD crc;
        BYTE  rgbReserved[14];
    } CRC;
} IMAGE_AUX_SYMBOL;
typedef IMAGE_AUX_SYMBOL UNALIGNED *PIMAGE_AUX_SYMBOL;

typedef union _IMAGE_AUX_SYMBOL_EX {
    struct {
        DWORD   WeakDefaultSymIndex;                       // the weak extern default symbol index
        DWORD   WeakSearchType;
        BYTE    rgbReserved[12];
    } Sym;
    struct {
        BYTE    Name[sizeof(IMAGE_SYMBOL_EX)];
    } File;
    struct {
        DWORD   Length;                         // section length
        WORD    NumberOfRelocations;            // number of relocation entries
        WORD    NumberOfLinenumbers;            // number of line numbers
        DWORD   CheckSum;                       // checksum for communal
        SHORT   Number;                         // section number to associate with
        BYTE    Selection;                      // communal selection type
        BYTE    bReserved;
        SHORT   HighNumber;                     // high bits of the section number
        BYTE    rgbReserved[2];
    } Section;
    struct{
        IMAGE_AUX_SYMBOL_TOKEN_DEF TokenDef;
        BYTE  rgbReserved[2];
    } DUMMYSTRUCTNAME;
    struct {
        DWORD crc;
        BYTE  rgbReserved[16];
    } CRC;
} IMAGE_AUX_SYMBOL_EX;
typedef IMAGE_AUX_SYMBOL_EX UNALIGNED *PIMAGE_AUX_SYMBOL_EX;

typedef enum IMAGE_AUX_SYMBOL_TYPE {
    IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF = 1,
} IMAGE_AUX_SYMBOL_TYPE;


//
// Communal selection types.
//

#define IMAGE_COMDAT_SELECT_NODUPLICATES    1
#define IMAGE_COMDAT_SELECT_ANY             2
#define IMAGE_COMDAT_SELECT_SAME_SIZE       3
#define IMAGE_COMDAT_SELECT_EXACT_MATCH     4
#define IMAGE_COMDAT_SELECT_ASSOCIATIVE     5
#define IMAGE_COMDAT_SELECT_LARGEST         6
#define IMAGE_COMDAT_SELECT_NEWEST          7

#define IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY  1
#define IMAGE_WEAK_EXTERN_SEARCH_LIBRARY    2
#define IMAGE_WEAK_EXTERN_SEARCH_ALIAS      3
#define IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY   4

//
// Relocation format.
//

typedef struct _IMAGE_RELOCATION {
    union {
        DWORD   VirtualAddress;
        DWORD   RelocCount;             // Set to the real count when IMAGE_SCN_LNK_NRELOC_OVFL is set
    } DUMMYUNIONNAME;
    DWORD   SymbolTableIndex;
    WORD    Type;
} IMAGE_RELOCATION;
typedef IMAGE_RELOCATION UNALIGNED *PIMAGE_RELOCATION;

//
// I386 relocation types.
//
#define IMAGE_REL_I386_ABSOLUTE         0x0000  // Reference is absolute, no relocation is necessary
#define IMAGE_REL_I386_DIR16            0x0001  // Direct 16-bit reference to the symbols virtual address
#define IMAGE_REL_I386_REL16            0x0002  // PC-relative 16-bit reference to the symbols virtual address
#define IMAGE_REL_I386_DIR32            0x0006  // Direct 32-bit reference to the symbols virtual address
#define IMAGE_REL_I386_DIR32NB          0x0007  // Direct 32-bit reference to the symbols virtual address, base not included
#define IMAGE_REL_I386_SEG12            0x0009  // Direct 16-bit reference to the segment-selector bits of a 32-bit virtual address
#define IMAGE_REL_I386_SECTION          0x000A
#define IMAGE_REL_I386_SECREL           0x000B
#define IMAGE_REL_I386_TOKEN            0x000C  // clr token
#define IMAGE_REL_I386_SECREL7          0x000D  // 7 bit offset from base of section containing target
#define IMAGE_REL_I386_REL32            0x0014  // PC-relative 32-bit reference to the symbols virtual address

//
// MIPS relocation types.
//
#define IMAGE_REL_MIPS_ABSOLUTE         0x0000  // Reference is absolute, no relocation is necessary
#define IMAGE_REL_MIPS_REFHALF          0x0001
#define IMAGE_REL_MIPS_REFWORD          0x0002
#define IMAGE_REL_MIPS_JMPADDR          0x0003
#define IMAGE_REL_MIPS_REFHI            0x0004
#define IMAGE_REL_MIPS_REFLO            0x0005
#define IMAGE_REL_MIPS_GPREL            0x0006
#define IMAGE_REL_MIPS_LITERAL          0x0007
#define IMAGE_REL_MIPS_SECTION          0x000A
#define IMAGE_REL_MIPS_SECREL           0x000B
#define IMAGE_REL_MIPS_SECRELLO         0x000C  // Low 16-bit section relative referemce (used for >32k TLS)
#define IMAGE_REL_MIPS_SECRELHI         0x000D  // High 16-bit section relative reference (used for >32k TLS)
#define IMAGE_REL_MIPS_TOKEN            0x000E  // clr token
#define IMAGE_REL_MIPS_JMPADDR16        0x0010
#define IMAGE_REL_MIPS_REFWORDNB        0x0022
#define IMAGE_REL_MIPS_PAIR             0x0025

//
// Alpha Relocation types.
//
#define IMAGE_REL_ALPHA_ABSOLUTE        0x0000
#define IMAGE_REL_ALPHA_REFLONG         0x0001
#define IMAGE_REL_ALPHA_REFQUAD         0x0002
#define IMAGE_REL_ALPHA_GPREL32         0x0003
#define IMAGE_REL_ALPHA_LITERAL         0x0004
#define IMAGE_REL_ALPHA_LITUSE          0x0005
#define IMAGE_REL_ALPHA_GPDISP          0x0006
#define IMAGE_REL_ALPHA_BRADDR          0x0007
#define IMAGE_REL_ALPHA_HINT            0x0008
#define IMAGE_REL_ALPHA_INLINE_REFLONG  0x0009
#define IMAGE_REL_ALPHA_REFHI           0x000A
#define IMAGE_REL_ALPHA_REFLO           0x000B
#define IMAGE_REL_ALPHA_PAIR            0x000C
#define IMAGE_REL_ALPHA_MATCH           0x000D
#define IMAGE_REL_ALPHA_SECTION         0x000E
#define IMAGE_REL_ALPHA_SECREL          0x000F
#define IMAGE_REL_ALPHA_REFLONGNB       0x0010
#define IMAGE_REL_ALPHA_SECRELLO        0x0011  // Low 16-bit section relative reference
#define IMAGE_REL_ALPHA_SECRELHI        0x0012  // High 16-bit section relative reference
#define IMAGE_REL_ALPHA_REFQ3           0x0013  // High 16 bits of 48 bit reference
#define IMAGE_REL_ALPHA_REFQ2           0x0014  // Middle 16 bits of 48 bit reference
#define IMAGE_REL_ALPHA_REFQ1           0x0015  // Low 16 bits of 48 bit reference
#define IMAGE_REL_ALPHA_GPRELLO         0x0016  // Low 16-bit GP relative reference
#define IMAGE_REL_ALPHA_GPRELHI         0x0017  // High 16-bit GP relative reference

//
// IBM PowerPC relocation types.
//
#define IMAGE_REL_PPC_ABSOLUTE          0x0000  // NOP
#define IMAGE_REL_PPC_ADDR64            0x0001  // 64-bit address
#define IMAGE_REL_PPC_ADDR32            0x0002  // 32-bit address
#define IMAGE_REL_PPC_ADDR24            0x0003  // 26-bit address, shifted left 2 (branch absolute)
#define IMAGE_REL_PPC_ADDR16            0x0004  // 16-bit address
#define IMAGE_REL_PPC_ADDR14            0x0005  // 16-bit address, shifted left 2 (load doubleword)
#define IMAGE_REL_PPC_REL24             0x0006  // 26-bit PC-relative offset, shifted left 2 (branch relative)
#define IMAGE_REL_PPC_REL14             0x0007  // 16-bit PC-relative offset, shifted left 2 (br cond relative)
#define IMAGE_REL_PPC_TOCREL16          0x0008  // 16-bit offset from TOC base
#define IMAGE_REL_PPC_TOCREL14          0x0009  // 16-bit offset from TOC base, shifted left 2 (load doubleword)

#define IMAGE_REL_PPC_ADDR32NB          0x000A  // 32-bit addr w/o image base
#define IMAGE_REL_PPC_SECREL            0x000B  // va of containing section (as in an image sectionhdr)
#define IMAGE_REL_PPC_SECTION           0x000C  // sectionheader number
#define IMAGE_REL_PPC_IFGLUE            0x000D  // substitute TOC restore instruction iff symbol is glue code
#define IMAGE_REL_PPC_IMGLUE            0x000E  // symbol is glue code; virtual address is TOC restore instruction
#define IMAGE_REL_PPC_SECREL16          0x000F  // va of containing section (limited to 16 bits)
#define IMAGE_REL_PPC_REFHI             0x0010
#define IMAGE_REL_PPC_REFLO             0x0011
#define IMAGE_REL_PPC_PAIR              0x0012
#define IMAGE_REL_PPC_SECRELLO          0x0013  // Low 16-bit section relative reference (used for >32k TLS)
#define IMAGE_REL_PPC_SECRELHI          0x0014  // High 16-bit section relative reference (used for >32k TLS)
#define IMAGE_REL_PPC_GPREL             0x0015
#define IMAGE_REL_PPC_TOKEN             0x0016  // clr token

#define IMAGE_REL_PPC_TYPEMASK          0x00FF  // mask to isolate above values in IMAGE_RELOCATION.Type

// Flag bits in IMAGE_RELOCATION.TYPE

#define IMAGE_REL_PPC_NEG               0x0100  // subtract reloc value rather than adding it
#define IMAGE_REL_PPC_BRTAKEN           0x0200  // fix branch prediction bit to predict branch taken
#define IMAGE_REL_PPC_BRNTAKEN          0x0400  // fix branch prediction bit to predict branch not taken
#define IMAGE_REL_PPC_TOCDEFN           0x0800  // toc slot defined in file (or, data in toc)

//
// Hitachi SH3 relocation types.
//
#define IMAGE_REL_SH3_ABSOLUTE          0x0000  // No relocation
#define IMAGE_REL_SH3_DIRECT16          0x0001  // 16 bit direct
#define IMAGE_REL_SH3_DIRECT32          0x0002  // 32 bit direct
#define IMAGE_REL_SH3_DIRECT8           0x0003  // 8 bit direct, -128..255
#define IMAGE_REL_SH3_DIRECT8_WORD      0x0004  // 8 bit direct .W (0 ext.)
#define IMAGE_REL_SH3_DIRECT8_LONG      0x0005  // 8 bit direct .L (0 ext.)
#define IMAGE_REL_SH3_DIRECT4           0x0006  // 4 bit direct (0 ext.)
#define IMAGE_REL_SH3_DIRECT4_WORD      0x0007  // 4 bit direct .W (0 ext.)
#define IMAGE_REL_SH3_DIRECT4_LONG      0x0008  // 4 bit direct .L (0 ext.)
#define IMAGE_REL_SH3_PCREL8_WORD       0x0009  // 8 bit PC relative .W
#define IMAGE_REL_SH3_PCREL8_LONG       0x000A  // 8 bit PC relative .L
#define IMAGE_REL_SH3_PCREL12_WORD      0x000B  // 12 LSB PC relative .W
#define IMAGE_REL_SH3_STARTOF_SECTION   0x000C  // Start of EXE section
#define IMAGE_REL_SH3_SIZEOF_SECTION    0x000D  // Size of EXE section
#define IMAGE_REL_SH3_SECTION           0x000E  // Section table index
#define IMAGE_REL_SH3_SECREL            0x000F  // Offset within section
#define IMAGE_REL_SH3_DIRECT32_NB       0x0010  // 32 bit direct not based
#define IMAGE_REL_SH3_GPREL4_LONG       0x0011  // GP-relative addressing
#define IMAGE_REL_SH3_TOKEN             0x0012  // clr token
#define IMAGE_REL_SHM_PCRELPT           0x0013  // Offset from current
                                                //  instruction in longwords
                                                //  if not NOMODE, insert the
                                                //  inverse of the low bit at
                                                //  bit 32 to select PTA/PTB
#define IMAGE_REL_SHM_REFLO             0x0014  // Low bits of 32-bit address
#define IMAGE_REL_SHM_REFHALF           0x0015  // High bits of 32-bit address
#define IMAGE_REL_SHM_RELLO             0x0016  // Low bits of relative reference
#define IMAGE_REL_SHM_RELHALF           0x0017  // High bits of relative reference
#define IMAGE_REL_SHM_PAIR              0x0018  // offset operand for relocation

#define IMAGE_REL_SH_NOMODE             0x8000  // relocation ignores section mode


#define IMAGE_REL_ARM_ABSOLUTE          0x0000  // No relocation required
#define IMAGE_REL_ARM_ADDR32            0x0001  // 32 bit address
#define IMAGE_REL_ARM_ADDR32NB          0x0002  // 32 bit address w/o image base
#define IMAGE_REL_ARM_BRANCH24          0x0003  // 24 bit offset << 2 & sign ext.
#define IMAGE_REL_ARM_BRANCH11          0x0004  // Thumb: 2 11 bit offsets
#define IMAGE_REL_ARM_TOKEN             0x0005  // clr token
#define IMAGE_REL_ARM_GPREL12           0x0006  // GP-relative addressing (ARM)
#define IMAGE_REL_ARM_GPREL7            0x0007  // GP-relative addressing (Thumb)
#define IMAGE_REL_ARM_BLX24             0x0008
#define IMAGE_REL_ARM_BLX11             0x0009
#define IMAGE_REL_ARM_SECTION           0x000E  // Section table index
#define IMAGE_REL_ARM_SECREL            0x000F  // Offset within section
#define IMAGE_REL_ARM_MOV32A            0x0010  // ARM: MOVW/MOVT
#define IMAGE_REL_ARM_MOV32             0x0010  // ARM: MOVW/MOVT (deprecated)
#define IMAGE_REL_ARM_MOV32T            0x0011  // Thumb: MOVW/MOVT
#define IMAGE_REL_THUMB_MOV32           0x0011  // Thumb: MOVW/MOVT (deprecated)
#define IMAGE_REL_ARM_BRANCH20T         0x0012  // Thumb: 32-bit conditional B
#define IMAGE_REL_THUMB_BRANCH20        0x0012  // Thumb: 32-bit conditional B (deprecated)
#define IMAGE_REL_ARM_BRANCH24T         0x0014  // Thumb: 32-bit B or BL
#define IMAGE_REL_THUMB_BRANCH24        0x0014  // Thumb: 32-bit B or BL (deprecated)
#define IMAGE_REL_ARM_BLX23T            0x0015  // Thumb: BLX immediate
#define IMAGE_REL_THUMB_BLX23           0x0015  // Thumb: BLX immediate (deprecated)

#define IMAGE_REL_AM_ABSOLUTE           0x0000
#define IMAGE_REL_AM_ADDR32             0x0001
#define IMAGE_REL_AM_ADDR32NB           0x0002
#define IMAGE_REL_AM_CALL32             0x0003
#define IMAGE_REL_AM_FUNCINFO           0x0004
#define IMAGE_REL_AM_REL32_1            0x0005
#define IMAGE_REL_AM_REL32_2            0x0006
#define IMAGE_REL_AM_SECREL             0x0007
#define IMAGE_REL_AM_SECTION            0x0008
#define IMAGE_REL_AM_TOKEN              0x0009

//
// ARM64 relocations types.
//

#define IMAGE_REL_ARM64_ABSOLUTE        0x0000  // No relocation required
#define IMAGE_REL_ARM64_ADDR32          0x0001  // 32 bit address. Review! do we need it?
#define IMAGE_REL_ARM64_ADDR32NB        0x0002  // 32 bit address w/o image base (RVA: for Data/PData/XData)
#define IMAGE_REL_ARM64_BRANCH26        0x0003  // 26 bit offset << 2 & sign ext. for B & BL
#define IMAGE_REL_ARM64_PAGEBASE_REL21  0x0004  // ADRP
#define IMAGE_REL_ARM64_REL21           0x0005  // ADR
#define IMAGE_REL_ARM64_PAGEOFFSET_12A  0x0006  // ADD/ADDS (immediate) with zero shift, for page offset
#define IMAGE_REL_ARM64_PAGEOFFSET_12L  0x0007  // LDR (indexed, unsigned immediate), for page offset
#define IMAGE_REL_ARM64_SECREL          0x0008  // Offset within section
#define IMAGE_REL_ARM64_SECREL_LOW12A   0x0009  // ADD/ADDS (immediate) with zero shift, for bit 0:11 of section offset
#define IMAGE_REL_ARM64_SECREL_HIGH12A  0x000A  // ADD/ADDS (immediate) with zero shift, for bit 12:23 of section offset
#define IMAGE_REL_ARM64_SECREL_LOW12L   0x000B  // LDR (indexed, unsigned immediate), for bit 0:11 of section offset
#define IMAGE_REL_ARM64_TOKEN           0x000C
#define IMAGE_REL_ARM64_SECTION         0x000D  // Section table index
#define IMAGE_REL_ARM64_ADDR64          0x000E  // 64 bit address
#define IMAGE_REL_ARM64_BRANCH19        0x000F  // 19 bit offset << 2 & sign ext. for conditional B

//
// x64 relocations
//
#define IMAGE_REL_AMD64_ABSOLUTE        0x0000  // Reference is absolute, no relocation is necessary
#define IMAGE_REL_AMD64_ADDR64          0x0001  // 64-bit address (VA).
#define IMAGE_REL_AMD64_ADDR32          0x0002  // 32-bit address (VA).
#define IMAGE_REL_AMD64_ADDR32NB        0x0003  // 32-bit address w/o image base (RVA).
#define IMAGE_REL_AMD64_REL32           0x0004  // 32-bit relative address from byte following reloc
#define IMAGE_REL_AMD64_REL32_1         0x0005  // 32-bit relative address from byte distance 1 from reloc
#define IMAGE_REL_AMD64_REL32_2         0x0006  // 32-bit relative address from byte distance 2 from reloc
#define IMAGE_REL_AMD64_REL32_3         0x0007  // 32-bit relative address from byte distance 3 from reloc
#define IMAGE_REL_AMD64_REL32_4         0x0008  // 32-bit relative address from byte distance 4 from reloc
#define IMAGE_REL_AMD64_REL32_5         0x0009  // 32-bit relative address from byte distance 5 from reloc
#define IMAGE_REL_AMD64_SECTION         0x000A  // Section index
#define IMAGE_REL_AMD64_SECREL          0x000B  // 32 bit offset from base of section containing target
#define IMAGE_REL_AMD64_SECREL7         0x000C  // 7 bit unsigned offset from base of section containing target
#define IMAGE_REL_AMD64_TOKEN           0x000D  // 32 bit metadata token
#define IMAGE_REL_AMD64_SREL32          0x000E  // 32 bit signed span-dependent value emitted into object
#define IMAGE_REL_AMD64_PAIR            0x000F
#define IMAGE_REL_AMD64_SSPAN32         0x0010  // 32 bit signed span-dependent value applied at link time
#define IMAGE_REL_AMD64_EHANDLER        0x0011
#define IMAGE_REL_AMD64_IMPORT_BR       0x0012  // Indirect branch to an import
#define IMAGE_REL_AMD64_IMPORT_CALL     0x0013  // Indirect call to an import
#define IMAGE_REL_AMD64_CFG_BR          0x0014  // Indirect branch to a CFG check
#define IMAGE_REL_AMD64_CFG_BR_REX      0x0015  // Indirect branch to a CFG check, with REX.W prefix
#define IMAGE_REL_AMD64_CFG_CALL        0x0016  // Indirect call to a CFG check
#define IMAGE_REL_AMD64_INDIR_BR        0x0017  // Indirect branch to a target in RAX (no CFG)
#define IMAGE_REL_AMD64_INDIR_BR_REX    0x0018  // Indirect branch to a target in RAX, with REX.W prefix (no CFG)
#define IMAGE_REL_AMD64_INDIR_CALL      0x0019  // Indirect call to a target in RAX (no CFG)
#define IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST  0x0020 // Indirect branch for a switch table using Reg 0 (RAX)
#define IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST   0x002F // Indirect branch for a switch table using Reg 15 (R15)

//
// IA64 relocation types.
//
#define IMAGE_REL_IA64_ABSOLUTE         0x0000
#define IMAGE_REL_IA64_IMM14            0x0001
#define IMAGE_REL_IA64_IMM22            0x0002
#define IMAGE_REL_IA64_IMM64            0x0003
#define IMAGE_REL_IA64_DIR32            0x0004
#define IMAGE_REL_IA64_DIR64            0x0005
#define IMAGE_REL_IA64_PCREL21B         0x0006
#define IMAGE_REL_IA64_PCREL21M         0x0007
#define IMAGE_REL_IA64_PCREL21F         0x0008
#define IMAGE_REL_IA64_GPREL22          0x0009
#define IMAGE_REL_IA64_LTOFF22          0x000A
#define IMAGE_REL_IA64_SECTION          0x000B
#define IMAGE_REL_IA64_SECREL22         0x000C
#define IMAGE_REL_IA64_SECREL64I        0x000D
#define IMAGE_REL_IA64_SECREL32         0x000E
//
#define IMAGE_REL_IA64_DIR32NB          0x0010
#define IMAGE_REL_IA64_SREL14           0x0011
#define IMAGE_REL_IA64_SREL22           0x0012
#define IMAGE_REL_IA64_SREL32           0x0013
#define IMAGE_REL_IA64_UREL32           0x0014
#define IMAGE_REL_IA64_PCREL60X         0x0015  // This is always a BRL and never converted
#define IMAGE_REL_IA64_PCREL60B         0x0016  // If possible, convert to MBB bundle with NOP.B in slot 1
#define IMAGE_REL_IA64_PCREL60F         0x0017  // If possible, convert to MFB bundle with NOP.F in slot 1
#define IMAGE_REL_IA64_PCREL60I         0x0018  // If possible, convert to MIB bundle with NOP.I in slot 1
#define IMAGE_REL_IA64_PCREL60M         0x0019  // If possible, convert to MMB bundle with NOP.M in slot 1
#define IMAGE_REL_IA64_IMMGPREL64       0x001A
#define IMAGE_REL_IA64_TOKEN            0x001B  // clr token
#define IMAGE_REL_IA64_GPREL32          0x001C
#define IMAGE_REL_IA64_ADDEND           0x001F

//
// CEF relocation types.
//
#define IMAGE_REL_CEF_ABSOLUTE          0x0000  // Reference is absolute, no relocation is necessary
#define IMAGE_REL_CEF_ADDR32            0x0001  // 32-bit address (VA).
#define IMAGE_REL_CEF_ADDR64            0x0002  // 64-bit address (VA).
#define IMAGE_REL_CEF_ADDR32NB          0x0003  // 32-bit address w/o image base (RVA).
#define IMAGE_REL_CEF_SECTION           0x0004  // Section index
#define IMAGE_REL_CEF_SECREL            0x0005  // 32 bit offset from base of section containing target
#define IMAGE_REL_CEF_TOKEN             0x0006  // 32 bit metadata token

//
// clr relocation types.
//
#define IMAGE_REL_CEE_ABSOLUTE          0x0000  // Reference is absolute, no relocation is necessary
#define IMAGE_REL_CEE_ADDR32            0x0001  // 32-bit address (VA).
#define IMAGE_REL_CEE_ADDR64            0x0002  // 64-bit address (VA).
#define IMAGE_REL_CEE_ADDR32NB          0x0003  // 32-bit address w/o image base (RVA).
#define IMAGE_REL_CEE_SECTION           0x0004  // Section index
#define IMAGE_REL_CEE_SECREL            0x0005  // 32 bit offset from base of section containing target
#define IMAGE_REL_CEE_TOKEN             0x0006  // 32 bit metadata token


#define IMAGE_REL_M32R_ABSOLUTE         0x0000  // No relocation required
#define IMAGE_REL_M32R_ADDR32           0x0001  // 32 bit address
#define IMAGE_REL_M32R_ADDR32NB         0x0002  // 32 bit address w/o image base
#define IMAGE_REL_M32R_ADDR24           0x0003  // 24 bit address
#define IMAGE_REL_M32R_GPREL16          0x0004  // GP relative addressing
#define IMAGE_REL_M32R_PCREL24          0x0005  // 24 bit offset << 2 & sign ext.
#define IMAGE_REL_M32R_PCREL16          0x0006  // 16 bit offset << 2 & sign ext.
#define IMAGE_REL_M32R_PCREL8           0x0007  // 8 bit offset << 2 & sign ext.
#define IMAGE_REL_M32R_REFHALF          0x0008  // 16 MSBs
#define IMAGE_REL_M32R_REFHI            0x0009  // 16 MSBs; adj for LSB sign ext.
#define IMAGE_REL_M32R_REFLO            0x000A  // 16 LSBs
#define IMAGE_REL_M32R_PAIR             0x000B  // Link HI and LO
#define IMAGE_REL_M32R_SECTION          0x000C  // Section table index
#define IMAGE_REL_M32R_SECREL32         0x000D  // 32 bit section relative reference
#define IMAGE_REL_M32R_TOKEN            0x000E  // clr token

#define IMAGE_REL_EBC_ABSOLUTE          0x0000  // No relocation required
#define IMAGE_REL_EBC_ADDR32NB          0x0001  // 32 bit address w/o image base
#define IMAGE_REL_EBC_REL32             0x0002  // 32-bit relative address from byte following reloc
#define IMAGE_REL_EBC_SECTION           0x0003  // Section table index
#define IMAGE_REL_EBC_SECREL            0x0004  // Offset within section

#define EXT_IMM64(Value, Address, Size, InstPos, ValPos)  /* Intel-IA64-Filler */           \
    Value |= (((ULONGLONG)((*(Address) >> InstPos) & (((ULONGLONG)1 << Size) - 1))) << ValPos)  // Intel-IA64-Filler

#define INS_IMM64(Value, Address, Size, InstPos, ValPos)  /* Intel-IA64-Filler */\
    *(PDWORD)Address = (*(PDWORD)Address & ~(((1 << Size) - 1) << InstPos)) | /* Intel-IA64-Filler */\
          ((DWORD)((((ULONGLONG)Value >> ValPos) & (((ULONGLONG)1 << Size) - 1))) << InstPos)  // Intel-IA64-Filler

#define EMARCH_ENC_I17_IMM7B_INST_WORD_X         3  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM7B_SIZE_X              7  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X     4  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM7B_VAL_POS_X           0  // Intel-IA64-Filler

#define EMARCH_ENC_I17_IMM9D_INST_WORD_X         3  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM9D_SIZE_X              9  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X     18 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM9D_VAL_POS_X           7  // Intel-IA64-Filler

#define EMARCH_ENC_I17_IMM5C_INST_WORD_X         3  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM5C_SIZE_X              5  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X     13 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM5C_VAL_POS_X           16 // Intel-IA64-Filler

#define EMARCH_ENC_I17_IC_INST_WORD_X            3  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IC_SIZE_X                 1  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IC_INST_WORD_POS_X        12 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IC_VAL_POS_X              21 // Intel-IA64-Filler

#define EMARCH_ENC_I17_IMM41a_INST_WORD_X        1  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41a_SIZE_X             10 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41a_INST_WORD_POS_X    14 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41a_VAL_POS_X          22 // Intel-IA64-Filler

#define EMARCH_ENC_I17_IMM41b_INST_WORD_X        1  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41b_SIZE_X             8  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41b_INST_WORD_POS_X    24 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41b_VAL_POS_X          32 // Intel-IA64-Filler

#define EMARCH_ENC_I17_IMM41c_INST_WORD_X        2  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41c_SIZE_X             23 // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41c_INST_WORD_POS_X    0  // Intel-IA64-Filler
#define EMARCH_ENC_I17_IMM41c_VAL_POS_X          40 // Intel-IA64-Filler

#define EMARCH_ENC_I17_SIGN_INST_WORD_X          3  // Intel-IA64-Filler
#define EMARCH_ENC_I17_SIGN_SIZE_X               1  // Intel-IA64-Filler
#define EMARCH_ENC_I17_SIGN_INST_WORD_POS_X      27 // Intel-IA64-Filler
#define EMARCH_ENC_I17_SIGN_VAL_POS_X            63 // Intel-IA64-Filler

#define X3_OPCODE_INST_WORD_X                    3  // Intel-IA64-Filler
#define X3_OPCODE_SIZE_X                         4  // Intel-IA64-Filler
#define X3_OPCODE_INST_WORD_POS_X                28 // Intel-IA64-Filler
#define X3_OPCODE_SIGN_VAL_POS_X                 0  // Intel-IA64-Filler

#define X3_I_INST_WORD_X                         3  // Intel-IA64-Filler
#define X3_I_SIZE_X                              1  // Intel-IA64-Filler
#define X3_I_INST_WORD_POS_X                     27 // Intel-IA64-Filler
#define X3_I_SIGN_VAL_POS_X                      59 // Intel-IA64-Filler

#define X3_D_WH_INST_WORD_X                      3  // Intel-IA64-Filler
#define X3_D_WH_SIZE_X                           3  // Intel-IA64-Filler
#define X3_D_WH_INST_WORD_POS_X                  24 // Intel-IA64-Filler
#define X3_D_WH_SIGN_VAL_POS_X                   0  // Intel-IA64-Filler

#define X3_IMM20_INST_WORD_X                     3  // Intel-IA64-Filler
#define X3_IMM20_SIZE_X                          20 // Intel-IA64-Filler
#define X3_IMM20_INST_WORD_POS_X                 4  // Intel-IA64-Filler
#define X3_IMM20_SIGN_VAL_POS_X                  0  // Intel-IA64-Filler

#define X3_IMM39_1_INST_WORD_X                   2  // Intel-IA64-Filler
#define X3_IMM39_1_SIZE_X                        23 // Intel-IA64-Filler
#define X3_IMM39_1_INST_WORD_POS_X               0  // Intel-IA64-Filler
#define X3_IMM39_1_SIGN_VAL_POS_X                36 // Intel-IA64-Filler

#define X3_IMM39_2_INST_WORD_X                   1  // Intel-IA64-Filler
#define X3_IMM39_2_SIZE_X                        16 // Intel-IA64-Filler
#define X3_IMM39_2_INST_WORD_POS_X               16 // Intel-IA64-Filler
#define X3_IMM39_2_SIGN_VAL_POS_X                20 // Intel-IA64-Filler

#define X3_P_INST_WORD_X                         3  // Intel-IA64-Filler
#define X3_P_SIZE_X                              4  // Intel-IA64-Filler
#define X3_P_INST_WORD_POS_X                     0  // Intel-IA64-Filler
#define X3_P_SIGN_VAL_POS_X                      0  // Intel-IA64-Filler

#define X3_TMPLT_INST_WORD_X                     0  // Intel-IA64-Filler
#define X3_TMPLT_SIZE_X                          4  // Intel-IA64-Filler
#define X3_TMPLT_INST_WORD_POS_X                 0  // Intel-IA64-Filler
#define X3_TMPLT_SIGN_VAL_POS_X                  0  // Intel-IA64-Filler

#define X3_BTYPE_QP_INST_WORD_X                  2  // Intel-IA64-Filler
#define X3_BTYPE_QP_SIZE_X                       9  // Intel-IA64-Filler
#define X3_BTYPE_QP_INST_WORD_POS_X              23 // Intel-IA64-Filler
#define X3_BTYPE_QP_INST_VAL_POS_X               0  // Intel-IA64-Filler

#define X3_EMPTY_INST_WORD_X                     1  // Intel-IA64-Filler
#define X3_EMPTY_SIZE_X                          2  // Intel-IA64-Filler
#define X3_EMPTY_INST_WORD_POS_X                 14 // Intel-IA64-Filler
#define X3_EMPTY_INST_VAL_POS_X                  0  // Intel-IA64-Filler

//
// Line number format.
//

typedef struct _IMAGE_LINENUMBER {
    union {
        DWORD   SymbolTableIndex;               // Symbol table index of function name if Linenumber is 0.
        DWORD   VirtualAddress;                 // Virtual address of line number.
    } Type;
    WORD    Linenumber;                         // Line number.
} IMAGE_LINENUMBER;
typedef IMAGE_LINENUMBER UNALIGNED *PIMAGE_LINENUMBER;

#ifndef _MAC
#include "poppack.h"                        // Back to 4 byte packing
#endif

//
// Based relocation format.
//

//@[comment("MVI_tracked")]
typedef struct _IMAGE_BASE_RELOCATION {
    DWORD   VirtualAddress;
    DWORD   SizeOfBlock;
//  WORD    TypeOffset[1];
} IMAGE_BASE_RELOCATION;
typedef IMAGE_BASE_RELOCATION UNALIGNED * PIMAGE_BASE_RELOCATION;

//
// Based relocation types.
//

#define IMAGE_REL_BASED_ABSOLUTE              0
#define IMAGE_REL_BASED_HIGH                  1
#define IMAGE_REL_BASED_LOW                   2
#define IMAGE_REL_BASED_HIGHLOW               3
#define IMAGE_REL_BASED_HIGHADJ               4
#define IMAGE_REL_BASED_MACHINE_SPECIFIC_5    5
#define IMAGE_REL_BASED_RESERVED              6
#define IMAGE_REL_BASED_MACHINE_SPECIFIC_7    7
#define IMAGE_REL_BASED_MACHINE_SPECIFIC_8    8
#define IMAGE_REL_BASED_MACHINE_SPECIFIC_9    9
#define IMAGE_REL_BASED_DIR64                 10

//
// Platform-specific based relocation types.
//

#define IMAGE_REL_BASED_IA64_IMM64            9

#define IMAGE_REL_BASED_MIPS_JMPADDR          5
#define IMAGE_REL_BASED_MIPS_JMPADDR16        9

#define IMAGE_REL_BASED_ARM_MOV32             5
#define IMAGE_REL_BASED_THUMB_MOV32           7


//
// Archive format.
//

#define IMAGE_ARCHIVE_START_SIZE             8
#define IMAGE_ARCHIVE_START                  "!<arch>\n"
#define IMAGE_ARCHIVE_END                    "`\n"
#define IMAGE_ARCHIVE_PAD                    "\n"
#define IMAGE_ARCHIVE_LINKER_MEMBER          "/               "
#define IMAGE_ARCHIVE_LONGNAMES_MEMBER       "//              "
#define IMAGE_ARCHIVE_HYBRIDMAP_MEMBER       "/<HYBRIDMAP>/   "


typedef struct _IMAGE_ARCHIVE_MEMBER_HEADER {
    BYTE     Name[16];                          // File member name - `/' terminated.
    BYTE     Date[12];                          // File member date - decimal.
    BYTE     UserID[6];                         // File member user id - decimal.
    BYTE     GroupID[6];                        // File member group id - decimal.
    BYTE     Mode[8];                           // File member mode - octal.
    BYTE     Size[10];                          // File member size - decimal.
    BYTE     EndHeader[2];                      // String to end header.
} IMAGE_ARCHIVE_MEMBER_HEADER, *PIMAGE_ARCHIVE_MEMBER_HEADER;

#define IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR      60

//
// DLL support.
//

//
// Export Format
//

//@[comment("MVI_tracked")]
typedef struct _IMAGE_EXPORT_DIRECTORY {
    DWORD   Characteristics;
    DWORD   TimeDateStamp;
    WORD    MajorVersion;
    WORD    MinorVersion;
    DWORD   Name;
    DWORD   Base;
    DWORD   NumberOfFunctions;
    DWORD   NumberOfNames;
    DWORD   AddressOfFunctions;     // RVA from base of image
    DWORD   AddressOfNames;         // RVA from base of image
    DWORD   AddressOfNameOrdinals;  // RVA from base of image
} IMAGE_EXPORT_DIRECTORY, *PIMAGE_EXPORT_DIRECTORY;

//
// Import Format
//

//@[comment("MVI_tracked")]
typedef struct _IMAGE_IMPORT_BY_NAME {
    WORD    Hint;
    CHAR   Name[1];
} IMAGE_IMPORT_BY_NAME, *PIMAGE_IMPORT_BY_NAME;

#include "pshpack8.h"                       // Use align 8 for the 64-bit IAT.

//@[comment("MVI_tracked")]
typedef struct _IMAGE_THUNK_DATA64 {
    union {
        ULONGLONG ForwarderString;  // PBYTE 
        ULONGLONG Function;         // PDWORD
        ULONGLONG Ordinal;
        ULONGLONG AddressOfData;    // PIMAGE_IMPORT_BY_NAME
    } u1;
} IMAGE_THUNK_DATA64;
typedef IMAGE_THUNK_DATA64 * PIMAGE_THUNK_DATA64;

#include "poppack.h"                        // Back to 4 byte packing

//@[comment("MVI_tracked")]
typedef struct _IMAGE_THUNK_DATA32 {
    union {
        DWORD ForwarderString;      // PBYTE 
        DWORD Function;             // PDWORD
        DWORD Ordinal;
        DWORD AddressOfData;        // PIMAGE_IMPORT_BY_NAME
    } u1;
} IMAGE_THUNK_DATA32;
typedef IMAGE_THUNK_DATA32 * PIMAGE_THUNK_DATA32;

#define IMAGE_ORDINAL_FLAG64 0x8000000000000000
#define IMAGE_ORDINAL_FLAG32 0x80000000
#define IMAGE_ORDINAL64(Ordinal) (Ordinal & 0xffff)
#define IMAGE_ORDINAL32(Ordinal) (Ordinal & 0xffff)
#define IMAGE_SNAP_BY_ORDINAL64(Ordinal) ((Ordinal & IMAGE_ORDINAL_FLAG64) != 0)
#define IMAGE_SNAP_BY_ORDINAL32(Ordinal) ((Ordinal & IMAGE_ORDINAL_FLAG32) != 0)

//
// Thread Local Storage
//

typedef VOID
(NTAPI *PIMAGE_TLS_CALLBACK) (
    PVOID DllHandle,
    DWORD Reason,
    PVOID Reserved
    );

typedef struct _IMAGE_TLS_DIRECTORY64 {
    ULONGLONG StartAddressOfRawData;
    ULONGLONG EndAddressOfRawData;
    ULONGLONG AddressOfIndex;         // PDWORD
    ULONGLONG AddressOfCallBacks;     // PIMAGE_TLS_CALLBACK *;
    DWORD SizeOfZeroFill;
    union {
        DWORD Characteristics;
        struct {
            DWORD Reserved0 : 20;
            DWORD Alignment : 4;
            DWORD Reserved1 : 8;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

} IMAGE_TLS_DIRECTORY64;

typedef IMAGE_TLS_DIRECTORY64 * PIMAGE_TLS_DIRECTORY64;

typedef struct _IMAGE_TLS_DIRECTORY32 {
    DWORD   StartAddressOfRawData;
    DWORD   EndAddressOfRawData;
    DWORD   AddressOfIndex;             // PDWORD
    DWORD   AddressOfCallBacks;         // PIMAGE_TLS_CALLBACK *
    DWORD   SizeOfZeroFill;
    union {
        DWORD Characteristics;
        struct {
            DWORD Reserved0 : 20;
            DWORD Alignment : 4;
            DWORD Reserved1 : 8;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

} IMAGE_TLS_DIRECTORY32;
typedef IMAGE_TLS_DIRECTORY32 * PIMAGE_TLS_DIRECTORY32;

#ifdef _WIN64
#define IMAGE_ORDINAL_FLAG              IMAGE_ORDINAL_FLAG64
#define IMAGE_ORDINAL(Ordinal)          IMAGE_ORDINAL64(Ordinal)
typedef IMAGE_THUNK_DATA64              IMAGE_THUNK_DATA;
typedef PIMAGE_THUNK_DATA64             PIMAGE_THUNK_DATA;
#define IMAGE_SNAP_BY_ORDINAL(Ordinal)  IMAGE_SNAP_BY_ORDINAL64(Ordinal)
typedef IMAGE_TLS_DIRECTORY64           IMAGE_TLS_DIRECTORY;
typedef PIMAGE_TLS_DIRECTORY64          PIMAGE_TLS_DIRECTORY;
#else
#define IMAGE_ORDINAL_FLAG              IMAGE_ORDINAL_FLAG32
#define IMAGE_ORDINAL(Ordinal)          IMAGE_ORDINAL32(Ordinal)
typedef IMAGE_THUNK_DATA32              IMAGE_THUNK_DATA;
typedef PIMAGE_THUNK_DATA32             PIMAGE_THUNK_DATA;
#define IMAGE_SNAP_BY_ORDINAL(Ordinal)  IMAGE_SNAP_BY_ORDINAL32(Ordinal)
typedef IMAGE_TLS_DIRECTORY32           IMAGE_TLS_DIRECTORY;
typedef PIMAGE_TLS_DIRECTORY32          PIMAGE_TLS_DIRECTORY;
#endif

//@[comment("MVI_tracked")]
typedef struct _IMAGE_IMPORT_DESCRIPTOR {
    union {
        DWORD   Characteristics;            // 0 for terminating null import descriptor
        DWORD   OriginalFirstThunk;         // RVA to original unbound IAT (PIMAGE_THUNK_DATA)
    } DUMMYUNIONNAME;
    DWORD   TimeDateStamp;                  // 0 if not bound,
                                            // -1 if bound, and real date\time stamp
                                            //     in IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT (new BIND)
                                            // O.W. date/time stamp of DLL bound to (Old BIND)

    DWORD   ForwarderChain;                 // -1 if no forwarders
    DWORD   Name;
    DWORD   FirstThunk;                     // RVA to IAT (if bound this IAT has actual addresses)
} IMAGE_IMPORT_DESCRIPTOR;
typedef IMAGE_IMPORT_DESCRIPTOR UNALIGNED *PIMAGE_IMPORT_DESCRIPTOR;

//
// New format import descriptors pointed to by DataDirectory[ IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT ]
//

typedef struct _IMAGE_BOUND_IMPORT_DESCRIPTOR {
    DWORD   TimeDateStamp;
    WORD    OffsetModuleName;
    WORD    NumberOfModuleForwarderRefs;
// Array of zero or more IMAGE_BOUND_FORWARDER_REF follows
} IMAGE_BOUND_IMPORT_DESCRIPTOR,  *PIMAGE_BOUND_IMPORT_DESCRIPTOR;

typedef struct _IMAGE_BOUND_FORWARDER_REF {
    DWORD   TimeDateStamp;
    WORD    OffsetModuleName;
    WORD    Reserved;
} IMAGE_BOUND_FORWARDER_REF, *PIMAGE_BOUND_FORWARDER_REF;

typedef struct _IMAGE_DELAYLOAD_DESCRIPTOR {
    union {
        DWORD AllAttributes;
        struct {
            DWORD RvaBased : 1;             // Delay load version 2
            DWORD ReservedAttributes : 31;
        } DUMMYSTRUCTNAME;
    } Attributes;

    DWORD DllNameRVA;                       // RVA to the name of the target library (NULL-terminate ASCII string)
    DWORD ModuleHandleRVA;                  // RVA to the HMODULE caching location (PHMODULE)
    DWORD ImportAddressTableRVA;            // RVA to the start of the IAT (PIMAGE_THUNK_DATA)
    DWORD ImportNameTableRVA;               // RVA to the start of the name table (PIMAGE_THUNK_DATA::AddressOfData)
    DWORD BoundImportAddressTableRVA;       // RVA to an optional bound IAT
    DWORD UnloadInformationTableRVA;        // RVA to an optional unload info table
    DWORD TimeDateStamp;                    // 0 if not bound,
                                            // Otherwise, date/time of the target DLL

} IMAGE_DELAYLOAD_DESCRIPTOR, *PIMAGE_DELAYLOAD_DESCRIPTOR;

typedef const IMAGE_DELAYLOAD_DESCRIPTOR *PCIMAGE_DELAYLOAD_DESCRIPTOR;

//
// Resource Format.
//

//
// Resource directory consists of two counts, following by a variable length
// array of directory entries.  The first count is the number of entries at
// beginning of the array that have actual names associated with each entry.
// The entries are in ascending order, case insensitive strings.  The second
// count is the number of entries that immediately follow the named entries.
// This second count identifies the number of entries that have 16-bit integer
// Ids as their name.  These entries are also sorted in ascending order.
//
// This structure allows fast lookup by either name or number, but for any
// given resource entry only one form of lookup is supported, not both.
// This is consistant with the syntax of the .RC file and the .RES file.
//

typedef struct _IMAGE_RESOURCE_DIRECTORY {
    DWORD   Characteristics;
    DWORD   TimeDateStamp;
    WORD    MajorVersion;
    WORD    MinorVersion;
    WORD    NumberOfNamedEntries;
    WORD    NumberOfIdEntries;
//  IMAGE_RESOURCE_DIRECTORY_ENTRY DirectoryEntries[];
} IMAGE_RESOURCE_DIRECTORY, *PIMAGE_RESOURCE_DIRECTORY;

#define IMAGE_RESOURCE_NAME_IS_STRING        0x80000000
#define IMAGE_RESOURCE_DATA_IS_DIRECTORY     0x80000000
//
// Each directory contains the 32-bit Name of the entry and an offset,
// relative to the beginning of the resource directory of the data associated
// with this directory entry.  If the name of the entry is an actual text
// string instead of an integer Id, then the high order bit of the name field
// is set to one and the low order 31-bits are an offset, relative to the
// beginning of the resource directory of the string, which is of type
// IMAGE_RESOURCE_DIRECTORY_STRING.  Otherwise the high bit is clear and the
// low-order 16-bits are the integer Id that identify this resource directory
// entry. If the directory entry is yet another resource directory (i.e. a
// subdirectory), then the high order bit of the offset field will be
// set to indicate this.  Otherwise the high bit is clear and the offset
// field points to a resource data entry.
//

//@[comment("MVI_tracked")]
typedef struct _IMAGE_RESOURCE_DIRECTORY_ENTRY {
    union {
        struct {
            DWORD NameOffset:31;
            DWORD NameIsString:1;
        } DUMMYSTRUCTNAME;
        DWORD   Name;
        WORD    Id;
    } DUMMYUNIONNAME;
    union {
        DWORD   OffsetToData;
        struct {
            DWORD   OffsetToDirectory:31;
            DWORD   DataIsDirectory:1;
        } DUMMYSTRUCTNAME2;
    } DUMMYUNIONNAME2;
} IMAGE_RESOURCE_DIRECTORY_ENTRY, *PIMAGE_RESOURCE_DIRECTORY_ENTRY;

//
// For resource directory entries that have actual string names, the Name
// field of the directory entry points to an object of the following type.
// All of these string objects are stored together after the last resource
// directory entry and before the first resource data object.  This minimizes
// the impact of these variable length objects on the alignment of the fixed
// size directory entry objects.
//

typedef struct _IMAGE_RESOURCE_DIRECTORY_STRING {
    WORD    Length;
    CHAR    NameString[ 1 ];
} IMAGE_RESOURCE_DIRECTORY_STRING, *PIMAGE_RESOURCE_DIRECTORY_STRING;


typedef struct _IMAGE_RESOURCE_DIR_STRING_U {
    WORD    Length;
    WCHAR   NameString[ 1 ];
} IMAGE_RESOURCE_DIR_STRING_U, *PIMAGE_RESOURCE_DIR_STRING_U;


//
// Each resource data entry describes a leaf node in the resource directory
// tree.  It contains an offset, relative to the beginning of the resource
// directory of the data for the resource, a size field that gives the number
// of bytes of data at that offset, a CodePage that should be used when
// decoding code point values within the resource data.  Typically for new
// applications the code page would be the unicode code page.
//

//@[comment("MVI_tracked")]
typedef struct _IMAGE_RESOURCE_DATA_ENTRY {
    DWORD   OffsetToData;
    DWORD   Size;
    DWORD   CodePage;
    DWORD   Reserved;
} IMAGE_RESOURCE_DATA_ENTRY, *PIMAGE_RESOURCE_DATA_ENTRY;

// begin_ntoshvp

//
// Code Integrity in loadconfig (CI)
//

typedef struct _IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    WORD    Flags;          // Flags to indicate if CI information is available, etc.
    WORD    Catalog;        // 0xFFFF means not available
    DWORD   CatalogOffset;
    DWORD   Reserved;       // Additional bitmask to be defined later
} IMAGE_LOAD_CONFIG_CODE_INTEGRITY, *PIMAGE_LOAD_CONFIG_CODE_INTEGRITY;

//
// Dynamic value relocation table in loadconfig
//

typedef struct _IMAGE_DYNAMIC_RELOCATION_TABLE {
    DWORD Version;
    DWORD Size;
//  IMAGE_DYNAMIC_RELOCATION DynamicRelocations[0];
} IMAGE_DYNAMIC_RELOCATION_TABLE, *PIMAGE_DYNAMIC_RELOCATION_TABLE;

//
// Dynamic value relocation entries following IMAGE_DYNAMIC_RELOCATION_TABLE
//

#include "pshpack1.h"

typedef struct _IMAGE_DYNAMIC_RELOCATION32 {
    DWORD      Symbol;
    DWORD      BaseRelocSize;
//  IMAGE_BASE_RELOCATION BaseRelocations[0];
} IMAGE_DYNAMIC_RELOCATION32, *PIMAGE_DYNAMIC_RELOCATION32;

typedef struct _IMAGE_DYNAMIC_RELOCATION64 {
    ULONGLONG  Symbol;
    DWORD      BaseRelocSize;
//  IMAGE_BASE_RELOCATION BaseRelocations[0];
} IMAGE_DYNAMIC_RELOCATION64, *PIMAGE_DYNAMIC_RELOCATION64;

typedef struct _IMAGE_DYNAMIC_RELOCATION32_V2 {
    DWORD      HeaderSize;
    DWORD      FixupInfoSize;
    DWORD      Symbol;
    DWORD      SymbolGroup;
    DWORD      Flags;
    // ...     variable length header fields
    // BYTE    FixupInfo[FixupInfoSize]
} IMAGE_DYNAMIC_RELOCATION32_V2, *PIMAGE_DYNAMIC_RELOCATION32_V2;

typedef struct _IMAGE_DYNAMIC_RELOCATION64_V2 {
    DWORD      HeaderSize;
    DWORD      FixupInfoSize;
    ULONGLONG  Symbol;
    DWORD      SymbolGroup;
    DWORD      Flags;
    // ...     variable length header fields
    // BYTE    FixupInfo[FixupInfoSize]
} IMAGE_DYNAMIC_RELOCATION64_V2, *PIMAGE_DYNAMIC_RELOCATION64_V2;

#include "poppack.h"                    // Back to 4 byte packing

#ifdef _WIN64
typedef IMAGE_DYNAMIC_RELOCATION64          IMAGE_DYNAMIC_RELOCATION;
typedef PIMAGE_DYNAMIC_RELOCATION64         PIMAGE_DYNAMIC_RELOCATION;
typedef IMAGE_DYNAMIC_RELOCATION64_V2       IMAGE_DYNAMIC_RELOCATION_V2;
typedef PIMAGE_DYNAMIC_RELOCATION64_V2      PIMAGE_DYNAMIC_RELOCATION_V2;
#else
typedef IMAGE_DYNAMIC_RELOCATION32          IMAGE_DYNAMIC_RELOCATION;
typedef PIMAGE_DYNAMIC_RELOCATION32         PIMAGE_DYNAMIC_RELOCATION;
typedef IMAGE_DYNAMIC_RELOCATION32_V2       IMAGE_DYNAMIC_RELOCATION_V2;
typedef PIMAGE_DYNAMIC_RELOCATION32_V2      PIMAGE_DYNAMIC_RELOCATION_V2;
#endif

//
// Defined symbolic dynamic relocation entries.
//

#define IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE   0x00000001
#define IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE   0x00000002
#define IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER  0x00000003
#define IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER   0x00000004
#define IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH       0x00000005
#define IMAGE_DYNAMIC_RELOCATION_FUNCTION_OVERRIDE              0x00000007
#define IMAGE_DYNAMIC_RELOCATION_ARM64_KERNEL_IMPORT_CALL_TRANSFER 0x00000008

#include "pshpack1.h"

typedef struct _IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    BYTE       PrologueByteCount;
    // BYTE    PrologueBytes[PrologueByteCount];
} IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER;
typedef IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER UNALIGNED * PIMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER;

typedef struct _IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    DWORD      EpilogueCount;
    BYTE       EpilogueByteCount;
    BYTE       BranchDescriptorElementSize;
    WORD       BranchDescriptorCount;
    // BYTE    BranchDescriptors[...];
    // BYTE    BranchDescriptorBitMap[...];
} IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER;
typedef IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER UNALIGNED * PIMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER;

typedef struct _IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    DWORD       PageRelativeOffset : 12;
    DWORD       IndirectCall       : 1;
    DWORD       IATIndex           : 19;
} IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION;
typedef IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION UNALIGNED * PIMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION;

//
// On ARM64, an optimized imported function uses the following data structure
// insted of a _IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION.
//

typedef struct _IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION {
    DWORD PageRelativeOffset : 10;  // Offset to the call instruction shifted right by 2 (4-byte aligned instruction)
    DWORD IndirectCall       :  1;  // 0 if target instruction is a BR, 1 if BLR.
    DWORD RegisterIndex      :  5;  // Register index used for the indirect call/jump.
    DWORD ImportType         :  1;  // 0 if this refers to a static import, 1 for delayload import
    DWORD IATIndex           : 15;  // IAT index of the corresponding import.
                                    // 0x7FFF is a special value indicating no index.
} IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION;
typedef IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION UNALIGNED * PIMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION;

//
// Platform-independent Import Control transfer dynamic relocations definitions
//

#if defined(_AMD64_)

#define IMAGE_DYNAMIC_RELOCATION_IMPORT_CONTROL_TRANSFER IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER

typedef IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION IMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION,
                                                         * PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION;

#else

#define IMAGE_DYNAMIC_RELOCATION_IMPORT_CONTROL_TRANSFER IMAGE_DYNAMIC_RELOCATION_ARM64_KERNEL_IMPORT_CALL_TRANSFER

typedef IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION IMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION,
                                                       * PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION;

#endif

#if !defined(__midl) && !defined(MIDL_PASS)

C_ASSERT(sizeof(IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION) ==
         sizeof(IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION));

#endif

typedef struct _IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    WORD        PageRelativeOffset : 12;
    WORD        IndirectCall       : 1;
    WORD        RexWPrefix         : 1;
    WORD        CfgCheck           : 1;
    WORD        Reserved           : 1;
} IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION;
typedef IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION UNALIGNED * PIMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION;

typedef struct _IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    WORD        PageRelativeOffset : 12;
    WORD        RegisterNumber     : 4;
} IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION;
typedef IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION UNALIGNED * PIMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION;

typedef struct _IMAGE_FUNCTION_OVERRIDE_HEADER {
    DWORD FuncOverrideSize;
 // IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION  FuncOverrideInfo[ANYSIZE_ARRAY]; // FuncOverrideSize bytes in size
 // IMAGE_BDD_INFO BDDInfo; // BDD region, size in bytes: DVRTEntrySize - sizeof(IMAGE_FUNCTION_OVERRIDE_HEADER) - FuncOverrideSize
} IMAGE_FUNCTION_OVERRIDE_HEADER;
typedef IMAGE_FUNCTION_OVERRIDE_HEADER UNALIGNED * PIMAGE_FUNCTION_OVERRIDE_HEADER;

typedef struct _IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION {
    DWORD OriginalRva;          // RVA of original function
    DWORD BDDOffset;            // Offset into the BDD region
    DWORD RvaSize;              // Size in bytes taken by RVAs. Must be multiple of sizeof(DWORD).
    DWORD BaseRelocSize;        // Size in bytes taken by BaseRelocs

    // DWORD RVAs[RvaSize / sizeof(DWORD)];     // Array containing overriding func RVAs.

    // IMAGE_BASE_RELOCATION  BaseRelocs[ANYSIZE_ARRAY]; // Base relocations (RVA + Size + TO)
                                                         //  Padded with extra TOs for 4B alignment
                                                         // BaseRelocSize size in bytes
} IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION;
typedef IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION * PIMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION;

typedef struct _IMAGE_BDD_INFO {
    DWORD         Version;      // decides the semantics of serialized BDD
    DWORD         BDDSize;
    // IMAGE_BDD_DYNAMIC_RELOCATION BDDNodes[ANYSIZE_ARRAY]; // BDDSize size in bytes.
} IMAGE_BDD_INFO;
typedef IMAGE_BDD_INFO * PIMAGE_BDD_INFO;

typedef struct _IMAGE_BDD_DYNAMIC_RELOCATION {
    WORD   Left;                // Index of FALSE edge in BDD array
    WORD   Right;               // Index of TRUE edge in BDD array
    DWORD  Value;               // Either FeatureNumber or Index into RVAs array
} IMAGE_BDD_DYNAMIC_RELOCATION;
typedef IMAGE_BDD_DYNAMIC_RELOCATION * PIMAGE_BDD_DYNAMIC_RELOCATION;

// Function override relocation types in DVRT records.

#define IMAGE_FUNCTION_OVERRIDE_INVALID         0
#define IMAGE_FUNCTION_OVERRIDE_X64_REL32       1  // 32-bit relative address from byte following reloc
#define IMAGE_FUNCTION_OVERRIDE_ARM64_BRANCH26  2  // 26 bit offset << 2 & sign ext. for B & BL
#define IMAGE_FUNCTION_OVERRIDE_ARM64_THUNK     3

#include "poppack.h"                    // Back to 4 byte packing

//
// Load Configuration Directory Entry
//

typedef struct _IMAGE_LOAD_CONFIG_DIRECTORY32 {
    DWORD   Size;
    DWORD   TimeDateStamp;
    WORD    MajorVersion;
    WORD    MinorVersion;
    DWORD   GlobalFlagsClear;
    DWORD   GlobalFlagsSet;
    DWORD   CriticalSectionDefaultTimeout;
    DWORD   DeCommitFreeBlockThreshold;
    DWORD   DeCommitTotalFreeThreshold;
    DWORD   LockPrefixTable;                // VA
    DWORD   MaximumAllocationSize;
    DWORD   VirtualMemoryThreshold;
    DWORD   ProcessHeapFlags;
    DWORD   ProcessAffinityMask;
    WORD    CSDVersion;
    WORD    DependentLoadFlags;
    DWORD   EditList;                       // VA
    DWORD   SecurityCookie;                 // VA
    DWORD   SEHandlerTable;                 // VA
    DWORD   SEHandlerCount;
    DWORD   GuardCFCheckFunctionPointer;    // VA
    DWORD   GuardCFDispatchFunctionPointer; // VA
    DWORD   GuardCFFunctionTable;           // VA
    DWORD   GuardCFFunctionCount;
    DWORD   GuardFlags;
    IMAGE_LOAD_CONFIG_CODE_INTEGRITY CodeIntegrity;
    DWORD   GuardAddressTakenIatEntryTable; // VA
    DWORD   GuardAddressTakenIatEntryCount;
    DWORD   GuardLongJumpTargetTable;       // VA
    DWORD   GuardLongJumpTargetCount;
    DWORD   DynamicValueRelocTable;         // VA
    DWORD   CHPEMetadataPointer;
    DWORD   GuardRFFailureRoutine;          // VA
    DWORD   GuardRFFailureRoutineFunctionPointer; // VA
    DWORD   DynamicValueRelocTableOffset;
    WORD    DynamicValueRelocTableSection;
    WORD    Reserved2;
    DWORD   GuardRFVerifyStackPointerFunctionPointer; // VA
    DWORD   HotPatchTableOffset;
    DWORD   Reserved3;
    DWORD   EnclaveConfigurationPointer;    // VA
    DWORD   VolatileMetadataPointer;        // VA
    DWORD   GuardEHContinuationTable;       // VA
    DWORD   GuardEHContinuationCount;
    DWORD   GuardXFGCheckFunctionPointer;   // VA
    DWORD   GuardXFGDispatchFunctionPointer; // VA
    DWORD   GuardXFGTableDispatchFunctionPointer; // VA
    DWORD   CastGuardOsDeterminedFailureMode; // VA
    DWORD   GuardMemcpyFunctionPointer;     // VA
    DWORD   UmaFunctionPointers;            // VA
} IMAGE_LOAD_CONFIG_DIRECTORY32, *PIMAGE_LOAD_CONFIG_DIRECTORY32;

typedef struct _IMAGE_LOAD_CONFIG_DIRECTORY64 {
    DWORD      Size;
    DWORD      TimeDateStamp;
    WORD       MajorVersion;
    WORD       MinorVersion;
    DWORD      GlobalFlagsClear;
    DWORD      GlobalFlagsSet;
    DWORD      CriticalSectionDefaultTimeout;
    ULONGLONG  DeCommitFreeBlockThreshold;
    ULONGLONG  DeCommitTotalFreeThreshold;
    ULONGLONG  LockPrefixTable;                // VA
    ULONGLONG  MaximumAllocationSize;
    ULONGLONG  VirtualMemoryThreshold;
    ULONGLONG  ProcessAffinityMask;
    DWORD      ProcessHeapFlags;
    WORD       CSDVersion;
    WORD       DependentLoadFlags;
    ULONGLONG  EditList;                       // VA
    ULONGLONG  SecurityCookie;                 // VA
    ULONGLONG  SEHandlerTable;                 // VA
    ULONGLONG  SEHandlerCount;
    ULONGLONG  GuardCFCheckFunctionPointer;    // VA
    ULONGLONG  GuardCFDispatchFunctionPointer; // VA
    ULONGLONG  GuardCFFunctionTable;           // VA
    ULONGLONG  GuardCFFunctionCount;
    DWORD      GuardFlags;
    IMAGE_LOAD_CONFIG_CODE_INTEGRITY CodeIntegrity;
    ULONGLONG  GuardAddressTakenIatEntryTable; // VA
    ULONGLONG  GuardAddressTakenIatEntryCount;
    ULONGLONG  GuardLongJumpTargetTable;       // VA
    ULONGLONG  GuardLongJumpTargetCount;
    ULONGLONG  DynamicValueRelocTable;         // VA
    ULONGLONG  CHPEMetadataPointer;            // VA
    ULONGLONG  GuardRFFailureRoutine;          // VA
    ULONGLONG  GuardRFFailureRoutineFunctionPointer; // VA
    DWORD      DynamicValueRelocTableOffset;
    WORD       DynamicValueRelocTableSection;
    WORD       Reserved2;
    ULONGLONG  GuardRFVerifyStackPointerFunctionPointer; // VA
    DWORD      HotPatchTableOffset;
    DWORD      Reserved3;
    ULONGLONG  EnclaveConfigurationPointer;    // VA
    ULONGLONG  VolatileMetadataPointer;        // VA
    ULONGLONG  GuardEHContinuationTable;       // VA
    ULONGLONG  GuardEHContinuationCount;
    ULONGLONG  GuardXFGCheckFunctionPointer;   // VA
    ULONGLONG  GuardXFGDispatchFunctionPointer; // VA
    ULONGLONG  GuardXFGTableDispatchFunctionPointer; // VA
    ULONGLONG  CastGuardOsDeterminedFailureMode; // VA
    ULONGLONG  GuardMemcpyFunctionPointer;     // VA
    ULONGLONG  UmaFunctionPointers;            // VA
} IMAGE_LOAD_CONFIG_DIRECTORY64, *PIMAGE_LOAD_CONFIG_DIRECTORY64;

// end_ntoshvp
// begin_ntoshvp

#ifdef _WIN64
typedef IMAGE_LOAD_CONFIG_DIRECTORY64     IMAGE_LOAD_CONFIG_DIRECTORY;
typedef PIMAGE_LOAD_CONFIG_DIRECTORY64    PIMAGE_LOAD_CONFIG_DIRECTORY;
#else
typedef IMAGE_LOAD_CONFIG_DIRECTORY32     IMAGE_LOAD_CONFIG_DIRECTORY;
typedef PIMAGE_LOAD_CONFIG_DIRECTORY32    PIMAGE_LOAD_CONFIG_DIRECTORY;
#endif

// end_ntoshvp

#define IMAGE_HOT_PATCH_INFO_FLAG_PATCHORDERCRITICAL 0x00000001
#define IMAGE_HOT_PATCH_INFO_FLAG_HOTSWAP            0x00000002

typedef struct _IMAGE_HOT_PATCH_INFO {
    DWORD Version;
    DWORD Size;
    DWORD SequenceNumber;
    DWORD BaseImageList;
    DWORD BaseImageCount;
    DWORD BufferOffset;             // Version 2 and later
    DWORD ExtraPatchSize;           // Version 3 and later
    DWORD MinSequenceNumber;        // Version 4 and later
    DWORD Flags;                    // Version 4 and later
} IMAGE_HOT_PATCH_INFO, *PIMAGE_HOT_PATCH_INFO;

typedef struct _IMAGE_HOT_PATCH_BASE {
    DWORD SequenceNumber;
    DWORD Flags;
    DWORD OriginalTimeDateStamp;
    DWORD OriginalCheckSum;
    DWORD CodeIntegrityInfo;
    DWORD CodeIntegritySize;
    DWORD PatchTable;
    DWORD BufferOffset;             // Version 2 and later
} IMAGE_HOT_PATCH_BASE, *PIMAGE_HOT_PATCH_BASE;

typedef struct _IMAGE_HOT_PATCH_MACHINE {
    struct {
        DWORD _x86     :  1;
        DWORD Amd64    :  1;
        DWORD Arm64    :  1;
        DWORD Amd64EC  :  1;
    } DUMMYSTRUCTNAME;
} IMAGE_HOT_PATCH_MACHINE, *PIMAGE_HOT_PATCH_MACHINE;

typedef struct _IMAGE_HOT_PATCH_HASHES {
    BYTE  SHA256[32];
    BYTE  SHA1[20];
} IMAGE_HOT_PATCH_HASHES, *PIMAGE_HOT_PATCH_HASHES;

#define IMAGE_HOT_PATCH_BASE_OBLIGATORY     0x00000001
#define IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK  0x00000002

#define IMAGE_HOT_PATCH_BASE_MACHINE_I386   0x00000004
#define IMAGE_HOT_PATCH_BASE_MACHINE_ARM64  0x00000008
#define IMAGE_HOT_PATCH_BASE_MACHINE_AMD64  0x00000010

#define IMAGE_HOT_PATCH_CHUNK_INVERSE       0x80000000
#define IMAGE_HOT_PATCH_CHUNK_OBLIGATORY    0x40000000
#define IMAGE_HOT_PATCH_CHUNK_RESERVED      0x3FF03000
#define IMAGE_HOT_PATCH_CHUNK_TYPE          0x000FC000
#define IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA    0x00008000
#define IMAGE_HOT_PATCH_CHUNK_TARGET_RVA    0x00004000
#define IMAGE_HOT_PATCH_CHUNK_SIZE          0x00000FFF

#define IMAGE_HOT_PATCH_NONE                0x00000000
#define IMAGE_HOT_PATCH_FUNCTION            0x0001C000
#define IMAGE_HOT_PATCH_ABSOLUTE            0x0002C000
#define IMAGE_HOT_PATCH_REL32               0x0003C000
#define IMAGE_HOT_PATCH_CALL_TARGET         0x00044000
#define IMAGE_HOT_PATCH_INDIRECT            0x0005C000
#define IMAGE_HOT_PATCH_NO_CALL_TARGET      0x00064000
#define IMAGE_HOT_PATCH_DYNAMIC_VALUE       0x00078000

#define IMAGE_GUARD_CF_INSTRUMENTED                    0x00000100 // Module performs control flow integrity checks using system-supplied support
#define IMAGE_GUARD_CFW_INSTRUMENTED                   0x00000200 // Module performs control flow and write integrity checks
#define IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT          0x00000400 // Module contains valid control flow target metadata
#define IMAGE_GUARD_SECURITY_COOKIE_UNUSED             0x00000800 // Module does not make use of the /GS security cookie
#define IMAGE_GUARD_PROTECT_DELAYLOAD_IAT              0x00001000 // Module supports read only delay load IAT
#define IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION   0x00002000 // Delayload import table in its own .didat section (with nothing else in it) that can be freely reprotected
#define IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT 0x00004000 // Module contains suppressed export information. This also infers that the address taken
                                                                  // taken IAT table is also present in the load config.
#define IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION       0x00008000 // Module enables suppression of exports
#define IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT          0x00010000 // Module contains longjmp target information
#define IMAGE_GUARD_RF_INSTRUMENTED                    0x00020000 // Module contains return flow instrumentation and metadata
#define IMAGE_GUARD_RF_ENABLE                          0x00040000 // Module requests that the OS enable return flow protection
#define IMAGE_GUARD_RF_STRICT                          0x00080000 // Module requests that the OS enable return flow protection in strict mode
#define IMAGE_GUARD_RETPOLINE_PRESENT                  0x00100000 // Module was built with retpoline support
// DO_NOT_USE                                          0x00200000 // Was EHCont flag on VB (20H1)
#define IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT      0x00400000 // Module contains EH continuation target information
#define IMAGE_GUARD_XFG_ENABLED                        0x00800000 // Module was built with xfg (deprecated)
#define IMAGE_GUARD_CASTGUARD_PRESENT                  0x01000000 // Module has CastGuard instrumentation present
#define IMAGE_GUARD_MEMCPY_PRESENT                     0x02000000 // Module has Guarded Memcpy instrumentation present

#define IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK        0xF0000000 // Stride of Guard CF function table encoded in these bits (additional count of bytes per element)
#define IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT       28         // Shift to right-justify Guard CF function table stride

//
// GFIDS table entry flags.
//

#define IMAGE_GUARD_FLAG_FID_SUPPRESSED               0x01       // The containing GFID entry is suppressed
#define IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED            0x02       // The containing GFID entry is export suppressed
#define IMAGE_GUARD_FLAG_FID_LANGEXCPTHANDLER         0x04
#define IMAGE_GUARD_FLAG_FID_XFG                      0x08

//
// WIN CE Exception table format
//

//
// Function table entry format.  Function table is pointed to by the
// IMAGE_DIRECTORY_ENTRY_EXCEPTION directory entry.
//

typedef struct _IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    DWORD FuncStart;
    DWORD PrologLen : 8;
    DWORD FuncLen : 22;
    DWORD ThirtyTwoBit : 1;
    DWORD ExceptionFlag : 1;
} IMAGE_CE_RUNTIME_FUNCTION_ENTRY, * PIMAGE_CE_RUNTIME_FUNCTION_ENTRY;

typedef struct _IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    DWORD BeginAddress;
    union {
        DWORD UnwindData;
        struct {
            DWORD Flag : 2;
            DWORD FunctionLength : 11;
            DWORD Ret : 2;
            DWORD H : 1;
            DWORD Reg : 3;
            DWORD R : 1;
            DWORD L : 1;
            DWORD C : 1;
            DWORD StackAdjust : 10;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} IMAGE_ARM_RUNTIME_FUNCTION_ENTRY, * PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY;

typedef enum ARM64_FNPDATA_FLAGS {
    PdataRefToFullXdata = 0,
    PdataPackedUnwindFunction = 1,
    PdataPackedUnwindFragment = 2,
} ARM64_FNPDATA_FLAGS;

typedef enum ARM64_FNPDATA_CR {
    PdataCrUnchained = 0,
    PdataCrUnchainedSavedLr = 1,
    PdataCrChainedWithPac = 2,
    PdataCrChained = 3,
} ARM64_FNPDATA_CR;

typedef struct _IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    DWORD BeginAddress;
    union {
        DWORD UnwindData;
        struct {
            DWORD Flag : 2;
            DWORD FunctionLength : 11;
            DWORD RegF : 3;
            DWORD RegI : 4;
            DWORD H : 1;
            DWORD CR : 2;
            DWORD FrameSize : 9;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;
} IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY, * PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY;

typedef union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    DWORD HeaderData;
    struct {
        DWORD FunctionLength : 18;      // in words (2 bytes)
        DWORD Version : 2;
        DWORD ExceptionDataPresent : 1;
        DWORD EpilogInHeader : 1;
        DWORD EpilogCount : 5;          // number of epilogs or byte index of the first unwind code for the one only epilog
        DWORD CodeWords : 5;            // number of dwords with unwind codes
    } DUMMYSTRUCTNAME;
} IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA;

typedef union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EXTENDED {
    DWORD ExtendedHeaderData;
    struct {
        DWORD ExtendedEpilogCount : 16;
        DWORD ExtendedCodeWords : 8;
    } DUMMYSTRUCTNAME;
} IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EXTENDED;

typedef union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EPILOG_SCOPE {
    DWORD EpilogScopeData;
    struct {
        DWORD EpilogStartOffset : 18;   // offset in bytes, divided by 4, of the epilog relative to the start of the function.
        DWORD Res0: 4;
        DWORD EpilogStartIndex : 10;    // byte index of the first unwind code that describes this epilog.
    } DUMMYSTRUCTNAME;
} IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EPILOG_SCOPE;

typedef struct _IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    ULONGLONG BeginAddress;
    ULONGLONG EndAddress;
    ULONGLONG ExceptionHandler;
    ULONGLONG HandlerData;
    ULONGLONG PrologEndAddress;
} IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY, *PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY;

typedef struct _IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    DWORD BeginAddress;
    DWORD EndAddress;
    DWORD ExceptionHandler;
    DWORD HandlerData;
    DWORD PrologEndAddress;
} IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY, *PIMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY;

typedef struct _IMAGE_RUNTIME_FUNCTION_ENTRY {
    DWORD BeginAddress;
    DWORD EndAddress;
    union {
        DWORD UnwindInfoAddress;
        DWORD UnwindData;
    } DUMMYUNIONNAME;
} _IMAGE_RUNTIME_FUNCTION_ENTRY, *_PIMAGE_RUNTIME_FUNCTION_ENTRY;

typedef  _IMAGE_RUNTIME_FUNCTION_ENTRY  IMAGE_IA64_RUNTIME_FUNCTION_ENTRY;
typedef _PIMAGE_RUNTIME_FUNCTION_ENTRY PIMAGE_IA64_RUNTIME_FUNCTION_ENTRY;

typedef  _IMAGE_RUNTIME_FUNCTION_ENTRY  IMAGE_AMD64_RUNTIME_FUNCTION_ENTRY;
typedef _PIMAGE_RUNTIME_FUNCTION_ENTRY PIMAGE_AMD64_RUNTIME_FUNCTION_ENTRY;

#if defined(_AXP64_)

typedef  IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY  IMAGE_AXP64_RUNTIME_FUNCTION_ENTRY;
typedef PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY PIMAGE_AXP64_RUNTIME_FUNCTION_ENTRY;
typedef  IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY  IMAGE_RUNTIME_FUNCTION_ENTRY;
typedef PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY PIMAGE_RUNTIME_FUNCTION_ENTRY;

#elif defined(_ALPHA_)

typedef  IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY  IMAGE_RUNTIME_FUNCTION_ENTRY;
typedef PIMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY PIMAGE_RUNTIME_FUNCTION_ENTRY;

#elif defined(_ARM64_)

typedef  IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY  IMAGE_RUNTIME_FUNCTION_ENTRY;
typedef PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY PIMAGE_RUNTIME_FUNCTION_ENTRY;

#elif defined(_ARM_)

typedef  IMAGE_ARM_RUNTIME_FUNCTION_ENTRY  IMAGE_RUNTIME_FUNCTION_ENTRY;
typedef PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY PIMAGE_RUNTIME_FUNCTION_ENTRY;

#else

typedef  _IMAGE_RUNTIME_FUNCTION_ENTRY  IMAGE_RUNTIME_FUNCTION_ENTRY;
typedef _PIMAGE_RUNTIME_FUNCTION_ENTRY PIMAGE_RUNTIME_FUNCTION_ENTRY;

#endif

//
// Sofware enclave information
//

#define IMAGE_ENCLAVE_LONG_ID_LENGTH    ENCLAVE_LONG_ID_LENGTH
#define IMAGE_ENCLAVE_SHORT_ID_LENGTH   ENCLAVE_SHORT_ID_LENGTH

typedef struct _IMAGE_ENCLAVE_CONFIG32 {
    DWORD Size;
    DWORD MinimumRequiredConfigSize;
    DWORD PolicyFlags;
    DWORD NumberOfImports;
    DWORD ImportList;
    DWORD ImportEntrySize;
    BYTE  FamilyID[IMAGE_ENCLAVE_SHORT_ID_LENGTH];
    BYTE  ImageID[IMAGE_ENCLAVE_SHORT_ID_LENGTH];
    DWORD ImageVersion;
    DWORD SecurityVersion;
    DWORD EnclaveSize;
    DWORD NumberOfThreads;
    DWORD EnclaveFlags;
} IMAGE_ENCLAVE_CONFIG32, *PIMAGE_ENCLAVE_CONFIG32;

typedef struct _IMAGE_ENCLAVE_CONFIG64 {
    DWORD Size;
    DWORD MinimumRequiredConfigSize;
    DWORD PolicyFlags;
    DWORD NumberOfImports;
    DWORD ImportList;
    DWORD ImportEntrySize;
    BYTE  FamilyID[IMAGE_ENCLAVE_SHORT_ID_LENGTH];
    BYTE  ImageID[IMAGE_ENCLAVE_SHORT_ID_LENGTH];
    DWORD ImageVersion;
    DWORD SecurityVersion;
    ULONGLONG EnclaveSize;
    DWORD NumberOfThreads;
    DWORD EnclaveFlags;
} IMAGE_ENCLAVE_CONFIG64, *PIMAGE_ENCLAVE_CONFIG64;

#ifdef _WIN64
typedef IMAGE_ENCLAVE_CONFIG64          IMAGE_ENCLAVE_CONFIG;
typedef PIMAGE_ENCLAVE_CONFIG64         PIMAGE_ENCLAVE_CONFIG;
#else
typedef IMAGE_ENCLAVE_CONFIG32          IMAGE_ENCLAVE_CONFIG;
typedef PIMAGE_ENCLAVE_CONFIG32         PIMAGE_ENCLAVE_CONFIG;
#endif

#define IMAGE_ENCLAVE_MINIMUM_CONFIG_SIZE   FIELD_OFFSET(IMAGE_ENCLAVE_CONFIG, EnclaveFlags)

#define IMAGE_ENCLAVE_POLICY_DEBUGGABLE     0x00000001
#define IMAGE_ENCLAVE_POLICY_STRICT_MEMORY  0x00000002

#define IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE    0x00000001

typedef struct _IMAGE_ENCLAVE_IMPORT {
    DWORD MatchType;
    DWORD MinimumSecurityVersion;
    BYTE  UniqueOrAuthorID[IMAGE_ENCLAVE_LONG_ID_LENGTH];
    BYTE  FamilyID[IMAGE_ENCLAVE_SHORT_ID_LENGTH];
    BYTE  ImageID[IMAGE_ENCLAVE_SHORT_ID_LENGTH];
    DWORD ImportName;
    DWORD Reserved;
} IMAGE_ENCLAVE_IMPORT, *PIMAGE_ENCLAVE_IMPORT;

#define IMAGE_ENCLAVE_IMPORT_MATCH_NONE             0x00000000
#define IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID        0x00000001
#define IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID        0x00000002
#define IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID        0x00000003
#define IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID         0x00000004

//
// Debug Format
//

typedef struct _IMAGE_DEBUG_DIRECTORY {
    DWORD   Characteristics;
    DWORD   TimeDateStamp;
    WORD    MajorVersion;
    WORD    MinorVersion;
    DWORD   Type;
    DWORD   SizeOfData;
    DWORD   AddressOfRawData;
    DWORD   PointerToRawData;
} IMAGE_DEBUG_DIRECTORY, *PIMAGE_DEBUG_DIRECTORY;

#define IMAGE_DEBUG_TYPE_UNKNOWN                0
#define IMAGE_DEBUG_TYPE_COFF                   1
#define IMAGE_DEBUG_TYPE_CODEVIEW               2
#define IMAGE_DEBUG_TYPE_FPO                    3
#define IMAGE_DEBUG_TYPE_MISC                   4
#define IMAGE_DEBUG_TYPE_EXCEPTION              5
#define IMAGE_DEBUG_TYPE_FIXUP                  6
#define IMAGE_DEBUG_TYPE_OMAP_TO_SRC            7
#define IMAGE_DEBUG_TYPE_OMAP_FROM_SRC          8
#define IMAGE_DEBUG_TYPE_BORLAND                9
#define IMAGE_DEBUG_TYPE_RESERVED10             10
#define IMAGE_DEBUG_TYPE_BBT                    IMAGE_DEBUG_TYPE_RESERVED10
#define IMAGE_DEBUG_TYPE_CLSID                  11
#define IMAGE_DEBUG_TYPE_VC_FEATURE             12
#define IMAGE_DEBUG_TYPE_POGO                   13
#define IMAGE_DEBUG_TYPE_ILTCG                  14
#define IMAGE_DEBUG_TYPE_MPX                    15
#define IMAGE_DEBUG_TYPE_REPRO                  16
#define IMAGE_DEBUG_TYPE_SPGO                   18
#define IMAGE_DEBUG_TYPE_EX_DLLCHARACTERISTICS  20

#define IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT                                  0x01
#define IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT_STRICT_MODE                      0x02
#define IMAGE_DLLCHARACTERISTICS_EX_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE  0x04
#define IMAGE_DLLCHARACTERISTICS_EX_CET_DYNAMIC_APIS_ALLOW_IN_PROC              0x08
#define IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_1                              0x10  // Reserved for CET policy *downgrade* only!
#define IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_2                              0x20  // Reserved for CET policy *downgrade* only!
#define IMAGE_DLLCHARACTERISTICS_EX_FORWARD_CFI_COMPAT                          0x40
#define IMAGE_DLLCHARACTERISTICS_EX_HOTPATCH_COMPATIBLE                         0x80


typedef struct _IMAGE_COFF_SYMBOLS_HEADER {
    DWORD   NumberOfSymbols;
    DWORD   LvaToFirstSymbol;
    DWORD   NumberOfLinenumbers;
    DWORD   LvaToFirstLinenumber;
    DWORD   RvaToFirstByteOfCode;
    DWORD   RvaToLastByteOfCode;
    DWORD   RvaToFirstByteOfData;
    DWORD   RvaToLastByteOfData;
} IMAGE_COFF_SYMBOLS_HEADER, *PIMAGE_COFF_SYMBOLS_HEADER;

#define FRAME_FPO       0
#define FRAME_TRAP      1
#define FRAME_TSS       2
#define FRAME_NONFPO    3

typedef struct _FPO_DATA {
    DWORD       ulOffStart;             // offset 1st byte of function code
    DWORD       cbProcSize;             // # bytes in function
    DWORD       cdwLocals;              // # bytes in locals/4
    WORD        cdwParams;              // # bytes in params/4
    WORD        cbProlog : 8;           // # bytes in prolog
    WORD        cbRegs   : 3;           // # regs saved
    WORD        fHasSEH  : 1;           // TRUE if SEH in func
    WORD        fUseBP   : 1;           // TRUE if EBP has been allocated
    WORD        reserved : 1;           // reserved for future use
    WORD        cbFrame  : 2;           // frame type
} FPO_DATA, *PFPO_DATA;
#define SIZEOF_RFPO_DATA 16


#define IMAGE_DEBUG_MISC_EXENAME    1

typedef struct _IMAGE_DEBUG_MISC {
    DWORD       DataType;               // type of misc data, see defines
    DWORD       Length;                 // total length of record, rounded to four
                                        // byte multiple.
    BOOLEAN     Unicode;                // TRUE if data is unicode string
    BYTE        Reserved[ 3 ];
    BYTE        Data[ 1 ];              // Actual data
} IMAGE_DEBUG_MISC, *PIMAGE_DEBUG_MISC;


//
// Function table extracted from MIPS/ALPHA/IA64 images.  Does not contain
// information needed only for runtime support.  Just those fields for
// each entry needed by a debugger.
//

typedef struct _IMAGE_FUNCTION_ENTRY {
    DWORD   StartingAddress;
    DWORD   EndingAddress;
    DWORD   EndOfPrologue;
} IMAGE_FUNCTION_ENTRY, *PIMAGE_FUNCTION_ENTRY;

typedef struct _IMAGE_FUNCTION_ENTRY64 {
    ULONGLONG   StartingAddress;
    ULONGLONG   EndingAddress;
    union {
        ULONGLONG   EndOfPrologue;
        ULONGLONG   UnwindInfoAddress;
    } DUMMYUNIONNAME;
} IMAGE_FUNCTION_ENTRY64, *PIMAGE_FUNCTION_ENTRY64;

//
// Debugging information can be stripped from an image file and placed
// in a separate .DBG file, whose file name part is the same as the
// image file name part (e.g. symbols for CMD.EXE could be stripped
// and placed in CMD.DBG).  This is indicated by the IMAGE_FILE_DEBUG_STRIPPED
// flag in the Characteristics field of the file header.  The beginning of
// the .DBG file contains the following structure which captures certain
// information from the image file.  This allows a debug to proceed even if
// the original image file is not accessable.  This header is followed by
// zero of more IMAGE_SECTION_HEADER structures, followed by zero or more
// IMAGE_DEBUG_DIRECTORY structures.  The latter structures and those in
// the image file contain file offsets relative to the beginning of the
// .DBG file.
//
// If symbols have been stripped from an image, the IMAGE_DEBUG_MISC structure
// is left in the image file, but not mapped.  This allows a debugger to
// compute the name of the .DBG file, from the name of the image in the
// IMAGE_DEBUG_MISC structure.
//

typedef struct _IMAGE_SEPARATE_DEBUG_HEADER {
    WORD        Signature;
    WORD        Flags;
    WORD        Machine;
    WORD        Characteristics;
    DWORD       TimeDateStamp;
    DWORD       CheckSum;
    DWORD       ImageBase;
    DWORD       SizeOfImage;
    DWORD       NumberOfSections;
    DWORD       ExportedNamesSize;
    DWORD       DebugDirectorySize;
    DWORD       SectionAlignment;
    DWORD       Reserved[2];
} IMAGE_SEPARATE_DEBUG_HEADER, *PIMAGE_SEPARATE_DEBUG_HEADER;

// begin_ntoshvp

typedef struct _NON_PAGED_DEBUG_INFO {
    WORD        Signature;
    WORD        Flags;
    DWORD       Size;
    WORD        Machine;
    WORD        Characteristics;
    DWORD       TimeDateStamp;
    DWORD       CheckSum;
    DWORD       SizeOfImage;
    ULONGLONG   ImageBase;
    //DebugDirectorySize
    //IMAGE_DEBUG_DIRECTORY
} NON_PAGED_DEBUG_INFO, *PNON_PAGED_DEBUG_INFO;

// end_ntoshvp

#ifndef _MAC
#define IMAGE_SEPARATE_DEBUG_SIGNATURE 0x4944
#define NON_PAGED_DEBUG_SIGNATURE      0x494E
#else
#define IMAGE_SEPARATE_DEBUG_SIGNATURE 0x4449  // DI
#define NON_PAGED_DEBUG_SIGNATURE      0x4E49  // NI
#endif

#define IMAGE_SEPARATE_DEBUG_FLAGS_MASK 0x8000
#define IMAGE_SEPARATE_DEBUG_MISMATCH   0x8000  // when DBG was updated, the
                                                // old checksum didn't match.

//
//  The .arch section is made up of headers, each describing an amask position/value
//  pointing to an array of IMAGE_ARCHITECTURE_ENTRY's.  Each "array" (both the header
//  and entry arrays) are terminiated by a quadword of 0xffffffffL.
//
//  NOTE: There may be quadwords of 0 sprinkled around and must be skipped.
//

typedef struct _ImageArchitectureHeader {
    unsigned int AmaskValue: 1;                 // 1 -> code section depends on mask bit
                                                // 0 -> new instruction depends on mask bit
    int :7;                                     // MBZ
    unsigned int AmaskShift: 8;                 // Amask bit in question for this fixup
    int :16;                                    // MBZ
    DWORD FirstEntryRVA;                        // RVA into .arch section to array of ARCHITECTURE_ENTRY's
} IMAGE_ARCHITECTURE_HEADER, *PIMAGE_ARCHITECTURE_HEADER;

typedef struct _ImageArchitectureEntry {
    DWORD FixupInstRVA;                         // RVA of instruction to fixup
    DWORD NewInst;                              // fixup instruction (see alphaops.h)
} IMAGE_ARCHITECTURE_ENTRY, *PIMAGE_ARCHITECTURE_ENTRY;

#include "poppack.h"                // Back to the initial value

// The following structure defines the new import object.  Note the values of the first two fields,
// which must be set as stated in order to differentiate old and new import members.
// Following this structure, the linker emits two null-terminated strings used to recreate the
// import at the time of use.  The first string is the import's name, the second is the dll's name.

#define IMPORT_OBJECT_HDR_SIG2  0xffff

typedef struct IMPORT_OBJECT_HEADER {
    WORD    Sig1;                       // Must be IMAGE_FILE_MACHINE_UNKNOWN
    WORD    Sig2;                       // Must be IMPORT_OBJECT_HDR_SIG2.
    WORD    Version;
    WORD    Machine;
    DWORD   TimeDateStamp;              // Time/date stamp
    DWORD   SizeOfData;                 // particularly useful for incremental links

    union {
        WORD    Ordinal;                // if grf & IMPORT_OBJECT_ORDINAL
        WORD    Hint;
    } DUMMYUNIONNAME;

    WORD    Type : 2;                   // IMPORT_TYPE
    WORD    NameType : 3;               // IMPORT_NAME_TYPE
    WORD    Reserved : 11;              // Reserved. Must be zero.
} IMPORT_OBJECT_HEADER;

typedef enum IMPORT_OBJECT_TYPE
{
    IMPORT_OBJECT_CODE = 0,
    IMPORT_OBJECT_DATA = 1,
    IMPORT_OBJECT_CONST = 2,
} IMPORT_OBJECT_TYPE;

typedef enum IMPORT_OBJECT_NAME_TYPE
{
    IMPORT_OBJECT_ORDINAL = 0,          // Import by ordinal
    IMPORT_OBJECT_NAME = 1,             // Import name == public symbol name.
    IMPORT_OBJECT_NAME_NO_PREFIX = 2,   // Import name == public symbol name skipping leading ?, @, or optionally _.
    IMPORT_OBJECT_NAME_UNDECORATE = 3,  // Import name == public symbol name skipping leading ?, @, or optionally _
                                        //  and truncating at first @.
    IMPORT_OBJECT_NAME_EXPORTAS = 4,    // Import name == a name is explicitly provided after the DLL name.
} IMPORT_OBJECT_NAME_TYPE;


#ifndef __IMAGE_COR20_HEADER_DEFINED__
#define __IMAGE_COR20_HEADER_DEFINED__

typedef enum ReplacesCorHdrNumericDefines
{
// COM+ Header entry point flags.
    COMIMAGE_FLAGS_ILONLY               =0x00000001,
    COMIMAGE_FLAGS_32BITREQUIRED        =0x00000002,
    COMIMAGE_FLAGS_IL_LIBRARY           =0x00000004,
    COMIMAGE_FLAGS_STRONGNAMESIGNED     =0x00000008,
    COMIMAGE_FLAGS_NATIVE_ENTRYPOINT    =0x00000010,
    COMIMAGE_FLAGS_TRACKDEBUGDATA       =0x00010000,
    COMIMAGE_FLAGS_32BITPREFERRED       =0x00020000,

// Version flags for image.
    COR_VERSION_MAJOR_V2                =2,
    COR_VERSION_MAJOR                   =COR_VERSION_MAJOR_V2,
    COR_VERSION_MINOR                   =5,
    COR_DELETED_NAME_LENGTH             =8,
    COR_VTABLEGAP_NAME_LENGTH           =8,

// Maximum size of a NativeType descriptor.
    NATIVE_TYPE_MAX_CB                  =1,
    COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE=0xFF,

// #defines for the MIH FLAGS
    IMAGE_COR_MIH_METHODRVA             =0x01,
    IMAGE_COR_MIH_EHRVA                 =0x02,
    IMAGE_COR_MIH_BASICBLOCK            =0x08,

// V-table constants
    COR_VTABLE_32BIT                    =0x01,          // V-table slots are 32-bits in size.
    COR_VTABLE_64BIT                    =0x02,          // V-table slots are 64-bits in size.
    COR_VTABLE_FROM_UNMANAGED           =0x04,          // If set, transition from unmanaged.
    COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN  =0x08,  // If set, transition from unmanaged with keeping the current appdomain.
    COR_VTABLE_CALL_MOST_DERIVED        =0x10,          // Call most derived method described by

// EATJ constants
    IMAGE_COR_EATJ_THUNK_SIZE           =32,            // Size of a jump thunk reserved range.

// Max name lengths
    //@todo: Change to unlimited name lengths.
    MAX_CLASS_NAME                      =1024,
    MAX_PACKAGE_NAME                    =1024,
} ReplacesCorHdrNumericDefines;

// CLR 2.0 header structure.
typedef struct IMAGE_COR20_HEADER
{
    // Header versioning
    DWORD                   cb;
    WORD                    MajorRuntimeVersion;
    WORD                    MinorRuntimeVersion;

    // Symbol table and startup information
    IMAGE_DATA_DIRECTORY    MetaData;
    DWORD                   Flags;

    // If COMIMAGE_FLAGS_NATIVE_ENTRYPOINT is not set, EntryPointToken represents a managed entrypoint.
    // If COMIMAGE_FLAGS_NATIVE_ENTRYPOINT is set, EntryPointRVA represents an RVA to a native entrypoint.
    union {
        DWORD               EntryPointToken;
        DWORD               EntryPointRVA;
    } DUMMYUNIONNAME;

    // Binding information
    IMAGE_DATA_DIRECTORY    Resources;
    IMAGE_DATA_DIRECTORY    StrongNameSignature;

    // Regular fixup and binding information
    IMAGE_DATA_DIRECTORY    CodeManagerTable;
    IMAGE_DATA_DIRECTORY    VTableFixups;
    IMAGE_DATA_DIRECTORY    ExportAddressTableJumps;

    // Precompiled image info (internal use only - set to zero)
    IMAGE_DATA_DIRECTORY    ManagedNativeHeader;

} IMAGE_COR20_HEADER, *PIMAGE_COR20_HEADER;

#endif // __IMAGE_COR20_HEADER_DEFINED__

//
// End Image Format
//

#ifndef _APISETRTLSUPPORT_
#define _APISETRTLSUPPORT_
#include <apiset.h>

//
// prototypes
//

// begin_ntifs

#pragma region Application or OneCore or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION > NTDDI_WINXP)

NTSYSAPI
_Success_(return != 0)
WORD  
NTAPI
RtlCaptureStackBackTrace(
    _In_ DWORD FramesToSkip,
    _In_ DWORD FramesToCapture,
    _Out_writes_to_(FramesToCapture,return) PVOID* BackTrace,
    _Out_opt_ PDWORD BackTraceHash
    );

#endif

#if (NTDDI_VERSION > NTDDI_WIN2K)

NTSYSAPI
VOID
NTAPI
RtlCaptureContext(
    _Out_ PCONTEXT ContextRecord
    );

#endif // (NTDDI_VERSION > NTDDI_WIN2K)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

#if defined(_AMD64_)

NTSYSAPI
VOID
NTAPI
RtlCaptureContext2(
    _Inout_ PCONTEXT ContextRecord
    );

#endif // defined(_AMD64_)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

// end_ntifs

#if defined (_AMD64_) || defined(_ARM_) || defined(_ARM64_)

//
// Define unwind history table structure.
//

#define UNWIND_HISTORY_TABLE_SIZE 12

typedef struct _UNWIND_HISTORY_TABLE_ENTRY {
    ULONG_PTR ImageBase;
    PRUNTIME_FUNCTION FunctionEntry;
} UNWIND_HISTORY_TABLE_ENTRY, *PUNWIND_HISTORY_TABLE_ENTRY;

typedef struct _UNWIND_HISTORY_TABLE {
    DWORD Count;
    BYTE  LocalHint;
    BYTE  GlobalHint;
    BYTE  Search;
    BYTE  Once;
    ULONG_PTR LowAddress;
    ULONG_PTR HighAddress;
    UNWIND_HISTORY_TABLE_ENTRY Entry[UNWIND_HISTORY_TABLE_SIZE];
} UNWIND_HISTORY_TABLE, *PUNWIND_HISTORY_TABLE;

#endif

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

NTSYSAPI
VOID
NTAPI
RtlUnwind(
    _In_opt_ PVOID TargetFrame,
    _In_opt_ PVOID TargetIp,
    _In_opt_ PEXCEPTION_RECORD ExceptionRecord,
    _In_ PVOID ReturnValue
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if defined(_AMD64_)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
BOOLEAN
__cdecl
RtlAddFunctionTable(
    _In_reads_(EntryCount) PRUNTIME_FUNCTION FunctionTable,
    _In_ DWORD EntryCount,
    _In_ DWORD64 BaseAddress
    );

NTSYSAPI
BOOLEAN
__cdecl
RtlDeleteFunctionTable(
    _In_ PRUNTIME_FUNCTION FunctionTable
    );

NTSYSAPI
BOOLEAN
__cdecl
RtlInstallFunctionTableCallback(
    _In_ DWORD64 TableIdentifier,
    _In_ DWORD64 BaseAddress,
    _In_ DWORD Length,
    _In_ PGET_RUNTIME_FUNCTION_CALLBACK Callback,
    _In_opt_ PVOID Context,
    _In_opt_ PCWSTR OutOfProcessCallbackDll
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN8)

NTSYSAPI
DWORD   
NTAPI
RtlAddGrowableFunctionTable(
    _Out_ PVOID* DynamicTable,
    _In_reads_(MaximumEntryCount) PRUNTIME_FUNCTION FunctionTable,
    _In_ DWORD EntryCount,
    _In_ DWORD MaximumEntryCount,
    _In_ ULONG_PTR RangeBase,
    _In_ ULONG_PTR RangeEnd
    );

NTSYSAPI
VOID
NTAPI
RtlGrowFunctionTable(
    _Inout_ PVOID DynamicTable,
    _In_ DWORD NewEntryCount
    );

NTSYSAPI
VOID
NTAPI
RtlDeleteGrowableFunctionTable(
    _In_ PVOID DynamicTable
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

NTSYSAPI
PRUNTIME_FUNCTION
NTAPI
RtlLookupFunctionEntry(
    _In_ DWORD64 ControlPc,
    _Out_ PDWORD64 ImageBase,
    _Inout_opt_ PUNWIND_HISTORY_TABLE HistoryTable
    );

NTSYSAPI
VOID
__cdecl
RtlRestoreContext(
    _In_ PCONTEXT ContextRecord,
    _In_opt_ struct _EXCEPTION_RECORD* ExceptionRecord
    );

NTSYSAPI
VOID
NTAPI
RtlUnwindEx(
    _In_opt_ PVOID TargetFrame,
    _In_opt_ PVOID TargetIp,
    _In_opt_ PEXCEPTION_RECORD ExceptionRecord,
    _In_ PVOID ReturnValue,
    _In_ PCONTEXT ContextRecord,
    _In_opt_ PUNWIND_HISTORY_TABLE HistoryTable
    );

NTSYSAPI
PEXCEPTION_ROUTINE
NTAPI
RtlVirtualUnwind(
    _In_ DWORD HandlerType,
    _In_ DWORD64 ImageBase,
    _In_ DWORD64 ControlPc,
    _In_ PRUNTIME_FUNCTION FunctionEntry,
    _Inout_ PCONTEXT ContextRecord,
    _Out_ PVOID* HandlerData,
    _Out_ PDWORD64 EstablisherFrame,
    _Inout_opt_ PKNONVOLATILE_CONTEXT_POINTERS ContextPointers
    );


#if defined(_M_ARM64EC)

NTSYSAPI
BOOLEAN
NTAPI
RtlIsEcCode(
    _In_ DWORD64 CodePointer
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // defined(_AMD64_)


#if defined(_ARM_)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
BOOLEAN
__cdecl
RtlAddFunctionTable(
    _In_reads_(EntryCount) PRUNTIME_FUNCTION FunctionTable,
    _In_ DWORD EntryCount,
    _In_ DWORD BaseAddress
    );

NTSYSAPI
BOOLEAN
__cdecl
RtlDeleteFunctionTable(
    _In_ PRUNTIME_FUNCTION FunctionTable
    );

NTSYSAPI
BOOLEAN
__cdecl
RtlInstallFunctionTableCallback(
    _In_ DWORD TableIdentifier,
    _In_ DWORD BaseAddress,
    _In_ DWORD Length,
    _In_ PGET_RUNTIME_FUNCTION_CALLBACK Callback,
    _In_opt_ PVOID Context,
    _In_opt_ PCWSTR OutOfProcessCallbackDll
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN8)

NTSYSAPI
DWORD   
NTAPI
RtlAddGrowableFunctionTable(
    _Out_ PVOID* DynamicTable,
    _In_reads_(MaximumEntryCount) PRUNTIME_FUNCTION FunctionTable,
    _In_ DWORD EntryCount,
    _In_ DWORD MaximumEntryCount,
    _In_ ULONG_PTR RangeBase,
    _In_ ULONG_PTR RangeEnd
    );

NTSYSAPI
VOID
NTAPI
RtlGrowFunctionTable(
    _Inout_ PVOID DynamicTable,
    _In_ DWORD NewEntryCount
    );

NTSYSAPI
VOID
NTAPI
RtlDeleteGrowableFunctionTable(
    _In_ PVOID DynamicTable
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

NTSYSAPI
PRUNTIME_FUNCTION
NTAPI
RtlLookupFunctionEntry(
    _In_ ULONG_PTR ControlPc,
    _Out_ PDWORD ImageBase,
    _Inout_opt_ PUNWIND_HISTORY_TABLE HistoryTable
    );

NTSYSAPI
VOID
__cdecl
RtlRestoreContext(
    _In_ PCONTEXT ContextRecord,
    _In_opt_ struct _EXCEPTION_RECORD* ExceptionRecord
    );

NTSYSAPI
VOID
NTAPI
RtlUnwindEx(
    _In_opt_ PVOID TargetFrame,
    _In_opt_ PVOID TargetIp,
    _In_opt_ PEXCEPTION_RECORD ExceptionRecord,
    _In_ PVOID ReturnValue,
    _In_ PCONTEXT ContextRecord,
    _In_opt_ PUNWIND_HISTORY_TABLE HistoryTable
    );

NTSYSAPI
PEXCEPTION_ROUTINE
NTAPI
RtlVirtualUnwind(
    _In_ DWORD HandlerType,
    _In_ DWORD ImageBase,
    _In_ DWORD ControlPc,
    _In_ PRUNTIME_FUNCTION FunctionEntry,
    _Inout_ PCONTEXT ContextRecord,
    _Out_ PVOID* HandlerData,
    _Out_ PDWORD EstablisherFrame,
    _Inout_opt_ PKNONVOLATILE_CONTEXT_POINTERS ContextPointers
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // defined(_ARM_)


#if defined(_ARM64_)

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
BOOLEAN
__cdecl
RtlAddFunctionTable(
    _In_reads_(EntryCount) PRUNTIME_FUNCTION FunctionTable,
    _In_ DWORD EntryCount,
    _In_ ULONG_PTR BaseAddress
    );

NTSYSAPI
BOOLEAN
__cdecl
RtlDeleteFunctionTable(
    _In_ PRUNTIME_FUNCTION FunctionTable
    );

NTSYSAPI
BOOLEAN
__cdecl
RtlInstallFunctionTableCallback(
    _In_ ULONG_PTR TableIdentifier,
    _In_ ULONG_PTR BaseAddress,
    _In_ DWORD Length,
    _In_ PGET_RUNTIME_FUNCTION_CALLBACK Callback,
    _In_opt_ PVOID Context,
    _In_opt_ PCWSTR OutOfProcessCallbackDll
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN8)

NTSYSAPI
DWORD   
NTAPI
RtlAddGrowableFunctionTable(
    _Out_ PVOID* DynamicTable,
    _In_reads_(MaximumEntryCount) PRUNTIME_FUNCTION FunctionTable,
    _In_ DWORD EntryCount,
    _In_ DWORD MaximumEntryCount,
    _In_ ULONG_PTR RangeBase,
    _In_ ULONG_PTR RangeEnd
    );

NTSYSAPI
VOID
NTAPI
RtlGrowFunctionTable(
    _Inout_ PVOID DynamicTable,
    _In_ DWORD NewEntryCount
    );

NTSYSAPI
VOID
NTAPI
RtlDeleteGrowableFunctionTable(
    _In_ PVOID DynamicTable
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

NTSYSAPI
PRUNTIME_FUNCTION
NTAPI
RtlLookupFunctionEntry(
    _In_ ULONG_PTR ControlPc,
    _Out_ PULONG_PTR ImageBase,
    _Inout_opt_ PUNWIND_HISTORY_TABLE HistoryTable
    );

NTSYSAPI
VOID
__cdecl
RtlRestoreContext(
    _In_ PCONTEXT ContextRecord,
    _In_opt_ struct _EXCEPTION_RECORD* ExceptionRecord
    );

NTSYSAPI
VOID
NTAPI
RtlUnwindEx(
    _In_opt_ PVOID TargetFrame,
    _In_opt_ PVOID TargetIp,
    _In_opt_ PEXCEPTION_RECORD ExceptionRecord,
    _In_ PVOID ReturnValue,
    _In_ PCONTEXT ContextRecord,
    _In_opt_ PUNWIND_HISTORY_TABLE HistoryTable
    );

NTSYSAPI
PEXCEPTION_ROUTINE
NTAPI
RtlVirtualUnwind(
    _In_ DWORD HandlerType,
    _In_ ULONG_PTR ImageBase,
    _In_ ULONG_PTR ControlPc,
    _In_ PRUNTIME_FUNCTION FunctionEntry,
    _Inout_ PCONTEXT ContextRecord,
    _Out_ PVOID* HandlerData,
    _Out_ PULONG_PTR EstablisherFrame,
    _Inout_opt_ PKNONVOLATILE_CONTEXT_POINTERS ContextPointers
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // defined(_ARM64_)


#if defined(_X86_)

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

NTSYSAPI
VOID
__cdecl
RtlRestoreContext(
    _In_ PCONTEXT ContextRecord,
    _In_opt_ struct _EXCEPTION_RECORD* ExceptionRecord
    );

#endif /* NTDDI_VERSION >= NTDDI_WIN10_FE */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // defined(_X86_)


#if defined(_CHPE_X86_ARM64_)

#pragma region Application or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
VOID
NTAPI
RtlUnwindEx(
    _In_opt_ PVOID TargetFrame,
    _In_opt_ PVOID TargetIp,
    _In_opt_ PEXCEPTION_RECORD ExceptionRecord,
    _In_ PVOID ReturnValue,
    _In_ PCONTEXT ContextRecord,
    _In_opt_ PVOID HistoryTable
    );

NTSYSAPI
PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY
NTAPI
RtlLookupFunctionEntryCHPE(
    _In_ ULONG_PTR ControlPc,
    _Out_ PULONG_PTR ImageBase,
    _Inout_opt_ PVOID HistoryTable
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // defined(_CHPE_X86_ARM64_)

#pragma region Application Family or OneCore Family or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES)

NTSYSAPI
VOID
NTAPI
RtlRaiseException(
    _In_ PEXCEPTION_RECORD ExceptionRecord
    );

NTSYSAPI
PVOID
NTAPI
RtlPcToFileHeader(
    _In_ PVOID PcValue,
    _Out_ PVOID* BaseOfImage
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM | WINAPI_PARTITION_GAMES) */
#pragma endregion


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

#if (NTDDI_VERSION >= NTDDI_WIN2K)

_Check_return_
NTSYSAPI
SIZE_T
NTAPI
RtlCompareMemory(
    _In_ const VOID* Source1,
    _In_ const VOID* Source2,
    _In_ SIZE_T Length
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif // _APISETRTLSUPPORT_
//
// for move macros
//
#ifdef _MAC
#ifndef _INC_STRING
#include <string.h>
#endif /* _INC_STRING */
#else
#include <string.h>
#endif // _MAC


#ifndef _SLIST_HEADER_
#define _SLIST_HEADER_

#if defined(_WIN64)

//
// The type SINGLE_LIST_ENTRY is not suitable for use with SLISTs.  For
// WIN64, an entry on an SLIST is required to be 16-byte aligned, while a
// SINGLE_LIST_ENTRY structure has only 8 byte alignment.
//
// Therefore, all SLIST code should use the SLIST_ENTRY type instead of the
// SINGLE_LIST_ENTRY type.
//

#pragma warning(push)
#pragma warning(disable:4324)   // structure padded due to align()

typedef struct DECLSPEC_ALIGN(16) _SLIST_ENTRY {
    struct _SLIST_ENTRY *Next;
} SLIST_ENTRY, *PSLIST_ENTRY;

#pragma warning(pop)

#else

typedef struct _SINGLE_LIST_ENTRY SLIST_ENTRY, *PSLIST_ENTRY;

#endif // _WIN64

#if defined(_AMD64_)

typedef union DECLSPEC_ALIGN(16) _SLIST_HEADER {
    struct {  // original struct
        ULONGLONG Alignment;
        ULONGLONG Region;
    } DUMMYSTRUCTNAME;
    struct {  // x64 16-byte header
        ULONGLONG Depth:16;
        ULONGLONG Sequence:48;
        ULONGLONG Reserved:4;
        ULONGLONG NextEntry:60; // last 4 bits are always 0's
    } HeaderX64;
} SLIST_HEADER, *PSLIST_HEADER;

#elif defined(_ARM64_)

// ARM64_WORKITEM: should this be merged with AMD64 above?
typedef union DECLSPEC_ALIGN(16) _SLIST_HEADER {
    struct {  // original struct
        ULONGLONG Alignment;
        ULONGLONG Region;
    } DUMMYSTRUCTNAME;
    struct {  // ARM64 16-byte header
        ULONGLONG Depth:16;
        ULONGLONG Sequence:48;
        ULONGLONG Reserved:4;
        ULONGLONG NextEntry:60; // last 4 bits are always 0's
    } HeaderArm64;
} SLIST_HEADER, *PSLIST_HEADER;

#elif defined(_X86_)

typedef union _SLIST_HEADER {
    ULONGLONG Alignment;
    struct {
        SLIST_ENTRY Next;
        WORD   Depth;
        WORD   CpuId;
    } DUMMYSTRUCTNAME;
} SLIST_HEADER, *PSLIST_HEADER;

#elif defined(_ARM_)

typedef union _SLIST_HEADER {
    ULONGLONG Alignment;
    struct {
        SLIST_ENTRY Next;
        WORD   Depth;
        WORD   Reserved;
    } DUMMYSTRUCTNAME;
} SLIST_HEADER, *PSLIST_HEADER;

#endif

#endif // _SLIST_HEADER_


#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

NTSYSAPI
VOID
NTAPI
RtlInitializeSListHead (
    _Out_ PSLIST_HEADER ListHead
    );

_Must_inspect_result_
NTSYSAPI
PSLIST_ENTRY
NTAPI
RtlFirstEntrySList (
    _In_ const SLIST_HEADER *ListHead
    );

NTSYSAPI
PSLIST_ENTRY
NTAPI
RtlInterlockedPopEntrySList (
    _Inout_ PSLIST_HEADER ListHead
    );

NTSYSAPI
PSLIST_ENTRY
NTAPI
RtlInterlockedPushEntrySList (
    _Inout_ PSLIST_HEADER ListHead,
    _Inout_ __drv_aliasesMem PSLIST_ENTRY ListEntry
    );

NTSYSAPI
PSLIST_ENTRY
NTAPI
RtlInterlockedPushListSListEx (
    _Inout_ PSLIST_HEADER ListHead,
    _Inout_ __drv_aliasesMem PSLIST_ENTRY List,
    _Inout_ PSLIST_ENTRY ListEnd,
    _In_ DWORD Count
    );

NTSYSAPI
PSLIST_ENTRY
NTAPI
RtlInterlockedFlushSList (
    _Inout_ PSLIST_HEADER ListHead
    );

NTSYSAPI
WORD  
NTAPI
RtlQueryDepthSList (
    _In_ PSLIST_HEADER ListHead
    );

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

NTSYSAPI
ULONG_PTR
NTAPI
RtlGetReturnAddressHijackTarget (
    VOID
    );

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


#ifndef _RTL_RUN_ONCE_DEF
#define _RTL_RUN_ONCE_DEF

//
// Run once
//

#define RTL_RUN_ONCE_INIT {0}   // Static initializer

//
// Run once flags
//

#define RTL_RUN_ONCE_CHECK_ONLY     0x00000001UL
#define RTL_RUN_ONCE_ASYNC          0x00000002UL
#define RTL_RUN_ONCE_INIT_FAILED    0x00000004UL

//
// The context stored in the run once structure must leave the following number
// of low order bits unused.
//

#define RTL_RUN_ONCE_CTX_RESERVED_BITS 2

typedef union _RTL_RUN_ONCE {       
    PVOID Ptr;                      
} RTL_RUN_ONCE, *PRTL_RUN_ONCE;     

#endif // _RTL_RUN_ONCE_DEF

typedef struct _RTL_BARRIER {                       
            DWORD Reserved1;                        
            DWORD Reserved2;                        
            ULONG_PTR Reserved3[2];                 
            DWORD Reserved4;                        
            DWORD Reserved5;                        
} RTL_BARRIER, *PRTL_BARRIER;                       
// begin_ntoshvp

// Include the more obscure SAL annotations (like __drv_aliasesMem) instead of assuming the crtdefs.h will include them.
#include <specstrings.h>

//
// Fast Fail (Fail Fast) Failure Codes (Parameter 0)
//
// N.B.  Failure Codes should never be changed after the initial definition
// N.B.  Failure Code zero (0) should not be used
//       It is reserved for compatibility with previous handling of the STATUS_STACK_BUFFER_OVERRUN exception status code
//

// When adding failure codes, please also update:
// - Debugger (onecore\sdktools\debuggers\ntsd64\util.cpp)
// - !analyze (onecore\sdktools\debuggers\exts\badev\extdll\uacommon.h & uexcep.cpp)

#define FAST_FAIL_LEGACY_GS_VIOLATION               0
#define FAST_FAIL_VTGUARD_CHECK_FAILURE             1
#define FAST_FAIL_STACK_COOKIE_CHECK_FAILURE        2
#define FAST_FAIL_CORRUPT_LIST_ENTRY                3
#define FAST_FAIL_INCORRECT_STACK                   4
#define FAST_FAIL_INVALID_ARG                       5
#define FAST_FAIL_GS_COOKIE_INIT                    6
#define FAST_FAIL_FATAL_APP_EXIT                    7
#define FAST_FAIL_RANGE_CHECK_FAILURE               8
#define FAST_FAIL_UNSAFE_REGISTRY_ACCESS            9
#define FAST_FAIL_GUARD_ICALL_CHECK_FAILURE         10
#define FAST_FAIL_GUARD_WRITE_CHECK_FAILURE         11
#define FAST_FAIL_INVALID_FIBER_SWITCH              12
#define FAST_FAIL_INVALID_SET_OF_CONTEXT            13
#define FAST_FAIL_INVALID_REFERENCE_COUNT           14
#define FAST_FAIL_INVALID_JUMP_BUFFER               18
#define FAST_FAIL_MRDATA_MODIFIED                   19
#define FAST_FAIL_CERTIFICATION_FAILURE             20
#define FAST_FAIL_INVALID_EXCEPTION_CHAIN           21
#define FAST_FAIL_CRYPTO_LIBRARY                    22
#define FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT       23
#define FAST_FAIL_INVALID_IMAGE_BASE                24
#define FAST_FAIL_DLOAD_PROTECTION_FAILURE          25
#define FAST_FAIL_UNSAFE_EXTENSION_CALL             26
#define FAST_FAIL_DEPRECATED_SERVICE_INVOKED        27
#define FAST_FAIL_INVALID_BUFFER_ACCESS             28
#define FAST_FAIL_INVALID_BALANCED_TREE             29
#define FAST_FAIL_INVALID_NEXT_THREAD               30
#define FAST_FAIL_GUARD_ICALL_CHECK_SUPPRESSED      31         // Telemetry, nonfatal
#define FAST_FAIL_APCS_DISABLED                     32
#define FAST_FAIL_INVALID_IDLE_STATE                33
#define FAST_FAIL_MRDATA_PROTECTION_FAILURE         34
#define FAST_FAIL_UNEXPECTED_HEAP_EXCEPTION         35
#define FAST_FAIL_INVALID_LOCK_STATE                36
#define FAST_FAIL_GUARD_JUMPTABLE                   37         // Known to compiler, must retain value 37
#define FAST_FAIL_INVALID_LONGJUMP_TARGET           38
#define FAST_FAIL_INVALID_DISPATCH_CONTEXT          39
#define FAST_FAIL_INVALID_THREAD                    40
#define FAST_FAIL_INVALID_SYSCALL_NUMBER            41         // Telemetry, nonfatal
#define FAST_FAIL_INVALID_FILE_OPERATION            42         // Telemetry, nonfatal
#define FAST_FAIL_LPAC_ACCESS_DENIED                43         // Telemetry, nonfatal
#define FAST_FAIL_GUARD_SS_FAILURE                  44
#define FAST_FAIL_LOADER_CONTINUITY_FAILURE         45         // Telemetry, nonfatal
#define FAST_FAIL_GUARD_EXPORT_SUPPRESSION_FAILURE  46
#define FAST_FAIL_INVALID_CONTROL_STACK             47
#define FAST_FAIL_SET_CONTEXT_DENIED                48
#define FAST_FAIL_INVALID_IAT                       49
#define FAST_FAIL_HEAP_METADATA_CORRUPTION          50
#define FAST_FAIL_PAYLOAD_RESTRICTION_VIOLATION     51
#define FAST_FAIL_LOW_LABEL_ACCESS_DENIED           52         // Telemetry, nonfatal
#define FAST_FAIL_ENCLAVE_CALL_FAILURE              53
#define FAST_FAIL_UNHANDLED_LSS_EXCEPTON            54
#define FAST_FAIL_ADMINLESS_ACCESS_DENIED           55         // Telemetry, nonfatal
#define FAST_FAIL_UNEXPECTED_CALL                   56
#define FAST_FAIL_CONTROL_INVALID_RETURN_ADDRESS    57
#define FAST_FAIL_UNEXPECTED_HOST_BEHAVIOR          58
#define FAST_FAIL_FLAGS_CORRUPTION                  59
#define FAST_FAIL_VEH_CORRUPTION                    60
#define FAST_FAIL_ETW_CORRUPTION                    61
#define FAST_FAIL_RIO_ABORT                         62
#define FAST_FAIL_INVALID_PFN                       63
#define FAST_FAIL_GUARD_ICALL_CHECK_FAILURE_XFG     64
#define FAST_FAIL_CAST_GUARD                        65         // Known to compiler, must retain value 65
#define FAST_FAIL_HOST_VISIBILITY_CHANGE            66
#define FAST_FAIL_KERNEL_CET_SHADOW_STACK_ASSIST    67
#define FAST_FAIL_PATCH_CALLBACK_FAILED             68
#define FAST_FAIL_NTDLL_PATCH_FAILED                69
#define FAST_FAIL_INVALID_FLS_DATA                  70
#define FAST_FAIL_ASAN_ERROR                        71         // Known to Asan, must retain value 71
#define FAST_FAIL_CLR_EXCEPTION_AOT                 72
#define FAST_FAIL_POINTER_AUTH_INVALID_RETURN_ADDRESS 73
#define FAST_FAIL_INVALID_THREAD_STATE              74
#define FAST_FAIL_CORRUPT_WOW64_STATE               75
#define FAST_FAIL_INVALID_EXTENDED_STATE            76
#define FAST_FAIL_KERNEL_POINTER_EXPECTED           77
#define FAST_FAIL_INVALID_FAST_FAIL_CODE            0xFFFFFFFF

#if _MSC_VER >= 1610

DECLSPEC_NORETURN
VOID
__fastfail(
    _In_ unsigned int Code
    );

#pragma intrinsic(__fastfail)

#endif

#define HEAP_NO_SERIALIZE               0x00000001      
#define HEAP_GROWABLE                   0x00000002      
#define HEAP_GENERATE_EXCEPTIONS        0x00000004      
#define HEAP_ZERO_MEMORY                0x00000008      
#define HEAP_REALLOC_IN_PLACE_ONLY      0x00000010      
#define HEAP_TAIL_CHECKING_ENABLED      0x00000020      
#define HEAP_FREE_CHECKING_ENABLED      0x00000040      
#define HEAP_DISABLE_COALESCE_ON_FREE   0x00000080      
#define HEAP_CREATE_ALIGN_16            0x00010000      
#define HEAP_CREATE_ENABLE_TRACING      0x00020000      
#define HEAP_CREATE_ENABLE_EXECUTE      0x00040000      
#define HEAP_MAXIMUM_TAG                0x0FFF              
#define HEAP_PSEUDO_TAG_FLAG            0x8000              
#define HEAP_TAG_SHIFT                  18                  
#define HEAP_CREATE_SEGMENT_HEAP        0x00000100      
#define HEAP_CREATE_HARDENED            0x00000200      
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if !defined(MIDL_PASS)
FORCEINLINE
DWORD
HEAP_MAKE_TAG_FLAGS (
    _In_ DWORD TagBase,
    _In_ DWORD Tag
    )

{
    return ((DWORD)((TagBase) + ((Tag) << HEAP_TAG_SHIFT)));
}
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#define IS_TEXT_UNICODE_ASCII16               0x0001
#define IS_TEXT_UNICODE_REVERSE_ASCII16       0x0010

#define IS_TEXT_UNICODE_STATISTICS            0x0002
#define IS_TEXT_UNICODE_REVERSE_STATISTICS    0x0020

#define IS_TEXT_UNICODE_CONTROLS              0x0004
#define IS_TEXT_UNICODE_REVERSE_CONTROLS      0x0040

#define IS_TEXT_UNICODE_SIGNATURE             0x0008
#define IS_TEXT_UNICODE_REVERSE_SIGNATURE     0x0080

#define IS_TEXT_UNICODE_ILLEGAL_CHARS         0x0100
#define IS_TEXT_UNICODE_ODD_LENGTH            0x0200
#define IS_TEXT_UNICODE_DBCS_LEADBYTE         0x0400
#define IS_TEXT_UNICODE_UTF8                  0x0800
#define IS_TEXT_UNICODE_NULL_BYTES            0x1000

#define IS_TEXT_UNICODE_UNICODE_MASK          0x000F
#define IS_TEXT_UNICODE_REVERSE_MASK          0x00F0
#define IS_TEXT_UNICODE_NOT_UNICODE_MASK      0x0F00
#define IS_TEXT_UNICODE_NOT_ASCII_MASK        0xF000

#define COMPRESSION_FORMAT_NONE          (0x0000)   
#define COMPRESSION_FORMAT_DEFAULT       (0x0001)   
#define COMPRESSION_FORMAT_LZNT1         (0x0002)   
#define COMPRESSION_FORMAT_XPRESS        (0x0003)   
#define COMPRESSION_FORMAT_XPRESS_HUFF   (0x0004)   
#define COMPRESSION_FORMAT_XP10          (0x0005)   
#define COMPRESSION_FORMAT_LZ4           (0x0006)   
#define COMPRESSION_FORMAT_DEFLATE       (0x0007)   
#define COMPRESSION_FORMAT_ZLIB          (0x0008)   
#define COMPRESSION_ENGINE_STANDARD      (0x0000)   
#define COMPRESSION_ENGINE_MAXIMUM       (0x0100)   
#define COMPRESSION_ENGINE_HIBER         (0x0200)   

#if defined(_DBG_MEMCPY_INLINE_) && !defined(MIDL_PASS) && !defined(_MEMCPY_INLINE_) && !defined(_CRTBLD)
#define _MEMCPY_INLINE_
FORCEINLINE
PVOID
__cdecl
memcpy_inline (
    _Out_writes_bytes_all_(size) void *dst,
    _In_reads_bytes_(size) const void *src,
    _In_ size_t size
    )
{
    //
    // Make sure the source and destination do not overlap such that the
    // move destroys the destination.
    //
    if (((char *)dst > (char *)src) &&
        ((char *)dst < ((char *)src + size))) {
        __debugbreak();
    }
    return memcpy(dst, src, size);
}
#define memcpy memcpy_inline
#endif


#define RtlEqualMemory(Destination,Source,Length) (!memcmp((Destination),(Source),(Length)))
#define RtlMoveMemory(Destination,Source,Length) memmove((Destination),(Source),(Length))
#define RtlCopyMemory(Destination,Source,Length) memcpy((Destination),(Source),(Length))
#define RtlFillMemory(Destination,Length,Fill) memset((Destination),(Fill),(Length))
#define RtlZeroMemory(Destination,Length) memset((Destination),0,(Length))

// begin_ntosp

#if !defined(MIDL_PASS) && !defined(SORTPP_PASS) && !defined(RC_INVOKED)

#ifndef _RTL_VOL_MEM_ACCESSORS_
#define _RTL_VOL_MEM_ACCESSORS_

volatile void*
__cdecl
RtlCopyDeviceMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_reads_bytes_(Length) volatile const void* Source,
    _In_ size_t Length
    );

volatile void*
__cdecl
RtlCopyVolatileMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_reads_bytes_(Length) volatile const void* Source,
    _In_ size_t Length
    );

volatile void*
__cdecl
RtlMoveVolatileMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_reads_bytes_(Length) volatile const void* Source,
    _In_ size_t Length
    );

volatile void*
__cdecl
RtlSetVolatileMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ int Fill,
    _In_ size_t Length
    );

FORCEINLINE
volatile void*
RtlFillVolatileMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ size_t Length,
    _In_ int Fill
    )
{
    return RtlSetVolatileMemory(Destination, Fill, Length);
}

FORCEINLINE
volatile void*
RtlZeroVolatileMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ size_t Length
    )
{
    return RtlFillVolatileMemory(Destination, Length, 0);
}

FORCEINLINE
volatile void*
RtlSecureZeroMemory2 (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ size_t Length
    )
{
    return RtlZeroVolatileMemory(Destination, Length);
}

#if defined(_ARM_) || defined(_ARM64_)

volatile void*
RtlFillDeviceMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ size_t Length,
    _In_ int Fill
    );

#define RtlEqualDeviceMemory(Source1,Source2,Length) (!_memcmp_strict_align((Source1),(Source2),(Length)))

SIZE_T
NTAPI
RtlCompareDeviceMemory (
    _In_reads_bytes_(Length) const VOID *Source1,
    _In_reads_bytes_(Length) const VOID *Source2,
    _In_ SIZE_T Length
    );

#else

FORCEINLINE
volatile void*
RtlFillDeviceMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ size_t Length,
    _In_ int Fill
    )
{
    return RtlSetVolatileMemory(Destination, Fill, Length);
}

#define RtlEqualDeviceMemory RtlEqualMemory

#define RtlCompareDeviceMemory RtlCompareMemory

#endif

FORCEINLINE
volatile void*
RtlZeroDeviceMemory (
    _Out_writes_bytes_all_(Length) volatile void* Destination,
    _In_ size_t Length
    )
{
    return RtlFillDeviceMemory(Destination, Length, 0);
}

#endif // _RTL_VOL_MEM_ACCESSORS_

#endif // !defined(MIDL_PASS) && !defined(SORTPP_PASS) && !defined(RC_INVOKED)

// end_ntosp

#if !defined(MIDL_PASS)

_Check_return_
FORCEINLINE
int
RtlConstantTimeEqualMemory(
    _In_reads_bytes_(len) const void* v1,
    _In_reads_bytes_(len) const void* v2,
    unsigned long len
    )
{
    char x = 0;
    unsigned long i = 0;

    // Use volatile to prevent compiler from optimizing read
    volatile const char* p1 = (volatile const char*) v1;
    volatile const char* p2 = (volatile const char*) v2;

    for (; i < len; i += 1) {

#if !defined(_M_CEE) && (defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC))

        // Faster is better even when constant time is required.  Optimize performance
        // by avoiding memory barrier generation for ARM.  The parameters to this
        // function are NOT volatile, so the caller must not expect the generation
        // of non-ISO volatile memory barriers regardless of the compiler flags used.
        // Instead, if the caller shares this memory with other threads, it is
        // responsible for synchronizing access with the associated acquire barrier.

        x |= __iso_volatile_load8(&p1[i]) ^ __iso_volatile_load8(&p2[i]);

#else

        x |= p1[i] ^ p2[i];

#endif

    }

    return x == 0;
}

#endif

#if !defined(MIDL_PASS) && !defined(SORTPP_PASS) && !defined(RC_INVOKED)

#if defined(VOLATILE_ACCESSOR_LIB)

#define RtlSecureZeroMemory(Ptr, cnt) RtlSecureZeroMemory2((Ptr), (cnt));

#else // defined(VOLATILE_ACCESSOR_LIB)

FORCEINLINE
PVOID
RtlSecureZeroMemory(
    _Out_writes_bytes_all_(cnt) PVOID ptr,
    _In_ SIZE_T cnt
    )
{
    volatile char *vptr = (volatile char *)ptr;

#if defined(_M_AMD64) && !defined(_M_ARM64EC)

    __stosb((PBYTE )((DWORD64)vptr), 0, cnt);

#else

    while (cnt) {

#if !defined(_M_CEE) && (defined(_M_ARM) || defined(_M_ARM64) || defined(_M_ARM64EC))

        __iso_volatile_store8(vptr, 0);

#else

        *vptr = 0;

#endif

        vptr++;
        cnt--;
    }

#endif // _M_AMD64 && !defined(_M_ARM64EC)

    return ptr;
}

#endif

#endif //!defined(MIDL_PASS)

// begin_wdm

#define SEF_DACL_AUTO_INHERIT             0x01
#define SEF_SACL_AUTO_INHERIT             0x02
#define SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT 0x04
#define SEF_AVOID_PRIVILEGE_CHECK         0x08
#define SEF_AVOID_OWNER_CHECK             0x10
#define SEF_DEFAULT_OWNER_FROM_PARENT     0x20
#define SEF_DEFAULT_GROUP_FROM_PARENT     0x40
#define SEF_MACL_NO_WRITE_UP              0x100
#define SEF_MACL_NO_READ_UP               0x200
#define SEF_MACL_NO_EXECUTE_UP            0x400
#define SEF_AI_USE_EXTRA_PARAMS           0x800
#define SEF_AVOID_OWNER_RESTRICTION       0x1000
#define SEF_FORCE_USER_MODE               0x2000
#define SEF_NORMALIZE_OUTPUT_DESCRIPTOR   0x4000

#define SEF_MACL_VALID_FLAGS              (SEF_MACL_NO_WRITE_UP   | \
                                           SEF_MACL_NO_READ_UP    | \
                                           SEF_MACL_NO_EXECUTE_UP)

// end_wdm
// end_ntifs
// begin_ntosifs

typedef struct _MESSAGE_RESOURCE_ENTRY {
    WORD   Length;
    WORD   Flags;
    BYTE  Text[ 1 ];
} MESSAGE_RESOURCE_ENTRY, *PMESSAGE_RESOURCE_ENTRY;

#define MESSAGE_RESOURCE_UNICODE 0x0001
#define MESSAGE_RESOURCE_UTF8 0x0002

typedef struct _MESSAGE_RESOURCE_BLOCK {
    DWORD LowId;
    DWORD HighId;
    DWORD OffsetToEntries;
} MESSAGE_RESOURCE_BLOCK, *PMESSAGE_RESOURCE_BLOCK;

typedef struct _MESSAGE_RESOURCE_DATA {
    DWORD NumberOfBlocks;
    MESSAGE_RESOURCE_BLOCK Blocks[ 1 ];
} MESSAGE_RESOURCE_DATA, *PMESSAGE_RESOURCE_DATA;

// end_ntosifs
typedef struct _OSVERSIONINFOA {
    DWORD dwOSVersionInfoSize;
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
    DWORD dwBuildNumber;
    DWORD dwPlatformId;
    CHAR   szCSDVersion[ 128 ];     // Maintenance string for PSS usage
} OSVERSIONINFOA, *POSVERSIONINFOA, *LPOSVERSIONINFOA;

typedef struct _OSVERSIONINFOW {
    DWORD dwOSVersionInfoSize;
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
    DWORD dwBuildNumber;
    DWORD dwPlatformId;
    WCHAR  szCSDVersion[ 128 ];     // Maintenance string for PSS usage
} OSVERSIONINFOW, *POSVERSIONINFOW, *LPOSVERSIONINFOW, RTL_OSVERSIONINFOW, *PRTL_OSVERSIONINFOW;
#ifdef UNICODE
typedef OSVERSIONINFOW OSVERSIONINFO;
typedef POSVERSIONINFOW POSVERSIONINFO;
typedef LPOSVERSIONINFOW LPOSVERSIONINFO;
#else
typedef OSVERSIONINFOA OSVERSIONINFO;
typedef POSVERSIONINFOA POSVERSIONINFO;
typedef LPOSVERSIONINFOA LPOSVERSIONINFO;
#endif // UNICODE

typedef struct _OSVERSIONINFOEXA {
    DWORD dwOSVersionInfoSize;
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
    DWORD dwBuildNumber;
    DWORD dwPlatformId;
    CHAR   szCSDVersion[ 128 ];     // Maintenance string for PSS usage
    WORD   wServicePackMajor;
    WORD   wServicePackMinor;
    WORD   wSuiteMask;
    BYTE  wProductType;
    BYTE  wReserved;
} OSVERSIONINFOEXA, *POSVERSIONINFOEXA, *LPOSVERSIONINFOEXA;
typedef struct _OSVERSIONINFOEXW {
    DWORD dwOSVersionInfoSize;
    DWORD dwMajorVersion;
    DWORD dwMinorVersion;
    DWORD dwBuildNumber;
    DWORD dwPlatformId;
    WCHAR  szCSDVersion[ 128 ];     // Maintenance string for PSS usage
    WORD   wServicePackMajor;
    WORD   wServicePackMinor;
    WORD   wSuiteMask;
    BYTE  wProductType;
    BYTE  wReserved;
} OSVERSIONINFOEXW, *POSVERSIONINFOEXW, *LPOSVERSIONINFOEXW, RTL_OSVERSIONINFOEXW, *PRTL_OSVERSIONINFOEXW;
#ifdef UNICODE
typedef OSVERSIONINFOEXW OSVERSIONINFOEX;
typedef POSVERSIONINFOEXW POSVERSIONINFOEX;
typedef LPOSVERSIONINFOEXW LPOSVERSIONINFOEX;
#else
typedef OSVERSIONINFOEXA OSVERSIONINFOEX;
typedef POSVERSIONINFOEXA POSVERSIONINFOEX;
typedef LPOSVERSIONINFOEXA LPOSVERSIONINFOEX;
#endif // UNICODE

// begin_wudfpwdm

//
// RtlVerifyVersionInfo() conditions
//

#define VER_EQUAL                       1
#define VER_GREATER                     2
#define VER_GREATER_EQUAL               3
#define VER_LESS                        4
#define VER_LESS_EQUAL                  5
#define VER_AND                         6
#define VER_OR                          7

#define VER_CONDITION_MASK              7
#define VER_NUM_BITS_PER_CONDITION_MASK 3

//
// RtlVerifyVersionInfo() type mask bits
//

#define VER_MINORVERSION                0x0000001
#define VER_MAJORVERSION                0x0000002
#define VER_BUILDNUMBER                 0x0000004
#define VER_PLATFORMID                  0x0000008
#define VER_SERVICEPACKMINOR            0x0000010
#define VER_SERVICEPACKMAJOR            0x0000020
#define VER_SUITENAME                   0x0000040
#define VER_PRODUCT_TYPE                0x0000080

//
// RtlVerifyVersionInfo() os product type values
//

#define VER_NT_WORKSTATION              0x0000001
#define VER_NT_DOMAIN_CONTROLLER        0x0000002
#define VER_NT_SERVER                   0x0000003

//
// dwPlatformId defines:
//

#define VER_PLATFORM_WIN32s             0
#define VER_PLATFORM_WIN32_WINDOWS      1
#define VER_PLATFORM_WIN32_NT           2

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

//
//
// VerifyVersionInfo() macro to set the condition mask
//
// For documentation sakes here's the old version of the macro that got
// changed to call an API
// #define VER_SET_CONDITION(_m_,_t_,_c_)  _m_=(_m_|(_c_<<(1<<_t_)))
//

#define VER_SET_CONDITION(_m_,_t_,_c_)  \
        ((_m_)=VerSetConditionMask((_m_),(_t_),(_c_)))

#if !defined(_WINBASE_) && !defined(MIDL_PASS)

#if (NTDDI_VERSION >= NTDDI_WIN2K)

NTSYSAPI
ULONGLONG
NTAPI
VerSetConditionMask(
    _In_ ULONGLONG ConditionMask,
    _In_ DWORD TypeMask,
    _In_ BYTE  Condition
    );

#endif

#endif // !defined(_WINBASE_) && !defined(MIDL_PASS)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

//

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// begin_ntddk

#if (NTDDI_VERSION >= NTDDI_VISTA)

NTSYSAPI
BOOLEAN
NTAPI
RtlGetProductInfo(
    _In_  DWORD  OSMajorVersion,
    _In_  DWORD  OSMinorVersion,
    _In_  DWORD  SpMajorVersion,
    _In_  DWORD  SpMinorVersion,
    _Out_ PDWORD ReturnedProductType
    );

#endif

// end_ntddk

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define RTL_UMS_VERSION  (0x0100)  

typedef enum _RTL_UMS_THREAD_INFO_CLASS {
    UmsThreadInvalidInfoClass = 0,
    UmsThreadUserContext,
    UmsThreadPriority,              // Reserved
    UmsThreadAffinity,              // Reserved
    UmsThreadTeb,
    UmsThreadIsSuspended,
    UmsThreadIsTerminated,
    UmsThreadMaxInfoClass
} RTL_UMS_THREAD_INFO_CLASS, *PRTL_UMS_THREAD_INFO_CLASS;

typedef enum _RTL_UMS_SCHEDULER_REASON {
    UmsSchedulerStartup = 0,
    UmsSchedulerThreadBlocked,
    UmsSchedulerThreadYield,
} RTL_UMS_SCHEDULER_REASON, *PRTL_UMS_SCHEDULER_REASON;

typedef
_Function_class_(RTL_UMS_SCHEDULER_ENTRY_POINT)
VOID
NTAPI
RTL_UMS_SCHEDULER_ENTRY_POINT(
    _In_ RTL_UMS_SCHEDULER_REASON Reason,
    _In_ ULONG_PTR ActivationPayload,
    _In_ PVOID SchedulerParam
    );

typedef RTL_UMS_SCHEDULER_ENTRY_POINT *PRTL_UMS_SCHEDULER_ENTRY_POINT;


#if !defined(IS_VALIDATION_ENABLED)

#if (NTDDI_VERSION >= NTDDI_WIN8)

//
// Validation runlevel helper macro: checks if a particular level L enables the
// validation class C.
//
// Returns a non-zero scalar if class C is enabled, and zero otherwise.
//
#define IS_VALIDATION_ENABLED(C,L) ((L) & (C))

//
// Validation classes are broken into:
//      8 predefined validation classes, spanning bits 0 to 7 of the level value
//     24 custom-defined validation classes, spanning bits 8 to 31
//
#define VRL_PREDEFINED_CLASS_BEGIN  (1 << 0)
#define VRL_CUSTOM_CLASS_BEGIN      (1 << 8)

//
// The following are predefined validation classes.
//
#define VRL_CLASS_CONSISTENCY       (VRL_PREDEFINED_CLASS_BEGIN << 0)

//
// Do not ignore kernel breaks when kernel debugging is disabled (debug builds only)
//
#define VRL_ENABLE_KERNEL_BREAKS    (1 << 31)

#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#endif // !defined(IS_VALIDATION_ENABLED)

#if (NTDDI_VERSION >= NTDDI_WIN8)

//
// RtlCheckTokenMembership flags.
//

#define CTMF_INCLUDE_APPCONTAINER   0x00000001UL
#define CTMF_INCLUDE_LPAC           0x00000002UL
#define CTMF_VALID_FLAGS            (CTMF_INCLUDE_APPCONTAINER | CTMF_INCLUDE_LPAC)

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


#if (NTDDI_VERSION >= NTDDI_WIN8)

// end_ntosp

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

// begin_ntosp

//
//  Crc32 and Crc64 routines that use standardized algorithms
//

NTSYSAPI
DWORD
NTAPI
RtlCrc32(
    _In_reads_bytes_(Size) const void *Buffer,
    _In_ size_t Size,
    _In_ DWORD InitialCrc
    );

NTSYSAPI
ULONGLONG
NTAPI
RtlCrc64(
    _In_reads_bytes_(Size) const void *Buffer,
    _In_ size_t Size,
    _In_ ULONGLONG InitialCrc
    );

// end_ntosp

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

// begin_ntosp

#endif // (NTDDI_VERSION >= NTDDI_WIN8)


#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
//
//  API to detect what type of OS Deployment this is.  Current valid values
//  are listed below
//

//
//  Valid OsDeployment values that can be returned
//

typedef enum _OS_DEPLOYEMENT_STATE_VALUES {
    OS_DEPLOYMENT_STANDARD = 1,
    OS_DEPLOYMENT_COMPACT
} OS_DEPLOYEMENT_STATE_VALUES;

NTSYSAPI
OS_DEPLOYEMENT_STATE_VALUES
NTAPI
RtlOsDeploymentState(
    _In_ DWORD Flags    /* No flags currently defined, passed 0 */
    );


#endif // NTDDI_VERSION >= NTDDI_WINTHRESHOLD


//
// Flush routines for DAX mapped files
//

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

typedef struct _NV_MEMORY_RANGE {
    VOID *BaseAddress;
    SIZE_T Length;
} NV_MEMORY_RANGE, *PNV_MEMORY_RANGE;

//
// Flags for RtlFlushNonVolatileMemory and RtlFlushNonVolatileMemoryRanges
//

#define FLUSH_NV_MEMORY_IN_FLAG_NO_DRAIN    (0x00000001)

//
// Default token is used to call the flush and drain routines without the
// RtlGetNonVolatileToken call, for callers who know the details about the
// region they are flushing.
//

#define FLUSH_NV_MEMORY_DEFAULT_TOKEN       (ULONG_PTR)(-1)

//
// Flags for RtlWriteNonVolatileMemory
//
// WRITE_NV_MEMORY_FLAG_FLUSH - The destination range is flushed
// after writing the data from the source.  By default the flush
// is followed by a drain, unless WRITE_NV_MEMORY_FLAG_NO_DRAIN
// is passed. This flag makes sure that the data is durable even
// in case of system powerloss.
//
// WRITE_NV_MEMORY_FLAG_NON_TEMPORAL - Performs a non-temporal
// copy when available.  Non-temporal means that data caching
// is not required.  Few notes:
//   - Some processors do not support non-temporal copy for range
//     smaller than address bus size, and issues regular copy. In
//     such case RtlWriteNonVolatileMemory peforms regular copy
//     followed by flush and drain.
//   - Some processors may ignore non-temporal copy altogether.
//     RtlWriteNonVolatileMemory doesn't replace it with flush
//     and drain in that case.
//
// WRITE_NV_MEMORY_FLAG_PERSIST - Makes sure that the write are
// persisted either by flushing or using non-temporal writes.
// The caller can make no assumptions on what's used.  Typically
// the approach that's less costly to persist the write is used.
// This flag makes sure that the data is durable even in case of
// system powerloss.  WRITE_NV_MEMORY_FLAG_NO_DRAIN is ignored
// when WRITE_NV_MEMORY_FLAG_PERSIST is used.
//
// WRITE_NV_MEMORY_FLAG_NO_DRAIN - Tells the routine to not wait
// for the flush to complete.
//

#define WRITE_NV_MEMORY_FLAG_FLUSH          (0x00000001)
#define WRITE_NV_MEMORY_FLAG_NON_TEMPORAL   (0x00000002)
#define WRITE_NV_MEMORY_FLAG_PERSIST        (WRITE_NV_MEMORY_FLAG_FLUSH \
                                              | WRITE_NV_MEMORY_FLAG_NON_TEMPORAL)
#define WRITE_NV_MEMORY_FLAG_NO_DRAIN       (0x00000100)

//
// Flags for RtlFillNonVolatileMemory
//
// Let the flags be available in pre 19H1 too, to ease callers
// of this function that use MmGetSystemRoutineAddress.
//
// FILL_NV_MEMORY_FLAG_FLUSH - The destination range is flushed
// after setting the given value into it.  By default the flush
// is followed by a drain, unless FILL_NV_MEMORY_FLAG_NO_DRAIN
// is passed.  This flag makes sure that the data is durable
// even in case of system powerloss.
//
// FILL_NV_MEMORY_FLAG_NON_TEMPORAL - Performs a non-temporal
// memset when available.  Non-temporal means that data caching
// is not required.  Few notes:
//   - Some processors do not support non-temporal moves for range
//     smaller than address bus size, and issues regular moves. In
//     such case RtlSetNonVolatileMemory peforms regular memset
//     followed by flush and drain.
//   - Some processors may ignore non-temporal moves altogether.
//     RtlSetNonVolatileMemory doesn't replace it with flush and
//     drain in that case.  However, we do replace it with flush
//     and drain if Windows doesn't haven an implementation for
//     a given architecture (say ARM64).
//
// FILL_NV_MEMORY_FLAG_PERSIST - Makes sure that the memset is
// persisted either by flushing or using non-temporal moves.
// The caller can make no assumptions on what's used.  Typically
// the approach that's less costly to persist the data is used.
// This flag makes sure that the data is durable even in case of
// system powerloss.  FILL_NV_MEMORY_FLAG_NO_DRAIN is ignored
// when FILL_NV_MEMORY_FLAG_PERSIST is used.
//
// FILL_NV_MEMORY_FLAG_NO_DRAIN - Tells the routine to not wait
// for the flush to complete.  This is honored only when the
// caller explicitly passed FILL_NV_MEMORY_FLAG_FLUSH and isn't
// honored for FILL_NV_MEMORY_FLAG_PERSIST or *_NON_TERMPORAL.
//

#define FILL_NV_MEMORY_FLAG_FLUSH           (0x00000001)
#define FILL_NV_MEMORY_FLAG_NON_TEMPORAL    (0x00000002)
#define FILL_NV_MEMORY_FLAG_PERSIST         (FILL_NV_MEMORY_FLAG_FLUSH \
                                              | FILL_NV_MEMORY_FLAG_NON_TEMPORAL)
#define FILL_NV_MEMORY_FLAG_NO_DRAIN        (0x00000100)

#if defined(_WIN64)

_IRQL_requires_max_(APC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlGetNonVolatileToken (
    _In_reads_bytes_(Size) PVOID NvBuffer,
    _In_ SIZE_T Size,
    _Outptr_ PVOID *NvToken
    );

_IRQL_requires_max_(DPC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlFreeNonVolatileToken (
    _In_ PVOID NvToken
    );

_IRQL_requires_max_(DPC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlFlushNonVolatileMemory (
    _In_ PVOID NvToken,
    _In_reads_bytes_(Size) PVOID NvBuffer,
    _In_ SIZE_T Size,
    _In_ DWORD Flags
    );

_IRQL_requires_max_(DPC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlDrainNonVolatileFlush (
    _In_ PVOID NvToken
    );

_IRQL_requires_max_(DPC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlWriteNonVolatileMemory (
    _In_ PVOID NvToken,
    _Out_writes_bytes_(Size) VOID UNALIGNED *NvDestination,
    _In_reads_bytes_(Size) CONST VOID UNALIGNED *Source,
    _In_ SIZE_T Size,
    _In_ DWORD Flags
    );

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)

_IRQL_requires_max_(DPC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlFillNonVolatileMemory (
    _In_ PVOID NvToken,
    _Out_writes_bytes_(Size) VOID UNALIGNED *NvDestination,
    _In_ SIZE_T Size,
    _In_ CONST BYTE  Value,
    _In_ DWORD Flags
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)

_IRQL_requires_max_(DPC_LEVEL)
NTSYSAPI
DWORD   
NTAPI
RtlFlushNonVolatileMemoryRanges (
    _In_ PVOID NvToken,
    _In_reads_(NumRanges) PNV_MEMORY_RANGE NvRanges,
    _In_ SIZE_T NumRanges,
    _In_ DWORD Flags
    );

#else // defined(_WIN64)

#define RtlGetNonVolatileToken(B,S,T) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#define RtlFreeNonVolatileToken(T) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#define RtlFlushNonVolatileMemory(T,B,S,F) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#define RtlDrainNonVolatileFlush(T) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#define RtlWriteNonVolatileMemory(T,D,S,L,F) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)

#define RtlFillNonVolatileMemory(T,B,S,V,F) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_19H1)

#define RtlFlushNonVolatileMemoryRanges(T,R,N,F) \
    (ASSERT(!"Call not expected in 32-bit architecture"), STATUS_NOT_IMPLEMENTED)

#endif // defined(_WIN64)

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)


//
// Correlation Vector Routines.
//

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)


#define RTL_CORRELATION_VECTOR_STRING_LENGTH 129
#define RTL_CORRELATION_VECTOR_VERSION_1 ((CHAR)1)
#define RTL_CORRELATION_VECTOR_VERSION_2 ((CHAR)2)
#define RTL_CORRELATION_VECTOR_VERSION_CURRENT RTL_CORRELATION_VECTOR_VERSION_2

#define RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH (16)
#define RTL_CORRELATION_VECTOR_V1_LENGTH (64)

#define RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH (22)
#define RTL_CORRELATION_VECTOR_V2_LENGTH (128)

typedef struct CORRELATION_VECTOR {
    CHAR Version;
    CHAR Vector[RTL_CORRELATION_VECTOR_STRING_LENGTH];
} CORRELATION_VECTOR;

typedef CORRELATION_VECTOR *PCORRELATION_VECTOR;

#define TraceLoggingCORRELATION_VECTOR(cv) TraceLoggingString((cv).Vector, "__TlgCV__")

NTSYSAPI
DWORD   
NTAPI
RtlInitializeCorrelationVector(
    _Out_ PCORRELATION_VECTOR CorrelationVector,
    _In_  int Version,
    _In_opt_  const GUID * Guid
    );


NTSYSAPI
DWORD   
NTAPI
RtlIncrementCorrelationVector(
    _Inout_ PCORRELATION_VECTOR CorrelationVector
    );

NTSYSAPI
DWORD   
NTAPI
RtlExtendCorrelationVector(
    _Inout_ PCORRELATION_VECTOR CorrelationVector
    );

NTSYSAPI
DWORD   
NTAPI
RtlValidateCorrelationVector(
    _In_ PCORRELATION_VECTOR Vector
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS2


#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)

typedef struct _CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    //
    // Size of the structure in bytes
    //
    DWORD Size;

    //
    // Guid used to identify background task to trigger
    //
    PCWSTR TriggerId;

} CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG, *PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG;

#if !defined(MIDL_PASS)
FORCEINLINE
VOID
CUSTOM_SYSTEM_EVENT_TRIGGER_INIT(
    _Out_    PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG Config,
    _In_opt_ PCWSTR TriggerId
    )
{
    RtlZeroMemory(Config, sizeof(CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG));

    Config->Size = sizeof(CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG);
    Config->TriggerId = TriggerId;
}
#endif // !defined(MIDL_PASS)

_Must_inspect_result_
_IRQL_requires_max_(PASSIVE_LEVEL)
DWORD   
NTAPI
RtlRaiseCustomSystemEventTrigger(
    _In_ PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG TriggerConfig
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS4


//
// Support for process policy settings embedded into executable image.
//

#define IMAGE_POLICY_METADATA_VERSION 1
#define IMAGE_POLICY_SECTION_NAME ".tPolicy"
#define IMAGE_POLICY_METADATA_NAME __ImagePolicyMetadata

typedef enum _IMAGE_POLICY_ENTRY_TYPE {
    ImagePolicyEntryTypeNone = 0,
    ImagePolicyEntryTypeBool,
    ImagePolicyEntryTypeInt8,
    ImagePolicyEntryTypeUInt8,
    ImagePolicyEntryTypeInt16,
    ImagePolicyEntryTypeUInt16,
    ImagePolicyEntryTypeInt32,
    ImagePolicyEntryTypeUInt32,
    ImagePolicyEntryTypeInt64,
    ImagePolicyEntryTypeUInt64,
    ImagePolicyEntryTypeAnsiString,
    ImagePolicyEntryTypeUnicodeString,
    ImagePolicyEntryTypeOverride,
    ImagePolicyEntryTypeMaximum
} IMAGE_POLICY_ENTRY_TYPE;

typedef enum _IMAGE_POLICY_ID {
    ImagePolicyIdNone = 0,
    ImagePolicyIdEtw,
    ImagePolicyIdDebug,
    ImagePolicyIdCrashDump,
    ImagePolicyIdCrashDumpKey,
    ImagePolicyIdCrashDumpKeyGuid,
    ImagePolicyIdParentSd,
    ImagePolicyIdParentSdRev,
    ImagePolicyIdSvn,
    ImagePolicyIdDeviceId,
    ImagePolicyIdCapability,
    ImagePolicyIdScenarioId,
    ImagePolicyIdCapabilityOverridable,
    ImagePolicyIdTrustletIdOverridable,
    ImagePolicyIdMaximum
} IMAGE_POLICY_ID;

typedef struct _IMAGE_POLICY_ENTRY {
    IMAGE_POLICY_ENTRY_TYPE Type;
    IMAGE_POLICY_ID PolicyId;
    union {
        const VOID* None;
        BOOLEAN BoolValue;
        INT8 Int8Value;
        UINT8 UInt8Value;
        INT16 Int16Value;
        UINT16 UInt16Value;
        INT32 Int32Value;
        UINT32 UInt32Value;
        INT64 Int64Value;
        UINT64 UInt64Value;
        PCSTR AnsiStringValue;
        PCWSTR UnicodeStringValue;
    } u;
} IMAGE_POLICY_ENTRY;
typedef const IMAGE_POLICY_ENTRY* PCIMAGE_POLICY_ENTRY;

#ifdef _MSC_EXTENSIONS

#pragma warning(push)
#pragma warning(disable:4200) // zero-sized array in struct/union
typedef struct _IMAGE_POLICY_METADATA {
    BYTE  Version;
    BYTE  Reserved0[7];
    ULONGLONG ApplicationId;
    IMAGE_POLICY_ENTRY Policies[];
} IMAGE_POLICY_METADATA;
typedef const IMAGE_POLICY_METADATA* PCIMAGE_POLICY_METADATA;
#pragma warning(pop)

#define IMAGE_POLICY_START(_ApplicationId_)                                   \
__pragma(const_seg(push, IMAGE_POLICY_SECTION_NAME));                         \
EXTERN_C __declspec(dllexport) const                                          \
IMAGE_POLICY_METADATA IMAGE_POLICY_METADATA_NAME = {                          \
    IMAGE_POLICY_METADATA_VERSION,                                            \
    {0},                                                                      \
    _ApplicationId_,                                                          \
    {

#define IMAGE_POLICY_END()                                                    \
        {ImagePolicyEntryTypeNone, ImagePolicyIdNone, NULL}                   \
    }                                                                         \
};                                                                            \
__pragma(const_seg(pop))

#endif

#define IMAGE_POLICY_BOOL(_PolicyId_, _Value_)             \
    {ImagePolicyEntryTypeBool, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_INT8(_PolicyId_, _Value_)             \
    {ImagePolicyEntryTypeInt8, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_UINT8(_PolicyId_, _Value_)            \
    {ImagePolicyEntryTypeUInt8, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_INT16(_PolicyId_, _Value_)            \
    {ImagePolicyEntryTypeInt16, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_UINT16(_PolicyId_, _Value_)           \
    {ImagePolicyEntryTypeUInt16, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_INT32(_PolicyId_, _Value_)            \
    {ImagePolicyEntryTypeInt32, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_UINT32(_PolicyId_, _Value_)           \
    {ImagePolicyEntryTypeUInt32, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_INT64(_PolicyId_, _Value_)            \
    {ImagePolicyEntryTypeInt64, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_UINT64(_PolicyId_, _Value_)           \
    {ImagePolicyEntryTypeUInt64, _PolicyId_, (const VOID*)_Value_},

#define IMAGE_POLICY_ANSI_STRING(_PolicyId_, _Value_)      \
    {ImagePolicyEntryTypeAnsiString, _PolicyId_, _Value_},

#define IMAGE_POLICY_UNICODE_STRING(_PolicyId_, _Value_)   \
    {ImagePolicyEntryTypeUnicodeString, _PolicyId_, _Value_},

#define IMAGE_POLICY_OVERRIDE(_PolicyId_)   \
    {ImagePolicyEntryTypeOverride, _PolicyId_, 0},


#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
NTSYSAPI
BOOLEAN
NTAPI
RtlIsZeroMemory (
    _In_ PVOID Buffer,
    _In_ SIZE_T Length
    );

__drv_maxIRQL(APC_LEVEL)
NTSYSAPI
BOOLEAN
NTAPI
RtlNormalizeSecurityDescriptor (
    _Inout_ PSECURITY_DESCRIPTOR *SecurityDescriptor,
    _In_ DWORD SecurityDescriptorLength,
    _Out_opt_ PSECURITY_DESCRIPTOR *NewSecurityDescriptor,
    _Out_opt_ PDWORD NewSecurityDescriptorLength,
    _In_ BOOLEAN CheckOnly
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_VB

#if (NTDDI_VERSION >= NTDDI_WIN10_FE)

//
// Flags for RtlVirtualUnwind2.
//

#define RTL_VIRTUAL_UNWIND2_VALIDATE_PAC        0x00000001UL

//
// Shared User Data fields and accessors.
//

typedef enum _RTL_SYSTEM_GLOBAL_DATA_ID {
    GlobalDataIdUnknown = 0,
    GlobalDataIdRngSeedVersion,
    GlobalDataIdInterruptTime,
    GlobalDataIdTimeZoneBias,
    GlobalDataIdImageNumberLow,
    GlobalDataIdImageNumberHigh,
    GlobalDataIdTimeZoneId,
    GlobalDataIdNtMajorVersion,
    GlobalDataIdNtMinorVersion,
    GlobalDataIdSystemExpirationDate,
    GlobalDataIdKdDebuggerEnabled,
    GlobalDataIdCyclesPerYield,
    GlobalDataIdSafeBootMode,
    GlobalDataIdLastSystemRITEventTickCount,
    GlobalDataIdConsoleSharedDataFlags,
    GlobalDataIdNtSystemRootDrive,
    GlobalDataIdQpcBypassEnabled,
    GlobalDataIdQpcData,
    GlobalDataIdQpcBias
} RTL_SYSTEM_GLOBAL_DATA_ID, *PRTL_SYSTEM_GLOBAL_DATA_ID;

NTSYSAPI
DWORD   
NTAPI
RtlGetSystemGlobalData (
    _In_ RTL_SYSTEM_GLOBAL_DATA_ID DataId,
    _Inout_ PVOID Buffer,
    _In_ DWORD Size
    );

NTSYSAPI
DWORD   
NTAPI
RtlSetSystemGlobalData (
    _In_ RTL_SYSTEM_GLOBAL_DATA_ID DataId,
    _In_ PVOID Buffer,
    _In_ DWORD Size
    );

#endif // NTDDI_VERSION >= NTDDI_WIN10_FE


typedef struct _RTL_CRITICAL_SECTION_DEBUG {
    WORD   Type;
    WORD   CreatorBackTraceIndex;
    struct _RTL_CRITICAL_SECTION *CriticalSection;
    LIST_ENTRY ProcessLocksList;
    DWORD EntryCount;
    DWORD ContentionCount;
    DWORD Flags;
    WORD   CreatorBackTraceIndexHigh;
    WORD   Identifier;
} RTL_CRITICAL_SECTION_DEBUG, *PRTL_CRITICAL_SECTION_DEBUG, RTL_RESOURCE_DEBUG, *PRTL_RESOURCE_DEBUG;

//
// These flags define the upper byte of the critical section SpinCount field
//
#define RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO         0x01000000
#define RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN          0x02000000
#define RTL_CRITICAL_SECTION_FLAG_STATIC_INIT           0x04000000
#define RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE         0x08000000
#define RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO      0x10000000
#define RTL_CRITICAL_SECTION_ALL_FLAG_BITS              0xFF000000
#define RTL_CRITICAL_SECTION_FLAG_RESERVED              (RTL_CRITICAL_SECTION_ALL_FLAG_BITS & (~(RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO | RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN | RTL_CRITICAL_SECTION_FLAG_STATIC_INIT | RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE | RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO)))

//
// These flags define possible values stored in the Flags field of a critsec debuginfo.
//
#define RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT     0x00000001

#pragma pack(push, 8)

typedef struct _RTL_CRITICAL_SECTION {
    PRTL_CRITICAL_SECTION_DEBUG DebugInfo;

    //
    //  The following three fields control entering and exiting the critical
    //  section for the resource
    //

    LONG LockCount;
    LONG RecursionCount;
    HANDLE OwningThread;        // from the thread's ClientId->UniqueThread
    HANDLE LockSemaphore;
    ULONG_PTR SpinCount;        // force size on 64-bit systems when packed
} RTL_CRITICAL_SECTION, *PRTL_CRITICAL_SECTION;

#pragma pack(pop)

typedef struct _RTL_SRWLOCK {                            
        PVOID Ptr;                                       
} RTL_SRWLOCK, *PRTL_SRWLOCK;                            
#define RTL_SRWLOCK_INIT {0}                            
typedef struct _RTL_CONDITION_VARIABLE {                    
        PVOID Ptr;                                       
} RTL_CONDITION_VARIABLE, *PRTL_CONDITION_VARIABLE;      
#define RTL_CONDITION_VARIABLE_INIT {0}                 
#define RTL_CONDITION_VARIABLE_LOCKMODE_SHARED  0x1     
typedef
VOID
(NTAPI *PAPCFUNC)(
    _In_ ULONG_PTR Parameter
    );
typedef LONG (NTAPI *PVECTORED_EXCEPTION_HANDLER)(
    struct _EXCEPTION_POINTERS *ExceptionInfo
    );

typedef enum _HEAP_INFORMATION_CLASS {

    HeapCompatibilityInformation = 0,
    HeapEnableTerminationOnCorruption = 1


#if ((NTDDI_VERSION > NTDDI_WINBLUE) || \
     (NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14)))
    ,

    HeapOptimizeResources = 3

#endif


    ,

    HeapTag = 7

} HEAP_INFORMATION_CLASS;

#if ((NTDDI_VERSION > NTDDI_WINBLUE) || \
     (NTDDI_VERSION == NTDDI_WINBLUE && defined(WINBLUE_KBSPRING14)))

#define HEAP_OPTIMIZE_RESOURCES_CURRENT_VERSION  1

typedef struct _HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    DWORD Version;
    DWORD Flags;
} HEAP_OPTIMIZE_RESOURCES_INFORMATION, *PHEAP_OPTIMIZE_RESOURCES_INFORMATION;

#endif


#define WT_EXECUTEDEFAULT       0x00000000                           
#define WT_EXECUTEINIOTHREAD    0x00000001                           
#define WT_EXECUTEINUITHREAD    0x00000002                           
#define WT_EXECUTEINWAITTHREAD  0x00000004                           
#define WT_EXECUTEONLYONCE      0x00000008                           
#define WT_EXECUTEINTIMERTHREAD 0x00000020                           
#define WT_EXECUTELONGFUNCTION  0x00000010                           
#define WT_EXECUTEINPERSISTENTIOTHREAD  0x00000040                   
#define WT_EXECUTEINPERSISTENTTHREAD 0x00000080                      
#define WT_TRANSFER_IMPERSONATION 0x00000100                         
#define WT_SET_MAX_THREADPOOL_THREADS(Flags, Limit)  ((Flags) |= (Limit)<<16) 
typedef VOID (NTAPI * WAITORTIMERCALLBACKFUNC) (PVOID, BOOLEAN );   
typedef VOID (NTAPI * WORKERCALLBACKFUNC) (PVOID );                 
typedef VOID (NTAPI * APC_CALLBACK_FUNCTION) (DWORD   , PVOID, PVOID); 
typedef WAITORTIMERCALLBACKFUNC WAITORTIMERCALLBACK; 
typedef
VOID
(NTAPI *PFLS_CALLBACK_FUNCTION) (
    _In_ PVOID lpFlsData
    );

typedef
BOOLEAN
(NTAPI *PSECURE_MEMORY_CACHE_CALLBACK) (
    _In_reads_bytes_(Range) PVOID Addr,
    _In_ SIZE_T Range
    );

#define WT_EXECUTEINLONGTHREAD  0x00000010                           
#define WT_EXECUTEDELETEWAIT    0x00000008                           

typedef enum _ACTIVATION_CONTEXT_INFO_CLASS {
    ActivationContextBasicInformation                       = 1,
    ActivationContextDetailedInformation                    = 2,
    AssemblyDetailedInformationInActivationContext          = 3,
    FileInformationInAssemblyOfAssemblyInActivationContext  = 4,
    RunlevelInformationInActivationContext                  = 5,
    CompatibilityInformationInActivationContext             = 6,
    ActivationContextManifestResourceName                   = 7,
    MaxActivationContextInfoClass,

    //
    // compatibility with old names
    //
    AssemblyDetailedInformationInActivationContxt           = 3,
    FileInformationInAssemblyOfAssemblyInActivationContxt   = 4
} ACTIVATION_CONTEXT_INFO_CLASS;

#define ACTIVATIONCONTEXTINFOCLASS ACTIVATION_CONTEXT_INFO_CLASS


typedef struct _ACTIVATION_CONTEXT_QUERY_INDEX {
    DWORD ulAssemblyIndex;
    DWORD ulFileIndexInAssembly;
} ACTIVATION_CONTEXT_QUERY_INDEX, * PACTIVATION_CONTEXT_QUERY_INDEX;

typedef const struct _ACTIVATION_CONTEXT_QUERY_INDEX * PCACTIVATION_CONTEXT_QUERY_INDEX;


#define ACTIVATION_CONTEXT_PATH_TYPE_NONE (1)
#define ACTIVATION_CONTEXT_PATH_TYPE_WIN32_FILE (2)
#define ACTIVATION_CONTEXT_PATH_TYPE_URL (3)
#define ACTIVATION_CONTEXT_PATH_TYPE_ASSEMBLYREF (4)

typedef struct _ASSEMBLY_FILE_DETAILED_INFORMATION {
    DWORD ulFlags;
    DWORD ulFilenameLength;
    DWORD ulPathLength;

    PCWSTR lpFileName;
    PCWSTR lpFilePath;
} ASSEMBLY_FILE_DETAILED_INFORMATION, *PASSEMBLY_FILE_DETAILED_INFORMATION;
typedef const ASSEMBLY_FILE_DETAILED_INFORMATION *PCASSEMBLY_FILE_DETAILED_INFORMATION;

//
// compatibility with old names
// The new names use "file" consistently.
//
#define  _ASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION  _ASSEMBLY_FILE_DETAILED_INFORMATION
#define   ASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION   ASSEMBLY_FILE_DETAILED_INFORMATION
#define  PASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION  PASSEMBLY_FILE_DETAILED_INFORMATION
#define PCASSEMBLY_DLL_REDIRECTION_DETAILED_INFORMATION PCASSEMBLY_FILE_DETAILED_INFORMATION

typedef struct _ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    DWORD ulFlags;
    DWORD ulEncodedAssemblyIdentityLength;      // in bytes
    DWORD ulManifestPathType;                   // ACTIVATION_CONTEXT_PATH_TYPE_*
    DWORD ulManifestPathLength;                 // in bytes
    LARGE_INTEGER liManifestLastWriteTime;      // FILETIME
    DWORD ulPolicyPathType;                     // ACTIVATION_CONTEXT_PATH_TYPE_*
    DWORD ulPolicyPathLength;                   // in bytes
    LARGE_INTEGER liPolicyLastWriteTime;        // FILETIME
    DWORD ulMetadataSatelliteRosterIndex;

    DWORD ulManifestVersionMajor;               // 1
    DWORD ulManifestVersionMinor;               // 0
    DWORD ulPolicyVersionMajor;                 // 0
    DWORD ulPolicyVersionMinor;                 // 0
    DWORD ulAssemblyDirectoryNameLength;        // in bytes

    PCWSTR lpAssemblyEncodedAssemblyIdentity;
    PCWSTR lpAssemblyManifestPath;
    PCWSTR lpAssemblyPolicyPath;
    PCWSTR lpAssemblyDirectoryName;

    DWORD  ulFileCount;
} ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION, * PACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION;

typedef const struct _ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION * PCACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION ;

typedef enum
{
    ACTCTX_RUN_LEVEL_UNSPECIFIED = 0,
    ACTCTX_RUN_LEVEL_AS_INVOKER,
    ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE,
    ACTCTX_RUN_LEVEL_REQUIRE_ADMIN,
    ACTCTX_RUN_LEVEL_NUMBERS
} ACTCTX_REQUESTED_RUN_LEVEL;

typedef struct _ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    DWORD ulFlags;
    ACTCTX_REQUESTED_RUN_LEVEL  RunLevel;
    DWORD UiAccess;
} ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION, * PACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION;

typedef const struct _ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION * PCACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION ;

typedef enum
{
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN = 0,
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS,
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION,
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED
} ACTCTX_COMPATIBILITY_ELEMENT_TYPE;

typedef struct _COMPATIBILITY_CONTEXT_ELEMENT {
    GUID Id;
    ACTCTX_COMPATIBILITY_ELEMENT_TYPE Type;
    ULONGLONG MaxVersionTested;
} COMPATIBILITY_CONTEXT_ELEMENT, *PCOMPATIBILITY_CONTEXT_ELEMENT;

typedef const struct _COMPATIBILITY_CONTEXT_ELEMENT *PCCOMPATIBILITY_CONTEXT_ELEMENT;

#ifdef _MSC_EXTENSIONS

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4200) // zero length array
#endif

typedef struct _ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    DWORD ElementCount;
    COMPATIBILITY_CONTEXT_ELEMENT Elements[];
} ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION, * PACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION;

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

typedef const struct _ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION * PCACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION;

#endif

typedef struct _SUPPORTED_OS_INFO {
    WORD   MajorVersion;
    WORD   MinorVersion;
} SUPPORTED_OS_INFO, *PSUPPORTED_OS_INFO;

typedef struct _MAXVERSIONTESTED_INFO {
    ULONGLONG MaxVersionTested;
} MAXVERSIONTESTED_INFO, *PMAXVERSIONTESTED_INFO;

typedef struct _ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    DWORD dwFlags;
    DWORD ulFormatVersion;
    DWORD ulAssemblyCount;
    DWORD ulRootManifestPathType;
    DWORD ulRootManifestPathChars;
    DWORD ulRootConfigurationPathType;
    DWORD ulRootConfigurationPathChars;
    DWORD ulAppDirPathType;
    DWORD ulAppDirPathChars;
    PCWSTR lpRootManifestPath;
    PCWSTR lpRootConfigurationPath;
    PCWSTR lpAppDirPath;
} ACTIVATION_CONTEXT_DETAILED_INFORMATION, *PACTIVATION_CONTEXT_DETAILED_INFORMATION;

typedef const struct _ACTIVATION_CONTEXT_DETAILED_INFORMATION *PCACTIVATION_CONTEXT_DETAILED_INFORMATION;


#define CREATE_BOUNDARY_DESCRIPTOR_ADD_APPCONTAINER_SID 0x1

typedef struct _HARDWARE_COUNTER_DATA {
    HARDWARE_COUNTER_TYPE Type;
    DWORD Reserved;
    DWORD64 Value;
} HARDWARE_COUNTER_DATA, *PHARDWARE_COUNTER_DATA;

#define PERFORMANCE_DATA_VERSION 1

typedef struct _PERFORMANCE_DATA {
    WORD   Size;
    BYTE  Version;
    BYTE  HwCountersCount;
    DWORD ContextSwitchCount;
    DWORD64 WaitReasonBitMap;
    DWORD64 CycleTime;
    DWORD RetryCount;
    DWORD Reserved;
    HARDWARE_COUNTER_DATA HwCounters[MAX_HW_COUNTERS];
} PERFORMANCE_DATA, *PPERFORMANCE_DATA;

#define READ_THREAD_PROFILING_FLAG_DISPATCHING        0x00000001
#define READ_THREAD_PROFILING_FLAG_HARDWARE_COUNTERS  0x00000002

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#define UNIFIEDBUILDREVISION_KEY                        L"\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion"
#define UNIFIEDBUILDREVISION_VALUE                      L"UBR"
#define UNIFIEDBUILDREVISION_MIN                        0x00000000

#define DEVICEFAMILYDEVICEFORM_KEY                      L"\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion\\OEM"
#define DEVICEFAMILYDEVICEFORM_VALUE                    L"DeviceForm"

#define DEVICEFAMILYINFOENUM_UAP                        0x00000000
#define DEVICEFAMILYINFOENUM_WINDOWS_8X                 0x00000001
#define DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X           0x00000002
#define DEVICEFAMILYINFOENUM_DESKTOP                    0x00000003
#define DEVICEFAMILYINFOENUM_MOBILE                     0x00000004
#define DEVICEFAMILYINFOENUM_XBOX                       0x00000005
#define DEVICEFAMILYINFOENUM_TEAM                       0x00000006
#define DEVICEFAMILYINFOENUM_IOT                        0x00000007
#define DEVICEFAMILYINFOENUM_IOT_HEADLESS               0x00000008
#define DEVICEFAMILYINFOENUM_SERVER                     0x00000009
#define DEVICEFAMILYINFOENUM_HOLOGRAPHIC                0x0000000A
#define DEVICEFAMILYINFOENUM_XBOXSRA                    0x0000000B
#define DEVICEFAMILYINFOENUM_XBOXERA                    0x0000000C
#define DEVICEFAMILYINFOENUM_SERVER_NANO                0x0000000D
#define DEVICEFAMILYINFOENUM_8828080                    0x0000000E
#define DEVICEFAMILYINFOENUM_7067329                    0x0000000F
#define DEVICEFAMILYINFOENUM_WINDOWS_CORE               0x00000010
#define DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS      0x00000011

#define DEVICEFAMILYINFOENUM_MAX                        0x00000011

#define DEVICEFAMILYDEVICEFORM_UNKNOWN                  0x00000000
#define DEVICEFAMILYDEVICEFORM_PHONE                    0x00000001
#define DEVICEFAMILYDEVICEFORM_TABLET                   0x00000002
#define DEVICEFAMILYDEVICEFORM_DESKTOP                  0x00000003
#define DEVICEFAMILYDEVICEFORM_NOTEBOOK                 0x00000004
#define DEVICEFAMILYDEVICEFORM_CONVERTIBLE              0x00000005
#define DEVICEFAMILYDEVICEFORM_DETACHABLE               0x00000006
#define DEVICEFAMILYDEVICEFORM_ALLINONE                 0x00000007
#define DEVICEFAMILYDEVICEFORM_STICKPC                  0x00000008
#define DEVICEFAMILYDEVICEFORM_PUCK                     0x00000009
#define DEVICEFAMILYDEVICEFORM_LARGESCREEN              0x0000000A
#define DEVICEFAMILYDEVICEFORM_HMD                      0x0000000B
#define DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD        0x0000000C
#define DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET          0x0000000D
#define DEVICEFAMILYDEVICEFORM_BANKING                  0x0000000E
#define DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION      0x0000000F
#define DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE          0x00000010
#define DEVICEFAMILYDEVICEFORM_GAMING                   0x00000011
#define DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION          0x00000012
#define DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION    0x00000013
#define DEVICEFAMILYDEVICEFORM_KIOSK                    0x00000014
#define DEVICEFAMILYDEVICEFORM_MAKER_BOARD              0x00000015
#define DEVICEFAMILYDEVICEFORM_MEDICAL                  0x00000016
#define DEVICEFAMILYDEVICEFORM_NETWORKING               0x00000017
#define DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE         0x00000018
#define DEVICEFAMILYDEVICEFORM_PRINTING                 0x00000019
#define DEVICEFAMILYDEVICEFORM_THIN_CLIENT              0x0000001A
#define DEVICEFAMILYDEVICEFORM_TOY                      0x0000001B
#define DEVICEFAMILYDEVICEFORM_VENDING                  0x0000001C
#define DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER           0x0000001D
#define DEVICEFAMILYDEVICEFORM_XBOX_ONE                 0x0000001E
#define DEVICEFAMILYDEVICEFORM_XBOX_ONE_S               0x0000001F
#define DEVICEFAMILYDEVICEFORM_XBOX_ONE_X               0x00000020
#define DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT        0x00000021
#define DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X            0x00000022
#define DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT     0x00000023
#define DEVICEFAMILYDEVICEFORM_XBOX_SERIES_S            0x00000024

// This is a range reserved for future Xbox consoles.
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01         0x00000025
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02         0x00000026
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03         0x00000027
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04         0x00000028
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05         0x00000029
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06         0x0000002A
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07         0x0000002B
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08         0x0000002C
#define DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09         0x0000002D

#define DEVICEFAMILYDEVICEFORM_GAMING_HANDHELD          0x0000002E
#define DEVICEFAMILYDEVICEFORM_GAMING_CONSOLE           0x0000002F

#define DEVICEFAMILYDEVICEFORM_MAX                      0x0000002F

VOID
NTAPI
RtlGetDeviceFamilyInfoEnum(
    _Out_opt_ ULONGLONG *pullUAPInfo,
    _Out_opt_ DWORD *pulDeviceFamily,
    _Out_opt_ DWORD *pulDeviceForm
);

DWORD   
NTAPI
RtlConvertDeviceFamilyInfoToString(
    _Inout_ PDWORD pulDeviceFamilyBufferSize,
    _Inout_ PDWORD pulDeviceFormBufferSize,
    _Out_writes_bytes_(*pulDeviceFamilyBufferSize) PWSTR DeviceFamily,
    _Out_writes_bytes_(*pulDeviceFormBufferSize) PWSTR DeviceForm

);

DWORD   
NTAPI
RtlSwitchedVVI(
    _In_ PRTL_OSVERSIONINFOEXW VersionInfo,
    _In_ DWORD TypeMask,
    _In_ ULONGLONG  ConditionMask
    );


#endif // (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma endregion

#define DLL_PROCESS_ATTACH   1    
#define DLL_THREAD_ATTACH    2    
#define DLL_THREAD_DETACH    3    
#define DLL_PROCESS_DETACH   0    

//
// Defines for the READ flags for Eventlogging
//
#define EVENTLOG_SEQUENTIAL_READ        0x0001
#define EVENTLOG_SEEK_READ              0x0002
#define EVENTLOG_FORWARDS_READ          0x0004
#define EVENTLOG_BACKWARDS_READ         0x0008

//
// The types of events that can be logged.
//
#define EVENTLOG_SUCCESS                0x0000
#define EVENTLOG_ERROR_TYPE             0x0001
#define EVENTLOG_WARNING_TYPE           0x0002
#define EVENTLOG_INFORMATION_TYPE       0x0004
#define EVENTLOG_AUDIT_SUCCESS          0x0008
#define EVENTLOG_AUDIT_FAILURE          0x0010

//
// Defines for the WRITE flags used by Auditing for paired events
// These are not implemented in Product 1
//

#define EVENTLOG_START_PAIRED_EVENT    0x0001
#define EVENTLOG_END_PAIRED_EVENT      0x0002
#define EVENTLOG_END_ALL_PAIRED_EVENTS 0x0004
#define EVENTLOG_PAIRED_EVENT_ACTIVE   0x0008
#define EVENTLOG_PAIRED_EVENT_INACTIVE 0x0010

//
// Structure that defines the header of the Eventlog record. This is the
// fixed-sized portion before all the variable-length strings, binary
// data and pad bytes.
//
// TimeGenerated and TimeWritten are the time the event was put into the log at the server end.
//

typedef struct _EVENTLOGRECORD {
    DWORD  Length;        // Length of full record
    DWORD  Reserved;      // Used by the service
    DWORD  RecordNumber;  // Absolute record number
    DWORD  TimeGenerated; // Seconds since 1-1-1970
    DWORD  TimeWritten;   // Seconds since 1-1-1970
    DWORD  EventID;
    WORD   EventType;
    WORD   NumStrings;
    WORD   EventCategory;
    WORD   ReservedFlags;
    DWORD  ClosingRecordNumber; // Reserved
    DWORD  StringOffset;  // Offset from beginning of record
    DWORD  UserSidLength;
    DWORD  UserSidOffset;
    DWORD  DataLength;
    DWORD  DataOffset;    // Offset from beginning of record
    //
    // Then follow:
    //
    // WCHAR SourceName[]
    // WCHAR Computername[]
    // SID   UserSid
    // WCHAR Strings[]
    // BYTE  Data[]
    // CHAR  Pad[]
    // DWORD Length;
    //
} EVENTLOGRECORD, *PEVENTLOGRECORD;

#define MAXLOGICALLOGNAMESIZE   256

struct _EVENTSFORLOGFILE;
typedef struct _EVENTSFORLOGFILE EVENTSFORLOGFILE, *PEVENTSFORLOGFILE;

struct _PACKEDEVENTINFO;
typedef struct _PACKEDEVENTINFO PACKEDEVENTINFO, *PPACKEDEVENTINFO;

#if defined(_MSC_EXTENSIONS)

#pragma warning(push)
#pragma warning(disable : 4200) /* nonstandard extension used : zero-sized array in struct/union */

struct
#if (_MSC_VER > 1310) && !defined(MIDL_PASS)
 __declspec(deprecated("struct EVENTSFORLOGFILE is deprecated and might not work on all platforms. For more info, see MSDN."))
 #endif
 _EVENTSFORLOGFILE {
    DWORD           ulSize;
    WCHAR           szLogicalLogFile[MAXLOGICALLOGNAMESIZE];        //name of the logical file-security/application/system
    DWORD           ulNumRecords;
    EVENTLOGRECORD  pEventLogRecords[];
};

struct
#if (_MSC_VER > 1310) && !defined(MIDL_PASS)
__declspec(deprecated("struct PACKEDEVENTINFO is deprecated and might not work on all platforms. For more info, see MSDN."))
#endif
_PACKEDEVENTINFO {
    DWORD               ulSize;  //total size of the structure
    DWORD               ulNumEventsForLogFile; //number of EventsForLogFile structure that follow
    DWORD               ulOffsets[];           //the offsets from the start of this structure to the EVENTSFORLOGFILE structure
};

#pragma warning(pop)

#endif
//

// begin_wdm
// begin_access

//
// Registry Specific Access Rights.
//

#define KEY_QUERY_VALUE         (0x0001)
#define KEY_SET_VALUE           (0x0002)
#define KEY_CREATE_SUB_KEY      (0x0004)
#define KEY_ENUMERATE_SUB_KEYS  (0x0008)
#define KEY_NOTIFY              (0x0010)
#define KEY_CREATE_LINK         (0x0020)
#define KEY_WOW64_32KEY         (0x0200)
#define KEY_WOW64_64KEY         (0x0100)
#define KEY_WOW64_RES           (0x0300)

#define KEY_READ                ((STANDARD_RIGHTS_READ       |\
                                  KEY_QUERY_VALUE            |\
                                  KEY_ENUMERATE_SUB_KEYS     |\
                                  KEY_NOTIFY)                 \
                                  &                           \
                                 (~SYNCHRONIZE))


#define KEY_WRITE               ((STANDARD_RIGHTS_WRITE      |\
                                  KEY_SET_VALUE              |\
                                  KEY_CREATE_SUB_KEY)         \
                                  &                           \
                                 (~SYNCHRONIZE))

#define KEY_EXECUTE             ((KEY_READ)                   \
                                  &                           \
                                 (~SYNCHRONIZE))

#define KEY_ALL_ACCESS          ((STANDARD_RIGHTS_ALL        |\
                                  KEY_QUERY_VALUE            |\
                                  KEY_SET_VALUE              |\
                                  KEY_CREATE_SUB_KEY         |\
                                  KEY_ENUMERATE_SUB_KEYS     |\
                                  KEY_NOTIFY                 |\
                                  KEY_CREATE_LINK)            \
                                  &                           \
                                 (~SYNCHRONIZE))

// end_access

//
// Open/Create Options
//

#define REG_OPTION_RESERVED             (0x00000000L)   // Parameter is reserved

#define REG_OPTION_NON_VOLATILE         (0x00000000L)   // Key is preserved
                                                        // when system is rebooted

#define REG_OPTION_VOLATILE             (0x00000001L)   // Key is not preserved
                                                        // when system is rebooted

#define REG_OPTION_CREATE_LINK          (0x00000002L)   // Created key is a
                                                        // symbolic link

#define REG_OPTION_BACKUP_RESTORE       (0x00000004L)   // open for backup or restore
                                                        // special access rules
                                                        // privilege required

#define REG_OPTION_OPEN_LINK            (0x00000008L)   // Open symbolic link

#define REG_OPTION_DONT_VIRTUALIZE      (0x00000010L)   // Disable Open/Read/Write
                                                        // virtualization for this
                                                        // open and the resulting
                                                        // handle.

#define REG_LEGAL_OPTION            \
                (REG_OPTION_RESERVED            |\
                 REG_OPTION_NON_VOLATILE        |\
                 REG_OPTION_VOLATILE            |\
                 REG_OPTION_CREATE_LINK         |\
                 REG_OPTION_BACKUP_RESTORE      |\
                 REG_OPTION_OPEN_LINK           |\
                 REG_OPTION_DONT_VIRTUALIZE)

#define REG_OPEN_LEGAL_OPTION       \
                (REG_OPTION_RESERVED            |\
                 REG_OPTION_BACKUP_RESTORE      |\
                 REG_OPTION_OPEN_LINK           |\
                 REG_OPTION_DONT_VIRTUALIZE)

//
// Key creation/open disposition
//

#define REG_CREATED_NEW_KEY         (0x00000001L)   // New Registry Key created
#define REG_OPENED_EXISTING_KEY     (0x00000002L)   // Existing Key opened

//
// hive format to be used by Reg(Nt)SaveKeyEx
//
#define REG_STANDARD_FORMAT     1
#define REG_LATEST_FORMAT       2
#define REG_NO_COMPRESSION      4

//
// Key restore & hive load flags
//

#define REG_WHOLE_HIVE_VOLATILE         (0x00000001L)   // Restore whole hive volatile
#define REG_REFRESH_HIVE                (0x00000002L)   // Unwind changes to last flush
#define REG_NO_LAZY_FLUSH               (0x00000004L)   // Never lazy flush this hive
#define REG_FORCE_RESTORE               (0x00000008L)   // Force the restore process even when we have open handles on subkeys
#define REG_APP_HIVE                    (0x00000010L)   // Loads the hive visible to the calling process
#define REG_PROCESS_PRIVATE             (0x00000020L)   // Hive cannot be mounted by any other process while in use
#define REG_START_JOURNAL               (0x00000040L)   // Starts Hive Journal
#define REG_HIVE_EXACT_FILE_GROWTH      (0x00000080L)   // Grow hive file in exact 4k increments
#define REG_HIVE_NO_RM                  (0x00000100L)   // No RM is started for this hive (no transactions)
#define REG_HIVE_SINGLE_LOG             (0x00000200L)   // Legacy single logging is used for this hive
#define REG_BOOT_HIVE                   (0x00000400L)   // This hive might be used by the OS loader
#define REG_LOAD_HIVE_OPEN_HANDLE       (0x00000800L)   // Load the hive and return a handle to its root kcb
#define REG_FLUSH_HIVE_FILE_GROWTH      (0x00001000L)   // Flush changes to primary hive file size as part of all flushes
#define REG_OPEN_READ_ONLY              (0x00002000L)   // Open a hive's files in read-only mode
#define REG_IMMUTABLE                   (0x00004000L)   // Load the hive, but don't allow any modification of it
#define REG_NO_IMPERSONATION_FALLBACK   (0x00008000L)   // Do not fall back to impersonating the caller if hive file access fails
#define REG_APP_HIVE_OPEN_READ_ONLY     (REG_OPEN_READ_ONLY)   // Open an app hive's files in read-only mode (if the hive was not previously loaded)

//
// Unload Flags
//
#define REG_FORCE_UNLOAD            1
#define REG_UNLOAD_LEGAL_FLAGS      (REG_FORCE_UNLOAD)

//
// Notify filter values
//

#define REG_NOTIFY_CHANGE_NAME          (0x00000001L) // Create or delete (child)
#define REG_NOTIFY_CHANGE_ATTRIBUTES    (0x00000002L)
#define REG_NOTIFY_CHANGE_LAST_SET      (0x00000004L) // time stamp
#define REG_NOTIFY_CHANGE_SECURITY      (0x00000008L)
#define REG_NOTIFY_THREAD_AGNOSTIC      (0x10000000L) // Not associated with a calling thread, can only be used
                                                      // for async user event based notification

#define REG_LEGAL_CHANGE_FILTER                 \
                (REG_NOTIFY_CHANGE_NAME          |\
                 REG_NOTIFY_CHANGE_ATTRIBUTES    |\
                 REG_NOTIFY_CHANGE_LAST_SET      |\
                 REG_NOTIFY_CHANGE_SECURITY      |\
                 REG_NOTIFY_THREAD_AGNOSTIC)

// end_wdm

//
//
// Predefined Value Types.
//

#define REG_NONE                    ( 0ul ) // No value type
#define REG_SZ                      ( 1ul ) // Unicode nul terminated string
#define REG_EXPAND_SZ               ( 2ul ) // Unicode nul terminated string
                                            // (with environment variable references)
#define REG_BINARY                  ( 3ul ) // Free form binary
#define REG_DWORD                   ( 4ul ) // 32-bit number
#define REG_DWORD_LITTLE_ENDIAN     ( 4ul ) // 32-bit number (same as REG_DWORD)
#define REG_DWORD_BIG_ENDIAN        ( 5ul ) // 32-bit number
#define REG_LINK                    ( 6ul ) // Symbolic Link (unicode)
#define REG_MULTI_SZ                ( 7ul ) // Multiple Unicode strings
#define REG_RESOURCE_LIST           ( 8ul ) // Resource list in the resource map
#define REG_FULL_RESOURCE_DESCRIPTOR ( 9ul ) // Resource list in the hardware description
#define REG_RESOURCE_REQUIREMENTS_LIST ( 10ul )
#define REG_QWORD                   ( 11ul ) // 64-bit number
#define REG_QWORD_LITTLE_ENDIAN     ( 11ul ) // 64-bit number (same as REG_QWORD)

// end_wdm

// begin_wdm
//
// Service Types (Bit Mask)
//
#define SERVICE_KERNEL_DRIVER          0x00000001
#define SERVICE_FILE_SYSTEM_DRIVER     0x00000002
#define SERVICE_ADAPTER                0x00000004
#define SERVICE_RECOGNIZER_DRIVER      0x00000008

#define SERVICE_DRIVER                 (SERVICE_KERNEL_DRIVER | \
                                        SERVICE_FILE_SYSTEM_DRIVER | \
                                        SERVICE_RECOGNIZER_DRIVER)

#define SERVICE_WIN32_OWN_PROCESS      0x00000010
#define SERVICE_WIN32_SHARE_PROCESS    0x00000020
#define SERVICE_WIN32                  (SERVICE_WIN32_OWN_PROCESS | \
                                        SERVICE_WIN32_SHARE_PROCESS)

#define SERVICE_USER_SERVICE           0x00000040
#define SERVICE_USERSERVICE_INSTANCE   0x00000080

#define SERVICE_USER_SHARE_PROCESS     (SERVICE_USER_SERVICE | \
                                        SERVICE_WIN32_SHARE_PROCESS)
#define SERVICE_USER_OWN_PROCESS       (SERVICE_USER_SERVICE | \
                                        SERVICE_WIN32_OWN_PROCESS)

#define SERVICE_INTERACTIVE_PROCESS    0x00000100
#define SERVICE_PKG_SERVICE            0x00000200

#define SERVICE_TYPE_ALL               (SERVICE_WIN32  | \
                                        SERVICE_ADAPTER | \
                                        SERVICE_DRIVER  | \
                                        SERVICE_INTERACTIVE_PROCESS | \
                                        SERVICE_USER_SERVICE | \
                                        SERVICE_USERSERVICE_INSTANCE | \
                                        SERVICE_PKG_SERVICE)

//
// Start Type
//

#define SERVICE_BOOT_START             0x00000000
#define SERVICE_SYSTEM_START           0x00000001
#define SERVICE_AUTO_START             0x00000002
#define SERVICE_DEMAND_START           0x00000003
#define SERVICE_DISABLED               0x00000004

//
// Error control type
//
#define SERVICE_ERROR_IGNORE           0x00000000
#define SERVICE_ERROR_NORMAL           0x00000001
#define SERVICE_ERROR_SEVERE           0x00000002
#define SERVICE_ERROR_CRITICAL         0x00000003

//
//
// Define the registry driver node enumerations
//

typedef enum _CM_SERVICE_NODE_TYPE {
    DriverType               = SERVICE_KERNEL_DRIVER,
    FileSystemType           = SERVICE_FILE_SYSTEM_DRIVER,
    Win32ServiceOwnProcess   = SERVICE_WIN32_OWN_PROCESS,
    Win32ServiceShareProcess = SERVICE_WIN32_SHARE_PROCESS,
    AdapterType              = SERVICE_ADAPTER,
    RecognizerType           = SERVICE_RECOGNIZER_DRIVER
} SERVICE_NODE_TYPE;

typedef enum _CM_SERVICE_LOAD_TYPE {
    BootLoad    = SERVICE_BOOT_START,
    SystemLoad  = SERVICE_SYSTEM_START,
    AutoLoad    = SERVICE_AUTO_START,
    DemandLoad  = SERVICE_DEMAND_START,
    DisableLoad = SERVICE_DISABLED
} SERVICE_LOAD_TYPE;

typedef enum _CM_ERROR_CONTROL_TYPE {
    IgnoreError   = SERVICE_ERROR_IGNORE,
    NormalError   = SERVICE_ERROR_NORMAL,
    SevereError   = SERVICE_ERROR_SEVERE,
    CriticalError = SERVICE_ERROR_CRITICAL
} SERVICE_ERROR_TYPE;

//
// Service node Flags. These flags are used by the OS loader to promote
// a driver's start type to boot start if the system is booting using
// the specified mechanism. The flags should be set in the driver's
// registry configuration.
//
// CM_SERVICE_NETWORK_BOOT_LOAD - Specified if a driver should be
// promoted on network boot.
//
// CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD - Specified if a driver should be
// promoted on booting from a VHD.
//
// CM_SERVICE_USB_DISK_BOOT_LOAD - Specified if a driver should be promoted
// while booting from a USB disk.
//
// CM_SERVICE_SD_DISK_BOOT_LOAD - Specified if a driver should be promoted
// while booting from SD storage.
//
// CM_SERVICE_USB3_DISK_BOOT_LOAD - Specified if a driver should be promoted
// while booting from a disk on a USB3 controller.
//
// CM_SERVICE_MEASURED_BOOT_LOAD - Specified if a driver should be promoted
// while booting with measured boot enabled.
//
// CM_SERVICE_VERIFIER_BOOT_LOAD - Specified if a driver should be promoted
// while booting with verifier boot enabled.
//
// CM_SERVICE_WINPE_BOOT_LOAD - Specified if a driver should be promoted
// on WinPE boot.
//
// CM_SERVICE_RAM_DISK_BOOT_LOAD - Specified if a driver should be promoted
// when booting from a RAM disk.
//

#define CM_SERVICE_NETWORK_BOOT_LOAD      0x00000001
#define CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD 0x00000002
#define CM_SERVICE_USB_DISK_BOOT_LOAD     0x00000004
#define CM_SERVICE_SD_DISK_BOOT_LOAD      0x00000008
#define CM_SERVICE_USB3_DISK_BOOT_LOAD    0x00000010
#define CM_SERVICE_MEASURED_BOOT_LOAD     0x00000020
#define CM_SERVICE_VERIFIER_BOOT_LOAD     0x00000040
#define CM_SERVICE_WINPE_BOOT_LOAD        0x00000080
#define CM_SERVICE_RAM_DISK_BOOT_LOAD     0x00000100

//
// Mask defining the legal promotion flag values.
//

#define CM_SERVICE_VALID_PROMOTION_MASK (CM_SERVICE_NETWORK_BOOT_LOAD |       \
                                         CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD |  \
                                         CM_SERVICE_USB_DISK_BOOT_LOAD |      \
                                         CM_SERVICE_SD_DISK_BOOT_LOAD |       \
                                         CM_SERVICE_USB3_DISK_BOOT_LOAD |     \
                                         CM_SERVICE_MEASURED_BOOT_LOAD |      \
                                         CM_SERVICE_VERIFIER_BOOT_LOAD |      \
                                         CM_SERVICE_WINPE_BOOT_LOAD |         \
                                         CM_SERVICE_RAM_DISK_BOOT_LOAD)


#ifndef _NTDDTAPE_WINNT_
#define _NTDDTAPE_WINNT_

//
// IOCTL_TAPE_ERASE definitions
//

#define TAPE_ERASE_SHORT            0L
#define TAPE_ERASE_LONG             1L

typedef struct _TAPE_ERASE {
    DWORD Type;
    BOOLEAN Immediate;
} TAPE_ERASE, *PTAPE_ERASE;

//
// IOCTL_TAPE_PREPARE definitions
//

#define TAPE_LOAD                   0L
#define TAPE_UNLOAD                 1L
#define TAPE_TENSION                2L
#define TAPE_LOCK                   3L
#define TAPE_UNLOCK                 4L
#define TAPE_FORMAT                 5L

typedef struct _TAPE_PREPARE {
    DWORD Operation;
    BOOLEAN Immediate;
} TAPE_PREPARE, *PTAPE_PREPARE;

//
// IOCTL_TAPE_WRITE_MARKS definitions
//

#define TAPE_SETMARKS               0L
#define TAPE_FILEMARKS              1L
#define TAPE_SHORT_FILEMARKS        2L
#define TAPE_LONG_FILEMARKS         3L

typedef struct _TAPE_WRITE_MARKS {
    DWORD Type;
    DWORD Count;
    BOOLEAN Immediate;
} TAPE_WRITE_MARKS, *PTAPE_WRITE_MARKS;

//
// IOCTL_TAPE_GET_POSITION definitions
//

#define TAPE_ABSOLUTE_POSITION       0L
#define TAPE_LOGICAL_POSITION        1L
#define TAPE_PSEUDO_LOGICAL_POSITION 2L

typedef struct _TAPE_GET_POSITION {
    DWORD Type;
    DWORD Partition;
    LARGE_INTEGER Offset;
} TAPE_GET_POSITION, *PTAPE_GET_POSITION;

//
// IOCTL_TAPE_SET_POSITION definitions
//

#define TAPE_REWIND                 0L
#define TAPE_ABSOLUTE_BLOCK         1L
#define TAPE_LOGICAL_BLOCK          2L
#define TAPE_PSEUDO_LOGICAL_BLOCK   3L
#define TAPE_SPACE_END_OF_DATA      4L
#define TAPE_SPACE_RELATIVE_BLOCKS  5L
#define TAPE_SPACE_FILEMARKS        6L
#define TAPE_SPACE_SEQUENTIAL_FMKS  7L
#define TAPE_SPACE_SETMARKS         8L
#define TAPE_SPACE_SEQUENTIAL_SMKS  9L

typedef struct _TAPE_SET_POSITION {
    DWORD Method;
    DWORD Partition;
    LARGE_INTEGER Offset;
    BOOLEAN Immediate;
} TAPE_SET_POSITION, *PTAPE_SET_POSITION;

//
// IOCTL_TAPE_GET_DRIVE_PARAMS definitions
//

//
// Definitions for FeaturesLow parameter
//

#define TAPE_DRIVE_FIXED            0x00000001
#define TAPE_DRIVE_SELECT           0x00000002
#define TAPE_DRIVE_INITIATOR        0x00000004

#define TAPE_DRIVE_ERASE_SHORT      0x00000010
#define TAPE_DRIVE_ERASE_LONG       0x00000020
#define TAPE_DRIVE_ERASE_BOP_ONLY   0x00000040
#define TAPE_DRIVE_ERASE_IMMEDIATE  0x00000080

#define TAPE_DRIVE_TAPE_CAPACITY    0x00000100
#define TAPE_DRIVE_TAPE_REMAINING   0x00000200
#define TAPE_DRIVE_FIXED_BLOCK      0x00000400
#define TAPE_DRIVE_VARIABLE_BLOCK   0x00000800

#define TAPE_DRIVE_WRITE_PROTECT    0x00001000
#define TAPE_DRIVE_EOT_WZ_SIZE      0x00002000

#define TAPE_DRIVE_ECC              0x00010000
#define TAPE_DRIVE_COMPRESSION      0x00020000
#define TAPE_DRIVE_PADDING          0x00040000
#define TAPE_DRIVE_REPORT_SMKS      0x00080000

#define TAPE_DRIVE_GET_ABSOLUTE_BLK 0x00100000
#define TAPE_DRIVE_GET_LOGICAL_BLK  0x00200000
#define TAPE_DRIVE_SET_EOT_WZ_SIZE  0x00400000

#define TAPE_DRIVE_EJECT_MEDIA      0x01000000
#define TAPE_DRIVE_CLEAN_REQUESTS   0x02000000
#define TAPE_DRIVE_SET_CMP_BOP_ONLY 0x04000000

#define TAPE_DRIVE_RESERVED_BIT     0x80000000  //don't use this bit!
//                                              //can't be a low features bit!
//                                              //reserved; high features only

//
// Definitions for FeaturesHigh parameter
//

#define TAPE_DRIVE_LOAD_UNLOAD      0x80000001
#define TAPE_DRIVE_TENSION          0x80000002
#define TAPE_DRIVE_LOCK_UNLOCK      0x80000004
#define TAPE_DRIVE_REWIND_IMMEDIATE 0x80000008

#define TAPE_DRIVE_SET_BLOCK_SIZE   0x80000010
#define TAPE_DRIVE_LOAD_UNLD_IMMED  0x80000020
#define TAPE_DRIVE_TENSION_IMMED    0x80000040
#define TAPE_DRIVE_LOCK_UNLK_IMMED  0x80000080

#define TAPE_DRIVE_SET_ECC          0x80000100
#define TAPE_DRIVE_SET_COMPRESSION  0x80000200
#define TAPE_DRIVE_SET_PADDING      0x80000400
#define TAPE_DRIVE_SET_REPORT_SMKS  0x80000800

#define TAPE_DRIVE_ABSOLUTE_BLK     0x80001000
#define TAPE_DRIVE_ABS_BLK_IMMED    0x80002000
#define TAPE_DRIVE_LOGICAL_BLK      0x80004000
#define TAPE_DRIVE_LOG_BLK_IMMED    0x80008000

#define TAPE_DRIVE_END_OF_DATA      0x80010000
#define TAPE_DRIVE_RELATIVE_BLKS    0x80020000
#define TAPE_DRIVE_FILEMARKS        0x80040000
#define TAPE_DRIVE_SEQUENTIAL_FMKS  0x80080000

#define TAPE_DRIVE_SETMARKS         0x80100000
#define TAPE_DRIVE_SEQUENTIAL_SMKS  0x80200000
#define TAPE_DRIVE_REVERSE_POSITION 0x80400000
#define TAPE_DRIVE_SPACE_IMMEDIATE  0x80800000

#define TAPE_DRIVE_WRITE_SETMARKS   0x81000000
#define TAPE_DRIVE_WRITE_FILEMARKS  0x82000000
#define TAPE_DRIVE_WRITE_SHORT_FMKS 0x84000000
#define TAPE_DRIVE_WRITE_LONG_FMKS  0x88000000

#define TAPE_DRIVE_WRITE_MARK_IMMED 0x90000000
#define TAPE_DRIVE_FORMAT           0xA0000000
#define TAPE_DRIVE_FORMAT_IMMEDIATE 0xC0000000
#define TAPE_DRIVE_HIGH_FEATURES    0x80000000  //mask for high features flag

typedef struct _TAPE_GET_DRIVE_PARAMETERS {
    BOOLEAN ECC;
    BOOLEAN Compression;
    BOOLEAN DataPadding;
    BOOLEAN ReportSetmarks;
    DWORD DefaultBlockSize;
    DWORD MaximumBlockSize;
    DWORD MinimumBlockSize;
    DWORD MaximumPartitionCount;
    DWORD FeaturesLow;
    DWORD FeaturesHigh;
    DWORD EOTWarningZoneSize;
} TAPE_GET_DRIVE_PARAMETERS, *PTAPE_GET_DRIVE_PARAMETERS;

//
// IOCTL_TAPE_SET_DRIVE_PARAMETERS definitions
//

typedef struct _TAPE_SET_DRIVE_PARAMETERS {
    BOOLEAN ECC;
    BOOLEAN Compression;
    BOOLEAN DataPadding;
    BOOLEAN ReportSetmarks;
    DWORD EOTWarningZoneSize;
} TAPE_SET_DRIVE_PARAMETERS, *PTAPE_SET_DRIVE_PARAMETERS;

//
// IOCTL_TAPE_GET_MEDIA_PARAMETERS definitions
//

typedef struct _TAPE_GET_MEDIA_PARAMETERS {
    LARGE_INTEGER Capacity;
    LARGE_INTEGER Remaining;
    DWORD BlockSize;
    DWORD PartitionCount;
    BOOLEAN WriteProtected;
} TAPE_GET_MEDIA_PARAMETERS, *PTAPE_GET_MEDIA_PARAMETERS;

//
// IOCTL_TAPE_SET_MEDIA_PARAMETERS definitions
//

typedef struct _TAPE_SET_MEDIA_PARAMETERS {
    DWORD BlockSize;
} TAPE_SET_MEDIA_PARAMETERS, *PTAPE_SET_MEDIA_PARAMETERS;

//
// IOCTL_TAPE_CREATE_PARTITION definitions
//

#define TAPE_FIXED_PARTITIONS       0L
#define TAPE_SELECT_PARTITIONS      1L
#define TAPE_INITIATOR_PARTITIONS   2L

typedef struct _TAPE_CREATE_PARTITION {
    DWORD Method;
    DWORD Count;
    DWORD Size;
} TAPE_CREATE_PARTITION, *PTAPE_CREATE_PARTITION;


//
// WMI Methods
//
#define TAPE_QUERY_DRIVE_PARAMETERS       0L
#define TAPE_QUERY_MEDIA_CAPACITY         1L
#define TAPE_CHECK_FOR_DRIVE_PROBLEM      2L
#define TAPE_QUERY_IO_ERROR_DATA          3L
#define TAPE_QUERY_DEVICE_ERROR_DATA      4L

typedef struct _TAPE_WMI_OPERATIONS {
   DWORD Method;
   DWORD DataBufferSize;
   PVOID DataBuffer;
} TAPE_WMI_OPERATIONS, *PTAPE_WMI_OPERATIONS;

//
// Type of drive errors
//
typedef enum _TAPE_DRIVE_PROBLEM_TYPE {
   TapeDriveProblemNone, TapeDriveReadWriteWarning,
   TapeDriveReadWriteError, TapeDriveReadWarning,
   TapeDriveWriteWarning, TapeDriveReadError,
   TapeDriveWriteError, TapeDriveHardwareError,
   TapeDriveUnsupportedMedia, TapeDriveScsiConnectionError,
   TapeDriveTimetoClean, TapeDriveCleanDriveNow,
   TapeDriveMediaLifeExpired, TapeDriveSnappedTape
} TAPE_DRIVE_PROBLEM_TYPE;

#endif
#ifndef _NTTMAPI_
#define _NTTMAPI_


#ifdef __cplusplus
extern "C" {
#endif


#include <ktmtypes.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

//
// Types for Nt level TM calls
//

// begin_access

//
// KTM Tm object rights
//
#define TRANSACTIONMANAGER_QUERY_INFORMATION     ( 0x0001 )
#define TRANSACTIONMANAGER_SET_INFORMATION       ( 0x0002 )
#define TRANSACTIONMANAGER_RECOVER               ( 0x0004 )
#define TRANSACTIONMANAGER_RENAME                ( 0x0008 )
#define TRANSACTIONMANAGER_CREATE_RM             ( 0x0010 )

// The following right is intended for DTC's use only; it will be
// deprecated, and no one else should take a dependency on it.
#define TRANSACTIONMANAGER_BIND_TRANSACTION      ( 0x0020 )

//
// Generic mappings for transaction manager rights.
//

#define TRANSACTIONMANAGER_GENERIC_READ            (STANDARD_RIGHTS_READ            |\
                                                    TRANSACTIONMANAGER_QUERY_INFORMATION)

#define TRANSACTIONMANAGER_GENERIC_WRITE           (STANDARD_RIGHTS_WRITE           |\
                                                    TRANSACTIONMANAGER_SET_INFORMATION     |\
                                                    TRANSACTIONMANAGER_RECOVER             |\
                                                    TRANSACTIONMANAGER_RENAME              |\
                                                    TRANSACTIONMANAGER_CREATE_RM)

#define TRANSACTIONMANAGER_GENERIC_EXECUTE         (STANDARD_RIGHTS_EXECUTE)

#define TRANSACTIONMANAGER_ALL_ACCESS              (STANDARD_RIGHTS_REQUIRED        |\
                                                    TRANSACTIONMANAGER_GENERIC_READ        |\
                                                    TRANSACTIONMANAGER_GENERIC_WRITE       |\
                                                    TRANSACTIONMANAGER_GENERIC_EXECUTE     |\
                                                    TRANSACTIONMANAGER_BIND_TRANSACTION)


//
// KTM transaction object rights.
//
#define TRANSACTION_QUERY_INFORMATION     ( 0x0001 )
#define TRANSACTION_SET_INFORMATION       ( 0x0002 )
#define TRANSACTION_ENLIST                ( 0x0004 )
#define TRANSACTION_COMMIT                ( 0x0008 )
#define TRANSACTION_ROLLBACK              ( 0x0010 )
#define TRANSACTION_PROPAGATE             ( 0x0020 )
#define TRANSACTION_RIGHT_RESERVED1       ( 0x0040 )

//
// Generic mappings for transaction rights.
// Resource managers, when enlisting, should generally use the macro
// TRANSACTION_RESOURCE_MANAGER_RIGHTS when opening a transaction.
// It's the same as generic read and write except that it does not allow
// a commit decision to be made.
//

#define TRANSACTION_GENERIC_READ            (STANDARD_RIGHTS_READ            |\
                                             TRANSACTION_QUERY_INFORMATION   |\
                                             SYNCHRONIZE)

#define TRANSACTION_GENERIC_WRITE           (STANDARD_RIGHTS_WRITE           |\
                                             TRANSACTION_SET_INFORMATION     |\
                                             TRANSACTION_COMMIT              |\
                                             TRANSACTION_ENLIST              |\
                                             TRANSACTION_ROLLBACK            |\
                                             TRANSACTION_PROPAGATE           |\
                                             SYNCHRONIZE)

#define TRANSACTION_GENERIC_EXECUTE         (STANDARD_RIGHTS_EXECUTE         |\
                                             TRANSACTION_COMMIT              |\
                                             TRANSACTION_ROLLBACK            |\
                                             SYNCHRONIZE)

#define TRANSACTION_ALL_ACCESS              (STANDARD_RIGHTS_REQUIRED        |\
                                             TRANSACTION_GENERIC_READ        |\
                                             TRANSACTION_GENERIC_WRITE       |\
                                             TRANSACTION_GENERIC_EXECUTE)

#define TRANSACTION_RESOURCE_MANAGER_RIGHTS (TRANSACTION_GENERIC_READ        |\
                                             STANDARD_RIGHTS_WRITE           |\
                                             TRANSACTION_SET_INFORMATION     |\
                                             TRANSACTION_ENLIST              |\
                                             TRANSACTION_ROLLBACK            |\
                                             TRANSACTION_PROPAGATE           |\
                                             SYNCHRONIZE)

//
// KTM resource manager object rights.
//
#define RESOURCEMANAGER_QUERY_INFORMATION     ( 0x0001 )
#define RESOURCEMANAGER_SET_INFORMATION       ( 0x0002 )
#define RESOURCEMANAGER_RECOVER               ( 0x0004 )
#define RESOURCEMANAGER_ENLIST                ( 0x0008 )
#define RESOURCEMANAGER_GET_NOTIFICATION      ( 0x0010 )
#define RESOURCEMANAGER_REGISTER_PROTOCOL     ( 0x0020 )
#define RESOURCEMANAGER_COMPLETE_PROPAGATION  ( 0x0040 )

//
// Generic mappings for resource manager rights.
//
#define RESOURCEMANAGER_GENERIC_READ        (STANDARD_RIGHTS_READ                 |\
                                             RESOURCEMANAGER_QUERY_INFORMATION    |\
                                             SYNCHRONIZE)

#define RESOURCEMANAGER_GENERIC_WRITE       (STANDARD_RIGHTS_WRITE                |\
                                             RESOURCEMANAGER_SET_INFORMATION      |\
                                             RESOURCEMANAGER_RECOVER              |\
                                             RESOURCEMANAGER_ENLIST               |\
                                             RESOURCEMANAGER_GET_NOTIFICATION     |\
                                             RESOURCEMANAGER_REGISTER_PROTOCOL    |\
                                             RESOURCEMANAGER_COMPLETE_PROPAGATION |\
                                             SYNCHRONIZE)

#define RESOURCEMANAGER_GENERIC_EXECUTE     (STANDARD_RIGHTS_EXECUTE              |\
                                             RESOURCEMANAGER_RECOVER              |\
                                             RESOURCEMANAGER_ENLIST               |\
                                             RESOURCEMANAGER_GET_NOTIFICATION     |\
                                             RESOURCEMANAGER_COMPLETE_PROPAGATION |\
                                             SYNCHRONIZE)

#define RESOURCEMANAGER_ALL_ACCESS          (STANDARD_RIGHTS_REQUIRED             |\
                                             RESOURCEMANAGER_GENERIC_READ         |\
                                             RESOURCEMANAGER_GENERIC_WRITE        |\
                                             RESOURCEMANAGER_GENERIC_EXECUTE)


//
// KTM enlistment object rights.
//
#define ENLISTMENT_QUERY_INFORMATION     ( 0x0001 )
#define ENLISTMENT_SET_INFORMATION       ( 0x0002 )
#define ENLISTMENT_RECOVER               ( 0x0004 )
#define ENLISTMENT_SUBORDINATE_RIGHTS    ( 0x0008 )
#define ENLISTMENT_SUPERIOR_RIGHTS       ( 0x0010 )

//
// Generic mappings for enlistment rights.
//
#define ENLISTMENT_GENERIC_READ        (STANDARD_RIGHTS_READ           |\
                                        ENLISTMENT_QUERY_INFORMATION)

#define ENLISTMENT_GENERIC_WRITE       (STANDARD_RIGHTS_WRITE          |\
                                        ENLISTMENT_SET_INFORMATION     |\
                                        ENLISTMENT_RECOVER             |\
                                        ENLISTMENT_SUBORDINATE_RIGHTS  |\
                                        ENLISTMENT_SUPERIOR_RIGHTS)

#define ENLISTMENT_GENERIC_EXECUTE     (STANDARD_RIGHTS_EXECUTE        |\
                                        ENLISTMENT_RECOVER             |\
                                        ENLISTMENT_SUBORDINATE_RIGHTS  |\
                                        ENLISTMENT_SUPERIOR_RIGHTS)

#define ENLISTMENT_ALL_ACCESS          (STANDARD_RIGHTS_REQUIRED       |\
                                        ENLISTMENT_GENERIC_READ        |\
                                        ENLISTMENT_GENERIC_WRITE       |\
                                        ENLISTMENT_GENERIC_EXECUTE)

// end_access

//
// Transaction outcomes.
//
// TODO: warning, must match values in KTRANSACTION_OUTCOME duplicated def 
// in tm.h.
//

typedef enum _TRANSACTION_OUTCOME {
    TransactionOutcomeUndetermined = 1,
    TransactionOutcomeCommitted,
    TransactionOutcomeAborted,
} TRANSACTION_OUTCOME;


typedef enum _TRANSACTION_STATE {
    TransactionStateNormal = 1,
    TransactionStateIndoubt,
    TransactionStateCommittedNotify,
} TRANSACTION_STATE;


typedef struct _TRANSACTION_BASIC_INFORMATION {
    GUID    TransactionId;
    DWORD   State;
    DWORD   Outcome;
} TRANSACTION_BASIC_INFORMATION, *PTRANSACTION_BASIC_INFORMATION;

typedef struct _TRANSACTIONMANAGER_BASIC_INFORMATION {
    GUID    TmIdentity;
    LARGE_INTEGER VirtualClock;
} TRANSACTIONMANAGER_BASIC_INFORMATION, *PTRANSACTIONMANAGER_BASIC_INFORMATION;

typedef struct _TRANSACTIONMANAGER_LOG_INFORMATION {
    GUID  LogIdentity;
} TRANSACTIONMANAGER_LOG_INFORMATION, *PTRANSACTIONMANAGER_LOG_INFORMATION;

typedef struct _TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    DWORD LogPathLength;
    _Field_size_(LogPathLength) WCHAR LogPath[1]; // Variable size
//  Data[1];                                        // Variable size data not declared
} TRANSACTIONMANAGER_LOGPATH_INFORMATION, *PTRANSACTIONMANAGER_LOGPATH_INFORMATION;

typedef struct _TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    ULONGLONG  LastRecoveredLsn;
} TRANSACTIONMANAGER_RECOVERY_INFORMATION, *PTRANSACTIONMANAGER_RECOVERY_INFORMATION;


// end_wdm
typedef struct _TRANSACTIONMANAGER_OLDEST_INFORMATION {
    GUID OldestTransactionGuid;
} TRANSACTIONMANAGER_OLDEST_INFORMATION, *PTRANSACTIONMANAGER_OLDEST_INFORMATION;
// begin_wdm


typedef struct _TRANSACTION_PROPERTIES_INFORMATION {
    DWORD              IsolationLevel;
    DWORD              IsolationFlags;
    LARGE_INTEGER      Timeout;
    DWORD              Outcome;
    DWORD              DescriptionLength;
    WCHAR              Description[1];            // Variable size
//          Data[1];            // Variable size data not declared
} TRANSACTION_PROPERTIES_INFORMATION, *PTRANSACTION_PROPERTIES_INFORMATION;

// The following info-class is intended for DTC's use only; it will be
// deprecated, and no one else should take a dependency on it.
typedef struct _TRANSACTION_BIND_INFORMATION {
    HANDLE TmHandle;
} TRANSACTION_BIND_INFORMATION, *PTRANSACTION_BIND_INFORMATION;

typedef struct _TRANSACTION_ENLISTMENT_PAIR {
    GUID   EnlistmentId;
    GUID   ResourceManagerId;
} TRANSACTION_ENLISTMENT_PAIR, *PTRANSACTION_ENLISTMENT_PAIR;

typedef struct _TRANSACTION_ENLISTMENTS_INFORMATION {
    DWORD                       NumberOfEnlistments;
    TRANSACTION_ENLISTMENT_PAIR EnlistmentPair[1]; // Variable size
} TRANSACTION_ENLISTMENTS_INFORMATION, *PTRANSACTION_ENLISTMENTS_INFORMATION;

typedef struct _TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    TRANSACTION_ENLISTMENT_PAIR SuperiorEnlistmentPair;
} TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION, *PTRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION;


typedef struct _RESOURCEMANAGER_BASIC_INFORMATION {
    GUID    ResourceManagerId;
    DWORD   DescriptionLength;
    WCHAR   Description[1];            // Variable size
} RESOURCEMANAGER_BASIC_INFORMATION, *PRESOURCEMANAGER_BASIC_INFORMATION;

typedef struct _RESOURCEMANAGER_COMPLETION_INFORMATION {
    HANDLE    IoCompletionPortHandle;
    ULONG_PTR CompletionKey;
} RESOURCEMANAGER_COMPLETION_INFORMATION, *PRESOURCEMANAGER_COMPLETION_INFORMATION;

// end_wdm

// begin_wdm
typedef enum _TRANSACTION_INFORMATION_CLASS {
    TransactionBasicInformation,
    TransactionPropertiesInformation,
    TransactionEnlistmentInformation,
    TransactionSuperiorEnlistmentInformation
// end_wdm
    ,
// The following info-classes are intended for DTC's use only; it will be
// deprecated, and no one else should take a dependency on it.
    TransactionBindInformation, // private and deprecated
    TransactionDTCPrivateInformation // private and deprecated
    ,
// begin_wdm
} TRANSACTION_INFORMATION_CLASS;

// begin_wdm
typedef enum _TRANSACTIONMANAGER_INFORMATION_CLASS {
    TransactionManagerBasicInformation,
    TransactionManagerLogInformation,
    TransactionManagerLogPathInformation,
    TransactionManagerRecoveryInformation = 4
// end_wdm
    ,
// The following info-classes are intended for internal use only; they
// are considered deprecated, and no one else should take a dependency
// on them.
    TransactionManagerOnlineProbeInformation = 3,
    TransactionManagerOldestTransactionInformation = 5
// end_wdm

// begin_wdm
} TRANSACTIONMANAGER_INFORMATION_CLASS;


// begin_wdm
typedef enum _RESOURCEMANAGER_INFORMATION_CLASS {
    ResourceManagerBasicInformation,
    ResourceManagerCompletionInformation,
} RESOURCEMANAGER_INFORMATION_CLASS;


typedef struct _ENLISTMENT_BASIC_INFORMATION {
    GUID    EnlistmentId;
    GUID    TransactionId;
    GUID    ResourceManagerId;
} ENLISTMENT_BASIC_INFORMATION, *PENLISTMENT_BASIC_INFORMATION;

typedef struct _ENLISTMENT_CRM_INFORMATION {
    GUID   CrmTransactionManagerId;
    GUID   CrmResourceManagerId;
    GUID   CrmEnlistmentId;
} ENLISTMENT_CRM_INFORMATION, *PENLISTMENT_CRM_INFORMATION;


// begin_wdm
typedef enum _ENLISTMENT_INFORMATION_CLASS {
    EnlistmentBasicInformation,
    EnlistmentRecoveryInformation,
    EnlistmentCrmInformation
} ENLISTMENT_INFORMATION_CLASS;

typedef struct _TRANSACTION_LIST_ENTRY {
    UOW    UOW;
} TRANSACTION_LIST_ENTRY, *PTRANSACTION_LIST_ENTRY;

typedef struct _TRANSACTION_LIST_INFORMATION {
    DWORD   NumberOfTransactions;
    TRANSACTION_LIST_ENTRY TransactionInformation[1]; // Var size
} TRANSACTION_LIST_INFORMATION, *PTRANSACTION_LIST_INFORMATION;


//
// Types of objects known to the kernel transaction manager.
//

typedef enum _KTMOBJECT_TYPE {

    KTMOBJECT_TRANSACTION,
    KTMOBJECT_TRANSACTION_MANAGER,
    KTMOBJECT_RESOURCE_MANAGER,
    KTMOBJECT_ENLISTMENT,
    KTMOBJECT_INVALID

} KTMOBJECT_TYPE, *PKTMOBJECT_TYPE;


//
// KTMOBJECT_CURSOR
//
// Used by NtEnumerateTransactionObject to enumerate a transaction
// object namespace (e.g. enlistments in a resource manager).
//

typedef struct _KTMOBJECT_CURSOR {

    //
    // The last GUID enumerated; zero if beginning enumeration.
    // 

    GUID    LastQuery;

    //
    // A count of GUIDs filled in by this last enumeration.
    // 

    DWORD   ObjectIdCount;

    //
    // ObjectIdCount GUIDs from the namespace specified.
    // 

    GUID    ObjectIds[1];

} KTMOBJECT_CURSOR, *PKTMOBJECT_CURSOR;

// begin_wdm

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#ifdef __cplusplus
}
#endif

#endif // _NTTMAPI_
typedef DWORD TP_VERSION, *PTP_VERSION; 

typedef struct _TP_CALLBACK_INSTANCE TP_CALLBACK_INSTANCE, *PTP_CALLBACK_INSTANCE;

typedef
VOID
NTAPI
TP_SIMPLE_CALLBACK (
    _Inout_     PTP_CALLBACK_INSTANCE Instance,
    _Inout_opt_ PVOID                 Context
    );

typedef TP_SIMPLE_CALLBACK *PTP_SIMPLE_CALLBACK;

typedef struct _TP_POOL TP_POOL, *PTP_POOL; 

typedef enum _TP_CALLBACK_PRIORITY {
    TP_CALLBACK_PRIORITY_HIGH,
    TP_CALLBACK_PRIORITY_NORMAL,
    TP_CALLBACK_PRIORITY_LOW,
    TP_CALLBACK_PRIORITY_INVALID,
    TP_CALLBACK_PRIORITY_COUNT = TP_CALLBACK_PRIORITY_INVALID
} TP_CALLBACK_PRIORITY;

typedef struct _TP_POOL_STACK_INFORMATION {
    SIZE_T StackReserve;
    SIZE_T StackCommit;
}TP_POOL_STACK_INFORMATION, *PTP_POOL_STACK_INFORMATION;

typedef struct _TP_CLEANUP_GROUP TP_CLEANUP_GROUP, *PTP_CLEANUP_GROUP; 

typedef
VOID
NTAPI
TP_CLEANUP_GROUP_CANCEL_CALLBACK (
    _Inout_opt_ PVOID ObjectContext,
    _Inout_opt_ PVOID CleanupContext
    );

typedef TP_CLEANUP_GROUP_CANCEL_CALLBACK *PTP_CLEANUP_GROUP_CANCEL_CALLBACK;


//
// Do not manipulate this structure directly!  Allocate space for it
// and use the inline interfaces below.
//

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

typedef struct _TP_CALLBACK_ENVIRON_V3 {
    TP_VERSION                         Version;
    PTP_POOL                           Pool;
    PTP_CLEANUP_GROUP                  CleanupGroup;
    PTP_CLEANUP_GROUP_CANCEL_CALLBACK  CleanupGroupCancelCallback;
    PVOID                              RaceDll;
    struct _ACTIVATION_CONTEXT        *ActivationContext;
    PTP_SIMPLE_CALLBACK                FinalizationCallback;
    union {
        DWORD                          Flags;
        struct {
            DWORD                      LongFunction :  1;
            DWORD                      Persistent   :  1;
            DWORD                      Private      : 30;
        } s;
    } u;
    TP_CALLBACK_PRIORITY               CallbackPriority;
    DWORD                              Size;
} TP_CALLBACK_ENVIRON_V3;

typedef TP_CALLBACK_ENVIRON_V3 TP_CALLBACK_ENVIRON, *PTP_CALLBACK_ENVIRON;

#else

typedef struct _TP_CALLBACK_ENVIRON_V1 {
    TP_VERSION                         Version;
    PTP_POOL                           Pool;
    PTP_CLEANUP_GROUP                  CleanupGroup;
    PTP_CLEANUP_GROUP_CANCEL_CALLBACK  CleanupGroupCancelCallback;
    PVOID                              RaceDll;
    struct _ACTIVATION_CONTEXT        *ActivationContext;
    PTP_SIMPLE_CALLBACK                FinalizationCallback;
    union {
        DWORD                          Flags;
        struct {
            DWORD                      LongFunction :  1;
            DWORD                      Persistent   :  1;
            DWORD                      Private      : 30;
        } s;
    } u;
} TP_CALLBACK_ENVIRON_V1;

typedef TP_CALLBACK_ENVIRON_V1 TP_CALLBACK_ENVIRON, *PTP_CALLBACK_ENVIRON;

#endif

#if !defined(MIDL_PASS)

FORCEINLINE
VOID
TpInitializeCallbackEnviron(
    _Out_ PTP_CALLBACK_ENVIRON CallbackEnviron
    )
{

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

    CallbackEnviron->Version = 3;

#else

    CallbackEnviron->Version = 1;

#endif

    CallbackEnviron->Pool = NULL;
    CallbackEnviron->CleanupGroup = NULL;
    CallbackEnviron->CleanupGroupCancelCallback = NULL;
    CallbackEnviron->RaceDll = NULL;
    CallbackEnviron->ActivationContext = NULL;
    CallbackEnviron->FinalizationCallback = NULL;
    CallbackEnviron->u.Flags = 0;

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

    CallbackEnviron->CallbackPriority = TP_CALLBACK_PRIORITY_NORMAL;
    CallbackEnviron->Size = sizeof(TP_CALLBACK_ENVIRON);

#endif

}

FORCEINLINE
VOID
TpSetCallbackThreadpool(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron,
    _In_    PTP_POOL             Pool
    )
{
    CallbackEnviron->Pool = Pool;
}

FORCEINLINE
VOID
TpSetCallbackCleanupGroup(
    _Inout_  PTP_CALLBACK_ENVIRON              CallbackEnviron,
    _In_     PTP_CLEANUP_GROUP                 CleanupGroup,
    _In_opt_ PTP_CLEANUP_GROUP_CANCEL_CALLBACK CleanupGroupCancelCallback
    )
{
    CallbackEnviron->CleanupGroup = CleanupGroup;
    CallbackEnviron->CleanupGroupCancelCallback = CleanupGroupCancelCallback;
}

FORCEINLINE
VOID
TpSetCallbackActivationContext(
    _Inout_  PTP_CALLBACK_ENVIRON CallbackEnviron,
    _In_opt_ struct _ACTIVATION_CONTEXT *ActivationContext
    )
{
    CallbackEnviron->ActivationContext = ActivationContext;
}

FORCEINLINE
VOID
TpSetCallbackNoActivationContext(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron
    )
{
    CallbackEnviron->ActivationContext = (struct _ACTIVATION_CONTEXT *)(LONG_PTR) -1; // INVALID_ACTIVATION_CONTEXT
}

FORCEINLINE
VOID
TpSetCallbackLongFunction(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron
    )
{
    CallbackEnviron->u.s.LongFunction = 1;
}

FORCEINLINE
VOID
TpSetCallbackRaceWithDll(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron,
    _In_    PVOID                DllHandle
    )
{
    CallbackEnviron->RaceDll = DllHandle;
}

FORCEINLINE
VOID
TpSetCallbackFinalizationCallback(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron,
    _In_    PTP_SIMPLE_CALLBACK  FinalizationCallback
    )
{
    CallbackEnviron->FinalizationCallback = FinalizationCallback;
}

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN7)

FORCEINLINE
VOID
TpSetCallbackPriority(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron,
    _In_    TP_CALLBACK_PRIORITY Priority
    )
{
    CallbackEnviron->CallbackPriority = Priority;
}

#endif

FORCEINLINE
VOID
TpSetCallbackPersistent(
    _Inout_ PTP_CALLBACK_ENVIRON CallbackEnviron
    )
{
    CallbackEnviron->u.s.Persistent = 1;
}


FORCEINLINE
VOID
TpDestroyCallbackEnviron(
    _In_ PTP_CALLBACK_ENVIRON CallbackEnviron
    )
{
    //
    // For the current version of the callback environment, no actions
    // need to be taken to tear down an initialized structure.  This
    // may change in a future release.
    //

    UNREFERENCED_PARAMETER(CallbackEnviron);
}

#endif // !defined(MIDL_PASS)


typedef struct _TP_WORK TP_WORK, *PTP_WORK;

typedef
VOID
NTAPI
TP_WORK_CALLBACK (
    _Inout_     PTP_CALLBACK_INSTANCE Instance,
    _Inout_opt_ PVOID                 Context,
    _Inout_     PTP_WORK              Work
    );

typedef TP_WORK_CALLBACK *PTP_WORK_CALLBACK;


typedef struct _TP_TIMER TP_TIMER, *PTP_TIMER;

typedef
VOID
NTAPI
TP_TIMER_CALLBACK (
    _Inout_     PTP_CALLBACK_INSTANCE Instance,
    _Inout_opt_ PVOID                 Context,
    _Inout_     PTP_TIMER             Timer
    );

typedef TP_TIMER_CALLBACK *PTP_TIMER_CALLBACK;


typedef DWORD    TP_WAIT_RESULT;

typedef struct _TP_WAIT TP_WAIT, *PTP_WAIT;

typedef
VOID
NTAPI
TP_WAIT_CALLBACK (
    _Inout_     PTP_CALLBACK_INSTANCE Instance,
    _Inout_opt_ PVOID                 Context,
    _Inout_     PTP_WAIT              Wait,
    _In_        TP_WAIT_RESULT        WaitResult
    );

typedef TP_WAIT_CALLBACK *PTP_WAIT_CALLBACK;


typedef struct _TP_IO TP_IO, *PTP_IO;

#if defined(_M_AMD64) && !defined(_M_ARM64EC) && !defined(__midl)

__forceinline
struct _TEB *
NtCurrentTeb (
    VOID
    )

{
    return (struct _TEB *)__readgsqword(FIELD_OFFSET(NT_TIB, Self));
}

#if defined(_KERNEL_MODE)

#pragma push_macro("__NtCurrentTebAddr")
#undef __NtCurrentTebAddr

#define __NtCurrentTebAddr(Offset) ((unsigned __int64)NtCurrentTeb() + (Offset))

//
// In kernel mode on AMD64, address of GS:[0] is KPCR instead of TEB,
// so read teb through NtCurrentTeb.
//

__forceinline
unsigned char
NtReadCurrentTebByte (
    unsigned int Offset
    )
{
    return *((const volatile unsigned char *)__NtCurrentTebAddr(Offset));
}

__forceinline
unsigned short
NtReadCurrentTebUshort (
    unsigned int Offset
    )
{
    return *((const volatile unsigned short *)__NtCurrentTebAddr(Offset));
}

__forceinline
unsigned int
NtReadCurrentTebUlong (
    unsigned int Offset
    )
{
    return *((const volatile unsigned int *)__NtCurrentTebAddr(Offset));
}

__forceinline
unsigned __int64
NtReadCurrentTebUlongPtr (
    unsigned int Offset
    )
{
    return *((const volatile unsigned __int64 *)__NtCurrentTebAddr(Offset));
}

__forceinline
void*
NtReadCurrentTebPVOID (
    unsigned int Offset
    )
{
    return (void*)NtReadCurrentTebUlongPtr(Offset);
}

__inline
unsigned __int64
NtReadCurrentTebUlonglong (
    unsigned int Offset
    )
{
    return *((const volatile unsigned __int64 *) __NtCurrentTebAddr(Offset));
}

#pragma pop_macro("__NtCurrentTebAddr")

#else

__forceinline
unsigned char
NtReadCurrentTebByte (
    unsigned int Offset
    )
{
    return __readgsbyte(Offset);
}

__forceinline
unsigned short
NtReadCurrentTebUshort (
    unsigned int Offset
    )
{
    return __readgsword(Offset);
}

__forceinline
unsigned int
NtReadCurrentTebUlong (
    unsigned int Offset
    )
{
    return __readgsdword(Offset);
}

__forceinline
unsigned __int64
NtReadCurrentTebUlongPtr (
    unsigned int Offset
    )
{
    return __readgsqword(Offset);
}

__forceinline
void*
NtReadCurrentTebPVOID (
    unsigned int Offset
    )
{
    return (void*)__readgsqword(Offset);
}

__forceinline
unsigned __int64
NtReadCurrentTebUlonglong (
    unsigned int Offset
    )
{
    return __readgsqword(Offset);
}

#endif // defined(_KERNEL_MODE)

__forceinline
PVOID
GetCurrentFiber (
    VOID
    )

{
    return (PVOID)__readgsqword(FIELD_OFFSET(NT_TIB, FiberData));
}

__forceinline
PVOID
GetFiberData (
    VOID
    )

{
    return *(PVOID *)GetCurrentFiber();
}

#endif // _M_AMD64 && !defined(__midl)


#if defined(_M_ARM) && !defined(__midl) && !defined(_M_CEE_PURE)

__forceinline
struct _TEB *
NtCurrentTeb (
    VOID
    )
{
    return (struct _TEB *)(ULONG_PTR)_MoveFromCoprocessor(CP15_TPIDRURW);
}

__forceinline
PVOID
GetCurrentFiber (
    VOID
    )
{
    return ((PNT_TIB )(ULONG_PTR)_MoveFromCoprocessor(CP15_TPIDRURW))->FiberData;
}

__forceinline
PVOID
GetFiberData (
    VOID
    )

{
    return *(PVOID *)GetCurrentFiber();
}

#endif // _M_ARM && !defined(__midl) && !defined(_M_CEE_PURE)


#if (defined(_M_ARM64) || defined(_M_ARM64EC)) && !defined(__midl) && !defined(_M_CEE_PURE)

__forceinline
struct _TEB *
NtCurrentTeb (
    VOID
    )
{
    return (struct _TEB *)__getReg(18);
}

#if defined(_MSC_VER) && (!defined(__clang__) || (defined(__clang_major__) && __clang_major__ >= 15))

__forceinline
unsigned char
NtReadCurrentTebByte (
    unsigned int Offset
    )
{
    return __readx18byte(Offset);
}

__forceinline
unsigned short
NtReadCurrentTebUshort (
    unsigned int Offset
    )
{
    return __readx18word(Offset);
}

__forceinline
unsigned int
NtReadCurrentTebUlong (
    unsigned int Offset
    )
{
    return __readx18dword(Offset);
}

__forceinline
unsigned __int64
NtReadCurrentTebUlongPtr (
    unsigned int Offset
    )
{
    return __readx18qword(Offset);
}

__forceinline
void*
NtReadCurrentTebPVOID (
    unsigned int Offset
    )
{
    return (void*)__readx18qword(Offset);
}

__forceinline
unsigned __int64
NtReadCurrentTebUlonglong (
    unsigned int Offset
    )
{
    return __readx18qword(Offset);
}

#else

#define __NtCurrentTebAddr(Offset) ((unsigned __int64) NtCurrentTeb() + (Offset))

#define NtReadCurrentTebByte(Offset) \
    ((unsigned char) __iso_volatile_load8((const volatile __int8 *) __NtCurrentTebAddr(Offset)))

#define NtReadCurrentTebUshort(Offset) \
    ((unsigned short) __iso_volatile_load16((const volatile __int16 *) __NtCurrentTebAddr(Offset)))

#define NtReadCurrentTebUlong(Offset) \
    ((unsigned int) __iso_volatile_load32((const volatile __int32 *) __NtCurrentTebAddr(Offset)))

#define NtReadCurrentTebUlongPtr(Offset) \
    ((unsigned __int64) __iso_volatile_load64((const volatile __int64*) __NtCurrentTebAddr(Offset)))

#define NtReadCurrentTebPVOID(Offset) \
    ((void*) __iso_volatile_load64((const volatile __int64*) __NtCurrentTebAddr(Offset)))

#define NtReadCurrentTebUlonglong(Offset) \
    ((unsigned __int64) __iso_volatile_load64((const volatile __int64*) __NtCurrentTebAddr(Offset)))

#endif

__forceinline
PVOID
GetCurrentFiber (
    VOID
    )
{
    return NtReadCurrentTebPVOID(FIELD_OFFSET(NT_TIB, FiberData));
}

__forceinline
PVOID
GetFiberData (
    VOID
    )

{
    return *(PVOID *)GetCurrentFiber();
}

#endif // _M_ARM64 && !defined(__midl) && !defined(_M_CEE_PURE)


#if defined(_M_IX86) && !defined(MIDL_PASS)

#define PcTeb 0x18

#if !defined(_M_CEE_PURE)

__inline
struct _TEB*
NtCurrentTeb(
    void)
{
    return (struct _TEB *) (ULONG_PTR) __readfsdword (PcTeb);
}

#if defined(_KERNEL_MODE)

#pragma push_macro("__NtCurrentTebAddr")
#undef __NtCurrentTebAddr

#define __NtCurrentTebAddr(Offset) ((unsigned int) NtCurrentTeb() + (Offset))

//
// In kernel mode on x86, address of FS:[0] is KPCR instead of TEB,
// so read teb through NtCurrentTeb.
//

__inline
unsigned char
NtReadCurrentTebByte (
    unsigned int Offset
    )
{
    return *((const volatile unsigned char *) __NtCurrentTebAddr(Offset));
}

__inline
unsigned short
NtReadCurrentTebUshort (
    unsigned int Offset
    )
{
    return *((const volatile unsigned short *) __NtCurrentTebAddr(Offset));
}

__inline
unsigned int
NtReadCurrentTebUlong (
    unsigned int Offset
    )
{
    return *((const volatile unsigned int *) __NtCurrentTebAddr(Offset));
}

__inline
unsigned int
NtReadCurrentTebUlongPtr (
    unsigned int Offset
    )
{
    return NtReadCurrentTebUlong(Offset);
}

__inline
void*
NtReadCurrentTebPVOID (
    unsigned int Offset
    )
{
    return (void*)NtReadCurrentTebUlongPtr(Offset);
}

#pragma pop_macro("__NtCurrentTebAddr")

#else

__inline
unsigned char
NtReadCurrentTebByte (
    unsigned int Offset
    )
{
    return __readfsbyte(Offset);
}

__inline
unsigned short
NtReadCurrentTebUshort (
    unsigned int Offset
    )
{
    return __readfsword(Offset);
}

__inline
unsigned int
NtReadCurrentTebUlong (
    unsigned int Offset
    )
{
    return __readfsdword(Offset);
}

__inline
unsigned int
NtReadCurrentTebUlongPtr (
    unsigned int Offset
    )
{
    return __readfsdword(Offset);
}

__inline
void*
NtReadCurrentTebPVOID (
    unsigned int Offset
    )
{
    return (void*)__readfsdword(Offset);
}

#endif // defined(_KERNEL_MODE)

#endif // !defined(_M_CEE_PURE)

#endif // defined(_M_IX86) && !defined(MIDL_PASS)

#if (_WIN32_WINNT > 0x0500) || (_WIN32_FUSION >= 0x0100) || ISOLATION_AWARE_ENABLED // winnt_only
#define ACTIVATION_CONTEXT_SECTION_ASSEMBLY_INFORMATION         (1)
#define ACTIVATION_CONTEXT_SECTION_DLL_REDIRECTION              (2)
#define ACTIVATION_CONTEXT_SECTION_WINDOW_CLASS_REDIRECTION     (3)
#define ACTIVATION_CONTEXT_SECTION_COM_SERVER_REDIRECTION       (4)
#define ACTIVATION_CONTEXT_SECTION_COM_INTERFACE_REDIRECTION    (5)
#define ACTIVATION_CONTEXT_SECTION_COM_TYPE_LIBRARY_REDIRECTION (6)
#define ACTIVATION_CONTEXT_SECTION_COM_PROGID_REDIRECTION       (7)
#define ACTIVATION_CONTEXT_SECTION_GLOBAL_OBJECT_RENAME_TABLE   (8)
#define ACTIVATION_CONTEXT_SECTION_CLR_SURROGATES               (9)
#define ACTIVATION_CONTEXT_SECTION_APPLICATION_SETTINGS         (10)
#define ACTIVATION_CONTEXT_SECTION_COMPATIBILITY_INFO           (11)
#define ACTIVATION_CONTEXT_SECTION_WINRT_ACTIVATABLE_CLASSES    (12)
#endif // winnt_only

#ifdef __cplusplus
}
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#else
#pragma warning(default:4200) // nonstandard extension used : zero-sized array in struct/union
#pragma warning(default:4201) // named type definition in parentheses
#pragma warning(default:4214) // bit field types other than int
#endif

#endif /* _WINNT_ */

