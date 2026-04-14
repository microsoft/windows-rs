

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

#ifndef __iedial_h__
#define __iedial_h__

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

#ifndef __IDialEventSink_FWD_DEFINED__
#define __IDialEventSink_FWD_DEFINED__
typedef interface IDialEventSink IDialEventSink;

#endif 	/* __IDialEventSink_FWD_DEFINED__ */


#ifndef __IDialEngine_FWD_DEFINED__
#define __IDialEngine_FWD_DEFINED__
typedef interface IDialEngine IDialEngine;

#endif 	/* __IDialEngine_FWD_DEFINED__ */


#ifndef __IDialBranding_FWD_DEFINED__
#define __IDialBranding_FWD_DEFINED__
typedef interface IDialBranding IDialBranding;

#endif 	/* __IDialBranding_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_iedial_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// iedial.h
//=--------------------------------------------------------------------------=
// (C) Copyright Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#pragma comment(lib,"uuid.lib")

//---------------------------------------------------------------------------=



extern RPC_IF_HANDLE __MIDL_itf_iedial_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iedial_0000_0000_v0_0_s_ifspec;

#ifndef __IDialEventSink_INTERFACE_DEFINED__
#define __IDialEventSink_INTERFACE_DEFINED__

/* interface IDialEventSink */
/* [object][helpstring][version][uuid] */ 


