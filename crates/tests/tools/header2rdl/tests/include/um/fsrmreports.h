

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

#ifndef __fsrmreports_h__
#define __fsrmreports_h__

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

#ifndef __IFsrmReportManager_FWD_DEFINED__
#define __IFsrmReportManager_FWD_DEFINED__
typedef interface IFsrmReportManager IFsrmReportManager;

#endif 	/* __IFsrmReportManager_FWD_DEFINED__ */


#ifndef __IFsrmReportJob_FWD_DEFINED__
#define __IFsrmReportJob_FWD_DEFINED__
typedef interface IFsrmReportJob IFsrmReportJob;

#endif 	/* __IFsrmReportJob_FWD_DEFINED__ */


#ifndef __IFsrmReport_FWD_DEFINED__
#define __IFsrmReport_FWD_DEFINED__
typedef interface IFsrmReport IFsrmReport;

#endif 	/* __IFsrmReport_FWD_DEFINED__ */


#ifndef __IFsrmReportScheduler_FWD_DEFINED__
#define __IFsrmReportScheduler_FWD_DEFINED__
typedef interface IFsrmReportScheduler IFsrmReportScheduler;

#endif 	/* __IFsrmReportScheduler_FWD_DEFINED__ */


#ifndef __IFsrmFileManagementJobManager_FWD_DEFINED__
#define __IFsrmFileManagementJobManager_FWD_DEFINED__
typedef interface IFsrmFileManagementJobManager IFsrmFileManagementJobManager;

#endif 	/* __IFsrmFileManagementJobManager_FWD_DEFINED__ */


#ifndef __IFsrmFileManagementJob_FWD_DEFINED__
#define __IFsrmFileManagementJob_FWD_DEFINED__
typedef interface IFsrmFileManagementJob IFsrmFileManagementJob;

#endif 	/* __IFsrmFileManagementJob_FWD_DEFINED__ */


#ifndef __IFsrmPropertyCondition_FWD_DEFINED__
#define __IFsrmPropertyCondition_FWD_DEFINED__
typedef interface IFsrmPropertyCondition IFsrmPropertyCondition;

#endif 	/* __IFsrmPropertyCondition_FWD_DEFINED__ */


#ifndef __IFsrmFileCondition_FWD_DEFINED__
#define __IFsrmFileCondition_FWD_DEFINED__
typedef interface IFsrmFileCondition IFsrmFileCondition;

#endif 	/* __IFsrmFileCondition_FWD_DEFINED__ */


#ifndef __IFsrmFileConditionProperty_FWD_DEFINED__
#define __IFsrmFileConditionProperty_FWD_DEFINED__
typedef interface IFsrmFileConditionProperty IFsrmFileConditionProperty;

#endif 	/* __IFsrmFileConditionProperty_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "fsrmenums.h"
#include "fsrm.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_fsrmreports_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






#define	FSRM_DISPID_REPORT_MANAGER	( ( FSRM_DISPID_FEATURE_REPORTS | 0x100000 )  )

#define	FSRM_DISPID_REPORT_JOB	( ( FSRM_DISPID_FEATURE_REPORTS | 0x200000 )  )

#define	FSRM_DISPID_REPORT	( ( FSRM_DISPID_FEATURE_REPORTS | 0x300000 )  )

#define	FSRM_DISPID_REPORT_SCHEDULER	( ( FSRM_DISPID_FEATURE_REPORTS | 0x400000 )  )

#define	FSRM_DISPID_FILE_MANAGEMENT_JOB_MANAGER	( ( FSRM_DISPID_FEATURE_REPORTS | 0x500000 )  )

#define	FSRM_DISPID_FILE_MANAGEMENT_JOB	( ( FSRM_DISPID_FEATURE_REPORTS | 0x600000 )  )

#define	FSRM_DISPID_FILE_MANAGEMENT_NOTIFICATION	( ( FSRM_DISPID_FEATURE_REPORTS | 0x700000 )  )

#define	FSRM_DISPID_PROPERTY_CONDITION	( ( FSRM_DISPID_FEATURE_REPORTS | 0x800000 )  )

#define	FSRM_DISPID_FILE_CONDITION	( ( FSRM_DISPID_FEATURE_REPORTS | 0x900000 )  )

#define	FSRM_DISPID_FILE_CONDITION_PROPERTY	( ( FSRM_DISPID_FEATURE_REPORTS | 0xa00000 )  )

#define	FSRM_DISPID_FILE_MANAGEMENT_JOB_2	( ( FSRM_DISPID_FEATURE_REPORTS | 0xb00000 )  )



extern RPC_IF_HANDLE __MIDL_itf_fsrmreports_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrmreports_0000_0000_v0_0_s_ifspec;

#ifndef __IFsrmReportManager_INTERFACE_DEFINED__
#define __IFsrmReportManager_INTERFACE_DEFINED__

