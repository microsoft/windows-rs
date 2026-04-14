#include <winapifamily.h>

#if _MSC_VER >= 1200
#pragma warning(push)
#pragma warning(disable:4001) /* nonstandard extension : single line comment */
#pragma warning(disable:4820) // padding added after data member
#endif

//+---------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//  File:       oleauto.h
//
//  Contents:   Defines Ole Automation support function prototypes, constants
//
//----------------------------------------------------------------------------

#if !defined( _OLEAUTO_H_ )
#define _OLEAUTO_H_

#if _MSC_VER > 1000
#pragma once
#endif

// Set packing to 8 for ISV, and Win95 support
#ifndef RC_INVOKED
#include <pshpack8.h>
#endif // RC_INVOKED

//  Definition of the OLE Automation APIs, and macros.

#ifdef _OLEAUT32_
#define WINOLEAUTAPI        STDAPI
#define WINOLEAUTAPI_(type) STDAPI_(type)
#else
#define WINOLEAUTAPI        EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define WINOLEAUTAPI_(type) EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif

EXTERN_C const IID IID_StdOle;

#define STDOLE_MAJORVERNUM  0x1
#define STDOLE_MINORVERNUM  0x0
#define STDOLE_LCID         0x0000

// Version # of stdole2.tlb
#define STDOLE2_MAJORVERNUM 0x2
#define STDOLE2_MINORVERNUM 0x0
#define STDOLE2_LCID        0x0000

/* if not already picked up from olenls.h */
#ifndef _LCID_DEFINED
typedef DWORD LCID;
# define _LCID_DEFINED
#endif  /* _LCID_DEFINED */

#ifndef BEGIN_INTERFACE
#define BEGIN_INTERFACE
#define END_INTERFACE
#endif

/* pull in the MIDL generated header */
#include <oaidl.h>

/*---------------------------------------------------------------------*/
/*                            BSTR API                                 */
/*---------------------------------------------------------------------*/

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI_(BSTR) SysAllocString(_In_opt_z_ const OLECHAR * psz);
WINOLEAUTAPI_(INT)  SysReAllocString(_Inout_ _At_(*pbstr, _Pre_z_ _Post_z_ _Post_readable_size_(_String_length_(psz)+1)) BSTR* pbstr, _In_opt_z_ const OLECHAR* psz);
WINOLEAUTAPI_(_Ret_writes_maybenull_z_(ui+1) BSTR) SysAllocStringLen(_In_reads_opt_(ui) const OLECHAR * strIn, UINT ui);
_Check_return_ WINOLEAUTAPI_(INT)  SysReAllocStringLen(_Inout_ _At_(*pbstr, _Pre_z_ _Post_z_ _Post_readable_size_(len+1)) BSTR* pbstr, _In_opt_z_ const OLECHAR* psz, _In_ unsigned int len);
WINOLEAUTAPI SysAddRefString(_In_ BSTR bstrString);
WINOLEAUTAPI_(void) SysReleaseString(_In_ BSTR bstrString);
WINOLEAUTAPI_(void) SysFreeString(_Frees_ptr_opt_ BSTR bstrString);
WINOLEAUTAPI_(_Post_equal_to_(pbstr == NULL ? 0 : _String_length_(pbstr)) UINT) SysStringLen(_In_opt_ BSTR pbstr);

#if (defined (_WIN32) || defined (_WIN64))
WINOLEAUTAPI_(_Post_equal_to_(_String_length_(bstr) * sizeof(OLECHAR)) UINT) SysStringByteLen(_In_opt_ BSTR bstr);
WINOLEAUTAPI_(BSTR) SysAllocStringByteLen(_In_opt_z_ LPCSTR psz, _In_ UINT len);

#endif /* (defined (_WIN32) || defined (_WIN64)) */

#endif /*  WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/*---------------------------------------------------------------------*/
/*                            Time API                                 */
/*---------------------------------------------------------------------*/

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI_(INT) DosDateTimeToVariantTime(_In_ USHORT wDosDate, _In_ USHORT wDosTime, _Out_ DOUBLE * pvtime);

WINOLEAUTAPI_(INT) VariantTimeToDosDateTime(_In_ DOUBLE vtime, _Out_ USHORT * pwDosDate, _Out_ USHORT * pwDosTime);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#if (defined (_WIN32) || defined (_WIN64))

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI_(INT) SystemTimeToVariantTime(_In_ LPSYSTEMTIME lpSystemTime, _Out_ DOUBLE *pvtime);
WINOLEAUTAPI_(INT) VariantTimeToSystemTime(_In_ DOUBLE vtime, _Out_ LPSYSTEMTIME lpSystemTime);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#endif /* (defined (_WIN32) || defined (_WIN64)) */


