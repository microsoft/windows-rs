
/*++

Copyright (c) 1998-1999  Microsoft Corporation

Module Name:

    minidrv.h

Abstract:

    Common header file for Plug-in minidrivers.

Environment:

    Windows NT printer drivers

Revision History:


--*/


#ifndef _MINIDRV_H_
#define _MINIDRV_H_
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <stddef.h>
#include <stdlib.h>
#include <sdkddkver.h>

// The following headers are unused by the resource complier (RC). Since the RC supports only subset of statements
// it causes dependencies of these headers to trigger W4 warnings. So we are removing them in the RC case.
#ifndef RC_INVOKED
#include <objbase.h>
#include <windef.h>
#include <winbase.h>
#include <tchar.h>
#endif

#include <stdarg.h>
#include <winerror.h>
#include <wingdi.h>
#include <winddi.h>
#include <excpt.h>

//
// defined(KERNEL_MODE) Rendering module DLL in either kernel mode or user mode.
// defined(KERNEL_MODE) & defined(USERMODE_DRIVER) User mode rendering DLL
// !defined(KERNEL_MODE) UI module
//

// User mode DLL
#include <winspool.h>

#if !defined(KERNEL_MODE)
// UI DLL
#include <windows.h>
#include <compstui.h>
#include <winddiui.h>
#endif

#if defined(USERMODE_DRIVER) || !defined(KERNEL_MODE)
// UI DLL or User mode rendering DLL
#include <stdio.h>
#endif

#ifdef __cplusplus
extern "C" {
#endif


#ifdef WINNT_40
//
// The LONG_PTR is guaranteed to be the same size as a pointer.  Its
// size with change with pointer size (32/64).  It should be used
// anywhere that a pointer is cast to an integer type. ULONG_PTR is
//

typedef long LONG_PTR, *PLONG_PTR;
typedef unsigned long ULONG_PTR, *PULONG_PTR;
typedef int INT_PTR, *PINT_PTR;
typedef unsigned int UINT_PTR, *PUINT_PTR;
typedef ULONG_PTR DWORD_PTR, *PDWORD_PTR;


#define HandleToUlong( h ) ((ULONG) (h) )
#define PtrToUlong( p )    ((ULONG) (p) )
#define PtrToLong( p )     ((LONG) (p) )
#define PtrToUshort( p )   ((unsigned short) (p) )
#define PtrToShort( p )    ((short) (p) )

#define GWLP_USERDATA       GWL_USERDATA
#define DWLP_USER           DWL_USER
#define DWLP_MSGRESULT      0
#define SetWindowLongPtr    SetWindowLong
#define GetWindowLongPtr    GetWindowLong

#endif // WINNT_40

#include <printoem.h>
#include <prntfont.h>

//
// These macros are used for debugging purposes. They expand
// to white spaces on a free build. Here is a brief description
// of what they do and how they are used:
//
// giDebugLevel
//  Global variable which set the current debug level to control
//  the amount of debug messages emitted.
//
// VERBOSE(msg)
//  Display a message if the current debug level is <= DBG_VERBOSE.
//
// TERSE(msg)
//  Display a message if the current debug level is <= DBG_TERSE.
//
// WARNING(msg)
//  Display a message if the current debug level is <= DBG_WARNING.
//  The message format is: WRN filename (linenumber): message
//
// ERR(msg)
//  Similiar to WARNING macro above - displays a message
//  if the current debug level is <= DBG_ERROR.
//
// ASSERT(cond)
//  Verify a condition is true. If not, force a breakpoint.
//
// ASSERTMSG(cond, msg)
//  Verify a condition is true. If not, display a message and
//  force a breakpoint.
//
// RIP(msg)
//  Display a message and force a breakpoint.
//
// Usage:
//  These macros require extra parantheses for the msg argument
//  example, ASSERTMSG(x > 0, ("x is less than 0\n"));
//           WARNING(("App passed NULL pointer, ignoring...\n"));
//

#define DBG_VERBOSE 1
#define DBG_TERSE   2
#define DBG_WARNING 3
#define DBG_ERROR   4
#define DBG_RIP     5

#if DBG

extern INT giDebugLevel;

#if defined(KERNEL_MODE) && !defined(USERMODE_DRIVER)

extern VOID DbgPrint(PCSTR, ...);
#define DbgBreakPoint EngDebugBreak

#else

extern ULONG _cdecl DbgPrint(PCSTR, ...);
extern VOID DbgBreakPoint(VOID);

#endif

#define DBGMSG(level, prefix, msg) { \
            if (giDebugLevel <= (level)) { \
                DbgPrint("%s %s (%d): ", prefix, __FILE__, __LINE__); \
                DbgPrint msg; \
            } \
        }

