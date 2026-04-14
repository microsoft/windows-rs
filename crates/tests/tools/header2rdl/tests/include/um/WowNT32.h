/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1995-1999, Microsoft Corporation

Module Name:

    wownt32.h

Abstract:

    Procedure declarations for functions in WOW32.DLL callable by
    3rd-party 32-bit thunking code.

--*/

#ifndef _WOWNT32_
#define _WOWNT32_

#if defined(_MSC_VER)
#if _MSC_VER > 1000
#pragma once
#endif
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

//
// 16:16 -> 0:32 Pointer translation.
//
// WOWGetVDMPointer will convert the passed in 16-bit address
// to the equivalent 32-bit flat pointer.  If fProtectedMode
// is TRUE, the function treats the upper 16 bits as a selector
// in the local descriptor table.  If fProtectedMode is FALSE,
// the upper 16 bits are treated as a real-mode segment value.
// In either case the lower 16 bits are treated as the offset.
//
// The return value is NULL if the selector is invalid.
//
// NOTE:  Limit checking is not performed in the retail build
// of Windows NT.  It is performed in the checked (debug) build
// of WOW32.DLL, which will cause NULL to be returned when the
// limit is exceeded by the supplied offset.
//

typedef LPVOID (WINAPI *PFNWOWGETVDMPOINTER)(DWORD vp, DWORD dwBytes, BOOL fProtectedMode);

LPVOID WINAPI WOWGetVDMPointer(DWORD vp, DWORD dwBytes, BOOL fProtectedMode);

//
// The following two functions are here for compatibility with
// Windows 95.  On Win95, the global heap can be rearranged,
// invalidating flat pointers returned by WOWGetVDMPointer, while
// a thunk is executing.  On Windows NT, the 16-bit VDM is completely
// halted while a thunk executes, so the only way the heap will
// be rearranged is if a callback is made to Win16 code.
//
// The Win95 versions of these functions call GlobalFix to
// lock down a segment's flat address, and GlobalUnfix to
// release the segment.
//
// The Windows NT implementations of these functions do *not*
// call GlobalFix/GlobalUnfix on the segment, because there
// will not be any heap motion unless a callback occurs.
// If your thunk does callback to the 16-bit side, be sure
// to discard flat pointers and call WOWGetVDMPointer again
// to be sure the flat address is correct.
//

LPVOID WINAPI WOWGetVDMPointerFix(DWORD vp, DWORD dwBytes,
                                  BOOL fProtectedMode);
VOID WINAPI WOWGetVDMPointerUnfix(DWORD vp);


//
// Win16 memory management.
//
// These functions can be used to manage memory in the Win16
// heap.  The following four functions are identical to their
// Win16 counterparts, except that they are called from Win32
// code.
//

// Interestingly, Alloc16 does not seem to be used anywhere. 
// typedef WORD  (WINAPI *PFNWOWGLOBALALLOC16)(WORD wFlags, DWORD cb);

typedef WORD  (WINAPI *PFNWOWGLOBALFREE16)(WORD hMem);
typedef DWORD (WINAPI *PFNWOWGLOBALLOCK16)(WORD hMem);
typedef BOOL  (WINAPI *PFNWOWGLOBALUNLOCK16)(WORD hMem);

WORD  WINAPI WOWGlobalAlloc16(WORD wFlags, DWORD cb);
WORD  WINAPI WOWGlobalFree16(WORD hMem);
DWORD WINAPI WOWGlobalLock16(WORD hMem);
BOOL  WINAPI WOWGlobalUnlock16(WORD hMem);

//
// The following three functions combine two common operations in
// one switch to 16-bit mode.
//

typedef DWORD (WINAPI *PFNWOWGLOBALALLOCLOCK16)(WORD wFlag, DWORD cb, WORD *phMem);
typedef WORD  (WINAPI *PFNWOWGLBALUNLOCKFREE16)(DWORD vpMem);
typedef DWORD (WINAPI *PFNWOWGLOBALLOCKSIZE16)(WORD hMem, PDWORD pcb);

DWORD WINAPI WOWGlobalAllocLock16(WORD wFlags, DWORD cb, WORD *phMem);
WORD  WINAPI WOWGlobalUnlockFree16(DWORD vpMem);
DWORD WINAPI WOWGlobalLockSize16(WORD hMem, PDWORD pcb);

//
// Yielding the Win16 nonpreemptive scheduler
//
// The following two functions are provided for Win32 code called
// via Generic Thunks which needs to yield the Win16 scheduler so
// that tasks in that VDM can execute while the thunk waits for
// something to complete.  These two functions are functionally
// identical to calling back to 16-bit code which calls Yield or
// DirectedYield.
//

typedef VOID (WINAPI *PFNWOWYIELD16)(VOID);
typedef VOID (WINAPI *PFNWOWDIRECTEDYIELD16)(WORD htask16);

