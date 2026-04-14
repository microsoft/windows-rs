
/*****************************************************************************\
*                                                                             *
* shlwapi.h - Interface for the Windows light-weight utility APIs             *
*                                                                             *
* Version 1.0                                                                 *
*                                                                             *
* Copyright (c) Microsoft Corporation. All rights reserved.                   *
*                                                                             *
\*****************************************************************************/


#ifndef _INC_SHLWAPI
#define _INC_SHLWAPI

#include <winapifamily.h>


#ifndef NOSHLWAPI

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#include <SpecStrings.h>
#include <objbase.h>
#include <shtypes.h>



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

//
// Define API decoration for direct importing of DLL references.
//
#ifndef WINSHLWAPI
#if !defined(_SHLWAPI_)
#define LWSTDAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define LWSTDAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#define LWSTDAPIV         EXTERN_C DECLSPEC_IMPORT HRESULT STDAPIVCALLTYPE
#define LWSTDAPIV_(type)  EXTERN_C DECLSPEC_IMPORT type STDAPIVCALLTYPE
#else
#define LWSTDAPI          STDAPI
#define LWSTDAPI_(type)   STDAPI_(type)
#define LWSTDAPIV         STDAPIV
#define LWSTDAPIV_(type)  STDAPIV_(type)
#endif
#endif // WINSHLWAPI

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4820) // padding added after data member
#endif

#ifdef _WIN32
#include <pshpack8.h>
#endif

#if defined(DEPRECATE_SUPPORTED)
#pragma warning(push)
#pragma warning(disable:4995)
#endif

// objidl.h
#ifndef __IBindCtx_FWD_DEFINED__
#define __IBindCtx_FWD_DEFINED__
typedef interface IBindCtx IBindCtx;
#endif  /* __IBindCtx_FWD_DEFINED__ */

#ifdef __cplusplus
extern "C" {
#endif

#if defined(__cplusplus) && defined(STRICT_CONST)
#define USE_STRICT_CONST
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


//
// Users of this header may define any number of these constants to avoid
// the definitions of each functional group.
//
//    NO_SHLWAPI_STRFCNS    String functions
//    NO_SHLWAPI_PATH       Path functions
//    NO_SHLWAPI_REG        Registry functions
//    NO_SHLWAPI_STREAM     Stream functions
//    NO_SHLWAPI_GDI        GDI helper functions

#ifndef NO_SHLWAPI_STRFCNS
//
//=============== String Routines ===================================
//

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#ifdef USE_STRICT_CONST
LWSTDAPI_(PCSTR)    StrChrA(_In_ PCSTR pszStart, WORD wMatch);
LWSTDAPI_(PCWSTR)   StrChrW(_In_ PCWSTR pszStart, WCHAR wMatch);
LWSTDAPI_(PCSTR)    StrChrIA(_In_ PCSTR pszStart, WORD wMatch);
LWSTDAPI_(PCWSTR)   StrChrIW(_In_ PCWSTR pszStart, WCHAR wMatch);
#if (_WIN32_IE >= _WIN32_IE_IE60)
LWSTDAPI_(PCWSTR)   StrChrNW(_In_ PCWSTR pszStart, WCHAR wMatch, UINT cchMax);
LWSTDAPI_(PCWSTR)   StrChrNIW(_In_ PCWSTR pszStart, WCHAR wMatch, UINT cchMax);
#endif // _WIN32_IE_IE60
#else
LWSTDAPI_(PSTR)     StrChrA(_In_ PCSTR pszStart, WORD wMatch);
LWSTDAPI_(PWSTR)    StrChrW(_In_ PCWSTR pszStart, WCHAR wMatch);
LWSTDAPI_(PSTR)     StrChrIA(_In_ PCSTR pszStart, WORD wMatch);
LWSTDAPI_(PWSTR)    StrChrIW(_In_ PCWSTR pszStart, WCHAR wMatch);
#if (_WIN32_IE >= _WIN32_IE_IE60)
LWSTDAPI_(PWSTR)    StrChrNW(_In_ PCWSTR pszStart, WCHAR wMatch, UINT cchMax);
LWSTDAPI_(PWSTR)    StrChrNIW(_In_ PCWSTR pszStart, WCHAR wMatch, UINT cchMax);
#endif // _WIN32_IE_IE60
#endif
LWSTDAPI_(int)      StrCmpNA(_In_ PCSTR psz1, _In_ PCSTR psz2, int nChar);
LWSTDAPI_(int)      StrCmpNW(_In_ PCWSTR psz1, _In_ PCWSTR psz2, int nChar);
LWSTDAPI_(int)      StrCmpNIA(_In_ PCSTR psz1, _In_ PCSTR psz2, int nChar);
LWSTDAPI_(int)      StrCmpNIW(_In_ PCWSTR psz1, _In_ PCWSTR psz2, int nChar);
LWSTDAPI_(int)      StrCSpnA(_In_ PCSTR pszStr, _In_ PCSTR pszSet);
LWSTDAPI_(int)      StrCSpnW(_In_ PCWSTR pszStr, _In_ PCWSTR pszSet);
LWSTDAPI_(int)      StrCSpnIA(_In_ PCSTR pszStr, _In_ PCSTR pszSet);
LWSTDAPI_(int)      StrCSpnIW(_In_ PCWSTR pszStr, _In_ PCWSTR pszSet);
LWSTDAPI_(PSTR)     StrDupA(_In_ PCSTR pszSrch);
LWSTDAPI_(PWSTR)    StrDupW(_In_ PCWSTR pszSrch);

#if (NTDDI_VERSION >= NTDDI_VISTASP1)
// StrFormatByteSizeEx takes a ULONGLONG as a byte count and formats a string
// representing that number of bytes in an appropriately concise manner, where
// "appropriate manner" is determine by several factors:
//
// 1) order - is this most appropriately expressed as KB? MB? GB?
//    for example: 1039 -> "1.01 KB", 5454608466 -> "5.08 GB", etc
//
// 2) number of whole number places shown - if there are more than a few whole
//    number places to display, decimal places are omitted.
//    for example: 1024 -> "1.00 KB", 12288 -> "12.0 KB", 125952 -> "123 KB"
//
// 3) the caller can specify whether the result should involve rounding to the
//    nearest displayed digit, or truncation of undisplayed digits. the caller
//    must specify either rounding or truncation when calling the API.
//    for example: with rounding,   2147483647 -> "2.00 GB"
//                 with truncation, 2147483647 -> "1.99 GB"

enum tagSFBS_FLAGS
{
    SFBS_FLAGS_ROUND_TO_NEAREST_DISPLAYED_DIGIT     = 0x0001,   // round to the nearest displayed digit
    SFBS_FLAGS_TRUNCATE_UNDISPLAYED_DECIMAL_DIGITS  = 0x0002,   // discard undisplayed digits
};
typedef int SFBS_FLAGS;

LWSTDAPI            StrFormatByteSizeEx(ULONGLONG ull, SFBS_FLAGS flags, _Out_writes_(cchBuf) PWSTR pszBuf, _In_range_(>,0) UINT cchBuf);
#endif // (NTDDI_VERSION >= NTDDI_VISTASP1)

LWSTDAPI_(PSTR)     StrFormatByteSizeA(DWORD dw, _Out_writes_(cchBuf) PSTR pszBuf, UINT cchBuf);
LWSTDAPI_(PSTR)     StrFormatByteSize64A(LONGLONG qdw, _Out_writes_(cchBuf) PSTR pszBuf, UINT cchBuf);
LWSTDAPI_(PWSTR)    StrFormatByteSizeW(LONGLONG qdw, _Out_writes_(cchBuf) PWSTR pszBuf, UINT cchBuf);
LWSTDAPI_(PWSTR)    StrFormatKBSizeW(LONGLONG qdw, _Out_writes_(cchBuf) PWSTR pszBuf, UINT cchBuf);
LWSTDAPI_(PSTR)     StrFormatKBSizeA(LONGLONG qdw, _Out_writes_(cchBuf) PSTR pszBuf, UINT cchBuf);
LWSTDAPI_(int)      StrFromTimeIntervalA(_Out_writes_(cchMax) PSTR pszOut, UINT cchMax, DWORD dwTimeMS, int digits);
LWSTDAPI_(int)      StrFromTimeIntervalW(_Out_writes_(cchMax) PWSTR pszOut, UINT cchMax, DWORD dwTimeMS, int digits);
LWSTDAPI_(BOOL)     StrIsIntlEqualA(BOOL fCaseSens, _In_ PCSTR pszString1, _In_ PCSTR pszString2, int nChar);
LWSTDAPI_(BOOL)     StrIsIntlEqualW(BOOL fCaseSens, _In_ PCWSTR pszString1, _In_ PCWSTR pszString2, int nChar);
LWSTDAPI_(PSTR)     StrNCatA(_Inout_updates_(cchMax) PSTR psz1, PCSTR psz2, int cchMax);
LWSTDAPI_(PWSTR)    StrNCatW(_Inout_updates_(cchMax) PWSTR psz1, PCWSTR psz2, int cchMax);
#ifdef USE_STRICT_CONST
LWSTDAPI_(PCSTR)    StrPBrkA(_In_ PCSTR psz, _In_ PCSTR pszSet);
LWSTDAPI_(PCWSTR)   StrPBrkW(_In_ PCWSTR psz, _In_ PCWSTR pszSet);
LWSTDAPI_(PCSTR)    StrRChrA(_In_ PCSTR pszStart, _In_opt_ PCSTR pszEnd, WORD wMatch);
LWSTDAPI_(PCWSTR)   StrRChrW(_In_ PCWSTR pszStart, _In_opt_ PCWSTR pszEnd, WCHAR wMatch);
LWSTDAPI_(PCSTR)    StrRChrIA(_In_ PCSTR pszStart, _In_opt_ PCSTR pszEnd, WORD wMatch);
LWSTDAPI_(PCWSTR)   StrRChrIW(_In_ PCWSTR pszStart, _In_opt_ PCWSTR pszEnd, WCHAR wMatch);
LWSTDAPI_(PCSTR)    StrRStrIA(_In_ PCSTR pszSource, _In_opt_ PCSTR pszLast, _In_ PCSTR pszSrch);
LWSTDAPI_(PCWSTR)   StrRStrIW(_In_ PCWSTR pszSource, _In_opt_ PCWSTR pszLast, _In_ PCWSTR pszSrch);
#else
LWSTDAPI_(PSTR)     StrPBrkA(_In_ PCSTR psz, _In_ PCSTR pszSet);
LWSTDAPI_(PWSTR)    StrPBrkW(_In_ PCWSTR psz, _In_ PCWSTR pszSet);
LWSTDAPI_(PSTR)     StrRChrA(_In_ PCSTR pszStart, _In_opt_ PCSTR pszEnd, WORD wMatch);
LWSTDAPI_(PWSTR)    StrRChrW(_In_ PCWSTR pszStart, _In_opt_ PCWSTR pszEnd, WCHAR wMatch);
LWSTDAPI_(PSTR)     StrRChrIA(_In_ PCSTR pszStart, _In_opt_ PCSTR pszEnd, WORD wMatch);
LWSTDAPI_(PWSTR)    StrRChrIW(_In_ PCWSTR pszStart, _In_opt_ PCWSTR pszEnd, WCHAR wMatch);
LWSTDAPI_(PSTR)     StrRStrIA(_In_ PCSTR pszSource, _In_opt_ PCSTR pszLast, _In_ PCSTR pszSrch);
LWSTDAPI_(PWSTR)    StrRStrIW(_In_ PCWSTR pszSource, _In_opt_ PCWSTR pszLast, _In_ PCWSTR pszSrch);
#endif
LWSTDAPI_(int)      StrSpnA(_In_ PCSTR psz, _In_ PCSTR pszSet);
LWSTDAPI_(int)      StrSpnW(_In_ PCWSTR psz, _In_ PCWSTR pszSet);
#ifdef USE_STRICT_CONST
LWSTDAPI_(PCSTR)    StrStrA(_In_ PCSTR pszFirst, _In_ PCSTR pszSrch);
LWSTDAPI_(PCWSTR)   StrStrW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch);
LWSTDAPI_(PCSTR)    StrStrIA(_In_ PCSTR pszFirst, _In_ PCSTR pszSrch);
LWSTDAPI_(PCWSTR)   StrStrIW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch);
#if (_WIN32_IE >= _WIN32_IE_IE6)
LWSTDAPI_(PCWSTR)   StrStrNW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch, UINT cchMax);
LWSTDAPI_(PCWSTR)   StrStrNIW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch, UINT cchMax);
#endif // _WIN32_IE_IE6
#else
LWSTDAPI_(PSTR)     StrStrA(_In_ PCSTR pszFirst, _In_ PCSTR pszSrch);
LWSTDAPI_(PWSTR)    StrStrW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch);
LWSTDAPI_(PSTR)     StrStrIA(_In_ PCSTR pszFirst, _In_ PCSTR pszSrch);
LWSTDAPI_(PWSTR)    StrStrIW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch);
#if (_WIN32_IE >= _WIN32_IE_IE60)
LWSTDAPI_(PWSTR)    StrStrNW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch, UINT cchMax);
LWSTDAPI_(PWSTR)    StrStrNIW(_In_ PCWSTR pszFirst, _In_ PCWSTR pszSrch, UINT cchMax);
#endif // _WIN32_IE_IE60
#endif

#define STIF_DEFAULT        0x00000000L
#define STIF_SUPPORT_HEX    0x00000001L
typedef int STIF_FLAGS;
LWSTDAPI_(int)      StrToIntA(_In_ PCSTR pszSrc);
LWSTDAPI_(int)      StrToIntW(_In_ PCWSTR pszSrc);
LWSTDAPI_(BOOL)     StrToIntExA(_In_ PCSTR pszString, STIF_FLAGS dwFlags, _Out_ int * piRet);
LWSTDAPI_(BOOL)     StrToIntExW(_In_ PCWSTR pszString, STIF_FLAGS dwFlags, _Out_ int * piRet);
#if (_WIN32_IE >= _WIN32_IE_IE60)
LWSTDAPI_(BOOL)     StrToInt64ExA(_In_ PCSTR pszString, STIF_FLAGS dwFlags, _Out_ LONGLONG * pllRet);
LWSTDAPI_(BOOL)     StrToInt64ExW(_In_ PCWSTR pszString, STIF_FLAGS dwFlags, _Out_ LONGLONG * pllRet);
#endif // _WIN32_IE_IE60

LWSTDAPI_(BOOL)     StrTrimA(_Inout_ PSTR psz, _In_ PCSTR pszTrimChars);
LWSTDAPI_(BOOL)     StrTrimW(_Inout_ PWSTR psz, _In_ PCWSTR pszTrimChars);


LWSTDAPI_(PWSTR)    StrCatW(_Inout_ PWSTR psz1, _In_ PCWSTR psz2);
LWSTDAPI_(int)      StrCmpW(_In_ PCWSTR psz1, _In_ PCWSTR psz2);
LWSTDAPI_(int)      StrCmpIW(_In_ PCWSTR psz1, _In_ PCWSTR psz2);
LWSTDAPI_(PWSTR)    StrCpyW(_Out_ PWSTR psz1, _In_ PCWSTR psz2);
LWSTDAPI_(PWSTR)    StrCpyNW(_Out_writes_(cchMax) PWSTR pszDst, _In_ PCWSTR pszSrc, int cchMax);

LWSTDAPI_(PWSTR)    StrCatBuffW(_Inout_updates_(cchDestBuffSize) PWSTR pszDest, _In_ PCWSTR pszSrc, int cchDestBuffSize);
LWSTDAPI_(PSTR)     StrCatBuffA(_Inout_updates_(cchDestBuffSize) PSTR pszDest, _In_ PCSTR pszSrc, int cchDestBuffSize);
LWSTDAPI_(BOOL)     ChrCmpIA(WORD w1, WORD w2);
LWSTDAPI_(BOOL)     ChrCmpIW(WCHAR w1, WCHAR w2);

LWSTDAPI_(int)      wvnsprintfA(_Out_writes_(cchDest) PSTR pszDest, _In_ int cchDest, _In_ _Printf_format_string_ PCSTR pszFmt, _In_ va_list arglist);
LWSTDAPI_(int)      wvnsprintfW(_Out_writes_(cchDest) PWSTR pszDest, _In_ int cchDest, _In_ _Printf_format_string_ PCWSTR pszFmt, _In_ va_list arglist);
LWSTDAPIV_(int)     wnsprintfA(_Out_writes_(cchDest) PSTR pszDest, _In_ int cchDest, _In_ _Printf_format_string_ PCSTR pszFmt, ...);
LWSTDAPIV_(int)     wnsprintfW(_Out_writes_(cchDest) PWSTR pszDest, _In_ int cchDest, _In_ _Printf_format_string_ PCWSTR pszFmt, ...);

#define StrIntlEqNA( s1, s2, nChar) StrIsIntlEqualA( TRUE, s1, s2, nChar)
#define StrIntlEqNW( s1, s2, nChar) StrIsIntlEqualW( TRUE, s1, s2, nChar)
#define StrIntlEqNIA(s1, s2, nChar) StrIsIntlEqualA(FALSE, s1, s2, nChar)
#define StrIntlEqNIW(s1, s2, nChar) StrIsIntlEqualW(FALSE, s1, s2, nChar)

LWSTDAPI StrRetToStrA(_Inout_ STRRET *pstr, _In_opt_ PCUITEMID_CHILD pidl, _Outptr_ LPSTR *ppsz);
LWSTDAPI StrRetToStrW(_Inout_ STRRET *pstr, _In_opt_ PCUITEMID_CHILD pidl, _Outptr_ LPWSTR *ppsz);
#ifdef UNICODE
#define StrRetToStr  StrRetToStrW
#else
#define StrRetToStr  StrRetToStrA
#endif // !UNICODE
LWSTDAPI StrRetToBufA(_Inout_ STRRET *pstr, _In_opt_ PCUITEMID_CHILD pidl, _Out_writes_(cchBuf) LPSTR pszBuf, UINT cchBuf);
LWSTDAPI StrRetToBufW(_Inout_ STRRET *pstr, _In_opt_ PCUITEMID_CHILD pidl, _Out_writes_(cchBuf) LPWSTR pszBuf, UINT cchBuf);
#ifdef UNICODE
#define StrRetToBuf  StrRetToBufW
#else
#define StrRetToBuf  StrRetToBufA
#endif // !UNICODE

// helper to duplicate a string using the task allocator

LWSTDAPI SHStrDupA(_In_ LPCSTR psz, _Outptr_result_nullonfailure_ LPWSTR *ppwsz);
LWSTDAPI SHStrDupW(_In_ LPCWSTR psz, _Outptr_result_nullonfailure_ LPWSTR *ppwsz);
#ifdef UNICODE
#define SHStrDup  SHStrDupW
#else
#define SHStrDup  SHStrDupA
#endif // !UNICODE

#ifdef __cplusplus
#pragma warning(push)
#pragma warning(disable:6387)

// make the above helper function a bit easier to use in the HRESULT world
inline HRESULT SHLocalStrDupW(_In_ PCWSTR psz, _Outptr_result_maybenull_ PWSTR *ppsz)
{
    *ppsz = StrDupW(psz);
    return *ppsz ? S_OK : E_OUTOFMEMORY;
}

inline HRESULT SHLocalStrDupA(_In_ PCSTR psz, _Outptr_result_maybenull_ PSTR *ppsz)
{
    *ppsz = StrDupA(psz);
    return *ppsz ? S_OK : E_OUTOFMEMORY;
}

#pragma warning(pop) // C6387

#ifdef UNICODE
#define SHLocalStrDup SHLocalStrDupW
#else
#define SHLocalStrDup SHLocalStrDupA
#endif
#endif // __cplusplus

