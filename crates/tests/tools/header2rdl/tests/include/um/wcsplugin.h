

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

#ifndef __wcsplugin_h__
#define __wcsplugin_h__

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

#ifndef __IDeviceModelPlugIn_FWD_DEFINED__
#define __IDeviceModelPlugIn_FWD_DEFINED__
typedef interface IDeviceModelPlugIn IDeviceModelPlugIn;

#endif 	/* __IDeviceModelPlugIn_FWD_DEFINED__ */


#ifndef __IGamutMapModelPlugIn_FWD_DEFINED__
#define __IGamutMapModelPlugIn_FWD_DEFINED__
typedef interface IGamutMapModelPlugIn IGamutMapModelPlugIn;

#endif 	/* __IGamutMapModelPlugIn_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_wcsplugin_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Copyright (c) Microsoft Corporation.  All rights reserved.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef struct _XYZColorF
    {
    FLOAT X;
    FLOAT Y;
    FLOAT Z;
    } 	XYZColorF;

typedef struct _JChColorF
    {
    FLOAT J;
    FLOAT C;
    FLOAT h;
    } 	JChColorF;

typedef struct _JabColorF
    {
    FLOAT J;
    FLOAT a;
    FLOAT b;
    } 	JabColorF;

typedef struct _GamutShellTriangle
    {
    UINT aVertexIndex[ 3 ];
    } 	GamutShellTriangle;

typedef struct _GamutShell
    {
    FLOAT JMin;
    FLOAT JMax;
    UINT cVertices;
    UINT cTriangles;
    /* [size_is] */ JabColorF *pVertices;
    /* [size_is] */ GamutShellTriangle *pTriangles;
    } 	GamutShell;

typedef struct _PrimaryJabColors
    {
    JabColorF red;
    JabColorF yellow;
    JabColorF green;
    JabColorF cyan;
    JabColorF blue;
    JabColorF magenta;
    JabColorF black;
    JabColorF white;
    } 	PrimaryJabColors;

typedef struct _PrimaryXYZColors
    {
    XYZColorF red;
    XYZColorF yellow;
    XYZColorF green;
    XYZColorF cyan;
    XYZColorF blue;
    XYZColorF magenta;
    XYZColorF black;
    XYZColorF white;
    } 	PrimaryXYZColors;

typedef struct _GamutBoundaryDescription
    {
    PrimaryJabColors *pPrimaries;
    UINT cNeutralSamples;
    /* [size_is] */ JabColorF *pNeutralSamples;
    GamutShell *pReferenceShell;
    GamutShell *pPlausibleShell;
    GamutShell *pPossibleShell;
    } 	GamutBoundaryDescription;

typedef struct _BlackInformation
    {
    BOOL fBlackOnly;
    FLOAT blackWeight;
    } 	BlackInformation;



extern RPC_IF_HANDLE __MIDL_itf_wcsplugin_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcsplugin_0000_0000_v0_0_s_ifspec;

#ifndef __IDeviceModelPlugIn_INTERFACE_DEFINED__
#define __IDeviceModelPlugIn_INTERFACE_DEFINED__

