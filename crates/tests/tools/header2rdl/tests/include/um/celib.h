//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//
//  Copyright (C) Microsoft Corporation, 1995 - 1999
//
//  File:       celib.h
//
//  Contents:   helper functions
//
//--------------------------------------------------------------------------

#ifndef __CELIB_H__
#define __CELIB_H__
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#include <stdio.h>


#define MAX_DECODE_BUFFER_SIZE     (1024 * 1000 * 100)       // allow 100MB max
#define CENCODEMAX	(64 * 1024)

// Bitmap manipulation routines.  Fetch or set a bit, given a base and index.
#define GETBIT(pb, i)	((pb)[(i) / 8] & (1 << ((i) % 8)))
#define SETBIT(pb, i)	((pb)[(i) / 8] |= (1 << ((i) % 8)))
#define CLEARBIT(pb, i)	((pb)[(i) / 8] &= ~(1 << ((i) % 8)))

#define BITSTOBYTES(b)	((int)(((b) + 7) / 8))

#define ceCASIGN_KEY_USAGE \
	    (CERT_KEY_CERT_SIGN_KEY_USAGE | CERT_DIGITAL_SIGNATURE_KEY_USAGE | \
	     CERT_CRL_SIGN_KEY_USAGE)

// Size of a fixed array: Use ARRAYSIZE(a)

// wcslen of a static string:
#define WSZARRAYSIZE(a)		((sizeof(a)/sizeof((a)[0])) - 1)

#define wszCERTENROLLSHAREPATH	L"CertSrv\\CertEnroll"

#define cwcHRESULTSTRING	40
#define cwcDWORDSPRINTF		(1 + 10 + 1)	// DWORD "%d" w/sign & '\0'

#define SAFE_SUBTRACT_POINTERS(p1, p2) \
    (assert(sizeof((*p1)) == sizeof(*(p2))), (DWORD)((p1) - (p2)))

#define printf	Use_wprintf_Instead_Of_printf

#define _LeaveError(hr, label, pszMessage) \
	_LeaveErrorStr2((hr), label, (pszMessage), NULL, S_OK)

#define _LeaveError2(hr, label, pszMessage, hr2) \
	_LeaveErrorStr2((hr), label, (pszMessage), NULL, (hr2))

#define _LeaveErrorStr(hr, label, pszMessage, pwszData) \
	_LeaveErrorStr2((hr), label, (pszMessage), (pwszData), S_OK)

#define _LeaveErrorStr2(hr, label, pszMessage, pwszData, hr2) \
    { \
	ceERRORPRINTLINESTR((pszMessage), (pwszData), (hr)); \
	goto label; \
    }

#define _LeaveIfError(hr, label, pszMessage) \
	_LeaveIfErrorStr2((hr), label, (pszMessage), NULL, S_OK)

#define _LeaveIfError2(hr, label, pszMessage, hr2) \
	_LeaveIfErrorStr2((hr), label, (pszMessage), NULL, (hr2))

#define _LeaveIfErrorStr(hr, label, pszMessage, pwszData) \
	_LeaveIfErrorStr2((hr), label, (pszMessage), (pwszData), S_OK)

#define _LeaveIfErrorStr2(hr, label, pszMessage, pwszData, hr2) \
    { \
	if (S_OK != (hr)) \
	{ \
	    ceERRORPRINTLINESTR((pszMessage), (pwszData), (hr)); \
	    goto label; \
	} \
    }

#define _PrintErrorStr(hr, pszMessage, pwsz) \
    ceERRORPRINTLINESTR((pszMessage), (pwsz), (hr))

#define _PrintErrorStr2(hr, pszMessage, pwsz, hr2) \
    _PrintErrorStr((hr), (pszMessage), (pwsz))

#define _PrintError2(hr, pszMessage, hr2) \
    _PrintErrorStr((hr), (pszMessage), NULL)

#define _PrintError(hr, pszMessage) \
    _PrintErrorStr((hr), (pszMessage), NULL)


#define _PrintIfErrorStr(hr, pszMessage, pwsz) \
    { \
	if (S_OK != (hr)) \
	{ \
	    ceERRORPRINTLINESTR((pszMessage), (pwsz), (hr)); \
	} \
    }

#define _PrintIfErrorStr2(hr, pszMessage, pwsz, hr2) \
    _PrintIfErrorStr((hr), (pszMessage), (pwsz))

