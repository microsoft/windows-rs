

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


#ifndef __msinkaut_h__
#define __msinkaut_h__

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

#ifndef __IInkRectangle_FWD_DEFINED__
#define __IInkRectangle_FWD_DEFINED__
typedef interface IInkRectangle IInkRectangle;

#endif 	/* __IInkRectangle_FWD_DEFINED__ */


#ifndef __IInkExtendedProperty_FWD_DEFINED__
#define __IInkExtendedProperty_FWD_DEFINED__
typedef interface IInkExtendedProperty IInkExtendedProperty;

#endif 	/* __IInkExtendedProperty_FWD_DEFINED__ */


#ifndef __IInkExtendedProperties_FWD_DEFINED__
#define __IInkExtendedProperties_FWD_DEFINED__
typedef interface IInkExtendedProperties IInkExtendedProperties;

#endif 	/* __IInkExtendedProperties_FWD_DEFINED__ */


#ifndef __IInkDrawingAttributes_FWD_DEFINED__
#define __IInkDrawingAttributes_FWD_DEFINED__
typedef interface IInkDrawingAttributes IInkDrawingAttributes;

#endif 	/* __IInkDrawingAttributes_FWD_DEFINED__ */


#ifndef __IInkTransform_FWD_DEFINED__
#define __IInkTransform_FWD_DEFINED__
typedef interface IInkTransform IInkTransform;

#endif 	/* __IInkTransform_FWD_DEFINED__ */


#ifndef __IInkGesture_FWD_DEFINED__
#define __IInkGesture_FWD_DEFINED__
typedef interface IInkGesture IInkGesture;

#endif 	/* __IInkGesture_FWD_DEFINED__ */


#ifndef __IInkCursor_FWD_DEFINED__
#define __IInkCursor_FWD_DEFINED__
typedef interface IInkCursor IInkCursor;

#endif 	/* __IInkCursor_FWD_DEFINED__ */


#ifndef __IInkCursors_FWD_DEFINED__
#define __IInkCursors_FWD_DEFINED__
typedef interface IInkCursors IInkCursors;

#endif 	/* __IInkCursors_FWD_DEFINED__ */


#ifndef __IInkCursorButton_FWD_DEFINED__
#define __IInkCursorButton_FWD_DEFINED__
typedef interface IInkCursorButton IInkCursorButton;

#endif 	/* __IInkCursorButton_FWD_DEFINED__ */


#ifndef __IInkCursorButtons_FWD_DEFINED__
#define __IInkCursorButtons_FWD_DEFINED__
typedef interface IInkCursorButtons IInkCursorButtons;

#endif 	/* __IInkCursorButtons_FWD_DEFINED__ */


#ifndef __IInkTablet_FWD_DEFINED__
#define __IInkTablet_FWD_DEFINED__
typedef interface IInkTablet IInkTablet;

#endif 	/* __IInkTablet_FWD_DEFINED__ */


#ifndef __IInkTablet2_FWD_DEFINED__
#define __IInkTablet2_FWD_DEFINED__
typedef interface IInkTablet2 IInkTablet2;

#endif 	/* __IInkTablet2_FWD_DEFINED__ */


#ifndef __IInkTablet3_FWD_DEFINED__
#define __IInkTablet3_FWD_DEFINED__
typedef interface IInkTablet3 IInkTablet3;

#endif 	/* __IInkTablet3_FWD_DEFINED__ */


#ifndef __IInkTablets_FWD_DEFINED__
#define __IInkTablets_FWD_DEFINED__
typedef interface IInkTablets IInkTablets;

#endif 	/* __IInkTablets_FWD_DEFINED__ */


#ifndef __IInkStrokeDisp_FWD_DEFINED__
#define __IInkStrokeDisp_FWD_DEFINED__
typedef interface IInkStrokeDisp IInkStrokeDisp;

#endif 	/* __IInkStrokeDisp_FWD_DEFINED__ */


#ifndef __IInkStrokes_FWD_DEFINED__
#define __IInkStrokes_FWD_DEFINED__
typedef interface IInkStrokes IInkStrokes;

#endif 	/* __IInkStrokes_FWD_DEFINED__ */


#ifndef __IInkCustomStrokes_FWD_DEFINED__
#define __IInkCustomStrokes_FWD_DEFINED__
typedef interface IInkCustomStrokes IInkCustomStrokes;

#endif 	/* __IInkCustomStrokes_FWD_DEFINED__ */


#ifndef ___IInkStrokesEvents_FWD_DEFINED__
#define ___IInkStrokesEvents_FWD_DEFINED__
typedef interface _IInkStrokesEvents _IInkStrokesEvents;

#endif 	/* ___IInkStrokesEvents_FWD_DEFINED__ */


#ifndef __IInkDisp_FWD_DEFINED__
#define __IInkDisp_FWD_DEFINED__
typedef interface IInkDisp IInkDisp;

#endif 	/* __IInkDisp_FWD_DEFINED__ */


#ifndef ___IInkEvents_FWD_DEFINED__
#define ___IInkEvents_FWD_DEFINED__
typedef interface _IInkEvents _IInkEvents;

#endif 	/* ___IInkEvents_FWD_DEFINED__ */


#ifndef __IInkRenderer_FWD_DEFINED__
#define __IInkRenderer_FWD_DEFINED__
typedef interface IInkRenderer IInkRenderer;

#endif 	/* __IInkRenderer_FWD_DEFINED__ */


#ifndef __IInkCollector_FWD_DEFINED__
#define __IInkCollector_FWD_DEFINED__
typedef interface IInkCollector IInkCollector;

#endif 	/* __IInkCollector_FWD_DEFINED__ */


#ifndef ___IInkCollectorEvents_FWD_DEFINED__
#define ___IInkCollectorEvents_FWD_DEFINED__
typedef interface _IInkCollectorEvents _IInkCollectorEvents;

#endif 	/* ___IInkCollectorEvents_FWD_DEFINED__ */


#ifndef __IInkOverlay_FWD_DEFINED__
#define __IInkOverlay_FWD_DEFINED__
typedef interface IInkOverlay IInkOverlay;

#endif 	/* __IInkOverlay_FWD_DEFINED__ */


#ifndef ___IInkOverlayEvents_FWD_DEFINED__
#define ___IInkOverlayEvents_FWD_DEFINED__
typedef interface _IInkOverlayEvents _IInkOverlayEvents;

#endif 	/* ___IInkOverlayEvents_FWD_DEFINED__ */


#ifndef __IInkPicture_FWD_DEFINED__
#define __IInkPicture_FWD_DEFINED__
typedef interface IInkPicture IInkPicture;

#endif 	/* __IInkPicture_FWD_DEFINED__ */


#ifndef ___IInkPictureEvents_FWD_DEFINED__
#define ___IInkPictureEvents_FWD_DEFINED__
typedef interface _IInkPictureEvents _IInkPictureEvents;

#endif 	/* ___IInkPictureEvents_FWD_DEFINED__ */


#ifndef __IInkRecognizer_FWD_DEFINED__
#define __IInkRecognizer_FWD_DEFINED__
typedef interface IInkRecognizer IInkRecognizer;

#endif 	/* __IInkRecognizer_FWD_DEFINED__ */


#ifndef __IInkRecognizer2_FWD_DEFINED__
#define __IInkRecognizer2_FWD_DEFINED__
typedef interface IInkRecognizer2 IInkRecognizer2;

#endif 	/* __IInkRecognizer2_FWD_DEFINED__ */


#ifndef __IInkRecognizers_FWD_DEFINED__
#define __IInkRecognizers_FWD_DEFINED__
typedef interface IInkRecognizers IInkRecognizers;

#endif 	/* __IInkRecognizers_FWD_DEFINED__ */


#ifndef ___IInkRecognitionEvents_FWD_DEFINED__
#define ___IInkRecognitionEvents_FWD_DEFINED__
typedef interface _IInkRecognitionEvents _IInkRecognitionEvents;

#endif 	/* ___IInkRecognitionEvents_FWD_DEFINED__ */


#ifndef __IInkRecognizerContext_FWD_DEFINED__
#define __IInkRecognizerContext_FWD_DEFINED__
typedef interface IInkRecognizerContext IInkRecognizerContext;

#endif 	/* __IInkRecognizerContext_FWD_DEFINED__ */


#ifndef __IInkRecognizerContext2_FWD_DEFINED__
#define __IInkRecognizerContext2_FWD_DEFINED__
typedef interface IInkRecognizerContext2 IInkRecognizerContext2;

#endif 	/* __IInkRecognizerContext2_FWD_DEFINED__ */


#ifndef __IInkRecognitionResult_FWD_DEFINED__
#define __IInkRecognitionResult_FWD_DEFINED__
typedef interface IInkRecognitionResult IInkRecognitionResult;

#endif 	/* __IInkRecognitionResult_FWD_DEFINED__ */


#ifndef __IInkRecognitionAlternate_FWD_DEFINED__
#define __IInkRecognitionAlternate_FWD_DEFINED__
typedef interface IInkRecognitionAlternate IInkRecognitionAlternate;

#endif 	/* __IInkRecognitionAlternate_FWD_DEFINED__ */


#ifndef __IInkRecognitionAlternates_FWD_DEFINED__
#define __IInkRecognitionAlternates_FWD_DEFINED__
typedef interface IInkRecognitionAlternates IInkRecognitionAlternates;

#endif 	/* __IInkRecognitionAlternates_FWD_DEFINED__ */


#ifndef __IInkRecognizerGuide_FWD_DEFINED__
#define __IInkRecognizerGuide_FWD_DEFINED__
typedef interface IInkRecognizerGuide IInkRecognizerGuide;

#endif 	/* __IInkRecognizerGuide_FWD_DEFINED__ */


#ifndef __IInkWordList_FWD_DEFINED__
#define __IInkWordList_FWD_DEFINED__
typedef interface IInkWordList IInkWordList;

#endif 	/* __IInkWordList_FWD_DEFINED__ */


#ifndef __IInkWordList2_FWD_DEFINED__
#define __IInkWordList2_FWD_DEFINED__
typedef interface IInkWordList2 IInkWordList2;

#endif 	/* __IInkWordList2_FWD_DEFINED__ */


#ifndef __InkDisp_FWD_DEFINED__
#define __InkDisp_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkDisp InkDisp;
#else
typedef struct InkDisp InkDisp;
#endif /* __cplusplus */

#endif 	/* __InkDisp_FWD_DEFINED__ */


#ifndef __InkOverlay_FWD_DEFINED__
#define __InkOverlay_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkOverlay InkOverlay;
#else
typedef struct InkOverlay InkOverlay;
#endif /* __cplusplus */

#endif 	/* __InkOverlay_FWD_DEFINED__ */


#ifndef __InkPicture_FWD_DEFINED__
#define __InkPicture_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkPicture InkPicture;
#else
typedef struct InkPicture InkPicture;
#endif /* __cplusplus */

#endif 	/* __InkPicture_FWD_DEFINED__ */


#ifndef __InkCollector_FWD_DEFINED__
#define __InkCollector_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkCollector InkCollector;
#else
typedef struct InkCollector InkCollector;
#endif /* __cplusplus */

#endif 	/* __InkCollector_FWD_DEFINED__ */


#ifndef __InkDrawingAttributes_FWD_DEFINED__
#define __InkDrawingAttributes_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkDrawingAttributes InkDrawingAttributes;
#else
typedef struct InkDrawingAttributes InkDrawingAttributes;
#endif /* __cplusplus */

#endif 	/* __InkDrawingAttributes_FWD_DEFINED__ */


#ifndef __InkRectangle_FWD_DEFINED__
#define __InkRectangle_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkRectangle InkRectangle;
#else
typedef struct InkRectangle InkRectangle;
#endif /* __cplusplus */

#endif 	/* __InkRectangle_FWD_DEFINED__ */


#ifndef __InkRenderer_FWD_DEFINED__
#define __InkRenderer_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkRenderer InkRenderer;
#else
typedef struct InkRenderer InkRenderer;
#endif /* __cplusplus */

#endif 	/* __InkRenderer_FWD_DEFINED__ */


#ifndef __InkTransform_FWD_DEFINED__
#define __InkTransform_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkTransform InkTransform;
#else
typedef struct InkTransform InkTransform;
#endif /* __cplusplus */

#endif 	/* __InkTransform_FWD_DEFINED__ */


#ifndef __InkRecognizers_FWD_DEFINED__
#define __InkRecognizers_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkRecognizers InkRecognizers;
#else
typedef struct InkRecognizers InkRecognizers;
#endif /* __cplusplus */

#endif 	/* __InkRecognizers_FWD_DEFINED__ */


#ifndef __InkRecognizerContext_FWD_DEFINED__
#define __InkRecognizerContext_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkRecognizerContext InkRecognizerContext;
#else
typedef struct InkRecognizerContext InkRecognizerContext;
#endif /* __cplusplus */

#endif 	/* __InkRecognizerContext_FWD_DEFINED__ */


#ifndef __InkRecognizerGuide_FWD_DEFINED__
#define __InkRecognizerGuide_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkRecognizerGuide InkRecognizerGuide;
#else
typedef struct InkRecognizerGuide InkRecognizerGuide;
#endif /* __cplusplus */

#endif 	/* __InkRecognizerGuide_FWD_DEFINED__ */


#ifndef __InkTablets_FWD_DEFINED__
#define __InkTablets_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkTablets InkTablets;
#else
typedef struct InkTablets InkTablets;
#endif /* __cplusplus */

#endif 	/* __InkTablets_FWD_DEFINED__ */


#ifndef __InkWordList_FWD_DEFINED__
#define __InkWordList_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkWordList InkWordList;
#else
typedef struct InkWordList InkWordList;
#endif /* __cplusplus */

#endif 	/* __InkWordList_FWD_DEFINED__ */


#ifndef __InkStrokes_FWD_DEFINED__
#define __InkStrokes_FWD_DEFINED__

#ifdef __cplusplus
typedef class InkStrokes InkStrokes;
#else
typedef struct InkStrokes InkStrokes;
#endif /* __cplusplus */

#endif 	/* __InkStrokes_FWD_DEFINED__ */


#ifndef __IInk_FWD_DEFINED__
#define __IInk_FWD_DEFINED__
typedef interface IInk IInk;

#endif 	/* __IInk_FWD_DEFINED__ */


#ifndef __IInkLineInfo_FWD_DEFINED__
#define __IInkLineInfo_FWD_DEFINED__
typedef interface IInkLineInfo IInkLineInfo;

#endif 	/* __IInkLineInfo_FWD_DEFINED__ */


#ifndef __ISketchInk_FWD_DEFINED__
#define __ISketchInk_FWD_DEFINED__
typedef interface ISketchInk ISketchInk;

#endif 	/* __ISketchInk_FWD_DEFINED__ */


#ifndef __Ink_FWD_DEFINED__
#define __Ink_FWD_DEFINED__

#ifdef __cplusplus
typedef class Ink Ink;
#else
typedef struct Ink Ink;
#endif /* __cplusplus */

#endif 	/* __Ink_FWD_DEFINED__ */


#ifndef __SketchInk_FWD_DEFINED__
#define __SketchInk_FWD_DEFINED__

#ifdef __cplusplus
typedef class SketchInk SketchInk;
#else
typedef struct SketchInk SketchInk;
#endif /* __cplusplus */

#endif 	/* __SketchInk_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "tpcshrd.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_msinkaut_0000_0000 */
/* [local] */ 

#ifndef _MSINKAUT_IDL_H_
#define _MSINKAUT_IDL_H_
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <TPCError.h> // for Tablet PC Error codes
typedef 
enum enumINKMETRIC_FLAGS
    {
        IMF_FONT_SELECTED_IN_HDC	= 0x1,
        IMF_ITALIC	= 0x2,
        IMF_BOLD	= 0x4
    } 	INK_METRIC_FLAGS;

typedef 
enum enumGetCandidateFlags
    {
        TCF_ALLOW_RECOGNITION	= 0x1,
        TCF_FORCE_RECOGNITION	= 0x2
    } 	GET_DANDIDATE_FLAGS;

typedef struct tagINKMETRIC
    {
    INT iHeight;
    INT iFontAscent;
    INT iFontDescent;
    DWORD dwFlags;
    COLORREF color;
    } 	INKMETRIC;

typedef struct tagINKMETRIC *PINKMETRIC;














extern RPC_IF_HANDLE __MIDL_itf_msinkaut_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msinkaut_0000_0000_v0_0_s_ifspec;


#ifndef __MSINKAUTLib_LIBRARY_DEFINED__
#define __MSINKAUTLib_LIBRARY_DEFINED__

/* library MSINKAUTLib */
/* [helpcontext][helpstring][version][uuid] */ 

#define INK_SERIALIZED_FORMAT	L"Ink Serialized Format"
typedef /* [helpcontext][helpstring] */ 
enum ItemSelectionConstants
    {
        ISC_FirstElement	= 0,
        ISC_AllElements	= -1
    } 	InkSelectionConstants;

#define STR_GUID_X                          L"{598A6A8F-52C0-4BA0-93AF-AF357411A561}"
#define STR_GUID_Y                          L"{B53F9F75-04E0-4498-A7EE-C30DBB5A9011}"
#define STR_GUID_Z                          L"{735ADB30-0EBB-4788-A0E4-0F316490055D}"
#define STR_GUID_PAKETSTATUS                L"{6E0E07BF-AFE7-4CF7-87D1-AF6446208418}"
#define STR_GUID_TIMERTICK                  L"{436510C5-FED3-45D1-8B76-71D3EA7A829D}"
#define STR_GUID_SERIALNUMBER               L"{78A81B56-0935-4493-BAAE-00541A8A16C4}"
#define STR_GUID_NORMALPRESSURE             L"{7307502D-F9F4-4E18-B3F2-2CE1B1A3610C}"
#define STR_GUID_TANGENTPRESSURE            L"{6DA4488B-5244-41EC-905B-32D89AB80809}"
#define STR_GUID_BUTTONPRESSURE             L"{8B7FEFC4-96AA-4BFE-AC26-8A5F0BE07BF5}"
#define STR_GUID_XTILTORIENTATION           L"{A8D07B3A-8BF0-40B0-95A9-B80A6BB787BF}"
#define STR_GUID_YTILTORIENTATION           L"{0E932389-1D77-43AF-AC00-5B950D6D4B2D}"
#define STR_GUID_AZIMUTHORIENTATION         L"{029123B4-8828-410B-B250-A0536595E5DC}"
#define STR_GUID_ALTITUDEORIENTATION        L"{82DEC5C7-F6BA-4906-894F-66D68DFC456C}"
#define STR_GUID_TWISTORIENTATION           L"{0D324960-13B2-41E4-ACE6-7AE9D43D2D3B}"
#define STR_GUID_PITCHROTATION              L"{7F7E57B7-BE37-4BE1-A356-7A84160E1893}"
#define STR_GUID_ROLLROTATION               L"{5D5D5E56-6BA9-4C5B-9FB0-851C91714E56}"
#define STR_GUID_YAWROTATION                L"{6A849980-7C3A-45B7-AA82-90A262950E89}"
#define STR_GUID_WIDTH                      L"{BAABE94D-2712-48F5-BE9D-8F8B5EA0711A}"
#define STR_GUID_HEIGHT                     L"{E61858D2-E447-4218-9D3F-18865C203DF4}"
#define STR_GUID_FINGERCONTACTCONFIDENCE    L"{E706C804-57F0-4F00-8A0C-853D57789BE9}"
#define STR_GUID_DEVICE_CONTACT_ID          L"{02585B91-049B-4750-9615-DF8948AB3C9C}"
const GUID GUID_PACKETPROPERTY_GUID_X =                        { 0x598a6a8f, 0x52c0, 0x4ba0, { 0x93, 0xaf, 0xaf, 0x35, 0x74, 0x11, 0xa5, 0x61} };
const GUID GUID_PACKETPROPERTY_GUID_Y =                        { 0xb53f9f75, 0x04e0, 0x4498, { 0xa7, 0xee, 0xc3, 0x0d, 0xbb, 0x5a, 0x90, 0x11} };
const GUID GUID_PACKETPROPERTY_GUID_Z =                        { 0x735adb30, 0x0ebb, 0x4788, { 0xa0, 0xe4, 0x0f, 0x31, 0x64, 0x90, 0x05, 0x5d} };
const GUID GUID_PACKETPROPERTY_GUID_PACKET_STATUS =            { 0x6e0e07bf, 0xafe7, 0x4cf7, { 0x87, 0xd1, 0xaf, 0x64, 0x46, 0x20, 0x84, 0x18} };
const GUID GUID_PACKETPROPERTY_GUID_TIMER_TICK =               { 0x436510c5, 0xfed3, 0x45d1, { 0x8b, 0x76, 0x71, 0xd3, 0xea, 0x7a, 0x82, 0x9d} };
const GUID GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER =            { 0x78a81b56, 0x0935, 0x4493, { 0xba, 0xae, 0x00, 0x54, 0x1a, 0x8a, 0x16, 0xc4} };
const GUID GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE =          { 0x7307502d, 0xf9f4, 0x4e18, { 0xb3, 0xf2, 0x2c, 0xe1, 0xb1, 0xa3, 0x61, 0x0c} };
const GUID GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE =         { 0x6da4488b, 0x5244, 0x41ec, { 0x90, 0x5b, 0x32, 0xd8, 0x9a, 0xb8, 0x08, 0x09} };
const GUID GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE =          { 0x8b7fefc4, 0x96aa, 0x4bfe, { 0xac, 0x26, 0x8a, 0x5f, 0x0b, 0xe0, 0x7b, 0xf5} };
const GUID GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION =       { 0xa8d07b3a, 0x8bf0, 0x40b0, { 0x95, 0xa9, 0xb8, 0x0a, 0x6b, 0xb7, 0x87, 0xbf} };
const GUID GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION =       { 0x0e932389, 0x1d77, 0x43af, { 0xac, 0x00, 0x5b, 0x95, 0x0d, 0x6d, 0x4b, 0x2d} };
const GUID GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION =      { 0x029123b4, 0x8828, 0x410b, { 0xb2, 0x50, 0xa0, 0x53, 0x65, 0x95, 0xe5, 0xdc} };
const GUID GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION =     { 0x82dec5c7, 0xf6ba, 0x4906, { 0x89, 0x4f, 0x66, 0xd6, 0x8d, 0xfc, 0x45, 0x6c} };
const GUID GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION =        { 0x0d324960, 0x13b2, 0x41e4, { 0xac, 0xe6, 0x7a, 0xe9, 0xd4, 0x3d, 0x2d, 0x3b} };
const GUID GUID_PACKETPROPERTY_GUID_PITCH_ROTATION =           { 0x7f7e57b7, 0xbe37, 0x4be1, { 0xa3, 0x56, 0x7a, 0x84, 0x16, 0x0e, 0x18, 0x93} };
const GUID GUID_PACKETPROPERTY_GUID_ROLL_ROTATION =            { 0x5d5d5e56, 0x6ba9, 0x4c5b, { 0x9f, 0xb0, 0x85, 0x1c, 0x91, 0x71, 0x4e, 0x56} };
const GUID GUID_PACKETPROPERTY_GUID_YAW_ROTATION =             { 0x6a849980, 0x7c3a, 0x45b7, { 0xaa, 0x82, 0x90, 0xa2, 0x62, 0x95, 0x0e, 0x89} };
const GUID GUID_PACKETPROPERTY_GUID_WIDTH =                    { 0xbaabe94d, 0x2712, 0x48f5, { 0xbe, 0x9d, 0x8f, 0x8b, 0x5e, 0xa0, 0x71, 0x1a} };
const GUID GUID_PACKETPROPERTY_GUID_HEIGHT =                   { 0xe61858d2, 0xe447, 0x4218, { 0x9d, 0x3f, 0x18, 0x86, 0x5c, 0x20, 0x3d, 0xf4} };
const GUID GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE =  { 0xe706c804, 0x57f0, 0x4f00, { 0x8a, 0x0c, 0x85, 0x3d, 0x57, 0x78, 0x9b, 0xe9} };
const GUID GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID =        { 0x02585b91, 0x049b, 0x4750, { 0x96, 0x15, 0xdf, 0x89, 0x48, 0xab, 0x3c, 0x9c} };
#define INKRECOGNITIONPROPERTY_LINENUMBER		  L"{DBF29F2C-5289-4BE8-B3D8-6EF63246253E}"
#define INKRECOGNITIONPROPERTY_BOXNUMBER		  L"{2C243E3A-F733-4EB6-B1F8-B5DC5C2C4CDA}"
#define INKRECOGNITIONPROPERTY_SEGMENTATION       L"{B3C0FE6C-FB51-4164-BA2F-844AF8F983DA}"
#define INKRECOGNITIONPROPERTY_HOTPOINT           L"{CA6F40DC-5292-452a-91FB-2181C0BEC0DE}"
#define INKRECOGNITIONPROPERTY_MAXIMUMSTROKECOUNT L"{BF0EEC4E-4B7D-47a9-8CFA-234DD24BD22A}"
#define INKRECOGNITIONPROPERTY_POINTSPERINCH      L"{7ED16B76-889C-468e-8276-0021B770187E}"
#define INKRECOGNITIONPROPERTY_CONFIDENCELEVEL    L"{7DFE11A7-FB5D-4958-8765-154ADF0D833F}"
#define INKRECOGNITIONPROPERTY_LINEMETRICS        L"{8CC24B27-30A9-4b96-9056-2D3A90DA0727}"
#define FACTOID_NONE				L"NONE"
#define FACTOID_DEFAULT				L"DEFAULT"
#define FACTOID_SYSTEMDICTIONARY	L"SYSDICT"
#define FACTOID_WORDLIST			L"WORDLIST"
#define FACTOID_EMAIL				L"EMAIL"
#define FACTOID_WEB					L"WEB"
#define FACTOID_ONECHAR				L"ONECHAR"
#define FACTOID_NUMBER				L"NUMBER"
#define FACTOID_DIGIT				L"DIGIT"
#define FACTOID_NUMBERSIMPLE		L"NUMSIMPLE"
#define FACTOID_CURRENCY			L"CURRENCY"
#define FACTOID_POSTALCODE			L"POSTALCODE"
#define FACTOID_PERCENT				L"PERCENT"
#define FACTOID_DATE				L"DATE"
#define FACTOID_TIME				L"TIME"
#define FACTOID_TELEPHONE			L"TELEPHONE"
#define FACTOID_FILENAME			L"FILENAME"
#define FACTOID_UPPERCHAR			L"UPPERCHAR"
#define FACTOID_LOWERCHAR			L"LOWERCHAR"
#define FACTOID_PUNCCHAR			L"PUNCCHAR"
#define FACTOID_JAPANESECOMMON		L"JPN_COMMON"
#define FACTOID_CHINESESIMPLECOMMON	L"CHS_COMMON"
#define FACTOID_CHINESETRADITIONALCOMMON L"CHT_COMMON"
#define FACTOID_KOREANCOMMON		L"KOR_COMMON"
#define FACTOID_HIRAGANA			L"HIRAGANA"
#define FACTOID_KATAKANA			L"KATAKANA"
#define FACTOID_KANJICOMMON		    L"KANJI_COMMON"
#define FACTOID_KANJIRARE			L"KANJI_RARE"
#define FACTOID_BOPOMOFO	        L"BOPOMOFO"
#define FACTOID_JAMO				L"JAMO"
#define FACTOID_HANGULCOMMON		L"HANGUL_COMMON"
#define FACTOID_HANGULRARE			L"HANGUL_RARE"
typedef /* [helpcontext][helpstring] */ 
enum InkBoundingBoxMode
    {
        IBBM_Default	= 0,
        IBBM_NoCurveFit	= 1,
        IBBM_CurveFit	= 2,
        IBBM_PointsOnly	= 3,
        IBBM_Union	= 4
    } 	InkBoundingBoxMode;

#define IBBM_Last IBBM_Union + 1
typedef /* [helpcontext][helpstring] */ 
enum InkExtractFlags
    {
        IEF_CopyFromOriginal	= 0,
        IEF_RemoveFromOriginal	= 0x1,
        IEF_Default	= IEF_RemoveFromOriginal
    } 	InkExtractFlags;

#define IEF_All  (IEF_RemoveFromOriginal | IEF_CopyFromOriginal)
typedef /* [helpcontext][helpstring] */ 
enum InkPersistenceFormat
    {
        IPF_InkSerializedFormat	= 0,
        IPF_Base64InkSerializedFormat	= 1,
        IPF_GIF	= 2,
        IPF_Base64GIF	= 3
    } 	InkPersistenceFormat;

#define IPF_Last     IPF_Base64GIF + 1 
typedef /* [helpcontext][helpstring] */ 
enum InkPersistenceCompressionMode
    {
        IPCM_Default	= 0,
        IPCM_MaximumCompression	= 1,
        IPCM_NoCompression	= 2
    } 	InkPersistenceCompressionMode;

typedef /* [helpcontext][helpstring] */ 
enum InkPenTip
    {
        IPT_Ball	= 0,
        IPT_Rectangle	= 1
    } 	InkPenTip;

#define IPT_Last     IPT_Rectangle + 1 
typedef /* [helpcontext][helpstring] */ 
enum InkRasterOperation
    {
        IRO_Black	= 1,
        IRO_NotMergePen	= 2,
        IRO_MaskNotPen	= 3,
        IRO_NotCopyPen	= 4,
        IRO_MaskPenNot	= 5,
        IRO_Not	= 6,
        IRO_XOrPen	= 7,
        IRO_NotMaskPen	= 8,
        IRO_MaskPen	= 9,
        IRO_NotXOrPen	= 10,
        IRO_NoOperation	= 11,
        IRO_MergeNotPen	= 12,
        IRO_CopyPen	= 13,
        IRO_MergePenNot	= 14,
        IRO_MergePen	= 15,
        IRO_White	= 16
    } 	InkRasterOperation;

#define IRO_Last IRO_White+1
typedef /* [helpcontext][helpstring] */ 
enum InkMousePointer
    {
        IMP_Default	= 0,
        IMP_Arrow	= 1,
        IMP_Crosshair	= 2,
        IMP_Ibeam	= 3,
        IMP_SizeNESW	= 4,
        IMP_SizeNS	= 5,
        IMP_SizeNWSE	= 6,
        IMP_SizeWE	= 7,
        IMP_UpArrow	= 8,
        IMP_Hourglass	= 9,
        IMP_NoDrop	= 10,
        IMP_ArrowHourglass	= 11,
        IMP_ArrowQuestion	= 12,
        IMP_SizeAll	= 13,
        IMP_Hand	= 14,
        IMP_Custom	= 99
    } 	InkMousePointer;

typedef /* [helpcontext][helpstring] */ 
enum InkClipboardModes
    {
        ICB_Copy	= 0,
        ICB_Cut	= 0x1,
        ICB_ExtractOnly	= 0x30,
        ICB_DelayedCopy	= 0x20,
        ICB_Default	= ICB_Copy
    } 	InkClipboardModes;

#define ICB_Last ICB_DelayedCopy+1
typedef /* [helpcontext][helpstring] */ 
enum InkClipboardFormats
    {
        ICF_None	= 0,
        ICF_InkSerializedFormat	= 0x1,
        ICF_SketchInk	= 0x2,
        ICF_TextInk	= 0x6,
        ICF_EnhancedMetafile	= 0x8,
        ICF_Metafile	= 0x20,
        ICF_Bitmap	= 0x40,
        ICF_PasteMask	= 0x7,
        ICF_CopyMask	= 0x7f,
        ICF_Default	= ICF_CopyMask
    } 	InkClipboardFormats;

#define ICF_Last ICF_CopyMask+1
typedef /* [helpcontext][helpstring] */ 
enum SelectionHitResult
    {
        SHR_None	= 0,
        SHR_NW	= ( SHR_None + 1 ) ,
        SHR_SE	= ( SHR_NW + 1 ) ,
        SHR_NE	= ( SHR_SE + 1 ) ,
        SHR_SW	= ( SHR_NE + 1 ) ,
        SHR_E	= ( SHR_SW + 1 ) ,
        SHR_W	= ( SHR_E + 1 ) ,
        SHR_N	= ( SHR_W + 1 ) ,
        SHR_S	= ( SHR_N + 1 ) ,
        SHR_Selection	= ( SHR_S + 1 ) 
    } 	SelectionHitResult;

typedef /* [helpcontext][helpstring] */ 
enum InkRecognitionStatus
    {
        IRS_NoError	= 0,
        IRS_Interrupted	= 0x1,
        IRS_ProcessFailed	= 0x2,
        IRS_InkAddedFailed	= 0x4,
        IRS_SetAutoCompletionModeFailed	= 0x8,
        IRS_SetStrokesFailed	= 0x10,
        IRS_SetGuideFailed	= 0x20,
        IRS_SetFlagsFailed	= 0x40,
        IRS_SetFactoidFailed	= 0x80,
        IRS_SetPrefixSuffixFailed	= 0x100,
        IRS_SetWordListFailed	= 0x200
    } 	InkRecognitionStatus;