LWSTDAPI_(int) StrCmpLogicalW(_In_ PCWSTR psz1, _In_ PCWSTR psz2);
LWSTDAPI_(DWORD) StrCatChainW(_Out_writes_(cchDst) PWSTR pszDst, DWORD cchDst, DWORD ichAt, _In_ PCWSTR pszSrc);
LWSTDAPI StrRetToBSTR(_Inout_ STRRET *pstr, _In_opt_ PCUITEMID_CHILD pidl, _Outptr_ BSTR *pbstr);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

LWSTDAPI SHLoadIndirectString(_In_ PCWSTR pszSource, _Out_writes_(cchOutBuf) PWSTR pszOutBuf, _In_ UINT cchOutBuf, _Reserved_ void **ppvReserved);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
LWSTDAPI_(BOOL) IsCharSpaceA(CHAR wch);
LWSTDAPI_(BOOL) IsCharSpaceW(WCHAR wch);
#ifdef UNICODE
#define IsCharSpace  IsCharSpaceW
#else
#define IsCharSpace  IsCharSpaceA
#endif // !UNICODE

LWSTDAPI_(int)  StrCmpCA(_In_ LPCSTR pszStr1, _In_ LPCSTR pszStr2);
LWSTDAPI_(int)  StrCmpCW(_In_ LPCWSTR pszStr1, _In_ LPCWSTR pszStr2);
#ifdef UNICODE
#define StrCmpC  StrCmpCW
#else
#define StrCmpC  StrCmpCA
#endif // !UNICODE

LWSTDAPI_(int)  StrCmpICA(_In_ LPCSTR pszStr1, _In_ LPCSTR pszStr2);
LWSTDAPI_(int)  StrCmpICW(_In_ LPCWSTR pszStr1, _In_ LPCWSTR pszStr2);
#ifdef UNICODE
#define StrCmpIC  StrCmpICW
#else
#define StrCmpIC  StrCmpICA
#endif // !UNICODE
#endif // _WIN32_IE_IE60SP2

#ifdef UNICODE
#define StrChr                  StrChrW
#define StrRChr                 StrRChrW
#define StrChrI                 StrChrIW
#define StrRChrI                StrRChrIW
#define StrCmpN                 StrCmpNW
#define StrCmpNI                StrCmpNIW
#define StrStr                  StrStrW
#define StrStrI                 StrStrIW
#define StrDup                  StrDupW
#define StrRStrI                StrRStrIW
#define StrCSpn                 StrCSpnW
#define StrCSpnI                StrCSpnIW
#define StrSpn                  StrSpnW
#define StrToInt                StrToIntW
#define StrPBrk                 StrPBrkW
#define StrToIntEx              StrToIntExW
#if (_WIN32_IE >= 0x0600)
#define StrToInt64Ex            StrToInt64ExW
#endif
#define StrFromTimeInterval     StrFromTimeIntervalW
#define StrIntlEqN              StrIntlEqNW
#define StrIntlEqNI             StrIntlEqNIW
#define StrFormatByteSize       StrFormatByteSizeW
#define StrFormatByteSize64     StrFormatByteSizeW
#define StrFormatKBSize         StrFormatKBSizeW
#define StrNCat                 StrNCatW
#define StrTrim                 StrTrimW
#define StrCatBuff              StrCatBuffW
#define ChrCmpI                 ChrCmpIW
#define wvnsprintf              wvnsprintfW
#define wnsprintf               wnsprintfW
#define StrIsIntlEqual          StrIsIntlEqualW


#else
#define StrChr                  StrChrA
#define StrRChr                 StrRChrA
#define StrChrI                 StrChrIA
#define StrRChrI                StrRChrIA
#define StrCmpN                 StrCmpNA
#define StrCmpNI                StrCmpNIA
#define StrStr                  StrStrA
#define StrStrI                 StrStrIA
#define StrDup                  StrDupA
#define StrRStrI                StrRStrIA
#define StrCSpn                 StrCSpnA
#define StrCSpnI                StrCSpnIA
#define StrSpn                  StrSpnA
#define StrToInt                StrToIntA
#define StrPBrk                 StrPBrkA
#define StrToIntEx              StrToIntExA
#if (_WIN32_IE >= 0x0600)
#define StrToInt64Ex            StrToInt64ExA
#endif
#define StrFromTimeInterval     StrFromTimeIntervalA
#define StrIntlEqN              StrIntlEqNA
#define StrIntlEqNI             StrIntlEqNIA
#define StrFormatByteSize       StrFormatByteSizeA
#define StrFormatByteSize64     StrFormatByteSize64A
#define StrFormatKBSize         StrFormatKBSizeA
#define StrNCat                 StrNCatA
#define StrTrim                 StrTrimA
#define StrCatBuff              StrCatBuffA
#define ChrCmpI                 ChrCmpIA
#define wvnsprintf              wvnsprintfA
#define wnsprintf               wnsprintfA
#define StrIsIntlEqual          StrIsIntlEqualA
#endif

// StrCmp*C* - Compare strings using C runtime collation rules.
LWSTDAPI_(int)  StrCmpNCA(_In_ LPCSTR pszStr1, _In_ LPCSTR pszStr2, int nChar);
// StrCmp*C* - Compare strings using C runtime collation rules.
LWSTDAPI_(int)  StrCmpNCW(_In_ LPCWSTR pszStr1, _In_ LPCWSTR pszStr2, int nChar);
#ifdef UNICODE
#define StrCmpNC  StrCmpNCW
#else
#define StrCmpNC  StrCmpNCA
#endif // !UNICODE
LWSTDAPI_(int)  StrCmpNICA(_In_ LPCSTR pszStr1, _In_ LPCSTR pszStr2, int nChar);
LWSTDAPI_(int)  StrCmpNICW(_In_ LPCWSTR pszStr1, _In_ LPCWSTR pszStr2, int nChar);
#ifdef UNICODE
#define StrCmpNIC  StrCmpNICW
#else
#define StrCmpNIC  StrCmpNICA
#endif // !UNICODE


// Backward compatible to NT's non-standard naming (strictly
// for comctl32)
//
LWSTDAPI_(BOOL)     IntlStrEqWorkerA(BOOL fCaseSens, _In_reads_(nChar) LPCSTR lpString1, _In_reads_(nChar) LPCSTR lpString2, int nChar);
LWSTDAPI_(BOOL)     IntlStrEqWorkerW(BOOL fCaseSens, _In_reads_(nChar) LPCWSTR lpString1, _In_reads_(nChar) LPCWSTR lpString2, int nChar);

#define IntlStrEqNA( s1, s2, nChar) IntlStrEqWorkerA( TRUE, s1, s2, nChar)
#define IntlStrEqNW( s1, s2, nChar) IntlStrEqWorkerW( TRUE, s1, s2, nChar)
#define IntlStrEqNIA(s1, s2, nChar) IntlStrEqWorkerA(FALSE, s1, s2, nChar)
#define IntlStrEqNIW(s1, s2, nChar) IntlStrEqWorkerW(FALSE, s1, s2, nChar)

#ifdef UNICODE
#define IntlStrEqN              IntlStrEqNW
#define IntlStrEqNI             IntlStrEqNIW
#else
#define IntlStrEqN              IntlStrEqNA
#define IntlStrEqNI             IntlStrEqNIA
#endif

#define SZ_CONTENTTYPE_HTMLA       "text/html"
#define SZ_CONTENTTYPE_HTMLW       L"text/html"
#define SZ_CONTENTTYPE_CDFA        "application/x-cdf"
#define SZ_CONTENTTYPE_CDFW        L"application/x-cdf"

#ifdef UNICODE
#define SZ_CONTENTTYPE_HTML     SZ_CONTENTTYPE_HTMLW
#define SZ_CONTENTTYPE_CDF      SZ_CONTENTTYPE_CDFW
#else
#define SZ_CONTENTTYPE_HTML     SZ_CONTENTTYPE_HTMLA
#define SZ_CONTENTTYPE_CDF      SZ_CONTENTTYPE_CDFA
#endif

#define PathIsHTMLFileA(pszPath)     PathIsContentTypeA(pszPath, SZ_CONTENTTYPE_HTMLA)
#define PathIsHTMLFileW(pszPath)     PathIsContentTypeW(pszPath, SZ_CONTENTTYPE_HTMLW)


#define StrCatA                 lstrcatA
#define StrCmpA                 lstrcmpA
#define StrCmpIA                lstrcmpiA
#define StrCpyA                 lstrcpyA
#define StrCpyNA                lstrcpynA


#define StrToLong               StrToInt
#define StrNCmp                 StrCmpN
#define StrNCmpI                StrCmpNI
#define StrNCpy                 StrCpyN
#define StrCatN                 StrNCat

#ifdef UNICODE
#define StrCat                  StrCatW
#define StrCmp                  StrCmpW
#define StrCmpI                 StrCmpIW
#define StrCpy                  StrCpyW
#define StrCpyN                 StrCpyNW
#define StrCatBuff              StrCatBuffW
#else
#define StrCat                  lstrcatA
#define StrCmp                  lstrcmpA
#define StrCmpI                 lstrcmpiA
#define StrCpy                  lstrcpyA
#define StrCpyN                 lstrcpynA
#define StrCatBuff              StrCatBuffA
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif //  NO_SHLWAPI_STRFCNS


#ifndef NO_SHLWAPI_PATH

//
//=============== Path Routines ===================================
//


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

LWSTDAPI_(LPSTR)  PathAddBackslashA(_Inout_updates_(MAX_PATH) LPSTR pszPath);
LWSTDAPI_(LPWSTR)  PathAddBackslashW(_Inout_updates_(MAX_PATH) LPWSTR pszPath);
#ifdef UNICODE
#define PathAddBackslash  PathAddBackslashW
#else
#define PathAddBackslash  PathAddBackslashA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathAddExtensionA(_Inout_updates_(MAX_PATH) LPSTR pszPath, _In_opt_ LPCSTR pszExt);
LWSTDAPI_(BOOL)     PathAddExtensionW(_Inout_updates_(MAX_PATH) LPWSTR pszPath, _In_opt_ LPCWSTR pszExt);
#ifdef UNICODE
#define PathAddExtension  PathAddExtensionW
#else
#define PathAddExtension  PathAddExtensionA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathAppendA(_Inout_updates_(MAX_PATH) LPSTR pszPath, _In_ LPCSTR pszMore);
LWSTDAPI_(BOOL)     PathAppendW(_Inout_updates_(MAX_PATH) LPWSTR pszPath, _In_ LPCWSTR pszMore);
LWSTDAPI_(LPSTR)  PathBuildRootA(_Out_writes_(4) LPSTR pszRoot, int iDrive);
LWSTDAPI_(LPWSTR)  PathBuildRootW(_Out_writes_(4) LPWSTR pszRoot, int iDrive);
#ifdef UNICODE
#define PathBuildRoot  PathBuildRootW
#else
#define PathBuildRoot  PathBuildRootA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathCanonicalizeA(_Out_writes_(MAX_PATH) LPSTR pszBuf, _In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathCanonicalizeW(_Out_writes_(MAX_PATH) LPWSTR pszBuf, _In_ LPCWSTR pszPath);
LWSTDAPI_(LPSTR)  PathCombineA(_Out_writes_(MAX_PATH) LPSTR pszDest, _In_opt_ LPCSTR pszDir, _In_opt_ LPCSTR pszFile);
LWSTDAPI_(LPWSTR)  PathCombineW(_Out_writes_(MAX_PATH) LPWSTR pszDest, _In_opt_ LPCWSTR pszDir, _In_opt_ LPCWSTR pszFile);
#ifdef UNICODE
#define PathCombine  PathCombineW
#else
#define PathCombine  PathCombineA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathCompactPathA(_In_opt_ HDC hDC, _Inout_updates_(MAX_PATH) LPSTR pszPath, _In_ UINT dx);
LWSTDAPI_(BOOL)     PathCompactPathW(_In_opt_ HDC hDC, _Inout_updates_(MAX_PATH) LPWSTR pszPath, _In_ UINT dx);
LWSTDAPI_(BOOL)     PathCompactPathExA(_Out_writes_(cchMax) LPSTR pszOut, _In_ LPCSTR pszSrc, _In_ UINT cchMax, _In_ DWORD dwFlags);
LWSTDAPI_(BOOL)     PathCompactPathExW(_Out_writes_(cchMax) LPWSTR pszOut, _In_ LPCWSTR pszSrc, _In_ UINT cchMax, _In_ DWORD dwFlags);
LWSTDAPI_(int)      PathCommonPrefixA(_In_ LPCSTR pszFile1, _In_ LPCSTR pszFile2, _Out_writes_opt_(MAX_PATH) LPSTR achPath);
LWSTDAPI_(int)      PathCommonPrefixW(_In_ LPCWSTR pszFile1, _In_ LPCWSTR pszFile2, _Out_writes_opt_(MAX_PATH) LPWSTR achPath);
LWSTDAPI_(BOOL)     PathFileExistsA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathFileExistsW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFileExists  PathFileExistsW
#else
#define PathFileExists  PathFileExistsA
#endif // !UNICODE
#ifdef USE_STRICT_CONST
LWSTDAPI_(LPCSTR)  PathFindExtensionA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPCWSTR)  PathFindExtensionW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFindExtension  PathFindExtensionW
#else
#define PathFindExtension  PathFindExtensionA
#endif // !UNICODE
LWSTDAPI_(LPCSTR)  PathFindFileNameA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPCWSTR)  PathFindFileNameW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFindFileName  PathFindFileNameW
#else
#define PathFindFileName  PathFindFileNameA
#endif // !UNICODE
LWSTDAPI_(LPCSTR)  PathFindNextComponentA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPCWSTR)  PathFindNextComponentW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFindNextComponent  PathFindNextComponentW
#else
#define PathFindNextComponent  PathFindNextComponentA
#endif // !UNICODE
#else
LWSTDAPI_(LPSTR)  PathFindExtensionA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPWSTR)  PathFindExtensionW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFindExtension  PathFindExtensionW
#else
#define PathFindExtension  PathFindExtensionA
#endif // !UNICODE
LWSTDAPI_(LPSTR)  PathFindFileNameA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPWSTR)  PathFindFileNameW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFindFileName  PathFindFileNameW
#else
#define PathFindFileName  PathFindFileNameA
#endif // !UNICODE
LWSTDAPI_(LPSTR)  PathFindNextComponentA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPWSTR)  PathFindNextComponentW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathFindNextComponent  PathFindNextComponentW
#else
#define PathFindNextComponent  PathFindNextComponentA
#endif // !UNICODE
#endif
LWSTDAPI_(BOOL)     PathFindOnPathA(_Inout_updates_(MAX_PATH) LPSTR pszPath, _In_opt_ PZPCSTR ppszOtherDirs);
LWSTDAPI_(BOOL)     PathFindOnPathW(_Inout_updates_(MAX_PATH) LPWSTR pszPath, _In_opt_ PZPCWSTR ppszOtherDirs);
LWSTDAPI_(LPCSTR) PathFindSuffixArrayA(_In_ LPCSTR pszPath, _In_reads_(iArraySize) const LPCSTR *apszSuffix, _In_ int iArraySize);
LWSTDAPI_(LPCWSTR) PathFindSuffixArrayW(_In_ LPCWSTR pszPath, _In_reads_(iArraySize) const LPCWSTR *apszSuffix, _In_ int iArraySize);
#ifdef UNICODE
#define PathFindSuffixArray  PathFindSuffixArrayW
#else
#define PathFindSuffixArray  PathFindSuffixArrayA
#endif // !UNICODE
#ifdef USE_STRICT_CONST
LWSTDAPI_(LPCSTR)  PathGetArgsA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPCWSTR)  PathGetArgsW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathGetArgs  PathGetArgsW
#else
#define PathGetArgs  PathGetArgsA
#endif // !UNICODE
#else
LWSTDAPI_(LPSTR)  PathGetArgsA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPWSTR)  PathGetArgsW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathGetArgs  PathGetArgsW
#else
#define PathGetArgs  PathGetArgsA
#endif // !UNICODE
#endif
LWSTDAPI_(BOOL)     PathIsLFNFileSpecA(_In_ LPCSTR pszName);
LWSTDAPI_(BOOL)     PathIsLFNFileSpecW(_In_ LPCWSTR pszName);
#ifdef UNICODE
#define PathIsLFNFileSpec  PathIsLFNFileSpecW
#else
#define PathIsLFNFileSpec  PathIsLFNFileSpecA
#endif // !UNICODE
LWSTDAPI_(UINT)     PathGetCharTypeA(_In_ UCHAR ch);
LWSTDAPI_(UINT)     PathGetCharTypeW(_In_ WCHAR ch);

// Return flags for PathGetCharType
#define GCT_INVALID             0x0000
#define GCT_LFNCHAR             0x0001
#define GCT_SHORTCHAR           0x0002
#define GCT_WILD                0x0004
#define GCT_SEPARATOR           0x0008

LWSTDAPI_(int)      PathGetDriveNumberA(_In_ LPCSTR pszPath);
LWSTDAPI_(int)      PathGetDriveNumberW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathGetDriveNumber  PathGetDriveNumberW
#else
#define PathGetDriveNumber  PathGetDriveNumberA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsDirectoryA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsDirectoryW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsDirectory  PathIsDirectoryW
#else
#define PathIsDirectory  PathIsDirectoryA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsDirectoryEmptyA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsDirectoryEmptyW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsDirectoryEmpty  PathIsDirectoryEmptyW
#else
#define PathIsDirectoryEmpty  PathIsDirectoryEmptyA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsFileSpecA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsFileSpecW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsFileSpec  PathIsFileSpecW
#else
#define PathIsFileSpec  PathIsFileSpecA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsPrefixA(_In_ LPCSTR pszPrefix, _In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsPrefixW(_In_ LPCWSTR pszPrefix, _In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsPrefix  PathIsPrefixW
#else
#define PathIsPrefix  PathIsPrefixA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsRelativeA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsRelativeW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsRelative  PathIsRelativeW
#else
#define PathIsRelative  PathIsRelativeA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsRootA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsRootW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsRoot  PathIsRootW
#else
#define PathIsRoot  PathIsRootA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsSameRootA(_In_ LPCSTR pszPath1, _In_ LPCSTR pszPath2);
LWSTDAPI_(BOOL)     PathIsSameRootW(_In_ LPCWSTR pszPath1, _In_ LPCWSTR pszPath2);
#ifdef UNICODE
#define PathIsSameRoot  PathIsSameRootW
#else
#define PathIsSameRoot  PathIsSameRootA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsUNCA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsUNCW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsUNC  PathIsUNCW
#else
#define PathIsUNC  PathIsUNCA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsNetworkPathA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsNetworkPathW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsNetworkPath  PathIsNetworkPathW
#else
#define PathIsNetworkPath  PathIsNetworkPathA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsUNCServerA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsUNCServerW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsUNCServer  PathIsUNCServerW
#else
#define PathIsUNCServer  PathIsUNCServerA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsUNCServerShareA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsUNCServerShareW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsUNCServerShare  PathIsUNCServerShareW
#else
#define PathIsUNCServerShare  PathIsUNCServerShareA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsContentTypeA(_In_ LPCSTR pszPath, _In_ LPCSTR pszContentType);
LWSTDAPI_(BOOL)     PathIsContentTypeW(_In_ LPCWSTR pszPath, _In_ LPCWSTR pszContentType);
LWSTDAPI_(BOOL)     PathIsURLA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathIsURLW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathIsURL  PathIsURLW
#else
#define PathIsURL  PathIsURLA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathMakePrettyA(_Inout_ LPSTR pszPath);
LWSTDAPI_(BOOL)     PathMakePrettyW(_Inout_ LPWSTR pszPath);
LWSTDAPI_(BOOL)     PathMatchSpecA(_In_ LPCSTR pszFile, _In_ LPCSTR pszSpec);
LWSTDAPI_(BOOL)     PathMatchSpecW(_In_ LPCWSTR pszFile, _In_ LPCWSTR pszSpec);

