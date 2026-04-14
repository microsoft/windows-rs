

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

#ifndef __PortableDeviceConnectApi_h__
#define __PortableDeviceConnectApi_h__

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

#ifndef __IEnumPortableDeviceConnectors_FWD_DEFINED__
#define __IEnumPortableDeviceConnectors_FWD_DEFINED__
typedef interface IEnumPortableDeviceConnectors IEnumPortableDeviceConnectors;

#endif 	/* __IEnumPortableDeviceConnectors_FWD_DEFINED__ */


#ifndef __IPortableDeviceConnector_FWD_DEFINED__
#define __IPortableDeviceConnector_FWD_DEFINED__
typedef interface IPortableDeviceConnector IPortableDeviceConnector;

#endif 	/* __IPortableDeviceConnector_FWD_DEFINED__ */


#ifndef __IConnectionRequestCallback_FWD_DEFINED__
#define __IConnectionRequestCallback_FWD_DEFINED__
typedef interface IConnectionRequestCallback IConnectionRequestCallback;

#endif 	/* __IConnectionRequestCallback_FWD_DEFINED__ */


#ifndef __EnumBthMtpConnectors_FWD_DEFINED__
#define __EnumBthMtpConnectors_FWD_DEFINED__

#ifdef __cplusplus
typedef class EnumBthMtpConnectors EnumBthMtpConnectors;
#else
typedef struct EnumBthMtpConnectors EnumBthMtpConnectors;
#endif /* __cplusplus */

#endif 	/* __EnumBthMtpConnectors_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "propsys.h"
#include "PortableDeviceConnectImports.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_PortableDeviceConnectApi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)





// Property set by the MTP Bluetooth Enumerator indicate whether the device is connected
// DEVPKEY_MTPBTH_IsConnected, DEVPROP_TYPE_BOOLEAN
// 
// {ea1237fa-589d-4472-84e4-0abe36fd62ef}.2
DEFINE_DEVPROPKEY(DEVPKEY_MTPBTH_IsConnected,            0xea1237fa, 0x589d, 0x4472, 0x84, 0xe4, 0x0a, 0xbe, 0x36, 0xfd, 0x62, 0xef, 2);



extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceConnectApi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceConnectApi_0000_0000_v0_0_s_ifspec;

#ifndef __IEnumPortableDeviceConnectors_INTERFACE_DEFINED__
#define __IEnumPortableDeviceConnectors_INTERFACE_DEFINED__

/* interface IEnumPortableDeviceConnectors */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumPortableDeviceConnectors;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bfdef549-9247-454f-bd82-06fe80853faa")
    IEnumPortableDeviceConnectors : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ UINT32 cRequested,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequested, *pcFetched) IPortableDeviceConnector **pConnectors,
            /* [unique][out][in] */ __RPC__inout_opt UINT32 *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ UINT32 cConnectors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceConnectors **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPortableDeviceConnectorsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPortableDeviceConnectors * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPortableDeviceConnectors * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPortableDeviceConnectors * This);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceConnectors, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPortableDeviceConnectors * This,
            /* [in] */ UINT32 cRequested,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cRequested, *pcFetched) IPortableDeviceConnector **pConnectors,
            /* [unique][out][in] */ __RPC__inout_opt UINT32 *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceConnectors, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumPortableDeviceConnectors * This,
            /* [in] */ UINT32 cConnectors);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceConnectors, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPortableDeviceConnectors * This);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceConnectors, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumPortableDeviceConnectors * This,
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceConnectors **ppEnum);
        
        END_INTERFACE
    } IEnumPortableDeviceConnectorsVtbl;

    interface IEnumPortableDeviceConnectors
    {
        CONST_VTBL struct IEnumPortableDeviceConnectorsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPortableDeviceConnectors_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPortableDeviceConnectors_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPortableDeviceConnectors_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPortableDeviceConnectors_Next(This,cRequested,pConnectors,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cRequested,pConnectors,pcFetched) ) 

#define IEnumPortableDeviceConnectors_Skip(This,cConnectors)	\
    ( (This)->lpVtbl -> Skip(This,cConnectors) ) 

#define IEnumPortableDeviceConnectors_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPortableDeviceConnectors_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPortableDeviceConnectors_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceConnector_INTERFACE_DEFINED__
#define __IPortableDeviceConnector_INTERFACE_DEFINED__