typedef /* [hidden] */ 
enum DISPID_InkRectangle
    {
        DISPID_IRTop	= 1,
        DISPID_IRLeft	= ( DISPID_IRTop + 1 ) ,
        DISPID_IRBottom	= ( DISPID_IRLeft + 1 ) ,
        DISPID_IRRight	= ( DISPID_IRBottom + 1 ) ,
        DISPID_IRGetRectangle	= ( DISPID_IRRight + 1 ) ,
        DISPID_IRSetRectangle	= ( DISPID_IRGetRectangle + 1 ) ,
        DISPID_IRData	= ( DISPID_IRSetRectangle + 1 ) 
    } 	DISPID_InkRectangle;

typedef /* [hidden] */ 
enum DISPID_InkExtendedProperty
    {
        DISPID_IEPGuid	= 1,
        DISPID_IEPData	= ( DISPID_IEPGuid + 1 ) 
    } 	DISPID_InkExtendedProperty;

typedef /* [hidden] */ 
enum DISPID_InkExtendedProperties
    {
        DISPID_IEPs_NewEnum	= DISPID_NEWENUM,
        DISPID_IEPsItem	= DISPID_VALUE,
        DISPID_IEPsCount	= 1,
        DISPID_IEPsAdd	= ( DISPID_IEPsCount + 1 ) ,
        DISPID_IEPsRemove	= ( DISPID_IEPsAdd + 1 ) ,
        DISPID_IEPsClear	= ( DISPID_IEPsRemove + 1 ) ,
        DISPID_IEPsDoesPropertyExist	= ( DISPID_IEPsClear + 1 ) 
    } 	DISPID_InkExtendedProperties;

typedef /* [hidden] */ 
enum DISPID_InkDrawingAttributes
    {
        DISPID_DAHeight	= 1,
        DISPID_DAColor	= ( DISPID_DAHeight + 1 ) ,
        DISPID_DAWidth	= ( DISPID_DAColor + 1 ) ,
        DISPID_DAFitToCurve	= ( DISPID_DAWidth + 1 ) ,
        DISPID_DAIgnorePressure	= ( DISPID_DAFitToCurve + 1 ) ,
        DISPID_DAAntiAliased	= ( DISPID_DAIgnorePressure + 1 ) ,
        DISPID_DATransparency	= ( DISPID_DAAntiAliased + 1 ) ,
        DISPID_DARasterOperation	= ( DISPID_DATransparency + 1 ) ,
        DISPID_DAPenTip	= ( DISPID_DARasterOperation + 1 ) ,
        DISPID_DAClone	= ( DISPID_DAPenTip + 1 ) ,
        DISPID_DAExtendedProperties	= ( DISPID_DAClone + 1 ) 
    } 	DISPID_InkDrawingAttributes;

typedef /* [hidden] */ 
enum DISPID_InkTransform
    {
        DISPID_ITReset	= 1,
        DISPID_ITTranslate	= ( DISPID_ITReset + 1 ) ,
        DISPID_ITRotate	= ( DISPID_ITTranslate + 1 ) ,
        DISPID_ITReflect	= ( DISPID_ITRotate + 1 ) ,
        DISPID_ITShear	= ( DISPID_ITReflect + 1 ) ,
        DISPID_ITScale	= ( DISPID_ITShear + 1 ) ,
        DISPID_ITeM11	= ( DISPID_ITScale + 1 ) ,
        DISPID_ITeM12	= ( DISPID_ITeM11 + 1 ) ,
        DISPID_ITeM21	= ( DISPID_ITeM12 + 1 ) ,
        DISPID_ITeM22	= ( DISPID_ITeM21 + 1 ) ,
        DISPID_ITeDx	= ( DISPID_ITeM22 + 1 ) ,
        DISPID_ITeDy	= ( DISPID_ITeDx + 1 ) ,
        DISPID_ITGetTransform	= ( DISPID_ITeDy + 1 ) ,
        DISPID_ITSetTransform	= ( DISPID_ITGetTransform + 1 ) ,
        DISPID_ITData	= ( DISPID_ITSetTransform + 1 ) 
    } 	DISPID_InkTransform;

typedef 
enum InkApplicationGesture
    {
        IAG_AllGestures	= 0,
        IAG_NoGesture	= 0xf000,
        IAG_Scratchout	= 0xf001,
        IAG_Triangle	= 0xf002,
        IAG_Square	= 0xf003,
        IAG_Star	= 0xf004,
        IAG_Check	= 0xf005,
        IAG_Curlicue	= 0xf010,
        IAG_DoubleCurlicue	= 0xf011,
        IAG_Circle	= 0xf020,
        IAG_DoubleCircle	= 0xf021,
        IAG_SemiCircleLeft	= 0xf028,
        IAG_SemiCircleRight	= 0xf029,
        IAG_ChevronUp	= 0xf030,
        IAG_ChevronDown	= 0xf031,
        IAG_ChevronLeft	= 0xf032,
        IAG_ChevronRight	= 0xf033,
        IAG_ArrowUp	= 0xf038,
        IAG_ArrowDown	= 0xf039,
        IAG_ArrowLeft	= 0xf03a,
        IAG_ArrowRight	= 0xf03b,
        IAG_Up	= 0xf058,
        IAG_Down	= 0xf059,
        IAG_Left	= 0xf05a,
        IAG_Right	= 0xf05b,
        IAG_UpDown	= 0xf060,
        IAG_DownUp	= 0xf061,
        IAG_LeftRight	= 0xf062,
        IAG_RightLeft	= 0xf063,
        IAG_UpLeftLong	= 0xf064,
        IAG_UpRightLong	= 0xf065,
        IAG_DownLeftLong	= 0xf066,
        IAG_DownRightLong	= 0xf067,
        IAG_UpLeft	= 0xf068,
        IAG_UpRight	= 0xf069,
        IAG_DownLeft	= 0xf06a,
        IAG_DownRight	= 0xf06b,
        IAG_LeftUp	= 0xf06c,
        IAG_LeftDown	= 0xf06d,
        IAG_RightUp	= 0xf06e,
        IAG_RightDown	= 0xf06f,
        IAG_Exclamation	= 0xf0a4,
        IAG_Tap	= 0xf0f0,
        IAG_DoubleTap	= 0xf0f1
    } 	InkApplicationGesture;

typedef 
enum InkSystemGesture
    {
        ISG_Tap	= 0x10,
        ISG_DoubleTap	= 0x11,
        ISG_RightTap	= 0x12,
        ISG_Drag	= 0x13,
        ISG_RightDrag	= 0x14,
        ISG_HoldEnter	= 0x15,
        ISG_HoldLeave	= 0x16,
        ISG_HoverEnter	= 0x17,
        ISG_HoverLeave	= 0x18,
        ISG_Flick	= 0x1f
    } 	InkSystemGesture;

#define ISG_First ISG_Tap
#define ISG_Last ISG_Flick
typedef 
enum InkRecognitionConfidence
    {
        IRC_Strong	= 0,
        IRC_Intermediate	= 1,
        IRC_Poor	= 2
    } 	InkRecognitionConfidence;

typedef /* [hidden] */ 
enum DISPID_InkGesture
    {
        DISPID_IGId	= DISPID_VALUE,
        DISPID_IGGetHotPoint	= ( DISPID_IGId + 1 ) ,
        DISPID_IGConfidence	= ( DISPID_IGGetHotPoint + 1 ) 
    } 	DISPID_InkGesture;

typedef /* [hidden] */ 
enum DISPID_InkCursor
    {
        DISPID_ICsrName	= DISPID_VALUE,
        DISPID_ICsrId	= 1,
        DISPID_ICsrDrawingAttributes	= ( DISPID_ICsrId + 1 ) ,
        DISPID_ICsrButtons	= ( DISPID_ICsrDrawingAttributes + 1 ) ,
        DISPID_ICsrInverted	= ( DISPID_ICsrButtons + 1 ) ,
        DISPID_ICsrTablet	= ( DISPID_ICsrInverted + 1 ) 
    } 	DISPID_InkCursor;

typedef /* [hidden] */ 
enum DISPID_InkCursors
    {
        DISPID_ICs_NewEnum	= DISPID_NEWENUM,
        DISPID_ICsItem	= DISPID_VALUE,
        DISPID_ICsCount	= 1
    } 	DISPID_InkCursors;

typedef 
enum InkCursorButtonState
    {
        ICBS_Unavailable	= 0,
        ICBS_Up	= ( ICBS_Unavailable + 1 ) ,
        ICBS_Down	= ( ICBS_Up + 1 ) 
    } 	InkCursorButtonState;

#define ICBS_Last ICBS_Down
typedef /* [hidden] */ 
enum DISPID_InkCursorButton
    {
        DISPID_ICBName	= DISPID_VALUE,
        DISPID_ICBId	= 1,
        DISPID_ICBState	= ( DISPID_ICBId + 1 ) 
    } 	DISPID_InkCursorButton;

typedef /* [hidden] */ 
enum DISPID_InkCursorButtons
    {
        DISPID_ICBs_NewEnum	= DISPID_NEWENUM,
        DISPID_ICBsItem	= DISPID_VALUE,
        DISPID_ICBsCount	= 1
    } 	DISPID_InkCursorButtons;

typedef 
enum TabletHardwareCapabilities
    {
        THWC_Integrated	= 0x1,
        THWC_CursorMustTouch	= 0x2,
        THWC_HardProximity	= 0x4,
        THWC_CursorsHavePhysicalIds	= 0x8
    } 	TabletHardwareCapabilities;

typedef 
enum TabletPropertyMetricUnit
    {
        TPMU_Default	= 0,
        TPMU_Inches	= ( TPMU_Default + 1 ) ,
        TPMU_Centimeters	= ( TPMU_Inches + 1 ) ,
        TPMU_Degrees	= ( TPMU_Centimeters + 1 ) ,
        TPMU_Radians	= ( TPMU_Degrees + 1 ) ,
        TPMU_Seconds	= ( TPMU_Radians + 1 ) ,
        TPMU_Pounds	= ( TPMU_Seconds + 1 ) ,
        TPMU_Grams	= ( TPMU_Pounds + 1 ) 
    } 	TabletPropertyMetricUnit;

typedef /* [hidden] */ 
enum DISPID_InkTablet
    {
        DISPID_ITName	= DISPID_VALUE,
        DISPID_ITPlugAndPlayId	= 1,
        DISPID_ITPropertyMetrics	= ( DISPID_ITPlugAndPlayId + 1 ) ,
        DISPID_ITIsPacketPropertySupported	= ( DISPID_ITPropertyMetrics + 1 ) ,
        DISPID_ITMaximumInputRectangle	= ( DISPID_ITIsPacketPropertySupported + 1 ) ,
        DISPID_ITHardwareCapabilities	= ( DISPID_ITMaximumInputRectangle + 1 ) 
    } 	DISPID_InkTablet;

typedef 
enum TabletDeviceKind
    {
        TDK_Mouse	= 0,
        TDK_Pen	= ( TDK_Mouse + 1 ) ,
        TDK_Touch	= ( TDK_Pen + 1 ) 
    } 	TabletDeviceKind;

typedef /* [hidden] */ 
enum DISPID_InkTablet2
    {
        DISPID_IT2DeviceKind	= DISPID_VALUE
    } 	DISPID_InkTablet2;

typedef /* [hidden] */ 
enum DISPID_InkTablet3
    {
        DISPID_IT3IsMultiTouch	= DISPID_VALUE,
        DISPID_IT3MaximumCursors	= ( DISPID_IT3IsMultiTouch + 1 ) 
    } 	DISPID_InkTablet3;

typedef /* [hidden] */ 
enum DISPID_InkTablets
    {
        DISPID_ITs_NewEnum	= DISPID_NEWENUM,
        DISPID_ITsItem	= DISPID_VALUE,
        DISPID_ITsDefaultTablet	= 1,
        DISPID_ITsCount	= ( DISPID_ITsDefaultTablet + 1 ) ,
        DISPID_ITsIsPacketPropertySupported	= ( DISPID_ITsCount + 1 ) 
    } 	DISPID_InkTablets;

typedef /* [hidden] */ 
enum DISPID_InkStrokeDisp
    {
        DISPID_ISDInkIndex	= 1,
        DISPID_ISDID	= ( DISPID_ISDInkIndex + 1 ) ,
        DISPID_ISDGetBoundingBox	= ( DISPID_ISDID + 1 ) ,
        DISPID_ISDDrawingAttributes	= ( DISPID_ISDGetBoundingBox + 1 ) ,
        DISPID_ISDFindIntersections	= ( DISPID_ISDDrawingAttributes + 1 ) ,
        DISPID_ISDGetRectangleIntersections	= ( DISPID_ISDFindIntersections + 1 ) ,
        DISPID_ISDClip	= ( DISPID_ISDGetRectangleIntersections + 1 ) ,
        DISPID_ISDHitTestCircle	= ( DISPID_ISDClip + 1 ) ,
        DISPID_ISDNearestPoint	= ( DISPID_ISDHitTestCircle + 1 ) ,
        DISPID_ISDSplit	= ( DISPID_ISDNearestPoint + 1 ) ,
        DISPID_ISDExtendedProperties	= ( DISPID_ISDSplit + 1 ) ,
        DISPID_ISDInk	= ( DISPID_ISDExtendedProperties + 1 ) ,
        DISPID_ISDBezierPoints	= ( DISPID_ISDInk + 1 ) ,
        DISPID_ISDPolylineCusps	= ( DISPID_ISDBezierPoints + 1 ) ,
        DISPID_ISDBezierCusps	= ( DISPID_ISDPolylineCusps + 1 ) ,
        DISPID_ISDSelfIntersections	= ( DISPID_ISDBezierCusps + 1 ) ,
        DISPID_ISDPacketCount	= ( DISPID_ISDSelfIntersections + 1 ) ,
        DISPID_ISDPacketSize	= ( DISPID_ISDPacketCount + 1 ) ,
        DISPID_ISDPacketDescription	= ( DISPID_ISDPacketSize + 1 ) ,
        DISPID_ISDDeleted	= ( DISPID_ISDPacketDescription + 1 ) ,
        DISPID_ISDGetPacketDescriptionPropertyMetrics	= ( DISPID_ISDDeleted + 1 ) ,
        DISPID_ISDGetPoints	= ( DISPID_ISDGetPacketDescriptionPropertyMetrics + 1 ) ,
        DISPID_ISDSetPoints	= ( DISPID_ISDGetPoints + 1 ) ,
        DISPID_ISDGetPacketData	= ( DISPID_ISDSetPoints + 1 ) ,
        DISPID_ISDGetPacketValuesByProperty	= ( DISPID_ISDGetPacketData + 1 ) ,
        DISPID_ISDSetPacketValuesByProperty	= ( DISPID_ISDGetPacketValuesByProperty + 1 ) ,
        DISPID_ISDGetFlattenedBezierPoints	= ( DISPID_ISDSetPacketValuesByProperty + 1 ) ,
        DISPID_ISDScaleToRectangle	= ( DISPID_ISDGetFlattenedBezierPoints + 1 ) ,
        DISPID_ISDTransform	= ( DISPID_ISDScaleToRectangle + 1 ) ,
        DISPID_ISDMove	= ( DISPID_ISDTransform + 1 ) ,
        DISPID_ISDRotate	= ( DISPID_ISDMove + 1 ) ,
        DISPID_ISDShear	= ( DISPID_ISDRotate + 1 ) ,
        DISPID_ISDScale	= ( DISPID_ISDShear + 1 ) 
    } 	DISPID_InkStrokeDisp;

typedef /* [hidden] */ 
enum DISPID_InkStrokes
    {
        DISPID_ISs_NewEnum	= DISPID_NEWENUM,
        DISPID_ISsItem	= DISPID_VALUE,
        DISPID_ISsCount	= 1,
        DISPID_ISsValid	= ( DISPID_ISsCount + 1 ) ,
        DISPID_ISsInk	= ( DISPID_ISsValid + 1 ) ,
        DISPID_ISsAdd	= ( DISPID_ISsInk + 1 ) ,
        DISPID_ISsAddStrokes	= ( DISPID_ISsAdd + 1 ) ,
        DISPID_ISsRemove	= ( DISPID_ISsAddStrokes + 1 ) ,
        DISPID_ISsRemoveStrokes	= ( DISPID_ISsRemove + 1 ) ,
        DISPID_ISsToString	= ( DISPID_ISsRemoveStrokes + 1 ) ,
        DISPID_ISsModifyDrawingAttributes	= ( DISPID_ISsToString + 1 ) ,
        DISPID_ISsGetBoundingBox	= ( DISPID_ISsModifyDrawingAttributes + 1 ) ,
        DISPID_ISsScaleToRectangle	= ( DISPID_ISsGetBoundingBox + 1 ) ,
        DISPID_ISsTransform	= ( DISPID_ISsScaleToRectangle + 1 ) ,
        DISPID_ISsMove	= ( DISPID_ISsTransform + 1 ) ,
        DISPID_ISsRotate	= ( DISPID_ISsMove + 1 ) ,
        DISPID_ISsShear	= ( DISPID_ISsRotate + 1 ) ,
        DISPID_ISsScale	= ( DISPID_ISsShear + 1 ) ,
        DISPID_ISsClip	= ( DISPID_ISsScale + 1 ) ,
        DISPID_ISsRecognitionResult	= ( DISPID_ISsClip + 1 ) ,
        DISPID_ISsRemoveRecognitionResult	= ( DISPID_ISsRecognitionResult + 1 ) 
    } 	DISPID_InkStrokes;

typedef /* [hidden] */ 
enum DISPID_InkCustomStrokes
    {
        DISPID_ICSs_NewEnum	= DISPID_NEWENUM,
        DISPID_ICSsItem	= DISPID_VALUE,
        DISPID_ICSsCount	= 1,
        DISPID_ICSsAdd	= ( DISPID_ICSsCount + 1 ) ,
        DISPID_ICSsRemove	= ( DISPID_ICSsAdd + 1 ) ,
        DISPID_ICSsClear	= ( DISPID_ICSsRemove + 1 ) 
    } 	DISPID_InkCustomStrokes;

typedef /* [hidden] */ 
enum DISPID_StrokeEvent
    {
        DISPID_SEStrokesAdded	= 1,
        DISPID_SEStrokesRemoved	= ( DISPID_SEStrokesAdded + 1 ) 
    } 	DISPID_StrokeEvent;

typedef /* [hidden] */ 
enum DISPID_Ink
    {
        DISPID_IStrokes	= 1,
        DISPID_IExtendedProperties	= ( DISPID_IStrokes + 1 ) ,
        DISPID_IGetBoundingBox	= ( DISPID_IExtendedProperties + 1 ) ,
        DISPID_IDeleteStrokes	= ( DISPID_IGetBoundingBox + 1 ) ,
        DISPID_IDeleteStroke	= ( DISPID_IDeleteStrokes + 1 ) ,
        DISPID_IExtractStrokes	= ( DISPID_IDeleteStroke + 1 ) ,
        DISPID_IExtractWithRectangle	= ( DISPID_IExtractStrokes + 1 ) ,
        DISPID_IDirty	= ( DISPID_IExtractWithRectangle + 1 ) ,
        DISPID_ICustomStrokes	= ( DISPID_IDirty + 1 ) ,
        DISPID_IClone	= ( DISPID_ICustomStrokes + 1 ) ,
        DISPID_IHitTestCircle	= ( DISPID_IClone + 1 ) ,
        DISPID_IHitTestWithRectangle	= ( DISPID_IHitTestCircle + 1 ) ,
        DISPID_IHitTestWithLasso	= ( DISPID_IHitTestWithRectangle + 1 ) ,
        DISPID_INearestPoint	= ( DISPID_IHitTestWithLasso + 1 ) ,
        DISPID_ICreateStrokes	= ( DISPID_INearestPoint + 1 ) ,
        DISPID_ICreateStroke	= ( DISPID_ICreateStrokes + 1 ) ,
        DISPID_IAddStrokesAtRectangle	= ( DISPID_ICreateStroke + 1 ) ,
        DISPID_IClip	= ( DISPID_IAddStrokesAtRectangle + 1 ) ,
        DISPID_ISave	= ( DISPID_IClip + 1 ) ,
        DISPID_ILoad	= ( DISPID_ISave + 1 ) ,
        DISPID_ICreateStrokeFromPoints	= ( DISPID_ILoad + 1 ) ,
        DISPID_IClipboardCopyWithRectangle	= ( DISPID_ICreateStrokeFromPoints + 1 ) ,
        DISPID_IClipboardCopy	= ( DISPID_IClipboardCopyWithRectangle + 1 ) ,
        DISPID_ICanPaste	= ( DISPID_IClipboardCopy + 1 ) ,
        DISPID_IClipboardPaste	= ( DISPID_ICanPaste + 1 ) 
    } 	DISPID_Ink;

typedef /* [hidden] */ 
enum DISPID_InkEvent
    {
        DISPID_IEInkAdded	= 1,
        DISPID_IEInkDeleted	= ( DISPID_IEInkAdded + 1 ) 
    } 	DISPID_InkEvent;

typedef /* [hidden] */ 
enum DISPID_InkRenderer
    {
        DISPID_IRGetViewTransform	= 1,
        DISPID_IRSetViewTransform	= ( DISPID_IRGetViewTransform + 1 ) ,
        DISPID_IRGetObjectTransform	= ( DISPID_IRSetViewTransform + 1 ) ,
        DISPID_IRSetObjectTransform	= ( DISPID_IRGetObjectTransform + 1 ) ,
        DISPID_IRDraw	= ( DISPID_IRSetObjectTransform + 1 ) ,
        DISPID_IRDrawStroke	= ( DISPID_IRDraw + 1 ) ,
        DISPID_IRPixelToInkSpace	= ( DISPID_IRDrawStroke + 1 ) ,
        DISPID_IRInkSpaceToPixel	= ( DISPID_IRPixelToInkSpace + 1 ) ,
        DISPID_IRPixelToInkSpaceFromPoints	= ( DISPID_IRInkSpaceToPixel + 1 ) ,
        DISPID_IRInkSpaceToPixelFromPoints	= ( DISPID_IRPixelToInkSpaceFromPoints + 1 ) ,
        DISPID_IRMeasure	= ( DISPID_IRInkSpaceToPixelFromPoints + 1 ) ,
        DISPID_IRMeasureStroke	= ( DISPID_IRMeasure + 1 ) ,
        DISPID_IRMove	= ( DISPID_IRMeasureStroke + 1 ) ,
        DISPID_IRRotate	= ( DISPID_IRMove + 1 ) ,
        DISPID_IRScale	= ( DISPID_IRRotate + 1 ) 
    } 	DISPID_InkRenderer;

typedef 
enum InkCollectorEventInterest
    {
        ICEI_DefaultEvents	= -1,
        ICEI_CursorDown	= ( ICEI_DefaultEvents + 1 ) ,
        ICEI_Stroke	= ( ICEI_CursorDown + 1 ) ,
        ICEI_NewPackets	= ( ICEI_Stroke + 1 ) ,
        ICEI_NewInAirPackets	= ( ICEI_NewPackets + 1 ) ,
        ICEI_CursorButtonDown	= ( ICEI_NewInAirPackets + 1 ) ,
        ICEI_CursorButtonUp	= ( ICEI_CursorButtonDown + 1 ) ,
        ICEI_CursorInRange	= ( ICEI_CursorButtonUp + 1 ) ,
        ICEI_CursorOutOfRange	= ( ICEI_CursorInRange + 1 ) ,
        ICEI_SystemGesture	= ( ICEI_CursorOutOfRange + 1 ) ,
        ICEI_TabletAdded	= ( ICEI_SystemGesture + 1 ) ,
        ICEI_TabletRemoved	= ( ICEI_TabletAdded + 1 ) ,
        ICEI_MouseDown	= ( ICEI_TabletRemoved + 1 ) ,
        ICEI_MouseMove	= ( ICEI_MouseDown + 1 ) ,
        ICEI_MouseUp	= ( ICEI_MouseMove + 1 ) ,
        ICEI_MouseWheel	= ( ICEI_MouseUp + 1 ) ,
        ICEI_DblClick	= ( ICEI_MouseWheel + 1 ) ,
        ICEI_AllEvents	= ( ICEI_DblClick + 1 ) 
    } 	InkCollectorEventInterest;

typedef 
enum InkMouseButton
    {
        IMF_Left	= 1,
        IMF_Right	= 2,
        IMF_Middle	= 4
    } 	InkMouseButton;

typedef 
enum InkShiftKeyModifierFlags
    {
        IKM_Shift	= 0x1,
        IKM_Control	= 0x2,
        IKM_Alt	= 0x4
    } 	InkShiftKeyModifierFlags;

typedef /* [hidden] */ 
enum DISPID_InkCollectorEvent
    {
        DISPID_ICEStroke	= 1,
        DISPID_ICECursorDown	= ( DISPID_ICEStroke + 1 ) ,
        DISPID_ICENewPackets	= ( DISPID_ICECursorDown + 1 ) ,
        DISPID_ICENewInAirPackets	= ( DISPID_ICENewPackets + 1 ) ,
        DISPID_ICECursorButtonDown	= ( DISPID_ICENewInAirPackets + 1 ) ,
        DISPID_ICECursorButtonUp	= ( DISPID_ICECursorButtonDown + 1 ) ,
        DISPID_ICECursorInRange	= ( DISPID_ICECursorButtonUp + 1 ) ,
        DISPID_ICECursorOutOfRange	= ( DISPID_ICECursorInRange + 1 ) ,
        DISPID_ICESystemGesture	= ( DISPID_ICECursorOutOfRange + 1 ) ,
        DISPID_ICEGesture	= ( DISPID_ICESystemGesture + 1 ) ,
        DISPID_ICETabletAdded	= ( DISPID_ICEGesture + 1 ) ,
        DISPID_ICETabletRemoved	= ( DISPID_ICETabletAdded + 1 ) ,
        DISPID_IOEPainting	= ( DISPID_ICETabletRemoved + 1 ) ,
        DISPID_IOEPainted	= ( DISPID_IOEPainting + 1 ) ,
        DISPID_IOESelectionChanging	= ( DISPID_IOEPainted + 1 ) ,
        DISPID_IOESelectionChanged	= ( DISPID_IOESelectionChanging + 1 ) ,
        DISPID_IOESelectionMoving	= ( DISPID_IOESelectionChanged + 1 ) ,
        DISPID_IOESelectionMoved	= ( DISPID_IOESelectionMoving + 1 ) ,
        DISPID_IOESelectionResizing	= ( DISPID_IOESelectionMoved + 1 ) ,
        DISPID_IOESelectionResized	= ( DISPID_IOESelectionResizing + 1 ) ,
        DISPID_IOEStrokesDeleting	= ( DISPID_IOESelectionResized + 1 ) ,
        DISPID_IOEStrokesDeleted	= ( DISPID_IOEStrokesDeleting + 1 ) ,
        DISPID_IPEChangeUICues	= ( DISPID_IOEStrokesDeleted + 1 ) ,
        DISPID_IPEClick	= ( DISPID_IPEChangeUICues + 1 ) ,
        DISPID_IPEDblClick	= ( DISPID_IPEClick + 1 ) ,
        DISPID_IPEInvalidated	= ( DISPID_IPEDblClick + 1 ) ,
        DISPID_IPEMouseDown	= ( DISPID_IPEInvalidated + 1 ) ,
        DISPID_IPEMouseEnter	= ( DISPID_IPEMouseDown + 1 ) ,
        DISPID_IPEMouseHover	= ( DISPID_IPEMouseEnter + 1 ) ,
        DISPID_IPEMouseLeave	= ( DISPID_IPEMouseHover + 1 ) ,
        DISPID_IPEMouseMove	= ( DISPID_IPEMouseLeave + 1 ) ,
        DISPID_IPEMouseUp	= ( DISPID_IPEMouseMove + 1 ) ,
        DISPID_IPEMouseWheel	= ( DISPID_IPEMouseUp + 1 ) ,
        DISPID_IPESizeModeChanged	= ( DISPID_IPEMouseWheel + 1 ) ,
        DISPID_IPEStyleChanged	= ( DISPID_IPESizeModeChanged + 1 ) ,
        DISPID_IPESystemColorsChanged	= ( DISPID_IPEStyleChanged + 1 ) ,
        DISPID_IPEKeyDown	= ( DISPID_IPESystemColorsChanged + 1 ) ,
        DISPID_IPEKeyPress	= ( DISPID_IPEKeyDown + 1 ) ,
        DISPID_IPEKeyUp	= ( DISPID_IPEKeyPress + 1 ) ,
        DISPID_IPEResize	= ( DISPID_IPEKeyUp + 1 ) ,
        DISPID_IPESizeChanged	= ( DISPID_IPEResize + 1 ) 
    } 	DISPID_InkCollectorEvent;

typedef /* [helpcontext][helpstring] */ 
enum InkOverlayEditingMode
    {
        IOEM_Ink	= 0,
        IOEM_Delete	= 1,
        IOEM_Select	= 2
    } 	InkOverlayEditingMode;

#define IOEM_Last IOEM_Select + 1
typedef /* [helpcontext][helpstring] */ 
enum InkOverlayAttachMode
    {
        IOAM_Behind	= 0,
        IOAM_InFront	= 1
    } 	InkOverlayAttachMode;

#define IOAM_Last IOAM_InFront+1
typedef /* [helpcontext][helpstring] */ 
enum InkPictureSizeMode
    {
        IPSM_AutoSize	= 0,
        IPSM_CenterImage	= 1,
        IPSM_Normal	= 2,
        IPSM_StretchImage	= 3
    } 	InkPictureSizeMode;

#define IPSM_Last IPSM_StretchImage + 1
typedef /* [helpcontext][helpstring] */ 
enum InkOverlayEraserMode
    {
        IOERM_StrokeErase	= 0,
        IOERM_PointErase	= 1
    } 	InkOverlayEraserMode;

#define IOERM_Last IOERM_PointErase + 1
typedef 
enum InkCollectionMode
    {
        ICM_InkOnly	= 0,
        ICM_GestureOnly	= ( ICM_InkOnly + 1 ) ,
        ICM_InkAndGesture	= ( ICM_GestureOnly + 1 ) 
    } 	InkCollectionMode;

#define ICM_Last ICM_InkAndGesture
typedef /* [hidden] */ 
enum DISPID_InkCollector
    {
        DISPID_ICEnabled	= 1,
        DISPID_ICHwnd	= ( DISPID_ICEnabled + 1 ) ,
        DISPID_ICPaint	= ( DISPID_ICHwnd + 1 ) ,
        DISPID_ICText	= ( DISPID_ICPaint + 1 ) ,
        DISPID_ICDefaultDrawingAttributes	= ( DISPID_ICText + 1 ) ,
        DISPID_ICRenderer	= ( DISPID_ICDefaultDrawingAttributes + 1 ) ,
        DISPID_ICInk	= ( DISPID_ICRenderer + 1 ) ,
        DISPID_ICAutoRedraw	= ( DISPID_ICInk + 1 ) ,
        DISPID_ICCollectingInk	= ( DISPID_ICAutoRedraw + 1 ) ,
        DISPID_ICSetEventInterest	= ( DISPID_ICCollectingInk + 1 ) ,
        DISPID_ICGetEventInterest	= ( DISPID_ICSetEventInterest + 1 ) ,
        DISPID_IOEditingMode	= ( DISPID_ICGetEventInterest + 1 ) ,
        DISPID_IOSelection	= ( DISPID_IOEditingMode + 1 ) ,
        DISPID_IOAttachMode	= ( DISPID_IOSelection + 1 ) ,
        DISPID_IOHitTestSelection	= ( DISPID_IOAttachMode + 1 ) ,
        DISPID_IODraw	= ( DISPID_IOHitTestSelection + 1 ) ,
        DISPID_IPPicture	= ( DISPID_IODraw + 1 ) ,
        DISPID_IPSizeMode	= ( DISPID_IPPicture + 1 ) ,
        DISPID_IPBackColor	= ( DISPID_IPSizeMode + 1 ) ,
        DISPID_ICCursors	= ( DISPID_IPBackColor + 1 ) ,
        DISPID_ICMarginX	= ( DISPID_ICCursors + 1 ) ,
        DISPID_ICMarginY	= ( DISPID_ICMarginX + 1 ) ,
        DISPID_ICSetWindowInputRectangle	= ( DISPID_ICMarginY + 1 ) ,
        DISPID_ICGetWindowInputRectangle	= ( DISPID_ICSetWindowInputRectangle + 1 ) ,
        DISPID_ICTablet	= ( DISPID_ICGetWindowInputRectangle + 1 ) ,
        DISPID_ICSetAllTabletsMode	= ( DISPID_ICTablet + 1 ) ,
        DISPID_ICSetSingleTabletIntegratedMode	= ( DISPID_ICSetAllTabletsMode + 1 ) ,
        DISPID_ICCollectionMode	= ( DISPID_ICSetSingleTabletIntegratedMode + 1 ) ,
        DISPID_ICSetGestureStatus	= ( DISPID_ICCollectionMode + 1 ) ,
        DISPID_ICGetGestureStatus	= ( DISPID_ICSetGestureStatus + 1 ) ,
        DISPID_ICDynamicRendering	= ( DISPID_ICGetGestureStatus + 1 ) ,
        DISPID_ICDesiredPacketDescription	= ( DISPID_ICDynamicRendering + 1 ) ,
        DISPID_IOEraserMode	= ( DISPID_ICDesiredPacketDescription + 1 ) ,
        DISPID_IOEraserWidth	= ( DISPID_IOEraserMode + 1 ) ,
        DISPID_ICMouseIcon	= ( DISPID_IOEraserWidth + 1 ) ,
        DISPID_ICMousePointer	= ( DISPID_ICMouseIcon + 1 ) ,
        DISPID_IPInkEnabled	= ( DISPID_ICMousePointer + 1 ) ,
        DISPID_ICSupportHighContrastInk	= ( DISPID_IPInkEnabled + 1 ) ,
        DISPID_IOSupportHighContrastSelectionUI	= ( DISPID_ICSupportHighContrastInk + 1 ) 
    } 	DISPID_InkCollector;

