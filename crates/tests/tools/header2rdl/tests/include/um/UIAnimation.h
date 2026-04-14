

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

#ifndef __UIAnimation_h__
#define __UIAnimation_h__

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

#ifndef __IUIAnimationManager_FWD_DEFINED__
#define __IUIAnimationManager_FWD_DEFINED__
typedef interface IUIAnimationManager IUIAnimationManager;

#endif 	/* __IUIAnimationManager_FWD_DEFINED__ */


#ifndef __IUIAnimationVariable_FWD_DEFINED__
#define __IUIAnimationVariable_FWD_DEFINED__
typedef interface IUIAnimationVariable IUIAnimationVariable;

#endif 	/* __IUIAnimationVariable_FWD_DEFINED__ */


#ifndef __IUIAnimationStoryboard_FWD_DEFINED__
#define __IUIAnimationStoryboard_FWD_DEFINED__
typedef interface IUIAnimationStoryboard IUIAnimationStoryboard;

#endif 	/* __IUIAnimationStoryboard_FWD_DEFINED__ */


#ifndef __IUIAnimationTransition_FWD_DEFINED__
#define __IUIAnimationTransition_FWD_DEFINED__
typedef interface IUIAnimationTransition IUIAnimationTransition;

#endif 	/* __IUIAnimationTransition_FWD_DEFINED__ */


#ifndef __IUIAnimationManagerEventHandler_FWD_DEFINED__
#define __IUIAnimationManagerEventHandler_FWD_DEFINED__
typedef interface IUIAnimationManagerEventHandler IUIAnimationManagerEventHandler;

#endif 	/* __IUIAnimationManagerEventHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationVariableChangeHandler_FWD_DEFINED__
#define __IUIAnimationVariableChangeHandler_FWD_DEFINED__
typedef interface IUIAnimationVariableChangeHandler IUIAnimationVariableChangeHandler;

#endif 	/* __IUIAnimationVariableChangeHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationVariableIntegerChangeHandler_FWD_DEFINED__
#define __IUIAnimationVariableIntegerChangeHandler_FWD_DEFINED__
typedef interface IUIAnimationVariableIntegerChangeHandler IUIAnimationVariableIntegerChangeHandler;

#endif 	/* __IUIAnimationVariableIntegerChangeHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationStoryboardEventHandler_FWD_DEFINED__
#define __IUIAnimationStoryboardEventHandler_FWD_DEFINED__
typedef interface IUIAnimationStoryboardEventHandler IUIAnimationStoryboardEventHandler;

#endif 	/* __IUIAnimationStoryboardEventHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationPriorityComparison_FWD_DEFINED__
#define __IUIAnimationPriorityComparison_FWD_DEFINED__
typedef interface IUIAnimationPriorityComparison IUIAnimationPriorityComparison;

#endif 	/* __IUIAnimationPriorityComparison_FWD_DEFINED__ */


#ifndef __IUIAnimationTransitionLibrary_FWD_DEFINED__
#define __IUIAnimationTransitionLibrary_FWD_DEFINED__
typedef interface IUIAnimationTransitionLibrary IUIAnimationTransitionLibrary;

#endif 	/* __IUIAnimationTransitionLibrary_FWD_DEFINED__ */


#ifndef __IUIAnimationInterpolator_FWD_DEFINED__
#define __IUIAnimationInterpolator_FWD_DEFINED__
typedef interface IUIAnimationInterpolator IUIAnimationInterpolator;

#endif 	/* __IUIAnimationInterpolator_FWD_DEFINED__ */


#ifndef __IUIAnimationTransitionFactory_FWD_DEFINED__
#define __IUIAnimationTransitionFactory_FWD_DEFINED__
typedef interface IUIAnimationTransitionFactory IUIAnimationTransitionFactory;

#endif 	/* __IUIAnimationTransitionFactory_FWD_DEFINED__ */


#ifndef __IUIAnimationTimer_FWD_DEFINED__
#define __IUIAnimationTimer_FWD_DEFINED__
typedef interface IUIAnimationTimer IUIAnimationTimer;

#endif 	/* __IUIAnimationTimer_FWD_DEFINED__ */


#ifndef __IUIAnimationTimerUpdateHandler_FWD_DEFINED__
#define __IUIAnimationTimerUpdateHandler_FWD_DEFINED__
typedef interface IUIAnimationTimerUpdateHandler IUIAnimationTimerUpdateHandler;

#endif 	/* __IUIAnimationTimerUpdateHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationTimerClientEventHandler_FWD_DEFINED__
#define __IUIAnimationTimerClientEventHandler_FWD_DEFINED__
typedef interface IUIAnimationTimerClientEventHandler IUIAnimationTimerClientEventHandler;

#endif 	/* __IUIAnimationTimerClientEventHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationTimerEventHandler_FWD_DEFINED__
#define __IUIAnimationTimerEventHandler_FWD_DEFINED__
typedef interface IUIAnimationTimerEventHandler IUIAnimationTimerEventHandler;

#endif 	/* __IUIAnimationTimerEventHandler_FWD_DEFINED__ */


#ifndef __IUIAnimationManager2_FWD_DEFINED__
#define __IUIAnimationManager2_FWD_DEFINED__
typedef interface IUIAnimationManager2 IUIAnimationManager2;

#endif 	/* __IUIAnimationManager2_FWD_DEFINED__ */


#ifndef __IUIAnimationVariable2_FWD_DEFINED__
#define __IUIAnimationVariable2_FWD_DEFINED__
typedef interface IUIAnimationVariable2 IUIAnimationVariable2;

#endif 	/* __IUIAnimationVariable2_FWD_DEFINED__ */


#ifndef __IUIAnimationTransition2_FWD_DEFINED__
#define __IUIAnimationTransition2_FWD_DEFINED__
typedef interface IUIAnimationTransition2 IUIAnimationTransition2;

#endif 	/* __IUIAnimationTransition2_FWD_DEFINED__ */


#ifndef __IUIAnimationManagerEventHandler2_FWD_DEFINED__
#define __IUIAnimationManagerEventHandler2_FWD_DEFINED__
typedef interface IUIAnimationManagerEventHandler2 IUIAnimationManagerEventHandler2;

#endif 	/* __IUIAnimationManagerEventHandler2_FWD_DEFINED__ */


#ifndef __IUIAnimationVariableChangeHandler2_FWD_DEFINED__
#define __IUIAnimationVariableChangeHandler2_FWD_DEFINED__
typedef interface IUIAnimationVariableChangeHandler2 IUIAnimationVariableChangeHandler2;

#endif 	/* __IUIAnimationVariableChangeHandler2_FWD_DEFINED__ */


#ifndef __IUIAnimationVariableIntegerChangeHandler2_FWD_DEFINED__
#define __IUIAnimationVariableIntegerChangeHandler2_FWD_DEFINED__
typedef interface IUIAnimationVariableIntegerChangeHandler2 IUIAnimationVariableIntegerChangeHandler2;

#endif 	/* __IUIAnimationVariableIntegerChangeHandler2_FWD_DEFINED__ */


#ifndef __IUIAnimationVariableCurveChangeHandler2_FWD_DEFINED__
#define __IUIAnimationVariableCurveChangeHandler2_FWD_DEFINED__
typedef interface IUIAnimationVariableCurveChangeHandler2 IUIAnimationVariableCurveChangeHandler2;

#endif 	/* __IUIAnimationVariableCurveChangeHandler2_FWD_DEFINED__ */


#ifndef __IUIAnimationStoryboardEventHandler2_FWD_DEFINED__
#define __IUIAnimationStoryboardEventHandler2_FWD_DEFINED__
typedef interface IUIAnimationStoryboardEventHandler2 IUIAnimationStoryboardEventHandler2;

#endif 	/* __IUIAnimationStoryboardEventHandler2_FWD_DEFINED__ */


#ifndef __IUIAnimationLoopIterationChangeHandler2_FWD_DEFINED__
#define __IUIAnimationLoopIterationChangeHandler2_FWD_DEFINED__
typedef interface IUIAnimationLoopIterationChangeHandler2 IUIAnimationLoopIterationChangeHandler2;

#endif 	/* __IUIAnimationLoopIterationChangeHandler2_FWD_DEFINED__ */


#ifndef __IUIAnimationPriorityComparison2_FWD_DEFINED__
#define __IUIAnimationPriorityComparison2_FWD_DEFINED__
typedef interface IUIAnimationPriorityComparison2 IUIAnimationPriorityComparison2;

#endif 	/* __IUIAnimationPriorityComparison2_FWD_DEFINED__ */


#ifndef __IUIAnimationTransitionLibrary2_FWD_DEFINED__
#define __IUIAnimationTransitionLibrary2_FWD_DEFINED__
typedef interface IUIAnimationTransitionLibrary2 IUIAnimationTransitionLibrary2;

#endif 	/* __IUIAnimationTransitionLibrary2_FWD_DEFINED__ */


#ifndef __IUIAnimationPrimitiveInterpolation_FWD_DEFINED__
#define __IUIAnimationPrimitiveInterpolation_FWD_DEFINED__
typedef interface IUIAnimationPrimitiveInterpolation IUIAnimationPrimitiveInterpolation;

#endif 	/* __IUIAnimationPrimitiveInterpolation_FWD_DEFINED__ */


#ifndef __IUIAnimationInterpolator2_FWD_DEFINED__
#define __IUIAnimationInterpolator2_FWD_DEFINED__
typedef interface IUIAnimationInterpolator2 IUIAnimationInterpolator2;

#endif 	/* __IUIAnimationInterpolator2_FWD_DEFINED__ */


#ifndef __IUIAnimationTransitionFactory2_FWD_DEFINED__
#define __IUIAnimationTransitionFactory2_FWD_DEFINED__
typedef interface IUIAnimationTransitionFactory2 IUIAnimationTransitionFactory2;

#endif 	/* __IUIAnimationTransitionFactory2_FWD_DEFINED__ */


#ifndef __IUIAnimationStoryboard2_FWD_DEFINED__
#define __IUIAnimationStoryboard2_FWD_DEFINED__
typedef interface IUIAnimationStoryboard2 IUIAnimationStoryboard2;

#endif 	/* __IUIAnimationStoryboard2_FWD_DEFINED__ */


#ifndef __UIAnimationManager_FWD_DEFINED__
#define __UIAnimationManager_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationManager UIAnimationManager;
#else
typedef struct UIAnimationManager UIAnimationManager;
#endif /* __cplusplus */

#endif 	/* __UIAnimationManager_FWD_DEFINED__ */


#ifndef __UIAnimationManager2_FWD_DEFINED__
#define __UIAnimationManager2_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationManager2 UIAnimationManager2;
#else
typedef struct UIAnimationManager2 UIAnimationManager2;
#endif /* __cplusplus */

#endif 	/* __UIAnimationManager2_FWD_DEFINED__ */


#ifndef __UIAnimationTransitionLibrary_FWD_DEFINED__
#define __UIAnimationTransitionLibrary_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationTransitionLibrary UIAnimationTransitionLibrary;
#else
typedef struct UIAnimationTransitionLibrary UIAnimationTransitionLibrary;
#endif /* __cplusplus */

#endif 	/* __UIAnimationTransitionLibrary_FWD_DEFINED__ */


#ifndef __UIAnimationTransitionLibrary2_FWD_DEFINED__
#define __UIAnimationTransitionLibrary2_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationTransitionLibrary2 UIAnimationTransitionLibrary2;
#else
typedef struct UIAnimationTransitionLibrary2 UIAnimationTransitionLibrary2;
#endif /* __cplusplus */

#endif 	/* __UIAnimationTransitionLibrary2_FWD_DEFINED__ */


#ifndef __UIAnimationTransitionFactory_FWD_DEFINED__
#define __UIAnimationTransitionFactory_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationTransitionFactory UIAnimationTransitionFactory;
#else
typedef struct UIAnimationTransitionFactory UIAnimationTransitionFactory;
#endif /* __cplusplus */

#endif 	/* __UIAnimationTransitionFactory_FWD_DEFINED__ */


#ifndef __UIAnimationTransitionFactory2_FWD_DEFINED__
#define __UIAnimationTransitionFactory2_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationTransitionFactory2 UIAnimationTransitionFactory2;
#else
typedef struct UIAnimationTransitionFactory2 UIAnimationTransitionFactory2;
#endif /* __cplusplus */

#endif 	/* __UIAnimationTransitionFactory2_FWD_DEFINED__ */


#ifndef __UIAnimationTimer_FWD_DEFINED__
#define __UIAnimationTimer_FWD_DEFINED__

#ifdef __cplusplus
typedef class UIAnimationTimer UIAnimationTimer;
#else
typedef struct UIAnimationTimer UIAnimationTimer;
#endif /* __cplusplus */

#endif 	/* __UIAnimationTimer_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"
#include "dcompanimation.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_UIAnimation_0000_0000 */
/* [local] */ 

//--------------------------------------------------------------------------
//
//  UIAnimation.h
//
//  Windows Animation interface definitions and related types and enums
//  (Generated from UIAnimation.idl)
//
//  Copyright (c) Microsoft Corporation. All rights reserved.
//
//--------------------------------------------------------------------------
#pragma warning(push)
#pragma warning(disable:4668) 
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)





























typedef DOUBLE UI_ANIMATION_SECONDS;

#define	UI_ANIMATION_SECONDS_EVENTUALLY	( -1 )

