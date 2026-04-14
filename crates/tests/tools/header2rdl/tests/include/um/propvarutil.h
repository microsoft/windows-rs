
//===========================================================================
//
// Copyright (c) Microsoft Corporation. All rights reserved.
//
// propvarutil.h - Variant and PropVariant helpers
//
//===========================================================================

#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

typedef _Return_type_success_(return >= 0) LONG NTSTATUS;
#include <propapi.h>

#include <shtypes.h>
#include <shlwapi.h>

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#ifndef PSSTDAPI
#if defined(_PROPSYS_)
#define PSSTDAPI          STDAPI
#define PSSTDAPI_(type)   STDAPI_(type)
#else
#define PSSTDAPI          EXTERN_C DECLSPEC_IMPORT HRESULT STDAPICALLTYPE
#define PSSTDAPI_(type)   EXTERN_C DECLSPEC_IMPORT type STDAPICALLTYPE
#endif
#endif // PSSTDAPI

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

enum tagPSTIME_FLAGS
{
    PSTF_UTC   = 0x00000000,
    PSTF_LOCAL = 0x00000001,
};
typedef int PSTIME_FLAGS;

//====================
//
// PropVariant Helpers
//
//====================

// Initialize a propvariant
PSSTDAPI InitPropVariantFromResource(_In_ HINSTANCE hinst, _In_ UINT id, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromBuffer(_In_reads_bytes_(cb) const void *pv, _In_ UINT cb, _Out_ PROPVARIANT *ppropvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI InitPropVariantFromCLSID(_In_ REFCLSID clsid, _Out_ PROPVARIANT *ppropvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
PSSTDAPI InitPropVariantFromGUIDAsString(_In_ REFGUID guid, _Out_ PROPVARIANT *ppropvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI InitPropVariantFromFileTime(_In_ const FILETIME *pftIn, _Out_ PROPVARIANT *ppropvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
PSSTDAPI InitPropVariantFromPropVariantVectorElem(_In_ REFPROPVARIANT propvarIn, _In_ ULONG iElem, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantVectorFromPropVariant(_In_ REFPROPVARIANT propvarSingle, _Out_ PROPVARIANT *ppropvarVector);
PSSTDAPI InitPropVariantFromStrRet(_Inout_ STRRET *pstrret, _In_opt_ PCUITEMID_CHILD pidl, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromBooleanVector(_In_reads_opt_(cElems) const BOOL *prgf, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromInt16Vector(_In_reads_opt_(cElems) const SHORT *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromUInt16Vector(_In_reads_opt_(cElems) const USHORT *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromInt32Vector(_In_reads_opt_(cElems) const LONG *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromUInt32Vector(_In_reads_opt_(cElems) const ULONG *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromInt64Vector(_In_reads_opt_(cElems) const LONGLONG *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromUInt64Vector(_In_reads_opt_(cElems) const ULONGLONG *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromDoubleVector(_In_reads_opt_(cElems) const DOUBLE *prgn, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromFileTimeVector(_In_reads_opt_(cElems) const FILETIME *prgft, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromStringVector(_In_reads_opt_(cElems) PCWSTR *prgsz, _In_ ULONG cElems, _Out_ PROPVARIANT *ppropvar);
PSSTDAPI InitPropVariantFromStringAsVector(_In_opt_ PCWSTR psz, _Out_ PROPVARIANT *ppropvar);
#ifdef __cplusplus
HRESULT  InitPropVariantFromBoolean(_In_ BOOL fVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromInt16(_In_ SHORT nVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromUInt16(_In_ USHORT uiVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromInt32(_In_ LONG lVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromUInt32(_In_ ULONG ulVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromInt64(_In_ LONGLONG llVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromUInt64(_In_ ULONGLONG ullVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromDouble(_In_ DOUBLE dblVal, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromString(_In_ PCWSTR psz, _Out_ PROPVARIANT *ppropvar);
HRESULT  InitPropVariantFromGUIDAsBuffer(_In_ REFGUID guid, _Out_ PROPVARIANT *ppropvar);
BOOL     IsPropVariantVector(_In_ REFPROPVARIANT propvar);
BOOL     IsPropVariantString(_In_ REFPROPVARIANT propvar);
#endif

// Extract data from a propvariant
PSSTDAPI_(BOOL)      PropVariantToBooleanWithDefault(_In_ REFPROPVARIANT propvarIn, _In_ BOOL fDefault);
PSSTDAPI_(SHORT)     PropVariantToInt16WithDefault(_In_ REFPROPVARIANT propvarIn, _In_ SHORT iDefault);
PSSTDAPI_(USHORT)    PropVariantToUInt16WithDefault(_In_ REFPROPVARIANT propvarIn, _In_ USHORT uiDefault);
PSSTDAPI_(LONG)      PropVariantToInt32WithDefault(_In_ REFPROPVARIANT propvarIn, _In_ LONG lDefault);
PSSTDAPI_(ULONG)     PropVariantToUInt32WithDefault(_In_ REFPROPVARIANT propvarIn, _In_ ULONG ulDefault);
PSSTDAPI_(LONGLONG)  PropVariantToInt64WithDefault(_In_ REFPROPVARIANT propvarIn, _In_ LONGLONG llDefault);
PSSTDAPI_(ULONGLONG) PropVariantToUInt64WithDefault(_In_ REFPROPVARIANT propvarIn, _In_ ULONGLONG ullDefault);
PSSTDAPI_(DOUBLE)    PropVariantToDoubleWithDefault(_In_ REFPROPVARIANT propvarIn, _In_ DOUBLE dblDefault);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI_(PCWSTR)    PropVariantToStringWithDefault(_In_ REFPROPVARIANT propvarIn, _In_opt_ LPCWSTR pszDefault);

PSSTDAPI             PropVariantToBoolean(_In_ REFPROPVARIANT propvarIn, _Out_ BOOL *pfRet);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
PSSTDAPI             PropVariantToInt16(_In_ REFPROPVARIANT propvarIn, _Out_ SHORT *piRet);
PSSTDAPI             PropVariantToUInt16(_In_ REFPROPVARIANT propvarIn, _Out_ USHORT *puiRet);
PSSTDAPI             PropVariantToInt32(_In_ REFPROPVARIANT propvarIn, _Out_ LONG *plRet);
PSSTDAPI             PropVariantToUInt32(_In_ REFPROPVARIANT propvarIn, _Out_ ULONG *pulRet);
PSSTDAPI             PropVariantToInt64(_In_ REFPROPVARIANT propvarIn, _Out_ LONGLONG *pllRet);
PSSTDAPI             PropVariantToUInt64(_In_ REFPROPVARIANT propvarIn, _Out_ ULONGLONG *pullRet);
PSSTDAPI             PropVariantToDouble(_In_ REFPROPVARIANT propvarIn, _Out_ DOUBLE *pdblRet);
PSSTDAPI             PropVariantToBuffer(_In_ REFPROPVARIANT propvar, _Out_writes_bytes_(cb) void *pv, _In_ UINT cb);
PSSTDAPI             PropVariantToString(_In_ REFPROPVARIANT propvar, _Out_writes_(cch) PWSTR psz, _In_ UINT cch);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI             PropVariantToGUID(_In_ REFPROPVARIANT propvar, _Out_ GUID *pguid);
_Check_return_ PSSTDAPI PropVariantToStringAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_nullonfailure_ PWSTR *ppszOut);
_Check_return_ PSSTDAPI PropVariantToBSTR(_In_ REFPROPVARIANT propvar, _Outptr_result_nullonfailure_ BSTR *pbstrOut);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
_Check_return_ PSSTDAPI PropVariantToStrRet(_In_ REFPROPVARIANT propvar, _Out_ STRRET *pstrret);
PSSTDAPI             PropVariantToFileTime(_In_ REFPROPVARIANT propvar, _In_ PSTIME_FLAGS pstfOut, _Out_ FILETIME* pftOut);
#ifdef __cplusplus
HRESULT              PropVariantToCLSID(_In_ REFPROPVARIANT propvar, _Out_ CLSID *pclsid);
#endif

// Returns element count of a VT_VECTOR or VT_ARRAY value; or 1 otherwise
PSSTDAPI_(ULONG) PropVariantGetElementCount(_In_ REFPROPVARIANT propvar);

// Extract data from a propvariant into a vector
PSSTDAPI PropVariantToBooleanVector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgf, *pcElem) BOOL *prgf, _In_ ULONG crgf, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToInt16Vector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) SHORT *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToUInt16Vector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) USHORT *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToInt32Vector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) LONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToUInt32Vector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) ULONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToInt64Vector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) LONGLONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToUInt64Vector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) ULONGLONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToDoubleVector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgn, *pcElem) DOUBLE *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI PropVariantToFileTimeVector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgft, *pcElem) FILETIME *prgft, _In_ ULONG crgft, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToStringVector(_In_ REFPROPVARIANT propvar, _Out_writes_to_(crgsz, *pcElem) PWSTR *prgsz, _In_ ULONG crgsz, _Out_ ULONG *pcElem);


// Extract data from a propvariant and return an newly allocated vector (free with CoTaskMemFree)
_Check_return_ PSSTDAPI PropVariantToBooleanVectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) BOOL **pprgf, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToInt16VectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) SHORT **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToUInt16VectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) USHORT **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToInt32VectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) LONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToUInt32VectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) ULONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToInt64VectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) LONGLONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToUInt64VectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) ULONGLONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToDoubleVectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) DOUBLE **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToFileTimeVectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) FILETIME **pprgft, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI PropVariantToStringVectorAlloc(_In_ REFPROPVARIANT propvar, _Outptr_result_buffer_(*pcElem) PWSTR **pprgsz, _Out_ ULONG *pcElem);

// Extract a single element from a propvariant.  If it is a VT_VECTOR or VT_ARRAY, returns the element you request.
// Otherwise iElem must equal 0 and the function will returns the value.
PSSTDAPI PropVariantGetBooleanElem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ BOOL *pfVal);
PSSTDAPI PropVariantGetInt16Elem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ SHORT *pnVal);
PSSTDAPI PropVariantGetUInt16Elem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ USHORT *pnVal);
PSSTDAPI PropVariantGetInt32Elem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ LONG *pnVal);
PSSTDAPI PropVariantGetUInt32Elem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ ULONG *pnVal);
PSSTDAPI PropVariantGetInt64Elem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ LONGLONG *pnVal);
PSSTDAPI PropVariantGetUInt64Elem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ ULONGLONG *pnVal);
PSSTDAPI PropVariantGetDoubleElem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ DOUBLE *pnVal);
PSSTDAPI PropVariantGetFileTimeElem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Out_ FILETIME *pftVal);
_Check_return_ PSSTDAPI PropVariantGetStringElem(_In_ REFPROPVARIANT propvar, _In_ ULONG iElem, _Outptr_ PWSTR *ppszVal);
#ifdef __cplusplus
HRESULT  PropVariantGetElem(_In_ REFPROPVARIANT propvarIn, _In_ ULONG iElem, _Out_ PROPVARIANT *ppropvar);
#endif

