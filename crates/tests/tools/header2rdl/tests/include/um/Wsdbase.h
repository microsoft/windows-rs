

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

#ifndef __wsdbase_h__
#define __wsdbase_h__

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

#ifndef __IWSDAddress_FWD_DEFINED__
#define __IWSDAddress_FWD_DEFINED__
typedef interface IWSDAddress IWSDAddress;

#endif 	/* __IWSDAddress_FWD_DEFINED__ */


#ifndef __IWSDTransportAddress_FWD_DEFINED__
#define __IWSDTransportAddress_FWD_DEFINED__
typedef interface IWSDTransportAddress IWSDTransportAddress;

#endif 	/* __IWSDTransportAddress_FWD_DEFINED__ */


#ifndef __IWSDMessageParameters_FWD_DEFINED__
#define __IWSDMessageParameters_FWD_DEFINED__
typedef interface IWSDMessageParameters IWSDMessageParameters;

#endif 	/* __IWSDMessageParameters_FWD_DEFINED__ */


#ifndef __IWSDUdpMessageParameters_FWD_DEFINED__
#define __IWSDUdpMessageParameters_FWD_DEFINED__
typedef interface IWSDUdpMessageParameters IWSDUdpMessageParameters;

#endif 	/* __IWSDUdpMessageParameters_FWD_DEFINED__ */


#ifndef __IWSDUdpAddress_FWD_DEFINED__
#define __IWSDUdpAddress_FWD_DEFINED__
typedef interface IWSDUdpAddress IWSDUdpAddress;

#endif 	/* __IWSDUdpAddress_FWD_DEFINED__ */


#ifndef __IWSDHttpMessageParameters_FWD_DEFINED__
#define __IWSDHttpMessageParameters_FWD_DEFINED__
typedef interface IWSDHttpMessageParameters IWSDHttpMessageParameters;

#endif 	/* __IWSDHttpMessageParameters_FWD_DEFINED__ */


#ifndef __IWSDHttpAddress_FWD_DEFINED__
#define __IWSDHttpAddress_FWD_DEFINED__
typedef interface IWSDHttpAddress IWSDHttpAddress;

#endif 	/* __IWSDHttpAddress_FWD_DEFINED__ */


#ifndef __IWSDSSLClientCertificate_FWD_DEFINED__
#define __IWSDSSLClientCertificate_FWD_DEFINED__
typedef interface IWSDSSLClientCertificate IWSDSSLClientCertificate;

#endif 	/* __IWSDSSLClientCertificate_FWD_DEFINED__ */


#ifndef __IWSDHttpAuthParameters_FWD_DEFINED__
#define __IWSDHttpAuthParameters_FWD_DEFINED__
typedef interface IWSDHttpAuthParameters IWSDHttpAuthParameters;

#endif 	/* __IWSDHttpAuthParameters_FWD_DEFINED__ */


#ifndef __IWSDSignatureProperty_FWD_DEFINED__
#define __IWSDSignatureProperty_FWD_DEFINED__
typedef interface IWSDSignatureProperty IWSDSignatureProperty;

#endif 	/* __IWSDSignatureProperty_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "wincrypt.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wsdbase_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






#if (WINVER >= _WIN32_WINNT_WIN7)
typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_wsdbase_0000_0000_0001
    {
        WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE	= 1,
        WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE	= 2,
        WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH	= 3,
        WSD_SECURITY_SSL_SERVER_CERT_VALIDATION	= 4,
        WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION	= 5,
        WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT	= 6,
        WSD_SECURITY_COMPACTSIG_SIGNING_CERT	= 7,
        WSD_SECURITY_COMPACTSIG_VALIDATION	= 8,
        WSD_CONFIG_HOSTING_ADDRESSES	= 9,
        WSD_CONFIG_DEVICE_ADDRESSES	= 10,
        WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH	= 11,
        WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH	= 12,
        WSD_SECURITY_USE_HTTP_CLIENT_AUTH	= 13
    } 	WSD_CONFIG_PARAM_TYPE;

typedef struct _WSD_CONFIG_PARAM
    {
    WSD_CONFIG_PARAM_TYPE configParamType;
    PVOID pConfigData;
    DWORD dwConfigDataSize;
    } 	WSD_CONFIG_PARAM;

typedef struct _WSD_CONFIG_PARAM *PWSD_CONFIG_PARAM;

typedef struct _WSD_SECURITY_CERT_VALIDATION_V1
    {
    PCCERT_CONTEXT *certMatchArray;
    DWORD dwCertMatchArrayCount;
    HCERTSTORE hCertMatchStore;
    HCERTSTORE hCertIssuerStore;
    DWORD dwCertCheckOptions;
    } 	WSD_SECURITY_CERT_VALIDATION_V1;

#if (_WIN32_WINNT >= _WIN32_WINNT_WIN8)
typedef struct _WSD_SECURITY_CERT_VALIDATION
    {
    PCCERT_CONTEXT *certMatchArray;
    DWORD dwCertMatchArrayCount;
    HCERTSTORE hCertMatchStore;
    HCERTSTORE hCertIssuerStore;
    DWORD dwCertCheckOptions;
    LPCWSTR pszCNGHashAlgId;
    BYTE *pbCertHash;
    DWORD dwCertHashSize;
    } 	WSD_SECURITY_CERT_VALIDATION;

#else
typedef WSD_SECURITY_CERT_VALIDATION_V1 WSD_SECURITY_CERT_VALIDATION;
#endif
typedef WSD_SECURITY_CERT_VALIDATION *PWSD_SECURITY_CERT_VALIDATION;

