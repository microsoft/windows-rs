

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

#ifndef __asyncinfo_h__
#define __asyncinfo_h__

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

#ifndef __IAsyncInfo_FWD_DEFINED__
#define __IAsyncInfo_FWD_DEFINED__
typedef interface IAsyncInfo IAsyncInfo;

#endif 	/* __IAsyncInfo_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_asyncinfo_0000_0000 */
/* [local] */ 

//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
#pragma once
#ifdef __cplusplus
namespace ABI { namespace Windows { namespace Foundation {
enum class AsyncStatus {
  Started = 0,
  Completed, 
  Canceled, 
  Error,
};
} } } // ABI::Windows:::Foundation
using ABI::Windows::Foundation::AsyncStatus;
#if !defined(_HIDE_GLOBAL_ASYNC_STATUS) && !defined(__cplusplus_winrt)
static const ABI::Windows::Foundation::AsyncStatus Started = AsyncStatus::Started;
static const ABI::Windows::Foundation::AsyncStatus Completed = AsyncStatus::Completed;
static const ABI::Windows::Foundation::AsyncStatus Canceled = AsyncStatus::Canceled;
static const ABI::Windows::Foundation::AsyncStatus Error = AsyncStatus::Error;
#endif // _HIDE_GLOBAL_ASYNC_STATUS
#else
typedef /* [v1_enum] */ 
enum AsyncStatus
    {
        Started	= 0,
        Completed	= ( Started + 1 ) ,
        Canceled	= ( Completed + 1 ) ,
        Error	= ( Canceled + 1 ) 
    } 	AsyncStatus;

#endif // __cplusplus


extern RPC_IF_HANDLE __MIDL_itf_asyncinfo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_asyncinfo_0000_0000_v0_0_s_ifspec;

#ifndef __IAsyncInfo_INTERFACE_DEFINED__
#define __IAsyncInfo_INTERFACE_DEFINED__

/* interface IAsyncInfo */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAsyncInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00000036-0000-0000-C000-000000000046")
    IAsyncInfo : public IInspectable
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out unsigned __int32 *id) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ __RPC__out AsyncStatus *status) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ErrorCode( 
            /* [retval][out] */ __RPC__out HRESULT *errorCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAsyncInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAsyncInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAsyncInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAsyncInfo * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in IAsyncInfo * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in IAsyncInfo * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in IAsyncInfo * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IAsyncInfo * This,
            /* [retval][out] */ __RPC__out unsigned __int32 *id);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, get_Status)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            __RPC__in IAsyncInfo * This,
            /* [retval][out] */ __RPC__out AsyncStatus *status);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, get_ErrorCode)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ErrorCode )( 
            __RPC__in IAsyncInfo * This,
            /* [retval][out] */ __RPC__out HRESULT *errorCode);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IAsyncInfo * This);
        
        DECLSPEC_XFGVIRT(IAsyncInfo, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IAsyncInfo * This);
        
        END_INTERFACE
    } IAsyncInfoVtbl;

    interface IAsyncInfo
    {
        CONST_VTBL struct IAsyncInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAsyncInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAsyncInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAsyncInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAsyncInfo_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IAsyncInfo_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IAsyncInfo_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IAsyncInfo_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IAsyncInfo_get_Status(This,status)	\
    ( (This)->lpVtbl -> get_Status(This,status) ) 

#define IAsyncInfo_get_ErrorCode(This,errorCode)	\
    ( (This)->lpVtbl -> get_ErrorCode(This,errorCode) ) 

#define IAsyncInfo_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IAsyncInfo_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAsyncInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_asyncinfo_0000_0001 */
/* [local] */ 

#ifdef __cplusplus
namespace ABI { namespace Windows { namespace Foundation {
using ::IAsyncInfo;
} } } // ABI::Windows:::Foundation
#endif // __cplusplus


extern RPC_IF_HANDLE __MIDL_itf_asyncinfo_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_asyncinfo_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