#if (_WIN32_IE >= _WIN32_IE_IE70)
// Flags for PathMatchSpecEx
#define PMSF_NORMAL            0x00000000
#define PMSF_MULTIPLE          0x00000001
#define PMSF_DONT_STRIP_SPACES 0x00010000  // modifies either of the above

LWSTDAPI PathMatchSpecExA(_In_ LPCSTR pszFile, _In_ LPCSTR pszSpec, _In_ DWORD dwFlags);
LWSTDAPI PathMatchSpecExW(_In_ LPCWSTR pszFile, _In_ LPCWSTR pszSpec, _In_ DWORD dwFlags);
#endif // _WIN32_IE_IE70

LWSTDAPI_(int)      PathParseIconLocationA(_Inout_ LPSTR pszIconFile);
LWSTDAPI_(int)      PathParseIconLocationW(_Inout_ LPWSTR pszIconFile);
LWSTDAPI_(BOOL)     PathQuoteSpacesA(_Inout_updates_(MAX_PATH) LPSTR lpsz);
LWSTDAPI_(BOOL)     PathQuoteSpacesW(_Inout_updates_(MAX_PATH) LPWSTR lpsz);
LWSTDAPI_(BOOL)     PathRelativePathToA(_Out_writes_(MAX_PATH) LPSTR pszPath, _In_ LPCSTR pszFrom, _In_ DWORD dwAttrFrom, _In_ LPCSTR pszTo, _In_ DWORD dwAttrTo);
LWSTDAPI_(BOOL)     PathRelativePathToW(_Out_writes_(MAX_PATH) LPWSTR pszPath, _In_ LPCWSTR pszFrom, _In_ DWORD dwAttrFrom, _In_ LPCWSTR pszTo, _In_ DWORD dwAttrTo);
LWSTDAPI_(void)     PathRemoveArgsA(_Inout_ LPSTR pszPath);
LWSTDAPI_(void)     PathRemoveArgsW(_Inout_ LPWSTR pszPath);
LWSTDAPI_(LPSTR)  PathRemoveBackslashA(_Inout_ LPSTR pszPath);
LWSTDAPI_(LPWSTR)  PathRemoveBackslashW(_Inout_ LPWSTR pszPath);
#ifdef UNICODE
#define PathRemoveBackslash  PathRemoveBackslashW
#else
#define PathRemoveBackslash  PathRemoveBackslashA
#endif // !UNICODE
LWSTDAPI_(void)     PathRemoveBlanksA(_Inout_ LPSTR pszPath);
LWSTDAPI_(void)     PathRemoveBlanksW(_Inout_ LPWSTR pszPath);
LWSTDAPI_(void)     PathRemoveExtensionA(_Inout_ LPSTR pszPath);
LWSTDAPI_(void)     PathRemoveExtensionW(_Inout_ LPWSTR pszPath);
LWSTDAPI_(BOOL)     PathRemoveFileSpecA(_Inout_ LPSTR pszPath);
LWSTDAPI_(BOOL)     PathRemoveFileSpecW(_Inout_ LPWSTR pszPath);
LWSTDAPI_(BOOL)     PathRenameExtensionA(_Inout_updates_(MAX_PATH) LPSTR pszPath, _In_ LPCSTR pszExt);
LWSTDAPI_(BOOL)     PathRenameExtensionW(_Inout_updates_(MAX_PATH) LPWSTR pszPath, _In_ LPCWSTR pszExt);
LWSTDAPI_(BOOL)     PathSearchAndQualifyA(_In_ LPCSTR pszPath, _Out_writes_(cchBuf) LPSTR pszBuf, _In_ UINT cchBuf);
LWSTDAPI_(BOOL)     PathSearchAndQualifyW(_In_ LPCWSTR pszPath, _Out_writes_(cchBuf) LPWSTR pszBuf, _In_ UINT cchBuf);
LWSTDAPI_(void)     PathSetDlgItemPathA(_In_ HWND hDlg, int id, LPCSTR pszPath);
LWSTDAPI_(void)     PathSetDlgItemPathW(_In_ HWND hDlg, int id, LPCWSTR pszPath);
#ifdef USE_STRICT_CONST
LWSTDAPI_(LPCSTR)  PathSkipRootA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPCWSTR)  PathSkipRootW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathSkipRoot  PathSkipRootW
#else
#define PathSkipRoot  PathSkipRootA
#endif // !UNICODE
#else
LWSTDAPI_(LPSTR)  PathSkipRootA(_In_ LPCSTR pszPath);
LWSTDAPI_(LPWSTR)  PathSkipRootW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathSkipRoot  PathSkipRootW
#else
#define PathSkipRoot  PathSkipRootA
#endif // !UNICODE
#endif
LWSTDAPI_(void)     PathStripPathA(_Inout_ LPSTR pszPath);
LWSTDAPI_(void)     PathStripPathW(_Inout_ LPWSTR pszPath);
#ifdef UNICODE
#define PathStripPath  PathStripPathW
#else
#define PathStripPath  PathStripPathA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathStripToRootA(_Inout_ LPSTR pszPath);
LWSTDAPI_(BOOL)     PathStripToRootW(_Inout_ LPWSTR pszPath);
#ifdef UNICODE
#define PathStripToRoot  PathStripToRootW
#else
#define PathStripToRoot  PathStripToRootA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathUnquoteSpacesA(_Inout_ LPSTR lpsz);
LWSTDAPI_(BOOL)     PathUnquoteSpacesW(_Inout_ LPWSTR lpsz);
LWSTDAPI_(BOOL)     PathMakeSystemFolderA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathMakeSystemFolderW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathMakeSystemFolder  PathMakeSystemFolderW
#else
#define PathMakeSystemFolder  PathMakeSystemFolderA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathUnmakeSystemFolderA(_In_ LPCSTR pszPath);
LWSTDAPI_(BOOL)     PathUnmakeSystemFolderW(_In_ LPCWSTR pszPath);
#ifdef UNICODE
#define PathUnmakeSystemFolder  PathUnmakeSystemFolderW
#else
#define PathUnmakeSystemFolder  PathUnmakeSystemFolderA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathIsSystemFolderA(_In_opt_ LPCSTR pszPath, _In_ DWORD dwAttrb);
LWSTDAPI_(BOOL)     PathIsSystemFolderW(_In_opt_ LPCWSTR pszPath, _In_ DWORD dwAttrb);
#ifdef UNICODE
#define PathIsSystemFolder  PathIsSystemFolderW
#else
#define PathIsSystemFolder  PathIsSystemFolderA
#endif // !UNICODE
LWSTDAPI_(void)     PathUndecorateA(_Inout_ LPSTR pszPath);
LWSTDAPI_(void)     PathUndecorateW(_Inout_ LPWSTR pszPath);
#ifdef UNICODE
#define PathUndecorate  PathUndecorateW
#else
#define PathUndecorate  PathUndecorateA
#endif // !UNICODE
LWSTDAPI_(BOOL)     PathUnExpandEnvStringsA(_In_ LPCSTR pszPath, _Out_writes_(cchBuf) LPSTR pszBuf, _In_ UINT cchBuf);
LWSTDAPI_(BOOL)     PathUnExpandEnvStringsW(_In_ LPCWSTR pszPath, _Out_writes_(cchBuf) LPWSTR pszBuf, _In_ UINT cchBuf);
#ifdef UNICODE
#define PathUnExpandEnvStrings  PathUnExpandEnvStringsW
#else
#define PathUnExpandEnvStrings  PathUnExpandEnvStringsA
#endif // !UNICODE


#ifdef UNICODE
#define PathAppend              PathAppendW
#define PathCanonicalize        PathCanonicalizeW
#define PathCompactPath         PathCompactPathW
#define PathCompactPathEx       PathCompactPathExW
#define PathCommonPrefix        PathCommonPrefixW
#define PathFindOnPath          PathFindOnPathW
#define PathGetCharType         PathGetCharTypeW
#define PathIsContentType       PathIsContentTypeW
#define PathIsHTMLFile          PathIsHTMLFileW
#define PathMakePretty          PathMakePrettyW
#define PathMatchSpec           PathMatchSpecW
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define PathMatchSpecEx         PathMatchSpecExW
#endif // _WIN32_IE_IE70
#define PathParseIconLocation   PathParseIconLocationW
#define PathQuoteSpaces         PathQuoteSpacesW
#define PathRelativePathTo      PathRelativePathToW
#define PathRemoveArgs          PathRemoveArgsW
#define PathRemoveBlanks        PathRemoveBlanksW
#define PathRemoveExtension     PathRemoveExtensionW
#define PathRemoveFileSpec      PathRemoveFileSpecW
#define PathRenameExtension     PathRenameExtensionW
#define PathSearchAndQualify    PathSearchAndQualifyW
#define PathSetDlgItemPath      PathSetDlgItemPathW
#define PathUnquoteSpaces       PathUnquoteSpacesW
#else
#define PathAppend              PathAppendA
#define PathCanonicalize        PathCanonicalizeA
#define PathCompactPath         PathCompactPathA
#define PathCompactPathEx       PathCompactPathExA
#define PathCommonPrefix        PathCommonPrefixA
#define PathFindOnPath          PathFindOnPathA
#define PathGetCharType         PathGetCharTypeA
#define PathIsContentType       PathIsContentTypeA
#define PathIsHTMLFile          PathIsHTMLFileA
#define PathMakePretty          PathMakePrettyA
#define PathMatchSpec           PathMatchSpecA
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define PathMatchSpecEx         PathMatchSpecExA
#endif // _WIN32_IE_IE70
#define PathParseIconLocation   PathParseIconLocationA
#define PathQuoteSpaces         PathQuoteSpacesA
#define PathRelativePathTo      PathRelativePathToA
#define PathRemoveArgs          PathRemoveArgsA
#define PathRemoveBlanks        PathRemoveBlanksA
#define PathRemoveExtension     PathRemoveExtensionA
#define PathRemoveFileSpec      PathRemoveFileSpecA
#define PathRenameExtension     PathRenameExtensionA
#define PathSearchAndQualify    PathSearchAndQualifyA
#define PathSetDlgItemPath      PathSetDlgItemPathA
#define PathUnquoteSpaces       PathUnquoteSpacesA
#endif

typedef enum
{
    URL_SCHEME_INVALID     = -1,
    URL_SCHEME_UNKNOWN     =  0,
    URL_SCHEME_FTP,
    URL_SCHEME_HTTP,
    URL_SCHEME_GOPHER,
    URL_SCHEME_MAILTO,
    URL_SCHEME_NEWS,
    URL_SCHEME_NNTP,
    URL_SCHEME_TELNET,
    URL_SCHEME_WAIS,
    URL_SCHEME_FILE,
    URL_SCHEME_MK,
    URL_SCHEME_HTTPS,
    URL_SCHEME_SHELL,
    URL_SCHEME_SNEWS,
    URL_SCHEME_LOCAL,
    URL_SCHEME_JAVASCRIPT,
    URL_SCHEME_VBSCRIPT,
    URL_SCHEME_ABOUT,
    URL_SCHEME_RES,
#if (_WIN32_IE >= _WIN32_IE_IE60)
    URL_SCHEME_MSSHELLROOTED,
    URL_SCHEME_MSSHELLIDLIST,
    URL_SCHEME_MSHELP,
#endif // _WIN32_IE_IE60
#if (_WIN32_IE >= _WIN32_IE_IE70)
    URL_SCHEME_MSSHELLDEVICE,
    URL_SCHEME_WILDCARD,
#endif // _WIN32_IE_IE70
#if (NTDDI_VERSION >= NTDDI_VISTA)
    URL_SCHEME_SEARCH_MS,
#endif
#if (NTDDI_VERSION >= NTDDI_VISTASP1)
    URL_SCHEME_SEARCH,
#endif
#if (NTDDI_VERSION >= NTDDI_WIN7)
    URL_SCHEME_KNOWNFOLDER,
#endif
    URL_SCHEME_MAXVALUE,
} URL_SCHEME;

typedef enum
{
    URL_PART_NONE       = 0,
    URL_PART_SCHEME     = 1,
    URL_PART_HOSTNAME,
    URL_PART_USERNAME,
    URL_PART_PASSWORD,
    URL_PART_PORT,
    URL_PART_QUERY,
} URL_PART;

typedef enum
{
    URLIS_URL,
    URLIS_OPAQUE,
    URLIS_NOHISTORY,
    URLIS_FILEURL,
    URLIS_APPLIABLE,
    URLIS_DIRECTORY,
    URLIS_HASQUERY,
} URLIS;

#define URL_UNESCAPE                    0x10000000
#define URL_ESCAPE_UNSAFE               0x20000000
#define URL_PLUGGABLE_PROTOCOL          0x40000000
#define URL_WININET_COMPATIBILITY       0x80000000
#define URL_DONT_ESCAPE_EXTRA_INFO      0x02000000
#define URL_DONT_UNESCAPE_EXTRA_INFO    URL_DONT_ESCAPE_EXTRA_INFO
#define URL_BROWSER_MODE                URL_DONT_ESCAPE_EXTRA_INFO
#define URL_ESCAPE_SPACES_ONLY          0x04000000
#define URL_DONT_SIMPLIFY               0x08000000
#define URL_NO_META                     URL_DONT_SIMPLIFY
#define URL_UNESCAPE_INPLACE            0x00100000
#define URL_CONVERT_IF_DOSPATH          0x00200000
#define URL_UNESCAPE_HIGH_ANSI_ONLY     0x00400000
#define URL_INTERNAL_PATH               0x00800000  // Will escape #'s in paths
#define URL_FILE_USE_PATHURL            0x00010000
#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
#define URL_DONT_UNESCAPE               0x00020000  // Do not unescape the path/url at all
#endif // _WIN32_IE_IE60SP2
#if (NTDDI_VERSION >= NTDDI_WIN7)
#define URL_ESCAPE_AS_UTF8              0x00040000  // Percent-encode all non-ASCII characters as their UTF-8 equivalents.
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#define URL_UNESCAPE_AS_UTF8            URL_ESCAPE_AS_UTF8
#define URL_ESCAPE_ASCII_URI_COMPONENT  0x00080000  // Percent-encode all ASCII characters outside of the unreserved set from URI RFC 3986 (a-zA-Z0-9-.~_) (i.e.) No need for URL_ESCAPE_PERCENT along with this.
#define URL_ESCAPE_URI_COMPONENT        (URL_ESCAPE_ASCII_URI_COMPONENT | URL_ESCAPE_AS_UTF8)
#define URL_UNESCAPE_URI_COMPONENT      URL_UNESCAPE_AS_UTF8
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#define URL_ESCAPE_PERCENT              0x00001000
#define URL_ESCAPE_SEGMENT_ONLY         0x00002000  // Treat the entire URL param as one URL segment.

#define URL_PARTFLAG_KEEPSCHEME         0x00000001

#define URL_APPLY_DEFAULT               0x00000001
#define URL_APPLY_GUESSSCHEME           0x00000002
#define URL_APPLY_GUESSFILE             0x00000004
#define URL_APPLY_FORCEAPPLY            0x00000008


LWSTDAPI_(int)          UrlCompareA(_In_ PCSTR psz1, _In_ PCSTR psz2, BOOL fIgnoreSlash);
LWSTDAPI_(int)          UrlCompareW(_In_ PCWSTR psz1, _In_ PCWSTR psz2, BOOL fIgnoreSlash);
LWSTDAPI                UrlCombineA(_In_ PCSTR pszBase, _In_ PCSTR pszRelative, _Out_writes_to_opt_(*pcchCombined, *pcchCombined) PSTR pszCombined, _Inout_ DWORD *pcchCombined, DWORD dwFlags);
LWSTDAPI                UrlCombineW(_In_ PCWSTR pszBase, _In_ PCWSTR pszRelative, _Out_writes_to_opt_(*pcchCombined, *pcchCombined) PWSTR pszCombined, _Inout_ DWORD *pcchCombined, DWORD dwFlags);
LWSTDAPI                UrlCanonicalizeA(_In_ PCSTR pszUrl, _Out_writes_to_(*pcchCanonicalized, *pcchCanonicalized) PSTR pszCanonicalized, _Inout_ DWORD *pcchCanonicalized, DWORD dwFlags);
LWSTDAPI                UrlCanonicalizeW(_In_ PCWSTR pszUrl, _Out_writes_to_(*pcchCanonicalized, *pcchCanonicalized) PWSTR pszCanonicalized, _Inout_ DWORD *pcchCanonicalized, DWORD dwFlags);
LWSTDAPI_(BOOL)         UrlIsOpaqueA(_In_ PCSTR pszURL);
LWSTDAPI_(BOOL)         UrlIsOpaqueW(_In_ PCWSTR pszURL);
LWSTDAPI_(BOOL)         UrlIsNoHistoryA(_In_ PCSTR pszURL);
LWSTDAPI_(BOOL)         UrlIsNoHistoryW(_In_ PCWSTR pszURL);
#define                 UrlIsFileUrlA(pszURL) UrlIsA(pszURL, URLIS_FILEURL)
#define                 UrlIsFileUrlW(pszURL) UrlIsW(pszURL, URLIS_FILEURL)
LWSTDAPI_(BOOL)         UrlIsA(_In_ PCSTR pszUrl, URLIS UrlIs);
LWSTDAPI_(BOOL)         UrlIsW(_In_ PCWSTR pszUrl, URLIS UrlIs);
LWSTDAPI_(LPCSTR)       UrlGetLocationA(_In_ PCSTR pszURL);
LWSTDAPI_(LPCWSTR)      UrlGetLocationW(_In_ PCWSTR pszURL);
LWSTDAPI                UrlUnescapeA(_Inout_ PSTR pszUrl, _Out_writes_to_opt_(*pcchUnescaped, *pcchUnescaped) PSTR pszUnescaped, _Inout_opt_ DWORD *pcchUnescaped, DWORD dwFlags);
LWSTDAPI                UrlUnescapeW(_Inout_ PWSTR pszUrl, _Out_writes_to_opt_(*pcchUnescaped, *pcchUnescaped) PWSTR pszUnescaped, _Inout_opt_ DWORD *pcchUnescaped, DWORD dwFlags);
LWSTDAPI                UrlEscapeA(_In_ PCSTR pszUrl, _Out_writes_to_(*pcchEscaped, *pcchEscaped) PSTR pszEscaped, _Inout_ DWORD *pcchEscaped, DWORD dwFlags);
LWSTDAPI                UrlEscapeW(_In_ PCWSTR pszUrl, _Out_writes_to_(*pcchEscaped, *pcchEscaped) PWSTR pszEscaped, _Inout_ DWORD *pcchEscaped, DWORD dwFlags);
LWSTDAPI                UrlCreateFromPathA(_In_ PCSTR pszPath, _Out_writes_to_(*pcchUrl, *pcchUrl) PSTR pszUrl, _Inout_ DWORD *pcchUrl, DWORD dwFlags);
LWSTDAPI                UrlCreateFromPathW(_In_ PCWSTR pszPath, _Out_writes_to_(*pcchUrl, *pcchUrl) PWSTR pszUrl, _Inout_ DWORD *pcchUrl, DWORD dwFlags);
LWSTDAPI                PathCreateFromUrlA(_In_ PCSTR pszUrl, _Out_writes_to_(*pcchPath, *pcchPath) PSTR pszPath, _Inout_ DWORD *pcchPath, DWORD dwFlags);
LWSTDAPI                PathCreateFromUrlW(_In_ PCWSTR pszUrl, _Out_writes_to_(*pcchPath, *pcchPath) PWSTR pszPath, _Inout_ DWORD *pcchPath, DWORD dwFlags);
#if (_WIN32_IE >= _WIN32_IE_IE70)
LWSTDAPI                PathCreateFromUrlAlloc(_In_ PCWSTR pszIn, _Outptr_ PWSTR *ppszOut, DWORD dwFlags);
#endif // _WIN32_IE_IE70
LWSTDAPI                UrlHashA(_In_ PCSTR pszUrl, _Out_writes_bytes_(cbHash) BYTE *pbHash, DWORD cbHash);
LWSTDAPI                UrlHashW(_In_ PCWSTR pszUrl, _Out_writes_bytes_(cbHash) BYTE *pbHash, DWORD cbHash);
LWSTDAPI                UrlGetPartW(_In_ PCWSTR pszIn, _Out_writes_(*pcchOut) PWSTR pszOut, _Inout_ DWORD *pcchOut, DWORD dwPart, DWORD dwFlags);
LWSTDAPI                UrlGetPartA(_In_ PCSTR pszIn, _Out_writes_(*pcchOut) PSTR pszOut, _Inout_ DWORD *pcchOut, DWORD dwPart, DWORD dwFlags);
LWSTDAPI                UrlApplySchemeA(_In_ PCSTR pszIn, _Out_writes_(*pcchOut) PSTR pszOut, _Inout_ DWORD *pcchOut, DWORD dwFlags);
LWSTDAPI                UrlApplySchemeW(_In_ PCWSTR pszIn, _Out_writes_(*pcchOut) PWSTR pszOut, _Inout_ DWORD *pcchOut, DWORD dwFlags);
LWSTDAPI                HashData(_In_reads_bytes_(cbData) BYTE *pbData, DWORD cbData, _Out_writes_bytes_(cbHash) BYTE *pbHash, DWORD cbHash);
LWSTDAPI                UrlFixupW(_In_ PCWSTR pcszUrl, _Out_writes_(cchMax) PWSTR pszTranslatedUrl, DWORD cchMax);

