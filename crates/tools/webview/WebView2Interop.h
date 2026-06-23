

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.xx.xxxx */
/* at a redacted point in time
 */
/* Compiler settings for ../../edge_embedded_browser/client/win/current/WebView2Interop.idl:
    Oicf, W1, Zp8, env=Win64 (32b run), target_arch=AMD64 8.xx.xxxx 
    protocol : dce , ms_ext, c_ext, robust
    error checks: allocation ref bounds_check enum stub_data 
    VC __declspec() decoration level: 
         __declspec(uuid()), __declspec(selectany), __declspec(novtable)
         DECLSPEC_UUID(), MIDL_INTERFACE()
*/
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
#endif

#include "rpc.h"
#include "rpcndr.h"

#ifndef __RPCNDR_H_VERSION__
#error this stub requires an updated version of <rpcndr.h>
#endif /* __RPCNDR_H_VERSION__ */


#ifndef __WebView2Interop_h__
#define __WebView2Interop_h__

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

#ifndef __ICoreWebView2Interop_FWD_DEFINED__
#define __ICoreWebView2Interop_FWD_DEFINED__
typedef interface ICoreWebView2Interop ICoreWebView2Interop;

#endif 	/* __ICoreWebView2Interop_FWD_DEFINED__ */


#ifndef __ICoreWebView2Interop2_FWD_DEFINED__
#define __ICoreWebView2Interop2_FWD_DEFINED__
typedef interface ICoreWebView2Interop2 ICoreWebView2Interop2;

#endif 	/* __ICoreWebView2Interop2_FWD_DEFINED__ */


#ifndef __ICoreWebView2CompositionControllerInterop_FWD_DEFINED__
#define __ICoreWebView2CompositionControllerInterop_FWD_DEFINED__
typedef interface ICoreWebView2CompositionControllerInterop ICoreWebView2CompositionControllerInterop;

#endif 	/* __ICoreWebView2CompositionControllerInterop_FWD_DEFINED__ */


#ifndef __ICoreWebView2CompositionControllerInterop2_FWD_DEFINED__
#define __ICoreWebView2CompositionControllerInterop2_FWD_DEFINED__
typedef interface ICoreWebView2CompositionControllerInterop2 ICoreWebView2CompositionControllerInterop2;

#endif 	/* __ICoreWebView2CompositionControllerInterop2_FWD_DEFINED__ */


#ifndef __ICoreWebView2EnvironmentInterop_FWD_DEFINED__
#define __ICoreWebView2EnvironmentInterop_FWD_DEFINED__
typedef interface ICoreWebView2EnvironmentInterop ICoreWebView2EnvironmentInterop;

#endif 	/* __ICoreWebView2EnvironmentInterop_FWD_DEFINED__ */


#ifndef __ICoreWebView2CompositionControllerInterop3_FWD_DEFINED__
#define __ICoreWebView2CompositionControllerInterop3_FWD_DEFINED__
typedef interface ICoreWebView2CompositionControllerInterop3 ICoreWebView2CompositionControllerInterop3;

#endif 	/* __ICoreWebView2CompositionControllerInterop3_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "WebView2.h"

