

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

#ifndef __printpreview_h__
#define __printpreview_h__

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

#ifndef __IPrintPreviewDxgiPackageTarget_FWD_DEFINED__
#define __IPrintPreviewDxgiPackageTarget_FWD_DEFINED__
typedef interface IPrintPreviewDxgiPackageTarget IPrintPreviewDxgiPackageTarget;

#endif 	/* __IPrintPreviewDxgiPackageTarget_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "dxgi.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_printpreview_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if (NTDDI_VERSION >= NTDDI_WIN8)
typedef /* [v1_enum] */ 
enum PageCountType
    {
        FinalPageCount	= 0,
        IntermediatePageCount	= ( FinalPageCount + 1 ) 
    } 	PageCountType;



extern RPC_IF_HANDLE __MIDL_itf_printpreview_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printpreview_0000_0000_v0_0_s_ifspec;

#ifndef __IPrintPreviewDxgiPackageTarget_INTERFACE_DEFINED__
#define __IPrintPreviewDxgiPackageTarget_INTERFACE_DEFINED__

/* interface IPrintPreviewDxgiPackageTarget */
/* [object][helpstring][uuid] */ 


EXTERN_C const IID IID_IPrintPreviewDxgiPackageTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1a6dd0ad-1e2a-4e99-a5ba-91f17818290e")
    IPrintPreviewDxgiPackageTarget : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetJobPageCount( 
            /* [in] */ PageCountType countType,
            /* [in] */ UINT32 count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DrawPage( 
            /* [in] */ UINT32 jobPageNumber,
            /* [in] */ __RPC__in_opt IDXGISurface *pageImage,
            /* [in] */ FLOAT dpiX,
            /* [in] */ FLOAT dpiY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InvalidatePreview( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPrintPreviewDxgiPackageTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IPrintPreviewDxgiPackageTarget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IPrintPreviewDxgiPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IPrintPreviewDxgiPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IPrintPreviewDxgiPackageTarget, SetJobPageCount)
        HRESULT ( STDMETHODCALLTYPE *SetJobPageCount )( 
            __RPC__in IPrintPreviewDxgiPackageTarget * This,
            /* [in] */ PageCountType countType,
            /* [in] */ UINT32 count);
        
        DECLSPEC_XFGVIRT(IPrintPreviewDxgiPackageTarget, DrawPage)
        HRESULT ( STDMETHODCALLTYPE *DrawPage )( 
            __RPC__in IPrintPreviewDxgiPackageTarget * This,
            /* [in] */ UINT32 jobPageNumber,
            /* [in] */ __RPC__in_opt IDXGISurface *pageImage,
            /* [in] */ FLOAT dpiX,
            /* [in] */ FLOAT dpiY);
        
        DECLSPEC_XFGVIRT(IPrintPreviewDxgiPackageTarget, InvalidatePreview)
        HRESULT ( STDMETHODCALLTYPE *InvalidatePreview )( 
            __RPC__in IPrintPreviewDxgiPackageTarget * This);
        
        END_INTERFACE
    } IPrintPreviewDxgiPackageTargetVtbl;

    interface IPrintPreviewDxgiPackageTarget
    {
        CONST_VTBL struct IPrintPreviewDxgiPackageTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPrintPreviewDxgiPackageTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPrintPreviewDxgiPackageTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPrintPreviewDxgiPackageTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPrintPreviewDxgiPackageTarget_SetJobPageCount(This,countType,count)	\
    ( (This)->lpVtbl -> SetJobPageCount(This,countType,count) ) 

#define IPrintPreviewDxgiPackageTarget_DrawPage(This,jobPageNumber,pageImage,dpiX,dpiY)	\
    ( (This)->lpVtbl -> DrawPage(This,jobPageNumber,pageImage,dpiX,dpiY) ) 

#define IPrintPreviewDxgiPackageTarget_InvalidatePreview(This)	\
    ( (This)->lpVtbl -> InvalidatePreview(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPrintPreviewDxgiPackageTarget_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_printpreview_0000_0001 */
/* [local] */ 

#define ID_PREVIEWPACKAGETARGET_DXGI __uuidof(IPrintPreviewDxgiPackageTarget)
#endif //(NTDDI_VERSION >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_printpreview_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_printpreview_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


