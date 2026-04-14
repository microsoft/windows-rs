

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

#ifndef __windows2Edata2Epdf2Einterop_h__
#define __windows2Edata2Epdf2Einterop_h__

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

#ifndef __IPdfRendererNative_FWD_DEFINED__
#define __IPdfRendererNative_FWD_DEFINED__
typedef interface IPdfRendererNative IPdfRendererNative;

#endif 	/* __IPdfRendererNative_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "dxgi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_windows2Edata2Epdf2Einterop_0000_0000 */
/* [local] */ 

#include <d2dbasetypes.h>
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#if 0
typedef DWORD *ID2D1DeviceContext;

typedef DWORD *D2D_RECT_F;

typedef DWORD *D2D_COLOR_F;

#endif
typedef interface ID2D1DeviceContext ID2D1DeviceContext;
typedef HRESULT (WINAPI* PFN_PDF_CREATE_RENDERER)(
    _In_ IDXGIDevice*,
    _Out_ IPdfRendererNative**);

HRESULT WINAPI PdfCreateRenderer(
    _In_ IDXGIDevice* pDevice,
    _Out_ IPdfRendererNative** ppRenderer);

typedef struct PDF_RENDER_PARAMS
    {
    D2D_RECT_F SourceRect;
    UINT32 DestinationWidth;
    UINT32 DestinationHeight;
    D2D_COLOR_F BackgroundColor;
    BOOLEAN IgnoreHighContrast;
    } 	PDF_RENDER_PARAMS;

#if defined(__cplusplus) && !defined(CINTERFACE)
static const D2D_RECT_F sc_PdfRenderParamsDefaultSrcRect = {0.f, 0.f, 0.f, 0.f};
static const D2D_COLOR_F sc_PdfRenderParamsDefaultBkColor = {1.f, 1.f, 1.f, 1.f};
__inline PDF_RENDER_PARAMS PdfRenderParams(
                                  _In_ CONST D2D_RECT_F& srcRect = sc_PdfRenderParamsDefaultSrcRect,
                                  _In_ UINT32 destinationWidth = 0.f,
                                  _In_ UINT32 destinationHeight = 0.f,
                                  _In_ CONST D2D_COLOR_F& bkColor = sc_PdfRenderParamsDefaultBkColor,
                                  _In_ BOOLEAN ignoreHighContrast = TRUE)
{ PDF_RENDER_PARAMS p = {srcRect, destinationWidth, destinationHeight, bkColor, ignoreHighContrast}; return p; }
#endif


extern RPC_IF_HANDLE __MIDL_itf_windows2Edata2Epdf2Einterop_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Edata2Epdf2Einterop_0000_0000_v0_0_s_ifspec;

#ifndef __IPdfRendererNative_INTERFACE_DEFINED__
#define __IPdfRendererNative_INTERFACE_DEFINED__

/* interface IPdfRendererNative */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IPdfRendererNative;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7d9dcd91-d277-4947-8527-07a0daeda94a")
    IPdfRendererNative : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RenderPageToSurface( 
            /* [annotation][in] */ 
            _In_  IUnknown *pdfPage,
            /* [annotation][in] */ 
            _In_  IDXGISurface *pSurface,
            /* [annotation][in] */ 
            _In_  POINT offset,
            /* [annotation][in] */ 
            _In_opt_  PDF_RENDER_PARAMS *pRenderParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderPageToDeviceContext( 
            /* [annotation][in] */ 
            _In_  IUnknown *pdfPage,
            /* [annotation][in] */ 
            _In_  ID2D1DeviceContext *pD2DDeviceContext,
            /* [annotation][in] */ 
            _In_opt_  PDF_RENDER_PARAMS *pRenderParams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPdfRendererNativeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPdfRendererNative * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPdfRendererNative * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPdfRendererNative * This);
        
        DECLSPEC_XFGVIRT(IPdfRendererNative, RenderPageToSurface)
        HRESULT ( STDMETHODCALLTYPE *RenderPageToSurface )( 
            IPdfRendererNative * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pdfPage,
            /* [annotation][in] */ 
            _In_  IDXGISurface *pSurface,
            /* [annotation][in] */ 
            _In_  POINT offset,
            /* [annotation][in] */ 
            _In_opt_  PDF_RENDER_PARAMS *pRenderParams);
        
        DECLSPEC_XFGVIRT(IPdfRendererNative, RenderPageToDeviceContext)
        HRESULT ( STDMETHODCALLTYPE *RenderPageToDeviceContext )( 
            IPdfRendererNative * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pdfPage,
            /* [annotation][in] */ 
            _In_  ID2D1DeviceContext *pD2DDeviceContext,
            /* [annotation][in] */ 
            _In_opt_  PDF_RENDER_PARAMS *pRenderParams);
        
        END_INTERFACE
    } IPdfRendererNativeVtbl;

    interface IPdfRendererNative
    {
        CONST_VTBL struct IPdfRendererNativeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPdfRendererNative_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPdfRendererNative_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPdfRendererNative_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPdfRendererNative_RenderPageToSurface(This,pdfPage,pSurface,offset,pRenderParams)	\
    ( (This)->lpVtbl -> RenderPageToSurface(This,pdfPage,pSurface,offset,pRenderParams) ) 

#define IPdfRendererNative_RenderPageToDeviceContext(This,pdfPage,pD2DDeviceContext,pRenderParams)	\
    ( (This)->lpVtbl -> RenderPageToDeviceContext(This,pdfPage,pD2DDeviceContext,pRenderParams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPdfRendererNative_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_windows2Edata2Epdf2Einterop_0000_0001 */
/* [local] */ 

#endif // NTDDI_VERSION >= NTDDI_WINBUE


extern RPC_IF_HANDLE __MIDL_itf_windows2Edata2Epdf2Einterop_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_windows2Edata2Epdf2Einterop_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


