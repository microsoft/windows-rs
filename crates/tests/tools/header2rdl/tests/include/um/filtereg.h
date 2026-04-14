

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

#ifndef __filtereg_h__
#define __filtereg_h__

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

#ifndef __ILoadFilter_FWD_DEFINED__
#define __ILoadFilter_FWD_DEFINED__
typedef interface ILoadFilter ILoadFilter;

#endif 	/* __ILoadFilter_FWD_DEFINED__ */


#ifndef __ILoadFilterWithPrivateComActivation_FWD_DEFINED__
#define __ILoadFilterWithPrivateComActivation_FWD_DEFINED__
typedef interface ILoadFilterWithPrivateComActivation ILoadFilterWithPrivateComActivation;

#endif 	/* __ILoadFilterWithPrivateComActivation_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "filter.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_filtereg_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_C const CLSID CLSID_FilterRegistration;


extern RPC_IF_HANDLE __MIDL_itf_filtereg_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_filtereg_0000_0000_v0_0_s_ifspec;

#ifndef __ILoadFilter_INTERFACE_DEFINED__
#define __ILoadFilter_INTERFACE_DEFINED__

/* interface ILoadFilter */
/* [unique][helpstring][uuid][object] */ 

typedef struct _FILTERED_DATA_SOURCES
    {
    const WCHAR *pwcsExtension;
    const WCHAR *pwcsMime;
    const CLSID *pClsid;
    const WCHAR *pwcsOverride;
    } 	FILTERED_DATA_SOURCES;