// Helpers
PSSTDAPI_(void) ClearPropVariantArray(_Inout_updates_(cVars) PROPVARIANT *rgPropVar, _In_ UINT cVars);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

typedef enum 
{
    PVCU_DEFAULT = 0,
    PVCU_SECOND  = 1,
    PVCU_MINUTE  = 2,
    PVCU_HOUR    = 3,
    PVCU_DAY     = 4,
    PVCU_MONTH   = 5,
    PVCU_YEAR    = 6
} PROPVAR_COMPARE_UNIT;

enum tagPROPVAR_COMPARE_FLAGS
{
    PVCF_DEFAULT                 = 0x00000000,   // When comparing strings, use StrCmpLogical
    PVCF_TREATEMPTYASGREATERTHAN = 0x00000001,   // Empty/null values are greater-than non-empty values
    PVCF_USESTRCMP               = 0x00000002,   // When comparing strings, use StrCmp
    PVCF_USESTRCMPC              = 0x00000004,   // When comparing strings, use StrCmpC
    PVCF_USESTRCMPI              = 0x00000008,   // When comparing strings, use StrCmpI
    PVCF_USESTRCMPIC             = 0x00000010,   // When comparing strings, use StrCmpIC
    PVCF_DIGITSASNUMBERS_CASESENSITIVE = 0x00000020,   // When comparing strings, use CompareStringEx with LOCALE_NAME_USER_DEFAULT and SORT_DIGITSASNUMBERS.  This corresponds to the linguistically correct order for UI lists.
};
typedef int PROPVAR_COMPARE_FLAGS;