#ifdef UNICODE
#define UrlCompare              UrlCompareW
#define UrlCombine              UrlCombineW
#define UrlCanonicalize         UrlCanonicalizeW
#define UrlIsOpaque             UrlIsOpaqueW
#define UrlIsFileUrl            UrlIsFileUrlW
#define UrlGetLocation          UrlGetLocationW
#define UrlUnescape             UrlUnescapeW
#define UrlEscape               UrlEscapeW
#define UrlCreateFromPath       UrlCreateFromPathW
#define PathCreateFromUrl       PathCreateFromUrlW
#define UrlHash                 UrlHashW
#define UrlGetPart              UrlGetPartW
#define UrlApplyScheme          UrlApplySchemeW
#define UrlIs                   UrlIsW
#define UrlFixup                UrlFixupW
#else //!UNICODE
#define UrlCompare              UrlCompareA
#define UrlCombine              UrlCombineA
#define UrlCanonicalize         UrlCanonicalizeA
#define UrlIsOpaque             UrlIsOpaqueA
#define UrlIsFileUrl            UrlIsFileUrlA
#define UrlGetLocation          UrlGetLocationA
#define UrlUnescape             UrlUnescapeA
#define UrlEscape               UrlEscapeA
#define UrlCreateFromPath       UrlCreateFromPathA
#define PathCreateFromUrl       PathCreateFromUrlA
#define UrlHash                 UrlHashA
#define UrlGetPart              UrlGetPartA
#define UrlApplyScheme          UrlApplySchemeA
#define UrlIs                   UrlIsA
// no UrlFixupA
#endif //UNICODE

#define UrlEscapeSpaces(pszUrl, pszEscaped, pcchEscaped)        UrlCanonicalize(pszUrl, pszEscaped, pcchEscaped, URL_ESCAPE_SPACES_ONLY |URL_DONT_ESCAPE_EXTRA_INFO )
#define UrlUnescapeInPlace(pszUrl, dwFlags)                     UrlUnescape(pszUrl, NULL, NULL, dwFlags | URL_UNESCAPE_INPLACE)


typedef struct tagPARSEDURLA {
    DWORD     cbSize;
    // Pointers into the buffer that was provided to ParseURL
    LPCSTR    pszProtocol;
    UINT      cchProtocol;
    LPCSTR    pszSuffix;
    UINT      cchSuffix;
    UINT      nScheme;            // One of URL_SCHEME_*
    } PARSEDURLA, * PPARSEDURLA;
typedef struct tagPARSEDURLW {
    DWORD     cbSize;
    // Pointers into the buffer that was provided to ParseURL
    LPCWSTR   pszProtocol;
    UINT      cchProtocol;
    LPCWSTR   pszSuffix;
    UINT      cchSuffix;
    UINT      nScheme;            // One of URL_SCHEME_*
    } PARSEDURLW, * PPARSEDURLW;
#ifdef UNICODE
typedef PARSEDURLW PARSEDURL;
typedef PPARSEDURLW PPARSEDURL;
#else
typedef PARSEDURLA PARSEDURL;
typedef PPARSEDURLA PPARSEDURL;
#endif // UNICODE

LWSTDAPI            ParseURLA(_In_ LPCSTR pcszURL, _Inout_ PARSEDURLA * ppu);
LWSTDAPI            ParseURLW(_In_ LPCWSTR pcszURL, _Inout_ PARSEDURLW * ppu);
#ifdef UNICODE
#define ParseURL  ParseURLW
#else
#define ParseURL  ParseURLA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif //  NO_SHLWAPI_PATH



#ifndef NO_SHLWAPI_REG
//
//=============== Registry Routines ===================================
//


#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// SHDeleteEmptyKey mimics RegDeleteKey as it behaves on NT.
// SHDeleteKey mimics RegDeleteKey as it behaves on Win95.

LWSTDAPI_(LSTATUS)  SHDeleteEmptyKeyA(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubKey);
LWSTDAPI_(LSTATUS)  SHDeleteEmptyKeyW(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubKey);
#ifdef UNICODE
#define SHDeleteEmptyKey  SHDeleteEmptyKeyW
#else
#define SHDeleteEmptyKey  SHDeleteEmptyKeyA
#endif // !UNICODE
LWSTDAPI_(LSTATUS)  SHDeleteKeyA(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubKey);
LWSTDAPI_(LSTATUS)  SHDeleteKeyW(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubKey);
#ifdef UNICODE
#define SHDeleteKey  SHDeleteKeyW
#else
#define SHDeleteKey  SHDeleteKeyA
#endif // !UNICODE
LWSTDAPI_(HKEY)     SHRegDuplicateHKey(_In_ HKEY hkey);


// These functions open the key, get/set/delete the value, then close
// the key.

LWSTDAPI_(LSTATUS)    SHDeleteValueA(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubKey, _In_ LPCSTR pszValue);
LWSTDAPI_(LSTATUS)    SHDeleteValueW(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubKey, _In_ LPCWSTR pszValue);
#ifdef UNICODE
#define SHDeleteValue  SHDeleteValueW
#else
#define SHDeleteValue  SHDeleteValueA
#endif // !UNICODE
LWSTDAPI_(LSTATUS)    SHGetValueA(
    _In_                       HKEY hkey,
    _In_opt_                   LPCSTR  pszSubKey,
    _In_opt_                   LPCSTR  pszValue,
    _Out_opt_                  DWORD    *pdwType,
    _Out_writes_bytes_opt_(*pcbData) void     *pvData,
    _Inout_opt_                DWORD    *pcbData);
LWSTDAPI_(LSTATUS)    SHGetValueW(
    _In_                       HKEY hkey,
    _In_opt_                   LPCWSTR  pszSubKey,
    _In_opt_                   LPCWSTR  pszValue,
    _Out_opt_                  DWORD    *pdwType,
    _Out_writes_bytes_opt_(*pcbData) void     *pvData,
    _Inout_opt_                DWORD    *pcbData);
#ifdef UNICODE
#define SHGetValue  SHGetValueW
#else
#define SHGetValue  SHGetValueA
#endif // !UNICODE
LWSTDAPI_(LSTATUS)    SHSetValueA(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubKey, _In_opt_ LPCSTR pszValue, _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) LPCVOID pvData, _In_ DWORD cbData);
LWSTDAPI_(LSTATUS)    SHSetValueW(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubKey, _In_opt_ LPCWSTR pszValue, _In_ DWORD dwType,
    _In_reads_bytes_opt_(cbData) LPCVOID pvData, _In_ DWORD cbData);
#ifdef UNICODE
#define SHSetValue  SHSetValueW
#else
#define SHSetValue  SHSetValueA
#endif // !UNICODE

#if (_WIN32_IE >= 0x0602)
//
// SRRF - Shell Registry Routine Flags (for SHRegGetValue)
//

typedef int SRRF;

#define SRRF_RT_REG_NONE        0x00000001  // restrict type to REG_NONE      (other data types will not return ERROR_SUCCESS)
#define SRRF_RT_REG_SZ          0x00000002  // restrict type to REG_SZ        (other data types will not return ERROR_SUCCESS) (automatically converts REG_EXPAND_SZ to REG_SZ unless SRRF_NOEXPAND is specified)
#define SRRF_RT_REG_EXPAND_SZ   0x00000004  // restrict type to REG_EXPAND_SZ (other data types will not return ERROR_SUCCESS) (must specify SRRF_NOEXPAND or SHRegGetValue will fail with ERROR_INVALID_PARAMETER)
#define SRRF_RT_REG_BINARY      0x00000008  // restrict type to REG_BINARY    (other data types will not return ERROR_SUCCESS)
#define SRRF_RT_REG_DWORD       0x00000010  // restrict type to REG_DWORD     (other data types will not return ERROR_SUCCESS)
#define SRRF_RT_REG_MULTI_SZ    0x00000020  // restrict type to REG_MULTI_SZ  (other data types will not return ERROR_SUCCESS)
#define SRRF_RT_REG_QWORD       0x00000040  // restrict type to REG_QWORD     (other data types will not return ERROR_SUCCESS)

#define SRRF_RT_DWORD           (SRRF_RT_REG_BINARY | SRRF_RT_REG_DWORD) // restrict type to *32-bit* SRRF_RT_REG_BINARY or SRRF_RT_REG_DWORD (other data types will not return ERROR_SUCCESS)
#define SRRF_RT_QWORD           (SRRF_RT_REG_BINARY | SRRF_RT_REG_QWORD) // restrict type to *64-bit* SRRF_RT_REG_BINARY or SRRF_RT_REG_DWORD (other data types will not return ERROR_SUCCESS)
#define SRRF_RT_ANY             0x0000ffff                               // no type restriction

#define SRRF_RM_ANY             0x00000000  // no mode restriction (default is to allow any mode)
#define SRRF_RM_NORMAL          0x00010000  // restrict system startup mode to "normal boot"               (other startup modes will not return ERROR_SUCCESS)
#define SRRF_RM_SAFE            0x00020000  // restrict system startup mode to "safe mode"                 (other startup modes will not return ERROR_SUCCESS)
#define SRRF_RM_SAFENETWORK     0x00040000  // restrict system startup mode to "safe mode with networking" (other startup modes will not return ERROR_SUCCESS)

#define SRRF_NOEXPAND           0x10000000  // do not automatically expand environment strings if value is of type REG_EXPAND_SZ
#define SRRF_ZEROONFAILURE      0x20000000  // if pvData is not NULL, set content to all zeros on failure
#define SRRF_NOVIRT             0x40000000  // if the requested key is virtualized, then fail with ERROR_FILE_NOT_FOUND


// Function:
//
//  SHRegGetValue()
//
// Purpose:
//
//  Gets a registry value.  SHRegGetValue() provides the following benefits:
//
//  - data type checking
//  - boot mode checking
//  - auto-expansion of REG_EXPAND_SZ data
//  - guaranteed NULL termination of REG_SZ, REG_EXPAND_SZ, REG_MULTI_SZ data
//
// Parameters:
//
//  hkey        - handle to a currently open key.
//
//  pszSubKey   - pointer to a null-terminated string specifying the relative
//                path from hkey to one of its subkeys from which the data is
//                to be retrieved.  this will be opened with KEY_READ sam.
//
//                Note1: pszSubKey can be NULL or "".  In either of these two
//                       cases, the data is retrieved from the hkey itself.
//                Note2: *** PERF ***
//                       If pszSubKey is not NULL or "", the subkey will be
//                       automatically be opened and closed by this routine
//                       in order to obtain the data.  If you are retrieving
//                       multiple values from the same subkey, it is better
//                       for perf to open the subkey via RegOpenKeyEx() prior
//                       to calling this method, and using this opened key as
//                       hkey with pszSubKey set to NULL.
//
//  pszValue    - pointer to a null-terminated string specifying the name of
//                the value to query for data
//
//                Note1: pszValue can be NULL or "".  In either of these two
//                       cases, the data is retrieved from the unnamed or
//                       default value.
//
//  srrfFlags   - bitwise or of SRRF_ flags, which cannot be 0:  at least one
//                type restriction must be specified (SRRF_RT_...), or if any
//                type is desired then SRRF_RT_ANY can be specified
//
//                Note1: SRRF_RT_ANY will allow any data type to be returned.
//                Note2: The following two type restrictions have special
//                       handling semantics:
//
//                         SRRF_RT_DWORD == SRRF_RT_REG_BINARY | SRRF_RT_REG_DWORD
//                         SRRF_RT_QWORD == SRRF_RT_REG_BINARY | SRRF_RT_REG_QWORD
//
//                       If either of these are specified, with no other type
//                       restrictions, then in the prior case the restriction
//                       will limit "valid" returned data to either REG_DWORD
//                       or 32-bit REG_BINARY data, and in the latter case
//                       the restriction will limit "valid" returned data to
//                       either REG_QWORD or 64-bit REG_BINARY.
//
//  pdwType     - pointer to a dword which receives a code indicating the
//                type of data stored in the specified value
//
//                Note1: pdwType can be NULL if no type information is wanted
//                Note2: If pdwType is not NULL, and the SRRF_NOEXPAND flag
//                       has not been set, data types of REG_EXPAND_SZ will
//                       be returned as REG_SZ since they are automatically
//                       expanded in this method.
//
//  pvData      - pointer to a buffer that receives the value's data
//
//                Note1: pvData can be NULL if the data is not required.
//                       pvData is usually NULL if doing either a simple
//                       existence test, or if interested in the size only.
//                Note2: *** PERF ***
//                       Reference 'perf' note for pcbData.
//
//  pcbData     - when pvData is NULL:
//                  optional pointer to a dword that receives a size in bytes
//                  which would be sufficient to hold the registry data (note
//                  this size is not guaranteed to be exact, merely sufficient)
//                when pvData is not NULL:
//                  required pointer to a dword that specifies the size in
//                  bytes of the buffer pointed to by the pvData parameter
//                  and receives a size in bytes of:
//                  a) the number of bytes read into pvData on ERROR_SUCCESS
//                     (note this size is guaranteed to be exact)
//                  b) the number of bytes which would be sufficient to hold
//                     the registry data on ERROR_MORE_DATA -- pvData was of
//                     insufficient size (note this size is not guaranteed to
//                     be exact, merely sufficient)
//
//                Note1: pcbData can be NULL only if pvData is NULL.
//                Note2: *** PERF ***
//                       The potential for an 'extra' call to the registry to
//                       read (or re-read) in the data exists when the data
//                       type is REG_EXPAND_SZ and the SRRF_NOEXPAND flag has
//                       not been set.  The following conditions will result
//                       in this 'extra' read operation:
//                       i)  when pvData is NULL and pcbData is not NULL
//                           we must read in the data from the registry
//                           anyway in order to obtain the string and perform
//                           an expand on it to obtain and return the total
//                           required size in pcbData
//                       ii) when pvData is not NULL but is of insufficient
//                           size we must re-read in the data from the
//                           registry in order to obtain the entire string
//                           and perform an expand on it to obtain and return
//                           the total required size in pcbData
//
// Remarks:
//
//  The key identified by hkey must have been opened with KEY_QUERY_VALUE
//  access.  If pszSubKey is not NULL or "", it must be able to be opened
//  with KEY_QUERY_VALUE access in the current calling context.
//
//  If the data type is REG_SZ, REG_EXPAND_SZ or REG_MULTI_SZ then any
//  returned data is guaranteed to take into account proper null termination.
//  For example:  if pcbData is not NULL, its returned size will include the
//  bytes for a null terminator  if pvData is not NULL, its returned data
//  will be properly null terminated.
//
//  If the data type is REG_EXPAND_SZ, then unless the SRRF_NOEXPAND flag
//  is set the data will be automatically expanded prior to being returned.
//  For example:  if pdwType is not NULL, its returned type will be changed
//  to REG_SZ,  if pcbData is not NULL, its returned size will include the
//  bytes for a properly expanded string.  if pvData is not NULL, its
//  returned data will be the expanded version of the string.
//
//  Reference MSDN documentation for RegQueryValueEx() for more information
//  of the behaviour when pdwType, pvData, and/or pcbData are equal to NULL.
//
// Return Values:
//
//  If the function succeeds, the return value is ERROR_SUCCESS and all out
//  parameters requested (pdwType, pvData, pcbData) are valid.
//
//  If the function fails due to insufficient space in a provided non-NULL
//  pvData, the return value is ERROR_MORE_DATA and only pdwType and pcbData
//  can contain valid data.  The content of pvData in this case is undefined.
//
// Examples:
//
//  1) read REG_SZ (or REG_EXPAND_SZ as REG_SZ) "string" data from the (default) value of an open hkey
//
//      TCHAR szData[128]
//      DWORD cbData = sizeof(pszData)
//      if (ERROR_SUCCESS == SHRegGetValue(hkey, NULL, NULL, SRRF_RT_REG_SZ, NULL, szData, &cbData))
//      {
//          // use sz (successful read)
//      }
//
//  2) read REG_SZ (or REG_EXPAND_SZ as REG_SZ) "string" data of unknown size from the "MyValue" value of an open hkey
//
//      DWORD cbData
//      if (ERROR_SUCCESS == SHRegGetValue(hkey, NULL, TEXT("MyValue"), SRRF_RT_REG_SZ, NULL, NULL, &cbData))
//      {
//          TCHAR *pszData = new TCHAR[cbData/sizeof(TCHAR)]
//          if (pszData)
//          {
//              if (ERROR_SUCCESS == SHRegGetValue(hkey, NULL, TEXT("MyValue"), SRRF_RT_REG_SZ, NULL, pszData, &cbData))
//              {
//                  // use pszData (successful read)
//              }
//              delete[] pszData
//          }
//      }
//
//  3) read "dword" data from the "MyValue" value of the "MySubKey" subkey of an open hkey
//
//      DWORD dwData
//      DWORD cbData = sizeof(dwData)
//      if (ERROR_SUCCESS == SHRegGetValue(hkey, TEXT("MySubKey"), TEXT("MyValue"), SRRF_RT_REG_DWORD, NULL, &dwData, &cbData))
//      {
//          // use dwData (successful read)
//      }
//
//  4) read "dword" data from the "MyValue" value of the "MySubKey" subkey of an open hkey (32-bit binary data also ok)
//
//      DWORD dwData
//      DWORD cbData = sizeof(dwData)
//      if (ERROR_SUCCESS == SHRegGetValue(hkey, TEXT("MySubKey"), TEXT("MyValue"), SRRF_RT_DWORD, NULL, &dwData, &cbData))
//      {
//          // use dwData (successful read)
//      }
//
//  5) determine existence of "MyValue" value of an open hkey
//
//      BOOL bExists = ERROR_SUCCESS == SHRegGetValue(hkey, NULL, TEXT("MyValue"), SRRF_RT_ANY, NULL, NULL, NULL)

