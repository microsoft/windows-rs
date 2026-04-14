

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

#ifndef __Presentation_h__
#define __Presentation_h__

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

#ifndef __IPresentationBuffer_FWD_DEFINED__
#define __IPresentationBuffer_FWD_DEFINED__
typedef interface IPresentationBuffer IPresentationBuffer;

#endif 	/* __IPresentationBuffer_FWD_DEFINED__ */


#ifndef __IPresentationContent_FWD_DEFINED__
#define __IPresentationContent_FWD_DEFINED__
typedef interface IPresentationContent IPresentationContent;

#endif 	/* __IPresentationContent_FWD_DEFINED__ */


#ifndef __IPresentationSurface_FWD_DEFINED__
#define __IPresentationSurface_FWD_DEFINED__
typedef interface IPresentationSurface IPresentationSurface;

#endif 	/* __IPresentationSurface_FWD_DEFINED__ */


#ifndef __IPresentationSurface2_FWD_DEFINED__
#define __IPresentationSurface2_FWD_DEFINED__
typedef interface IPresentationSurface2 IPresentationSurface2;

#endif 	/* __IPresentationSurface2_FWD_DEFINED__ */


#ifndef __IPresentStatistics_FWD_DEFINED__
#define __IPresentStatistics_FWD_DEFINED__
typedef interface IPresentStatistics IPresentStatistics;

#endif 	/* __IPresentStatistics_FWD_DEFINED__ */


#ifndef __IPresentationManager_FWD_DEFINED__
#define __IPresentationManager_FWD_DEFINED__
typedef interface IPresentationManager IPresentationManager;

#endif 	/* __IPresentationManager_FWD_DEFINED__ */


#ifndef __IPresentationFactory_FWD_DEFINED__
#define __IPresentationFactory_FWD_DEFINED__
typedef interface IPresentationFactory IPresentationFactory;

#endif 	/* __IPresentationFactory_FWD_DEFINED__ */


#ifndef __IPresentationFactory_SupportHdrAware_FWD_DEFINED__
#define __IPresentationFactory_SupportHdrAware_FWD_DEFINED__
typedef interface IPresentationFactory_SupportHdrAware IPresentationFactory_SupportHdrAware;

#endif 	/* __IPresentationFactory_SupportHdrAware_FWD_DEFINED__ */


#ifndef __IPresentStatusPresentStatistics_FWD_DEFINED__
#define __IPresentStatusPresentStatistics_FWD_DEFINED__
typedef interface IPresentStatusPresentStatistics IPresentStatusPresentStatistics;

#endif 	/* __IPresentStatusPresentStatistics_FWD_DEFINED__ */


#ifndef __ICompositionFramePresentStatistics_FWD_DEFINED__
#define __ICompositionFramePresentStatistics_FWD_DEFINED__
typedef interface ICompositionFramePresentStatistics ICompositionFramePresentStatistics;

#endif 	/* __ICompositionFramePresentStatistics_FWD_DEFINED__ */


#ifndef __IIndependentFlipFramePresentStatistics_FWD_DEFINED__
#define __IIndependentFlipFramePresentStatistics_FWD_DEFINED__
typedef interface IIndependentFlipFramePresentStatistics IIndependentFlipFramePresentStatistics;

#endif 	/* __IIndependentFlipFramePresentStatistics_FWD_DEFINED__ */


/* header files for imported files */
#include "PresentationTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


#ifndef __IPresentationBuffer_INTERFACE_DEFINED__
#define __IPresentationBuffer_INTERFACE_DEFINED__

