

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

#ifndef __mshtmldiagnostics_h__
#define __mshtmldiagnostics_h__

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

#ifndef __IDiagnosticsScriptEngineSite_FWD_DEFINED__
#define __IDiagnosticsScriptEngineSite_FWD_DEFINED__
typedef interface IDiagnosticsScriptEngineSite IDiagnosticsScriptEngineSite;

#endif 	/* __IDiagnosticsScriptEngineSite_FWD_DEFINED__ */


#ifndef __IDiagnosticsScriptEngine_FWD_DEFINED__
#define __IDiagnosticsScriptEngine_FWD_DEFINED__
typedef interface IDiagnosticsScriptEngine IDiagnosticsScriptEngine;

#endif 	/* __IDiagnosticsScriptEngine_FWD_DEFINED__ */


#ifndef __IDiagnosticsScriptEngineProvider_FWD_DEFINED__
#define __IDiagnosticsScriptEngineProvider_FWD_DEFINED__
typedef interface IDiagnosticsScriptEngineProvider IDiagnosticsScriptEngineProvider;

#endif 	/* __IDiagnosticsScriptEngineProvider_FWD_DEFINED__ */


/* header files for imported files */
#include "ActivScp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mshtmldiagnostics_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// MSHTMLDiagnostics.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mshtmldiagnostics_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmldiagnostics_0000_0000_v0_0_s_ifspec;

#ifndef __IDiagnosticsScriptEngineSite_INTERFACE_DEFINED__
#define __IDiagnosticsScriptEngineSite_INTERFACE_DEFINED__

