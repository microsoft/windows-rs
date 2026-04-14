

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

#ifndef __xpsobjectmodel_h__
#define __xpsobjectmodel_h__

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

#ifndef __IXpsOMShareable_FWD_DEFINED__
#define __IXpsOMShareable_FWD_DEFINED__
typedef interface IXpsOMShareable IXpsOMShareable;

#endif 	/* __IXpsOMShareable_FWD_DEFINED__ */


#ifndef __IXpsOMVisual_FWD_DEFINED__
#define __IXpsOMVisual_FWD_DEFINED__
typedef interface IXpsOMVisual IXpsOMVisual;

#endif 	/* __IXpsOMVisual_FWD_DEFINED__ */


#ifndef __IXpsOMPart_FWD_DEFINED__
#define __IXpsOMPart_FWD_DEFINED__
typedef interface IXpsOMPart IXpsOMPart;

#endif 	/* __IXpsOMPart_FWD_DEFINED__ */


#ifndef __IXpsOMGlyphsEditor_FWD_DEFINED__
#define __IXpsOMGlyphsEditor_FWD_DEFINED__
typedef interface IXpsOMGlyphsEditor IXpsOMGlyphsEditor;

#endif 	/* __IXpsOMGlyphsEditor_FWD_DEFINED__ */


#ifndef __IXpsOMGlyphs_FWD_DEFINED__
#define __IXpsOMGlyphs_FWD_DEFINED__
typedef interface IXpsOMGlyphs IXpsOMGlyphs;

#endif 	/* __IXpsOMGlyphs_FWD_DEFINED__ */


#ifndef __IXpsOMDashCollection_FWD_DEFINED__
#define __IXpsOMDashCollection_FWD_DEFINED__
typedef interface IXpsOMDashCollection IXpsOMDashCollection;

#endif 	/* __IXpsOMDashCollection_FWD_DEFINED__ */


#ifndef __IXpsOMMatrixTransform_FWD_DEFINED__
#define __IXpsOMMatrixTransform_FWD_DEFINED__
typedef interface IXpsOMMatrixTransform IXpsOMMatrixTransform;

#endif 	/* __IXpsOMMatrixTransform_FWD_DEFINED__ */


#ifndef __IXpsOMGeometry_FWD_DEFINED__
#define __IXpsOMGeometry_FWD_DEFINED__
typedef interface IXpsOMGeometry IXpsOMGeometry;

#endif 	/* __IXpsOMGeometry_FWD_DEFINED__ */


#ifndef __IXpsOMGeometryFigure_FWD_DEFINED__
#define __IXpsOMGeometryFigure_FWD_DEFINED__
typedef interface IXpsOMGeometryFigure IXpsOMGeometryFigure;

#endif 	/* __IXpsOMGeometryFigure_FWD_DEFINED__ */


#ifndef __IXpsOMGeometryFigureCollection_FWD_DEFINED__
#define __IXpsOMGeometryFigureCollection_FWD_DEFINED__
typedef interface IXpsOMGeometryFigureCollection IXpsOMGeometryFigureCollection;

#endif 	/* __IXpsOMGeometryFigureCollection_FWD_DEFINED__ */


#ifndef __IXpsOMPath_FWD_DEFINED__
#define __IXpsOMPath_FWD_DEFINED__
typedef interface IXpsOMPath IXpsOMPath;

#endif 	/* __IXpsOMPath_FWD_DEFINED__ */


#ifndef __IXpsOMBrush_FWD_DEFINED__
#define __IXpsOMBrush_FWD_DEFINED__
typedef interface IXpsOMBrush IXpsOMBrush;

#endif 	/* __IXpsOMBrush_FWD_DEFINED__ */


#ifndef __IXpsOMGradientStopCollection_FWD_DEFINED__
#define __IXpsOMGradientStopCollection_FWD_DEFINED__
typedef interface IXpsOMGradientStopCollection IXpsOMGradientStopCollection;

#endif 	/* __IXpsOMGradientStopCollection_FWD_DEFINED__ */


#ifndef __IXpsOMSolidColorBrush_FWD_DEFINED__
#define __IXpsOMSolidColorBrush_FWD_DEFINED__
typedef interface IXpsOMSolidColorBrush IXpsOMSolidColorBrush;

#endif 	/* __IXpsOMSolidColorBrush_FWD_DEFINED__ */


#ifndef __IXpsOMTileBrush_FWD_DEFINED__
#define __IXpsOMTileBrush_FWD_DEFINED__
typedef interface IXpsOMTileBrush IXpsOMTileBrush;

#endif 	/* __IXpsOMTileBrush_FWD_DEFINED__ */


#ifndef __IXpsOMVisualBrush_FWD_DEFINED__
#define __IXpsOMVisualBrush_FWD_DEFINED__
typedef interface IXpsOMVisualBrush IXpsOMVisualBrush;

#endif 	/* __IXpsOMVisualBrush_FWD_DEFINED__ */


#ifndef __IXpsOMImageBrush_FWD_DEFINED__
#define __IXpsOMImageBrush_FWD_DEFINED__
typedef interface IXpsOMImageBrush IXpsOMImageBrush;

#endif 	/* __IXpsOMImageBrush_FWD_DEFINED__ */


#ifndef __IXpsOMGradientStop_FWD_DEFINED__
#define __IXpsOMGradientStop_FWD_DEFINED__
typedef interface IXpsOMGradientStop IXpsOMGradientStop;

#endif 	/* __IXpsOMGradientStop_FWD_DEFINED__ */


#ifndef __IXpsOMGradientBrush_FWD_DEFINED__
#define __IXpsOMGradientBrush_FWD_DEFINED__
typedef interface IXpsOMGradientBrush IXpsOMGradientBrush;

#endif 	/* __IXpsOMGradientBrush_FWD_DEFINED__ */


#ifndef __IXpsOMLinearGradientBrush_FWD_DEFINED__
#define __IXpsOMLinearGradientBrush_FWD_DEFINED__
typedef interface IXpsOMLinearGradientBrush IXpsOMLinearGradientBrush;

#endif 	/* __IXpsOMLinearGradientBrush_FWD_DEFINED__ */


#ifndef __IXpsOMRadialGradientBrush_FWD_DEFINED__
#define __IXpsOMRadialGradientBrush_FWD_DEFINED__
typedef interface IXpsOMRadialGradientBrush IXpsOMRadialGradientBrush;

#endif 	/* __IXpsOMRadialGradientBrush_FWD_DEFINED__ */


#ifndef __IXpsOMResource_FWD_DEFINED__
#define __IXpsOMResource_FWD_DEFINED__
typedef interface IXpsOMResource IXpsOMResource;

#endif 	/* __IXpsOMResource_FWD_DEFINED__ */


#ifndef __IXpsOMPartResources_FWD_DEFINED__
#define __IXpsOMPartResources_FWD_DEFINED__
typedef interface IXpsOMPartResources IXpsOMPartResources;

#endif 	/* __IXpsOMPartResources_FWD_DEFINED__ */


#ifndef __IXpsOMDictionary_FWD_DEFINED__
#define __IXpsOMDictionary_FWD_DEFINED__
typedef interface IXpsOMDictionary IXpsOMDictionary;

#endif 	/* __IXpsOMDictionary_FWD_DEFINED__ */


#ifndef __IXpsOMFontResource_FWD_DEFINED__
#define __IXpsOMFontResource_FWD_DEFINED__
typedef interface IXpsOMFontResource IXpsOMFontResource;

#endif 	/* __IXpsOMFontResource_FWD_DEFINED__ */


#ifndef __IXpsOMFontResourceCollection_FWD_DEFINED__
#define __IXpsOMFontResourceCollection_FWD_DEFINED__
typedef interface IXpsOMFontResourceCollection IXpsOMFontResourceCollection;

#endif 	/* __IXpsOMFontResourceCollection_FWD_DEFINED__ */


#ifndef __IXpsOMImageResource_FWD_DEFINED__
#define __IXpsOMImageResource_FWD_DEFINED__
typedef interface IXpsOMImageResource IXpsOMImageResource;

#endif 	/* __IXpsOMImageResource_FWD_DEFINED__ */


#ifndef __IXpsOMImageResourceCollection_FWD_DEFINED__
#define __IXpsOMImageResourceCollection_FWD_DEFINED__
typedef interface IXpsOMImageResourceCollection IXpsOMImageResourceCollection;

#endif 	/* __IXpsOMImageResourceCollection_FWD_DEFINED__ */


#ifndef __IXpsOMColorProfileResource_FWD_DEFINED__
#define __IXpsOMColorProfileResource_FWD_DEFINED__
typedef interface IXpsOMColorProfileResource IXpsOMColorProfileResource;

#endif 	/* __IXpsOMColorProfileResource_FWD_DEFINED__ */


#ifndef __IXpsOMColorProfileResourceCollection_FWD_DEFINED__
#define __IXpsOMColorProfileResourceCollection_FWD_DEFINED__
typedef interface IXpsOMColorProfileResourceCollection IXpsOMColorProfileResourceCollection;

#endif 	/* __IXpsOMColorProfileResourceCollection_FWD_DEFINED__ */


#ifndef __IXpsOMPrintTicketResource_FWD_DEFINED__
#define __IXpsOMPrintTicketResource_FWD_DEFINED__
typedef interface IXpsOMPrintTicketResource IXpsOMPrintTicketResource;

#endif 	/* __IXpsOMPrintTicketResource_FWD_DEFINED__ */


#ifndef __IXpsOMRemoteDictionaryResource_FWD_DEFINED__
#define __IXpsOMRemoteDictionaryResource_FWD_DEFINED__
typedef interface IXpsOMRemoteDictionaryResource IXpsOMRemoteDictionaryResource;

#endif 	/* __IXpsOMRemoteDictionaryResource_FWD_DEFINED__ */


#ifndef __IXpsOMRemoteDictionaryResourceCollection_FWD_DEFINED__
#define __IXpsOMRemoteDictionaryResourceCollection_FWD_DEFINED__
typedef interface IXpsOMRemoteDictionaryResourceCollection IXpsOMRemoteDictionaryResourceCollection;

#endif 	/* __IXpsOMRemoteDictionaryResourceCollection_FWD_DEFINED__ */


#ifndef __IXpsOMSignatureBlockResourceCollection_FWD_DEFINED__
#define __IXpsOMSignatureBlockResourceCollection_FWD_DEFINED__
typedef interface IXpsOMSignatureBlockResourceCollection IXpsOMSignatureBlockResourceCollection;

#endif 	/* __IXpsOMSignatureBlockResourceCollection_FWD_DEFINED__ */


#ifndef __IXpsOMDocumentStructureResource_FWD_DEFINED__
#define __IXpsOMDocumentStructureResource_FWD_DEFINED__
typedef interface IXpsOMDocumentStructureResource IXpsOMDocumentStructureResource;

#endif 	/* __IXpsOMDocumentStructureResource_FWD_DEFINED__ */


#ifndef __IXpsOMStoryFragmentsResource_FWD_DEFINED__
#define __IXpsOMStoryFragmentsResource_FWD_DEFINED__
typedef interface IXpsOMStoryFragmentsResource IXpsOMStoryFragmentsResource;

#endif 	/* __IXpsOMStoryFragmentsResource_FWD_DEFINED__ */


#ifndef __IXpsOMSignatureBlockResource_FWD_DEFINED__
#define __IXpsOMSignatureBlockResource_FWD_DEFINED__
typedef interface IXpsOMSignatureBlockResource IXpsOMSignatureBlockResource;

#endif 	/* __IXpsOMSignatureBlockResource_FWD_DEFINED__ */


#ifndef __IXpsOMVisualCollection_FWD_DEFINED__
#define __IXpsOMVisualCollection_FWD_DEFINED__
typedef interface IXpsOMVisualCollection IXpsOMVisualCollection;

#endif 	/* __IXpsOMVisualCollection_FWD_DEFINED__ */


#ifndef __IXpsOMCanvas_FWD_DEFINED__
#define __IXpsOMCanvas_FWD_DEFINED__
typedef interface IXpsOMCanvas IXpsOMCanvas;

#endif 	/* __IXpsOMCanvas_FWD_DEFINED__ */


#ifndef __IXpsOMPage_FWD_DEFINED__
#define __IXpsOMPage_FWD_DEFINED__
typedef interface IXpsOMPage IXpsOMPage;

#endif 	/* __IXpsOMPage_FWD_DEFINED__ */


#ifndef __IXpsOMPageReference_FWD_DEFINED__
#define __IXpsOMPageReference_FWD_DEFINED__
typedef interface IXpsOMPageReference IXpsOMPageReference;

#endif 	/* __IXpsOMPageReference_FWD_DEFINED__ */


#ifndef __IXpsOMPageReferenceCollection_FWD_DEFINED__
#define __IXpsOMPageReferenceCollection_FWD_DEFINED__
typedef interface IXpsOMPageReferenceCollection IXpsOMPageReferenceCollection;

#endif 	/* __IXpsOMPageReferenceCollection_FWD_DEFINED__ */


#ifndef __IXpsOMDocument_FWD_DEFINED__
#define __IXpsOMDocument_FWD_DEFINED__
typedef interface IXpsOMDocument IXpsOMDocument;

#endif 	/* __IXpsOMDocument_FWD_DEFINED__ */


#ifndef __IXpsOMDocumentCollection_FWD_DEFINED__
#define __IXpsOMDocumentCollection_FWD_DEFINED__
typedef interface IXpsOMDocumentCollection IXpsOMDocumentCollection;

#endif 	/* __IXpsOMDocumentCollection_FWD_DEFINED__ */


#ifndef __IXpsOMDocumentSequence_FWD_DEFINED__
#define __IXpsOMDocumentSequence_FWD_DEFINED__
typedef interface IXpsOMDocumentSequence IXpsOMDocumentSequence;

#endif 	/* __IXpsOMDocumentSequence_FWD_DEFINED__ */


#ifndef __IXpsOMCoreProperties_FWD_DEFINED__
#define __IXpsOMCoreProperties_FWD_DEFINED__
typedef interface IXpsOMCoreProperties IXpsOMCoreProperties;

#endif 	/* __IXpsOMCoreProperties_FWD_DEFINED__ */


#ifndef __IXpsOMPackage_FWD_DEFINED__
#define __IXpsOMPackage_FWD_DEFINED__
typedef interface IXpsOMPackage IXpsOMPackage;

#endif 	/* __IXpsOMPackage_FWD_DEFINED__ */


#ifndef __IXpsOMObjectFactory_FWD_DEFINED__
#define __IXpsOMObjectFactory_FWD_DEFINED__
typedef interface IXpsOMObjectFactory IXpsOMObjectFactory;

#endif 	/* __IXpsOMObjectFactory_FWD_DEFINED__ */


#ifndef __IXpsOMNameCollection_FWD_DEFINED__
#define __IXpsOMNameCollection_FWD_DEFINED__
typedef interface IXpsOMNameCollection IXpsOMNameCollection;

#endif 	/* __IXpsOMNameCollection_FWD_DEFINED__ */


#ifndef __IXpsOMPartUriCollection_FWD_DEFINED__
#define __IXpsOMPartUriCollection_FWD_DEFINED__
typedef interface IXpsOMPartUriCollection IXpsOMPartUriCollection;

#endif 	/* __IXpsOMPartUriCollection_FWD_DEFINED__ */


#ifndef __IXpsOMPackageWriter_FWD_DEFINED__
#define __IXpsOMPackageWriter_FWD_DEFINED__
typedef interface IXpsOMPackageWriter IXpsOMPackageWriter;

#endif 	/* __IXpsOMPackageWriter_FWD_DEFINED__ */


#ifndef __IXpsOMPackageTarget_FWD_DEFINED__
#define __IXpsOMPackageTarget_FWD_DEFINED__
typedef interface IXpsOMPackageTarget IXpsOMPackageTarget;

#endif 	/* __IXpsOMPackageTarget_FWD_DEFINED__ */


#ifndef __IXpsOMThumbnailGenerator_FWD_DEFINED__
#define __IXpsOMThumbnailGenerator_FWD_DEFINED__
typedef interface IXpsOMThumbnailGenerator IXpsOMThumbnailGenerator;

#endif 	/* __IXpsOMThumbnailGenerator_FWD_DEFINED__ */


#ifndef __XpsOMObjectFactory_FWD_DEFINED__
#define __XpsOMObjectFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class XpsOMObjectFactory XpsOMObjectFactory;
#else
typedef struct XpsOMObjectFactory XpsOMObjectFactory;
#endif /* __cplusplus */

#endif 	/* __XpsOMObjectFactory_FWD_DEFINED__ */


#ifndef __XpsOMThumbnailGenerator_FWD_DEFINED__
#define __XpsOMThumbnailGenerator_FWD_DEFINED__

#ifdef __cplusplus
typedef class XpsOMThumbnailGenerator XpsOMThumbnailGenerator;
#else
typedef struct XpsOMThumbnailGenerator XpsOMThumbnailGenerator;
#endif /* __cplusplus */

#endif 	/* __XpsOMThumbnailGenerator_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "msopc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_xpsobjectmodel_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
//+-------------------------------------------------------------------------
//
//  Microsoft Windows
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#if (NTDDI_VERSION >= NTDDI_WIN7)
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)














































typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0001
    {
        XPS_TILE_MODE_NONE	= 1,
        XPS_TILE_MODE_TILE	= ( XPS_TILE_MODE_NONE + 1 ) ,
        XPS_TILE_MODE_FLIPX	= ( XPS_TILE_MODE_TILE + 1 ) ,
        XPS_TILE_MODE_FLIPY	= ( XPS_TILE_MODE_FLIPX + 1 ) ,
        XPS_TILE_MODE_FLIPXY	= ( XPS_TILE_MODE_FLIPY + 1 ) 
    } 	XPS_TILE_MODE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0002
    {
        XPS_COLOR_INTERPOLATION_SCRGBLINEAR	= 1,
        XPS_COLOR_INTERPOLATION_SRGBLINEAR	= ( XPS_COLOR_INTERPOLATION_SCRGBLINEAR + 1 ) 
    } 	XPS_COLOR_INTERPOLATION;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0003
    {
        XPS_SPREAD_METHOD_PAD	= 1,
        XPS_SPREAD_METHOD_REFLECT	= ( XPS_SPREAD_METHOD_PAD + 1 ) ,
        XPS_SPREAD_METHOD_REPEAT	= ( XPS_SPREAD_METHOD_REFLECT + 1 ) 
    } 	XPS_SPREAD_METHOD;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0004
    {
        XPS_STYLE_SIMULATION_NONE	= 1,
        XPS_STYLE_SIMULATION_ITALIC	= ( XPS_STYLE_SIMULATION_NONE + 1 ) ,
        XPS_STYLE_SIMULATION_BOLD	= ( XPS_STYLE_SIMULATION_ITALIC + 1 ) ,
        XPS_STYLE_SIMULATION_BOLDITALIC	= ( XPS_STYLE_SIMULATION_BOLD + 1 ) 
    } 	XPS_STYLE_SIMULATION;

typedef /* [public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0005
    {
        XPS_LINE_CAP_FLAT	= 1,
        XPS_LINE_CAP_ROUND	= ( XPS_LINE_CAP_FLAT + 1 ) ,
        XPS_LINE_CAP_SQUARE	= ( XPS_LINE_CAP_ROUND + 1 ) ,
        XPS_LINE_CAP_TRIANGLE	= ( XPS_LINE_CAP_SQUARE + 1 ) 
    } 	XPS_LINE_CAP;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0006
    {
        XPS_DASH_CAP_FLAT	= 1,
        XPS_DASH_CAP_ROUND	= ( XPS_DASH_CAP_FLAT + 1 ) ,
        XPS_DASH_CAP_SQUARE	= ( XPS_DASH_CAP_ROUND + 1 ) ,
        XPS_DASH_CAP_TRIANGLE	= ( XPS_DASH_CAP_SQUARE + 1 ) 
    } 	XPS_DASH_CAP;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0007
    {
        XPS_LINE_JOIN_MITER	= 1,
        XPS_LINE_JOIN_BEVEL	= ( XPS_LINE_JOIN_MITER + 1 ) ,
        XPS_LINE_JOIN_ROUND	= ( XPS_LINE_JOIN_BEVEL + 1 ) 
    } 	XPS_LINE_JOIN;

typedef /* [public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0008
    {
        XPS_IMAGE_TYPE_JPEG	= 1,
        XPS_IMAGE_TYPE_PNG	= ( XPS_IMAGE_TYPE_JPEG + 1 ) ,
        XPS_IMAGE_TYPE_TIFF	= ( XPS_IMAGE_TYPE_PNG + 1 ) ,
        XPS_IMAGE_TYPE_WDP	= ( XPS_IMAGE_TYPE_TIFF + 1 ) ,
        XPS_IMAGE_TYPE_JXR	= ( XPS_IMAGE_TYPE_WDP + 1 ) 
    } 	XPS_IMAGE_TYPE;

typedef /* [public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0009
    {
        XPS_COLOR_TYPE_SRGB	= 1,
        XPS_COLOR_TYPE_SCRGB	= ( XPS_COLOR_TYPE_SRGB + 1 ) ,
        XPS_COLOR_TYPE_CONTEXT	= ( XPS_COLOR_TYPE_SCRGB + 1 ) 
    } 	XPS_COLOR_TYPE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0010
    {
        XPS_FILL_RULE_EVENODD	= 1,
        XPS_FILL_RULE_NONZERO	= ( XPS_FILL_RULE_EVENODD + 1 ) 
    } 	XPS_FILL_RULE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0011
    {
        XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE	= 1,
        XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE	= ( XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE + 1 ) ,
        XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE	= ( XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE + 1 ) ,
        XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE	= ( XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE + 1 ) ,
        XPS_SEGMENT_TYPE_BEZIER	= ( XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE + 1 ) ,
        XPS_SEGMENT_TYPE_LINE	= ( XPS_SEGMENT_TYPE_BEZIER + 1 ) ,
        XPS_SEGMENT_TYPE_QUADRATIC_BEZIER	= ( XPS_SEGMENT_TYPE_LINE + 1 ) 
    } 	XPS_SEGMENT_TYPE;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0012
    {
        XPS_SEGMENT_STROKE_PATTERN_ALL	= 1,
        XPS_SEGMENT_STROKE_PATTERN_NONE	= ( XPS_SEGMENT_STROKE_PATTERN_ALL + 1 ) ,
        XPS_SEGMENT_STROKE_PATTERN_MIXED	= ( XPS_SEGMENT_STROKE_PATTERN_NONE + 1 ) 
    } 	XPS_SEGMENT_STROKE_PATTERN;

typedef /* [public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0013
    {
        XPS_FONT_EMBEDDING_NORMAL	= 1,
        XPS_FONT_EMBEDDING_OBFUSCATED	= ( XPS_FONT_EMBEDDING_NORMAL + 1 ) ,
        XPS_FONT_EMBEDDING_RESTRICTED	= ( XPS_FONT_EMBEDDING_OBFUSCATED + 1 ) ,
        XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED	= ( XPS_FONT_EMBEDDING_RESTRICTED + 1 ) 
    } 	XPS_FONT_EMBEDDING;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0014
    {
        XPS_OBJECT_TYPE_CANVAS	= 1,
        XPS_OBJECT_TYPE_GLYPHS	= ( XPS_OBJECT_TYPE_CANVAS + 1 ) ,
        XPS_OBJECT_TYPE_PATH	= ( XPS_OBJECT_TYPE_GLYPHS + 1 ) ,
        XPS_OBJECT_TYPE_MATRIX_TRANSFORM	= ( XPS_OBJECT_TYPE_PATH + 1 ) ,
        XPS_OBJECT_TYPE_GEOMETRY	= ( XPS_OBJECT_TYPE_MATRIX_TRANSFORM + 1 ) ,
        XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH	= ( XPS_OBJECT_TYPE_GEOMETRY + 1 ) ,
        XPS_OBJECT_TYPE_IMAGE_BRUSH	= ( XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH + 1 ) ,
        XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH	= ( XPS_OBJECT_TYPE_IMAGE_BRUSH + 1 ) ,
        XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH	= ( XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH + 1 ) ,
        XPS_OBJECT_TYPE_VISUAL_BRUSH	= ( XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH + 1 ) 
    } 	XPS_OBJECT_TYPE;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0015
    {
        XPS_THUMBNAIL_SIZE_VERYSMALL	= 1,
        XPS_THUMBNAIL_SIZE_SMALL	= ( XPS_THUMBNAIL_SIZE_VERYSMALL + 1 ) ,
        XPS_THUMBNAIL_SIZE_MEDIUM	= ( XPS_THUMBNAIL_SIZE_SMALL + 1 ) ,
        XPS_THUMBNAIL_SIZE_LARGE	= ( XPS_THUMBNAIL_SIZE_MEDIUM + 1 ) 
    } 	XPS_THUMBNAIL_SIZE;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0016
    {
        XPS_INTERLEAVING_OFF	= 1,
        XPS_INTERLEAVING_ON	= ( XPS_INTERLEAVING_OFF + 1 ) 
    } 	XPS_INTERLEAVING;

typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0017
    {
    FLOAT x;
    FLOAT y;
    } 	XPS_POINT;

typedef /* [public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0018
    {
    FLOAT width;
    FLOAT height;
    } 	XPS_SIZE;

typedef /* [public][public][public][public][public][public][public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0019
    {
    FLOAT x;
    FLOAT y;
    FLOAT width;
    FLOAT height;
    } 	XPS_RECT;

typedef /* [public][public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0020
    {
    FLOAT length;
    FLOAT gap;
    } 	XPS_DASH;

typedef /* [public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0021
    {
    LONG index;
    FLOAT advanceWidth;
    FLOAT horizontalOffset;
    FLOAT verticalOffset;
    } 	XPS_GLYPH_INDEX;

typedef /* [public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0022
    {
    UINT32 unicodeStringStart;
    UINT16 unicodeStringLength;
    UINT32 glyphIndicesStart;
    UINT16 glyphIndicesLength;
    } 	XPS_GLYPH_MAPPING;

typedef /* [public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0023
    {
    FLOAT m11;
    FLOAT m12;
    FLOAT m21;
    FLOAT m22;
    FLOAT m31;
    FLOAT m32;
    } 	XPS_MATRIX;

typedef /* [public][public][public][public][public][public][public] */ struct __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0024
    {
    XPS_COLOR_TYPE colorType;
    /* [switch_is] */ /* [switch_type] */ union __MIDL___MIDL_itf_xpsobjectmodel_0000_0000_0028
        {
        /* [case()] */ struct 
            {
            UINT8 alpha;
            UINT8 red;
            UINT8 green;
            UINT8 blue;
            } 	sRGB;
        /* [case()] */ struct 
            {
            FLOAT alpha;
            FLOAT red;
            FLOAT green;
            FLOAT blue;
            } 	scRGB;
        /* [case()] */ struct 
            {
            UINT8 channelCount;
            FLOAT channels[ 9 ];
            } 	context;
        } 	value;
    } 	XPS_COLOR;



extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0000_v0_0_s_ifspec;

#ifndef __IXpsOMShareable_INTERFACE_DEFINED__
#define __IXpsOMShareable_INTERFACE_DEFINED__

