

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

#ifndef __documenttarget_h__
#define __documenttarget_h__

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

#ifndef __IPrintDocumentPackageTarget_FWD_DEFINED__
#define __IPrintDocumentPackageTarget_FWD_DEFINED__
typedef interface IPrintDocumentPackageTarget IPrintDocumentPackageTarget;

#endif 	/* __IPrintDocumentPackageTarget_FWD_DEFINED__ */


#ifndef __IPrintDocumentPackageTarget2_FWD_DEFINED__
#define __IPrintDocumentPackageTarget2_FWD_DEFINED__
typedef interface IPrintDocumentPackageTarget2 IPrintDocumentPackageTarget2;

#endif 	/* __IPrintDocumentPackageTarget2_FWD_DEFINED__ */


#ifndef __IPrintDocumentPackageStatusEvent_FWD_DEFINED__
#define __IPrintDocumentPackageStatusEvent_FWD_DEFINED__
typedef interface IPrintDocumentPackageStatusEvent IPrintDocumentPackageStatusEvent;

#endif 	/* __IPrintDocumentPackageStatusEvent_FWD_DEFINED__ */


#ifndef __IPrintDocumentPackageTargetFactory_FWD_DEFINED__
#define __IPrintDocumentPackageTargetFactory_FWD_DEFINED__
typedef interface IPrintDocumentPackageTargetFactory IPrintDocumentPackageTargetFactory;

#endif 	/* __IPrintDocumentPackageTargetFactory_FWD_DEFINED__ */


#ifndef __IPrintDocumentPackageTarget_FWD_DEFINED__
#define __IPrintDocumentPackageTarget_FWD_DEFINED__
typedef interface IPrintDocumentPackageTarget IPrintDocumentPackageTarget;

#endif 	/* __IPrintDocumentPackageTarget_FWD_DEFINED__ */


#ifndef __IPrintDocumentPackageStatusEvent_FWD_DEFINED__
#define __IPrintDocumentPackageStatusEvent_FWD_DEFINED__
typedef interface IPrintDocumentPackageStatusEvent IPrintDocumentPackageStatusEvent;

#endif 	/* __IPrintDocumentPackageStatusEvent_FWD_DEFINED__ */


#ifndef __IPrintDocumentPackageTargetFactory_FWD_DEFINED__
#define __IPrintDocumentPackageTargetFactory_FWD_DEFINED__
typedef interface IPrintDocumentPackageTargetFactory IPrintDocumentPackageTargetFactory;

#endif 	/* __IPrintDocumentPackageTargetFactory_FWD_DEFINED__ */


#ifndef __PrintDocumentPackageTarget_FWD_DEFINED__
#define __PrintDocumentPackageTarget_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrintDocumentPackageTarget PrintDocumentPackageTarget;
#else
typedef struct PrintDocumentPackageTarget PrintDocumentPackageTarget;
#endif /* __cplusplus */

#endif 	/* __PrintDocumentPackageTarget_FWD_DEFINED__ */


#ifndef __PrintDocumentPackageTargetFactory_FWD_DEFINED__
#define __PrintDocumentPackageTargetFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrintDocumentPackageTargetFactory PrintDocumentPackageTargetFactory;
#else
typedef struct PrintDocumentPackageTargetFactory PrintDocumentPackageTargetFactory;
#endif /* __cplusplus */

#endif 	/* __PrintDocumentPackageTargetFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_documenttarget_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if (NTDDI_VERSION >= NTDDI_WIN7)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0000_v0_0_s_ifspec;

#ifndef __IPrintDocumentPackageTarget_INTERFACE_DEFINED__
#define __IPrintDocumentPackageTarget_INTERFACE_DEFINED__

