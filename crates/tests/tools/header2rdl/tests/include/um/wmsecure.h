

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

#ifndef __wmsecure_h__
#define __wmsecure_h__

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

#ifndef __IWMAuthorizer_FWD_DEFINED__
#define __IWMAuthorizer_FWD_DEFINED__
typedef interface IWMAuthorizer IWMAuthorizer;

#endif 	/* __IWMAuthorizer_FWD_DEFINED__ */


#ifndef __IWMSecureChannel_FWD_DEFINED__
#define __IWMSecureChannel_FWD_DEFINED__
typedef interface IWMSecureChannel IWMSecureChannel;

#endif 	/* __IWMSecureChannel_FWD_DEFINED__ */


#ifndef __IWMGetSecureChannel_FWD_DEFINED__
#define __IWMGetSecureChannel_FWD_DEFINED__
typedef interface IWMGetSecureChannel IWMGetSecureChannel;

#endif 	/* __IWMGetSecureChannel_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wmsecure_0000_0000 */
/* [local] */ 

//=========================================================================
//
//  THIS SOFTWARE HAS BEEN LICENSED FROM MICROSOFT CORPORATION PURSUANT 
//  TO THE TERMS OF AN END USER LICENSE AGREEMENT ("EULA").  
//  PLEASE REFER TO THE TEXT OF THE EULA TO DETERMINE THE RIGHTS TO USE THE SOFTWARE.  
//
// Copyright (C) Microsoft Corporation, 1999 - 1999  All Rights Reserved.
//
//=========================================================================
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( IID_IWMAuthorizer,     0xd9b67d36, 0xa9ad, 0x4eb4, 0xba, 0xef, 0xdb, 0x28, 0x4e, 0xf5, 0x50, 0x4c );
EXTERN_GUID( IID_IWMSecureChannel,  0x2720598a, 0xd0f2, 0x4189, 0xbd, 0x10, 0x91, 0xc4, 0x6e, 0xf0, 0x93, 0x6f );
EXTERN_GUID( IID_IWMGetSecureChannel, 0x94bc0598, 0xc3d2, 0x11d3, 0xbe, 0xdf, 0x00, 0xc0, 0x4f, 0x61, 0x29, 0x86 );


extern RPC_IF_HANDLE __MIDL_itf_wmsecure_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmsecure_0000_0000_v0_0_s_ifspec;

#ifndef __IWMAuthorizer_INTERFACE_DEFINED__
#define __IWMAuthorizer_INTERFACE_DEFINED__

