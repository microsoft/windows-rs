

/* this ALWAYS GENERATED file contains the definitions for the interfaces */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */

#pragma warning( disable: 4049 )  /* more than 64k source lines */


/* verify that the <rpcndr.h> version is high enough to compile this file*/
#ifndef __REQUIRED_RPCNDR_H_VERSION__
#define __REQUIRED_RPCNDR_H_VERSION__ 475
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


#ifndef __uiautomationcore_h__
#define __uiautomationcore_h__

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

#ifndef __IRawElementProviderSimple_FWD_DEFINED__
#define __IRawElementProviderSimple_FWD_DEFINED__
typedef interface IRawElementProviderSimple IRawElementProviderSimple;

#endif 	/* __IRawElementProviderSimple_FWD_DEFINED__ */


#ifndef __IAccessibleEx_FWD_DEFINED__
#define __IAccessibleEx_FWD_DEFINED__
typedef interface IAccessibleEx IAccessibleEx;

#endif 	/* __IAccessibleEx_FWD_DEFINED__ */


#ifndef __IRawElementProviderSimple2_FWD_DEFINED__
#define __IRawElementProviderSimple2_FWD_DEFINED__
typedef interface IRawElementProviderSimple2 IRawElementProviderSimple2;

#endif 	/* __IRawElementProviderSimple2_FWD_DEFINED__ */


#ifndef __IRawElementProviderSimple3_FWD_DEFINED__
#define __IRawElementProviderSimple3_FWD_DEFINED__
typedef interface IRawElementProviderSimple3 IRawElementProviderSimple3;

#endif 	/* __IRawElementProviderSimple3_FWD_DEFINED__ */


#ifndef __IRawElementProviderFragmentRoot_FWD_DEFINED__
#define __IRawElementProviderFragmentRoot_FWD_DEFINED__
typedef interface IRawElementProviderFragmentRoot IRawElementProviderFragmentRoot;

#endif 	/* __IRawElementProviderFragmentRoot_FWD_DEFINED__ */


#ifndef __IRawElementProviderFragment_FWD_DEFINED__
#define __IRawElementProviderFragment_FWD_DEFINED__
typedef interface IRawElementProviderFragment IRawElementProviderFragment;

#endif 	/* __IRawElementProviderFragment_FWD_DEFINED__ */


#ifndef __IRawElementProviderAdviseEvents_FWD_DEFINED__
#define __IRawElementProviderAdviseEvents_FWD_DEFINED__
typedef interface IRawElementProviderAdviseEvents IRawElementProviderAdviseEvents;

#endif 	/* __IRawElementProviderAdviseEvents_FWD_DEFINED__ */


#ifndef __IRawElementProviderHwndOverride_FWD_DEFINED__
#define __IRawElementProviderHwndOverride_FWD_DEFINED__
typedef interface IRawElementProviderHwndOverride IRawElementProviderHwndOverride;

#endif 	/* __IRawElementProviderHwndOverride_FWD_DEFINED__ */


#ifndef __IProxyProviderWinEventSink_FWD_DEFINED__
#define __IProxyProviderWinEventSink_FWD_DEFINED__
typedef interface IProxyProviderWinEventSink IProxyProviderWinEventSink;

#endif 	/* __IProxyProviderWinEventSink_FWD_DEFINED__ */


#ifndef __IProxyProviderWinEventHandler_FWD_DEFINED__
#define __IProxyProviderWinEventHandler_FWD_DEFINED__
typedef interface IProxyProviderWinEventHandler IProxyProviderWinEventHandler;

#endif 	/* __IProxyProviderWinEventHandler_FWD_DEFINED__ */


#ifndef __IRawElementProviderWindowlessSite_FWD_DEFINED__
#define __IRawElementProviderWindowlessSite_FWD_DEFINED__
typedef interface IRawElementProviderWindowlessSite IRawElementProviderWindowlessSite;

#endif 	/* __IRawElementProviderWindowlessSite_FWD_DEFINED__ */


#ifndef __IAccessibleHostingElementProviders_FWD_DEFINED__
#define __IAccessibleHostingElementProviders_FWD_DEFINED__
typedef interface IAccessibleHostingElementProviders IAccessibleHostingElementProviders;

#endif 	/* __IAccessibleHostingElementProviders_FWD_DEFINED__ */


#ifndef __IRawElementProviderHostingAccessibles_FWD_DEFINED__
#define __IRawElementProviderHostingAccessibles_FWD_DEFINED__
typedef interface IRawElementProviderHostingAccessibles IRawElementProviderHostingAccessibles;

#endif 	/* __IRawElementProviderHostingAccessibles_FWD_DEFINED__ */


#ifndef __IDockProvider_FWD_DEFINED__
#define __IDockProvider_FWD_DEFINED__
typedef interface IDockProvider IDockProvider;

#endif 	/* __IDockProvider_FWD_DEFINED__ */


#ifndef __IExpandCollapseProvider_FWD_DEFINED__
#define __IExpandCollapseProvider_FWD_DEFINED__
typedef interface IExpandCollapseProvider IExpandCollapseProvider;

#endif 	/* __IExpandCollapseProvider_FWD_DEFINED__ */


#ifndef __IGridProvider_FWD_DEFINED__
#define __IGridProvider_FWD_DEFINED__
typedef interface IGridProvider IGridProvider;

#endif 	/* __IGridProvider_FWD_DEFINED__ */


#ifndef __IGridItemProvider_FWD_DEFINED__
#define __IGridItemProvider_FWD_DEFINED__
typedef interface IGridItemProvider IGridItemProvider;

#endif 	/* __IGridItemProvider_FWD_DEFINED__ */


#ifndef __IInvokeProvider_FWD_DEFINED__
#define __IInvokeProvider_FWD_DEFINED__
typedef interface IInvokeProvider IInvokeProvider;

#endif 	/* __IInvokeProvider_FWD_DEFINED__ */


#ifndef __IMultipleViewProvider_FWD_DEFINED__
#define __IMultipleViewProvider_FWD_DEFINED__
typedef interface IMultipleViewProvider IMultipleViewProvider;

#endif 	/* __IMultipleViewProvider_FWD_DEFINED__ */


#ifndef __IRangeValueProvider_FWD_DEFINED__
#define __IRangeValueProvider_FWD_DEFINED__
typedef interface IRangeValueProvider IRangeValueProvider;

#endif 	/* __IRangeValueProvider_FWD_DEFINED__ */


#ifndef __IScrollItemProvider_FWD_DEFINED__
#define __IScrollItemProvider_FWD_DEFINED__
typedef interface IScrollItemProvider IScrollItemProvider;

#endif 	/* __IScrollItemProvider_FWD_DEFINED__ */


#ifndef __ISelectionProvider_FWD_DEFINED__
#define __ISelectionProvider_FWD_DEFINED__
typedef interface ISelectionProvider ISelectionProvider;

#endif 	/* __ISelectionProvider_FWD_DEFINED__ */


#ifndef __ISelectionProvider2_FWD_DEFINED__
#define __ISelectionProvider2_FWD_DEFINED__
typedef interface ISelectionProvider2 ISelectionProvider2;

#endif 	/* __ISelectionProvider2_FWD_DEFINED__ */


#ifndef __IScrollProvider_FWD_DEFINED__
#define __IScrollProvider_FWD_DEFINED__
typedef interface IScrollProvider IScrollProvider;

#endif 	/* __IScrollProvider_FWD_DEFINED__ */


#ifndef __ISelectionItemProvider_FWD_DEFINED__
#define __ISelectionItemProvider_FWD_DEFINED__
typedef interface ISelectionItemProvider ISelectionItemProvider;

#endif 	/* __ISelectionItemProvider_FWD_DEFINED__ */


#ifndef __ISynchronizedInputProvider_FWD_DEFINED__
#define __ISynchronizedInputProvider_FWD_DEFINED__
typedef interface ISynchronizedInputProvider ISynchronizedInputProvider;

#endif 	/* __ISynchronizedInputProvider_FWD_DEFINED__ */


#ifndef __ITableProvider_FWD_DEFINED__
#define __ITableProvider_FWD_DEFINED__
typedef interface ITableProvider ITableProvider;

#endif 	/* __ITableProvider_FWD_DEFINED__ */


#ifndef __ITableItemProvider_FWD_DEFINED__
#define __ITableItemProvider_FWD_DEFINED__
typedef interface ITableItemProvider ITableItemProvider;

#endif 	/* __ITableItemProvider_FWD_DEFINED__ */


#ifndef __IToggleProvider_FWD_DEFINED__
#define __IToggleProvider_FWD_DEFINED__
typedef interface IToggleProvider IToggleProvider;

#endif 	/* __IToggleProvider_FWD_DEFINED__ */


#ifndef __ITransformProvider_FWD_DEFINED__
#define __ITransformProvider_FWD_DEFINED__
typedef interface ITransformProvider ITransformProvider;

#endif 	/* __ITransformProvider_FWD_DEFINED__ */


#ifndef __IValueProvider_FWD_DEFINED__
#define __IValueProvider_FWD_DEFINED__
typedef interface IValueProvider IValueProvider;

#endif 	/* __IValueProvider_FWD_DEFINED__ */


#ifndef __IWindowProvider_FWD_DEFINED__
#define __IWindowProvider_FWD_DEFINED__
typedef interface IWindowProvider IWindowProvider;

#endif 	/* __IWindowProvider_FWD_DEFINED__ */


#ifndef __ILegacyIAccessibleProvider_FWD_DEFINED__
#define __ILegacyIAccessibleProvider_FWD_DEFINED__
typedef interface ILegacyIAccessibleProvider ILegacyIAccessibleProvider;

#endif 	/* __ILegacyIAccessibleProvider_FWD_DEFINED__ */


#ifndef __IItemContainerProvider_FWD_DEFINED__
#define __IItemContainerProvider_FWD_DEFINED__
typedef interface IItemContainerProvider IItemContainerProvider;

#endif 	/* __IItemContainerProvider_FWD_DEFINED__ */


#ifndef __IVirtualizedItemProvider_FWD_DEFINED__
#define __IVirtualizedItemProvider_FWD_DEFINED__
typedef interface IVirtualizedItemProvider IVirtualizedItemProvider;

#endif 	/* __IVirtualizedItemProvider_FWD_DEFINED__ */


#ifndef __IObjectModelProvider_FWD_DEFINED__
#define __IObjectModelProvider_FWD_DEFINED__
typedef interface IObjectModelProvider IObjectModelProvider;

#endif 	/* __IObjectModelProvider_FWD_DEFINED__ */


#ifndef __IAnnotationProvider_FWD_DEFINED__
#define __IAnnotationProvider_FWD_DEFINED__
typedef interface IAnnotationProvider IAnnotationProvider;

#endif 	/* __IAnnotationProvider_FWD_DEFINED__ */


#ifndef __IStylesProvider_FWD_DEFINED__
#define __IStylesProvider_FWD_DEFINED__
typedef interface IStylesProvider IStylesProvider;

#endif 	/* __IStylesProvider_FWD_DEFINED__ */


#ifndef __ISpreadsheetProvider_FWD_DEFINED__
#define __ISpreadsheetProvider_FWD_DEFINED__
typedef interface ISpreadsheetProvider ISpreadsheetProvider;

#endif 	/* __ISpreadsheetProvider_FWD_DEFINED__ */


#ifndef __ISpreadsheetItemProvider_FWD_DEFINED__
#define __ISpreadsheetItemProvider_FWD_DEFINED__
typedef interface ISpreadsheetItemProvider ISpreadsheetItemProvider;

#endif 	/* __ISpreadsheetItemProvider_FWD_DEFINED__ */


#ifndef __ITransformProvider2_FWD_DEFINED__
#define __ITransformProvider2_FWD_DEFINED__
typedef interface ITransformProvider2 ITransformProvider2;

#endif 	/* __ITransformProvider2_FWD_DEFINED__ */


#ifndef __IDragProvider_FWD_DEFINED__
#define __IDragProvider_FWD_DEFINED__
typedef interface IDragProvider IDragProvider;

#endif 	/* __IDragProvider_FWD_DEFINED__ */


#ifndef __IDropTargetProvider_FWD_DEFINED__
#define __IDropTargetProvider_FWD_DEFINED__
typedef interface IDropTargetProvider IDropTargetProvider;

#endif 	/* __IDropTargetProvider_FWD_DEFINED__ */


#ifndef __ITextRangeProvider_FWD_DEFINED__
#define __ITextRangeProvider_FWD_DEFINED__
typedef interface ITextRangeProvider ITextRangeProvider;

#endif 	/* __ITextRangeProvider_FWD_DEFINED__ */


#ifndef __ITextProvider_FWD_DEFINED__
#define __ITextProvider_FWD_DEFINED__
typedef interface ITextProvider ITextProvider;

#endif 	/* __ITextProvider_FWD_DEFINED__ */


#ifndef __ITextProvider2_FWD_DEFINED__
#define __ITextProvider2_FWD_DEFINED__
typedef interface ITextProvider2 ITextProvider2;

#endif 	/* __ITextProvider2_FWD_DEFINED__ */


#ifndef __ITextEditProvider_FWD_DEFINED__
#define __ITextEditProvider_FWD_DEFINED__
typedef interface ITextEditProvider ITextEditProvider;

#endif 	/* __ITextEditProvider_FWD_DEFINED__ */


#ifndef __ITextRangeProvider2_FWD_DEFINED__
#define __ITextRangeProvider2_FWD_DEFINED__
typedef interface ITextRangeProvider2 ITextRangeProvider2;

#endif 	/* __ITextRangeProvider2_FWD_DEFINED__ */


#ifndef __ITextChildProvider_FWD_DEFINED__
#define __ITextChildProvider_FWD_DEFINED__
typedef interface ITextChildProvider ITextChildProvider;

#endif 	/* __ITextChildProvider_FWD_DEFINED__ */


#ifndef __ICustomNavigationProvider_FWD_DEFINED__
#define __ICustomNavigationProvider_FWD_DEFINED__
typedef interface ICustomNavigationProvider ICustomNavigationProvider;

#endif 	/* __ICustomNavigationProvider_FWD_DEFINED__ */


#ifndef __IUIAutomationPatternInstance_FWD_DEFINED__
#define __IUIAutomationPatternInstance_FWD_DEFINED__
typedef interface IUIAutomationPatternInstance IUIAutomationPatternInstance;

#endif 	/* __IUIAutomationPatternInstance_FWD_DEFINED__ */


#ifndef __IUIAutomationPatternHandler_FWD_DEFINED__
#define __IUIAutomationPatternHandler_FWD_DEFINED__
typedef interface IUIAutomationPatternHandler IUIAutomationPatternHandler;

#endif 	/* __IUIAutomationPatternHandler_FWD_DEFINED__ */


#ifndef __IUIAutomationRegistrar_FWD_DEFINED__
#define __IUIAutomationRegistrar_FWD_DEFINED__
typedef interface IUIAutomationRegistrar IUIAutomationRegistrar;

#endif 	/* __IUIAutomationRegistrar_FWD_DEFINED__ */


#ifndef __CUIAutomationRegistrar_FWD_DEFINED__
#define __CUIAutomationRegistrar_FWD_DEFINED__

#ifdef __cplusplus
typedef class CUIAutomationRegistrar CUIAutomationRegistrar;
#else
typedef struct CUIAutomationRegistrar CUIAutomationRegistrar;
#endif /* __cplusplus */

#endif 	/* __CUIAutomationRegistrar_FWD_DEFINED__ */


#ifndef __IUIAutomationClientInfo_FWD_DEFINED__
#define __IUIAutomationClientInfo_FWD_DEFINED__
typedef interface IUIAutomationClientInfo IUIAutomationClientInfo;

#endif 	/* __IUIAutomationClientInfo_FWD_DEFINED__ */


#ifndef __CUIAutomationClientInfo_FWD_DEFINED__
#define __CUIAutomationClientInfo_FWD_DEFINED__

#ifdef __cplusplus
typedef class CUIAutomationClientInfo CUIAutomationClientInfo;
#else
typedef struct CUIAutomationClientInfo CUIAutomationClientInfo;
#endif /* __cplusplus */

#endif 	/* __CUIAutomationClientInfo_FWD_DEFINED__ */


#ifndef __IUIAutomationClientConnectionCallback_FWD_DEFINED__
#define __IUIAutomationClientConnectionCallback_FWD_DEFINED__
typedef interface IUIAutomationClientConnectionCallback IUIAutomationClientConnectionCallback;

#endif 	/* __IUIAutomationClientConnectionCallback_FWD_DEFINED__ */


#ifndef __IUIAutomationClientInfoSource_FWD_DEFINED__
#define __IUIAutomationClientInfoSource_FWD_DEFINED__
typedef interface IUIAutomationClientInfoSource IUIAutomationClientInfoSource;

#endif 	/* __IUIAutomationClientInfoSource_FWD_DEFINED__ */


#ifndef __CUIAutomationClientInfoSource_FWD_DEFINED__
#define __CUIAutomationClientInfoSource_FWD_DEFINED__

#ifdef __cplusplus
typedef class CUIAutomationClientInfoSource CUIAutomationClientInfoSource;
#else
typedef struct CUIAutomationClientInfoSource CUIAutomationClientInfoSource;
#endif /* __cplusplus */

#endif 	/* __CUIAutomationClientInfoSource_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "oleacc.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_uiautomationcore_0000_0000 */
/* [local] */ 

// -------------------------------------------------------------
// UIAutomationCore.H
//
// UIAutomation interface definitions and related types and enums
// (Generated from UIAutomationCore.idl)
//
// Copyright (c) Microsoft Corporation. All rights reserved.
// -------------------------------------------------------------

enum NavigateDirection
    {
        NavigateDirection_Parent	= 0,
        NavigateDirection_NextSibling	= 1,
        NavigateDirection_PreviousSibling	= 2,
        NavigateDirection_FirstChild	= 3,
        NavigateDirection_LastChild	= 4
    } ;

enum ProviderOptions
    {
        ProviderOptions_ClientSideProvider	= 0x1,
        ProviderOptions_ServerSideProvider	= 0x2,
        ProviderOptions_NonClientAreaProvider	= 0x4,
        ProviderOptions_OverrideProvider	= 0x8,
        ProviderOptions_ProviderOwnsSetFocus	= 0x10,
        ProviderOptions_UseComThreading	= 0x20,
        ProviderOptions_RefuseNonClientSupport	= 0x40,
        ProviderOptions_HasNativeIAccessible	= 0x80,
        ProviderOptions_UseClientCoordinates	= 0x100
    } ;
DEFINE_ENUM_FLAG_OPERATORS(ProviderOptions)

enum StructureChangeType
    {
        StructureChangeType_ChildAdded	= 0,
        StructureChangeType_ChildRemoved	= ( StructureChangeType_ChildAdded + 1 ) ,
        StructureChangeType_ChildrenInvalidated	= ( StructureChangeType_ChildRemoved + 1 ) ,
        StructureChangeType_ChildrenBulkAdded	= ( StructureChangeType_ChildrenInvalidated + 1 ) ,
        StructureChangeType_ChildrenBulkRemoved	= ( StructureChangeType_ChildrenBulkAdded + 1 ) ,
        StructureChangeType_ChildrenReordered	= ( StructureChangeType_ChildrenBulkRemoved + 1 ) 
    } ;

enum TextEditChangeType
    {
        TextEditChangeType_None	= 0,
        TextEditChangeType_AutoCorrect	= 1,
        TextEditChangeType_Composition	= 2,
        TextEditChangeType_CompositionFinalized	= 3,
        TextEditChangeType_AutoComplete	= 4
    } ;

enum OrientationType
    {
        OrientationType_None	= 0,
        OrientationType_Horizontal	= 1,
        OrientationType_Vertical	= 2
    } ;

enum DockPosition
    {
        DockPosition_Top	= 0,
        DockPosition_Left	= 1,
        DockPosition_Bottom	= 2,
        DockPosition_Right	= 3,
        DockPosition_Fill	= 4,
        DockPosition_None	= 5
    } ;

enum ExpandCollapseState
    {
        ExpandCollapseState_Collapsed	= 0,
        ExpandCollapseState_Expanded	= 1,
        ExpandCollapseState_PartiallyExpanded	= 2,
        ExpandCollapseState_LeafNode	= 3
    } ;

