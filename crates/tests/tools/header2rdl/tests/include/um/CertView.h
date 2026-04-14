

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

#ifndef __certview_h__
#define __certview_h__

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

#ifndef __IEnumCERTVIEWCOLUMN_FWD_DEFINED__
#define __IEnumCERTVIEWCOLUMN_FWD_DEFINED__
typedef interface IEnumCERTVIEWCOLUMN IEnumCERTVIEWCOLUMN;

#endif 	/* __IEnumCERTVIEWCOLUMN_FWD_DEFINED__ */


#ifndef __IEnumCERTVIEWATTRIBUTE_FWD_DEFINED__
#define __IEnumCERTVIEWATTRIBUTE_FWD_DEFINED__
typedef interface IEnumCERTVIEWATTRIBUTE IEnumCERTVIEWATTRIBUTE;

#endif 	/* __IEnumCERTVIEWATTRIBUTE_FWD_DEFINED__ */


#ifndef __IEnumCERTVIEWEXTENSION_FWD_DEFINED__
#define __IEnumCERTVIEWEXTENSION_FWD_DEFINED__
typedef interface IEnumCERTVIEWEXTENSION IEnumCERTVIEWEXTENSION;

#endif 	/* __IEnumCERTVIEWEXTENSION_FWD_DEFINED__ */


#ifndef __IEnumCERTVIEWROW_FWD_DEFINED__
#define __IEnumCERTVIEWROW_FWD_DEFINED__
typedef interface IEnumCERTVIEWROW IEnumCERTVIEWROW;

#endif 	/* __IEnumCERTVIEWROW_FWD_DEFINED__ */


#ifndef __ICertView_FWD_DEFINED__
#define __ICertView_FWD_DEFINED__
typedef interface ICertView ICertView;

#endif 	/* __ICertView_FWD_DEFINED__ */


#ifndef __ICertView2_FWD_DEFINED__
#define __ICertView2_FWD_DEFINED__
typedef interface ICertView2 ICertView2;

#endif 	/* __ICertView2_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_certview_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define	CV_OUT_BASE64HEADER	( 0 )

#define	CV_OUT_BASE64	( 0x1 )

#define	CV_OUT_BINARY	( 0x2 )

#define	CV_OUT_BASE64REQUESTHEADER	( 0x3 )

#define	CV_OUT_HEX	( 0x4 )

#define	CV_OUT_HEXASCII	( 0x5 )

#define	CV_OUT_BASE64X509CRLHEADER	( 0x9 )

#define	CV_OUT_HEXADDR	( 0xa )

#define	CV_OUT_HEXASCIIADDR	( 0xb )

#define	CV_OUT_HEXRAW	( 0xc )

#define	CV_OUT_ENCODEMASK	( 0xff )

#define	CV_OUT_NOCRLF	( 0x40000000 )

#define	CV_OUT_NOCR	( 0x80000000 )

#define	CVR_SEEK_NONE	( 0 )

#define	CVR_SEEK_EQ	( 0x1 )

#define	CVR_SEEK_LT	( 0x2 )

#define	CVR_SEEK_LE	( 0x4 )

#define	CVR_SEEK_GE	( 0x8 )

#define	CVR_SEEK_GT	( 0x10 )

#define	CVR_SEEK_MASK	( 0xff )

#define	CVR_SEEK_NODELTA	( 0x1000 )

#define	CVR_SORT_NONE	( 0 )

#define	CVR_SORT_ASCEND	( 0x1 )

#define	CVR_SORT_DESCEND	( 0x2 )

#define	CV_COLUMN_QUEUE_DEFAULT	( -1 )

#define	CV_COLUMN_LOG_DEFAULT	( -2 )

#define	CV_COLUMN_LOG_FAILED_DEFAULT	( -3 )

#define	CV_COLUMN_EXTENSION_DEFAULT	( -4 )

#define	CV_COLUMN_ATTRIBUTE_DEFAULT	( -5 )

#define	CV_COLUMN_CRL_DEFAULT	( -6 )

