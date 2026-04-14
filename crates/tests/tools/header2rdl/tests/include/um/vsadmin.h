

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

#ifndef __vsadmin_h__
#define __vsadmin_h__

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

#ifndef __IVssAdmin_FWD_DEFINED__
#define __IVssAdmin_FWD_DEFINED__
typedef interface IVssAdmin IVssAdmin;

#endif 	/* __IVssAdmin_FWD_DEFINED__ */


#ifndef __IVssAdminEx_FWD_DEFINED__
#define __IVssAdminEx_FWD_DEFINED__
typedef interface IVssAdminEx IVssAdminEx;

#endif 	/* __IVssAdminEx_FWD_DEFINED__ */


#ifndef __VSSCoordinator_FWD_DEFINED__
#define __VSSCoordinator_FWD_DEFINED__

#ifdef __cplusplus
typedef class VSSCoordinator VSSCoordinator;
#else
typedef struct VSSCoordinator VSSCoordinator;
#endif /* __cplusplus */

#endif 	/* __VSSCoordinator_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "vss.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_vsadmin_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_vsadmin_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vsadmin_0000_0000_v0_0_s_ifspec;

#ifndef __IVssAdmin_INTERFACE_DEFINED__
#define __IVssAdmin_INTERFACE_DEFINED__