EXTERN_C const IID IID_ILoadFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7310722-ac80-11d1-8df3-00c04fb6ef4f")
    ILoadFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadIFilter( 
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwcsPath,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *pFilteredSources,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoadIFilterFromStorage( 
            /* [in] */ __RPC__in_opt IStorage *pStg,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwcsOverride,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoadIFilterFromStream( 
            /* [in] */ __RPC__in_opt IStream *pStm,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *pFilteredSources,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILoadFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILoadFilter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILoadFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILoadFilter * This);
        
        DECLSPEC_XFGVIRT(ILoadFilter, LoadIFilter)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilter )( 
            __RPC__in ILoadFilter * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwcsPath,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *pFilteredSources,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt);
        
        DECLSPEC_XFGVIRT(ILoadFilter, LoadIFilterFromStorage)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilterFromStorage )( 
            __RPC__in ILoadFilter * This,
            /* [in] */ __RPC__in_opt IStorage *pStg,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwcsOverride,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt);
        
        DECLSPEC_XFGVIRT(ILoadFilter, LoadIFilterFromStream)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilterFromStream )( 
            __RPC__in ILoadFilter * This,
            /* [in] */ __RPC__in_opt IStream *pStm,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *pFilteredSources,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt);
        
        END_INTERFACE
    } ILoadFilterVtbl;

    interface ILoadFilter
    {
        CONST_VTBL struct ILoadFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILoadFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILoadFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILoadFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILoadFilter_LoadIFilter(This,pwcsPath,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt)	\
    ( (This)->lpVtbl -> LoadIFilter(This,pwcsPath,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt) ) 

#define ILoadFilter_LoadIFilterFromStorage(This,pStg,pUnkOuter,pwcsOverride,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt)	\
    ( (This)->lpVtbl -> LoadIFilterFromStorage(This,pStg,pUnkOuter,pwcsOverride,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt) ) 

#define ILoadFilter_LoadIFilterFromStream(This,pStm,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt)	\
    ( (This)->lpVtbl -> LoadIFilterFromStream(This,pStm,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILoadFilter_INTERFACE_DEFINED__ */


#ifndef __ILoadFilterWithPrivateComActivation_INTERFACE_DEFINED__
#define __ILoadFilterWithPrivateComActivation_INTERFACE_DEFINED__

/* interface ILoadFilterWithPrivateComActivation */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_ILoadFilterWithPrivateComActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("40BDBD34-780B-48D3-9BB6-12EBD4AD2E75")
    ILoadFilterWithPrivateComActivation : public ILoadFilter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadIFilterWithPrivateComActivation( 
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *filteredSources,
            /* [in] */ BOOL useDefault,
            /* [out] */ __RPC__out CLSID *filterClsid,
            /* [out] */ __RPC__out BOOL *isFilterPrivateComActivated,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **filterObj) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILoadFilterWithPrivateComActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This);
        
        DECLSPEC_XFGVIRT(ILoadFilter, LoadIFilter)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilter )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwcsPath,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *pFilteredSources,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt);
        
        DECLSPEC_XFGVIRT(ILoadFilter, LoadIFilterFromStorage)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilterFromStorage )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This,
            /* [in] */ __RPC__in_opt IStorage *pStg,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [unique][in] */ __RPC__in_opt LPCWSTR pwcsOverride,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt);
        
        DECLSPEC_XFGVIRT(ILoadFilter, LoadIFilterFromStream)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilterFromStream )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This,
            /* [in] */ __RPC__in_opt IStream *pStm,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *pFilteredSources,
            /* [unique][in] */ __RPC__in_opt IUnknown *pUnkOuter,
            /* [in] */ BOOL fUseDefault,
            /* [unique][out][in] */ __RPC__inout_opt CLSID *pFilterClsid,
            /* [unique][out][in] */ __RPC__inout_opt int *SearchDecSize,
            /* [length_is][length_is][size_is][size_is][unique][out][in] */ __RPC__deref_opt_inout_ecount_part_opt(( *SearchDecSize + 1 ) , ( *SearchDecSize + 1 ) ) WCHAR **pwcsSearchDesc,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **ppIFilt);
        
        DECLSPEC_XFGVIRT(ILoadFilterWithPrivateComActivation, LoadIFilterWithPrivateComActivation)
        HRESULT ( STDMETHODCALLTYPE *LoadIFilterWithPrivateComActivation )( 
            __RPC__in ILoadFilterWithPrivateComActivation * This,
            /* [unique][in] */ __RPC__in_opt FILTERED_DATA_SOURCES *filteredSources,
            /* [in] */ BOOL useDefault,
            /* [out] */ __RPC__out CLSID *filterClsid,
            /* [out] */ __RPC__out BOOL *isFilterPrivateComActivated,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IFilter **filterObj);
        
        END_INTERFACE
    } ILoadFilterWithPrivateComActivationVtbl;

    interface ILoadFilterWithPrivateComActivation
    {
        CONST_VTBL struct ILoadFilterWithPrivateComActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILoadFilterWithPrivateComActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILoadFilterWithPrivateComActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILoadFilterWithPrivateComActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILoadFilterWithPrivateComActivation_LoadIFilter(This,pwcsPath,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt)	\
    ( (This)->lpVtbl -> LoadIFilter(This,pwcsPath,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt) ) 

#define ILoadFilterWithPrivateComActivation_LoadIFilterFromStorage(This,pStg,pUnkOuter,pwcsOverride,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt)	\
    ( (This)->lpVtbl -> LoadIFilterFromStorage(This,pStg,pUnkOuter,pwcsOverride,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt) ) 

#define ILoadFilterWithPrivateComActivation_LoadIFilterFromStream(This,pStm,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt)	\
    ( (This)->lpVtbl -> LoadIFilterFromStream(This,pStm,pFilteredSources,pUnkOuter,fUseDefault,pFilterClsid,SearchDecSize,pwcsSearchDesc,ppIFilt) ) 


#define ILoadFilterWithPrivateComActivation_LoadIFilterWithPrivateComActivation(This,filteredSources,useDefault,filterClsid,isFilterPrivateComActivated,filterObj)	\
    ( (This)->lpVtbl -> LoadIFilterWithPrivateComActivation(This,filteredSources,useDefault,filterClsid,isFilterPrivateComActivated,filterObj) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILoadFilterWithPrivateComActivation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_filtereg_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_filtereg_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_filtereg_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