typedef struct _WSD_SECURITY_SIGNATURE_VALIDATION
    {
    PCCERT_CONTEXT *signingCertArray;
    DWORD dwSigningCertArrayCount;
    HCERTSTORE hSigningCertStore;
    DWORD dwFlags;
    } 	WSD_SECURITY_SIGNATURE_VALIDATION;

typedef struct _WSD_SECURITY_SIGNATURE_VALIDATION *PWSD_SECURITY_SIGNATURE_VALIDATION;

#define WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS     0x0
#define WSDAPI_SSL_CERT_IGNORE_REVOCATION        0x1
#define WSDAPI_SSL_CERT_IGNORE_EXPIRY            0x2
#define WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE       0x4
#define WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA        0x8
#define WSDAPI_SSL_CERT_IGNORE_INVALID_CN        0x10
#define WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES    0x1
typedef DWORD WSD_SECURITY_HTTP_AUTH_SCHEMES;

typedef DWORD *PWSD_SECURITY_HTTP_AUTH_SCHEMES;

#define WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE  0x1
#define WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM       0x2
typedef struct _WSD_CONFIG_ADDRESSES
    {
    IWSDAddress **addresses;
    DWORD dwAddressCount;
    } 	WSD_CONFIG_ADDRESSES;

typedef struct _WSD_CONFIG_ADDRESSES *PWSD_CONFIG_ADDRESSES;

#endif
#define WSDAPI_ADDRESSFAMILY_IPV4 1
#define WSDAPI_ADDRESSFAMILY_IPV6 2


extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0000_v0_0_s_ifspec;

#ifndef __IWSDAddress_INTERFACE_DEFINED__
#define __IWSDAddress_INTERFACE_DEFINED__

/* interface IWSDAddress */
/* [local][uuid][object] */ 