/*---------------------------------------------------------------------*/
/*                          SafeArray API                              */
/*---------------------------------------------------------------------*/

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI SafeArrayAllocDescriptor(_In_ UINT cDims, _Outptr_ SAFEARRAY ** ppsaOut);
WINOLEAUTAPI SafeArrayAllocDescriptorEx(_In_ VARTYPE vt, _In_ UINT cDims, _Outptr_ SAFEARRAY ** ppsaOut);
WINOLEAUTAPI SafeArrayAllocData(_In_ SAFEARRAY * psa);
WINOLEAUTAPI_(SAFEARRAY *) SafeArrayCreate(_In_ VARTYPE vt, _In_ UINT cDims, _In_ SAFEARRAYBOUND * rgsabound);
WINOLEAUTAPI_(SAFEARRAY *) SafeArrayCreateEx(_In_ VARTYPE vt, _In_ UINT cDims, _In_ SAFEARRAYBOUND * rgsabound, _In_ PVOID pvExtra);
_Check_return_
WINOLEAUTAPI SafeArrayCopyData(_In_ SAFEARRAY *psaSource, _In_ SAFEARRAY *psaTarget);
WINOLEAUTAPI_(void) SafeArrayReleaseDescriptor(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayDestroyDescriptor(_In_ SAFEARRAY * psa);
WINOLEAUTAPI_(void) SafeArrayReleaseData(_In_ PVOID pData);
WINOLEAUTAPI SafeArrayDestroyData(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayAddRef(_In_ SAFEARRAY * psa, _Out_ PVOID *ppDataToRelease);
WINOLEAUTAPI SafeArrayDestroy(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayRedim(_Inout_ SAFEARRAY * psa, _In_ SAFEARRAYBOUND * psaboundNew);
WINOLEAUTAPI_(UINT) SafeArrayGetDim(_In_ SAFEARRAY * psa);
WINOLEAUTAPI_(UINT) SafeArrayGetElemsize(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayGetUBound(_In_ SAFEARRAY * psa, _In_ UINT nDim, _Out_ LONG * plUbound);
WINOLEAUTAPI SafeArrayGetLBound(_In_ SAFEARRAY * psa, _In_ UINT nDim, _Out_ LONG * plLbound);
WINOLEAUTAPI SafeArrayLock(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayUnlock(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayAccessData(_In_ SAFEARRAY * psa, _Outptr_result_buffer_(_Inexpressible_(psa->cbElements * product(psa->rgsabound[0..psa->cDims-1]->cElements))) void HUGEP** ppvData);
WINOLEAUTAPI SafeArrayUnaccessData(_In_ SAFEARRAY * psa);
WINOLEAUTAPI SafeArrayGetElement(_In_ SAFEARRAY * psa, _In_reads_(_Inexpressible_(psa->cDims)) LONG * rgIndices, _Out_ void * pv);
_Check_return_
WINOLEAUTAPI SafeArrayPutElement(_In_ SAFEARRAY * psa, _In_reads_(_Inexpressible_(psa->cDims)) LONG * rgIndices, _In_ void * pv);
_Check_return_
WINOLEAUTAPI SafeArrayCopy(_In_ SAFEARRAY * psa, _Outptr_ SAFEARRAY ** ppsaOut);
WINOLEAUTAPI SafeArrayPtrOfIndex(_In_ SAFEARRAY * psa, _In_reads_(psa->cDims) LONG * rgIndices, _Outptr_result_bytebuffer_(psa->cbElements) void ** ppvData);
WINOLEAUTAPI SafeArraySetRecordInfo(_In_ SAFEARRAY * psa, _In_ IRecordInfo * prinfo);
WINOLEAUTAPI SafeArrayGetRecordInfo(_In_ SAFEARRAY * psa, _Outptr_ IRecordInfo ** prinfo);
WINOLEAUTAPI SafeArraySetIID(_In_ SAFEARRAY * psa, _In_ REFGUID guid);
WINOLEAUTAPI SafeArrayGetIID(_In_ SAFEARRAY * psa, _Out_ GUID * pguid);
WINOLEAUTAPI SafeArrayGetVartype(_In_ SAFEARRAY * psa, _Out_ VARTYPE * pvt);
WINOLEAUTAPI_(SAFEARRAY *) SafeArrayCreateVector(_In_ VARTYPE vt, _In_ LONG lLbound, _In_ ULONG cElements);
WINOLEAUTAPI_(SAFEARRAY *) SafeArrayCreateVectorEx(_In_ VARTYPE vt, _In_ LONG lLbound, _In_ ULONG cElements, _In_ PVOID pvExtra);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/*---------------------------------------------------------------------*/
/*                           VARIANT API                               */
/*---------------------------------------------------------------------*/


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI_(void) VariantInit(_Out_ VARIANTARG * pvarg);
WINOLEAUTAPI VariantClear(_Inout_ VARIANTARG * pvarg);
_Check_return_
WINOLEAUTAPI VariantCopy(_Inout_ VARIANTARG * pvargDest, _In_ const VARIANTARG * pvargSrc);
_Check_return_
WINOLEAUTAPI VariantCopyInd(_Inout_ VARIANT * pvarDest, _In_ const VARIANTARG * pvargSrc);
_Check_return_
WINOLEAUTAPI VariantChangeType(_Inout_ VARIANTARG * pvargDest,
               _In_ const VARIANTARG * pvarSrc, _In_ USHORT wFlags, _In_ VARTYPE vt);
_Check_return_
WINOLEAUTAPI VariantChangeTypeEx(_Inout_ VARIANTARG * pvargDest,
               _In_ const VARIANTARG * pvarSrc, _In_ LCID lcid, _In_ USHORT wFlags, _In_ VARTYPE vt);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

// Flags for VariantChangeType/VariantChangeTypeEx
#define VARIANT_NOVALUEPROP      0x01
#define VARIANT_ALPHABOOL        0x02 // For VT_BOOL to VT_BSTR conversions,
                                      // convert to "True"/"False" instead of
                                      // "-1"/"0"
#define VARIANT_NOUSEROVERRIDE   0x04 // For conversions to/from VT_BSTR,
				      // passes LOCALE_NOUSEROVERRIDE
				      // to core coercion routines
#define VARIANT_CALENDAR_HIJRI   0x08
#define VARIANT_LOCALBOOL        0x10 // For VT_BOOL to VT_BSTR and back,
                                      // convert to local language rather than
                                      // English
#define VARIANT_CALENDAR_THAI		0x20  // SOUTHASIA calendar support
#define VARIANT_CALENDAR_GREGORIAN	0x40  // SOUTHASIA calendar support
#define VARIANT_USE_NLS                 0x80  // NLS function call support
/*---------------------------------------------------------------------*/
/*                Vector <-> Bstr conversion APIs                      */
/*---------------------------------------------------------------------*/

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

_Check_return_
WINOLEAUTAPI VectorFromBstr (_In_ BSTR bstr, _Outptr_ SAFEARRAY ** ppsa);
_Check_return_
WINOLEAUTAPI BstrFromVector (_In_ SAFEARRAY *psa, _Out_ BSTR *pbstr);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/*---------------------------------------------------------------------*/
/*                     Variant API Flags                               */
/*---------------------------------------------------------------------*/

/* Any of the coersion functions that converts either from or to a string
 * takes an additional lcid and dwFlags arguments. The lcid argument allows
 * locale specific parsing to occur.  The dwFlags allow additional function
 * specific condition to occur.  All function that accept the dwFlags argument
 * can include either 0 or LOCALE_NOUSEROVERRIDE flag.
 */

/* The VarDateFromStr and VarBstrFromDate functions also accept the
 * VAR_TIMEVALUEONLY and VAR_DATEVALUEONLY flags
 */
#define VAR_TIMEVALUEONLY       ((DWORD)0x00000001)    /* return time value */
#define VAR_DATEVALUEONLY       ((DWORD)0x00000002)    /* return date value */

/* VarDateFromUdate() only */
#define VAR_VALIDDATE           ((DWORD)0x00000004)

/* Accepted by all date & format APIs */
#define VAR_CALENDAR_HIJRI      ((DWORD)0x00000008)    /* use Hijri calender */

/* Booleans can optionally be accepted in localized form. Pass VAR_LOCALBOOL
 * into VarBoolFromStr and VarBstrFromBool to use localized boolean names
 */
#define VAR_LOCALBOOL           ((DWORD)0x00000010)

/* When passed into VarFormat and VarFormatFromTokens, prevents substitution
 * of formats in the case where a string is passed in that can not be
 * coverted into the desired type. (for ex, 'Format("Hello", "General Number")')
 */
#define VAR_FORMAT_NOSUBSTITUTE ((DWORD)0x00000020)

/*
 * For VarBstrFromDate only - forces years to be 4 digits rather than shortening
 * to 2-digits when the years is in the date window.
 */
#define VAR_FOURDIGITYEARS	((DWORD)0x00000040)

/*
 * Use NLS functions to format date, currency, time, and number.
 */
#ifndef LOCALE_USE_NLS
#define LOCALE_USE_NLS 0x10000000
#endif

// SOUTHASIA START
/* SOUTHASIA
 * For VarBstrFromDate only - forces years to be 4 digits rather than shortening
 * to 2-digits when the years is in the date window.
 */
#define VAR_CALENDAR_THAI	   ((DWORD)0x00000080)
#define	VAR_CALENDAR_GREGORIAN ((DWORD)0x00000100)
//SOUTHASIA END

#define VTDATEGRE_MAX 2958465   /* Dec 31, 9999, 0:0:0 in Gregorain Calendar */
#define VTDATEGRE_MIN -657434   /* Jan  1,  100, 0:0:0 in Gregorain Calendar */
/*---------------------------------------------------------------------*/
/*                     VARTYPE Coercion API                            */
/*---------------------------------------------------------------------*/

/* Note: The routines that convert *from* a string are defined
 * to take a OLECHAR* rather than a BSTR because no allocation is
 * required, and this makes the routines a bit more generic.
 * They may of course still be passed a BSTR as the strIn param.
 */

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI VarUI1FromI2(SHORT sIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromI4(LONG lIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromI8(LONG64 i64In, _Out_ BYTE FAR* pbOut);
WINOLEAUTAPI VarUI1FromR4(FLOAT fltIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromR8(DOUBLE dblIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromCy(CY cyIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromDate(DATE dateIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromStr(_In_ LPCOLESTR strIn, LCID lcid, ULONG dwFlags, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromDisp(IDispatch * pdispIn, LCID lcid, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromBool(VARIANT_BOOL boolIn, _Out_ BYTE * pbOut);
WINOLEAUTAPI VarUI1FromI1(CHAR cIn, _Out_ BYTE *pbOut);
WINOLEAUTAPI VarUI1FromUI2(USHORT uiIn, _Out_ BYTE *pbOut);
WINOLEAUTAPI VarUI1FromUI4(ULONG ulIn, _Out_ BYTE *pbOut);
WINOLEAUTAPI VarUI1FromUI8(ULONG64 ui64In, _Out_ BYTE FAR* pbOut);
WINOLEAUTAPI VarUI1FromDec(_In_ const DECIMAL *pdecIn, _Out_ BYTE *pbOut);

WINOLEAUTAPI VarI2FromUI1(BYTE bIn, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromI4(LONG lIn, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromI8(LONG64 i64In, _Out_ SHORT FAR* psOut);
WINOLEAUTAPI VarI2FromR4(FLOAT fltIn, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromR8(DOUBLE dblIn, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromCy(CY cyIn, SHORT * psOut);
WINOLEAUTAPI VarI2FromDate(DATE dateIn, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromStr(_In_ LPCOLESTR strIn, LCID lcid, ULONG dwFlags, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromDisp(IDispatch * pdispIn, LCID lcid, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromBool(VARIANT_BOOL boolIn, _Out_ SHORT * psOut);
WINOLEAUTAPI VarI2FromI1(CHAR cIn, _Out_ SHORT *psOut);
WINOLEAUTAPI VarI2FromUI2(USHORT uiIn, _Out_ SHORT *psOut);
WINOLEAUTAPI VarI2FromUI4(ULONG ulIn, _Out_ SHORT *psOut);
WINOLEAUTAPI VarI2FromUI8(ULONG64 ui64In, _Out_ SHORT FAR* psOut);
WINOLEAUTAPI VarI2FromDec(_In_ const DECIMAL *pdecIn, _Out_ SHORT *psOut);

WINOLEAUTAPI VarI4FromUI1(BYTE bIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromI2(SHORT sIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromI8(LONG64 i64In, _Out_ LONG FAR* plOut);
WINOLEAUTAPI VarI4FromR4(FLOAT fltIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromR8(DOUBLE dblIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromCy(CY cyIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromDate(DATE dateIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromStr(_In_ LPCOLESTR strIn, LCID lcid, ULONG dwFlags, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromDisp(IDispatch * pdispIn, _In_ LCID lcid, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromBool(VARIANT_BOOL boolIn, _Out_ LONG * plOut);
WINOLEAUTAPI VarI4FromI1(CHAR cIn, _Out_ LONG *plOut);
WINOLEAUTAPI VarI4FromUI2(USHORT uiIn, _Out_ LONG *plOut);
WINOLEAUTAPI VarI4FromUI4(ULONG ulIn, _Out_ LONG *plOut);
WINOLEAUTAPI VarI4FromUI8(ULONG64 ui64In, _Out_ LONG FAR* plOut);
WINOLEAUTAPI VarI4FromDec(_In_ const DECIMAL *pdecIn, _Out_ LONG *plOut);

/******************************************/

WINOLEAUTAPI VarI8FromUI1(BYTE bIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromI2(SHORT sIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromR4(FLOAT fltIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromR8(DOUBLE dblIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromCy(_In_ CY cyIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromDate(DATE dateIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromDisp(IDispatch FAR* pdispIn, _In_ LCID lcid, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromBool(VARIANT_BOOL boolIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromI1(CHAR cIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromUI2(USHORT uiIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromUI4(ULONG ulIn, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromUI8(ULONG64 ui64In, _Out_ LONG64 FAR* pi64Out);
WINOLEAUTAPI VarI8FromDec(_In_ const DECIMAL *pdecIn, _Out_ LONG64 FAR* pi64Out);

/*********************/



WINOLEAUTAPI VarR4FromUI1(BYTE bIn, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromI2(SHORT sIn, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromI4(LONG lIn, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromI8(LONG64 i64In, _Out_ FLOAT FAR* pfltOut);
WINOLEAUTAPI VarR4FromR8(DOUBLE dblIn, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromCy(CY cyIn, FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromDate(DATE dateIn, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromStr(_In_ LPCOLESTR strIn, LCID lcid, ULONG dwFlags, _Out_ FLOAT *pfltOut);
WINOLEAUTAPI VarR4FromDisp(IDispatch * pdispIn, LCID lcid, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromBool(VARIANT_BOOL boolIn, _Out_ FLOAT * pfltOut);
WINOLEAUTAPI VarR4FromI1(CHAR cIn, _Out_ FLOAT *pfltOut);
WINOLEAUTAPI VarR4FromUI2(USHORT uiIn, _Out_ FLOAT *pfltOut);
WINOLEAUTAPI VarR4FromUI4(ULONG ulIn, _Out_ FLOAT *pfltOut);
WINOLEAUTAPI VarR4FromUI8(ULONG64 ui64In, _Out_ FLOAT FAR* pfltOut);
WINOLEAUTAPI VarR4FromDec(_In_ const DECIMAL *pdecIn, _Out_ FLOAT *pfltOut);

WINOLEAUTAPI VarR8FromUI1(BYTE bIn, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromI2(SHORT sIn, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromI4(LONG lIn, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromI8(LONG64 i64In, _Out_ DOUBLE FAR* pdblOut);
WINOLEAUTAPI VarR8FromR4(FLOAT fltIn, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromCy(CY cyIn, DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromDate(DATE dateIn, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromStr(_In_ LPCOLESTR strIn, LCID lcid, ULONG dwFlags, _Out_ DOUBLE *pdblOut);
WINOLEAUTAPI VarR8FromDisp(IDispatch * pdispIn, LCID lcid, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromBool(VARIANT_BOOL boolIn, _Out_ DOUBLE * pdblOut);
WINOLEAUTAPI VarR8FromI1(CHAR cIn, DOUBLE *pdblOut);
WINOLEAUTAPI VarR8FromUI2(USHORT uiIn, _Out_ DOUBLE *pdblOut);
WINOLEAUTAPI VarR8FromUI4(ULONG ulIn, _Out_ DOUBLE *pdblOut);
WINOLEAUTAPI VarR8FromUI8(ULONG64 ui64In, _Out_ DOUBLE FAR* pdblOut);
WINOLEAUTAPI VarR8FromDec(_In_ const DECIMAL *pdecIn, _Out_ DOUBLE *pdblOut);

WINOLEAUTAPI VarDateFromUI1(BYTE bIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromI2(SHORT sIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromI4(LONG lIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromI8(LONG64 i64In, _Out_ DATE FAR* pdateOut);
WINOLEAUTAPI VarDateFromR4(FLOAT fltIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromR8(DOUBLE dblIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromCy(CY cyIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ DATE *pdateOut);
WINOLEAUTAPI VarDateFromDisp(IDispatch * pdispIn, LCID lcid, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromBool(VARIANT_BOOL boolIn, _Out_ DATE * pdateOut);
WINOLEAUTAPI VarDateFromI1(CHAR cIn, _Out_ DATE *pdateOut);
WINOLEAUTAPI VarDateFromUI2(USHORT uiIn, _Out_ DATE *pdateOut);
WINOLEAUTAPI VarDateFromUI4(ULONG ulIn, _Out_ DATE *pdateOut);
WINOLEAUTAPI VarDateFromUI8(ULONG64 ui64In, _Out_ DATE FAR* pdateOut);
WINOLEAUTAPI VarDateFromDec(_In_ const DECIMAL *pdecIn, _Out_ DATE *pdateOut);

WINOLEAUTAPI VarCyFromUI1(BYTE bIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromI2(SHORT sIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromI4(LONG lIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromI8(LONG64 i64In, _Out_ CY FAR* pcyOut);
WINOLEAUTAPI VarCyFromR4(FLOAT fltIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromR8(DOUBLE dblIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromDate(DATE dateIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromDisp(_In_ IDispatch * pdispIn, LCID lcid, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromBool(VARIANT_BOOL boolIn, _Out_ CY * pcyOut);
WINOLEAUTAPI VarCyFromI1(CHAR cIn, _Out_ CY *pcyOut);
WINOLEAUTAPI VarCyFromUI2(USHORT uiIn, _Out_ CY *pcyOut);
WINOLEAUTAPI VarCyFromUI4(ULONG ulIn, _Out_ CY *pcyOut);
WINOLEAUTAPI VarCyFromUI8(ULONG64 ui64In, _Out_ CY FAR* pcyOut);
WINOLEAUTAPI VarCyFromDec(_In_ const DECIMAL *pdecIn, _Out_ CY *pcyOut);

WINOLEAUTAPI VarBstrFromUI1(BYTE bVal, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromI2(SHORT iVal, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromI4(LONG lIn, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromI8(LONG64 i64In, LCID lcid, ULONG dwFlags, _Out_ BSTR FAR* pbstrOut);
WINOLEAUTAPI VarBstrFromR4(FLOAT fltIn, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromR8(DOUBLE dblIn, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromCy(CY cyIn, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromDate(_In_ DATE dateIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromDisp(IDispatch * pdispIn, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromBool(VARIANT_BOOL boolIn, LCID lcid, ULONG dwFlags, _Out_ BSTR * pbstrOut);
WINOLEAUTAPI VarBstrFromI1(CHAR cIn, LCID lcid, ULONG dwFlags, _Out_ BSTR *pbstrOut);
WINOLEAUTAPI VarBstrFromUI2(USHORT uiIn, LCID lcid, ULONG dwFlags, _Out_ BSTR *pbstrOut);
WINOLEAUTAPI VarBstrFromUI4(ULONG ulIn, LCID lcid, ULONG dwFlags, _Out_ BSTR *pbstrOut);
WINOLEAUTAPI VarBstrFromUI8(ULONG64 ui64In, LCID lcid, ULONG dwFlags, _Out_ BSTR FAR* pbstrOut);
WINOLEAUTAPI VarBstrFromDec(_In_ const DECIMAL *pdecIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ BSTR *pbstrOut);

WINOLEAUTAPI VarBoolFromUI1(BYTE bIn, _Out_ VARIANT_BOOL * pboolOut);
_Check_return_ WINOLEAUTAPI VarBoolFromI2(_In_ SHORT sIn, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromI4(LONG lIn, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromI8(LONG64 i64In, _Out_ VARIANT_BOOL FAR* pboolOut);
WINOLEAUTAPI VarBoolFromR4(FLOAT fltIn, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromR8(DOUBLE dblIn, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromDate(DATE dateIn, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromCy(CY cyIn, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromStr(_In_ LPCOLESTR strIn, LCID lcid, ULONG dwFlags, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromDisp(IDispatch * pdispIn, LCID lcid, _Out_ VARIANT_BOOL * pboolOut);
WINOLEAUTAPI VarBoolFromI1(CHAR cIn, _Out_ VARIANT_BOOL *pboolOut);
WINOLEAUTAPI VarBoolFromUI2(USHORT uiIn, _Out_ VARIANT_BOOL *pboolOut);
WINOLEAUTAPI VarBoolFromUI4(ULONG ulIn, _Out_ VARIANT_BOOL *pboolOut);
WINOLEAUTAPI VarBoolFromUI8(ULONG64 i64In, _Out_ VARIANT_BOOL FAR* pboolOut);
WINOLEAUTAPI VarBoolFromDec(_In_ const DECIMAL *pdecIn, _Out_ VARIANT_BOOL *pboolOut);

WINOLEAUTAPI
VarI1FromUI1(
    _In_ BYTE bIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromI2(
    _In_ SHORT uiIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromI4(
    _In_ LONG lIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromI8(
    _In_ LONG64 i64In,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromR4(
    _In_ FLOAT fltIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromR8(
    _In_ DOUBLE dblIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromDate(
    _In_ DATE dateIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromCy(
    _In_ CY cyIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromStr(
    _In_ LPCOLESTR strIn,
    _In_ LCID lcid,
    _In_ ULONG dwFlags,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromDisp(
    _In_ IDispatch *pdispIn,
    _In_ LCID lcid,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromBool(
    _In_ VARIANT_BOOL boolIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromUI2(
    _In_ USHORT uiIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromUI4(
    _In_ ULONG ulIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromUI8(
    _In_ ULONG64 i64In,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI
VarI1FromDec(
    _In_ const DECIMAL *pdecIn,
    _Out_ CHAR *pcOut
    );

WINOLEAUTAPI VarUI2FromUI1(BYTE bIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromI2(SHORT uiIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromI4(LONG lIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromI8(LONG64 i64In, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromR4(FLOAT fltIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromR8(DOUBLE dblIn, USHORT *puiOut);
WINOLEAUTAPI VarUI2FromDate(DATE dateIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromCy(CY cyIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromDisp(_In_ IDispatch *pdispIn, LCID lcid, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromBool(VARIANT_BOOL boolIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromI1(CHAR cIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromUI4(ULONG ulIn, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromUI8(ULONG64 i64In, _Out_ USHORT *puiOut);
WINOLEAUTAPI VarUI2FromDec(_In_ const DECIMAL *pdecIn, _Out_ USHORT *puiOut);

WINOLEAUTAPI VarUI4FromUI1(BYTE bIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromI2(_In_ SHORT uiIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromI4(LONG lIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromI8(LONG64 i64In, _Out_ ULONG *plOut);
WINOLEAUTAPI VarUI4FromR4(FLOAT fltIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromR8(DOUBLE dblIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromDate(DATE dateIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromCy(CY cyIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromDisp(_In_ IDispatch *pdispIn, LCID lcid, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromBool(VARIANT_BOOL boolIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromI1(CHAR cIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromUI2(USHORT uiIn, _Out_ ULONG *pulOut);
WINOLEAUTAPI VarUI4FromUI8(ULONG64 ui64In, _Out_ ULONG *plOut);
WINOLEAUTAPI VarUI4FromDec(_In_ const DECIMAL *pdecIn, _Out_ ULONG *pulOut);

/******************************************/

WINOLEAUTAPI VarUI8FromUI1(BYTE bIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromI2(SHORT sIn, _Out_ ULONG64 FAR* pi64Out);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

WINOLEAUTAPI VarUI8FromI4(LONG lIn, _Out_ ULONG64 FAR* pi64Out);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI VarUI8FromI8(LONG64 ui64In, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromR4(FLOAT fltIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromR8(DOUBLE dblIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromCy(CY cyIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromDate(DATE dateIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromDisp(_In_ IDispatch FAR* pdispIn, LCID lcid, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromBool(VARIANT_BOOL boolIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromI1(CHAR cIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromUI2(USHORT uiIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromUI4(ULONG ulIn, _Out_ ULONG64 FAR* pi64Out);
WINOLEAUTAPI VarUI8FromDec(_In_ const DECIMAL *pdecIn, _Out_ ULONG64 FAR* pi64Out);

/*********************/




WINOLEAUTAPI VarDecFromUI1(_In_ BYTE bIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromI2(_In_ SHORT uiIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromI4(_In_ LONG lIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromI8(LONG64 i64In, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromR4(_In_ FLOAT fltIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromR8(_In_ DOUBLE dblIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromDate(_In_ DATE dateIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromCy(_In_ CY cyIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromDisp(_In_ IDispatch *pdispIn, _In_ LCID lcid, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromBool(_In_ VARIANT_BOOL boolIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromI1(_In_ CHAR cIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromUI2(_In_ USHORT uiIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromUI4(_In_ ULONG ulIn, _Out_ DECIMAL *pdecOut);
WINOLEAUTAPI VarDecFromUI8(ULONG64 ui64In, _Out_ DECIMAL *pdecOut);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define VarUI4FromUI4(in, pOut) (*(pOut) = (in))
#define VarI4FromI4(in, pOut)   (*(pOut) = (in))


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI VarI4FromI8(LONG64 i64In, _Out_ LONG *plOut);
WINOLEAUTAPI VarI4FromUI8(ULONG64 ui64In, _Out_ LONG *plOut);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#define VarUI8FromUI8(in, pOut) (*(pOut) = (in))
#define VarI8FromI8(in, pOut)   (*(pOut) = (in))


#define VarUI1FromInt       VarUI1FromI4
#define VarUI1FromUint      VarUI1FromUI4
#define VarI2FromInt        VarI2FromI4
#define VarI2FromUint       VarI2FromUI4

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define VarI4FromInt        VarI4FromI4

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#define VarI4FromUint       VarI4FromUI4
#define VarI8FromUint       VarI8FromUI4
#define VarR4FromInt        VarR4FromI4
#define VarR4FromUint       VarR4FromUI4
#define VarR8FromInt        VarR8FromI4
#define VarR8FromUint       VarR8FromUI4
#define VarDateFromInt      VarDateFromI4
#define VarDateFromUint     VarDateFromUI4
#define VarCyFromInt        VarCyFromI4
#define VarCyFromUint       VarCyFromUI4
#define VarBstrFromInt      VarBstrFromI4
#define VarBstrFromUint     VarBstrFromUI4
#define VarBoolFromInt      VarBoolFromI4
#define VarBoolFromUint     VarBoolFromUI4
#define VarI1FromInt        VarI1FromI4
#define VarI1FromUint       VarI1FromUI4
#define VarUI2FromInt       VarUI2FromI4
#define VarUI2FromUint      VarUI2FromUI4
#define VarUI4FromInt       VarUI4FromI4
#define VarUI4FromUint      VarUI4FromUI4
#define VarDecFromInt       VarDecFromI4
#define VarDecFromUint      VarDecFromUI4
#define VarIntFromUI1       VarI4FromUI1
#define VarIntFromI2        VarI4FromI2
#define VarIntFromI4        VarI4FromI4
#define VarIntFromI8        VarI4FromI8
#define VarIntFromR4        VarI4FromR4
#define VarIntFromR8        VarI4FromR8
#define VarIntFromDate      VarI4FromDate
#define VarIntFromCy        VarI4FromCy
#define VarIntFromStr       VarI4FromStr
#define VarIntFromDisp      VarI4FromDisp
#define VarIntFromBool      VarI4FromBool
#define VarIntFromI1        VarI4FromI1
#define VarIntFromUI2       VarI4FromUI2
#define VarIntFromUI4       VarI4FromUI4
#define VarIntFromUI8       VarI4FromUI8
#define VarIntFromDec       VarI4FromDec
#define VarIntFromUint      VarI4FromUI4
#define VarUintFromUI1      VarUI4FromUI1
#define VarUintFromI2       VarUI4FromI2
#define VarUintFromI4       VarUI4FromI4
#define VarUintFromI8       VarUI4FromI8
#define VarUintFromR4       VarUI4FromR4
#define VarUintFromR8       VarUI4FromR8
#define VarUintFromDate     VarUI4FromDate
#define VarUintFromCy       VarUI4FromCy
#define VarUintFromStr      VarUI4FromStr
#define VarUintFromDisp     VarUI4FromDisp
#define VarUintFromBool     VarUI4FromBool
#define VarUintFromI1       VarUI4FromI1
#define VarUintFromUI2      VarUI4FromUI2
#define VarUintFromUI4      VarUI4FromUI4
#define VarUintFromUI8      VarUI4FromUI8
#define VarUintFromDec      VarUI4FromDec
#define VarUintFromInt      VarUI4FromI4

/* Mac Note: On the Mac, the coersion functions support the
 * Symantec C++ calling convention for float/double. To support
 * float/double arguments compiled with the MPW C compiler,
 * use the following APIs to move MPW float/double values into
 * a VARIANT.
 */

/*---------------------------------------------------------------------*/
/*            New VARIANT <-> string parsing functions                 */
/*---------------------------------------------------------------------*/

typedef struct {
    INT   cDig;
    ULONG dwInFlags;
    ULONG dwOutFlags;
    INT   cchUsed;
    INT   nBaseShift;
    INT   nPwr10;
} NUMPARSE;

/* flags used by both dwInFlags and dwOutFlags:
 */
#define NUMPRS_LEADING_WHITE    0x0001
#define NUMPRS_TRAILING_WHITE   0x0002
#define NUMPRS_LEADING_PLUS     0x0004
#define NUMPRS_TRAILING_PLUS    0x0008
#define NUMPRS_LEADING_MINUS    0x0010
#define NUMPRS_TRAILING_MINUS   0x0020
#define NUMPRS_HEX_OCT          0x0040
#define NUMPRS_PARENS           0x0080
#define NUMPRS_DECIMAL          0x0100
#define NUMPRS_THOUSANDS        0x0200
#define NUMPRS_CURRENCY         0x0400
#define NUMPRS_EXPONENT         0x0800
#define NUMPRS_USE_ALL          0x1000
#define NUMPRS_STD              0x1FFF

/* flags used by dwOutFlags only:
 */
#define NUMPRS_NEG              0x10000
#define NUMPRS_INEXACT          0x20000

/* flags used by VarNumFromParseNum to indicate acceptable result types:
 */
#define VTBIT_I1        (1 << VT_I1)
#define VTBIT_UI1       (1 << VT_UI1)
#define VTBIT_I2        (1 << VT_I2)
#define VTBIT_UI2       (1 << VT_UI2)
#define VTBIT_I4        (1 << VT_I4)
#define VTBIT_UI4       (1 << VT_UI4)
#define VTBIT_I8		(1 << VT_I8)
#define VTBIT_UI8		(1 << VT_UI8)
#define VTBIT_R4        (1 << VT_R4)
#define VTBIT_R8        (1 << VT_R8)
#define VTBIT_CY        (1 << VT_CY)
#define VTBIT_DECIMAL   (1 << VT_DECIMAL)


#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

_Check_return_
WINOLEAUTAPI VarParseNumFromStr(_In_ LPCOLESTR strIn, _In_ LCID lcid, _In_ ULONG dwFlags,
            _Inout_ NUMPARSE * pnumprs, _Out_writes_(pnumprs->cDig) BYTE * rgbDig);

_Check_return_
WINOLEAUTAPI VarNumFromParseNum(_In_ NUMPARSE * pnumprs, _In_reads_(pnumprs->cDig) BYTE * rgbDig,
            _In_ ULONG dwVtBits, _Out_ VARIANT * pvar);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

/*---------------------------------------------------------------------*/
/*                     VARTYPE Math API                                */
/*---------------------------------------------------------------------*/

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

STDAPI VarAdd(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarAnd(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarCat(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarDiv(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarEqv(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarIdiv(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarImp(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarMod(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarMul(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarOr(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarPow(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarSub(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);
STDAPI VarXor(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _Out_ LPVARIANT pvarResult);

STDAPI VarAbs(_In_ LPVARIANT pvarIn, _Out_ LPVARIANT pvarResult);
STDAPI VarFix(_In_ LPVARIANT pvarIn, _Out_ LPVARIANT pvarResult);
STDAPI VarInt(_In_ LPVARIANT pvarIn, _Out_ LPVARIANT pvarResult);
STDAPI VarNeg(_In_ LPVARIANT pvarIn, _Out_ LPVARIANT pvarResult);
STDAPI VarNot(_In_ LPVARIANT pvarIn, _Out_ LPVARIANT pvarResult);

STDAPI VarRound(_In_ LPVARIANT pvarIn, _In_ int cDecimals, _Out_ LPVARIANT pvarResult);

// dwFlags passed to CompareString if a string compare
STDAPI VarCmp(_In_ LPVARIANT pvarLeft, _In_ LPVARIANT pvarRight, _In_ LCID lcid, _In_ ULONG dwFlags);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#ifdef __cplusplus
extern "C++" {

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

// Add wrapper for old ATL headers to call
__inline
HRESULT
#if !defined(_M_CEE_PURE)
STDAPICALLTYPE
#endif
VarCmp(LPVARIANT pvarLeft, LPVARIANT pvarRight, LCID lcid) {
    return VarCmp(pvarLeft, pvarRight, lcid, 0);
}

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

} // extern "C++"
#endif /*  __cplusplus */


// Decimal math
//

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

STDAPI VarDecAdd(_In_ LPDECIMAL pdecLeft, _In_ LPDECIMAL pdecRight, _Out_ LPDECIMAL pdecResult);
STDAPI VarDecDiv(_In_ LPDECIMAL pdecLeft, _In_ LPDECIMAL pdecRight, _Out_ LPDECIMAL pdecResult);
STDAPI VarDecMul(_In_ LPDECIMAL pdecLeft, _In_ LPDECIMAL pdecRight, _Out_ LPDECIMAL pdecResult);
STDAPI VarDecSub(_In_ LPDECIMAL pdecLeft, _In_ LPDECIMAL pdecRight, _Out_ LPDECIMAL pdecResult);

STDAPI VarDecAbs(_In_ LPDECIMAL pdecIn, _Out_ LPDECIMAL pdecResult);
STDAPI VarDecFix(_In_ LPDECIMAL pdecIn, _Out_ LPDECIMAL pdecResult);
STDAPI VarDecInt(_In_ LPDECIMAL pdecIn, _Out_ LPDECIMAL pdecResult);
STDAPI VarDecNeg(_In_ LPDECIMAL pdecIn, _Out_ LPDECIMAL pdecResult);

STDAPI VarDecRound(_In_ LPDECIMAL pdecIn, int cDecimals, _Out_ LPDECIMAL pdecResult);

STDAPI VarDecCmp(_In_ LPDECIMAL pdecLeft, _In_ LPDECIMAL pdecRight);
STDAPI VarDecCmpR8(_In_ LPDECIMAL pdecLeft, _In_ double dblRight);


// Currency math
//
STDAPI VarCyAdd(_In_ CY cyLeft, _In_ CY cyRight, _Out_ LPCY pcyResult);
STDAPI VarCyMul(_In_ CY cyLeft, _In_ CY cyRight, _Out_ LPCY pcyResult);
STDAPI VarCyMulI4(_In_ CY cyLeft, _In_ LONG lRight, _Out_ LPCY pcyResult);
STDAPI VarCyMulI8(_In_ CY cyLeft, _In_ LONG64 lRight, _Out_ LPCY pcyResult);
STDAPI VarCySub(_In_ CY cyLeft, _In_ CY cyRight, _Out_ LPCY pcyResult);

STDAPI VarCyAbs(_In_ CY cyIn, _Out_ LPCY pcyResult);
STDAPI VarCyFix(_In_ CY cyIn, _Out_ LPCY pcyResult);
STDAPI VarCyInt(_In_ CY cyIn, _Out_ LPCY pcyResult);
STDAPI VarCyNeg(_In_ CY cyIn, _Out_ LPCY pcyResult);

STDAPI VarCyRound(_In_ CY cyIn, _In_ int cDecimals, _Out_ LPCY pcyResult);

STDAPI VarCyCmp(_In_ CY cyLeft, _In_ CY cyRight);
STDAPI VarCyCmpR8(_In_ CY cyLeft, _In_ double dblRight);


// Misc support functions
//
STDAPI VarBstrCat(_In_ BSTR bstrLeft, _In_ BSTR bstrRight, _Out_ LPBSTR pbstrResult);
STDAPI VarBstrCmp(_In_ BSTR bstrLeft, _In_ BSTR bstrRight, _In_ LCID lcid, _In_ ULONG dwFlags); // dwFlags passed to CompareString
STDAPI VarR8Pow(_In_ double dblLeft, _In_ double dblRight, _Out_ double *pdblResult);
STDAPI VarR4CmpR8(_In_ float fltLeft, _In_ double dblRight);
STDAPI VarR8Round(_In_ double dblIn, _In_ int cDecimals, _Out_ double *pdblResult);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

// Compare results.  These are returned as a SUCCESS HResult.  Subtracting
// one gives the usual values of -1 for Less Than, 0 for Equal To, +1 for
// Greater Than.
//
#define VARCMP_LT   0
#define VARCMP_EQ   1
#define VARCMP_GT   2
#define VARCMP_NULL 3

// VT_HARDTYPE tells the compare routine that the argument is a literal or
// otherwise declared of that specific type.  It causes comparison rules to
// change. For example, if a hard-type string is compared to a variant (not hard
// -type) number, the number is converted to string.  If a hard-type number is
// compared to a variant string, the string is converted to number.  If they're
// both variant, then number < string.
#define VT_HARDTYPE VT_RESERVED

/*---------------------------------------------------------------------*/
/*                   New date functions                                */
/*---------------------------------------------------------------------*/

/* The UDATE structure is used with VarDateFromUdate() and VarUdateFromDate().
 * It represents an "unpacked date".
 */
typedef struct {
    SYSTEMTIME st;
    USHORT  wDayOfYear;
} UDATE;

/* APIs to "pack" and "unpack" dates.
 * NOTE: Ex version of VarDateFromUdate obeys 2 digit year setting in
 * control panel.
 */

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

_Check_return_
WINOLEAUTAPI VarDateFromUdate(_In_ UDATE *pudateIn, _In_ ULONG dwFlags, _Out_ DATE *pdateOut);
WINOLEAUTAPI VarDateFromUdateEx(_In_ UDATE *pudateIn, _In_ LCID lcid, _In_ ULONG dwFlags, _Out_ DATE *pdateOut);
_Check_return_
WINOLEAUTAPI VarUdateFromDate(_In_ DATE dateIn, _In_ ULONG dwFlags, _Out_ UDATE *pudateOut);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* API to retrieve the secondary(altername) month names
   Useful for Hijri, Polish and Russian alternate month names
*/
_Check_return_
WINOLEAUTAPI GetAltMonthNames(LCID lcid, _Outptr_result_buffer_maybenull_(13) LPOLESTR * * prgp);

/*---------------------------------------------------------------------*/
/*                 Format                                              */
/*---------------------------------------------------------------------*/

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI VarFormat(
	_In_ LPVARIANT pvarIn,
	_In_opt_ LPOLESTR pstrFormat,
	int iFirstDay,
	int iFirstWeek,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarFormatDateTime(
	_In_ LPVARIANT pvarIn,
	int iNamedFormat,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarFormatNumber(
	_In_ LPVARIANT pvarIn,
	int iNumDig,
	int iIncLead,
	int iUseParens,
	int iGroup,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarFormatPercent(
	_In_ LPVARIANT pvarIn,
	int iNumDig,
	int iIncLead,
	int iUseParens,
	int iGroup,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarFormatCurrency(
	_In_ LPVARIANT pvarIn,
	int iNumDig,
	int iIncLead,
	int iUseParens,
	int iGroup,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarWeekdayName(
	int iWeekday,
	int fAbbrev,
	int iFirstDay,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarMonthName(
	int iMonth,
	int fAbbrev,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut
	);

WINOLEAUTAPI VarFormatFromTokens(
	_In_ LPVARIANT pvarIn,
	_In_opt_ LPOLESTR pstrFormat,
	_In_reads_(_Inexpressible_("from VarTokenizeFormatString")) LPBYTE pbTokCur,
	ULONG dwFlags,
	_Out_ BSTR *pbstrOut,
	LCID lcid
	);

WINOLEAUTAPI VarTokenizeFormatString(
	_In_opt_ LPOLESTR pstrFormat,
	_Inout_updates_(cbTok) LPBYTE rgbTok,
	int cbTok,
	int iFirstDay,
	int iFirstWeek,
	LCID lcid,
	_In_opt_ int *pcbActual
	);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/*---------------------------------------------------------------------*/
/*                 ITypeLib                                            */
/*---------------------------------------------------------------------*/

typedef /* [unique] */  __RPC_unique_pointer ITypeLib *LPTYPELIB;


/*---------------------------------------------------------------------*/
/*                ITypeInfo                                            */
/*---------------------------------------------------------------------*/


typedef LONG DISPID;
typedef DISPID MEMBERID;

#define MEMBERID_NIL DISPID_UNKNOWN
#define ID_DEFAULTINST  -2

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

/* Flags for IDispatch::Invoke */
#define DISPATCH_METHOD         0x1
#define DISPATCH_PROPERTYGET    0x2
#define DISPATCH_PROPERTYPUT    0x4
#define DISPATCH_PROPERTYPUTREF 0x8

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

typedef /* [unique] */  __RPC_unique_pointer ITypeInfo *LPTYPEINFO;


/*---------------------------------------------------------------------*/
/*                ITypeComp                                            */
/*---------------------------------------------------------------------*/

typedef /* [unique] */  __RPC_unique_pointer ITypeComp *LPTYPECOMP;


/*---------------------------------------------------------------------*/
/*             ICreateTypeLib                                          */
/*---------------------------------------------------------------------*/

typedef ICreateTypeLib * LPCREATETYPELIB;

typedef ICreateTypeInfo * LPCREATETYPEINFO;

/*---------------------------------------------------------------------*/
/*             TypeInfo API                                            */
/*---------------------------------------------------------------------*/

/* compute a 16bit hash value for the given name
 */
#if (defined (_WIN32) || defined (_WIN64))
_Check_return_
WINOLEAUTAPI_(ULONG) LHashValOfNameSysA(SYSKIND syskind, LCID lcid,
            LPCSTR szName);
#endif

_Check_return_
WINOLEAUTAPI_(ULONG)
LHashValOfNameSys(SYSKIND syskind, LCID lcid, const OLECHAR * szName);

#define LHashValOfName(lcid, szName) \
            LHashValOfNameSys(SYS_WIN32, lcid, szName)

#define WHashValOfLHashVal(lhashval) \
            ((USHORT) (0x0000ffff & (lhashval)))

#define IsHashValCompatible(lhashval1, lhashval2) \
            ((BOOL) ((0x00ff0000 & (lhashval1)) == (0x00ff0000 & (lhashval2))))

/* load the typelib from the file with the given filename
 */
WINOLEAUTAPI LoadTypeLib(_In_z_ LPCOLESTR szFile, ITypeLib ** pptlib);

/* Control how a type library is registered
 */
typedef enum tagREGKIND
{
    REGKIND_DEFAULT,
    REGKIND_REGISTER,
    REGKIND_NONE
} REGKIND;


// Constants for specifying format in which TLB should be loaded
// (the default format is 32-bit on WIN32 and 64-bit on WIN64)
#define LOAD_TLB_AS_32BIT	0x20
#define LOAD_TLB_AS_64BIT	0x40
#define MASK_TO_RESET_TLB_BITS		~(LOAD_TLB_AS_32BIT | LOAD_TLB_AS_64BIT)

_Check_return_
WINOLEAUTAPI LoadTypeLibEx(LPCOLESTR szFile, REGKIND regkind,
            ITypeLib ** pptlib);

/* load registered typelib
 */
_Check_return_
WINOLEAUTAPI LoadRegTypeLib(REFGUID rguid, WORD wVerMajor, WORD wVerMinor,
            LCID lcid, ITypeLib ** pptlib);

/* get path to registered typelib
 */
WINOLEAUTAPI QueryPathOfRegTypeLib(REFGUID guid, USHORT wMaj, USHORT wMin,
            LCID lcid, _Out_ LPBSTR lpbstrPathName);

/* add typelib to registry
 */
_Check_return_
WINOLEAUTAPI RegisterTypeLib(ITypeLib * ptlib, _In_ LPCOLESTR szFullPath,
            _In_opt_ LPCOLESTR szHelpDir);

/* remove typelib from registry
 */

_Check_return_
WINOLEAUTAPI UnRegisterTypeLib(REFGUID libID, WORD wVerMajor,
            WORD wVerMinor, LCID lcid, SYSKIND syskind);

/* Registers a type library for use by the calling user.
*/
WINOLEAUTAPI RegisterTypeLibForUser(ITypeLib *ptlib,_In_ OLECHAR  *szFullPath,
			_In_opt_ OLECHAR  *szHelpDir);

/* Removes type library information that was registered by using RegisterTypeLibForUser.
*/
WINOLEAUTAPI UnRegisterTypeLibForUser(
    REFGUID         libID,
    WORD   wMajorVerNum,
    WORD   wMinorVerNum,
    LCID            lcid,
    SYSKIND         syskind);

_Check_return_
WINOLEAUTAPI CreateTypeLib(SYSKIND syskind, LPCOLESTR szFile,
            ICreateTypeLib ** ppctlib);

_Check_return_
WINOLEAUTAPI CreateTypeLib2(SYSKIND syskind, LPCOLESTR szFile,
            ICreateTypeLib2 **ppctlib);


/*---------------------------------------------------------------------*/
/*           IDispatch implementation support                          */
/*---------------------------------------------------------------------*/

typedef /* [unique] */  __RPC_unique_pointer IDispatch *LPDISPATCH;

typedef struct tagPARAMDATA {
    OLECHAR * szName;   /* parameter name */
    VARTYPE vt;         /* parameter type */
} PARAMDATA, * LPPARAMDATA;

typedef struct tagMETHODDATA {
    OLECHAR * szName;   /* method name */
    PARAMDATA * ppdata; /* pointer to an array of PARAMDATAs */
    DISPID dispid;      /* method ID */
    UINT iMeth;         /* method index */
    CALLCONV cc;        /* calling convention */
    UINT cArgs;         /* count of arguments */
    WORD wFlags;        /* same wFlags as on IDispatch::Invoke() */
    VARTYPE vtReturn;
} METHODDATA, * LPMETHODDATA;

typedef struct tagINTERFACEDATA {
    METHODDATA * pmethdata;  /* pointer to an array of METHODDATAs */
    UINT cMembers;      /* count of members */
} INTERFACEDATA, * LPINTERFACEDATA;



/* Locate the parameter indicated by the given position, and
 * return it coerced to the given target VARTYPE (vtTarg).
 */
_Check_return_
WINOLEAUTAPI DispGetParam(
	_In_ DISPPARAMS * pdispparams,
	UINT position,
	VARTYPE vtTarg,
	_Out_ VARIANT * pvarResult,
	_Out_opt_ UINT * puArgErr
	);

/* Automatic TypeInfo driven implementation of IDispatch::GetIDsOfNames()
 */
_Check_return_ WINOLEAUTAPI DispGetIDsOfNames(ITypeInfo * ptinfo, _In_reads_(cNames) LPOLESTR* rgszNames,
            UINT cNames, _Out_writes_(cNames) DISPID * rgdispid);

/* Automatic TypeInfo driven implementation of IDispatch::Invoke()
 */
_Check_return_

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Application Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM)

WINOLEAUTAPI DispInvoke(void * _this, ITypeInfo * ptinfo, DISPID dispidMember,
            WORD wFlags, DISPPARAMS * pparams, VARIANT * pvarResult,
            EXCEPINFO * pexcepinfo, UINT * puArgErr);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

/* Construct a TypeInfo from an interface data description
 */
_Check_return_
WINOLEAUTAPI CreateDispTypeInfo(INTERFACEDATA * pidata, LCID lcid,
            ITypeInfo ** pptinfo);

/* Create an instance of the standard TypeInfo driven IDispatch
 * implementation.
 */
_Check_return_
WINOLEAUTAPI CreateStdDispatch(IUnknown * punkOuter, void * pvThis,
            ITypeInfo * ptinfo, IUnknown ** ppunkStdDisp);

/* Low-level helper for IDispatch::Invoke() provides machine independence
 * for customized Invoke().
 */
WINOLEAUTAPI DispCallFunc(_In_opt_ void * pvInstance, ULONG_PTR oVft, CALLCONV cc,
            VARTYPE vtReturn, UINT  cActuals, _In_reads_(cActuals) VARTYPE * prgvt,
            _In_reads_(cActuals) VARIANTARG ** prgpvarg, _Out_ VARIANT * pvargResult);


/*---------------------------------------------------------------------*/
/*            Active Object Registration API                           */
/*---------------------------------------------------------------------*/

/* flags for RegisterActiveObject */
#define ACTIVEOBJECT_STRONG 0x0
#define ACTIVEOBJECT_WEAK 0x1

_Check_return_
WINOLEAUTAPI RegisterActiveObject(IUnknown * punk, REFCLSID rclsid,
            DWORD dwFlags, DWORD * pdwRegister);

WINOLEAUTAPI RevokeActiveObject(DWORD dwRegister, void * pvReserved);

WINOLEAUTAPI GetActiveObject(REFCLSID rclsid, void * pvReserved,
            IUnknown ** ppunk);

/*---------------------------------------------------------------------*/
/*                           ErrorInfo API                             */
/*---------------------------------------------------------------------*/

WINOLEAUTAPI SetErrorInfo(_In_ ULONG dwReserved, _In_opt_ IErrorInfo * perrinfo);
_Check_return_
WINOLEAUTAPI GetErrorInfo(_In_ ULONG dwReserved, _Outptr_ IErrorInfo ** pperrinfo);
_Check_return_
WINOLEAUTAPI CreateErrorInfo(_Outptr_ ICreateErrorInfo ** pperrinfo);

/*---------------------------------------------------------------------*/
/*           User Defined Data types support                           */
/*---------------------------------------------------------------------*/

WINOLEAUTAPI GetRecordInfoFromTypeInfo(ITypeInfo * pTypeInfo,
            IRecordInfo ** ppRecInfo);

WINOLEAUTAPI GetRecordInfoFromGuids(REFGUID rGuidTypeLib,
            ULONG uVerMajor, ULONG uVerMinor, LCID lcid,
            REFGUID rGuidTypeInfo, IRecordInfo ** ppRecInfo);

/*---------------------------------------------------------------------*/
/*                           MISC API                                  */
/*---------------------------------------------------------------------*/

WINOLEAUTAPI_(ULONG) OaBuildVersion(void);

WINOLEAUTAPI_(void) ClearCustData(LPCUSTDATA pCustData);

#if (NTDDI_VERSION >= NTDDI_VISTASP1)
WINOLEAUTAPI_(void) OaEnablePerUserTLibRegistration(void);
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM) */
#pragma endregion

// Declare variant access functions.

#if (__STDC__ && !defined(_FORCENAMELESSUNION)) || defined(NONAMELESSUNION) || (!defined(_MSC_EXTENSIONS) && !defined(_FORCENAMELESSUNION))
#define V_UNION(X, Y)   ((X)->n1.n2.n3.Y)
#define V_VT(X)         ((X)->n1.n2.vt)
#define V_RECORDINFO(X) ((X)->n1.n2.n3.brecVal.pRecInfo)
#define V_RECORD(X)     ((X)->n1.n2.n3.brecVal.pvRecord)
#define V_DECIMAL(X)    ((X)->n1.decVal)
#else
#define V_UNION(X, Y)   ((X)->Y)
#define V_VT(X)         ((X)->vt)
#define V_RECORDINFO(X) ((X)->pRecInfo)
#define V_RECORD(X)     ((X)->pvRecord)
#define V_DECIMAL(X)    ((X)->decVal)
#endif

/* Variant access macros
 */
#define V_ISBYREF(X)     (V_VT(X)&VT_BYREF)
#define V_ISARRAY(X)     (V_VT(X)&VT_ARRAY)
#define V_ISVECTOR(X)    (V_VT(X)&VT_VECTOR)
#define V_NONE(X)        V_I2(X)

#define V_UI1(X)         V_UNION(X, bVal)
#define V_UI1REF(X)      V_UNION(X, pbVal)
#define V_I2(X)          V_UNION(X, iVal)
#define V_I2REF(X)       V_UNION(X, piVal)
#define V_I4(X)          V_UNION(X, lVal)
#define V_I4REF(X)       V_UNION(X, plVal)
#define V_I8(X)          V_UNION(X, llVal)
#define V_I8REF(X)       V_UNION(X, pllVal)
#define V_R4(X)          V_UNION(X, fltVal)
#define V_R4REF(X)       V_UNION(X, pfltVal)
#define V_R8(X)          V_UNION(X, dblVal)
#define V_R8REF(X)       V_UNION(X, pdblVal)
#define V_I1(X)          V_UNION(X, cVal)
#define V_I1REF(X)       V_UNION(X, pcVal)
#define V_UI2(X)         V_UNION(X, uiVal)
#define V_UI2REF(X)      V_UNION(X, puiVal)
#define V_UI4(X)         V_UNION(X, ulVal)
#define V_UI4REF(X)      V_UNION(X, pulVal)
#define V_UI8(X)         V_UNION(X, ullVal)
#define V_UI8REF(X)      V_UNION(X, pullVal)
#define V_INT(X)         V_UNION(X, intVal)
#define V_INTREF(X)      V_UNION(X, pintVal)
#define V_UINT(X)        V_UNION(X, uintVal)
#define V_UINTREF(X)     V_UNION(X, puintVal)

#ifdef _WIN64
#define V_INT_PTR(X)        V_UNION(X, llVal)
#define V_UINT_PTR(X)       V_UNION(X, ullVal)
#define V_INT_PTRREF(X)     V_UNION(X, pllVal)
#define V_UINT_PTRREF(X)    V_UNION(X, pullVal)
#else
#define V_INT_PTR(X)        V_UNION(X, lVal)
#define V_UINT_PTR(X)       V_UNION(X, ulVal)
#define V_INT_PTRREF(X)     V_UNION(X, plVal)
#define V_UINT_PTRREF(X)    V_UNION(X, pulVal)
#endif

#define V_CY(X)          V_UNION(X, cyVal)
#define V_CYREF(X)       V_UNION(X, pcyVal)
#define V_DATE(X)        V_UNION(X, date)
#define V_DATEREF(X)     V_UNION(X, pdate)
#define V_BSTR(X)        V_UNION(X, bstrVal)
#define V_BSTRREF(X)     V_UNION(X, pbstrVal)
#define V_DISPATCH(X)    V_UNION(X, pdispVal)
#define V_DISPATCHREF(X) V_UNION(X, ppdispVal)
#define V_ERROR(X)       V_UNION(X, scode)
#define V_ERRORREF(X)    V_UNION(X, pscode)
#define V_BOOL(X)        V_UNION(X, boolVal)
#define V_BOOLREF(X)     V_UNION(X, pboolVal)
#define V_UNKNOWN(X)     V_UNION(X, punkVal)
#define V_UNKNOWNREF(X)  V_UNION(X, ppunkVal)
#define V_VARIANTREF(X)  V_UNION(X, pvarVal)
#define V_ARRAY(X)       V_UNION(X, parray)
#define V_ARRAYREF(X)    V_UNION(X, pparray)
#define V_BYREF(X)       V_UNION(X, byref)

#define V_DECIMALREF(X)  V_UNION(X, pdecVal)

#ifndef RC_INVOKED
#include <poppack.h>
#endif // RC_INVOKED

#endif     // __OLEAUTO_H__

#if _MSC_VER >= 1200
#pragma warning(pop)
#endif