#define _PrintIfError2(hr, pszMessage, hr2) \
    _PrintIfErrorStr((hr), (pszMessage), NULL)

#define _PrintIfError(hr, pszMessage) \
    _PrintIfErrorStr((hr), (pszMessage), NULL)


#define _JumpErrorStr(hr, label, pszMessage, pwsz) \
    _JumpError((hr), label, (pszMessage))

#define _JumpError(hr, label, pszMessage) \
    { \
	ceERRORPRINTLINESTR((pszMessage), NULL, (hr)); \
	goto label; \
    }


#define _JumpIfErrorStr(hr, label, pszMessage, pwsz) \
    { \
	if (S_OK != (hr)) \
	{ \
	    ceERRORPRINTLINESTR((pszMessage), (pwsz), (hr)); \
	    goto label; \
	} \
    }

#define _JumpIfErrorStr2(hr, label, pszMessage, pwsz, hr2) \
    _JumpIfErrorStr((hr), label, (pszMessage), NULL)

#define _JumpIfError2(hr, label, pszMessage, hr2) \
    _JumpIfErrorStr((hr), label, (pszMessage), NULL)

#define _JumpIfError(hr, label, pszMessage) \
    _JumpIfErrorStr((hr), label, (pszMessage), NULL)


#if DBG
#define ceERRORPRINTLINE(pszMessage, hr) \
	    ceErrorPrintLine(__FILE__, __LINE__, (pszMessage), NULL, (hr))

#define ceERRORPRINTLINESTR(pszMessage, pwszData, hr) \
	    ceErrorPrintLine(__FILE__, __LINE__, (pszMessage), (pwszData), (hr))

#define DBGPRINT(a)	ceDbgPrintf a
#else
// Windows OS Bug:1412284
#define ceERRORPRINTLINE(pszMessage, hr)
#define ceERRORPRINTLINESTR(pszMessage, pwszData, hr)
#define DBGPRINT(a)

#endif

// typedef for buffers that may be read up to and including the first sequence
// of two NULL characters:
typedef _NullNull_terminated_ WCHAR *	    CSPZZWSTR;
typedef _NullNull_terminated_ WCHAR const *  CSPCZZWSTR;

// typedef for buffers that may be read up to and including the first NULL
// pointer:
typedef _Null_terminated_ PCWSTR const *     CSPCZPCWSTR;


int WINAPIV ceDbgPrintf(BOOL fDebug, char const *pszfmt, ...);


VOID
ceErrorPrintLine(
    IN char const *pszFile,
    IN DWORD line,
    IN char const *pszMessage,
    IN WCHAR const *pwszData,
    IN HRESULT hr);

HRESULT
ceHLastError(VOID);

HRESULT
ceHError(
    IN HRESULT hr);

#define ceHEXCEPTIONCODE(_ex) ceHError((_ex).GetSeCode())

#define chLBRACE	'{'
#define chRBRACE	'}'
#define szLBRACE	"{"
#define szRBRACE	"}"
#define wcLBRACE	L'{'
#define wcRBRACE	L'}'
#define wszLBRACE	L"{"
#define wszRBRACE	L"}"

#define chLPAREN	'('
#define chRPAREN	')'
#define szLPAREN	"("
#define szRPAREN	")"
#define wcLPAREN	L'('
#define wcRPAREN	L')'
#define wszLPAREN	L"("
#define wszRPAREN	L")"

// Constants chosen to avoid DWORD overflow:

#define CVT_WEEKS	(7 * CVT_DAYS)
#define CVT_DAYS	(24 * CVT_HOURS)
#define CVT_HOURS	(60 * CVT_MINUTES)
#define CVT_MINUTES	(60 * CVT_SECONDS)
#define CVT_SECONDS	(1)
#define CVT_BASE	(1000 * 1000 * 10)


enum ENUM_PERIOD
{
    ENUM_PERIOD_INVALID = -1,
    ENUM_PERIOD_SECONDS = 0,
    ENUM_PERIOD_MINUTES,
    ENUM_PERIOD_HOURS,
    ENUM_PERIOD_DAYS,
    ENUM_PERIOD_WEEKS,
    ENUM_PERIOD_MONTHS,
    ENUM_PERIOD_YEARS
};


#define IsNullOrEmptyBStr(str) \
    __pragma(prefast(suppress: __WARNING_325, "Intentional NULL check, avoid SysStringLen")) \
    (NULL == str || 0 == SysStringByteLen(str))