typedef /* [hidden] */ 
enum DISPID_InkRecognizer
    {
        DISPID_RecoClsid	= 1,
        DISPID_RecoName	= ( DISPID_RecoClsid + 1 ) ,
        DISPID_RecoVendor	= ( DISPID_RecoName + 1 ) ,
        DISPID_RecoCapabilities	= ( DISPID_RecoVendor + 1 ) ,
        DISPID_RecoLanguageID	= ( DISPID_RecoCapabilities + 1 ) ,
        DISPID_RecoPreferredPacketDescription	= ( DISPID_RecoLanguageID + 1 ) ,
        DISPID_RecoCreateRecognizerContext	= ( DISPID_RecoPreferredPacketDescription + 1 ) ,
        DISPID_RecoSupportedProperties	= ( DISPID_RecoCreateRecognizerContext + 1 ) 
    } 	DISPID_InkRecognizer;

typedef 
enum InkRecognizerCapabilities
    {
        IRC_DontCare	= 1,
        IRC_Object	= 2,
        IRC_FreeInput	= 4,
        IRC_LinedInput	= 8,
        IRC_BoxedInput	= 16,
        IRC_CharacterAutoCompletionInput	= 32,
        IRC_RightAndDown	= 64,
        IRC_LeftAndDown	= 128,
        IRC_DownAndLeft	= 256,
        IRC_DownAndRight	= 512,
        IRC_ArbitraryAngle	= 1024,
        IRC_Lattice	= 2048,
        IRC_AdviseInkChange	= 4096,
        IRC_StrokeReorder	= 8192,
        IRC_Personalizable	= 16384,
        IRC_PrefersArbitraryAngle	= 32768,
        IRC_PrefersParagraphBreaking	= 65536,
        IRC_PrefersSegmentation	= 131072,
        IRC_Cursive	= 262144,
        IRC_TextPrediction	= 524288,
        IRC_Alpha	= 1048576,
        IRC_Beta	= 2097152
    } 	InkRecognizerCapabilities;

typedef /* [hidden] */ 
enum DISPID_InkRecognizer2
    {
        DISPID_RecoId	= DISPID_VALUE,
        DISPID_RecoUnicodeRanges	= 1
    } 	DISPID_InkRecognizer2;

typedef /* [hidden] */ 
enum DISPID_InkRecognizers
    {
        DISPID_IRecos_NewEnum	= DISPID_NEWENUM,
        DISPID_IRecosItem	= DISPID_VALUE,
        DISPID_IRecosCount	= 1,
        DISPID_IRecosGetDefaultRecognizer	= ( DISPID_IRecosCount + 1 ) 
    } 	DISPID_InkRecognizers;

typedef 
enum InkRecognizerCharacterAutoCompletionMode
    {
        IRCACM_Full	= 0,
        IRCACM_Prefix	= ( IRCACM_Full + 1 ) ,
        IRCACM_Random	= ( IRCACM_Prefix + 1 ) 
    } 	InkRecognizerCharacterAutoCompletionMode;

typedef 
enum InkRecognitionModes
    {
        IRM_None	= 0,
        IRM_WordModeOnly	= 0x1,
        IRM_Coerce	= 0x2,
        IRM_TopInkBreaksOnly	= 0x4,
        IRM_PrefixOk	= 0x8,
        IRM_LineMode	= 0x10,
        IRM_DisablePersonalization	= 0x20,
        IRM_AutoSpace	= 0x40,
        IRM_Max	= 0x80
    } 	InkRecognitionModes;

typedef /* [hidden] */ 
enum DISPID_InkRecognitionEvent
    {
        DISPID_IRERecognitionWithAlternates	= 1,
        DISPID_IRERecognition	= ( DISPID_IRERecognitionWithAlternates + 1 ) 
    } 	DISPID_InkRecognitionEvent;

typedef /* [hidden] */ 
enum DISPID_InkRecoContext
    {
        DISPID_IRecoCtx_Strokes	= 1,
        DISPID_IRecoCtx_CharacterAutoCompletionMode	= ( DISPID_IRecoCtx_Strokes + 1 ) ,
        DISPID_IRecoCtx_Factoid	= ( DISPID_IRecoCtx_CharacterAutoCompletionMode + 1 ) ,
        DISPID_IRecoCtx_WordList	= ( DISPID_IRecoCtx_Factoid + 1 ) ,
        DISPID_IRecoCtx_Recognizer	= ( DISPID_IRecoCtx_WordList + 1 ) ,
        DISPID_IRecoCtx_Guide	= ( DISPID_IRecoCtx_Recognizer + 1 ) ,
        DISPID_IRecoCtx_Flags	= ( DISPID_IRecoCtx_Guide + 1 ) ,
        DISPID_IRecoCtx_PrefixText	= ( DISPID_IRecoCtx_Flags + 1 ) ,
        DISPID_IRecoCtx_SuffixText	= ( DISPID_IRecoCtx_PrefixText + 1 ) ,
        DISPID_IRecoCtx_StopRecognition	= ( DISPID_IRecoCtx_SuffixText + 1 ) ,
        DISPID_IRecoCtx_Clone	= ( DISPID_IRecoCtx_StopRecognition + 1 ) ,
        DISPID_IRecoCtx_Recognize	= ( DISPID_IRecoCtx_Clone + 1 ) ,
        DISPID_IRecoCtx_StopBackgroundRecognition	= ( DISPID_IRecoCtx_Recognize + 1 ) ,
        DISPID_IRecoCtx_EndInkInput	= ( DISPID_IRecoCtx_StopBackgroundRecognition + 1 ) ,
        DISPID_IRecoCtx_BackgroundRecognize	= ( DISPID_IRecoCtx_EndInkInput + 1 ) ,
        DISPID_IRecoCtx_BackgroundRecognizeWithAlternates	= ( DISPID_IRecoCtx_BackgroundRecognize + 1 ) ,
        DISPID_IRecoCtx_IsStringSupported	= ( DISPID_IRecoCtx_BackgroundRecognizeWithAlternates + 1 ) 
    } 	DISPID_InkRecoContext;

typedef /* [hidden] */ 
enum DISPID_InkRecoContext2
    {
        DISPID_IRecoCtx2_EnabledUnicodeRanges	= DISPID_VALUE
    } 	DISPID_InkRecoContext2;

typedef /* [helpcontext][helpstring] */ 
enum InkRecognitionAlternatesSelection
    {
        IRAS_Start	= 0,
        IRAS_DefaultCount	= 10,
        IRAS_All	= -1
    } 	InkRecognitionAlternatesSelection;

typedef /* [hidden] */ 
enum DISPID_InkRecognitionResult
    {
        DISPID_InkRecognitionResult_TopString	= 1,
        DISPID_InkRecognitionResult_TopAlternate	= ( DISPID_InkRecognitionResult_TopString + 1 ) ,
        DISPID_InkRecognitionResult_Strokes	= ( DISPID_InkRecognitionResult_TopAlternate + 1 ) ,
        DISPID_InkRecognitionResult_TopConfidence	= ( DISPID_InkRecognitionResult_Strokes + 1 ) ,
        DISPID_InkRecognitionResult_AlternatesFromSelection	= ( DISPID_InkRecognitionResult_TopConfidence + 1 ) ,
        DISPID_InkRecognitionResult_ModifyTopAlternate	= ( DISPID_InkRecognitionResult_AlternatesFromSelection + 1 ) ,
        DISPID_InkRecognitionResult_SetResultOnStrokes	= ( DISPID_InkRecognitionResult_ModifyTopAlternate + 1 ) 
    } 	DISPID_InkRecognitionResult;

typedef /* [hidden] */ 
enum DISPID_InkRecoAlternate
    {
        DISPID_InkRecoAlternate_String	= 1,
        DISPID_InkRecoAlternate_LineNumber	= ( DISPID_InkRecoAlternate_String + 1 ) ,
        DISPID_InkRecoAlternate_Baseline	= ( DISPID_InkRecoAlternate_LineNumber + 1 ) ,
        DISPID_InkRecoAlternate_Midline	= ( DISPID_InkRecoAlternate_Baseline + 1 ) ,
        DISPID_InkRecoAlternate_Ascender	= ( DISPID_InkRecoAlternate_Midline + 1 ) ,
        DISPID_InkRecoAlternate_Descender	= ( DISPID_InkRecoAlternate_Ascender + 1 ) ,
        DISPID_InkRecoAlternate_Confidence	= ( DISPID_InkRecoAlternate_Descender + 1 ) ,
        DISPID_InkRecoAlternate_Strokes	= ( DISPID_InkRecoAlternate_Confidence + 1 ) ,
        DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges	= ( DISPID_InkRecoAlternate_Strokes + 1 ) ,
        DISPID_InkRecoAlternate_GetStrokesFromTextRange	= ( DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges + 1 ) ,
        DISPID_InkRecoAlternate_GetTextRangeFromStrokes	= ( DISPID_InkRecoAlternate_GetStrokesFromTextRange + 1 ) ,
        DISPID_InkRecoAlternate_GetPropertyValue	= ( DISPID_InkRecoAlternate_GetTextRangeFromStrokes + 1 ) ,
        DISPID_InkRecoAlternate_LineAlternates	= ( DISPID_InkRecoAlternate_GetPropertyValue + 1 ) ,
        DISPID_InkRecoAlternate_ConfidenceAlternates	= ( DISPID_InkRecoAlternate_LineAlternates + 1 ) ,
        DISPID_InkRecoAlternate_AlternatesWithConstantPropertyValues	= ( DISPID_InkRecoAlternate_ConfidenceAlternates + 1 ) 
    } 	DISPID_InkRecoAlternate;

typedef /* [hidden] */ 
enum DISPID_InkRecognitionAlternates
    {
        DISPID_InkRecognitionAlternates_NewEnum	= DISPID_NEWENUM,
        DISPID_InkRecognitionAlternates_Item	= DISPID_VALUE,
        DISPID_InkRecognitionAlternates_Count	= 1,
        DISPID_InkRecognitionAlternates_Strokes	= ( DISPID_InkRecognitionAlternates_Count + 1 ) 
    } 	DISPID_InkRecognitionAlternates;

typedef /* [hidden] */ struct _InkRecoGuide
    {
    RECT rectWritingBox;
    RECT rectDrawnBox;
    long cRows;
    long cColumns;
    long midline;
    } 	InkRecoGuide;

typedef /* [hidden] */ 
enum DISPID_InkRecognizerGuide
    {
        DISPID_IRGWritingBox	= 1,
        DISPID_IRGDrawnBox	= ( DISPID_IRGWritingBox + 1 ) ,
        DISPID_IRGRows	= ( DISPID_IRGDrawnBox + 1 ) ,
        DISPID_IRGColumns	= ( DISPID_IRGRows + 1 ) ,
        DISPID_IRGMidline	= ( DISPID_IRGColumns + 1 ) ,
        DISPID_IRGGuideData	= ( DISPID_IRGMidline + 1 ) 
    } 	DISPID_InkRecognizerGuide;

typedef /* [hidden] */ 
enum DISPID_InkWordList
    {
        DISPID_InkWordList_AddWord	= 0,
        DISPID_InkWordList_RemoveWord	= ( DISPID_InkWordList_AddWord + 1 ) ,
        DISPID_InkWordList_Merge	= ( DISPID_InkWordList_RemoveWord + 1 ) 
    } 	DISPID_InkWordList;

typedef /* [hidden] */ 
enum DISPID_InkWordList2
    {
        DISPID_InkWordList2_AddWords	= ( DISPID_InkWordList_Merge + 1 ) 
    } 	DISPID_InkWordList2;


EXTERN_C const IID LIBID_MSINKAUTLib;


#ifndef __InkConstants_MODULE_DEFINED__
#define __InkConstants_MODULE_DEFINED__


/* module InkConstants */
/* [uuid] */ 

const LONG InkMinTransparencyValue	=	0;

const LONG InkMaxTransparencyValue	=	255;

#endif /* __InkConstants_MODULE_DEFINED__ */


#ifndef __InkCollectorConstants_MODULE_DEFINED__
#define __InkCollectorConstants_MODULE_DEFINED__


/* module InkCollectorConstants */
/* [uuid] */ 

const LONG InkCollectorClipInkToMargin	=	0;

const LONG InkCollectorDefaultMargin	=	( -2147483647L - 1 ) ;

#endif /* __InkCollectorConstants_MODULE_DEFINED__ */

#ifndef __IInkRectangle_INTERFACE_DEFINED__
#define __IInkRectangle_INTERFACE_DEFINED__

/* interface IInkRectangle */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRectangle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9794FF82-6071-4717-8A8B-6AC7C64A686E")
    IInkRectangle : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Top( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Top( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Left( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Left( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Bottom( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Bottom( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Right( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Right( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__out RECT *Rect) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ RECT Rect) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRectangle( 
            /* [out] */ __RPC__out long *Top,
            /* [out] */ __RPC__out long *Left,
            /* [out] */ __RPC__out long *Bottom,
            /* [out] */ __RPC__out long *Right) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetRectangle( 
            /* [in] */ long Top,
            /* [in] */ long Left,
            /* [in] */ long Bottom,
            /* [in] */ long Right) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRectangleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRectangle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRectangle * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRectangle * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRectangle * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRectangle, get_Top)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Top )( 
            __RPC__in IInkRectangle * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, put_Top)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Top )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, get_Left)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Left )( 
            __RPC__in IInkRectangle * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, put_Left)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Left )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, get_Bottom)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Bottom )( 
            __RPC__in IInkRectangle * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, put_Bottom)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Bottom )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, get_Right)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Right )( 
            __RPC__in IInkRectangle * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, put_Right)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Right )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRectangle, get_Data)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IInkRectangle * This,
            /* [retval][out] */ __RPC__out RECT *Rect);
        
        DECLSPEC_XFGVIRT(IInkRectangle, put_Data)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ RECT Rect);
        
        DECLSPEC_XFGVIRT(IInkRectangle, GetRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRectangle )( 
            __RPC__in IInkRectangle * This,
            /* [out] */ __RPC__out long *Top,
            /* [out] */ __RPC__out long *Left,
            /* [out] */ __RPC__out long *Bottom,
            /* [out] */ __RPC__out long *Right);
        
        DECLSPEC_XFGVIRT(IInkRectangle, SetRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetRectangle )( 
            __RPC__in IInkRectangle * This,
            /* [in] */ long Top,
            /* [in] */ long Left,
            /* [in] */ long Bottom,
            /* [in] */ long Right);
        
        END_INTERFACE
    } IInkRectangleVtbl;

    interface IInkRectangle
    {
        CONST_VTBL struct IInkRectangleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRectangle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRectangle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRectangle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRectangle_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRectangle_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRectangle_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRectangle_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRectangle_get_Top(This,Units)	\
    ( (This)->lpVtbl -> get_Top(This,Units) ) 

#define IInkRectangle_put_Top(This,Units)	\
    ( (This)->lpVtbl -> put_Top(This,Units) ) 

#define IInkRectangle_get_Left(This,Units)	\
    ( (This)->lpVtbl -> get_Left(This,Units) ) 

#define IInkRectangle_put_Left(This,Units)	\
    ( (This)->lpVtbl -> put_Left(This,Units) ) 

#define IInkRectangle_get_Bottom(This,Units)	\
    ( (This)->lpVtbl -> get_Bottom(This,Units) ) 

#define IInkRectangle_put_Bottom(This,Units)	\
    ( (This)->lpVtbl -> put_Bottom(This,Units) ) 

#define IInkRectangle_get_Right(This,Units)	\
    ( (This)->lpVtbl -> get_Right(This,Units) ) 

#define IInkRectangle_put_Right(This,Units)	\
    ( (This)->lpVtbl -> put_Right(This,Units) ) 

#define IInkRectangle_get_Data(This,Rect)	\
    ( (This)->lpVtbl -> get_Data(This,Rect) ) 

#define IInkRectangle_put_Data(This,Rect)	\
    ( (This)->lpVtbl -> put_Data(This,Rect) ) 

#define IInkRectangle_GetRectangle(This,Top,Left,Bottom,Right)	\
    ( (This)->lpVtbl -> GetRectangle(This,Top,Left,Bottom,Right) ) 

#define IInkRectangle_SetRectangle(This,Top,Left,Bottom,Right)	\
    ( (This)->lpVtbl -> SetRectangle(This,Top,Left,Bottom,Right) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRectangle_INTERFACE_DEFINED__ */


#ifndef __IInkExtendedProperty_INTERFACE_DEFINED__
#define __IInkExtendedProperty_INTERFACE_DEFINED__

/* interface IInkExtendedProperty */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkExtendedProperty;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DB489209-B7C3-411D-90F6-1548CFFF271E")
    IInkExtendedProperty : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Guid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Guid) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__out VARIANT *Data) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ VARIANT Data) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkExtendedPropertyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkExtendedProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkExtendedProperty * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkExtendedProperty * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkExtendedProperty * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkExtendedProperty * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkExtendedProperty * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkExtendedProperty * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperty, get_Guid)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guid )( 
            __RPC__in IInkExtendedProperty * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Guid);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperty, get_Data)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IInkExtendedProperty * This,
            /* [retval][out] */ __RPC__out VARIANT *Data);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperty, put_Data)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IInkExtendedProperty * This,
            /* [in] */ VARIANT Data);
        
        END_INTERFACE
    } IInkExtendedPropertyVtbl;

    interface IInkExtendedProperty
    {
        CONST_VTBL struct IInkExtendedPropertyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkExtendedProperty_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkExtendedProperty_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkExtendedProperty_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkExtendedProperty_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkExtendedProperty_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkExtendedProperty_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkExtendedProperty_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkExtendedProperty_get_Guid(This,Guid)	\
    ( (This)->lpVtbl -> get_Guid(This,Guid) ) 

#define IInkExtendedProperty_get_Data(This,Data)	\
    ( (This)->lpVtbl -> get_Data(This,Data) ) 

#define IInkExtendedProperty_put_Data(This,Data)	\
    ( (This)->lpVtbl -> put_Data(This,Data) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkExtendedProperty_INTERFACE_DEFINED__ */


#ifndef __IInkExtendedProperties_INTERFACE_DEFINED__
#define __IInkExtendedProperties_INTERFACE_DEFINED__

/* interface IInkExtendedProperties */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkExtendedProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89F2A8BE-95A9-4530-8B8F-88E971E3E25F")
    IInkExtendedProperties : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ VARIANT Identifier,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperty **Item) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR Guid,
            /* [in] */ VARIANT Data,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperty **InkExtendedProperty) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT Identifier) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE DoesPropertyExist( 
            /* [in] */ __RPC__in BSTR Guid,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *DoesPropertyExist) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkExtendedPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkExtendedProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkExtendedProperties * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkExtendedProperties * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkExtendedProperties * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkExtendedProperties * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkExtendedProperties * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ VARIANT Identifier,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperty **Item);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, Add)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ __RPC__in BSTR Guid,
            /* [in] */ VARIANT Data,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperty **InkExtendedProperty);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, Remove)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ VARIANT Identifier);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, Clear)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IInkExtendedProperties * This);
        
        DECLSPEC_XFGVIRT(IInkExtendedProperties, DoesPropertyExist)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DoesPropertyExist )( 
            __RPC__in IInkExtendedProperties * This,
            /* [in] */ __RPC__in BSTR Guid,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *DoesPropertyExist);
        
        END_INTERFACE
    } IInkExtendedPropertiesVtbl;

    interface IInkExtendedProperties
    {
        CONST_VTBL struct IInkExtendedPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkExtendedProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkExtendedProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkExtendedProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkExtendedProperties_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkExtendedProperties_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkExtendedProperties_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkExtendedProperties_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkExtendedProperties_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkExtendedProperties_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkExtendedProperties_Item(This,Identifier,Item)	\
    ( (This)->lpVtbl -> Item(This,Identifier,Item) ) 

#define IInkExtendedProperties_Add(This,Guid,Data,InkExtendedProperty)	\
    ( (This)->lpVtbl -> Add(This,Guid,Data,InkExtendedProperty) ) 

#define IInkExtendedProperties_Remove(This,Identifier)	\
    ( (This)->lpVtbl -> Remove(This,Identifier) ) 

#define IInkExtendedProperties_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IInkExtendedProperties_DoesPropertyExist(This,Guid,DoesPropertyExist)	\
    ( (This)->lpVtbl -> DoesPropertyExist(This,Guid,DoesPropertyExist) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkExtendedProperties_INTERFACE_DEFINED__ */


#ifndef __IInkDrawingAttributes_INTERFACE_DEFINED__
#define __IInkDrawingAttributes_INTERFACE_DEFINED__

/* interface IInkDrawingAttributes */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkDrawingAttributes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BF519B75-0A15-4623-ADC9-C00D436A8092")
    IInkDrawingAttributes : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Color( 
            /* [retval][out] */ __RPC__out long *CurrentColor) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Color( 
            /* [in] */ long NewColor) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Width( 
            /* [retval][out] */ __RPC__out float *CurrentWidth) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Width( 
            /* [in] */ float NewWidth) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Height( 
            /* [retval][out] */ __RPC__out float *CurrentHeight) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Height( 
            /* [in] */ float NewHeight) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_FitToCurve( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Flag) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_FitToCurve( 
            /* [in] */ VARIANT_BOOL Flag) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IgnorePressure( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Flag) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_IgnorePressure( 
            /* [in] */ VARIANT_BOOL Flag) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AntiAliased( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Flag) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AntiAliased( 
            /* [in] */ VARIANT_BOOL Flag) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Transparency( 
            /* [retval][out] */ __RPC__out long *CurrentTransparency) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Transparency( 
            /* [in] */ long NewTransparency) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RasterOperation( 
            /* [retval][out] */ __RPC__out InkRasterOperation *CurrentRasterOperation) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RasterOperation( 
            /* [in] */ InkRasterOperation NewRasterOperation) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PenTip( 
            /* [retval][out] */ __RPC__out InkPenTip *CurrentPenTip) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PenTip( 
            /* [in] */ InkPenTip NewPenTip) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperties **Properties) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **DrawingAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDrawingAttributesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDrawingAttributes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDrawingAttributes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkDrawingAttributes * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_Color)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Color )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out long *CurrentColor);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_Color)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Color )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ long NewColor);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_Width)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Width )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out float *CurrentWidth);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_Width)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Width )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ float NewWidth);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_Height)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Height )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out float *CurrentHeight);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_Height)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Height )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ float NewHeight);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_FitToCurve)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_FitToCurve )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Flag);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_FitToCurve)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_FitToCurve )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ VARIANT_BOOL Flag);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_IgnorePressure)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IgnorePressure )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Flag);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_IgnorePressure)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_IgnorePressure )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ VARIANT_BOOL Flag);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_AntiAliased)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AntiAliased )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Flag);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_AntiAliased)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AntiAliased )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ VARIANT_BOOL Flag);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_Transparency)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Transparency )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out long *CurrentTransparency);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_Transparency)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Transparency )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ long NewTransparency);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_RasterOperation)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RasterOperation )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out InkRasterOperation *CurrentRasterOperation);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_RasterOperation)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RasterOperation )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ InkRasterOperation NewRasterOperation);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_PenTip)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PenTip )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__out InkPenTip *CurrentPenTip);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, put_PenTip)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PenTip )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [in] */ InkPenTip NewPenTip);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, get_ExtendedProperties)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedProperties )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperties **Properties);
        
        DECLSPEC_XFGVIRT(IInkDrawingAttributes, Clone)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IInkDrawingAttributes * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **DrawingAttributes);
        
        END_INTERFACE
    } IInkDrawingAttributesVtbl;

    interface IInkDrawingAttributes
    {
        CONST_VTBL struct IInkDrawingAttributesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDrawingAttributes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDrawingAttributes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDrawingAttributes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDrawingAttributes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkDrawingAttributes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkDrawingAttributes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkDrawingAttributes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkDrawingAttributes_get_Color(This,CurrentColor)	\
    ( (This)->lpVtbl -> get_Color(This,CurrentColor) ) 

#define IInkDrawingAttributes_put_Color(This,NewColor)	\
    ( (This)->lpVtbl -> put_Color(This,NewColor) ) 

#define IInkDrawingAttributes_get_Width(This,CurrentWidth)	\
    ( (This)->lpVtbl -> get_Width(This,CurrentWidth) ) 

#define IInkDrawingAttributes_put_Width(This,NewWidth)	\
    ( (This)->lpVtbl -> put_Width(This,NewWidth) ) 

#define IInkDrawingAttributes_get_Height(This,CurrentHeight)	\
    ( (This)->lpVtbl -> get_Height(This,CurrentHeight) ) 

#define IInkDrawingAttributes_put_Height(This,NewHeight)	\
    ( (This)->lpVtbl -> put_Height(This,NewHeight) ) 

#define IInkDrawingAttributes_get_FitToCurve(This,Flag)	\
    ( (This)->lpVtbl -> get_FitToCurve(This,Flag) ) 

#define IInkDrawingAttributes_put_FitToCurve(This,Flag)	\
    ( (This)->lpVtbl -> put_FitToCurve(This,Flag) ) 

#define IInkDrawingAttributes_get_IgnorePressure(This,Flag)	\
    ( (This)->lpVtbl -> get_IgnorePressure(This,Flag) ) 

#define IInkDrawingAttributes_put_IgnorePressure(This,Flag)	\
    ( (This)->lpVtbl -> put_IgnorePressure(This,Flag) ) 

#define IInkDrawingAttributes_get_AntiAliased(This,Flag)	\
    ( (This)->lpVtbl -> get_AntiAliased(This,Flag) ) 

#define IInkDrawingAttributes_put_AntiAliased(This,Flag)	\
    ( (This)->lpVtbl -> put_AntiAliased(This,Flag) ) 

#define IInkDrawingAttributes_get_Transparency(This,CurrentTransparency)	\
    ( (This)->lpVtbl -> get_Transparency(This,CurrentTransparency) ) 

#define IInkDrawingAttributes_put_Transparency(This,NewTransparency)	\
    ( (This)->lpVtbl -> put_Transparency(This,NewTransparency) ) 

#define IInkDrawingAttributes_get_RasterOperation(This,CurrentRasterOperation)	\
    ( (This)->lpVtbl -> get_RasterOperation(This,CurrentRasterOperation) ) 

#define IInkDrawingAttributes_put_RasterOperation(This,NewRasterOperation)	\
    ( (This)->lpVtbl -> put_RasterOperation(This,NewRasterOperation) ) 

#define IInkDrawingAttributes_get_PenTip(This,CurrentPenTip)	\
    ( (This)->lpVtbl -> get_PenTip(This,CurrentPenTip) ) 

#define IInkDrawingAttributes_put_PenTip(This,NewPenTip)	\
    ( (This)->lpVtbl -> put_PenTip(This,NewPenTip) ) 

#define IInkDrawingAttributes_get_ExtendedProperties(This,Properties)	\
    ( (This)->lpVtbl -> get_ExtendedProperties(This,Properties) ) 

#define IInkDrawingAttributes_Clone(This,DrawingAttributes)	\
    ( (This)->lpVtbl -> Clone(This,DrawingAttributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDrawingAttributes_INTERFACE_DEFINED__ */


#ifndef __IInkTransform_INTERFACE_DEFINED__
#define __IInkTransform_INTERFACE_DEFINED__

/* interface IInkTransform */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("615F1D43-8703-4565-88E2-8201D2ECD7B7")
    IInkTransform : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Translate( 
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Rotate( 
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x = 0,
            /* [defaultvalue][in] */ float y = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Reflect( 
            /* [in] */ VARIANT_BOOL Horizontally,
            /* [in] */ VARIANT_BOOL Vertically) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Shear( 
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleTransform( 
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTransform( 
            /* [out] */ __RPC__out float *eM11,
            /* [out] */ __RPC__out float *eM12,
            /* [out] */ __RPC__out float *eM21,
            /* [out] */ __RPC__out float *eM22,
            /* [out] */ __RPC__out float *eDx,
            /* [out] */ __RPC__out float *eDy) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetTransform( 
            /* [in] */ float eM11,
            /* [in] */ float eM12,
            /* [in] */ float eM21,
            /* [in] */ float eM22,
            /* [in] */ float eDx,
            /* [in] */ float eDy) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_eM11( 
            /* [retval][out] */ __RPC__out float *Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_eM11( 
            /* [in] */ float Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_eM12( 
            /* [retval][out] */ __RPC__out float *Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_eM12( 
            /* [in] */ float Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_eM21( 
            /* [retval][out] */ __RPC__out float *Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_eM21( 
            /* [in] */ float Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_eM22( 
            /* [retval][out] */ __RPC__out float *Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_eM22( 
            /* [in] */ float Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_eDx( 
            /* [retval][out] */ __RPC__out float *Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_eDx( 
            /* [in] */ float Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_eDy( 
            /* [retval][out] */ __RPC__out float *Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_eDy( 
            /* [in] */ float Value) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_Data( 
            /* [retval][out] */ __RPC__out XFORM *XForm) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_Data( 
            /* [in] */ XFORM XForm) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkTransform * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkTransform * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkTransform * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkTransform * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkTransform * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkTransform, Reset)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IInkTransform * This);
        
        DECLSPEC_XFGVIRT(IInkTransform, Translate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Translate )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent);
        
        DECLSPEC_XFGVIRT(IInkTransform, Rotate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x,
            /* [defaultvalue][in] */ float y);
        
        DECLSPEC_XFGVIRT(IInkTransform, Reflect)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Reflect )( 
            __RPC__in IInkTransform * This,
            /* [in] */ VARIANT_BOOL Horizontally,
            /* [in] */ VARIANT_BOOL Vertically);
        
        DECLSPEC_XFGVIRT(IInkTransform, Shear)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Shear )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent);
        
        DECLSPEC_XFGVIRT(IInkTransform, ScaleTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleTransform )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier);
        
        DECLSPEC_XFGVIRT(IInkTransform, GetTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IInkTransform * This,
            /* [out] */ __RPC__out float *eM11,
            /* [out] */ __RPC__out float *eM12,
            /* [out] */ __RPC__out float *eM21,
            /* [out] */ __RPC__out float *eM22,
            /* [out] */ __RPC__out float *eDx,
            /* [out] */ __RPC__out float *eDy);
        
        DECLSPEC_XFGVIRT(IInkTransform, SetTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetTransform )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float eM11,
            /* [in] */ float eM12,
            /* [in] */ float eM21,
            /* [in] */ float eM22,
            /* [in] */ float eDx,
            /* [in] */ float eDy);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_eM11)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_eM11 )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out float *Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_eM11)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_eM11 )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_eM12)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_eM12 )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out float *Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_eM12)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_eM12 )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_eM21)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_eM21 )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out float *Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_eM21)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_eM21 )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_eM22)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_eM22 )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out float *Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_eM22)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_eM22 )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_eDx)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_eDx )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out float *Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_eDx)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_eDx )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_eDy)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_eDy )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out float *Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_eDy)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_eDy )( 
            __RPC__in IInkTransform * This,
            /* [in] */ float Value);
        
        DECLSPEC_XFGVIRT(IInkTransform, get_Data)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Data )( 
            __RPC__in IInkTransform * This,
            /* [retval][out] */ __RPC__out XFORM *XForm);
        
        DECLSPEC_XFGVIRT(IInkTransform, put_Data)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Data )( 
            __RPC__in IInkTransform * This,
            /* [in] */ XFORM XForm);
        
        END_INTERFACE
    } IInkTransformVtbl;

    interface IInkTransform
    {
        CONST_VTBL struct IInkTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkTransform_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkTransform_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkTransform_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkTransform_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkTransform_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IInkTransform_Translate(This,HorizontalComponent,VerticalComponent)	\
    ( (This)->lpVtbl -> Translate(This,HorizontalComponent,VerticalComponent) ) 

#define IInkTransform_Rotate(This,Degrees,x,y)	\
    ( (This)->lpVtbl -> Rotate(This,Degrees,x,y) ) 

#define IInkTransform_Reflect(This,Horizontally,Vertically)	\
    ( (This)->lpVtbl -> Reflect(This,Horizontally,Vertically) ) 

#define IInkTransform_Shear(This,HorizontalComponent,VerticalComponent)	\
    ( (This)->lpVtbl -> Shear(This,HorizontalComponent,VerticalComponent) ) 

#define IInkTransform_ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier)	\
    ( (This)->lpVtbl -> ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier) ) 