EXTERN_C const IID IID_IWSDAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b9574c6c-12a6-4f74-93a1-3318ff605759")
    IWSDAddress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Serialize( 
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchLength)  LPWSTR pszBuffer,
            /* [in] */ DWORD cchLength,
            /* [in] */ BOOL fSafe) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deserialize( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDAddress * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDAddress * This);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            IWSDAddress * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchLength)  LPWSTR pszBuffer,
            /* [in] */ DWORD cchLength,
            /* [in] */ BOOL fSafe);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Deserialize)
        HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            IWSDAddress * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszBuffer);
        
        END_INTERFACE
    } IWSDAddressVtbl;

    interface IWSDAddress
    {
        CONST_VTBL struct IWSDAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDAddress_Serialize(This,pszBuffer,cchLength,fSafe)	\
    ( (This)->lpVtbl -> Serialize(This,pszBuffer,cchLength,fSafe) ) 

#define IWSDAddress_Deserialize(This,pszBuffer)	\
    ( (This)->lpVtbl -> Deserialize(This,pszBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDAddress_INTERFACE_DEFINED__ */


#ifndef __IWSDTransportAddress_INTERFACE_DEFINED__
#define __IWSDTransportAddress_INTERFACE_DEFINED__

/* interface IWSDTransportAddress */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IWSDTransportAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70d23498-4ee6-4340-a3df-d845d2235467")
    IWSDTransportAddress : public IWSDAddress
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPort( 
            /* [annotation][out] */ 
            _Out_  WORD *pwPort) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPort( 
            /* [in] */ WORD wPort) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportAddress( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportAddressEx( 
            /* [in] */ BOOL fSafe,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransportAddress( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszAddress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDTransportAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDTransportAddress * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDTransportAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDTransportAddress * This);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            IWSDTransportAddress * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchLength)  LPWSTR pszBuffer,
            /* [in] */ DWORD cchLength,
            /* [in] */ BOOL fSafe);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Deserialize)
        HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            IWSDTransportAddress * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszBuffer);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetPort)
        HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            IWSDTransportAddress * This,
            /* [annotation][out] */ 
            _Out_  WORD *pwPort);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, SetPort)
        HRESULT ( STDMETHODCALLTYPE *SetPort )( 
            IWSDTransportAddress * This,
            /* [in] */ WORD wPort);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAddress )( 
            IWSDTransportAddress * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetTransportAddressEx)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAddressEx )( 
            IWSDTransportAddress * This,
            /* [in] */ BOOL fSafe,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, SetTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *SetTransportAddress )( 
            IWSDTransportAddress * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszAddress);
        
        END_INTERFACE
    } IWSDTransportAddressVtbl;

    interface IWSDTransportAddress
    {
        CONST_VTBL struct IWSDTransportAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDTransportAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDTransportAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDTransportAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDTransportAddress_Serialize(This,pszBuffer,cchLength,fSafe)	\
    ( (This)->lpVtbl -> Serialize(This,pszBuffer,cchLength,fSafe) ) 

#define IWSDTransportAddress_Deserialize(This,pszBuffer)	\
    ( (This)->lpVtbl -> Deserialize(This,pszBuffer) ) 


#define IWSDTransportAddress_GetPort(This,pwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pwPort) ) 

#define IWSDTransportAddress_SetPort(This,wPort)	\
    ( (This)->lpVtbl -> SetPort(This,wPort) ) 

#define IWSDTransportAddress_GetTransportAddress(This,ppszAddress)	\
    ( (This)->lpVtbl -> GetTransportAddress(This,ppszAddress) ) 

#define IWSDTransportAddress_GetTransportAddressEx(This,fSafe,ppszAddress)	\
    ( (This)->lpVtbl -> GetTransportAddressEx(This,fSafe,ppszAddress) ) 

#define IWSDTransportAddress_SetTransportAddress(This,pszAddress)	\
    ( (This)->lpVtbl -> SetTransportAddress(This,pszAddress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDTransportAddress_INTERFACE_DEFINED__ */


#ifndef __IWSDMessageParameters_INTERFACE_DEFINED__
#define __IWSDMessageParameters_INTERFACE_DEFINED__

/* interface IWSDMessageParameters */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWSDMessageParameters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1fafe8a2-e6fc-4b80-b6cf-b7d45c416d7c")
    IWSDMessageParameters : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLocalAddress( 
            /* [out] */ __RPC__deref_out_opt IWSDAddress **ppAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLocalAddress( 
            /* [in] */ __RPC__in_opt IWSDAddress *pAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRemoteAddress( 
            /* [out] */ __RPC__deref_out_opt IWSDAddress **ppAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRemoteAddress( 
            /* [in] */ __RPC__in_opt IWSDAddress *pAddress) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLowerParameters( 
            /* [out] */ __RPC__deref_out_opt IWSDMessageParameters **ppTxParams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDMessageParametersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWSDMessageParameters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWSDMessageParameters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWSDMessageParameters * This);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetLocalAddress)
        HRESULT ( STDMETHODCALLTYPE *GetLocalAddress )( 
            __RPC__in IWSDMessageParameters * This,
            /* [out] */ __RPC__deref_out_opt IWSDAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, SetLocalAddress)
        HRESULT ( STDMETHODCALLTYPE *SetLocalAddress )( 
            __RPC__in IWSDMessageParameters * This,
            /* [in] */ __RPC__in_opt IWSDAddress *pAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetRemoteAddress)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteAddress )( 
            __RPC__in IWSDMessageParameters * This,
            /* [out] */ __RPC__deref_out_opt IWSDAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, SetRemoteAddress)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteAddress )( 
            __RPC__in IWSDMessageParameters * This,
            /* [in] */ __RPC__in_opt IWSDAddress *pAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetLowerParameters)
        HRESULT ( STDMETHODCALLTYPE *GetLowerParameters )( 
            __RPC__in IWSDMessageParameters * This,
            /* [out] */ __RPC__deref_out_opt IWSDMessageParameters **ppTxParams);
        
        END_INTERFACE
    } IWSDMessageParametersVtbl;

    interface IWSDMessageParameters
    {
        CONST_VTBL struct IWSDMessageParametersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDMessageParameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDMessageParameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDMessageParameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDMessageParameters_GetLocalAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> GetLocalAddress(This,ppAddress) ) 

#define IWSDMessageParameters_SetLocalAddress(This,pAddress)	\
    ( (This)->lpVtbl -> SetLocalAddress(This,pAddress) ) 

#define IWSDMessageParameters_GetRemoteAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> GetRemoteAddress(This,ppAddress) ) 

#define IWSDMessageParameters_SetRemoteAddress(This,pAddress)	\
    ( (This)->lpVtbl -> SetRemoteAddress(This,pAddress) ) 

#define IWSDMessageParameters_GetLowerParameters(This,ppTxParams)	\
    ( (This)->lpVtbl -> GetLowerParameters(This,ppTxParams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDMessageParameters_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdbase_0000_0003 */
/* [local] */ 

HRESULT WINAPI
WSDCreateUdpMessageParameters(
    IWSDUdpMessageParameters** ppTxParams);
typedef struct _WSDUdpRetransmitParams
    {
    ULONG ulSendDelay;
    ULONG ulRepeat;
    ULONG ulRepeatMinDelay;
    ULONG ulRepeatMaxDelay;
    ULONG ulRepeatUpperDelay;
    } 	WSDUdpRetransmitParams;



extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0003_v0_0_s_ifspec;

#ifndef __IWSDUdpMessageParameters_INTERFACE_DEFINED__
#define __IWSDUdpMessageParameters_INTERFACE_DEFINED__

/* interface IWSDUdpMessageParameters */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWSDUdpMessageParameters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9934149f-8f0c-447b-aa0b-73124b0ca7f0")
    IWSDUdpMessageParameters : public IWSDMessageParameters
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRetransmitParams( 
            /* [in] */ __RPC__in const WSDUdpRetransmitParams *pParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRetransmitParams( 
            /* [out] */ __RPC__out WSDUdpRetransmitParams *pParams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDUdpMessageParametersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWSDUdpMessageParameters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWSDUdpMessageParameters * This);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetLocalAddress)
        HRESULT ( STDMETHODCALLTYPE *GetLocalAddress )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [out] */ __RPC__deref_out_opt IWSDAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, SetLocalAddress)
        HRESULT ( STDMETHODCALLTYPE *SetLocalAddress )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [in] */ __RPC__in_opt IWSDAddress *pAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetRemoteAddress)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteAddress )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [out] */ __RPC__deref_out_opt IWSDAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, SetRemoteAddress)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteAddress )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [in] */ __RPC__in_opt IWSDAddress *pAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetLowerParameters)
        HRESULT ( STDMETHODCALLTYPE *GetLowerParameters )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [out] */ __RPC__deref_out_opt IWSDMessageParameters **ppTxParams);
        
        DECLSPEC_XFGVIRT(IWSDUdpMessageParameters, SetRetransmitParams)
        HRESULT ( STDMETHODCALLTYPE *SetRetransmitParams )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [in] */ __RPC__in const WSDUdpRetransmitParams *pParams);
        
        DECLSPEC_XFGVIRT(IWSDUdpMessageParameters, GetRetransmitParams)
        HRESULT ( STDMETHODCALLTYPE *GetRetransmitParams )( 
            __RPC__in IWSDUdpMessageParameters * This,
            /* [out] */ __RPC__out WSDUdpRetransmitParams *pParams);
        
        END_INTERFACE
    } IWSDUdpMessageParametersVtbl;

    interface IWSDUdpMessageParameters
    {
        CONST_VTBL struct IWSDUdpMessageParametersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDUdpMessageParameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDUdpMessageParameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDUdpMessageParameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDUdpMessageParameters_GetLocalAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> GetLocalAddress(This,ppAddress) ) 

