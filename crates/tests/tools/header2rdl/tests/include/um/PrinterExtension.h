

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 500
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

#ifndef __printdriver_h__
#define __printdriver_h__

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

#ifndef __IPrintSchemaElement_FWD_DEFINED__
#define __IPrintSchemaElement_FWD_DEFINED__
typedef interface IPrintSchemaElement IPrintSchemaElement;

#endif 	/* __IPrintSchemaElement_FWD_DEFINED__ */


#ifndef __IPrintSchemaDisplayableElement_FWD_DEFINED__
#define __IPrintSchemaDisplayableElement_FWD_DEFINED__
typedef interface IPrintSchemaDisplayableElement IPrintSchemaDisplayableElement;

#endif 	/* __IPrintSchemaDisplayableElement_FWD_DEFINED__ */


#ifndef __IPrintSchemaOption_FWD_DEFINED__
#define __IPrintSchemaOption_FWD_DEFINED__
typedef interface IPrintSchemaOption IPrintSchemaOption;

#endif 	/* __IPrintSchemaOption_FWD_DEFINED__ */


#ifndef __IPrintSchemaPageMediaSizeOption_FWD_DEFINED__
#define __IPrintSchemaPageMediaSizeOption_FWD_DEFINED__
typedef interface IPrintSchemaPageMediaSizeOption IPrintSchemaPageMediaSizeOption;

#endif 	/* __IPrintSchemaPageMediaSizeOption_FWD_DEFINED__ */


#ifndef __IPrintSchemaNUpOption_FWD_DEFINED__
#define __IPrintSchemaNUpOption_FWD_DEFINED__
typedef interface IPrintSchemaNUpOption IPrintSchemaNUpOption;

#endif 	/* __IPrintSchemaNUpOption_FWD_DEFINED__ */


#ifndef __IPrintSchemaOptionCollection_FWD_DEFINED__
#define __IPrintSchemaOptionCollection_FWD_DEFINED__
typedef interface IPrintSchemaOptionCollection IPrintSchemaOptionCollection;

#endif 	/* __IPrintSchemaOptionCollection_FWD_DEFINED__ */


#ifndef __IPrintSchemaFeature_FWD_DEFINED__
#define __IPrintSchemaFeature_FWD_DEFINED__
typedef interface IPrintSchemaFeature IPrintSchemaFeature;

#endif 	/* __IPrintSchemaFeature_FWD_DEFINED__ */


#ifndef __IPrintSchemaPageImageableSize_FWD_DEFINED__
#define __IPrintSchemaPageImageableSize_FWD_DEFINED__
typedef interface IPrintSchemaPageImageableSize IPrintSchemaPageImageableSize;

#endif 	/* __IPrintSchemaPageImageableSize_FWD_DEFINED__ */


#ifndef __IPrintSchemaParameterDefinition_FWD_DEFINED__
#define __IPrintSchemaParameterDefinition_FWD_DEFINED__
typedef interface IPrintSchemaParameterDefinition IPrintSchemaParameterDefinition;

#endif 	/* __IPrintSchemaParameterDefinition_FWD_DEFINED__ */


#ifndef __IPrintSchemaParameterInitializer_FWD_DEFINED__
#define __IPrintSchemaParameterInitializer_FWD_DEFINED__
typedef interface IPrintSchemaParameterInitializer IPrintSchemaParameterInitializer;

#endif 	/* __IPrintSchemaParameterInitializer_FWD_DEFINED__ */


#ifndef __IPrintSchemaCapabilities_FWD_DEFINED__
#define __IPrintSchemaCapabilities_FWD_DEFINED__
typedef interface IPrintSchemaCapabilities IPrintSchemaCapabilities;

#endif 	/* __IPrintSchemaCapabilities_FWD_DEFINED__ */


#ifndef __IPrintSchemaCapabilities2_FWD_DEFINED__
#define __IPrintSchemaCapabilities2_FWD_DEFINED__
typedef interface IPrintSchemaCapabilities2 IPrintSchemaCapabilities2;

#endif 	/* __IPrintSchemaCapabilities2_FWD_DEFINED__ */


#ifndef __IPrintSchemaAsyncOperation_FWD_DEFINED__
#define __IPrintSchemaAsyncOperation_FWD_DEFINED__
typedef interface IPrintSchemaAsyncOperation IPrintSchemaAsyncOperation;

#endif 	/* __IPrintSchemaAsyncOperation_FWD_DEFINED__ */


#ifndef __IPrintSchemaTicket_FWD_DEFINED__
#define __IPrintSchemaTicket_FWD_DEFINED__
typedef interface IPrintSchemaTicket IPrintSchemaTicket;

#endif 	/* __IPrintSchemaTicket_FWD_DEFINED__ */


#ifndef __IPrintSchemaTicket2_FWD_DEFINED__
#define __IPrintSchemaTicket2_FWD_DEFINED__
typedef interface IPrintSchemaTicket2 IPrintSchemaTicket2;

#endif 	/* __IPrintSchemaTicket2_FWD_DEFINED__ */


#ifndef __IPrintSchemaAsyncOperationEvent_FWD_DEFINED__
#define __IPrintSchemaAsyncOperationEvent_FWD_DEFINED__
typedef interface IPrintSchemaAsyncOperationEvent IPrintSchemaAsyncOperationEvent;

#endif 	/* __IPrintSchemaAsyncOperationEvent_FWD_DEFINED__ */


#ifndef __IPrinterScriptableSequentialStream_FWD_DEFINED__
#define __IPrinterScriptableSequentialStream_FWD_DEFINED__
typedef interface IPrinterScriptableSequentialStream IPrinterScriptableSequentialStream;

#endif 	/* __IPrinterScriptableSequentialStream_FWD_DEFINED__ */


#ifndef __IPrinterScriptableStream_FWD_DEFINED__
#define __IPrinterScriptableStream_FWD_DEFINED__
typedef interface IPrinterScriptableStream IPrinterScriptableStream;

#endif 	/* __IPrinterScriptableStream_FWD_DEFINED__ */


#ifndef __IPrinterPropertyBag_FWD_DEFINED__
#define __IPrinterPropertyBag_FWD_DEFINED__
typedef interface IPrinterPropertyBag IPrinterPropertyBag;

#endif 	/* __IPrinterPropertyBag_FWD_DEFINED__ */


#ifndef __IPrinterScriptablePropertyBag_FWD_DEFINED__
#define __IPrinterScriptablePropertyBag_FWD_DEFINED__
typedef interface IPrinterScriptablePropertyBag IPrinterScriptablePropertyBag;

#endif 	/* __IPrinterScriptablePropertyBag_FWD_DEFINED__ */


#ifndef __IPrinterScriptablePropertyBag2_FWD_DEFINED__
#define __IPrinterScriptablePropertyBag2_FWD_DEFINED__
typedef interface IPrinterScriptablePropertyBag2 IPrinterScriptablePropertyBag2;

#endif 	/* __IPrinterScriptablePropertyBag2_FWD_DEFINED__ */


#ifndef __IPrinterQueue_FWD_DEFINED__
#define __IPrinterQueue_FWD_DEFINED__
typedef interface IPrinterQueue IPrinterQueue;

#endif 	/* __IPrinterQueue_FWD_DEFINED__ */


#ifndef __IPrintJob_FWD_DEFINED__
#define __IPrintJob_FWD_DEFINED__
typedef interface IPrintJob IPrintJob;

#endif 	/* __IPrintJob_FWD_DEFINED__ */


#ifndef __IPrintJobCollection_FWD_DEFINED__
#define __IPrintJobCollection_FWD_DEFINED__
typedef interface IPrintJobCollection IPrintJobCollection;

#endif 	/* __IPrintJobCollection_FWD_DEFINED__ */


#ifndef __IPrinterQueueViewEvent_FWD_DEFINED__
#define __IPrinterQueueViewEvent_FWD_DEFINED__
typedef interface IPrinterQueueViewEvent IPrinterQueueViewEvent;

#endif 	/* __IPrinterQueueViewEvent_FWD_DEFINED__ */


#ifndef __IPrinterQueueView_FWD_DEFINED__
#define __IPrinterQueueView_FWD_DEFINED__
typedef interface IPrinterQueueView IPrinterQueueView;

#endif 	/* __IPrinterQueueView_FWD_DEFINED__ */


#ifndef __IPrinterQueueEvent_FWD_DEFINED__
#define __IPrinterQueueEvent_FWD_DEFINED__
typedef interface IPrinterQueueEvent IPrinterQueueEvent;

#endif 	/* __IPrinterQueueEvent_FWD_DEFINED__ */


#ifndef __IPrinterBidiSetRequestCallback_FWD_DEFINED__
#define __IPrinterBidiSetRequestCallback_FWD_DEFINED__
typedef interface IPrinterBidiSetRequestCallback IPrinterBidiSetRequestCallback;

#endif 	/* __IPrinterBidiSetRequestCallback_FWD_DEFINED__ */


#ifndef __IPrinterExtensionAsyncOperation_FWD_DEFINED__
#define __IPrinterExtensionAsyncOperation_FWD_DEFINED__
typedef interface IPrinterExtensionAsyncOperation IPrinterExtensionAsyncOperation;

#endif 	/* __IPrinterExtensionAsyncOperation_FWD_DEFINED__ */


#ifndef __IPrinterQueue2_FWD_DEFINED__
#define __IPrinterQueue2_FWD_DEFINED__
typedef interface IPrinterQueue2 IPrinterQueue2;

#endif 	/* __IPrinterQueue2_FWD_DEFINED__ */


#ifndef __IPrinterExtensionContext_FWD_DEFINED__
#define __IPrinterExtensionContext_FWD_DEFINED__
typedef interface IPrinterExtensionContext IPrinterExtensionContext;

#endif 	/* __IPrinterExtensionContext_FWD_DEFINED__ */


#ifndef __IPrinterExtensionRequest_FWD_DEFINED__
#define __IPrinterExtensionRequest_FWD_DEFINED__
typedef interface IPrinterExtensionRequest IPrinterExtensionRequest;

#endif 	/* __IPrinterExtensionRequest_FWD_DEFINED__ */


#ifndef __IPrinterExtensionEventArgs_FWD_DEFINED__
#define __IPrinterExtensionEventArgs_FWD_DEFINED__
typedef interface IPrinterExtensionEventArgs IPrinterExtensionEventArgs;

#endif 	/* __IPrinterExtensionEventArgs_FWD_DEFINED__ */


#ifndef __IPrinterExtensionContextCollection_FWD_DEFINED__
#define __IPrinterExtensionContextCollection_FWD_DEFINED__
typedef interface IPrinterExtensionContextCollection IPrinterExtensionContextCollection;

#endif 	/* __IPrinterExtensionContextCollection_FWD_DEFINED__ */


#ifndef __IPrinterExtensionEvent_FWD_DEFINED__
#define __IPrinterExtensionEvent_FWD_DEFINED__
typedef interface IPrinterExtensionEvent IPrinterExtensionEvent;

#endif 	/* __IPrinterExtensionEvent_FWD_DEFINED__ */


#ifndef __IPrinterExtensionManager_FWD_DEFINED__
#define __IPrinterExtensionManager_FWD_DEFINED__
typedef interface IPrinterExtensionManager IPrinterExtensionManager;

#endif 	/* __IPrinterExtensionManager_FWD_DEFINED__ */


#ifndef __IPrinterScriptContext_FWD_DEFINED__
#define __IPrinterScriptContext_FWD_DEFINED__
typedef interface IPrinterScriptContext IPrinterScriptContext;

#endif 	/* __IPrinterScriptContext_FWD_DEFINED__ */


#ifndef __IPrintSchemaElement_FWD_DEFINED__
#define __IPrintSchemaElement_FWD_DEFINED__
typedef interface IPrintSchemaElement IPrintSchemaElement;

#endif 	/* __IPrintSchemaElement_FWD_DEFINED__ */


#ifndef __IPrintSchemaDisplayableElement_FWD_DEFINED__
#define __IPrintSchemaDisplayableElement_FWD_DEFINED__
typedef interface IPrintSchemaDisplayableElement IPrintSchemaDisplayableElement;

#endif 	/* __IPrintSchemaDisplayableElement_FWD_DEFINED__ */


#ifndef __IPrintSchemaOption_FWD_DEFINED__
#define __IPrintSchemaOption_FWD_DEFINED__
typedef interface IPrintSchemaOption IPrintSchemaOption;

#endif 	/* __IPrintSchemaOption_FWD_DEFINED__ */


#ifndef __IPrintSchemaPageMediaSizeOption_FWD_DEFINED__
#define __IPrintSchemaPageMediaSizeOption_FWD_DEFINED__
typedef interface IPrintSchemaPageMediaSizeOption IPrintSchemaPageMediaSizeOption;

#endif 	/* __IPrintSchemaPageMediaSizeOption_FWD_DEFINED__ */


#ifndef __IPrintSchemaNUpOption_FWD_DEFINED__
#define __IPrintSchemaNUpOption_FWD_DEFINED__
typedef interface IPrintSchemaNUpOption IPrintSchemaNUpOption;

#endif 	/* __IPrintSchemaNUpOption_FWD_DEFINED__ */


#ifndef __IPrintSchemaOptionCollection_FWD_DEFINED__
#define __IPrintSchemaOptionCollection_FWD_DEFINED__
typedef interface IPrintSchemaOptionCollection IPrintSchemaOptionCollection;

#endif 	/* __IPrintSchemaOptionCollection_FWD_DEFINED__ */


#ifndef __IPrintSchemaFeature_FWD_DEFINED__
#define __IPrintSchemaFeature_FWD_DEFINED__
typedef interface IPrintSchemaFeature IPrintSchemaFeature;

#endif 	/* __IPrintSchemaFeature_FWD_DEFINED__ */


#ifndef __IPrintSchemaPageImageableSize_FWD_DEFINED__
#define __IPrintSchemaPageImageableSize_FWD_DEFINED__
typedef interface IPrintSchemaPageImageableSize IPrintSchemaPageImageableSize;

#endif 	/* __IPrintSchemaPageImageableSize_FWD_DEFINED__ */


#ifndef __IPrintSchemaParameterDefinition_FWD_DEFINED__
#define __IPrintSchemaParameterDefinition_FWD_DEFINED__
typedef interface IPrintSchemaParameterDefinition IPrintSchemaParameterDefinition;

#endif 	/* __IPrintSchemaParameterDefinition_FWD_DEFINED__ */


#ifndef __IPrintSchemaParameterInitializer_FWD_DEFINED__
#define __IPrintSchemaParameterInitializer_FWD_DEFINED__
typedef interface IPrintSchemaParameterInitializer IPrintSchemaParameterInitializer;

#endif 	/* __IPrintSchemaParameterInitializer_FWD_DEFINED__ */


#ifndef __IPrintSchemaTicket_FWD_DEFINED__
#define __IPrintSchemaTicket_FWD_DEFINED__
typedef interface IPrintSchemaTicket IPrintSchemaTicket;