#define IInkTransform_GetTransform(This,eM11,eM12,eM21,eM22,eDx,eDy)	\
    ( (This)->lpVtbl -> GetTransform(This,eM11,eM12,eM21,eM22,eDx,eDy) ) 

#define IInkTransform_SetTransform(This,eM11,eM12,eM21,eM22,eDx,eDy)	\
    ( (This)->lpVtbl -> SetTransform(This,eM11,eM12,eM21,eM22,eDx,eDy) ) 

#define IInkTransform_get_eM11(This,Value)	\
    ( (This)->lpVtbl -> get_eM11(This,Value) ) 

#define IInkTransform_put_eM11(This,Value)	\
    ( (This)->lpVtbl -> put_eM11(This,Value) ) 

#define IInkTransform_get_eM12(This,Value)	\
    ( (This)->lpVtbl -> get_eM12(This,Value) ) 

#define IInkTransform_put_eM12(This,Value)	\
    ( (This)->lpVtbl -> put_eM12(This,Value) ) 

#define IInkTransform_get_eM21(This,Value)	\
    ( (This)->lpVtbl -> get_eM21(This,Value) ) 

#define IInkTransform_put_eM21(This,Value)	\
    ( (This)->lpVtbl -> put_eM21(This,Value) ) 

#define IInkTransform_get_eM22(This,Value)	\
    ( (This)->lpVtbl -> get_eM22(This,Value) ) 

#define IInkTransform_put_eM22(This,Value)	\
    ( (This)->lpVtbl -> put_eM22(This,Value) ) 

#define IInkTransform_get_eDx(This,Value)	\
    ( (This)->lpVtbl -> get_eDx(This,Value) ) 

#define IInkTransform_put_eDx(This,Value)	\
    ( (This)->lpVtbl -> put_eDx(This,Value) ) 

#define IInkTransform_get_eDy(This,Value)	\
    ( (This)->lpVtbl -> get_eDy(This,Value) ) 

#define IInkTransform_put_eDy(This,Value)	\
    ( (This)->lpVtbl -> put_eDy(This,Value) ) 

#define IInkTransform_get_Data(This,XForm)	\
    ( (This)->lpVtbl -> get_Data(This,XForm) ) 

#define IInkTransform_put_Data(This,XForm)	\
    ( (This)->lpVtbl -> put_Data(This,XForm) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkTransform_INTERFACE_DEFINED__ */


#ifndef __IInkGesture_INTERFACE_DEFINED__
#define __IInkGesture_INTERFACE_DEFINED__

/* interface IInkGesture */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkGesture;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3BDC0A97-04E5-4e26-B813-18F052D41DEF")
    IInkGesture : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Confidence( 
            /* [retval][out] */ __RPC__out InkRecognitionConfidence *Confidence) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out InkApplicationGesture *Id) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetHotPoint( 
            /* [out][in] */ __RPC__inout long *X,
            /* [out][in] */ __RPC__inout long *Y) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkGestureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkGesture * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkGesture * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkGesture * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkGesture * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkGesture * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkGesture * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkGesture * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkGesture, get_Confidence)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Confidence )( 
            __RPC__in IInkGesture * This,
            /* [retval][out] */ __RPC__out InkRecognitionConfidence *Confidence);
        
        DECLSPEC_XFGVIRT(IInkGesture, get_Id)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IInkGesture * This,
            /* [retval][out] */ __RPC__out InkApplicationGesture *Id);
        
        DECLSPEC_XFGVIRT(IInkGesture, GetHotPoint)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetHotPoint )( 
            __RPC__in IInkGesture * This,
            /* [out][in] */ __RPC__inout long *X,
            /* [out][in] */ __RPC__inout long *Y);
        
        END_INTERFACE
    } IInkGestureVtbl;

    interface IInkGesture
    {
        CONST_VTBL struct IInkGestureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkGesture_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkGesture_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkGesture_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkGesture_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkGesture_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkGesture_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkGesture_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkGesture_get_Confidence(This,Confidence)	\
    ( (This)->lpVtbl -> get_Confidence(This,Confidence) ) 

#define IInkGesture_get_Id(This,Id)	\
    ( (This)->lpVtbl -> get_Id(This,Id) ) 

#define IInkGesture_GetHotPoint(This,X,Y)	\
    ( (This)->lpVtbl -> GetHotPoint(This,X,Y) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkGesture_INTERFACE_DEFINED__ */


#ifndef __IInkCursor_INTERFACE_DEFINED__
#define __IInkCursor_INTERFACE_DEFINED__

/* interface IInkCursor */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkCursor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AD30C630-40C5-4350-8405-9C71012FC558")
    IInkCursor : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__out long *Id) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Inverted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Status) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **Attributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_DrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *Attributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tablet( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **Tablet) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Buttons( 
            /* [retval][out] */ __RPC__deref_out_opt IInkCursorButtons **Buttons) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCursorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCursor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCursor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCursor * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkCursor * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkCursor * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkCursor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkCursor * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkCursor, get_Name)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IInkCursor * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(IInkCursor, get_Id)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IInkCursor * This,
            /* [retval][out] */ __RPC__out long *Id);
        
        DECLSPEC_XFGVIRT(IInkCursor, get_Inverted)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Inverted )( 
            __RPC__in IInkCursor * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Status);
        
        DECLSPEC_XFGVIRT(IInkCursor, get_DrawingAttributes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DrawingAttributes )( 
            __RPC__in IInkCursor * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **Attributes);
        
        DECLSPEC_XFGVIRT(IInkCursor, putref_DrawingAttributes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_DrawingAttributes )( 
            __RPC__in IInkCursor * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *Attributes);
        
        DECLSPEC_XFGVIRT(IInkCursor, get_Tablet)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tablet )( 
            __RPC__in IInkCursor * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **Tablet);
        
        DECLSPEC_XFGVIRT(IInkCursor, get_Buttons)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Buttons )( 
            __RPC__in IInkCursor * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursorButtons **Buttons);
        
        END_INTERFACE
    } IInkCursorVtbl;

    interface IInkCursor
    {
        CONST_VTBL struct IInkCursorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCursor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCursor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCursor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCursor_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkCursor_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkCursor_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkCursor_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkCursor_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define IInkCursor_get_Id(This,Id)	\
    ( (This)->lpVtbl -> get_Id(This,Id) ) 

#define IInkCursor_get_Inverted(This,Status)	\
    ( (This)->lpVtbl -> get_Inverted(This,Status) ) 

#define IInkCursor_get_DrawingAttributes(This,Attributes)	\
    ( (This)->lpVtbl -> get_DrawingAttributes(This,Attributes) ) 

#define IInkCursor_putref_DrawingAttributes(This,Attributes)	\
    ( (This)->lpVtbl -> putref_DrawingAttributes(This,Attributes) ) 

#define IInkCursor_get_Tablet(This,Tablet)	\
    ( (This)->lpVtbl -> get_Tablet(This,Tablet) ) 

#define IInkCursor_get_Buttons(This,Buttons)	\
    ( (This)->lpVtbl -> get_Buttons(This,Buttons) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCursor_INTERFACE_DEFINED__ */


#ifndef __IInkCursors_INTERFACE_DEFINED__
#define __IInkCursors_INTERFACE_DEFINED__

/* interface IInkCursors */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkCursors;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A248C1AC-C698-4e06-9E5C-D57F77C7E647")
    IInkCursors : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursor **Cursor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCursorsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCursors * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCursors * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCursors * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkCursors * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkCursors * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkCursors * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkCursors * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkCursors, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkCursors * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkCursors, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkCursors * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkCursors, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkCursors * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursor **Cursor);
        
        END_INTERFACE
    } IInkCursorsVtbl;

    interface IInkCursors
    {
        CONST_VTBL struct IInkCursorsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCursors_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCursors_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCursors_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCursors_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkCursors_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkCursors_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkCursors_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkCursors_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkCursors_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkCursors_Item(This,Index,Cursor)	\
    ( (This)->lpVtbl -> Item(This,Index,Cursor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCursors_INTERFACE_DEFINED__ */


#ifndef __IInkCursorButton_INTERFACE_DEFINED__
#define __IInkCursorButton_INTERFACE_DEFINED__

/* interface IInkCursorButton */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkCursorButton;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("85EF9417-1D59-49b2-A13C-702C85430894")
    IInkCursorButton : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Id) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out InkCursorButtonState *CurrentState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCursorButtonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCursorButton * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCursorButton * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCursorButton * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkCursorButton * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkCursorButton * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkCursorButton * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkCursorButton * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkCursorButton, get_Name)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IInkCursorButton * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(IInkCursorButton, get_Id)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IInkCursorButton * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Id);
        
        DECLSPEC_XFGVIRT(IInkCursorButton, get_State)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in IInkCursorButton * This,
            /* [retval][out] */ __RPC__out InkCursorButtonState *CurrentState);
        
        END_INTERFACE
    } IInkCursorButtonVtbl;

    interface IInkCursorButton
    {
        CONST_VTBL struct IInkCursorButtonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCursorButton_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCursorButton_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCursorButton_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCursorButton_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkCursorButton_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkCursorButton_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkCursorButton_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkCursorButton_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define IInkCursorButton_get_Id(This,Id)	\
    ( (This)->lpVtbl -> get_Id(This,Id) ) 

#define IInkCursorButton_get_State(This,CurrentState)	\
    ( (This)->lpVtbl -> get_State(This,CurrentState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCursorButton_INTERFACE_DEFINED__ */


#ifndef __IInkCursorButtons_INTERFACE_DEFINED__
#define __IInkCursorButtons_INTERFACE_DEFINED__

/* interface IInkCursorButtons */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkCursorButtons;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3671CC40-B624-4671-9FA0-DB119D952D54")
    IInkCursorButtons : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ VARIANT Identifier,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursorButton **Button) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCursorButtonsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCursorButtons * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCursorButtons * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCursorButtons * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkCursorButtons * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkCursorButtons * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkCursorButtons * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkCursorButtons * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkCursorButtons, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkCursorButtons * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkCursorButtons, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkCursorButtons * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkCursorButtons, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkCursorButtons * This,
            /* [in] */ VARIANT Identifier,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursorButton **Button);
        
        END_INTERFACE
    } IInkCursorButtonsVtbl;

    interface IInkCursorButtons
    {
        CONST_VTBL struct IInkCursorButtonsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCursorButtons_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCursorButtons_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCursorButtons_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCursorButtons_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkCursorButtons_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkCursorButtons_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkCursorButtons_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkCursorButtons_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkCursorButtons_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkCursorButtons_Item(This,Identifier,Button)	\
    ( (This)->lpVtbl -> Item(This,Identifier,Button) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCursorButtons_INTERFACE_DEFINED__ */


#ifndef __IInkTablet_INTERFACE_DEFINED__
#define __IInkTablet_INTERFACE_DEFINED__

/* interface IInkTablet */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkTablet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2DE25EAA-6EF8-42d5-AEE9-185BC81B912D")
    IInkTablet : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PlugAndPlayId( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Id) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaximumInputRectangle( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_HardwareCapabilities( 
            /* [retval][out] */ __RPC__out TabletHardwareCapabilities *Capabilities) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE IsPacketPropertySupported( 
            /* [in] */ __RPC__in BSTR packetPropertyName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Supported) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPropertyMetrics( 
            /* [in] */ __RPC__in BSTR propertyName,
            /* [out] */ __RPC__out long *Minimum,
            /* [out] */ __RPC__out long *Maximum,
            /* [out] */ __RPC__out TabletPropertyMetricUnit *Units,
            /* [out] */ __RPC__out float *Resolution) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkTabletVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkTablet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkTablet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkTablet * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkTablet * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkTablet * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkTablet * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkTablet * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkTablet, get_Name)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IInkTablet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(IInkTablet, get_PlugAndPlayId)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PlugAndPlayId )( 
            __RPC__in IInkTablet * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Id);
        
        DECLSPEC_XFGVIRT(IInkTablet, get_MaximumInputRectangle)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumInputRectangle )( 
            __RPC__in IInkTablet * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkTablet, get_HardwareCapabilities)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_HardwareCapabilities )( 
            __RPC__in IInkTablet * This,
            /* [retval][out] */ __RPC__out TabletHardwareCapabilities *Capabilities);
        
        DECLSPEC_XFGVIRT(IInkTablet, IsPacketPropertySupported)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsPacketPropertySupported )( 
            __RPC__in IInkTablet * This,
            /* [in] */ __RPC__in BSTR packetPropertyName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Supported);
        
        DECLSPEC_XFGVIRT(IInkTablet, GetPropertyMetrics)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyMetrics )( 
            __RPC__in IInkTablet * This,
            /* [in] */ __RPC__in BSTR propertyName,
            /* [out] */ __RPC__out long *Minimum,
            /* [out] */ __RPC__out long *Maximum,
            /* [out] */ __RPC__out TabletPropertyMetricUnit *Units,
            /* [out] */ __RPC__out float *Resolution);
        
        END_INTERFACE
    } IInkTabletVtbl;

    interface IInkTablet
    {
        CONST_VTBL struct IInkTabletVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkTablet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkTablet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkTablet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkTablet_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkTablet_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkTablet_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkTablet_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkTablet_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define IInkTablet_get_PlugAndPlayId(This,Id)	\
    ( (This)->lpVtbl -> get_PlugAndPlayId(This,Id) ) 

#define IInkTablet_get_MaximumInputRectangle(This,Rectangle)	\
    ( (This)->lpVtbl -> get_MaximumInputRectangle(This,Rectangle) ) 

#define IInkTablet_get_HardwareCapabilities(This,Capabilities)	\
    ( (This)->lpVtbl -> get_HardwareCapabilities(This,Capabilities) ) 

#define IInkTablet_IsPacketPropertySupported(This,packetPropertyName,Supported)	\
    ( (This)->lpVtbl -> IsPacketPropertySupported(This,packetPropertyName,Supported) ) 

#define IInkTablet_GetPropertyMetrics(This,propertyName,Minimum,Maximum,Units,Resolution)	\
    ( (This)->lpVtbl -> GetPropertyMetrics(This,propertyName,Minimum,Maximum,Units,Resolution) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkTablet_INTERFACE_DEFINED__ */


#ifndef __IInkTablet2_INTERFACE_DEFINED__
#define __IInkTablet2_INTERFACE_DEFINED__

/* interface IInkTablet2 */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkTablet2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90c91ad2-fa36-49d6-9516-ce8d570f6f85")
    IInkTablet2 : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DeviceKind( 
            /* [retval][out] */ __RPC__out TabletDeviceKind *Kind) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkTablet2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkTablet2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkTablet2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkTablet2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkTablet2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkTablet2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkTablet2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkTablet2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkTablet2, get_DeviceKind)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DeviceKind )( 
            __RPC__in IInkTablet2 * This,
            /* [retval][out] */ __RPC__out TabletDeviceKind *Kind);
        
        END_INTERFACE
    } IInkTablet2Vtbl;

    interface IInkTablet2
    {
        CONST_VTBL struct IInkTablet2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkTablet2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkTablet2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkTablet2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkTablet2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkTablet2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkTablet2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkTablet2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkTablet2_get_DeviceKind(This,Kind)	\
    ( (This)->lpVtbl -> get_DeviceKind(This,Kind) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkTablet2_INTERFACE_DEFINED__ */


#ifndef __IInkTablet3_INTERFACE_DEFINED__
#define __IInkTablet3_INTERFACE_DEFINED__

/* interface IInkTablet3 */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkTablet3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7e313997-1327-41dd-8ca9-79f24be17250")
    IInkTablet3 : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_IsMultiTouch( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pIsMultiTouch) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MaximumCursors( 
            /* [retval][out] */ __RPC__out ULONG *pMaximumCursors) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkTablet3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkTablet3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkTablet3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkTablet3 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkTablet3 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkTablet3 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkTablet3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkTablet3 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkTablet3, get_IsMultiTouch)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsMultiTouch )( 
            __RPC__in IInkTablet3 * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pIsMultiTouch);
        
        DECLSPEC_XFGVIRT(IInkTablet3, get_MaximumCursors)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MaximumCursors )( 
            __RPC__in IInkTablet3 * This,
            /* [retval][out] */ __RPC__out ULONG *pMaximumCursors);
        
        END_INTERFACE
    } IInkTablet3Vtbl;

    interface IInkTablet3
    {
        CONST_VTBL struct IInkTablet3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkTablet3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkTablet3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkTablet3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkTablet3_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkTablet3_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkTablet3_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkTablet3_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkTablet3_get_IsMultiTouch(This,pIsMultiTouch)	\
    ( (This)->lpVtbl -> get_IsMultiTouch(This,pIsMultiTouch) ) 

#define IInkTablet3_get_MaximumCursors(This,pMaximumCursors)	\
    ( (This)->lpVtbl -> get_MaximumCursors(This,pMaximumCursors) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkTablet3_INTERFACE_DEFINED__ */


#ifndef __IInkTablets_INTERFACE_DEFINED__
#define __IInkTablets_INTERFACE_DEFINED__

/* interface IInkTablets */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkTablets;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("112086D9-7779-4535-A699-862B43AC1863")
    IInkTablets : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultTablet( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **DefaultTablet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **Tablet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE IsPacketPropertySupported( 
            /* [in] */ __RPC__in BSTR packetPropertyName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Supported) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkTabletsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkTablets * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkTablets * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkTablets * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkTablets * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkTablets * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkTablets * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkTablets * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkTablets, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkTablets * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkTablets, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkTablets * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkTablets, get_DefaultTablet)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultTablet )( 
            __RPC__in IInkTablets * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **DefaultTablet);
        
        DECLSPEC_XFGVIRT(IInkTablets, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkTablets * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **Tablet);
        
        DECLSPEC_XFGVIRT(IInkTablets, IsPacketPropertySupported)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsPacketPropertySupported )( 
            __RPC__in IInkTablets * This,
            /* [in] */ __RPC__in BSTR packetPropertyName,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Supported);
        
        END_INTERFACE
    } IInkTabletsVtbl;

    interface IInkTablets
    {
        CONST_VTBL struct IInkTabletsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkTablets_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkTablets_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkTablets_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkTablets_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkTablets_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkTablets_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkTablets_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkTablets_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkTablets_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkTablets_get_DefaultTablet(This,DefaultTablet)	\
    ( (This)->lpVtbl -> get_DefaultTablet(This,DefaultTablet) ) 

#define IInkTablets_Item(This,Index,Tablet)	\
    ( (This)->lpVtbl -> Item(This,Index,Tablet) ) 

#define IInkTablets_IsPacketPropertySupported(This,packetPropertyName,Supported)	\
    ( (This)->lpVtbl -> IsPacketPropertySupported(This,packetPropertyName,Supported) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkTablets_INTERFACE_DEFINED__ */


#ifndef __IInkStrokeDisp_INTERFACE_DEFINED__
#define __IInkStrokeDisp_INTERFACE_DEFINED__

/* interface IInkStrokeDisp */
/* [unique][dual][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IInkStrokeDisp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("43242FEA-91D1-4a72-963E-FBB91829CFA2")
    IInkStrokeDisp : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ID( 
            /* [retval][out] */ __RPC__out long *ID) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BezierPoints( 
            /* [retval][out] */ __RPC__out VARIANT *Points) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **DrawAttrs) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_DrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *DrawAttrs) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ink( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperties **Properties) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PolylineCusps( 
            /* [retval][out] */ __RPC__out VARIANT *Cusps) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BezierCusps( 
            /* [retval][out] */ __RPC__out VARIANT *Cusps) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SelfIntersections( 
            /* [retval][out] */ __RPC__out VARIANT *Intersections) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PacketCount( 
            /* [retval][out] */ __RPC__out long *plCount) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PacketSize( 
            /* [retval][out] */ __RPC__out long *plSize) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PacketDescription( 
            /* [retval][out] */ __RPC__out VARIANT *PacketDescription) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Deleted( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Deleted) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBoundingBox( 
            /* [defaultvalue][in] */ InkBoundingBoxMode BoundingBoxMode,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE FindIntersections( 
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [retval][out] */ __RPC__out VARIANT *Intersections) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetRectangleIntersections( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle,
            /* [retval][out] */ __RPC__out VARIANT *Intersections) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clip( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE HitTestCircle( 
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [in] */ float Radius,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Intersects) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE NearestPoint( 
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [defaultvalue][out][in] */ __RPC__inout float *Distance,
            /* [retval][out] */ __RPC__out float *Point) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Split( 
            /* [in] */ float SplitAt,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **NewStroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPacketDescriptionPropertyMetrics( 
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [out] */ __RPC__out long *Minimum,
            /* [out] */ __RPC__out long *Maximum,
            /* [out] */ __RPC__out TabletPropertyMetricUnit *Units,
            /* [out] */ __RPC__out float *Resolution) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPoints( 
            /* [in][defaultvalue] */ long Index,
            /* [in][defaultvalue] */ long Count,
            /* [retval][out] */ __RPC__out VARIANT *Points) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPoints( 
            /* [in] */ VARIANT Points,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out long *NumberOfPointsSet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPacketData( 
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out VARIANT *PacketData) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPacketValuesByProperty( 
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out VARIANT *PacketValues) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetPacketValuesByProperty( 
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ VARIANT PacketValues,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out long *NumberOfPacketsSet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetFlattenedBezierPoints( 
            /* [defaultvalue][in] */ long FittingError,
            /* [retval][out] */ __RPC__out VARIANT *FlattenedBezierPoints) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Transform( 
            /* [in] */ __RPC__in_opt IInkTransform *Transform,
            /* [defaultvalue][in] */ VARIANT_BOOL ApplyOnPenWidth = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleToRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Rotate( 
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x = 0,
            /* [defaultvalue][in] */ float y = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Shear( 
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleTransform( 
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkStrokeDispVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkStrokeDisp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkStrokeDisp * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkStrokeDisp * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkStrokeDisp * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_ID)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ID )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out long *ID);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_BezierPoints)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BezierPoints )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out VARIANT *Points);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_DrawingAttributes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DrawingAttributes )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **DrawAttrs);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, putref_DrawingAttributes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_DrawingAttributes )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *DrawAttrs);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_Ink)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ink )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_ExtendedProperties)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedProperties )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperties **Properties);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_PolylineCusps)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PolylineCusps )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out VARIANT *Cusps);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_BezierCusps)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BezierCusps )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out VARIANT *Cusps);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_SelfIntersections)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SelfIntersections )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out VARIANT *Intersections);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_PacketCount)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PacketCount )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out long *plCount);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_PacketSize)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PacketSize )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out long *plSize);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_PacketDescription)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PacketDescription )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out VARIANT *PacketDescription);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, get_Deleted)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Deleted )( 
            __RPC__in IInkStrokeDisp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Deleted);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetBoundingBox)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBoundingBox )( 
            __RPC__in IInkStrokeDisp * This,
            /* [defaultvalue][in] */ InkBoundingBoxMode BoundingBoxMode,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, FindIntersections)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *FindIntersections )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [retval][out] */ __RPC__out VARIANT *Intersections);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetRectangleIntersections)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetRectangleIntersections )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle,
            /* [retval][out] */ __RPC__out VARIANT *Intersections);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, Clip)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clip )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, HitTestCircle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HitTestCircle )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [in] */ float Radius,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Intersects);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, NearestPoint)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NearestPoint )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [defaultvalue][out][in] */ __RPC__inout float *Distance,
            /* [retval][out] */ __RPC__out float *Point);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, Split)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Split )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ float SplitAt,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **NewStroke);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetPacketDescriptionPropertyMetrics)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPacketDescriptionPropertyMetrics )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [out] */ __RPC__out long *Minimum,
            /* [out] */ __RPC__out long *Maximum,
            /* [out] */ __RPC__out TabletPropertyMetricUnit *Units,
            /* [out] */ __RPC__out float *Resolution);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetPoints)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPoints )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in][defaultvalue] */ long Index,
            /* [in][defaultvalue] */ long Count,
            /* [retval][out] */ __RPC__out VARIANT *Points);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, SetPoints)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPoints )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ VARIANT Points,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out long *NumberOfPointsSet);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetPacketData)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPacketData )( 
            __RPC__in IInkStrokeDisp * This,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out VARIANT *PacketData);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetPacketValuesByProperty)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPacketValuesByProperty )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in BSTR PropertyName,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out VARIANT *PacketValues);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, SetPacketValuesByProperty)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetPacketValuesByProperty )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in BSTR bstrPropertyName,
            /* [in] */ VARIANT PacketValues,
            /* [defaultvalue][in] */ long Index,
            /* [defaultvalue][in] */ long Count,
            /* [retval][out] */ __RPC__out long *NumberOfPacketsSet);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, GetFlattenedBezierPoints)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetFlattenedBezierPoints )( 
            __RPC__in IInkStrokeDisp * This,
            /* [defaultvalue][in] */ long FittingError,
            /* [retval][out] */ __RPC__out VARIANT *FlattenedBezierPoints);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, Transform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Transform )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in_opt IInkTransform *Transform,
            /* [defaultvalue][in] */ VARIANT_BOOL ApplyOnPenWidth);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, ScaleToRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleToRectangle )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, Move)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, Rotate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x,
            /* [defaultvalue][in] */ float y);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, Shear)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Shear )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier);
        
        DECLSPEC_XFGVIRT(IInkStrokeDisp, ScaleTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleTransform )( 
            __RPC__in IInkStrokeDisp * This,
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier);
        
        END_INTERFACE
    } IInkStrokeDispVtbl;

    interface IInkStrokeDisp
    {
        CONST_VTBL struct IInkStrokeDispVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkStrokeDisp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkStrokeDisp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkStrokeDisp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkStrokeDisp_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkStrokeDisp_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkStrokeDisp_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkStrokeDisp_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkStrokeDisp_get_ID(This,ID)	\
    ( (This)->lpVtbl -> get_ID(This,ID) ) 

#define IInkStrokeDisp_get_BezierPoints(This,Points)	\
    ( (This)->lpVtbl -> get_BezierPoints(This,Points) ) 

#define IInkStrokeDisp_get_DrawingAttributes(This,DrawAttrs)	\
    ( (This)->lpVtbl -> get_DrawingAttributes(This,DrawAttrs) ) 

#define IInkStrokeDisp_putref_DrawingAttributes(This,DrawAttrs)	\
    ( (This)->lpVtbl -> putref_DrawingAttributes(This,DrawAttrs) ) 

#define IInkStrokeDisp_get_Ink(This,Ink)	\
    ( (This)->lpVtbl -> get_Ink(This,Ink) ) 

#define IInkStrokeDisp_get_ExtendedProperties(This,Properties)	\
    ( (This)->lpVtbl -> get_ExtendedProperties(This,Properties) ) 

#define IInkStrokeDisp_get_PolylineCusps(This,Cusps)	\
    ( (This)->lpVtbl -> get_PolylineCusps(This,Cusps) ) 

#define IInkStrokeDisp_get_BezierCusps(This,Cusps)	\
    ( (This)->lpVtbl -> get_BezierCusps(This,Cusps) ) 

#define IInkStrokeDisp_get_SelfIntersections(This,Intersections)	\
    ( (This)->lpVtbl -> get_SelfIntersections(This,Intersections) ) 

#define IInkStrokeDisp_get_PacketCount(This,plCount)	\
    ( (This)->lpVtbl -> get_PacketCount(This,plCount) ) 

#define IInkStrokeDisp_get_PacketSize(This,plSize)	\
    ( (This)->lpVtbl -> get_PacketSize(This,plSize) ) 

#define IInkStrokeDisp_get_PacketDescription(This,PacketDescription)	\
    ( (This)->lpVtbl -> get_PacketDescription(This,PacketDescription) ) 

#define IInkStrokeDisp_get_Deleted(This,Deleted)	\
    ( (This)->lpVtbl -> get_Deleted(This,Deleted) ) 

#define IInkStrokeDisp_GetBoundingBox(This,BoundingBoxMode,Rectangle)	\
    ( (This)->lpVtbl -> GetBoundingBox(This,BoundingBoxMode,Rectangle) ) 

#define IInkStrokeDisp_FindIntersections(This,Strokes,Intersections)	\
    ( (This)->lpVtbl -> FindIntersections(This,Strokes,Intersections) ) 

#define IInkStrokeDisp_GetRectangleIntersections(This,Rectangle,Intersections)	\
    ( (This)->lpVtbl -> GetRectangleIntersections(This,Rectangle,Intersections) ) 

#define IInkStrokeDisp_Clip(This,Rectangle)	\
    ( (This)->lpVtbl -> Clip(This,Rectangle) ) 

#define IInkStrokeDisp_HitTestCircle(This,X,Y,Radius,Intersects)	\
    ( (This)->lpVtbl -> HitTestCircle(This,X,Y,Radius,Intersects) ) 

#define IInkStrokeDisp_NearestPoint(This,X,Y,Distance,Point)	\
    ( (This)->lpVtbl -> NearestPoint(This,X,Y,Distance,Point) ) 

#define IInkStrokeDisp_Split(This,SplitAt,NewStroke)	\
    ( (This)->lpVtbl -> Split(This,SplitAt,NewStroke) ) 

#define IInkStrokeDisp_GetPacketDescriptionPropertyMetrics(This,PropertyName,Minimum,Maximum,Units,Resolution)	\
    ( (This)->lpVtbl -> GetPacketDescriptionPropertyMetrics(This,PropertyName,Minimum,Maximum,Units,Resolution) ) 

#define IInkStrokeDisp_GetPoints(This,Index,Count,Points)	\
    ( (This)->lpVtbl -> GetPoints(This,Index,Count,Points) ) 

#define IInkStrokeDisp_SetPoints(This,Points,Index,Count,NumberOfPointsSet)	\
    ( (This)->lpVtbl -> SetPoints(This,Points,Index,Count,NumberOfPointsSet) ) 

#define IInkStrokeDisp_GetPacketData(This,Index,Count,PacketData)	\
    ( (This)->lpVtbl -> GetPacketData(This,Index,Count,PacketData) ) 

#define IInkStrokeDisp_GetPacketValuesByProperty(This,PropertyName,Index,Count,PacketValues)	\
    ( (This)->lpVtbl -> GetPacketValuesByProperty(This,PropertyName,Index,Count,PacketValues) ) 

#define IInkStrokeDisp_SetPacketValuesByProperty(This,bstrPropertyName,PacketValues,Index,Count,NumberOfPacketsSet)	\
    ( (This)->lpVtbl -> SetPacketValuesByProperty(This,bstrPropertyName,PacketValues,Index,Count,NumberOfPacketsSet) ) 

#define IInkStrokeDisp_GetFlattenedBezierPoints(This,FittingError,FlattenedBezierPoints)	\
    ( (This)->lpVtbl -> GetFlattenedBezierPoints(This,FittingError,FlattenedBezierPoints) ) 

#define IInkStrokeDisp_Transform(This,Transform,ApplyOnPenWidth)	\
    ( (This)->lpVtbl -> Transform(This,Transform,ApplyOnPenWidth) ) 

#define IInkStrokeDisp_ScaleToRectangle(This,Rectangle)	\
    ( (This)->lpVtbl -> ScaleToRectangle(This,Rectangle) ) 

#define IInkStrokeDisp_Move(This,HorizontalComponent,VerticalComponent)	\
    ( (This)->lpVtbl -> Move(This,HorizontalComponent,VerticalComponent) ) 

#define IInkStrokeDisp_Rotate(This,Degrees,x,y)	\
    ( (This)->lpVtbl -> Rotate(This,Degrees,x,y) ) 

#define IInkStrokeDisp_Shear(This,HorizontalMultiplier,VerticalMultiplier)	\
    ( (This)->lpVtbl -> Shear(This,HorizontalMultiplier,VerticalMultiplier) ) 

#define IInkStrokeDisp_ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier)	\
    ( (This)->lpVtbl -> ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkStrokeDisp_INTERFACE_DEFINED__ */


#ifndef __IInkStrokes_INTERFACE_DEFINED__
#define __IInkStrokes_INTERFACE_DEFINED__

/* interface IInkStrokes */
/* [unique][dual][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IInkStrokes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F1F4C9D8-590A-4963-B3AE-1935671BB6F3")
    IInkStrokes : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ink( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecognitionResult( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionResult **RecognitionResult) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ToString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ToString) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **Stroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IInkStrokeDisp *InkStroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddStrokes( 
            /* [in] */ __RPC__in_opt IInkStrokes *InkStrokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ __RPC__in_opt IInkStrokeDisp *InkStroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveStrokes( 
            /* [in] */ __RPC__in_opt IInkStrokes *InkStrokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ModifyDrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *DrawAttrs) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBoundingBox( 
            /* [defaultvalue][in] */ InkBoundingBoxMode BoundingBoxMode,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **BoundingBox) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Transform( 
            /* [in] */ __RPC__in_opt IInkTransform *Transform,
            /* [defaultvalue][in] */ VARIANT_BOOL ApplyOnPenWidth = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleToRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Rotate( 
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x = 0,
            /* [defaultvalue][in] */ float y = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Shear( 
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleTransform( 
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clip( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveRecognitionResult( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkStrokesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkStrokes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkStrokes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkStrokes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkStrokes * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkStrokes, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkStrokes * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkStrokes, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkStrokes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkStrokes, get_Ink)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ink )( 
            __RPC__in IInkStrokes * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink);
        
        DECLSPEC_XFGVIRT(IInkStrokes, get_RecognitionResult)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecognitionResult )( 
            __RPC__in IInkStrokes * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionResult **RecognitionResult);
        
        DECLSPEC_XFGVIRT(IInkStrokes, ToString)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ToString )( 
            __RPC__in IInkStrokes * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *ToString);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **Stroke);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Add)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkStrokeDisp *InkStroke);
        
        DECLSPEC_XFGVIRT(IInkStrokes, AddStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddStrokes )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkStrokes *InkStrokes);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Remove)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkStrokeDisp *InkStroke);
        
        DECLSPEC_XFGVIRT(IInkStrokes, RemoveStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveStrokes )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkStrokes *InkStrokes);
        
        DECLSPEC_XFGVIRT(IInkStrokes, ModifyDrawingAttributes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyDrawingAttributes )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *DrawAttrs);
        
        DECLSPEC_XFGVIRT(IInkStrokes, GetBoundingBox)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBoundingBox )( 
            __RPC__in IInkStrokes * This,
            /* [defaultvalue][in] */ InkBoundingBoxMode BoundingBoxMode,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **BoundingBox);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Transform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Transform )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkTransform *Transform,
            /* [defaultvalue][in] */ VARIANT_BOOL ApplyOnPenWidth);
        
        DECLSPEC_XFGVIRT(IInkStrokes, ScaleToRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleToRectangle )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Move)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Rotate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x,
            /* [defaultvalue][in] */ float y);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Shear)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Shear )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier);
        
        DECLSPEC_XFGVIRT(IInkStrokes, ScaleTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleTransform )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier);
        
        DECLSPEC_XFGVIRT(IInkStrokes, Clip)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clip )( 
            __RPC__in IInkStrokes * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkStrokes, RemoveRecognitionResult)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveRecognitionResult )( 
            __RPC__in IInkStrokes * This);
        
        END_INTERFACE
    } IInkStrokesVtbl;

    interface IInkStrokes
    {
        CONST_VTBL struct IInkStrokesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkStrokes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkStrokes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkStrokes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkStrokes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkStrokes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkStrokes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkStrokes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkStrokes_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkStrokes_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkStrokes_get_Ink(This,Ink)	\
    ( (This)->lpVtbl -> get_Ink(This,Ink) ) 

#define IInkStrokes_get_RecognitionResult(This,RecognitionResult)	\
    ( (This)->lpVtbl -> get_RecognitionResult(This,RecognitionResult) ) 

#define IInkStrokes_ToString(This,ToString)	\
    ( (This)->lpVtbl -> ToString(This,ToString) ) 

#define IInkStrokes_Item(This,Index,Stroke)	\
    ( (This)->lpVtbl -> Item(This,Index,Stroke) ) 

#define IInkStrokes_Add(This,InkStroke)	\
    ( (This)->lpVtbl -> Add(This,InkStroke) ) 

#define IInkStrokes_AddStrokes(This,InkStrokes)	\
    ( (This)->lpVtbl -> AddStrokes(This,InkStrokes) ) 

#define IInkStrokes_Remove(This,InkStroke)	\
    ( (This)->lpVtbl -> Remove(This,InkStroke) ) 

#define IInkStrokes_RemoveStrokes(This,InkStrokes)	\
    ( (This)->lpVtbl -> RemoveStrokes(This,InkStrokes) ) 

#define IInkStrokes_ModifyDrawingAttributes(This,DrawAttrs)	\
    ( (This)->lpVtbl -> ModifyDrawingAttributes(This,DrawAttrs) ) 

#define IInkStrokes_GetBoundingBox(This,BoundingBoxMode,BoundingBox)	\
    ( (This)->lpVtbl -> GetBoundingBox(This,BoundingBoxMode,BoundingBox) ) 

#define IInkStrokes_Transform(This,Transform,ApplyOnPenWidth)	\
    ( (This)->lpVtbl -> Transform(This,Transform,ApplyOnPenWidth) ) 

#define IInkStrokes_ScaleToRectangle(This,Rectangle)	\
    ( (This)->lpVtbl -> ScaleToRectangle(This,Rectangle) ) 

#define IInkStrokes_Move(This,HorizontalComponent,VerticalComponent)	\
    ( (This)->lpVtbl -> Move(This,HorizontalComponent,VerticalComponent) ) 

#define IInkStrokes_Rotate(This,Degrees,x,y)	\
    ( (This)->lpVtbl -> Rotate(This,Degrees,x,y) ) 

#define IInkStrokes_Shear(This,HorizontalMultiplier,VerticalMultiplier)	\
    ( (This)->lpVtbl -> Shear(This,HorizontalMultiplier,VerticalMultiplier) ) 

#define IInkStrokes_ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier)	\
    ( (This)->lpVtbl -> ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier) ) 

