

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __certenc_h__
#define __certenc_h__

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

#ifndef __ICertEncodeStringArray_FWD_DEFINED__
#define __ICertEncodeStringArray_FWD_DEFINED__
typedef interface ICertEncodeStringArray ICertEncodeStringArray;

#endif 	/* __ICertEncodeStringArray_FWD_DEFINED__ */


#ifndef __ICertEncodeStringArray2_FWD_DEFINED__
#define __ICertEncodeStringArray2_FWD_DEFINED__
typedef interface ICertEncodeStringArray2 ICertEncodeStringArray2;

#endif 	/* __ICertEncodeStringArray2_FWD_DEFINED__ */


#ifndef __ICertEncodeLongArray_FWD_DEFINED__
#define __ICertEncodeLongArray_FWD_DEFINED__
typedef interface ICertEncodeLongArray ICertEncodeLongArray;

#endif 	/* __ICertEncodeLongArray_FWD_DEFINED__ */


#ifndef __ICertEncodeLongArray2_FWD_DEFINED__
#define __ICertEncodeLongArray2_FWD_DEFINED__
typedef interface ICertEncodeLongArray2 ICertEncodeLongArray2;

#endif 	/* __ICertEncodeLongArray2_FWD_DEFINED__ */


#ifndef __ICertEncodeDateArray_FWD_DEFINED__
#define __ICertEncodeDateArray_FWD_DEFINED__
typedef interface ICertEncodeDateArray ICertEncodeDateArray;

#endif 	/* __ICertEncodeDateArray_FWD_DEFINED__ */


#ifndef __ICertEncodeDateArray2_FWD_DEFINED__
#define __ICertEncodeDateArray2_FWD_DEFINED__
typedef interface ICertEncodeDateArray2 ICertEncodeDateArray2;

#endif 	/* __ICertEncodeDateArray2_FWD_DEFINED__ */


#ifndef __ICertEncodeCRLDistInfo_FWD_DEFINED__
#define __ICertEncodeCRLDistInfo_FWD_DEFINED__
typedef interface ICertEncodeCRLDistInfo ICertEncodeCRLDistInfo;

#endif 	/* __ICertEncodeCRLDistInfo_FWD_DEFINED__ */


#ifndef __ICertEncodeCRLDistInfo2_FWD_DEFINED__
#define __ICertEncodeCRLDistInfo2_FWD_DEFINED__
typedef interface ICertEncodeCRLDistInfo2 ICertEncodeCRLDistInfo2;

#endif 	/* __ICertEncodeCRLDistInfo2_FWD_DEFINED__ */


#ifndef __ICertEncodeAltName_FWD_DEFINED__
#define __ICertEncodeAltName_FWD_DEFINED__
typedef interface ICertEncodeAltName ICertEncodeAltName;

#endif 	/* __ICertEncodeAltName_FWD_DEFINED__ */


#ifndef __ICertEncodeAltName2_FWD_DEFINED__
#define __ICertEncodeAltName2_FWD_DEFINED__
typedef interface ICertEncodeAltName2 ICertEncodeAltName2;

#endif 	/* __ICertEncodeAltName2_FWD_DEFINED__ */


#ifndef __ICertEncodeBitString_FWD_DEFINED__
#define __ICertEncodeBitString_FWD_DEFINED__
typedef interface ICertEncodeBitString ICertEncodeBitString;

#endif 	/* __ICertEncodeBitString_FWD_DEFINED__ */


#ifndef __ICertEncodeBitString2_FWD_DEFINED__
#define __ICertEncodeBitString2_FWD_DEFINED__
typedef interface ICertEncodeBitString2 ICertEncodeBitString2;

#endif 	/* __ICertEncodeBitString2_FWD_DEFINED__ */


#ifndef __CCertEncodeStringArray_FWD_DEFINED__
#define __CCertEncodeStringArray_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertEncodeStringArray CCertEncodeStringArray;
#else
typedef struct CCertEncodeStringArray CCertEncodeStringArray;
#endif /* __cplusplus */

#endif 	/* __CCertEncodeStringArray_FWD_DEFINED__ */


#ifndef __CCertEncodeLongArray_FWD_DEFINED__
#define __CCertEncodeLongArray_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertEncodeLongArray CCertEncodeLongArray;
#else
typedef struct CCertEncodeLongArray CCertEncodeLongArray;
#endif /* __cplusplus */

#endif 	/* __CCertEncodeLongArray_FWD_DEFINED__ */


#ifndef __CCertEncodeDateArray_FWD_DEFINED__
#define __CCertEncodeDateArray_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertEncodeDateArray CCertEncodeDateArray;
#else
typedef struct CCertEncodeDateArray CCertEncodeDateArray;
#endif /* __cplusplus */

#endif 	/* __CCertEncodeDateArray_FWD_DEFINED__ */