VOID WINAPI WOWYield16(VOID);
VOID WINAPI WOWDirectedYield16(WORD htask16);


//
// 16 <--> 32 Handle mapping functions.
//
// NOTE:  While some of these functions perform trivial
// conversions, these functions must be used to maintain
// compatibility with future versions of Windows NT which
// may require different handle mapping.
//

typedef enum _WOW_HANDLE_TYPE { /* WOW */
    WOW_TYPE_HWND,
    WOW_TYPE_HMENU,
    WOW_TYPE_HDWP,
    WOW_TYPE_HDROP,
    WOW_TYPE_HDC,
    WOW_TYPE_HFONT,
    WOW_TYPE_HMETAFILE,
    WOW_TYPE_HRGN,
    WOW_TYPE_HBITMAP,
    WOW_TYPE_HBRUSH,
    WOW_TYPE_HPALETTE,
    WOW_TYPE_HPEN,
    WOW_TYPE_HACCEL,
    WOW_TYPE_HTASK,
    WOW_TYPE_FULLHWND
} WOW_HANDLE_TYPE;

typedef HANDLE (WINAPI *PFNWOWHANDLE32)(WORD, WOW_HANDLE_TYPE);
typedef WORD   (WINAPI *PFNWOWHANDLE16)(HANDLE, WOW_HANDLE_TYPE);


HANDLE WINAPI WOWHandle32 (WORD, WOW_HANDLE_TYPE);
WORD WINAPI WOWHandle16 (HANDLE, WOW_HANDLE_TYPE);

#ifndef REGISTERED_WOW_HANDLERS
#define WOWHANDLE32 WOWHandle32
#define WOWHANDLE16 WOWHandle16
#else
#define WOWHANDLE32 REGISTERED_WOW_HANDLERS.pfnWOWHandle32
#define WOWHANDLE16 REGISTERED_WOW_HANDLERS.pfnWOWHandle16
#endif

#define HWND_32(h16)      ((HWND)      (WOWHANDLE32(h16, WOW_TYPE_HWND)))
#define HMENU_32(h16)     ((HMENU)     (WOWHANDLE32(h16, WOW_TYPE_HMENU)))
#define HDWP_32(h16)      ((HDWP)      (WOWHANDLE32(h16, WOW_TYPE_HDWP)))
#define HDROP_32(h16)     ((HDROP)     (WOWHANDLE32(h16, WOW_TYPE_HDROP)))
#define HDC_32(h16)       ((HDC)       (WOWHANDLE32(h16, WOW_TYPE_HDC)))
#define HFONT_32(h16)     ((HFONT)     (WOWHANDLE32(h16, WOW_TYPE_HFONT)))
#define HMETAFILE_32(h16) ((HMETAFILE) (WOWHANDLE32(h16, WOW_TYPE_HMETAFILE)))
#define HRGN_32(h16)      ((HRGN)      (WOWHANDLE32(h16, WOW_TYPE_HRGN)))
#define HBITMAP_32(h16)   ((HBITMAP)   (WOWHANDLE32(h16, WOW_TYPE_HBITMAP)))
#define HBRUSH_32(h16)    ((HBRUSH)    (WOWHANDLE32(h16, WOW_TYPE_HBRUSH)))
#define HPALETTE_32(h16)  ((HPALETTE)  (WOWHANDLE32(h16, WOW_TYPE_HPALETTE)))
#define HPEN_32(h16)      ((HPEN)      (WOWHANDLE32(h16, WOW_TYPE_HPEN)))
#define HACCEL_32(h16)	  ((HACCEL)    (WOWHANDLE32(h16, WOW_TYPE_HACCEL)))
#define HTASK_32(h16)	  ((DWORD)     (WOWHANDLE32(h16, WOW_TYPE_HTASK)))
#define FULLHWND_32(h16)  ((HWND)      (WOWHANDLE32(h16, WOW_TYPE_FULLHWND)))

#define HWND_16(h32)      (WOWHANDLE16(h32, WOW_TYPE_HWND))
#define HMENU_16(h32)     (WOWHANDLE16(h32, WOW_TYPE_HMENU))
#define HDWP_16(h32)      (WOWHANDLE16(h32, WOW_TYPE_HDWP))
#define HDROP_16(h32)     (WOWHANDLE16(h32, WOW_TYPE_HDROP))
#define HDC_16(h32)       (WOWHANDLE16(h32, WOW_TYPE_HDC))
#define HFONT_16(h32)     (WOWHANDLE16(h32, WOW_TYPE_HFONT))
#define HMETAFILE_16(h32) (WOWHANDLE16(h32, WOW_TYPE_HMETAFILE))
#define HRGN_16(h32)      (WOWHANDLE16(h32, WOW_TYPE_HRGN))
#define HBITMAP_16(h32)   (WOWHANDLE16(h32, WOW_TYPE_HBITMAP))
#define HBRUSH_16(h32)    (WOWHANDLE16(h32, WOW_TYPE_HBRUSH))
#define HPALETTE_16(h32)  (WOWHANDLE16(h32, WOW_TYPE_HPALETTE))
#define HPEN_16(h32)      (WOWHANDLE16(h32, WOW_TYPE_HPEN))
#define HACCEL_16(h32)	  (WOWHANDLE16(h32, WOW_TYPE_HACCEL))
#define HTASK_16(h32)	  (WOWHANDLE16(h32, WOW_TYPE_HTASK))