#define IInkStrokes_Clip(This,Rectangle)	\
    ( (This)->lpVtbl -> Clip(This,Rectangle) ) 

#define IInkStrokes_RemoveRecognitionResult(This)	\
    ( (This)->lpVtbl -> RemoveRecognitionResult(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkStrokes_INTERFACE_DEFINED__ */


#ifndef __IInkCustomStrokes_INTERFACE_DEFINED__
#define __IInkCustomStrokes_INTERFACE_DEFINED__

/* interface IInkCustomStrokes */
/* [unique][dual][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IInkCustomStrokes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7E23A88F-C30E-420f-9BDB-28902543F0C1")
    IInkCustomStrokes : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ VARIANT Identifier,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Remove( 
            /* [in] */ VARIANT Identifier) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCustomStrokesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCustomStrokes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCustomStrokes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCustomStrokes * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkCustomStrokes * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkCustomStrokes * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkCustomStrokes * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkCustomStrokes * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkCustomStrokes, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkCustomStrokes * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkCustomStrokes, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkCustomStrokes * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkCustomStrokes, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkCustomStrokes * This,
            /* [in] */ VARIANT Identifier,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkCustomStrokes, Add)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IInkCustomStrokes * This,
            /* [in] */ __RPC__in BSTR Name,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes);
        
        DECLSPEC_XFGVIRT(IInkCustomStrokes, Remove)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Remove )( 
            __RPC__in IInkCustomStrokes * This,
            /* [in] */ VARIANT Identifier);
        
        DECLSPEC_XFGVIRT(IInkCustomStrokes, Clear)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            __RPC__in IInkCustomStrokes * This);
        
        END_INTERFACE
    } IInkCustomStrokesVtbl;

    interface IInkCustomStrokes
    {
        CONST_VTBL struct IInkCustomStrokesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCustomStrokes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCustomStrokes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCustomStrokes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCustomStrokes_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkCustomStrokes_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkCustomStrokes_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkCustomStrokes_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkCustomStrokes_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkCustomStrokes_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkCustomStrokes_Item(This,Identifier,Strokes)	\
    ( (This)->lpVtbl -> Item(This,Identifier,Strokes) ) 

#define IInkCustomStrokes_Add(This,Name,Strokes)	\
    ( (This)->lpVtbl -> Add(This,Name,Strokes) ) 

#define IInkCustomStrokes_Remove(This,Identifier)	\
    ( (This)->lpVtbl -> Remove(This,Identifier) ) 

#define IInkCustomStrokes_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCustomStrokes_INTERFACE_DEFINED__ */


#ifndef ___IInkStrokesEvents_DISPINTERFACE_DEFINED__
#define ___IInkStrokesEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkStrokesEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkStrokesEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("F33053EC-5D25-430a-928F-76A6491DDE15")
    _IInkStrokesEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkStrokesEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkStrokesEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkStrokesEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkStrokesEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkStrokesEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkStrokesEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkStrokesEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkStrokesEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkStrokesEventsVtbl;

    interface _IInkStrokesEvents
    {
        CONST_VTBL struct _IInkStrokesEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkStrokesEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkStrokesEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkStrokesEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkStrokesEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkStrokesEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkStrokesEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkStrokesEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkStrokesEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IInkDisp_INTERFACE_DEFINED__
#define __IInkDisp_INTERFACE_DEFINED__

/* interface IInkDisp */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkDisp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9D398FA0-C4E2-4fcd-9973-975CAAF47EA6")
    IInkDisp : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedProperties( 
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperties **Properties) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Dirty( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Dirty) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Dirty( 
            /* [in] */ VARIANT_BOOL Dirty) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CustomStrokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkCustomStrokes **ppunkInkCustomStrokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetBoundingBox( 
            /* [defaultvalue][in] */ InkBoundingBoxMode BoundingBoxMode,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteStrokes( 
            /* [unique][defaultvalue][in] */ __RPC__in_opt IInkStrokes *Strokes = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE DeleteStroke( 
            /* [in] */ __RPC__in_opt IInkStrokeDisp *Stroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractStrokes( 
            /* [defaultvalue][in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [defaultvalue][in] */ InkExtractFlags ExtractFlags,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **ExtractedInk) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ExtractWithRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle,
            /* [defaultvalue][in] */ InkExtractFlags extractFlags,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **ExtractedInk) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clip( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **NewInk) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE HitTestCircle( 
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [in] */ float radius,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE HitTestWithRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *SelectionRectangle,
            /* [in] */ float IntersectPercent,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE HitTestWithLasso( 
            /* [in] */ VARIANT Points,
            /* [in] */ float IntersectPercent,
            /* [defaultvalue][unique][out][in] */ __RPC__inout_opt VARIANT *LassoPoints,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE NearestPoint( 
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [defaultvalue][out][in] */ __RPC__inout float *PointOnStroke,
            /* [defaultvalue][out][in] */ __RPC__inout float *DistanceFromPacket,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **Stroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateStrokes( 
            /* [defaultvalue][in] */ VARIANT StrokeIds,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddStrokesAtRectangle( 
            /* [in] */ __RPC__in_opt IInkStrokes *SourceStrokes,
            /* [in] */ __RPC__in_opt IInkRectangle *TargetRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Save( 
            /* [defaultvalue][in] */ InkPersistenceFormat PersistenceFormat,
            /* [defaultvalue][in] */ InkPersistenceCompressionMode CompressionMode,
            /* [retval][out] */ __RPC__out VARIANT *Data) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ VARIANT Data) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateStroke( 
            /* [in] */ VARIANT PacketData,
            /* [in] */ VARIANT PacketDescription,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **Stroke) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ClipboardCopyWithRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle,
            /* [defaultvalue][in] */ InkClipboardFormats ClipboardFormats,
            /* [defaultvalue][in] */ InkClipboardModes ClipboardModes,
            /* [retval][out] */ __RPC__deref_out_opt IDataObject **DataObject) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ClipboardCopy( 
            /* [defaultvalue][in] */ __RPC__in_opt IInkStrokes *strokes,
            /* [defaultvalue][in] */ InkClipboardFormats ClipboardFormats,
            /* [defaultvalue][in] */ InkClipboardModes ClipboardModes,
            /* [retval][out] */ __RPC__deref_out_opt IDataObject **DataObject) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE CanPaste( 
            /* [defaultvalue][in] */ __RPC__in_opt IDataObject *DataObject,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *CanPaste) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ClipboardPaste( 
            /* [defaultvalue][in] */ long x,
            /* [defaultvalue][in] */ long y,
            /* [defaultvalue][unique][in] */ __RPC__in_opt IDataObject *DataObject,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkDispVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkDisp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkDisp * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkDisp * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkDisp * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkDisp * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkDisp, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, get_ExtendedProperties)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedProperties )( 
            __RPC__in IInkDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkExtendedProperties **Properties);
        
        DECLSPEC_XFGVIRT(IInkDisp, get_Dirty)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Dirty )( 
            __RPC__in IInkDisp * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Dirty);
        
        DECLSPEC_XFGVIRT(IInkDisp, put_Dirty)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Dirty )( 
            __RPC__in IInkDisp * This,
            /* [in] */ VARIANT_BOOL Dirty);
        
        DECLSPEC_XFGVIRT(IInkDisp, get_CustomStrokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CustomStrokes )( 
            __RPC__in IInkDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkCustomStrokes **ppunkInkCustomStrokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, GetBoundingBox)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetBoundingBox )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ InkBoundingBoxMode BoundingBoxMode,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkDisp, DeleteStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteStrokes )( 
            __RPC__in IInkDisp * This,
            /* [unique][defaultvalue][in] */ __RPC__in_opt IInkStrokes *Strokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, DeleteStroke)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DeleteStroke )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in_opt IInkStrokeDisp *Stroke);
        
        DECLSPEC_XFGVIRT(IInkDisp, ExtractStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExtractStrokes )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [defaultvalue][in] */ InkExtractFlags ExtractFlags,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **ExtractedInk);
        
        DECLSPEC_XFGVIRT(IInkDisp, ExtractWithRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ExtractWithRectangle )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle,
            /* [defaultvalue][in] */ InkExtractFlags extractFlags,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **ExtractedInk);
        
        DECLSPEC_XFGVIRT(IInkDisp, Clip)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clip )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkDisp, Clone)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IInkDisp * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **NewInk);
        
        DECLSPEC_XFGVIRT(IInkDisp, HitTestCircle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HitTestCircle )( 
            __RPC__in IInkDisp * This,
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [in] */ float radius,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, HitTestWithRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HitTestWithRectangle )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *SelectionRectangle,
            /* [in] */ float IntersectPercent,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, HitTestWithLasso)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HitTestWithLasso )( 
            __RPC__in IInkDisp * This,
            /* [in] */ VARIANT Points,
            /* [in] */ float IntersectPercent,
            /* [defaultvalue][unique][out][in] */ __RPC__inout_opt VARIANT *LassoPoints,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, NearestPoint)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *NearestPoint )( 
            __RPC__in IInkDisp * This,
            /* [in] */ long X,
            /* [in] */ long Y,
            /* [defaultvalue][out][in] */ __RPC__inout float *PointOnStroke,
            /* [defaultvalue][out][in] */ __RPC__inout float *DistanceFromPacket,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **Stroke);
        
        DECLSPEC_XFGVIRT(IInkDisp, CreateStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateStrokes )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ VARIANT StrokeIds,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkDisp, AddStrokesAtRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddStrokesAtRectangle )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in_opt IInkStrokes *SourceStrokes,
            /* [in] */ __RPC__in_opt IInkRectangle *TargetRectangle);
        
        DECLSPEC_XFGVIRT(IInkDisp, Save)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Save )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ InkPersistenceFormat PersistenceFormat,
            /* [defaultvalue][in] */ InkPersistenceCompressionMode CompressionMode,
            /* [retval][out] */ __RPC__out VARIANT *Data);
        
        DECLSPEC_XFGVIRT(IInkDisp, Load)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Load )( 
            __RPC__in IInkDisp * This,
            /* [in] */ VARIANT Data);
        
        DECLSPEC_XFGVIRT(IInkDisp, CreateStroke)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateStroke )( 
            __RPC__in IInkDisp * This,
            /* [in] */ VARIANT PacketData,
            /* [in] */ VARIANT PacketDescription,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokeDisp **Stroke);
        
        DECLSPEC_XFGVIRT(IInkDisp, ClipboardCopyWithRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClipboardCopyWithRectangle )( 
            __RPC__in IInkDisp * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle,
            /* [defaultvalue][in] */ InkClipboardFormats ClipboardFormats,
            /* [defaultvalue][in] */ InkClipboardModes ClipboardModes,
            /* [retval][out] */ __RPC__deref_out_opt IDataObject **DataObject);
        
        DECLSPEC_XFGVIRT(IInkDisp, ClipboardCopy)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClipboardCopy )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ __RPC__in_opt IInkStrokes *strokes,
            /* [defaultvalue][in] */ InkClipboardFormats ClipboardFormats,
            /* [defaultvalue][in] */ InkClipboardModes ClipboardModes,
            /* [retval][out] */ __RPC__deref_out_opt IDataObject **DataObject);
        
        DECLSPEC_XFGVIRT(IInkDisp, CanPaste)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CanPaste )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ __RPC__in_opt IDataObject *DataObject,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *CanPaste);
        
        DECLSPEC_XFGVIRT(IInkDisp, ClipboardPaste)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ClipboardPaste )( 
            __RPC__in IInkDisp * This,
            /* [defaultvalue][in] */ long x,
            /* [defaultvalue][in] */ long y,
            /* [defaultvalue][unique][in] */ __RPC__in_opt IDataObject *DataObject,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        END_INTERFACE
    } IInkDispVtbl;

    interface IInkDisp
    {
        CONST_VTBL struct IInkDispVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkDisp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkDisp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkDisp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkDisp_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkDisp_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkDisp_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkDisp_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkDisp_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkDisp_get_ExtendedProperties(This,Properties)	\
    ( (This)->lpVtbl -> get_ExtendedProperties(This,Properties) ) 

#define IInkDisp_get_Dirty(This,Dirty)	\
    ( (This)->lpVtbl -> get_Dirty(This,Dirty) ) 

#define IInkDisp_put_Dirty(This,Dirty)	\
    ( (This)->lpVtbl -> put_Dirty(This,Dirty) ) 

#define IInkDisp_get_CustomStrokes(This,ppunkInkCustomStrokes)	\
    ( (This)->lpVtbl -> get_CustomStrokes(This,ppunkInkCustomStrokes) ) 

#define IInkDisp_GetBoundingBox(This,BoundingBoxMode,Rectangle)	\
    ( (This)->lpVtbl -> GetBoundingBox(This,BoundingBoxMode,Rectangle) ) 

#define IInkDisp_DeleteStrokes(This,Strokes)	\
    ( (This)->lpVtbl -> DeleteStrokes(This,Strokes) ) 

#define IInkDisp_DeleteStroke(This,Stroke)	\
    ( (This)->lpVtbl -> DeleteStroke(This,Stroke) ) 

#define IInkDisp_ExtractStrokes(This,Strokes,ExtractFlags,ExtractedInk)	\
    ( (This)->lpVtbl -> ExtractStrokes(This,Strokes,ExtractFlags,ExtractedInk) ) 

#define IInkDisp_ExtractWithRectangle(This,Rectangle,extractFlags,ExtractedInk)	\
    ( (This)->lpVtbl -> ExtractWithRectangle(This,Rectangle,extractFlags,ExtractedInk) ) 

#define IInkDisp_Clip(This,Rectangle)	\
    ( (This)->lpVtbl -> Clip(This,Rectangle) ) 

#define IInkDisp_Clone(This,NewInk)	\
    ( (This)->lpVtbl -> Clone(This,NewInk) ) 

#define IInkDisp_HitTestCircle(This,X,Y,radius,Strokes)	\
    ( (This)->lpVtbl -> HitTestCircle(This,X,Y,radius,Strokes) ) 

#define IInkDisp_HitTestWithRectangle(This,SelectionRectangle,IntersectPercent,Strokes)	\
    ( (This)->lpVtbl -> HitTestWithRectangle(This,SelectionRectangle,IntersectPercent,Strokes) ) 

#define IInkDisp_HitTestWithLasso(This,Points,IntersectPercent,LassoPoints,Strokes)	\
    ( (This)->lpVtbl -> HitTestWithLasso(This,Points,IntersectPercent,LassoPoints,Strokes) ) 

#define IInkDisp_NearestPoint(This,X,Y,PointOnStroke,DistanceFromPacket,Stroke)	\
    ( (This)->lpVtbl -> NearestPoint(This,X,Y,PointOnStroke,DistanceFromPacket,Stroke) ) 

#define IInkDisp_CreateStrokes(This,StrokeIds,Strokes)	\
    ( (This)->lpVtbl -> CreateStrokes(This,StrokeIds,Strokes) ) 

#define IInkDisp_AddStrokesAtRectangle(This,SourceStrokes,TargetRectangle)	\
    ( (This)->lpVtbl -> AddStrokesAtRectangle(This,SourceStrokes,TargetRectangle) ) 

#define IInkDisp_Save(This,PersistenceFormat,CompressionMode,Data)	\
    ( (This)->lpVtbl -> Save(This,PersistenceFormat,CompressionMode,Data) ) 

#define IInkDisp_Load(This,Data)	\
    ( (This)->lpVtbl -> Load(This,Data) ) 

#define IInkDisp_CreateStroke(This,PacketData,PacketDescription,Stroke)	\
    ( (This)->lpVtbl -> CreateStroke(This,PacketData,PacketDescription,Stroke) ) 

#define IInkDisp_ClipboardCopyWithRectangle(This,Rectangle,ClipboardFormats,ClipboardModes,DataObject)	\
    ( (This)->lpVtbl -> ClipboardCopyWithRectangle(This,Rectangle,ClipboardFormats,ClipboardModes,DataObject) ) 

#define IInkDisp_ClipboardCopy(This,strokes,ClipboardFormats,ClipboardModes,DataObject)	\
    ( (This)->lpVtbl -> ClipboardCopy(This,strokes,ClipboardFormats,ClipboardModes,DataObject) ) 

#define IInkDisp_CanPaste(This,DataObject,CanPaste)	\
    ( (This)->lpVtbl -> CanPaste(This,DataObject,CanPaste) ) 

#define IInkDisp_ClipboardPaste(This,x,y,DataObject,Strokes)	\
    ( (This)->lpVtbl -> ClipboardPaste(This,x,y,DataObject,Strokes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkDisp_INTERFACE_DEFINED__ */


#ifndef ___IInkEvents_DISPINTERFACE_DEFINED__
#define ___IInkEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("427B1865-CA3F-479a-83A9-0F420F2A0073")
    _IInkEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkEventsVtbl;

    interface _IInkEvents
    {
        CONST_VTBL struct _IInkEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IInkRenderer_INTERFACE_DEFINED__
#define __IInkRenderer_INTERFACE_DEFINED__

/* interface IInkRenderer */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRenderer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E6257A9C-B511-4f4c-A8B0-A7DBC9506B83")
    IInkRenderer : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetViewTransform( 
            /* [in] */ __RPC__in_opt IInkTransform *ViewTransform) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetViewTransform( 
            /* [in] */ __RPC__in_opt IInkTransform *ViewTransform) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetObjectTransform( 
            /* [in] */ __RPC__in_opt IInkTransform *ObjectTransform) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetObjectTransform( 
            /* [in] */ __RPC__in_opt IInkTransform *ObjectTransform) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Draw( 
            /* [in] */ LONG_PTR hDC,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE DrawStroke( 
            /* [in] */ LONG_PTR hDC,
            /* [in] */ __RPC__in_opt IInkStrokeDisp *Stroke,
            /* [defaultvalue][in] */ __RPC__in_opt IInkDrawingAttributes *DrawingAttributes = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE PixelToInkSpace( 
            /* [in] */ LONG_PTR hDC,
            /* [out][in] */ __RPC__inout long *x,
            /* [out][in] */ __RPC__inout long *y) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE InkSpaceToPixel( 
            /* [in] */ LONG_PTR hdcDisplay,
            /* [out][in] */ __RPC__inout long *x,
            /* [out][in] */ __RPC__inout long *y) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE PixelToInkSpaceFromPoints( 
            /* [in] */ LONG_PTR hDC,
            /* [out][in] */ __RPC__inout VARIANT *Points) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE InkSpaceToPixelFromPoints( 
            /* [in] */ LONG_PTR hDC,
            /* [out][in] */ __RPC__inout VARIANT *Points) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Measure( 
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE MeasureStroke( 
            /* [in] */ __RPC__in_opt IInkStrokeDisp *Stroke,
            /* [defaultvalue][in] */ __RPC__in_opt IInkDrawingAttributes *DrawingAttributes,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Rotate( 
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x = 0,
            /* [defaultvalue][in] */ float y = 0) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ScaleTransform( 
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier,
            /* [defaultvalue][in] */ VARIANT_BOOL ApplyOnPenWidth = -1) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRendererVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRenderer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRenderer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRenderer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRenderer * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRenderer, GetViewTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetViewTransform )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in_opt IInkTransform *ViewTransform);
        
        DECLSPEC_XFGVIRT(IInkRenderer, SetViewTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetViewTransform )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in_opt IInkTransform *ViewTransform);
        
        DECLSPEC_XFGVIRT(IInkRenderer, GetObjectTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetObjectTransform )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in_opt IInkTransform *ObjectTransform);
        
        DECLSPEC_XFGVIRT(IInkRenderer, SetObjectTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetObjectTransform )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in_opt IInkTransform *ObjectTransform);
        
        DECLSPEC_XFGVIRT(IInkRenderer, Draw)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Draw )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ LONG_PTR hDC,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes);
        
        DECLSPEC_XFGVIRT(IInkRenderer, DrawStroke)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *DrawStroke )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ LONG_PTR hDC,
            /* [in] */ __RPC__in_opt IInkStrokeDisp *Stroke,
            /* [defaultvalue][in] */ __RPC__in_opt IInkDrawingAttributes *DrawingAttributes);
        
        DECLSPEC_XFGVIRT(IInkRenderer, PixelToInkSpace)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PixelToInkSpace )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ LONG_PTR hDC,
            /* [out][in] */ __RPC__inout long *x,
            /* [out][in] */ __RPC__inout long *y);
        
        DECLSPEC_XFGVIRT(IInkRenderer, InkSpaceToPixel)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InkSpaceToPixel )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ LONG_PTR hdcDisplay,
            /* [out][in] */ __RPC__inout long *x,
            /* [out][in] */ __RPC__inout long *y);
        
        DECLSPEC_XFGVIRT(IInkRenderer, PixelToInkSpaceFromPoints)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *PixelToInkSpaceFromPoints )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ LONG_PTR hDC,
            /* [out][in] */ __RPC__inout VARIANT *Points);
        
        DECLSPEC_XFGVIRT(IInkRenderer, InkSpaceToPixelFromPoints)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *InkSpaceToPixelFromPoints )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ LONG_PTR hDC,
            /* [out][in] */ __RPC__inout VARIANT *Points);
        
        DECLSPEC_XFGVIRT(IInkRenderer, Measure)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Measure )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkRenderer, MeasureStroke)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *MeasureStroke )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ __RPC__in_opt IInkStrokeDisp *Stroke,
            /* [defaultvalue][in] */ __RPC__in_opt IInkDrawingAttributes *DrawingAttributes,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkRenderer, Move)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ float HorizontalComponent,
            /* [in] */ float VerticalComponent);
        
        DECLSPEC_XFGVIRT(IInkRenderer, Rotate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ float Degrees,
            /* [defaultvalue][in] */ float x,
            /* [defaultvalue][in] */ float y);
        
        DECLSPEC_XFGVIRT(IInkRenderer, ScaleTransform)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ScaleTransform )( 
            __RPC__in IInkRenderer * This,
            /* [in] */ float HorizontalMultiplier,
            /* [in] */ float VerticalMultiplier,
            /* [defaultvalue][in] */ VARIANT_BOOL ApplyOnPenWidth);
        
        END_INTERFACE
    } IInkRendererVtbl;

    interface IInkRenderer
    {
        CONST_VTBL struct IInkRendererVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRenderer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRenderer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRenderer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRenderer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRenderer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRenderer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRenderer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRenderer_GetViewTransform(This,ViewTransform)	\
    ( (This)->lpVtbl -> GetViewTransform(This,ViewTransform) ) 

#define IInkRenderer_SetViewTransform(This,ViewTransform)	\
    ( (This)->lpVtbl -> SetViewTransform(This,ViewTransform) ) 

#define IInkRenderer_GetObjectTransform(This,ObjectTransform)	\
    ( (This)->lpVtbl -> GetObjectTransform(This,ObjectTransform) ) 

#define IInkRenderer_SetObjectTransform(This,ObjectTransform)	\
    ( (This)->lpVtbl -> SetObjectTransform(This,ObjectTransform) ) 

#define IInkRenderer_Draw(This,hDC,Strokes)	\
    ( (This)->lpVtbl -> Draw(This,hDC,Strokes) ) 

#define IInkRenderer_DrawStroke(This,hDC,Stroke,DrawingAttributes)	\
    ( (This)->lpVtbl -> DrawStroke(This,hDC,Stroke,DrawingAttributes) ) 

#define IInkRenderer_PixelToInkSpace(This,hDC,x,y)	\
    ( (This)->lpVtbl -> PixelToInkSpace(This,hDC,x,y) ) 

#define IInkRenderer_InkSpaceToPixel(This,hdcDisplay,x,y)	\
    ( (This)->lpVtbl -> InkSpaceToPixel(This,hdcDisplay,x,y) ) 

#define IInkRenderer_PixelToInkSpaceFromPoints(This,hDC,Points)	\
    ( (This)->lpVtbl -> PixelToInkSpaceFromPoints(This,hDC,Points) ) 

#define IInkRenderer_InkSpaceToPixelFromPoints(This,hDC,Points)	\
    ( (This)->lpVtbl -> InkSpaceToPixelFromPoints(This,hDC,Points) ) 

#define IInkRenderer_Measure(This,Strokes,Rectangle)	\
    ( (This)->lpVtbl -> Measure(This,Strokes,Rectangle) ) 

#define IInkRenderer_MeasureStroke(This,Stroke,DrawingAttributes,Rectangle)	\
    ( (This)->lpVtbl -> MeasureStroke(This,Stroke,DrawingAttributes,Rectangle) ) 

#define IInkRenderer_Move(This,HorizontalComponent,VerticalComponent)	\
    ( (This)->lpVtbl -> Move(This,HorizontalComponent,VerticalComponent) ) 

#define IInkRenderer_Rotate(This,Degrees,x,y)	\
    ( (This)->lpVtbl -> Rotate(This,Degrees,x,y) ) 

#define IInkRenderer_ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier,ApplyOnPenWidth)	\
    ( (This)->lpVtbl -> ScaleTransform(This,HorizontalMultiplier,VerticalMultiplier,ApplyOnPenWidth) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRenderer_INTERFACE_DEFINED__ */


#ifndef __IInkCollector_INTERFACE_DEFINED__
#define __IInkCollector_INTERFACE_DEFINED__

/* interface IInkCollector */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkCollector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F0F060B5-8B1F-4a7c-89EC-880692588A4F")
    IInkCollector : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_hWnd( 
            /* [retval][out] */ __RPC__out LONG_PTR *CurrentWindow) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_hWnd( 
            /* [in] */ LONG_PTR NewWindow) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultDrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **CurrentAttributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_DefaultDrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *NewAttributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Renderer( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRenderer **CurrentInkRenderer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Renderer( 
            /* [in] */ __RPC__in_opt IInkRenderer *NewInkRenderer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ink( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Ink( 
            /* [in] */ __RPC__in_opt IInkDisp *NewInk) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoRedraw( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *AutoRedraw) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoRedraw( 
            /* [in] */ VARIANT_BOOL AutoRedraw) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CollectingInk( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CollectionMode( 
            /* [retval][out] */ __RPC__out InkCollectionMode *Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CollectionMode( 
            /* [in] */ InkCollectionMode Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DynamicRendering( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Enabled) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DynamicRendering( 
            /* [in] */ VARIANT_BOOL Enabled) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DesiredPacketDescription( 
            /* [retval][out] */ __RPC__out VARIANT *PacketGuids) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DesiredPacketDescription( 
            /* [in] */ VARIANT PacketGuids) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MouseIcon( 
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MouseIcon( 
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_MouseIcon( 
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MousePointer( 
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MousePointer( 
            /* [in] */ InkMousePointer MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cursors( 
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **Cursors) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MarginX( 
            /* [retval][out] */ __RPC__out long *MarginX) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MarginX( 
            /* [in] */ long MarginX) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MarginY( 
            /* [retval][out] */ __RPC__out long *MarginY) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MarginY( 
            /* [in] */ long MarginY) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tablet( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **SingleTablet) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportHighContrastInk( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SupportHighContrastInk( 
            /* [in] */ VARIANT_BOOL Support) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listening) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWindowInputRectangle( 
            /* [out][in] */ __RPC__deref_inout_opt IInkRectangle **WindowInputRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWindowInputRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *WindowInputRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAllTabletsMode( 
            /* [defaultvalue][in] */ VARIANT_BOOL UseMouseForInput = -1) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSingleTabletIntegratedMode( 
            /* [in] */ __RPC__in_opt IInkTablet *Tablet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEventInterest( 
            /* [in] */ InkCollectorEventInterest EventId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEventInterest( 
            /* [in] */ InkCollectorEventInterest EventId,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkCollectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkCollector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkCollector * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkCollector * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkCollector * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkCollector * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_hWnd)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_hWnd )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out LONG_PTR *CurrentWindow);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_hWnd)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_hWnd )( 
            __RPC__in IInkCollector * This,
            /* [in] */ LONG_PTR NewWindow);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_Enabled)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_Enabled)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IInkCollector * This,
            /* [in] */ VARIANT_BOOL Collecting);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_DefaultDrawingAttributes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultDrawingAttributes )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **CurrentAttributes);
        
        DECLSPEC_XFGVIRT(IInkCollector, putref_DefaultDrawingAttributes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_DefaultDrawingAttributes )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *NewAttributes);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_Renderer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Renderer )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRenderer **CurrentInkRenderer);
        
        DECLSPEC_XFGVIRT(IInkCollector, putref_Renderer)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Renderer )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in_opt IInkRenderer *NewInkRenderer);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_Ink)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ink )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink);
        
        DECLSPEC_XFGVIRT(IInkCollector, putref_Ink)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Ink )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in_opt IInkDisp *NewInk);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_AutoRedraw)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoRedraw )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *AutoRedraw);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_AutoRedraw)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoRedraw )( 
            __RPC__in IInkCollector * This,
            /* [in] */ VARIANT_BOOL AutoRedraw);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_CollectingInk)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectingInk )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_CollectionMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectionMode )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out InkCollectionMode *Mode);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_CollectionMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CollectionMode )( 
            __RPC__in IInkCollector * This,
            /* [in] */ InkCollectionMode Mode);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_DynamicRendering)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DynamicRendering )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Enabled);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_DynamicRendering)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DynamicRendering )( 
            __RPC__in IInkCollector * This,
            /* [in] */ VARIANT_BOOL Enabled);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_DesiredPacketDescription)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesiredPacketDescription )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out VARIANT *PacketGuids);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_DesiredPacketDescription)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesiredPacketDescription )( 
            __RPC__in IInkCollector * This,
            /* [in] */ VARIANT PacketGuids);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_MouseIcon)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MouseIcon )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_MouseIcon)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MouseIcon )( 
            __RPC__in IInkCollector * This,
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkCollector, putref_MouseIcon)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_MouseIcon )( 
            __RPC__in IInkCollector * This,
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_MousePointer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MousePointer )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_MousePointer)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MousePointer )( 
            __RPC__in IInkCollector * This,
            /* [in] */ InkMousePointer MousePointer);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_Cursors)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cursors )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **Cursors);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_MarginX)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MarginX )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out long *MarginX);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_MarginX)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MarginX )( 
            __RPC__in IInkCollector * This,
            /* [in] */ long MarginX);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_MarginY)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MarginY )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out long *MarginY);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_MarginY)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MarginY )( 
            __RPC__in IInkCollector * This,
            /* [in] */ long MarginY);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_Tablet)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tablet )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **SingleTablet);
        
        DECLSPEC_XFGVIRT(IInkCollector, get_SupportHighContrastInk)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportHighContrastInk )( 
            __RPC__in IInkCollector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support);
        
        DECLSPEC_XFGVIRT(IInkCollector, put_SupportHighContrastInk)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SupportHighContrastInk )( 
            __RPC__in IInkCollector * This,
            /* [in] */ VARIANT_BOOL Support);
        
        DECLSPEC_XFGVIRT(IInkCollector, SetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetGestureStatus )( 
            __RPC__in IInkCollector * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen);
        
        DECLSPEC_XFGVIRT(IInkCollector, GetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGestureStatus )( 
            __RPC__in IInkCollector * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listening);
        
        DECLSPEC_XFGVIRT(IInkCollector, GetWindowInputRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWindowInputRectangle )( 
            __RPC__in IInkCollector * This,
            /* [out][in] */ __RPC__deref_inout_opt IInkRectangle **WindowInputRectangle);
        
        DECLSPEC_XFGVIRT(IInkCollector, SetWindowInputRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWindowInputRectangle )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in_opt IInkRectangle *WindowInputRectangle);
        
        DECLSPEC_XFGVIRT(IInkCollector, SetAllTabletsMode)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllTabletsMode )( 
            __RPC__in IInkCollector * This,
            /* [defaultvalue][in] */ VARIANT_BOOL UseMouseForInput);
        
        DECLSPEC_XFGVIRT(IInkCollector, SetSingleTabletIntegratedMode)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSingleTabletIntegratedMode )( 
            __RPC__in IInkCollector * This,
            /* [in] */ __RPC__in_opt IInkTablet *Tablet);
        
        DECLSPEC_XFGVIRT(IInkCollector, GetEventInterest)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEventInterest )( 
            __RPC__in IInkCollector * This,
            /* [in] */ InkCollectorEventInterest EventId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listen);
        
        DECLSPEC_XFGVIRT(IInkCollector, SetEventInterest)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEventInterest )( 
            __RPC__in IInkCollector * This,
            /* [in] */ InkCollectorEventInterest EventId,
            /* [in] */ VARIANT_BOOL Listen);
        
        END_INTERFACE
    } IInkCollectorVtbl;

    interface IInkCollector
    {
        CONST_VTBL struct IInkCollectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkCollector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkCollector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkCollector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkCollector_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkCollector_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkCollector_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkCollector_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkCollector_get_hWnd(This,CurrentWindow)	\
    ( (This)->lpVtbl -> get_hWnd(This,CurrentWindow) ) 

#define IInkCollector_put_hWnd(This,NewWindow)	\
    ( (This)->lpVtbl -> put_hWnd(This,NewWindow) ) 

#define IInkCollector_get_Enabled(This,Collecting)	\
    ( (This)->lpVtbl -> get_Enabled(This,Collecting) ) 

#define IInkCollector_put_Enabled(This,Collecting)	\
    ( (This)->lpVtbl -> put_Enabled(This,Collecting) ) 

#define IInkCollector_get_DefaultDrawingAttributes(This,CurrentAttributes)	\
    ( (This)->lpVtbl -> get_DefaultDrawingAttributes(This,CurrentAttributes) ) 

#define IInkCollector_putref_DefaultDrawingAttributes(This,NewAttributes)	\
    ( (This)->lpVtbl -> putref_DefaultDrawingAttributes(This,NewAttributes) ) 

#define IInkCollector_get_Renderer(This,CurrentInkRenderer)	\
    ( (This)->lpVtbl -> get_Renderer(This,CurrentInkRenderer) ) 

#define IInkCollector_putref_Renderer(This,NewInkRenderer)	\
    ( (This)->lpVtbl -> putref_Renderer(This,NewInkRenderer) ) 

#define IInkCollector_get_Ink(This,Ink)	\
    ( (This)->lpVtbl -> get_Ink(This,Ink) ) 

#define IInkCollector_putref_Ink(This,NewInk)	\
    ( (This)->lpVtbl -> putref_Ink(This,NewInk) ) 

#define IInkCollector_get_AutoRedraw(This,AutoRedraw)	\
    ( (This)->lpVtbl -> get_AutoRedraw(This,AutoRedraw) ) 

#define IInkCollector_put_AutoRedraw(This,AutoRedraw)	\
    ( (This)->lpVtbl -> put_AutoRedraw(This,AutoRedraw) ) 

#define IInkCollector_get_CollectingInk(This,Collecting)	\
    ( (This)->lpVtbl -> get_CollectingInk(This,Collecting) ) 

#define IInkCollector_get_CollectionMode(This,Mode)	\
    ( (This)->lpVtbl -> get_CollectionMode(This,Mode) ) 

#define IInkCollector_put_CollectionMode(This,Mode)	\
    ( (This)->lpVtbl -> put_CollectionMode(This,Mode) ) 

#define IInkCollector_get_DynamicRendering(This,Enabled)	\
    ( (This)->lpVtbl -> get_DynamicRendering(This,Enabled) ) 

#define IInkCollector_put_DynamicRendering(This,Enabled)	\
    ( (This)->lpVtbl -> put_DynamicRendering(This,Enabled) ) 

#define IInkCollector_get_DesiredPacketDescription(This,PacketGuids)	\
    ( (This)->lpVtbl -> get_DesiredPacketDescription(This,PacketGuids) ) 

#define IInkCollector_put_DesiredPacketDescription(This,PacketGuids)	\
    ( (This)->lpVtbl -> put_DesiredPacketDescription(This,PacketGuids) ) 

#define IInkCollector_get_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> get_MouseIcon(This,MouseIcon) ) 

#define IInkCollector_put_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> put_MouseIcon(This,MouseIcon) ) 

#define IInkCollector_putref_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> putref_MouseIcon(This,MouseIcon) ) 

#define IInkCollector_get_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> get_MousePointer(This,MousePointer) ) 

#define IInkCollector_put_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> put_MousePointer(This,MousePointer) ) 

#define IInkCollector_get_Cursors(This,Cursors)	\
    ( (This)->lpVtbl -> get_Cursors(This,Cursors) ) 

#define IInkCollector_get_MarginX(This,MarginX)	\
    ( (This)->lpVtbl -> get_MarginX(This,MarginX) ) 

#define IInkCollector_put_MarginX(This,MarginX)	\
    ( (This)->lpVtbl -> put_MarginX(This,MarginX) ) 

#define IInkCollector_get_MarginY(This,MarginY)	\
    ( (This)->lpVtbl -> get_MarginY(This,MarginY) ) 

#define IInkCollector_put_MarginY(This,MarginY)	\
    ( (This)->lpVtbl -> put_MarginY(This,MarginY) ) 

#define IInkCollector_get_Tablet(This,SingleTablet)	\
    ( (This)->lpVtbl -> get_Tablet(This,SingleTablet) ) 

#define IInkCollector_get_SupportHighContrastInk(This,Support)	\
    ( (This)->lpVtbl -> get_SupportHighContrastInk(This,Support) ) 

#define IInkCollector_put_SupportHighContrastInk(This,Support)	\
    ( (This)->lpVtbl -> put_SupportHighContrastInk(This,Support) ) 

#define IInkCollector_SetGestureStatus(This,Gesture,Listen)	\
    ( (This)->lpVtbl -> SetGestureStatus(This,Gesture,Listen) ) 

#define IInkCollector_GetGestureStatus(This,Gesture,Listening)	\
    ( (This)->lpVtbl -> GetGestureStatus(This,Gesture,Listening) ) 

#define IInkCollector_GetWindowInputRectangle(This,WindowInputRectangle)	\
    ( (This)->lpVtbl -> GetWindowInputRectangle(This,WindowInputRectangle) ) 

#define IInkCollector_SetWindowInputRectangle(This,WindowInputRectangle)	\
    ( (This)->lpVtbl -> SetWindowInputRectangle(This,WindowInputRectangle) ) 

#define IInkCollector_SetAllTabletsMode(This,UseMouseForInput)	\
    ( (This)->lpVtbl -> SetAllTabletsMode(This,UseMouseForInput) ) 

#define IInkCollector_SetSingleTabletIntegratedMode(This,Tablet)	\
    ( (This)->lpVtbl -> SetSingleTabletIntegratedMode(This,Tablet) ) 

#define IInkCollector_GetEventInterest(This,EventId,Listen)	\
    ( (This)->lpVtbl -> GetEventInterest(This,EventId,Listen) ) 

#define IInkCollector_SetEventInterest(This,EventId,Listen)	\
    ( (This)->lpVtbl -> SetEventInterest(This,EventId,Listen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkCollector_INTERFACE_DEFINED__ */


#ifndef ___IInkCollectorEvents_DISPINTERFACE_DEFINED__
#define ___IInkCollectorEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkCollectorEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkCollectorEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("11A583F2-712D-4fea-ABCF-AB4AF38EA06B")
    _IInkCollectorEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkCollectorEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkCollectorEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkCollectorEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkCollectorEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkCollectorEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkCollectorEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkCollectorEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkCollectorEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkCollectorEventsVtbl;

    interface _IInkCollectorEvents
    {
        CONST_VTBL struct _IInkCollectorEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkCollectorEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkCollectorEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkCollectorEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkCollectorEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkCollectorEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkCollectorEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkCollectorEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkCollectorEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IInkOverlay_INTERFACE_DEFINED__
#define __IInkOverlay_INTERFACE_DEFINED__

/* interface IInkOverlay */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkOverlay;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b82a463b-c1c5-45a3-997c-deab5651b67a")
    IInkOverlay : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_hWnd( 
            /* [retval][out] */ __RPC__out LONG_PTR *CurrentWindow) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_hWnd( 
            /* [in] */ LONG_PTR NewWindow) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultDrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **CurrentAttributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_DefaultDrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *NewAttributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Renderer( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRenderer **CurrentInkRenderer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Renderer( 
            /* [in] */ __RPC__in_opt IInkRenderer *NewInkRenderer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ink( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Ink( 
            /* [in] */ __RPC__in_opt IInkDisp *NewInk) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoRedraw( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *AutoRedraw) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoRedraw( 
            /* [in] */ VARIANT_BOOL AutoRedraw) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CollectingInk( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CollectionMode( 
            /* [retval][out] */ __RPC__out InkCollectionMode *Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CollectionMode( 
            /* [in] */ InkCollectionMode Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DynamicRendering( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Enabled) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DynamicRendering( 
            /* [in] */ VARIANT_BOOL Enabled) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DesiredPacketDescription( 
            /* [retval][out] */ __RPC__out VARIANT *PacketGuids) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DesiredPacketDescription( 
            /* [in] */ VARIANT PacketGuids) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MouseIcon( 
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MouseIcon( 
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_MouseIcon( 
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MousePointer( 
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MousePointer( 
            /* [in] */ InkMousePointer MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EditingMode( 
            /* [retval][out] */ __RPC__out InkOverlayEditingMode *EditingMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EditingMode( 
            /* [in] */ InkOverlayEditingMode EditingMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Selection( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Selection) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Selection( 
            /* [in] */ __RPC__in_opt IInkStrokes *Selection) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EraserMode( 
            /* [retval][out] */ __RPC__out InkOverlayEraserMode *EraserMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EraserMode( 
            /* [in] */ InkOverlayEraserMode EraserMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EraserWidth( 
            /* [retval][out] */ __RPC__out long *EraserWidth) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EraserWidth( 
            /* [in] */ long newEraserWidth) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AttachMode( 
            /* [retval][out] */ __RPC__out InkOverlayAttachMode *AttachMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AttachMode( 
            /* [in] */ InkOverlayAttachMode AttachMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cursors( 
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **Cursors) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MarginX( 
            /* [retval][out] */ __RPC__out long *MarginX) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MarginX( 
            /* [in] */ long MarginX) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MarginY( 
            /* [retval][out] */ __RPC__out long *MarginY) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MarginY( 
            /* [in] */ long MarginY) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tablet( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **SingleTablet) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportHighContrastInk( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SupportHighContrastInk( 
            /* [in] */ VARIANT_BOOL Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportHighContrastSelectionUI( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SupportHighContrastSelectionUI( 
            /* [in] */ VARIANT_BOOL Support) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE HitTestSelection( 
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__out SelectionHitResult *SelArea) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Draw( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rect) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listening) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWindowInputRectangle( 
            /* [out][in] */ __RPC__deref_inout_opt IInkRectangle **WindowInputRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWindowInputRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *WindowInputRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAllTabletsMode( 
            /* [defaultvalue][in] */ VARIANT_BOOL UseMouseForInput = -1) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSingleTabletIntegratedMode( 
            /* [in] */ __RPC__in_opt IInkTablet *Tablet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEventInterest( 
            /* [in] */ InkCollectorEventInterest EventId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEventInterest( 
            /* [in] */ InkCollectorEventInterest EventId,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkOverlayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkOverlay * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkOverlay * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkOverlay * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkOverlay * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_hWnd)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_hWnd )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out LONG_PTR *CurrentWindow);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_hWnd)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_hWnd )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ LONG_PTR NewWindow);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_Enabled)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_Enabled)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ VARIANT_BOOL Collecting);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_DefaultDrawingAttributes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultDrawingAttributes )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **CurrentAttributes);
        
        DECLSPEC_XFGVIRT(IInkOverlay, putref_DefaultDrawingAttributes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_DefaultDrawingAttributes )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *NewAttributes);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_Renderer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Renderer )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRenderer **CurrentInkRenderer);
        
        DECLSPEC_XFGVIRT(IInkOverlay, putref_Renderer)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Renderer )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkRenderer *NewInkRenderer);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_Ink)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ink )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink);
        
        DECLSPEC_XFGVIRT(IInkOverlay, putref_Ink)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Ink )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkDisp *NewInk);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_AutoRedraw)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoRedraw )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *AutoRedraw);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_AutoRedraw)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoRedraw )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ VARIANT_BOOL AutoRedraw);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_CollectingInk)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectingInk )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_CollectionMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectionMode )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out InkCollectionMode *Mode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_CollectionMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CollectionMode )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkCollectionMode Mode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_DynamicRendering)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DynamicRendering )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Enabled);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_DynamicRendering)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DynamicRendering )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ VARIANT_BOOL Enabled);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_DesiredPacketDescription)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesiredPacketDescription )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT *PacketGuids);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_DesiredPacketDescription)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesiredPacketDescription )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ VARIANT PacketGuids);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_MouseIcon)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MouseIcon )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_MouseIcon)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MouseIcon )( 
            __RPC__in IInkOverlay * This,
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkOverlay, putref_MouseIcon)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_MouseIcon )( 
            __RPC__in IInkOverlay * This,
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_MousePointer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MousePointer )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_MousePointer)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MousePointer )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkMousePointer MousePointer);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_EditingMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EditingMode )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out InkOverlayEditingMode *EditingMode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_EditingMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EditingMode )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkOverlayEditingMode EditingMode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_Selection)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Selection )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Selection);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_Selection)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Selection )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Selection);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_EraserMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EraserMode )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out InkOverlayEraserMode *EraserMode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_EraserMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EraserMode )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkOverlayEraserMode EraserMode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_EraserWidth)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EraserWidth )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out long *EraserWidth);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_EraserWidth)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EraserWidth )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ long newEraserWidth);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_AttachMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AttachMode )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out InkOverlayAttachMode *AttachMode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_AttachMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AttachMode )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkOverlayAttachMode AttachMode);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_Cursors)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cursors )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **Cursors);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_MarginX)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MarginX )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out long *MarginX);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_MarginX)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MarginX )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ long MarginX);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_MarginY)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MarginY )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out long *MarginY);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_MarginY)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MarginY )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ long MarginY);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_Tablet)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tablet )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **SingleTablet);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_SupportHighContrastInk)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportHighContrastInk )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_SupportHighContrastInk)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SupportHighContrastInk )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ VARIANT_BOOL Support);
        
        DECLSPEC_XFGVIRT(IInkOverlay, get_SupportHighContrastSelectionUI)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportHighContrastSelectionUI )( 
            __RPC__in IInkOverlay * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support);
        
        DECLSPEC_XFGVIRT(IInkOverlay, put_SupportHighContrastSelectionUI)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SupportHighContrastSelectionUI )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ VARIANT_BOOL Support);
        
        DECLSPEC_XFGVIRT(IInkOverlay, HitTestSelection)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HitTestSelection )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__out SelectionHitResult *SelArea);
        
        DECLSPEC_XFGVIRT(IInkOverlay, Draw)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Draw )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rect);
        
        DECLSPEC_XFGVIRT(IInkOverlay, SetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetGestureStatus )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen);
        
        DECLSPEC_XFGVIRT(IInkOverlay, GetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGestureStatus )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listening);
        
        DECLSPEC_XFGVIRT(IInkOverlay, GetWindowInputRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWindowInputRectangle )( 
            __RPC__in IInkOverlay * This,
            /* [out][in] */ __RPC__deref_inout_opt IInkRectangle **WindowInputRectangle);
        
        DECLSPEC_XFGVIRT(IInkOverlay, SetWindowInputRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWindowInputRectangle )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkRectangle *WindowInputRectangle);
        
        DECLSPEC_XFGVIRT(IInkOverlay, SetAllTabletsMode)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllTabletsMode )( 
            __RPC__in IInkOverlay * This,
            /* [defaultvalue][in] */ VARIANT_BOOL UseMouseForInput);
        
        DECLSPEC_XFGVIRT(IInkOverlay, SetSingleTabletIntegratedMode)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSingleTabletIntegratedMode )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ __RPC__in_opt IInkTablet *Tablet);
        
        DECLSPEC_XFGVIRT(IInkOverlay, GetEventInterest)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEventInterest )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkCollectorEventInterest EventId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listen);
        
        DECLSPEC_XFGVIRT(IInkOverlay, SetEventInterest)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEventInterest )( 
            __RPC__in IInkOverlay * This,
            /* [in] */ InkCollectorEventInterest EventId,
            /* [in] */ VARIANT_BOOL Listen);
        
        END_INTERFACE
    } IInkOverlayVtbl;

    interface IInkOverlay
    {
        CONST_VTBL struct IInkOverlayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkOverlay_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkOverlay_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkOverlay_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkOverlay_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkOverlay_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkOverlay_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkOverlay_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkOverlay_get_hWnd(This,CurrentWindow)	\
    ( (This)->lpVtbl -> get_hWnd(This,CurrentWindow) ) 

#define IInkOverlay_put_hWnd(This,NewWindow)	\
    ( (This)->lpVtbl -> put_hWnd(This,NewWindow) ) 

#define IInkOverlay_get_Enabled(This,Collecting)	\
    ( (This)->lpVtbl -> get_Enabled(This,Collecting) ) 

#define IInkOverlay_put_Enabled(This,Collecting)	\
    ( (This)->lpVtbl -> put_Enabled(This,Collecting) ) 

#define IInkOverlay_get_DefaultDrawingAttributes(This,CurrentAttributes)	\
    ( (This)->lpVtbl -> get_DefaultDrawingAttributes(This,CurrentAttributes) ) 

#define IInkOverlay_putref_DefaultDrawingAttributes(This,NewAttributes)	\
    ( (This)->lpVtbl -> putref_DefaultDrawingAttributes(This,NewAttributes) ) 

#define IInkOverlay_get_Renderer(This,CurrentInkRenderer)	\
    ( (This)->lpVtbl -> get_Renderer(This,CurrentInkRenderer) ) 

#define IInkOverlay_putref_Renderer(This,NewInkRenderer)	\
    ( (This)->lpVtbl -> putref_Renderer(This,NewInkRenderer) ) 

#define IInkOverlay_get_Ink(This,Ink)	\
    ( (This)->lpVtbl -> get_Ink(This,Ink) ) 

#define IInkOverlay_putref_Ink(This,NewInk)	\
    ( (This)->lpVtbl -> putref_Ink(This,NewInk) ) 

#define IInkOverlay_get_AutoRedraw(This,AutoRedraw)	\
    ( (This)->lpVtbl -> get_AutoRedraw(This,AutoRedraw) ) 

#define IInkOverlay_put_AutoRedraw(This,AutoRedraw)	\
    ( (This)->lpVtbl -> put_AutoRedraw(This,AutoRedraw) ) 

#define IInkOverlay_get_CollectingInk(This,Collecting)	\
    ( (This)->lpVtbl -> get_CollectingInk(This,Collecting) ) 

#define IInkOverlay_get_CollectionMode(This,Mode)	\
    ( (This)->lpVtbl -> get_CollectionMode(This,Mode) ) 

#define IInkOverlay_put_CollectionMode(This,Mode)	\
    ( (This)->lpVtbl -> put_CollectionMode(This,Mode) ) 

#define IInkOverlay_get_DynamicRendering(This,Enabled)	\
    ( (This)->lpVtbl -> get_DynamicRendering(This,Enabled) ) 

#define IInkOverlay_put_DynamicRendering(This,Enabled)	\
    ( (This)->lpVtbl -> put_DynamicRendering(This,Enabled) ) 

#define IInkOverlay_get_DesiredPacketDescription(This,PacketGuids)	\
    ( (This)->lpVtbl -> get_DesiredPacketDescription(This,PacketGuids) ) 

#define IInkOverlay_put_DesiredPacketDescription(This,PacketGuids)	\
    ( (This)->lpVtbl -> put_DesiredPacketDescription(This,PacketGuids) ) 

#define IInkOverlay_get_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> get_MouseIcon(This,MouseIcon) ) 

#define IInkOverlay_put_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> put_MouseIcon(This,MouseIcon) ) 

#define IInkOverlay_putref_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> putref_MouseIcon(This,MouseIcon) ) 

#define IInkOverlay_get_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> get_MousePointer(This,MousePointer) ) 

#define IInkOverlay_put_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> put_MousePointer(This,MousePointer) ) 

#define IInkOverlay_get_EditingMode(This,EditingMode)	\
    ( (This)->lpVtbl -> get_EditingMode(This,EditingMode) ) 

#define IInkOverlay_put_EditingMode(This,EditingMode)	\
    ( (This)->lpVtbl -> put_EditingMode(This,EditingMode) ) 

#define IInkOverlay_get_Selection(This,Selection)	\
    ( (This)->lpVtbl -> get_Selection(This,Selection) ) 

#define IInkOverlay_put_Selection(This,Selection)	\
    ( (This)->lpVtbl -> put_Selection(This,Selection) ) 

#define IInkOverlay_get_EraserMode(This,EraserMode)	\
    ( (This)->lpVtbl -> get_EraserMode(This,EraserMode) ) 

#define IInkOverlay_put_EraserMode(This,EraserMode)	\
    ( (This)->lpVtbl -> put_EraserMode(This,EraserMode) ) 

#define IInkOverlay_get_EraserWidth(This,EraserWidth)	\
    ( (This)->lpVtbl -> get_EraserWidth(This,EraserWidth) ) 

#define IInkOverlay_put_EraserWidth(This,newEraserWidth)	\
    ( (This)->lpVtbl -> put_EraserWidth(This,newEraserWidth) ) 

#define IInkOverlay_get_AttachMode(This,AttachMode)	\
    ( (This)->lpVtbl -> get_AttachMode(This,AttachMode) ) 

#define IInkOverlay_put_AttachMode(This,AttachMode)	\
    ( (This)->lpVtbl -> put_AttachMode(This,AttachMode) ) 

#define IInkOverlay_get_Cursors(This,Cursors)	\
    ( (This)->lpVtbl -> get_Cursors(This,Cursors) ) 

#define IInkOverlay_get_MarginX(This,MarginX)	\
    ( (This)->lpVtbl -> get_MarginX(This,MarginX) ) 

#define IInkOverlay_put_MarginX(This,MarginX)	\
    ( (This)->lpVtbl -> put_MarginX(This,MarginX) ) 

#define IInkOverlay_get_MarginY(This,MarginY)	\
    ( (This)->lpVtbl -> get_MarginY(This,MarginY) ) 

#define IInkOverlay_put_MarginY(This,MarginY)	\
    ( (This)->lpVtbl -> put_MarginY(This,MarginY) ) 

#define IInkOverlay_get_Tablet(This,SingleTablet)	\
    ( (This)->lpVtbl -> get_Tablet(This,SingleTablet) ) 

#define IInkOverlay_get_SupportHighContrastInk(This,Support)	\
    ( (This)->lpVtbl -> get_SupportHighContrastInk(This,Support) ) 

#define IInkOverlay_put_SupportHighContrastInk(This,Support)	\
    ( (This)->lpVtbl -> put_SupportHighContrastInk(This,Support) ) 

#define IInkOverlay_get_SupportHighContrastSelectionUI(This,Support)	\
    ( (This)->lpVtbl -> get_SupportHighContrastSelectionUI(This,Support) ) 

#define IInkOverlay_put_SupportHighContrastSelectionUI(This,Support)	\
    ( (This)->lpVtbl -> put_SupportHighContrastSelectionUI(This,Support) ) 

#define IInkOverlay_HitTestSelection(This,x,y,SelArea)	\
    ( (This)->lpVtbl -> HitTestSelection(This,x,y,SelArea) ) 

#define IInkOverlay_Draw(This,Rect)	\
    ( (This)->lpVtbl -> Draw(This,Rect) ) 

#define IInkOverlay_SetGestureStatus(This,Gesture,Listen)	\
    ( (This)->lpVtbl -> SetGestureStatus(This,Gesture,Listen) ) 

#define IInkOverlay_GetGestureStatus(This,Gesture,Listening)	\
    ( (This)->lpVtbl -> GetGestureStatus(This,Gesture,Listening) ) 

#define IInkOverlay_GetWindowInputRectangle(This,WindowInputRectangle)	\
    ( (This)->lpVtbl -> GetWindowInputRectangle(This,WindowInputRectangle) ) 

#define IInkOverlay_SetWindowInputRectangle(This,WindowInputRectangle)	\
    ( (This)->lpVtbl -> SetWindowInputRectangle(This,WindowInputRectangle) ) 

#define IInkOverlay_SetAllTabletsMode(This,UseMouseForInput)	\
    ( (This)->lpVtbl -> SetAllTabletsMode(This,UseMouseForInput) ) 

#define IInkOverlay_SetSingleTabletIntegratedMode(This,Tablet)	\
    ( (This)->lpVtbl -> SetSingleTabletIntegratedMode(This,Tablet) ) 

#define IInkOverlay_GetEventInterest(This,EventId,Listen)	\
    ( (This)->lpVtbl -> GetEventInterest(This,EventId,Listen) ) 

#define IInkOverlay_SetEventInterest(This,EventId,Listen)	\
    ( (This)->lpVtbl -> SetEventInterest(This,EventId,Listen) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkOverlay_INTERFACE_DEFINED__ */


#ifndef ___IInkOverlayEvents_DISPINTERFACE_DEFINED__
#define ___IInkOverlayEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkOverlayEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkOverlayEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("31179b69-e563-489e-b16f-712f1e8a0651")
    _IInkOverlayEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkOverlayEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkOverlayEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkOverlayEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkOverlayEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkOverlayEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkOverlayEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkOverlayEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkOverlayEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkOverlayEventsVtbl;

    interface _IInkOverlayEvents
    {
        CONST_VTBL struct _IInkOverlayEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkOverlayEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkOverlayEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkOverlayEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkOverlayEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkOverlayEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkOverlayEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkOverlayEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkOverlayEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IInkPicture_INTERFACE_DEFINED__
#define __IInkPicture_INTERFACE_DEFINED__

/* interface IInkPicture */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkPicture;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e85662e0-379a-40d7-9b5c-757d233f9923")
    IInkPicture : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_hWnd( 
            /* [retval][out] */ __RPC__out LONG_PTR *CurrentWindow) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DefaultDrawingAttributes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **CurrentAttributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_DefaultDrawingAttributes( 
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *NewAttributes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Renderer( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRenderer **CurrentInkRenderer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Renderer( 
            /* [in] */ __RPC__in_opt IInkRenderer *NewInkRenderer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ink( 
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Ink( 
            /* [in] */ __RPC__in_opt IInkDisp *NewInk) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_AutoRedraw( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *AutoRedraw) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_AutoRedraw( 
            /* [in] */ VARIANT_BOOL AutoRedraw) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CollectingInk( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CollectionMode( 
            /* [retval][out] */ __RPC__out InkCollectionMode *Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CollectionMode( 
            /* [in] */ InkCollectionMode Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DynamicRendering( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Enabled) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DynamicRendering( 
            /* [in] */ VARIANT_BOOL Enabled) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DesiredPacketDescription( 
            /* [retval][out] */ __RPC__out VARIANT *PacketGuids) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DesiredPacketDescription( 
            /* [in] */ VARIANT PacketGuids) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MouseIcon( 
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MouseIcon( 
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_MouseIcon( 
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MousePointer( 
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MousePointer( 
            /* [in] */ InkMousePointer MousePointer) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EditingMode( 
            /* [retval][out] */ __RPC__out InkOverlayEditingMode *EditingMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EditingMode( 
            /* [in] */ InkOverlayEditingMode EditingMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Selection( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Selection) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Selection( 
            /* [in] */ __RPC__in_opt IInkStrokes *Selection) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EraserMode( 
            /* [retval][out] */ __RPC__out InkOverlayEraserMode *EraserMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EraserMode( 
            /* [in] */ InkOverlayEraserMode EraserMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EraserWidth( 
            /* [retval][out] */ __RPC__out long *EraserWidth) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EraserWidth( 
            /* [in] */ long newEraserWidth) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Picture( 
            /* [in] */ __RPC__in_opt IPictureDisp *pPicture) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Picture( 
            /* [in] */ __RPC__in_opt IPictureDisp *pPicture) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Picture( 
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **ppPicture) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SizeMode( 
            /* [in] */ InkPictureSizeMode smNewSizeMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SizeMode( 
            /* [retval][out] */ __RPC__out InkPictureSizeMode *smSizeMode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_BackColor( 
            /* [in] */ OLE_COLOR newColor) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_BackColor( 
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Cursors( 
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **Cursors) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MarginX( 
            /* [retval][out] */ __RPC__out long *MarginX) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MarginX( 
            /* [in] */ long MarginX) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_MarginY( 
            /* [retval][out] */ __RPC__out long *MarginY) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_MarginY( 
            /* [in] */ long MarginY) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Tablet( 
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **SingleTablet) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportHighContrastInk( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SupportHighContrastInk( 
            /* [in] */ VARIANT_BOOL Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportHighContrastSelectionUI( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SupportHighContrastSelectionUI( 
            /* [in] */ VARIANT_BOOL Support) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE HitTestSelection( 
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__out SelectionHitResult *SelArea) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetGestureStatus( 
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listening) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetWindowInputRectangle( 
            /* [out][in] */ __RPC__deref_inout_opt IInkRectangle **WindowInputRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetWindowInputRectangle( 
            /* [in] */ __RPC__in_opt IInkRectangle *WindowInputRectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetAllTabletsMode( 
            /* [defaultvalue][in] */ VARIANT_BOOL UseMouseForInput = -1) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetSingleTabletIntegratedMode( 
            /* [in] */ __RPC__in_opt IInkTablet *Tablet) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetEventInterest( 
            /* [in] */ InkCollectorEventInterest EventId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetEventInterest( 
            /* [in] */ InkCollectorEventInterest EventId,
            /* [in] */ VARIANT_BOOL Listen) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_InkEnabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_InkEnabled( 
            /* [in] */ VARIANT_BOOL Collecting) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Enabled( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbool) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Enabled( 
            /* [in] */ VARIANT_BOOL vbool) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkPictureVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkPicture * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkPicture * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkPicture * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkPicture * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkPicture * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_hWnd)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_hWnd )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out LONG_PTR *CurrentWindow);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_DefaultDrawingAttributes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultDrawingAttributes )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDrawingAttributes **CurrentAttributes);
        
        DECLSPEC_XFGVIRT(IInkPicture, putref_DefaultDrawingAttributes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_DefaultDrawingAttributes )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IInkDrawingAttributes *NewAttributes);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Renderer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Renderer )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRenderer **CurrentInkRenderer);
        
        DECLSPEC_XFGVIRT(IInkPicture, putref_Renderer)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Renderer )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IInkRenderer *NewInkRenderer);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Ink)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ink )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkDisp **Ink);
        
        DECLSPEC_XFGVIRT(IInkPicture, putref_Ink)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Ink )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IInkDisp *NewInk);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_AutoRedraw)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_AutoRedraw )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *AutoRedraw);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_AutoRedraw)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_AutoRedraw )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT_BOOL AutoRedraw);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_CollectingInk)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectingInk )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_CollectionMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CollectionMode )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out InkCollectionMode *Mode);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_CollectionMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CollectionMode )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkCollectionMode Mode);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_DynamicRendering)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DynamicRendering )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Enabled);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_DynamicRendering)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DynamicRendering )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT_BOOL Enabled);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_DesiredPacketDescription)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DesiredPacketDescription )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT *PacketGuids);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_DesiredPacketDescription)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DesiredPacketDescription )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT PacketGuids);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_MouseIcon)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MouseIcon )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_MouseIcon)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MouseIcon )( 
            __RPC__in IInkPicture * This,
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkPicture, putref_MouseIcon)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_MouseIcon )( 
            __RPC__in IInkPicture * This,
            /* [unique][in] */ __RPC__in_opt IPictureDisp *MouseIcon);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_MousePointer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MousePointer )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out InkMousePointer *MousePointer);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_MousePointer)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MousePointer )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkMousePointer MousePointer);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_EditingMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EditingMode )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out InkOverlayEditingMode *EditingMode);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_EditingMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EditingMode )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkOverlayEditingMode EditingMode);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Selection)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Selection )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Selection);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_Selection)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Selection )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Selection);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_EraserMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EraserMode )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out InkOverlayEraserMode *EraserMode);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_EraserMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EraserMode )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkOverlayEraserMode EraserMode);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_EraserWidth)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EraserWidth )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out long *EraserWidth);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_EraserWidth)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EraserWidth )( 
            __RPC__in IInkPicture * This,
            /* [in] */ long newEraserWidth);
        
        DECLSPEC_XFGVIRT(IInkPicture, putref_Picture)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Picture )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IPictureDisp *pPicture);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_Picture)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Picture )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IPictureDisp *pPicture);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Picture)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Picture )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IPictureDisp **ppPicture);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_SizeMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SizeMode )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkPictureSizeMode smNewSizeMode);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_SizeMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SizeMode )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out InkPictureSizeMode *smSizeMode);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_BackColor)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_BackColor )( 
            __RPC__in IInkPicture * This,
            /* [in] */ OLE_COLOR newColor);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_BackColor)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_BackColor )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out OLE_COLOR *pColor);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Cursors)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Cursors )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkCursors **Cursors);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_MarginX)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MarginX )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out long *MarginX);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_MarginX)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MarginX )( 
            __RPC__in IInkPicture * This,
            /* [in] */ long MarginX);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_MarginY)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_MarginY )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out long *MarginY);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_MarginY)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_MarginY )( 
            __RPC__in IInkPicture * This,
            /* [in] */ long MarginY);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Tablet)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Tablet )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkTablet **SingleTablet);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_SupportHighContrastInk)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportHighContrastInk )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_SupportHighContrastInk)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SupportHighContrastInk )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT_BOOL Support);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_SupportHighContrastSelectionUI)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportHighContrastSelectionUI )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Support);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_SupportHighContrastSelectionUI)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SupportHighContrastSelectionUI )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT_BOOL Support);
        
        DECLSPEC_XFGVIRT(IInkPicture, HitTestSelection)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *HitTestSelection )( 
            __RPC__in IInkPicture * This,
            /* [in] */ long x,
            /* [in] */ long y,
            /* [retval][out] */ __RPC__out SelectionHitResult *SelArea);
        
        DECLSPEC_XFGVIRT(IInkPicture, SetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetGestureStatus )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [in] */ VARIANT_BOOL Listen);
        
        DECLSPEC_XFGVIRT(IInkPicture, GetGestureStatus)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetGestureStatus )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkApplicationGesture Gesture,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listening);
        
        DECLSPEC_XFGVIRT(IInkPicture, GetWindowInputRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetWindowInputRectangle )( 
            __RPC__in IInkPicture * This,
            /* [out][in] */ __RPC__deref_inout_opt IInkRectangle **WindowInputRectangle);
        
        DECLSPEC_XFGVIRT(IInkPicture, SetWindowInputRectangle)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetWindowInputRectangle )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IInkRectangle *WindowInputRectangle);
        
        DECLSPEC_XFGVIRT(IInkPicture, SetAllTabletsMode)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetAllTabletsMode )( 
            __RPC__in IInkPicture * This,
            /* [defaultvalue][in] */ VARIANT_BOOL UseMouseForInput);
        
        DECLSPEC_XFGVIRT(IInkPicture, SetSingleTabletIntegratedMode)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetSingleTabletIntegratedMode )( 
            __RPC__in IInkPicture * This,
            /* [in] */ __RPC__in_opt IInkTablet *Tablet);
        
        DECLSPEC_XFGVIRT(IInkPicture, GetEventInterest)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetEventInterest )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkCollectorEventInterest EventId,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Listen);
        
        DECLSPEC_XFGVIRT(IInkPicture, SetEventInterest)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetEventInterest )( 
            __RPC__in IInkPicture * This,
            /* [in] */ InkCollectorEventInterest EventId,
            /* [in] */ VARIANT_BOOL Listen);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_InkEnabled)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_InkEnabled )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Collecting);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_InkEnabled)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_InkEnabled )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT_BOOL Collecting);
        
        DECLSPEC_XFGVIRT(IInkPicture, get_Enabled)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Enabled )( 
            __RPC__in IInkPicture * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pbool);
        
        DECLSPEC_XFGVIRT(IInkPicture, put_Enabled)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Enabled )( 
            __RPC__in IInkPicture * This,
            /* [in] */ VARIANT_BOOL vbool);
        
        END_INTERFACE
    } IInkPictureVtbl;

    interface IInkPicture
    {
        CONST_VTBL struct IInkPictureVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkPicture_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkPicture_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkPicture_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkPicture_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkPicture_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkPicture_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkPicture_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkPicture_get_hWnd(This,CurrentWindow)	\
    ( (This)->lpVtbl -> get_hWnd(This,CurrentWindow) ) 

#define IInkPicture_get_DefaultDrawingAttributes(This,CurrentAttributes)	\
    ( (This)->lpVtbl -> get_DefaultDrawingAttributes(This,CurrentAttributes) ) 

#define IInkPicture_putref_DefaultDrawingAttributes(This,NewAttributes)	\
    ( (This)->lpVtbl -> putref_DefaultDrawingAttributes(This,NewAttributes) ) 

#define IInkPicture_get_Renderer(This,CurrentInkRenderer)	\
    ( (This)->lpVtbl -> get_Renderer(This,CurrentInkRenderer) ) 

#define IInkPicture_putref_Renderer(This,NewInkRenderer)	\
    ( (This)->lpVtbl -> putref_Renderer(This,NewInkRenderer) ) 

#define IInkPicture_get_Ink(This,Ink)	\
    ( (This)->lpVtbl -> get_Ink(This,Ink) ) 

#define IInkPicture_putref_Ink(This,NewInk)	\
    ( (This)->lpVtbl -> putref_Ink(This,NewInk) ) 

#define IInkPicture_get_AutoRedraw(This,AutoRedraw)	\
    ( (This)->lpVtbl -> get_AutoRedraw(This,AutoRedraw) ) 

#define IInkPicture_put_AutoRedraw(This,AutoRedraw)	\
    ( (This)->lpVtbl -> put_AutoRedraw(This,AutoRedraw) ) 

#define IInkPicture_get_CollectingInk(This,Collecting)	\
    ( (This)->lpVtbl -> get_CollectingInk(This,Collecting) ) 

#define IInkPicture_get_CollectionMode(This,Mode)	\
    ( (This)->lpVtbl -> get_CollectionMode(This,Mode) ) 

#define IInkPicture_put_CollectionMode(This,Mode)	\
    ( (This)->lpVtbl -> put_CollectionMode(This,Mode) ) 

#define IInkPicture_get_DynamicRendering(This,Enabled)	\
    ( (This)->lpVtbl -> get_DynamicRendering(This,Enabled) ) 

#define IInkPicture_put_DynamicRendering(This,Enabled)	\
    ( (This)->lpVtbl -> put_DynamicRendering(This,Enabled) ) 

#define IInkPicture_get_DesiredPacketDescription(This,PacketGuids)	\
    ( (This)->lpVtbl -> get_DesiredPacketDescription(This,PacketGuids) ) 

#define IInkPicture_put_DesiredPacketDescription(This,PacketGuids)	\
    ( (This)->lpVtbl -> put_DesiredPacketDescription(This,PacketGuids) ) 

#define IInkPicture_get_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> get_MouseIcon(This,MouseIcon) ) 

#define IInkPicture_put_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> put_MouseIcon(This,MouseIcon) ) 

#define IInkPicture_putref_MouseIcon(This,MouseIcon)	\
    ( (This)->lpVtbl -> putref_MouseIcon(This,MouseIcon) ) 

#define IInkPicture_get_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> get_MousePointer(This,MousePointer) ) 

#define IInkPicture_put_MousePointer(This,MousePointer)	\
    ( (This)->lpVtbl -> put_MousePointer(This,MousePointer) ) 

#define IInkPicture_get_EditingMode(This,EditingMode)	\
    ( (This)->lpVtbl -> get_EditingMode(This,EditingMode) ) 

#define IInkPicture_put_EditingMode(This,EditingMode)	\
    ( (This)->lpVtbl -> put_EditingMode(This,EditingMode) ) 

#define IInkPicture_get_Selection(This,Selection)	\
    ( (This)->lpVtbl -> get_Selection(This,Selection) ) 

#define IInkPicture_put_Selection(This,Selection)	\
    ( (This)->lpVtbl -> put_Selection(This,Selection) ) 

#define IInkPicture_get_EraserMode(This,EraserMode)	\
    ( (This)->lpVtbl -> get_EraserMode(This,EraserMode) ) 

#define IInkPicture_put_EraserMode(This,EraserMode)	\
    ( (This)->lpVtbl -> put_EraserMode(This,EraserMode) ) 

#define IInkPicture_get_EraserWidth(This,EraserWidth)	\
    ( (This)->lpVtbl -> get_EraserWidth(This,EraserWidth) ) 

#define IInkPicture_put_EraserWidth(This,newEraserWidth)	\
    ( (This)->lpVtbl -> put_EraserWidth(This,newEraserWidth) ) 

#define IInkPicture_putref_Picture(This,pPicture)	\
    ( (This)->lpVtbl -> putref_Picture(This,pPicture) ) 

#define IInkPicture_put_Picture(This,pPicture)	\
    ( (This)->lpVtbl -> put_Picture(This,pPicture) ) 

#define IInkPicture_get_Picture(This,ppPicture)	\
    ( (This)->lpVtbl -> get_Picture(This,ppPicture) ) 

#define IInkPicture_put_SizeMode(This,smNewSizeMode)	\
    ( (This)->lpVtbl -> put_SizeMode(This,smNewSizeMode) ) 

#define IInkPicture_get_SizeMode(This,smSizeMode)	\
    ( (This)->lpVtbl -> get_SizeMode(This,smSizeMode) ) 

#define IInkPicture_put_BackColor(This,newColor)	\
    ( (This)->lpVtbl -> put_BackColor(This,newColor) ) 

#define IInkPicture_get_BackColor(This,pColor)	\
    ( (This)->lpVtbl -> get_BackColor(This,pColor) ) 

#define IInkPicture_get_Cursors(This,Cursors)	\
    ( (This)->lpVtbl -> get_Cursors(This,Cursors) ) 

#define IInkPicture_get_MarginX(This,MarginX)	\
    ( (This)->lpVtbl -> get_MarginX(This,MarginX) ) 

#define IInkPicture_put_MarginX(This,MarginX)	\
    ( (This)->lpVtbl -> put_MarginX(This,MarginX) ) 

#define IInkPicture_get_MarginY(This,MarginY)	\
    ( (This)->lpVtbl -> get_MarginY(This,MarginY) ) 

#define IInkPicture_put_MarginY(This,MarginY)	\
    ( (This)->lpVtbl -> put_MarginY(This,MarginY) ) 

#define IInkPicture_get_Tablet(This,SingleTablet)	\
    ( (This)->lpVtbl -> get_Tablet(This,SingleTablet) ) 

#define IInkPicture_get_SupportHighContrastInk(This,Support)	\
    ( (This)->lpVtbl -> get_SupportHighContrastInk(This,Support) ) 

#define IInkPicture_put_SupportHighContrastInk(This,Support)	\
    ( (This)->lpVtbl -> put_SupportHighContrastInk(This,Support) ) 

#define IInkPicture_get_SupportHighContrastSelectionUI(This,Support)	\
    ( (This)->lpVtbl -> get_SupportHighContrastSelectionUI(This,Support) ) 

#define IInkPicture_put_SupportHighContrastSelectionUI(This,Support)	\
    ( (This)->lpVtbl -> put_SupportHighContrastSelectionUI(This,Support) ) 

#define IInkPicture_HitTestSelection(This,x,y,SelArea)	\
    ( (This)->lpVtbl -> HitTestSelection(This,x,y,SelArea) ) 

#define IInkPicture_SetGestureStatus(This,Gesture,Listen)	\
    ( (This)->lpVtbl -> SetGestureStatus(This,Gesture,Listen) ) 

#define IInkPicture_GetGestureStatus(This,Gesture,Listening)	\
    ( (This)->lpVtbl -> GetGestureStatus(This,Gesture,Listening) ) 

#define IInkPicture_GetWindowInputRectangle(This,WindowInputRectangle)	\
    ( (This)->lpVtbl -> GetWindowInputRectangle(This,WindowInputRectangle) ) 

#define IInkPicture_SetWindowInputRectangle(This,WindowInputRectangle)	\
    ( (This)->lpVtbl -> SetWindowInputRectangle(This,WindowInputRectangle) ) 

#define IInkPicture_SetAllTabletsMode(This,UseMouseForInput)	\
    ( (This)->lpVtbl -> SetAllTabletsMode(This,UseMouseForInput) ) 

#define IInkPicture_SetSingleTabletIntegratedMode(This,Tablet)	\
    ( (This)->lpVtbl -> SetSingleTabletIntegratedMode(This,Tablet) ) 

#define IInkPicture_GetEventInterest(This,EventId,Listen)	\
    ( (This)->lpVtbl -> GetEventInterest(This,EventId,Listen) ) 

#define IInkPicture_SetEventInterest(This,EventId,Listen)	\
    ( (This)->lpVtbl -> SetEventInterest(This,EventId,Listen) ) 

#define IInkPicture_get_InkEnabled(This,Collecting)	\
    ( (This)->lpVtbl -> get_InkEnabled(This,Collecting) ) 

#define IInkPicture_put_InkEnabled(This,Collecting)	\
    ( (This)->lpVtbl -> put_InkEnabled(This,Collecting) ) 

#define IInkPicture_get_Enabled(This,pbool)	\
    ( (This)->lpVtbl -> get_Enabled(This,pbool) ) 

#define IInkPicture_put_Enabled(This,vbool)	\
    ( (This)->lpVtbl -> put_Enabled(This,vbool) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkPicture_INTERFACE_DEFINED__ */


#ifndef ___IInkPictureEvents_DISPINTERFACE_DEFINED__
#define ___IInkPictureEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkPictureEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkPictureEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("60ff4fee-22ff-4484-acc1-d308d9cd7ea3")
    _IInkPictureEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkPictureEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkPictureEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkPictureEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkPictureEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkPictureEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkPictureEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkPictureEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkPictureEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkPictureEventsVtbl;

    interface _IInkPictureEvents
    {
        CONST_VTBL struct _IInkPictureEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkPictureEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkPictureEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkPictureEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkPictureEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkPictureEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkPictureEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkPictureEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkPictureEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IInkRecognizer_INTERFACE_DEFINED__
#define __IInkRecognizer_INTERFACE_DEFINED__

/* interface IInkRecognizer */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognizer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("782BF7CF-034B-4396-8A32-3A1833CF6B56")
    IInkRecognizer : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Vendor( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Vendor) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Capabilities( 
            /* [retval][out] */ __RPC__out InkRecognizerCapabilities *CapabilitiesFlags) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Languages( 
            /* [retval][out] */ __RPC__out VARIANT *Languages) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SupportedProperties( 
            /* [retval][out] */ __RPC__out VARIANT *SupportedProperties) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PreferredPacketDescription( 
            /* [retval][out] */ __RPC__out VARIANT *PreferredPacketDescription) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE CreateRecognizerContext( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerContext **Context) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognizerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognizer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognizer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognizer * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognizer * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognizer * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognizer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognizer * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, get_Name)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Name);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, get_Vendor)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Vendor )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Vendor);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, get_Capabilities)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Capabilities )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__out InkRecognizerCapabilities *CapabilitiesFlags);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, get_Languages)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Languages )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__out VARIANT *Languages);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, get_SupportedProperties)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedProperties )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__out VARIANT *SupportedProperties);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, get_PreferredPacketDescription)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PreferredPacketDescription )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__out VARIANT *PreferredPacketDescription);
        
        DECLSPEC_XFGVIRT(IInkRecognizer, CreateRecognizerContext)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *CreateRecognizerContext )( 
            __RPC__in IInkRecognizer * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerContext **Context);
        
        END_INTERFACE
    } IInkRecognizerVtbl;

    interface IInkRecognizer
    {
        CONST_VTBL struct IInkRecognizerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognizer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognizer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognizer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognizer_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognizer_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognizer_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognizer_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognizer_get_Name(This,Name)	\
    ( (This)->lpVtbl -> get_Name(This,Name) ) 

#define IInkRecognizer_get_Vendor(This,Vendor)	\
    ( (This)->lpVtbl -> get_Vendor(This,Vendor) ) 

#define IInkRecognizer_get_Capabilities(This,CapabilitiesFlags)	\
    ( (This)->lpVtbl -> get_Capabilities(This,CapabilitiesFlags) ) 

#define IInkRecognizer_get_Languages(This,Languages)	\
    ( (This)->lpVtbl -> get_Languages(This,Languages) ) 

#define IInkRecognizer_get_SupportedProperties(This,SupportedProperties)	\
    ( (This)->lpVtbl -> get_SupportedProperties(This,SupportedProperties) ) 

#define IInkRecognizer_get_PreferredPacketDescription(This,PreferredPacketDescription)	\
    ( (This)->lpVtbl -> get_PreferredPacketDescription(This,PreferredPacketDescription) ) 

#define IInkRecognizer_CreateRecognizerContext(This,Context)	\
    ( (This)->lpVtbl -> CreateRecognizerContext(This,Context) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognizer_INTERFACE_DEFINED__ */


#ifndef __IInkRecognizer2_INTERFACE_DEFINED__
#define __IInkRecognizer2_INTERFACE_DEFINED__

/* interface IInkRecognizer2 */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognizer2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6110118A-3A75-4ad6-B2AA-04B2B72BBE65")
    IInkRecognizer2 : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Id( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_UnicodeRanges( 
            /* [retval][out] */ __RPC__out VARIANT *UnicodeRanges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognizer2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognizer2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognizer2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognizer2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognizer2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognizer2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognizer2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognizer2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognizer2, get_Id)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Id )( 
            __RPC__in IInkRecognizer2 * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pbstrId);
        
        DECLSPEC_XFGVIRT(IInkRecognizer2, get_UnicodeRanges)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_UnicodeRanges )( 
            __RPC__in IInkRecognizer2 * This,
            /* [retval][out] */ __RPC__out VARIANT *UnicodeRanges);
        
        END_INTERFACE
    } IInkRecognizer2Vtbl;

    interface IInkRecognizer2
    {
        CONST_VTBL struct IInkRecognizer2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognizer2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognizer2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognizer2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognizer2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognizer2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognizer2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognizer2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognizer2_get_Id(This,pbstrId)	\
    ( (This)->lpVtbl -> get_Id(This,pbstrId) ) 

#define IInkRecognizer2_get_UnicodeRanges(This,UnicodeRanges)	\
    ( (This)->lpVtbl -> get_UnicodeRanges(This,UnicodeRanges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognizer2_INTERFACE_DEFINED__ */


#ifndef __IInkRecognizers_INTERFACE_DEFINED__
#define __IInkRecognizers_INTERFACE_DEFINED__

/* interface IInkRecognizers */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognizers;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9CCC4F12-B0B7-4a8b-BF58-4AECA4E8CEFD")
    IInkRecognizers : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetDefaultRecognizer( 
            /* [defaultvalue][in] */ long lcid,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **DefaultRecognizer) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **InkRecognizer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognizersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognizers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognizers * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognizers * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognizers * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognizers * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognizers * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognizers * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognizers, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkRecognizers * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkRecognizers, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkRecognizers * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkRecognizers, GetDefaultRecognizer)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetDefaultRecognizer )( 
            __RPC__in IInkRecognizers * This,
            /* [defaultvalue][in] */ long lcid,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **DefaultRecognizer);
        
        DECLSPEC_XFGVIRT(IInkRecognizers, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkRecognizers * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **InkRecognizer);
        
        END_INTERFACE
    } IInkRecognizersVtbl;

    interface IInkRecognizers
    {
        CONST_VTBL struct IInkRecognizersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognizers_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognizers_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognizers_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognizers_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognizers_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognizers_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognizers_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognizers_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkRecognizers_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkRecognizers_GetDefaultRecognizer(This,lcid,DefaultRecognizer)	\
    ( (This)->lpVtbl -> GetDefaultRecognizer(This,lcid,DefaultRecognizer) ) 

#define IInkRecognizers_Item(This,Index,InkRecognizer)	\
    ( (This)->lpVtbl -> Item(This,Index,InkRecognizer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognizers_INTERFACE_DEFINED__ */


#ifndef ___IInkRecognitionEvents_DISPINTERFACE_DEFINED__
#define ___IInkRecognitionEvents_DISPINTERFACE_DEFINED__

/* dispinterface _IInkRecognitionEvents */
/* [helpcontext][helpstring][uuid] */ 


EXTERN_C const IID DIID__IInkRecognitionEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)

    MIDL_INTERFACE("17BCE92F-2E21-47FD-9D33-3C6AFBFD8C59")
    _IInkRecognitionEvents : public IDispatch
    {
    };
    
#else 	/* C style interface */

    typedef struct _IInkRecognitionEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in _IInkRecognitionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in _IInkRecognitionEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in _IInkRecognitionEvents * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in _IInkRecognitionEvents * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in _IInkRecognitionEvents * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in _IInkRecognitionEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            _IInkRecognitionEvents * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } _IInkRecognitionEventsVtbl;

    interface _IInkRecognitionEvents
    {
        CONST_VTBL struct _IInkRecognitionEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define _IInkRecognitionEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define _IInkRecognitionEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define _IInkRecognitionEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define _IInkRecognitionEvents_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define _IInkRecognitionEvents_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define _IInkRecognitionEvents_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define _IInkRecognitionEvents_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */


#endif 	/* ___IInkRecognitionEvents_DISPINTERFACE_DEFINED__ */


#ifndef __IInkRecognizerContext_INTERFACE_DEFINED__
#define __IInkRecognizerContext_INTERFACE_DEFINED__

/* interface IInkRecognizerContext */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognizerContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C68F52F9-32A3-4625-906C-44FC23B40958")
    IInkRecognizerContext : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Strokes( 
            /* [unique][in] */ __RPC__in_opt IInkStrokes *Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_CharacterAutoCompletionMode( 
            /* [retval][out] */ __RPC__out InkRecognizerCharacterAutoCompletionMode *Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_CharacterAutoCompletionMode( 
            /* [in] */ InkRecognizerCharacterAutoCompletionMode Mode) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Factoid( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Factoid) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Factoid( 
            /* [in] */ __RPC__in BSTR factoid) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Guide( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerGuide **RecognizerGuide) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_Guide( 
            /* [in] */ __RPC__in_opt IInkRecognizerGuide *RecognizerGuide) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_PrefixText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Prefix) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_PrefixText( 
            /* [in] */ __RPC__in BSTR Prefix) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_SuffixText( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Suffix) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_SuffixText( 
            /* [in] */ __RPC__in BSTR Suffix) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_RecognitionFlags( 
            /* [retval][out] */ __RPC__out InkRecognitionModes *Modes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_RecognitionFlags( 
            /* [in] */ InkRecognitionModes Modes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WordList( 
            /* [retval][out] */ __RPC__deref_out_opt IInkWordList **WordList) = 0;
        
        virtual /* [helpcontext][helpstring][id][propputref] */ HRESULT STDMETHODCALLTYPE putref_WordList( 
            /* [in] */ __RPC__in_opt IInkWordList *WordList) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Recognizer( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **Recognizer) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Recognize( 
            /* [out][in] */ __RPC__inout InkRecognitionStatus *RecognitionStatus,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionResult **RecognitionResult) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE StopBackgroundRecognition( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE EndInkInput( void) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE BackgroundRecognize( 
            /* [optional][in] */ VARIANT CustomData) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE BackgroundRecognizeWithAlternates( 
            /* [optional][in] */ VARIANT CustomData) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerContext **RecoContext) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE IsStringSupported( 
            /* [in] */ __RPC__in BSTR String,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Supported) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognizerContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognizerContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognizerContext * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognizerContext * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognizerContext * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, putref_Strokes)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Strokes )( 
            __RPC__in IInkRecognizerContext * This,
            /* [unique][in] */ __RPC__in_opt IInkStrokes *Strokes);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_CharacterAutoCompletionMode)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_CharacterAutoCompletionMode )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__out InkRecognizerCharacterAutoCompletionMode *Mode);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, put_CharacterAutoCompletionMode)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_CharacterAutoCompletionMode )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ InkRecognizerCharacterAutoCompletionMode Mode);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_Factoid)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Factoid )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Factoid);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, put_Factoid)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Factoid )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in BSTR factoid);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_Guide)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Guide )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerGuide **RecognizerGuide);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, putref_Guide)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_Guide )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in_opt IInkRecognizerGuide *RecognizerGuide);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_PrefixText)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_PrefixText )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Prefix);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, put_PrefixText)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_PrefixText )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in BSTR Prefix);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_SuffixText)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_SuffixText )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *Suffix);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, put_SuffixText)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_SuffixText )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in BSTR Suffix);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_RecognitionFlags)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_RecognitionFlags )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__out InkRecognitionModes *Modes);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, put_RecognitionFlags)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_RecognitionFlags )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ InkRecognitionModes Modes);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_WordList)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WordList )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkWordList **WordList);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, putref_WordList)
        /* [helpcontext][helpstring][id][propputref] */ HRESULT ( STDMETHODCALLTYPE *putref_WordList )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in_opt IInkWordList *WordList);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, get_Recognizer)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Recognizer )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizer **Recognizer);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, Recognize)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Recognize )( 
            __RPC__in IInkRecognizerContext * This,
            /* [out][in] */ __RPC__inout InkRecognitionStatus *RecognitionStatus,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionResult **RecognitionResult);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, StopBackgroundRecognition)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *StopBackgroundRecognition )( 
            __RPC__in IInkRecognizerContext * This);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, EndInkInput)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *EndInkInput )( 
            __RPC__in IInkRecognizerContext * This);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, BackgroundRecognize)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BackgroundRecognize )( 
            __RPC__in IInkRecognizerContext * This,
            /* [optional][in] */ VARIANT CustomData);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, BackgroundRecognizeWithAlternates)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *BackgroundRecognizeWithAlternates )( 
            __RPC__in IInkRecognizerContext * This,
            /* [optional][in] */ VARIANT CustomData);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, Clone)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IInkRecognizerContext * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognizerContext **RecoContext);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext, IsStringSupported)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *IsStringSupported )( 
            __RPC__in IInkRecognizerContext * This,
            /* [in] */ __RPC__in BSTR String,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *Supported);
        
        END_INTERFACE
    } IInkRecognizerContextVtbl;

    interface IInkRecognizerContext
    {
        CONST_VTBL struct IInkRecognizerContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognizerContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognizerContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognizerContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognizerContext_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognizerContext_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognizerContext_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognizerContext_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognizerContext_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkRecognizerContext_putref_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> putref_Strokes(This,Strokes) ) 