/* interface IPrintDocumentPackageTarget */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPrintDocumentPackageTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1b8efec4-3019-4c27-964e-367202156906")
    IPrintDocumentPackageTarget : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPackageTargetTypes( 
            /* [out] */ __RPC__out UINT32 *targetCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*targetCount) GUID **targetTypes) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPackageTarget( 
            /* [in] */ __RPC__in REFGUID guidTargetType,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintDocumentPackageTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintDocumentPackageTarget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintDocumentPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintDocumentPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageTarget, GetPackageTargetTypes)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPackageTargetTypes )( 
            __RPC__in IPrintDocumentPackageTarget * This,
            /* [out] */ __RPC__out UINT32 *targetCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*targetCount) GUID **targetTypes);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageTarget, GetPackageTarget)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPackageTarget )( 
            __RPC__in IPrintDocumentPackageTarget * This,
            /* [in] */ __RPC__in REFGUID guidTargetType,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvTarget);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageTarget, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPrintDocumentPackageTarget * This);
        
        END_INTERFACE
    } IPrintDocumentPackageTargetVtbl;

    interface IPrintDocumentPackageTarget
    {
        CONST_VTBL struct IPrintDocumentPackageTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintDocumentPackageTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintDocumentPackageTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintDocumentPackageTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintDocumentPackageTarget_GetPackageTargetTypes(This,targetCount,targetTypes)	\
    ( (This)->lpVtbl -> GetPackageTargetTypes(This,targetCount,targetTypes) ) 

#define IPrintDocumentPackageTarget_GetPackageTarget(This,guidTargetType,riid,ppvTarget)	\
    ( (This)->lpVtbl -> GetPackageTarget(This,guidTargetType,riid,ppvTarget) ) 

#define IPrintDocumentPackageTarget_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintDocumentPackageTarget_INTERFACE_DEFINED__ */


#ifndef __IPrintDocumentPackageTarget2_INTERFACE_DEFINED__
#define __IPrintDocumentPackageTarget2_INTERFACE_DEFINED__

/* interface IPrintDocumentPackageTarget2 */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPrintDocumentPackageTarget2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c560298a-535c-48f9-866a-632540660cb4")
    IPrintDocumentPackageTarget2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetIsTargetIppPrinter( 
            /* [out] */ __RPC__out BOOL *isIppPrinter) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetTargetIppPrintDevice( 
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvTarget) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintDocumentPackageTarget2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintDocumentPackageTarget2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintDocumentPackageTarget2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintDocumentPackageTarget2 * This);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageTarget2, GetIsTargetIppPrinter)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetIsTargetIppPrinter )( 
            __RPC__in IPrintDocumentPackageTarget2 * This,
            /* [out] */ __RPC__out BOOL *isIppPrinter);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageTarget2, GetTargetIppPrintDevice)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetTargetIppPrintDevice )( 
            __RPC__in IPrintDocumentPackageTarget2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppvTarget);
        
        END_INTERFACE
    } IPrintDocumentPackageTarget2Vtbl;

    interface IPrintDocumentPackageTarget2
    {
        CONST_VTBL struct IPrintDocumentPackageTarget2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintDocumentPackageTarget2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintDocumentPackageTarget2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintDocumentPackageTarget2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintDocumentPackageTarget2_GetIsTargetIppPrinter(This,isIppPrinter)	\
    ( (This)->lpVtbl -> GetIsTargetIppPrinter(This,isIppPrinter) ) 

#define IPrintDocumentPackageTarget2_GetTargetIppPrintDevice(This,riid,ppvTarget)	\
    ( (This)->lpVtbl -> GetTargetIppPrintDevice(This,riid,ppvTarget) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintDocumentPackageTarget2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_documenttarget_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef /* [v1_enum] */ 
enum PrintDocumentPackageCompletion
    {
        PrintDocumentPackageCompletion_InProgress	= 0,
        PrintDocumentPackageCompletion_Completed	= ( PrintDocumentPackageCompletion_InProgress + 1 ) ,
        PrintDocumentPackageCompletion_Canceled	= ( PrintDocumentPackageCompletion_Completed + 1 ) ,
        PrintDocumentPackageCompletion_Failed	= ( PrintDocumentPackageCompletion_Canceled + 1 ) 
    } 	PrintDocumentPackageCompletion;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_documenttarget_0000_0002_0001
    {
    UINT32 JobId;
    INT32 CurrentDocument;
    INT32 CurrentPage;
    INT32 CurrentPageTotal;
    PrintDocumentPackageCompletion Completion;
    HRESULT PackageStatus;
    } 	PrintDocumentPackageStatus;



extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0002_v0_0_s_ifspec;

#ifndef __IPrintDocumentPackageStatusEvent_INTERFACE_DEFINED__
#define __IPrintDocumentPackageStatusEvent_INTERFACE_DEFINED__

/* interface IPrintDocumentPackageStatusEvent */
/* [nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintDocumentPackageStatusEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ed90c8ad-5c34-4d05-a1ec-0e8a9b3ad7af")
    IPrintDocumentPackageStatusEvent : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE PackageStatusUpdated( 
            /* [in] */ __RPC__in PrintDocumentPackageStatus *packageStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintDocumentPackageStatusEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintDocumentPackageStatusEvent * This,
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
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageStatusEvent, PackageStatusUpdated)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PackageStatusUpdated )( 
            __RPC__in IPrintDocumentPackageStatusEvent * This,
            /* [in] */ __RPC__in PrintDocumentPackageStatus *packageStatus);
        
        END_INTERFACE
    } IPrintDocumentPackageStatusEventVtbl;

    interface IPrintDocumentPackageStatusEvent
    {
        CONST_VTBL struct IPrintDocumentPackageStatusEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintDocumentPackageStatusEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintDocumentPackageStatusEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintDocumentPackageStatusEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintDocumentPackageStatusEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintDocumentPackageStatusEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintDocumentPackageStatusEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintDocumentPackageStatusEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintDocumentPackageStatusEvent_PackageStatusUpdated(This,packageStatus)	\
    ( (This)->lpVtbl -> PackageStatusUpdated(This,packageStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintDocumentPackageStatusEvent_INTERFACE_DEFINED__ */


#ifndef __IPrintDocumentPackageTargetFactory_INTERFACE_DEFINED__
#define __IPrintDocumentPackageTargetFactory_INTERFACE_DEFINED__

/* interface IPrintDocumentPackageTargetFactory */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPrintDocumentPackageTargetFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d2959bf7-b31b-4a3d-9600-712eb1335ba4")
    IPrintDocumentPackageTargetFactory : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateDocumentPackageTargetForPrintJob( 
            /* [string][in] */ __RPC__in_string LPCWSTR printerName,
            /* [string][in] */ __RPC__in_string LPCWSTR jobName,
            /* [in] */ __RPC__in_opt IStream *jobOutputStream,
            /* [in] */ __RPC__in_opt IStream *jobPrintTicketStream,
            /* [out] */ __RPC__deref_out_opt IPrintDocumentPackageTarget **docPackageTarget) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintDocumentPackageTargetFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintDocumentPackageTargetFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintDocumentPackageTargetFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintDocumentPackageTargetFactory * This);
        
        DECLSPEC_XFGVIRT(IPrintDocumentPackageTargetFactory, CreateDocumentPackageTargetForPrintJob)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateDocumentPackageTargetForPrintJob )( 
            __RPC__in IPrintDocumentPackageTargetFactory * This,
            /* [string][in] */ __RPC__in_string LPCWSTR printerName,
            /* [string][in] */ __RPC__in_string LPCWSTR jobName,
            /* [in] */ __RPC__in_opt IStream *jobOutputStream,
            /* [in] */ __RPC__in_opt IStream *jobPrintTicketStream,
            /* [out] */ __RPC__deref_out_opt IPrintDocumentPackageTarget **docPackageTarget);
        
        END_INTERFACE
    } IPrintDocumentPackageTargetFactoryVtbl;

    interface IPrintDocumentPackageTargetFactory
    {
        CONST_VTBL struct IPrintDocumentPackageTargetFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintDocumentPackageTargetFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintDocumentPackageTargetFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintDocumentPackageTargetFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintDocumentPackageTargetFactory_CreateDocumentPackageTargetForPrintJob(This,printerName,jobName,jobOutputStream,jobPrintTicketStream,docPackageTarget)	\
    ( (This)->lpVtbl -> CreateDocumentPackageTargetForPrintJob(This,printerName,jobName,jobOutputStream,jobPrintTicketStream,docPackageTarget) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintDocumentPackageTargetFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_documenttarget_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0004_v0_0_s_ifspec;


#ifndef __PrintDocumentTargetLib_LIBRARY_DEFINED__
#define __PrintDocumentTargetLib_LIBRARY_DEFINED__

/* library PrintDocumentTargetLib */
/* [helpstring][version][uuid] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

EXTERN_C const IID LIBID_PrintDocumentTargetLib;

EXTERN_C const CLSID CLSID_PrintDocumentPackageTarget;

#ifdef __cplusplus

class DECLSPEC_UUID("4842669e-9947-46ea-8ba2-d8cce432c2ca")
PrintDocumentPackageTarget;
#endif

EXTERN_C const CLSID CLSID_PrintDocumentPackageTargetFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("348ef17d-6c81-4982-92b4-ee188a43867a")
PrintDocumentPackageTargetFactory;
#endif
#endif /* __PrintDocumentTargetLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_documenttarget_0000_0005 */
/* [local] */ 

DEFINE_GUID(ID_DOCUMENTPACKAGETARGET_MSXPS,   0x9cae40a8, 0xded1, 0x41c9, 0xa9, 0xfd, 0xd7, 0x35, 0xef, 0x33, 0xae, 0xda);
DEFINE_GUID(ID_DOCUMENTPACKAGETARGET_OPENXPS, 0x0056bb72, 0x8c9c, 0x4612, 0xbd, 0x0f, 0x93, 0x01, 0x2a, 0x87, 0x09, 0x9d);
#endif // (NTDDI_VERSION >= NTDDI_WIN7)
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
DEFINE_GUID(ID_DOCUMENTPACKAGETARGET_OPENXPS_WITH_3D, 0x63dbd720, 0x8b14, 0x4577, 0xb0, 0x74, 0x7b, 0xb1, 0x1b, 0x59, 0x6d, 0x28);
#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_documenttarget_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