/* interface IDiagnosticsScriptEngineSite */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDiagnosticsScriptEngineSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510858-98b5-11cf-bb82-00aa00bdce0b")
    IDiagnosticsScriptEngineSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnMessage( 
            /* [annotation][size_is][in] */ 
            _In_reads_(ulDataCount)  LPCWSTR *pszData,
            /* [in] */ ULONG ulDataCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnScriptError( 
            /* [annotation][in] */ 
            _In_  IActiveScriptError *pScriptError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiagnosticsScriptEngineSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDiagnosticsScriptEngineSite * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDiagnosticsScriptEngineSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDiagnosticsScriptEngineSite * This);
        
        DECLSPEC_XFGVIRT(IDiagnosticsScriptEngineSite, OnMessage)
        HRESULT ( STDMETHODCALLTYPE *OnMessage )( 
            IDiagnosticsScriptEngineSite * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(ulDataCount)  LPCWSTR *pszData,
            /* [in] */ ULONG ulDataCount);
        
        DECLSPEC_XFGVIRT(IDiagnosticsScriptEngineSite, OnScriptError)
        HRESULT ( STDMETHODCALLTYPE *OnScriptError )( 
            IDiagnosticsScriptEngineSite * This,
            /* [annotation][in] */ 
            _In_  IActiveScriptError *pScriptError);
        
        END_INTERFACE
    } IDiagnosticsScriptEngineSiteVtbl;

    interface IDiagnosticsScriptEngineSite
    {
        CONST_VTBL struct IDiagnosticsScriptEngineSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiagnosticsScriptEngineSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiagnosticsScriptEngineSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiagnosticsScriptEngineSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiagnosticsScriptEngineSite_OnMessage(This,pszData,ulDataCount)	\
    ( (This)->lpVtbl -> OnMessage(This,pszData,ulDataCount) ) 

#define IDiagnosticsScriptEngineSite_OnScriptError(This,pScriptError)	\
    ( (This)->lpVtbl -> OnScriptError(This,pScriptError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiagnosticsScriptEngineSite_INTERFACE_DEFINED__ */


#ifndef __IDiagnosticsScriptEngine_INTERFACE_DEFINED__
#define __IDiagnosticsScriptEngine_INTERFACE_DEFINED__

/* interface IDiagnosticsScriptEngine */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDiagnosticsScriptEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("30510859-98b5-11cf-bb82-00aa00bdce0b")
    IDiagnosticsScriptEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EvaluateScript( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScript,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScriptName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FireScriptMessageEvent( 
            /* [annotation][size_is][in] */ 
            _In_reads_(ulPropertyCount)  LPCWSTR *pszNames,
            /* [annotation][size_is][in] */ 
            _In_reads_(ulPropertyCount)  LPCWSTR *pszValues,
            ULONG ulPropertyCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Detach( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiagnosticsScriptEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDiagnosticsScriptEngine * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDiagnosticsScriptEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDiagnosticsScriptEngine * This);
        
        DECLSPEC_XFGVIRT(IDiagnosticsScriptEngine, EvaluateScript)
        HRESULT ( STDMETHODCALLTYPE *EvaluateScript )( 
            IDiagnosticsScriptEngine * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScript,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScriptName);
        
        DECLSPEC_XFGVIRT(IDiagnosticsScriptEngine, FireScriptMessageEvent)
        HRESULT ( STDMETHODCALLTYPE *FireScriptMessageEvent )( 
            IDiagnosticsScriptEngine * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(ulPropertyCount)  LPCWSTR *pszNames,
            /* [annotation][size_is][in] */ 
            _In_reads_(ulPropertyCount)  LPCWSTR *pszValues,
            ULONG ulPropertyCount);
        
        DECLSPEC_XFGVIRT(IDiagnosticsScriptEngine, Detach)
        HRESULT ( STDMETHODCALLTYPE *Detach )( 
            IDiagnosticsScriptEngine * This);
        
        END_INTERFACE
    } IDiagnosticsScriptEngineVtbl;

    interface IDiagnosticsScriptEngine
    {
        CONST_VTBL struct IDiagnosticsScriptEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiagnosticsScriptEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiagnosticsScriptEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiagnosticsScriptEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiagnosticsScriptEngine_EvaluateScript(This,pszScript,pszScriptName)	\
    ( (This)->lpVtbl -> EvaluateScript(This,pszScript,pszScriptName) ) 

#define IDiagnosticsScriptEngine_FireScriptMessageEvent(This,pszNames,pszValues,ulPropertyCount)	\
    ( (This)->lpVtbl -> FireScriptMessageEvent(This,pszNames,pszValues,ulPropertyCount) ) 

#define IDiagnosticsScriptEngine_Detach(This)	\
    ( (This)->lpVtbl -> Detach(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiagnosticsScriptEngine_INTERFACE_DEFINED__ */


#ifndef __IDiagnosticsScriptEngineProvider_INTERFACE_DEFINED__
#define __IDiagnosticsScriptEngineProvider_INTERFACE_DEFINED__

/* interface IDiagnosticsScriptEngineProvider */
/* [local][uuid][unique][object] */ 


EXTERN_C const IID IID_IDiagnosticsScriptEngineProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3051085a-98b5-11cf-bb82-00aa00bdce0b")
    IDiagnosticsScriptEngineProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateDiagnosticsScriptEngine( 
            /* [annotation][in] */ 
            _In_opt_  IDiagnosticsScriptEngineSite *pScriptSite,
            /* [in] */ BOOL fDebuggingEnabled,
            /* [in] */ ULONG ulProcessId,
            /* [annotation][out] */ 
            _Out_  IDiagnosticsScriptEngine **ppEngine) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDiagnosticsScriptEngineProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDiagnosticsScriptEngineProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDiagnosticsScriptEngineProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDiagnosticsScriptEngineProvider * This);
        
        DECLSPEC_XFGVIRT(IDiagnosticsScriptEngineProvider, CreateDiagnosticsScriptEngine)
        HRESULT ( STDMETHODCALLTYPE *CreateDiagnosticsScriptEngine )( 
            IDiagnosticsScriptEngineProvider * This,
            /* [annotation][in] */ 
            _In_opt_  IDiagnosticsScriptEngineSite *pScriptSite,
            /* [in] */ BOOL fDebuggingEnabled,
            /* [in] */ ULONG ulProcessId,
            /* [annotation][out] */ 
            _Out_  IDiagnosticsScriptEngine **ppEngine);
        
        END_INTERFACE
    } IDiagnosticsScriptEngineProviderVtbl;

    interface IDiagnosticsScriptEngineProvider
    {
        CONST_VTBL struct IDiagnosticsScriptEngineProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDiagnosticsScriptEngineProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDiagnosticsScriptEngineProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDiagnosticsScriptEngineProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDiagnosticsScriptEngineProvider_CreateDiagnosticsScriptEngine(This,pScriptSite,fDebuggingEnabled,ulProcessId,ppEngine)	\
    ( (This)->lpVtbl -> CreateDiagnosticsScriptEngine(This,pScriptSite,fDebuggingEnabled,ulProcessId,ppEngine) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDiagnosticsScriptEngineProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mshtmldiagnostics_0000_0003 */
/* [local] */ 

#define SID_SDiagnosticsScriptEngineProvider IID_IDiagnosticsScriptEngineProvider
#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mshtmldiagnostics_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mshtmldiagnostics_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


