

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

#ifndef __audioenginebaseapo_h__
#define __audioenginebaseapo_h__

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

#ifndef __IAudioProcessingObjectRT_FWD_DEFINED__
#define __IAudioProcessingObjectRT_FWD_DEFINED__
typedef interface IAudioProcessingObjectRT IAudioProcessingObjectRT;

#endif 	/* __IAudioProcessingObjectRT_FWD_DEFINED__ */


#ifndef __IAudioProcessingObjectVBR_FWD_DEFINED__
#define __IAudioProcessingObjectVBR_FWD_DEFINED__
typedef interface IAudioProcessingObjectVBR IAudioProcessingObjectVBR;

#endif 	/* __IAudioProcessingObjectVBR_FWD_DEFINED__ */


#ifndef __IAudioProcessingObjectConfiguration_FWD_DEFINED__
#define __IAudioProcessingObjectConfiguration_FWD_DEFINED__
typedef interface IAudioProcessingObjectConfiguration IAudioProcessingObjectConfiguration;

#endif 	/* __IAudioProcessingObjectConfiguration_FWD_DEFINED__ */


#ifndef __IAudioProcessingObject_FWD_DEFINED__
#define __IAudioProcessingObject_FWD_DEFINED__
typedef interface IAudioProcessingObject IAudioProcessingObject;

#endif 	/* __IAudioProcessingObject_FWD_DEFINED__ */


#ifndef __IAudioDeviceModulesClient_FWD_DEFINED__
#define __IAudioDeviceModulesClient_FWD_DEFINED__
typedef interface IAudioDeviceModulesClient IAudioDeviceModulesClient;

#endif 	/* __IAudioDeviceModulesClient_FWD_DEFINED__ */


#ifndef __IAudioSystemEffects_FWD_DEFINED__
#define __IAudioSystemEffects_FWD_DEFINED__
typedef interface IAudioSystemEffects IAudioSystemEffects;

#endif 	/* __IAudioSystemEffects_FWD_DEFINED__ */


#ifndef __IAudioSystemEffects2_FWD_DEFINED__
#define __IAudioSystemEffects2_FWD_DEFINED__
typedef interface IAudioSystemEffects2 IAudioSystemEffects2;

#endif 	/* __IAudioSystemEffects2_FWD_DEFINED__ */


#ifndef __IAudioSystemEffectsCustomFormats_FWD_DEFINED__
#define __IAudioSystemEffectsCustomFormats_FWD_DEFINED__
typedef interface IAudioSystemEffectsCustomFormats IAudioSystemEffectsCustomFormats;

#endif 	/* __IAudioSystemEffectsCustomFormats_FWD_DEFINED__ */


#ifndef __IApoAuxiliaryInputConfiguration_FWD_DEFINED__
#define __IApoAuxiliaryInputConfiguration_FWD_DEFINED__
typedef interface IApoAuxiliaryInputConfiguration IApoAuxiliaryInputConfiguration;

#endif 	/* __IApoAuxiliaryInputConfiguration_FWD_DEFINED__ */


#ifndef __IApoAuxiliaryInputRT_FWD_DEFINED__
#define __IApoAuxiliaryInputRT_FWD_DEFINED__
typedef interface IApoAuxiliaryInputRT IApoAuxiliaryInputRT;

#endif 	/* __IApoAuxiliaryInputRT_FWD_DEFINED__ */


#ifndef __IApoAcousticEchoCancellation_FWD_DEFINED__
#define __IApoAcousticEchoCancellation_FWD_DEFINED__
typedef interface IApoAcousticEchoCancellation IApoAcousticEchoCancellation;

#endif 	/* __IApoAcousticEchoCancellation_FWD_DEFINED__ */


#ifndef __IApoAcousticEchoCancellation2_FWD_DEFINED__
#define __IApoAcousticEchoCancellation2_FWD_DEFINED__
typedef interface IApoAcousticEchoCancellation2 IApoAcousticEchoCancellation2;

#endif 	/* __IApoAcousticEchoCancellation2_FWD_DEFINED__ */


/* header files for imported files */
#include "oaidl.h"
#include "ocidl.h"
#include "mmdeviceapi.h"
#include "audiomediatype.h"
#include "AudioAPOTypes.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_audioenginebaseapo_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
//
// Error Codes for APO
// The facility code for APOs is 0x87D.
//
// The object has already been initialized.
#define APOERR_ALREADY_INITIALIZED               _HRESULT_TYPEDEF_(0x887D0001)
// Object/structure is not initialized.
#define APOERR_NOT_INITIALIZED                   _HRESULT_TYPEDEF_(0x887D0002)
// a pin supporting the format cannot be found.
#define APOERR_FORMAT_NOT_SUPPORTED              _HRESULT_TYPEDEF_(0x887D0003)
// Invalid CLSID in an APO Initialization structure
#define APOERR_INVALID_APO_CLSID                 _HRESULT_TYPEDEF_(0x887D0004)
// Buffers overlap on an APO that does not accept in-place buffers.
#define APOERR_BUFFERS_OVERLAP                   _HRESULT_TYPEDEF_(0x887D0005)
// APO is already in an unlocked state
#define APOERR_ALREADY_UNLOCKED                  _HRESULT_TYPEDEF_(0x887D0006)
// number of input or output connections not valid on this APO
#define APOERR_NUM_CONNECTIONS_INVALID           _HRESULT_TYPEDEF_(0x887D0007)
// Output maxFrameCount not large enough.
#define APOERR_INVALID_OUTPUT_MAXFRAMECOUNT      _HRESULT_TYPEDEF_(0x887D0008)
// Invalid connection format for this operation
#define APOERR_INVALID_CONNECTION_FORMAT         _HRESULT_TYPEDEF_(0x887D0009)
// APO is locked ready to process and can not be changed
#define APOERR_APO_LOCKED                        _HRESULT_TYPEDEF_(0x887D000A)
// Invalid coefficient count
#define APOERR_INVALID_COEFFCOUNT                _HRESULT_TYPEDEF_(0x887D000B)
// Invalid coefficient
#define APOERR_INVALID_COEFFICIENT               _HRESULT_TYPEDEF_(0x887D000C)
// an invalid curve parameter was specified
#define APOERR_INVALID_CURVE_PARAM               _HRESULT_TYPEDEF_(0x887D000D)
// Invalid auxiliary input index
#define APOERR_INVALID_INPUTID                   _HRESULT_TYPEDEF_(0x887D000E)
//
// Signatures for data structures.
//
#define APO_CONNECTION_DESCRIPTOR_SIGNATURE     'ACDS'
#define APO_CONNECTION_PROPERTY_SIGNATURE       'ACPS'
#define APO_CONNECTION_PROPERTY_V2_SIGNATURE    'ACP2'