/* interface IFsrmReportManager */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmReportManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("27b899fe-6ffa-4481-a184-d3daade8a02b")
    IFsrmReportManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumReportJobs( 
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **reportJobs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateReportJob( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmReportJob **reportJob) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetReportJob( 
            /* [in] */ __RPC__in BSTR taskName,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmReportJob **reportJob) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetOutputDirectory( 
            /* [in] */ FsrmReportGenerationContext context,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetOutputDirectory( 
            /* [in] */ FsrmReportGenerationContext context,
            /* [in] */ __RPC__in BSTR path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsFilterValidForReportType( 
            /* [in] */ FsrmReportType reportType,
            /* [in] */ FsrmReportFilter filter,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *valid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultFilter( 
            /* [in] */ FsrmReportType reportType,
            /* [in] */ FsrmReportFilter filter,
            /* [retval][out] */ __RPC__out VARIANT *filterValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetDefaultFilter( 
            /* [in] */ FsrmReportType reportType,
            /* [in] */ FsrmReportFilter filter,
            /* [in] */ VARIANT filterValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetReportSizeLimit( 
            /* [in] */ FsrmReportLimit limit,
            /* [retval][out] */ __RPC__out VARIANT *limitValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetReportSizeLimit( 
            /* [in] */ FsrmReportLimit limit,
            /* [in] */ VARIANT limitValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmReportManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmReportManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmReportManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmReportManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmReportManager * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, EnumReportJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumReportJobs )( 
            __RPC__in IFsrmReportManager * This,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **reportJobs);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, CreateReportJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateReportJob )( 
            __RPC__in IFsrmReportManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmReportJob **reportJob);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, GetReportJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetReportJob )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ __RPC__in BSTR taskName,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmReportJob **reportJob);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, GetOutputDirectory)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetOutputDirectory )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportGenerationContext context,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, SetOutputDirectory)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetOutputDirectory )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportGenerationContext context,
            /* [in] */ __RPC__in BSTR path);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, IsFilterValidForReportType)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsFilterValidForReportType )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportType reportType,
            /* [in] */ FsrmReportFilter filter,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *valid);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, GetDefaultFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultFilter )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportType reportType,
            /* [in] */ FsrmReportFilter filter,
            /* [retval][out] */ __RPC__out VARIANT *filterValue);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, SetDefaultFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetDefaultFilter )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportType reportType,
            /* [in] */ FsrmReportFilter filter,
            /* [in] */ VARIANT filterValue);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, GetReportSizeLimit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetReportSizeLimit )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportLimit limit,
            /* [retval][out] */ __RPC__out VARIANT *limitValue);
        
        DECLSPEC_XFGVIRT(IFsrmReportManager, SetReportSizeLimit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetReportSizeLimit )( 
            __RPC__in IFsrmReportManager * This,
            /* [in] */ FsrmReportLimit limit,
            /* [in] */ VARIANT limitValue);
        
        END_INTERFACE
    } IFsrmReportManagerVtbl;

    interface IFsrmReportManager
    {
        CONST_VTBL struct IFsrmReportManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmReportManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmReportManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmReportManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmReportManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmReportManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmReportManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmReportManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmReportManager_EnumReportJobs(This,options,reportJobs)	\
    ( (This)->lpVtbl -> EnumReportJobs(This,options,reportJobs) ) 

#define IFsrmReportManager_CreateReportJob(This,reportJob)	\
    ( (This)->lpVtbl -> CreateReportJob(This,reportJob) ) 

#define IFsrmReportManager_GetReportJob(This,taskName,reportJob)	\
    ( (This)->lpVtbl -> GetReportJob(This,taskName,reportJob) ) 

#define IFsrmReportManager_GetOutputDirectory(This,context,path)	\
    ( (This)->lpVtbl -> GetOutputDirectory(This,context,path) ) 

#define IFsrmReportManager_SetOutputDirectory(This,context,path)	\
    ( (This)->lpVtbl -> SetOutputDirectory(This,context,path) ) 

#define IFsrmReportManager_IsFilterValidForReportType(This,reportType,filter,valid)	\
    ( (This)->lpVtbl -> IsFilterValidForReportType(This,reportType,filter,valid) ) 

#define IFsrmReportManager_GetDefaultFilter(This,reportType,filter,filterValue)	\
    ( (This)->lpVtbl -> GetDefaultFilter(This,reportType,filter,filterValue) ) 

#define IFsrmReportManager_SetDefaultFilter(This,reportType,filter,filterValue)	\
    ( (This)->lpVtbl -> SetDefaultFilter(This,reportType,filter,filterValue) ) 

#define IFsrmReportManager_GetReportSizeLimit(This,limit,limitValue)	\
    ( (This)->lpVtbl -> GetReportSizeLimit(This,limit,limitValue) ) 

#define IFsrmReportManager_SetReportSizeLimit(This,limit,limitValue)	\
    ( (This)->lpVtbl -> SetReportSizeLimit(This,limit,limitValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmReportManager_INTERFACE_DEFINED__ */


#ifndef __IFsrmReportJob_INTERFACE_DEFINED__
#define __IFsrmReportJob_INTERFACE_DEFINED__

/* interface IFsrmReportJob */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmReportJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("38e87280-715c-4c7d-a280-ea1651a19fef")
    IFsrmReportJob : public IFsrmObject
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Task( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *taskName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Task( 
            /* [in] */ __RPC__in BSTR taskName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NamespaceRoots( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *namespaceRoots) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_NamespaceRoots( 
            /* [in] */ __RPC__in SAFEARRAY * namespaceRoots) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Formats( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *formats) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Formats( 
            /* [in] */ __RPC__in SAFEARRAY * formats) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailTo( 
            /* [in] */ __RPC__in BSTR mailTo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RunningStatus( 
            /* [retval][out] */ __RPC__out FsrmReportRunningStatus *runningStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastRun( 
            /* [retval][out] */ __RPC__out DATE *lastRun) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastError( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *lastError) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastGeneratedInDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumReports( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **reports) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateReport( 
            /* [in] */ FsrmReportType reportType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmReport **report) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ FsrmReportGenerationContext context) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WaitForCompletion( 
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmReportJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmReportJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmReportJob * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmReportJob * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmReportJob * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmReportJob * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmReportJob * This);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_Task)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Task )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *taskName);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, put_Task)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Task )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in BSTR taskName);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_NamespaceRoots)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceRoots )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *namespaceRoots);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, put_NamespaceRoots)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NamespaceRoots )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in SAFEARRAY * namespaceRoots);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_Formats)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Formats )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *formats);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, put_Formats)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Formats )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in SAFEARRAY * formats);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_MailTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailTo )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, put_MailTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailTo )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ __RPC__in BSTR mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_RunningStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunningStatus )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__out FsrmReportRunningStatus *runningStatus);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_LastRun)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastRun )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__out DATE *lastRun);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_LastError)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastError )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *lastError);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, get_LastGeneratedInDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastGeneratedInDirectory )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, EnumReports)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumReports )( 
            __RPC__in IFsrmReportJob * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **reports);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, CreateReport)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateReport )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ FsrmReportType reportType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmReport **report);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, Run)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Run )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ FsrmReportGenerationContext context);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, WaitForCompletion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IFsrmReportJob * This,
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed);
        
        DECLSPEC_XFGVIRT(IFsrmReportJob, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFsrmReportJob * This);
        
        END_INTERFACE
    } IFsrmReportJobVtbl;

    interface IFsrmReportJob
    {
        CONST_VTBL struct IFsrmReportJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmReportJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmReportJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmReportJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmReportJob_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmReportJob_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmReportJob_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmReportJob_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmReportJob_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmReportJob_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmReportJob_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmReportJob_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmReportJob_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmReportJob_get_Task(This,taskName)	\
    ( (This)->lpVtbl -> get_Task(This,taskName) ) 

#define IFsrmReportJob_put_Task(This,taskName)	\
    ( (This)->lpVtbl -> put_Task(This,taskName) ) 

#define IFsrmReportJob_get_NamespaceRoots(This,namespaceRoots)	\
    ( (This)->lpVtbl -> get_NamespaceRoots(This,namespaceRoots) ) 

#define IFsrmReportJob_put_NamespaceRoots(This,namespaceRoots)	\
    ( (This)->lpVtbl -> put_NamespaceRoots(This,namespaceRoots) ) 

#define IFsrmReportJob_get_Formats(This,formats)	\
    ( (This)->lpVtbl -> get_Formats(This,formats) ) 

#define IFsrmReportJob_put_Formats(This,formats)	\
    ( (This)->lpVtbl -> put_Formats(This,formats) ) 

#define IFsrmReportJob_get_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> get_MailTo(This,mailTo) ) 

#define IFsrmReportJob_put_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> put_MailTo(This,mailTo) ) 

#define IFsrmReportJob_get_RunningStatus(This,runningStatus)	\
    ( (This)->lpVtbl -> get_RunningStatus(This,runningStatus) ) 

#define IFsrmReportJob_get_LastRun(This,lastRun)	\
    ( (This)->lpVtbl -> get_LastRun(This,lastRun) ) 

#define IFsrmReportJob_get_LastError(This,lastError)	\
    ( (This)->lpVtbl -> get_LastError(This,lastError) ) 

#define IFsrmReportJob_get_LastGeneratedInDirectory(This,path)	\
    ( (This)->lpVtbl -> get_LastGeneratedInDirectory(This,path) ) 

#define IFsrmReportJob_EnumReports(This,reports)	\
    ( (This)->lpVtbl -> EnumReports(This,reports) ) 

#define IFsrmReportJob_CreateReport(This,reportType,report)	\
    ( (This)->lpVtbl -> CreateReport(This,reportType,report) ) 

#define IFsrmReportJob_Run(This,context)	\
    ( (This)->lpVtbl -> Run(This,context) ) 

#define IFsrmReportJob_WaitForCompletion(This,waitSeconds,completed)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,waitSeconds,completed) ) 

#define IFsrmReportJob_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmReportJob_INTERFACE_DEFINED__ */


#ifndef __IFsrmReport_INTERFACE_DEFINED__
#define __IFsrmReport_INTERFACE_DEFINED__

/* interface IFsrmReport */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d8cc81d9-46b8-4fa4-bfa5-4aa9dec9b638")
    IFsrmReport : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out FsrmReportType *reportType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Description( 
            /* [in] */ __RPC__in BSTR description) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastGeneratedFileNamePrefix( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *prefix) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFilter( 
            /* [in] */ FsrmReportFilter filter,
            /* [retval][out] */ __RPC__out VARIANT *filterValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SetFilter( 
            /* [in] */ FsrmReportFilter filter,
            /* [in] */ VARIANT filterValue) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmReport * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmReport * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmReport * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmReport, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IFsrmReport * This,
            /* [retval][out] */ __RPC__out FsrmReportType *reportType);
        
        DECLSPEC_XFGVIRT(IFsrmReport, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsrmReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFsrmReport, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFsrmReport, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmReport, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmReport, get_LastGeneratedFileNamePrefix)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastGeneratedFileNamePrefix )( 
            __RPC__in IFsrmReport * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *prefix);
        
        DECLSPEC_XFGVIRT(IFsrmReport, GetFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFilter )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ FsrmReportFilter filter,
            /* [retval][out] */ __RPC__out VARIANT *filterValue);
        
        DECLSPEC_XFGVIRT(IFsrmReport, SetFilter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetFilter )( 
            __RPC__in IFsrmReport * This,
            /* [in] */ FsrmReportFilter filter,
            /* [in] */ VARIANT filterValue);
        
        DECLSPEC_XFGVIRT(IFsrmReport, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmReport * This);
        
        END_INTERFACE
    } IFsrmReportVtbl;

    interface IFsrmReport
    {
        CONST_VTBL struct IFsrmReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmReport_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmReport_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmReport_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmReport_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmReport_get_Type(This,reportType)	\
    ( (This)->lpVtbl -> get_Type(This,reportType) ) 

#define IFsrmReport_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFsrmReport_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IFsrmReport_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmReport_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmReport_get_LastGeneratedFileNamePrefix(This,prefix)	\
    ( (This)->lpVtbl -> get_LastGeneratedFileNamePrefix(This,prefix) ) 

#define IFsrmReport_GetFilter(This,filter,filterValue)	\
    ( (This)->lpVtbl -> GetFilter(This,filter,filterValue) ) 

#define IFsrmReport_SetFilter(This,filter,filterValue)	\
    ( (This)->lpVtbl -> SetFilter(This,filter,filterValue) ) 

#define IFsrmReport_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmReport_INTERFACE_DEFINED__ */


#ifndef __IFsrmReportScheduler_INTERFACE_DEFINED__
#define __IFsrmReportScheduler_INTERFACE_DEFINED__

/* interface IFsrmReportScheduler */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmReportScheduler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6879caf9-6617-4484-8719-71c3d8645f94")
    IFsrmReportScheduler : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE VerifyNamespaces( 
            /* [in] */ __RPC__in VARIANT *namespacesSafeArray) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateScheduleTask( 
            /* [in] */ __RPC__in BSTR taskName,
            /* [in] */ __RPC__in VARIANT *namespacesSafeArray,
            /* [in] */ __RPC__in BSTR serializedTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ModifyScheduleTask( 
            /* [in] */ __RPC__in BSTR taskName,
            /* [in] */ __RPC__in VARIANT *namespacesSafeArray,
            /* [in] */ __RPC__in BSTR serializedTask) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteScheduleTask( 
            /* [in] */ __RPC__in BSTR taskName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmReportSchedulerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmReportScheduler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmReportScheduler * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmReportScheduler * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmReportScheduler, VerifyNamespaces)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *VerifyNamespaces )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ __RPC__in VARIANT *namespacesSafeArray);
        
        DECLSPEC_XFGVIRT(IFsrmReportScheduler, CreateScheduleTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateScheduleTask )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ __RPC__in BSTR taskName,
            /* [in] */ __RPC__in VARIANT *namespacesSafeArray,
            /* [in] */ __RPC__in BSTR serializedTask);
        
        DECLSPEC_XFGVIRT(IFsrmReportScheduler, ModifyScheduleTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyScheduleTask )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ __RPC__in BSTR taskName,
            /* [in] */ __RPC__in VARIANT *namespacesSafeArray,
            /* [in] */ __RPC__in BSTR serializedTask);
        
        DECLSPEC_XFGVIRT(IFsrmReportScheduler, DeleteScheduleTask)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteScheduleTask )( 
            __RPC__in IFsrmReportScheduler * This,
            /* [in] */ __RPC__in BSTR taskName);
        
        END_INTERFACE
    } IFsrmReportSchedulerVtbl;

    interface IFsrmReportScheduler
    {
        CONST_VTBL struct IFsrmReportSchedulerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmReportScheduler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmReportScheduler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmReportScheduler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmReportScheduler_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmReportScheduler_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmReportScheduler_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmReportScheduler_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmReportScheduler_VerifyNamespaces(This,namespacesSafeArray)	\
    ( (This)->lpVtbl -> VerifyNamespaces(This,namespacesSafeArray) ) 

#define IFsrmReportScheduler_CreateScheduleTask(This,taskName,namespacesSafeArray,serializedTask)	\
    ( (This)->lpVtbl -> CreateScheduleTask(This,taskName,namespacesSafeArray,serializedTask) ) 

#define IFsrmReportScheduler_ModifyScheduleTask(This,taskName,namespacesSafeArray,serializedTask)	\
    ( (This)->lpVtbl -> ModifyScheduleTask(This,taskName,namespacesSafeArray,serializedTask) ) 

#define IFsrmReportScheduler_DeleteScheduleTask(This,taskName)	\
    ( (This)->lpVtbl -> DeleteScheduleTask(This,taskName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmReportScheduler_INTERFACE_DEFINED__ */


#ifndef __IFsrmFileManagementJobManager_INTERFACE_DEFINED__
#define __IFsrmFileManagementJobManager_INTERFACE_DEFINED__

/* interface IFsrmFileManagementJobManager */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmFileManagementJobManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ee321ecb-d95e-48e9-907c-c7685a013235")
    IFsrmFileManagementJobManager : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActionVariables( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *variables) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ActionVariableDescriptions( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *descriptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumFileManagementJobs( 
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **fileManagementJobs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateFileManagementJob( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmFileManagementJob **fileManagementJob) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFileManagementJob( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmFileManagementJob **fileManagementJob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmFileManagementJobManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmFileManagementJobManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmFileManagementJobManager * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmFileManagementJobManager * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJobManager, get_ActionVariables)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionVariables )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *variables);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJobManager, get_ActionVariableDescriptions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ActionVariableDescriptions )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *descriptions);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJobManager, EnumFileManagementJobs)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumFileManagementJobs )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [defaultvalue][in] */ FsrmEnumOptions options,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **fileManagementJobs);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJobManager, CreateFileManagementJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateFileManagementJob )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmFileManagementJob **fileManagementJob);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJobManager, GetFileManagementJob)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFileManagementJob )( 
            __RPC__in IFsrmFileManagementJobManager * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmFileManagementJob **fileManagementJob);
        
        END_INTERFACE
    } IFsrmFileManagementJobManagerVtbl;

    interface IFsrmFileManagementJobManager
    {
        CONST_VTBL struct IFsrmFileManagementJobManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmFileManagementJobManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmFileManagementJobManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmFileManagementJobManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmFileManagementJobManager_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmFileManagementJobManager_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmFileManagementJobManager_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmFileManagementJobManager_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmFileManagementJobManager_get_ActionVariables(This,variables)	\
    ( (This)->lpVtbl -> get_ActionVariables(This,variables) ) 

#define IFsrmFileManagementJobManager_get_ActionVariableDescriptions(This,descriptions)	\
    ( (This)->lpVtbl -> get_ActionVariableDescriptions(This,descriptions) ) 

#define IFsrmFileManagementJobManager_EnumFileManagementJobs(This,options,fileManagementJobs)	\
    ( (This)->lpVtbl -> EnumFileManagementJobs(This,options,fileManagementJobs) ) 

#define IFsrmFileManagementJobManager_CreateFileManagementJob(This,fileManagementJob)	\
    ( (This)->lpVtbl -> CreateFileManagementJob(This,fileManagementJob) ) 

#define IFsrmFileManagementJobManager_GetFileManagementJob(This,name,fileManagementJob)	\
    ( (This)->lpVtbl -> GetFileManagementJob(This,name,fileManagementJob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmFileManagementJobManager_INTERFACE_DEFINED__ */


#ifndef __IFsrmFileManagementJob_INTERFACE_DEFINED__
#define __IFsrmFileManagementJob_INTERFACE_DEFINED__

/* interface IFsrmFileManagementJob */
/* [object][oleautomation][dual][version][uuid] */ 

#define	FsrmDaysNotSpecified	( -1 )

#define	FsrmDateNotSpecified	( ( DATE  )-1 )


EXTERN_C const IID IID_IFsrmFileManagementJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0770687e-9f36-4d6f-8778-599d188461c9")
    IFsrmFileManagementJob : public IFsrmObject
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_NamespaceRoots( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *namespaceRoots) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_NamespaceRoots( 
            /* [in] */ __RPC__in SAFEARRAY * namespaceRoots) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL enabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_OperationType( 
            /* [retval][out] */ __RPC__out FsrmFileManagementType *operationType) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_OperationType( 
            /* [in] */ FsrmFileManagementType operationType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExpirationDirectory( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *expirationDirectory) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ExpirationDirectory( 
            /* [in] */ __RPC__in BSTR expirationDirectory) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CustomAction( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmActionCommand **action) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Notifications( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *notifications) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Logging( 
            /* [retval][out] */ __RPC__out long *loggingFlags) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Logging( 
            /* [in] */ long loggingFlags) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ReportEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *reportEnabled) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ReportEnabled( 
            /* [in] */ VARIANT_BOOL reportEnabled) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Formats( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *formats) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Formats( 
            /* [in] */ __RPC__in SAFEARRAY * formats) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MailTo( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MailTo( 
            /* [in] */ __RPC__in BSTR mailTo) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DaysSinceFileCreated( 
            /* [retval][out] */ __RPC__out long *daysSinceCreation) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DaysSinceFileCreated( 
            /* [in] */ long daysSinceCreation) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DaysSinceFileLastAccessed( 
            /* [retval][out] */ __RPC__out long *daysSinceAccess) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DaysSinceFileLastAccessed( 
            /* [in] */ long daysSinceAccess) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DaysSinceFileLastModified( 
            /* [retval][out] */ __RPC__out long *daysSinceModify) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DaysSinceFileLastModified( 
            /* [in] */ long daysSinceModify) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PropertyConditions( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **propertyConditions) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FromDate( 
            /* [retval][out] */ __RPC__out DATE *fromDate) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FromDate( 
            /* [in] */ DATE fromDate) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Task( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *taskName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Task( 
            /* [in] */ __RPC__in BSTR taskName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Parameters( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *parameters) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Parameters( 
            /* [in] */ __RPC__in SAFEARRAY * parameters) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RunningStatus( 
            /* [retval][out] */ __RPC__out FsrmReportRunningStatus *runningStatus) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastError( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *lastError) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastReportPathWithoutExtension( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LastRun( 
            /* [retval][out] */ __RPC__out DATE *lastRun) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FileNamePattern( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *fileNamePattern) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FileNamePattern( 
            /* [in] */ __RPC__in BSTR fileNamePattern) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Run( 
            /* [in] */ FsrmReportGenerationContext context) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE WaitForCompletion( 
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AddNotification( 
            /* [in] */ long days) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteNotification( 
            /* [in] */ long days) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ModifyNotification( 
            /* [in] */ long days,
            /* [in] */ long newDays) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateNotificationAction( 
            /* [in] */ long days,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE EnumNotificationActions( 
            /* [in] */ long days,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreatePropertyCondition( 
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmPropertyCondition **propertyCondition) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateCustomAction( 
            /* [retval][out] */ __RPC__deref_out_opt IFsrmActionCommand **customAction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmFileManagementJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmFileManagementJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmFileManagementJob * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmFileManagementJob * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Id)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out FSRM_OBJECT_ID *id);
        
        DECLSPEC_XFGVIRT(IFsrmObject, get_Description)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, put_Description)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Description )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR description);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmFileManagementJob * This);
        
        DECLSPEC_XFGVIRT(IFsrmObject, Commit)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IFsrmFileManagementJob * This);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_NamespaceRoots)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceRoots )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *namespaceRoots);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_NamespaceRoots)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_NamespaceRoots )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in SAFEARRAY * namespaceRoots);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Enabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *enabled);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_Enabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ VARIANT_BOOL enabled);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_OperationType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OperationType )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out FsrmFileManagementType *operationType);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_OperationType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_OperationType )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ FsrmFileManagementType operationType);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_ExpirationDirectory)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpirationDirectory )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *expirationDirectory);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_ExpirationDirectory)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ExpirationDirectory )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR expirationDirectory);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_CustomAction)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CustomAction )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmActionCommand **action);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Notifications)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Notifications )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *notifications);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Logging)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Logging )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out long *loggingFlags);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_Logging)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Logging )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long loggingFlags);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_ReportEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReportEnabled )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *reportEnabled);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_ReportEnabled)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ReportEnabled )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ VARIANT_BOOL reportEnabled);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Formats)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Formats )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *formats);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_Formats)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Formats )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in SAFEARRAY * formats);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_MailTo)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MailTo )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_MailTo)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MailTo )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR mailTo);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_DaysSinceFileCreated)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DaysSinceFileCreated )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out long *daysSinceCreation);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_DaysSinceFileCreated)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DaysSinceFileCreated )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long daysSinceCreation);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_DaysSinceFileLastAccessed)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DaysSinceFileLastAccessed )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out long *daysSinceAccess);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_DaysSinceFileLastAccessed)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DaysSinceFileLastAccessed )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long daysSinceAccess);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_DaysSinceFileLastModified)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DaysSinceFileLastModified )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out long *daysSinceModify);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_DaysSinceFileLastModified)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DaysSinceFileLastModified )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long daysSinceModify);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_PropertyConditions)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PropertyConditions )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **propertyConditions);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_FromDate)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FromDate )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out DATE *fromDate);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_FromDate)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FromDate )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ DATE fromDate);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Task)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Task )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *taskName);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_Task)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Task )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR taskName);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_Parameters)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parameters )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *parameters);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_Parameters)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Parameters )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in SAFEARRAY * parameters);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_RunningStatus)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RunningStatus )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out FsrmReportRunningStatus *runningStatus);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_LastError)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastError )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *lastError);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_LastReportPathWithoutExtension)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastReportPathWithoutExtension )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *path);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_LastRun)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastRun )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__out DATE *lastRun);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, get_FileNamePattern)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FileNamePattern )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *fileNamePattern);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, put_FileNamePattern)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FileNamePattern )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR fileNamePattern);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, Run)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Run )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ FsrmReportGenerationContext context);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, WaitForCompletion)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *WaitForCompletion )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long waitSeconds,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *completed);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, Cancel)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IFsrmFileManagementJob * This);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, AddNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddNotification )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long days);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, DeleteNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteNotification )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long days);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, ModifyNotification)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyNotification )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long days,
            /* [in] */ long newDays);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, CreateNotificationAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateNotificationAction )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long days,
            /* [in] */ FsrmActionType actionType,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmAction **action);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, EnumNotificationActions)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EnumNotificationActions )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ long days,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmCollection **actions);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, CreatePropertyCondition)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreatePropertyCondition )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [in] */ __RPC__in BSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmPropertyCondition **propertyCondition);
        
        DECLSPEC_XFGVIRT(IFsrmFileManagementJob, CreateCustomAction)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateCustomAction )( 
            __RPC__in IFsrmFileManagementJob * This,
            /* [retval][out] */ __RPC__deref_out_opt IFsrmActionCommand **customAction);
        
        END_INTERFACE
    } IFsrmFileManagementJobVtbl;

    interface IFsrmFileManagementJob
    {
        CONST_VTBL struct IFsrmFileManagementJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmFileManagementJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmFileManagementJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmFileManagementJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmFileManagementJob_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmFileManagementJob_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmFileManagementJob_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmFileManagementJob_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmFileManagementJob_get_Id(This,id)	\
    ( (This)->lpVtbl -> get_Id(This,id) ) 

#define IFsrmFileManagementJob_get_Description(This,description)	\
    ( (This)->lpVtbl -> get_Description(This,description) ) 

#define IFsrmFileManagementJob_put_Description(This,description)	\
    ( (This)->lpVtbl -> put_Description(This,description) ) 

#define IFsrmFileManagementJob_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#define IFsrmFileManagementJob_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 


#define IFsrmFileManagementJob_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFsrmFileManagementJob_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IFsrmFileManagementJob_get_NamespaceRoots(This,namespaceRoots)	\
    ( (This)->lpVtbl -> get_NamespaceRoots(This,namespaceRoots) ) 

#define IFsrmFileManagementJob_put_NamespaceRoots(This,namespaceRoots)	\
    ( (This)->lpVtbl -> put_NamespaceRoots(This,namespaceRoots) ) 

#define IFsrmFileManagementJob_get_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> get_Enabled(This,enabled) ) 

#define IFsrmFileManagementJob_put_Enabled(This,enabled)	\
    ( (This)->lpVtbl -> put_Enabled(This,enabled) ) 

#define IFsrmFileManagementJob_get_OperationType(This,operationType)	\
    ( (This)->lpVtbl -> get_OperationType(This,operationType) ) 

#define IFsrmFileManagementJob_put_OperationType(This,operationType)	\
    ( (This)->lpVtbl -> put_OperationType(This,operationType) ) 

#define IFsrmFileManagementJob_get_ExpirationDirectory(This,expirationDirectory)	\
    ( (This)->lpVtbl -> get_ExpirationDirectory(This,expirationDirectory) ) 

#define IFsrmFileManagementJob_put_ExpirationDirectory(This,expirationDirectory)	\
    ( (This)->lpVtbl -> put_ExpirationDirectory(This,expirationDirectory) ) 

#define IFsrmFileManagementJob_get_CustomAction(This,action)	\
    ( (This)->lpVtbl -> get_CustomAction(This,action) ) 

#define IFsrmFileManagementJob_get_Notifications(This,notifications)	\
    ( (This)->lpVtbl -> get_Notifications(This,notifications) ) 

#define IFsrmFileManagementJob_get_Logging(This,loggingFlags)	\
    ( (This)->lpVtbl -> get_Logging(This,loggingFlags) ) 

#define IFsrmFileManagementJob_put_Logging(This,loggingFlags)	\
    ( (This)->lpVtbl -> put_Logging(This,loggingFlags) ) 

#define IFsrmFileManagementJob_get_ReportEnabled(This,reportEnabled)	\
    ( (This)->lpVtbl -> get_ReportEnabled(This,reportEnabled) ) 

#define IFsrmFileManagementJob_put_ReportEnabled(This,reportEnabled)	\
    ( (This)->lpVtbl -> put_ReportEnabled(This,reportEnabled) ) 

#define IFsrmFileManagementJob_get_Formats(This,formats)	\
    ( (This)->lpVtbl -> get_Formats(This,formats) ) 

#define IFsrmFileManagementJob_put_Formats(This,formats)	\
    ( (This)->lpVtbl -> put_Formats(This,formats) ) 

#define IFsrmFileManagementJob_get_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> get_MailTo(This,mailTo) ) 

#define IFsrmFileManagementJob_put_MailTo(This,mailTo)	\
    ( (This)->lpVtbl -> put_MailTo(This,mailTo) ) 

#define IFsrmFileManagementJob_get_DaysSinceFileCreated(This,daysSinceCreation)	\
    ( (This)->lpVtbl -> get_DaysSinceFileCreated(This,daysSinceCreation) ) 

#define IFsrmFileManagementJob_put_DaysSinceFileCreated(This,daysSinceCreation)	\
    ( (This)->lpVtbl -> put_DaysSinceFileCreated(This,daysSinceCreation) ) 

#define IFsrmFileManagementJob_get_DaysSinceFileLastAccessed(This,daysSinceAccess)	\
    ( (This)->lpVtbl -> get_DaysSinceFileLastAccessed(This,daysSinceAccess) ) 

#define IFsrmFileManagementJob_put_DaysSinceFileLastAccessed(This,daysSinceAccess)	\
    ( (This)->lpVtbl -> put_DaysSinceFileLastAccessed(This,daysSinceAccess) ) 

#define IFsrmFileManagementJob_get_DaysSinceFileLastModified(This,daysSinceModify)	\
    ( (This)->lpVtbl -> get_DaysSinceFileLastModified(This,daysSinceModify) ) 

#define IFsrmFileManagementJob_put_DaysSinceFileLastModified(This,daysSinceModify)	\
    ( (This)->lpVtbl -> put_DaysSinceFileLastModified(This,daysSinceModify) ) 

#define IFsrmFileManagementJob_get_PropertyConditions(This,propertyConditions)	\
    ( (This)->lpVtbl -> get_PropertyConditions(This,propertyConditions) ) 

#define IFsrmFileManagementJob_get_FromDate(This,fromDate)	\
    ( (This)->lpVtbl -> get_FromDate(This,fromDate) ) 

#define IFsrmFileManagementJob_put_FromDate(This,fromDate)	\
    ( (This)->lpVtbl -> put_FromDate(This,fromDate) ) 

#define IFsrmFileManagementJob_get_Task(This,taskName)	\
    ( (This)->lpVtbl -> get_Task(This,taskName) ) 

#define IFsrmFileManagementJob_put_Task(This,taskName)	\
    ( (This)->lpVtbl -> put_Task(This,taskName) ) 

#define IFsrmFileManagementJob_get_Parameters(This,parameters)	\
    ( (This)->lpVtbl -> get_Parameters(This,parameters) ) 

#define IFsrmFileManagementJob_put_Parameters(This,parameters)	\
    ( (This)->lpVtbl -> put_Parameters(This,parameters) ) 

#define IFsrmFileManagementJob_get_RunningStatus(This,runningStatus)	\
    ( (This)->lpVtbl -> get_RunningStatus(This,runningStatus) ) 

#define IFsrmFileManagementJob_get_LastError(This,lastError)	\
    ( (This)->lpVtbl -> get_LastError(This,lastError) ) 

#define IFsrmFileManagementJob_get_LastReportPathWithoutExtension(This,path)	\
    ( (This)->lpVtbl -> get_LastReportPathWithoutExtension(This,path) ) 

#define IFsrmFileManagementJob_get_LastRun(This,lastRun)	\
    ( (This)->lpVtbl -> get_LastRun(This,lastRun) ) 

#define IFsrmFileManagementJob_get_FileNamePattern(This,fileNamePattern)	\
    ( (This)->lpVtbl -> get_FileNamePattern(This,fileNamePattern) ) 

#define IFsrmFileManagementJob_put_FileNamePattern(This,fileNamePattern)	\
    ( (This)->lpVtbl -> put_FileNamePattern(This,fileNamePattern) ) 

#define IFsrmFileManagementJob_Run(This,context)	\
    ( (This)->lpVtbl -> Run(This,context) ) 

#define IFsrmFileManagementJob_WaitForCompletion(This,waitSeconds,completed)	\
    ( (This)->lpVtbl -> WaitForCompletion(This,waitSeconds,completed) ) 

#define IFsrmFileManagementJob_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IFsrmFileManagementJob_AddNotification(This,days)	\
    ( (This)->lpVtbl -> AddNotification(This,days) ) 

#define IFsrmFileManagementJob_DeleteNotification(This,days)	\
    ( (This)->lpVtbl -> DeleteNotification(This,days) ) 

#define IFsrmFileManagementJob_ModifyNotification(This,days,newDays)	\
    ( (This)->lpVtbl -> ModifyNotification(This,days,newDays) ) 

#define IFsrmFileManagementJob_CreateNotificationAction(This,days,actionType,action)	\
    ( (This)->lpVtbl -> CreateNotificationAction(This,days,actionType,action) ) 

#define IFsrmFileManagementJob_EnumNotificationActions(This,days,actions)	\
    ( (This)->lpVtbl -> EnumNotificationActions(This,days,actions) ) 

#define IFsrmFileManagementJob_CreatePropertyCondition(This,name,propertyCondition)	\
    ( (This)->lpVtbl -> CreatePropertyCondition(This,name,propertyCondition) ) 

#define IFsrmFileManagementJob_CreateCustomAction(This,customAction)	\
    ( (This)->lpVtbl -> CreateCustomAction(This,customAction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmFileManagementJob_INTERFACE_DEFINED__ */


#ifndef __IFsrmPropertyCondition_INTERFACE_DEFINED__
#define __IFsrmPropertyCondition_INTERFACE_DEFINED__

/* interface IFsrmPropertyCondition */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmPropertyCondition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("326af66f-2ac0-4f68-bf8c-4759f054fa29")
    IFsrmPropertyCondition : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Name( 
            /* [in] */ __RPC__in BSTR name) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out FsrmPropertyConditionType *type) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Type( 
            /* [in] */ FsrmPropertyConditionType type) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *value) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ __RPC__in BSTR value) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmPropertyConditionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmPropertyCondition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmPropertyCondition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmPropertyCondition * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *name);
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, put_Name)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Name )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [in] */ __RPC__in BSTR name);
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [retval][out] */ __RPC__out FsrmPropertyConditionType *type);
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, put_Type)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Type )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [in] */ FsrmPropertyConditionType type);
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *value);
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, put_Value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IFsrmPropertyCondition * This,
            /* [in] */ __RPC__in BSTR value);
        
        DECLSPEC_XFGVIRT(IFsrmPropertyCondition, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmPropertyCondition * This);
        
        END_INTERFACE
    } IFsrmPropertyConditionVtbl;

    interface IFsrmPropertyCondition
    {
        CONST_VTBL struct IFsrmPropertyConditionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmPropertyCondition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmPropertyCondition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmPropertyCondition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmPropertyCondition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmPropertyCondition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmPropertyCondition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmPropertyCondition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmPropertyCondition_get_Name(This,name)	\
    ( (This)->lpVtbl -> get_Name(This,name) ) 

#define IFsrmPropertyCondition_put_Name(This,name)	\
    ( (This)->lpVtbl -> put_Name(This,name) ) 

#define IFsrmPropertyCondition_get_Type(This,type)	\
    ( (This)->lpVtbl -> get_Type(This,type) ) 

#define IFsrmPropertyCondition_put_Type(This,type)	\
    ( (This)->lpVtbl -> put_Type(This,type) ) 

#define IFsrmPropertyCondition_get_Value(This,value)	\
    ( (This)->lpVtbl -> get_Value(This,value) ) 

#define IFsrmPropertyCondition_put_Value(This,value)	\
    ( (This)->lpVtbl -> put_Value(This,value) ) 

#define IFsrmPropertyCondition_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmPropertyCondition_INTERFACE_DEFINED__ */


#ifndef __IFsrmFileCondition_INTERFACE_DEFINED__
#define __IFsrmFileCondition_INTERFACE_DEFINED__

/* interface IFsrmFileCondition */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmFileCondition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70684FFC-691A-4A1A-B922-97752E138CC1")
    IFsrmFileCondition : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Type( 
            /* [retval][out] */ __RPC__out FsrmFileConditionType *pVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Delete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmFileConditionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmFileCondition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmFileCondition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmFileCondition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmFileCondition * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmFileCondition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmFileCondition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmFileCondition * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmFileCondition, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IFsrmFileCondition * This,
            /* [retval][out] */ __RPC__out FsrmFileConditionType *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileCondition, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmFileCondition * This);
        
        END_INTERFACE
    } IFsrmFileConditionVtbl;

    interface IFsrmFileCondition
    {
        CONST_VTBL struct IFsrmFileConditionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmFileCondition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmFileCondition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmFileCondition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmFileCondition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmFileCondition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmFileCondition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmFileCondition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmFileCondition_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 

#define IFsrmFileCondition_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmFileCondition_INTERFACE_DEFINED__ */


#ifndef __IFsrmFileConditionProperty_INTERFACE_DEFINED__
#define __IFsrmFileConditionProperty_INTERFACE_DEFINED__

/* interface IFsrmFileConditionProperty */
/* [object][oleautomation][dual][version][uuid] */ 


EXTERN_C const IID IID_IFsrmFileConditionProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("81926775-B981-4479-988F-DA171D627360")
    IFsrmFileConditionProperty : public IFsrmFileCondition
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PropertyName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PropertyName( 
            /* [in] */ __RPC__in BSTR newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PropertyId( 
            /* [retval][out] */ __RPC__out FsrmFileSystemPropertyId *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PropertyId( 
            /* [in] */ FsrmFileSystemPropertyId newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Operator( 
            /* [retval][out] */ __RPC__out FsrmPropertyConditionType *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Operator( 
            /* [in] */ FsrmPropertyConditionType newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ValueType( 
            /* [retval][out] */ __RPC__out FsrmPropertyValueType *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_ValueType( 
            /* [in] */ FsrmPropertyValueType newVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ VARIANT newVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFsrmFileConditionPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IFsrmFileConditionProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IFsrmFileConditionProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IFsrmFileConditionProperty * This,
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
        
        DECLSPEC_XFGVIRT(IFsrmFileCondition, get_Type)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Type )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [retval][out] */ __RPC__out FsrmFileConditionType *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileCondition, Delete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IFsrmFileConditionProperty * This);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, get_PropertyName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PropertyName )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, put_PropertyName)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PropertyName )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ __RPC__in BSTR newVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, get_PropertyId)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PropertyId )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [retval][out] */ __RPC__out FsrmFileSystemPropertyId *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, put_PropertyId)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PropertyId )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ FsrmFileSystemPropertyId newVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, get_Operator)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Operator )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [retval][out] */ __RPC__out FsrmPropertyConditionType *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, put_Operator)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Operator )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ FsrmPropertyConditionType newVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, get_ValueType)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ValueType )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [retval][out] */ __RPC__out FsrmPropertyValueType *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, put_ValueType)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_ValueType )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ FsrmPropertyValueType newVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *pVal);
        
        DECLSPEC_XFGVIRT(IFsrmFileConditionProperty, put_Value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IFsrmFileConditionProperty * This,
            /* [in] */ VARIANT newVal);
        
        END_INTERFACE
    } IFsrmFileConditionPropertyVtbl;

    interface IFsrmFileConditionProperty
    {
        CONST_VTBL struct IFsrmFileConditionPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFsrmFileConditionProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFsrmFileConditionProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFsrmFileConditionProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFsrmFileConditionProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IFsrmFileConditionProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IFsrmFileConditionProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IFsrmFileConditionProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IFsrmFileConditionProperty_get_Type(This,pVal)	\
    ( (This)->lpVtbl -> get_Type(This,pVal) ) 

#define IFsrmFileConditionProperty_Delete(This)	\
    ( (This)->lpVtbl -> Delete(This) ) 


#define IFsrmFileConditionProperty_get_PropertyName(This,pVal)	\
    ( (This)->lpVtbl -> get_PropertyName(This,pVal) ) 

#define IFsrmFileConditionProperty_put_PropertyName(This,newVal)	\
    ( (This)->lpVtbl -> put_PropertyName(This,newVal) ) 

#define IFsrmFileConditionProperty_get_PropertyId(This,pVal)	\
    ( (This)->lpVtbl -> get_PropertyId(This,pVal) ) 

#define IFsrmFileConditionProperty_put_PropertyId(This,newVal)	\
    ( (This)->lpVtbl -> put_PropertyId(This,newVal) ) 

#define IFsrmFileConditionProperty_get_Operator(This,pVal)	\
    ( (This)->lpVtbl -> get_Operator(This,pVal) ) 

#define IFsrmFileConditionProperty_put_Operator(This,newVal)	\
    ( (This)->lpVtbl -> put_Operator(This,newVal) ) 

#define IFsrmFileConditionProperty_get_ValueType(This,pVal)	\
    ( (This)->lpVtbl -> get_ValueType(This,pVal) ) 

#define IFsrmFileConditionProperty_put_ValueType(This,newVal)	\
    ( (This)->lpVtbl -> put_ValueType(This,newVal) ) 

#define IFsrmFileConditionProperty_get_Value(This,pVal)	\
    ( (This)->lpVtbl -> get_Value(This,pVal) ) 

#define IFsrmFileConditionProperty_put_Value(This,newVal)	\
    ( (This)->lpVtbl -> put_Value(This,newVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFsrmFileConditionProperty_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_fsrmreports_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_fsrmreports_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_fsrmreports_0000_0009_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


