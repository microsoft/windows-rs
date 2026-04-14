

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

#ifndef __windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_h__
#define __windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_h__

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

#ifndef __IDesktopWindowXamlSourceNative_FWD_DEFINED__
#define __IDesktopWindowXamlSourceNative_FWD_DEFINED__
typedef interface IDesktopWindowXamlSourceNative IDesktopWindowXamlSourceNative;

#endif 	/* __IDesktopWindowXamlSourceNative_FWD_DEFINED__ */


#ifndef __IDesktopWindowXamlSourceNative2_FWD_DEFINED__
#define __IDesktopWindowXamlSourceNative2_FWD_DEFINED__
typedef interface IDesktopWindowXamlSourceNative2 IDesktopWindowXamlSourceNative2;

#endif 	/* __IDesktopWindowXamlSourceNative2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN10_RS5)


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0000_v0_0_s_ifspec;

#ifndef __IDesktopWindowXamlSourceNative_INTERFACE_DEFINED__
#define __IDesktopWindowXamlSourceNative_INTERFACE_DEFINED__

/* interface IDesktopWindowXamlSourceNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IDesktopWindowXamlSourceNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3cbcf1bf-2f76-4e9c-96ab-e84b37972554")
    IDesktopWindowXamlSourceNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AttachToWindow( 
            /* [annotation][in] */ 
            _In_  HWND parentWnd) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowHandle( 
            /* [retval][out] */ HWND *hWnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDesktopWindowXamlSourceNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDesktopWindowXamlSourceNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDesktopWindowXamlSourceNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDesktopWindowXamlSourceNative * This);
        
        DECLSPEC_XFGVIRT(IDesktopWindowXamlSourceNative, AttachToWindow)
        HRESULT ( STDMETHODCALLTYPE *AttachToWindow )( 
            IDesktopWindowXamlSourceNative * This,
            /* [annotation][in] */ 
            _In_  HWND parentWnd);
        
        DECLSPEC_XFGVIRT(IDesktopWindowXamlSourceNative, get_WindowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowHandle )( 
            IDesktopWindowXamlSourceNative * This,
            /* [retval][out] */ HWND *hWnd);
        
        END_INTERFACE
    } IDesktopWindowXamlSourceNativeVtbl;

    interface IDesktopWindowXamlSourceNative
    {
        CONST_VTBL struct IDesktopWindowXamlSourceNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDesktopWindowXamlSourceNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDesktopWindowXamlSourceNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDesktopWindowXamlSourceNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDesktopWindowXamlSourceNative_AttachToWindow(This,parentWnd)	\
    ( (This)->lpVtbl -> AttachToWindow(This,parentWnd) ) 

#define IDesktopWindowXamlSourceNative_get_WindowHandle(This,hWnd)	\
    ( (This)->lpVtbl -> get_WindowHandle(This,hWnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDesktopWindowXamlSourceNative_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0001 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WIN10_RS5
#if (NTDDI_VERSION >= NTDDI_WIN10_19H1)


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0001_v0_0_s_ifspec;

#ifndef __IDesktopWindowXamlSourceNative2_INTERFACE_DEFINED__
#define __IDesktopWindowXamlSourceNative2_INTERFACE_DEFINED__

/* interface IDesktopWindowXamlSourceNative2 */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IDesktopWindowXamlSourceNative2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e3dcd8c7-3057-4692-99c3-7b7720afda31")
    IDesktopWindowXamlSourceNative2 : public IDesktopWindowXamlSourceNative
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PreTranslateMessage( 
            /* [annotation][in] */ 
            _In_  const MSG *message,
            /* [retval][out] */ BOOL *result) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDesktopWindowXamlSourceNative2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDesktopWindowXamlSourceNative2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDesktopWindowXamlSourceNative2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDesktopWindowXamlSourceNative2 * This);
        
        DECLSPEC_XFGVIRT(IDesktopWindowXamlSourceNative, AttachToWindow)
        HRESULT ( STDMETHODCALLTYPE *AttachToWindow )( 
            IDesktopWindowXamlSourceNative2 * This,
            /* [annotation][in] */ 
            _In_  HWND parentWnd);
        
        DECLSPEC_XFGVIRT(IDesktopWindowXamlSourceNative, get_WindowHandle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowHandle )( 
            IDesktopWindowXamlSourceNative2 * This,
            /* [retval][out] */ HWND *hWnd);
        
        DECLSPEC_XFGVIRT(IDesktopWindowXamlSourceNative2, PreTranslateMessage)
        HRESULT ( STDMETHODCALLTYPE *PreTranslateMessage )( 
            IDesktopWindowXamlSourceNative2 * This,
            /* [annotation][in] */ 
            _In_  const MSG *message,
            /* [retval][out] */ BOOL *result);
        
        END_INTERFACE
    } IDesktopWindowXamlSourceNative2Vtbl;

    interface IDesktopWindowXamlSourceNative2
    {
        CONST_VTBL struct IDesktopWindowXamlSourceNative2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDesktopWindowXamlSourceNative2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDesktopWindowXamlSourceNative2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDesktopWindowXamlSourceNative2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDesktopWindowXamlSourceNative2_AttachToWindow(This,parentWnd)	\
    ( (This)->lpVtbl -> AttachToWindow(This,parentWnd) ) 

#define IDesktopWindowXamlSourceNative2_get_WindowHandle(This,hWnd)	\
    ( (This)->lpVtbl -> get_WindowHandle(This,hWnd) ) 


#define IDesktopWindowXamlSourceNative2_PreTranslateMessage(This,message,result)	\
    ( (This)->lpVtbl -> PreTranslateMessage(This,message,result) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDesktopWindowXamlSourceNative2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0002 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WIN10_19H1


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Ehosting2Edesktopwindowxamlsource_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


