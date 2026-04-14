

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

#ifndef __spatialaudiohrtf_h__
#define __spatialaudiohrtf_h__

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

#ifndef __ISpatialAudioObjectForHrtf_FWD_DEFINED__
#define __ISpatialAudioObjectForHrtf_FWD_DEFINED__
typedef interface ISpatialAudioObjectForHrtf ISpatialAudioObjectForHrtf;

#endif 	/* __ISpatialAudioObjectForHrtf_FWD_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamForHrtf_FWD_DEFINED__
#define __ISpatialAudioObjectRenderStreamForHrtf_FWD_DEFINED__
typedef interface ISpatialAudioObjectRenderStreamForHrtf ISpatialAudioObjectRenderStreamForHrtf;

#endif 	/* __ISpatialAudioObjectRenderStreamForHrtf_FWD_DEFINED__ */


/* header files for imported files */
#include "wtypes.h"
#include "unknwn.h"
#include "hstring.h"
#include "SpatialAudioClient.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_spatialaudiohrtf_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Application and Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum SpatialAudioHrtfDirectivityType
    {
        SpatialAudioHrtfDirectivity_OmniDirectional	= 0,
        SpatialAudioHrtfDirectivity_Cardioid	= ( SpatialAudioHrtfDirectivity_OmniDirectional + 1 ) ,
        SpatialAudioHrtfDirectivity_Cone	= ( SpatialAudioHrtfDirectivity_Cardioid + 1 ) 
    } 	SpatialAudioHrtfDirectivityType;

typedef 
enum SpatialAudioHrtfEnvironmentType
    {
        SpatialAudioHrtfEnvironment_Small	= 0,
        SpatialAudioHrtfEnvironment_Medium	= ( SpatialAudioHrtfEnvironment_Small + 1 ) ,
        SpatialAudioHrtfEnvironment_Large	= ( SpatialAudioHrtfEnvironment_Medium + 1 ) ,
        SpatialAudioHrtfEnvironment_Outdoors	= ( SpatialAudioHrtfEnvironment_Large + 1 ) ,
        SpatialAudioHrtfEnvironment_Average	= ( SpatialAudioHrtfEnvironment_Outdoors + 1 ) 
    } 	SpatialAudioHrtfEnvironmentType;

typedef 
enum SpatialAudioHrtfDistanceDecayType
    {
        SpatialAudioHrtfDistanceDecay_NaturalDecay	= 0,
        SpatialAudioHrtfDistanceDecay_CustomDecay	= ( SpatialAudioHrtfDistanceDecay_NaturalDecay + 1 ) 
    } 	SpatialAudioHrtfDistanceDecayType;


#pragma pack(push, 1)
typedef struct SpatialAudioHrtfDirectivity
    {
    SpatialAudioHrtfDirectivityType Type;
    float Scaling;
    } 	SpatialAudioHrtfDirectivity;

typedef struct SpatialAudioHrtfDirectivityCardioid
    {
    SpatialAudioHrtfDirectivity directivity;
    float Order;
    } 	SpatialAudioHrtfDirectivityCardioid;

typedef struct SpatialAudioHrtfDirectivityCone
    {
    SpatialAudioHrtfDirectivity directivity;
    float InnerAngle;
    float OuterAngle;
    } 	SpatialAudioHrtfDirectivityCone;

typedef union SpatialAudioHrtfDirectivityUnion
    {
    SpatialAudioHrtfDirectivityCone Cone;
    SpatialAudioHrtfDirectivityCardioid Cardiod;
    SpatialAudioHrtfDirectivity Omni;
    } 	SpatialAudioHrtfDirectivityUnion;

typedef struct SpatialAudioHrtfDistanceDecay
    {
    SpatialAudioHrtfDistanceDecayType Type;
    float MaxGain;
    float MinGain;
    float UnityGainDistance;
    float CutoffDistance;
    } 	SpatialAudioHrtfDistanceDecay;

typedef float SpatialAudioHrtfOrientation[ 9 ];

typedef struct SpatialAudioHrtfActivationParams
    {
    const WAVEFORMATEX *ObjectFormat;
    AudioObjectType StaticObjectTypeMask;
    UINT32 MinDynamicObjectCount;
    UINT32 MaxDynamicObjectCount;
    AUDIO_STREAM_CATEGORY Category;
    HANDLE EventHandle;
    ISpatialAudioObjectRenderStreamNotify *NotifyObject;
    SpatialAudioHrtfDistanceDecay *DistanceDecay;
    SpatialAudioHrtfDirectivityUnion *Directivity;
    SpatialAudioHrtfEnvironmentType *Environment;
    SpatialAudioHrtfOrientation *Orientation;
    } 	SpatialAudioHrtfActivationParams;