#define IsNullBStr(str) \
    __pragma(prefast(suppress: __WARNING_325, "Intentional NULL check")) \
    (NULL == str)


HMODULE
ceLoadSystem32Library(
    IN WCHAR const *pwszLibFileName);

typedef struct _LLFILETIME
{
    union {
	LONGLONG ll;
	FILETIME ft;
    };
} LLFILETIME;


__inline VOID
ceAddToFileTime(
    IN OUT FILETIME *pft,
    IN LONGLONG ll)
{
    LLFILETIME llft;

    llft.ft = *pft;
    llft.ll += ll;
    *pft = llft.ft;
}


__inline LONGLONG
ceSubtractFileTimes(
    IN FILETIME const *pft1,
    IN FILETIME const *pft2)
{
    LLFILETIME llft1;
    LLFILETIME llft2;

    llft1.ft = *pft1;
    llft2.ft = *pft2;
    return(llft1.ll - llft2.ll);
}


HRESULT
ceMakeExprDate(
    IN OUT DATE *pDate,
    IN LONG lDelta,
    IN enum ENUM_PERIOD enumPeriod);

HRESULT
ceTranslatePeriodUnits(
    IN WCHAR const *pwszPeriod,
    IN LONG lCount,
    OUT enum ENUM_PERIOD *penumPeriod,
    OUT LONG *plCount);

WCHAR const *
ceGetOIDNameA(
    IN char const *pszObjId);

WCHAR const *
ceGetOIDName(
    IN WCHAR const *pwszObjId);

BOOL
ceDecodeObject(
    IN DWORD dwEncodingType,
    IN LPCSTR lpszStructType,
    IN BYTE const *pbEncoded,
    IN DWORD cbEncoded,
    IN BOOL fCoTaskMemAlloc,
    OUT VOID **ppvStructInfo,
    OUT DWORD *pcbStructInfo);

BOOL
ceDecodeObjectEx(
    IN DWORD dwEncodingType,
    IN LPCSTR lpszStructType,
    IN BYTE const *pbEncoded,
    IN DWORD cbEncoded,
    IN DWORD dwFlags,
    OUT VOID **ppvStructInfo,
    OUT DWORD *pcbStructInfo);

BOOL
ceEncodeObject(
    DWORD dwEncodingType,
    IN LPCSTR lpszStructType,
    IN VOID const *pvStructInfo,
    IN DWORD dwFlags,
    IN BOOL fCoTaskMemAlloc,
    OUT BYTE **ppbEncoded,
    OUT DWORD *pcbEncoded);

VOID 
ceGetBStrBuffer(
    _In_opt_ BSTR str,
    _Out_ DWORD *pcbOut,
    _Outptr_result_buffer_(*pcbOut) PBYTE *ppbOut);

BSTR
ceAllocBStrBlob(
    _In_reads_bytes_opt_(cbIn) BYTE const *pbIn,
    _In_ DWORD cbIn);

HRESULT
ceDecodeCertString(
    _In_ BSTR const bstrIn,
    _In_ DWORD Flags,
    _Outptr_result_bytebuffer_(*pcbOut) BYTE **ppbOut,
    _Out_ DWORD *pcbOut);

HRESULT
ceEncodeCertString(
    _In_reads_bytes_(cbIn) BYTE const *pbIn,
    _In_ DWORD cbIn,
    _In_ DWORD Flags,
    _Outptr_ BSTR *pbstrOut);

WCHAR *
ceDuplicateString(
    IN WCHAR const *pwsz);

HRESULT
ceDupString(
    IN WCHAR const *pwszIn,
    _Outptr_ PWSTR *ppwszOut);

BOOL
ceConvertWszToSz(
    _Outptr_ PSTR *ppsz,
    IN WCHAR const *pwc,
    IN LONG cb);

BOOL
ceConvertWszToBstr(
    OUT BSTR *pbstr,
    IN WCHAR const *pwc,
    IN LONG cb);

BOOL
ceConvertSzToWsz(
    _Outptr_ PWSTR *ppwsz,
    IN char const *pch,
    IN LONG cch);

BOOL
ceConvertSzToBstr(
    OUT BSTR *pbstr,
    IN CHAR const *pch,
    IN LONG cch);

HRESULT
ceDateToFileTime(
    IN DATE const *pDate,
    OUT FILETIME *pft);

