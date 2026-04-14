

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

#ifndef __thumbnailstreamcache_h__
#define __thumbnailstreamcache_h__

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

#ifndef __IThumbnailStreamCache_FWD_DEFINED__
#define __IThumbnailStreamCache_FWD_DEFINED__
typedef interface IThumbnailStreamCache IThumbnailStreamCache;

#endif 	/* __IThumbnailStreamCache_FWD_DEFINED__ */


#ifndef __ThumbnailStreamCache_FWD_DEFINED__
#define __ThumbnailStreamCache_FWD_DEFINED__

#ifdef __cplusplus
typedef class ThumbnailStreamCache ThumbnailStreamCache;
#else
typedef struct ThumbnailStreamCache ThumbnailStreamCache;
#endif /* __cplusplus */

#endif 	/* __ThumbnailStreamCache_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "shtypes.h"
#include "shobjidl_core.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_thumbnailstreamcache_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region App Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [v1_enum] */ 
enum ThumbnailStreamCacheOptions
    {
        ExtractIfNotCached	= 0,
        ReturnOnlyIfCached	= 0x1,
        ResizeThumbnail	= 0x2,
        AllowSmallerSize	= 0x4
    } 	ThumbnailStreamCacheOptions;

DEFINE_ENUM_FLAG_OPERATORS(ThumbnailStreamCacheOptions)


extern RPC_IF_HANDLE __MIDL_itf_thumbnailstreamcache_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_thumbnailstreamcache_0000_0000_v0_0_s_ifspec;

#ifndef __IThumbnailStreamCache_INTERFACE_DEFINED__
#define __IThumbnailStreamCache_INTERFACE_DEFINED__

/* interface IThumbnailStreamCache */
/* [uuid][object] */ 


EXTERN_C const IID IID_IThumbnailStreamCache;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90E11430-9569-41D8-AE75-6D4D2AE7CCA0")
    IThumbnailStreamCache : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetThumbnailStream( 
            /* [in] */ __RPC__in LPCWSTR path,
            /* [in] */ ULONGLONG cacheId,
            /* [in] */ ThumbnailStreamCacheOptions options,
            /* [in] */ UINT requestedThumbnailSize,
            /* [out] */ __RPC__out SIZE *thumbnailSize,
            /* [out] */ __RPC__deref_out_opt IStream **thumbnailStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThumbnailStream( 
            /* [in] */ __RPC__in LPCWSTR path,
            /* [in] */ ULONGLONG cacheId,
            /* [in] */ SIZE thumbnailSize,
            /* [in] */ __RPC__in_opt IStream *thumbnailStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IThumbnailStreamCacheVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IThumbnailStreamCache * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IThumbnailStreamCache * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IThumbnailStreamCache * This);
        
        DECLSPEC_XFGVIRT(IThumbnailStreamCache, GetThumbnailStream)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnailStream )( 
            __RPC__in IThumbnailStreamCache * This,
            /* [in] */ __RPC__in LPCWSTR path,
            /* [in] */ ULONGLONG cacheId,
            /* [in] */ ThumbnailStreamCacheOptions options,
            /* [in] */ UINT requestedThumbnailSize,
            /* [out] */ __RPC__out SIZE *thumbnailSize,
            /* [out] */ __RPC__deref_out_opt IStream **thumbnailStream);
        
        DECLSPEC_XFGVIRT(IThumbnailStreamCache, SetThumbnailStream)
        HRESULT ( STDMETHODCALLTYPE *SetThumbnailStream )( 
            __RPC__in IThumbnailStreamCache * This,
            /* [in] */ __RPC__in LPCWSTR path,
            /* [in] */ ULONGLONG cacheId,
            /* [in] */ SIZE thumbnailSize,
            /* [in] */ __RPC__in_opt IStream *thumbnailStream);
        
        END_INTERFACE
    } IThumbnailStreamCacheVtbl;

    interface IThumbnailStreamCache
    {
        CONST_VTBL struct IThumbnailStreamCacheVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IThumbnailStreamCache_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IThumbnailStreamCache_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IThumbnailStreamCache_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IThumbnailStreamCache_GetThumbnailStream(This,path,cacheId,options,requestedThumbnailSize,thumbnailSize,thumbnailStream)	\
    ( (This)->lpVtbl -> GetThumbnailStream(This,path,cacheId,options,requestedThumbnailSize,thumbnailSize,thumbnailStream) ) 

#define IThumbnailStreamCache_SetThumbnailStream(This,path,cacheId,thumbnailSize,thumbnailStream)	\
    ( (This)->lpVtbl -> SetThumbnailStream(This,path,cacheId,thumbnailSize,thumbnailStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IThumbnailStreamCache_INTERFACE_DEFINED__ */



#ifndef __ThumbnailStreamCacheLib_LIBRARY_DEFINED__
#define __ThumbnailStreamCacheLib_LIBRARY_DEFINED__

/* library ThumbnailStreamCacheLib */
/* [uuid] */ 


EXTERN_C const IID LIBID_ThumbnailStreamCacheLib;

EXTERN_C const CLSID CLSID_ThumbnailStreamCache;

#ifdef __cplusplus

class DECLSPEC_UUID("CBE0FED3-4B91-4E90-8354-8A8C84EC6872")
ThumbnailStreamCache;
#endif
#endif /* __ThumbnailStreamCacheLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_thumbnailstreamcache_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_thumbnailstreamcache_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_thumbnailstreamcache_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