#endif 	/* __IPrintSchemaTicket_FWD_DEFINED__ */


#ifndef __IPrintSchemaTicket2_FWD_DEFINED__
#define __IPrintSchemaTicket2_FWD_DEFINED__
typedef interface IPrintSchemaTicket2 IPrintSchemaTicket2;

#endif 	/* __IPrintSchemaTicket2_FWD_DEFINED__ */


#ifndef __IPrintSchemaCapabilities_FWD_DEFINED__
#define __IPrintSchemaCapabilities_FWD_DEFINED__
typedef interface IPrintSchemaCapabilities IPrintSchemaCapabilities;

#endif 	/* __IPrintSchemaCapabilities_FWD_DEFINED__ */


#ifndef __IPrintSchemaCapabilities2_FWD_DEFINED__
#define __IPrintSchemaCapabilities2_FWD_DEFINED__
typedef interface IPrintSchemaCapabilities2 IPrintSchemaCapabilities2;

#endif 	/* __IPrintSchemaCapabilities2_FWD_DEFINED__ */


#ifndef __IPrintSchemaAsyncOperation_FWD_DEFINED__
#define __IPrintSchemaAsyncOperation_FWD_DEFINED__
typedef interface IPrintSchemaAsyncOperation IPrintSchemaAsyncOperation;

#endif 	/* __IPrintSchemaAsyncOperation_FWD_DEFINED__ */


#ifndef __IPrintSchemaAsyncOperationEvent_FWD_DEFINED__
#define __IPrintSchemaAsyncOperationEvent_FWD_DEFINED__
typedef interface IPrintSchemaAsyncOperationEvent IPrintSchemaAsyncOperationEvent;

#endif 	/* __IPrintSchemaAsyncOperationEvent_FWD_DEFINED__ */


#ifndef __IPrinterBidiSetRequestCallback_FWD_DEFINED__
#define __IPrinterBidiSetRequestCallback_FWD_DEFINED__
typedef interface IPrinterBidiSetRequestCallback IPrinterBidiSetRequestCallback;

#endif 	/* __IPrinterBidiSetRequestCallback_FWD_DEFINED__ */


#ifndef __IPrinterPropertyBag_FWD_DEFINED__
#define __IPrinterPropertyBag_FWD_DEFINED__
typedef interface IPrinterPropertyBag IPrinterPropertyBag;

#endif 	/* __IPrinterPropertyBag_FWD_DEFINED__ */


#ifndef __IPrinterQueue_FWD_DEFINED__
#define __IPrinterQueue_FWD_DEFINED__
typedef interface IPrinterQueue IPrinterQueue;

#endif 	/* __IPrinterQueue_FWD_DEFINED__ */


#ifndef __IPrinterQueue2_FWD_DEFINED__
#define __IPrinterQueue2_FWD_DEFINED__
typedef interface IPrinterQueue2 IPrinterQueue2;

#endif 	/* __IPrinterQueue2_FWD_DEFINED__ */


#ifndef __IPrinterQueueEvent_FWD_DEFINED__
#define __IPrinterQueueEvent_FWD_DEFINED__
typedef interface IPrinterQueueEvent IPrinterQueueEvent;

#endif 	/* __IPrinterQueueEvent_FWD_DEFINED__ */


#ifndef __IPrinterQueueView_FWD_DEFINED__
#define __IPrinterQueueView_FWD_DEFINED__
typedef interface IPrinterQueueView IPrinterQueueView;

#endif 	/* __IPrinterQueueView_FWD_DEFINED__ */


#ifndef __IPrinterQueueViewEvent_FWD_DEFINED__
#define __IPrinterQueueViewEvent_FWD_DEFINED__
typedef interface IPrinterQueueViewEvent IPrinterQueueViewEvent;

#endif 	/* __IPrinterQueueViewEvent_FWD_DEFINED__ */


#ifndef __IPrinterExtensionContext_FWD_DEFINED__
#define __IPrinterExtensionContext_FWD_DEFINED__
typedef interface IPrinterExtensionContext IPrinterExtensionContext;

#endif 	/* __IPrinterExtensionContext_FWD_DEFINED__ */


#ifndef __PrinterQueue_FWD_DEFINED__
#define __PrinterQueue_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrinterQueue PrinterQueue;
#else
typedef struct PrinterQueue PrinterQueue;
#endif /* __cplusplus */

#endif 	/* __PrinterQueue_FWD_DEFINED__ */


#ifndef __PrinterQueueView_FWD_DEFINED__
#define __PrinterQueueView_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrinterQueueView PrinterQueueView;
#else
typedef struct PrinterQueueView PrinterQueueView;
#endif /* __cplusplus */

#endif 	/* __PrinterQueueView_FWD_DEFINED__ */


#ifndef __PrintSchemaAsyncOperation_FWD_DEFINED__
#define __PrintSchemaAsyncOperation_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrintSchemaAsyncOperation PrintSchemaAsyncOperation;
#else
typedef struct PrintSchemaAsyncOperation PrintSchemaAsyncOperation;
#endif /* __cplusplus */

#endif 	/* __PrintSchemaAsyncOperation_FWD_DEFINED__ */


#ifndef __IPrinterExtensionAsyncOperation_FWD_DEFINED__
#define __IPrinterExtensionAsyncOperation_FWD_DEFINED__
typedef interface IPrinterExtensionAsyncOperation IPrinterExtensionAsyncOperation;

#endif 	/* __IPrinterExtensionAsyncOperation_FWD_DEFINED__ */


#ifndef __IPrinterExtensionContextCollection_FWD_DEFINED__
#define __IPrinterExtensionContextCollection_FWD_DEFINED__
typedef interface IPrinterExtensionContextCollection IPrinterExtensionContextCollection;

#endif 	/* __IPrinterExtensionContextCollection_FWD_DEFINED__ */


#ifndef __IPrinterExtensionRequest_FWD_DEFINED__
#define __IPrinterExtensionRequest_FWD_DEFINED__
typedef interface IPrinterExtensionRequest IPrinterExtensionRequest;

#endif 	/* __IPrinterExtensionRequest_FWD_DEFINED__ */


#ifndef __IPrinterExtensionEventArgs_FWD_DEFINED__
#define __IPrinterExtensionEventArgs_FWD_DEFINED__
typedef interface IPrinterExtensionEventArgs IPrinterExtensionEventArgs;

#endif 	/* __IPrinterExtensionEventArgs_FWD_DEFINED__ */


#ifndef __IPrinterExtensionEvent_FWD_DEFINED__
#define __IPrinterExtensionEvent_FWD_DEFINED__
typedef interface IPrinterExtensionEvent IPrinterExtensionEvent;

#endif 	/* __IPrinterExtensionEvent_FWD_DEFINED__ */


#ifndef __IPrinterExtensionManager_FWD_DEFINED__
#define __IPrinterExtensionManager_FWD_DEFINED__
typedef interface IPrinterExtensionManager IPrinterExtensionManager;

#endif 	/* __IPrinterExtensionManager_FWD_DEFINED__ */


#ifndef __IPrinterScriptContext_FWD_DEFINED__
#define __IPrinterScriptContext_FWD_DEFINED__
typedef interface IPrinterScriptContext IPrinterScriptContext;

#endif 	/* __IPrinterScriptContext_FWD_DEFINED__ */


#ifndef __IPrinterScriptablePropertyBag_FWD_DEFINED__
#define __IPrinterScriptablePropertyBag_FWD_DEFINED__
typedef interface IPrinterScriptablePropertyBag IPrinterScriptablePropertyBag;

#endif 	/* __IPrinterScriptablePropertyBag_FWD_DEFINED__ */


#ifndef __IPrinterScriptablePropertyBag2_FWD_DEFINED__
#define __IPrinterScriptablePropertyBag2_FWD_DEFINED__
typedef interface IPrinterScriptablePropertyBag2 IPrinterScriptablePropertyBag2;

#endif 	/* __IPrinterScriptablePropertyBag2_FWD_DEFINED__ */


#ifndef __IPrinterScriptableSequentialStream_FWD_DEFINED__
#define __IPrinterScriptableSequentialStream_FWD_DEFINED__
typedef interface IPrinterScriptableSequentialStream IPrinterScriptableSequentialStream;

#endif 	/* __IPrinterScriptableSequentialStream_FWD_DEFINED__ */


#ifndef __IPrinterScriptableStream_FWD_DEFINED__
#define __IPrinterScriptableStream_FWD_DEFINED__
typedef interface IPrinterScriptableStream IPrinterScriptableStream;

#endif 	/* __IPrinterScriptableStream_FWD_DEFINED__ */


#ifndef __PrinterExtensionManager_FWD_DEFINED__
#define __PrinterExtensionManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class PrinterExtensionManager PrinterExtensionManager;
#else
typedef struct PrinterExtensionManager PrinterExtensionManager;
#endif /* __cplusplus */

#endif 	/* __PrinterExtensionManager_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_printdriver_0000_0000 */
/* [local] */ 

//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Application Family
#pragma endregion 
#pragma region Desktop Family
#pragma endregion 
#pragma region Application Family
#pragma endregion 
#pragma region Desktop Family
#pragma endregion 
#pragma region Application Family
#pragma endregion 
#pragma region Desktop Family
#pragma endregion 
#pragma region Application Family
#pragma endregion 
#define PRINTSCHEMA_KEYWORDS_NAMESPACE_URI  "http://schemas.microsoft.com/windows/2003/08/printing/printschemakeywords"
#define PRINTSCHEMA_KEYWORDSV11_NAMESPACE_URI  L"http://schemas.microsoft.com/windows/2013/05/printing/printschemakeywordsv11"
#if (NTDDI_VERSION >= NTDDI_WIN8)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0000_v0_0_s_ifspec;

#ifndef __IPrintSchemaElement_INTERFACE_DEFINED__
#define __IPrintSchemaElement_INTERFACE_DEFINED__