#ifndef __CCertEncodeCRLDistInfo_FWD_DEFINED__
#define __CCertEncodeCRLDistInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertEncodeCRLDistInfo CCertEncodeCRLDistInfo;
#else
typedef struct CCertEncodeCRLDistInfo CCertEncodeCRLDistInfo;
#endif /* __cplusplus */

#endif 	/* __CCertEncodeCRLDistInfo_FWD_DEFINED__ */


#ifndef __CCertEncodeAltName_FWD_DEFINED__
#define __CCertEncodeAltName_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertEncodeAltName CCertEncodeAltName;
#else
typedef struct CCertEncodeAltName CCertEncodeAltName;
#endif /* __cplusplus */

#endif 	/* __CCertEncodeAltName_FWD_DEFINED__ */


#ifndef __CCertEncodeBitString_FWD_DEFINED__
#define __CCertEncodeBitString_FWD_DEFINED__

#ifdef __cplusplus
typedef class CCertEncodeBitString CCertEncodeBitString;
#else
typedef struct CCertEncodeBitString CCertEncodeBitString;
#endif /* __cplusplus */

#endif 	/* __CCertEncodeBitString_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "certenroll.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_certenc_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_certenc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certenc_0000_0000_v0_0_s_ifspec;

#ifndef __ICertEncodeStringArray_INTERFACE_DEFINED__
#define __ICertEncodeStringArray_INTERFACE_DEFINED__

/* interface ICertEncodeStringArray */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeStringArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12a88820-7494-11d0-8816-00a0c903b83c")
    ICertEncodeStringArray : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ __RPC__in const BSTR strBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringType( 
            /* [retval][out] */ __RPC__out LONG *pStringType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ LONG Count,
            /* [in] */ LONG StringType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ LONG Index,
            /* [in] */ __RPC__in const BSTR str) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Encode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeStringArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeStringArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeStringArray * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeStringArray * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, GetStringType)
        HRESULT ( STDMETHODCALLTYPE *GetStringType )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [retval][out] */ __RPC__out LONG *pStringType);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ LONG Count,
            /* [in] */ LONG StringType);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [in] */ LONG Index,
            /* [in] */ __RPC__in const BSTR str);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeStringArray * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        END_INTERFACE
    } ICertEncodeStringArrayVtbl;

    interface ICertEncodeStringArray
    {
        CONST_VTBL struct ICertEncodeStringArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeStringArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeStringArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeStringArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeStringArray_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeStringArray_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeStringArray_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeStringArray_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeStringArray_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeStringArray_GetStringType(This,pStringType)	\
    ( (This)->lpVtbl -> GetStringType(This,pStringType) ) 

#define ICertEncodeStringArray_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ICertEncodeStringArray_GetValue(This,Index,pstr)	\
    ( (This)->lpVtbl -> GetValue(This,Index,pstr) ) 

#define ICertEncodeStringArray_Reset(This,Count,StringType)	\
    ( (This)->lpVtbl -> Reset(This,Count,StringType) ) 

#define ICertEncodeStringArray_SetValue(This,Index,str)	\
    ( (This)->lpVtbl -> SetValue(This,Index,str) ) 

#define ICertEncodeStringArray_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeStringArray_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeStringArray2_INTERFACE_DEFINED__
#define __ICertEncodeStringArray2_INTERFACE_DEFINED__

/* interface ICertEncodeStringArray2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeStringArray2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9c680d93-9b7d-4e95-9018-4ffe10ba5ada")
    ICertEncodeStringArray2 : public ICertEncodeStringArray
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DecodeBlob( 
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncodeBlob( 
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeStringArray2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeStringArray2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeStringArray2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeStringArray2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, GetStringType)
        HRESULT ( STDMETHODCALLTYPE *GetStringType )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [retval][out] */ __RPC__out LONG *pStringType);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstr);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ LONG Count,
            /* [in] */ LONG StringType);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ LONG Index,
            /* [in] */ __RPC__in const BSTR str);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray2, DecodeBlob)
        HRESULT ( STDMETHODCALLTYPE *DecodeBlob )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        DECLSPEC_XFGVIRT(ICertEncodeStringArray2, EncodeBlob)
        HRESULT ( STDMETHODCALLTYPE *EncodeBlob )( 
            __RPC__in ICertEncodeStringArray2 * This,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData);
        
        END_INTERFACE
    } ICertEncodeStringArray2Vtbl;

    interface ICertEncodeStringArray2
    {
        CONST_VTBL struct ICertEncodeStringArray2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeStringArray2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeStringArray2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeStringArray2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeStringArray2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeStringArray2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeStringArray2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeStringArray2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeStringArray2_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeStringArray2_GetStringType(This,pStringType)	\
    ( (This)->lpVtbl -> GetStringType(This,pStringType) ) 

#define ICertEncodeStringArray2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ICertEncodeStringArray2_GetValue(This,Index,pstr)	\
    ( (This)->lpVtbl -> GetValue(This,Index,pstr) ) 

#define ICertEncodeStringArray2_Reset(This,Count,StringType)	\
    ( (This)->lpVtbl -> Reset(This,Count,StringType) ) 

#define ICertEncodeStringArray2_SetValue(This,Index,str)	\
    ( (This)->lpVtbl -> SetValue(This,Index,str) ) 

#define ICertEncodeStringArray2_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 


#define ICertEncodeStringArray2_DecodeBlob(This,strEncodedData,Encoding)	\
    ( (This)->lpVtbl -> DecodeBlob(This,strEncodedData,Encoding) ) 

#define ICertEncodeStringArray2_EncodeBlob(This,Encoding,pstrEncodedData)	\
    ( (This)->lpVtbl -> EncodeBlob(This,Encoding,pstrEncodedData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeStringArray2_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeLongArray_INTERFACE_DEFINED__
#define __ICertEncodeLongArray_INTERFACE_DEFINED__

/* interface ICertEncodeLongArray */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeLongArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("15e2f230-a0a2-11d0-8821-00a0c903b83c")
    ICertEncodeLongArray : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ __RPC__in const BSTR strBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out LONG *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ LONG Count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ LONG Index,
            /* [in] */ LONG Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Encode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeLongArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeLongArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeLongArray * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeLongArray * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out LONG *pValue);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ LONG Count);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [in] */ LONG Index,
            /* [in] */ LONG Value);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeLongArray * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        END_INTERFACE
    } ICertEncodeLongArrayVtbl;

    interface ICertEncodeLongArray
    {
        CONST_VTBL struct ICertEncodeLongArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeLongArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeLongArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeLongArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeLongArray_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeLongArray_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeLongArray_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeLongArray_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeLongArray_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeLongArray_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ICertEncodeLongArray_GetValue(This,Index,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,Index,pValue) ) 

