

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

#ifndef __wsdclient_h__
#define __wsdclient_h__

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

#ifndef __IWSDEndpointProxy_FWD_DEFINED__
#define __IWSDEndpointProxy_FWD_DEFINED__
typedef interface IWSDEndpointProxy IWSDEndpointProxy;

#endif 	/* __IWSDEndpointProxy_FWD_DEFINED__ */


#ifndef __IWSDMetadataExchange_FWD_DEFINED__
#define __IWSDMetadataExchange_FWD_DEFINED__
typedef interface IWSDMetadataExchange IWSDMetadataExchange;

#endif 	/* __IWSDMetadataExchange_FWD_DEFINED__ */


#ifndef __IWSDServiceProxy_FWD_DEFINED__
#define __IWSDServiceProxy_FWD_DEFINED__
typedef interface IWSDServiceProxy IWSDServiceProxy;

#endif 	/* __IWSDServiceProxy_FWD_DEFINED__ */


#ifndef __IWSDServiceProxyEventing_FWD_DEFINED__
#define __IWSDServiceProxyEventing_FWD_DEFINED__
typedef interface IWSDServiceProxyEventing IWSDServiceProxyEventing;

#endif 	/* __IWSDServiceProxyEventing_FWD_DEFINED__ */


#ifndef __IWSDDeviceProxy_FWD_DEFINED__
#define __IWSDDeviceProxy_FWD_DEFINED__
typedef interface IWSDDeviceProxy IWSDDeviceProxy;

#endif 	/* __IWSDDeviceProxy_FWD_DEFINED__ */


#ifndef __IWSDAsyncResult_FWD_DEFINED__
#define __IWSDAsyncResult_FWD_DEFINED__
typedef interface IWSDAsyncResult IWSDAsyncResult;

#endif 	/* __IWSDAsyncResult_FWD_DEFINED__ */


#ifndef __IWSDAsyncCallback_FWD_DEFINED__
#define __IWSDAsyncCallback_FWD_DEFINED__
typedef interface IWSDAsyncCallback IWSDAsyncCallback;

#endif 	/* __IWSDAsyncCallback_FWD_DEFINED__ */


#ifndef __IWSDEventingStatus_FWD_DEFINED__
#define __IWSDEventingStatus_FWD_DEFINED__
typedef interface IWSDEventingStatus IWSDEventingStatus;

#endif 	/* __IWSDEventingStatus_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wsdxmldom.h"
#include "wsdtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wsdclient_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)










HRESULT WINAPI
WSDCreateDeviceProxy(
    _In_ LPCWSTR pszDeviceId,
    _In_ LPCWSTR pszLocalId,
    IWSDXMLContext* pContext,
    _Outptr_ IWSDDeviceProxy** ppDeviceProxy);
HRESULT WINAPI
WSDCreateDeviceProxyAdvanced(
    _In_ LPCWSTR pszDeviceId,
    IWSDAddress* pDeviceAddress,
    _In_ LPCWSTR pszLocalId,
    IWSDXMLContext* pContext,
    _Outptr_ IWSDDeviceProxy** ppDeviceProxy);
#if (WINVER >= _WIN32_WINNT_WIN7)
HRESULT WINAPI
WSDCreateDeviceProxy2(
    _In_ LPCWSTR pszDeviceId,
    _In_ LPCWSTR pszLocalId,
    IWSDXMLContext* pContext,
    _In_reads_opt_(dwConfigParamCount) WSD_CONFIG_PARAM* pConfigParams,
    DWORD dwConfigParamCount,
    _Outptr_ IWSDDeviceProxy** ppDeviceProxy);
#endif


extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0000_v0_0_s_ifspec;

#ifndef __IWSDEndpointProxy_INTERFACE_DEFINED__
#define __IWSDEndpointProxy_INTERFACE_DEFINED__