/* interface IWMAuthorizer */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMAuthorizer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D9B67D36-A9AD-4eb4-BAEF-DB284EF5504C")
    IWMAuthorizer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCertCount( 
            /* [out] */ DWORD *pcCerts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCert( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ BYTE **ppbCertData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSharedData( 
            /* [in] */ DWORD dwCertIndex,
            /* [in] */ const BYTE *pbSharedData,
            /* [in] */ BYTE *pbCert,
            /* [out] */ BYTE **ppbSharedData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMAuthorizerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMAuthorizer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMAuthorizer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMAuthorizer * This);
        
        DECLSPEC_XFGVIRT(IWMAuthorizer, GetCertCount)
        HRESULT ( STDMETHODCALLTYPE *GetCertCount )( 
            IWMAuthorizer * This,
            /* [out] */ DWORD *pcCerts);
        
        DECLSPEC_XFGVIRT(IWMAuthorizer, GetCert)
        HRESULT ( STDMETHODCALLTYPE *GetCert )( 
            IWMAuthorizer * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ BYTE **ppbCertData);
        
        DECLSPEC_XFGVIRT(IWMAuthorizer, GetSharedData)
        HRESULT ( STDMETHODCALLTYPE *GetSharedData )( 
            IWMAuthorizer * This,
            /* [in] */ DWORD dwCertIndex,
            /* [in] */ const BYTE *pbSharedData,
            /* [in] */ BYTE *pbCert,
            /* [out] */ BYTE **ppbSharedData);
        
        END_INTERFACE
    } IWMAuthorizerVtbl;

    interface IWMAuthorizer
    {
        CONST_VTBL struct IWMAuthorizerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMAuthorizer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMAuthorizer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMAuthorizer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMAuthorizer_GetCertCount(This,pcCerts)	\
    ( (This)->lpVtbl -> GetCertCount(This,pcCerts) ) 

#define IWMAuthorizer_GetCert(This,dwIndex,ppbCertData)	\
    ( (This)->lpVtbl -> GetCert(This,dwIndex,ppbCertData) ) 

#define IWMAuthorizer_GetSharedData(This,dwCertIndex,pbSharedData,pbCert,ppbSharedData)	\
    ( (This)->lpVtbl -> GetSharedData(This,dwCertIndex,pbSharedData,pbCert,ppbSharedData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMAuthorizer_INTERFACE_DEFINED__ */


#ifndef __IWMSecureChannel_INTERFACE_DEFINED__
#define __IWMSecureChannel_INTERFACE_DEFINED__

/* interface IWMSecureChannel */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMSecureChannel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2720598A-D0F2-4189-BD10-91C46EF0936F")
    IWMSecureChannel : public IWMAuthorizer
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WMSC_AddCertificate( 
            /* [in] */ IWMAuthorizer *pCert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_AddSignature( 
            /* [in] */ BYTE *pbCertSig,
            /* [in] */ DWORD cbCertSig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_Connect( 
            /* [in] */ IWMSecureChannel *pOtherSide) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_IsConnected( 
            /* [out] */ BOOL *pfIsConnected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_Disconnect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_GetValidCertificate( 
            /* [out] */ BYTE **ppbCertificate,
            /* [out] */ DWORD *pdwSignature) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_Encrypt( 
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD cbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_Decrypt( 
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD cbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_Lock( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_Unlock( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WMSC_SetSharedData( 
            /* [in] */ DWORD dwCertIndex,
            /* [in] */ const BYTE *pbSharedData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMSecureChannelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMSecureChannel * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IWMAuthorizer, GetCertCount)
        HRESULT ( STDMETHODCALLTYPE *GetCertCount )( 
            IWMSecureChannel * This,
            /* [out] */ DWORD *pcCerts);
        
        DECLSPEC_XFGVIRT(IWMAuthorizer, GetCert)
        HRESULT ( STDMETHODCALLTYPE *GetCert )( 
            IWMSecureChannel * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ BYTE **ppbCertData);
        
        DECLSPEC_XFGVIRT(IWMAuthorizer, GetSharedData)
        HRESULT ( STDMETHODCALLTYPE *GetSharedData )( 
            IWMSecureChannel * This,
            /* [in] */ DWORD dwCertIndex,
            /* [in] */ const BYTE *pbSharedData,
            /* [in] */ BYTE *pbCert,
            /* [out] */ BYTE **ppbSharedData);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_AddCertificate)
        HRESULT ( STDMETHODCALLTYPE *WMSC_AddCertificate )( 
            IWMSecureChannel * This,
            /* [in] */ IWMAuthorizer *pCert);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_AddSignature)
        HRESULT ( STDMETHODCALLTYPE *WMSC_AddSignature )( 
            IWMSecureChannel * This,
            /* [in] */ BYTE *pbCertSig,
            /* [in] */ DWORD cbCertSig);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_Connect)
        HRESULT ( STDMETHODCALLTYPE *WMSC_Connect )( 
            IWMSecureChannel * This,
            /* [in] */ IWMSecureChannel *pOtherSide);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_IsConnected)
        HRESULT ( STDMETHODCALLTYPE *WMSC_IsConnected )( 
            IWMSecureChannel * This,
            /* [out] */ BOOL *pfIsConnected);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_Disconnect)
        HRESULT ( STDMETHODCALLTYPE *WMSC_Disconnect )( 
            IWMSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_GetValidCertificate)
        HRESULT ( STDMETHODCALLTYPE *WMSC_GetValidCertificate )( 
            IWMSecureChannel * This,
            /* [out] */ BYTE **ppbCertificate,
            /* [out] */ DWORD *pdwSignature);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_Encrypt)
        HRESULT ( STDMETHODCALLTYPE *WMSC_Encrypt )( 
            IWMSecureChannel * This,
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD cbData);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_Decrypt)
        HRESULT ( STDMETHODCALLTYPE *WMSC_Decrypt )( 
            IWMSecureChannel * This,
            /* [in] */ BYTE *pbData,
            /* [in] */ DWORD cbData);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_Lock)
        HRESULT ( STDMETHODCALLTYPE *WMSC_Lock )( 
            IWMSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_Unlock)
        HRESULT ( STDMETHODCALLTYPE *WMSC_Unlock )( 
            IWMSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IWMSecureChannel, WMSC_SetSharedData)
        HRESULT ( STDMETHODCALLTYPE *WMSC_SetSharedData )( 
            IWMSecureChannel * This,
            /* [in] */ DWORD dwCertIndex,
            /* [in] */ const BYTE *pbSharedData);
        
        END_INTERFACE
    } IWMSecureChannelVtbl;

    interface IWMSecureChannel
    {
        CONST_VTBL struct IWMSecureChannelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMSecureChannel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMSecureChannel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMSecureChannel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMSecureChannel_GetCertCount(This,pcCerts)	\
    ( (This)->lpVtbl -> GetCertCount(This,pcCerts) ) 

#define IWMSecureChannel_GetCert(This,dwIndex,ppbCertData)	\
    ( (This)->lpVtbl -> GetCert(This,dwIndex,ppbCertData) ) 

#define IWMSecureChannel_GetSharedData(This,dwCertIndex,pbSharedData,pbCert,ppbSharedData)	\
    ( (This)->lpVtbl -> GetSharedData(This,dwCertIndex,pbSharedData,pbCert,ppbSharedData) ) 


#define IWMSecureChannel_WMSC_AddCertificate(This,pCert)	\
    ( (This)->lpVtbl -> WMSC_AddCertificate(This,pCert) ) 

#define IWMSecureChannel_WMSC_AddSignature(This,pbCertSig,cbCertSig)	\
    ( (This)->lpVtbl -> WMSC_AddSignature(This,pbCertSig,cbCertSig) ) 

#define IWMSecureChannel_WMSC_Connect(This,pOtherSide)	\
    ( (This)->lpVtbl -> WMSC_Connect(This,pOtherSide) ) 

#define IWMSecureChannel_WMSC_IsConnected(This,pfIsConnected)	\
    ( (This)->lpVtbl -> WMSC_IsConnected(This,pfIsConnected) ) 

#define IWMSecureChannel_WMSC_Disconnect(This)	\
    ( (This)->lpVtbl -> WMSC_Disconnect(This) ) 

#define IWMSecureChannel_WMSC_GetValidCertificate(This,ppbCertificate,pdwSignature)	\
    ( (This)->lpVtbl -> WMSC_GetValidCertificate(This,ppbCertificate,pdwSignature) ) 

#define IWMSecureChannel_WMSC_Encrypt(This,pbData,cbData)	\
    ( (This)->lpVtbl -> WMSC_Encrypt(This,pbData,cbData) ) 

#define IWMSecureChannel_WMSC_Decrypt(This,pbData,cbData)	\
    ( (This)->lpVtbl -> WMSC_Decrypt(This,pbData,cbData) ) 

#define IWMSecureChannel_WMSC_Lock(This)	\
    ( (This)->lpVtbl -> WMSC_Lock(This) ) 

#define IWMSecureChannel_WMSC_Unlock(This)	\
    ( (This)->lpVtbl -> WMSC_Unlock(This) ) 

#define IWMSecureChannel_WMSC_SetSharedData(This,dwCertIndex,pbSharedData)	\
    ( (This)->lpVtbl -> WMSC_SetSharedData(This,dwCertIndex,pbSharedData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMSecureChannel_INTERFACE_DEFINED__ */


#ifndef __IWMGetSecureChannel_INTERFACE_DEFINED__
#define __IWMGetSecureChannel_INTERFACE_DEFINED__

/* interface IWMGetSecureChannel */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWMGetSecureChannel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94bc0598-c3d2-11d3-bedf-00c04f612986")
    IWMGetSecureChannel : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPeerSecureChannelInterface( 
            /* [out] */ IWMSecureChannel **ppPeer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWMGetSecureChannelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IWMGetSecureChannel * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IWMGetSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IWMGetSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IWMGetSecureChannel, GetPeerSecureChannelInterface)
        HRESULT ( STDMETHODCALLTYPE *GetPeerSecureChannelInterface )( 
            IWMGetSecureChannel * This,
            /* [out] */ IWMSecureChannel **ppPeer);
        
        END_INTERFACE
    } IWMGetSecureChannelVtbl;

    interface IWMGetSecureChannel
    {
        CONST_VTBL struct IWMGetSecureChannelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWMGetSecureChannel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWMGetSecureChannel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWMGetSecureChannel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWMGetSecureChannel_GetPeerSecureChannelInterface(This,ppPeer)	\
    ( (This)->lpVtbl -> GetPeerSecureChannelInterface(This,ppPeer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWMGetSecureChannel_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wmsecure_0000_0003 */
/* [local] */ 

HRESULT STDMETHODCALLTYPE WMCreateSecureChannel( IWMSecureChannel** ppChannel );
HRESULT STDMETHODCALLTYPE WMCreateSecureChannel_Certified( IWMSecureChannel** ppChannel ); 
HRESULT STDMETHODCALLTYPE WMCreateSecureChannel_DES( IWMSecureChannel** ppChannel );
HRESULT STDMETHODCALLTYPE WMCreateSecureChannel_Certified_DES( IWMSecureChannel** ppChannel ); 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wmsecure_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wmsecure_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