LWSTDAPI_(LSTATUS) SHRegGetValueA(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubKey, _In_opt_ LPCSTR pszValue,
    _In_ SRRF srrfFlags, _Out_opt_ DWORD *pdwType, _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void *pvData,
    _Inout_opt_ DWORD *pcbData);
LWSTDAPI_(LSTATUS) SHRegGetValueW(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubKey, _In_opt_ LPCWSTR pszValue,
    _In_ SRRF srrfFlags, _Out_opt_ DWORD *pdwType, _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void *pvData,
    _Inout_opt_ DWORD *pcbData);
#ifdef UNICODE
#define SHRegGetValue  SHRegGetValueW
#else
#define SHRegGetValue  SHRegGetValueA
#endif // !UNICODE

LWSTDAPI_(LSTATUS) SHRegSetValue(_In_ HKEY    hkey, _In_opt_ LPCWSTR pszSubKey, _In_opt_ LPCWSTR pszValue, _In_ SRRF srrfFlags,
    _In_ DWORD dwType, _In_reads_bytes_opt_(cbData) LPCVOID pvData, _In_opt_ DWORD cbData);

LWSTDAPI_(LSTATUS) SHRegGetValueFromHKCUHKLM(_In_ PCWSTR pwszKey, _In_opt_ PCWSTR pwszValue, _In_ SRRF srrfFlags,
                                        _Out_opt_ DWORD* pdwType, _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void* pvData,
                                        _Inout_opt_ _When_(pvData != 0, _Pre_notnull_) DWORD * pcbData);
STDAPI_(BOOL) SHRegGetBoolValueFromHKCUHKLM(_In_ PCWSTR pszKey, _In_opt_ PCWSTR pszValue, _In_ BOOL fDefault);
#endif  // (_WIN32_IE >= 0x0602)

// These functions behave just like RegQueryValueEx(), except if the data
// type is REG_SZ, REG_EXPAND_SZ or REG_MULTI_SZ then the string is
// guaranteed to be properly null terminated.
//
// Additionally, if the data type is REG_EXPAND_SZ these functions will
// go ahead and expand out the string, and "massage" the returned *pdwType
// to be REG_SZ.
LWSTDAPI_(LSTATUS) SHQueryValueExA(
    _In_                       HKEY      hkey,
    _In_opt_                   LPCSTR  pszValue,
    _Reserved_                 DWORD    *pdwReserved,
    _Out_opt_                  DWORD    *pdwType,
    _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void *pvData,
    _Inout_opt_                DWORD    *pcbData);
// These functions behave just like RegQueryValueEx(), except if the data
// type is REG_SZ, REG_EXPAND_SZ or REG_MULTI_SZ then the string is
// guaranteed to be properly null terminated.
//
// Additionally, if the data type is REG_EXPAND_SZ these functions will
// go ahead and expand out the string, and "massage" the returned *pdwType
// to be REG_SZ.
LWSTDAPI_(LSTATUS) SHQueryValueExW(
    _In_                       HKEY      hkey,
    _In_opt_                   LPCWSTR  pszValue,
    _Reserved_                 DWORD    *pdwReserved,
    _Out_opt_                  DWORD    *pdwType,
    _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void *pvData,
    _Inout_opt_                DWORD    *pcbData);
#ifdef UNICODE
#define SHQueryValueEx  SHQueryValueExW
#else
#define SHQueryValueEx  SHQueryValueExA
#endif // !UNICODE

// Enumeration functions support.

LWSTDAPI_(LSTATUS)     SHEnumKeyExA(_In_ HKEY hkey, _In_ DWORD dwIndex, _Out_writes_(*pcchName) LPSTR pszName, _Inout_ LPDWORD pcchName);
LWSTDAPI_(LSTATUS)     SHEnumKeyExW(_In_ HKEY hkey, _In_ DWORD dwIndex, _Out_writes_(*pcchName) LPWSTR pszName, _Inout_ LPDWORD pcchName);
LWSTDAPI_(LSTATUS)     SHEnumValueA(_In_ HKEY hkey, _In_ DWORD dwIndex, _Out_writes_opt_(*pcchValueName) PSTR pszValueName,
    _Inout_opt_ LPDWORD pcchValueName, _Out_opt_ LPDWORD pdwType, _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void *pvData,
    _Inout_opt_ LPDWORD pcbData);
LWSTDAPI_(LSTATUS)     SHEnumValueW(_In_ HKEY hkey, _In_ DWORD dwIndex, _Out_writes_opt_(*pcchValueName) PWSTR pszValueName,
    _Inout_opt_ LPDWORD pcchValueName, _Out_opt_ LPDWORD pdwType, _Out_writes_bytes_to_opt_(*pcbData,*pcbData) void *pvData,
    _Inout_opt_ LPDWORD pcbData);
LWSTDAPI_(LSTATUS)     SHQueryInfoKeyA(_In_ HKEY hkey, _Out_opt_ LPDWORD pcSubKeys, _Out_opt_ LPDWORD pcchMaxSubKeyLen, _Out_opt_ LPDWORD pcValues, _Out_opt_ LPDWORD pcchMaxValueNameLen);
LWSTDAPI_(LSTATUS)     SHQueryInfoKeyW(_In_ HKEY hkey, _Out_opt_ LPDWORD pcSubKeys, _Out_opt_ LPDWORD pcchMaxSubKeyLen, _Out_opt_ LPDWORD pcValues, _Out_opt_ LPDWORD pcchMaxValueNameLen);

// recursive key copy
LWSTDAPI_(LSTATUS)     SHCopyKeyA(_In_ HKEY hkeySrc, _In_opt_ LPCSTR  pszSrcSubKey, _In_ HKEY hkeyDest, _Reserved_ DWORD fReserved);
LWSTDAPI_(LSTATUS)     SHCopyKeyW(_In_ HKEY hkeySrc, _In_opt_ LPCWSTR pszSrcSubKey, _In_ HKEY hkeyDest, _Reserved_ DWORD fReserved);

// Getting and setting file system paths with environment variables

LWSTDAPI_(LSTATUS)    SHRegGetPathA(_In_ HKEY hKey, _In_opt_ LPCSTR pcszSubKey, _In_opt_ LPCSTR pcszValue, _Out_writes_(MAX_PATH) LPSTR pszPath, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)    SHRegGetPathW(_In_ HKEY hKey, _In_opt_ LPCWSTR pcszSubKey, _In_opt_ LPCWSTR pcszValue, _Out_writes_(MAX_PATH) LPWSTR pszPath, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)    SHRegSetPathA(_In_ HKEY hKey, _In_opt_ LPCSTR pcszSubKey, _In_opt_ LPCSTR pcszValue, _In_ LPCSTR pcszPath, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)    SHRegSetPathW(_In_ HKEY hKey, _In_opt_ LPCWSTR pcszSubKey, _In_opt_ LPCWSTR pcszValue, _In_ LPCWSTR pcszPath, _In_ DWORD dwFlags);

#ifdef UNICODE
#define SHEnumKeyEx           SHEnumKeyExW
#define SHEnumValue           SHEnumValueW
#define SHQueryInfoKey        SHQueryInfoKeyW
#define SHCopyKey             SHCopyKeyW
#define SHRegGetPath          SHRegGetPathW
#define SHRegSetPath          SHRegSetPathW
#else
#define SHEnumKeyEx           SHEnumKeyExA
#define SHEnumValue           SHEnumValueA
#define SHQueryInfoKey        SHQueryInfoKeyA
#define SHCopyKey             SHCopyKeyA
#define SHRegGetPath          SHRegGetPathA
#define SHRegSetPath          SHRegSetPathA
#endif


//////////////////////////////////////////////
// User Specific Registry Access Functions
//////////////////////////////////////////////

typedef enum
{
    SHREGDEL_DEFAULT = 0x00000000,       // Delete's HKCU, or HKLM if HKCU is not found.
    SHREGDEL_HKCU    = 0x00000001,       // Delete HKCU only
    SHREGDEL_HKLM    = 0x00000010,       // Delete HKLM only.
    SHREGDEL_BOTH    = 0x00000011,       // Delete both HKCU and HKLM.
} SHREGDEL_FLAGS;

typedef enum
{
    SHREGENUM_DEFAULT = 0x00000000,       // Enumerates HKCU or HKLM if not found.
    SHREGENUM_HKCU    = 0x00000001,       // Enumerates HKCU only
    SHREGENUM_HKLM    = 0x00000010,       // Enumerates HKLM only.
    SHREGENUM_BOTH    = 0x00000011,       // Enumerates both HKCU and HKLM without duplicates.
                                          // This option is NYI.
} SHREGENUM_FLAGS;

#define     SHREGSET_HKCU           0x00000001       // Write to HKCU if empty.
#define     SHREGSET_FORCE_HKCU     0x00000002       // Write to HKCU.
#define     SHREGSET_HKLM           0x00000004       // Write to HKLM if empty.
#define     SHREGSET_FORCE_HKLM     0x00000008       // Write to HKLM.
#define     SHREGSET_DEFAULT        (SHREGSET_FORCE_HKCU | SHREGSET_HKLM)          // Default is SHREGSET_FORCE_HKCU | SHREGSET_HKLM.

typedef HANDLE HUSKEY;  // HUSKEY is a Handle to a User Specific KEY.
typedef HUSKEY *PHUSKEY;

LWSTDAPI_(LSTATUS)     SHRegCreateUSKeyA(_In_ LPCSTR pszPath, _In_ REGSAM samDesired, _In_opt_ HUSKEY hRelativeUSKey, _Out_ PHUSKEY phNewUSKey, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)     SHRegCreateUSKeyW(_In_ LPCWSTR pwzPath, _In_ REGSAM samDesired, _In_opt_ HUSKEY hRelativeUSKey, _Out_ PHUSKEY phNewUSKey, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)     SHRegOpenUSKeyA(_In_ LPCSTR pszPath, _In_ REGSAM samDesired, _In_opt_ HUSKEY hRelativeUSKey, _Out_ PHUSKEY phNewUSKey, _In_ BOOL fIgnoreHKCU);
LWSTDAPI_(LSTATUS)     SHRegOpenUSKeyW(_In_ LPCWSTR pwzPath, _In_ REGSAM samDesired, _In_opt_ HUSKEY hRelativeUSKey, _Out_ PHUSKEY phNewUSKey, _In_ BOOL fIgnoreHKCU);
LWSTDAPI_(LSTATUS)     SHRegQueryUSValueA(
    _In_                                      HUSKEY  hUSKey,
    _In_opt_                                  LPCSTR  pszValue,
    _Inout_opt_                               DWORD * pdwType,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *  pvData,
    _Inout_opt_                               DWORD * pcbData,
    _In_                                      BOOL    fIgnoreHKCU,
    _In_reads_bytes_opt_(dwDefaultDataSize)        void *  pvDefaultData,
    _In_opt_                                  DWORD   dwDefaultDataSize);
LWSTDAPI_(LSTATUS)     SHRegQueryUSValueW(
    _In_                                      HUSKEY  hUSKey,
    _In_opt_                                  LPCWSTR  pszValue,
    _Inout_opt_                               DWORD * pdwType,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *  pvData,
    _Inout_opt_                               DWORD * pcbData,
    _In_                                      BOOL    fIgnoreHKCU,
    _In_reads_bytes_opt_(dwDefaultDataSize)        void *  pvDefaultData,
    _In_opt_                                  DWORD   dwDefaultDataSize);
LWSTDAPI_(LSTATUS)     SHRegWriteUSValueA(_In_ HUSKEY hUSKey, _In_ LPCSTR pszValue, _In_ DWORD dwType, _In_reads_bytes_(cbData) const void *pvData, _In_ DWORD cbData, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)     SHRegWriteUSValueW(_In_ HUSKEY hUSKey, _In_ LPCWSTR pwzValue, _In_ DWORD dwType, _In_reads_bytes_(cbData) const void *pvData, _In_ DWORD cbData, _In_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)     SHRegDeleteUSValueA(_In_ HUSKEY hUSKey, _In_ LPCSTR pszValue, _In_ SHREGDEL_FLAGS delRegFlags);
LWSTDAPI_(LSTATUS)     SHRegDeleteUSValueW(_In_ HUSKEY hUSKey, _In_ LPCWSTR pwzValue, _In_ SHREGDEL_FLAGS delRegFlags);
LWSTDAPI_(LSTATUS)     SHRegDeleteEmptyUSKeyW(_In_ HUSKEY hUSKey, _In_ LPCWSTR pwzSubKey, _In_ SHREGDEL_FLAGS delRegFlags);
LWSTDAPI_(LSTATUS)     SHRegDeleteEmptyUSKeyA(_In_ HUSKEY hUSKey, _In_ LPCSTR pszSubKey, _In_ SHREGDEL_FLAGS delRegFlags);
LWSTDAPI_(LSTATUS)     SHRegEnumUSKeyA(_In_ HUSKEY hUSKey, _In_ DWORD dwIndex, _Out_writes_to_(*pcchName,*pcchName) LPSTR pszName, _Inout_ LPDWORD pcchName, _In_ SHREGENUM_FLAGS enumRegFlags);
LWSTDAPI_(LSTATUS)     SHRegEnumUSKeyW(_In_ HUSKEY hUSKey, _In_ DWORD dwIndex, _Out_writes_to_(*pcchName,*pcchName) LPWSTR pwzName, _Inout_ LPDWORD pcchName, _In_ SHREGENUM_FLAGS enumRegFlags);
LWSTDAPI_(LSTATUS)     SHRegEnumUSValueA(_In_ HUSKEY hUSkey, _In_ DWORD dwIndex,
    _Out_writes_to_(*pcchValueName,*pcchValueName) LPSTR pszValueName, _Inout_ LPDWORD pcchValueName, _Out_opt_ LPDWORD pdwType,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData, _Inout_opt_ LPDWORD pcbData, _In_ SHREGENUM_FLAGS enumRegFlags);
LWSTDAPI_(LSTATUS)     SHRegEnumUSValueW(_In_ HUSKEY hUSkey, _In_ DWORD dwIndex,
    _Out_writes_to_(*pcchValueName,*pcchValueName) LPWSTR pszValueName, _Inout_ LPDWORD pcchValueName, _Out_opt_ LPDWORD pdwType,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void *pvData, _Inout_opt_ LPDWORD pcbData, _In_ SHREGENUM_FLAGS enumRegFlags);
LWSTDAPI_(LSTATUS)     SHRegQueryInfoUSKeyA(_In_ HUSKEY hUSKey, _Out_opt_ LPDWORD pcSubKeys, _Out_opt_ LPDWORD pcchMaxSubKeyLen, _Out_opt_ LPDWORD pcValues, _Out_opt_ LPDWORD pcchMaxValueNameLen, _In_ SHREGENUM_FLAGS enumRegFlags);
LWSTDAPI_(LSTATUS)     SHRegQueryInfoUSKeyW(_In_ HUSKEY hUSKey, _Out_opt_ LPDWORD pcSubKeys, _Out_opt_ LPDWORD pcchMaxSubKeyLen, _Out_opt_ LPDWORD pcValues, _Out_opt_ LPDWORD pcchMaxValueNameLen, _In_ SHREGENUM_FLAGS enumRegFlags);
LWSTDAPI_(LSTATUS)     SHRegCloseUSKey(_In_ HUSKEY hUSKey);


// These calls are equal to an SHRegOpenUSKey, SHRegQueryUSValue, and then a SHRegCloseUSKey.
STDAPI_(LSTATUS) SHRegGetUSValueA(
    _In_                                      LPCSTR  pszSubKey,
    _In_opt_                                  LPCSTR  pszValue,
    _Inout_opt_                               DWORD  *pdwType,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void   *pvData,
    _Inout_opt_                               DWORD  *pcbData,
    _In_                                      BOOL    fIgnoreHKCU,
    _In_reads_bytes_opt_(dwDefaultDataSize)        void   *pvDefaultData,
    _In_                                      DWORD   dwDefaultDataSize);
STDAPI_(LSTATUS) SHRegGetUSValueW(
    _In_                                      LPCWSTR  pszSubKey,
    _In_opt_                                  LPCWSTR  pszValue,
    _Inout_opt_                               DWORD  *pdwType,
    _Out_writes_bytes_to_opt_(*pcbData, *pcbData) void   *pvData,
    _Inout_opt_                               DWORD  *pcbData,
    _In_                                      BOOL    fIgnoreHKCU,
    _In_reads_bytes_opt_(dwDefaultDataSize)        void   *pvDefaultData,
    _In_                                      DWORD   dwDefaultDataSize);
LWSTDAPI_(LSTATUS)     SHRegSetUSValueA(_In_ LPCSTR pszSubKey, _In_ LPCSTR pszValue, _In_ DWORD dwType, _In_reads_bytes_opt_(cbData) const void *pvData, _In_opt_ DWORD cbData, _In_opt_ DWORD dwFlags);
LWSTDAPI_(LSTATUS)     SHRegSetUSValueW(_In_ LPCWSTR pwzSubKey, _In_ LPCWSTR pwzValue, _In_ DWORD dwType, _In_reads_bytes_opt_(cbData) const void *pvData, _In_opt_ DWORD cbData, _In_opt_ DWORD dwFlags);
LWSTDAPI_(int)         SHRegGetIntW(_In_ HKEY hk, _In_opt_ PCWSTR pwzKey, _In_ int iDefault);

#ifdef UNICODE
#define SHRegCreateUSKey        SHRegCreateUSKeyW
#define SHRegOpenUSKey          SHRegOpenUSKeyW
#define SHRegQueryUSValue       SHRegQueryUSValueW
#define SHRegWriteUSValue       SHRegWriteUSValueW
#define SHRegDeleteUSValue      SHRegDeleteUSValueW
#define SHRegDeleteEmptyUSKey   SHRegDeleteEmptyUSKeyW
#define SHRegEnumUSKey          SHRegEnumUSKeyW
#define SHRegEnumUSValue        SHRegEnumUSValueW
#define SHRegQueryInfoUSKey     SHRegQueryInfoUSKeyW
#define SHRegGetUSValue         SHRegGetUSValueW
#define SHRegSetUSValue         SHRegSetUSValueW
#define SHRegGetInt             SHRegGetIntW
#else
#define SHRegCreateUSKey        SHRegCreateUSKeyA
#define SHRegOpenUSKey          SHRegOpenUSKeyA
#define SHRegQueryUSValue       SHRegQueryUSValueA
#define SHRegWriteUSValue       SHRegWriteUSValueA
#define SHRegDeleteUSValue      SHRegDeleteUSValueA
#define SHRegDeleteEmptyUSKey   SHRegDeleteEmptyUSKeyA
#define SHRegEnumUSKey          SHRegEnumUSKeyA
#define SHRegEnumUSValue        SHRegEnumUSValueA
#define SHRegQueryInfoUSKey     SHRegQueryInfoUSKeyA
#define SHRegGetUSValue         SHRegGetUSValueA
#define SHRegSetUSValue         SHRegSetUSValueA
#endif

LWSTDAPI_(BOOL) SHRegGetBoolUSValueA(_In_ LPCSTR pszSubKey, _In_opt_ LPCSTR pszValue, _In_ BOOL fIgnoreHKCU, _In_ BOOL fDefault);
LWSTDAPI_(BOOL) SHRegGetBoolUSValueW(_In_ LPCWSTR pszSubKey, _In_opt_ LPCWSTR pszValue, _In_ BOOL fIgnoreHKCU, _In_ BOOL fDefault);
#ifdef UNICODE
#define SHRegGetBoolUSValue  SHRegGetBoolUSValueW
#else
#define SHRegGetBoolUSValue  SHRegGetBoolUSValueA
#endif // !UNICODE