// Min and max framerates for the engine
#define AUDIO_MIN_FRAMERATE                    10.0     // Minimum frame rate for APOs
#define AUDIO_MAX_FRAMERATE                     384000.0 // Maximum frame rate for APOs

// Min and max # of channels (samples per frame) for the APOs
#define AUDIO_MIN_CHANNELS                      1    // Current minimum number of channels for APOs
#define AUDIO_MAX_CHANNELS                      4096 // Current maximum number of channels for APOs

//-----------------------------------------------------------------------------

typedef 
enum APO_CONNECTION_BUFFER_TYPE
    {
        APO_CONNECTION_BUFFER_TYPE_ALLOCATED	= 0,
        APO_CONNECTION_BUFFER_TYPE_EXTERNAL	= 1,
        APO_CONNECTION_BUFFER_TYPE_DEPENDANT	= 2
    } 	APO_CONNECTION_BUFFER_TYPE;

typedef struct APO_CONNECTION_DESCRIPTOR
    {
    APO_CONNECTION_BUFFER_TYPE Type;
    UINT_PTR pBuffer;
    UINT32 u32MaxFrameCount;
    IAudioMediaType *pFormat;
    UINT32 u32Signature;
    } 	APO_CONNECTION_DESCRIPTOR;

typedef 
enum APO_FLAG
    {
        APO_FLAG_NONE	= 0,
        APO_FLAG_INPLACE	= 0x1,
        APO_FLAG_SAMPLESPERFRAME_MUST_MATCH	= 0x2,
        APO_FLAG_FRAMESPERSECOND_MUST_MATCH	= 0x4,
        APO_FLAG_BITSPERSAMPLE_MUST_MATCH	= 0x8,
        APO_FLAG_MIXER	= 0x10,
        APO_FLAG_DEFAULT	= ( ( APO_FLAG_SAMPLESPERFRAME_MUST_MATCH | APO_FLAG_FRAMESPERSECOND_MUST_MATCH )  | APO_FLAG_BITSPERSAMPLE_MUST_MATCH ) 
    } 	APO_FLAG;

typedef struct APO_REG_PROPERTIES
    {
    CLSID clsid;
    APO_FLAG Flags;
    WCHAR szFriendlyName[ 256 ];
    WCHAR szCopyrightInfo[ 256 ];
    UINT32 u32MajorVersion;
    UINT32 u32MinorVersion;
    UINT32 u32MinInputConnections;
    UINT32 u32MaxInputConnections;
    UINT32 u32MinOutputConnections;
    UINT32 u32MaxOutputConnections;
    UINT32 u32MaxInstances;
    UINT32 u32NumAPOInterfaces;
    /* [size_is] */ IID iidAPOInterfaceList[ 1 ];
    } 	APO_REG_PROPERTIES;

typedef struct APO_REG_PROPERTIES *PAPO_REG_PROPERTIES;

typedef struct APOInitBaseStruct
    {
    UINT32 cbSize;
    CLSID clsid;
    } 	APOInitBaseStruct;

typedef 
enum AUDIO_FLOW_TYPE
    {
        AUDIO_FLOW_PULL	= 0,
        AUDIO_FLOW_PUSH	= ( AUDIO_FLOW_PULL + 1 ) 
    } 	AUDIO_FLOW_TYPE;

typedef 
enum EAudioConstriction
    {
        eAudioConstrictionOff	= 0,
        eAudioConstriction48_16	= ( eAudioConstrictionOff + 1 ) ,
        eAudioConstriction44_16	= ( eAudioConstriction48_16 + 1 ) ,
        eAudioConstriction14_14	= ( eAudioConstriction44_16 + 1 ) ,
        eAudioConstrictionMute	= ( eAudioConstriction14_14 + 1 ) 
    } 	EAudioConstriction;



extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0000_v0_0_s_ifspec;

#ifndef __IAudioProcessingObjectRT_INTERFACE_DEFINED__
#define __IAudioProcessingObjectRT_INTERFACE_DEFINED__

