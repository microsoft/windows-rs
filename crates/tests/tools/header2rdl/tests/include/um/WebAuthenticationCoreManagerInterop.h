

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

#ifndef __webauthenticationcoremanagerinterop_h__
#define __webauthenticationcoremanagerinterop_h__

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

#ifndef __IWebAuthenticationCoreManagerInterop_FWD_DEFINED__
#define __IWebAuthenticationCoreManagerInterop_FWD_DEFINED__
typedef interface IWebAuthenticationCoreManagerInterop IWebAuthenticationCoreManagerInterop;

#endif 	/* __IWebAuthenticationCoreManagerInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "Inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_webauthenticationcoremanagerinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_webauthenticationcoremanagerinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webauthenticationcoremanagerinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IWebAuthenticationCoreManagerInterop_INTERFACE_DEFINED__
#define __IWebAuthenticationCoreManagerInterop_INTERFACE_DEFINED__

/* interface IWebAuthenticationCoreManagerInterop */
/* [object][local][unique][uuid] */ 


EXTERN_C const IID IID_IWebAuthenticationCoreManagerInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4B8E804-811E-4436-B69C-44CB67B72084")
    IWebAuthenticationCoreManagerInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestTokenForWindowAsync( 
            /* [in] */ HWND appWindow,
            /* [in] */ IInspectable *request,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **asyncInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestTokenWithWebAccountForWindowAsync( 
            /* [in] */ HWND appWindow,
            /* [in] */ IInspectable *request,
            /* [in] */ IInspectable *webAccount,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **asyncInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWebAuthenticationCoreManagerInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWebAuthenticationCoreManagerInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWebAuthenticationCoreManagerInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWebAuthenticationCoreManagerInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IWebAuthenticationCoreManagerInterop * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IWebAuthenticationCoreManagerInterop * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IWebAuthenticationCoreManagerInterop * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IWebAuthenticationCoreManagerInterop, RequestTokenForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestTokenForWindowAsync )( 
            IWebAuthenticationCoreManagerInterop * This,
            /* [in] */ HWND appWindow,
            /* [in] */ IInspectable *request,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **asyncInfo);
        
        DECLSPEC_XFGVIRT(IWebAuthenticationCoreManagerInterop, RequestTokenWithWebAccountForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestTokenWithWebAccountForWindowAsync )( 
            IWebAuthenticationCoreManagerInterop * This,
            /* [in] */ HWND appWindow,
            /* [in] */ IInspectable *request,
            /* [in] */ IInspectable *webAccount,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **asyncInfo);
        
        END_INTERFACE
    } IWebAuthenticationCoreManagerInteropVtbl;

    interface IWebAuthenticationCoreManagerInterop
    {
        CONST_VTBL struct IWebAuthenticationCoreManagerInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWebAuthenticationCoreManagerInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWebAuthenticationCoreManagerInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWebAuthenticationCoreManagerInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWebAuthenticationCoreManagerInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IWebAuthenticationCoreManagerInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IWebAuthenticationCoreManagerInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IWebAuthenticationCoreManagerInterop_RequestTokenForWindowAsync(This,appWindow,request,riid,asyncInfo)	\
    ( (This)->lpVtbl -> RequestTokenForWindowAsync(This,appWindow,request,riid,asyncInfo) ) 

#define IWebAuthenticationCoreManagerInterop_RequestTokenWithWebAccountForWindowAsync(This,appWindow,request,webAccount,riid,asyncInfo)	\
    ( (This)->lpVtbl -> RequestTokenWithWebAccountForWindowAsync(This,appWindow,request,webAccount,riid,asyncInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWebAuthenticationCoreManagerInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_webauthenticationcoremanagerinterop_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WIN10)


extern RPC_IF_HANDLE __MIDL_itf_webauthenticationcoremanagerinterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_webauthenticationcoremanagerinterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