/* interface IDeviceModelPlugIn */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IDeviceModelPlugIn;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1CD63475-07C4-46FE-A903-D655316D11FD")
    IDeviceModelPlugIn : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in BSTR bstrXml,
            /* [in] */ UINT cNumModels,
            /* [in] */ UINT iModelPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumChannels( 
            /* [out] */ __RPC__out UINT *pNumChannels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeviceToColorimetricColors( 
            /* [in] */ UINT cColors,
            /* [in] */ UINT cChannels,
            /* [size_is][in] */ __RPC__in_ecount_full(( cColors * cChannels ) ) const FLOAT *pDeviceValues,
            /* [size_is][out] */ __RPC__out_ecount_full(cColors) XYZColorF *pXYZColors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ColorimetricToDeviceColors( 
            /* [in] */ UINT cColors,
            /* [in] */ UINT cChannels,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const XYZColorF *pXYZColors,
            /* [size_is][out] */ __RPC__out_ecount_full(( cColors * cChannels ) ) FLOAT *pDeviceValues) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ColorimetricToDeviceColorsWithBlack( 
            /* [in] */ UINT cColors,
            /* [in] */ UINT cChannels,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const XYZColorF *pXYZColors,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const BlackInformation *pBlackInformation,
            /* [size_is][out] */ __RPC__out_ecount_full(( cColors * cChannels ) ) FLOAT *pDeviceValues) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransformDeviceModelInfo( 
            /* [in] */ UINT iModelPosition,
            /* [in] */ __RPC__in_opt IDeviceModelPlugIn *pIDeviceModelOther) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrimarySamples( 
            /* [out] */ __RPC__out PrimaryXYZColors *pPrimaryColor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGamutBoundaryMeshSize( 
            /* [out] */ __RPC__out UINT *pNumVertices,
            /* [out] */ __RPC__out UINT *pNumTriangles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGamutBoundaryMesh( 
            /* [in] */ UINT cChannels,
            /* [in] */ UINT cVertices,
            /* [in] */ UINT cTriangles,
            /* [size_is][out] */ __RPC__out_ecount_full(( cVertices * cChannels ) ) FLOAT *pVertices,
            /* [size_is][out] */ __RPC__out_ecount_full(cTriangles) GamutShellTriangle *pTriangles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNeutralAxisSize( 
            /* [out] */ __RPC__out UINT *pcColors) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNeutralAxis( 
            /* [in] */ UINT cColors,
            /* [size_is][out] */ __RPC__out_ecount_full(cColors) XYZColorF *pXYZColors) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDeviceModelPlugInVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IDeviceModelPlugIn * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IDeviceModelPlugIn * This);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ __RPC__in BSTR bstrXml,
            /* [in] */ UINT cNumModels,
            /* [in] */ UINT iModelPosition);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, GetNumChannels)
        HRESULT ( STDMETHODCALLTYPE *GetNumChannels )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [out] */ __RPC__out UINT *pNumChannels);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, DeviceToColorimetricColors)
        HRESULT ( STDMETHODCALLTYPE *DeviceToColorimetricColors )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ UINT cColors,
            /* [in] */ UINT cChannels,
            /* [size_is][in] */ __RPC__in_ecount_full(( cColors * cChannels ) ) const FLOAT *pDeviceValues,
            /* [size_is][out] */ __RPC__out_ecount_full(cColors) XYZColorF *pXYZColors);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, ColorimetricToDeviceColors)
        HRESULT ( STDMETHODCALLTYPE *ColorimetricToDeviceColors )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ UINT cColors,
            /* [in] */ UINT cChannels,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const XYZColorF *pXYZColors,
            /* [size_is][out] */ __RPC__out_ecount_full(( cColors * cChannels ) ) FLOAT *pDeviceValues);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, ColorimetricToDeviceColorsWithBlack)
        HRESULT ( STDMETHODCALLTYPE *ColorimetricToDeviceColorsWithBlack )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ UINT cColors,
            /* [in] */ UINT cChannels,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const XYZColorF *pXYZColors,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const BlackInformation *pBlackInformation,
            /* [size_is][out] */ __RPC__out_ecount_full(( cColors * cChannels ) ) FLOAT *pDeviceValues);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, SetTransformDeviceModelInfo)
        HRESULT ( STDMETHODCALLTYPE *SetTransformDeviceModelInfo )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ UINT iModelPosition,
            /* [in] */ __RPC__in_opt IDeviceModelPlugIn *pIDeviceModelOther);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, GetPrimarySamples)
        HRESULT ( STDMETHODCALLTYPE *GetPrimarySamples )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [out] */ __RPC__out PrimaryXYZColors *pPrimaryColor);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, GetGamutBoundaryMeshSize)
        HRESULT ( STDMETHODCALLTYPE *GetGamutBoundaryMeshSize )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [out] */ __RPC__out UINT *pNumVertices,
            /* [out] */ __RPC__out UINT *pNumTriangles);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, GetGamutBoundaryMesh)
        HRESULT ( STDMETHODCALLTYPE *GetGamutBoundaryMesh )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ UINT cChannels,
            /* [in] */ UINT cVertices,
            /* [in] */ UINT cTriangles,
            /* [size_is][out] */ __RPC__out_ecount_full(( cVertices * cChannels ) ) FLOAT *pVertices,
            /* [size_is][out] */ __RPC__out_ecount_full(cTriangles) GamutShellTriangle *pTriangles);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, GetNeutralAxisSize)
        HRESULT ( STDMETHODCALLTYPE *GetNeutralAxisSize )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [out] */ __RPC__out UINT *pcColors);
        
        DECLSPEC_XFGVIRT(IDeviceModelPlugIn, GetNeutralAxis)
        HRESULT ( STDMETHODCALLTYPE *GetNeutralAxis )( 
            __RPC__in IDeviceModelPlugIn * This,
            /* [in] */ UINT cColors,
            /* [size_is][out] */ __RPC__out_ecount_full(cColors) XYZColorF *pXYZColors);
        
        END_INTERFACE
    } IDeviceModelPlugInVtbl;

    interface IDeviceModelPlugIn
    {
        CONST_VTBL struct IDeviceModelPlugInVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDeviceModelPlugIn_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDeviceModelPlugIn_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDeviceModelPlugIn_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDeviceModelPlugIn_Initialize(This,bstrXml,cNumModels,iModelPosition)	\
    ( (This)->lpVtbl -> Initialize(This,bstrXml,cNumModels,iModelPosition) ) 

#define IDeviceModelPlugIn_GetNumChannels(This,pNumChannels)	\
    ( (This)->lpVtbl -> GetNumChannels(This,pNumChannels) ) 

#define IDeviceModelPlugIn_DeviceToColorimetricColors(This,cColors,cChannels,pDeviceValues,pXYZColors)	\
    ( (This)->lpVtbl -> DeviceToColorimetricColors(This,cColors,cChannels,pDeviceValues,pXYZColors) ) 

#define IDeviceModelPlugIn_ColorimetricToDeviceColors(This,cColors,cChannels,pXYZColors,pDeviceValues)	\
    ( (This)->lpVtbl -> ColorimetricToDeviceColors(This,cColors,cChannels,pXYZColors,pDeviceValues) ) 

#define IDeviceModelPlugIn_ColorimetricToDeviceColorsWithBlack(This,cColors,cChannels,pXYZColors,pBlackInformation,pDeviceValues)	\
    ( (This)->lpVtbl -> ColorimetricToDeviceColorsWithBlack(This,cColors,cChannels,pXYZColors,pBlackInformation,pDeviceValues) ) 

#define IDeviceModelPlugIn_SetTransformDeviceModelInfo(This,iModelPosition,pIDeviceModelOther)	\
    ( (This)->lpVtbl -> SetTransformDeviceModelInfo(This,iModelPosition,pIDeviceModelOther) ) 

#define IDeviceModelPlugIn_GetPrimarySamples(This,pPrimaryColor)	\
    ( (This)->lpVtbl -> GetPrimarySamples(This,pPrimaryColor) ) 

#define IDeviceModelPlugIn_GetGamutBoundaryMeshSize(This,pNumVertices,pNumTriangles)	\
    ( (This)->lpVtbl -> GetGamutBoundaryMeshSize(This,pNumVertices,pNumTriangles) ) 

#define IDeviceModelPlugIn_GetGamutBoundaryMesh(This,cChannels,cVertices,cTriangles,pVertices,pTriangles)	\
    ( (This)->lpVtbl -> GetGamutBoundaryMesh(This,cChannels,cVertices,cTriangles,pVertices,pTriangles) ) 

#define IDeviceModelPlugIn_GetNeutralAxisSize(This,pcColors)	\
    ( (This)->lpVtbl -> GetNeutralAxisSize(This,pcColors) ) 

#define IDeviceModelPlugIn_GetNeutralAxis(This,cColors,pXYZColors)	\
    ( (This)->lpVtbl -> GetNeutralAxis(This,cColors,pXYZColors) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDeviceModelPlugIn_INTERFACE_DEFINED__ */


#ifndef __IGamutMapModelPlugIn_INTERFACE_DEFINED__
#define __IGamutMapModelPlugIn_INTERFACE_DEFINED__

/* interface IGamutMapModelPlugIn */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IGamutMapModelPlugIn;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2DD80115-AD1E-41F6-A219-A4F4B583D1F9")
    IGamutMapModelPlugIn : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ __RPC__in BSTR bstrXml,
            /* [in] */ __RPC__in_opt IDeviceModelPlugIn *pSrcPlugIn,
            /* [in] */ __RPC__in_opt IDeviceModelPlugIn *pDestPlugIn,
            /* [in] */ __RPC__in GamutBoundaryDescription *pSrcGBD,
            /* [in] */ __RPC__in GamutBoundaryDescription *pDestGBD) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SourceToDestinationAppearanceColors( 
            /* [in] */ UINT cColors,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const JChColorF *pInputColors,
            /* [size_is][out] */ __RPC__out_ecount_full(cColors) JChColorF *pOutputColors) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGamutMapModelPlugInVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IGamutMapModelPlugIn * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IGamutMapModelPlugIn * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IGamutMapModelPlugIn * This);
        
        DECLSPEC_XFGVIRT(IGamutMapModelPlugIn, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IGamutMapModelPlugIn * This,
            /* [in] */ __RPC__in BSTR bstrXml,
            /* [in] */ __RPC__in_opt IDeviceModelPlugIn *pSrcPlugIn,
            /* [in] */ __RPC__in_opt IDeviceModelPlugIn *pDestPlugIn,
            /* [in] */ __RPC__in GamutBoundaryDescription *pSrcGBD,
            /* [in] */ __RPC__in GamutBoundaryDescription *pDestGBD);
        
        DECLSPEC_XFGVIRT(IGamutMapModelPlugIn, SourceToDestinationAppearanceColors)
        HRESULT ( STDMETHODCALLTYPE *SourceToDestinationAppearanceColors )( 
            __RPC__in IGamutMapModelPlugIn * This,
            /* [in] */ UINT cColors,
            /* [size_is][in] */ __RPC__in_ecount_full(cColors) const JChColorF *pInputColors,
            /* [size_is][out] */ __RPC__out_ecount_full(cColors) JChColorF *pOutputColors);
        
        END_INTERFACE
    } IGamutMapModelPlugInVtbl;

    interface IGamutMapModelPlugIn
    {
        CONST_VTBL struct IGamutMapModelPlugInVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGamutMapModelPlugIn_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGamutMapModelPlugIn_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGamutMapModelPlugIn_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGamutMapModelPlugIn_Initialize(This,bstrXml,pSrcPlugIn,pDestPlugIn,pSrcGBD,pDestGBD)	\
    ( (This)->lpVtbl -> Initialize(This,bstrXml,pSrcPlugIn,pDestPlugIn,pSrcGBD,pDestGBD) ) 

#define IGamutMapModelPlugIn_SourceToDestinationAppearanceColors(This,cColors,pInputColors,pOutputColors)	\
    ( (This)->lpVtbl -> SourceToDestinationAppearanceColors(This,cColors,pInputColors,pOutputColors) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGamutMapModelPlugIn_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_wcsplugin_0000_0002 */
/* [local] */ 

DEFINE_GUID(CATID_WcsPlugin, 0xa0b402e0, 0x8240, 0x405f, 0x8a, 0x16, 0x8a, 0x5b, 0x4d, 0xf2, 0xf0, 0xdd);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_wcsplugin_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_wcsplugin_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


