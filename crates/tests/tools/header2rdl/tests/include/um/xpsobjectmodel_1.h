

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

#ifndef __xpsobjectmodel_1_h__
#define __xpsobjectmodel_1_h__

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

#ifndef __IXpsOMObjectFactory1_FWD_DEFINED__
#define __IXpsOMObjectFactory1_FWD_DEFINED__
typedef interface IXpsOMObjectFactory1 IXpsOMObjectFactory1;

#endif 	/* __IXpsOMObjectFactory1_FWD_DEFINED__ */


#ifndef __IXpsOMPackage1_FWD_DEFINED__
#define __IXpsOMPackage1_FWD_DEFINED__
typedef interface IXpsOMPackage1 IXpsOMPackage1;

#endif 	/* __IXpsOMPackage1_FWD_DEFINED__ */


#ifndef __IXpsOMPage1_FWD_DEFINED__
#define __IXpsOMPage1_FWD_DEFINED__
typedef interface IXpsOMPage1 IXpsOMPage1;

#endif 	/* __IXpsOMPage1_FWD_DEFINED__ */


#ifndef __IXpsDocumentPackageTarget_FWD_DEFINED__
#define __IXpsDocumentPackageTarget_FWD_DEFINED__
typedef interface IXpsDocumentPackageTarget IXpsDocumentPackageTarget;

#endif 	/* __IXpsDocumentPackageTarget_FWD_DEFINED__ */


#ifndef __IXpsOMRemoteDictionaryResource1_FWD_DEFINED__
#define __IXpsOMRemoteDictionaryResource1_FWD_DEFINED__
typedef interface IXpsOMRemoteDictionaryResource1 IXpsOMRemoteDictionaryResource1;

#endif 	/* __IXpsOMRemoteDictionaryResource1_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "XpsObjectModel.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xpsobjectmodel_1_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if (NTDDI_VERSION >= NTDDI_WIN8)
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)



typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_1_0000_0000_0001
    {
        XPS_DOCUMENT_TYPE_UNSPECIFIED	= 1,
        XPS_DOCUMENT_TYPE_XPS	= ( XPS_DOCUMENT_TYPE_UNSPECIFIED + 1 ) ,
        XPS_DOCUMENT_TYPE_OPENXPS	= ( XPS_DOCUMENT_TYPE_XPS + 1 ) 
    } 	XPS_DOCUMENT_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_1_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_1_0000_0000_v0_0_s_ifspec;

#ifndef __IXpsOMObjectFactory1_INTERFACE_DEFINED__
#define __IXpsOMObjectFactory1_INTERFACE_DEFINED__