#define IWSDUdpMessageParameters_SetLocalAddress(This,pAddress)	\
    ( (This)->lpVtbl -> SetLocalAddress(This,pAddress) ) 

#define IWSDUdpMessageParameters_GetRemoteAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> GetRemoteAddress(This,ppAddress) ) 

#define IWSDUdpMessageParameters_SetRemoteAddress(This,pAddress)	\
    ( (This)->lpVtbl -> SetRemoteAddress(This,pAddress) ) 

#define IWSDUdpMessageParameters_GetLowerParameters(This,ppTxParams)	\
    ( (This)->lpVtbl -> GetLowerParameters(This,ppTxParams) ) 


#define IWSDUdpMessageParameters_SetRetransmitParams(This,pParams)	\
    ( (This)->lpVtbl -> SetRetransmitParams(This,pParams) ) 

#define IWSDUdpMessageParameters_GetRetransmitParams(This,pParams)	\
    ( (This)->lpVtbl -> GetRetransmitParams(This,pParams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDUdpMessageParameters_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdbase_0000_0004 */
/* [local] */ 

HRESULT WINAPI
WSDCreateUdpAddress(
    _Outptr_ IWSDUdpAddress** ppAddress);
#if 1
// Forward declaration to avoid winsock2.h inclusion
#ifndef __CSADDR_DEFINED__
struct SOCKADDR_STORAGE;
#endif // __CSADDR_DEFINED__
#else  // 1
typedef void SOCKADDR_STORAGE;

#endif // 1
typedef 
enum _WSDUdpMessageType
    {
        ONE_WAY	= 0,
        TWO_WAY	= ( ONE_WAY + 1 ) 
    } 	WSDUdpMessageType;



extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0004_v0_0_s_ifspec;

#ifndef __IWSDUdpAddress_INTERFACE_DEFINED__
#define __IWSDUdpAddress_INTERFACE_DEFINED__

/* interface IWSDUdpAddress */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IWSDUdpAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("74d6124a-a441-4f78-a1eb-97a8d1996893")
    IWSDUdpAddress : public IWSDTransportAddress
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSockaddr( 
            /* [in] */ const SOCKADDR_STORAGE *pSockAddr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSockaddr( 
            /* [annotation][out] */ 
            _Out_  SOCKADDR_STORAGE *pSockAddr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetExclusive( 
            /* [in] */ BOOL fExclusive) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExclusive( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMessageType( 
            /* [in] */ WSDUdpMessageType messageType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMessageType( 
            /* [annotation][out] */ 
            _Out_  WSDUdpMessageType *pMessageType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTTL( 
            /* [in] */ DWORD dwTTL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTTL( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTTL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAlias( 
            /* [in] */ const GUID *pAlias) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlias( 
            /* [annotation][out] */ 
            _Out_  GUID *pAlias) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDUdpAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDUdpAddress * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDUdpAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDUdpAddress * This);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            IWSDUdpAddress * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchLength)  LPWSTR pszBuffer,
            /* [in] */ DWORD cchLength,
            /* [in] */ BOOL fSafe);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Deserialize)
        HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            IWSDUdpAddress * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszBuffer);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetPort)
        HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            IWSDUdpAddress * This,
            /* [annotation][out] */ 
            _Out_  WORD *pwPort);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, SetPort)
        HRESULT ( STDMETHODCALLTYPE *SetPort )( 
            IWSDUdpAddress * This,
            /* [in] */ WORD wPort);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAddress )( 
            IWSDUdpAddress * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetTransportAddressEx)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAddressEx )( 
            IWSDUdpAddress * This,
            /* [in] */ BOOL fSafe,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, SetTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *SetTransportAddress )( 
            IWSDUdpAddress * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszAddress);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, SetSockaddr)
        HRESULT ( STDMETHODCALLTYPE *SetSockaddr )( 
            IWSDUdpAddress * This,
            /* [in] */ const SOCKADDR_STORAGE *pSockAddr);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, GetSockaddr)
        HRESULT ( STDMETHODCALLTYPE *GetSockaddr )( 
            IWSDUdpAddress * This,
            /* [annotation][out] */ 
            _Out_  SOCKADDR_STORAGE *pSockAddr);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, SetExclusive)
        HRESULT ( STDMETHODCALLTYPE *SetExclusive )( 
            IWSDUdpAddress * This,
            /* [in] */ BOOL fExclusive);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, GetExclusive)
        HRESULT ( STDMETHODCALLTYPE *GetExclusive )( 
            IWSDUdpAddress * This);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, SetMessageType)
        HRESULT ( STDMETHODCALLTYPE *SetMessageType )( 
            IWSDUdpAddress * This,
            /* [in] */ WSDUdpMessageType messageType);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, GetMessageType)
        HRESULT ( STDMETHODCALLTYPE *GetMessageType )( 
            IWSDUdpAddress * This,
            /* [annotation][out] */ 
            _Out_  WSDUdpMessageType *pMessageType);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, SetTTL)
        HRESULT ( STDMETHODCALLTYPE *SetTTL )( 
            IWSDUdpAddress * This,
            /* [in] */ DWORD dwTTL);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, GetTTL)
        HRESULT ( STDMETHODCALLTYPE *GetTTL )( 
            IWSDUdpAddress * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTTL);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, SetAlias)
        HRESULT ( STDMETHODCALLTYPE *SetAlias )( 
            IWSDUdpAddress * This,
            /* [in] */ const GUID *pAlias);
        
        DECLSPEC_XFGVIRT(IWSDUdpAddress, GetAlias)
        HRESULT ( STDMETHODCALLTYPE *GetAlias )( 
            IWSDUdpAddress * This,
            /* [annotation][out] */ 
            _Out_  GUID *pAlias);
        
        END_INTERFACE
    } IWSDUdpAddressVtbl;

    interface IWSDUdpAddress
    {
        CONST_VTBL struct IWSDUdpAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDUdpAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDUdpAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDUdpAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDUdpAddress_Serialize(This,pszBuffer,cchLength,fSafe)	\
    ( (This)->lpVtbl -> Serialize(This,pszBuffer,cchLength,fSafe) ) 

#define IWSDUdpAddress_Deserialize(This,pszBuffer)	\
    ( (This)->lpVtbl -> Deserialize(This,pszBuffer) ) 


#define IWSDUdpAddress_GetPort(This,pwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pwPort) ) 

#define IWSDUdpAddress_SetPort(This,wPort)	\
    ( (This)->lpVtbl -> SetPort(This,wPort) ) 

#define IWSDUdpAddress_GetTransportAddress(This,ppszAddress)	\
    ( (This)->lpVtbl -> GetTransportAddress(This,ppszAddress) ) 

#define IWSDUdpAddress_GetTransportAddressEx(This,fSafe,ppszAddress)	\
    ( (This)->lpVtbl -> GetTransportAddressEx(This,fSafe,ppszAddress) ) 

#define IWSDUdpAddress_SetTransportAddress(This,pszAddress)	\
    ( (This)->lpVtbl -> SetTransportAddress(This,pszAddress) ) 


#define IWSDUdpAddress_SetSockaddr(This,pSockAddr)	\
    ( (This)->lpVtbl -> SetSockaddr(This,pSockAddr) ) 

#define IWSDUdpAddress_GetSockaddr(This,pSockAddr)	\
    ( (This)->lpVtbl -> GetSockaddr(This,pSockAddr) ) 

#define IWSDUdpAddress_SetExclusive(This,fExclusive)	\
    ( (This)->lpVtbl -> SetExclusive(This,fExclusive) ) 

#define IWSDUdpAddress_GetExclusive(This)	\
    ( (This)->lpVtbl -> GetExclusive(This) ) 

#define IWSDUdpAddress_SetMessageType(This,messageType)	\
    ( (This)->lpVtbl -> SetMessageType(This,messageType) ) 

#define IWSDUdpAddress_GetMessageType(This,pMessageType)	\
    ( (This)->lpVtbl -> GetMessageType(This,pMessageType) ) 

#define IWSDUdpAddress_SetTTL(This,dwTTL)	\
    ( (This)->lpVtbl -> SetTTL(This,dwTTL) ) 

#define IWSDUdpAddress_GetTTL(This,pdwTTL)	\
    ( (This)->lpVtbl -> GetTTL(This,pdwTTL) ) 

#define IWSDUdpAddress_SetAlias(This,pAlias)	\
    ( (This)->lpVtbl -> SetAlias(This,pAlias) ) 

#define IWSDUdpAddress_GetAlias(This,pAlias)	\
    ( (This)->lpVtbl -> GetAlias(This,pAlias) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDUdpAddress_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdbase_0000_0005 */
/* [local] */ 

HRESULT WINAPI
WSDCreateHttpMessageParameters(
    _Outptr_ IWSDHttpMessageParameters** ppTxParams);


extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0005_v0_0_s_ifspec;

#ifndef __IWSDHttpMessageParameters_INTERFACE_DEFINED__
#define __IWSDHttpMessageParameters_INTERFACE_DEFINED__

/* interface IWSDHttpMessageParameters */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IWSDHttpMessageParameters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("540bd122-5c83-4dec-b396-ea62a2697fdf")
    IWSDHttpMessageParameters : public IWSDMessageParameters
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInboundHttpHeaders( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInboundHttpHeaders( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutboundHttpHeaders( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutboundHttpHeaders( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszHeaders) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetID( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetID( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContext( 
            /* [in] */ IUnknown *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContext( 
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDHttpMessageParametersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDHttpMessageParameters * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDHttpMessageParameters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDHttpMessageParameters * This);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetLocalAddress)
        HRESULT ( STDMETHODCALLTYPE *GetLocalAddress )( 
            IWSDHttpMessageParameters * This,
            /* [out] */ IWSDAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, SetLocalAddress)
        HRESULT ( STDMETHODCALLTYPE *SetLocalAddress )( 
            IWSDHttpMessageParameters * This,
            /* [in] */ IWSDAddress *pAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetRemoteAddress)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteAddress )( 
            IWSDHttpMessageParameters * This,
            /* [out] */ IWSDAddress **ppAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, SetRemoteAddress)
        HRESULT ( STDMETHODCALLTYPE *SetRemoteAddress )( 
            IWSDHttpMessageParameters * This,
            /* [in] */ IWSDAddress *pAddress);
        
        DECLSPEC_XFGVIRT(IWSDMessageParameters, GetLowerParameters)
        HRESULT ( STDMETHODCALLTYPE *GetLowerParameters )( 
            IWSDHttpMessageParameters * This,
            /* [out] */ IWSDMessageParameters **ppTxParams);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, SetInboundHttpHeaders)
        HRESULT ( STDMETHODCALLTYPE *SetInboundHttpHeaders )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszHeaders);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, GetInboundHttpHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetInboundHttpHeaders )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszHeaders);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, SetOutboundHttpHeaders)
        HRESULT ( STDMETHODCALLTYPE *SetOutboundHttpHeaders )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszHeaders);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, GetOutboundHttpHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetOutboundHttpHeaders )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszHeaders);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, SetID)
        HRESULT ( STDMETHODCALLTYPE *SetID )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszId);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, GetID)
        HRESULT ( STDMETHODCALLTYPE *GetID )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszId);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, SetContext)
        HRESULT ( STDMETHODCALLTYPE *SetContext )( 
            IWSDHttpMessageParameters * This,
            /* [in] */ IUnknown *pContext);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, GetContext)
        HRESULT ( STDMETHODCALLTYPE *GetContext )( 
            IWSDHttpMessageParameters * This,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppContext);
        
        DECLSPEC_XFGVIRT(IWSDHttpMessageParameters, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IWSDHttpMessageParameters * This);
        
        END_INTERFACE
    } IWSDHttpMessageParametersVtbl;

    interface IWSDHttpMessageParameters
    {
        CONST_VTBL struct IWSDHttpMessageParametersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDHttpMessageParameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDHttpMessageParameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDHttpMessageParameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDHttpMessageParameters_GetLocalAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> GetLocalAddress(This,ppAddress) ) 

