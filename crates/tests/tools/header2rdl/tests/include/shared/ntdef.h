/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    ntdef.h

Abstract:

    Type definitions for the basic types.


--*/

#ifndef _NTDEF_
#define _NTDEF_

#if _MSC_VER > 1000
#pragma once
#endif


#include <ctype.h>  // winnt ntndis
#include <winapifamily.h>  // winnt

// begin_winnt begin_ntoshvp begin_ntddk

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

// end_winnt end_ntoshvp end_ntddk

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4201) // nameless struct/union
#pragma warning(disable:4668) // #if not_defined treated as #if 0
#pragma warning(disable:4820) // padding added after data member
#endif

// begin_ntminiport begin_ntndis begin_ntminitape
// begin_ntoshvp

#ifndef IN
#define IN
#endif

#ifndef OUT
#define OUT
#endif

#ifndef OPTIONAL
#define OPTIONAL
#endif

#ifndef NOTHING
#define NOTHING
#endif

#ifndef CRITICAL
#define CRITICAL
#endif

#ifndef ANYSIZE_ARRAY
#define ANYSIZE_ARRAY 1       // winnt
#endif

// begin_winnt

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
#define MAX_NATURAL_ALIGNMENT sizeof(ULONG)
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
#define PROBE_ALIGNMENT( _s ) TYPE_ALIGNMENT( ULONG )
#elif defined(_IA64_) || defined(_ARM_) || defined(_ARM64_)

//
// TODO: WOWXX - Unblock ARM. Make all alignment checks ULONG for now.
//

#define PROBE_ALIGNMENT( _s ) TYPE_ALIGNMENT( ULONG )
#elif !defined(RC_INVOKED)
#error "No Target Architecture"
#endif

//
// Define PROBE_ALIGNMENT32 to be the same as PROBE_ALIGNMENT on x86, so that
// code hosting x86 under WoW can handle x86's maximum guaranteed alignment.
//

#define PROBE_ALIGNMENT32( _s ) TYPE_ALIGNMENT( ULONG )

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

// end_winnt

#ifndef CONST
#define CONST               const
#endif

// begin_winnt

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

// end_winnt end_ntminiport end_ntndis end_ntminitape

#ifndef DECLSPEC_GUARDIGNORE
#if (_MSC_FULL_VER >= 170065501) || defined(_D1VERSIONLKG171_)
#define DECLSPEC_GUARDIGNORE  __declspec(guard(ignore))
#else
#define DECLSPEC_GUARDIGNORE
#endif
#endif

// begin_winnt begin_ntminiport begin_ntndis begin_ntminitape

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

// end_winnt
// begin_wudfwdm

#ifndef _MANAGED
#if defined(_M_IX86)
#define FASTCALL __fastcall
#else
#define FASTCALL
#endif
#else
#define FASTCALL NTAPI
#endif

// end_wudfwdm
// begin_winnt

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
typedef WCHAR TUCHAR, *PTUCHAR;
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
typedef unsigned char TUCHAR, *PTUCHAR;
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


// end_winnt

//
// The type QUAD and UQUAD are intended to use when a 8 byte aligned structure
// is required, but it is not a floating point number.
//

typedef double DOUBLE;

typedef struct _QUAD {
    union {
        __int64 UseThisFieldToCopy;
        double  DoNotUseThisField;
    } DUMMYUNIONNAME;

} QUAD;

//
// Pointer to Basics
//

typedef SHORT *PSHORT;  // winnt
typedef LONG *PLONG;    // winnt
typedef QUAD *PQUAD;

//
// Unsigned Basics
//

// Tell windef.h that some types are already defined.
#define BASETYPES

typedef unsigned char UCHAR;
typedef unsigned short USHORT;
typedef unsigned long ULONG;
typedef QUAD UQUAD;

//
// Pointer to Unsigned Basics
//

typedef UCHAR *PUCHAR;
typedef USHORT *PUSHORT;
typedef ULONG *PULONG;
typedef UQUAD *PUQUAD;

// end_ntoshvp

