

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

#ifndef __termmgr_h__
#define __termmgr_h__

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

#ifndef __ITTerminalControl_FWD_DEFINED__
#define __ITTerminalControl_FWD_DEFINED__
typedef interface ITTerminalControl ITTerminalControl;

#endif 	/* __ITTerminalControl_FWD_DEFINED__ */


#ifndef __ITPluggableTerminalInitialization_FWD_DEFINED__
#define __ITPluggableTerminalInitialization_FWD_DEFINED__
typedef interface ITPluggableTerminalInitialization ITPluggableTerminalInitialization;

#endif 	/* __ITPluggableTerminalInitialization_FWD_DEFINED__ */


#ifndef __ITTerminalManager_FWD_DEFINED__
#define __ITTerminalManager_FWD_DEFINED__
typedef interface ITTerminalManager ITTerminalManager;

#endif 	/* __ITTerminalManager_FWD_DEFINED__ */


#ifndef __ITTerminalManager2_FWD_DEFINED__
#define __ITTerminalManager2_FWD_DEFINED__
typedef interface ITTerminalManager2 ITTerminalManager2;

#endif 	/* __ITTerminalManager2_FWD_DEFINED__ */


#ifndef __ITPluggableTerminalClassRegistration_FWD_DEFINED__
#define __ITPluggableTerminalClassRegistration_FWD_DEFINED__
typedef interface ITPluggableTerminalClassRegistration ITPluggableTerminalClassRegistration;

#endif 	/* __ITPluggableTerminalClassRegistration_FWD_DEFINED__ */


#ifndef __ITPluggableTerminalSuperclassRegistration_FWD_DEFINED__
#define __ITPluggableTerminalSuperclassRegistration_FWD_DEFINED__
typedef interface ITPluggableTerminalSuperclassRegistration ITPluggableTerminalSuperclassRegistration;

#endif 	/* __ITPluggableTerminalSuperclassRegistration_FWD_DEFINED__ */


#ifndef __TerminalManager_FWD_DEFINED__
#define __TerminalManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class TerminalManager TerminalManager;
#else
typedef struct TerminalManager TerminalManager;
#endif /* __cplusplus */

#endif 	/* __TerminalManager_FWD_DEFINED__ */


#ifndef __PluggableSuperclassRegistration_FWD_DEFINED__
#define __PluggableSuperclassRegistration_FWD_DEFINED__

#ifdef __cplusplus
typedef class PluggableSuperclassRegistration PluggableSuperclassRegistration;
#else
typedef struct PluggableSuperclassRegistration PluggableSuperclassRegistration;
#endif /* __cplusplus */

#endif 	/* __PluggableSuperclassRegistration_FWD_DEFINED__ */


#ifndef __PluggableTerminalRegistration_FWD_DEFINED__
#define __PluggableTerminalRegistration_FWD_DEFINED__

#ifdef __cplusplus
typedef class PluggableTerminalRegistration PluggableTerminalRegistration;
#else
typedef struct PluggableTerminalRegistration PluggableTerminalRegistration;
#endif /* __cplusplus */

#endif 	/* __PluggableTerminalRegistration_FWD_DEFINED__ */


/* header files for imported files */
#include "Objsafe.h"
#include "tapi3if.h"
#include "tapi3ds.h"
#include "msp.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_termmgr_0000_0000 */
/* [local] */ 

/* Copyright (c) Microsoft Corporation. All rights reserved. */
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_termmgr_0000_0000_0001
    {
        TMGR_TD_CAPTURE	= 1,
        TMGR_TD_RENDER	= 2,
        TMGR_TD_BOTH	= 3
    } 	TMGR_DIRECTION;

#define	CLSID_String_VideoSuperclass	( L"{714C6F8C-6244-4685-87B3-B91F3F9EADA7}" )

#define	CLSID_String_StreamingSuperclass	( L"{214F4ACC-AE0B-4464-8405-07029003F8E2}" )

