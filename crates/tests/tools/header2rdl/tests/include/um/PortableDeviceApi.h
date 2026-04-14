

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

#ifndef __PortableDeviceApi_h__
#define __PortableDeviceApi_h__

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

#ifndef __IPortableDeviceManager_FWD_DEFINED__
#define __IPortableDeviceManager_FWD_DEFINED__
typedef interface IPortableDeviceManager IPortableDeviceManager;

#endif 	/* __IPortableDeviceManager_FWD_DEFINED__ */


#ifndef __IPortableDevice_FWD_DEFINED__
#define __IPortableDevice_FWD_DEFINED__
typedef interface IPortableDevice IPortableDevice;

#endif 	/* __IPortableDevice_FWD_DEFINED__ */


#ifndef __IPortableDeviceContent_FWD_DEFINED__
#define __IPortableDeviceContent_FWD_DEFINED__
typedef interface IPortableDeviceContent IPortableDeviceContent;

#endif 	/* __IPortableDeviceContent_FWD_DEFINED__ */


#ifndef __IPortableDeviceContent2_FWD_DEFINED__
#define __IPortableDeviceContent2_FWD_DEFINED__
typedef interface IPortableDeviceContent2 IPortableDeviceContent2;

#endif 	/* __IPortableDeviceContent2_FWD_DEFINED__ */


#ifndef __IEnumPortableDeviceObjectIDs_FWD_DEFINED__
#define __IEnumPortableDeviceObjectIDs_FWD_DEFINED__
typedef interface IEnumPortableDeviceObjectIDs IEnumPortableDeviceObjectIDs;

#endif 	/* __IEnumPortableDeviceObjectIDs_FWD_DEFINED__ */


#ifndef __IPortableDeviceProperties_FWD_DEFINED__
#define __IPortableDeviceProperties_FWD_DEFINED__
typedef interface IPortableDeviceProperties IPortableDeviceProperties;

#endif 	/* __IPortableDeviceProperties_FWD_DEFINED__ */


#ifndef __IPortableDeviceResources_FWD_DEFINED__
#define __IPortableDeviceResources_FWD_DEFINED__
typedef interface IPortableDeviceResources IPortableDeviceResources;

#endif 	/* __IPortableDeviceResources_FWD_DEFINED__ */


#ifndef __IPortableDeviceCapabilities_FWD_DEFINED__
#define __IPortableDeviceCapabilities_FWD_DEFINED__
typedef interface IPortableDeviceCapabilities IPortableDeviceCapabilities;

#endif 	/* __IPortableDeviceCapabilities_FWD_DEFINED__ */


#ifndef __IPortableDeviceEventCallback_FWD_DEFINED__
#define __IPortableDeviceEventCallback_FWD_DEFINED__
typedef interface IPortableDeviceEventCallback IPortableDeviceEventCallback;

#endif 	/* __IPortableDeviceEventCallback_FWD_DEFINED__ */


#ifndef __IPortableDeviceDataStream_FWD_DEFINED__
#define __IPortableDeviceDataStream_FWD_DEFINED__
typedef interface IPortableDeviceDataStream IPortableDeviceDataStream;

#endif 	/* __IPortableDeviceDataStream_FWD_DEFINED__ */


#ifndef __IPortableDeviceUnitsStream_FWD_DEFINED__
#define __IPortableDeviceUnitsStream_FWD_DEFINED__
typedef interface IPortableDeviceUnitsStream IPortableDeviceUnitsStream;

#endif 	/* __IPortableDeviceUnitsStream_FWD_DEFINED__ */


#ifndef __IPortableDevicePropertiesBulk_FWD_DEFINED__
#define __IPortableDevicePropertiesBulk_FWD_DEFINED__
typedef interface IPortableDevicePropertiesBulk IPortableDevicePropertiesBulk;

#endif 	/* __IPortableDevicePropertiesBulk_FWD_DEFINED__ */


#ifndef __IPortableDevicePropertiesBulkCallback_FWD_DEFINED__
#define __IPortableDevicePropertiesBulkCallback_FWD_DEFINED__
typedef interface IPortableDevicePropertiesBulkCallback IPortableDevicePropertiesBulkCallback;

#endif 	/* __IPortableDevicePropertiesBulkCallback_FWD_DEFINED__ */


#ifndef __IPortableDeviceServiceManager_FWD_DEFINED__
#define __IPortableDeviceServiceManager_FWD_DEFINED__
typedef interface IPortableDeviceServiceManager IPortableDeviceServiceManager;

#endif 	/* __IPortableDeviceServiceManager_FWD_DEFINED__ */


#ifndef __IPortableDeviceService_FWD_DEFINED__
#define __IPortableDeviceService_FWD_DEFINED__
typedef interface IPortableDeviceService IPortableDeviceService;

#endif 	/* __IPortableDeviceService_FWD_DEFINED__ */


#ifndef __IPortableDeviceServiceCapabilities_FWD_DEFINED__
#define __IPortableDeviceServiceCapabilities_FWD_DEFINED__
typedef interface IPortableDeviceServiceCapabilities IPortableDeviceServiceCapabilities;

#endif 	/* __IPortableDeviceServiceCapabilities_FWD_DEFINED__ */


#ifndef __IPortableDeviceServiceMethods_FWD_DEFINED__
#define __IPortableDeviceServiceMethods_FWD_DEFINED__
typedef interface IPortableDeviceServiceMethods IPortableDeviceServiceMethods;

#endif 	/* __IPortableDeviceServiceMethods_FWD_DEFINED__ */


#ifndef __IPortableDeviceServiceMethodCallback_FWD_DEFINED__
#define __IPortableDeviceServiceMethodCallback_FWD_DEFINED__
typedef interface IPortableDeviceServiceMethodCallback IPortableDeviceServiceMethodCallback;

#endif 	/* __IPortableDeviceServiceMethodCallback_FWD_DEFINED__ */


#ifndef __IPortableDeviceServiceActivation_FWD_DEFINED__
#define __IPortableDeviceServiceActivation_FWD_DEFINED__
typedef interface IPortableDeviceServiceActivation IPortableDeviceServiceActivation;

#endif 	/* __IPortableDeviceServiceActivation_FWD_DEFINED__ */


#ifndef __IPortableDeviceServiceOpenCallback_FWD_DEFINED__
#define __IPortableDeviceServiceOpenCallback_FWD_DEFINED__
typedef interface IPortableDeviceServiceOpenCallback IPortableDeviceServiceOpenCallback;

#endif 	/* __IPortableDeviceServiceOpenCallback_FWD_DEFINED__ */


#ifndef __IPortableDeviceDispatchFactory_FWD_DEFINED__
#define __IPortableDeviceDispatchFactory_FWD_DEFINED__
typedef interface IPortableDeviceDispatchFactory IPortableDeviceDispatchFactory;

#endif 	/* __IPortableDeviceDispatchFactory_FWD_DEFINED__ */


#ifndef __IPortableDeviceWebControl_FWD_DEFINED__
#define __IPortableDeviceWebControl_FWD_DEFINED__
typedef interface IPortableDeviceWebControl IPortableDeviceWebControl;

#endif 	/* __IPortableDeviceWebControl_FWD_DEFINED__ */


#ifndef __PortableDevice_FWD_DEFINED__
#define __PortableDevice_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDevice PortableDevice;
#else
typedef struct PortableDevice PortableDevice;
#endif /* __cplusplus */

#endif 	/* __PortableDevice_FWD_DEFINED__ */


#ifndef __PortableDeviceManager_FWD_DEFINED__
#define __PortableDeviceManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDeviceManager PortableDeviceManager;
#else
typedef struct PortableDeviceManager PortableDeviceManager;
#endif /* __cplusplus */

#endif 	/* __PortableDeviceManager_FWD_DEFINED__ */


#ifndef __PortableDeviceService_FWD_DEFINED__
#define __PortableDeviceService_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDeviceService PortableDeviceService;
#else
typedef struct PortableDeviceService PortableDeviceService;
#endif /* __cplusplus */

#endif 	/* __PortableDeviceService_FWD_DEFINED__ */


#ifndef __PortableDeviceDispatchFactory_FWD_DEFINED__
#define __PortableDeviceDispatchFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDeviceDispatchFactory PortableDeviceDispatchFactory;
#else
typedef struct PortableDeviceDispatchFactory PortableDeviceDispatchFactory;
#endif /* __cplusplus */

#endif 	/* __PortableDeviceDispatchFactory_FWD_DEFINED__ */


#ifndef __PortableDeviceFTM_FWD_DEFINED__
#define __PortableDeviceFTM_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDeviceFTM PortableDeviceFTM;
#else
typedef struct PortableDeviceFTM PortableDeviceFTM;
#endif /* __cplusplus */

#endif 	/* __PortableDeviceFTM_FWD_DEFINED__ */


#ifndef __PortableDeviceServiceFTM_FWD_DEFINED__
#define __PortableDeviceServiceFTM_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDeviceServiceFTM PortableDeviceServiceFTM;
#else
typedef struct PortableDeviceServiceFTM PortableDeviceServiceFTM;
#endif /* __cplusplus */

#endif 	/* __PortableDeviceServiceFTM_FWD_DEFINED__ */


#ifndef __PortableDeviceWebControl_FWD_DEFINED__
#define __PortableDeviceWebControl_FWD_DEFINED__

#ifdef __cplusplus
typedef class PortableDeviceWebControl PortableDeviceWebControl;
#else
typedef struct PortableDeviceWebControl PortableDeviceWebControl;
#endif /* __cplusplus */

#endif 	/* __PortableDeviceWebControl_FWD_DEFINED__ */


/* header files for imported files */
#include "propidl.h"
#include "PortableDeviceTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_PortableDeviceApi_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (_WIN32_WINNT >= 0x0501) // XP and later
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)






















extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0000_v0_0_s_ifspec;

#ifndef __IPortableDeviceManager_INTERFACE_DEFINED__
#define __IPortableDeviceManager_INTERFACE_DEFINED__

