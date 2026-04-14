

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

#ifndef __comadmin_h__
#define __comadmin_h__

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

#ifndef __ICOMAdminCatalog_FWD_DEFINED__
#define __ICOMAdminCatalog_FWD_DEFINED__
typedef interface ICOMAdminCatalog ICOMAdminCatalog;

#endif 	/* __ICOMAdminCatalog_FWD_DEFINED__ */


#ifndef __ICOMAdminCatalog2_FWD_DEFINED__
#define __ICOMAdminCatalog2_FWD_DEFINED__
typedef interface ICOMAdminCatalog2 ICOMAdminCatalog2;

#endif 	/* __ICOMAdminCatalog2_FWD_DEFINED__ */


#ifndef __ICatalogObject_FWD_DEFINED__
#define __ICatalogObject_FWD_DEFINED__
typedef interface ICatalogObject ICatalogObject;

#endif 	/* __ICatalogObject_FWD_DEFINED__ */


#ifndef __ICatalogCollection_FWD_DEFINED__
#define __ICatalogCollection_FWD_DEFINED__
typedef interface ICatalogCollection ICatalogCollection;

#endif 	/* __ICatalogCollection_FWD_DEFINED__ */


#ifndef __COMAdminCatalog_FWD_DEFINED__
#define __COMAdminCatalog_FWD_DEFINED__

#ifdef __cplusplus
typedef class COMAdminCatalog COMAdminCatalog;
#else
typedef struct COMAdminCatalog COMAdminCatalog;
#endif /* __cplusplus */

#endif 	/* __COMAdminCatalog_FWD_DEFINED__ */


#ifndef __COMAdminCatalogObject_FWD_DEFINED__
#define __COMAdminCatalogObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class COMAdminCatalogObject COMAdminCatalogObject;
#else
typedef struct COMAdminCatalogObject COMAdminCatalogObject;
#endif /* __cplusplus */

#endif 	/* __COMAdminCatalogObject_FWD_DEFINED__ */


#ifndef __COMAdminCatalogCollection_FWD_DEFINED__
#define __COMAdminCatalogCollection_FWD_DEFINED__

#ifdef __cplusplus
typedef class COMAdminCatalogCollection COMAdminCatalogCollection;
#else
typedef struct COMAdminCatalogCollection COMAdminCatalogCollection;
#endif /* __cplusplus */

#endif 	/* __COMAdminCatalogCollection_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_comadmin_0000_0000 */
/* [local] */ 

// -----------------------------------------------------------------------   
// comadmin.h  -- COM Administration Programming Interfaces                  
//                                                                           
// This file provides the prototypes for the APIs and COM interfaces         
// used by Microsoft COM applications.                                       
//                                                                           
// Copyright (c) 1995-2001 Microsoft Corporation.  All Rights Reserved.      
// -----------------------------------------------------------------------   
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <objbase.h>
#ifndef DECLSPEC_UUID
#if (_MSC_VER >= 1100) && defined (__cplusplus)
#define DECLSPEC_UUID(x)    __declspec(uuid(x))
#else
#define DECLSPEC_UUID(x)
#endif
#endif
#pragma once
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma once
#pragma region Input Buffer SAL 1 compatibility macros
#pragma endregion Input Buffer SAL 1 compatibility macros
#pragma once
#pragma once
#pragma warning(pop)


extern RPC_IF_HANDLE __MIDL_itf_comadmin_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comadmin_0000_0000_v0_0_s_ifspec;

#ifndef __ICOMAdminCatalog_INTERFACE_DEFINED__
#define __ICOMAdminCatalog_INTERFACE_DEFINED__

/* interface ICOMAdminCatalog */
/* [unique][helpstring][dual][uuid][object] */ 