enum ScrollAmount
    {
        ScrollAmount_LargeDecrement	= 0,
        ScrollAmount_SmallDecrement	= 1,
        ScrollAmount_NoAmount	= 2,
        ScrollAmount_LargeIncrement	= 3,
        ScrollAmount_SmallIncrement	= 4
    } ;

enum RowOrColumnMajor
    {
        RowOrColumnMajor_RowMajor	= 0,
        RowOrColumnMajor_ColumnMajor	= 1,
        RowOrColumnMajor_Indeterminate	= 2
    } ;

enum ToggleState
    {
        ToggleState_Off	= 0,
        ToggleState_On	= 1,
        ToggleState_Indeterminate	= 2
    } ;

enum WindowVisualState
    {
        WindowVisualState_Normal	= 0,
        WindowVisualState_Maximized	= 1,
        WindowVisualState_Minimized	= 2
    } ;

enum SynchronizedInputType
    {
        SynchronizedInputType_KeyUp	= 0x1,
        SynchronizedInputType_KeyDown	= 0x2,
        SynchronizedInputType_LeftMouseUp	= 0x4,
        SynchronizedInputType_LeftMouseDown	= 0x8,
        SynchronizedInputType_RightMouseUp	= 0x10,
        SynchronizedInputType_RightMouseDown	= 0x20
    } ;
DEFINE_ENUM_FLAG_OPERATORS(SynchronizedInputType)

enum WindowInteractionState
    {
        WindowInteractionState_Running	= 0,
        WindowInteractionState_Closing	= 1,
        WindowInteractionState_ReadyForUserInteraction	= 2,
        WindowInteractionState_BlockedByModalWindow	= 3,
        WindowInteractionState_NotResponding	= 4
    } ;

enum SayAsInterpretAs
    {
        SayAsInterpretAs_None	= 0,
        SayAsInterpretAs_Spell	= 1,
        SayAsInterpretAs_Cardinal	= 2,
        SayAsInterpretAs_Ordinal	= 3,
        SayAsInterpretAs_Number	= 4,
        SayAsInterpretAs_Date	= 5,
        SayAsInterpretAs_Time	= 6,
        SayAsInterpretAs_Telephone	= 7,
        SayAsInterpretAs_Currency	= 8,
        SayAsInterpretAs_Net	= 9,
        SayAsInterpretAs_Url	= 10,
        SayAsInterpretAs_Address	= 11,
        SayAsInterpretAs_Alphanumeric	= 12,
        SayAsInterpretAs_Name	= 13,
        SayAsInterpretAs_Media	= 14,
        SayAsInterpretAs_Date_MonthDayYear	= 15,
        SayAsInterpretAs_Date_DayMonthYear	= 16,
        SayAsInterpretAs_Date_YearMonthDay	= 17,
        SayAsInterpretAs_Date_YearMonth	= 18,
        SayAsInterpretAs_Date_MonthYear	= 19,
        SayAsInterpretAs_Date_DayMonth	= 20,
        SayAsInterpretAs_Date_MonthDay	= 21,
        SayAsInterpretAs_Date_Year	= 22,
        SayAsInterpretAs_Time_HoursMinutesSeconds12	= 23,
        SayAsInterpretAs_Time_HoursMinutes12	= 24,
        SayAsInterpretAs_Time_HoursMinutesSeconds24	= 25,
        SayAsInterpretAs_Time_HoursMinutes24	= 26
    } ;

enum TextUnit
    {
        TextUnit_Character	= 0,
        TextUnit_Format	= 1,
        TextUnit_Word	= 2,
        TextUnit_Line	= 3,
        TextUnit_Paragraph	= 4,
        TextUnit_Page	= 5,
        TextUnit_Document	= 6
    } ;

enum TextPatternRangeEndpoint
    {
        TextPatternRangeEndpoint_Start	= 0,
        TextPatternRangeEndpoint_End	= 1
    } ;

enum SupportedTextSelection
    {
        SupportedTextSelection_None	= 0,
        SupportedTextSelection_Single	= 1,
        SupportedTextSelection_Multiple	= 2
    } ;

enum LiveSetting
    {
        Off	= 0,
        Polite	= 1,
        Assertive	= 2
    } ;

enum ActiveEnd
    {
        ActiveEnd_None	= 0,
        ActiveEnd_Start	= 1,
        ActiveEnd_End	= 2
    } ;

enum CaretPosition
    {
        CaretPosition_Unknown	= 0,
        CaretPosition_EndOfLine	= 1,
        CaretPosition_BeginningOfLine	= 2
    } ;

enum CaretBidiMode
    {
        CaretBidiMode_LTR	= 0,
        CaretBidiMode_RTL	= 1
    } ;

enum ZoomUnit
    {
        ZoomUnit_NoAmount	= 0,
        ZoomUnit_LargeDecrement	= 1,
        ZoomUnit_SmallDecrement	= 2,
        ZoomUnit_LargeIncrement	= 3,
        ZoomUnit_SmallIncrement	= 4
    } ;

enum AnimationStyle
    {
        AnimationStyle_None	= 0,
        AnimationStyle_LasVegasLights	= 1,
        AnimationStyle_BlinkingBackground	= 2,
        AnimationStyle_SparkleText	= 3,
        AnimationStyle_MarchingBlackAnts	= 4,
        AnimationStyle_MarchingRedAnts	= 5,
        AnimationStyle_Shimmer	= 6,
        AnimationStyle_Other	= -1
    } ;

enum BulletStyle
    {
        BulletStyle_None	= 0,
        BulletStyle_HollowRoundBullet	= 1,
        BulletStyle_FilledRoundBullet	= 2,
        BulletStyle_HollowSquareBullet	= 3,
        BulletStyle_FilledSquareBullet	= 4,
        BulletStyle_DashBullet	= 5,
        BulletStyle_Other	= -1
    } ;

enum CapStyle
    {
        CapStyle_None	= 0,
        CapStyle_SmallCap	= 1,
        CapStyle_AllCap	= 2,
        CapStyle_AllPetiteCaps	= 3,
        CapStyle_PetiteCaps	= 4,
        CapStyle_Unicase	= 5,
        CapStyle_Titling	= 6,
        CapStyle_Other	= -1
    } ;

enum FillType
    {
        FillType_None	= 0,
        FillType_Color	= 1,
        FillType_Gradient	= 2,
        FillType_Picture	= 3,
        FillType_Pattern	= 4
    } ;

enum FlowDirections
    {
        FlowDirections_Default	= 0,
        FlowDirections_RightToLeft	= 0x1,
        FlowDirections_BottomToTop	= 0x2,
        FlowDirections_Vertical	= 0x4
    } ;

enum HorizontalTextAlignment
    {
        HorizontalTextAlignment_Left	= 0,
        HorizontalTextAlignment_Centered	= 1,
        HorizontalTextAlignment_Right	= 2,
        HorizontalTextAlignment_Justified	= 3
    } ;

enum OutlineStyles
    {
        OutlineStyles_None	= 0,
        OutlineStyles_Outline	= 1,
        OutlineStyles_Shadow	= 2,
        OutlineStyles_Engraved	= 4,
        OutlineStyles_Embossed	= 8
    } ;

enum TextDecorationLineStyle
    {
        TextDecorationLineStyle_None	= 0,
        TextDecorationLineStyle_Single	= 1,
        TextDecorationLineStyle_WordsOnly	= 2,
        TextDecorationLineStyle_Double	= 3,
        TextDecorationLineStyle_Dot	= 4,
        TextDecorationLineStyle_Dash	= 5,
        TextDecorationLineStyle_DashDot	= 6,
        TextDecorationLineStyle_DashDotDot	= 7,
        TextDecorationLineStyle_Wavy	= 8,
        TextDecorationLineStyle_ThickSingle	= 9,
        TextDecorationLineStyle_DoubleWavy	= 11,
        TextDecorationLineStyle_ThickWavy	= 12,
        TextDecorationLineStyle_LongDash	= 13,
        TextDecorationLineStyle_ThickDash	= 14,
        TextDecorationLineStyle_ThickDashDot	= 15,
        TextDecorationLineStyle_ThickDashDotDot	= 16,
        TextDecorationLineStyle_ThickDot	= 17,
        TextDecorationLineStyle_ThickLongDash	= 18,
        TextDecorationLineStyle_Other	= -1
    } ;

enum VisualEffects
    {
        VisualEffects_None	= 0,
        VisualEffects_Shadow	= ( 1 << 0 ) ,
        VisualEffects_Reflection	= ( 1 << 1 ) ,
        VisualEffects_Glow	= ( 1 << 2 ) ,
        VisualEffects_SoftEdges	= ( 1 << 3 ) ,
        VisualEffects_Bevel	= ( 1 << 4 ) 
    } ;

enum NotificationProcessing
    {
        NotificationProcessing_ImportantAll	= 0,
        NotificationProcessing_ImportantMostRecent	= 1,
        NotificationProcessing_All	= 2,
        NotificationProcessing_MostRecent	= 3,
        NotificationProcessing_CurrentThenMostRecent	= 4,
        NotificationProcessing_ImportantCurrentThenMostRecent	= 5
    } ;

enum NotificationKind
    {
        NotificationKind_ItemAdded	= 0,
        NotificationKind_ItemRemoved	= 1,
        NotificationKind_ActionCompleted	= 2,
        NotificationKind_ActionAborted	= 3,
        NotificationKind_Other	= 4
    } ;
typedef int PROPERTYID;

typedef int PATTERNID;

typedef int EVENTID;

typedef int TEXTATTRIBUTEID;

typedef int CONTROLTYPEID;

typedef int LANDMARKTYPEID;

typedef int METADATAID;

typedef int HEADINGLEVELID;

struct UiaRect
    {
    double left;
    double top;
    double width;
    double height;
    } ;
struct UiaPoint
    {
    double x;
    double y;
    } ;
struct UiaChangeInfo
    {
    int uiaId;
    VARIANT payload;
    VARIANT extraInfo;
    } ;


extern RPC_IF_HANDLE __MIDL_itf_uiautomationcore_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_uiautomationcore_0000_0000_v0_0_s_ifspec;


#ifndef __UIA_LIBRARY_DEFINED__
#define __UIA_LIBRARY_DEFINED__

/* library UIA */
/* [hidden][version][lcid][uuid] */ 




enum UIAutomationType
    {
        UIAutomationType_Int	= 0x1,
        UIAutomationType_Bool	= 0x2,
        UIAutomationType_String	= 0x3,
        UIAutomationType_Double	= 0x4,
        UIAutomationType_Point	= 0x5,
        UIAutomationType_Rect	= 0x6,
        UIAutomationType_Element	= 0x7,
        UIAutomationType_Array	= 0x10000,
        UIAutomationType_Out	= 0x20000,
        UIAutomationType_IntArray	= ( UIAutomationType_Int | UIAutomationType_Array ) ,
        UIAutomationType_BoolArray	= ( UIAutomationType_Bool | UIAutomationType_Array ) ,
        UIAutomationType_StringArray	= ( UIAutomationType_String | UIAutomationType_Array ) ,
        UIAutomationType_DoubleArray	= ( UIAutomationType_Double | UIAutomationType_Array ) ,
        UIAutomationType_PointArray	= ( UIAutomationType_Point | UIAutomationType_Array ) ,
        UIAutomationType_RectArray	= ( UIAutomationType_Rect | UIAutomationType_Array ) ,
        UIAutomationType_ElementArray	= ( UIAutomationType_Element | UIAutomationType_Array ) ,
        UIAutomationType_OutInt	= ( UIAutomationType_Int | UIAutomationType_Out ) ,
        UIAutomationType_OutBool	= ( UIAutomationType_Bool | UIAutomationType_Out ) ,
        UIAutomationType_OutString	= ( UIAutomationType_String | UIAutomationType_Out ) ,
        UIAutomationType_OutDouble	= ( UIAutomationType_Double | UIAutomationType_Out ) ,
        UIAutomationType_OutPoint	= ( UIAutomationType_Point | UIAutomationType_Out ) ,
        UIAutomationType_OutRect	= ( UIAutomationType_Rect | UIAutomationType_Out ) ,
        UIAutomationType_OutElement	= ( UIAutomationType_Element | UIAutomationType_Out ) ,
        UIAutomationType_OutIntArray	= ( ( UIAutomationType_Int | UIAutomationType_Array )  | UIAutomationType_Out ) ,
        UIAutomationType_OutBoolArray	= ( ( UIAutomationType_Bool | UIAutomationType_Array )  | UIAutomationType_Out ) ,
        UIAutomationType_OutStringArray	= ( ( UIAutomationType_String | UIAutomationType_Array )  | UIAutomationType_Out ) ,
        UIAutomationType_OutDoubleArray	= ( ( UIAutomationType_Double | UIAutomationType_Array )  | UIAutomationType_Out ) ,
        UIAutomationType_OutPointArray	= ( ( UIAutomationType_Point | UIAutomationType_Array )  | UIAutomationType_Out ) ,
        UIAutomationType_OutRectArray	= ( ( UIAutomationType_Rect | UIAutomationType_Array )  | UIAutomationType_Out ) ,
        UIAutomationType_OutElementArray	= ( ( UIAutomationType_Element | UIAutomationType_Array )  | UIAutomationType_Out ) 
    } ;
DEFINE_ENUM_FLAG_OPERATORS(UIAutomationType)
struct UIAutomationParameter
    {
    enum UIAutomationType type;
    void *pData;
    } ;
struct UIAutomationPropertyInfo
    {
    GUID guid;
    LPCWSTR pProgrammaticName;
    enum UIAutomationType type;
    } ;
struct UIAutomationEventInfo
    {
    GUID guid;
    LPCWSTR pProgrammaticName;
    } ;
struct UIAutomationMethodInfo
    {
    LPCWSTR pProgrammaticName;
    BOOL doSetFocus;
    UINT cInParameters;
    UINT cOutParameters;
    /* [size_is] */ enum UIAutomationType *pParameterTypes;
    /* [size_is] */ LPCWSTR *pParameterNames;
    } ;
struct UIAutomationPatternInfo
    {
    GUID guid;
    LPCWSTR pProgrammaticName;
    GUID providerInterfaceId;
    GUID clientInterfaceId;
    UINT cProperties;
    /* [size_is] */ struct UIAutomationPropertyInfo *pProperties;
    UINT cMethods;
    /* [size_is] */ struct UIAutomationMethodInfo *pMethods;
    UINT cEvents;
    /* [size_is] */ struct UIAutomationEventInfo *pEvents;
    IUIAutomationPatternHandler *pPatternHandler;
    } ;

EXTERN_C const IID LIBID_UIA;


#ifndef __UIA_OtherConstants_MODULE_DEFINED__
#define __UIA_OtherConstants_MODULE_DEFINED__


/* module UIA_OtherConstants */
/* [dllname] */ 

const double UIA_ScrollPatternNoScroll	=	-1;

#endif /* __UIA_OtherConstants_MODULE_DEFINED__ */

#ifndef __IRawElementProviderSimple_INTERFACE_DEFINED__
#define __IRawElementProviderSimple_INTERFACE_DEFINED__

