

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

#ifndef COM_NO_WINDOWS_H
#include "windows.h"
#include "ole2.h"
#endif /*COM_NO_WINDOWS_H*/

#ifndef __directmanipulation_h__
#define __directmanipulation_h__

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

#ifndef __IDirectManipulationManager_FWD_DEFINED__
#define __IDirectManipulationManager_FWD_DEFINED__
typedef interface IDirectManipulationManager IDirectManipulationManager;

#endif 	/* __IDirectManipulationManager_FWD_DEFINED__ */


#ifndef __IDirectManipulationManager2_FWD_DEFINED__
#define __IDirectManipulationManager2_FWD_DEFINED__
typedef interface IDirectManipulationManager2 IDirectManipulationManager2;

#endif 	/* __IDirectManipulationManager2_FWD_DEFINED__ */


#ifndef __IDirectManipulationManager3_FWD_DEFINED__
#define __IDirectManipulationManager3_FWD_DEFINED__
typedef interface IDirectManipulationManager3 IDirectManipulationManager3;

#endif 	/* __IDirectManipulationManager3_FWD_DEFINED__ */


#ifndef __IDirectManipulationViewport_FWD_DEFINED__
#define __IDirectManipulationViewport_FWD_DEFINED__
typedef interface IDirectManipulationViewport IDirectManipulationViewport;

#endif 	/* __IDirectManipulationViewport_FWD_DEFINED__ */


#ifndef __IDirectManipulationViewport2_FWD_DEFINED__
#define __IDirectManipulationViewport2_FWD_DEFINED__
typedef interface IDirectManipulationViewport2 IDirectManipulationViewport2;

#endif 	/* __IDirectManipulationViewport2_FWD_DEFINED__ */


#ifndef __IDirectManipulationViewportEventHandler_FWD_DEFINED__
#define __IDirectManipulationViewportEventHandler_FWD_DEFINED__
typedef interface IDirectManipulationViewportEventHandler IDirectManipulationViewportEventHandler;

#endif 	/* __IDirectManipulationViewportEventHandler_FWD_DEFINED__ */


#ifndef __IDirectManipulationContent_FWD_DEFINED__
#define __IDirectManipulationContent_FWD_DEFINED__
typedef interface IDirectManipulationContent IDirectManipulationContent;

#endif 	/* __IDirectManipulationContent_FWD_DEFINED__ */


#ifndef __IDirectManipulationPrimaryContent_FWD_DEFINED__
#define __IDirectManipulationPrimaryContent_FWD_DEFINED__
typedef interface IDirectManipulationPrimaryContent IDirectManipulationPrimaryContent;

#endif 	/* __IDirectManipulationPrimaryContent_FWD_DEFINED__ */


#ifndef __IDirectManipulationDragDropEventHandler_FWD_DEFINED__
#define __IDirectManipulationDragDropEventHandler_FWD_DEFINED__
typedef interface IDirectManipulationDragDropEventHandler IDirectManipulationDragDropEventHandler;

#endif 	/* __IDirectManipulationDragDropEventHandler_FWD_DEFINED__ */


#ifndef __IDirectManipulationDragDropBehavior_FWD_DEFINED__
#define __IDirectManipulationDragDropBehavior_FWD_DEFINED__
typedef interface IDirectManipulationDragDropBehavior IDirectManipulationDragDropBehavior;

#endif 	/* __IDirectManipulationDragDropBehavior_FWD_DEFINED__ */


#ifndef __IDirectManipulationInteractionEventHandler_FWD_DEFINED__
#define __IDirectManipulationInteractionEventHandler_FWD_DEFINED__
typedef interface IDirectManipulationInteractionEventHandler IDirectManipulationInteractionEventHandler;

#endif 	/* __IDirectManipulationInteractionEventHandler_FWD_DEFINED__ */


#ifndef __IDirectManipulationFrameInfoProvider_FWD_DEFINED__
#define __IDirectManipulationFrameInfoProvider_FWD_DEFINED__
typedef interface IDirectManipulationFrameInfoProvider IDirectManipulationFrameInfoProvider;

#endif 	/* __IDirectManipulationFrameInfoProvider_FWD_DEFINED__ */


#ifndef __IDirectManipulationCompositor_FWD_DEFINED__
#define __IDirectManipulationCompositor_FWD_DEFINED__
typedef interface IDirectManipulationCompositor IDirectManipulationCompositor;

#endif 	/* __IDirectManipulationCompositor_FWD_DEFINED__ */


#ifndef __IDirectManipulationCompositor2_FWD_DEFINED__
#define __IDirectManipulationCompositor2_FWD_DEFINED__
typedef interface IDirectManipulationCompositor2 IDirectManipulationCompositor2;

#endif 	/* __IDirectManipulationCompositor2_FWD_DEFINED__ */


#ifndef __IDirectManipulationUpdateHandler_FWD_DEFINED__
#define __IDirectManipulationUpdateHandler_FWD_DEFINED__
typedef interface IDirectManipulationUpdateHandler IDirectManipulationUpdateHandler;

#endif 	/* __IDirectManipulationUpdateHandler_FWD_DEFINED__ */


#ifndef __IDirectManipulationUpdateManager_FWD_DEFINED__
#define __IDirectManipulationUpdateManager_FWD_DEFINED__
typedef interface IDirectManipulationUpdateManager IDirectManipulationUpdateManager;

#endif 	/* __IDirectManipulationUpdateManager_FWD_DEFINED__ */


#ifndef __IDirectManipulationAutoScrollBehavior_FWD_DEFINED__
#define __IDirectManipulationAutoScrollBehavior_FWD_DEFINED__
typedef interface IDirectManipulationAutoScrollBehavior IDirectManipulationAutoScrollBehavior;

#endif 	/* __IDirectManipulationAutoScrollBehavior_FWD_DEFINED__ */


#ifndef __IDirectManipulationDeferContactService_FWD_DEFINED__
#define __IDirectManipulationDeferContactService_FWD_DEFINED__
typedef interface IDirectManipulationDeferContactService IDirectManipulationDeferContactService;

#endif 	/* __IDirectManipulationDeferContactService_FWD_DEFINED__ */


#ifndef __IDirectManipulationSnapPointsInertiaBehavior_FWD_DEFINED__
#define __IDirectManipulationSnapPointsInertiaBehavior_FWD_DEFINED__
typedef interface IDirectManipulationSnapPointsInertiaBehavior IDirectManipulationSnapPointsInertiaBehavior;

#endif 	/* __IDirectManipulationSnapPointsInertiaBehavior_FWD_DEFINED__ */


#ifndef __IDirectManipulationContent2_FWD_DEFINED__
#define __IDirectManipulationContent2_FWD_DEFINED__
typedef interface IDirectManipulationContent2 IDirectManipulationContent2;

#endif 	/* __IDirectManipulationContent2_FWD_DEFINED__ */


#ifndef __IDirectManipulationViewport3_FWD_DEFINED__
#define __IDirectManipulationViewport3_FWD_DEFINED__
typedef interface IDirectManipulationViewport3 IDirectManipulationViewport3;

#endif 	/* __IDirectManipulationViewport3_FWD_DEFINED__ */


#ifndef __IDirectManipulationPrimaryContent2_FWD_DEFINED__
#define __IDirectManipulationPrimaryContent2_FWD_DEFINED__
typedef interface IDirectManipulationPrimaryContent2 IDirectManipulationPrimaryContent2;

#endif 	/* __IDirectManipulationPrimaryContent2_FWD_DEFINED__ */


#ifndef __IDirectManipulationParametricMotionBehavior_FWD_DEFINED__
#define __IDirectManipulationParametricMotionBehavior_FWD_DEFINED__
typedef interface IDirectManipulationParametricMotionBehavior IDirectManipulationParametricMotionBehavior;

#endif 	/* __IDirectManipulationParametricMotionBehavior_FWD_DEFINED__ */


#ifndef __IDirectManipulationParametricMotionCurve_FWD_DEFINED__
#define __IDirectManipulationParametricMotionCurve_FWD_DEFINED__
typedef interface IDirectManipulationParametricMotionCurve IDirectManipulationParametricMotionCurve;

#endif 	/* __IDirectManipulationParametricMotionCurve_FWD_DEFINED__ */


#ifndef __IDirectManipulationParametricRestPointList_FWD_DEFINED__
#define __IDirectManipulationParametricRestPointList_FWD_DEFINED__
typedef interface IDirectManipulationParametricRestPointList IDirectManipulationParametricRestPointList;

#endif 	/* __IDirectManipulationParametricRestPointList_FWD_DEFINED__ */


#ifndef __IDirectManipulationParametricRestPointBehavior_FWD_DEFINED__
#define __IDirectManipulationParametricRestPointBehavior_FWD_DEFINED__
typedef interface IDirectManipulationParametricRestPointBehavior IDirectManipulationParametricRestPointBehavior;

#endif 	/* __IDirectManipulationParametricRestPointBehavior_FWD_DEFINED__ */


#ifndef __IDirectManipulationCompositorPartner_FWD_DEFINED__
#define __IDirectManipulationCompositorPartner_FWD_DEFINED__
typedef interface IDirectManipulationCompositorPartner IDirectManipulationCompositorPartner;

#endif 	/* __IDirectManipulationCompositorPartner_FWD_DEFINED__ */


#ifndef __IDirectManipulationManagerPartner_FWD_DEFINED__
#define __IDirectManipulationManagerPartner_FWD_DEFINED__
typedef interface IDirectManipulationManagerPartner IDirectManipulationManagerPartner;

#endif 	/* __IDirectManipulationManagerPartner_FWD_DEFINED__ */


#ifndef __IDirectManipulationViewportPartner_FWD_DEFINED__
#define __IDirectManipulationViewportPartner_FWD_DEFINED__
typedef interface IDirectManipulationViewportPartner IDirectManipulationViewportPartner;

#endif 	/* __IDirectManipulationViewportPartner_FWD_DEFINED__ */