HRESULT
ceFileTimeToDate(
    IN FILETIME const *pft,
    OUT DATE *pDate);

HRESULT
ceVerifyObjIdA(
    IN char const *pszObjId);

HRESULT
ceVerifyObjId(
    IN WCHAR const *pwszObjId);

HRESULT
ceVerifyAndConvertWszToSzObjIds(
    _In_opt_ CSPZZWSTR ppwszObjIds,
   _Out_ DWORD* pdwcObjIds,
    _Outptr_ PZPSTR *pppszObjIds);

HRESULT
ceVerifyAltNameString(
    IN LONG NameChoice,
    IN BSTR strName);

HRESULT
ceDispatchSetErrorInfo(
    IN HRESULT hrError,
    IN WCHAR const *pwszDescription,
    OPTIONAL IN WCHAR const *pwszProgId,
    OPTIONAL IN IID const *piid);

VOID
ceDispatchClearErrorInfo(VOID);

VOID
ceInitErrorMessageText(
    IN HMODULE hMod,
    IN DWORD idsUnexpected,
    IN DWORD idsUnknownErrorCode);	// L"Error %ws %ws"

WCHAR const *
ceGetErrorMessageText(
    IN HRESULT hr,
    IN BOOL fHResultString);

WCHAR const *
ceGetErrorMessageTextEx(
    IN HRESULT hr,
    IN BOOL fHResultString,
    IN OPTIONAL WCHAR const * const *papwszInsertionText);

WCHAR const *
ceHResultToString(
    _Inout_ PWSTR awchr,
    IN HRESULT hr);


#define cwcFILENAMESUFFIXMAX		20
#define cwcSUFFIXMAX	(1 + 5 + 1)	// five decimal digits plus parentheses

#define wszFCSAPARM_SERVERDNSNAME		L"%1"
#define wszFCSAPARM_SERVERSHORTNAME		L"%2"
#define wszFCSAPARM_SANITIZEDCANAME		L"%3"
#define wszFCSAPARM_CERTFILENAMESUFFIX		L"%4"
#define wszFCSAPARM_DOMAINDN			L"%5"
#define wszFCSAPARM_CONFIGDN			L"%6"
#define wszFCSAPARM_SANITIZEDCANAMEHASH		L"%7"
#define wszFCSAPARM_CRLFILENAMESUFFIX		L"%8"
#define wszFCSAPARM_CRLDELTAFILENAMESUFFIX	L"%9"
#define wszFCSAPARM_DSCRLATTRIBUTE		L"%10"
#define wszFCSAPARM_DSCACERTATTRIBUTE		L"%11"
#define wszFCSAPARM_DSUSERCERTATTRIBUTE		L"%12"
#define wszFCSAPARM_DSKRACERTATTRIBUTE		L"%13"
#define wszFCSAPARM_DSCROSSCERTPAIRATTRIBUTE	L"%14"


HRESULT
ceFormatCertsrvStringArray(
    IN BOOL fURL,
    IN LPCWSTR pwszServerName_p1_2,
    IN LPCWSTR pwszSanitizedName_p3_7,
    IN DWORD   iCert_p4,
    IN DWORD   iCertTarget_p4,
    IN LPCWSTR pwszDomainDN_p5,
    IN LPCWSTR pwszConfigDN_p6,
    IN DWORD   iCRL_p8,
    IN BOOL    fDeltaCRL_p9,
    IN BOOL    fDSAttrib_p10_11,
    IN DWORD   cStrings,
    IN LPCWSTR *apwszStringsIn,
    _Outptr_ LPWSTR *apwszStringsOut);

HRESULT
ceBuildPathAndExt(
    IN WCHAR const *pwszDir,
    IN WCHAR const *pwszFile,
    OPTIONAL IN WCHAR const *pwszExt,
    _Outptr_ PWSTR *ppwszPath);

HRESULT
ceInternetCanonicalizeUrl(
    IN WCHAR const *pwszIn,
    _Outptr_ PWSTR *ppwszOut);

int 
ceWtoI(
    IN WCHAR const *pwszDigitString,
    OUT BOOL *pfValid);

int
celstrcmpiL(
    IN WCHAR const *pwsz1,
    IN WCHAR const *pwsz2);

HRESULT
ceIsConfigLocal(
    _In_ PCWSTR pwszConfig,
    _Outptr_opt_ PCWSTR *ppwszMachine,
    _Out_ BOOL *pfLocal);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __CELIB_H__