/* interface IXpsOMObjectFactory1 */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMObjectFactory1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0a91b617-d612-4181-bf7c-be5824e9cc8f")
    IXpsOMObjectFactory1 : public IXpsOMObjectFactory
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentTypeFromFile( 
            /* [in] */ LPCWSTR filename,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentTypeFromStream( 
            /* [in] */ IStream *xpsDocumentStream,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertHDPhotoToJpegXR( 
            /* [out][in] */ IXpsOMImageResource *imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertJpegXRToHDPhoto( 
            /* [out][in] */ IXpsOMImageResource *imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageWriterOnFile1( 
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [in] */ XPS_DOCUMENT_TYPE documentType,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageWriterOnStream1( 
            /* [in] */ ISequentialStream *outputStream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [in] */ XPS_DOCUMENT_TYPE documentType,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackage1( 
            /* [retval][out] */ IXpsOMPackage1 **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageFromStream1( 
            /* [in] */ IStream *stream,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage1 **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageFromFile1( 
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage1 **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePage1( 
            /* [in] */ const XPS_SIZE *pageDimensions,
            /* [string][in] */ LPCWSTR language,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPage1 **page) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePageFromStream1( 
            /* [in] */ IStream *pageMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPage1 **page) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRemoteDictionaryResourceFromStream1( 
            /* [in] */ IStream *dictionaryMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **dictionaryResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMObjectFactory1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMObjectFactory1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMObjectFactory1 * This);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackage)
        HRESULT ( STDMETHODCALLTYPE *CreatePackage )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageFromFile)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageFromFile )( 
            IXpsOMObjectFactory1 * This,
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageFromStream )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *stream,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateStoryFragmentsResource)
        HRESULT ( STDMETHODCALLTYPE *CreateStoryFragmentsResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMStoryFragmentsResource **storyFragmentsResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDocumentStructureResource)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentStructureResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocumentStructureResource **documentStructureResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateSignatureBlockResource)
        HRESULT ( STDMETHODCALLTYPE *CreateSignatureBlockResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMSignatureBlockResource **signatureBlockResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateRemoteDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *CreateRemoteDictionaryResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IXpsOMDictionary *dictionary,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateRemoteDictionaryResourceFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreateRemoteDictionaryResourceFromStream )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *dictionaryMarkupStream,
            /* [in] */ IOpcPartUri *dictionaryPartUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **dictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePartResources)
        HRESULT ( STDMETHODCALLTYPE *CreatePartResources )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMPartResources **partResources);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDocumentSequence)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentSequence )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocumentSequence **documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDocument)
        HRESULT ( STDMETHODCALLTYPE *CreateDocument )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocument **document);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePageReference)
        HRESULT ( STDMETHODCALLTYPE *CreatePageReference )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_SIZE *advisoryPageDimensions,
            /* [retval][out] */ IXpsOMPageReference **pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePage)
        HRESULT ( STDMETHODCALLTYPE *CreatePage )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_SIZE *pageDimensions,
            /* [string][in] */ LPCWSTR language,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPage **page);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePageFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreatePageFromStream )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *pageMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPage **page);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateCanvas)
        HRESULT ( STDMETHODCALLTYPE *CreateCanvas )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMCanvas **canvas);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGlyphs)
        HRESULT ( STDMETHODCALLTYPE *CreateGlyphs )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IXpsOMFontResource *fontResource,
            /* [retval][out] */ IXpsOMGlyphs **glyphs);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePath)
        HRESULT ( STDMETHODCALLTYPE *CreatePath )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMPath **path);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGeometry)
        HRESULT ( STDMETHODCALLTYPE *CreateGeometry )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMGeometry **geometry);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGeometryFigure)
        HRESULT ( STDMETHODCALLTYPE *CreateGeometryFigure )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_POINT *startPoint,
            /* [retval][out] */ IXpsOMGeometryFigure **figure);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateMatrixTransform)
        HRESULT ( STDMETHODCALLTYPE *CreateMatrixTransform )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_MATRIX *matrix,
            /* [retval][out] */ IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateSolidColorBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateSolidColorBrush )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_COLOR *color,
            /* [in] */ IXpsOMColorProfileResource *colorProfile,
            /* [retval][out] */ IXpsOMSolidColorBrush **solidColorBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateColorProfileResource)
        HRESULT ( STDMETHODCALLTYPE *CreateColorProfileResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMColorProfileResource **colorProfileResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateImageBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateImageBrush )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IXpsOMImageResource *image,
            /* [in] */ const XPS_RECT *viewBox,
            /* [in] */ const XPS_RECT *viewPort,
            /* [retval][out] */ IXpsOMImageBrush **imageBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateVisualBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateVisualBrush )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_RECT *viewBox,
            /* [in] */ const XPS_RECT *viewPort,
            /* [retval][out] */ IXpsOMVisualBrush **visualBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateImageResource)
        HRESULT ( STDMETHODCALLTYPE *CreateImageResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ XPS_IMAGE_TYPE contentType,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMImageResource **imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *CreatePrintTicketResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPrintTicketResource **printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateFontResource)
        HRESULT ( STDMETHODCALLTYPE *CreateFontResource )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ XPS_FONT_EMBEDDING fontEmbedding,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ BOOL isObfSourceStream,
            /* [retval][out] */ IXpsOMFontResource **fontResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGradientStop)
        HRESULT ( STDMETHODCALLTYPE *CreateGradientStop )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_COLOR *color,
            /* [in] */ IXpsOMColorProfileResource *colorProfile,
            /* [in] */ FLOAT offset,
            /* [retval][out] */ IXpsOMGradientStop **gradientStop);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateLinearGradientBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateLinearGradientBrush )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IXpsOMGradientStop *gradStop1,
            /* [in] */ IXpsOMGradientStop *gradStop2,
            /* [in] */ const XPS_POINT *startPoint,
            /* [in] */ const XPS_POINT *endPoint,
            /* [retval][out] */ IXpsOMLinearGradientBrush **linearGradientBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateRadialGradientBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateRadialGradientBrush )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IXpsOMGradientStop *gradStop1,
            /* [in] */ IXpsOMGradientStop *gradStop2,
            /* [in] */ const XPS_POINT *centerPoint,
            /* [in] */ const XPS_POINT *gradientOrigin,
            /* [in] */ const XPS_SIZE *radiiSizes,
            /* [retval][out] */ IXpsOMRadialGradientBrush **radialGradientBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateCoreProperties)
        HRESULT ( STDMETHODCALLTYPE *CreateCoreProperties )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMCoreProperties **coreProperties);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDictionary)
        HRESULT ( STDMETHODCALLTYPE *CreateDictionary )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMDictionary **dictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePartUriCollection)
        HRESULT ( STDMETHODCALLTYPE *CreatePartUriCollection )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMPartUriCollection **partUriCollection);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageWriterOnFile)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageWriterOnFile )( 
            IXpsOMObjectFactory1 * This,
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageWriterOnStream)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageWriterOnStream )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ ISequentialStream *outputStream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePartUri)
        HRESULT ( STDMETHODCALLTYPE *CreatePartUri )( 
            IXpsOMObjectFactory1 * This,
            /* [string][in] */ LPCWSTR uri,
            /* [retval][out] */ IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateReadOnlyStreamOnFile)
        HRESULT ( STDMETHODCALLTYPE *CreateReadOnlyStreamOnFile )( 
            IXpsOMObjectFactory1 * This,
            /* [string][in] */ LPCWSTR filename,
            /* [retval][out] */ IStream **stream);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, GetDocumentTypeFromFile)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentTypeFromFile )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ LPCWSTR filename,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, GetDocumentTypeFromStream)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentTypeFromStream )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *xpsDocumentStream,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, ConvertHDPhotoToJpegXR)
        HRESULT ( STDMETHODCALLTYPE *ConvertHDPhotoToJpegXR )( 
            IXpsOMObjectFactory1 * This,
            /* [out][in] */ IXpsOMImageResource *imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, ConvertJpegXRToHDPhoto)
        HRESULT ( STDMETHODCALLTYPE *ConvertJpegXRToHDPhoto )( 
            IXpsOMObjectFactory1 * This,
            /* [out][in] */ IXpsOMImageResource *imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePackageWriterOnFile1)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageWriterOnFile1 )( 
            IXpsOMObjectFactory1 * This,
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [in] */ XPS_DOCUMENT_TYPE documentType,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePackageWriterOnStream1)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageWriterOnStream1 )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ ISequentialStream *outputStream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [in] */ XPS_DOCUMENT_TYPE documentType,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePackage1)
        HRESULT ( STDMETHODCALLTYPE *CreatePackage1 )( 
            IXpsOMObjectFactory1 * This,
            /* [retval][out] */ IXpsOMPackage1 **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePackageFromStream1)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageFromStream1 )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *stream,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage1 **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePackageFromFile1)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageFromFile1 )( 
            IXpsOMObjectFactory1 * This,
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage1 **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePage1)
        HRESULT ( STDMETHODCALLTYPE *CreatePage1 )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ const XPS_SIZE *pageDimensions,
            /* [string][in] */ LPCWSTR language,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPage1 **page);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreatePageFromStream1)
        HRESULT ( STDMETHODCALLTYPE *CreatePageFromStream1 )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *pageMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPage1 **page);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory1, CreateRemoteDictionaryResourceFromStream1)
        HRESULT ( STDMETHODCALLTYPE *CreateRemoteDictionaryResourceFromStream1 )( 
            IXpsOMObjectFactory1 * This,
            /* [in] */ IStream *dictionaryMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **dictionaryResource);
        
        END_INTERFACE
    } IXpsOMObjectFactory1Vtbl;

    interface IXpsOMObjectFactory1
    {
        CONST_VTBL struct IXpsOMObjectFactory1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMObjectFactory1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMObjectFactory1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMObjectFactory1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMObjectFactory1_CreatePackage(This,package)	\
    ( (This)->lpVtbl -> CreatePackage(This,package) ) 

#define IXpsOMObjectFactory1_CreatePackageFromFile(This,filename,reuseObjects,package)	\
    ( (This)->lpVtbl -> CreatePackageFromFile(This,filename,reuseObjects,package) ) 

#define IXpsOMObjectFactory1_CreatePackageFromStream(This,stream,reuseObjects,package)	\
    ( (This)->lpVtbl -> CreatePackageFromStream(This,stream,reuseObjects,package) ) 

#define IXpsOMObjectFactory1_CreateStoryFragmentsResource(This,acquiredStream,partUri,storyFragmentsResource)	\
    ( (This)->lpVtbl -> CreateStoryFragmentsResource(This,acquiredStream,partUri,storyFragmentsResource) ) 

#define IXpsOMObjectFactory1_CreateDocumentStructureResource(This,acquiredStream,partUri,documentStructureResource)	\
    ( (This)->lpVtbl -> CreateDocumentStructureResource(This,acquiredStream,partUri,documentStructureResource) ) 

#define IXpsOMObjectFactory1_CreateSignatureBlockResource(This,acquiredStream,partUri,signatureBlockResource)	\
    ( (This)->lpVtbl -> CreateSignatureBlockResource(This,acquiredStream,partUri,signatureBlockResource) ) 

#define IXpsOMObjectFactory1_CreateRemoteDictionaryResource(This,dictionary,partUri,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> CreateRemoteDictionaryResource(This,dictionary,partUri,remoteDictionaryResource) ) 

#define IXpsOMObjectFactory1_CreateRemoteDictionaryResourceFromStream(This,dictionaryMarkupStream,dictionaryPartUri,resources,dictionaryResource)	\
    ( (This)->lpVtbl -> CreateRemoteDictionaryResourceFromStream(This,dictionaryMarkupStream,dictionaryPartUri,resources,dictionaryResource) ) 

#define IXpsOMObjectFactory1_CreatePartResources(This,partResources)	\
    ( (This)->lpVtbl -> CreatePartResources(This,partResources) ) 

#define IXpsOMObjectFactory1_CreateDocumentSequence(This,partUri,documentSequence)	\
    ( (This)->lpVtbl -> CreateDocumentSequence(This,partUri,documentSequence) ) 

#define IXpsOMObjectFactory1_CreateDocument(This,partUri,document)	\
    ( (This)->lpVtbl -> CreateDocument(This,partUri,document) ) 

#define IXpsOMObjectFactory1_CreatePageReference(This,advisoryPageDimensions,pageReference)	\
    ( (This)->lpVtbl -> CreatePageReference(This,advisoryPageDimensions,pageReference) ) 

#define IXpsOMObjectFactory1_CreatePage(This,pageDimensions,language,partUri,page)	\
    ( (This)->lpVtbl -> CreatePage(This,pageDimensions,language,partUri,page) ) 

#define IXpsOMObjectFactory1_CreatePageFromStream(This,pageMarkupStream,partUri,resources,reuseObjects,page)	\
    ( (This)->lpVtbl -> CreatePageFromStream(This,pageMarkupStream,partUri,resources,reuseObjects,page) ) 

#define IXpsOMObjectFactory1_CreateCanvas(This,canvas)	\
    ( (This)->lpVtbl -> CreateCanvas(This,canvas) ) 

#define IXpsOMObjectFactory1_CreateGlyphs(This,fontResource,glyphs)	\
    ( (This)->lpVtbl -> CreateGlyphs(This,fontResource,glyphs) ) 

#define IXpsOMObjectFactory1_CreatePath(This,path)	\
    ( (This)->lpVtbl -> CreatePath(This,path) ) 

#define IXpsOMObjectFactory1_CreateGeometry(This,geometry)	\
    ( (This)->lpVtbl -> CreateGeometry(This,geometry) ) 

#define IXpsOMObjectFactory1_CreateGeometryFigure(This,startPoint,figure)	\
    ( (This)->lpVtbl -> CreateGeometryFigure(This,startPoint,figure) ) 

#define IXpsOMObjectFactory1_CreateMatrixTransform(This,matrix,transform)	\
    ( (This)->lpVtbl -> CreateMatrixTransform(This,matrix,transform) ) 

#define IXpsOMObjectFactory1_CreateSolidColorBrush(This,color,colorProfile,solidColorBrush)	\
    ( (This)->lpVtbl -> CreateSolidColorBrush(This,color,colorProfile,solidColorBrush) ) 

#define IXpsOMObjectFactory1_CreateColorProfileResource(This,acquiredStream,partUri,colorProfileResource)	\
    ( (This)->lpVtbl -> CreateColorProfileResource(This,acquiredStream,partUri,colorProfileResource) ) 

#define IXpsOMObjectFactory1_CreateImageBrush(This,image,viewBox,viewPort,imageBrush)	\
    ( (This)->lpVtbl -> CreateImageBrush(This,image,viewBox,viewPort,imageBrush) ) 

#define IXpsOMObjectFactory1_CreateVisualBrush(This,viewBox,viewPort,visualBrush)	\
    ( (This)->lpVtbl -> CreateVisualBrush(This,viewBox,viewPort,visualBrush) ) 

#define IXpsOMObjectFactory1_CreateImageResource(This,acquiredStream,contentType,partUri,imageResource)	\
    ( (This)->lpVtbl -> CreateImageResource(This,acquiredStream,contentType,partUri,imageResource) ) 

#define IXpsOMObjectFactory1_CreatePrintTicketResource(This,acquiredStream,partUri,printTicketResource)	\
    ( (This)->lpVtbl -> CreatePrintTicketResource(This,acquiredStream,partUri,printTicketResource) ) 

#define IXpsOMObjectFactory1_CreateFontResource(This,acquiredStream,fontEmbedding,partUri,isObfSourceStream,fontResource)	\
    ( (This)->lpVtbl -> CreateFontResource(This,acquiredStream,fontEmbedding,partUri,isObfSourceStream,fontResource) ) 

#define IXpsOMObjectFactory1_CreateGradientStop(This,color,colorProfile,offset,gradientStop)	\
    ( (This)->lpVtbl -> CreateGradientStop(This,color,colorProfile,offset,gradientStop) ) 

#define IXpsOMObjectFactory1_CreateLinearGradientBrush(This,gradStop1,gradStop2,startPoint,endPoint,linearGradientBrush)	\
    ( (This)->lpVtbl -> CreateLinearGradientBrush(This,gradStop1,gradStop2,startPoint,endPoint,linearGradientBrush) ) 

#define IXpsOMObjectFactory1_CreateRadialGradientBrush(This,gradStop1,gradStop2,centerPoint,gradientOrigin,radiiSizes,radialGradientBrush)	\
    ( (This)->lpVtbl -> CreateRadialGradientBrush(This,gradStop1,gradStop2,centerPoint,gradientOrigin,radiiSizes,radialGradientBrush) ) 

#define IXpsOMObjectFactory1_CreateCoreProperties(This,partUri,coreProperties)	\
    ( (This)->lpVtbl -> CreateCoreProperties(This,partUri,coreProperties) ) 

#define IXpsOMObjectFactory1_CreateDictionary(This,dictionary)	\
    ( (This)->lpVtbl -> CreateDictionary(This,dictionary) ) 

#define IXpsOMObjectFactory1_CreatePartUriCollection(This,partUriCollection)	\
    ( (This)->lpVtbl -> CreatePartUriCollection(This,partUriCollection) ) 

#define IXpsOMObjectFactory1_CreatePackageWriterOnFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter)	\
    ( (This)->lpVtbl -> CreatePackageWriterOnFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter) ) 