#if _WIN32_WINNT >= 0x0600 || (defined(__cplusplus) && defined(WINDOWS_ENABLE_CPLUSPLUS))

//
// Pointer to Const Unsigned Basics
//

typedef CONST UCHAR *PCUCHAR;
typedef CONST USHORT *PCUSHORT;
typedef CONST ULONG *PCULONG;
typedef CONST UQUAD *PCUQUAD;

#endif // _WIN32_WINNT >= 0x0600

//
// Signed characters
//

typedef signed char SCHAR;
typedef SCHAR *PSCHAR;

#if _WIN32_WINNT >= 0x0600 || (defined(__cplusplus) && defined(WINDOWS_ENABLE_CPLUSPLUS))

typedef CONST SCHAR *PCSCHAR;

#endif // _WIN32_WINNT >= 0x0600

#ifndef NO_STRICT
#ifndef STRICT
#define STRICT 1
#endif
#endif

// begin_winnt

#define ALL_PROCESSOR_GROUPS        0xffff

//
// Structure to represent a system wide processor number. It contains a
// group number and relative processor number within the group.
//

typedef struct _PROCESSOR_NUMBER {
    USHORT Group;
    UCHAR Number;
    UCHAR Reserved;
} PROCESSOR_NUMBER, *PPROCESSOR_NUMBER;

// begin_ntoshvp

//
// Structure to represent a group-specific affinity, such as that of a
// thread.  Specifies the group number and the affinity within that group.
//

typedef struct _GROUP_AFFINITY {
    KAFFINITY Mask;
    USHORT Group;
    USHORT Reserved[3];
} GROUP_AFFINITY, *PGROUP_AFFINITY;

typedef struct _GROUP_AFFINITY32 {
    ULONG Mask;
    USHORT Group;
    USHORT Reserved[3];
} GROUP_AFFINITY32, *PGROUP_AFFINITY32;