#define ICertEncodeLongArray_Reset(This,Count)	\
    ( (This)->lpVtbl -> Reset(This,Count) ) 

#define ICertEncodeLongArray_SetValue(This,Index,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Index,Value) ) 

#define ICertEncodeLongArray_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeLongArray_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeLongArray2_INTERFACE_DEFINED__
#define __ICertEncodeLongArray2_INTERFACE_DEFINED__

/* interface ICertEncodeLongArray2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeLongArray2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4efde84a-bd9b-4fc2-a108-c347d478840f")
    ICertEncodeLongArray2 : public ICertEncodeLongArray
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DecodeBlob( 
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncodeBlob( 
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeLongArray2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeLongArray2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeLongArray2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeLongArray2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out LONG *pValue);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ LONG Count);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ LONG Index,
            /* [in] */ LONG Value);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray2, DecodeBlob)
        HRESULT ( STDMETHODCALLTYPE *DecodeBlob )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        DECLSPEC_XFGVIRT(ICertEncodeLongArray2, EncodeBlob)
        HRESULT ( STDMETHODCALLTYPE *EncodeBlob )( 
            __RPC__in ICertEncodeLongArray2 * This,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData);
        
        END_INTERFACE
    } ICertEncodeLongArray2Vtbl;

    interface ICertEncodeLongArray2
    {
        CONST_VTBL struct ICertEncodeLongArray2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeLongArray2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeLongArray2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeLongArray2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeLongArray2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeLongArray2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeLongArray2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeLongArray2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeLongArray2_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeLongArray2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ICertEncodeLongArray2_GetValue(This,Index,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,Index,pValue) ) 

#define ICertEncodeLongArray2_Reset(This,Count)	\
    ( (This)->lpVtbl -> Reset(This,Count) ) 

#define ICertEncodeLongArray2_SetValue(This,Index,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Index,Value) ) 

#define ICertEncodeLongArray2_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 


#define ICertEncodeLongArray2_DecodeBlob(This,strEncodedData,Encoding)	\
    ( (This)->lpVtbl -> DecodeBlob(This,strEncodedData,Encoding) ) 

#define ICertEncodeLongArray2_EncodeBlob(This,Encoding,pstrEncodedData)	\
    ( (This)->lpVtbl -> EncodeBlob(This,Encoding,pstrEncodedData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeLongArray2_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeDateArray_INTERFACE_DEFINED__
#define __ICertEncodeDateArray_INTERFACE_DEFINED__

/* interface ICertEncodeDateArray */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeDateArray;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2f9469a0-a470-11d0-8821-00a0c903b83c")
    ICertEncodeDateArray : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ __RPC__in const BSTR strBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out LONG *pCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out DATE *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ LONG Count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ LONG Index,
            /* [in] */ DATE Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Encode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeDateArrayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeDateArray * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeDateArray * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeDateArray * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out DATE *pValue);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ LONG Count);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [in] */ LONG Index,
            /* [in] */ DATE Value);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeDateArray * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        END_INTERFACE
    } ICertEncodeDateArrayVtbl;

    interface ICertEncodeDateArray
    {
        CONST_VTBL struct ICertEncodeDateArrayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeDateArray_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeDateArray_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeDateArray_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeDateArray_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeDateArray_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeDateArray_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeDateArray_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeDateArray_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeDateArray_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ICertEncodeDateArray_GetValue(This,Index,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,Index,pValue) ) 

