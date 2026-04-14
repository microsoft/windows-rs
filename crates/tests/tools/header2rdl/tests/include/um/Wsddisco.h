

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

#ifndef __wsddisco_h__
#define __wsddisco_h__

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

#ifndef __IWSDiscoveryProvider_FWD_DEFINED__
#define __IWSDiscoveryProvider_FWD_DEFINED__
typedef interface IWSDiscoveryProvider IWSDiscoveryProvider;

#endif 	/* __IWSDiscoveryProvider_FWD_DEFINED__ */


#ifndef __IWSDiscoveryProviderNotify_FWD_DEFINED__
#define __IWSDiscoveryProviderNotify_FWD_DEFINED__
typedef interface IWSDiscoveryProviderNotify IWSDiscoveryProviderNotify;

#endif 	/* __IWSDiscoveryProviderNotify_FWD_DEFINED__ */


#ifndef __IWSDiscoveredService_FWD_DEFINED__
#define __IWSDiscoveredService_FWD_DEFINED__
typedef interface IWSDiscoveredService IWSDiscoveredService;

#endif 	/* __IWSDiscoveredService_FWD_DEFINED__ */


#ifndef __IWSDiscoveryPublisher_FWD_DEFINED__
#define __IWSDiscoveryPublisher_FWD_DEFINED__
typedef interface IWSDiscoveryPublisher IWSDiscoveryPublisher;

#endif 	/* __IWSDiscoveryPublisher_FWD_DEFINED__ */


#ifndef __IWSDiscoveryPublisherNotify_FWD_DEFINED__
#define __IWSDiscoveryPublisherNotify_FWD_DEFINED__
typedef interface IWSDiscoveryPublisherNotify IWSDiscoveryPublisherNotify;

#endif 	/* __IWSDiscoveryPublisherNotify_FWD_DEFINED__ */


#ifndef __IWSDScopeMatchingRule_FWD_DEFINED__
#define __IWSDScopeMatchingRule_FWD_DEFINED__
typedef interface IWSDScopeMatchingRule IWSDScopeMatchingRule;

#endif 	/* __IWSDScopeMatchingRule_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "wsdxmldom.h"
#include "wsdtypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wsddisco_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)







HRESULT WINAPI
WSDCreateDiscoveryProvider(
    IWSDXMLContext* pContext,
    _Outptr_ IWSDiscoveryProvider** ppProvider);
#if (WINVER >= _WIN32_WINNT_WIN7)
HRESULT WINAPI
WSDCreateDiscoveryProvider2(
    IWSDXMLContext* pContext,
    _In_reads_opt_(dwConfigParamCount) WSD_CONFIG_PARAM* pConfigParams,
    DWORD dwConfigParamCount,
    _Outptr_ IWSDiscoveryProvider** ppProvider);
#endif


extern RPC_IF_HANDLE __MIDL_itf_wsddisco_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsddisco_0000_0000_v0_0_s_ifspec;

#ifndef __IWSDiscoveryProvider_INTERFACE_DEFINED__
#define __IWSDiscoveryProvider_INTERFACE_DEFINED__