typedef struct _GROUP_AFFINITY64 {
    unsigned __int64 Mask;
    USHORT Group;
    USHORT Reserved[3];
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

typedef UCHAR  FCHAR;
typedef USHORT FSHORT;
typedef ULONG  FLONG;

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

// end_winnt


//
// Low order two bits of a handle are ignored by the system and available
// for use by application code as tag bits.  The remaining bits are opaque
// and used to store a serial number and table index.
//

#define OBJ_HANDLE_TAGBITS  0x00000003L

// begin_ntoshvp
// begin_wudfpwdm

//
// Cardinal Data Types [0 - 2**N-2)
//

typedef char CCHAR;          // winnt
typedef short CSHORT;
typedef ULONG CLONG;

typedef CCHAR *PCCHAR;
typedef CSHORT *PCSHORT;
typedef CLONG *PCLONG;

// end_wudfpwdm
// end_ntoshvp
// end_ntminiport end_ntndis end_ntminitape

//
// NLS basics (Locale and Language Ids)
//

//
// ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED ** DEPRECATED **
//
// The LCID and LANGID concepts are deprecated and no longer supported.
// Please use Locale Names instead.  See the documentation for GetLocaleInfoEx.
//
// There are numerous challenges with the LCID concept which makes it impossible for
// them to support all of the languages and dialects that may be encountered by a
// modern application.  Please use Locale Names.
//
typedef ULONG LCID;         // DEPRECATED, use Locale Names.  See documentation for GetLocaleInfoEx. // winnt
typedef PULONG PLCID;       // DEPRECATED, use Locale Names.  See documentation for GetLocaleInfoEx. // winnt
typedef USHORT LANGID;      // DEPRECATED, use Locale Names.  See documentation for GetLocaleInfoEx. // winnt

// begin_winnt

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

// end_winnt

// begin_ntoshvp
// begin_wudfpwdm

//
// Logical Data Type - These are 32-bit logical values.
//
typedef ULONG LOGICAL;
typedef ULONG *PLOGICAL;

// end_wudfpwdm
// end_ntoshvp

// begin_ntndis begin_windbgkd
// begin_ntoshvp
// begin_wudfwdm
// begin_sdfwdm
//
// NTSTATUS
//

typedef _Return_type_success_(return >= 0) LONG NTSTATUS;
/*lint -save -e624 */  // Don't complain about different typedefs.
typedef NTSTATUS *PNTSTATUS;
/*lint -restore */  // Resume checking for different typedefs.

#if _WIN32_WINNT >= 0x0600
typedef CONST NTSTATUS *PCNTSTATUS;
#endif // _WIN32_WINNT >= 0x0600

//
//  Status values are 32 bit values laid out as follows:
//
//   3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
//   1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
//  +---+-+-------------------------+-------------------------------+
//  |Sev|C|       Facility          |               Code            |
//  +---+-+-------------------------+-------------------------------+
//
//  where
//
//      Sev - is the severity code
//
//          00 - Success
//          01 - Informational
//          10 - Warning
//          11 - Error
//
//      C - is the Customer code flag
//
//      Facility - is the facility code
//
//      Code - is the facility's status code
//

//
// Generic test for success on any status value (non-negative numbers
// indicate success).
//

#define NT_SUCCESS(Status) (((NTSTATUS)(Status)) >= 0)

//
// Generic test for information on any status value.
//

#define NT_INFORMATION(Status) ((((ULONG)(Status)) >> 30) == 1)

//
// Generic test for warning on any status value.
//

#define NT_WARNING(Status) ((((ULONG)(Status)) >> 30) == 2)

//
// Generic test for error on any status value.
//

#define NT_ERROR(Status) ((((ULONG)(Status)) >> 30) == 3)

// end_sdfwdm
// end_wudfwdm
// end_ntoshvp
// end_windbgkd
// begin_winnt
#define APPLICATION_ERROR_MASK       0x20000000
#define ERROR_SEVERITY_SUCCESS       0x00000000
#define ERROR_SEVERITY_INFORMATIONAL 0x40000000
#define ERROR_SEVERITY_WARNING       0x80000000
#define ERROR_SEVERITY_ERROR         0xC0000000
// end_winnt

#ifndef __SECSTATUS_DEFINED__
typedef long SECURITY_STATUS;
#define __SECSTATUS_DEFINED__
#endif


// end_ntndis
//
// Large (64-bit) integer types and operations
//

#define TIME LARGE_INTEGER
#define _TIME _LARGE_INTEGER
#define PTIME PLARGE_INTEGER
#define LowTime LowPart
#define HighTime HighPart

// begin_winnt
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

// end_winnt


// begin_winnt begin_ntminiport begin_ntndis begin_ntminitape

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
        ULONG LowPart;
        LONG HighPart;
    } DUMMYSTRUCTNAME;
    struct {
        ULONG LowPart;
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
        ULONG LowPart;
        ULONG HighPart;
    } DUMMYSTRUCTNAME;
    struct {
        ULONG LowPart;
        ULONG HighPart;
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
    ULONG LowPart;
    LONG HighPart;
} LUID, *PLUID;

#define _DWORDLONG_
typedef ULONGLONG  DWORDLONG;
typedef DWORDLONG *PDWORDLONG;

// end_winnt

// begin_sdfwdm
// begin_ntminiport begin_ntndis
// begin_wudfwdm
// begin_ntoshvp

//
// Physical address.
//

typedef LARGE_INTEGER PHYSICAL_ADDRESS, *PPHYSICAL_ADDRESS;

// end_ntoshvp
// end_wudfwdm
// end_ntminiport end_ntndis
// end_sdfwdm

// begin_winnt

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
    _In_ ULONG ShiftCount
    );

LONGLONG
NTAPI
Int64ShraMod32 (
    _In_ LONGLONG Value,
    _In_ ULONG ShiftCount
    );

ULONGLONG
NTAPI
Int64ShrlMod32 (
    _In_ ULONGLONG Value,
    _In_ ULONG ShiftCount
    );

#if _MSC_VER >= 1200
#pragma warning(push)
#endif
#pragma warning(disable:4035 4793)               // re-enable below