EXTERN_C const IID IID_ICOMAdminCatalog;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DD662187-DFC2-11d1-A2CF-00805FC79235")
    ICOMAdminCatalog : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCollection( 
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ __RPC__in BSTR bstrCatalogServerName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MajorVersion( 
            /* [retval][out] */ __RPC__out long *plMajorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MinorVersion( 
            /* [retval][out] */ __RPC__out long *plMinorVersion) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCollectionByQuery( 
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarQuery,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportComponent( 
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallComponent( 
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrDLL,
            /* [in] */ __RPC__in BSTR bstrTLB,
            /* [in] */ __RPC__in BSTR bstrPSDLL) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ShutdownApplication( 
            /* [in] */ __RPC__in BSTR bstrApplIDOrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportApplication( 
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [in] */ long lOptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [optional][in] */ __RPC__in BSTR bstrDestinationDirectory,
            /* [optional][in] */ long lOptions,
            /* [optional][in] */ __RPC__in BSTR bstrUserId,
            /* [optional][in] */ __RPC__in BSTR bstrPassword,
            /* [optional][in] */ __RPC__in BSTR bstrRSN) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StopRouter( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RefreshRouter( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartRouter( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reserved1( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Reserved2( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallMultipleComponents( 
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarCLSIDs) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetMultipleComponentsInfo( 
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarCLSIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarClassNames,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarFileFlags,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarComponentFlags) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RefreshComponents( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE BackupREGDB( 
            /* [in] */ __RPC__in BSTR bstrBackupFilePath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RestoreREGDB( 
            /* [in] */ __RPC__in BSTR bstrBackupFilePath) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryApplicationFile( 
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrApplicationName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrApplicationDescription,
            /* [out] */ __RPC__out VARIANT_BOOL *pbHasUsers,
            /* [out] */ __RPC__out VARIANT_BOOL *pbIsProxy,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarFileNames) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE StartApplication( 
            /* [in] */ __RPC__in BSTR bstrApplIdOrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ServiceCheck( 
            /* [in] */ long lService,
            /* [retval][out] */ __RPC__out long *plStatus) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallMultipleEventClasses( 
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarCLSIDS) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallEventClass( 
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__in BSTR bstrDLL,
            /* [in] */ __RPC__in BSTR bstrTLB,
            /* [in] */ __RPC__in BSTR bstrPSDLL) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEventClassesForIID( 
            /* [in] */ __RPC__in BSTR bstrIID,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarCLSIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarProgIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarDescriptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICOMAdminCatalogVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICOMAdminCatalog * This,
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
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCollection )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrCatalogServerName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [retval][out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetCollectionByQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCollectionByQuery )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarQuery,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ImportComponent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportComponent )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallComponent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallComponent )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrDLL,
            /* [in] */ __RPC__in BSTR bstrTLB,
            /* [in] */ __RPC__in BSTR bstrPSDLL);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ShutdownApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ShutdownApplication )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ExportApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportApplication )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [in] */ long lOptions);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallApplication )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [optional][in] */ __RPC__in BSTR bstrDestinationDirectory,
            /* [optional][in] */ long lOptions,
            /* [optional][in] */ __RPC__in BSTR bstrUserId,
            /* [optional][in] */ __RPC__in BSTR bstrPassword,
            /* [optional][in] */ __RPC__in BSTR bstrRSN);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, StopRouter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopRouter )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, RefreshRouter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshRouter )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, StartRouter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartRouter )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, Reserved1)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reserved1 )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, Reserved2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reserved2 )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallMultipleComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallMultipleComponents )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarCLSIDs);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetMultipleComponentsInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMultipleComponentsInfo )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarCLSIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarClassNames,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarFileFlags,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarComponentFlags);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, RefreshComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshComponents )( 
            __RPC__in ICOMAdminCatalog * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, BackupREGDB)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BackupREGDB )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrBackupFilePath);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, RestoreREGDB)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreREGDB )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrBackupFilePath);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, QueryApplicationFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryApplicationFile )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrApplicationName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrApplicationDescription,
            /* [out] */ __RPC__out VARIANT_BOOL *pbHasUsers,
            /* [out] */ __RPC__out VARIANT_BOOL *pbIsProxy,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarFileNames);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, StartApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartApplication )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ServiceCheck)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ServiceCheck )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ long lService,
            /* [retval][out] */ __RPC__out long *plStatus);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallMultipleEventClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallMultipleEventClasses )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarCLSIDS);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallEventClass)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallEventClass )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__in BSTR bstrDLL,
            /* [in] */ __RPC__in BSTR bstrTLB,
            /* [in] */ __RPC__in BSTR bstrPSDLL);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetEventClassesForIID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEventClassesForIID )( 
            __RPC__in ICOMAdminCatalog * This,
            /* [in] */ __RPC__in BSTR bstrIID,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarCLSIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarProgIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarDescriptions);
        
        END_INTERFACE
    } ICOMAdminCatalogVtbl;

    interface ICOMAdminCatalog
    {
        CONST_VTBL struct ICOMAdminCatalogVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICOMAdminCatalog_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICOMAdminCatalog_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICOMAdminCatalog_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICOMAdminCatalog_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICOMAdminCatalog_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICOMAdminCatalog_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICOMAdminCatalog_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICOMAdminCatalog_GetCollection(This,bstrCollName,ppCatalogCollection)	\
    ( (This)->lpVtbl -> GetCollection(This,bstrCollName,ppCatalogCollection) ) 

#define ICOMAdminCatalog_Connect(This,bstrCatalogServerName,ppCatalogCollection)	\
    ( (This)->lpVtbl -> Connect(This,bstrCatalogServerName,ppCatalogCollection) ) 

#define ICOMAdminCatalog_get_MajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,plMajorVersion) ) 

#define ICOMAdminCatalog_get_MinorVersion(This,plMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,plMinorVersion) ) 

#define ICOMAdminCatalog_GetCollectionByQuery(This,bstrCollName,ppsaVarQuery,ppCatalogCollection)	\
    ( (This)->lpVtbl -> GetCollectionByQuery(This,bstrCollName,ppsaVarQuery,ppCatalogCollection) ) 

#define ICOMAdminCatalog_ImportComponent(This,bstrApplIDOrName,bstrCLSIDOrProgID)	\
    ( (This)->lpVtbl -> ImportComponent(This,bstrApplIDOrName,bstrCLSIDOrProgID) ) 

#define ICOMAdminCatalog_InstallComponent(This,bstrApplIDOrName,bstrDLL,bstrTLB,bstrPSDLL)	\
    ( (This)->lpVtbl -> InstallComponent(This,bstrApplIDOrName,bstrDLL,bstrTLB,bstrPSDLL) ) 

#define ICOMAdminCatalog_ShutdownApplication(This,bstrApplIDOrName)	\
    ( (This)->lpVtbl -> ShutdownApplication(This,bstrApplIDOrName) ) 

#define ICOMAdminCatalog_ExportApplication(This,bstrApplIDOrName,bstrApplicationFile,lOptions)	\
    ( (This)->lpVtbl -> ExportApplication(This,bstrApplIDOrName,bstrApplicationFile,lOptions) ) 

#define ICOMAdminCatalog_InstallApplication(This,bstrApplicationFile,bstrDestinationDirectory,lOptions,bstrUserId,bstrPassword,bstrRSN)	\
    ( (This)->lpVtbl -> InstallApplication(This,bstrApplicationFile,bstrDestinationDirectory,lOptions,bstrUserId,bstrPassword,bstrRSN) ) 

#define ICOMAdminCatalog_StopRouter(This)	\
    ( (This)->lpVtbl -> StopRouter(This) ) 

#define ICOMAdminCatalog_RefreshRouter(This)	\
    ( (This)->lpVtbl -> RefreshRouter(This) ) 

#define ICOMAdminCatalog_StartRouter(This)	\
    ( (This)->lpVtbl -> StartRouter(This) ) 

#define ICOMAdminCatalog_Reserved1(This)	\
    ( (This)->lpVtbl -> Reserved1(This) ) 

#define ICOMAdminCatalog_Reserved2(This)	\
    ( (This)->lpVtbl -> Reserved2(This) ) 

#define ICOMAdminCatalog_InstallMultipleComponents(This,bstrApplIDOrName,ppsaVarFileNames,ppsaVarCLSIDs)	\
    ( (This)->lpVtbl -> InstallMultipleComponents(This,bstrApplIDOrName,ppsaVarFileNames,ppsaVarCLSIDs) ) 

#define ICOMAdminCatalog_GetMultipleComponentsInfo(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDs,ppsaVarClassNames,ppsaVarFileFlags,ppsaVarComponentFlags)	\
    ( (This)->lpVtbl -> GetMultipleComponentsInfo(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDs,ppsaVarClassNames,ppsaVarFileFlags,ppsaVarComponentFlags) ) 

#define ICOMAdminCatalog_RefreshComponents(This)	\
    ( (This)->lpVtbl -> RefreshComponents(This) ) 

#define ICOMAdminCatalog_BackupREGDB(This,bstrBackupFilePath)	\
    ( (This)->lpVtbl -> BackupREGDB(This,bstrBackupFilePath) ) 

#define ICOMAdminCatalog_RestoreREGDB(This,bstrBackupFilePath)	\
    ( (This)->lpVtbl -> RestoreREGDB(This,bstrBackupFilePath) ) 

#define ICOMAdminCatalog_QueryApplicationFile(This,bstrApplicationFile,pbstrApplicationName,pbstrApplicationDescription,pbHasUsers,pbIsProxy,ppsaVarFileNames)	\
    ( (This)->lpVtbl -> QueryApplicationFile(This,bstrApplicationFile,pbstrApplicationName,pbstrApplicationDescription,pbHasUsers,pbIsProxy,ppsaVarFileNames) ) 

#define ICOMAdminCatalog_StartApplication(This,bstrApplIdOrName)	\
    ( (This)->lpVtbl -> StartApplication(This,bstrApplIdOrName) ) 

#define ICOMAdminCatalog_ServiceCheck(This,lService,plStatus)	\
    ( (This)->lpVtbl -> ServiceCheck(This,lService,plStatus) ) 

#define ICOMAdminCatalog_InstallMultipleEventClasses(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDS)	\
    ( (This)->lpVtbl -> InstallMultipleEventClasses(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDS) ) 

#define ICOMAdminCatalog_InstallEventClass(This,bstrApplIdOrName,bstrDLL,bstrTLB,bstrPSDLL)	\
    ( (This)->lpVtbl -> InstallEventClass(This,bstrApplIdOrName,bstrDLL,bstrTLB,bstrPSDLL) ) 

#define ICOMAdminCatalog_GetEventClassesForIID(This,bstrIID,ppsaVarCLSIDs,ppsaVarProgIDs,ppsaVarDescriptions)	\
    ( (This)->lpVtbl -> GetEventClassesForIID(This,bstrIID,ppsaVarCLSIDs,ppsaVarProgIDs,ppsaVarDescriptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICOMAdminCatalog_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_comadmin_0000_0001 */
/* [local] */ 

typedef /* [helpstring] */ 
enum COMAdminInUse
    {
        COMAdminNotInUse	= 0,
        COMAdminInUseByCatalog	= 0x1,
        COMAdminInUseByRegistryUnknown	= 0x2,
        COMAdminInUseByRegistryProxyStub	= 0x3,
        COMAdminInUseByRegistryTypeLib	= 0x4,
        COMAdminInUseByRegistryClsid	= 0x5
    } 	COMAdminInUse;



extern RPC_IF_HANDLE __MIDL_itf_comadmin_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comadmin_0000_0001_v0_0_s_ifspec;

#ifndef __ICOMAdminCatalog2_INTERFACE_DEFINED__
#define __ICOMAdminCatalog2_INTERFACE_DEFINED__

/* interface ICOMAdminCatalog2 */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICOMAdminCatalog2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("790C6E0B-9194-4cc9-9426-A48A63185696")
    ICOMAdminCatalog2 : public ICOMAdminCatalog
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCollectionByQuery2( 
            /* [in] */ __RPC__in BSTR bstrCollectionName,
            /* [in] */ __RPC__in VARIANT *pVarQueryStrings,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetApplicationInstanceIDFromProcessID( 
            /* [in] */ long lProcessID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationInstanceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ShutdownApplicationInstances( 
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PauseApplicationInstances( 
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ResumeApplicationInstances( 
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE RecycleApplicationInstances( 
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID,
            /* [in] */ long lReasonCode) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AreApplicationInstancesPaused( 
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBoolPaused) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DumpApplicationInstance( 
            /* [in] */ __RPC__in BSTR bstrApplicationInstanceID,
            /* [in] */ __RPC__in BSTR bstrDirectory,
            /* [in] */ long lMaxImages,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDumpFile) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsApplicationInstanceDumpSupported( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBoolDumpSupported) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateServiceForApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in BSTR bstrServiceName,
            /* [in] */ __RPC__in BSTR bstrStartType,
            /* [in] */ __RPC__in BSTR bstrErrorControl,
            /* [in] */ __RPC__in BSTR bstrDependencies,
            /* [in] */ __RPC__in BSTR bstrRunAs,
            /* [in] */ __RPC__in BSTR bstrPassword,
            /* [in] */ VARIANT_BOOL bDesktopOk) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteServiceForApplication( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPartitionID( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPartitionName( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionName) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CurrentPartition( 
            /* [in] */ __RPC__in BSTR bstrPartitionIDOrName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPartitionID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionID) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CurrentPartitionName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionName) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_GlobalPartitionID( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGlobalPartitionID) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE FlushPartitionCache( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyApplications( 
            /* [in] */ __RPC__in BSTR bstrSourcePartitionIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarApplicationID,
            /* [in] */ __RPC__in BSTR bstrDestinationPartitionIDOrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE CopyComponents( 
            /* [in] */ __RPC__in BSTR bstrSourceApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [in] */ __RPC__in BSTR bstrDestinationApplicationIDOrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE MoveComponents( 
            /* [in] */ __RPC__in BSTR bstrSourceApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [in] */ __RPC__in BSTR bstrDestinationApplicationIDOrName) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE AliasComponent( 
            /* [in] */ __RPC__in BSTR bstrSrcApplicationIDOrName,
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID,
            /* [in] */ __RPC__in BSTR bstrDestApplicationIDOrName,
            /* [in] */ __RPC__in BSTR bstrNewProgId,
            /* [in] */ __RPC__in BSTR bstrNewClsid) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsSafeToDelete( 
            /* [in] */ __RPC__in BSTR bstrDllName,
            /* [retval][out] */ __RPC__out COMAdminInUse *pCOMAdminInUse) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportUnconfiguredComponents( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [optional][in] */ __RPC__in VARIANT *pVarComponentType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PromoteUnconfiguredComponents( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [optional][in] */ __RPC__in VARIANT *pVarComponentType) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ImportComponents( 
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [optional][in] */ __RPC__in VARIANT *pVarComponentType) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Is64BitCatalogServer( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIs64Bit) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE ExportPartition( 
            /* [in] */ __RPC__in BSTR bstrPartitionIDOrName,
            /* [in] */ __RPC__in BSTR bstrPartitionFileName,
            /* [in] */ long lOptions) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE InstallPartition( 
            /* [in] */ __RPC__in BSTR bstrFileName,
            /* [in] */ __RPC__in BSTR bstrDestDirectory,
            /* [in] */ long lOptions,
            /* [in] */ __RPC__in BSTR bstrUserID,
            /* [in] */ __RPC__in BSTR bstrPassword,
            /* [in] */ __RPC__in BSTR bstrRSN) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE QueryApplicationFile2( 
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppFilesForImport) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetComponentVersionCount( 
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID,
            /* [retval][out] */ __RPC__out long *plVersionCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICOMAdminCatalog2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICOMAdminCatalog2 * This,
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
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCollection )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, Connect)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrCatalogServerName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, get_MajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MajorVersion )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, get_MinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MinorVersion )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__out long *plMinorVersion);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetCollectionByQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCollectionByQuery )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarQuery,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ImportComponent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportComponent )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallComponent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallComponent )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrDLL,
            /* [in] */ __RPC__in BSTR bstrTLB,
            /* [in] */ __RPC__in BSTR bstrPSDLL);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ShutdownApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ShutdownApplication )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ExportApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportApplication )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [in] */ long lOptions);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallApplication )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [optional][in] */ __RPC__in BSTR bstrDestinationDirectory,
            /* [optional][in] */ long lOptions,
            /* [optional][in] */ __RPC__in BSTR bstrUserId,
            /* [optional][in] */ __RPC__in BSTR bstrPassword,
            /* [optional][in] */ __RPC__in BSTR bstrRSN);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, StopRouter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopRouter )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, RefreshRouter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshRouter )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, StartRouter)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartRouter )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, Reserved1)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reserved1 )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, Reserved2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reserved2 )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallMultipleComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallMultipleComponents )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIDOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarCLSIDs);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetMultipleComponentsInfo)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetMultipleComponentsInfo )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarCLSIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarClassNames,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarFileFlags,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarComponentFlags);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, RefreshComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RefreshComponents )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, BackupREGDB)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BackupREGDB )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrBackupFilePath);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, RestoreREGDB)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RestoreREGDB )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrBackupFilePath);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, QueryApplicationFile)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryApplicationFile )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrApplicationName,
            /* [out] */ __RPC__deref_out_opt BSTR *pbstrApplicationDescription,
            /* [out] */ __RPC__out VARIANT_BOOL *pbHasUsers,
            /* [out] */ __RPC__out VARIANT_BOOL *pbIsProxy,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarFileNames);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, StartApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StartApplication )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, ServiceCheck)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ServiceCheck )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ long lService,
            /* [retval][out] */ __RPC__out long *plStatus);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallMultipleEventClasses)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallMultipleEventClasses )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarFileNames,
            /* [in] */ __RPC__deref_in_opt SAFEARRAY * *ppsaVarCLSIDS);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, InstallEventClass)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallEventClass )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplIdOrName,
            /* [in] */ __RPC__in BSTR bstrDLL,
            /* [in] */ __RPC__in BSTR bstrTLB,
            /* [in] */ __RPC__in BSTR bstrPSDLL);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog, GetEventClassesForIID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEventClassesForIID )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrIID,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarCLSIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarProgIDs,
            /* [out] */ __RPC__deref_out_opt SAFEARRAY * *ppsaVarDescriptions);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, GetCollectionByQuery2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCollectionByQuery2 )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrCollectionName,
            /* [in] */ __RPC__in VARIANT *pVarQueryStrings,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, GetApplicationInstanceIDFromProcessID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetApplicationInstanceIDFromProcessID )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ long lProcessID,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrApplicationInstanceID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, ShutdownApplicationInstances)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ShutdownApplicationInstances )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, PauseApplicationInstances)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PauseApplicationInstances )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, ResumeApplicationInstances)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ResumeApplicationInstances )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, RecycleApplicationInstances)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RecycleApplicationInstances )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID,
            /* [in] */ long lReasonCode);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, AreApplicationInstancesPaused)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AreApplicationInstancesPaused )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in VARIANT *pVarApplicationInstanceID,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBoolPaused);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, DumpApplicationInstance)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DumpApplicationInstance )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationInstanceID,
            /* [in] */ __RPC__in BSTR bstrDirectory,
            /* [in] */ long lMaxImages,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDumpFile);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, get_IsApplicationInstanceDumpSupported)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsApplicationInstanceDumpSupported )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBoolDumpSupported);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, CreateServiceForApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateServiceForApplication )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in BSTR bstrServiceName,
            /* [in] */ __RPC__in BSTR bstrStartType,
            /* [in] */ __RPC__in BSTR bstrErrorControl,
            /* [in] */ __RPC__in BSTR bstrDependencies,
            /* [in] */ __RPC__in BSTR bstrRunAs,
            /* [in] */ __RPC__in BSTR bstrPassword,
            /* [in] */ VARIANT_BOOL bDesktopOk);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, DeleteServiceForApplication)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteServiceForApplication )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, GetPartitionID)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPartitionID )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, GetPartitionName)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPartitionName )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, put_CurrentPartition)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CurrentPartition )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrPartitionIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, get_CurrentPartitionID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPartitionID )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, get_CurrentPartitionName)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentPartitionName )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrPartitionName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, get_GlobalPartitionID)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GlobalPartitionID )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrGlobalPartitionID);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, FlushPartitionCache)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FlushPartitionCache )( 
            __RPC__in ICOMAdminCatalog2 * This);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, CopyApplications)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyApplications )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrSourcePartitionIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarApplicationID,
            /* [in] */ __RPC__in BSTR bstrDestinationPartitionIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, CopyComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CopyComponents )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrSourceApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [in] */ __RPC__in BSTR bstrDestinationApplicationIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, MoveComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MoveComponents )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrSourceApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [in] */ __RPC__in BSTR bstrDestinationApplicationIDOrName);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, AliasComponent)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AliasComponent )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrSrcApplicationIDOrName,
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID,
            /* [in] */ __RPC__in BSTR bstrDestApplicationIDOrName,
            /* [in] */ __RPC__in BSTR bstrNewProgId,
            /* [in] */ __RPC__in BSTR bstrNewClsid);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, IsSafeToDelete)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsSafeToDelete )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrDllName,
            /* [retval][out] */ __RPC__out COMAdminInUse *pCOMAdminInUse);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, ImportUnconfiguredComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportUnconfiguredComponents )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [optional][in] */ __RPC__in VARIANT *pVarComponentType);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, PromoteUnconfiguredComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PromoteUnconfiguredComponents )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [optional][in] */ __RPC__in VARIANT *pVarComponentType);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, ImportComponents)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ImportComponents )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationIDOrName,
            /* [in] */ __RPC__in VARIANT *pVarCLSIDOrProgID,
            /* [optional][in] */ __RPC__in VARIANT *pVarComponentType);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, get_Is64BitCatalogServer)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Is64BitCatalogServer )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbIs64Bit);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, ExportPartition)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExportPartition )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrPartitionIDOrName,
            /* [in] */ __RPC__in BSTR bstrPartitionFileName,
            /* [in] */ long lOptions);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, InstallPartition)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InstallPartition )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrFileName,
            /* [in] */ __RPC__in BSTR bstrDestDirectory,
            /* [in] */ long lOptions,
            /* [in] */ __RPC__in BSTR bstrUserID,
            /* [in] */ __RPC__in BSTR bstrPassword,
            /* [in] */ __RPC__in BSTR bstrRSN);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, QueryApplicationFile2)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *QueryApplicationFile2 )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrApplicationFile,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppFilesForImport);
        
        DECLSPEC_XFGVIRT(ICOMAdminCatalog2, GetComponentVersionCount)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetComponentVersionCount )( 
            __RPC__in ICOMAdminCatalog2 * This,
            /* [in] */ __RPC__in BSTR bstrCLSIDOrProgID,
            /* [retval][out] */ __RPC__out long *plVersionCount);
        
        END_INTERFACE
    } ICOMAdminCatalog2Vtbl;

    interface ICOMAdminCatalog2
    {
        CONST_VTBL struct ICOMAdminCatalog2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICOMAdminCatalog2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICOMAdminCatalog2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICOMAdminCatalog2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICOMAdminCatalog2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICOMAdminCatalog2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICOMAdminCatalog2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICOMAdminCatalog2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICOMAdminCatalog2_GetCollection(This,bstrCollName,ppCatalogCollection)	\
    ( (This)->lpVtbl -> GetCollection(This,bstrCollName,ppCatalogCollection) ) 

#define ICOMAdminCatalog2_Connect(This,bstrCatalogServerName,ppCatalogCollection)	\
    ( (This)->lpVtbl -> Connect(This,bstrCatalogServerName,ppCatalogCollection) ) 

#define ICOMAdminCatalog2_get_MajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_MajorVersion(This,plMajorVersion) ) 

#define ICOMAdminCatalog2_get_MinorVersion(This,plMinorVersion)	\
    ( (This)->lpVtbl -> get_MinorVersion(This,plMinorVersion) ) 

#define ICOMAdminCatalog2_GetCollectionByQuery(This,bstrCollName,ppsaVarQuery,ppCatalogCollection)	\
    ( (This)->lpVtbl -> GetCollectionByQuery(This,bstrCollName,ppsaVarQuery,ppCatalogCollection) ) 

#define ICOMAdminCatalog2_ImportComponent(This,bstrApplIDOrName,bstrCLSIDOrProgID)	\
    ( (This)->lpVtbl -> ImportComponent(This,bstrApplIDOrName,bstrCLSIDOrProgID) ) 

#define ICOMAdminCatalog2_InstallComponent(This,bstrApplIDOrName,bstrDLL,bstrTLB,bstrPSDLL)	\
    ( (This)->lpVtbl -> InstallComponent(This,bstrApplIDOrName,bstrDLL,bstrTLB,bstrPSDLL) ) 

#define ICOMAdminCatalog2_ShutdownApplication(This,bstrApplIDOrName)	\
    ( (This)->lpVtbl -> ShutdownApplication(This,bstrApplIDOrName) ) 

#define ICOMAdminCatalog2_ExportApplication(This,bstrApplIDOrName,bstrApplicationFile,lOptions)	\
    ( (This)->lpVtbl -> ExportApplication(This,bstrApplIDOrName,bstrApplicationFile,lOptions) ) 

#define ICOMAdminCatalog2_InstallApplication(This,bstrApplicationFile,bstrDestinationDirectory,lOptions,bstrUserId,bstrPassword,bstrRSN)	\
    ( (This)->lpVtbl -> InstallApplication(This,bstrApplicationFile,bstrDestinationDirectory,lOptions,bstrUserId,bstrPassword,bstrRSN) ) 

#define ICOMAdminCatalog2_StopRouter(This)	\
    ( (This)->lpVtbl -> StopRouter(This) ) 

#define ICOMAdminCatalog2_RefreshRouter(This)	\
    ( (This)->lpVtbl -> RefreshRouter(This) ) 

#define ICOMAdminCatalog2_StartRouter(This)	\
    ( (This)->lpVtbl -> StartRouter(This) ) 

#define ICOMAdminCatalog2_Reserved1(This)	\
    ( (This)->lpVtbl -> Reserved1(This) ) 

#define ICOMAdminCatalog2_Reserved2(This)	\
    ( (This)->lpVtbl -> Reserved2(This) ) 

#define ICOMAdminCatalog2_InstallMultipleComponents(This,bstrApplIDOrName,ppsaVarFileNames,ppsaVarCLSIDs)	\
    ( (This)->lpVtbl -> InstallMultipleComponents(This,bstrApplIDOrName,ppsaVarFileNames,ppsaVarCLSIDs) ) 

#define ICOMAdminCatalog2_GetMultipleComponentsInfo(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDs,ppsaVarClassNames,ppsaVarFileFlags,ppsaVarComponentFlags)	\
    ( (This)->lpVtbl -> GetMultipleComponentsInfo(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDs,ppsaVarClassNames,ppsaVarFileFlags,ppsaVarComponentFlags) ) 

#define ICOMAdminCatalog2_RefreshComponents(This)	\
    ( (This)->lpVtbl -> RefreshComponents(This) ) 

#define ICOMAdminCatalog2_BackupREGDB(This,bstrBackupFilePath)	\
    ( (This)->lpVtbl -> BackupREGDB(This,bstrBackupFilePath) ) 

#define ICOMAdminCatalog2_RestoreREGDB(This,bstrBackupFilePath)	\
    ( (This)->lpVtbl -> RestoreREGDB(This,bstrBackupFilePath) ) 

#define ICOMAdminCatalog2_QueryApplicationFile(This,bstrApplicationFile,pbstrApplicationName,pbstrApplicationDescription,pbHasUsers,pbIsProxy,ppsaVarFileNames)	\
    ( (This)->lpVtbl -> QueryApplicationFile(This,bstrApplicationFile,pbstrApplicationName,pbstrApplicationDescription,pbHasUsers,pbIsProxy,ppsaVarFileNames) ) 

#define ICOMAdminCatalog2_StartApplication(This,bstrApplIdOrName)	\
    ( (This)->lpVtbl -> StartApplication(This,bstrApplIdOrName) ) 

#define ICOMAdminCatalog2_ServiceCheck(This,lService,plStatus)	\
    ( (This)->lpVtbl -> ServiceCheck(This,lService,plStatus) ) 

#define ICOMAdminCatalog2_InstallMultipleEventClasses(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDS)	\
    ( (This)->lpVtbl -> InstallMultipleEventClasses(This,bstrApplIdOrName,ppsaVarFileNames,ppsaVarCLSIDS) ) 

#define ICOMAdminCatalog2_InstallEventClass(This,bstrApplIdOrName,bstrDLL,bstrTLB,bstrPSDLL)	\
    ( (This)->lpVtbl -> InstallEventClass(This,bstrApplIdOrName,bstrDLL,bstrTLB,bstrPSDLL) ) 

#define ICOMAdminCatalog2_GetEventClassesForIID(This,bstrIID,ppsaVarCLSIDs,ppsaVarProgIDs,ppsaVarDescriptions)	\
    ( (This)->lpVtbl -> GetEventClassesForIID(This,bstrIID,ppsaVarCLSIDs,ppsaVarProgIDs,ppsaVarDescriptions) ) 


#define ICOMAdminCatalog2_GetCollectionByQuery2(This,bstrCollectionName,pVarQueryStrings,ppCatalogCollection)	\
    ( (This)->lpVtbl -> GetCollectionByQuery2(This,bstrCollectionName,pVarQueryStrings,ppCatalogCollection) ) 

#define ICOMAdminCatalog2_GetApplicationInstanceIDFromProcessID(This,lProcessID,pbstrApplicationInstanceID)	\
    ( (This)->lpVtbl -> GetApplicationInstanceIDFromProcessID(This,lProcessID,pbstrApplicationInstanceID) ) 

#define ICOMAdminCatalog2_ShutdownApplicationInstances(This,pVarApplicationInstanceID)	\
    ( (This)->lpVtbl -> ShutdownApplicationInstances(This,pVarApplicationInstanceID) ) 

#define ICOMAdminCatalog2_PauseApplicationInstances(This,pVarApplicationInstanceID)	\
    ( (This)->lpVtbl -> PauseApplicationInstances(This,pVarApplicationInstanceID) ) 

#define ICOMAdminCatalog2_ResumeApplicationInstances(This,pVarApplicationInstanceID)	\
    ( (This)->lpVtbl -> ResumeApplicationInstances(This,pVarApplicationInstanceID) ) 

#define ICOMAdminCatalog2_RecycleApplicationInstances(This,pVarApplicationInstanceID,lReasonCode)	\
    ( (This)->lpVtbl -> RecycleApplicationInstances(This,pVarApplicationInstanceID,lReasonCode) ) 

#define ICOMAdminCatalog2_AreApplicationInstancesPaused(This,pVarApplicationInstanceID,pVarBoolPaused)	\
    ( (This)->lpVtbl -> AreApplicationInstancesPaused(This,pVarApplicationInstanceID,pVarBoolPaused) ) 

#define ICOMAdminCatalog2_DumpApplicationInstance(This,bstrApplicationInstanceID,bstrDirectory,lMaxImages,pbstrDumpFile)	\
    ( (This)->lpVtbl -> DumpApplicationInstance(This,bstrApplicationInstanceID,bstrDirectory,lMaxImages,pbstrDumpFile) ) 

#define ICOMAdminCatalog2_get_IsApplicationInstanceDumpSupported(This,pVarBoolDumpSupported)	\
    ( (This)->lpVtbl -> get_IsApplicationInstanceDumpSupported(This,pVarBoolDumpSupported) ) 

#define ICOMAdminCatalog2_CreateServiceForApplication(This,bstrApplicationIDOrName,bstrServiceName,bstrStartType,bstrErrorControl,bstrDependencies,bstrRunAs,bstrPassword,bDesktopOk)	\
    ( (This)->lpVtbl -> CreateServiceForApplication(This,bstrApplicationIDOrName,bstrServiceName,bstrStartType,bstrErrorControl,bstrDependencies,bstrRunAs,bstrPassword,bDesktopOk) ) 

#define ICOMAdminCatalog2_DeleteServiceForApplication(This,bstrApplicationIDOrName)	\
    ( (This)->lpVtbl -> DeleteServiceForApplication(This,bstrApplicationIDOrName) ) 

#define ICOMAdminCatalog2_GetPartitionID(This,bstrApplicationIDOrName,pbstrPartitionID)	\
    ( (This)->lpVtbl -> GetPartitionID(This,bstrApplicationIDOrName,pbstrPartitionID) ) 

#define ICOMAdminCatalog2_GetPartitionName(This,bstrApplicationIDOrName,pbstrPartitionName)	\
    ( (This)->lpVtbl -> GetPartitionName(This,bstrApplicationIDOrName,pbstrPartitionName) ) 

#define ICOMAdminCatalog2_put_CurrentPartition(This,bstrPartitionIDOrName)	\
    ( (This)->lpVtbl -> put_CurrentPartition(This,bstrPartitionIDOrName) ) 

#define ICOMAdminCatalog2_get_CurrentPartitionID(This,pbstrPartitionID)	\
    ( (This)->lpVtbl -> get_CurrentPartitionID(This,pbstrPartitionID) ) 

#define ICOMAdminCatalog2_get_CurrentPartitionName(This,pbstrPartitionName)	\
    ( (This)->lpVtbl -> get_CurrentPartitionName(This,pbstrPartitionName) ) 

#define ICOMAdminCatalog2_get_GlobalPartitionID(This,pbstrGlobalPartitionID)	\
    ( (This)->lpVtbl -> get_GlobalPartitionID(This,pbstrGlobalPartitionID) ) 

#define ICOMAdminCatalog2_FlushPartitionCache(This)	\
    ( (This)->lpVtbl -> FlushPartitionCache(This) ) 

#define ICOMAdminCatalog2_CopyApplications(This,bstrSourcePartitionIDOrName,pVarApplicationID,bstrDestinationPartitionIDOrName)	\
    ( (This)->lpVtbl -> CopyApplications(This,bstrSourcePartitionIDOrName,pVarApplicationID,bstrDestinationPartitionIDOrName) ) 

#define ICOMAdminCatalog2_CopyComponents(This,bstrSourceApplicationIDOrName,pVarCLSIDOrProgID,bstrDestinationApplicationIDOrName)	\
    ( (This)->lpVtbl -> CopyComponents(This,bstrSourceApplicationIDOrName,pVarCLSIDOrProgID,bstrDestinationApplicationIDOrName) ) 

#define ICOMAdminCatalog2_MoveComponents(This,bstrSourceApplicationIDOrName,pVarCLSIDOrProgID,bstrDestinationApplicationIDOrName)	\
    ( (This)->lpVtbl -> MoveComponents(This,bstrSourceApplicationIDOrName,pVarCLSIDOrProgID,bstrDestinationApplicationIDOrName) ) 

#define ICOMAdminCatalog2_AliasComponent(This,bstrSrcApplicationIDOrName,bstrCLSIDOrProgID,bstrDestApplicationIDOrName,bstrNewProgId,bstrNewClsid)	\
    ( (This)->lpVtbl -> AliasComponent(This,bstrSrcApplicationIDOrName,bstrCLSIDOrProgID,bstrDestApplicationIDOrName,bstrNewProgId,bstrNewClsid) ) 

#define ICOMAdminCatalog2_IsSafeToDelete(This,bstrDllName,pCOMAdminInUse)	\
    ( (This)->lpVtbl -> IsSafeToDelete(This,bstrDllName,pCOMAdminInUse) ) 

#define ICOMAdminCatalog2_ImportUnconfiguredComponents(This,bstrApplicationIDOrName,pVarCLSIDOrProgID,pVarComponentType)	\
    ( (This)->lpVtbl -> ImportUnconfiguredComponents(This,bstrApplicationIDOrName,pVarCLSIDOrProgID,pVarComponentType) ) 

#define ICOMAdminCatalog2_PromoteUnconfiguredComponents(This,bstrApplicationIDOrName,pVarCLSIDOrProgID,pVarComponentType)	\
    ( (This)->lpVtbl -> PromoteUnconfiguredComponents(This,bstrApplicationIDOrName,pVarCLSIDOrProgID,pVarComponentType) ) 

#define ICOMAdminCatalog2_ImportComponents(This,bstrApplicationIDOrName,pVarCLSIDOrProgID,pVarComponentType)	\
    ( (This)->lpVtbl -> ImportComponents(This,bstrApplicationIDOrName,pVarCLSIDOrProgID,pVarComponentType) ) 

#define ICOMAdminCatalog2_get_Is64BitCatalogServer(This,pbIs64Bit)	\
    ( (This)->lpVtbl -> get_Is64BitCatalogServer(This,pbIs64Bit) ) 

#define ICOMAdminCatalog2_ExportPartition(This,bstrPartitionIDOrName,bstrPartitionFileName,lOptions)	\
    ( (This)->lpVtbl -> ExportPartition(This,bstrPartitionIDOrName,bstrPartitionFileName,lOptions) ) 

#define ICOMAdminCatalog2_InstallPartition(This,bstrFileName,bstrDestDirectory,lOptions,bstrUserID,bstrPassword,bstrRSN)	\
    ( (This)->lpVtbl -> InstallPartition(This,bstrFileName,bstrDestDirectory,lOptions,bstrUserID,bstrPassword,bstrRSN) ) 

#define ICOMAdminCatalog2_QueryApplicationFile2(This,bstrApplicationFile,ppFilesForImport)	\
    ( (This)->lpVtbl -> QueryApplicationFile2(This,bstrApplicationFile,ppFilesForImport) ) 

#define ICOMAdminCatalog2_GetComponentVersionCount(This,bstrCLSIDOrProgID,plVersionCount)	\
    ( (This)->lpVtbl -> GetComponentVersionCount(This,bstrCLSIDOrProgID,plVersionCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICOMAdminCatalog2_INTERFACE_DEFINED__ */


#ifndef __ICatalogObject_INTERFACE_DEFINED__
#define __ICatalogObject_INTERFACE_DEFINED__

/* interface ICatalogObject */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICatalogObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6eb22871-8a19-11d0-81b6-00a0c9231c29")
    ICatalogObject : public IDispatch
    {
    public:
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT *pvarRetVal) = 0;
        
        virtual /* [helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [in] */ VARIANT val) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Key( 
            /* [retval][out] */ __RPC__out VARIANT *pvarRetVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__out VARIANT *pvarRetVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsPropertyReadOnly( 
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRetVal) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Valid( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRetVal) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE IsPropertyWriteOnly( 
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICatalogObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICatalogObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICatalogObject * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICatalogObject * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICatalogObject * This,
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
        
        DECLSPEC_XFGVIRT(ICatalogObject, get_Value)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT *pvarRetVal);
        
        DECLSPEC_XFGVIRT(ICatalogObject, put_Value)
        /* [helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [in] */ VARIANT val);
        
        DECLSPEC_XFGVIRT(ICatalogObject, get_Key)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Key )( 
            __RPC__in ICatalogObject * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarRetVal);
        
        DECLSPEC_XFGVIRT(ICatalogObject, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ICatalogObject * This,
            /* [retval][out] */ __RPC__out VARIANT *pvarRetVal);
        
        DECLSPEC_XFGVIRT(ICatalogObject, IsPropertyReadOnly)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsPropertyReadOnly )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRetVal);
        
        DECLSPEC_XFGVIRT(ICatalogObject, get_Valid)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Valid )( 
            __RPC__in ICatalogObject * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRetVal);
        
        DECLSPEC_XFGVIRT(ICatalogObject, IsPropertyWriteOnly)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsPropertyWriteOnly )( 
            __RPC__in ICatalogObject * This,
            /* [in] */ __RPC__in BSTR bstrPropName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbRetVal);
        
        END_INTERFACE
    } ICatalogObjectVtbl;

    interface ICatalogObject
    {
        CONST_VTBL struct ICatalogObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalogObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICatalogObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICatalogObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICatalogObject_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICatalogObject_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICatalogObject_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICatalogObject_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICatalogObject_get_Value(This,bstrPropName,pvarRetVal)	\
    ( (This)->lpVtbl -> get_Value(This,bstrPropName,pvarRetVal) ) 

#define ICatalogObject_put_Value(This,bstrPropName,val)	\
    ( (This)->lpVtbl -> put_Value(This,bstrPropName,val) ) 

#define ICatalogObject_get_Key(This,pvarRetVal)	\
    ( (This)->lpVtbl -> get_Key(This,pvarRetVal) ) 

#define ICatalogObject_get_Name(This,pvarRetVal)	\
    ( (This)->lpVtbl -> get_Name(This,pvarRetVal) ) 

#define ICatalogObject_IsPropertyReadOnly(This,bstrPropName,pbRetVal)	\
    ( (This)->lpVtbl -> IsPropertyReadOnly(This,bstrPropName,pbRetVal) ) 

#define ICatalogObject_get_Valid(This,pbRetVal)	\
    ( (This)->lpVtbl -> get_Valid(This,pbRetVal) ) 

#define ICatalogObject_IsPropertyWriteOnly(This,bstrPropName,pbRetVal)	\
    ( (This)->lpVtbl -> IsPropertyWriteOnly(This,bstrPropName,pbRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICatalogObject_INTERFACE_DEFINED__ */


#ifndef __ICatalogCollection_INTERFACE_DEFINED__
#define __ICatalogCollection_INTERFACE_DEFINED__

/* interface ICatalogCollection */
/* [unique][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ICatalogCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6eb22872-8a19-11d0-81b6-00a0c9231c29")
    ICatalogCollection : public IDispatch
    {
    public:
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumVariant) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Item( 
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogObject) = 0;
        
        virtual /* [helpstring][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *plObjectCount) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ long lIndex) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogObject) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE Populate( void) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE SaveChanges( 
            /* [retval][out] */ __RPC__out long *pcChanges) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetCollection( 
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [in] */ VARIANT varObjectKey,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__out VARIANT *pVarNamel) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AddEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBool) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RemoveEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBool) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetUtilInterface( 
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppIDispatch) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataStoreMajorVersion( 
            /* [retval][out] */ __RPC__out long *plMajorVersion) = 0;
        
        virtual /* [helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DataStoreMinorVersion( 
            /* [retval][out] */ __RPC__out long *plMinorVersionl) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PopulateByKey( 
            /* [in] */ __RPC__in SAFEARRAY * psaKeys) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PopulateByQuery( 
            /* [in] */ __RPC__in BSTR bstrQueryString,
            /* [in] */ long lQueryType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICatalogCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICatalogCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICatalogCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ICatalogCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ICatalogCollection * This,
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
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppEnumVariant);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_Item)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Item )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ long lIndex,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogObject);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_Count)
        /* [helpstring][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out long *plObjectCount);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, Remove)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ long lIndex);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, Add)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogObject);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, Populate)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Populate )( 
            __RPC__in ICatalogCollection * This);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, SaveChanges)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SaveChanges )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out long *pcChanges);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, GetCollection)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetCollection )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ __RPC__in BSTR bstrCollName,
            /* [in] */ VARIANT varObjectKey,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppCatalogCollection);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_Name)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out VARIANT *pVarNamel);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_AddEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AddEnabled )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBool);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_RemoveEnabled)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RemoveEnabled )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pVarBool);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, GetUtilInterface)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetUtilInterface )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppIDispatch);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_DataStoreMajorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataStoreMajorVersion )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out long *plMajorVersion);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, get_DataStoreMinorVersion)
        /* [helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataStoreMinorVersion )( 
            __RPC__in ICatalogCollection * This,
            /* [retval][out] */ __RPC__out long *plMinorVersionl);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, PopulateByKey)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PopulateByKey )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ __RPC__in SAFEARRAY * psaKeys);
        
        DECLSPEC_XFGVIRT(ICatalogCollection, PopulateByQuery)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PopulateByQuery )( 
            __RPC__in ICatalogCollection * This,
            /* [in] */ __RPC__in BSTR bstrQueryString,
            /* [in] */ long lQueryType);
        
        END_INTERFACE
    } ICatalogCollectionVtbl;

    interface ICatalogCollection
    {
        CONST_VTBL struct ICatalogCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICatalogCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICatalogCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICatalogCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICatalogCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ICatalogCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ICatalogCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ICatalogCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define ICatalogCollection_get__NewEnum(This,ppEnumVariant)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppEnumVariant) ) 

