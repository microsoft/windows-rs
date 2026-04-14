

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

#ifndef __activdbg100_h__
#define __activdbg100_h__

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

#ifndef __IDebugApplicationNode100_FWD_DEFINED__
#define __IDebugApplicationNode100_FWD_DEFINED__
typedef interface IDebugApplicationNode100 IDebugApplicationNode100;

#endif 	/* __IDebugApplicationNode100_FWD_DEFINED__ */


#ifndef __IWebAppDiagnosticsSetup_FWD_DEFINED__
#define __IWebAppDiagnosticsSetup_FWD_DEFINED__
typedef interface IWebAppDiagnosticsSetup IWebAppDiagnosticsSetup;

#endif 	/* __IWebAppDiagnosticsSetup_FWD_DEFINED__ */


#ifndef __IRemoteDebugApplication110_FWD_DEFINED__
#define __IRemoteDebugApplication110_FWD_DEFINED__
typedef interface IRemoteDebugApplication110 IRemoteDebugApplication110;

#endif 	/* __IRemoteDebugApplication110_FWD_DEFINED__ */


#ifndef __IDebugApplication11032_FWD_DEFINED__
#define __IDebugApplication11032_FWD_DEFINED__
typedef interface IDebugApplication11032 IDebugApplication11032;

#endif 	/* __IDebugApplication11032_FWD_DEFINED__ */


#ifndef __IDebugApplication11064_FWD_DEFINED__
#define __IDebugApplication11064_FWD_DEFINED__
typedef interface IDebugApplication11064 IDebugApplication11064;

#endif 	/* __IDebugApplication11064_FWD_DEFINED__ */


#ifndef __IWebAppDiagnosticsObjectInitialization_FWD_DEFINED__
#define __IWebAppDiagnosticsObjectInitialization_FWD_DEFINED__
typedef interface IWebAppDiagnosticsObjectInitialization IWebAppDiagnosticsObjectInitialization;

#endif 	/* __IWebAppDiagnosticsObjectInitialization_FWD_DEFINED__ */


#ifndef __IActiveScriptWinRTErrorDebug_FWD_DEFINED__
#define __IActiveScriptWinRTErrorDebug_FWD_DEFINED__
typedef interface IActiveScriptWinRTErrorDebug IActiveScriptWinRTErrorDebug;

#endif 	/* __IActiveScriptWinRTErrorDebug_FWD_DEFINED__ */


#ifndef __IActiveScriptErrorDebug110_FWD_DEFINED__
#define __IActiveScriptErrorDebug110_FWD_DEFINED__
typedef interface IActiveScriptErrorDebug110 IActiveScriptErrorDebug110;

#endif 	/* __IActiveScriptErrorDebug110_FWD_DEFINED__ */


#ifndef __IDebugApplicationThreadEvents110_FWD_DEFINED__
#define __IDebugApplicationThreadEvents110_FWD_DEFINED__
typedef interface IDebugApplicationThreadEvents110 IDebugApplicationThreadEvents110;

#endif 	/* __IDebugApplicationThreadEvents110_FWD_DEFINED__ */


#ifndef __IDebugApplicationThread11032_FWD_DEFINED__
#define __IDebugApplicationThread11032_FWD_DEFINED__
typedef interface IDebugApplicationThread11032 IDebugApplicationThread11032;

#endif 	/* __IDebugApplicationThread11032_FWD_DEFINED__ */


#ifndef __IDebugApplicationThread11064_FWD_DEFINED__
#define __IDebugApplicationThread11064_FWD_DEFINED__
typedef interface IDebugApplicationThread11064 IDebugApplicationThread11064;

#endif 	/* __IDebugApplicationThread11064_FWD_DEFINED__ */


#ifndef __IRemoteDebugCriticalErrorEvent110_FWD_DEFINED__
#define __IRemoteDebugCriticalErrorEvent110_FWD_DEFINED__
typedef interface IRemoteDebugCriticalErrorEvent110 IRemoteDebugCriticalErrorEvent110;

#endif 	/* __IRemoteDebugCriticalErrorEvent110_FWD_DEFINED__ */


#ifndef __IScriptInvocationContext_FWD_DEFINED__
#define __IScriptInvocationContext_FWD_DEFINED__
typedef interface IScriptInvocationContext IScriptInvocationContext;

#endif 	/* __IScriptInvocationContext_FWD_DEFINED__ */


#ifndef __IDebugStackFrame110_FWD_DEFINED__
#define __IDebugStackFrame110_FWD_DEFINED__
typedef interface IDebugStackFrame110 IDebugStackFrame110;

#endif 	/* __IDebugStackFrame110_FWD_DEFINED__ */


#ifndef __IRemoteDebugInfoEvent110_FWD_DEFINED__
#define __IRemoteDebugInfoEvent110_FWD_DEFINED__
typedef interface IRemoteDebugInfoEvent110 IRemoteDebugInfoEvent110;

#endif 	/* __IRemoteDebugInfoEvent110_FWD_DEFINED__ */


/* header files for imported files */
#include "activdbg.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_activdbg100_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ActivDbg100.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=
//
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//
// Declarations for ActiveX Scripting authoring/Debugging.
//
#pragma once
typedef 
enum tagAPPLICATION_NODE_EVENT_FILTER
    {
        FILTER_EXCLUDE_NOTHING	= 0,
        FILTER_EXCLUDE_ANONYMOUS_CODE	= 0x1,
        FILTER_EXCLUDE_EVAL_CODE	= 0x2
    } 	APPLICATION_NODE_EVENT_FILTER;

typedef struct tagTEXT_DOCUMENT_ARRAY
    {
    DWORD dwCount;
    /* [size_is] */ IDebugDocumentText **Members;
    } 	TEXT_DOCUMENT_ARRAY;



extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0000_v0_0_s_ifspec;

#ifndef __IDebugApplicationNode100_INTERFACE_DEFINED__
#define __IDebugApplicationNode100_INTERFACE_DEFINED__

