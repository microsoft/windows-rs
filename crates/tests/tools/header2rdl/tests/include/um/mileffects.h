

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


#ifndef __mileffects_h__
#define __mileffects_h__

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

#ifndef __IMILBitmapEffectConnectorInfo_FWD_DEFINED__
#define __IMILBitmapEffectConnectorInfo_FWD_DEFINED__
typedef interface IMILBitmapEffectConnectorInfo IMILBitmapEffectConnectorInfo;

#endif 	/* __IMILBitmapEffectConnectorInfo_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectConnectionsInfo_FWD_DEFINED__
#define __IMILBitmapEffectConnectionsInfo_FWD_DEFINED__
typedef interface IMILBitmapEffectConnectionsInfo IMILBitmapEffectConnectionsInfo;

#endif 	/* __IMILBitmapEffectConnectionsInfo_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectConnections_FWD_DEFINED__
#define __IMILBitmapEffectConnections_FWD_DEFINED__
typedef interface IMILBitmapEffectConnections IMILBitmapEffectConnections;

#endif 	/* __IMILBitmapEffectConnections_FWD_DEFINED__ */


#ifndef __IMILBitmapEffect_FWD_DEFINED__
#define __IMILBitmapEffect_FWD_DEFINED__
typedef interface IMILBitmapEffect IMILBitmapEffect;

#endif 	/* __IMILBitmapEffect_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectImpl_FWD_DEFINED__
#define __IMILBitmapEffectImpl_FWD_DEFINED__
typedef interface IMILBitmapEffectImpl IMILBitmapEffectImpl;

#endif 	/* __IMILBitmapEffectImpl_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectGroup_FWD_DEFINED__
#define __IMILBitmapEffectGroup_FWD_DEFINED__
typedef interface IMILBitmapEffectGroup IMILBitmapEffectGroup;

#endif 	/* __IMILBitmapEffectGroup_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectGroupImpl_FWD_DEFINED__
#define __IMILBitmapEffectGroupImpl_FWD_DEFINED__
typedef interface IMILBitmapEffectGroupImpl IMILBitmapEffectGroupImpl;

#endif 	/* __IMILBitmapEffectGroupImpl_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectRenderContext_FWD_DEFINED__
#define __IMILBitmapEffectRenderContext_FWD_DEFINED__
typedef interface IMILBitmapEffectRenderContext IMILBitmapEffectRenderContext;

#endif 	/* __IMILBitmapEffectRenderContext_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectRenderContextImpl_FWD_DEFINED__
#define __IMILBitmapEffectRenderContextImpl_FWD_DEFINED__
typedef interface IMILBitmapEffectRenderContextImpl IMILBitmapEffectRenderContextImpl;

#endif 	/* __IMILBitmapEffectRenderContextImpl_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectFactory_FWD_DEFINED__
#define __IMILBitmapEffectFactory_FWD_DEFINED__
typedef interface IMILBitmapEffectFactory IMILBitmapEffectFactory;

#endif 	/* __IMILBitmapEffectFactory_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectPrimitive_FWD_DEFINED__
#define __IMILBitmapEffectPrimitive_FWD_DEFINED__
typedef interface IMILBitmapEffectPrimitive IMILBitmapEffectPrimitive;

#endif 	/* __IMILBitmapEffectPrimitive_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectPrimitiveImpl_FWD_DEFINED__
#define __IMILBitmapEffectPrimitiveImpl_FWD_DEFINED__
typedef interface IMILBitmapEffectPrimitiveImpl IMILBitmapEffectPrimitiveImpl;

#endif 	/* __IMILBitmapEffectPrimitiveImpl_FWD_DEFINED__ */


#ifndef __IMILBitmapEffects_FWD_DEFINED__
#define __IMILBitmapEffects_FWD_DEFINED__
typedef interface IMILBitmapEffects IMILBitmapEffects;

#endif 	/* __IMILBitmapEffects_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectConnector_FWD_DEFINED__
#define __IMILBitmapEffectConnector_FWD_DEFINED__
typedef interface IMILBitmapEffectConnector IMILBitmapEffectConnector;

#endif 	/* __IMILBitmapEffectConnector_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectInputConnector_FWD_DEFINED__
#define __IMILBitmapEffectInputConnector_FWD_DEFINED__
typedef interface IMILBitmapEffectInputConnector IMILBitmapEffectInputConnector;

#endif 	/* __IMILBitmapEffectInputConnector_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectOutputConnector_FWD_DEFINED__
#define __IMILBitmapEffectOutputConnector_FWD_DEFINED__
typedef interface IMILBitmapEffectOutputConnector IMILBitmapEffectOutputConnector;

#endif 	/* __IMILBitmapEffectOutputConnector_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectOutputConnectorImpl_FWD_DEFINED__
#define __IMILBitmapEffectOutputConnectorImpl_FWD_DEFINED__
typedef interface IMILBitmapEffectOutputConnectorImpl IMILBitmapEffectOutputConnectorImpl;

#endif 	/* __IMILBitmapEffectOutputConnectorImpl_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectInteriorInputConnector_FWD_DEFINED__
#define __IMILBitmapEffectInteriorInputConnector_FWD_DEFINED__
typedef interface IMILBitmapEffectInteriorInputConnector IMILBitmapEffectInteriorInputConnector;

#endif 	/* __IMILBitmapEffectInteriorInputConnector_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectInteriorOutputConnector_FWD_DEFINED__
#define __IMILBitmapEffectInteriorOutputConnector_FWD_DEFINED__
typedef interface IMILBitmapEffectInteriorOutputConnector IMILBitmapEffectInteriorOutputConnector;

#endif 	/* __IMILBitmapEffectInteriorOutputConnector_FWD_DEFINED__ */


#ifndef __IMILBitmapEffectEvents_FWD_DEFINED__
#define __IMILBitmapEffectEvents_FWD_DEFINED__
typedef interface IMILBitmapEffectEvents IMILBitmapEffectEvents;

#endif 	/* __IMILBitmapEffectEvents_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "wincodec.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mileffects_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if 0
typedef DWORD MilPoint2D;

typedef DWORD MilRectD;

typedef DWORD MilMatrix3x2D;

#endif
#ifndef MILTYPES_DEFINED
typedef struct MilRectD
{
    DOUBLE left;
    DOUBLE top;
    DOUBLE right;
    DOUBLE bottom;
} MilRectD;

typedef struct MilPoint2D
{
    DOUBLE X;
    DOUBLE Y;
} MilPoint2D;

typedef struct MilMatrix3x2D
{
    DOUBLE S_11;
    DOUBLE S_12;
    DOUBLE S_21;
    DOUBLE S_22;
    DOUBLE DX;
    DOUBLE DY;
} MilMatrix3x2D;
#endif // MILTYPES_DEFINED

typedef MilPoint2D MIL_2DPOINTD;

#ifndef MILCORE_MIL_MATRIX3X2D_COMPAT_TYPEDEF

typedef MilMatrix3x2D MIL_MATRIX3X2D;

#define MILCORE_MIL_MATRIX3X2D_COMPAT_TYPEDEF
#endif // MILCORE_MIL_MATRIX3X2D_COMPAT_TYPEDEF



extern RPC_IF_HANDLE __MIDL_itf_mileffects_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mileffects_0000_0000_v0_0_s_ifspec;


#ifndef __MILEffects_LIBRARY_DEFINED__
#define __MILEffects_LIBRARY_DEFINED__