#define ICatalogCollection_get_Item(This,lIndex,ppCatalogObject)	\
    ( (This)->lpVtbl -> get_Item(This,lIndex,ppCatalogObject) ) 

#define ICatalogCollection_get_Count(This,plObjectCount)	\
    ( (This)->lpVtbl -> get_Count(This,plObjectCount) ) 

#define ICatalogCollection_Remove(This,lIndex)	\
    ( (This)->lpVtbl -> Remove(This,lIndex) ) 

#define ICatalogCollection_Add(This,ppCatalogObject)	\
    ( (This)->lpVtbl -> Add(This,ppCatalogObject) ) 

#define ICatalogCollection_Populate(This)	\
    ( (This)->lpVtbl -> Populate(This) ) 

#define ICatalogCollection_SaveChanges(This,pcChanges)	\
    ( (This)->lpVtbl -> SaveChanges(This,pcChanges) ) 

#define ICatalogCollection_GetCollection(This,bstrCollName,varObjectKey,ppCatalogCollection)	\
    ( (This)->lpVtbl -> GetCollection(This,bstrCollName,varObjectKey,ppCatalogCollection) ) 

#define ICatalogCollection_get_Name(This,pVarNamel)	\
    ( (This)->lpVtbl -> get_Name(This,pVarNamel) ) 

