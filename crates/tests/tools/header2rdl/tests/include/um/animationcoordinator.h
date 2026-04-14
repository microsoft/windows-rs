

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

#ifndef __animationcoordinator_h__
#define __animationcoordinator_h__

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

#ifndef __IInputPaneAnimationCoordinator_FWD_DEFINED__
#define __IInputPaneAnimationCoordinator_FWD_DEFINED__
typedef interface IInputPaneAnimationCoordinator IInputPaneAnimationCoordinator;

#endif 	/* __IInputPaneAnimationCoordinator_FWD_DEFINED__ */


#ifndef __ShowInputPaneAnimationCoordinator_FWD_DEFINED__
#define __ShowInputPaneAnimationCoordinator_FWD_DEFINED__

#ifdef __cplusplus
typedef class ShowInputPaneAnimationCoordinator ShowInputPaneAnimationCoordinator;
#else
typedef struct ShowInputPaneAnimationCoordinator ShowInputPaneAnimationCoordinator;
#endif /* __cplusplus */

#endif 	/* __ShowInputPaneAnimationCoordinator_FWD_DEFINED__ */


#ifndef __HideInputPaneAnimationCoordinator_FWD_DEFINED__
#define __HideInputPaneAnimationCoordinator_FWD_DEFINED__

#ifdef __cplusplus
typedef class HideInputPaneAnimationCoordinator HideInputPaneAnimationCoordinator;
#else
typedef struct HideInputPaneAnimationCoordinator HideInputPaneAnimationCoordinator;
#endif /* __cplusplus */

#endif 	/* __HideInputPaneAnimationCoordinator_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"
#include "dcompanimation.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_animationcoordinator_0000_0000 */
/* [local] */ 

//---------------------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//---------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (NTDDI_VERSION >= NTDDI_WINBLUE)


extern RPC_IF_HANDLE __MIDL_itf_animationcoordinator_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_animationcoordinator_0000_0000_v0_0_s_ifspec;

#ifndef __IInputPaneAnimationCoordinator_INTERFACE_DEFINED__
#define __IInputPaneAnimationCoordinator_INTERFACE_DEFINED__

/* interface IInputPaneAnimationCoordinator */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IInputPaneAnimationCoordinator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2AF16BA9-2DE5-4B75-82D9-01372AFBFFB4")
    IInputPaneAnimationCoordinator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddAnimation( 
            /* [annotation][in] */ 
            _In_  IUnknown *device,
            /* [annotation][in] */ 
            _In_  IDCompositionAnimation *animation) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IInputPaneAnimationCoordinatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IInputPaneAnimationCoordinator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IInputPaneAnimationCoordinator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IInputPaneAnimationCoordinator * This);
        
        DECLSPEC_XFGVIRT(IInputPaneAnimationCoordinator, AddAnimation)
        HRESULT ( STDMETHODCALLTYPE *AddAnimation )( 
            IInputPaneAnimationCoordinator * This,
            /* [annotation][in] */ 
            _In_  IUnknown *device,
            /* [annotation][in] */ 
            _In_  IDCompositionAnimation *animation);
        
        END_INTERFACE
    } IInputPaneAnimationCoordinatorVtbl;

    interface IInputPaneAnimationCoordinator
    {
        CONST_VTBL struct IInputPaneAnimationCoordinatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IInputPaneAnimationCoordinator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IInputPaneAnimationCoordinator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IInputPaneAnimationCoordinator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IInputPaneAnimationCoordinator_AddAnimation(This,device,animation)	\
    ( (This)->lpVtbl -> AddAnimation(This,device,animation) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IInputPaneAnimationCoordinator_INTERFACE_DEFINED__ */



#ifndef __AnimationCoordinatorLib_LIBRARY_DEFINED__
#define __AnimationCoordinatorLib_LIBRARY_DEFINED__

/* library AnimationCoordinatorLib */
/* [version][uuid] */ 


EXTERN_C const IID LIBID_AnimationCoordinatorLib;

EXTERN_C const CLSID CLSID_ShowInputPaneAnimationCoordinator;

#ifdef __cplusplus

class DECLSPEC_UUID("1F046ABF-3202-4DC1-8CB5-3C67617CE1FA")
ShowInputPaneAnimationCoordinator;
#endif

EXTERN_C const CLSID CLSID_HideInputPaneAnimationCoordinator;

#ifdef __cplusplus

class DECLSPEC_UUID("384742B1-2A77-4CB3-8CF8-1136F5E17E59")
HideInputPaneAnimationCoordinator;
#endif
#endif /* __AnimationCoordinatorLib_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_animationcoordinator_0000_0002 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WINBLUE)
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_animationcoordinator_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_animationcoordinator_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


