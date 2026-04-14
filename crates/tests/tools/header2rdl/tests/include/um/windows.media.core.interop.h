

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

#ifndef __windows2Emedia2Ecore2Einterop_h__
#define __windows2Emedia2Ecore2Einterop_h__

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

#ifndef __IAudioFrameNative_FWD_DEFINED__
#define __IAudioFrameNative_FWD_DEFINED__
typedef interface IAudioFrameNative IAudioFrameNative;

#endif 	/* __IAudioFrameNative_FWD_DEFINED__ */


#ifndef __IVideoFrameNative_FWD_DEFINED__
#define __IVideoFrameNative_FWD_DEFINED__
typedef interface IVideoFrameNative IVideoFrameNative;

#endif 	/* __IVideoFrameNative_FWD_DEFINED__ */


#ifndef __IAudioFrameNativeFactory_FWD_DEFINED__
#define __IAudioFrameNativeFactory_FWD_DEFINED__
typedef interface IAudioFrameNativeFactory IAudioFrameNativeFactory;

#endif 	/* __IAudioFrameNativeFactory_FWD_DEFINED__ */


#ifndef __IVideoFrameNativeFactory_FWD_DEFINED__
#define __IVideoFrameNativeFactory_FWD_DEFINED__
typedef interface IVideoFrameNativeFactory IVideoFrameNativeFactory;

#endif 	/* __IVideoFrameNativeFactory_FWD_DEFINED__ */


/* header files for imported files */
#include "Inspectable.h"
#include "mfobjects.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Emedia2Ecore2Einterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
// {16A0A3B9-9F65-4102-9367-2CDA3A4F372A}
extern const __declspec(selectany) CLSID CLSID_AudioFrameNativeFactory = { 0x16a0a3b9, 0x9f65, 0x4102, { 0x93, 0x67, 0x2c, 0xda, 0x3a, 0x4f, 0x37, 0x2a } };
// {D194386A-04E3-4814-8100-B2B0AE6D78C7}
extern const __declspec(selectany) CLSID CLSID_VideoFrameNativeFactory = { 0xd194386a, 0x4e3, 0x4814, { 0x81, 0x0, 0xb2, 0xb0, 0xae, 0x6d, 0x78, 0xc7 } };


extern RPC_IF_HANDLE __MIDL_itf_windows2Emedia2Ecore2Einterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Emedia2Ecore2Einterop_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioFrameNative_INTERFACE_DEFINED__
#define __IAudioFrameNative_INTERFACE_DEFINED__

/* interface IAudioFrameNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IAudioFrameNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20BE1E2E-930F-4746-9335-3C332F255093")
    IAudioFrameNative : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioFrameNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioFrameNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioFrameNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioFrameNative * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IAudioFrameNative * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IAudioFrameNative * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IAudioFrameNative * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IAudioFrameNative, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IAudioFrameNative * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        END_INTERFACE
    } IAudioFrameNativeVtbl;

    interface IAudioFrameNative
    {
        CONST_VTBL struct IAudioFrameNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioFrameNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioFrameNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioFrameNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioFrameNative_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IAudioFrameNative_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IAudioFrameNative_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IAudioFrameNative_GetData(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetData(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioFrameNative_INTERFACE_DEFINED__ */


#ifndef __IVideoFrameNative_INTERFACE_DEFINED__
#define __IVideoFrameNative_INTERFACE_DEFINED__

/* interface IVideoFrameNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IVideoFrameNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26BA702B-314A-4620-AAF6-7A51AA58FA18")
    IVideoFrameNative : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDevice( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVideoFrameNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVideoFrameNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVideoFrameNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVideoFrameNative * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IVideoFrameNative * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IVideoFrameNative * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IVideoFrameNative * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IVideoFrameNative, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IVideoFrameNative * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IVideoFrameNative, GetDevice)
        HRESULT ( STDMETHODCALLTYPE *GetDevice )( 
            IVideoFrameNative * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        END_INTERFACE
    } IVideoFrameNativeVtbl;

    interface IVideoFrameNative
    {
        CONST_VTBL struct IVideoFrameNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVideoFrameNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVideoFrameNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVideoFrameNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVideoFrameNative_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IVideoFrameNative_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IVideoFrameNative_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IVideoFrameNative_GetData(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetData(This,riid,ppv) ) 

#define IVideoFrameNative_GetDevice(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetDevice(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVideoFrameNative_INTERFACE_DEFINED__ */


