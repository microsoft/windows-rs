

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

#ifndef __mfmediaengine_h__
#define __mfmediaengine_h__

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

#ifndef __IMFMediaError_FWD_DEFINED__
#define __IMFMediaError_FWD_DEFINED__
typedef interface IMFMediaError IMFMediaError;

#endif 	/* __IMFMediaError_FWD_DEFINED__ */


#ifndef __IMFMediaTimeRange_FWD_DEFINED__
#define __IMFMediaTimeRange_FWD_DEFINED__
typedef interface IMFMediaTimeRange IMFMediaTimeRange;

#endif 	/* __IMFMediaTimeRange_FWD_DEFINED__ */


#ifndef __IMFMediaEngineNotify_FWD_DEFINED__
#define __IMFMediaEngineNotify_FWD_DEFINED__
typedef interface IMFMediaEngineNotify IMFMediaEngineNotify;

#endif 	/* __IMFMediaEngineNotify_FWD_DEFINED__ */


#ifndef __IMFMediaEngineSrcElements_FWD_DEFINED__
#define __IMFMediaEngineSrcElements_FWD_DEFINED__
typedef interface IMFMediaEngineSrcElements IMFMediaEngineSrcElements;

#endif 	/* __IMFMediaEngineSrcElements_FWD_DEFINED__ */


#ifndef __IMFMediaEngine_FWD_DEFINED__
#define __IMFMediaEngine_FWD_DEFINED__
typedef interface IMFMediaEngine IMFMediaEngine;

#endif 	/* __IMFMediaEngine_FWD_DEFINED__ */


#ifndef __IMFMediaEngineEx_FWD_DEFINED__
#define __IMFMediaEngineEx_FWD_DEFINED__
typedef interface IMFMediaEngineEx IMFMediaEngineEx;

#endif 	/* __IMFMediaEngineEx_FWD_DEFINED__ */


#ifndef __IMFMediaEngineAudioEndpointId_FWD_DEFINED__
#define __IMFMediaEngineAudioEndpointId_FWD_DEFINED__
typedef interface IMFMediaEngineAudioEndpointId IMFMediaEngineAudioEndpointId;

#endif 	/* __IMFMediaEngineAudioEndpointId_FWD_DEFINED__ */


#ifndef __IMFMediaEngineExtension_FWD_DEFINED__
#define __IMFMediaEngineExtension_FWD_DEFINED__
typedef interface IMFMediaEngineExtension IMFMediaEngineExtension;

#endif 	/* __IMFMediaEngineExtension_FWD_DEFINED__ */


#ifndef __IMFMediaEngineProtectedContent_FWD_DEFINED__
#define __IMFMediaEngineProtectedContent_FWD_DEFINED__
typedef interface IMFMediaEngineProtectedContent IMFMediaEngineProtectedContent;

#endif 	/* __IMFMediaEngineProtectedContent_FWD_DEFINED__ */


#ifndef __IAudioSourceProvider_FWD_DEFINED__
#define __IAudioSourceProvider_FWD_DEFINED__
typedef interface IAudioSourceProvider IAudioSourceProvider;

#endif 	/* __IAudioSourceProvider_FWD_DEFINED__ */


#ifndef __IMFMediaEngineWebSupport_FWD_DEFINED__
#define __IMFMediaEngineWebSupport_FWD_DEFINED__
typedef interface IMFMediaEngineWebSupport IMFMediaEngineWebSupport;

#endif 	/* __IMFMediaEngineWebSupport_FWD_DEFINED__ */


#ifndef __IMFMediaSourceExtensionNotify_FWD_DEFINED__
#define __IMFMediaSourceExtensionNotify_FWD_DEFINED__
typedef interface IMFMediaSourceExtensionNotify IMFMediaSourceExtensionNotify;

#endif 	/* __IMFMediaSourceExtensionNotify_FWD_DEFINED__ */


#ifndef __IMFBufferListNotify_FWD_DEFINED__
#define __IMFBufferListNotify_FWD_DEFINED__
typedef interface IMFBufferListNotify IMFBufferListNotify;

#endif 	/* __IMFBufferListNotify_FWD_DEFINED__ */


#ifndef __IMFSourceBufferNotify_FWD_DEFINED__
#define __IMFSourceBufferNotify_FWD_DEFINED__
typedef interface IMFSourceBufferNotify IMFSourceBufferNotify;

#endif 	/* __IMFSourceBufferNotify_FWD_DEFINED__ */


#ifndef __IMFSourceBuffer_FWD_DEFINED__
#define __IMFSourceBuffer_FWD_DEFINED__
typedef interface IMFSourceBuffer IMFSourceBuffer;

#endif 	/* __IMFSourceBuffer_FWD_DEFINED__ */


#ifndef __IMFSourceBufferAppendMode_FWD_DEFINED__
#define __IMFSourceBufferAppendMode_FWD_DEFINED__
typedef interface IMFSourceBufferAppendMode IMFSourceBufferAppendMode;

#endif 	/* __IMFSourceBufferAppendMode_FWD_DEFINED__ */


#ifndef __IMFSourceBufferList_FWD_DEFINED__
#define __IMFSourceBufferList_FWD_DEFINED__
typedef interface IMFSourceBufferList IMFSourceBufferList;

#endif 	/* __IMFSourceBufferList_FWD_DEFINED__ */


#ifndef __IMFMediaSourceExtension_FWD_DEFINED__
#define __IMFMediaSourceExtension_FWD_DEFINED__
typedef interface IMFMediaSourceExtension IMFMediaSourceExtension;

#endif 	/* __IMFMediaSourceExtension_FWD_DEFINED__ */


#ifndef __IMFMediaSourceExtensionLiveSeekableRange_FWD_DEFINED__
#define __IMFMediaSourceExtensionLiveSeekableRange_FWD_DEFINED__
typedef interface IMFMediaSourceExtensionLiveSeekableRange IMFMediaSourceExtensionLiveSeekableRange;

#endif 	/* __IMFMediaSourceExtensionLiveSeekableRange_FWD_DEFINED__ */


#ifndef __IMFMediaEngineEME_FWD_DEFINED__
#define __IMFMediaEngineEME_FWD_DEFINED__
typedef interface IMFMediaEngineEME IMFMediaEngineEME;

#endif 	/* __IMFMediaEngineEME_FWD_DEFINED__ */


#ifndef __IMFMediaEngineSrcElementsEx_FWD_DEFINED__
#define __IMFMediaEngineSrcElementsEx_FWD_DEFINED__
typedef interface IMFMediaEngineSrcElementsEx IMFMediaEngineSrcElementsEx;

#endif 	/* __IMFMediaEngineSrcElementsEx_FWD_DEFINED__ */


#ifndef __IMFMediaEngineNeedKeyNotify_FWD_DEFINED__
#define __IMFMediaEngineNeedKeyNotify_FWD_DEFINED__
typedef interface IMFMediaEngineNeedKeyNotify IMFMediaEngineNeedKeyNotify;

#endif 	/* __IMFMediaEngineNeedKeyNotify_FWD_DEFINED__ */


#ifndef __IMFMediaKeys_FWD_DEFINED__
#define __IMFMediaKeys_FWD_DEFINED__
typedef interface IMFMediaKeys IMFMediaKeys;

#endif 	/* __IMFMediaKeys_FWD_DEFINED__ */


#ifndef __IMFMediaKeySession_FWD_DEFINED__
#define __IMFMediaKeySession_FWD_DEFINED__
typedef interface IMFMediaKeySession IMFMediaKeySession;

#endif 	/* __IMFMediaKeySession_FWD_DEFINED__ */


#ifndef __IMFMediaKeySessionNotify_FWD_DEFINED__
#define __IMFMediaKeySessionNotify_FWD_DEFINED__
typedef interface IMFMediaKeySessionNotify IMFMediaKeySessionNotify;

#endif 	/* __IMFMediaKeySessionNotify_FWD_DEFINED__ */


#ifndef __IMFCdmSuspendNotify_FWD_DEFINED__
#define __IMFCdmSuspendNotify_FWD_DEFINED__
typedef interface IMFCdmSuspendNotify IMFCdmSuspendNotify;

#endif 	/* __IMFCdmSuspendNotify_FWD_DEFINED__ */


#ifndef __IMFHDCPStatus_FWD_DEFINED__
#define __IMFHDCPStatus_FWD_DEFINED__
typedef interface IMFHDCPStatus IMFHDCPStatus;

#endif 	/* __IMFHDCPStatus_FWD_DEFINED__ */


#ifndef __IMFMediaEngineOPMInfo_FWD_DEFINED__
#define __IMFMediaEngineOPMInfo_FWD_DEFINED__
typedef interface IMFMediaEngineOPMInfo IMFMediaEngineOPMInfo;

#endif 	/* __IMFMediaEngineOPMInfo_FWD_DEFINED__ */


#ifndef __IMFMediaEngineClassFactory_FWD_DEFINED__
#define __IMFMediaEngineClassFactory_FWD_DEFINED__
typedef interface IMFMediaEngineClassFactory IMFMediaEngineClassFactory;

#endif 	/* __IMFMediaEngineClassFactory_FWD_DEFINED__ */


#ifndef __IMFMediaEngineClassFactoryEx_FWD_DEFINED__
#define __IMFMediaEngineClassFactoryEx_FWD_DEFINED__
typedef interface IMFMediaEngineClassFactoryEx IMFMediaEngineClassFactoryEx;

#endif 	/* __IMFMediaEngineClassFactoryEx_FWD_DEFINED__ */


#ifndef __IMFMediaEngineClassFactory2_FWD_DEFINED__
#define __IMFMediaEngineClassFactory2_FWD_DEFINED__
typedef interface IMFMediaEngineClassFactory2 IMFMediaEngineClassFactory2;

#endif 	/* __IMFMediaEngineClassFactory2_FWD_DEFINED__ */


#ifndef __IMFExtendedDRMTypeSupport_FWD_DEFINED__
#define __IMFExtendedDRMTypeSupport_FWD_DEFINED__
typedef interface IMFExtendedDRMTypeSupport IMFExtendedDRMTypeSupport;

#endif 	/* __IMFExtendedDRMTypeSupport_FWD_DEFINED__ */


#ifndef __IMFMediaEngineSupportsSourceTransfer_FWD_DEFINED__
#define __IMFMediaEngineSupportsSourceTransfer_FWD_DEFINED__
typedef interface IMFMediaEngineSupportsSourceTransfer IMFMediaEngineSupportsSourceTransfer;

#endif 	/* __IMFMediaEngineSupportsSourceTransfer_FWD_DEFINED__ */


#ifndef __IMFMediaEngineTransferSource_FWD_DEFINED__
#define __IMFMediaEngineTransferSource_FWD_DEFINED__
typedef interface IMFMediaEngineTransferSource IMFMediaEngineTransferSource;

#endif 	/* __IMFMediaEngineTransferSource_FWD_DEFINED__ */


#ifndef __IMFTimedText_FWD_DEFINED__
#define __IMFTimedText_FWD_DEFINED__
typedef interface IMFTimedText IMFTimedText;

#endif 	/* __IMFTimedText_FWD_DEFINED__ */


#ifndef __IMFTimedTextNotify_FWD_DEFINED__
#define __IMFTimedTextNotify_FWD_DEFINED__
typedef interface IMFTimedTextNotify IMFTimedTextNotify;

#endif 	/* __IMFTimedTextNotify_FWD_DEFINED__ */


#ifndef __IMFTimedTextTrack_FWD_DEFINED__
#define __IMFTimedTextTrack_FWD_DEFINED__
typedef interface IMFTimedTextTrack IMFTimedTextTrack;

#endif 	/* __IMFTimedTextTrack_FWD_DEFINED__ */


#ifndef __IMFTimedTextTrackList_FWD_DEFINED__
#define __IMFTimedTextTrackList_FWD_DEFINED__
typedef interface IMFTimedTextTrackList IMFTimedTextTrackList;

#endif 	/* __IMFTimedTextTrackList_FWD_DEFINED__ */


#ifndef __IMFTimedTextCue_FWD_DEFINED__
#define __IMFTimedTextCue_FWD_DEFINED__
typedef interface IMFTimedTextCue IMFTimedTextCue;

#endif 	/* __IMFTimedTextCue_FWD_DEFINED__ */


#ifndef __IMFTimedTextFormattedText_FWD_DEFINED__
#define __IMFTimedTextFormattedText_FWD_DEFINED__
typedef interface IMFTimedTextFormattedText IMFTimedTextFormattedText;

#endif 	/* __IMFTimedTextFormattedText_FWD_DEFINED__ */


#ifndef __IMFTimedTextStyle_FWD_DEFINED__
#define __IMFTimedTextStyle_FWD_DEFINED__
typedef interface IMFTimedTextStyle IMFTimedTextStyle;

#endif 	/* __IMFTimedTextStyle_FWD_DEFINED__ */


#ifndef __IMFTimedTextRegion_FWD_DEFINED__
#define __IMFTimedTextRegion_FWD_DEFINED__
typedef interface IMFTimedTextRegion IMFTimedTextRegion;

#endif 	/* __IMFTimedTextRegion_FWD_DEFINED__ */


#ifndef __IMFTimedTextBinary_FWD_DEFINED__
#define __IMFTimedTextBinary_FWD_DEFINED__
typedef interface IMFTimedTextBinary IMFTimedTextBinary;

#endif 	/* __IMFTimedTextBinary_FWD_DEFINED__ */


#ifndef __IMFTimedTextCueList_FWD_DEFINED__
#define __IMFTimedTextCueList_FWD_DEFINED__
typedef interface IMFTimedTextCueList IMFTimedTextCueList;

#endif 	/* __IMFTimedTextCueList_FWD_DEFINED__ */


#ifndef __IMFTimedTextRuby_FWD_DEFINED__
#define __IMFTimedTextRuby_FWD_DEFINED__
typedef interface IMFTimedTextRuby IMFTimedTextRuby;

#endif 	/* __IMFTimedTextRuby_FWD_DEFINED__ */


#ifndef __IMFTimedTextBouten_FWD_DEFINED__
#define __IMFTimedTextBouten_FWD_DEFINED__
typedef interface IMFTimedTextBouten IMFTimedTextBouten;

#endif 	/* __IMFTimedTextBouten_FWD_DEFINED__ */


#ifndef __IMFTimedTextStyle2_FWD_DEFINED__
#define __IMFTimedTextStyle2_FWD_DEFINED__
typedef interface IMFTimedTextStyle2 IMFTimedTextStyle2;

#endif 	/* __IMFTimedTextStyle2_FWD_DEFINED__ */


#ifndef __IMFMediaEngineEMENotify_FWD_DEFINED__
#define __IMFMediaEngineEMENotify_FWD_DEFINED__
typedef interface IMFMediaEngineEMENotify IMFMediaEngineEMENotify;

#endif 	/* __IMFMediaEngineEMENotify_FWD_DEFINED__ */


#ifndef __IMFMediaKeySessionNotify2_FWD_DEFINED__
#define __IMFMediaKeySessionNotify2_FWD_DEFINED__
typedef interface IMFMediaKeySessionNotify2 IMFMediaKeySessionNotify2;

#endif 	/* __IMFMediaKeySessionNotify2_FWD_DEFINED__ */


#ifndef __IMFMediaKeySystemAccess_FWD_DEFINED__
#define __IMFMediaKeySystemAccess_FWD_DEFINED__
typedef interface IMFMediaKeySystemAccess IMFMediaKeySystemAccess;

#endif 	/* __IMFMediaKeySystemAccess_FWD_DEFINED__ */


#ifndef __IMFMediaEngineClassFactory3_FWD_DEFINED__
#define __IMFMediaEngineClassFactory3_FWD_DEFINED__
typedef interface IMFMediaEngineClassFactory3 IMFMediaEngineClassFactory3;

#endif 	/* __IMFMediaEngineClassFactory3_FWD_DEFINED__ */


#ifndef __IMFMediaKeys2_FWD_DEFINED__
#define __IMFMediaKeys2_FWD_DEFINED__
typedef interface IMFMediaKeys2 IMFMediaKeys2;

#endif 	/* __IMFMediaKeys2_FWD_DEFINED__ */


#ifndef __IMFMediaKeySession2_FWD_DEFINED__
#define __IMFMediaKeySession2_FWD_DEFINED__
typedef interface IMFMediaKeySession2 IMFMediaKeySession2;

#endif 	/* __IMFMediaKeySession2_FWD_DEFINED__ */


#ifndef __IMFMediaEngineClassFactory4_FWD_DEFINED__
#define __IMFMediaEngineClassFactory4_FWD_DEFINED__
typedef interface IMFMediaEngineClassFactory4 IMFMediaEngineClassFactory4;

#endif 	/* __IMFMediaEngineClassFactory4_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "mfidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfmediaengine_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#if (WINVER >= _WIN32_WINNT_WIN8) 

// Prevent a name collision
#undef GetCurrentTime

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#define MF_INVALID_PRESENTATION_TIME 0x8000000000000000
typedef 
enum MF_MEDIA_ENGINE_ERR
    {
        MF_MEDIA_ENGINE_ERR_NOERROR	= 0,
        MF_MEDIA_ENGINE_ERR_ABORTED	= 1,
        MF_MEDIA_ENGINE_ERR_NETWORK	= 2,
        MF_MEDIA_ENGINE_ERR_DECODE	= 3,
        MF_MEDIA_ENGINE_ERR_SRC_NOT_SUPPORTED	= 4,
        MF_MEDIA_ENGINE_ERR_ENCRYPTED	= 5
    } 	MF_MEDIA_ENGINE_ERR;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0000_v0_0_s_ifspec;

#ifndef __IMFMediaError_INTERFACE_DEFINED__
#define __IMFMediaError_INTERFACE_DEFINED__