#define IXpsOMObjectFactory1_CreatePackageWriterOnStream(This,outputStream,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter)	\
    ( (This)->lpVtbl -> CreatePackageWriterOnStream(This,outputStream,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter) ) 

#define IXpsOMObjectFactory1_CreatePartUri(This,uri,partUri)	\
    ( (This)->lpVtbl -> CreatePartUri(This,uri,partUri) ) 

#define IXpsOMObjectFactory1_CreateReadOnlyStreamOnFile(This,filename,stream)	\
    ( (This)->lpVtbl -> CreateReadOnlyStreamOnFile(This,filename,stream) ) 


#define IXpsOMObjectFactory1_GetDocumentTypeFromFile(This,filename,documentType)	\
    ( (This)->lpVtbl -> GetDocumentTypeFromFile(This,filename,documentType) ) 

#define IXpsOMObjectFactory1_GetDocumentTypeFromStream(This,xpsDocumentStream,documentType)	\
    ( (This)->lpVtbl -> GetDocumentTypeFromStream(This,xpsDocumentStream,documentType) ) 

#define IXpsOMObjectFactory1_ConvertHDPhotoToJpegXR(This,imageResource)	\
    ( (This)->lpVtbl -> ConvertHDPhotoToJpegXR(This,imageResource) ) 

