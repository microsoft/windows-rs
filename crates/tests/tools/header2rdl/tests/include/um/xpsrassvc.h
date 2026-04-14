

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

#ifndef __xpsrassvc_h__
#define __xpsrassvc_h__

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

#ifndef __IXpsRasterizerNotificationCallback_FWD_DEFINED__
#define __IXpsRasterizerNotificationCallback_FWD_DEFINED__
typedef interface IXpsRasterizerNotificationCallback IXpsRasterizerNotificationCallback;

#endif 	/* __IXpsRasterizerNotificationCallback_FWD_DEFINED__ */


#ifndef __IXpsRasterizer_FWD_DEFINED__
#define __IXpsRasterizer_FWD_DEFINED__
typedef interface IXpsRasterizer IXpsRasterizer;

#endif 	/* __IXpsRasterizer_FWD_DEFINED__ */


#ifndef __IXpsRasterizationFactory_FWD_DEFINED__
#define __IXpsRasterizationFactory_FWD_DEFINED__
typedef interface IXpsRasterizationFactory IXpsRasterizationFactory;

#endif 	/* __IXpsRasterizationFactory_FWD_DEFINED__ */


#ifndef __IXpsRasterizationFactory1_FWD_DEFINED__
#define __IXpsRasterizationFactory1_FWD_DEFINED__
typedef interface IXpsRasterizationFactory1 IXpsRasterizationFactory1;

#endif 	/* __IXpsRasterizationFactory1_FWD_DEFINED__ */


#ifndef __IXpsRasterizationFactory2_FWD_DEFINED__
#define __IXpsRasterizationFactory2_FWD_DEFINED__
typedef interface IXpsRasterizationFactory2 IXpsRasterizationFactory2;

#endif 	/* __IXpsRasterizationFactory2_FWD_DEFINED__ */


/* header files for imported files */
#include "wincodec.h"
#include "XpsObjectModel.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xpsrassvc_0000_0000 */
/* [local] */ 

//+--------------------------------------------------------------------------
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//----------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region App Partition
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
DEFINE_GUID(CLSID_XPSRASTERIZER_FACTORY, 0x503E79BF, 0x1D09, 0x4764, 0x9D, 0x72, 0x1E, 0xB0, 0xC6, 0x59, 0x67, 0xC6);


extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0000_v0_0_s_ifspec;

#ifndef __IXpsRasterizerNotificationCallback_INTERFACE_DEFINED__
#define __IXpsRasterizerNotificationCallback_INTERFACE_DEFINED__

/* interface IXpsRasterizerNotificationCallback */
/* [ref][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IXpsRasterizerNotificationCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9AB8FD0D-CB94-49c2-9CB0-97EC1D5469D2")
    IXpsRasterizerNotificationCallback : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE Continue( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsRasterizerNotificationCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsRasterizerNotificationCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsRasterizerNotificationCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsRasterizerNotificationCallback * This);
        
        DECLSPEC_XFGVIRT(IXpsRasterizerNotificationCallback, Continue)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *Continue )( 
            __RPC__in IXpsRasterizerNotificationCallback * This);
        
        END_INTERFACE
    } IXpsRasterizerNotificationCallbackVtbl;

    interface IXpsRasterizerNotificationCallback
    {
        CONST_VTBL struct IXpsRasterizerNotificationCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsRasterizerNotificationCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsRasterizerNotificationCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsRasterizerNotificationCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsRasterizerNotificationCallback_Continue(This)	\
    ( (This)->lpVtbl -> Continue(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsRasterizerNotificationCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsrassvc_0000_0001 */
/* [local] */ 

typedef /* [public][public][public][public][public][public][public] */ 
enum __MIDL___MIDL_itf_xpsrassvc_0000_0001_0001
    {
        XPSRAS_RENDERING_MODE_ANTIALIASED	= 0,
        XPSRAS_RENDERING_MODE_ALIASED	= 1
    } 	XPSRAS_RENDERING_MODE;



extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0001_v0_0_s_ifspec;

#ifndef __IXpsRasterizer_INTERFACE_DEFINED__
#define __IXpsRasterizer_INTERFACE_DEFINED__