/* interface IMFMediaError */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaError;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fc0e10d2-ab2a-4501-a951-06bb1075184c")
    IMFMediaError : public IUnknown
    {
    public:
        virtual USHORT STDMETHODCALLTYPE GetErrorCode( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExtendedErrorCode( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetErrorCode( 
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_ERR error) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetExtendedErrorCode( 
            /* [annotation][in] */ 
            _In_  HRESULT error) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaErrorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaError * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaError * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaError * This);
        
        DECLSPEC_XFGVIRT(IMFMediaError, GetErrorCode)
        USHORT ( STDMETHODCALLTYPE *GetErrorCode )( 
            IMFMediaError * This);
        
        DECLSPEC_XFGVIRT(IMFMediaError, GetExtendedErrorCode)
        HRESULT ( STDMETHODCALLTYPE *GetExtendedErrorCode )( 
            IMFMediaError * This);
        
        DECLSPEC_XFGVIRT(IMFMediaError, SetErrorCode)
        HRESULT ( STDMETHODCALLTYPE *SetErrorCode )( 
            IMFMediaError * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_ERR error);
        
        DECLSPEC_XFGVIRT(IMFMediaError, SetExtendedErrorCode)
        HRESULT ( STDMETHODCALLTYPE *SetExtendedErrorCode )( 
            IMFMediaError * This,
            /* [annotation][in] */ 
            _In_  HRESULT error);
        
        END_INTERFACE
    } IMFMediaErrorVtbl;

    interface IMFMediaError
    {
        CONST_VTBL struct IMFMediaErrorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaError_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaError_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaError_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaError_GetErrorCode(This)	\
    ( (This)->lpVtbl -> GetErrorCode(This) ) 

#define IMFMediaError_GetExtendedErrorCode(This)	\
    ( (This)->lpVtbl -> GetExtendedErrorCode(This) ) 

#define IMFMediaError_SetErrorCode(This,error)	\
    ( (This)->lpVtbl -> SetErrorCode(This,error) ) 

#define IMFMediaError_SetExtendedErrorCode(This,error)	\
    ( (This)->lpVtbl -> SetExtendedErrorCode(This,error) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaError_INTERFACE_DEFINED__ */


#ifndef __IMFMediaTimeRange_INTERFACE_DEFINED__
#define __IMFMediaTimeRange_INTERFACE_DEFINED__

/* interface IMFMediaTimeRange */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaTimeRange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("db71a2fc-078a-414e-9df9-8c2531b0aa6c")
    IMFMediaTimeRange : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetLength( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStart( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  double *pStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnd( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  double *pEnd) = 0;
        
        virtual BOOL STDMETHODCALLTYPE ContainsTime( 
            /* [annotation][in] */ 
            _In_  double time) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddRange( 
            /* [annotation][in] */ 
            _In_  double startTime,
            /* [annotation][in] */ 
            _In_  double endTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaTimeRangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaTimeRange * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaTimeRange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaTimeRange * This);
        
        DECLSPEC_XFGVIRT(IMFMediaTimeRange, GetLength)
        DWORD ( STDMETHODCALLTYPE *GetLength )( 
            IMFMediaTimeRange * This);
        
        DECLSPEC_XFGVIRT(IMFMediaTimeRange, GetStart)
        HRESULT ( STDMETHODCALLTYPE *GetStart )( 
            IMFMediaTimeRange * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  double *pStart);
        
        DECLSPEC_XFGVIRT(IMFMediaTimeRange, GetEnd)
        HRESULT ( STDMETHODCALLTYPE *GetEnd )( 
            IMFMediaTimeRange * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  double *pEnd);
        
        DECLSPEC_XFGVIRT(IMFMediaTimeRange, ContainsTime)
        BOOL ( STDMETHODCALLTYPE *ContainsTime )( 
            IMFMediaTimeRange * This,
            /* [annotation][in] */ 
            _In_  double time);
        
        DECLSPEC_XFGVIRT(IMFMediaTimeRange, AddRange)
        HRESULT ( STDMETHODCALLTYPE *AddRange )( 
            IMFMediaTimeRange * This,
            /* [annotation][in] */ 
            _In_  double startTime,
            /* [annotation][in] */ 
            _In_  double endTime);
        
        DECLSPEC_XFGVIRT(IMFMediaTimeRange, Clear)
        HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IMFMediaTimeRange * This);
        
        END_INTERFACE
    } IMFMediaTimeRangeVtbl;

    interface IMFMediaTimeRange
    {
        CONST_VTBL struct IMFMediaTimeRangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaTimeRange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaTimeRange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaTimeRange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaTimeRange_GetLength(This)	\
    ( (This)->lpVtbl -> GetLength(This) ) 

#define IMFMediaTimeRange_GetStart(This,index,pStart)	\
    ( (This)->lpVtbl -> GetStart(This,index,pStart) ) 

#define IMFMediaTimeRange_GetEnd(This,index,pEnd)	\
    ( (This)->lpVtbl -> GetEnd(This,index,pEnd) ) 

#define IMFMediaTimeRange_ContainsTime(This,time)	\
    ( (This)->lpVtbl -> ContainsTime(This,time) ) 

#define IMFMediaTimeRange_AddRange(This,startTime,endTime)	\
    ( (This)->lpVtbl -> AddRange(This,startTime,endTime) ) 

#define IMFMediaTimeRange_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaTimeRange_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0002 */
/* [local] */ 

typedef 
enum MF_MEDIA_ENGINE_EVENT
    {
        MF_MEDIA_ENGINE_EVENT_LOADSTART	= 1,
        MF_MEDIA_ENGINE_EVENT_PROGRESS	= 2,
        MF_MEDIA_ENGINE_EVENT_SUSPEND	= 3,
        MF_MEDIA_ENGINE_EVENT_ABORT	= 4,
        MF_MEDIA_ENGINE_EVENT_ERROR	= 5,
        MF_MEDIA_ENGINE_EVENT_EMPTIED	= 6,
        MF_MEDIA_ENGINE_EVENT_STALLED	= 7,
        MF_MEDIA_ENGINE_EVENT_PLAY	= 8,
        MF_MEDIA_ENGINE_EVENT_PAUSE	= 9,
        MF_MEDIA_ENGINE_EVENT_LOADEDMETADATA	= 10,
        MF_MEDIA_ENGINE_EVENT_LOADEDDATA	= 11,
        MF_MEDIA_ENGINE_EVENT_WAITING	= 12,
        MF_MEDIA_ENGINE_EVENT_PLAYING	= 13,
        MF_MEDIA_ENGINE_EVENT_CANPLAY	= 14,
        MF_MEDIA_ENGINE_EVENT_CANPLAYTHROUGH	= 15,
        MF_MEDIA_ENGINE_EVENT_SEEKING	= 16,
        MF_MEDIA_ENGINE_EVENT_SEEKED	= 17,
        MF_MEDIA_ENGINE_EVENT_TIMEUPDATE	= 18,
        MF_MEDIA_ENGINE_EVENT_ENDED	= 19,
        MF_MEDIA_ENGINE_EVENT_RATECHANGE	= 20,
        MF_MEDIA_ENGINE_EVENT_DURATIONCHANGE	= 21,
        MF_MEDIA_ENGINE_EVENT_VOLUMECHANGE	= 22,
        MF_MEDIA_ENGINE_EVENT_FORMATCHANGE	= 1000,
        MF_MEDIA_ENGINE_EVENT_PURGEQUEUEDEVENTS	= 1001,
        MF_MEDIA_ENGINE_EVENT_TIMELINE_MARKER	= 1002,
        MF_MEDIA_ENGINE_EVENT_BALANCECHANGE	= 1003,
        MF_MEDIA_ENGINE_EVENT_DOWNLOADCOMPLETE	= 1004,
        MF_MEDIA_ENGINE_EVENT_BUFFERINGSTARTED	= 1005,
        MF_MEDIA_ENGINE_EVENT_BUFFERINGENDED	= 1006,
        MF_MEDIA_ENGINE_EVENT_FRAMESTEPCOMPLETED	= 1007,
        MF_MEDIA_ENGINE_EVENT_NOTIFYSTABLESTATE	= 1008,
        MF_MEDIA_ENGINE_EVENT_FIRSTFRAMEREADY	= 1009,
        MF_MEDIA_ENGINE_EVENT_TRACKSCHANGE	= 1010,
        MF_MEDIA_ENGINE_EVENT_OPMINFO	= 1011,
        MF_MEDIA_ENGINE_EVENT_RESOURCELOST	= 1012,
        MF_MEDIA_ENGINE_EVENT_DELAYLOADEVENT_CHANGED	= 1013,
        MF_MEDIA_ENGINE_EVENT_STREAMRENDERINGERROR	= 1014,
        MF_MEDIA_ENGINE_EVENT_SUPPORTEDRATES_CHANGED	= 1015,
        MF_MEDIA_ENGINE_EVENT_AUDIOENDPOINTCHANGE	= 1016
    } 	MF_MEDIA_ENGINE_EVENT;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0002_v0_0_s_ifspec;

#ifndef __IMFMediaEngineNotify_INTERFACE_DEFINED__
#define __IMFMediaEngineNotify_INTERFACE_DEFINED__

/* interface IMFMediaEngineNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fee7c112-e776-42b5-9bbf-0048524e2bd5")
    IMFMediaEngineNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EventNotify( 
            /* [annotation][in] */ 
            _In_  DWORD event,
            /* [annotation][in] */ 
            _In_  DWORD_PTR param1,
            /* [annotation][in] */ 
            _In_  DWORD param2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineNotify, EventNotify)
        HRESULT ( STDMETHODCALLTYPE *EventNotify )( 
            IMFMediaEngineNotify * This,
            /* [annotation][in] */ 
            _In_  DWORD event,
            /* [annotation][in] */ 
            _In_  DWORD_PTR param1,
            /* [annotation][in] */ 
            _In_  DWORD param2);
        
        END_INTERFACE
    } IMFMediaEngineNotifyVtbl;

    interface IMFMediaEngineNotify
    {
        CONST_VTBL struct IMFMediaEngineNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineNotify_EventNotify(This,event,param1,param2)	\
    ( (This)->lpVtbl -> EventNotify(This,event,param1,param2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineNotify_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineSrcElements_INTERFACE_DEFINED__
#define __IMFMediaEngineSrcElements_INTERFACE_DEFINED__

/* interface IMFMediaEngineSrcElements */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineSrcElements;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a5e5354-b114-4c72-b991-3131d75032ea")
    IMFMediaEngineSrcElements : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetLength( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetURL( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetType( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMedia( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pMedia) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddElement( 
            /* [annotation][in] */ 
            _In_opt_  BSTR pURL,
            /* [annotation][in] */ 
            _In_opt_  BSTR pType,
            /* [annotation][in] */ 
            _In_opt_  BSTR pMedia) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllElements( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineSrcElementsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineSrcElements * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineSrcElements * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineSrcElements * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetLength)
        DWORD ( STDMETHODCALLTYPE *GetLength )( 
            IMFMediaEngineSrcElements * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetURL)
        HRESULT ( STDMETHODCALLTYPE *GetURL )( 
            IMFMediaEngineSrcElements * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pURL);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            IMFMediaEngineSrcElements * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pType);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetMedia)
        HRESULT ( STDMETHODCALLTYPE *GetMedia )( 
            IMFMediaEngineSrcElements * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pMedia);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, AddElement)
        HRESULT ( STDMETHODCALLTYPE *AddElement )( 
            IMFMediaEngineSrcElements * This,
            /* [annotation][in] */ 
            _In_opt_  BSTR pURL,
            /* [annotation][in] */ 
            _In_opt_  BSTR pType,
            /* [annotation][in] */ 
            _In_opt_  BSTR pMedia);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, RemoveAllElements)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllElements )( 
            IMFMediaEngineSrcElements * This);
        
        END_INTERFACE
    } IMFMediaEngineSrcElementsVtbl;

    interface IMFMediaEngineSrcElements
    {
        CONST_VTBL struct IMFMediaEngineSrcElementsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineSrcElements_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineSrcElements_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineSrcElements_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineSrcElements_GetLength(This)	\
    ( (This)->lpVtbl -> GetLength(This) ) 

#define IMFMediaEngineSrcElements_GetURL(This,index,pURL)	\
    ( (This)->lpVtbl -> GetURL(This,index,pURL) ) 

#define IMFMediaEngineSrcElements_GetType(This,index,pType)	\
    ( (This)->lpVtbl -> GetType(This,index,pType) ) 

#define IMFMediaEngineSrcElements_GetMedia(This,index,pMedia)	\
    ( (This)->lpVtbl -> GetMedia(This,index,pMedia) ) 

#define IMFMediaEngineSrcElements_AddElement(This,pURL,pType,pMedia)	\
    ( (This)->lpVtbl -> AddElement(This,pURL,pType,pMedia) ) 

#define IMFMediaEngineSrcElements_RemoveAllElements(This)	\
    ( (This)->lpVtbl -> RemoveAllElements(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineSrcElements_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0004 */
/* [local] */ 

typedef 
enum MF_MEDIA_ENGINE_NETWORK
    {
        MF_MEDIA_ENGINE_NETWORK_EMPTY	= 0,
        MF_MEDIA_ENGINE_NETWORK_IDLE	= 1,
        MF_MEDIA_ENGINE_NETWORK_LOADING	= 2,
        MF_MEDIA_ENGINE_NETWORK_NO_SOURCE	= 3
    } 	MF_MEDIA_ENGINE_NETWORK;

typedef 
enum MF_MEDIA_ENGINE_READY
    {
        MF_MEDIA_ENGINE_READY_HAVE_NOTHING	= 0,
        MF_MEDIA_ENGINE_READY_HAVE_METADATA	= 1,
        MF_MEDIA_ENGINE_READY_HAVE_CURRENT_DATA	= 2,
        MF_MEDIA_ENGINE_READY_HAVE_FUTURE_DATA	= 3,
        MF_MEDIA_ENGINE_READY_HAVE_ENOUGH_DATA	= 4
    } 	MF_MEDIA_ENGINE_READY;

typedef 
enum MF_MEDIA_ENGINE_CANPLAY
    {
        MF_MEDIA_ENGINE_CANPLAY_NOT_SUPPORTED	= 0,
        MF_MEDIA_ENGINE_CANPLAY_MAYBE	= 1,
        MF_MEDIA_ENGINE_CANPLAY_PROBABLY	= 2
    } 	MF_MEDIA_ENGINE_CANPLAY;

typedef 
enum MF_MEDIA_ENGINE_PRELOAD
    {
        MF_MEDIA_ENGINE_PRELOAD_MISSING	= 0,
        MF_MEDIA_ENGINE_PRELOAD_EMPTY	= 1,
        MF_MEDIA_ENGINE_PRELOAD_NONE	= 2,
        MF_MEDIA_ENGINE_PRELOAD_METADATA	= 3,
        MF_MEDIA_ENGINE_PRELOAD_AUTOMATIC	= 4
    } 	MF_MEDIA_ENGINE_PRELOAD;

#ifndef _MFVideoNormalizedRect_
#define _MFVideoNormalizedRect_
typedef struct MFVideoNormalizedRect
    {
    float left;
    float top;
    float right;
    float bottom;
    } 	MFVideoNormalizedRect;

#endif


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0004_v0_0_s_ifspec;

#ifndef __IMFMediaEngine_INTERFACE_DEFINED__
#define __IMFMediaEngine_INTERFACE_DEFINED__

/* interface IMFMediaEngine */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngine;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("98a1b0bb-03eb-4935-ae7c-93c1fa0e1c93")
    IMFMediaEngine : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetError( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetErrorCode( 
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_ERR error) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSourceElements( 
            /* [annotation][in] */ 
            _In_  IMFMediaEngineSrcElements *pSrcElements) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSource( 
            /* [annotation][in] */ 
            _In_  BSTR pUrl) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentSource( 
            /* [annotation][out] */ 
            _Out_  BSTR *ppUrl) = 0;
        
        virtual USHORT STDMETHODCALLTYPE GetNetworkState( void) = 0;
        
        virtual MF_MEDIA_ENGINE_PRELOAD STDMETHODCALLTYPE GetPreload( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreload( 
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_PRELOAD Preload) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBuffered( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppBuffered) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanPlayType( 
            /* [annotation][in] */ 
            _In_  BSTR type,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer) = 0;
        
        virtual USHORT STDMETHODCALLTYPE GetReadyState( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsSeeking( void) = 0;
        
        virtual double STDMETHODCALLTYPE GetCurrentTime( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentTime( 
            /* [annotation][in] */ 
            _In_  double seekTime) = 0;
        
        virtual double STDMETHODCALLTYPE GetStartTime( void) = 0;
        
        virtual double STDMETHODCALLTYPE GetDuration( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsPaused( void) = 0;
        
        virtual double STDMETHODCALLTYPE GetDefaultPlaybackRate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultPlaybackRate( 
            /* [annotation][in] */ 
            _In_  double Rate) = 0;
        
        virtual double STDMETHODCALLTYPE GetPlaybackRate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPlaybackRate( 
            /* [annotation][in] */ 
            _In_  double Rate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlayed( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppPlayed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSeekable( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppSeekable) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsEnded( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE GetAutoPlay( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAutoPlay( 
            /* [annotation][in] */ 
            _In_  BOOL AutoPlay) = 0;
        
        virtual BOOL STDMETHODCALLTYPE GetLoop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLoop( 
            /* [annotation][in] */ 
            _In_  BOOL Loop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Play( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE GetMuted( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMuted( 
            /* [annotation][in] */ 
            _In_  BOOL Muted) = 0;
        
        virtual double STDMETHODCALLTYPE GetVolume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVolume( 
            /* [annotation][in] */ 
            _In_  double Volume) = 0;
        
        virtual BOOL STDMETHODCALLTYPE HasVideo( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE HasAudio( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNativeVideoSize( 
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoAspectRatio( 
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransferVideoFrame( 
            /* [annotation][in] */ 
            _In_  IUnknown *pDstSurf,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnVideoStreamTick( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pPts) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngine * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetErrorCode)
        HRESULT ( STDMETHODCALLTYPE *SetErrorCode )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_ERR error);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetSourceElements)
        HRESULT ( STDMETHODCALLTYPE *SetSourceElements )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  IMFMediaEngineSrcElements *pSrcElements);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetSource)
        HRESULT ( STDMETHODCALLTYPE *SetSource )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  BSTR pUrl);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetCurrentSource)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSource )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Out_  BSTR *ppUrl);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetNetworkState)
        USHORT ( STDMETHODCALLTYPE *GetNetworkState )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPreload)
        MF_MEDIA_ENGINE_PRELOAD ( STDMETHODCALLTYPE *GetPreload )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetPreload)
        HRESULT ( STDMETHODCALLTYPE *SetPreload )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_PRELOAD Preload);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetBuffered)
        HRESULT ( STDMETHODCALLTYPE *GetBuffered )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppBuffered);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, CanPlayType)
        HRESULT ( STDMETHODCALLTYPE *CanPlayType )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  BSTR type,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetReadyState)
        USHORT ( STDMETHODCALLTYPE *GetReadyState )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsSeeking)
        BOOL ( STDMETHODCALLTYPE *IsSeeking )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetCurrentTime)
        double ( STDMETHODCALLTYPE *GetCurrentTime )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetCurrentTime)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentTime )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  double seekTime);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetStartTime)
        double ( STDMETHODCALLTYPE *GetStartTime )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetDuration)
        double ( STDMETHODCALLTYPE *GetDuration )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsPaused)
        BOOL ( STDMETHODCALLTYPE *IsPaused )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetDefaultPlaybackRate)
        double ( STDMETHODCALLTYPE *GetDefaultPlaybackRate )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetDefaultPlaybackRate)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultPlaybackRate )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  double Rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPlaybackRate)
        double ( STDMETHODCALLTYPE *GetPlaybackRate )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetPlaybackRate)
        HRESULT ( STDMETHODCALLTYPE *SetPlaybackRate )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  double Rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPlayed)
        HRESULT ( STDMETHODCALLTYPE *GetPlayed )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppPlayed);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetSeekable)
        HRESULT ( STDMETHODCALLTYPE *GetSeekable )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppSeekable);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsEnded)
        BOOL ( STDMETHODCALLTYPE *IsEnded )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetAutoPlay)
        BOOL ( STDMETHODCALLTYPE *GetAutoPlay )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetAutoPlay)
        HRESULT ( STDMETHODCALLTYPE *SetAutoPlay )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL AutoPlay);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetLoop)
        BOOL ( STDMETHODCALLTYPE *GetLoop )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetLoop)
        HRESULT ( STDMETHODCALLTYPE *SetLoop )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL Loop);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Play)
        HRESULT ( STDMETHODCALLTYPE *Play )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetMuted)
        BOOL ( STDMETHODCALLTYPE *GetMuted )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetMuted)
        HRESULT ( STDMETHODCALLTYPE *SetMuted )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  BOOL Muted);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetVolume)
        double ( STDMETHODCALLTYPE *GetVolume )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetVolume)
        HRESULT ( STDMETHODCALLTYPE *SetVolume )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  double Volume);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, HasVideo)
        BOOL ( STDMETHODCALLTYPE *HasVideo )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, HasAudio)
        BOOL ( STDMETHODCALLTYPE *HasAudio )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetNativeVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetNativeVideoSize )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetVideoAspectRatio)
        HRESULT ( STDMETHODCALLTYPE *GetVideoAspectRatio )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaEngine * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, TransferVideoFrame)
        HRESULT ( STDMETHODCALLTYPE *TransferVideoFrame )( 
            IMFMediaEngine * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pDstSurf,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, OnVideoStreamTick)
        HRESULT ( STDMETHODCALLTYPE *OnVideoStreamTick )( 
            IMFMediaEngine * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pPts);
        
        END_INTERFACE
    } IMFMediaEngineVtbl;

    interface IMFMediaEngine
    {
        CONST_VTBL struct IMFMediaEngineVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngine_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngine_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngine_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngine_GetError(This,ppError)	\
    ( (This)->lpVtbl -> GetError(This,ppError) ) 

#define IMFMediaEngine_SetErrorCode(This,error)	\
    ( (This)->lpVtbl -> SetErrorCode(This,error) ) 

#define IMFMediaEngine_SetSourceElements(This,pSrcElements)	\
    ( (This)->lpVtbl -> SetSourceElements(This,pSrcElements) ) 

#define IMFMediaEngine_SetSource(This,pUrl)	\
    ( (This)->lpVtbl -> SetSource(This,pUrl) ) 

#define IMFMediaEngine_GetCurrentSource(This,ppUrl)	\
    ( (This)->lpVtbl -> GetCurrentSource(This,ppUrl) ) 

#define IMFMediaEngine_GetNetworkState(This)	\
    ( (This)->lpVtbl -> GetNetworkState(This) ) 

#define IMFMediaEngine_GetPreload(This)	\
    ( (This)->lpVtbl -> GetPreload(This) ) 

#define IMFMediaEngine_SetPreload(This,Preload)	\
    ( (This)->lpVtbl -> SetPreload(This,Preload) ) 

#define IMFMediaEngine_GetBuffered(This,ppBuffered)	\
    ( (This)->lpVtbl -> GetBuffered(This,ppBuffered) ) 

#define IMFMediaEngine_Load(This)	\
    ( (This)->lpVtbl -> Load(This) ) 

#define IMFMediaEngine_CanPlayType(This,type,pAnswer)	\
    ( (This)->lpVtbl -> CanPlayType(This,type,pAnswer) ) 

#define IMFMediaEngine_GetReadyState(This)	\
    ( (This)->lpVtbl -> GetReadyState(This) ) 

#define IMFMediaEngine_IsSeeking(This)	\
    ( (This)->lpVtbl -> IsSeeking(This) ) 

#define IMFMediaEngine_GetCurrentTime(This)	\
    ( (This)->lpVtbl -> GetCurrentTime(This) ) 

#define IMFMediaEngine_SetCurrentTime(This,seekTime)	\
    ( (This)->lpVtbl -> SetCurrentTime(This,seekTime) ) 

#define IMFMediaEngine_GetStartTime(This)	\
    ( (This)->lpVtbl -> GetStartTime(This) ) 

#define IMFMediaEngine_GetDuration(This)	\
    ( (This)->lpVtbl -> GetDuration(This) ) 

#define IMFMediaEngine_IsPaused(This)	\
    ( (This)->lpVtbl -> IsPaused(This) ) 

#define IMFMediaEngine_GetDefaultPlaybackRate(This)	\
    ( (This)->lpVtbl -> GetDefaultPlaybackRate(This) ) 

#define IMFMediaEngine_SetDefaultPlaybackRate(This,Rate)	\
    ( (This)->lpVtbl -> SetDefaultPlaybackRate(This,Rate) ) 

#define IMFMediaEngine_GetPlaybackRate(This)	\
    ( (This)->lpVtbl -> GetPlaybackRate(This) ) 

#define IMFMediaEngine_SetPlaybackRate(This,Rate)	\
    ( (This)->lpVtbl -> SetPlaybackRate(This,Rate) ) 

#define IMFMediaEngine_GetPlayed(This,ppPlayed)	\
    ( (This)->lpVtbl -> GetPlayed(This,ppPlayed) ) 

#define IMFMediaEngine_GetSeekable(This,ppSeekable)	\
    ( (This)->lpVtbl -> GetSeekable(This,ppSeekable) ) 

#define IMFMediaEngine_IsEnded(This)	\
    ( (This)->lpVtbl -> IsEnded(This) ) 

#define IMFMediaEngine_GetAutoPlay(This)	\
    ( (This)->lpVtbl -> GetAutoPlay(This) ) 

#define IMFMediaEngine_SetAutoPlay(This,AutoPlay)	\
    ( (This)->lpVtbl -> SetAutoPlay(This,AutoPlay) ) 

#define IMFMediaEngine_GetLoop(This)	\
    ( (This)->lpVtbl -> GetLoop(This) ) 

#define IMFMediaEngine_SetLoop(This,Loop)	\
    ( (This)->lpVtbl -> SetLoop(This,Loop) ) 

#define IMFMediaEngine_Play(This)	\
    ( (This)->lpVtbl -> Play(This) ) 

#define IMFMediaEngine_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaEngine_GetMuted(This)	\
    ( (This)->lpVtbl -> GetMuted(This) ) 

#define IMFMediaEngine_SetMuted(This,Muted)	\
    ( (This)->lpVtbl -> SetMuted(This,Muted) ) 

#define IMFMediaEngine_GetVolume(This)	\
    ( (This)->lpVtbl -> GetVolume(This) ) 

#define IMFMediaEngine_SetVolume(This,Volume)	\
    ( (This)->lpVtbl -> SetVolume(This,Volume) ) 

#define IMFMediaEngine_HasVideo(This)	\
    ( (This)->lpVtbl -> HasVideo(This) ) 

#define IMFMediaEngine_HasAudio(This)	\
    ( (This)->lpVtbl -> HasAudio(This) ) 

#define IMFMediaEngine_GetNativeVideoSize(This,cx,cy)	\
    ( (This)->lpVtbl -> GetNativeVideoSize(This,cx,cy) ) 

#define IMFMediaEngine_GetVideoAspectRatio(This,cx,cy)	\
    ( (This)->lpVtbl -> GetVideoAspectRatio(This,cx,cy) ) 

#define IMFMediaEngine_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFMediaEngine_TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr)	\
    ( (This)->lpVtbl -> TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr) ) 

#define IMFMediaEngine_OnVideoStreamTick(This,pPts)	\
    ( (This)->lpVtbl -> OnVideoStreamTick(This,pPts) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngine_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0005 */
/* [local] */ 

typedef 
enum MF_MEDIA_ENGINE_S3D_PACKING_MODE
    {
        MF_MEDIA_ENGINE_S3D_PACKING_MODE_NONE	= 0,
        MF_MEDIA_ENGINE_S3D_PACKING_MODE_SIDE_BY_SIDE	= 1,
        MF_MEDIA_ENGINE_S3D_PACKING_MODE_TOP_BOTTOM	= 2
    } 	MF_MEDIA_ENGINE_S3D_PACKING_MODE;

typedef 
enum MF_MEDIA_ENGINE_STATISTIC
    {
        MF_MEDIA_ENGINE_STATISTIC_FRAMES_RENDERED	= 0,
        MF_MEDIA_ENGINE_STATISTIC_FRAMES_DROPPED	= 1,
        MF_MEDIA_ENGINE_STATISTIC_BYTES_DOWNLOADED	= 2,
        MF_MEDIA_ENGINE_STATISTIC_BUFFER_PROGRESS	= 3,
        MF_MEDIA_ENGINE_STATISTIC_FRAMES_PER_SECOND	= 4,
        MF_MEDIA_ENGINE_STATISTIC_PLAYBACK_JITTER	= 5,
        MF_MEDIA_ENGINE_STATISTIC_FRAMES_CORRUPTED	= 6,
        MF_MEDIA_ENGINE_STATISTIC_TOTAL_FRAME_DELAY	= 7
    } 	MF_MEDIA_ENGINE_STATISTIC;

typedef 
enum MF_MEDIA_ENGINE_SEEK_MODE
    {
        MF_MEDIA_ENGINE_SEEK_MODE_NORMAL	= 0,
        MF_MEDIA_ENGINE_SEEK_MODE_APPROXIMATE	= 1
    } 	MF_MEDIA_ENGINE_SEEK_MODE;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0005_v0_0_s_ifspec;

#ifndef __IMFMediaEngineEx_INTERFACE_DEFINED__
#define __IMFMediaEngineEx_INTERFACE_DEFINED__

/* interface IMFMediaEngineEx */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83015ead-b1e6-40d0-a98a-37145ffe1ad1")
    IMFMediaEngineEx : public IMFMediaEngine
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSourceFromByteStream( 
            /* [annotation][in] */ 
            _In_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  BSTR pURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatistics( 
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_STATISTIC StatisticID,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pStatistic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateVideoStream( 
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_opt_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr) = 0;
        
        virtual double STDMETHODCALLTYPE GetBalance( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBalance( 
            /* [annotation][in] */ 
            _In_  double balance) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsPlaybackRateSupported( 
            /* [annotation][in] */ 
            _In_  double rate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FrameStep( 
            /* [annotation][in] */ 
            _In_  BOOL Forward) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetResourceCharacteristics( 
            /* [annotation][out] */ 
            _Out_  DWORD *pCharacteristics) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentationAttribute( 
            /* [annotation][in] */ 
            _In_  REFGUID guidMFAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfStreams( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamAttribute( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidMFAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamSelection( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamSelection( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL Enabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ApplyStreamSelections( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsProtected( 
            /* [annotation][out] */ 
            _Out_  BOOL *pProtected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertVideoEffect( 
            /* [annotation][in] */ 
            _In_  IUnknown *pEffect,
            /* [annotation][in] */ 
            _In_  BOOL fOptional) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InsertAudioEffect( 
            /* [annotation][in] */ 
            _In_  IUnknown *pEffect,
            /* [annotation][in] */ 
            _In_  BOOL fOptional) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllEffects( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimelineMarkerTimer( 
            /* [annotation][in] */ 
            _In_  double timeToFire) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimelineMarkerTimer( 
            /* [annotation][out] */ 
            _Out_  double *pTimeToFire) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelTimelineMarkerTimer( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsStereo3D( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStereo3DFramePackingMode( 
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_S3D_PACKING_MODE *packMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStereo3DFramePackingMode( 
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_S3D_PACKING_MODE packMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStereo3DRenderMode( 
            /* [annotation][out] */ 
            _Out_  MF3DVideoOutputType *outputType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStereo3DRenderMode( 
            /* [annotation][in] */ 
            _In_  MF3DVideoOutputType outputType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableWindowlessSwapchainMode( 
            /* [annotation][in] */ 
            _In_  BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoSwapchainHandle( 
            /* [annotation][out] */ 
            _Out_  HANDLE *phSwapchain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableHorizontalMirrorMode( 
            /* [annotation][in] */ 
            _In_  BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioStreamCategory( 
            /* [annotation][out] */ 
            _Out_  UINT32 *pCategory) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAudioStreamCategory( 
            /* [annotation][in] */ 
            _In_  UINT32 category) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioEndpointRole( 
            /* [annotation][out] */ 
            _Out_  UINT32 *pRole) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAudioEndpointRole( 
            /* [annotation][in] */ 
            _In_  UINT32 role) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRealTimeMode( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRealTimeMode( 
            /* [annotation][in] */ 
            _In_  BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCurrentTimeEx( 
            /* [annotation][in] */ 
            _In_  double seekTime,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_SEEK_MODE seekMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableTimeUpdateTimer( 
            /* [annotation][in] */ 
            _In_  BOOL fEnableTimer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetErrorCode)
        HRESULT ( STDMETHODCALLTYPE *SetErrorCode )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_ERR error);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetSourceElements)
        HRESULT ( STDMETHODCALLTYPE *SetSourceElements )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  IMFMediaEngineSrcElements *pSrcElements);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetSource)
        HRESULT ( STDMETHODCALLTYPE *SetSource )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BSTR pUrl);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetCurrentSource)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSource )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  BSTR *ppUrl);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetNetworkState)
        USHORT ( STDMETHODCALLTYPE *GetNetworkState )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPreload)
        MF_MEDIA_ENGINE_PRELOAD ( STDMETHODCALLTYPE *GetPreload )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetPreload)
        HRESULT ( STDMETHODCALLTYPE *SetPreload )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_PRELOAD Preload);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetBuffered)
        HRESULT ( STDMETHODCALLTYPE *GetBuffered )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppBuffered);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, CanPlayType)
        HRESULT ( STDMETHODCALLTYPE *CanPlayType )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BSTR type,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetReadyState)
        USHORT ( STDMETHODCALLTYPE *GetReadyState )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsSeeking)
        BOOL ( STDMETHODCALLTYPE *IsSeeking )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetCurrentTime)
        double ( STDMETHODCALLTYPE *GetCurrentTime )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetCurrentTime)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentTime )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double seekTime);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetStartTime)
        double ( STDMETHODCALLTYPE *GetStartTime )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetDuration)
        double ( STDMETHODCALLTYPE *GetDuration )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsPaused)
        BOOL ( STDMETHODCALLTYPE *IsPaused )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetDefaultPlaybackRate)
        double ( STDMETHODCALLTYPE *GetDefaultPlaybackRate )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetDefaultPlaybackRate)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultPlaybackRate )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double Rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPlaybackRate)
        double ( STDMETHODCALLTYPE *GetPlaybackRate )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetPlaybackRate)
        HRESULT ( STDMETHODCALLTYPE *SetPlaybackRate )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double Rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetPlayed)
        HRESULT ( STDMETHODCALLTYPE *GetPlayed )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppPlayed);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetSeekable)
        HRESULT ( STDMETHODCALLTYPE *GetSeekable )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppSeekable);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, IsEnded)
        BOOL ( STDMETHODCALLTYPE *IsEnded )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetAutoPlay)
        BOOL ( STDMETHODCALLTYPE *GetAutoPlay )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetAutoPlay)
        HRESULT ( STDMETHODCALLTYPE *SetAutoPlay )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL AutoPlay);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetLoop)
        BOOL ( STDMETHODCALLTYPE *GetLoop )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetLoop)
        HRESULT ( STDMETHODCALLTYPE *SetLoop )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL Loop);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Play)
        HRESULT ( STDMETHODCALLTYPE *Play )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetMuted)
        BOOL ( STDMETHODCALLTYPE *GetMuted )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetMuted)
        HRESULT ( STDMETHODCALLTYPE *SetMuted )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL Muted);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetVolume)
        double ( STDMETHODCALLTYPE *GetVolume )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, SetVolume)
        HRESULT ( STDMETHODCALLTYPE *SetVolume )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double Volume);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, HasVideo)
        BOOL ( STDMETHODCALLTYPE *HasVideo )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, HasAudio)
        BOOL ( STDMETHODCALLTYPE *HasAudio )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetNativeVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetNativeVideoSize )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, GetVideoAspectRatio)
        HRESULT ( STDMETHODCALLTYPE *GetVideoAspectRatio )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cx,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *cy);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, TransferVideoFrame)
        HRESULT ( STDMETHODCALLTYPE *TransferVideoFrame )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pDstSurf,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr);
        
        DECLSPEC_XFGVIRT(IMFMediaEngine, OnVideoStreamTick)
        HRESULT ( STDMETHODCALLTYPE *OnVideoStreamTick )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pPts);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetSourceFromByteStream)
        HRESULT ( STDMETHODCALLTYPE *SetSourceFromByteStream )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  BSTR pURL);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetStatistics)
        HRESULT ( STDMETHODCALLTYPE *GetStatistics )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_STATISTIC StatisticID,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pStatistic);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, UpdateVideoStream)
        HRESULT ( STDMETHODCALLTYPE *UpdateVideoStream )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_opt_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetBalance)
        double ( STDMETHODCALLTYPE *GetBalance )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetBalance)
        HRESULT ( STDMETHODCALLTYPE *SetBalance )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double balance);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, IsPlaybackRateSupported)
        BOOL ( STDMETHODCALLTYPE *IsPlaybackRateSupported )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double rate);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, FrameStep)
        HRESULT ( STDMETHODCALLTYPE *FrameStep )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL Forward);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetResourceCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetResourceCharacteristics )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetPresentationAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationAttribute )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  REFGUID guidMFAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfStreams )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwStreamCount);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetStreamAttribute)
        HRESULT ( STDMETHODCALLTYPE *GetStreamAttribute )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  REFGUID guidMFAttribute,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetStreamSelection)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSelection )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */ 
            _Out_  BOOL *pEnabled);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetStreamSelection)
        HRESULT ( STDMETHODCALLTYPE *SetStreamSelection )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  BOOL Enabled);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, ApplyStreamSelections)
        HRESULT ( STDMETHODCALLTYPE *ApplyStreamSelections )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, IsProtected)
        HRESULT ( STDMETHODCALLTYPE *IsProtected )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pProtected);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, InsertVideoEffect)
        HRESULT ( STDMETHODCALLTYPE *InsertVideoEffect )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pEffect,
            /* [annotation][in] */ 
            _In_  BOOL fOptional);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, InsertAudioEffect)
        HRESULT ( STDMETHODCALLTYPE *InsertAudioEffect )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pEffect,
            /* [annotation][in] */ 
            _In_  BOOL fOptional);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, RemoveAllEffects)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllEffects )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetTimelineMarkerTimer)
        HRESULT ( STDMETHODCALLTYPE *SetTimelineMarkerTimer )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double timeToFire);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetTimelineMarkerTimer)
        HRESULT ( STDMETHODCALLTYPE *GetTimelineMarkerTimer )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  double *pTimeToFire);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, CancelTimelineMarkerTimer)
        HRESULT ( STDMETHODCALLTYPE *CancelTimelineMarkerTimer )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, IsStereo3D)
        BOOL ( STDMETHODCALLTYPE *IsStereo3D )( 
            IMFMediaEngineEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetStereo3DFramePackingMode)
        HRESULT ( STDMETHODCALLTYPE *GetStereo3DFramePackingMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_S3D_PACKING_MODE *packMode);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetStereo3DFramePackingMode)
        HRESULT ( STDMETHODCALLTYPE *SetStereo3DFramePackingMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_S3D_PACKING_MODE packMode);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetStereo3DRenderMode)
        HRESULT ( STDMETHODCALLTYPE *GetStereo3DRenderMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  MF3DVideoOutputType *outputType);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetStereo3DRenderMode)
        HRESULT ( STDMETHODCALLTYPE *SetStereo3DRenderMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  MF3DVideoOutputType outputType);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, EnableWindowlessSwapchainMode)
        HRESULT ( STDMETHODCALLTYPE *EnableWindowlessSwapchainMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetVideoSwapchainHandle)
        HRESULT ( STDMETHODCALLTYPE *GetVideoSwapchainHandle )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  HANDLE *phSwapchain);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, EnableHorizontalMirrorMode)
        HRESULT ( STDMETHODCALLTYPE *EnableHorizontalMirrorMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetAudioStreamCategory)
        HRESULT ( STDMETHODCALLTYPE *GetAudioStreamCategory )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *pCategory);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetAudioStreamCategory)
        HRESULT ( STDMETHODCALLTYPE *SetAudioStreamCategory )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  UINT32 category);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetAudioEndpointRole)
        HRESULT ( STDMETHODCALLTYPE *GetAudioEndpointRole )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  UINT32 *pRole);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetAudioEndpointRole)
        HRESULT ( STDMETHODCALLTYPE *SetAudioEndpointRole )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  UINT32 role);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, GetRealTimeMode)
        HRESULT ( STDMETHODCALLTYPE *GetRealTimeMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfEnabled);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetRealTimeMode)
        HRESULT ( STDMETHODCALLTYPE *SetRealTimeMode )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, SetCurrentTimeEx)
        HRESULT ( STDMETHODCALLTYPE *SetCurrentTimeEx )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  double seekTime,
            /* [annotation][in] */ 
            _In_  MF_MEDIA_ENGINE_SEEK_MODE seekMode);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEx, EnableTimeUpdateTimer)
        HRESULT ( STDMETHODCALLTYPE *EnableTimeUpdateTimer )( 
            IMFMediaEngineEx * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnableTimer);
        
        END_INTERFACE
    } IMFMediaEngineExVtbl;

    interface IMFMediaEngineEx
    {
        CONST_VTBL struct IMFMediaEngineExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineEx_GetError(This,ppError)	\
    ( (This)->lpVtbl -> GetError(This,ppError) ) 

#define IMFMediaEngineEx_SetErrorCode(This,error)	\
    ( (This)->lpVtbl -> SetErrorCode(This,error) ) 

#define IMFMediaEngineEx_SetSourceElements(This,pSrcElements)	\
    ( (This)->lpVtbl -> SetSourceElements(This,pSrcElements) ) 

#define IMFMediaEngineEx_SetSource(This,pUrl)	\
    ( (This)->lpVtbl -> SetSource(This,pUrl) ) 

#define IMFMediaEngineEx_GetCurrentSource(This,ppUrl)	\
    ( (This)->lpVtbl -> GetCurrentSource(This,ppUrl) ) 

#define IMFMediaEngineEx_GetNetworkState(This)	\
    ( (This)->lpVtbl -> GetNetworkState(This) ) 

#define IMFMediaEngineEx_GetPreload(This)	\
    ( (This)->lpVtbl -> GetPreload(This) ) 

#define IMFMediaEngineEx_SetPreload(This,Preload)	\
    ( (This)->lpVtbl -> SetPreload(This,Preload) ) 

#define IMFMediaEngineEx_GetBuffered(This,ppBuffered)	\
    ( (This)->lpVtbl -> GetBuffered(This,ppBuffered) ) 

#define IMFMediaEngineEx_Load(This)	\
    ( (This)->lpVtbl -> Load(This) ) 

#define IMFMediaEngineEx_CanPlayType(This,type,pAnswer)	\
    ( (This)->lpVtbl -> CanPlayType(This,type,pAnswer) ) 

#define IMFMediaEngineEx_GetReadyState(This)	\
    ( (This)->lpVtbl -> GetReadyState(This) ) 

#define IMFMediaEngineEx_IsSeeking(This)	\
    ( (This)->lpVtbl -> IsSeeking(This) ) 

#define IMFMediaEngineEx_GetCurrentTime(This)	\
    ( (This)->lpVtbl -> GetCurrentTime(This) ) 

#define IMFMediaEngineEx_SetCurrentTime(This,seekTime)	\
    ( (This)->lpVtbl -> SetCurrentTime(This,seekTime) ) 

#define IMFMediaEngineEx_GetStartTime(This)	\
    ( (This)->lpVtbl -> GetStartTime(This) ) 

#define IMFMediaEngineEx_GetDuration(This)	\
    ( (This)->lpVtbl -> GetDuration(This) ) 

#define IMFMediaEngineEx_IsPaused(This)	\
    ( (This)->lpVtbl -> IsPaused(This) ) 

#define IMFMediaEngineEx_GetDefaultPlaybackRate(This)	\
    ( (This)->lpVtbl -> GetDefaultPlaybackRate(This) ) 

#define IMFMediaEngineEx_SetDefaultPlaybackRate(This,Rate)	\
    ( (This)->lpVtbl -> SetDefaultPlaybackRate(This,Rate) ) 

#define IMFMediaEngineEx_GetPlaybackRate(This)	\
    ( (This)->lpVtbl -> GetPlaybackRate(This) ) 

#define IMFMediaEngineEx_SetPlaybackRate(This,Rate)	\
    ( (This)->lpVtbl -> SetPlaybackRate(This,Rate) ) 

#define IMFMediaEngineEx_GetPlayed(This,ppPlayed)	\
    ( (This)->lpVtbl -> GetPlayed(This,ppPlayed) ) 

#define IMFMediaEngineEx_GetSeekable(This,ppSeekable)	\
    ( (This)->lpVtbl -> GetSeekable(This,ppSeekable) ) 

#define IMFMediaEngineEx_IsEnded(This)	\
    ( (This)->lpVtbl -> IsEnded(This) ) 

#define IMFMediaEngineEx_GetAutoPlay(This)	\
    ( (This)->lpVtbl -> GetAutoPlay(This) ) 

#define IMFMediaEngineEx_SetAutoPlay(This,AutoPlay)	\
    ( (This)->lpVtbl -> SetAutoPlay(This,AutoPlay) ) 

#define IMFMediaEngineEx_GetLoop(This)	\
    ( (This)->lpVtbl -> GetLoop(This) ) 

#define IMFMediaEngineEx_SetLoop(This,Loop)	\
    ( (This)->lpVtbl -> SetLoop(This,Loop) ) 

#define IMFMediaEngineEx_Play(This)	\
    ( (This)->lpVtbl -> Play(This) ) 

#define IMFMediaEngineEx_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaEngineEx_GetMuted(This)	\
    ( (This)->lpVtbl -> GetMuted(This) ) 

#define IMFMediaEngineEx_SetMuted(This,Muted)	\
    ( (This)->lpVtbl -> SetMuted(This,Muted) ) 

#define IMFMediaEngineEx_GetVolume(This)	\
    ( (This)->lpVtbl -> GetVolume(This) ) 

#define IMFMediaEngineEx_SetVolume(This,Volume)	\
    ( (This)->lpVtbl -> SetVolume(This,Volume) ) 

#define IMFMediaEngineEx_HasVideo(This)	\
    ( (This)->lpVtbl -> HasVideo(This) ) 

#define IMFMediaEngineEx_HasAudio(This)	\
    ( (This)->lpVtbl -> HasAudio(This) ) 

#define IMFMediaEngineEx_GetNativeVideoSize(This,cx,cy)	\
    ( (This)->lpVtbl -> GetNativeVideoSize(This,cx,cy) ) 

#define IMFMediaEngineEx_GetVideoAspectRatio(This,cx,cy)	\
    ( (This)->lpVtbl -> GetVideoAspectRatio(This,cx,cy) ) 

#define IMFMediaEngineEx_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFMediaEngineEx_TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr)	\
    ( (This)->lpVtbl -> TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr) ) 