#ifndef __DirectManipulationViewport_FWD_DEFINED__
#define __DirectManipulationViewport_FWD_DEFINED__

#ifdef __cplusplus
typedef class DirectManipulationViewport DirectManipulationViewport;
#else
typedef struct DirectManipulationViewport DirectManipulationViewport;
#endif /* __cplusplus */

#endif 	/* __DirectManipulationViewport_FWD_DEFINED__ */


#ifndef __DirectManipulationUpdateManager_FWD_DEFINED__
#define __DirectManipulationUpdateManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class DirectManipulationUpdateManager DirectManipulationUpdateManager;
#else
typedef struct DirectManipulationUpdateManager DirectManipulationUpdateManager;
#endif /* __cplusplus */

#endif 	/* __DirectManipulationUpdateManager_FWD_DEFINED__ */


#ifndef __DirectManipulationPrimaryContent_FWD_DEFINED__
#define __DirectManipulationPrimaryContent_FWD_DEFINED__

#ifdef __cplusplus
typedef class DirectManipulationPrimaryContent DirectManipulationPrimaryContent;
#else
typedef struct DirectManipulationPrimaryContent DirectManipulationPrimaryContent;
#endif /* __cplusplus */

#endif 	/* __DirectManipulationPrimaryContent_FWD_DEFINED__ */


#ifndef __DirectManipulationManager_FWD_DEFINED__
#define __DirectManipulationManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class DirectManipulationManager DirectManipulationManager;
#else
typedef struct DirectManipulationManager DirectManipulationManager;
#endif /* __cplusplus */

#endif 	/* __DirectManipulationManager_FWD_DEFINED__ */


#ifndef __DirectManipulationSharedManager_FWD_DEFINED__
#define __DirectManipulationSharedManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class DirectManipulationSharedManager DirectManipulationSharedManager;
#else
typedef struct DirectManipulationSharedManager DirectManipulationSharedManager;
#endif /* __cplusplus */

#endif 	/* __DirectManipulationSharedManager_FWD_DEFINED__ */


#ifndef __DCompManipulationCompositor_FWD_DEFINED__
#define __DCompManipulationCompositor_FWD_DEFINED__

#ifdef __cplusplus
typedef class DCompManipulationCompositor DCompManipulationCompositor;
#else
typedef struct DCompManipulationCompositor DCompManipulationCompositor;
#endif /* __cplusplus */

#endif 	/* __DCompManipulationCompositor_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_directmanipulation_0000_0000 */
/* [local] */ 

//--------------------------------------------------------------------------
//
//  directmanipulation.h
//
//  Direct Manipulation interface definitions and related types and enums
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WIN8)
#if 0
typedef void *HWND;

#endif







typedef 
enum DIRECTMANIPULATION_STATUS
    {
        DIRECTMANIPULATION_BUILDING	= 0,
        DIRECTMANIPULATION_ENABLED	= 1,
        DIRECTMANIPULATION_DISABLED	= 2,
        DIRECTMANIPULATION_RUNNING	= 3,
        DIRECTMANIPULATION_INERTIA	= 4,
        DIRECTMANIPULATION_READY	= 5,
        DIRECTMANIPULATION_SUSPENDED	= 6
    } 	DIRECTMANIPULATION_STATUS;

typedef 
enum DIRECTMANIPULATION_HITTEST_TYPE
    {
        DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS	= 0,
        DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS	= 0x1,
        DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS	= 0x2
    } 	DIRECTMANIPULATION_HITTEST_TYPE;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_HITTEST_TYPE)
typedef 
enum DIRECTMANIPULATION_CONFIGURATION
    {
        DIRECTMANIPULATION_CONFIGURATION_NONE	= 0,
        DIRECTMANIPULATION_CONFIGURATION_INTERACTION	= 0x1,
        DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X	= 0x2,
        DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y	= 0x4,
        DIRECTMANIPULATION_CONFIGURATION_SCALING	= 0x10,
        DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA	= 0x20,
        DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA	= 0x80,
        DIRECTMANIPULATION_CONFIGURATION_RAILS_X	= 0x100,
        DIRECTMANIPULATION_CONFIGURATION_RAILS_Y	= 0x200
    } 	DIRECTMANIPULATION_CONFIGURATION;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_CONFIGURATION)
typedef 
enum DIRECTMANIPULATION_GESTURE_CONFIGURATION
    {
        DIRECTMANIPULATION_GESTURE_NONE	= 0,
        DIRECTMANIPULATION_GESTURE_DEFAULT	= 0,
        DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL	= 0x8,
        DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL	= 0x10,
        DIRECTMANIPULATION_GESTURE_PINCH_ZOOM	= 0x20
    } 	DIRECTMANIPULATION_GESTURE_CONFIGURATION;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_GESTURE_CONFIGURATION)
typedef 
enum DIRECTMANIPULATION_MOTION_TYPES
    {
        DIRECTMANIPULATION_MOTION_NONE	= 0,
        DIRECTMANIPULATION_MOTION_TRANSLATEX	= 0x1,
        DIRECTMANIPULATION_MOTION_TRANSLATEY	= 0x2,
        DIRECTMANIPULATION_MOTION_ZOOM	= 0x4,
        DIRECTMANIPULATION_MOTION_CENTERX	= 0x10,
        DIRECTMANIPULATION_MOTION_CENTERY	= 0x20,
        DIRECTMANIPULATION_MOTION_ALL	= ( ( ( ( DIRECTMANIPULATION_MOTION_TRANSLATEX | DIRECTMANIPULATION_MOTION_TRANSLATEY )  | DIRECTMANIPULATION_MOTION_ZOOM )  | DIRECTMANIPULATION_MOTION_CENTERX )  | DIRECTMANIPULATION_MOTION_CENTERY ) 
    } 	DIRECTMANIPULATION_MOTION_TYPES;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_MOTION_TYPES)
typedef 
enum DIRECTMANIPULATION_VIEWPORT_OPTIONS
    {
        DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT	= 0,
        DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE	= 0x1,
        DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE	= 0x2,
        DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT	= 0x4,
        DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST	= 0x8,
        DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING	= 0x10
    } 	DIRECTMANIPULATION_VIEWPORT_OPTIONS;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_VIEWPORT_OPTIONS)
typedef 
enum DIRECTMANIPULATION_SNAPPOINT_TYPE
    {
        DIRECTMANIPULATION_SNAPPOINT_MANDATORY	= 0,
        DIRECTMANIPULATION_SNAPPOINT_OPTIONAL	= 1,
        DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE	= 2,
        DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE	= 3
    } 	DIRECTMANIPULATION_SNAPPOINT_TYPE;

typedef 
enum DIRECTMANIPULATION_SNAPPOINT_COORDINATE
    {
        DIRECTMANIPULATION_COORDINATE_BOUNDARY	= 0,
        DIRECTMANIPULATION_COORDINATE_ORIGIN	= 0x1,
        DIRECTMANIPULATION_COORDINATE_MIRRORED	= 0x10
    } 	DIRECTMANIPULATION_SNAPPOINT_COORDINATE;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_SNAPPOINT_COORDINATE)
typedef 
enum DIRECTMANIPULATION_HORIZONTALALIGNMENT
    {
        DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE	= 0,
        DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT	= 0x1,
        DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER	= 0x2,
        DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT	= 0x4,
        DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER	= 0x8
    } 	DIRECTMANIPULATION_HORIZONTALALIGNMENT;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_HORIZONTALALIGNMENT)
typedef 
enum DIRECTMANIPULATION_VERTICALALIGNMENT
    {
        DIRECTMANIPULATION_VERTICALALIGNMENT_NONE	= 0,
        DIRECTMANIPULATION_VERTICALALIGNMENT_TOP	= 0x1,
        DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER	= 0x2,
        DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM	= 0x4,
        DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER	= 0x8
    } 	DIRECTMANIPULATION_VERTICALALIGNMENT;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_VERTICALALIGNMENT)
typedef 
enum DIRECTMANIPULATION_INPUT_MODE
    {
        DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC	= 0,
        DIRECTMANIPULATION_INPUT_MODE_MANUAL	= 1
    } 	DIRECTMANIPULATION_INPUT_MODE;

#define	DIRECTMANIPULATION_KEYBOARDFOCUS	( 0xfffffffe )

#define	DIRECTMANIPULATION_MOUSEFOCUS	( 0xfffffffd )

#define	DIRECTMANIPULATION_MINIMUM_ZOOM	( ( float  )0.1 )

DEFINE_GUID(CLSID_VerticalIndicatorContent, 0xa10b5f17, 0xafe0, 0x4aa2, 0x91, 0xe9, 0x3e, 0x70, 0x1, 0xd2, 0xe6, 0xb4);
DEFINE_GUID(CLSID_HorizontalIndicatorContent, 0xe7d18cf5, 0x3ec7, 0x44d5, 0xa7, 0x6b, 0x37, 0x70, 0xf3, 0xcf, 0x90, 0x3d);
DEFINE_GUID(CLSID_VirtualViewportContent, 0x3206a19a, 0x86f0, 0x4cb4, 0xa7, 0xf3, 0x16, 0xe3, 0xb7, 0xe2, 0xd8, 0x52);
DEFINE_GUID(CLSID_DragDropConfigurationBehavior, 0x09b01b3e, 0xba6c, 0x454d, 0x82, 0xe8, 0x95, 0xe3, 0x52, 0x32, 0x9f, 0x23);
DEFINE_GUID(CLSID_AutoScrollBehavior, 0x26126a51, 0x3c70, 0x4c9a, 0xae, 0xc2, 0x94, 0x88, 0x49, 0xee, 0xb0, 0x93);
DEFINE_GUID(CLSID_DeferContactService, 0xd7b67cf4, 0x84bb, 0x434e, 0x86, 0xae, 0x65, 0x92, 0xbb, 0xc9, 0xab, 0xd9);


extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0000_v0_0_s_ifspec;

