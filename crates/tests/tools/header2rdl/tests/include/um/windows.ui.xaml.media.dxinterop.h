

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

#ifndef __windows2Eui2Examl2Emedia2Edxinterop_h__
#define __windows2Eui2Examl2Emedia2Edxinterop_h__

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

#ifndef __ISurfaceImageSourceNative_FWD_DEFINED__
#define __ISurfaceImageSourceNative_FWD_DEFINED__
typedef interface ISurfaceImageSourceNative ISurfaceImageSourceNative;

#endif 	/* __ISurfaceImageSourceNative_FWD_DEFINED__ */


#ifndef __IVirtualSurfaceUpdatesCallbackNative_FWD_DEFINED__
#define __IVirtualSurfaceUpdatesCallbackNative_FWD_DEFINED__
typedef interface IVirtualSurfaceUpdatesCallbackNative IVirtualSurfaceUpdatesCallbackNative;

#endif 	/* __IVirtualSurfaceUpdatesCallbackNative_FWD_DEFINED__ */


#ifndef __IVirtualSurfaceImageSourceNative_FWD_DEFINED__
#define __IVirtualSurfaceImageSourceNative_FWD_DEFINED__
typedef interface IVirtualSurfaceImageSourceNative IVirtualSurfaceImageSourceNative;

#endif 	/* __IVirtualSurfaceImageSourceNative_FWD_DEFINED__ */


#ifndef __ISwapChainBackgroundPanelNative_FWD_DEFINED__
#define __ISwapChainBackgroundPanelNative_FWD_DEFINED__
typedef interface ISwapChainBackgroundPanelNative ISwapChainBackgroundPanelNative;

#endif 	/* __ISwapChainBackgroundPanelNative_FWD_DEFINED__ */


#ifndef __ISurfaceImageSourceManagerNative_FWD_DEFINED__
#define __ISurfaceImageSourceManagerNative_FWD_DEFINED__
typedef interface ISurfaceImageSourceManagerNative ISurfaceImageSourceManagerNative;

#endif 	/* __ISurfaceImageSourceManagerNative_FWD_DEFINED__ */


#ifndef __ISurfaceImageSourceNativeWithD2D_FWD_DEFINED__
#define __ISurfaceImageSourceNativeWithD2D_FWD_DEFINED__
typedef interface ISurfaceImageSourceNativeWithD2D ISurfaceImageSourceNativeWithD2D;

#endif 	/* __ISurfaceImageSourceNativeWithD2D_FWD_DEFINED__ */


#ifndef __ISwapChainPanelNative_FWD_DEFINED__
#define __ISwapChainPanelNative_FWD_DEFINED__
typedef interface ISwapChainPanelNative ISwapChainPanelNative;

#endif 	/* __ISwapChainPanelNative_FWD_DEFINED__ */


#ifndef __ISwapChainPanelNative2_FWD_DEFINED__
#define __ISwapChainPanelNative2_FWD_DEFINED__
typedef interface ISwapChainPanelNative2 ISwapChainPanelNative2;

#endif 	/* __ISwapChainPanelNative2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "dxgi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0000 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN8)
#if 0
typedef RECT *REFRECT;

#endif // 0
#ifndef REFRECT
#ifdef __cplusplus
#define REFRECT const RECT &
#else // !__cplusplus
#define REFRECT const RECT * __MIDL_CONST
#endif // __cplusplus
#endif //REFRECT


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0000_v0_0_s_ifspec;

#ifndef __ISurfaceImageSourceNative_INTERFACE_DEFINED__
#define __ISurfaceImageSourceNative_INTERFACE_DEFINED__