/* interface IPrintSchemaElement */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("724c1646-e64b-4bbf-8eb4-d45e4fd580da")
    IPrintSchemaElement : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_XmlNode( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_NamespaceUri( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaElement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaElement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaElement * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaElement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaElement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaElement * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaElement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaElement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        END_INTERFACE
    } IPrintSchemaElementVtbl;

    interface IPrintSchemaElement
    {
        CONST_VTBL struct IPrintSchemaElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaElement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaElement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaElement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaElement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaElement_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaElement_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaElement_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaElement_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaDisplayableElement_INTERFACE_DEFINED__
#define __IPrintSchemaDisplayableElement_INTERFACE_DEFINED__

/* interface IPrintSchemaDisplayableElement */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaDisplayableElement;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("af45af49-d6aa-407d-bf87-3912236e9d94")
    IPrintSchemaDisplayableElement : public IPrintSchemaElement
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaDisplayableElementVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaDisplayableElement * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaDisplayableElement * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaDisplayableElement * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaDisplayableElement, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPrintSchemaDisplayableElement * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        END_INTERFACE
    } IPrintSchemaDisplayableElementVtbl;

    interface IPrintSchemaDisplayableElement
    {
        CONST_VTBL struct IPrintSchemaDisplayableElementVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaDisplayableElement_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaDisplayableElement_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaDisplayableElement_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaDisplayableElement_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaDisplayableElement_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaDisplayableElement_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaDisplayableElement_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaDisplayableElement_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaDisplayableElement_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaDisplayableElement_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaDisplayableElement_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaDisplayableElement_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0002 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagPrintSchemaConstrainedSetting
    {
        PrintSchemaConstrainedSetting_None	= 0,
        PrintSchemaConstrainedSetting_PrintTicket	= ( PrintSchemaConstrainedSetting_None + 1 ) ,
        PrintSchemaConstrainedSetting_Admin	= ( PrintSchemaConstrainedSetting_PrintTicket + 1 ) ,
        PrintSchemaConstrainedSetting_Device	= ( PrintSchemaConstrainedSetting_Admin + 1 ) 
    } 	PrintSchemaConstrainedSetting;



extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0002_v0_0_s_ifspec;

#ifndef __IPrintSchemaOption_INTERFACE_DEFINED__
#define __IPrintSchemaOption_INTERFACE_DEFINED__

/* interface IPrintSchemaOption */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaOption;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("66bb2f51-5844-4997-8d70-4b7cc221cf92")
    IPrintSchemaOption : public IPrintSchemaDisplayableElement
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Selected( 
            /* [retval][out] */ __RPC__out BOOL *pbIsSelected) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Constrained( 
            /* [retval][out] */ __RPC__out PrintSchemaConstrainedSetting *pSetting) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetPropertyValue( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlValueNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaOptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaOption * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaOption * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaOption * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaOption * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaOption * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaOption * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaOption * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaOption * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaDisplayableElement, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPrintSchemaOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, get_Selected)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Selected )( 
            __RPC__in IPrintSchemaOption * This,
            /* [retval][out] */ __RPC__out BOOL *pbIsSelected);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, get_Constrained)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Constrained )( 
            __RPC__in IPrintSchemaOption * This,
            /* [retval][out] */ __RPC__out PrintSchemaConstrainedSetting *pSetting);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, GetPropertyValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IPrintSchemaOption * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlValueNode);
        
        END_INTERFACE
    } IPrintSchemaOptionVtbl;

    interface IPrintSchemaOption
    {
        CONST_VTBL struct IPrintSchemaOptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaOption_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaOption_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaOption_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaOption_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaOption_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaOption_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaOption_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaOption_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaOption_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaOption_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaOption_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 


#define IPrintSchemaOption_get_Selected(This,pbIsSelected)	\
    ( (This)->lpVtbl -> get_Selected(This,pbIsSelected) ) 

#define IPrintSchemaOption_get_Constrained(This,pSetting)	\
    ( (This)->lpVtbl -> get_Constrained(This,pSetting) ) 

#define IPrintSchemaOption_GetPropertyValue(This,bstrName,bstrNamespaceUri,ppXmlValueNode)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,bstrName,bstrNamespaceUri,ppXmlValueNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaOption_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaPageMediaSizeOption_INTERFACE_DEFINED__
#define __IPrintSchemaPageMediaSizeOption_INTERFACE_DEFINED__

/* interface IPrintSchemaPageMediaSizeOption */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaPageMediaSizeOption;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68746729-f493-4830-a10f-69028774605d")
    IPrintSchemaPageMediaSizeOption : public IPrintSchemaOption
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_WidthInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulWidth) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_HeightInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulHeight) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaPageMediaSizeOptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaPageMediaSizeOption * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaDisplayableElement, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, get_Selected)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Selected )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__out BOOL *pbIsSelected);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, get_Constrained)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Constrained )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__out PrintSchemaConstrainedSetting *pSetting);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, GetPropertyValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlValueNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageMediaSizeOption, get_WidthInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WidthInMicrons )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__out ULONG *pulWidth);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageMediaSizeOption, get_HeightInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HeightInMicrons )( 
            __RPC__in IPrintSchemaPageMediaSizeOption * This,
            /* [retval][out] */ __RPC__out ULONG *pulHeight);
        
        END_INTERFACE
    } IPrintSchemaPageMediaSizeOptionVtbl;

    interface IPrintSchemaPageMediaSizeOption
    {
        CONST_VTBL struct IPrintSchemaPageMediaSizeOptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaPageMediaSizeOption_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaPageMediaSizeOption_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaPageMediaSizeOption_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaPageMediaSizeOption_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaPageMediaSizeOption_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaPageMediaSizeOption_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaPageMediaSizeOption_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaPageMediaSizeOption_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaPageMediaSizeOption_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaPageMediaSizeOption_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaPageMediaSizeOption_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 


#define IPrintSchemaPageMediaSizeOption_get_Selected(This,pbIsSelected)	\
    ( (This)->lpVtbl -> get_Selected(This,pbIsSelected) ) 

#define IPrintSchemaPageMediaSizeOption_get_Constrained(This,pSetting)	\
    ( (This)->lpVtbl -> get_Constrained(This,pSetting) ) 

#define IPrintSchemaPageMediaSizeOption_GetPropertyValue(This,bstrName,bstrNamespaceUri,ppXmlValueNode)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,bstrName,bstrNamespaceUri,ppXmlValueNode) ) 


#define IPrintSchemaPageMediaSizeOption_get_WidthInMicrons(This,pulWidth)	\
    ( (This)->lpVtbl -> get_WidthInMicrons(This,pulWidth) ) 

#define IPrintSchemaPageMediaSizeOption_get_HeightInMicrons(This,pulHeight)	\
    ( (This)->lpVtbl -> get_HeightInMicrons(This,pulHeight) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaPageMediaSizeOption_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaNUpOption_INTERFACE_DEFINED__
#define __IPrintSchemaNUpOption_INTERFACE_DEFINED__

/* interface IPrintSchemaNUpOption */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaNUpOption;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1f6342f2-d848-42e3-8995-c10a9ef9a3ba")
    IPrintSchemaNUpOption : public IPrintSchemaOption
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PagesPerSheet( 
            /* [retval][out] */ __RPC__out ULONG *pulPagesPerSheet) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaNUpOptionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaNUpOption * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaNUpOption * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaNUpOption * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaDisplayableElement, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, get_Selected)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Selected )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__out BOOL *pbIsSelected);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, get_Constrained)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Constrained )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__out PrintSchemaConstrainedSetting *pSetting);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOption, GetPropertyValue)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlValueNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaNUpOption, get_PagesPerSheet)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PagesPerSheet )( 
            __RPC__in IPrintSchemaNUpOption * This,
            /* [retval][out] */ __RPC__out ULONG *pulPagesPerSheet);
        
        END_INTERFACE
    } IPrintSchemaNUpOptionVtbl;

    interface IPrintSchemaNUpOption
    {
        CONST_VTBL struct IPrintSchemaNUpOptionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaNUpOption_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaNUpOption_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaNUpOption_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaNUpOption_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaNUpOption_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaNUpOption_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaNUpOption_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaNUpOption_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaNUpOption_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaNUpOption_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaNUpOption_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 


#define IPrintSchemaNUpOption_get_Selected(This,pbIsSelected)	\
    ( (This)->lpVtbl -> get_Selected(This,pbIsSelected) ) 

#define IPrintSchemaNUpOption_get_Constrained(This,pSetting)	\
    ( (This)->lpVtbl -> get_Constrained(This,pSetting) ) 

#define IPrintSchemaNUpOption_GetPropertyValue(This,bstrName,bstrNamespaceUri,ppXmlValueNode)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,bstrName,bstrNamespaceUri,ppXmlValueNode) ) 


#define IPrintSchemaNUpOption_get_PagesPerSheet(This,pulPagesPerSheet)	\
    ( (This)->lpVtbl -> get_PagesPerSheet(This,pulPagesPerSheet) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaNUpOption_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0005 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum tagPrintSchemaSelectionType
    {
        PrintSchemaSelectionType_PickOne	= 0,
        PrintSchemaSelectionType_PickMany	= ( PrintSchemaSelectionType_PickOne + 1 ) 
    } 	PrintSchemaSelectionType;



extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0005_v0_0_s_ifspec;

#ifndef __IPrintSchemaOptionCollection_INTERFACE_DEFINED__
#define __IPrintSchemaOptionCollection_INTERFACE_DEFINED__

/* interface IPrintSchemaOptionCollection */
/* [custom][unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaOptionCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("baecb0bd-a946-4771-bc30-e8b24f8d45c1")
    IPrintSchemaOptionCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *pulCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaOptionCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaOptionCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaOptionCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaOptionCollection * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaOptionCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [retval][out] */ __RPC__out ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOptionCollection, GetAt)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption);
        
        DECLSPEC_XFGVIRT(IPrintSchemaOptionCollection, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IPrintSchemaOptionCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        END_INTERFACE
    } IPrintSchemaOptionCollectionVtbl;

    interface IPrintSchemaOptionCollection
    {
        CONST_VTBL struct IPrintSchemaOptionCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaOptionCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaOptionCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaOptionCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaOptionCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaOptionCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaOptionCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaOptionCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaOptionCollection_get_Count(This,pulCount)	\
    ( (This)->lpVtbl -> get_Count(This,pulCount) ) 

#define IPrintSchemaOptionCollection_GetAt(This,ulIndex,ppOption)	\
    ( (This)->lpVtbl -> GetAt(This,ulIndex,ppOption) ) 

#define IPrintSchemaOptionCollection_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaOptionCollection_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaFeature_INTERFACE_DEFINED__
#define __IPrintSchemaFeature_INTERFACE_DEFINED__

/* interface IPrintSchemaFeature */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaFeature;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ef189461-5d62-4626-8e57-ff83583c4826")
    IPrintSchemaFeature : public IPrintSchemaDisplayableElement
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SelectedOption( 
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_SelectedOption( 
            /* [in] */ __RPC__in_opt IPrintSchemaOption *pOption) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SelectionType( 
            /* [retval][out] */ __RPC__out PrintSchemaSelectionType *pSelectionType) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetOption( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DisplayUI( 
            /* [retval][out] */ __RPC__out BOOL *pbShow) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaFeatureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaFeature * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaFeature * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaFeature * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaDisplayableElement, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaFeature, get_SelectedOption)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SelectedOption )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption);
        
        DECLSPEC_XFGVIRT(IPrintSchemaFeature, put_SelectedOption)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SelectedOption )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [in] */ __RPC__in_opt IPrintSchemaOption *pOption);
        
        DECLSPEC_XFGVIRT(IPrintSchemaFeature, get_SelectionType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SelectionType )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__out PrintSchemaSelectionType *pSelectionType);
        
        DECLSPEC_XFGVIRT(IPrintSchemaFeature, GetOption)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetOption )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption);
        
        DECLSPEC_XFGVIRT(IPrintSchemaFeature, get_DisplayUI)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayUI )( 
            __RPC__in IPrintSchemaFeature * This,
            /* [retval][out] */ __RPC__out BOOL *pbShow);
        
        END_INTERFACE
    } IPrintSchemaFeatureVtbl;

    interface IPrintSchemaFeature
    {
        CONST_VTBL struct IPrintSchemaFeatureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaFeature_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaFeature_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaFeature_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaFeature_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaFeature_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaFeature_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaFeature_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaFeature_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaFeature_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaFeature_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaFeature_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 


#define IPrintSchemaFeature_get_SelectedOption(This,ppOption)	\
    ( (This)->lpVtbl -> get_SelectedOption(This,ppOption) ) 

#define IPrintSchemaFeature_put_SelectedOption(This,pOption)	\
    ( (This)->lpVtbl -> put_SelectedOption(This,pOption) ) 

#define IPrintSchemaFeature_get_SelectionType(This,pSelectionType)	\
    ( (This)->lpVtbl -> get_SelectionType(This,pSelectionType) ) 

#define IPrintSchemaFeature_GetOption(This,bstrName,bstrNamespaceUri,ppOption)	\
    ( (This)->lpVtbl -> GetOption(This,bstrName,bstrNamespaceUri,ppOption) ) 

#define IPrintSchemaFeature_get_DisplayUI(This,pbShow)	\
    ( (This)->lpVtbl -> get_DisplayUI(This,pbShow) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaFeature_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaPageImageableSize_INTERFACE_DEFINED__
#define __IPrintSchemaPageImageableSize_INTERFACE_DEFINED__

/* interface IPrintSchemaPageImageableSize */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaPageImageableSize;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7c85bf5e-dc7c-4f61-839b-4107e1c9b68e")
    IPrintSchemaPageImageableSize : public IPrintSchemaElement
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageableSizeWidthInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulImageableSizeWidth) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ImageableSizeHeightInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulImageableSizeHeight) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OriginWidthInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulOriginWidth) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_OriginHeightInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulOriginHeight) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtentWidthInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulExtentWidth) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtentHeightInMicrons( 
            /* [retval][out] */ __RPC__out ULONG *pulExtentHeight) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaPageImageableSizeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaPageImageableSize * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaPageImageableSize * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaPageImageableSize * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageImageableSize, get_ImageableSizeWidthInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageableSizeWidthInMicrons )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__out ULONG *pulImageableSizeWidth);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageImageableSize, get_ImageableSizeHeightInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ImageableSizeHeightInMicrons )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__out ULONG *pulImageableSizeHeight);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageImageableSize, get_OriginWidthInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OriginWidthInMicrons )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__out ULONG *pulOriginWidth);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageImageableSize, get_OriginHeightInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_OriginHeightInMicrons )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__out ULONG *pulOriginHeight);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageImageableSize, get_ExtentWidthInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtentWidthInMicrons )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__out ULONG *pulExtentWidth);
        
        DECLSPEC_XFGVIRT(IPrintSchemaPageImageableSize, get_ExtentHeightInMicrons)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtentHeightInMicrons )( 
            __RPC__in IPrintSchemaPageImageableSize * This,
            /* [retval][out] */ __RPC__out ULONG *pulExtentHeight);
        
        END_INTERFACE
    } IPrintSchemaPageImageableSizeVtbl;

    interface IPrintSchemaPageImageableSize
    {
        CONST_VTBL struct IPrintSchemaPageImageableSizeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaPageImageableSize_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaPageImageableSize_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaPageImageableSize_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaPageImageableSize_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaPageImageableSize_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaPageImageableSize_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaPageImageableSize_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaPageImageableSize_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaPageImageableSize_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaPageImageableSize_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaPageImageableSize_get_ImageableSizeWidthInMicrons(This,pulImageableSizeWidth)	\
    ( (This)->lpVtbl -> get_ImageableSizeWidthInMicrons(This,pulImageableSizeWidth) ) 

#define IPrintSchemaPageImageableSize_get_ImageableSizeHeightInMicrons(This,pulImageableSizeHeight)	\
    ( (This)->lpVtbl -> get_ImageableSizeHeightInMicrons(This,pulImageableSizeHeight) ) 

#define IPrintSchemaPageImageableSize_get_OriginWidthInMicrons(This,pulOriginWidth)	\
    ( (This)->lpVtbl -> get_OriginWidthInMicrons(This,pulOriginWidth) ) 

#define IPrintSchemaPageImageableSize_get_OriginHeightInMicrons(This,pulOriginHeight)	\
    ( (This)->lpVtbl -> get_OriginHeightInMicrons(This,pulOriginHeight) ) 

#define IPrintSchemaPageImageableSize_get_ExtentWidthInMicrons(This,pulExtentWidth)	\
    ( (This)->lpVtbl -> get_ExtentWidthInMicrons(This,pulExtentWidth) ) 

#define IPrintSchemaPageImageableSize_get_ExtentHeightInMicrons(This,pulExtentHeight)	\
    ( (This)->lpVtbl -> get_ExtentHeightInMicrons(This,pulExtentHeight) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaPageImageableSize_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0008 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef /* [v1_enum] */ 
enum tagPrintSchemaParameterDataType
    {
        PrintSchemaParameterDataType_Integer	= 0,
        PrintSchemaParameterDataType_NumericString	= ( PrintSchemaParameterDataType_Integer + 1 ) ,
        PrintSchemaParameterDataType_String	= ( PrintSchemaParameterDataType_NumericString + 1 ) 
    } 	PrintSchemaParameterDataType;



extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0008_v0_0_s_ifspec;

#ifndef __IPrintSchemaParameterDefinition_INTERFACE_DEFINED__
#define __IPrintSchemaParameterDefinition_INTERFACE_DEFINED__

/* interface IPrintSchemaParameterDefinition */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaParameterDefinition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b5ade81e-0e61-4fe1-81c6-c333e4ffe0f1")
    IPrintSchemaParameterDefinition : public IPrintSchemaDisplayableElement
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserInputRequired( 
            /* [retval][out] */ __RPC__out BOOL *pbIsRequired) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UnitType( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUnitType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DataType( 
            /* [retval][out] */ __RPC__out PrintSchemaParameterDataType *pDataType) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_RangeMin( 
            /* [retval][out] */ __RPC__out INT *pRangeMin) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_RangeMax( 
            /* [retval][out] */ __RPC__out INT *pRangeMax) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaParameterDefinitionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaParameterDefinition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaParameterDefinition * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaParameterDefinition * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaDisplayableElement, get_DisplayName)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DisplayName )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrDisplayName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterDefinition, get_UserInputRequired)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserInputRequired )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__out BOOL *pbIsRequired);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterDefinition, get_UnitType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UnitType )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrUnitType);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterDefinition, get_DataType)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DataType )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__out PrintSchemaParameterDataType *pDataType);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterDefinition, get_RangeMin)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RangeMin )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__out INT *pRangeMin);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterDefinition, get_RangeMax)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RangeMax )( 
            __RPC__in IPrintSchemaParameterDefinition * This,
            /* [retval][out] */ __RPC__out INT *pRangeMax);
        
        END_INTERFACE
    } IPrintSchemaParameterDefinitionVtbl;

    interface IPrintSchemaParameterDefinition
    {
        CONST_VTBL struct IPrintSchemaParameterDefinitionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaParameterDefinition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaParameterDefinition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaParameterDefinition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaParameterDefinition_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaParameterDefinition_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaParameterDefinition_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaParameterDefinition_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaParameterDefinition_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaParameterDefinition_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaParameterDefinition_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaParameterDefinition_get_DisplayName(This,pbstrDisplayName)	\
    ( (This)->lpVtbl -> get_DisplayName(This,pbstrDisplayName) ) 


#define IPrintSchemaParameterDefinition_get_UserInputRequired(This,pbIsRequired)	\
    ( (This)->lpVtbl -> get_UserInputRequired(This,pbIsRequired) ) 

#define IPrintSchemaParameterDefinition_get_UnitType(This,pbstrUnitType)	\
    ( (This)->lpVtbl -> get_UnitType(This,pbstrUnitType) ) 

#define IPrintSchemaParameterDefinition_get_DataType(This,pDataType)	\
    ( (This)->lpVtbl -> get_DataType(This,pDataType) ) 

#define IPrintSchemaParameterDefinition_get_RangeMin(This,pRangeMin)	\
    ( (This)->lpVtbl -> get_RangeMin(This,pRangeMin) ) 

#define IPrintSchemaParameterDefinition_get_RangeMax(This,pRangeMax)	\
    ( (This)->lpVtbl -> get_RangeMax(This,pRangeMax) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaParameterDefinition_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaParameterInitializer_INTERFACE_DEFINED__
#define __IPrintSchemaParameterInitializer_INTERFACE_DEFINED__

/* interface IPrintSchemaParameterInitializer */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaParameterInitializer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("52027082-0b74-4648-9564-828cc6cb656c")
    IPrintSchemaParameterInitializer : public IPrintSchemaElement
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out VARIANT *pVar) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_Value( 
            /* [in] */ __RPC__in VARIANT *pVar) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaParameterInitializerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaParameterInitializer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaParameterInitializer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaParameterInitializer * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterInitializer, get_Value)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [retval][out] */ __RPC__out VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(IPrintSchemaParameterInitializer, put_Value)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Value )( 
            __RPC__in IPrintSchemaParameterInitializer * This,
            /* [in] */ __RPC__in VARIANT *pVar);
        
        END_INTERFACE
    } IPrintSchemaParameterInitializerVtbl;

    interface IPrintSchemaParameterInitializer
    {
        CONST_VTBL struct IPrintSchemaParameterInitializerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaParameterInitializer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaParameterInitializer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaParameterInitializer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaParameterInitializer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaParameterInitializer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaParameterInitializer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaParameterInitializer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaParameterInitializer_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaParameterInitializer_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaParameterInitializer_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaParameterInitializer_get_Value(This,pVar)	\
    ( (This)->lpVtbl -> get_Value(This,pVar) ) 

#define IPrintSchemaParameterInitializer_put_Value(This,pVar)	\
    ( (This)->lpVtbl -> put_Value(This,pVar) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaParameterInitializer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0010 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0010_v0_0_s_ifspec;

#ifndef __IPrintSchemaCapabilities_INTERFACE_DEFINED__
#define __IPrintSchemaCapabilities_INTERFACE_DEFINED__

/* interface IPrintSchemaCapabilities */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5a577640-501d-4927-bcd0-5ef57a7ed175")
    IPrintSchemaCapabilities : public IPrintSchemaElement
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetFeatureByKeyName( 
            /* [in] */ __RPC__in BSTR bstrKeyName,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetFeature( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PageImageableSize( 
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaPageImageableSize **ppPageImageableSize) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_JobCopiesAllDocumentsMinValue( 
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocumentsMinValue) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_JobCopiesAllDocumentsMaxValue( 
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocumentsMaxValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetSelectedOptionInPrintTicket( 
            /* [in] */ __RPC__in_opt IPrintSchemaFeature *pFeature,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetOptions( 
            /* [in] */ __RPC__in_opt IPrintSchemaFeature *pFeature,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOptionCollection **ppOptionCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaCapabilities * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaCapabilities * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetFeatureByKeyName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeatureByKeyName )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ __RPC__in BSTR bstrKeyName,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeature )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, get_PageImageableSize)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PageImageableSize )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaPageImageableSize **ppPageImageableSize);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, get_JobCopiesAllDocumentsMinValue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobCopiesAllDocumentsMinValue )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocumentsMinValue);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, get_JobCopiesAllDocumentsMaxValue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobCopiesAllDocumentsMaxValue )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocumentsMaxValue);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetSelectedOptionInPrintTicket)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetSelectedOptionInPrintTicket )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ __RPC__in_opt IPrintSchemaFeature *pFeature,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetOptions )( 
            __RPC__in IPrintSchemaCapabilities * This,
            /* [in] */ __RPC__in_opt IPrintSchemaFeature *pFeature,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOptionCollection **ppOptionCollection);
        
        END_INTERFACE
    } IPrintSchemaCapabilitiesVtbl;

    interface IPrintSchemaCapabilities
    {
        CONST_VTBL struct IPrintSchemaCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaCapabilities_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaCapabilities_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaCapabilities_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaCapabilities_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaCapabilities_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaCapabilities_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaCapabilities_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaCapabilities_GetFeatureByKeyName(This,bstrKeyName,ppFeature)	\
    ( (This)->lpVtbl -> GetFeatureByKeyName(This,bstrKeyName,ppFeature) ) 

#define IPrintSchemaCapabilities_GetFeature(This,bstrName,bstrNamespaceUri,ppFeature)	\
    ( (This)->lpVtbl -> GetFeature(This,bstrName,bstrNamespaceUri,ppFeature) ) 

#define IPrintSchemaCapabilities_get_PageImageableSize(This,ppPageImageableSize)	\
    ( (This)->lpVtbl -> get_PageImageableSize(This,ppPageImageableSize) ) 

#define IPrintSchemaCapabilities_get_JobCopiesAllDocumentsMinValue(This,pulJobCopiesAllDocumentsMinValue)	\
    ( (This)->lpVtbl -> get_JobCopiesAllDocumentsMinValue(This,pulJobCopiesAllDocumentsMinValue) ) 

#define IPrintSchemaCapabilities_get_JobCopiesAllDocumentsMaxValue(This,pulJobCopiesAllDocumentsMaxValue)	\
    ( (This)->lpVtbl -> get_JobCopiesAllDocumentsMaxValue(This,pulJobCopiesAllDocumentsMaxValue) ) 

#define IPrintSchemaCapabilities_GetSelectedOptionInPrintTicket(This,pFeature,ppOption)	\
    ( (This)->lpVtbl -> GetSelectedOptionInPrintTicket(This,pFeature,ppOption) ) 

#define IPrintSchemaCapabilities_GetOptions(This,pFeature,ppOptionCollection)	\
    ( (This)->lpVtbl -> GetOptions(This,pFeature,ppOptionCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaCapabilities_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0011 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0011_v0_0_s_ifspec;

#ifndef __IPrintSchemaCapabilities2_INTERFACE_DEFINED__
#define __IPrintSchemaCapabilities2_INTERFACE_DEFINED__

/* interface IPrintSchemaCapabilities2 */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaCapabilities2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b58845f4-9970-4d87-a636-169fb82ed642")
    IPrintSchemaCapabilities2 : public IPrintSchemaCapabilities
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetParameterDefinition( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaParameterDefinition **ppParameterDefinition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaCapabilities2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaCapabilities2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaCapabilities2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaCapabilities2 * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetFeatureByKeyName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeatureByKeyName )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in BSTR bstrKeyName,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeature )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, get_PageImageableSize)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PageImageableSize )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaPageImageableSize **ppPageImageableSize);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, get_JobCopiesAllDocumentsMinValue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobCopiesAllDocumentsMinValue )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocumentsMinValue);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, get_JobCopiesAllDocumentsMaxValue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobCopiesAllDocumentsMaxValue )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocumentsMaxValue);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetSelectedOptionInPrintTicket)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetSelectedOptionInPrintTicket )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in_opt IPrintSchemaFeature *pFeature,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOption **ppOption);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities, GetOptions)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetOptions )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in_opt IPrintSchemaFeature *pFeature,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaOptionCollection **ppOptionCollection);
        
        DECLSPEC_XFGVIRT(IPrintSchemaCapabilities2, GetParameterDefinition)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetParameterDefinition )( 
            __RPC__in IPrintSchemaCapabilities2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaParameterDefinition **ppParameterDefinition);
        
        END_INTERFACE
    } IPrintSchemaCapabilities2Vtbl;

    interface IPrintSchemaCapabilities2
    {
        CONST_VTBL struct IPrintSchemaCapabilities2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaCapabilities2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaCapabilities2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaCapabilities2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaCapabilities2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaCapabilities2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaCapabilities2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaCapabilities2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaCapabilities2_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaCapabilities2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaCapabilities2_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaCapabilities2_GetFeatureByKeyName(This,bstrKeyName,ppFeature)	\
    ( (This)->lpVtbl -> GetFeatureByKeyName(This,bstrKeyName,ppFeature) ) 

#define IPrintSchemaCapabilities2_GetFeature(This,bstrName,bstrNamespaceUri,ppFeature)	\
    ( (This)->lpVtbl -> GetFeature(This,bstrName,bstrNamespaceUri,ppFeature) ) 

#define IPrintSchemaCapabilities2_get_PageImageableSize(This,ppPageImageableSize)	\
    ( (This)->lpVtbl -> get_PageImageableSize(This,ppPageImageableSize) ) 

#define IPrintSchemaCapabilities2_get_JobCopiesAllDocumentsMinValue(This,pulJobCopiesAllDocumentsMinValue)	\
    ( (This)->lpVtbl -> get_JobCopiesAllDocumentsMinValue(This,pulJobCopiesAllDocumentsMinValue) ) 

#define IPrintSchemaCapabilities2_get_JobCopiesAllDocumentsMaxValue(This,pulJobCopiesAllDocumentsMaxValue)	\
    ( (This)->lpVtbl -> get_JobCopiesAllDocumentsMaxValue(This,pulJobCopiesAllDocumentsMaxValue) ) 

#define IPrintSchemaCapabilities2_GetSelectedOptionInPrintTicket(This,pFeature,ppOption)	\
    ( (This)->lpVtbl -> GetSelectedOptionInPrintTicket(This,pFeature,ppOption) ) 

#define IPrintSchemaCapabilities2_GetOptions(This,pFeature,ppOptionCollection)	\
    ( (This)->lpVtbl -> GetOptions(This,pFeature,ppOptionCollection) ) 


#define IPrintSchemaCapabilities2_GetParameterDefinition(This,bstrName,bstrNamespaceUri,ppParameterDefinition)	\
    ( (This)->lpVtbl -> GetParameterDefinition(This,bstrName,bstrNamespaceUri,ppParameterDefinition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaCapabilities2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0012 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0012_v0_0_s_ifspec;

#ifndef __IPrintSchemaAsyncOperation_INTERFACE_DEFINED__
#define __IPrintSchemaAsyncOperation_INTERFACE_DEFINED__

/* interface IPrintSchemaAsyncOperation */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaAsyncOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("143c8dcb-d37f-47f7-88e8-6b1d21f2c5f7")
    IPrintSchemaAsyncOperation : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaAsyncOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaAsyncOperation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaAsyncOperation * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaAsyncOperation * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaAsyncOperation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaAsyncOperation * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaAsyncOperation, Start)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IPrintSchemaAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IPrintSchemaAsyncOperation, Cancel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPrintSchemaAsyncOperation * This);
        
        END_INTERFACE
    } IPrintSchemaAsyncOperationVtbl;

    interface IPrintSchemaAsyncOperation
    {
        CONST_VTBL struct IPrintSchemaAsyncOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaAsyncOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaAsyncOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaAsyncOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaAsyncOperation_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaAsyncOperation_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaAsyncOperation_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaAsyncOperation_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaAsyncOperation_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IPrintSchemaAsyncOperation_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaAsyncOperation_INTERFACE_DEFINED__ */


#ifndef __IPrintSchemaTicket_INTERFACE_DEFINED__
#define __IPrintSchemaTicket_INTERFACE_DEFINED__

/* interface IPrintSchemaTicket */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaTicket;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e480b861-4708-4e6d-a5b4-a2b4eeb9baa4")
    IPrintSchemaTicket : public IPrintSchemaElement
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetFeatureByKeyName( 
            /* [in] */ __RPC__in BSTR bstrKeyName,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetFeature( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE ValidateAsync( 
            /* [out] */ __RPC__deref_out_opt IPrintSchemaAsyncOperation **ppAsyncOperation) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE CommitAsync( 
            /* [in] */ __RPC__in_opt IPrintSchemaTicket *pPrintTicketCommit,
            /* [out] */ __RPC__deref_out_opt IPrintSchemaAsyncOperation **ppAsyncOperation) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE NotifyXmlChanged( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaCapabilities **ppCapabilities) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_JobCopiesAllDocuments( 
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocuments) = 0;
        
        virtual /* [id][propput] */ HRESULT STDMETHODCALLTYPE put_JobCopiesAllDocuments( 
            /* [in] */ ULONG ulJobCopiesAllDocuments) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaTicketVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaTicket * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaTicket * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaTicket * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, GetFeatureByKeyName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeatureByKeyName )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ __RPC__in BSTR bstrKeyName,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, GetFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeature )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, ValidateAsync)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ValidateAsync )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [out] */ __RPC__deref_out_opt IPrintSchemaAsyncOperation **ppAsyncOperation);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, CommitAsync)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CommitAsync )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ __RPC__in_opt IPrintSchemaTicket *pPrintTicketCommit,
            /* [out] */ __RPC__deref_out_opt IPrintSchemaAsyncOperation **ppAsyncOperation);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, NotifyXmlChanged)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NotifyXmlChanged )( 
            __RPC__in IPrintSchemaTicket * This);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, GetCapabilities)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaCapabilities **ppCapabilities);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, get_JobCopiesAllDocuments)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobCopiesAllDocuments )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocuments);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, put_JobCopiesAllDocuments)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_JobCopiesAllDocuments )( 
            __RPC__in IPrintSchemaTicket * This,
            /* [in] */ ULONG ulJobCopiesAllDocuments);
        
        END_INTERFACE
    } IPrintSchemaTicketVtbl;

    interface IPrintSchemaTicket
    {
        CONST_VTBL struct IPrintSchemaTicketVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaTicket_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaTicket_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaTicket_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaTicket_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaTicket_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaTicket_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaTicket_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaTicket_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaTicket_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaTicket_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaTicket_GetFeatureByKeyName(This,bstrKeyName,ppFeature)	\
    ( (This)->lpVtbl -> GetFeatureByKeyName(This,bstrKeyName,ppFeature) ) 

#define IPrintSchemaTicket_GetFeature(This,bstrName,bstrNamespaceUri,ppFeature)	\
    ( (This)->lpVtbl -> GetFeature(This,bstrName,bstrNamespaceUri,ppFeature) ) 

#define IPrintSchemaTicket_ValidateAsync(This,ppAsyncOperation)	\
    ( (This)->lpVtbl -> ValidateAsync(This,ppAsyncOperation) ) 

#define IPrintSchemaTicket_CommitAsync(This,pPrintTicketCommit,ppAsyncOperation)	\
    ( (This)->lpVtbl -> CommitAsync(This,pPrintTicketCommit,ppAsyncOperation) ) 

#define IPrintSchemaTicket_NotifyXmlChanged(This)	\
    ( (This)->lpVtbl -> NotifyXmlChanged(This) ) 

#define IPrintSchemaTicket_GetCapabilities(This,ppCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,ppCapabilities) ) 

#define IPrintSchemaTicket_get_JobCopiesAllDocuments(This,pulJobCopiesAllDocuments)	\
    ( (This)->lpVtbl -> get_JobCopiesAllDocuments(This,pulJobCopiesAllDocuments) ) 

#define IPrintSchemaTicket_put_JobCopiesAllDocuments(This,ulJobCopiesAllDocuments)	\
    ( (This)->lpVtbl -> put_JobCopiesAllDocuments(This,ulJobCopiesAllDocuments) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaTicket_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0014 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0014_v0_0_s_ifspec;

#ifndef __IPrintSchemaTicket2_INTERFACE_DEFINED__
#define __IPrintSchemaTicket2_INTERFACE_DEFINED__

/* interface IPrintSchemaTicket2 */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaTicket2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2ec1f844-766a-47a1-91f4-2eeb6190f80c")
    IPrintSchemaTicket2 : public IPrintSchemaTicket
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetParameterInitializer( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaParameterInitializer **ppParameterInitializer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaTicket2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaTicket2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaTicket2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaTicket2 * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_XmlNode)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_XmlNode )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintSchemaElement, get_NamespaceUri)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_NamespaceUri )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrNamespaceUri);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, GetFeatureByKeyName)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeatureByKeyName )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ __RPC__in BSTR bstrKeyName,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, GetFeature)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetFeature )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaFeature **ppFeature);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, ValidateAsync)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *ValidateAsync )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [out] */ __RPC__deref_out_opt IPrintSchemaAsyncOperation **ppAsyncOperation);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, CommitAsync)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *CommitAsync )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ __RPC__in_opt IPrintSchemaTicket *pPrintTicketCommit,
            /* [out] */ __RPC__deref_out_opt IPrintSchemaAsyncOperation **ppAsyncOperation);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, NotifyXmlChanged)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *NotifyXmlChanged )( 
            __RPC__in IPrintSchemaTicket2 * This);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, GetCapabilities)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaCapabilities **ppCapabilities);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, get_JobCopiesAllDocuments)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_JobCopiesAllDocuments )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [retval][out] */ __RPC__out ULONG *pulJobCopiesAllDocuments);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket, put_JobCopiesAllDocuments)
        /* [id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_JobCopiesAllDocuments )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ ULONG ulJobCopiesAllDocuments);
        
        DECLSPEC_XFGVIRT(IPrintSchemaTicket2, GetParameterInitializer)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetParameterInitializer )( 
            __RPC__in IPrintSchemaTicket2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [defaultvalue][in] */ __RPC__in BSTR bstrNamespaceUri,
            /* [retval][out] */ __RPC__deref_out_opt IPrintSchemaParameterInitializer **ppParameterInitializer);
        
        END_INTERFACE
    } IPrintSchemaTicket2Vtbl;

    interface IPrintSchemaTicket2
    {
        CONST_VTBL struct IPrintSchemaTicket2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaTicket2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaTicket2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaTicket2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaTicket2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaTicket2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaTicket2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaTicket2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaTicket2_get_XmlNode(This,ppXmlNode)	\
    ( (This)->lpVtbl -> get_XmlNode(This,ppXmlNode) ) 

#define IPrintSchemaTicket2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintSchemaTicket2_get_NamespaceUri(This,pbstrNamespaceUri)	\
    ( (This)->lpVtbl -> get_NamespaceUri(This,pbstrNamespaceUri) ) 


#define IPrintSchemaTicket2_GetFeatureByKeyName(This,bstrKeyName,ppFeature)	\
    ( (This)->lpVtbl -> GetFeatureByKeyName(This,bstrKeyName,ppFeature) ) 

#define IPrintSchemaTicket2_GetFeature(This,bstrName,bstrNamespaceUri,ppFeature)	\
    ( (This)->lpVtbl -> GetFeature(This,bstrName,bstrNamespaceUri,ppFeature) ) 

#define IPrintSchemaTicket2_ValidateAsync(This,ppAsyncOperation)	\
    ( (This)->lpVtbl -> ValidateAsync(This,ppAsyncOperation) ) 

#define IPrintSchemaTicket2_CommitAsync(This,pPrintTicketCommit,ppAsyncOperation)	\
    ( (This)->lpVtbl -> CommitAsync(This,pPrintTicketCommit,ppAsyncOperation) ) 

#define IPrintSchemaTicket2_NotifyXmlChanged(This)	\
    ( (This)->lpVtbl -> NotifyXmlChanged(This) ) 

#define IPrintSchemaTicket2_GetCapabilities(This,ppCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,ppCapabilities) ) 

#define IPrintSchemaTicket2_get_JobCopiesAllDocuments(This,pulJobCopiesAllDocuments)	\
    ( (This)->lpVtbl -> get_JobCopiesAllDocuments(This,pulJobCopiesAllDocuments) ) 

#define IPrintSchemaTicket2_put_JobCopiesAllDocuments(This,ulJobCopiesAllDocuments)	\
    ( (This)->lpVtbl -> put_JobCopiesAllDocuments(This,ulJobCopiesAllDocuments) ) 


#define IPrintSchemaTicket2_GetParameterInitializer(This,bstrName,bstrNamespaceUri,ppParameterInitializer)	\
    ( (This)->lpVtbl -> GetParameterInitializer(This,bstrName,bstrNamespaceUri,ppParameterInitializer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaTicket2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0015 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0015_v0_0_s_ifspec;

#ifndef __IPrintSchemaAsyncOperationEvent_INTERFACE_DEFINED__
#define __IPrintSchemaAsyncOperationEvent_INTERFACE_DEFINED__

/* interface IPrintSchemaAsyncOperationEvent */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrintSchemaAsyncOperationEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23adbb16-0133-4906-b29a-1dce1d026379")
    IPrintSchemaAsyncOperationEvent : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Completed( 
            /* [in] */ __RPC__in_opt IPrintSchemaTicket *pTicket,
            /* [in] */ HRESULT hrOperation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintSchemaAsyncOperationEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintSchemaAsyncOperationEvent * This,
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
        
        DECLSPEC_XFGVIRT(IPrintSchemaAsyncOperationEvent, Completed)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Completed )( 
            __RPC__in IPrintSchemaAsyncOperationEvent * This,
            /* [in] */ __RPC__in_opt IPrintSchemaTicket *pTicket,
            /* [in] */ HRESULT hrOperation);
        
        END_INTERFACE
    } IPrintSchemaAsyncOperationEventVtbl;

    interface IPrintSchemaAsyncOperationEvent
    {
        CONST_VTBL struct IPrintSchemaAsyncOperationEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintSchemaAsyncOperationEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintSchemaAsyncOperationEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintSchemaAsyncOperationEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintSchemaAsyncOperationEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintSchemaAsyncOperationEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintSchemaAsyncOperationEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintSchemaAsyncOperationEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintSchemaAsyncOperationEvent_Completed(This,pTicket,hrOperation)	\
    ( (This)->lpVtbl -> Completed(This,pTicket,hrOperation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintSchemaAsyncOperationEvent_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0016 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0016_v0_0_s_ifspec;

#ifndef __IPrinterScriptableSequentialStream_INTERFACE_DEFINED__
#define __IPrinterScriptableSequentialStream_INTERFACE_DEFINED__

/* interface IPrinterScriptableSequentialStream */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrinterScriptableSequentialStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2072838a-316f-467a-a949-27f68c44a854")
    IPrinterScriptableSequentialStream : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Read( 
            /* [in] */ LONG cbRead,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppArray) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ __RPC__in_opt IDispatch *pArray,
            /* [retval][out] */ __RPC__out LONG *pcbWritten) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterScriptableSequentialStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterScriptableSequentialStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterScriptableSequentialStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterScriptableSequentialStream * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrinterScriptableSequentialStream * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrinterScriptableSequentialStream * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrinterScriptableSequentialStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterScriptableSequentialStream * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterScriptableSequentialStream, Read)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IPrinterScriptableSequentialStream * This,
            /* [in] */ LONG cbRead,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppArray);
        
        DECLSPEC_XFGVIRT(IPrinterScriptableSequentialStream, Write)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IPrinterScriptableSequentialStream * This,
            /* [in] */ __RPC__in_opt IDispatch *pArray,
            /* [retval][out] */ __RPC__out LONG *pcbWritten);
        
        END_INTERFACE
    } IPrinterScriptableSequentialStreamVtbl;

    interface IPrinterScriptableSequentialStream
    {
        CONST_VTBL struct IPrinterScriptableSequentialStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterScriptableSequentialStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterScriptableSequentialStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterScriptableSequentialStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterScriptableSequentialStream_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterScriptableSequentialStream_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterScriptableSequentialStream_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterScriptableSequentialStream_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterScriptableSequentialStream_Read(This,cbRead,ppArray)	\
    ( (This)->lpVtbl -> Read(This,cbRead,ppArray) ) 

#define IPrinterScriptableSequentialStream_Write(This,pArray,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pArray,pcbWritten) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterScriptableSequentialStream_INTERFACE_DEFINED__ */


#ifndef __IPrinterScriptableStream_INTERFACE_DEFINED__
#define __IPrinterScriptableStream_INTERFACE_DEFINED__

/* interface IPrinterScriptableStream */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrinterScriptableStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7edf9a92-4750-41a5-a17f-879a6f4f7dcb")
    IPrinterScriptableStream : public IPrinterScriptableSequentialStream
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Seek( 
            /* [in] */ LONG lOffset,
            /* [in] */ STREAM_SEEK streamSeek,
            /* [retval][out] */ __RPC__out LONG *plPosition) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetSize( 
            /* [in] */ LONG lSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterScriptableStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterScriptableStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterScriptableStream * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterScriptableStream * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterScriptableSequentialStream, Read)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ LONG cbRead,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppArray);
        
        DECLSPEC_XFGVIRT(IPrinterScriptableSequentialStream, Write)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ __RPC__in_opt IDispatch *pArray,
            /* [retval][out] */ __RPC__out LONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IPrinterScriptableStream, Commit)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IPrinterScriptableStream * This);
        
        DECLSPEC_XFGVIRT(IPrinterScriptableStream, Seek)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ LONG lOffset,
            /* [in] */ STREAM_SEEK streamSeek,
            /* [retval][out] */ __RPC__out LONG *plPosition);
        
        DECLSPEC_XFGVIRT(IPrinterScriptableStream, SetSize)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IPrinterScriptableStream * This,
            /* [in] */ LONG lSize);
        
        END_INTERFACE
    } IPrinterScriptableStreamVtbl;

    interface IPrinterScriptableStream
    {
        CONST_VTBL struct IPrinterScriptableStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterScriptableStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterScriptableStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterScriptableStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterScriptableStream_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterScriptableStream_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterScriptableStream_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterScriptableStream_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterScriptableStream_Read(This,cbRead,ppArray)	\
    ( (This)->lpVtbl -> Read(This,cbRead,ppArray) ) 

#define IPrinterScriptableStream_Write(This,pArray,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pArray,pcbWritten) ) 


#define IPrinterScriptableStream_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#define IPrinterScriptableStream_Seek(This,lOffset,streamSeek,plPosition)	\
    ( (This)->lpVtbl -> Seek(This,lOffset,streamSeek,plPosition) ) 

#define IPrinterScriptableStream_SetSize(This,lSize)	\
    ( (This)->lpVtbl -> SetSize(This,lSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterScriptableStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0018 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0018_v0_0_s_ifspec;

#ifndef __IPrinterPropertyBag_INTERFACE_DEFINED__
#define __IPrinterPropertyBag_INTERFACE_DEFINED__

/* interface IPrinterPropertyBag */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterPropertyBag;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fea77364-df95-4a23-a905-019b79a8e481")
    IPrinterPropertyBag : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetBool( 
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ BOOL *pbValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetBool( 
            /* [in] */ BSTR bstrName,
            /* [in] */ BOOL bValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetInt32( 
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ LONG *pnValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetInt32( 
            /* [in] */ BSTR bstrName,
            /* [in] */ LONG nValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetString( 
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ BSTR *pbstrValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetString( 
            /* [in] */ BSTR bstrName,
            /* [in] */ BSTR bstrValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetBytes( 
            /* [in] */ BSTR bstrName,
            /* [out] */ DWORD *pcbValue,
            /* [size_is][size_is][out] */ BYTE **ppValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetBytes( 
            /* [in] */ BSTR bstrName,
            /* [in] */ DWORD cbValue,
            /* [size_is][in] */ BYTE *pValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetReadStream( 
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ IStream **ppValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetWriteStream( 
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ IStream **ppValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterPropertyBagVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterPropertyBag * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterPropertyBag * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterPropertyBag * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterPropertyBag * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterPropertyBag * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, GetBool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetBool )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ BOOL *pbValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, SetBool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetBool )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [in] */ BOOL bValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, GetInt32)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInt32 )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ LONG *pnValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, SetInt32)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInt32 )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [in] */ LONG nValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, GetString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, SetString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [in] */ BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, GetBytes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetBytes )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [out] */ DWORD *pcbValue,
            /* [size_is][size_is][out] */ BYTE **ppValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, SetBytes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetBytes )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [in] */ DWORD cbValue,
            /* [size_is][in] */ BYTE *pValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, GetReadStream)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetReadStream )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ IStream **ppValue);
        
        DECLSPEC_XFGVIRT(IPrinterPropertyBag, GetWriteStream)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetWriteStream )( 
            IPrinterPropertyBag * This,
            /* [in] */ BSTR bstrName,
            /* [retval][out] */ IStream **ppValue);
        
        END_INTERFACE
    } IPrinterPropertyBagVtbl;

    interface IPrinterPropertyBag
    {
        CONST_VTBL struct IPrinterPropertyBagVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterPropertyBag_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterPropertyBag_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterPropertyBag_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterPropertyBag_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterPropertyBag_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterPropertyBag_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterPropertyBag_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterPropertyBag_GetBool(This,bstrName,pbValue)	\
    ( (This)->lpVtbl -> GetBool(This,bstrName,pbValue) ) 

#define IPrinterPropertyBag_SetBool(This,bstrName,bValue)	\
    ( (This)->lpVtbl -> SetBool(This,bstrName,bValue) ) 

#define IPrinterPropertyBag_GetInt32(This,bstrName,pnValue)	\
    ( (This)->lpVtbl -> GetInt32(This,bstrName,pnValue) ) 

#define IPrinterPropertyBag_SetInt32(This,bstrName,nValue)	\
    ( (This)->lpVtbl -> SetInt32(This,bstrName,nValue) ) 

#define IPrinterPropertyBag_GetString(This,bstrName,pbstrValue)	\
    ( (This)->lpVtbl -> GetString(This,bstrName,pbstrValue) ) 

#define IPrinterPropertyBag_SetString(This,bstrName,bstrValue)	\
    ( (This)->lpVtbl -> SetString(This,bstrName,bstrValue) ) 

#define IPrinterPropertyBag_GetBytes(This,bstrName,pcbValue,ppValue)	\
    ( (This)->lpVtbl -> GetBytes(This,bstrName,pcbValue,ppValue) ) 

#define IPrinterPropertyBag_SetBytes(This,bstrName,cbValue,pValue)	\
    ( (This)->lpVtbl -> SetBytes(This,bstrName,cbValue,pValue) ) 

#define IPrinterPropertyBag_GetReadStream(This,bstrName,ppValue)	\
    ( (This)->lpVtbl -> GetReadStream(This,bstrName,ppValue) ) 

#define IPrinterPropertyBag_GetWriteStream(This,bstrName,ppValue)	\
    ( (This)->lpVtbl -> GetWriteStream(This,bstrName,ppValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterPropertyBag_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0019 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0019_v0_0_s_ifspec;

#ifndef __IPrinterScriptablePropertyBag_INTERFACE_DEFINED__
#define __IPrinterScriptablePropertyBag_INTERFACE_DEFINED__

/* interface IPrinterScriptablePropertyBag */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrinterScriptablePropertyBag;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("91c7765f-ed57-49ad-8b01-dc24816a5294")
    IPrinterScriptablePropertyBag : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetBool( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out BOOL *pbValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetBool( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ BOOL bValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetInt32( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out LONG *pnValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetInt32( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ LONG nValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetString( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetString( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetBytes( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppArray) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetBytes( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in_opt IDispatch *pArray) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetReadStream( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptableStream **ppStream) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetWriteStream( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptableStream **ppStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterScriptablePropertyBagVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterScriptablePropertyBag * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterScriptablePropertyBag * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterScriptablePropertyBag * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetBool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetBool )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out BOOL *pbValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetBool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetBool )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ BOOL bValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetInt32)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInt32 )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out LONG *pnValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetInt32)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInt32 )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ LONG nValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetBytes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetBytes )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppArray);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetBytes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetBytes )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in_opt IDispatch *pArray);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetReadStream)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetReadStream )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptableStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetWriteStream)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetWriteStream )( 
            __RPC__in IPrinterScriptablePropertyBag * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptableStream **ppStream);
        
        END_INTERFACE
    } IPrinterScriptablePropertyBagVtbl;

    interface IPrinterScriptablePropertyBag
    {
        CONST_VTBL struct IPrinterScriptablePropertyBagVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterScriptablePropertyBag_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterScriptablePropertyBag_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterScriptablePropertyBag_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterScriptablePropertyBag_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterScriptablePropertyBag_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterScriptablePropertyBag_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterScriptablePropertyBag_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterScriptablePropertyBag_GetBool(This,bstrName,pbValue)	\
    ( (This)->lpVtbl -> GetBool(This,bstrName,pbValue) ) 

#define IPrinterScriptablePropertyBag_SetBool(This,bstrName,bValue)	\
    ( (This)->lpVtbl -> SetBool(This,bstrName,bValue) ) 

#define IPrinterScriptablePropertyBag_GetInt32(This,bstrName,pnValue)	\
    ( (This)->lpVtbl -> GetInt32(This,bstrName,pnValue) ) 

#define IPrinterScriptablePropertyBag_SetInt32(This,bstrName,nValue)	\
    ( (This)->lpVtbl -> SetInt32(This,bstrName,nValue) ) 

#define IPrinterScriptablePropertyBag_GetString(This,bstrName,pbstrValue)	\
    ( (This)->lpVtbl -> GetString(This,bstrName,pbstrValue) ) 

#define IPrinterScriptablePropertyBag_SetString(This,bstrName,bstrValue)	\
    ( (This)->lpVtbl -> SetString(This,bstrName,bstrValue) ) 

#define IPrinterScriptablePropertyBag_GetBytes(This,bstrName,ppArray)	\
    ( (This)->lpVtbl -> GetBytes(This,bstrName,ppArray) ) 

#define IPrinterScriptablePropertyBag_SetBytes(This,bstrName,pArray)	\
    ( (This)->lpVtbl -> SetBytes(This,bstrName,pArray) ) 

#define IPrinterScriptablePropertyBag_GetReadStream(This,bstrName,ppStream)	\
    ( (This)->lpVtbl -> GetReadStream(This,bstrName,ppStream) ) 

#define IPrinterScriptablePropertyBag_GetWriteStream(This,bstrName,ppStream)	\
    ( (This)->lpVtbl -> GetWriteStream(This,bstrName,ppStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterScriptablePropertyBag_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0020 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0020_v0_0_s_ifspec;

#ifndef __IPrinterScriptablePropertyBag2_INTERFACE_DEFINED__
#define __IPrinterScriptablePropertyBag2_INTERFACE_DEFINED__

/* interface IPrinterScriptablePropertyBag2 */
/* [unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrinterScriptablePropertyBag2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2A1C53C4-8638-4B3E-B518-2773C94556A3")
    IPrinterScriptablePropertyBag2 : public IPrinterScriptablePropertyBag
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetReadStreamAsXML( 
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterScriptablePropertyBag2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterScriptablePropertyBag2 * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetBool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetBool )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out BOOL *pbValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetBool)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetBool )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ BOOL bValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetInt32)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetInt32 )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__out LONG *pnValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetInt32)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetInt32 )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ LONG nValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetString)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in BSTR bstrValue);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetBytes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetBytes )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IDispatch **ppArray);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, SetBytes)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetBytes )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [in] */ __RPC__in_opt IDispatch *pArray);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetReadStream)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetReadStream )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptableStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag, GetWriteStream)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetWriteStream )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptableStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPrinterScriptablePropertyBag2, GetReadStreamAsXML)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetReadStreamAsXML )( 
            __RPC__in IPrinterScriptablePropertyBag2 * This,
            /* [in] */ __RPC__in BSTR bstrName,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppXmlNode);
        
        END_INTERFACE
    } IPrinterScriptablePropertyBag2Vtbl;

    interface IPrinterScriptablePropertyBag2
    {
        CONST_VTBL struct IPrinterScriptablePropertyBag2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterScriptablePropertyBag2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterScriptablePropertyBag2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterScriptablePropertyBag2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterScriptablePropertyBag2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterScriptablePropertyBag2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterScriptablePropertyBag2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterScriptablePropertyBag2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterScriptablePropertyBag2_GetBool(This,bstrName,pbValue)	\
    ( (This)->lpVtbl -> GetBool(This,bstrName,pbValue) ) 

#define IPrinterScriptablePropertyBag2_SetBool(This,bstrName,bValue)	\
    ( (This)->lpVtbl -> SetBool(This,bstrName,bValue) ) 

#define IPrinterScriptablePropertyBag2_GetInt32(This,bstrName,pnValue)	\
    ( (This)->lpVtbl -> GetInt32(This,bstrName,pnValue) ) 

#define IPrinterScriptablePropertyBag2_SetInt32(This,bstrName,nValue)	\
    ( (This)->lpVtbl -> SetInt32(This,bstrName,nValue) ) 

#define IPrinterScriptablePropertyBag2_GetString(This,bstrName,pbstrValue)	\
    ( (This)->lpVtbl -> GetString(This,bstrName,pbstrValue) ) 

#define IPrinterScriptablePropertyBag2_SetString(This,bstrName,bstrValue)	\
    ( (This)->lpVtbl -> SetString(This,bstrName,bstrValue) ) 

#define IPrinterScriptablePropertyBag2_GetBytes(This,bstrName,ppArray)	\
    ( (This)->lpVtbl -> GetBytes(This,bstrName,ppArray) ) 

#define IPrinterScriptablePropertyBag2_SetBytes(This,bstrName,pArray)	\
    ( (This)->lpVtbl -> SetBytes(This,bstrName,pArray) ) 

#define IPrinterScriptablePropertyBag2_GetReadStream(This,bstrName,ppStream)	\
    ( (This)->lpVtbl -> GetReadStream(This,bstrName,ppStream) ) 

#define IPrinterScriptablePropertyBag2_GetWriteStream(This,bstrName,ppStream)	\
    ( (This)->lpVtbl -> GetWriteStream(This,bstrName,ppStream) ) 


#define IPrinterScriptablePropertyBag2_GetReadStreamAsXML(This,bstrName,ppXmlNode)	\
    ( (This)->lpVtbl -> GetReadStreamAsXML(This,bstrName,ppXmlNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterScriptablePropertyBag2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0021 */
/* [local] */ 

#endif /* (NTDDI_VERSION >= NTDDI_WINTHRESHOLD) */
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0021_v0_0_s_ifspec;

#ifndef __IPrinterQueue_INTERFACE_DEFINED__
#define __IPrinterQueue_INTERFACE_DEFINED__

/* interface IPrinterQueue */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterQueue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3580a828-07fe-4b94-ac1a-757d9d2d3056")
    IPrinterQueue : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Handle( 
            /* [retval][out] */ HANDLE *phPrinter) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *pbstrName) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SendBidiQuery( 
            /* [in] */ BSTR bstrBidiQuery) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterQueueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterQueue * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterQueue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterQueue * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterQueue * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterQueue * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterQueue * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterQueue * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterQueue, get_Handle)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            IPrinterQueue * This,
            /* [retval][out] */ HANDLE *phPrinter);
        
        DECLSPEC_XFGVIRT(IPrinterQueue, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IPrinterQueue * This,
            /* [retval][out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrinterQueue, SendBidiQuery)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SendBidiQuery )( 
            IPrinterQueue * This,
            /* [in] */ BSTR bstrBidiQuery);
        
        DECLSPEC_XFGVIRT(IPrinterQueue, GetProperties)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            IPrinterQueue * This,
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag);
        
        END_INTERFACE
    } IPrinterQueueVtbl;

    interface IPrinterQueue
    {
        CONST_VTBL struct IPrinterQueueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterQueue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterQueue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterQueue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterQueue_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterQueue_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterQueue_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterQueue_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterQueue_get_Handle(This,phPrinter)	\
    ( (This)->lpVtbl -> get_Handle(This,phPrinter) ) 

#define IPrinterQueue_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrinterQueue_SendBidiQuery(This,bstrBidiQuery)	\
    ( (This)->lpVtbl -> SendBidiQuery(This,bstrBidiQuery) ) 

#define IPrinterQueue_GetProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> GetProperties(This,ppPropertyBag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterQueue_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0022 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)
typedef /* [v1_enum] */ 
enum tagPrintJobStatus
    {
        PrintJobStatus_Paused	= 0x1,
        PrintJobStatus_Error	= 0x2,
        PrintJobStatus_Deleting	= 0x4,
        PrintJobStatus_Spooling	= 0x8,
        PrintJobStatus_Printing	= 0x10,
        PrintJobStatus_Offline	= 0x20,
        PrintJobStatus_PaperOut	= 0x40,
        PrintJobStatus_Printed	= 0x80,
        PrintJobStatus_Deleted	= 0x100,
        PrintJobStatus_BlockedDeviceQueue	= 0x200,
        PrintJobStatus_UserIntervention	= 0x400,
        PrintJobStatus_Restarted	= 0x800,
        PrintJobStatus_Complete	= 0x1000,
        PrintJobStatus_Retained	= 0x2000
    } 	PrintJobStatus;



extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0022_v0_0_s_ifspec;

#ifndef __IPrintJob_INTERFACE_DEFINED__
#define __IPrintJob_INTERFACE_DEFINED__

/* interface IPrintJob */
/* [unique][nonextensible][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b771dab8-1282-41b7-858c-f206e4d20577")
    IPrintJob : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ BSTR *pbstrName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ ULONG *pulID) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PrintedPages( 
            /* [retval][out] */ ULONG *pulPages) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TotalPages( 
            /* [retval][out] */ ULONG *pulPages) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Status( 
            /* [retval][out] */ PrintJobStatus *pStatus) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SubmissionTime( 
            /* [retval][out] */ DATE *pSubmissionTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestCancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintJob * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintJob * This);
        
        DECLSPEC_XFGVIRT(IPrintJob, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IPrintJob * This,
            /* [retval][out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrintJob, get_Id)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            IPrintJob * This,
            /* [retval][out] */ ULONG *pulID);
        
        DECLSPEC_XFGVIRT(IPrintJob, get_PrintedPages)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrintedPages )( 
            IPrintJob * This,
            /* [retval][out] */ ULONG *pulPages);
        
        DECLSPEC_XFGVIRT(IPrintJob, get_TotalPages)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TotalPages )( 
            IPrintJob * This,
            /* [retval][out] */ ULONG *pulPages);
        
        DECLSPEC_XFGVIRT(IPrintJob, get_Status)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Status )( 
            IPrintJob * This,
            /* [retval][out] */ PrintJobStatus *pStatus);
        
        DECLSPEC_XFGVIRT(IPrintJob, get_SubmissionTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SubmissionTime )( 
            IPrintJob * This,
            /* [retval][out] */ DATE *pSubmissionTime);
        
        DECLSPEC_XFGVIRT(IPrintJob, RequestCancel)
        HRESULT ( STDMETHODCALLTYPE *RequestCancel )( 
            IPrintJob * This);
        
        END_INTERFACE
    } IPrintJobVtbl;

    interface IPrintJob
    {
        CONST_VTBL struct IPrintJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintJob_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrintJob_get_Id(This,pulID)	\
    ( (This)->lpVtbl -> get_Id(This,pulID) ) 

#define IPrintJob_get_PrintedPages(This,pulPages)	\
    ( (This)->lpVtbl -> get_PrintedPages(This,pulPages) ) 

#define IPrintJob_get_TotalPages(This,pulPages)	\
    ( (This)->lpVtbl -> get_TotalPages(This,pulPages) ) 

#define IPrintJob_get_Status(This,pStatus)	\
    ( (This)->lpVtbl -> get_Status(This,pStatus) ) 

#define IPrintJob_get_SubmissionTime(This,pSubmissionTime)	\
    ( (This)->lpVtbl -> get_SubmissionTime(This,pSubmissionTime) ) 

#define IPrintJob_RequestCancel(This)	\
    ( (This)->lpVtbl -> RequestCancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintJob_INTERFACE_DEFINED__ */


#ifndef __IPrintJobCollection_INTERFACE_DEFINED__
#define __IPrintJobCollection_INTERFACE_DEFINED__

/* interface IPrintJobCollection */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrintJobCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72b82a24-a598-4e87-895f-cdb23a49e9dc")
    IPrintJobCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ ULONG *pulCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ IPrintJob **ppJob) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintJobCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrintJobCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrintJobCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrintJobCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrintJobCollection * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrintJobCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrintJobCollection * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrintJobCollection * This,
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
        
        DECLSPEC_XFGVIRT(IPrintJobCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            IPrintJobCollection * This,
            /* [retval][out] */ ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(IPrintJobCollection, GetAt)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            IPrintJobCollection * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ IPrintJob **ppJob);
        
        DECLSPEC_XFGVIRT(IPrintJobCollection, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            IPrintJobCollection * This,
            /* [retval][out] */ IUnknown **ppUnk);
        
        END_INTERFACE
    } IPrintJobCollectionVtbl;

    interface IPrintJobCollection
    {
        CONST_VTBL struct IPrintJobCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintJobCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintJobCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintJobCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintJobCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrintJobCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrintJobCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrintJobCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrintJobCollection_get_Count(This,pulCount)	\
    ( (This)->lpVtbl -> get_Count(This,pulCount) ) 

#define IPrintJobCollection_GetAt(This,ulIndex,ppJob)	\
    ( (This)->lpVtbl -> GetAt(This,ulIndex,ppJob) ) 

#define IPrintJobCollection_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintJobCollection_INTERFACE_DEFINED__ */


#ifndef __IPrinterQueueViewEvent_INTERFACE_DEFINED__
#define __IPrinterQueueViewEvent_INTERFACE_DEFINED__

/* interface IPrinterQueueViewEvent */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterQueueViewEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c5b6042b-fd21-404a-a0ef-e2fbb52b9080")
    IPrinterQueueViewEvent : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OnChanged( 
            /* [in] */ IPrintJobCollection *pCollection,
            /* [in] */ ULONG ulViewOffset,
            /* [in] */ ULONG ulViewSize,
            /* [in] */ ULONG ulCountJobsInPrintQueue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterQueueViewEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterQueueViewEvent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterQueueViewEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterQueueViewEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterQueueViewEvent * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterQueueViewEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterQueueViewEvent * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterQueueViewEvent * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterQueueViewEvent, OnChanged)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OnChanged )( 
            IPrinterQueueViewEvent * This,
            /* [in] */ IPrintJobCollection *pCollection,
            /* [in] */ ULONG ulViewOffset,
            /* [in] */ ULONG ulViewSize,
            /* [in] */ ULONG ulCountJobsInPrintQueue);
        
        END_INTERFACE
    } IPrinterQueueViewEventVtbl;

    interface IPrinterQueueViewEvent
    {
        CONST_VTBL struct IPrinterQueueViewEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterQueueViewEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterQueueViewEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterQueueViewEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterQueueViewEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterQueueViewEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterQueueViewEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterQueueViewEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterQueueViewEvent_OnChanged(This,pCollection,ulViewOffset,ulViewSize,ulCountJobsInPrintQueue)	\
    ( (This)->lpVtbl -> OnChanged(This,pCollection,ulViewOffset,ulViewSize,ulCountJobsInPrintQueue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterQueueViewEvent_INTERFACE_DEFINED__ */


#ifndef __IPrinterQueueView_INTERFACE_DEFINED__
#define __IPrinterQueueView_INTERFACE_DEFINED__

/* interface IPrinterQueueView */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterQueueView;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("476e2969-3b2b-4b3f-8277-cff6056042aa")
    IPrinterQueueView : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SetViewRange( 
            /* [in] */ ULONG ulViewOffset,
            /* [in] */ ULONG ulViewSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterQueueViewVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterQueueView * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterQueueView * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterQueueView * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterQueueView * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterQueueView * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterQueueView * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterQueueView * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterQueueView, SetViewRange)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SetViewRange )( 
            IPrinterQueueView * This,
            /* [in] */ ULONG ulViewOffset,
            /* [in] */ ULONG ulViewSize);
        
        END_INTERFACE
    } IPrinterQueueViewVtbl;

    interface IPrinterQueueView
    {
        CONST_VTBL struct IPrinterQueueViewVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterQueueView_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterQueueView_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterQueueView_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterQueueView_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterQueueView_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterQueueView_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterQueueView_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterQueueView_SetViewRange(This,ulViewOffset,ulViewSize)	\
    ( (This)->lpVtbl -> SetViewRange(This,ulViewOffset,ulViewSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterQueueView_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0026 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0026_v0_0_s_ifspec;

#ifndef __IPrinterQueueEvent_INTERFACE_DEFINED__
#define __IPrinterQueueEvent_INTERFACE_DEFINED__

/* interface IPrinterQueueEvent */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterQueueEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("214685F6-7B78-4681-87E0-495F739273D1")
    IPrinterQueueEvent : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OnBidiResponseReceived( 
            /* [in] */ BSTR bstrResponse,
            /* [in] */ HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterQueueEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterQueueEvent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterQueueEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterQueueEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterQueueEvent * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterQueueEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterQueueEvent * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterQueueEvent * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterQueueEvent, OnBidiResponseReceived)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OnBidiResponseReceived )( 
            IPrinterQueueEvent * This,
            /* [in] */ BSTR bstrResponse,
            /* [in] */ HRESULT hrStatus);
        
        END_INTERFACE
    } IPrinterQueueEventVtbl;

    interface IPrinterQueueEvent
    {
        CONST_VTBL struct IPrinterQueueEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterQueueEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterQueueEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterQueueEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterQueueEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterQueueEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterQueueEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterQueueEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterQueueEvent_OnBidiResponseReceived(This,bstrResponse,hrStatus)	\
    ( (This)->lpVtbl -> OnBidiResponseReceived(This,bstrResponse,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterQueueEvent_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0027 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0027_v0_0_s_ifspec;

#ifndef __IPrinterBidiSetRequestCallback_INTERFACE_DEFINED__
#define __IPrinterBidiSetRequestCallback_INTERFACE_DEFINED__

/* interface IPrinterBidiSetRequestCallback */
/* [unique][nonextensible][oleautomation][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterBidiSetRequestCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c52d32dd-f2b4-4052-8502-ec4305ecb71f")
    IPrinterBidiSetRequestCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Completed( 
            /* [in] */ BSTR bstrResponse,
            /* [in] */ HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterBidiSetRequestCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterBidiSetRequestCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterBidiSetRequestCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterBidiSetRequestCallback * This);
        
        DECLSPEC_XFGVIRT(IPrinterBidiSetRequestCallback, Completed)
        HRESULT ( STDMETHODCALLTYPE *Completed )( 
            IPrinterBidiSetRequestCallback * This,
            /* [in] */ BSTR bstrResponse,
            /* [in] */ HRESULT hrStatus);
        
        END_INTERFACE
    } IPrinterBidiSetRequestCallbackVtbl;

    interface IPrinterBidiSetRequestCallback
    {
        CONST_VTBL struct IPrinterBidiSetRequestCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterBidiSetRequestCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterBidiSetRequestCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterBidiSetRequestCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterBidiSetRequestCallback_Completed(This,bstrResponse,hrStatus)	\
    ( (This)->lpVtbl -> Completed(This,bstrResponse,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterBidiSetRequestCallback_INTERFACE_DEFINED__ */


#ifndef __IPrinterExtensionAsyncOperation_INTERFACE_DEFINED__
#define __IPrinterExtensionAsyncOperation_INTERFACE_DEFINED__

/* interface IPrinterExtensionAsyncOperation */
/* [unique][nonextensible][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionAsyncOperation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("108d6a23-6a4b-4552-9448-68b427186acd")
    IPrinterExtensionAsyncOperation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionAsyncOperationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterExtensionAsyncOperation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterExtensionAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterExtensionAsyncOperation * This);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionAsyncOperation, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            IPrinterExtensionAsyncOperation * This);
        
        END_INTERFACE
    } IPrinterExtensionAsyncOperationVtbl;

    interface IPrinterExtensionAsyncOperation
    {
        CONST_VTBL struct IPrinterExtensionAsyncOperationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionAsyncOperation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionAsyncOperation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionAsyncOperation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionAsyncOperation_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionAsyncOperation_INTERFACE_DEFINED__ */


#ifndef __IPrinterQueue2_INTERFACE_DEFINED__
#define __IPrinterQueue2_INTERFACE_DEFINED__

/* interface IPrinterQueue2 */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterQueue2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8cd444e8-c9bb-49b3-8e38-e03209416131")
    IPrinterQueue2 : public IPrinterQueue
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE SendBidiSetRequestAsync( 
            /* [in] */ BSTR bstrBidiRequest,
            /* [in] */ IPrinterBidiSetRequestCallback *pCallback,
            /* [retval][out] */ IPrinterExtensionAsyncOperation **ppAsyncOperation) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetPrinterQueueView( 
            /* [in] */ ULONG ulViewOffset,
            /* [in] */ ULONG ulViewSize,
            /* [retval][out] */ IPrinterQueueView **ppJobView) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterQueue2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterQueue2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterQueue2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterQueue2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterQueue2 * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterQueue2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterQueue2 * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterQueue2 * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterQueue, get_Handle)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Handle )( 
            IPrinterQueue2 * This,
            /* [retval][out] */ HANDLE *phPrinter);
        
        DECLSPEC_XFGVIRT(IPrinterQueue, get_Name)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            IPrinterQueue2 * This,
            /* [retval][out] */ BSTR *pbstrName);
        
        DECLSPEC_XFGVIRT(IPrinterQueue, SendBidiQuery)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SendBidiQuery )( 
            IPrinterQueue2 * This,
            /* [in] */ BSTR bstrBidiQuery);
        
        DECLSPEC_XFGVIRT(IPrinterQueue, GetProperties)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            IPrinterQueue2 * This,
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag);
        
        DECLSPEC_XFGVIRT(IPrinterQueue2, SendBidiSetRequestAsync)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *SendBidiSetRequestAsync )( 
            IPrinterQueue2 * This,
            /* [in] */ BSTR bstrBidiRequest,
            /* [in] */ IPrinterBidiSetRequestCallback *pCallback,
            /* [retval][out] */ IPrinterExtensionAsyncOperation **ppAsyncOperation);
        
        DECLSPEC_XFGVIRT(IPrinterQueue2, GetPrinterQueueView)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetPrinterQueueView )( 
            IPrinterQueue2 * This,
            /* [in] */ ULONG ulViewOffset,
            /* [in] */ ULONG ulViewSize,
            /* [retval][out] */ IPrinterQueueView **ppJobView);
        
        END_INTERFACE
    } IPrinterQueue2Vtbl;

    interface IPrinterQueue2
    {
        CONST_VTBL struct IPrinterQueue2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterQueue2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterQueue2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterQueue2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterQueue2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterQueue2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterQueue2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterQueue2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterQueue2_get_Handle(This,phPrinter)	\
    ( (This)->lpVtbl -> get_Handle(This,phPrinter) ) 

#define IPrinterQueue2_get_Name(This,pbstrName)	\
    ( (This)->lpVtbl -> get_Name(This,pbstrName) ) 

#define IPrinterQueue2_SendBidiQuery(This,bstrBidiQuery)	\
    ( (This)->lpVtbl -> SendBidiQuery(This,bstrBidiQuery) ) 

#define IPrinterQueue2_GetProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> GetProperties(This,ppPropertyBag) ) 


#define IPrinterQueue2_SendBidiSetRequestAsync(This,bstrBidiRequest,pCallback,ppAsyncOperation)	\
    ( (This)->lpVtbl -> SendBidiSetRequestAsync(This,bstrBidiRequest,pCallback,ppAsyncOperation) ) 

#define IPrinterQueue2_GetPrinterQueueView(This,ulViewOffset,ulViewSize,ppJobView)	\
    ( (This)->lpVtbl -> GetPrinterQueueView(This,ulViewOffset,ulViewSize,ppJobView) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterQueue2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0030 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0030_v0_0_s_ifspec;

#ifndef __IPrinterExtensionContext_INTERFACE_DEFINED__
#define __IPrinterExtensionContext_INTERFACE_DEFINED__

/* interface IPrinterExtensionContext */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39843bf2-c4d2-41fd-b4b2-aedbee5e1900")
    IPrinterExtensionContext : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrinterQueue( 
            /* [retval][out] */ IPrinterQueue **ppQueue) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_PrintSchemaTicket( 
            /* [retval][out] */ IPrintSchemaTicket **ppTicket) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DriverProperties( 
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserProperties( 
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterExtensionContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterExtensionContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterExtensionContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterExtensionContext * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterExtensionContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterExtensionContext * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterExtensionContext * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_PrinterQueue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrinterQueue )( 
            IPrinterExtensionContext * This,
            /* [retval][out] */ IPrinterQueue **ppQueue);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_PrintSchemaTicket)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrintSchemaTicket )( 
            IPrinterExtensionContext * This,
            /* [retval][out] */ IPrintSchemaTicket **ppTicket);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_DriverProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DriverProperties )( 
            IPrinterExtensionContext * This,
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_UserProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserProperties )( 
            IPrinterExtensionContext * This,
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag);
        
        END_INTERFACE
    } IPrinterExtensionContextVtbl;

    interface IPrinterExtensionContext
    {
        CONST_VTBL struct IPrinterExtensionContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterExtensionContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterExtensionContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterExtensionContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterExtensionContext_get_PrinterQueue(This,ppQueue)	\
    ( (This)->lpVtbl -> get_PrinterQueue(This,ppQueue) ) 

#define IPrinterExtensionContext_get_PrintSchemaTicket(This,ppTicket)	\
    ( (This)->lpVtbl -> get_PrintSchemaTicket(This,ppTicket) ) 

#define IPrinterExtensionContext_get_DriverProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_DriverProperties(This,ppPropertyBag) ) 

#define IPrinterExtensionContext_get_UserProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_UserProperties(This,ppPropertyBag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0031 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0031_v0_0_s_ifspec;

#ifndef __IPrinterExtensionRequest_INTERFACE_DEFINED__
#define __IPrinterExtensionRequest_INTERFACE_DEFINED__

/* interface IPrinterExtensionRequest */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39843bf3-c4d2-41fd-b4b2-aedbee5e1900")
    IPrinterExtensionRequest : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Cancel( 
            /* [in] */ HRESULT hrStatus,
            /* [in] */ BSTR bstrLogMessage) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE Complete( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterExtensionRequest * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterExtensionRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterExtensionRequest * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterExtensionRequest * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterExtensionRequest * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterExtensionRequest * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterExtensionRequest * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterExtensionRequest, Cancel)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            IPrinterExtensionRequest * This,
            /* [in] */ HRESULT hrStatus,
            /* [in] */ BSTR bstrLogMessage);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionRequest, Complete)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *Complete )( 
            IPrinterExtensionRequest * This);
        
        END_INTERFACE
    } IPrinterExtensionRequestVtbl;

    interface IPrinterExtensionRequest
    {
        CONST_VTBL struct IPrinterExtensionRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionRequest_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterExtensionRequest_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterExtensionRequest_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterExtensionRequest_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterExtensionRequest_Cancel(This,hrStatus,bstrLogMessage)	\
    ( (This)->lpVtbl -> Cancel(This,hrStatus,bstrLogMessage) ) 

#define IPrinterExtensionRequest_Complete(This)	\
    ( (This)->lpVtbl -> Complete(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionRequest_INTERFACE_DEFINED__ */


#ifndef __IPrinterExtensionEventArgs_INTERFACE_DEFINED__
#define __IPrinterExtensionEventArgs_INTERFACE_DEFINED__

/* interface IPrinterExtensionEventArgs */
/* [unique][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionEventArgs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39843bf4-c4d2-41fd-b4b2-aedbee5e1900")
    IPrinterExtensionEventArgs : public IPrinterExtensionContext
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_BidiNotification( 
            /* [retval][out] */ BSTR *pbstrBidiNotification) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_ReasonId( 
            /* [retval][out] */ GUID *pReasonId) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Request( 
            /* [retval][out] */ IPrinterExtensionRequest **ppRequest) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_SourceApplication( 
            /* [retval][out] */ BSTR *pbstrApplication) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DetailedReasonId( 
            /* [retval][out] */ GUID *pDetailedReasonId) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_WindowModal( 
            /* [retval][out] */ BOOL *pbModal) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_WindowParent( 
            /* [retval][out] */ HANDLE *phwndParent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionEventArgsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterExtensionEventArgs * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterExtensionEventArgs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterExtensionEventArgs * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterExtensionEventArgs * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterExtensionEventArgs * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterExtensionEventArgs * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterExtensionEventArgs * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_PrinterQueue)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrinterQueue )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ IPrinterQueue **ppQueue);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_PrintSchemaTicket)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrintSchemaTicket )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ IPrintSchemaTicket **ppTicket);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_DriverProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DriverProperties )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContext, get_UserProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserProperties )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ IPrinterPropertyBag **ppPropertyBag);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_BidiNotification)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BidiNotification )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ BSTR *pbstrBidiNotification);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_ReasonId)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ReasonId )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ GUID *pReasonId);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_Request)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Request )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ IPrinterExtensionRequest **ppRequest);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_SourceApplication)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SourceApplication )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ BSTR *pbstrApplication);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_DetailedReasonId)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DetailedReasonId )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ GUID *pDetailedReasonId);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_WindowModal)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowModal )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ BOOL *pbModal);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEventArgs, get_WindowParent)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowParent )( 
            IPrinterExtensionEventArgs * This,
            /* [retval][out] */ HANDLE *phwndParent);
        
        END_INTERFACE
    } IPrinterExtensionEventArgsVtbl;

    interface IPrinterExtensionEventArgs
    {
        CONST_VTBL struct IPrinterExtensionEventArgsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionEventArgs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionEventArgs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionEventArgs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionEventArgs_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterExtensionEventArgs_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterExtensionEventArgs_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterExtensionEventArgs_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterExtensionEventArgs_get_PrinterQueue(This,ppQueue)	\
    ( (This)->lpVtbl -> get_PrinterQueue(This,ppQueue) ) 

#define IPrinterExtensionEventArgs_get_PrintSchemaTicket(This,ppTicket)	\
    ( (This)->lpVtbl -> get_PrintSchemaTicket(This,ppTicket) ) 

#define IPrinterExtensionEventArgs_get_DriverProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_DriverProperties(This,ppPropertyBag) ) 

#define IPrinterExtensionEventArgs_get_UserProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_UserProperties(This,ppPropertyBag) ) 


#define IPrinterExtensionEventArgs_get_BidiNotification(This,pbstrBidiNotification)	\
    ( (This)->lpVtbl -> get_BidiNotification(This,pbstrBidiNotification) ) 

#define IPrinterExtensionEventArgs_get_ReasonId(This,pReasonId)	\
    ( (This)->lpVtbl -> get_ReasonId(This,pReasonId) ) 

#define IPrinterExtensionEventArgs_get_Request(This,ppRequest)	\
    ( (This)->lpVtbl -> get_Request(This,ppRequest) ) 

#define IPrinterExtensionEventArgs_get_SourceApplication(This,pbstrApplication)	\
    ( (This)->lpVtbl -> get_SourceApplication(This,pbstrApplication) ) 

#define IPrinterExtensionEventArgs_get_DetailedReasonId(This,pDetailedReasonId)	\
    ( (This)->lpVtbl -> get_DetailedReasonId(This,pDetailedReasonId) ) 

#define IPrinterExtensionEventArgs_get_WindowModal(This,pbModal)	\
    ( (This)->lpVtbl -> get_WindowModal(This,pbModal) ) 

#define IPrinterExtensionEventArgs_get_WindowParent(This,phwndParent)	\
    ( (This)->lpVtbl -> get_WindowParent(This,phwndParent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionEventArgs_INTERFACE_DEFINED__ */


#ifndef __IPrinterExtensionContextCollection_INTERFACE_DEFINED__
#define __IPrinterExtensionContextCollection_INTERFACE_DEFINED__

/* interface IPrinterExtensionContextCollection */
/* [custom][unique][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionContextCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fb476970-9bab-4861-811e-3e98b0c5addf")
    IPrinterExtensionContextCollection : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *pulCount) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterExtensionContext **ppContext) = 0;
        
        virtual /* [id][restricted][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionContextCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterExtensionContextCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterExtensionContextCollection * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterExtensionContextCollection * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContextCollection, get_Count)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [retval][out] */ __RPC__out ULONG *pulCount);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContextCollection, GetAt)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterExtensionContext **ppContext);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionContextCollection, get__NewEnum)
        /* [id][restricted][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IPrinterExtensionContextCollection * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnk);
        
        END_INTERFACE
    } IPrinterExtensionContextCollectionVtbl;

    interface IPrinterExtensionContextCollection
    {
        CONST_VTBL struct IPrinterExtensionContextCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionContextCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionContextCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionContextCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionContextCollection_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterExtensionContextCollection_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterExtensionContextCollection_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterExtensionContextCollection_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterExtensionContextCollection_get_Count(This,pulCount)	\
    ( (This)->lpVtbl -> get_Count(This,pulCount) ) 

#define IPrinterExtensionContextCollection_GetAt(This,ulIndex,ppContext)	\
    ( (This)->lpVtbl -> GetAt(This,ulIndex,ppContext) ) 

#define IPrinterExtensionContextCollection_get__NewEnum(This,ppUnk)	\
    ( (This)->lpVtbl -> get__NewEnum(This,ppUnk) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionContextCollection_INTERFACE_DEFINED__ */