/* interface IPortableDeviceManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a1567595-4c2f-4574-a6fa-ecef917b9a40")
    IPortableDeviceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDevices( 
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *pPnPDeviceIDs,
            /* [out][in] */ __RPC__inout DWORD *pcPnPDeviceIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RefreshDeviceList( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceFriendlyName( 
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt WCHAR *pDeviceFriendlyName,
            /* [out][in] */ __RPC__inout DWORD *pcchDeviceFriendlyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceDescription( 
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt WCHAR *pDeviceDescription,
            /* [out][in] */ __RPC__inout DWORD *pcchDeviceDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceManufacturer( 
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt WCHAR *pDeviceManufacturer,
            /* [out][in] */ __RPC__inout DWORD *pcchDeviceManufacturer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceProperty( 
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [in] */ __RPC__in LPCWSTR pszDevicePropertyName,
            /* [unique][out][in] */ __RPC__inout_opt BYTE *pData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateDevices( 
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *pPnPDeviceIDs,
            /* [out][in] */ __RPC__inout DWORD *pcPnPDeviceIDs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, GetDevices)
        HRESULT ( STDMETHODCALLTYPE *GetDevices )( 
            __RPC__in IPortableDeviceManager * This,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *pPnPDeviceIDs,
            /* [out][in] */ __RPC__inout DWORD *pcPnPDeviceIDs);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, RefreshDeviceList)
        HRESULT ( STDMETHODCALLTYPE *RefreshDeviceList )( 
            __RPC__in IPortableDeviceManager * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, GetDeviceFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceFriendlyName )( 
            __RPC__in IPortableDeviceManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt WCHAR *pDeviceFriendlyName,
            /* [out][in] */ __RPC__inout DWORD *pcchDeviceFriendlyName);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, GetDeviceDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceDescription )( 
            __RPC__in IPortableDeviceManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt WCHAR *pDeviceDescription,
            /* [out][in] */ __RPC__inout DWORD *pcchDeviceDescription);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, GetDeviceManufacturer)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceManufacturer )( 
            __RPC__in IPortableDeviceManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [unique][out][in] */ __RPC__inout_opt WCHAR *pDeviceManufacturer,
            /* [out][in] */ __RPC__inout DWORD *pcchDeviceManufacturer);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, GetDeviceProperty)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceProperty )( 
            __RPC__in IPortableDeviceManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [in] */ __RPC__in LPCWSTR pszDevicePropertyName,
            /* [unique][out][in] */ __RPC__inout_opt BYTE *pData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pcbData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwType);
        
        DECLSPEC_XFGVIRT(IPortableDeviceManager, GetPrivateDevices)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateDevices )( 
            __RPC__in IPortableDeviceManager * This,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *pPnPDeviceIDs,
            /* [out][in] */ __RPC__inout DWORD *pcPnPDeviceIDs);
        
        END_INTERFACE
    } IPortableDeviceManagerVtbl;

    interface IPortableDeviceManager
    {
        CONST_VTBL struct IPortableDeviceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceManager_GetDevices(This,pPnPDeviceIDs,pcPnPDeviceIDs)	\
    ( (This)->lpVtbl -> GetDevices(This,pPnPDeviceIDs,pcPnPDeviceIDs) ) 

#define IPortableDeviceManager_RefreshDeviceList(This)	\
    ( (This)->lpVtbl -> RefreshDeviceList(This) ) 

#define IPortableDeviceManager_GetDeviceFriendlyName(This,pszPnPDeviceID,pDeviceFriendlyName,pcchDeviceFriendlyName)	\
    ( (This)->lpVtbl -> GetDeviceFriendlyName(This,pszPnPDeviceID,pDeviceFriendlyName,pcchDeviceFriendlyName) ) 

#define IPortableDeviceManager_GetDeviceDescription(This,pszPnPDeviceID,pDeviceDescription,pcchDeviceDescription)	\
    ( (This)->lpVtbl -> GetDeviceDescription(This,pszPnPDeviceID,pDeviceDescription,pcchDeviceDescription) ) 

#define IPortableDeviceManager_GetDeviceManufacturer(This,pszPnPDeviceID,pDeviceManufacturer,pcchDeviceManufacturer)	\
    ( (This)->lpVtbl -> GetDeviceManufacturer(This,pszPnPDeviceID,pDeviceManufacturer,pcchDeviceManufacturer) ) 

#define IPortableDeviceManager_GetDeviceProperty(This,pszPnPDeviceID,pszDevicePropertyName,pData,pcbData,pdwType)	\
    ( (This)->lpVtbl -> GetDeviceProperty(This,pszPnPDeviceID,pszDevicePropertyName,pData,pcbData,pdwType) ) 

#define IPortableDeviceManager_GetPrivateDevices(This,pPnPDeviceIDs,pcPnPDeviceIDs)	\
    ( (This)->lpVtbl -> GetPrivateDevices(This,pPnPDeviceIDs,pcPnPDeviceIDs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceManager_INTERFACE_DEFINED__ */


#ifndef __IPortableDevice_INTERFACE_DEFINED__
#define __IPortableDevice_INTERFACE_DEFINED__

/* interface IPortableDevice */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("625e2df8-6392-4cf0-9ad1-3cfa5f17775c")
    IPortableDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pClientInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendCommand( 
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Content( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceContent **ppContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Capabilities( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceCapabilities **ppCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceEventCallback *pCallback,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ __RPC__in LPCWSTR pszCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPnPDeviceID( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPnPDeviceID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDevice * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDevice * This);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IPortableDevice * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pClientInfo);
        
        DECLSPEC_XFGVIRT(IPortableDevice, SendCommand)
        HRESULT ( STDMETHODCALLTYPE *SendCommand )( 
            __RPC__in IPortableDevice * This,
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Content)
        HRESULT ( STDMETHODCALLTYPE *Content )( 
            __RPC__in IPortableDevice * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceContent **ppContent);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Capabilities)
        HRESULT ( STDMETHODCALLTYPE *Capabilities )( 
            __RPC__in IPortableDevice * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceCapabilities **ppCapabilities);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDevice * This);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IPortableDevice * This);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in IPortableDevice * This,
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceEventCallback *pCallback,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCookie);
        
        DECLSPEC_XFGVIRT(IPortableDevice, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in IPortableDevice * This,
            /* [in] */ __RPC__in LPCWSTR pszCookie);
        
        DECLSPEC_XFGVIRT(IPortableDevice, GetPnPDeviceID)
        HRESULT ( STDMETHODCALLTYPE *GetPnPDeviceID )( 
            __RPC__in IPortableDevice * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPnPDeviceID);
        
        END_INTERFACE
    } IPortableDeviceVtbl;

    interface IPortableDevice
    {
        CONST_VTBL struct IPortableDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDevice_Open(This,pszPnPDeviceID,pClientInfo)	\
    ( (This)->lpVtbl -> Open(This,pszPnPDeviceID,pClientInfo) ) 

#define IPortableDevice_SendCommand(This,dwFlags,pParameters,ppResults)	\
    ( (This)->lpVtbl -> SendCommand(This,dwFlags,pParameters,ppResults) ) 

#define IPortableDevice_Content(This,ppContent)	\
    ( (This)->lpVtbl -> Content(This,ppContent) ) 

#define IPortableDevice_Capabilities(This,ppCapabilities)	\
    ( (This)->lpVtbl -> Capabilities(This,ppCapabilities) ) 

#define IPortableDevice_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IPortableDevice_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IPortableDevice_Advise(This,dwFlags,pCallback,pParameters,ppszCookie)	\
    ( (This)->lpVtbl -> Advise(This,dwFlags,pCallback,pParameters,ppszCookie) ) 

#define IPortableDevice_Unadvise(This,pszCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,pszCookie) ) 

#define IPortableDevice_GetPnPDeviceID(This,ppszPnPDeviceID)	\
    ( (This)->lpVtbl -> GetPnPDeviceID(This,ppszPnPDeviceID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDevice_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceContent_INTERFACE_DEFINED__
#define __IPortableDeviceContent_INTERFACE_DEFINED__

/* interface IPortableDeviceContent */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6a96ed84-7c73-4480-9938-bf5af477d426")
    IPortableDeviceContent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumObjects( 
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in LPCWSTR pszParentObjectID,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pFilter,
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceObjectIDs **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Properties( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceProperties **ppProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Transfer( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceResources **ppResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateObjectWithPropertiesOnly( 
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszObjectID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateObjectWithPropertiesAndData( 
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ const DWORD dwOptions,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectIDsFromPersistentUniqueIDs( 
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pPersistentUniqueIDs,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppObjectIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in LPCWSTR pszDestinationFolderObjectID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Copy( 
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in LPCWSTR pszDestinationFolderObjectID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceContent * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, EnumObjects)
        HRESULT ( STDMETHODCALLTYPE *EnumObjects )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in LPCWSTR pszParentObjectID,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pFilter,
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceObjectIDs **ppEnum);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Properties)
        HRESULT ( STDMETHODCALLTYPE *Properties )( 
            __RPC__in IPortableDeviceContent * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Transfer)
        HRESULT ( STDMETHODCALLTYPE *Transfer )( 
            __RPC__in IPortableDeviceContent * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceResources **ppResources);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, CreateObjectWithPropertiesOnly)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectWithPropertiesOnly )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszObjectID);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, CreateObjectWithPropertiesAndData)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectWithPropertiesAndData )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszCookie);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ const DWORD dwOptions,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, GetObjectIDsFromPersistentUniqueIDs)
        HRESULT ( STDMETHODCALLTYPE *GetObjectIDsFromPersistentUniqueIDs )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pPersistentUniqueIDs,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppObjectIDs);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceContent * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in LPCWSTR pszDestinationFolderObjectID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Copy)
        HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in IPortableDeviceContent * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in LPCWSTR pszDestinationFolderObjectID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults);
        
        END_INTERFACE
    } IPortableDeviceContentVtbl;

    interface IPortableDeviceContent
    {
        CONST_VTBL struct IPortableDeviceContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceContent_EnumObjects(This,dwFlags,pszParentObjectID,pFilter,ppEnum)	\
    ( (This)->lpVtbl -> EnumObjects(This,dwFlags,pszParentObjectID,pFilter,ppEnum) ) 

#define IPortableDeviceContent_Properties(This,ppProperties)	\
    ( (This)->lpVtbl -> Properties(This,ppProperties) ) 

#define IPortableDeviceContent_Transfer(This,ppResources)	\
    ( (This)->lpVtbl -> Transfer(This,ppResources) ) 

#define IPortableDeviceContent_CreateObjectWithPropertiesOnly(This,pValues,ppszObjectID)	\
    ( (This)->lpVtbl -> CreateObjectWithPropertiesOnly(This,pValues,ppszObjectID) ) 

#define IPortableDeviceContent_CreateObjectWithPropertiesAndData(This,pValues,ppData,pdwOptimalWriteBufferSize,ppszCookie)	\
    ( (This)->lpVtbl -> CreateObjectWithPropertiesAndData(This,pValues,ppData,pdwOptimalWriteBufferSize,ppszCookie) ) 

#define IPortableDeviceContent_Delete(This,dwOptions,pObjectIDs,ppResults)	\
    ( (This)->lpVtbl -> Delete(This,dwOptions,pObjectIDs,ppResults) ) 

#define IPortableDeviceContent_GetObjectIDsFromPersistentUniqueIDs(This,pPersistentUniqueIDs,ppObjectIDs)	\
    ( (This)->lpVtbl -> GetObjectIDsFromPersistentUniqueIDs(This,pPersistentUniqueIDs,ppObjectIDs) ) 

#define IPortableDeviceContent_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IPortableDeviceContent_Move(This,pObjectIDs,pszDestinationFolderObjectID,ppResults)	\
    ( (This)->lpVtbl -> Move(This,pObjectIDs,pszDestinationFolderObjectID,ppResults) ) 

#define IPortableDeviceContent_Copy(This,pObjectIDs,pszDestinationFolderObjectID,ppResults)	\
    ( (This)->lpVtbl -> Copy(This,pObjectIDs,pszDestinationFolderObjectID,ppResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceContent_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceContent2_INTERFACE_DEFINED__
#define __IPortableDeviceContent2_INTERFACE_DEFINED__

/* interface IPortableDeviceContent2 */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceContent2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9b4add96-f6bf-4034-8708-eca72bf10554")
    IPortableDeviceContent2 : public IPortableDeviceContent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UpdateObjectWithPropertiesAndData( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pProperties,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceContent2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceContent2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceContent2 * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, EnumObjects)
        HRESULT ( STDMETHODCALLTYPE *EnumObjects )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in LPCWSTR pszParentObjectID,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pFilter,
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceObjectIDs **ppEnum);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Properties)
        HRESULT ( STDMETHODCALLTYPE *Properties )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceProperties **ppProperties);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Transfer)
        HRESULT ( STDMETHODCALLTYPE *Transfer )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceResources **ppResources);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, CreateObjectWithPropertiesOnly)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectWithPropertiesOnly )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszObjectID);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, CreateObjectWithPropertiesAndData)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectWithPropertiesAndData )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszCookie);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ const DWORD dwOptions,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, GetObjectIDsFromPersistentUniqueIDs)
        HRESULT ( STDMETHODCALLTYPE *GetObjectIDsFromPersistentUniqueIDs )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pPersistentUniqueIDs,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppObjectIDs);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceContent2 * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in LPCWSTR pszDestinationFolderObjectID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent, Copy)
        HRESULT ( STDMETHODCALLTYPE *Copy )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in LPCWSTR pszDestinationFolderObjectID,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDevicePropVariantCollection **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceContent2, UpdateObjectWithPropertiesAndData)
        HRESULT ( STDMETHODCALLTYPE *UpdateObjectWithPropertiesAndData )( 
            __RPC__in IPortableDeviceContent2 * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pProperties,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize);
        
        END_INTERFACE
    } IPortableDeviceContent2Vtbl;

    interface IPortableDeviceContent2
    {
        CONST_VTBL struct IPortableDeviceContent2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceContent2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceContent2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceContent2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceContent2_EnumObjects(This,dwFlags,pszParentObjectID,pFilter,ppEnum)	\
    ( (This)->lpVtbl -> EnumObjects(This,dwFlags,pszParentObjectID,pFilter,ppEnum) ) 

#define IPortableDeviceContent2_Properties(This,ppProperties)	\
    ( (This)->lpVtbl -> Properties(This,ppProperties) ) 

#define IPortableDeviceContent2_Transfer(This,ppResources)	\
    ( (This)->lpVtbl -> Transfer(This,ppResources) ) 

#define IPortableDeviceContent2_CreateObjectWithPropertiesOnly(This,pValues,ppszObjectID)	\
    ( (This)->lpVtbl -> CreateObjectWithPropertiesOnly(This,pValues,ppszObjectID) ) 

#define IPortableDeviceContent2_CreateObjectWithPropertiesAndData(This,pValues,ppData,pdwOptimalWriteBufferSize,ppszCookie)	\
    ( (This)->lpVtbl -> CreateObjectWithPropertiesAndData(This,pValues,ppData,pdwOptimalWriteBufferSize,ppszCookie) ) 

#define IPortableDeviceContent2_Delete(This,dwOptions,pObjectIDs,ppResults)	\
    ( (This)->lpVtbl -> Delete(This,dwOptions,pObjectIDs,ppResults) ) 

#define IPortableDeviceContent2_GetObjectIDsFromPersistentUniqueIDs(This,pPersistentUniqueIDs,ppObjectIDs)	\
    ( (This)->lpVtbl -> GetObjectIDsFromPersistentUniqueIDs(This,pPersistentUniqueIDs,ppObjectIDs) ) 

#define IPortableDeviceContent2_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IPortableDeviceContent2_Move(This,pObjectIDs,pszDestinationFolderObjectID,ppResults)	\
    ( (This)->lpVtbl -> Move(This,pObjectIDs,pszDestinationFolderObjectID,ppResults) ) 

#define IPortableDeviceContent2_Copy(This,pObjectIDs,pszDestinationFolderObjectID,ppResults)	\
    ( (This)->lpVtbl -> Copy(This,pObjectIDs,pszDestinationFolderObjectID,ppResults) ) 


#define IPortableDeviceContent2_UpdateObjectWithPropertiesAndData(This,pszObjectID,pProperties,ppData,pdwOptimalWriteBufferSize)	\
    ( (This)->lpVtbl -> UpdateObjectWithPropertiesAndData(This,pszObjectID,pProperties,ppData,pdwOptimalWriteBufferSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceContent2_INTERFACE_DEFINED__ */


#ifndef __IEnumPortableDeviceObjectIDs_INTERFACE_DEFINED__
#define __IEnumPortableDeviceObjectIDs_INTERFACE_DEFINED__

/* interface IEnumPortableDeviceObjectIDs */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IEnumPortableDeviceObjectIDs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("10ece955-cf41-4728-bfa0-41eedf1bbf19")
    IEnumPortableDeviceObjectIDs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cObjects,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cObjects, *pcFetched) LPWSTR *pObjIDs,
            /* [unique][out][in] */ __RPC__inout_opt ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cObjects) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceObjectIDs **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPortableDeviceObjectIDsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceObjectIDs, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This,
            /* [in] */ ULONG cObjects,
            /* [length_is][size_is][out] */ __RPC__out_ecount_part(cObjects, *pcFetched) LPWSTR *pObjIDs,
            /* [unique][out][in] */ __RPC__inout_opt ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceObjectIDs, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This,
            /* [in] */ ULONG cObjects);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceObjectIDs, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceObjectIDs, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This,
            /* [out] */ __RPC__deref_out_opt IEnumPortableDeviceObjectIDs **ppEnum);
        
        DECLSPEC_XFGVIRT(IEnumPortableDeviceObjectIDs, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IEnumPortableDeviceObjectIDs * This);
        
        END_INTERFACE
    } IEnumPortableDeviceObjectIDsVtbl;

    interface IEnumPortableDeviceObjectIDs
    {
        CONST_VTBL struct IEnumPortableDeviceObjectIDsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPortableDeviceObjectIDs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPortableDeviceObjectIDs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPortableDeviceObjectIDs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPortableDeviceObjectIDs_Next(This,cObjects,pObjIDs,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cObjects,pObjIDs,pcFetched) ) 