#define	CV_COLUMN_LOG_REVOKED_DEFAULT	( -7 )

#define	CVRC_COLUMN_SCHEMA	( 0 )

#define	CVRC_COLUMN_RESULT	( 0x1 )

#define	CVRC_COLUMN_VALUE	( 0x2 )

#define	CVRC_COLUMN_MASK	( 0xfff )

#define	CVRC_TABLE_REQCERT	( 0 )

#define	CVRC_TABLE_EXTENSIONS	( 0x3000 )

#define	CVRC_TABLE_ATTRIBUTES	( 0x4000 )

#define	CVRC_TABLE_CRL	( 0x5000 )

#define	CVRC_TABLE_MASK	( 0xf000 )

#define	CVRC_TABLE_SHIFT	( 12 )



extern RPC_IF_HANDLE __MIDL_itf_certview_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certview_0000_0000_v0_0_s_ifspec;

#ifndef __IEnumCERTVIEWCOLUMN_INTERFACE_DEFINED__
#define __IEnumCERTVIEWCOLUMN_INTERFACE_DEFINED__

/* interface IEnumCERTVIEWCOLUMN */
/* [unique][helpstring][local][dual][uuid][object] */ 


EXTERN_C const IID IID_IEnumCERTVIEWCOLUMN;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9c735be2-57a5-11d1-9bdb-00c04fb683fa")
    IEnumCERTVIEWCOLUMN : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ LONG *pIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ BSTR *pstrOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [retval][out] */ BSTR *pstrOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [retval][out] */ LONG *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsIndexed( 
            /* [retval][out] */ LONG *pIndexed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxLength( 
            /* [retval][out] */ LONG *pMaxLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ LONG Flags,
            /* [retval][out] */ VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ LONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumCERTVIEWCOLUMN **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCERTVIEWCOLUMNVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumCERTVIEWCOLUMN * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumCERTVIEWCOLUMN * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEnumCERTVIEWCOLUMN * This,
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
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ LONG *pIndex);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ LONG *pType);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, IsIndexed)
        HRESULT ( STDMETHODCALLTYPE *IsIndexed )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ LONG *pIndexed);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, GetMaxLength)
        HRESULT ( STDMETHODCALLTYPE *GetMaxLength )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ LONG *pMaxLength);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [in] */ LONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumCERTVIEWCOLUMN * This);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWCOLUMN, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumCERTVIEWCOLUMN * This,
            /* [retval][out] */ IEnumCERTVIEWCOLUMN **ppenum);
        
        END_INTERFACE
    } IEnumCERTVIEWCOLUMNVtbl;

    interface IEnumCERTVIEWCOLUMN
    {
        CONST_VTBL struct IEnumCERTVIEWCOLUMNVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCERTVIEWCOLUMN_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCERTVIEWCOLUMN_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCERTVIEWCOLUMN_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCERTVIEWCOLUMN_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEnumCERTVIEWCOLUMN_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEnumCERTVIEWCOLUMN_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEnumCERTVIEWCOLUMN_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEnumCERTVIEWCOLUMN_Next(This,pIndex)	\
    ( (This)->lpVtbl -> Next(This,pIndex) ) 

#define IEnumCERTVIEWCOLUMN_GetName(This,pstrOut)	\
    ( (This)->lpVtbl -> GetName(This,pstrOut) ) 

#define IEnumCERTVIEWCOLUMN_GetDisplayName(This,pstrOut)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pstrOut) ) 

#define IEnumCERTVIEWCOLUMN_GetType(This,pType)	\
    ( (This)->lpVtbl -> GetType(This,pType) ) 

#define IEnumCERTVIEWCOLUMN_IsIndexed(This,pIndexed)	\
    ( (This)->lpVtbl -> IsIndexed(This,pIndexed) ) 

#define IEnumCERTVIEWCOLUMN_GetMaxLength(This,pMaxLength)	\
    ( (This)->lpVtbl -> GetMaxLength(This,pMaxLength) ) 

#define IEnumCERTVIEWCOLUMN_GetValue(This,Flags,pvarValue)	\
    ( (This)->lpVtbl -> GetValue(This,Flags,pvarValue) ) 

#define IEnumCERTVIEWCOLUMN_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCERTVIEWCOLUMN_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCERTVIEWCOLUMN_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCERTVIEWCOLUMN_INTERFACE_DEFINED__ */


#ifndef __IEnumCERTVIEWATTRIBUTE_INTERFACE_DEFINED__
#define __IEnumCERTVIEWATTRIBUTE_INTERFACE_DEFINED__

/* interface IEnumCERTVIEWATTRIBUTE */
/* [unique][helpstring][local][dual][uuid][object] */ 


EXTERN_C const IID IID_IEnumCERTVIEWATTRIBUTE;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e77db656-7653-11d1-9bde-00c04fb683fa")
    IEnumCERTVIEWATTRIBUTE : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ LONG *pIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ BSTR *pstrOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [retval][out] */ BSTR *pstrOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ LONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumCERTVIEWATTRIBUTE **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCERTVIEWATTRIBUTEVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumCERTVIEWATTRIBUTE * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumCERTVIEWATTRIBUTE * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEnumCERTVIEWATTRIBUTE * This,
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
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWATTRIBUTE, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [retval][out] */ LONG *pIndex);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWATTRIBUTE, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [retval][out] */ BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWATTRIBUTE, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [retval][out] */ BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWATTRIBUTE, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [in] */ LONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWATTRIBUTE, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumCERTVIEWATTRIBUTE * This);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWATTRIBUTE, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumCERTVIEWATTRIBUTE * This,
            /* [retval][out] */ IEnumCERTVIEWATTRIBUTE **ppenum);
        
        END_INTERFACE
    } IEnumCERTVIEWATTRIBUTEVtbl;

    interface IEnumCERTVIEWATTRIBUTE
    {
        CONST_VTBL struct IEnumCERTVIEWATTRIBUTEVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCERTVIEWATTRIBUTE_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCERTVIEWATTRIBUTE_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCERTVIEWATTRIBUTE_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCERTVIEWATTRIBUTE_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEnumCERTVIEWATTRIBUTE_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEnumCERTVIEWATTRIBUTE_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEnumCERTVIEWATTRIBUTE_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEnumCERTVIEWATTRIBUTE_Next(This,pIndex)	\
    ( (This)->lpVtbl -> Next(This,pIndex) ) 

#define IEnumCERTVIEWATTRIBUTE_GetName(This,pstrOut)	\
    ( (This)->lpVtbl -> GetName(This,pstrOut) ) 

#define IEnumCERTVIEWATTRIBUTE_GetValue(This,pstrOut)	\
    ( (This)->lpVtbl -> GetValue(This,pstrOut) ) 

#define IEnumCERTVIEWATTRIBUTE_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCERTVIEWATTRIBUTE_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCERTVIEWATTRIBUTE_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCERTVIEWATTRIBUTE_INTERFACE_DEFINED__ */


#ifndef __IEnumCERTVIEWEXTENSION_INTERFACE_DEFINED__
#define __IEnumCERTVIEWEXTENSION_INTERFACE_DEFINED__

/* interface IEnumCERTVIEWEXTENSION */
/* [unique][helpstring][local][dual][uuid][object] */ 


EXTERN_C const IID IID_IEnumCERTVIEWEXTENSION;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e7dd1466-7653-11d1-9bde-00c04fb683fa")
    IEnumCERTVIEWEXTENSION : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ LONG *pIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][out] */ BSTR *pstrOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [retval][out] */ LONG *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ LONG Type,
            /* [in] */ LONG Flags,
            /* [retval][out] */ VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ LONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumCERTVIEWEXTENSION **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCERTVIEWEXTENSIONVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumCERTVIEWEXTENSION * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumCERTVIEWEXTENSION * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEnumCERTVIEWEXTENSION * This,
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
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [retval][out] */ LONG *pIndex);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [retval][out] */ BSTR *pstrOut);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [retval][out] */ LONG *pFlags);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [in] */ LONG Type,
            /* [in] */ LONG Flags,
            /* [retval][out] */ VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [in] */ LONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumCERTVIEWEXTENSION * This);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWEXTENSION, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumCERTVIEWEXTENSION * This,
            /* [retval][out] */ IEnumCERTVIEWEXTENSION **ppenum);
        
        END_INTERFACE
    } IEnumCERTVIEWEXTENSIONVtbl;

    interface IEnumCERTVIEWEXTENSION
    {
        CONST_VTBL struct IEnumCERTVIEWEXTENSIONVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCERTVIEWEXTENSION_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCERTVIEWEXTENSION_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCERTVIEWEXTENSION_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCERTVIEWEXTENSION_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEnumCERTVIEWEXTENSION_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEnumCERTVIEWEXTENSION_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEnumCERTVIEWEXTENSION_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEnumCERTVIEWEXTENSION_Next(This,pIndex)	\
    ( (This)->lpVtbl -> Next(This,pIndex) ) 

#define IEnumCERTVIEWEXTENSION_GetName(This,pstrOut)	\
    ( (This)->lpVtbl -> GetName(This,pstrOut) ) 

#define IEnumCERTVIEWEXTENSION_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define IEnumCERTVIEWEXTENSION_GetValue(This,Type,Flags,pvarValue)	\
    ( (This)->lpVtbl -> GetValue(This,Type,Flags,pvarValue) ) 

#define IEnumCERTVIEWEXTENSION_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCERTVIEWEXTENSION_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCERTVIEWEXTENSION_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCERTVIEWEXTENSION_INTERFACE_DEFINED__ */


#ifndef __IEnumCERTVIEWROW_INTERFACE_DEFINED__
#define __IEnumCERTVIEWROW_INTERFACE_DEFINED__

/* interface IEnumCERTVIEWROW */
/* [unique][helpstring][local][dual][uuid][object] */ 


EXTERN_C const IID IID_IEnumCERTVIEWROW;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d1157f4c-5af2-11d1-9bdc-00c04fb683fa")
    IEnumCERTVIEWROW : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [retval][out] */ LONG *pIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCertViewColumn( 
            /* [retval][out] */ IEnumCERTVIEWCOLUMN **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCertViewAttribute( 
            /* [in] */ LONG Flags,
            /* [retval][out] */ IEnumCERTVIEWATTRIBUTE **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCertViewExtension( 
            /* [in] */ LONG Flags,
            /* [retval][out] */ IEnumCERTVIEWEXTENSION **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ LONG celt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ IEnumCERTVIEWROW **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxIndex( 
            /* [retval][out] */ LONG *pIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumCERTVIEWROWVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumCERTVIEWROW * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumCERTVIEWROW * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumCERTVIEWROW * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IEnumCERTVIEWROW * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IEnumCERTVIEWROW * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IEnumCERTVIEWROW * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IEnumCERTVIEWROW * This,
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
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumCERTVIEWROW * This,
            /* [retval][out] */ LONG *pIndex);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, EnumCertViewColumn)
        HRESULT ( STDMETHODCALLTYPE *EnumCertViewColumn )( 
            IEnumCERTVIEWROW * This,
            /* [retval][out] */ IEnumCERTVIEWCOLUMN **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, EnumCertViewAttribute)
        HRESULT ( STDMETHODCALLTYPE *EnumCertViewAttribute )( 
            IEnumCERTVIEWROW * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ IEnumCERTVIEWATTRIBUTE **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, EnumCertViewExtension)
        HRESULT ( STDMETHODCALLTYPE *EnumCertViewExtension )( 
            IEnumCERTVIEWROW * This,
            /* [in] */ LONG Flags,
            /* [retval][out] */ IEnumCERTVIEWEXTENSION **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumCERTVIEWROW * This,
            /* [in] */ LONG celt);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumCERTVIEWROW * This);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumCERTVIEWROW * This,
            /* [retval][out] */ IEnumCERTVIEWROW **ppenum);
        
        DECLSPEC_XFGVIRT(IEnumCERTVIEWROW, GetMaxIndex)
        HRESULT ( STDMETHODCALLTYPE *GetMaxIndex )( 
            IEnumCERTVIEWROW * This,
            /* [retval][out] */ LONG *pIndex);
        
        END_INTERFACE
    } IEnumCERTVIEWROWVtbl;

    interface IEnumCERTVIEWROW
    {
        CONST_VTBL struct IEnumCERTVIEWROWVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumCERTVIEWROW_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumCERTVIEWROW_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumCERTVIEWROW_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumCERTVIEWROW_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IEnumCERTVIEWROW_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IEnumCERTVIEWROW_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IEnumCERTVIEWROW_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IEnumCERTVIEWROW_Next(This,pIndex)	\
    ( (This)->lpVtbl -> Next(This,pIndex) ) 

#define IEnumCERTVIEWROW_EnumCertViewColumn(This,ppenum)	\
    ( (This)->lpVtbl -> EnumCertViewColumn(This,ppenum) ) 

#define IEnumCERTVIEWROW_EnumCertViewAttribute(This,Flags,ppenum)	\
    ( (This)->lpVtbl -> EnumCertViewAttribute(This,Flags,ppenum) ) 

#define IEnumCERTVIEWROW_EnumCertViewExtension(This,Flags,ppenum)	\
    ( (This)->lpVtbl -> EnumCertViewExtension(This,Flags,ppenum) ) 

#define IEnumCERTVIEWROW_Skip(This,celt)	\
    ( (This)->lpVtbl -> Skip(This,celt) ) 

#define IEnumCERTVIEWROW_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumCERTVIEWROW_Clone(This,ppenum)	\
    ( (This)->lpVtbl -> Clone(This,ppenum) ) 

#define IEnumCERTVIEWROW_GetMaxIndex(This,pIndex)	\
    ( (This)->lpVtbl -> GetMaxIndex(This,pIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumCERTVIEWROW_INTERFACE_DEFINED__ */


#ifndef __ICertView_INTERFACE_DEFINED__
#define __ICertView_INTERFACE_DEFINED__

/* interface ICertView */
/* [unique][helpstring][local][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c3fac344-1e84-11d1-9bd6-00c04fb683fa")
    ICertView : public IDispatch
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenConnection( 
            /* [in] */ const BSTR strConfig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCertViewColumn( 
            /* [in] */ LONG fResultColumn,
            /* [retval][out] */ IEnumCERTVIEWCOLUMN **ppenum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnCount( 
            /* [in] */ LONG fResultColumn,
            /* [retval][out] */ LONG *pcColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnIndex( 
            /* [in] */ LONG fResultColumn,
            /* [in] */ const BSTR strColumnName,
            /* [retval][out] */ LONG *pColumnIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetResultColumnCount( 
            /* [in] */ LONG cResultColumn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetResultColumn( 
            /* [in] */ LONG ColumnIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRestriction( 
            /* [in] */ LONG ColumnIndex,
            /* [in] */ LONG SeekOperator,
            /* [in] */ LONG SortOrder,
            /* [in] */ const VARIANT *pvarValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenView( 
            /* [retval][out] */ IEnumCERTVIEWROW **ppenum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICertView * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICertView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICertView * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            ICertView * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            ICertView * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            ICertView * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertView * This,
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
        
        DECLSPEC_XFGVIRT(ICertView, OpenConnection)
        HRESULT ( STDMETHODCALLTYPE *OpenConnection )( 
            ICertView * This,
            /* [in] */ const BSTR strConfig);
        
        DECLSPEC_XFGVIRT(ICertView, EnumCertViewColumn)
        HRESULT ( STDMETHODCALLTYPE *EnumCertViewColumn )( 
            ICertView * This,
            /* [in] */ LONG fResultColumn,
            /* [retval][out] */ IEnumCERTVIEWCOLUMN **ppenum);
        
        DECLSPEC_XFGVIRT(ICertView, GetColumnCount)
        HRESULT ( STDMETHODCALLTYPE *GetColumnCount )( 
            ICertView * This,
            /* [in] */ LONG fResultColumn,
            /* [retval][out] */ LONG *pcColumn);
        
        DECLSPEC_XFGVIRT(ICertView, GetColumnIndex)
        HRESULT ( STDMETHODCALLTYPE *GetColumnIndex )( 
            ICertView * This,
            /* [in] */ LONG fResultColumn,
            /* [in] */ const BSTR strColumnName,
            /* [retval][out] */ LONG *pColumnIndex);
        
        DECLSPEC_XFGVIRT(ICertView, SetResultColumnCount)
        HRESULT ( STDMETHODCALLTYPE *SetResultColumnCount )( 
            ICertView * This,
            /* [in] */ LONG cResultColumn);
        
        DECLSPEC_XFGVIRT(ICertView, SetResultColumn)
        HRESULT ( STDMETHODCALLTYPE *SetResultColumn )( 
            ICertView * This,
            /* [in] */ LONG ColumnIndex);
        
        DECLSPEC_XFGVIRT(ICertView, SetRestriction)
        HRESULT ( STDMETHODCALLTYPE *SetRestriction )( 
            ICertView * This,
            /* [in] */ LONG ColumnIndex,
            /* [in] */ LONG SeekOperator,
            /* [in] */ LONG SortOrder,
            /* [in] */ const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertView, OpenView)
        HRESULT ( STDMETHODCALLTYPE *OpenView )( 
            ICertView * This,
            /* [retval][out] */ IEnumCERTVIEWROW **ppenum);
        
        END_INTERFACE
    } ICertViewVtbl;

    interface ICertView
    {
        CONST_VTBL struct ICertViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertView_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertView_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertView_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertView_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertView_OpenConnection(This,strConfig)	\
    ( (This)->lpVtbl -> OpenConnection(This,strConfig) ) 

#define ICertView_EnumCertViewColumn(This,fResultColumn,ppenum)	\
    ( (This)->lpVtbl -> EnumCertViewColumn(This,fResultColumn,ppenum) ) 

#define ICertView_GetColumnCount(This,fResultColumn,pcColumn)	\
    ( (This)->lpVtbl -> GetColumnCount(This,fResultColumn,pcColumn) ) 

#define ICertView_GetColumnIndex(This,fResultColumn,strColumnName,pColumnIndex)	\
    ( (This)->lpVtbl -> GetColumnIndex(This,fResultColumn,strColumnName,pColumnIndex) ) 

#define ICertView_SetResultColumnCount(This,cResultColumn)	\
    ( (This)->lpVtbl -> SetResultColumnCount(This,cResultColumn) ) 

#define ICertView_SetResultColumn(This,ColumnIndex)	\
    ( (This)->lpVtbl -> SetResultColumn(This,ColumnIndex) ) 

#define ICertView_SetRestriction(This,ColumnIndex,SeekOperator,SortOrder,pvarValue)	\
    ( (This)->lpVtbl -> SetRestriction(This,ColumnIndex,SeekOperator,SortOrder,pvarValue) ) 

#define ICertView_OpenView(This,ppenum)	\
    ( (This)->lpVtbl -> OpenView(This,ppenum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertView_INTERFACE_DEFINED__ */


#ifndef __ICertView2_INTERFACE_DEFINED__
#define __ICertView2_INTERFACE_DEFINED__

/* interface ICertView2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICertView2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d594b282-8851-4b61-9c66-3edadf848863")
    ICertView2 : public ICertView
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetTable( 
            /* [in] */ LONG Table) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICertView2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICertView2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICertView2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICertView2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICertView2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICertView2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICertView2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICertView2 * This,
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
        
        DECLSPEC_XFGVIRT(ICertView, OpenConnection)
        HRESULT ( STDMETHODCALLTYPE *OpenConnection )( 
            __RPC__in ICertView2 * This,
            /* [in] */ __RPC__in const BSTR strConfig);
        
        DECLSPEC_XFGVIRT(ICertView, EnumCertViewColumn)
        HRESULT ( STDMETHODCALLTYPE *EnumCertViewColumn )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG fResultColumn,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCERTVIEWCOLUMN **ppenum);
        
        DECLSPEC_XFGVIRT(ICertView, GetColumnCount)
        HRESULT ( STDMETHODCALLTYPE *GetColumnCount )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG fResultColumn,
            /* [retval][out] */ __RPC__out LONG *pcColumn);
        
        DECLSPEC_XFGVIRT(ICertView, GetColumnIndex)
        HRESULT ( STDMETHODCALLTYPE *GetColumnIndex )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG fResultColumn,
            /* [in] */ __RPC__in const BSTR strColumnName,
            /* [retval][out] */ __RPC__out LONG *pColumnIndex);
        
        DECLSPEC_XFGVIRT(ICertView, SetResultColumnCount)
        HRESULT ( STDMETHODCALLTYPE *SetResultColumnCount )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG cResultColumn);
        
        DECLSPEC_XFGVIRT(ICertView, SetResultColumn)
        HRESULT ( STDMETHODCALLTYPE *SetResultColumn )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG ColumnIndex);
        
        DECLSPEC_XFGVIRT(ICertView, SetRestriction)
        HRESULT ( STDMETHODCALLTYPE *SetRestriction )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG ColumnIndex,
            /* [in] */ LONG SeekOperator,
            /* [in] */ LONG SortOrder,
            /* [in] */ __RPC__in const VARIANT *pvarValue);
        
        DECLSPEC_XFGVIRT(ICertView, OpenView)
        HRESULT ( STDMETHODCALLTYPE *OpenView )( 
            __RPC__in ICertView2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumCERTVIEWROW **ppenum);
        
        DECLSPEC_XFGVIRT(ICertView2, SetTable)
        HRESULT ( STDMETHODCALLTYPE *SetTable )( 
            __RPC__in ICertView2 * This,
            /* [in] */ LONG Table);
        
        END_INTERFACE
    } ICertView2Vtbl;

    interface ICertView2
    {
        CONST_VTBL struct ICertView2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICertView2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICertView2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICertView2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICertView2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICertView2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICertView2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICertView2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICertView2_OpenConnection(This,strConfig)	\
    ( (This)->lpVtbl -> OpenConnection(This,strConfig) ) 

#define ICertView2_EnumCertViewColumn(This,fResultColumn,ppenum)	\
    ( (This)->lpVtbl -> EnumCertViewColumn(This,fResultColumn,ppenum) ) 

#define ICertView2_GetColumnCount(This,fResultColumn,pcColumn)	\
    ( (This)->lpVtbl -> GetColumnCount(This,fResultColumn,pcColumn) ) 

#define ICertView2_GetColumnIndex(This,fResultColumn,strColumnName,pColumnIndex)	\
    ( (This)->lpVtbl -> GetColumnIndex(This,fResultColumn,strColumnName,pColumnIndex) ) 

#define ICertView2_SetResultColumnCount(This,cResultColumn)	\
    ( (This)->lpVtbl -> SetResultColumnCount(This,cResultColumn) ) 

#define ICertView2_SetResultColumn(This,ColumnIndex)	\
    ( (This)->lpVtbl -> SetResultColumn(This,ColumnIndex) ) 

#define ICertView2_SetRestriction(This,ColumnIndex,SeekOperator,SortOrder,pvarValue)	\
    ( (This)->lpVtbl -> SetRestriction(This,ColumnIndex,SeekOperator,SortOrder,pvarValue) ) 

#define ICertView2_OpenView(This,ppenum)	\
    ( (This)->lpVtbl -> OpenView(This,ppenum) ) 


#define ICertView2_SetTable(This,Table)	\
    ( (This)->lpVtbl -> SetTable(This,Table) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICertView2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_certview_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_certview_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_certview_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


