

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

#ifndef __iewebdriver_h__
#define __iewebdriver_h__

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

#ifndef __IIEWebDriverSite_FWD_DEFINED__
#define __IIEWebDriverSite_FWD_DEFINED__
typedef interface IIEWebDriverSite IIEWebDriverSite;

#endif 	/* __IIEWebDriverSite_FWD_DEFINED__ */


#ifndef __IIEWebDriverManager_FWD_DEFINED__
#define __IIEWebDriverManager_FWD_DEFINED__
typedef interface IIEWebDriverManager IIEWebDriverManager;

#endif 	/* __IIEWebDriverManager_FWD_DEFINED__ */


#ifndef __IEWebDriverManager_FWD_DEFINED__
#define __IEWebDriverManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class IEWebDriverManager IEWebDriverManager;
#else
typedef struct IEWebDriverManager IEWebDriverManager;
#endif /* __cplusplus */

#endif 	/* __IEWebDriverManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iewebdriver_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// iewebdriver.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_iewebdriver_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iewebdriver_0000_0000_v0_0_s_ifspec;

#ifndef __IIEWebDriverSite_INTERFACE_DEFINED__
#define __IIEWebDriverSite_INTERFACE_DEFINED__

/* interface IIEWebDriverSite */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IIEWebDriverSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FFB84444-453D-4FBC-9F9D-8DB5C471EC75")
    IIEWebDriverSite : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE WindowOperation( 
            /* [in] */ DWORD operationCode,
            /* [in] */ DWORD hWnd) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE DetachWebdriver( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkWD) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCapabilityValue( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkWD,
            /* [in] */ __RPC__in LPWSTR capName,
            /* [out] */ __RPC__out VARIANT *capValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIEWebDriverSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIEWebDriverSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIEWebDriverSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIEWebDriverSite * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IIEWebDriverSite * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IIEWebDriverSite * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IIEWebDriverSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIEWebDriverSite * This,
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
        
        DECLSPEC_XFGVIRT(IIEWebDriverSite, WindowOperation)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *WindowOperation )( 
            __RPC__in IIEWebDriverSite * This,
            /* [in] */ DWORD operationCode,
            /* [in] */ DWORD hWnd);
        
        DECLSPEC_XFGVIRT(IIEWebDriverSite, DetachWebdriver)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *DetachWebdriver )( 
            __RPC__in IIEWebDriverSite * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkWD);
        
        DECLSPEC_XFGVIRT(IIEWebDriverSite, GetCapabilityValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCapabilityValue )( 
            __RPC__in IIEWebDriverSite * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkWD,
            /* [in] */ __RPC__in LPWSTR capName,
            /* [out] */ __RPC__out VARIANT *capValue);
        
        END_INTERFACE
    } IIEWebDriverSiteVtbl;

    interface IIEWebDriverSite
    {
        CONST_VTBL struct IIEWebDriverSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIEWebDriverSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIEWebDriverSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIEWebDriverSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIEWebDriverSite_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIEWebDriverSite_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIEWebDriverSite_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIEWebDriverSite_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIEWebDriverSite_WindowOperation(This,operationCode,hWnd)	\
    ( (This)->lpVtbl -> WindowOperation(This,operationCode,hWnd) ) 

#define IIEWebDriverSite_DetachWebdriver(This,pUnkWD)	\
    ( (This)->lpVtbl -> DetachWebdriver(This,pUnkWD) ) 

#define IIEWebDriverSite_GetCapabilityValue(This,pUnkWD,capName,capValue)	\
    ( (This)->lpVtbl -> GetCapabilityValue(This,pUnkWD,capName,capValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIEWebDriverSite_INTERFACE_DEFINED__ */


#ifndef __IIEWebDriverManager_INTERFACE_DEFINED__
#define __IIEWebDriverManager_INTERFACE_DEFINED__

/* interface IIEWebDriverManager */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IIEWebDriverManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BD1DC630-6590-4CA2-A293-6BC72B2438D8")
    IIEWebDriverManager : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ExecuteCommand( 
            /* [in] */ __RPC__in LPWSTR command,
            /* [out] */ __RPC__deref_out_opt LPWSTR *response) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIEWebDriverManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IIEWebDriverManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IIEWebDriverManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IIEWebDriverManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IIEWebDriverManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IIEWebDriverManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IIEWebDriverManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IIEWebDriverManager * This,
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
        
        DECLSPEC_XFGVIRT(IIEWebDriverManager, ExecuteCommand)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ExecuteCommand )( 
            __RPC__in IIEWebDriverManager * This,
            /* [in] */ __RPC__in LPWSTR command,
            /* [out] */ __RPC__deref_out_opt LPWSTR *response);
        
        END_INTERFACE
    } IIEWebDriverManagerVtbl;

    interface IIEWebDriverManager
    {
        CONST_VTBL struct IIEWebDriverManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIEWebDriverManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIEWebDriverManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIEWebDriverManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIEWebDriverManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IIEWebDriverManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IIEWebDriverManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IIEWebDriverManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IIEWebDriverManager_ExecuteCommand(This,command,response)	\
    ( (This)->lpVtbl -> ExecuteCommand(This,command,response) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIEWebDriverManager_INTERFACE_DEFINED__ */



#ifndef __IEWebDriverLib_LIBRARY_DEFINED__
#define __IEWebDriverLib_LIBRARY_DEFINED__

/* library IEWebDriverLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_IEWebDriverLib;

EXTERN_C const CLSID CLSID_IEWebDriverManager;

#ifdef __cplusplus

class DECLSPEC_UUID("90314AF2-5250-47B3-89D8-6295FC23BC22")
IEWebDriverManager;
#endif
#endif /* __IEWebDriverLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_iewebdriver_0000_0003 */
/* [local] */ 

#define SID_SIEWebDriverManager IID_IIEWebDriverManager
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_iewebdriver_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iewebdriver_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


