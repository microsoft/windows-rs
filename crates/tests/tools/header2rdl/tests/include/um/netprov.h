

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

#ifndef __netprov_h__
#define __netprov_h__

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

#ifndef __IProvisioningDomain_FWD_DEFINED__
#define __IProvisioningDomain_FWD_DEFINED__
typedef interface IProvisioningDomain IProvisioningDomain;

#endif 	/* __IProvisioningDomain_FWD_DEFINED__ */


#ifndef __IProvisioningProfileWireless_FWD_DEFINED__
#define __IProvisioningProfileWireless_FWD_DEFINED__
typedef interface IProvisioningProfileWireless IProvisioningProfileWireless;

#endif 	/* __IProvisioningProfileWireless_FWD_DEFINED__ */


#ifndef __NetProvisioning_FWD_DEFINED__
#define __NetProvisioning_FWD_DEFINED__

#ifdef __cplusplus
typedef class NetProvisioning NetProvisioning;
#else
typedef struct NetProvisioning NetProvisioning;
#endif /* __cplusplus */

#endif 	/* __NetProvisioning_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "prsht.h"
#include "msxml.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_netprov_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if ( _MSC_VER >= 800 )
#pragma warning(disable:4201)
#endif




extern RPC_IF_HANDLE __MIDL_itf_netprov_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netprov_0000_0000_v0_0_s_ifspec;

#ifndef __IProvisioningDomain_INTERFACE_DEFINED__
#define __IProvisioningDomain_INTERFACE_DEFINED__