//
//  Association APIs
//
//  these APIs are to assist in accessing the data in HKCR
//  getting the Command strings and exe paths
//  for different verbs and extensions are simplified this way
//

enum
{
    ASSOCF_NONE                        = 0x00000000,
    ASSOCF_INIT_NOREMAPCLSID           = 0x00000001,  //  do not remap clsids to progids
    ASSOCF_INIT_BYEXENAME              = 0x00000002,  //  executable is being passed in
    ASSOCF_OPEN_BYEXENAME              = 0x00000002,  //  executable is being passed in
    ASSOCF_INIT_DEFAULTTOSTAR          = 0x00000004,  //  treat "*" as the BaseClass
    ASSOCF_INIT_DEFAULTTOFOLDER        = 0x00000008,  //  treat "Folder" as the BaseClass
    ASSOCF_NOUSERSETTINGS              = 0x00000010,  //  dont use HKCU
    ASSOCF_NOTRUNCATE                  = 0x00000020,  //  dont truncate the return string
    ASSOCF_VERIFY                      = 0x00000040,  //  verify data is accurate (DISK HITS)
    ASSOCF_REMAPRUNDLL                 = 0x00000080,  //  actually gets info about rundlls target if applicable
    ASSOCF_NOFIXUPS                    = 0x00000100,  //  attempt to fix errors if found
    ASSOCF_IGNOREBASECLASS             = 0x00000200,  //  dont recurse into the baseclass
    ASSOCF_INIT_IGNOREUNKNOWN          = 0x00000400,  //  dont use the "Unknown" progid, instead fail
#if (NTDDI_VERSION >= NTDDI_WIN8)
    ASSOCF_INIT_FIXED_PROGID           = 0x00000800,  //  the Init() pszAssoc value is a ProgId that should not be mapped using the current user defaults
    ASSOCF_IS_PROTOCOL                 = 0x00001000,  //  the Init() pszAssoc value is an uri scheme (not including the ":") that should be mapped using the current user defaults
    ASSOCF_INIT_FOR_FILE               = 0x00002000,  //  use this flag when specifying ASSOCF_INIT_FIXED_PROGID if the ProgId corresponds with a file extension based association
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
    ASSOCF_IS_FULL_URI                 = 0x00004000,  //  Used to specify that full http/https URI is being passed for target resolution
                                                      //  Only one of ASSOCF_INIT_FIXED_PROGID, ASSOCF_IS_PROTOCOL or ASSOCF_IS_FULL_URI can be specified at a time.
    ASSOCF_PER_MACHINE_ONLY            = 0x00008000,  //  Enforces per-machine association look-up only and avoid HKCU.
#endif
#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
    // For http/https uri associations, this enables selecting the OS default browser, that is a controlled by
    // policy settings, instead of the default browser.
    ASSOCF_APP_TO_APP                  = 0x00010000,
#endif
};
typedef DWORD ASSOCF;

typedef enum
{
    ASSOCSTR_COMMAND      = 1,  //  shell\verb\command string
    ASSOCSTR_EXECUTABLE,        //  the executable part of command string
    ASSOCSTR_FRIENDLYDOCNAME,   //  friendly name of the document type
    ASSOCSTR_FRIENDLYAPPNAME,   //  friendly name of executable
    ASSOCSTR_NOOPEN,            //  noopen value
    ASSOCSTR_SHELLNEWVALUE,     //  query values under the shellnew key
    ASSOCSTR_DDECOMMAND,        //  template for DDE commands
    ASSOCSTR_DDEIFEXEC,         //  DDECOMMAND to use if just create a process
    ASSOCSTR_DDEAPPLICATION,    //  Application name in DDE broadcast
    ASSOCSTR_DDETOPIC,          //  Topic Name in DDE broadcast
    ASSOCSTR_INFOTIP,           //  info tip for an item, or list of properties to create info tip from
#if (_WIN32_IE >= _WIN32_IE_IE60)
    ASSOCSTR_QUICKTIP,          //  same as ASSOCSTR_INFOTIP, except, this list contains only quickly retrievable properties
    ASSOCSTR_TILEINFO,          //  similar to ASSOCSTR_INFOTIP - lists important properties for tileview
    ASSOCSTR_CONTENTTYPE,       //  MIME Content type
    ASSOCSTR_DEFAULTICON,       //  Default icon source
    ASSOCSTR_SHELLEXTENSION,    //  Guid string pointing to the Shellex\Shellextensionhandler value.
#endif // _WIN32_IE_IE60
#if (_WIN32_IE >= _WIN32_IE_IE80)
    ASSOCSTR_DROPTARGET,        //  The CLSID of DropTarget
    ASSOCSTR_DELEGATEEXECUTE,   //  The CLSID of DelegateExecute
#endif // _WIN32_IE_IE80
    // a string value of the uri protocol schemes, for example "http:https:ftp:file:" or "*" indicating all
    ASSOCSTR_SUPPORTED_URI_PROTOCOLS,
#if (NTDDI_VERSION >= NTDDI_WIN10)
    ASSOCSTR_PROGID,            // The ProgId provided by the app associated with the file type or uri scheme based on user default settings.
    ASSOCSTR_APPID,             // The AppUserModelID of the app associated with the file type or uri scheme based on user default settings.
    ASSOCSTR_APPPUBLISHER,      // THe publisher of the app associated with the file type or uri scheme based on user default settings.
    ASSOCSTR_APPICONREFERENCE,  // The icon reference of the app associated with the file type or uri scheme based on user default settings.
#endif // NTDDI_WIN10
    ASSOCSTR_MAX                //  last item in enum...
} ASSOCSTR;

typedef enum
{
    ASSOCKEY_SHELLEXECCLASS = 1,  //  the key that should be passed to ShellExec(hkeyClass)
    ASSOCKEY_APP,                 //  the "Application" key for the association
    ASSOCKEY_CLASS,               //  the progid or class key
    ASSOCKEY_BASECLASS,           //  the BaseClass key
    ASSOCKEY_MAX                  //  last item in enum...
} ASSOCKEY;

typedef enum
{
    ASSOCDATA_MSIDESCRIPTOR = 1,  //  Component Descriptor to pass to MSI APIs
    ASSOCDATA_NOACTIVATEHANDLER,  //  restrict attempts to activate window
    ASSOCDATA_UNUSED1,            //  removed QUERYCLASSSTORE, dead code
    ASSOCDATA_HASPERUSERASSOC,    //  defaults to user specified association
#if (_WIN32_IE >= _WIN32_IE_IE60)
    ASSOCDATA_EDITFLAGS,          //  Edit flags.
    ASSOCDATA_VALUE,              //  use pszExtra as the Value name
#endif // _WIN32_IE_IE60
    ASSOCDATA_MAX
} ASSOCDATA;

typedef enum
{
    ASSOCENUM_NONE
} ASSOCENUM;

// Stored under HKCR\<progId> EditFlags(REG_DWORD)
//
// Retrieve these values using IQueryAssociations::GetData as follows
//
// DWORD editFlags, size = sizeof(editFlags);
// queryAssoc->GetData(nullptr, ASSOCDATA_EDITFLAGS, nullptr, &editFlags, &size);
//
// Some of these flags are no longer used since editing file type associations has been
// removed from Explorer's folder options UI.

typedef enum
{
    FTA_None                    = 0x00000000,
    FTA_Exclude                 = 0x00000001, // used to exclude (hide) types like drvfile
    FTA_Show                    = 0x00000002, // used to show types like folder that don't have associations
    FTA_HasExtension            = 0x00000004, // type has a file name extension
    FTA_NoEdit                  = 0x00000008, // no editing of file type
    FTA_NoRemove                = 0x00000010, // no removing of the file type
    FTA_NoNewVerb               = 0x00000020, // no adding of verbs
    FTA_NoEditVerb              = 0x00000040, // no editing of predefined verbs
    FTA_NoRemoveVerb            = 0x00000080, // no removing of predefined verbs
    FTA_NoEditDesc              = 0x00000100, // no editing of file type description
    FTA_NoEditIcon              = 0x00000200, // no editing of doc icon
    FTA_NoEditDflt              = 0x00000400, // no changing of default verb
    FTA_NoEditVerbCmd           = 0x00000800, // no modification of the commnds associated with the verbs
    FTA_NoEditVerbExe           = 0x00001000, // no editing of the verb's exe
    FTA_NoDDE                   = 0x00002000, // no editing of the DDE fields

    FTA_NoEditMIME              = 0x00008000, // no editing of the Content Type or Default Extension fields
    FTA_OpenIsSafe              = 0x00010000, // the open verb should be invoked automaticaly for downloaded files
    FTA_AlwaysUnsafe            = 0x00020000, // don't allow the "Never ask me" checkbox to be enabled; File Type dialog still allows user to turn this off

    FTA_NoRecentDocs            = 0x00100000, // don't add this file type to the Recent Documents folder
    FTA_SafeForElevation        = 0x00200000, // Win8: can be launched in medium IL by a process running in AppContainer
    FTA_AlwaysUseDirectInvoke   = 0x00400000, // Win8: when downloading use the direct invoke feature even if the server headers are not provided
} FILETYPEATTRIBUTEFLAGS;
DEFINE_ENUM_FLAG_OPERATORS(FILETYPEATTRIBUTEFLAGS)

#undef INTERFACE
#define INTERFACE IQueryAssociations

DECLARE_INTERFACE_IID_( IQueryAssociations, IUnknown, "c46ca590-3c3f-11d2-bee6-0000f805ca57" )
{
    // IQueryAssociations methods
    STDMETHOD (Init)(THIS_ _In_ ASSOCF flags, _In_opt_ LPCWSTR pszAssoc, _In_opt_ HKEY hkProgid, _In_opt_ HWND hwnd) PURE;
    STDMETHOD (GetString)(THIS_ _In_ ASSOCF flags, _In_ ASSOCSTR str, _In_opt_ LPCWSTR pszExtra, _Out_writes_opt_(*pcchOut) LPWSTR pszOut, _Inout_ DWORD *pcchOut) PURE;
    STDMETHOD (GetKey)(THIS_ _In_ ASSOCF flags, _In_ ASSOCKEY key, _In_opt_ LPCWSTR pszExtra, _Out_ HKEY *phkeyOut) PURE;
    STDMETHOD (GetData)(THIS_ _In_ ASSOCF flags, _In_ ASSOCDATA data, _In_opt_ LPCWSTR pszExtra, _Out_writes_bytes_opt_(*pcbOut) void * pvOut, _Inout_opt_ DWORD *pcbOut) PURE;
    STDMETHOD (GetEnum)(THIS_ _In_ ASSOCF flags, _In_ ASSOCENUM assocenum, _In_opt_ LPCWSTR pszExtra, _In_ REFIID riid, _Outptr_ void **ppvOut) PURE;
};


// use CLSID_QueryAssociations for clsid, object implements IQueryAssociations
// AssocCreateForClasses() is the more functional version of this API
LWSTDAPI AssocCreate(_In_ CLSID clsid, _In_ REFIID riid, _Outptr_ void **ppv);

// Retrieve an array of class keys from an IQueryAssociations object
// if the caller is just interested in the primary class key,
// call with cKeys == 1.  the return value is the number of keys
// inserted into the array.
STDAPI_(DWORD) SHGetAssocKeys(_In_ IQueryAssociations *pqa, _Out_writes_to_(cKeys, return) HKEY *rgKeys, DWORD cKeys);

//  wrappers for the interface
LWSTDAPI AssocQueryStringA(_In_ ASSOCF flags, _In_ ASSOCSTR str, _In_ LPCSTR pszAssoc, _In_opt_ LPCSTR pszExtra, _Out_writes_opt_(*pcchOut) LPSTR pszOut, _Inout_ DWORD *pcchOut);
//  wrappers for the interface
LWSTDAPI AssocQueryStringW(_In_ ASSOCF flags, _In_ ASSOCSTR str, _In_ LPCWSTR pszAssoc, _In_opt_ LPCWSTR pszExtra, _Out_writes_opt_(*pcchOut) LPWSTR pszOut, _Inout_ DWORD *pcchOut);
#ifdef UNICODE
#define AssocQueryString  AssocQueryStringW
#else
#define AssocQueryString  AssocQueryStringA
#endif // !UNICODE
LWSTDAPI AssocQueryStringByKeyA(_In_ ASSOCF flags, _In_ ASSOCSTR str, _In_ HKEY hkAssoc, _In_opt_ LPCSTR pszExtra, _Out_writes_opt_(*pcchOut) LPSTR pszOut, _Inout_ DWORD *pcchOut);
LWSTDAPI AssocQueryStringByKeyW(_In_ ASSOCF flags, _In_ ASSOCSTR str, _In_ HKEY hkAssoc, _In_opt_ LPCWSTR pszExtra, _Out_writes_opt_(*pcchOut) LPWSTR pszOut, _Inout_ DWORD *pcchOut);
#ifdef UNICODE
#define AssocQueryStringByKey  AssocQueryStringByKeyW
#else
#define AssocQueryStringByKey  AssocQueryStringByKeyA
#endif // !UNICODE
LWSTDAPI AssocQueryKeyA(_In_ ASSOCF flags, _In_ ASSOCKEY key, _In_ LPCSTR pszAssoc, _In_opt_ LPCSTR pszExtra, _Out_ HKEY *phkeyOut);
LWSTDAPI AssocQueryKeyW(_In_ ASSOCF flags, _In_ ASSOCKEY key, _In_ LPCWSTR pszAssoc, _In_opt_ LPCWSTR pszExtra, _Out_ HKEY *phkeyOut);
#ifdef UNICODE
#define AssocQueryKey  AssocQueryKeyW
#else
#define AssocQueryKey  AssocQueryKeyA
#endif // !UNICODE

#if (_WIN32_IE >= 0x0601)
//  AssocIsDangerous() checks a file type to determine whether it is "Dangerous"
//      this maps to the IE download dialog's forcing a prompt to open or save.
//      dangerous file types should be handled more carefully than other file types.
//
//  Parameter:  pszAssoc - type to check.  may be an extension or progid.  (".exe" or "exefile" would both be valid)
//
//  Returns: TRUE if the file type is dangerous.
//
//  NOTES:
//
//      this API first checks a hardcoded list of known dangerous types.
//      then it checks the editflags for the file type looking for the FTA_AlwaysUnsafe bit.
//      then it checks Safer policies.
//
LWSTDAPI_(BOOL) AssocIsDangerous(_In_ PCWSTR pszAssoc);

#endif  // _WIN32_IE >= 0x0601

#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
LWSTDAPI AssocGetPerceivedType(_In_ PCWSTR pszExt, _Out_ PERCEIVED *ptype, _Out_ PERCEIVEDFLAG *pflag, _Outptr_opt_ PWSTR *ppszType);
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif //  NO_SHLWAPI_REG



#ifndef NO_SHLWAPI_STREAM
//
//=============== Stream Routines ===================================
//

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// objidl.h
#ifndef __IStream_FWD_DEFINED__
#define __IStream_FWD_DEFINED__
typedef interface IStream IStream;
#endif  /* __IStream_FWD_DEFINED__ */

LWSTDAPI_(IStream *) SHOpenRegStreamA(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubkey, _In_opt_ LPCSTR pszValue, _In_ DWORD grfMode);
LWSTDAPI_(IStream *) SHOpenRegStreamW(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubkey, _In_opt_ LPCWSTR pszValue, _In_ DWORD grfMode);
#ifdef UNICODE
#define SHOpenRegStream  SHOpenRegStreamW
#else
#define SHOpenRegStream  SHOpenRegStreamA
#endif // !UNICODE
LWSTDAPI_(IStream *) SHOpenRegStream2A(_In_ HKEY hkey, _In_opt_ LPCSTR pszSubkey, _In_opt_ LPCSTR pszValue, _In_ DWORD grfMode);
LWSTDAPI_(IStream *) SHOpenRegStream2W(_In_ HKEY hkey, _In_opt_ LPCWSTR pszSubkey, _In_opt_ LPCWSTR pszValue, _In_ DWORD grfMode);
#ifdef UNICODE
#define SHOpenRegStream2  SHOpenRegStream2W
#else
#define SHOpenRegStream2  SHOpenRegStream2A
#endif // !UNICODE
// New code always wants new implementation...
#undef SHOpenRegStream
#define SHOpenRegStream SHOpenRegStream2

LWSTDAPI SHCreateStreamOnFileA(_In_ LPCSTR pszFile, _In_ DWORD grfMode, _Outptr_ IStream **ppstm);
LWSTDAPI SHCreateStreamOnFileW(_In_ LPCWSTR pszFile, _In_ DWORD grfMode, _Outptr_ IStream **ppstm);
#ifdef UNICODE
#define SHCreateStreamOnFile  SHCreateStreamOnFileW
#else
#define SHCreateStreamOnFile  SHCreateStreamOnFileA
#endif // !UNICODE


#if (_WIN32_IE >= 0x0600)

LWSTDAPI SHCreateStreamOnFileEx(_In_ LPCWSTR pszFile, _In_ DWORD grfMode, _In_ DWORD dwAttributes, _In_ BOOL fCreate, _In_opt_ IStream *pstmTemplate, _Outptr_ IStream **ppstm);

#endif // (_WIN32_IE >= 0x0600)

LWSTDAPI_(IStream *) SHCreateMemStream(_In_reads_bytes_opt_(cbInit) const BYTE *pInit, _In_ UINT cbInit);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


#endif // NO_SHLWAPI_STREAM


#ifndef NO_SHLWAPI_HTTP
//
//=============== HTTP helper Routines ===================================
//

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#if (_WIN32_IE >= 0x0603)
LWSTDAPI GetAcceptLanguagesA(_Out_writes_to_(*pcchLanguages, *pcchLanguages) LPSTR pszLanguages, _Inout_ DWORD *pcchLanguages);
LWSTDAPI GetAcceptLanguagesW(_Out_writes_to_(*pcchLanguages, *pcchLanguages) LPWSTR pszLanguages, _Inout_ DWORD *pcchLanguages);
#ifdef UNICODE
#define GetAcceptLanguages  GetAcceptLanguagesW
#else
#define GetAcceptLanguages  GetAcceptLanguagesA
#endif // !UNICODE
#endif // (_WIN32_IE >= 0x0603)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#endif // NO_SHLWAPI_HTTP

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if (_WIN32_IE < _WIN32_IE_IE70) && !defined(NO_SHLWAPI_STOPWATCH)

#define SPMODE_SHELL      0x00000001
#define SPMODE_DEBUGOUT   0x00000002
#define SPMODE_TEST       0x00000004
#define SPMODE_BROWSER    0x00000008
#define SPMODE_FLUSH      0x00000010
#define SPMODE_EVENT      0x00000020
#define SPMODE_MSVM       0x00000040
#define SPMODE_FORMATTEXT 0x00000080
#define SPMODE_PROFILE    0x00000100
#define SPMODE_DEBUGBREAK 0x00000200
#define SPMODE_MSGTRACE   0x00000400
#define SPMODE_PERFTAGS   0x00000800
#define SPMODE_MEMWATCH   0x00001000
#define SPMODE_DBMON      0x00002000
#define SPMODE_MULTISTOP  0x00004000
#ifndef NO_ETW_TRACING
#define SPMODE_EVENTTRACE 0x00008000 // Event Tracing for Windows Enabled
#endif

DWORD WINAPI StopWatchMode(void);
DWORD WINAPI StopWatchFlush(void);