/* interface IWSDiscoveryProvider */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDiscoveryProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8FFC8E55-F0EB-480F-88B7-B435DD281D45")
    IWSDiscoveryProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAddressFamily( 
            /* [in] */ DWORD dwAddressFamily) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Attach( 
            /* [in] */ IWSDiscoveryProviderNotify *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Detach( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SearchById( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SearchByAddress( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszAddress,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SearchByType( 
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszMatchBy,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXMLContext( 
            /* [annotation][out] */ 
            _Outptr_  IWSDXMLContext **ppContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDiscoveryProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDiscoveryProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDiscoveryProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDiscoveryProvider * This);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, SetAddressFamily)
        HRESULT ( STDMETHODCALLTYPE *SetAddressFamily )( 
            IWSDiscoveryProvider * This,
            /* [in] */ DWORD dwAddressFamily);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, Attach)
        HRESULT ( STDMETHODCALLTYPE *Attach )( 
            IWSDiscoveryProvider * This,
            /* [in] */ IWSDiscoveryProviderNotify *pSink);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, Detach)
        HRESULT ( STDMETHODCALLTYPE *Detach )( 
            IWSDiscoveryProvider * This);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, SearchById)
        HRESULT ( STDMETHODCALLTYPE *SearchById )( 
            IWSDiscoveryProvider * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, SearchByAddress)
        HRESULT ( STDMETHODCALLTYPE *SearchByAddress )( 
            IWSDiscoveryProvider * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszAddress,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, SearchByType)
        HRESULT ( STDMETHODCALLTYPE *SearchByType )( 
            IWSDiscoveryProvider * This,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszMatchBy,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProvider, GetXMLContext)
        HRESULT ( STDMETHODCALLTYPE *GetXMLContext )( 
            IWSDiscoveryProvider * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDXMLContext **ppContext);
        
        END_INTERFACE
    } IWSDiscoveryProviderVtbl;

    interface IWSDiscoveryProvider
    {
        CONST_VTBL struct IWSDiscoveryProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDiscoveryProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDiscoveryProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDiscoveryProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDiscoveryProvider_SetAddressFamily(This,dwAddressFamily)	\
    ( (This)->lpVtbl -> SetAddressFamily(This,dwAddressFamily) ) 

#define IWSDiscoveryProvider_Attach(This,pSink)	\
    ( (This)->lpVtbl -> Attach(This,pSink) ) 

#define IWSDiscoveryProvider_Detach(This)	\
    ( (This)->lpVtbl -> Detach(This) ) 

#define IWSDiscoveryProvider_SearchById(This,pszId,pszTag)	\
    ( (This)->lpVtbl -> SearchById(This,pszId,pszTag) ) 

#define IWSDiscoveryProvider_SearchByAddress(This,pszAddress,pszTag)	\
    ( (This)->lpVtbl -> SearchByAddress(This,pszAddress,pszTag) ) 

#define IWSDiscoveryProvider_SearchByType(This,pTypesList,pScopesList,pszMatchBy,pszTag)	\
    ( (This)->lpVtbl -> SearchByType(This,pTypesList,pScopesList,pszMatchBy,pszTag) ) 

#define IWSDiscoveryProvider_GetXMLContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetXMLContext(This,ppContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDiscoveryProvider_INTERFACE_DEFINED__ */


#ifndef __IWSDiscoveryProviderNotify_INTERFACE_DEFINED__
#define __IWSDiscoveryProviderNotify_INTERFACE_DEFINED__

/* interface IWSDiscoveryProviderNotify */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDiscoveryProviderNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("73ee3ced-b6e6-4329-a546-3e8ad46563d2")
    IWSDiscoveryProviderNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ IWSDiscoveredService *pService) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ IWSDiscoveredService *pService) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SearchFailed( 
            /* [in] */ HRESULT hr,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SearchComplete( 
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDiscoveryProviderNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDiscoveryProviderNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDiscoveryProviderNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDiscoveryProviderNotify * This);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProviderNotify, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            IWSDiscoveryProviderNotify * This,
            /* [in] */ IWSDiscoveredService *pService);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProviderNotify, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IWSDiscoveryProviderNotify * This,
            /* [in] */ IWSDiscoveredService *pService);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProviderNotify, SearchFailed)
        HRESULT ( STDMETHODCALLTYPE *SearchFailed )( 
            IWSDiscoveryProviderNotify * This,
            /* [in] */ HRESULT hr,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryProviderNotify, SearchComplete)
        HRESULT ( STDMETHODCALLTYPE *SearchComplete )( 
            IWSDiscoveryProviderNotify * This,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszTag);
        
        END_INTERFACE
    } IWSDiscoveryProviderNotifyVtbl;

    interface IWSDiscoveryProviderNotify
    {
        CONST_VTBL struct IWSDiscoveryProviderNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDiscoveryProviderNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDiscoveryProviderNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDiscoveryProviderNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDiscoveryProviderNotify_Add(This,pService)	\
    ( (This)->lpVtbl -> Add(This,pService) ) 

#define IWSDiscoveryProviderNotify_Remove(This,pService)	\
    ( (This)->lpVtbl -> Remove(This,pService) ) 

#define IWSDiscoveryProviderNotify_SearchFailed(This,hr,pszTag)	\
    ( (This)->lpVtbl -> SearchFailed(This,hr,pszTag) ) 

#define IWSDiscoveryProviderNotify_SearchComplete(This,pszTag)	\
    ( (This)->lpVtbl -> SearchComplete(This,pszTag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDiscoveryProviderNotify_INTERFACE_DEFINED__ */


#ifndef __IWSDiscoveredService_INTERFACE_DEFINED__
#define __IWSDiscoveredService_INTERFACE_DEFINED__

/* interface IWSDiscoveredService */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDiscoveredService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4BAD8A3B-B374-4420-9632-AAC945B374AA")
    IWSDiscoveredService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEndpointReference( 
            /* [annotation][out] */ 
            _Outptr_  WSD_ENDPOINT_REFERENCE **ppEndpointReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTypes( 
            /* [annotation][out] */ 
            _Outptr_  WSD_NAME_LIST **ppTypesList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScopes( 
            /* [annotation][out] */ 
            _Outptr_  WSD_URI_LIST **ppScopesList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXAddrs( 
            /* [annotation][out] */ 
            _Outptr_  WSD_URI_LIST **ppXAddrsList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadataVersion( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullMetadataVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExtendedDiscoXML( 
            /* [annotation][out] */ 
            _Outptr_  WSDXML_ELEMENT **ppHeaderAny,
            /* [annotation][out] */ 
            _Outptr_  WSDXML_ELEMENT **ppBodyAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProbeResolveTag( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszTag) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRemoteTransportAddress( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszRemoteTransportAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocalTransportAddress( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszLocalTransportAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLocalInterfaceGUID( 
            /* [annotation][out] */ 
            _Out_  GUID *pGuid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInstanceId( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullInstanceId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDiscoveredServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDiscoveredService * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDiscoveredService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDiscoveredService * This);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetEndpointReference)
        HRESULT ( STDMETHODCALLTYPE *GetEndpointReference )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_ENDPOINT_REFERENCE **ppEndpointReference);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetTypes)
        HRESULT ( STDMETHODCALLTYPE *GetTypes )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_NAME_LIST **ppTypesList);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetScopes)
        HRESULT ( STDMETHODCALLTYPE *GetScopes )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_URI_LIST **ppScopesList);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetXAddrs)
        HRESULT ( STDMETHODCALLTYPE *GetXAddrs )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  WSD_URI_LIST **ppXAddrsList);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetMetadataVersion)
        HRESULT ( STDMETHODCALLTYPE *GetMetadataVersion )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullMetadataVersion);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetExtendedDiscoXML)
        HRESULT ( STDMETHODCALLTYPE *GetExtendedDiscoXML )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  WSDXML_ELEMENT **ppHeaderAny,
            /* [annotation][out] */ 
            _Outptr_  WSDXML_ELEMENT **ppBodyAny);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetProbeResolveTag)
        HRESULT ( STDMETHODCALLTYPE *GetProbeResolveTag )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszTag);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetRemoteTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteTransportAddress )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszRemoteTransportAddress);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetLocalTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *GetLocalTransportAddress )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszLocalTransportAddress);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetLocalInterfaceGUID)
        HRESULT ( STDMETHODCALLTYPE *GetLocalInterfaceGUID )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Out_  GUID *pGuid);
        
        DECLSPEC_XFGVIRT(IWSDiscoveredService, GetInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetInstanceId )( 
            IWSDiscoveredService * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullInstanceId);
        
        END_INTERFACE
    } IWSDiscoveredServiceVtbl;

    interface IWSDiscoveredService
    {
        CONST_VTBL struct IWSDiscoveredServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDiscoveredService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDiscoveredService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDiscoveredService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDiscoveredService_GetEndpointReference(This,ppEndpointReference)	\
    ( (This)->lpVtbl -> GetEndpointReference(This,ppEndpointReference) ) 

#define IWSDiscoveredService_GetTypes(This,ppTypesList)	\
    ( (This)->lpVtbl -> GetTypes(This,ppTypesList) ) 

#define IWSDiscoveredService_GetScopes(This,ppScopesList)	\
    ( (This)->lpVtbl -> GetScopes(This,ppScopesList) ) 

#define IWSDiscoveredService_GetXAddrs(This,ppXAddrsList)	\
    ( (This)->lpVtbl -> GetXAddrs(This,ppXAddrsList) ) 

#define IWSDiscoveredService_GetMetadataVersion(This,pullMetadataVersion)	\
    ( (This)->lpVtbl -> GetMetadataVersion(This,pullMetadataVersion) ) 

#define IWSDiscoveredService_GetExtendedDiscoXML(This,ppHeaderAny,ppBodyAny)	\
    ( (This)->lpVtbl -> GetExtendedDiscoXML(This,ppHeaderAny,ppBodyAny) ) 

#define IWSDiscoveredService_GetProbeResolveTag(This,ppszTag)	\
    ( (This)->lpVtbl -> GetProbeResolveTag(This,ppszTag) ) 

#define IWSDiscoveredService_GetRemoteTransportAddress(This,ppszRemoteTransportAddress)	\
    ( (This)->lpVtbl -> GetRemoteTransportAddress(This,ppszRemoteTransportAddress) ) 

#define IWSDiscoveredService_GetLocalTransportAddress(This,ppszLocalTransportAddress)	\
    ( (This)->lpVtbl -> GetLocalTransportAddress(This,ppszLocalTransportAddress) ) 

#define IWSDiscoveredService_GetLocalInterfaceGUID(This,pGuid)	\
    ( (This)->lpVtbl -> GetLocalInterfaceGUID(This,pGuid) ) 

#define IWSDiscoveredService_GetInstanceId(This,pullInstanceId)	\
    ( (This)->lpVtbl -> GetInstanceId(This,pullInstanceId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDiscoveredService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsddisco_0000_0003 */
/* [local] */ 

HRESULT WINAPI
WSDCreateDiscoveryPublisher(
    IWSDXMLContext* pContext,
    _Outptr_ IWSDiscoveryPublisher** ppPublisher);
#if (WINVER >= _WIN32_WINNT_WIN7)
HRESULT WINAPI
WSDCreateDiscoveryPublisher2(
    IWSDXMLContext* pContext,
    _In_reads_opt_(dwConfigParamCount) WSD_CONFIG_PARAM* pConfigParams,
    DWORD dwConfigParamCount,
    _Outptr_ IWSDiscoveryPublisher** ppPublisher);
#endif


extern RPC_IF_HANDLE __MIDL_itf_wsddisco_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsddisco_0000_0003_v0_0_s_ifspec;

#ifndef __IWSDiscoveryPublisher_INTERFACE_DEFINED__
#define __IWSDiscoveryPublisher_INTERFACE_DEFINED__

/* interface IWSDiscoveryPublisher */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDiscoveryPublisher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE01E1A8-3FF9-4148-8116-057CC616FE13")
    IWSDiscoveryPublisher : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAddressFamily( 
            /* [in] */ DWORD dwAddressFamily) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterNotificationSink( 
            /* [in] */ IWSDiscoveryPublisherNotify *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterNotificationSink( 
            /* [in] */ IWSDiscoveryPublisherNotify *pSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Publish( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnPublish( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MatchProbe( 
            /* [in] */ const WSD_SOAP_MESSAGE *pProbeMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MatchResolve( 
            /* [in] */ const WSD_SOAP_MESSAGE *pResolveMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PublishEx( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pHeaderAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pReferenceParameterAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pPolicyAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pEndpointReferenceAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MatchProbeEx( 
            /* [in] */ const WSD_SOAP_MESSAGE *pProbeMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pHeaderAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pReferenceParameterAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pPolicyAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pEndpointReferenceAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MatchResolveEx( 
            /* [in] */ const WSD_SOAP_MESSAGE *pResolveMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pHeaderAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pReferenceParameterAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pPolicyAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pEndpointReferenceAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterScopeMatchingRule( 
            /* [in] */ IWSDScopeMatchingRule *pScopeMatchingRule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterScopeMatchingRule( 
            /* [in] */ IWSDScopeMatchingRule *pScopeMatchingRule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXMLContext( 
            /* [annotation][out] */ 
            _Outptr_  IWSDXMLContext **ppContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDiscoveryPublisherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDiscoveryPublisher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDiscoveryPublisher * This);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, SetAddressFamily)
        HRESULT ( STDMETHODCALLTYPE *SetAddressFamily )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ DWORD dwAddressFamily);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, RegisterNotificationSink)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotificationSink )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ IWSDiscoveryPublisherNotify *pSink);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, UnRegisterNotificationSink)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterNotificationSink )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ IWSDiscoveryPublisherNotify *pSink);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, Publish)
        HRESULT ( STDMETHODCALLTYPE *Publish )( 
            IWSDiscoveryPublisher * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, UnPublish)
        HRESULT ( STDMETHODCALLTYPE *UnPublish )( 
            IWSDiscoveryPublisher * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, MatchProbe)
        HRESULT ( STDMETHODCALLTYPE *MatchProbe )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ const WSD_SOAP_MESSAGE *pProbeMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, MatchResolve)
        HRESULT ( STDMETHODCALLTYPE *MatchResolve )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ const WSD_SOAP_MESSAGE *pResolveMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, PublishEx)
        HRESULT ( STDMETHODCALLTYPE *PublishEx )( 
            IWSDiscoveryPublisher * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pHeaderAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pReferenceParameterAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pPolicyAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pEndpointReferenceAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, MatchProbeEx)
        HRESULT ( STDMETHODCALLTYPE *MatchProbeEx )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ const WSD_SOAP_MESSAGE *pProbeMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pHeaderAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pReferenceParameterAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pPolicyAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pEndpointReferenceAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, MatchResolveEx)
        HRESULT ( STDMETHODCALLTYPE *MatchResolveEx )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ const WSD_SOAP_MESSAGE *pResolveMessage,
            /* [in] */ IWSDMessageParameters *pMessageParameters,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszId,
            /* [in] */ ULONGLONG ullMetadataVersion,
            /* [in] */ ULONGLONG ullInstanceId,
            /* [in] */ ULONGLONG ullMessageNumber,
            /* [annotation][optional][in] */ 
            _In_opt_  LPCWSTR pszSessionId,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_NAME_LIST *pTypesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pScopesList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSD_URI_LIST *pXAddrsList,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pHeaderAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pReferenceParameterAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pPolicyAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pEndpointReferenceAny,
            /* [annotation][optional][in] */ 
            _In_opt_  const WSDXML_ELEMENT *pAny);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, RegisterScopeMatchingRule)
        HRESULT ( STDMETHODCALLTYPE *RegisterScopeMatchingRule )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ IWSDScopeMatchingRule *pScopeMatchingRule);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, UnRegisterScopeMatchingRule)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterScopeMatchingRule )( 
            IWSDiscoveryPublisher * This,
            /* [in] */ IWSDScopeMatchingRule *pScopeMatchingRule);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisher, GetXMLContext)
        HRESULT ( STDMETHODCALLTYPE *GetXMLContext )( 
            IWSDiscoveryPublisher * This,
            /* [annotation][out] */ 
            _Outptr_  IWSDXMLContext **ppContext);
        
        END_INTERFACE
    } IWSDiscoveryPublisherVtbl;

    interface IWSDiscoveryPublisher
    {
        CONST_VTBL struct IWSDiscoveryPublisherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDiscoveryPublisher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDiscoveryPublisher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDiscoveryPublisher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDiscoveryPublisher_SetAddressFamily(This,dwAddressFamily)	\
    ( (This)->lpVtbl -> SetAddressFamily(This,dwAddressFamily) ) 

#define IWSDiscoveryPublisher_RegisterNotificationSink(This,pSink)	\
    ( (This)->lpVtbl -> RegisterNotificationSink(This,pSink) ) 

#define IWSDiscoveryPublisher_UnRegisterNotificationSink(This,pSink)	\
    ( (This)->lpVtbl -> UnRegisterNotificationSink(This,pSink) ) 

#define IWSDiscoveryPublisher_Publish(This,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList)	\
    ( (This)->lpVtbl -> Publish(This,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList) ) 

#define IWSDiscoveryPublisher_UnPublish(This,pszId,ullInstanceId,ullMessageNumber,pszSessionId,pAny)	\
    ( (This)->lpVtbl -> UnPublish(This,pszId,ullInstanceId,ullMessageNumber,pszSessionId,pAny) ) 

#define IWSDiscoveryPublisher_MatchProbe(This,pProbeMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList)	\
    ( (This)->lpVtbl -> MatchProbe(This,pProbeMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList) ) 

#define IWSDiscoveryPublisher_MatchResolve(This,pResolveMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList)	\
    ( (This)->lpVtbl -> MatchResolve(This,pResolveMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList) ) 

#define IWSDiscoveryPublisher_PublishEx(This,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList,pHeaderAny,pReferenceParameterAny,pPolicyAny,pEndpointReferenceAny,pAny)	\
    ( (This)->lpVtbl -> PublishEx(This,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList,pHeaderAny,pReferenceParameterAny,pPolicyAny,pEndpointReferenceAny,pAny) ) 

#define IWSDiscoveryPublisher_MatchProbeEx(This,pProbeMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList,pHeaderAny,pReferenceParameterAny,pPolicyAny,pEndpointReferenceAny,pAny)	\
    ( (This)->lpVtbl -> MatchProbeEx(This,pProbeMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList,pHeaderAny,pReferenceParameterAny,pPolicyAny,pEndpointReferenceAny,pAny) ) 

#define IWSDiscoveryPublisher_MatchResolveEx(This,pResolveMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList,pHeaderAny,pReferenceParameterAny,pPolicyAny,pEndpointReferenceAny,pAny)	\
    ( (This)->lpVtbl -> MatchResolveEx(This,pResolveMessage,pMessageParameters,pszId,ullMetadataVersion,ullInstanceId,ullMessageNumber,pszSessionId,pTypesList,pScopesList,pXAddrsList,pHeaderAny,pReferenceParameterAny,pPolicyAny,pEndpointReferenceAny,pAny) ) 

#define IWSDiscoveryPublisher_RegisterScopeMatchingRule(This,pScopeMatchingRule)	\
    ( (This)->lpVtbl -> RegisterScopeMatchingRule(This,pScopeMatchingRule) ) 

#define IWSDiscoveryPublisher_UnRegisterScopeMatchingRule(This,pScopeMatchingRule)	\
    ( (This)->lpVtbl -> UnRegisterScopeMatchingRule(This,pScopeMatchingRule) ) 

#define IWSDiscoveryPublisher_GetXMLContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetXMLContext(This,ppContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDiscoveryPublisher_INTERFACE_DEFINED__ */


#ifndef __IWSDiscoveryPublisherNotify_INTERFACE_DEFINED__
#define __IWSDiscoveryPublisherNotify_INTERFACE_DEFINED__

/* interface IWSDiscoveryPublisherNotify */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDiscoveryPublisherNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e67651b0-337a-4b3c-9758-733388568251")
    IWSDiscoveryPublisherNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProbeHandler( 
            /* [in] */ const WSD_SOAP_MESSAGE *pSoap,
            /* [in] */ IWSDMessageParameters *pMessageParameters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResolveHandler( 
            /* [in] */ const WSD_SOAP_MESSAGE *pSoap,
            /* [in] */ IWSDMessageParameters *pMessageParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDiscoveryPublisherNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDiscoveryPublisherNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDiscoveryPublisherNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDiscoveryPublisherNotify * This);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisherNotify, ProbeHandler)
        HRESULT ( STDMETHODCALLTYPE *ProbeHandler )( 
            IWSDiscoveryPublisherNotify * This,
            /* [in] */ const WSD_SOAP_MESSAGE *pSoap,
            /* [in] */ IWSDMessageParameters *pMessageParameters);
        
        DECLSPEC_XFGVIRT(IWSDiscoveryPublisherNotify, ResolveHandler)
        HRESULT ( STDMETHODCALLTYPE *ResolveHandler )( 
            IWSDiscoveryPublisherNotify * This,
            /* [in] */ const WSD_SOAP_MESSAGE *pSoap,
            /* [in] */ IWSDMessageParameters *pMessageParameters);
        
        END_INTERFACE
    } IWSDiscoveryPublisherNotifyVtbl;

    interface IWSDiscoveryPublisherNotify
    {
        CONST_VTBL struct IWSDiscoveryPublisherNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDiscoveryPublisherNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDiscoveryPublisherNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDiscoveryPublisherNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDiscoveryPublisherNotify_ProbeHandler(This,pSoap,pMessageParameters)	\
    ( (This)->lpVtbl -> ProbeHandler(This,pSoap,pMessageParameters) ) 

#define IWSDiscoveryPublisherNotify_ResolveHandler(This,pSoap,pMessageParameters)	\
    ( (This)->lpVtbl -> ResolveHandler(This,pSoap,pMessageParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDiscoveryPublisherNotify_INTERFACE_DEFINED__ */


#ifndef __IWSDScopeMatchingRule_INTERFACE_DEFINED__
#define __IWSDScopeMatchingRule_INTERFACE_DEFINED__

/* interface IWSDScopeMatchingRule */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDScopeMatchingRule;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fcafe424-fef5-481a-bd9f-33ce0574256f")
    IWSDScopeMatchingRule : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetScopeRule( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszScopeMatchingRule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MatchScopes( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScope1,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScope2,
            /* [annotation][out] */ 
            _Out_  BOOL *pfMatch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDScopeMatchingRuleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDScopeMatchingRule * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDScopeMatchingRule * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDScopeMatchingRule * This);
        
        DECLSPEC_XFGVIRT(IWSDScopeMatchingRule, GetScopeRule)
        HRESULT ( STDMETHODCALLTYPE *GetScopeRule )( 
            IWSDScopeMatchingRule * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszScopeMatchingRule);
        
        DECLSPEC_XFGVIRT(IWSDScopeMatchingRule, MatchScopes)
        HRESULT ( STDMETHODCALLTYPE *MatchScopes )( 
            IWSDScopeMatchingRule * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScope1,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszScope2,
            /* [annotation][out] */ 
            _Out_  BOOL *pfMatch);
        
        END_INTERFACE
    } IWSDScopeMatchingRuleVtbl;

    interface IWSDScopeMatchingRule
    {
        CONST_VTBL struct IWSDScopeMatchingRuleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDScopeMatchingRule_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDScopeMatchingRule_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDScopeMatchingRule_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDScopeMatchingRule_GetScopeRule(This,ppszScopeMatchingRule)	\
    ( (This)->lpVtbl -> GetScopeRule(This,ppszScopeMatchingRule) ) 

#define IWSDScopeMatchingRule_MatchScopes(This,pszScope1,pszScope2,pfMatch)	\
    ( (This)->lpVtbl -> MatchScopes(This,pszScope1,pszScope2,pfMatch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDScopeMatchingRule_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsddisco_0000_0006 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wsddisco_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsddisco_0000_0006_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


