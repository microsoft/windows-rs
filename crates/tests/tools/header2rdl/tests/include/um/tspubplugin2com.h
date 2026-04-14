

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

#ifndef __tspubplugin2com_h__
#define __tspubplugin2com_h__

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

#ifndef __ItsPubPlugin2_FWD_DEFINED__
#define __ItsPubPlugin2_FWD_DEFINED__
typedef interface ItsPubPlugin2 ItsPubPlugin2;

#endif 	/* __ItsPubPlugin2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "tspubplugincom.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_tspubplugin2com_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_tspubplugin2com_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tspubplugin2com_0000_0000_v0_0_s_ifspec;

#ifndef __ItsPubPlugin2_INTERFACE_DEFINED__
#define __ItsPubPlugin2_INTERFACE_DEFINED__

/* interface ItsPubPlugin2 */
/* [unique][helpstring][uuid][object] */ 

typedef /* [public][public][public][public] */ struct __MIDL_ItsPubPlugin2_0001
    {
    WCHAR extName[ 256 ];
    boolean primaryHandler;
    unsigned long pceIconSize;
    /* [size_is] */ byte *iconContents;
    } 	pluginResource2FileAssociation;

typedef /* [string] */  __RPC_string WCHAR *pluginFolderName;

typedef /* [public][public][public] */ struct __MIDL_ItsPubPlugin2_0002
    {
    pluginResource resourceV1;
    unsigned long pceFileAssocListSize;
    /* [size_is] */ pluginResource2FileAssociation *fileAssocList;
    /* [unique][string] */ WCHAR *securityDescriptor;
    unsigned long pceFolderListSize;
    /* [size_is] */ pluginFolderName *folderList;
    } 	pluginResource2;

typedef 
enum _TSPUB_PLUGIN_PD_RESOLUTION_TYPE
    {
        TSPUB_PLUGIN_PD_QUERY_OR_CREATE	= 0,
        TSPUB_PLUGIN_PD_QUERY_EXISTING	= 1
    } 	TSPUB_PLUGIN_PD_RESOLUTION_TYPE;

typedef 
enum _TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE
    {
        TSPUB_PLUGIN_PD_ASSIGNMENT_NEW	= 0,
        TSPUB_PLUGIN_PD_ASSIGNMENT_EXISTING	= 1
    } 	TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE;