/* interface IProvisioningDomain */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProvisioningDomain;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c96fbd50-24dd-11d8-89fb-00904b2ea9c6")
    IProvisioningDomain : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszwPathToFolder) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [string][in] */ __RPC__in_string LPCWSTR pszwDomain,
            /* [string][in] */ __RPC__in_string LPCWSTR pszwLanguage,
            /* [string][in] */ __RPC__in_string LPCWSTR pszwXPathQuery,
            /* [out] */ __RPC__deref_out_opt IXMLDOMNodeList **Nodes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvisioningDomainVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvisioningDomain * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvisioningDomain * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvisioningDomain * This);
        
        DECLSPEC_XFGVIRT(IProvisioningDomain, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IProvisioningDomain * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszwPathToFolder);
        
        DECLSPEC_XFGVIRT(IProvisioningDomain, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            __RPC__in IProvisioningDomain * This,
            /* [string][in] */ __RPC__in_string LPCWSTR pszwDomain,
            /* [string][in] */ __RPC__in_string LPCWSTR pszwLanguage,
            /* [string][in] */ __RPC__in_string LPCWSTR pszwXPathQuery,
            /* [out] */ __RPC__deref_out_opt IXMLDOMNodeList **Nodes);
        
        END_INTERFACE
    } IProvisioningDomainVtbl;

    interface IProvisioningDomain
    {
        CONST_VTBL struct IProvisioningDomainVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvisioningDomain_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvisioningDomain_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvisioningDomain_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvisioningDomain_Add(This,pszwPathToFolder)	\
    ( (This)->lpVtbl -> Add(This,pszwPathToFolder) ) 

#define IProvisioningDomain_Query(This,pszwDomain,pszwLanguage,pszwXPathQuery,Nodes)	\
    ( (This)->lpVtbl -> Query(This,pszwDomain,pszwLanguage,pszwXPathQuery,Nodes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvisioningDomain_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_netprov_0000_0001 */
/* [local] */ 

//
// WZC Wireless CreateProfile private error codes (returned in *pulStatus)
// WZC_PROFILE_SUCCESS is applicable only if the HRESULT returned is also
// S_OK
//
#define     WZC_PROFILE_SUCCESS                                         0
#define     WZC_PROFILE_XML_ERROR_NO_VERSION                            1
#define     WZC_PROFILE_XML_ERROR_BAD_VERSION                           2
#define     WZC_PROFILE_XML_ERROR_UNSUPPORTED_VERSION                   3
#define     WZC_PROFILE_XML_ERROR_SSID_NOT_FOUND                        4
#define     WZC_PROFILE_XML_ERROR_BAD_SSID                              5
#define     WZC_PROFILE_XML_ERROR_CONNECTION_TYPE                       6
#define     WZC_PROFILE_XML_ERROR_AUTHENTICATION                        7
#define     WZC_PROFILE_XML_ERROR_ENCRYPTION                            8
#define     WZC_PROFILE_XML_ERROR_KEY_PROVIDED_AUTOMATICALLY            9
#define     WZC_PROFILE_XML_ERROR_1X_ENABLED                            10
#define     WZC_PROFILE_XML_ERROR_EAP_METHOD                            11
#define     WZC_PROFILE_XML_ERROR_BAD_KEY_INDEX                         12
#define     WZC_PROFILE_XML_ERROR_KEY_INDEX_RANGE                       13
#define     WZC_PROFILE_XML_ERROR_BAD_NETWORK_KEY                       14
#define     WZC_PROFILE_CONFIG_ERROR_INVALID_AUTH_FOR_CONNECTION_TYPE   15
#define     WZC_PROFILE_CONFIG_ERROR_INVALID_ENCRYPTION_FOR_AUTHMODE    16
#define     WZC_PROFILE_CONFIG_ERROR_KEY_REQUIRED                       17
#define     WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_REQUIRED                 18
#define     WZC_PROFILE_CONFIG_ERROR_KEY_INDEX_NOT_APPLICABLE           19
#define     WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED                     20
#define     WZC_PROFILE_CONFIG_ERROR_1X_NOT_ALLOWED_KEY_REQUIRED        21
#define     WZC_PROFILE_CONFIG_ERROR_1X_NOT_ENABLED_KEY_PROVIDED        22
#define     WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_REQUIRED                23
#define     WZC_PROFILE_CONFIG_ERROR_EAP_METHOD_NOT_APPLICABLE          24
#define     WZC_PROFILE_CONFIG_ERROR_WPA_NOT_SUPPORTED                  25
#define     WZC_PROFILE_CONFIG_ERROR_WPA_ENCRYPTION_NOT_SUPPORTED       26
#define     WZC_PROFILE_SET_ERROR_DUPLICATE_NETWORK                     27
#define     WZC_PROFILE_SET_ERROR_MEMORY_ALLOCATION                     28
#define     WZC_PROFILE_SET_ERROR_READING_1X_CONFIG                     29
#define     WZC_PROFILE_SET_ERROR_WRITING_1X_CONFIG                     30
#define     WZC_PROFILE_SET_ERROR_WRITING_WZC_CFG                       31
#define     WZC_PROFILE_API_ERROR_NOT_SUPPORTED                         32
#define     WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_XML                    33
#define     WZC_PROFILE_API_ERROR_FAILED_TO_LOAD_SCHEMA                 34
#define     WZC_PROFILE_API_ERROR_XML_VALIDATION_FAILED                 35
#define     WZC_PROFILE_API_ERROR_INTERNAL                              36


extern RPC_IF_HANDLE __MIDL_itf_netprov_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netprov_0000_0001_v0_0_s_ifspec;

#ifndef __IProvisioningProfileWireless_INTERFACE_DEFINED__
#define __IProvisioningProfileWireless_INTERFACE_DEFINED__

/* interface IProvisioningProfileWireless */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IProvisioningProfileWireless;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c96fbd51-24dd-11d8-89fb-00904b2ea9c6")
    IProvisioningProfileWireless : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateProfile( 
            /* [in] */ __RPC__in BSTR bstrXMLWirelessConfigProfile,
            /* [in] */ __RPC__in BSTR bstrXMLConnectionConfigProfile,
            /* [in] */ __RPC__in GUID *pAdapterInstanceGuid,
            /* [out] */ __RPC__out ULONG *pulStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProvisioningProfileWirelessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProvisioningProfileWireless * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProvisioningProfileWireless * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProvisioningProfileWireless * This);
        
        DECLSPEC_XFGVIRT(IProvisioningProfileWireless, CreateProfile)
        HRESULT ( STDMETHODCALLTYPE *CreateProfile )( 
            __RPC__in IProvisioningProfileWireless * This,
            /* [in] */ __RPC__in BSTR bstrXMLWirelessConfigProfile,
            /* [in] */ __RPC__in BSTR bstrXMLConnectionConfigProfile,
            /* [in] */ __RPC__in GUID *pAdapterInstanceGuid,
            /* [out] */ __RPC__out ULONG *pulStatus);
        
        END_INTERFACE
    } IProvisioningProfileWirelessVtbl;

    interface IProvisioningProfileWireless
    {
        CONST_VTBL struct IProvisioningProfileWirelessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProvisioningProfileWireless_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProvisioningProfileWireless_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProvisioningProfileWireless_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProvisioningProfileWireless_CreateProfile(This,bstrXMLWirelessConfigProfile,bstrXMLConnectionConfigProfile,pAdapterInstanceGuid,pulStatus)	\
    ( (This)->lpVtbl -> CreateProfile(This,bstrXMLWirelessConfigProfile,bstrXMLConnectionConfigProfile,pAdapterInstanceGuid,pulStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProvisioningProfileWireless_INTERFACE_DEFINED__ */



#ifndef __NETPROVLib_LIBRARY_DEFINED__
#define __NETPROVLib_LIBRARY_DEFINED__

/* library NETPROVLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_NETPROVLib;

EXTERN_C const CLSID CLSID_NetProvisioning;

#ifdef __cplusplus

class DECLSPEC_UUID("2aa2b5fe-b846-4d07-810c-b21ee45320e3")
NetProvisioning;
#endif
#endif /* __NETPROVLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_netprov_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_netprov_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_netprov_0000_0003_v0_0_s_ifspec;

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