#define IWSDHttpMessageParameters_SetLocalAddress(This,pAddress)	\
    ( (This)->lpVtbl -> SetLocalAddress(This,pAddress) ) 

#define IWSDHttpMessageParameters_GetRemoteAddress(This,ppAddress)	\
    ( (This)->lpVtbl -> GetRemoteAddress(This,ppAddress) ) 

#define IWSDHttpMessageParameters_SetRemoteAddress(This,pAddress)	\
    ( (This)->lpVtbl -> SetRemoteAddress(This,pAddress) ) 

#define IWSDHttpMessageParameters_GetLowerParameters(This,ppTxParams)	\
    ( (This)->lpVtbl -> GetLowerParameters(This,ppTxParams) ) 


#define IWSDHttpMessageParameters_SetInboundHttpHeaders(This,pszHeaders)	\
    ( (This)->lpVtbl -> SetInboundHttpHeaders(This,pszHeaders) ) 

#define IWSDHttpMessageParameters_GetInboundHttpHeaders(This,ppszHeaders)	\
    ( (This)->lpVtbl -> GetInboundHttpHeaders(This,ppszHeaders) ) 

#define IWSDHttpMessageParameters_SetOutboundHttpHeaders(This,pszHeaders)	\
    ( (This)->lpVtbl -> SetOutboundHttpHeaders(This,pszHeaders) ) 