#define IMFMediaEngineEx_OnVideoStreamTick(This,pPts)	\
    ( (This)->lpVtbl -> OnVideoStreamTick(This,pPts) ) 


#define IMFMediaEngineEx_SetSourceFromByteStream(This,pByteStream,pURL)	\
    ( (This)->lpVtbl -> SetSourceFromByteStream(This,pByteStream,pURL) ) 

#define IMFMediaEngineEx_GetStatistics(This,StatisticID,pStatistic)	\
    ( (This)->lpVtbl -> GetStatistics(This,StatisticID,pStatistic) ) 

#define IMFMediaEngineEx_UpdateVideoStream(This,pSrc,pDst,pBorderClr)	\
    ( (This)->lpVtbl -> UpdateVideoStream(This,pSrc,pDst,pBorderClr) ) 

#define IMFMediaEngineEx_GetBalance(This)	\
    ( (This)->lpVtbl -> GetBalance(This) ) 

#define IMFMediaEngineEx_SetBalance(This,balance)	\
    ( (This)->lpVtbl -> SetBalance(This,balance) ) 

#define IMFMediaEngineEx_IsPlaybackRateSupported(This,rate)	\
    ( (This)->lpVtbl -> IsPlaybackRateSupported(This,rate) ) 

#define IMFMediaEngineEx_FrameStep(This,Forward)	\
    ( (This)->lpVtbl -> FrameStep(This,Forward) ) 

#define IMFMediaEngineEx_GetResourceCharacteristics(This,pCharacteristics)	\
    ( (This)->lpVtbl -> GetResourceCharacteristics(This,pCharacteristics) ) 

#define IMFMediaEngineEx_GetPresentationAttribute(This,guidMFAttribute,pvValue)	\
    ( (This)->lpVtbl -> GetPresentationAttribute(This,guidMFAttribute,pvValue) ) 

#define IMFMediaEngineEx_GetNumberOfStreams(This,pdwStreamCount)	\
    ( (This)->lpVtbl -> GetNumberOfStreams(This,pdwStreamCount) ) 

#define IMFMediaEngineEx_GetStreamAttribute(This,dwStreamIndex,guidMFAttribute,pvValue)	\
    ( (This)->lpVtbl -> GetStreamAttribute(This,dwStreamIndex,guidMFAttribute,pvValue) ) 

#define IMFMediaEngineEx_GetStreamSelection(This,dwStreamIndex,pEnabled)	\
    ( (This)->lpVtbl -> GetStreamSelection(This,dwStreamIndex,pEnabled) ) 

#define IMFMediaEngineEx_SetStreamSelection(This,dwStreamIndex,Enabled)	\
    ( (This)->lpVtbl -> SetStreamSelection(This,dwStreamIndex,Enabled) ) 

#define IMFMediaEngineEx_ApplyStreamSelections(This)	\
    ( (This)->lpVtbl -> ApplyStreamSelections(This) ) 

#define IMFMediaEngineEx_IsProtected(This,pProtected)	\
    ( (This)->lpVtbl -> IsProtected(This,pProtected) ) 

#define IMFMediaEngineEx_InsertVideoEffect(This,pEffect,fOptional)	\
    ( (This)->lpVtbl -> InsertVideoEffect(This,pEffect,fOptional) ) 

#define IMFMediaEngineEx_InsertAudioEffect(This,pEffect,fOptional)	\
    ( (This)->lpVtbl -> InsertAudioEffect(This,pEffect,fOptional) ) 

#define IMFMediaEngineEx_RemoveAllEffects(This)	\
    ( (This)->lpVtbl -> RemoveAllEffects(This) ) 

#define IMFMediaEngineEx_SetTimelineMarkerTimer(This,timeToFire)	\
    ( (This)->lpVtbl -> SetTimelineMarkerTimer(This,timeToFire) ) 

#define IMFMediaEngineEx_GetTimelineMarkerTimer(This,pTimeToFire)	\
    ( (This)->lpVtbl -> GetTimelineMarkerTimer(This,pTimeToFire) ) 

#define IMFMediaEngineEx_CancelTimelineMarkerTimer(This)	\
    ( (This)->lpVtbl -> CancelTimelineMarkerTimer(This) ) 

#define IMFMediaEngineEx_IsStereo3D(This)	\
    ( (This)->lpVtbl -> IsStereo3D(This) ) 

#define IMFMediaEngineEx_GetStereo3DFramePackingMode(This,packMode)	\
    ( (This)->lpVtbl -> GetStereo3DFramePackingMode(This,packMode) ) 

#define IMFMediaEngineEx_SetStereo3DFramePackingMode(This,packMode)	\
    ( (This)->lpVtbl -> SetStereo3DFramePackingMode(This,packMode) ) 

#define IMFMediaEngineEx_GetStereo3DRenderMode(This,outputType)	\
    ( (This)->lpVtbl -> GetStereo3DRenderMode(This,outputType) ) 

#define IMFMediaEngineEx_SetStereo3DRenderMode(This,outputType)	\
    ( (This)->lpVtbl -> SetStereo3DRenderMode(This,outputType) ) 

#define IMFMediaEngineEx_EnableWindowlessSwapchainMode(This,fEnable)	\
    ( (This)->lpVtbl -> EnableWindowlessSwapchainMode(This,fEnable) ) 

#define IMFMediaEngineEx_GetVideoSwapchainHandle(This,phSwapchain)	\
    ( (This)->lpVtbl -> GetVideoSwapchainHandle(This,phSwapchain) ) 

#define IMFMediaEngineEx_EnableHorizontalMirrorMode(This,fEnable)	\
    ( (This)->lpVtbl -> EnableHorizontalMirrorMode(This,fEnable) ) 

#define IMFMediaEngineEx_GetAudioStreamCategory(This,pCategory)	\
    ( (This)->lpVtbl -> GetAudioStreamCategory(This,pCategory) ) 

#define IMFMediaEngineEx_SetAudioStreamCategory(This,category)	\
    ( (This)->lpVtbl -> SetAudioStreamCategory(This,category) ) 

#define IMFMediaEngineEx_GetAudioEndpointRole(This,pRole)	\
    ( (This)->lpVtbl -> GetAudioEndpointRole(This,pRole) ) 

#define IMFMediaEngineEx_SetAudioEndpointRole(This,role)	\
    ( (This)->lpVtbl -> SetAudioEndpointRole(This,role) ) 

#define IMFMediaEngineEx_GetRealTimeMode(This,pfEnabled)	\
    ( (This)->lpVtbl -> GetRealTimeMode(This,pfEnabled) ) 

#define IMFMediaEngineEx_SetRealTimeMode(This,fEnable)	\
    ( (This)->lpVtbl -> SetRealTimeMode(This,fEnable) ) 

#define IMFMediaEngineEx_SetCurrentTimeEx(This,seekTime,seekMode)	\
    ( (This)->lpVtbl -> SetCurrentTimeEx(This,seekTime,seekMode) ) 