/* library MILEffects */
/* [version][uuid] */ 

#define MILBITMAPEFFECT_SDK_VERSION 0x01000000
DEFINE_GUID(CLSID_MILBitmapEffectGroup, 0xac9c1a9a, 0x7e18, 0x4f64, 0xac, 0x7e, 0x47, 0xcf, 0x7f, 0x05, 0x1e, 0x95);
DEFINE_GUID(CLSID_MILBitmapEffectBlur, 0xa924df87, 0x225d, 0x4373, 0x8f, 0x5b, 0xb9, 0xe, 0xc8, 0x5a, 0xe3, 0xde);
DEFINE_GUID(CLSID_MILBitmapEffectDropShadow, 0x459a3fbe, 0xd8ac, 0x4692, 0x87, 0x4b, 0x7a, 0x26, 0x57, 0x15, 0xaa, 0x16);
DEFINE_GUID(CLSID_MILBitmapEffectOuterGlow, 0xe2161bdd, 0x7eb6, 0x4725, 0x9c, 0x0b, 0x8a, 0x2a, 0x1b, 0x4f, 0x06, 0x67);
DEFINE_GUID(CLSID_MILBitmapEffectBevel, 0xfd361dbe, 0x6c9b, 0x4de0, 0x82, 0x90, 0xf6, 0x40, 0xc, 0x27, 0x37, 0xed);
DEFINE_GUID(CLSID_MILBitmapEffectEmboss, 0xcd299846, 0x824f, 0x47ec, 0xa0, 0x07, 0x12, 0xaa, 0x76, 0x7f, 0x28, 0x16);
extern STDMETHODIMP MILCreateBitmapEffectFactory(DWORD dwSDKVersion, IMILBitmapEffectFactory **ppFactory);




















typedef /* [public] */ REFGUID REFWICPixelFormatGUID;

typedef /* [public] */ GUID WICPixelFormatGUID;

typedef /* [public] */ struct MILMatrixF
    {
    DOUBLE _11;
    DOUBLE _12;
    DOUBLE _13;
    DOUBLE _14;
    DOUBLE _21;
    DOUBLE _22;
    DOUBLE _23;
    DOUBLE _24;
    DOUBLE _31;
    DOUBLE _32;
    DOUBLE _33;
    DOUBLE _34;
    DOUBLE _41;
    DOUBLE _42;
    DOUBLE _43;
    DOUBLE _44;
    } 	MILMatrixF;


EXTERN_C const IID LIBID_MILEffects;

#ifndef __IMILBitmapEffectConnectorInfo_INTERFACE_DEFINED__
#define __IMILBitmapEffectConnectorInfo_INTERFACE_DEFINED__

/* interface IMILBitmapEffectConnectorInfo */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectConnectorInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F66D2E4B-B46B-42FC-859E-3DA0ECDB3C43")
    IMILBitmapEffectConnectorInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetIndex( 
            /* [retval][out] */ __RPC__out ULONG *puiIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOptimalFormat( 
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberFormats( 
            /* [retval][out] */ __RPC__out ULONG *pulNumberFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectConnectorInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This,
            /* [retval][out] */ __RPC__out ULONG *puiIndex);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetOptimalFormat)
        HRESULT ( STDMETHODCALLTYPE *GetOptimalFormat )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetNumberFormats)
        HRESULT ( STDMETHODCALLTYPE *GetNumberFormats )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This,
            /* [retval][out] */ __RPC__out ULONG *pulNumberFormats);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in IMILBitmapEffectConnectorInfo * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        END_INTERFACE
    } IMILBitmapEffectConnectorInfoVtbl;

    interface IMILBitmapEffectConnectorInfo
    {
        CONST_VTBL struct IMILBitmapEffectConnectorInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectConnectorInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectConnectorInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectConnectorInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectConnectorInfo_GetIndex(This,puiIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,puiIndex) ) 

#define IMILBitmapEffectConnectorInfo_GetOptimalFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetOptimalFormat(This,pFormat) ) 

#define IMILBitmapEffectConnectorInfo_GetNumberFormats(This,pulNumberFormats)	\
    ( (This)->lpVtbl -> GetNumberFormats(This,pulNumberFormats) ) 