#ifdef __cplusplus
extern "C"{
#endif 



#ifndef __WebView2Interop_LIBRARY_DEFINED__
#define __WebView2Interop_LIBRARY_DEFINED__

/* library WebView2Interop */
/* [version][uuid] */ 







EXTERN_C const IID LIBID_WebView2Interop;

#ifndef __ICoreWebView2Interop_INTERFACE_DEFINED__
#define __ICoreWebView2Interop_INTERFACE_DEFINED__

/* interface ICoreWebView2Interop */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2Interop = {0x912b34a7,0xd10b,0x49c4,{0xaf,0x18,0x7c,0xb7,0xe6,0x04,0xe0,0x1a}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("912b34a7-d10b-49c4-af18-7cb7e604e01a")
    ICoreWebView2Interop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddHostObjectToScript( 
            /* [in] */ LPCWSTR name,
            /* [in] */ VARIANT *object) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2InteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2Interop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2Interop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2Interop * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2Interop, AddHostObjectToScript)
        HRESULT ( STDMETHODCALLTYPE *AddHostObjectToScript )( 
            ICoreWebView2Interop * This,
            /* [in] */ LPCWSTR name,
            /* [in] */ VARIANT *object);
        
        END_INTERFACE
    } ICoreWebView2InteropVtbl;

    interface ICoreWebView2Interop
    {
        CONST_VTBL struct ICoreWebView2InteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2Interop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2Interop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2Interop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2Interop_AddHostObjectToScript(This,name,object)	\
    ( (This)->lpVtbl -> AddHostObjectToScript(This,name,object) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2Interop_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2Interop2_INTERFACE_DEFINED__
#define __ICoreWebView2Interop2_INTERFACE_DEFINED__

/* interface ICoreWebView2Interop2 */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2Interop2 = {0xB151AD7C,0xCFB0,0x4ECF,{0xB9,0xB2,0xAF,0xCA,0x86,0x85,0x81,0xA6}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B151AD7C-CFB0-4ECF-B9B2-AFCA868581A6")
    ICoreWebView2Interop2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetComICoreWebView2( 
            /* [retval][out] */ ICoreWebView2 **coreWebView2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2Interop2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2Interop2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2Interop2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2Interop2 * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2Interop2, GetComICoreWebView2)
        HRESULT ( STDMETHODCALLTYPE *GetComICoreWebView2 )( 
            ICoreWebView2Interop2 * This,
            /* [retval][out] */ ICoreWebView2 **coreWebView2);
        
        END_INTERFACE
    } ICoreWebView2Interop2Vtbl;

    interface ICoreWebView2Interop2
    {
        CONST_VTBL struct ICoreWebView2Interop2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2Interop2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2Interop2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2Interop2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2Interop2_GetComICoreWebView2(This,coreWebView2)	\
    ( (This)->lpVtbl -> GetComICoreWebView2(This,coreWebView2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2Interop2_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2CompositionControllerInterop_INTERFACE_DEFINED__
#define __ICoreWebView2CompositionControllerInterop_INTERFACE_DEFINED__

/* interface ICoreWebView2CompositionControllerInterop */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2CompositionControllerInterop = {0x8e9922ce,0x9c80,0x42e6,{0xba,0xd7,0xfc,0xeb,0xf2,0x91,0xa4,0x95}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8e9922ce-9c80-42e6-bad7-fcebf291a495")
    ICoreWebView2CompositionControllerInterop : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AutomationProvider( 
            /* [retval][out] */ IUnknown **provider) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RootVisualTarget( 
            /* [retval][out] */ IUnknown **target) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_RootVisualTarget( 
            /* [in] */ IUnknown *target) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2CompositionControllerInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2CompositionControllerInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2CompositionControllerInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2CompositionControllerInterop * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, get_AutomationProvider)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutomationProvider )( 
            ICoreWebView2CompositionControllerInterop * This,
            /* [retval][out] */ IUnknown **provider);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, get_RootVisualTarget)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootVisualTarget )( 
            ICoreWebView2CompositionControllerInterop * This,
            /* [retval][out] */ IUnknown **target);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, put_RootVisualTarget)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootVisualTarget )( 
            ICoreWebView2CompositionControllerInterop * This,
            /* [in] */ IUnknown *target);
        
        END_INTERFACE
    } ICoreWebView2CompositionControllerInteropVtbl;

    interface ICoreWebView2CompositionControllerInterop
    {
        CONST_VTBL struct ICoreWebView2CompositionControllerInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2CompositionControllerInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2CompositionControllerInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2CompositionControllerInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2CompositionControllerInterop_get_AutomationProvider(This,provider)	\
    ( (This)->lpVtbl -> get_AutomationProvider(This,provider) ) 

#define ICoreWebView2CompositionControllerInterop_get_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> get_RootVisualTarget(This,target) ) 

#define ICoreWebView2CompositionControllerInterop_put_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> put_RootVisualTarget(This,target) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2CompositionControllerInterop_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2CompositionControllerInterop2_INTERFACE_DEFINED__
#define __ICoreWebView2CompositionControllerInterop2_INTERFACE_DEFINED__

/* interface ICoreWebView2CompositionControllerInterop2 */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2CompositionControllerInterop2 = {0x6b47bbe1,0x2480,0x4ff8,{0xa5,0xba,0x69,0xc2,0xf0,0xb8,0x68,0xb3}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6b47bbe1-2480-4ff8-a5ba-69c2f0b868b3")
    ICoreWebView2CompositionControllerInterop2 : public ICoreWebView2CompositionControllerInterop
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DragEnter( 
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DragLeave( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DragOver( 
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Drop( 
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2CompositionControllerInterop2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2CompositionControllerInterop2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2CompositionControllerInterop2 * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, get_AutomationProvider)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutomationProvider )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [retval][out] */ IUnknown **provider);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, get_RootVisualTarget)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootVisualTarget )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [retval][out] */ IUnknown **target);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, put_RootVisualTarget)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootVisualTarget )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [in] */ IUnknown *target);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, DragEnter)
        HRESULT ( STDMETHODCALLTYPE *DragEnter )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, DragLeave)
        HRESULT ( STDMETHODCALLTYPE *DragLeave )( 
            ICoreWebView2CompositionControllerInterop2 * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, DragOver)
        HRESULT ( STDMETHODCALLTYPE *DragOver )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, Drop)
        HRESULT ( STDMETHODCALLTYPE *Drop )( 
            ICoreWebView2CompositionControllerInterop2 * This,
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        END_INTERFACE
    } ICoreWebView2CompositionControllerInterop2Vtbl;

    interface ICoreWebView2CompositionControllerInterop2
    {
        CONST_VTBL struct ICoreWebView2CompositionControllerInterop2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2CompositionControllerInterop2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2CompositionControllerInterop2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2CompositionControllerInterop2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2CompositionControllerInterop2_get_AutomationProvider(This,provider)	\
    ( (This)->lpVtbl -> get_AutomationProvider(This,provider) ) 

#define ICoreWebView2CompositionControllerInterop2_get_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> get_RootVisualTarget(This,target) ) 

#define ICoreWebView2CompositionControllerInterop2_put_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> put_RootVisualTarget(This,target) ) 


#define ICoreWebView2CompositionControllerInterop2_DragEnter(This,dataObject,keyState,point,effect)	\
    ( (This)->lpVtbl -> DragEnter(This,dataObject,keyState,point,effect) ) 

#define ICoreWebView2CompositionControllerInterop2_DragLeave(This)	\
    ( (This)->lpVtbl -> DragLeave(This) ) 

#define ICoreWebView2CompositionControllerInterop2_DragOver(This,keyState,point,effect)	\
    ( (This)->lpVtbl -> DragOver(This,keyState,point,effect) ) 

#define ICoreWebView2CompositionControllerInterop2_Drop(This,dataObject,keyState,point,effect)	\
    ( (This)->lpVtbl -> Drop(This,dataObject,keyState,point,effect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2CompositionControllerInterop2_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2EnvironmentInterop_INTERFACE_DEFINED__
#define __ICoreWebView2EnvironmentInterop_INTERFACE_DEFINED__

/* interface ICoreWebView2EnvironmentInterop */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2EnvironmentInterop = {0xee503a63,0xc1e2,0x4fbf,{0x8a,0x4d,0x82,0x4e,0x95,0xf8,0xbb,0x13}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ee503a63-c1e2-4fbf-8a4d-824e95f8bb13")
    ICoreWebView2EnvironmentInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAutomationProviderForWindow( 
            /* [in] */ HWND hwnd,
            /* [retval][out] */ IUnknown **provider) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2EnvironmentInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2EnvironmentInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2EnvironmentInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2EnvironmentInterop * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2EnvironmentInterop, GetAutomationProviderForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetAutomationProviderForWindow )( 
            ICoreWebView2EnvironmentInterop * This,
            /* [in] */ HWND hwnd,
            /* [retval][out] */ IUnknown **provider);
        
        END_INTERFACE
    } ICoreWebView2EnvironmentInteropVtbl;

    interface ICoreWebView2EnvironmentInterop
    {
        CONST_VTBL struct ICoreWebView2EnvironmentInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2EnvironmentInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2EnvironmentInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2EnvironmentInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2EnvironmentInterop_GetAutomationProviderForWindow(This,hwnd,provider)	\
    ( (This)->lpVtbl -> GetAutomationProviderForWindow(This,hwnd,provider) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2EnvironmentInterop_INTERFACE_DEFINED__ */


#ifndef __ICoreWebView2CompositionControllerInterop3_INTERFACE_DEFINED__
#define __ICoreWebView2CompositionControllerInterop3_INTERFACE_DEFINED__

/* interface ICoreWebView2CompositionControllerInterop3 */
/* [unique][object][uuid] */ 


EXTERN_C __declspec(selectany) const IID IID_ICoreWebView2CompositionControllerInterop3 = {0xb211edcf,0x7ef3,0x44ad,{0x8a,0xed,0x4d,0x3e,0xf0,0xaf,0x18,0x13}};

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b211edcf-7ef3-44ad-8aed-4d3ef0af1813")
    ICoreWebView2CompositionControllerInterop3 : public ICoreWebView2CompositionControllerInterop2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE add_DragStarting( 
            /* [in] */ ICoreWebView2DragStartingEventHandler *eventHandler,
            /* [out] */ EventRegistrationToken *token) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE remove_DragStarting( 
            /* [in] */ EventRegistrationToken token) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWebView2CompositionControllerInterop3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICoreWebView2CompositionControllerInterop3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICoreWebView2CompositionControllerInterop3 * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, get_AutomationProvider)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutomationProvider )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [retval][out] */ IUnknown **provider);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, get_RootVisualTarget)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RootVisualTarget )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [retval][out] */ IUnknown **target);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop, put_RootVisualTarget)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_RootVisualTarget )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ IUnknown *target);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, DragEnter)
        HRESULT ( STDMETHODCALLTYPE *DragEnter )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, DragLeave)
        HRESULT ( STDMETHODCALLTYPE *DragLeave )( 
            ICoreWebView2CompositionControllerInterop3 * This);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, DragOver)
        HRESULT ( STDMETHODCALLTYPE *DragOver )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop2, Drop)
        HRESULT ( STDMETHODCALLTYPE *Drop )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ IDataObject *dataObject,
            /* [in] */ DWORD keyState,
            /* [in] */ POINT point,
            /* [retval][out] */ DWORD *effect);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop3, add_DragStarting)
        HRESULT ( STDMETHODCALLTYPE *add_DragStarting )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ ICoreWebView2DragStartingEventHandler *eventHandler,
            /* [out] */ EventRegistrationToken *token);
        
        DECLSPEC_XFGVIRT(ICoreWebView2CompositionControllerInterop3, remove_DragStarting)
        HRESULT ( STDMETHODCALLTYPE *remove_DragStarting )( 
            ICoreWebView2CompositionControllerInterop3 * This,
            /* [in] */ EventRegistrationToken token);
        
        END_INTERFACE
    } ICoreWebView2CompositionControllerInterop3Vtbl;

    interface ICoreWebView2CompositionControllerInterop3
    {
        CONST_VTBL struct ICoreWebView2CompositionControllerInterop3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWebView2CompositionControllerInterop3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWebView2CompositionControllerInterop3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWebView2CompositionControllerInterop3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWebView2CompositionControllerInterop3_get_AutomationProvider(This,provider)	\
    ( (This)->lpVtbl -> get_AutomationProvider(This,provider) ) 

#define ICoreWebView2CompositionControllerInterop3_get_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> get_RootVisualTarget(This,target) ) 

#define ICoreWebView2CompositionControllerInterop3_put_RootVisualTarget(This,target)	\
    ( (This)->lpVtbl -> put_RootVisualTarget(This,target) ) 


#define ICoreWebView2CompositionControllerInterop3_DragEnter(This,dataObject,keyState,point,effect)	\
    ( (This)->lpVtbl -> DragEnter(This,dataObject,keyState,point,effect) ) 

#define ICoreWebView2CompositionControllerInterop3_DragLeave(This)	\
    ( (This)->lpVtbl -> DragLeave(This) ) 

#define ICoreWebView2CompositionControllerInterop3_DragOver(This,keyState,point,effect)	\
    ( (This)->lpVtbl -> DragOver(This,keyState,point,effect) ) 

#define ICoreWebView2CompositionControllerInterop3_Drop(This,dataObject,keyState,point,effect)	\
    ( (This)->lpVtbl -> Drop(This,dataObject,keyState,point,effect) ) 


#define ICoreWebView2CompositionControllerInterop3_add_DragStarting(This,eventHandler,token)	\
    ( (This)->lpVtbl -> add_DragStarting(This,eventHandler,token) ) 

#define ICoreWebView2CompositionControllerInterop3_remove_DragStarting(This,token)	\
    ( (This)->lpVtbl -> remove_DragStarting(This,token) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWebView2CompositionControllerInterop3_INTERFACE_DEFINED__ */

#endif /* __WebView2Interop_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


