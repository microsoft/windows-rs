

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

#ifndef __mfmediacapture_h__
#define __mfmediacapture_h__

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

#ifndef __IAdvancedMediaCaptureInitializationSettings_FWD_DEFINED__
#define __IAdvancedMediaCaptureInitializationSettings_FWD_DEFINED__
typedef interface IAdvancedMediaCaptureInitializationSettings IAdvancedMediaCaptureInitializationSettings;

#endif 	/* __IAdvancedMediaCaptureInitializationSettings_FWD_DEFINED__ */


#ifndef __IAdvancedMediaCaptureSettings_FWD_DEFINED__
#define __IAdvancedMediaCaptureSettings_FWD_DEFINED__
typedef interface IAdvancedMediaCaptureSettings IAdvancedMediaCaptureSettings;

#endif 	/* __IAdvancedMediaCaptureSettings_FWD_DEFINED__ */


#ifndef __IAdvancedMediaCapture_FWD_DEFINED__
#define __IAdvancedMediaCapture_FWD_DEFINED__
typedef interface IAdvancedMediaCapture IAdvancedMediaCapture;

#endif 	/* __IAdvancedMediaCapture_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfmediacapture_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN8)
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfmediacapture_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediacapture_0000_0000_v0_0_s_ifspec;

#ifndef __IAdvancedMediaCaptureInitializationSettings_INTERFACE_DEFINED__
#define __IAdvancedMediaCaptureInitializationSettings_INTERFACE_DEFINED__

/* interface IAdvancedMediaCaptureInitializationSettings */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IAdvancedMediaCaptureInitializationSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3DE21209-8BA6-4f2a-A577-2819B56FF14D")
    IAdvancedMediaCaptureInitializationSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDirectxDeviceManager( 
            /* [in] */ IMFDXGIDeviceManager *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAdvancedMediaCaptureInitializationSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAdvancedMediaCaptureInitializationSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAdvancedMediaCaptureInitializationSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAdvancedMediaCaptureInitializationSettings * This);
        
        DECLSPEC_XFGVIRT(IAdvancedMediaCaptureInitializationSettings, SetDirectxDeviceManager)
        HRESULT ( STDMETHODCALLTYPE *SetDirectxDeviceManager )( 
            IAdvancedMediaCaptureInitializationSettings * This,
            /* [in] */ IMFDXGIDeviceManager *value);
        
        END_INTERFACE
    } IAdvancedMediaCaptureInitializationSettingsVtbl;

    interface IAdvancedMediaCaptureInitializationSettings
    {
        CONST_VTBL struct IAdvancedMediaCaptureInitializationSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAdvancedMediaCaptureInitializationSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAdvancedMediaCaptureInitializationSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAdvancedMediaCaptureInitializationSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAdvancedMediaCaptureInitializationSettings_SetDirectxDeviceManager(This,value)	\
    ( (This)->lpVtbl -> SetDirectxDeviceManager(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAdvancedMediaCaptureInitializationSettings_INTERFACE_DEFINED__ */


#ifndef __IAdvancedMediaCaptureSettings_INTERFACE_DEFINED__
#define __IAdvancedMediaCaptureSettings_INTERFACE_DEFINED__

/* interface IAdvancedMediaCaptureSettings */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IAdvancedMediaCaptureSettings;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24E0485F-A33E-4aa1-B564-6019B1D14F65")
    IAdvancedMediaCaptureSettings : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDirectxDeviceManager( 
            /* [out] */ IMFDXGIDeviceManager **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAdvancedMediaCaptureSettingsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAdvancedMediaCaptureSettings * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAdvancedMediaCaptureSettings * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAdvancedMediaCaptureSettings * This);
        
        DECLSPEC_XFGVIRT(IAdvancedMediaCaptureSettings, GetDirectxDeviceManager)
        HRESULT ( STDMETHODCALLTYPE *GetDirectxDeviceManager )( 
            IAdvancedMediaCaptureSettings * This,
            /* [out] */ IMFDXGIDeviceManager **value);
        
        END_INTERFACE
    } IAdvancedMediaCaptureSettingsVtbl;

    interface IAdvancedMediaCaptureSettings
    {
        CONST_VTBL struct IAdvancedMediaCaptureSettingsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAdvancedMediaCaptureSettings_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAdvancedMediaCaptureSettings_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAdvancedMediaCaptureSettings_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAdvancedMediaCaptureSettings_GetDirectxDeviceManager(This,value)	\
    ( (This)->lpVtbl -> GetDirectxDeviceManager(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAdvancedMediaCaptureSettings_INTERFACE_DEFINED__ */


#ifndef __IAdvancedMediaCapture_INTERFACE_DEFINED__
#define __IAdvancedMediaCapture_INTERFACE_DEFINED__

/* interface IAdvancedMediaCapture */
/* [uuid][object] */ 


EXTERN_C const IID IID_IAdvancedMediaCapture;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D0751585-D216-4344-B5BF-463B68F977BB")
    IAdvancedMediaCapture : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAdvancedMediaCaptureSettings( 
            /* [out] */ __RPC__deref_out_opt IAdvancedMediaCaptureSettings **value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAdvancedMediaCaptureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAdvancedMediaCapture * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAdvancedMediaCapture * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAdvancedMediaCapture * This);
        
        DECLSPEC_XFGVIRT(IAdvancedMediaCapture, GetAdvancedMediaCaptureSettings)
        HRESULT ( STDMETHODCALLTYPE *GetAdvancedMediaCaptureSettings )( 
            __RPC__in IAdvancedMediaCapture * This,
            /* [out] */ __RPC__deref_out_opt IAdvancedMediaCaptureSettings **value);
        
        END_INTERFACE
    } IAdvancedMediaCaptureVtbl;

    interface IAdvancedMediaCapture
    {
        CONST_VTBL struct IAdvancedMediaCaptureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAdvancedMediaCapture_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAdvancedMediaCapture_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAdvancedMediaCapture_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAdvancedMediaCapture_GetAdvancedMediaCaptureSettings(This,value)	\
    ( (This)->lpVtbl -> GetAdvancedMediaCaptureSettings(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAdvancedMediaCapture_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediacapture_0000_0003 */
/* [local] */ 

#endif // (WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) 
#endif // (NTDDI >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_mfmediacapture_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediacapture_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


