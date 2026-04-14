

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

#ifndef __fsrmtlb_h__
#define __fsrmtlb_h__

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

#ifndef __DIFsrmClassificationEvents_FWD_DEFINED__
#define __DIFsrmClassificationEvents_FWD_DEFINED__
typedef interface DIFsrmClassificationEvents DIFsrmClassificationEvents;

#endif 	/* __DIFsrmClassificationEvents_FWD_DEFINED__ */


#ifndef __DIFsrmClassificationEvents_FWD_DEFINED__
#define __DIFsrmClassificationEvents_FWD_DEFINED__
typedef interface DIFsrmClassificationEvents DIFsrmClassificationEvents;

#endif 	/* __DIFsrmClassificationEvents_FWD_DEFINED__ */


#ifndef __FsrmSetting_FWD_DEFINED__
#define __FsrmSetting_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmSetting FsrmSetting;
#else
typedef struct FsrmSetting FsrmSetting;
#endif /* __cplusplus */

#endif 	/* __FsrmSetting_FWD_DEFINED__ */


#ifndef __FsrmPathMapper_FWD_DEFINED__
#define __FsrmPathMapper_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmPathMapper FsrmPathMapper;
#else
typedef struct FsrmPathMapper FsrmPathMapper;
#endif /* __cplusplus */

#endif 	/* __FsrmPathMapper_FWD_DEFINED__ */


#ifndef __FsrmExportImport_FWD_DEFINED__
#define __FsrmExportImport_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmExportImport FsrmExportImport;
#else
typedef struct FsrmExportImport FsrmExportImport;
#endif /* __cplusplus */

#endif 	/* __FsrmExportImport_FWD_DEFINED__ */


#ifndef __FsrmQuotaManager_FWD_DEFINED__
#define __FsrmQuotaManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmQuotaManager FsrmQuotaManager;
#else
typedef struct FsrmQuotaManager FsrmQuotaManager;
#endif /* __cplusplus */

#endif 	/* __FsrmQuotaManager_FWD_DEFINED__ */


#ifndef __FsrmQuotaTemplateManager_FWD_DEFINED__
#define __FsrmQuotaTemplateManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmQuotaTemplateManager FsrmQuotaTemplateManager;
#else
typedef struct FsrmQuotaTemplateManager FsrmQuotaTemplateManager;
#endif /* __cplusplus */

#endif 	/* __FsrmQuotaTemplateManager_FWD_DEFINED__ */


#ifndef __FsrmFileGroupManager_FWD_DEFINED__
#define __FsrmFileGroupManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmFileGroupManager FsrmFileGroupManager;
#else
typedef struct FsrmFileGroupManager FsrmFileGroupManager;
#endif /* __cplusplus */

#endif 	/* __FsrmFileGroupManager_FWD_DEFINED__ */


#ifndef __FsrmFileScreenManager_FWD_DEFINED__
#define __FsrmFileScreenManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmFileScreenManager FsrmFileScreenManager;
#else
typedef struct FsrmFileScreenManager FsrmFileScreenManager;
#endif /* __cplusplus */

#endif 	/* __FsrmFileScreenManager_FWD_DEFINED__ */


#ifndef __FsrmFileScreenTemplateManager_FWD_DEFINED__
#define __FsrmFileScreenTemplateManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmFileScreenTemplateManager FsrmFileScreenTemplateManager;
#else
typedef struct FsrmFileScreenTemplateManager FsrmFileScreenTemplateManager;
#endif /* __cplusplus */

#endif 	/* __FsrmFileScreenTemplateManager_FWD_DEFINED__ */


#ifndef __FsrmReportManager_FWD_DEFINED__
#define __FsrmReportManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmReportManager FsrmReportManager;
#else
typedef struct FsrmReportManager FsrmReportManager;
#endif /* __cplusplus */

#endif 	/* __FsrmReportManager_FWD_DEFINED__ */