/* interface IWSDEndpointProxy */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDEndpointProxy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1860d430-b24c-4975-9f90-dbb39baa24ec")
    IWSDEndpointProxy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SendOneWayRequest( 
            /* [in] */ const void *pBody,
            /* [in] */ const WSD_OPERATION *pOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendTwoWayRequest( 
            /* [in] */ const void *pBody,
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_SYNCHRONOUS_RESPONSE_CONTEXT *pResponseContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendTwoWayRequestAsync( 
            /* [in] */ const void *pBody,
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [in] */ IUnknown *pAsyncState,
            /* [in] */ IWSDAsyncCallback *pCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AbortAsyncOperation( 
            /* [in] */ IWSDAsyncResult *pAsyncResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessFault( 
            /* [in] */ const WSD_SOAP_FAULT *pFault) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrorInfo( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszErrorInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFaultInfo( 
            /* [annotation][out] */ 
            _Outptr_  WSD_SOAP_FAULT **ppFault) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDEndpointProxyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDEndpointProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDEndpointProxy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDEndpointProxy * This);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, SendOneWayRequest)
        HRESULT ( STDMETHODCALLTYPE *SendOneWayRequest )( 
            IWSDEndpointProxy * This,
            /* [in] */ const void *pBody,
            /* [in] */ const WSD_OPERATION *pOperation);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, SendTwoWayRequest)
        HRESULT ( STDMETHODCALLTYPE *SendTwoWayRequest )( 
            IWSDEndpointProxy * This,
            /* [in] */ const void *pBody,
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_SYNCHRONOUS_RESPONSE_CONTEXT *pResponseContext);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, SendTwoWayRequestAsync)
        HRESULT ( STDMETHODCALLTYPE *SendTwoWayRequestAsync )( 
            IWSDEndpointProxy * This,
            /* [in] */ const void *pBody,
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [in] */ IUnknown *pAsyncState,
            /* [in] */ IWSDAsyncCallback *pCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **pResult);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, AbortAsyncOperation)
        HRESULT ( STDMETHODCALLTYPE *AbortAsyncOperation )( 
            IWSDEndpointProxy * This,
            /* [in] */ IWSDAsyncResult *pAsyncResult);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, ProcessFault)
        HRESULT ( STDMETHODCALLTYPE *ProcessFault )( 
            IWSDEndpointProxy * This,
            /* [in] */ const WSD_SOAP_FAULT *pFault);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, GetErrorInfo)
        HRESULT ( STDMETHODCALLTYPE *GetErrorInfo )( 
            IWSDEndpointProxy * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszErrorInfo);
        
        DECLSPEC_XFGVIRT(IWSDEndpointProxy, GetFaultInfo)
        HRESULT ( STDMETHODCALLTYPE *GetFaultInfo )( 
            IWSDEndpointProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_SOAP_FAULT **ppFault);
        
        END_INTERFACE
    } IWSDEndpointProxyVtbl;

    interface IWSDEndpointProxy
    {
        CONST_VTBL struct IWSDEndpointProxyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDEndpointProxy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDEndpointProxy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDEndpointProxy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDEndpointProxy_SendOneWayRequest(This,pBody,pOperation)	\
    ( (This)->lpVtbl -> SendOneWayRequest(This,pBody,pOperation) ) 

#define IWSDEndpointProxy_SendTwoWayRequest(This,pBody,pOperation,pResponseContext)	\
    ( (This)->lpVtbl -> SendTwoWayRequest(This,pBody,pOperation,pResponseContext) ) 

#define IWSDEndpointProxy_SendTwoWayRequestAsync(This,pBody,pOperation,pAsyncState,pCallback,pResult)	\
    ( (This)->lpVtbl -> SendTwoWayRequestAsync(This,pBody,pOperation,pAsyncState,pCallback,pResult) ) 

#define IWSDEndpointProxy_AbortAsyncOperation(This,pAsyncResult)	\
    ( (This)->lpVtbl -> AbortAsyncOperation(This,pAsyncResult) ) 

#define IWSDEndpointProxy_ProcessFault(This,pFault)	\
    ( (This)->lpVtbl -> ProcessFault(This,pFault) ) 

#define IWSDEndpointProxy_GetErrorInfo(This,ppszErrorInfo)	\
    ( (This)->lpVtbl -> GetErrorInfo(This,ppszErrorInfo) ) 

#define IWSDEndpointProxy_GetFaultInfo(This,ppFault)	\
    ( (This)->lpVtbl -> GetFaultInfo(This,ppFault) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDEndpointProxy_INTERFACE_DEFINED__ */


#ifndef __IWSDMetadataExchange_INTERFACE_DEFINED__
#define __IWSDMetadataExchange_INTERFACE_DEFINED__

/* interface IWSDMetadataExchange */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDMetadataExchange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("06996d57-1d67-4928-9307-3d7833fdb846")
    IWSDMetadataExchange : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMetadata( 
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **MetadataOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDMetadataExchangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDMetadataExchange * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDMetadataExchange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDMetadataExchange * This);
        
        DECLSPEC_XFGVIRT(IWSDMetadataExchange, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            IWSDMetadataExchange * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **MetadataOut);
        
        END_INTERFACE
    } IWSDMetadataExchangeVtbl;

    interface IWSDMetadataExchange
    {
        CONST_VTBL struct IWSDMetadataExchangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDMetadataExchange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDMetadataExchange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDMetadataExchange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDMetadataExchange_GetMetadata(This,MetadataOut)	\
    ( (This)->lpVtbl -> GetMetadata(This,MetadataOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDMetadataExchange_INTERFACE_DEFINED__ */


#ifndef __IWSDServiceProxy_INTERFACE_DEFINED__
#define __IWSDServiceProxy_INTERFACE_DEFINED__

/* interface IWSDServiceProxy */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDServiceProxy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d4c7fb9c-03ab-4175-9d67-094fafebf487")
    IWSDServiceProxy : public IWSDMetadataExchange
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginGetMetadata( 
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetMetadata( 
            /* [in] */ IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **ppMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceMetadata( 
            /* [annotation][out] */ 
            _Outptr_  WSD_SERVICE_METADATA **ppServiceMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SubscribeToOperation( 
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [in] */ IUnknown *pUnknown,
            /* [in] */ const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnsubscribeToOperation( 
            /* [in] */ const WSD_OPERATION *pOperation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEventingStatusCallback( 
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDEventingStatus *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndpointProxy( 
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppProxy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDServiceProxyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDServiceProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDServiceProxy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDServiceProxy * This);
        
        DECLSPEC_XFGVIRT(IWSDMetadataExchange, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            IWSDServiceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **MetadataOut);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, BeginGetMetadata)
        HRESULT ( STDMETHODCALLTYPE *BeginGetMetadata )( 
            IWSDServiceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, EndGetMetadata)
        HRESULT ( STDMETHODCALLTYPE *EndGetMetadata )( 
            IWSDServiceProxy * This,
            /* [in] */ IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, GetServiceMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetServiceMetadata )( 
            IWSDServiceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_SERVICE_METADATA **ppServiceMetadata);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, SubscribeToOperation)
        HRESULT ( STDMETHODCALLTYPE *SubscribeToOperation )( 
            IWSDServiceProxy * This,
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [in] */ IUnknown *pUnknown,
            /* [in] */ const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, UnsubscribeToOperation)
        HRESULT ( STDMETHODCALLTYPE *UnsubscribeToOperation )( 
            IWSDServiceProxy * This,
            /* [in] */ const WSD_OPERATION *pOperation);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, SetEventingStatusCallback)
        HRESULT ( STDMETHODCALLTYPE *SetEventingStatusCallback )( 
            IWSDServiceProxy * This,
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDEventingStatus *pStatus);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, GetEndpointProxy)
        HRESULT ( STDMETHODCALLTYPE *GetEndpointProxy )( 
            IWSDServiceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppProxy);
        
        END_INTERFACE
    } IWSDServiceProxyVtbl;

    interface IWSDServiceProxy
    {
        CONST_VTBL struct IWSDServiceProxyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDServiceProxy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDServiceProxy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDServiceProxy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDServiceProxy_GetMetadata(This,MetadataOut)	\
    ( (This)->lpVtbl -> GetMetadata(This,MetadataOut) ) 


#define IWSDServiceProxy_BeginGetMetadata(This,ppResult)	\
    ( (This)->lpVtbl -> BeginGetMetadata(This,ppResult) ) 

#define IWSDServiceProxy_EndGetMetadata(This,pResult,ppMetadata)	\
    ( (This)->lpVtbl -> EndGetMetadata(This,pResult,ppMetadata) ) 

#define IWSDServiceProxy_GetServiceMetadata(This,ppServiceMetadata)	\
    ( (This)->lpVtbl -> GetServiceMetadata(This,ppServiceMetadata) ) 

#define IWSDServiceProxy_SubscribeToOperation(This,pOperation,pUnknown,pAny,ppAny)	\
    ( (This)->lpVtbl -> SubscribeToOperation(This,pOperation,pUnknown,pAny,ppAny) ) 

#define IWSDServiceProxy_UnsubscribeToOperation(This,pOperation)	\
    ( (This)->lpVtbl -> UnsubscribeToOperation(This,pOperation) ) 

#define IWSDServiceProxy_SetEventingStatusCallback(This,pStatus)	\
    ( (This)->lpVtbl -> SetEventingStatusCallback(This,pStatus) ) 

#define IWSDServiceProxy_GetEndpointProxy(This,ppProxy)	\
    ( (This)->lpVtbl -> GetEndpointProxy(This,ppProxy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDServiceProxy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdclient_0000_0002 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0002_v0_0_s_ifspec;

#ifndef __IWSDServiceProxyEventing_INTERFACE_DEFINED__
#define __IWSDServiceProxyEventing_INTERFACE_DEFINED__

/* interface IWSDServiceProxyEventing */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDServiceProxyEventing;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9279d6d-1012-4a94-b8cc-fd35d2202bfe")
    IWSDServiceProxyEventing : public IWSDServiceProxy
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SubscribeToMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnknown,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginSubscribeToMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnknown,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndSubscribeToMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnsubscribeToMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [in] */ const WSDXML_ELEMENT *pAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginUnsubscribeToMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndUnsubscribeToMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenewMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginRenewMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndRenewMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatusForMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginGetStatusForMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetStatusForMultipleOperations( 
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDServiceProxyEventingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDServiceProxyEventing * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDServiceProxyEventing * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDServiceProxyEventing * This);
        
        DECLSPEC_XFGVIRT(IWSDMetadataExchange, GetMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMetadata )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **MetadataOut);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, BeginGetMetadata)
        HRESULT ( STDMETHODCALLTYPE *BeginGetMetadata )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, EndGetMetadata)
        HRESULT ( STDMETHODCALLTYPE *EndGetMetadata )( 
            IWSDServiceProxyEventing * This,
            /* [in] */ IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, GetServiceMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetServiceMetadata )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_SERVICE_METADATA **ppServiceMetadata);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, SubscribeToOperation)
        HRESULT ( STDMETHODCALLTYPE *SubscribeToOperation )( 
            IWSDServiceProxyEventing * This,
            /* [in] */ const WSD_OPERATION *pOperation,
            /* [in] */ IUnknown *pUnknown,
            /* [in] */ const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, UnsubscribeToOperation)
        HRESULT ( STDMETHODCALLTYPE *UnsubscribeToOperation )( 
            IWSDServiceProxyEventing * This,
            /* [in] */ const WSD_OPERATION *pOperation);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, SetEventingStatusCallback)
        HRESULT ( STDMETHODCALLTYPE *SetEventingStatusCallback )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDEventingStatus *pStatus);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxy, GetEndpointProxy)
        HRESULT ( STDMETHODCALLTYPE *GetEndpointProxy )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppProxy);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, SubscribeToMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *SubscribeToMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnknown,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, BeginSubscribeToMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *BeginSubscribeToMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pUnknown,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, EndSubscribeToMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *EndSubscribeToMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, UnsubscribeToMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *UnsubscribeToMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [in] */ const WSDXML_ELEMENT *pAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, BeginUnsubscribeToMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *BeginUnsubscribeToMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, EndUnsubscribeToMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *EndUnsubscribeToMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, RenewMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *RenewMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, BeginRenewMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *BeginRenewMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSD_EVENTING_EXPIRES *pExpires,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, EndRenewMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *EndRenewMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, GetStatusForMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *GetStatusForMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, BeginGetStatusForMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *BeginGetStatusForMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pAsyncState,
            /* [annotation][in] */ 
            _In_opt_  IWSDAsyncCallback *pAsyncCallback,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDServiceProxyEventing, EndGetStatusForMultipleOperations)
        HRESULT ( STDMETHODCALLTYPE *EndGetStatusForMultipleOperations )( 
            IWSDServiceProxyEventing * This,
            /* [annotation][in] */ 
            _In_reads_(dwOperationCount)  const WSD_OPERATION *pOperations,
            /* [in] */ DWORD dwOperationCount,
            /* [annotation][in] */ 
            _In_  IWSDAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_opt_  WSD_EVENTING_EXPIRES **ppExpires,
            /* [annotation][out] */ 
            _Outptr_opt_  WSDXML_ELEMENT **ppAny);
        
        END_INTERFACE
    } IWSDServiceProxyEventingVtbl;

    interface IWSDServiceProxyEventing
    {
        CONST_VTBL struct IWSDServiceProxyEventingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDServiceProxyEventing_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDServiceProxyEventing_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDServiceProxyEventing_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDServiceProxyEventing_GetMetadata(This,MetadataOut)	\
    ( (This)->lpVtbl -> GetMetadata(This,MetadataOut) ) 


#define IWSDServiceProxyEventing_BeginGetMetadata(This,ppResult)	\
    ( (This)->lpVtbl -> BeginGetMetadata(This,ppResult) ) 

#define IWSDServiceProxyEventing_EndGetMetadata(This,pResult,ppMetadata)	\
    ( (This)->lpVtbl -> EndGetMetadata(This,pResult,ppMetadata) ) 

#define IWSDServiceProxyEventing_GetServiceMetadata(This,ppServiceMetadata)	\
    ( (This)->lpVtbl -> GetServiceMetadata(This,ppServiceMetadata) ) 

#define IWSDServiceProxyEventing_SubscribeToOperation(This,pOperation,pUnknown,pAny,ppAny)	\
    ( (This)->lpVtbl -> SubscribeToOperation(This,pOperation,pUnknown,pAny,ppAny) ) 

#define IWSDServiceProxyEventing_UnsubscribeToOperation(This,pOperation)	\
    ( (This)->lpVtbl -> UnsubscribeToOperation(This,pOperation) ) 

#define IWSDServiceProxyEventing_SetEventingStatusCallback(This,pStatus)	\
    ( (This)->lpVtbl -> SetEventingStatusCallback(This,pStatus) ) 

#define IWSDServiceProxyEventing_GetEndpointProxy(This,ppProxy)	\
    ( (This)->lpVtbl -> GetEndpointProxy(This,ppProxy) ) 


#define IWSDServiceProxyEventing_SubscribeToMultipleOperations(This,pOperations,dwOperationCount,pUnknown,pExpires,pAny,ppExpires,ppAny)	\
    ( (This)->lpVtbl -> SubscribeToMultipleOperations(This,pOperations,dwOperationCount,pUnknown,pExpires,pAny,ppExpires,ppAny) ) 

#define IWSDServiceProxyEventing_BeginSubscribeToMultipleOperations(This,pOperations,dwOperationCount,pUnknown,pExpires,pAny,pAsyncState,pAsyncCallback,ppResult)	\
    ( (This)->lpVtbl -> BeginSubscribeToMultipleOperations(This,pOperations,dwOperationCount,pUnknown,pExpires,pAny,pAsyncState,pAsyncCallback,ppResult) ) 

#define IWSDServiceProxyEventing_EndSubscribeToMultipleOperations(This,pOperations,dwOperationCount,pResult,ppExpires,ppAny)	\
    ( (This)->lpVtbl -> EndSubscribeToMultipleOperations(This,pOperations,dwOperationCount,pResult,ppExpires,ppAny) ) 

#define IWSDServiceProxyEventing_UnsubscribeToMultipleOperations(This,pOperations,dwOperationCount,pAny)	\
    ( (This)->lpVtbl -> UnsubscribeToMultipleOperations(This,pOperations,dwOperationCount,pAny) ) 

#define IWSDServiceProxyEventing_BeginUnsubscribeToMultipleOperations(This,pOperations,dwOperationCount,pAny,pAsyncState,pAsyncCallback,ppResult)	\
    ( (This)->lpVtbl -> BeginUnsubscribeToMultipleOperations(This,pOperations,dwOperationCount,pAny,pAsyncState,pAsyncCallback,ppResult) ) 

#define IWSDServiceProxyEventing_EndUnsubscribeToMultipleOperations(This,pOperations,dwOperationCount,pResult)	\
    ( (This)->lpVtbl -> EndUnsubscribeToMultipleOperations(This,pOperations,dwOperationCount,pResult) ) 

#define IWSDServiceProxyEventing_RenewMultipleOperations(This,pOperations,dwOperationCount,pExpires,pAny,ppExpires,ppAny)	\
    ( (This)->lpVtbl -> RenewMultipleOperations(This,pOperations,dwOperationCount,pExpires,pAny,ppExpires,ppAny) ) 

#define IWSDServiceProxyEventing_BeginRenewMultipleOperations(This,pOperations,dwOperationCount,pExpires,pAny,pAsyncState,pAsyncCallback,ppResult)	\
    ( (This)->lpVtbl -> BeginRenewMultipleOperations(This,pOperations,dwOperationCount,pExpires,pAny,pAsyncState,pAsyncCallback,ppResult) ) 

#define IWSDServiceProxyEventing_EndRenewMultipleOperations(This,pOperations,dwOperationCount,pResult,ppExpires,ppAny)	\
    ( (This)->lpVtbl -> EndRenewMultipleOperations(This,pOperations,dwOperationCount,pResult,ppExpires,ppAny) ) 

#define IWSDServiceProxyEventing_GetStatusForMultipleOperations(This,pOperations,dwOperationCount,pAny,ppExpires,ppAny)	\
    ( (This)->lpVtbl -> GetStatusForMultipleOperations(This,pOperations,dwOperationCount,pAny,ppExpires,ppAny) ) 

#define IWSDServiceProxyEventing_BeginGetStatusForMultipleOperations(This,pOperations,dwOperationCount,pAny,pAsyncState,pAsyncCallback,ppResult)	\
    ( (This)->lpVtbl -> BeginGetStatusForMultipleOperations(This,pOperations,dwOperationCount,pAny,pAsyncState,pAsyncCallback,ppResult) ) 

#define IWSDServiceProxyEventing_EndGetStatusForMultipleOperations(This,pOperations,dwOperationCount,pResult,ppExpires,ppAny)	\
    ( (This)->lpVtbl -> EndGetStatusForMultipleOperations(This,pOperations,dwOperationCount,pResult,ppExpires,ppAny) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDServiceProxyEventing_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdclient_0000_0003 */
/* [local] */ 

#endif


extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0003_v0_0_s_ifspec;

#ifndef __IWSDDeviceProxy_INTERFACE_DEFINED__
#define __IWSDDeviceProxy_INTERFACE_DEFINED__

/* interface IWSDDeviceProxy */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDDeviceProxy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eee0c031-c578-4c0e-9a3b-973c35f409db")
    IWSDDeviceProxy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszDeviceId,
            /* [in] */ IWSDAddress *pDeviceAddress,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszLocalId,
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDXMLContext *pContext,
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDDeviceProxy *pSponsor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginGetMetadata( 
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetMetadata( 
            /* [in] */ IWSDAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHostMetadata( 
            /* [annotation][out] */ 
            _Outptr_  WSD_HOST_METADATA **ppHostMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThisModelMetadata( 
            /* [annotation][out] */ 
            _Outptr_  WSD_THIS_MODEL_METADATA **ppManufacturerMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThisDeviceMetadata( 
            /* [annotation][out] */ 
            _Outptr_  WSD_THIS_DEVICE_METADATA **ppThisDeviceMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllMetadata( 
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **ppMetadata) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceProxyById( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszServiceId,
            /* [annotation][out] */ 
            _Outptr_  IWSDServiceProxy **ppServiceProxy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceProxyByType( 
            /* [in] */ const WSDXML_NAME *pType,
            /* [annotation][out] */ 
            _Outptr_  IWSDServiceProxy **ppServiceProxy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndpointProxy( 
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppProxy) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDDeviceProxyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDDeviceProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDDeviceProxy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDDeviceProxy * This);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            IWSDDeviceProxy * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszDeviceId,
            /* [in] */ IWSDAddress *pDeviceAddress,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszLocalId,
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDXMLContext *pContext,
            /* [annotation][optional][in] */ 
            _In_opt_  IWSDDeviceProxy *pSponsor);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, BeginGetMetadata)
        HRESULT ( STDMETHODCALLTYPE *BeginGetMetadata )( 
            IWSDDeviceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDAsyncResult **ppResult);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, EndGetMetadata)
        HRESULT ( STDMETHODCALLTYPE *EndGetMetadata )( 
            IWSDDeviceProxy * This,
            /* [in] */ IWSDAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetHostMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetHostMetadata )( 
            IWSDDeviceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_HOST_METADATA **ppHostMetadata);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetThisModelMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetThisModelMetadata )( 
            IWSDDeviceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_THIS_MODEL_METADATA **ppManufacturerMetadata);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetThisDeviceMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetThisDeviceMetadata )( 
            IWSDDeviceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_THIS_DEVICE_METADATA **ppThisDeviceMetadata);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetAllMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetAllMetadata )( 
            IWSDDeviceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_METADATA_SECTION_LIST **ppMetadata);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetServiceProxyById)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProxyById )( 
            IWSDDeviceProxy * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszServiceId,
            /* [annotation][out] */ 
            _Outptr_  IWSDServiceProxy **ppServiceProxy);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetServiceProxyByType)
        HRESULT ( STDMETHODCALLTYPE *GetServiceProxyByType )( 
            IWSDDeviceProxy * This,
            /* [in] */ const WSDXML_NAME *pType,
            /* [annotation][out] */ 
            _Outptr_  IWSDServiceProxy **ppServiceProxy);
        
        DECLSPEC_XFGVIRT(IWSDDeviceProxy, GetEndpointProxy)
        HRESULT ( STDMETHODCALLTYPE *GetEndpointProxy )( 
            IWSDDeviceProxy * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppProxy);
        
        END_INTERFACE
    } IWSDDeviceProxyVtbl;

    interface IWSDDeviceProxy
    {
        CONST_VTBL struct IWSDDeviceProxyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDDeviceProxy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDDeviceProxy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDDeviceProxy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDDeviceProxy_Init(This,pszDeviceId,pDeviceAddress,pszLocalId,pContext,pSponsor)	\
    ( (This)->lpVtbl -> Init(This,pszDeviceId,pDeviceAddress,pszLocalId,pContext,pSponsor) ) 

#define IWSDDeviceProxy_BeginGetMetadata(This,ppResult)	\
    ( (This)->lpVtbl -> BeginGetMetadata(This,ppResult) ) 

#define IWSDDeviceProxy_EndGetMetadata(This,pResult)	\
    ( (This)->lpVtbl -> EndGetMetadata(This,pResult) ) 

#define IWSDDeviceProxy_GetHostMetadata(This,ppHostMetadata)	\
    ( (This)->lpVtbl -> GetHostMetadata(This,ppHostMetadata) ) 

#define IWSDDeviceProxy_GetThisModelMetadata(This,ppManufacturerMetadata)	\
    ( (This)->lpVtbl -> GetThisModelMetadata(This,ppManufacturerMetadata) ) 

#define IWSDDeviceProxy_GetThisDeviceMetadata(This,ppThisDeviceMetadata)	\
    ( (This)->lpVtbl -> GetThisDeviceMetadata(This,ppThisDeviceMetadata) ) 

#define IWSDDeviceProxy_GetAllMetadata(This,ppMetadata)	\
    ( (This)->lpVtbl -> GetAllMetadata(This,ppMetadata) ) 

#define IWSDDeviceProxy_GetServiceProxyById(This,pszServiceId,ppServiceProxy)	\
    ( (This)->lpVtbl -> GetServiceProxyById(This,pszServiceId,ppServiceProxy) ) 

#define IWSDDeviceProxy_GetServiceProxyByType(This,pType,ppServiceProxy)	\
    ( (This)->lpVtbl -> GetServiceProxyByType(This,pType,ppServiceProxy) ) 

#define IWSDDeviceProxy_GetEndpointProxy(This,ppProxy)	\
    ( (This)->lpVtbl -> GetEndpointProxy(This,ppProxy) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDDeviceProxy_INTERFACE_DEFINED__ */


#ifndef __IWSDAsyncResult_INTERFACE_DEFINED__
#define __IWSDAsyncResult_INTERFACE_DEFINED__

/* interface IWSDAsyncResult */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDAsyncResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("11a9852a-8dd8-423e-b537-9356db4fbfb8")
    IWSDAsyncResult : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCallback( 
            /* [in] */ IWSDAsyncCallback *pCallback,
            /* [in] */ IUnknown *pAsyncState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWaitHandle( 
            /* [in] */ HANDLE hWaitHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasCompleted( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAsyncState( 
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppAsyncState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEvent( 
            /* [annotation][out] */ 
            _Out_  WSD_EVENT *pEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndpointProxy( 
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppEndpoint) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDAsyncResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDAsyncResult * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, SetCallback)
        HRESULT ( STDMETHODCALLTYPE *SetCallback )( 
            IWSDAsyncResult * This,
            /* [in] */ IWSDAsyncCallback *pCallback,
            /* [in] */ IUnknown *pAsyncState);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, SetWaitHandle)
        HRESULT ( STDMETHODCALLTYPE *SetWaitHandle )( 
            IWSDAsyncResult * This,
            /* [in] */ HANDLE hWaitHandle);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, HasCompleted)
        HRESULT ( STDMETHODCALLTYPE *HasCompleted )( 
            IWSDAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, GetAsyncState)
        HRESULT ( STDMETHODCALLTYPE *GetAsyncState )( 
            IWSDAsyncResult * This,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppAsyncState);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IWSDAsyncResult * This);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            IWSDAsyncResult * This,
            /* [annotation][out] */ 
            _Out_  WSD_EVENT *pEvent);
        
        DECLSPEC_XFGVIRT(IWSDAsyncResult, GetEndpointProxy)
        HRESULT ( STDMETHODCALLTYPE *GetEndpointProxy )( 
            IWSDAsyncResult * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDEndpointProxy **ppEndpoint);
        
        END_INTERFACE
    } IWSDAsyncResultVtbl;

    interface IWSDAsyncResult
    {
        CONST_VTBL struct IWSDAsyncResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDAsyncResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDAsyncResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDAsyncResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDAsyncResult_SetCallback(This,pCallback,pAsyncState)	\
    ( (This)->lpVtbl -> SetCallback(This,pCallback,pAsyncState) ) 

#define IWSDAsyncResult_SetWaitHandle(This,hWaitHandle)	\
    ( (This)->lpVtbl -> SetWaitHandle(This,hWaitHandle) ) 

#define IWSDAsyncResult_HasCompleted(This)	\
    ( (This)->lpVtbl -> HasCompleted(This) ) 

#define IWSDAsyncResult_GetAsyncState(This,ppAsyncState)	\
    ( (This)->lpVtbl -> GetAsyncState(This,ppAsyncState) ) 

#define IWSDAsyncResult_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IWSDAsyncResult_GetEvent(This,pEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,pEvent) ) 

#define IWSDAsyncResult_GetEndpointProxy(This,ppEndpoint)	\
    ( (This)->lpVtbl -> GetEndpointProxy(This,ppEndpoint) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDAsyncResult_INTERFACE_DEFINED__ */


#ifndef __IWSDAsyncCallback_INTERFACE_DEFINED__
#define __IWSDAsyncCallback_INTERFACE_DEFINED__

/* interface IWSDAsyncCallback */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDAsyncCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a63e109d-ce72-49e2-ba98-e845f5ee1666")
    IWSDAsyncCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AsyncOperationComplete( 
            /* [in] */ IWSDAsyncResult *pAsyncResult,
            /* [in] */ IUnknown *pAsyncState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDAsyncCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDAsyncCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDAsyncCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDAsyncCallback * This);
        
        DECLSPEC_XFGVIRT(IWSDAsyncCallback, AsyncOperationComplete)
        HRESULT ( STDMETHODCALLTYPE *AsyncOperationComplete )( 
            IWSDAsyncCallback * This,
            /* [in] */ IWSDAsyncResult *pAsyncResult,
            /* [in] */ IUnknown *pAsyncState);
        
        END_INTERFACE
    } IWSDAsyncCallbackVtbl;

    interface IWSDAsyncCallback
    {
        CONST_VTBL struct IWSDAsyncCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDAsyncCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDAsyncCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDAsyncCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDAsyncCallback_AsyncOperationComplete(This,pAsyncResult,pAsyncState)	\
    ( (This)->lpVtbl -> AsyncOperationComplete(This,pAsyncResult,pAsyncState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDAsyncCallback_INTERFACE_DEFINED__ */


#ifndef __IWSDEventingStatus_INTERFACE_DEFINED__
#define __IWSDEventingStatus_INTERFACE_DEFINED__

/* interface IWSDEventingStatus */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDEventingStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("49b17f52-637a-407a-ae99-fbe82a4d38c0")
    IWSDEventingStatus : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE SubscriptionRenewed( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSubscriptionAction) = 0;
        
        virtual void STDMETHODCALLTYPE SubscriptionRenewalFailed( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSubscriptionAction,
            /* [in] */ HRESULT hr) = 0;
        
        virtual void STDMETHODCALLTYPE SubscriptionEnded( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSubscriptionAction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDEventingStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDEventingStatus * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDEventingStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDEventingStatus * This);
        
        DECLSPEC_XFGVIRT(IWSDEventingStatus, SubscriptionRenewed)
        void ( STDMETHODCALLTYPE *SubscriptionRenewed )( 
            IWSDEventingStatus * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSubscriptionAction);
        
        DECLSPEC_XFGVIRT(IWSDEventingStatus, SubscriptionRenewalFailed)
        void ( STDMETHODCALLTYPE *SubscriptionRenewalFailed )( 
            IWSDEventingStatus * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSubscriptionAction,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(IWSDEventingStatus, SubscriptionEnded)
        void ( STDMETHODCALLTYPE *SubscriptionEnded )( 
            IWSDEventingStatus * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszSubscriptionAction);
        
        END_INTERFACE
    } IWSDEventingStatusVtbl;

    interface IWSDEventingStatus
    {
        CONST_VTBL struct IWSDEventingStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDEventingStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDEventingStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDEventingStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDEventingStatus_SubscriptionRenewed(This,pszSubscriptionAction)	\
    ( (This)->lpVtbl -> SubscriptionRenewed(This,pszSubscriptionAction) ) 

#define IWSDEventingStatus_SubscriptionRenewalFailed(This,pszSubscriptionAction,hr)	\
    ( (This)->lpVtbl -> SubscriptionRenewalFailed(This,pszSubscriptionAction,hr) ) 

#define IWSDEventingStatus_SubscriptionEnded(This,pszSubscriptionAction)	\
    ( (This)->lpVtbl -> SubscriptionEnded(This,pszSubscriptionAction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDEventingStatus_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdclient_0000_0008 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdclient_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