#define IInkRecognizerContext_get_CharacterAutoCompletionMode(This,Mode)	\
    ( (This)->lpVtbl -> get_CharacterAutoCompletionMode(This,Mode) ) 

#define IInkRecognizerContext_put_CharacterAutoCompletionMode(This,Mode)	\
    ( (This)->lpVtbl -> put_CharacterAutoCompletionMode(This,Mode) ) 

#define IInkRecognizerContext_get_Factoid(This,Factoid)	\
    ( (This)->lpVtbl -> get_Factoid(This,Factoid) ) 

#define IInkRecognizerContext_put_Factoid(This,factoid)	\
    ( (This)->lpVtbl -> put_Factoid(This,factoid) ) 

#define IInkRecognizerContext_get_Guide(This,RecognizerGuide)	\
    ( (This)->lpVtbl -> get_Guide(This,RecognizerGuide) ) 

#define IInkRecognizerContext_putref_Guide(This,RecognizerGuide)	\
    ( (This)->lpVtbl -> putref_Guide(This,RecognizerGuide) ) 

#define IInkRecognizerContext_get_PrefixText(This,Prefix)	\
    ( (This)->lpVtbl -> get_PrefixText(This,Prefix) ) 

#define IInkRecognizerContext_put_PrefixText(This,Prefix)	\
    ( (This)->lpVtbl -> put_PrefixText(This,Prefix) ) 