#define IXpsOMObjectFactory1_ConvertJpegXRToHDPhoto(This,imageResource)	\
    ( (This)->lpVtbl -> ConvertJpegXRToHDPhoto(This,imageResource) ) 

#define IXpsOMObjectFactory1_CreatePackageWriterOnFile1(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,documentType,packageWriter)	\
    ( (This)->lpVtbl -> CreatePackageWriterOnFile1(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,documentType,packageWriter) ) 

#define IXpsOMObjectFactory1_CreatePackageWriterOnStream1(This,outputStream,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,documentType,packageWriter)	\
    ( (This)->lpVtbl -> CreatePackageWriterOnStream1(This,outputStream,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,documentType,packageWriter) ) 

#define IXpsOMObjectFactory1_CreatePackage1(This,package)	\
    ( (This)->lpVtbl -> CreatePackage1(This,package) ) 

#define IXpsOMObjectFactory1_CreatePackageFromStream1(This,stream,reuseObjects,package)	\
    ( (This)->lpVtbl -> CreatePackageFromStream1(This,stream,reuseObjects,package) ) 

#define IXpsOMObjectFactory1_CreatePackageFromFile1(This,filename,reuseObjects,package)	\
    ( (This)->lpVtbl -> CreatePackageFromFile1(This,filename,reuseObjects,package) ) 