#ifndef __IDirectManipulationManager_INTERFACE_DEFINED__
#define __IDirectManipulationManager_INTERFACE_DEFINED__

/* interface IDirectManipulationManager */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FBF5D3B4-70C7-4163-9322-5A6F660D6FBC")
    IDirectManipulationManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Activate( 
            /* [annotation][in] */ 
            _In_  HWND window) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Deactivate( 
            /* [annotation][in] */ 
            _In_  HWND window) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterHitTestTarget( 
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_opt_  HWND hitTestWindow,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_HITTEST_TYPE type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessInput( 
            /* [annotation][in] */ 
            _In_  const MSG *message,
            /* [annotation][retval][out] */ 
            _Out_  BOOL *handled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUpdateManager( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateViewport( 
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateContent( 
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationManager * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_  HWND window);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_  HWND window);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, RegisterHitTestTarget)
        HRESULT ( STDMETHODCALLTYPE *RegisterHitTestTarget )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_opt_  HWND hitTestWindow,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_HITTEST_TYPE type);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, ProcessInput)
        HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_  const MSG *message,
            /* [annotation][retval][out] */ 
            _Out_  BOOL *handled);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, GetUpdateManager)
        HRESULT ( STDMETHODCALLTYPE *GetUpdateManager )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, CreateViewport)
        HRESULT ( STDMETHODCALLTYPE *CreateViewport )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, CreateContent)
        HRESULT ( STDMETHODCALLTYPE *CreateContent )( 
            IDirectManipulationManager * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        END_INTERFACE
    } IDirectManipulationManagerVtbl;

    interface IDirectManipulationManager
    {
        CONST_VTBL struct IDirectManipulationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationManager_Activate(This,window)	\
    ( (This)->lpVtbl -> Activate(This,window) ) 

#define IDirectManipulationManager_Deactivate(This,window)	\
    ( (This)->lpVtbl -> Deactivate(This,window) ) 

#define IDirectManipulationManager_RegisterHitTestTarget(This,window,hitTestWindow,type)	\
    ( (This)->lpVtbl -> RegisterHitTestTarget(This,window,hitTestWindow,type) ) 

#define IDirectManipulationManager_ProcessInput(This,message,handled)	\
    ( (This)->lpVtbl -> ProcessInput(This,message,handled) ) 

#define IDirectManipulationManager_GetUpdateManager(This,riid,object)	\
    ( (This)->lpVtbl -> GetUpdateManager(This,riid,object) ) 

#define IDirectManipulationManager_CreateViewport(This,frameInfo,window,riid,object)	\
    ( (This)->lpVtbl -> CreateViewport(This,frameInfo,window,riid,object) ) 

#define IDirectManipulationManager_CreateContent(This,frameInfo,clsid,riid,object)	\
    ( (This)->lpVtbl -> CreateContent(This,frameInfo,clsid,riid,object) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationManager_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationManager2_INTERFACE_DEFINED__
#define __IDirectManipulationManager2_INTERFACE_DEFINED__

/* interface IDirectManipulationManager2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FA1005E9-3D16-484C-BFC9-62B61E56EC4E")
    IDirectManipulationManager2 : public IDirectManipulationManager
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateBehavior( 
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationManager2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationManager2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_  HWND window);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_  HWND window);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, RegisterHitTestTarget)
        HRESULT ( STDMETHODCALLTYPE *RegisterHitTestTarget )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_opt_  HWND hitTestWindow,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_HITTEST_TYPE type);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, ProcessInput)
        HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_  const MSG *message,
            /* [annotation][retval][out] */ 
            _Out_  BOOL *handled);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, GetUpdateManager)
        HRESULT ( STDMETHODCALLTYPE *GetUpdateManager )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, CreateViewport)
        HRESULT ( STDMETHODCALLTYPE *CreateViewport )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, CreateContent)
        HRESULT ( STDMETHODCALLTYPE *CreateContent )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager2, CreateBehavior)
        HRESULT ( STDMETHODCALLTYPE *CreateBehavior )( 
            IDirectManipulationManager2 * This,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        END_INTERFACE
    } IDirectManipulationManager2Vtbl;

    interface IDirectManipulationManager2
    {
        CONST_VTBL struct IDirectManipulationManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationManager2_Activate(This,window)	\
    ( (This)->lpVtbl -> Activate(This,window) ) 

#define IDirectManipulationManager2_Deactivate(This,window)	\
    ( (This)->lpVtbl -> Deactivate(This,window) ) 

#define IDirectManipulationManager2_RegisterHitTestTarget(This,window,hitTestWindow,type)	\
    ( (This)->lpVtbl -> RegisterHitTestTarget(This,window,hitTestWindow,type) ) 

#define IDirectManipulationManager2_ProcessInput(This,message,handled)	\
    ( (This)->lpVtbl -> ProcessInput(This,message,handled) ) 

#define IDirectManipulationManager2_GetUpdateManager(This,riid,object)	\
    ( (This)->lpVtbl -> GetUpdateManager(This,riid,object) ) 

#define IDirectManipulationManager2_CreateViewport(This,frameInfo,window,riid,object)	\
    ( (This)->lpVtbl -> CreateViewport(This,frameInfo,window,riid,object) ) 

#define IDirectManipulationManager2_CreateContent(This,frameInfo,clsid,riid,object)	\
    ( (This)->lpVtbl -> CreateContent(This,frameInfo,clsid,riid,object) ) 


#define IDirectManipulationManager2_CreateBehavior(This,clsid,riid,object)	\
    ( (This)->lpVtbl -> CreateBehavior(This,clsid,riid,object) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationManager2_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationManager3_INTERFACE_DEFINED__
#define __IDirectManipulationManager3_INTERFACE_DEFINED__

/* interface IDirectManipulationManager3 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationManager3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2CB6B33D-FFE8-488C-B750-FBDFE88DCA8C")
    IDirectManipulationManager3 : public IDirectManipulationManager2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetService( 
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationManager3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationManager3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationManager3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationManager3 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, Activate)
        HRESULT ( STDMETHODCALLTYPE *Activate )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  HWND window);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, Deactivate)
        HRESULT ( STDMETHODCALLTYPE *Deactivate )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  HWND window);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, RegisterHitTestTarget)
        HRESULT ( STDMETHODCALLTYPE *RegisterHitTestTarget )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_opt_  HWND hitTestWindow,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_HITTEST_TYPE type);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, ProcessInput)
        HRESULT ( STDMETHODCALLTYPE *ProcessInput )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  const MSG *message,
            /* [annotation][retval][out] */ 
            _Out_  BOOL *handled);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, GetUpdateManager)
        HRESULT ( STDMETHODCALLTYPE *GetUpdateManager )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, CreateViewport)
        HRESULT ( STDMETHODCALLTYPE *CreateViewport )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  HWND window,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager, CreateContent)
        HRESULT ( STDMETHODCALLTYPE *CreateContent )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager2, CreateBehavior)
        HRESULT ( STDMETHODCALLTYPE *CreateBehavior )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationManager3, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            IDirectManipulationManager3 * This,
            /* [annotation][in] */ 
            _In_  REFCLSID clsid,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        END_INTERFACE
    } IDirectManipulationManager3Vtbl;

    interface IDirectManipulationManager3
    {
        CONST_VTBL struct IDirectManipulationManager3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationManager3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationManager3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationManager3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationManager3_Activate(This,window)	\
    ( (This)->lpVtbl -> Activate(This,window) ) 

#define IDirectManipulationManager3_Deactivate(This,window)	\
    ( (This)->lpVtbl -> Deactivate(This,window) ) 

#define IDirectManipulationManager3_RegisterHitTestTarget(This,window,hitTestWindow,type)	\
    ( (This)->lpVtbl -> RegisterHitTestTarget(This,window,hitTestWindow,type) ) 

#define IDirectManipulationManager3_ProcessInput(This,message,handled)	\
    ( (This)->lpVtbl -> ProcessInput(This,message,handled) ) 

#define IDirectManipulationManager3_GetUpdateManager(This,riid,object)	\
    ( (This)->lpVtbl -> GetUpdateManager(This,riid,object) ) 

#define IDirectManipulationManager3_CreateViewport(This,frameInfo,window,riid,object)	\
    ( (This)->lpVtbl -> CreateViewport(This,frameInfo,window,riid,object) ) 

#define IDirectManipulationManager3_CreateContent(This,frameInfo,clsid,riid,object)	\
    ( (This)->lpVtbl -> CreateContent(This,frameInfo,clsid,riid,object) ) 


#define IDirectManipulationManager3_CreateBehavior(This,clsid,riid,object)	\
    ( (This)->lpVtbl -> CreateBehavior(This,clsid,riid,object) ) 


#define IDirectManipulationManager3_GetService(This,clsid,riid,object)	\
    ( (This)->lpVtbl -> GetService(This,clsid,riid,object) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationManager3_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationViewport_INTERFACE_DEFINED__
#define __IDirectManipulationViewport_INTERFACE_DEFINED__

/* interface IDirectManipulationViewport */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationViewport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("28b85a3d-60a0-48bd-9ba1-5ce8d9ea3a6d")
    IDirectManipulationViewport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Enable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContact( 
            /* [annotation][in] */ 
            _In_  UINT32 pointerId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseContact( 
            /* [annotation][in] */ 
            _In_  UINT32 pointerId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseAllContacts( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][retval][out] */ 
            _Out_  DIRECTMANIPULATION_STATUS *status) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_opt_  void **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewportRect( 
            /* [annotation][retval][out] */ 
            _Out_  RECT *viewport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetViewportRect( 
            /* [annotation][in] */ 
            _In_  const RECT *viewport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ZoomToRect( 
            /* [annotation][in] */ 
            _In_  const float left,
            /* [annotation][in] */ 
            _In_  const float top,
            /* [annotation][in] */ 
            _In_  const float right,
            /* [annotation][in] */ 
            _In_  const float bottom,
            /* [annotation][in] */ 
            _In_  BOOL animate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetViewportTransform( 
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SyncDisplayTransform( 
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrimaryContent( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddContent( 
            /* [annotation][unique][in] */ 
            _In_  IDirectManipulationContent *content) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveContent( 
            /* [annotation][unique][in] */ 
            _In_  IDirectManipulationContent *content) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetViewportOptions( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_VIEWPORT_OPTIONS options) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddConfiguration( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveConfiguration( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateConfiguration( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetManualGesture( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_GESTURE_CONFIGURATION configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetChaining( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES enabledTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddEventHandler( 
            /* [annotation][in] */ 
            _In_opt_  HWND window,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewportEventHandler *eventHandler,
            /* [annotation][retval][out] */ 
            _Out_  DWORD *cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveEventHandler( 
            /* [annotation][in] */ 
            _In_  DWORD cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputMode( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INPUT_MODE mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUpdateMode( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INPUT_MODE mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abandon( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationViewportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationViewport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationViewport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationViewport * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IDirectManipulationViewport * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Disable)
        HRESULT ( STDMETHODCALLTYPE *Disable )( 
            IDirectManipulationViewport * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetContact)
        HRESULT ( STDMETHODCALLTYPE *SetContact )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ReleaseContact)
        HRESULT ( STDMETHODCALLTYPE *ReleaseContact )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ReleaseAllContacts)
        HRESULT ( STDMETHODCALLTYPE *ReleaseAllContacts )( 
            IDirectManipulationViewport * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IDirectManipulationViewport * This,
            /* [annotation][retval][out] */ 
            _Out_  DIRECTMANIPULATION_STATUS *status);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_opt_  void **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetTag)
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IDirectManipulationViewport * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetViewportRect)
        HRESULT ( STDMETHODCALLTYPE *GetViewportRect )( 
            IDirectManipulationViewport * This,
            /* [annotation][retval][out] */ 
            _Out_  RECT *viewport);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetViewportRect)
        HRESULT ( STDMETHODCALLTYPE *SetViewportRect )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  const RECT *viewport);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ZoomToRect)
        HRESULT ( STDMETHODCALLTYPE *ZoomToRect )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  const float left,
            /* [annotation][in] */ 
            _In_  const float top,
            /* [annotation][in] */ 
            _In_  const float right,
            /* [annotation][in] */ 
            _In_  const float bottom,
            /* [annotation][in] */ 
            _In_  BOOL animate);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetViewportTransform)
        HRESULT ( STDMETHODCALLTYPE *SetViewportTransform )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SyncDisplayTransform)
        HRESULT ( STDMETHODCALLTYPE *SyncDisplayTransform )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetPrimaryContent)
        HRESULT ( STDMETHODCALLTYPE *GetPrimaryContent )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, AddContent)
        HRESULT ( STDMETHODCALLTYPE *AddContent )( 
            IDirectManipulationViewport * This,
            /* [annotation][unique][in] */ 
            _In_  IDirectManipulationContent *content);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, RemoveContent)
        HRESULT ( STDMETHODCALLTYPE *RemoveContent )( 
            IDirectManipulationViewport * This,
            /* [annotation][unique][in] */ 
            _In_  IDirectManipulationContent *content);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetViewportOptions)
        HRESULT ( STDMETHODCALLTYPE *SetViewportOptions )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_VIEWPORT_OPTIONS options);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, AddConfiguration)
        HRESULT ( STDMETHODCALLTYPE *AddConfiguration )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, RemoveConfiguration)
        HRESULT ( STDMETHODCALLTYPE *RemoveConfiguration )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ActivateConfiguration)
        HRESULT ( STDMETHODCALLTYPE *ActivateConfiguration )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetManualGesture)
        HRESULT ( STDMETHODCALLTYPE *SetManualGesture )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_GESTURE_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetChaining)
        HRESULT ( STDMETHODCALLTYPE *SetChaining )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES enabledTypes);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, AddEventHandler)
        HRESULT ( STDMETHODCALLTYPE *AddEventHandler )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_opt_  HWND window,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewportEventHandler *eventHandler,
            /* [annotation][retval][out] */ 
            _Out_  DWORD *cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, RemoveEventHandler)
        HRESULT ( STDMETHODCALLTYPE *RemoveEventHandler )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DWORD cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetInputMode)
        HRESULT ( STDMETHODCALLTYPE *SetInputMode )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INPUT_MODE mode);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetUpdateMode)
        HRESULT ( STDMETHODCALLTYPE *SetUpdateMode )( 
            IDirectManipulationViewport * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INPUT_MODE mode);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IDirectManipulationViewport * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Abandon)
        HRESULT ( STDMETHODCALLTYPE *Abandon )( 
            IDirectManipulationViewport * This);
        
        END_INTERFACE
    } IDirectManipulationViewportVtbl;

    interface IDirectManipulationViewport
    {
        CONST_VTBL struct IDirectManipulationViewportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationViewport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationViewport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationViewport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationViewport_Enable(This)	\
    ( (This)->lpVtbl -> Enable(This) ) 

#define IDirectManipulationViewport_Disable(This)	\
    ( (This)->lpVtbl -> Disable(This) ) 

#define IDirectManipulationViewport_SetContact(This,pointerId)	\
    ( (This)->lpVtbl -> SetContact(This,pointerId) ) 

#define IDirectManipulationViewport_ReleaseContact(This,pointerId)	\
    ( (This)->lpVtbl -> ReleaseContact(This,pointerId) ) 

#define IDirectManipulationViewport_ReleaseAllContacts(This)	\
    ( (This)->lpVtbl -> ReleaseAllContacts(This) ) 

#define IDirectManipulationViewport_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#define IDirectManipulationViewport_GetTag(This,riid,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,riid,object,id) ) 

#define IDirectManipulationViewport_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IDirectManipulationViewport_GetViewportRect(This,viewport)	\
    ( (This)->lpVtbl -> GetViewportRect(This,viewport) ) 

#define IDirectManipulationViewport_SetViewportRect(This,viewport)	\
    ( (This)->lpVtbl -> SetViewportRect(This,viewport) ) 

#define IDirectManipulationViewport_ZoomToRect(This,left,top,right,bottom,animate)	\
    ( (This)->lpVtbl -> ZoomToRect(This,left,top,right,bottom,animate) ) 

#define IDirectManipulationViewport_SetViewportTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> SetViewportTransform(This,matrix,pointCount) ) 

#define IDirectManipulationViewport_SyncDisplayTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> SyncDisplayTransform(This,matrix,pointCount) ) 

#define IDirectManipulationViewport_GetPrimaryContent(This,riid,object)	\
    ( (This)->lpVtbl -> GetPrimaryContent(This,riid,object) ) 

#define IDirectManipulationViewport_AddContent(This,content)	\
    ( (This)->lpVtbl -> AddContent(This,content) ) 

#define IDirectManipulationViewport_RemoveContent(This,content)	\
    ( (This)->lpVtbl -> RemoveContent(This,content) ) 

#define IDirectManipulationViewport_SetViewportOptions(This,options)	\
    ( (This)->lpVtbl -> SetViewportOptions(This,options) ) 

#define IDirectManipulationViewport_AddConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> AddConfiguration(This,configuration) ) 

#define IDirectManipulationViewport_RemoveConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> RemoveConfiguration(This,configuration) ) 

#define IDirectManipulationViewport_ActivateConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> ActivateConfiguration(This,configuration) ) 

#define IDirectManipulationViewport_SetManualGesture(This,configuration)	\
    ( (This)->lpVtbl -> SetManualGesture(This,configuration) ) 

#define IDirectManipulationViewport_SetChaining(This,enabledTypes)	\
    ( (This)->lpVtbl -> SetChaining(This,enabledTypes) ) 

#define IDirectManipulationViewport_AddEventHandler(This,window,eventHandler,cookie)	\
    ( (This)->lpVtbl -> AddEventHandler(This,window,eventHandler,cookie) ) 

#define IDirectManipulationViewport_RemoveEventHandler(This,cookie)	\
    ( (This)->lpVtbl -> RemoveEventHandler(This,cookie) ) 

#define IDirectManipulationViewport_SetInputMode(This,mode)	\
    ( (This)->lpVtbl -> SetInputMode(This,mode) ) 

#define IDirectManipulationViewport_SetUpdateMode(This,mode)	\
    ( (This)->lpVtbl -> SetUpdateMode(This,mode) ) 

#define IDirectManipulationViewport_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IDirectManipulationViewport_Abandon(This)	\
    ( (This)->lpVtbl -> Abandon(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationViewport_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationViewport2_INTERFACE_DEFINED__
#define __IDirectManipulationViewport2_INTERFACE_DEFINED__

/* interface IDirectManipulationViewport2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationViewport2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("923CCAAC-61E1-4385-B726-017AF189882A")
    IDirectManipulationViewport2 : public IDirectManipulationViewport
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddBehavior( 
            /* [annotation][unique][in] */ 
            _In_  IUnknown *behavior,
            /* [annotation][retval][out] */ 
            _Out_  DWORD *cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveBehavior( 
            /* [annotation][in] */ 
            _In_  DWORD cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllBehaviors( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationViewport2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationViewport2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Disable)
        HRESULT ( STDMETHODCALLTYPE *Disable )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetContact)
        HRESULT ( STDMETHODCALLTYPE *SetContact )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ReleaseContact)
        HRESULT ( STDMETHODCALLTYPE *ReleaseContact )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ReleaseAllContacts)
        HRESULT ( STDMETHODCALLTYPE *ReleaseAllContacts )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][retval][out] */ 
            _Out_  DIRECTMANIPULATION_STATUS *status);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_opt_  void **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetTag)
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetViewportRect)
        HRESULT ( STDMETHODCALLTYPE *GetViewportRect )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][retval][out] */ 
            _Out_  RECT *viewport);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetViewportRect)
        HRESULT ( STDMETHODCALLTYPE *SetViewportRect )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  const RECT *viewport);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ZoomToRect)
        HRESULT ( STDMETHODCALLTYPE *ZoomToRect )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  const float left,
            /* [annotation][in] */ 
            _In_  const float top,
            /* [annotation][in] */ 
            _In_  const float right,
            /* [annotation][in] */ 
            _In_  const float bottom,
            /* [annotation][in] */ 
            _In_  BOOL animate);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetViewportTransform)
        HRESULT ( STDMETHODCALLTYPE *SetViewportTransform )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SyncDisplayTransform)
        HRESULT ( STDMETHODCALLTYPE *SyncDisplayTransform )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, GetPrimaryContent)
        HRESULT ( STDMETHODCALLTYPE *GetPrimaryContent )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, AddContent)
        HRESULT ( STDMETHODCALLTYPE *AddContent )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][unique][in] */ 
            _In_  IDirectManipulationContent *content);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, RemoveContent)
        HRESULT ( STDMETHODCALLTYPE *RemoveContent )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][unique][in] */ 
            _In_  IDirectManipulationContent *content);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetViewportOptions)
        HRESULT ( STDMETHODCALLTYPE *SetViewportOptions )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_VIEWPORT_OPTIONS options);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, AddConfiguration)
        HRESULT ( STDMETHODCALLTYPE *AddConfiguration )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, RemoveConfiguration)
        HRESULT ( STDMETHODCALLTYPE *RemoveConfiguration )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, ActivateConfiguration)
        HRESULT ( STDMETHODCALLTYPE *ActivateConfiguration )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetManualGesture)
        HRESULT ( STDMETHODCALLTYPE *SetManualGesture )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_GESTURE_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetChaining)
        HRESULT ( STDMETHODCALLTYPE *SetChaining )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES enabledTypes);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, AddEventHandler)
        HRESULT ( STDMETHODCALLTYPE *AddEventHandler )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_opt_  HWND window,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewportEventHandler *eventHandler,
            /* [annotation][retval][out] */ 
            _Out_  DWORD *cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, RemoveEventHandler)
        HRESULT ( STDMETHODCALLTYPE *RemoveEventHandler )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DWORD cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetInputMode)
        HRESULT ( STDMETHODCALLTYPE *SetInputMode )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INPUT_MODE mode);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, SetUpdateMode)
        HRESULT ( STDMETHODCALLTYPE *SetUpdateMode )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INPUT_MODE mode);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport, Abandon)
        HRESULT ( STDMETHODCALLTYPE *Abandon )( 
            IDirectManipulationViewport2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport2, AddBehavior)
        HRESULT ( STDMETHODCALLTYPE *AddBehavior )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][unique][in] */ 
            _In_  IUnknown *behavior,
            /* [annotation][retval][out] */ 
            _Out_  DWORD *cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport2, RemoveBehavior)
        HRESULT ( STDMETHODCALLTYPE *RemoveBehavior )( 
            IDirectManipulationViewport2 * This,
            /* [annotation][in] */ 
            _In_  DWORD cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewport2, RemoveAllBehaviors)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllBehaviors )( 
            IDirectManipulationViewport2 * This);
        
        END_INTERFACE
    } IDirectManipulationViewport2Vtbl;

    interface IDirectManipulationViewport2
    {
        CONST_VTBL struct IDirectManipulationViewport2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationViewport2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationViewport2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationViewport2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationViewport2_Enable(This)	\
    ( (This)->lpVtbl -> Enable(This) ) 

#define IDirectManipulationViewport2_Disable(This)	\
    ( (This)->lpVtbl -> Disable(This) ) 

#define IDirectManipulationViewport2_SetContact(This,pointerId)	\
    ( (This)->lpVtbl -> SetContact(This,pointerId) ) 

#define IDirectManipulationViewport2_ReleaseContact(This,pointerId)	\
    ( (This)->lpVtbl -> ReleaseContact(This,pointerId) ) 

#define IDirectManipulationViewport2_ReleaseAllContacts(This)	\
    ( (This)->lpVtbl -> ReleaseAllContacts(This) ) 

#define IDirectManipulationViewport2_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#define IDirectManipulationViewport2_GetTag(This,riid,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,riid,object,id) ) 

#define IDirectManipulationViewport2_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IDirectManipulationViewport2_GetViewportRect(This,viewport)	\
    ( (This)->lpVtbl -> GetViewportRect(This,viewport) ) 

#define IDirectManipulationViewport2_SetViewportRect(This,viewport)	\
    ( (This)->lpVtbl -> SetViewportRect(This,viewport) ) 

#define IDirectManipulationViewport2_ZoomToRect(This,left,top,right,bottom,animate)	\
    ( (This)->lpVtbl -> ZoomToRect(This,left,top,right,bottom,animate) ) 

#define IDirectManipulationViewport2_SetViewportTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> SetViewportTransform(This,matrix,pointCount) ) 

#define IDirectManipulationViewport2_SyncDisplayTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> SyncDisplayTransform(This,matrix,pointCount) ) 

#define IDirectManipulationViewport2_GetPrimaryContent(This,riid,object)	\
    ( (This)->lpVtbl -> GetPrimaryContent(This,riid,object) ) 

#define IDirectManipulationViewport2_AddContent(This,content)	\
    ( (This)->lpVtbl -> AddContent(This,content) ) 

#define IDirectManipulationViewport2_RemoveContent(This,content)	\
    ( (This)->lpVtbl -> RemoveContent(This,content) ) 

#define IDirectManipulationViewport2_SetViewportOptions(This,options)	\
    ( (This)->lpVtbl -> SetViewportOptions(This,options) ) 

#define IDirectManipulationViewport2_AddConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> AddConfiguration(This,configuration) ) 

#define IDirectManipulationViewport2_RemoveConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> RemoveConfiguration(This,configuration) ) 

#define IDirectManipulationViewport2_ActivateConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> ActivateConfiguration(This,configuration) ) 

#define IDirectManipulationViewport2_SetManualGesture(This,configuration)	\
    ( (This)->lpVtbl -> SetManualGesture(This,configuration) ) 

#define IDirectManipulationViewport2_SetChaining(This,enabledTypes)	\
    ( (This)->lpVtbl -> SetChaining(This,enabledTypes) ) 

#define IDirectManipulationViewport2_AddEventHandler(This,window,eventHandler,cookie)	\
    ( (This)->lpVtbl -> AddEventHandler(This,window,eventHandler,cookie) ) 

#define IDirectManipulationViewport2_RemoveEventHandler(This,cookie)	\
    ( (This)->lpVtbl -> RemoveEventHandler(This,cookie) ) 

#define IDirectManipulationViewport2_SetInputMode(This,mode)	\
    ( (This)->lpVtbl -> SetInputMode(This,mode) ) 

#define IDirectManipulationViewport2_SetUpdateMode(This,mode)	\
    ( (This)->lpVtbl -> SetUpdateMode(This,mode) ) 

#define IDirectManipulationViewport2_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IDirectManipulationViewport2_Abandon(This)	\
    ( (This)->lpVtbl -> Abandon(This) ) 


#define IDirectManipulationViewport2_AddBehavior(This,behavior,cookie)	\
    ( (This)->lpVtbl -> AddBehavior(This,behavior,cookie) ) 

#define IDirectManipulationViewport2_RemoveBehavior(This,cookie)	\
    ( (This)->lpVtbl -> RemoveBehavior(This,cookie) ) 

#define IDirectManipulationViewport2_RemoveAllBehaviors(This)	\
    ( (This)->lpVtbl -> RemoveAllBehaviors(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationViewport2_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationViewportEventHandler_INTERFACE_DEFINED__
#define __IDirectManipulationViewportEventHandler_INTERFACE_DEFINED__

/* interface IDirectManipulationViewportEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationViewportEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("952121DA-D69F-45F9-B0F9-F23944321A6D")
    IDirectManipulationViewportEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnViewportStatusChanged( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport *viewport,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_STATUS current,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_STATUS previous) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnViewportUpdated( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport *viewport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnContentUpdated( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport *viewport,
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationViewportEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationViewportEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationViewportEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationViewportEventHandler * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewportEventHandler, OnViewportStatusChanged)
        HRESULT ( STDMETHODCALLTYPE *OnViewportStatusChanged )( 
            IDirectManipulationViewportEventHandler * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport *viewport,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_STATUS current,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_STATUS previous);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewportEventHandler, OnViewportUpdated)
        HRESULT ( STDMETHODCALLTYPE *OnViewportUpdated )( 
            IDirectManipulationViewportEventHandler * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport *viewport);
        
        DECLSPEC_XFGVIRT(IDirectManipulationViewportEventHandler, OnContentUpdated)
        HRESULT ( STDMETHODCALLTYPE *OnContentUpdated )( 
            IDirectManipulationViewportEventHandler * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport *viewport,
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content);
        
        END_INTERFACE
    } IDirectManipulationViewportEventHandlerVtbl;

    interface IDirectManipulationViewportEventHandler
    {
        CONST_VTBL struct IDirectManipulationViewportEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationViewportEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationViewportEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationViewportEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationViewportEventHandler_OnViewportStatusChanged(This,viewport,current,previous)	\
    ( (This)->lpVtbl -> OnViewportStatusChanged(This,viewport,current,previous) ) 

#define IDirectManipulationViewportEventHandler_OnViewportUpdated(This,viewport)	\
    ( (This)->lpVtbl -> OnViewportUpdated(This,viewport) ) 

#define IDirectManipulationViewportEventHandler_OnContentUpdated(This,viewport,content)	\
    ( (This)->lpVtbl -> OnContentUpdated(This,viewport,content) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationViewportEventHandler_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationContent_INTERFACE_DEFINED__
#define __IDirectManipulationContent_INTERFACE_DEFINED__

/* interface IDirectManipulationContent */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B89962CB-3D89-442B-BB58-5098FA0F9F16")
    IDirectManipulationContent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetContentRect( 
            /* [annotation][out] */ 
            _Out_  RECT *contentSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContentRect( 
            /* [annotation][in] */ 
            _In_  const RECT *contentSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetViewport( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTag( 
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_opt_  void **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputTransform( 
            /* [annotation][out] */ 
            _Out_writes_(pointCount)  float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContentTransform( 
            /* [annotation][out] */ 
            _Out_writes_(pointCount)  float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SyncContentTransform( 
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationContent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationContent * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, GetContentRect)
        HRESULT ( STDMETHODCALLTYPE *GetContentRect )( 
            IDirectManipulationContent * This,
            /* [annotation][out] */ 
            _Out_  RECT *contentSize);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, SetContentRect)
        HRESULT ( STDMETHODCALLTYPE *SetContentRect )( 
            IDirectManipulationContent * This,
            /* [annotation][in] */ 
            _In_  const RECT *contentSize);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, GetViewport)
        HRESULT ( STDMETHODCALLTYPE *GetViewport )( 
            IDirectManipulationContent * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][retval][iid_is][out] */ 
            _COM_Outptr_  void **object);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, GetTag)
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IDirectManipulationContent * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_opt_  void **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, SetTag)
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IDirectManipulationContent * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, GetOutputTransform)
        HRESULT ( STDMETHODCALLTYPE *GetOutputTransform )( 
            IDirectManipulationContent * This,
            /* [annotation][out] */ 
            _Out_writes_(pointCount)  float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, GetContentTransform)
        HRESULT ( STDMETHODCALLTYPE *GetContentTransform )( 
            IDirectManipulationContent * This,
            /* [annotation][out] */ 
            _Out_writes_(pointCount)  float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationContent, SyncContentTransform)
        HRESULT ( STDMETHODCALLTYPE *SyncContentTransform )( 
            IDirectManipulationContent * This,
            /* [annotation][in] */ 
            _In_reads_(pointCount)  const float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        END_INTERFACE
    } IDirectManipulationContentVtbl;

    interface IDirectManipulationContent
    {
        CONST_VTBL struct IDirectManipulationContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationContent_GetContentRect(This,contentSize)	\
    ( (This)->lpVtbl -> GetContentRect(This,contentSize) ) 

#define IDirectManipulationContent_SetContentRect(This,contentSize)	\
    ( (This)->lpVtbl -> SetContentRect(This,contentSize) ) 

#define IDirectManipulationContent_GetViewport(This,riid,object)	\
    ( (This)->lpVtbl -> GetViewport(This,riid,object) ) 

#define IDirectManipulationContent_GetTag(This,riid,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,riid,object,id) ) 

#define IDirectManipulationContent_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IDirectManipulationContent_GetOutputTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> GetOutputTransform(This,matrix,pointCount) ) 

#define IDirectManipulationContent_GetContentTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> GetContentTransform(This,matrix,pointCount) ) 

#define IDirectManipulationContent_SyncContentTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> SyncContentTransform(This,matrix,pointCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationContent_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationPrimaryContent_INTERFACE_DEFINED__
#define __IDirectManipulationPrimaryContent_INTERFACE_DEFINED__

/* interface IDirectManipulationPrimaryContent */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationPrimaryContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C12851E4-1698-4625-B9B1-7CA3EC18630B")
    IDirectManipulationPrimaryContent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSnapInterval( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_  float interval,
            /* [annotation][in] */ 
            _In_  float offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSnapPoints( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_reads_opt_(pointCount)  const float *points,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSnapType( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_SNAPPOINT_TYPE type) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSnapCoordinate( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_SNAPPOINT_COORDINATE coordinate,
            /* [annotation][in] */ 
            _In_  float origin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZoomBoundaries( 
            /* [annotation][in] */ 
            _In_  float zoomMinimum,
            /* [annotation][in] */ 
            _In_  float zoomMaximum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetHorizontalAlignment( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_HORIZONTALALIGNMENT alignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVerticalAlignment( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_VERTICALALIGNMENT alignment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInertiaEndTransform( 
            /* [annotation][out] */ 
            _Out_writes_(pointCount)  float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCenterPoint( 
            /* [annotation][out] */ 
            _Out_  float *centerX,
            /* [annotation][out] */ 
            _Out_  float *centerY) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationPrimaryContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationPrimaryContent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationPrimaryContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationPrimaryContent * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetSnapInterval)
        HRESULT ( STDMETHODCALLTYPE *SetSnapInterval )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_  float interval,
            /* [annotation][in] */ 
            _In_  float offset);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetSnapPoints)
        HRESULT ( STDMETHODCALLTYPE *SetSnapPoints )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_reads_opt_(pointCount)  const float *points,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetSnapType)
        HRESULT ( STDMETHODCALLTYPE *SetSnapType )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_SNAPPOINT_TYPE type);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetSnapCoordinate)
        HRESULT ( STDMETHODCALLTYPE *SetSnapCoordinate )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motion,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_SNAPPOINT_COORDINATE coordinate,
            /* [annotation][in] */ 
            _In_  float origin);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetZoomBoundaries)
        HRESULT ( STDMETHODCALLTYPE *SetZoomBoundaries )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  float zoomMinimum,
            /* [annotation][in] */ 
            _In_  float zoomMaximum);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetHorizontalAlignment)
        HRESULT ( STDMETHODCALLTYPE *SetHorizontalAlignment )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_HORIZONTALALIGNMENT alignment);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, SetVerticalAlignment)
        HRESULT ( STDMETHODCALLTYPE *SetVerticalAlignment )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_VERTICALALIGNMENT alignment);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, GetInertiaEndTransform)
        HRESULT ( STDMETHODCALLTYPE *GetInertiaEndTransform )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][out] */ 
            _Out_writes_(pointCount)  float *matrix,
            /* [annotation][in] */ 
            _In_  DWORD pointCount);
        
        DECLSPEC_XFGVIRT(IDirectManipulationPrimaryContent, GetCenterPoint)
        HRESULT ( STDMETHODCALLTYPE *GetCenterPoint )( 
            IDirectManipulationPrimaryContent * This,
            /* [annotation][out] */ 
            _Out_  float *centerX,
            /* [annotation][out] */ 
            _Out_  float *centerY);
        
        END_INTERFACE
    } IDirectManipulationPrimaryContentVtbl;

    interface IDirectManipulationPrimaryContent
    {
        CONST_VTBL struct IDirectManipulationPrimaryContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationPrimaryContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationPrimaryContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationPrimaryContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationPrimaryContent_SetSnapInterval(This,motion,interval,offset)	\
    ( (This)->lpVtbl -> SetSnapInterval(This,motion,interval,offset) ) 

#define IDirectManipulationPrimaryContent_SetSnapPoints(This,motion,points,pointCount)	\
    ( (This)->lpVtbl -> SetSnapPoints(This,motion,points,pointCount) ) 

#define IDirectManipulationPrimaryContent_SetSnapType(This,motion,type)	\
    ( (This)->lpVtbl -> SetSnapType(This,motion,type) ) 

#define IDirectManipulationPrimaryContent_SetSnapCoordinate(This,motion,coordinate,origin)	\
    ( (This)->lpVtbl -> SetSnapCoordinate(This,motion,coordinate,origin) ) 

#define IDirectManipulationPrimaryContent_SetZoomBoundaries(This,zoomMinimum,zoomMaximum)	\
    ( (This)->lpVtbl -> SetZoomBoundaries(This,zoomMinimum,zoomMaximum) ) 

#define IDirectManipulationPrimaryContent_SetHorizontalAlignment(This,alignment)	\
    ( (This)->lpVtbl -> SetHorizontalAlignment(This,alignment) ) 

#define IDirectManipulationPrimaryContent_SetVerticalAlignment(This,alignment)	\
    ( (This)->lpVtbl -> SetVerticalAlignment(This,alignment) ) 

#define IDirectManipulationPrimaryContent_GetInertiaEndTransform(This,matrix,pointCount)	\
    ( (This)->lpVtbl -> GetInertiaEndTransform(This,matrix,pointCount) ) 

#define IDirectManipulationPrimaryContent_GetCenterPoint(This,centerX,centerY)	\
    ( (This)->lpVtbl -> GetCenterPoint(This,centerX,centerY) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationPrimaryContent_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_directmanipulation_0000_0008 */
/* [local] */ 

typedef 
enum DIRECTMANIPULATION_DRAG_DROP_STATUS
    {
        DIRECTMANIPULATION_DRAG_DROP_READY	= 0,
        DIRECTMANIPULATION_DRAG_DROP_PRESELECT	= 1,
        DIRECTMANIPULATION_DRAG_DROP_SELECTING	= 2,
        DIRECTMANIPULATION_DRAG_DROP_DRAGGING	= 3,
        DIRECTMANIPULATION_DRAG_DROP_CANCELLED	= 4,
        DIRECTMANIPULATION_DRAG_DROP_COMMITTED	= 5
    } 	DIRECTMANIPULATION_DRAG_DROP_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0008_v0_0_s_ifspec;

#ifndef __IDirectManipulationDragDropEventHandler_INTERFACE_DEFINED__
#define __IDirectManipulationDragDropEventHandler_INTERFACE_DEFINED__

/* interface IDirectManipulationDragDropEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationDragDropEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1FA11B10-701B-41AE-B5F2-49E36BD595AA")
    IDirectManipulationDragDropEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDragDropStatusChange( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport2 *viewport,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_DRAG_DROP_STATUS current,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_DRAG_DROP_STATUS previous) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationDragDropEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationDragDropEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationDragDropEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationDragDropEventHandler * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationDragDropEventHandler, OnDragDropStatusChange)
        HRESULT ( STDMETHODCALLTYPE *OnDragDropStatusChange )( 
            IDirectManipulationDragDropEventHandler * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport2 *viewport,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_DRAG_DROP_STATUS current,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_DRAG_DROP_STATUS previous);
        
        END_INTERFACE
    } IDirectManipulationDragDropEventHandlerVtbl;

    interface IDirectManipulationDragDropEventHandler
    {
        CONST_VTBL struct IDirectManipulationDragDropEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationDragDropEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationDragDropEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationDragDropEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationDragDropEventHandler_OnDragDropStatusChange(This,viewport,current,previous)	\
    ( (This)->lpVtbl -> OnDragDropStatusChange(This,viewport,current,previous) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationDragDropEventHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_directmanipulation_0000_0009 */
/* [local] */ 

typedef 
enum DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION
    {
        DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL	= 0x1,
        DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL	= 0x2,
        DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY	= 0x10,
        DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG	= 0x20,
        DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG	= 0x40
    } 	DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION;

DEFINE_ENUM_FLAG_OPERATORS(DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION)


extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0009_v0_0_s_ifspec;

#ifndef __IDirectManipulationDragDropBehavior_INTERFACE_DEFINED__
#define __IDirectManipulationDragDropBehavior_INTERFACE_DEFINED__

/* interface IDirectManipulationDragDropBehavior */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationDragDropBehavior;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("814B5AF5-C2C8-4270-A9B7-A198CE8D02FA")
    IDirectManipulationDragDropBehavior : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetConfiguration( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][retval][out] */ 
            _Out_  DIRECTMANIPULATION_DRAG_DROP_STATUS *status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationDragDropBehaviorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationDragDropBehavior * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationDragDropBehavior * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationDragDropBehavior * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationDragDropBehavior, SetConfiguration)
        HRESULT ( STDMETHODCALLTYPE *SetConfiguration )( 
            IDirectManipulationDragDropBehavior * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION configuration);
        
        DECLSPEC_XFGVIRT(IDirectManipulationDragDropBehavior, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IDirectManipulationDragDropBehavior * This,
            /* [annotation][retval][out] */ 
            _Out_  DIRECTMANIPULATION_DRAG_DROP_STATUS *status);
        
        END_INTERFACE
    } IDirectManipulationDragDropBehaviorVtbl;

    interface IDirectManipulationDragDropBehavior
    {
        CONST_VTBL struct IDirectManipulationDragDropBehaviorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationDragDropBehavior_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationDragDropBehavior_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationDragDropBehavior_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationDragDropBehavior_SetConfiguration(This,configuration)	\
    ( (This)->lpVtbl -> SetConfiguration(This,configuration) ) 

#define IDirectManipulationDragDropBehavior_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationDragDropBehavior_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_directmanipulation_0000_0010 */
/* [local] */ 

typedef 
enum DIRECTMANIPULATION_INTERACTION_TYPE
    {
        DIRECTMANIPULATION_INTERACTION_BEGIN	= 0,
        DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION	= 1,
        DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP	= 2,
        DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD	= 3,
        DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE	= 4,
        DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM	= 5,
        DIRECTMANIPULATION_INTERACTION_END	= 100
    } 	DIRECTMANIPULATION_INTERACTION_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0010_v0_0_s_ifspec;

#ifndef __IDirectManipulationInteractionEventHandler_INTERFACE_DEFINED__
#define __IDirectManipulationInteractionEventHandler_INTERFACE_DEFINED__

/* interface IDirectManipulationInteractionEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationInteractionEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E43F45B8-42B4-403E-B1F2-273B8F510830")
    IDirectManipulationInteractionEventHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnInteraction( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport2 *viewport,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INTERACTION_TYPE interaction) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationInteractionEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationInteractionEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationInteractionEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationInteractionEventHandler * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationInteractionEventHandler, OnInteraction)
        HRESULT ( STDMETHODCALLTYPE *OnInteraction )( 
            IDirectManipulationInteractionEventHandler * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationViewport2 *viewport,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_INTERACTION_TYPE interaction);
        
        END_INTERFACE
    } IDirectManipulationInteractionEventHandlerVtbl;

    interface IDirectManipulationInteractionEventHandler
    {
        CONST_VTBL struct IDirectManipulationInteractionEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationInteractionEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationInteractionEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationInteractionEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationInteractionEventHandler_OnInteraction(This,viewport,interaction)	\
    ( (This)->lpVtbl -> OnInteraction(This,viewport,interaction) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationInteractionEventHandler_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationFrameInfoProvider_INTERFACE_DEFINED__
#define __IDirectManipulationFrameInfoProvider_INTERFACE_DEFINED__

/* interface IDirectManipulationFrameInfoProvider */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationFrameInfoProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fb759dba-6f4c-4c01-874e-19c8a05907f9")
    IDirectManipulationFrameInfoProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNextFrameInfo( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *time,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *processTime,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *compositionTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationFrameInfoProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationFrameInfoProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationFrameInfoProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationFrameInfoProvider * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationFrameInfoProvider, GetNextFrameInfo)
        HRESULT ( STDMETHODCALLTYPE *GetNextFrameInfo )( 
            IDirectManipulationFrameInfoProvider * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *time,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *processTime,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *compositionTime);
        
        END_INTERFACE
    } IDirectManipulationFrameInfoProviderVtbl;

    interface IDirectManipulationFrameInfoProvider
    {
        CONST_VTBL struct IDirectManipulationFrameInfoProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationFrameInfoProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationFrameInfoProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationFrameInfoProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationFrameInfoProvider_GetNextFrameInfo(This,time,processTime,compositionTime)	\
    ( (This)->lpVtbl -> GetNextFrameInfo(This,time,processTime,compositionTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationFrameInfoProvider_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationCompositor_INTERFACE_DEFINED__
#define __IDirectManipulationCompositor_INTERFACE_DEFINED__

/* interface IDirectManipulationCompositor */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationCompositor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("537A0825-0387-4EFA-B62F-71EB1F085A7E")
    IDirectManipulationCompositor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddContent( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *device,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *parentVisual,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *childVisual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveContent( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUpdateManager( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationUpdateManager *updateManager) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationCompositorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationCompositor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationCompositor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationCompositor * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, AddContent)
        HRESULT ( STDMETHODCALLTYPE *AddContent )( 
            IDirectManipulationCompositor * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *device,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *parentVisual,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *childVisual);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, RemoveContent)
        HRESULT ( STDMETHODCALLTYPE *RemoveContent )( 
            IDirectManipulationCompositor * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, SetUpdateManager)
        HRESULT ( STDMETHODCALLTYPE *SetUpdateManager )( 
            IDirectManipulationCompositor * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationUpdateManager *updateManager);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IDirectManipulationCompositor * This);
        
        END_INTERFACE
    } IDirectManipulationCompositorVtbl;

    interface IDirectManipulationCompositor
    {
        CONST_VTBL struct IDirectManipulationCompositorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationCompositor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationCompositor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationCompositor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationCompositor_AddContent(This,content,device,parentVisual,childVisual)	\
    ( (This)->lpVtbl -> AddContent(This,content,device,parentVisual,childVisual) ) 

#define IDirectManipulationCompositor_RemoveContent(This,content)	\
    ( (This)->lpVtbl -> RemoveContent(This,content) ) 

#define IDirectManipulationCompositor_SetUpdateManager(This,updateManager)	\
    ( (This)->lpVtbl -> SetUpdateManager(This,updateManager) ) 

#define IDirectManipulationCompositor_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationCompositor_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationCompositor2_INTERFACE_DEFINED__
#define __IDirectManipulationCompositor2_INTERFACE_DEFINED__

/* interface IDirectManipulationCompositor2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationCompositor2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D38C7822-F1CB-43CB-B4B9-AC0C767A412E")
    IDirectManipulationCompositor2 : public IDirectManipulationCompositor
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddContentWithCrossProcessChaining( 
            /* [annotation][in] */ 
            _In_  IDirectManipulationPrimaryContent *content,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *device,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *parentVisual,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *childVisual) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationCompositor2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationCompositor2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationCompositor2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationCompositor2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, AddContent)
        HRESULT ( STDMETHODCALLTYPE *AddContent )( 
            IDirectManipulationCompositor2 * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *device,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *parentVisual,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *childVisual);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, RemoveContent)
        HRESULT ( STDMETHODCALLTYPE *RemoveContent )( 
            IDirectManipulationCompositor2 * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationContent *content);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, SetUpdateManager)
        HRESULT ( STDMETHODCALLTYPE *SetUpdateManager )( 
            IDirectManipulationCompositor2 * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationUpdateManager *updateManager);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            IDirectManipulationCompositor2 * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationCompositor2, AddContentWithCrossProcessChaining)
        HRESULT ( STDMETHODCALLTYPE *AddContentWithCrossProcessChaining )( 
            IDirectManipulationCompositor2 * This,
            /* [annotation][in] */ 
            _In_  IDirectManipulationPrimaryContent *content,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *device,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *parentVisual,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *childVisual);
        
        END_INTERFACE
    } IDirectManipulationCompositor2Vtbl;

    interface IDirectManipulationCompositor2
    {
        CONST_VTBL struct IDirectManipulationCompositor2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationCompositor2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationCompositor2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationCompositor2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationCompositor2_AddContent(This,content,device,parentVisual,childVisual)	\
    ( (This)->lpVtbl -> AddContent(This,content,device,parentVisual,childVisual) ) 

#define IDirectManipulationCompositor2_RemoveContent(This,content)	\
    ( (This)->lpVtbl -> RemoveContent(This,content) ) 

#define IDirectManipulationCompositor2_SetUpdateManager(This,updateManager)	\
    ( (This)->lpVtbl -> SetUpdateManager(This,updateManager) ) 

#define IDirectManipulationCompositor2_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 


#define IDirectManipulationCompositor2_AddContentWithCrossProcessChaining(This,content,device,parentVisual,childVisual)	\
    ( (This)->lpVtbl -> AddContentWithCrossProcessChaining(This,content,device,parentVisual,childVisual) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationCompositor2_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationUpdateHandler_INTERFACE_DEFINED__
#define __IDirectManipulationUpdateHandler_INTERFACE_DEFINED__

/* interface IDirectManipulationUpdateHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationUpdateHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("790B6337-64F8-4FF5-A269-B32BC2AF27A7")
    IDirectManipulationUpdateHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Update( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationUpdateHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationUpdateHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationUpdateHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationUpdateHandler * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationUpdateHandler, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IDirectManipulationUpdateHandler * This);
        
        END_INTERFACE
    } IDirectManipulationUpdateHandlerVtbl;

    interface IDirectManipulationUpdateHandler
    {
        CONST_VTBL struct IDirectManipulationUpdateHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationUpdateHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationUpdateHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationUpdateHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationUpdateHandler_Update(This)	\
    ( (This)->lpVtbl -> Update(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationUpdateHandler_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationUpdateManager_INTERFACE_DEFINED__
#define __IDirectManipulationUpdateManager_INTERFACE_DEFINED__

/* interface IDirectManipulationUpdateManager */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationUpdateManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B0AE62FD-BE34-46E7-9CAA-D361FACBB9CC")
    IDirectManipulationUpdateManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterWaitHandleCallback( 
            /* [annotation][in] */ 
            _In_  HANDLE handle,
            /* [annotation][in] */ 
            _In_  IDirectManipulationUpdateHandler *eventHandler,
            /* [annotation][out] */ 
            _Out_  DWORD *cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterWaitHandleCallback( 
            /* [annotation][in] */ 
            _In_  DWORD cookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationUpdateManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationUpdateManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationUpdateManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationUpdateManager * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationUpdateManager, RegisterWaitHandleCallback)
        HRESULT ( STDMETHODCALLTYPE *RegisterWaitHandleCallback )( 
            IDirectManipulationUpdateManager * This,
            /* [annotation][in] */ 
            _In_  HANDLE handle,
            /* [annotation][in] */ 
            _In_  IDirectManipulationUpdateHandler *eventHandler,
            /* [annotation][out] */ 
            _Out_  DWORD *cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationUpdateManager, UnregisterWaitHandleCallback)
        HRESULT ( STDMETHODCALLTYPE *UnregisterWaitHandleCallback )( 
            IDirectManipulationUpdateManager * This,
            /* [annotation][in] */ 
            _In_  DWORD cookie);
        
        DECLSPEC_XFGVIRT(IDirectManipulationUpdateManager, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IDirectManipulationUpdateManager * This,
            /* [annotation][in] */ 
            _In_opt_  IDirectManipulationFrameInfoProvider *frameInfo);
        
        END_INTERFACE
    } IDirectManipulationUpdateManagerVtbl;

    interface IDirectManipulationUpdateManager
    {
        CONST_VTBL struct IDirectManipulationUpdateManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationUpdateManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationUpdateManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationUpdateManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationUpdateManager_RegisterWaitHandleCallback(This,handle,eventHandler,cookie)	\
    ( (This)->lpVtbl -> RegisterWaitHandleCallback(This,handle,eventHandler,cookie) ) 

#define IDirectManipulationUpdateManager_UnregisterWaitHandleCallback(This,cookie)	\
    ( (This)->lpVtbl -> UnregisterWaitHandleCallback(This,cookie) ) 

#define IDirectManipulationUpdateManager_Update(This,frameInfo)	\
    ( (This)->lpVtbl -> Update(This,frameInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationUpdateManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_directmanipulation_0000_0016 */
/* [local] */ 

typedef 
enum DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION
    {
        DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP	= 0,
        DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD	= 1,
        DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE	= 2
    } 	DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION;



extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0016_v0_0_s_ifspec;

#ifndef __IDirectManipulationAutoScrollBehavior_INTERFACE_DEFINED__
#define __IDirectManipulationAutoScrollBehavior_INTERFACE_DEFINED__

/* interface IDirectManipulationAutoScrollBehavior */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationAutoScrollBehavior;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D5954D4-2003-4356-9B31-D051C9FF0AF7")
    IDirectManipulationAutoScrollBehavior : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetConfiguration( 
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motionTypes,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION scrollMotion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationAutoScrollBehaviorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationAutoScrollBehavior * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationAutoScrollBehavior * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationAutoScrollBehavior * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationAutoScrollBehavior, SetConfiguration)
        HRESULT ( STDMETHODCALLTYPE *SetConfiguration )( 
            IDirectManipulationAutoScrollBehavior * This,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_MOTION_TYPES motionTypes,
            /* [annotation][in] */ 
            _In_  DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION scrollMotion);
        
        END_INTERFACE
    } IDirectManipulationAutoScrollBehaviorVtbl;

    interface IDirectManipulationAutoScrollBehavior
    {
        CONST_VTBL struct IDirectManipulationAutoScrollBehaviorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationAutoScrollBehavior_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationAutoScrollBehavior_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationAutoScrollBehavior_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationAutoScrollBehavior_SetConfiguration(This,motionTypes,scrollMotion)	\
    ( (This)->lpVtbl -> SetConfiguration(This,motionTypes,scrollMotion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationAutoScrollBehavior_INTERFACE_DEFINED__ */


#ifndef __IDirectManipulationDeferContactService_INTERFACE_DEFINED__
#define __IDirectManipulationDeferContactService_INTERFACE_DEFINED__

/* interface IDirectManipulationDeferContactService */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDirectManipulationDeferContactService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("652D5C71-FE60-4A98-BE70-E5F21291E7F1")
    IDirectManipulationDeferContactService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DeferContact( 
            /* [annotation][in] */ 
            _In_  UINT32 pointerId,
            /* [annotation][in] */ 
            _In_  UINT32 timeout) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelContact( 
            /* [annotation][in] */ 
            _In_  UINT32 pointerId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelDeferral( 
            /* [annotation][in] */ 
            _In_  UINT32 pointerId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDirectManipulationDeferContactServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDirectManipulationDeferContactService * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDirectManipulationDeferContactService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDirectManipulationDeferContactService * This);
        
        DECLSPEC_XFGVIRT(IDirectManipulationDeferContactService, DeferContact)
        HRESULT ( STDMETHODCALLTYPE *DeferContact )( 
            IDirectManipulationDeferContactService * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId,
            /* [annotation][in] */ 
            _In_  UINT32 timeout);
        
        DECLSPEC_XFGVIRT(IDirectManipulationDeferContactService, CancelContact)
        HRESULT ( STDMETHODCALLTYPE *CancelContact )( 
            IDirectManipulationDeferContactService * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId);
        
        DECLSPEC_XFGVIRT(IDirectManipulationDeferContactService, CancelDeferral)
        HRESULT ( STDMETHODCALLTYPE *CancelDeferral )( 
            IDirectManipulationDeferContactService * This,
            /* [annotation][in] */ 
            _In_  UINT32 pointerId);
        
        END_INTERFACE
    } IDirectManipulationDeferContactServiceVtbl;

    interface IDirectManipulationDeferContactService
    {
        CONST_VTBL struct IDirectManipulationDeferContactServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDirectManipulationDeferContactService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDirectManipulationDeferContactService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDirectManipulationDeferContactService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDirectManipulationDeferContactService_DeferContact(This,pointerId,timeout)	\
    ( (This)->lpVtbl -> DeferContact(This,pointerId,timeout) ) 

#define IDirectManipulationDeferContactService_CancelContact(This,pointerId)	\
    ( (This)->lpVtbl -> CancelContact(This,pointerId) ) 

#define IDirectManipulationDeferContactService_CancelDeferral(This,pointerId)	\
    ( (This)->lpVtbl -> CancelDeferral(This,pointerId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDirectManipulationDeferContactService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_directmanipulation_0000_0018 */
/* [local] */ 

#endif // #if (NTDDI_VERSION > NTDDI_WIN8)


extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0029_v0_0_s_ifspec;


#ifndef __DirectManipulation_LIBRARY_DEFINED__
#define __DirectManipulation_LIBRARY_DEFINED__

/* library DirectManipulation */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_DirectManipulation;

EXTERN_C const CLSID CLSID_DirectManipulationViewport;

#ifdef __cplusplus

class DECLSPEC_UUID("34E211B6-3650-4F75-8334-FA359598E1C5")
DirectManipulationViewport;
#endif

EXTERN_C const CLSID CLSID_DirectManipulationUpdateManager;

#ifdef __cplusplus

class DECLSPEC_UUID("9FC1BFD5-1835-441A-B3B1-B6CC74B727D0")
DirectManipulationUpdateManager;
#endif

EXTERN_C const CLSID CLSID_DirectManipulationPrimaryContent;

#ifdef __cplusplus

class DECLSPEC_UUID("CAA02661-D59E-41C7-8393-3BA3BACB6B57")
DirectManipulationPrimaryContent;
#endif

EXTERN_C const CLSID CLSID_DirectManipulationManager;

#ifdef __cplusplus

class DECLSPEC_UUID("54E211B6-3650-4F75-8334-FA359598E1C5")
DirectManipulationManager;
#endif

EXTERN_C const CLSID CLSID_DirectManipulationSharedManager;

#ifdef __cplusplus

class DECLSPEC_UUID("99793286-77CC-4B57-96DB-3B354F6F9FB5")
DirectManipulationSharedManager;
#endif

EXTERN_C const CLSID CLSID_DCompManipulationCompositor;

#ifdef __cplusplus

class DECLSPEC_UUID("79DEA627-A08A-43AC-8EF5-6900B9299126")
DCompManipulationCompositor;
#endif
#endif /* __DirectManipulation_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_directmanipulation_0000_0030 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_directmanipulation_0000_0030_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif



