

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

#ifndef __corewindow_h__
#define __corewindow_h__

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

#ifndef __ICoreWindowInterop_FWD_DEFINED__
#define __ICoreWindowInterop_FWD_DEFINED__
typedef interface ICoreWindowInterop ICoreWindowInterop;

#endif 	/* __ICoreWindowInterop_FWD_DEFINED__ */


#ifndef __ICoreInputInterop_FWD_DEFINED__
#define __ICoreInputInterop_FWD_DEFINED__
typedef interface ICoreInputInterop ICoreInputInterop;

#endif 	/* __ICoreInputInterop_FWD_DEFINED__ */


#ifndef __ICoreWindowComponentInterop_FWD_DEFINED__
#define __ICoreWindowComponentInterop_FWD_DEFINED__
typedef interface ICoreWindowComponentInterop ICoreWindowComponentInterop;

#endif 	/* __ICoreWindowComponentInterop_FWD_DEFINED__ */


#ifndef __ICoreWindowAdapterInterop_FWD_DEFINED__
#define __ICoreWindowAdapterInterop_FWD_DEFINED__
typedef interface ICoreWindowAdapterInterop ICoreWindowAdapterInterop;

#endif 	/* __ICoreWindowAdapterInterop_FWD_DEFINED__ */


#ifndef __ICoreInputInterop2_FWD_DEFINED__
#define __ICoreInputInterop2_FWD_DEFINED__
typedef interface ICoreInputInterop2 ICoreInputInterop2;

#endif 	/* __ICoreInputInterop2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "inspectable.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_corewindow_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0000_v0_0_s_ifspec;

#ifndef __ICoreWindowInterop_INTERFACE_DEFINED__
#define __ICoreWindowInterop_INTERFACE_DEFINED__

/* interface ICoreWindowInterop */
/* [uuid][object] */ 


