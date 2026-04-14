

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

#ifndef __userconsentverifierinterop_h__
#define __userconsentverifierinterop_h__

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

#ifndef __IUserConsentVerifierInterop_FWD_DEFINED__
#define __IUserConsentVerifierInterop_FWD_DEFINED__
typedef interface IUserConsentVerifierInterop IUserConsentVerifierInterop;

#endif 	/* __IUserConsentVerifierInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"
#include "asyncinfo.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_userconsentverifierinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3)


extern RPC_IF_HANDLE __MIDL_itf_userconsentverifierinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_userconsentverifierinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IUserConsentVerifierInterop_INTERFACE_DEFINED__
#define __IUserConsentVerifierInterop_INTERFACE_DEFINED__

/* interface IUserConsentVerifierInterop */
/* [local][object][unique][uuid] */ 


EXTERN_C const IID IID_IUserConsentVerifierInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39E050C3-4E74-441A-8DC0-B81104DF949C")
    IUserConsentVerifierInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestVerificationForWindowAsync( 
            /* [in] */ HWND appWindow,
            /* [in] */ HSTRING message,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **asyncOperation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUserConsentVerifierInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUserConsentVerifierInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUserConsentVerifierInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUserConsentVerifierInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IUserConsentVerifierInterop * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IUserConsentVerifierInterop * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IUserConsentVerifierInterop * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IUserConsentVerifierInterop, RequestVerificationForWindowAsync)
        HRESULT ( STDMETHODCALLTYPE *RequestVerificationForWindowAsync )( 
            IUserConsentVerifierInterop * This,
            /* [in] */ HWND appWindow,
            /* [in] */ HSTRING message,
            /* [in] */ REFIID riid,
            /* [iid_is][retval][out] */ void **asyncOperation);
        
        END_INTERFACE
    } IUserConsentVerifierInteropVtbl;

    interface IUserConsentVerifierInterop
    {
        CONST_VTBL struct IUserConsentVerifierInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUserConsentVerifierInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUserConsentVerifierInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUserConsentVerifierInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUserConsentVerifierInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IUserConsentVerifierInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IUserConsentVerifierInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IUserConsentVerifierInterop_RequestVerificationForWindowAsync(This,appWindow,message,riid,asyncOperation)	\
    ( (This)->lpVtbl -> RequestVerificationForWindowAsync(This,appWindow,message,riid,asyncOperation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUserConsentVerifierInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_userconsentverifierinterop_0000_0001 */
/* [local] */ 

#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS3)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_userconsentverifierinterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_userconsentverifierinterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