#define IMILBitmapEffectConnectorInfo_GetFormat(This,ulIndex,pFormat)	\
    ( (This)->lpVtbl -> GetFormat(This,ulIndex,pFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectConnectorInfo_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectConnectionsInfo_INTERFACE_DEFINED__
#define __IMILBitmapEffectConnectionsInfo_INTERFACE_DEFINED__

/* interface IMILBitmapEffectConnectionsInfo */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectConnectionsInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("476B538A-C765-4237-BA4A-D6A880FF0CFC")
    IMILBitmapEffectConnectionsInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberInputs( 
            /* [retval][out] */ __RPC__out ULONG *puiNumInputs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOutputs( 
            /* [retval][out] */ __RPC__out ULONG *puiNumOutputs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputConnectorInfo( 
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffectConnectorInfo **ppConnectorInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputConnectorInfo( 
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffectConnectorInfo **ppConnectorInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectConnectionsInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectionsInfo, GetNumberInputs)
        HRESULT ( STDMETHODCALLTYPE *GetNumberInputs )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This,
            /* [retval][out] */ __RPC__out ULONG *puiNumInputs);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectionsInfo, GetNumberOutputs)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOutputs )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This,
            /* [retval][out] */ __RPC__out ULONG *puiNumOutputs);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectionsInfo, GetInputConnectorInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInputConnectorInfo )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This,
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffectConnectorInfo **ppConnectorInfo);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectionsInfo, GetOutputConnectorInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOutputConnectorInfo )( 
            __RPC__in IMILBitmapEffectConnectionsInfo * This,
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffectConnectorInfo **ppConnectorInfo);
        
        END_INTERFACE
    } IMILBitmapEffectConnectionsInfoVtbl;

    interface IMILBitmapEffectConnectionsInfo
    {
        CONST_VTBL struct IMILBitmapEffectConnectionsInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectConnectionsInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectConnectionsInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectConnectionsInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectConnectionsInfo_GetNumberInputs(This,puiNumInputs)	\
    ( (This)->lpVtbl -> GetNumberInputs(This,puiNumInputs) ) 

#define IMILBitmapEffectConnectionsInfo_GetNumberOutputs(This,puiNumOutputs)	\
    ( (This)->lpVtbl -> GetNumberOutputs(This,puiNumOutputs) ) 

#define IMILBitmapEffectConnectionsInfo_GetInputConnectorInfo(This,uiIndex,ppConnectorInfo)	\
    ( (This)->lpVtbl -> GetInputConnectorInfo(This,uiIndex,ppConnectorInfo) ) 

#define IMILBitmapEffectConnectionsInfo_GetOutputConnectorInfo(This,uiIndex,ppConnectorInfo)	\
    ( (This)->lpVtbl -> GetOutputConnectorInfo(This,uiIndex,ppConnectorInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectConnectionsInfo_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectConnections_INTERFACE_DEFINED__
#define __IMILBitmapEffectConnections_INTERFACE_DEFINED__

/* interface IMILBitmapEffectConnections */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectConnections;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C2B5D861-9B1A-4374-89B0-DEC4874D6A81")
    IMILBitmapEffectConnections : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInputConnector( 
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **ppConnector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputConnector( 
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **ppConnector) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectConnectionsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectConnections * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectConnections * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectConnections * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnections, GetInputConnector)
        HRESULT ( STDMETHODCALLTYPE *GetInputConnector )( 
            __RPC__in IMILBitmapEffectConnections * This,
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **ppConnector);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnections, GetOutputConnector)
        HRESULT ( STDMETHODCALLTYPE *GetOutputConnector )( 
            __RPC__in IMILBitmapEffectConnections * This,
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **ppConnector);
        
        END_INTERFACE
    } IMILBitmapEffectConnectionsVtbl;

    interface IMILBitmapEffectConnections
    {
        CONST_VTBL struct IMILBitmapEffectConnectionsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectConnections_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectConnections_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectConnections_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectConnections_GetInputConnector(This,uiIndex,ppConnector)	\
    ( (This)->lpVtbl -> GetInputConnector(This,uiIndex,ppConnector) ) 

#define IMILBitmapEffectConnections_GetOutputConnector(This,uiIndex,ppConnector)	\
    ( (This)->lpVtbl -> GetOutputConnector(This,uiIndex,ppConnector) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectConnections_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffect_INTERFACE_DEFINED__
#define __IMILBitmapEffect_INTERFACE_DEFINED__

/* interface IMILBitmapEffect */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8A6FF321-C944-4A1B-9944-9954AF301258")
    IMILBitmapEffect : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOutput( 
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParentEffect( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectGroup **ppParentEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInputSource( 
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IWICBitmapSource *pBitmapSource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffect * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffect * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffect, GetOutput)
        HRESULT ( STDMETHODCALLTYPE *GetOutput )( 
            __RPC__in IMILBitmapEffect * This,
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffect, GetParentEffect)
        HRESULT ( STDMETHODCALLTYPE *GetParentEffect )( 
            __RPC__in IMILBitmapEffect * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectGroup **ppParentEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffect, SetInputSource)
        HRESULT ( STDMETHODCALLTYPE *SetInputSource )( 
            __RPC__in IMILBitmapEffect * This,
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IWICBitmapSource *pBitmapSource);
        
        END_INTERFACE
    } IMILBitmapEffectVtbl;

    interface IMILBitmapEffect
    {
        CONST_VTBL struct IMILBitmapEffectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffect_GetOutput(This,uiIndex,pContext,ppBitmapSource)	\
    ( (This)->lpVtbl -> GetOutput(This,uiIndex,pContext,ppBitmapSource) ) 

#define IMILBitmapEffect_GetParentEffect(This,ppParentEffect)	\
    ( (This)->lpVtbl -> GetParentEffect(This,ppParentEffect) ) 

#define IMILBitmapEffect_SetInputSource(This,uiIndex,pBitmapSource)	\
    ( (This)->lpVtbl -> SetInputSource(This,uiIndex,pBitmapSource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffect_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectImpl_INTERFACE_DEFINED__
#define __IMILBitmapEffectImpl_INTERFACE_DEFINED__

/* interface IMILBitmapEffectImpl */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectImpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CC2468F2-9936-47BE-B4AF-06B5DF5DBCBB")
    IMILBitmapEffectImpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsInPlaceModificationAllowed( 
            /* [in] */ __RPC__in_opt IMILBitmapEffectOutputConnector *pOutputConnector,
            /* [out] */ __RPC__out VARIANT_BOOL *pfModifyInPlace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetParentEffect( 
            /* [in] */ __RPC__in_opt IMILBitmapEffectGroup *pParentEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputSource( 
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputSourceBounds( 
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__out MilRectD *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputBitmapSource( 
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pRenderContext,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfModifyInPlace,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputBitmapSource( 
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pRenderContext,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfModifyInPlace,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in_opt IUnknown *pInner) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectImplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectImpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectImpl * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, IsInPlaceModificationAllowed)
        HRESULT ( STDMETHODCALLTYPE *IsInPlaceModificationAllowed )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffectOutputConnector *pOutputConnector,
            /* [out] */ __RPC__out VARIANT_BOOL *pfModifyInPlace);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, SetParentEffect)
        HRESULT ( STDMETHODCALLTYPE *SetParentEffect )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffectGroup *pParentEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, GetInputSource)
        HRESULT ( STDMETHODCALLTYPE *GetInputSource )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, GetInputSourceBounds)
        HRESULT ( STDMETHODCALLTYPE *GetInputSourceBounds )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__out MilRectD *pRect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, GetInputBitmapSource)
        HRESULT ( STDMETHODCALLTYPE *GetInputBitmapSource )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pRenderContext,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfModifyInPlace,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, GetOutputBitmapSource)
        HRESULT ( STDMETHODCALLTYPE *GetOutputBitmapSource )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pRenderContext,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfModifyInPlace,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectImpl, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IMILBitmapEffectImpl * This,
            /* [in] */ __RPC__in_opt IUnknown *pInner);
        
        END_INTERFACE
    } IMILBitmapEffectImplVtbl;

    interface IMILBitmapEffectImpl
    {
        CONST_VTBL struct IMILBitmapEffectImplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectImpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectImpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectImpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectImpl_IsInPlaceModificationAllowed(This,pOutputConnector,pfModifyInPlace)	\
    ( (This)->lpVtbl -> IsInPlaceModificationAllowed(This,pOutputConnector,pfModifyInPlace) ) 

#define IMILBitmapEffectImpl_SetParentEffect(This,pParentEffect)	\
    ( (This)->lpVtbl -> SetParentEffect(This,pParentEffect) ) 

#define IMILBitmapEffectImpl_GetInputSource(This,uiIndex,ppBitmapSource)	\
    ( (This)->lpVtbl -> GetInputSource(This,uiIndex,ppBitmapSource) ) 

#define IMILBitmapEffectImpl_GetInputSourceBounds(This,uiIndex,pRect)	\
    ( (This)->lpVtbl -> GetInputSourceBounds(This,uiIndex,pRect) ) 

#define IMILBitmapEffectImpl_GetInputBitmapSource(This,uiIndex,pRenderContext,pfModifyInPlace,ppBitmapSource)	\
    ( (This)->lpVtbl -> GetInputBitmapSource(This,uiIndex,pRenderContext,pfModifyInPlace,ppBitmapSource) ) 

#define IMILBitmapEffectImpl_GetOutputBitmapSource(This,uiIndex,pRenderContext,pfModifyInPlace,ppBitmapSource)	\
    ( (This)->lpVtbl -> GetOutputBitmapSource(This,uiIndex,pRenderContext,pfModifyInPlace,ppBitmapSource) ) 

#define IMILBitmapEffectImpl_Initialize(This,pInner)	\
    ( (This)->lpVtbl -> Initialize(This,pInner) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectImpl_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectGroup_INTERFACE_DEFINED__
#define __IMILBitmapEffectGroup_INTERFACE_DEFINED__

/* interface IMILBitmapEffectGroup */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2F952360-698A-4ac6-81A1-BCFDF08EB8E8")
    IMILBitmapEffectGroup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInteriorInputConnector( 
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **ppConnector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInteriorOutputConnector( 
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **ppConnector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Add( 
            /* [in] */ __RPC__in_opt IMILBitmapEffect *pEffect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectGroup * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectGroup * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectGroup, GetInteriorInputConnector)
        HRESULT ( STDMETHODCALLTYPE *GetInteriorInputConnector )( 
            __RPC__in IMILBitmapEffectGroup * This,
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **ppConnector);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectGroup, GetInteriorOutputConnector)
        HRESULT ( STDMETHODCALLTYPE *GetInteriorOutputConnector )( 
            __RPC__in IMILBitmapEffectGroup * This,
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **ppConnector);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectGroup, Add)
        HRESULT ( STDMETHODCALLTYPE *Add )( 
            __RPC__in IMILBitmapEffectGroup * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffect *pEffect);
        
        END_INTERFACE
    } IMILBitmapEffectGroupVtbl;

    interface IMILBitmapEffectGroup
    {
        CONST_VTBL struct IMILBitmapEffectGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectGroup_GetInteriorInputConnector(This,uiIndex,ppConnector)	\
    ( (This)->lpVtbl -> GetInteriorInputConnector(This,uiIndex,ppConnector) ) 

#define IMILBitmapEffectGroup_GetInteriorOutputConnector(This,uiIndex,ppConnector)	\
    ( (This)->lpVtbl -> GetInteriorOutputConnector(This,uiIndex,ppConnector) ) 

#define IMILBitmapEffectGroup_Add(This,pEffect)	\
    ( (This)->lpVtbl -> Add(This,pEffect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectGroup_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectGroupImpl_INTERFACE_DEFINED__
#define __IMILBitmapEffectGroupImpl_INTERFACE_DEFINED__

/* interface IMILBitmapEffectGroupImpl */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectGroupImpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("78FED518-1CFC-4807-8B85-6B6E51398F62")
    IMILBitmapEffectGroupImpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Preprocess( 
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberChildren( 
            /* [retval][out] */ __RPC__out ULONG *puiNumberChildren) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChildren( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffects **pChildren) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectGroupImplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectGroupImpl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectGroupImpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectGroupImpl * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectGroupImpl, Preprocess)
        HRESULT ( STDMETHODCALLTYPE *Preprocess )( 
            __RPC__in IMILBitmapEffectGroupImpl * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectGroupImpl, GetNumberChildren)
        HRESULT ( STDMETHODCALLTYPE *GetNumberChildren )( 
            __RPC__in IMILBitmapEffectGroupImpl * This,
            /* [retval][out] */ __RPC__out ULONG *puiNumberChildren);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectGroupImpl, GetChildren)
        HRESULT ( STDMETHODCALLTYPE *GetChildren )( 
            __RPC__in IMILBitmapEffectGroupImpl * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffects **pChildren);
        
        END_INTERFACE
    } IMILBitmapEffectGroupImplVtbl;

    interface IMILBitmapEffectGroupImpl
    {
        CONST_VTBL struct IMILBitmapEffectGroupImplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectGroupImpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectGroupImpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectGroupImpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectGroupImpl_Preprocess(This,pContext)	\
    ( (This)->lpVtbl -> Preprocess(This,pContext) ) 

#define IMILBitmapEffectGroupImpl_GetNumberChildren(This,puiNumberChildren)	\
    ( (This)->lpVtbl -> GetNumberChildren(This,puiNumberChildren) ) 

#define IMILBitmapEffectGroupImpl_GetChildren(This,pChildren)	\
    ( (This)->lpVtbl -> GetChildren(This,pChildren) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectGroupImpl_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectRenderContext_INTERFACE_DEFINED__
#define __IMILBitmapEffectRenderContext_INTERFACE_DEFINED__

/* interface IMILBitmapEffectRenderContext */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectRenderContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("12A2EC7E-2D33-44b2-B334-1ABB7846E390")
    IMILBitmapEffectRenderContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutputPixelFormat( 
            /* [in] */ __RPC__in REFWICPixelFormatGUID format) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputPixelFormat( 
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUseSoftwareRenderer( 
            /* [in] */ VARIANT_BOOL fSoftware) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInitialTransform( 
            /* [in] */ __RPC__in MILMatrixF *pMatrix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFinalTransform( 
            /* [out] */ __RPC__out MILMatrixF *pMatrix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputDPI( 
            /* [in] */ double dblDpiX,
            /* [in] */ double dblDpiY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputDPI( 
            /* [out] */ __RPC__out double *pdblDpiX,
            /* [out] */ __RPC__out double *pdblDpiY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRegionOfInterest( 
            /* [in] */ __RPC__in MilRectD *pRect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectRenderContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectRenderContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectRenderContext * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, SetOutputPixelFormat)
        HRESULT ( STDMETHODCALLTYPE *SetOutputPixelFormat )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [in] */ __RPC__in REFWICPixelFormatGUID format);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, GetOutputPixelFormat)
        HRESULT ( STDMETHODCALLTYPE *GetOutputPixelFormat )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, SetUseSoftwareRenderer)
        HRESULT ( STDMETHODCALLTYPE *SetUseSoftwareRenderer )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [in] */ VARIANT_BOOL fSoftware);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, SetInitialTransform)
        HRESULT ( STDMETHODCALLTYPE *SetInitialTransform )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [in] */ __RPC__in MILMatrixF *pMatrix);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, GetFinalTransform)
        HRESULT ( STDMETHODCALLTYPE *GetFinalTransform )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [out] */ __RPC__out MILMatrixF *pMatrix);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, SetOutputDPI)
        HRESULT ( STDMETHODCALLTYPE *SetOutputDPI )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [in] */ double dblDpiX,
            /* [in] */ double dblDpiY);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, GetOutputDPI)
        HRESULT ( STDMETHODCALLTYPE *GetOutputDPI )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [out] */ __RPC__out double *pdblDpiX,
            /* [out] */ __RPC__out double *pdblDpiY);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContext, SetRegionOfInterest)
        HRESULT ( STDMETHODCALLTYPE *SetRegionOfInterest )( 
            __RPC__in IMILBitmapEffectRenderContext * This,
            /* [in] */ __RPC__in MilRectD *pRect);
        
        END_INTERFACE
    } IMILBitmapEffectRenderContextVtbl;

    interface IMILBitmapEffectRenderContext
    {
        CONST_VTBL struct IMILBitmapEffectRenderContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectRenderContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectRenderContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectRenderContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectRenderContext_SetOutputPixelFormat(This,format)	\
    ( (This)->lpVtbl -> SetOutputPixelFormat(This,format) ) 

#define IMILBitmapEffectRenderContext_GetOutputPixelFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetOutputPixelFormat(This,pFormat) ) 

#define IMILBitmapEffectRenderContext_SetUseSoftwareRenderer(This,fSoftware)	\
    ( (This)->lpVtbl -> SetUseSoftwareRenderer(This,fSoftware) ) 

#define IMILBitmapEffectRenderContext_SetInitialTransform(This,pMatrix)	\
    ( (This)->lpVtbl -> SetInitialTransform(This,pMatrix) ) 

#define IMILBitmapEffectRenderContext_GetFinalTransform(This,pMatrix)	\
    ( (This)->lpVtbl -> GetFinalTransform(This,pMatrix) ) 

#define IMILBitmapEffectRenderContext_SetOutputDPI(This,dblDpiX,dblDpiY)	\
    ( (This)->lpVtbl -> SetOutputDPI(This,dblDpiX,dblDpiY) ) 

#define IMILBitmapEffectRenderContext_GetOutputDPI(This,pdblDpiX,pdblDpiY)	\
    ( (This)->lpVtbl -> GetOutputDPI(This,pdblDpiX,pdblDpiY) ) 

#define IMILBitmapEffectRenderContext_SetRegionOfInterest(This,pRect)	\
    ( (This)->lpVtbl -> SetRegionOfInterest(This,pRect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectRenderContext_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectRenderContextImpl_INTERFACE_DEFINED__
#define __IMILBitmapEffectRenderContextImpl_INTERFACE_DEFINED__

/* interface IMILBitmapEffectRenderContextImpl */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectRenderContextImpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4D25ACCB-797D-4fd2-B128-DFFEFF84FCC3")
    IMILBitmapEffectRenderContextImpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUseSoftwareRenderer( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSoftware) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransform( 
            /* [out][in] */ __RPC__inout MILMatrixF *pMatrix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateTransform( 
            /* [in] */ __RPC__in MILMatrixF *pMatrix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputBounds( 
            /* [out][in] */ __RPC__inout MilRectD *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateOutputBounds( 
            /* [in] */ __RPC__in MilRectD *pRect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectRenderContextImplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContextImpl, GetUseSoftwareRenderer)
        HRESULT ( STDMETHODCALLTYPE *GetUseSoftwareRenderer )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfSoftware);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContextImpl, GetTransform)
        HRESULT ( STDMETHODCALLTYPE *GetTransform )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This,
            /* [out][in] */ __RPC__inout MILMatrixF *pMatrix);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContextImpl, UpdateTransform)
        HRESULT ( STDMETHODCALLTYPE *UpdateTransform )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This,
            /* [in] */ __RPC__in MILMatrixF *pMatrix);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContextImpl, GetOutputBounds)
        HRESULT ( STDMETHODCALLTYPE *GetOutputBounds )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This,
            /* [out][in] */ __RPC__inout MilRectD *pRect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectRenderContextImpl, UpdateOutputBounds)
        HRESULT ( STDMETHODCALLTYPE *UpdateOutputBounds )( 
            __RPC__in IMILBitmapEffectRenderContextImpl * This,
            /* [in] */ __RPC__in MilRectD *pRect);
        
        END_INTERFACE
    } IMILBitmapEffectRenderContextImplVtbl;

    interface IMILBitmapEffectRenderContextImpl
    {
        CONST_VTBL struct IMILBitmapEffectRenderContextImplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectRenderContextImpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectRenderContextImpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectRenderContextImpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectRenderContextImpl_GetUseSoftwareRenderer(This,pfSoftware)	\
    ( (This)->lpVtbl -> GetUseSoftwareRenderer(This,pfSoftware) ) 

#define IMILBitmapEffectRenderContextImpl_GetTransform(This,pMatrix)	\
    ( (This)->lpVtbl -> GetTransform(This,pMatrix) ) 

#define IMILBitmapEffectRenderContextImpl_UpdateTransform(This,pMatrix)	\
    ( (This)->lpVtbl -> UpdateTransform(This,pMatrix) ) 

#define IMILBitmapEffectRenderContextImpl_GetOutputBounds(This,pRect)	\
    ( (This)->lpVtbl -> GetOutputBounds(This,pRect) ) 

#define IMILBitmapEffectRenderContextImpl_UpdateOutputBounds(This,pRect)	\
    ( (This)->lpVtbl -> UpdateOutputBounds(This,pRect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectRenderContextImpl_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectFactory_INTERFACE_DEFINED__
#define __IMILBitmapEffectFactory_INTERFACE_DEFINED__

/* interface IMILBitmapEffectFactory */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33A9DF34-A403-4EC7-B07E-BC0682370845")
    IMILBitmapEffectFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateEffect( 
            /* [in] */ __RPC__in const GUID *pguidEffect,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateContext( 
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffectRenderContext **ppContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateEffectOuter( 
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectFactory * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectFactory * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectFactory, CreateEffect)
        HRESULT ( STDMETHODCALLTYPE *CreateEffect )( 
            __RPC__in IMILBitmapEffectFactory * This,
            /* [in] */ __RPC__in const GUID *pguidEffect,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectFactory, CreateContext)
        HRESULT ( STDMETHODCALLTYPE *CreateContext )( 
            __RPC__in IMILBitmapEffectFactory * This,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffectRenderContext **ppContext);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectFactory, CreateEffectOuter)
        HRESULT ( STDMETHODCALLTYPE *CreateEffectOuter )( 
            __RPC__in IMILBitmapEffectFactory * This,
            /* [out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect);
        
        END_INTERFACE
    } IMILBitmapEffectFactoryVtbl;

    interface IMILBitmapEffectFactory
    {
        CONST_VTBL struct IMILBitmapEffectFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectFactory_CreateEffect(This,pguidEffect,ppEffect)	\
    ( (This)->lpVtbl -> CreateEffect(This,pguidEffect,ppEffect) ) 

#define IMILBitmapEffectFactory_CreateContext(This,ppContext)	\
    ( (This)->lpVtbl -> CreateContext(This,ppContext) ) 

#define IMILBitmapEffectFactory_CreateEffectOuter(This,ppEffect)	\
    ( (This)->lpVtbl -> CreateEffectOuter(This,ppEffect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectFactory_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectPrimitive_INTERFACE_DEFINED__
#define __IMILBitmapEffectPrimitive_INTERFACE_DEFINED__

/* interface IMILBitmapEffectPrimitive */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectPrimitive;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("67E31025-3091-4dfc-98D6-DD494551461D")
    IMILBitmapEffectPrimitive : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOutput( 
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfModifyInPlace,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransformPoint( 
            /* [in] */ ULONG uiIndex,
            /* [out][in] */ __RPC__inout MilPoint2D *p,
            /* [in] */ VARIANT_BOOL fForwardTransform,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext,
            /* [out] */ __RPC__out VARIANT_BOOL *pfPointTransformed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransformRect( 
            /* [in] */ ULONG uiIndex,
            /* [out][in] */ __RPC__inout MilRectD *p,
            /* [in] */ VARIANT_BOOL fForwardTransform,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasAffineTransform( 
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__out VARIANT_BOOL *pfAffine) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasInverseTransform( 
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__out VARIANT_BOOL *pfHasInverse) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAffineMatrix( 
            /* [in] */ ULONG uiIndex,
            /* [out][in] */ __RPC__inout MilMatrix3x2D *pMatrix) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectPrimitiveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectPrimitive * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectPrimitive * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitive, GetOutput)
        HRESULT ( STDMETHODCALLTYPE *GetOutput )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ ULONG uiIndex,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext,
            /* [out][in] */ __RPC__inout VARIANT_BOOL *pfModifyInPlace,
            /* [retval][out] */ __RPC__deref_out_opt IWICBitmapSource **ppBitmapSource);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitive, TransformPoint)
        HRESULT ( STDMETHODCALLTYPE *TransformPoint )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ ULONG uiIndex,
            /* [out][in] */ __RPC__inout MilPoint2D *p,
            /* [in] */ VARIANT_BOOL fForwardTransform,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext,
            /* [out] */ __RPC__out VARIANT_BOOL *pfPointTransformed);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitive, TransformRect)
        HRESULT ( STDMETHODCALLTYPE *TransformRect )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ ULONG uiIndex,
            /* [out][in] */ __RPC__inout MilRectD *p,
            /* [in] */ VARIANT_BOOL fForwardTransform,
            /* [in] */ __RPC__in_opt IMILBitmapEffectRenderContext *pContext);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitive, HasAffineTransform)
        HRESULT ( STDMETHODCALLTYPE *HasAffineTransform )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__out VARIANT_BOOL *pfAffine);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitive, HasInverseTransform)
        HRESULT ( STDMETHODCALLTYPE *HasInverseTransform )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ ULONG uiIndex,
            /* [out] */ __RPC__out VARIANT_BOOL *pfHasInverse);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitive, GetAffineMatrix)
        HRESULT ( STDMETHODCALLTYPE *GetAffineMatrix )( 
            __RPC__in IMILBitmapEffectPrimitive * This,
            /* [in] */ ULONG uiIndex,
            /* [out][in] */ __RPC__inout MilMatrix3x2D *pMatrix);
        
        END_INTERFACE
    } IMILBitmapEffectPrimitiveVtbl;

    interface IMILBitmapEffectPrimitive
    {
        CONST_VTBL struct IMILBitmapEffectPrimitiveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectPrimitive_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectPrimitive_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectPrimitive_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectPrimitive_GetOutput(This,uiIndex,pContext,pfModifyInPlace,ppBitmapSource)	\
    ( (This)->lpVtbl -> GetOutput(This,uiIndex,pContext,pfModifyInPlace,ppBitmapSource) ) 

#define IMILBitmapEffectPrimitive_TransformPoint(This,uiIndex,p,fForwardTransform,pContext,pfPointTransformed)	\
    ( (This)->lpVtbl -> TransformPoint(This,uiIndex,p,fForwardTransform,pContext,pfPointTransformed) ) 

#define IMILBitmapEffectPrimitive_TransformRect(This,uiIndex,p,fForwardTransform,pContext)	\
    ( (This)->lpVtbl -> TransformRect(This,uiIndex,p,fForwardTransform,pContext) ) 

#define IMILBitmapEffectPrimitive_HasAffineTransform(This,uiIndex,pfAffine)	\
    ( (This)->lpVtbl -> HasAffineTransform(This,uiIndex,pfAffine) ) 

#define IMILBitmapEffectPrimitive_HasInverseTransform(This,uiIndex,pfHasInverse)	\
    ( (This)->lpVtbl -> HasInverseTransform(This,uiIndex,pfHasInverse) ) 

#define IMILBitmapEffectPrimitive_GetAffineMatrix(This,uiIndex,pMatrix)	\
    ( (This)->lpVtbl -> GetAffineMatrix(This,uiIndex,pMatrix) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectPrimitive_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectPrimitiveImpl_INTERFACE_DEFINED__
#define __IMILBitmapEffectPrimitiveImpl_INTERFACE_DEFINED__

/* interface IMILBitmapEffectPrimitiveImpl */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectPrimitiveImpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CE41E00B-EFA6-44e7-B007-DD042E3AE126")
    IMILBitmapEffectPrimitiveImpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsDirty( 
            /* [in] */ ULONG uiOutputIndex,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfDirty) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsVolatile( 
            /* [in] */ ULONG uiOutputIndex,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfVolatile) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectPrimitiveImplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectPrimitiveImpl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectPrimitiveImpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectPrimitiveImpl * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitiveImpl, IsDirty)
        HRESULT ( STDMETHODCALLTYPE *IsDirty )( 
            __RPC__in IMILBitmapEffectPrimitiveImpl * This,
            /* [in] */ ULONG uiOutputIndex,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfDirty);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectPrimitiveImpl, IsVolatile)
        HRESULT ( STDMETHODCALLTYPE *IsVolatile )( 
            __RPC__in IMILBitmapEffectPrimitiveImpl * This,
            /* [in] */ ULONG uiOutputIndex,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfVolatile);
        
        END_INTERFACE
    } IMILBitmapEffectPrimitiveImplVtbl;

    interface IMILBitmapEffectPrimitiveImpl
    {
        CONST_VTBL struct IMILBitmapEffectPrimitiveImplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectPrimitiveImpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectPrimitiveImpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectPrimitiveImpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectPrimitiveImpl_IsDirty(This,uiOutputIndex,pfDirty)	\
    ( (This)->lpVtbl -> IsDirty(This,uiOutputIndex,pfDirty) ) 

#define IMILBitmapEffectPrimitiveImpl_IsVolatile(This,uiOutputIndex,pfVolatile)	\
    ( (This)->lpVtbl -> IsVolatile(This,uiOutputIndex,pfVolatile) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectPrimitiveImpl_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffects_INTERFACE_DEFINED__
#define __IMILBitmapEffects_INTERFACE_DEFINED__

/* interface IMILBitmapEffects */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffects;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("51AC3DCE-67C5-448b-9180-AD3EABDDD5DD")
    IMILBitmapEffects : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE _NewEnum( 
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppiuReturn) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Parent( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectGroup **ppEffect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Item( 
            ULONG uindex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect) = 0;
        
        virtual /* [propget] */ HRESULT STDMETHODCALLTYPE get_Count( 
            /* [retval][out] */ __RPC__out ULONG *puiCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffects * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffects * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffects * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffects, _NewEnum)
        HRESULT ( STDMETHODCALLTYPE *_NewEnum )( 
            __RPC__in IMILBitmapEffects * This,
            /* [retval][out] */ __RPC__deref_out_opt IUnknown **ppiuReturn);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffects, get_Parent)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Parent )( 
            __RPC__in IMILBitmapEffects * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectGroup **ppEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffects, Item)
        HRESULT ( STDMETHODCALLTYPE *Item )( 
            __RPC__in IMILBitmapEffects * This,
            ULONG uindex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffects, get_Count)
        /* [propget] */ HRESULT ( STDMETHODCALLTYPE *get_Count )( 
            __RPC__in IMILBitmapEffects * This,
            /* [retval][out] */ __RPC__out ULONG *puiCount);
        
        END_INTERFACE
    } IMILBitmapEffectsVtbl;

    interface IMILBitmapEffects
    {
        CONST_VTBL struct IMILBitmapEffectsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffects_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffects_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffects_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffects__NewEnum(This,ppiuReturn)	\
    ( (This)->lpVtbl -> _NewEnum(This,ppiuReturn) ) 

#define IMILBitmapEffects_get_Parent(This,ppEffect)	\
    ( (This)->lpVtbl -> get_Parent(This,ppEffect) ) 

#define IMILBitmapEffects_Item(This,uindex,ppEffect)	\
    ( (This)->lpVtbl -> Item(This,uindex,ppEffect) ) 

#define IMILBitmapEffects_get_Count(This,puiCount)	\
    ( (This)->lpVtbl -> get_Count(This,puiCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffects_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectConnector_INTERFACE_DEFINED__
#define __IMILBitmapEffectConnector_INTERFACE_DEFINED__

/* interface IMILBitmapEffectConnector */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F59567B3-76C1-4d47-BA1E-79F955E350EF")
    IMILBitmapEffectConnector : public IMILBitmapEffectConnectorInfo
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsConnected( 
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfConnected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBitmapEffect( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectConnector * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [retval][out] */ __RPC__out ULONG *puiIndex);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetOptimalFormat)
        HRESULT ( STDMETHODCALLTYPE *GetOptimalFormat )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetNumberFormats)
        HRESULT ( STDMETHODCALLTYPE *GetNumberFormats )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [retval][out] */ __RPC__out ULONG *pulNumberFormats);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnector, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfConnected);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnector, GetBitmapEffect)
        HRESULT ( STDMETHODCALLTYPE *GetBitmapEffect )( 
            __RPC__in IMILBitmapEffectConnector * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect);
        
        END_INTERFACE
    } IMILBitmapEffectConnectorVtbl;

    interface IMILBitmapEffectConnector
    {
        CONST_VTBL struct IMILBitmapEffectConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectConnector_GetIndex(This,puiIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,puiIndex) ) 