/* interface IPortableDeviceConnector */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("625e2df8-6392-4cf0-9ad1-3cfa5f17775c")
    IPortableDeviceConnector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ __RPC__in_opt IConnectionRequestCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( 
            /* [in] */ __RPC__in_opt IConnectionRequestCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( 
            /* [in] */ __RPC__in_opt IConnectionRequestCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in const DEVPROPKEY *pPropertyKey,
            /* [out] */ __RPC__out DEVPROPTYPE *pPropertyType,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppData,
            /* [out] */ __RPC__out UINT32 *pcbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ __RPC__in const DEVPROPKEY *pPropertyKey,
            /* [in] */ DEVPROPTYPE PropertyType,
            /* [size_is][in] */ __RPC__in_ecount_full(cbData) const BYTE *pData,
            /* [in] */ UINT32 cbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPnPID( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszPnPID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceConnector * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceConnector, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [in] */ __RPC__in_opt IConnectionRequestCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IPortableDeviceConnector, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [in] */ __RPC__in_opt IConnectionRequestCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IPortableDeviceConnector, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [in] */ __RPC__in_opt IConnectionRequestCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IPortableDeviceConnector, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [in] */ __RPC__in const DEVPROPKEY *pPropertyKey,
            /* [out] */ __RPC__out DEVPROPTYPE *pPropertyType,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppData,
            /* [out] */ __RPC__out UINT32 *pcbData);
        
        DECLSPEC_XFGVIRT(IPortableDeviceConnector, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [in] */ __RPC__in const DEVPROPKEY *pPropertyKey,
            /* [in] */ DEVPROPTYPE PropertyType,
            /* [size_is][in] */ __RPC__in_ecount_full(cbData) const BYTE *pData,
            /* [in] */ UINT32 cbData);
        
        DECLSPEC_XFGVIRT(IPortableDeviceConnector, GetPnPID)
        HRESULT ( STDMETHODCALLTYPE *GetPnPID )( 
            __RPC__in IPortableDeviceConnector * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszPnPID);
        
        END_INTERFACE
    } IPortableDeviceConnectorVtbl;

    interface IPortableDeviceConnector
    {
        CONST_VTBL struct IPortableDeviceConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceConnector_Connect(This,pCallback)	\
    ( (This)->lpVtbl -> Connect(This,pCallback) ) 

#define IPortableDeviceConnector_Disconnect(This,pCallback)	\
    ( (This)->lpVtbl -> Disconnect(This,pCallback) ) 

#define IPortableDeviceConnector_Cancel(This,pCallback)	\
    ( (This)->lpVtbl -> Cancel(This,pCallback) ) 

#define IPortableDeviceConnector_GetProperty(This,pPropertyKey,pPropertyType,ppData,pcbData)	\
    ( (This)->lpVtbl -> GetProperty(This,pPropertyKey,pPropertyType,ppData,pcbData) ) 

#define IPortableDeviceConnector_SetProperty(This,pPropertyKey,PropertyType,pData,cbData)	\
    ( (This)->lpVtbl -> SetProperty(This,pPropertyKey,PropertyType,pData,cbData) ) 

#define IPortableDeviceConnector_GetPnPID(This,ppwszPnPID)	\
    ( (This)->lpVtbl -> GetPnPID(This,ppwszPnPID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceConnector_INTERFACE_DEFINED__ */


#ifndef __IConnectionRequestCallback_INTERFACE_DEFINED__
#define __IConnectionRequestCallback_INTERFACE_DEFINED__

/* interface IConnectionRequestCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IConnectionRequestCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("272c9ae0-7161-4ae0-91bd-9f448ee9c427")
    IConnectionRequestCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnComplete( 
            /* [in] */ HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConnectionRequestCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IConnectionRequestCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IConnectionRequestCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IConnectionRequestCallback * This);
        
        DECLSPEC_XFGVIRT(IConnectionRequestCallback, OnComplete)
        HRESULT ( STDMETHODCALLTYPE *OnComplete )( 
            __RPC__in IConnectionRequestCallback * This,
            /* [in] */ HRESULT hrStatus);
        
        END_INTERFACE
    } IConnectionRequestCallbackVtbl;

    interface IConnectionRequestCallback
    {
        CONST_VTBL struct IConnectionRequestCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConnectionRequestCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConnectionRequestCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConnectionRequestCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConnectionRequestCallback_OnComplete(This,hrStatus)	\
    ( (This)->lpVtbl -> OnComplete(This,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConnectionRequestCallback_INTERFACE_DEFINED__ */



#ifndef __PortableDeviceConnectApiLib_LIBRARY_DEFINED__
#define __PortableDeviceConnectApiLib_LIBRARY_DEFINED__

/* library PortableDeviceConnectApiLib */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_PortableDeviceConnectApiLib;

EXTERN_C const CLSID CLSID_EnumBthMtpConnectors;

#ifdef __cplusplus

class DECLSPEC_UUID("a1570149-e645-4f43-8b0d-409b061db2fc")
EnumBthMtpConnectors;
#endif
#endif /* __PortableDeviceConnectApiLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_PortableDeviceConnectApi_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceConnectApi_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceConnectApi_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


