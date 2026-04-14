

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

#ifndef __tspubplugincom_h__
#define __tspubplugincom_h__

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

#ifndef __ItsPubPlugin_FWD_DEFINED__
#define __ItsPubPlugin_FWD_DEFINED__
typedef interface ItsPubPlugin ItsPubPlugin;

#endif 	/* __ItsPubPlugin_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tspubplugincom_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_tspubplugincom_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tspubplugincom_0000_0000_v0_0_s_ifspec;

#ifndef __ItsPubPlugin_INTERFACE_DEFINED__
#define __ItsPubPlugin_INTERFACE_DEFINED__

/* interface ItsPubPlugin */
/* [unique][helpstring][uuid][object] */ 

typedef /* [public][public][public] */ struct __MIDL_ItsPubPlugin_0001
    {
    WCHAR alias[ 256 ];
    WCHAR name[ 256 ];
    /* [unique][string] */ WCHAR *resourceFileContents;
    WCHAR fileExtension[ 256 ];
    WCHAR resourcePluginType[ 256 ];
    boolean isDiscoverable;
    long resourceType;
    unsigned long pceIconSize;
    /* [size_is] */ byte *iconContents;
    unsigned long pcePluginBlobSize;
    /* [size_is] */ byte *blobContents;
    } 	pluginResource;


EXTERN_C const IID IID_ItsPubPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70C04B05-F347-412B-822F-36C99C54CA45")
    ItsPubPlugin : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetResourceList( 
            /* [in] */ __RPC__in LPCWSTR userID,
            /* [out] */ __RPC__out LONG *pceAppListSize,
            /* [out] */ __RPC__deref_out_opt pluginResource **resourceList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResource( 
            /* [in] */ __RPC__in LPCWSTR alias,
            LONG flags,
            /* [out] */ __RPC__out pluginResource *resource) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetCacheLastUpdateTime( 
            /* [out] */ __RPC__out unsigned long long *lastUpdateTime) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_pluginName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_pluginVersion( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResolveResource( 
            /* [out] */ __RPC__out DWORD *resourceType,
            /* [out][string] */ __RPC__out_ecount_full_string(256) wchar_t resourceLocation[ 256 ],
            /* [out][string] */ __RPC__out_ecount_full_string(256) wchar_t endPointName[ 256 ],
            /* [string][in] */ __RPC__in_string wchar_t *userID,
            /* [string][in] */ __RPC__in_string wchar_t *alias) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ItsPubPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ItsPubPlugin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ItsPubPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ItsPubPlugin * This);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, GetResourceList)
        HRESULT ( STDMETHODCALLTYPE *GetResourceList )( 
            __RPC__in ItsPubPlugin * This,
            /* [in] */ __RPC__in LPCWSTR userID,
            /* [out] */ __RPC__out LONG *pceAppListSize,
            /* [out] */ __RPC__deref_out_opt pluginResource **resourceList);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, GetResource)
        HRESULT ( STDMETHODCALLTYPE *GetResource )( 
            __RPC__in ItsPubPlugin * This,
            /* [in] */ __RPC__in LPCWSTR alias,
            LONG flags,
            /* [out] */ __RPC__out pluginResource *resource);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, GetCacheLastUpdateTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCacheLastUpdateTime )( 
            __RPC__in ItsPubPlugin * This,
            /* [out] */ __RPC__out unsigned long long *lastUpdateTime);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, get_pluginName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_pluginName )( 
            __RPC__in ItsPubPlugin * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, get_pluginVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_pluginVersion )( 
            __RPC__in ItsPubPlugin * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, ResolveResource)
        HRESULT ( STDMETHODCALLTYPE *ResolveResource )( 
            __RPC__in ItsPubPlugin * This,
            /* [out] */ __RPC__out DWORD *resourceType,
            /* [out][string] */ __RPC__out_ecount_full_string(256) wchar_t resourceLocation[ 256 ],
            /* [out][string] */ __RPC__out_ecount_full_string(256) wchar_t endPointName[ 256 ],
            /* [string][in] */ __RPC__in_string wchar_t *userID,
            /* [string][in] */ __RPC__in_string wchar_t *alias);
        
        END_INTERFACE
    } ItsPubPluginVtbl;

    interface ItsPubPlugin
    {
        CONST_VTBL struct ItsPubPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ItsPubPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ItsPubPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ItsPubPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ItsPubPlugin_GetResourceList(This,userID,pceAppListSize,resourceList)	\
    ( (This)->lpVtbl -> GetResourceList(This,userID,pceAppListSize,resourceList) ) 

#define ItsPubPlugin_GetResource(This,alias,flags,resource)	\
    ( (This)->lpVtbl -> GetResource(This,alias,flags,resource) ) 

#define ItsPubPlugin_GetCacheLastUpdateTime(This,lastUpdateTime)	\
    ( (This)->lpVtbl -> GetCacheLastUpdateTime(This,lastUpdateTime) ) 

#define ItsPubPlugin_get_pluginName(This,pVal)	\
    ( (This)->lpVtbl -> get_pluginName(This,pVal) ) 

#define ItsPubPlugin_get_pluginVersion(This,pVal)	\
    ( (This)->lpVtbl -> get_pluginVersion(This,pVal) ) 

#define ItsPubPlugin_ResolveResource(This,resourceType,resourceLocation,endPointName,userID,alias)	\
    ( (This)->lpVtbl -> ResolveResource(This,resourceType,resourceLocation,endPointName,userID,alias) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ItsPubPlugin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tspubplugincom_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tspubplugincom_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tspubplugincom_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