#ifndef __FsrmReportScheduler_FWD_DEFINED__
#define __FsrmReportScheduler_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmReportScheduler FsrmReportScheduler;
#else
typedef struct FsrmReportScheduler FsrmReportScheduler;
#endif /* __cplusplus */

#endif 	/* __FsrmReportScheduler_FWD_DEFINED__ */


#ifndef __FsrmFileManagementJobManager_FWD_DEFINED__
#define __FsrmFileManagementJobManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmFileManagementJobManager FsrmFileManagementJobManager;
#else
typedef struct FsrmFileManagementJobManager FsrmFileManagementJobManager;
#endif /* __cplusplus */

#endif 	/* __FsrmFileManagementJobManager_FWD_DEFINED__ */


#ifndef __FsrmClassificationManager_FWD_DEFINED__
#define __FsrmClassificationManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmClassificationManager FsrmClassificationManager;
#else
typedef struct FsrmClassificationManager FsrmClassificationManager;
#endif /* __cplusplus */

#endif 	/* __FsrmClassificationManager_FWD_DEFINED__ */


#ifndef __FsrmPipelineModuleConnector_FWD_DEFINED__
#define __FsrmPipelineModuleConnector_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmPipelineModuleConnector FsrmPipelineModuleConnector;
#else
typedef struct FsrmPipelineModuleConnector FsrmPipelineModuleConnector;
#endif /* __cplusplus */

#endif 	/* __FsrmPipelineModuleConnector_FWD_DEFINED__ */


#ifndef __AdSyncTask_FWD_DEFINED__
#define __AdSyncTask_FWD_DEFINED__

#ifdef __cplusplus
typedef class AdSyncTask AdSyncTask;
#else
typedef struct AdSyncTask AdSyncTask;
#endif /* __cplusplus */

#endif 	/* __AdSyncTask_FWD_DEFINED__ */


#ifndef __FsrmAccessDeniedRemediationClient_FWD_DEFINED__
#define __FsrmAccessDeniedRemediationClient_FWD_DEFINED__

#ifdef __cplusplus
typedef class FsrmAccessDeniedRemediationClient FsrmAccessDeniedRemediationClient;
#else
typedef struct FsrmAccessDeniedRemediationClient FsrmAccessDeniedRemediationClient;
#endif /* __cplusplus */

#endif 	/* __FsrmAccessDeniedRemediationClient_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "fsrmenums.h"
#include "fsrm.h"
#include "fsrmquota.h"
#include "fsrmscreen.h"
#include "fsrmreports.h"
#include "fsrmpipeline.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_fsrmtlb_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_fsrmtlb_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrmtlb_0000_0000_v0_0_s_ifspec;


#ifndef __FsrmLib_LIBRARY_DEFINED__
#define __FsrmLib_LIBRARY_DEFINED__

/* library FsrmLib */
/* [helpstring][version][uuid] */ 




























































































EXTERN_C const IID LIBID_FsrmLib;

#ifndef __DIFsrmClassificationEvents_DISPINTERFACE_DEFINED__
#define __DIFsrmClassificationEvents_DISPINTERFACE_DEFINED__

/* dispinterface DIFsrmClassificationEvents */
/* [helpstring][uuid] */ 