#define IMILBitmapEffectConnector_GetOptimalFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetOptimalFormat(This,pFormat) ) 

#define IMILBitmapEffectConnector_GetNumberFormats(This,pulNumberFormats)	\
    ( (This)->lpVtbl -> GetNumberFormats(This,pulNumberFormats) ) 

#define IMILBitmapEffectConnector_GetFormat(This,ulIndex,pFormat)	\
    ( (This)->lpVtbl -> GetFormat(This,ulIndex,pFormat) ) 


#define IMILBitmapEffectConnector_IsConnected(This,pfConnected)	\
    ( (This)->lpVtbl -> IsConnected(This,pfConnected) ) 

#define IMILBitmapEffectConnector_GetBitmapEffect(This,ppEffect)	\
    ( (This)->lpVtbl -> GetBitmapEffect(This,ppEffect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectConnector_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectInputConnector_INTERFACE_DEFINED__
#define __IMILBitmapEffectInputConnector_INTERFACE_DEFINED__

/* interface IMILBitmapEffectInputConnector */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectInputConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A9B4ECAA-7A3C-45e7-8573-F4B81B60DD6C")
    IMILBitmapEffectInputConnector : public IMILBitmapEffectConnector
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ConnectTo( 
            /* [in] */ __RPC__in_opt IMILBitmapEffectOutputConnector *pConnector) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnection( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **ppConnector) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectInputConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectInputConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectInputConnector * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [retval][out] */ __RPC__out ULONG *puiIndex);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetOptimalFormat)
        HRESULT ( STDMETHODCALLTYPE *GetOptimalFormat )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetNumberFormats)
        HRESULT ( STDMETHODCALLTYPE *GetNumberFormats )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [retval][out] */ __RPC__out ULONG *pulNumberFormats);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnector, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfConnected);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnector, GetBitmapEffect)
        HRESULT ( STDMETHODCALLTYPE *GetBitmapEffect )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectInputConnector, ConnectTo)
        HRESULT ( STDMETHODCALLTYPE *ConnectTo )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffectOutputConnector *pConnector);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectInputConnector, GetConnection)
        HRESULT ( STDMETHODCALLTYPE *GetConnection )( 
            __RPC__in IMILBitmapEffectInputConnector * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **ppConnector);
        
        END_INTERFACE
    } IMILBitmapEffectInputConnectorVtbl;

    interface IMILBitmapEffectInputConnector
    {
        CONST_VTBL struct IMILBitmapEffectInputConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectInputConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectInputConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectInputConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectInputConnector_GetIndex(This,puiIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,puiIndex) ) 

