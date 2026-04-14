

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

#ifndef __tapi3ds_h__
#define __tapi3ds_h__

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

#ifndef __ITAMMediaFormat_FWD_DEFINED__
#define __ITAMMediaFormat_FWD_DEFINED__
typedef interface ITAMMediaFormat ITAMMediaFormat;

#endif 	/* __ITAMMediaFormat_FWD_DEFINED__ */


#ifndef __ITAllocatorProperties_FWD_DEFINED__
#define __ITAllocatorProperties_FWD_DEFINED__
typedef interface ITAllocatorProperties ITAllocatorProperties;

#endif 	/* __ITAllocatorProperties_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "strmif.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tapi3ds_0000_0000 */
/* [local] */ 

/* Copyright (c) Microsoft Corporation. All rights reserved. */
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_tapi3ds_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3ds_0000_0000_v0_0_s_ifspec;

#ifndef __ITAMMediaFormat_INTERFACE_DEFINED__
#define __ITAMMediaFormat_INTERFACE_DEFINED__

/* interface ITAMMediaFormat */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAMMediaFormat;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0364EB00-4A77-11D1-A671-006097C9A2E8")
    ITAMMediaFormat : public IUnknown
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MediaFormat( 
            /* [retval][out] */ __RPC__deref_out_opt AM_MEDIA_TYPE **ppmt) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MediaFormat( 
            /* [in] */ __RPC__in const AM_MEDIA_TYPE *pmt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAMMediaFormatVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAMMediaFormat * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAMMediaFormat * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAMMediaFormat * This);
        
        DECLSPEC_XFGVIRT(ITAMMediaFormat, get_MediaFormat)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MediaFormat )( 
            __RPC__in ITAMMediaFormat * This,
            /* [retval][out] */ __RPC__deref_out_opt AM_MEDIA_TYPE **ppmt);
        
        DECLSPEC_XFGVIRT(ITAMMediaFormat, put_MediaFormat)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MediaFormat )( 
            __RPC__in ITAMMediaFormat * This,
            /* [in] */ __RPC__in const AM_MEDIA_TYPE *pmt);
        
        END_INTERFACE
    } ITAMMediaFormatVtbl;

    interface ITAMMediaFormat
    {
        CONST_VTBL struct ITAMMediaFormatVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAMMediaFormat_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAMMediaFormat_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAMMediaFormat_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAMMediaFormat_get_MediaFormat(This,ppmt)	\
    ( (This)->lpVtbl -> get_MediaFormat(This,ppmt) ) 

#define ITAMMediaFormat_put_MediaFormat(This,pmt)	\
    ( (This)->lpVtbl -> put_MediaFormat(This,pmt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAMMediaFormat_INTERFACE_DEFINED__ */


#ifndef __ITAllocatorProperties_INTERFACE_DEFINED__
#define __ITAllocatorProperties_INTERFACE_DEFINED__

/* interface ITAllocatorProperties */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_ITAllocatorProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C1BC3C90-BCFE-11D1-9745-00C04FD91AC0")
    ITAllocatorProperties : public IUnknown
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAllocatorProperties( 
            /* [in] */ __RPC__in ALLOCATOR_PROPERTIES *pAllocProperties) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAllocatorProperties( 
            /* [out] */ __RPC__out ALLOCATOR_PROPERTIES *pAllocProperties) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAllocateBuffers( 
            /* [in] */ BOOL bAllocBuffers) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetAllocateBuffers( 
            /* [out] */ __RPC__out BOOL *pbAllocBuffers) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetBufferSize( 
            /* [in] */ DWORD BufferSize) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBufferSize( 
            /* [out] */ __RPC__out DWORD *pBufferSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITAllocatorPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITAllocatorProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITAllocatorProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITAllocatorProperties * This);
        
        DECLSPEC_XFGVIRT(ITAllocatorProperties, SetAllocatorProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllocatorProperties )( 
            __RPC__in ITAllocatorProperties * This,
            /* [in] */ __RPC__in ALLOCATOR_PROPERTIES *pAllocProperties);
        
        DECLSPEC_XFGVIRT(ITAllocatorProperties, GetAllocatorProperties)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAllocatorProperties )( 
            __RPC__in ITAllocatorProperties * This,
            /* [out] */ __RPC__out ALLOCATOR_PROPERTIES *pAllocProperties);
        
        DECLSPEC_XFGVIRT(ITAllocatorProperties, SetAllocateBuffers)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllocateBuffers )( 
            __RPC__in ITAllocatorProperties * This,
            /* [in] */ BOOL bAllocBuffers);
        
        DECLSPEC_XFGVIRT(ITAllocatorProperties, GetAllocateBuffers)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetAllocateBuffers )( 
            __RPC__in ITAllocatorProperties * This,
            /* [out] */ __RPC__out BOOL *pbAllocBuffers);
        
        DECLSPEC_XFGVIRT(ITAllocatorProperties, SetBufferSize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetBufferSize )( 
            __RPC__in ITAllocatorProperties * This,
            /* [in] */ DWORD BufferSize);
        
        DECLSPEC_XFGVIRT(ITAllocatorProperties, GetBufferSize)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBufferSize )( 
            __RPC__in ITAllocatorProperties * This,
            /* [out] */ __RPC__out DWORD *pBufferSize);
        
        END_INTERFACE
    } ITAllocatorPropertiesVtbl;

    interface ITAllocatorProperties
    {
        CONST_VTBL struct ITAllocatorPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITAllocatorProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITAllocatorProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITAllocatorProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITAllocatorProperties_SetAllocatorProperties(This,pAllocProperties)	\
    ( (This)->lpVtbl -> SetAllocatorProperties(This,pAllocProperties) ) 

#define ITAllocatorProperties_GetAllocatorProperties(This,pAllocProperties)	\
    ( (This)->lpVtbl -> GetAllocatorProperties(This,pAllocProperties) ) 

#define ITAllocatorProperties_SetAllocateBuffers(This,bAllocBuffers)	\
    ( (This)->lpVtbl -> SetAllocateBuffers(This,bAllocBuffers) ) 

#define ITAllocatorProperties_GetAllocateBuffers(This,pbAllocBuffers)	\
    ( (This)->lpVtbl -> GetAllocateBuffers(This,pbAllocBuffers) ) 

#define ITAllocatorProperties_SetBufferSize(This,BufferSize)	\
    ( (This)->lpVtbl -> SetBufferSize(This,BufferSize) ) 

#define ITAllocatorProperties_GetBufferSize(This,pBufferSize)	\
    ( (This)->lpVtbl -> GetBufferSize(This,pBufferSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITAllocatorProperties_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tapi3ds_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tapi3ds_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tapi3ds_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