#define IXpsOMObjectFactory1_CreatePage1(This,pageDimensions,language,partUri,page)	\
    ( (This)->lpVtbl -> CreatePage1(This,pageDimensions,language,partUri,page) ) 

#define IXpsOMObjectFactory1_CreatePageFromStream1(This,pageMarkupStream,partUri,resources,reuseObjects,page)	\
    ( (This)->lpVtbl -> CreatePageFromStream1(This,pageMarkupStream,partUri,resources,reuseObjects,page) ) 

#define IXpsOMObjectFactory1_CreateRemoteDictionaryResourceFromStream1(This,dictionaryMarkupStream,partUri,resources,dictionaryResource)	\
    ( (This)->lpVtbl -> CreateRemoteDictionaryResourceFromStream1(This,dictionaryMarkupStream,partUri,resources,dictionaryResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMObjectFactory1_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPackage1_INTERFACE_DEFINED__
#define __IXpsOMPackage1_INTERFACE_DEFINED__

/* interface IXpsOMPackage1 */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPackage1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95a9435e-12bb-461b-8e7f-c6adb04cd96a")
    IXpsOMPackage1 : public IXpsOMPackage
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentType( 
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteToFile1( 
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_DOCUMENT_TYPE documentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteToStream1( 
            /* [in] */ ISequentialStream *outputStream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_DOCUMENT_TYPE documentType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPackage1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMPackage1 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMPackage1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMPackage1 * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetDocumentSequence)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentSequence )( 
            IXpsOMPackage1 * This,
            /* [retval][out] */ IXpsOMDocumentSequence **documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetDocumentSequence)
        HRESULT ( STDMETHODCALLTYPE *SetDocumentSequence )( 
            IXpsOMPackage1 * This,
            /* [in] */ IXpsOMDocumentSequence *documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetCoreProperties)
        HRESULT ( STDMETHODCALLTYPE *GetCoreProperties )( 
            IXpsOMPackage1 * This,
            /* [retval][out] */ IXpsOMCoreProperties **coreProperties);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetCoreProperties)
        HRESULT ( STDMETHODCALLTYPE *SetCoreProperties )( 
            IXpsOMPackage1 * This,
            /* [in] */ IXpsOMCoreProperties *coreProperties);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetDiscardControlPartName)
        HRESULT ( STDMETHODCALLTYPE *GetDiscardControlPartName )( 
            IXpsOMPackage1 * This,
            /* [retval][out] */ IOpcPartUri **discardControlPartUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetDiscardControlPartName)
        HRESULT ( STDMETHODCALLTYPE *SetDiscardControlPartName )( 
            IXpsOMPackage1 * This,
            /* [in] */ IOpcPartUri *discardControlPartUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetThumbnailResource)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnailResource )( 
            IXpsOMPackage1 * This,
            /* [retval][out] */ IXpsOMImageResource **imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetThumbnailResource)
        HRESULT ( STDMETHODCALLTYPE *SetThumbnailResource )( 
            IXpsOMPackage1 * This,
            /* [in] */ IXpsOMImageResource *imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, WriteToFile)
        HRESULT ( STDMETHODCALLTYPE *WriteToFile )( 
            IXpsOMPackage1 * This,
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, WriteToStream)
        HRESULT ( STDMETHODCALLTYPE *WriteToStream )( 
            IXpsOMPackage1 * This,
            /* [in] */ ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage1, GetDocumentType)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentType )( 
            IXpsOMPackage1 * This,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage1, WriteToFile1)
        HRESULT ( STDMETHODCALLTYPE *WriteToFile1 )( 
            IXpsOMPackage1 * This,
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_DOCUMENT_TYPE documentType);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage1, WriteToStream1)
        HRESULT ( STDMETHODCALLTYPE *WriteToStream1 )( 
            IXpsOMPackage1 * This,
            /* [in] */ ISequentialStream *outputStream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_DOCUMENT_TYPE documentType);
        
        END_INTERFACE
    } IXpsOMPackage1Vtbl;

    interface IXpsOMPackage1
    {
        CONST_VTBL struct IXpsOMPackage1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPackage1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPackage1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPackage1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPackage1_GetDocumentSequence(This,documentSequence)	\
    ( (This)->lpVtbl -> GetDocumentSequence(This,documentSequence) ) 

#define IXpsOMPackage1_SetDocumentSequence(This,documentSequence)	\
    ( (This)->lpVtbl -> SetDocumentSequence(This,documentSequence) ) 

#define IXpsOMPackage1_GetCoreProperties(This,coreProperties)	\
    ( (This)->lpVtbl -> GetCoreProperties(This,coreProperties) ) 

#define IXpsOMPackage1_SetCoreProperties(This,coreProperties)	\
    ( (This)->lpVtbl -> SetCoreProperties(This,coreProperties) ) 

#define IXpsOMPackage1_GetDiscardControlPartName(This,discardControlPartUri)	\
    ( (This)->lpVtbl -> GetDiscardControlPartName(This,discardControlPartUri) ) 

#define IXpsOMPackage1_SetDiscardControlPartName(This,discardControlPartUri)	\
    ( (This)->lpVtbl -> SetDiscardControlPartName(This,discardControlPartUri) ) 

#define IXpsOMPackage1_GetThumbnailResource(This,imageResource)	\
    ( (This)->lpVtbl -> GetThumbnailResource(This,imageResource) ) 

#define IXpsOMPackage1_SetThumbnailResource(This,imageResource)	\
    ( (This)->lpVtbl -> SetThumbnailResource(This,imageResource) ) 

#define IXpsOMPackage1_WriteToFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize)	\
    ( (This)->lpVtbl -> WriteToFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize) ) 