#define DBGPRINT(level, msg) { \
            if (giDebugLevel <= (level)) { \
                DbgPrint msg; \
            } \
        }

#define VERBOSE(msg) DBGPRINT(DBG_VERBOSE, msg)
#define TERSE(msg) DBGPRINT(DBG_TERSE, msg)
#define WARNING(msg) DBGMSG(DBG_WARNING, "WRN", msg)
#define ERR(msg) DBGMSG(DBG_ERROR, "ERR", msg)

#ifndef __MDT__                 // Don't redefine ASSERT when included in MINIDEV.EXE.
#define ASSERT(cond) { \
            if (! (cond)) { \
                RIP(("\n")); \
            } \
        }
#endif

#define ASSERTMSG(cond, msg) { \
            if (! (cond)) { \
                RIP(msg); \
            } \
        }

#define RIP(msg) { \
            DBGMSG(DBG_RIP, "RIP", msg); \
            DbgBreakPoint(); \
        }


#else // !DBG

#define VERBOSE(msg)
#define TERSE(msg)
#define WARNING(msg)
#define ERR(msg)

#ifndef __MDT__                 // Don't redefine ASSERT when included in MINIDEV.EXE.
#define ASSERT(cond)
#endif

#define ASSERTMSG(cond, msg)
#define RIP(msg)
#define DBGMSG(level, prefix, msg)
#define DBGPRINT(level, msg)

#endif

//
// The following macros let you enable tracing on per-file and per-function level.
// To use these macros in a file, here is what you should do:
//
// At the beginning of the file (after header includes):
//
//  Define a bit constant for each function you want to trace
//  Add the following line
//      DEFINE_FUNCTION_TRACE_FLAGS(flags);
//  where flags is a bit-wise OR of the functions you want to trace, e.g.
//      TRACE_FLAG_FUNC1 | TRACE_FLAG_FUNC2 | ...
//
//  To generate trace inside each function you want to trace, use:
//      FUNCTION_TRACE(FunctionTraceFlag, (args));
//

#if DBG

#define DEFINE_FUNCTION_TRACE_FLAGS(flags) \
        static DWORD gdwFunctionTraceFlags = (flags)

#define FUNCTION_TRACE(flag, args) { \
            if (gdwFunctionTraceFlags & (flag)) { \
                DbgPrint args; \
            } \
        }

#else // !DBG

#define DEFINE_FUNCTION_TRACE_FLAGS(flags)
#define FUNCTION_TRACE(flag, args)

#endif // !DBG


//
// Memory allocation function macros
//
#define MemAlloc(size)      ((PVOID) LocalAlloc(LMEM_FIXED, (size)))
#define MemAllocZ(size)     ((PVOID) LocalAlloc(LPTR, (size)))
#define MemFree(p)          { if (p) LocalFree((HLOCAL) (p)); }


//
// DBCS CharSet handling macros
//
//
// 128: SHIFTJIS_CHARSET
// 129: HANGEUL_CHARSET
// 130: JOHAB_CHARSET (defined if WINVER >= 0x0400)
// 134: GB2312_CHARSET
// 136: CHINESEBIG5_CHARSET

#define IS_DBCSCHARSET(j) \
    (((j) == SHIFTJIS_CHARSET)    || \
    ((j) == HANGEUL_CHARSET)     || \
    ((j) == JOHAB_CHARSET)       || \
    ((j) == GB2312_CHARSET)      || \
    ((j) == CHINESEBIG5_CHARSET))

//  932: Japan
//  936: Chinese (PRC, Singapore)
//  949: Korean
//  950: Chinese (China, Hong Kong SAR, Taiwan)
// 1361: Korean (Johab)

#define IS_DBCSCODEPAGE(j) \
    (((j) == 932)   || \
    ((j) == 936)   || \
    ((j) == 949)   || \
    ((j) == 950)   || \
    ((j) == 1361))


//
//  The following are the resource types used in minidrivers and
//  used in the .rc file.
//

#define RC_TABLES      257
#define RC_FONT        258
#define RC_TRANSTAB    259

//
// 5.0 resource types
//

#define RC_UFM         260
#define RC_GTT         261
#define RC_HTPATTERN   264
//
// Internal resource type
//

#define RC_FD_GLYPHSET 262

#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_MINIDRV_H_