/* interface ISurfaceImageSourceNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISurfaceImageSourceNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f2e9edc1-d307-4525-9886-0fafaa44163c")
    ISurfaceImageSourceNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDevice( 
            /* [annotation][in] */ 
            _In_  IDXGIDevice *device) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginDraw( 
            /* [annotation][in] */ 
            _In_  RECT updateRect,
            /* [annotation][out] */ 
            _Out_  IDXGISurface **surface,
            /* [annotation][out] */ 
            _Out_  POINT *offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndDraw( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurfaceImageSourceNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISurfaceImageSourceNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISurfaceImageSourceNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISurfaceImageSourceNative * This);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNative, SetDevice)
        HRESULT ( STDMETHODCALLTYPE *SetDevice )( 
            ISurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_  IDXGIDevice *device);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNative, BeginDraw)
        HRESULT ( STDMETHODCALLTYPE *BeginDraw )( 
            ISurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_  RECT updateRect,
            /* [annotation][out] */ 
            _Out_  IDXGISurface **surface,
            /* [annotation][out] */ 
            _Out_  POINT *offset);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNative, EndDraw)
        HRESULT ( STDMETHODCALLTYPE *EndDraw )( 
            ISurfaceImageSourceNative * This);
        
        END_INTERFACE
    } ISurfaceImageSourceNativeVtbl;

    interface ISurfaceImageSourceNative
    {
        CONST_VTBL struct ISurfaceImageSourceNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurfaceImageSourceNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurfaceImageSourceNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurfaceImageSourceNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurfaceImageSourceNative_SetDevice(This,device)	\
    ( (This)->lpVtbl -> SetDevice(This,device) ) 

#define ISurfaceImageSourceNative_BeginDraw(This,updateRect,surface,offset)	\
    ( (This)->lpVtbl -> BeginDraw(This,updateRect,surface,offset) ) 

#define ISurfaceImageSourceNative_EndDraw(This)	\
    ( (This)->lpVtbl -> EndDraw(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurfaceImageSourceNative_INTERFACE_DEFINED__ */


#ifndef __IVirtualSurfaceUpdatesCallbackNative_INTERFACE_DEFINED__
#define __IVirtualSurfaceUpdatesCallbackNative_INTERFACE_DEFINED__

/* interface IVirtualSurfaceUpdatesCallbackNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IVirtualSurfaceUpdatesCallbackNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dbf2e947-8e6c-4254-9eee-7738f71386c9")
    IVirtualSurfaceUpdatesCallbackNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UpdatesNeeded( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVirtualSurfaceUpdatesCallbackNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVirtualSurfaceUpdatesCallbackNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVirtualSurfaceUpdatesCallbackNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVirtualSurfaceUpdatesCallbackNative * This);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceUpdatesCallbackNative, UpdatesNeeded)
        HRESULT ( STDMETHODCALLTYPE *UpdatesNeeded )( 
            IVirtualSurfaceUpdatesCallbackNative * This);
        
        END_INTERFACE
    } IVirtualSurfaceUpdatesCallbackNativeVtbl;

    interface IVirtualSurfaceUpdatesCallbackNative
    {
        CONST_VTBL struct IVirtualSurfaceUpdatesCallbackNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVirtualSurfaceUpdatesCallbackNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVirtualSurfaceUpdatesCallbackNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVirtualSurfaceUpdatesCallbackNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVirtualSurfaceUpdatesCallbackNative_UpdatesNeeded(This)	\
    ( (This)->lpVtbl -> UpdatesNeeded(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVirtualSurfaceUpdatesCallbackNative_INTERFACE_DEFINED__ */


#ifndef __IVirtualSurfaceImageSourceNative_INTERFACE_DEFINED__
#define __IVirtualSurfaceImageSourceNative_INTERFACE_DEFINED__

/* interface IVirtualSurfaceImageSourceNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IVirtualSurfaceImageSourceNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e9550983-360b-4f53-b391-afd695078691")
    IVirtualSurfaceImageSourceNative : public ISurfaceImageSourceNative
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invalidate( 
            /* [annotation][in] */ 
            _In_  RECT updateRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUpdateRectCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUpdateRects( 
            /* [annotation][size_is][out] */ 
            _Out_writes_(count)  RECT *updates,
            /* [annotation][in] */ 
            _In_  DWORD count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVisibleBounds( 
            /* [annotation][out] */ 
            _Out_  RECT *bounds) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForUpdatesNeeded( 
            /* [annotation][in] */ 
            _In_opt_  IVirtualSurfaceUpdatesCallbackNative *callback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resize( 
            /* [annotation][in] */ 
            _In_  INT newWidth,
            /* [annotation][in] */ 
            _In_  INT newHeight) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVirtualSurfaceImageSourceNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVirtualSurfaceImageSourceNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVirtualSurfaceImageSourceNative * This);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNative, SetDevice)
        HRESULT ( STDMETHODCALLTYPE *SetDevice )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_  IDXGIDevice *device);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNative, BeginDraw)
        HRESULT ( STDMETHODCALLTYPE *BeginDraw )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_  RECT updateRect,
            /* [annotation][out] */ 
            _Out_  IDXGISurface **surface,
            /* [annotation][out] */ 
            _Out_  POINT *offset);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNative, EndDraw)
        HRESULT ( STDMETHODCALLTYPE *EndDraw )( 
            IVirtualSurfaceImageSourceNative * This);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceImageSourceNative, Invalidate)
        HRESULT ( STDMETHODCALLTYPE *Invalidate )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_  RECT updateRect);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceImageSourceNative, GetUpdateRectCount)
        HRESULT ( STDMETHODCALLTYPE *GetUpdateRectCount )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][out] */ 
            _Out_  DWORD *count);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceImageSourceNative, GetUpdateRects)
        HRESULT ( STDMETHODCALLTYPE *GetUpdateRects )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(count)  RECT *updates,
            /* [annotation][in] */ 
            _In_  DWORD count);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceImageSourceNative, GetVisibleBounds)
        HRESULT ( STDMETHODCALLTYPE *GetVisibleBounds )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][out] */ 
            _Out_  RECT *bounds);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceImageSourceNative, RegisterForUpdatesNeeded)
        HRESULT ( STDMETHODCALLTYPE *RegisterForUpdatesNeeded )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_opt_  IVirtualSurfaceUpdatesCallbackNative *callback);
        
        DECLSPEC_XFGVIRT(IVirtualSurfaceImageSourceNative, Resize)
        HRESULT ( STDMETHODCALLTYPE *Resize )( 
            IVirtualSurfaceImageSourceNative * This,
            /* [annotation][in] */ 
            _In_  INT newWidth,
            /* [annotation][in] */ 
            _In_  INT newHeight);
        
        END_INTERFACE
    } IVirtualSurfaceImageSourceNativeVtbl;

    interface IVirtualSurfaceImageSourceNative
    {
        CONST_VTBL struct IVirtualSurfaceImageSourceNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVirtualSurfaceImageSourceNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVirtualSurfaceImageSourceNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVirtualSurfaceImageSourceNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVirtualSurfaceImageSourceNative_SetDevice(This,device)	\
    ( (This)->lpVtbl -> SetDevice(This,device) ) 

#define IVirtualSurfaceImageSourceNative_BeginDraw(This,updateRect,surface,offset)	\
    ( (This)->lpVtbl -> BeginDraw(This,updateRect,surface,offset) ) 

#define IVirtualSurfaceImageSourceNative_EndDraw(This)	\
    ( (This)->lpVtbl -> EndDraw(This) ) 


#define IVirtualSurfaceImageSourceNative_Invalidate(This,updateRect)	\
    ( (This)->lpVtbl -> Invalidate(This,updateRect) ) 

#define IVirtualSurfaceImageSourceNative_GetUpdateRectCount(This,count)	\
    ( (This)->lpVtbl -> GetUpdateRectCount(This,count) ) 

#define IVirtualSurfaceImageSourceNative_GetUpdateRects(This,updates,count)	\
    ( (This)->lpVtbl -> GetUpdateRects(This,updates,count) ) 

#define IVirtualSurfaceImageSourceNative_GetVisibleBounds(This,bounds)	\
    ( (This)->lpVtbl -> GetVisibleBounds(This,bounds) ) 

#define IVirtualSurfaceImageSourceNative_RegisterForUpdatesNeeded(This,callback)	\
    ( (This)->lpVtbl -> RegisterForUpdatesNeeded(This,callback) ) 

#define IVirtualSurfaceImageSourceNative_Resize(This,newWidth,newHeight)	\
    ( (This)->lpVtbl -> Resize(This,newWidth,newHeight) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVirtualSurfaceImageSourceNative_INTERFACE_DEFINED__ */


#ifndef __ISwapChainBackgroundPanelNative_INTERFACE_DEFINED__
#define __ISwapChainBackgroundPanelNative_INTERFACE_DEFINED__

/* interface ISwapChainBackgroundPanelNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISwapChainBackgroundPanelNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43bebd4e-add5-4035-8f85-5608d08e9dc9")
    ISwapChainBackgroundPanelNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSwapChain( 
            /* [annotation][in] */ 
            _In_  IDXGISwapChain *swapChain) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISwapChainBackgroundPanelNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISwapChainBackgroundPanelNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISwapChainBackgroundPanelNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISwapChainBackgroundPanelNative * This);
        
        DECLSPEC_XFGVIRT(ISwapChainBackgroundPanelNative, SetSwapChain)
        HRESULT ( STDMETHODCALLTYPE *SetSwapChain )( 
            ISwapChainBackgroundPanelNative * This,
            /* [annotation][in] */ 
            _In_  IDXGISwapChain *swapChain);
        
        END_INTERFACE
    } ISwapChainBackgroundPanelNativeVtbl;

    interface ISwapChainBackgroundPanelNative
    {
        CONST_VTBL struct ISwapChainBackgroundPanelNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISwapChainBackgroundPanelNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISwapChainBackgroundPanelNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISwapChainBackgroundPanelNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISwapChainBackgroundPanelNative_SetSwapChain(This,swapChain)	\
    ( (This)->lpVtbl -> SetSwapChain(This,swapChain) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISwapChainBackgroundPanelNative_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0004 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WIN8
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#define E_SURFACE_CONTENTS_LOST 0x802b0020


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0004_v0_0_s_ifspec;

#ifndef __ISurfaceImageSourceManagerNative_INTERFACE_DEFINED__
#define __ISurfaceImageSourceManagerNative_INTERFACE_DEFINED__

/* interface ISurfaceImageSourceManagerNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISurfaceImageSourceManagerNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4C8798B7-1D88-4A0F-B59B-B93F600DE8C8")
    ISurfaceImageSourceManagerNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FlushAllSurfacesWithDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *device) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurfaceImageSourceManagerNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISurfaceImageSourceManagerNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISurfaceImageSourceManagerNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISurfaceImageSourceManagerNative * This);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceManagerNative, FlushAllSurfacesWithDevice)
        HRESULT ( STDMETHODCALLTYPE *FlushAllSurfacesWithDevice )( 
            ISurfaceImageSourceManagerNative * This,
            /* [annotation][in] */ 
            _In_  IUnknown *device);
        
        END_INTERFACE
    } ISurfaceImageSourceManagerNativeVtbl;

    interface ISurfaceImageSourceManagerNative
    {
        CONST_VTBL struct ISurfaceImageSourceManagerNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurfaceImageSourceManagerNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurfaceImageSourceManagerNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurfaceImageSourceManagerNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurfaceImageSourceManagerNative_FlushAllSurfacesWithDevice(This,device)	\
    ( (This)->lpVtbl -> FlushAllSurfacesWithDevice(This,device) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurfaceImageSourceManagerNative_INTERFACE_DEFINED__ */


#ifndef __ISurfaceImageSourceNativeWithD2D_INTERFACE_DEFINED__
#define __ISurfaceImageSourceNativeWithD2D_INTERFACE_DEFINED__

/* interface ISurfaceImageSourceNativeWithD2D */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISurfaceImageSourceNativeWithD2D;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54298223-41e1-4a41-9c08-02e8256864a1")
    ISurfaceImageSourceNativeWithD2D : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *device) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginDraw( 
            /* [annotation][in] */ 
            _In_  REFRECT updateRect,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][out] */ 
            _COM_Outptr_  void **updateObject,
            /* [annotation][out] */ 
            _Out_  POINT *offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndDraw( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SuspendDraw( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResumeDraw( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISurfaceImageSourceNativeWithD2DVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISurfaceImageSourceNativeWithD2D * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISurfaceImageSourceNativeWithD2D * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISurfaceImageSourceNativeWithD2D * This);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNativeWithD2D, SetDevice)
        HRESULT ( STDMETHODCALLTYPE *SetDevice )( 
            ISurfaceImageSourceNativeWithD2D * This,
            /* [annotation][in] */ 
            _In_  IUnknown *device);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNativeWithD2D, BeginDraw)
        HRESULT ( STDMETHODCALLTYPE *BeginDraw )( 
            ISurfaceImageSourceNativeWithD2D * This,
            /* [annotation][in] */ 
            _In_  REFRECT updateRect,
            /* [annotation][in] */ 
            _In_  REFIID iid,
            /* [annotation][out] */ 
            _COM_Outptr_  void **updateObject,
            /* [annotation][out] */ 
            _Out_  POINT *offset);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNativeWithD2D, EndDraw)
        HRESULT ( STDMETHODCALLTYPE *EndDraw )( 
            ISurfaceImageSourceNativeWithD2D * This);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNativeWithD2D, SuspendDraw)
        HRESULT ( STDMETHODCALLTYPE *SuspendDraw )( 
            ISurfaceImageSourceNativeWithD2D * This);
        
        DECLSPEC_XFGVIRT(ISurfaceImageSourceNativeWithD2D, ResumeDraw)
        HRESULT ( STDMETHODCALLTYPE *ResumeDraw )( 
            ISurfaceImageSourceNativeWithD2D * This);
        
        END_INTERFACE
    } ISurfaceImageSourceNativeWithD2DVtbl;

    interface ISurfaceImageSourceNativeWithD2D
    {
        CONST_VTBL struct ISurfaceImageSourceNativeWithD2DVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISurfaceImageSourceNativeWithD2D_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISurfaceImageSourceNativeWithD2D_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISurfaceImageSourceNativeWithD2D_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISurfaceImageSourceNativeWithD2D_SetDevice(This,device)	\
    ( (This)->lpVtbl -> SetDevice(This,device) ) 

#define ISurfaceImageSourceNativeWithD2D_BeginDraw(This,updateRect,iid,updateObject,offset)	\
    ( (This)->lpVtbl -> BeginDraw(This,updateRect,iid,updateObject,offset) ) 

#define ISurfaceImageSourceNativeWithD2D_EndDraw(This)	\
    ( (This)->lpVtbl -> EndDraw(This) ) 

#define ISurfaceImageSourceNativeWithD2D_SuspendDraw(This)	\
    ( (This)->lpVtbl -> SuspendDraw(This) ) 

#define ISurfaceImageSourceNativeWithD2D_ResumeDraw(This)	\
    ( (This)->lpVtbl -> ResumeDraw(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISurfaceImageSourceNativeWithD2D_INTERFACE_DEFINED__ */


#ifndef __ISwapChainPanelNative_INTERFACE_DEFINED__
#define __ISwapChainPanelNative_INTERFACE_DEFINED__

/* interface ISwapChainPanelNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISwapChainPanelNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F92F19D2-3ADE-45A6-A20C-F6F1EA90554B")
    ISwapChainPanelNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSwapChain( 
            /* [annotation][in] */ 
            _In_  IDXGISwapChain *swapChain) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISwapChainPanelNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISwapChainPanelNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISwapChainPanelNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISwapChainPanelNative * This);
        
        DECLSPEC_XFGVIRT(ISwapChainPanelNative, SetSwapChain)
        HRESULT ( STDMETHODCALLTYPE *SetSwapChain )( 
            ISwapChainPanelNative * This,
            /* [annotation][in] */ 
            _In_  IDXGISwapChain *swapChain);
        
        END_INTERFACE
    } ISwapChainPanelNativeVtbl;

    interface ISwapChainPanelNative
    {
        CONST_VTBL struct ISwapChainPanelNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISwapChainPanelNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISwapChainPanelNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISwapChainPanelNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISwapChainPanelNative_SetSwapChain(This,swapChain)	\
    ( (This)->lpVtbl -> SetSwapChain(This,swapChain) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISwapChainPanelNative_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0007 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WINBLUE
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0007_v0_0_s_ifspec;

#ifndef __ISwapChainPanelNative2_INTERFACE_DEFINED__
#define __ISwapChainPanelNative2_INTERFACE_DEFINED__

/* interface ISwapChainPanelNative2 */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_ISwapChainPanelNative2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D5A2F60C-37B2-44A2-937B-8D8EB9726821")
    ISwapChainPanelNative2 : public ISwapChainPanelNative
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSwapChainHandle( 
            /* [annotation][in] */ 
            _In_  HANDLE swapChainHandle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISwapChainPanelNative2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISwapChainPanelNative2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISwapChainPanelNative2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISwapChainPanelNative2 * This);
        
        DECLSPEC_XFGVIRT(ISwapChainPanelNative, SetSwapChain)
        HRESULT ( STDMETHODCALLTYPE *SetSwapChain )( 
            ISwapChainPanelNative2 * This,
            /* [annotation][in] */ 
            _In_  IDXGISwapChain *swapChain);
        
        DECLSPEC_XFGVIRT(ISwapChainPanelNative2, SetSwapChainHandle)
        HRESULT ( STDMETHODCALLTYPE *SetSwapChainHandle )( 
            ISwapChainPanelNative2 * This,
            /* [annotation][in] */ 
            _In_  HANDLE swapChainHandle);
        
        END_INTERFACE
    } ISwapChainPanelNative2Vtbl;

    interface ISwapChainPanelNative2
    {
        CONST_VTBL struct ISwapChainPanelNative2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISwapChainPanelNative2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISwapChainPanelNative2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISwapChainPanelNative2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISwapChainPanelNative2_SetSwapChain(This,swapChain)	\
    ( (This)->lpVtbl -> SetSwapChain(This,swapChain) ) 


#define ISwapChainPanelNative2_SetSwapChainHandle(This,swapChainHandle)	\
    ( (This)->lpVtbl -> SetSwapChainHandle(This,swapChainHandle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISwapChainPanelNative2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0008 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WINTHRESHOLD


extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Eui2Examl2Emedia2Edxinterop_0000_0008_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