/* interface IRawElementProviderSimple */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderSimple;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d6dd68d1-86fd-4332-8666-9abedea2d24c")
    IRawElementProviderSimple : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProviderOptions( 
            /* [retval][out] */ __RPC__out enum ProviderOptions *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPatternProvider( 
            /* [in] */ PATTERNID patternId,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPropertyValue( 
            /* [in] */ PROPERTYID propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HostRawElementProvider( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderSimpleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderSimple * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderSimple * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderSimple * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, get_ProviderOptions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderOptions )( 
            __RPC__in IRawElementProviderSimple * This,
            /* [retval][out] */ __RPC__out enum ProviderOptions *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, GetPatternProvider)
        HRESULT ( STDMETHODCALLTYPE *GetPatternProvider )( 
            __RPC__in IRawElementProviderSimple * This,
            /* [in] */ PATTERNID patternId,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, GetPropertyValue)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IRawElementProviderSimple * This,
            /* [in] */ PROPERTYID propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, get_HostRawElementProvider)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostRawElementProvider )( 
            __RPC__in IRawElementProviderSimple * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        END_INTERFACE
    } IRawElementProviderSimpleVtbl;

    interface IRawElementProviderSimple
    {
        CONST_VTBL struct IRawElementProviderSimpleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderSimple_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderSimple_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderSimple_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderSimple_get_ProviderOptions(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ProviderOptions(This,pRetVal) ) 

#define IRawElementProviderSimple_GetPatternProvider(This,patternId,pRetVal)	\
    ( (This)->lpVtbl -> GetPatternProvider(This,patternId,pRetVal) ) 

#define IRawElementProviderSimple_GetPropertyValue(This,propertyId,pRetVal)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,propertyId,pRetVal) ) 

#define IRawElementProviderSimple_get_HostRawElementProvider(This,pRetVal)	\
    ( (This)->lpVtbl -> get_HostRawElementProvider(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderSimple_INTERFACE_DEFINED__ */


#ifndef __IAccessibleEx_INTERFACE_DEFINED__
#define __IAccessibleEx_INTERFACE_DEFINED__

/* interface IAccessibleEx */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IAccessibleEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f8b80ada-2c44-48d0-89be-5ff23c9cd875")
    IAccessibleEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetObjectForChild( 
            /* [in] */ long idChild,
            /* [retval][out] */ __RPC__deref_out_opt IAccessibleEx **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIAccessiblePair( 
            /* [out] */ __RPC__deref_out_opt IAccessible **ppAcc,
            /* [out] */ __RPC__out long *pidChild) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRuntimeId( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertReturnedElement( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pIn,
            /* [out] */ __RPC__deref_out_opt IAccessibleEx **ppRetValOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibleEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibleEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibleEx * This);
        
        DECLSPEC_XFGVIRT(IAccessibleEx, GetObjectForChild)
        HRESULT ( STDMETHODCALLTYPE *GetObjectForChild )( 
            __RPC__in IAccessibleEx * This,
            /* [in] */ long idChild,
            /* [retval][out] */ __RPC__deref_out_opt IAccessibleEx **pRetVal);
        
        DECLSPEC_XFGVIRT(IAccessibleEx, GetIAccessiblePair)
        HRESULT ( STDMETHODCALLTYPE *GetIAccessiblePair )( 
            __RPC__in IAccessibleEx * This,
            /* [out] */ __RPC__deref_out_opt IAccessible **ppAcc,
            /* [out] */ __RPC__out long *pidChild);
        
        DECLSPEC_XFGVIRT(IAccessibleEx, GetRuntimeId)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeId )( 
            __RPC__in IAccessibleEx * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(IAccessibleEx, ConvertReturnedElement)
        HRESULT ( STDMETHODCALLTYPE *ConvertReturnedElement )( 
            __RPC__in IAccessibleEx * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pIn,
            /* [out] */ __RPC__deref_out_opt IAccessibleEx **ppRetValOut);
        
        END_INTERFACE
    } IAccessibleExVtbl;

    interface IAccessibleEx
    {
        CONST_VTBL struct IAccessibleExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleEx_GetObjectForChild(This,idChild,pRetVal)	\
    ( (This)->lpVtbl -> GetObjectForChild(This,idChild,pRetVal) ) 

#define IAccessibleEx_GetIAccessiblePair(This,ppAcc,pidChild)	\
    ( (This)->lpVtbl -> GetIAccessiblePair(This,ppAcc,pidChild) ) 

#define IAccessibleEx_GetRuntimeId(This,pRetVal)	\
    ( (This)->lpVtbl -> GetRuntimeId(This,pRetVal) ) 

#define IAccessibleEx_ConvertReturnedElement(This,pIn,ppRetValOut)	\
    ( (This)->lpVtbl -> ConvertReturnedElement(This,pIn,ppRetValOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleEx_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderSimple2_INTERFACE_DEFINED__
#define __IRawElementProviderSimple2_INTERFACE_DEFINED__

/* interface IRawElementProviderSimple2 */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderSimple2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A0A839A9-8DA1-4A82-806A-8E0D44E79F56")
    IRawElementProviderSimple2 : public IRawElementProviderSimple
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowContextMenu( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderSimple2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderSimple2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderSimple2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderSimple2 * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, get_ProviderOptions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderOptions )( 
            __RPC__in IRawElementProviderSimple2 * This,
            /* [retval][out] */ __RPC__out enum ProviderOptions *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, GetPatternProvider)
        HRESULT ( STDMETHODCALLTYPE *GetPatternProvider )( 
            __RPC__in IRawElementProviderSimple2 * This,
            /* [in] */ PATTERNID patternId,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, GetPropertyValue)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IRawElementProviderSimple2 * This,
            /* [in] */ PROPERTYID propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, get_HostRawElementProvider)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostRawElementProvider )( 
            __RPC__in IRawElementProviderSimple2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple2, ShowContextMenu)
        HRESULT ( STDMETHODCALLTYPE *ShowContextMenu )( 
            __RPC__in IRawElementProviderSimple2 * This);
        
        END_INTERFACE
    } IRawElementProviderSimple2Vtbl;

    interface IRawElementProviderSimple2
    {
        CONST_VTBL struct IRawElementProviderSimple2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderSimple2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderSimple2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderSimple2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderSimple2_get_ProviderOptions(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ProviderOptions(This,pRetVal) ) 

#define IRawElementProviderSimple2_GetPatternProvider(This,patternId,pRetVal)	\
    ( (This)->lpVtbl -> GetPatternProvider(This,patternId,pRetVal) ) 

#define IRawElementProviderSimple2_GetPropertyValue(This,propertyId,pRetVal)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,propertyId,pRetVal) ) 

#define IRawElementProviderSimple2_get_HostRawElementProvider(This,pRetVal)	\
    ( (This)->lpVtbl -> get_HostRawElementProvider(This,pRetVal) ) 


#define IRawElementProviderSimple2_ShowContextMenu(This)	\
    ( (This)->lpVtbl -> ShowContextMenu(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderSimple2_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderSimple3_INTERFACE_DEFINED__
#define __IRawElementProviderSimple3_INTERFACE_DEFINED__

/* interface IRawElementProviderSimple3 */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderSimple3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fcf5d820-d7ec-4613-bdf6-42a84ce7daaf")
    IRawElementProviderSimple3 : public IRawElementProviderSimple2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMetadataValue( 
            /* [in] */ int targetId,
            /* [in] */ METADATAID metadataId,
            /* [retval][out] */ __RPC__out VARIANT *returnVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderSimple3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderSimple3 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderSimple3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderSimple3 * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, get_ProviderOptions)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProviderOptions )( 
            __RPC__in IRawElementProviderSimple3 * This,
            /* [retval][out] */ __RPC__out enum ProviderOptions *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, GetPatternProvider)
        HRESULT ( STDMETHODCALLTYPE *GetPatternProvider )( 
            __RPC__in IRawElementProviderSimple3 * This,
            /* [in] */ PATTERNID patternId,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, GetPropertyValue)
        HRESULT ( STDMETHODCALLTYPE *GetPropertyValue )( 
            __RPC__in IRawElementProviderSimple3 * This,
            /* [in] */ PROPERTYID propertyId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple, get_HostRawElementProvider)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HostRawElementProvider )( 
            __RPC__in IRawElementProviderSimple3 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple2, ShowContextMenu)
        HRESULT ( STDMETHODCALLTYPE *ShowContextMenu )( 
            __RPC__in IRawElementProviderSimple3 * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderSimple3, GetMetadataValue)
        HRESULT ( STDMETHODCALLTYPE *GetMetadataValue )( 
            __RPC__in IRawElementProviderSimple3 * This,
            /* [in] */ int targetId,
            /* [in] */ METADATAID metadataId,
            /* [retval][out] */ __RPC__out VARIANT *returnVal);
        
        END_INTERFACE
    } IRawElementProviderSimple3Vtbl;

    interface IRawElementProviderSimple3
    {
        CONST_VTBL struct IRawElementProviderSimple3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderSimple3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderSimple3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderSimple3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderSimple3_get_ProviderOptions(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ProviderOptions(This,pRetVal) ) 

#define IRawElementProviderSimple3_GetPatternProvider(This,patternId,pRetVal)	\
    ( (This)->lpVtbl -> GetPatternProvider(This,patternId,pRetVal) ) 

#define IRawElementProviderSimple3_GetPropertyValue(This,propertyId,pRetVal)	\
    ( (This)->lpVtbl -> GetPropertyValue(This,propertyId,pRetVal) ) 

#define IRawElementProviderSimple3_get_HostRawElementProvider(This,pRetVal)	\
    ( (This)->lpVtbl -> get_HostRawElementProvider(This,pRetVal) ) 


#define IRawElementProviderSimple3_ShowContextMenu(This)	\
    ( (This)->lpVtbl -> ShowContextMenu(This) ) 


#define IRawElementProviderSimple3_GetMetadataValue(This,targetId,metadataId,returnVal)	\
    ( (This)->lpVtbl -> GetMetadataValue(This,targetId,metadataId,returnVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderSimple3_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderFragmentRoot_INTERFACE_DEFINED__
#define __IRawElementProviderFragmentRoot_INTERFACE_DEFINED__

/* interface IRawElementProviderFragmentRoot */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderFragmentRoot;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("620ce2a5-ab8f-40a9-86cb-de3c75599b58")
    IRawElementProviderFragmentRoot : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ElementProviderFromPoint( 
            /* [in] */ double x,
            /* [in] */ double y,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFocus( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderFragmentRootVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderFragmentRoot * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderFragmentRoot * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderFragmentRoot * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragmentRoot, ElementProviderFromPoint)
        HRESULT ( STDMETHODCALLTYPE *ElementProviderFromPoint )( 
            __RPC__in IRawElementProviderFragmentRoot * This,
            /* [in] */ double x,
            /* [in] */ double y,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragmentRoot, GetFocus)
        HRESULT ( STDMETHODCALLTYPE *GetFocus )( 
            __RPC__in IRawElementProviderFragmentRoot * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **pRetVal);
        
        END_INTERFACE
    } IRawElementProviderFragmentRootVtbl;

    interface IRawElementProviderFragmentRoot
    {
        CONST_VTBL struct IRawElementProviderFragmentRootVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderFragmentRoot_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderFragmentRoot_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderFragmentRoot_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderFragmentRoot_ElementProviderFromPoint(This,x,y,pRetVal)	\
    ( (This)->lpVtbl -> ElementProviderFromPoint(This,x,y,pRetVal) ) 

#define IRawElementProviderFragmentRoot_GetFocus(This,pRetVal)	\
    ( (This)->lpVtbl -> GetFocus(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderFragmentRoot_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderFragment_INTERFACE_DEFINED__
#define __IRawElementProviderFragment_INTERFACE_DEFINED__

/* interface IRawElementProviderFragment */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderFragment;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f7063da8-8359-439c-9297-bbc5299a7d87")
    IRawElementProviderFragment : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Navigate( 
            /* [in] */ enum NavigateDirection direction,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRuntimeId( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BoundingRectangle( 
            /* [retval][out] */ __RPC__out struct UiaRect *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEmbeddedFragmentRoots( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFocus( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FragmentRoot( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragmentRoot **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderFragmentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderFragment * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderFragment * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderFragment * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragment, Navigate)
        HRESULT ( STDMETHODCALLTYPE *Navigate )( 
            __RPC__in IRawElementProviderFragment * This,
            /* [in] */ enum NavigateDirection direction,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragment, GetRuntimeId)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeId )( 
            __RPC__in IRawElementProviderFragment * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragment, get_BoundingRectangle)
        HRESULT ( STDMETHODCALLTYPE *get_BoundingRectangle )( 
            __RPC__in IRawElementProviderFragment * This,
            /* [retval][out] */ __RPC__out struct UiaRect *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragment, GetEmbeddedFragmentRoots)
        HRESULT ( STDMETHODCALLTYPE *GetEmbeddedFragmentRoots )( 
            __RPC__in IRawElementProviderFragment * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragment, SetFocus)
        HRESULT ( STDMETHODCALLTYPE *SetFocus )( 
            __RPC__in IRawElementProviderFragment * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderFragment, get_FragmentRoot)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FragmentRoot )( 
            __RPC__in IRawElementProviderFragment * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragmentRoot **pRetVal);
        
        END_INTERFACE
    } IRawElementProviderFragmentVtbl;

    interface IRawElementProviderFragment
    {
        CONST_VTBL struct IRawElementProviderFragmentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderFragment_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderFragment_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderFragment_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderFragment_Navigate(This,direction,pRetVal)	\
    ( (This)->lpVtbl -> Navigate(This,direction,pRetVal) ) 

#define IRawElementProviderFragment_GetRuntimeId(This,pRetVal)	\
    ( (This)->lpVtbl -> GetRuntimeId(This,pRetVal) ) 

#define IRawElementProviderFragment_get_BoundingRectangle(This,pRetVal)	\
    ( (This)->lpVtbl -> get_BoundingRectangle(This,pRetVal) ) 

#define IRawElementProviderFragment_GetEmbeddedFragmentRoots(This,pRetVal)	\
    ( (This)->lpVtbl -> GetEmbeddedFragmentRoots(This,pRetVal) ) 

#define IRawElementProviderFragment_SetFocus(This)	\
    ( (This)->lpVtbl -> SetFocus(This) ) 

#define IRawElementProviderFragment_get_FragmentRoot(This,pRetVal)	\
    ( (This)->lpVtbl -> get_FragmentRoot(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderFragment_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderAdviseEvents_INTERFACE_DEFINED__
#define __IRawElementProviderAdviseEvents_INTERFACE_DEFINED__

/* interface IRawElementProviderAdviseEvents */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderAdviseEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a407b27b-0f6d-4427-9292-473c7bf93258")
    IRawElementProviderAdviseEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseEventAdded( 
            /* [in] */ EVENTID eventId,
            /* [in] */ __RPC__in SAFEARRAY * propertyIDs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AdviseEventRemoved( 
            /* [in] */ EVENTID eventId,
            /* [in] */ __RPC__in SAFEARRAY * propertyIDs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderAdviseEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderAdviseEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderAdviseEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderAdviseEvents * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderAdviseEvents, AdviseEventAdded)
        HRESULT ( STDMETHODCALLTYPE *AdviseEventAdded )( 
            __RPC__in IRawElementProviderAdviseEvents * This,
            /* [in] */ EVENTID eventId,
            /* [in] */ __RPC__in SAFEARRAY * propertyIDs);
        
        DECLSPEC_XFGVIRT(IRawElementProviderAdviseEvents, AdviseEventRemoved)
        HRESULT ( STDMETHODCALLTYPE *AdviseEventRemoved )( 
            __RPC__in IRawElementProviderAdviseEvents * This,
            /* [in] */ EVENTID eventId,
            /* [in] */ __RPC__in SAFEARRAY * propertyIDs);
        
        END_INTERFACE
    } IRawElementProviderAdviseEventsVtbl;

    interface IRawElementProviderAdviseEvents
    {
        CONST_VTBL struct IRawElementProviderAdviseEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderAdviseEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderAdviseEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderAdviseEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderAdviseEvents_AdviseEventAdded(This,eventId,propertyIDs)	\
    ( (This)->lpVtbl -> AdviseEventAdded(This,eventId,propertyIDs) ) 

#define IRawElementProviderAdviseEvents_AdviseEventRemoved(This,eventId,propertyIDs)	\
    ( (This)->lpVtbl -> AdviseEventRemoved(This,eventId,propertyIDs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderAdviseEvents_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderHwndOverride_INTERFACE_DEFINED__
#define __IRawElementProviderHwndOverride_INTERFACE_DEFINED__

/* interface IRawElementProviderHwndOverride */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderHwndOverride;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1d5df27c-8947-4425-b8d9-79787bb460b8")
    IRawElementProviderHwndOverride : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOverrideProviderForHwnd( 
            /* [in] */ __RPC__in HWND hwnd,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderHwndOverrideVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderHwndOverride * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderHwndOverride * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderHwndOverride * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderHwndOverride, GetOverrideProviderForHwnd)
        HRESULT ( STDMETHODCALLTYPE *GetOverrideProviderForHwnd )( 
            __RPC__in IRawElementProviderHwndOverride * This,
            /* [in] */ __RPC__in HWND hwnd,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        END_INTERFACE
    } IRawElementProviderHwndOverrideVtbl;

    interface IRawElementProviderHwndOverride
    {
        CONST_VTBL struct IRawElementProviderHwndOverrideVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderHwndOverride_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderHwndOverride_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderHwndOverride_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderHwndOverride_GetOverrideProviderForHwnd(This,hwnd,pRetVal)	\
    ( (This)->lpVtbl -> GetOverrideProviderForHwnd(This,hwnd,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderHwndOverride_INTERFACE_DEFINED__ */


#ifndef __IProxyProviderWinEventSink_INTERFACE_DEFINED__
#define __IProxyProviderWinEventSink_INTERFACE_DEFINED__

/* interface IProxyProviderWinEventSink */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IProxyProviderWinEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4fd82b78-a43e-46ac-9803-0a6969c7c183")
    IProxyProviderWinEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddAutomationPropertyChangedEvent( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [in] */ PROPERTYID id,
            /* [in] */ VARIANT newValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddAutomationEvent( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [in] */ EVENTID id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStructureChangedEvent( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [in] */ enum StructureChangeType structureChangeType,
            /* [in] */ __RPC__in SAFEARRAY * runtimeId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProxyProviderWinEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProxyProviderWinEventSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProxyProviderWinEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProxyProviderWinEventSink * This);
        
        DECLSPEC_XFGVIRT(IProxyProviderWinEventSink, AddAutomationPropertyChangedEvent)
        HRESULT ( STDMETHODCALLTYPE *AddAutomationPropertyChangedEvent )( 
            __RPC__in IProxyProviderWinEventSink * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [in] */ PROPERTYID id,
            /* [in] */ VARIANT newValue);
        
        DECLSPEC_XFGVIRT(IProxyProviderWinEventSink, AddAutomationEvent)
        HRESULT ( STDMETHODCALLTYPE *AddAutomationEvent )( 
            __RPC__in IProxyProviderWinEventSink * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [in] */ EVENTID id);
        
        DECLSPEC_XFGVIRT(IProxyProviderWinEventSink, AddStructureChangedEvent)
        HRESULT ( STDMETHODCALLTYPE *AddStructureChangedEvent )( 
            __RPC__in IProxyProviderWinEventSink * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [in] */ enum StructureChangeType structureChangeType,
            /* [in] */ __RPC__in SAFEARRAY * runtimeId);
        
        END_INTERFACE
    } IProxyProviderWinEventSinkVtbl;

    interface IProxyProviderWinEventSink
    {
        CONST_VTBL struct IProxyProviderWinEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProxyProviderWinEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProxyProviderWinEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProxyProviderWinEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProxyProviderWinEventSink_AddAutomationPropertyChangedEvent(This,pProvider,id,newValue)	\
    ( (This)->lpVtbl -> AddAutomationPropertyChangedEvent(This,pProvider,id,newValue) ) 

#define IProxyProviderWinEventSink_AddAutomationEvent(This,pProvider,id)	\
    ( (This)->lpVtbl -> AddAutomationEvent(This,pProvider,id) ) 

#define IProxyProviderWinEventSink_AddStructureChangedEvent(This,pProvider,structureChangeType,runtimeId)	\
    ( (This)->lpVtbl -> AddStructureChangedEvent(This,pProvider,structureChangeType,runtimeId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProxyProviderWinEventSink_INTERFACE_DEFINED__ */


#ifndef __IProxyProviderWinEventHandler_INTERFACE_DEFINED__
#define __IProxyProviderWinEventHandler_INTERFACE_DEFINED__

/* interface IProxyProviderWinEventHandler */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IProxyProviderWinEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89592ad4-f4e0-43d5-a3b6-bad7e111b435")
    IProxyProviderWinEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RespondToWinEvent( 
            /* [in] */ DWORD idWinEvent,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ LONG idObject,
            /* [in] */ LONG idChild,
            /* [in] */ __RPC__in_opt IProxyProviderWinEventSink *pSink) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IProxyProviderWinEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IProxyProviderWinEventHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IProxyProviderWinEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IProxyProviderWinEventHandler * This);
        
        DECLSPEC_XFGVIRT(IProxyProviderWinEventHandler, RespondToWinEvent)
        HRESULT ( STDMETHODCALLTYPE *RespondToWinEvent )( 
            __RPC__in IProxyProviderWinEventHandler * This,
            /* [in] */ DWORD idWinEvent,
            /* [in] */ __RPC__in HWND hwnd,
            /* [in] */ LONG idObject,
            /* [in] */ LONG idChild,
            /* [in] */ __RPC__in_opt IProxyProviderWinEventSink *pSink);
        
        END_INTERFACE
    } IProxyProviderWinEventHandlerVtbl;

    interface IProxyProviderWinEventHandler
    {
        CONST_VTBL struct IProxyProviderWinEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IProxyProviderWinEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IProxyProviderWinEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IProxyProviderWinEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IProxyProviderWinEventHandler_RespondToWinEvent(This,idWinEvent,hwnd,idObject,idChild,pSink)	\
    ( (This)->lpVtbl -> RespondToWinEvent(This,idWinEvent,hwnd,idObject,idChild,pSink) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IProxyProviderWinEventHandler_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderWindowlessSite_INTERFACE_DEFINED__
#define __IRawElementProviderWindowlessSite_INTERFACE_DEFINED__

/* interface IRawElementProviderWindowlessSite */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderWindowlessSite;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0a2a93cc-bfad-42ac-9b2e-0991fb0d3ea0")
    IRawElementProviderWindowlessSite : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAdjacentFragment( 
            /* [in] */ enum NavigateDirection direction,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **ppParent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRuntimeIdPrefix( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderWindowlessSiteVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderWindowlessSite * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderWindowlessSite * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderWindowlessSite * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderWindowlessSite, GetAdjacentFragment)
        HRESULT ( STDMETHODCALLTYPE *GetAdjacentFragment )( 
            __RPC__in IRawElementProviderWindowlessSite * This,
            /* [in] */ enum NavigateDirection direction,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderFragment **ppParent);
        
        DECLSPEC_XFGVIRT(IRawElementProviderWindowlessSite, GetRuntimeIdPrefix)
        HRESULT ( STDMETHODCALLTYPE *GetRuntimeIdPrefix )( 
            __RPC__in IRawElementProviderWindowlessSite * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } IRawElementProviderWindowlessSiteVtbl;

    interface IRawElementProviderWindowlessSite
    {
        CONST_VTBL struct IRawElementProviderWindowlessSiteVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderWindowlessSite_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderWindowlessSite_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderWindowlessSite_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderWindowlessSite_GetAdjacentFragment(This,direction,ppParent)	\
    ( (This)->lpVtbl -> GetAdjacentFragment(This,direction,ppParent) ) 

#define IRawElementProviderWindowlessSite_GetRuntimeIdPrefix(This,pRetVal)	\
    ( (This)->lpVtbl -> GetRuntimeIdPrefix(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderWindowlessSite_INTERFACE_DEFINED__ */


#ifndef __IAccessibleHostingElementProviders_INTERFACE_DEFINED__
#define __IAccessibleHostingElementProviders_INTERFACE_DEFINED__

/* interface IAccessibleHostingElementProviders */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IAccessibleHostingElementProviders;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33AC331B-943E-4020-B295-DB37784974A3")
    IAccessibleHostingElementProviders : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEmbeddedFragmentRoots( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObjectIdForProvider( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [out] */ __RPC__out long *pidObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAccessibleHostingElementProvidersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAccessibleHostingElementProviders * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAccessibleHostingElementProviders * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAccessibleHostingElementProviders * This);
        
        DECLSPEC_XFGVIRT(IAccessibleHostingElementProviders, GetEmbeddedFragmentRoots)
        HRESULT ( STDMETHODCALLTYPE *GetEmbeddedFragmentRoots )( 
            __RPC__in IAccessibleHostingElementProviders * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(IAccessibleHostingElementProviders, GetObjectIdForProvider)
        HRESULT ( STDMETHODCALLTYPE *GetObjectIdForProvider )( 
            __RPC__in IAccessibleHostingElementProviders * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pProvider,
            /* [out] */ __RPC__out long *pidObject);
        
        END_INTERFACE
    } IAccessibleHostingElementProvidersVtbl;

    interface IAccessibleHostingElementProviders
    {
        CONST_VTBL struct IAccessibleHostingElementProvidersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAccessibleHostingElementProviders_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAccessibleHostingElementProviders_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAccessibleHostingElementProviders_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAccessibleHostingElementProviders_GetEmbeddedFragmentRoots(This,pRetVal)	\
    ( (This)->lpVtbl -> GetEmbeddedFragmentRoots(This,pRetVal) ) 

#define IAccessibleHostingElementProviders_GetObjectIdForProvider(This,pProvider,pidObject)	\
    ( (This)->lpVtbl -> GetObjectIdForProvider(This,pProvider,pidObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAccessibleHostingElementProviders_INTERFACE_DEFINED__ */


#ifndef __IRawElementProviderHostingAccessibles_INTERFACE_DEFINED__
#define __IRawElementProviderHostingAccessibles_INTERFACE_DEFINED__

/* interface IRawElementProviderHostingAccessibles */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRawElementProviderHostingAccessibles;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24BE0B07-D37D-487A-98CF-A13ED465E9B3")
    IRawElementProviderHostingAccessibles : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEmbeddedAccessibles( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRawElementProviderHostingAccessiblesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRawElementProviderHostingAccessibles * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRawElementProviderHostingAccessibles * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRawElementProviderHostingAccessibles * This);
        
        DECLSPEC_XFGVIRT(IRawElementProviderHostingAccessibles, GetEmbeddedAccessibles)
        HRESULT ( STDMETHODCALLTYPE *GetEmbeddedAccessibles )( 
            __RPC__in IRawElementProviderHostingAccessibles * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } IRawElementProviderHostingAccessiblesVtbl;

    interface IRawElementProviderHostingAccessibles
    {
        CONST_VTBL struct IRawElementProviderHostingAccessiblesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRawElementProviderHostingAccessibles_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRawElementProviderHostingAccessibles_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRawElementProviderHostingAccessibles_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRawElementProviderHostingAccessibles_GetEmbeddedAccessibles(This,pRetVal)	\
    ( (This)->lpVtbl -> GetEmbeddedAccessibles(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRawElementProviderHostingAccessibles_INTERFACE_DEFINED__ */


#ifndef __IDockProvider_INTERFACE_DEFINED__
#define __IDockProvider_INTERFACE_DEFINED__

/* interface IDockProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IDockProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("159bc72c-4ad3-485e-9637-d7052edf0146")
    IDockProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDockPosition( 
            /* [in] */ enum DockPosition dockPosition) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DockPosition( 
            /* [retval][out] */ __RPC__out enum DockPosition *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDockProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDockProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDockProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDockProvider * This);
        
        DECLSPEC_XFGVIRT(IDockProvider, SetDockPosition)
        HRESULT ( STDMETHODCALLTYPE *SetDockPosition )( 
            __RPC__in IDockProvider * This,
            /* [in] */ enum DockPosition dockPosition);
        
        DECLSPEC_XFGVIRT(IDockProvider, get_DockPosition)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DockPosition )( 
            __RPC__in IDockProvider * This,
            /* [retval][out] */ __RPC__out enum DockPosition *pRetVal);
        
        END_INTERFACE
    } IDockProviderVtbl;

    interface IDockProvider
    {
        CONST_VTBL struct IDockProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDockProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDockProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDockProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDockProvider_SetDockPosition(This,dockPosition)	\
    ( (This)->lpVtbl -> SetDockPosition(This,dockPosition) ) 

#define IDockProvider_get_DockPosition(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DockPosition(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDockProvider_INTERFACE_DEFINED__ */


#ifndef __IExpandCollapseProvider_INTERFACE_DEFINED__
#define __IExpandCollapseProvider_INTERFACE_DEFINED__

/* interface IExpandCollapseProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IExpandCollapseProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d847d3a5-cab0-4a98-8c32-ecb45c59ad24")
    IExpandCollapseProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Expand( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Collapse( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExpandCollapseState( 
            /* [retval][out] */ __RPC__out enum ExpandCollapseState *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IExpandCollapseProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IExpandCollapseProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IExpandCollapseProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IExpandCollapseProvider * This);
        
        DECLSPEC_XFGVIRT(IExpandCollapseProvider, Expand)
        HRESULT ( STDMETHODCALLTYPE *Expand )( 
            __RPC__in IExpandCollapseProvider * This);
        
        DECLSPEC_XFGVIRT(IExpandCollapseProvider, Collapse)
        HRESULT ( STDMETHODCALLTYPE *Collapse )( 
            __RPC__in IExpandCollapseProvider * This);
        
        DECLSPEC_XFGVIRT(IExpandCollapseProvider, get_ExpandCollapseState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExpandCollapseState )( 
            __RPC__in IExpandCollapseProvider * This,
            /* [retval][out] */ __RPC__out enum ExpandCollapseState *pRetVal);
        
        END_INTERFACE
    } IExpandCollapseProviderVtbl;

    interface IExpandCollapseProvider
    {
        CONST_VTBL struct IExpandCollapseProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IExpandCollapseProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IExpandCollapseProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IExpandCollapseProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IExpandCollapseProvider_Expand(This)	\
    ( (This)->lpVtbl -> Expand(This) ) 

#define IExpandCollapseProvider_Collapse(This)	\
    ( (This)->lpVtbl -> Collapse(This) ) 

#define IExpandCollapseProvider_get_ExpandCollapseState(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ExpandCollapseState(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IExpandCollapseProvider_INTERFACE_DEFINED__ */


#ifndef __IGridProvider_INTERFACE_DEFINED__
#define __IGridProvider_INTERFACE_DEFINED__

/* interface IGridProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IGridProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b17d6187-0907-464b-a168-0ef17a1572b1")
    IGridProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetItem( 
            /* [in] */ int row,
            /* [in] */ int column,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RowCount( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ColumnCount( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGridProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGridProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGridProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGridProvider * This);
        
        DECLSPEC_XFGVIRT(IGridProvider, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IGridProvider * This,
            /* [in] */ int row,
            /* [in] */ int column,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        DECLSPEC_XFGVIRT(IGridProvider, get_RowCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RowCount )( 
            __RPC__in IGridProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(IGridProvider, get_ColumnCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ColumnCount )( 
            __RPC__in IGridProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        END_INTERFACE
    } IGridProviderVtbl;

    interface IGridProvider
    {
        CONST_VTBL struct IGridProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGridProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGridProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGridProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGridProvider_GetItem(This,row,column,pRetVal)	\
    ( (This)->lpVtbl -> GetItem(This,row,column,pRetVal) ) 

#define IGridProvider_get_RowCount(This,pRetVal)	\
    ( (This)->lpVtbl -> get_RowCount(This,pRetVal) ) 

#define IGridProvider_get_ColumnCount(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ColumnCount(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGridProvider_INTERFACE_DEFINED__ */


#ifndef __IGridItemProvider_INTERFACE_DEFINED__
#define __IGridItemProvider_INTERFACE_DEFINED__

/* interface IGridItemProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IGridItemProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d02541f1-fb81-4d64-ae32-f520f8a6dbd1")
    IGridItemProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Row( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Column( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RowSpan( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ColumnSpan( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ContainingGrid( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGridItemProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGridItemProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGridItemProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGridItemProvider * This);
        
        DECLSPEC_XFGVIRT(IGridItemProvider, get_Row)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Row )( 
            __RPC__in IGridItemProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(IGridItemProvider, get_Column)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Column )( 
            __RPC__in IGridItemProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(IGridItemProvider, get_RowSpan)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RowSpan )( 
            __RPC__in IGridItemProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(IGridItemProvider, get_ColumnSpan)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ColumnSpan )( 
            __RPC__in IGridItemProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(IGridItemProvider, get_ContainingGrid)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ContainingGrid )( 
            __RPC__in IGridItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        END_INTERFACE
    } IGridItemProviderVtbl;

    interface IGridItemProvider
    {
        CONST_VTBL struct IGridItemProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGridItemProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGridItemProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGridItemProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGridItemProvider_get_Row(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Row(This,pRetVal) ) 

#define IGridItemProvider_get_Column(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Column(This,pRetVal) ) 

#define IGridItemProvider_get_RowSpan(This,pRetVal)	\
    ( (This)->lpVtbl -> get_RowSpan(This,pRetVal) ) 

#define IGridItemProvider_get_ColumnSpan(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ColumnSpan(This,pRetVal) ) 

#define IGridItemProvider_get_ContainingGrid(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ContainingGrid(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGridItemProvider_INTERFACE_DEFINED__ */


#ifndef __IInvokeProvider_INTERFACE_DEFINED__
#define __IInvokeProvider_INTERFACE_DEFINED__

/* interface IInvokeProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IInvokeProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54fcb24b-e18e-47a2-b4d3-eccbe77599a2")
    IInvokeProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Invoke( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInvokeProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IInvokeProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IInvokeProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IInvokeProvider * This);
        
        DECLSPEC_XFGVIRT(IInvokeProvider, Invoke)
        HRESULT ( STDMETHODCALLTYPE *Invoke )( 
            __RPC__in IInvokeProvider * This);
        
        END_INTERFACE
    } IInvokeProviderVtbl;

    interface IInvokeProvider
    {
        CONST_VTBL struct IInvokeProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInvokeProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInvokeProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInvokeProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInvokeProvider_Invoke(This)	\
    ( (This)->lpVtbl -> Invoke(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInvokeProvider_INTERFACE_DEFINED__ */


#ifndef __IMultipleViewProvider_INTERFACE_DEFINED__
#define __IMultipleViewProvider_INTERFACE_DEFINED__

/* interface IMultipleViewProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IMultipleViewProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6278cab1-b556-4a1a-b4e0-418acc523201")
    IMultipleViewProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetViewName( 
            /* [in] */ int viewId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentView( 
            /* [in] */ int viewId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentView( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedViews( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMultipleViewProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMultipleViewProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMultipleViewProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMultipleViewProvider * This);
        
        DECLSPEC_XFGVIRT(IMultipleViewProvider, GetViewName)
        HRESULT ( STDMETHODCALLTYPE *GetViewName )( 
            __RPC__in IMultipleViewProvider * This,
            /* [in] */ int viewId,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IMultipleViewProvider, SetCurrentView)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentView )( 
            __RPC__in IMultipleViewProvider * This,
            /* [in] */ int viewId);
        
        DECLSPEC_XFGVIRT(IMultipleViewProvider, get_CurrentView)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentView )( 
            __RPC__in IMultipleViewProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(IMultipleViewProvider, GetSupportedViews)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedViews )( 
            __RPC__in IMultipleViewProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } IMultipleViewProviderVtbl;

    interface IMultipleViewProvider
    {
        CONST_VTBL struct IMultipleViewProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMultipleViewProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMultipleViewProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMultipleViewProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMultipleViewProvider_GetViewName(This,viewId,pRetVal)	\
    ( (This)->lpVtbl -> GetViewName(This,viewId,pRetVal) ) 

#define IMultipleViewProvider_SetCurrentView(This,viewId)	\
    ( (This)->lpVtbl -> SetCurrentView(This,viewId) ) 

#define IMultipleViewProvider_get_CurrentView(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CurrentView(This,pRetVal) ) 

#define IMultipleViewProvider_GetSupportedViews(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSupportedViews(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMultipleViewProvider_INTERFACE_DEFINED__ */


#ifndef __IRangeValueProvider_INTERFACE_DEFINED__
#define __IRangeValueProvider_INTERFACE_DEFINED__

/* interface IRangeValueProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IRangeValueProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36dc7aef-33e6-4691-afe1-2be7274b3d33")
    IRangeValueProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ double val) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsReadOnly( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Maximum( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Minimum( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LargeChange( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SmallChange( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRangeValueProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IRangeValueProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IRangeValueProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IRangeValueProvider * This);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IRangeValueProvider * This,
            /* [in] */ double val);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IRangeValueProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, get_IsReadOnly)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsReadOnly )( 
            __RPC__in IRangeValueProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, get_Maximum)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Maximum )( 
            __RPC__in IRangeValueProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, get_Minimum)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Minimum )( 
            __RPC__in IRangeValueProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, get_LargeChange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LargeChange )( 
            __RPC__in IRangeValueProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IRangeValueProvider, get_SmallChange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SmallChange )( 
            __RPC__in IRangeValueProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        END_INTERFACE
    } IRangeValueProviderVtbl;

    interface IRangeValueProvider
    {
        CONST_VTBL struct IRangeValueProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRangeValueProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRangeValueProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRangeValueProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRangeValueProvider_SetValue(This,val)	\
    ( (This)->lpVtbl -> SetValue(This,val) ) 

#define IRangeValueProvider_get_Value(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Value(This,pRetVal) ) 

#define IRangeValueProvider_get_IsReadOnly(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsReadOnly(This,pRetVal) ) 

#define IRangeValueProvider_get_Maximum(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Maximum(This,pRetVal) ) 

#define IRangeValueProvider_get_Minimum(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Minimum(This,pRetVal) ) 

#define IRangeValueProvider_get_LargeChange(This,pRetVal)	\
    ( (This)->lpVtbl -> get_LargeChange(This,pRetVal) ) 

#define IRangeValueProvider_get_SmallChange(This,pRetVal)	\
    ( (This)->lpVtbl -> get_SmallChange(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRangeValueProvider_INTERFACE_DEFINED__ */


#ifndef __IScrollItemProvider_INTERFACE_DEFINED__
#define __IScrollItemProvider_INTERFACE_DEFINED__

/* interface IScrollItemProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IScrollItemProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2360c714-4bf1-4b26-ba65-9b21316127eb")
    IScrollItemProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ScrollIntoView( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScrollItemProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScrollItemProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScrollItemProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScrollItemProvider * This);
        
        DECLSPEC_XFGVIRT(IScrollItemProvider, ScrollIntoView)
        HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in IScrollItemProvider * This);
        
        END_INTERFACE
    } IScrollItemProviderVtbl;

    interface IScrollItemProvider
    {
        CONST_VTBL struct IScrollItemProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScrollItemProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScrollItemProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScrollItemProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScrollItemProvider_ScrollIntoView(This)	\
    ( (This)->lpVtbl -> ScrollIntoView(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScrollItemProvider_INTERFACE_DEFINED__ */


#ifndef __ISelectionProvider_INTERFACE_DEFINED__
#define __ISelectionProvider_INTERFACE_DEFINED__

/* interface ISelectionProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ISelectionProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fb8b03af-3bdf-48d4-bd36-1a65793be168")
    ISelectionProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanSelectMultiple( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsSelectionRequired( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISelectionProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISelectionProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISelectionProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISelectionProvider * This);
        
        DECLSPEC_XFGVIRT(ISelectionProvider, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ISelectionProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider, get_CanSelectMultiple)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanSelectMultiple )( 
            __RPC__in ISelectionProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider, get_IsSelectionRequired)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSelectionRequired )( 
            __RPC__in ISelectionProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        END_INTERFACE
    } ISelectionProviderVtbl;

    interface ISelectionProvider
    {
        CONST_VTBL struct ISelectionProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISelectionProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISelectionProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISelectionProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISelectionProvider_GetSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSelection(This,pRetVal) ) 

#define ISelectionProvider_get_CanSelectMultiple(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanSelectMultiple(This,pRetVal) ) 

#define ISelectionProvider_get_IsSelectionRequired(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsSelectionRequired(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISelectionProvider_INTERFACE_DEFINED__ */


#ifndef __ISelectionProvider2_INTERFACE_DEFINED__
#define __ISelectionProvider2_INTERFACE_DEFINED__

/* interface ISelectionProvider2 */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ISelectionProvider2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("14f68475-ee1c-44f6-a869-d239381f0fe7")
    ISelectionProvider2 : public ISelectionProvider
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FirstSelectedItem( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_LastSelectedItem( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CurrentSelectedItem( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ItemCount( 
            /* [retval][out] */ __RPC__out int *retVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISelectionProvider2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISelectionProvider2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISelectionProvider2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISelectionProvider2 * This);
        
        DECLSPEC_XFGVIRT(ISelectionProvider, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider, get_CanSelectMultiple)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanSelectMultiple )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider, get_IsSelectionRequired)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSelectionRequired )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider2, get_FirstSelectedItem)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FirstSelectedItem )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider2, get_LastSelectedItem)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_LastSelectedItem )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider2, get_CurrentSelectedItem)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CurrentSelectedItem )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal);
        
        DECLSPEC_XFGVIRT(ISelectionProvider2, get_ItemCount)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ItemCount )( 
            __RPC__in ISelectionProvider2 * This,
            /* [retval][out] */ __RPC__out int *retVal);
        
        END_INTERFACE
    } ISelectionProvider2Vtbl;

    interface ISelectionProvider2
    {
        CONST_VTBL struct ISelectionProvider2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISelectionProvider2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISelectionProvider2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISelectionProvider2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISelectionProvider2_GetSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSelection(This,pRetVal) ) 

#define ISelectionProvider2_get_CanSelectMultiple(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanSelectMultiple(This,pRetVal) ) 

#define ISelectionProvider2_get_IsSelectionRequired(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsSelectionRequired(This,pRetVal) ) 


#define ISelectionProvider2_get_FirstSelectedItem(This,retVal)	\
    ( (This)->lpVtbl -> get_FirstSelectedItem(This,retVal) ) 

#define ISelectionProvider2_get_LastSelectedItem(This,retVal)	\
    ( (This)->lpVtbl -> get_LastSelectedItem(This,retVal) ) 

#define ISelectionProvider2_get_CurrentSelectedItem(This,retVal)	\
    ( (This)->lpVtbl -> get_CurrentSelectedItem(This,retVal) ) 

#define ISelectionProvider2_get_ItemCount(This,retVal)	\
    ( (This)->lpVtbl -> get_ItemCount(This,retVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISelectionProvider2_INTERFACE_DEFINED__ */


#ifndef __IScrollProvider_INTERFACE_DEFINED__
#define __IScrollProvider_INTERFACE_DEFINED__

/* interface IScrollProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IScrollProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b38b8077-1fc3-42a5-8cae-d40c2215055a")
    IScrollProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Scroll( 
            /* [in] */ enum ScrollAmount horizontalAmount,
            /* [in] */ enum ScrollAmount verticalAmount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetScrollPercent( 
            /* [in] */ double horizontalPercent,
            /* [in] */ double verticalPercent) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HorizontalScrollPercent( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_VerticalScrollPercent( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HorizontalViewSize( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_VerticalViewSize( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_HorizontallyScrollable( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_VerticallyScrollable( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IScrollProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IScrollProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IScrollProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IScrollProvider * This);
        
        DECLSPEC_XFGVIRT(IScrollProvider, Scroll)
        HRESULT ( STDMETHODCALLTYPE *Scroll )( 
            __RPC__in IScrollProvider * This,
            /* [in] */ enum ScrollAmount horizontalAmount,
            /* [in] */ enum ScrollAmount verticalAmount);
        
        DECLSPEC_XFGVIRT(IScrollProvider, SetScrollPercent)
        HRESULT ( STDMETHODCALLTYPE *SetScrollPercent )( 
            __RPC__in IScrollProvider * This,
            /* [in] */ double horizontalPercent,
            /* [in] */ double verticalPercent);
        
        DECLSPEC_XFGVIRT(IScrollProvider, get_HorizontalScrollPercent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HorizontalScrollPercent )( 
            __RPC__in IScrollProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IScrollProvider, get_VerticalScrollPercent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VerticalScrollPercent )( 
            __RPC__in IScrollProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IScrollProvider, get_HorizontalViewSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HorizontalViewSize )( 
            __RPC__in IScrollProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IScrollProvider, get_VerticalViewSize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VerticalViewSize )( 
            __RPC__in IScrollProvider * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(IScrollProvider, get_HorizontallyScrollable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_HorizontallyScrollable )( 
            __RPC__in IScrollProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IScrollProvider, get_VerticallyScrollable)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_VerticallyScrollable )( 
            __RPC__in IScrollProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        END_INTERFACE
    } IScrollProviderVtbl;

    interface IScrollProvider
    {
        CONST_VTBL struct IScrollProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IScrollProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IScrollProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IScrollProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IScrollProvider_Scroll(This,horizontalAmount,verticalAmount)	\
    ( (This)->lpVtbl -> Scroll(This,horizontalAmount,verticalAmount) ) 

#define IScrollProvider_SetScrollPercent(This,horizontalPercent,verticalPercent)	\
    ( (This)->lpVtbl -> SetScrollPercent(This,horizontalPercent,verticalPercent) ) 

#define IScrollProvider_get_HorizontalScrollPercent(This,pRetVal)	\
    ( (This)->lpVtbl -> get_HorizontalScrollPercent(This,pRetVal) ) 

#define IScrollProvider_get_VerticalScrollPercent(This,pRetVal)	\
    ( (This)->lpVtbl -> get_VerticalScrollPercent(This,pRetVal) ) 

#define IScrollProvider_get_HorizontalViewSize(This,pRetVal)	\
    ( (This)->lpVtbl -> get_HorizontalViewSize(This,pRetVal) ) 

#define IScrollProvider_get_VerticalViewSize(This,pRetVal)	\
    ( (This)->lpVtbl -> get_VerticalViewSize(This,pRetVal) ) 

#define IScrollProvider_get_HorizontallyScrollable(This,pRetVal)	\
    ( (This)->lpVtbl -> get_HorizontallyScrollable(This,pRetVal) ) 

#define IScrollProvider_get_VerticallyScrollable(This,pRetVal)	\
    ( (This)->lpVtbl -> get_VerticallyScrollable(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IScrollProvider_INTERFACE_DEFINED__ */


#ifndef __ISelectionItemProvider_INTERFACE_DEFINED__
#define __ISelectionItemProvider_INTERFACE_DEFINED__

/* interface ISelectionItemProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ISelectionItemProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2acad808-b2d4-452d-a407-91ff1ad167b2")
    ISelectionItemProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Select( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddToSelection( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveFromSelection( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsSelected( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SelectionContainer( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISelectionItemProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISelectionItemProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISelectionItemProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISelectionItemProvider * This);
        
        DECLSPEC_XFGVIRT(ISelectionItemProvider, Select)
        HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ISelectionItemProvider * This);
        
        DECLSPEC_XFGVIRT(ISelectionItemProvider, AddToSelection)
        HRESULT ( STDMETHODCALLTYPE *AddToSelection )( 
            __RPC__in ISelectionItemProvider * This);
        
        DECLSPEC_XFGVIRT(ISelectionItemProvider, RemoveFromSelection)
        HRESULT ( STDMETHODCALLTYPE *RemoveFromSelection )( 
            __RPC__in ISelectionItemProvider * This);
        
        DECLSPEC_XFGVIRT(ISelectionItemProvider, get_IsSelected)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsSelected )( 
            __RPC__in ISelectionItemProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ISelectionItemProvider, get_SelectionContainer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SelectionContainer )( 
            __RPC__in ISelectionItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        END_INTERFACE
    } ISelectionItemProviderVtbl;

    interface ISelectionItemProvider
    {
        CONST_VTBL struct ISelectionItemProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISelectionItemProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISelectionItemProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISelectionItemProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISelectionItemProvider_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ISelectionItemProvider_AddToSelection(This)	\
    ( (This)->lpVtbl -> AddToSelection(This) ) 

#define ISelectionItemProvider_RemoveFromSelection(This)	\
    ( (This)->lpVtbl -> RemoveFromSelection(This) ) 

#define ISelectionItemProvider_get_IsSelected(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsSelected(This,pRetVal) ) 

#define ISelectionItemProvider_get_SelectionContainer(This,pRetVal)	\
    ( (This)->lpVtbl -> get_SelectionContainer(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISelectionItemProvider_INTERFACE_DEFINED__ */


#ifndef __ISynchronizedInputProvider_INTERFACE_DEFINED__
#define __ISynchronizedInputProvider_INTERFACE_DEFINED__

/* interface ISynchronizedInputProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ISynchronizedInputProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29db1a06-02ce-4cf7-9b42-565d4fab20ee")
    ISynchronizedInputProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartListening( 
            /* [in] */ enum SynchronizedInputType inputType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISynchronizedInputProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISynchronizedInputProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISynchronizedInputProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISynchronizedInputProvider * This);
        
        DECLSPEC_XFGVIRT(ISynchronizedInputProvider, StartListening)
        HRESULT ( STDMETHODCALLTYPE *StartListening )( 
            __RPC__in ISynchronizedInputProvider * This,
            /* [in] */ enum SynchronizedInputType inputType);
        
        DECLSPEC_XFGVIRT(ISynchronizedInputProvider, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in ISynchronizedInputProvider * This);
        
        END_INTERFACE
    } ISynchronizedInputProviderVtbl;

    interface ISynchronizedInputProvider
    {
        CONST_VTBL struct ISynchronizedInputProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISynchronizedInputProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISynchronizedInputProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISynchronizedInputProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISynchronizedInputProvider_StartListening(This,inputType)	\
    ( (This)->lpVtbl -> StartListening(This,inputType) ) 

#define ISynchronizedInputProvider_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISynchronizedInputProvider_INTERFACE_DEFINED__ */


#ifndef __ITableProvider_INTERFACE_DEFINED__
#define __ITableProvider_INTERFACE_DEFINED__

/* interface ITableProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITableProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9c860395-97b3-490a-b52a-858cc22af166")
    ITableProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRowHeaders( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnHeaders( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_RowOrColumnMajor( 
            /* [retval][out] */ __RPC__out enum RowOrColumnMajor *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITableProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITableProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITableProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITableProvider * This);
        
        DECLSPEC_XFGVIRT(ITableProvider, GetRowHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetRowHeaders )( 
            __RPC__in ITableProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITableProvider, GetColumnHeaders)
        HRESULT ( STDMETHODCALLTYPE *GetColumnHeaders )( 
            __RPC__in ITableProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITableProvider, get_RowOrColumnMajor)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_RowOrColumnMajor )( 
            __RPC__in ITableProvider * This,
            /* [retval][out] */ __RPC__out enum RowOrColumnMajor *pRetVal);
        
        END_INTERFACE
    } ITableProviderVtbl;

    interface ITableProvider
    {
        CONST_VTBL struct ITableProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITableProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITableProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITableProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITableProvider_GetRowHeaders(This,pRetVal)	\
    ( (This)->lpVtbl -> GetRowHeaders(This,pRetVal) ) 

#define ITableProvider_GetColumnHeaders(This,pRetVal)	\
    ( (This)->lpVtbl -> GetColumnHeaders(This,pRetVal) ) 

#define ITableProvider_get_RowOrColumnMajor(This,pRetVal)	\
    ( (This)->lpVtbl -> get_RowOrColumnMajor(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITableProvider_INTERFACE_DEFINED__ */


#ifndef __ITableItemProvider_INTERFACE_DEFINED__
#define __ITableItemProvider_INTERFACE_DEFINED__

/* interface ITableItemProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITableItemProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b9734fa6-771f-4d78-9c90-2517999349cd")
    ITableItemProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRowHeaderItems( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColumnHeaderItems( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITableItemProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITableItemProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITableItemProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITableItemProvider * This);
        
        DECLSPEC_XFGVIRT(ITableItemProvider, GetRowHeaderItems)
        HRESULT ( STDMETHODCALLTYPE *GetRowHeaderItems )( 
            __RPC__in ITableItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITableItemProvider, GetColumnHeaderItems)
        HRESULT ( STDMETHODCALLTYPE *GetColumnHeaderItems )( 
            __RPC__in ITableItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } ITableItemProviderVtbl;

    interface ITableItemProvider
    {
        CONST_VTBL struct ITableItemProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITableItemProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITableItemProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITableItemProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITableItemProvider_GetRowHeaderItems(This,pRetVal)	\
    ( (This)->lpVtbl -> GetRowHeaderItems(This,pRetVal) ) 

#define ITableItemProvider_GetColumnHeaderItems(This,pRetVal)	\
    ( (This)->lpVtbl -> GetColumnHeaderItems(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITableItemProvider_INTERFACE_DEFINED__ */


#ifndef __IToggleProvider_INTERFACE_DEFINED__
#define __IToggleProvider_INTERFACE_DEFINED__

/* interface IToggleProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IToggleProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56d00bd0-c4f4-433c-a836-1a52a57e0892")
    IToggleProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Toggle( void) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ToggleState( 
            /* [retval][out] */ __RPC__out enum ToggleState *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IToggleProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IToggleProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IToggleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IToggleProvider * This);
        
        DECLSPEC_XFGVIRT(IToggleProvider, Toggle)
        HRESULT ( STDMETHODCALLTYPE *Toggle )( 
            __RPC__in IToggleProvider * This);
        
        DECLSPEC_XFGVIRT(IToggleProvider, get_ToggleState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ToggleState )( 
            __RPC__in IToggleProvider * This,
            /* [retval][out] */ __RPC__out enum ToggleState *pRetVal);
        
        END_INTERFACE
    } IToggleProviderVtbl;

    interface IToggleProvider
    {
        CONST_VTBL struct IToggleProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IToggleProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IToggleProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IToggleProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IToggleProvider_Toggle(This)	\
    ( (This)->lpVtbl -> Toggle(This) ) 

#define IToggleProvider_get_ToggleState(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ToggleState(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IToggleProvider_INTERFACE_DEFINED__ */


#ifndef __ITransformProvider_INTERFACE_DEFINED__
#define __ITransformProvider_INTERFACE_DEFINED__

/* interface ITransformProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITransformProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6829ddc4-4f91-4ffa-b86f-bd3e2987cb4c")
    ITransformProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ double x,
            /* [in] */ double y) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resize( 
            /* [in] */ double width,
            /* [in] */ double height) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Rotate( 
            /* [in] */ double degrees) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanMove( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanResize( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanRotate( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransformProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransformProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransformProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransformProvider * This);
        
        DECLSPEC_XFGVIRT(ITransformProvider, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITransformProvider * This,
            /* [in] */ double x,
            /* [in] */ double y);
        
        DECLSPEC_XFGVIRT(ITransformProvider, Resize)
        HRESULT ( STDMETHODCALLTYPE *Resize )( 
            __RPC__in ITransformProvider * This,
            /* [in] */ double width,
            /* [in] */ double height);
        
        DECLSPEC_XFGVIRT(ITransformProvider, Rotate)
        HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            __RPC__in ITransformProvider * This,
            /* [in] */ double degrees);
        
        DECLSPEC_XFGVIRT(ITransformProvider, get_CanMove)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanMove )( 
            __RPC__in ITransformProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider, get_CanResize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanResize )( 
            __RPC__in ITransformProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider, get_CanRotate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanRotate )( 
            __RPC__in ITransformProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        END_INTERFACE
    } ITransformProviderVtbl;

    interface ITransformProvider
    {
        CONST_VTBL struct ITransformProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransformProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransformProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransformProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransformProvider_Move(This,x,y)	\
    ( (This)->lpVtbl -> Move(This,x,y) ) 

#define ITransformProvider_Resize(This,width,height)	\
    ( (This)->lpVtbl -> Resize(This,width,height) ) 

#define ITransformProvider_Rotate(This,degrees)	\
    ( (This)->lpVtbl -> Rotate(This,degrees) ) 

#define ITransformProvider_get_CanMove(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanMove(This,pRetVal) ) 

#define ITransformProvider_get_CanResize(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanResize(This,pRetVal) ) 

#define ITransformProvider_get_CanRotate(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanRotate(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransformProvider_INTERFACE_DEFINED__ */


#ifndef __IValueProvider_INTERFACE_DEFINED__
#define __IValueProvider_INTERFACE_DEFINED__

/* interface IValueProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IValueProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c7935180-6fb3-4201-b174-7df73adbf64a")
    IValueProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ __RPC__in LPCWSTR val) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsReadOnly( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IValueProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IValueProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IValueProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IValueProvider * This);
        
        DECLSPEC_XFGVIRT(IValueProvider, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in IValueProvider * This,
            /* [in] */ __RPC__in LPCWSTR val);
        
        DECLSPEC_XFGVIRT(IValueProvider, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in IValueProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IValueProvider, get_IsReadOnly)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsReadOnly )( 
            __RPC__in IValueProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        END_INTERFACE
    } IValueProviderVtbl;

    interface IValueProvider
    {
        CONST_VTBL struct IValueProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IValueProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IValueProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IValueProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IValueProvider_SetValue(This,val)	\
    ( (This)->lpVtbl -> SetValue(This,val) ) 

#define IValueProvider_get_Value(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Value(This,pRetVal) ) 

#define IValueProvider_get_IsReadOnly(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsReadOnly(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IValueProvider_INTERFACE_DEFINED__ */


#ifndef __IWindowProvider_INTERFACE_DEFINED__
#define __IWindowProvider_INTERFACE_DEFINED__

/* interface IWindowProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IWindowProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("987df77b-db06-4d77-8f8a-86a9c3bb90b9")
    IWindowProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetVisualState( 
            /* [in] */ enum WindowVisualState state) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitForInputIdle( 
            /* [in] */ int milliseconds,
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanMaximize( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanMinimize( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsModal( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowVisualState( 
            /* [retval][out] */ __RPC__out enum WindowVisualState *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_WindowInteractionState( 
            /* [retval][out] */ __RPC__out enum WindowInteractionState *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsTopmost( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IWindowProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IWindowProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IWindowProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IWindowProvider * This);
        
        DECLSPEC_XFGVIRT(IWindowProvider, SetVisualState)
        HRESULT ( STDMETHODCALLTYPE *SetVisualState )( 
            __RPC__in IWindowProvider * This,
            /* [in] */ enum WindowVisualState state);
        
        DECLSPEC_XFGVIRT(IWindowProvider, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IWindowProvider * This);
        
        DECLSPEC_XFGVIRT(IWindowProvider, WaitForInputIdle)
        HRESULT ( STDMETHODCALLTYPE *WaitForInputIdle )( 
            __RPC__in IWindowProvider * This,
            /* [in] */ int milliseconds,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IWindowProvider, get_CanMaximize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanMaximize )( 
            __RPC__in IWindowProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IWindowProvider, get_CanMinimize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanMinimize )( 
            __RPC__in IWindowProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IWindowProvider, get_IsModal)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsModal )( 
            __RPC__in IWindowProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IWindowProvider, get_WindowVisualState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowVisualState )( 
            __RPC__in IWindowProvider * This,
            /* [retval][out] */ __RPC__out enum WindowVisualState *pRetVal);
        
        DECLSPEC_XFGVIRT(IWindowProvider, get_WindowInteractionState)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_WindowInteractionState )( 
            __RPC__in IWindowProvider * This,
            /* [retval][out] */ __RPC__out enum WindowInteractionState *pRetVal);
        
        DECLSPEC_XFGVIRT(IWindowProvider, get_IsTopmost)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsTopmost )( 
            __RPC__in IWindowProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        END_INTERFACE
    } IWindowProviderVtbl;

    interface IWindowProvider
    {
        CONST_VTBL struct IWindowProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IWindowProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IWindowProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IWindowProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IWindowProvider_SetVisualState(This,state)	\
    ( (This)->lpVtbl -> SetVisualState(This,state) ) 

#define IWindowProvider_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IWindowProvider_WaitForInputIdle(This,milliseconds,pRetVal)	\
    ( (This)->lpVtbl -> WaitForInputIdle(This,milliseconds,pRetVal) ) 

#define IWindowProvider_get_CanMaximize(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanMaximize(This,pRetVal) ) 

#define IWindowProvider_get_CanMinimize(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanMinimize(This,pRetVal) ) 

#define IWindowProvider_get_IsModal(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsModal(This,pRetVal) ) 

#define IWindowProvider_get_WindowVisualState(This,pRetVal)	\
    ( (This)->lpVtbl -> get_WindowVisualState(This,pRetVal) ) 

#define IWindowProvider_get_WindowInteractionState(This,pRetVal)	\
    ( (This)->lpVtbl -> get_WindowInteractionState(This,pRetVal) ) 

#define IWindowProvider_get_IsTopmost(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsTopmost(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IWindowProvider_INTERFACE_DEFINED__ */


#ifndef __ILegacyIAccessibleProvider_INTERFACE_DEFINED__
#define __ILegacyIAccessibleProvider_INTERFACE_DEFINED__

/* interface ILegacyIAccessibleProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ILegacyIAccessibleProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e44c3566-915d-4070-99c6-047bff5a08f5")
    ILegacyIAccessibleProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Select( 
            long flagsSelect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DoDefaultAction( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            __RPC__in LPCWSTR szValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIAccessible( 
            /* [retval][out] */ __RPC__deref_out_opt IAccessible **ppAccessible) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ChildId( 
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Name( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszName) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Value( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszValue) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Description( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDescription) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Role( 
            /* [retval][out] */ __RPC__out DWORD *pdwRole) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_State( 
            /* [retval][out] */ __RPC__out DWORD *pdwState) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Help( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszHelp) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_KeyboardShortcut( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszKeyboardShortcut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pvarSelectedChildren) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DefaultAction( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDefaultAction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ILegacyIAccessibleProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ILegacyIAccessibleProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ILegacyIAccessibleProvider * This);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, Select)
        HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            long flagsSelect);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, DoDefaultAction)
        HRESULT ( STDMETHODCALLTYPE *DoDefaultAction )( 
            __RPC__in ILegacyIAccessibleProvider * This);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            __RPC__in LPCWSTR szValue);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, GetIAccessible)
        HRESULT ( STDMETHODCALLTYPE *GetIAccessible )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IAccessible **ppAccessible);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_ChildId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ChildId )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_Name)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Name )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszName);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_Value)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Value )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszValue);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_Description)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Description )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDescription);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_Role)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Role )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__out DWORD *pdwRole);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_State)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_State )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__out DWORD *pdwState);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_Help)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Help )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszHelp);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_KeyboardShortcut)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_KeyboardShortcut )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszKeyboardShortcut);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pvarSelectedChildren);
        
        DECLSPEC_XFGVIRT(ILegacyIAccessibleProvider, get_DefaultAction)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DefaultAction )( 
            __RPC__in ILegacyIAccessibleProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pszDefaultAction);
        
        END_INTERFACE
    } ILegacyIAccessibleProviderVtbl;

    interface ILegacyIAccessibleProvider
    {
        CONST_VTBL struct ILegacyIAccessibleProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ILegacyIAccessibleProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ILegacyIAccessibleProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ILegacyIAccessibleProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ILegacyIAccessibleProvider_Select(This,flagsSelect)	\
    ( (This)->lpVtbl -> Select(This,flagsSelect) ) 

#define ILegacyIAccessibleProvider_DoDefaultAction(This)	\
    ( (This)->lpVtbl -> DoDefaultAction(This) ) 

#define ILegacyIAccessibleProvider_SetValue(This,szValue)	\
    ( (This)->lpVtbl -> SetValue(This,szValue) ) 

#define ILegacyIAccessibleProvider_GetIAccessible(This,ppAccessible)	\
    ( (This)->lpVtbl -> GetIAccessible(This,ppAccessible) ) 

#define ILegacyIAccessibleProvider_get_ChildId(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ChildId(This,pRetVal) ) 

#define ILegacyIAccessibleProvider_get_Name(This,pszName)	\
    ( (This)->lpVtbl -> get_Name(This,pszName) ) 

#define ILegacyIAccessibleProvider_get_Value(This,pszValue)	\
    ( (This)->lpVtbl -> get_Value(This,pszValue) ) 

#define ILegacyIAccessibleProvider_get_Description(This,pszDescription)	\
    ( (This)->lpVtbl -> get_Description(This,pszDescription) ) 

#define ILegacyIAccessibleProvider_get_Role(This,pdwRole)	\
    ( (This)->lpVtbl -> get_Role(This,pdwRole) ) 

#define ILegacyIAccessibleProvider_get_State(This,pdwState)	\
    ( (This)->lpVtbl -> get_State(This,pdwState) ) 

#define ILegacyIAccessibleProvider_get_Help(This,pszHelp)	\
    ( (This)->lpVtbl -> get_Help(This,pszHelp) ) 

#define ILegacyIAccessibleProvider_get_KeyboardShortcut(This,pszKeyboardShortcut)	\
    ( (This)->lpVtbl -> get_KeyboardShortcut(This,pszKeyboardShortcut) ) 

#define ILegacyIAccessibleProvider_GetSelection(This,pvarSelectedChildren)	\
    ( (This)->lpVtbl -> GetSelection(This,pvarSelectedChildren) ) 

#define ILegacyIAccessibleProvider_get_DefaultAction(This,pszDefaultAction)	\
    ( (This)->lpVtbl -> get_DefaultAction(This,pszDefaultAction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ILegacyIAccessibleProvider_INTERFACE_DEFINED__ */


#ifndef __IItemContainerProvider_INTERFACE_DEFINED__
#define __IItemContainerProvider_INTERFACE_DEFINED__

/* interface IItemContainerProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IItemContainerProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e747770b-39ce-4382-ab30-d8fb3f336f24")
    IItemContainerProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindItemByProperty( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pStartAfter,
            /* [in] */ PROPERTYID propertyId,
            /* [in] */ VARIANT value,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pFound) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IItemContainerProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IItemContainerProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IItemContainerProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IItemContainerProvider * This);
        
        DECLSPEC_XFGVIRT(IItemContainerProvider, FindItemByProperty)
        HRESULT ( STDMETHODCALLTYPE *FindItemByProperty )( 
            __RPC__in IItemContainerProvider * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *pStartAfter,
            /* [in] */ PROPERTYID propertyId,
            /* [in] */ VARIANT value,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pFound);
        
        END_INTERFACE
    } IItemContainerProviderVtbl;

    interface IItemContainerProvider
    {
        CONST_VTBL struct IItemContainerProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IItemContainerProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IItemContainerProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IItemContainerProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IItemContainerProvider_FindItemByProperty(This,pStartAfter,propertyId,value,pFound)	\
    ( (This)->lpVtbl -> FindItemByProperty(This,pStartAfter,propertyId,value,pFound) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IItemContainerProvider_INTERFACE_DEFINED__ */


#ifndef __IVirtualizedItemProvider_INTERFACE_DEFINED__
#define __IVirtualizedItemProvider_INTERFACE_DEFINED__

/* interface IVirtualizedItemProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IVirtualizedItemProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cb98b665-2d35-4fac-ad35-f3c60d0c0b8b")
    IVirtualizedItemProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Realize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVirtualizedItemProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IVirtualizedItemProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IVirtualizedItemProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IVirtualizedItemProvider * This);
        
        DECLSPEC_XFGVIRT(IVirtualizedItemProvider, Realize)
        HRESULT ( STDMETHODCALLTYPE *Realize )( 
            __RPC__in IVirtualizedItemProvider * This);
        
        END_INTERFACE
    } IVirtualizedItemProviderVtbl;

    interface IVirtualizedItemProvider
    {
        CONST_VTBL struct IVirtualizedItemProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVirtualizedItemProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVirtualizedItemProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVirtualizedItemProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVirtualizedItemProvider_Realize(This)	\
    ( (This)->lpVtbl -> Realize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVirtualizedItemProvider_INTERFACE_DEFINED__ */


#ifndef __IObjectModelProvider_INTERFACE_DEFINED__
#define __IObjectModelProvider_INTERFACE_DEFINED__

/* interface IObjectModelProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IObjectModelProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3ad86ebd-f5ef-483d-bb18-b1042a475d64")
    IObjectModelProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUnderlyingObjectModel( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnknown) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IObjectModelProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IObjectModelProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IObjectModelProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IObjectModelProvider * This);
        
        DECLSPEC_XFGVIRT(IObjectModelProvider, GetUnderlyingObjectModel)
        HRESULT ( STDMETHODCALLTYPE *GetUnderlyingObjectModel )( 
            __RPC__in IObjectModelProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppUnknown);
        
        END_INTERFACE
    } IObjectModelProviderVtbl;

    interface IObjectModelProvider
    {
        CONST_VTBL struct IObjectModelProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IObjectModelProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IObjectModelProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IObjectModelProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IObjectModelProvider_GetUnderlyingObjectModel(This,ppUnknown)	\
    ( (This)->lpVtbl -> GetUnderlyingObjectModel(This,ppUnknown) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IObjectModelProvider_INTERFACE_DEFINED__ */


#ifndef __IAnnotationProvider_INTERFACE_DEFINED__
#define __IAnnotationProvider_INTERFACE_DEFINED__

/* interface IAnnotationProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IAnnotationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f95c7e80-bd63-4601-9782-445ebff011fc")
    IAnnotationProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AnnotationTypeId( 
            /* [retval][out] */ __RPC__out int *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_AnnotationTypeName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Author( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DateTime( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Target( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAnnotationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAnnotationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAnnotationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAnnotationProvider * This);
        
        DECLSPEC_XFGVIRT(IAnnotationProvider, get_AnnotationTypeId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AnnotationTypeId )( 
            __RPC__in IAnnotationProvider * This,
            /* [retval][out] */ __RPC__out int *retVal);
        
        DECLSPEC_XFGVIRT(IAnnotationProvider, get_AnnotationTypeName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_AnnotationTypeName )( 
            __RPC__in IAnnotationProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        DECLSPEC_XFGVIRT(IAnnotationProvider, get_Author)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Author )( 
            __RPC__in IAnnotationProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        DECLSPEC_XFGVIRT(IAnnotationProvider, get_DateTime)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DateTime )( 
            __RPC__in IAnnotationProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        DECLSPEC_XFGVIRT(IAnnotationProvider, get_Target)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Target )( 
            __RPC__in IAnnotationProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **retVal);
        
        END_INTERFACE
    } IAnnotationProviderVtbl;

    interface IAnnotationProvider
    {
        CONST_VTBL struct IAnnotationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAnnotationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAnnotationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAnnotationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAnnotationProvider_get_AnnotationTypeId(This,retVal)	\
    ( (This)->lpVtbl -> get_AnnotationTypeId(This,retVal) ) 

#define IAnnotationProvider_get_AnnotationTypeName(This,retVal)	\
    ( (This)->lpVtbl -> get_AnnotationTypeName(This,retVal) ) 

#define IAnnotationProvider_get_Author(This,retVal)	\
    ( (This)->lpVtbl -> get_Author(This,retVal) ) 

#define IAnnotationProvider_get_DateTime(This,retVal)	\
    ( (This)->lpVtbl -> get_DateTime(This,retVal) ) 

#define IAnnotationProvider_get_Target(This,retVal)	\
    ( (This)->lpVtbl -> get_Target(This,retVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAnnotationProvider_INTERFACE_DEFINED__ */


#ifndef __IStylesProvider_INTERFACE_DEFINED__
#define __IStylesProvider_INTERFACE_DEFINED__

/* interface IStylesProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IStylesProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19b6b649-f5d7-4a6d-bdcb-129252be588a")
    IStylesProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StyleId( 
            /* [retval][out] */ __RPC__out int *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_StyleName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FillColor( 
            /* [retval][out] */ __RPC__out int *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FillPatternStyle( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Shape( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_FillPatternColor( 
            /* [retval][out] */ __RPC__out int *retVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ExtendedProperties( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStylesProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IStylesProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IStylesProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IStylesProvider * This);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_StyleId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StyleId )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__out int *retVal);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_StyleName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_StyleName )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_FillColor)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FillColor )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__out int *retVal);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_FillPatternStyle)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FillPatternStyle )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_Shape)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Shape )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_FillPatternColor)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_FillPatternColor )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__out int *retVal);
        
        DECLSPEC_XFGVIRT(IStylesProvider, get_ExtendedProperties)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ExtendedProperties )( 
            __RPC__in IStylesProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *retVal);
        
        END_INTERFACE
    } IStylesProviderVtbl;

    interface IStylesProvider
    {
        CONST_VTBL struct IStylesProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStylesProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStylesProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStylesProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStylesProvider_get_StyleId(This,retVal)	\
    ( (This)->lpVtbl -> get_StyleId(This,retVal) ) 

#define IStylesProvider_get_StyleName(This,retVal)	\
    ( (This)->lpVtbl -> get_StyleName(This,retVal) ) 

#define IStylesProvider_get_FillColor(This,retVal)	\
    ( (This)->lpVtbl -> get_FillColor(This,retVal) ) 

#define IStylesProvider_get_FillPatternStyle(This,retVal)	\
    ( (This)->lpVtbl -> get_FillPatternStyle(This,retVal) ) 

#define IStylesProvider_get_Shape(This,retVal)	\
    ( (This)->lpVtbl -> get_Shape(This,retVal) ) 

#define IStylesProvider_get_FillPatternColor(This,retVal)	\
    ( (This)->lpVtbl -> get_FillPatternColor(This,retVal) ) 

#define IStylesProvider_get_ExtendedProperties(This,retVal)	\
    ( (This)->lpVtbl -> get_ExtendedProperties(This,retVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStylesProvider_INTERFACE_DEFINED__ */


#ifndef __ISpreadsheetProvider_INTERFACE_DEFINED__
#define __ISpreadsheetProvider_INTERFACE_DEFINED__

/* interface ISpreadsheetProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpreadsheetProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6f6b5d35-5525-4f80-b758-85473832ffc7")
    ISpreadsheetProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetItemByName( 
            /* [in] */ __RPC__in LPCWSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpreadsheetProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpreadsheetProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpreadsheetProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpreadsheetProvider * This);
        
        DECLSPEC_XFGVIRT(ISpreadsheetProvider, GetItemByName)
        HRESULT ( STDMETHODCALLTYPE *GetItemByName )( 
            __RPC__in ISpreadsheetProvider * This,
            /* [in] */ __RPC__in LPCWSTR name,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        END_INTERFACE
    } ISpreadsheetProviderVtbl;

    interface ISpreadsheetProvider
    {
        CONST_VTBL struct ISpreadsheetProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpreadsheetProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpreadsheetProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpreadsheetProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpreadsheetProvider_GetItemByName(This,name,pRetVal)	\
    ( (This)->lpVtbl -> GetItemByName(This,name,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpreadsheetProvider_INTERFACE_DEFINED__ */


#ifndef __ISpreadsheetItemProvider_INTERFACE_DEFINED__
#define __ISpreadsheetItemProvider_INTERFACE_DEFINED__

/* interface ISpreadsheetItemProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpreadsheetItemProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("eaed4660-7b3d-4879-a2e6-365ce603f3d0")
    ISpreadsheetItemProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Formula( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAnnotationObjects( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAnnotationTypes( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpreadsheetItemProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ISpreadsheetItemProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ISpreadsheetItemProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ISpreadsheetItemProvider * This);
        
        DECLSPEC_XFGVIRT(ISpreadsheetItemProvider, get_Formula)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Formula )( 
            __RPC__in ISpreadsheetItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(ISpreadsheetItemProvider, GetAnnotationObjects)
        HRESULT ( STDMETHODCALLTYPE *GetAnnotationObjects )( 
            __RPC__in ISpreadsheetItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ISpreadsheetItemProvider, GetAnnotationTypes)
        HRESULT ( STDMETHODCALLTYPE *GetAnnotationTypes )( 
            __RPC__in ISpreadsheetItemProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } ISpreadsheetItemProviderVtbl;

    interface ISpreadsheetItemProvider
    {
        CONST_VTBL struct ISpreadsheetItemProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpreadsheetItemProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpreadsheetItemProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpreadsheetItemProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpreadsheetItemProvider_get_Formula(This,pRetVal)	\
    ( (This)->lpVtbl -> get_Formula(This,pRetVal) ) 

#define ISpreadsheetItemProvider_GetAnnotationObjects(This,pRetVal)	\
    ( (This)->lpVtbl -> GetAnnotationObjects(This,pRetVal) ) 

#define ISpreadsheetItemProvider_GetAnnotationTypes(This,pRetVal)	\
    ( (This)->lpVtbl -> GetAnnotationTypes(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpreadsheetItemProvider_INTERFACE_DEFINED__ */


#ifndef __ITransformProvider2_INTERFACE_DEFINED__
#define __ITransformProvider2_INTERFACE_DEFINED__

/* interface ITransformProvider2 */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITransformProvider2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4758742f-7ac2-460c-bc48-09fc09308a93")
    ITransformProvider2 : public ITransformProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Zoom( 
            /* [in] */ double zoom) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_CanZoom( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ZoomLevel( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ZoomMinimum( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ZoomMaximum( 
            /* [retval][out] */ __RPC__out double *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ZoomByUnit( 
            /* [in] */ enum ZoomUnit zoomUnit) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITransformProvider2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITransformProvider2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITransformProvider2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITransformProvider2 * This);
        
        DECLSPEC_XFGVIRT(ITransformProvider, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITransformProvider2 * This,
            /* [in] */ double x,
            /* [in] */ double y);
        
        DECLSPEC_XFGVIRT(ITransformProvider, Resize)
        HRESULT ( STDMETHODCALLTYPE *Resize )( 
            __RPC__in ITransformProvider2 * This,
            /* [in] */ double width,
            /* [in] */ double height);
        
        DECLSPEC_XFGVIRT(ITransformProvider, Rotate)
        HRESULT ( STDMETHODCALLTYPE *Rotate )( 
            __RPC__in ITransformProvider2 * This,
            /* [in] */ double degrees);
        
        DECLSPEC_XFGVIRT(ITransformProvider, get_CanMove)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanMove )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider, get_CanResize)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanResize )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider, get_CanRotate)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanRotate )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider2, Zoom)
        HRESULT ( STDMETHODCALLTYPE *Zoom )( 
            __RPC__in ITransformProvider2 * This,
            /* [in] */ double zoom);
        
        DECLSPEC_XFGVIRT(ITransformProvider2, get_CanZoom)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_CanZoom )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider2, get_ZoomLevel)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ZoomLevel )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider2, get_ZoomMinimum)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ZoomMinimum )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider2, get_ZoomMaximum)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ZoomMaximum )( 
            __RPC__in ITransformProvider2 * This,
            /* [retval][out] */ __RPC__out double *pRetVal);
        
        DECLSPEC_XFGVIRT(ITransformProvider2, ZoomByUnit)
        HRESULT ( STDMETHODCALLTYPE *ZoomByUnit )( 
            __RPC__in ITransformProvider2 * This,
            /* [in] */ enum ZoomUnit zoomUnit);
        
        END_INTERFACE
    } ITransformProvider2Vtbl;

    interface ITransformProvider2
    {
        CONST_VTBL struct ITransformProvider2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITransformProvider2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITransformProvider2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITransformProvider2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITransformProvider2_Move(This,x,y)	\
    ( (This)->lpVtbl -> Move(This,x,y) ) 

#define ITransformProvider2_Resize(This,width,height)	\
    ( (This)->lpVtbl -> Resize(This,width,height) ) 

#define ITransformProvider2_Rotate(This,degrees)	\
    ( (This)->lpVtbl -> Rotate(This,degrees) ) 

#define ITransformProvider2_get_CanMove(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanMove(This,pRetVal) ) 

#define ITransformProvider2_get_CanResize(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanResize(This,pRetVal) ) 

#define ITransformProvider2_get_CanRotate(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanRotate(This,pRetVal) ) 


#define ITransformProvider2_Zoom(This,zoom)	\
    ( (This)->lpVtbl -> Zoom(This,zoom) ) 

#define ITransformProvider2_get_CanZoom(This,pRetVal)	\
    ( (This)->lpVtbl -> get_CanZoom(This,pRetVal) ) 

#define ITransformProvider2_get_ZoomLevel(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ZoomLevel(This,pRetVal) ) 

#define ITransformProvider2_get_ZoomMinimum(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ZoomMinimum(This,pRetVal) ) 

#define ITransformProvider2_get_ZoomMaximum(This,pRetVal)	\
    ( (This)->lpVtbl -> get_ZoomMaximum(This,pRetVal) ) 

#define ITransformProvider2_ZoomByUnit(This,zoomUnit)	\
    ( (This)->lpVtbl -> ZoomByUnit(This,zoomUnit) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITransformProvider2_INTERFACE_DEFINED__ */


#ifndef __IDragProvider_INTERFACE_DEFINED__
#define __IDragProvider_INTERFACE_DEFINED__

/* interface IDragProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IDragProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6aa7bbbb-7ff9-497d-904f-d20b897929d8")
    IDragProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_IsGrabbed( 
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DropEffect( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DropEffects( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGrabbedItems( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDragProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDragProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDragProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDragProvider * This);
        
        DECLSPEC_XFGVIRT(IDragProvider, get_IsGrabbed)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_IsGrabbed )( 
            __RPC__in IDragProvider * This,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(IDragProvider, get_DropEffect)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DropEffect )( 
            __RPC__in IDragProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IDragProvider, get_DropEffects)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DropEffects )( 
            __RPC__in IDragProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(IDragProvider, GetGrabbedItems)
        HRESULT ( STDMETHODCALLTYPE *GetGrabbedItems )( 
            __RPC__in IDragProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } IDragProviderVtbl;

    interface IDragProvider
    {
        CONST_VTBL struct IDragProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDragProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDragProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDragProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDragProvider_get_IsGrabbed(This,pRetVal)	\
    ( (This)->lpVtbl -> get_IsGrabbed(This,pRetVal) ) 

#define IDragProvider_get_DropEffect(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DropEffect(This,pRetVal) ) 

#define IDragProvider_get_DropEffects(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DropEffects(This,pRetVal) ) 

#define IDragProvider_GetGrabbedItems(This,pRetVal)	\
    ( (This)->lpVtbl -> GetGrabbedItems(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDragProvider_INTERFACE_DEFINED__ */


#ifndef __IDropTargetProvider_INTERFACE_DEFINED__
#define __IDropTargetProvider_INTERFACE_DEFINED__

/* interface IDropTargetProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IDropTargetProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bae82bfd-358a-481c-85a0-d8b4d90a5d61")
    IDropTargetProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DropTargetEffect( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DropTargetEffects( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDropTargetProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDropTargetProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDropTargetProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDropTargetProvider * This);
        
        DECLSPEC_XFGVIRT(IDropTargetProvider, get_DropTargetEffect)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DropTargetEffect )( 
            __RPC__in IDropTargetProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(IDropTargetProvider, get_DropTargetEffects)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DropTargetEffects )( 
            __RPC__in IDropTargetProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } IDropTargetProviderVtbl;

    interface IDropTargetProvider
    {
        CONST_VTBL struct IDropTargetProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDropTargetProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDropTargetProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDropTargetProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDropTargetProvider_get_DropTargetEffect(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DropTargetEffect(This,pRetVal) ) 

#define IDropTargetProvider_get_DropTargetEffects(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DropTargetEffects(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDropTargetProvider_INTERFACE_DEFINED__ */


#ifndef __ITextRangeProvider_INTERFACE_DEFINED__
#define __ITextRangeProvider_INTERFACE_DEFINED__

/* interface ITextRangeProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextRangeProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5347ad7b-c355-46f8-aff5-909033582f63")
    ITextRangeProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Compare( 
            /* [in] */ __RPC__in_opt ITextRangeProvider *range,
            /* [retval][out] */ __RPC__out BOOL *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompareEndpoints( 
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ __RPC__in_opt ITextRangeProvider *targetRange,
            /* [in] */ enum TextPatternRangeEndpoint targetEndpoint,
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ExpandToEnclosingUnit( 
            /* [in] */ enum TextUnit unit) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindAttribute( 
            /* [in] */ TEXTATTRIBUTEID attributeId,
            /* [in] */ VARIANT val,
            /* [in] */ BOOL backward,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindText( 
            /* [in] */ __RPC__in BSTR text,
            /* [in] */ BOOL backward,
            /* [in] */ BOOL ignoreCase,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAttributeValue( 
            /* [in] */ TEXTATTRIBUTEID attributeId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBoundingRectangles( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnclosingElement( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [in] */ int maxLength,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Move( 
            /* [in] */ enum TextUnit unit,
            /* [in] */ int count,
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveEndpointByUnit( 
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ enum TextUnit unit,
            /* [in] */ int count,
            /* [retval][out] */ __RPC__out int *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MoveEndpointByRange( 
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ __RPC__in_opt ITextRangeProvider *targetRange,
            /* [in] */ enum TextPatternRangeEndpoint targetEndpoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Select( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddToSelection( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveFromSelection( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ScrollIntoView( 
            /* [in] */ BOOL alignToTop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChildren( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextRangeProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextRangeProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextRangeProvider * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITextRangeProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ __RPC__in_opt ITextRangeProvider *range,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, CompareEndpoints)
        HRESULT ( STDMETHODCALLTYPE *CompareEndpoints )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ __RPC__in_opt ITextRangeProvider *targetRange,
            /* [in] */ enum TextPatternRangeEndpoint targetEndpoint,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, ExpandToEnclosingUnit)
        HRESULT ( STDMETHODCALLTYPE *ExpandToEnclosingUnit )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ enum TextUnit unit);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, FindAttribute)
        HRESULT ( STDMETHODCALLTYPE *FindAttribute )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ TEXTATTRIBUTEID attributeId,
            /* [in] */ VARIANT val,
            /* [in] */ BOOL backward,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, FindText)
        HRESULT ( STDMETHODCALLTYPE *FindText )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ __RPC__in BSTR text,
            /* [in] */ BOOL backward,
            /* [in] */ BOOL ignoreCase,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetAttributeValue)
        HRESULT ( STDMETHODCALLTYPE *GetAttributeValue )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ TEXTATTRIBUTEID attributeId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetBoundingRectangles)
        HRESULT ( STDMETHODCALLTYPE *GetBoundingRectangles )( 
            __RPC__in ITextRangeProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetEnclosingElement)
        HRESULT ( STDMETHODCALLTYPE *GetEnclosingElement )( 
            __RPC__in ITextRangeProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ int maxLength,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ enum TextUnit unit,
            /* [in] */ int count,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, MoveEndpointByUnit)
        HRESULT ( STDMETHODCALLTYPE *MoveEndpointByUnit )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ enum TextUnit unit,
            /* [in] */ int count,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, MoveEndpointByRange)
        HRESULT ( STDMETHODCALLTYPE *MoveEndpointByRange )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ __RPC__in_opt ITextRangeProvider *targetRange,
            /* [in] */ enum TextPatternRangeEndpoint targetEndpoint);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Select)
        HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ITextRangeProvider * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, AddToSelection)
        HRESULT ( STDMETHODCALLTYPE *AddToSelection )( 
            __RPC__in ITextRangeProvider * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, RemoveFromSelection)
        HRESULT ( STDMETHODCALLTYPE *RemoveFromSelection )( 
            __RPC__in ITextRangeProvider * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, ScrollIntoView)
        HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in ITextRangeProvider * This,
            /* [in] */ BOOL alignToTop);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetChildren)
        HRESULT ( STDMETHODCALLTYPE *GetChildren )( 
            __RPC__in ITextRangeProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        END_INTERFACE
    } ITextRangeProviderVtbl;

    interface ITextRangeProvider
    {
        CONST_VTBL struct ITextRangeProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextRangeProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextRangeProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextRangeProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextRangeProvider_Clone(This,pRetVal)	\
    ( (This)->lpVtbl -> Clone(This,pRetVal) ) 

#define ITextRangeProvider_Compare(This,range,pRetVal)	\
    ( (This)->lpVtbl -> Compare(This,range,pRetVal) ) 

#define ITextRangeProvider_CompareEndpoints(This,endpoint,targetRange,targetEndpoint,pRetVal)	\
    ( (This)->lpVtbl -> CompareEndpoints(This,endpoint,targetRange,targetEndpoint,pRetVal) ) 

#define ITextRangeProvider_ExpandToEnclosingUnit(This,unit)	\
    ( (This)->lpVtbl -> ExpandToEnclosingUnit(This,unit) ) 

#define ITextRangeProvider_FindAttribute(This,attributeId,val,backward,pRetVal)	\
    ( (This)->lpVtbl -> FindAttribute(This,attributeId,val,backward,pRetVal) ) 

#define ITextRangeProvider_FindText(This,text,backward,ignoreCase,pRetVal)	\
    ( (This)->lpVtbl -> FindText(This,text,backward,ignoreCase,pRetVal) ) 

#define ITextRangeProvider_GetAttributeValue(This,attributeId,pRetVal)	\
    ( (This)->lpVtbl -> GetAttributeValue(This,attributeId,pRetVal) ) 

#define ITextRangeProvider_GetBoundingRectangles(This,pRetVal)	\
    ( (This)->lpVtbl -> GetBoundingRectangles(This,pRetVal) ) 

#define ITextRangeProvider_GetEnclosingElement(This,pRetVal)	\
    ( (This)->lpVtbl -> GetEnclosingElement(This,pRetVal) ) 

#define ITextRangeProvider_GetText(This,maxLength,pRetVal)	\
    ( (This)->lpVtbl -> GetText(This,maxLength,pRetVal) ) 

#define ITextRangeProvider_Move(This,unit,count,pRetVal)	\
    ( (This)->lpVtbl -> Move(This,unit,count,pRetVal) ) 

#define ITextRangeProvider_MoveEndpointByUnit(This,endpoint,unit,count,pRetVal)	\
    ( (This)->lpVtbl -> MoveEndpointByUnit(This,endpoint,unit,count,pRetVal) ) 

#define ITextRangeProvider_MoveEndpointByRange(This,endpoint,targetRange,targetEndpoint)	\
    ( (This)->lpVtbl -> MoveEndpointByRange(This,endpoint,targetRange,targetEndpoint) ) 

#define ITextRangeProvider_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ITextRangeProvider_AddToSelection(This)	\
    ( (This)->lpVtbl -> AddToSelection(This) ) 

#define ITextRangeProvider_RemoveFromSelection(This)	\
    ( (This)->lpVtbl -> RemoveFromSelection(This) ) 

#define ITextRangeProvider_ScrollIntoView(This,alignToTop)	\
    ( (This)->lpVtbl -> ScrollIntoView(This,alignToTop) ) 

#define ITextRangeProvider_GetChildren(This,pRetVal)	\
    ( (This)->lpVtbl -> GetChildren(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextRangeProvider_INTERFACE_DEFINED__ */


#ifndef __ITextProvider_INTERFACE_DEFINED__
#define __ITextProvider_INTERFACE_DEFINED__

/* interface ITextProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3589c92c-63f3-4367-99bb-ada653b77cf2")
    ITextProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSelection( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVisibleRanges( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RangeFromChild( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *childElement,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RangeFromPoint( 
            /* [in] */ struct UiaPoint point,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_DocumentRange( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_SupportedTextSelection( 
            /* [retval][out] */ __RPC__out enum SupportedTextSelection *pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextProvider * This);
        
        DECLSPEC_XFGVIRT(ITextProvider, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, GetVisibleRanges)
        HRESULT ( STDMETHODCALLTYPE *GetVisibleRanges )( 
            __RPC__in ITextProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, RangeFromChild)
        HRESULT ( STDMETHODCALLTYPE *RangeFromChild )( 
            __RPC__in ITextProvider * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *childElement,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, RangeFromPoint)
        HRESULT ( STDMETHODCALLTYPE *RangeFromPoint )( 
            __RPC__in ITextProvider * This,
            /* [in] */ struct UiaPoint point,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, get_DocumentRange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentRange )( 
            __RPC__in ITextProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, get_SupportedTextSelection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedTextSelection )( 
            __RPC__in ITextProvider * This,
            /* [retval][out] */ __RPC__out enum SupportedTextSelection *pRetVal);
        
        END_INTERFACE
    } ITextProviderVtbl;

    interface ITextProvider
    {
        CONST_VTBL struct ITextProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextProvider_GetSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSelection(This,pRetVal) ) 

#define ITextProvider_GetVisibleRanges(This,pRetVal)	\
    ( (This)->lpVtbl -> GetVisibleRanges(This,pRetVal) ) 

#define ITextProvider_RangeFromChild(This,childElement,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromChild(This,childElement,pRetVal) ) 

#define ITextProvider_RangeFromPoint(This,point,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromPoint(This,point,pRetVal) ) 

#define ITextProvider_get_DocumentRange(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DocumentRange(This,pRetVal) ) 

#define ITextProvider_get_SupportedTextSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> get_SupportedTextSelection(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextProvider_INTERFACE_DEFINED__ */


#ifndef __ITextProvider2_INTERFACE_DEFINED__
#define __ITextProvider2_INTERFACE_DEFINED__

/* interface ITextProvider2 */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextProvider2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0dc5e6ed-3e16-4bf1-8f9a-a979878bc195")
    ITextProvider2 : public ITextProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RangeFromAnnotation( 
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *annotationElement,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCaretRange( 
            /* [out] */ __RPC__out BOOL *isActive,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextProvider2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextProvider2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextProvider2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextProvider2 * This);
        
        DECLSPEC_XFGVIRT(ITextProvider, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, GetVisibleRanges)
        HRESULT ( STDMETHODCALLTYPE *GetVisibleRanges )( 
            __RPC__in ITextProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, RangeFromChild)
        HRESULT ( STDMETHODCALLTYPE *RangeFromChild )( 
            __RPC__in ITextProvider2 * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *childElement,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, RangeFromPoint)
        HRESULT ( STDMETHODCALLTYPE *RangeFromPoint )( 
            __RPC__in ITextProvider2 * This,
            /* [in] */ struct UiaPoint point,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, get_DocumentRange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentRange )( 
            __RPC__in ITextProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, get_SupportedTextSelection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedTextSelection )( 
            __RPC__in ITextProvider2 * This,
            /* [retval][out] */ __RPC__out enum SupportedTextSelection *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider2, RangeFromAnnotation)
        HRESULT ( STDMETHODCALLTYPE *RangeFromAnnotation )( 
            __RPC__in ITextProvider2 * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *annotationElement,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider2, GetCaretRange)
        HRESULT ( STDMETHODCALLTYPE *GetCaretRange )( 
            __RPC__in ITextProvider2 * This,
            /* [out] */ __RPC__out BOOL *isActive,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        END_INTERFACE
    } ITextProvider2Vtbl;

    interface ITextProvider2
    {
        CONST_VTBL struct ITextProvider2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextProvider2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextProvider2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextProvider2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextProvider2_GetSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSelection(This,pRetVal) ) 

#define ITextProvider2_GetVisibleRanges(This,pRetVal)	\
    ( (This)->lpVtbl -> GetVisibleRanges(This,pRetVal) ) 

#define ITextProvider2_RangeFromChild(This,childElement,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromChild(This,childElement,pRetVal) ) 

#define ITextProvider2_RangeFromPoint(This,point,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromPoint(This,point,pRetVal) ) 

#define ITextProvider2_get_DocumentRange(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DocumentRange(This,pRetVal) ) 

#define ITextProvider2_get_SupportedTextSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> get_SupportedTextSelection(This,pRetVal) ) 


#define ITextProvider2_RangeFromAnnotation(This,annotationElement,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromAnnotation(This,annotationElement,pRetVal) ) 

#define ITextProvider2_GetCaretRange(This,isActive,pRetVal)	\
    ( (This)->lpVtbl -> GetCaretRange(This,isActive,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextProvider2_INTERFACE_DEFINED__ */


#ifndef __ITextEditProvider_INTERFACE_DEFINED__
#define __ITextEditProvider_INTERFACE_DEFINED__

/* interface ITextEditProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextEditProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EA3605B4-3A05-400E-B5F9-4E91B40F6176")
    ITextEditProvider : public ITextProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetActiveComposition( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConversionTarget( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextEditProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextEditProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextEditProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextEditProvider * This);
        
        DECLSPEC_XFGVIRT(ITextProvider, GetSelection)
        HRESULT ( STDMETHODCALLTYPE *GetSelection )( 
            __RPC__in ITextEditProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, GetVisibleRanges)
        HRESULT ( STDMETHODCALLTYPE *GetVisibleRanges )( 
            __RPC__in ITextEditProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, RangeFromChild)
        HRESULT ( STDMETHODCALLTYPE *RangeFromChild )( 
            __RPC__in ITextEditProvider * This,
            /* [in] */ __RPC__in_opt IRawElementProviderSimple *childElement,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, RangeFromPoint)
        HRESULT ( STDMETHODCALLTYPE *RangeFromPoint )( 
            __RPC__in ITextEditProvider * This,
            /* [in] */ struct UiaPoint point,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, get_DocumentRange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_DocumentRange )( 
            __RPC__in ITextEditProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextProvider, get_SupportedTextSelection)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_SupportedTextSelection )( 
            __RPC__in ITextEditProvider * This,
            /* [retval][out] */ __RPC__out enum SupportedTextSelection *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextEditProvider, GetActiveComposition)
        HRESULT ( STDMETHODCALLTYPE *GetActiveComposition )( 
            __RPC__in ITextEditProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextEditProvider, GetConversionTarget)
        HRESULT ( STDMETHODCALLTYPE *GetConversionTarget )( 
            __RPC__in ITextEditProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        END_INTERFACE
    } ITextEditProviderVtbl;

    interface ITextEditProvider
    {
        CONST_VTBL struct ITextEditProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextEditProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextEditProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextEditProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextEditProvider_GetSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> GetSelection(This,pRetVal) ) 

#define ITextEditProvider_GetVisibleRanges(This,pRetVal)	\
    ( (This)->lpVtbl -> GetVisibleRanges(This,pRetVal) ) 

#define ITextEditProvider_RangeFromChild(This,childElement,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromChild(This,childElement,pRetVal) ) 

#define ITextEditProvider_RangeFromPoint(This,point,pRetVal)	\
    ( (This)->lpVtbl -> RangeFromPoint(This,point,pRetVal) ) 

#define ITextEditProvider_get_DocumentRange(This,pRetVal)	\
    ( (This)->lpVtbl -> get_DocumentRange(This,pRetVal) ) 

#define ITextEditProvider_get_SupportedTextSelection(This,pRetVal)	\
    ( (This)->lpVtbl -> get_SupportedTextSelection(This,pRetVal) ) 


#define ITextEditProvider_GetActiveComposition(This,pRetVal)	\
    ( (This)->lpVtbl -> GetActiveComposition(This,pRetVal) ) 

#define ITextEditProvider_GetConversionTarget(This,pRetVal)	\
    ( (This)->lpVtbl -> GetConversionTarget(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextEditProvider_INTERFACE_DEFINED__ */


#ifndef __ITextRangeProvider2_INTERFACE_DEFINED__
#define __ITextRangeProvider2_INTERFACE_DEFINED__

/* interface ITextRangeProvider2 */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextRangeProvider2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9BBCE42C-1921-4F18-89CA-DBA1910A0386")
    ITextRangeProvider2 : public ITextRangeProvider
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowContextMenu( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextRangeProvider2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextRangeProvider2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextRangeProvider2 * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ __RPC__in_opt ITextRangeProvider *range,
            /* [retval][out] */ __RPC__out BOOL *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, CompareEndpoints)
        HRESULT ( STDMETHODCALLTYPE *CompareEndpoints )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ __RPC__in_opt ITextRangeProvider *targetRange,
            /* [in] */ enum TextPatternRangeEndpoint targetEndpoint,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, ExpandToEnclosingUnit)
        HRESULT ( STDMETHODCALLTYPE *ExpandToEnclosingUnit )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ enum TextUnit unit);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, FindAttribute)
        HRESULT ( STDMETHODCALLTYPE *FindAttribute )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ TEXTATTRIBUTEID attributeId,
            /* [in] */ VARIANT val,
            /* [in] */ BOOL backward,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, FindText)
        HRESULT ( STDMETHODCALLTYPE *FindText )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ __RPC__in BSTR text,
            /* [in] */ BOOL backward,
            /* [in] */ BOOL ignoreCase,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetAttributeValue)
        HRESULT ( STDMETHODCALLTYPE *GetAttributeValue )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ TEXTATTRIBUTEID attributeId,
            /* [retval][out] */ __RPC__out VARIANT *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetBoundingRectangles)
        HRESULT ( STDMETHODCALLTYPE *GetBoundingRectangles )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetEnclosingElement)
        HRESULT ( STDMETHODCALLTYPE *GetEnclosingElement )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ int maxLength,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Move)
        HRESULT ( STDMETHODCALLTYPE *Move )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ enum TextUnit unit,
            /* [in] */ int count,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, MoveEndpointByUnit)
        HRESULT ( STDMETHODCALLTYPE *MoveEndpointByUnit )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ enum TextUnit unit,
            /* [in] */ int count,
            /* [retval][out] */ __RPC__out int *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, MoveEndpointByRange)
        HRESULT ( STDMETHODCALLTYPE *MoveEndpointByRange )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ enum TextPatternRangeEndpoint endpoint,
            /* [in] */ __RPC__in_opt ITextRangeProvider *targetRange,
            /* [in] */ enum TextPatternRangeEndpoint targetEndpoint);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, Select)
        HRESULT ( STDMETHODCALLTYPE *Select )( 
            __RPC__in ITextRangeProvider2 * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, AddToSelection)
        HRESULT ( STDMETHODCALLTYPE *AddToSelection )( 
            __RPC__in ITextRangeProvider2 * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, RemoveFromSelection)
        HRESULT ( STDMETHODCALLTYPE *RemoveFromSelection )( 
            __RPC__in ITextRangeProvider2 * This);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, ScrollIntoView)
        HRESULT ( STDMETHODCALLTYPE *ScrollIntoView )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [in] */ BOOL alignToTop);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider, GetChildren)
        HRESULT ( STDMETHODCALLTYPE *GetChildren )( 
            __RPC__in ITextRangeProvider2 * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *pRetVal);
        
        DECLSPEC_XFGVIRT(ITextRangeProvider2, ShowContextMenu)
        HRESULT ( STDMETHODCALLTYPE *ShowContextMenu )( 
            __RPC__in ITextRangeProvider2 * This);
        
        END_INTERFACE
    } ITextRangeProvider2Vtbl;

    interface ITextRangeProvider2
    {
        CONST_VTBL struct ITextRangeProvider2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextRangeProvider2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextRangeProvider2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextRangeProvider2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextRangeProvider2_Clone(This,pRetVal)	\
    ( (This)->lpVtbl -> Clone(This,pRetVal) ) 

#define ITextRangeProvider2_Compare(This,range,pRetVal)	\
    ( (This)->lpVtbl -> Compare(This,range,pRetVal) ) 

#define ITextRangeProvider2_CompareEndpoints(This,endpoint,targetRange,targetEndpoint,pRetVal)	\
    ( (This)->lpVtbl -> CompareEndpoints(This,endpoint,targetRange,targetEndpoint,pRetVal) ) 

#define ITextRangeProvider2_ExpandToEnclosingUnit(This,unit)	\
    ( (This)->lpVtbl -> ExpandToEnclosingUnit(This,unit) ) 

#define ITextRangeProvider2_FindAttribute(This,attributeId,val,backward,pRetVal)	\
    ( (This)->lpVtbl -> FindAttribute(This,attributeId,val,backward,pRetVal) ) 

#define ITextRangeProvider2_FindText(This,text,backward,ignoreCase,pRetVal)	\
    ( (This)->lpVtbl -> FindText(This,text,backward,ignoreCase,pRetVal) ) 

#define ITextRangeProvider2_GetAttributeValue(This,attributeId,pRetVal)	\
    ( (This)->lpVtbl -> GetAttributeValue(This,attributeId,pRetVal) ) 

#define ITextRangeProvider2_GetBoundingRectangles(This,pRetVal)	\
    ( (This)->lpVtbl -> GetBoundingRectangles(This,pRetVal) ) 

#define ITextRangeProvider2_GetEnclosingElement(This,pRetVal)	\
    ( (This)->lpVtbl -> GetEnclosingElement(This,pRetVal) ) 

#define ITextRangeProvider2_GetText(This,maxLength,pRetVal)	\
    ( (This)->lpVtbl -> GetText(This,maxLength,pRetVal) ) 

#define ITextRangeProvider2_Move(This,unit,count,pRetVal)	\
    ( (This)->lpVtbl -> Move(This,unit,count,pRetVal) ) 

#define ITextRangeProvider2_MoveEndpointByUnit(This,endpoint,unit,count,pRetVal)	\
    ( (This)->lpVtbl -> MoveEndpointByUnit(This,endpoint,unit,count,pRetVal) ) 

#define ITextRangeProvider2_MoveEndpointByRange(This,endpoint,targetRange,targetEndpoint)	\
    ( (This)->lpVtbl -> MoveEndpointByRange(This,endpoint,targetRange,targetEndpoint) ) 

#define ITextRangeProvider2_Select(This)	\
    ( (This)->lpVtbl -> Select(This) ) 

#define ITextRangeProvider2_AddToSelection(This)	\
    ( (This)->lpVtbl -> AddToSelection(This) ) 

#define ITextRangeProvider2_RemoveFromSelection(This)	\
    ( (This)->lpVtbl -> RemoveFromSelection(This) ) 

#define ITextRangeProvider2_ScrollIntoView(This,alignToTop)	\
    ( (This)->lpVtbl -> ScrollIntoView(This,alignToTop) ) 

#define ITextRangeProvider2_GetChildren(This,pRetVal)	\
    ( (This)->lpVtbl -> GetChildren(This,pRetVal) ) 


#define ITextRangeProvider2_ShowContextMenu(This)	\
    ( (This)->lpVtbl -> ShowContextMenu(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextRangeProvider2_INTERFACE_DEFINED__ */


#ifndef __ITextChildProvider_INTERFACE_DEFINED__
#define __ITextChildProvider_INTERFACE_DEFINED__

/* interface ITextChildProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ITextChildProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4c2de2b9-c88f-4f88-a111-f1d336b7d1a9")
    ITextChildProvider : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TextContainer( 
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_TextRange( 
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ITextChildProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ITextChildProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ITextChildProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ITextChildProvider * This);
        
        DECLSPEC_XFGVIRT(ITextChildProvider, get_TextContainer)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TextContainer )( 
            __RPC__in ITextChildProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        DECLSPEC_XFGVIRT(ITextChildProvider, get_TextRange)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_TextRange )( 
            __RPC__in ITextChildProvider * This,
            /* [retval][out] */ __RPC__deref_out_opt ITextRangeProvider **pRetVal);
        
        END_INTERFACE
    } ITextChildProviderVtbl;

    interface ITextChildProvider
    {
        CONST_VTBL struct ITextChildProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ITextChildProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ITextChildProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ITextChildProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ITextChildProvider_get_TextContainer(This,pRetVal)	\
    ( (This)->lpVtbl -> get_TextContainer(This,pRetVal) ) 

#define ITextChildProvider_get_TextRange(This,pRetVal)	\
    ( (This)->lpVtbl -> get_TextRange(This,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ITextChildProvider_INTERFACE_DEFINED__ */


#ifndef __ICustomNavigationProvider_INTERFACE_DEFINED__
#define __ICustomNavigationProvider_INTERFACE_DEFINED__

/* interface ICustomNavigationProvider */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_ICustomNavigationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2062A28A-8C07-4B94-8E12-7037C622AEB8")
    ICustomNavigationProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Navigate( 
            /* [in] */ enum NavigateDirection direction,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICustomNavigationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in ICustomNavigationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in ICustomNavigationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in ICustomNavigationProvider * This);
        
        DECLSPEC_XFGVIRT(ICustomNavigationProvider, Navigate)
        HRESULT ( STDMETHODCALLTYPE *Navigate )( 
            __RPC__in ICustomNavigationProvider * This,
            /* [in] */ enum NavigateDirection direction,
            /* [retval][out] */ __RPC__deref_out_opt IRawElementProviderSimple **pRetVal);
        
        END_INTERFACE
    } ICustomNavigationProviderVtbl;

    interface ICustomNavigationProvider
    {
        CONST_VTBL struct ICustomNavigationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICustomNavigationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICustomNavigationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICustomNavigationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICustomNavigationProvider_Navigate(This,direction,pRetVal)	\
    ( (This)->lpVtbl -> Navigate(This,direction,pRetVal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICustomNavigationProvider_INTERFACE_DEFINED__ */


#ifndef __IUIAutomationPatternInstance_INTERFACE_DEFINED__
#define __IUIAutomationPatternInstance_INTERFACE_DEFINED__

/* interface IUIAutomationPatternInstance */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IUIAutomationPatternInstance;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c03a7fe4-9431-409f-bed8-ae7c2299bc8d")
    IUIAutomationPatternInstance : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ UINT index,
            /* [in] */ BOOL cached,
            /* [in] */ enum UIAutomationType type,
            /* [out] */ void *pPtr) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CallMethod( 
            /* [in] */ UINT index,
            /* [in] */ const struct UIAutomationParameter *pParams,
            /* [in] */ UINT cParams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAutomationPatternInstanceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUIAutomationPatternInstance * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUIAutomationPatternInstance * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUIAutomationPatternInstance * This);
        
        DECLSPEC_XFGVIRT(IUIAutomationPatternInstance, GetProperty)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            IUIAutomationPatternInstance * This,
            /* [in] */ UINT index,
            /* [in] */ BOOL cached,
            /* [in] */ enum UIAutomationType type,
            /* [out] */ void *pPtr);
        
        DECLSPEC_XFGVIRT(IUIAutomationPatternInstance, CallMethod)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CallMethod )( 
            IUIAutomationPatternInstance * This,
            /* [in] */ UINT index,
            /* [in] */ const struct UIAutomationParameter *pParams,
            /* [in] */ UINT cParams);
        
        END_INTERFACE
    } IUIAutomationPatternInstanceVtbl;

    interface IUIAutomationPatternInstance
    {
        CONST_VTBL struct IUIAutomationPatternInstanceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAutomationPatternInstance_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAutomationPatternInstance_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAutomationPatternInstance_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAutomationPatternInstance_GetProperty(This,index,cached,type,pPtr)	\
    ( (This)->lpVtbl -> GetProperty(This,index,cached,type,pPtr) ) 

#define IUIAutomationPatternInstance_CallMethod(This,index,pParams,cParams)	\
    ( (This)->lpVtbl -> CallMethod(This,index,pParams,cParams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAutomationPatternInstance_INTERFACE_DEFINED__ */


#ifndef __IUIAutomationPatternHandler_INTERFACE_DEFINED__
#define __IUIAutomationPatternHandler_INTERFACE_DEFINED__

/* interface IUIAutomationPatternHandler */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IUIAutomationPatternHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d97022f3-a947-465e-8b2a-ac4315fa54e8")
    IUIAutomationPatternHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateClientWrapper( 
            /* [in] */ __RPC__in_opt IUIAutomationPatternInstance *pPatternInstance,
            /* [out] */ __RPC__deref_out_opt IUnknown **pClientWrapper) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Dispatch( 
            /* [in] */ IUnknown *pTarget,
            /* [in] */ UINT index,
            /* [in] */ const struct UIAutomationParameter *pParams,
            /* [in] */ UINT cParams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAutomationPatternHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUIAutomationPatternHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUIAutomationPatternHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUIAutomationPatternHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAutomationPatternHandler, CreateClientWrapper)
        HRESULT ( STDMETHODCALLTYPE *CreateClientWrapper )( 
            __RPC__in IUIAutomationPatternHandler * This,
            /* [in] */ __RPC__in_opt IUIAutomationPatternInstance *pPatternInstance,
            /* [out] */ __RPC__deref_out_opt IUnknown **pClientWrapper);
        
        DECLSPEC_XFGVIRT(IUIAutomationPatternHandler, Dispatch)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Dispatch )( 
            IUIAutomationPatternHandler * This,
            /* [in] */ IUnknown *pTarget,
            /* [in] */ UINT index,
            /* [in] */ const struct UIAutomationParameter *pParams,
            /* [in] */ UINT cParams);
        
        END_INTERFACE
    } IUIAutomationPatternHandlerVtbl;

    interface IUIAutomationPatternHandler
    {
        CONST_VTBL struct IUIAutomationPatternHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAutomationPatternHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAutomationPatternHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAutomationPatternHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAutomationPatternHandler_CreateClientWrapper(This,pPatternInstance,pClientWrapper)	\
    ( (This)->lpVtbl -> CreateClientWrapper(This,pPatternInstance,pClientWrapper) ) 

#define IUIAutomationPatternHandler_Dispatch(This,pTarget,index,pParams,cParams)	\
    ( (This)->lpVtbl -> Dispatch(This,pTarget,index,pParams,cParams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAutomationPatternHandler_INTERFACE_DEFINED__ */


#ifndef __IUIAutomationRegistrar_INTERFACE_DEFINED__
#define __IUIAutomationRegistrar_INTERFACE_DEFINED__

/* interface IUIAutomationRegistrar */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IUIAutomationRegistrar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8609c4ec-4a1a-4d88-a357-5a66e060e1cf")
    IUIAutomationRegistrar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterProperty( 
            /* [in] */ __RPC__in const struct UIAutomationPropertyInfo *property,
            /* [out] */ __RPC__out PROPERTYID *propertyId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterEvent( 
            /* [in] */ __RPC__in const struct UIAutomationEventInfo *event,
            /* [out] */ __RPC__out EVENTID *eventId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterPattern( 
            /* [in] */ __RPC__in const struct UIAutomationPatternInfo *pattern,
            /* [out] */ __RPC__out PATTERNID *pPatternId,
            /* [out] */ __RPC__out PROPERTYID *pPatternAvailablePropertyId,
            /* [in] */ UINT propertyIdCount,
            /* [size_is][out] */ __RPC__out_ecount_full(propertyIdCount) PROPERTYID *pPropertyIds,
            /* [in] */ UINT eventIdCount,
            /* [size_is][out] */ __RPC__out_ecount_full(eventIdCount) EVENTID *pEventIds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAutomationRegistrarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUIAutomationRegistrar * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUIAutomationRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUIAutomationRegistrar * This);
        
        DECLSPEC_XFGVIRT(IUIAutomationRegistrar, RegisterProperty)
        HRESULT ( STDMETHODCALLTYPE *RegisterProperty )( 
            __RPC__in IUIAutomationRegistrar * This,
            /* [in] */ __RPC__in const struct UIAutomationPropertyInfo *property,
            /* [out] */ __RPC__out PROPERTYID *propertyId);
        
        DECLSPEC_XFGVIRT(IUIAutomationRegistrar, RegisterEvent)
        HRESULT ( STDMETHODCALLTYPE *RegisterEvent )( 
            __RPC__in IUIAutomationRegistrar * This,
            /* [in] */ __RPC__in const struct UIAutomationEventInfo *event,
            /* [out] */ __RPC__out EVENTID *eventId);
        
        DECLSPEC_XFGVIRT(IUIAutomationRegistrar, RegisterPattern)
        HRESULT ( STDMETHODCALLTYPE *RegisterPattern )( 
            __RPC__in IUIAutomationRegistrar * This,
            /* [in] */ __RPC__in const struct UIAutomationPatternInfo *pattern,
            /* [out] */ __RPC__out PATTERNID *pPatternId,
            /* [out] */ __RPC__out PROPERTYID *pPatternAvailablePropertyId,
            /* [in] */ UINT propertyIdCount,
            /* [size_is][out] */ __RPC__out_ecount_full(propertyIdCount) PROPERTYID *pPropertyIds,
            /* [in] */ UINT eventIdCount,
            /* [size_is][out] */ __RPC__out_ecount_full(eventIdCount) EVENTID *pEventIds);
        
        END_INTERFACE
    } IUIAutomationRegistrarVtbl;

    interface IUIAutomationRegistrar
    {
        CONST_VTBL struct IUIAutomationRegistrarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAutomationRegistrar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAutomationRegistrar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAutomationRegistrar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAutomationRegistrar_RegisterProperty(This,property,propertyId)	\
    ( (This)->lpVtbl -> RegisterProperty(This,property,propertyId) ) 

#define IUIAutomationRegistrar_RegisterEvent(This,event,eventId)	\
    ( (This)->lpVtbl -> RegisterEvent(This,event,eventId) ) 

#define IUIAutomationRegistrar_RegisterPattern(This,pattern,pPatternId,pPatternAvailablePropertyId,propertyIdCount,pPropertyIds,eventIdCount,pEventIds)	\
    ( (This)->lpVtbl -> RegisterPattern(This,pattern,pPatternId,pPatternAvailablePropertyId,propertyIdCount,pPropertyIds,eventIdCount,pEventIds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAutomationRegistrar_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_CUIAutomationRegistrar;

#ifdef __cplusplus

class DECLSPEC_UUID("6e29fabf-9977-42d1-8d0e-ca7e61ad87e6")
CUIAutomationRegistrar;
#endif

#ifndef __IUIAutomationClientInfo_INTERFACE_DEFINED__
#define __IUIAutomationClientInfo_INTERFACE_DEFINED__

/* interface IUIAutomationClientInfo */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IUIAutomationClientInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B2E8A3F1-4C5D-4E7A-8F6B-3D2E1C9A0B8F")
    IUIAutomationClientInfo : public IUnknown
    {
    public:
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProcessId( 
            /* [retval][out] */ __RPC__out DWORD *processId) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_ProcessName( 
            /* [retval][out] */ __RPC__deref_out_opt BSTR *processName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAutomationClientInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUIAutomationClientInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUIAutomationClientInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUIAutomationClientInfo * This);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientInfo, get_ProcessId)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessId )( 
            __RPC__in IUIAutomationClientInfo * This,
            /* [retval][out] */ __RPC__out DWORD *processId);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientInfo, get_ProcessName)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_ProcessName )( 
            __RPC__in IUIAutomationClientInfo * This,
            /* [retval][out] */ __RPC__deref_out_opt BSTR *processName);
        
        END_INTERFACE
    } IUIAutomationClientInfoVtbl;

    interface IUIAutomationClientInfo
    {
        CONST_VTBL struct IUIAutomationClientInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAutomationClientInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAutomationClientInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAutomationClientInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAutomationClientInfo_get_ProcessId(This,processId)	\
    ( (This)->lpVtbl -> get_ProcessId(This,processId) ) 

#define IUIAutomationClientInfo_get_ProcessName(This,processName)	\
    ( (This)->lpVtbl -> get_ProcessName(This,processName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAutomationClientInfo_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_CUIAutomationClientInfo;

#ifdef __cplusplus

class DECLSPEC_UUID("C2D4F567-8A9B-4C3E-9F1A-2B5C7D8E0F3A")
CUIAutomationClientInfo;
#endif

#ifndef __IUIAutomationClientConnectionCallback_INTERFACE_DEFINED__
#define __IUIAutomationClientConnectionCallback_INTERFACE_DEFINED__

/* interface IUIAutomationClientConnectionCallback */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IUIAutomationClientConnectionCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5B8E8F2A-9C7D-4F3E-A1B2-8D6E9F4C0A1B")
    IUIAutomationClientConnectionCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnConnected( 
            /* [in] */ __RPC__in_opt IUIAutomationClientInfo *clientInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnDisconnected( 
            /* [in] */ __RPC__in_opt IUIAutomationClientInfo *clientInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAutomationClientConnectionCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUIAutomationClientConnectionCallback * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUIAutomationClientConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUIAutomationClientConnectionCallback * This);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientConnectionCallback, OnConnected)
        HRESULT ( STDMETHODCALLTYPE *OnConnected )( 
            __RPC__in IUIAutomationClientConnectionCallback * This,
            /* [in] */ __RPC__in_opt IUIAutomationClientInfo *clientInfo);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientConnectionCallback, OnDisconnected)
        HRESULT ( STDMETHODCALLTYPE *OnDisconnected )( 
            __RPC__in IUIAutomationClientConnectionCallback * This,
            /* [in] */ __RPC__in_opt IUIAutomationClientInfo *clientInfo);
        
        END_INTERFACE
    } IUIAutomationClientConnectionCallbackVtbl;

    interface IUIAutomationClientConnectionCallback
    {
        CONST_VTBL struct IUIAutomationClientConnectionCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAutomationClientConnectionCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAutomationClientConnectionCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAutomationClientConnectionCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAutomationClientConnectionCallback_OnConnected(This,clientInfo)	\
    ( (This)->lpVtbl -> OnConnected(This,clientInfo) ) 

#define IUIAutomationClientConnectionCallback_OnDisconnected(This,clientInfo)	\
    ( (This)->lpVtbl -> OnDisconnected(This,clientInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAutomationClientConnectionCallback_INTERFACE_DEFINED__ */


#ifndef __IUIAutomationClientInfoSource_INTERFACE_DEFINED__
#define __IUIAutomationClientInfoSource_INTERFACE_DEFINED__

/* interface IUIAutomationClientInfoSource */
/* [oleautomation][unique][uuid][object] */ 


EXTERN_C const IID IID_IUIAutomationClientInfoSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F4B8A2E1-9C3D-4A7E-8F6B-2D5E4C1A9B8F")
    IUIAutomationClientInfoSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterClientConnectionCallback( 
            /* [in] */ __RPC__in_opt IUIAutomationClientConnectionCallback *callback,
            /* [out] */ __RPC__out unsigned __int64 *handle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterClientConnectionCallback( 
            /* [in] */ unsigned __int64 handle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnectedClients( 
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *clients) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAutomationClientInfoSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IUIAutomationClientInfoSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IUIAutomationClientInfoSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IUIAutomationClientInfoSource * This);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientInfoSource, RegisterClientConnectionCallback)
        HRESULT ( STDMETHODCALLTYPE *RegisterClientConnectionCallback )( 
            __RPC__in IUIAutomationClientInfoSource * This,
            /* [in] */ __RPC__in_opt IUIAutomationClientConnectionCallback *callback,
            /* [out] */ __RPC__out unsigned __int64 *handle);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientInfoSource, UnregisterClientConnectionCallback)
        HRESULT ( STDMETHODCALLTYPE *UnregisterClientConnectionCallback )( 
            __RPC__in IUIAutomationClientInfoSource * This,
            /* [in] */ unsigned __int64 handle);
        
        DECLSPEC_XFGVIRT(IUIAutomationClientInfoSource, GetConnectedClients)
        HRESULT ( STDMETHODCALLTYPE *GetConnectedClients )( 
            __RPC__in IUIAutomationClientInfoSource * This,
            /* [retval][out] */ __RPC__deref_out_opt SAFEARRAY * *clients);
        
        END_INTERFACE
    } IUIAutomationClientInfoSourceVtbl;

    interface IUIAutomationClientInfoSource
    {
        CONST_VTBL struct IUIAutomationClientInfoSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAutomationClientInfoSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAutomationClientInfoSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAutomationClientInfoSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAutomationClientInfoSource_RegisterClientConnectionCallback(This,callback,handle)	\
    ( (This)->lpVtbl -> RegisterClientConnectionCallback(This,callback,handle) ) 

#define IUIAutomationClientInfoSource_UnregisterClientConnectionCallback(This,handle)	\
    ( (This)->lpVtbl -> UnregisterClientConnectionCallback(This,handle) ) 

#define IUIAutomationClientInfoSource_GetConnectedClients(This,clients)	\
    ( (This)->lpVtbl -> GetConnectedClients(This,clients) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAutomationClientInfoSource_INTERFACE_DEFINED__ */


EXTERN_C const CLSID CLSID_CUIAutomationClientInfoSource;

#ifdef __cplusplus

class DECLSPEC_UUID("A8D4F123-7B2C-4E5F-9A1B-3C8D6E9F0A2B")
CUIAutomationClientInfoSource;
#endif
#endif /* __UIA_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


