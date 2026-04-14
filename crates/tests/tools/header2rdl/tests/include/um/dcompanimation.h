

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

#ifndef __dcompanimation_h__
#define __dcompanimation_h__

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

#ifndef __IDCompositionAnimation_FWD_DEFINED__
#define __IDCompositionAnimation_FWD_DEFINED__
typedef interface IDCompositionAnimation IDCompositionAnimation;

#endif 	/* __IDCompositionAnimation_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_dcompanimation_0000_0000 */
/* [local] */ 

//---------------------------------------------------------------------------
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//---------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_dcompanimation_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dcompanimation_0000_0000_v0_0_s_ifspec;

#ifndef __IDCompositionAnimation_INTERFACE_DEFINED__
#define __IDCompositionAnimation_INTERFACE_DEFINED__

/* interface IDCompositionAnimation */
/* [unique][helpstring][uuid][object][local] */ 


EXTERN_C const IID IID_IDCompositionAnimation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CBFD91D9-51B2-45e4-B3DE-D19CCFB863C5")
    IDCompositionAnimation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAbsoluteBeginTime( 
            LARGE_INTEGER beginTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddCubic( 
            double beginOffset,
            float constantCoefficient,
            float linearCoefficient,
            float quadraticCoefficient,
            float cubicCoefficient) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSinusoidal( 
            double beginOffset,
            float bias,
            float amplitude,
            float frequency,
            float phase) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRepeat( 
            double beginOffset,
            double durationToRepeat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End( 
            double endOffset,
            float endValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDCompositionAnimationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDCompositionAnimation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDCompositionAnimation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDCompositionAnimation * This);
        
        DECLSPEC_XFGVIRT(IDCompositionAnimation, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IDCompositionAnimation * This);
        
        DECLSPEC_XFGVIRT(IDCompositionAnimation, SetAbsoluteBeginTime)
        HRESULT ( STDMETHODCALLTYPE *SetAbsoluteBeginTime )( 
            IDCompositionAnimation * This,
            LARGE_INTEGER beginTime);
        
        DECLSPEC_XFGVIRT(IDCompositionAnimation, AddCubic)
        HRESULT ( STDMETHODCALLTYPE *AddCubic )( 
            IDCompositionAnimation * This,
            double beginOffset,
            float constantCoefficient,
            float linearCoefficient,
            float quadraticCoefficient,
            float cubicCoefficient);
        
        DECLSPEC_XFGVIRT(IDCompositionAnimation, AddSinusoidal)
        HRESULT ( STDMETHODCALLTYPE *AddSinusoidal )( 
            IDCompositionAnimation * This,
            double beginOffset,
            float bias,
            float amplitude,
            float frequency,
            float phase);
        
        DECLSPEC_XFGVIRT(IDCompositionAnimation, AddRepeat)
        HRESULT ( STDMETHODCALLTYPE *AddRepeat )( 
            IDCompositionAnimation * This,
            double beginOffset,
            double durationToRepeat);
        
        DECLSPEC_XFGVIRT(IDCompositionAnimation, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            IDCompositionAnimation * This,
            double endOffset,
            float endValue);
        
        END_INTERFACE
    } IDCompositionAnimationVtbl;

    interface IDCompositionAnimation
    {
        CONST_VTBL struct IDCompositionAnimationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDCompositionAnimation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDCompositionAnimation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDCompositionAnimation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDCompositionAnimation_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IDCompositionAnimation_SetAbsoluteBeginTime(This,beginTime)	\
    ( (This)->lpVtbl -> SetAbsoluteBeginTime(This,beginTime) ) 

#define IDCompositionAnimation_AddCubic(This,beginOffset,constantCoefficient,linearCoefficient,quadraticCoefficient,cubicCoefficient)	\
    ( (This)->lpVtbl -> AddCubic(This,beginOffset,constantCoefficient,linearCoefficient,quadraticCoefficient,cubicCoefficient) ) 

#define IDCompositionAnimation_AddSinusoidal(This,beginOffset,bias,amplitude,frequency,phase)	\
    ( (This)->lpVtbl -> AddSinusoidal(This,beginOffset,bias,amplitude,frequency,phase) ) 

#define IDCompositionAnimation_AddRepeat(This,beginOffset,durationToRepeat)	\
    ( (This)->lpVtbl -> AddRepeat(This,beginOffset,durationToRepeat) ) 

#define IDCompositionAnimation_End(This,endOffset,endValue)	\
    ( (This)->lpVtbl -> End(This,endOffset,endValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDCompositionAnimation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_dcompanimation_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_dcompanimation_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_dcompanimation_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


