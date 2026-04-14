

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

#ifndef __remotesystemsinterop_h__
#define __remotesystemsinterop_h__

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

#ifndef __ICorrelationVectorInformation_FWD_DEFINED__
#define __ICorrelationVectorInformation_FWD_DEFINED__
typedef interface ICorrelationVectorInformation ICorrelationVectorInformation;

#endif 	/* __ICorrelationVectorInformation_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "Inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_remotesystemsinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_VB)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_remotesystemsinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_remotesystemsinterop_0000_0000_v0_0_s_ifspec;

#ifndef __ICorrelationVectorInformation_INTERFACE_DEFINED__
#define __ICorrelationVectorInformation_INTERFACE_DEFINED__

/* interface ICorrelationVectorInformation */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_ICorrelationVectorInformation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83C78B3C-D88B-4950-AA6E-22B8D22AABD3")
    ICorrelationVectorInformation : public IInspectable
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LastCorrelationVectorForThread( 
            /* [retval][out] */ HSTRING *cv) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_NextCorrelationVectorForThread( 
            /* [retval][out] */ HSTRING *cv) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_NextCorrelationVectorForThread( 
            /* [in] */ HSTRING cv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICorrelationVectorInformationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICorrelationVectorInformation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICorrelationVectorInformation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICorrelationVectorInformation * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            ICorrelationVectorInformation * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            ICorrelationVectorInformation * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            ICorrelationVectorInformation * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(ICorrelationVectorInformation, get_LastCorrelationVectorForThread)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastCorrelationVectorForThread )( 
            ICorrelationVectorInformation * This,
            /* [retval][out] */ HSTRING *cv);
        
        DECLSPEC_XFGVIRT(ICorrelationVectorInformation, get_NextCorrelationVectorForThread)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_NextCorrelationVectorForThread )( 
            ICorrelationVectorInformation * This,
            /* [retval][out] */ HSTRING *cv);
        
        DECLSPEC_XFGVIRT(ICorrelationVectorInformation, put_NextCorrelationVectorForThread)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_NextCorrelationVectorForThread )( 
            ICorrelationVectorInformation * This,
            /* [in] */ HSTRING cv);
        
        END_INTERFACE
    } ICorrelationVectorInformationVtbl;

    interface ICorrelationVectorInformation
    {
        CONST_VTBL struct ICorrelationVectorInformationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICorrelationVectorInformation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICorrelationVectorInformation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICorrelationVectorInformation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICorrelationVectorInformation_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define ICorrelationVectorInformation_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define ICorrelationVectorInformation_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define ICorrelationVectorInformation_get_LastCorrelationVectorForThread(This,cv)	\
    ( (This)->lpVtbl -> get_LastCorrelationVectorForThread(This,cv) ) 

#define ICorrelationVectorInformation_get_NextCorrelationVectorForThread(This,cv)	\
    ( (This)->lpVtbl -> get_NextCorrelationVectorForThread(This,cv) ) 

#define ICorrelationVectorInformation_put_NextCorrelationVectorForThread(This,cv)	\
    ( (This)->lpVtbl -> put_NextCorrelationVectorForThread(This,cv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICorrelationVectorInformation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_remotesystemsinterop_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif /* (NTDDI_VERSION >= NTDDI_WIN10_VB) */


extern RPC_IF_HANDLE __MIDL_itf_remotesystemsinterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_remotesystemsinterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