#ifndef __IPrinterExtensionEvent_INTERFACE_DEFINED__
#define __IPrinterExtensionEvent_INTERFACE_DEFINED__

/* interface IPrinterExtensionEvent */
/* [ref][nonextensible][dual][local][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionEvent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C093CB63-5EF5-4585-AF8E-4D5637487B57")
    IPrinterExtensionEvent : public IDispatch
    {
    public:
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OnDriverEvent( 
            /* [in] */ IPrinterExtensionEventArgs *pEventArgs) = 0;
        
        virtual /* [id] */ HRESULT STDMETHODCALLTYPE OnPrinterQueuesEnumerated( 
            /* [in] */ IPrinterExtensionContextCollection *pContextCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionEventVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPrinterExtensionEvent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPrinterExtensionEvent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPrinterExtensionEvent * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPrinterExtensionEvent * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPrinterExtensionEvent * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPrinterExtensionEvent * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterExtensionEvent * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEvent, OnDriverEvent)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OnDriverEvent )( 
            IPrinterExtensionEvent * This,
            /* [in] */ IPrinterExtensionEventArgs *pEventArgs);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionEvent, OnPrinterQueuesEnumerated)
        /* [id] */ HRESULT ( STDMETHODCALLTYPE *OnPrinterQueuesEnumerated )( 
            IPrinterExtensionEvent * This,
            /* [in] */ IPrinterExtensionContextCollection *pContextCollection);
        
        END_INTERFACE
    } IPrinterExtensionEventVtbl;

    interface IPrinterExtensionEvent
    {
        CONST_VTBL struct IPrinterExtensionEventVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionEvent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionEvent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionEvent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionEvent_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterExtensionEvent_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterExtensionEvent_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterExtensionEvent_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterExtensionEvent_OnDriverEvent(This,pEventArgs)	\
    ( (This)->lpVtbl -> OnDriverEvent(This,pEventArgs) ) 

#define IPrinterExtensionEvent_OnPrinterQueuesEnumerated(This,pContextCollection)	\
    ( (This)->lpVtbl -> OnPrinterQueuesEnumerated(This,pContextCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionEvent_INTERFACE_DEFINED__ */


#ifndef __IPrinterExtensionManager_INTERFACE_DEFINED__
#define __IPrinterExtensionManager_INTERFACE_DEFINED__

/* interface IPrinterExtensionManager */
/* [ref][uuid][object] */ 


EXTERN_C const IID IID_IPrinterExtensionManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("93C6EB8C-B001-4355-9629-8E8A1B3F8E77")
    IPrinterExtensionManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnableEvents( 
            /* [in] */ GUID printerDriverId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisableEvents( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterExtensionManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterExtensionManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterExtensionManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterExtensionManager * This);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionManager, EnableEvents)
        HRESULT ( STDMETHODCALLTYPE *EnableEvents )( 
            __RPC__in IPrinterExtensionManager * This,
            /* [in] */ GUID printerDriverId);
        
        DECLSPEC_XFGVIRT(IPrinterExtensionManager, DisableEvents)
        HRESULT ( STDMETHODCALLTYPE *DisableEvents )( 
            __RPC__in IPrinterExtensionManager * This);
        
        END_INTERFACE
    } IPrinterExtensionManagerVtbl;

    interface IPrinterExtensionManager
    {
        CONST_VTBL struct IPrinterExtensionManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterExtensionManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterExtensionManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterExtensionManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterExtensionManager_EnableEvents(This,printerDriverId)	\
    ( (This)->lpVtbl -> EnableEvents(This,printerDriverId) ) 

#define IPrinterExtensionManager_DisableEvents(This)	\
    ( (This)->lpVtbl -> DisableEvents(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterExtensionManager_INTERFACE_DEFINED__ */


#ifndef __IPrinterScriptContext_INTERFACE_DEFINED__
#define __IPrinterScriptContext_INTERFACE_DEFINED__

/* interface IPrinterScriptContext */
/* [ref][nonextensible][dual][uuid][object] */ 


EXTERN_C const IID IID_IPrinterScriptContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("066ACBCA-8881-49C9-BB98-FAE16B4889E1")
    IPrinterScriptContext : public IDispatch
    {
    public:
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_DriverProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptablePropertyBag **ppPropertyBag) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_QueueProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptablePropertyBag **ppPropertyBag) = 0;
        
        virtual /* [id][propget] */ HRESULT STDMETHODCALLTYPE get_UserProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptablePropertyBag **ppPropertyBag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrinterScriptContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrinterScriptContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrinterScriptContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrinterScriptContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IPrinterScriptContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IPrinterScriptContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IPrinterScriptContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPrinterScriptContext * This,
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
        
        DECLSPEC_XFGVIRT(IPrinterScriptContext, get_DriverProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DriverProperties )( 
            __RPC__in IPrinterScriptContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptablePropertyBag **ppPropertyBag);
        
        DECLSPEC_XFGVIRT(IPrinterScriptContext, get_QueueProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_QueueProperties )( 
            __RPC__in IPrinterScriptContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptablePropertyBag **ppPropertyBag);
        
        DECLSPEC_XFGVIRT(IPrinterScriptContext, get_UserProperties)
        /* [id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UserProperties )( 
            __RPC__in IPrinterScriptContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IPrinterScriptablePropertyBag **ppPropertyBag);
        
        END_INTERFACE
    } IPrinterScriptContextVtbl;

    interface IPrinterScriptContext
    {
        CONST_VTBL struct IPrinterScriptContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrinterScriptContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrinterScriptContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrinterScriptContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrinterScriptContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPrinterScriptContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPrinterScriptContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPrinterScriptContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPrinterScriptContext_get_DriverProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_DriverProperties(This,ppPropertyBag) ) 

#define IPrinterScriptContext_get_QueueProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_QueueProperties(This,ppPropertyBag) ) 

#define IPrinterScriptContext_get_UserProperties(This,ppPropertyBag)	\
    ( (This)->lpVtbl -> get_UserProperties(This,ppPropertyBag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrinterScriptContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printdriver_0000_0037 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0037_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0037_v0_0_s_ifspec;


#ifndef __PrinterExtensionLib_LIBRARY_DEFINED__
#define __PrinterExtensionLib_LIBRARY_DEFINED__

/* library PrinterExtensionLib */
/* [helpstring][version][uuid] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
























#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)











#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

EXTERN_C const IID LIBID_PrinterExtensionLib;

EXTERN_C const CLSID CLSID_PrinterQueue;

#ifdef __cplusplus

class DECLSPEC_UUID("EB54C230-798C-4C9E-B461-29FAD04039B1")
PrinterQueue;
#endif

EXTERN_C const CLSID CLSID_PrinterQueueView;

#ifdef __cplusplus

class DECLSPEC_UUID("EB54C231-798C-4C9E-B461-29FAD04039B1")
PrinterQueueView;
#endif

EXTERN_C const CLSID CLSID_PrintSchemaAsyncOperation;

#ifdef __cplusplus

class DECLSPEC_UUID("43b2f83d-10f2-48ab-831b-55fdbdbd34a4")
PrintSchemaAsyncOperation;
#endif

EXTERN_C const CLSID CLSID_PrinterExtensionManager;

#ifdef __cplusplus

class DECLSPEC_UUID("331B60DA-9E90-4DD0-9C84-EAC4E659B61F")
PrinterExtensionManager;
#endif
#endif /* __PrinterExtensionLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_printdriver_0000_0038 */
/* [local] */ 

DEFINE_GUID(PRINTER_EXTENSION_DETAILEDREASON_PRINTER_STATUS, 0x5d5a1704, 0xdfd1, 0x4181, 0x8e, 0xee, 0x81, 0x5c, 0x86, 0xed, 0xad, 0x31);
DEFINE_GUID(PRINTER_EXTENSION_REASON_PRINT_PREFERENCES, 0xec8f261f, 0x267c, 0x469f, 0xb5, 0xd6, 0x39, 0x33, 0x02, 0x3c, 0x29, 0xcc);
DEFINE_GUID(PRINTER_EXTENSION_REASON_DRIVER_EVENT,      0x23bb1328, 0x63de, 0x4293, 0x91, 0x5b, 0xa6, 0xa2, 0x3d, 0x92, 0x9a, 0xcb);
DEFINE_GUID(FMTID_PrinterPropertyBag,                   0x75f9adca, 0x097d, 0x45c3, 0xa6, 0xe4, 0xba, 0xb2, 0x9e, 0x27, 0x6f, 0x3e);
#endif // (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0038_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printdriver_0000_0038_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  VARIANT_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in VARIANT * ); 
unsigned char * __RPC_USER  VARIANT_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out VARIANT * ); 
void                      __RPC_USER  VARIANT_UserFree64(     __RPC__in unsigned long *, __RPC__in VARIANT * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