EXTERN_C const IID IID_IDialEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2d86f4ff-6e2d-4488-b2e9-6934afd41bea")
    IDialEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [in] */ DWORD dwEvent,
            /* [in] */ DWORD dwStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDialEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDialEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDialEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDialEventSink * This);
        
        DECLSPEC_XFGVIRT(IDialEventSink, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            __RPC__in IDialEventSink * This,
            /* [in] */ DWORD dwEvent,
            /* [in] */ DWORD dwStatus);
        
        END_INTERFACE
    } IDialEventSinkVtbl;

    interface IDialEventSink
    {
        CONST_VTBL struct IDialEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDialEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDialEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDialEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDialEventSink_OnEvent(This,dwEvent,dwStatus)	\
    ( (This)->lpVtbl -> OnEvent(This,dwEvent,dwStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDialEventSink_INTERFACE_DEFINED__ */


#ifndef __IDialEngine_INTERFACE_DEFINED__
#define __IDialEngine_INTERFACE_DEFINED__

/* interface IDialEngine */
/* [object][helpstring][version][uuid] */ 


EXTERN_C const IID IID_IDialEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39fd782b-7905-40d5-9148-3c9b190423d5")
    IDialEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in LPCWSTR pwzConnectoid,
            /* [in] */ __RPC__in_opt IDialEventSink *pIDES) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwzProperty,
            /* [in] */ __RPC__in LPWSTR pwzValue,
            /* [in] */ DWORD dwBufSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwzProperty,
            /* [in] */ __RPC__in LPCWSTR pwzValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Dial( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HangUp( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnectedState( 
            /* [out] */ __RPC__out DWORD *pdwState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnectHandle( 
            /* [out] */ __RPC__out DWORD_PTR *pdwHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDialEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDialEngine * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDialEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDialEngine * This);
        
        DECLSPEC_XFGVIRT(IDialEngine, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IDialEngine * This,
            /* [in] */ __RPC__in LPCWSTR pwzConnectoid,
            /* [in] */ __RPC__in_opt IDialEventSink *pIDES);
        
        DECLSPEC_XFGVIRT(IDialEngine, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IDialEngine * This,
            /* [in] */ __RPC__in LPCWSTR pwzProperty,
            /* [in] */ __RPC__in LPWSTR pwzValue,
            /* [in] */ DWORD dwBufSize);
        
        DECLSPEC_XFGVIRT(IDialEngine, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IDialEngine * This,
            /* [in] */ __RPC__in LPCWSTR pwzProperty,
            /* [in] */ __RPC__in LPCWSTR pwzValue);
        
        DECLSPEC_XFGVIRT(IDialEngine, Dial)
        HRESULT ( STDMETHODCALLTYPE *Dial )( 
            __RPC__in IDialEngine * This);
        
        DECLSPEC_XFGVIRT(IDialEngine, HangUp)
        HRESULT ( STDMETHODCALLTYPE *HangUp )( 
            __RPC__in IDialEngine * This);
        
        DECLSPEC_XFGVIRT(IDialEngine, GetConnectedState)
        HRESULT ( STDMETHODCALLTYPE *GetConnectedState )( 
            __RPC__in IDialEngine * This,
            /* [out] */ __RPC__out DWORD *pdwState);
        
        DECLSPEC_XFGVIRT(IDialEngine, GetConnectHandle)
        HRESULT ( STDMETHODCALLTYPE *GetConnectHandle )( 
            __RPC__in IDialEngine * This,
            /* [out] */ __RPC__out DWORD_PTR *pdwHandle);
        
        END_INTERFACE
    } IDialEngineVtbl;

    interface IDialEngine
    {
        CONST_VTBL struct IDialEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDialEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDialEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDialEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDialEngine_Initialize(This,pwzConnectoid,pIDES)	\
    ( (This)->lpVtbl -> Initialize(This,pwzConnectoid,pIDES) ) 

#define IDialEngine_GetProperty(This,pwzProperty,pwzValue,dwBufSize)	\
    ( (This)->lpVtbl -> GetProperty(This,pwzProperty,pwzValue,dwBufSize) ) 

#define IDialEngine_SetProperty(This,pwzProperty,pwzValue)	\
    ( (This)->lpVtbl -> SetProperty(This,pwzProperty,pwzValue) ) 

#define IDialEngine_Dial(This)	\
    ( (This)->lpVtbl -> Dial(This) ) 

#define IDialEngine_HangUp(This)	\
    ( (This)->lpVtbl -> HangUp(This) ) 

#define IDialEngine_GetConnectedState(This,pdwState)	\
    ( (This)->lpVtbl -> GetConnectedState(This,pdwState) ) 

#define IDialEngine_GetConnectHandle(This,pdwHandle)	\
    ( (This)->lpVtbl -> GetConnectHandle(This,pdwHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDialEngine_INTERFACE_DEFINED__ */


#ifndef __IDialBranding_INTERFACE_DEFINED__
#define __IDialBranding_INTERFACE_DEFINED__

/* interface IDialBranding */
/* [object][helpstring][version][uuid] */ 


EXTERN_C const IID IID_IDialBranding;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8aecafa9-4306-43cc-8c5a-765f2979cc16")
    IDialBranding : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in LPCWSTR pwzConnectoid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBitmap( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phBitmap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDialBrandingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDialBranding * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDialBranding * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDialBranding * This);
        
        DECLSPEC_XFGVIRT(IDialBranding, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IDialBranding * This,
            /* [in] */ __RPC__in LPCWSTR pwzConnectoid);
        
        DECLSPEC_XFGVIRT(IDialBranding, GetBitmap)
        HRESULT ( STDMETHODCALLTYPE *GetBitmap )( 
            __RPC__in IDialBranding * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt HBITMAP *phBitmap);
        
        END_INTERFACE
    } IDialBrandingVtbl;

    interface IDialBranding
    {
        CONST_VTBL struct IDialBrandingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDialBranding_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDialBranding_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDialBranding_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDialBranding_Initialize(This,pwzConnectoid)	\
    ( (This)->lpVtbl -> Initialize(This,pwzConnectoid) ) 

#define IDialBranding_GetBitmap(This,dwIndex,phBitmap)	\
    ( (This)->lpVtbl -> GetBitmap(This,dwIndex,phBitmap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDialBranding_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_iedial_0000_0003 */
/* [local] */ 

#define DIALPROP_USERNAME       L"UserName"        
#define DIALPROP_PASSWORD       L"Password"        
#define DIALPROP_DOMAIN         L"Domain"          
#define DIALPROP_SAVEPASSWORD   L"SavePassword"    
#define DIALPROP_REDIALCOUNT    L"RedialCount"     
#define DIALPROP_REDIALINTERVAL L"RedialInterval"  
#define DIALPROP_PHONENUMBER    L"PhoneNumber"     
#define DIALPROP_LASTERROR      L"LastError"       
#define DIALPROP_RESOLVEDPHONE  L"ResolvedPhone"   

#define DIALENG_OperationComplete   0x10000          
#define DIALENG_RedialAttempt       0x10001          
#define DIALENG_RedialWait          0x10002          
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_iedial_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_iedial_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HBITMAP_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

unsigned long             __RPC_USER  HBITMAP_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HBITMAP * ); 
unsigned char * __RPC_USER  HBITMAP_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HBITMAP * ); 
void                      __RPC_USER  HBITMAP_UserFree64(     __RPC__in unsigned long *, __RPC__in HBITMAP * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


