

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 501
#endif

/* verify that the <rpcsal.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCSAL_H_VERSION__
#define __REQUIRED_RPCSAL_H_VERSION__ 100
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __inputscope_h__
#define __inputscope_h__

#if defined(_MSC_VER) && (_MSC_VER >= 1020)
#pragma once
#endif

#ifndef DECLSPEC_XFGVIRT
#if defined(_CONTROL_FLOW_GUARD_XFG)
#define DECLSPEC_XFGVIRT(base, func) __declspec(xfg_virtual(base, func))
#else
#define DECLSPEC_XFGVIRT(base, func)
#endif
#endif

/* Forward Declarations */ 

#ifndef __ITfInputScope_FWD_DEFINED__
#define __ITfInputScope_FWD_DEFINED__
typedef interface ITfInputScope ITfInputScope;

#endif 	/* __ITfInputScope_FWD_DEFINED__ */


#ifndef __ITfInputScope2_FWD_DEFINED__
#define __ITfInputScope2_FWD_DEFINED__
typedef interface ITfInputScope2 ITfInputScope2;

#endif 	/* __ITfInputScope2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_inputscope_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//=--------------------------------------------------------------------------=
// InputScope.h


// InputScope declarations.

//=--------------------------------------------------------------------------=
// (C) Copyright 1995-2003 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR TFPLIED, INCLUDING BUT NOT LIMITED TO
// THE TFPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#ifndef INPUTSCOPE_DEFINED
#define INPUTSCOPE_DEFINED

#include <windows.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_inputscope_0000_0000_0001
    {
        IS_DEFAULT	= 0,
        IS_URL	= 1,
        IS_FILE_FULLFILEPATH	= 2,
        IS_FILE_FILENAME	= 3,
        IS_EMAIL_USERNAME	= 4,
        IS_EMAIL_SMTPEMAILADDRESS	= 5,
        IS_LOGINNAME	= 6,
        IS_PERSONALNAME_FULLNAME	= 7,
        IS_PERSONALNAME_PREFIX	= 8,
        IS_PERSONALNAME_GIVENNAME	= 9,
        IS_PERSONALNAME_MIDDLENAME	= 10,
        IS_PERSONALNAME_SURNAME	= 11,
        IS_PERSONALNAME_SUFFIX	= 12,
        IS_ADDRESS_FULLPOSTALADDRESS	= 13,
        IS_ADDRESS_POSTALCODE	= 14,
        IS_ADDRESS_STREET	= 15,
        IS_ADDRESS_STATEORPROVINCE	= 16,
        IS_ADDRESS_CITY	= 17,
        IS_ADDRESS_COUNTRYNAME	= 18,
        IS_ADDRESS_COUNTRYSHORTNAME	= 19,
        IS_CURRENCY_AMOUNTANDSYMBOL	= 20,
        IS_CURRENCY_AMOUNT	= 21,
        IS_DATE_FULLDATE	= 22,
        IS_DATE_MONTH	= 23,
        IS_DATE_DAY	= 24,
        IS_DATE_YEAR	= 25,
        IS_DATE_MONTHNAME	= 26,
        IS_DATE_DAYNAME	= 27,
        IS_DIGITS	= 28,
        IS_NUMBER	= 29,
        IS_ONECHAR	= 30,
        IS_PASSWORD	= 31,
        IS_TELEPHONE_FULLTELEPHONENUMBER	= 32,
        IS_TELEPHONE_COUNTRYCODE	= 33,
        IS_TELEPHONE_AREACODE	= 34,
        IS_TELEPHONE_LOCALNUMBER	= 35,
        IS_TIME_FULLTIME	= 36,
        IS_TIME_HOUR	= 37,
        IS_TIME_MINORSEC	= 38,
        IS_NUMBER_FULLWIDTH	= 39,
        IS_ALPHANUMERIC_HALFWIDTH	= 40,
        IS_ALPHANUMERIC_FULLWIDTH	= 41,
        IS_CURRENCY_CHINESE	= 42,
        IS_BOPOMOFO	= 43,
        IS_HIRAGANA	= 44,
        IS_KATAKANA_HALFWIDTH	= 45,
        IS_KATAKANA_FULLWIDTH	= 46,
        IS_HANJA	= 47,
        IS_HANGUL_HALFWIDTH	= 48,
        IS_HANGUL_FULLWIDTH	= 49,
        IS_SEARCH	= 50,
        IS_FORMULA	= 51,
        IS_SEARCH_INCREMENTAL	= 52,
        IS_CHINESE_HALFWIDTH	= 53,
        IS_CHINESE_FULLWIDTH	= 54,
        IS_NATIVE_SCRIPT	= 55,
        IS_YOMI	= 56,
        IS_TEXT	= 57,
        IS_CHAT	= 58,
        IS_NAME_OR_PHONENUMBER	= 59,
        IS_EMAILNAME_OR_ADDRESS	= 60,
        IS_PRIVATE	= 61,
        IS_MAPS	= 62,
        IS_NUMERIC_PASSWORD	= 63,
        IS_NUMERIC_PIN	= 64,
        IS_ALPHANUMERIC_PIN	= 65,
        IS_ALPHANUMERIC_PIN_SET	= 66,
        IS_FORMULA_NUMBER	= 67,
        IS_CHAT_WITHOUT_EMOJI	= 68,
        IS_PHRASELIST	= -1,
        IS_REGULAREXPRESSION	= -2,
        IS_SRGS	= -3,
        IS_XML	= -4,
        IS_ENUMSTRING	= -5
    } 	InputScope;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma endregion
