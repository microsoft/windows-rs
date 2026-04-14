

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

#ifndef __wcndevice_h__
#define __wcndevice_h__

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

#ifndef __IWCNDevice_FWD_DEFINED__
#define __IWCNDevice_FWD_DEFINED__
typedef interface IWCNDevice IWCNDevice;

#endif 	/* __IWCNDevice_FWD_DEFINED__ */


#ifndef __IWCNConnectNotify_FWD_DEFINED__
#define __IWCNConnectNotify_FWD_DEFINED__
typedef interface IWCNConnectNotify IWCNConnectNotify;

#endif 	/* __IWCNConnectNotify_FWD_DEFINED__ */


#ifndef __WCNDeviceObject_FWD_DEFINED__
#define __WCNDeviceObject_FWD_DEFINED__

#ifdef __cplusplus
typedef class WCNDeviceObject WCNDeviceObject;
#else
typedef struct WCNDeviceObject WCNDeviceObject;
#endif /* __cplusplus */

#endif 	/* __WCNDeviceObject_FWD_DEFINED__ */


/* header files for imported files */
#include "objidl.h"
#include "WcnTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wcndevice_0000_0000 */
/* [local] */ 

/*++

Copyright (c) Microsoft Corporation

Module Name:

    WcnDevice.h

Abstract:

    Declares the IWCNDevice interface for WCN API

--*/
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if NTDDI_VERSION >= NTDDI_WIN7


#define	WCN_API_MAX_BUFFER_SIZE	( 2096 )

typedef /* [v1_enum] */ 
enum tagWCN_PASSWORD_TYPE
    {
        WCN_PASSWORD_TYPE_PUSH_BUTTON	= 0,
        WCN_PASSWORD_TYPE_PIN	= ( WCN_PASSWORD_TYPE_PUSH_BUTTON + 1 ) ,
        WCN_PASSWORD_TYPE_PIN_REGISTRAR_SPECIFIED	= ( WCN_PASSWORD_TYPE_PIN + 1 ) ,
        WCN_PASSWORD_TYPE_OOB_SPECIFIED	= ( WCN_PASSWORD_TYPE_PIN_REGISTRAR_SPECIFIED + 1 ) ,
        WCN_PASSWORD_TYPE_WFDS	= ( WCN_PASSWORD_TYPE_OOB_SPECIFIED + 1 ) 
    } 	WCN_PASSWORD_TYPE;

typedef /* [v1_enum] */ 
enum tagWCN_SESSION_STATUS
    {
        WCN_SESSION_STATUS_SUCCESS	= 0,
        WCN_SESSION_STATUS_FAILURE_GENERIC	= ( WCN_SESSION_STATUS_SUCCESS + 1 ) ,
        WCN_SESSION_STATUS_FAILURE_TIMEOUT	= ( WCN_SESSION_STATUS_FAILURE_GENERIC + 1 ) 
    } 	WCN_SESSION_STATUS;

typedef struct tagWCN_VENDOR_EXTENSION_SPEC
    {
    DWORD VendorId;
    DWORD SubType;
    DWORD Index;
    DWORD Flags;
    } 	WCN_VENDOR_EXTENSION_SPEC;

#define	WCN_MICROSOFT_VENDOR_ID	( 311 )

#define	WCN_NO_SUBTYPE	( 0xfffffffe )

#define	WCN_FLAG_DISCOVERY_VE	( 0x1 )

#define	WCN_FLAG_AUTHENTICATED_VE	( 0x2 )

#define	WCN_FLAG_ENCRYPTED_VE	( 0x4 )



extern RPC_IF_HANDLE __MIDL_itf_wcndevice_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcndevice_0000_0000_v0_0_s_ifspec;

#ifndef __IWCNDevice_INTERFACE_DEFINED__
#define __IWCNDevice_INTERFACE_DEFINED__