#define ICatalogCollection_get_AddEnabled(This,pVarBool)	\
    ( (This)->lpVtbl -> get_AddEnabled(This,pVarBool) ) 

#define ICatalogCollection_get_RemoveEnabled(This,pVarBool)	\
    ( (This)->lpVtbl -> get_RemoveEnabled(This,pVarBool) ) 

#define ICatalogCollection_GetUtilInterface(This,ppIDispatch)	\
    ( (This)->lpVtbl -> GetUtilInterface(This,ppIDispatch) ) 

#define ICatalogCollection_get_DataStoreMajorVersion(This,plMajorVersion)	\
    ( (This)->lpVtbl -> get_DataStoreMajorVersion(This,plMajorVersion) ) 

#define ICatalogCollection_get_DataStoreMinorVersion(This,plMinorVersionl)	\
    ( (This)->lpVtbl -> get_DataStoreMinorVersion(This,plMinorVersionl) ) 

#define ICatalogCollection_PopulateByKey(This,psaKeys)	\
    ( (This)->lpVtbl -> PopulateByKey(This,psaKeys) ) 

#define ICatalogCollection_PopulateByQuery(This,bstrQueryString,lQueryType)	\
    ( (This)->lpVtbl -> PopulateByQuery(This,bstrQueryString,lQueryType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICatalogCollection_INTERFACE_DEFINED__ */



#ifndef __COMAdmin_LIBRARY_DEFINED__
#define __COMAdmin_LIBRARY_DEFINED__

/* library COMAdmin */
/* [helpstring][version][uuid] */ 

typedef /* [helpstring] */ 
enum COMAdminComponentType
    {
        COMAdmin32BitComponent	= 0x1,
        COMAdmin64BitComponent	= 0x2
    } 	COMAdminComponentType;

typedef /* [helpstring] */ 
enum COMAdminApplicationInstallOptions
    {
        COMAdminInstallNoUsers	= 0,
        COMAdminInstallUsers	= 1,
        COMAdminInstallForceOverwriteOfFiles	= 2
    } 	COMAdminApplicationInstallOptions;

typedef /* [helpstring] */ 
enum COMAdminApplicationExportOptions
    {
        COMAdminExportNoUsers	= 0,
        COMAdminExportUsers	= 1,
        COMAdminExportApplicationProxy	= 2,
        COMAdminExportForceOverwriteOfFiles	= 4,
        COMAdminExportIn10Format	= 16
    } 	COMAdminApplicationExportOptions;

typedef /* [helpstring] */ 
enum COMAdminThreadingModels
    {
        COMAdminThreadingModelApartment	= 0,
        COMAdminThreadingModelFree	= 1,
        COMAdminThreadingModelMain	= 2,
        COMAdminThreadingModelBoth	= 3,
        COMAdminThreadingModelNeutral	= 4,
        COMAdminThreadingModelNotSpecified	= 5
    } 	COMAdminThreadingModels;

typedef /* [helpstring] */ 
enum COMAdminTransactionOptions
    {
        COMAdminTransactionIgnored	= 0,
        COMAdminTransactionNone	= 1,
        COMAdminTransactionSupported	= 2,
        COMAdminTransactionRequired	= 3,
        COMAdminTransactionRequiresNew	= 4
    } 	COMAdminTransactionOptions;

typedef /* [helpstring] */ 
enum COMAdminTxIsolationLevelOptions
    {
        COMAdminTxIsolationLevelAny	= 0,
        COMAdminTxIsolationLevelReadUnCommitted	= ( COMAdminTxIsolationLevelAny + 1 ) ,
        COMAdminTxIsolationLevelReadCommitted	= ( COMAdminTxIsolationLevelReadUnCommitted + 1 ) ,
        COMAdminTxIsolationLevelRepeatableRead	= ( COMAdminTxIsolationLevelReadCommitted + 1 ) ,
        COMAdminTxIsolationLevelSerializable	= ( COMAdminTxIsolationLevelRepeatableRead + 1 ) 
    } 	COMAdminTxIsolationLevelOptions;

typedef /* [helpstring] */ 
enum COMAdminSynchronizationOptions
    {
        COMAdminSynchronizationIgnored	= 0,
        COMAdminSynchronizationNone	= 1,
        COMAdminSynchronizationSupported	= 2,
        COMAdminSynchronizationRequired	= 3,
        COMAdminSynchronizationRequiresNew	= 4
    } 	COMAdminSynchronizationOptions;

typedef /* [helpstring] */ 
enum COMAdminActivationOptions
    {
        COMAdminActivationInproc	= 0,
        COMAdminActivationLocal	= 1
    } 	COMAdminActivationOptions;

typedef /* [helpstring] */ 
enum COMAdminAccessChecksLevelOptions
    {
        COMAdminAccessChecksApplicationLevel	= 0,
        COMAdminAccessChecksApplicationComponentLevel	= 1
    } 	COMAdminAccessChecksLevelOptions;

typedef /* [helpstring] */ 
enum COMAdminAuthenticationLevelOptions
    {
        COMAdminAuthenticationDefault	= 0,
        COMAdminAuthenticationNone	= 1,
        COMAdminAuthenticationConnect	= 2,
        COMAdminAuthenticationCall	= 3,
        COMAdminAuthenticationPacket	= 4,
        COMAdminAuthenticationIntegrity	= 5,
        COMAdminAuthenticationPrivacy	= 6
    } 	COMAdminAuthenticationLevelOptions;

typedef /* [helpstring] */ 
enum COMAdminImpersonationLevelOptions
    {
        COMAdminImpersonationAnonymous	= 1,
        COMAdminImpersonationIdentify	= 2,
        COMAdminImpersonationImpersonate	= 3,
        COMAdminImpersonationDelegate	= 4
    } 	COMAdminImpersonationLevelOptions;

typedef /* [helpstring] */ 
enum COMAdminAuthenticationCapabilitiesOptions
    {
        COMAdminAuthenticationCapabilitiesNone	= 0,
        COMAdminAuthenticationCapabilitiesSecureReference	= 0x2,
        COMAdminAuthenticationCapabilitiesStaticCloaking	= 0x20,
        COMAdminAuthenticationCapabilitiesDynamicCloaking	= 0x40
    } 	COMAdminAuthenticationCapabilitiesOptions;

typedef /* [helpstring] */ 
enum COMAdminOS
    {
        COMAdminOSNotInitialized	= 0,
        COMAdminOSWindows3_1	= 1,
        COMAdminOSWindows9x	= 2,
        COMAdminOSWindows2000	= 3,
        COMAdminOSWindows2000AdvancedServer	= 4,
        COMAdminOSWindows2000Unknown	= 5,
        COMAdminOSUnknown	= 6,
        COMAdminOSWindowsXPPersonal	= 11,
        COMAdminOSWindowsXPProfessional	= 12,
        COMAdminOSWindowsNETStandardServer	= 13,
        COMAdminOSWindowsNETEnterpriseServer	= 14,
        COMAdminOSWindowsNETDatacenterServer	= 15,
        COMAdminOSWindowsNETWebServer	= 16,
        COMAdminOSWindowsLonghornPersonal	= 17,
        COMAdminOSWindowsLonghornProfessional	= 18,
        COMAdminOSWindowsLonghornStandardServer	= 19,
        COMAdminOSWindowsLonghornEnterpriseServer	= 20,
        COMAdminOSWindowsLonghornDatacenterServer	= 21,
        COMAdminOSWindowsLonghornWebServer	= 22,
        COMAdminOSWindows7Personal	= 23,
        COMAdminOSWindows7Professional	= 24,
        COMAdminOSWindows7StandardServer	= 25,
        COMAdminOSWindows7EnterpriseServer	= 26,
        COMAdminOSWindows7DatacenterServer	= 27,
        COMAdminOSWindows7WebServer	= 28,
        COMAdminOSWindows8Personal	= 29,
        COMAdminOSWindows8Professional	= 30,
        COMAdminOSWindows8StandardServer	= 31,
        COMAdminOSWindows8EnterpriseServer	= 32,
        COMAdminOSWindows8DatacenterServer	= 33,
        COMAdminOSWindows8WebServer	= 34,
        COMAdminOSWindowsBluePersonal	= 35,
        COMAdminOSWindowsBlueProfessional	= 36,
        COMAdminOSWindowsBlueStandardServer	= 37,
        COMAdminOSWindowsBlueEnterpriseServer	= 38,
        COMAdminOSWindowsBlueDatacenterServer	= 39,
        COMAdminOSWindowsBlueWebServer	= 40
    } 	COMAdminOS;

typedef /* [helpstring] */ 
enum COMAdminServiceOptions
    {
        COMAdminServiceLoadBalanceRouter	= 1
    } 	COMAdminServiceOptions;

typedef /* [helpstring] */ 
enum COMAdminServiceStatusOptions
    {
        COMAdminServiceStopped	= 0,
        COMAdminServiceStartPending	= ( COMAdminServiceStopped + 1 ) ,
        COMAdminServiceStopPending	= ( COMAdminServiceStartPending + 1 ) ,
        COMAdminServiceRunning	= ( COMAdminServiceStopPending + 1 ) ,
        COMAdminServiceContinuePending	= ( COMAdminServiceRunning + 1 ) ,
        COMAdminServicePausePending	= ( COMAdminServiceContinuePending + 1 ) ,
        COMAdminServicePaused	= ( COMAdminServicePausePending + 1 ) ,
        COMAdminServiceUnknownState	= ( COMAdminServicePaused + 1 ) 
    } 	COMAdminServiceStatusOptions;

typedef /* [helpstring] */ 
enum COMAdminQCMessageAuthenticateOptions
    {
        COMAdminQCMessageAuthenticateSecureApps	= 0,
        COMAdminQCMessageAuthenticateOff	= 1,
        COMAdminQCMessageAuthenticateOn	= 2
    } 	COMAdminQCMessageAuthenticateOptions;

typedef /* [helpstring] */ 
enum COMAdminFileFlags
    {
        COMAdminFileFlagLoadable	= 0x1,
        COMAdminFileFlagCOM	= 0x2,
        COMAdminFileFlagContainsPS	= 0x4,
        COMAdminFileFlagContainsComp	= 0x8,
        COMAdminFileFlagContainsTLB	= 0x10,
        COMAdminFileFlagSelfReg	= 0x20,
        COMAdminFileFlagSelfUnReg	= 0x40,
        COMAdminFileFlagUnloadableDLL	= 0x80,
        COMAdminFileFlagDoesNotExist	= 0x100,
        COMAdminFileFlagAlreadyInstalled	= 0x200,
        COMAdminFileFlagBadTLB	= 0x400,
        COMAdminFileFlagGetClassObjFailed	= 0x800,
        COMAdminFileFlagClassNotAvailable	= 0x1000,
        COMAdminFileFlagRegistrar	= 0x2000,
        COMAdminFileFlagNoRegistrar	= 0x4000,
        COMAdminFileFlagDLLRegsvrFailed	= 0x8000,
        COMAdminFileFlagRegTLBFailed	= 0x10000,
        COMAdminFileFlagRegistrarFailed	= 0x20000,
        COMAdminFileFlagError	= 0x40000
    } 	COMAdminFileFlags;

typedef /* [helpstring] */ 
enum COMAdminComponentFlags
    {
        COMAdminCompFlagTypeInfoFound	= 0x1,
        COMAdminCompFlagCOMPlusPropertiesFound	= 0x2,
        COMAdminCompFlagProxyFound	= 0x4,
        COMAdminCompFlagInterfacesFound	= 0x8,
        COMAdminCompFlagAlreadyInstalled	= 0x10,
        COMAdminCompFlagNotInApplication	= 0x20
    } 	COMAdminComponentFlags;

#define	COMAdminCollectionRoot	( "Root" )

#define	COMAdminCollectionApplications	( "Applications" )

#define	COMAdminCollectionComponents	( "Components" )

#define	COMAdminCollectionComputerList	( "ComputerList" )

#define	COMAdminCollectionApplicationCluster	( "ApplicationCluster" )

#define	COMAdminCollectionLocalComputer	( "LocalComputer" )

#define	COMAdminCollectionInprocServers	( "InprocServers" )

#define	COMAdminCollectionRelatedCollectionInfo	( "RelatedCollectionInfo" )

#define	COMAdminCollectionPropertyInfo	( "PropertyInfo" )

#define	COMAdminCollectionRoles	( "Roles" )

#define	COMAdminCollectionErrorInfo	( "ErrorInfo" )

#define	COMAdminCollectionInterfacesForComponent	( "InterfacesForComponent" )

#define	COMAdminCollectionRolesForComponent	( "RolesForComponent" )

#define	COMAdminCollectionMethodsForInterface	( "MethodsForInterface" )

#define	COMAdminCollectionRolesForInterface	( "RolesForInterface" )

#define	COMAdminCollectionRolesForMethod	( "RolesForMethod" )

#define	COMAdminCollectionUsersInRole	( "UsersInRole" )

#define	COMAdminCollectionDCOMProtocols	( "DCOMProtocols" )

#define	COMAdminCollectionPartitions	( "Partitions" )

typedef /* [helpstring] */ 
enum COMAdminErrorCodes
    {
        COMAdminErrObjectErrors	= ( HRESULT  )0x80110401L,
        COMAdminErrObjectInvalid	= ( HRESULT  )0x80110402L,
        COMAdminErrKeyMissing	= ( HRESULT  )0x80110403L,
        COMAdminErrAlreadyInstalled	= ( HRESULT  )0x80110404L,
        COMAdminErrAppFileWriteFail	= ( HRESULT  )0x80110407L,
        COMAdminErrAppFileReadFail	= ( HRESULT  )0x80110408L,
        COMAdminErrAppFileVersion	= ( HRESULT  )0x80110409L,
        COMAdminErrBadPath	= ( HRESULT  )0x8011040aL,
        COMAdminErrApplicationExists	= ( HRESULT  )0x8011040bL,
        COMAdminErrRoleExists	= ( HRESULT  )0x8011040cL,
        COMAdminErrCantCopyFile	= ( HRESULT  )0x8011040dL,
        COMAdminErrNoUser	= ( HRESULT  )0x8011040fL,
        COMAdminErrInvalidUserids	= ( HRESULT  )0x80110410L,
        COMAdminErrNoRegistryCLSID	= ( HRESULT  )0x80110411L,
        COMAdminErrBadRegistryProgID	= ( HRESULT  )0x80110412L,
        COMAdminErrAuthenticationLevel	= ( HRESULT  )0x80110413L,
        COMAdminErrUserPasswdNotValid	= ( HRESULT  )0x80110414L,
        COMAdminErrCLSIDOrIIDMismatch	= ( HRESULT  )0x80110418L,
        COMAdminErrRemoteInterface	= ( HRESULT  )0x80110419L,
        COMAdminErrDllRegisterServer	= ( HRESULT  )0x8011041aL,
        COMAdminErrNoServerShare	= ( HRESULT  )0x8011041bL,
        COMAdminErrDllLoadFailed	= ( HRESULT  )0x8011041dL,
        COMAdminErrBadRegistryLibID	= ( HRESULT  )0x8011041eL,
        COMAdminErrAppDirNotFound	= ( HRESULT  )0x8011041fL,
        COMAdminErrRegistrarFailed	= ( HRESULT  )0x80110423L,
        COMAdminErrCompFileDoesNotExist	= ( HRESULT  )0x80110424L,
        COMAdminErrCompFileLoadDLLFail	= ( HRESULT  )0x80110425L,
        COMAdminErrCompFileGetClassObj	= ( HRESULT  )0x80110426L,
        COMAdminErrCompFileClassNotAvail	= ( HRESULT  )0x80110427L,
        COMAdminErrCompFileBadTLB	= ( HRESULT  )0x80110428L,
        COMAdminErrCompFileNotInstallable	= ( HRESULT  )0x80110429L,
        COMAdminErrNotChangeable	= ( HRESULT  )0x8011042aL,
        COMAdminErrNotDeletable	= ( HRESULT  )0x8011042bL,
        COMAdminErrSession	= ( HRESULT  )0x8011042cL,
        COMAdminErrCompMoveLocked	= ( HRESULT  )0x8011042dL,
        COMAdminErrCompMoveBadDest	= ( HRESULT  )0x8011042eL,
        COMAdminErrRegisterTLB	= ( HRESULT  )0x80110430L,
        COMAdminErrSystemApp	= ( HRESULT  )0x80110433L,
        COMAdminErrCompFileNoRegistrar	= ( HRESULT  )0x80110434L,
        COMAdminErrCoReqCompInstalled	= ( HRESULT  )0x80110435L,
        COMAdminErrServiceNotInstalled	= ( HRESULT  )0x80110436L,
        COMAdminErrPropertySaveFailed	= ( HRESULT  )0x80110437L,
        COMAdminErrObjectExists	= ( HRESULT  )0x80110438L,
        COMAdminErrComponentExists	= ( HRESULT  )0x80110439L,
        COMAdminErrRegFileCorrupt	= ( HRESULT  )0x8011043bL,
        COMAdminErrPropertyOverflow	= ( HRESULT  )0x8011043cL,
        COMAdminErrNotInRegistry	= ( HRESULT  )0x8011043eL,
        COMAdminErrObjectNotPoolable	= ( HRESULT  )0x8011043fL,
        COMAdminErrApplidMatchesClsid	= ( HRESULT  )0x80110446L,
        COMAdminErrRoleDoesNotExist	= ( HRESULT  )0x80110447L,
        COMAdminErrStartAppNeedsComponents	= ( HRESULT  )0x80110448L,
        COMAdminErrRequiresDifferentPlatform	= ( HRESULT  )0x80110449L,
        COMAdminErrQueuingServiceNotAvailable	= ( HRESULT  )0x80110602L,
        COMAdminErrObjectParentMissing	= ( HRESULT  )0x80110808L,
        COMAdminErrObjectDoesNotExist	= ( HRESULT  )0x80110809L,
        COMAdminErrCanNotExportAppProxy	= ( HRESULT  )0x8011044aL,
        COMAdminErrCanNotStartApp	= ( HRESULT  )0x8011044bL,
        COMAdminErrCanNotExportSystemApp	= ( HRESULT  )0x8011044cL,
        COMAdminErrCanNotSubscribeToComponent	= ( HRESULT  )0x8011044dL,
        COMAdminErrAppNotRunning	= ( HRESULT  )0x8011080aL,
        COMAdminErrEventClassCannotBeSubscriber	= ( HRESULT  )0x8011044eL,
        COMAdminErrLibAppProxyIncompatible	= ( HRESULT  )0x8011044fL,
        COMAdminErrBasePartitionOnly	= ( HRESULT  )0x80110450L,
        COMAdminErrDuplicatePartitionName	= ( HRESULT  )0x80110457L,
        COMAdminErrPartitionInUse	= ( HRESULT  )0x80110459L,
        COMAdminErrImportedComponentsNotAllowed	= ( HRESULT  )0x8011045bL,
        COMAdminErrRegdbNotInitialized	= ( HRESULT  )0x80110472L,
        COMAdminErrRegdbNotOpen	= ( HRESULT  )0x80110473L,
        COMAdminErrRegdbSystemErr	= ( HRESULT  )0x80110474L,
        COMAdminErrRegdbAlreadyRunning	= ( HRESULT  )0x80110475L,
        COMAdminErrMigVersionNotSupported	= ( HRESULT  )0x80110480L,
        COMAdminErrMigSchemaNotFound	= ( HRESULT  )0x80110481L,
        COMAdminErrCatBitnessMismatch	= ( HRESULT  )0x80110482L,
        COMAdminErrCatUnacceptableBitness	= ( HRESULT  )0x80110483L,
        COMAdminErrCatWrongAppBitnessBitness	= ( HRESULT  )0x80110484L,
        COMAdminErrCatPauseResumeNotSupported	= ( HRESULT  )0x80110485L,
        COMAdminErrCatServerFault	= ( HRESULT  )0x80110486L,
        COMAdminErrCantRecycleLibraryApps	= ( HRESULT  )0x8011080fL,
        COMAdminErrCantRecycleServiceApps	= ( HRESULT  )0x80110811L,
        COMAdminErrProcessAlreadyRecycled	= ( HRESULT  )0x80110812L,
        COMAdminErrPausedProcessMayNotBeRecycled	= ( HRESULT  )0x80110813L,
        COMAdminErrInvalidPartition	= ( HRESULT  )0x8011080bL,
        COMAdminErrPartitionMsiOnly	= ( HRESULT  )0x80110819L,
        COMAdminErrStartAppDisabled	= ( HRESULT  )0x80110451L,
        COMAdminErrCompMoveSource	= ( HRESULT  )0x8011081cL,
        COMAdminErrCompMoveDest	= ( HRESULT  )0x8011081dL,
        COMAdminErrCompMovePrivate	= ( HRESULT  )0x8011081eL,
        COMAdminErrCannotCopyEventClass	= ( HRESULT  )0x80110820L
    } 	COMAdminErrorCodes;


EXTERN_C const IID LIBID_COMAdmin;

EXTERN_C const CLSID CLSID_COMAdminCatalog;

#ifdef __cplusplus

class DECLSPEC_UUID("F618C514-DFB8-11d1-A2CF-00805FC79235")
COMAdminCatalog;
#endif

EXTERN_C const CLSID CLSID_COMAdminCatalogObject;

#ifdef __cplusplus

class DECLSPEC_UUID("F618C515-DFB8-11d1-A2CF-00805FC79235")
COMAdminCatalogObject;
#endif

EXTERN_C const CLSID CLSID_COMAdminCatalogCollection;

#ifdef __cplusplus

class DECLSPEC_UUID("F618C516-DFB8-11d1-A2CF-00805FC79235")
COMAdminCatalogCollection;
#endif
#endif /* __COMAdmin_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_comadmin_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_comadmin_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_comadmin_0000_0005_v0_0_s_ifspec;

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