/* interface IDebugApplicationNode100 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationNode100;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90a7734e-841b-4f77-9384-a2891e76e7e2")
    IDebugApplicationNode100 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFilterForEventSink( 
            /* [in] */ DWORD dwCookie,
            /* [in] */ APPLICATION_NODE_EVENT_FILTER filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExcludedDocuments( 
            /* [in] */ APPLICATION_NODE_EVENT_FILTER filter,
            /* [out] */ __RPC__out TEXT_DOCUMENT_ARRAY *pDocuments) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryIsChildNode( 
            /* [in] */ __RPC__in_opt IDebugDocument *pSearchKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationNode100Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugApplicationNode100 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugApplicationNode100 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugApplicationNode100 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode100, SetFilterForEventSink)
        HRESULT ( STDMETHODCALLTYPE *SetFilterForEventSink )( 
            __RPC__in IDebugApplicationNode100 * This,
            /* [in] */ DWORD dwCookie,
            /* [in] */ APPLICATION_NODE_EVENT_FILTER filter);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode100, GetExcludedDocuments)
        HRESULT ( STDMETHODCALLTYPE *GetExcludedDocuments )( 
            __RPC__in IDebugApplicationNode100 * This,
            /* [in] */ APPLICATION_NODE_EVENT_FILTER filter,
            /* [out] */ __RPC__out TEXT_DOCUMENT_ARRAY *pDocuments);
        
        DECLSPEC_XFGVIRT(IDebugApplicationNode100, QueryIsChildNode)
        HRESULT ( STDMETHODCALLTYPE *QueryIsChildNode )( 
            __RPC__in IDebugApplicationNode100 * This,
            /* [in] */ __RPC__in_opt IDebugDocument *pSearchKey);
        
        END_INTERFACE
    } IDebugApplicationNode100Vtbl;

    interface IDebugApplicationNode100
    {
        CONST_VTBL struct IDebugApplicationNode100Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationNode100_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationNode100_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationNode100_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationNode100_SetFilterForEventSink(This,dwCookie,filter)	\
    ( (This)->lpVtbl -> SetFilterForEventSink(This,dwCookie,filter) ) 

#define IDebugApplicationNode100_GetExcludedDocuments(This,filter,pDocuments)	\
    ( (This)->lpVtbl -> GetExcludedDocuments(This,filter,pDocuments) ) 

#define IDebugApplicationNode100_QueryIsChildNode(This,pSearchKey)	\
    ( (This)->lpVtbl -> QueryIsChildNode(This,pSearchKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationNode100_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0001 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0001_v0_0_s_ifspec;

#ifndef __IWebAppDiagnosticsSetup_INTERFACE_DEFINED__
#define __IWebAppDiagnosticsSetup_INTERFACE_DEFINED__

/* interface IWebAppDiagnosticsSetup */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IWebAppDiagnosticsSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("379BFBE1-C6C9-432A-93E1-6D17656C538C")
    IWebAppDiagnosticsSetup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DiagnosticsSupported( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateObjectWithSiteAtWebApp( 
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ DWORD_PTR hPassToObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebAppDiagnosticsSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWebAppDiagnosticsSetup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWebAppDiagnosticsSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWebAppDiagnosticsSetup * This);
        
        DECLSPEC_XFGVIRT(IWebAppDiagnosticsSetup, DiagnosticsSupported)
        HRESULT ( STDMETHODCALLTYPE *DiagnosticsSupported )( 
            __RPC__in IWebAppDiagnosticsSetup * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IWebAppDiagnosticsSetup, CreateObjectWithSiteAtWebApp)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectWithSiteAtWebApp )( 
            __RPC__in IWebAppDiagnosticsSetup * This,
            /* [in] */ __RPC__in REFCLSID rclsid,
            /* [in] */ DWORD dwClsContext,
            /* [in] */ __RPC__in REFIID riid,
            /* [in] */ DWORD_PTR hPassToObject);
        
        END_INTERFACE
    } IWebAppDiagnosticsSetupVtbl;

    interface IWebAppDiagnosticsSetup
    {
        CONST_VTBL struct IWebAppDiagnosticsSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebAppDiagnosticsSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebAppDiagnosticsSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebAppDiagnosticsSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebAppDiagnosticsSetup_DiagnosticsSupported(This,pRetVal)	\
    ( (This)->lpVtbl -> DiagnosticsSupported(This,pRetVal) ) 

#define IWebAppDiagnosticsSetup_CreateObjectWithSiteAtWebApp(This,rclsid,dwClsContext,riid,hPassToObject)	\
    ( (This)->lpVtbl -> CreateObjectWithSiteAtWebApp(This,rclsid,dwClsContext,riid,hPassToObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebAppDiagnosticsSetup_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0002 */
/* [local] */ 


enum SCRIPT_DEBUGGER_OPTIONS
    {
        SDO_NONE	= 0,
        SDO_ENABLE_FIRST_CHANCE_EXCEPTIONS	= 0x1,
        SDO_ENABLE_WEB_WORKER_SUPPORT	= 0x2,
        SDO_ENABLE_NONUSER_CODE_SUPPORT	= 0x4,
        SDO_ENABLE_LIBRARY_STACK_FRAME	= 0x8
    } ;
DEFINE_ENUM_FLAG_OPERATORS(SCRIPT_DEBUGGER_OPTIONS)


extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0002_v0_0_s_ifspec;

#ifndef __IRemoteDebugApplication110_INTERFACE_DEFINED__
#define __IRemoteDebugApplication110_INTERFACE_DEFINED__

/* interface IRemoteDebugApplication110 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IRemoteDebugApplication110;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5FE005B-2836-485e-B1F9-89D91AA24FD4")
    IRemoteDebugApplication110 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDebuggerOptions( 
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS mask,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentDebuggerOptions( 
            /* [out] */ __RPC__out enum SCRIPT_DEBUGGER_OPTIONS *pCurrentOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMainThread( 
            /* [out] */ __RPC__deref_out_opt IRemoteDebugApplicationThread **ppThread) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDebugApplication110Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDebugApplication110 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDebugApplication110 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDebugApplication110 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, SetDebuggerOptions)
        HRESULT ( STDMETHODCALLTYPE *SetDebuggerOptions )( 
            __RPC__in IRemoteDebugApplication110 * This,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS mask,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS value);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, GetCurrentDebuggerOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentDebuggerOptions )( 
            __RPC__in IRemoteDebugApplication110 * This,
            /* [out] */ __RPC__out enum SCRIPT_DEBUGGER_OPTIONS *pCurrentOptions);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, GetMainThread)
        HRESULT ( STDMETHODCALLTYPE *GetMainThread )( 
            __RPC__in IRemoteDebugApplication110 * This,
            /* [out] */ __RPC__deref_out_opt IRemoteDebugApplicationThread **ppThread);
        
        END_INTERFACE
    } IRemoteDebugApplication110Vtbl;

    interface IRemoteDebugApplication110
    {
        CONST_VTBL struct IRemoteDebugApplication110Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDebugApplication110_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDebugApplication110_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDebugApplication110_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDebugApplication110_SetDebuggerOptions(This,mask,value)	\
    ( (This)->lpVtbl -> SetDebuggerOptions(This,mask,value) ) 

#define IRemoteDebugApplication110_GetCurrentDebuggerOptions(This,pCurrentOptions)	\
    ( (This)->lpVtbl -> GetCurrentDebuggerOptions(This,pCurrentOptions) ) 

#define IRemoteDebugApplication110_GetMainThread(This,ppThread)	\
    ( (This)->lpVtbl -> GetMainThread(This,ppThread) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDebugApplication110_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0003 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IDebugApplication110 IDebugApplication11064
#define IID_IDebugApplication110 IID_IDebugApplication11064
#else
#define IDebugApplication110 IDebugApplication11032
#define IID_IDebugApplication110 IID_IDebugApplication11032
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0003_v0_0_s_ifspec;

#ifndef __IDebugApplication11032_INTERFACE_DEFINED__
#define __IDebugApplication11032_INTERFACE_DEFINED__

/* interface IDebugApplication11032 */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplication11032;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BDB3B5DE-89F2-4E11-84A5-97445F941C7D")
    IDebugApplication11032 : public IRemoteDebugApplication110
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SynchronousCallInMainThread( 
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsynchronousCallInMainThread( 
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CallableWaitForHandles( 
            /* [in] */ DWORD handleCount,
            /* [size_is][in] */ const HANDLE *pHandles,
            /* [out] */ DWORD *pIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplication11032Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplication11032 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplication11032 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplication11032 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, SetDebuggerOptions)
        HRESULT ( STDMETHODCALLTYPE *SetDebuggerOptions )( 
            IDebugApplication11032 * This,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS mask,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS value);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, GetCurrentDebuggerOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentDebuggerOptions )( 
            IDebugApplication11032 * This,
            /* [out] */ enum SCRIPT_DEBUGGER_OPTIONS *pCurrentOptions);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, GetMainThread)
        HRESULT ( STDMETHODCALLTYPE *GetMainThread )( 
            IDebugApplication11032 * This,
            /* [out] */ IRemoteDebugApplicationThread **ppThread);
        
        DECLSPEC_XFGVIRT(IDebugApplication11032, SynchronousCallInMainThread)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallInMainThread )( 
            IDebugApplication11032 * This,
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplication11032, AsynchronousCallInMainThread)
        HRESULT ( STDMETHODCALLTYPE *AsynchronousCallInMainThread )( 
            IDebugApplication11032 * This,
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplication11032, CallableWaitForHandles)
        HRESULT ( STDMETHODCALLTYPE *CallableWaitForHandles )( 
            IDebugApplication11032 * This,
            /* [in] */ DWORD handleCount,
            /* [size_is][in] */ const HANDLE *pHandles,
            /* [out] */ DWORD *pIndex);
        
        END_INTERFACE
    } IDebugApplication11032Vtbl;

    interface IDebugApplication11032
    {
        CONST_VTBL struct IDebugApplication11032Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplication11032_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplication11032_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplication11032_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplication11032_SetDebuggerOptions(This,mask,value)	\
    ( (This)->lpVtbl -> SetDebuggerOptions(This,mask,value) ) 

#define IDebugApplication11032_GetCurrentDebuggerOptions(This,pCurrentOptions)	\
    ( (This)->lpVtbl -> GetCurrentDebuggerOptions(This,pCurrentOptions) ) 

#define IDebugApplication11032_GetMainThread(This,ppThread)	\
    ( (This)->lpVtbl -> GetMainThread(This,ppThread) ) 


#define IDebugApplication11032_SynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplication11032_AsynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> AsynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplication11032_CallableWaitForHandles(This,handleCount,pHandles,pIndex)	\
    ( (This)->lpVtbl -> CallableWaitForHandles(This,handleCount,pHandles,pIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplication11032_INTERFACE_DEFINED__ */


#ifndef __IDebugApplication11064_INTERFACE_DEFINED__
#define __IDebugApplication11064_INTERFACE_DEFINED__

/* interface IDebugApplication11064 */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplication11064;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2039D958-4EEB-496A-87BB-2E5201EADEEF")
    IDebugApplication11064 : public IRemoteDebugApplication110
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SynchronousCallInMainThread( 
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsynchronousCallInMainThread( 
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CallableWaitForHandles( 
            /* [in] */ DWORD handleCount,
            /* [size_is][in] */ const HANDLE *pHandles,
            /* [out] */ DWORD *pIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplication11064Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplication11064 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplication11064 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplication11064 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, SetDebuggerOptions)
        HRESULT ( STDMETHODCALLTYPE *SetDebuggerOptions )( 
            IDebugApplication11064 * This,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS mask,
            /* [in] */ enum SCRIPT_DEBUGGER_OPTIONS value);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, GetCurrentDebuggerOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentDebuggerOptions )( 
            IDebugApplication11064 * This,
            /* [out] */ enum SCRIPT_DEBUGGER_OPTIONS *pCurrentOptions);
        
        DECLSPEC_XFGVIRT(IRemoteDebugApplication110, GetMainThread)
        HRESULT ( STDMETHODCALLTYPE *GetMainThread )( 
            IDebugApplication11064 * This,
            /* [out] */ IRemoteDebugApplicationThread **ppThread);
        
        DECLSPEC_XFGVIRT(IDebugApplication11064, SynchronousCallInMainThread)
        HRESULT ( STDMETHODCALLTYPE *SynchronousCallInMainThread )( 
            IDebugApplication11064 * This,
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplication11064, AsynchronousCallInMainThread)
        HRESULT ( STDMETHODCALLTYPE *AsynchronousCallInMainThread )( 
            IDebugApplication11064 * This,
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3);
        
        DECLSPEC_XFGVIRT(IDebugApplication11064, CallableWaitForHandles)
        HRESULT ( STDMETHODCALLTYPE *CallableWaitForHandles )( 
            IDebugApplication11064 * This,
            /* [in] */ DWORD handleCount,
            /* [size_is][in] */ const HANDLE *pHandles,
            /* [out] */ DWORD *pIndex);
        
        END_INTERFACE
    } IDebugApplication11064Vtbl;

    interface IDebugApplication11064
    {
        CONST_VTBL struct IDebugApplication11064Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplication11064_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplication11064_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplication11064_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplication11064_SetDebuggerOptions(This,mask,value)	\
    ( (This)->lpVtbl -> SetDebuggerOptions(This,mask,value) ) 

#define IDebugApplication11064_GetCurrentDebuggerOptions(This,pCurrentOptions)	\
    ( (This)->lpVtbl -> GetCurrentDebuggerOptions(This,pCurrentOptions) ) 

#define IDebugApplication11064_GetMainThread(This,ppThread)	\
    ( (This)->lpVtbl -> GetMainThread(This,ppThread) ) 


#define IDebugApplication11064_SynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> SynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplication11064_AsynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> AsynchronousCallInMainThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#define IDebugApplication11064_CallableWaitForHandles(This,handleCount,pHandles,pIndex)	\
    ( (This)->lpVtbl -> CallableWaitForHandles(This,handleCount,pHandles,pIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplication11064_INTERFACE_DEFINED__ */


#ifndef __IWebAppDiagnosticsObjectInitialization_INTERFACE_DEFINED__
#define __IWebAppDiagnosticsObjectInitialization_INTERFACE_DEFINED__

/* interface IWebAppDiagnosticsObjectInitialization */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IWebAppDiagnosticsObjectInitialization;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("16FF3A42-A5F5-432B-B625-8E8E16F57E15")
    IWebAppDiagnosticsObjectInitialization : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [annotation][in] */ 
            _In_  HANDLE_PTR hPassedHandle,
            /* [annotation][in] */ 
            _In_  IUnknown *pDebugApplication) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebAppDiagnosticsObjectInitializationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebAppDiagnosticsObjectInitialization * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebAppDiagnosticsObjectInitialization * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebAppDiagnosticsObjectInitialization * This);
        
        DECLSPEC_XFGVIRT(IWebAppDiagnosticsObjectInitialization, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            IWebAppDiagnosticsObjectInitialization * This,
            /* [annotation][in] */ 
            _In_  HANDLE_PTR hPassedHandle,
            /* [annotation][in] */ 
            _In_  IUnknown *pDebugApplication);
        
        END_INTERFACE
    } IWebAppDiagnosticsObjectInitializationVtbl;

    interface IWebAppDiagnosticsObjectInitialization
    {
        CONST_VTBL struct IWebAppDiagnosticsObjectInitializationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebAppDiagnosticsObjectInitialization_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebAppDiagnosticsObjectInitialization_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebAppDiagnosticsObjectInitialization_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebAppDiagnosticsObjectInitialization_Initialize(This,hPassedHandle,pDebugApplication)	\
    ( (This)->lpVtbl -> Initialize(This,hPassedHandle,pDebugApplication) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebAppDiagnosticsObjectInitialization_INTERFACE_DEFINED__ */


#ifndef __IActiveScriptWinRTErrorDebug_INTERFACE_DEFINED__
#define __IActiveScriptWinRTErrorDebug_INTERFACE_DEFINED__

/* interface IActiveScriptWinRTErrorDebug */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptWinRTErrorDebug;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73A3F82A-0FE9-4B33-BA3B-FE095F697E0A")
    IActiveScriptWinRTErrorDebug : public IActiveScriptError
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRestrictedErrorString( 
            /* [out] */ __RPC__deref_out_opt BSTR *errorString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRestrictedErrorReference( 
            /* [out] */ __RPC__deref_out_opt BSTR *referenceString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCapabilitySid( 
            /* [out] */ __RPC__deref_out_opt BSTR *capabilitySid) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptWinRTErrorDebugVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetExceptionInfo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetExceptionInfo )( 
            IActiveScriptWinRTErrorDebug * This,
            /* [out] */ EXCEPINFO *pexcepinfo);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourcePosition)
        HRESULT ( STDMETHODCALLTYPE *GetSourcePosition )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This,
            /* [out] */ __RPC__out DWORD *pdwSourceContext,
            /* [out] */ __RPC__out ULONG *pulLineNumber,
            /* [out] */ __RPC__out LONG *plCharacterPosition);
        
        DECLSPEC_XFGVIRT(IActiveScriptError, GetSourceLineText)
        HRESULT ( STDMETHODCALLTYPE *GetSourceLineText )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSourceLine);
        
        DECLSPEC_XFGVIRT(IActiveScriptWinRTErrorDebug, GetRestrictedErrorString)
        HRESULT ( STDMETHODCALLTYPE *GetRestrictedErrorString )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt BSTR *errorString);
        
        DECLSPEC_XFGVIRT(IActiveScriptWinRTErrorDebug, GetRestrictedErrorReference)
        HRESULT ( STDMETHODCALLTYPE *GetRestrictedErrorReference )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt BSTR *referenceString);
        
        DECLSPEC_XFGVIRT(IActiveScriptWinRTErrorDebug, GetCapabilitySid)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilitySid )( 
            __RPC__in IActiveScriptWinRTErrorDebug * This,
            /* [out] */ __RPC__deref_out_opt BSTR *capabilitySid);
        
        END_INTERFACE
    } IActiveScriptWinRTErrorDebugVtbl;

    interface IActiveScriptWinRTErrorDebug
    {
        CONST_VTBL struct IActiveScriptWinRTErrorDebugVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptWinRTErrorDebug_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptWinRTErrorDebug_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptWinRTErrorDebug_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptWinRTErrorDebug_GetExceptionInfo(This,pexcepinfo)	\
    ( (This)->lpVtbl -> GetExceptionInfo(This,pexcepinfo) ) 

#define IActiveScriptWinRTErrorDebug_GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition)	\
    ( (This)->lpVtbl -> GetSourcePosition(This,pdwSourceContext,pulLineNumber,plCharacterPosition) ) 

#define IActiveScriptWinRTErrorDebug_GetSourceLineText(This,pbstrSourceLine)	\
    ( (This)->lpVtbl -> GetSourceLineText(This,pbstrSourceLine) ) 


#define IActiveScriptWinRTErrorDebug_GetRestrictedErrorString(This,errorString)	\
    ( (This)->lpVtbl -> GetRestrictedErrorString(This,errorString) ) 

#define IActiveScriptWinRTErrorDebug_GetRestrictedErrorReference(This,referenceString)	\
    ( (This)->lpVtbl -> GetRestrictedErrorReference(This,referenceString) ) 

#define IActiveScriptWinRTErrorDebug_GetCapabilitySid(This,capabilitySid)	\
    ( (This)->lpVtbl -> GetCapabilitySid(This,capabilitySid) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptWinRTErrorDebug_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0007 */
/* [local] */ 

typedef 
enum tagSCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND
    {
        ETK_FIRST_CHANCE	= 0,
        ETK_USER_UNHANDLED	= 0x1,
        ETK_UNHANDLED	= 0x2
    } 	SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND;



extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0007_v0_0_s_ifspec;

#ifndef __IActiveScriptErrorDebug110_INTERFACE_DEFINED__
#define __IActiveScriptErrorDebug110_INTERFACE_DEFINED__

/* interface IActiveScriptErrorDebug110 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IActiveScriptErrorDebug110;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("516E42B6-89A8-4530-937B-5F0708431442")
    IActiveScriptErrorDebug110 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetExceptionThrownKind( 
            /* [out] */ __RPC__out SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND *pExceptionKind) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IActiveScriptErrorDebug110Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IActiveScriptErrorDebug110 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IActiveScriptErrorDebug110 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IActiveScriptErrorDebug110 * This);
        
        DECLSPEC_XFGVIRT(IActiveScriptErrorDebug110, GetExceptionThrownKind)
        HRESULT ( STDMETHODCALLTYPE *GetExceptionThrownKind )( 
            __RPC__in IActiveScriptErrorDebug110 * This,
            /* [out] */ __RPC__out SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND *pExceptionKind);
        
        END_INTERFACE
    } IActiveScriptErrorDebug110Vtbl;

    interface IActiveScriptErrorDebug110
    {
        CONST_VTBL struct IActiveScriptErrorDebug110Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IActiveScriptErrorDebug110_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IActiveScriptErrorDebug110_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IActiveScriptErrorDebug110_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IActiveScriptErrorDebug110_GetExceptionThrownKind(This,pExceptionKind)	\
    ( (This)->lpVtbl -> GetExceptionThrownKind(This,pExceptionKind) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IActiveScriptErrorDebug110_INTERFACE_DEFINED__ */


#ifndef __IDebugApplicationThreadEvents110_INTERFACE_DEFINED__
#define __IDebugApplicationThreadEvents110_INTERFACE_DEFINED__

/* interface IDebugApplicationThreadEvents110 */
/* [unique][local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationThreadEvents110;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("84E5E468-D5DA-48A8-83F4-40366429007B")
    IDebugApplicationThreadEvents110 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSuspendForBreakPoint( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnResumeFromBreakPoint( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnThreadRequestComplete( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnBeginThreadRequest( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationThreadEvents110Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplicationThreadEvents110 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplicationThreadEvents110 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplicationThreadEvents110 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThreadEvents110, OnSuspendForBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *OnSuspendForBreakPoint )( 
            IDebugApplicationThreadEvents110 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThreadEvents110, OnResumeFromBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *OnResumeFromBreakPoint )( 
            IDebugApplicationThreadEvents110 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThreadEvents110, OnThreadRequestComplete)
        HRESULT ( STDMETHODCALLTYPE *OnThreadRequestComplete )( 
            IDebugApplicationThreadEvents110 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThreadEvents110, OnBeginThreadRequest)
        HRESULT ( STDMETHODCALLTYPE *OnBeginThreadRequest )( 
            IDebugApplicationThreadEvents110 * This);
        
        END_INTERFACE
    } IDebugApplicationThreadEvents110Vtbl;

    interface IDebugApplicationThreadEvents110
    {
        CONST_VTBL struct IDebugApplicationThreadEvents110Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationThreadEvents110_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationThreadEvents110_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationThreadEvents110_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationThreadEvents110_OnSuspendForBreakPoint(This)	\
    ( (This)->lpVtbl -> OnSuspendForBreakPoint(This) ) 

#define IDebugApplicationThreadEvents110_OnResumeFromBreakPoint(This)	\
    ( (This)->lpVtbl -> OnResumeFromBreakPoint(This) ) 

#define IDebugApplicationThreadEvents110_OnThreadRequestComplete(This)	\
    ( (This)->lpVtbl -> OnThreadRequestComplete(This) ) 

#define IDebugApplicationThreadEvents110_OnBeginThreadRequest(This)	\
    ( (This)->lpVtbl -> OnBeginThreadRequest(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationThreadEvents110_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0009 */
/* [local] */ 

#ifndef DISABLE_ACTIVDBG_INTERFACE_WRAPPERS
#ifdef _WIN64
#define IDebugApplicationThread110 IDebugApplicationThread11064
#define IID_IDebugApplicationThread110 IID_IDebugApplicationThread11064
#else
#define IDebugApplicationThread110 IDebugApplicationThread11032
#define IID_IDebugApplicationThread110 IID_IDebugApplicationThread11032
#endif
#endif


extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0009_v0_0_s_ifspec;

#ifndef __IDebugApplicationThread11032_INTERFACE_DEFINED__
#define __IDebugApplicationThread11032_INTERFACE_DEFINED__

/* interface IDebugApplicationThread11032 */
/* [unique][local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationThread11032;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2194AC5C-6561-404A-A2E9-F57D72DE3702")
    IDebugApplicationThread11032 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetActiveThreadRequestCount( 
            /* [annotation][out] */ 
            _Out_  UINT *puiThreadRequests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSuspendedForBreakPoint( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsSuspended) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsThreadCallable( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsCallable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsynchronousCallIntoThread( 
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationThread11032Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplicationThread11032 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplicationThread11032 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplicationThread11032 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11032, GetActiveThreadRequestCount)
        HRESULT ( STDMETHODCALLTYPE *GetActiveThreadRequestCount )( 
            IDebugApplicationThread11032 * This,
            /* [annotation][out] */ 
            _Out_  UINT *puiThreadRequests);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11032, IsSuspendedForBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *IsSuspendedForBreakPoint )( 
            IDebugApplicationThread11032 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsSuspended);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11032, IsThreadCallable)
        HRESULT ( STDMETHODCALLTYPE *IsThreadCallable )( 
            IDebugApplicationThread11032 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsCallable);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11032, AsynchronousCallIntoThread)
        HRESULT ( STDMETHODCALLTYPE *AsynchronousCallIntoThread )( 
            IDebugApplicationThread11032 * This,
            /* [in] */ IDebugThreadCall32 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3);
        
        END_INTERFACE
    } IDebugApplicationThread11032Vtbl;

    interface IDebugApplicationThread11032
    {
        CONST_VTBL struct IDebugApplicationThread11032Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationThread11032_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationThread11032_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationThread11032_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationThread11032_GetActiveThreadRequestCount(This,puiThreadRequests)	\
    ( (This)->lpVtbl -> GetActiveThreadRequestCount(This,puiThreadRequests) ) 

#define IDebugApplicationThread11032_IsSuspendedForBreakPoint(This,pfIsSuspended)	\
    ( (This)->lpVtbl -> IsSuspendedForBreakPoint(This,pfIsSuspended) ) 

#define IDebugApplicationThread11032_IsThreadCallable(This,pfIsCallable)	\
    ( (This)->lpVtbl -> IsThreadCallable(This,pfIsCallable) ) 

#define IDebugApplicationThread11032_AsynchronousCallIntoThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> AsynchronousCallIntoThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationThread11032_INTERFACE_DEFINED__ */


#ifndef __IDebugApplicationThread11064_INTERFACE_DEFINED__
#define __IDebugApplicationThread11064_INTERFACE_DEFINED__

/* interface IDebugApplicationThread11064 */
/* [unique][local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugApplicationThread11064;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("420AA4CC-EFD8-4DAC-983B-47127826917D")
    IDebugApplicationThread11064 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetActiveThreadRequestCount( 
            /* [annotation][out] */ 
            _Out_  UINT *puiThreadRequests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSuspendedForBreakPoint( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsSuspended) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsThreadCallable( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsCallable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AsynchronousCallIntoThread( 
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugApplicationThread11064Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDebugApplicationThread11064 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDebugApplicationThread11064 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDebugApplicationThread11064 * This);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11064, GetActiveThreadRequestCount)
        HRESULT ( STDMETHODCALLTYPE *GetActiveThreadRequestCount )( 
            IDebugApplicationThread11064 * This,
            /* [annotation][out] */ 
            _Out_  UINT *puiThreadRequests);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11064, IsSuspendedForBreakPoint)
        HRESULT ( STDMETHODCALLTYPE *IsSuspendedForBreakPoint )( 
            IDebugApplicationThread11064 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsSuspended);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11064, IsThreadCallable)
        HRESULT ( STDMETHODCALLTYPE *IsThreadCallable )( 
            IDebugApplicationThread11064 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsCallable);
        
        DECLSPEC_XFGVIRT(IDebugApplicationThread11064, AsynchronousCallIntoThread)
        HRESULT ( STDMETHODCALLTYPE *AsynchronousCallIntoThread )( 
            IDebugApplicationThread11064 * This,
            /* [in] */ IDebugThreadCall64 *pptc,
            /* [in] */ DWORD_PTR dwParam1,
            /* [in] */ DWORD_PTR dwParam2,
            /* [in] */ DWORD_PTR dwParam3);
        
        END_INTERFACE
    } IDebugApplicationThread11064Vtbl;

    interface IDebugApplicationThread11064
    {
        CONST_VTBL struct IDebugApplicationThread11064Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugApplicationThread11064_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugApplicationThread11064_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugApplicationThread11064_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugApplicationThread11064_GetActiveThreadRequestCount(This,puiThreadRequests)	\
    ( (This)->lpVtbl -> GetActiveThreadRequestCount(This,puiThreadRequests) ) 

#define IDebugApplicationThread11064_IsSuspendedForBreakPoint(This,pfIsSuspended)	\
    ( (This)->lpVtbl -> IsSuspendedForBreakPoint(This,pfIsSuspended) ) 

#define IDebugApplicationThread11064_IsThreadCallable(This,pfIsCallable)	\
    ( (This)->lpVtbl -> IsThreadCallable(This,pfIsCallable) ) 

#define IDebugApplicationThread11064_AsynchronousCallIntoThread(This,pptc,dwParam1,dwParam2,dwParam3)	\
    ( (This)->lpVtbl -> AsynchronousCallIntoThread(This,pptc,dwParam1,dwParam2,dwParam3) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugApplicationThread11064_INTERFACE_DEFINED__ */


#ifndef __IRemoteDebugCriticalErrorEvent110_INTERFACE_DEFINED__
#define __IRemoteDebugCriticalErrorEvent110_INTERFACE_DEFINED__

/* interface IRemoteDebugCriticalErrorEvent110 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IRemoteDebugCriticalErrorEvent110;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2f69c611-6b14-47e8-9260-4bb7c52f504b")
    IRemoteDebugCriticalErrorEvent110 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetErrorInfo( 
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSource,
            /* [out] */ __RPC__out int *pMessageId,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrMessage,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppLocation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDebugCriticalErrorEvent110Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDebugCriticalErrorEvent110 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDebugCriticalErrorEvent110 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDebugCriticalErrorEvent110 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugCriticalErrorEvent110, GetErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *GetErrorInfo )( 
            __RPC__in IRemoteDebugCriticalErrorEvent110 * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrSource,
            /* [out] */ __RPC__out int *pMessageId,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrMessage,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppLocation);
        
        END_INTERFACE
    } IRemoteDebugCriticalErrorEvent110Vtbl;

    interface IRemoteDebugCriticalErrorEvent110
    {
        CONST_VTBL struct IRemoteDebugCriticalErrorEvent110Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDebugCriticalErrorEvent110_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDebugCriticalErrorEvent110_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDebugCriticalErrorEvent110_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDebugCriticalErrorEvent110_GetErrorInfo(This,pbstrSource,pMessageId,pbstrMessage,ppLocation)	\
    ( (This)->lpVtbl -> GetErrorInfo(This,pbstrSource,pMessageId,pbstrMessage,ppLocation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDebugCriticalErrorEvent110_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0012 */
/* [local] */ 

typedef 
enum tagSCRIPT_INVOCATION_CONTEXT_TYPE
    {
        SICT_Event	= 0,
        SICT_SetTimeout	= ( SICT_Event + 1 ) ,
        SICT_SetInterval	= ( SICT_SetTimeout + 1 ) ,
        SICT_SetImmediate	= ( SICT_SetInterval + 1 ) ,
        SICT_RequestAnimationFrame	= ( SICT_SetImmediate + 1 ) ,
        SICT_ToString	= ( SICT_RequestAnimationFrame + 1 ) ,
        SICT_MutationObserverCheckpoint	= ( SICT_ToString + 1 ) ,
        SICT_WWAExecUnsafeLocalFunction	= ( SICT_MutationObserverCheckpoint + 1 ) ,
        SICT_WWAExecAtPriority	= ( SICT_WWAExecUnsafeLocalFunction + 1 ) 
    } 	SCRIPT_INVOCATION_CONTEXT_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0012_v0_0_s_ifspec;

#ifndef __IScriptInvocationContext_INTERFACE_DEFINED__
#define __IScriptInvocationContext_INTERFACE_DEFINED__

/* interface IScriptInvocationContext */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IScriptInvocationContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5D7741B7-AF7E-4A2A-85E5-C77F4D0659FB")
    IScriptInvocationContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetContextType( 
            /* [out] */ __RPC__out SCRIPT_INVOCATION_CONTEXT_TYPE *pInvocationContextType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContextDescription( 
            /* [out] */ __RPC__deref_out_opt BSTR *pDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContextObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppContextObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScriptInvocationContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScriptInvocationContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScriptInvocationContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScriptInvocationContext * This);
        
        DECLSPEC_XFGVIRT(IScriptInvocationContext, GetContextType)
        HRESULT ( STDMETHODCALLTYPE *GetContextType )( 
            __RPC__in IScriptInvocationContext * This,
            /* [out] */ __RPC__out SCRIPT_INVOCATION_CONTEXT_TYPE *pInvocationContextType);
        
        DECLSPEC_XFGVIRT(IScriptInvocationContext, GetContextDescription)
        HRESULT ( STDMETHODCALLTYPE *GetContextDescription )( 
            __RPC__in IScriptInvocationContext * This,
            /* [out] */ __RPC__deref_out_opt BSTR *pDescription);
        
        DECLSPEC_XFGVIRT(IScriptInvocationContext, GetContextObject)
        HRESULT ( STDMETHODCALLTYPE *GetContextObject )( 
            __RPC__in IScriptInvocationContext * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppContextObject);
        
        END_INTERFACE
    } IScriptInvocationContextVtbl;

    interface IScriptInvocationContext
    {
        CONST_VTBL struct IScriptInvocationContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScriptInvocationContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScriptInvocationContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScriptInvocationContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScriptInvocationContext_GetContextType(This,pInvocationContextType)	\
    ( (This)->lpVtbl -> GetContextType(This,pInvocationContextType) ) 

#define IScriptInvocationContext_GetContextDescription(This,pDescription)	\
    ( (This)->lpVtbl -> GetContextDescription(This,pDescription) ) 

#define IScriptInvocationContext_GetContextObject(This,ppContextObject)	\
    ( (This)->lpVtbl -> GetContextObject(This,ppContextObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScriptInvocationContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0013 */
/* [local] */ 

typedef 
enum tagDEBUG_STACKFRAME_TYPE
    {
        DST_SCRIPT_FRAME	= 0,
        DST_INTERNAL_FRAME	= ( DST_SCRIPT_FRAME + 1 ) ,
        DST_INVOCATION_FRAME	= ( DST_INTERNAL_FRAME + 1 ) 
    } 	DEBUG_STACKFRAME_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0013_v0_0_s_ifspec;

#ifndef __IDebugStackFrame110_INTERFACE_DEFINED__
#define __IDebugStackFrame110_INTERFACE_DEFINED__

/* interface IDebugStackFrame110 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDebugStackFrame110;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4B509611-B6EA-4B24-ADCB-D0CCFD1A7E33")
    IDebugStackFrame110 : public IDebugStackFrame
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStackFrameType( 
            /* [out] */ __RPC__out DEBUG_STACKFRAME_TYPE *pStackFrameKind) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScriptInvocationContext( 
            /* [out] */ __RPC__deref_out_opt IScriptInvocationContext **ppInvocationContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDebugStackFrame110Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDebugStackFrame110 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDebugStackFrame110 * This);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetCodeContext)
        HRESULT ( STDMETHODCALLTYPE *GetCodeContext )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [out] */ __RPC__deref_out_opt IDebugCodeContext **ppcc);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetDescriptionString)
        HRESULT ( STDMETHODCALLTYPE *GetDescriptionString )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [in] */ BOOL fLong,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrDescription);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetLanguageString)
        HRESULT ( STDMETHODCALLTYPE *GetLanguageString )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [in] */ BOOL fLong,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrLanguage);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetThread)
        HRESULT ( STDMETHODCALLTYPE *GetThread )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [out] */ __RPC__deref_out_opt IDebugApplicationThread **ppat);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame, GetDebugProperty)
        HRESULT ( STDMETHODCALLTYPE *GetDebugProperty )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [out] */ __RPC__deref_out_opt IDebugProperty **ppDebugProp);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame110, GetStackFrameType)
        HRESULT ( STDMETHODCALLTYPE *GetStackFrameType )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [out] */ __RPC__out DEBUG_STACKFRAME_TYPE *pStackFrameKind);
        
        DECLSPEC_XFGVIRT(IDebugStackFrame110, GetScriptInvocationContext)
        HRESULT ( STDMETHODCALLTYPE *GetScriptInvocationContext )( 
            __RPC__in IDebugStackFrame110 * This,
            /* [out] */ __RPC__deref_out_opt IScriptInvocationContext **ppInvocationContext);
        
        END_INTERFACE
    } IDebugStackFrame110Vtbl;

    interface IDebugStackFrame110
    {
        CONST_VTBL struct IDebugStackFrame110Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDebugStackFrame110_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDebugStackFrame110_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDebugStackFrame110_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDebugStackFrame110_GetCodeContext(This,ppcc)	\
    ( (This)->lpVtbl -> GetCodeContext(This,ppcc) ) 

#define IDebugStackFrame110_GetDescriptionString(This,fLong,pbstrDescription)	\
    ( (This)->lpVtbl -> GetDescriptionString(This,fLong,pbstrDescription) ) 

#define IDebugStackFrame110_GetLanguageString(This,fLong,pbstrLanguage)	\
    ( (This)->lpVtbl -> GetLanguageString(This,fLong,pbstrLanguage) ) 

#define IDebugStackFrame110_GetThread(This,ppat)	\
    ( (This)->lpVtbl -> GetThread(This,ppat) ) 

#define IDebugStackFrame110_GetDebugProperty(This,ppDebugProp)	\
    ( (This)->lpVtbl -> GetDebugProperty(This,ppDebugProp) ) 


#define IDebugStackFrame110_GetStackFrameType(This,pStackFrameKind)	\
    ( (This)->lpVtbl -> GetStackFrameType(This,pStackFrameKind) ) 

#define IDebugStackFrame110_GetScriptInvocationContext(This,ppInvocationContext)	\
    ( (This)->lpVtbl -> GetScriptInvocationContext(This,ppInvocationContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDebugStackFrame110_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0014 */
/* [local] */ 

typedef 
enum tagDEBUG_EVENT_INFO_TYPE
    {
        DEIT_GENERAL	= 0,
        DEIT_ASMJS_IN_DEBUGGING	= ( DEIT_GENERAL + 1 ) ,
        DEIT_ASMJS_SUCCEEDED	= ( DEIT_ASMJS_IN_DEBUGGING + 1 ) ,
        DEIT_ASMJS_FAILED	= ( DEIT_ASMJS_SUCCEEDED + 1 ) 
    } 	DEBUG_EVENT_INFO_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0014_v0_0_s_ifspec;

#ifndef __IRemoteDebugInfoEvent110_INTERFACE_DEFINED__
#define __IRemoteDebugInfoEvent110_INTERFACE_DEFINED__

/* interface IRemoteDebugInfoEvent110 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IRemoteDebugInfoEvent110;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9FF56BB6-EB89-4C0F-8823-CC2A4C0B7F26")
    IRemoteDebugInfoEvent110 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEventInfo( 
            /* [out] */ __RPC__out DEBUG_EVENT_INFO_TYPE *pMessageType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrMessage,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUrl,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppLocation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRemoteDebugInfoEvent110Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRemoteDebugInfoEvent110 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRemoteDebugInfoEvent110 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRemoteDebugInfoEvent110 * This);
        
        DECLSPEC_XFGVIRT(IRemoteDebugInfoEvent110, GetEventInfo)
        HRESULT ( STDMETHODCALLTYPE *GetEventInfo )( 
            __RPC__in IRemoteDebugInfoEvent110 * This,
            /* [out] */ __RPC__out DEBUG_EVENT_INFO_TYPE *pMessageType,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrMessage,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrUrl,
            /* [out] */ __RPC__deref_out_opt IDebugDocumentContext **ppLocation);
        
        END_INTERFACE
    } IRemoteDebugInfoEvent110Vtbl;

    interface IRemoteDebugInfoEvent110
    {
        CONST_VTBL struct IRemoteDebugInfoEvent110Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRemoteDebugInfoEvent110_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRemoteDebugInfoEvent110_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRemoteDebugInfoEvent110_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRemoteDebugInfoEvent110_GetEventInfo(This,pMessageType,pbstrMessage,pbstrUrl,ppLocation)	\
    ( (This)->lpVtbl -> GetEventInfo(This,pMessageType,pbstrMessage,pbstrUrl,ppLocation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRemoteDebugInfoEvent110_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_activdbg100_0000_0015 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_activdbg100_0000_0015_v0_0_s_ifspec;

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