EXTERN_C const IID IID_ItsPubPlugin2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FA4CE418-AAD7-4ec6-BAD1-0A321BA465D5")
    ItsPubPlugin2 : public ItsPubPlugin
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetResource2List( 
            /* [in] */ __RPC__in LPCWSTR userID,
            /* [out] */ __RPC__out LONG *pceAppListSize,
            /* [out] */ __RPC__deref_out_opt pluginResource2 **resourceList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResource2( 
            /* [in] */ __RPC__in LPCWSTR alias,
            LONG flags,
            /* [out] */ __RPC__out pluginResource2 *resource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResolvePersonalDesktop( 
            /* [string][in] */ __RPC__in_string const wchar_t *userId,
            /* [string][in] */ __RPC__in_string const wchar_t *poolId,
            /* [in] */ TSPUB_PLUGIN_PD_RESOLUTION_TYPE ePdResolutionType,
            /* [out] */ __RPC__out TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE *pPdAssignmentType,
            /* [string][out] */ __RPC__out_ecount_full_string(256) wchar_t endPointName[ 256 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeletePersonalDesktopAssignment( 
            /* [string][in] */ __RPC__in_string const wchar_t *userId,
            /* [string][in] */ __RPC__in_string const wchar_t *poolId,
            /* [string][in] */ __RPC__in_string const wchar_t *endpointName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ItsPubPlugin2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ItsPubPlugin2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ItsPubPlugin2 * This);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, GetResourceList)
        HRESULT ( STDMETHODCALLTYPE *GetResourceList )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [in] */ __RPC__in LPCWSTR userID,
            /* [out] */ __RPC__out LONG *pceAppListSize,
            /* [out] */ __RPC__deref_out_opt pluginResource **resourceList);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, GetResource)
        HRESULT ( STDMETHODCALLTYPE *GetResource )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [in] */ __RPC__in LPCWSTR alias,
            LONG flags,
            /* [out] */ __RPC__out pluginResource *resource);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, GetCacheLastUpdateTime)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCacheLastUpdateTime )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [out] */ __RPC__out unsigned long long *lastUpdateTime);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, get_pluginName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_pluginName )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, get_pluginVersion)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_pluginVersion )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin, ResolveResource)
        HRESULT ( STDMETHODCALLTYPE *ResolveResource )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [out] */ __RPC__out DWORD *resourceType,
            /* [out][string] */ __RPC__out_ecount_full_string(256) wchar_t resourceLocation[ 256 ],
            /* [out][string] */ __RPC__out_ecount_full_string(256) wchar_t endPointName[ 256 ],
            /* [string][in] */ __RPC__in_string wchar_t *userID,
            /* [string][in] */ __RPC__in_string wchar_t *alias);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin2, GetResource2List)
        HRESULT ( STDMETHODCALLTYPE *GetResource2List )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [in] */ __RPC__in LPCWSTR userID,
            /* [out] */ __RPC__out LONG *pceAppListSize,
            /* [out] */ __RPC__deref_out_opt pluginResource2 **resourceList);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin2, GetResource2)
        HRESULT ( STDMETHODCALLTYPE *GetResource2 )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [in] */ __RPC__in LPCWSTR alias,
            LONG flags,
            /* [out] */ __RPC__out pluginResource2 *resource);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin2, ResolvePersonalDesktop)
        HRESULT ( STDMETHODCALLTYPE *ResolvePersonalDesktop )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [string][in] */ __RPC__in_string const wchar_t *userId,
            /* [string][in] */ __RPC__in_string const wchar_t *poolId,
            /* [in] */ TSPUB_PLUGIN_PD_RESOLUTION_TYPE ePdResolutionType,
            /* [out] */ __RPC__out TSPUB_PLUGIN_PD_ASSIGNMENT_TYPE *pPdAssignmentType,
            /* [string][out] */ __RPC__out_ecount_full_string(256) wchar_t endPointName[ 256 ]);
        
        DECLSPEC_XFGVIRT(ItsPubPlugin2, DeletePersonalDesktopAssignment)
        HRESULT ( STDMETHODCALLTYPE *DeletePersonalDesktopAssignment )( 
            __RPC__in ItsPubPlugin2 * This,
            /* [string][in] */ __RPC__in_string const wchar_t *userId,
            /* [string][in] */ __RPC__in_string const wchar_t *poolId,
            /* [string][in] */ __RPC__in_string const wchar_t *endpointName);
        
        END_INTERFACE
    } ItsPubPlugin2Vtbl;

    interface ItsPubPlugin2
    {
        CONST_VTBL struct ItsPubPlugin2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ItsPubPlugin2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ItsPubPlugin2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ItsPubPlugin2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ItsPubPlugin2_GetResourceList(This,userID,pceAppListSize,resourceList)	\
    ( (This)->lpVtbl -> GetResourceList(This,userID,pceAppListSize,resourceList) ) 

#define ItsPubPlugin2_GetResource(This,alias,flags,resource)	\
    ( (This)->lpVtbl -> GetResource(This,alias,flags,resource) ) 

#define ItsPubPlugin2_GetCacheLastUpdateTime(This,lastUpdateTime)	\
    ( (This)->lpVtbl -> GetCacheLastUpdateTime(This,lastUpdateTime) ) 

#define ItsPubPlugin2_get_pluginName(This,pVal)	\
    ( (This)->lpVtbl -> get_pluginName(This,pVal) ) 

#define ItsPubPlugin2_get_pluginVersion(This,pVal)	\
    ( (This)->lpVtbl -> get_pluginVersion(This,pVal) ) 

#define ItsPubPlugin2_ResolveResource(This,resourceType,resourceLocation,endPointName,userID,alias)	\
    ( (This)->lpVtbl -> ResolveResource(This,resourceType,resourceLocation,endPointName,userID,alias) ) 


#define ItsPubPlugin2_GetResource2List(This,userID,pceAppListSize,resourceList)	\
    ( (This)->lpVtbl -> GetResource2List(This,userID,pceAppListSize,resourceList) ) 

#define ItsPubPlugin2_GetResource2(This,alias,flags,resource)	\
    ( (This)->lpVtbl -> GetResource2(This,alias,flags,resource) ) 

#define ItsPubPlugin2_ResolvePersonalDesktop(This,userId,poolId,ePdResolutionType,pPdAssignmentType,endPointName)	\
    ( (This)->lpVtbl -> ResolvePersonalDesktop(This,userId,poolId,ePdResolutionType,pPdAssignmentType,endPointName) ) 

#define ItsPubPlugin2_DeletePersonalDesktopAssignment(This,userId,poolId,endpointName)	\
    ( (This)->lpVtbl -> DeletePersonalDesktopAssignment(This,userId,poolId,endpointName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ItsPubPlugin2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_tspubplugin2com_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_tspubplugin2com_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_tspubplugin2com_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