/* interface IPresentationBuffer */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2E217D3A-5ABB-4138-9A13-A775593C89CA")
    IPresentationBuffer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAvailableEvent( 
            /* [retval][out] */ HANDLE *availableEventHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAvailable( 
            /* [retval][out] */ boolean *isAvailable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationBuffer * This);
        
        DECLSPEC_XFGVIRT(IPresentationBuffer, GetAvailableEvent)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableEvent )( 
            IPresentationBuffer * This,
            /* [retval][out] */ HANDLE *availableEventHandle);
        
        DECLSPEC_XFGVIRT(IPresentationBuffer, IsAvailable)
        HRESULT ( STDMETHODCALLTYPE *IsAvailable )( 
            IPresentationBuffer * This,
            /* [retval][out] */ boolean *isAvailable);
        
        END_INTERFACE
    } IPresentationBufferVtbl;

    interface IPresentationBuffer
    {
        CONST_VTBL struct IPresentationBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentationBuffer_GetAvailableEvent(This,availableEventHandle)	\
    ( (This)->lpVtbl -> GetAvailableEvent(This,availableEventHandle) ) 

#define IPresentationBuffer_IsAvailable(This,isAvailable)	\
    ( (This)->lpVtbl -> IsAvailable(This,isAvailable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationBuffer_INTERFACE_DEFINED__ */


#ifndef __IPresentationContent_INTERFACE_DEFINED__
#define __IPresentationContent_INTERFACE_DEFINED__

/* interface IPresentationContent */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5668BB79-3D8E-415C-B215-F38020F2D252")
    IPresentationContent : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE SetTag( 
            /* [in] */ UINT_PTR tag) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationContent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationContent * This);
        
        DECLSPEC_XFGVIRT(IPresentationContent, SetTag)
        void ( STDMETHODCALLTYPE *SetTag )( 
            IPresentationContent * This,
            /* [in] */ UINT_PTR tag);
        
        END_INTERFACE
    } IPresentationContentVtbl;

    interface IPresentationContent
    {
        CONST_VTBL struct IPresentationContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentationContent_SetTag(This,tag)	\
    ( (This)->lpVtbl -> SetTag(This,tag) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationContent_INTERFACE_DEFINED__ */


#ifndef __IPresentationSurface_INTERFACE_DEFINED__
#define __IPresentationSurface_INTERFACE_DEFINED__

/* interface IPresentationSurface */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationSurface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("956710FB-EA40-4EBA-A3EB-4375A0EB4EDC")
    IPresentationSurface : public IPresentationContent
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBuffer( 
            /* [in] */ IPresentationBuffer *presentationBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColorSpace( 
            /* [in] */ DXGI_COLOR_SPACE_TYPE colorSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAlphaMode( 
            /* [in] */ DXGI_ALPHA_MODE alphaMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSourceRect( 
            /* [in] */ const RECT *sourceRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransform( 
            /* [in] */ PresentationTransform *transform) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestrictToOutput( 
            /* [in] */ IUnknown *output) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDisableReadback( 
            /* [in] */ boolean value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLetterboxingMargins( 
            /* [in] */ float leftLetterboxSize,
            /* [in] */ float topLetterboxSize,
            /* [in] */ float rightLetterboxSize,
            /* [in] */ float bottomLetterboxSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationSurfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationSurface * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationSurface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationSurface * This);
        
        DECLSPEC_XFGVIRT(IPresentationContent, SetTag)
        void ( STDMETHODCALLTYPE *SetTag )( 
            IPresentationSurface * This,
            /* [in] */ UINT_PTR tag);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetBuffer)
        HRESULT ( STDMETHODCALLTYPE *SetBuffer )( 
            IPresentationSurface * This,
            /* [in] */ IPresentationBuffer *presentationBuffer);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetColorSpace)
        HRESULT ( STDMETHODCALLTYPE *SetColorSpace )( 
            IPresentationSurface * This,
            /* [in] */ DXGI_COLOR_SPACE_TYPE colorSpace);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetAlphaMode)
        HRESULT ( STDMETHODCALLTYPE *SetAlphaMode )( 
            IPresentationSurface * This,
            /* [in] */ DXGI_ALPHA_MODE alphaMode);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetSourceRect)
        HRESULT ( STDMETHODCALLTYPE *SetSourceRect )( 
            IPresentationSurface * This,
            /* [in] */ const RECT *sourceRect);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetTransform)
        HRESULT ( STDMETHODCALLTYPE *SetTransform )( 
            IPresentationSurface * This,
            /* [in] */ PresentationTransform *transform);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, RestrictToOutput)
        HRESULT ( STDMETHODCALLTYPE *RestrictToOutput )( 
            IPresentationSurface * This,
            /* [in] */ IUnknown *output);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetDisableReadback)
        HRESULT ( STDMETHODCALLTYPE *SetDisableReadback )( 
            IPresentationSurface * This,
            /* [in] */ boolean value);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetLetterboxingMargins)
        HRESULT ( STDMETHODCALLTYPE *SetLetterboxingMargins )( 
            IPresentationSurface * This,
            /* [in] */ float leftLetterboxSize,
            /* [in] */ float topLetterboxSize,
            /* [in] */ float rightLetterboxSize,
            /* [in] */ float bottomLetterboxSize);
        
        END_INTERFACE
    } IPresentationSurfaceVtbl;

    interface IPresentationSurface
    {
        CONST_VTBL struct IPresentationSurfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationSurface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationSurface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationSurface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentationSurface_SetTag(This,tag)	\
    ( (This)->lpVtbl -> SetTag(This,tag) ) 


#define IPresentationSurface_SetBuffer(This,presentationBuffer)	\
    ( (This)->lpVtbl -> SetBuffer(This,presentationBuffer) ) 

#define IPresentationSurface_SetColorSpace(This,colorSpace)	\
    ( (This)->lpVtbl -> SetColorSpace(This,colorSpace) ) 

#define IPresentationSurface_SetAlphaMode(This,alphaMode)	\
    ( (This)->lpVtbl -> SetAlphaMode(This,alphaMode) ) 

#define IPresentationSurface_SetSourceRect(This,sourceRect)	\
    ( (This)->lpVtbl -> SetSourceRect(This,sourceRect) ) 

#define IPresentationSurface_SetTransform(This,transform)	\
    ( (This)->lpVtbl -> SetTransform(This,transform) ) 

#define IPresentationSurface_RestrictToOutput(This,output)	\
    ( (This)->lpVtbl -> RestrictToOutput(This,output) ) 

#define IPresentationSurface_SetDisableReadback(This,value)	\
    ( (This)->lpVtbl -> SetDisableReadback(This,value) ) 

#define IPresentationSurface_SetLetterboxingMargins(This,leftLetterboxSize,topLetterboxSize,rightLetterboxSize,bottomLetterboxSize)	\
    ( (This)->lpVtbl -> SetLetterboxingMargins(This,leftLetterboxSize,topLetterboxSize,rightLetterboxSize,bottomLetterboxSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationSurface_INTERFACE_DEFINED__ */


#ifndef __IPresentationSurface2_INTERFACE_DEFINED__
#define __IPresentationSurface2_INTERFACE_DEFINED__

/* interface IPresentationSurface2 */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationSurface2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("95609569-C5F0-47F9-8804-5345F2E2767E")
    IPresentationSurface2 : public IPresentationSurface
    {
    public:
        virtual void STDMETHODCALLTYPE SetIsHdrContent( 
            /* [in] */ boolean isHdrContent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationSurface2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationSurface2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationSurface2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationSurface2 * This);
        
        DECLSPEC_XFGVIRT(IPresentationContent, SetTag)
        void ( STDMETHODCALLTYPE *SetTag )( 
            IPresentationSurface2 * This,
            /* [in] */ UINT_PTR tag);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetBuffer)
        HRESULT ( STDMETHODCALLTYPE *SetBuffer )( 
            IPresentationSurface2 * This,
            /* [in] */ IPresentationBuffer *presentationBuffer);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetColorSpace)
        HRESULT ( STDMETHODCALLTYPE *SetColorSpace )( 
            IPresentationSurface2 * This,
            /* [in] */ DXGI_COLOR_SPACE_TYPE colorSpace);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetAlphaMode)
        HRESULT ( STDMETHODCALLTYPE *SetAlphaMode )( 
            IPresentationSurface2 * This,
            /* [in] */ DXGI_ALPHA_MODE alphaMode);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetSourceRect)
        HRESULT ( STDMETHODCALLTYPE *SetSourceRect )( 
            IPresentationSurface2 * This,
            /* [in] */ const RECT *sourceRect);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetTransform)
        HRESULT ( STDMETHODCALLTYPE *SetTransform )( 
            IPresentationSurface2 * This,
            /* [in] */ PresentationTransform *transform);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, RestrictToOutput)
        HRESULT ( STDMETHODCALLTYPE *RestrictToOutput )( 
            IPresentationSurface2 * This,
            /* [in] */ IUnknown *output);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetDisableReadback)
        HRESULT ( STDMETHODCALLTYPE *SetDisableReadback )( 
            IPresentationSurface2 * This,
            /* [in] */ boolean value);
        
        DECLSPEC_XFGVIRT(IPresentationSurface, SetLetterboxingMargins)
        HRESULT ( STDMETHODCALLTYPE *SetLetterboxingMargins )( 
            IPresentationSurface2 * This,
            /* [in] */ float leftLetterboxSize,
            /* [in] */ float topLetterboxSize,
            /* [in] */ float rightLetterboxSize,
            /* [in] */ float bottomLetterboxSize);
        
        DECLSPEC_XFGVIRT(IPresentationSurface2, SetIsHdrContent)
        void ( STDMETHODCALLTYPE *SetIsHdrContent )( 
            IPresentationSurface2 * This,
            /* [in] */ boolean isHdrContent);
        
        END_INTERFACE
    } IPresentationSurface2Vtbl;

    interface IPresentationSurface2
    {
        CONST_VTBL struct IPresentationSurface2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationSurface2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationSurface2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationSurface2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentationSurface2_SetTag(This,tag)	\
    ( (This)->lpVtbl -> SetTag(This,tag) ) 


#define IPresentationSurface2_SetBuffer(This,presentationBuffer)	\
    ( (This)->lpVtbl -> SetBuffer(This,presentationBuffer) ) 

#define IPresentationSurface2_SetColorSpace(This,colorSpace)	\
    ( (This)->lpVtbl -> SetColorSpace(This,colorSpace) ) 

#define IPresentationSurface2_SetAlphaMode(This,alphaMode)	\
    ( (This)->lpVtbl -> SetAlphaMode(This,alphaMode) ) 

#define IPresentationSurface2_SetSourceRect(This,sourceRect)	\
    ( (This)->lpVtbl -> SetSourceRect(This,sourceRect) ) 

#define IPresentationSurface2_SetTransform(This,transform)	\
    ( (This)->lpVtbl -> SetTransform(This,transform) ) 

#define IPresentationSurface2_RestrictToOutput(This,output)	\
    ( (This)->lpVtbl -> RestrictToOutput(This,output) ) 

#define IPresentationSurface2_SetDisableReadback(This,value)	\
    ( (This)->lpVtbl -> SetDisableReadback(This,value) ) 

#define IPresentationSurface2_SetLetterboxingMargins(This,leftLetterboxSize,topLetterboxSize,rightLetterboxSize,bottomLetterboxSize)	\
    ( (This)->lpVtbl -> SetLetterboxingMargins(This,leftLetterboxSize,topLetterboxSize,rightLetterboxSize,bottomLetterboxSize) ) 


#define IPresentationSurface2_SetIsHdrContent(This,isHdrContent)	\
    ( (This)->lpVtbl -> SetIsHdrContent(This,isHdrContent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationSurface2_INTERFACE_DEFINED__ */


#ifndef __IPresentStatistics_INTERFACE_DEFINED__
#define __IPresentStatistics_INTERFACE_DEFINED__

/* interface IPresentStatistics */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentStatistics;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B44B8BDA-7282-495D-9DD7-CEADD8B4BB86")
    IPresentStatistics : public IUnknown
    {
    public:
        virtual UINT64 STDMETHODCALLTYPE GetPresentId( void) = 0;
        
        virtual PresentStatisticsKind STDMETHODCALLTYPE GetKind( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentStatisticsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentStatistics * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetPresentId)
        UINT64 ( STDMETHODCALLTYPE *GetPresentId )( 
            IPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetKind)
        PresentStatisticsKind ( STDMETHODCALLTYPE *GetKind )( 
            IPresentStatistics * This);
        
        END_INTERFACE
    } IPresentStatisticsVtbl;

    interface IPresentStatistics
    {
        CONST_VTBL struct IPresentStatisticsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentStatistics_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentStatistics_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentStatistics_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentStatistics_GetPresentId(This)	\
    ( (This)->lpVtbl -> GetPresentId(This) ) 

#define IPresentStatistics_GetKind(This)	\
    ( (This)->lpVtbl -> GetKind(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentStatistics_INTERFACE_DEFINED__ */


#ifndef __IPresentationManager_INTERFACE_DEFINED__
#define __IPresentationManager_INTERFACE_DEFINED__

/* interface IPresentationManager */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FB562F82-6292-470A-88B1-843661E7F20C")
    IPresentationManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddBufferFromResource( 
            /* [in] */ IUnknown *resource,
            /* [retval][out] */ IPresentationBuffer **presentationBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePresentationSurface( 
            /* [in] */ HANDLE compositionSurfaceHandle,
            /* [retval][out] */ IPresentationSurface **presentationSurface) = 0;
        
        virtual UINT64 STDMETHODCALLTYPE GetNextPresentId( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTargetTime( 
            /* [in] */ SystemInterruptTime targetTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreferredPresentDuration( 
            /* [in] */ SystemInterruptTime preferredDuration,
            /* [in] */ SystemInterruptTime deviationTolerance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ForceVSyncInterrupt( 
            /* [in] */ boolean forceVsyncInterrupt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Present( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentRetiringFence( 
            /* [in] */ REFIID riid,
            /* [retval][out] */ void **fence) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelPresentsFrom( 
            /* [in] */ UINT64 presentIdToCancelFrom) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLostEvent( 
            /* [retval][out] */ HANDLE *lostEventHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentStatisticsAvailableEvent( 
            /* [retval][out] */ HANDLE *presentStatisticsAvailableEventHandle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnablePresentStatisticsKind( 
            /* [in] */ PresentStatisticsKind presentStatisticsKind,
            /* [in] */ boolean enabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNextPresentStatistics( 
            /* [retval][out] */ IPresentStatistics **nextPresentStatistics) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationManager * This);
        
        DECLSPEC_XFGVIRT(IPresentationManager, AddBufferFromResource)
        HRESULT ( STDMETHODCALLTYPE *AddBufferFromResource )( 
            IPresentationManager * This,
            /* [in] */ IUnknown *resource,
            /* [retval][out] */ IPresentationBuffer **presentationBuffer);
        
        DECLSPEC_XFGVIRT(IPresentationManager, CreatePresentationSurface)
        HRESULT ( STDMETHODCALLTYPE *CreatePresentationSurface )( 
            IPresentationManager * This,
            /* [in] */ HANDLE compositionSurfaceHandle,
            /* [retval][out] */ IPresentationSurface **presentationSurface);
        
        DECLSPEC_XFGVIRT(IPresentationManager, GetNextPresentId)
        UINT64 ( STDMETHODCALLTYPE *GetNextPresentId )( 
            IPresentationManager * This);
        
        DECLSPEC_XFGVIRT(IPresentationManager, SetTargetTime)
        HRESULT ( STDMETHODCALLTYPE *SetTargetTime )( 
            IPresentationManager * This,
            /* [in] */ SystemInterruptTime targetTime);
        
        DECLSPEC_XFGVIRT(IPresentationManager, SetPreferredPresentDuration)
        HRESULT ( STDMETHODCALLTYPE *SetPreferredPresentDuration )( 
            IPresentationManager * This,
            /* [in] */ SystemInterruptTime preferredDuration,
            /* [in] */ SystemInterruptTime deviationTolerance);
        
        DECLSPEC_XFGVIRT(IPresentationManager, ForceVSyncInterrupt)
        HRESULT ( STDMETHODCALLTYPE *ForceVSyncInterrupt )( 
            IPresentationManager * This,
            /* [in] */ boolean forceVsyncInterrupt);
        
        DECLSPEC_XFGVIRT(IPresentationManager, Present)
        HRESULT ( STDMETHODCALLTYPE *Present )( 
            IPresentationManager * This);
        
        DECLSPEC_XFGVIRT(IPresentationManager, GetPresentRetiringFence)
        HRESULT ( STDMETHODCALLTYPE *GetPresentRetiringFence )( 
            IPresentationManager * This,
            /* [in] */ REFIID riid,
            /* [retval][out] */ void **fence);
        
        DECLSPEC_XFGVIRT(IPresentationManager, CancelPresentsFrom)
        HRESULT ( STDMETHODCALLTYPE *CancelPresentsFrom )( 
            IPresentationManager * This,
            /* [in] */ UINT64 presentIdToCancelFrom);
        
        DECLSPEC_XFGVIRT(IPresentationManager, GetLostEvent)
        HRESULT ( STDMETHODCALLTYPE *GetLostEvent )( 
            IPresentationManager * This,
            /* [retval][out] */ HANDLE *lostEventHandle);
        
        DECLSPEC_XFGVIRT(IPresentationManager, GetPresentStatisticsAvailableEvent)
        HRESULT ( STDMETHODCALLTYPE *GetPresentStatisticsAvailableEvent )( 
            IPresentationManager * This,
            /* [retval][out] */ HANDLE *presentStatisticsAvailableEventHandle);
        
        DECLSPEC_XFGVIRT(IPresentationManager, EnablePresentStatisticsKind)
        HRESULT ( STDMETHODCALLTYPE *EnablePresentStatisticsKind )( 
            IPresentationManager * This,
            /* [in] */ PresentStatisticsKind presentStatisticsKind,
            /* [in] */ boolean enabled);
        
        DECLSPEC_XFGVIRT(IPresentationManager, GetNextPresentStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetNextPresentStatistics )( 
            IPresentationManager * This,
            /* [retval][out] */ IPresentStatistics **nextPresentStatistics);
        
        END_INTERFACE
    } IPresentationManagerVtbl;

    interface IPresentationManager
    {
        CONST_VTBL struct IPresentationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentationManager_AddBufferFromResource(This,resource,presentationBuffer)	\
    ( (This)->lpVtbl -> AddBufferFromResource(This,resource,presentationBuffer) ) 

#define IPresentationManager_CreatePresentationSurface(This,compositionSurfaceHandle,presentationSurface)	\
    ( (This)->lpVtbl -> CreatePresentationSurface(This,compositionSurfaceHandle,presentationSurface) ) 

#define IPresentationManager_GetNextPresentId(This)	\
    ( (This)->lpVtbl -> GetNextPresentId(This) ) 

#define IPresentationManager_SetTargetTime(This,targetTime)	\
    ( (This)->lpVtbl -> SetTargetTime(This,targetTime) ) 

#define IPresentationManager_SetPreferredPresentDuration(This,preferredDuration,deviationTolerance)	\
    ( (This)->lpVtbl -> SetPreferredPresentDuration(This,preferredDuration,deviationTolerance) ) 

#define IPresentationManager_ForceVSyncInterrupt(This,forceVsyncInterrupt)	\
    ( (This)->lpVtbl -> ForceVSyncInterrupt(This,forceVsyncInterrupt) ) 

#define IPresentationManager_Present(This)	\
    ( (This)->lpVtbl -> Present(This) ) 

#define IPresentationManager_GetPresentRetiringFence(This,riid,fence)	\
    ( (This)->lpVtbl -> GetPresentRetiringFence(This,riid,fence) ) 

#define IPresentationManager_CancelPresentsFrom(This,presentIdToCancelFrom)	\
    ( (This)->lpVtbl -> CancelPresentsFrom(This,presentIdToCancelFrom) ) 

#define IPresentationManager_GetLostEvent(This,lostEventHandle)	\
    ( (This)->lpVtbl -> GetLostEvent(This,lostEventHandle) ) 

#define IPresentationManager_GetPresentStatisticsAvailableEvent(This,presentStatisticsAvailableEventHandle)	\
    ( (This)->lpVtbl -> GetPresentStatisticsAvailableEvent(This,presentStatisticsAvailableEventHandle) ) 

#define IPresentationManager_EnablePresentStatisticsKind(This,presentStatisticsKind,enabled)	\
    ( (This)->lpVtbl -> EnablePresentStatisticsKind(This,presentStatisticsKind,enabled) ) 

#define IPresentationManager_GetNextPresentStatistics(This,nextPresentStatistics)	\
    ( (This)->lpVtbl -> GetNextPresentStatistics(This,nextPresentStatistics) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationManager_INTERFACE_DEFINED__ */


#ifndef __IPresentationFactory_INTERFACE_DEFINED__
#define __IPresentationFactory_INTERFACE_DEFINED__

/* interface IPresentationFactory */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8FB37B58-1D74-4F64-A49C-1F97A80A2EC0")
    IPresentationFactory : public IUnknown
    {
    public:
        virtual boolean STDMETHODCALLTYPE IsPresentationSupported( void) = 0;
        
        virtual boolean STDMETHODCALLTYPE IsPresentationSupportedWithIndependentFlip( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePresentationManager( 
            /* [retval][out] */ IPresentationManager **ppPresentationManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationFactory * This);
        
        DECLSPEC_XFGVIRT(IPresentationFactory, IsPresentationSupported)
        boolean ( STDMETHODCALLTYPE *IsPresentationSupported )( 
            IPresentationFactory * This);
        
        DECLSPEC_XFGVIRT(IPresentationFactory, IsPresentationSupportedWithIndependentFlip)
        boolean ( STDMETHODCALLTYPE *IsPresentationSupportedWithIndependentFlip )( 
            IPresentationFactory * This);
        
        DECLSPEC_XFGVIRT(IPresentationFactory, CreatePresentationManager)
        HRESULT ( STDMETHODCALLTYPE *CreatePresentationManager )( 
            IPresentationFactory * This,
            /* [retval][out] */ IPresentationManager **ppPresentationManager);
        
        END_INTERFACE
    } IPresentationFactoryVtbl;

    interface IPresentationFactory
    {
        CONST_VTBL struct IPresentationFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentationFactory_IsPresentationSupported(This)	\
    ( (This)->lpVtbl -> IsPresentationSupported(This) ) 

#define IPresentationFactory_IsPresentationSupportedWithIndependentFlip(This)	\
    ( (This)->lpVtbl -> IsPresentationSupportedWithIndependentFlip(This) ) 

#define IPresentationFactory_CreatePresentationManager(This,ppPresentationManager)	\
    ( (This)->lpVtbl -> CreatePresentationManager(This,ppPresentationManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationFactory_INTERFACE_DEFINED__ */


#ifndef __IPresentationFactory_SupportHdrAware_INTERFACE_DEFINED__
#define __IPresentationFactory_SupportHdrAware_INTERFACE_DEFINED__

/* interface IPresentationFactory_SupportHdrAware */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentationFactory_SupportHdrAware;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2BD0B885-A16F-4BD9-A59A-D073E069D416")
    IPresentationFactory_SupportHdrAware : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentationFactory_SupportHdrAwareVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentationFactory_SupportHdrAware * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentationFactory_SupportHdrAware * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentationFactory_SupportHdrAware * This);
        
        END_INTERFACE
    } IPresentationFactory_SupportHdrAwareVtbl;

    interface IPresentationFactory_SupportHdrAware
    {
        CONST_VTBL struct IPresentationFactory_SupportHdrAwareVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentationFactory_SupportHdrAware_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentationFactory_SupportHdrAware_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentationFactory_SupportHdrAware_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentationFactory_SupportHdrAware_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_Presentation_0000_0008 */
/* [local] */ 

typedef 
enum PresentStatus
    {
        PresentStatus_Queued	= 0,
        PresentStatus_Skipped	= 1,
        PresentStatus_Canceled	= 2
    } 	PresentStatus;



extern RPC_IF_HANDLE __MIDL_itf_Presentation_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_Presentation_0000_0008_v0_0_s_ifspec;

#ifndef __IPresentStatusPresentStatistics_INTERFACE_DEFINED__
#define __IPresentStatusPresentStatistics_INTERFACE_DEFINED__

/* interface IPresentStatusPresentStatistics */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IPresentStatusPresentStatistics;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C9ED2A41-79CB-435E-964E-C8553055420C")
    IPresentStatusPresentStatistics : public IPresentStatistics
    {
    public:
        virtual CompositionFrameId STDMETHODCALLTYPE GetCompositionFrameId( void) = 0;
        
        virtual PresentStatus STDMETHODCALLTYPE GetPresentStatus( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPresentStatusPresentStatisticsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPresentStatusPresentStatistics * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPresentStatusPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPresentStatusPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetPresentId)
        UINT64 ( STDMETHODCALLTYPE *GetPresentId )( 
            IPresentStatusPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetKind)
        PresentStatisticsKind ( STDMETHODCALLTYPE *GetKind )( 
            IPresentStatusPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatusPresentStatistics, GetCompositionFrameId)
        CompositionFrameId ( STDMETHODCALLTYPE *GetCompositionFrameId )( 
            IPresentStatusPresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatusPresentStatistics, GetPresentStatus)
        PresentStatus ( STDMETHODCALLTYPE *GetPresentStatus )( 
            IPresentStatusPresentStatistics * This);
        
        END_INTERFACE
    } IPresentStatusPresentStatisticsVtbl;

    interface IPresentStatusPresentStatistics
    {
        CONST_VTBL struct IPresentStatusPresentStatisticsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPresentStatusPresentStatistics_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPresentStatusPresentStatistics_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPresentStatusPresentStatistics_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPresentStatusPresentStatistics_GetPresentId(This)	\
    ( (This)->lpVtbl -> GetPresentId(This) ) 

#define IPresentStatusPresentStatistics_GetKind(This)	\
    ( (This)->lpVtbl -> GetKind(This) ) 


#define IPresentStatusPresentStatistics_GetCompositionFrameId(This)	\
    ( (This)->lpVtbl -> GetCompositionFrameId(This) ) 

#define IPresentStatusPresentStatistics_GetPresentStatus(This)	\
    ( (This)->lpVtbl -> GetPresentStatus(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPresentStatusPresentStatistics_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_Presentation_0000_0009 */
/* [local] */ 

typedef 
enum CompositionFrameInstanceKind
    {
        CompositionFrameInstanceKind_ComposedOnScreen	= 0,
        CompositionFrameInstanceKind_ScanoutOnScreen	= 1,
        CompositionFrameInstanceKind_ComposedToIntermediate	= 2
    } 	CompositionFrameInstanceKind;

typedef struct CompositionFrameDisplayInstance
    {
    LUID displayAdapterLUID;
    UINT displayVidPnSourceId;
    UINT displayUniqueId;
    LUID renderAdapterLUID;
    CompositionFrameInstanceKind instanceKind;
    PresentationTransform finalTransform;
    boolean requiredCrossAdapterCopy;
    DXGI_COLOR_SPACE_TYPE colorSpace;
    } 	CompositionFrameDisplayInstance;



extern RPC_IF_HANDLE __MIDL_itf_Presentation_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_Presentation_0000_0009_v0_0_s_ifspec;

#ifndef __ICompositionFramePresentStatistics_INTERFACE_DEFINED__
#define __ICompositionFramePresentStatistics_INTERFACE_DEFINED__

/* interface ICompositionFramePresentStatistics */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_ICompositionFramePresentStatistics;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("AB41D127-C101-4C0A-911D-F9F2E9D08E64")
    ICompositionFramePresentStatistics : public IPresentStatistics
    {
    public:
        virtual UINT_PTR STDMETHODCALLTYPE GetContentTag( void) = 0;
        
        virtual CompositionFrameId STDMETHODCALLTYPE GetCompositionFrameId( void) = 0;
        
        virtual void STDMETHODCALLTYPE GetDisplayInstanceArray( 
            /* [out] */ UINT *displayInstanceArrayCount,
            /* [out] */ const CompositionFrameDisplayInstance **displayInstanceArray) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICompositionFramePresentStatisticsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICompositionFramePresentStatistics * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICompositionFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICompositionFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetPresentId)
        UINT64 ( STDMETHODCALLTYPE *GetPresentId )( 
            ICompositionFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetKind)
        PresentStatisticsKind ( STDMETHODCALLTYPE *GetKind )( 
            ICompositionFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(ICompositionFramePresentStatistics, GetContentTag)
        UINT_PTR ( STDMETHODCALLTYPE *GetContentTag )( 
            ICompositionFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(ICompositionFramePresentStatistics, GetCompositionFrameId)
        CompositionFrameId ( STDMETHODCALLTYPE *GetCompositionFrameId )( 
            ICompositionFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(ICompositionFramePresentStatistics, GetDisplayInstanceArray)
        void ( STDMETHODCALLTYPE *GetDisplayInstanceArray )( 
            ICompositionFramePresentStatistics * This,
            /* [out] */ UINT *displayInstanceArrayCount,
            /* [out] */ const CompositionFrameDisplayInstance **displayInstanceArray);
        
        END_INTERFACE
    } ICompositionFramePresentStatisticsVtbl;

    interface ICompositionFramePresentStatistics
    {
        CONST_VTBL struct ICompositionFramePresentStatisticsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICompositionFramePresentStatistics_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICompositionFramePresentStatistics_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICompositionFramePresentStatistics_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICompositionFramePresentStatistics_GetPresentId(This)	\
    ( (This)->lpVtbl -> GetPresentId(This) ) 

#define ICompositionFramePresentStatistics_GetKind(This)	\
    ( (This)->lpVtbl -> GetKind(This) ) 


#define ICompositionFramePresentStatistics_GetContentTag(This)	\
    ( (This)->lpVtbl -> GetContentTag(This) ) 

#define ICompositionFramePresentStatistics_GetCompositionFrameId(This)	\
    ( (This)->lpVtbl -> GetCompositionFrameId(This) ) 

#define ICompositionFramePresentStatistics_GetDisplayInstanceArray(This,displayInstanceArrayCount,displayInstanceArray)	\
    ( (This)->lpVtbl -> GetDisplayInstanceArray(This,displayInstanceArrayCount,displayInstanceArray) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICompositionFramePresentStatistics_INTERFACE_DEFINED__ */


#ifndef __IIndependentFlipFramePresentStatistics_INTERFACE_DEFINED__
#define __IIndependentFlipFramePresentStatistics_INTERFACE_DEFINED__

/* interface IIndependentFlipFramePresentStatistics */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IIndependentFlipFramePresentStatistics;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8C93BE27-AD94-4DA0-8FD4-2413132D124E")
    IIndependentFlipFramePresentStatistics : public IPresentStatistics
    {
    public:
        virtual LUID STDMETHODCALLTYPE GetOutputAdapterLUID( void) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetOutputVidPnSourceId( void) = 0;
        
        virtual UINT_PTR STDMETHODCALLTYPE GetContentTag( void) = 0;
        
        virtual SystemInterruptTime STDMETHODCALLTYPE GetDisplayedTime( void) = 0;
        
        virtual SystemInterruptTime STDMETHODCALLTYPE GetPresentDuration( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIndependentFlipFramePresentStatisticsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIndependentFlipFramePresentStatistics * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetPresentId)
        UINT64 ( STDMETHODCALLTYPE *GetPresentId )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IPresentStatistics, GetKind)
        PresentStatisticsKind ( STDMETHODCALLTYPE *GetKind )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IIndependentFlipFramePresentStatistics, GetOutputAdapterLUID)
        LUID ( STDMETHODCALLTYPE *GetOutputAdapterLUID )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IIndependentFlipFramePresentStatistics, GetOutputVidPnSourceId)
        UINT ( STDMETHODCALLTYPE *GetOutputVidPnSourceId )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IIndependentFlipFramePresentStatistics, GetContentTag)
        UINT_PTR ( STDMETHODCALLTYPE *GetContentTag )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IIndependentFlipFramePresentStatistics, GetDisplayedTime)
        SystemInterruptTime ( STDMETHODCALLTYPE *GetDisplayedTime )( 
            IIndependentFlipFramePresentStatistics * This);
        
        DECLSPEC_XFGVIRT(IIndependentFlipFramePresentStatistics, GetPresentDuration)
        SystemInterruptTime ( STDMETHODCALLTYPE *GetPresentDuration )( 
            IIndependentFlipFramePresentStatistics * This);
        
        END_INTERFACE
    } IIndependentFlipFramePresentStatisticsVtbl;

    interface IIndependentFlipFramePresentStatistics
    {
        CONST_VTBL struct IIndependentFlipFramePresentStatisticsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIndependentFlipFramePresentStatistics_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIndependentFlipFramePresentStatistics_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIndependentFlipFramePresentStatistics_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIndependentFlipFramePresentStatistics_GetPresentId(This)	\
    ( (This)->lpVtbl -> GetPresentId(This) ) 

#define IIndependentFlipFramePresentStatistics_GetKind(This)	\
    ( (This)->lpVtbl -> GetKind(This) ) 


#define IIndependentFlipFramePresentStatistics_GetOutputAdapterLUID(This)	\
    ( (This)->lpVtbl -> GetOutputAdapterLUID(This) ) 

#define IIndependentFlipFramePresentStatistics_GetOutputVidPnSourceId(This)	\
    ( (This)->lpVtbl -> GetOutputVidPnSourceId(This) ) 

#define IIndependentFlipFramePresentStatistics_GetContentTag(This)	\
    ( (This)->lpVtbl -> GetContentTag(This) ) 

#define IIndependentFlipFramePresentStatistics_GetDisplayedTime(This)	\
    ( (This)->lpVtbl -> GetDisplayedTime(This) ) 

#define IIndependentFlipFramePresentStatistics_GetPresentDuration(This)	\
    ( (This)->lpVtbl -> GetPresentDuration(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



LUID STDMETHODCALLTYPE IIndependentFlipFramePresentStatistics_GetOutputAdapterLUID_Proxy( 
    IIndependentFlipFramePresentStatistics * This);


void __RPC_STUB IIndependentFlipFramePresentStatistics_GetOutputAdapterLUID_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


SystemInterruptTime STDMETHODCALLTYPE IIndependentFlipFramePresentStatistics_GetDisplayedTime_Proxy( 
    IIndependentFlipFramePresentStatistics * This);


void __RPC_STUB IIndependentFlipFramePresentStatistics_GetDisplayedTime_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


SystemInterruptTime STDMETHODCALLTYPE IIndependentFlipFramePresentStatistics_GetPresentDuration_Proxy( 
    IIndependentFlipFramePresentStatistics * This);


void __RPC_STUB IIndependentFlipFramePresentStatistics_GetPresentDuration_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IIndependentFlipFramePresentStatistics_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_Presentation_0000_0011 */
/* [local] */ 

//
// Main entrypoint for creating a presentation factory
//
STDAPI CreatePresentationFactory(
    _In_ IUnknown * d3dDevice,
    _In_ REFIID riid,
    _Outptr_ void ** presentationFactory);



extern RPC_IF_HANDLE __MIDL_itf_Presentation_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_Presentation_0000_0011_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