#define	CLSID_String_FileSuperclass	( L"{B4790031-56DB-4d3e-88C8-6FFAAFA08A91}" )



extern RPC_IF_HANDLE __MIDL_itf_termmgr_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_termmgr_0000_0000_v0_0_s_ifspec;

#ifndef __ITTerminalControl_INTERFACE_DEFINED__
#define __ITTerminalControl_INTERFACE_DEFINED__

/* interface ITTerminalControl */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTerminalControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AED6483B-3304-11d2-86F1-006008B0E5D2")
    ITTerminalControl : public IUnknown
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_AddressHandle( 
            /* [out] */ __RPC__deref_out_opt MSP_HANDLE *phtAddress) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ConnectTerminal( 
            /* [in] */ __RPC__in_opt IGraphBuilder *pGraph,
            /* [in] */ DWORD dwTerminalDirection,
            /* [out][in] */ __RPC__inout DWORD *pdwNumPins,
            /* [out] */ __RPC__deref_out_opt IPin **ppPins) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CompleteConnectTerminal( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DisconnectTerminal( 
            /* [in] */ __RPC__in_opt IGraphBuilder *pGraph,
            /* [in] */ DWORD dwReserved) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RunRenderFilter( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopRenderFilter( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTerminalControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTerminalControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTerminalControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTerminalControl * This);
        
        DECLSPEC_XFGVIRT(ITTerminalControl, get_AddressHandle)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_AddressHandle )( 
            __RPC__in ITTerminalControl * This,
            /* [out] */ __RPC__deref_out_opt MSP_HANDLE *phtAddress);
        
        DECLSPEC_XFGVIRT(ITTerminalControl, ConnectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ConnectTerminal )( 
            __RPC__in ITTerminalControl * This,
            /* [in] */ __RPC__in_opt IGraphBuilder *pGraph,
            /* [in] */ DWORD dwTerminalDirection,
            /* [out][in] */ __RPC__inout DWORD *pdwNumPins,
            /* [out] */ __RPC__deref_out_opt IPin **ppPins);
        
        DECLSPEC_XFGVIRT(ITTerminalControl, CompleteConnectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CompleteConnectTerminal )( 
            __RPC__in ITTerminalControl * This);
        
        DECLSPEC_XFGVIRT(ITTerminalControl, DisconnectTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DisconnectTerminal )( 
            __RPC__in ITTerminalControl * This,
            /* [in] */ __RPC__in_opt IGraphBuilder *pGraph,
            /* [in] */ DWORD dwReserved);
        
        DECLSPEC_XFGVIRT(ITTerminalControl, RunRenderFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RunRenderFilter )( 
            __RPC__in ITTerminalControl * This);
        
        DECLSPEC_XFGVIRT(ITTerminalControl, StopRenderFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopRenderFilter )( 
            __RPC__in ITTerminalControl * This);
        
        END_INTERFACE
    } ITTerminalControlVtbl;

    interface ITTerminalControl
    {
        CONST_VTBL struct ITTerminalControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTerminalControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTerminalControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTerminalControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTerminalControl_get_AddressHandle(This,phtAddress)	\
    ( (This)->lpVtbl -> get_AddressHandle(This,phtAddress) ) 

#define ITTerminalControl_ConnectTerminal(This,pGraph,dwTerminalDirection,pdwNumPins,ppPins)	\
    ( (This)->lpVtbl -> ConnectTerminal(This,pGraph,dwTerminalDirection,pdwNumPins,ppPins) ) 

#define ITTerminalControl_CompleteConnectTerminal(This)	\
    ( (This)->lpVtbl -> CompleteConnectTerminal(This) ) 

#define ITTerminalControl_DisconnectTerminal(This,pGraph,dwReserved)	\
    ( (This)->lpVtbl -> DisconnectTerminal(This,pGraph,dwReserved) ) 

#define ITTerminalControl_RunRenderFilter(This)	\
    ( (This)->lpVtbl -> RunRenderFilter(This) ) 

#define ITTerminalControl_StopRenderFilter(This)	\
    ( (This)->lpVtbl -> StopRenderFilter(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTerminalControl_INTERFACE_DEFINED__ */


#ifndef __ITPluggableTerminalInitialization_INTERFACE_DEFINED__
#define __ITPluggableTerminalInitialization_INTERFACE_DEFINED__

/* interface ITPluggableTerminalInitialization */
/* [object][dual][helpstring][uuid] */ 


EXTERN_C const IID IID_ITPluggableTerminalInitialization;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AED6483C-3304-11d2-86F1-006008B0E5D2")
    ITPluggableTerminalInitialization : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InitializeDynamic( 
            /* [in] */ IID iidTerminalClass,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [in] */ __RPC__in MSP_HANDLE htAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalInitializationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalInitialization * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalInitialization * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalInitialization * This);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalInitialization, InitializeDynamic)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InitializeDynamic )( 
            __RPC__in ITPluggableTerminalInitialization * This,
            /* [in] */ IID iidTerminalClass,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [in] */ __RPC__in MSP_HANDLE htAddress);
        
        END_INTERFACE
    } ITPluggableTerminalInitializationVtbl;

    interface ITPluggableTerminalInitialization
    {
        CONST_VTBL struct ITPluggableTerminalInitializationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalInitialization_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalInitialization_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalInitialization_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalInitialization_InitializeDynamic(This,iidTerminalClass,dwMediaType,Direction,htAddress)	\
    ( (This)->lpVtbl -> InitializeDynamic(This,iidTerminalClass,dwMediaType,Direction,htAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalInitialization_INTERFACE_DEFINED__ */


#ifndef __ITTerminalManager_INTERFACE_DEFINED__
#define __ITTerminalManager_INTERFACE_DEFINED__

/* interface ITTerminalManager */
/* [hidden][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ITTerminalManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7170F2DE-9BE3-11D0-A009-00AA00B605A4")
    ITTerminalManager : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDynamicTerminalClasses( 
            /* [in] */ DWORD dwMediaTypes,
            /* [out][in] */ __RPC__inout DWORD *pdwNumClasses,
            /* [out] */ __RPC__out IID *pTerminalClasses) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateDynamicTerminal( 
            /* [in] */ __RPC__in_opt IUnknown *pOuterUnknown,
            /* [in] */ IID iidTerminalClass,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [in] */ __RPC__in MSP_HANDLE htAddress,
            /* [out] */ __RPC__deref_out_opt ITTerminal **ppTerminal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTerminalManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTerminalManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTerminalManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTerminalManager * This);
        
        DECLSPEC_XFGVIRT(ITTerminalManager, GetDynamicTerminalClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDynamicTerminalClasses )( 
            __RPC__in ITTerminalManager * This,
            /* [in] */ DWORD dwMediaTypes,
            /* [out][in] */ __RPC__inout DWORD *pdwNumClasses,
            /* [out] */ __RPC__out IID *pTerminalClasses);
        
        DECLSPEC_XFGVIRT(ITTerminalManager, CreateDynamicTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDynamicTerminal )( 
            __RPC__in ITTerminalManager * This,
            /* [in] */ __RPC__in_opt IUnknown *pOuterUnknown,
            /* [in] */ IID iidTerminalClass,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [in] */ __RPC__in MSP_HANDLE htAddress,
            /* [out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        END_INTERFACE
    } ITTerminalManagerVtbl;

    interface ITTerminalManager
    {
        CONST_VTBL struct ITTerminalManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTerminalManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTerminalManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTerminalManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTerminalManager_GetDynamicTerminalClasses(This,dwMediaTypes,pdwNumClasses,pTerminalClasses)	\
    ( (This)->lpVtbl -> GetDynamicTerminalClasses(This,dwMediaTypes,pdwNumClasses,pTerminalClasses) ) 

#define ITTerminalManager_CreateDynamicTerminal(This,pOuterUnknown,iidTerminalClass,dwMediaType,Direction,htAddress,ppTerminal)	\
    ( (This)->lpVtbl -> CreateDynamicTerminal(This,pOuterUnknown,iidTerminalClass,dwMediaType,Direction,htAddress,ppTerminal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTerminalManager_INTERFACE_DEFINED__ */


#ifndef __ITTerminalManager2_INTERFACE_DEFINED__
#define __ITTerminalManager2_INTERFACE_DEFINED__

/* interface ITTerminalManager2 */
/* [object][hidden][unique][helpstring][uuid] */ 


EXTERN_C const IID IID_ITTerminalManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB33DEC6-B2C7-46E6-9ED1-498B91FA85AC")
    ITTerminalManager2 : public ITTerminalManager
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPluggableSuperclasses( 
            /* [out][in] */ __RPC__inout DWORD *pdwNumSuperclasses,
            /* [out] */ __RPC__out IID *pSuperclasses) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPluggableTerminalClasses( 
            /* [in] */ IID iidSuperclass,
            /* [in] */ DWORD dwMediaTypes,
            /* [out][in] */ __RPC__inout DWORD *pdwNumClasses,
            /* [out] */ __RPC__out IID *pTerminalClasses) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITTerminalManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITTerminalManager2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITTerminalManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITTerminalManager2 * This);
        
        DECLSPEC_XFGVIRT(ITTerminalManager, GetDynamicTerminalClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDynamicTerminalClasses )( 
            __RPC__in ITTerminalManager2 * This,
            /* [in] */ DWORD dwMediaTypes,
            /* [out][in] */ __RPC__inout DWORD *pdwNumClasses,
            /* [out] */ __RPC__out IID *pTerminalClasses);
        
        DECLSPEC_XFGVIRT(ITTerminalManager, CreateDynamicTerminal)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateDynamicTerminal )( 
            __RPC__in ITTerminalManager2 * This,
            /* [in] */ __RPC__in_opt IUnknown *pOuterUnknown,
            /* [in] */ IID iidTerminalClass,
            /* [in] */ DWORD dwMediaType,
            /* [in] */ TERMINAL_DIRECTION Direction,
            /* [in] */ __RPC__in MSP_HANDLE htAddress,
            /* [out] */ __RPC__deref_out_opt ITTerminal **ppTerminal);
        
        DECLSPEC_XFGVIRT(ITTerminalManager2, GetPluggableSuperclasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPluggableSuperclasses )( 
            __RPC__in ITTerminalManager2 * This,
            /* [out][in] */ __RPC__inout DWORD *pdwNumSuperclasses,
            /* [out] */ __RPC__out IID *pSuperclasses);
        
        DECLSPEC_XFGVIRT(ITTerminalManager2, GetPluggableTerminalClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPluggableTerminalClasses )( 
            __RPC__in ITTerminalManager2 * This,
            /* [in] */ IID iidSuperclass,
            /* [in] */ DWORD dwMediaTypes,
            /* [out][in] */ __RPC__inout DWORD *pdwNumClasses,
            /* [out] */ __RPC__out IID *pTerminalClasses);
        
        END_INTERFACE
    } ITTerminalManager2Vtbl;

    interface ITTerminalManager2
    {
        CONST_VTBL struct ITTerminalManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITTerminalManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITTerminalManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITTerminalManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITTerminalManager2_GetDynamicTerminalClasses(This,dwMediaTypes,pdwNumClasses,pTerminalClasses)	\
    ( (This)->lpVtbl -> GetDynamicTerminalClasses(This,dwMediaTypes,pdwNumClasses,pTerminalClasses) ) 

#define ITTerminalManager2_CreateDynamicTerminal(This,pOuterUnknown,iidTerminalClass,dwMediaType,Direction,htAddress,ppTerminal)	\
    ( (This)->lpVtbl -> CreateDynamicTerminal(This,pOuterUnknown,iidTerminalClass,dwMediaType,Direction,htAddress,ppTerminal) ) 


#define ITTerminalManager2_GetPluggableSuperclasses(This,pdwNumSuperclasses,pSuperclasses)	\
    ( (This)->lpVtbl -> GetPluggableSuperclasses(This,pdwNumSuperclasses,pSuperclasses) ) 

#define ITTerminalManager2_GetPluggableTerminalClasses(This,iidSuperclass,dwMediaTypes,pdwNumClasses,pTerminalClasses)	\
    ( (This)->lpVtbl -> GetPluggableTerminalClasses(This,iidSuperclass,dwMediaTypes,pdwNumClasses,pTerminalClasses) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITTerminalManager2_INTERFACE_DEFINED__ */


#ifndef __ITPluggableTerminalClassRegistration_INTERFACE_DEFINED__
#define __ITPluggableTerminalClassRegistration_INTERFACE_DEFINED__

/* interface ITPluggableTerminalClassRegistration */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITPluggableTerminalClassRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("924A3723-A00B-4f5f-9FEE-8E9AEB9E82AA")
    ITPluggableTerminalClassRegistration : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Company( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCompany) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Company( 
            /* [in] */ __RPC__in BSTR bstrCompany) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Version( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVersion) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Version( 
            /* [in] */ __RPC__in BSTR bstrVersion) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TerminalClass( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTerminalClass) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_TerminalClass( 
            /* [in] */ __RPC__in BSTR bstrTerminalClass) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_CLSID( 
            /* [in] */ __RPC__in BSTR bstrCLSID) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Direction( 
            /* [retval][out] */ __RPC__out TMGR_DIRECTION *pDirection) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Direction( 
            /* [in] */ TMGR_DIRECTION nDirection) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_MediaTypes( 
            /* [retval][out] */ __RPC__out long *pMediaTypes) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_MediaTypes( 
            /* [in] */ long nMediaTypes) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR bstrSuperclass) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in BSTR bstrSuperclass) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTerminalClassInfo( 
            /* [in] */ __RPC__in BSTR bstrSuperclass) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalClassRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalClassRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalClassRegistration * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPluggableTerminalClassRegistration * This,
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
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_Company)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Company )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCompany);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_Company)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Company )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrCompany);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_Version)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Version )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVersion);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_Version)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Version )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrVersion);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_TerminalClass)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalClass )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pTerminalClass);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_TerminalClass)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_TerminalClass )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrTerminalClass);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_CLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_CLSID)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CLSID )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrCLSID);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_Direction)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Direction )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__out TMGR_DIRECTION *pDirection);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_Direction)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Direction )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ TMGR_DIRECTION nDirection);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, get_MediaTypes)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_MediaTypes )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [retval][out] */ __RPC__out long *pMediaTypes);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, put_MediaTypes)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_MediaTypes )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ long nMediaTypes);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrSuperclass);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrSuperclass);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalClassRegistration, GetTerminalClassInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTerminalClassInfo )( 
            __RPC__in ITPluggableTerminalClassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrSuperclass);
        
        END_INTERFACE
    } ITPluggableTerminalClassRegistrationVtbl;

    interface ITPluggableTerminalClassRegistration
    {
        CONST_VTBL struct ITPluggableTerminalClassRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalClassRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalClassRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalClassRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalClassRegistration_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPluggableTerminalClassRegistration_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPluggableTerminalClassRegistration_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPluggableTerminalClassRegistration_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPluggableTerminalClassRegistration_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define ITPluggableTerminalClassRegistration_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define ITPluggableTerminalClassRegistration_get_Company(This,pCompany)	\
    ( (This)->lpVtbl -> get_Company(This,pCompany) ) 

#define ITPluggableTerminalClassRegistration_put_Company(This,bstrCompany)	\
    ( (This)->lpVtbl -> put_Company(This,bstrCompany) ) 

#define ITPluggableTerminalClassRegistration_get_Version(This,pVersion)	\
    ( (This)->lpVtbl -> get_Version(This,pVersion) ) 

#define ITPluggableTerminalClassRegistration_put_Version(This,bstrVersion)	\
    ( (This)->lpVtbl -> put_Version(This,bstrVersion) ) 

#define ITPluggableTerminalClassRegistration_get_TerminalClass(This,pTerminalClass)	\
    ( (This)->lpVtbl -> get_TerminalClass(This,pTerminalClass) ) 

#define ITPluggableTerminalClassRegistration_put_TerminalClass(This,bstrTerminalClass)	\
    ( (This)->lpVtbl -> put_TerminalClass(This,bstrTerminalClass) ) 

#define ITPluggableTerminalClassRegistration_get_CLSID(This,pCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,pCLSID) ) 

#define ITPluggableTerminalClassRegistration_put_CLSID(This,bstrCLSID)	\
    ( (This)->lpVtbl -> put_CLSID(This,bstrCLSID) ) 

#define ITPluggableTerminalClassRegistration_get_Direction(This,pDirection)	\
    ( (This)->lpVtbl -> get_Direction(This,pDirection) ) 

#define ITPluggableTerminalClassRegistration_put_Direction(This,nDirection)	\
    ( (This)->lpVtbl -> put_Direction(This,nDirection) ) 

#define ITPluggableTerminalClassRegistration_get_MediaTypes(This,pMediaTypes)	\
    ( (This)->lpVtbl -> get_MediaTypes(This,pMediaTypes) ) 

#define ITPluggableTerminalClassRegistration_put_MediaTypes(This,nMediaTypes)	\
    ( (This)->lpVtbl -> put_MediaTypes(This,nMediaTypes) ) 

#define ITPluggableTerminalClassRegistration_Add(This,bstrSuperclass)	\
    ( (This)->lpVtbl -> Add(This,bstrSuperclass) ) 

#define ITPluggableTerminalClassRegistration_Delete(This,bstrSuperclass)	\
    ( (This)->lpVtbl -> Delete(This,bstrSuperclass) ) 

#define ITPluggableTerminalClassRegistration_GetTerminalClassInfo(This,bstrSuperclass)	\
    ( (This)->lpVtbl -> GetTerminalClassInfo(This,bstrSuperclass) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalClassRegistration_INTERFACE_DEFINED__ */


#ifndef __ITPluggableTerminalSuperclassRegistration_INTERFACE_DEFINED__
#define __ITPluggableTerminalSuperclassRegistration_INTERFACE_DEFINED__

/* interface ITPluggableTerminalSuperclassRegistration */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ITPluggableTerminalSuperclassRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60D3C08A-C13E-4195-9AB0-8DE768090F25")
    ITPluggableTerminalSuperclassRegistration : public IDispatch
    {
    public:
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR bstrName) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_CLSID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID) = 0;
        
        virtual /* [helpstring][propput][id] */ HRESULT STDMETHODCALLTYPE put_CLSID( 
            /* [in] */ __RPC__in BSTR bstrCLSID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTerminalSuperclassInfo( void) = 0;
        
        virtual /* [helpstring][propget][id] */ HRESULT STDMETHODCALLTYPE get_TerminalClasses( 
            /* [retval][out] */ __RPC__out VARIANT *pTerminals) = 0;
        
        virtual /* [hidden][helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumerateTerminalClasses( 
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppTerminals) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITPluggableTerminalSuperclassRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ITPluggableTerminalSuperclassRegistration * This,
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
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, get_Name)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pName);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, put_Name)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrName);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, get_CLSID)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_CLSID )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pCLSID);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, put_CLSID)
        /* [helpstring][propput][id] */ HRESULT ( STDMETHODCALLTYPE *put_CLSID )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [in] */ __RPC__in BSTR bstrCLSID);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, Add)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, GetTerminalSuperclassInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTerminalSuperclassInfo )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, get_TerminalClasses)
        /* [helpstring][propget][id] */ HRESULT ( STDMETHODCALLTYPE *get_TerminalClasses )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [retval][out] */ __RPC__out VARIANT *pTerminals);
        
        DECLSPEC_XFGVIRT(ITPluggableTerminalSuperclassRegistration, EnumerateTerminalClasses)
        /* [hidden][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumerateTerminalClasses )( 
            __RPC__in ITPluggableTerminalSuperclassRegistration * This,
            /* [retval][out] */ __RPC__deref_out_opt IEnumTerminalClass **ppTerminals);
        
        END_INTERFACE
    } ITPluggableTerminalSuperclassRegistrationVtbl;

    interface ITPluggableTerminalSuperclassRegistration
    {
        CONST_VTBL struct ITPluggableTerminalSuperclassRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITPluggableTerminalSuperclassRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITPluggableTerminalSuperclassRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITPluggableTerminalSuperclassRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITPluggableTerminalSuperclassRegistration_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ITPluggableTerminalSuperclassRegistration_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ITPluggableTerminalSuperclassRegistration_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ITPluggableTerminalSuperclassRegistration_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ITPluggableTerminalSuperclassRegistration_get_Name(This,pName)	\
    ( (This)->lpVtbl -> get_Name(This,pName) ) 

#define ITPluggableTerminalSuperclassRegistration_put_Name(This,bstrName)	\
    ( (This)->lpVtbl -> put_Name(This,bstrName) ) 

#define ITPluggableTerminalSuperclassRegistration_get_CLSID(This,pCLSID)	\
    ( (This)->lpVtbl -> get_CLSID(This,pCLSID) ) 

#define ITPluggableTerminalSuperclassRegistration_put_CLSID(This,bstrCLSID)	\
    ( (This)->lpVtbl -> put_CLSID(This,bstrCLSID) ) 

#define ITPluggableTerminalSuperclassRegistration_Add(This)	\
    ( (This)->lpVtbl -> Add(This) ) 

#define ITPluggableTerminalSuperclassRegistration_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define ITPluggableTerminalSuperclassRegistration_GetTerminalSuperclassInfo(This)	\
    ( (This)->lpVtbl -> GetTerminalSuperclassInfo(This) ) 

#define ITPluggableTerminalSuperclassRegistration_get_TerminalClasses(This,pTerminals)	\
    ( (This)->lpVtbl -> get_TerminalClasses(This,pTerminals) ) 

#define ITPluggableTerminalSuperclassRegistration_EnumerateTerminalClasses(This,ppTerminals)	\
    ( (This)->lpVtbl -> EnumerateTerminalClasses(This,ppTerminals) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITPluggableTerminalSuperclassRegistration_INTERFACE_DEFINED__ */



#ifndef __TERMMGRLib_LIBRARY_DEFINED__
#define __TERMMGRLib_LIBRARY_DEFINED__

/* library TERMMGRLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_TERMMGRLib;

EXTERN_C const CLSID CLSID_TerminalManager;

#ifdef __cplusplus

class DECLSPEC_UUID("7170F2E0-9BE3-11D0-A009-00AA00B605A4")
TerminalManager;
#endif

EXTERN_C const CLSID CLSID_PluggableSuperclassRegistration;

#ifdef __cplusplus

class DECLSPEC_UUID("BB918E32-2A5C-4986-AB40-1686A034390A")
PluggableSuperclassRegistration;
#endif

EXTERN_C const CLSID CLSID_PluggableTerminalRegistration;

#ifdef __cplusplus

class DECLSPEC_UUID("45234E3E-61CC-4311-A3AB-248082554482")
PluggableTerminalRegistration;
#endif
#endif /* __TERMMGRLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_termmgr_0000_0007 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_termmgr_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_termmgr_0000_0007_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