#ifndef __IAudioFrameNativeFactory_INTERFACE_DEFINED__
#define __IAudioFrameNativeFactory_INTERFACE_DEFINED__

/* interface IAudioFrameNativeFactory */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IAudioFrameNativeFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7BD67CF8-BF7D-43E6-AF8D-B170EE0C0110")
    IAudioFrameNativeFactory : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateFromMFSample( 
            /* [annotation][in] */ 
            _In_  IMFSample *data,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioFrameNativeFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioFrameNativeFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioFrameNativeFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioFrameNativeFactory * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IAudioFrameNativeFactory * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IAudioFrameNativeFactory * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IAudioFrameNativeFactory * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IAudioFrameNativeFactory, CreateFromMFSample)
        HRESULT ( STDMETHODCALLTYPE *CreateFromMFSample )( 
            IAudioFrameNativeFactory * This,
            /* [annotation][in] */ 
            _In_  IMFSample *data,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        END_INTERFACE
    } IAudioFrameNativeFactoryVtbl;

    interface IAudioFrameNativeFactory
    {
        CONST_VTBL struct IAudioFrameNativeFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioFrameNativeFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioFrameNativeFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioFrameNativeFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioFrameNativeFactory_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IAudioFrameNativeFactory_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IAudioFrameNativeFactory_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IAudioFrameNativeFactory_CreateFromMFSample(This,data,forceReadOnly,riid,ppv)	\
    ( (This)->lpVtbl -> CreateFromMFSample(This,data,forceReadOnly,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioFrameNativeFactory_INTERFACE_DEFINED__ */


#ifndef __IVideoFrameNativeFactory_INTERFACE_DEFINED__
#define __IVideoFrameNativeFactory_INTERFACE_DEFINED__

/* interface IVideoFrameNativeFactory */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IVideoFrameNativeFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("69E3693E-8E1E-4E63-AC4C-7FDC21D9731D")
    IVideoFrameNativeFactory : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateFromMFSample( 
            /* [annotation][in] */ 
            _In_  IMFSample *data,
            /* [annotation][in] */ 
            _In_  REFGUID subtype,
            /* [annotation][in] */ 
            _In_  UINT32 width,
            /* [annotation][in] */ 
            _In_  UINT32 height,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoArea *minDisplayAperture,
            /* [annotation][in] */ 
            _In_opt_  IMFDXGIDeviceManager *device,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVideoFrameNativeFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVideoFrameNativeFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVideoFrameNativeFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVideoFrameNativeFactory * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IVideoFrameNativeFactory * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IVideoFrameNativeFactory * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IVideoFrameNativeFactory * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IVideoFrameNativeFactory, CreateFromMFSample)
        HRESULT ( STDMETHODCALLTYPE *CreateFromMFSample )( 
            IVideoFrameNativeFactory * This,
            /* [annotation][in] */ 
            _In_  IMFSample *data,
            /* [annotation][in] */ 
            _In_  REFGUID subtype,
            /* [annotation][in] */ 
            _In_  UINT32 width,
            /* [annotation][in] */ 
            _In_  UINT32 height,
            /* [annotation][in] */ 
            _In_  BOOL forceReadOnly,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoArea *minDisplayAperture,
            /* [annotation][in] */ 
            _In_opt_  IMFDXGIDeviceManager *device,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  LPVOID *ppv);
        
        END_INTERFACE
    } IVideoFrameNativeFactoryVtbl;

    interface IVideoFrameNativeFactory
    {
        CONST_VTBL struct IVideoFrameNativeFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVideoFrameNativeFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVideoFrameNativeFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVideoFrameNativeFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVideoFrameNativeFactory_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IVideoFrameNativeFactory_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IVideoFrameNativeFactory_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IVideoFrameNativeFactory_CreateFromMFSample(This,data,subtype,width,height,forceReadOnly,minDisplayAperture,device,riid,ppv)	\
    ( (This)->lpVtbl -> CreateFromMFSample(This,data,subtype,width,height,forceReadOnly,minDisplayAperture,device,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVideoFrameNativeFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Emedia2Ecore2Einterop_0000_0004 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_windows2Emedia2Ecore2Einterop_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Emedia2Ecore2Einterop_0000_0004_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


