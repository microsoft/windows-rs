

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

#ifndef __xpsobjectmodel_2_h__
#define __xpsobjectmodel_2_h__

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

#ifndef __IXpsOMPackageWriter3D_FWD_DEFINED__
#define __IXpsOMPackageWriter3D_FWD_DEFINED__
typedef interface IXpsOMPackageWriter3D IXpsOMPackageWriter3D;

#endif 	/* __IXpsOMPackageWriter3D_FWD_DEFINED__ */


#ifndef __IXpsDocumentPackageTarget3D_FWD_DEFINED__
#define __IXpsDocumentPackageTarget3D_FWD_DEFINED__
typedef interface IXpsDocumentPackageTarget3D IXpsDocumentPackageTarget3D;

#endif 	/* __IXpsDocumentPackageTarget3D_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "XpsObjectModel_1.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xpsobjectmodel_2_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if (NTDDI_VERSION >= NTDDI_WINBLUE)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_2_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_2_0000_0000_v0_0_s_ifspec;

#ifndef __IXpsOMPackageWriter3D_INTERFACE_DEFINED__
#define __IXpsOMPackageWriter3D_INTERFACE_DEFINED__

/* interface IXpsOMPackageWriter3D */
/* [local][ref][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPackageWriter3D;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e8a45033-640e-43fa-9bdf-fddeaa31c6a0")
    IXpsOMPackageWriter3D : public IXpsOMPackageWriter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddModelTexture( 
            /* [in] */ IOpcPartUri *texturePartName,
            /* [in] */ IStream *textureData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetModelPrintTicket( 
            /* [in] */ IOpcPartUri *printTicketPartName,
            /* [in] */ IStream *printTicketData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPackageWriter3DVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMPackageWriter3D * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMPackageWriter3D * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMPackageWriter3D * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, StartNewDocument)
        HRESULT ( STDMETHODCALLTYPE *StartNewDocument )( 
            IXpsOMPackageWriter3D * This,
            /* [in] */ IOpcPartUri *documentPartName,
            /* [in] */ IXpsOMPrintTicketResource *documentPrintTicket,
            /* [in] */ IXpsOMDocumentStructureResource *documentStructure,
            /* [in] */ IXpsOMSignatureBlockResourceCollection *signatureBlockResources,
            /* [in] */ IXpsOMPartUriCollection *restrictedFonts);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, AddPage)
        HRESULT ( STDMETHODCALLTYPE *AddPage )( 
            IXpsOMPackageWriter3D * This,
            /* [in] */ IXpsOMPage *page,
            /* [in] */ const XPS_SIZE *advisoryPageDimensions,
            /* [in] */ IXpsOMPartUriCollection *discardableResourceParts,
            /* [in] */ IXpsOMStoryFragmentsResource *storyFragments,
            /* [in] */ IXpsOMPrintTicketResource *pagePrintTicket,
            /* [in] */ IXpsOMImageResource *pageThumbnail);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, AddResource)
        HRESULT ( STDMETHODCALLTYPE *AddResource )( 
            IXpsOMPackageWriter3D * This,
            /* [in] */ IXpsOMResource *resource);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IXpsOMPackageWriter3D * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, IsClosed)
        HRESULT ( STDMETHODCALLTYPE *IsClosed )( 
            IXpsOMPackageWriter3D * This,
            /* [retval][out] */ BOOL *isClosed);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter3D, AddModelTexture)
        HRESULT ( STDMETHODCALLTYPE *AddModelTexture )( 
            IXpsOMPackageWriter3D * This,
            /* [in] */ IOpcPartUri *texturePartName,
            /* [in] */ IStream *textureData);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter3D, SetModelPrintTicket)
        HRESULT ( STDMETHODCALLTYPE *SetModelPrintTicket )( 
            IXpsOMPackageWriter3D * This,
            /* [in] */ IOpcPartUri *printTicketPartName,
            /* [in] */ IStream *printTicketData);
        
        END_INTERFACE
    } IXpsOMPackageWriter3DVtbl;

    interface IXpsOMPackageWriter3D
    {
        CONST_VTBL struct IXpsOMPackageWriter3DVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPackageWriter3D_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPackageWriter3D_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPackageWriter3D_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPackageWriter3D_StartNewDocument(This,documentPartName,documentPrintTicket,documentStructure,signatureBlockResources,restrictedFonts)	\
    ( (This)->lpVtbl -> StartNewDocument(This,documentPartName,documentPrintTicket,documentStructure,signatureBlockResources,restrictedFonts) ) 

#define IXpsOMPackageWriter3D_AddPage(This,page,advisoryPageDimensions,discardableResourceParts,storyFragments,pagePrintTicket,pageThumbnail)	\
    ( (This)->lpVtbl -> AddPage(This,page,advisoryPageDimensions,discardableResourceParts,storyFragments,pagePrintTicket,pageThumbnail) ) 

#define IXpsOMPackageWriter3D_AddResource(This,resource)	\
    ( (This)->lpVtbl -> AddResource(This,resource) ) 

#define IXpsOMPackageWriter3D_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IXpsOMPackageWriter3D_IsClosed(This,isClosed)	\
    ( (This)->lpVtbl -> IsClosed(This,isClosed) ) 


#define IXpsOMPackageWriter3D_AddModelTexture(This,texturePartName,textureData)	\
    ( (This)->lpVtbl -> AddModelTexture(This,texturePartName,textureData) ) 

#define IXpsOMPackageWriter3D_SetModelPrintTicket(This,printTicketPartName,printTicketData)	\
    ( (This)->lpVtbl -> SetModelPrintTicket(This,printTicketPartName,printTicketData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPackageWriter3D_INTERFACE_DEFINED__ */


#ifndef __IXpsDocumentPackageTarget3D_INTERFACE_DEFINED__
#define __IXpsDocumentPackageTarget3D_INTERFACE_DEFINED__

/* interface IXpsDocumentPackageTarget3D */
/* [local][ref][uuid][object] */ 


EXTERN_C const IID IID_IXpsDocumentPackageTarget3D;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60ba71b8-3101-4984-9199-f4ea775ff01d")
    IXpsDocumentPackageTarget3D : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetXpsOMPackageWriter3D( 
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [in] */ IOpcPartUri *modelPartName,
            /* [in] */ IStream *modelData,
            /* [retval][out] */ IXpsOMPackageWriter3D **packageWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXpsOMFactory( 
            /* [retval][out] */ IXpsOMObjectFactory **xpsFactory) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsDocumentPackageTarget3DVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsDocumentPackageTarget3D * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsDocumentPackageTarget3D * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsDocumentPackageTarget3D * This);
        
        DECLSPEC_XFGVIRT(IXpsDocumentPackageTarget3D, GetXpsOMPackageWriter3D)
        HRESULT ( STDMETHODCALLTYPE *GetXpsOMPackageWriter3D )( 
            IXpsDocumentPackageTarget3D * This,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [in] */ IOpcPartUri *modelPartName,
            /* [in] */ IStream *modelData,
            /* [retval][out] */ IXpsOMPackageWriter3D **packageWriter);
        
        DECLSPEC_XFGVIRT(IXpsDocumentPackageTarget3D, GetXpsOMFactory)
        HRESULT ( STDMETHODCALLTYPE *GetXpsOMFactory )( 
            IXpsDocumentPackageTarget3D * This,
            /* [retval][out] */ IXpsOMObjectFactory **xpsFactory);
        
        END_INTERFACE
    } IXpsDocumentPackageTarget3DVtbl;

    interface IXpsDocumentPackageTarget3D
    {
        CONST_VTBL struct IXpsDocumentPackageTarget3DVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsDocumentPackageTarget3D_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsDocumentPackageTarget3D_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsDocumentPackageTarget3D_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsDocumentPackageTarget3D_GetXpsOMPackageWriter3D(This,documentSequencePartName,discardControlPartName,modelPartName,modelData,packageWriter)	\
    ( (This)->lpVtbl -> GetXpsOMPackageWriter3D(This,documentSequencePartName,discardControlPartName,modelPartName,modelData,packageWriter) ) 

#define IXpsDocumentPackageTarget3D_GetXpsOMFactory(This,xpsFactory)	\
    ( (This)->lpVtbl -> GetXpsOMFactory(This,xpsFactory) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsDocumentPackageTarget3D_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsobjectmodel_2_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif


extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_2_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_2_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