#endif // (_WIN32_IE < _WIN32_IE_IE70) && !defined(NO_SHLWAPI_STOPWATCH)


    LWSTDAPI_(void) IUnknown_Set(_Inout_ IUnknown ** ppunk, _In_opt_ IUnknown * punk);
    LWSTDAPI_(void) IUnknown_AtomicRelease(_Inout_opt_ void ** ppunk);
    LWSTDAPI IUnknown_GetWindow(_In_ IUnknown* punk, _Out_ HWND* phwnd);
    LWSTDAPI IUnknown_SetSite(_In_ IUnknown *punk, _In_opt_ IUnknown *punkSite);
    LWSTDAPI IUnknown_GetSite(_In_ IUnknown *punk, _In_ REFIID riid, _Outptr_ void **ppv);
    LWSTDAPI IUnknown_QueryService(_In_opt_ IUnknown* punk, _In_ REFGUID guidService, _In_ REFIID riid, _COM_Outptr_ void ** ppvOut);


#if !defined(__cplusplus) && defined(COBJMACROS)
#undef IStream_Read
#undef IStream_Write
#endif

LWSTDAPI IStream_Read(_In_ IStream *pstm, _Out_writes_bytes_all_(cb) void *pv, _In_ ULONG cb);
LWSTDAPI IStream_Write(_In_ IStream *pstm, _In_reads_bytes_(cb) const void *pv, _In_ ULONG cb);
LWSTDAPI IStream_Reset(_In_ IStream *pstm);
LWSTDAPI IStream_Size(_In_ IStream *pstm, _Out_ ULARGE_INTEGER *pui);

// ocidl.h
#ifndef __IConnectionPoint_FWD_DEFINED__
#define __IConnectionPoint_FWD_DEFINED__
typedef interface IConnectionPoint IConnectionPoint;
#endif  /* __IConnectionPoint_FWD_DEFINED__ */

LWSTDAPI ConnectToConnectionPoint(_In_opt_ IUnknown* punk, _In_ REFIID riidEvent, BOOL fConnect, _In_ IUnknown* punkTarget, _Out_ DWORD* pdwCookie, _Outptr_opt_ IConnectionPoint** ppcpOut);

#if (NTDDI_VERSION >= NTDDI_VISTA)

LWSTDAPI IStream_ReadPidl(_In_ IStream *pstm, _Outptr_ PIDLIST_RELATIVE *ppidlOut);
LWSTDAPI IStream_WritePidl(_In_ IStream *pstm, _In_ PCUIDLIST_RELATIVE pidlWrite);

#endif // (NTDDI_VERSION >= NTDDI_VISTA)

#if (_WIN32_IE >= _WIN32_IE_IE70)

LWSTDAPI IStream_ReadStr(_In_ IStream *pstm, _Outptr_ PWSTR *ppsz);
LWSTDAPI IStream_WriteStr(_In_ IStream *pstm, _In_ PCWSTR psz);

LWSTDAPI IStream_Copy(_In_ IStream *pstmFrom, _In_ IStream *pstmTo, DWORD cb);

#endif // (_WIN32_IE >= _WIN32_IE_IE70)


#if (_WIN32_IE >= 0x0600)
#define SHGVSPB_PERUSER             0x00000001 // must have one of PERUSER or ALLUSERS
#define SHGVSPB_ALLUSERS            0x00000002
#define SHGVSPB_PERFOLDER           0x00000004 // must have one of PERFOLDER ALLFOLDERS or INHERIT
#define SHGVSPB_ALLFOLDERS          0x00000008
#define SHGVSPB_INHERIT             0x00000010
#define SHGVSPB_ROAM                0x00000020 // modifies the above
#define SHGVSPB_NOAUTODEFAULTS      0x80000000 // turns off read delegation to more general property bags

#define SHGVSPB_FOLDER              (SHGVSPB_PERUSER | SHGVSPB_PERFOLDER)
#define SHGVSPB_FOLDERNODEFAULTS    (SHGVSPB_PERUSER | SHGVSPB_PERFOLDER | SHGVSPB_NOAUTODEFAULTS)
#define SHGVSPB_USERDEFAULTS        (SHGVSPB_PERUSER | SHGVSPB_ALLFOLDERS)
#define SHGVSPB_GLOBALDEFAULTS      (SHGVSPB_ALLUSERS | SHGVSPB_ALLFOLDERS)

LWSTDAPI SHGetViewStatePropertyBag(_In_opt_ PCIDLIST_ABSOLUTE pidl, _In_opt_ PCWSTR pszBagName, DWORD dwFlags, _In_ REFIID riid, _Outptr_ void **ppv);
#endif  // (_WIN32_IE >= 0x0600)

// SHFormatDateTime flags
//  (FDTF_SHORTDATE and FDTF_LONGDATE are mutually exclusive, as is
//   FDTF_SHORTIME and FDTF_LONGTIME.)
//
#define FDTF_SHORTTIME          0x00000001      // eg, "7:48 PM"
#define FDTF_SHORTDATE          0x00000002      // eg, "3/29/98"
#define FDTF_DEFAULT            (FDTF_SHORTDATE | FDTF_SHORTTIME) // eg, "3/29/98 7:48 PM"
#define FDTF_LONGDATE           0x00000004      // eg, "Monday, March 29, 1998"
#define FDTF_LONGTIME           0x00000008      // eg. "7:48:33 PM"
#define FDTF_RELATIVE           0x00000010      // uses "Yesterday", etc. if possible
#define FDTF_LTRDATE            0x00000100      // Left To Right reading order
#define FDTF_RTLDATE            0x00000200      // Right To Left reading order
#define FDTF_NOAUTOREADINGORDER 0x00000400      // Don't detect reading order automatically. Useful if you will be converting to Ansi and don't want Unicode reading order characters

LWSTDAPI_(int)  SHFormatDateTimeA(_In_ const FILETIME UNALIGNED * pft, _Inout_opt_ DWORD * pdwFlags, _Out_writes_(cchBuf) LPSTR pszBuf, UINT cchBuf);
LWSTDAPI_(int)  SHFormatDateTimeW(_In_ const FILETIME UNALIGNED * pft, _Inout_opt_ DWORD * pdwFlags, _Out_writes_(cchBuf) LPWSTR pszBuf, UINT cchBuf);
#ifdef UNICODE
#define SHFormatDateTime  SHFormatDateTimeW
#else
#define SHFormatDateTime  SHFormatDateTimeA
#endif // !UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Success_(return > 0)
LWSTDAPI_(int)  SHAnsiToUnicode(_In_ PCSTR pszSrc, _Out_writes_(cwchBuf) PWSTR pwszDst, _In_ int cwchBuf);
_Success_(return > 0)
LWSTDAPI_(int)  SHAnsiToAnsi(_In_ PCSTR pszSrc, _Out_writes_(cchBuf) PSTR pszDst, _In_ int cchBuf);
_Success_(return > 0)
LWSTDAPI_(int)  SHUnicodeToAnsi(_In_ PCWSTR pwszSrc, _Out_writes_(cchBuf) PSTR pszDst, _In_ int cchBuf);
_Success_(return > 0)
LWSTDAPI_(int)  SHUnicodeToUnicode(_In_ PCWSTR pwzSrc, _Out_writes_(cwchBuf) PWSTR pwzDst, _In_ int cwchBuf);

// The return value from all SH<Type>To<Type> is the size of szDest including the terminater.
#ifdef UNICODE
#define SHTCharToUnicode(wzSrc, wzDest, cchSize)                SHUnicodeToUnicode(wzSrc, wzDest, cchSize)
#define SHTCharToAnsi(wzSrc, szDest, cchSize)                   SHUnicodeToAnsi(wzSrc, szDest, cchSize)
#define SHUnicodeToTChar(wzSrc, wzDest, cchSize)                SHUnicodeToUnicode(wzSrc, wzDest, cchSize)
#define SHAnsiToTChar(szSrc, wzDest, cchSize)                   SHAnsiToUnicode(szSrc, wzDest, cchSize)
#else // UNICODE
#define SHTCharToUnicode(szSrc, wzDest, cchSize)                SHAnsiToUnicode(szSrc, wzDest, cchSize)
#define SHTCharToAnsi(szSrc, szDest, cchSize)                   SHAnsiToAnsi(szSrc, szDest, cchSize)
#define SHUnicodeToTChar(wzSrc, szDest, cchSize)                SHUnicodeToAnsi(wzSrc, szDest, cchSize)
#define SHAnsiToTChar(szSrc, szDest, cchSize)                   SHAnsiToAnsi(szSrc, szDest, cchSize)
#endif // UNICODE

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


LWSTDAPI_(int) SHMessageBoxCheckA(_In_opt_ HWND hwnd, _In_ LPCSTR pszText, _In_ LPCSTR pszCaption, UINT uType, int iDefault, _In_ LPCSTR pszRegVal);
LWSTDAPI_(int) SHMessageBoxCheckW(_In_opt_ HWND hwnd, _In_ LPCWSTR pszText, _In_ LPCWSTR pszCaption, UINT uType, int iDefault, _In_ LPCWSTR pszRegVal);
#ifdef UNICODE
#define SHMessageBoxCheck  SHMessageBoxCheckW
#else
#define SHMessageBoxCheck  SHMessageBoxCheckA
#endif // !UNICODE


// Prevents hang do to hung window on broadcast
LWSTDAPI_(LRESULT) SHSendMessageBroadcastA(_In_ UINT uMsg, _In_ WPARAM wParam, _In_ LPARAM lParam);
// Prevents hang do to hung window on broadcast
LWSTDAPI_(LRESULT) SHSendMessageBroadcastW(_In_ UINT uMsg, _In_ WPARAM wParam, _In_ LPARAM lParam);
#ifdef UNICODE
#define SHSendMessageBroadcast  SHSendMessageBroadcastW
#else
#define SHSendMessageBroadcast  SHSendMessageBroadcastA
#endif // !UNICODE

LWSTDAPI_(CHAR) SHStripMneumonicA(_Inout_ LPSTR pszMenu);
LWSTDAPI_(WCHAR) SHStripMneumonicW(_Inout_ LPWSTR pszMenu);
#ifdef UNICODE
#define SHStripMneumonic  SHStripMneumonicW
#else
#define SHStripMneumonic  SHStripMneumonicA
#endif // !UNICODE

#ifndef NO_SHLWAPI_ISOS

// Returns TRUE/FALSE depending on question
#define OS_WINDOWS                  0           // Windows 9x vs. NT
#define OS_NT                       1           // Windows 9x vs. NT
#define OS_WIN95ORGREATER           2           // Win95 or greater
#define OS_NT4ORGREATER             3           // NT4 or greater
#define OS_WIN98ORGREATER           5           // Win98 or greater
#define OS_WIN98_GOLD               6           // Win98 Gold (Version 4.10 build 1998)
#define OS_WIN2000ORGREATER         7           // Some derivative of Win2000

// NOTE: these flags check explicitly for (dwMajorVersion == 5)
#define OS_WIN2000PRO               8           // Windows 2000 Professional (Workstation)
#define OS_WIN2000SERVER            9           // Windows 2000 Server
#define OS_WIN2000ADVSERVER         10          // Windows 2000 Advanced Server
#define OS_WIN2000DATACENTER        11          // Windows 2000 Data Center Server
#define OS_WIN2000TERMINAL          12          // Windows 2000 Terminal Server in "Application Server" mode (now simply called "Terminal Server")

#define OS_EMBEDDED                 13          // Embedded Windows Edition
#define OS_TERMINALCLIENT           14          // Windows Terminal Client (eg user is comming in via tsclient)
#define OS_TERMINALREMOTEADMIN      15          // Terminal Server in "Remote Administration" mode
#define OS_WIN95_GOLD               16          // Windows 95 Gold (Version 4.0 Build 1995)
#define OS_MEORGREATER              17          // Windows Millennium (Version 5.0)
#define OS_XPORGREATER              18          // Windows XP or greater
#define OS_HOME                     19          // Home Edition (eg NOT Professional, Server, Advanced Server, or Datacenter)
#define OS_PROFESSIONAL             20          // Professional     (aka Workstation; eg NOT Server, Advanced Server, or Datacenter)
#define OS_DATACENTER               21          // Datacenter       (eg NOT Server, Advanced Server, Professional, or Personal)
#define OS_ADVSERVER                22          // Advanced Server  (eg NOT Datacenter, Server, Professional, or Personal)
#define OS_SERVER                   23          // Server           (eg NOT Datacenter, Advanced Server, Professional, or Personal)
#define OS_TERMINALSERVER           24          // Terminal Server - server running in what used to be called "Application Server" mode (now simply called "Terminal Server")
#define OS_PERSONALTERMINALSERVER   25          // Personal Terminal Server - per/pro machine running in single user TS mode
#define OS_FASTUSERSWITCHING        26          // Fast User Switching
#define OS_WELCOMELOGONUI           27          // New friendly logon UI
#define OS_DOMAINMEMBER             28          // Is this machine a member of a domain (eg NOT a workgroup)
#define OS_ANYSERVER                29          // is this machine any type of server? (eg datacenter or advanced server or server)?
#define OS_WOW6432                  30          // Is this process a 32-bit process running on an 64-bit platform?
#define OS_WEBSERVER                31          // Web Edition Server
#define OS_SMALLBUSINESSSERVER      32          // SBS Server
#define OS_TABLETPC                 33          // Are we running on a TabletPC?
#define OS_SERVERADMINUI            34          // Should defaults lean towards those preferred by server administrators?
#define OS_MEDIACENTER              35          // eHome Freestyle Project
#define OS_APPLIANCE                36          // Windows .NET Appliance Server

LWSTDAPI_(BOOL) IsOS(DWORD dwOS);

#endif // NO_SHLWAPI_ISOS


typedef enum
{
    GLOBALCOUNTER_SEARCHMANAGER,
    GLOBALCOUNTER_SEARCHOPTIONS,
    GLOBALCOUNTER_FOLDERSETTINGSCHANGE,
    GLOBALCOUNTER_RATINGS,
    GLOBALCOUNTER_APPROVEDSITES,
    GLOBALCOUNTER_RESTRICTIONS,
    GLOBALCOUNTER_SHELLSETTINGSCHANGED,
    GLOBALCOUNTER_SYSTEMPIDLCHANGE,
    GLOBALCOUNTER_OVERLAYMANAGER,
    GLOBALCOUNTER_QUERYASSOCIATIONS,
    GLOBALCOUNTER_IESESSIONS,
    GLOBALCOUNTER_IEONLY_SESSIONS,
    GLOBALCOUNTER_APPLICATION_DESTINATIONS,
    __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_CSCSYNCINPROGRESS,
    GLOBALCOUNTER_BITBUCKETNUMDELETERS,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SHARES,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_A,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_B,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_C,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_D,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_E,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_F,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_G,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_H,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_I,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_J,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_K,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_L,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_M,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_N,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_O,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_P,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Q,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_R,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_S,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_T,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_U,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_V,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_W,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_X,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Y,
    GLOBALCOUNTER_RECYCLEDIRTYCOUNT_DRIVE_Z,
    __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEDIRTYCOUNT_SERVERDRIVE,
    __UNUSED_RECYCLE_WAS_GLOBALCOUNTER_RECYCLEGLOBALDIRTYCOUNT,
    GLOBALCOUNTER_RECYCLEBINENUM,
    GLOBALCOUNTER_RECYCLEBINCORRUPTED,
    GLOBALCOUNTER_RATINGS_STATECOUNTER,
    GLOBALCOUNTER_PRIVATE_PROFILE_CACHE,
    GLOBALCOUNTER_INTERNETTOOLBAR_LAYOUT,
    GLOBALCOUNTER_FOLDERDEFINITION_CACHE,
    GLOBALCOUNTER_COMMONPLACES_LIST_CACHE,
    GLOBALCOUNTER_PRIVATE_PROFILE_CACHE_MACHINEWIDE,
    GLOBALCOUNTER_ASSOCCHANGED,  // throttles reading of the registry value "GlobalAssocChangedCounter" from HKLM\Software\Microsoft\Windows\CurrentVersion\Explorer
#if (NTDDI_VERSION >= NTDDI_WIN8)
    GLOBALCOUNTER_APP_ITEMS_STATE_STORE_CACHE,
    GLOBALCOUNTER_SETTINGSYNC_ENABLED,
    GLOBALCOUNTER_APPSFOLDER_FILETYPEASSOCIATION_COUNTER,
    GLOBALCOUNTER_USERINFOCHANGED,
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
    GLOBALCOUNTER_SYNC_ENGINE_INFORMATION_CACHE_MACHINEWIDE,
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
#endif // (NTDDI_VERSION >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
    GLOBALCOUNTER_BANNERS_DATAMODEL_CACHE_MACHINEWIDE,
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)
    GLOBALCOUNTER_MAXIMUMVALUE // should always be last value
} SHGLOBALCOUNTER;

#if (NTDDI_VERSION >= NTDDI_WIN7)
LWSTDAPI_(long) SHGlobalCounterGetValue(const SHGLOBALCOUNTER id);
LWSTDAPI_(long) SHGlobalCounterIncrement(const SHGLOBALCOUNTER id);
LWSTDAPI_(long) SHGlobalCounterDecrement(const SHGLOBALCOUNTER id);
#endif // (NTDDI_VERSION >= NTDDI_WIN7)


// Shared memory apis


#if (_WIN32_IE >= 0x0603)
LWSTDAPI_(HANDLE)   SHAllocShared(_In_reads_bytes_opt_(dwSize) const void *pvData, _In_ DWORD dwSize, _In_ DWORD dwProcessId);
LWSTDAPI_(BOOL)     SHFreeShared(_In_ HANDLE hData, _In_ DWORD dwProcessId);
LWSTDAPI_(void *)   SHLockShared(_In_ HANDLE hData, _In_ DWORD dwProcessId);
LWSTDAPI_(BOOL)     SHUnlockShared(_In_reads_(_Inexpressible_("header-preceded")) void *pvData);
#endif // _WIN32_IE >= 0x0603


LWSTDAPI_(UINT) WhichPlatform(void);

// Return values of WhichPlatform
#define PLATFORM_UNKNOWN     0
#define PLATFORM_IE3         1      // obsolete: use PLATFORM_BROWSERONLY
#define PLATFORM_BROWSERONLY 1      // browser-only (no new shell)
#define PLATFORM_INTEGRATED  2      // integrated shell

//***   QueryInterface helpers
// NOTES
//  ATL has a fancier version of this.  if we need to extend ours, we
//  should probably just switch to ATL's rather than reinvent.
// EXAMPLE
//  Cfoo::QI(REFIID riid, void **ppv)
//  {
//      // (the IID_xxx comments make grep'ing work!)
//      static const QITAB qit = {
//          QITABENT(Cfoo, Iiface1),    // IID_Iiface1
//          ...
//          QITABENT(Cfoo, IifaceN),    // IID_IifaceN
//          { 0 },                      // n.b. don't forget the 0
//      };
//
//      // n.b. make sure you don't cast 'this'
//      hr = QISearch(this, qit, riid, ppv);
//      if (FAILED(hr))
//          hr = SUPER::QI(riid, ppv);
//      // custom code could be added here for FAILED() case
//      return hr;
//  }
//
//  If you have an interface that is implemented multiple times,
//  the compiler will claim an ambiguous cast, so you need to give
//  some help.  For example, if your class is defined as
//
//  class CClass : public IPersistStream, public IPersistFile
//
//  then you need to say
//
//      QITABENTMULTI(CClass, IPersist, IPersistStream)
//      QITABENT(CClass, IPersistStream)
//      QITABENT(CClass, IPersistFile)
//
//  The MULTI is needed for IPersist because the compiler doesn't
//  know whether you wanted the IPersist from IPersistStream
//  or the IPersist from IPersistFile.  Unless you have played weird
//  compiler tricks, they are the same implementation, so you can
//  just pick either one, doesn't matter.
//
//  Common mistake: You do not need to use MULTI if the base interface
//  can be unambiguously determined.  E.g.,
//
//  class CSimple : public IContextMenu3
//
//      QITABENT(CClass, IContextMenu)      // do not need MULTI
//      QITABENT(CClass, IContextMenu2)     // do not need MULTI
//      QITABENT(CClass, IContextMenu3)
//
//  Do not create an entry for IUnknown; the first entry in the
//  table will be used for IUnknown.
//