EXTERN_C const IID IID_ICoreWindowInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("45D64A29-A63E-4CB6-B498-5781D298CB4F")
    ICoreWindowInterop : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowHandle( 
            /* [retval][out] */ __RPC__deref_out_opt HWND *hwnd) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MessageHandled( 
            /* [in] */ boolean value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWindowInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoreWindowInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoreWindowInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoreWindowInterop * This);
        
        DECLSPEC_XFGVIRT(ICoreWindowInterop, get_WindowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowHandle )( 
            __RPC__in ICoreWindowInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt HWND *hwnd);
        
        DECLSPEC_XFGVIRT(ICoreWindowInterop, put_MessageHandled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageHandled )( 
            __RPC__in ICoreWindowInterop * This,
            /* [in] */ boolean value);
        
        END_INTERFACE
    } ICoreWindowInteropVtbl;

    interface ICoreWindowInterop
    {
        CONST_VTBL struct ICoreWindowInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWindowInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWindowInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWindowInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWindowInterop_get_WindowHandle(This,hwnd)	\
    ( (This)->lpVtbl -> get_WindowHandle(This,hwnd) ) 

#define ICoreWindowInterop_put_MessageHandled(This,value)	\
    ( (This)->lpVtbl -> put_MessageHandled(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWindowInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_corewindow_0000_0001 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WIN8
#if (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0001_v0_0_s_ifspec;

#ifndef __ICoreInputInterop_INTERFACE_DEFINED__
#define __ICoreInputInterop_INTERFACE_DEFINED__

/* interface ICoreInputInterop */
/* [uuid][object] */ 


EXTERN_C const IID IID_ICoreInputInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("40BFE3E3-B75A-4479-AC96-475365749BB8")
    ICoreInputInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetInputSource( 
            /* [in] */ __RPC__in_opt IUnknown *value) = 0;
        
        virtual /* [propput] */ HRESULT STDMETHODCALLTYPE put_MessageHandled( 
            /* [in] */ boolean value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreInputInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoreInputInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoreInputInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoreInputInterop * This);
        
        DECLSPEC_XFGVIRT(ICoreInputInterop, SetInputSource)
        HRESULT ( STDMETHODCALLTYPE *SetInputSource )( 
            __RPC__in ICoreInputInterop * This,
            /* [in] */ __RPC__in_opt IUnknown *value);
        
        DECLSPEC_XFGVIRT(ICoreInputInterop, put_MessageHandled)
        /* [propput] */ HRESULT ( STDMETHODCALLTYPE *put_MessageHandled )( 
            __RPC__in ICoreInputInterop * This,
            /* [in] */ boolean value);
        
        END_INTERFACE
    } ICoreInputInteropVtbl;

    interface ICoreInputInterop
    {
        CONST_VTBL struct ICoreInputInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreInputInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreInputInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreInputInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreInputInterop_SetInputSource(This,value)	\
    ( (This)->lpVtbl -> SetInputSource(This,value) ) 

#define ICoreInputInterop_put_MessageHandled(This,value)	\
    ( (This)->lpVtbl -> put_MessageHandled(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreInputInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_corewindow_0000_0002 */
/* [local] */ 


HRESULT _cdecl CreateControlInput(
    _In_ REFIID riid,
    _COM_Outptr_ void **ppv);

HRESULT _cdecl CreateControlInputEx(
    _In_ IUnknown *pCoreWindow,
    _In_ REFIID riid,
    _COM_Outptr_ void **ppv);

#endif // NTDDI_VERSION >= NTDDI_WINBLUE
#if (NTDDI_VERSION >= NTDDI_THRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0002_v0_0_s_ifspec;

#ifndef __ICoreWindowComponentInterop_INTERFACE_DEFINED__
#define __ICoreWindowComponentInterop_INTERFACE_DEFINED__

/* interface ICoreWindowComponentInterop */
/* [uuid][object] */ 


EXTERN_C const IID IID_ICoreWindowComponentInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0576AB31-A310-4C40-BA31-FD37E0298DFA")
    ICoreWindowComponentInterop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConfigureComponentInput( 
            /* [in] */ UINT32 hostViewInstanceId,
            /* [in] */ __RPC__in HWND hwndHost,
            /* [in] */ __RPC__in_opt IUnknown *inputSourceVisual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewInstanceId( 
            /* [out] */ __RPC__out UINT32 *componentViewInstanceId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWindowComponentInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoreWindowComponentInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoreWindowComponentInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoreWindowComponentInterop * This);
        
        DECLSPEC_XFGVIRT(ICoreWindowComponentInterop, ConfigureComponentInput)
        HRESULT ( STDMETHODCALLTYPE *ConfigureComponentInput )( 
            __RPC__in ICoreWindowComponentInterop * This,
            /* [in] */ UINT32 hostViewInstanceId,
            /* [in] */ __RPC__in HWND hwndHost,
            /* [in] */ __RPC__in_opt IUnknown *inputSourceVisual);
        
        DECLSPEC_XFGVIRT(ICoreWindowComponentInterop, GetViewInstanceId)
        HRESULT ( STDMETHODCALLTYPE *GetViewInstanceId )( 
            __RPC__in ICoreWindowComponentInterop * This,
            /* [out] */ __RPC__out UINT32 *componentViewInstanceId);
        
        END_INTERFACE
    } ICoreWindowComponentInteropVtbl;

    interface ICoreWindowComponentInterop
    {
        CONST_VTBL struct ICoreWindowComponentInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWindowComponentInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWindowComponentInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWindowComponentInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWindowComponentInterop_ConfigureComponentInput(This,hostViewInstanceId,hwndHost,inputSourceVisual)	\
    ( (This)->lpVtbl -> ConfigureComponentInput(This,hostViewInstanceId,hwndHost,inputSourceVisual) ) 

#define ICoreWindowComponentInterop_GetViewInstanceId(This,componentViewInstanceId)	\
    ( (This)->lpVtbl -> GetViewInstanceId(This,componentViewInstanceId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWindowComponentInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_corewindow_0000_0003 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_THRESHOLD
#if (NTDDI_VERSION >= NTDDI_THRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0003_v0_0_s_ifspec;

#ifndef __ICoreWindowAdapterInterop_INTERFACE_DEFINED__
#define __ICoreWindowAdapterInterop_INTERFACE_DEFINED__

/* interface ICoreWindowAdapterInterop */
/* [uuid][object] */ 


EXTERN_C const IID IID_ICoreWindowAdapterInterop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a5b6fd1-cd73-4b6c-9cf4-2e869eaf470a")
    ICoreWindowAdapterInterop : public IInspectable
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AppActivationClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ApplicationViewClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CoreApplicationViewClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HoloViewClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_PositionerClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SystemNavigationClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TitleBarClientAdapter( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWindowClientAdapter( 
            /* [in] */ __RPC__in_opt IUnknown *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreWindowAdapterInteropVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoreWindowAdapterInterop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoreWindowAdapterInterop * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_AppActivationClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AppActivationClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_ApplicationViewClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ApplicationViewClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_CoreApplicationViewClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CoreApplicationViewClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_HoloViewClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HoloViewClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_PositionerClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_PositionerClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_SystemNavigationClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SystemNavigationClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, get_TitleBarClientAdapter)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TitleBarClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **value);
        
        DECLSPEC_XFGVIRT(ICoreWindowAdapterInterop, SetWindowClientAdapter)
        HRESULT ( STDMETHODCALLTYPE *SetWindowClientAdapter )( 
            __RPC__in ICoreWindowAdapterInterop * This,
            /* [in] */ __RPC__in_opt IUnknown *value);
        
        END_INTERFACE
    } ICoreWindowAdapterInteropVtbl;

    interface ICoreWindowAdapterInterop
    {
        CONST_VTBL struct ICoreWindowAdapterInteropVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreWindowAdapterInterop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreWindowAdapterInterop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreWindowAdapterInterop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreWindowAdapterInterop_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define ICoreWindowAdapterInterop_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define ICoreWindowAdapterInterop_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define ICoreWindowAdapterInterop_get_AppActivationClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_AppActivationClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_get_ApplicationViewClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_ApplicationViewClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_get_CoreApplicationViewClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_CoreApplicationViewClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_get_HoloViewClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_HoloViewClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_get_PositionerClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_PositionerClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_get_SystemNavigationClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_SystemNavigationClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_get_TitleBarClientAdapter(This,value)	\
    ( (This)->lpVtbl -> get_TitleBarClientAdapter(This,value) ) 

#define ICoreWindowAdapterInterop_SetWindowClientAdapter(This,value)	\
    ( (This)->lpVtbl -> SetWindowClientAdapter(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreWindowAdapterInterop_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_corewindow_0000_0004 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_THRESHOLD
#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)


extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0004_v0_0_s_ifspec;

#ifndef __ICoreInputInterop2_INTERFACE_DEFINED__
#define __ICoreInputInterop2_INTERFACE_DEFINED__

/* interface ICoreInputInterop2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_ICoreInputInterop2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B8A2ACD7-A0F0-40EE-8EE7-C82F59CC5CD4")
    ICoreInputInterop2 : public IInspectable
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowHandle( 
            /* [retval][out] */ __RPC__deref_out_opt HWND *window) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangeHostingContext( 
            /* [in] */ __RPC__in HWND newParentWindow,
            /* [in] */ UINT32 newViewInstanceId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICoreInputInterop2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICoreInputInterop2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICoreInputInterop2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICoreInputInterop2 * This);
        
        DECLSPEC_XFGVIRT(IInspectable, GetIids)
        HRESULT ( STDMETHODCALLTYPE *GetIids )( 
            __RPC__in ICoreInputInterop2 * This,
            /* [out] */ __RPC__out ULONG *iidCount,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*iidCount) IID **iids);
        
        DECLSPEC_XFGVIRT(IInspectable, GetRuntimeClassName)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeClassName )( 
            __RPC__in ICoreInputInterop2 * This,
            /* [out] */ __RPC__deref_out_opt HSTRING *className);
        
        DECLSPEC_XFGVIRT(IInspectable, GetTrustLevel)
        HRESULT ( STDMETHODCALLTYPE *GetTrustLevel )( 
            __RPC__in ICoreInputInterop2 * This,
            /* [out] */ __RPC__out TrustLevel *trustLevel);
        
        DECLSPEC_XFGVIRT(ICoreInputInterop2, get_WindowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowHandle )( 
            __RPC__in ICoreInputInterop2 * This,
            /* [retval][out] */ __RPC__deref_out_opt HWND *window);
        
        DECLSPEC_XFGVIRT(ICoreInputInterop2, ChangeHostingContext)
        HRESULT ( STDMETHODCALLTYPE *ChangeHostingContext )( 
            __RPC__in ICoreInputInterop2 * This,
            /* [in] */ __RPC__in HWND newParentWindow,
            /* [in] */ UINT32 newViewInstanceId);
        
        END_INTERFACE
    } ICoreInputInterop2Vtbl;

    interface ICoreInputInterop2
    {
        CONST_VTBL struct ICoreInputInterop2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICoreInputInterop2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICoreInputInterop2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICoreInputInterop2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICoreInputInterop2_GetIids(This,iidCount,iids)	\
    ( (This)->lpVtbl -> GetIids(This,iidCount,iids) ) 

#define ICoreInputInterop2_GetRuntimeClassName(This,className)	\
    ( (This)->lpVtbl -> GetRuntimeClassName(This,className) ) 

#define ICoreInputInterop2_GetTrustLevel(This,trustLevel)	\
    ( (This)->lpVtbl -> GetTrustLevel(This,trustLevel) ) 


#define ICoreInputInterop2_get_WindowHandle(This,window)	\
    ( (This)->lpVtbl -> get_WindowHandle(This,window) ) 

#define ICoreInputInterop2_ChangeHostingContext(This,newParentWindow,newViewInstanceId)	\
    ( (This)->lpVtbl -> ChangeHostingContext(This,newParentWindow,newViewInstanceId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICoreInputInterop2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_corewindow_0000_0005 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WIN10_19H1
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_corewindow_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  HWND_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree(     __RPC__in unsigned long *, __RPC__in HWND * ); 

unsigned long             __RPC_USER  HWND_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in HWND * ); 
unsigned char * __RPC_USER  HWND_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out HWND * ); 
void                      __RPC_USER  HWND_UserFree64(     __RPC__in unsigned long *, __RPC__in HWND * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


