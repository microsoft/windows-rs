

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

#ifndef __windows2Edevices2Edisplay2Ecore2Einterop_h__
#define __windows2Edevices2Edisplay2Ecore2Einterop_h__

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

#ifndef __IDisplayDeviceInterop_FWD_DEFINED__
#define __IDisplayDeviceInterop_FWD_DEFINED__
typedef interface IDisplayDeviceInterop IDisplayDeviceInterop;

#endif 	/* __IDisplayDeviceInterop_FWD_DEFINED__ */


#ifndef __IDisplayPathInterop_FWD_DEFINED__
#define __IDisplayPathInterop_FWD_DEFINED__
typedef interface IDisplayPathInterop IDisplayPathInterop;

#endif 	/* __IDisplayPathInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "Inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Edevices2Edisplay2Ecore2Einterop_0000_0000 */
/* [local] */ 

#ifdef __midl
#ifndef LUID_DEFINED
#define LUID_DEFINED 1
typedef struct __LUID
    {
    DWORD LowPart;
    LONG HighPart;
    } 	LUID;

typedef struct __LUID *PLUID;

#endif
#endif
#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#ifdef __cplusplus
inline INT64 Int64FromLuid(const LUID &Luid)
{
    LARGE_INTEGER val;
    val.LowPart = Luid.LowPart;
    val.HighPart = Luid.HighPart;
    return val.QuadPart;
}

inline LUID LuidFromInt64(INT64 Int64)
{
    LARGE_INTEGER val;
    val.QuadPart= Int64;

    LUID Luid;
    Luid.LowPart = val.LowPart;
    Luid.HighPart = val.HighPart;
    return Luid;
}
#endif


extern RPC_IF_HANDLE __MIDL_itf_windows2Edevices2Edisplay2Ecore2Einterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Edevices2Edisplay2Ecore2Einterop_0000_0000_v0_0_s_ifspec;

#ifndef __IDisplayDeviceInterop_INTERFACE_DEFINED__
#define __IDisplayDeviceInterop_INTERFACE_DEFINED__

/* interface IDisplayDeviceInterop */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IDisplayDeviceInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64338358-366A-471B-BD56-DD8EF48E439B")
    IDisplayDeviceInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSharedHandle( 
            /* [in] */ IInspectable *pObject,
            /* [in] */ const SECURITY_ATTRIBUTES *pSecurityAttributes,
            /* [in] */ DWORD Access,
            /* [in] */ HSTRING Name,
            /* [retval][out] */ HANDLE *pHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenSharedHandle( 
            /* [in] */ HANDLE NTHandle,
            /* [in] */ IID riid,
            /* [retval][out] */ void **ppvObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDisplayDeviceInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDisplayDeviceInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDisplayDeviceInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDisplayDeviceInterop * This);
        
        DECLSPEC_XFGVIRT(IDisplayDeviceInterop, CreateSharedHandle)
        HRESULT ( STDMETHODCALLTYPE *CreateSharedHandle )( 
            IDisplayDeviceInterop * This,
            /* [in] */ IInspectable *pObject,
            /* [in] */ const SECURITY_ATTRIBUTES *pSecurityAttributes,
            /* [in] */ DWORD Access,
            /* [in] */ HSTRING Name,
            /* [retval][out] */ HANDLE *pHandle);
        
        DECLSPEC_XFGVIRT(IDisplayDeviceInterop, OpenSharedHandle)
        HRESULT ( STDMETHODCALLTYPE *OpenSharedHandle )( 
            IDisplayDeviceInterop * This,
            /* [in] */ HANDLE NTHandle,
            /* [in] */ IID riid,
            /* [retval][out] */ void **ppvObj);
        
        END_INTERFACE
    } IDisplayDeviceInteropVtbl;

    interface IDisplayDeviceInterop
    {
        CONST_VTBL struct IDisplayDeviceInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDisplayDeviceInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDisplayDeviceInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDisplayDeviceInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDisplayDeviceInterop_CreateSharedHandle(This,pObject,pSecurityAttributes,Access,Name,pHandle)	\
    ( (This)->lpVtbl -> CreateSharedHandle(This,pObject,pSecurityAttributes,Access,Name,pHandle) ) 

#define IDisplayDeviceInterop_OpenSharedHandle(This,NTHandle,riid,ppvObj)	\
    ( (This)->lpVtbl -> OpenSharedHandle(This,NTHandle,riid,ppvObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDisplayDeviceInterop_INTERFACE_DEFINED__ */


#ifndef __IDisplayPathInterop_INTERFACE_DEFINED__
#define __IDisplayPathInterop_INTERFACE_DEFINED__

/* interface IDisplayPathInterop */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IDisplayPathInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A6BA4205-E59E-4E71-B25B-4E436D21EE3D")
    IDisplayPathInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSourcePresentationHandle( 
            /* [retval][out] */ HANDLE *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceId( 
            /* [retval][out] */ UINT *pSourceId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDisplayPathInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDisplayPathInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDisplayPathInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDisplayPathInterop * This);
        
        DECLSPEC_XFGVIRT(IDisplayPathInterop, CreateSourcePresentationHandle)
        HRESULT ( STDMETHODCALLTYPE *CreateSourcePresentationHandle )( 
            IDisplayPathInterop * This,
            /* [retval][out] */ HANDLE *pValue);
        
        DECLSPEC_XFGVIRT(IDisplayPathInterop, GetSourceId)
        HRESULT ( STDMETHODCALLTYPE *GetSourceId )( 
            IDisplayPathInterop * This,
            /* [retval][out] */ UINT *pSourceId);
        
        END_INTERFACE
    } IDisplayPathInteropVtbl;

    interface IDisplayPathInterop
    {
        CONST_VTBL struct IDisplayPathInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDisplayPathInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDisplayPathInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDisplayPathInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDisplayPathInterop_CreateSourcePresentationHandle(This,pValue)	\
    ( (This)->lpVtbl -> CreateSourcePresentationHandle(This,pValue) ) 

#define IDisplayPathInterop_GetSourceId(This,pSourceId)	\
    ( (This)->lpVtbl -> GetSourceId(This,pSourceId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDisplayPathInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Edevices2Edisplay2Ecore2Einterop_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_windows2Edevices2Edisplay2Ecore2Einterop_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Edevices2Edisplay2Ecore2Einterop_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


