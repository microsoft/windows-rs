/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1985-1999, Microsoft Corporation

Module Name:

    vdmdbg.h

Abstract:

    Prodecure declarations, constant definitions, type definition and macros
    for the VDMDBG.DLL VDM Debugger interface.

--*/

#ifndef _VDMDBG_
#define _VDMDBG_

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus
extern "C" {
#endif

#include <pshpack4.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#endif

#pragma warning(disable:4214) // bit field types other than int
#pragma warning(disable:4121) // structure is sensitive to alignment

#include <vdmctxt.h>

#define STATUS_VDM_EVENT    STATUS_SEGMENT_NOTIFICATION

#ifndef DBG_SEGLOAD
#define DBG_SEGLOAD     0
#define DBG_SEGMOVE     1
#define DBG_SEGFREE     2
#define DBG_MODLOAD     3
#define DBG_MODFREE     4
#define DBG_SINGLESTEP  5
#define DBG_BREAK       6
#define DBG_GPFAULT     7
#define DBG_DIVOVERFLOW 8
#define DBG_INSTRFAULT  9
#define DBG_TASKSTART   10
#define DBG_TASKSTOP    11
#define DBG_DLLSTART    12
#define DBG_DLLSTOP     13
#define DBG_ATTACH      14
#define DBG_TOOLHELP    15
#define DBG_STACKFAULT  16
#define DBG_WOWINIT     17
#define DBG_TEMPBP      18
#define DBG_MODMOVE     19
#define DBG_INIT        20
#define DBG_GPFAULT2    21
#endif

//
// These flags are set in the same WORD as the DBG_ event id's (above).
//
#define VDMEVENT_NEEDS_INTERACTIVE  0x8000
#define VDMEVENT_VERBOSE            0x4000
#define VDMEVENT_PE                 0x2000
#define VDMEVENT_ALLFLAGS           0xe000
//
// These flags are set in the second WORD of the exception event
// parameters.
//
#define VDMEVENT_V86                0x0001
#define VDMEVENT_PM16               0x0002

#define MAX_MODULE_NAME  8 + 1
#define MAX_PATH16      255

typedef struct _SEGMENT_NOTE {
    WORD    Selector1;                      // Selector of operation
    WORD    Selector2;                      // Dest. Sel. for moving segments
    WORD    Segment;                        // Segment within Module
    CHAR    Module[MAX_MODULE_NAME+1];      // Module name
    CHAR    FileName[MAX_PATH16+1];         // PathName to executable image
    WORD    Type;                           // Code / Data, etc.
    DWORD   Length;                         // Length of image
} SEGMENT_NOTE;

typedef struct _IMAGE_NOTE {
    CHAR    Module[MAX_MODULE_NAME+1];      // Module
    CHAR    FileName[MAX_PATH16+1];         // Path to executable image
    WORD    hModule;                        // 16-bit hModule
    WORD    hTask;                          // 16-bit hTask
} IMAGE_NOTE;

typedef struct {
    DWORD   dwSize;
    char    szModule[MAX_MODULE_NAME+1];
    HANDLE  hModule;
    WORD    wcUsage;
    char    szExePath[MAX_PATH16+1];
    WORD    wNext;
} MODULEENTRY, *LPMODULEENTRY;


#define SN_CODE 0                           // Protect mode code segment
#define SN_DATA 1                           // Protect mode data segment
#define SN_V86  2                           // V86 mode segment

typedef struct _TEMP_BP_NOTE {
    WORD    Seg;                            // Dest. Segment or Selector
    DWORD   Offset;                         // Dest. Offset
    BOOL    bPM;                            // TRUE for PM, FALSE for V86
} TEMP_BP_NOTE;

typedef struct _VDM_SEGINFO {
    WORD    Selector;                       // Selector or RM segment
    WORD    SegNumber;                      // Logical segment number in executable
    DWORD   Length;                         // Length of segment
    WORD    Type;                           // Type (0=v86, 1=PM)
    CHAR    ModuleName[MAX_MODULE_NAME];    // Module
    CHAR    FileName[MAX_PATH16];           // Path to executable image
} VDM_SEGINFO;

/* GlobalFirst()/GlobalNext() flags */
#define GLOBAL_ALL      0
#define GLOBAL_LRU      1
#define GLOBAL_FREE     2

/* GLOBALENTRY.wType entries */
#define GT_UNKNOWN      0
#define GT_DGROUP       1
#define GT_DATA         2
#define GT_CODE         3
#define GT_TASK         4
#define GT_RESOURCE     5
#define GT_MODULE       6
#define GT_FREE         7
#define GT_INTERNAL     8
#define GT_SENTINEL     9
#define GT_BURGERMASTER 10

/* If GLOBALENTRY.wType==GT_RESOURCE, the following is GLOBALENTRY.wData: */
#define GD_USERDEFINED      0
#define GD_CURSORCOMPONENT  1
#define GD_BITMAP           2
#define GD_ICONCOMPONENT    3
#define GD_MENU             4
#define GD_DIALOG           5
#define GD_STRING           6
#define GD_FONTDIR          7
#define GD_FONT             8
#define GD_ACCELERATORS     9
#define GD_RCDATA           10
#define GD_ERRTABLE         11
#define GD_CURSOR           12
#define GD_ICON             14
#define GD_NAMETABLE        15
#define GD_MAX_RESOURCE     15

typedef struct {
    DWORD   dwSize;
    DWORD   dwAddress;
    DWORD   dwBlockSize;
    HANDLE  hBlock;
    WORD    wcLock;
    WORD    wcPageLock;
    WORD    wFlags;
    BOOL    wHeapPresent;
    HANDLE  hOwner;
    WORD    wType;
    WORD    wData;
    DWORD   dwNext;
    DWORD   dwNextAlt;
} GLOBALENTRY, *LPGLOBALENTRY;

typedef DWORD (CALLBACK* DEBUGEVENTPROC)( LPDEBUG_EVENT, LPVOID );

// Macros to access VDM_EVENT parameters
#define W1(x) ((USHORT)(x.ExceptionInformation[0]))
#define W2(x) ((USHORT)(x.ExceptionInformation[0] >> 16))
#define W3(x) ((USHORT)(x.ExceptionInformation[1]))
#define W4(x) ((USHORT)(x.ExceptionInformation[1] >> 16))
#define DW3(x) (x.ExceptionInformation[2])
#define DW4(x) (x.ExceptionInformation[3])

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif

#include <poppack.h>


BOOL
WINAPI
VDMProcessException(
    LPDEBUG_EVENT   lpDebugEvent
    );

BOOL
WINAPI
VDMGetThreadSelectorEntry(
    HANDLE          hProcess,
    HANDLE          hThread,
    WORD            wSelector,
    LPVDMLDT_ENTRY  lpSelectorEntry
    );

ULONG
WINAPI
VDMGetPointer(
    HANDLE          hProcess,
    HANDLE          hThread,
    WORD            wSelector,
    DWORD           dwOffset,
    BOOL            fProtMode
    );

// VDMGetThreadContext, VDMSetThreadContext are obselete
// Use VDMGetContext, VDMSetContext
BOOL
WINAPI
VDMGetContext(
    HANDLE          hProcess,
    HANDLE          hThread,
    LPVDMCONTEXT    lpVDMContext
);

BOOL
WINAPI
VDMSetContext(
    HANDLE          hProcess,
    HANDLE          hThread,
    LPVDMCONTEXT    lpVDMContext
);

BOOL
WINAPI
VDMGetSelectorModule(
    HANDLE          hProcess,
    HANDLE          hThread,
    WORD            wSelector,
    PUINT           lpSegmentNumber,
    _In_ LPSTR      lpModuleName,
    UINT            nNameSize,
    _In_ LPSTR      lpModulePath,
    UINT            nPathSize
);

BOOL
WINAPI
VDMGetModuleSelector(
    HANDLE          hProcess,
    HANDLE          hThread,
    UINT            wSegmentNumber,
    _In_ LPSTR      lpModuleName,
    LPWORD          lpSelector
);

BOOL
WINAPI
VDMModuleFirst(
    HANDLE          hProcess,
    HANDLE          hThread,
    LPMODULEENTRY   lpModuleEntry,
    DEBUGEVENTPROC  lpEventProc,
    LPVOID          lpData
);

BOOL
WINAPI
VDMModuleNext(
    HANDLE          hProcess,
    HANDLE          hThread,
    LPMODULEENTRY   lpModuleEntry,
    DEBUGEVENTPROC  lpEventProc,
    LPVOID          lpData
);

BOOL
WINAPI
VDMGlobalFirst(
    HANDLE          hProcess,
    HANDLE          hThread,
    LPGLOBALENTRY   lpGlobalEntry,
    WORD            wFlags,
    DEBUGEVENTPROC  lpEventProc,
    LPVOID          lpData
);

BOOL
WINAPI
VDMGlobalNext(
    HANDLE          hProcess,
    HANDLE          hThread,
    LPGLOBALENTRY   lpGlobalEntry,
    WORD            wFlags,
    DEBUGEVENTPROC  lpEventProc,
    LPVOID          lpData
);

typedef BOOL (WINAPI *PROCESSENUMPROC)( DWORD dwProcessId, DWORD dwAttributes, LPARAM lpUserDefined );
typedef BOOL (WINAPI *TASKENUMPROC)( DWORD dwThreadId, WORD hMod16, WORD hTask16, LPARAM lpUserDefined );
typedef BOOL (WINAPI *TASKENUMPROCEX)( DWORD dwThreadId, WORD hMod16, WORD hTask16,
                                       PSZ pszModName, PSZ pszFileName, LPARAM lpUserDefined );

#define WOW_SYSTEM  (DWORD)0x0001

INT
WINAPI
VDMEnumProcessWOW(
    PROCESSENUMPROC fp,
    LPARAM          lparam
);

INT
WINAPI
VDMEnumTaskWOW(
    DWORD           dwProcessId,
    TASKENUMPROC    fp,
    LPARAM          lparam
);

//
// VDMEnumTaskWOWEx is the same as VDMEnumTaskWOW except
// the callback procedure gets two more parameters,
// the module name of the EXE and the full path to the
// EXE.
//

INT
WINAPI
VDMEnumTaskWOWEx(
    DWORD           dwProcessId,
    TASKENUMPROCEX  fp,
    LPARAM          lparam
);

//
// VDMTerminateTaskWOW rudely terminates a 16-bit WOW task
// similar to the way TerminateProcess kills a Win32
// process.
//

BOOL
WINAPI
VDMTerminateTaskWOW(
    DWORD           dwProcessId,
    WORD            htask
);

//
// VDMStartTaskInWOW launches a Win16 task in a pre-existing
// WOW VDM.  Note that the caller is responsible for ensuring
// the program is a 16-bit Windows program.  If it is a DOS
// or Win32 program, it will still be launched from within
// the target WOW VDM.
//
// The supplied command line and show command are passed
// unchanged to the 16-bit WinExec API in the target WOW VDM.
//
// Note this routine is ANSI-only.
//

BOOL
WINAPI
VDMStartTaskInWOW(
    DWORD           dwProcessId,
    _In_ LPSTR      lpCommandLine,
    WORD            wShow
);

//
// VDMKillWOW is not implemented.
//

BOOL
WINAPI
VDMKillWOW(
    VOID
);

//
// VDMDetectWOW is not implemented.
//

BOOL
WINAPI
VDMDetectWOW(
    VOID
);

BOOL
WINAPI
VDMBreakThread(
    HANDLE          hProcess,
    HANDLE          hThread
);

DWORD
WINAPI
VDMGetDbgFlags(
    HANDLE          hProcess
    );

BOOL
WINAPI
VDMSetDbgFlags(
    HANDLE          hProcess,
    DWORD           dwFlags
    );

#define VDMDBG_BREAK_DOSTASK      0x00000001
#define VDMDBG_BREAK_WOWTASK      0x00000002
#define VDMDBG_BREAK_LOADDLL      0x00000004
#define VDMDBG_BREAK_EXCEPTIONS   0x00000008
#define VDMDBG_BREAK_DEBUGGER     0x00000010
#define VDMDBG_TRACE_HISTORY      0x00000080
#define VDMDBG_BREAK_DIVIDEBYZERO 0x00000100

#define VDMDBG_INITIAL_FLAGS      VDMDBG_BREAK_DIVIDEBYZERO

//
// VDMIsModuleLoaded can be used to determine if the 16-bit
// executable referenced by the full path name parameter is
// loaded in ntvdm.
//
// Note that this function uses an internal table in vdmdbg.dll
// to determine a module's existence. One important usage of this
// function is to print a message when a particular module is
// loaded for the first time. To accomplish this, call this
// routine during a DBG_SEGLOAD notification BEFORE the entry
// point VDMProcessException has been called. If it returns FALSE,
// then the module has not yet been loaded.
//
BOOL
WINAPI
VDMIsModuleLoaded(
    _In_ LPSTR szPath
    );

BOOL
WINAPI
VDMGetSegmentInfo(
    WORD Selector,
    ULONG Offset,
    BOOL bProtectMode,
    VDM_SEGINFO *pSegInfo
    );

#define VDMDBG_MAX_SYMBOL_BUFFER 256

//
// VDMGetSymbol
//
// This routine reads the standard .SYM file format.
//
// szModule         - module name (max 9 chars)
// SegNumber        - logical segment number of segment (see VDM_SEGINFO)
// Offset           - offset in segment
// bProtectMode     - TRUE for PM, FALSE for V86 mode
// bNextSymbol      - FALSE to find nearest sym BEFORE offset, TRUE for AFTER
// szSymbolName     - receives symbol name (must point to 256 byte buffer)
// pDisplacement    - distance in bytes from nearest symbol
//

_Success_(return != FALSE)
BOOL
WINAPI
VDMGetSymbol(
    _In_ LPSTR szModule,
    WORD SegNumber,
    DWORD Offset,
    BOOL bProtectMode,
    BOOL bNextSymbol,
    _Out_writes_(VDMDBG_MAX_SYMBOL_BUFFER) LPSTR szSymbolName,
    PDWORD pDisplacement
    );

BOOL
WINAPI
VDMGetAddrExpression(
    _In_ LPSTR  szModule,
    _In_ LPSTR  szSymbol,
    PWORD  Selector,
    PDWORD Offset,
    PWORD  Type
    );

#define VDMADDR_V86     2
#define VDMADDR_PM16    4
#define VDMADDR_PM32   16

//
// typedefs for main entry points
//

typedef BOOL  (WINAPI *VDMPROCESSEXCEPTIONPROC)(LPDEBUG_EVENT);
typedef BOOL  (WINAPI *VDMGETTHREADSELECTORENTRYPROC)(HANDLE,HANDLE,DWORD,LPVDMLDT_ENTRY);
typedef ULONG (WINAPI *VDMGETPOINTERPROC)(HANDLE,HANDLE,WORD,DWORD,BOOL);
typedef BOOL  (WINAPI *VDMGETCONTEXTPROC)(HANDLE,HANDLE,LPVDMCONTEXT);
typedef BOOL  (WINAPI *VDMSETCONTEXTPROC)(HANDLE,HANDLE,LPVDMCONTEXT);
typedef BOOL  (WINAPI *VDMKILLWOWPROC)(VOID);
typedef BOOL  (WINAPI *VDMDETECTWOWPROC)(VOID);
typedef BOOL  (WINAPI *VDMBREAKTHREADPROC)(HANDLE);
typedef BOOL  (WINAPI *VDMGETSELECTORMODULEPROC)(HANDLE,HANDLE,WORD,PUINT,LPSTR, UINT,LPSTR, UINT);
typedef BOOL  (WINAPI *VDMGETMODULESELECTORPROC)(HANDLE,HANDLE,UINT,LPSTR,LPWORD);
typedef BOOL  (WINAPI *VDMMODULEFIRSTPROC)(HANDLE,HANDLE,LPMODULEENTRY,DEBUGEVENTPROC,LPVOID);
typedef BOOL  (WINAPI *VDMMODULENEXTPROC)(HANDLE,HANDLE,LPMODULEENTRY,DEBUGEVENTPROC,LPVOID);
typedef BOOL  (WINAPI *VDMGLOBALFIRSTPROC)(HANDLE,HANDLE,LPGLOBALENTRY,WORD,DEBUGEVENTPROC,LPVOID);
typedef BOOL  (WINAPI *VDMGLOBALNEXTPROC)(HANDLE,HANDLE,LPGLOBALENTRY,WORD,DEBUGEVENTPROC,LPVOID);

typedef INT   (WINAPI *VDMENUMPROCESSWOWPROC)(PROCESSENUMPROC,LPARAM);
typedef INT   (WINAPI *VDMENUMTASKWOWPROC)(DWORD,TASKENUMPROC,LPARAM);
typedef INT   (WINAPI *VDMENUMTASKWOWEXPROC)(DWORD,TASKENUMPROCEX,LPARAM);
typedef BOOL  (WINAPI *VDMTERMINATETASKINWOWPROC)(DWORD,WORD);
typedef BOOL  (WINAPI *VDMSTARTTASKINWOWPROC)(DWORD,LPSTR,WORD);

typedef DWORD (WINAPI *VDMGETDBGFLAGSPROC)(HANDLE);
typedef BOOL  (WINAPI *VDMSETDBGFLAGSPROC)(HANDLE,DWORD);
typedef BOOL  (WINAPI *VDMISMODULELOADEDPROC)(LPSTR);
typedef BOOL  (WINAPI *VDMGETSEGMENTINFOPROC)(WORD,ULONG,BOOL,VDM_SEGINFO);
typedef BOOL  (WINAPI *VDMGETSYMBOLPROC)(_In_ LPSTR, WORD, DWORD, BOOL, BOOL, _Out_writes_(VDMDBG_MAX_SYMBOL_BUFFER) LPSTR, PDWORD);
typedef BOOL  (WINAPI *VDMGETADDREXPRESSIONPROC)(LPSTR, LPSTR, PWORD, PDWORD, PWORD);


#ifdef __cplusplus
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // _VDMDBG_