#define IInkRecognizerContext_get_SuffixText(This,Suffix)	\
    ( (This)->lpVtbl -> get_SuffixText(This,Suffix) ) 

#define IInkRecognizerContext_put_SuffixText(This,Suffix)	\
    ( (This)->lpVtbl -> put_SuffixText(This,Suffix) ) 

#define IInkRecognizerContext_get_RecognitionFlags(This,Modes)	\
    ( (This)->lpVtbl -> get_RecognitionFlags(This,Modes) ) 

#define IInkRecognizerContext_put_RecognitionFlags(This,Modes)	\
    ( (This)->lpVtbl -> put_RecognitionFlags(This,Modes) ) 

#define IInkRecognizerContext_get_WordList(This,WordList)	\
    ( (This)->lpVtbl -> get_WordList(This,WordList) ) 

#define IInkRecognizerContext_putref_WordList(This,WordList)	\
    ( (This)->lpVtbl -> putref_WordList(This,WordList) ) 

#define IInkRecognizerContext_get_Recognizer(This,Recognizer)	\
    ( (This)->lpVtbl -> get_Recognizer(This,Recognizer) ) 

#define IInkRecognizerContext_Recognize(This,RecognitionStatus,RecognitionResult)	\
    ( (This)->lpVtbl -> Recognize(This,RecognitionStatus,RecognitionResult) ) 

#define IInkRecognizerContext_StopBackgroundRecognition(This)	\
    ( (This)->lpVtbl -> StopBackgroundRecognition(This) ) 

#define IInkRecognizerContext_EndInkInput(This)	\
    ( (This)->lpVtbl -> EndInkInput(This) ) 

#define IInkRecognizerContext_BackgroundRecognize(This,CustomData)	\
    ( (This)->lpVtbl -> BackgroundRecognize(This,CustomData) ) 

#define IInkRecognizerContext_BackgroundRecognizeWithAlternates(This,CustomData)	\
    ( (This)->lpVtbl -> BackgroundRecognizeWithAlternates(This,CustomData) ) 

#define IInkRecognizerContext_Clone(This,RecoContext)	\
    ( (This)->lpVtbl -> Clone(This,RecoContext) ) 