typedef struct SpatialAudioHrtfActivationParams2
    {
    const WAVEFORMATEX *ObjectFormat;
    AudioObjectType StaticObjectTypeMask;
    UINT32 MinDynamicObjectCount;
    UINT32 MaxDynamicObjectCount;
    AUDIO_STREAM_CATEGORY Category;
    HANDLE EventHandle;
    ISpatialAudioObjectRenderStreamNotify *NotifyObject;
    SpatialAudioHrtfDistanceDecay *DistanceDecay;
    SpatialAudioHrtfDirectivityUnion *Directivity;
    SpatialAudioHrtfEnvironmentType *Environment;
    SpatialAudioHrtfOrientation *Orientation;
    SPATIAL_AUDIO_STREAM_OPTIONS Options;
    } 	SpatialAudioHrtfActivationParams2;


#pragma pack(pop)


extern RPC_IF_HANDLE __MIDL_itf_spatialaudiohrtf_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spatialaudiohrtf_0000_0000_v0_0_s_ifspec;

#ifndef __ISpatialAudioObjectForHrtf_INTERFACE_DEFINED__
#define __ISpatialAudioObjectForHrtf_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectForHrtf */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectForHrtf;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D7436ADE-1978-4E14-ABA0-555BD8EB83B4")
    ISpatialAudioObjectForHrtf : public ISpatialAudioObjectBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPosition( 
            /* [annotation][in] */ 
            _In_  float x,
            /* [annotation][in] */ 
            _In_  float y,
            /* [annotation][in] */ 
            _In_  float z) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGain( 
            /* [annotation][in] */ 
            _In_  float gain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOrientation( 
            /* [annotation][in] */ 
            _In_  const SpatialAudioHrtfOrientation *orientation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEnvironment( 
            /* [annotation][in] */ 
            _In_  SpatialAudioHrtfEnvironmentType environment) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDistanceDecay( 
            /* [annotation][in] */ 
            _In_  SpatialAudioHrtfDistanceDecay *distanceDecay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDirectivity( 
            /* [annotation][in] */ 
            _In_  SpatialAudioHrtfDirectivityUnion *directivity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectForHrtfVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectForHrtf * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectForHrtf * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectForHrtf * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*bufferLength)  BYTE **buffer,
            /* [annotation][out] */ 
            _Out_  UINT32 *bufferLength);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, SetEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SetEndOfStream )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  UINT32 frameCount);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, IsActive)
        HRESULT ( STDMETHODCALLTYPE *IsActive )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][out] */ 
            _Out_  BOOL *isActive);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectBase, GetAudioObjectType)
        HRESULT ( STDMETHODCALLTYPE *GetAudioObjectType )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][out] */ 
            _Out_  AudioObjectType *audioObjectType);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForHrtf, SetPosition)
        HRESULT ( STDMETHODCALLTYPE *SetPosition )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  float x,
            /* [annotation][in] */ 
            _In_  float y,
            /* [annotation][in] */ 
            _In_  float z);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForHrtf, SetGain)
        HRESULT ( STDMETHODCALLTYPE *SetGain )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  float gain);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForHrtf, SetOrientation)
        HRESULT ( STDMETHODCALLTYPE *SetOrientation )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  const SpatialAudioHrtfOrientation *orientation);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForHrtf, SetEnvironment)
        HRESULT ( STDMETHODCALLTYPE *SetEnvironment )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  SpatialAudioHrtfEnvironmentType environment);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForHrtf, SetDistanceDecay)
        HRESULT ( STDMETHODCALLTYPE *SetDistanceDecay )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  SpatialAudioHrtfDistanceDecay *distanceDecay);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectForHrtf, SetDirectivity)
        HRESULT ( STDMETHODCALLTYPE *SetDirectivity )( 
            ISpatialAudioObjectForHrtf * This,
            /* [annotation][in] */ 
            _In_  SpatialAudioHrtfDirectivityUnion *directivity);
        
        END_INTERFACE
    } ISpatialAudioObjectForHrtfVtbl;

    interface ISpatialAudioObjectForHrtf
    {
        CONST_VTBL struct ISpatialAudioObjectForHrtfVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectForHrtf_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectForHrtf_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectForHrtf_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectForHrtf_GetBuffer(This,buffer,bufferLength)	\
    ( (This)->lpVtbl -> GetBuffer(This,buffer,bufferLength) ) 

#define ISpatialAudioObjectForHrtf_SetEndOfStream(This,frameCount)	\
    ( (This)->lpVtbl -> SetEndOfStream(This,frameCount) ) 

#define ISpatialAudioObjectForHrtf_IsActive(This,isActive)	\
    ( (This)->lpVtbl -> IsActive(This,isActive) ) 

#define ISpatialAudioObjectForHrtf_GetAudioObjectType(This,audioObjectType)	\
    ( (This)->lpVtbl -> GetAudioObjectType(This,audioObjectType) ) 


#define ISpatialAudioObjectForHrtf_SetPosition(This,x,y,z)	\
    ( (This)->lpVtbl -> SetPosition(This,x,y,z) ) 

#define ISpatialAudioObjectForHrtf_SetGain(This,gain)	\
    ( (This)->lpVtbl -> SetGain(This,gain) ) 

#define ISpatialAudioObjectForHrtf_SetOrientation(This,orientation)	\
    ( (This)->lpVtbl -> SetOrientation(This,orientation) ) 

#define ISpatialAudioObjectForHrtf_SetEnvironment(This,environment)	\
    ( (This)->lpVtbl -> SetEnvironment(This,environment) ) 

#define ISpatialAudioObjectForHrtf_SetDistanceDecay(This,distanceDecay)	\
    ( (This)->lpVtbl -> SetDistanceDecay(This,distanceDecay) ) 

#define ISpatialAudioObjectForHrtf_SetDirectivity(This,directivity)	\
    ( (This)->lpVtbl -> SetDirectivity(This,directivity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectForHrtf_INTERFACE_DEFINED__ */


#ifndef __ISpatialAudioObjectRenderStreamForHrtf_INTERFACE_DEFINED__
#define __ISpatialAudioObjectRenderStreamForHrtf_INTERFACE_DEFINED__

/* interface ISpatialAudioObjectRenderStreamForHrtf */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_ISpatialAudioObjectRenderStreamForHrtf;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E08DEEF9-5363-406E-9FDC-080EE247BBE0")
    ISpatialAudioObjectRenderStreamForHrtf : public ISpatialAudioObjectRenderStreamBase
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ActivateSpatialAudioObjectForHrtf( 
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObjectForHrtf **audioObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISpatialAudioObjectRenderStreamForHrtfVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISpatialAudioObjectRenderStreamForHrtf * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISpatialAudioObjectRenderStreamForHrtf * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISpatialAudioObjectRenderStreamForHrtf * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetAvailableDynamicObjectCount)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableDynamicObjectCount )( 
            ISpatialAudioObjectRenderStreamForHrtf * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *value);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            ISpatialAudioObjectRenderStreamForHrtf * This,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **service);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            ISpatialAudioObjectRenderStreamForHrtf * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            ISpatialAudioObjectRenderStreamForHrtf * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            ISpatialAudioObjectRenderStreamForHrtf * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, BeginUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *BeginUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStreamForHrtf * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *availableDynamicObjectCount,
            /* [annotation][out] */ 
            _Out_  UINT32 *frameCountPerBuffer);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamBase, EndUpdatingAudioObjects)
        HRESULT ( STDMETHODCALLTYPE *EndUpdatingAudioObjects )( 
            ISpatialAudioObjectRenderStreamForHrtf * This);
        
        DECLSPEC_XFGVIRT(ISpatialAudioObjectRenderStreamForHrtf, ActivateSpatialAudioObjectForHrtf)
        HRESULT ( STDMETHODCALLTYPE *ActivateSpatialAudioObjectForHrtf )( 
            ISpatialAudioObjectRenderStreamForHrtf * This,
            /* [annotation][in] */ 
            _In_  AudioObjectType type,
            /* [annotation][out] */ 
            _COM_Outptr_  ISpatialAudioObjectForHrtf **audioObject);
        
        END_INTERFACE
    } ISpatialAudioObjectRenderStreamForHrtfVtbl;

    interface ISpatialAudioObjectRenderStreamForHrtf
    {
        CONST_VTBL struct ISpatialAudioObjectRenderStreamForHrtfVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISpatialAudioObjectRenderStreamForHrtf_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISpatialAudioObjectRenderStreamForHrtf_GetAvailableDynamicObjectCount(This,value)	\
    ( (This)->lpVtbl -> GetAvailableDynamicObjectCount(This,value) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_GetService(This,riid,service)	\
    ( (This)->lpVtbl -> GetService(This,riid,service) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer)	\
    ( (This)->lpVtbl -> BeginUpdatingAudioObjects(This,availableDynamicObjectCount,frameCountPerBuffer) ) 

#define ISpatialAudioObjectRenderStreamForHrtf_EndUpdatingAudioObjects(This)	\
    ( (This)->lpVtbl -> EndUpdatingAudioObjects(This) ) 


#define ISpatialAudioObjectRenderStreamForHrtf_ActivateSpatialAudioObjectForHrtf(This,type,audioObject)	\
    ( (This)->lpVtbl -> ActivateSpatialAudioObjectForHrtf(This,type,audioObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISpatialAudioObjectRenderStreamForHrtf_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_spatialaudiohrtf_0000_0002 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */


extern RPC_IF_HANDLE __MIDL_itf_spatialaudiohrtf_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_spatialaudiohrtf_0000_0002_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