/* interface IXpsRasterizer */
/* [ref][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IXpsRasterizer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7567CFC8-C156-47a8-9DAC-11A2AE5BDD6B")
    IXpsRasterizer : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE RasterizeRect( 
            /* [in] */ INT x,
            /* [in] */ INT y,
            /* [in] */ INT width,
            /* [in] */ INT height,
            /* [in] */ __RPC__in_opt IXpsRasterizerNotificationCallback *notificationCallback,
            /* [out] */ __RPC__deref_out_opt IWICBitmap **bitmap) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetMinimalLineWidth( 
            /* [in] */ INT width) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsRasterizerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsRasterizer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsRasterizer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsRasterizer * This);
        
        DECLSPEC_XFGVIRT(IXpsRasterizer, RasterizeRect)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *RasterizeRect )( 
            __RPC__in IXpsRasterizer * This,
            /* [in] */ INT x,
            /* [in] */ INT y,
            /* [in] */ INT width,
            /* [in] */ INT height,
            /* [in] */ __RPC__in_opt IXpsRasterizerNotificationCallback *notificationCallback,
            /* [out] */ __RPC__deref_out_opt IWICBitmap **bitmap);
        
        DECLSPEC_XFGVIRT(IXpsRasterizer, SetMinimalLineWidth)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetMinimalLineWidth )( 
            __RPC__in IXpsRasterizer * This,
            /* [in] */ INT width);
        
        END_INTERFACE
    } IXpsRasterizerVtbl;

    interface IXpsRasterizer
    {
        CONST_VTBL struct IXpsRasterizerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsRasterizer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsRasterizer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsRasterizer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsRasterizer_RasterizeRect(This,x,y,width,height,notificationCallback,bitmap)	\
    ( (This)->lpVtbl -> RasterizeRect(This,x,y,width,height,notificationCallback,bitmap) ) 

#define IXpsRasterizer_SetMinimalLineWidth(This,width)	\
    ( (This)->lpVtbl -> SetMinimalLineWidth(This,width) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsRasterizer_INTERFACE_DEFINED__ */


#ifndef __IXpsRasterizationFactory_INTERFACE_DEFINED__
#define __IXpsRasterizationFactory_INTERFACE_DEFINED__

/* interface IXpsRasterizationFactory */
/* [ref][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IXpsRasterizationFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E094808A-24C6-482b-A3A7-C21AC9B55F17")
    IXpsRasterizationFactory : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateRasterizer( 
            /* [in] */ __RPC__in_opt IXpsOMPage *xpsPage,
            /* [in] */ FLOAT DPI,
            /* [in] */ XPSRAS_RENDERING_MODE nonTextRenderingMode,
            /* [in] */ XPSRAS_RENDERING_MODE textRenderingMode,
            /* [out] */ __RPC__deref_out_opt IXpsRasterizer **ppIXPSRasterizer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsRasterizationFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsRasterizationFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsRasterizationFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsRasterizationFactory * This);
        
        DECLSPEC_XFGVIRT(IXpsRasterizationFactory, CreateRasterizer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateRasterizer )( 
            __RPC__in IXpsRasterizationFactory * This,
            /* [in] */ __RPC__in_opt IXpsOMPage *xpsPage,
            /* [in] */ FLOAT DPI,
            /* [in] */ XPSRAS_RENDERING_MODE nonTextRenderingMode,
            /* [in] */ XPSRAS_RENDERING_MODE textRenderingMode,
            /* [out] */ __RPC__deref_out_opt IXpsRasterizer **ppIXPSRasterizer);
        
        END_INTERFACE
    } IXpsRasterizationFactoryVtbl;

    interface IXpsRasterizationFactory
    {
        CONST_VTBL struct IXpsRasterizationFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsRasterizationFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsRasterizationFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsRasterizationFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsRasterizationFactory_CreateRasterizer(This,xpsPage,DPI,nonTextRenderingMode,textRenderingMode,ppIXPSRasterizer)	\
    ( (This)->lpVtbl -> CreateRasterizer(This,xpsPage,DPI,nonTextRenderingMode,textRenderingMode,ppIXPSRasterizer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsRasterizationFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsrassvc_0000_0003 */
/* [local] */ 

#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_xpsrassvc_0000_0003_0001
    {
        XPSRAS_PIXEL_FORMAT_32BPP_PBGRA_UINT_SRGB	= 1,
        XPSRAS_PIXEL_FORMAT_64BPP_PRGBA_HALF_SCRGB	= ( XPSRAS_PIXEL_FORMAT_32BPP_PBGRA_UINT_SRGB + 1 ) ,
        XPSRAS_PIXEL_FORMAT_128BPP_PRGBA_FLOAT_SCRGB	= ( XPSRAS_PIXEL_FORMAT_64BPP_PRGBA_HALF_SCRGB + 1 ) 
    } 	XPSRAS_PIXEL_FORMAT;



extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0003_v0_0_s_ifspec;

#ifndef __IXpsRasterizationFactory1_INTERFACE_DEFINED__
#define __IXpsRasterizationFactory1_INTERFACE_DEFINED__

/* interface IXpsRasterizationFactory1 */
/* [ref][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IXpsRasterizationFactory1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D6E5F77-6414-4A1E-A8E0-D4194CE6A26F")
    IXpsRasterizationFactory1 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateRasterizer( 
            /* [in] */ __RPC__in_opt IXpsOMPage *xpsPage,
            /* [in] */ FLOAT DPI,
            /* [in] */ XPSRAS_RENDERING_MODE nonTextRenderingMode,
            /* [in] */ XPSRAS_RENDERING_MODE textRenderingMode,
            /* [in] */ XPSRAS_PIXEL_FORMAT pixelFormat,
            /* [out] */ __RPC__deref_out_opt IXpsRasterizer **ppIXPSRasterizer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsRasterizationFactory1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsRasterizationFactory1 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsRasterizationFactory1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsRasterizationFactory1 * This);
        
        DECLSPEC_XFGVIRT(IXpsRasterizationFactory1, CreateRasterizer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateRasterizer )( 
            __RPC__in IXpsRasterizationFactory1 * This,
            /* [in] */ __RPC__in_opt IXpsOMPage *xpsPage,
            /* [in] */ FLOAT DPI,
            /* [in] */ XPSRAS_RENDERING_MODE nonTextRenderingMode,
            /* [in] */ XPSRAS_RENDERING_MODE textRenderingMode,
            /* [in] */ XPSRAS_PIXEL_FORMAT pixelFormat,
            /* [out] */ __RPC__deref_out_opt IXpsRasterizer **ppIXPSRasterizer);
        
        END_INTERFACE
    } IXpsRasterizationFactory1Vtbl;

    interface IXpsRasterizationFactory1
    {
        CONST_VTBL struct IXpsRasterizationFactory1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsRasterizationFactory1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsRasterizationFactory1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsRasterizationFactory1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsRasterizationFactory1_CreateRasterizer(This,xpsPage,DPI,nonTextRenderingMode,textRenderingMode,pixelFormat,ppIXPSRasterizer)	\
    ( (This)->lpVtbl -> CreateRasterizer(This,xpsPage,DPI,nonTextRenderingMode,textRenderingMode,pixelFormat,ppIXPSRasterizer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsRasterizationFactory1_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsrassvc_0000_0004 */
/* [local] */ 

#endif // (NTDDI >= NTDDI_WIN8)
#if (NTDDI_VERSION >= NTDDI_WINTHRESHOLD)
typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_xpsrassvc_0000_0004_0001
    {
        XPSRAS_BACKGROUND_COLOR_TRANSPARENT	= 0,
        XPSRAS_BACKGROUND_COLOR_OPAQUE	= 1
    } 	XPSRAS_BACKGROUND_COLOR;



extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0004_v0_0_s_ifspec;

#ifndef __IXpsRasterizationFactory2_INTERFACE_DEFINED__
#define __IXpsRasterizationFactory2_INTERFACE_DEFINED__

/* interface IXpsRasterizationFactory2 */
/* [ref][helpstring][nonextensible][uuid][object] */ 


EXTERN_C const IID IID_IXpsRasterizationFactory2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9C16CE3E-10F5-41FD-9DDC-6826669C2FF6")
    IXpsRasterizationFactory2 : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE CreateRasterizer( 
            /* [in] */ __RPC__in_opt IXpsOMPage *xpsPage,
            /* [in] */ FLOAT DPIX,
            /* [in] */ FLOAT DPIY,
            /* [in] */ XPSRAS_RENDERING_MODE nonTextRenderingMode,
            /* [in] */ XPSRAS_RENDERING_MODE textRenderingMode,
            /* [in] */ XPSRAS_PIXEL_FORMAT pixelFormat,
            /* [in] */ XPSRAS_BACKGROUND_COLOR backgroundColor,
            /* [out] */ __RPC__deref_out_opt IXpsRasterizer **ppIXpsRasterizer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsRasterizationFactory2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsRasterizationFactory2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsRasterizationFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsRasterizationFactory2 * This);
        
        DECLSPEC_XFGVIRT(IXpsRasterizationFactory2, CreateRasterizer)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *CreateRasterizer )( 
            __RPC__in IXpsRasterizationFactory2 * This,
            /* [in] */ __RPC__in_opt IXpsOMPage *xpsPage,
            /* [in] */ FLOAT DPIX,
            /* [in] */ FLOAT DPIY,
            /* [in] */ XPSRAS_RENDERING_MODE nonTextRenderingMode,
            /* [in] */ XPSRAS_RENDERING_MODE textRenderingMode,
            /* [in] */ XPSRAS_PIXEL_FORMAT pixelFormat,
            /* [in] */ XPSRAS_BACKGROUND_COLOR backgroundColor,
            /* [out] */ __RPC__deref_out_opt IXpsRasterizer **ppIXpsRasterizer);
        
        END_INTERFACE
    } IXpsRasterizationFactory2Vtbl;

    interface IXpsRasterizationFactory2
    {
        CONST_VTBL struct IXpsRasterizationFactory2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsRasterizationFactory2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsRasterizationFactory2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsRasterizationFactory2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsRasterizationFactory2_CreateRasterizer(This,xpsPage,DPIX,DPIY,nonTextRenderingMode,textRenderingMode,pixelFormat,backgroundColor,ppIXpsRasterizer)	\
    ( (This)->lpVtbl -> CreateRasterizer(This,xpsPage,DPIX,DPIY,nonTextRenderingMode,textRenderingMode,pixelFormat,backgroundColor,ppIXpsRasterizer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsRasterizationFactory2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsrassvc_0000_0005 */
/* [local] */ 

#endif // (NTDDI >= NTDDI_WINTHRESHOLD)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsrassvc_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


