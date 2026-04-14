/*++

Copyright (c) Microsoft Corporation.  All rights reserved.

Module Name:

    lzexpand.h

Abstract:

    Public interface to LZEXP?.LIB.

Author:


Revision History:

--*/

#ifndef _LZEXPAND_
#define _LZEXPAND_

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

/*
** Error Return Codes
*/

#define LZERROR_BADINHANDLE   (-1)  /* invalid input handle */
#define LZERROR_BADOUTHANDLE  (-2)  /* invalid output handle */
#define LZERROR_READ          (-3)  /* corrupt compressed file format */
#define LZERROR_WRITE         (-4)  /* out of space for output file */
#define LZERROR_GLOBALLOC     (-5)  /* insufficient memory for LZFile struct */
#define LZERROR_GLOBLOCK      (-6)  /* bad global handle */
#define LZERROR_BADVALUE      (-7)  /* input parameter out of acceptable range*/
#define LZERROR_UNKNOWNALG    (-8)  /* compression algorithm not recognized */


/*
** Prototypes
*/

_Success_(return >= 0)
_Check_return_
INT
APIENTRY
LZStart(
    VOID
    );

VOID
APIENTRY
LZDone(
    VOID
    );


_Success_(return >= 0)
_Check_return_
LONG
APIENTRY
CopyLZFile(
    _In_ INT hfSource,
    _In_ INT hfDest
    );

_Success_(return >= 0)
_Check_return_
LONG
APIENTRY
LZCopy(
    _In_ INT hfSource,
    _In_ INT hfDest
    );

_Success_(return >= 0)
_Check_return_
INT
APIENTRY
LZInit(
    _In_ INT hfSource
    );

_Success_(return >= 0)
_Check_return_
INT
APIENTRY
GetExpandedNameA(
    _In_ LPSTR lpszSource,
    _Out_writes_(MAX_PATH) LPSTR lpszBuffer
    );
_Success_(return >= 0)
_Check_return_
INT
APIENTRY
GetExpandedNameW(
    _In_ LPWSTR lpszSource,
    _Out_writes_(MAX_PATH) LPWSTR lpszBuffer
    );
#ifdef UNICODE
#define GetExpandedName  GetExpandedNameW
#else
#define GetExpandedName  GetExpandedNameA
#endif // !UNICODE

_Success_(return >= 0)
_Check_return_
INT
APIENTRY
LZOpenFileA(
    _In_ LPSTR lpFileName,
    _Inout_ LPOFSTRUCT lpReOpenBuf,
    _In_ WORD wStyle
    );
_Success_(return >= 0)
_Check_return_
INT
APIENTRY
LZOpenFileW(
    _In_ LPWSTR lpFileName,
    _Inout_ LPOFSTRUCT lpReOpenBuf,
    _In_ WORD wStyle
    );
#ifdef UNICODE
#define LZOpenFile  LZOpenFileW
#else
#define LZOpenFile  LZOpenFileA
#endif // !UNICODE

_Success_(return >= 0)
_Check_return_
LONG
APIENTRY
LZSeek(
    _In_ INT hFile,
    _In_ LONG lOffset,
    _In_ INT iOrigin
    );

_Success_(return >= 0)
_Check_return_
INT
APIENTRY
LZRead(
    _In_ INT hFile,
    _Out_writes_bytes_to_(cbRead, return) CHAR* lpBuffer,
    _In_ INT cbRead
    );

VOID
APIENTRY
LZClose(
    _In_ INT hFile
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif


#endif // _LZEXPAND_