#define IEnumPortableDeviceObjectIDs_Skip(This,cObjects)	\
    ( (This)->lpVtbl -> Skip(This,cObjects) ) 

#define IEnumPortableDeviceObjectIDs_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPortableDeviceObjectIDs_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#define IEnumPortableDeviceObjectIDs_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPortableDeviceObjectIDs_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceProperties_INTERFACE_DEFINED__
#define __IPortableDeviceProperties_INTERFACE_DEFINED__

/* interface IPortableDeviceProperties */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7f6d695c-03df-4439-a809-59266beee3a6")
    IPortableDeviceProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedProperties( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyAttributes( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValues( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppValues) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValues( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDevicePropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceProperties * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceProperties, GetSupportedProperties)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedProperties )( 
            __RPC__in IPortableDeviceProperties * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys);
        
        DECLSPEC_XFGVIRT(IPortableDeviceProperties, GetPropertyAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyAttributes )( 
            __RPC__in IPortableDeviceProperties * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceProperties, GetValues)
        HRESULT ( STDMETHODCALLTYPE *GetValues )( 
            __RPC__in IPortableDeviceProperties * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppValues);
        
        DECLSPEC_XFGVIRT(IPortableDeviceProperties, SetValues)
        HRESULT ( STDMETHODCALLTYPE *SetValues )( 
            __RPC__in IPortableDeviceProperties * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pValues,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceProperties, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IPortableDeviceProperties * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys);
        
        DECLSPEC_XFGVIRT(IPortableDeviceProperties, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceProperties * This);
        
        END_INTERFACE
    } IPortableDevicePropertiesVtbl;

    interface IPortableDeviceProperties
    {
        CONST_VTBL struct IPortableDevicePropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceProperties_GetSupportedProperties(This,pszObjectID,ppKeys)	\
    ( (This)->lpVtbl -> GetSupportedProperties(This,pszObjectID,ppKeys) ) 

#define IPortableDeviceProperties_GetPropertyAttributes(This,pszObjectID,Key,ppAttributes)	\
    ( (This)->lpVtbl -> GetPropertyAttributes(This,pszObjectID,Key,ppAttributes) ) 

#define IPortableDeviceProperties_GetValues(This,pszObjectID,pKeys,ppValues)	\
    ( (This)->lpVtbl -> GetValues(This,pszObjectID,pKeys,ppValues) ) 

#define IPortableDeviceProperties_SetValues(This,pszObjectID,pValues,ppResults)	\
    ( (This)->lpVtbl -> SetValues(This,pszObjectID,pValues,ppResults) ) 

#define IPortableDeviceProperties_Delete(This,pszObjectID,pKeys)	\
    ( (This)->lpVtbl -> Delete(This,pszObjectID,pKeys) ) 

#define IPortableDeviceProperties_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceProperties_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceResources_INTERFACE_DEFINED__
#define __IPortableDeviceResources_INTERFACE_DEFINED__

/* interface IPortableDeviceResources */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceResources;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fd8878ac-d841-4d17-891c-e6829cdb6934")
    IPortableDeviceResources : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedResources( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResourceAttributes( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResourceAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [in] */ const DWORD dwMode,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalBufferSize,
            /* [out] */ __RPC__deref_out_opt IStream **ppStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Delete( 
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateResource( 
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pResourceAttributes,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceResourcesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceResources * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceResources * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceResources, GetSupportedResources)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedResources )( 
            __RPC__in IPortableDeviceResources * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys);
        
        DECLSPEC_XFGVIRT(IPortableDeviceResources, GetResourceAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetResourceAttributes )( 
            __RPC__in IPortableDeviceResources * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResourceAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceResources, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IPortableDeviceResources * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [in] */ const DWORD dwMode,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalBufferSize,
            /* [out] */ __RPC__deref_out_opt IStream **ppStream);
        
        DECLSPEC_XFGVIRT(IPortableDeviceResources, Delete)
        HRESULT ( STDMETHODCALLTYPE *Delete )( 
            __RPC__in IPortableDeviceResources * This,
            /* [in] */ __RPC__in LPCWSTR pszObjectID,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys);
        
        DECLSPEC_XFGVIRT(IPortableDeviceResources, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceResources * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceResources, CreateResource)
        HRESULT ( STDMETHODCALLTYPE *CreateResource )( 
            __RPC__in IPortableDeviceResources * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pResourceAttributes,
            /* [out] */ __RPC__deref_out_opt IStream **ppData,
            /* [unique][out][in] */ __RPC__inout_opt DWORD *pdwOptimalWriteBufferSize,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *ppszCookie);
        
        END_INTERFACE
    } IPortableDeviceResourcesVtbl;

    interface IPortableDeviceResources
    {
        CONST_VTBL struct IPortableDeviceResourcesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceResources_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceResources_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceResources_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceResources_GetSupportedResources(This,pszObjectID,ppKeys)	\
    ( (This)->lpVtbl -> GetSupportedResources(This,pszObjectID,ppKeys) ) 

#define IPortableDeviceResources_GetResourceAttributes(This,pszObjectID,Key,ppResourceAttributes)	\
    ( (This)->lpVtbl -> GetResourceAttributes(This,pszObjectID,Key,ppResourceAttributes) ) 

#define IPortableDeviceResources_GetStream(This,pszObjectID,Key,dwMode,pdwOptimalBufferSize,ppStream)	\
    ( (This)->lpVtbl -> GetStream(This,pszObjectID,Key,dwMode,pdwOptimalBufferSize,ppStream) ) 

#define IPortableDeviceResources_Delete(This,pszObjectID,pKeys)	\
    ( (This)->lpVtbl -> Delete(This,pszObjectID,pKeys) ) 

#define IPortableDeviceResources_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IPortableDeviceResources_CreateResource(This,pResourceAttributes,ppData,pdwOptimalWriteBufferSize,ppszCookie)	\
    ( (This)->lpVtbl -> CreateResource(This,pResourceAttributes,ppData,pdwOptimalWriteBufferSize,ppszCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceResources_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceCapabilities_INTERFACE_DEFINED__
#define __IPortableDeviceCapabilities_INTERFACE_DEFINED__

/* interface IPortableDeviceCapabilities */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2c8c6dbf-e3dc-4061-becc-8542e810d126")
    IPortableDeviceCapabilities : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedCommands( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppCommands) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCommandOptions( 
            /* [in] */ __RPC__in REFPROPERTYKEY Command,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFunctionalCategories( 
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppCategories) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFunctionalObjects( 
            /* [in] */ __RPC__in REFGUID Category,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppObjectIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedContentTypes( 
            /* [in] */ __RPC__in REFGUID Category,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppContentTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedFormats( 
            /* [in] */ __RPC__in REFGUID ContentType,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedFormatProperties( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFixedPropertyAttributes( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedEvents( 
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventOptions( 
            /* [in] */ __RPC__in REFGUID Event,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppOptions) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceCapabilities * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetSupportedCommands)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedCommands )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppCommands);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetCommandOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCommandOptions )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFPROPERTYKEY Command,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppOptions);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetFunctionalCategories)
        HRESULT ( STDMETHODCALLTYPE *GetFunctionalCategories )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppCategories);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetFunctionalObjects)
        HRESULT ( STDMETHODCALLTYPE *GetFunctionalObjects )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Category,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppObjectIDs);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetSupportedContentTypes)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedContentTypes )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Category,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppContentTypes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetSupportedFormats)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedFormats )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFGUID ContentType,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppFormats);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetSupportedFormatProperties)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedFormatProperties )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetFixedPropertyAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetFixedPropertyAttributes )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [in] */ __RPC__in REFPROPERTYKEY Key,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceCapabilities * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetSupportedEvents)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedEvents )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppEvents);
        
        DECLSPEC_XFGVIRT(IPortableDeviceCapabilities, GetEventOptions)
        HRESULT ( STDMETHODCALLTYPE *GetEventOptions )( 
            __RPC__in IPortableDeviceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Event,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppOptions);
        
        END_INTERFACE
    } IPortableDeviceCapabilitiesVtbl;

    interface IPortableDeviceCapabilities
    {
        CONST_VTBL struct IPortableDeviceCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceCapabilities_GetSupportedCommands(This,ppCommands)	\
    ( (This)->lpVtbl -> GetSupportedCommands(This,ppCommands) ) 

#define IPortableDeviceCapabilities_GetCommandOptions(This,Command,ppOptions)	\
    ( (This)->lpVtbl -> GetCommandOptions(This,Command,ppOptions) ) 

#define IPortableDeviceCapabilities_GetFunctionalCategories(This,ppCategories)	\
    ( (This)->lpVtbl -> GetFunctionalCategories(This,ppCategories) ) 

#define IPortableDeviceCapabilities_GetFunctionalObjects(This,Category,ppObjectIDs)	\
    ( (This)->lpVtbl -> GetFunctionalObjects(This,Category,ppObjectIDs) ) 

#define IPortableDeviceCapabilities_GetSupportedContentTypes(This,Category,ppContentTypes)	\
    ( (This)->lpVtbl -> GetSupportedContentTypes(This,Category,ppContentTypes) ) 

#define IPortableDeviceCapabilities_GetSupportedFormats(This,ContentType,ppFormats)	\
    ( (This)->lpVtbl -> GetSupportedFormats(This,ContentType,ppFormats) ) 

#define IPortableDeviceCapabilities_GetSupportedFormatProperties(This,Format,ppKeys)	\
    ( (This)->lpVtbl -> GetSupportedFormatProperties(This,Format,ppKeys) ) 

#define IPortableDeviceCapabilities_GetFixedPropertyAttributes(This,Format,Key,ppAttributes)	\
    ( (This)->lpVtbl -> GetFixedPropertyAttributes(This,Format,Key,ppAttributes) ) 

#define IPortableDeviceCapabilities_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IPortableDeviceCapabilities_GetSupportedEvents(This,ppEvents)	\
    ( (This)->lpVtbl -> GetSupportedEvents(This,ppEvents) ) 

#define IPortableDeviceCapabilities_GetEventOptions(This,Event,ppOptions)	\
    ( (This)->lpVtbl -> GetEventOptions(This,Event,ppOptions) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceCapabilities_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceEventCallback_INTERFACE_DEFINED__
#define __IPortableDeviceEventCallback_INTERFACE_DEFINED__

/* interface IPortableDeviceEventCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceEventCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a8792a31-f385-493c-a893-40f64eb45f6e")
    IPortableDeviceEventCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pEventParameters) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceEventCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceEventCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceEventCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceEventCallback * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceEventCallback, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            __RPC__in IPortableDeviceEventCallback * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pEventParameters);
        
        END_INTERFACE
    } IPortableDeviceEventCallbackVtbl;

    interface IPortableDeviceEventCallback
    {
        CONST_VTBL struct IPortableDeviceEventCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceEventCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceEventCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceEventCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceEventCallback_OnEvent(This,pEventParameters)	\
    ( (This)->lpVtbl -> OnEvent(This,pEventParameters) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceEventCallback_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceDataStream_INTERFACE_DEFINED__
#define __IPortableDeviceDataStream_INTERFACE_DEFINED__

/* interface IPortableDeviceDataStream */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceDataStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("88e04db3-1012-4d64-9996-f703a950d3f4")
    IPortableDeviceDataStream : public IStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetObjectID( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszObjectID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceDataStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceDataStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceDataStream * This);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IPortableDeviceDataStream * This,
            /* [annotation] */ 
            _Out_writes_bytes_to_(cb, *pcbRead)  void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(ISequentialStream, Write)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Write )( 
            IPortableDeviceDataStream * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const void *pv,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation] */ 
            _Out_opt_  ULONG *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Seek)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Seek )( 
            IPortableDeviceDataStream * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IStream, SetSize)
        HRESULT ( STDMETHODCALLTYPE *SetSize )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [in] */ ULARGE_INTEGER libNewSize);
        
        DECLSPEC_XFGVIRT(IStream, CopyTo)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CopyTo )( 
            IPortableDeviceDataStream * This,
            /* [annotation][unique][in] */ 
            _In_  IStream *pstm,
            /* [in] */ ULARGE_INTEGER cb,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbRead,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *pcbWritten);
        
        DECLSPEC_XFGVIRT(IStream, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [in] */ DWORD grfCommitFlags);
        
        DECLSPEC_XFGVIRT(IStream, Revert)
        HRESULT ( STDMETHODCALLTYPE *Revert )( 
            __RPC__in IPortableDeviceDataStream * This);
        
        DECLSPEC_XFGVIRT(IStream, LockRegion)
        HRESULT ( STDMETHODCALLTYPE *LockRegion )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, UnlockRegion)
        HRESULT ( STDMETHODCALLTYPE *UnlockRegion )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [in] */ ULARGE_INTEGER libOffset,
            /* [in] */ ULARGE_INTEGER cb,
            /* [in] */ DWORD dwLockType);
        
        DECLSPEC_XFGVIRT(IStream, Stat)
        HRESULT ( STDMETHODCALLTYPE *Stat )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [out] */ __RPC__out STATSTG *pstatstg,
            /* [in] */ DWORD grfStatFlag);
        
        DECLSPEC_XFGVIRT(IStream, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [out] */ __RPC__deref_out_opt IStream **ppstm);
        
        DECLSPEC_XFGVIRT(IPortableDeviceDataStream, GetObjectID)
        HRESULT ( STDMETHODCALLTYPE *GetObjectID )( 
            __RPC__in IPortableDeviceDataStream * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszObjectID);
        
        DECLSPEC_XFGVIRT(IPortableDeviceDataStream, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceDataStream * This);
        
        END_INTERFACE
    } IPortableDeviceDataStreamVtbl;

    interface IPortableDeviceDataStream
    {
        CONST_VTBL struct IPortableDeviceDataStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceDataStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceDataStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceDataStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceDataStream_Read(This,pv,cb,pcbRead)	\
    ( (This)->lpVtbl -> Read(This,pv,cb,pcbRead) ) 

#define IPortableDeviceDataStream_Write(This,pv,cb,pcbWritten)	\
    ( (This)->lpVtbl -> Write(This,pv,cb,pcbWritten) ) 


#define IPortableDeviceDataStream_Seek(This,dlibMove,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> Seek(This,dlibMove,dwOrigin,plibNewPosition) ) 

#define IPortableDeviceDataStream_SetSize(This,libNewSize)	\
    ( (This)->lpVtbl -> SetSize(This,libNewSize) ) 

#define IPortableDeviceDataStream_CopyTo(This,pstm,cb,pcbRead,pcbWritten)	\
    ( (This)->lpVtbl -> CopyTo(This,pstm,cb,pcbRead,pcbWritten) ) 

#define IPortableDeviceDataStream_Commit(This,grfCommitFlags)	\
    ( (This)->lpVtbl -> Commit(This,grfCommitFlags) ) 

#define IPortableDeviceDataStream_Revert(This)	\
    ( (This)->lpVtbl -> Revert(This) ) 

#define IPortableDeviceDataStream_LockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> LockRegion(This,libOffset,cb,dwLockType) ) 

#define IPortableDeviceDataStream_UnlockRegion(This,libOffset,cb,dwLockType)	\
    ( (This)->lpVtbl -> UnlockRegion(This,libOffset,cb,dwLockType) ) 

#define IPortableDeviceDataStream_Stat(This,pstatstg,grfStatFlag)	\
    ( (This)->lpVtbl -> Stat(This,pstatstg,grfStatFlag) ) 

#define IPortableDeviceDataStream_Clone(This,ppstm)	\
    ( (This)->lpVtbl -> Clone(This,ppstm) ) 


#define IPortableDeviceDataStream_GetObjectID(This,ppszObjectID)	\
    ( (This)->lpVtbl -> GetObjectID(This,ppszObjectID) ) 

#define IPortableDeviceDataStream_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceDataStream_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceUnitsStream_INTERFACE_DEFINED__
#define __IPortableDeviceUnitsStream_INTERFACE_DEFINED__

/* interface IPortableDeviceUnitsStream */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceUnitsStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e98025f-bfc4-47a2-9a5f-bc900a507c67")
    IPortableDeviceUnitsStream : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SeekInUnits( 
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ WPD_STREAM_UNITS units,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceUnitsStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceUnitsStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceUnitsStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceUnitsStream * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceUnitsStream, SeekInUnits)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SeekInUnits )( 
            IPortableDeviceUnitsStream * This,
            /* [in] */ LARGE_INTEGER dlibMove,
            /* [in] */ WPD_STREAM_UNITS units,
            /* [in] */ DWORD dwOrigin,
            /* [annotation] */ 
            _Out_opt_  ULARGE_INTEGER *plibNewPosition);
        
        DECLSPEC_XFGVIRT(IPortableDeviceUnitsStream, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceUnitsStream * This);
        
        END_INTERFACE
    } IPortableDeviceUnitsStreamVtbl;

    interface IPortableDeviceUnitsStream
    {
        CONST_VTBL struct IPortableDeviceUnitsStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceUnitsStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceUnitsStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceUnitsStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceUnitsStream_SeekInUnits(This,dlibMove,units,dwOrigin,plibNewPosition)	\
    ( (This)->lpVtbl -> SeekInUnits(This,dlibMove,units,dwOrigin,plibNewPosition) ) 

#define IPortableDeviceUnitsStream_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IPortableDeviceUnitsStream_RemoteSeekInUnits_Proxy( 
    __RPC__in IPortableDeviceUnitsStream * This,
    /* [in] */ LARGE_INTEGER dlibMove,
    /* [in] */ WPD_STREAM_UNITS units,
    /* [in] */ DWORD dwOrigin,
    /* [out] */ __RPC__out ULARGE_INTEGER *plibNewPosition);