__inline ULONGLONG
NTAPI
Int64ShllMod32 (
    _In_ ULONGLONG Value,
    _In_ ULONG ShiftCount
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
    _In_ ULONG ShiftCount
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
    _In_ ULONG ShiftCount
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

// end_winnt

// begin_wudfwdm
//
// Event type
//

typedef enum _EVENT_TYPE {
    NotificationEvent,
    SynchronizationEvent
} EVENT_TYPE;

// end_wudfwdm
//
// Timer type
//

typedef enum _TIMER_TYPE {
    NotificationTimer,
    SynchronizationTimer
} TIMER_TYPE;

//
// Wait type
//

typedef enum _WAIT_TYPE {
    WaitAll,
    WaitAny,
    WaitNotification,
    WaitDequeue,
    WaitDpc
} WAIT_TYPE;

// begin_ntoshvp
// begin_wudfwdm
// begin_sdfwdm
//
// Pointer to an Asciiz string
//

typedef _Null_terminated_ CHAR *PSZ;
typedef _Null_terminated_ CONST char *PCSZ;

// begin_ntndis
//
// Counted String
//

typedef USHORT RTL_STRING_LENGTH_TYPE;

typedef struct _STRING {
    USHORT Length;
    USHORT MaximumLength;
#ifdef MIDL_PASS
    [size_is(MaximumLength), length_is(Length) ]
#endif // MIDL_PASS
    _Field_size_bytes_part_opt_(MaximumLength, Length) PCHAR Buffer;
} STRING;
typedef STRING *PSTRING;
typedef STRING ANSI_STRING;
typedef PSTRING PANSI_STRING;

// end_sdfwdm
// end_wudfwdm

typedef STRING OEM_STRING;
typedef PSTRING POEM_STRING;
typedef CONST STRING* PCOEM_STRING;

//
// CONSTCounted String
//

typedef struct _CSTRING {
    USHORT Length;
    USHORT MaximumLength;
    CONST char *Buffer;
} CSTRING;
typedef CSTRING *PCSTRING;
#define ANSI_NULL ((CHAR)0)     // winnt

// begin_wudfwdm
// begin_sdfwdm

typedef STRING CANSI_STRING;
typedef PSTRING PCANSI_STRING;

typedef STRING UTF8_STRING;
typedef PSTRING PUTF8_STRING;

//
// Unicode strings are counted 16-bit character strings. If they are
// NULL terminated, Length does not include trailing NULL.
//

typedef struct _UNICODE_STRING {
    USHORT Length;
    USHORT MaximumLength;
#ifdef MIDL_PASS
    [size_is(MaximumLength / 2), length_is((Length) / 2) ] USHORT * Buffer;
#else // MIDL_PASS
    _Field_size_bytes_part_opt_(MaximumLength, Length) PWCH   Buffer;
#endif // MIDL_PASS
} UNICODE_STRING;
typedef UNICODE_STRING *PUNICODE_STRING;
typedef const UNICODE_STRING *PCUNICODE_STRING;
// end_sdfwdm
// end_wudfwdm

#define UNICODE_NULL ((WCHAR)0) // winnt

#if _WIN32_WINNT >= 0x0500

#define UNICODE_STRING_MAX_BYTES ((USHORT) 65534) // winnt
#define UNICODE_STRING_MAX_CHARS (32767) // winnt

// begin_wudfwdm
// begin_sdfwdm

#define DECLARE_CONST_UNICODE_STRING(_var, _string) \
const WCHAR _var ## _buffer[] = _string; \
__pragma(warning(push)) \
__pragma(warning(disable:4221)) __pragma(warning(disable:4204)) \
const UNICODE_STRING _var = { sizeof(_string) - sizeof(WCHAR), sizeof(_string), (PWCH) _var ## _buffer } \
__pragma(warning(pop))

#define DECLARE_GLOBAL_CONST_UNICODE_STRING(_var, _str) \
extern const __declspec(selectany) UNICODE_STRING _var = RTL_CONSTANT_STRING(_str)

#define DECLARE_UNICODE_STRING_SIZE(_var, _size) \
WCHAR _var ## _buffer[_size]; \
__pragma(warning(push)) \
__pragma(warning(disable:4221)) __pragma(warning(disable:4204)) \
UNICODE_STRING _var = { 0, (_size) * sizeof(WCHAR) , _var ## _buffer } \
__pragma(warning(pop))

// end_sdfwdm
// end_wudfwdm

#endif // _WIN32_WINNT >= 0x0500


// begin_ntminiport begin_ntminitape

//
// Boolean
//

typedef UCHAR BOOLEAN;           // winnt
typedef BOOLEAN *PBOOLEAN;       // winnt

// end_ntminiport end_ntminitape

// begin_winnt
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
// end_winnt end_ntndis
// begin_wudfpwdm

//
// Balanced tree node (AVL or RB) structure definition.
//

#pragma warning(push)
#pragma warning(disable:4214)

typedef struct _RTL_BALANCED_NODE {
    union {
        struct _RTL_BALANCED_NODE *Children[2];
        struct {
            struct _RTL_BALANCED_NODE *Left;
            struct _RTL_BALANCED_NODE *Right;
        } DUMMYSTRUCTNAME;
    } DUMMYUNIONNAME;

#define RTL_BALANCED_NODE_RESERVED_PARENT_MASK 3

    union {
        UCHAR Red : 1;
        UCHAR Balance : 2;
        ULONG_PTR ParentValue;
    } DUMMYUNIONNAME2;
} RTL_BALANCED_NODE, *PRTL_BALANCED_NODE;

#pragma warning(pop)

#define RTL_BALANCED_NODE_GET_PARENT_POINTER(Node) \
    ((PRTL_BALANCED_NODE)((Node)->ParentValue & \
                          ~RTL_BALANCED_NODE_RESERVED_PARENT_MASK))

// end_wudfpwdm
// begin_winnt
// begin_ntoshvp

//
// These are needed for portable debugger support.
//

typedef struct LIST_ENTRY32 {
    ULONG Flink;
    ULONG Blink;
} LIST_ENTRY32;
typedef LIST_ENTRY32 *PLIST_ENTRY32;

typedef struct LIST_ENTRY64 {
    ULONGLONG Flink;
    ULONGLONG Blink;
} LIST_ENTRY64;
typedef LIST_ENTRY64 *PLIST_ENTRY64;

// end_winnt

typedef struct _SINGLE_LIST_ENTRY32 {
    ULONG Next;
} SINGLE_LIST_ENTRY32, *PSINGLE_LIST_ENTRY32;

#if !defined(MIDL_PASS)
__inline
void
ListEntry32To64(
    _In_ PLIST_ENTRY32 l32,
    _Out_ PLIST_ENTRY64 l64
    )
{
    l64->Flink = (ULONGLONG)(ULONG)l32->Flink;
    l64->Blink = (ULONGLONG)(ULONG)l32->Blink;
}

__inline
void
ListEntry64To32(
    _In_ PLIST_ENTRY64 l64,
    _Out_ PLIST_ENTRY32 l32
    )
{
    l32->Flink = (ULONG)l64->Flink;
    l32->Blink = (ULONG)l64->Blink;
}
#endif

//
// The WNF state name is the fixed-length binary that uniquely identifies the
// state. Clients should treat it as opaque data of a fixed size and
// use provided API to manipulate state names.
//
// This is being moved from ntexapi_h.h -- so for now it is defined in both places.
//

#ifndef _DEFINED__WNF_STATE_NAME
#define _DEFINED__WNF_STATE_NAME
typedef struct _WNF_STATE_NAME {
    ULONG Data[2];
} WNF_STATE_NAME;

typedef struct _WNF_STATE_NAME* PWNF_STATE_NAME;
typedef const struct _WNF_STATE_NAME* PCWNF_STATE_NAME;
#endif


typedef struct _STRING32 {
    USHORT   Length;
    USHORT   MaximumLength;
    ULONG  Buffer;
} STRING32;
typedef STRING32 *PSTRING32;

typedef STRING32 UNICODE_STRING32;
typedef UNICODE_STRING32 *PUNICODE_STRING32;

typedef STRING32 ANSI_STRING32;
typedef ANSI_STRING32 *PANSI_STRING32;


typedef struct _STRING64 {
    USHORT   Length;
    USHORT   MaximumLength;
    ULONGLONG  Buffer;
} STRING64;
typedef STRING64 *PSTRING64;

typedef STRING64 UNICODE_STRING64;
typedef UNICODE_STRING64 *PUNICODE_STRING64;

typedef STRING64 ANSI_STRING64;
typedef ANSI_STRING64 *PANSI_STRING64;

// begin_wudfwdm
//
// Valid values for the Attributes field
//

#define OBJ_INHERIT                         0x00000002L
#define OBJ_PERMANENT                       0x00000010L
#define OBJ_EXCLUSIVE                       0x00000020L
#define OBJ_CASE_INSENSITIVE                0x00000040L
#define OBJ_OPENIF                          0x00000080L
#define OBJ_OPENLINK                        0x00000100L
#define OBJ_KERNEL_HANDLE                   0x00000200L
#define OBJ_FORCE_ACCESS_CHECK              0x00000400L
#define OBJ_IGNORE_IMPERSONATED_DEVICEMAP   0x00000800L
#define OBJ_DONT_REPARSE                    0x00001000L
#define OBJ_VALID_ATTRIBUTES                0x00001FF2L

// end_wudfwdm
//
// Object Attributes structure
//

typedef struct _OBJECT_ATTRIBUTES64 {
    ULONG Length;
    ULONG64 RootDirectory;
    ULONG64 ObjectName;
    ULONG Attributes;
    ULONG64 SecurityDescriptor;
    ULONG64 SecurityQualityOfService;
} OBJECT_ATTRIBUTES64;
typedef OBJECT_ATTRIBUTES64 *POBJECT_ATTRIBUTES64;
typedef CONST OBJECT_ATTRIBUTES64 *PCOBJECT_ATTRIBUTES64;

typedef struct _OBJECT_ATTRIBUTES32 {
    ULONG Length;
    ULONG RootDirectory;
    ULONG ObjectName;
    ULONG Attributes;
    ULONG SecurityDescriptor;
    ULONG SecurityQualityOfService;
} OBJECT_ATTRIBUTES32;
typedef OBJECT_ATTRIBUTES32 *POBJECT_ATTRIBUTES32;
typedef CONST OBJECT_ATTRIBUTES32 *PCOBJECT_ATTRIBUTES32;

// begin_wudfwdm

typedef struct _OBJECT_ATTRIBUTES {
    ULONG Length;
    HANDLE RootDirectory;
    PUNICODE_STRING ObjectName;
    ULONG Attributes;
    PVOID SecurityDescriptor;        // Points to type SECURITY_DESCRIPTOR
    PVOID SecurityQualityOfService;  // Points to type SECURITY_QUALITY_OF_SERVICE
} OBJECT_ATTRIBUTES;
typedef OBJECT_ATTRIBUTES *POBJECT_ATTRIBUTES;
typedef CONST OBJECT_ATTRIBUTES *PCOBJECT_ATTRIBUTES;

//++
//
// VOID
// InitializeObjectAttributes(
//     _Out_ POBJECT_ATTRIBUTES p,
//     _In_ PUNICODE_STRING n,
//     _In_ ULONG a,
//     _In_ HANDLE r,
//     _In_ PSECURITY_DESCRIPTOR s
//     )
//
//--

#define InitializeObjectAttributes( p, n, a, r, s ) { \
    (p)->Length = sizeof( OBJECT_ATTRIBUTES );          \
    (p)->RootDirectory = r;                             \
    (p)->Attributes = a;                                \
    (p)->ObjectName = n;                                \
    (p)->SecurityDescriptor = s;                        \
    (p)->SecurityQualityOfService = NULL;               \
    }

// end_wudfwdm

// RTL_ to avoid collisions in the global namespace.
// I don't believe there are possible/likely constant RootDirectory
// or SecurityDescriptor values other than NULL, so they are hardcoded.
// As well, the string will generally be const, so we cast that away.
#define RTL_CONSTANT_OBJECT_ATTRIBUTES(n, a) \
    { sizeof(OBJECT_ATTRIBUTES), NULL, RTL_CONST_CAST(PUNICODE_STRING)(n), a, NULL, NULL }

// This synonym is more appropriate for initializing what isn't actually const.
#define RTL_INIT_OBJECT_ATTRIBUTES(n, a) RTL_CONSTANT_OBJECT_ATTRIBUTES(n, a)

// begin_ntminiport begin_ntndis begin_ntminitape

//
// Constants
//

#define FALSE   0
#define TRUE    1

#ifndef NULL
#ifdef __cplusplus
#define NULL    0
#define NULL64  0
#else
#define NULL    ((void *)0)
#define NULL64  ((void * POINTER_64)0)
#endif
#endif // NULL

// end_ntoshvp
// end_ntminiport end_ntndis end_ntminitape

// begin_winnt begin_ntndis

#include <guiddef.h>

#ifndef __OBJECTID_DEFINED
#define __OBJECTID_DEFINED

typedef struct  _OBJECTID {     // size is 20
    GUID Lineage;
    ULONG Uniquifier;
} OBJECTID;
#endif // !_OBJECTID_DEFINED

// end_winnt end_ntndis
// begin_wudfpwdm

#define MINCHAR     0x80        // winnt
#define MAXCHAR     0x7f        // winnt
#define MINSHORT    0x8000      // winnt
#define MAXSHORT    0x7fff      // winnt
#define MINLONG     0x80000000  // winnt
#define MAXLONG     0x7fffffff  // winnt
#define MAXUCHAR    0xff        // winnt
#define MAXUSHORT   0xffff      // winnt
#define MAXULONG    0xffffffff  // winnt

// end_wudfpwdm

//
// Useful Helper Macros
//

// begin_ntndis
// begin_wudfwdm
//
// Determine if an argument is present by testing the value of the pointer
// to the argument value.
//

#define ARGUMENT_PRESENT(ArgumentPointer)    (\
    (CHAR *)((ULONG_PTR)(ArgumentPointer)) != (CHAR *)(NULL) )

// end_wudfwdm
// begin_winnt begin_ntminiport
// begin_ntoshvp
//
// Calculate the byte offset of a field in a structure of type type.
//

#ifdef __has_builtin
#if __has_builtin(__builtin_offsetof)
#define FIELD_OFFSET(type, field)    ((LONG)__builtin_offsetof(type, field))
#define UFIELD_OFFSET(type, field)    ((ULONG)__builtin_offsetof(type, field))
#endif
#endif

#ifndef FIELD_OFFSET
#define FIELD_OFFSET(type, field)    ((LONG)(LONG_PTR)&(((type *)0)->field))
#define UFIELD_OFFSET(type, field)    ((ULONG)(LONG_PTR)&(((type *)0)->field))
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
//   ULONG Buffer[100];
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
//   ULONG Integer;
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

// end_winnt
//
// This works "generically" for Unicode and Ansi/Oem strings.
// Usage:
//   const static UNICODE_STRING FooU = RTL_CONSTANT_STRING(L"Foo");
//   const static         STRING Foo  = RTL_CONSTANT_STRING( "Foo");
// instead of the slower:
//   UNICODE_STRING FooU;
//           STRING Foo;
//   RtlInitUnicodeString(&FooU, L"Foo");
//          RtlInitString(&Foo ,  "Foo");
//
// Or:
//   const static char szFoo[] = "Foo";
//   const static STRING sFoo = RTL_CONSTANT_STRING(szFoo);
//
// This will compile without error or warning in C++. C will get a warning.
//
#ifdef __cplusplus
extern "C++"
{
template <size_t N> char _RTL_CONSTANT_STRING_type_check(const char  (&s)[N]);
template <size_t N> char _RTL_CONSTANT_STRING_type_check(const WCHAR (&s)[N]);
// __typeof would be desirable here instead of sizeof.
template <size_t N> class _RTL_CONSTANT_STRING_remove_const_template_class;
template <> class _RTL_CONSTANT_STRING_remove_const_template_class<sizeof(char)>  {public: typedef  char T; };
template <> class _RTL_CONSTANT_STRING_remove_const_template_class<sizeof(WCHAR)> {public: typedef WCHAR T; };
#define _RTL_CONSTANT_STRING_remove_const_macro(s) \
    (const_cast<_RTL_CONSTANT_STRING_remove_const_template_class<sizeof((s)[0])>::T*>(s))
}
#else
char _RTL_CONSTANT_STRING_type_check(const void *s);
#define _RTL_CONSTANT_STRING_remove_const_macro(s) (s)
#endif
#define RTL_CONSTANT_STRING(s) \
{ \
    sizeof( s ) - sizeof( (s)[0] ), \
    sizeof( s ) / sizeof(_RTL_CONSTANT_STRING_type_check(s)), \
    _RTL_CONSTANT_STRING_remove_const_macro(s) \
}
// begin_winnt

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

// end_winnt
// begin_ntminiport begin_ntndis
// begin_wudfwdm
// begin_ntoshvp

//
// Interrupt Request Level (IRQL)
//

typedef UCHAR KIRQL;

typedef KIRQL *PKIRQL;

// end_ntoshvp
// end_wudfwdm
// end_ntminiport end_ntndis

// begin_winnt

//
// Enclave ID definitions
//

#define ENCLAVE_SHORT_ID_LENGTH             16
#define ENCLAVE_LONG_ID_LENGTH              32

// end_winnt

//
// Product types
//

typedef enum _NT_PRODUCT_TYPE {
    NtProductWinNt = 1,
    NtProductLanManNt,
    NtProductServer
} NT_PRODUCT_TYPE, *PNT_PRODUCT_TYPE;


//
// the bit mask, SharedUserData->SuiteMask, is a ULONG
// so there can be a maximum of 32 entries
// in this enum.
//

typedef _Enum_is_bitflag_ enum _SUITE_TYPE {
    SmallBusiness,
    Enterprise,
    BackOffice,
    CommunicationServer,
    TerminalServer,
    SmallBusinessRestricted,
    EmbeddedNT,
    DataCenter,
    SingleUserTS,
    Personal,
    Blade,
    EmbeddedRestricted,
    SecurityAppliance,
    StorageServer,
    ComputeServer,
    WHServer,
    PhoneNT,
    MultiUserTS,
    MaxSuiteType
} SUITE_TYPE;

// begin_winnt

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

// end_winnt

// begin_winnt

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

// end_winnt

// begin_ntminiport begin_winnt begin_ntminitape
#include <sdkddkver.h>
// end_ntminiport end_winnt end_ntminitape

// begin_winnt begin_r_winnt

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
#define MAKELANGID(p, s)       ((((USHORT)(s)) << 10) | (USHORT)(p))
#define PRIMARYLANGID(lgid)    ((USHORT)(lgid) & 0x3ff)
#define SUBLANGID(lgid)        ((USHORT)(lgid) >> 10)

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

#define MAKELCID(lgid, srtid)  ((ULONG)((((ULONG)((USHORT)(srtid))) << 16) |  \
                                         ((ULONG)((USHORT)(lgid)))))
#define MAKESORTLCID(lgid, srtid, ver)                                            \
                               ((ULONG)((MAKELCID(lgid, srtid)) |             \
                                    (((ULONG)((USHORT)(ver))) << 20)))
#define LANGIDFROMLCID(lcid)   ((USHORT)(lcid))
#define SORTIDFROMLCID(lcid)   ((USHORT)((((ULONG)(lcid)) >> 16) & 0xf))
#define SORTVERSIONFROMLCID(lcid)  ((USHORT)((((ULONG)(lcid)) >> 20) & 0xf))

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

// end_winnt

//
//  Define standard min and max macros
//

#ifndef NOMINMAX

#ifndef min
#define min(a,b) (((a) < (b)) ? (a) : (b))
#endif

#ifndef max
#define max(a,b) (((a) > (b)) ? (a) : (b))
#endif

#endif  // NOMINMAX

// end_ntndis end_ntminitape
// begin_winnt

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

// end_ntminiport end_winnt

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif // _NTDEF_

