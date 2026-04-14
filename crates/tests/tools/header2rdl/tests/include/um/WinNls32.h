/*++ BUILD Version: 0003    // Increment this if a change has global effects

Copyright (c) 1985-1998, Microsoft Corporation

Module Name:

    winnls32.h

Abstract:

    Procedure declarations, constant definitions and macros for the
    Windows NT 3.x compatible FarEast IMM component.

--*/

#ifndef _WINNLS32_
#define _WINNLS32_

#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef struct _tagDATETIME {
    WORD    year;
    WORD    month;
    WORD    day;
    WORD    hour;
    WORD    min;
    WORD    sec;
} DATETIME;

typedef struct _tagIMEPROA {
    HWND        hWnd;
    DATETIME    InstDate;
    UINT        wVersion;
    BYTE        szDescription[50];
    BYTE        szName[80];
    BYTE        szOptions[30];
} IMEPROA,*PIMEPROA,NEAR *NPIMEPROA,FAR *LPIMEPROA;
typedef struct _tagIMEPROW {
    HWND        hWnd;
    DATETIME    InstDate;
    UINT        wVersion;
    WCHAR       szDescription[50];
    WCHAR       szName[80];
    WCHAR       szOptions[30];
} IMEPROW,*PIMEPROW,NEAR *NPIMEPROW,FAR *LPIMEPROW;
#ifdef UNICODE
typedef IMEPROW IMEPRO;
typedef PIMEPROW PIMEPRO;
typedef NPIMEPROW NPIMEPRO;
typedef LPIMEPROW LPIMEPRO;
#else
typedef IMEPROA IMEPRO;
typedef PIMEPROA PIMEPRO;
typedef NPIMEPROA NPIMEPRO;
typedef LPIMEPROA LPIMEPRO;
#endif // UNICODE

BOOL  WINAPI IMPGetIMEA( IN HWND, OUT LPIMEPROA);
BOOL  WINAPI IMPGetIMEW( IN HWND, OUT LPIMEPROW);
#ifdef UNICODE
#define IMPGetIME  IMPGetIMEW
#else
#define IMPGetIME  IMPGetIMEA
#endif // !UNICODE
BOOL  WINAPI IMPQueryIMEA( IN OUT LPIMEPROA);
BOOL  WINAPI IMPQueryIMEW( IN OUT LPIMEPROW);
#ifdef UNICODE
#define IMPQueryIME  IMPQueryIMEW
#else
#define IMPQueryIME  IMPQueryIMEA
#endif // !UNICODE
BOOL  WINAPI IMPSetIMEA( IN HWND, IN LPIMEPROA);
BOOL  WINAPI IMPSetIMEW( IN HWND, IN LPIMEPROW);
#ifdef UNICODE
#define IMPSetIME  IMPSetIMEW
#else
#define IMPSetIME  IMPSetIMEA
#endif // !UNICODE

UINT  WINAPI WINNLSGetIMEHotkey( IN HWND);
BOOL  WINAPI WINNLSEnableIME( IN HWND, IN BOOL);
BOOL  WINAPI WINNLSGetEnableStatus( IN HWND);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#ifdef __cplusplus
}
#endif  /* __cplusplus */

#endif // _WINNLS32_