#pragma region Desktop Family
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
HRESULT WINAPI SetInputScope(HWND hwnd, InputScope inputscope);
HRESULT WINAPI SetInputScopes(HWND hwnd, const InputScope *pInputScopes, UINT cInputScopes, _In_reads_(cPhrases) PWSTR *ppszPhraseList, UINT cPhrases, _In_opt_ PWSTR pszRegExp, _In_opt_ PWSTR pszSRGS);
HRESULT WINAPI SetInputScopeXML(HWND hwnd, _In_opt_ PWSTR pszXML);
HRESULT WINAPI SetInputScopes2(HWND hwnd, const InputScope *pInputScopes, UINT cInputScopes, IEnumString *pEnumString, _In_opt_ PWSTR pszRegExp, _In_opt_ PWSTR pszSRGS);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma endregion
#pragma region Application Family
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
DEFINE_GUID(IID_ITfInputScope, 0xfde1eaee, 0x6924, 0x4cdf, 0x91, 0xe7, 0xda, 0x38, 0xcf, 0xf5, 0x55, 0x9d);
DEFINE_GUID(IID_ITfInputScope2, 0x5731eaa0, 0x6bc2, 0x4681, 0xa5, 0x32, 0x92, 0xfb, 0xb7, 0x4d, 0x7c, 0x41);
DEFINE_GUID(GUID_PROP_INPUTSCOPE, 0x1713dd5a, 0x68e7, 0x4a5b, 0x9a, 0xf6, 0x59, 0x2a, 0x59, 0x5c, 0x77, 0x8d);
#ifdef __cplusplus
}
#endif  /* __cplusplus */


extern RPC_IF_HANDLE __MIDL_itf_inputscope_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inputscope_0000_0000_v0_0_s_ifspec;

#ifndef __ITfInputScope_INTERFACE_DEFINED__
#define __ITfInputScope_INTERFACE_DEFINED__

