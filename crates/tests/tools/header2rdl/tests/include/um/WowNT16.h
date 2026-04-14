/*++ BUILD Version: 0001    // Increment this if a change has global effects

Copyright (c) 1995-1999, Microsoft Corporation

Module Name:

    wownt16.h

Abstract:

    Procedure declarations for functions in WOW32.DLL callable by
    3rd-party 16-bit thunking code.

--*/

#ifndef _WOWNT16_
#define _WOWNT16_

#if defined(_MSC_VER)
#if _MSC_VER > 1000
#pragma once
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

//
// 16:16 -> 0:32 Pointer translation.
//
// GetVDMPointer32W will convert the passed in 16-bit address
// to the equivalent 32-bit flat pointer. The upper 16 bits
// of the address are treated according to the value passed in
// fMode: if fMode = 1, then the hiword of vp is used as a
// protected mode selector. Otherwise it is used as a real mode
// segment value.
// The lower 16 bits are treated as the offset.
//
// The return value is 0 if the selector is invalid.
//
// NOTE:  Limit checking is not performed in the retail build
// of Windows NT.  It is performed in the checked (debug) build
// of WOW32.DLL, which will cause 0 to be returned when the
// limit is exceeded by the supplied offset.
//

DWORD FAR PASCAL GetVDMPointer32W(LPVOID vp, UINT fMode);


//
// Win32 module management.
//
// The following routines accept parameters that correspond directly
// to the respective Win32 API function calls that they invoke. Refer
// to the Win32 reference documentation for more detail.

DWORD FAR PASCAL LoadLibraryEx32W(LPCSTR lpszLibFile, DWORD hFile, DWORD dwFlags);
DWORD FAR PASCAL GetProcAddress32W(DWORD hModule, LPCSTR lpszProc);
DWORD FAR PASCAL FreeLibrary32W(DWORD hLibModule);

//
// Generic Thunk Routine:
//
//   CallProc32W
//
// Transitions to 32 bits and calls specified routine
//
// This routine can pass a variable number of arguments, up to 32, to the
// target 32-bit routine. These arguments are given to CallProc32W following
// the 3 required parameters.
//
//   DWORD cParams          - Number of optional DWORD parameters (0-32)
//
//   LPVOID fAddressConvert - Bit Field, for 16:16 address Convertion. The
//                            optional parameters can be automatically converted
//                            from a 16:16 address format to flat by specifying
//                            a 1 bit in the corresponding position in this mask.
//                            eg (bit 1 means convert parameter 1 from 16:16
//                              to flat address before calling routine)
//
//   DWORD lpProcAddress   -  32 bit native address to call (use LoadLibraryEx32W
//                            and GetProcAddress32W to get this address).
//
// Returns:
//   What ever the API returned on 32 bit side in AX:DX
//
// Error Returns:
//   AX = 0, more than 32 parameters.
//
//
// The function prototype must be declared by the application source code
// in the following format:
//
// DWORD FAR PASCAL CallProc32W( DWORD p1, ... , DWORD lpProcAddress,
//                                        DWORD fAddressConvert, DWORD cParams);
//
// where the value in cParams must match the actual number of optional
// parameters (p1-pn) given AND the "DWORD p1, ..." must be replaced by
// the correct number of parameters being passed.  For example, passing 3
// parameter would simply require the removal of the ... and it insertion of
// "DWORD p2, DWORD p3" instead.  The fAddressConvert parameter uses bit 1
// for the last parameter (p3 in our example), with bit 2 for the next to last,
// etc.
//
// Generic Thunk Routine:
//
//   CallProcEx32W
//
// Transitions to 32 bits and calls specified routine
//
// Similar to the CallProc32W function, the CallProcEx32W is an equivalent
// function that is C calling convention and allows easier and more flexible
// prototyping.  See the prototype below.  The fAddressConvert parameter uses
// bit 1 for the 1st parameter, bit 2 for the 2nd parameter, etc.
//
// Both CallProc32W and CallProcEx32W accept a flag OR'd with the parameter
// count to indicate the calling convention of the function in 32 bits.
// For example, to call a cdecl function in 32-bits with 1 parameter, it would
// look like this:
//
// dwResult = CallProcEx32W( CPEX_DEST_CDECL | 1, 0, dwfn32, p1 );
//

DWORD FAR CDECL CallProcEx32W( DWORD, DWORD, DWORD, ... );

#define CPEX_DEST_STDCALL   0x00000000L
#define CPEX_DEST_CDECL     0x80000000L

#ifdef __cplusplus
}
#endif

#endif /* !_WOWNT16_ */
