

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

#ifndef __wbemads_h__
#define __wbemads_h__

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

#ifndef __IWMIExtension_FWD_DEFINED__
#define __IWMIExtension_FWD_DEFINED__
typedef interface IWMIExtension IWMIExtension;

#endif 	/* __IWMIExtension_FWD_DEFINED__ */


#ifndef __WMIExtension_FWD_DEFINED__
#define __WMIExtension_FWD_DEFINED__

#ifdef __cplusplus
typedef class WMIExtension WMIExtension;
#else
typedef struct WMIExtension WMIExtension;
#endif /* __cplusplus */

#endif 	/* __WMIExtension_FWD_DEFINED__ */


#ifndef __IWMIExtension_FWD_DEFINED__
#define __IWMIExtension_FWD_DEFINED__
typedef interface IWMIExtension IWMIExtension;

#endif 	/* __IWMIExtension_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wbemdisp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wbemads_0000_0000 */
/* [local] */ 

/*******************************************************************************/
/*                                                                             */
/*    Copyright (c) Microsoft Corporation.  All rights reserved.               */
/*                                                                             */
/*    IDL source for WMI ADSI extension                                        */
/*                                                                             */
/*******************************************************************************/
#include <winapifamily.h>
#pragma region Desktop Family or WinMgmt Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT)


extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0000_v0_0_s_ifspec;


#ifndef __WMIEXTENSIONLib_LIBRARY_DEFINED__
#define __WMIEXTENSIONLib_LIBRARY_DEFINED__

/* library WMIEXTENSIONLib */
/* [helpstring][version][uuid] */ 



EXTERN_C const IID LIBID_WMIEXTENSIONLib;

#ifndef __IWMIExtension_INTERFACE_DEFINED__
#define __IWMIExtension_INTERFACE_DEFINED__

/* interface IWMIExtension */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IWMIExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("adc1f06e-5c7e-11d2-8b74-00104b2afb41")
    IWMIExtension : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_WMIObjectPath( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strWMIObjectPath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWMIObject( 
            /* [retval][out] */ __RPC__deref_out_opt ISWbemObject **objWMIObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWMIServices( 
            /* [retval][out] */ __RPC__deref_out_opt ISWbemServices **objWMIServices) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMIExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWMIExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWMIExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWMIExtension * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IWMIExtension * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IWMIExtension * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IWMIExtension * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IWMIExtension * This,
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
        
        DECLSPEC_XFGVIRT(IWMIExtension, get_WMIObjectPath)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_WMIObjectPath )( 
            __RPC__in IWMIExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *strWMIObjectPath);
        
        DECLSPEC_XFGVIRT(IWMIExtension, GetWMIObject)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIObject )( 
            __RPC__in IWMIExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt ISWbemObject **objWMIObject);
        
        DECLSPEC_XFGVIRT(IWMIExtension, GetWMIServices)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWMIServices )( 
            __RPC__in IWMIExtension * This,
            /* [retval][out] */ __RPC__deref_out_opt ISWbemServices **objWMIServices);
        
        END_INTERFACE
    } IWMIExtensionVtbl;

    interface IWMIExtension
    {
        CONST_VTBL struct IWMIExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMIExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMIExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMIExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMIExtension_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IWMIExtension_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IWMIExtension_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IWMIExtension_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IWMIExtension_get_WMIObjectPath(This,strWMIObjectPath)	\
    ( (This)->lpVtbl -> get_WMIObjectPath(This,strWMIObjectPath) ) 

#define IWMIExtension_GetWMIObject(This,objWMIObject)	\
    ( (This)->lpVtbl -> GetWMIObject(This,objWMIObject) ) 

#define IWMIExtension_GetWMIServices(This,objWMIServices)	\
    ( (This)->lpVtbl -> GetWMIServices(This,objWMIServices) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMIExtension_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_WMIExtension;

#ifdef __cplusplus

class DECLSPEC_UUID("f0975afe-5c7f-11d2-8b74-00104b2afb41")
WMIExtension;
#endif
#endif /* __WMIEXTENSIONLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wbemads_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WINMGMT) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wbemads_0000_0002_v0_0_s_ifspec;

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