/* interface ITfInputScope */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputScope;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fde1eaee-6924-4cdf-91e7-da38cff5559d")
    ITfInputScope : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInputScopes( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCount) InputScope **pprgInputScopes,
            /* [out] */ __RPC__out UINT *pcCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPhrase( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCount) BSTR **ppbstrPhrases,
            /* [out] */ __RPC__out UINT *pcCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegularExpression( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRegExp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSRGS( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSRGS) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXML( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXML) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputScopeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputScope * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputScope * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputScope * This);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetInputScopes)
        HRESULT ( STDMETHODCALLTYPE *GetInputScopes )( 
            __RPC__in ITfInputScope * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCount) InputScope **pprgInputScopes,
            /* [out] */ __RPC__out UINT *pcCount);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetPhrase)
        HRESULT ( STDMETHODCALLTYPE *GetPhrase )( 
            __RPC__in ITfInputScope * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCount) BSTR **ppbstrPhrases,
            /* [out] */ __RPC__out UINT *pcCount);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetRegularExpression)
        HRESULT ( STDMETHODCALLTYPE *GetRegularExpression )( 
            __RPC__in ITfInputScope * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRegExp);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetSRGS)
        HRESULT ( STDMETHODCALLTYPE *GetSRGS )( 
            __RPC__in ITfInputScope * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSRGS);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetXML)
        HRESULT ( STDMETHODCALLTYPE *GetXML )( 
            __RPC__in ITfInputScope * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXML);
        
        END_INTERFACE
    } ITfInputScopeVtbl;

    interface ITfInputScope
    {
        CONST_VTBL struct ITfInputScopeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputScope_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputScope_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputScope_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputScope_GetInputScopes(This,pprgInputScopes,pcCount)	\
    ( (This)->lpVtbl -> GetInputScopes(This,pprgInputScopes,pcCount) ) 

#define ITfInputScope_GetPhrase(This,ppbstrPhrases,pcCount)	\
    ( (This)->lpVtbl -> GetPhrase(This,ppbstrPhrases,pcCount) ) 

#define ITfInputScope_GetRegularExpression(This,pbstrRegExp)	\
    ( (This)->lpVtbl -> GetRegularExpression(This,pbstrRegExp) ) 

#define ITfInputScope_GetSRGS(This,pbstrSRGS)	\
    ( (This)->lpVtbl -> GetSRGS(This,pbstrSRGS) ) 

#define ITfInputScope_GetXML(This,pbstrXML)	\
    ( (This)->lpVtbl -> GetXML(This,pbstrXML) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputScope_INTERFACE_DEFINED__ */


#ifndef __ITfInputScope2_INTERFACE_DEFINED__
#define __ITfInputScope2_INTERFACE_DEFINED__

/* interface ITfInputScope2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfInputScope2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5731eaa0-6bc2-4681-a532-92fbb74d7c41")
    ITfInputScope2 : public ITfInputScope
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumWordList( 
            /* [out] */ __RPC__deref_out_opt IEnumString **ppEnumString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfInputScope2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfInputScope2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfInputScope2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfInputScope2 * This);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetInputScopes)
        HRESULT ( STDMETHODCALLTYPE *GetInputScopes )( 
            __RPC__in ITfInputScope2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCount) InputScope **pprgInputScopes,
            /* [out] */ __RPC__out UINT *pcCount);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetPhrase)
        HRESULT ( STDMETHODCALLTYPE *GetPhrase )( 
            __RPC__in ITfInputScope2 * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcCount) BSTR **ppbstrPhrases,
            /* [out] */ __RPC__out UINT *pcCount);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetRegularExpression)
        HRESULT ( STDMETHODCALLTYPE *GetRegularExpression )( 
            __RPC__in ITfInputScope2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrRegExp);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetSRGS)
        HRESULT ( STDMETHODCALLTYPE *GetSRGS )( 
            __RPC__in ITfInputScope2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSRGS);
        
        DECLSPEC_XFGVIRT(ITfInputScope, GetXML)
        HRESULT ( STDMETHODCALLTYPE *GetXML )( 
            __RPC__in ITfInputScope2 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrXML);
        
        DECLSPEC_XFGVIRT(ITfInputScope2, EnumWordList)
        HRESULT ( STDMETHODCALLTYPE *EnumWordList )( 
            __RPC__in ITfInputScope2 * This,
            /* [out] */ __RPC__deref_out_opt IEnumString **ppEnumString);
        
        END_INTERFACE
    } ITfInputScope2Vtbl;

    interface ITfInputScope2
    {
        CONST_VTBL struct ITfInputScope2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfInputScope2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfInputScope2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfInputScope2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfInputScope2_GetInputScopes(This,pprgInputScopes,pcCount)	\
    ( (This)->lpVtbl -> GetInputScopes(This,pprgInputScopes,pcCount) ) 

#define ITfInputScope2_GetPhrase(This,ppbstrPhrases,pcCount)	\
    ( (This)->lpVtbl -> GetPhrase(This,ppbstrPhrases,pcCount) ) 

#define ITfInputScope2_GetRegularExpression(This,pbstrRegExp)	\
    ( (This)->lpVtbl -> GetRegularExpression(This,pbstrRegExp) ) 

#define ITfInputScope2_GetSRGS(This,pbstrSRGS)	\
    ( (This)->lpVtbl -> GetSRGS(This,pbstrSRGS) ) 

#define ITfInputScope2_GetXML(This,pbstrXML)	\
    ( (This)->lpVtbl -> GetXML(This,pbstrXML) ) 


#define ITfInputScope2_EnumWordList(This,ppEnumString)	\
    ( (This)->lpVtbl -> EnumWordList(This,ppEnumString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfInputScope2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_inputscope_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // INPUTSCOPE_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_inputscope_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_inputscope_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