/* interface IVssAdmin */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssAdmin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("77ED5996-2F63-11d3-8A39-00C04F72D8E3")
    IVssAdmin : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RegisterProvider( 
            /* [in] */ VSS_ID pProviderId,
            /* [in] */ CLSID ClassId,
            /* [in] */ __RPC__in VSS_PWSZ pwszProviderName,
            /* [in] */ VSS_PROVIDER_TYPE eProviderType,
            /* [in] */ __RPC__in VSS_PWSZ pwszProviderVersion,
            /* [in] */ VSS_ID ProviderVersionId) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE UnregisterProvider( 
            /* [in] */ VSS_ID ProviderId) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE QueryProviders( 
            /* [out] */ __RPC__deref_out_opt IVssEnumObject **ppEnum) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE AbortAllSnapshotsInProgress( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssAdminVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssAdmin * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssAdmin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssAdmin * This);
        
        DECLSPEC_XFGVIRT(IVssAdmin, RegisterProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterProvider )( 
            __RPC__in IVssAdmin * This,
            /* [in] */ VSS_ID pProviderId,
            /* [in] */ CLSID ClassId,
            /* [in] */ __RPC__in VSS_PWSZ pwszProviderName,
            /* [in] */ VSS_PROVIDER_TYPE eProviderType,
            /* [in] */ __RPC__in VSS_PWSZ pwszProviderVersion,
            /* [in] */ VSS_ID ProviderVersionId);
        
        DECLSPEC_XFGVIRT(IVssAdmin, UnregisterProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterProvider )( 
            __RPC__in IVssAdmin * This,
            /* [in] */ VSS_ID ProviderId);
        
        DECLSPEC_XFGVIRT(IVssAdmin, QueryProviders)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryProviders )( 
            __RPC__in IVssAdmin * This,
            /* [out] */ __RPC__deref_out_opt IVssEnumObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssAdmin, AbortAllSnapshotsInProgress)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AbortAllSnapshotsInProgress )( 
            __RPC__in IVssAdmin * This);
        
        END_INTERFACE
    } IVssAdminVtbl;

    interface IVssAdmin
    {
        CONST_VTBL struct IVssAdminVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssAdmin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssAdmin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssAdmin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssAdmin_RegisterProvider(This,pProviderId,ClassId,pwszProviderName,eProviderType,pwszProviderVersion,ProviderVersionId)	\
    ( (This)->lpVtbl -> RegisterProvider(This,pProviderId,ClassId,pwszProviderName,eProviderType,pwszProviderVersion,ProviderVersionId) ) 

#define IVssAdmin_UnregisterProvider(This,ProviderId)	\
    ( (This)->lpVtbl -> UnregisterProvider(This,ProviderId) ) 

#define IVssAdmin_QueryProviders(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryProviders(This,ppEnum) ) 

#define IVssAdmin_AbortAllSnapshotsInProgress(This)	\
    ( (This)->lpVtbl -> AbortAllSnapshotsInProgress(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssAdmin_INTERFACE_DEFINED__ */


#ifndef __IVssAdminEx_INTERFACE_DEFINED__
#define __IVssAdminEx_INTERFACE_DEFINED__

/* interface IVssAdminEx */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IVssAdminEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7858A9F8-B1FA-41a6-964F-B9B36B8CD8D8")
    IVssAdminEx : public IVssAdmin
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProviderCapability( 
            /* [in] */ VSS_ID pProviderId,
            /* [out] */ __RPC__out ULONGLONG *pllOriginalCapabilityMask) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetProviderContext( 
            /* [in] */ VSS_ID ProviderId,
            /* [out] */ __RPC__out LONG *plContext) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetProviderContext( 
            /* [in] */ VSS_ID ProviderId,
            /* [in] */ LONG lContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVssAdminExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVssAdminEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVssAdminEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVssAdminEx * This);
        
        DECLSPEC_XFGVIRT(IVssAdmin, RegisterProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RegisterProvider )( 
            __RPC__in IVssAdminEx * This,
            /* [in] */ VSS_ID pProviderId,
            /* [in] */ CLSID ClassId,
            /* [in] */ __RPC__in VSS_PWSZ pwszProviderName,
            /* [in] */ VSS_PROVIDER_TYPE eProviderType,
            /* [in] */ __RPC__in VSS_PWSZ pwszProviderVersion,
            /* [in] */ VSS_ID ProviderVersionId);
        
        DECLSPEC_XFGVIRT(IVssAdmin, UnregisterProvider)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *UnregisterProvider )( 
            __RPC__in IVssAdminEx * This,
            /* [in] */ VSS_ID ProviderId);
        
        DECLSPEC_XFGVIRT(IVssAdmin, QueryProviders)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *QueryProviders )( 
            __RPC__in IVssAdminEx * This,
            /* [out] */ __RPC__deref_out_opt IVssEnumObject **ppEnum);
        
        DECLSPEC_XFGVIRT(IVssAdmin, AbortAllSnapshotsInProgress)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *AbortAllSnapshotsInProgress )( 
            __RPC__in IVssAdminEx * This);
        
        DECLSPEC_XFGVIRT(IVssAdminEx, GetProviderCapability)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProviderCapability )( 
            __RPC__in IVssAdminEx * This,
            /* [in] */ VSS_ID pProviderId,
            /* [out] */ __RPC__out ULONGLONG *pllOriginalCapabilityMask);
        
        DECLSPEC_XFGVIRT(IVssAdminEx, GetProviderContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetProviderContext )( 
            __RPC__in IVssAdminEx * This,
            /* [in] */ VSS_ID ProviderId,
            /* [out] */ __RPC__out LONG *plContext);
        
        DECLSPEC_XFGVIRT(IVssAdminEx, SetProviderContext)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetProviderContext )( 
            __RPC__in IVssAdminEx * This,
            /* [in] */ VSS_ID ProviderId,
            /* [in] */ LONG lContext);
        
        END_INTERFACE
    } IVssAdminExVtbl;

    interface IVssAdminEx
    {
        CONST_VTBL struct IVssAdminExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVssAdminEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVssAdminEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVssAdminEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVssAdminEx_RegisterProvider(This,pProviderId,ClassId,pwszProviderName,eProviderType,pwszProviderVersion,ProviderVersionId)	\
    ( (This)->lpVtbl -> RegisterProvider(This,pProviderId,ClassId,pwszProviderName,eProviderType,pwszProviderVersion,ProviderVersionId) ) 

#define IVssAdminEx_UnregisterProvider(This,ProviderId)	\
    ( (This)->lpVtbl -> UnregisterProvider(This,ProviderId) ) 

#define IVssAdminEx_QueryProviders(This,ppEnum)	\
    ( (This)->lpVtbl -> QueryProviders(This,ppEnum) ) 

#define IVssAdminEx_AbortAllSnapshotsInProgress(This)	\
    ( (This)->lpVtbl -> AbortAllSnapshotsInProgress(This) ) 


#define IVssAdminEx_GetProviderCapability(This,pProviderId,pllOriginalCapabilityMask)	\
    ( (This)->lpVtbl -> GetProviderCapability(This,pProviderId,pllOriginalCapabilityMask) ) 

#define IVssAdminEx_GetProviderContext(This,ProviderId,plContext)	\
    ( (This)->lpVtbl -> GetProviderContext(This,ProviderId,plContext) ) 

#define IVssAdminEx_SetProviderContext(This,ProviderId,lContext)	\
    ( (This)->lpVtbl -> SetProviderContext(This,ProviderId,lContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVssAdminEx_INTERFACE_DEFINED__ */



#ifndef __VSSAdmin_LIBRARY_DEFINED__
#define __VSSAdmin_LIBRARY_DEFINED__

/* library VSSAdmin */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_VSSAdmin;

EXTERN_C const CLSID CLSID_VSSCoordinator;

#ifdef __cplusplus

class DECLSPEC_UUID("E579AB5F-1CC4-44b4-BED9-DE0991FF0623")
VSSCoordinator;
#endif
#endif /* __VSSAdmin_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_vsadmin_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_vsadmin_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_vsadmin_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