#define IWSDHttpMessageParameters_GetOutboundHttpHeaders(This,ppszHeaders)	\
    ( (This)->lpVtbl -> GetOutboundHttpHeaders(This,ppszHeaders) ) 

#define IWSDHttpMessageParameters_SetID(This,pszId)	\
    ( (This)->lpVtbl -> SetID(This,pszId) ) 

#define IWSDHttpMessageParameters_GetID(This,ppszId)	\
    ( (This)->lpVtbl -> GetID(This,ppszId) ) 

#define IWSDHttpMessageParameters_SetContext(This,pContext)	\
    ( (This)->lpVtbl -> SetContext(This,pContext) ) 

#define IWSDHttpMessageParameters_GetContext(This,ppContext)	\
    ( (This)->lpVtbl -> GetContext(This,ppContext) ) 

#define IWSDHttpMessageParameters_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDHttpMessageParameters_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdbase_0000_0006 */
/* [local] */ 

HRESULT WINAPI
WSDCreateHttpAddress(
    _Outptr_ IWSDHttpAddress** ppAddress);


extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0006_v0_0_s_ifspec;

#ifndef __IWSDHttpAddress_INTERFACE_DEFINED__
#define __IWSDHttpAddress_INTERFACE_DEFINED__

/* interface IWSDHttpAddress */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IWSDHttpAddress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d09ac7bd-2a3e-4b85-8605-2737ff3e4ea0")
    IWSDHttpAddress : public IWSDTransportAddress
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSecure( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSecure( 
            /* [in] */ BOOL fSecure) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPath( 
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPath( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDHttpAddressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDHttpAddress * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDHttpAddress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDHttpAddress * This);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Serialize)
        HRESULT ( STDMETHODCALLTYPE *Serialize )( 
            IWSDHttpAddress * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cchLength)  LPWSTR pszBuffer,
            /* [in] */ DWORD cchLength,
            /* [in] */ BOOL fSafe);
        
        DECLSPEC_XFGVIRT(IWSDAddress, Deserialize)
        HRESULT ( STDMETHODCALLTYPE *Deserialize )( 
            IWSDHttpAddress * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszBuffer);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetPort)
        HRESULT ( STDMETHODCALLTYPE *GetPort )( 
            IWSDHttpAddress * This,
            /* [annotation][out] */ 
            _Out_  WORD *pwPort);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, SetPort)
        HRESULT ( STDMETHODCALLTYPE *SetPort )( 
            IWSDHttpAddress * This,
            /* [in] */ WORD wPort);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAddress )( 
            IWSDHttpAddress * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, GetTransportAddressEx)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAddressEx )( 
            IWSDHttpAddress * This,
            /* [in] */ BOOL fSafe,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszAddress);
        
        DECLSPEC_XFGVIRT(IWSDTransportAddress, SetTransportAddress)
        HRESULT ( STDMETHODCALLTYPE *SetTransportAddress )( 
            IWSDHttpAddress * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszAddress);
        
        DECLSPEC_XFGVIRT(IWSDHttpAddress, GetSecure)
        HRESULT ( STDMETHODCALLTYPE *GetSecure )( 
            IWSDHttpAddress * This);
        
        DECLSPEC_XFGVIRT(IWSDHttpAddress, SetSecure)
        HRESULT ( STDMETHODCALLTYPE *SetSecure )( 
            IWSDHttpAddress * This,
            /* [in] */ BOOL fSecure);
        
        DECLSPEC_XFGVIRT(IWSDHttpAddress, GetPath)
        HRESULT ( STDMETHODCALLTYPE *GetPath )( 
            IWSDHttpAddress * This,
            /* [annotation][out] */ 
            _Outptr_  LPCWSTR *ppszPath);
        
        DECLSPEC_XFGVIRT(IWSDHttpAddress, SetPath)
        HRESULT ( STDMETHODCALLTYPE *SetPath )( 
            IWSDHttpAddress * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszPath);
        
        END_INTERFACE
    } IWSDHttpAddressVtbl;

    interface IWSDHttpAddress
    {
        CONST_VTBL struct IWSDHttpAddressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDHttpAddress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDHttpAddress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDHttpAddress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDHttpAddress_Serialize(This,pszBuffer,cchLength,fSafe)	\
    ( (This)->lpVtbl -> Serialize(This,pszBuffer,cchLength,fSafe) ) 

#define IWSDHttpAddress_Deserialize(This,pszBuffer)	\
    ( (This)->lpVtbl -> Deserialize(This,pszBuffer) ) 


#define IWSDHttpAddress_GetPort(This,pwPort)	\
    ( (This)->lpVtbl -> GetPort(This,pwPort) ) 

#define IWSDHttpAddress_SetPort(This,wPort)	\
    ( (This)->lpVtbl -> SetPort(This,wPort) ) 

#define IWSDHttpAddress_GetTransportAddress(This,ppszAddress)	\
    ( (This)->lpVtbl -> GetTransportAddress(This,ppszAddress) ) 

#define IWSDHttpAddress_GetTransportAddressEx(This,fSafe,ppszAddress)	\
    ( (This)->lpVtbl -> GetTransportAddressEx(This,fSafe,ppszAddress) ) 

#define IWSDHttpAddress_SetTransportAddress(This,pszAddress)	\
    ( (This)->lpVtbl -> SetTransportAddress(This,pszAddress) ) 


#define IWSDHttpAddress_GetSecure(This)	\
    ( (This)->lpVtbl -> GetSecure(This) ) 

#define IWSDHttpAddress_SetSecure(This,fSecure)	\
    ( (This)->lpVtbl -> SetSecure(This,fSecure) ) 

#define IWSDHttpAddress_GetPath(This,ppszPath)	\
    ( (This)->lpVtbl -> GetPath(This,ppszPath) ) 

#define IWSDHttpAddress_SetPath(This,pszPath)	\
    ( (This)->lpVtbl -> SetPath(This,pszPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDHttpAddress_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdbase_0000_0007 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0007_v0_0_s_ifspec;

#ifndef __IWSDSSLClientCertificate_INTERFACE_DEFINED__
#define __IWSDSSLClientCertificate_INTERFACE_DEFINED__

/* interface IWSDSSLClientCertificate */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IWSDSSLClientCertificate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("de105e87-a0da-418e-98ad-27b9eed87bdc")
    IWSDSSLClientCertificate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClientCertificate( 
            /* [annotation][out][in] */ 
            _Outptr_  PCCERT_CONTEXT *ppCertContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMappedAccessToken( 
            /* [annotation][out][in] */ 
            _Out_  HANDLE *phToken) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDSSLClientCertificateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDSSLClientCertificate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDSSLClientCertificate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDSSLClientCertificate * This);
        
        DECLSPEC_XFGVIRT(IWSDSSLClientCertificate, GetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetClientCertificate )( 
            IWSDSSLClientCertificate * This,
            /* [annotation][out][in] */ 
            _Outptr_  PCCERT_CONTEXT *ppCertContext);
        
        DECLSPEC_XFGVIRT(IWSDSSLClientCertificate, GetMappedAccessToken)
        HRESULT ( STDMETHODCALLTYPE *GetMappedAccessToken )( 
            IWSDSSLClientCertificate * This,
            /* [annotation][out][in] */ 
            _Out_  HANDLE *phToken);
        
        END_INTERFACE
    } IWSDSSLClientCertificateVtbl;

    interface IWSDSSLClientCertificate
    {
        CONST_VTBL struct IWSDSSLClientCertificateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDSSLClientCertificate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDSSLClientCertificate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDSSLClientCertificate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDSSLClientCertificate_GetClientCertificate(This,ppCertContext)	\
    ( (This)->lpVtbl -> GetClientCertificate(This,ppCertContext) ) 

#define IWSDSSLClientCertificate_GetMappedAccessToken(This,phToken)	\
    ( (This)->lpVtbl -> GetMappedAccessToken(This,phToken) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDSSLClientCertificate_INTERFACE_DEFINED__ */


#ifndef __IWSDHttpAuthParameters_INTERFACE_DEFINED__
#define __IWSDHttpAuthParameters_INTERFACE_DEFINED__

/* interface IWSDHttpAuthParameters */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IWSDHttpAuthParameters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0b476df0-8dac-480d-b05c-99781a5884aa")
    IWSDHttpAuthParameters : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClientAccessToken( 
            /* [annotation][out] */ 
            _Out_  HANDLE *phToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAuthType( 
            /* [annotation][out] */ 
            _Out_  PWSD_SECURITY_HTTP_AUTH_SCHEMES pAuthType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDHttpAuthParametersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDHttpAuthParameters * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDHttpAuthParameters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDHttpAuthParameters * This);
        
        DECLSPEC_XFGVIRT(IWSDHttpAuthParameters, GetClientAccessToken)
        HRESULT ( STDMETHODCALLTYPE *GetClientAccessToken )( 
            IWSDHttpAuthParameters * This,
            /* [annotation][out] */ 
            _Out_  HANDLE *phToken);
        
        DECLSPEC_XFGVIRT(IWSDHttpAuthParameters, GetAuthType)
        HRESULT ( STDMETHODCALLTYPE *GetAuthType )( 
            IWSDHttpAuthParameters * This,
            /* [annotation][out] */ 
            _Out_  PWSD_SECURITY_HTTP_AUTH_SCHEMES pAuthType);
        
        END_INTERFACE
    } IWSDHttpAuthParametersVtbl;

    interface IWSDHttpAuthParameters
    {
        CONST_VTBL struct IWSDHttpAuthParametersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDHttpAuthParameters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDHttpAuthParameters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDHttpAuthParameters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDHttpAuthParameters_GetClientAccessToken(This,phToken)	\
    ( (This)->lpVtbl -> GetClientAccessToken(This,phToken) ) 

#define IWSDHttpAuthParameters_GetAuthType(This,pAuthType)	\
    ( (This)->lpVtbl -> GetAuthType(This,pAuthType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDHttpAuthParameters_INTERFACE_DEFINED__ */


#ifndef __IWSDSignatureProperty_INTERFACE_DEFINED__
#define __IWSDSignatureProperty_INTERFACE_DEFINED__

/* interface IWSDSignatureProperty */
/* [local][restricted][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWSDSignatureProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03ce20aa-71c4-45e2-b32e-3766c61c790f")
    IWSDSignatureProperty : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsMessageSigned( 
            /* [annotation][out] */ 
            _Out_  BOOL *pbSigned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsMessageSignatureTrusted( 
            /* [annotation][out] */ 
            _Out_  BOOL *pbSignatureTrusted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeyInfo( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_opt_(*pdwKeyInfoSize, *pdwKeyInfoSize)  BYTE *pbKeyInfo,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwKeyInfoSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignature( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_opt_(*pdwSignatureSize, *pdwSignatureSize)  BYTE *pbSignature,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwSignatureSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignedInfoHash( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_opt_(*pdwHashSize, *pdwHashSize)  BYTE *pbSignedInfoHash,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwHashSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWSDSignaturePropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWSDSignatureProperty * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWSDSignatureProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWSDSignatureProperty * This);
        
        DECLSPEC_XFGVIRT(IWSDSignatureProperty, IsMessageSigned)
        HRESULT ( STDMETHODCALLTYPE *IsMessageSigned )( 
            IWSDSignatureProperty * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbSigned);
        
        DECLSPEC_XFGVIRT(IWSDSignatureProperty, IsMessageSignatureTrusted)
        HRESULT ( STDMETHODCALLTYPE *IsMessageSignatureTrusted )( 
            IWSDSignatureProperty * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pbSignatureTrusted);
        
        DECLSPEC_XFGVIRT(IWSDSignatureProperty, GetKeyInfo)
        HRESULT ( STDMETHODCALLTYPE *GetKeyInfo )( 
            IWSDSignatureProperty * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_opt_(*pdwKeyInfoSize, *pdwKeyInfoSize)  BYTE *pbKeyInfo,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwKeyInfoSize);
        
        DECLSPEC_XFGVIRT(IWSDSignatureProperty, GetSignature)
        HRESULT ( STDMETHODCALLTYPE *GetSignature )( 
            IWSDSignatureProperty * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_opt_(*pdwSignatureSize, *pdwSignatureSize)  BYTE *pbSignature,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwSignatureSize);
        
        DECLSPEC_XFGVIRT(IWSDSignatureProperty, GetSignedInfoHash)
        HRESULT ( STDMETHODCALLTYPE *GetSignedInfoHash )( 
            IWSDSignatureProperty * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_opt_(*pdwHashSize, *pdwHashSize)  BYTE *pbSignedInfoHash,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwHashSize);
        
        END_INTERFACE
    } IWSDSignaturePropertyVtbl;

    interface IWSDSignatureProperty
    {
        CONST_VTBL struct IWSDSignaturePropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWSDSignatureProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWSDSignatureProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWSDSignatureProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWSDSignatureProperty_IsMessageSigned(This,pbSigned)	\
    ( (This)->lpVtbl -> IsMessageSigned(This,pbSigned) ) 

#define IWSDSignatureProperty_IsMessageSignatureTrusted(This,pbSignatureTrusted)	\
    ( (This)->lpVtbl -> IsMessageSignatureTrusted(This,pbSignatureTrusted) ) 

#define IWSDSignatureProperty_GetKeyInfo(This,pbKeyInfo,pdwKeyInfoSize)	\
    ( (This)->lpVtbl -> GetKeyInfo(This,pbKeyInfo,pdwKeyInfoSize) ) 

#define IWSDSignatureProperty_GetSignature(This,pbSignature,pdwSignatureSize)	\
    ( (This)->lpVtbl -> GetSignature(This,pbSignature,pdwSignatureSize) ) 

#define IWSDSignatureProperty_GetSignedInfoHash(This,pbSignedInfoHash,pdwHashSize)	\
    ( (This)->lpVtbl -> GetSignedInfoHash(This,pbSignedInfoHash,pdwHashSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWSDSignatureProperty_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wsdbase_0000_0010 */
/* [local] */ 

#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wsdbase_0000_0010_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