#define IMFMediaEngineEx_EnableTimeUpdateTimer(This,fEnableTimer)	\
    ( (This)->lpVtbl -> EnableTimeUpdateTimer(This,fEnableTimer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineEx_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineAudioEndpointId_INTERFACE_DEFINED__
#define __IMFMediaEngineAudioEndpointId_INTERFACE_DEFINED__

/* interface IMFMediaEngineAudioEndpointId */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineAudioEndpointId;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a3bac98-0e76-49fb-8c20-8a86fd98eaf2")
    IMFMediaEngineAudioEndpointId : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAudioEndpointId( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszEndpointId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioEndpointId( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppszEndpointId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineAudioEndpointIdVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineAudioEndpointId * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineAudioEndpointId * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineAudioEndpointId * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineAudioEndpointId, SetAudioEndpointId)
        HRESULT ( STDMETHODCALLTYPE *SetAudioEndpointId )( 
            IMFMediaEngineAudioEndpointId * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR pszEndpointId);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineAudioEndpointId, GetAudioEndpointId)
        HRESULT ( STDMETHODCALLTYPE *GetAudioEndpointId )( 
            IMFMediaEngineAudioEndpointId * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppszEndpointId);
        
        END_INTERFACE
    } IMFMediaEngineAudioEndpointIdVtbl;

    interface IMFMediaEngineAudioEndpointId
    {
        CONST_VTBL struct IMFMediaEngineAudioEndpointIdVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineAudioEndpointId_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineAudioEndpointId_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineAudioEndpointId_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineAudioEndpointId_SetAudioEndpointId(This,pszEndpointId)	\
    ( (This)->lpVtbl -> SetAudioEndpointId(This,pszEndpointId) ) 

#define IMFMediaEngineAudioEndpointId_GetAudioEndpointId(This,ppszEndpointId)	\
    ( (This)->lpVtbl -> GetAudioEndpointId(This,ppszEndpointId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineAudioEndpointId_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0007 */
/* [local] */ 

typedef 
enum MF_MEDIA_ENGINE_EXTENSION_TYPE
    {
        MF_MEDIA_ENGINE_EXTENSION_TYPE_MEDIASOURCE	= 0,
        MF_MEDIA_ENGINE_EXTENSION_TYPE_BYTESTREAM	= 1
    } 	MF_MEDIA_ENGINE_EXTENSION_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0007_v0_0_s_ifspec;

#ifndef __IMFMediaEngineExtension_INTERFACE_DEFINED__
#define __IMFMediaEngineExtension_INTERFACE_DEFINED__

/* interface IMFMediaEngineExtension */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2f69d622-20b5-41e9-afdf-89ced1dda04e")
    IMFMediaEngineExtension : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CanPlayType( 
            /* [annotation][in] */ 
            _In_  BOOL AudioOnly,
            /* [annotation][in] */ 
            _In_  BSTR MimeType,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginCreateObject( 
            /* [annotation][in] */ 
            _In_  BSTR bstrURL,
            /* [annotation][in] */ 
            _In_opt_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  MF_OBJECT_TYPE type,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppIUnknownCancelCookie,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelObjectCreation( 
            /* [annotation][in] */ 
            _In_  IUnknown *pIUnknownCancelCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndCreateObject( 
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineExtension * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineExtension * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineExtension, CanPlayType)
        HRESULT ( STDMETHODCALLTYPE *CanPlayType )( 
            IMFMediaEngineExtension * This,
            /* [annotation][in] */ 
            _In_  BOOL AudioOnly,
            /* [annotation][in] */ 
            _In_  BSTR MimeType,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineExtension, BeginCreateObject)
        HRESULT ( STDMETHODCALLTYPE *BeginCreateObject )( 
            IMFMediaEngineExtension * This,
            /* [annotation][in] */ 
            _In_  BSTR bstrURL,
            /* [annotation][in] */ 
            _In_opt_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  MF_OBJECT_TYPE type,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppIUnknownCancelCookie,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineExtension, CancelObjectCreation)
        HRESULT ( STDMETHODCALLTYPE *CancelObjectCreation )( 
            IMFMediaEngineExtension * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pIUnknownCancelCookie);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineExtension, EndCreateObject)
        HRESULT ( STDMETHODCALLTYPE *EndCreateObject )( 
            IMFMediaEngineExtension * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        END_INTERFACE
    } IMFMediaEngineExtensionVtbl;

    interface IMFMediaEngineExtension
    {
        CONST_VTBL struct IMFMediaEngineExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineExtension_CanPlayType(This,AudioOnly,MimeType,pAnswer)	\
    ( (This)->lpVtbl -> CanPlayType(This,AudioOnly,MimeType,pAnswer) ) 

#define IMFMediaEngineExtension_BeginCreateObject(This,bstrURL,pByteStream,type,ppIUnknownCancelCookie,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginCreateObject(This,bstrURL,pByteStream,type,ppIUnknownCancelCookie,pCallback,punkState) ) 

#define IMFMediaEngineExtension_CancelObjectCreation(This,pIUnknownCancelCookie)	\
    ( (This)->lpVtbl -> CancelObjectCreation(This,pIUnknownCancelCookie) ) 

#define IMFMediaEngineExtension_EndCreateObject(This,pResult,ppObject)	\
    ( (This)->lpVtbl -> EndCreateObject(This,pResult,ppObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineExtension_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0008 */
/* [local] */ 


enum MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS
    {
        MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_PROTECTED	= 0x1,
        MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_REQUIRES_SURFACE_PROTECTION	= 0x2,
        MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_REQUIRES_ANTI_SCREEN_SCRAPE_PROTECTION	= 0x4
    } ;


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0008_v0_0_s_ifspec;

#ifndef __IMFMediaEngineProtectedContent_INTERFACE_DEFINED__
#define __IMFMediaEngineProtectedContent_INTERFACE_DEFINED__

/* interface IMFMediaEngineProtectedContent */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineProtectedContent;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9f8021e8-9c8c-487e-bb5c-79aa4779938c")
    IMFMediaEngineProtectedContent : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShareResources( 
            /* [annotation] */ 
            _In_  IUnknown *pUnkDeviceContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRequiredProtections( 
            /* [annotation][out] */ 
            _Out_  DWORD *pFrameProtectionFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOPMWindow( 
            /* [annotation][in] */ 
            _In_  HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TransferVideoFrame( 
            /* [annotation][in] */ 
            _In_  IUnknown *pDstSurf,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr,
            /* [annotation][out] */ 
            _Out_  DWORD *pFrameProtectionFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContentProtectionManager( 
            /* [annotation][in] */ 
            _In_opt_  IMFContentProtectionManager *pCPM) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetApplicationCertificate( 
            /* [annotation][in] */ 
            _In_reads_bytes_(cbBlob)  const BYTE *pbBlob,
            /* [annotation][in] */ 
            _In_  DWORD cbBlob) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineProtectedContentVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineProtectedContent * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineProtectedContent * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineProtectedContent * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineProtectedContent, ShareResources)
        HRESULT ( STDMETHODCALLTYPE *ShareResources )( 
            IMFMediaEngineProtectedContent * This,
            /* [annotation] */ 
            _In_  IUnknown *pUnkDeviceContext);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineProtectedContent, GetRequiredProtections)
        HRESULT ( STDMETHODCALLTYPE *GetRequiredProtections )( 
            IMFMediaEngineProtectedContent * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pFrameProtectionFlags);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineProtectedContent, SetOPMWindow)
        HRESULT ( STDMETHODCALLTYPE *SetOPMWindow )( 
            IMFMediaEngineProtectedContent * This,
            /* [annotation][in] */ 
            _In_  HWND hwnd);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineProtectedContent, TransferVideoFrame)
        HRESULT ( STDMETHODCALLTYPE *TransferVideoFrame )( 
            IMFMediaEngineProtectedContent * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pDstSurf,
            /* [annotation][in] */ 
            _In_opt_  const MFVideoNormalizedRect *pSrc,
            /* [annotation][in] */ 
            _In_  const RECT *pDst,
            /* [annotation][in] */ 
            _In_opt_  const MFARGB *pBorderClr,
            /* [annotation][out] */ 
            _Out_  DWORD *pFrameProtectionFlags);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineProtectedContent, SetContentProtectionManager)
        HRESULT ( STDMETHODCALLTYPE *SetContentProtectionManager )( 
            IMFMediaEngineProtectedContent * This,
            /* [annotation][in] */ 
            _In_opt_  IMFContentProtectionManager *pCPM);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineProtectedContent, SetApplicationCertificate)
        HRESULT ( STDMETHODCALLTYPE *SetApplicationCertificate )( 
            IMFMediaEngineProtectedContent * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(cbBlob)  const BYTE *pbBlob,
            /* [annotation][in] */ 
            _In_  DWORD cbBlob);
        
        END_INTERFACE
    } IMFMediaEngineProtectedContentVtbl;

    interface IMFMediaEngineProtectedContent
    {
        CONST_VTBL struct IMFMediaEngineProtectedContentVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineProtectedContent_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineProtectedContent_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineProtectedContent_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineProtectedContent_ShareResources(This,pUnkDeviceContext)	\
    ( (This)->lpVtbl -> ShareResources(This,pUnkDeviceContext) ) 

#define IMFMediaEngineProtectedContent_GetRequiredProtections(This,pFrameProtectionFlags)	\
    ( (This)->lpVtbl -> GetRequiredProtections(This,pFrameProtectionFlags) ) 

#define IMFMediaEngineProtectedContent_SetOPMWindow(This,hwnd)	\
    ( (This)->lpVtbl -> SetOPMWindow(This,hwnd) ) 

#define IMFMediaEngineProtectedContent_TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr,pFrameProtectionFlags)	\
    ( (This)->lpVtbl -> TransferVideoFrame(This,pDstSurf,pSrc,pDst,pBorderClr,pFrameProtectionFlags) ) 

#define IMFMediaEngineProtectedContent_SetContentProtectionManager(This,pCPM)	\
    ( (This)->lpVtbl -> SetContentProtectionManager(This,pCPM) ) 

#define IMFMediaEngineProtectedContent_SetApplicationCertificate(This,pbBlob,cbBlob)	\
    ( (This)->lpVtbl -> SetApplicationCertificate(This,pbBlob,cbBlob) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineProtectedContent_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0009 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0009_v0_0_s_ifspec;

#ifndef __IAudioSourceProvider_INTERFACE_DEFINED__
#define __IAudioSourceProvider_INTERFACE_DEFINED__

/* interface IAudioSourceProvider */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IAudioSourceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EBBAF249-AFC2-4582-91C6-B60DF2E84954")
    IAudioSourceProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ProvideInput( 
            /* [annotation][in] */ 
            _In_  DWORD dwSampleCount,
            /* [annotation][in] */ 
            _Inout_  DWORD *pdwChannelCount,
            /* [annotation][out] */ 
            _Out_writes_opt_(dwSampleCount * (*pdwChannelCount))  float *pInterleavedAudioData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAudioSourceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAudioSourceProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAudioSourceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAudioSourceProvider * This);
        
        DECLSPEC_XFGVIRT(IAudioSourceProvider, ProvideInput)
        HRESULT ( STDMETHODCALLTYPE *ProvideInput )( 
            IAudioSourceProvider * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSampleCount,
            /* [annotation][in] */ 
            _Inout_  DWORD *pdwChannelCount,
            /* [annotation][out] */ 
            _Out_writes_opt_(dwSampleCount * (*pdwChannelCount))  float *pInterleavedAudioData);
        
        END_INTERFACE
    } IAudioSourceProviderVtbl;

    interface IAudioSourceProvider
    {
        CONST_VTBL struct IAudioSourceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAudioSourceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAudioSourceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAudioSourceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAudioSourceProvider_ProvideInput(This,dwSampleCount,pdwChannelCount,pInterleavedAudioData)	\
    ( (This)->lpVtbl -> ProvideInput(This,dwSampleCount,pdwChannelCount,pInterleavedAudioData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAudioSourceProvider_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineWebSupport_INTERFACE_DEFINED__
#define __IMFMediaEngineWebSupport_INTERFACE_DEFINED__

/* interface IMFMediaEngineWebSupport */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineWebSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ba2743a1-07e0-48ef-84b6-9a2ed023ca6c")
    IMFMediaEngineWebSupport : public IUnknown
    {
    public:
        virtual BOOL STDMETHODCALLTYPE ShouldDelayTheLoadEvent( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectWebAudio( 
            /* [annotation][in] */ 
            _In_  DWORD dwSampleRate,
            /* [annotation][out] */ 
            _COM_Outptr_  IAudioSourceProvider **ppSourceProvider) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisconnectWebAudio( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineWebSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineWebSupport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineWebSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineWebSupport * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineWebSupport, ShouldDelayTheLoadEvent)
        BOOL ( STDMETHODCALLTYPE *ShouldDelayTheLoadEvent )( 
            IMFMediaEngineWebSupport * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineWebSupport, ConnectWebAudio)
        HRESULT ( STDMETHODCALLTYPE *ConnectWebAudio )( 
            IMFMediaEngineWebSupport * This,
            /* [annotation][in] */ 
            _In_  DWORD dwSampleRate,
            /* [annotation][out] */ 
            _COM_Outptr_  IAudioSourceProvider **ppSourceProvider);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineWebSupport, DisconnectWebAudio)
        HRESULT ( STDMETHODCALLTYPE *DisconnectWebAudio )( 
            IMFMediaEngineWebSupport * This);
        
        END_INTERFACE
    } IMFMediaEngineWebSupportVtbl;

    interface IMFMediaEngineWebSupport
    {
        CONST_VTBL struct IMFMediaEngineWebSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineWebSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineWebSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineWebSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineWebSupport_ShouldDelayTheLoadEvent(This)	\
    ( (This)->lpVtbl -> ShouldDelayTheLoadEvent(This) ) 

#define IMFMediaEngineWebSupport_ConnectWebAudio(This,dwSampleRate,ppSourceProvider)	\
    ( (This)->lpVtbl -> ConnectWebAudio(This,dwSampleRate,ppSourceProvider) ) 

#define IMFMediaEngineWebSupport_DisconnectWebAudio(This)	\
    ( (This)->lpVtbl -> DisconnectWebAudio(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineWebSupport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0011 */
/* [local] */ 

EXTERN_GUID(MF_MSE_CALLBACK,
0x9063a7c0, 0x42c5, 0x4ffd, 0xa8, 0xa8, 0x6f, 0xcf, 0x9e, 0xa3, 0xd0, 0x0c);
EXTERN_GUID(MF_MSE_ACTIVELIST_CALLBACK,
0x949bda0f, 0x4549, 0x46d5, 0xad, 0x7f, 0xb8, 0x46, 0xe1, 0xab, 0x16, 0x52);
EXTERN_GUID(MF_MSE_BUFFERLIST_CALLBACK,
0x42e669b0, 0xd60e, 0x4afb, 0xa8, 0x5b, 0xd8, 0xe5, 0xfe, 0x6b, 0xda, 0xb5);
EXTERN_GUID(MF_MSE_VP9_SUPPORT,
0x92d78429, 0xd88b, 0x4ff0, 0x83, 0x22, 0x80, 0x3e, 0xfa, 0x6e, 0x96, 0x26);
EXTERN_GUID(MF_MSE_OPUS_SUPPORT,
0x4d224cc1, 0x8cc4, 0x48a3, 0xa7, 0xa7, 0xe4, 0xc1, 0x6c, 0xe6, 0x38, 0x8a);
typedef 
enum MF_MSE_VP9_SUPPORT_TYPE
    {
        MF_MSE_VP9_SUPPORT_DEFAULT	= 0,
        MF_MSE_VP9_SUPPORT_ON	= 1,
        MF_MSE_VP9_SUPPORT_OFF	= 2
    } 	MF_MSE_VP9_SUPPORT_TYPE;

typedef 
enum MF_MSE_OPUS_SUPPORT_TYPE
    {
        MF_MSE_OPUS_SUPPORT_ON	= 0,
        MF_MSE_OPUS_SUPPORT_OFF	= 1
    } 	MF_MSE_OPUS_SUPPORT_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0011_v0_0_s_ifspec;

#ifndef __IMFMediaSourceExtensionNotify_INTERFACE_DEFINED__
#define __IMFMediaSourceExtensionNotify_INTERFACE_DEFINED__

/* interface IMFMediaSourceExtensionNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSourceExtensionNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a7901327-05dd-4469-a7b7-0e01979e361d")
    IMFMediaSourceExtensionNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnSourceOpen( void) = 0;
        
        virtual void STDMETHODCALLTYPE OnSourceEnded( void) = 0;
        
        virtual void STDMETHODCALLTYPE OnSourceClose( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourceExtensionNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaSourceExtensionNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaSourceExtensionNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaSourceExtensionNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtensionNotify, OnSourceOpen)
        void ( STDMETHODCALLTYPE *OnSourceOpen )( 
            IMFMediaSourceExtensionNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtensionNotify, OnSourceEnded)
        void ( STDMETHODCALLTYPE *OnSourceEnded )( 
            IMFMediaSourceExtensionNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtensionNotify, OnSourceClose)
        void ( STDMETHODCALLTYPE *OnSourceClose )( 
            IMFMediaSourceExtensionNotify * This);
        
        END_INTERFACE
    } IMFMediaSourceExtensionNotifyVtbl;

    interface IMFMediaSourceExtensionNotify
    {
        CONST_VTBL struct IMFMediaSourceExtensionNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSourceExtensionNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSourceExtensionNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSourceExtensionNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSourceExtensionNotify_OnSourceOpen(This)	\
    ( (This)->lpVtbl -> OnSourceOpen(This) ) 

#define IMFMediaSourceExtensionNotify_OnSourceEnded(This)	\
    ( (This)->lpVtbl -> OnSourceEnded(This) ) 

#define IMFMediaSourceExtensionNotify_OnSourceClose(This)	\
    ( (This)->lpVtbl -> OnSourceClose(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSourceExtensionNotify_INTERFACE_DEFINED__ */


#ifndef __IMFBufferListNotify_INTERFACE_DEFINED__
#define __IMFBufferListNotify_INTERFACE_DEFINED__

/* interface IMFBufferListNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFBufferListNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24cd47f7-81d8-4785-adb2-af697a963cd2")
    IMFBufferListNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnAddSourceBuffer( void) = 0;
        
        virtual void STDMETHODCALLTYPE OnRemoveSourceBuffer( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFBufferListNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFBufferListNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFBufferListNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFBufferListNotify * This);
        
        DECLSPEC_XFGVIRT(IMFBufferListNotify, OnAddSourceBuffer)
        void ( STDMETHODCALLTYPE *OnAddSourceBuffer )( 
            IMFBufferListNotify * This);
        
        DECLSPEC_XFGVIRT(IMFBufferListNotify, OnRemoveSourceBuffer)
        void ( STDMETHODCALLTYPE *OnRemoveSourceBuffer )( 
            IMFBufferListNotify * This);
        
        END_INTERFACE
    } IMFBufferListNotifyVtbl;

    interface IMFBufferListNotify
    {
        CONST_VTBL struct IMFBufferListNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFBufferListNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFBufferListNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFBufferListNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFBufferListNotify_OnAddSourceBuffer(This)	\
    ( (This)->lpVtbl -> OnAddSourceBuffer(This) ) 

#define IMFBufferListNotify_OnRemoveSourceBuffer(This)	\
    ( (This)->lpVtbl -> OnRemoveSourceBuffer(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFBufferListNotify_INTERFACE_DEFINED__ */


#ifndef __IMFSourceBufferNotify_INTERFACE_DEFINED__
#define __IMFSourceBufferNotify_INTERFACE_DEFINED__

/* interface IMFSourceBufferNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceBufferNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("87e47623-2ceb-45d6-9b88-d8520c4dcbbc")
    IMFSourceBufferNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnUpdateStart( void) = 0;
        
        virtual void STDMETHODCALLTYPE OnAbort( void) = 0;
        
        virtual void STDMETHODCALLTYPE OnError( 
            /* [annotation][in] */ 
            _In_  HRESULT hr) = 0;
        
        virtual void STDMETHODCALLTYPE OnUpdate( void) = 0;
        
        virtual void STDMETHODCALLTYPE OnUpdateEnd( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceBufferNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceBufferNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceBufferNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceBufferNotify * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferNotify, OnUpdateStart)
        void ( STDMETHODCALLTYPE *OnUpdateStart )( 
            IMFSourceBufferNotify * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferNotify, OnAbort)
        void ( STDMETHODCALLTYPE *OnAbort )( 
            IMFSourceBufferNotify * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferNotify, OnError)
        void ( STDMETHODCALLTYPE *OnError )( 
            IMFSourceBufferNotify * This,
            /* [annotation][in] */ 
            _In_  HRESULT hr);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferNotify, OnUpdate)
        void ( STDMETHODCALLTYPE *OnUpdate )( 
            IMFSourceBufferNotify * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferNotify, OnUpdateEnd)
        void ( STDMETHODCALLTYPE *OnUpdateEnd )( 
            IMFSourceBufferNotify * This);
        
        END_INTERFACE
    } IMFSourceBufferNotifyVtbl;

    interface IMFSourceBufferNotify
    {
        CONST_VTBL struct IMFSourceBufferNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceBufferNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceBufferNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceBufferNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceBufferNotify_OnUpdateStart(This)	\
    ( (This)->lpVtbl -> OnUpdateStart(This) ) 

#define IMFSourceBufferNotify_OnAbort(This)	\
    ( (This)->lpVtbl -> OnAbort(This) ) 

#define IMFSourceBufferNotify_OnError(This,hr)	\
    ( (This)->lpVtbl -> OnError(This,hr) ) 

#define IMFSourceBufferNotify_OnUpdate(This)	\
    ( (This)->lpVtbl -> OnUpdate(This) ) 

#define IMFSourceBufferNotify_OnUpdateEnd(This)	\
    ( (This)->lpVtbl -> OnUpdateEnd(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceBufferNotify_INTERFACE_DEFINED__ */


#ifndef __IMFSourceBuffer_INTERFACE_DEFINED__
#define __IMFSourceBuffer_INTERFACE_DEFINED__

/* interface IMFSourceBuffer */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceBuffer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e2cd3a4b-af25-4d3d-9110-da0e6f8ee877")
    IMFSourceBuffer : public IUnknown
    {
    public:
        virtual BOOL STDMETHODCALLTYPE GetUpdating( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBuffered( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppBuffered) = 0;
        
        virtual double STDMETHODCALLTYPE GetTimeStampOffset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimeStampOffset( 
            /* [annotation][in] */ 
            _In_  double offset) = 0;
        
        virtual double STDMETHODCALLTYPE GetAppendWindowStart( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAppendWindowStart( 
            /* [annotation][in] */ 
            _In_  double time) = 0;
        
        virtual double STDMETHODCALLTYPE GetAppendWindowEnd( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAppendWindowEnd( 
            /* [annotation][in] */ 
            _In_  double time) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Append( 
            /* [annotation][in] */ 
            _In_reads_bytes_(len)  const BYTE *pData,
            /* [annotation][in] */ 
            _In_  DWORD len) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AppendByteStream( 
            /* [annotation][in] */ 
            _In_  IMFByteStream *pStream,
            /* [annotation][in] */ 
            _In_opt_  DWORDLONG *pMaxLen) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( 
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double end) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceBufferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceBuffer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, GetUpdating)
        BOOL ( STDMETHODCALLTYPE *GetUpdating )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, GetBuffered)
        HRESULT ( STDMETHODCALLTYPE *GetBuffered )( 
            IMFSourceBuffer * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppBuffered);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, GetTimeStampOffset)
        double ( STDMETHODCALLTYPE *GetTimeStampOffset )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, SetTimeStampOffset)
        HRESULT ( STDMETHODCALLTYPE *SetTimeStampOffset )( 
            IMFSourceBuffer * This,
            /* [annotation][in] */ 
            _In_  double offset);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, GetAppendWindowStart)
        double ( STDMETHODCALLTYPE *GetAppendWindowStart )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, SetAppendWindowStart)
        HRESULT ( STDMETHODCALLTYPE *SetAppendWindowStart )( 
            IMFSourceBuffer * This,
            /* [annotation][in] */ 
            _In_  double time);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, GetAppendWindowEnd)
        double ( STDMETHODCALLTYPE *GetAppendWindowEnd )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, SetAppendWindowEnd)
        HRESULT ( STDMETHODCALLTYPE *SetAppendWindowEnd )( 
            IMFSourceBuffer * This,
            /* [annotation][in] */ 
            _In_  double time);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, Append)
        HRESULT ( STDMETHODCALLTYPE *Append )( 
            IMFSourceBuffer * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(len)  const BYTE *pData,
            /* [annotation][in] */ 
            _In_  DWORD len);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, AppendByteStream)
        HRESULT ( STDMETHODCALLTYPE *AppendByteStream )( 
            IMFSourceBuffer * This,
            /* [annotation][in] */ 
            _In_  IMFByteStream *pStream,
            /* [annotation][in] */ 
            _In_opt_  DWORDLONG *pMaxLen);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IMFSourceBuffer * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBuffer, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IMFSourceBuffer * This,
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double end);
        
        END_INTERFACE
    } IMFSourceBufferVtbl;

    interface IMFSourceBuffer
    {
        CONST_VTBL struct IMFSourceBufferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceBuffer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceBuffer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceBuffer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceBuffer_GetUpdating(This)	\
    ( (This)->lpVtbl -> GetUpdating(This) ) 

#define IMFSourceBuffer_GetBuffered(This,ppBuffered)	\
    ( (This)->lpVtbl -> GetBuffered(This,ppBuffered) ) 

#define IMFSourceBuffer_GetTimeStampOffset(This)	\
    ( (This)->lpVtbl -> GetTimeStampOffset(This) ) 

#define IMFSourceBuffer_SetTimeStampOffset(This,offset)	\
    ( (This)->lpVtbl -> SetTimeStampOffset(This,offset) ) 

#define IMFSourceBuffer_GetAppendWindowStart(This)	\
    ( (This)->lpVtbl -> GetAppendWindowStart(This) ) 

#define IMFSourceBuffer_SetAppendWindowStart(This,time)	\
    ( (This)->lpVtbl -> SetAppendWindowStart(This,time) ) 

#define IMFSourceBuffer_GetAppendWindowEnd(This)	\
    ( (This)->lpVtbl -> GetAppendWindowEnd(This) ) 

#define IMFSourceBuffer_SetAppendWindowEnd(This,time)	\
    ( (This)->lpVtbl -> SetAppendWindowEnd(This,time) ) 

#define IMFSourceBuffer_Append(This,pData,len)	\
    ( (This)->lpVtbl -> Append(This,pData,len) ) 

#define IMFSourceBuffer_AppendByteStream(This,pStream,pMaxLen)	\
    ( (This)->lpVtbl -> AppendByteStream(This,pStream,pMaxLen) ) 

#define IMFSourceBuffer_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IMFSourceBuffer_Remove(This,start,end)	\
    ( (This)->lpVtbl -> Remove(This,start,end) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceBuffer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0015 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
typedef 
enum MF_MSE_APPEND_MODE
    {
        MF_MSE_APPEND_MODE_SEGMENTS	= 0,
        MF_MSE_APPEND_MODE_SEQUENCE	= 1
    } 	MF_MSE_APPEND_MODE;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0015_v0_0_s_ifspec;

#ifndef __IMFSourceBufferAppendMode_INTERFACE_DEFINED__
#define __IMFSourceBufferAppendMode_INTERFACE_DEFINED__

/* interface IMFSourceBufferAppendMode */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceBufferAppendMode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19666fb4-babe-4c55-bc03-0a074da37e2a")
    IMFSourceBufferAppendMode : public IUnknown
    {
    public:
        virtual MF_MSE_APPEND_MODE STDMETHODCALLTYPE GetAppendMode( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAppendMode( 
            /* [annotation][in] */ 
            _In_  MF_MSE_APPEND_MODE mode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceBufferAppendModeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceBufferAppendMode * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceBufferAppendMode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceBufferAppendMode * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferAppendMode, GetAppendMode)
        MF_MSE_APPEND_MODE ( STDMETHODCALLTYPE *GetAppendMode )( 
            IMFSourceBufferAppendMode * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferAppendMode, SetAppendMode)
        HRESULT ( STDMETHODCALLTYPE *SetAppendMode )( 
            IMFSourceBufferAppendMode * This,
            /* [annotation][in] */ 
            _In_  MF_MSE_APPEND_MODE mode);
        
        END_INTERFACE
    } IMFSourceBufferAppendModeVtbl;

    interface IMFSourceBufferAppendMode
    {
        CONST_VTBL struct IMFSourceBufferAppendModeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceBufferAppendMode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceBufferAppendMode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceBufferAppendMode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceBufferAppendMode_GetAppendMode(This)	\
    ( (This)->lpVtbl -> GetAppendMode(This) ) 

#define IMFSourceBufferAppendMode_SetAppendMode(This,mode)	\
    ( (This)->lpVtbl -> SetAppendMode(This,mode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceBufferAppendMode_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0016 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0016_v0_0_s_ifspec;

#ifndef __IMFSourceBufferList_INTERFACE_DEFINED__
#define __IMFSourceBufferList_INTERFACE_DEFINED__

/* interface IMFSourceBufferList */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFSourceBufferList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("249981f8-8325-41f3-b80c-3b9e3aad0cbe")
    IMFSourceBufferList : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetLength( void) = 0;
        
        virtual IMFSourceBuffer *STDMETHODCALLTYPE GetSourceBuffer( 
            /* [annotation][in] */ 
            _In_  DWORD index) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceBufferListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSourceBufferList * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSourceBufferList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSourceBufferList * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferList, GetLength)
        DWORD ( STDMETHODCALLTYPE *GetLength )( 
            IMFSourceBufferList * This);
        
        DECLSPEC_XFGVIRT(IMFSourceBufferList, GetSourceBuffer)
        IMFSourceBuffer *( STDMETHODCALLTYPE *GetSourceBuffer )( 
            IMFSourceBufferList * This,
            /* [annotation][in] */ 
            _In_  DWORD index);
        
        END_INTERFACE
    } IMFSourceBufferListVtbl;

    interface IMFSourceBufferList
    {
        CONST_VTBL struct IMFSourceBufferListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceBufferList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceBufferList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceBufferList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceBufferList_GetLength(This)	\
    ( (This)->lpVtbl -> GetLength(This) ) 

#define IMFSourceBufferList_GetSourceBuffer(This,index)	\
    ( (This)->lpVtbl -> GetSourceBuffer(This,index) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceBufferList_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0017 */
/* [local] */ 

typedef 
enum MF_MSE_READY
    {
        MF_MSE_READY_CLOSED	= 1,
        MF_MSE_READY_OPEN	= 2,
        MF_MSE_READY_ENDED	= 3
    } 	MF_MSE_READY;

typedef 
enum MF_MSE_ERROR
    {
        MF_MSE_ERROR_NOERROR	= 0,
        MF_MSE_ERROR_NETWORK	= 1,
        MF_MSE_ERROR_DECODE	= 2,
        MF_MSE_ERROR_UNKNOWN_ERROR	= 3
    } 	MF_MSE_ERROR;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0017_v0_0_s_ifspec;

#ifndef __IMFMediaSourceExtension_INTERFACE_DEFINED__
#define __IMFMediaSourceExtension_INTERFACE_DEFINED__

/* interface IMFMediaSourceExtension */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSourceExtension;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e467b94e-a713-4562-a802-816a42e9008a")
    IMFMediaSourceExtension : public IUnknown
    {
    public:
        virtual IMFSourceBufferList *STDMETHODCALLTYPE GetSourceBuffers( void) = 0;
        
        virtual IMFSourceBufferList *STDMETHODCALLTYPE GetActiveSourceBuffers( void) = 0;
        
        virtual MF_MSE_READY STDMETHODCALLTYPE GetReadyState( void) = 0;
        
        virtual double STDMETHODCALLTYPE GetDuration( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDuration( 
            /* [annotation][in] */ 
            _In_  double duration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSourceBuffer( 
            /* [annotation][in] */ 
            _In_  BSTR type,
            /* [annotation][in] */ 
            _In_  IMFSourceBufferNotify *pNotify,
            /* [annotation][out] */ 
            _Outptr_  IMFSourceBuffer **ppSourceBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveSourceBuffer( 
            /* [annotation][in] */ 
            _In_  IMFSourceBuffer *pSourceBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEndOfStream( 
            /* [annotation][in] */ 
            _In_  MF_MSE_ERROR error) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsTypeSupported( 
            /* [annotation][in] */ 
            _In_  BSTR type) = 0;
        
        virtual IMFSourceBuffer *STDMETHODCALLTYPE GetSourceBuffer( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourceExtensionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaSourceExtension * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaSourceExtension * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaSourceExtension * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, GetSourceBuffers)
        IMFSourceBufferList *( STDMETHODCALLTYPE *GetSourceBuffers )( 
            IMFMediaSourceExtension * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, GetActiveSourceBuffers)
        IMFSourceBufferList *( STDMETHODCALLTYPE *GetActiveSourceBuffers )( 
            IMFMediaSourceExtension * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, GetReadyState)
        MF_MSE_READY ( STDMETHODCALLTYPE *GetReadyState )( 
            IMFMediaSourceExtension * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, GetDuration)
        double ( STDMETHODCALLTYPE *GetDuration )( 
            IMFMediaSourceExtension * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, SetDuration)
        HRESULT ( STDMETHODCALLTYPE *SetDuration )( 
            IMFMediaSourceExtension * This,
            /* [annotation][in] */ 
            _In_  double duration);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, AddSourceBuffer)
        HRESULT ( STDMETHODCALLTYPE *AddSourceBuffer )( 
            IMFMediaSourceExtension * This,
            /* [annotation][in] */ 
            _In_  BSTR type,
            /* [annotation][in] */ 
            _In_  IMFSourceBufferNotify *pNotify,
            /* [annotation][out] */ 
            _Outptr_  IMFSourceBuffer **ppSourceBuffer);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, RemoveSourceBuffer)
        HRESULT ( STDMETHODCALLTYPE *RemoveSourceBuffer )( 
            IMFMediaSourceExtension * This,
            /* [annotation][in] */ 
            _In_  IMFSourceBuffer *pSourceBuffer);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, SetEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *SetEndOfStream )( 
            IMFMediaSourceExtension * This,
            /* [annotation][in] */ 
            _In_  MF_MSE_ERROR error);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, IsTypeSupported)
        BOOL ( STDMETHODCALLTYPE *IsTypeSupported )( 
            IMFMediaSourceExtension * This,
            /* [annotation][in] */ 
            _In_  BSTR type);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtension, GetSourceBuffer)
        IMFSourceBuffer *( STDMETHODCALLTYPE *GetSourceBuffer )( 
            IMFMediaSourceExtension * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex);
        
        END_INTERFACE
    } IMFMediaSourceExtensionVtbl;

    interface IMFMediaSourceExtension
    {
        CONST_VTBL struct IMFMediaSourceExtensionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSourceExtension_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSourceExtension_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSourceExtension_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSourceExtension_GetSourceBuffers(This)	\
    ( (This)->lpVtbl -> GetSourceBuffers(This) ) 

#define IMFMediaSourceExtension_GetActiveSourceBuffers(This)	\
    ( (This)->lpVtbl -> GetActiveSourceBuffers(This) ) 

#define IMFMediaSourceExtension_GetReadyState(This)	\
    ( (This)->lpVtbl -> GetReadyState(This) ) 

#define IMFMediaSourceExtension_GetDuration(This)	\
    ( (This)->lpVtbl -> GetDuration(This) ) 

#define IMFMediaSourceExtension_SetDuration(This,duration)	\
    ( (This)->lpVtbl -> SetDuration(This,duration) ) 

#define IMFMediaSourceExtension_AddSourceBuffer(This,type,pNotify,ppSourceBuffer)	\
    ( (This)->lpVtbl -> AddSourceBuffer(This,type,pNotify,ppSourceBuffer) ) 

#define IMFMediaSourceExtension_RemoveSourceBuffer(This,pSourceBuffer)	\
    ( (This)->lpVtbl -> RemoveSourceBuffer(This,pSourceBuffer) ) 

#define IMFMediaSourceExtension_SetEndOfStream(This,error)	\
    ( (This)->lpVtbl -> SetEndOfStream(This,error) ) 

#define IMFMediaSourceExtension_IsTypeSupported(This,type)	\
    ( (This)->lpVtbl -> IsTypeSupported(This,type) ) 

#define IMFMediaSourceExtension_GetSourceBuffer(This,dwStreamIndex)	\
    ( (This)->lpVtbl -> GetSourceBuffer(This,dwStreamIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSourceExtension_INTERFACE_DEFINED__ */


#ifndef __IMFMediaSourceExtensionLiveSeekableRange_INTERFACE_DEFINED__
#define __IMFMediaSourceExtensionLiveSeekableRange_INTERFACE_DEFINED__

/* interface IMFMediaSourceExtensionLiveSeekableRange */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSourceExtensionLiveSeekableRange;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5D1ABFD6-450A-4D92-9EFC-D6B6CBC1F4DA")
    IMFMediaSourceExtensionLiveSeekableRange : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetLiveSeekableRange( 
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double end) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearLiveSeekableRange( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourceExtensionLiveSeekableRangeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaSourceExtensionLiveSeekableRange * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaSourceExtensionLiveSeekableRange * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaSourceExtensionLiveSeekableRange * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtensionLiveSeekableRange, SetLiveSeekableRange)
        HRESULT ( STDMETHODCALLTYPE *SetLiveSeekableRange )( 
            IMFMediaSourceExtensionLiveSeekableRange * This,
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double end);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceExtensionLiveSeekableRange, ClearLiveSeekableRange)
        HRESULT ( STDMETHODCALLTYPE *ClearLiveSeekableRange )( 
            IMFMediaSourceExtensionLiveSeekableRange * This);
        
        END_INTERFACE
    } IMFMediaSourceExtensionLiveSeekableRangeVtbl;

    interface IMFMediaSourceExtensionLiveSeekableRange
    {
        CONST_VTBL struct IMFMediaSourceExtensionLiveSeekableRangeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSourceExtensionLiveSeekableRange_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSourceExtensionLiveSeekableRange_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSourceExtensionLiveSeekableRange_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSourceExtensionLiveSeekableRange_SetLiveSeekableRange(This,start,end)	\
    ( (This)->lpVtbl -> SetLiveSeekableRange(This,start,end) ) 

#define IMFMediaSourceExtensionLiveSeekableRange_ClearLiveSeekableRange(This)	\
    ( (This)->lpVtbl -> ClearLiveSeekableRange(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSourceExtensionLiveSeekableRange_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0019 */
/* [local] */ 








extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0019_v0_0_s_ifspec;

#ifndef __IMFMediaEngineEME_INTERFACE_DEFINED__
#define __IMFMediaEngineEME_INTERFACE_DEFINED__

/* interface IMFMediaEngineEME */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineEME;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("50dc93e4-ba4f-4275-ae66-83e836e57469")
    IMFMediaEngineEME : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_Keys( 
            /* [annotation] */ 
            _COM_Outptr_result_maybenull_  IMFMediaKeys **keys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaKeys( 
            /* [annotation] */ 
            _In_opt_  IMFMediaKeys *keys) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineEMEVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineEME * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineEME * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineEME * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEME, get_Keys)
        HRESULT ( STDMETHODCALLTYPE *get_Keys )( 
            IMFMediaEngineEME * This,
            /* [annotation] */ 
            _COM_Outptr_result_maybenull_  IMFMediaKeys **keys);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEME, SetMediaKeys)
        HRESULT ( STDMETHODCALLTYPE *SetMediaKeys )( 
            IMFMediaEngineEME * This,
            /* [annotation] */ 
            _In_opt_  IMFMediaKeys *keys);
        
        END_INTERFACE
    } IMFMediaEngineEMEVtbl;

    interface IMFMediaEngineEME
    {
        CONST_VTBL struct IMFMediaEngineEMEVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineEME_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineEME_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineEME_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineEME_get_Keys(This,keys)	\
    ( (This)->lpVtbl -> get_Keys(This,keys) ) 

#define IMFMediaEngineEME_SetMediaKeys(This,keys)	\
    ( (This)->lpVtbl -> SetMediaKeys(This,keys) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineEME_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineSrcElementsEx_INTERFACE_DEFINED__
#define __IMFMediaEngineSrcElementsEx_INTERFACE_DEFINED__

/* interface IMFMediaEngineSrcElementsEx */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineSrcElementsEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("654a6bb3-e1a3-424a-9908-53a43a0dfda0")
    IMFMediaEngineSrcElementsEx : public IMFMediaEngineSrcElements
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddElementEx( 
            /* [annotation] */ 
            _In_opt_  BSTR pURL,
            /* [annotation] */ 
            _In_opt_  BSTR pType,
            /* [annotation] */ 
            _In_opt_  BSTR pMedia,
            /* [annotation] */ 
            _In_opt_  BSTR keySystem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKeySystem( 
            /* [annotation] */ 
            _In_  DWORD index,
            /* [annotation] */ 
            _Outptr_result_maybenull_  BSTR *pType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineSrcElementsExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineSrcElementsEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineSrcElementsEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetLength)
        DWORD ( STDMETHODCALLTYPE *GetLength )( 
            IMFMediaEngineSrcElementsEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetURL)
        HRESULT ( STDMETHODCALLTYPE *GetURL )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pURL);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetType)
        HRESULT ( STDMETHODCALLTYPE *GetType )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pType);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, GetMedia)
        HRESULT ( STDMETHODCALLTYPE *GetMedia )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  BSTR *pMedia);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, AddElement)
        HRESULT ( STDMETHODCALLTYPE *AddElement )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [annotation][in] */ 
            _In_opt_  BSTR pURL,
            /* [annotation][in] */ 
            _In_opt_  BSTR pType,
            /* [annotation][in] */ 
            _In_opt_  BSTR pMedia);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElements, RemoveAllElements)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllElements )( 
            IMFMediaEngineSrcElementsEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElementsEx, AddElementEx)
        HRESULT ( STDMETHODCALLTYPE *AddElementEx )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [annotation] */ 
            _In_opt_  BSTR pURL,
            /* [annotation] */ 
            _In_opt_  BSTR pType,
            /* [annotation] */ 
            _In_opt_  BSTR pMedia,
            /* [annotation] */ 
            _In_opt_  BSTR keySystem);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSrcElementsEx, GetKeySystem)
        HRESULT ( STDMETHODCALLTYPE *GetKeySystem )( 
            IMFMediaEngineSrcElementsEx * This,
            /* [annotation] */ 
            _In_  DWORD index,
            /* [annotation] */ 
            _Outptr_result_maybenull_  BSTR *pType);
        
        END_INTERFACE
    } IMFMediaEngineSrcElementsExVtbl;

    interface IMFMediaEngineSrcElementsEx
    {
        CONST_VTBL struct IMFMediaEngineSrcElementsExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineSrcElementsEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineSrcElementsEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineSrcElementsEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineSrcElementsEx_GetLength(This)	\
    ( (This)->lpVtbl -> GetLength(This) ) 

#define IMFMediaEngineSrcElementsEx_GetURL(This,index,pURL)	\
    ( (This)->lpVtbl -> GetURL(This,index,pURL) ) 

#define IMFMediaEngineSrcElementsEx_GetType(This,index,pType)	\
    ( (This)->lpVtbl -> GetType(This,index,pType) ) 

#define IMFMediaEngineSrcElementsEx_GetMedia(This,index,pMedia)	\
    ( (This)->lpVtbl -> GetMedia(This,index,pMedia) ) 

#define IMFMediaEngineSrcElementsEx_AddElement(This,pURL,pType,pMedia)	\
    ( (This)->lpVtbl -> AddElement(This,pURL,pType,pMedia) ) 

#define IMFMediaEngineSrcElementsEx_RemoveAllElements(This)	\
    ( (This)->lpVtbl -> RemoveAllElements(This) ) 


#define IMFMediaEngineSrcElementsEx_AddElementEx(This,pURL,pType,pMedia,keySystem)	\
    ( (This)->lpVtbl -> AddElementEx(This,pURL,pType,pMedia,keySystem) ) 

#define IMFMediaEngineSrcElementsEx_GetKeySystem(This,index,pType)	\
    ( (This)->lpVtbl -> GetKeySystem(This,index,pType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineSrcElementsEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0021 */
/* [local] */ 

EXTERN_GUID(MF_MEDIA_ENGINE_NEEDKEY_CALLBACK,
0x7ea80843, 0xb6e4, 0x432c, 0x8e, 0xa4, 0x78, 0x48, 0xff, 0xe4, 0x22, 0x0e);


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0021_v0_0_s_ifspec;

#ifndef __IMFMediaEngineNeedKeyNotify_INTERFACE_DEFINED__
#define __IMFMediaEngineNeedKeyNotify_INTERFACE_DEFINED__

/* interface IMFMediaEngineNeedKeyNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineNeedKeyNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("46a30204-a696-4b18-8804-246b8f031bb1")
    IMFMediaEngineNeedKeyNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE NeedKey( 
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *initData,
            /* [annotation] */ 
            _In_  DWORD cb) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineNeedKeyNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineNeedKeyNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineNeedKeyNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineNeedKeyNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineNeedKeyNotify, NeedKey)
        void ( STDMETHODCALLTYPE *NeedKey )( 
            IMFMediaEngineNeedKeyNotify * This,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *initData,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        END_INTERFACE
    } IMFMediaEngineNeedKeyNotifyVtbl;

    interface IMFMediaEngineNeedKeyNotify
    {
        CONST_VTBL struct IMFMediaEngineNeedKeyNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineNeedKeyNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineNeedKeyNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineNeedKeyNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineNeedKeyNotify_NeedKey(This,initData,cb)	\
    ( (This)->lpVtbl -> NeedKey(This,initData,cb) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineNeedKeyNotify_INTERFACE_DEFINED__ */


#ifndef __IMFMediaKeys_INTERFACE_DEFINED__
#define __IMFMediaKeys_INTERFACE_DEFINED__

/* interface IMFMediaKeys */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeys;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5cb31c05-61ff-418f-afda-caaf41421a38")
    IMFMediaKeys : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSession( 
            /* [annotation] */ 
            _In_opt_  BSTR mimeType,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *initData,
            /* [annotation] */ 
            _In_opt_  DWORD cb,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cbCustomData)  const BYTE *customData,
            /* [annotation] */ 
            _In_  DWORD cbCustomData,
            /* [annotation] */ 
            _In_  IMFMediaKeySessionNotify *notify,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySession **ppSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_KeySystem( 
            /* [annotation] */ 
            _COM_Outptr_  BSTR *keySystem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSuspendNotify( 
            /* [annotation] */ 
            _COM_Outptr_  IMFCdmSuspendNotify **notify) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeysVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeys * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeys * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeys * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, CreateSession)
        HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            IMFMediaKeys * This,
            /* [annotation] */ 
            _In_opt_  BSTR mimeType,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *initData,
            /* [annotation] */ 
            _In_opt_  DWORD cb,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cbCustomData)  const BYTE *customData,
            /* [annotation] */ 
            _In_  DWORD cbCustomData,
            /* [annotation] */ 
            _In_  IMFMediaKeySessionNotify *notify,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySession **ppSession);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, get_KeySystem)
        HRESULT ( STDMETHODCALLTYPE *get_KeySystem )( 
            IMFMediaKeys * This,
            /* [annotation] */ 
            _COM_Outptr_  BSTR *keySystem);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaKeys * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, GetSuspendNotify)
        HRESULT ( STDMETHODCALLTYPE *GetSuspendNotify )( 
            IMFMediaKeys * This,
            /* [annotation] */ 
            _COM_Outptr_  IMFCdmSuspendNotify **notify);
        
        END_INTERFACE
    } IMFMediaKeysVtbl;

    interface IMFMediaKeys
    {
        CONST_VTBL struct IMFMediaKeysVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeys_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeys_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeys_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeys_CreateSession(This,mimeType,initData,cb,customData,cbCustomData,notify,ppSession)	\
    ( (This)->lpVtbl -> CreateSession(This,mimeType,initData,cb,customData,cbCustomData,notify,ppSession) ) 

#define IMFMediaKeys_get_KeySystem(This,keySystem)	\
    ( (This)->lpVtbl -> get_KeySystem(This,keySystem) ) 

#define IMFMediaKeys_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFMediaKeys_GetSuspendNotify(This,notify)	\
    ( (This)->lpVtbl -> GetSuspendNotify(This,notify) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeys_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0023 */
/* [local] */ 

typedef 
enum _MF_MEDIA_ENGINE_KEYERR
    {
        MF_MEDIAENGINE_KEYERR_UNKNOWN	= 1,
        MF_MEDIAENGINE_KEYERR_CLIENT	= 2,
        MF_MEDIAENGINE_KEYERR_SERVICE	= 3,
        MF_MEDIAENGINE_KEYERR_OUTPUT	= 4,
        MF_MEDIAENGINE_KEYERR_HARDWARECHANGE	= 5,
        MF_MEDIAENGINE_KEYERR_DOMAIN	= 6
    } 	MF_MEDIA_ENGINE_KEYERR;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0023_v0_0_s_ifspec;

#ifndef __IMFMediaKeySession_INTERFACE_DEFINED__
#define __IMFMediaKeySession_INTERFACE_DEFINED__

/* interface IMFMediaKeySession */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeySession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24fa67d5-d1d0-4dc5-995c-c0efdc191fb5")
    IMFMediaKeySession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetError( 
            /* [annotation] */ 
            _Out_  USHORT *code,
            /* [annotation] */ 
            _Out_  DWORD *systemCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_KeySystem( 
            /* [annotation] */ 
            _COM_Outptr_  BSTR *keySystem) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SessionId( 
            /* [annotation] */ 
            _COM_Outptr_  BSTR *sessionId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Update( 
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *key,
            /* [annotation] */ 
            _In_  DWORD cb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeySessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeySession * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeySession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeySession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            IMFMediaKeySession * This,
            /* [annotation] */ 
            _Out_  USHORT *code,
            /* [annotation] */ 
            _Out_  DWORD *systemCode);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, get_KeySystem)
        HRESULT ( STDMETHODCALLTYPE *get_KeySystem )( 
            IMFMediaKeySession * This,
            /* [annotation] */ 
            _COM_Outptr_  BSTR *keySystem);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, get_SessionId)
        HRESULT ( STDMETHODCALLTYPE *get_SessionId )( 
            IMFMediaKeySession * This,
            /* [annotation] */ 
            _COM_Outptr_  BSTR *sessionId);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IMFMediaKeySession * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *key,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IMFMediaKeySession * This);
        
        END_INTERFACE
    } IMFMediaKeySessionVtbl;

    interface IMFMediaKeySession
    {
        CONST_VTBL struct IMFMediaKeySessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeySession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeySession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeySession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeySession_GetError(This,code,systemCode)	\
    ( (This)->lpVtbl -> GetError(This,code,systemCode) ) 

#define IMFMediaKeySession_get_KeySystem(This,keySystem)	\
    ( (This)->lpVtbl -> get_KeySystem(This,keySystem) ) 

#define IMFMediaKeySession_get_SessionId(This,sessionId)	\
    ( (This)->lpVtbl -> get_SessionId(This,sessionId) ) 

#define IMFMediaKeySession_Update(This,key,cb)	\
    ( (This)->lpVtbl -> Update(This,key,cb) ) 

#define IMFMediaKeySession_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeySession_INTERFACE_DEFINED__ */


#ifndef __IMFMediaKeySessionNotify_INTERFACE_DEFINED__
#define __IMFMediaKeySessionNotify_INTERFACE_DEFINED__

/* interface IMFMediaKeySessionNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeySessionNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6a0083f9-8947-4c1d-9ce0-cdee22b23135")
    IMFMediaKeySessionNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE KeyMessage( 
            /* [annotation] */ 
            _In_opt_  BSTR destinationURL,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *message,
            /* [annotation] */ 
            _In_  DWORD cb) = 0;
        
        virtual void STDMETHODCALLTYPE KeyAdded( void) = 0;
        
        virtual void STDMETHODCALLTYPE KeyError( 
            /* [annotation] */ 
            _In_  USHORT code,
            /* [annotation] */ 
            _In_  DWORD systemCode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeySessionNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeySessionNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeySessionNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeySessionNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify, KeyMessage)
        void ( STDMETHODCALLTYPE *KeyMessage )( 
            IMFMediaKeySessionNotify * This,
            /* [annotation] */ 
            _In_opt_  BSTR destinationURL,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *message,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify, KeyAdded)
        void ( STDMETHODCALLTYPE *KeyAdded )( 
            IMFMediaKeySessionNotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify, KeyError)
        void ( STDMETHODCALLTYPE *KeyError )( 
            IMFMediaKeySessionNotify * This,
            /* [annotation] */ 
            _In_  USHORT code,
            /* [annotation] */ 
            _In_  DWORD systemCode);
        
        END_INTERFACE
    } IMFMediaKeySessionNotifyVtbl;

    interface IMFMediaKeySessionNotify
    {
        CONST_VTBL struct IMFMediaKeySessionNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeySessionNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeySessionNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeySessionNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeySessionNotify_KeyMessage(This,destinationURL,message,cb)	\
    ( (This)->lpVtbl -> KeyMessage(This,destinationURL,message,cb) ) 

#define IMFMediaKeySessionNotify_KeyAdded(This)	\
    ( (This)->lpVtbl -> KeyAdded(This) ) 

#define IMFMediaKeySessionNotify_KeyError(This,code,systemCode)	\
    ( (This)->lpVtbl -> KeyError(This,code,systemCode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeySessionNotify_INTERFACE_DEFINED__ */


#ifndef __IMFCdmSuspendNotify_INTERFACE_DEFINED__
#define __IMFCdmSuspendNotify_INTERFACE_DEFINED__

/* interface IMFCdmSuspendNotify */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFCdmSuspendNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a5645d2-43bd-47fd-87b7-dcd24cc7d692")
    IMFCdmSuspendNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Begin( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE End( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCdmSuspendNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFCdmSuspendNotify * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFCdmSuspendNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFCdmSuspendNotify * This);
        
        DECLSPEC_XFGVIRT(IMFCdmSuspendNotify, Begin)
        HRESULT ( STDMETHODCALLTYPE *Begin )( 
            __RPC__in IMFCdmSuspendNotify * This);
        
        DECLSPEC_XFGVIRT(IMFCdmSuspendNotify, End)
        HRESULT ( STDMETHODCALLTYPE *End )( 
            __RPC__in IMFCdmSuspendNotify * This);
        
        END_INTERFACE
    } IMFCdmSuspendNotifyVtbl;

    interface IMFCdmSuspendNotify
    {
        CONST_VTBL struct IMFCdmSuspendNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCdmSuspendNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCdmSuspendNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCdmSuspendNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCdmSuspendNotify_Begin(This)	\
    ( (This)->lpVtbl -> Begin(This) ) 

#define IMFCdmSuspendNotify_End(This)	\
    ( (This)->lpVtbl -> End(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCdmSuspendNotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0026 */
/* [local] */ 

typedef 
enum _MF_HDCP_STATUS
    {
        MF_HDCP_STATUS_ON	= 0,
        MF_HDCP_STATUS_OFF	= 1,
        MF_HDCP_STATUS_ON_WITH_TYPE_ENFORCEMENT	= 2
    } 	MF_HDCP_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0026_v0_0_s_ifspec;

#ifndef __IMFHDCPStatus_INTERFACE_DEFINED__
#define __IMFHDCPStatus_INTERFACE_DEFINED__

/* interface IMFHDCPStatus */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFHDCPStatus;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DE400F54-5BF1-40CF-8964-0BEA136B1E3D")
    IMFHDCPStatus : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Query( 
            /* [annotation][in] */ 
            _Inout_  MF_HDCP_STATUS *pStatus,
            /* [annotation][in] */ 
            _Inout_  BOOL *pfStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Set( 
            /* [annotation][in] */ 
            _In_  MF_HDCP_STATUS status) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFHDCPStatusVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFHDCPStatus * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFHDCPStatus * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFHDCPStatus * This);
        
        DECLSPEC_XFGVIRT(IMFHDCPStatus, Query)
        HRESULT ( STDMETHODCALLTYPE *Query )( 
            IMFHDCPStatus * This,
            /* [annotation][in] */ 
            _Inout_  MF_HDCP_STATUS *pStatus,
            /* [annotation][in] */ 
            _Inout_  BOOL *pfStatus);
        
        DECLSPEC_XFGVIRT(IMFHDCPStatus, Set)
        HRESULT ( STDMETHODCALLTYPE *Set )( 
            IMFHDCPStatus * This,
            /* [annotation][in] */ 
            _In_  MF_HDCP_STATUS status);
        
        END_INTERFACE
    } IMFHDCPStatusVtbl;

    interface IMFHDCPStatus
    {
        CONST_VTBL struct IMFHDCPStatusVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFHDCPStatus_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFHDCPStatus_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFHDCPStatus_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFHDCPStatus_Query(This,pStatus,pfStatus)	\
    ( (This)->lpVtbl -> Query(This,pStatus,pfStatus) ) 

#define IMFHDCPStatus_Set(This,status)	\
    ( (This)->lpVtbl -> Set(This,status) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFHDCPStatus_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0027 */
/* [local] */ 

typedef 
enum MF_MEDIA_ENGINE_OPM_STATUS
    {
        MF_MEDIA_ENGINE_OPM_NOT_REQUESTED	= 0,
        MF_MEDIA_ENGINE_OPM_ESTABLISHED	= 1,
        MF_MEDIA_ENGINE_OPM_FAILED_VM	= 2,
        MF_MEDIA_ENGINE_OPM_FAILED_BDA	= 3,
        MF_MEDIA_ENGINE_OPM_FAILED_UNSIGNED_DRIVER	= 4,
        MF_MEDIA_ENGINE_OPM_FAILED	= 5
    } 	MF_MEDIA_ENGINE_OPM_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0027_v0_0_s_ifspec;

#ifndef __IMFMediaEngineOPMInfo_INTERFACE_DEFINED__
#define __IMFMediaEngineOPMInfo_INTERFACE_DEFINED__

/* interface IMFMediaEngineOPMInfo */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineOPMInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("765763e6-6c01-4b01-bb0f-b829f60ed28c")
    IMFMediaEngineOPMInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOPMInfo( 
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_OPM_STATUS *pStatus,
            /* [annotation][out] */ 
            _Out_  BOOL *pConstricted) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineOPMInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineOPMInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineOPMInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineOPMInfo * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineOPMInfo, GetOPMInfo)
        HRESULT ( STDMETHODCALLTYPE *GetOPMInfo )( 
            IMFMediaEngineOPMInfo * This,
            /* [annotation][out] */ 
            _Out_  MF_MEDIA_ENGINE_OPM_STATUS *pStatus,
            /* [annotation][out] */ 
            _Out_  BOOL *pConstricted);
        
        END_INTERFACE
    } IMFMediaEngineOPMInfoVtbl;

    interface IMFMediaEngineOPMInfo
    {
        CONST_VTBL struct IMFMediaEngineOPMInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineOPMInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineOPMInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineOPMInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineOPMInfo_GetOPMInfo(This,pStatus,pConstricted)	\
    ( (This)->lpVtbl -> GetOPMInfo(This,pStatus,pConstricted) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineOPMInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0028 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#ifndef NO_MEDIA_ENGINE_FACTORY
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID(MF_MEDIA_ENGINE_CALLBACK,
0xc60381b8,0x83a4,0x41f8,0xa3,0xd0,0xde,0x05,0x07,0x68,0x49,0xa9);
EXTERN_GUID(MF_MEDIA_ENGINE_DXGI_MANAGER,
0x065702da,0x1094,0x486d,0x86,0x17,0xee,0x7c,0xc4,0xee,0x46,0x48);
EXTERN_GUID(MF_MEDIA_ENGINE_EXTENSION,
0x3109fd46,0x060d,0x4b62,0x8d,0xcf,0xfa,0xff,0x81,0x13,0x18,0xd2);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID(MF_MEDIA_ENGINE_PLAYBACK_HWND,
0xd988879b,0x67c9,0x4d92,0xba,0xa7,0x6e,0xad,0xd4,0x46,0x03,0x9d);
EXTERN_GUID(MF_MEDIA_ENGINE_OPM_HWND,
0xa0be8ee7, 0x0572, 0x4f2c, 0xa8, 0x01, 0x2a, 0x15, 0x1b, 0xd3, 0xe7, 0x26);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID(MF_MEDIA_ENGINE_PLAYBACK_VISUAL,
0x6debd26f,0x6ab9,0x4d7e,0xb0,0xee,0xc6,0x1a,0x73,0xff,0xad,0x15);
EXTERN_GUID(MF_MEDIA_ENGINE_COREWINDOW,
0xfccae4dc, 0x0b7f, 0x41c2, 0x9f, 0x96, 0x46, 0x59, 0x94, 0x8a, 0xcd, 0xdc);
EXTERN_GUID(MF_MEDIA_ENGINE_VIDEO_OUTPUT_FORMAT,
0x5066893c,0x8cf9,0x42bc,0x8b,0x8a,0x47,0x22,0x12,0xe5,0x27,0x26);
EXTERN_GUID(MF_MEDIA_ENGINE_CONTENT_PROTECTION_FLAGS,
0xe0350223, 0x5aaf, 0x4d76, 0xa7, 0xc3, 0x06, 0xde, 0x70, 0x89, 0x4d, 0xb4);
EXTERN_GUID(MF_MEDIA_ENGINE_CONTENT_PROTECTION_MANAGER,
0xfdd6dfaa, 0xbd85, 0x4af3, 0x9e, 0x0f, 0xa0, 0x1d, 0x53, 0x9d, 0x87, 0x6a);
EXTERN_GUID(MF_MEDIA_ENGINE_AUDIO_ENDPOINT_ROLE,
0xd2cb93d1, 0x116a, 0x44f2, 0x93, 0x85, 0xf7, 0xd0, 0xfd, 0xa2, 0xfb, 0x46);
EXTERN_GUID(MF_MEDIA_ENGINE_AUDIO_CATEGORY,
0xc8d4c51d, 0x350e, 0x41f2, 0xba, 0x46, 0xfa, 0xeb, 0xbb, 0x08, 0x57, 0xf6);
EXTERN_GUID(MF_MEDIA_ENGINE_STREAM_CONTAINS_ALPHA_CHANNEL,
0x5cbfaf44, 0xd2b2, 0x4cfb, 0x80, 0xa7, 0xd4, 0x29, 0xc7, 0x4c, 0x78, 0x9d);
EXTERN_GUID(MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE,
0x4e0212e2, 0xe18f, 0x41e1, 0x95, 0xe5, 0xc0, 0xe7, 0xe9, 0x23, 0x5b, 0xc3);
EXTERN_GUID(MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE9,
0x052c2d39, 0x40c0, 0x4188, 0xab, 0x86, 0xf8, 0x28, 0x27, 0x3b, 0x75, 0x22);
EXTERN_GUID(MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE10,
0x11a47afd, 0x6589, 0x4124, 0xb3, 0x12, 0x61, 0x58, 0xec, 0x51, 0x7f, 0xc3);
EXTERN_GUID(MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE11,
0x1cf1315f, 0xce3f, 0x4035, 0x93, 0x91, 0x16, 0x14, 0x2f, 0x77, 0x51, 0x89);
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD)
EXTERN_GUID(MF_MEDIA_ENGINE_BROWSER_COMPATIBILITY_MODE_IE_EDGE,
0xa6f3e465, 0x3aca, 0x442c, 0xa3, 0xf0, 0xad, 0x6d, 0xda, 0xd8, 0x39, 0xae);
EXTERN_GUID(MF_MEDIA_ENGINE_COMPATIBILITY_MODE,
0x3ef26ad4, 0xdc54, 0x45de, 0xb9, 0xaf, 0x76, 0xc8, 0xc6, 0x6b, 0xfa, 0x8e);
EXTERN_GUID(MF_MEDIA_ENGINE_COMPATIBILITY_MODE_WWA_EDGE,
0x15b29098, 0x9f01, 0x4e4d, 0xb6, 0x5a, 0xc0, 0x6c, 0x6c, 0x89, 0xda, 0x2a);
EXTERN_GUID(MF_MEDIA_ENGINE_COMPATIBILITY_MODE_WIN10,
0x5b25e089, 0x6ca7, 0x4139, 0xa2, 0xcb, 0xfc, 0xaa, 0xb3, 0x95, 0x52, 0xa3);
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_GUID(MF_MEDIA_ENGINE_SOURCE_RESOLVER_CONFIG_STORE,
0x0ac0c497, 0xb3c4, 0x48c9, 0x9c, 0xde, 0xbb, 0x8c, 0xa2, 0x44, 0x2c, 0xa3);
EXTERN_GUID(MF_MEDIA_ENGINE_TRACK_ID,
0x65bea312, 0x4043, 0x4815, 0x8e, 0xab, 0x44, 0xdc, 0xe2, 0xef, 0x8f, 0x2a);
EXTERN_GUID(MF_MEDIA_ENGINE_TELEMETRY_APPLICATION_ID,
0x1e7b273b, 0xa7e4, 0x402a, 0x8f, 0x51, 0xc4, 0x8e, 0x88, 0xa2, 0xca, 0xbc);
EXTERN_GUID(MF_MEDIA_ENGINE_SYNCHRONOUS_CLOSE,
0xc3c2e12f, 0x7e0e, 0x4e43, 0xb9, 0x1c, 0xdc, 0x99, 0x2c, 0xcd, 0xfa, 0x5e);
EXTERN_GUID(MF_MEDIA_ENGINE_MEDIA_PLAYER_MODE,
0x3ddd8d45, 0x5aa1, 0x4112, 0x82, 0xe5, 0x36, 0xf6, 0xa2, 0x19, 0x7e, 0x6e);
typedef 
enum MF_MEDIA_ENGINE_CREATEFLAGS
    {
        MF_MEDIA_ENGINE_AUDIOONLY	= 0x1,
        MF_MEDIA_ENGINE_WAITFORSTABLE_STATE	= 0x2,
        MF_MEDIA_ENGINE_FORCEMUTE	= 0x4,
        MF_MEDIA_ENGINE_REAL_TIME_MODE	= 0x8,
        MF_MEDIA_ENGINE_DISABLE_LOCAL_PLUGINS	= 0x10,
        MF_MEDIA_ENGINE_CREATEFLAGS_MASK	= 0x1f
    } 	MF_MEDIA_ENGINE_CREATEFLAGS;

typedef 
enum MF_MEDIA_ENGINE_PROTECTION_FLAGS
    {
        MF_MEDIA_ENGINE_ENABLE_PROTECTED_CONTENT	= 1,
        MF_MEDIA_ENGINE_USE_PMP_FOR_ALL_CONTENT	= 2,
        MF_MEDIA_ENGINE_USE_UNPROTECTED_PMP	= 4
    } 	MF_MEDIA_ENGINE_PROTECTION_FLAGS;



extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0028_v0_0_s_ifspec;

#ifndef __IMFMediaEngineClassFactory_INTERFACE_DEFINED__
#define __IMFMediaEngineClassFactory_INTERFACE_DEFINED__

/* interface IMFMediaEngineClassFactory */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineClassFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4D645ACE-26AA-4688-9BE1-DF3516990B93")
    IMFMediaEngineClassFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateInstance( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaEngine **ppPlayer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTimeRange( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppTimeRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateError( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineClassFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineClassFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineClassFactory * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IMFMediaEngineClassFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaEngine **ppPlayer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory, CreateTimeRange)
        HRESULT ( STDMETHODCALLTYPE *CreateTimeRange )( 
            IMFMediaEngineClassFactory * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppTimeRange);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory, CreateError)
        HRESULT ( STDMETHODCALLTYPE *CreateError )( 
            IMFMediaEngineClassFactory * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError);
        
        END_INTERFACE
    } IMFMediaEngineClassFactoryVtbl;

    interface IMFMediaEngineClassFactory
    {
        CONST_VTBL struct IMFMediaEngineClassFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineClassFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineClassFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineClassFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineClassFactory_CreateInstance(This,dwFlags,pAttr,ppPlayer)	\
    ( (This)->lpVtbl -> CreateInstance(This,dwFlags,pAttr,ppPlayer) ) 

#define IMFMediaEngineClassFactory_CreateTimeRange(This,ppTimeRange)	\
    ( (This)->lpVtbl -> CreateTimeRange(This,ppTimeRange) ) 

#define IMFMediaEngineClassFactory_CreateError(This,ppError)	\
    ( (This)->lpVtbl -> CreateError(This,ppError) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineClassFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0029 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0029_v0_0_s_ifspec;

#ifndef __IMFMediaEngineClassFactoryEx_INTERFACE_DEFINED__
#define __IMFMediaEngineClassFactoryEx_INTERFACE_DEFINED__

/* interface IMFMediaEngineClassFactoryEx */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineClassFactoryEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c56156c6-ea5b-48a5-9df8-fbe035d0929e")
    IMFMediaEngineClassFactoryEx : public IMFMediaEngineClassFactory
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateMediaSourceExtension( 
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSourceExtension **ppMSE) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateMediaKeys( 
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _In_opt_  BSTR cdmStorePath,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeys **ppKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsTypeSupported( 
            /* [annotation] */ 
            _In_opt_  BSTR type,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _Out_  BOOL *isSupported) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineClassFactoryExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineClassFactoryEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineClassFactoryEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory, CreateInstance)
        HRESULT ( STDMETHODCALLTYPE *CreateInstance )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaEngine **ppPlayer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory, CreateTimeRange)
        HRESULT ( STDMETHODCALLTYPE *CreateTimeRange )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaTimeRange **ppTimeRange);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory, CreateError)
        HRESULT ( STDMETHODCALLTYPE *CreateError )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaError **ppError);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactoryEx, CreateMediaSourceExtension)
        HRESULT ( STDMETHODCALLTYPE *CreateMediaSourceExtension )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [annotation][in] */ 
            _In_  DWORD dwFlags,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttr,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSourceExtension **ppMSE);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactoryEx, CreateMediaKeys)
        HRESULT ( STDMETHODCALLTYPE *CreateMediaKeys )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _In_opt_  BSTR cdmStorePath,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeys **ppKeys);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactoryEx, IsTypeSupported)
        HRESULT ( STDMETHODCALLTYPE *IsTypeSupported )( 
            IMFMediaEngineClassFactoryEx * This,
            /* [annotation] */ 
            _In_opt_  BSTR type,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _Out_  BOOL *isSupported);
        
        END_INTERFACE
    } IMFMediaEngineClassFactoryExVtbl;

    interface IMFMediaEngineClassFactoryEx
    {
        CONST_VTBL struct IMFMediaEngineClassFactoryExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineClassFactoryEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineClassFactoryEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineClassFactoryEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineClassFactoryEx_CreateInstance(This,dwFlags,pAttr,ppPlayer)	\
    ( (This)->lpVtbl -> CreateInstance(This,dwFlags,pAttr,ppPlayer) ) 

#define IMFMediaEngineClassFactoryEx_CreateTimeRange(This,ppTimeRange)	\
    ( (This)->lpVtbl -> CreateTimeRange(This,ppTimeRange) ) 

#define IMFMediaEngineClassFactoryEx_CreateError(This,ppError)	\
    ( (This)->lpVtbl -> CreateError(This,ppError) ) 


#define IMFMediaEngineClassFactoryEx_CreateMediaSourceExtension(This,dwFlags,pAttr,ppMSE)	\
    ( (This)->lpVtbl -> CreateMediaSourceExtension(This,dwFlags,pAttr,ppMSE) ) 

#define IMFMediaEngineClassFactoryEx_CreateMediaKeys(This,keySystem,cdmStorePath,ppKeys)	\
    ( (This)->lpVtbl -> CreateMediaKeys(This,keySystem,cdmStorePath,ppKeys) ) 

#define IMFMediaEngineClassFactoryEx_IsTypeSupported(This,type,keySystem,isSupported)	\
    ( (This)->lpVtbl -> IsTypeSupported(This,type,keySystem,isSupported) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineClassFactoryEx_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineClassFactory2_INTERFACE_DEFINED__
#define __IMFMediaEngineClassFactory2_INTERFACE_DEFINED__

/* interface IMFMediaEngineClassFactory2 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineClassFactory2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09083cef-867f-4bf6-8776-dee3a7b42fca")
    IMFMediaEngineClassFactory2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateMediaKeys2( 
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _In_  BSTR defaultCdmStorePath,
            /* [annotation] */ 
            _In_opt_  BSTR inprivateCdmStorePath,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeys **ppKeys) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineClassFactory2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineClassFactory2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineClassFactory2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineClassFactory2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory2, CreateMediaKeys2)
        HRESULT ( STDMETHODCALLTYPE *CreateMediaKeys2 )( 
            IMFMediaEngineClassFactory2 * This,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _In_  BSTR defaultCdmStorePath,
            /* [annotation] */ 
            _In_opt_  BSTR inprivateCdmStorePath,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeys **ppKeys);
        
        END_INTERFACE
    } IMFMediaEngineClassFactory2Vtbl;

    interface IMFMediaEngineClassFactory2
    {
        CONST_VTBL struct IMFMediaEngineClassFactory2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineClassFactory2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineClassFactory2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineClassFactory2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineClassFactory2_CreateMediaKeys2(This,keySystem,defaultCdmStorePath,inprivateCdmStorePath,ppKeys)	\
    ( (This)->lpVtbl -> CreateMediaKeys2(This,keySystem,defaultCdmStorePath,inprivateCdmStorePath,ppKeys) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineClassFactory2_INTERFACE_DEFINED__ */


#ifndef __IMFExtendedDRMTypeSupport_INTERFACE_DEFINED__
#define __IMFExtendedDRMTypeSupport_INTERFACE_DEFINED__

/* interface IMFExtendedDRMTypeSupport */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedDRMTypeSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("332EC562-3758-468D-A784-E38F23552128")
    IMFExtendedDRMTypeSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsTypeSupportedEx( 
            /* [annotation] */ 
            _In_opt_  BSTR type,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedDRMTypeSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedDRMTypeSupport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedDRMTypeSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedDRMTypeSupport * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedDRMTypeSupport, IsTypeSupportedEx)
        HRESULT ( STDMETHODCALLTYPE *IsTypeSupportedEx )( 
            IMFExtendedDRMTypeSupport * This,
            /* [annotation] */ 
            _In_opt_  BSTR type,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _Out_  MF_MEDIA_ENGINE_CANPLAY *pAnswer);
        
        END_INTERFACE
    } IMFExtendedDRMTypeSupportVtbl;

    interface IMFExtendedDRMTypeSupport
    {
        CONST_VTBL struct IMFExtendedDRMTypeSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedDRMTypeSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedDRMTypeSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedDRMTypeSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedDRMTypeSupport_IsTypeSupportedEx(This,type,keySystem,pAnswer)	\
    ( (This)->lpVtbl -> IsTypeSupportedEx(This,type,keySystem,pAnswer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedDRMTypeSupport_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineSupportsSourceTransfer_INTERFACE_DEFINED__
#define __IMFMediaEngineSupportsSourceTransfer_INTERFACE_DEFINED__

/* interface IMFMediaEngineSupportsSourceTransfer */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineSupportsSourceTransfer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a724b056-1b2e-4642-a6f3-db9420c52908")
    IMFMediaEngineSupportsSourceTransfer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShouldTransferSource( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfShouldTransfer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DetachMediaSource( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFByteStream **ppByteStream,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSource **ppMediaSource,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSourceExtension **ppMSE) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AttachMediaSource( 
            /* [annotation][in] */ 
            _In_opt_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  IMFMediaSource *pMediaSource,
            /* [annotation][in] */ 
            _In_opt_  IMFMediaSourceExtension *pMSE) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineSupportsSourceTransferVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineSupportsSourceTransfer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineSupportsSourceTransfer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineSupportsSourceTransfer * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSupportsSourceTransfer, ShouldTransferSource)
        HRESULT ( STDMETHODCALLTYPE *ShouldTransferSource )( 
            IMFMediaEngineSupportsSourceTransfer * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfShouldTransfer);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSupportsSourceTransfer, DetachMediaSource)
        HRESULT ( STDMETHODCALLTYPE *DetachMediaSource )( 
            IMFMediaEngineSupportsSourceTransfer * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFByteStream **ppByteStream,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSource **ppMediaSource,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSourceExtension **ppMSE);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineSupportsSourceTransfer, AttachMediaSource)
        HRESULT ( STDMETHODCALLTYPE *AttachMediaSource )( 
            IMFMediaEngineSupportsSourceTransfer * This,
            /* [annotation][in] */ 
            _In_opt_  IMFByteStream *pByteStream,
            /* [annotation][in] */ 
            _In_  IMFMediaSource *pMediaSource,
            /* [annotation][in] */ 
            _In_opt_  IMFMediaSourceExtension *pMSE);
        
        END_INTERFACE
    } IMFMediaEngineSupportsSourceTransferVtbl;

    interface IMFMediaEngineSupportsSourceTransfer
    {
        CONST_VTBL struct IMFMediaEngineSupportsSourceTransferVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineSupportsSourceTransfer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineSupportsSourceTransfer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineSupportsSourceTransfer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineSupportsSourceTransfer_ShouldTransferSource(This,pfShouldTransfer)	\
    ( (This)->lpVtbl -> ShouldTransferSource(This,pfShouldTransfer) ) 

#define IMFMediaEngineSupportsSourceTransfer_DetachMediaSource(This,ppByteStream,ppMediaSource,ppMSE)	\
    ( (This)->lpVtbl -> DetachMediaSource(This,ppByteStream,ppMediaSource,ppMSE) ) 

#define IMFMediaEngineSupportsSourceTransfer_AttachMediaSource(This,pByteStream,pMediaSource,pMSE)	\
    ( (This)->lpVtbl -> AttachMediaSource(This,pByteStream,pMediaSource,pMSE) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineSupportsSourceTransfer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0033 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0033_v0_0_s_ifspec;

#ifndef __IMFMediaEngineTransferSource_INTERFACE_DEFINED__
#define __IMFMediaEngineTransferSource_INTERFACE_DEFINED__

/* interface IMFMediaEngineTransferSource */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineTransferSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("24230452-fe54-40cc-94f3-fcc394c340d6")
    IMFMediaEngineTransferSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TransferSourceToMediaEngine( 
            /* [annotation][in] */ 
            _In_  IMFMediaEngine *destination) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineTransferSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineTransferSource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineTransferSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineTransferSource * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineTransferSource, TransferSourceToMediaEngine)
        HRESULT ( STDMETHODCALLTYPE *TransferSourceToMediaEngine )( 
            IMFMediaEngineTransferSource * This,
            /* [annotation][in] */ 
            _In_  IMFMediaEngine *destination);
        
        END_INTERFACE
    } IMFMediaEngineTransferSourceVtbl;

    interface IMFMediaEngineTransferSource
    {
        CONST_VTBL struct IMFMediaEngineTransferSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineTransferSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineTransferSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineTransferSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineTransferSource_TransferSourceToMediaEngine(This,destination)	\
    ( (This)->lpVtbl -> TransferSourceToMediaEngine(This,destination) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineTransferSource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0034 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID(CLSID_MFMediaEngineClassFactory,
0xb44392da, 0x499b, 0x446b, 0xa4, 0xcb, 0x0, 0x5f, 0xea, 0xd0, 0xe6, 0xd5);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (NO_MEDIA_ENGINE_FACTORY) 
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MF_MEDIA_ENGINE_TIMEDTEXT, 0x805ea411, 0x92e0, 0x4e59, 0x9b, 0x6e, 0x5c, 0x7d, 0x79, 0x15, 0xe6, 0x4f);
typedef 
enum MF_TIMED_TEXT_TRACK_KIND
    {
        MF_TIMED_TEXT_TRACK_KIND_UNKNOWN	= 0,
        MF_TIMED_TEXT_TRACK_KIND_SUBTITLES	= 1,
        MF_TIMED_TEXT_TRACK_KIND_CAPTIONS	= 2,
        MF_TIMED_TEXT_TRACK_KIND_METADATA	= 3
    } 	MF_TIMED_TEXT_TRACK_KIND;

typedef 
enum MF_TIMED_TEXT_UNIT_TYPE
    {
        MF_TIMED_TEXT_UNIT_TYPE_PIXELS	= 0,
        MF_TIMED_TEXT_UNIT_TYPE_PERCENTAGE	= 1
    } 	MF_TIMED_TEXT_UNIT_TYPE;

typedef 
enum MF_TIMED_TEXT_FONT_STYLE
    {
        MF_TIMED_TEXT_FONT_STYLE_NORMAL	= 0,
        MF_TIMED_TEXT_FONT_STYLE_OBLIQUE	= 1,
        MF_TIMED_TEXT_FONT_STYLE_ITALIC	= 2
    } 	MF_TIMED_TEXT_FONT_STYLE;

typedef 
enum MF_TIMED_TEXT_ALIGNMENT
    {
        MF_TIMED_TEXT_ALIGNMENT_START	= 0,
        MF_TIMED_TEXT_ALIGNMENT_END	= 1,
        MF_TIMED_TEXT_ALIGNMENT_CENTER	= 2
    } 	MF_TIMED_TEXT_ALIGNMENT;

typedef 
enum MF_TIMED_TEXT_DISPLAY_ALIGNMENT
    {
        MF_TIMED_TEXT_DISPLAY_ALIGNMENT_BEFORE	= 0,
        MF_TIMED_TEXT_DISPLAY_ALIGNMENT_AFTER	= 1,
        MF_TIMED_TEXT_DISPLAY_ALIGNMENT_CENTER	= 2
    } 	MF_TIMED_TEXT_DISPLAY_ALIGNMENT;

typedef 
enum MF_TIMED_TEXT_DECORATION
    {
        MF_TIMED_TEXT_DECORATION_NONE	= 0,
        MF_TIMED_TEXT_DECORATION_UNDERLINE	= 1,
        MF_TIMED_TEXT_DECORATION_LINE_THROUGH	= 2,
        MF_TIMED_TEXT_DECORATION_OVERLINE	= 4
    } 	MF_TIMED_TEXT_DECORATION;

typedef 
enum MF_TIMED_TEXT_WRITING_MODE
    {
        MF_TIMED_TEXT_WRITING_MODE_LRTB	= 0,
        MF_TIMED_TEXT_WRITING_MODE_RLTB	= 1,
        MF_TIMED_TEXT_WRITING_MODE_TBRL	= 2,
        MF_TIMED_TEXT_WRITING_MODE_TBLR	= 3,
        MF_TIMED_TEXT_WRITING_MODE_LR	= 4,
        MF_TIMED_TEXT_WRITING_MODE_RL	= 5,
        MF_TIMED_TEXT_WRITING_MODE_TB	= 6
    } 	MF_TIMED_TEXT_WRITING_MODE;

typedef 
enum MF_TIMED_TEXT_SCROLL_MODE
    {
        MF_TIMED_TEXT_SCROLL_MODE_POP_ON	= 0,
        MF_TIMED_TEXT_SCROLL_MODE_ROLL_UP	= 1
    } 	MF_TIMED_TEXT_SCROLL_MODE;

typedef 
enum MF_TIMED_TEXT_ERROR_CODE
    {
        MF_TIMED_TEXT_ERROR_CODE_NOERROR	= 0,
        MF_TIMED_TEXT_ERROR_CODE_FATAL	= 1,
        MF_TIMED_TEXT_ERROR_CODE_DATA_FORMAT	= 2,
        MF_TIMED_TEXT_ERROR_CODE_NETWORK	= 3,
        MF_TIMED_TEXT_ERROR_CODE_INTERNAL	= 4
    } 	MF_TIMED_TEXT_ERROR_CODE;

typedef 
enum MF_TIMED_TEXT_CUE_EVENT
    {
        MF_TIMED_TEXT_CUE_EVENT_ACTIVE	= 0,
        MF_TIMED_TEXT_CUE_EVENT_INACTIVE	= ( MF_TIMED_TEXT_CUE_EVENT_ACTIVE + 1 ) ,
        MF_TIMED_TEXT_CUE_EVENT_CLEAR	= ( MF_TIMED_TEXT_CUE_EVENT_INACTIVE + 1 ) 
    } 	MF_TIMED_TEXT_CUE_EVENT;

typedef 
enum MF_TIMED_TEXT_TRACK_READY_STATE
    {
        MF_TIMED_TEXT_TRACK_READY_STATE_NONE	= 0,
        MF_TIMED_TEXT_TRACK_READY_STATE_LOADING	= ( MF_TIMED_TEXT_TRACK_READY_STATE_NONE + 1 ) ,
        MF_TIMED_TEXT_TRACK_READY_STATE_LOADED	= ( MF_TIMED_TEXT_TRACK_READY_STATE_LOADING + 1 ) ,
        MF_TIMED_TEXT_TRACK_READY_STATE_ERROR	= ( MF_TIMED_TEXT_TRACK_READY_STATE_LOADED + 1 ) 
    } 	MF_TIMED_TEXT_TRACK_READY_STATE;

typedef 
enum MF_TIMED_TEXT_RUBY_POSITION
    {
        MF_TIMED_TEXT_RUBY_POSITION_BEFORE	= 0,
        MF_TIMED_TEXT_RUBY_POSITION_AFTER	= 1,
        MF_TIMED_TEXT_RUBY_POSITION_OUTSIDE	= 2
    } 	MF_TIMED_TEXT_RUBY_POSITION;

typedef 
enum MF_TIMED_TEXT_RUBY_ALIGN
    {
        MF_TIMED_TEXT_RUBY_ALIGN_CENTER	= 0,
        MF_TIMED_TEXT_RUBY_ALIGN_START	= 1,
        MF_TIMED_TEXT_RUBY_ALIGN_END	= 2,
        MF_TIMED_TEXT_RUBY_ALIGN_SPACEAROUND	= 3,
        MF_TIMED_TEXT_RUBY_ALIGN_SPACEBETWEEN	= 4,
        MF_TIMED_TEXT_RUBY_ALIGN_WITHBASE	= 5
    } 	MF_TIMED_TEXT_RUBY_ALIGN;

typedef 
enum MF_TIMED_TEXT_RUBY_RESERVE
    {
        MF_TIMED_TEXT_RUBY_RESERVE_NONE	= 0,
        MF_TIMED_TEXT_RUBY_RESERVE_BEFORE	= 1,
        MF_TIMED_TEXT_RUBY_RESERVE_AFTER	= 2,
        MF_TIMED_TEXT_RUBY_RESERVE_BOTH	= 3,
        MF_TIMED_TEXT_RUBY_RESERVE_OUTSIDE	= 4
    } 	MF_TIMED_TEXT_RUBY_RESERVE;

typedef 
enum MF_TIMED_TEXT_BOUTEN_TYPE
    {
        MF_TIMED_TEXT_BOUTEN_TYPE_NONE	= 0,
        MF_TIMED_TEXT_BOUTEN_TYPE_AUTO	= 1,
        MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDCIRCLE	= 2,
        MF_TIMED_TEXT_BOUTEN_TYPE_OPENCIRCLE	= 3,
        MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDDOT	= 4,
        MF_TIMED_TEXT_BOUTEN_TYPE_OPENDOT	= 5,
        MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDSESAME	= 6,
        MF_TIMED_TEXT_BOUTEN_TYPE_OPENSESAME	= 7
    } 	MF_TIMED_TEXT_BOUTEN_TYPE;

typedef 
enum MF_TIMED_TEXT_BOUTEN_POSITION
    {
        MF_TIMED_TEXT_BOUTEN_POSITION_BEFORE	= 0,
        MF_TIMED_TEXT_BOUTEN_POSITION_AFTER	= 1,
        MF_TIMED_TEXT_BOUTEN_POSITION_OUTSIDE	= 2
    } 	MF_TIMED_TEXT_BOUTEN_POSITION;













extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0034_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0034_v0_0_s_ifspec;

#ifndef __IMFTimedText_INTERFACE_DEFINED__
#define __IMFTimedText_INTERFACE_DEFINED__

/* interface IMFTimedText */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1f2a94c9-a3df-430d-9d0f-acd85ddc29af")
    IMFTimedText : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterNotifications( 
            /* [annotation][in] */ 
            _In_opt_  IMFTimedTextNotify *notify) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectTrack( 
            /* [annotation][in] */ 
            _In_  DWORD trackId,
            /* [annotation][in] */ 
            _In_  BOOL selected) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDataSource( 
            /* [annotation][in] */ 
            _In_  IMFByteStream *byteStream,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR label,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR language,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_TRACK_KIND kind,
            /* [annotation][in] */ 
            _In_  BOOL isDefault,
            /* [annotation][in] */ 
            _Out_  DWORD *trackId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDataSourceFromUrl( 
            /* [annotation][in] */ 
            _In_  LPCWSTR url,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR label,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR language,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_TRACK_KIND kind,
            /* [annotation][in] */ 
            _In_  BOOL isDefault,
            /* [annotation][in] */ 
            _Out_  DWORD *trackId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTrack( 
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR label,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR language,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_TRACK_KIND kind,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrack **track) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveTrack( 
            /* [annotation][in] */ 
            _In_  IMFTimedTextTrack *track) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCueTimeOffset( 
            /* [annotation][out] */ 
            _Out_  double *offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCueTimeOffset( 
            /* [annotation][in] */ 
            _In_  double offset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTracks( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **tracks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActiveTracks( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **activeTracks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextTracks( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **textTracks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMetadataTracks( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **metadataTracks) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetInBandEnabled( 
            /* [annotation][in] */ 
            _In_  BOOL enabled) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsInBandEnabled( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedText * This);
        
        DECLSPEC_XFGVIRT(IMFTimedText, RegisterNotifications)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotifications )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_opt_  IMFTimedTextNotify *notify);
        
        DECLSPEC_XFGVIRT(IMFTimedText, SelectTrack)
        HRESULT ( STDMETHODCALLTYPE *SelectTrack )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_  DWORD trackId,
            /* [annotation][in] */ 
            _In_  BOOL selected);
        
        DECLSPEC_XFGVIRT(IMFTimedText, AddDataSource)
        HRESULT ( STDMETHODCALLTYPE *AddDataSource )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_  IMFByteStream *byteStream,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR label,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR language,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_TRACK_KIND kind,
            /* [annotation][in] */ 
            _In_  BOOL isDefault,
            /* [annotation][in] */ 
            _Out_  DWORD *trackId);
        
        DECLSPEC_XFGVIRT(IMFTimedText, AddDataSourceFromUrl)
        HRESULT ( STDMETHODCALLTYPE *AddDataSourceFromUrl )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR url,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR label,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR language,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_TRACK_KIND kind,
            /* [annotation][in] */ 
            _In_  BOOL isDefault,
            /* [annotation][in] */ 
            _Out_  DWORD *trackId);
        
        DECLSPEC_XFGVIRT(IMFTimedText, AddTrack)
        HRESULT ( STDMETHODCALLTYPE *AddTrack )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR label,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR language,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_TRACK_KIND kind,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrack **track);
        
        DECLSPEC_XFGVIRT(IMFTimedText, RemoveTrack)
        HRESULT ( STDMETHODCALLTYPE *RemoveTrack )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_  IMFTimedTextTrack *track);
        
        DECLSPEC_XFGVIRT(IMFTimedText, GetCueTimeOffset)
        HRESULT ( STDMETHODCALLTYPE *GetCueTimeOffset )( 
            IMFTimedText * This,
            /* [annotation][out] */ 
            _Out_  double *offset);
        
        DECLSPEC_XFGVIRT(IMFTimedText, SetCueTimeOffset)
        HRESULT ( STDMETHODCALLTYPE *SetCueTimeOffset )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_  double offset);
        
        DECLSPEC_XFGVIRT(IMFTimedText, GetTracks)
        HRESULT ( STDMETHODCALLTYPE *GetTracks )( 
            IMFTimedText * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **tracks);
        
        DECLSPEC_XFGVIRT(IMFTimedText, GetActiveTracks)
        HRESULT ( STDMETHODCALLTYPE *GetActiveTracks )( 
            IMFTimedText * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **activeTracks);
        
        DECLSPEC_XFGVIRT(IMFTimedText, GetTextTracks)
        HRESULT ( STDMETHODCALLTYPE *GetTextTracks )( 
            IMFTimedText * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **textTracks);
        
        DECLSPEC_XFGVIRT(IMFTimedText, GetMetadataTracks)
        HRESULT ( STDMETHODCALLTYPE *GetMetadataTracks )( 
            IMFTimedText * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrackList **metadataTracks);
        
        DECLSPEC_XFGVIRT(IMFTimedText, SetInBandEnabled)
        HRESULT ( STDMETHODCALLTYPE *SetInBandEnabled )( 
            IMFTimedText * This,
            /* [annotation][in] */ 
            _In_  BOOL enabled);
        
        DECLSPEC_XFGVIRT(IMFTimedText, IsInBandEnabled)
        BOOL ( STDMETHODCALLTYPE *IsInBandEnabled )( 
            IMFTimedText * This);
        
        END_INTERFACE
    } IMFTimedTextVtbl;

    interface IMFTimedText
    {
        CONST_VTBL struct IMFTimedTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedText_RegisterNotifications(This,notify)	\
    ( (This)->lpVtbl -> RegisterNotifications(This,notify) ) 

#define IMFTimedText_SelectTrack(This,trackId,selected)	\
    ( (This)->lpVtbl -> SelectTrack(This,trackId,selected) ) 

#define IMFTimedText_AddDataSource(This,byteStream,label,language,kind,isDefault,trackId)	\
    ( (This)->lpVtbl -> AddDataSource(This,byteStream,label,language,kind,isDefault,trackId) ) 

#define IMFTimedText_AddDataSourceFromUrl(This,url,label,language,kind,isDefault,trackId)	\
    ( (This)->lpVtbl -> AddDataSourceFromUrl(This,url,label,language,kind,isDefault,trackId) ) 

#define IMFTimedText_AddTrack(This,label,language,kind,track)	\
    ( (This)->lpVtbl -> AddTrack(This,label,language,kind,track) ) 

#define IMFTimedText_RemoveTrack(This,track)	\
    ( (This)->lpVtbl -> RemoveTrack(This,track) ) 

#define IMFTimedText_GetCueTimeOffset(This,offset)	\
    ( (This)->lpVtbl -> GetCueTimeOffset(This,offset) ) 

#define IMFTimedText_SetCueTimeOffset(This,offset)	\
    ( (This)->lpVtbl -> SetCueTimeOffset(This,offset) ) 

#define IMFTimedText_GetTracks(This,tracks)	\
    ( (This)->lpVtbl -> GetTracks(This,tracks) ) 

#define IMFTimedText_GetActiveTracks(This,activeTracks)	\
    ( (This)->lpVtbl -> GetActiveTracks(This,activeTracks) ) 

#define IMFTimedText_GetTextTracks(This,textTracks)	\
    ( (This)->lpVtbl -> GetTextTracks(This,textTracks) ) 

#define IMFTimedText_GetMetadataTracks(This,metadataTracks)	\
    ( (This)->lpVtbl -> GetMetadataTracks(This,metadataTracks) ) 

#define IMFTimedText_SetInBandEnabled(This,enabled)	\
    ( (This)->lpVtbl -> SetInBandEnabled(This,enabled) ) 

#define IMFTimedText_IsInBandEnabled(This)	\
    ( (This)->lpVtbl -> IsInBandEnabled(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedText_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextNotify_INTERFACE_DEFINED__
#define __IMFTimedTextNotify_INTERFACE_DEFINED__

/* interface IMFTimedTextNotify */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("df6b87b6-ce12-45db-aba7-432fe054e57d")
    IMFTimedTextNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE TrackAdded( 
            /* [annotation][in] */ 
            _In_  DWORD trackId) = 0;
        
        virtual void STDMETHODCALLTYPE TrackRemoved( 
            /* [annotation][in] */ 
            _In_  DWORD trackId) = 0;
        
        virtual void STDMETHODCALLTYPE TrackSelected( 
            /* [annotation][in] */ 
            _In_  DWORD trackId,
            /* [annotation][in] */ 
            _In_  BOOL selected) = 0;
        
        virtual void STDMETHODCALLTYPE TrackReadyStateChanged( 
            /* [annotation][in] */ 
            _In_  DWORD trackId) = 0;
        
        virtual void STDMETHODCALLTYPE Error( 
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_ERROR_CODE errorCode,
            /* [annotation][in] */ 
            _In_  HRESULT extendedErrorCode,
            /* [annotation][in] */ 
            _In_  DWORD sourceTrackId) = 0;
        
        virtual void STDMETHODCALLTYPE Cue( 
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_CUE_EVENT cueEvent,
            /* [annotation][in] */ 
            _In_  double currentTime,
            /* [annotation][in] */ 
            _In_opt_  IMFTimedTextCue *cue) = 0;
        
        virtual void STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextNotify * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, TrackAdded)
        void ( STDMETHODCALLTYPE *TrackAdded )( 
            IMFTimedTextNotify * This,
            /* [annotation][in] */ 
            _In_  DWORD trackId);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, TrackRemoved)
        void ( STDMETHODCALLTYPE *TrackRemoved )( 
            IMFTimedTextNotify * This,
            /* [annotation][in] */ 
            _In_  DWORD trackId);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, TrackSelected)
        void ( STDMETHODCALLTYPE *TrackSelected )( 
            IMFTimedTextNotify * This,
            /* [annotation][in] */ 
            _In_  DWORD trackId,
            /* [annotation][in] */ 
            _In_  BOOL selected);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, TrackReadyStateChanged)
        void ( STDMETHODCALLTYPE *TrackReadyStateChanged )( 
            IMFTimedTextNotify * This,
            /* [annotation][in] */ 
            _In_  DWORD trackId);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, Error)
        void ( STDMETHODCALLTYPE *Error )( 
            IMFTimedTextNotify * This,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_ERROR_CODE errorCode,
            /* [annotation][in] */ 
            _In_  HRESULT extendedErrorCode,
            /* [annotation][in] */ 
            _In_  DWORD sourceTrackId);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, Cue)
        void ( STDMETHODCALLTYPE *Cue )( 
            IMFTimedTextNotify * This,
            /* [annotation][in] */ 
            _In_  MF_TIMED_TEXT_CUE_EVENT cueEvent,
            /* [annotation][in] */ 
            _In_  double currentTime,
            /* [annotation][in] */ 
            _In_opt_  IMFTimedTextCue *cue);
        
        DECLSPEC_XFGVIRT(IMFTimedTextNotify, Reset)
        void ( STDMETHODCALLTYPE *Reset )( 
            IMFTimedTextNotify * This);
        
        END_INTERFACE
    } IMFTimedTextNotifyVtbl;

    interface IMFTimedTextNotify
    {
        CONST_VTBL struct IMFTimedTextNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextNotify_TrackAdded(This,trackId)	\
    ( (This)->lpVtbl -> TrackAdded(This,trackId) ) 

#define IMFTimedTextNotify_TrackRemoved(This,trackId)	\
    ( (This)->lpVtbl -> TrackRemoved(This,trackId) ) 

#define IMFTimedTextNotify_TrackSelected(This,trackId,selected)	\
    ( (This)->lpVtbl -> TrackSelected(This,trackId,selected) ) 

#define IMFTimedTextNotify_TrackReadyStateChanged(This,trackId)	\
    ( (This)->lpVtbl -> TrackReadyStateChanged(This,trackId) ) 

#define IMFTimedTextNotify_Error(This,errorCode,extendedErrorCode,sourceTrackId)	\
    ( (This)->lpVtbl -> Error(This,errorCode,extendedErrorCode,sourceTrackId) ) 

#define IMFTimedTextNotify_Cue(This,cueEvent,currentTime,cue)	\
    ( (This)->lpVtbl -> Cue(This,cueEvent,currentTime,cue) ) 

#define IMFTimedTextNotify_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextNotify_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextTrack_INTERFACE_DEFINED__
#define __IMFTimedTextTrack_INTERFACE_DEFINED__

/* interface IMFTimedTextTrack */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextTrack;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8822c32d-654e-4233-bf21-d7f2e67d30d4")
    IMFTimedTextTrack : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetId( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLabel( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *label) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLabel( 
            /* [annotation][in] */ 
            _In_  LPCWSTR label) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguage( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *language) = 0;
        
        virtual MF_TIMED_TEXT_TRACK_KIND STDMETHODCALLTYPE GetTrackKind( void) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsInBand( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInBandMetadataTrackDispatchType( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *dispatchType) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsActive( void) = 0;
        
        virtual MF_TIMED_TEXT_ERROR_CODE STDMETHODCALLTYPE GetErrorCode( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExtendedErrorCode( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDataFormat( 
            /* [annotation][out] */ 
            _Out_  GUID *format) = 0;
        
        virtual MF_TIMED_TEXT_TRACK_READY_STATE STDMETHODCALLTYPE GetReadyState( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCueList( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCueList **cues) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextTrackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextTrack * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetId)
        DWORD ( STDMETHODCALLTYPE *GetId )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetLabel)
        HRESULT ( STDMETHODCALLTYPE *GetLabel )( 
            IMFTimedTextTrack * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *label);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, SetLabel)
        HRESULT ( STDMETHODCALLTYPE *SetLabel )( 
            IMFTimedTextTrack * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR label);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            IMFTimedTextTrack * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *language);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetTrackKind)
        MF_TIMED_TEXT_TRACK_KIND ( STDMETHODCALLTYPE *GetTrackKind )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, IsInBand)
        BOOL ( STDMETHODCALLTYPE *IsInBand )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetInBandMetadataTrackDispatchType)
        HRESULT ( STDMETHODCALLTYPE *GetInBandMetadataTrackDispatchType )( 
            IMFTimedTextTrack * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *dispatchType);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, IsActive)
        BOOL ( STDMETHODCALLTYPE *IsActive )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetErrorCode)
        MF_TIMED_TEXT_ERROR_CODE ( STDMETHODCALLTYPE *GetErrorCode )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetExtendedErrorCode)
        HRESULT ( STDMETHODCALLTYPE *GetExtendedErrorCode )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetDataFormat)
        HRESULT ( STDMETHODCALLTYPE *GetDataFormat )( 
            IMFTimedTextTrack * This,
            /* [annotation][out] */ 
            _Out_  GUID *format);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetReadyState)
        MF_TIMED_TEXT_TRACK_READY_STATE ( STDMETHODCALLTYPE *GetReadyState )( 
            IMFTimedTextTrack * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrack, GetCueList)
        HRESULT ( STDMETHODCALLTYPE *GetCueList )( 
            IMFTimedTextTrack * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCueList **cues);
        
        END_INTERFACE
    } IMFTimedTextTrackVtbl;

    interface IMFTimedTextTrack
    {
        CONST_VTBL struct IMFTimedTextTrackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextTrack_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextTrack_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextTrack_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextTrack_GetId(This)	\
    ( (This)->lpVtbl -> GetId(This) ) 

#define IMFTimedTextTrack_GetLabel(This,label)	\
    ( (This)->lpVtbl -> GetLabel(This,label) ) 

#define IMFTimedTextTrack_SetLabel(This,label)	\
    ( (This)->lpVtbl -> SetLabel(This,label) ) 

#define IMFTimedTextTrack_GetLanguage(This,language)	\
    ( (This)->lpVtbl -> GetLanguage(This,language) ) 

#define IMFTimedTextTrack_GetTrackKind(This)	\
    ( (This)->lpVtbl -> GetTrackKind(This) ) 

#define IMFTimedTextTrack_IsInBand(This)	\
    ( (This)->lpVtbl -> IsInBand(This) ) 

#define IMFTimedTextTrack_GetInBandMetadataTrackDispatchType(This,dispatchType)	\
    ( (This)->lpVtbl -> GetInBandMetadataTrackDispatchType(This,dispatchType) ) 

#define IMFTimedTextTrack_IsActive(This)	\
    ( (This)->lpVtbl -> IsActive(This) ) 

#define IMFTimedTextTrack_GetErrorCode(This)	\
    ( (This)->lpVtbl -> GetErrorCode(This) ) 

#define IMFTimedTextTrack_GetExtendedErrorCode(This)	\
    ( (This)->lpVtbl -> GetExtendedErrorCode(This) ) 

#define IMFTimedTextTrack_GetDataFormat(This,format)	\
    ( (This)->lpVtbl -> GetDataFormat(This,format) ) 

#define IMFTimedTextTrack_GetReadyState(This)	\
    ( (This)->lpVtbl -> GetReadyState(This) ) 

#define IMFTimedTextTrack_GetCueList(This,cues)	\
    ( (This)->lpVtbl -> GetCueList(This,cues) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextTrack_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextTrackList_INTERFACE_DEFINED__
#define __IMFTimedTextTrackList_INTERFACE_DEFINED__

/* interface IMFTimedTextTrackList */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextTrackList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("23ff334c-442c-445f-bccc-edc438aa11e2")
    IMFTimedTextTrackList : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetLength( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTrack( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrack **track) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTrackById( 
            /* [annotation][in] */ 
            _In_  DWORD trackId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrack **track) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextTrackListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextTrackList * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextTrackList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextTrackList * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrackList, GetLength)
        DWORD ( STDMETHODCALLTYPE *GetLength )( 
            IMFTimedTextTrackList * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrackList, GetTrack)
        HRESULT ( STDMETHODCALLTYPE *GetTrack )( 
            IMFTimedTextTrackList * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrack **track);
        
        DECLSPEC_XFGVIRT(IMFTimedTextTrackList, GetTrackById)
        HRESULT ( STDMETHODCALLTYPE *GetTrackById )( 
            IMFTimedTextTrackList * This,
            /* [annotation][in] */ 
            _In_  DWORD trackId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextTrack **track);
        
        END_INTERFACE
    } IMFTimedTextTrackListVtbl;

    interface IMFTimedTextTrackList
    {
        CONST_VTBL struct IMFTimedTextTrackListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextTrackList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextTrackList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextTrackList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextTrackList_GetLength(This)	\
    ( (This)->lpVtbl -> GetLength(This) ) 

#define IMFTimedTextTrackList_GetTrack(This,index,track)	\
    ( (This)->lpVtbl -> GetTrack(This,index,track) ) 

#define IMFTimedTextTrackList_GetTrackById(This,trackId,track)	\
    ( (This)->lpVtbl -> GetTrackById(This,trackId,track) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextTrackList_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextCue_INTERFACE_DEFINED__
#define __IMFTimedTextCue_INTERFACE_DEFINED__

/* interface IMFTimedTextCue */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextCue;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1e560447-9a2b-43e1-a94c-b0aaabfbfbc9")
    IMFTimedTextCue : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetId( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginalId( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *originalId) = 0;
        
        virtual MF_TIMED_TEXT_TRACK_KIND STDMETHODCALLTYPE GetCueKind( void) = 0;
        
        virtual double STDMETHODCALLTYPE GetStartTime( void) = 0;
        
        virtual double STDMETHODCALLTYPE GetDuration( void) = 0;
        
        virtual DWORD STDMETHODCALLTYPE GetTrackId( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextBinary **data) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRegion( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextRegion **region) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStyle( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextStyle **style) = 0;
        
        virtual DWORD STDMETHODCALLTYPE GetLineCount( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLine( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextFormattedText **line) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextCueVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextCue * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetId)
        DWORD ( STDMETHODCALLTYPE *GetId )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetOriginalId)
        HRESULT ( STDMETHODCALLTYPE *GetOriginalId )( 
            IMFTimedTextCue * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *originalId);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetCueKind)
        MF_TIMED_TEXT_TRACK_KIND ( STDMETHODCALLTYPE *GetCueKind )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetStartTime)
        double ( STDMETHODCALLTYPE *GetStartTime )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetDuration)
        double ( STDMETHODCALLTYPE *GetDuration )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetTrackId)
        DWORD ( STDMETHODCALLTYPE *GetTrackId )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMFTimedTextCue * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextBinary **data);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetRegion)
        HRESULT ( STDMETHODCALLTYPE *GetRegion )( 
            IMFTimedTextCue * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextRegion **region);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetStyle)
        HRESULT ( STDMETHODCALLTYPE *GetStyle )( 
            IMFTimedTextCue * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextStyle **style);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetLineCount)
        DWORD ( STDMETHODCALLTYPE *GetLineCount )( 
            IMFTimedTextCue * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCue, GetLine)
        HRESULT ( STDMETHODCALLTYPE *GetLine )( 
            IMFTimedTextCue * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextFormattedText **line);
        
        END_INTERFACE
    } IMFTimedTextCueVtbl;

    interface IMFTimedTextCue
    {
        CONST_VTBL struct IMFTimedTextCueVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextCue_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextCue_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextCue_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextCue_GetId(This)	\
    ( (This)->lpVtbl -> GetId(This) ) 

#define IMFTimedTextCue_GetOriginalId(This,originalId)	\
    ( (This)->lpVtbl -> GetOriginalId(This,originalId) ) 

#define IMFTimedTextCue_GetCueKind(This)	\
    ( (This)->lpVtbl -> GetCueKind(This) ) 

#define IMFTimedTextCue_GetStartTime(This)	\
    ( (This)->lpVtbl -> GetStartTime(This) ) 

#define IMFTimedTextCue_GetDuration(This)	\
    ( (This)->lpVtbl -> GetDuration(This) ) 

#define IMFTimedTextCue_GetTrackId(This)	\
    ( (This)->lpVtbl -> GetTrackId(This) ) 

#define IMFTimedTextCue_GetData(This,data)	\
    ( (This)->lpVtbl -> GetData(This,data) ) 

#define IMFTimedTextCue_GetRegion(This,region)	\
    ( (This)->lpVtbl -> GetRegion(This,region) ) 

#define IMFTimedTextCue_GetStyle(This,style)	\
    ( (This)->lpVtbl -> GetStyle(This,style) ) 

#define IMFTimedTextCue_GetLineCount(This)	\
    ( (This)->lpVtbl -> GetLineCount(This) ) 

#define IMFTimedTextCue_GetLine(This,index,line)	\
    ( (This)->lpVtbl -> GetLine(This,index,line) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextCue_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextFormattedText_INTERFACE_DEFINED__
#define __IMFTimedTextFormattedText_INTERFACE_DEFINED__

/* interface IMFTimedTextFormattedText */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextFormattedText;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e13af3c1-4d47-4354-b1f5-e83ae0ecae60")
    IMFTimedTextFormattedText : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetText( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *text) = 0;
        
        virtual DWORD STDMETHODCALLTYPE GetSubformattingCount( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubformatting( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  DWORD *firstChar,
            /* [annotation][out] */ 
            _Out_  DWORD *charLength,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextStyle **style) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextFormattedTextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextFormattedText * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextFormattedText * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextFormattedText * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextFormattedText, GetText)
        HRESULT ( STDMETHODCALLTYPE *GetText )( 
            IMFTimedTextFormattedText * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  LPWSTR *text);
        
        DECLSPEC_XFGVIRT(IMFTimedTextFormattedText, GetSubformattingCount)
        DWORD ( STDMETHODCALLTYPE *GetSubformattingCount )( 
            IMFTimedTextFormattedText * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextFormattedText, GetSubformatting)
        HRESULT ( STDMETHODCALLTYPE *GetSubformatting )( 
            IMFTimedTextFormattedText * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _Out_  DWORD *firstChar,
            /* [annotation][out] */ 
            _Out_  DWORD *charLength,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextStyle **style);
        
        END_INTERFACE
    } IMFTimedTextFormattedTextVtbl;

    interface IMFTimedTextFormattedText
    {
        CONST_VTBL struct IMFTimedTextFormattedTextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextFormattedText_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextFormattedText_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextFormattedText_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextFormattedText_GetText(This,text)	\
    ( (This)->lpVtbl -> GetText(This,text) ) 

#define IMFTimedTextFormattedText_GetSubformattingCount(This)	\
    ( (This)->lpVtbl -> GetSubformattingCount(This) ) 

#define IMFTimedTextFormattedText_GetSubformatting(This,index,firstChar,charLength,style)	\
    ( (This)->lpVtbl -> GetSubformatting(This,index,firstChar,charLength,style) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextFormattedText_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextStyle_INTERFACE_DEFINED__
#define __IMFTimedTextStyle_INTERFACE_DEFINED__

/* interface IMFTimedTextStyle */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextStyle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09b2455d-b834-4f01-a347-9052e21c450e")
    IMFTimedTextStyle : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *name) = 0;
        
        virtual BOOL STDMETHODCALLTYPE IsExternal( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontFamily( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *fontFamily) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontSize( 
            /* [annotation][out] */ 
            _Out_  double *fontSize,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColor( 
            /* [annotation][out] */ 
            _Out_  MFARGB *color) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBackgroundColor( 
            /* [annotation][out] */ 
            _Out_  MFARGB *bgColor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetShowBackgroundAlways( 
            /* [annotation][out] */ 
            _Out_  BOOL *showBackgroundAlways) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontStyle( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_FONT_STYLE *fontStyle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBold( 
            /* [annotation][out] */ 
            _Out_  BOOL *bold) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRightToLeft( 
            /* [annotation][out] */ 
            _Out_  BOOL *rightToLeft) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextAlignment( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_ALIGNMENT *textAlign) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextDecoration( 
            /* [annotation][out] */ 
            _Out_  DWORD *textDecoration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTextOutline( 
            /* [annotation][out] */ 
            _Out_  MFARGB *color,
            /* [annotation][out] */ 
            _Out_  double *thickness,
            /* [annotation][out] */ 
            _Out_  double *blurRadius,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextStyleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextStyle * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextStyle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextStyle * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, IsExternal)
        BOOL ( STDMETHODCALLTYPE *IsExternal )( 
            IMFTimedTextStyle * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetFontFamily)
        HRESULT ( STDMETHODCALLTYPE *GetFontFamily )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *fontFamily);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetFontSize)
        HRESULT ( STDMETHODCALLTYPE *GetFontSize )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  double *fontSize,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetColor)
        HRESULT ( STDMETHODCALLTYPE *GetColor )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  MFARGB *color);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetBackgroundColor)
        HRESULT ( STDMETHODCALLTYPE *GetBackgroundColor )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  MFARGB *bgColor);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetShowBackgroundAlways)
        HRESULT ( STDMETHODCALLTYPE *GetShowBackgroundAlways )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  BOOL *showBackgroundAlways);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetFontStyle)
        HRESULT ( STDMETHODCALLTYPE *GetFontStyle )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_FONT_STYLE *fontStyle);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetBold)
        HRESULT ( STDMETHODCALLTYPE *GetBold )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  BOOL *bold);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetRightToLeft)
        HRESULT ( STDMETHODCALLTYPE *GetRightToLeft )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  BOOL *rightToLeft);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetTextAlignment)
        HRESULT ( STDMETHODCALLTYPE *GetTextAlignment )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_ALIGNMENT *textAlign);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetTextDecoration)
        HRESULT ( STDMETHODCALLTYPE *GetTextDecoration )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  DWORD *textDecoration);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle, GetTextOutline)
        HRESULT ( STDMETHODCALLTYPE *GetTextOutline )( 
            IMFTimedTextStyle * This,
            /* [annotation][out] */ 
            _Out_  MFARGB *color,
            /* [annotation][out] */ 
            _Out_  double *thickness,
            /* [annotation][out] */ 
            _Out_  double *blurRadius,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType);
        
        END_INTERFACE
    } IMFTimedTextStyleVtbl;

    interface IMFTimedTextStyle
    {
        CONST_VTBL struct IMFTimedTextStyleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextStyle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextStyle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextStyle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextStyle_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IMFTimedTextStyle_IsExternal(This)	\
    ( (This)->lpVtbl -> IsExternal(This) ) 

#define IMFTimedTextStyle_GetFontFamily(This,fontFamily)	\
    ( (This)->lpVtbl -> GetFontFamily(This,fontFamily) ) 

#define IMFTimedTextStyle_GetFontSize(This,fontSize,unitType)	\
    ( (This)->lpVtbl -> GetFontSize(This,fontSize,unitType) ) 

#define IMFTimedTextStyle_GetColor(This,color)	\
    ( (This)->lpVtbl -> GetColor(This,color) ) 

#define IMFTimedTextStyle_GetBackgroundColor(This,bgColor)	\
    ( (This)->lpVtbl -> GetBackgroundColor(This,bgColor) ) 

#define IMFTimedTextStyle_GetShowBackgroundAlways(This,showBackgroundAlways)	\
    ( (This)->lpVtbl -> GetShowBackgroundAlways(This,showBackgroundAlways) ) 

#define IMFTimedTextStyle_GetFontStyle(This,fontStyle)	\
    ( (This)->lpVtbl -> GetFontStyle(This,fontStyle) ) 

#define IMFTimedTextStyle_GetBold(This,bold)	\
    ( (This)->lpVtbl -> GetBold(This,bold) ) 

#define IMFTimedTextStyle_GetRightToLeft(This,rightToLeft)	\
    ( (This)->lpVtbl -> GetRightToLeft(This,rightToLeft) ) 

#define IMFTimedTextStyle_GetTextAlignment(This,textAlign)	\
    ( (This)->lpVtbl -> GetTextAlignment(This,textAlign) ) 

#define IMFTimedTextStyle_GetTextDecoration(This,textDecoration)	\
    ( (This)->lpVtbl -> GetTextDecoration(This,textDecoration) ) 

#define IMFTimedTextStyle_GetTextOutline(This,color,thickness,blurRadius,unitType)	\
    ( (This)->lpVtbl -> GetTextOutline(This,color,thickness,blurRadius,unitType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextStyle_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextRegion_INTERFACE_DEFINED__
#define __IMFTimedTextRegion_INTERFACE_DEFINED__

/* interface IMFTimedTextRegion */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextRegion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c8d22afc-bc47-4bdf-9b04-787e49ce3f58")
    IMFTimedTextRegion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetName( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPosition( 
            /* [annotation][out] */ 
            _Out_  double *pX,
            /* [annotation][out] */ 
            _Out_  double *pY,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetExtent( 
            /* [annotation][out] */ 
            _Out_  double *pWidth,
            /* [annotation][out] */ 
            _Out_  double *pHeight,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBackgroundColor( 
            /* [annotation][out] */ 
            _Out_  MFARGB *bgColor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWritingMode( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_WRITING_MODE *writingMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayAlignment( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_DISPLAY_ALIGNMENT *displayAlign) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLineHeight( 
            /* [annotation][out] */ 
            _Out_  double *pLineHeight,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClipOverflow( 
            /* [annotation][out] */ 
            _Out_  BOOL *clipOverflow) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPadding( 
            /* [annotation][out] */ 
            _Out_  double *before,
            /* [annotation][out] */ 
            _Out_  double *start,
            /* [annotation][out] */ 
            _Out_  double *after,
            /* [annotation][out] */ 
            _Out_  double *end,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWrap( 
            /* [annotation][out] */ 
            _Out_  BOOL *wrap) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZIndex( 
            /* [annotation][out] */ 
            _Out_  INT32 *zIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetScrollMode( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_SCROLL_MODE *scrollMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextRegionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextRegion * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextRegion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextRegion * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetName)
        HRESULT ( STDMETHODCALLTYPE *GetName )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *name);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetPosition)
        HRESULT ( STDMETHODCALLTYPE *GetPosition )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  double *pX,
            /* [annotation][out] */ 
            _Out_  double *pY,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetExtent)
        HRESULT ( STDMETHODCALLTYPE *GetExtent )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  double *pWidth,
            /* [annotation][out] */ 
            _Out_  double *pHeight,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetBackgroundColor)
        HRESULT ( STDMETHODCALLTYPE *GetBackgroundColor )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  MFARGB *bgColor);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetWritingMode)
        HRESULT ( STDMETHODCALLTYPE *GetWritingMode )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_WRITING_MODE *writingMode);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetDisplayAlignment)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayAlignment )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_DISPLAY_ALIGNMENT *displayAlign);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetLineHeight)
        HRESULT ( STDMETHODCALLTYPE *GetLineHeight )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  double *pLineHeight,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetClipOverflow)
        HRESULT ( STDMETHODCALLTYPE *GetClipOverflow )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  BOOL *clipOverflow);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetPadding)
        HRESULT ( STDMETHODCALLTYPE *GetPadding )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  double *before,
            /* [annotation][out] */ 
            _Out_  double *start,
            /* [annotation][out] */ 
            _Out_  double *after,
            /* [annotation][out] */ 
            _Out_  double *end,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_UNIT_TYPE *unitType);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetWrap)
        HRESULT ( STDMETHODCALLTYPE *GetWrap )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  BOOL *wrap);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetZIndex)
        HRESULT ( STDMETHODCALLTYPE *GetZIndex )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  INT32 *zIndex);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRegion, GetScrollMode)
        HRESULT ( STDMETHODCALLTYPE *GetScrollMode )( 
            IMFTimedTextRegion * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_SCROLL_MODE *scrollMode);
        
        END_INTERFACE
    } IMFTimedTextRegionVtbl;

    interface IMFTimedTextRegion
    {
        CONST_VTBL struct IMFTimedTextRegionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextRegion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextRegion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextRegion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextRegion_GetName(This,name)	\
    ( (This)->lpVtbl -> GetName(This,name) ) 

#define IMFTimedTextRegion_GetPosition(This,pX,pY,unitType)	\
    ( (This)->lpVtbl -> GetPosition(This,pX,pY,unitType) ) 

#define IMFTimedTextRegion_GetExtent(This,pWidth,pHeight,unitType)	\
    ( (This)->lpVtbl -> GetExtent(This,pWidth,pHeight,unitType) ) 

#define IMFTimedTextRegion_GetBackgroundColor(This,bgColor)	\
    ( (This)->lpVtbl -> GetBackgroundColor(This,bgColor) ) 

#define IMFTimedTextRegion_GetWritingMode(This,writingMode)	\
    ( (This)->lpVtbl -> GetWritingMode(This,writingMode) ) 

#define IMFTimedTextRegion_GetDisplayAlignment(This,displayAlign)	\
    ( (This)->lpVtbl -> GetDisplayAlignment(This,displayAlign) ) 

#define IMFTimedTextRegion_GetLineHeight(This,pLineHeight,unitType)	\
    ( (This)->lpVtbl -> GetLineHeight(This,pLineHeight,unitType) ) 

#define IMFTimedTextRegion_GetClipOverflow(This,clipOverflow)	\
    ( (This)->lpVtbl -> GetClipOverflow(This,clipOverflow) ) 

#define IMFTimedTextRegion_GetPadding(This,before,start,after,end,unitType)	\
    ( (This)->lpVtbl -> GetPadding(This,before,start,after,end,unitType) ) 

#define IMFTimedTextRegion_GetWrap(This,wrap)	\
    ( (This)->lpVtbl -> GetWrap(This,wrap) ) 

#define IMFTimedTextRegion_GetZIndex(This,zIndex)	\
    ( (This)->lpVtbl -> GetZIndex(This,zIndex) ) 

#define IMFTimedTextRegion_GetScrollMode(This,scrollMode)	\
    ( (This)->lpVtbl -> GetScrollMode(This,scrollMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextRegion_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextBinary_INTERFACE_DEFINED__
#define __IMFTimedTextBinary_INTERFACE_DEFINED__

/* interface IMFTimedTextBinary */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextBinary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ae3a412-0545-43c4-bf6f-6b97a5c6c432")
    IMFTimedTextBinary : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*length)  const BYTE **data,
            /* [annotation][out] */ 
            _Out_  DWORD *length) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextBinaryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextBinary * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextBinary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextBinary * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextBinary, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMFTimedTextBinary * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*length)  const BYTE **data,
            /* [annotation][out] */ 
            _Out_  DWORD *length);
        
        END_INTERFACE
    } IMFTimedTextBinaryVtbl;

    interface IMFTimedTextBinary
    {
        CONST_VTBL struct IMFTimedTextBinaryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextBinary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextBinary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextBinary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextBinary_GetData(This,data,length)	\
    ( (This)->lpVtbl -> GetData(This,data,length) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextBinary_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextCueList_INTERFACE_DEFINED__
#define __IMFTimedTextCueList_INTERFACE_DEFINED__

/* interface IMFTimedTextCueList */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextCueList;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ad128745-211b-40a0-9981-fe65f166d0fd")
    IMFTimedTextCueList : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetLength( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCueByIndex( 
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCue **cue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCueById( 
            /* [annotation][in] */ 
            _In_  DWORD id,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCue **cue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCueByOriginalId( 
            /* [annotation][in] */ 
            _In_  LPCWSTR originalId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCue **cue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddTextCue( 
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double duration,
            /* [annotation][in] */ 
            _In_  LPCWSTR text,
            /* [annotation][out] */ 
            _COM_Outptr_opt_  IMFTimedTextCue **cue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddDataCue( 
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double duration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dataSize)  const BYTE *data,
            /* [annotation][in] */ 
            _In_  DWORD dataSize,
            /* [annotation][out] */ 
            _COM_Outptr_opt_  IMFTimedTextCue **cue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveCue( 
            /* [annotation][in] */ 
            _In_  IMFTimedTextCue *cue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextCueListVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextCueList * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextCueList * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextCueList * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, GetLength)
        DWORD ( STDMETHODCALLTYPE *GetLength )( 
            IMFTimedTextCueList * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, GetCueByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetCueByIndex )( 
            IMFTimedTextCueList * This,
            /* [annotation][in] */ 
            _In_  DWORD index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCue **cue);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, GetCueById)
        HRESULT ( STDMETHODCALLTYPE *GetCueById )( 
            IMFTimedTextCueList * This,
            /* [annotation][in] */ 
            _In_  DWORD id,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCue **cue);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, GetCueByOriginalId)
        HRESULT ( STDMETHODCALLTYPE *GetCueByOriginalId )( 
            IMFTimedTextCueList * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR originalId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFTimedTextCue **cue);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, AddTextCue)
        HRESULT ( STDMETHODCALLTYPE *AddTextCue )( 
            IMFTimedTextCueList * This,
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double duration,
            /* [annotation][in] */ 
            _In_  LPCWSTR text,
            /* [annotation][out] */ 
            _COM_Outptr_opt_  IMFTimedTextCue **cue);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, AddDataCue)
        HRESULT ( STDMETHODCALLTYPE *AddDataCue )( 
            IMFTimedTextCueList * This,
            /* [annotation][in] */ 
            _In_  double start,
            /* [annotation][in] */ 
            _In_  double duration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dataSize)  const BYTE *data,
            /* [annotation][in] */ 
            _In_  DWORD dataSize,
            /* [annotation][out] */ 
            _COM_Outptr_opt_  IMFTimedTextCue **cue);
        
        DECLSPEC_XFGVIRT(IMFTimedTextCueList, RemoveCue)
        HRESULT ( STDMETHODCALLTYPE *RemoveCue )( 
            IMFTimedTextCueList * This,
            /* [annotation][in] */ 
            _In_  IMFTimedTextCue *cue);
        
        END_INTERFACE
    } IMFTimedTextCueListVtbl;

    interface IMFTimedTextCueList
    {
        CONST_VTBL struct IMFTimedTextCueListVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextCueList_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextCueList_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextCueList_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextCueList_GetLength(This)	\
    ( (This)->lpVtbl -> GetLength(This) ) 

#define IMFTimedTextCueList_GetCueByIndex(This,index,cue)	\
    ( (This)->lpVtbl -> GetCueByIndex(This,index,cue) ) 

#define IMFTimedTextCueList_GetCueById(This,id,cue)	\
    ( (This)->lpVtbl -> GetCueById(This,id,cue) ) 

#define IMFTimedTextCueList_GetCueByOriginalId(This,originalId,cue)	\
    ( (This)->lpVtbl -> GetCueByOriginalId(This,originalId,cue) ) 

#define IMFTimedTextCueList_AddTextCue(This,start,duration,text,cue)	\
    ( (This)->lpVtbl -> AddTextCue(This,start,duration,text,cue) ) 

#define IMFTimedTextCueList_AddDataCue(This,start,duration,data,dataSize,cue)	\
    ( (This)->lpVtbl -> AddDataCue(This,start,duration,data,dataSize,cue) ) 

#define IMFTimedTextCueList_RemoveCue(This,cue)	\
    ( (This)->lpVtbl -> RemoveCue(This,cue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextCueList_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextRuby_INTERFACE_DEFINED__
#define __IMFTimedTextRuby_INTERFACE_DEFINED__

/* interface IMFTimedTextRuby */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextRuby;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76c6a6f5-4955-4de5-b27b-14b734cc14b4")
    IMFTimedTextRuby : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRubyText( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *rubyText) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRubyPosition( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_RUBY_POSITION *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRubyAlign( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_RUBY_ALIGN *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRubyReserve( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_RUBY_RESERVE *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextRubyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextRuby * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextRuby * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextRuby * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRuby, GetRubyText)
        HRESULT ( STDMETHODCALLTYPE *GetRubyText )( 
            IMFTimedTextRuby * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *rubyText);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRuby, GetRubyPosition)
        HRESULT ( STDMETHODCALLTYPE *GetRubyPosition )( 
            IMFTimedTextRuby * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_RUBY_POSITION *value);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRuby, GetRubyAlign)
        HRESULT ( STDMETHODCALLTYPE *GetRubyAlign )( 
            IMFTimedTextRuby * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_RUBY_ALIGN *value);
        
        DECLSPEC_XFGVIRT(IMFTimedTextRuby, GetRubyReserve)
        HRESULT ( STDMETHODCALLTYPE *GetRubyReserve )( 
            IMFTimedTextRuby * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_RUBY_RESERVE *value);
        
        END_INTERFACE
    } IMFTimedTextRubyVtbl;

    interface IMFTimedTextRuby
    {
        CONST_VTBL struct IMFTimedTextRubyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextRuby_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextRuby_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextRuby_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextRuby_GetRubyText(This,rubyText)	\
    ( (This)->lpVtbl -> GetRubyText(This,rubyText) ) 

#define IMFTimedTextRuby_GetRubyPosition(This,value)	\
    ( (This)->lpVtbl -> GetRubyPosition(This,value) ) 

#define IMFTimedTextRuby_GetRubyAlign(This,value)	\
    ( (This)->lpVtbl -> GetRubyAlign(This,value) ) 

#define IMFTimedTextRuby_GetRubyReserve(This,value)	\
    ( (This)->lpVtbl -> GetRubyReserve(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextRuby_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextBouten_INTERFACE_DEFINED__
#define __IMFTimedTextBouten_INTERFACE_DEFINED__

/* interface IMFTimedTextBouten */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextBouten;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3c5f3e8a-90c0-464e-8136-898d2975f847")
    IMFTimedTextBouten : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetBoutenType( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_BOUTEN_TYPE *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBoutenColor( 
            /* [annotation][out] */ 
            _Out_  MFARGB *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBoutenPosition( 
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_BOUTEN_POSITION *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextBoutenVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextBouten * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextBouten * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextBouten * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextBouten, GetBoutenType)
        HRESULT ( STDMETHODCALLTYPE *GetBoutenType )( 
            IMFTimedTextBouten * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_BOUTEN_TYPE *value);
        
        DECLSPEC_XFGVIRT(IMFTimedTextBouten, GetBoutenColor)
        HRESULT ( STDMETHODCALLTYPE *GetBoutenColor )( 
            IMFTimedTextBouten * This,
            /* [annotation][out] */ 
            _Out_  MFARGB *value);
        
        DECLSPEC_XFGVIRT(IMFTimedTextBouten, GetBoutenPosition)
        HRESULT ( STDMETHODCALLTYPE *GetBoutenPosition )( 
            IMFTimedTextBouten * This,
            /* [annotation][out] */ 
            _Out_  MF_TIMED_TEXT_BOUTEN_POSITION *value);
        
        END_INTERFACE
    } IMFTimedTextBoutenVtbl;

    interface IMFTimedTextBouten
    {
        CONST_VTBL struct IMFTimedTextBoutenVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextBouten_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextBouten_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextBouten_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextBouten_GetBoutenType(This,value)	\
    ( (This)->lpVtbl -> GetBoutenType(This,value) ) 

#define IMFTimedTextBouten_GetBoutenColor(This,value)	\
    ( (This)->lpVtbl -> GetBoutenColor(This,value) ) 

#define IMFTimedTextBouten_GetBoutenPosition(This,value)	\
    ( (This)->lpVtbl -> GetBoutenPosition(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextBouten_INTERFACE_DEFINED__ */


#ifndef __IMFTimedTextStyle2_INTERFACE_DEFINED__
#define __IMFTimedTextStyle2_INTERFACE_DEFINED__

/* interface IMFTimedTextStyle2 */
/* [local][unique][uuid][object] */ 


EXTERN_C const IID IID_IMFTimedTextStyle2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("db639199-c809-4c89-bfca-d0bbb9729d6e")
    IMFTimedTextStyle2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRuby( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextRuby **ruby) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBouten( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextBouten **bouten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsTextCombined( 
            /* [annotation][out] */ 
            _Out_  BOOL *value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFontAngleInDegrees( 
            /* [annotation][out] */ 
            _Out_  double *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimedTextStyle2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimedTextStyle2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimedTextStyle2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimedTextStyle2 * This);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle2, GetRuby)
        HRESULT ( STDMETHODCALLTYPE *GetRuby )( 
            IMFTimedTextStyle2 * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextRuby **ruby);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle2, GetBouten)
        HRESULT ( STDMETHODCALLTYPE *GetBouten )( 
            IMFTimedTextStyle2 * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFTimedTextBouten **bouten);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle2, IsTextCombined)
        HRESULT ( STDMETHODCALLTYPE *IsTextCombined )( 
            IMFTimedTextStyle2 * This,
            /* [annotation][out] */ 
            _Out_  BOOL *value);
        
        DECLSPEC_XFGVIRT(IMFTimedTextStyle2, GetFontAngleInDegrees)
        HRESULT ( STDMETHODCALLTYPE *GetFontAngleInDegrees )( 
            IMFTimedTextStyle2 * This,
            /* [annotation][out] */ 
            _Out_  double *value);
        
        END_INTERFACE
    } IMFTimedTextStyle2Vtbl;

    interface IMFTimedTextStyle2
    {
        CONST_VTBL struct IMFTimedTextStyle2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimedTextStyle2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimedTextStyle2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimedTextStyle2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimedTextStyle2_GetRuby(This,ruby)	\
    ( (This)->lpVtbl -> GetRuby(This,ruby) ) 

#define IMFTimedTextStyle2_GetBouten(This,bouten)	\
    ( (This)->lpVtbl -> GetBouten(This,bouten) ) 

#define IMFTimedTextStyle2_IsTextCombined(This,value)	\
    ( (This)->lpVtbl -> IsTextCombined(This,value) ) 

#define IMFTimedTextStyle2_GetFontAngleInDegrees(This,value)	\
    ( (This)->lpVtbl -> GetFontAngleInDegrees(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimedTextStyle2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0047 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_GUID(MF_MEDIA_ENGINE_CONTINUE_ON_CODEC_ERROR,
0xdbcdb7f9,0x48e4,0x4295,0xb7,0x0d,0xd5,0x18,0x23,0x4e,0xeb,0x38);
typedef 
enum MF_MEDIA_ENGINE_STREAMTYPE_FAILED
    {
        MF_MEDIA_ENGINE_STREAMTYPE_FAILED_UNKNOWN	= 0,
        MF_MEDIA_ENGINE_STREAMTYPE_FAILED_AUDIO	= 1,
        MF_MEDIA_ENGINE_STREAMTYPE_FAILED_VIDEO	= 2
    } 	MF_MEDIA_ENGINE_STREAMTYPE_FAILED;

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region EME2






#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_GUID(MF_MEDIA_ENGINE_EME_CALLBACK,
0x494553a7, 0xa481, 0x4cb7, 0xbe, 0xc5, 0x38, 0x09, 0x03, 0x51, 0x37, 0x31);


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0047_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0047_v0_0_s_ifspec;

#ifndef __IMFMediaEngineEMENotify_INTERFACE_DEFINED__
#define __IMFMediaEngineEMENotify_INTERFACE_DEFINED__

/* interface IMFMediaEngineEMENotify */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineEMENotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9e184d15-cdb7-4f86-b49e-566689f4a601")
    IMFMediaEngineEMENotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE Encrypted( 
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *pbInitData,
            /* [annotation] */ 
            _In_  DWORD cb,
            /* [annotation] */ 
            _In_  BSTR bstrInitDataType) = 0;
        
        virtual void STDMETHODCALLTYPE WaitingForKey( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineEMENotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineEMENotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineEMENotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineEMENotify * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEMENotify, Encrypted)
        void ( STDMETHODCALLTYPE *Encrypted )( 
            IMFMediaEngineEMENotify * This,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *pbInitData,
            /* [annotation] */ 
            _In_  DWORD cb,
            /* [annotation] */ 
            _In_  BSTR bstrInitDataType);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineEMENotify, WaitingForKey)
        void ( STDMETHODCALLTYPE *WaitingForKey )( 
            IMFMediaEngineEMENotify * This);
        
        END_INTERFACE
    } IMFMediaEngineEMENotifyVtbl;

    interface IMFMediaEngineEMENotify
    {
        CONST_VTBL struct IMFMediaEngineEMENotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineEMENotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineEMENotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineEMENotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineEMENotify_Encrypted(This,pbInitData,cb,bstrInitDataType)	\
    ( (This)->lpVtbl -> Encrypted(This,pbInitData,cb,bstrInitDataType) ) 

#define IMFMediaEngineEMENotify_WaitingForKey(This)	\
    ( (This)->lpVtbl -> WaitingForKey(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineEMENotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0048 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
typedef 
enum MF_MEDIAKEYS_REQUIREMENT
    {
        MF_MEDIAKEYS_REQUIREMENT_REQUIRED	= 1,
        MF_MEDIAKEYS_REQUIREMENT_OPTIONAL	= 2,
        MF_MEDIAKEYS_REQUIREMENT_NOT_ALLOWED	= 3
    } 	MF_MEDIAKEYS_REQUIREMENT;

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0048_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0048_v0_0_s_ifspec;

#ifndef __IMFMediaKeySessionNotify2_INTERFACE_DEFINED__
#define __IMFMediaKeySessionNotify2_INTERFACE_DEFINED__

/* interface IMFMediaKeySessionNotify2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeySessionNotify2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c3a9e92a-da88-46b0-a110-6cf953026cb9")
    IMFMediaKeySessionNotify2 : public IMFMediaKeySessionNotify
    {
    public:
        virtual void STDMETHODCALLTYPE KeyMessage2( 
            /* [annotation] */ 
            _In_  MF_MEDIAKEYSESSION_MESSAGETYPE eMessageType,
            /* [annotation] */ 
            _In_opt_  BSTR destinationURL,
            /* [annotation] */ 
            _In_reads_bytes_(cbMessage)  const BYTE *pbMessage,
            /* [annotation] */ 
            _In_  DWORD cbMessage) = 0;
        
        virtual void STDMETHODCALLTYPE KeyStatusChange( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeySessionNotify2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeySessionNotify2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeySessionNotify2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeySessionNotify2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify, KeyMessage)
        void ( STDMETHODCALLTYPE *KeyMessage )( 
            IMFMediaKeySessionNotify2 * This,
            /* [annotation] */ 
            _In_opt_  BSTR destinationURL,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *message,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify, KeyAdded)
        void ( STDMETHODCALLTYPE *KeyAdded )( 
            IMFMediaKeySessionNotify2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify, KeyError)
        void ( STDMETHODCALLTYPE *KeyError )( 
            IMFMediaKeySessionNotify2 * This,
            /* [annotation] */ 
            _In_  USHORT code,
            /* [annotation] */ 
            _In_  DWORD systemCode);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify2, KeyMessage2)
        void ( STDMETHODCALLTYPE *KeyMessage2 )( 
            IMFMediaKeySessionNotify2 * This,
            /* [annotation] */ 
            _In_  MF_MEDIAKEYSESSION_MESSAGETYPE eMessageType,
            /* [annotation] */ 
            _In_opt_  BSTR destinationURL,
            /* [annotation] */ 
            _In_reads_bytes_(cbMessage)  const BYTE *pbMessage,
            /* [annotation] */ 
            _In_  DWORD cbMessage);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySessionNotify2, KeyStatusChange)
        void ( STDMETHODCALLTYPE *KeyStatusChange )( 
            IMFMediaKeySessionNotify2 * This);
        
        END_INTERFACE
    } IMFMediaKeySessionNotify2Vtbl;

    interface IMFMediaKeySessionNotify2
    {
        CONST_VTBL struct IMFMediaKeySessionNotify2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeySessionNotify2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeySessionNotify2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeySessionNotify2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeySessionNotify2_KeyMessage(This,destinationURL,message,cb)	\
    ( (This)->lpVtbl -> KeyMessage(This,destinationURL,message,cb) ) 

#define IMFMediaKeySessionNotify2_KeyAdded(This)	\
    ( (This)->lpVtbl -> KeyAdded(This) ) 

#define IMFMediaKeySessionNotify2_KeyError(This,code,systemCode)	\
    ( (This)->lpVtbl -> KeyError(This,code,systemCode) ) 


#define IMFMediaKeySessionNotify2_KeyMessage2(This,eMessageType,destinationURL,pbMessage,cbMessage)	\
    ( (This)->lpVtbl -> KeyMessage2(This,eMessageType,destinationURL,pbMessage,cbMessage) ) 

#define IMFMediaKeySessionNotify2_KeyStatusChange(This)	\
    ( (This)->lpVtbl -> KeyStatusChange(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeySessionNotify2_INTERFACE_DEFINED__ */


#ifndef __IMFMediaKeySystemAccess_INTERFACE_DEFINED__
#define __IMFMediaKeySystemAccess_INTERFACE_DEFINED__

/* interface IMFMediaKeySystemAccess */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeySystemAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aec63fda-7a97-4944-b35c-6c6df8085cc3")
    IMFMediaKeySystemAccess : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateMediaKeys( 
            /* [annotation] */ 
            _In_opt_  IPropertyStore *pCdmCustomConfig,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeys2 **ppKeys) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_SupportedConfiguration( 
            /* [annotation] */ 
            _COM_Outptr_  IPropertyStore **ppSupportedConfiguration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_KeySystem( 
            /* [annotation] */ 
            _Out_  BSTR *pKeySystem) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeySystemAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeySystemAccess * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeySystemAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeySystemAccess * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySystemAccess, CreateMediaKeys)
        HRESULT ( STDMETHODCALLTYPE *CreateMediaKeys )( 
            IMFMediaKeySystemAccess * This,
            /* [annotation] */ 
            _In_opt_  IPropertyStore *pCdmCustomConfig,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeys2 **ppKeys);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySystemAccess, get_SupportedConfiguration)
        HRESULT ( STDMETHODCALLTYPE *get_SupportedConfiguration )( 
            IMFMediaKeySystemAccess * This,
            /* [annotation] */ 
            _COM_Outptr_  IPropertyStore **ppSupportedConfiguration);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySystemAccess, get_KeySystem)
        HRESULT ( STDMETHODCALLTYPE *get_KeySystem )( 
            IMFMediaKeySystemAccess * This,
            /* [annotation] */ 
            _Out_  BSTR *pKeySystem);
        
        END_INTERFACE
    } IMFMediaKeySystemAccessVtbl;

    interface IMFMediaKeySystemAccess
    {
        CONST_VTBL struct IMFMediaKeySystemAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeySystemAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeySystemAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeySystemAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeySystemAccess_CreateMediaKeys(This,pCdmCustomConfig,ppKeys)	\
    ( (This)->lpVtbl -> CreateMediaKeys(This,pCdmCustomConfig,ppKeys) ) 

#define IMFMediaKeySystemAccess_get_SupportedConfiguration(This,ppSupportedConfiguration)	\
    ( (This)->lpVtbl -> get_SupportedConfiguration(This,ppSupportedConfiguration) ) 

#define IMFMediaKeySystemAccess_get_KeySystem(This,pKeySystem)	\
    ( (This)->lpVtbl -> get_KeySystem(This,pKeySystem) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeySystemAccess_INTERFACE_DEFINED__ */


#ifndef __IMFMediaEngineClassFactory3_INTERFACE_DEFINED__
#define __IMFMediaEngineClassFactory3_INTERFACE_DEFINED__

/* interface IMFMediaEngineClassFactory3 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineClassFactory3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3787614f-65f7-4003-b673-ead8293a0e60")
    IMFMediaEngineClassFactory3 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateMediaKeySystemAccess( 
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _In_count_(uSize)  IPropertyStore **ppSupportedConfigurationsArray,
            /* [annotation] */ 
            _In_  UINT uSize,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySystemAccess **ppKeyAccess) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineClassFactory3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineClassFactory3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineClassFactory3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineClassFactory3 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory3, CreateMediaKeySystemAccess)
        HRESULT ( STDMETHODCALLTYPE *CreateMediaKeySystemAccess )( 
            IMFMediaEngineClassFactory3 * This,
            /* [annotation] */ 
            _In_  BSTR keySystem,
            /* [annotation] */ 
            _In_count_(uSize)  IPropertyStore **ppSupportedConfigurationsArray,
            /* [annotation] */ 
            _In_  UINT uSize,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySystemAccess **ppKeyAccess);
        
        END_INTERFACE
    } IMFMediaEngineClassFactory3Vtbl;

    interface IMFMediaEngineClassFactory3
    {
        CONST_VTBL struct IMFMediaEngineClassFactory3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineClassFactory3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineClassFactory3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineClassFactory3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineClassFactory3_CreateMediaKeySystemAccess(This,keySystem,ppSupportedConfigurationsArray,uSize,ppKeyAccess)	\
    ( (This)->lpVtbl -> CreateMediaKeySystemAccess(This,keySystem,ppSupportedConfigurationsArray,uSize,ppKeyAccess) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineClassFactory3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0051 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_INITDATATYPES =     { { 0x497d231b, 0x4eb9, 0x4df0, { 0xb4, 0x74, 0xb9, 0xaf, 0xeb, 0x0a, 0xdf, 0x38 } }, PID_FIRST_USABLE+0x00000001 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_DISTINCTIVEID =     { { 0x7dc9c4a5, 0x12be, 0x497e, { 0x8b, 0xff, 0x9b, 0x60, 0xb2, 0xdc, 0x58, 0x45 } }, PID_FIRST_USABLE+0x00000002 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_PERSISTEDSTATE =    { { 0x5d4df6ae, 0x9af1, 0x4e3d, { 0x95, 0x5b, 0x0e, 0x4b, 0xd2, 0x2f, 0xed, 0xf0 } }, PID_FIRST_USABLE+0x00000003 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_AUDIOCAPABILITIES = { { 0x980fbb84, 0x297d, 0x4ea7, { 0x89, 0x5f, 0xbc, 0xf2, 0x8a, 0x46, 0x28, 0x81 } }, PID_FIRST_USABLE+0x00000004 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_VIDEOCAPABILITIES = { { 0xb172f83d, 0x30dd, 0x4c10, { 0x80, 0x06, 0xed, 0x53, 0xda, 0x4d, 0x3b, 0xdb } }, PID_FIRST_USABLE+0x00000005 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_LABEL =             { { 0x9eae270e, 0xb2d7, 0x4817, { 0xb8, 0x8f, 0x54, 0x00, 0x99, 0xf2, 0xef, 0x4e } }, PID_FIRST_USABLE+0x00000006 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_SESSIONTYPES =      { { 0x7623384f, 0x00f5, 0x4376, { 0x86, 0x98, 0x34, 0x58, 0xdb, 0x03, 0x0e, 0xd5 } }, PID_FIRST_USABLE+0x00000007 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_ROBUSTNESS =  { { 0x9d3d2b9e, 0x7023, 0x4944, { 0xa8, 0xf5, 0xec, 0xca, 0x52, 0xa4, 0x69, 0x90 } }, PID_FIRST_USABLE+0x00000001 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_CONTENTTYPE = { { 0x289fb1fc, 0xd9c4, 0x4cc7, { 0xb2, 0xbe, 0x97, 0x2b, 0x0e, 0x9b, 0x28, 0x3a } }, PID_FIRST_USABLE+0x00000002 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_CDM_INPRIVATESTOREPATH = { { 0xec305fd9, 0x039f, 0x4ac8, { 0x98, 0xda, 0xe7, 0x92, 0x1e, 0x00, 0x6a, 0x90 } }, PID_FIRST_USABLE+0x00000001 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MF_EME_CDM_STOREPATH =          { { 0xf795841e, 0x99f9, 0x44d7, { 0xaf, 0xc0, 0xd3, 0x09, 0xc0, 0x4c, 0x94, 0xab } }, PID_FIRST_USABLE+0x00000002 }; 
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0051_v0_0_s_ifspec;

#ifndef __IMFMediaKeys2_INTERFACE_DEFINED__
#define __IMFMediaKeys2_INTERFACE_DEFINED__

/* interface IMFMediaKeys2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeys2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("45892507-ad66-4de2-83a2-acbb13cd8d43")
    IMFMediaKeys2 : public IMFMediaKeys
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateSession2( 
            /* [annotation] */ 
            _In_  MF_MEDIAKEYSESSION_TYPE eSessionType,
            /* [annotation] */ 
            _In_  IMFMediaKeySessionNotify2 *pMFMediaKeySessionNotify2,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySession2 **ppSession) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetServerCertificate( 
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *pbServerCertificate,
            /* [annotation] */ 
            _In_  DWORD cb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDOMException( 
            /* [annotation] */ 
            _In_  HRESULT systemCode,
            /* [annotation] */ 
            _Out_  HRESULT *code) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeys2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeys2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeys2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeys2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, CreateSession)
        HRESULT ( STDMETHODCALLTYPE *CreateSession )( 
            IMFMediaKeys2 * This,
            /* [annotation] */ 
            _In_opt_  BSTR mimeType,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *initData,
            /* [annotation] */ 
            _In_opt_  DWORD cb,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cbCustomData)  const BYTE *customData,
            /* [annotation] */ 
            _In_  DWORD cbCustomData,
            /* [annotation] */ 
            _In_  IMFMediaKeySessionNotify *notify,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySession **ppSession);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, get_KeySystem)
        HRESULT ( STDMETHODCALLTYPE *get_KeySystem )( 
            IMFMediaKeys2 * This,
            /* [annotation] */ 
            _COM_Outptr_  BSTR *keySystem);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaKeys2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys, GetSuspendNotify)
        HRESULT ( STDMETHODCALLTYPE *GetSuspendNotify )( 
            IMFMediaKeys2 * This,
            /* [annotation] */ 
            _COM_Outptr_  IMFCdmSuspendNotify **notify);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys2, CreateSession2)
        HRESULT ( STDMETHODCALLTYPE *CreateSession2 )( 
            IMFMediaKeys2 * This,
            /* [annotation] */ 
            _In_  MF_MEDIAKEYSESSION_TYPE eSessionType,
            /* [annotation] */ 
            _In_  IMFMediaKeySessionNotify2 *pMFMediaKeySessionNotify2,
            /* [annotation] */ 
            _COM_Outptr_  IMFMediaKeySession2 **ppSession);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys2, SetServerCertificate)
        HRESULT ( STDMETHODCALLTYPE *SetServerCertificate )( 
            IMFMediaKeys2 * This,
            /* [annotation] */ 
            _In_reads_bytes_opt_(cb)  const BYTE *pbServerCertificate,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IMFMediaKeys2, GetDOMException)
        HRESULT ( STDMETHODCALLTYPE *GetDOMException )( 
            IMFMediaKeys2 * This,
            /* [annotation] */ 
            _In_  HRESULT systemCode,
            /* [annotation] */ 
            _Out_  HRESULT *code);
        
        END_INTERFACE
    } IMFMediaKeys2Vtbl;

    interface IMFMediaKeys2
    {
        CONST_VTBL struct IMFMediaKeys2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeys2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeys2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeys2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeys2_CreateSession(This,mimeType,initData,cb,customData,cbCustomData,notify,ppSession)	\
    ( (This)->lpVtbl -> CreateSession(This,mimeType,initData,cb,customData,cbCustomData,notify,ppSession) ) 

#define IMFMediaKeys2_get_KeySystem(This,keySystem)	\
    ( (This)->lpVtbl -> get_KeySystem(This,keySystem) ) 

#define IMFMediaKeys2_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFMediaKeys2_GetSuspendNotify(This,notify)	\
    ( (This)->lpVtbl -> GetSuspendNotify(This,notify) ) 


#define IMFMediaKeys2_CreateSession2(This,eSessionType,pMFMediaKeySessionNotify2,ppSession)	\
    ( (This)->lpVtbl -> CreateSession2(This,eSessionType,pMFMediaKeySessionNotify2,ppSession) ) 

#define IMFMediaKeys2_SetServerCertificate(This,pbServerCertificate,cb)	\
    ( (This)->lpVtbl -> SetServerCertificate(This,pbServerCertificate,cb) ) 

#define IMFMediaKeys2_GetDOMException(This,systemCode,code)	\
    ( (This)->lpVtbl -> GetDOMException(This,systemCode,code) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeys2_INTERFACE_DEFINED__ */


#ifndef __IMFMediaKeySession2_INTERFACE_DEFINED__
#define __IMFMediaKeySession2_INTERFACE_DEFINED__

/* interface IMFMediaKeySession2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaKeySession2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e9707e05-6d55-4636-b185-3de21210bd75")
    IMFMediaKeySession2 : public IMFMediaKeySession
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_KeyStatuses( 
            /* [annotation] */ 
            _Outptr_result_buffer_(*puSize)  MFMediaKeyStatus **pKeyStatusesArray,
            /* [annotation] */ 
            _Out_  UINT *puSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [annotation] */ 
            _In_  BSTR bstrSessionId,
            /* [annotation] */ 
            _Out_  BOOL *pfLoaded) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GenerateRequest( 
            /* [annotation] */ 
            _In_  BSTR initDataType,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *pbInitData,
            /* [annotation] */ 
            _In_  DWORD cb) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Expiration( 
            /* [annotation] */ 
            _Out_  double *dblExpiration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Remove( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaKeySession2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaKeySession2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaKeySession2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaKeySession2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, GetError)
        HRESULT ( STDMETHODCALLTYPE *GetError )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _Out_  USHORT *code,
            /* [annotation] */ 
            _Out_  DWORD *systemCode);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, get_KeySystem)
        HRESULT ( STDMETHODCALLTYPE *get_KeySystem )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _COM_Outptr_  BSTR *keySystem);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, get_SessionId)
        HRESULT ( STDMETHODCALLTYPE *get_SessionId )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _COM_Outptr_  BSTR *sessionId);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, Update)
        HRESULT ( STDMETHODCALLTYPE *Update )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *key,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IMFMediaKeySession2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession2, get_KeyStatuses)
        HRESULT ( STDMETHODCALLTYPE *get_KeyStatuses )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _Outptr_result_buffer_(*puSize)  MFMediaKeyStatus **pKeyStatusesArray,
            /* [annotation] */ 
            _Out_  UINT *puSize);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession2, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _In_  BSTR bstrSessionId,
            /* [annotation] */ 
            _Out_  BOOL *pfLoaded);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession2, GenerateRequest)
        HRESULT ( STDMETHODCALLTYPE *GenerateRequest )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _In_  BSTR initDataType,
            /* [annotation] */ 
            _In_reads_bytes_(cb)  const BYTE *pbInitData,
            /* [annotation] */ 
            _In_  DWORD cb);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession2, get_Expiration)
        HRESULT ( STDMETHODCALLTYPE *get_Expiration )( 
            IMFMediaKeySession2 * This,
            /* [annotation] */ 
            _Out_  double *dblExpiration);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession2, Remove)
        HRESULT ( STDMETHODCALLTYPE *Remove )( 
            IMFMediaKeySession2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaKeySession2, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaKeySession2 * This);
        
        END_INTERFACE
    } IMFMediaKeySession2Vtbl;

    interface IMFMediaKeySession2
    {
        CONST_VTBL struct IMFMediaKeySession2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaKeySession2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaKeySession2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaKeySession2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaKeySession2_GetError(This,code,systemCode)	\
    ( (This)->lpVtbl -> GetError(This,code,systemCode) ) 

#define IMFMediaKeySession2_get_KeySystem(This,keySystem)	\
    ( (This)->lpVtbl -> get_KeySystem(This,keySystem) ) 

#define IMFMediaKeySession2_get_SessionId(This,sessionId)	\
    ( (This)->lpVtbl -> get_SessionId(This,sessionId) ) 

#define IMFMediaKeySession2_Update(This,key,cb)	\
    ( (This)->lpVtbl -> Update(This,key,cb) ) 

#define IMFMediaKeySession2_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 


#define IMFMediaKeySession2_get_KeyStatuses(This,pKeyStatusesArray,puSize)	\
    ( (This)->lpVtbl -> get_KeyStatuses(This,pKeyStatusesArray,puSize) ) 

#define IMFMediaKeySession2_Load(This,bstrSessionId,pfLoaded)	\
    ( (This)->lpVtbl -> Load(This,bstrSessionId,pfLoaded) ) 

#define IMFMediaKeySession2_GenerateRequest(This,initDataType,pbInitData,cb)	\
    ( (This)->lpVtbl -> GenerateRequest(This,initDataType,pbInitData,cb) ) 

#define IMFMediaKeySession2_get_Expiration(This,dblExpiration)	\
    ( (This)->lpVtbl -> get_Expiration(This,dblExpiration) ) 

#define IMFMediaKeySession2_Remove(This)	\
    ( (This)->lpVtbl -> Remove(This) ) 

#define IMFMediaKeySession2_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaKeySession2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0053 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0053_v0_0_s_ifspec;

#ifndef __IMFMediaEngineClassFactory4_INTERFACE_DEFINED__
#define __IMFMediaEngineClassFactory4_INTERFACE_DEFINED__

/* interface IMFMediaEngineClassFactory4 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaEngineClassFactory4;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fbe256c1-43cf-4a9b-8cb8-ce8632a34186")
    IMFMediaEngineClassFactory4 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateContentDecryptionModuleFactory( 
            /* [annotation][in] */ 
            _In_  LPCWSTR keySystem,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  LPVOID *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaEngineClassFactory4Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaEngineClassFactory4 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaEngineClassFactory4 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaEngineClassFactory4 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEngineClassFactory4, CreateContentDecryptionModuleFactory)
        HRESULT ( STDMETHODCALLTYPE *CreateContentDecryptionModuleFactory )( 
            IMFMediaEngineClassFactory4 * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR keySystem,
            /* [annotation][in] */ 
            _In_  REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  LPVOID *ppvObject);
        
        END_INTERFACE
    } IMFMediaEngineClassFactory4Vtbl;

    interface IMFMediaEngineClassFactory4
    {
        CONST_VTBL struct IMFMediaEngineClassFactory4Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaEngineClassFactory4_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaEngineClassFactory4_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaEngineClassFactory4_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaEngineClassFactory4_CreateContentDecryptionModuleFactory(This,keySystem,riid,ppvObject)	\
    ( (This)->lpVtbl -> CreateContentDecryptionModuleFactory(This,keySystem,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaEngineClassFactory4_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfmediaengine_0000_0054 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0054_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfmediaengine_0000_0054_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


