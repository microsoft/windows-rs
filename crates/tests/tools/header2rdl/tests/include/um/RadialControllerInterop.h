

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

#ifndef __radialcontrollerinterop_h__
#define __radialcontrollerinterop_h__

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

#ifndef __IRadialControllerInterop_FWD_DEFINED__
#define __IRadialControllerInterop_FWD_DEFINED__
typedef interface IRadialControllerInterop IRadialControllerInterop;

#endif 	/* __IRadialControllerInterop_FWD_DEFINED__ */


#ifndef __IRadialControllerConfigurationInterop_FWD_DEFINED__
#define __IRadialControllerConfigurationInterop_FWD_DEFINED__
typedef interface IRadialControllerConfigurationInterop IRadialControllerConfigurationInterop;

#endif 	/* __IRadialControllerConfigurationInterop_FWD_DEFINED__ */


#ifndef __IRadialControllerIndependentInputSourceInterop_FWD_DEFINED__
#define __IRadialControllerIndependentInputSourceInterop_FWD_DEFINED__
typedef interface IRadialControllerIndependentInputSourceInterop IRadialControllerIndependentInputSourceInterop;

#endif 	/* __IRadialControllerIndependentInputSourceInterop_FWD_DEFINED__ */


/* header files for imported files */
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_radialcontrollerinterop_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_radialcontrollerinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_radialcontrollerinterop_0000_0000_v0_0_s_ifspec;

#ifndef __IRadialControllerInterop_INTERFACE_DEFINED__
#define __IRadialControllerInterop_INTERFACE_DEFINED__

/* interface IRadialControllerInterop */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IRadialControllerInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1B0535C9-57AD-45C1-9D79-AD5C34360513")
    IRadialControllerInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateForWindow( 
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRadialControllerInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRadialControllerInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRadialControllerInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRadialControllerInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IRadialControllerInterop * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IRadialControllerInterop * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IRadialControllerInterop * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IRadialControllerInterop, CreateForWindow)
        HRESULT ( STDMETHODCALLTYPE *CreateForWindow )( 
            IRadialControllerInterop * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppv);
        
        END_INTERFACE
    } IRadialControllerInteropVtbl;

    interface IRadialControllerInterop
    {
        CONST_VTBL struct IRadialControllerInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRadialControllerInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRadialControllerInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRadialControllerInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRadialControllerInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IRadialControllerInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IRadialControllerInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IRadialControllerInterop_CreateForWindow(This,hwnd,riid,ppv)	\
    ( (This)->lpVtbl -> CreateForWindow(This,hwnd,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRadialControllerInterop_INTERFACE_DEFINED__ */


#ifndef __IRadialControllerConfigurationInterop_INTERFACE_DEFINED__
#define __IRadialControllerConfigurationInterop_INTERFACE_DEFINED__

/* interface IRadialControllerConfigurationInterop */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IRadialControllerConfigurationInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("787cdaac-3186-476d-87e4-b9374a7b9970")
    IRadialControllerConfigurationInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetForWindow( 
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRadialControllerConfigurationInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRadialControllerConfigurationInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRadialControllerConfigurationInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRadialControllerConfigurationInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IRadialControllerConfigurationInterop * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IRadialControllerConfigurationInterop * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IRadialControllerConfigurationInterop * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IRadialControllerConfigurationInterop, GetForWindow)
        HRESULT ( STDMETHODCALLTYPE *GetForWindow )( 
            IRadialControllerConfigurationInterop * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppv);
        
        END_INTERFACE
    } IRadialControllerConfigurationInteropVtbl;

    interface IRadialControllerConfigurationInterop
    {
        CONST_VTBL struct IRadialControllerConfigurationInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRadialControllerConfigurationInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRadialControllerConfigurationInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRadialControllerConfigurationInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRadialControllerConfigurationInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IRadialControllerConfigurationInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IRadialControllerConfigurationInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IRadialControllerConfigurationInterop_GetForWindow(This,hwnd,riid,ppv)	\
    ( (This)->lpVtbl -> GetForWindow(This,hwnd,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRadialControllerConfigurationInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_radialcontrollerinterop_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS1)
#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_radialcontrollerinterop_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_radialcontrollerinterop_0000_0002_v0_0_s_ifspec;

#ifndef __IRadialControllerIndependentInputSourceInterop_INTERFACE_DEFINED__
#define __IRadialControllerIndependentInputSourceInterop_INTERFACE_DEFINED__

/* interface IRadialControllerIndependentInputSourceInterop */
/* [local][object][uuid] */ 


EXTERN_C const IID IID_IRadialControllerIndependentInputSourceInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D577EFF-4CEE-11E6-B535-001BDC06AB3B")
    IRadialControllerIndependentInputSourceInterop : public IInspectable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateForWindow( 
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRadialControllerIndependentInputSourceInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRadialControllerIndependentInputSourceInterop * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRadialControllerIndependentInputSourceInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRadialControllerIndependentInputSourceInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            IRadialControllerIndependentInputSourceInterop * This,
            /* [out] */ ULONG *iidCount,
            /* [size_is][size_is][out] */ IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            IRadialControllerIndependentInputSourceInterop * This,
            /* [out] */ HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            IRadialControllerIndependentInputSourceInterop * This,
            /* [out] */ TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(IRadialControllerIndependentInputSourceInterop, CreateForWindow)
        HRESULT ( STDMETHODCALLTYPE *CreateForWindow )( 
            IRadialControllerIndependentInputSourceInterop * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppv);
        
        END_INTERFACE
    } IRadialControllerIndependentInputSourceInteropVtbl;

    interface IRadialControllerIndependentInputSourceInterop
    {
        CONST_VTBL struct IRadialControllerIndependentInputSourceInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRadialControllerIndependentInputSourceInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRadialControllerIndependentInputSourceInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRadialControllerIndependentInputSourceInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRadialControllerIndependentInputSourceInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define IRadialControllerIndependentInputSourceInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define IRadialControllerIndependentInputSourceInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define IRadialControllerIndependentInputSourceInterop_CreateForWindow(This,hwnd,riid,ppv)	\
    ( (This)->lpVtbl -> CreateForWindow(This,hwnd,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRadialControllerIndependentInputSourceInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_radialcontrollerinterop_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif //(NTDDI_VERSION >= NTDDI_WIN10_RS2)


extern RPC_IF_HANDLE __MIDL_itf_radialcontrollerinterop_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_radialcontrollerinterop_0000_0003_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