#define IXpsOMPackage1_WriteToStream(This,stream,optimizeMarkupSize)	\
    ( (This)->lpVtbl -> WriteToStream(This,stream,optimizeMarkupSize) ) 


#define IXpsOMPackage1_GetDocumentType(This,documentType)	\
    ( (This)->lpVtbl -> GetDocumentType(This,documentType) ) 

#define IXpsOMPackage1_WriteToFile1(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,documentType)	\
    ( (This)->lpVtbl -> WriteToFile1(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,documentType) ) 

#define IXpsOMPackage1_WriteToStream1(This,outputStream,optimizeMarkupSize,documentType)	\
    ( (This)->lpVtbl -> WriteToStream1(This,outputStream,optimizeMarkupSize,documentType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPackage1_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPage1_INTERFACE_DEFINED__
#define __IXpsOMPage1_INTERFACE_DEFINED__

/* interface IXpsOMPage1 */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPage1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("305b60ef-6892-4dda-9cbb-3aa65974508a")
    IXpsOMPage1 : public IXpsOMPage
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentType( 
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Write1( 
            /* [in] */ ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_DOCUMENT_TYPE documentType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPage1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMPage1 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMPage1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMPage1 * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            IXpsOMPage1 * This,
            /* [in] */ IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IXpsOMPageReference **pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetVisuals)
        HRESULT ( STDMETHODCALLTYPE *GetVisuals )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IXpsOMVisualCollection **visuals);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetPageDimensions)
        HRESULT ( STDMETHODCALLTYPE *GetPageDimensions )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ XPS_SIZE *pageDimensions);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetPageDimensions)
        HRESULT ( STDMETHODCALLTYPE *SetPageDimensions )( 
            IXpsOMPage1 * This,
            /* [in] */ const XPS_SIZE *pageDimensions);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetContentBox)
        HRESULT ( STDMETHODCALLTYPE *GetContentBox )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ XPS_RECT *contentBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetContentBox)
        HRESULT ( STDMETHODCALLTYPE *SetContentBox )( 
            IXpsOMPage1 * This,
            /* [in] */ const XPS_RECT *contentBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetBleedBox)
        HRESULT ( STDMETHODCALLTYPE *GetBleedBox )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ XPS_RECT *bleedBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetBleedBox)
        HRESULT ( STDMETHODCALLTYPE *SetBleedBox )( 
            IXpsOMPage1 * This,
            /* [in] */ const XPS_RECT *bleedBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            IXpsOMPage1 * This,
            /* [retval][string][out] */ LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            IXpsOMPage1 * This,
            /* [string][in] */ LPCWSTR language);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IXpsOMPage1 * This,
            /* [retval][string][out] */ LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            IXpsOMPage1 * This,
            /* [string][in] */ LPCWSTR name);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *GetIsHyperlinkTarget )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ BOOL *isHyperlinkTarget);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *SetIsHyperlinkTarget )( 
            IXpsOMPage1 * This,
            /* [in] */ BOOL isHyperlinkTarget);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetDictionary)
        HRESULT ( STDMETHODCALLTYPE *GetDictionary )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IXpsOMDictionary **resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetDictionaryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetDictionaryLocal )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IXpsOMDictionary **resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetDictionaryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetDictionaryLocal )( 
            IXpsOMPage1 * This,
            /* [in] */ IXpsOMDictionary *resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *GetDictionaryResource )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *SetDictionaryResource )( 
            IXpsOMPage1 * This,
            /* [in] */ IXpsOMRemoteDictionaryResource *remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            IXpsOMPage1 * This,
            /* [in] */ ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GenerateUnusedLookupKey)
        HRESULT ( STDMETHODCALLTYPE *GenerateUnusedLookupKey )( 
            IXpsOMPage1 * This,
            /* [in] */ XPS_OBJECT_TYPE type,
            /* [retval][string][out] */ LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ IXpsOMPage **page);
        
        DECLSPEC_XFGVIRT(IXpsOMPage1, GetDocumentType)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentType )( 
            IXpsOMPage1 * This,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType);
        
        DECLSPEC_XFGVIRT(IXpsOMPage1, Write1)
        HRESULT ( STDMETHODCALLTYPE *Write1 )( 
            IXpsOMPage1 * This,
            /* [in] */ ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_DOCUMENT_TYPE documentType);
        
        END_INTERFACE
    } IXpsOMPage1Vtbl;

    interface IXpsOMPage1
    {
        CONST_VTBL struct IXpsOMPage1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPage1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPage1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPage1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPage1_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMPage1_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 


#define IXpsOMPage1_GetOwner(This,pageReference)	\
    ( (This)->lpVtbl -> GetOwner(This,pageReference) ) 

#define IXpsOMPage1_GetVisuals(This,visuals)	\
    ( (This)->lpVtbl -> GetVisuals(This,visuals) ) 

#define IXpsOMPage1_GetPageDimensions(This,pageDimensions)	\
    ( (This)->lpVtbl -> GetPageDimensions(This,pageDimensions) ) 

#define IXpsOMPage1_SetPageDimensions(This,pageDimensions)	\
    ( (This)->lpVtbl -> SetPageDimensions(This,pageDimensions) ) 

#define IXpsOMPage1_GetContentBox(This,contentBox)	\
    ( (This)->lpVtbl -> GetContentBox(This,contentBox) ) 

#define IXpsOMPage1_SetContentBox(This,contentBox)	\
    ( (This)->lpVtbl -> SetContentBox(This,contentBox) ) 

#define IXpsOMPage1_GetBleedBox(This,bleedBox)	\
    ( (This)->lpVtbl -> GetBleedBox(This,bleedBox) ) 

#define IXpsOMPage1_SetBleedBox(This,bleedBox)	\
    ( (This)->lpVtbl -> SetBleedBox(This,bleedBox) ) 

#define IXpsOMPage1_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMPage1_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 

#define IXpsOMPage1_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IXpsOMPage1_SetName(This,name)	\
    ( (This)->lpVtbl -> SetName(This,name) ) 

#define IXpsOMPage1_GetIsHyperlinkTarget(This,isHyperlinkTarget)	\
    ( (This)->lpVtbl -> GetIsHyperlinkTarget(This,isHyperlinkTarget) ) 

#define IXpsOMPage1_SetIsHyperlinkTarget(This,isHyperlinkTarget)	\
    ( (This)->lpVtbl -> SetIsHyperlinkTarget(This,isHyperlinkTarget) ) 

#define IXpsOMPage1_GetDictionary(This,resourceDictionary)	\
    ( (This)->lpVtbl -> GetDictionary(This,resourceDictionary) ) 

#define IXpsOMPage1_GetDictionaryLocal(This,resourceDictionary)	\
    ( (This)->lpVtbl -> GetDictionaryLocal(This,resourceDictionary) ) 

#define IXpsOMPage1_SetDictionaryLocal(This,resourceDictionary)	\
    ( (This)->lpVtbl -> SetDictionaryLocal(This,resourceDictionary) ) 

#define IXpsOMPage1_GetDictionaryResource(This,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> GetDictionaryResource(This,remoteDictionaryResource) ) 

#define IXpsOMPage1_SetDictionaryResource(This,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> SetDictionaryResource(This,remoteDictionaryResource) ) 

#define IXpsOMPage1_Write(This,stream,optimizeMarkupSize)	\
    ( (This)->lpVtbl -> Write(This,stream,optimizeMarkupSize) ) 

#define IXpsOMPage1_GenerateUnusedLookupKey(This,type,key)	\
    ( (This)->lpVtbl -> GenerateUnusedLookupKey(This,type,key) ) 

#define IXpsOMPage1_Clone(This,page)	\
    ( (This)->lpVtbl -> Clone(This,page) ) 


#define IXpsOMPage1_GetDocumentType(This,documentType)	\
    ( (This)->lpVtbl -> GetDocumentType(This,documentType) ) 

#define IXpsOMPage1_Write1(This,stream,optimizeMarkupSize,documentType)	\
    ( (This)->lpVtbl -> Write1(This,stream,optimizeMarkupSize,documentType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPage1_INTERFACE_DEFINED__ */


#ifndef __IXpsDocumentPackageTarget_INTERFACE_DEFINED__
#define __IXpsDocumentPackageTarget_INTERFACE_DEFINED__

/* interface IXpsDocumentPackageTarget */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsDocumentPackageTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3b0b6d38-53ad-41da-b212-d37637a6714e")
    IXpsDocumentPackageTarget : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetXpsOMPackageWriter( 
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXpsOMFactory( 
            /* [retval][out] */ IXpsOMObjectFactory **xpsFactory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXpsType( 
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsDocumentPackageTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsDocumentPackageTarget * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsDocumentPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsDocumentPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IXpsDocumentPackageTarget, GetXpsOMPackageWriter)
        HRESULT ( STDMETHODCALLTYPE *GetXpsOMPackageWriter )( 
            IXpsDocumentPackageTarget * This,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter);
        
        DECLSPEC_XFGVIRT(IXpsDocumentPackageTarget, GetXpsOMFactory)
        HRESULT ( STDMETHODCALLTYPE *GetXpsOMFactory )( 
            IXpsDocumentPackageTarget * This,
            /* [retval][out] */ IXpsOMObjectFactory **xpsFactory);
        
        DECLSPEC_XFGVIRT(IXpsDocumentPackageTarget, GetXpsType)
        HRESULT ( STDMETHODCALLTYPE *GetXpsType )( 
            IXpsDocumentPackageTarget * This,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType);
        
        END_INTERFACE
    } IXpsDocumentPackageTargetVtbl;

    interface IXpsDocumentPackageTarget
    {
        CONST_VTBL struct IXpsDocumentPackageTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsDocumentPackageTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsDocumentPackageTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsDocumentPackageTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsDocumentPackageTarget_GetXpsOMPackageWriter(This,documentSequencePartName,discardControlPartName,packageWriter)	\
    ( (This)->lpVtbl -> GetXpsOMPackageWriter(This,documentSequencePartName,discardControlPartName,packageWriter) ) 

#define IXpsDocumentPackageTarget_GetXpsOMFactory(This,xpsFactory)	\
    ( (This)->lpVtbl -> GetXpsOMFactory(This,xpsFactory) ) 

#define IXpsDocumentPackageTarget_GetXpsType(This,documentType)	\
    ( (This)->lpVtbl -> GetXpsType(This,documentType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsDocumentPackageTarget_INTERFACE_DEFINED__ */


#ifndef __IXpsOMRemoteDictionaryResource1_INTERFACE_DEFINED__
#define __IXpsOMRemoteDictionaryResource1_INTERFACE_DEFINED__

/* interface IXpsOMRemoteDictionaryResource1 */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMRemoteDictionaryResource1;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF8FC1D4-9D46-4141-BA5F-94BB9250D041")
    IXpsOMRemoteDictionaryResource1 : public IXpsOMRemoteDictionaryResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDocumentType( 
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Write1( 
            /* [in] */ ISequentialStream *stream,
            /* [in] */ XPS_DOCUMENT_TYPE documentType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMRemoteDictionaryResource1Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMRemoteDictionaryResource1 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMRemoteDictionaryResource1 * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [retval][out] */ IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [in] */ IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResource, GetDictionary)
        HRESULT ( STDMETHODCALLTYPE *GetDictionary )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [retval][out] */ IXpsOMDictionary **dictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResource, SetDictionary)
        HRESULT ( STDMETHODCALLTYPE *SetDictionary )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [in] */ IXpsOMDictionary *dictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResource1, GetDocumentType)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentType )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [retval][out] */ XPS_DOCUMENT_TYPE *documentType);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResource1, Write1)
        HRESULT ( STDMETHODCALLTYPE *Write1 )( 
            IXpsOMRemoteDictionaryResource1 * This,
            /* [in] */ ISequentialStream *stream,
            /* [in] */ XPS_DOCUMENT_TYPE documentType);
        
        END_INTERFACE
    } IXpsOMRemoteDictionaryResource1Vtbl;

    interface IXpsOMRemoteDictionaryResource1
    {
        CONST_VTBL struct IXpsOMRemoteDictionaryResource1Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMRemoteDictionaryResource1_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMRemoteDictionaryResource1_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMRemoteDictionaryResource1_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMRemoteDictionaryResource1_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMRemoteDictionaryResource1_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMRemoteDictionaryResource1_GetDictionary(This,dictionary)	\
    ( (This)->lpVtbl -> GetDictionary(This,dictionary) ) 

#define IXpsOMRemoteDictionaryResource1_SetDictionary(This,dictionary)	\
    ( (This)->lpVtbl -> SetDictionary(This,dictionary) ) 


#define IXpsOMRemoteDictionaryResource1_GetDocumentType(This,documentType)	\
    ( (This)->lpVtbl -> GetDocumentType(This,documentType) ) 

#define IXpsOMRemoteDictionaryResource1_Write1(This,stream,documentType)	\
    ( (This)->lpVtbl -> Write1(This,stream,documentType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMRemoteDictionaryResource1_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsobjectmodel_1_0000_0005 */
/* [local] */ 

#define XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS     MAKE_HRESULT(1, FACILITY_XPS, 0x600) 
#define XPS_E_ABSOLUTE_REFERENCE                             MAKE_HRESULT(1, FACILITY_XPS, 0x601) 
#define XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS               MAKE_HRESULT(1, FACILITY_XPS, 0x602) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_1_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_1_0000_0005_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