#define IInkRecognizerContext_IsStringSupported(This,String,Supported)	\
    ( (This)->lpVtbl -> IsStringSupported(This,String,Supported) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognizerContext_INTERFACE_DEFINED__ */


#ifndef __IInkRecognizerContext2_INTERFACE_DEFINED__
#define __IInkRecognizerContext2_INTERFACE_DEFINED__

/* interface IInkRecognizerContext2 */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognizerContext2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D6F0E32F-73D8-408e-8E9F-5FEA592C363F")
    IInkRecognizerContext2 : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_EnabledUnicodeRanges( 
            /* [retval][out] */ __RPC__out VARIANT *UnicodeRanges) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_EnabledUnicodeRanges( 
            /* [in] */ VARIANT UnicodeRanges) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognizerContext2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognizerContext2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognizerContext2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognizerContext2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognizerContext2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognizerContext2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognizerContext2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognizerContext2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext2, get_EnabledUnicodeRanges)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_EnabledUnicodeRanges )( 
            __RPC__in IInkRecognizerContext2 * This,
            /* [retval][out] */ __RPC__out VARIANT *UnicodeRanges);
        
        DECLSPEC_XFGVIRT(IInkRecognizerContext2, put_EnabledUnicodeRanges)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_EnabledUnicodeRanges )( 
            __RPC__in IInkRecognizerContext2 * This,
            /* [in] */ VARIANT UnicodeRanges);
        
        END_INTERFACE
    } IInkRecognizerContext2Vtbl;

    interface IInkRecognizerContext2
    {
        CONST_VTBL struct IInkRecognizerContext2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognizerContext2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognizerContext2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognizerContext2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognizerContext2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognizerContext2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognizerContext2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognizerContext2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognizerContext2_get_EnabledUnicodeRanges(This,UnicodeRanges)	\
    ( (This)->lpVtbl -> get_EnabledUnicodeRanges(This,UnicodeRanges) ) 

#define IInkRecognizerContext2_put_EnabledUnicodeRanges(This,UnicodeRanges)	\
    ( (This)->lpVtbl -> put_EnabledUnicodeRanges(This,UnicodeRanges) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognizerContext2_INTERFACE_DEFINED__ */


#ifndef __IInkRecognitionResult_INTERFACE_DEFINED__
#define __IInkRecognitionResult_INTERFACE_DEFINED__

/* interface IInkRecognitionResult */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognitionResult;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3BC129A8-86CD-45ad-BDE8-E0D32D61C16D")
    IInkRecognitionResult : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TopString( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *TopString) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TopAlternate( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternate **TopAlternate) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_TopConfidence( 
            /* [retval][out] */ __RPC__out InkRecognitionConfidence *TopConfidence) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AlternatesFromSelection( 
            /* [defaultvalue][in] */ long selectionStart,
            /* [defaultvalue][in] */ long selectionLength,
            /* [defaultvalue][in] */ long maximumAlternates,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **AlternatesFromSelection) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE ModifyTopAlternate( 
            /* [in] */ __RPC__in_opt IInkRecognitionAlternate *Alternate) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE SetResultOnStrokes( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognitionResultVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognitionResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognitionResult * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognitionResult * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognitionResult * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognitionResult * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognitionResult * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognitionResult * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, get_TopString)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TopString )( 
            __RPC__in IInkRecognitionResult * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *TopString);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, get_TopAlternate)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TopAlternate )( 
            __RPC__in IInkRecognitionResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternate **TopAlternate);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, get_TopConfidence)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_TopConfidence )( 
            __RPC__in IInkRecognitionResult * This,
            /* [retval][out] */ __RPC__out InkRecognitionConfidence *TopConfidence);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkRecognitionResult * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, AlternatesFromSelection)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AlternatesFromSelection )( 
            __RPC__in IInkRecognitionResult * This,
            /* [defaultvalue][in] */ long selectionStart,
            /* [defaultvalue][in] */ long selectionLength,
            /* [defaultvalue][in] */ long maximumAlternates,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **AlternatesFromSelection);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, ModifyTopAlternate)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *ModifyTopAlternate )( 
            __RPC__in IInkRecognitionResult * This,
            /* [in] */ __RPC__in_opt IInkRecognitionAlternate *Alternate);
        
        DECLSPEC_XFGVIRT(IInkRecognitionResult, SetResultOnStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *SetResultOnStrokes )( 
            __RPC__in IInkRecognitionResult * This);
        
        END_INTERFACE
    } IInkRecognitionResultVtbl;

    interface IInkRecognitionResult
    {
        CONST_VTBL struct IInkRecognitionResultVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognitionResult_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognitionResult_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognitionResult_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognitionResult_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognitionResult_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognitionResult_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognitionResult_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognitionResult_get_TopString(This,TopString)	\
    ( (This)->lpVtbl -> get_TopString(This,TopString) ) 

#define IInkRecognitionResult_get_TopAlternate(This,TopAlternate)	\
    ( (This)->lpVtbl -> get_TopAlternate(This,TopAlternate) ) 

#define IInkRecognitionResult_get_TopConfidence(This,TopConfidence)	\
    ( (This)->lpVtbl -> get_TopConfidence(This,TopConfidence) ) 

#define IInkRecognitionResult_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkRecognitionResult_AlternatesFromSelection(This,selectionStart,selectionLength,maximumAlternates,AlternatesFromSelection)	\
    ( (This)->lpVtbl -> AlternatesFromSelection(This,selectionStart,selectionLength,maximumAlternates,AlternatesFromSelection) ) 

#define IInkRecognitionResult_ModifyTopAlternate(This,Alternate)	\
    ( (This)->lpVtbl -> ModifyTopAlternate(This,Alternate) ) 

#define IInkRecognitionResult_SetResultOnStrokes(This)	\
    ( (This)->lpVtbl -> SetResultOnStrokes(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognitionResult_INTERFACE_DEFINED__ */


#ifndef __IInkRecognitionAlternate_INTERFACE_DEFINED__
#define __IInkRecognitionAlternate_INTERFACE_DEFINED__

/* interface IInkRecognitionAlternate */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognitionAlternate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B7E660AD-77E4-429b-ADDA-873780D1FC4A")
    IInkRecognitionAlternate : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_String( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *RecoString) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Confidence( 
            /* [retval][out] */ __RPC__out InkRecognitionConfidence *Confidence) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Baseline( 
            /* [retval][out] */ __RPC__out VARIANT *Baseline) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Midline( 
            /* [retval][out] */ __RPC__out VARIANT *Midline) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Ascender( 
            /* [retval][out] */ __RPC__out VARIANT *Ascender) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Descender( 
            /* [retval][out] */ __RPC__out VARIANT *Descender) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LineNumber( 
            /* [retval][out] */ __RPC__out long *LineNumber) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_LineAlternates( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **LineAlternates) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_ConfidenceAlternates( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **ConfidenceAlternates) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStrokesFromStrokeRanges( 
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **GetStrokesFromStrokeRanges) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetStrokesFromTextRange( 
            /* [out][in] */ __RPC__inout long *selectionStart,
            /* [out][in] */ __RPC__inout long *selectionLength,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **GetStrokesFromTextRange) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetTextRangeFromStrokes( 
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [out][in] */ __RPC__inout long *selectionStart,
            /* [out][in] */ __RPC__inout long *selectionLength) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AlternatesWithConstantPropertyValues( 
            /* [in] */ __RPC__in BSTR PropertyType,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **AlternatesWithConstantPropertyValues) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE GetPropertyValue( 
            /* [in] */ __RPC__in BSTR PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *PropertyValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognitionAlternateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognitionAlternate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognitionAlternate * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognitionAlternate * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_String)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_String )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *RecoString);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_Confidence)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Confidence )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__out InkRecognitionConfidence *Confidence);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_Baseline)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Baseline )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__out VARIANT *Baseline);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_Midline)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Midline )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__out VARIANT *Midline);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_Ascender)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Ascender )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__out VARIANT *Ascender);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_Descender)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Descender )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__out VARIANT *Descender);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_LineNumber)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LineNumber )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__out long *LineNumber);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_LineAlternates)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_LineAlternates )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **LineAlternates);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, get_ConfidenceAlternates)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_ConfidenceAlternates )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **ConfidenceAlternates);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, GetStrokesFromStrokeRanges)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStrokesFromStrokeRanges )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **GetStrokesFromStrokeRanges);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, GetStrokesFromTextRange)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetStrokesFromTextRange )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [out][in] */ __RPC__inout long *selectionStart,
            /* [out][in] */ __RPC__inout long *selectionLength,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **GetStrokesFromTextRange);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, GetTextRangeFromStrokes)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetTextRangeFromStrokes )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ __RPC__in_opt IInkStrokes *Strokes,
            /* [out][in] */ __RPC__inout long *selectionStart,
            /* [out][in] */ __RPC__inout long *selectionLength);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, AlternatesWithConstantPropertyValues)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AlternatesWithConstantPropertyValues )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ __RPC__in BSTR PropertyType,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternates **AlternatesWithConstantPropertyValues);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternate, GetPropertyValue)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IInkRecognitionAlternate * This,
            /* [in] */ __RPC__in BSTR PropertyType,
            /* [retval][out] */ __RPC__out VARIANT *PropertyValue);
        
        END_INTERFACE
    } IInkRecognitionAlternateVtbl;

    interface IInkRecognitionAlternate
    {
        CONST_VTBL struct IInkRecognitionAlternateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognitionAlternate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognitionAlternate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognitionAlternate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognitionAlternate_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognitionAlternate_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognitionAlternate_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognitionAlternate_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognitionAlternate_get_String(This,RecoString)	\
    ( (This)->lpVtbl -> get_String(This,RecoString) ) 

#define IInkRecognitionAlternate_get_Confidence(This,Confidence)	\
    ( (This)->lpVtbl -> get_Confidence(This,Confidence) ) 

#define IInkRecognitionAlternate_get_Baseline(This,Baseline)	\
    ( (This)->lpVtbl -> get_Baseline(This,Baseline) ) 

#define IInkRecognitionAlternate_get_Midline(This,Midline)	\
    ( (This)->lpVtbl -> get_Midline(This,Midline) ) 

#define IInkRecognitionAlternate_get_Ascender(This,Ascender)	\
    ( (This)->lpVtbl -> get_Ascender(This,Ascender) ) 

#define IInkRecognitionAlternate_get_Descender(This,Descender)	\
    ( (This)->lpVtbl -> get_Descender(This,Descender) ) 

#define IInkRecognitionAlternate_get_LineNumber(This,LineNumber)	\
    ( (This)->lpVtbl -> get_LineNumber(This,LineNumber) ) 

#define IInkRecognitionAlternate_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkRecognitionAlternate_get_LineAlternates(This,LineAlternates)	\
    ( (This)->lpVtbl -> get_LineAlternates(This,LineAlternates) ) 

#define IInkRecognitionAlternate_get_ConfidenceAlternates(This,ConfidenceAlternates)	\
    ( (This)->lpVtbl -> get_ConfidenceAlternates(This,ConfidenceAlternates) ) 

#define IInkRecognitionAlternate_GetStrokesFromStrokeRanges(This,Strokes,GetStrokesFromStrokeRanges)	\
    ( (This)->lpVtbl -> GetStrokesFromStrokeRanges(This,Strokes,GetStrokesFromStrokeRanges) ) 

#define IInkRecognitionAlternate_GetStrokesFromTextRange(This,selectionStart,selectionLength,GetStrokesFromTextRange)	\
    ( (This)->lpVtbl -> GetStrokesFromTextRange(This,selectionStart,selectionLength,GetStrokesFromTextRange) ) 

#define IInkRecognitionAlternate_GetTextRangeFromStrokes(This,Strokes,selectionStart,selectionLength)	\
    ( (This)->lpVtbl -> GetTextRangeFromStrokes(This,Strokes,selectionStart,selectionLength) ) 

#define IInkRecognitionAlternate_AlternatesWithConstantPropertyValues(This,PropertyType,AlternatesWithConstantPropertyValues)	\
    ( (This)->lpVtbl -> AlternatesWithConstantPropertyValues(This,PropertyType,AlternatesWithConstantPropertyValues) ) 

#define IInkRecognitionAlternate_GetPropertyValue(This,PropertyType,PropertyValue)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,PropertyType,PropertyValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognitionAlternate_INTERFACE_DEFINED__ */


#ifndef __IInkRecognitionAlternates_INTERFACE_DEFINED__
#define __IInkRecognitionAlternates_INTERFACE_DEFINED__

/* interface IInkRecognitionAlternates */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognitionAlternates;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("286A167F-9F19-4c61-9D53-4F07BE622B84")
    IInkRecognitionAlternates : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out long *Count) = 0;
        
        virtual /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT STDMETHODCALLTYPE get__NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Strokes( 
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Item( 
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternate **InkRecoAlternate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognitionAlternatesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognitionAlternates * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognitionAlternates * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognitionAlternates * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternates, get_Count)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [retval][out] */ __RPC__out long *Count);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternates, get__NewEnum)
        /* [helpcontext][helpstring][restricted][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get__NewEnum )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **_NewEnum);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternates, get_Strokes)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Strokes )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkStrokes **Strokes);
        
        DECLSPEC_XFGVIRT(IInkRecognitionAlternates, Item)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IInkRecognitionAlternates * This,
            /* [in] */ long Index,
            /* [retval][out] */ __RPC__deref_out_opt IInkRecognitionAlternate **InkRecoAlternate);
        
        END_INTERFACE
    } IInkRecognitionAlternatesVtbl;

    interface IInkRecognitionAlternates
    {
        CONST_VTBL struct IInkRecognitionAlternatesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognitionAlternates_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognitionAlternates_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognitionAlternates_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognitionAlternates_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognitionAlternates_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognitionAlternates_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognitionAlternates_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognitionAlternates_get_Count(This,Count)	\
    ( (This)->lpVtbl -> get_Count(This,Count) ) 

#define IInkRecognitionAlternates_get__NewEnum(This,_NewEnum)	\
    ( (This)->lpVtbl -> get__NewEnum(This,_NewEnum) ) 

#define IInkRecognitionAlternates_get_Strokes(This,Strokes)	\
    ( (This)->lpVtbl -> get_Strokes(This,Strokes) ) 

#define IInkRecognitionAlternates_Item(This,Index,InkRecoAlternate)	\
    ( (This)->lpVtbl -> Item(This,Index,InkRecoAlternate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognitionAlternates_INTERFACE_DEFINED__ */


#ifndef __IInkRecognizerGuide_INTERFACE_DEFINED__
#define __IInkRecognizerGuide_INTERFACE_DEFINED__

/* interface IInkRecognizerGuide */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkRecognizerGuide;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D934BE07-7B84-4208-9136-83C20994E905")
    IInkRecognizerGuide : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_WritingBox( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_WritingBox( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_DrawnBox( 
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_DrawnBox( 
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Rows( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Rows( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Columns( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Columns( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propget] */ HRESULT STDMETHODCALLTYPE get_Midline( 
            /* [retval][out] */ __RPC__out long *Units) = 0;
        
        virtual /* [helpcontext][helpstring][id][propput] */ HRESULT STDMETHODCALLTYPE put_Midline( 
            /* [in] */ long Units) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT STDMETHODCALLTYPE get_GuideData( 
            /* [retval][out] */ __RPC__out InkRecoGuide *pRecoGuide) = 0;
        
        virtual /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT STDMETHODCALLTYPE put_GuideData( 
            /* [in] */ InkRecoGuide recoGuide) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkRecognizerGuideVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkRecognizerGuide * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkRecognizerGuide * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkRecognizerGuide * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, get_WritingBox)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_WritingBox )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, put_WritingBox)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_WritingBox )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, get_DrawnBox)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_DrawnBox )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [retval][out] */ __RPC__deref_out_opt IInkRectangle **Rectangle);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, put_DrawnBox)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_DrawnBox )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ __RPC__in_opt IInkRectangle *Rectangle);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, get_Rows)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Rows )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, put_Rows)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Rows )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, get_Columns)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Columns )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, put_Columns)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Columns )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, get_Midline)
        /* [helpcontext][helpstring][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_Midline )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [retval][out] */ __RPC__out long *Units);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, put_Midline)
        /* [helpcontext][helpstring][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_Midline )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ long Units);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, get_GuideData)
        /* [helpcontext][helpstring][hidden][id][propget] */ HRESULT ( STDMETHODCALLTYPE *get_GuideData )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [retval][out] */ __RPC__out InkRecoGuide *pRecoGuide);
        
        DECLSPEC_XFGVIRT(IInkRecognizerGuide, put_GuideData)
        /* [helpcontext][helpstring][hidden][id][propput] */ HRESULT ( STDMETHODCALLTYPE *put_GuideData )( 
            __RPC__in IInkRecognizerGuide * This,
            /* [in] */ InkRecoGuide recoGuide);
        
        END_INTERFACE
    } IInkRecognizerGuideVtbl;

    interface IInkRecognizerGuide
    {
        CONST_VTBL struct IInkRecognizerGuideVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkRecognizerGuide_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkRecognizerGuide_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkRecognizerGuide_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkRecognizerGuide_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkRecognizerGuide_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkRecognizerGuide_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkRecognizerGuide_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkRecognizerGuide_get_WritingBox(This,Rectangle)	\
    ( (This)->lpVtbl -> get_WritingBox(This,Rectangle) ) 

#define IInkRecognizerGuide_put_WritingBox(This,Rectangle)	\
    ( (This)->lpVtbl -> put_WritingBox(This,Rectangle) ) 

#define IInkRecognizerGuide_get_DrawnBox(This,Rectangle)	\
    ( (This)->lpVtbl -> get_DrawnBox(This,Rectangle) ) 

#define IInkRecognizerGuide_put_DrawnBox(This,Rectangle)	\
    ( (This)->lpVtbl -> put_DrawnBox(This,Rectangle) ) 

#define IInkRecognizerGuide_get_Rows(This,Units)	\
    ( (This)->lpVtbl -> get_Rows(This,Units) ) 

#define IInkRecognizerGuide_put_Rows(This,Units)	\
    ( (This)->lpVtbl -> put_Rows(This,Units) ) 

#define IInkRecognizerGuide_get_Columns(This,Units)	\
    ( (This)->lpVtbl -> get_Columns(This,Units) ) 

#define IInkRecognizerGuide_put_Columns(This,Units)	\
    ( (This)->lpVtbl -> put_Columns(This,Units) ) 

#define IInkRecognizerGuide_get_Midline(This,Units)	\
    ( (This)->lpVtbl -> get_Midline(This,Units) ) 

#define IInkRecognizerGuide_put_Midline(This,Units)	\
    ( (This)->lpVtbl -> put_Midline(This,Units) ) 

#define IInkRecognizerGuide_get_GuideData(This,pRecoGuide)	\
    ( (This)->lpVtbl -> get_GuideData(This,pRecoGuide) ) 

#define IInkRecognizerGuide_put_GuideData(This,recoGuide)	\
    ( (This)->lpVtbl -> put_GuideData(This,recoGuide) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkRecognizerGuide_INTERFACE_DEFINED__ */


#ifndef __IInkWordList_INTERFACE_DEFINED__
#define __IInkWordList_INTERFACE_DEFINED__

/* interface IInkWordList */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkWordList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76BA3491-CB2F-406b-9961-0E0C4CDAAEF2")
    IInkWordList : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddWord( 
            /* [in] */ __RPC__in BSTR NewWord) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE RemoveWord( 
            /* [in] */ __RPC__in BSTR RemoveWord) = 0;
        
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE Merge( 
            /* [in] */ __RPC__in_opt IInkWordList *MergeWordList) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkWordListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkWordList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkWordList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkWordList * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkWordList * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkWordList * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkWordList * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkWordList * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkWordList, AddWord)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddWord )( 
            __RPC__in IInkWordList * This,
            /* [in] */ __RPC__in BSTR NewWord);
        
        DECLSPEC_XFGVIRT(IInkWordList, RemoveWord)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *RemoveWord )( 
            __RPC__in IInkWordList * This,
            /* [in] */ __RPC__in BSTR RemoveWord);
        
        DECLSPEC_XFGVIRT(IInkWordList, Merge)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *Merge )( 
            __RPC__in IInkWordList * This,
            /* [in] */ __RPC__in_opt IInkWordList *MergeWordList);
        
        END_INTERFACE
    } IInkWordListVtbl;

    interface IInkWordList
    {
        CONST_VTBL struct IInkWordListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkWordList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkWordList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkWordList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkWordList_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkWordList_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkWordList_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkWordList_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkWordList_AddWord(This,NewWord)	\
    ( (This)->lpVtbl -> AddWord(This,NewWord) ) 

#define IInkWordList_RemoveWord(This,RemoveWord)	\
    ( (This)->lpVtbl -> RemoveWord(This,RemoveWord) ) 

#define IInkWordList_Merge(This,MergeWordList)	\
    ( (This)->lpVtbl -> Merge(This,MergeWordList) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkWordList_INTERFACE_DEFINED__ */


#ifndef __IInkWordList2_INTERFACE_DEFINED__
#define __IInkWordList2_INTERFACE_DEFINED__

/* interface IInkWordList2 */
/* [unique][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInkWordList2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("14542586-11BF-4f5f-B6E7-49D0744AAB6E")
    IInkWordList2 : public IDispatch
    {
    public:
        virtual /* [helpcontext][helpstring][id] */ HRESULT STDMETHODCALLTYPE AddWords( 
            /* [in] */ __RPC__in BSTR NewWords) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkWordList2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkWordList2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkWordList2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkWordList2 * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInkWordList2 * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInkWordList2 * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInkWordList2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInkWordList2 * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        DECLSPEC_XFGVIRT(IInkWordList2, AddWords)
        /* [helpcontext][helpstring][id] */ HRESULT ( STDMETHODCALLTYPE *AddWords )( 
            __RPC__in IInkWordList2 * This,
            /* [in] */ __RPC__in BSTR NewWords);
        
        END_INTERFACE
    } IInkWordList2Vtbl;

    interface IInkWordList2
    {
        CONST_VTBL struct IInkWordList2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkWordList2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkWordList2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkWordList2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkWordList2_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInkWordList2_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInkWordList2_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInkWordList2_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#define IInkWordList2_AddWords(This,NewWords)	\
    ( (This)->lpVtbl -> AddWords(This,NewWords) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkWordList2_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_InkDisp;

#ifdef __cplusplus

class DECLSPEC_UUID("937C1A34-151D-4610-9CA6-A8CC9BDB5D83")
InkDisp;
#endif

EXTERN_C const CLSID CLSID_InkOverlay;

#ifdef __cplusplus

class DECLSPEC_UUID("65d00646-cde3-4a88-9163-6769f0f1a97d")
InkOverlay;
#endif

EXTERN_C const CLSID CLSID_InkPicture;

#ifdef __cplusplus

class DECLSPEC_UUID("04a1e553-fe36-4fde-865e-344194e69424")
InkPicture;
#endif

EXTERN_C const CLSID CLSID_InkCollector;

#ifdef __cplusplus

class DECLSPEC_UUID("43FB1553-AD74-4ee8-88E4-3E6DAAC915DB")
InkCollector;
#endif

EXTERN_C const CLSID CLSID_InkDrawingAttributes;

#ifdef __cplusplus

class DECLSPEC_UUID("D8BF32A2-05A5-44c3-B3AA-5E80AC7D2576")
InkDrawingAttributes;
#endif

EXTERN_C const CLSID CLSID_InkRectangle;

#ifdef __cplusplus

class DECLSPEC_UUID("43B07326-AAE0-4B62-A83D-5FD768B7353C")
InkRectangle;
#endif

EXTERN_C const CLSID CLSID_InkRenderer;

#ifdef __cplusplus

class DECLSPEC_UUID("9C1CC6E4-D7EB-4eeb-9091-15A7C8791ED9")
InkRenderer;
#endif

EXTERN_C const CLSID CLSID_InkTransform;

#ifdef __cplusplus

class DECLSPEC_UUID("E3D5D93C-1663-4A78-A1A7-22375DFEBAEE")
InkTransform;
#endif

EXTERN_C const CLSID CLSID_InkRecognizers;

#ifdef __cplusplus

class DECLSPEC_UUID("9FD4E808-F6E6-4e65-98D3-AA39054C1255")
InkRecognizers;
#endif

EXTERN_C const CLSID CLSID_InkRecognizerContext;

#ifdef __cplusplus

class DECLSPEC_UUID("AAC46A37-9229-4fc0-8CCE-4497569BF4D1")
InkRecognizerContext;
#endif

EXTERN_C const CLSID CLSID_InkRecognizerGuide;

#ifdef __cplusplus

class DECLSPEC_UUID("8770D941-A63A-4671-A375-2855A18EBA73")
InkRecognizerGuide;
#endif

EXTERN_C const CLSID CLSID_InkTablets;

#ifdef __cplusplus

class DECLSPEC_UUID("6E4FCB12-510A-4d40-9304-1DA10AE9147C")
InkTablets;
#endif

EXTERN_C const CLSID CLSID_InkWordList;

#ifdef __cplusplus

class DECLSPEC_UUID("9DE85094-F71F-44f1-8471-15A2FA76FCF3")
InkWordList;
#endif

EXTERN_C const CLSID CLSID_InkStrokes;

#ifdef __cplusplus

class DECLSPEC_UUID("48F491BC-240E-4860-B079-A1E94D3D2C86")
InkStrokes;
#endif

#ifndef __IInk_INTERFACE_DEFINED__
#define __IInk_INTERFACE_DEFINED__

/* interface IInk */
/* [unique][hidden][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_IInk;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03F8E511-43A1-11D3-8BB6-0080C7D6BAD5")
    IInk : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IInkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInk * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInk * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in IInk * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in IInk * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in IInk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            IInk * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } IInkVtbl;

    interface IInk
    {
        CONST_VTBL struct IInkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInk_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInk_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInk_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInk_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define IInk_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define IInk_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define IInk_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInk_INTERFACE_DEFINED__ */


#ifndef __IInkLineInfo_INTERFACE_DEFINED__
#define __IInkLineInfo_INTERFACE_DEFINED__

/* interface IInkLineInfo */
/* [unique][hidden][helpcontext][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IInkLineInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9C1C5AD6-F22F-4DE4-B453-A2CC482E7C33")
    IInkLineInfo : public IUnknown
    {
    public:
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE SetFormat( 
            __RPC__in INKMETRIC *pim) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetFormat( 
            __RPC__in INKMETRIC *pim) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetInkExtent( 
            __RPC__in INKMETRIC *pim,
            __RPC__in UINT *pnWidth) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE GetCandidate( 
            UINT nCandidateNum,
            __RPC__in LPWSTR pwcRecogWord,
            __RPC__in UINT *pcwcRecogWord,
            DWORD dwFlags) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE SetCandidate( 
            UINT nCandidateNum,
            __RPC__in LPWSTR strRecogWord) = 0;
        
        virtual /* [helpcontext][helpstring] */ HRESULT STDMETHODCALLTYPE Recognize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInkLineInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInkLineInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInkLineInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInkLineInfo * This);
        
        DECLSPEC_XFGVIRT(IInkLineInfo, SetFormat)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetFormat )( 
            __RPC__in IInkLineInfo * This,
            __RPC__in INKMETRIC *pim);
        
        DECLSPEC_XFGVIRT(IInkLineInfo, GetFormat)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in IInkLineInfo * This,
            __RPC__in INKMETRIC *pim);
        
        DECLSPEC_XFGVIRT(IInkLineInfo, GetInkExtent)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetInkExtent )( 
            __RPC__in IInkLineInfo * This,
            __RPC__in INKMETRIC *pim,
            __RPC__in UINT *pnWidth);
        
        DECLSPEC_XFGVIRT(IInkLineInfo, GetCandidate)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *GetCandidate )( 
            __RPC__in IInkLineInfo * This,
            UINT nCandidateNum,
            __RPC__in LPWSTR pwcRecogWord,
            __RPC__in UINT *pcwcRecogWord,
            DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IInkLineInfo, SetCandidate)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *SetCandidate )( 
            __RPC__in IInkLineInfo * This,
            UINT nCandidateNum,
            __RPC__in LPWSTR strRecogWord);
        
        DECLSPEC_XFGVIRT(IInkLineInfo, Recognize)
        /* [helpcontext][helpstring] */ HRESULT ( STDMETHODCALLTYPE *Recognize )( 
            __RPC__in IInkLineInfo * This);
        
        END_INTERFACE
    } IInkLineInfoVtbl;

    interface IInkLineInfo
    {
        CONST_VTBL struct IInkLineInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInkLineInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInkLineInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInkLineInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInkLineInfo_SetFormat(This,pim)	\
    ( (This)->lpVtbl -> SetFormat(This,pim) ) 

#define IInkLineInfo_GetFormat(This,pim)	\
    ( (This)->lpVtbl -> GetFormat(This,pim) ) 

#define IInkLineInfo_GetInkExtent(This,pim,pnWidth)	\
    ( (This)->lpVtbl -> GetInkExtent(This,pim,pnWidth) ) 

#define IInkLineInfo_GetCandidate(This,nCandidateNum,pwcRecogWord,pcwcRecogWord,dwFlags)	\
    ( (This)->lpVtbl -> GetCandidate(This,nCandidateNum,pwcRecogWord,pcwcRecogWord,dwFlags) ) 

#define IInkLineInfo_SetCandidate(This,nCandidateNum,strRecogWord)	\
    ( (This)->lpVtbl -> SetCandidate(This,nCandidateNum,strRecogWord) ) 

#define IInkLineInfo_Recognize(This)	\
    ( (This)->lpVtbl -> Recognize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInkLineInfo_INTERFACE_DEFINED__ */


#ifndef __ISketchInk_INTERFACE_DEFINED__
#define __ISketchInk_INTERFACE_DEFINED__

/* interface ISketchInk */
/* [unique][hidden][helpcontext][helpstring][dual][uuid][object] */ 


EXTERN_C const IID IID_ISketchInk;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B4563688-98EB-4646-B279-44DA14D45748")
    ISketchInk : public IDispatch
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct ISketchInkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISketchInk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISketchInk * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISketchInk * This);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfoCount)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfoCount )( 
            __RPC__in ISketchInk * This,
            /* [out] */ __RPC__out UINT *pctinfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetTypeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetTypeInfo )( 
            __RPC__in ISketchInk * This,
            /* [in] */ UINT iTInfo,
            /* [in] */ LCID lcid,
            /* [out] */ __RPC__deref_out_opt ITypeInfo **ppTInfo);
        
        DECLSPEC_XFGVIRT(IDispatch, GetIDsOfNames)
        HRESULT ( STDMETHODCALLTYPE *GetIDsOfNames )( 
            __RPC__in ISketchInk * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [size_is][in] */ __RPC__in_ecount_full(cNames) LPOLESTR *rgszNames,
            /* [range][in] */ __RPC__in_range(0,16384) UINT cNames,
            /* [in] */ LCID lcid,
            /* [size_is][out] */ __RPC__out_ecount_full(cNames) DISPID *rgDispId);
        
        DECLSPEC_XFGVIRT(IDispatch, Invoke)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            ISketchInk * This,
            /* [annotation][in] */ 
            _In_  DISPID dispIdMember,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][in] */ 
            _In_  LCID lcid,
            /* [annotation][in] */ 
            _In_  WORD wFlags,
            /* [annotation][out][in] */ 
            _In_  DISPPARAMS *pDispParams,
            /* [annotation][out] */ 
            _Out_opt_  VARIANT *pVarResult,
            /* [annotation][out] */ 
            _Out_opt_  EXCEPINFO *pExcepInfo,
            /* [annotation][out] */ 
            _Out_opt_  UINT *puArgErr);
        
        END_INTERFACE
    } ISketchInkVtbl;

    interface ISketchInk
    {
        CONST_VTBL struct ISketchInkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISketchInk_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISketchInk_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISketchInk_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISketchInk_GetTypeInfoCount(This,pctinfo)	\
    ( (This)->lpVtbl -> GetTypeInfoCount(This,pctinfo) ) 

#define ISketchInk_GetTypeInfo(This,iTInfo,lcid,ppTInfo)	\
    ( (This)->lpVtbl -> GetTypeInfo(This,iTInfo,lcid,ppTInfo) ) 

#define ISketchInk_GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId)	\
    ( (This)->lpVtbl -> GetIDsOfNames(This,riid,rgszNames,cNames,lcid,rgDispId) ) 

#define ISketchInk_Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr)	\
    ( (This)->lpVtbl -> Invoke(This,dispIdMember,riid,lcid,wFlags,pDispParams,pVarResult,pExcepInfo,puArgErr) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISketchInk_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_Ink;

#ifdef __cplusplus

class DECLSPEC_UUID("13DE4A42-8D21-4C8E-BF9C-8F69CB068FCA")
Ink;
#endif

EXTERN_C const CLSID CLSID_SketchInk;

#ifdef __cplusplus

class DECLSPEC_UUID("F0291081-E87C-4E07-97DA-A0A03761E586")
SketchInk;
#endif
#endif /* __MSINKAUTLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_msinkaut_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* of #define _MSINKAUT_IDL_H_*/


extern RPC_IF_HANDLE __MIDL_itf_msinkaut_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_msinkaut_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