/* interface IWCNDevice */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IWCNDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C100BE9C-D33A-4a4b-BF23-BBEF4663D017")
    IWCNDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPassword( 
            /* [in] */ WCN_PASSWORD_TYPE Type,
            /* [in] */ DWORD dwPasswordLength,
            /* [size_is][in] */ __RPC__in_ecount_full(dwPasswordLength) const BYTE pbPassword[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [unique][in] */ __RPC__in_opt IWCNConnectNotify *pNotify) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttribute( 
            /* [in] */ WCN_ATTRIBUTE_TYPE AttributeType,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD dwMaxBufferSize,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(dwMaxBufferSize, *pdwBufferUsed) BYTE pbBuffer[  ],
            /* [out] */ __RPC__out DWORD *pdwBufferUsed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIntegerAttribute( 
            /* [in] */ WCN_ATTRIBUTE_TYPE AttributeType,
            /* [out] */ __RPC__out UINT *puInteger) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStringAttribute( 
            /* [in] */ WCN_ATTRIBUTE_TYPE AttributeType,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD cchMaxString,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchMaxString) WCHAR wszString[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNetworkProfile( 
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD cchMaxStringLength,
            /* [size_is][out] */ __RPC__out_ecount_full(cchMaxStringLength) LPWSTR wszProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNetworkProfile( 
            /* [in] */ __RPC__in LPCWSTR pszProfileXml) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVendorExtension( 
            /* [in] */ __RPC__in const WCN_VENDOR_EXTENSION_SPEC *pVendorExtSpec,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD dwMaxBufferSize,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(dwMaxBufferSize, *pdwBufferUsed) BYTE pbBuffer[  ],
            /* [out] */ __RPC__out DWORD *pdwBufferUsed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVendorExtension( 
            /* [in] */ __RPC__in const WCN_VENDOR_EXTENSION_SPEC *pVendorExtSpec,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD cbBuffer,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBuffer) const BYTE pbBuffer[  ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNFCPasswordParams( 
            /* [in] */ WCN_PASSWORD_TYPE Type,
            /* [in] */ DWORD dwOOBPasswordID,
            /* [in] */ DWORD dwPasswordLength,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwPasswordLength) const BYTE pbPassword[  ],
            /* [in] */ DWORD dwRemotePublicKeyHashLength,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwRemotePublicKeyHashLength) const BYTE pbRemotePublicKeyHash[  ],
            /* [in] */ DWORD dwDHKeyBlobLength,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwDHKeyBlobLength) const BYTE pbDHKeyBlob[  ]) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWCNDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWCNDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWCNDevice * This);
        
        DECLSPEC_XFGVIRT(IWCNDevice, SetPassword)
        HRESULT ( STDMETHODCALLTYPE *SetPassword )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ WCN_PASSWORD_TYPE Type,
            /* [in] */ DWORD dwPasswordLength,
            /* [size_is][in] */ __RPC__in_ecount_full(dwPasswordLength) const BYTE pbPassword[  ]);
        
        DECLSPEC_XFGVIRT(IWCNDevice, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IWCNDevice * This,
            /* [unique][in] */ __RPC__in_opt IWCNConnectNotify *pNotify);
        
        DECLSPEC_XFGVIRT(IWCNDevice, GetAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetAttribute )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ WCN_ATTRIBUTE_TYPE AttributeType,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD dwMaxBufferSize,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(dwMaxBufferSize, *pdwBufferUsed) BYTE pbBuffer[  ],
            /* [out] */ __RPC__out DWORD *pdwBufferUsed);
        
        DECLSPEC_XFGVIRT(IWCNDevice, GetIntegerAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetIntegerAttribute )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ WCN_ATTRIBUTE_TYPE AttributeType,
            /* [out] */ __RPC__out UINT *puInteger);
        
        DECLSPEC_XFGVIRT(IWCNDevice, GetStringAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetStringAttribute )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ WCN_ATTRIBUTE_TYPE AttributeType,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD cchMaxString,
            /* [size_is][string][out] */ __RPC__out_ecount_full_string(cchMaxString) WCHAR wszString[  ]);
        
        DECLSPEC_XFGVIRT(IWCNDevice, GetNetworkProfile)
        HRESULT ( STDMETHODCALLTYPE *GetNetworkProfile )( 
            __RPC__in IWCNDevice * This,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD cchMaxStringLength,
            /* [size_is][out] */ __RPC__out_ecount_full(cchMaxStringLength) LPWSTR wszProfile);
        
        DECLSPEC_XFGVIRT(IWCNDevice, SetNetworkProfile)
        HRESULT ( STDMETHODCALLTYPE *SetNetworkProfile )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ __RPC__in LPCWSTR pszProfileXml);
        
        DECLSPEC_XFGVIRT(IWCNDevice, GetVendorExtension)
        HRESULT ( STDMETHODCALLTYPE *GetVendorExtension )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ __RPC__in const WCN_VENDOR_EXTENSION_SPEC *pVendorExtSpec,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD dwMaxBufferSize,
            /* [size_is][length_is][out] */ __RPC__out_ecount_part(dwMaxBufferSize, *pdwBufferUsed) BYTE pbBuffer[  ],
            /* [out] */ __RPC__out DWORD *pdwBufferUsed);
        
        DECLSPEC_XFGVIRT(IWCNDevice, SetVendorExtension)
        HRESULT ( STDMETHODCALLTYPE *SetVendorExtension )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ __RPC__in const WCN_VENDOR_EXTENSION_SPEC *pVendorExtSpec,
            /* [range][in] */ __RPC__in_range(0,WCN_API_MAX_BUFFER_SIZE) DWORD cbBuffer,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBuffer) const BYTE pbBuffer[  ]);
        
        DECLSPEC_XFGVIRT(IWCNDevice, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in IWCNDevice * This);
        
        DECLSPEC_XFGVIRT(IWCNDevice, SetNFCPasswordParams)
        HRESULT ( STDMETHODCALLTYPE *SetNFCPasswordParams )( 
            __RPC__in IWCNDevice * This,
            /* [in] */ WCN_PASSWORD_TYPE Type,
            /* [in] */ DWORD dwOOBPasswordID,
            /* [in] */ DWORD dwPasswordLength,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwPasswordLength) const BYTE pbPassword[  ],
            /* [in] */ DWORD dwRemotePublicKeyHashLength,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwRemotePublicKeyHashLength) const BYTE pbRemotePublicKeyHash[  ],
            /* [in] */ DWORD dwDHKeyBlobLength,
            /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(dwDHKeyBlobLength) const BYTE pbDHKeyBlob[  ]);
        
        END_INTERFACE
    } IWCNDeviceVtbl;

    interface IWCNDevice
    {
        CONST_VTBL struct IWCNDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWCNDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWCNDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWCNDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWCNDevice_SetPassword(This,Type,dwPasswordLength,pbPassword)	\
    ( (This)->lpVtbl -> SetPassword(This,Type,dwPasswordLength,pbPassword) ) 

#define IWCNDevice_Connect(This,pNotify)	\
    ( (This)->lpVtbl -> Connect(This,pNotify) ) 

#define IWCNDevice_GetAttribute(This,AttributeType,dwMaxBufferSize,pbBuffer,pdwBufferUsed)	\
    ( (This)->lpVtbl -> GetAttribute(This,AttributeType,dwMaxBufferSize,pbBuffer,pdwBufferUsed) ) 

#define IWCNDevice_GetIntegerAttribute(This,AttributeType,puInteger)	\
    ( (This)->lpVtbl -> GetIntegerAttribute(This,AttributeType,puInteger) ) 

#define IWCNDevice_GetStringAttribute(This,AttributeType,cchMaxString,wszString)	\
    ( (This)->lpVtbl -> GetStringAttribute(This,AttributeType,cchMaxString,wszString) ) 

#define IWCNDevice_GetNetworkProfile(This,cchMaxStringLength,wszProfile)	\
    ( (This)->lpVtbl -> GetNetworkProfile(This,cchMaxStringLength,wszProfile) ) 

#define IWCNDevice_SetNetworkProfile(This,pszProfileXml)	\
    ( (This)->lpVtbl -> SetNetworkProfile(This,pszProfileXml) ) 

#define IWCNDevice_GetVendorExtension(This,pVendorExtSpec,dwMaxBufferSize,pbBuffer,pdwBufferUsed)	\
    ( (This)->lpVtbl -> GetVendorExtension(This,pVendorExtSpec,dwMaxBufferSize,pbBuffer,pdwBufferUsed) ) 

#define IWCNDevice_SetVendorExtension(This,pVendorExtSpec,cbBuffer,pbBuffer)	\
    ( (This)->lpVtbl -> SetVendorExtension(This,pVendorExtSpec,cbBuffer,pbBuffer) ) 

#define IWCNDevice_Unadvise(This)	\
    ( (This)->lpVtbl -> Unadvise(This) ) 

#define IWCNDevice_SetNFCPasswordParams(This,Type,dwOOBPasswordID,dwPasswordLength,pbPassword,dwRemotePublicKeyHashLength,pbRemotePublicKeyHash,dwDHKeyBlobLength,pbDHKeyBlob)	\
    ( (This)->lpVtbl -> SetNFCPasswordParams(This,Type,dwOOBPasswordID,dwPasswordLength,pbPassword,dwRemotePublicKeyHashLength,pbRemotePublicKeyHash,dwDHKeyBlobLength,pbDHKeyBlob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWCNDevice_INTERFACE_DEFINED__ */


#ifndef __IWCNConnectNotify_INTERFACE_DEFINED__
#define __IWCNConnectNotify_INTERFACE_DEFINED__

/* interface IWCNConnectNotify */
/* [uuid][object] */ 


EXTERN_C const IID IID_IWCNConnectNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C100BE9F-D33A-4a4b-BF23-BBEF4663D017")
    IWCNConnectNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConnectSucceeded( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectFailed( 
            /* [in] */ HRESULT hrFailure) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWCNConnectNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWCNConnectNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWCNConnectNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWCNConnectNotify * This);
        
        DECLSPEC_XFGVIRT(IWCNConnectNotify, ConnectSucceeded)
        HRESULT ( STDMETHODCALLTYPE *ConnectSucceeded )( 
            __RPC__in IWCNConnectNotify * This);
        
        DECLSPEC_XFGVIRT(IWCNConnectNotify, ConnectFailed)
        HRESULT ( STDMETHODCALLTYPE *ConnectFailed )( 
            __RPC__in IWCNConnectNotify * This,
            /* [in] */ HRESULT hrFailure);
        
        END_INTERFACE
    } IWCNConnectNotifyVtbl;

    interface IWCNConnectNotify
    {
        CONST_VTBL struct IWCNConnectNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWCNConnectNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWCNConnectNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWCNConnectNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWCNConnectNotify_ConnectSucceeded(This)	\
    ( (This)->lpVtbl -> ConnectSucceeded(This) ) 

#define IWCNConnectNotify_ConnectFailed(This,hrFailure)	\
    ( (This)->lpVtbl -> ConnectFailed(This,hrFailure) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWCNConnectNotify_INTERFACE_DEFINED__ */



#ifndef __WCNDeviceObjectLibrary_LIBRARY_DEFINED__
#define __WCNDeviceObjectLibrary_LIBRARY_DEFINED__

/* library WCNDeviceObjectLibrary */
/* [uuid] */ 


EXTERN_C const IID LIBID_WCNDeviceObjectLibrary;

EXTERN_C const CLSID CLSID_WCNDeviceObject;

#ifdef __cplusplus

class DECLSPEC_UUID("C100BEA7-D33A-4a4b-BF23-BBEF4663D017")
WCNDeviceObject;
#endif
#endif /* __WCNDeviceObjectLibrary_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_wcndevice_0000_0003 */
/* [local] */ 


#endif // NTDDI_VERSION >= NTDDI_WIN7
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wcndevice_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcndevice_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