/* interface IXpsOMShareable */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMShareable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7137398F-2FC1-454D-8C6A-2C3115A16ECE")
    IXpsOMShareable : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMShareableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMShareable * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMShareable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMShareable * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMShareable * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMShareable * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        END_INTERFACE
    } IXpsOMShareableVtbl;

    interface IXpsOMShareable
    {
        CONST_VTBL struct IXpsOMShareableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMShareable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMShareable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMShareable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMShareable_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMShareable_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMShareable_INTERFACE_DEFINED__ */


#ifndef __IXpsOMVisual_INTERFACE_DEFINED__
#define __IXpsOMVisual_INTERFACE_DEFINED__

/* interface IXpsOMVisual */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMVisual;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BC3E7333-FB0B-4af3-A819-0B4EAAD0D2FD")
    IXpsOMVisual : public IXpsOMShareable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTransform( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLocal( 
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *matrixTransform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClipGeometry( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClipGeometryLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClipGeometryLocal( 
            /* [in] */ __RPC__in_opt IXpsOMGeometry *clipGeometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClipGeometryLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetClipGeometryLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOpacity( 
            /* [retval][out] */ __RPC__out FLOAT *opacity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOpacity( 
            /* [in] */ FLOAT opacity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOpacityMaskBrush( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOpacityMaskBrushLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOpacityMaskBrushLocal( 
            /* [in] */ __RPC__in_opt IXpsOMBrush *opacityMaskBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOpacityMaskBrushLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOpacityMaskBrushLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [string][in] */ __RPC__in_string LPCWSTR name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsHyperlinkTarget( 
            /* [retval][out] */ __RPC__out BOOL *isHyperlink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIsHyperlinkTarget( 
            /* [in] */ BOOL isHyperlink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHyperlinkNavigateUri( 
            /* [retval][out] */ __RPC__deref_out_opt IUri **hyperlinkUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHyperlinkNavigateUri( 
            /* [in] */ __RPC__in_opt IUri *hyperlinkUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguage( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLanguage( 
            /* [string][in] */ __RPC__in_string LPCWSTR language) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMVisualVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMVisual * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMVisual * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMVisual * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometry)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometry )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLocal )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLocal )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ __RPC__in_opt IXpsOMGeometry *clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLookup )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLookup )( 
            __RPC__in IXpsOMVisual * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrush)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrush )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMVisual * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IXpsOMVisual * This,
            /* [string][in] */ __RPC__in_string LPCWSTR name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *GetIsHyperlinkTarget )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__out BOOL *isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *SetIsHyperlinkTarget )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ BOOL isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *GetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][out] */ __RPC__deref_out_opt IUri **hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *SetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMVisual * This,
            /* [in] */ __RPC__in_opt IUri *hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IXpsOMVisual * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IXpsOMVisual * This,
            /* [string][in] */ __RPC__in_string LPCWSTR language);
        
        END_INTERFACE
    } IXpsOMVisualVtbl;

    interface IXpsOMVisual
    {
        CONST_VTBL struct IXpsOMVisualVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMVisual_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMVisual_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMVisual_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMVisual_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMVisual_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMVisual_GetTransform(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransform(This,matrixTransform) ) 

#define IXpsOMVisual_GetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,matrixTransform) ) 

#define IXpsOMVisual_SetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,matrixTransform) ) 

#define IXpsOMVisual_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMVisual_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMVisual_GetClipGeometry(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometry(This,clipGeometry) ) 

#define IXpsOMVisual_GetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMVisual_SetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> SetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMVisual_GetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> GetClipGeometryLookup(This,key) ) 

#define IXpsOMVisual_SetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> SetClipGeometryLookup(This,key) ) 

#define IXpsOMVisual_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMVisual_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 

#define IXpsOMVisual_GetOpacityMaskBrush(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrush(This,opacityMaskBrush) ) 

#define IXpsOMVisual_GetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMVisual_SetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMVisual_GetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMVisual_SetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMVisual_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IXpsOMVisual_SetName(This,name)	\
    ( (This)->lpVtbl -> SetName(This,name) ) 

#define IXpsOMVisual_GetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> GetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMVisual_SetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> SetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMVisual_GetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> GetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMVisual_SetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> SetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMVisual_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMVisual_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMVisual_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPart_INTERFACE_DEFINED__
#define __IXpsOMPart_INTERFACE_DEFINED__

/* interface IXpsOMPart */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPart;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("74eb2f0b-a91e-4486-afac-0fabeca3dfc6")
    IXpsOMPart : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPartName( 
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPartVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPart * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPart * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPart * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMPart * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMPart * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        END_INTERFACE
    } IXpsOMPartVtbl;

    interface IXpsOMPart
    {
        CONST_VTBL struct IXpsOMPartVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPart_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPart_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPart_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPart_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMPart_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPart_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGlyphsEditor_INTERFACE_DEFINED__
#define __IXpsOMGlyphsEditor_INTERFACE_DEFINED__

/* interface IXpsOMGlyphsEditor */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGlyphsEditor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A5AB8616-5B16-4B9F-9629-89B323ED7909")
    IXpsOMGlyphsEditor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ApplyEdits( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUnicodeString( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *unicodeString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUnicodeString( 
            /* [string][in] */ __RPC__in_string LPCWSTR unicodeString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphIndexCount( 
            /* [retval][out] */ __RPC__out UINT32 *indexCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphIndices( 
            /* [out][in] */ __RPC__inout UINT32 *indexCount,
            /* [out] */ __RPC__out XPS_GLYPH_INDEX *glyphIndices) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGlyphIndices( 
            /* [in] */ UINT32 indexCount,
            /* [in] */ __RPC__in const XPS_GLYPH_INDEX *glyphIndices) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphMappingCount( 
            /* [retval][out] */ __RPC__out UINT32 *glyphMappingCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphMappings( 
            /* [out][in] */ __RPC__inout UINT32 *glyphMappingCount,
            /* [out] */ __RPC__out XPS_GLYPH_MAPPING *glyphMappings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGlyphMappings( 
            /* [in] */ UINT32 glyphMappingCount,
            /* [in] */ __RPC__in const XPS_GLYPH_MAPPING *glyphMappings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProhibitedCaretStopCount( 
            /* [retval][out] */ __RPC__out UINT32 *prohibitedCaretStopCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProhibitedCaretStops( 
            /* [out][in] */ __RPC__inout UINT32 *count,
            /* [out] */ __RPC__out UINT32 *prohibitedCaretStops) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProhibitedCaretStops( 
            /* [in] */ UINT32 count,
            /* [in] */ __RPC__in const UINT32 *prohibitedCaretStops) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBidiLevel( 
            /* [retval][out] */ __RPC__out UINT32 *bidiLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBidiLevel( 
            /* [in] */ UINT32 bidiLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsSideways( 
            /* [retval][out] */ __RPC__out BOOL *isSideways) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIsSideways( 
            /* [in] */ BOOL isSideways) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceFontName( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *deviceFontName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDeviceFontName( 
            /* [string][in] */ __RPC__in_string LPCWSTR deviceFontName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGlyphsEditorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGlyphsEditor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGlyphsEditor * This);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, ApplyEdits)
        HRESULT ( STDMETHODCALLTYPE *ApplyEdits )( 
            __RPC__in IXpsOMGlyphsEditor * This);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetUnicodeString)
        HRESULT ( STDMETHODCALLTYPE *GetUnicodeString )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *unicodeString);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetUnicodeString)
        HRESULT ( STDMETHODCALLTYPE *SetUnicodeString )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [string][in] */ __RPC__in_string LPCWSTR unicodeString);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetGlyphIndexCount)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphIndexCount )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][out] */ __RPC__out UINT32 *indexCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetGlyphIndices)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphIndices )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [out][in] */ __RPC__inout UINT32 *indexCount,
            /* [out] */ __RPC__out XPS_GLYPH_INDEX *glyphIndices);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetGlyphIndices)
        HRESULT ( STDMETHODCALLTYPE *SetGlyphIndices )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [in] */ UINT32 indexCount,
            /* [in] */ __RPC__in const XPS_GLYPH_INDEX *glyphIndices);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetGlyphMappingCount)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphMappingCount )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][out] */ __RPC__out UINT32 *glyphMappingCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetGlyphMappings)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphMappings )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [out][in] */ __RPC__inout UINT32 *glyphMappingCount,
            /* [out] */ __RPC__out XPS_GLYPH_MAPPING *glyphMappings);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetGlyphMappings)
        HRESULT ( STDMETHODCALLTYPE *SetGlyphMappings )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [in] */ UINT32 glyphMappingCount,
            /* [in] */ __RPC__in const XPS_GLYPH_MAPPING *glyphMappings);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetProhibitedCaretStopCount)
        HRESULT ( STDMETHODCALLTYPE *GetProhibitedCaretStopCount )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][out] */ __RPC__out UINT32 *prohibitedCaretStopCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetProhibitedCaretStops)
        HRESULT ( STDMETHODCALLTYPE *GetProhibitedCaretStops )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [out][in] */ __RPC__inout UINT32 *count,
            /* [out] */ __RPC__out UINT32 *prohibitedCaretStops);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetProhibitedCaretStops)
        HRESULT ( STDMETHODCALLTYPE *SetProhibitedCaretStops )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [in] */ UINT32 count,
            /* [in] */ __RPC__in const UINT32 *prohibitedCaretStops);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetBidiLevel)
        HRESULT ( STDMETHODCALLTYPE *GetBidiLevel )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][out] */ __RPC__out UINT32 *bidiLevel);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetBidiLevel)
        HRESULT ( STDMETHODCALLTYPE *SetBidiLevel )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [in] */ UINT32 bidiLevel);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetIsSideways)
        HRESULT ( STDMETHODCALLTYPE *GetIsSideways )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][out] */ __RPC__out BOOL *isSideways);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetIsSideways)
        HRESULT ( STDMETHODCALLTYPE *SetIsSideways )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [in] */ BOOL isSideways);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, GetDeviceFontName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceFontName )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *deviceFontName);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphsEditor, SetDeviceFontName)
        HRESULT ( STDMETHODCALLTYPE *SetDeviceFontName )( 
            __RPC__in IXpsOMGlyphsEditor * This,
            /* [string][in] */ __RPC__in_string LPCWSTR deviceFontName);
        
        END_INTERFACE
    } IXpsOMGlyphsEditorVtbl;

    interface IXpsOMGlyphsEditor
    {
        CONST_VTBL struct IXpsOMGlyphsEditorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGlyphsEditor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGlyphsEditor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGlyphsEditor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGlyphsEditor_ApplyEdits(This)	\
    ( (This)->lpVtbl -> ApplyEdits(This) ) 

#define IXpsOMGlyphsEditor_GetUnicodeString(This,unicodeString)	\
    ( (This)->lpVtbl -> GetUnicodeString(This,unicodeString) ) 

#define IXpsOMGlyphsEditor_SetUnicodeString(This,unicodeString)	\
    ( (This)->lpVtbl -> SetUnicodeString(This,unicodeString) ) 

#define IXpsOMGlyphsEditor_GetGlyphIndexCount(This,indexCount)	\
    ( (This)->lpVtbl -> GetGlyphIndexCount(This,indexCount) ) 

#define IXpsOMGlyphsEditor_GetGlyphIndices(This,indexCount,glyphIndices)	\
    ( (This)->lpVtbl -> GetGlyphIndices(This,indexCount,glyphIndices) ) 

#define IXpsOMGlyphsEditor_SetGlyphIndices(This,indexCount,glyphIndices)	\
    ( (This)->lpVtbl -> SetGlyphIndices(This,indexCount,glyphIndices) ) 

#define IXpsOMGlyphsEditor_GetGlyphMappingCount(This,glyphMappingCount)	\
    ( (This)->lpVtbl -> GetGlyphMappingCount(This,glyphMappingCount) ) 

#define IXpsOMGlyphsEditor_GetGlyphMappings(This,glyphMappingCount,glyphMappings)	\
    ( (This)->lpVtbl -> GetGlyphMappings(This,glyphMappingCount,glyphMappings) ) 

#define IXpsOMGlyphsEditor_SetGlyphMappings(This,glyphMappingCount,glyphMappings)	\
    ( (This)->lpVtbl -> SetGlyphMappings(This,glyphMappingCount,glyphMappings) ) 

#define IXpsOMGlyphsEditor_GetProhibitedCaretStopCount(This,prohibitedCaretStopCount)	\
    ( (This)->lpVtbl -> GetProhibitedCaretStopCount(This,prohibitedCaretStopCount) ) 

#define IXpsOMGlyphsEditor_GetProhibitedCaretStops(This,count,prohibitedCaretStops)	\
    ( (This)->lpVtbl -> GetProhibitedCaretStops(This,count,prohibitedCaretStops) ) 

#define IXpsOMGlyphsEditor_SetProhibitedCaretStops(This,count,prohibitedCaretStops)	\
    ( (This)->lpVtbl -> SetProhibitedCaretStops(This,count,prohibitedCaretStops) ) 

#define IXpsOMGlyphsEditor_GetBidiLevel(This,bidiLevel)	\
    ( (This)->lpVtbl -> GetBidiLevel(This,bidiLevel) ) 

#define IXpsOMGlyphsEditor_SetBidiLevel(This,bidiLevel)	\
    ( (This)->lpVtbl -> SetBidiLevel(This,bidiLevel) ) 

#define IXpsOMGlyphsEditor_GetIsSideways(This,isSideways)	\
    ( (This)->lpVtbl -> GetIsSideways(This,isSideways) ) 

#define IXpsOMGlyphsEditor_SetIsSideways(This,isSideways)	\
    ( (This)->lpVtbl -> SetIsSideways(This,isSideways) ) 

#define IXpsOMGlyphsEditor_GetDeviceFontName(This,deviceFontName)	\
    ( (This)->lpVtbl -> GetDeviceFontName(This,deviceFontName) ) 

#define IXpsOMGlyphsEditor_SetDeviceFontName(This,deviceFontName)	\
    ( (This)->lpVtbl -> SetDeviceFontName(This,deviceFontName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGlyphsEditor_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGlyphs_INTERFACE_DEFINED__
#define __IXpsOMGlyphs_INTERFACE_DEFINED__

/* interface IXpsOMGlyphs */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGlyphs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("819B3199-0A5A-4B64-BEC7-A9E17E780DE2")
    IXpsOMGlyphs : public IXpsOMVisual
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUnicodeString( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *unicodeString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphIndexCount( 
            /* [retval][out] */ __RPC__out UINT32 *indexCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphIndices( 
            /* [out][in] */ __RPC__inout UINT32 *indexCount,
            /* [out][in] */ __RPC__inout XPS_GLYPH_INDEX *glyphIndices) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphMappingCount( 
            /* [retval][out] */ __RPC__out UINT32 *glyphMappingCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphMappings( 
            /* [out][in] */ __RPC__inout UINT32 *glyphMappingCount,
            /* [out][in] */ __RPC__inout XPS_GLYPH_MAPPING *glyphMappings) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProhibitedCaretStopCount( 
            /* [retval][out] */ __RPC__out UINT32 *prohibitedCaretStopCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProhibitedCaretStops( 
            /* [out][in] */ __RPC__inout UINT32 *prohibitedCaretStopCount,
            /* [out] */ __RPC__out UINT32 *prohibitedCaretStops) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBidiLevel( 
            /* [retval][out] */ __RPC__out UINT32 *bidiLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsSideways( 
            /* [retval][out] */ __RPC__out BOOL *isSideways) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceFontName( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *deviceFontName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStyleSimulations( 
            /* [retval][out] */ __RPC__out XPS_STYLE_SIMULATION *styleSimulations) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStyleSimulations( 
            /* [in] */ XPS_STYLE_SIMULATION styleSimulations) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOrigin( 
            /* [retval][out] */ __RPC__out XPS_POINT *origin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOrigin( 
            /* [in] */ __RPC__in const XPS_POINT *origin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontRenderingEmSize( 
            /* [retval][out] */ __RPC__out FLOAT *fontRenderingEmSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFontRenderingEmSize( 
            /* [in] */ FLOAT fontRenderingEmSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResource **fontResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFontResource( 
            /* [in] */ __RPC__in_opt IXpsOMFontResource *fontResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontFaceIndex( 
            /* [retval][out] */ __RPC__out SHORT *fontFaceIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFontFaceIndex( 
            /* [in] */ SHORT fontFaceIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillBrush( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **fillBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillBrushLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **fillBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFillBrushLocal( 
            /* [in] */ __RPC__in_opt IXpsOMBrush *fillBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillBrushLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFillBrushLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGlyphsEditor( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGlyphsEditor **editor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGlyphs **glyphs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGlyphsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGlyphs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGlyphs * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometry)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometry )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in_opt IXpsOMGeometry *clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrush)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrush )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [string][in] */ __RPC__in_string LPCWSTR name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *GetIsHyperlinkTarget )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out BOOL *isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *SetIsHyperlinkTarget )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ BOOL isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *GetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IUri **hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *SetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in_opt IUri *hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [string][in] */ __RPC__in_string LPCWSTR language);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetUnicodeString)
        HRESULT ( STDMETHODCALLTYPE *GetUnicodeString )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *unicodeString);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetGlyphIndexCount)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphIndexCount )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out UINT32 *indexCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetGlyphIndices)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphIndices )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [out][in] */ __RPC__inout UINT32 *indexCount,
            /* [out][in] */ __RPC__inout XPS_GLYPH_INDEX *glyphIndices);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetGlyphMappingCount)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphMappingCount )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out UINT32 *glyphMappingCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetGlyphMappings)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphMappings )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [out][in] */ __RPC__inout UINT32 *glyphMappingCount,
            /* [out][in] */ __RPC__inout XPS_GLYPH_MAPPING *glyphMappings);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetProhibitedCaretStopCount)
        HRESULT ( STDMETHODCALLTYPE *GetProhibitedCaretStopCount )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out UINT32 *prohibitedCaretStopCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetProhibitedCaretStops)
        HRESULT ( STDMETHODCALLTYPE *GetProhibitedCaretStops )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [out][in] */ __RPC__inout UINT32 *prohibitedCaretStopCount,
            /* [out] */ __RPC__out UINT32 *prohibitedCaretStops);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetBidiLevel)
        HRESULT ( STDMETHODCALLTYPE *GetBidiLevel )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out UINT32 *bidiLevel);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetIsSideways)
        HRESULT ( STDMETHODCALLTYPE *GetIsSideways )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out BOOL *isSideways);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetDeviceFontName)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceFontName )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *deviceFontName);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetStyleSimulations)
        HRESULT ( STDMETHODCALLTYPE *GetStyleSimulations )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out XPS_STYLE_SIMULATION *styleSimulations);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetStyleSimulations)
        HRESULT ( STDMETHODCALLTYPE *SetStyleSimulations )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ XPS_STYLE_SIMULATION styleSimulations);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetOrigin )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out XPS_POINT *origin);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetOrigin)
        HRESULT ( STDMETHODCALLTYPE *SetOrigin )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in const XPS_POINT *origin);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetFontRenderingEmSize)
        HRESULT ( STDMETHODCALLTYPE *GetFontRenderingEmSize )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out FLOAT *fontRenderingEmSize);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetFontRenderingEmSize)
        HRESULT ( STDMETHODCALLTYPE *SetFontRenderingEmSize )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ FLOAT fontRenderingEmSize);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetFontResource)
        HRESULT ( STDMETHODCALLTYPE *GetFontResource )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResource **fontResource);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetFontResource)
        HRESULT ( STDMETHODCALLTYPE *SetFontResource )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in_opt IXpsOMFontResource *fontResource);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetFontFaceIndex)
        HRESULT ( STDMETHODCALLTYPE *GetFontFaceIndex )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__out SHORT *fontFaceIndex);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetFontFaceIndex)
        HRESULT ( STDMETHODCALLTYPE *SetFontFaceIndex )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ SHORT fontFaceIndex);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetFillBrush)
        HRESULT ( STDMETHODCALLTYPE *GetFillBrush )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **fillBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetFillBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetFillBrushLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **fillBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetFillBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetFillBrushLocal )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *fillBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetFillBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetFillBrushLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, SetFillBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetFillBrushLookup )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, GetGlyphsEditor)
        HRESULT ( STDMETHODCALLTYPE *GetGlyphsEditor )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGlyphsEditor **editor);
        
        DECLSPEC_XFGVIRT(IXpsOMGlyphs, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMGlyphs * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGlyphs **glyphs);
        
        END_INTERFACE
    } IXpsOMGlyphsVtbl;

    interface IXpsOMGlyphs
    {
        CONST_VTBL struct IXpsOMGlyphsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGlyphs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGlyphs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGlyphs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGlyphs_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMGlyphs_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMGlyphs_GetTransform(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransform(This,matrixTransform) ) 

#define IXpsOMGlyphs_GetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,matrixTransform) ) 

#define IXpsOMGlyphs_SetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,matrixTransform) ) 

#define IXpsOMGlyphs_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMGlyphs_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMGlyphs_GetClipGeometry(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometry(This,clipGeometry) ) 

#define IXpsOMGlyphs_GetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMGlyphs_SetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> SetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMGlyphs_GetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> GetClipGeometryLookup(This,key) ) 

#define IXpsOMGlyphs_SetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> SetClipGeometryLookup(This,key) ) 

#define IXpsOMGlyphs_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMGlyphs_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 

#define IXpsOMGlyphs_GetOpacityMaskBrush(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrush(This,opacityMaskBrush) ) 

#define IXpsOMGlyphs_GetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMGlyphs_SetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMGlyphs_GetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMGlyphs_SetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMGlyphs_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IXpsOMGlyphs_SetName(This,name)	\
    ( (This)->lpVtbl -> SetName(This,name) ) 

#define IXpsOMGlyphs_GetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> GetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMGlyphs_SetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> SetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMGlyphs_GetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> GetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMGlyphs_SetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> SetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMGlyphs_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMGlyphs_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 


#define IXpsOMGlyphs_GetUnicodeString(This,unicodeString)	\
    ( (This)->lpVtbl -> GetUnicodeString(This,unicodeString) ) 

#define IXpsOMGlyphs_GetGlyphIndexCount(This,indexCount)	\
    ( (This)->lpVtbl -> GetGlyphIndexCount(This,indexCount) ) 

#define IXpsOMGlyphs_GetGlyphIndices(This,indexCount,glyphIndices)	\
    ( (This)->lpVtbl -> GetGlyphIndices(This,indexCount,glyphIndices) ) 

#define IXpsOMGlyphs_GetGlyphMappingCount(This,glyphMappingCount)	\
    ( (This)->lpVtbl -> GetGlyphMappingCount(This,glyphMappingCount) ) 

#define IXpsOMGlyphs_GetGlyphMappings(This,glyphMappingCount,glyphMappings)	\
    ( (This)->lpVtbl -> GetGlyphMappings(This,glyphMappingCount,glyphMappings) ) 

#define IXpsOMGlyphs_GetProhibitedCaretStopCount(This,prohibitedCaretStopCount)	\
    ( (This)->lpVtbl -> GetProhibitedCaretStopCount(This,prohibitedCaretStopCount) ) 

#define IXpsOMGlyphs_GetProhibitedCaretStops(This,prohibitedCaretStopCount,prohibitedCaretStops)	\
    ( (This)->lpVtbl -> GetProhibitedCaretStops(This,prohibitedCaretStopCount,prohibitedCaretStops) ) 

#define IXpsOMGlyphs_GetBidiLevel(This,bidiLevel)	\
    ( (This)->lpVtbl -> GetBidiLevel(This,bidiLevel) ) 

#define IXpsOMGlyphs_GetIsSideways(This,isSideways)	\
    ( (This)->lpVtbl -> GetIsSideways(This,isSideways) ) 

#define IXpsOMGlyphs_GetDeviceFontName(This,deviceFontName)	\
    ( (This)->lpVtbl -> GetDeviceFontName(This,deviceFontName) ) 

#define IXpsOMGlyphs_GetStyleSimulations(This,styleSimulations)	\
    ( (This)->lpVtbl -> GetStyleSimulations(This,styleSimulations) ) 

#define IXpsOMGlyphs_SetStyleSimulations(This,styleSimulations)	\
    ( (This)->lpVtbl -> SetStyleSimulations(This,styleSimulations) ) 

#define IXpsOMGlyphs_GetOrigin(This,origin)	\
    ( (This)->lpVtbl -> GetOrigin(This,origin) ) 

#define IXpsOMGlyphs_SetOrigin(This,origin)	\
    ( (This)->lpVtbl -> SetOrigin(This,origin) ) 

#define IXpsOMGlyphs_GetFontRenderingEmSize(This,fontRenderingEmSize)	\
    ( (This)->lpVtbl -> GetFontRenderingEmSize(This,fontRenderingEmSize) ) 

#define IXpsOMGlyphs_SetFontRenderingEmSize(This,fontRenderingEmSize)	\
    ( (This)->lpVtbl -> SetFontRenderingEmSize(This,fontRenderingEmSize) ) 

#define IXpsOMGlyphs_GetFontResource(This,fontResource)	\
    ( (This)->lpVtbl -> GetFontResource(This,fontResource) ) 

#define IXpsOMGlyphs_SetFontResource(This,fontResource)	\
    ( (This)->lpVtbl -> SetFontResource(This,fontResource) ) 

#define IXpsOMGlyphs_GetFontFaceIndex(This,fontFaceIndex)	\
    ( (This)->lpVtbl -> GetFontFaceIndex(This,fontFaceIndex) ) 

#define IXpsOMGlyphs_SetFontFaceIndex(This,fontFaceIndex)	\
    ( (This)->lpVtbl -> SetFontFaceIndex(This,fontFaceIndex) ) 

#define IXpsOMGlyphs_GetFillBrush(This,fillBrush)	\
    ( (This)->lpVtbl -> GetFillBrush(This,fillBrush) ) 

#define IXpsOMGlyphs_GetFillBrushLocal(This,fillBrush)	\
    ( (This)->lpVtbl -> GetFillBrushLocal(This,fillBrush) ) 

#define IXpsOMGlyphs_SetFillBrushLocal(This,fillBrush)	\
    ( (This)->lpVtbl -> SetFillBrushLocal(This,fillBrush) ) 

#define IXpsOMGlyphs_GetFillBrushLookup(This,key)	\
    ( (This)->lpVtbl -> GetFillBrushLookup(This,key) ) 

#define IXpsOMGlyphs_SetFillBrushLookup(This,key)	\
    ( (This)->lpVtbl -> SetFillBrushLookup(This,key) ) 

#define IXpsOMGlyphs_GetGlyphsEditor(This,editor)	\
    ( (This)->lpVtbl -> GetGlyphsEditor(This,editor) ) 

#define IXpsOMGlyphs_Clone(This,glyphs)	\
    ( (This)->lpVtbl -> Clone(This,glyphs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGlyphs_INTERFACE_DEFINED__ */


#ifndef __IXpsOMDashCollection_INTERFACE_DEFINED__
#define __IXpsOMDashCollection_INTERFACE_DEFINED__

/* interface IXpsOMDashCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMDashCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("081613F4-74EB-48F2-83B3-37A9CE2D7DC6")
    IXpsOMDashCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__out XPS_DASH *dash) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in const XPS_DASH *dash) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in const XPS_DASH *dash) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in const XPS_DASH *dash) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMDashCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMDashCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMDashCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMDashCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMDashCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__out XPS_DASH *dash);
        
        DECLSPEC_XFGVIRT(IXpsOMDashCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in const XPS_DASH *dash);
        
        DECLSPEC_XFGVIRT(IXpsOMDashCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMDashCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in const XPS_DASH *dash);
        
        DECLSPEC_XFGVIRT(IXpsOMDashCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMDashCollection * This,
            /* [in] */ __RPC__in const XPS_DASH *dash);
        
        END_INTERFACE
    } IXpsOMDashCollectionVtbl;

    interface IXpsOMDashCollection
    {
        CONST_VTBL struct IXpsOMDashCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMDashCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMDashCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMDashCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMDashCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMDashCollection_GetAt(This,index,dash)	\
    ( (This)->lpVtbl -> GetAt(This,index,dash) ) 

#define IXpsOMDashCollection_InsertAt(This,index,dash)	\
    ( (This)->lpVtbl -> InsertAt(This,index,dash) ) 

#define IXpsOMDashCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMDashCollection_SetAt(This,index,dash)	\
    ( (This)->lpVtbl -> SetAt(This,index,dash) ) 

#define IXpsOMDashCollection_Append(This,dash)	\
    ( (This)->lpVtbl -> Append(This,dash) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMDashCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMMatrixTransform_INTERFACE_DEFINED__
#define __IXpsOMMatrixTransform_INTERFACE_DEFINED__

/* interface IXpsOMMatrixTransform */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMMatrixTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B77330FF-BB37-4501-A93E-F1B1E50BFC46")
    IXpsOMMatrixTransform : public IXpsOMShareable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMatrix( 
            /* [retval][out] */ __RPC__out XPS_MATRIX *matrix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMatrix( 
            /* [in] */ __RPC__in const XPS_MATRIX *matrix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMMatrixTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMMatrixTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMMatrixTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMMatrixTransform * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMMatrixTransform * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMMatrixTransform * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMMatrixTransform, GetMatrix)
        HRESULT ( STDMETHODCALLTYPE *GetMatrix )( 
            __RPC__in IXpsOMMatrixTransform * This,
            /* [retval][out] */ __RPC__out XPS_MATRIX *matrix);
        
        DECLSPEC_XFGVIRT(IXpsOMMatrixTransform, SetMatrix)
        HRESULT ( STDMETHODCALLTYPE *SetMatrix )( 
            __RPC__in IXpsOMMatrixTransform * This,
            /* [in] */ __RPC__in const XPS_MATRIX *matrix);
        
        DECLSPEC_XFGVIRT(IXpsOMMatrixTransform, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMMatrixTransform * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        END_INTERFACE
    } IXpsOMMatrixTransformVtbl;

    interface IXpsOMMatrixTransform
    {
        CONST_VTBL struct IXpsOMMatrixTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMMatrixTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMMatrixTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMMatrixTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMMatrixTransform_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMMatrixTransform_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMMatrixTransform_GetMatrix(This,matrix)	\
    ( (This)->lpVtbl -> GetMatrix(This,matrix) ) 

#define IXpsOMMatrixTransform_SetMatrix(This,matrix)	\
    ( (This)->lpVtbl -> SetMatrix(This,matrix) ) 

#define IXpsOMMatrixTransform_Clone(This,matrixTransform)	\
    ( (This)->lpVtbl -> Clone(This,matrixTransform) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMMatrixTransform_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGeometry_INTERFACE_DEFINED__
#define __IXpsOMGeometry_INTERFACE_DEFINED__

/* interface IXpsOMGeometry */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGeometry;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64FCF3D7-4D58-44BA-AD73-A13AF6492072")
    IXpsOMGeometry : public IXpsOMShareable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFigures( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometryFigureCollection **figures) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillRule( 
            /* [retval][out] */ __RPC__out XPS_FILL_RULE *fillRule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFillRule( 
            /* [in] */ XPS_FILL_RULE fillRule) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransform( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLocal( 
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **geometry) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGeometryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGeometry * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGeometry * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGeometry * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, GetFigures)
        HRESULT ( STDMETHODCALLTYPE *GetFigures )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometryFigureCollection **figures);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, GetFillRule)
        HRESULT ( STDMETHODCALLTYPE *GetFillRule )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__out XPS_FILL_RULE *fillRule);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, SetFillRule)
        HRESULT ( STDMETHODCALLTYPE *SetFillRule )( 
            __RPC__in IXpsOMGeometry * This,
            /* [in] */ XPS_FILL_RULE fillRule);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMGeometry * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMGeometry * This,
            /* [string][in] */ __RPC__in_string LPCWSTR lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometry, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMGeometry * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **geometry);
        
        END_INTERFACE
    } IXpsOMGeometryVtbl;

    interface IXpsOMGeometry
    {
        CONST_VTBL struct IXpsOMGeometryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGeometry_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGeometry_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGeometry_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGeometry_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMGeometry_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMGeometry_GetFigures(This,figures)	\
    ( (This)->lpVtbl -> GetFigures(This,figures) ) 

#define IXpsOMGeometry_GetFillRule(This,fillRule)	\
    ( (This)->lpVtbl -> GetFillRule(This,fillRule) ) 

#define IXpsOMGeometry_SetFillRule(This,fillRule)	\
    ( (This)->lpVtbl -> SetFillRule(This,fillRule) ) 

#define IXpsOMGeometry_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMGeometry_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMGeometry_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMGeometry_GetTransformLookup(This,lookup)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,lookup) ) 

#define IXpsOMGeometry_SetTransformLookup(This,lookup)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,lookup) ) 

#define IXpsOMGeometry_Clone(This,geometry)	\
    ( (This)->lpVtbl -> Clone(This,geometry) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGeometry_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGeometryFigure_INTERFACE_DEFINED__
#define __IXpsOMGeometryFigure_INTERFACE_DEFINED__

/* interface IXpsOMGeometryFigure */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGeometryFigure;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D410DC83-908C-443E-8947-B1795D3C165A")
    IXpsOMGeometryFigure : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentData( 
            /* [out][in] */ __RPC__inout UINT32 *dataCount,
            /* [out][in] */ __RPC__inout FLOAT *segmentData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentTypes( 
            /* [out][in] */ __RPC__inout UINT32 *segmentCount,
            /* [out][in] */ __RPC__inout XPS_SEGMENT_TYPE *segmentTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentStrokes( 
            /* [out][in] */ __RPC__inout UINT32 *segmentCount,
            /* [out][in] */ __RPC__inout BOOL *segmentStrokes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSegments( 
            /* [in] */ UINT32 segmentCount,
            /* [in] */ UINT32 segmentDataCount,
            /* [in] */ __RPC__in const XPS_SEGMENT_TYPE *segmentTypes,
            /* [in] */ __RPC__in const FLOAT *segmentData,
            /* [in] */ __RPC__in const BOOL *segmentStrokes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStartPoint( 
            /* [retval][out] */ __RPC__out XPS_POINT *startPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStartPoint( 
            /* [in] */ __RPC__in const XPS_POINT *startPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsClosed( 
            /* [retval][out] */ __RPC__out BOOL *isClosed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIsClosed( 
            /* [in] */ BOOL isClosed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsFilled( 
            /* [retval][out] */ __RPC__out BOOL *isFilled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIsFilled( 
            /* [in] */ BOOL isFilled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentCount( 
            /* [retval][out] */ __RPC__out UINT32 *segmentCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentDataCount( 
            /* [retval][out] */ __RPC__out UINT32 *segmentDataCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSegmentStrokePattern( 
            /* [retval][out] */ __RPC__out XPS_SEGMENT_STROKE_PATTERN *segmentStrokePattern) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometryFigure **geometryFigure) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGeometryFigureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGeometryFigure * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGeometryFigure * This);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetSegmentData)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentData )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [out][in] */ __RPC__inout UINT32 *dataCount,
            /* [out][in] */ __RPC__inout FLOAT *segmentData);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetSegmentTypes)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentTypes )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [out][in] */ __RPC__inout UINT32 *segmentCount,
            /* [out][in] */ __RPC__inout XPS_SEGMENT_TYPE *segmentTypes);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetSegmentStrokes)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentStrokes )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [out][in] */ __RPC__inout UINT32 *segmentCount,
            /* [out][in] */ __RPC__inout BOOL *segmentStrokes);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, SetSegments)
        HRESULT ( STDMETHODCALLTYPE *SetSegments )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [in] */ UINT32 segmentCount,
            /* [in] */ UINT32 segmentDataCount,
            /* [in] */ __RPC__in const XPS_SEGMENT_TYPE *segmentTypes,
            /* [in] */ __RPC__in const FLOAT *segmentData,
            /* [in] */ __RPC__in const BOOL *segmentStrokes);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetStartPoint)
        HRESULT ( STDMETHODCALLTYPE *GetStartPoint )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__out XPS_POINT *startPoint);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, SetStartPoint)
        HRESULT ( STDMETHODCALLTYPE *SetStartPoint )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [in] */ __RPC__in const XPS_POINT *startPoint);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetIsClosed)
        HRESULT ( STDMETHODCALLTYPE *GetIsClosed )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__out BOOL *isClosed);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, SetIsClosed)
        HRESULT ( STDMETHODCALLTYPE *SetIsClosed )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [in] */ BOOL isClosed);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetIsFilled)
        HRESULT ( STDMETHODCALLTYPE *GetIsFilled )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__out BOOL *isFilled);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, SetIsFilled)
        HRESULT ( STDMETHODCALLTYPE *SetIsFilled )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [in] */ BOOL isFilled);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetSegmentCount)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentCount )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__out UINT32 *segmentCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetSegmentDataCount)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentDataCount )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__out UINT32 *segmentDataCount);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, GetSegmentStrokePattern)
        HRESULT ( STDMETHODCALLTYPE *GetSegmentStrokePattern )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__out XPS_SEGMENT_STROKE_PATTERN *segmentStrokePattern);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigure, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMGeometryFigure * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometryFigure **geometryFigure);
        
        END_INTERFACE
    } IXpsOMGeometryFigureVtbl;

    interface IXpsOMGeometryFigure
    {
        CONST_VTBL struct IXpsOMGeometryFigureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGeometryFigure_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGeometryFigure_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGeometryFigure_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGeometryFigure_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMGeometryFigure_GetSegmentData(This,dataCount,segmentData)	\
    ( (This)->lpVtbl -> GetSegmentData(This,dataCount,segmentData) ) 

#define IXpsOMGeometryFigure_GetSegmentTypes(This,segmentCount,segmentTypes)	\
    ( (This)->lpVtbl -> GetSegmentTypes(This,segmentCount,segmentTypes) ) 

#define IXpsOMGeometryFigure_GetSegmentStrokes(This,segmentCount,segmentStrokes)	\
    ( (This)->lpVtbl -> GetSegmentStrokes(This,segmentCount,segmentStrokes) ) 

#define IXpsOMGeometryFigure_SetSegments(This,segmentCount,segmentDataCount,segmentTypes,segmentData,segmentStrokes)	\
    ( (This)->lpVtbl -> SetSegments(This,segmentCount,segmentDataCount,segmentTypes,segmentData,segmentStrokes) ) 

#define IXpsOMGeometryFigure_GetStartPoint(This,startPoint)	\
    ( (This)->lpVtbl -> GetStartPoint(This,startPoint) ) 

#define IXpsOMGeometryFigure_SetStartPoint(This,startPoint)	\
    ( (This)->lpVtbl -> SetStartPoint(This,startPoint) ) 

#define IXpsOMGeometryFigure_GetIsClosed(This,isClosed)	\
    ( (This)->lpVtbl -> GetIsClosed(This,isClosed) ) 

#define IXpsOMGeometryFigure_SetIsClosed(This,isClosed)	\
    ( (This)->lpVtbl -> SetIsClosed(This,isClosed) ) 

#define IXpsOMGeometryFigure_GetIsFilled(This,isFilled)	\
    ( (This)->lpVtbl -> GetIsFilled(This,isFilled) ) 

#define IXpsOMGeometryFigure_SetIsFilled(This,isFilled)	\
    ( (This)->lpVtbl -> SetIsFilled(This,isFilled) ) 

#define IXpsOMGeometryFigure_GetSegmentCount(This,segmentCount)	\
    ( (This)->lpVtbl -> GetSegmentCount(This,segmentCount) ) 

#define IXpsOMGeometryFigure_GetSegmentDataCount(This,segmentDataCount)	\
    ( (This)->lpVtbl -> GetSegmentDataCount(This,segmentDataCount) ) 

#define IXpsOMGeometryFigure_GetSegmentStrokePattern(This,segmentStrokePattern)	\
    ( (This)->lpVtbl -> GetSegmentStrokePattern(This,segmentStrokePattern) ) 

#define IXpsOMGeometryFigure_Clone(This,geometryFigure)	\
    ( (This)->lpVtbl -> Clone(This,geometryFigure) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGeometryFigure_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGeometryFigureCollection_INTERFACE_DEFINED__
#define __IXpsOMGeometryFigureCollection_INTERFACE_DEFINED__

/* interface IXpsOMGeometryFigureCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGeometryFigureCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD48C3F3-A58E-4B5A-8826-1DE54ABE72B2")
    IXpsOMGeometryFigureCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometryFigure **geometryFigure) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGeometryFigure *geometryFigure) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGeometryFigure *geometryFigure) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMGeometryFigure *geometryFigure) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGeometryFigureCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGeometryFigureCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGeometryFigureCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigureCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigureCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometryFigure **geometryFigure);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigureCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGeometryFigure *geometryFigure);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigureCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigureCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGeometryFigure *geometryFigure);
        
        DECLSPEC_XFGVIRT(IXpsOMGeometryFigureCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMGeometryFigureCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMGeometryFigure *geometryFigure);
        
        END_INTERFACE
    } IXpsOMGeometryFigureCollectionVtbl;

    interface IXpsOMGeometryFigureCollection
    {
        CONST_VTBL struct IXpsOMGeometryFigureCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGeometryFigureCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGeometryFigureCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGeometryFigureCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGeometryFigureCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMGeometryFigureCollection_GetAt(This,index,geometryFigure)	\
    ( (This)->lpVtbl -> GetAt(This,index,geometryFigure) ) 

#define IXpsOMGeometryFigureCollection_InsertAt(This,index,geometryFigure)	\
    ( (This)->lpVtbl -> InsertAt(This,index,geometryFigure) ) 

#define IXpsOMGeometryFigureCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMGeometryFigureCollection_SetAt(This,index,geometryFigure)	\
    ( (This)->lpVtbl -> SetAt(This,index,geometryFigure) ) 

#define IXpsOMGeometryFigureCollection_Append(This,geometryFigure)	\
    ( (This)->lpVtbl -> Append(This,geometryFigure) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGeometryFigureCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPath_INTERFACE_DEFINED__
#define __IXpsOMPath_INTERFACE_DEFINED__

/* interface IXpsOMPath */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPath;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("37D38BB6-3EE9-4110-9312-14B194163337")
    IXpsOMPath : public IXpsOMVisual
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetGeometry( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **geometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGeometryLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **geometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGeometryLocal( 
            /* [in] */ __RPC__in_opt IXpsOMGeometry *geometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGeometryLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGeometryLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAccessibilityShortDescription( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *shortDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccessibilityShortDescription( 
            /* [string][in] */ __RPC__in_string LPCWSTR shortDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAccessibilityLongDescription( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *longDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccessibilityLongDescription( 
            /* [string][in] */ __RPC__in_string LPCWSTR longDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSnapsToPixels( 
            /* [retval][out] */ __RPC__out BOOL *snapsToPixels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSnapsToPixels( 
            /* [in] */ BOOL snapsToPixels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeBrush( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeBrushLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeBrushLocal( 
            /* [in] */ __RPC__in_opt IXpsOMBrush *brush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeBrushLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeBrushLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeDashes( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDashCollection **strokeDashes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeDashCap( 
            /* [retval][out] */ __RPC__out XPS_DASH_CAP *strokeDashCap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeDashCap( 
            /* [in] */ XPS_DASH_CAP strokeDashCap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeDashOffset( 
            /* [retval][out] */ __RPC__out FLOAT *strokeDashOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeDashOffset( 
            /* [in] */ FLOAT strokeDashOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeStartLineCap( 
            /* [retval][out] */ __RPC__out XPS_LINE_CAP *strokeStartLineCap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeStartLineCap( 
            /* [in] */ XPS_LINE_CAP strokeStartLineCap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeEndLineCap( 
            /* [retval][out] */ __RPC__out XPS_LINE_CAP *strokeEndLineCap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeEndLineCap( 
            /* [in] */ XPS_LINE_CAP strokeEndLineCap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeLineJoin( 
            /* [retval][out] */ __RPC__out XPS_LINE_JOIN *strokeLineJoin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeLineJoin( 
            /* [in] */ XPS_LINE_JOIN strokeLineJoin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeMiterLimit( 
            /* [retval][out] */ __RPC__out FLOAT *strokeMiterLimit) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeMiterLimit( 
            /* [in] */ FLOAT strokeMiterLimit) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStrokeThickness( 
            /* [retval][out] */ __RPC__out FLOAT *strokeThickness) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStrokeThickness( 
            /* [in] */ FLOAT strokeThickness) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillBrush( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillBrushLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFillBrushLocal( 
            /* [in] */ __RPC__in_opt IXpsOMBrush *brush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFillBrushLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFillBrushLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPath **path) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPathVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPath * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPath * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometry)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometry )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IXpsOMGeometry *clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrush)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrush )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *GetIsHyperlinkTarget )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out BOOL *isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *SetIsHyperlinkTarget )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ BOOL isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *GetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IUri **hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *SetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IUri *hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR language);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetGeometry)
        HRESULT ( STDMETHODCALLTYPE *GetGeometry )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **geometry);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetGeometryLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **geometry);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetGeometryLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IXpsOMGeometry *geometry);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *GetGeometryLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *SetGeometryLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetAccessibilityShortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetAccessibilityShortDescription )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *shortDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetAccessibilityShortDescription)
        HRESULT ( STDMETHODCALLTYPE *SetAccessibilityShortDescription )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR shortDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetAccessibilityLongDescription)
        HRESULT ( STDMETHODCALLTYPE *GetAccessibilityLongDescription )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *longDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetAccessibilityLongDescription)
        HRESULT ( STDMETHODCALLTYPE *SetAccessibilityLongDescription )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR longDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetSnapsToPixels)
        HRESULT ( STDMETHODCALLTYPE *GetSnapsToPixels )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out BOOL *snapsToPixels);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetSnapsToPixels)
        HRESULT ( STDMETHODCALLTYPE *SetSnapsToPixels )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ BOOL snapsToPixels);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeBrush)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeBrush )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeBrushLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeBrushLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *brush);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeBrushLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeBrushLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeDashes)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeDashes )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDashCollection **strokeDashes);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeDashCap)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeDashCap )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out XPS_DASH_CAP *strokeDashCap);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeDashCap)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeDashCap )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ XPS_DASH_CAP strokeDashCap);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeDashOffset)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeDashOffset )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out FLOAT *strokeDashOffset);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeDashOffset)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeDashOffset )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ FLOAT strokeDashOffset);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeStartLineCap)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeStartLineCap )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out XPS_LINE_CAP *strokeStartLineCap);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeStartLineCap)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeStartLineCap )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ XPS_LINE_CAP strokeStartLineCap);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeEndLineCap)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeEndLineCap )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out XPS_LINE_CAP *strokeEndLineCap);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeEndLineCap)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeEndLineCap )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ XPS_LINE_CAP strokeEndLineCap);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeLineJoin)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeLineJoin )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out XPS_LINE_JOIN *strokeLineJoin);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeLineJoin)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeLineJoin )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ XPS_LINE_JOIN strokeLineJoin);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeMiterLimit)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeMiterLimit )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out FLOAT *strokeMiterLimit);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeMiterLimit)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeMiterLimit )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ FLOAT strokeMiterLimit);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetStrokeThickness)
        HRESULT ( STDMETHODCALLTYPE *GetStrokeThickness )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__out FLOAT *strokeThickness);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetStrokeThickness)
        HRESULT ( STDMETHODCALLTYPE *SetStrokeThickness )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ FLOAT strokeThickness);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetFillBrush)
        HRESULT ( STDMETHODCALLTYPE *GetFillBrush )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetFillBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetFillBrushLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **brush);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetFillBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetFillBrushLocal )( 
            __RPC__in IXpsOMPath * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *brush);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, GetFillBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetFillBrushLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, SetFillBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetFillBrushLookup )( 
            __RPC__in IXpsOMPath * This,
            /* [string][in] */ __RPC__in_string LPCWSTR lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMPath, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMPath * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPath **path);
        
        END_INTERFACE
    } IXpsOMPathVtbl;

    interface IXpsOMPath
    {
        CONST_VTBL struct IXpsOMPathVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPath_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPath_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPath_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPath_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMPath_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMPath_GetTransform(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransform(This,matrixTransform) ) 

#define IXpsOMPath_GetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,matrixTransform) ) 

#define IXpsOMPath_SetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,matrixTransform) ) 

#define IXpsOMPath_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMPath_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMPath_GetClipGeometry(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometry(This,clipGeometry) ) 

#define IXpsOMPath_GetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMPath_SetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> SetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMPath_GetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> GetClipGeometryLookup(This,key) ) 

#define IXpsOMPath_SetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> SetClipGeometryLookup(This,key) ) 

#define IXpsOMPath_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMPath_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 

#define IXpsOMPath_GetOpacityMaskBrush(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrush(This,opacityMaskBrush) ) 

#define IXpsOMPath_GetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMPath_SetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMPath_GetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMPath_SetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMPath_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IXpsOMPath_SetName(This,name)	\
    ( (This)->lpVtbl -> SetName(This,name) ) 

#define IXpsOMPath_GetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> GetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMPath_SetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> SetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMPath_GetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> GetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMPath_SetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> SetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMPath_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMPath_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 


#define IXpsOMPath_GetGeometry(This,geometry)	\
    ( (This)->lpVtbl -> GetGeometry(This,geometry) ) 

#define IXpsOMPath_GetGeometryLocal(This,geometry)	\
    ( (This)->lpVtbl -> GetGeometryLocal(This,geometry) ) 

#define IXpsOMPath_SetGeometryLocal(This,geometry)	\
    ( (This)->lpVtbl -> SetGeometryLocal(This,geometry) ) 

#define IXpsOMPath_GetGeometryLookup(This,lookup)	\
    ( (This)->lpVtbl -> GetGeometryLookup(This,lookup) ) 

#define IXpsOMPath_SetGeometryLookup(This,lookup)	\
    ( (This)->lpVtbl -> SetGeometryLookup(This,lookup) ) 

#define IXpsOMPath_GetAccessibilityShortDescription(This,shortDescription)	\
    ( (This)->lpVtbl -> GetAccessibilityShortDescription(This,shortDescription) ) 

#define IXpsOMPath_SetAccessibilityShortDescription(This,shortDescription)	\
    ( (This)->lpVtbl -> SetAccessibilityShortDescription(This,shortDescription) ) 

#define IXpsOMPath_GetAccessibilityLongDescription(This,longDescription)	\
    ( (This)->lpVtbl -> GetAccessibilityLongDescription(This,longDescription) ) 

#define IXpsOMPath_SetAccessibilityLongDescription(This,longDescription)	\
    ( (This)->lpVtbl -> SetAccessibilityLongDescription(This,longDescription) ) 

#define IXpsOMPath_GetSnapsToPixels(This,snapsToPixels)	\
    ( (This)->lpVtbl -> GetSnapsToPixels(This,snapsToPixels) ) 

#define IXpsOMPath_SetSnapsToPixels(This,snapsToPixels)	\
    ( (This)->lpVtbl -> SetSnapsToPixels(This,snapsToPixels) ) 

#define IXpsOMPath_GetStrokeBrush(This,brush)	\
    ( (This)->lpVtbl -> GetStrokeBrush(This,brush) ) 

#define IXpsOMPath_GetStrokeBrushLocal(This,brush)	\
    ( (This)->lpVtbl -> GetStrokeBrushLocal(This,brush) ) 

#define IXpsOMPath_SetStrokeBrushLocal(This,brush)	\
    ( (This)->lpVtbl -> SetStrokeBrushLocal(This,brush) ) 

#define IXpsOMPath_GetStrokeBrushLookup(This,lookup)	\
    ( (This)->lpVtbl -> GetStrokeBrushLookup(This,lookup) ) 

#define IXpsOMPath_SetStrokeBrushLookup(This,lookup)	\
    ( (This)->lpVtbl -> SetStrokeBrushLookup(This,lookup) ) 

#define IXpsOMPath_GetStrokeDashes(This,strokeDashes)	\
    ( (This)->lpVtbl -> GetStrokeDashes(This,strokeDashes) ) 

#define IXpsOMPath_GetStrokeDashCap(This,strokeDashCap)	\
    ( (This)->lpVtbl -> GetStrokeDashCap(This,strokeDashCap) ) 

#define IXpsOMPath_SetStrokeDashCap(This,strokeDashCap)	\
    ( (This)->lpVtbl -> SetStrokeDashCap(This,strokeDashCap) ) 

#define IXpsOMPath_GetStrokeDashOffset(This,strokeDashOffset)	\
    ( (This)->lpVtbl -> GetStrokeDashOffset(This,strokeDashOffset) ) 

#define IXpsOMPath_SetStrokeDashOffset(This,strokeDashOffset)	\
    ( (This)->lpVtbl -> SetStrokeDashOffset(This,strokeDashOffset) ) 

#define IXpsOMPath_GetStrokeStartLineCap(This,strokeStartLineCap)	\
    ( (This)->lpVtbl -> GetStrokeStartLineCap(This,strokeStartLineCap) ) 

#define IXpsOMPath_SetStrokeStartLineCap(This,strokeStartLineCap)	\
    ( (This)->lpVtbl -> SetStrokeStartLineCap(This,strokeStartLineCap) ) 

#define IXpsOMPath_GetStrokeEndLineCap(This,strokeEndLineCap)	\
    ( (This)->lpVtbl -> GetStrokeEndLineCap(This,strokeEndLineCap) ) 

#define IXpsOMPath_SetStrokeEndLineCap(This,strokeEndLineCap)	\
    ( (This)->lpVtbl -> SetStrokeEndLineCap(This,strokeEndLineCap) ) 

#define IXpsOMPath_GetStrokeLineJoin(This,strokeLineJoin)	\
    ( (This)->lpVtbl -> GetStrokeLineJoin(This,strokeLineJoin) ) 

#define IXpsOMPath_SetStrokeLineJoin(This,strokeLineJoin)	\
    ( (This)->lpVtbl -> SetStrokeLineJoin(This,strokeLineJoin) ) 

#define IXpsOMPath_GetStrokeMiterLimit(This,strokeMiterLimit)	\
    ( (This)->lpVtbl -> GetStrokeMiterLimit(This,strokeMiterLimit) ) 

#define IXpsOMPath_SetStrokeMiterLimit(This,strokeMiterLimit)	\
    ( (This)->lpVtbl -> SetStrokeMiterLimit(This,strokeMiterLimit) ) 

#define IXpsOMPath_GetStrokeThickness(This,strokeThickness)	\
    ( (This)->lpVtbl -> GetStrokeThickness(This,strokeThickness) ) 

#define IXpsOMPath_SetStrokeThickness(This,strokeThickness)	\
    ( (This)->lpVtbl -> SetStrokeThickness(This,strokeThickness) ) 

#define IXpsOMPath_GetFillBrush(This,brush)	\
    ( (This)->lpVtbl -> GetFillBrush(This,brush) ) 

#define IXpsOMPath_GetFillBrushLocal(This,brush)	\
    ( (This)->lpVtbl -> GetFillBrushLocal(This,brush) ) 

#define IXpsOMPath_SetFillBrushLocal(This,brush)	\
    ( (This)->lpVtbl -> SetFillBrushLocal(This,brush) ) 

#define IXpsOMPath_GetFillBrushLookup(This,lookup)	\
    ( (This)->lpVtbl -> GetFillBrushLookup(This,lookup) ) 

#define IXpsOMPath_SetFillBrushLookup(This,lookup)	\
    ( (This)->lpVtbl -> SetFillBrushLookup(This,lookup) ) 

#define IXpsOMPath_Clone(This,path)	\
    ( (This)->lpVtbl -> Clone(This,path) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPath_INTERFACE_DEFINED__ */


#ifndef __IXpsOMBrush_INTERFACE_DEFINED__
#define __IXpsOMBrush_INTERFACE_DEFINED__

/* interface IXpsOMBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56A3F80C-EA4C-4187-A57B-A2A473B2B42B")
    IXpsOMBrush : public IXpsOMShareable
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOpacity( 
            /* [retval][out] */ __RPC__out FLOAT *opacity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOpacity( 
            /* [in] */ FLOAT opacity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMBrush * This,
            /* [in] */ FLOAT opacity);
        
        END_INTERFACE
    } IXpsOMBrushVtbl;

    interface IXpsOMBrush
    {
        CONST_VTBL struct IXpsOMBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGradientStopCollection_INTERFACE_DEFINED__
#define __IXpsOMGradientStopCollection_INTERFACE_DEFINED__

/* interface IXpsOMGradientStopCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGradientStopCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9174C3A-3CD3-4319-BDA4-11A39392CEEF")
    IXpsOMGradientStopCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStop **stop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGradientStop *stop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGradientStop *stop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMGradientStop *stop) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGradientStopCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGradientStopCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGradientStopCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStopCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStopCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStop **stop);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStopCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGradientStop *stop);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStopCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStopCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMGradientStop *stop);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStopCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMGradientStopCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMGradientStop *stop);
        
        END_INTERFACE
    } IXpsOMGradientStopCollectionVtbl;

    interface IXpsOMGradientStopCollection
    {
        CONST_VTBL struct IXpsOMGradientStopCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGradientStopCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGradientStopCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGradientStopCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGradientStopCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMGradientStopCollection_GetAt(This,index,stop)	\
    ( (This)->lpVtbl -> GetAt(This,index,stop) ) 

#define IXpsOMGradientStopCollection_InsertAt(This,index,stop)	\
    ( (This)->lpVtbl -> InsertAt(This,index,stop) ) 

#define IXpsOMGradientStopCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMGradientStopCollection_SetAt(This,index,stop)	\
    ( (This)->lpVtbl -> SetAt(This,index,stop) ) 

#define IXpsOMGradientStopCollection_Append(This,stop)	\
    ( (This)->lpVtbl -> Append(This,stop) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGradientStopCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMSolidColorBrush_INTERFACE_DEFINED__
#define __IXpsOMSolidColorBrush_INTERFACE_DEFINED__

/* interface IXpsOMSolidColorBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMSolidColorBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A06F9F05-3BE9-4763-98A8-094FC672E488")
    IXpsOMSolidColorBrush : public IXpsOMBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetColor( 
            /* [out] */ __RPC__out XPS_COLOR *color,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **colorProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColor( 
            /* [in] */ __RPC__in const XPS_COLOR *color,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *colorProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSolidColorBrush **solidColorBrush) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMSolidColorBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMSolidColorBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMSolidColorBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMSolidColorBrush, GetColor)
        HRESULT ( STDMETHODCALLTYPE *GetColor )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [out] */ __RPC__out XPS_COLOR *color,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **colorProfile);
        
        DECLSPEC_XFGVIRT(IXpsOMSolidColorBrush, SetColor)
        HRESULT ( STDMETHODCALLTYPE *SetColor )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [in] */ __RPC__in const XPS_COLOR *color,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *colorProfile);
        
        DECLSPEC_XFGVIRT(IXpsOMSolidColorBrush, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMSolidColorBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSolidColorBrush **solidColorBrush);
        
        END_INTERFACE
    } IXpsOMSolidColorBrushVtbl;

    interface IXpsOMSolidColorBrush
    {
        CONST_VTBL struct IXpsOMSolidColorBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMSolidColorBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMSolidColorBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMSolidColorBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMSolidColorBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMSolidColorBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMSolidColorBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMSolidColorBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMSolidColorBrush_GetColor(This,color,colorProfile)	\
    ( (This)->lpVtbl -> GetColor(This,color,colorProfile) ) 

#define IXpsOMSolidColorBrush_SetColor(This,color,colorProfile)	\
    ( (This)->lpVtbl -> SetColor(This,color,colorProfile) ) 

#define IXpsOMSolidColorBrush_Clone(This,solidColorBrush)	\
    ( (This)->lpVtbl -> Clone(This,solidColorBrush) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMSolidColorBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMTileBrush_INTERFACE_DEFINED__
#define __IXpsOMTileBrush_INTERFACE_DEFINED__

/* interface IXpsOMTileBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMTileBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0FC2328D-D722-4A54-B2EC-BE90218A789E")
    IXpsOMTileBrush : public IXpsOMBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTransform( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLocal( 
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewbox( 
            /* [retval][out] */ __RPC__out XPS_RECT *viewbox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetViewbox( 
            /* [in] */ __RPC__in const XPS_RECT *viewbox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewport( 
            /* [retval][out] */ __RPC__out XPS_RECT *viewport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetViewport( 
            /* [in] */ __RPC__in const XPS_RECT *viewport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTileMode( 
            /* [retval][out] */ __RPC__out XPS_TILE_MODE *tileMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTileMode( 
            /* [in] */ XPS_TILE_MODE tileMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMTileBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMTileBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMTileBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetViewbox)
        HRESULT ( STDMETHODCALLTYPE *GetViewbox )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__out XPS_RECT *viewbox);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetViewbox)
        HRESULT ( STDMETHODCALLTYPE *SetViewbox )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [in] */ __RPC__in const XPS_RECT *viewbox);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetViewport)
        HRESULT ( STDMETHODCALLTYPE *GetViewport )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__out XPS_RECT *viewport);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetViewport)
        HRESULT ( STDMETHODCALLTYPE *SetViewport )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [in] */ __RPC__in const XPS_RECT *viewport);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTileMode)
        HRESULT ( STDMETHODCALLTYPE *GetTileMode )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [retval][out] */ __RPC__out XPS_TILE_MODE *tileMode);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTileMode)
        HRESULT ( STDMETHODCALLTYPE *SetTileMode )( 
            __RPC__in IXpsOMTileBrush * This,
            /* [in] */ XPS_TILE_MODE tileMode);
        
        END_INTERFACE
    } IXpsOMTileBrushVtbl;

    interface IXpsOMTileBrush
    {
        CONST_VTBL struct IXpsOMTileBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMTileBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMTileBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMTileBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMTileBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMTileBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMTileBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMTileBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMTileBrush_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMTileBrush_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMTileBrush_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMTileBrush_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMTileBrush_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMTileBrush_GetViewbox(This,viewbox)	\
    ( (This)->lpVtbl -> GetViewbox(This,viewbox) ) 

#define IXpsOMTileBrush_SetViewbox(This,viewbox)	\
    ( (This)->lpVtbl -> SetViewbox(This,viewbox) ) 

#define IXpsOMTileBrush_GetViewport(This,viewport)	\
    ( (This)->lpVtbl -> GetViewport(This,viewport) ) 

#define IXpsOMTileBrush_SetViewport(This,viewport)	\
    ( (This)->lpVtbl -> SetViewport(This,viewport) ) 

#define IXpsOMTileBrush_GetTileMode(This,tileMode)	\
    ( (This)->lpVtbl -> GetTileMode(This,tileMode) ) 

#define IXpsOMTileBrush_SetTileMode(This,tileMode)	\
    ( (This)->lpVtbl -> SetTileMode(This,tileMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMTileBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMVisualBrush_INTERFACE_DEFINED__
#define __IXpsOMVisualBrush_INTERFACE_DEFINED__

/* interface IXpsOMVisualBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMVisualBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97E294AF-5B37-46B4-8057-874D2F64119B")
    IXpsOMVisualBrush : public IXpsOMTileBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetVisual( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisual **visual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVisualLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisual **visual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVisualLocal( 
            /* [in] */ __RPC__in_opt IXpsOMVisual *visual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVisualLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVisualLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR lookup) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisualBrush **visualBrush) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMVisualBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMVisualBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMVisualBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetViewbox)
        HRESULT ( STDMETHODCALLTYPE *GetViewbox )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__out XPS_RECT *viewbox);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetViewbox)
        HRESULT ( STDMETHODCALLTYPE *SetViewbox )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ __RPC__in const XPS_RECT *viewbox);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetViewport)
        HRESULT ( STDMETHODCALLTYPE *GetViewport )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__out XPS_RECT *viewport);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetViewport)
        HRESULT ( STDMETHODCALLTYPE *SetViewport )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ __RPC__in const XPS_RECT *viewport);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTileMode)
        HRESULT ( STDMETHODCALLTYPE *GetTileMode )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__out XPS_TILE_MODE *tileMode);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTileMode)
        HRESULT ( STDMETHODCALLTYPE *SetTileMode )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ XPS_TILE_MODE tileMode);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualBrush, GetVisual)
        HRESULT ( STDMETHODCALLTYPE *GetVisual )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisual **visual);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualBrush, GetVisualLocal)
        HRESULT ( STDMETHODCALLTYPE *GetVisualLocal )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisual **visual);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualBrush, SetVisualLocal)
        HRESULT ( STDMETHODCALLTYPE *SetVisualLocal )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMVisual *visual);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualBrush, GetVisualLookup)
        HRESULT ( STDMETHODCALLTYPE *GetVisualLookup )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualBrush, SetVisualLookup)
        HRESULT ( STDMETHODCALLTYPE *SetVisualLookup )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR lookup);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualBrush, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMVisualBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisualBrush **visualBrush);
        
        END_INTERFACE
    } IXpsOMVisualBrushVtbl;

    interface IXpsOMVisualBrush
    {
        CONST_VTBL struct IXpsOMVisualBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMVisualBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMVisualBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMVisualBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMVisualBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMVisualBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMVisualBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMVisualBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMVisualBrush_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMVisualBrush_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMVisualBrush_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMVisualBrush_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMVisualBrush_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMVisualBrush_GetViewbox(This,viewbox)	\
    ( (This)->lpVtbl -> GetViewbox(This,viewbox) ) 

#define IXpsOMVisualBrush_SetViewbox(This,viewbox)	\
    ( (This)->lpVtbl -> SetViewbox(This,viewbox) ) 

#define IXpsOMVisualBrush_GetViewport(This,viewport)	\
    ( (This)->lpVtbl -> GetViewport(This,viewport) ) 

#define IXpsOMVisualBrush_SetViewport(This,viewport)	\
    ( (This)->lpVtbl -> SetViewport(This,viewport) ) 

#define IXpsOMVisualBrush_GetTileMode(This,tileMode)	\
    ( (This)->lpVtbl -> GetTileMode(This,tileMode) ) 

#define IXpsOMVisualBrush_SetTileMode(This,tileMode)	\
    ( (This)->lpVtbl -> SetTileMode(This,tileMode) ) 


#define IXpsOMVisualBrush_GetVisual(This,visual)	\
    ( (This)->lpVtbl -> GetVisual(This,visual) ) 

#define IXpsOMVisualBrush_GetVisualLocal(This,visual)	\
    ( (This)->lpVtbl -> GetVisualLocal(This,visual) ) 

#define IXpsOMVisualBrush_SetVisualLocal(This,visual)	\
    ( (This)->lpVtbl -> SetVisualLocal(This,visual) ) 

#define IXpsOMVisualBrush_GetVisualLookup(This,lookup)	\
    ( (This)->lpVtbl -> GetVisualLookup(This,lookup) ) 

#define IXpsOMVisualBrush_SetVisualLookup(This,lookup)	\
    ( (This)->lpVtbl -> SetVisualLookup(This,lookup) ) 

#define IXpsOMVisualBrush_Clone(This,visualBrush)	\
    ( (This)->lpVtbl -> Clone(This,visualBrush) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMVisualBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMImageBrush_INTERFACE_DEFINED__
#define __IXpsOMImageBrush_INTERFACE_DEFINED__

/* interface IXpsOMImageBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMImageBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3DF0B466-D382-49EF-8550-DD94C80242E4")
    IXpsOMImageBrush : public IXpsOMTileBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetImageResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetImageResource( 
            /* [in] */ __RPC__in_opt IXpsOMImageResource *imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColorProfileResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **colorProfileResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColorProfileResource( 
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *colorProfileResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageBrush **imageBrush) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMImageBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMImageBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMImageBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetViewbox)
        HRESULT ( STDMETHODCALLTYPE *GetViewbox )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__out XPS_RECT *viewbox);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetViewbox)
        HRESULT ( STDMETHODCALLTYPE *SetViewbox )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ __RPC__in const XPS_RECT *viewbox);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetViewport)
        HRESULT ( STDMETHODCALLTYPE *GetViewport )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__out XPS_RECT *viewport);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetViewport)
        HRESULT ( STDMETHODCALLTYPE *SetViewport )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ __RPC__in const XPS_RECT *viewport);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, GetTileMode)
        HRESULT ( STDMETHODCALLTYPE *GetTileMode )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__out XPS_TILE_MODE *tileMode);
        
        DECLSPEC_XFGVIRT(IXpsOMTileBrush, SetTileMode)
        HRESULT ( STDMETHODCALLTYPE *SetTileMode )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ XPS_TILE_MODE tileMode);
        
        DECLSPEC_XFGVIRT(IXpsOMImageBrush, GetImageResource)
        HRESULT ( STDMETHODCALLTYPE *GetImageResource )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMImageBrush, SetImageResource)
        HRESULT ( STDMETHODCALLTYPE *SetImageResource )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMImageBrush, GetColorProfileResource)
        HRESULT ( STDMETHODCALLTYPE *GetColorProfileResource )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **colorProfileResource);
        
        DECLSPEC_XFGVIRT(IXpsOMImageBrush, SetColorProfileResource)
        HRESULT ( STDMETHODCALLTYPE *SetColorProfileResource )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *colorProfileResource);
        
        DECLSPEC_XFGVIRT(IXpsOMImageBrush, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMImageBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageBrush **imageBrush);
        
        END_INTERFACE
    } IXpsOMImageBrushVtbl;

    interface IXpsOMImageBrush
    {
        CONST_VTBL struct IXpsOMImageBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMImageBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMImageBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMImageBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMImageBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMImageBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMImageBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMImageBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMImageBrush_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMImageBrush_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMImageBrush_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMImageBrush_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMImageBrush_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMImageBrush_GetViewbox(This,viewbox)	\
    ( (This)->lpVtbl -> GetViewbox(This,viewbox) ) 

#define IXpsOMImageBrush_SetViewbox(This,viewbox)	\
    ( (This)->lpVtbl -> SetViewbox(This,viewbox) ) 

#define IXpsOMImageBrush_GetViewport(This,viewport)	\
    ( (This)->lpVtbl -> GetViewport(This,viewport) ) 

#define IXpsOMImageBrush_SetViewport(This,viewport)	\
    ( (This)->lpVtbl -> SetViewport(This,viewport) ) 

#define IXpsOMImageBrush_GetTileMode(This,tileMode)	\
    ( (This)->lpVtbl -> GetTileMode(This,tileMode) ) 

#define IXpsOMImageBrush_SetTileMode(This,tileMode)	\
    ( (This)->lpVtbl -> SetTileMode(This,tileMode) ) 


#define IXpsOMImageBrush_GetImageResource(This,imageResource)	\
    ( (This)->lpVtbl -> GetImageResource(This,imageResource) ) 

#define IXpsOMImageBrush_SetImageResource(This,imageResource)	\
    ( (This)->lpVtbl -> SetImageResource(This,imageResource) ) 

#define IXpsOMImageBrush_GetColorProfileResource(This,colorProfileResource)	\
    ( (This)->lpVtbl -> GetColorProfileResource(This,colorProfileResource) ) 

#define IXpsOMImageBrush_SetColorProfileResource(This,colorProfileResource)	\
    ( (This)->lpVtbl -> SetColorProfileResource(This,colorProfileResource) ) 

#define IXpsOMImageBrush_Clone(This,imageBrush)	\
    ( (This)->lpVtbl -> Clone(This,imageBrush) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMImageBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGradientStop_INTERFACE_DEFINED__
#define __IXpsOMGradientStop_INTERFACE_DEFINED__

/* interface IXpsOMGradientStop */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGradientStop;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5CF4F5CC-3969-49B5-A70A-5550B618FE49")
    IXpsOMGradientStop : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientBrush **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOffset( 
            /* [retval][out] */ __RPC__out FLOAT *offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOffset( 
            /* [in] */ FLOAT offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColor( 
            /* [out] */ __RPC__out XPS_COLOR *color,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **colorProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColor( 
            /* [in] */ __RPC__in const XPS_COLOR *color,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *colorProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStop **gradientStop) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGradientStopVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGradientStop * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGradientStop * This);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStop, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientBrush **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStop, GetOffset)
        HRESULT ( STDMETHODCALLTYPE *GetOffset )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [retval][out] */ __RPC__out FLOAT *offset);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStop, SetOffset)
        HRESULT ( STDMETHODCALLTYPE *SetOffset )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [in] */ FLOAT offset);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStop, GetColor)
        HRESULT ( STDMETHODCALLTYPE *GetColor )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [out] */ __RPC__out XPS_COLOR *color,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **colorProfile);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStop, SetColor)
        HRESULT ( STDMETHODCALLTYPE *SetColor )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [in] */ __RPC__in const XPS_COLOR *color,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *colorProfile);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientStop, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMGradientStop * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStop **gradientStop);
        
        END_INTERFACE
    } IXpsOMGradientStopVtbl;

    interface IXpsOMGradientStop
    {
        CONST_VTBL struct IXpsOMGradientStopVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGradientStop_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGradientStop_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGradientStop_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGradientStop_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMGradientStop_GetOffset(This,offset)	\
    ( (This)->lpVtbl -> GetOffset(This,offset) ) 

#define IXpsOMGradientStop_SetOffset(This,offset)	\
    ( (This)->lpVtbl -> SetOffset(This,offset) ) 

#define IXpsOMGradientStop_GetColor(This,color,colorProfile)	\
    ( (This)->lpVtbl -> GetColor(This,color,colorProfile) ) 

#define IXpsOMGradientStop_SetColor(This,color,colorProfile)	\
    ( (This)->lpVtbl -> SetColor(This,color,colorProfile) ) 

#define IXpsOMGradientStop_Clone(This,gradientStop)	\
    ( (This)->lpVtbl -> Clone(This,gradientStop) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGradientStop_INTERFACE_DEFINED__ */


#ifndef __IXpsOMGradientBrush_INTERFACE_DEFINED__
#define __IXpsOMGradientBrush_INTERFACE_DEFINED__

/* interface IXpsOMGradientBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMGradientBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EDB59622-61A2-42C3-BACE-ACF2286C06BF")
    IXpsOMGradientBrush : public IXpsOMBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetGradientStops( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStopCollection **gradientStops) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransform( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLocal( 
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformLookup( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformLookup( 
            /* [string][in] */ __RPC__in_string LPCWSTR key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSpreadMethod( 
            /* [retval][out] */ __RPC__out XPS_SPREAD_METHOD *spreadMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSpreadMethod( 
            /* [in] */ XPS_SPREAD_METHOD spreadMethod) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColorInterpolationMode( 
            /* [retval][out] */ __RPC__out XPS_COLOR_INTERPOLATION *colorInterpolationMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColorInterpolationMode( 
            /* [in] */ XPS_COLOR_INTERPOLATION colorInterpolationMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMGradientBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMGradientBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMGradientBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetGradientStops)
        HRESULT ( STDMETHODCALLTYPE *GetGradientStops )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStopCollection **gradientStops);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetSpreadMethod)
        HRESULT ( STDMETHODCALLTYPE *GetSpreadMethod )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_SPREAD_METHOD *spreadMethod);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetSpreadMethod)
        HRESULT ( STDMETHODCALLTYPE *SetSpreadMethod )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [in] */ XPS_SPREAD_METHOD spreadMethod);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetColorInterpolationMode)
        HRESULT ( STDMETHODCALLTYPE *GetColorInterpolationMode )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_COLOR_INTERPOLATION *colorInterpolationMode);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetColorInterpolationMode)
        HRESULT ( STDMETHODCALLTYPE *SetColorInterpolationMode )( 
            __RPC__in IXpsOMGradientBrush * This,
            /* [in] */ XPS_COLOR_INTERPOLATION colorInterpolationMode);
        
        END_INTERFACE
    } IXpsOMGradientBrushVtbl;

    interface IXpsOMGradientBrush
    {
        CONST_VTBL struct IXpsOMGradientBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMGradientBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMGradientBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMGradientBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMGradientBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMGradientBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMGradientBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMGradientBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMGradientBrush_GetGradientStops(This,gradientStops)	\
    ( (This)->lpVtbl -> GetGradientStops(This,gradientStops) ) 

#define IXpsOMGradientBrush_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMGradientBrush_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMGradientBrush_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMGradientBrush_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMGradientBrush_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMGradientBrush_GetSpreadMethod(This,spreadMethod)	\
    ( (This)->lpVtbl -> GetSpreadMethod(This,spreadMethod) ) 

#define IXpsOMGradientBrush_SetSpreadMethod(This,spreadMethod)	\
    ( (This)->lpVtbl -> SetSpreadMethod(This,spreadMethod) ) 

#define IXpsOMGradientBrush_GetColorInterpolationMode(This,colorInterpolationMode)	\
    ( (This)->lpVtbl -> GetColorInterpolationMode(This,colorInterpolationMode) ) 

#define IXpsOMGradientBrush_SetColorInterpolationMode(This,colorInterpolationMode)	\
    ( (This)->lpVtbl -> SetColorInterpolationMode(This,colorInterpolationMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMGradientBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMLinearGradientBrush_INTERFACE_DEFINED__
#define __IXpsOMLinearGradientBrush_INTERFACE_DEFINED__

/* interface IXpsOMLinearGradientBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMLinearGradientBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("005E279F-C30D-40FF-93EC-1950D3C528DB")
    IXpsOMLinearGradientBrush : public IXpsOMGradientBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStartPoint( 
            /* [retval][out] */ __RPC__out XPS_POINT *startPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStartPoint( 
            /* [in] */ __RPC__in const XPS_POINT *startPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEndPoint( 
            /* [retval][out] */ __RPC__out XPS_POINT *endPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEndPoint( 
            /* [in] */ __RPC__in const XPS_POINT *endPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMLinearGradientBrush **linearGradientBrush) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMLinearGradientBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMLinearGradientBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMLinearGradientBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetGradientStops)
        HRESULT ( STDMETHODCALLTYPE *GetGradientStops )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStopCollection **gradientStops);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetSpreadMethod)
        HRESULT ( STDMETHODCALLTYPE *GetSpreadMethod )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_SPREAD_METHOD *spreadMethod);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetSpreadMethod)
        HRESULT ( STDMETHODCALLTYPE *SetSpreadMethod )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ XPS_SPREAD_METHOD spreadMethod);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetColorInterpolationMode)
        HRESULT ( STDMETHODCALLTYPE *GetColorInterpolationMode )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_COLOR_INTERPOLATION *colorInterpolationMode);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetColorInterpolationMode)
        HRESULT ( STDMETHODCALLTYPE *SetColorInterpolationMode )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ XPS_COLOR_INTERPOLATION colorInterpolationMode);
        
        DECLSPEC_XFGVIRT(IXpsOMLinearGradientBrush, GetStartPoint)
        HRESULT ( STDMETHODCALLTYPE *GetStartPoint )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_POINT *startPoint);
        
        DECLSPEC_XFGVIRT(IXpsOMLinearGradientBrush, SetStartPoint)
        HRESULT ( STDMETHODCALLTYPE *SetStartPoint )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ __RPC__in const XPS_POINT *startPoint);
        
        DECLSPEC_XFGVIRT(IXpsOMLinearGradientBrush, GetEndPoint)
        HRESULT ( STDMETHODCALLTYPE *GetEndPoint )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_POINT *endPoint);
        
        DECLSPEC_XFGVIRT(IXpsOMLinearGradientBrush, SetEndPoint)
        HRESULT ( STDMETHODCALLTYPE *SetEndPoint )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [in] */ __RPC__in const XPS_POINT *endPoint);
        
        DECLSPEC_XFGVIRT(IXpsOMLinearGradientBrush, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMLinearGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMLinearGradientBrush **linearGradientBrush);
        
        END_INTERFACE
    } IXpsOMLinearGradientBrushVtbl;

    interface IXpsOMLinearGradientBrush
    {
        CONST_VTBL struct IXpsOMLinearGradientBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMLinearGradientBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMLinearGradientBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMLinearGradientBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMLinearGradientBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMLinearGradientBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMLinearGradientBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMLinearGradientBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMLinearGradientBrush_GetGradientStops(This,gradientStops)	\
    ( (This)->lpVtbl -> GetGradientStops(This,gradientStops) ) 

#define IXpsOMLinearGradientBrush_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMLinearGradientBrush_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMLinearGradientBrush_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMLinearGradientBrush_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMLinearGradientBrush_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMLinearGradientBrush_GetSpreadMethod(This,spreadMethod)	\
    ( (This)->lpVtbl -> GetSpreadMethod(This,spreadMethod) ) 

#define IXpsOMLinearGradientBrush_SetSpreadMethod(This,spreadMethod)	\
    ( (This)->lpVtbl -> SetSpreadMethod(This,spreadMethod) ) 

#define IXpsOMLinearGradientBrush_GetColorInterpolationMode(This,colorInterpolationMode)	\
    ( (This)->lpVtbl -> GetColorInterpolationMode(This,colorInterpolationMode) ) 

#define IXpsOMLinearGradientBrush_SetColorInterpolationMode(This,colorInterpolationMode)	\
    ( (This)->lpVtbl -> SetColorInterpolationMode(This,colorInterpolationMode) ) 


#define IXpsOMLinearGradientBrush_GetStartPoint(This,startPoint)	\
    ( (This)->lpVtbl -> GetStartPoint(This,startPoint) ) 

#define IXpsOMLinearGradientBrush_SetStartPoint(This,startPoint)	\
    ( (This)->lpVtbl -> SetStartPoint(This,startPoint) ) 

#define IXpsOMLinearGradientBrush_GetEndPoint(This,endPoint)	\
    ( (This)->lpVtbl -> GetEndPoint(This,endPoint) ) 

#define IXpsOMLinearGradientBrush_SetEndPoint(This,endPoint)	\
    ( (This)->lpVtbl -> SetEndPoint(This,endPoint) ) 

#define IXpsOMLinearGradientBrush_Clone(This,linearGradientBrush)	\
    ( (This)->lpVtbl -> Clone(This,linearGradientBrush) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMLinearGradientBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMRadialGradientBrush_INTERFACE_DEFINED__
#define __IXpsOMRadialGradientBrush_INTERFACE_DEFINED__

/* interface IXpsOMRadialGradientBrush */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMRadialGradientBrush;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75F207E5-08BF-413C-96B1-B82B4064176B")
    IXpsOMRadialGradientBrush : public IXpsOMGradientBrush
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCenter( 
            /* [retval][out] */ __RPC__out XPS_POINT *center) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCenter( 
            /* [in] */ __RPC__in const XPS_POINT *center) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRadiiSizes( 
            /* [retval][out] */ __RPC__out XPS_SIZE *radiiSizes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRadiiSizes( 
            /* [in] */ __RPC__in const XPS_SIZE *radiiSizes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGradientOrigin( 
            /* [retval][out] */ __RPC__out XPS_POINT *origin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGradientOrigin( 
            /* [in] */ __RPC__in const XPS_POINT *origin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRadialGradientBrush **radialGradientBrush) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMRadialGradientBrushVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMRadialGradientBrush * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMRadialGradientBrush * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMBrush, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetGradientStops)
        HRESULT ( STDMETHODCALLTYPE *GetGradientStops )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGradientStopCollection **gradientStops);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *transform);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetSpreadMethod)
        HRESULT ( STDMETHODCALLTYPE *GetSpreadMethod )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_SPREAD_METHOD *spreadMethod);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetSpreadMethod)
        HRESULT ( STDMETHODCALLTYPE *SetSpreadMethod )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ XPS_SPREAD_METHOD spreadMethod);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, GetColorInterpolationMode)
        HRESULT ( STDMETHODCALLTYPE *GetColorInterpolationMode )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_COLOR_INTERPOLATION *colorInterpolationMode);
        
        DECLSPEC_XFGVIRT(IXpsOMGradientBrush, SetColorInterpolationMode)
        HRESULT ( STDMETHODCALLTYPE *SetColorInterpolationMode )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ XPS_COLOR_INTERPOLATION colorInterpolationMode);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, GetCenter)
        HRESULT ( STDMETHODCALLTYPE *GetCenter )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_POINT *center);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, SetCenter)
        HRESULT ( STDMETHODCALLTYPE *SetCenter )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ __RPC__in const XPS_POINT *center);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, GetRadiiSizes)
        HRESULT ( STDMETHODCALLTYPE *GetRadiiSizes )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_SIZE *radiiSizes);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, SetRadiiSizes)
        HRESULT ( STDMETHODCALLTYPE *SetRadiiSizes )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ __RPC__in const XPS_SIZE *radiiSizes);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, GetGradientOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetGradientOrigin )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__out XPS_POINT *origin);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, SetGradientOrigin)
        HRESULT ( STDMETHODCALLTYPE *SetGradientOrigin )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [in] */ __RPC__in const XPS_POINT *origin);
        
        DECLSPEC_XFGVIRT(IXpsOMRadialGradientBrush, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMRadialGradientBrush * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRadialGradientBrush **radialGradientBrush);
        
        END_INTERFACE
    } IXpsOMRadialGradientBrushVtbl;

    interface IXpsOMRadialGradientBrush
    {
        CONST_VTBL struct IXpsOMRadialGradientBrushVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMRadialGradientBrush_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMRadialGradientBrush_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMRadialGradientBrush_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMRadialGradientBrush_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMRadialGradientBrush_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMRadialGradientBrush_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMRadialGradientBrush_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 


#define IXpsOMRadialGradientBrush_GetGradientStops(This,gradientStops)	\
    ( (This)->lpVtbl -> GetGradientStops(This,gradientStops) ) 

#define IXpsOMRadialGradientBrush_GetTransform(This,transform)	\
    ( (This)->lpVtbl -> GetTransform(This,transform) ) 

#define IXpsOMRadialGradientBrush_GetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,transform) ) 

#define IXpsOMRadialGradientBrush_SetTransformLocal(This,transform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,transform) ) 

#define IXpsOMRadialGradientBrush_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMRadialGradientBrush_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMRadialGradientBrush_GetSpreadMethod(This,spreadMethod)	\
    ( (This)->lpVtbl -> GetSpreadMethod(This,spreadMethod) ) 

#define IXpsOMRadialGradientBrush_SetSpreadMethod(This,spreadMethod)	\
    ( (This)->lpVtbl -> SetSpreadMethod(This,spreadMethod) ) 

#define IXpsOMRadialGradientBrush_GetColorInterpolationMode(This,colorInterpolationMode)	\
    ( (This)->lpVtbl -> GetColorInterpolationMode(This,colorInterpolationMode) ) 

#define IXpsOMRadialGradientBrush_SetColorInterpolationMode(This,colorInterpolationMode)	\
    ( (This)->lpVtbl -> SetColorInterpolationMode(This,colorInterpolationMode) ) 


#define IXpsOMRadialGradientBrush_GetCenter(This,center)	\
    ( (This)->lpVtbl -> GetCenter(This,center) ) 

#define IXpsOMRadialGradientBrush_SetCenter(This,center)	\
    ( (This)->lpVtbl -> SetCenter(This,center) ) 

#define IXpsOMRadialGradientBrush_GetRadiiSizes(This,radiiSizes)	\
    ( (This)->lpVtbl -> GetRadiiSizes(This,radiiSizes) ) 

#define IXpsOMRadialGradientBrush_SetRadiiSizes(This,radiiSizes)	\
    ( (This)->lpVtbl -> SetRadiiSizes(This,radiiSizes) ) 

#define IXpsOMRadialGradientBrush_GetGradientOrigin(This,origin)	\
    ( (This)->lpVtbl -> GetGradientOrigin(This,origin) ) 

#define IXpsOMRadialGradientBrush_SetGradientOrigin(This,origin)	\
    ( (This)->lpVtbl -> SetGradientOrigin(This,origin) ) 

#define IXpsOMRadialGradientBrush_Clone(This,radialGradientBrush)	\
    ( (This)->lpVtbl -> Clone(This,radialGradientBrush) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMRadialGradientBrush_INTERFACE_DEFINED__ */


#ifndef __IXpsOMResource_INTERFACE_DEFINED__
#define __IXpsOMResource_INTERFACE_DEFINED__

/* interface IXpsOMResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("da2ac0a2-73a2-4975-ad14-74097c3ff3a5")
    IXpsOMResource : public IXpsOMPart
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        END_INTERFACE
    } IXpsOMResourceVtbl;

    interface IXpsOMResource
    {
        CONST_VTBL struct IXpsOMResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPartResources_INTERFACE_DEFINED__
#define __IXpsOMPartResources_INTERFACE_DEFINED__

/* interface IXpsOMPartResources */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPartResources;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f4cf7729-4864-4275-99b3-a8717163ecaf")
    IXpsOMPartResources : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFontResources( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResourceCollection **fontResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImageResources( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResourceCollection **imageResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColorProfileResources( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResourceCollection **colorProfileResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRemoteDictionaryResources( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResourceCollection **dictionaryResources) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPartResourcesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPartResources * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPartResources * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPartResources * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPartResources, GetFontResources)
        HRESULT ( STDMETHODCALLTYPE *GetFontResources )( 
            __RPC__in IXpsOMPartResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResourceCollection **fontResources);
        
        DECLSPEC_XFGVIRT(IXpsOMPartResources, GetImageResources)
        HRESULT ( STDMETHODCALLTYPE *GetImageResources )( 
            __RPC__in IXpsOMPartResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResourceCollection **imageResources);
        
        DECLSPEC_XFGVIRT(IXpsOMPartResources, GetColorProfileResources)
        HRESULT ( STDMETHODCALLTYPE *GetColorProfileResources )( 
            __RPC__in IXpsOMPartResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResourceCollection **colorProfileResources);
        
        DECLSPEC_XFGVIRT(IXpsOMPartResources, GetRemoteDictionaryResources)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteDictionaryResources )( 
            __RPC__in IXpsOMPartResources * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResourceCollection **dictionaryResources);
        
        END_INTERFACE
    } IXpsOMPartResourcesVtbl;

    interface IXpsOMPartResources
    {
        CONST_VTBL struct IXpsOMPartResourcesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPartResources_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPartResources_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPartResources_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPartResources_GetFontResources(This,fontResources)	\
    ( (This)->lpVtbl -> GetFontResources(This,fontResources) ) 

#define IXpsOMPartResources_GetImageResources(This,imageResources)	\
    ( (This)->lpVtbl -> GetImageResources(This,imageResources) ) 

#define IXpsOMPartResources_GetColorProfileResources(This,colorProfileResources)	\
    ( (This)->lpVtbl -> GetColorProfileResources(This,colorProfileResources) ) 

#define IXpsOMPartResources_GetRemoteDictionaryResources(This,dictionaryResources)	\
    ( (This)->lpVtbl -> GetRemoteDictionaryResources(This,dictionaryResources) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPartResources_INTERFACE_DEFINED__ */


#ifndef __IXpsOMDictionary_INTERFACE_DEFINED__
#define __IXpsOMDictionary_INTERFACE_DEFINED__

/* interface IXpsOMDictionary */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMDictionary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("897C86B8-8EAF-4AE3-BDDE-56419FCF4236")
    IXpsOMDictionary : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *key,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMShareable **entry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetByKey( 
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *beforeEntry,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMShareable **entry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIndex( 
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry,
            /* [retval][out] */ __RPC__out UINT32 *index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **dictionary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMDictionaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMDictionary * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMDictionary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMDictionary * This);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMDictionary * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMDictionary * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMDictionary * This,
            /* [in] */ UINT32 index,
            /* [string][out] */ __RPC__deref_out_opt_string LPWSTR *key,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMShareable **entry);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, GetByKey)
        HRESULT ( STDMETHODCALLTYPE *GetByKey )( 
            __RPC__in IXpsOMDictionary * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *beforeEntry,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMShareable **entry);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, GetIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in IXpsOMDictionary * This,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry,
            /* [retval][out] */ __RPC__out UINT32 *index);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMDictionary * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMDictionary * This,
            /* [in] */ UINT32 index,
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMDictionary * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMDictionary * This,
            /* [in] */ UINT32 index,
            /* [string][in] */ __RPC__in_string LPCWSTR key,
            /* [in] */ __RPC__in_opt IXpsOMShareable *entry);
        
        DECLSPEC_XFGVIRT(IXpsOMDictionary, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMDictionary * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **dictionary);
        
        END_INTERFACE
    } IXpsOMDictionaryVtbl;

    interface IXpsOMDictionary
    {
        CONST_VTBL struct IXpsOMDictionaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMDictionary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMDictionary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMDictionary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMDictionary_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMDictionary_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMDictionary_GetAt(This,index,key,entry)	\
    ( (This)->lpVtbl -> GetAt(This,index,key,entry) ) 

#define IXpsOMDictionary_GetByKey(This,key,beforeEntry,entry)	\
    ( (This)->lpVtbl -> GetByKey(This,key,beforeEntry,entry) ) 

#define IXpsOMDictionary_GetIndex(This,entry,index)	\
    ( (This)->lpVtbl -> GetIndex(This,entry,index) ) 

#define IXpsOMDictionary_Append(This,key,entry)	\
    ( (This)->lpVtbl -> Append(This,key,entry) ) 

#define IXpsOMDictionary_InsertAt(This,index,key,entry)	\
    ( (This)->lpVtbl -> InsertAt(This,index,key,entry) ) 

#define IXpsOMDictionary_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMDictionary_SetAt(This,index,key,entry)	\
    ( (This)->lpVtbl -> SetAt(This,index,key,entry) ) 

#define IXpsOMDictionary_Clone(This,dictionary)	\
    ( (This)->lpVtbl -> Clone(This,dictionary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMDictionary_INTERFACE_DEFINED__ */


#ifndef __IXpsOMFontResource_INTERFACE_DEFINED__
#define __IXpsOMFontResource_INTERFACE_DEFINED__

/* interface IXpsOMFontResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMFontResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a8c45708-47d9-4af4-8d20-33b48c9b8485")
    IXpsOMFontResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **readerStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ XPS_FONT_EMBEDDING embeddingOption,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEmbeddingOption( 
            /* [retval][out] */ __RPC__out XPS_FONT_EMBEDDING *embeddingOption) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMFontResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMFontResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMFontResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMFontResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMFontResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMFontResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMFontResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **readerStream);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMFontResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ XPS_FONT_EMBEDDING embeddingOption,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResource, GetEmbeddingOption)
        HRESULT ( STDMETHODCALLTYPE *GetEmbeddingOption )( 
            __RPC__in IXpsOMFontResource * This,
            /* [retval][out] */ __RPC__out XPS_FONT_EMBEDDING *embeddingOption);
        
        END_INTERFACE
    } IXpsOMFontResourceVtbl;

    interface IXpsOMFontResource
    {
        CONST_VTBL struct IXpsOMFontResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMFontResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMFontResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMFontResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMFontResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMFontResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMFontResource_GetStream(This,readerStream)	\
    ( (This)->lpVtbl -> GetStream(This,readerStream) ) 

#define IXpsOMFontResource_SetContent(This,sourceStream,embeddingOption,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,embeddingOption,partName) ) 

#define IXpsOMFontResource_GetEmbeddingOption(This,embeddingOption)	\
    ( (This)->lpVtbl -> GetEmbeddingOption(This,embeddingOption) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMFontResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMFontResourceCollection_INTERFACE_DEFINED__
#define __IXpsOMFontResourceCollection_INTERFACE_DEFINED__

/* interface IXpsOMFontResourceCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMFontResourceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70B4A6BB-88D4-4FA8-AAF9-6D9C596FDBAD")
    IXpsOMFontResourceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResource **value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMFontResource *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMFontResource *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMFontResource *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetByPartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResource **part) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMFontResourceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMFontResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMFontResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResource **value);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMFontResource *value);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMFontResource *value);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMFontResource *value);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMFontResourceCollection, GetByPartName)
        HRESULT ( STDMETHODCALLTYPE *GetByPartName )( 
            __RPC__in IXpsOMFontResourceCollection * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMFontResource **part);
        
        END_INTERFACE
    } IXpsOMFontResourceCollectionVtbl;

    interface IXpsOMFontResourceCollection
    {
        CONST_VTBL struct IXpsOMFontResourceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMFontResourceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMFontResourceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMFontResourceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMFontResourceCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMFontResourceCollection_GetAt(This,index,value)	\
    ( (This)->lpVtbl -> GetAt(This,index,value) ) 

#define IXpsOMFontResourceCollection_SetAt(This,index,value)	\
    ( (This)->lpVtbl -> SetAt(This,index,value) ) 

#define IXpsOMFontResourceCollection_InsertAt(This,index,value)	\
    ( (This)->lpVtbl -> InsertAt(This,index,value) ) 

#define IXpsOMFontResourceCollection_Append(This,value)	\
    ( (This)->lpVtbl -> Append(This,value) ) 

#define IXpsOMFontResourceCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMFontResourceCollection_GetByPartName(This,partName,part)	\
    ( (This)->lpVtbl -> GetByPartName(This,partName,part) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMFontResourceCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMImageResource_INTERFACE_DEFINED__
#define __IXpsOMImageResource_INTERFACE_DEFINED__

/* interface IXpsOMImageResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMImageResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3db8417d-ae50-485e-9a44-d7758f78a23f")
    IXpsOMImageResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **readerStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ XPS_IMAGE_TYPE imageType,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetImageType( 
            /* [retval][out] */ __RPC__out XPS_IMAGE_TYPE *imageType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMImageResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMImageResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMImageResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMImageResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMImageResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMImageResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMImageResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **readerStream);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMImageResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ XPS_IMAGE_TYPE imageType,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResource, GetImageType)
        HRESULT ( STDMETHODCALLTYPE *GetImageType )( 
            __RPC__in IXpsOMImageResource * This,
            /* [retval][out] */ __RPC__out XPS_IMAGE_TYPE *imageType);
        
        END_INTERFACE
    } IXpsOMImageResourceVtbl;

    interface IXpsOMImageResource
    {
        CONST_VTBL struct IXpsOMImageResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMImageResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMImageResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMImageResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMImageResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMImageResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMImageResource_GetStream(This,readerStream)	\
    ( (This)->lpVtbl -> GetStream(This,readerStream) ) 

#define IXpsOMImageResource_SetContent(This,sourceStream,imageType,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,imageType,partName) ) 

#define IXpsOMImageResource_GetImageType(This,imageType)	\
    ( (This)->lpVtbl -> GetImageType(This,imageType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMImageResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMImageResourceCollection_INTERFACE_DEFINED__
#define __IXpsOMImageResourceCollection_INTERFACE_DEFINED__

/* interface IXpsOMImageResourceCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMImageResourceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7A4A1A71-9CDE-4B71-B33F-62DE843EABFE")
    IXpsOMImageResourceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMImageResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetByPartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **part) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMImageResourceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMImageResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMImageResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **object);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMImageResourceCollection, GetByPartName)
        HRESULT ( STDMETHODCALLTYPE *GetByPartName )( 
            __RPC__in IXpsOMImageResourceCollection * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **part);
        
        END_INTERFACE
    } IXpsOMImageResourceCollectionVtbl;

    interface IXpsOMImageResourceCollection
    {
        CONST_VTBL struct IXpsOMImageResourceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMImageResourceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMImageResourceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMImageResourceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMImageResourceCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMImageResourceCollection_GetAt(This,index,object)	\
    ( (This)->lpVtbl -> GetAt(This,index,object) ) 

#define IXpsOMImageResourceCollection_InsertAt(This,index,object)	\
    ( (This)->lpVtbl -> InsertAt(This,index,object) ) 

#define IXpsOMImageResourceCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMImageResourceCollection_SetAt(This,index,object)	\
    ( (This)->lpVtbl -> SetAt(This,index,object) ) 

#define IXpsOMImageResourceCollection_Append(This,object)	\
    ( (This)->lpVtbl -> Append(This,object) ) 

#define IXpsOMImageResourceCollection_GetByPartName(This,partName,part)	\
    ( (This)->lpVtbl -> GetByPartName(This,partName,part) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMImageResourceCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMColorProfileResource_INTERFACE_DEFINED__
#define __IXpsOMColorProfileResource_INTERFACE_DEFINED__

/* interface IXpsOMColorProfileResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMColorProfileResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("67bd7d69-1eef-4bb1-b5e7-6f4f87be8abe")
    IXpsOMColorProfileResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMColorProfileResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMColorProfileResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMColorProfileResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMColorProfileResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMColorProfileResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMColorProfileResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMColorProfileResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMColorProfileResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        END_INTERFACE
    } IXpsOMColorProfileResourceVtbl;

    interface IXpsOMColorProfileResource
    {
        CONST_VTBL struct IXpsOMColorProfileResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMColorProfileResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMColorProfileResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMColorProfileResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMColorProfileResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMColorProfileResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMColorProfileResource_GetStream(This,stream)	\
    ( (This)->lpVtbl -> GetStream(This,stream) ) 

#define IXpsOMColorProfileResource_SetContent(This,sourceStream,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,partName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMColorProfileResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMColorProfileResourceCollection_INTERFACE_DEFINED__
#define __IXpsOMColorProfileResourceCollection_INTERFACE_DEFINED__

/* interface IXpsOMColorProfileResourceCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMColorProfileResourceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12759630-5FBA-4283-8F7D-CCA849809EDB")
    IXpsOMColorProfileResourceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetByPartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **part) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMColorProfileResourceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **object);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMColorProfileResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMColorProfileResourceCollection, GetByPartName)
        HRESULT ( STDMETHODCALLTYPE *GetByPartName )( 
            __RPC__in IXpsOMColorProfileResourceCollection * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMColorProfileResource **part);
        
        END_INTERFACE
    } IXpsOMColorProfileResourceCollectionVtbl;

    interface IXpsOMColorProfileResourceCollection
    {
        CONST_VTBL struct IXpsOMColorProfileResourceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMColorProfileResourceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMColorProfileResourceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMColorProfileResourceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMColorProfileResourceCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMColorProfileResourceCollection_GetAt(This,index,object)	\
    ( (This)->lpVtbl -> GetAt(This,index,object) ) 

#define IXpsOMColorProfileResourceCollection_InsertAt(This,index,object)	\
    ( (This)->lpVtbl -> InsertAt(This,index,object) ) 

#define IXpsOMColorProfileResourceCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMColorProfileResourceCollection_SetAt(This,index,object)	\
    ( (This)->lpVtbl -> SetAt(This,index,object) ) 

#define IXpsOMColorProfileResourceCollection_Append(This,object)	\
    ( (This)->lpVtbl -> Append(This,object) ) 

#define IXpsOMColorProfileResourceCollection_GetByPartName(This,partName,part)	\
    ( (This)->lpVtbl -> GetByPartName(This,partName,part) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMColorProfileResourceCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPrintTicketResource_INTERFACE_DEFINED__
#define __IXpsOMPrintTicketResource_INTERFACE_DEFINED__

/* interface IXpsOMPrintTicketResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPrintTicketResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e7ff32d2-34aa-499b-bbe9-9cd4ee6c59f7")
    IXpsOMPrintTicketResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPrintTicketResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPrintTicketResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPrintTicketResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPrintTicketResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMPrintTicketResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMPrintTicketResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPrintTicketResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMPrintTicketResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream);
        
        DECLSPEC_XFGVIRT(IXpsOMPrintTicketResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMPrintTicketResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        END_INTERFACE
    } IXpsOMPrintTicketResourceVtbl;

    interface IXpsOMPrintTicketResource
    {
        CONST_VTBL struct IXpsOMPrintTicketResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPrintTicketResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPrintTicketResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPrintTicketResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPrintTicketResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMPrintTicketResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMPrintTicketResource_GetStream(This,stream)	\
    ( (This)->lpVtbl -> GetStream(This,stream) ) 

#define IXpsOMPrintTicketResource_SetContent(This,sourceStream,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,partName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPrintTicketResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMRemoteDictionaryResource_INTERFACE_DEFINED__
#define __IXpsOMRemoteDictionaryResource_INTERFACE_DEFINED__

/* interface IXpsOMRemoteDictionaryResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMRemoteDictionaryResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c9bd7cd4-e16a-4bf8-8c84-c950af7a3061")
    IXpsOMRemoteDictionaryResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDictionary( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **dictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDictionary( 
            /* [in] */ __RPC__in_opt IXpsOMDictionary *dictionary) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMRemoteDictionaryResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResource, GetDictionary)
        HRESULT ( STDMETHODCALLTYPE *GetDictionary )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **dictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResource, SetDictionary)
        HRESULT ( STDMETHODCALLTYPE *SetDictionary )( 
            __RPC__in IXpsOMRemoteDictionaryResource * This,
            /* [in] */ __RPC__in_opt IXpsOMDictionary *dictionary);
        
        END_INTERFACE
    } IXpsOMRemoteDictionaryResourceVtbl;

    interface IXpsOMRemoteDictionaryResource
    {
        CONST_VTBL struct IXpsOMRemoteDictionaryResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMRemoteDictionaryResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMRemoteDictionaryResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMRemoteDictionaryResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMRemoteDictionaryResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMRemoteDictionaryResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMRemoteDictionaryResource_GetDictionary(This,dictionary)	\
    ( (This)->lpVtbl -> GetDictionary(This,dictionary) ) 

#define IXpsOMRemoteDictionaryResource_SetDictionary(This,dictionary)	\
    ( (This)->lpVtbl -> SetDictionary(This,dictionary) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMRemoteDictionaryResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMRemoteDictionaryResourceCollection_INTERFACE_DEFINED__
#define __IXpsOMRemoteDictionaryResourceCollection_INTERFACE_DEFINED__

/* interface IXpsOMRemoteDictionaryResourceCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMRemoteDictionaryResourceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5C38DB61-7FEC-464A-87BD-41E3BEF018BE")
    IXpsOMRemoteDictionaryResourceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetByPartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **remoteDictionaryResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMRemoteDictionaryResourceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **object);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *object);
        
        DECLSPEC_XFGVIRT(IXpsOMRemoteDictionaryResourceCollection, GetByPartName)
        HRESULT ( STDMETHODCALLTYPE *GetByPartName )( 
            __RPC__in IXpsOMRemoteDictionaryResourceCollection * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **remoteDictionaryResource);
        
        END_INTERFACE
    } IXpsOMRemoteDictionaryResourceCollectionVtbl;

    interface IXpsOMRemoteDictionaryResourceCollection
    {
        CONST_VTBL struct IXpsOMRemoteDictionaryResourceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMRemoteDictionaryResourceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMRemoteDictionaryResourceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMRemoteDictionaryResourceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMRemoteDictionaryResourceCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMRemoteDictionaryResourceCollection_GetAt(This,index,object)	\
    ( (This)->lpVtbl -> GetAt(This,index,object) ) 

#define IXpsOMRemoteDictionaryResourceCollection_InsertAt(This,index,object)	\
    ( (This)->lpVtbl -> InsertAt(This,index,object) ) 

#define IXpsOMRemoteDictionaryResourceCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMRemoteDictionaryResourceCollection_SetAt(This,index,object)	\
    ( (This)->lpVtbl -> SetAt(This,index,object) ) 

#define IXpsOMRemoteDictionaryResourceCollection_Append(This,object)	\
    ( (This)->lpVtbl -> Append(This,object) ) 

#define IXpsOMRemoteDictionaryResourceCollection_GetByPartName(This,partName,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> GetByPartName(This,partName,remoteDictionaryResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMRemoteDictionaryResourceCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMSignatureBlockResourceCollection_INTERFACE_DEFINED__
#define __IXpsOMSignatureBlockResourceCollection_INTERFACE_DEFINED__

/* interface IXpsOMSignatureBlockResourceCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMSignatureBlockResourceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB8F5D8E-351B-4D33-AAED-FA56F0022931")
    IXpsOMSignatureBlockResourceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSignatureBlockResource **signatureBlockResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResource *signatureBlockResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResource *signatureBlockResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResource *signatureBlockResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetByPartName( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSignatureBlockResource **signatureBlockResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMSignatureBlockResourceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSignatureBlockResource **signatureBlockResource);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResource *signatureBlockResource);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResource *signatureBlockResource);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResource *signatureBlockResource);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResourceCollection, GetByPartName)
        HRESULT ( STDMETHODCALLTYPE *GetByPartName )( 
            __RPC__in IXpsOMSignatureBlockResourceCollection * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSignatureBlockResource **signatureBlockResource);
        
        END_INTERFACE
    } IXpsOMSignatureBlockResourceCollectionVtbl;

    interface IXpsOMSignatureBlockResourceCollection
    {
        CONST_VTBL struct IXpsOMSignatureBlockResourceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMSignatureBlockResourceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMSignatureBlockResourceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMSignatureBlockResourceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMSignatureBlockResourceCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMSignatureBlockResourceCollection_GetAt(This,index,signatureBlockResource)	\
    ( (This)->lpVtbl -> GetAt(This,index,signatureBlockResource) ) 

#define IXpsOMSignatureBlockResourceCollection_InsertAt(This,index,signatureBlockResource)	\
    ( (This)->lpVtbl -> InsertAt(This,index,signatureBlockResource) ) 

#define IXpsOMSignatureBlockResourceCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMSignatureBlockResourceCollection_SetAt(This,index,signatureBlockResource)	\
    ( (This)->lpVtbl -> SetAt(This,index,signatureBlockResource) ) 

#define IXpsOMSignatureBlockResourceCollection_Append(This,signatureBlockResource)	\
    ( (This)->lpVtbl -> Append(This,signatureBlockResource) ) 

#define IXpsOMSignatureBlockResourceCollection_GetByPartName(This,partName,signatureBlockResource)	\
    ( (This)->lpVtbl -> GetByPartName(This,partName,signatureBlockResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMSignatureBlockResourceCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMDocumentStructureResource_INTERFACE_DEFINED__
#define __IXpsOMDocumentStructureResource_INTERFACE_DEFINED__

/* interface IXpsOMDocumentStructureResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMDocumentStructureResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85febc8a-6b63-48a9-af07-7064e4ecff30")
    IXpsOMDocumentStructureResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMDocumentStructureResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMDocumentStructureResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMDocumentStructureResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMDocumentStructureResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMDocumentStructureResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMDocumentStructureResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentStructureResource, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMDocumentStructureResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentStructureResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMDocumentStructureResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentStructureResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMDocumentStructureResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        END_INTERFACE
    } IXpsOMDocumentStructureResourceVtbl;

    interface IXpsOMDocumentStructureResource
    {
        CONST_VTBL struct IXpsOMDocumentStructureResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMDocumentStructureResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMDocumentStructureResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMDocumentStructureResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMDocumentStructureResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMDocumentStructureResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMDocumentStructureResource_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMDocumentStructureResource_GetStream(This,stream)	\
    ( (This)->lpVtbl -> GetStream(This,stream) ) 

#define IXpsOMDocumentStructureResource_SetContent(This,sourceStream,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,partName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMDocumentStructureResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMStoryFragmentsResource_INTERFACE_DEFINED__
#define __IXpsOMStoryFragmentsResource_INTERFACE_DEFINED__

/* interface IXpsOMStoryFragmentsResource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMStoryFragmentsResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c2b3ca09-0473-4282-87ae-1780863223f0")
    IXpsOMStoryFragmentsResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMStoryFragmentsResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMStoryFragmentsResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMStoryFragmentsResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMStoryFragmentsResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMStoryFragmentsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMStoryFragmentsResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMStoryFragmentsResource, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMStoryFragmentsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMStoryFragmentsResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMStoryFragmentsResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream);
        
        DECLSPEC_XFGVIRT(IXpsOMStoryFragmentsResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMStoryFragmentsResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        END_INTERFACE
    } IXpsOMStoryFragmentsResourceVtbl;

    interface IXpsOMStoryFragmentsResource
    {
        CONST_VTBL struct IXpsOMStoryFragmentsResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMStoryFragmentsResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMStoryFragmentsResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMStoryFragmentsResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMStoryFragmentsResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMStoryFragmentsResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMStoryFragmentsResource_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMStoryFragmentsResource_GetStream(This,stream)	\
    ( (This)->lpVtbl -> GetStream(This,stream) ) 

#define IXpsOMStoryFragmentsResource_SetContent(This,sourceStream,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,partName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMStoryFragmentsResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMSignatureBlockResource_INTERFACE_DEFINED__
#define __IXpsOMSignatureBlockResource_INTERFACE_DEFINED__

/* interface IXpsOMSignatureBlockResource */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMSignatureBlockResource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4776ad35-2e04-4357-8743-ebf6c171a905")
    IXpsOMSignatureBlockResource : public IXpsOMResource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **owner) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStream( 
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContent( 
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMSignatureBlockResourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMSignatureBlockResource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMSignatureBlockResource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMSignatureBlockResource * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMSignatureBlockResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMSignatureBlockResource * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResource, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMSignatureBlockResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResource, GetStream)
        HRESULT ( STDMETHODCALLTYPE *GetStream )( 
            __RPC__in IXpsOMSignatureBlockResource * This,
            /* [retval][out] */ __RPC__deref_out_opt IStream **stream);
        
        DECLSPEC_XFGVIRT(IXpsOMSignatureBlockResource, SetContent)
        HRESULT ( STDMETHODCALLTYPE *SetContent )( 
            __RPC__in IXpsOMSignatureBlockResource * This,
            /* [in] */ __RPC__in_opt IStream *sourceStream,
            /* [in] */ __RPC__in_opt IOpcPartUri *partName);
        
        END_INTERFACE
    } IXpsOMSignatureBlockResourceVtbl;

    interface IXpsOMSignatureBlockResource
    {
        CONST_VTBL struct IXpsOMSignatureBlockResourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMSignatureBlockResource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMSignatureBlockResource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMSignatureBlockResource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMSignatureBlockResource_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMSignatureBlockResource_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 



#define IXpsOMSignatureBlockResource_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMSignatureBlockResource_GetStream(This,stream)	\
    ( (This)->lpVtbl -> GetStream(This,stream) ) 

#define IXpsOMSignatureBlockResource_SetContent(This,sourceStream,partName)	\
    ( (This)->lpVtbl -> SetContent(This,sourceStream,partName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMSignatureBlockResource_INTERFACE_DEFINED__ */


#ifndef __IXpsOMVisualCollection_INTERFACE_DEFINED__
#define __IXpsOMVisualCollection_INTERFACE_DEFINED__

/* interface IXpsOMVisualCollection */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMVisualCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("94D8ABDE-AB91-46A8-82B7-F5B05EF01A96")
    IXpsOMVisualCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisual **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMVisual *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMVisual *object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMVisual *object) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMVisualCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMVisualCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMVisualCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisual **object);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMVisual *object);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMVisual *object);
        
        DECLSPEC_XFGVIRT(IXpsOMVisualCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMVisualCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMVisual *object);
        
        END_INTERFACE
    } IXpsOMVisualCollectionVtbl;

    interface IXpsOMVisualCollection
    {
        CONST_VTBL struct IXpsOMVisualCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMVisualCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMVisualCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMVisualCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMVisualCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMVisualCollection_GetAt(This,index,object)	\
    ( (This)->lpVtbl -> GetAt(This,index,object) ) 

#define IXpsOMVisualCollection_InsertAt(This,index,object)	\
    ( (This)->lpVtbl -> InsertAt(This,index,object) ) 

#define IXpsOMVisualCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMVisualCollection_SetAt(This,index,object)	\
    ( (This)->lpVtbl -> SetAt(This,index,object) ) 

#define IXpsOMVisualCollection_Append(This,object)	\
    ( (This)->lpVtbl -> Append(This,object) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMVisualCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMCanvas_INTERFACE_DEFINED__
#define __IXpsOMCanvas_INTERFACE_DEFINED__

/* interface IXpsOMCanvas */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMCanvas;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("221D1452-331E-47c6-87E9-6CCEFB9B5BA3")
    IXpsOMCanvas : public IXpsOMVisual
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetVisuals( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisualCollection **visuals) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUseAliasedEdgeMode( 
            /* [retval][out] */ __RPC__out BOOL *useAliasedEdgeMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUseAliasedEdgeMode( 
            /* [in] */ BOOL useAliasedEdgeMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAccessibilityShortDescription( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *shortDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccessibilityShortDescription( 
            /* [string][in] */ __RPC__in_string LPCWSTR shortDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAccessibilityLongDescription( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *longDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAccessibilityLongDescription( 
            /* [string][in] */ __RPC__in_string LPCWSTR longDescription) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDictionary( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDictionaryLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDictionaryLocal( 
            /* [in] */ __RPC__in_opt IXpsOMDictionary *resourceDictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDictionaryResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **remoteDictionaryResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDictionaryResource( 
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *remoteDictionaryResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMCanvas **canvas) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMCanvasVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMCanvas * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMCanvas * This);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **owner);
        
        DECLSPEC_XFGVIRT(IXpsOMShareable, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__out XPS_OBJECT_TYPE *type);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMMatrixTransform **matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLocal)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in_opt IXpsOMMatrixTransform *matrixTransform);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *GetTransformLookup )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetTransformLookup)
        HRESULT ( STDMETHODCALLTYPE *SetTransformLookup )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometry)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometry )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMGeometry **clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in_opt IXpsOMGeometry *clipGeometry);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *GetClipGeometryLookup )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetClipGeometryLookup)
        HRESULT ( STDMETHODCALLTYPE *SetClipGeometryLookup )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacity)
        HRESULT ( STDMETHODCALLTYPE *GetOpacity )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__out FLOAT *opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacity)
        HRESULT ( STDMETHODCALLTYPE *SetOpacity )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ FLOAT opacity);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrush)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrush )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMBrush **opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLocal)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in_opt IXpsOMBrush *opacityMaskBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *GetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetOpacityMaskBrushLookup)
        HRESULT ( STDMETHODCALLTYPE *SetOpacityMaskBrushLookup )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR key);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR name);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *GetIsHyperlinkTarget )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__out BOOL *isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *SetIsHyperlinkTarget )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ BOOL isHyperlink);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *GetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IUri **hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetHyperlinkNavigateUri)
        HRESULT ( STDMETHODCALLTYPE *SetHyperlinkNavigateUri )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in_opt IUri *hyperlinkUri);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMVisual, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR language);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetVisuals)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetVisuals )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisualCollection **visuals);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetUseAliasedEdgeMode)
        HRESULT ( STDMETHODCALLTYPE *GetUseAliasedEdgeMode )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__out BOOL *useAliasedEdgeMode);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, SetUseAliasedEdgeMode)
        HRESULT ( STDMETHODCALLTYPE *SetUseAliasedEdgeMode )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ BOOL useAliasedEdgeMode);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetAccessibilityShortDescription)
        HRESULT ( STDMETHODCALLTYPE *GetAccessibilityShortDescription )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *shortDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, SetAccessibilityShortDescription)
        HRESULT ( STDMETHODCALLTYPE *SetAccessibilityShortDescription )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR shortDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetAccessibilityLongDescription)
        HRESULT ( STDMETHODCALLTYPE *GetAccessibilityLongDescription )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *longDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, SetAccessibilityLongDescription)
        HRESULT ( STDMETHODCALLTYPE *SetAccessibilityLongDescription )( 
            __RPC__in IXpsOMCanvas * This,
            /* [string][in] */ __RPC__in_string LPCWSTR longDescription);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetDictionary)
        HRESULT ( STDMETHODCALLTYPE *GetDictionary )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetDictionaryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetDictionaryLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, SetDictionaryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetDictionaryLocal )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in_opt IXpsOMDictionary *resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, GetDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *GetDictionaryResource )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, SetDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *SetDictionaryResource )( 
            __RPC__in IXpsOMCanvas * This,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMCanvas, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMCanvas * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMCanvas **canvas);
        
        END_INTERFACE
    } IXpsOMCanvasVtbl;

    interface IXpsOMCanvas
    {
        CONST_VTBL struct IXpsOMCanvasVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMCanvas_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMCanvas_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMCanvas_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMCanvas_GetOwner(This,owner)	\
    ( (This)->lpVtbl -> GetOwner(This,owner) ) 

#define IXpsOMCanvas_GetType(This,type)	\
    ( (This)->lpVtbl -> GetType(This,type) ) 


#define IXpsOMCanvas_GetTransform(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransform(This,matrixTransform) ) 

#define IXpsOMCanvas_GetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> GetTransformLocal(This,matrixTransform) ) 

#define IXpsOMCanvas_SetTransformLocal(This,matrixTransform)	\
    ( (This)->lpVtbl -> SetTransformLocal(This,matrixTransform) ) 

#define IXpsOMCanvas_GetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> GetTransformLookup(This,key) ) 

#define IXpsOMCanvas_SetTransformLookup(This,key)	\
    ( (This)->lpVtbl -> SetTransformLookup(This,key) ) 

#define IXpsOMCanvas_GetClipGeometry(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometry(This,clipGeometry) ) 

#define IXpsOMCanvas_GetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> GetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMCanvas_SetClipGeometryLocal(This,clipGeometry)	\
    ( (This)->lpVtbl -> SetClipGeometryLocal(This,clipGeometry) ) 

#define IXpsOMCanvas_GetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> GetClipGeometryLookup(This,key) ) 

#define IXpsOMCanvas_SetClipGeometryLookup(This,key)	\
    ( (This)->lpVtbl -> SetClipGeometryLookup(This,key) ) 

#define IXpsOMCanvas_GetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> GetOpacity(This,opacity) ) 

#define IXpsOMCanvas_SetOpacity(This,opacity)	\
    ( (This)->lpVtbl -> SetOpacity(This,opacity) ) 

#define IXpsOMCanvas_GetOpacityMaskBrush(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrush(This,opacityMaskBrush) ) 

#define IXpsOMCanvas_GetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMCanvas_SetOpacityMaskBrushLocal(This,opacityMaskBrush)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLocal(This,opacityMaskBrush) ) 

#define IXpsOMCanvas_GetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> GetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMCanvas_SetOpacityMaskBrushLookup(This,key)	\
    ( (This)->lpVtbl -> SetOpacityMaskBrushLookup(This,key) ) 

#define IXpsOMCanvas_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IXpsOMCanvas_SetName(This,name)	\
    ( (This)->lpVtbl -> SetName(This,name) ) 

#define IXpsOMCanvas_GetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> GetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMCanvas_SetIsHyperlinkTarget(This,isHyperlink)	\
    ( (This)->lpVtbl -> SetIsHyperlinkTarget(This,isHyperlink) ) 

#define IXpsOMCanvas_GetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> GetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMCanvas_SetHyperlinkNavigateUri(This,hyperlinkUri)	\
    ( (This)->lpVtbl -> SetHyperlinkNavigateUri(This,hyperlinkUri) ) 

#define IXpsOMCanvas_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMCanvas_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 


#define IXpsOMCanvas_GetVisuals(This,visuals)	\
    ( (This)->lpVtbl -> GetVisuals(This,visuals) ) 

#define IXpsOMCanvas_GetUseAliasedEdgeMode(This,useAliasedEdgeMode)	\
    ( (This)->lpVtbl -> GetUseAliasedEdgeMode(This,useAliasedEdgeMode) ) 

#define IXpsOMCanvas_SetUseAliasedEdgeMode(This,useAliasedEdgeMode)	\
    ( (This)->lpVtbl -> SetUseAliasedEdgeMode(This,useAliasedEdgeMode) ) 

#define IXpsOMCanvas_GetAccessibilityShortDescription(This,shortDescription)	\
    ( (This)->lpVtbl -> GetAccessibilityShortDescription(This,shortDescription) ) 

#define IXpsOMCanvas_SetAccessibilityShortDescription(This,shortDescription)	\
    ( (This)->lpVtbl -> SetAccessibilityShortDescription(This,shortDescription) ) 

#define IXpsOMCanvas_GetAccessibilityLongDescription(This,longDescription)	\
    ( (This)->lpVtbl -> GetAccessibilityLongDescription(This,longDescription) ) 

#define IXpsOMCanvas_SetAccessibilityLongDescription(This,longDescription)	\
    ( (This)->lpVtbl -> SetAccessibilityLongDescription(This,longDescription) ) 

#define IXpsOMCanvas_GetDictionary(This,resourceDictionary)	\
    ( (This)->lpVtbl -> GetDictionary(This,resourceDictionary) ) 

#define IXpsOMCanvas_GetDictionaryLocal(This,resourceDictionary)	\
    ( (This)->lpVtbl -> GetDictionaryLocal(This,resourceDictionary) ) 

#define IXpsOMCanvas_SetDictionaryLocal(This,resourceDictionary)	\
    ( (This)->lpVtbl -> SetDictionaryLocal(This,resourceDictionary) ) 

#define IXpsOMCanvas_GetDictionaryResource(This,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> GetDictionaryResource(This,remoteDictionaryResource) ) 

#define IXpsOMCanvas_SetDictionaryResource(This,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> SetDictionaryResource(This,remoteDictionaryResource) ) 

#define IXpsOMCanvas_Clone(This,canvas)	\
    ( (This)->lpVtbl -> Clone(This,canvas) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMCanvas_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPage_INTERFACE_DEFINED__
#define __IXpsOMPage_INTERFACE_DEFINED__

/* interface IXpsOMPage */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d3e18888-f120-4fee-8c68-35296eae91d4")
    IXpsOMPage : public IXpsOMPart
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **pageReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVisuals( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisualCollection **visuals) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPageDimensions( 
            /* [retval][out] */ __RPC__out XPS_SIZE *pageDimensions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPageDimensions( 
            /* [in] */ __RPC__in const XPS_SIZE *pageDimensions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentBox( 
            /* [retval][out] */ __RPC__out XPS_RECT *contentBox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContentBox( 
            /* [in] */ __RPC__in const XPS_RECT *contentBox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBleedBox( 
            /* [retval][out] */ __RPC__out XPS_RECT *bleedBox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBleedBox( 
            /* [in] */ __RPC__in const XPS_RECT *bleedBox) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguage( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLanguage( 
            /* [string][in] */ __RPC__in_string LPCWSTR language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetName( 
            /* [string][in] */ __RPC__in_string LPCWSTR name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIsHyperlinkTarget( 
            /* [retval][out] */ __RPC__out BOOL *isHyperlinkTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIsHyperlinkTarget( 
            /* [in] */ BOOL isHyperlinkTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDictionary( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDictionaryLocal( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDictionaryLocal( 
            /* [in] */ __RPC__in_opt IXpsOMDictionary *resourceDictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDictionaryResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **remoteDictionaryResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDictionaryResource( 
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *remoteDictionaryResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Write( 
            /* [in] */ __RPC__in_opt ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateUnusedLookupKey( 
            /* [in] */ XPS_OBJECT_TYPE type,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPage **page) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPage * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetVisuals)
        HRESULT ( STDMETHODCALLTYPE *GetVisuals )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMVisualCollection **visuals);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetPageDimensions)
        HRESULT ( STDMETHODCALLTYPE *GetPageDimensions )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__out XPS_SIZE *pageDimensions);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetPageDimensions)
        HRESULT ( STDMETHODCALLTYPE *SetPageDimensions )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in const XPS_SIZE *pageDimensions);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetContentBox)
        HRESULT ( STDMETHODCALLTYPE *GetContentBox )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__out XPS_RECT *contentBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetContentBox)
        HRESULT ( STDMETHODCALLTYPE *SetContentBox )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in const XPS_RECT *contentBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetBleedBox)
        HRESULT ( STDMETHODCALLTYPE *GetBleedBox )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__out XPS_RECT *bleedBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetBleedBox)
        HRESULT ( STDMETHODCALLTYPE *SetBleedBox )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in const XPS_RECT *bleedBox);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IXpsOMPage * This,
            /* [string][in] */ __RPC__in_string LPCWSTR language);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetName)
        HRESULT ( STDMETHODCALLTYPE *SetName )( 
            __RPC__in IXpsOMPage * This,
            /* [string][in] */ __RPC__in_string LPCWSTR name);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *GetIsHyperlinkTarget )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__out BOOL *isHyperlinkTarget);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetIsHyperlinkTarget)
        HRESULT ( STDMETHODCALLTYPE *SetIsHyperlinkTarget )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ BOOL isHyperlinkTarget);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetDictionary)
        HRESULT ( STDMETHODCALLTYPE *GetDictionary )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetDictionaryLocal)
        HRESULT ( STDMETHODCALLTYPE *GetDictionaryLocal )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDictionary **resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetDictionaryLocal)
        HRESULT ( STDMETHODCALLTYPE *SetDictionaryLocal )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in_opt IXpsOMDictionary *resourceDictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GetDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *GetDictionaryResource )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMRemoteDictionaryResource **remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, SetDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *SetDictionaryResource )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in_opt IXpsOMRemoteDictionaryResource *remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ __RPC__in_opt ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, GenerateUnusedLookupKey)
        HRESULT ( STDMETHODCALLTYPE *GenerateUnusedLookupKey )( 
            __RPC__in IXpsOMPage * This,
            /* [in] */ XPS_OBJECT_TYPE type,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *key);
        
        DECLSPEC_XFGVIRT(IXpsOMPage, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMPage * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPage **page);
        
        END_INTERFACE
    } IXpsOMPageVtbl;

    interface IXpsOMPage
    {
        CONST_VTBL struct IXpsOMPageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPage_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMPage_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 


#define IXpsOMPage_GetOwner(This,pageReference)	\
    ( (This)->lpVtbl -> GetOwner(This,pageReference) ) 

#define IXpsOMPage_GetVisuals(This,visuals)	\
    ( (This)->lpVtbl -> GetVisuals(This,visuals) ) 

#define IXpsOMPage_GetPageDimensions(This,pageDimensions)	\
    ( (This)->lpVtbl -> GetPageDimensions(This,pageDimensions) ) 

#define IXpsOMPage_SetPageDimensions(This,pageDimensions)	\
    ( (This)->lpVtbl -> SetPageDimensions(This,pageDimensions) ) 

#define IXpsOMPage_GetContentBox(This,contentBox)	\
    ( (This)->lpVtbl -> GetContentBox(This,contentBox) ) 

#define IXpsOMPage_SetContentBox(This,contentBox)	\
    ( (This)->lpVtbl -> SetContentBox(This,contentBox) ) 

#define IXpsOMPage_GetBleedBox(This,bleedBox)	\
    ( (This)->lpVtbl -> GetBleedBox(This,bleedBox) ) 

#define IXpsOMPage_SetBleedBox(This,bleedBox)	\
    ( (This)->lpVtbl -> SetBleedBox(This,bleedBox) ) 

#define IXpsOMPage_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMPage_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 

#define IXpsOMPage_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IXpsOMPage_SetName(This,name)	\
    ( (This)->lpVtbl -> SetName(This,name) ) 

#define IXpsOMPage_GetIsHyperlinkTarget(This,isHyperlinkTarget)	\
    ( (This)->lpVtbl -> GetIsHyperlinkTarget(This,isHyperlinkTarget) ) 

#define IXpsOMPage_SetIsHyperlinkTarget(This,isHyperlinkTarget)	\
    ( (This)->lpVtbl -> SetIsHyperlinkTarget(This,isHyperlinkTarget) ) 

#define IXpsOMPage_GetDictionary(This,resourceDictionary)	\
    ( (This)->lpVtbl -> GetDictionary(This,resourceDictionary) ) 

#define IXpsOMPage_GetDictionaryLocal(This,resourceDictionary)	\
    ( (This)->lpVtbl -> GetDictionaryLocal(This,resourceDictionary) ) 

#define IXpsOMPage_SetDictionaryLocal(This,resourceDictionary)	\
    ( (This)->lpVtbl -> SetDictionaryLocal(This,resourceDictionary) ) 

#define IXpsOMPage_GetDictionaryResource(This,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> GetDictionaryResource(This,remoteDictionaryResource) ) 

#define IXpsOMPage_SetDictionaryResource(This,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> SetDictionaryResource(This,remoteDictionaryResource) ) 

#define IXpsOMPage_Write(This,stream,optimizeMarkupSize)	\
    ( (This)->lpVtbl -> Write(This,stream,optimizeMarkupSize) ) 

#define IXpsOMPage_GenerateUnusedLookupKey(This,type,key)	\
    ( (This)->lpVtbl -> GenerateUnusedLookupKey(This,type,key) ) 

#define IXpsOMPage_Clone(This,page)	\
    ( (This)->lpVtbl -> Clone(This,page) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPage_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPageReference_INTERFACE_DEFINED__
#define __IXpsOMPageReference_INTERFACE_DEFINED__

/* interface IXpsOMPageReference */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPageReference;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ED360180-6F92-4998-890D-2F208531A0A0")
    IXpsOMPageReference : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **document) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetPage( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPage **page) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE SetPage( 
            /* [in] */ __RPC__in_opt IXpsOMPage *page) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DiscardPage( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsPageLoaded( 
            /* [retval][out] */ __RPC__out BOOL *isPageLoaded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAdvisoryPageDimensions( 
            /* [retval][out] */ __RPC__out XPS_SIZE *pageDimensions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAdvisoryPageDimensions( 
            /* [in] */ __RPC__in const XPS_SIZE *pageDimensions) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStoryFragmentsResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMStoryFragmentsResource **storyFragmentsResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStoryFragmentsResource( 
            /* [in] */ __RPC__in_opt IXpsOMStoryFragmentsResource *storyFragmentsResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrintTicketResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPrintTicketResource **printTicketResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrintTicketResource( 
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *printTicketResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThumbnailResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThumbnailResource( 
            /* [in] */ __RPC__in_opt IXpsOMImageResource *imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CollectLinkTargets( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMNameCollection **linkTargets) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CollectPartResources( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPartResources **partResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasRestrictedFonts( 
            /* [retval][out] */ __RPC__out BOOL *restrictedFonts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **pageReference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPageReferenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPageReference * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPageReference * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPageReference * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, GetOwner)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **document);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, GetPage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetPage )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPage **page);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, SetPage)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetPage )( 
            __RPC__in IXpsOMPageReference * This,
            /* [in] */ __RPC__in_opt IXpsOMPage *page);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, DiscardPage)
        HRESULT ( STDMETHODCALLTYPE *DiscardPage )( 
            __RPC__in IXpsOMPageReference * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, IsPageLoaded)
        HRESULT ( STDMETHODCALLTYPE *IsPageLoaded )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__out BOOL *isPageLoaded);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, GetAdvisoryPageDimensions)
        HRESULT ( STDMETHODCALLTYPE *GetAdvisoryPageDimensions )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__out XPS_SIZE *pageDimensions);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, SetAdvisoryPageDimensions)
        HRESULT ( STDMETHODCALLTYPE *SetAdvisoryPageDimensions )( 
            __RPC__in IXpsOMPageReference * This,
            /* [in] */ __RPC__in const XPS_SIZE *pageDimensions);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, GetStoryFragmentsResource)
        HRESULT ( STDMETHODCALLTYPE *GetStoryFragmentsResource )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMStoryFragmentsResource **storyFragmentsResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, SetStoryFragmentsResource)
        HRESULT ( STDMETHODCALLTYPE *SetStoryFragmentsResource )( 
            __RPC__in IXpsOMPageReference * This,
            /* [in] */ __RPC__in_opt IXpsOMStoryFragmentsResource *storyFragmentsResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, GetPrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *GetPrintTicketResource )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPrintTicketResource **printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, SetPrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *SetPrintTicketResource )( 
            __RPC__in IXpsOMPageReference * This,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, GetThumbnailResource)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnailResource )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMImageResource **imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, SetThumbnailResource)
        HRESULT ( STDMETHODCALLTYPE *SetThumbnailResource )( 
            __RPC__in IXpsOMPageReference * This,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, CollectLinkTargets)
        HRESULT ( STDMETHODCALLTYPE *CollectLinkTargets )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMNameCollection **linkTargets);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, CollectPartResources)
        HRESULT ( STDMETHODCALLTYPE *CollectPartResources )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPartResources **partResources);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, HasRestrictedFonts)
        HRESULT ( STDMETHODCALLTYPE *HasRestrictedFonts )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__out BOOL *restrictedFonts);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReference, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMPageReference * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **pageReference);
        
        END_INTERFACE
    } IXpsOMPageReferenceVtbl;

    interface IXpsOMPageReference
    {
        CONST_VTBL struct IXpsOMPageReferenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPageReference_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPageReference_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPageReference_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPageReference_GetOwner(This,document)	\
    ( (This)->lpVtbl -> GetOwner(This,document) ) 

#define IXpsOMPageReference_GetPage(This,page)	\
    ( (This)->lpVtbl -> GetPage(This,page) ) 

#define IXpsOMPageReference_SetPage(This,page)	\
    ( (This)->lpVtbl -> SetPage(This,page) ) 

#define IXpsOMPageReference_DiscardPage(This)	\
    ( (This)->lpVtbl -> DiscardPage(This) ) 

#define IXpsOMPageReference_IsPageLoaded(This,isPageLoaded)	\
    ( (This)->lpVtbl -> IsPageLoaded(This,isPageLoaded) ) 

#define IXpsOMPageReference_GetAdvisoryPageDimensions(This,pageDimensions)	\
    ( (This)->lpVtbl -> GetAdvisoryPageDimensions(This,pageDimensions) ) 

#define IXpsOMPageReference_SetAdvisoryPageDimensions(This,pageDimensions)	\
    ( (This)->lpVtbl -> SetAdvisoryPageDimensions(This,pageDimensions) ) 

#define IXpsOMPageReference_GetStoryFragmentsResource(This,storyFragmentsResource)	\
    ( (This)->lpVtbl -> GetStoryFragmentsResource(This,storyFragmentsResource) ) 

#define IXpsOMPageReference_SetStoryFragmentsResource(This,storyFragmentsResource)	\
    ( (This)->lpVtbl -> SetStoryFragmentsResource(This,storyFragmentsResource) ) 

#define IXpsOMPageReference_GetPrintTicketResource(This,printTicketResource)	\
    ( (This)->lpVtbl -> GetPrintTicketResource(This,printTicketResource) ) 

#define IXpsOMPageReference_SetPrintTicketResource(This,printTicketResource)	\
    ( (This)->lpVtbl -> SetPrintTicketResource(This,printTicketResource) ) 

#define IXpsOMPageReference_GetThumbnailResource(This,imageResource)	\
    ( (This)->lpVtbl -> GetThumbnailResource(This,imageResource) ) 

#define IXpsOMPageReference_SetThumbnailResource(This,imageResource)	\
    ( (This)->lpVtbl -> SetThumbnailResource(This,imageResource) ) 

#define IXpsOMPageReference_CollectLinkTargets(This,linkTargets)	\
    ( (This)->lpVtbl -> CollectLinkTargets(This,linkTargets) ) 

#define IXpsOMPageReference_CollectPartResources(This,partResources)	\
    ( (This)->lpVtbl -> CollectPartResources(This,partResources) ) 

#define IXpsOMPageReference_HasRestrictedFonts(This,restrictedFonts)	\
    ( (This)->lpVtbl -> HasRestrictedFonts(This,restrictedFonts) ) 

#define IXpsOMPageReference_Clone(This,pageReference)	\
    ( (This)->lpVtbl -> Clone(This,pageReference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPageReference_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPageReferenceCollection_INTERFACE_DEFINED__
#define __IXpsOMPageReferenceCollection_INTERFACE_DEFINED__

/* interface IXpsOMPageReferenceCollection */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPageReferenceCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CA16BA4D-E7B9-45C5-958B-F98022473745")
    IXpsOMPageReferenceCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **pageReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMPageReference *pageReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMPageReference *pageReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMPageReference *pageReference) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPageReferenceCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPageReferenceCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPageReferenceCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReferenceCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReferenceCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReference **pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReferenceCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMPageReference *pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReferenceCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReferenceCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMPageReference *pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMPageReferenceCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMPageReferenceCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMPageReference *pageReference);
        
        END_INTERFACE
    } IXpsOMPageReferenceCollectionVtbl;

    interface IXpsOMPageReferenceCollection
    {
        CONST_VTBL struct IXpsOMPageReferenceCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPageReferenceCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPageReferenceCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPageReferenceCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPageReferenceCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMPageReferenceCollection_GetAt(This,index,pageReference)	\
    ( (This)->lpVtbl -> GetAt(This,index,pageReference) ) 

#define IXpsOMPageReferenceCollection_InsertAt(This,index,pageReference)	\
    ( (This)->lpVtbl -> InsertAt(This,index,pageReference) ) 

#define IXpsOMPageReferenceCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMPageReferenceCollection_SetAt(This,index,pageReference)	\
    ( (This)->lpVtbl -> SetAt(This,index,pageReference) ) 

#define IXpsOMPageReferenceCollection_Append(This,pageReference)	\
    ( (This)->lpVtbl -> Append(This,pageReference) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPageReferenceCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMDocument_INTERFACE_DEFINED__
#define __IXpsOMDocument_INTERFACE_DEFINED__

/* interface IXpsOMDocument */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMDocument;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2C2C94CB-AC5F-4254-8EE9-23948309D9F0")
    IXpsOMDocument : public IXpsOMPart
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocumentSequence **documentSequence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPageReferences( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReferenceCollection **pageReferences) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrintTicketResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPrintTicketResource **printTicketResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrintTicketResource( 
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *printTicketResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDocumentStructureResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocumentStructureResource **documentStructureResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocumentStructureResource( 
            /* [in] */ __RPC__in_opt IXpsOMDocumentStructureResource *documentStructureResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSignatureBlockResources( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSignatureBlockResourceCollection **signatureBlockResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **document) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMDocumentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMDocument * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMDocument * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMDocument * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMDocument * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocumentSequence **documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, GetPageReferences)
        HRESULT ( STDMETHODCALLTYPE *GetPageReferences )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPageReferenceCollection **pageReferences);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, GetPrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *GetPrintTicketResource )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPrintTicketResource **printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, SetPrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *SetPrintTicketResource )( 
            __RPC__in IXpsOMDocument * This,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, GetDocumentStructureResource)
        HRESULT ( STDMETHODCALLTYPE *GetDocumentStructureResource )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocumentStructureResource **documentStructureResource);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, SetDocumentStructureResource)
        HRESULT ( STDMETHODCALLTYPE *SetDocumentStructureResource )( 
            __RPC__in IXpsOMDocument * This,
            /* [in] */ __RPC__in_opt IXpsOMDocumentStructureResource *documentStructureResource);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, GetSignatureBlockResources)
        HRESULT ( STDMETHODCALLTYPE *GetSignatureBlockResources )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMSignatureBlockResourceCollection **signatureBlockResources);
        
        DECLSPEC_XFGVIRT(IXpsOMDocument, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMDocument * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **document);
        
        END_INTERFACE
    } IXpsOMDocumentVtbl;

    interface IXpsOMDocument
    {
        CONST_VTBL struct IXpsOMDocumentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMDocument_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMDocument_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMDocument_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMDocument_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMDocument_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 


#define IXpsOMDocument_GetOwner(This,documentSequence)	\
    ( (This)->lpVtbl -> GetOwner(This,documentSequence) ) 

#define IXpsOMDocument_GetPageReferences(This,pageReferences)	\
    ( (This)->lpVtbl -> GetPageReferences(This,pageReferences) ) 

#define IXpsOMDocument_GetPrintTicketResource(This,printTicketResource)	\
    ( (This)->lpVtbl -> GetPrintTicketResource(This,printTicketResource) ) 

#define IXpsOMDocument_SetPrintTicketResource(This,printTicketResource)	\
    ( (This)->lpVtbl -> SetPrintTicketResource(This,printTicketResource) ) 

#define IXpsOMDocument_GetDocumentStructureResource(This,documentStructureResource)	\
    ( (This)->lpVtbl -> GetDocumentStructureResource(This,documentStructureResource) ) 

#define IXpsOMDocument_SetDocumentStructureResource(This,documentStructureResource)	\
    ( (This)->lpVtbl -> SetDocumentStructureResource(This,documentStructureResource) ) 

#define IXpsOMDocument_GetSignatureBlockResources(This,signatureBlockResources)	\
    ( (This)->lpVtbl -> GetSignatureBlockResources(This,signatureBlockResources) ) 

#define IXpsOMDocument_Clone(This,document)	\
    ( (This)->lpVtbl -> Clone(This,document) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMDocument_INTERFACE_DEFINED__ */


#ifndef __IXpsOMDocumentCollection_INTERFACE_DEFINED__
#define __IXpsOMDocumentCollection_INTERFACE_DEFINED__

/* interface IXpsOMDocumentCollection */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMDocumentCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D1C87F0D-E947-4754-8A25-971478F7E83E")
    IXpsOMDocumentCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **document) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMDocument *document) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMDocument *document) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IXpsOMDocument *document) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMDocumentCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMDocumentCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMDocumentCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocument **document);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMDocument *document);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IXpsOMDocument *document);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMDocumentCollection * This,
            /* [in] */ __RPC__in_opt IXpsOMDocument *document);
        
        END_INTERFACE
    } IXpsOMDocumentCollectionVtbl;

    interface IXpsOMDocumentCollection
    {
        CONST_VTBL struct IXpsOMDocumentCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMDocumentCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMDocumentCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMDocumentCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMDocumentCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMDocumentCollection_GetAt(This,index,document)	\
    ( (This)->lpVtbl -> GetAt(This,index,document) ) 

#define IXpsOMDocumentCollection_InsertAt(This,index,document)	\
    ( (This)->lpVtbl -> InsertAt(This,index,document) ) 

#define IXpsOMDocumentCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMDocumentCollection_SetAt(This,index,document)	\
    ( (This)->lpVtbl -> SetAt(This,index,document) ) 

#define IXpsOMDocumentCollection_Append(This,document)	\
    ( (This)->lpVtbl -> Append(This,document) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMDocumentCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMDocumentSequence_INTERFACE_DEFINED__
#define __IXpsOMDocumentSequence_INTERFACE_DEFINED__

/* interface IXpsOMDocumentSequence */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMDocumentSequence;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56492EB4-D8D5-425e-8256-4C2B64AD0264")
    IXpsOMDocumentSequence : public IXpsOMPart
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPackage **package) = 0;
        
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDocuments( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocumentCollection **documents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrintTicketResource( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPrintTicketResource **printTicketResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrintTicketResource( 
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *printTicketResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMDocumentSequenceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMDocumentSequence * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMDocumentSequence * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentSequence, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentSequence, GetDocuments)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDocuments )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMDocumentCollection **documents);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentSequence, GetPrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *GetPrintTicketResource )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPrintTicketResource **printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMDocumentSequence, SetPrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *SetPrintTicketResource )( 
            __RPC__in IXpsOMDocumentSequence * This,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *printTicketResource);
        
        END_INTERFACE
    } IXpsOMDocumentSequenceVtbl;

    interface IXpsOMDocumentSequence
    {
        CONST_VTBL struct IXpsOMDocumentSequenceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMDocumentSequence_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMDocumentSequence_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMDocumentSequence_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMDocumentSequence_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMDocumentSequence_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 


#define IXpsOMDocumentSequence_GetOwner(This,package)	\
    ( (This)->lpVtbl -> GetOwner(This,package) ) 

#define IXpsOMDocumentSequence_GetDocuments(This,documents)	\
    ( (This)->lpVtbl -> GetDocuments(This,documents) ) 

#define IXpsOMDocumentSequence_GetPrintTicketResource(This,printTicketResource)	\
    ( (This)->lpVtbl -> GetPrintTicketResource(This,printTicketResource) ) 

#define IXpsOMDocumentSequence_SetPrintTicketResource(This,printTicketResource)	\
    ( (This)->lpVtbl -> SetPrintTicketResource(This,printTicketResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMDocumentSequence_INTERFACE_DEFINED__ */


#ifndef __IXpsOMCoreProperties_INTERFACE_DEFINED__
#define __IXpsOMCoreProperties_INTERFACE_DEFINED__

/* interface IXpsOMCoreProperties */
/* [ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMCoreProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3340FE8F-4027-4aa1-8F5F-D35AE45FE597")
    IXpsOMCoreProperties : public IXpsOMPart
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOwner( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPackage **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCategory( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *category) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCategory( 
            /* [string][in] */ __RPC__in_string LPCWSTR category) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentStatus( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContentStatus( 
            /* [string][in] */ __RPC__in_string LPCWSTR contentStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentType( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContentType( 
            /* [string][in] */ __RPC__in_string LPCWSTR contentType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCreated( 
            /* [retval][out] */ __RPC__out SYSTEMTIME *created) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCreated( 
            /* [in] */ __RPC__in const SYSTEMTIME *created) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCreator( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *creator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCreator( 
            /* [string][in] */ __RPC__in_string LPCWSTR creator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDescription( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *description) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDescription( 
            /* [string][in] */ __RPC__in_string LPCWSTR description) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIdentifier( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *identifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIdentifier( 
            /* [string][in] */ __RPC__in_string LPCWSTR identifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeywords( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *keywords) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetKeywords( 
            /* [string][in] */ __RPC__in_string LPCWSTR keywords) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguage( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLanguage( 
            /* [string][in] */ __RPC__in_string LPCWSTR language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastModifiedBy( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lastModifiedBy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLastModifiedBy( 
            /* [string][in] */ __RPC__in_string LPCWSTR lastModifiedBy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLastPrinted( 
            /* [retval][out] */ __RPC__out SYSTEMTIME *lastPrinted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLastPrinted( 
            /* [in] */ __RPC__in const SYSTEMTIME *lastPrinted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetModified( 
            /* [retval][out] */ __RPC__out SYSTEMTIME *modified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetModified( 
            /* [in] */ __RPC__in const SYSTEMTIME *modified) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRevision( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *revision) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRevision( 
            /* [string][in] */ __RPC__in_string LPCWSTR revision) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubject( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *subject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSubject( 
            /* [string][in] */ __RPC__in_string LPCWSTR subject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTitle( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *title) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTitle( 
            /* [string][in] */ __RPC__in_string LPCWSTR title) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVersion( 
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *version) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVersion( 
            /* [string][in] */ __RPC__in_string LPCWSTR version) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMCoreProperties **coreProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMCorePropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMCoreProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMCoreProperties * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, GetPartName)
        HRESULT ( STDMETHODCALLTYPE *GetPartName )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPart, SetPartName)
        HRESULT ( STDMETHODCALLTYPE *SetPartName )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetOwner)
        HRESULT ( STDMETHODCALLTYPE *GetOwner )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetCategory)
        HRESULT ( STDMETHODCALLTYPE *GetCategory )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *category);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetCategory)
        HRESULT ( STDMETHODCALLTYPE *SetCategory )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR category);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetContentStatus)
        HRESULT ( STDMETHODCALLTYPE *GetContentStatus )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentStatus);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetContentStatus)
        HRESULT ( STDMETHODCALLTYPE *SetContentStatus )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR contentStatus);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetContentType)
        HRESULT ( STDMETHODCALLTYPE *GetContentType )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *contentType);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetContentType)
        HRESULT ( STDMETHODCALLTYPE *SetContentType )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR contentType);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetCreated)
        HRESULT ( STDMETHODCALLTYPE *GetCreated )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *created);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetCreated)
        HRESULT ( STDMETHODCALLTYPE *SetCreated )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [in] */ __RPC__in const SYSTEMTIME *created);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetCreator)
        HRESULT ( STDMETHODCALLTYPE *GetCreator )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *creator);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetCreator)
        HRESULT ( STDMETHODCALLTYPE *SetCreator )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR creator);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetDescription)
        HRESULT ( STDMETHODCALLTYPE *GetDescription )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *description);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetDescription)
        HRESULT ( STDMETHODCALLTYPE *SetDescription )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR description);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetIdentifier )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *identifier);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetIdentifier)
        HRESULT ( STDMETHODCALLTYPE *SetIdentifier )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR identifier);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetKeywords)
        HRESULT ( STDMETHODCALLTYPE *GetKeywords )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *keywords);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetKeywords)
        HRESULT ( STDMETHODCALLTYPE *SetKeywords )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR keywords);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR language);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetLastModifiedBy)
        HRESULT ( STDMETHODCALLTYPE *GetLastModifiedBy )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *lastModifiedBy);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetLastModifiedBy)
        HRESULT ( STDMETHODCALLTYPE *SetLastModifiedBy )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR lastModifiedBy);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetLastPrinted)
        HRESULT ( STDMETHODCALLTYPE *GetLastPrinted )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *lastPrinted);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetLastPrinted)
        HRESULT ( STDMETHODCALLTYPE *SetLastPrinted )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [in] */ __RPC__in const SYSTEMTIME *lastPrinted);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetModified)
        HRESULT ( STDMETHODCALLTYPE *GetModified )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][out] */ __RPC__out SYSTEMTIME *modified);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetModified)
        HRESULT ( STDMETHODCALLTYPE *SetModified )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [in] */ __RPC__in const SYSTEMTIME *modified);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetRevision)
        HRESULT ( STDMETHODCALLTYPE *GetRevision )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *revision);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetRevision)
        HRESULT ( STDMETHODCALLTYPE *SetRevision )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR revision);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetSubject)
        HRESULT ( STDMETHODCALLTYPE *GetSubject )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *subject);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetSubject)
        HRESULT ( STDMETHODCALLTYPE *SetSubject )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR subject);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetTitle)
        HRESULT ( STDMETHODCALLTYPE *GetTitle )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *title);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetTitle)
        HRESULT ( STDMETHODCALLTYPE *SetTitle )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR title);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, GetVersion)
        HRESULT ( STDMETHODCALLTYPE *GetVersion )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *version);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, SetVersion)
        HRESULT ( STDMETHODCALLTYPE *SetVersion )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [string][in] */ __RPC__in_string LPCWSTR version);
        
        DECLSPEC_XFGVIRT(IXpsOMCoreProperties, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IXpsOMCoreProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IXpsOMCoreProperties **coreProperties);
        
        END_INTERFACE
    } IXpsOMCorePropertiesVtbl;

    interface IXpsOMCoreProperties
    {
        CONST_VTBL struct IXpsOMCorePropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMCoreProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMCoreProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMCoreProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMCoreProperties_GetPartName(This,partUri)	\
    ( (This)->lpVtbl -> GetPartName(This,partUri) ) 

#define IXpsOMCoreProperties_SetPartName(This,partUri)	\
    ( (This)->lpVtbl -> SetPartName(This,partUri) ) 


#define IXpsOMCoreProperties_GetOwner(This,package)	\
    ( (This)->lpVtbl -> GetOwner(This,package) ) 

#define IXpsOMCoreProperties_GetCategory(This,category)	\
    ( (This)->lpVtbl -> GetCategory(This,category) ) 

#define IXpsOMCoreProperties_SetCategory(This,category)	\
    ( (This)->lpVtbl -> SetCategory(This,category) ) 

#define IXpsOMCoreProperties_GetContentStatus(This,contentStatus)	\
    ( (This)->lpVtbl -> GetContentStatus(This,contentStatus) ) 

#define IXpsOMCoreProperties_SetContentStatus(This,contentStatus)	\
    ( (This)->lpVtbl -> SetContentStatus(This,contentStatus) ) 

#define IXpsOMCoreProperties_GetContentType(This,contentType)	\
    ( (This)->lpVtbl -> GetContentType(This,contentType) ) 

#define IXpsOMCoreProperties_SetContentType(This,contentType)	\
    ( (This)->lpVtbl -> SetContentType(This,contentType) ) 

#define IXpsOMCoreProperties_GetCreated(This,created)	\
    ( (This)->lpVtbl -> GetCreated(This,created) ) 

#define IXpsOMCoreProperties_SetCreated(This,created)	\
    ( (This)->lpVtbl -> SetCreated(This,created) ) 

#define IXpsOMCoreProperties_GetCreator(This,creator)	\
    ( (This)->lpVtbl -> GetCreator(This,creator) ) 

#define IXpsOMCoreProperties_SetCreator(This,creator)	\
    ( (This)->lpVtbl -> SetCreator(This,creator) ) 

#define IXpsOMCoreProperties_GetDescription(This,description)	\
    ( (This)->lpVtbl -> GetDescription(This,description) ) 

#define IXpsOMCoreProperties_SetDescription(This,description)	\
    ( (This)->lpVtbl -> SetDescription(This,description) ) 

#define IXpsOMCoreProperties_GetIdentifier(This,identifier)	\
    ( (This)->lpVtbl -> GetIdentifier(This,identifier) ) 

#define IXpsOMCoreProperties_SetIdentifier(This,identifier)	\
    ( (This)->lpVtbl -> SetIdentifier(This,identifier) ) 

#define IXpsOMCoreProperties_GetKeywords(This,keywords)	\
    ( (This)->lpVtbl -> GetKeywords(This,keywords) ) 

#define IXpsOMCoreProperties_SetKeywords(This,keywords)	\
    ( (This)->lpVtbl -> SetKeywords(This,keywords) ) 

#define IXpsOMCoreProperties_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IXpsOMCoreProperties_SetLanguage(This,language)	\
    ( (This)->lpVtbl -> SetLanguage(This,language) ) 

#define IXpsOMCoreProperties_GetLastModifiedBy(This,lastModifiedBy)	\
    ( (This)->lpVtbl -> GetLastModifiedBy(This,lastModifiedBy) ) 

#define IXpsOMCoreProperties_SetLastModifiedBy(This,lastModifiedBy)	\
    ( (This)->lpVtbl -> SetLastModifiedBy(This,lastModifiedBy) ) 

#define IXpsOMCoreProperties_GetLastPrinted(This,lastPrinted)	\
    ( (This)->lpVtbl -> GetLastPrinted(This,lastPrinted) ) 

#define IXpsOMCoreProperties_SetLastPrinted(This,lastPrinted)	\
    ( (This)->lpVtbl -> SetLastPrinted(This,lastPrinted) ) 

#define IXpsOMCoreProperties_GetModified(This,modified)	\
    ( (This)->lpVtbl -> GetModified(This,modified) ) 

#define IXpsOMCoreProperties_SetModified(This,modified)	\
    ( (This)->lpVtbl -> SetModified(This,modified) ) 

#define IXpsOMCoreProperties_GetRevision(This,revision)	\
    ( (This)->lpVtbl -> GetRevision(This,revision) ) 

#define IXpsOMCoreProperties_SetRevision(This,revision)	\
    ( (This)->lpVtbl -> SetRevision(This,revision) ) 

#define IXpsOMCoreProperties_GetSubject(This,subject)	\
    ( (This)->lpVtbl -> GetSubject(This,subject) ) 

#define IXpsOMCoreProperties_SetSubject(This,subject)	\
    ( (This)->lpVtbl -> SetSubject(This,subject) ) 

#define IXpsOMCoreProperties_GetTitle(This,title)	\
    ( (This)->lpVtbl -> GetTitle(This,title) ) 

#define IXpsOMCoreProperties_SetTitle(This,title)	\
    ( (This)->lpVtbl -> SetTitle(This,title) ) 

#define IXpsOMCoreProperties_GetVersion(This,version)	\
    ( (This)->lpVtbl -> GetVersion(This,version) ) 

#define IXpsOMCoreProperties_SetVersion(This,version)	\
    ( (This)->lpVtbl -> SetVersion(This,version) ) 

#define IXpsOMCoreProperties_Clone(This,coreProperties)	\
    ( (This)->lpVtbl -> Clone(This,coreProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMCoreProperties_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPackage_INTERFACE_DEFINED__
#define __IXpsOMPackage_INTERFACE_DEFINED__

/* interface IXpsOMPackage */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPackage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("18C3DF65-81E1-4674-91DC-FC452F5A416F")
    IXpsOMPackage : public IUnknown
    {
    public:
        virtual /* [helpstring] */ HRESULT STDMETHODCALLTYPE GetDocumentSequence( 
            /* [retval][out] */ IXpsOMDocumentSequence **documentSequence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDocumentSequence( 
            /* [in] */ IXpsOMDocumentSequence *documentSequence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCoreProperties( 
            /* [retval][out] */ IXpsOMCoreProperties **coreProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCoreProperties( 
            /* [in] */ IXpsOMCoreProperties *coreProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDiscardControlPartName( 
            /* [retval][out] */ IOpcPartUri **discardControlPartUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDiscardControlPartName( 
            /* [in] */ IOpcPartUri *discardControlPartUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetThumbnailResource( 
            /* [retval][out] */ IXpsOMImageResource **imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetThumbnailResource( 
            /* [in] */ IXpsOMImageResource *imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteToFile( 
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteToStream( 
            /* [in] */ ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPackageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMPackage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMPackage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMPackage * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetDocumentSequence)
        /* [helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetDocumentSequence )( 
            IXpsOMPackage * This,
            /* [retval][out] */ IXpsOMDocumentSequence **documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetDocumentSequence)
        HRESULT ( STDMETHODCALLTYPE *SetDocumentSequence )( 
            IXpsOMPackage * This,
            /* [in] */ IXpsOMDocumentSequence *documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetCoreProperties)
        HRESULT ( STDMETHODCALLTYPE *GetCoreProperties )( 
            IXpsOMPackage * This,
            /* [retval][out] */ IXpsOMCoreProperties **coreProperties);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetCoreProperties)
        HRESULT ( STDMETHODCALLTYPE *SetCoreProperties )( 
            IXpsOMPackage * This,
            /* [in] */ IXpsOMCoreProperties *coreProperties);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetDiscardControlPartName)
        HRESULT ( STDMETHODCALLTYPE *GetDiscardControlPartName )( 
            IXpsOMPackage * This,
            /* [retval][out] */ IOpcPartUri **discardControlPartUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetDiscardControlPartName)
        HRESULT ( STDMETHODCALLTYPE *SetDiscardControlPartName )( 
            IXpsOMPackage * This,
            /* [in] */ IOpcPartUri *discardControlPartUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, GetThumbnailResource)
        HRESULT ( STDMETHODCALLTYPE *GetThumbnailResource )( 
            IXpsOMPackage * This,
            /* [retval][out] */ IXpsOMImageResource **imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, SetThumbnailResource)
        HRESULT ( STDMETHODCALLTYPE *SetThumbnailResource )( 
            IXpsOMPackage * This,
            /* [in] */ IXpsOMImageResource *imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, WriteToFile)
        HRESULT ( STDMETHODCALLTYPE *WriteToFile )( 
            IXpsOMPackage * This,
            /* [string][in] */ LPCWSTR fileName,
            /* [unique][in] */ LPSECURITY_ATTRIBUTES securityAttributes,
            /* [in] */ DWORD flagsAndAttributes,
            /* [in] */ BOOL optimizeMarkupSize);
        
        DECLSPEC_XFGVIRT(IXpsOMPackage, WriteToStream)
        HRESULT ( STDMETHODCALLTYPE *WriteToStream )( 
            IXpsOMPackage * This,
            /* [in] */ ISequentialStream *stream,
            /* [in] */ BOOL optimizeMarkupSize);
        
        END_INTERFACE
    } IXpsOMPackageVtbl;

    interface IXpsOMPackage
    {
        CONST_VTBL struct IXpsOMPackageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPackage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPackage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPackage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPackage_GetDocumentSequence(This,documentSequence)	\
    ( (This)->lpVtbl -> GetDocumentSequence(This,documentSequence) ) 

#define IXpsOMPackage_SetDocumentSequence(This,documentSequence)	\
    ( (This)->lpVtbl -> SetDocumentSequence(This,documentSequence) ) 

#define IXpsOMPackage_GetCoreProperties(This,coreProperties)	\
    ( (This)->lpVtbl -> GetCoreProperties(This,coreProperties) ) 

#define IXpsOMPackage_SetCoreProperties(This,coreProperties)	\
    ( (This)->lpVtbl -> SetCoreProperties(This,coreProperties) ) 

#define IXpsOMPackage_GetDiscardControlPartName(This,discardControlPartUri)	\
    ( (This)->lpVtbl -> GetDiscardControlPartName(This,discardControlPartUri) ) 

#define IXpsOMPackage_SetDiscardControlPartName(This,discardControlPartUri)	\
    ( (This)->lpVtbl -> SetDiscardControlPartName(This,discardControlPartUri) ) 

#define IXpsOMPackage_GetThumbnailResource(This,imageResource)	\
    ( (This)->lpVtbl -> GetThumbnailResource(This,imageResource) ) 

#define IXpsOMPackage_SetThumbnailResource(This,imageResource)	\
    ( (This)->lpVtbl -> SetThumbnailResource(This,imageResource) ) 

#define IXpsOMPackage_WriteToFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize)	\
    ( (This)->lpVtbl -> WriteToFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize) ) 

#define IXpsOMPackage_WriteToStream(This,stream,optimizeMarkupSize)	\
    ( (This)->lpVtbl -> WriteToStream(This,stream,optimizeMarkupSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPackage_INTERFACE_DEFINED__ */


#ifndef __IXpsOMObjectFactory_INTERFACE_DEFINED__
#define __IXpsOMObjectFactory_INTERFACE_DEFINED__

/* interface IXpsOMObjectFactory */
/* [local][ref][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMObjectFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f9b2a685-a50d-4fc2-b764-b56e093ea0ca")
    IXpsOMObjectFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreatePackage( 
            /* [retval][out] */ IXpsOMPackage **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageFromFile( 
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageFromStream( 
            /* [in] */ IStream *stream,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage **package) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateStoryFragmentsResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMStoryFragmentsResource **storyFragmentsResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDocumentStructureResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocumentStructureResource **documentStructureResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSignatureBlockResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMSignatureBlockResource **signatureBlockResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRemoteDictionaryResource( 
            /* [in] */ IXpsOMDictionary *dictionary,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **remoteDictionaryResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRemoteDictionaryResourceFromStream( 
            /* [in] */ IStream *dictionaryMarkupStream,
            /* [in] */ IOpcPartUri *dictionaryPartUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **dictionaryResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePartResources( 
            /* [retval][out] */ IXpsOMPartResources **partResources) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDocumentSequence( 
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocumentSequence **documentSequence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDocument( 
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocument **document) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePageReference( 
            /* [in] */ const XPS_SIZE *advisoryPageDimensions,
            /* [retval][out] */ IXpsOMPageReference **pageReference) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePage( 
            /* [in] */ const XPS_SIZE *pageDimensions,
            /* [string][in] */ LPCWSTR language,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPage **page) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePageFromStream( 
            /* [in] */ IStream *pageMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPage **page) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCanvas( 
            /* [retval][out] */ IXpsOMCanvas **canvas) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGlyphs( 
            /* [in] */ IXpsOMFontResource *fontResource,
            /* [retval][out] */ IXpsOMGlyphs **glyphs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePath( 
            /* [retval][out] */ IXpsOMPath **path) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGeometry( 
            /* [retval][out] */ IXpsOMGeometry **geometry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryFigure( 
            /* [in] */ const XPS_POINT *startPoint,
            /* [retval][out] */ IXpsOMGeometryFigure **figure) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateMatrixTransform( 
            /* [in] */ const XPS_MATRIX *matrix,
            /* [retval][out] */ IXpsOMMatrixTransform **transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateSolidColorBrush( 
            /* [in] */ const XPS_COLOR *color,
            /* [in] */ IXpsOMColorProfileResource *colorProfile,
            /* [retval][out] */ IXpsOMSolidColorBrush **solidColorBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateColorProfileResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMColorProfileResource **colorProfileResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateImageBrush( 
            /* [in] */ IXpsOMImageResource *image,
            /* [in] */ const XPS_RECT *viewBox,
            /* [in] */ const XPS_RECT *viewPort,
            /* [retval][out] */ IXpsOMImageBrush **imageBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateVisualBrush( 
            /* [in] */ const XPS_RECT *viewBox,
            /* [in] */ const XPS_RECT *viewPort,
            /* [retval][out] */ IXpsOMVisualBrush **visualBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateImageResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ XPS_IMAGE_TYPE contentType,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMImageResource **imageResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePrintTicketResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPrintTicketResource **printTicketResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateFontResource( 
            /* [in] */ IStream *acquiredStream,
            /* [in] */ XPS_FONT_EMBEDDING fontEmbedding,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ BOOL isObfSourceStream,
            /* [retval][out] */ IXpsOMFontResource **fontResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGradientStop( 
            /* [in] */ const XPS_COLOR *color,
            /* [in] */ IXpsOMColorProfileResource *colorProfile,
            /* [in] */ FLOAT offset,
            /* [retval][out] */ IXpsOMGradientStop **gradientStop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateLinearGradientBrush( 
            /* [in] */ IXpsOMGradientStop *gradStop1,
            /* [in] */ IXpsOMGradientStop *gradStop2,
            /* [in] */ const XPS_POINT *startPoint,
            /* [in] */ const XPS_POINT *endPoint,
            /* [retval][out] */ IXpsOMLinearGradientBrush **linearGradientBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRadialGradientBrush( 
            /* [in] */ IXpsOMGradientStop *gradStop1,
            /* [in] */ IXpsOMGradientStop *gradStop2,
            /* [in] */ const XPS_POINT *centerPoint,
            /* [in] */ const XPS_POINT *gradientOrigin,
            /* [in] */ const XPS_SIZE *radiiSizes,
            /* [retval][out] */ IXpsOMRadialGradientBrush **radialGradientBrush) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCoreProperties( 
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMCoreProperties **coreProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDictionary( 
            /* [retval][out] */ IXpsOMDictionary **dictionary) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePartUriCollection( 
            /* [retval][out] */ IXpsOMPartUriCollection **partUriCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageWriterOnFile( 
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
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePackageWriterOnStream( 
            /* [in] */ ISequentialStream *outputStream,
            /* [in] */ BOOL optimizeMarkupSize,
            /* [in] */ XPS_INTERLEAVING interleaving,
            /* [in] */ IOpcPartUri *documentSequencePartName,
            /* [in] */ IXpsOMCoreProperties *coreProperties,
            /* [in] */ IXpsOMImageResource *packageThumbnail,
            /* [in] */ IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ IOpcPartUri *discardControlPartName,
            /* [retval][out] */ IXpsOMPackageWriter **packageWriter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePartUri( 
            /* [string][in] */ LPCWSTR uri,
            /* [retval][out] */ IOpcPartUri **partUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateReadOnlyStreamOnFile( 
            /* [string][in] */ LPCWSTR filename,
            /* [retval][out] */ IStream **stream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMObjectFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMObjectFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMObjectFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMObjectFactory * This);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackage)
        HRESULT ( STDMETHODCALLTYPE *CreatePackage )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageFromFile)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageFromFile )( 
            IXpsOMObjectFactory * This,
            /* [string][in] */ LPCWSTR filename,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageFromStream )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *stream,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPackage **package);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateStoryFragmentsResource)
        HRESULT ( STDMETHODCALLTYPE *CreateStoryFragmentsResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMStoryFragmentsResource **storyFragmentsResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDocumentStructureResource)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentStructureResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocumentStructureResource **documentStructureResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateSignatureBlockResource)
        HRESULT ( STDMETHODCALLTYPE *CreateSignatureBlockResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMSignatureBlockResource **signatureBlockResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateRemoteDictionaryResource)
        HRESULT ( STDMETHODCALLTYPE *CreateRemoteDictionaryResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IXpsOMDictionary *dictionary,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **remoteDictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateRemoteDictionaryResourceFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreateRemoteDictionaryResourceFromStream )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *dictionaryMarkupStream,
            /* [in] */ IOpcPartUri *dictionaryPartUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [retval][out] */ IXpsOMRemoteDictionaryResource **dictionaryResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePartResources)
        HRESULT ( STDMETHODCALLTYPE *CreatePartResources )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMPartResources **partResources);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDocumentSequence)
        HRESULT ( STDMETHODCALLTYPE *CreateDocumentSequence )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocumentSequence **documentSequence);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDocument)
        HRESULT ( STDMETHODCALLTYPE *CreateDocument )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMDocument **document);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePageReference)
        HRESULT ( STDMETHODCALLTYPE *CreatePageReference )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_SIZE *advisoryPageDimensions,
            /* [retval][out] */ IXpsOMPageReference **pageReference);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePage)
        HRESULT ( STDMETHODCALLTYPE *CreatePage )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_SIZE *pageDimensions,
            /* [string][in] */ LPCWSTR language,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPage **page);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePageFromStream)
        HRESULT ( STDMETHODCALLTYPE *CreatePageFromStream )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *pageMarkupStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ IXpsOMPartResources *resources,
            /* [in] */ BOOL reuseObjects,
            /* [retval][out] */ IXpsOMPage **page);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateCanvas)
        HRESULT ( STDMETHODCALLTYPE *CreateCanvas )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMCanvas **canvas);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGlyphs)
        HRESULT ( STDMETHODCALLTYPE *CreateGlyphs )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IXpsOMFontResource *fontResource,
            /* [retval][out] */ IXpsOMGlyphs **glyphs);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePath)
        HRESULT ( STDMETHODCALLTYPE *CreatePath )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMPath **path);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGeometry)
        HRESULT ( STDMETHODCALLTYPE *CreateGeometry )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMGeometry **geometry);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGeometryFigure)
        HRESULT ( STDMETHODCALLTYPE *CreateGeometryFigure )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_POINT *startPoint,
            /* [retval][out] */ IXpsOMGeometryFigure **figure);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateMatrixTransform)
        HRESULT ( STDMETHODCALLTYPE *CreateMatrixTransform )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_MATRIX *matrix,
            /* [retval][out] */ IXpsOMMatrixTransform **transform);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateSolidColorBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateSolidColorBrush )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_COLOR *color,
            /* [in] */ IXpsOMColorProfileResource *colorProfile,
            /* [retval][out] */ IXpsOMSolidColorBrush **solidColorBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateColorProfileResource)
        HRESULT ( STDMETHODCALLTYPE *CreateColorProfileResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMColorProfileResource **colorProfileResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateImageBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateImageBrush )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IXpsOMImageResource *image,
            /* [in] */ const XPS_RECT *viewBox,
            /* [in] */ const XPS_RECT *viewPort,
            /* [retval][out] */ IXpsOMImageBrush **imageBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateVisualBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateVisualBrush )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_RECT *viewBox,
            /* [in] */ const XPS_RECT *viewPort,
            /* [retval][out] */ IXpsOMVisualBrush **visualBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateImageResource)
        HRESULT ( STDMETHODCALLTYPE *CreateImageResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ XPS_IMAGE_TYPE contentType,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMImageResource **imageResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePrintTicketResource)
        HRESULT ( STDMETHODCALLTYPE *CreatePrintTicketResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMPrintTicketResource **printTicketResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateFontResource)
        HRESULT ( STDMETHODCALLTYPE *CreateFontResource )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IStream *acquiredStream,
            /* [in] */ XPS_FONT_EMBEDDING fontEmbedding,
            /* [in] */ IOpcPartUri *partUri,
            /* [in] */ BOOL isObfSourceStream,
            /* [retval][out] */ IXpsOMFontResource **fontResource);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateGradientStop)
        HRESULT ( STDMETHODCALLTYPE *CreateGradientStop )( 
            IXpsOMObjectFactory * This,
            /* [in] */ const XPS_COLOR *color,
            /* [in] */ IXpsOMColorProfileResource *colorProfile,
            /* [in] */ FLOAT offset,
            /* [retval][out] */ IXpsOMGradientStop **gradientStop);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateLinearGradientBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateLinearGradientBrush )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IXpsOMGradientStop *gradStop1,
            /* [in] */ IXpsOMGradientStop *gradStop2,
            /* [in] */ const XPS_POINT *startPoint,
            /* [in] */ const XPS_POINT *endPoint,
            /* [retval][out] */ IXpsOMLinearGradientBrush **linearGradientBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateRadialGradientBrush)
        HRESULT ( STDMETHODCALLTYPE *CreateRadialGradientBrush )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IXpsOMGradientStop *gradStop1,
            /* [in] */ IXpsOMGradientStop *gradStop2,
            /* [in] */ const XPS_POINT *centerPoint,
            /* [in] */ const XPS_POINT *gradientOrigin,
            /* [in] */ const XPS_SIZE *radiiSizes,
            /* [retval][out] */ IXpsOMRadialGradientBrush **radialGradientBrush);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateCoreProperties)
        HRESULT ( STDMETHODCALLTYPE *CreateCoreProperties )( 
            IXpsOMObjectFactory * This,
            /* [in] */ IOpcPartUri *partUri,
            /* [retval][out] */ IXpsOMCoreProperties **coreProperties);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateDictionary)
        HRESULT ( STDMETHODCALLTYPE *CreateDictionary )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMDictionary **dictionary);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePartUriCollection)
        HRESULT ( STDMETHODCALLTYPE *CreatePartUriCollection )( 
            IXpsOMObjectFactory * This,
            /* [retval][out] */ IXpsOMPartUriCollection **partUriCollection);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreatePackageWriterOnFile)
        HRESULT ( STDMETHODCALLTYPE *CreatePackageWriterOnFile )( 
            IXpsOMObjectFactory * This,
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
            IXpsOMObjectFactory * This,
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
            IXpsOMObjectFactory * This,
            /* [string][in] */ LPCWSTR uri,
            /* [retval][out] */ IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMObjectFactory, CreateReadOnlyStreamOnFile)
        HRESULT ( STDMETHODCALLTYPE *CreateReadOnlyStreamOnFile )( 
            IXpsOMObjectFactory * This,
            /* [string][in] */ LPCWSTR filename,
            /* [retval][out] */ IStream **stream);
        
        END_INTERFACE
    } IXpsOMObjectFactoryVtbl;

    interface IXpsOMObjectFactory
    {
        CONST_VTBL struct IXpsOMObjectFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMObjectFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMObjectFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMObjectFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMObjectFactory_CreatePackage(This,package)	\
    ( (This)->lpVtbl -> CreatePackage(This,package) ) 

#define IXpsOMObjectFactory_CreatePackageFromFile(This,filename,reuseObjects,package)	\
    ( (This)->lpVtbl -> CreatePackageFromFile(This,filename,reuseObjects,package) ) 

#define IXpsOMObjectFactory_CreatePackageFromStream(This,stream,reuseObjects,package)	\
    ( (This)->lpVtbl -> CreatePackageFromStream(This,stream,reuseObjects,package) ) 

#define IXpsOMObjectFactory_CreateStoryFragmentsResource(This,acquiredStream,partUri,storyFragmentsResource)	\
    ( (This)->lpVtbl -> CreateStoryFragmentsResource(This,acquiredStream,partUri,storyFragmentsResource) ) 

#define IXpsOMObjectFactory_CreateDocumentStructureResource(This,acquiredStream,partUri,documentStructureResource)	\
    ( (This)->lpVtbl -> CreateDocumentStructureResource(This,acquiredStream,partUri,documentStructureResource) ) 

#define IXpsOMObjectFactory_CreateSignatureBlockResource(This,acquiredStream,partUri,signatureBlockResource)	\
    ( (This)->lpVtbl -> CreateSignatureBlockResource(This,acquiredStream,partUri,signatureBlockResource) ) 

#define IXpsOMObjectFactory_CreateRemoteDictionaryResource(This,dictionary,partUri,remoteDictionaryResource)	\
    ( (This)->lpVtbl -> CreateRemoteDictionaryResource(This,dictionary,partUri,remoteDictionaryResource) ) 

#define IXpsOMObjectFactory_CreateRemoteDictionaryResourceFromStream(This,dictionaryMarkupStream,dictionaryPartUri,resources,dictionaryResource)	\
    ( (This)->lpVtbl -> CreateRemoteDictionaryResourceFromStream(This,dictionaryMarkupStream,dictionaryPartUri,resources,dictionaryResource) ) 

#define IXpsOMObjectFactory_CreatePartResources(This,partResources)	\
    ( (This)->lpVtbl -> CreatePartResources(This,partResources) ) 

#define IXpsOMObjectFactory_CreateDocumentSequence(This,partUri,documentSequence)	\
    ( (This)->lpVtbl -> CreateDocumentSequence(This,partUri,documentSequence) ) 

#define IXpsOMObjectFactory_CreateDocument(This,partUri,document)	\
    ( (This)->lpVtbl -> CreateDocument(This,partUri,document) ) 

#define IXpsOMObjectFactory_CreatePageReference(This,advisoryPageDimensions,pageReference)	\
    ( (This)->lpVtbl -> CreatePageReference(This,advisoryPageDimensions,pageReference) ) 

#define IXpsOMObjectFactory_CreatePage(This,pageDimensions,language,partUri,page)	\
    ( (This)->lpVtbl -> CreatePage(This,pageDimensions,language,partUri,page) ) 

#define IXpsOMObjectFactory_CreatePageFromStream(This,pageMarkupStream,partUri,resources,reuseObjects,page)	\
    ( (This)->lpVtbl -> CreatePageFromStream(This,pageMarkupStream,partUri,resources,reuseObjects,page) ) 

#define IXpsOMObjectFactory_CreateCanvas(This,canvas)	\
    ( (This)->lpVtbl -> CreateCanvas(This,canvas) ) 

#define IXpsOMObjectFactory_CreateGlyphs(This,fontResource,glyphs)	\
    ( (This)->lpVtbl -> CreateGlyphs(This,fontResource,glyphs) ) 

#define IXpsOMObjectFactory_CreatePath(This,path)	\
    ( (This)->lpVtbl -> CreatePath(This,path) ) 

#define IXpsOMObjectFactory_CreateGeometry(This,geometry)	\
    ( (This)->lpVtbl -> CreateGeometry(This,geometry) ) 

#define IXpsOMObjectFactory_CreateGeometryFigure(This,startPoint,figure)	\
    ( (This)->lpVtbl -> CreateGeometryFigure(This,startPoint,figure) ) 

#define IXpsOMObjectFactory_CreateMatrixTransform(This,matrix,transform)	\
    ( (This)->lpVtbl -> CreateMatrixTransform(This,matrix,transform) ) 

#define IXpsOMObjectFactory_CreateSolidColorBrush(This,color,colorProfile,solidColorBrush)	\
    ( (This)->lpVtbl -> CreateSolidColorBrush(This,color,colorProfile,solidColorBrush) ) 

#define IXpsOMObjectFactory_CreateColorProfileResource(This,acquiredStream,partUri,colorProfileResource)	\
    ( (This)->lpVtbl -> CreateColorProfileResource(This,acquiredStream,partUri,colorProfileResource) ) 

#define IXpsOMObjectFactory_CreateImageBrush(This,image,viewBox,viewPort,imageBrush)	\
    ( (This)->lpVtbl -> CreateImageBrush(This,image,viewBox,viewPort,imageBrush) ) 

#define IXpsOMObjectFactory_CreateVisualBrush(This,viewBox,viewPort,visualBrush)	\
    ( (This)->lpVtbl -> CreateVisualBrush(This,viewBox,viewPort,visualBrush) ) 

#define IXpsOMObjectFactory_CreateImageResource(This,acquiredStream,contentType,partUri,imageResource)	\
    ( (This)->lpVtbl -> CreateImageResource(This,acquiredStream,contentType,partUri,imageResource) ) 

#define IXpsOMObjectFactory_CreatePrintTicketResource(This,acquiredStream,partUri,printTicketResource)	\
    ( (This)->lpVtbl -> CreatePrintTicketResource(This,acquiredStream,partUri,printTicketResource) ) 

#define IXpsOMObjectFactory_CreateFontResource(This,acquiredStream,fontEmbedding,partUri,isObfSourceStream,fontResource)	\
    ( (This)->lpVtbl -> CreateFontResource(This,acquiredStream,fontEmbedding,partUri,isObfSourceStream,fontResource) ) 

#define IXpsOMObjectFactory_CreateGradientStop(This,color,colorProfile,offset,gradientStop)	\
    ( (This)->lpVtbl -> CreateGradientStop(This,color,colorProfile,offset,gradientStop) ) 

#define IXpsOMObjectFactory_CreateLinearGradientBrush(This,gradStop1,gradStop2,startPoint,endPoint,linearGradientBrush)	\
    ( (This)->lpVtbl -> CreateLinearGradientBrush(This,gradStop1,gradStop2,startPoint,endPoint,linearGradientBrush) ) 

#define IXpsOMObjectFactory_CreateRadialGradientBrush(This,gradStop1,gradStop2,centerPoint,gradientOrigin,radiiSizes,radialGradientBrush)	\
    ( (This)->lpVtbl -> CreateRadialGradientBrush(This,gradStop1,gradStop2,centerPoint,gradientOrigin,radiiSizes,radialGradientBrush) ) 

#define IXpsOMObjectFactory_CreateCoreProperties(This,partUri,coreProperties)	\
    ( (This)->lpVtbl -> CreateCoreProperties(This,partUri,coreProperties) ) 

#define IXpsOMObjectFactory_CreateDictionary(This,dictionary)	\
    ( (This)->lpVtbl -> CreateDictionary(This,dictionary) ) 

#define IXpsOMObjectFactory_CreatePartUriCollection(This,partUriCollection)	\
    ( (This)->lpVtbl -> CreatePartUriCollection(This,partUriCollection) ) 

#define IXpsOMObjectFactory_CreatePackageWriterOnFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter)	\
    ( (This)->lpVtbl -> CreatePackageWriterOnFile(This,fileName,securityAttributes,flagsAndAttributes,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter) ) 

#define IXpsOMObjectFactory_CreatePackageWriterOnStream(This,outputStream,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter)	\
    ( (This)->lpVtbl -> CreatePackageWriterOnStream(This,outputStream,optimizeMarkupSize,interleaving,documentSequencePartName,coreProperties,packageThumbnail,documentSequencePrintTicket,discardControlPartName,packageWriter) ) 

#define IXpsOMObjectFactory_CreatePartUri(This,uri,partUri)	\
    ( (This)->lpVtbl -> CreatePartUri(This,uri,partUri) ) 

#define IXpsOMObjectFactory_CreateReadOnlyStreamOnFile(This,filename,stream)	\
    ( (This)->lpVtbl -> CreateReadOnlyStreamOnFile(This,filename,stream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMObjectFactory_INTERFACE_DEFINED__ */


#ifndef __IXpsOMNameCollection_INTERFACE_DEFINED__
#define __IXpsOMNameCollection_INTERFACE_DEFINED__

/* interface IXpsOMNameCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMNameCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4BDDF8EC-C915-421B-A166-D173D25653D2")
    IXpsOMNameCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMNameCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMNameCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMNameCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMNameCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMNameCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMNameCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMNameCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMNameCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][string][out] */ __RPC__deref_out_opt_string LPWSTR *name);
        
        END_INTERFACE
    } IXpsOMNameCollectionVtbl;

    interface IXpsOMNameCollection
    {
        CONST_VTBL struct IXpsOMNameCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMNameCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMNameCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMNameCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMNameCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMNameCollection_GetAt(This,index,name)	\
    ( (This)->lpVtbl -> GetAt(This,index,name) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMNameCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPartUriCollection_INTERFACE_DEFINED__
#define __IXpsOMPartUriCollection_INTERFACE_DEFINED__

/* interface IXpsOMPartUriCollection */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPartUriCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("57C650D4-067C-4893-8C33-F62A0633730F")
    IXpsOMPartUriCollection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [retval][out] */ __RPC__out UINT32 *count) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAt( 
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAt( 
            /* [in] */ UINT32 index) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAt( 
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPartUriCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPartUriCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPartUriCollection * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPartUriCollection, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [retval][out] */ __RPC__out UINT32 *count);
        
        DECLSPEC_XFGVIRT(IXpsOMPartUriCollection, GetAt)
        HRESULT ( STDMETHODCALLTYPE *GetAt )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [in] */ UINT32 index,
            /* [retval][out] */ __RPC__deref_out_opt IOpcPartUri **partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPartUriCollection, InsertAt)
        HRESULT ( STDMETHODCALLTYPE *InsertAt )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPartUriCollection, RemoveAt)
        HRESULT ( STDMETHODCALLTYPE *RemoveAt )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [in] */ UINT32 index);
        
        DECLSPEC_XFGVIRT(IXpsOMPartUriCollection, SetAt)
        HRESULT ( STDMETHODCALLTYPE *SetAt )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [in] */ UINT32 index,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        DECLSPEC_XFGVIRT(IXpsOMPartUriCollection, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            __RPC__in IXpsOMPartUriCollection * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *partUri);
        
        END_INTERFACE
    } IXpsOMPartUriCollectionVtbl;

    interface IXpsOMPartUriCollection
    {
        CONST_VTBL struct IXpsOMPartUriCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPartUriCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPartUriCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPartUriCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPartUriCollection_GetCount(This,count)	\
    ( (This)->lpVtbl -> GetCount(This,count) ) 

#define IXpsOMPartUriCollection_GetAt(This,index,partUri)	\
    ( (This)->lpVtbl -> GetAt(This,index,partUri) ) 

#define IXpsOMPartUriCollection_InsertAt(This,index,partUri)	\
    ( (This)->lpVtbl -> InsertAt(This,index,partUri) ) 

#define IXpsOMPartUriCollection_RemoveAt(This,index)	\
    ( (This)->lpVtbl -> RemoveAt(This,index) ) 

#define IXpsOMPartUriCollection_SetAt(This,index,partUri)	\
    ( (This)->lpVtbl -> SetAt(This,index,partUri) ) 

#define IXpsOMPartUriCollection_Append(This,partUri)	\
    ( (This)->lpVtbl -> Append(This,partUri) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPartUriCollection_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPackageWriter_INTERFACE_DEFINED__
#define __IXpsOMPackageWriter_INTERFACE_DEFINED__

/* interface IXpsOMPackageWriter */
/* [uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPackageWriter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4E2AA182-A443-42c6-B41B-4F8E9DE73FF9")
    IXpsOMPackageWriter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartNewDocument( 
            /* [in] */ __RPC__in_opt IOpcPartUri *documentPartName,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *documentPrintTicket,
            /* [in] */ __RPC__in_opt IXpsOMDocumentStructureResource *documentStructure,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResourceCollection *signatureBlockResources,
            /* [in] */ __RPC__in_opt IXpsOMPartUriCollection *restrictedFonts) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddPage( 
            /* [in] */ __RPC__in_opt IXpsOMPage *page,
            /* [in] */ __RPC__in const XPS_SIZE *advisoryPageDimensions,
            /* [in] */ __RPC__in_opt IXpsOMPartUriCollection *discardableResourceParts,
            /* [in] */ __RPC__in_opt IXpsOMStoryFragmentsResource *storyFragments,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *pagePrintTicket,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *pageThumbnail) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddResource( 
            /* [in] */ __RPC__in_opt IXpsOMResource *resource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsClosed( 
            /* [retval][out] */ __RPC__out BOOL *isClosed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPackageWriterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPackageWriter * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPackageWriter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPackageWriter * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, StartNewDocument)
        HRESULT ( STDMETHODCALLTYPE *StartNewDocument )( 
            __RPC__in IXpsOMPackageWriter * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *documentPartName,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *documentPrintTicket,
            /* [in] */ __RPC__in_opt IXpsOMDocumentStructureResource *documentStructure,
            /* [in] */ __RPC__in_opt IXpsOMSignatureBlockResourceCollection *signatureBlockResources,
            /* [in] */ __RPC__in_opt IXpsOMPartUriCollection *restrictedFonts);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, AddPage)
        HRESULT ( STDMETHODCALLTYPE *AddPage )( 
            __RPC__in IXpsOMPackageWriter * This,
            /* [in] */ __RPC__in_opt IXpsOMPage *page,
            /* [in] */ __RPC__in const XPS_SIZE *advisoryPageDimensions,
            /* [in] */ __RPC__in_opt IXpsOMPartUriCollection *discardableResourceParts,
            /* [in] */ __RPC__in_opt IXpsOMStoryFragmentsResource *storyFragments,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *pagePrintTicket,
            /* [in] */ __RPC__in_opt IXpsOMImageResource *pageThumbnail);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, AddResource)
        HRESULT ( STDMETHODCALLTYPE *AddResource )( 
            __RPC__in IXpsOMPackageWriter * This,
            /* [in] */ __RPC__in_opt IXpsOMResource *resource);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IXpsOMPackageWriter * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageWriter, IsClosed)
        HRESULT ( STDMETHODCALLTYPE *IsClosed )( 
            __RPC__in IXpsOMPackageWriter * This,
            /* [retval][out] */ __RPC__out BOOL *isClosed);
        
        END_INTERFACE
    } IXpsOMPackageWriterVtbl;

    interface IXpsOMPackageWriter
    {
        CONST_VTBL struct IXpsOMPackageWriterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPackageWriter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPackageWriter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPackageWriter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPackageWriter_StartNewDocument(This,documentPartName,documentPrintTicket,documentStructure,signatureBlockResources,restrictedFonts)	\
    ( (This)->lpVtbl -> StartNewDocument(This,documentPartName,documentPrintTicket,documentStructure,signatureBlockResources,restrictedFonts) ) 

#define IXpsOMPackageWriter_AddPage(This,page,advisoryPageDimensions,discardableResourceParts,storyFragments,pagePrintTicket,pageThumbnail)	\
    ( (This)->lpVtbl -> AddPage(This,page,advisoryPageDimensions,discardableResourceParts,storyFragments,pagePrintTicket,pageThumbnail) ) 

#define IXpsOMPackageWriter_AddResource(This,resource)	\
    ( (This)->lpVtbl -> AddResource(This,resource) ) 

#define IXpsOMPackageWriter_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IXpsOMPackageWriter_IsClosed(This,isClosed)	\
    ( (This)->lpVtbl -> IsClosed(This,isClosed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPackageWriter_INTERFACE_DEFINED__ */


#ifndef __IXpsOMPackageTarget_INTERFACE_DEFINED__
#define __IXpsOMPackageTarget_INTERFACE_DEFINED__

/* interface IXpsOMPackageTarget */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMPackageTarget;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("219A9DB0-4959-47D0-8034-B1CE84F41A4D")
    IXpsOMPackageTarget : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateXpsOMPackageWriter( 
            /* [in] */ __RPC__in_opt IOpcPartUri *documentSequencePartName,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ __RPC__in_opt IOpcPartUri *discardControlPartName,
            /* [out] */ __RPC__deref_out_opt IXpsOMPackageWriter **packageWriter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMPackageTargetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IXpsOMPackageTarget * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IXpsOMPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IXpsOMPackageTarget * This);
        
        DECLSPEC_XFGVIRT(IXpsOMPackageTarget, CreateXpsOMPackageWriter)
        HRESULT ( STDMETHODCALLTYPE *CreateXpsOMPackageWriter )( 
            __RPC__in IXpsOMPackageTarget * This,
            /* [in] */ __RPC__in_opt IOpcPartUri *documentSequencePartName,
            /* [in] */ __RPC__in_opt IXpsOMPrintTicketResource *documentSequencePrintTicket,
            /* [in] */ __RPC__in_opt IOpcPartUri *discardControlPartName,
            /* [out] */ __RPC__deref_out_opt IXpsOMPackageWriter **packageWriter);
        
        END_INTERFACE
    } IXpsOMPackageTargetVtbl;

    interface IXpsOMPackageTarget
    {
        CONST_VTBL struct IXpsOMPackageTargetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMPackageTarget_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMPackageTarget_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMPackageTarget_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMPackageTarget_CreateXpsOMPackageWriter(This,documentSequencePartName,documentSequencePrintTicket,discardControlPartName,packageWriter)	\
    ( (This)->lpVtbl -> CreateXpsOMPackageWriter(This,documentSequencePartName,documentSequencePrintTicket,discardControlPartName,packageWriter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMPackageTarget_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsobjectmodel_0000_0052 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0052_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0052_v0_0_s_ifspec;

#ifndef __IXpsOMThumbnailGenerator_INTERFACE_DEFINED__
#define __IXpsOMThumbnailGenerator_INTERFACE_DEFINED__

/* interface IXpsOMThumbnailGenerator */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IXpsOMThumbnailGenerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("15B873D5-1971-41E8-83A3-6578403064C7")
    IXpsOMThumbnailGenerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GenerateThumbnail( 
            /* [in] */ IXpsOMPage *page,
            /* [in] */ XPS_IMAGE_TYPE thumbnailType,
            /* [in] */ XPS_THUMBNAIL_SIZE thumbnailSize,
            /* [in] */ IOpcPartUri *imageResourcePartName,
            /* [retval][out] */ IXpsOMImageResource **imageResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IXpsOMThumbnailGeneratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IXpsOMThumbnailGenerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IXpsOMThumbnailGenerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IXpsOMThumbnailGenerator * This);
        
        DECLSPEC_XFGVIRT(IXpsOMThumbnailGenerator, GenerateThumbnail)
        HRESULT ( STDMETHODCALLTYPE *GenerateThumbnail )( 
            IXpsOMThumbnailGenerator * This,
            /* [in] */ IXpsOMPage *page,
            /* [in] */ XPS_IMAGE_TYPE thumbnailType,
            /* [in] */ XPS_THUMBNAIL_SIZE thumbnailSize,
            /* [in] */ IOpcPartUri *imageResourcePartName,
            /* [retval][out] */ IXpsOMImageResource **imageResource);
        
        END_INTERFACE
    } IXpsOMThumbnailGeneratorVtbl;

    interface IXpsOMThumbnailGenerator
    {
        CONST_VTBL struct IXpsOMThumbnailGeneratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IXpsOMThumbnailGenerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IXpsOMThumbnailGenerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IXpsOMThumbnailGenerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IXpsOMThumbnailGenerator_GenerateThumbnail(This,page,thumbnailType,thumbnailSize,imageResourcePartName,imageResource)	\
    ( (This)->lpVtbl -> GenerateThumbnail(This,page,thumbnailType,thumbnailSize,imageResourcePartName,imageResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IXpsOMThumbnailGenerator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_xpsobjectmodel_0000_0053 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0053_v0_0_s_ifspec;


#ifndef __MSXPS_LIBRARY_DEFINED__
#define __MSXPS_LIBRARY_DEFINED__

/* library MSXPS */
/* [uuid] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

EXTERN_C const IID LIBID_MSXPS;

EXTERN_C const CLSID CLSID_XpsOMObjectFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("E974D26D-3D9B-4D47-88CC-3872F2DC3585")
XpsOMObjectFactory;
#endif

EXTERN_C const CLSID CLSID_XpsOMThumbnailGenerator;

#ifdef __cplusplus

class DECLSPEC_UUID("7E4A23E2-B969-4761-BE35-1A8CED58E323")
XpsOMThumbnailGenerator;
#endif
#endif /* __MSXPS_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_xpsobjectmodel_0000_0054 */
/* [local] */ 

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#define XPS_E_INVALID_LANGUAGE                           MAKE_HRESULT(1, FACILITY_XPS, 0x000) 
#define XPS_E_INVALID_NAME                               MAKE_HRESULT(1, FACILITY_XPS, 0x001) 
#define XPS_E_INVALID_RESOURCE_KEY                       MAKE_HRESULT(1, FACILITY_XPS, 0x002) 
#define XPS_E_INVALID_PAGE_SIZE                          MAKE_HRESULT(1, FACILITY_XPS, 0x003) 
#define XPS_E_INVALID_BLEED_BOX                          MAKE_HRESULT(1, FACILITY_XPS, 0x004) 
#define XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE               MAKE_HRESULT(1, FACILITY_XPS, 0x005) 
#define XPS_E_INVALID_LOOKUP_TYPE                        MAKE_HRESULT(1, FACILITY_XPS, 0x006) 
#define XPS_E_INVALID_FLOAT                              MAKE_HRESULT(1, FACILITY_XPS, 0x007) 
#define XPS_E_UNEXPECTED_CONTENT_TYPE                    MAKE_HRESULT(1, FACILITY_XPS, 0x008) 
#define XPS_E_INVALID_FONT_URI                           MAKE_HRESULT(1, FACILITY_XPS, 0x00a) 
#define XPS_E_INVALID_CONTENT_BOX                        MAKE_HRESULT(1, FACILITY_XPS, 0x00b) 
#define XPS_E_INVALID_MARKUP                             MAKE_HRESULT(1, FACILITY_XPS, 0x00c) 
#define XPS_E_INVALID_XML_ENCODING                       MAKE_HRESULT(1, FACILITY_XPS, 0x00d) 
#define XPS_E_INVALID_CONTENT_TYPE                       MAKE_HRESULT(1, FACILITY_XPS, 0x00e) 
#define XPS_E_INVALID_OBFUSCATED_FONT_URI                MAKE_HRESULT(1, FACILITY_XPS, 0x00f) 
#define XPS_E_UNEXPECTED_RELATIONSHIP_TYPE               MAKE_HRESULT(1, FACILITY_XPS, 0x010) 
#define XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP    MAKE_HRESULT(1, FACILITY_XPS, 0x011) 
#define XPS_E_MISSING_NAME                               MAKE_HRESULT(1, FACILITY_XPS, 0x100) 
#define XPS_E_MISSING_LOOKUP                             MAKE_HRESULT(1, FACILITY_XPS, 0x101) 
#define XPS_E_MISSING_GLYPHS                             MAKE_HRESULT(1, FACILITY_XPS, 0x102) 
#define XPS_E_MISSING_SEGMENT_DATA                       MAKE_HRESULT(1, FACILITY_XPS, 0x103) 
#define XPS_E_MISSING_COLORPROFILE                       MAKE_HRESULT(1, FACILITY_XPS, 0x104) 
#define XPS_E_MISSING_RELATIONSHIP_TARGET                MAKE_HRESULT(1, FACILITY_XPS, 0x105) 
#define XPS_E_MISSING_RESOURCE_RELATIONSHIP              MAKE_HRESULT(1, FACILITY_XPS, 0x106) 
#define XPS_E_MISSING_FONTURI                            MAKE_HRESULT(1, FACILITY_XPS, 0x107) 
#define XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP      MAKE_HRESULT(1, FACILITY_XPS, 0x108) 
#define XPS_E_MISSING_DOCUMENT                           MAKE_HRESULT(1, FACILITY_XPS, 0x109) 
#define XPS_E_MISSING_REFERRED_DOCUMENT                  MAKE_HRESULT(1, FACILITY_XPS, 0x10a) 
#define XPS_E_MISSING_REFERRED_PAGE                      MAKE_HRESULT(1, FACILITY_XPS, 0x10b) 
#define XPS_E_MISSING_PAGE_IN_DOCUMENT                   MAKE_HRESULT(1, FACILITY_XPS, 0x10c) 
#define XPS_E_MISSING_PAGE_IN_PAGEREFERENCE              MAKE_HRESULT(1, FACILITY_XPS, 0x10d) 
#define XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH                MAKE_HRESULT(1, FACILITY_XPS, 0x10e) 
#define XPS_E_MISSING_RESOURCE_KEY                       MAKE_HRESULT(1, FACILITY_XPS, 0x10f) 
#define XPS_E_MISSING_PART_REFERENCE                     MAKE_HRESULT(1, FACILITY_XPS, 0x110) 
#define XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP       MAKE_HRESULT(1, FACILITY_XPS, 0x111) 
#define XPS_E_MISSING_DISCARDCONTROL                     MAKE_HRESULT(1, FACILITY_XPS, 0x112) 
#define XPS_E_MISSING_PART_STREAM                        MAKE_HRESULT(1, FACILITY_XPS, 0x113) 
#define XPS_E_UNAVAILABLE_PACKAGE                        MAKE_HRESULT(1, FACILITY_XPS, 0x114) 
#define XPS_E_DUPLICATE_RESOURCE_KEYS                    MAKE_HRESULT(1, FACILITY_XPS, 0x200) 
#define XPS_E_MULTIPLE_RESOURCES                         MAKE_HRESULT(1, FACILITY_XPS, 0x201) 
#define XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS    MAKE_HRESULT(1, FACILITY_XPS, 0x202) 
#define XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE                MAKE_HRESULT(1, FACILITY_XPS, 0x203) 
#define XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE             MAKE_HRESULT(1, FACILITY_XPS, 0x204) 
#define XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE              MAKE_HRESULT(1, FACILITY_XPS, 0x205) 
#define XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT          MAKE_HRESULT(1, FACILITY_XPS, 0x206) 
#define XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE  MAKE_HRESULT(1, FACILITY_XPS, 0x207) 
#define XPS_E_MULTIPLE_REFERENCES_TO_PART                MAKE_HRESULT(1, FACILITY_XPS, 0x208) 
#define XPS_E_DUPLICATE_NAMES                            MAKE_HRESULT(1, FACILITY_XPS, 0x209) 
#define XPS_E_STRING_TOO_LONG                            MAKE_HRESULT(1, FACILITY_XPS, 0x300) 
#define XPS_E_TOO_MANY_INDICES                           MAKE_HRESULT(1, FACILITY_XPS, 0x301) 
#define XPS_E_MAPPING_OUT_OF_ORDER                       MAKE_HRESULT(1, FACILITY_XPS, 0x302) 
#define XPS_E_MAPPING_OUTSIDE_STRING                     MAKE_HRESULT(1, FACILITY_XPS, 0x303) 
#define XPS_E_MAPPING_OUTSIDE_INDICES                    MAKE_HRESULT(1, FACILITY_XPS, 0x304) 
#define XPS_E_CARET_OUTSIDE_STRING                       MAKE_HRESULT(1, FACILITY_XPS, 0x305) 
#define XPS_E_CARET_OUT_OF_ORDER                         MAKE_HRESULT(1, FACILITY_XPS, 0x306) 
#define XPS_E_ODD_BIDILEVEL                              MAKE_HRESULT(1, FACILITY_XPS, 0x307) 
#define XPS_E_ONE_TO_ONE_MAPPING_EXPECTED                MAKE_HRESULT(1, FACILITY_XPS, 0x308) 
#define XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED             MAKE_HRESULT(1, FACILITY_XPS, 0x309) 
#define XPS_E_NEGATIVE_FLOAT                             MAKE_HRESULT(1, FACILITY_XPS, 0x30a) 
#define XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT         MAKE_HRESULT(1, FACILITY_XPS, 0x400) 
#define XPS_E_DICTIONARY_ITEM_NAMED                      MAKE_HRESULT(1, FACILITY_XPS, 0x401) 
#define XPS_E_NESTED_REMOTE_DICTIONARY                   MAKE_HRESULT(1, FACILITY_XPS, 0x402) 
#define XPS_E_INDEX_OUT_OF_RANGE                         MAKE_HRESULT(1, FACILITY_XPS, 0x500) 
#define XPS_E_VISUAL_CIRCULAR_REF                        MAKE_HRESULT(1, FACILITY_XPS, 0x501) 
#define XPS_E_NO_CUSTOM_OBJECTS                          MAKE_HRESULT(1, FACILITY_XPS, 0x502) 
#define XPS_E_ALREADY_OWNED                              MAKE_HRESULT(1, FACILITY_XPS, 0x503) 
#define XPS_E_RESOURCE_NOT_OWNED                         MAKE_HRESULT(1, FACILITY_XPS, 0x504) 
#define XPS_E_UNEXPECTED_COLORPROFILE                    MAKE_HRESULT(1, FACILITY_XPS, 0x505) 
#define XPS_E_COLOR_COMPONENT_OUT_OF_RANGE               MAKE_HRESULT(1, FACILITY_XPS, 0x506) 
#define XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT    MAKE_HRESULT(1, FACILITY_XPS, 0x507) 
#define XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT       MAKE_HRESULT(1, FACILITY_XPS, 0x508) 
#define XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC      MAKE_HRESULT(1, FACILITY_XPS, 0x509) 
#define XPS_E_RELATIONSHIP_EXTERNAL                      MAKE_HRESULT(1, FACILITY_XPS, 0x50a) 
#define XPS_E_NOT_ENOUGH_GRADIENT_STOPS                  MAKE_HRESULT(1, FACILITY_XPS, 0x50b) 
#define XPS_E_PACKAGE_WRITER_NOT_CLOSED                  MAKE_HRESULT(1, FACILITY_XPS, 0x50c) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NTDDI >= NTDDI_WIN7)


extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0054_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_xpsobjectmodel_0000_0054_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