#define ICertEncodeDateArray_Reset(This,Count)	\
    ( (This)->lpVtbl -> Reset(This,Count) ) 

#define ICertEncodeDateArray_SetValue(This,Index,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Index,Value) ) 

#define ICertEncodeDateArray_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeDateArray_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeDateArray2_INTERFACE_DEFINED__
#define __ICertEncodeDateArray2_INTERFACE_DEFINED__

/* interface ICertEncodeDateArray2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeDateArray2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("99a4edb5-2b8e-448d-bf95-bba8d7789dc8")
    ICertEncodeDateArray2 : public ICertEncodeDateArray
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DecodeBlob( 
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncodeBlob( 
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeDateArray2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeDateArray2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeDateArray2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeDateArray2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [retval][out] */ __RPC__out LONG *pCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ LONG Index,
            /* [retval][out] */ __RPC__out DATE *pValue);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ LONG Count);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ LONG Index,
            /* [in] */ DATE Value);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray2, DecodeBlob)
        HRESULT ( STDMETHODCALLTYPE *DecodeBlob )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        DECLSPEC_XFGVIRT(ICertEncodeDateArray2, EncodeBlob)
        HRESULT ( STDMETHODCALLTYPE *EncodeBlob )( 
            __RPC__in ICertEncodeDateArray2 * This,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData);
        
        END_INTERFACE
    } ICertEncodeDateArray2Vtbl;

    interface ICertEncodeDateArray2
    {
        CONST_VTBL struct ICertEncodeDateArray2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeDateArray2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeDateArray2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeDateArray2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeDateArray2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeDateArray2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeDateArray2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeDateArray2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeDateArray2_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeDateArray2_GetCount(This,pCount)	\
    ( (This)->lpVtbl -> GetCount(This,pCount) ) 

#define ICertEncodeDateArray2_GetValue(This,Index,pValue)	\
    ( (This)->lpVtbl -> GetValue(This,Index,pValue) ) 

#define ICertEncodeDateArray2_Reset(This,Count)	\
    ( (This)->lpVtbl -> Reset(This,Count) ) 

#define ICertEncodeDateArray2_SetValue(This,Index,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Index,Value) ) 

#define ICertEncodeDateArray2_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 


#define ICertEncodeDateArray2_DecodeBlob(This,strEncodedData,Encoding)	\
    ( (This)->lpVtbl -> DecodeBlob(This,strEncodedData,Encoding) ) 

#define ICertEncodeDateArray2_EncodeBlob(This,Encoding,pstrEncodedData)	\
    ( (This)->lpVtbl -> EncodeBlob(This,Encoding,pstrEncodedData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeDateArray2_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeCRLDistInfo_INTERFACE_DEFINED__
#define __ICertEncodeCRLDistInfo_INTERFACE_DEFINED__

/* interface ICertEncodeCRLDistInfo */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeCRLDistInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("01958640-bbff-11d0-8825-00a0c903b83c")
    ICertEncodeCRLDistInfo : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ __RPC__in const BSTR strBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDistPointCount( 
            /* [retval][out] */ __RPC__out LONG *pDistPointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameCount( 
            /* [in] */ LONG DistPointIndex,
            /* [retval][out] */ __RPC__out LONG *pNameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameChoice( 
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__out LONG *pNameChoice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ LONG DistPointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNameCount( 
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNameEntry( 
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Encode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeCRLDistInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeCRLDistInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeCRLDistInfo * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeCRLDistInfo * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetDistPointCount)
        HRESULT ( STDMETHODCALLTYPE *GetDistPointCount )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [retval][out] */ __RPC__out LONG *pDistPointCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetNameCount)
        HRESULT ( STDMETHODCALLTYPE *GetNameCount )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ LONG DistPointIndex,
            /* [retval][out] */ __RPC__out LONG *pNameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetNameChoice)
        HRESULT ( STDMETHODCALLTYPE *GetNameChoice )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__out LONG *pNameChoice);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ LONG DistPointCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, SetNameCount)
        HRESULT ( STDMETHODCALLTYPE *SetNameCount )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, SetNameEntry)
        HRESULT ( STDMETHODCALLTYPE *SetNameEntry )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeCRLDistInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        END_INTERFACE
    } ICertEncodeCRLDistInfoVtbl;

    interface ICertEncodeCRLDistInfo
    {
        CONST_VTBL struct ICertEncodeCRLDistInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeCRLDistInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeCRLDistInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeCRLDistInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeCRLDistInfo_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeCRLDistInfo_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeCRLDistInfo_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeCRLDistInfo_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeCRLDistInfo_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeCRLDistInfo_GetDistPointCount(This,pDistPointCount)	\
    ( (This)->lpVtbl -> GetDistPointCount(This,pDistPointCount) ) 

#define ICertEncodeCRLDistInfo_GetNameCount(This,DistPointIndex,pNameCount)	\
    ( (This)->lpVtbl -> GetNameCount(This,DistPointIndex,pNameCount) ) 

#define ICertEncodeCRLDistInfo_GetNameChoice(This,DistPointIndex,NameIndex,pNameChoice)	\
    ( (This)->lpVtbl -> GetNameChoice(This,DistPointIndex,NameIndex,pNameChoice) ) 

#define ICertEncodeCRLDistInfo_GetName(This,DistPointIndex,NameIndex,pstrName)	\
    ( (This)->lpVtbl -> GetName(This,DistPointIndex,NameIndex,pstrName) ) 

#define ICertEncodeCRLDistInfo_Reset(This,DistPointCount)	\
    ( (This)->lpVtbl -> Reset(This,DistPointCount) ) 

#define ICertEncodeCRLDistInfo_SetNameCount(This,DistPointIndex,NameCount)	\
    ( (This)->lpVtbl -> SetNameCount(This,DistPointIndex,NameCount) ) 

#define ICertEncodeCRLDistInfo_SetNameEntry(This,DistPointIndex,NameIndex,NameChoice,strName)	\
    ( (This)->lpVtbl -> SetNameEntry(This,DistPointIndex,NameIndex,NameChoice,strName) ) 

#define ICertEncodeCRLDistInfo_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeCRLDistInfo_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeCRLDistInfo2_INTERFACE_DEFINED__
#define __ICertEncodeCRLDistInfo2_INTERFACE_DEFINED__

/* interface ICertEncodeCRLDistInfo2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeCRLDistInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b4275d4b-3e30-446f-ad36-09d03120b078")
    ICertEncodeCRLDistInfo2 : public ICertEncodeCRLDistInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DecodeBlob( 
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncodeBlob( 
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeCRLDistInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeCRLDistInfo2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetDistPointCount)
        HRESULT ( STDMETHODCALLTYPE *GetDistPointCount )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [retval][out] */ __RPC__out LONG *pDistPointCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetNameCount)
        HRESULT ( STDMETHODCALLTYPE *GetNameCount )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ LONG DistPointIndex,
            /* [retval][out] */ __RPC__out LONG *pNameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetNameChoice)
        HRESULT ( STDMETHODCALLTYPE *GetNameChoice )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__out LONG *pNameChoice);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ LONG DistPointCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, SetNameCount)
        HRESULT ( STDMETHODCALLTYPE *SetNameCount )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, SetNameEntry)
        HRESULT ( STDMETHODCALLTYPE *SetNameEntry )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ LONG DistPointIndex,
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo2, DecodeBlob)
        HRESULT ( STDMETHODCALLTYPE *DecodeBlob )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        DECLSPEC_XFGVIRT(ICertEncodeCRLDistInfo2, EncodeBlob)
        HRESULT ( STDMETHODCALLTYPE *EncodeBlob )( 
            __RPC__in ICertEncodeCRLDistInfo2 * This,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData);
        
        END_INTERFACE
    } ICertEncodeCRLDistInfo2Vtbl;

    interface ICertEncodeCRLDistInfo2
    {
        CONST_VTBL struct ICertEncodeCRLDistInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeCRLDistInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeCRLDistInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeCRLDistInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeCRLDistInfo2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeCRLDistInfo2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeCRLDistInfo2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeCRLDistInfo2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeCRLDistInfo2_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeCRLDistInfo2_GetDistPointCount(This,pDistPointCount)	\
    ( (This)->lpVtbl -> GetDistPointCount(This,pDistPointCount) ) 

#define ICertEncodeCRLDistInfo2_GetNameCount(This,DistPointIndex,pNameCount)	\
    ( (This)->lpVtbl -> GetNameCount(This,DistPointIndex,pNameCount) ) 

#define ICertEncodeCRLDistInfo2_GetNameChoice(This,DistPointIndex,NameIndex,pNameChoice)	\
    ( (This)->lpVtbl -> GetNameChoice(This,DistPointIndex,NameIndex,pNameChoice) ) 

#define ICertEncodeCRLDistInfo2_GetName(This,DistPointIndex,NameIndex,pstrName)	\
    ( (This)->lpVtbl -> GetName(This,DistPointIndex,NameIndex,pstrName) ) 

#define ICertEncodeCRLDistInfo2_Reset(This,DistPointCount)	\
    ( (This)->lpVtbl -> Reset(This,DistPointCount) ) 

#define ICertEncodeCRLDistInfo2_SetNameCount(This,DistPointIndex,NameCount)	\
    ( (This)->lpVtbl -> SetNameCount(This,DistPointIndex,NameCount) ) 

#define ICertEncodeCRLDistInfo2_SetNameEntry(This,DistPointIndex,NameIndex,NameChoice,strName)	\
    ( (This)->lpVtbl -> SetNameEntry(This,DistPointIndex,NameIndex,NameChoice,strName) ) 

#define ICertEncodeCRLDistInfo2_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 


#define ICertEncodeCRLDistInfo2_DecodeBlob(This,strEncodedData,Encoding)	\
    ( (This)->lpVtbl -> DecodeBlob(This,strEncodedData,Encoding) ) 

#define ICertEncodeCRLDistInfo2_EncodeBlob(This,Encoding,pstrEncodedData)	\
    ( (This)->lpVtbl -> EncodeBlob(This,Encoding,pstrEncodedData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeCRLDistInfo2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_certenc_0000_0008 */
/* [local] */ 

#define	EAN_NAMEOBJECTID	( 0x80000000 )

#define	EANR_SUPPRESS_IA5CONVERSION	( 0x80000000 )



extern RPC_IF_HANDLE __MIDL_itf_certenc_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certenc_0000_0008_v0_0_s_ifspec;

#ifndef __ICertEncodeAltName_INTERFACE_DEFINED__
#define __ICertEncodeAltName_INTERFACE_DEFINED__

/* interface ICertEncodeAltName */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeAltName;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1c9a8c70-1271-11d1-9bd4-00c04fb683fa")
    ICertEncodeAltName : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ __RPC__in const BSTR strBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameCount( 
            /* [retval][out] */ __RPC__out LONG *pNameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameChoice( 
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__out LONG *pNameChoice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( 
            /* [in] */ LONG NameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNameEntry( 
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Encode( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeAltNameVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeAltName * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeAltName * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeAltName * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeAltName * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, GetNameCount)
        HRESULT ( STDMETHODCALLTYPE *GetNameCount )( 
            __RPC__in ICertEncodeAltName * This,
            /* [retval][out] */ __RPC__out LONG *pNameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, GetNameChoice)
        HRESULT ( STDMETHODCALLTYPE *GetNameChoice )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__out LONG *pNameChoice);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ LONG NameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, SetNameEntry)
        HRESULT ( STDMETHODCALLTYPE *SetNameEntry )( 
            __RPC__in ICertEncodeAltName * This,
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeAltName * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        END_INTERFACE
    } ICertEncodeAltNameVtbl;

    interface ICertEncodeAltName
    {
        CONST_VTBL struct ICertEncodeAltNameVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeAltName_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeAltName_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeAltName_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeAltName_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeAltName_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeAltName_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeAltName_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeAltName_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeAltName_GetNameCount(This,pNameCount)	\
    ( (This)->lpVtbl -> GetNameCount(This,pNameCount) ) 

#define ICertEncodeAltName_GetNameChoice(This,NameIndex,pNameChoice)	\
    ( (This)->lpVtbl -> GetNameChoice(This,NameIndex,pNameChoice) ) 

#define ICertEncodeAltName_GetName(This,NameIndex,pstrName)	\
    ( (This)->lpVtbl -> GetName(This,NameIndex,pstrName) ) 

#define ICertEncodeAltName_Reset(This,NameCount)	\
    ( (This)->lpVtbl -> Reset(This,NameCount) ) 

#define ICertEncodeAltName_SetNameEntry(This,NameIndex,NameChoice,strName)	\
    ( (This)->lpVtbl -> SetNameEntry(This,NameIndex,NameChoice,strName) ) 

#define ICertEncodeAltName_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeAltName_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeAltName2_INTERFACE_DEFINED__
#define __ICertEncodeAltName2_INTERFACE_DEFINED__

/* interface ICertEncodeAltName2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeAltName2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f67fe177-5ef1-4535-b4ce-29df15e2e0c3")
    ICertEncodeAltName2 : public ICertEncodeAltName
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DecodeBlob( 
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncodeBlob( 
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNameBlob( 
            /* [in] */ LONG NameIndex,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNameEntryBlob( 
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeAltName2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeAltName2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeAltName2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeAltName2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, GetNameCount)
        HRESULT ( STDMETHODCALLTYPE *GetNameCount )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [retval][out] */ __RPC__out LONG *pNameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, GetNameChoice)
        HRESULT ( STDMETHODCALLTYPE *GetNameChoice )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__out LONG *pNameChoice);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ LONG NameIndex,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ LONG NameCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, SetNameEntry)
        HRESULT ( STDMETHODCALLTYPE *SetNameEntry )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName2, DecodeBlob)
        HRESULT ( STDMETHODCALLTYPE *DecodeBlob )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName2, EncodeBlob)
        HRESULT ( STDMETHODCALLTYPE *EncodeBlob )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName2, GetNameBlob)
        HRESULT ( STDMETHODCALLTYPE *GetNameBlob )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ LONG NameIndex,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrName);
        
        DECLSPEC_XFGVIRT(ICertEncodeAltName2, SetNameEntryBlob)
        HRESULT ( STDMETHODCALLTYPE *SetNameEntryBlob )( 
            __RPC__in ICertEncodeAltName2 * This,
            /* [in] */ LONG NameIndex,
            /* [in] */ LONG NameChoice,
            /* [in] */ __RPC__in const BSTR strName,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        END_INTERFACE
    } ICertEncodeAltName2Vtbl;

    interface ICertEncodeAltName2
    {
        CONST_VTBL struct ICertEncodeAltName2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeAltName2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeAltName2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeAltName2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeAltName2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeAltName2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeAltName2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeAltName2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeAltName2_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeAltName2_GetNameCount(This,pNameCount)	\
    ( (This)->lpVtbl -> GetNameCount(This,pNameCount) ) 

#define ICertEncodeAltName2_GetNameChoice(This,NameIndex,pNameChoice)	\
    ( (This)->lpVtbl -> GetNameChoice(This,NameIndex,pNameChoice) ) 

#define ICertEncodeAltName2_GetName(This,NameIndex,pstrName)	\
    ( (This)->lpVtbl -> GetName(This,NameIndex,pstrName) ) 

#define ICertEncodeAltName2_Reset(This,NameCount)	\
    ( (This)->lpVtbl -> Reset(This,NameCount) ) 

#define ICertEncodeAltName2_SetNameEntry(This,NameIndex,NameChoice,strName)	\
    ( (This)->lpVtbl -> SetNameEntry(This,NameIndex,NameChoice,strName) ) 

#define ICertEncodeAltName2_Encode(This,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,pstrBinary) ) 


#define ICertEncodeAltName2_DecodeBlob(This,strEncodedData,Encoding)	\
    ( (This)->lpVtbl -> DecodeBlob(This,strEncodedData,Encoding) ) 

#define ICertEncodeAltName2_EncodeBlob(This,Encoding,pstrEncodedData)	\
    ( (This)->lpVtbl -> EncodeBlob(This,Encoding,pstrEncodedData) ) 

#define ICertEncodeAltName2_GetNameBlob(This,NameIndex,Encoding,pstrName)	\
    ( (This)->lpVtbl -> GetNameBlob(This,NameIndex,Encoding,pstrName) ) 

#define ICertEncodeAltName2_SetNameEntryBlob(This,NameIndex,NameChoice,strName,Encoding)	\
    ( (This)->lpVtbl -> SetNameEntryBlob(This,NameIndex,NameChoice,strName,Encoding) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeAltName2_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeBitString_INTERFACE_DEFINED__
#define __ICertEncodeBitString_INTERFACE_DEFINED__

/* interface ICertEncodeBitString */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeBitString;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6db525be-1278-11d1-9bd4-00c04fb683fa")
    ICertEncodeBitString : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Decode( 
            /* [in] */ __RPC__in const BSTR strBinary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBitCount( 
            /* [retval][out] */ __RPC__out LONG *pBitCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBitString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBitString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Encode( 
            /* [in] */ LONG BitCount,
            /* [in] */ __RPC__in BSTR strBitString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeBitStringVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeBitString * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeBitString * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeBitString * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeBitString * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeBitString * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeBitString * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeBitString * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeBitString * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, GetBitCount)
        HRESULT ( STDMETHODCALLTYPE *GetBitCount )( 
            __RPC__in ICertEncodeBitString * This,
            /* [retval][out] */ __RPC__out LONG *pBitCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, GetBitString)
        HRESULT ( STDMETHODCALLTYPE *GetBitString )( 
            __RPC__in ICertEncodeBitString * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBitString);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeBitString * This,
            /* [in] */ LONG BitCount,
            /* [in] */ __RPC__in BSTR strBitString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        END_INTERFACE
    } ICertEncodeBitStringVtbl;

    interface ICertEncodeBitString
    {
        CONST_VTBL struct ICertEncodeBitStringVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeBitString_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeBitString_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeBitString_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeBitString_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeBitString_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeBitString_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeBitString_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeBitString_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeBitString_GetBitCount(This,pBitCount)	\
    ( (This)->lpVtbl -> GetBitCount(This,pBitCount) ) 

#define ICertEncodeBitString_GetBitString(This,pstrBitString)	\
    ( (This)->lpVtbl -> GetBitString(This,pstrBitString) ) 

#define ICertEncodeBitString_Encode(This,BitCount,strBitString,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,BitCount,strBitString,pstrBinary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeBitString_INTERFACE_DEFINED__ */


#ifndef __ICertEncodeBitString2_INTERFACE_DEFINED__
#define __ICertEncodeBitString2_INTERFACE_DEFINED__

/* interface ICertEncodeBitString2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertEncodeBitString2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e070d6e7-23ef-4dd2-8242-ebd9c928cb30")
    ICertEncodeBitString2 : public ICertEncodeBitString
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DecodeBlob( 
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding = XCN_CRYPT_STRING_BASE64) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EncodeBlob( 
            /* [in] */ LONG BitCount,
            /* [in] */ __RPC__in const BSTR strBitString,
            /* [defaultvalue][in] */ EncodingType EncodingIn,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBitStringBlob( 
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBitString) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertEncodeBitString2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertEncodeBitString2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertEncodeBitString2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertEncodeBitString2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, Decode)
        HRESULT ( STDMETHODCALLTYPE *Decode )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ __RPC__in const BSTR strBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, GetBitCount)
        HRESULT ( STDMETHODCALLTYPE *GetBitCount )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [retval][out] */ __RPC__out LONG *pBitCount);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, GetBitString)
        HRESULT ( STDMETHODCALLTYPE *GetBitString )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBitString);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString, Encode)
        HRESULT ( STDMETHODCALLTYPE *Encode )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ LONG BitCount,
            /* [in] */ __RPC__in BSTR strBitString,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBinary);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString2, DecodeBlob)
        HRESULT ( STDMETHODCALLTYPE *DecodeBlob )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ __RPC__in const BSTR strEncodedData,
            /* [defaultvalue][in] */ EncodingType Encoding);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString2, EncodeBlob)
        HRESULT ( STDMETHODCALLTYPE *EncodeBlob )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [in] */ LONG BitCount,
            /* [in] */ __RPC__in const BSTR strBitString,
            /* [defaultvalue][in] */ EncodingType EncodingIn,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrEncodedData);
        
        DECLSPEC_XFGVIRT(ICertEncodeBitString2, GetBitStringBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBitStringBlob )( 
            __RPC__in ICertEncodeBitString2 * This,
            /* [defaultvalue][in] */ EncodingType Encoding,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pstrBitString);
        
        END_INTERFACE
    } ICertEncodeBitString2Vtbl;

    interface ICertEncodeBitString2
    {
        CONST_VTBL struct ICertEncodeBitString2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertEncodeBitString2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertEncodeBitString2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertEncodeBitString2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertEncodeBitString2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertEncodeBitString2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertEncodeBitString2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertEncodeBitString2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertEncodeBitString2_Decode(This,strBinary)	\
    ( (This)->lpVtbl -> Decode(This,strBinary) ) 

#define ICertEncodeBitString2_GetBitCount(This,pBitCount)	\
    ( (This)->lpVtbl -> GetBitCount(This,pBitCount) ) 

#define ICertEncodeBitString2_GetBitString(This,pstrBitString)	\
    ( (This)->lpVtbl -> GetBitString(This,pstrBitString) ) 

#define ICertEncodeBitString2_Encode(This,BitCount,strBitString,pstrBinary)	\
    ( (This)->lpVtbl -> Encode(This,BitCount,strBitString,pstrBinary) ) 


#define ICertEncodeBitString2_DecodeBlob(This,strEncodedData,Encoding)	\
    ( (This)->lpVtbl -> DecodeBlob(This,strEncodedData,Encoding) ) 

#define ICertEncodeBitString2_EncodeBlob(This,BitCount,strBitString,EncodingIn,Encoding,pstrEncodedData)	\
    ( (This)->lpVtbl -> EncodeBlob(This,BitCount,strBitString,EncodingIn,Encoding,pstrEncodedData) ) 

#define ICertEncodeBitString2_GetBitStringBlob(This,Encoding,pstrBitString)	\
    ( (This)->lpVtbl -> GetBitStringBlob(This,Encoding,pstrBitString) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertEncodeBitString2_INTERFACE_DEFINED__ */



#ifndef __CERTENCODELib_LIBRARY_DEFINED__
#define __CERTENCODELib_LIBRARY_DEFINED__

/* library CERTENCODELib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_CERTENCODELib;

EXTERN_C const CLSID CLSID_CCertEncodeStringArray;

#ifdef __cplusplus

class DECLSPEC_UUID("19a76fe0-7494-11d0-8816-00a0c903b83c")
CCertEncodeStringArray;
#endif

EXTERN_C const CLSID CLSID_CCertEncodeLongArray;

#ifdef __cplusplus

class DECLSPEC_UUID("4e0680a0-a0a2-11d0-8821-00a0c903b83c")
CCertEncodeLongArray;
#endif

EXTERN_C const CLSID CLSID_CCertEncodeDateArray;

#ifdef __cplusplus

class DECLSPEC_UUID("301f77b0-a470-11d0-8821-00a0c903b83c")
CCertEncodeDateArray;
#endif

EXTERN_C const CLSID CLSID_CCertEncodeCRLDistInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("01fa60a0-bbff-11d0-8825-00a0c903b83c")
CCertEncodeCRLDistInfo;
#endif

EXTERN_C const CLSID CLSID_CCertEncodeAltName;

#ifdef __cplusplus

class DECLSPEC_UUID("1cfc4cda-1271-11d1-9bd4-00c04fb683fa")
CCertEncodeAltName;
#endif

EXTERN_C const CLSID CLSID_CCertEncodeBitString;

#ifdef __cplusplus

class DECLSPEC_UUID("6d6b3cd8-1278-11d1-9bd4-00c04fb683fa")
CCertEncodeBitString;
#endif
#endif /* __CERTENCODELib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_certenc_0000_0013 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_certenc_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certenc_0000_0013_v0_0_s_ifspec;

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