EXTERN_C const IID DIID_DIFsrmClassificationEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("26942db0-dabf-41d8-bbdd-b129a9f70424")
    DIFsrmClassificationEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct DIFsrmClassificationEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in DIFsrmClassificationEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in DIFsrmClassificationEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in DIFsrmClassificationEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in DIFsrmClassificationEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in DIFsrmClassificationEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in DIFsrmClassificationEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            DIFsrmClassificationEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } DIFsrmClassificationEventsVtbl;

    interface DIFsrmClassificationEvents
    {
        CONST_VTBL struct DIFsrmClassificationEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define DIFsrmClassificationEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define DIFsrmClassificationEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define DIFsrmClassificationEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define DIFsrmClassificationEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define DIFsrmClassificationEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define DIFsrmClassificationEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define DIFsrmClassificationEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* __DIFsrmClassificationEvents_DISPINTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_FsrmSetting;

#ifdef __cplusplus

class DECLSPEC_UUID("f556d708-6d4d-4594-9c61-7dbb0dae2a46")
FsrmSetting;
#endif

EXTERN_C const CLSID CLSID_FsrmPathMapper;

#ifdef __cplusplus

class DECLSPEC_UUID("f3be42bd-8ac2-409e-bbd8-faf9b6b41feb")
FsrmPathMapper;
#endif

EXTERN_C const CLSID CLSID_FsrmExportImport;

#ifdef __cplusplus

class DECLSPEC_UUID("1482dc37-fae9-4787-9025-8ce4e024ab56")
FsrmExportImport;
#endif

EXTERN_C const CLSID CLSID_FsrmQuotaManager;

#ifdef __cplusplus

class DECLSPEC_UUID("90dcab7f-347c-4bfc-b543-540326305fbe")
FsrmQuotaManager;
#endif

EXTERN_C const CLSID CLSID_FsrmQuotaTemplateManager;

#ifdef __cplusplus

class DECLSPEC_UUID("97d3d443-251c-4337-81e7-b32e8f4ee65e")
FsrmQuotaTemplateManager;
#endif

EXTERN_C const CLSID CLSID_FsrmFileGroupManager;

#ifdef __cplusplus

class DECLSPEC_UUID("8f1363f6-656f-4496-9226-13aecbd7718f")
FsrmFileGroupManager;
#endif

EXTERN_C const CLSID CLSID_FsrmFileScreenManager;

#ifdef __cplusplus

class DECLSPEC_UUID("95941183-db53-4c5f-b37b-7d0921cf9dc7")
FsrmFileScreenManager;
#endif

EXTERN_C const CLSID CLSID_FsrmFileScreenTemplateManager;

#ifdef __cplusplus

class DECLSPEC_UUID("243111df-e474-46aa-a054-eaa33edc292a")
FsrmFileScreenTemplateManager;
#endif

EXTERN_C const CLSID CLSID_FsrmReportManager;

#ifdef __cplusplus

class DECLSPEC_UUID("0058ef37-aa66-4c48-bd5b-2fce432ab0c8")
FsrmReportManager;
#endif

EXTERN_C const CLSID CLSID_FsrmReportScheduler;

#ifdef __cplusplus

class DECLSPEC_UUID("ea25f1b8-1b8d-4290-8ee8-e17c12c2fe20")
FsrmReportScheduler;
#endif

EXTERN_C const CLSID CLSID_FsrmFileManagementJobManager;

#ifdef __cplusplus

class DECLSPEC_UUID("eb18f9b2-4c3a-4321-b203-205120cff614")
FsrmFileManagementJobManager;
#endif

EXTERN_C const CLSID CLSID_FsrmClassificationManager;

#ifdef __cplusplus

class DECLSPEC_UUID("b15c0e47-c391-45b9-95c8-eb596c853f3a")
FsrmClassificationManager;
#endif

EXTERN_C const CLSID CLSID_FsrmPipelineModuleConnector;

#ifdef __cplusplus

class DECLSPEC_UUID("c7643375-1eb5-44de-a062-623547d933bc")
FsrmPipelineModuleConnector;
#endif

EXTERN_C const CLSID CLSID_AdSyncTask;

#ifdef __cplusplus

class DECLSPEC_UUID("2ae64751-b728-4d6b-97a0-b2da2e7d2a3b")
AdSyncTask;
#endif

EXTERN_C const CLSID CLSID_FsrmAccessDeniedRemediationClient;

#ifdef __cplusplus

class DECLSPEC_UUID("100B4FC8-74C1-470F-B1B7-DD7B6BAE79BD")
FsrmAccessDeniedRemediationClient;
#endif
#endif /* __FsrmLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_fsrmtlb_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_fsrmtlb_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrmtlb_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