//
// Generic Callbacks.
//
// WOWCallback16 can be used in Win32 code called
// from 16-bit (such as by using Generic Thunks) to call back to
// the 16-bit side.  The function called must be declared similarly
// to the following:
//
// LONG FAR PASCAL CallbackRoutine(DWORD dwParam);
//
// If you are passing a pointer, declare the parameter as such:
//
// LONG FAR PASCAL CallbackRoutine(VOID FAR *vp);
//
// NOTE: If you are passing a pointer, you'll need to get the
// pointer using WOWGlobalAlloc16 or WOWGlobalAllocLock16
//
// If the function called returns a WORD instead of a DWORD, the
// upper 16 bits of the return value is undefined.  Similarly, if
// the function called has no return value, the entire return value
// is undefined.
//
// WOWCallback16Ex allows any combination of arguments up to
// WCB16_MAX_CBARGS bytes total to be passed to the 16-bit routine.
// cbArgs is used to properly clean up the 16-bit stack after calling
// the routine.  Regardless of the value of cbArgs, WCB16_MAX_CBARGS
// bytes will always be copied from pArgs to the 16-bit stack.  If
// pArgs is less than WCB16_MAX_CBARGS bytes from the end of a page,
// and the next page is inaccessible, WOWCallback16Ex will incur an
// access violation.
//
// If cbArgs is larger than the WCB16_MAX_ARGS which the running
// system supports, the function returns FALSE and GetLastError
// returns ERROR_INVALID_PARAMETER.  Otherwise the function
// returns TRUE and the DWORD pointed to by pdwRetCode contains
// the return code from the callback routine.  If the callback
// routine returns a WORD, the HIWORD of the return code is
// undefined and should be ignored using LOWORD(dwRetCode).
//
// WOWCallback16Ex can call routines using the PASCAL and CDECL
// calling conventions.  The default is to use the PASCAL
// calling convention.  To use CDECL, pass WCB16_CDECL in the
// dwFlags parameter.
//
// The arguments pointed to by pArgs must be in the correct
// order for the callback routine's calling convention.
// To call the PASCAL routine SetWindowText,
//
// LONG FAR PASCAL SetWindowText(HWND hwnd, LPCSTR lpsz);
//
// pArgs would point to an array of words:
//
// WORD SetWindowTextArgs[] = {OFFSETOF(lpsz), SELECTOROF(lpsz), hwnd};
//
// In other words, the arguments are placed in the array in reverse
// order with the least significant word first for DWORDs and offset
// first for FAR pointers.
//
// To call the CDECL routine wsprintf, for example
//
// LPSTR lpszFormat = "%d %s";
// int __cdecl wsprintf(lpsz, lpszFormat, nValue. lpszString);
//
// pArgs would point to the array:
//
// WORD wsprintfArgs[] = {OFFSETOF(lpsz), SELECTOROF(lpsz),
//                        OFFSETOF(lpszFormat), SELECTOROF(lpszFormat),
//                        nValue,
//                        OFFSETOF(lpszString), SELECTOROF(lpszString)};
//
// In other words, the arguments are placed in the array in the order
// listed in the function prototype with the least significant word
// first for DWORDs and offset first for FAR pointers.
//

typedef DWORD (WINAPI *PFNWOWCALLBACK16)(DWORD vpfn16, DWORD dwParam);
typedef BOOL  (WINAPI *PFNWOWCALLBACK16EX)(DWORD  vpfn16,
                                           DWORD  dwFlags,
                                           DWORD  cbArgs,
                                           PVOID  pArgs,
                                           PDWORD pdwRetCode);

DWORD WINAPI WOWCallback16(DWORD vpfn16, DWORD dwParam);

#define WCB16_MAX_CBARGS (16)

#define WCB16_PASCAL     (0x0)
#define WCB16_CDECL      (0x1)

BOOL WINAPI WOWCallback16Ex(
                DWORD  vpfn16,
                DWORD  dwFlags,
                DWORD  cbArgs,
                PVOID  pArgs,
                PDWORD pdwRetCode
                );

// For OLETHK32

typedef BOOL (WINAPI *PFNWOWFREEMETAFILE)( HANDLE h32 );
BOOL WINAPI WOWFreeMetafile( HANDLE h32 );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif /* !_WOWNT32_ */