#define IMILBitmapEffectInputConnector_GetOptimalFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetOptimalFormat(This,pFormat) ) 

#define IMILBitmapEffectInputConnector_GetNumberFormats(This,pulNumberFormats)	\
    ( (This)->lpVtbl -> GetNumberFormats(This,pulNumberFormats) ) 

#define IMILBitmapEffectInputConnector_GetFormat(This,ulIndex,pFormat)	\
    ( (This)->lpVtbl -> GetFormat(This,ulIndex,pFormat) ) 


#define IMILBitmapEffectInputConnector_IsConnected(This,pfConnected)	\
    ( (This)->lpVtbl -> IsConnected(This,pfConnected) ) 

#define IMILBitmapEffectInputConnector_GetBitmapEffect(This,ppEffect)	\
    ( (This)->lpVtbl -> GetBitmapEffect(This,ppEffect) ) 


#define IMILBitmapEffectInputConnector_ConnectTo(This,pConnector)	\
    ( (This)->lpVtbl -> ConnectTo(This,pConnector) ) 

#define IMILBitmapEffectInputConnector_GetConnection(This,ppConnector)	\
    ( (This)->lpVtbl -> GetConnection(This,ppConnector) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectInputConnector_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectOutputConnector_INTERFACE_DEFINED__
#define __IMILBitmapEffectOutputConnector_INTERFACE_DEFINED__

/* interface IMILBitmapEffectOutputConnector */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectOutputConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92957AAD-841B-4866-82EC-8752468B07FD")
    IMILBitmapEffectOutputConnector : public IMILBitmapEffectConnector
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberConnections( 
            /* [retval][out] */ __RPC__out ULONG *puiNumberConnections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConnection( 
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **ppConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectOutputConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectOutputConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectOutputConnector * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIndex )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [retval][out] */ __RPC__out ULONG *puiIndex);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetOptimalFormat)
        HRESULT ( STDMETHODCALLTYPE *GetOptimalFormat )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetNumberFormats)
        HRESULT ( STDMETHODCALLTYPE *GetNumberFormats )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [retval][out] */ __RPC__out ULONG *pulNumberFormats);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnectorInfo, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [in] */ ULONG ulIndex,
            /* [retval][out] */ __RPC__out WICPixelFormatGUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnector, IsConnected)
        HRESULT ( STDMETHODCALLTYPE *IsConnected )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [retval][out] */ __RPC__out VARIANT_BOOL *pfConnected);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectConnector, GetBitmapEffect)
        HRESULT ( STDMETHODCALLTYPE *GetBitmapEffect )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffect **ppEffect);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectOutputConnector, GetNumberConnections)
        HRESULT ( STDMETHODCALLTYPE *GetNumberConnections )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [retval][out] */ __RPC__out ULONG *puiNumberConnections);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectOutputConnector, GetConnection)
        HRESULT ( STDMETHODCALLTYPE *GetConnection )( 
            __RPC__in IMILBitmapEffectOutputConnector * This,
            /* [in] */ ULONG uiIndex,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **ppConnection);
        
        END_INTERFACE
    } IMILBitmapEffectOutputConnectorVtbl;

    interface IMILBitmapEffectOutputConnector
    {
        CONST_VTBL struct IMILBitmapEffectOutputConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectOutputConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectOutputConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectOutputConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectOutputConnector_GetIndex(This,puiIndex)	\
    ( (This)->lpVtbl -> GetIndex(This,puiIndex) ) 

#define IMILBitmapEffectOutputConnector_GetOptimalFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetOptimalFormat(This,pFormat) ) 

#define IMILBitmapEffectOutputConnector_GetNumberFormats(This,pulNumberFormats)	\
    ( (This)->lpVtbl -> GetNumberFormats(This,pulNumberFormats) ) 

#define IMILBitmapEffectOutputConnector_GetFormat(This,ulIndex,pFormat)	\
    ( (This)->lpVtbl -> GetFormat(This,ulIndex,pFormat) ) 


#define IMILBitmapEffectOutputConnector_IsConnected(This,pfConnected)	\
    ( (This)->lpVtbl -> IsConnected(This,pfConnected) ) 

#define IMILBitmapEffectOutputConnector_GetBitmapEffect(This,ppEffect)	\
    ( (This)->lpVtbl -> GetBitmapEffect(This,ppEffect) ) 


#define IMILBitmapEffectOutputConnector_GetNumberConnections(This,puiNumberConnections)	\
    ( (This)->lpVtbl -> GetNumberConnections(This,puiNumberConnections) ) 

#define IMILBitmapEffectOutputConnector_GetConnection(This,uiIndex,ppConnection)	\
    ( (This)->lpVtbl -> GetConnection(This,uiIndex,ppConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectOutputConnector_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectOutputConnectorImpl_INTERFACE_DEFINED__
#define __IMILBitmapEffectOutputConnectorImpl_INTERFACE_DEFINED__

/* interface IMILBitmapEffectOutputConnectorImpl */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectOutputConnectorImpl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("21FAE777-8B39-4bfa-9F2D-F3941ED36913")
    IMILBitmapEffectOutputConnectorImpl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddBackLink( 
            /* [in] */ __RPC__in_opt IMILBitmapEffectInputConnector *pConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveBackLink( 
            /* [in] */ __RPC__in_opt IMILBitmapEffectInputConnector *pConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectOutputConnectorImplVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectOutputConnectorImpl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectOutputConnectorImpl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectOutputConnectorImpl * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectOutputConnectorImpl, AddBackLink)
        HRESULT ( STDMETHODCALLTYPE *AddBackLink )( 
            __RPC__in IMILBitmapEffectOutputConnectorImpl * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffectInputConnector *pConnection);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectOutputConnectorImpl, RemoveBackLink)
        HRESULT ( STDMETHODCALLTYPE *RemoveBackLink )( 
            __RPC__in IMILBitmapEffectOutputConnectorImpl * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffectInputConnector *pConnection);
        
        END_INTERFACE
    } IMILBitmapEffectOutputConnectorImplVtbl;

    interface IMILBitmapEffectOutputConnectorImpl
    {
        CONST_VTBL struct IMILBitmapEffectOutputConnectorImplVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectOutputConnectorImpl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectOutputConnectorImpl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectOutputConnectorImpl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectOutputConnectorImpl_AddBackLink(This,pConnection)	\
    ( (This)->lpVtbl -> AddBackLink(This,pConnection) ) 

#define IMILBitmapEffectOutputConnectorImpl_RemoveBackLink(This,pConnection)	\
    ( (This)->lpVtbl -> RemoveBackLink(This,pConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectOutputConnectorImpl_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectInteriorInputConnector_INTERFACE_DEFINED__
#define __IMILBitmapEffectInteriorInputConnector_INTERFACE_DEFINED__

/* interface IMILBitmapEffectInteriorInputConnector */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectInteriorInputConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20287E9E-86A2-4e15-953D-EB1438A5B842")
    IMILBitmapEffectInteriorInputConnector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInputConnector( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **pInputConnector) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectInteriorInputConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectInteriorInputConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectInteriorInputConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectInteriorInputConnector * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectInteriorInputConnector, GetInputConnector)
        HRESULT ( STDMETHODCALLTYPE *GetInputConnector )( 
            __RPC__in IMILBitmapEffectInteriorInputConnector * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectInputConnector **pInputConnector);
        
        END_INTERFACE
    } IMILBitmapEffectInteriorInputConnectorVtbl;

    interface IMILBitmapEffectInteriorInputConnector
    {
        CONST_VTBL struct IMILBitmapEffectInteriorInputConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectInteriorInputConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectInteriorInputConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectInteriorInputConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectInteriorInputConnector_GetInputConnector(This,pInputConnector)	\
    ( (This)->lpVtbl -> GetInputConnector(This,pInputConnector) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectInteriorInputConnector_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectInteriorOutputConnector_INTERFACE_DEFINED__
#define __IMILBitmapEffectInteriorOutputConnector_INTERFACE_DEFINED__

/* interface IMILBitmapEffectInteriorOutputConnector */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectInteriorOutputConnector;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00BBB6DC-ACC9-4bfc-B344-8BEE383DFEFA")
    IMILBitmapEffectInteriorOutputConnector : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOutputConnector( 
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **pOutputConnector) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectInteriorOutputConnectorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectInteriorOutputConnector * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectInteriorOutputConnector * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectInteriorOutputConnector * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectInteriorOutputConnector, GetOutputConnector)
        HRESULT ( STDMETHODCALLTYPE *GetOutputConnector )( 
            __RPC__in IMILBitmapEffectInteriorOutputConnector * This,
            /* [retval][out] */ __RPC__deref_out_opt IMILBitmapEffectOutputConnector **pOutputConnector);
        
        END_INTERFACE
    } IMILBitmapEffectInteriorOutputConnectorVtbl;

    interface IMILBitmapEffectInteriorOutputConnector
    {
        CONST_VTBL struct IMILBitmapEffectInteriorOutputConnectorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectInteriorOutputConnector_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectInteriorOutputConnector_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectInteriorOutputConnector_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectInteriorOutputConnector_GetOutputConnector(This,pOutputConnector)	\
    ( (This)->lpVtbl -> GetOutputConnector(This,pOutputConnector) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectInteriorOutputConnector_INTERFACE_DEFINED__ */


#ifndef __IMILBitmapEffectEvents_INTERFACE_DEFINED__
#define __IMILBitmapEffectEvents_INTERFACE_DEFINED__

/* interface IMILBitmapEffectEvents */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMILBitmapEffectEvents;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2E880DD8-F8CE-457b-8199-D60BB3D7EF98")
    IMILBitmapEffectEvents : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PropertyChange( 
            /* [in] */ __RPC__in_opt IMILBitmapEffect *pEffect,
            /* [in] */ __RPC__in BSTR bstrPropertyName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DirtyRegion( 
            /* [in] */ __RPC__in_opt IMILBitmapEffect *pEffect,
            /* [in] */ __RPC__in MilRectD *pRect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMILBitmapEffectEventsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMILBitmapEffectEvents * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMILBitmapEffectEvents * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMILBitmapEffectEvents * This);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectEvents, PropertyChange)
        HRESULT ( STDMETHODCALLTYPE *PropertyChange )( 
            __RPC__in IMILBitmapEffectEvents * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffect *pEffect,
            /* [in] */ __RPC__in BSTR bstrPropertyName);
        
        DECLSPEC_XFGVIRT(IMILBitmapEffectEvents, DirtyRegion)
        HRESULT ( STDMETHODCALLTYPE *DirtyRegion )( 
            __RPC__in IMILBitmapEffectEvents * This,
            /* [in] */ __RPC__in_opt IMILBitmapEffect *pEffect,
            /* [in] */ __RPC__in MilRectD *pRect);
        
        END_INTERFACE
    } IMILBitmapEffectEventsVtbl;

    interface IMILBitmapEffectEvents
    {
        CONST_VTBL struct IMILBitmapEffectEventsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMILBitmapEffectEvents_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMILBitmapEffectEvents_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMILBitmapEffectEvents_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMILBitmapEffectEvents_PropertyChange(This,pEffect,bstrPropertyName)	\
    ( (This)->lpVtbl -> PropertyChange(This,pEffect,bstrPropertyName) ) 

#define IMILBitmapEffectEvents_DirtyRegion(This,pEffect,pRect)	\
    ( (This)->lpVtbl -> DirtyRegion(This,pEffect,pRect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMILBitmapEffectEvents_INTERFACE_DEFINED__ */

#endif /* __MILEffects_LIBRARY_DEFINED__ */

/* interface __MIDL_itf_mileffects_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mileffects_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mileffects_0000_0001_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