void __RPC_STUB IPortableDeviceUnitsStream_RemoteSeekInUnits_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IPortableDeviceUnitsStream_INTERFACE_DEFINED__ */


#ifndef __IPortableDevicePropertiesBulk_INTERFACE_DEFINED__
#define __IPortableDevicePropertiesBulk_INTERFACE_DEFINED__

/* interface IPortableDevicePropertiesBulk */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDevicePropertiesBulk;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("482b05c0-4056-44ed-9e0f-5e23b009da93")
    IPortableDevicePropertiesBulk : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueueGetValuesByObjectList( 
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [in] */ __RPC__in_opt IPortableDevicePropertiesBulkCallback *pCallback,
            /* [out] */ __RPC__out GUID *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueueGetValuesByObjectFormat( 
            /* [in] */ __RPC__in REFGUID pguidObjectFormat,
            /* [in] */ __RPC__in LPCWSTR pszParentObjectID,
            /* [in] */ const DWORD dwDepth,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [in] */ __RPC__in_opt IPortableDevicePropertiesBulkCallback *pCallback,
            /* [out] */ __RPC__out GUID *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueueSetValuesByObjectList( 
            /* [in] */ __RPC__in_opt IPortableDeviceValuesCollection *pObjectValues,
            /* [in] */ __RPC__in_opt IPortableDevicePropertiesBulkCallback *pCallback,
            /* [out] */ __RPC__out GUID *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ __RPC__in REFGUID pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( 
            /* [in] */ __RPC__in REFGUID pContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDevicePropertiesBulkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDevicePropertiesBulk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDevicePropertiesBulk * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDevicePropertiesBulk * This);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulk, QueueGetValuesByObjectList)
        HRESULT ( STDMETHODCALLTYPE *QueueGetValuesByObjectList )( 
            __RPC__in IPortableDevicePropertiesBulk * This,
            /* [in] */ __RPC__in_opt IPortableDevicePropVariantCollection *pObjectIDs,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [in] */ __RPC__in_opt IPortableDevicePropertiesBulkCallback *pCallback,
            /* [out] */ __RPC__out GUID *pContext);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulk, QueueGetValuesByObjectFormat)
        HRESULT ( STDMETHODCALLTYPE *QueueGetValuesByObjectFormat )( 
            __RPC__in IPortableDevicePropertiesBulk * This,
            /* [in] */ __RPC__in REFGUID pguidObjectFormat,
            /* [in] */ __RPC__in LPCWSTR pszParentObjectID,
            /* [in] */ const DWORD dwDepth,
            /* [in] */ __RPC__in_opt IPortableDeviceKeyCollection *pKeys,
            /* [in] */ __RPC__in_opt IPortableDevicePropertiesBulkCallback *pCallback,
            /* [out] */ __RPC__out GUID *pContext);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulk, QueueSetValuesByObjectList)
        HRESULT ( STDMETHODCALLTYPE *QueueSetValuesByObjectList )( 
            __RPC__in IPortableDevicePropertiesBulk * This,
            /* [in] */ __RPC__in_opt IPortableDeviceValuesCollection *pObjectValues,
            /* [in] */ __RPC__in_opt IPortableDevicePropertiesBulkCallback *pCallback,
            /* [out] */ __RPC__out GUID *pContext);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulk, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IPortableDevicePropertiesBulk * This,
            /* [in] */ __RPC__in REFGUID pContext);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulk, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDevicePropertiesBulk * This,
            /* [in] */ __RPC__in REFGUID pContext);
        
        END_INTERFACE
    } IPortableDevicePropertiesBulkVtbl;

    interface IPortableDevicePropertiesBulk
    {
        CONST_VTBL struct IPortableDevicePropertiesBulkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDevicePropertiesBulk_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDevicePropertiesBulk_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDevicePropertiesBulk_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDevicePropertiesBulk_QueueGetValuesByObjectList(This,pObjectIDs,pKeys,pCallback,pContext)	\
    ( (This)->lpVtbl -> QueueGetValuesByObjectList(This,pObjectIDs,pKeys,pCallback,pContext) ) 

#define IPortableDevicePropertiesBulk_QueueGetValuesByObjectFormat(This,pguidObjectFormat,pszParentObjectID,dwDepth,pKeys,pCallback,pContext)	\
    ( (This)->lpVtbl -> QueueGetValuesByObjectFormat(This,pguidObjectFormat,pszParentObjectID,dwDepth,pKeys,pCallback,pContext) ) 

#define IPortableDevicePropertiesBulk_QueueSetValuesByObjectList(This,pObjectValues,pCallback,pContext)	\
    ( (This)->lpVtbl -> QueueSetValuesByObjectList(This,pObjectValues,pCallback,pContext) ) 

#define IPortableDevicePropertiesBulk_Start(This,pContext)	\
    ( (This)->lpVtbl -> Start(This,pContext) ) 

#define IPortableDevicePropertiesBulk_Cancel(This,pContext)	\
    ( (This)->lpVtbl -> Cancel(This,pContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDevicePropertiesBulk_INTERFACE_DEFINED__ */


#ifndef __IPortableDevicePropertiesBulkCallback_INTERFACE_DEFINED__
#define __IPortableDevicePropertiesBulkCallback_INTERFACE_DEFINED__

/* interface IPortableDevicePropertiesBulkCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDevicePropertiesBulkCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9deacb80-11e8-40e3-a9f3-f557986a7845")
    IPortableDevicePropertiesBulkCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnStart( 
            /* [in] */ __RPC__in REFGUID pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnProgress( 
            /* [in] */ __RPC__in REFGUID pContext,
            /* [in] */ __RPC__in_opt IPortableDeviceValuesCollection *pResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnEnd( 
            /* [in] */ __RPC__in REFGUID pContext,
            /* [in] */ HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDevicePropertiesBulkCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDevicePropertiesBulkCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDevicePropertiesBulkCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDevicePropertiesBulkCallback * This);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulkCallback, OnStart)
        HRESULT ( STDMETHODCALLTYPE *OnStart )( 
            __RPC__in IPortableDevicePropertiesBulkCallback * This,
            /* [in] */ __RPC__in REFGUID pContext);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulkCallback, OnProgress)
        HRESULT ( STDMETHODCALLTYPE *OnProgress )( 
            __RPC__in IPortableDevicePropertiesBulkCallback * This,
            /* [in] */ __RPC__in REFGUID pContext,
            /* [in] */ __RPC__in_opt IPortableDeviceValuesCollection *pResults);
        
        DECLSPEC_XFGVIRT(IPortableDevicePropertiesBulkCallback, OnEnd)
        HRESULT ( STDMETHODCALLTYPE *OnEnd )( 
            __RPC__in IPortableDevicePropertiesBulkCallback * This,
            /* [in] */ __RPC__in REFGUID pContext,
            /* [in] */ HRESULT hrStatus);
        
        END_INTERFACE
    } IPortableDevicePropertiesBulkCallbackVtbl;

    interface IPortableDevicePropertiesBulkCallback
    {
        CONST_VTBL struct IPortableDevicePropertiesBulkCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDevicePropertiesBulkCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDevicePropertiesBulkCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDevicePropertiesBulkCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDevicePropertiesBulkCallback_OnStart(This,pContext)	\
    ( (This)->lpVtbl -> OnStart(This,pContext) ) 

#define IPortableDevicePropertiesBulkCallback_OnProgress(This,pContext,pResults)	\
    ( (This)->lpVtbl -> OnProgress(This,pContext,pResults) ) 

#define IPortableDevicePropertiesBulkCallback_OnEnd(This,pContext,hrStatus)	\
    ( (This)->lpVtbl -> OnEnd(This,pContext,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDevicePropertiesBulkCallback_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceServiceManager_INTERFACE_DEFINED__
#define __IPortableDeviceServiceManager_INTERFACE_DEFINED__

/* interface IPortableDeviceServiceManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceServiceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a8abc4e9-a84a-47a9-80b3-c5d9b172a961")
    IPortableDeviceServiceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceServices( 
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [in] */ __RPC__in REFGUID guidServiceCategory,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *pServices,
            /* [out][in] */ __RPC__inout DWORD *pcServices) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceForService( 
            /* [in] */ __RPC__in LPCWSTR pszPnPServiceID,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPnPDeviceID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceServiceManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceServiceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceServiceManager * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceManager, GetDeviceServices)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceServices )( 
            __RPC__in IPortableDeviceServiceManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPDeviceID,
            /* [in] */ __RPC__in REFGUID guidServiceCategory,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt LPWSTR *pServices,
            /* [out][in] */ __RPC__inout DWORD *pcServices);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceManager, GetDeviceForService)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceForService )( 
            __RPC__in IPortableDeviceServiceManager * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPServiceID,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPnPDeviceID);
        
        END_INTERFACE
    } IPortableDeviceServiceManagerVtbl;

    interface IPortableDeviceServiceManager
    {
        CONST_VTBL struct IPortableDeviceServiceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceServiceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceServiceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceServiceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceServiceManager_GetDeviceServices(This,pszPnPDeviceID,guidServiceCategory,pServices,pcServices)	\
    ( (This)->lpVtbl -> GetDeviceServices(This,pszPnPDeviceID,guidServiceCategory,pServices,pcServices) ) 

#define IPortableDeviceServiceManager_GetDeviceForService(This,pszPnPServiceID,ppszPnPDeviceID)	\
    ( (This)->lpVtbl -> GetDeviceForService(This,pszPnPServiceID,ppszPnPDeviceID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceServiceManager_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceService_INTERFACE_DEFINED__
#define __IPortableDeviceService_INTERFACE_DEFINED__

/* interface IPortableDeviceService */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d3bd3a44-d7b5-40a9-98b7-2fa4d01dec08")
    IPortableDeviceService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Open( 
            /* [in] */ __RPC__in LPCWSTR pszPnPServiceID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pClientInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Capabilities( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceServiceCapabilities **ppCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Content( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceContent2 **ppContent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Methods( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceServiceMethods **ppMethods) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetServiceObjectID( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszServiceObjectID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPnPServiceID( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPnPServiceID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceEventCallback *pCallback,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ __RPC__in LPCWSTR pszCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendCommand( 
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceService * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Open)
        HRESULT ( STDMETHODCALLTYPE *Open )( 
            __RPC__in IPortableDeviceService * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPServiceID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pClientInfo);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Capabilities)
        HRESULT ( STDMETHODCALLTYPE *Capabilities )( 
            __RPC__in IPortableDeviceService * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceServiceCapabilities **ppCapabilities);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Content)
        HRESULT ( STDMETHODCALLTYPE *Content )( 
            __RPC__in IPortableDeviceService * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceContent2 **ppContent);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Methods)
        HRESULT ( STDMETHODCALLTYPE *Methods )( 
            __RPC__in IPortableDeviceService * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceServiceMethods **ppMethods);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceService * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IPortableDeviceService * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, GetServiceObjectID)
        HRESULT ( STDMETHODCALLTYPE *GetServiceObjectID )( 
            __RPC__in IPortableDeviceService * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszServiceObjectID);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, GetPnPServiceID)
        HRESULT ( STDMETHODCALLTYPE *GetPnPServiceID )( 
            __RPC__in IPortableDeviceService * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszPnPServiceID);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            __RPC__in IPortableDeviceService * This,
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceEventCallback *pCallback,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppszCookie);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            __RPC__in IPortableDeviceService * This,
            /* [in] */ __RPC__in LPCWSTR pszCookie);
        
        DECLSPEC_XFGVIRT(IPortableDeviceService, SendCommand)
        HRESULT ( STDMETHODCALLTYPE *SendCommand )( 
            __RPC__in IPortableDeviceService * This,
            /* [in] */ const DWORD dwFlags,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppResults);
        
        END_INTERFACE
    } IPortableDeviceServiceVtbl;

    interface IPortableDeviceService
    {
        CONST_VTBL struct IPortableDeviceServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceService_Open(This,pszPnPServiceID,pClientInfo)	\
    ( (This)->lpVtbl -> Open(This,pszPnPServiceID,pClientInfo) ) 

#define IPortableDeviceService_Capabilities(This,ppCapabilities)	\
    ( (This)->lpVtbl -> Capabilities(This,ppCapabilities) ) 

#define IPortableDeviceService_Content(This,ppContent)	\
    ( (This)->lpVtbl -> Content(This,ppContent) ) 

#define IPortableDeviceService_Methods(This,ppMethods)	\
    ( (This)->lpVtbl -> Methods(This,ppMethods) ) 

#define IPortableDeviceService_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#define IPortableDeviceService_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IPortableDeviceService_GetServiceObjectID(This,ppszServiceObjectID)	\
    ( (This)->lpVtbl -> GetServiceObjectID(This,ppszServiceObjectID) ) 

#define IPortableDeviceService_GetPnPServiceID(This,ppszPnPServiceID)	\
    ( (This)->lpVtbl -> GetPnPServiceID(This,ppszPnPServiceID) ) 

#define IPortableDeviceService_Advise(This,dwFlags,pCallback,pParameters,ppszCookie)	\
    ( (This)->lpVtbl -> Advise(This,dwFlags,pCallback,pParameters,ppszCookie) ) 

#define IPortableDeviceService_Unadvise(This,pszCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,pszCookie) ) 

#define IPortableDeviceService_SendCommand(This,dwFlags,pParameters,ppResults)	\
    ( (This)->lpVtbl -> SendCommand(This,dwFlags,pParameters,ppResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceService_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceServiceCapabilities_INTERFACE_DEFINED__
#define __IPortableDeviceServiceCapabilities_INTERFACE_DEFINED__

/* interface IPortableDeviceServiceCapabilities */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceServiceCapabilities;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24dbd89d-413e-43e0-bd5b-197f3c56c886")
    IPortableDeviceServiceCapabilities : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSupportedMethods( 
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppMethods) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedMethodsByFormat( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppMethods) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodAttributes( 
            /* [in] */ __RPC__in REFGUID Method,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMethodParameterAttributes( 
            /* [in] */ __RPC__in REFGUID Method,
            /* [in] */ __RPC__in REFPROPERTYKEY Parameter,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedFormats( 
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatAttributes( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedFormatProperties( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatPropertyAttributes( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [in] */ __RPC__in REFPROPERTYKEY Property,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedEvents( 
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventAttributes( 
            /* [in] */ __RPC__in REFGUID Event,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEventParameterAttributes( 
            /* [in] */ __RPC__in REFGUID Event,
            /* [in] */ __RPC__in REFPROPERTYKEY Parameter,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInheritedServices( 
            /* [in] */ const DWORD dwInheritanceType,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppServices) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatRenderingProfiles( 
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValuesCollection **ppRenderingProfiles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedCommands( 
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppCommands) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCommandOptions( 
            /* [in] */ __RPC__in REFPROPERTYKEY Command,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppOptions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceCapabilitiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceServiceCapabilities * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceServiceCapabilities * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetSupportedMethods)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedMethods )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppMethods);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetSupportedMethodsByFormat)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedMethodsByFormat )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppMethods);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetMethodAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetMethodAttributes )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Method,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetMethodParameterAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetMethodParameterAttributes )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Method,
            /* [in] */ __RPC__in REFPROPERTYKEY Parameter,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetSupportedFormats)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedFormats )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppFormats);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetFormatAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetFormatAttributes )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetSupportedFormatProperties)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedFormatProperties )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppKeys);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetFormatPropertyAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetFormatPropertyAttributes )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [in] */ __RPC__in REFPROPERTYKEY Property,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetSupportedEvents)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedEvents )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppEvents);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetEventAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetEventAttributes )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Event,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetEventParameterAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetEventParameterAttributes )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Event,
            /* [in] */ __RPC__in REFPROPERTYKEY Parameter,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppAttributes);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetInheritedServices)
        HRESULT ( STDMETHODCALLTYPE *GetInheritedServices )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ const DWORD dwInheritanceType,
            /* [out] */ __RPC__deref_out_opt IPortableDevicePropVariantCollection **ppServices);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetFormatRenderingProfiles)
        HRESULT ( STDMETHODCALLTYPE *GetFormatRenderingProfiles )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFGUID Format,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValuesCollection **ppRenderingProfiles);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetSupportedCommands)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedCommands )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceKeyCollection **ppCommands);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, GetCommandOptions)
        HRESULT ( STDMETHODCALLTYPE *GetCommandOptions )( 
            __RPC__in IPortableDeviceServiceCapabilities * This,
            /* [in] */ __RPC__in REFPROPERTYKEY Command,
            /* [out] */ __RPC__deref_out_opt IPortableDeviceValues **ppOptions);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceCapabilities, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceServiceCapabilities * This);
        
        END_INTERFACE
    } IPortableDeviceServiceCapabilitiesVtbl;

    interface IPortableDeviceServiceCapabilities
    {
        CONST_VTBL struct IPortableDeviceServiceCapabilitiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceServiceCapabilities_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceServiceCapabilities_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceServiceCapabilities_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceServiceCapabilities_GetSupportedMethods(This,ppMethods)	\
    ( (This)->lpVtbl -> GetSupportedMethods(This,ppMethods) ) 

#define IPortableDeviceServiceCapabilities_GetSupportedMethodsByFormat(This,Format,ppMethods)	\
    ( (This)->lpVtbl -> GetSupportedMethodsByFormat(This,Format,ppMethods) ) 

#define IPortableDeviceServiceCapabilities_GetMethodAttributes(This,Method,ppAttributes)	\
    ( (This)->lpVtbl -> GetMethodAttributes(This,Method,ppAttributes) ) 

#define IPortableDeviceServiceCapabilities_GetMethodParameterAttributes(This,Method,Parameter,ppAttributes)	\
    ( (This)->lpVtbl -> GetMethodParameterAttributes(This,Method,Parameter,ppAttributes) ) 

#define IPortableDeviceServiceCapabilities_GetSupportedFormats(This,ppFormats)	\
    ( (This)->lpVtbl -> GetSupportedFormats(This,ppFormats) ) 

#define IPortableDeviceServiceCapabilities_GetFormatAttributes(This,Format,ppAttributes)	\
    ( (This)->lpVtbl -> GetFormatAttributes(This,Format,ppAttributes) ) 

#define IPortableDeviceServiceCapabilities_GetSupportedFormatProperties(This,Format,ppKeys)	\
    ( (This)->lpVtbl -> GetSupportedFormatProperties(This,Format,ppKeys) ) 

#define IPortableDeviceServiceCapabilities_GetFormatPropertyAttributes(This,Format,Property,ppAttributes)	\
    ( (This)->lpVtbl -> GetFormatPropertyAttributes(This,Format,Property,ppAttributes) ) 

#define IPortableDeviceServiceCapabilities_GetSupportedEvents(This,ppEvents)	\
    ( (This)->lpVtbl -> GetSupportedEvents(This,ppEvents) ) 

#define IPortableDeviceServiceCapabilities_GetEventAttributes(This,Event,ppAttributes)	\
    ( (This)->lpVtbl -> GetEventAttributes(This,Event,ppAttributes) ) 

#define IPortableDeviceServiceCapabilities_GetEventParameterAttributes(This,Event,Parameter,ppAttributes)	\
    ( (This)->lpVtbl -> GetEventParameterAttributes(This,Event,Parameter,ppAttributes) ) 

#define IPortableDeviceServiceCapabilities_GetInheritedServices(This,dwInheritanceType,ppServices)	\
    ( (This)->lpVtbl -> GetInheritedServices(This,dwInheritanceType,ppServices) ) 

#define IPortableDeviceServiceCapabilities_GetFormatRenderingProfiles(This,Format,ppRenderingProfiles)	\
    ( (This)->lpVtbl -> GetFormatRenderingProfiles(This,Format,ppRenderingProfiles) ) 

#define IPortableDeviceServiceCapabilities_GetSupportedCommands(This,ppCommands)	\
    ( (This)->lpVtbl -> GetSupportedCommands(This,ppCommands) ) 

#define IPortableDeviceServiceCapabilities_GetCommandOptions(This,Command,ppOptions)	\
    ( (This)->lpVtbl -> GetCommandOptions(This,Command,ppOptions) ) 

#define IPortableDeviceServiceCapabilities_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceServiceCapabilities_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceServiceMethods_INTERFACE_DEFINED__
#define __IPortableDeviceServiceMethods_INTERFACE_DEFINED__

/* interface IPortableDeviceServiceMethods */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceServiceMethods;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e20333c9-fd34-412d-a381-cc6f2d820df7")
    IPortableDeviceServiceMethods : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invoke( 
            /* [in] */ __RPC__in REFGUID Method,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDeviceValues **ppResults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InvokeAsync( 
            /* [in] */ __RPC__in REFGUID Method,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceServiceMethodCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( 
            /* [unique][in] */ __RPC__in_opt IPortableDeviceServiceMethodCallback *pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceMethodsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceServiceMethods * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceServiceMethods * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceServiceMethods * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceMethods, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in IPortableDeviceServiceMethods * This,
            /* [in] */ __RPC__in REFGUID Method,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [unique][out][in] */ __RPC__deref_opt_inout_opt IPortableDeviceValues **ppResults);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceMethods, InvokeAsync)
        HRESULT ( STDMETHODCALLTYPE *InvokeAsync )( 
            __RPC__in IPortableDeviceServiceMethods * This,
            /* [in] */ __RPC__in REFGUID Method,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceValues *pParameters,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceServiceMethodCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceMethods, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IPortableDeviceServiceMethods * This,
            /* [unique][in] */ __RPC__in_opt IPortableDeviceServiceMethodCallback *pCallback);
        
        END_INTERFACE
    } IPortableDeviceServiceMethodsVtbl;

    interface IPortableDeviceServiceMethods
    {
        CONST_VTBL struct IPortableDeviceServiceMethodsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceServiceMethods_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceServiceMethods_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceServiceMethods_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceServiceMethods_Invoke(This,Method,pParameters,ppResults)	\
    ( (This)->lpVtbl -> Invoke(This,Method,pParameters,ppResults) ) 

#define IPortableDeviceServiceMethods_InvokeAsync(This,Method,pParameters,pCallback)	\
    ( (This)->lpVtbl -> InvokeAsync(This,Method,pParameters,pCallback) ) 

#define IPortableDeviceServiceMethods_Cancel(This,pCallback)	\
    ( (This)->lpVtbl -> Cancel(This,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceServiceMethods_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceServiceMethodCallback_INTERFACE_DEFINED__
#define __IPortableDeviceServiceMethodCallback_INTERFACE_DEFINED__

/* interface IPortableDeviceServiceMethodCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceServiceMethodCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c424233c-afce-4828-a756-7ed7a2350083")
    IPortableDeviceServiceMethodCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnComplete( 
            /* [in] */ HRESULT hrStatus,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pResults) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceMethodCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceServiceMethodCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceServiceMethodCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceServiceMethodCallback * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceMethodCallback, OnComplete)
        HRESULT ( STDMETHODCALLTYPE *OnComplete )( 
            __RPC__in IPortableDeviceServiceMethodCallback * This,
            /* [in] */ HRESULT hrStatus,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pResults);
        
        END_INTERFACE
    } IPortableDeviceServiceMethodCallbackVtbl;

    interface IPortableDeviceServiceMethodCallback
    {
        CONST_VTBL struct IPortableDeviceServiceMethodCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceServiceMethodCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceServiceMethodCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceServiceMethodCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceServiceMethodCallback_OnComplete(This,hrStatus,pResults)	\
    ( (This)->lpVtbl -> OnComplete(This,hrStatus,pResults) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceServiceMethodCallback_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceServiceActivation_INTERFACE_DEFINED__
#define __IPortableDeviceServiceActivation_INTERFACE_DEFINED__

/* interface IPortableDeviceServiceActivation */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceServiceActivation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e56b0534-d9b9-425c-9b99-75f97cb3d7c8")
    IPortableDeviceServiceActivation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OpenAsync( 
            /* [in] */ __RPC__in LPCWSTR pszPnPServiceID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pClientInfo,
            /* [in] */ __RPC__in_opt IPortableDeviceServiceOpenCallback *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelOpenAsync( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceActivationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceServiceActivation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceServiceActivation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceServiceActivation * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceActivation, OpenAsync)
        HRESULT ( STDMETHODCALLTYPE *OpenAsync )( 
            __RPC__in IPortableDeviceServiceActivation * This,
            /* [in] */ __RPC__in LPCWSTR pszPnPServiceID,
            /* [in] */ __RPC__in_opt IPortableDeviceValues *pClientInfo,
            /* [in] */ __RPC__in_opt IPortableDeviceServiceOpenCallback *pCallback);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceActivation, CancelOpenAsync)
        HRESULT ( STDMETHODCALLTYPE *CancelOpenAsync )( 
            __RPC__in IPortableDeviceServiceActivation * This);
        
        END_INTERFACE
    } IPortableDeviceServiceActivationVtbl;

    interface IPortableDeviceServiceActivation
    {
        CONST_VTBL struct IPortableDeviceServiceActivationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceServiceActivation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceServiceActivation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceServiceActivation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceServiceActivation_OpenAsync(This,pszPnPServiceID,pClientInfo,pCallback)	\
    ( (This)->lpVtbl -> OpenAsync(This,pszPnPServiceID,pClientInfo,pCallback) ) 

#define IPortableDeviceServiceActivation_CancelOpenAsync(This)	\
    ( (This)->lpVtbl -> CancelOpenAsync(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceServiceActivation_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceServiceOpenCallback_INTERFACE_DEFINED__
#define __IPortableDeviceServiceOpenCallback_INTERFACE_DEFINED__

/* interface IPortableDeviceServiceOpenCallback */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IPortableDeviceServiceOpenCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bced49c8-8efe-41ed-960b-61313abd47a9")
    IPortableDeviceServiceOpenCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnComplete( 
            /* [in] */ HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceServiceOpenCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPortableDeviceServiceOpenCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPortableDeviceServiceOpenCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPortableDeviceServiceOpenCallback * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceServiceOpenCallback, OnComplete)
        HRESULT ( STDMETHODCALLTYPE *OnComplete )( 
            __RPC__in IPortableDeviceServiceOpenCallback * This,
            /* [in] */ HRESULT hrStatus);
        
        END_INTERFACE
    } IPortableDeviceServiceOpenCallbackVtbl;

    interface IPortableDeviceServiceOpenCallback
    {
        CONST_VTBL struct IPortableDeviceServiceOpenCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceServiceOpenCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceServiceOpenCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceServiceOpenCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceServiceOpenCallback_OnComplete(This,hrStatus)	\
    ( (This)->lpVtbl -> OnComplete(This,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceServiceOpenCallback_INTERFACE_DEFINED__ */


#ifndef __IPortableDeviceDispatchFactory_INTERFACE_DEFINED__
#define __IPortableDeviceDispatchFactory_INTERFACE_DEFINED__

/* interface IPortableDeviceDispatchFactory */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IPortableDeviceDispatchFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5e1eafc3-e3d7-4132-96fa-759c0f9d1e0f")
    IPortableDeviceDispatchFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceDispatch( 
            /* [in] */ LPCWSTR pszPnPDeviceID,
            /* [out] */ IDispatch **ppDeviceDispatch) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceDispatchFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPortableDeviceDispatchFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPortableDeviceDispatchFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPortableDeviceDispatchFactory * This);
        
        DECLSPEC_XFGVIRT(IPortableDeviceDispatchFactory, GetDeviceDispatch)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceDispatch )( 
            IPortableDeviceDispatchFactory * This,
            /* [in] */ LPCWSTR pszPnPDeviceID,
            /* [out] */ IDispatch **ppDeviceDispatch);
        
        END_INTERFACE
    } IPortableDeviceDispatchFactoryVtbl;

    interface IPortableDeviceDispatchFactory
    {
        CONST_VTBL struct IPortableDeviceDispatchFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceDispatchFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceDispatchFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceDispatchFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceDispatchFactory_GetDeviceDispatch(This,pszPnPDeviceID,ppDeviceDispatch)	\
    ( (This)->lpVtbl -> GetDeviceDispatch(This,pszPnPDeviceID,ppDeviceDispatch) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceDispatchFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_PortableDeviceApi_0000_0021 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0021_v0_0_s_ifspec;

#ifndef __IPortableDeviceWebControl_INTERFACE_DEFINED__
#define __IPortableDeviceWebControl_INTERFACE_DEFINED__

/* interface IPortableDeviceWebControl */
/* [uuid][nonextensible][local][dual][object] */ 


EXTERN_C const IID IID_IPortableDeviceWebControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94fc7953-5ca1-483a-8aee-df52e7747d00")
    IPortableDeviceWebControl : public IDispatch
    {
    public:
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDeviceFromId( 
            /* [annotation][in] */ 
            _In_  BSTR deviceId,
            /* [annotation][retval][out] */ 
            _COM_Outptr_  IDispatch **ppDevice) = 0;
        
        virtual /* [helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDeviceFromIdAsync( 
            /* [annotation][in] */ 
            _In_  BSTR deviceId,
            /* [annotation][in] */ 
            _In_  IDispatch *pCompletionHandler,
            /* [annotation][unique][in] */ 
            _In_opt_  IDispatch *pErrorHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPortableDeviceWebControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPortableDeviceWebControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPortableDeviceWebControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPortableDeviceWebControl * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            IPortableDeviceWebControl * This,
            /* [out] */ UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            IPortableDeviceWebControl * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            IPortableDeviceWebControl * This,
            /* [in] */ REFIID riid,
            /* [size_is][in] */ LPOLESTR *rgszNames,
            /* [range][in] */ UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IPortableDeviceWebControl * This,
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
        
        DECLSPEC_XFGVIRT(IPortableDeviceWebControl, GetDeviceFromId)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceFromId )( 
            IPortableDeviceWebControl * This,
            /* [annotation][in] */ 
            _In_  BSTR deviceId,
            /* [annotation][retval][out] */ 
            _COM_Outptr_  IDispatch **ppDevice);
        
        DECLSPEC_XFGVIRT(IPortableDeviceWebControl, GetDeviceFromIdAsync)
        /* [helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDeviceFromIdAsync )( 
            IPortableDeviceWebControl * This,
            /* [annotation][in] */ 
            _In_  BSTR deviceId,
            /* [annotation][in] */ 
            _In_  IDispatch *pCompletionHandler,
            /* [annotation][unique][in] */ 
            _In_opt_  IDispatch *pErrorHandler);
        
        END_INTERFACE
    } IPortableDeviceWebControlVtbl;

    interface IPortableDeviceWebControl
    {
        CONST_VTBL struct IPortableDeviceWebControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPortableDeviceWebControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPortableDeviceWebControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPortableDeviceWebControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPortableDeviceWebControl_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IPortableDeviceWebControl_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IPortableDeviceWebControl_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IPortableDeviceWebControl_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IPortableDeviceWebControl_GetDeviceFromId(This,deviceId,ppDevice)	\
    ( (This)->lpVtbl -> GetDeviceFromId(This,deviceId,ppDevice) ) 

#define IPortableDeviceWebControl_GetDeviceFromIdAsync(This,deviceId,pCompletionHandler,pErrorHandler)	\
    ( (This)->lpVtbl -> GetDeviceFromIdAsync(This,deviceId,pCompletionHandler,pErrorHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPortableDeviceWebControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_PortableDeviceApi_0000_0022 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0022_v0_0_s_ifspec;


#ifndef __PortableDeviceApiLib_LIBRARY_DEFINED__
#define __PortableDeviceApiLib_LIBRARY_DEFINED__

/* library PortableDeviceApiLib */
/* [helpstring][version][uuid] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

EXTERN_C const IID LIBID_PortableDeviceApiLib;

EXTERN_C const CLSID CLSID_PortableDevice;

#ifdef __cplusplus

class DECLSPEC_UUID("728a21c5-3d9e-48d7-9810-864848f0f404")
PortableDevice;
#endif

EXTERN_C const CLSID CLSID_PortableDeviceManager;

#ifdef __cplusplus

class DECLSPEC_UUID("0af10cec-2ecd-4b92-9581-34f6ae0637f3")
PortableDeviceManager;
#endif

EXTERN_C const CLSID CLSID_PortableDeviceService;

#ifdef __cplusplus

class DECLSPEC_UUID("ef5db4c2-9312-422c-9152-411cd9c4dd84")
PortableDeviceService;
#endif

EXTERN_C const CLSID CLSID_PortableDeviceDispatchFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("43232233-8338-4658-ae01-0b4ae830b6b0")
PortableDeviceDispatchFactory;
#endif

EXTERN_C const CLSID CLSID_PortableDeviceFTM;

#ifdef __cplusplus

class DECLSPEC_UUID("f7c0039a-4762-488a-b4b3-760ef9a1ba9b")
PortableDeviceFTM;
#endif

EXTERN_C const CLSID CLSID_PortableDeviceServiceFTM;

#ifdef __cplusplus

class DECLSPEC_UUID("1649b154-c794-497a-9b03-f3f0121302f3")
PortableDeviceServiceFTM;
#endif

EXTERN_C const CLSID CLSID_PortableDeviceWebControl;

#ifdef __cplusplus

class DECLSPEC_UUID("186dd02c-2dec-41b5-a7d4-b59056fade51")
PortableDeviceWebControl;
#endif
#endif /* __PortableDeviceApiLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_PortableDeviceApi_0000_0023 */
/* [local] */ 

#endif  // (_WIN32_WINNT >= 0x0501)


extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_PortableDeviceApi_0000_0023_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* [local] */ HRESULT STDMETHODCALLTYPE IPortableDeviceUnitsStream_SeekInUnits_Proxy( 
    IPortableDeviceUnitsStream * This,
    /* [in] */ LARGE_INTEGER dlibMove,
    /* [in] */ WPD_STREAM_UNITS units,
    /* [in] */ DWORD dwOrigin,
    /* [annotation] */ 
    _Out_opt_  ULARGE_INTEGER *plibNewPosition);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IPortableDeviceUnitsStream_SeekInUnits_Stub( 
    __RPC__in IPortableDeviceUnitsStream * This,
    /* [in] */ LARGE_INTEGER dlibMove,
    /* [in] */ WPD_STREAM_UNITS units,
    /* [in] */ DWORD dwOrigin,
    /* [out] */ __RPC__out ULARGE_INTEGER *plibNewPosition);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