typedef /* [public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0000_0001
    {
        UI_ANIMATION_UPDATE_NO_CHANGE	= 0,
        UI_ANIMATION_UPDATE_VARIABLES_CHANGED	= 1
    } 	UI_ANIMATION_UPDATE_RESULT;

typedef /* [public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0000_0002
    {
        UI_ANIMATION_MANAGER_IDLE	= 0,
        UI_ANIMATION_MANAGER_BUSY	= 1
    } 	UI_ANIMATION_MANAGER_STATUS;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0000_0003
    {
        UI_ANIMATION_MODE_DISABLED	= 0,
        UI_ANIMATION_MODE_SYSTEM_DEFAULT	= 1,
        UI_ANIMATION_MODE_ENABLED	= 2
    } 	UI_ANIMATION_MODE;

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0000_0004
    {
        UI_ANIMATION_REPEAT_MODE_NORMAL	= 0,
        UI_ANIMATION_REPEAT_MODE_ALTERNATE	= 1
    } 	UI_ANIMATION_REPEAT_MODE;



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0000_v0_0_s_ifspec;

#ifndef __IUIAnimationManager_INTERFACE_DEFINED__
#define __IUIAnimationManager_INTERFACE_DEFINED__

/* interface IUIAnimationManager */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9169896C-AC8D-4e7d-94E5-67FA4DC2F2E8")
    IUIAnimationManager : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateAnimationVariable( 
            /* [annotation][in] */ 
            _In_  DOUBLE initialValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable **variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE ScheduleTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateStoryboard( 
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard **storyboard) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE FinishAllStoryboards( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AbandonAllStoryboards( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Update( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_UPDATE_RESULT *updateResult = 0) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetVariableFromTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable **variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetStoryboardFromTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard **storyboard) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_MANAGER_STATUS *status) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetAnimationMode( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MODE mode) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetManagerEventHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationManagerEventHandler *handler) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetCancelPriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTrimPriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetCompressPriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetConcludePriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetDefaultLongestAcceptableDelay( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationManager * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, CreateAnimationVariable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateAnimationVariable )( 
            IUIAnimationManager * This,
            /* [annotation][in] */ 
            _In_  DOUBLE initialValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable **variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, ScheduleTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *ScheduleTransition )( 
            IUIAnimationManager * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, CreateStoryboard)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateStoryboard )( 
            IUIAnimationManager * This,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard **storyboard);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, FinishAllStoryboards)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *FinishAllStoryboards )( 
            IUIAnimationManager * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, AbandonAllStoryboards)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AbandonAllStoryboards )( 
            IUIAnimationManager * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, Update)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUIAnimationManager * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_UPDATE_RESULT *updateResult);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, GetVariableFromTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetVariableFromTag )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable **variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, GetStoryboardFromTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetStoryboardFromTag )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard **storyboard);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, GetStatus)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IUIAnimationManager * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_MANAGER_STATUS *status);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetAnimationMode)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetAnimationMode )( 
            IUIAnimationManager * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MODE mode);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, Pause)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IUIAnimationManager * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, Resume)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IUIAnimationManager * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetManagerEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetManagerEventHandler )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationManagerEventHandler *handler);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetCancelPriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetCancelPriorityComparison )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetTrimPriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTrimPriorityComparison )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetCompressPriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetCompressPriorityComparison )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetConcludePriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetConcludePriorityComparison )( 
            IUIAnimationManager * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, SetDefaultLongestAcceptableDelay)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetDefaultLongestAcceptableDelay )( 
            IUIAnimationManager * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager, Shutdown)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IUIAnimationManager * This);
        
        END_INTERFACE
    } IUIAnimationManagerVtbl;

    interface IUIAnimationManager
    {
        CONST_VTBL struct IUIAnimationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationManager_CreateAnimationVariable(This,initialValue,variable)	\
    ( (This)->lpVtbl -> CreateAnimationVariable(This,initialValue,variable) ) 

#define IUIAnimationManager_ScheduleTransition(This,variable,transition,timeNow)	\
    ( (This)->lpVtbl -> ScheduleTransition(This,variable,transition,timeNow) ) 

#define IUIAnimationManager_CreateStoryboard(This,storyboard)	\
    ( (This)->lpVtbl -> CreateStoryboard(This,storyboard) ) 

#define IUIAnimationManager_FinishAllStoryboards(This,completionDeadline)	\
    ( (This)->lpVtbl -> FinishAllStoryboards(This,completionDeadline) ) 

#define IUIAnimationManager_AbandonAllStoryboards(This)	\
    ( (This)->lpVtbl -> AbandonAllStoryboards(This) ) 

#define IUIAnimationManager_Update(This,timeNow,updateResult)	\
    ( (This)->lpVtbl -> Update(This,timeNow,updateResult) ) 

#define IUIAnimationManager_GetVariableFromTag(This,object,id,variable)	\
    ( (This)->lpVtbl -> GetVariableFromTag(This,object,id,variable) ) 

#define IUIAnimationManager_GetStoryboardFromTag(This,object,id,storyboard)	\
    ( (This)->lpVtbl -> GetStoryboardFromTag(This,object,id,storyboard) ) 

#define IUIAnimationManager_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#define IUIAnimationManager_SetAnimationMode(This,mode)	\
    ( (This)->lpVtbl -> SetAnimationMode(This,mode) ) 

#define IUIAnimationManager_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IUIAnimationManager_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IUIAnimationManager_SetManagerEventHandler(This,handler)	\
    ( (This)->lpVtbl -> SetManagerEventHandler(This,handler) ) 

#define IUIAnimationManager_SetCancelPriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetCancelPriorityComparison(This,comparison) ) 

#define IUIAnimationManager_SetTrimPriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetTrimPriorityComparison(This,comparison) ) 

#define IUIAnimationManager_SetCompressPriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetCompressPriorityComparison(This,comparison) ) 

#define IUIAnimationManager_SetConcludePriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetConcludePriorityComparison(This,comparison) ) 

#define IUIAnimationManager_SetDefaultLongestAcceptableDelay(This,delay)	\
    ( (This)->lpVtbl -> SetDefaultLongestAcceptableDelay(This,delay) ) 

#define IUIAnimationManager_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0001 */
/* [local] */ 

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0001_0001
    {
        UI_ANIMATION_ROUNDING_NEAREST	= 0,
        UI_ANIMATION_ROUNDING_FLOOR	= 1,
        UI_ANIMATION_ROUNDING_CEILING	= 2
    } 	UI_ANIMATION_ROUNDING_MODE;



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0001_v0_0_s_ifspec;

#ifndef __IUIAnimationVariable_INTERFACE_DEFINED__
#define __IUIAnimationVariable_INTERFACE_DEFINED__

/* interface IUIAnimationVariable */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariable;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")
    IUIAnimationVariable : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *finalValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPreviousValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *previousValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetIntegerValue( 
            /* [annotation][retval][out] */ 
            _Out_  INT32 *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalIntegerValue( 
            /* [annotation][retval][out] */ 
            _Out_  INT32 *finalValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPreviousIntegerValue( 
            /* [annotation][retval][out] */ 
            _Out_  INT32 *previousValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetCurrentStoryboard( 
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard **storyboard) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetLowerBound( 
            /* [annotation][in] */ 
            _In_  DOUBLE bound) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetUpperBound( 
            /* [annotation][in] */ 
            _In_  DOUBLE bound) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetRoundingMode( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_ROUNDING_MODE mode) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetTag( 
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetVariableChangeHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableChangeHandler *handler) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetVariableIntegerChangeHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableIntegerChangeHandler *handler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariableVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariable * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariable * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariable * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetFinalValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalValue )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *finalValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetPreviousValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPreviousValue )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *previousValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetIntegerValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetIntegerValue )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Out_  INT32 *value);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetFinalIntegerValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalIntegerValue )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Out_  INT32 *finalValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetPreviousIntegerValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPreviousIntegerValue )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Out_  INT32 *previousValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetCurrentStoryboard)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetCurrentStoryboard )( 
            IUIAnimationVariable * This,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard **storyboard);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, SetLowerBound)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetLowerBound )( 
            IUIAnimationVariable * This,
            /* [annotation][in] */ 
            _In_  DOUBLE bound);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, SetUpperBound)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetUpperBound )( 
            IUIAnimationVariable * This,
            /* [annotation][in] */ 
            _In_  DOUBLE bound);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, SetRoundingMode)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetRoundingMode )( 
            IUIAnimationVariable * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_ROUNDING_MODE mode);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, SetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IUIAnimationVariable * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, GetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IUIAnimationVariable * This,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, SetVariableChangeHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetVariableChangeHandler )( 
            IUIAnimationVariable * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableChangeHandler *handler);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable, SetVariableIntegerChangeHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetVariableIntegerChangeHandler )( 
            IUIAnimationVariable * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableIntegerChangeHandler *handler);
        
        END_INTERFACE
    } IUIAnimationVariableVtbl;

    interface IUIAnimationVariable
    {
        CONST_VTBL struct IUIAnimationVariableVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariable_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariable_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariable_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariable_GetValue(This,value)	\
    ( (This)->lpVtbl -> GetValue(This,value) ) 

#define IUIAnimationVariable_GetFinalValue(This,finalValue)	\
    ( (This)->lpVtbl -> GetFinalValue(This,finalValue) ) 

#define IUIAnimationVariable_GetPreviousValue(This,previousValue)	\
    ( (This)->lpVtbl -> GetPreviousValue(This,previousValue) ) 

#define IUIAnimationVariable_GetIntegerValue(This,value)	\
    ( (This)->lpVtbl -> GetIntegerValue(This,value) ) 

#define IUIAnimationVariable_GetFinalIntegerValue(This,finalValue)	\
    ( (This)->lpVtbl -> GetFinalIntegerValue(This,finalValue) ) 

#define IUIAnimationVariable_GetPreviousIntegerValue(This,previousValue)	\
    ( (This)->lpVtbl -> GetPreviousIntegerValue(This,previousValue) ) 

#define IUIAnimationVariable_GetCurrentStoryboard(This,storyboard)	\
    ( (This)->lpVtbl -> GetCurrentStoryboard(This,storyboard) ) 

#define IUIAnimationVariable_SetLowerBound(This,bound)	\
    ( (This)->lpVtbl -> SetLowerBound(This,bound) ) 

#define IUIAnimationVariable_SetUpperBound(This,bound)	\
    ( (This)->lpVtbl -> SetUpperBound(This,bound) ) 

#define IUIAnimationVariable_SetRoundingMode(This,mode)	\
    ( (This)->lpVtbl -> SetRoundingMode(This,mode) ) 

#define IUIAnimationVariable_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IUIAnimationVariable_GetTag(This,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,object,id) ) 

#define IUIAnimationVariable_SetVariableChangeHandler(This,handler)	\
    ( (This)->lpVtbl -> SetVariableChangeHandler(This,handler) ) 

#define IUIAnimationVariable_SetVariableIntegerChangeHandler(This,handler)	\
    ( (This)->lpVtbl -> SetVariableIntegerChangeHandler(This,handler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariable_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0002 */
/* [local] */ 

typedef /* [public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0002_0001
    {
        UI_ANIMATION_STORYBOARD_BUILDING	= 0,
        UI_ANIMATION_STORYBOARD_SCHEDULED	= 1,
        UI_ANIMATION_STORYBOARD_CANCELLED	= 2,
        UI_ANIMATION_STORYBOARD_PLAYING	= 3,
        UI_ANIMATION_STORYBOARD_TRUNCATED	= 4,
        UI_ANIMATION_STORYBOARD_FINISHED	= 5,
        UI_ANIMATION_STORYBOARD_READY	= 6,
        UI_ANIMATION_STORYBOARD_INSUFFICIENT_PRIORITY	= 7
    } 	UI_ANIMATION_STORYBOARD_STATUS;

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0002_0002
    {
        UI_ANIMATION_SCHEDULING_UNEXPECTED_FAILURE	= 0,
        UI_ANIMATION_SCHEDULING_INSUFFICIENT_PRIORITY	= 1,
        UI_ANIMATION_SCHEDULING_ALREADY_SCHEDULED	= 2,
        UI_ANIMATION_SCHEDULING_SUCCEEDED	= 3,
        UI_ANIMATION_SCHEDULING_DEFERRED	= 4
    } 	UI_ANIMATION_SCHEDULING_RESULT;

typedef struct __MIDL___MIDL_itf_UIAnimation_0000_0002_0003
    {
    int _;
    } 	*UI_ANIMATION_KEYFRAME;

#define	UI_ANIMATION_KEYFRAME_STORYBOARD_START	( ( UI_ANIMATION_KEYFRAME  )-1 )

#define	UI_ANIMATION_REPEAT_INDEFINITELY	( -1 )

#define	UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_END	( UI_ANIMATION_REPEAT_INDEFINITELY )

#define	UI_ANIMATION_REPEAT_INDEFINITELY_CONCLUDE_AT_START	( -2 )



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0002_v0_0_s_ifspec;

#ifndef __IUIAnimationStoryboard_INTERFACE_DEFINED__
#define __IUIAnimationStoryboard_INTERFACE_DEFINED__

/* interface IUIAnimationStoryboard */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationStoryboard;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A8FF128F-9BF9-4af1-9E67-E5E410DEFB84")
    IUIAnimationStoryboard : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddKeyframeAtOffset( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME existingKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddKeyframeAfterTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddTransitionAtKeyframe( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddTransitionBetweenKeyframes( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE RepeatBetweenKeyframes( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe,
            /* [annotation][in] */ 
            _In_  INT32 repetitionCount) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE HoldVariable( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetLongestAcceptableDelay( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Schedule( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_SCHEDULING_RESULT *schedulingResult = 0) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Conclude( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Finish( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Abandon( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetTag( 
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_STORYBOARD_STATUS *status) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetElapsedTime( 
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_SECONDS *elapsedTime) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetStoryboardEventHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationStoryboardEventHandler *handler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationStoryboardVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationStoryboard * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationStoryboard * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationStoryboard * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, AddTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddTransition )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, AddKeyframeAtOffset)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddKeyframeAtOffset )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME existingKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, AddKeyframeAfterTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddKeyframeAfterTransition )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, AddTransitionAtKeyframe)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddTransitionAtKeyframe )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, AddTransitionBetweenKeyframes)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddTransitionBetweenKeyframes )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, RepeatBetweenKeyframes)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *RepeatBetweenKeyframes )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe,
            /* [annotation][in] */ 
            _In_  INT32 repetitionCount);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, HoldVariable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *HoldVariable )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, SetLongestAcceptableDelay)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetLongestAcceptableDelay )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, Schedule)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Schedule )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_SCHEDULING_RESULT *schedulingResult);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, Conclude)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Conclude )( 
            IUIAnimationStoryboard * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, Finish)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Finish )( 
            IUIAnimationStoryboard * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, Abandon)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Abandon )( 
            IUIAnimationStoryboard * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, SetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IUIAnimationStoryboard * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, GetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IUIAnimationStoryboard * This,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, GetStatus)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IUIAnimationStoryboard * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_STORYBOARD_STATUS *status);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, GetElapsedTime)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetElapsedTime )( 
            IUIAnimationStoryboard * This,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_SECONDS *elapsedTime);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard, SetStoryboardEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetStoryboardEventHandler )( 
            IUIAnimationStoryboard * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationStoryboardEventHandler *handler);
        
        END_INTERFACE
    } IUIAnimationStoryboardVtbl;

    interface IUIAnimationStoryboard
    {
        CONST_VTBL struct IUIAnimationStoryboardVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationStoryboard_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationStoryboard_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationStoryboard_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationStoryboard_AddTransition(This,variable,transition)	\
    ( (This)->lpVtbl -> AddTransition(This,variable,transition) ) 

#define IUIAnimationStoryboard_AddKeyframeAtOffset(This,existingKeyframe,offset,keyframe)	\
    ( (This)->lpVtbl -> AddKeyframeAtOffset(This,existingKeyframe,offset,keyframe) ) 

#define IUIAnimationStoryboard_AddKeyframeAfterTransition(This,transition,keyframe)	\
    ( (This)->lpVtbl -> AddKeyframeAfterTransition(This,transition,keyframe) ) 

#define IUIAnimationStoryboard_AddTransitionAtKeyframe(This,variable,transition,startKeyframe)	\
    ( (This)->lpVtbl -> AddTransitionAtKeyframe(This,variable,transition,startKeyframe) ) 

#define IUIAnimationStoryboard_AddTransitionBetweenKeyframes(This,variable,transition,startKeyframe,endKeyframe)	\
    ( (This)->lpVtbl -> AddTransitionBetweenKeyframes(This,variable,transition,startKeyframe,endKeyframe) ) 

#define IUIAnimationStoryboard_RepeatBetweenKeyframes(This,startKeyframe,endKeyframe,repetitionCount)	\
    ( (This)->lpVtbl -> RepeatBetweenKeyframes(This,startKeyframe,endKeyframe,repetitionCount) ) 

#define IUIAnimationStoryboard_HoldVariable(This,variable)	\
    ( (This)->lpVtbl -> HoldVariable(This,variable) ) 

#define IUIAnimationStoryboard_SetLongestAcceptableDelay(This,delay)	\
    ( (This)->lpVtbl -> SetLongestAcceptableDelay(This,delay) ) 

#define IUIAnimationStoryboard_Schedule(This,timeNow,schedulingResult)	\
    ( (This)->lpVtbl -> Schedule(This,timeNow,schedulingResult) ) 

#define IUIAnimationStoryboard_Conclude(This)	\
    ( (This)->lpVtbl -> Conclude(This) ) 

#define IUIAnimationStoryboard_Finish(This,completionDeadline)	\
    ( (This)->lpVtbl -> Finish(This,completionDeadline) ) 

#define IUIAnimationStoryboard_Abandon(This)	\
    ( (This)->lpVtbl -> Abandon(This) ) 

#define IUIAnimationStoryboard_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IUIAnimationStoryboard_GetTag(This,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,object,id) ) 

#define IUIAnimationStoryboard_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#define IUIAnimationStoryboard_GetElapsedTime(This,elapsedTime)	\
    ( (This)->lpVtbl -> GetElapsedTime(This,elapsedTime) ) 

#define IUIAnimationStoryboard_SetStoryboardEventHandler(This,handler)	\
    ( (This)->lpVtbl -> SetStoryboardEventHandler(This,handler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationStoryboard_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationTransition_INTERFACE_DEFINED__
#define __IUIAnimationTransition_INTERFACE_DEFINED__

/* interface IUIAnimationTransition */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTransition;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DC6CE252-F731-41cf-B610-614B6CA049AD")
    IUIAnimationTransition : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialValue( 
            /* [annotation][in] */ 
            _In_  DOUBLE value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialVelocity( 
            /* [annotation][in] */ 
            _In_  DOUBLE velocity) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE IsDurationKnown( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTransitionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTransition * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTransition * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTransition * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition, SetInitialValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialValue )( 
            IUIAnimationTransition * This,
            /* [annotation][in] */ 
            _In_  DOUBLE value);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition, SetInitialVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialVelocity )( 
            IUIAnimationTransition * This,
            /* [annotation][in] */ 
            _In_  DOUBLE velocity);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition, IsDurationKnown)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *IsDurationKnown )( 
            IUIAnimationTransition * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition, GetDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            IUIAnimationTransition * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration);
        
        END_INTERFACE
    } IUIAnimationTransitionVtbl;

    interface IUIAnimationTransition
    {
        CONST_VTBL struct IUIAnimationTransitionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTransition_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTransition_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTransition_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTransition_SetInitialValue(This,value)	\
    ( (This)->lpVtbl -> SetInitialValue(This,value) ) 

#define IUIAnimationTransition_SetInitialVelocity(This,velocity)	\
    ( (This)->lpVtbl -> SetInitialVelocity(This,velocity) ) 

#define IUIAnimationTransition_IsDurationKnown(This)	\
    ( (This)->lpVtbl -> IsDurationKnown(This) ) 

#define IUIAnimationTransition_GetDuration(This,duration)	\
    ( (This)->lpVtbl -> GetDuration(This,duration) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTransition_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationManagerEventHandler_INTERFACE_DEFINED__
#define __IUIAnimationManagerEventHandler_INTERFACE_DEFINED__

/* interface IUIAnimationManagerEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationManagerEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("783321ED-78A3-4366-B574-6AF607A64788")
    IUIAnimationManagerEventHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnManagerStatusChanged( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS previousStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationManagerEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationManagerEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationManagerEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationManagerEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManagerEventHandler, OnManagerStatusChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnManagerStatusChanged )( 
            IUIAnimationManagerEventHandler * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS previousStatus);
        
        END_INTERFACE
    } IUIAnimationManagerEventHandlerVtbl;

    interface IUIAnimationManagerEventHandler
    {
        CONST_VTBL struct IUIAnimationManagerEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationManagerEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationManagerEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationManagerEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationManagerEventHandler_OnManagerStatusChanged(This,newStatus,previousStatus)	\
    ( (This)->lpVtbl -> OnManagerStatusChanged(This,newStatus,previousStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationManagerEventHandler_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationVariableChangeHandler_INTERFACE_DEFINED__
#define __IUIAnimationVariableChangeHandler_INTERFACE_DEFINED__

/* interface IUIAnimationVariableChangeHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariableChangeHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6358B7BA-87D2-42d5-BF71-82E919DD5862")
    IUIAnimationVariableChangeHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnValueChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  DOUBLE newValue,
            /* [annotation][in] */ 
            _In_  DOUBLE previousValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariableChangeHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariableChangeHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariableChangeHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariableChangeHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariableChangeHandler, OnValueChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnValueChanged )( 
            IUIAnimationVariableChangeHandler * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  DOUBLE newValue,
            /* [annotation][in] */ 
            _In_  DOUBLE previousValue);
        
        END_INTERFACE
    } IUIAnimationVariableChangeHandlerVtbl;

    interface IUIAnimationVariableChangeHandler
    {
        CONST_VTBL struct IUIAnimationVariableChangeHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariableChangeHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariableChangeHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariableChangeHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariableChangeHandler_OnValueChanged(This,storyboard,variable,newValue,previousValue)	\
    ( (This)->lpVtbl -> OnValueChanged(This,storyboard,variable,newValue,previousValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariableChangeHandler_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationVariableIntegerChangeHandler_INTERFACE_DEFINED__
#define __IUIAnimationVariableIntegerChangeHandler_INTERFACE_DEFINED__

/* interface IUIAnimationVariableIntegerChangeHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariableIntegerChangeHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB3E1550-356E-44b0-99DA-85AC6017865E")
    IUIAnimationVariableIntegerChangeHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnIntegerValueChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  INT32 newValue,
            /* [annotation][in] */ 
            _In_  INT32 previousValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariableIntegerChangeHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariableIntegerChangeHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariableIntegerChangeHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariableIntegerChangeHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariableIntegerChangeHandler, OnIntegerValueChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnIntegerValueChanged )( 
            IUIAnimationVariableIntegerChangeHandler * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable *variable,
            /* [annotation][in] */ 
            _In_  INT32 newValue,
            /* [annotation][in] */ 
            _In_  INT32 previousValue);
        
        END_INTERFACE
    } IUIAnimationVariableIntegerChangeHandlerVtbl;

    interface IUIAnimationVariableIntegerChangeHandler
    {
        CONST_VTBL struct IUIAnimationVariableIntegerChangeHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariableIntegerChangeHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariableIntegerChangeHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariableIntegerChangeHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariableIntegerChangeHandler_OnIntegerValueChanged(This,storyboard,variable,newValue,previousValue)	\
    ( (This)->lpVtbl -> OnIntegerValueChanged(This,storyboard,variable,newValue,previousValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariableIntegerChangeHandler_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationStoryboardEventHandler_INTERFACE_DEFINED__
#define __IUIAnimationStoryboardEventHandler_INTERFACE_DEFINED__

/* interface IUIAnimationStoryboardEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationStoryboardEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3D5C9008-EC7C-4364-9F8A-9AF3C58CBAE6")
    IUIAnimationStoryboardEventHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnStoryboardStatusChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS previousStatus) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnStoryboardUpdated( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationStoryboardEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationStoryboardEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationStoryboardEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationStoryboardEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboardEventHandler, OnStoryboardStatusChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnStoryboardStatusChanged )( 
            IUIAnimationStoryboardEventHandler * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS previousStatus);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboardEventHandler, OnStoryboardUpdated)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnStoryboardUpdated )( 
            IUIAnimationStoryboardEventHandler * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *storyboard);
        
        END_INTERFACE
    } IUIAnimationStoryboardEventHandlerVtbl;

    interface IUIAnimationStoryboardEventHandler
    {
        CONST_VTBL struct IUIAnimationStoryboardEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationStoryboardEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationStoryboardEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationStoryboardEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationStoryboardEventHandler_OnStoryboardStatusChanged(This,storyboard,newStatus,previousStatus)	\
    ( (This)->lpVtbl -> OnStoryboardStatusChanged(This,storyboard,newStatus,previousStatus) ) 

#define IUIAnimationStoryboardEventHandler_OnStoryboardUpdated(This,storyboard)	\
    ( (This)->lpVtbl -> OnStoryboardUpdated(This,storyboard) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationStoryboardEventHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0008 */
/* [local] */ 

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0008_0001
    {
        UI_ANIMATION_PRIORITY_EFFECT_FAILURE	= 0,
        UI_ANIMATION_PRIORITY_EFFECT_DELAY	= 1
    } 	UI_ANIMATION_PRIORITY_EFFECT;



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0008_v0_0_s_ifspec;

#ifndef __IUIAnimationPriorityComparison_INTERFACE_DEFINED__
#define __IUIAnimationPriorityComparison_INTERFACE_DEFINED__

/* interface IUIAnimationPriorityComparison */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationPriorityComparison;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83FA9B74-5F86-4618-BC6A-A2FAC19B3F44")
    IUIAnimationPriorityComparison : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE HasPriority( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *scheduledStoryboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *newStoryboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_PRIORITY_EFFECT priorityEffect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationPriorityComparisonVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationPriorityComparison * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationPriorityComparison * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationPriorityComparison * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationPriorityComparison, HasPriority)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *HasPriority )( 
            IUIAnimationPriorityComparison * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *scheduledStoryboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard *newStoryboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_PRIORITY_EFFECT priorityEffect);
        
        END_INTERFACE
    } IUIAnimationPriorityComparisonVtbl;

    interface IUIAnimationPriorityComparison
    {
        CONST_VTBL struct IUIAnimationPriorityComparisonVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationPriorityComparison_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationPriorityComparison_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationPriorityComparison_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationPriorityComparison_HasPriority(This,scheduledStoryboard,newStoryboard,priorityEffect)	\
    ( (This)->lpVtbl -> HasPriority(This,scheduledStoryboard,newStoryboard,priorityEffect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationPriorityComparison_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0009 */
/* [local] */ 

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0009_0001
    {
        UI_ANIMATION_SLOPE_INCREASING	= 0,
        UI_ANIMATION_SLOPE_DECREASING	= 1
    } 	UI_ANIMATION_SLOPE;



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0009_v0_0_s_ifspec;

#ifndef __IUIAnimationTransitionLibrary_INTERFACE_DEFINED__
#define __IUIAnimationTransitionLibrary_INTERFACE_DEFINED__

/* interface IUIAnimationTransitionLibrary */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTransitionLibrary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CA5A14B1-D24F-48b8-8FE4-C78169BA954E")
    IUIAnimationTransitionLibrary : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateInstantaneousTransition( 
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateConstantTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateDiscreteTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS hold,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateLinearTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateLinearTransitionFromSpeed( 
            /* [annotation][in] */ 
            _In_  DOUBLE speed,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateSinusoidalTransitionFromVelocity( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateSinusoidalTransitionFromRange( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE minimumValue,
            /* [annotation][in] */ 
            _In_  DOUBLE maximumValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SLOPE slope,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateAccelerateDecelerateTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE accelerationRatio,
            /* [annotation][in] */ 
            _In_  DOUBLE decelerationRatio,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateReversalTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateCubicTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateSmoothStopTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS maximumDuration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateParabolicTransitionFromAcceleration( 
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][in] */ 
            _In_  DOUBLE acceleration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTransitionLibraryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTransitionLibrary * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTransitionLibrary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTransitionLibrary * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateInstantaneousTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateInstantaneousTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateConstantTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateConstantTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateDiscreteTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateDiscreteTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS hold,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateLinearTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateLinearTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateLinearTransitionFromSpeed)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateLinearTransitionFromSpeed )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  DOUBLE speed,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateSinusoidalTransitionFromVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateSinusoidalTransitionFromVelocity )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateSinusoidalTransitionFromRange)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateSinusoidalTransitionFromRange )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE minimumValue,
            /* [annotation][in] */ 
            _In_  DOUBLE maximumValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SLOPE slope,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateAccelerateDecelerateTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateAccelerateDecelerateTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE accelerationRatio,
            /* [annotation][in] */ 
            _In_  DOUBLE decelerationRatio,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateReversalTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateReversalTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateCubicTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateCubicTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateSmoothStopTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateSmoothStopTransition )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS maximumDuration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary, CreateParabolicTransitionFromAcceleration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateParabolicTransitionFromAcceleration )( 
            IUIAnimationTransitionLibrary * This,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][in] */ 
            _In_  DOUBLE acceleration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        END_INTERFACE
    } IUIAnimationTransitionLibraryVtbl;

    interface IUIAnimationTransitionLibrary
    {
        CONST_VTBL struct IUIAnimationTransitionLibraryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTransitionLibrary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTransitionLibrary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTransitionLibrary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTransitionLibrary_CreateInstantaneousTransition(This,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateInstantaneousTransition(This,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary_CreateConstantTransition(This,duration,transition)	\
    ( (This)->lpVtbl -> CreateConstantTransition(This,duration,transition) ) 

#define IUIAnimationTransitionLibrary_CreateDiscreteTransition(This,delay,finalValue,hold,transition)	\
    ( (This)->lpVtbl -> CreateDiscreteTransition(This,delay,finalValue,hold,transition) ) 

#define IUIAnimationTransitionLibrary_CreateLinearTransition(This,duration,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateLinearTransition(This,duration,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary_CreateLinearTransitionFromSpeed(This,speed,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateLinearTransitionFromSpeed(This,speed,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary_CreateSinusoidalTransitionFromVelocity(This,duration,period,transition)	\
    ( (This)->lpVtbl -> CreateSinusoidalTransitionFromVelocity(This,duration,period,transition) ) 

#define IUIAnimationTransitionLibrary_CreateSinusoidalTransitionFromRange(This,duration,minimumValue,maximumValue,period,slope,transition)	\
    ( (This)->lpVtbl -> CreateSinusoidalTransitionFromRange(This,duration,minimumValue,maximumValue,period,slope,transition) ) 

#define IUIAnimationTransitionLibrary_CreateAccelerateDecelerateTransition(This,duration,finalValue,accelerationRatio,decelerationRatio,transition)	\
    ( (This)->lpVtbl -> CreateAccelerateDecelerateTransition(This,duration,finalValue,accelerationRatio,decelerationRatio,transition) ) 

#define IUIAnimationTransitionLibrary_CreateReversalTransition(This,duration,transition)	\
    ( (This)->lpVtbl -> CreateReversalTransition(This,duration,transition) ) 

#define IUIAnimationTransitionLibrary_CreateCubicTransition(This,duration,finalValue,finalVelocity,transition)	\
    ( (This)->lpVtbl -> CreateCubicTransition(This,duration,finalValue,finalVelocity,transition) ) 

#define IUIAnimationTransitionLibrary_CreateSmoothStopTransition(This,maximumDuration,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateSmoothStopTransition(This,maximumDuration,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary_CreateParabolicTransitionFromAcceleration(This,finalValue,finalVelocity,acceleration,transition)	\
    ( (This)->lpVtbl -> CreateParabolicTransitionFromAcceleration(This,finalValue,finalVelocity,acceleration,transition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTransitionLibrary_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0010 */
/* [local] */ 

typedef /* [public][public][public][public][public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0010_0001
    {
        UI_ANIMATION_DEPENDENCY_NONE	= 0,
        UI_ANIMATION_DEPENDENCY_INTERMEDIATE_VALUES	= 0x1,
        UI_ANIMATION_DEPENDENCY_FINAL_VALUE	= 0x2,
        UI_ANIMATION_DEPENDENCY_FINAL_VELOCITY	= 0x4,
        UI_ANIMATION_DEPENDENCY_DURATION	= 0x8
    } 	UI_ANIMATION_DEPENDENCIES;

DEFINE_ENUM_FLAG_OPERATORS(UI_ANIMATION_DEPENDENCIES);


extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0010_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0010_v0_0_s_ifspec;

#ifndef __IUIAnimationInterpolator_INTERFACE_DEFINED__
#define __IUIAnimationInterpolator_INTERFACE_DEFINED__

/* interface IUIAnimationInterpolator */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationInterpolator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7815CBBA-DDF7-478c-A46C-7B6C738B7978")
    IUIAnimationInterpolator : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialValueAndVelocity( 
            /* [annotation][in] */ 
            _In_  DOUBLE initialValue,
            /* [annotation][in] */ 
            _In_  DOUBLE initialVelocity) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetDuration( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE InterpolateValue( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE InterpolateVelocity( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *velocity) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDependencies( 
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialValueDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialVelocityDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *durationDependencies) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationInterpolatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationInterpolator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationInterpolator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationInterpolator * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, SetInitialValueAndVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialValueAndVelocity )( 
            IUIAnimationInterpolator * This,
            /* [annotation][in] */ 
            _In_  DOUBLE initialValue,
            /* [annotation][in] */ 
            _In_  DOUBLE initialVelocity);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, SetDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetDuration )( 
            IUIAnimationInterpolator * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, GetDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            IUIAnimationInterpolator * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, GetFinalValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalValue )( 
            IUIAnimationInterpolator * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, InterpolateValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *InterpolateValue )( 
            IUIAnimationInterpolator * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, InterpolateVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *InterpolateVelocity )( 
            IUIAnimationInterpolator * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *velocity);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator, GetDependencies)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDependencies )( 
            IUIAnimationInterpolator * This,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialValueDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialVelocityDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *durationDependencies);
        
        END_INTERFACE
    } IUIAnimationInterpolatorVtbl;

    interface IUIAnimationInterpolator
    {
        CONST_VTBL struct IUIAnimationInterpolatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationInterpolator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationInterpolator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationInterpolator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationInterpolator_SetInitialValueAndVelocity(This,initialValue,initialVelocity)	\
    ( (This)->lpVtbl -> SetInitialValueAndVelocity(This,initialValue,initialVelocity) ) 

#define IUIAnimationInterpolator_SetDuration(This,duration)	\
    ( (This)->lpVtbl -> SetDuration(This,duration) ) 

#define IUIAnimationInterpolator_GetDuration(This,duration)	\
    ( (This)->lpVtbl -> GetDuration(This,duration) ) 

#define IUIAnimationInterpolator_GetFinalValue(This,value)	\
    ( (This)->lpVtbl -> GetFinalValue(This,value) ) 

#define IUIAnimationInterpolator_InterpolateValue(This,offset,value)	\
    ( (This)->lpVtbl -> InterpolateValue(This,offset,value) ) 

#define IUIAnimationInterpolator_InterpolateVelocity(This,offset,velocity)	\
    ( (This)->lpVtbl -> InterpolateVelocity(This,offset,velocity) ) 

#define IUIAnimationInterpolator_GetDependencies(This,initialValueDependencies,initialVelocityDependencies,durationDependencies)	\
    ( (This)->lpVtbl -> GetDependencies(This,initialValueDependencies,initialVelocityDependencies,durationDependencies) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationInterpolator_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationTransitionFactory_INTERFACE_DEFINED__
#define __IUIAnimationTransitionFactory_INTERFACE_DEFINED__

/* interface IUIAnimationTransitionFactory */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTransitionFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FCD91E03-3E3B-45ad-BBB1-6DFC8153743D")
    IUIAnimationTransitionFactory : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationInterpolator *interpolator,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTransitionFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTransitionFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTransitionFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTransitionFactory * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionFactory, CreateTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateTransition )( 
            IUIAnimationTransitionFactory * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationInterpolator *interpolator,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition **transition);
        
        END_INTERFACE
    } IUIAnimationTransitionFactoryVtbl;

    interface IUIAnimationTransitionFactory
    {
        CONST_VTBL struct IUIAnimationTransitionFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTransitionFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTransitionFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTransitionFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTransitionFactory_CreateTransition(This,interpolator,transition)	\
    ( (This)->lpVtbl -> CreateTransition(This,interpolator,transition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTransitionFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0012 */
/* [local] */ 

typedef /* [public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0012_0001
    {
        UI_ANIMATION_IDLE_BEHAVIOR_CONTINUE	= 0,
        UI_ANIMATION_IDLE_BEHAVIOR_DISABLE	= 1
    } 	UI_ANIMATION_IDLE_BEHAVIOR;



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0012_v0_0_s_ifspec;

#ifndef __IUIAnimationTimer_INTERFACE_DEFINED__
#define __IUIAnimationTimer_INTERFACE_DEFINED__

/* interface IUIAnimationTimer */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTimer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6B0EFAD1-A053-41d6-9085-33A689144665")
    IUIAnimationTimer : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTimerUpdateHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationTimerUpdateHandler *updateHandler,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_IDLE_BEHAVIOR idleBehavior) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTimerEventHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationTimerEventHandler *handler) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Enable( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Disable( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE IsEnabled( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetTime( 
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_SECONDS *seconds) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetFrameRateThreshold( 
            /* [annotation][in] */ 
            _In_  UINT32 framesPerSecond) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTimerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTimer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTimer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTimer * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, SetTimerUpdateHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTimerUpdateHandler )( 
            IUIAnimationTimer * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationTimerUpdateHandler *updateHandler,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_IDLE_BEHAVIOR idleBehavior);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, SetTimerEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTimerEventHandler )( 
            IUIAnimationTimer * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationTimerEventHandler *handler);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, Enable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IUIAnimationTimer * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, Disable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Disable )( 
            IUIAnimationTimer * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, IsEnabled)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *IsEnabled )( 
            IUIAnimationTimer * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, GetTime)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IUIAnimationTimer * This,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_SECONDS *seconds);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimer, SetFrameRateThreshold)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetFrameRateThreshold )( 
            IUIAnimationTimer * This,
            /* [annotation][in] */ 
            _In_  UINT32 framesPerSecond);
        
        END_INTERFACE
    } IUIAnimationTimerVtbl;

    interface IUIAnimationTimer
    {
        CONST_VTBL struct IUIAnimationTimerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTimer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTimer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTimer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTimer_SetTimerUpdateHandler(This,updateHandler,idleBehavior)	\
    ( (This)->lpVtbl -> SetTimerUpdateHandler(This,updateHandler,idleBehavior) ) 

#define IUIAnimationTimer_SetTimerEventHandler(This,handler)	\
    ( (This)->lpVtbl -> SetTimerEventHandler(This,handler) ) 

#define IUIAnimationTimer_Enable(This)	\
    ( (This)->lpVtbl -> Enable(This) ) 

#define IUIAnimationTimer_Disable(This)	\
    ( (This)->lpVtbl -> Disable(This) ) 

#define IUIAnimationTimer_IsEnabled(This)	\
    ( (This)->lpVtbl -> IsEnabled(This) ) 

#define IUIAnimationTimer_GetTime(This,seconds)	\
    ( (This)->lpVtbl -> GetTime(This,seconds) ) 

#define IUIAnimationTimer_SetFrameRateThreshold(This,framesPerSecond)	\
    ( (This)->lpVtbl -> SetFrameRateThreshold(This,framesPerSecond) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTimer_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationTimerUpdateHandler_INTERFACE_DEFINED__
#define __IUIAnimationTimerUpdateHandler_INTERFACE_DEFINED__

/* interface IUIAnimationTimerUpdateHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTimerUpdateHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("195509B7-5D5E-4e3e-B278-EE3759B367AD")
    IUIAnimationTimerUpdateHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnUpdate( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_UPDATE_RESULT *result) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTimerClientEventHandler( 
            /* [annotation][in] */ 
            _In_  IUIAnimationTimerClientEventHandler *handler) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE ClearTimerClientEventHandler( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTimerUpdateHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTimerUpdateHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTimerUpdateHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTimerUpdateHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerUpdateHandler, OnUpdate)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnUpdate )( 
            IUIAnimationTimerUpdateHandler * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_UPDATE_RESULT *result);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerUpdateHandler, SetTimerClientEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTimerClientEventHandler )( 
            IUIAnimationTimerUpdateHandler * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationTimerClientEventHandler *handler);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerUpdateHandler, ClearTimerClientEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *ClearTimerClientEventHandler )( 
            IUIAnimationTimerUpdateHandler * This);
        
        END_INTERFACE
    } IUIAnimationTimerUpdateHandlerVtbl;

    interface IUIAnimationTimerUpdateHandler
    {
        CONST_VTBL struct IUIAnimationTimerUpdateHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTimerUpdateHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTimerUpdateHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTimerUpdateHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTimerUpdateHandler_OnUpdate(This,timeNow,result)	\
    ( (This)->lpVtbl -> OnUpdate(This,timeNow,result) ) 

#define IUIAnimationTimerUpdateHandler_SetTimerClientEventHandler(This,handler)	\
    ( (This)->lpVtbl -> SetTimerClientEventHandler(This,handler) ) 

#define IUIAnimationTimerUpdateHandler_ClearTimerClientEventHandler(This)	\
    ( (This)->lpVtbl -> ClearTimerClientEventHandler(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTimerUpdateHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0014 */
/* [local] */ 

typedef /* [public][public][public][v1_enum] */ 
enum __MIDL___MIDL_itf_UIAnimation_0000_0014_0001
    {
        UI_ANIMATION_TIMER_CLIENT_IDLE	= 0,
        UI_ANIMATION_TIMER_CLIENT_BUSY	= 1
    } 	UI_ANIMATION_TIMER_CLIENT_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0014_v0_0_s_ifspec;

#ifndef __IUIAnimationTimerClientEventHandler_INTERFACE_DEFINED__
#define __IUIAnimationTimerClientEventHandler_INTERFACE_DEFINED__

/* interface IUIAnimationTimerClientEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTimerClientEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEDB4DB6-94FA-4bfb-A47F-EF2D9E408C25")
    IUIAnimationTimerClientEventHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnTimerClientStatusChanged( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_TIMER_CLIENT_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_TIMER_CLIENT_STATUS previousStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTimerClientEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTimerClientEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTimerClientEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTimerClientEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerClientEventHandler, OnTimerClientStatusChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnTimerClientStatusChanged )( 
            IUIAnimationTimerClientEventHandler * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_TIMER_CLIENT_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_TIMER_CLIENT_STATUS previousStatus);
        
        END_INTERFACE
    } IUIAnimationTimerClientEventHandlerVtbl;

    interface IUIAnimationTimerClientEventHandler
    {
        CONST_VTBL struct IUIAnimationTimerClientEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTimerClientEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTimerClientEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTimerClientEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTimerClientEventHandler_OnTimerClientStatusChanged(This,newStatus,previousStatus)	\
    ( (This)->lpVtbl -> OnTimerClientStatusChanged(This,newStatus,previousStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTimerClientEventHandler_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationTimerEventHandler_INTERFACE_DEFINED__
#define __IUIAnimationTimerEventHandler_INTERFACE_DEFINED__

/* interface IUIAnimationTimerEventHandler */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTimerEventHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("274A7DEA-D771-4095-ABBD-8DF7ABD23CE3")
    IUIAnimationTimerEventHandler : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnPreUpdate( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnPostUpdate( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnRenderingTooSlow( 
            /* [annotation][in] */ 
            _In_  UINT32 framesPerSecond) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTimerEventHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTimerEventHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTimerEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTimerEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerEventHandler, OnPreUpdate)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnPreUpdate )( 
            IUIAnimationTimerEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerEventHandler, OnPostUpdate)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnPostUpdate )( 
            IUIAnimationTimerEventHandler * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTimerEventHandler, OnRenderingTooSlow)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnRenderingTooSlow )( 
            IUIAnimationTimerEventHandler * This,
            /* [annotation][in] */ 
            _In_  UINT32 framesPerSecond);
        
        END_INTERFACE
    } IUIAnimationTimerEventHandlerVtbl;

    interface IUIAnimationTimerEventHandler
    {
        CONST_VTBL struct IUIAnimationTimerEventHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTimerEventHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTimerEventHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTimerEventHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTimerEventHandler_OnPreUpdate(This)	\
    ( (This)->lpVtbl -> OnPreUpdate(This) ) 

#define IUIAnimationTimerEventHandler_OnPostUpdate(This)	\
    ( (This)->lpVtbl -> OnPostUpdate(This) ) 

#define IUIAnimationTimerEventHandler_OnRenderingTooSlow(This,framesPerSecond)	\
    ( (This)->lpVtbl -> OnRenderingTooSlow(This,framesPerSecond) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTimerEventHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0016 */
/* [local] */ 

#define	UI_ANIMATION_SECONDS_INFINITE	( -1 )



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0016_v0_0_s_ifspec;

#ifndef __IUIAnimationManager2_INTERFACE_DEFINED__
#define __IUIAnimationManager2_INTERFACE_DEFINED__

/* interface IUIAnimationManager2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationManager2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8B6F7D4-4109-4d3f-ACEE-879926968CB1")
    IUIAnimationManager2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateAnimationVectorVariable( 
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *initialValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable2 **variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateAnimationVariable( 
            /* [annotation][in] */ 
            _In_  DOUBLE initialValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable2 **variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE ScheduleTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateStoryboard( 
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard2 **storyboard) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE FinishAllStoryboards( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AbandonAllStoryboards( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Update( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_UPDATE_RESULT *updateResult = 0) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetVariableFromTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable2 **variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetStoryboardFromTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard2 **storyboard) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE EstimateNextEventTime( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *seconds) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_MANAGER_STATUS *status) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetAnimationMode( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MODE mode) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetManagerEventHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationManagerEventHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent = FALSE) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetCancelPriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTrimPriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetCompressPriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetConcludePriorityComparison( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetDefaultLongestAcceptableDelay( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationManager2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationManager2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, CreateAnimationVectorVariable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateAnimationVectorVariable )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *initialValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable2 **variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, CreateAnimationVariable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateAnimationVariable )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE initialValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable2 **variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, ScheduleTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *ScheduleTransition )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, CreateStoryboard)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateStoryboard )( 
            IUIAnimationManager2 * This,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard2 **storyboard);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, FinishAllStoryboards)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *FinishAllStoryboards )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, AbandonAllStoryboards)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AbandonAllStoryboards )( 
            IUIAnimationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, Update)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_UPDATE_RESULT *updateResult);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, GetVariableFromTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetVariableFromTag )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationVariable2 **variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, GetStoryboardFromTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetStoryboardFromTag )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard2 **storyboard);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, EstimateNextEventTime)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *EstimateNextEventTime )( 
            IUIAnimationManager2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *seconds);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, GetStatus)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IUIAnimationManager2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_MANAGER_STATUS *status);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetAnimationMode)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetAnimationMode )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MODE mode);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, Pause)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IUIAnimationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, Resume)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IUIAnimationManager2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetManagerEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetManagerEventHandler )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationManagerEventHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetCancelPriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetCancelPriorityComparison )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetTrimPriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTrimPriorityComparison )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetCompressPriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetCompressPriorityComparison )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetConcludePriorityComparison)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetConcludePriorityComparison )( 
            IUIAnimationManager2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationPriorityComparison2 *comparison);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, SetDefaultLongestAcceptableDelay)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetDefaultLongestAcceptableDelay )( 
            IUIAnimationManager2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay);
        
        DECLSPEC_XFGVIRT(IUIAnimationManager2, Shutdown)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IUIAnimationManager2 * This);
        
        END_INTERFACE
    } IUIAnimationManager2Vtbl;

    interface IUIAnimationManager2
    {
        CONST_VTBL struct IUIAnimationManager2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationManager2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationManager2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationManager2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationManager2_CreateAnimationVectorVariable(This,initialValue,cDimension,variable)	\
    ( (This)->lpVtbl -> CreateAnimationVectorVariable(This,initialValue,cDimension,variable) ) 

#define IUIAnimationManager2_CreateAnimationVariable(This,initialValue,variable)	\
    ( (This)->lpVtbl -> CreateAnimationVariable(This,initialValue,variable) ) 

#define IUIAnimationManager2_ScheduleTransition(This,variable,transition,timeNow)	\
    ( (This)->lpVtbl -> ScheduleTransition(This,variable,transition,timeNow) ) 

#define IUIAnimationManager2_CreateStoryboard(This,storyboard)	\
    ( (This)->lpVtbl -> CreateStoryboard(This,storyboard) ) 

#define IUIAnimationManager2_FinishAllStoryboards(This,completionDeadline)	\
    ( (This)->lpVtbl -> FinishAllStoryboards(This,completionDeadline) ) 

#define IUIAnimationManager2_AbandonAllStoryboards(This)	\
    ( (This)->lpVtbl -> AbandonAllStoryboards(This) ) 

#define IUIAnimationManager2_Update(This,timeNow,updateResult)	\
    ( (This)->lpVtbl -> Update(This,timeNow,updateResult) ) 

#define IUIAnimationManager2_GetVariableFromTag(This,object,id,variable)	\
    ( (This)->lpVtbl -> GetVariableFromTag(This,object,id,variable) ) 

#define IUIAnimationManager2_GetStoryboardFromTag(This,object,id,storyboard)	\
    ( (This)->lpVtbl -> GetStoryboardFromTag(This,object,id,storyboard) ) 

#define IUIAnimationManager2_EstimateNextEventTime(This,seconds)	\
    ( (This)->lpVtbl -> EstimateNextEventTime(This,seconds) ) 

#define IUIAnimationManager2_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#define IUIAnimationManager2_SetAnimationMode(This,mode)	\
    ( (This)->lpVtbl -> SetAnimationMode(This,mode) ) 

#define IUIAnimationManager2_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IUIAnimationManager2_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IUIAnimationManager2_SetManagerEventHandler(This,handler,fRegisterForNextAnimationEvent)	\
    ( (This)->lpVtbl -> SetManagerEventHandler(This,handler,fRegisterForNextAnimationEvent) ) 

#define IUIAnimationManager2_SetCancelPriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetCancelPriorityComparison(This,comparison) ) 

#define IUIAnimationManager2_SetTrimPriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetTrimPriorityComparison(This,comparison) ) 

#define IUIAnimationManager2_SetCompressPriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetCompressPriorityComparison(This,comparison) ) 

#define IUIAnimationManager2_SetConcludePriorityComparison(This,comparison)	\
    ( (This)->lpVtbl -> SetConcludePriorityComparison(This,comparison) ) 

#define IUIAnimationManager2_SetDefaultLongestAcceptableDelay(This,delay)	\
    ( (This)->lpVtbl -> SetDefaultLongestAcceptableDelay(This,delay) ) 

#define IUIAnimationManager2_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationManager2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationVariable2_INTERFACE_DEFINED__
#define __IUIAnimationVariable2_INTERFACE_DEFINED__

/* interface IUIAnimationVariable2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariable2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4914B304-96AB-44d9-9E77-D5109B7E7466")
    IUIAnimationVariable2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDimension( 
            /* [annotation][retval][out] */ 
            _Out_  UINT *dimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetVectorValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetCurve( 
            /* [annotation][in] */ 
            _In_  IDCompositionAnimation *animation) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetVectorCurve( 
            /* [annotation][in] */ 
            _In_reads_(cDimension)  IDCompositionAnimation **animation,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *finalValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalVectorValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPreviousValue( 
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *previousValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPreviousVectorValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetIntegerValue( 
            /* [annotation][retval][out] */ 
            _Out_  INT32 *value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetIntegerVectorValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  INT32 *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalIntegerValue( 
            /* [annotation][retval][out] */ 
            _Out_  INT32 *finalValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalIntegerVectorValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  INT32 *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPreviousIntegerValue( 
            /* [annotation][retval][out] */ 
            _Out_  INT32 *previousValue) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPreviousIntegerVectorValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  INT32 *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetCurrentStoryboard( 
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard2 **storyboard) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetLowerBound( 
            /* [annotation][in] */ 
            _In_  DOUBLE bound) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetLowerBoundVector( 
            /* [annotation][out] */ 
            _In_reads_(cDimension)  const DOUBLE *bound,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetUpperBound( 
            /* [annotation][in] */ 
            _In_  DOUBLE bound) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetUpperBoundVector( 
            /* [annotation][out] */ 
            _In_reads_(cDimension)  const DOUBLE *bound,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetRoundingMode( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_ROUNDING_MODE mode) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetTag( 
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetVariableChangeHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableChangeHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent = FALSE) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetVariableIntegerChangeHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableIntegerChangeHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent = FALSE) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetVariableCurveChangeHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableCurveChangeHandler2 *handler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariable2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariable2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariable2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariable2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetDimension)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDimension )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UINT *dimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *value);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetVectorValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetCurve)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetCurve )( 
            IUIAnimationVariable2 * This,
            /* [annotation][in] */ 
            _In_  IDCompositionAnimation *animation);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetVectorCurve)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetVectorCurve )( 
            IUIAnimationVariable2 * This,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  IDCompositionAnimation **animation,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetFinalValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *finalValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetFinalVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalVectorValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetPreviousValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPreviousValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  DOUBLE *previousValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetPreviousVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPreviousVectorValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetIntegerValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetIntegerValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  INT32 *value);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetIntegerVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetIntegerVectorValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  INT32 *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetFinalIntegerValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalIntegerValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  INT32 *finalValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetFinalIntegerVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalIntegerVectorValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  INT32 *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetPreviousIntegerValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPreviousIntegerValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Out_  INT32 *previousValue);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetPreviousIntegerVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPreviousIntegerVectorValue )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  INT32 *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetCurrentStoryboard)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetCurrentStoryboard )( 
            IUIAnimationVariable2 * This,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationStoryboard2 **storyboard);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetLowerBound)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetLowerBound )( 
            IUIAnimationVariable2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE bound);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetLowerBoundVector)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetLowerBoundVector )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _In_reads_(cDimension)  const DOUBLE *bound,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetUpperBound)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetUpperBound )( 
            IUIAnimationVariable2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE bound);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetUpperBoundVector)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetUpperBoundVector )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _In_reads_(cDimension)  const DOUBLE *bound,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetRoundingMode)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetRoundingMode )( 
            IUIAnimationVariable2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_ROUNDING_MODE mode);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IUIAnimationVariable2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, GetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IUIAnimationVariable2 * This,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetVariableChangeHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetVariableChangeHandler )( 
            IUIAnimationVariable2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableChangeHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetVariableIntegerChangeHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetVariableIntegerChangeHandler )( 
            IUIAnimationVariable2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableIntegerChangeHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariable2, SetVariableCurveChangeHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetVariableCurveChangeHandler )( 
            IUIAnimationVariable2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationVariableCurveChangeHandler2 *handler);
        
        END_INTERFACE
    } IUIAnimationVariable2Vtbl;

    interface IUIAnimationVariable2
    {
        CONST_VTBL struct IUIAnimationVariable2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariable2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariable2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariable2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariable2_GetDimension(This,dimension)	\
    ( (This)->lpVtbl -> GetDimension(This,dimension) ) 

#define IUIAnimationVariable2_GetValue(This,value)	\
    ( (This)->lpVtbl -> GetValue(This,value) ) 

#define IUIAnimationVariable2_GetVectorValue(This,value,cDimension)	\
    ( (This)->lpVtbl -> GetVectorValue(This,value,cDimension) ) 

#define IUIAnimationVariable2_GetCurve(This,animation)	\
    ( (This)->lpVtbl -> GetCurve(This,animation) ) 

#define IUIAnimationVariable2_GetVectorCurve(This,animation,cDimension)	\
    ( (This)->lpVtbl -> GetVectorCurve(This,animation,cDimension) ) 

#define IUIAnimationVariable2_GetFinalValue(This,finalValue)	\
    ( (This)->lpVtbl -> GetFinalValue(This,finalValue) ) 

#define IUIAnimationVariable2_GetFinalVectorValue(This,finalValue,cDimension)	\
    ( (This)->lpVtbl -> GetFinalVectorValue(This,finalValue,cDimension) ) 

#define IUIAnimationVariable2_GetPreviousValue(This,previousValue)	\
    ( (This)->lpVtbl -> GetPreviousValue(This,previousValue) ) 

#define IUIAnimationVariable2_GetPreviousVectorValue(This,previousValue,cDimension)	\
    ( (This)->lpVtbl -> GetPreviousVectorValue(This,previousValue,cDimension) ) 

#define IUIAnimationVariable2_GetIntegerValue(This,value)	\
    ( (This)->lpVtbl -> GetIntegerValue(This,value) ) 

#define IUIAnimationVariable2_GetIntegerVectorValue(This,value,cDimension)	\
    ( (This)->lpVtbl -> GetIntegerVectorValue(This,value,cDimension) ) 

#define IUIAnimationVariable2_GetFinalIntegerValue(This,finalValue)	\
    ( (This)->lpVtbl -> GetFinalIntegerValue(This,finalValue) ) 

#define IUIAnimationVariable2_GetFinalIntegerVectorValue(This,finalValue,cDimension)	\
    ( (This)->lpVtbl -> GetFinalIntegerVectorValue(This,finalValue,cDimension) ) 

#define IUIAnimationVariable2_GetPreviousIntegerValue(This,previousValue)	\
    ( (This)->lpVtbl -> GetPreviousIntegerValue(This,previousValue) ) 

#define IUIAnimationVariable2_GetPreviousIntegerVectorValue(This,previousValue,cDimension)	\
    ( (This)->lpVtbl -> GetPreviousIntegerVectorValue(This,previousValue,cDimension) ) 

#define IUIAnimationVariable2_GetCurrentStoryboard(This,storyboard)	\
    ( (This)->lpVtbl -> GetCurrentStoryboard(This,storyboard) ) 

#define IUIAnimationVariable2_SetLowerBound(This,bound)	\
    ( (This)->lpVtbl -> SetLowerBound(This,bound) ) 

#define IUIAnimationVariable2_SetLowerBoundVector(This,bound,cDimension)	\
    ( (This)->lpVtbl -> SetLowerBoundVector(This,bound,cDimension) ) 

#define IUIAnimationVariable2_SetUpperBound(This,bound)	\
    ( (This)->lpVtbl -> SetUpperBound(This,bound) ) 

#define IUIAnimationVariable2_SetUpperBoundVector(This,bound,cDimension)	\
    ( (This)->lpVtbl -> SetUpperBoundVector(This,bound,cDimension) ) 

#define IUIAnimationVariable2_SetRoundingMode(This,mode)	\
    ( (This)->lpVtbl -> SetRoundingMode(This,mode) ) 

#define IUIAnimationVariable2_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IUIAnimationVariable2_GetTag(This,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,object,id) ) 

#define IUIAnimationVariable2_SetVariableChangeHandler(This,handler,fRegisterForNextAnimationEvent)	\
    ( (This)->lpVtbl -> SetVariableChangeHandler(This,handler,fRegisterForNextAnimationEvent) ) 

#define IUIAnimationVariable2_SetVariableIntegerChangeHandler(This,handler,fRegisterForNextAnimationEvent)	\
    ( (This)->lpVtbl -> SetVariableIntegerChangeHandler(This,handler,fRegisterForNextAnimationEvent) ) 

#define IUIAnimationVariable2_SetVariableCurveChangeHandler(This,handler)	\
    ( (This)->lpVtbl -> SetVariableCurveChangeHandler(This,handler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariable2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0018 */
/* [local] */ 

#define	UI_ANIMATION_DIMENSION_UNKNOWN	( ( UINT  )-1 )



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0018_v0_0_s_ifspec;

#ifndef __IUIAnimationTransition2_INTERFACE_DEFINED__
#define __IUIAnimationTransition2_INTERFACE_DEFINED__

/* interface IUIAnimationTransition2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTransition2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("62FF9123-A85A-4e9b-A218-435A93E268FD")
    IUIAnimationTransition2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDimension( 
            /* [annotation][retval][out] */ 
            _Out_  UINT *dimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialValue( 
            /* [annotation][in] */ 
            _In_  DOUBLE value) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialVectorValue( 
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialVelocity( 
            /* [annotation][in] */ 
            _In_  DOUBLE velocity) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialVectorVelocity( 
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *velocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE IsDurationKnown( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTransition2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTransition2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTransition2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTransition2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, GetDimension)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDimension )( 
            IUIAnimationTransition2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UINT *dimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, SetInitialValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialValue )( 
            IUIAnimationTransition2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE value);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, SetInitialVectorValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialVectorValue )( 
            IUIAnimationTransition2 * This,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, SetInitialVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialVelocity )( 
            IUIAnimationTransition2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE velocity);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, SetInitialVectorVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialVectorVelocity )( 
            IUIAnimationTransition2 * This,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *velocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, IsDurationKnown)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *IsDurationKnown )( 
            IUIAnimationTransition2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransition2, GetDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            IUIAnimationTransition2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration);
        
        END_INTERFACE
    } IUIAnimationTransition2Vtbl;

    interface IUIAnimationTransition2
    {
        CONST_VTBL struct IUIAnimationTransition2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTransition2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTransition2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTransition2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTransition2_GetDimension(This,dimension)	\
    ( (This)->lpVtbl -> GetDimension(This,dimension) ) 

#define IUIAnimationTransition2_SetInitialValue(This,value)	\
    ( (This)->lpVtbl -> SetInitialValue(This,value) ) 

#define IUIAnimationTransition2_SetInitialVectorValue(This,value,cDimension)	\
    ( (This)->lpVtbl -> SetInitialVectorValue(This,value,cDimension) ) 

#define IUIAnimationTransition2_SetInitialVelocity(This,velocity)	\
    ( (This)->lpVtbl -> SetInitialVelocity(This,velocity) ) 

#define IUIAnimationTransition2_SetInitialVectorVelocity(This,velocity,cDimension)	\
    ( (This)->lpVtbl -> SetInitialVectorVelocity(This,velocity,cDimension) ) 

#define IUIAnimationTransition2_IsDurationKnown(This)	\
    ( (This)->lpVtbl -> IsDurationKnown(This) ) 

#define IUIAnimationTransition2_GetDuration(This,duration)	\
    ( (This)->lpVtbl -> GetDuration(This,duration) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTransition2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationManagerEventHandler2_INTERFACE_DEFINED__
#define __IUIAnimationManagerEventHandler2_INTERFACE_DEFINED__

/* interface IUIAnimationManagerEventHandler2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationManagerEventHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F6E022BA-BFF3-42EC-9033-E073F33E83C3")
    IUIAnimationManagerEventHandler2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnManagerStatusChanged( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS previousStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationManagerEventHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationManagerEventHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationManagerEventHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationManagerEventHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationManagerEventHandler2, OnManagerStatusChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnManagerStatusChanged )( 
            IUIAnimationManagerEventHandler2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_MANAGER_STATUS previousStatus);
        
        END_INTERFACE
    } IUIAnimationManagerEventHandler2Vtbl;

    interface IUIAnimationManagerEventHandler2
    {
        CONST_VTBL struct IUIAnimationManagerEventHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationManagerEventHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationManagerEventHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationManagerEventHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationManagerEventHandler2_OnManagerStatusChanged(This,newStatus,previousStatus)	\
    ( (This)->lpVtbl -> OnManagerStatusChanged(This,newStatus,previousStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationManagerEventHandler2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationVariableChangeHandler2_INTERFACE_DEFINED__
#define __IUIAnimationVariableChangeHandler2_INTERFACE_DEFINED__

/* interface IUIAnimationVariableChangeHandler2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariableChangeHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("63ACC8D2-6EAE-4bb0-B879-586DD8CFBE42")
    IUIAnimationVariableChangeHandler2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnValueChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *newValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariableChangeHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariableChangeHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariableChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariableChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariableChangeHandler2, OnValueChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnValueChanged )( 
            IUIAnimationVariableChangeHandler2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *newValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        END_INTERFACE
    } IUIAnimationVariableChangeHandler2Vtbl;

    interface IUIAnimationVariableChangeHandler2
    {
        CONST_VTBL struct IUIAnimationVariableChangeHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariableChangeHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariableChangeHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariableChangeHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariableChangeHandler2_OnValueChanged(This,storyboard,variable,newValue,previousValue,cDimension)	\
    ( (This)->lpVtbl -> OnValueChanged(This,storyboard,variable,newValue,previousValue,cDimension) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariableChangeHandler2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationVariableIntegerChangeHandler2_INTERFACE_DEFINED__
#define __IUIAnimationVariableIntegerChangeHandler2_INTERFACE_DEFINED__

/* interface IUIAnimationVariableIntegerChangeHandler2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariableIntegerChangeHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("829B6CF1-4F3A-4412-AE09-B243EB4C6B58")
    IUIAnimationVariableIntegerChangeHandler2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnIntegerValueChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  INT32 *newValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  INT32 *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariableIntegerChangeHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariableIntegerChangeHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariableIntegerChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariableIntegerChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariableIntegerChangeHandler2, OnIntegerValueChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnIntegerValueChanged )( 
            IUIAnimationVariableIntegerChangeHandler2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  INT32 *newValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  INT32 *previousValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        END_INTERFACE
    } IUIAnimationVariableIntegerChangeHandler2Vtbl;

    interface IUIAnimationVariableIntegerChangeHandler2
    {
        CONST_VTBL struct IUIAnimationVariableIntegerChangeHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariableIntegerChangeHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariableIntegerChangeHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariableIntegerChangeHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariableIntegerChangeHandler2_OnIntegerValueChanged(This,storyboard,variable,newValue,previousValue,cDimension)	\
    ( (This)->lpVtbl -> OnIntegerValueChanged(This,storyboard,variable,newValue,previousValue,cDimension) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariableIntegerChangeHandler2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationVariableCurveChangeHandler2_INTERFACE_DEFINED__
#define __IUIAnimationVariableCurveChangeHandler2_INTERFACE_DEFINED__

/* interface IUIAnimationVariableCurveChangeHandler2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationVariableCurveChangeHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("72895E91-0145-4C21-9192-5AAB40EDDF80")
    IUIAnimationVariableCurveChangeHandler2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnCurveChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationVariableCurveChangeHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationVariableCurveChangeHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationVariableCurveChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationVariableCurveChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationVariableCurveChangeHandler2, OnCurveChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnCurveChanged )( 
            IUIAnimationVariableCurveChangeHandler2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable);
        
        END_INTERFACE
    } IUIAnimationVariableCurveChangeHandler2Vtbl;

    interface IUIAnimationVariableCurveChangeHandler2
    {
        CONST_VTBL struct IUIAnimationVariableCurveChangeHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationVariableCurveChangeHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationVariableCurveChangeHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationVariableCurveChangeHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationVariableCurveChangeHandler2_OnCurveChanged(This,variable)	\
    ( (This)->lpVtbl -> OnCurveChanged(This,variable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationVariableCurveChangeHandler2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationStoryboardEventHandler2_INTERFACE_DEFINED__
#define __IUIAnimationStoryboardEventHandler2_INTERFACE_DEFINED__

/* interface IUIAnimationStoryboardEventHandler2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationStoryboardEventHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAC5F55A-BA7C-414C-B599-FBF850F553C6")
    IUIAnimationStoryboardEventHandler2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnStoryboardStatusChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS previousStatus) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnStoryboardUpdated( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationStoryboardEventHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationStoryboardEventHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationStoryboardEventHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationStoryboardEventHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboardEventHandler2, OnStoryboardStatusChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnStoryboardStatusChanged )( 
            IUIAnimationStoryboardEventHandler2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS newStatus,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_STORYBOARD_STATUS previousStatus);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboardEventHandler2, OnStoryboardUpdated)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnStoryboardUpdated )( 
            IUIAnimationStoryboardEventHandler2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard);
        
        END_INTERFACE
    } IUIAnimationStoryboardEventHandler2Vtbl;

    interface IUIAnimationStoryboardEventHandler2
    {
        CONST_VTBL struct IUIAnimationStoryboardEventHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationStoryboardEventHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationStoryboardEventHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationStoryboardEventHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationStoryboardEventHandler2_OnStoryboardStatusChanged(This,storyboard,newStatus,previousStatus)	\
    ( (This)->lpVtbl -> OnStoryboardStatusChanged(This,storyboard,newStatus,previousStatus) ) 

#define IUIAnimationStoryboardEventHandler2_OnStoryboardUpdated(This,storyboard)	\
    ( (This)->lpVtbl -> OnStoryboardUpdated(This,storyboard) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationStoryboardEventHandler2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_UIAnimation_0000_0024 */
/* [local] */ 

#define	UI_ANIMATION_ITERATION_NONE	( ( UINT32  )-1 )



extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_UIAnimation_0000_0024_v0_0_s_ifspec;

#ifndef __IUIAnimationLoopIterationChangeHandler2_INTERFACE_DEFINED__
#define __IUIAnimationLoopIterationChangeHandler2_INTERFACE_DEFINED__

/* interface IUIAnimationLoopIterationChangeHandler2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationLoopIterationChangeHandler2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2D3B15A4-4762-47AB-A030-B23221DF3AE0")
    IUIAnimationLoopIterationChangeHandler2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE OnLoopIterationChanged( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  UINT_PTR id,
            /* [annotation][in] */ 
            _In_  UINT32 newIterationCount,
            /* [annotation][in] */ 
            _In_  UINT32 oldIterationCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationLoopIterationChangeHandler2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationLoopIterationChangeHandler2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationLoopIterationChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationLoopIterationChangeHandler2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationLoopIterationChangeHandler2, OnLoopIterationChanged)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *OnLoopIterationChanged )( 
            IUIAnimationLoopIterationChangeHandler2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *storyboard,
            /* [annotation][in] */ 
            _In_  UINT_PTR id,
            /* [annotation][in] */ 
            _In_  UINT32 newIterationCount,
            /* [annotation][in] */ 
            _In_  UINT32 oldIterationCount);
        
        END_INTERFACE
    } IUIAnimationLoopIterationChangeHandler2Vtbl;

    interface IUIAnimationLoopIterationChangeHandler2
    {
        CONST_VTBL struct IUIAnimationLoopIterationChangeHandler2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationLoopIterationChangeHandler2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationLoopIterationChangeHandler2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationLoopIterationChangeHandler2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationLoopIterationChangeHandler2_OnLoopIterationChanged(This,storyboard,id,newIterationCount,oldIterationCount)	\
    ( (This)->lpVtbl -> OnLoopIterationChanged(This,storyboard,id,newIterationCount,oldIterationCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationLoopIterationChangeHandler2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationPriorityComparison2_INTERFACE_DEFINED__
#define __IUIAnimationPriorityComparison2_INTERFACE_DEFINED__

/* interface IUIAnimationPriorityComparison2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationPriorityComparison2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5B6D7A37-4621-467C-8B05-70131DE62DDB")
    IUIAnimationPriorityComparison2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE HasPriority( 
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *scheduledStoryboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *newStoryboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_PRIORITY_EFFECT priorityEffect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationPriorityComparison2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationPriorityComparison2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationPriorityComparison2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationPriorityComparison2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationPriorityComparison2, HasPriority)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *HasPriority )( 
            IUIAnimationPriorityComparison2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *scheduledStoryboard,
            /* [annotation][in] */ 
            _In_  IUIAnimationStoryboard2 *newStoryboard,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_PRIORITY_EFFECT priorityEffect);
        
        END_INTERFACE
    } IUIAnimationPriorityComparison2Vtbl;

    interface IUIAnimationPriorityComparison2
    {
        CONST_VTBL struct IUIAnimationPriorityComparison2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationPriorityComparison2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationPriorityComparison2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationPriorityComparison2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationPriorityComparison2_HasPriority(This,scheduledStoryboard,newStoryboard,priorityEffect)	\
    ( (This)->lpVtbl -> HasPriority(This,scheduledStoryboard,newStoryboard,priorityEffect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationPriorityComparison2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationTransitionLibrary2_INTERFACE_DEFINED__
#define __IUIAnimationTransitionLibrary2_INTERFACE_DEFINED__

/* interface IUIAnimationTransitionLibrary2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTransitionLibrary2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03CFAE53-9580-4ee3-B363-2ECE51B4AF6A")
    IUIAnimationTransitionLibrary2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateInstantaneousTransition( 
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateInstantaneousVectorTransition( 
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateConstantTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateDiscreteTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS hold,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateDiscreteVectorTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS hold,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateLinearTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateLinearVectorTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateLinearTransitionFromSpeed( 
            /* [annotation][in] */ 
            _In_  DOUBLE speed,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateLinearVectorTransitionFromSpeed( 
            /* [annotation][in] */ 
            _In_  DOUBLE speed,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateSinusoidalTransitionFromVelocity( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateSinusoidalTransitionFromRange( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE minimumValue,
            /* [annotation][in] */ 
            _In_  DOUBLE maximumValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SLOPE slope,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateAccelerateDecelerateTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE accelerationRatio,
            /* [annotation][in] */ 
            _In_  DOUBLE decelerationRatio,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateReversalTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateCubicTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateCubicVectorTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalVelocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateSmoothStopTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS maximumDuration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateParabolicTransitionFromAcceleration( 
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][in] */ 
            _In_  DOUBLE acceleration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateCubicBezierLinearTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE x1,
            /* [annotation][in] */ 
            _In_  DOUBLE y1,
            /* [annotation][in] */ 
            _In_  DOUBLE x2,
            /* [annotation][in] */ 
            _In_  DOUBLE y2,
            /* [annotation][out] */ 
            _Outptr_  IUIAnimationTransition2 **ppTransition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateCubicBezierLinearVectorTransition( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][in] */ 
            _In_  DOUBLE x1,
            /* [annotation][in] */ 
            _In_  DOUBLE y1,
            /* [annotation][in] */ 
            _In_  DOUBLE x2,
            /* [annotation][in] */ 
            _In_  DOUBLE y2,
            /* [annotation][out] */ 
            _Outptr_  IUIAnimationTransition2 **ppTransition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTransitionLibrary2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTransitionLibrary2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTransitionLibrary2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateInstantaneousTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateInstantaneousTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateInstantaneousVectorTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateInstantaneousVectorTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateConstantTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateConstantTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateDiscreteTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateDiscreteTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS hold,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateDiscreteVectorTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateDiscreteVectorTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS hold,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateLinearTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateLinearTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateLinearVectorTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateLinearVectorTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateLinearTransitionFromSpeed)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateLinearTransitionFromSpeed )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE speed,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateLinearVectorTransitionFromSpeed)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateLinearVectorTransitionFromSpeed )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE speed,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateSinusoidalTransitionFromVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateSinusoidalTransitionFromVelocity )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateSinusoidalTransitionFromRange)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateSinusoidalTransitionFromRange )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE minimumValue,
            /* [annotation][in] */ 
            _In_  DOUBLE maximumValue,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS period,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SLOPE slope,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateAccelerateDecelerateTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateAccelerateDecelerateTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE accelerationRatio,
            /* [annotation][in] */ 
            _In_  DOUBLE decelerationRatio,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateReversalTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateReversalTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateCubicTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateCubicTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateCubicVectorTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateCubicVectorTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalVelocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateSmoothStopTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateSmoothStopTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS maximumDuration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateParabolicTransitionFromAcceleration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateParabolicTransitionFromAcceleration )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE finalVelocity,
            /* [annotation][in] */ 
            _In_  DOUBLE acceleration,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateCubicBezierLinearTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateCubicBezierLinearTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_  DOUBLE finalValue,
            /* [annotation][in] */ 
            _In_  DOUBLE x1,
            /* [annotation][in] */ 
            _In_  DOUBLE y1,
            /* [annotation][in] */ 
            _In_  DOUBLE x2,
            /* [annotation][in] */ 
            _In_  DOUBLE y2,
            /* [annotation][out] */ 
            _Outptr_  IUIAnimationTransition2 **ppTransition);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionLibrary2, CreateCubicBezierLinearVectorTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateCubicBezierLinearVectorTransition )( 
            IUIAnimationTransitionLibrary2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  const DOUBLE *finalValue,
            /* [annotation][in] */ 
            _In_  UINT cDimension,
            /* [annotation][in] */ 
            _In_  DOUBLE x1,
            /* [annotation][in] */ 
            _In_  DOUBLE y1,
            /* [annotation][in] */ 
            _In_  DOUBLE x2,
            /* [annotation][in] */ 
            _In_  DOUBLE y2,
            /* [annotation][out] */ 
            _Outptr_  IUIAnimationTransition2 **ppTransition);
        
        END_INTERFACE
    } IUIAnimationTransitionLibrary2Vtbl;

    interface IUIAnimationTransitionLibrary2
    {
        CONST_VTBL struct IUIAnimationTransitionLibrary2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTransitionLibrary2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTransitionLibrary2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTransitionLibrary2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTransitionLibrary2_CreateInstantaneousTransition(This,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateInstantaneousTransition(This,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateInstantaneousVectorTransition(This,finalValue,cDimension,transition)	\
    ( (This)->lpVtbl -> CreateInstantaneousVectorTransition(This,finalValue,cDimension,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateConstantTransition(This,duration,transition)	\
    ( (This)->lpVtbl -> CreateConstantTransition(This,duration,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateDiscreteTransition(This,delay,finalValue,hold,transition)	\
    ( (This)->lpVtbl -> CreateDiscreteTransition(This,delay,finalValue,hold,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateDiscreteVectorTransition(This,delay,finalValue,cDimension,hold,transition)	\
    ( (This)->lpVtbl -> CreateDiscreteVectorTransition(This,delay,finalValue,cDimension,hold,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateLinearTransition(This,duration,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateLinearTransition(This,duration,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateLinearVectorTransition(This,duration,finalValue,cDimension,transition)	\
    ( (This)->lpVtbl -> CreateLinearVectorTransition(This,duration,finalValue,cDimension,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateLinearTransitionFromSpeed(This,speed,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateLinearTransitionFromSpeed(This,speed,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateLinearVectorTransitionFromSpeed(This,speed,finalValue,cDimension,transition)	\
    ( (This)->lpVtbl -> CreateLinearVectorTransitionFromSpeed(This,speed,finalValue,cDimension,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateSinusoidalTransitionFromVelocity(This,duration,period,transition)	\
    ( (This)->lpVtbl -> CreateSinusoidalTransitionFromVelocity(This,duration,period,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateSinusoidalTransitionFromRange(This,duration,minimumValue,maximumValue,period,slope,transition)	\
    ( (This)->lpVtbl -> CreateSinusoidalTransitionFromRange(This,duration,minimumValue,maximumValue,period,slope,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateAccelerateDecelerateTransition(This,duration,finalValue,accelerationRatio,decelerationRatio,transition)	\
    ( (This)->lpVtbl -> CreateAccelerateDecelerateTransition(This,duration,finalValue,accelerationRatio,decelerationRatio,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateReversalTransition(This,duration,transition)	\
    ( (This)->lpVtbl -> CreateReversalTransition(This,duration,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateCubicTransition(This,duration,finalValue,finalVelocity,transition)	\
    ( (This)->lpVtbl -> CreateCubicTransition(This,duration,finalValue,finalVelocity,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateCubicVectorTransition(This,duration,finalValue,finalVelocity,cDimension,transition)	\
    ( (This)->lpVtbl -> CreateCubicVectorTransition(This,duration,finalValue,finalVelocity,cDimension,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateSmoothStopTransition(This,maximumDuration,finalValue,transition)	\
    ( (This)->lpVtbl -> CreateSmoothStopTransition(This,maximumDuration,finalValue,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateParabolicTransitionFromAcceleration(This,finalValue,finalVelocity,acceleration,transition)	\
    ( (This)->lpVtbl -> CreateParabolicTransitionFromAcceleration(This,finalValue,finalVelocity,acceleration,transition) ) 

#define IUIAnimationTransitionLibrary2_CreateCubicBezierLinearTransition(This,duration,finalValue,x1,y1,x2,y2,ppTransition)	\
    ( (This)->lpVtbl -> CreateCubicBezierLinearTransition(This,duration,finalValue,x1,y1,x2,y2,ppTransition) ) 

#define IUIAnimationTransitionLibrary2_CreateCubicBezierLinearVectorTransition(This,duration,finalValue,cDimension,x1,y1,x2,y2,ppTransition)	\
    ( (This)->lpVtbl -> CreateCubicBezierLinearVectorTransition(This,duration,finalValue,cDimension,x1,y1,x2,y2,ppTransition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTransitionLibrary2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationPrimitiveInterpolation_INTERFACE_DEFINED__
#define __IUIAnimationPrimitiveInterpolation_INTERFACE_DEFINED__

/* interface IUIAnimationPrimitiveInterpolation */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationPrimitiveInterpolation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAB20D63-4361-45DA-A24F-AB8508846B5B")
    IUIAnimationPrimitiveInterpolation : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddCubic( 
            /* [annotation][in] */ 
            _In_  UINT dimension,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS beginOffset,
            /* [annotation][in] */ 
            _In_  FLOAT constantCoefficient,
            /* [annotation][in] */ 
            _In_  FLOAT linearCoefficient,
            /* [annotation][in] */ 
            _In_  FLOAT quadraticCoefficient,
            /* [annotation][in] */ 
            _In_  FLOAT cubicCoefficient) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSinusoidal( 
            /* [annotation][in] */ 
            _In_  UINT dimension,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS beginOffset,
            /* [annotation][in] */ 
            _In_  FLOAT bias,
            /* [annotation][in] */ 
            _In_  FLOAT amplitude,
            /* [annotation][in] */ 
            _In_  FLOAT frequency,
            /* [annotation][in] */ 
            _In_  FLOAT phase) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationPrimitiveInterpolationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationPrimitiveInterpolation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationPrimitiveInterpolation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationPrimitiveInterpolation * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationPrimitiveInterpolation, AddCubic)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddCubic )( 
            IUIAnimationPrimitiveInterpolation * This,
            /* [annotation][in] */ 
            _In_  UINT dimension,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS beginOffset,
            /* [annotation][in] */ 
            _In_  FLOAT constantCoefficient,
            /* [annotation][in] */ 
            _In_  FLOAT linearCoefficient,
            /* [annotation][in] */ 
            _In_  FLOAT quadraticCoefficient,
            /* [annotation][in] */ 
            _In_  FLOAT cubicCoefficient);
        
        DECLSPEC_XFGVIRT(IUIAnimationPrimitiveInterpolation, AddSinusoidal)
        HRESULT ( STDMETHODCALLTYPE *AddSinusoidal )( 
            IUIAnimationPrimitiveInterpolation * This,
            /* [annotation][in] */ 
            _In_  UINT dimension,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS beginOffset,
            /* [annotation][in] */ 
            _In_  FLOAT bias,
            /* [annotation][in] */ 
            _In_  FLOAT amplitude,
            /* [annotation][in] */ 
            _In_  FLOAT frequency,
            /* [annotation][in] */ 
            _In_  FLOAT phase);
        
        END_INTERFACE
    } IUIAnimationPrimitiveInterpolationVtbl;

    interface IUIAnimationPrimitiveInterpolation
    {
        CONST_VTBL struct IUIAnimationPrimitiveInterpolationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationPrimitiveInterpolation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationPrimitiveInterpolation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationPrimitiveInterpolation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationPrimitiveInterpolation_AddCubic(This,dimension,beginOffset,constantCoefficient,linearCoefficient,quadraticCoefficient,cubicCoefficient)	\
    ( (This)->lpVtbl -> AddCubic(This,dimension,beginOffset,constantCoefficient,linearCoefficient,quadraticCoefficient,cubicCoefficient) ) 

#define IUIAnimationPrimitiveInterpolation_AddSinusoidal(This,dimension,beginOffset,bias,amplitude,frequency,phase)	\
    ( (This)->lpVtbl -> AddSinusoidal(This,dimension,beginOffset,bias,amplitude,frequency,phase) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationPrimitiveInterpolation_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationInterpolator2_INTERFACE_DEFINED__
#define __IUIAnimationInterpolator2_INTERFACE_DEFINED__

/* interface IUIAnimationInterpolator2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationInterpolator2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EA76AFF8-EA22-4a23-A0EF-A6A966703518")
    IUIAnimationInterpolator2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDimension( 
            /* [annotation][retval][out] */ 
            _Out_  UINT *dimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetInitialValueAndVelocity( 
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *initialValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *initialVelocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetDuration( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetFinalValue( 
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE InterpolateValue( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE InterpolateVelocity( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *velocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetPrimitiveInterpolation( 
            /* [annotation][in] */ 
            _In_  IUIAnimationPrimitiveInterpolation *interpolation,
            /* [annotation][in] */ 
            _In_  UINT cDimension) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetDependencies( 
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialValueDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialVelocityDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *durationDependencies) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationInterpolator2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationInterpolator2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationInterpolator2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationInterpolator2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, GetDimension)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDimension )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UINT *dimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, SetInitialValueAndVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetInitialValueAndVelocity )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *initialValue,
            /* [annotation][in] */ 
            _In_reads_(cDimension)  DOUBLE *initialVelocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, SetDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetDuration )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS duration);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, GetDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_SECONDS *duration);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, GetFinalValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetFinalValue )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, InterpolateValue)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *InterpolateValue )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *value,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, InterpolateVelocity)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *InterpolateVelocity )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][out] */ 
            _Out_writes_(cDimension)  DOUBLE *velocity,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, GetPrimitiveInterpolation)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetPrimitiveInterpolation )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationPrimitiveInterpolation *interpolation,
            /* [annotation][in] */ 
            _In_  UINT cDimension);
        
        DECLSPEC_XFGVIRT(IUIAnimationInterpolator2, GetDependencies)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetDependencies )( 
            IUIAnimationInterpolator2 * This,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialValueDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *initialVelocityDependencies,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_DEPENDENCIES *durationDependencies);
        
        END_INTERFACE
    } IUIAnimationInterpolator2Vtbl;

    interface IUIAnimationInterpolator2
    {
        CONST_VTBL struct IUIAnimationInterpolator2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationInterpolator2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationInterpolator2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationInterpolator2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationInterpolator2_GetDimension(This,dimension)	\
    ( (This)->lpVtbl -> GetDimension(This,dimension) ) 

#define IUIAnimationInterpolator2_SetInitialValueAndVelocity(This,initialValue,initialVelocity,cDimension)	\
    ( (This)->lpVtbl -> SetInitialValueAndVelocity(This,initialValue,initialVelocity,cDimension) ) 

#define IUIAnimationInterpolator2_SetDuration(This,duration)	\
    ( (This)->lpVtbl -> SetDuration(This,duration) ) 

#define IUIAnimationInterpolator2_GetDuration(This,duration)	\
    ( (This)->lpVtbl -> GetDuration(This,duration) ) 

#define IUIAnimationInterpolator2_GetFinalValue(This,value,cDimension)	\
    ( (This)->lpVtbl -> GetFinalValue(This,value,cDimension) ) 

#define IUIAnimationInterpolator2_InterpolateValue(This,offset,value,cDimension)	\
    ( (This)->lpVtbl -> InterpolateValue(This,offset,value,cDimension) ) 

#define IUIAnimationInterpolator2_InterpolateVelocity(This,offset,velocity,cDimension)	\
    ( (This)->lpVtbl -> InterpolateVelocity(This,offset,velocity,cDimension) ) 

#define IUIAnimationInterpolator2_GetPrimitiveInterpolation(This,interpolation,cDimension)	\
    ( (This)->lpVtbl -> GetPrimitiveInterpolation(This,interpolation,cDimension) ) 

#define IUIAnimationInterpolator2_GetDependencies(This,initialValueDependencies,initialVelocityDependencies,durationDependencies)	\
    ( (This)->lpVtbl -> GetDependencies(This,initialValueDependencies,initialVelocityDependencies,durationDependencies) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationInterpolator2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationTransitionFactory2_INTERFACE_DEFINED__
#define __IUIAnimationTransitionFactory2_INTERFACE_DEFINED__

/* interface IUIAnimationTransitionFactory2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationTransitionFactory2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("937D4916-C1A6-42d5-88D8-30344D6EFE31")
    IUIAnimationTransitionFactory2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE CreateTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationInterpolator2 *interpolator,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationTransitionFactory2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationTransitionFactory2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationTransitionFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationTransitionFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationTransitionFactory2, CreateTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *CreateTransition )( 
            IUIAnimationTransitionFactory2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationInterpolator2 *interpolator,
            /* [annotation][retval][out] */ 
            _Outptr_  IUIAnimationTransition2 **transition);
        
        END_INTERFACE
    } IUIAnimationTransitionFactory2Vtbl;

    interface IUIAnimationTransitionFactory2
    {
        CONST_VTBL struct IUIAnimationTransitionFactory2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationTransitionFactory2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationTransitionFactory2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationTransitionFactory2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationTransitionFactory2_CreateTransition(This,interpolator,transition)	\
    ( (This)->lpVtbl -> CreateTransition(This,interpolator,transition) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationTransitionFactory2_INTERFACE_DEFINED__ */


#ifndef __IUIAnimationStoryboard2_INTERFACE_DEFINED__
#define __IUIAnimationStoryboard2_INTERFACE_DEFINED__

/* interface IUIAnimationStoryboard2 */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IUIAnimationStoryboard2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AE289CD2-12D4-4945-9419-9E41BE034DF2")
    IUIAnimationStoryboard2 : public IUnknown
    {
    public:
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddKeyframeAtOffset( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME existingKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddKeyframeAfterTransition( 
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddTransitionAtKeyframe( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE AddTransitionBetweenKeyframes( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE RepeatBetweenKeyframes( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe,
            /* [annotation][in] */ 
            _In_  DOUBLE cRepetition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_REPEAT_MODE repeatMode,
            /* [annotation][defaultvalue][in] */ 
            _In_opt_  IUIAnimationLoopIterationChangeHandler2 *pIterationChangeHandler = 0,
            /* [annotation][defaultvalue][in] */ 
            _In_  UINT_PTR id = 0,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent = 0) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE HoldVariable( 
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetLongestAcceptableDelay( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetSkipDuration( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS secondsDuration) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Schedule( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_SCHEDULING_RESULT *schedulingResult = 0) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Conclude( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Finish( 
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE Abandon( void) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetTag( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetTag( 
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_STORYBOARD_STATUS *status) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE GetElapsedTime( 
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_SECONDS *elapsedTime) = 0;
        
        virtual /* [annotation] */ 
        _Check_return_
        HRESULT STDMETHODCALLTYPE SetStoryboardEventHandler( 
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationStoryboardEventHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterStatusChangeForNextAnimationEvent = FALSE,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterUpdateForNextAnimationEvent = FALSE) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IUIAnimationStoryboard2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IUIAnimationStoryboard2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IUIAnimationStoryboard2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IUIAnimationStoryboard2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, AddTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddTransition )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, AddKeyframeAtOffset)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddKeyframeAtOffset )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME existingKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS offset,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, AddKeyframeAfterTransition)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddKeyframeAfterTransition )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_KEYFRAME *keyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, AddTransitionAtKeyframe)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddTransitionAtKeyframe )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, AddTransitionBetweenKeyframes)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *AddTransitionBetweenKeyframes )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable,
            /* [annotation][in] */ 
            _In_  IUIAnimationTransition2 *transition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, RepeatBetweenKeyframes)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *RepeatBetweenKeyframes )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME startKeyframe,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_KEYFRAME endKeyframe,
            /* [annotation][in] */ 
            _In_  DOUBLE cRepetition,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_REPEAT_MODE repeatMode,
            /* [annotation][defaultvalue][in] */ 
            _In_opt_  IUIAnimationLoopIterationChangeHandler2 *pIterationChangeHandler,
            /* [annotation][defaultvalue][in] */ 
            _In_  UINT_PTR id,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterForNextAnimationEvent);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, HoldVariable)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *HoldVariable )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  IUIAnimationVariable2 *variable);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, SetLongestAcceptableDelay)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetLongestAcceptableDelay )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS delay);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, SetSkipDuration)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetSkipDuration )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS secondsDuration);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, Schedule)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Schedule )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS timeNow,
            /* [annotation][defaultvalue][out] */ 
            _Out_opt_  UI_ANIMATION_SCHEDULING_RESULT *schedulingResult);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, Conclude)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Conclude )( 
            IUIAnimationStoryboard2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, Finish)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Finish )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][in] */ 
            _In_  UI_ANIMATION_SECONDS completionDeadline);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, Abandon)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *Abandon )( 
            IUIAnimationStoryboard2 * This);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, SetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetTag )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUnknown *object,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, GetTag)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetTag )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **object,
            /* [annotation][out] */ 
            _Out_opt_  UINT32 *id);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, GetStatus)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UI_ANIMATION_STORYBOARD_STATUS *status);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, GetElapsedTime)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *GetElapsedTime )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][out] */ 
            _Out_  UI_ANIMATION_SECONDS *elapsedTime);
        
        DECLSPEC_XFGVIRT(IUIAnimationStoryboard2, SetStoryboardEventHandler)
        /* [annotation] */ 
        _Check_return_
        HRESULT ( STDMETHODCALLTYPE *SetStoryboardEventHandler )( 
            IUIAnimationStoryboard2 * This,
            /* [annotation][unique][in] */ 
            _In_opt_  IUIAnimationStoryboardEventHandler2 *handler,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterStatusChangeForNextAnimationEvent,
            /* [annotation][defaultvalue][in] */ 
            _In_  BOOL fRegisterUpdateForNextAnimationEvent);
        
        END_INTERFACE
    } IUIAnimationStoryboard2Vtbl;

    interface IUIAnimationStoryboard2
    {
        CONST_VTBL struct IUIAnimationStoryboard2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IUIAnimationStoryboard2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IUIAnimationStoryboard2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IUIAnimationStoryboard2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IUIAnimationStoryboard2_AddTransition(This,variable,transition)	\
    ( (This)->lpVtbl -> AddTransition(This,variable,transition) ) 

#define IUIAnimationStoryboard2_AddKeyframeAtOffset(This,existingKeyframe,offset,keyframe)	\
    ( (This)->lpVtbl -> AddKeyframeAtOffset(This,existingKeyframe,offset,keyframe) ) 

#define IUIAnimationStoryboard2_AddKeyframeAfterTransition(This,transition,keyframe)	\
    ( (This)->lpVtbl -> AddKeyframeAfterTransition(This,transition,keyframe) ) 

#define IUIAnimationStoryboard2_AddTransitionAtKeyframe(This,variable,transition,startKeyframe)	\
    ( (This)->lpVtbl -> AddTransitionAtKeyframe(This,variable,transition,startKeyframe) ) 

#define IUIAnimationStoryboard2_AddTransitionBetweenKeyframes(This,variable,transition,startKeyframe,endKeyframe)	\
    ( (This)->lpVtbl -> AddTransitionBetweenKeyframes(This,variable,transition,startKeyframe,endKeyframe) ) 

#define IUIAnimationStoryboard2_RepeatBetweenKeyframes(This,startKeyframe,endKeyframe,cRepetition,repeatMode,pIterationChangeHandler,id,fRegisterForNextAnimationEvent)	\
    ( (This)->lpVtbl -> RepeatBetweenKeyframes(This,startKeyframe,endKeyframe,cRepetition,repeatMode,pIterationChangeHandler,id,fRegisterForNextAnimationEvent) ) 

#define IUIAnimationStoryboard2_HoldVariable(This,variable)	\
    ( (This)->lpVtbl -> HoldVariable(This,variable) ) 

#define IUIAnimationStoryboard2_SetLongestAcceptableDelay(This,delay)	\
    ( (This)->lpVtbl -> SetLongestAcceptableDelay(This,delay) ) 

#define IUIAnimationStoryboard2_SetSkipDuration(This,secondsDuration)	\
    ( (This)->lpVtbl -> SetSkipDuration(This,secondsDuration) ) 

#define IUIAnimationStoryboard2_Schedule(This,timeNow,schedulingResult)	\
    ( (This)->lpVtbl -> Schedule(This,timeNow,schedulingResult) ) 

#define IUIAnimationStoryboard2_Conclude(This)	\
    ( (This)->lpVtbl -> Conclude(This) ) 

#define IUIAnimationStoryboard2_Finish(This,completionDeadline)	\
    ( (This)->lpVtbl -> Finish(This,completionDeadline) ) 

#define IUIAnimationStoryboard2_Abandon(This)	\
    ( (This)->lpVtbl -> Abandon(This) ) 

#define IUIAnimationStoryboard2_SetTag(This,object,id)	\
    ( (This)->lpVtbl -> SetTag(This,object,id) ) 

#define IUIAnimationStoryboard2_GetTag(This,object,id)	\
    ( (This)->lpVtbl -> GetTag(This,object,id) ) 

#define IUIAnimationStoryboard2_GetStatus(This,status)	\
    ( (This)->lpVtbl -> GetStatus(This,status) ) 

#define IUIAnimationStoryboard2_GetElapsedTime(This,elapsedTime)	\
    ( (This)->lpVtbl -> GetElapsedTime(This,elapsedTime) ) 

#define IUIAnimationStoryboard2_SetStoryboardEventHandler(This,handler,fRegisterStatusChangeForNextAnimationEvent,fRegisterUpdateForNextAnimationEvent)	\
    ( (This)->lpVtbl -> SetStoryboardEventHandler(This,handler,fRegisterStatusChangeForNextAnimationEvent,fRegisterUpdateForNextAnimationEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IUIAnimationStoryboard2_INTERFACE_DEFINED__ */



#ifndef __UIAnimation_LIBRARY_DEFINED__
#define __UIAnimation_LIBRARY_DEFINED__

/* library UIAnimation */
/* [helpstring][version][uuid] */ 


EXTERN_C const IID LIBID_UIAnimation;

EXTERN_C const CLSID CLSID_UIAnimationManager;

#ifdef __cplusplus

class DECLSPEC_UUID("4C1FC63A-695C-47E8-A339-1A194BE3D0B8")
UIAnimationManager;
#endif

EXTERN_C const CLSID CLSID_UIAnimationManager2;

#ifdef __cplusplus

class DECLSPEC_UUID("D25D8842-8884-4A4A-B321-091314379BDD")
UIAnimationManager2;
#endif

EXTERN_C const CLSID CLSID_UIAnimationTransitionLibrary;

#ifdef __cplusplus

class DECLSPEC_UUID("1D6322AD-AA85-4EF5-A828-86D71067D145")
UIAnimationTransitionLibrary;
#endif

EXTERN_C const CLSID CLSID_UIAnimationTransitionLibrary2;

#ifdef __cplusplus

class DECLSPEC_UUID("812F944A-C5C8-4CD9-B0A6-B3DA802F228D")
UIAnimationTransitionLibrary2;
#endif

EXTERN_C const CLSID CLSID_UIAnimationTransitionFactory;

#ifdef __cplusplus

class DECLSPEC_UUID("8A9B1CDD-FCD7-419c-8B44-42FD17DB1887")
UIAnimationTransitionFactory;
#endif

EXTERN_C const CLSID CLSID_UIAnimationTransitionFactory2;

#ifdef __cplusplus

class DECLSPEC_UUID("84302F97-7F7B-4040-B190-72AC9D18E420")
UIAnimationTransitionFactory2;
#endif

EXTERN_C const CLSID CLSID_UIAnimationTimer;

#ifdef __cplusplus

class DECLSPEC_UUID("BFCD4A0C-06B6-4384-B768-0DAA792C380E")
UIAnimationTimer;
#endif
#endif /* __UIAnimation_LIBRARY_DEFINED__ */

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