/* interface IAudioProcessingObjectRT */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioProcessingObjectRT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9E1D6A6D-DDBC-4E95-A4C7-AD64BA37846C")
    IAudioProcessingObjectRT : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE APOProcess( 
            /* [annotation][in] */ 
            _In_  UINT32 u32NumInputConnections,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_PROPERTY **ppInputConnections,
            /* [annotation][in] */ 
            _In_  UINT32 u32NumOutputConnections,
            /* [annotation][out][in] */ 
            _Inout_  APO_CONNECTION_PROPERTY **ppOutputConnections) = 0;
        
        virtual UINT32 STDMETHODCALLTYPE CalcInputFrames( 
            /* [in] */ UINT32 u32OutputFrameCount) = 0;
        
        virtual UINT32 STDMETHODCALLTYPE CalcOutputFrames( 
            /* [annotation][in] */ 
            _In_  UINT32 u32InputFrameCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioProcessingObjectRTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioProcessingObjectRT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioProcessingObjectRT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioProcessingObjectRT * This);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectRT, APOProcess)
        void ( STDMETHODCALLTYPE *APOProcess )( 
            IAudioProcessingObjectRT * This,
            /* [annotation][in] */ 
            _In_  UINT32 u32NumInputConnections,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_PROPERTY **ppInputConnections,
            /* [annotation][in] */ 
            _In_  UINT32 u32NumOutputConnections,
            /* [annotation][out][in] */ 
            _Inout_  APO_CONNECTION_PROPERTY **ppOutputConnections);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectRT, CalcInputFrames)
        UINT32 ( STDMETHODCALLTYPE *CalcInputFrames )( 
            IAudioProcessingObjectRT * This,
            /* [in] */ UINT32 u32OutputFrameCount);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectRT, CalcOutputFrames)
        UINT32 ( STDMETHODCALLTYPE *CalcOutputFrames )( 
            IAudioProcessingObjectRT * This,
            /* [annotation][in] */ 
            _In_  UINT32 u32InputFrameCount);
        
        END_INTERFACE
    } IAudioProcessingObjectRTVtbl;

    interface IAudioProcessingObjectRT
    {
        CONST_VTBL struct IAudioProcessingObjectRTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioProcessingObjectRT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioProcessingObjectRT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioProcessingObjectRT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioProcessingObjectRT_APOProcess(This,u32NumInputConnections,ppInputConnections,u32NumOutputConnections,ppOutputConnections)	\
    ( (This)->lpVtbl -> APOProcess(This,u32NumInputConnections,ppInputConnections,u32NumOutputConnections,ppOutputConnections) ) 

#define IAudioProcessingObjectRT_CalcInputFrames(This,u32OutputFrameCount)	\
    ( (This)->lpVtbl -> CalcInputFrames(This,u32OutputFrameCount) ) 

#define IAudioProcessingObjectRT_CalcOutputFrames(This,u32InputFrameCount)	\
    ( (This)->lpVtbl -> CalcOutputFrames(This,u32InputFrameCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioProcessingObjectRT_INTERFACE_DEFINED__ */


#ifndef __IAudioProcessingObjectVBR_INTERFACE_DEFINED__
#define __IAudioProcessingObjectVBR_INTERFACE_DEFINED__

/* interface IAudioProcessingObjectVBR */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioProcessingObjectVBR;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7ba1db8f-78ad-49cd-9591-f79d80a17c81")
    IAudioProcessingObjectVBR : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CalcMaxInputFrames( 
            /* [in] */ UINT32 u32MaxOutputFrameCount,
            /* [out] */ __RPC__out UINT32 *pu32InputFrameCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CalcMaxOutputFrames( 
            /* [in] */ UINT32 u32MaxInputFrameCount,
            /* [out] */ __RPC__out UINT32 *pu32OutputFrameCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioProcessingObjectVBRVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAudioProcessingObjectVBR * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAudioProcessingObjectVBR * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAudioProcessingObjectVBR * This);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectVBR, CalcMaxInputFrames)
        HRESULT ( STDMETHODCALLTYPE *CalcMaxInputFrames )( 
            __RPC__in IAudioProcessingObjectVBR * This,
            /* [in] */ UINT32 u32MaxOutputFrameCount,
            /* [out] */ __RPC__out UINT32 *pu32InputFrameCount);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectVBR, CalcMaxOutputFrames)
        HRESULT ( STDMETHODCALLTYPE *CalcMaxOutputFrames )( 
            __RPC__in IAudioProcessingObjectVBR * This,
            /* [in] */ UINT32 u32MaxInputFrameCount,
            /* [out] */ __RPC__out UINT32 *pu32OutputFrameCount);
        
        END_INTERFACE
    } IAudioProcessingObjectVBRVtbl;

    interface IAudioProcessingObjectVBR
    {
        CONST_VTBL struct IAudioProcessingObjectVBRVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioProcessingObjectVBR_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioProcessingObjectVBR_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioProcessingObjectVBR_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioProcessingObjectVBR_CalcMaxInputFrames(This,u32MaxOutputFrameCount,pu32InputFrameCount)	\
    ( (This)->lpVtbl -> CalcMaxInputFrames(This,u32MaxOutputFrameCount,pu32InputFrameCount) ) 

#define IAudioProcessingObjectVBR_CalcMaxOutputFrames(This,u32MaxInputFrameCount,pu32OutputFrameCount)	\
    ( (This)->lpVtbl -> CalcMaxOutputFrames(This,u32MaxInputFrameCount,pu32OutputFrameCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioProcessingObjectVBR_INTERFACE_DEFINED__ */


#ifndef __IAudioProcessingObjectConfiguration_INTERFACE_DEFINED__
#define __IAudioProcessingObjectConfiguration_INTERFACE_DEFINED__

/* interface IAudioProcessingObjectConfiguration */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioProcessingObjectConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0E5ED805-ABA6-49c3-8F9A-2B8C889C4FA8")
    IAudioProcessingObjectConfiguration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LockForProcess( 
            /* [annotation][in] */ 
            _In_  UINT32 u32NumInputConnections,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_DESCRIPTOR **ppInputConnections,
            /* [annotation][in] */ 
            _In_  UINT32 u32NumOutputConnections,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_DESCRIPTOR **ppOutputConnections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockForProcess( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioProcessingObjectConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioProcessingObjectConfiguration * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioProcessingObjectConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioProcessingObjectConfiguration * This);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectConfiguration, LockForProcess)
        HRESULT ( STDMETHODCALLTYPE *LockForProcess )( 
            IAudioProcessingObjectConfiguration * This,
            /* [annotation][in] */ 
            _In_  UINT32 u32NumInputConnections,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_DESCRIPTOR **ppInputConnections,
            /* [annotation][in] */ 
            _In_  UINT32 u32NumOutputConnections,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_DESCRIPTOR **ppOutputConnections);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObjectConfiguration, UnlockForProcess)
        HRESULT ( STDMETHODCALLTYPE *UnlockForProcess )( 
            IAudioProcessingObjectConfiguration * This);
        
        END_INTERFACE
    } IAudioProcessingObjectConfigurationVtbl;

    interface IAudioProcessingObjectConfiguration
    {
        CONST_VTBL struct IAudioProcessingObjectConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioProcessingObjectConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioProcessingObjectConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioProcessingObjectConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioProcessingObjectConfiguration_LockForProcess(This,u32NumInputConnections,ppInputConnections,u32NumOutputConnections,ppOutputConnections)	\
    ( (This)->lpVtbl -> LockForProcess(This,u32NumInputConnections,ppInputConnections,u32NumOutputConnections,ppOutputConnections) ) 

#define IAudioProcessingObjectConfiguration_UnlockForProcess(This)	\
    ( (This)->lpVtbl -> UnlockForProcess(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioProcessingObjectConfiguration_INTERFACE_DEFINED__ */


#ifndef __IAudioProcessingObject_INTERFACE_DEFINED__
#define __IAudioProcessingObject_INTERFACE_DEFINED__

/* interface IAudioProcessingObject */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioProcessingObject;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FD7F2B29-24D0-4b5c-B177-592C39F9CA10")
    IAudioProcessingObject : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLatency( 
            /* [out] */ __RPC__out HNSTIME *pTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegistrationProperties( 
            /* [out] */ __RPC__deref_out_opt APO_REG_PROPERTIES **ppRegProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Initialize( 
            /* [in] */ UINT32 cbDataSize,
            /* [size_is][in] */ __RPC__in_ecount_full(cbDataSize) BYTE *pbyData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsInputFormatSupported( 
            /* [unique][in] */ __RPC__in_opt IAudioMediaType *pOppositeFormat,
            /* [in] */ __RPC__in_opt IAudioMediaType *pRequestedInputFormat,
            /* [out] */ __RPC__deref_out_opt IAudioMediaType **ppSupportedInputFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsOutputFormatSupported( 
            /* [unique][in] */ __RPC__in_opt IAudioMediaType *pOppositeFormat,
            /* [in] */ __RPC__in_opt IAudioMediaType *pRequestedOutputFormat,
            /* [out] */ __RPC__deref_out_opt IAudioMediaType **ppSupportedOutputFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputChannelCount( 
            /* [out] */ __RPC__out UINT32 *pu32ChannelCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioProcessingObjectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAudioProcessingObject * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAudioProcessingObject * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAudioProcessingObject * This);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IAudioProcessingObject * This);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, GetLatency)
        HRESULT ( STDMETHODCALLTYPE *GetLatency )( 
            __RPC__in IAudioProcessingObject * This,
            /* [out] */ __RPC__out HNSTIME *pTime);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, GetRegistrationProperties)
        HRESULT ( STDMETHODCALLTYPE *GetRegistrationProperties )( 
            __RPC__in IAudioProcessingObject * This,
            /* [out] */ __RPC__deref_out_opt APO_REG_PROPERTIES **ppRegProps);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, Initialize)
        HRESULT ( STDMETHODCALLTYPE *Initialize )( 
            __RPC__in IAudioProcessingObject * This,
            /* [in] */ UINT32 cbDataSize,
            /* [size_is][in] */ __RPC__in_ecount_full(cbDataSize) BYTE *pbyData);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, IsInputFormatSupported)
        HRESULT ( STDMETHODCALLTYPE *IsInputFormatSupported )( 
            __RPC__in IAudioProcessingObject * This,
            /* [unique][in] */ __RPC__in_opt IAudioMediaType *pOppositeFormat,
            /* [in] */ __RPC__in_opt IAudioMediaType *pRequestedInputFormat,
            /* [out] */ __RPC__deref_out_opt IAudioMediaType **ppSupportedInputFormat);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, IsOutputFormatSupported)
        HRESULT ( STDMETHODCALLTYPE *IsOutputFormatSupported )( 
            __RPC__in IAudioProcessingObject * This,
            /* [unique][in] */ __RPC__in_opt IAudioMediaType *pOppositeFormat,
            /* [in] */ __RPC__in_opt IAudioMediaType *pRequestedOutputFormat,
            /* [out] */ __RPC__deref_out_opt IAudioMediaType **ppSupportedOutputFormat);
        
        DECLSPEC_XFGVIRT(IAudioProcessingObject, GetInputChannelCount)
        HRESULT ( STDMETHODCALLTYPE *GetInputChannelCount )( 
            __RPC__in IAudioProcessingObject * This,
            /* [out] */ __RPC__out UINT32 *pu32ChannelCount);
        
        END_INTERFACE
    } IAudioProcessingObjectVtbl;

    interface IAudioProcessingObject
    {
        CONST_VTBL struct IAudioProcessingObjectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioProcessingObject_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioProcessingObject_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioProcessingObject_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioProcessingObject_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IAudioProcessingObject_GetLatency(This,pTime)	\
    ( (This)->lpVtbl -> GetLatency(This,pTime) ) 

#define IAudioProcessingObject_GetRegistrationProperties(This,ppRegProps)	\
    ( (This)->lpVtbl -> GetRegistrationProperties(This,ppRegProps) ) 

#define IAudioProcessingObject_Initialize(This,cbDataSize,pbyData)	\
    ( (This)->lpVtbl -> Initialize(This,cbDataSize,pbyData) ) 

#define IAudioProcessingObject_IsInputFormatSupported(This,pOppositeFormat,pRequestedInputFormat,ppSupportedInputFormat)	\
    ( (This)->lpVtbl -> IsInputFormatSupported(This,pOppositeFormat,pRequestedInputFormat,ppSupportedInputFormat) ) 

#define IAudioProcessingObject_IsOutputFormatSupported(This,pOppositeFormat,pRequestedOutputFormat,ppSupportedOutputFormat)	\
    ( (This)->lpVtbl -> IsOutputFormatSupported(This,pOppositeFormat,pRequestedOutputFormat,ppSupportedOutputFormat) ) 

#define IAudioProcessingObject_GetInputChannelCount(This,pu32ChannelCount)	\
    ( (This)->lpVtbl -> GetInputChannelCount(This,pu32ChannelCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioProcessingObject_INTERFACE_DEFINED__ */


#ifndef __IAudioDeviceModulesClient_INTERFACE_DEFINED__
#define __IAudioDeviceModulesClient_INTERFACE_DEFINED__

/* interface IAudioDeviceModulesClient */
/* [unique][helpstring][nonextensible][uuid][local][object] */ 


EXTERN_C const IID IID_IAudioDeviceModulesClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("98F37DAC-D0B6-49F5-896A-AA4D169A4C48")
    IAudioDeviceModulesClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAudioDeviceModulesManager( 
            /* [annotation][in] */ 
            _In_  IUnknown *pAudioDeviceModulesManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioDeviceModulesClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioDeviceModulesClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioDeviceModulesClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioDeviceModulesClient * This);
        
        DECLSPEC_XFGVIRT(IAudioDeviceModulesClient, SetAudioDeviceModulesManager)
        HRESULT ( STDMETHODCALLTYPE *SetAudioDeviceModulesManager )( 
            IAudioDeviceModulesClient * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pAudioDeviceModulesManager);
        
        END_INTERFACE
    } IAudioDeviceModulesClientVtbl;

    interface IAudioDeviceModulesClient
    {
        CONST_VTBL struct IAudioDeviceModulesClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioDeviceModulesClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioDeviceModulesClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioDeviceModulesClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioDeviceModulesClient_SetAudioDeviceModulesManager(This,pAudioDeviceModulesManager)	\
    ( (This)->lpVtbl -> SetAudioDeviceModulesManager(This,pAudioDeviceModulesManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioDeviceModulesClient_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audioenginebaseapo_0000_0005 */
/* [local] */ 

//
// APO registration functions
//
typedef HRESULT (WINAPI FNAPONOTIFICATIONCALLBACK)(APO_REG_PROPERTIES* pProperties, VOID* pvRefData);
extern HRESULT WINAPI RegisterAPO(APO_REG_PROPERTIES const* pProperties);
extern HRESULT WINAPI UnregisterAPO(REFCLSID clsid);
extern HRESULT WINAPI RegisterAPONotification(HANDLE hEvent);
extern HRESULT WINAPI UnregisterAPONotification(HANDLE hEvent);
extern HRESULT WINAPI EnumerateAPOs(FNAPONOTIFICATIONCALLBACK pfnCallback, PVOID pvRefData);
extern HRESULT WINAPI GetAPOProperties(REFCLSID clsid, APO_REG_PROPERTIES* pProperties);


extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0005_v0_0_s_ifspec;

#ifndef __IAudioSystemEffects_INTERFACE_DEFINED__
#define __IAudioSystemEffects_INTERFACE_DEFINED__

/* interface IAudioSystemEffects */
/* [object][uuid] */ 


EXTERN_C const IID IID_IAudioSystemEffects;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5FA00F27-ADD6-499a-8A9D-6B98521FA75B")
    IAudioSystemEffects : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSystemEffectsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAudioSystemEffects * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAudioSystemEffects * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAudioSystemEffects * This);
        
        END_INTERFACE
    } IAudioSystemEffectsVtbl;

    interface IAudioSystemEffects
    {
        CONST_VTBL struct IAudioSystemEffectsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSystemEffects_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSystemEffects_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSystemEffects_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSystemEffects_INTERFACE_DEFINED__ */


#ifndef __IAudioSystemEffects2_INTERFACE_DEFINED__
#define __IAudioSystemEffects2_INTERFACE_DEFINED__

/* interface IAudioSystemEffects2 */
/* [object][uuid][local] */ 


EXTERN_C const IID IID_IAudioSystemEffects2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BAFE99D2-7436-44CE-9E0E-4D89AFBFFF56")
    IAudioSystemEffects2 : public IAudioSystemEffects
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEffectsList( 
            /* [annotation][out] */ 
            _Outptr_result_buffer_maybenull_(*pcEffects)  LPGUID *ppEffectsIds,
            /* [annotation][out] */ 
            _Out_  UINT *pcEffects,
            /* [annotation][in] */ 
            _In_  HANDLE Event) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSystemEffects2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSystemEffects2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSystemEffects2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSystemEffects2 * This);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffects2, GetEffectsList)
        HRESULT ( STDMETHODCALLTYPE *GetEffectsList )( 
            IAudioSystemEffects2 * This,
            /* [annotation][out] */ 
            _Outptr_result_buffer_maybenull_(*pcEffects)  LPGUID *ppEffectsIds,
            /* [annotation][out] */ 
            _Out_  UINT *pcEffects,
            /* [annotation][in] */ 
            _In_  HANDLE Event);
        
        END_INTERFACE
    } IAudioSystemEffects2Vtbl;

    interface IAudioSystemEffects2
    {
        CONST_VTBL struct IAudioSystemEffects2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSystemEffects2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSystemEffects2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSystemEffects2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 



#define IAudioSystemEffects2_GetEffectsList(This,ppEffectsIds,pcEffects,Event)	\
    ( (This)->lpVtbl -> GetEffectsList(This,ppEffectsIds,pcEffects,Event) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSystemEffects2_INTERFACE_DEFINED__ */


#ifndef __IAudioSystemEffectsCustomFormats_INTERFACE_DEFINED__
#define __IAudioSystemEffectsCustomFormats_INTERFACE_DEFINED__

/* interface IAudioSystemEffectsCustomFormats */
/* [object][uuid] */ 


EXTERN_C const IID IID_IAudioSystemEffectsCustomFormats;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B1176E34-BB7F-4f05-BEBD-1B18A534E097")
    IAudioSystemEffectsCustomFormats : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFormatCount( 
            /* [out] */ __RPC__out UINT *pcFormats) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [in] */ UINT nFormat,
            /* [out] */ __RPC__deref_out_opt IAudioMediaType **ppFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormatRepresentation( 
            /* [in] */ UINT nFormat,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwstrFormatRep) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSystemEffectsCustomFormatsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IAudioSystemEffectsCustomFormats * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IAudioSystemEffectsCustomFormats * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IAudioSystemEffectsCustomFormats * This);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsCustomFormats, GetFormatCount)
        HRESULT ( STDMETHODCALLTYPE *GetFormatCount )( 
            __RPC__in IAudioSystemEffectsCustomFormats * This,
            /* [out] */ __RPC__out UINT *pcFormats);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsCustomFormats, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            __RPC__in IAudioSystemEffectsCustomFormats * This,
            /* [in] */ UINT nFormat,
            /* [out] */ __RPC__deref_out_opt IAudioMediaType **ppFormat);
        
        DECLSPEC_XFGVIRT(IAudioSystemEffectsCustomFormats, GetFormatRepresentation)
        HRESULT ( STDMETHODCALLTYPE *GetFormatRepresentation )( 
            __RPC__in IAudioSystemEffectsCustomFormats * This,
            /* [in] */ UINT nFormat,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwstrFormatRep);
        
        END_INTERFACE
    } IAudioSystemEffectsCustomFormatsVtbl;

    interface IAudioSystemEffectsCustomFormats
    {
        CONST_VTBL struct IAudioSystemEffectsCustomFormatsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSystemEffectsCustomFormats_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSystemEffectsCustomFormats_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSystemEffectsCustomFormats_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSystemEffectsCustomFormats_GetFormatCount(This,pcFormats)	\
    ( (This)->lpVtbl -> GetFormatCount(This,pcFormats) ) 

#define IAudioSystemEffectsCustomFormats_GetFormat(This,nFormat,ppFormat)	\
    ( (This)->lpVtbl -> GetFormat(This,nFormat,ppFormat) ) 

#define IAudioSystemEffectsCustomFormats_GetFormatRepresentation(This,nFormat,ppwstrFormatRep)	\
    ( (This)->lpVtbl -> GetFormatRepresentation(This,nFormat,ppwstrFormatRep) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSystemEffectsCustomFormats_INTERFACE_DEFINED__ */


#ifndef __IApoAuxiliaryInputConfiguration_INTERFACE_DEFINED__
#define __IApoAuxiliaryInputConfiguration_INTERFACE_DEFINED__

/* interface IApoAuxiliaryInputConfiguration */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IApoAuxiliaryInputConfiguration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4CEB0AAB-FA19-48ED-A857-87771AE1B768")
    IApoAuxiliaryInputConfiguration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddAuxiliaryInput( 
            /* [in] */ DWORD dwInputId,
            /* [in] */ UINT32 cbDataSize,
            /* [size_is][in] */ BYTE *pbyData,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_DESCRIPTOR *pInputConnection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAuxiliaryInput( 
            /* [in] */ DWORD dwInputId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsInputFormatSupported( 
            /* [in] */ IAudioMediaType *pRequestedInputFormat,
            /* [out] */ IAudioMediaType **ppSupportedInputFormat) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApoAuxiliaryInputConfigurationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IApoAuxiliaryInputConfiguration * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IApoAuxiliaryInputConfiguration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IApoAuxiliaryInputConfiguration * This);
        
        DECLSPEC_XFGVIRT(IApoAuxiliaryInputConfiguration, AddAuxiliaryInput)
        HRESULT ( STDMETHODCALLTYPE *AddAuxiliaryInput )( 
            IApoAuxiliaryInputConfiguration * This,
            /* [in] */ DWORD dwInputId,
            /* [in] */ UINT32 cbDataSize,
            /* [size_is][in] */ BYTE *pbyData,
            /* [annotation][in] */ 
            _In_  APO_CONNECTION_DESCRIPTOR *pInputConnection);
        
        DECLSPEC_XFGVIRT(IApoAuxiliaryInputConfiguration, RemoveAuxiliaryInput)
        HRESULT ( STDMETHODCALLTYPE *RemoveAuxiliaryInput )( 
            IApoAuxiliaryInputConfiguration * This,
            /* [in] */ DWORD dwInputId);
        
        DECLSPEC_XFGVIRT(IApoAuxiliaryInputConfiguration, IsInputFormatSupported)
        HRESULT ( STDMETHODCALLTYPE *IsInputFormatSupported )( 
            IApoAuxiliaryInputConfiguration * This,
            /* [in] */ IAudioMediaType *pRequestedInputFormat,
            /* [out] */ IAudioMediaType **ppSupportedInputFormat);
        
        END_INTERFACE
    } IApoAuxiliaryInputConfigurationVtbl;

    interface IApoAuxiliaryInputConfiguration
    {
        CONST_VTBL struct IApoAuxiliaryInputConfigurationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApoAuxiliaryInputConfiguration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApoAuxiliaryInputConfiguration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApoAuxiliaryInputConfiguration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApoAuxiliaryInputConfiguration_AddAuxiliaryInput(This,dwInputId,cbDataSize,pbyData,pInputConnection)	\
    ( (This)->lpVtbl -> AddAuxiliaryInput(This,dwInputId,cbDataSize,pbyData,pInputConnection) ) 

#define IApoAuxiliaryInputConfiguration_RemoveAuxiliaryInput(This,dwInputId)	\
    ( (This)->lpVtbl -> RemoveAuxiliaryInput(This,dwInputId) ) 

#define IApoAuxiliaryInputConfiguration_IsInputFormatSupported(This,pRequestedInputFormat,ppSupportedInputFormat)	\
    ( (This)->lpVtbl -> IsInputFormatSupported(This,pRequestedInputFormat,ppSupportedInputFormat) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApoAuxiliaryInputConfiguration_INTERFACE_DEFINED__ */


#ifndef __IApoAuxiliaryInputRT_INTERFACE_DEFINED__
#define __IApoAuxiliaryInputRT_INTERFACE_DEFINED__

/* interface IApoAuxiliaryInputRT */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IApoAuxiliaryInputRT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F851809C-C177-49A0-B1B2-B66F017943AB")
    IApoAuxiliaryInputRT : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE AcceptInput( 
            /* [annotation][in] */ 
            _In_  DWORD dwInputId,
            /* [annotation][in] */ 
            _In_  const APO_CONNECTION_PROPERTY *pInputConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApoAuxiliaryInputRTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IApoAuxiliaryInputRT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IApoAuxiliaryInputRT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IApoAuxiliaryInputRT * This);
        
        DECLSPEC_XFGVIRT(IApoAuxiliaryInputRT, AcceptInput)
        void ( STDMETHODCALLTYPE *AcceptInput )( 
            IApoAuxiliaryInputRT * This,
            /* [annotation][in] */ 
            _In_  DWORD dwInputId,
            /* [annotation][in] */ 
            _In_  const APO_CONNECTION_PROPERTY *pInputConnection);
        
        END_INTERFACE
    } IApoAuxiliaryInputRTVtbl;

    interface IApoAuxiliaryInputRT
    {
        CONST_VTBL struct IApoAuxiliaryInputRTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApoAuxiliaryInputRT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApoAuxiliaryInputRT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApoAuxiliaryInputRT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IApoAuxiliaryInputRT_AcceptInput(This,dwInputId,pInputConnection)	\
    ( (This)->lpVtbl -> AcceptInput(This,dwInputId,pInputConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApoAuxiliaryInputRT_INTERFACE_DEFINED__ */


#ifndef __IApoAcousticEchoCancellation_INTERFACE_DEFINED__
#define __IApoAcousticEchoCancellation_INTERFACE_DEFINED__

/* interface IApoAcousticEchoCancellation */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IApoAcousticEchoCancellation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("25385759-3236-4101-A943-25693DFB5D2D")
    IApoAcousticEchoCancellation : public IUnknown
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IApoAcousticEchoCancellationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApoAcousticEchoCancellation * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApoAcousticEchoCancellation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApoAcousticEchoCancellation * This);
        
        END_INTERFACE
    } IApoAcousticEchoCancellationVtbl;

    interface IApoAcousticEchoCancellation
    {
        CONST_VTBL struct IApoAcousticEchoCancellationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApoAcousticEchoCancellation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApoAcousticEchoCancellation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApoAcousticEchoCancellation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApoAcousticEchoCancellation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audioenginebaseapo_0000_0011 */
/* [local] */ 

typedef /* [v1_enum] */ 
enum APO_REFERENCE_STREAM_PROPERTIES
    {
        APO_REFERENCE_STREAM_PROPERTIES_NONE	= 0,
        APO_REFERENCE_STREAM_PROPERTIES_POST_VOLUME_LOOPBACK	= 0x1
    } 	APO_REFERENCE_STREAM_PROPERTIES;

DEFINE_ENUM_FLAG_OPERATORS(APO_REFERENCE_STREAM_PROPERTIES);


extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0011_v0_0_s_ifspec;

#ifndef __IApoAcousticEchoCancellation2_INTERFACE_DEFINED__
#define __IApoAcousticEchoCancellation2_INTERFACE_DEFINED__

/* interface IApoAcousticEchoCancellation2 */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IApoAcousticEchoCancellation2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F235855F-F06D-45B3-A63F-EE4B71509DC2")
    IApoAcousticEchoCancellation2 : public IApoAcousticEchoCancellation
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDesiredReferenceStreamProperties( 
            /* [out] */ __RPC__out APO_REFERENCE_STREAM_PROPERTIES *pProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IApoAcousticEchoCancellation2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IApoAcousticEchoCancellation2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IApoAcousticEchoCancellation2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IApoAcousticEchoCancellation2 * This);
        
        DECLSPEC_XFGVIRT(IApoAcousticEchoCancellation2, GetDesiredReferenceStreamProperties)
        HRESULT ( STDMETHODCALLTYPE *GetDesiredReferenceStreamProperties )( 
            __RPC__in IApoAcousticEchoCancellation2 * This,
            /* [out] */ __RPC__out APO_REFERENCE_STREAM_PROPERTIES *pProperties);
        
        END_INTERFACE
    } IApoAcousticEchoCancellation2Vtbl;

    interface IApoAcousticEchoCancellation2
    {
        CONST_VTBL struct IApoAcousticEchoCancellation2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IApoAcousticEchoCancellation2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IApoAcousticEchoCancellation2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IApoAcousticEchoCancellation2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 



#define IApoAcousticEchoCancellation2_GetDesiredReferenceStreamProperties(This,pProperties)	\
    ( (This)->lpVtbl -> GetDesiredReferenceStreamProperties(This,pProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IApoAcousticEchoCancellation2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_audioenginebaseapo_0000_0012 */
/* [local] */ 

typedef struct APOInitSystemEffects
    {
    APOInitBaseStruct APOInit;
    IPropertyStore *pAPOEndpointProperties;
    IPropertyStore *pAPOSystemEffectsProperties;
    void *pReserved;
    IMMDeviceCollection *pDeviceCollection;
    } 	APOInitSystemEffects;

typedef struct APOInitSystemEffects2
    {
    APOInitBaseStruct APOInit;
    IPropertyStore *pAPOEndpointProperties;
    IPropertyStore *pAPOSystemEffectsProperties;
    void *pReserved;
    IMMDeviceCollection *pDeviceCollection;
    UINT nSoftwareIoDeviceInCollection;
    UINT nSoftwareIoConnectorIndex;
    GUID AudioProcessingMode;
    BOOL InitializeForDiscoveryOnly;
    } 	APOInitSystemEffects2;

typedef /* [public] */ struct __MIDL___MIDL_itf_audioenginebaseapo_0000_0012_0001
    {
    LPARAM AddPageParam;
    LPWSTR pwstrEndpointID;
    IPropertyStore *pFxProperties;
    } 	AudioFXExtensionParams;

DEFINE_PROPERTYKEY(PKEY_FX_Association, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 0);
DEFINE_PROPERTYKEY(PKEY_FX_PreMixEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 1);
DEFINE_PROPERTYKEY(PKEY_FX_PostMixEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 2);
DEFINE_PROPERTYKEY(PKEY_FX_UserInterfaceClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 3);
DEFINE_PROPERTYKEY(PKEY_FX_FriendlyName, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 4);
DEFINE_PROPERTYKEY(PKEY_FX_StreamEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 5);
DEFINE_PROPERTYKEY(PKEY_FX_ModeEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 6);
DEFINE_PROPERTYKEY(PKEY_FX_EndpointEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 7);
DEFINE_PROPERTYKEY(PKEY_FX_KeywordDetector_StreamEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 8);
DEFINE_PROPERTYKEY(PKEY_FX_KeywordDetector_ModeEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 9);
DEFINE_PROPERTYKEY(PKEY_FX_KeywordDetector_EndpointEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 10);
DEFINE_PROPERTYKEY(PKEY_FX_Offload_StreamEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 11);
DEFINE_PROPERTYKEY(PKEY_FX_Offload_ModeEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 12);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_StreamEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 13);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_ModeEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 14);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_EndpointEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 15);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_KeywordDetector_StreamEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 16);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_KeywordDetector_ModeEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 17);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_KeywordDetector_EndpointEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 18);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_Offload_StreamEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 19);
DEFINE_PROPERTYKEY(PKEY_CompositeFX_Offload_ModeEffectClsid, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 20);
DEFINE_PROPERTYKEY(PKEY_FX_SupportAppLauncher, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 21);
DEFINE_PROPERTYKEY(PKEY_FX_SupportedFormats, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 22);
DEFINE_PROPERTYKEY(PKEY_FX_Enumerator, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 23);
DEFINE_PROPERTYKEY(PKEY_FX_VersionMajor, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 24);
DEFINE_PROPERTYKEY(PKEY_FX_VersionMinor, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 25);
DEFINE_PROPERTYKEY(PKEY_FX_Author, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 26);
DEFINE_PROPERTYKEY(PKEY_FX_ObjectId, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 27);
DEFINE_PROPERTYKEY(PKEY_FX_State, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 28);
DEFINE_PROPERTYKEY(PKEY_FX_EffectPackSchema_Version, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 29);
DEFINE_PROPERTYKEY(PKEY_FX_ApplyToBluetooth, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 30);
DEFINE_PROPERTYKEY(PKEY_FX_ApplyToUsb, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 31);
DEFINE_PROPERTYKEY(PKEY_FX_ApplyToRender, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 32);
DEFINE_PROPERTYKEY(PKEY_FX_ApplyToCapture, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 33);
DEFINE_PROPERTYKEY(PKEY_FX_RequestSetAsDefault, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 34);
DEFINE_PROPERTYKEY(PKEY_FX_RequestSetAsDefaultPriority, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 35);
DEFINE_PROPERTYKEY(PKEY_FX_OEM_Preferred_EffectPack_Id, 0xD04E05A6, 0x594B, 0x4fb6, 0xA8, 0x0D, 0x01, 0xAF, 0x5E, 0xED, 0x7D, 0x1D, 36);
DEFINE_PROPERTYKEY(PKEY_SFX_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 5);
DEFINE_PROPERTYKEY(PKEY_MFX_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 6);
DEFINE_PROPERTYKEY(PKEY_EFX_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 7);
DEFINE_PROPERTYKEY(PKEY_SFX_KeywordDetector_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 8);
DEFINE_PROPERTYKEY(PKEY_MFX_KeywordDetector_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 9);
DEFINE_PROPERTYKEY(PKEY_EFX_KeywordDetector_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 10);
DEFINE_PROPERTYKEY(PKEY_SFX_Offload_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 11);
DEFINE_PROPERTYKEY(PKEY_MFX_Offload_ProcessingModes_Supported_For_Streaming, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 12);
DEFINE_PROPERTYKEY(PKEY_APO_SWFallback_ProcessingModes, 0xd3993a3f, 0x99c2, 0x4402, 0xb5, 0xec, 0xa9, 0x2a, 0x3, 0x67, 0x66, 0x4b, 13);
DEFINE_GUID(PKEY_FX_EffectPack_Schema_V1, 0x7abf23d9, 0x727e, 0x4d0b, 0x86, 0xa3, 0xdd, 0x50, 0x1d, 0x26, 0x0, 0x1);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_audioenginebaseapo_0000_0012_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


