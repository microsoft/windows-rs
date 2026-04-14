

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

#ifndef __tipautocomplete_h__
#define __tipautocomplete_h__

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

#ifndef __ITipAutoCompleteProvider_FWD_DEFINED__
#define __ITipAutoCompleteProvider_FWD_DEFINED__
typedef interface ITipAutoCompleteProvider ITipAutoCompleteProvider;

#endif 	/* __ITipAutoCompleteProvider_FWD_DEFINED__ */


#ifndef __ITipAutoCompleteClient_FWD_DEFINED__
#define __ITipAutoCompleteClient_FWD_DEFINED__
typedef interface ITipAutoCompleteClient ITipAutoCompleteClient;

#endif 	/* __ITipAutoCompleteClient_FWD_DEFINED__ */


#ifndef __TipAutoCompleteClient_FWD_DEFINED__
#define __TipAutoCompleteClient_FWD_DEFINED__

#ifdef __cplusplus
typedef class TipAutoCompleteClient TipAutoCompleteClient;
#else
typedef struct TipAutoCompleteClient TipAutoCompleteClient;
#endif /* __cplusplus */

#endif 	/* __TipAutoCompleteClient_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tipautocomplete_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_tipautocomplete_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tipautocomplete_0000_0000_v0_0_s_ifspec;

#ifndef __ITipAutoCompleteProvider_INTERFACE_DEFINED__
#define __ITipAutoCompleteProvider_INTERFACE_DEFINED__

/* interface ITipAutoCompleteProvider */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_ITipAutoCompleteProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7C6CF46D-8404-46b9-AD33-F5B6036D4007")
    ITipAutoCompleteProvider : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UpdatePendingText( 
            /* [in] */ __RPC__in BSTR bstrPendingText) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Show( 
            /* [in] */ BOOL fShow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITipAutoCompleteProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITipAutoCompleteProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITipAutoCompleteProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITipAutoCompleteProvider * This);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteProvider, UpdatePendingText)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UpdatePendingText )( 
            __RPC__in ITipAutoCompleteProvider * This,
            /* [in] */ __RPC__in BSTR bstrPendingText);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteProvider, Show)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Show )( 
            __RPC__in ITipAutoCompleteProvider * This,
            /* [in] */ BOOL fShow);
        
        END_INTERFACE
    } ITipAutoCompleteProviderVtbl;

    interface ITipAutoCompleteProvider
    {
        CONST_VTBL struct ITipAutoCompleteProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITipAutoCompleteProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITipAutoCompleteProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITipAutoCompleteProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITipAutoCompleteProvider_UpdatePendingText(This,bstrPendingText)	\
    ( (This)->lpVtbl -> UpdatePendingText(This,bstrPendingText) ) 

#define ITipAutoCompleteProvider_Show(This,fShow)	\
    ( (This)->lpVtbl -> Show(This,fShow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITipAutoCompleteProvider_INTERFACE_DEFINED__ */


#ifndef __ITipAutoCompleteClient_INTERFACE_DEFINED__
#define __ITipAutoCompleteClient_INTERFACE_DEFINED__

/* interface ITipAutoCompleteClient */
/* [helpstring][unique][uuid][object] */ 


EXTERN_C const IID IID_ITipAutoCompleteClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5E078E03-8265-4bbe-9487-D242EDBEF910")
    ITipAutoCompleteClient : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AdviseProvider( 
            /* [in] */ __RPC__in HWND hWndField,
            /* [in] */ __RPC__in_opt ITipAutoCompleteProvider *pIProvider) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnadviseProvider( 
            /* [in] */ __RPC__in HWND hWndField,
            /* [in] */ __RPC__in_opt ITipAutoCompleteProvider *pIProvider) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UserSelection( void) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE PreferredRects( 
            /* [in] */ __RPC__in RECT *prcACList,
            /* [in] */ __RPC__in RECT *prcField,
            /* [out] */ __RPC__out RECT *prcModifiedACList,
            /* [out] */ __RPC__out BOOL *pfShownAboveTip) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RequestShowUI( 
            /* [in] */ __RPC__in HWND hWndList,
            /* [out] */ __RPC__out BOOL *pfAllowShowing) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITipAutoCompleteClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITipAutoCompleteClient * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITipAutoCompleteClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITipAutoCompleteClient * This);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteClient, AdviseProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AdviseProvider )( 
            __RPC__in ITipAutoCompleteClient * This,
            /* [in] */ __RPC__in HWND hWndField,
            /* [in] */ __RPC__in_opt ITipAutoCompleteProvider *pIProvider);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteClient, UnadviseProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnadviseProvider )( 
            __RPC__in ITipAutoCompleteClient * This,
            /* [in] */ __RPC__in HWND hWndField,
            /* [in] */ __RPC__in_opt ITipAutoCompleteProvider *pIProvider);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteClient, UserSelection)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UserSelection )( 
            __RPC__in ITipAutoCompleteClient * This);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteClient, PreferredRects)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *PreferredRects )( 
            __RPC__in ITipAutoCompleteClient * This,
            /* [in] */ __RPC__in RECT *prcACList,
            /* [in] */ __RPC__in RECT *prcField,
            /* [out] */ __RPC__out RECT *prcModifiedACList,
            /* [out] */ __RPC__out BOOL *pfShownAboveTip);
        
        DECLSPEC_XFGVIRT(ITipAutoCompleteClient, RequestShowUI)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RequestShowUI )( 
            __RPC__in ITipAutoCompleteClient * This,
            /* [in] */ __RPC__in HWND hWndList,
            /* [out] */ __RPC__out BOOL *pfAllowShowing);
        
        END_INTERFACE
    } ITipAutoCompleteClientVtbl;

    interface ITipAutoCompleteClient
    {
        CONST_VTBL struct ITipAutoCompleteClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITipAutoCompleteClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITipAutoCompleteClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITipAutoCompleteClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITipAutoCompleteClient_AdviseProvider(This,hWndField,pIProvider)	\
    ( (This)->lpVtbl -> AdviseProvider(This,hWndField,pIProvider) ) 

#define ITipAutoCompleteClient_UnadviseProvider(This,hWndField,pIProvider)	\
    ( (This)->lpVtbl -> UnadviseProvider(This,hWndField,pIProvider) ) 

#define ITipAutoCompleteClient_UserSelection(This)	\
    ( (This)->lpVtbl -> UserSelection(This) ) 

#define ITipAutoCompleteClient_PreferredRects(This,prcACList,prcField,prcModifiedACList,pfShownAboveTip)	\
    ( (This)->lpVtbl -> PreferredRects(This,prcACList,prcField,prcModifiedACList,pfShownAboveTip) ) 

#define ITipAutoCompleteClient_RequestShowUI(This,hWndList,pfAllowShowing)	\
    ( (This)->lpVtbl -> RequestShowUI(This,hWndList,pfAllowShowing) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITipAutoCompleteClient_INTERFACE_DEFINED__ */



#ifndef __TipAutoCompleteClientLib_LIBRARY_DEFINED__
#define __TipAutoCompleteClientLib_LIBRARY_DEFINED__

/* library TipAutoCompleteClientLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_TipAutoCompleteClientLib;

EXTERN_C const CLSID CLSID_TipAutoCompleteClient;

#ifdef __cplusplus

class DECLSPEC_UUID("807C1E6C-1D00-453f-B920-B61BB7CDD997")
TipAutoCompleteClient;
#endif
#endif /* __TipAutoCompleteClientLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_tipautocomplete_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tipautocomplete_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tipautocomplete_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