// Comparisons
PSSTDAPI_(int) PropVariantCompareEx(_In_ REFPROPVARIANT propvar1, _In_ REFPROPVARIANT propvar2, _In_ PROPVAR_COMPARE_UNIT unit, _In_ PROPVAR_COMPARE_FLAGS flags);
#ifdef __cplusplus
int PropVariantCompare(_In_ REFPROPVARIANT propvar1, _In_ REFPROPVARIANT propvar2);
#endif

enum tagPROPVAR_CHANGE_FLAGS
{
    PVCHF_DEFAULT           = 0x00000000,
    PVCHF_NOVALUEPROP       = 0x00000001,       // Maps to VARIANT_NOVALUEPROP for VariantChangeType
    PVCHF_ALPHABOOL         = 0x00000002,       // Maps to VARIANT_ALPHABOOL for VariantChangeType
    PVCHF_NOUSEROVERRIDE    = 0x00000004,       // Maps to VARIANT_NOUSEROVERRIDE for VariantChangeType
    PVCHF_LOCALBOOL         = 0x00000008,       // Maps to VARIANT_LOCALBOOL for VariantChangeType
    PVCHF_NOHEXSTRING       = 0x00000010,       // Don't convert a string that looks like hexadecimal (0xABCD) to the numerical equivalent
};
typedef int PROPVAR_CHANGE_FLAGS;

