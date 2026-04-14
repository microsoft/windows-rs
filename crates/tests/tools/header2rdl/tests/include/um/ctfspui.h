

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

#ifndef __ctfspui_h__
#define __ctfspui_h__

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

#ifndef __ITfSpeechUIServer_FWD_DEFINED__
#define __ITfSpeechUIServer_FWD_DEFINED__
typedef interface ITfSpeechUIServer ITfSpeechUIServer;

#endif 	/* __ITfSpeechUIServer_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "msctf.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_ctfspui_0000_0000 */
/* [local] */ 

//=--------------------------------------------------------------------------=
// ctfspui.h


// Speech UI declarations.

//=--------------------------------------------------------------------------=
// (C) Copyright 1995-2001 Microsoft Corporation.  All Rights Reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR TFPLIED, INCLUDING BUT NOT LIMITED TO
// THE TFPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//=--------------------------------------------------------------------------=

#ifndef CTFSPUI_DEFINED
#define CTFSPUI_DEFINED

#include <windows.h>
#include <winapifamily.h>

#ifdef __cplusplus
extern "C" {
#endif /* __cplusplus */

#ifdef __cplusplus
}
#endif  /* __cplusplus */
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_C const CLSID CLSID_SpeechUIServer;


extern RPC_IF_HANDLE __MIDL_itf_ctfspui_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ctfspui_0000_0000_v0_0_s_ifspec;

#ifndef __ITfSpeechUIServer_INTERFACE_DEFINED__
#define __ITfSpeechUIServer_INTERFACE_DEFINED__

/* interface ITfSpeechUIServer */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_ITfSpeechUIServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90e9a944-9244-489f-a78f-de67afc013a7")
    ITfSpeechUIServer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowUI( 
            /* [in] */ BOOL fShow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateBalloon( 
            /* [in] */ TfLBBalloonStyle style,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pch,
            /* [in] */ ULONG cch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITfSpeechUIServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITfSpeechUIServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITfSpeechUIServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITfSpeechUIServer * This);
        
        DECLSPEC_XFGVIRT(ITfSpeechUIServer, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in ITfSpeechUIServer * This);
        
        DECLSPEC_XFGVIRT(ITfSpeechUIServer, ShowUI)
        HRESULT ( STDMETHODCALLTYPE *ShowUI )( 
            __RPC__in ITfSpeechUIServer * This,
            /* [in] */ BOOL fShow);
        
        DECLSPEC_XFGVIRT(ITfSpeechUIServer, UpdateBalloon)
        HRESULT ( STDMETHODCALLTYPE *UpdateBalloon )( 
            __RPC__in ITfSpeechUIServer * This,
            /* [in] */ TfLBBalloonStyle style,
            /* [size_is][in] */ __RPC__in_ecount_full(cch) const WCHAR *pch,
            /* [in] */ ULONG cch);
        
        END_INTERFACE
    } ITfSpeechUIServerVtbl;

    interface ITfSpeechUIServer
    {
        CONST_VTBL struct ITfSpeechUIServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITfSpeechUIServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITfSpeechUIServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITfSpeechUIServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITfSpeechUIServer_Initialize(This)	\
    ( (This)->lpVtbl -> Initialize(This) ) 

#define ITfSpeechUIServer_ShowUI(This,fShow)	\
    ( (This)->lpVtbl -> ShowUI(This,fShow) ) 

#define ITfSpeechUIServer_UpdateBalloon(This,style,pch,cch)	\
    ( (This)->lpVtbl -> UpdateBalloon(This,style,pch,cch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITfSpeechUIServer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_ctfspui_0000_0001 */
/* [local] */ 


DEFINE_GUID(IID_ITfSpeechUIServer, 0x90e9a944, 0x9244, 0x489f, 0xa7, 0x8f, 0xde, 0x67, 0xaf, 0xc0, 0x13, 0xa7 );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif // CTFSPUI_DEFINED


extern RPC_IF_HANDLE __MIDL_itf_ctfspui_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_ctfspui_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