typedef struct
{
    const IID * piid;
    DWORD         dwOffset;
} QITAB, *LPQITAB;
typedef const QITAB *LPCQITAB;

#ifdef __cplusplus

#define QITABENTMULTI(Cthis, Ifoo, Iimpl) \
    { &__uuidof(Ifoo), OFFSETOFCLASS(Iimpl, Cthis) }

#else

#define QITABENTMULTI(Cthis, Ifoo, Iimpl) \
    { (IID*) &IID_##Ifoo, OFFSETOFCLASS(Iimpl, Cthis) }

#endif  // __cplusplus

#define QITABENTMULTI2(Cthis, Ifoo, Iimpl) \
    { (IID*) &Ifoo, OFFSETOFCLASS(Iimpl, Cthis) }

#define QITABENT(Cthis, Ifoo) QITABENTMULTI(Cthis, Ifoo, Ifoo)

STDAPI QISearch(_Inout_ void* that, _In_ LPCQITAB pqit, _In_ REFIID riid, _COM_Outptr_ void **ppv);

#ifndef STATIC_CAST
//***   STATIC_CAST -- 'portable' static_cast<>
// NOTES
//  do *not* use SAFE_CAST (see comment in OFFSETOFCLASS)
#define STATIC_CAST(typ)   static_cast<typ>
#ifndef _X86_
    // assume only intel compiler (>=vc5) supports static_cast for now
    // we could key off of _MSC_VER >= 1100 but i'm not sure that will work
    //
    // a straight cast will give the correct result but no error checking,
    // so we'll have to catch errors on intel.
    #undef  STATIC_CAST
    #define STATIC_CAST(typ)   (typ)
#endif
#endif

#ifndef OFFSETOFCLASS
//***   OFFSETOFCLASS -- (stolen from ATL)
// we use STATIC_CAST not SAFE_CAST because the compiler gets confused
// (it doesn't constant-fold the ,-op in SAFE_CAST so we end up generating
// code for the table!)

#define OFFSETOFCLASS(base, derived) \
    ((DWORD)(DWORD_PTR)(STATIC_CAST(base*)((derived*)8))-8)
#endif


// Types for SHIsLowMemoryMachine
#define ILMM_IE4    0       // 1997-era machine
LWSTDAPI_(BOOL) SHIsLowMemoryMachine(DWORD dwType);

// Menu Helpers
LWSTDAPI_(int)  GetMenuPosFromID(_In_ HMENU hmenu, UINT id);

LWSTDAPI        SHGetInverseCMAP(_Out_writes_bytes_(cbMap) BYTE *pbMap, ULONG cbMap);


// SHAutoComplete
//      hwndEdit - HWND of editbox, ComboBox or ComboBoxEx.
//      dwFlags - Flags to indicate what to AutoAppend or AutoSuggest for the editbox.
//
// WARNING:
//    Caller needs to have called CoInitialize() or OleInitialize()
//    and cannot call CoUninit/OleUninit until after
//    WM_DESTROY on hwndEdit.
//
//  dwFlags values:
#define SHACF_DEFAULT                   0x00000000  // Currently (SHACF_FILESYSTEM | SHACF_URLALL)
#define SHACF_FILESYSTEM                0x00000001  // This includes the File System as well as the rest of the shell (Desktop\My Computer\Control Panel\)
#define SHACF_URLALL                    (SHACF_URLHISTORY | SHACF_URLMRU)
#define SHACF_URLHISTORY                0x00000002  // URLs in the User's History
#define SHACF_URLMRU                    0x00000004  // URLs in the User's Recently Used list.
#define SHACF_USETAB                    0x00000008  // Use the tab to move thru the autocomplete possibilities instead of to the next dialog/window control.
#define SHACF_FILESYS_ONLY              0x00000010  // This includes the File System
#if (_WIN32_IE >= 0x0600)
#define SHACF_FILESYS_DIRS              0x00000020  // Same as SHACF_FILESYS_ONLY except it only includes directories, UNC servers, and UNC server shares.
#endif // (_WIN32_IE >= 0x0600)
#if (_WIN32_IE >= _WIN32_IE_IE70)
#define SHACF_VIRTUAL_NAMESPACE         0x00000040  // Also include the virtual namespace
#endif // _WIN32_IE_IE70
#define SHACF_AUTOSUGGEST_FORCE_ON      0x10000000  // Ignore the registry default and force the feature on.
#define SHACF_AUTOSUGGEST_FORCE_OFF     0x20000000  // Ignore the registry default and force the feature off.
#define SHACF_AUTOAPPEND_FORCE_ON       0x40000000  // Ignore the registry default and force the feature on. (Also know as AutoComplete)
#define SHACF_AUTOAPPEND_FORCE_OFF      0x80000000  // Ignore the registry default and force the feature off. (Also know as AutoComplete)

LWSTDAPI SHAutoComplete(_In_ HWND hwndEdit, DWORD dwFlags);

#if (_WIN32_IE >= _WIN32_IE_IE60SP2)
LWSTDAPI SHCreateThreadRef(_Inout_ LONG *pcRef, _Outptr_ IUnknown **ppunk);
#endif // _WIN32_IE_IE60SP2
LWSTDAPI SHSetThreadRef(_In_opt_ IUnknown *punk);
LWSTDAPI SHGetThreadRef(_COM_Outptr_ IUnknown **ppunk);

LWSTDAPI_(BOOL) SHSkipJunction(_In_opt_ IBindCtx* pbc, _In_ const CLSID *pclsid);

enum
{
    CTF_INSIST              = 0x00000001,   // call pfnThreadProc synchronously if CreateThread() fails
    CTF_THREAD_REF          = 0x00000002,   // hold a reference to the creating thread
    CTF_PROCESS_REF         = 0x00000004,   // hold a reference to the creating process
    CTF_COINIT_STA          = 0x00000008,   // init COM as STA for the created thread
    CTF_COINIT              = 0x00000008,   // init COM as STA for the created thread
#if (_WIN32_IE >= _WIN32_IE_IE60)
    CTF_FREELIBANDEXIT      = 0x00000010,   // hold a ref to the DLL and call FreeLibraryAndExitThread() when done
    CTF_REF_COUNTED         = 0x00000020,   // thread supports ref counting via SHGetThreadRef() or CTF_THREAD_REF so that child threads can keep this thread alive
    CTF_WAIT_ALLOWCOM       = 0x00000040,   // while waiting for pfnCallback, allow COM marshaling to the blocked calling thread
#endif // _WIN32_IE_IE60
#if (_WIN32_IE >= _WIN32_IE_IE70)
    CTF_UNUSED              = 0x00000080,
    CTF_INHERITWOW64        = 0x00000100,   // new thread should inherit the wow64 disable state for the file system redirector
#endif // _WIN32_IE_IE70
#if (NTDDI_VERSION >= NTDDI_VISTA)
    CTF_WAIT_NO_REENTRANCY  = 0x00000200,   // don't allow re-entrancy when waiting for the sync proc, this won't work with marshalled objects or SendMessages() from the sync proc
#endif // (NTDDI_VERSION >= NTDDI_VISTA)
#if (NTDDI_VERSION >= NTDDI_WIN7)
    CTF_KEYBOARD_LOCALE     = 0x00000400,   // carry the keyboard locale from creating to created thread
    CTF_OLEINITIALIZE       = 0x00000800,   // init OLE on the created thread (this will also init COM as STA)
    CTF_COINIT_MTA          = 0x00001000,   // init COM as MTA for the created thread
    CTF_NOADDREFLIB         = 0x00002000,   // this flag is the opposite of CTF_FREELIBANDEXIT that is now implicit as of Vista
                                            // this avoids the LoadLibrary/FreeLibraryAndExitThread calls that result in contention for the loader lock
                                            // only use this when the thread being created has some other means to ensure that the code
                                            // of the thread proc will remain loaded. This should not be used in the context of COM objects as those
                                            // need to ensure that the DLL stays loaded as COM will unload DLLs
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
};


typedef DWORD SHCT_FLAGS;   // SHCreateThread flags values

LWSTDAPI_(BOOL) SHCreateThread(_In_ LPTHREAD_START_ROUTINE pfnThreadProc, _In_opt_ void *pData, _In_ SHCT_FLAGS flags, _In_opt_ LPTHREAD_START_ROUTINE pfnCallback);
LWSTDAPI_(BOOL) SHCreateThreadWithHandle(_In_ LPTHREAD_START_ROUTINE pfnThreadProc, _In_opt_ void *pData, _In_ SHCT_FLAGS flags, _In_opt_ LPTHREAD_START_ROUTINE pfnCallback, _Out_opt_ HANDLE *pHandle);

#if (NTDDI_VERSION >= NTDDI_WIN8)
LWSTDAPI_(void) SetProcessReference(_In_opt_ IUnknown *punk);
LWSTDAPI GetProcessReference(_COM_Outptr_ IUnknown **punk);
#endif // (NTDDI_VERSION >= NTDDI_WIN8)

#if (_WIN32_IE >= _WIN32_IE_IE60)
LWSTDAPI SHReleaseThreadRef(void); // release a CTF_THREAD_REF reference earlier than the return of pfnThreadProc
#endif // _WIN32_IE_IE60

#ifndef NO_SHLWAPI_GDI
//
//====== GDI helper functions  ================================================
//

LWSTDAPI_(HPALETTE) SHCreateShellPalette(_In_opt_ HDC hdc);

LWSTDAPI_(void)     ColorRGBToHLS(COLORREF clrRGB, _Out_ WORD* pwHue, _Out_ WORD* pwLuminance, _Out_ WORD* pwSaturation);
LWSTDAPI_(COLORREF) ColorHLSToRGB(WORD wHue, WORD wLuminance, WORD wSaturation);
LWSTDAPI_(COLORREF) ColorAdjustLuma(COLORREF clrRGB, int n, BOOL fScale);


#endif // NO_SHLWAPI_GDI

//
//====== DllGetVersion  =======================================================
//

typedef struct _DLLVERSIONINFO
{
    DWORD cbSize;
    DWORD dwMajorVersion;                   // Major version
    DWORD dwMinorVersion;                   // Minor version
    DWORD dwBuildNumber;                    // Build number
    DWORD dwPlatformID;                     // DLLVER_PLATFORM_*
} DLLVERSIONINFO;

// Platform IDs for DLLVERSIONINFO
#define DLLVER_PLATFORM_WINDOWS         0x00000001      // Windows 95
#define DLLVER_PLATFORM_NT              0x00000002      // Windows NT

typedef struct _DLLVERSIONINFO2
{
    DLLVERSIONINFO info1;
    DWORD dwFlags;                          // No flags currently defined
    ULONGLONG ullVersion;                   // Encoded as:
                                            // Major 0xFFFF 0000 0000 0000
                                            // Minor 0x0000 FFFF 0000 0000
                                            // Build 0x0000 0000 FFFF 0000
                                            // QFE   0x0000 0000 0000 FFFF
} DLLVERSIONINFO2;

#define DLLVER_MAJOR_MASK                    0xFFFF000000000000
#define DLLVER_MINOR_MASK                    0x0000FFFF00000000
#define DLLVER_BUILD_MASK                    0x00000000FFFF0000
#define DLLVER_QFE_MASK                      0x000000000000FFFF

#define MAKEDLLVERULL(major, minor, build, qfe) \
        (((ULONGLONG)(major) << 48) |        \
         ((ULONGLONG)(minor) << 32) |        \
         ((ULONGLONG)(build) << 16) |        \
         ((ULONGLONG)(  qfe) <<  0))

//
// The caller should always GetProcAddress("DllGetVersion"), not
// implicitly link to it.
//

typedef HRESULT (CALLBACK* DLLGETVERSIONPROC)(DLLVERSIONINFO *);

// DllInstall (to be implemented by self-installing DLLs)
STDAPI DllInstall(BOOL bInstall, _In_opt_ PCWSTR pszCmdLine);


#if (_WIN32_IE >= 0x0602)
// Function to see if Internet Explorer Enhanced Security Configuration is active for the current user
LWSTDAPI_(BOOL) IsInternetESCEnabled(void);
#endif // (_WIN32_IE >= 0x0602)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#ifdef __cplusplus
} /* Close extern "C" { */
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#ifdef __cplusplus


#if defined(USE_STRICT_CONST) && !defined(NO_SHLWAPI_STRFCNS)

//=============================================================================
// C++ versions to help detect "const" violations

__inline PSTR StrChrA(_In_ PSTR pszStart, WORD wMatch)
{
    return const_cast<PSTR>(StrChrA(const_cast<PCSTR>(pszStart), wMatch));
}
__inline PWSTR StrChrW(_In_ PWSTR pszStart, WCHAR wMatch)
{
    return const_cast<PWSTR>(StrChrW(const_cast<PCWSTR>(pszStart), wMatch));
}
__inline PSTR StrChrIA(_In_ PSTR pszStart, WORD wMatch)
{
    return const_cast<PSTR>(StrChrIA(const_cast<PCSTR>(pszStart), wMatch));
}
__inline PWSTR StrChrIW(_In_ PWSTR pszStart, WCHAR wMatch)
{
    return const_cast<PWSTR>(StrChrIW(const_cast<PCWSTR>(pszStart), wMatch));
}
#if (_WIN32_IE >= _WIN32_IE_IE60)
__inline PWSTR StrChrNW(_In_ PWSTR pszStart, WCHAR wMatch, UINT cchMax)
{
    return const_cast<PWSTR>(StrChrNW(const_cast<PCWSTR>(pszStart), wMatch, cchMax));
}
__inline PWSTR StrChrNIW(_In_ PWSTR pszStart, WCHAR wMatch, UINT cchMax)
{
    return const_cast<PWSTR>(StrChrNIW(const_cast<PCWSTR>(pszStart), wMatch, cchMax));
}
#endif // _WIN32_IE_IE60
__inline PSTR StrPBrkA(_In_ PSTR psz, _In_ PCSTR pszSet)
{
    return const_cast<PSTR>(StrPBrkA(const_cast<PCSTR>(psz), pszSet));
}
__inline PWSTR StrPBrkW(_In_ PWSTR psz, _In_ PCWSTR pszSet)
{
    return const_cast<PWSTR>(StrPBrkW(const_cast<PCWSTR>(psz), pszSet));
}
__inline PSTR StrRChrA(_In_ PSTR pszStart, _In_opt_ PCSTR pszEnd, WORD wMatch)
{
    return const_cast<PSTR>(StrRChrA(const_cast<PCSTR>(pszStart), pszEnd, wMatch));
}
__inline PWSTR StrRChrW(_In_ PWSTR pszStart, _In_opt_ PCWSTR pszEnd, WCHAR wMatch)
{
    return const_cast<PWSTR>(StrRChrW(const_cast<PCWSTR>(pszStart), pszEnd, wMatch));
}
__inline PSTR StrRChrIA(_In_ PSTR pszStart, _In_opt_ PCSTR pszEnd, WORD wMatch)
{
    return const_cast<PSTR>(StrRChrIA(const_cast<PCSTR>(pszStart), pszEnd, wMatch));
}
__inline PWSTR StrRChrIW(_In_ PWSTR pszStart, _In_opt_ PCWSTR pszEnd, WCHAR wMatch)
{
    return const_cast<PWSTR>(StrRChrIW(const_cast<PCWSTR>(pszStart), pszEnd, wMatch));
}
__inline PSTR StrRStrIA(_In_ PSTR pszSource, _In_opt_ PCSTR pszLast, _In_ PCSTR pszSrch)
{
    return const_cast<PSTR>(StrRStrIA(const_cast<PCSTR>(pszSource), pszLast, pszSrch));
}
__inline PWSTR StrRStrIW(_In_ PWSTR pszSource, _In_opt_ PCWSTR pszLast, _In_ PCWSTR pszSrch)
{
    return const_cast<PWSTR>(StrRStrIW(const_cast<PCWSTR>(pszSource), pszLast, pszSrch));
}
__inline PSTR StrStrA(_In_ PSTR pszFirst, _In_ PCSTR pszSrch)
{
    return const_cast<PSTR>(StrStrA(const_cast<PCSTR>(pszFirst), pszSrch));
}
__inline PWSTR StrStrW(_In_ PWSTR pszFirst, _In_ PCWSTR pszSrch)
{
    return const_cast<PWSTR>(StrStrW(const_cast<PCWSTR>(pszFirst), pszSrch));
}
__inline PSTR StrStrIA(_In_ PSTR pszFirst, _In_ PCSTR pszSrch)
{
    return const_cast<PSTR>(StrStrIA(const_cast<PCSTR>(pszFirst), pszSrch));
}
__inline PWSTR StrStrIW(_In_ PWSTR pszFirst, _In_ PCWSTR pszSrch)
{
    return const_cast<PWSTR>(StrStrIW(const_cast<PCWSTR>(pszFirst), pszSrch));
}
#if (_WIN32_IE >= _WIN32_IE_IE60)
__inline PWSTR StrStrNW(_In_ PWSTR pszFirst, _In_ PCWSTR pszSrch, UINT cchMax)
{
    return const_cast<PWSTR>(StrStrNW(const_cast<PCWSTR>(pszFirst), pszSrch, cchMax));
}
__inline PWSTR StrStrNIW(_In_ PWSTR pszFirst, _In_ PCWSTR pszSrch, UINT cchMax)
{
    return const_cast<PWSTR>(StrStrNIW(const_cast<PCWSTR>(pszFirst), pszSrch, cchMax));
}
#endif
__inline PSTR PathFindExtensionA(_In_ PSTR pszPath)
{
    return const_cast<PSTR>(PathFindExtensionA(const_cast<PCSTR>(pszPath)));
}
__inline PWSTR PathFindExtensionW(_In_ PWSTR pszPath)
{
    return const_cast<PWSTR>(PathFindExtensionW(const_cast<PCWSTR>(pszPath)));
}
__inline PSTR PathFindFileNameA(_In_ PSTR pszPath)
{
    return const_cast<PSTR>(PathFindFileNameA(const_cast<PCSTR>(pszPath)));
}
__inline PWSTR PathFindFileNameW(_In_ PWSTR pszPath)
{
    return const_cast<PWSTR>(PathFindFileNameW(const_cast<PCWSTR>(pszPath)));
}
__inline PSTR PathFindNextComponentA(_In_ PSTR pszPath)
{
    return const_cast<PSTR>(PathFindNextComponentA(const_cast<PCSTR>(pszPath)));
}
__inline PWSTR PathFindNextComponentW(_In_ PWSTR pszPath)
{
    return const_cast<PWSTR>(PathFindNextComponentW(const_cast<PCWSTR>(pszPath)));
}
__inline PSTR PathGetArgsA(_In_ PSTR pszPath)
{
    return const_cast<PSTR>(PathGetArgsA(const_cast<PCSTR>(pszPath)));
}
__inline PWSTR PathGetArgsW(_In_ PWSTR pszPath)
{
    return const_cast<PWSTR>(PathGetArgsW(const_cast<PCWSTR>(pszPath)));
}
__inline PSTR PathSkipRootA(_In_ PSTR pszPath)
{
    return const_cast<PSTR>(PathSkipRootA(const_cast<PCSTR>(pszPath)));
}
__inline PWSTR PathSkipRootW(_In_ PWSTR pszPath)
{
    return const_cast<PWSTR>(PathSkipRootW(const_cast<PCWSTR>(pszPath)));
}
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)


#endif

#if defined(DEPRECATE_SUPPORTED)
#pragma warning(pop)
#endif

#ifdef _WIN32
#include <poppack.h>
#endif

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion


// NOSHLWAPI
#endif



#endif  // _INC_SHLWAPI