// Coersions
PSSTDAPI PropVariantChangeType(_Out_ PROPVARIANT *ppropvarDest, _In_ REFPROPVARIANT propvarSrc, _In_ PROPVAR_CHANGE_FLAGS flags, _In_ VARTYPE vt);

// Conversions
PSSTDAPI PropVariantToVariant(_In_ const PROPVARIANT *pPropVar, _Out_ VARIANT *pVar);
PSSTDAPI VariantToPropVariant(_In_ const VARIANT* pVar, _Out_ PROPVARIANT* pPropVar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// Stg functions
_Check_return_ PSSTDAPI StgSerializePropVariant(
            _In_ const PROPVARIANT* ppropvar,
            _Outptr_result_bytebuffer_(*pcb) SERIALIZEDPROPERTYVALUE** ppProp,
            _Out_ ULONG* pcb);
    
PSSTDAPI StgDeserializePropVariant(
            _In_ const SERIALIZEDPROPERTYVALUE* pprop,
            _In_ ULONG cbMax,
            _Out_ PROPVARIANT* ppropvar);



//================
//
// Variant Helpers
//
//================

#ifdef __cplusplus
BOOL IsVarTypeFloat(_In_ VARTYPE vt);
BOOL IsVariantArray(_In_ REFVARIANT var);
BOOL IsVariantString(_In_ REFVARIANT var);
BOOL IsVarTypeNumber(_In_ VARTYPE vt);
BOOL IsVarTypeInteger(_In_ VARTYPE vt);
#endif

// Initialize a VARIANT
PSSTDAPI InitVariantFromResource(_In_ HINSTANCE hinst, _In_ UINT id, _Out_ VARIANT *pvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI InitVariantFromBuffer(_In_reads_bytes_(cb) const void *pv, _In_ UINT cb, _Out_ VARIANT *pvar);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
PSSTDAPI InitVariantFromGUIDAsString(_In_ REFGUID guid, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromFileTime(_In_ const FILETIME *pft, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromFileTimeArray(_In_reads_opt_(cElems) const FILETIME *prgft, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromStrRet(_In_ STRRET *pstrret, _In_ PCUITEMID_CHILD pidl, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromVariantArrayElem(_In_ REFVARIANT varIn, _In_ ULONG iElem, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromBooleanArray(_In_reads_(cElems) const BOOL *prgf, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromInt16Array(_In_reads_(cElems) const SHORT *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromUInt16Array(_In_reads_(cElems) const USHORT *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromInt32Array(_In_reads_(cElems) const LONG *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromUInt32Array(_In_reads_(cElems) const ULONG *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromInt64Array(_In_reads_(cElems) const LONGLONG *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromUInt64Array(_In_reads_(cElems) const ULONGLONG *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromDoubleArray(_In_reads_(cElems) const DOUBLE *prgn, _In_ ULONG cElems, _Out_ VARIANT *pvar);
PSSTDAPI InitVariantFromStringArray(_In_reads_(cElems) PCWSTR *prgsz, _In_ ULONG cElems, _Out_ VARIANT *pvar);
#ifdef __cplusplus
HRESULT  InitVariantFromBoolean(_In_ BOOL fVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromInt16(_In_ SHORT iVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromUInt16(_In_ USHORT uiVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromInt32(_In_ LONG lVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromUInt32(_In_ ULONG ulVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromInt64(_In_ LONGLONG llVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromUInt64(_In_ ULONGLONG ullVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromDouble(_In_ DOUBLE dblVal, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromString(_In_ PCWSTR psz, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromDispatch(_In_opt_ IDispatch* pdisp, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromDosDateTime(_In_ WORD wDate, _In_ WORD wTime, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromGUIDAsBuffer(_In_ REFGUID guid, _Out_ VARIANT *pvar);
HRESULT  InitVariantFromUnknown(_In_opt_ IUnknown *unknown, _Out_ VARIANT* variant);
#endif

// Extract data from a VARIANT
PSSTDAPI_(BOOL)       VariantToBooleanWithDefault(_In_ REFVARIANT varIn, _In_ BOOL fDefault);
PSSTDAPI_(SHORT)      VariantToInt16WithDefault(_In_ REFVARIANT varIn, _In_ SHORT iDefault);
PSSTDAPI_(USHORT)     VariantToUInt16WithDefault(_In_ REFVARIANT varIn, _In_ USHORT uiDefault);
PSSTDAPI_(LONG)       VariantToInt32WithDefault(_In_ REFVARIANT varIn, _In_ LONG lDefault);
PSSTDAPI_(ULONG)      VariantToUInt32WithDefault(_In_ REFVARIANT varIn, _In_ ULONG ulDefault);
PSSTDAPI_(LONGLONG)   VariantToInt64WithDefault(_In_ REFVARIANT varIn, _In_ LONGLONG llDefault);
PSSTDAPI_(ULONGLONG)  VariantToUInt64WithDefault(_In_ REFVARIANT varIn, _In_ ULONGLONG ullDefault);
PSSTDAPI_(DOUBLE)     VariantToDoubleWithDefault(_In_ REFVARIANT varIn, _In_ DOUBLE dblDefault);
PSSTDAPI_(PCWSTR)     VariantToStringWithDefault(_In_ REFVARIANT varIn, _In_opt_ LPCWSTR pszDefault);

PSSTDAPI              VariantToBoolean(_In_ REFVARIANT varIn, _Out_ BOOL *pfRet);
PSSTDAPI              VariantToInt16(_In_ REFVARIANT varIn, _Out_ SHORT *piRet);
PSSTDAPI              VariantToUInt16(_In_ REFVARIANT varIn, _Out_ USHORT *puiRet);
PSSTDAPI              VariantToInt32(_In_ REFVARIANT varIn, _Out_ LONG *plRet);
PSSTDAPI              VariantToUInt32(_In_ REFVARIANT varIn, _Out_ ULONG *pulRet);
PSSTDAPI              VariantToInt64(_In_ REFVARIANT varIn, _Out_ LONGLONG *pllRet);
PSSTDAPI              VariantToUInt64(_In_ REFVARIANT varIn, _Out_ ULONGLONG *pullRet);
PSSTDAPI              VariantToDouble(_In_ REFVARIANT varIn, _Out_ DOUBLE *pdblRet);
PSSTDAPI              VariantToBuffer(_In_ REFVARIANT varIn, _Out_writes_bytes_(cb) void *pv, _In_ UINT cb);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
PSSTDAPI              VariantToGUID(_In_ REFVARIANT varIn, _Out_ GUID *pguid);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
PSSTDAPI              VariantToString(_In_ REFVARIANT varIn, _Out_writes_(cchBuf) PWSTR pszBuf, _In_ UINT cchBuf);
_Check_return_ PSSTDAPI VariantToStringAlloc(_In_ REFVARIANT varIn, _Outptr_result_nullonfailure_ PWSTR *ppszBuf);
PSSTDAPI              VariantToDosDateTime(_In_ REFVARIANT varIn, _Out_ WORD *pwDate, _Out_ WORD *pwTime);
_Check_return_ PSSTDAPI VariantToStrRet(_In_ REFVARIANT varIn, _Out_ STRRET *pstrret);
PSSTDAPI              VariantToFileTime(_In_ REFVARIANT varIn, _In_ PSTIME_FLAGS stfOut, _Out_ FILETIME* pftOut);

// Get the element count.  Returns number of elements for values of type VT_ARRAY; returns 1 otherwise.
PSSTDAPI_(ULONG) VariantGetElementCount(_In_ REFVARIANT varIn);

// Extract data from a VARIANT into a vector
PSSTDAPI VariantToBooleanArray(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) BOOL *prgf, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToInt16Array(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) SHORT *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToUInt16Array(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) USHORT *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToInt32Array(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) LONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToUInt32Array(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) ULONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToInt64Array(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) LONGLONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToUInt64Array(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) ULONGLONG *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
PSSTDAPI VariantToDoubleArray(_In_ REFVARIANT var, _Out_writes_to_(crgn, *pcElem) DOUBLE *prgn, _In_ ULONG crgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToStringArray(_In_ REFVARIANT var, _Out_writes_to_(crgsz, *pcElem) PWSTR *prgsz, _In_ ULONG crgsz, _Out_ ULONG *pcElem);

// Extract data from a VARIANT into a newly allocated vector (free with CoTaskMemFree)
_Check_return_ PSSTDAPI VariantToBooleanArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) BOOL **pprgf, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToInt16ArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) SHORT **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToUInt16ArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) USHORT **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToInt32ArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) LONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToUInt32ArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) ULONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToInt64ArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) LONGLONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToUInt64ArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) ULONGLONG **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToDoubleArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) DOUBLE **pprgn, _Out_ ULONG *pcElem);
_Check_return_ PSSTDAPI VariantToStringArrayAlloc(_In_ REFVARIANT var, _Outptr_result_buffer_(*pcElem) PWSTR **pprgsz, _Out_ ULONG *pcElem);

// Get a single element of a VARIANT.  If it is type VT_ARRAY, returns a the requested element.  Otherwise
// iElem must equal 0 and the function returns the value.
PSSTDAPI VariantGetBooleanElem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ BOOL *pfVal);
PSSTDAPI VariantGetInt16Elem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ SHORT *pnVal);
PSSTDAPI VariantGetUInt16Elem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ USHORT *pnVal);
PSSTDAPI VariantGetInt32Elem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ LONG *pnVal);
PSSTDAPI VariantGetUInt32Elem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ ULONG *pnVal);
PSSTDAPI VariantGetInt64Elem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ LONGLONG *pnVal);
PSSTDAPI VariantGetUInt64Elem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ ULONGLONG *pnVal);
PSSTDAPI VariantGetDoubleElem(_In_ REFVARIANT var, _In_ ULONG iElem, _Out_ DOUBLE *pnVal);
_Check_return_ PSSTDAPI VariantGetStringElem(_In_ REFVARIANT var, _In_ ULONG iElem, _Outptr_ PWSTR *ppszVal);
#ifdef __cplusplus
HRESULT  VariantGetElem(_In_ REFVARIANT varIn, _In_ ULONG iElem, _Out_ VARIANT *pvar);
#endif

// Helpers
PSSTDAPI_(void) ClearVariantArray(_Inout_updates_(cvars) VARIANT *pvars, _In_ UINT cvars);
PSSTDAPI_(int) VariantCompare(_In_ REFVARIANT var1, _In_ REFVARIANT var2);

//===========================
//
// Property-specific notions
//
//===========================


// The progress bar property control uses a specially formatted PROPVARIANT to convey the look of the progress bar
// propvar.vt = VT_UI4
// propvar.caul.pElems[0] = current progress
// propvar.caul.pElems[1] = total progress
// propvar.caul.pElems[2] = DRAWPROGRESSFLAGS (see below);
typedef enum DRAWPROGRESSFLAGS
{
    DPF_NONE                = 0x0,  // No progress flags.
    DPF_MARQUEE             = 0x1,  // The progress bar should draw in marquee mode.
    DPF_MARQUEE_COMPLETE    = 0x2,  // The marquee format progress bar has completed.
    DPF_ERROR               = 0x4,  // The progress bar should be drawn in the error state.
    DPF_WARNING             = 0x8,  // The progress bar should be drawn in the warning state.
    DPF_STOPPED             = 0x10, // The progress bar is stopped.
} DRAWPROGRESSFLAGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

//================
//
// Inline Helpers
//
//================

#if defined(__cplusplus) && !defined(CINTERFACE)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

inline HRESULT InitPropVariantFromBoolean(_In_ BOOL fVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_BOOL;
    V_UNION(ppropvar, boolVal) = fVal ? VARIANT_TRUE : VARIANT_FALSE;
    return S_OK;
}

inline HRESULT InitPropVariantFromInt16(_In_ SHORT nVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_I2;
    V_UNION(ppropvar, iVal) = nVal;
    return S_OK;
}

inline HRESULT InitPropVariantFromUInt16(_In_ USHORT uiVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_UI2;
    V_UNION(ppropvar, uiVal) = uiVal;
    return S_OK;
}

inline HRESULT InitPropVariantFromInt32(_In_ LONG lVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_I4;
    V_UNION(ppropvar, lVal) = lVal;
    return S_OK;
}

inline HRESULT InitPropVariantFromUInt32(_In_ ULONG ulVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_UI4;
    V_UNION(ppropvar, ulVal) = ulVal;
    return S_OK;
}

inline HRESULT InitPropVariantFromInt64(_In_ LONGLONG llVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_I8;
    V_UNION(ppropvar, hVal).QuadPart = llVal;
    return S_OK;
}

inline HRESULT InitPropVariantFromUInt64(_In_ ULONGLONG ullVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_UI8;
    V_UNION(ppropvar, uhVal).QuadPart = ullVal;
    return S_OK;
}

inline HRESULT InitPropVariantFromDouble(_In_ DOUBLE dblVal, _Out_ PROPVARIANT *ppropvar)
{
    V_VT(ppropvar) = VT_R8;
    V_UNION(ppropvar, dblVal) = dblVal;
    return S_OK;
}

// Creates a VT_LPWSTR propvariant.
inline HRESULT InitPropVariantFromString(_In_ PCWSTR psz, _Out_ PROPVARIANT *ppropvar)
{
    HRESULT hr = psz != nullptr ? S_OK : E_INVALIDARG; // Previous API behavior counter to the SAL requirement.
    if (SUCCEEDED(hr))
    {
        SIZE_T const byteCount = static_cast<SIZE_T>((wcslen(psz) + 1) * sizeof(*psz));
        V_UNION(ppropvar, pwszVal) = static_cast<PWSTR>(CoTaskMemAlloc(byteCount));
        hr = V_UNION(ppropvar, pwszVal) ? S_OK : E_OUTOFMEMORY;
        if (SUCCEEDED(hr))
        {
            memcpy_s(V_UNION(ppropvar, pwszVal), byteCount, psz, byteCount);
            V_VT(ppropvar) = VT_LPWSTR;
        }
    }
    if (FAILED(hr))
    {
        PropVariantInit(ppropvar);
    }
    return hr;
}

// Creates a VT_VECTOR | VT_UI1 propvariant.
inline HRESULT InitPropVariantFromGUIDAsBuffer(_In_ REFGUID guid, _Out_ PROPVARIANT *ppropvar)
{
    return InitPropVariantFromBuffer(&guid, sizeof(GUID), ppropvar);
}

inline BOOL IsPropVariantVector(_In_ REFPROPVARIANT propvar)
{
    return (propvar.vt & (VT_ARRAY | VT_VECTOR));
}

// If TRUE, propvar contains a unicode string.  Use PropVariantToStringWithDefault(propvar, NULL) to retrieve it.
inline BOOL IsPropVariantString(_In_ REFPROPVARIANT propvar)
{
    return (PropVariantToStringWithDefault(propvar, NULL) != NULL);
}

// Handles int instead of LONG
inline HRESULT PropVariantToInt32(_In_ REFPROPVARIANT propvarIn, _Out_ int *piRet)
{
    return PropVariantToInt32(propvarIn, (LONG*)piRet);
}

// Handles UINT instead of ULONG
inline HRESULT PropVariantToUInt32(_In_ REFPROPVARIANT propvarIn, _Out_ UINT *piRet)
{
    return PropVariantToUInt32(propvarIn, (ULONG*)piRet);
}

inline HRESULT PropVariantToCLSID(_In_ REFPROPVARIANT propvarIn, _Out_ CLSID *pclsid)
{ 
    return PropVariantToGUID(propvarIn, (GUID*)pclsid);
}  


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

inline int PropVariantCompare(_In_ REFPROPVARIANT propvar1, _In_ REFPROPVARIANT propvar2)
{
    return PropVariantCompareEx(propvar1, propvar2, PVCU_DEFAULT, PVCF_DEFAULT);
}

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

inline HRESULT PropVariantGetElem(_In_ REFPROPVARIANT propvarIn, _In_ ULONG iElem, _Out_ PROPVARIANT *ppropvar)
{
    return InitPropVariantFromPropVariantVectorElem(propvarIn, iElem, ppropvar);
}

inline HRESULT InitVariantFromBoolean(_In_ BOOL fVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_BOOL;
    V_UNION(pvar, boolVal) = fVal ? VARIANT_TRUE : VARIANT_FALSE;
    return S_OK;
}

inline HRESULT InitVariantFromInt16(_In_ short iVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_I2;
    V_UNION(pvar, iVal) = iVal;
    return S_OK;
}

inline HRESULT InitVariantFromUInt16(_In_ USHORT uiVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_UI2;
    V_UNION(pvar, uiVal) = uiVal;
    return S_OK;
}

inline HRESULT InitVariantFromInt32(_In_ LONG lVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_I4;
    V_UNION(pvar, lVal) = lVal;
    return S_OK;
}

inline HRESULT InitVariantFromUInt32(_In_ ULONG ulVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_UI4;
    V_UNION(pvar, ulVal) = ulVal;
    return S_OK;
}

inline HRESULT InitVariantFromInt64(_In_ LONGLONG llVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_I8;
    V_UNION(pvar, llVal) = llVal;
    return S_OK;
}

inline HRESULT InitVariantFromUInt64(_In_ ULONGLONG ullVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_UI8;
    V_UNION(pvar, ullVal) = ullVal;
    return S_OK;
}

inline HRESULT InitVariantFromDouble(_In_ DOUBLE dblVal, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_R8;
    V_UNION(pvar, dblVal) = dblVal;
    return S_OK;
}

inline HRESULT InitVariantFromString(_In_ PCWSTR psz, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_BSTR;
    V_UNION(pvar, bstrVal) = SysAllocString(psz);
    HRESULT hr =  V_UNION(pvar, bstrVal) ? S_OK : (psz ? E_OUTOFMEMORY : E_INVALIDARG);
    if (FAILED(hr))
    {
        VariantInit(pvar);
    }
    return hr;
}

inline HRESULT InitVariantFromDispatch(_In_opt_ IDispatch* pdisp, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_DISPATCH;
    V_UNION(pvar, pdispVal) = pdisp;
    if (V_UNION(pvar, pdispVal))
    {
        (V_UNION(pvar, pdispVal))->AddRef();
    }
    return S_OK;
}

// Creates a VT_DATE variant
inline HRESULT InitVariantFromDosDateTime(_In_ WORD wDate, _In_ WORD wTime, _Out_ VARIANT *pvar)
{
    V_VT(pvar) = VT_DATE;
    return DosDateTimeToVariantTime(wDate, wTime, &V_UNION(pvar, date)) ? S_OK : S_FALSE;
}

inline HRESULT InitVariantFromUnknown(_In_opt_ IUnknown *unknown, _Out_ VARIANT* variant)
{
    VariantInit(variant);
    V_VT(variant) = VT_UNKNOWN;
    V_UNION(variant, punkVal) = unknown;
    if (V_UNION(variant, punkVal))
    {
        V_UNION(variant, punkVal)->AddRef();
    }
    return S_OK;
}

inline BOOL IsVarTypeFloat(_In_ VARTYPE vt)
{
    return (vt == VT_R4 || vt == VT_R8);
}

inline BOOL IsVariantArray(_In_ REFVARIANT var)
{
    return (var.vt & VT_ARRAY);
}

// if TRUE, you can use VariantToStringCast to obtain the string pointer
inline BOOL IsVariantString(_In_ REFVARIANT var)
{
    return (VariantToStringWithDefault(var, NULL) != NULL);
}

inline BOOL IsVarTypeNumber(_In_ VARTYPE vt)
{
    return IsVarTypeInteger(vt) || IsVarTypeFloat(vt);
}

inline BOOL IsVarTypeSignedInteger(_In_ VARTYPE vt)
{
    BOOL fRet = FALSE;
    switch (vt)
    {
        case VT_I1:
        case VT_I2:
        case VT_I4:
        case VT_I8:
        fRet = TRUE;
    }
    return fRet;
}

inline BOOL IsVarTypeUnsignedInteger(_In_ VARTYPE vt)
{
    BOOL fRet = FALSE;
    switch (vt)
    {
        case VT_UI1:
        case VT_UI2:
        case VT_UI4:
        case VT_UI8:
        fRet = TRUE;
    }
    return fRet;
}

inline BOOL IsVarTypeInteger(_In_ VARTYPE vt)
{
    return IsVarTypeSignedInteger(vt) || IsVarTypeUnsignedInteger(vt);
}

// Creates a VT_ARRAY | VT_UI1 variant.
inline HRESULT InitVariantFromGUIDAsBuffer(_In_ REFGUID guid, _Out_ VARIANT *pvar)
{
    return InitVariantFromBuffer(&guid, sizeof(GUID), pvar);
}

// Handles int instead of LONG
inline HRESULT VariantToInt32(_In_ REFVARIANT varIn, _Out_ int *piRet)
{
    return VariantToInt32(varIn, (LONG*)piRet);
}

// Handles UINT instead of ULONG
inline HRESULT VariantToUInt32(_In_ REFVARIANT varIn, _Out_ UINT *piRet)
{
    return VariantToUInt32(varIn, (ULONG*)piRet);
}

inline HRESULT VariantGetElem(_In_ REFVARIANT varIn, _In_ ULONG iElem, _Out_ VARIANT *pvar)
{
    return InitVariantFromVariantArrayElem(varIn, iElem, pvar);
}

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // __cplusplus && !CINTERFACE

