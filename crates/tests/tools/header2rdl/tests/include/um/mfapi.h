#include <winapifamily.h>

//*@@@+++@@@@******************************************************************
//
// Microsoft Windows Media Foundation
// Copyright (C) Microsoft Corporation. All rights reserved.
//
//*@@@---@@@@******************************************************************
//

//
// MFAPI.h is the header containing the APIs for using the MF platform.
//

#pragma once
#if !defined(__MFAPI_H__)
#define __MFAPI_H__

#pragma pack(push, mfhrds)
#include <mfobjects.h>
#pragma pack(pop, mfhrds)

#include "mmreg.h"

#include <avrt.h>

#if !defined(MF_VERSION)

#if (WINVER >= _WIN32_WINNT_WIN7)

#define MF_SDK_VERSION 0x0002

#else // Vista

#define MF_SDK_VERSION 0x0001

#endif // (WINVER >= _WIN32_WINNT_WIN7)

#define MF_API_VERSION 0x0070 // This value is unused in the Win7 release and left at its Vista release value
#define MF_VERSION (MF_SDK_VERSION << 16 | MF_API_VERSION)

#endif //!defined(MF_VERSION)


#define MFSTARTUP_NOSOCKET 0x1
#define MFSTARTUP_LITE (MFSTARTUP_NOSOCKET)
#define MFSTARTUP_FULL 0

#if defined(__cplusplus)
extern "C" {
#endif

////////////////////////////////////////////////////////////////////////////////
///////////////////////////////   Startup/Shutdown  ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// Initializes the platform object.
// Must be called before using Media Foundation.
// A matching MFShutdown call must be made when the application is done using
// Media Foundation.
// The "Version" parameter should be set to MF_API_VERSION.
// Application should not call MFStartup / MFShutdown from workqueue threads
//
#if defined(__cplusplus)

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFStartup( ULONG Version, DWORD dwFlags = MFSTARTUP_FULL );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#else

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFStartup( ULONG Version, DWORD dwFlags );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// Shuts down the platform object.
// Releases all resources including threads.
// Application should call MFShutdown the same number of times as MFStartup
// Application should not call MFStartup / MFShutdown from workqueue threads
//
STDAPI MFShutdown();


////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////    Platform    ///////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// These functions can be used to keep the MF platform object in place.
// Every call to MFLockPlatform should have a matching call to MFUnlockPlatform
//
STDAPI MFLockPlatform();
STDAPI MFUnlockPlatform();

///////////////////////////////////////////////////////////////////////////////

//
// MF workitem functions
//
typedef unsigned __int64 MFWORKITEM_KEY;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFPutWorkItem(
            DWORD dwQueue,
            IMFAsyncCallback * pCallback,
            IUnknown * pState);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFPutWorkItem2(
            DWORD dwQueue,
            LONG Priority,
            _In_ IMFAsyncCallback * pCallback,
            _In_opt_ IUnknown * pState);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFPutWorkItemEx(
            DWORD dwQueue,
            IMFAsyncResult * pResult);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFPutWorkItemEx2(
            DWORD dwQueue,
            LONG Priority,
            _In_ IMFAsyncResult * pResult);

STDAPI MFPutWaitingWorkItem (
            HANDLE hEvent,
            LONG Priority,
            _In_ IMFAsyncResult * pResult,
            _Out_opt_ MFWORKITEM_KEY * pKey
            );

STDAPI MFAllocateSerialWorkQueue (
            _In_ DWORD dwWorkQueue,
            _Out_ OUT DWORD * pdwWorkQueue);
            
STDAPI MFScheduleWorkItemEx(
            IMFAsyncResult * pResult,
            INT64 Timeout,
            _Out_opt_ MFWORKITEM_KEY * pKey);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFScheduleWorkItem(
            IMFAsyncCallback * pCallback,
            IUnknown * pState,
            INT64 Timeout,
            _Out_opt_ MFWORKITEM_KEY * pKey);

//
//   The CancelWorkItem method is used by objects to cancel scheduled operation
//   Due to asynchronous nature of timers, application might still get a
//   timer callback after MFCancelWorkItem has returned.
//
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFCancelWorkItem(
            MFWORKITEM_KEY Key);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

///////////////////////////////////////////////////////////////////////////////

//
// MF periodic callbacks
//
STDAPI MFGetTimerPeriodicity(
            _Out_ DWORD * Periodicity);

typedef void (*MFPERIODICCALLBACK)(IUnknown* pContext);

STDAPI MFAddPeriodicCallback(
            MFPERIODICCALLBACK Callback,
            IUnknown * pContext,
            _Out_opt_ DWORD * pdwKey);

STDAPI MFRemovePeriodicCallback(
            DWORD dwKey);

///////////////////////////////////////////////////////////////////////////////

//
// MF work queues
//

#if (WINVER >= _WIN32_WINNT_WIN7)
//
// MFASYNC_WORKQUEUE_TYPE: types of work queue used by MFAllocateWorkQueueEx
//
typedef enum
{
    // MF_STANDARD_WORKQUEUE: Work queue in a thread without Window 
    // message loop.
    MF_STANDARD_WORKQUEUE = 0,

    // MF_WINDOW_WORKQUEUE: Work queue in a thread running Window 
    // Message loop that calls PeekMessage() / DispatchMessage()..
    MF_WINDOW_WORKQUEUE = 1,

    //
    //
    MF_MULTITHREADED_WORKQUEUE = 2, // common MT threadpool
}   MFASYNC_WORKQUEUE_TYPE;

STDAPI MFAllocateWorkQueueEx(
            _In_ MFASYNC_WORKQUEUE_TYPE WorkQueueType,
            _Out_ OUT DWORD * pdwWorkQueue);
#endif // (WINVER >= _WIN32_WINNT_WIN7)

//
// Allocate a standard work queue. the behaviour is the same with:
// MFAllocateWorkQueueEx( MF_STANDARD_WORKQUEUE, pdwWorkQueue )
//
STDAPI MFAllocateWorkQueue(
            _Out_ OUT DWORD * pdwWorkQueue);


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFLockWorkQueue(
            _In_ DWORD dwWorkQueue);

STDAPI MFUnlockWorkQueue(
            _In_ DWORD dwWorkQueue);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFBeginRegisterWorkQueueWithMMCSS(
            DWORD dwWorkQueueId,
            _In_ LPCWSTR wszClass,
            DWORD dwTaskId,
            _In_ IMFAsyncCallback * pDoneCallback,
            _In_ IUnknown * pDoneState );

STDAPI MFBeginRegisterWorkQueueWithMMCSSEx(
            DWORD dwWorkQueueId,
            _In_ LPCWSTR wszClass,
            DWORD dwTaskId,
            LONG lPriority,
            _In_ IMFAsyncCallback * pDoneCallback,
            _In_ IUnknown * pDoneState );

STDAPI MFEndRegisterWorkQueueWithMMCSS(
            _In_ IMFAsyncResult * pResult,
            _Out_ DWORD * pdwTaskId );

STDAPI MFBeginUnregisterWorkQueueWithMMCSS(
            DWORD dwWorkQueueId,
            _In_ IMFAsyncCallback * pDoneCallback,
            _In_ IUnknown * pDoneState );

STDAPI MFEndUnregisterWorkQueueWithMMCSS(
            _In_ IMFAsyncResult * pResult );

STDAPI MFGetWorkQueueMMCSSClass(
            DWORD dwWorkQueueId,
            _Out_writes_to_opt_(*pcchClass,*pcchClass)  LPWSTR pwszClass,
            _Inout_  DWORD *pcchClass );

STDAPI MFGetWorkQueueMMCSSTaskId(
            DWORD dwWorkQueueId,
            _Out_ LPDWORD pdwTaskId );

STDAPI MFRegisterPlatformWithMMCSS(
    _In_ PCWSTR wszClass,
    _Inout_ DWORD* pdwTaskId,
    _In_ LONG lPriority );

STDAPI MFUnregisterPlatformFromMMCSS();

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFLockSharedWorkQueue(
    _In_ PCWSTR wszClass,
    _In_ LONG BasePriority,
    _Inout_ DWORD* pdwTaskId,
    _Out_ DWORD* pID );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFGetWorkQueueMMCSSPriority(
            DWORD dwWorkQueueId,
            _Out_ LONG* lPriority );


///////////////////////////////////////////////////////////////////////////////
/////////////////////////////////    Async Model //////////////////////////////
///////////////////////////////////////////////////////////////////////////////

//
// Instantiates the MF-provided Async Result implementation
//
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFCreateAsyncResult(
    IUnknown * punkObject,
    IMFAsyncCallback * pCallback,
    IUnknown * punkState,
    _Out_ IMFAsyncResult ** ppAsyncResult );

//
// Helper for calling IMFAsyncCallback::Invoke
//
STDAPI MFInvokeCallback(
    IMFAsyncResult * pAsyncResult );

//
// MFASYNCRESULT struct.
// Any implementation of IMFAsyncResult must inherit from this struct;
// the Media Foundation workqueue implementation depends on this.
//
#if defined(__cplusplus) && !defined(CINTERFACE)
typedef struct tagMFASYNCRESULT : public IMFAsyncResult
{
    OVERLAPPED overlapped;
    IMFAsyncCallback * pCallback;
    HRESULT hrStatusResult;
    DWORD dwBytesTransferred;
    HANDLE hEvent;
}   MFASYNCRESULT;
#else /* C style interface */
typedef struct tagMFASYNCRESULT
{
    IMFAsyncResult AsyncResult;
    OVERLAPPED overlapped;
    IMFAsyncCallback * pCallback;
    HRESULT hrStatusResult;
    DWORD dwBytesTransferred;
    HANDLE hEvent;
}   MFASYNCRESULT;
#endif /* C style interface */

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

///////////////////////////////////////////////////////////////////////////////
/////////////////////////////////    Files       //////////////////////////////
///////////////////////////////////////////////////////////////////////////////

//
// Regardless of the access mode with which the file is opened, the sharing
// permissions will allow shared reading and deleting.
//
STDAPI MFCreateFile(
    MF_FILE_ACCESSMODE  AccessMode,
    MF_FILE_OPENMODE    OpenMode,
    MF_FILE_FLAGS       fFlags,
    LPCWSTR             pwszFileURL,
    _Out_ IMFByteStream       **ppIByteStream );

STDAPI MFCreateTempFile(
    MF_FILE_ACCESSMODE  AccessMode,
    MF_FILE_OPENMODE    OpenMode,
    MF_FILE_FLAGS       fFlags,
    _Out_ IMFByteStream       **ppIByteStream );

STDAPI MFBeginCreateFile(
    MF_FILE_ACCESSMODE  AccessMode,
    MF_FILE_OPENMODE    OpenMode,
    MF_FILE_FLAGS       fFlags,
    LPCWSTR             pwszFilePath,
    IMFAsyncCallback *  pCallback,
    IUnknown *          pState,
    _Out_ IUnknown ** ppCancelCookie);

STDAPI MFEndCreateFile(
    IMFAsyncResult * pResult,
    _Out_ IMFByteStream **ppFile );

STDAPI MFCancelCreateFile(
    IUnknown * pCancelCookie);


///////////////////////////////////////////////////////////////////////////////
/////////////////////////////////    Buffers     //////////////////////////////
///////////////////////////////////////////////////////////////////////////////

//
// Creates an IMFMediaBuffer in memory
//
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFCreateMemoryBuffer(
    _In_ DWORD                      cbMaxLength,
    _Out_ IMFMediaBuffer **         ppBuffer );

//
// Creates an IMFMediaBuffer wrapper at the given offset and length
// within an existing IMFMediaBuffer
//
STDAPI MFCreateMediaBufferWrapper(
    _In_ IMFMediaBuffer *           pBuffer,
    _In_ DWORD                      cbOffset,
    _In_ DWORD                      dwLength,
    _Out_ IMFMediaBuffer **         ppBuffer );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// Creates a legacy buffer (IMediaBuffer) wrapper at the given offset within
// an existing IMFMediaBuffer.
// pSample is optional.  It can point to the original IMFSample from which this
// IMFMediaBuffer came.  If provided, then *ppMediaBuffer will succeed
// QueryInterface for IID_IMFSample, from which the original sample's attributes
// can be obtained
//
STDAPI MFCreateLegacyMediaBufferOnMFMediaBuffer(
    _In_opt_ IMFSample *            pSample,
    _In_ IMFMediaBuffer *           pMFMediaBuffer,
    _In_ DWORD                      cbOffset,
    _Outptr_ IMediaBuffer **     ppMediaBuffer );

//
// Create a DirectX surface buffer
//
#include <dxgiformat.h>
STDAPI_(DXGI_FORMAT) MFMapDX9FormatToDXGIFormat( _In_ DWORD dx9 );
STDAPI_(DWORD) MFMapDXGIFormatToDX9Format( _In_ DXGI_FORMAT dx11 );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFLockDXGIDeviceManager(
    _Out_opt_ UINT* pResetToken,
    _Outptr_ IMFDXGIDeviceManager** ppManager
    );

STDAPI MFUnlockDXGIDeviceManager();
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFCreateDXSurfaceBuffer(
    _In_ REFIID                     riid,
    _In_ IUnknown *                 punkSurface,
    _In_ BOOL                       fBottomUpWhenLinear,
    _Outptr_ IMFMediaBuffer **   ppBuffer );

STDAPI MFCreateWICBitmapBuffer(
    _In_ REFIID                     riid,
    _In_ IUnknown *                 punkSurface,
    _Outptr_ IMFMediaBuffer **   ppBuffer
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI
MFCreateDXGISurfaceBuffer(
    _In_ REFIID riid,
    _In_ IUnknown* punkSurface,
    _In_ UINT uSubresourceIndex,
    _In_ BOOL fBottomUpWhenLinear,
    _Outptr_ IMFMediaBuffer** ppBuffer
    );

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

STDAPI MFCreateDXGICrossAdapterBuffer(
    _In_ REFIID riid,
    _In_ IUnknown            *punkDevice,
    _In_ IMFMediaType        *pMediaType,
    _In_ UINT                uSubresource,
    _COM_Outptr_ IMFMediaBuffer     **ppBuffer
    );

#endif /*(NTDDI_VERSION >= NTDDI_WIN11_GE)*/

STDAPI MFCreateVideoSampleAllocatorEx(
    _In_   REFIID riid,
    _Outptr_  void** ppSampleAllocator
    );

STDAPI
MFCreateDXGIDeviceManager(
    _Out_ UINT* resetToken,
    _Outptr_ IMFDXGIDeviceManager** ppDeviceManager
    );

#if (NTDDI_VERSION >= NTDDI_WIN11_GE)

//
// Get the D3D Device version in DXGIDeviceManager
//
STDAPI
MFGetDXGIDeviceManageMode(
    _In_ IUnknown* pDeviceManager,
    _Out_ MF_DXGI_DEVICE_MANAGER_MODE* mode
);
#endif /*(NTDDI_VERSION >= NTDDI_WIN11_GE)*/

#define MF_E_DXGI_DEVICE_NOT_INITIALIZED ((HRESULT)0x80041000L)  // DXVA2_E_NOT_INITIALIZED     
#define MF_E_DXGI_NEW_VIDEO_DEVICE       ((HRESULT)0x80041001L)  // DXVA2_E_NEW_VIDEO_DEVICE    
#define MF_E_DXGI_VIDEO_DEVICE_LOCKED    ((HRESULT)0x80041002L)  // DXVA2_E_VIDEO_DEVICE_LOCKED 

//
// Create an aligned memory buffer.
// The following constants were chosen for parity with the alignment constants
// in ntioapi.h
// 
#define MF_1_BYTE_ALIGNMENT       0x00000000 
#define MF_2_BYTE_ALIGNMENT       0x00000001
#define MF_4_BYTE_ALIGNMENT       0x00000003
#define MF_8_BYTE_ALIGNMENT       0x00000007 
#define MF_16_BYTE_ALIGNMENT      0x0000000f
#define MF_32_BYTE_ALIGNMENT      0x0000001f
#define MF_64_BYTE_ALIGNMENT      0x0000003f
#define MF_128_BYTE_ALIGNMENT     0x0000007f
#define MF_256_BYTE_ALIGNMENT     0x000000ff
#define MF_512_BYTE_ALIGNMENT     0x000001ff
#define MF_1024_BYTE_ALIGNMENT    0x000003ff
#define MF_2048_BYTE_ALIGNMENT    0x000007ff 
#define MF_4096_BYTE_ALIGNMENT    0x00000fff
#define MF_8192_BYTE_ALIGNMENT    0x00001fff

STDAPI MFCreateAlignedMemoryBuffer(
    _In_ DWORD                      cbMaxLength,
    _In_ DWORD                      cbAligment, 
    _Out_ IMFMediaBuffer **         ppBuffer );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// This GUID is used in IMFGetService::GetService calls to retrieve 
// interfaces from the buffer.  Its value is defined in evr.h
// 
EXTERN_C const GUID MR_BUFFER_SERVICE;

///////////////////////////////////////////////////////////////////////////////
/////////////////////////////////    Events      //////////////////////////////
///////////////////////////////////////////////////////////////////////////////

//
// Instantiates the MF-provided Media Event implementation.
//

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFCreateMediaEvent(
    _In_ MediaEventType met,
    _In_ REFGUID guidExtendedType,
    _In_ HRESULT hrStatus,
    _In_opt_ const PROPVARIANT * pvValue,
    _Out_ IMFMediaEvent ** ppEvent );

//
// Instantiates an object that implements IMFMediaEventQueue.
// Components that provide an IMFMediaEventGenerator can use this object
// internally to do their Media Event Generator work for them.
// IMFMediaEventGenerator calls should be forwarded to the similar call
// on this object's IMFMediaEventQueue interface (e.g. BeginGetEvent,
// EndGetEvent), and the various IMFMediaEventQueue::QueueEventXXX methods
// can be used to queue events that the caller will consume.
//
STDAPI MFCreateEventQueue(
    _Out_ IMFMediaEventQueue **ppMediaEventQueue );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// Event attributes
// Some of the common Media Foundation events have associated attributes
// that go in their IMFAttributes stores
//

//
// MESessionCapabilitiesChanged attributes
//

// MF_EVENT_SESSIONCAPS {7E5EBCD0-11B8-4abe-AFAD-10F6599A7F42}
// Type: UINT32
DEFINE_GUID(MF_EVENT_SESSIONCAPS,
0x7e5ebcd0, 0x11b8, 0x4abe, 0xaf, 0xad, 0x10, 0xf6, 0x59, 0x9a, 0x7f, 0x42);

// MF_EVENT_SESSIONCAPS_DELTA {7E5EBCD1-11B8-4abe-AFAD-10F6599A7F42}
// Type: UINT32
DEFINE_GUID(MF_EVENT_SESSIONCAPS_DELTA,
0x7e5ebcd1, 0x11b8, 0x4abe, 0xaf, 0xad, 0x10, 0xf6, 0x59, 0x9a, 0x7f, 0x42);

// Session capabilities bitflags
#define MFSESSIONCAP_START                  0x00000001
#define MFSESSIONCAP_SEEK                   0x00000002
#define MFSESSIONCAP_PAUSE                  0x00000004
#define MFSESSIONCAP_RATE_FORWARD           0x00000010
#define MFSESSIONCAP_RATE_REVERSE           0x00000020
#define MFSESSIONCAP_DOES_NOT_USE_NETWORK   0x00000040

//
// MESessionTopologyStatus attributes
//

// Possible values for MF_EVENT_TOPOLOGY_STATUS attribute.
//
// For a given topology, these status values will arrive via
// MESessionTopologyStatus in the order below.
//
// However, there are no guarantees about how these status values will be
// ordered between two consecutive topologies.  For example,
// MF_TOPOSTATUS_READY could arrive for topology n+1 before
// MF_TOPOSTATUS_ENDED arrives for topology n if the application called
// IMFMediaSession::SetTopology for topology n+1 well enough in advance of the
// end of topology n.  Conversely, if topology n ends before the application
// calls IMFMediaSession::SetTopology for topology n+1, then
// MF_TOPOSTATUS_ENDED will arrive for topology n before MF_TOPOSTATUS_READY
// arrives for topology n+1.
typedef enum
{
    // MF_TOPOSTATUS_INVALID: Invalid value; will not be sent
    MF_TOPOSTATUS_INVALID = 0,

    // MF_TOPOSTATUS_READY: The topology has been put in place and is
    // ready to start.  All GetService calls to the Media Session will use
    // this topology.
    MF_TOPOSTATUS_READY     = 100,

    // MF_TOPOSTATUS_STARTED_SOURCE: The Media Session has started to read
    // and process data from the Media Source(s) in this topology.
    MF_TOPOSTATUS_STARTED_SOURCE = 200,


#if (WINVER >= _WIN32_WINNT_WIN7)
    // MF_TOPOSTATUS_DYNAMIC_CHANGED: The topology has been dynamic changed
    // due to the format change.
    MF_TOPOSTATUS_DYNAMIC_CHANGED = 210,
#endif // (WINVER >= _WIN32_WINNT_WIN7) 

    // MF_TOPOSTATUS_SINK_SWITCHED: The Media Sinks in the pipeline have
    // switched from a previous topology to this topology.
    // Note that this status does not get sent for the first topology;
    // applications can assume that the sinks are playing the first
    // topology when they receive MESessionStarted.
    MF_TOPOSTATUS_SINK_SWITCHED = 300,
    
    // MF_TOPOSTATUS_ENDED: Playback of this topology is complete.
    // Before deleting this topology, however, the application should wait
    // for either MESessionEnded or the MF_TOPOSTATUS_STARTED_SOURCE status
    // on the next topology to ensure that the Media Session is no longer
    // using this topology.
    MF_TOPOSTATUS_ENDED = 400,

}   MF_TOPOSTATUS;

// MF_EVENT_TOPOLOGY_STATUS {30C5018D-9A53-454b-AD9E-6D5F8FA7C43B}
// Type: UINT32 {MF_TOPOLOGY_STATUS}
DEFINE_GUID(MF_EVENT_TOPOLOGY_STATUS,
0x30c5018d, 0x9a53, 0x454b, 0xad, 0x9e, 0x6d, 0x5f, 0x8f, 0xa7, 0xc4, 0x3b);

//
// MESessionNotifyPresentationTime attributes
//

// MF_EVENT_START_PRESENTATION_TIME {5AD914D0-9B45-4a8d-A2C0-81D1E50BFB07}
// Type: UINT64
DEFINE_GUID(MF_EVENT_START_PRESENTATION_TIME,
0x5ad914d0, 0x9b45, 0x4a8d, 0xa2, 0xc0, 0x81, 0xd1, 0xe5, 0xb, 0xfb, 0x7);

// MF_EVENT_PRESENTATION_TIME_OFFSET {5AD914D1-9B45-4a8d-A2C0-81D1E50BFB07}
// Type: UINT64
DEFINE_GUID(MF_EVENT_PRESENTATION_TIME_OFFSET,
0x5ad914d1, 0x9b45, 0x4a8d, 0xa2, 0xc0, 0x81, 0xd1, 0xe5, 0xb, 0xfb, 0x7);

// MF_EVENT_START_PRESENTATION_TIME_AT_OUTPUT {5AD914D2-9B45-4a8d-A2C0-81D1E50BFB07}
// Type: UINT64
DEFINE_GUID(MF_EVENT_START_PRESENTATION_TIME_AT_OUTPUT,
0x5ad914d2, 0x9b45, 0x4a8d, 0xa2, 0xc0, 0x81, 0xd1, 0xe5, 0xb, 0xfb, 0x7);

//

//
// MESourceStarted attributes
//

// MF_EVENT_SOURCE_FAKE_START {a8cc55a7-6b31-419f-845d-ffb351a2434b}
// Type: UINT32
DEFINE_GUID(MF_EVENT_SOURCE_FAKE_START,
0xa8cc55a7, 0x6b31, 0x419f, 0x84, 0x5d, 0xff, 0xb3, 0x51, 0xa2, 0x43, 0x4b);

// MF_EVENT_SOURCE_PROJECTSTART {a8cc55a8-6b31-419f-845d-ffb351a2434b}
// Type: UINT64
DEFINE_GUID(MF_EVENT_SOURCE_PROJECTSTART,
0xa8cc55a8, 0x6b31, 0x419f, 0x84, 0x5d, 0xff, 0xb3, 0x51, 0xa2, 0x43, 0x4b);

// MF_EVENT_SOURCE_ACTUAL_START {a8cc55a9-6b31-419f-845d-ffb351a2434b}
// Type: UINT64
DEFINE_GUID(MF_EVENT_SOURCE_ACTUAL_START,
0xa8cc55a9, 0x6b31, 0x419f, 0x84, 0x5d, 0xff, 0xb3, 0x51, 0xa2, 0x43, 0x4b);

//
// MEEndOfPresentationSegment attributes
//

// MF_EVENT_SOURCE_TOPOLOGY_CANCELED {DB62F650-9A5E-4704-ACF3-563BC6A73364}
// Type: UINT32
DEFINE_GUID(MF_EVENT_SOURCE_TOPOLOGY_CANCELED,
0xdb62f650, 0x9a5e, 0x4704, 0xac, 0xf3, 0x56, 0x3b, 0xc6, 0xa7, 0x33, 0x64);

//
// MESourceCharacteristicsChanged attributes
//

// MF_EVENT_SOURCE_CHARACTERISTICS {47DB8490-8B22-4f52-AFDA-9CE1B2D3CFA8}
// Type: UINT32
DEFINE_GUID(MF_EVENT_SOURCE_CHARACTERISTICS,
0x47db8490, 0x8b22, 0x4f52, 0xaf, 0xda, 0x9c, 0xe1, 0xb2, 0xd3, 0xcf, 0xa8);

// MF_EVENT_SOURCE_CHARACTERISTICS_OLD {47DB8491-8B22-4f52-AFDA-9CE1B2D3CFA8}
// Type: UINT32
DEFINE_GUID(MF_EVENT_SOURCE_CHARACTERISTICS_OLD,
0x47db8491, 0x8b22, 0x4f52, 0xaf, 0xda, 0x9c, 0xe1, 0xb2, 0xd3, 0xcf, 0xa8);

//
// MESourceRateChangeRequested attributes
//

// MF_EVENT_DO_THINNING {321EA6FB-DAD9-46e4-B31D-D2EAE7090E30}
// Type: UINT32
DEFINE_GUID(MF_EVENT_DO_THINNING,
0x321ea6fb, 0xdad9, 0x46e4, 0xb3, 0x1d, 0xd2, 0xea, 0xe7, 0x9, 0xe, 0x30);

//
// MEStreamSinkScrubSampleComplete attributes
//

// MF_EVENT_SCRUBSAMPLE_TIME {9AC712B3-DCB8-44d5-8D0C-37455A2782E3}
// Type: UINT64
DEFINE_GUID(MF_EVENT_SCRUBSAMPLE_TIME,
0x9ac712b3, 0xdcb8, 0x44d5, 0x8d, 0xc, 0x37, 0x45, 0x5a, 0x27, 0x82, 0xe3);

//
// MESinkInvalidated and MESessionStreamSinkFormatChanged attributes
//

// MF_EVENT_OUTPUT_NODE {830f1a8b-c060-46dd-a801-1c95dec9b107}
// Type: UINT64
DEFINE_GUID(MF_EVENT_OUTPUT_NODE,
0x830f1a8b, 0xc060, 0x46dd, 0xa8, 0x01, 0x1c, 0x95, 0xde, 0xc9, 0xb1, 0x07);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

#if (WINVER >= _WIN32_WINNT_WIN7)
//
// METransformNeedInput attributes
// 

// MF_EVENT_MFT_INPUT_STREAM_ID {F29C2CCA-7AE6-42d2-B284-BF837CC874E2}
// Type: UINT32
DEFINE_GUID(MF_EVENT_MFT_INPUT_STREAM_ID, 
0xf29c2cca, 0x7ae6, 0x42d2, 0xb2, 0x84, 0xbf, 0x83, 0x7c, 0xc8, 0x74, 0xe2);

//
// METransformDrainComplete and METransformMarker attributes
//

// MF_EVENT_MFT_CONTEXT {B7CD31F1-899E-4b41-80C9-26A896D32977}
// Type: UINT64
DEFINE_GUID(MF_EVENT_MFT_CONTEXT, 
0xb7cd31f1, 0x899e, 0x4b41, 0x80, 0xc9, 0x26, 0xa8, 0x96, 0xd3, 0x29, 0x77);

#endif // (WINVER >= _WIN32_WINNT_WIN7)

#if (WINVER >= _WIN32_WINNT_WINBLUE)
//
// MEContentProtectionMetadata attributes
// 

// MF_EVENT_STREAM_METADATA_KEYDATA {CD59A4A1-4A3B-4BBD-8665-72A40FBEA776}
// Type: BLOB
DEFINE_GUID(MF_EVENT_STREAM_METADATA_KEYDATA, 
0xcd59a4a1, 0x4a3b, 0x4bbd, 0x86, 0x65, 0x72, 0xa4, 0xf, 0xbe, 0xa7, 0x76);

// MF_EVENT_STREAM_METADATA_CONTENT_KEYIDS {5063449D-CC29-4FC6-A75A-D247B35AF85C}
// Type: BLOB
DEFINE_GUID(MF_EVENT_STREAM_METADATA_CONTENT_KEYIDS, 
0x5063449d, 0xcc29, 0x4fc6, 0xa7, 0x5a, 0xd2, 0x47, 0xb3, 0x5a, 0xf8, 0x5c);

// MF_EVENT_STREAM_METADATA_SYSTEMID {1EA2EF64-BA16-4A36-8719-FE7560BA32AD}
// Type: BLOB
DEFINE_GUID(MF_EVENT_STREAM_METADATA_SYSTEMID, 
0x1ea2ef64, 0xba16, 0x4a36, 0x87, 0x19, 0xfe, 0x75, 0x60, 0xba, 0x32, 0xad);



#endif // (WINVER >= _WIN32_WINNT_WINBLUE)

////////////////////////////////////////////////////////////////////////////////
///////////////////////////////  Samples  //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// Creates an instance of the Media Foundation implementation of IMFSample
//



STDAPI MFCreateSample( _Out_ IMFSample **ppIMFSample );


//
// Sample attributes
// These are the well-known attributes that can be present on an MF Sample's
// IMFAttributes store
//

//@@MFSampleExtension_MaxDecodeFrameSize  
/// <summary> 
// {D3CC654F-F9F3-4A13-889F-F04EB2B5B957} MFSampleExtension_MaxDecodeFrameSize                {UINT64 (HI32(Width),LO32(Height))}
// specify the maxiumum resolution of compressed input bitstream, 
// the decoder shall decode any comressed pictures below the specified maximum resolution
// any input compressed pictures beyond the maximum resolution shall not be decoded and dropped by the decoder
// the attribute shall be set on input sample
/// </summary>  
DEFINE_GUID(MFSampleExtension_MaxDecodeFrameSize,
    0xd3cc654f, 0xf9f3, 0x4a13, 0x88, 0x9f, 0xf0, 0x4e, 0xb2, 0xb5, 0xb9, 0x57);


//@@MFSampleExtension_AccumulatedNonRefPicPercent  
/// <summary> 
// {79EA74DF-A740-445B-BC98-C9ED1F260EEE} MFSampleExtension_AccumulatedNonRefPicPercent               
// Type: UINT32 
// specify the percentage of accumulated non-reference pictures up to this output sample in decoding order
// The most common examples are,
// 1. if the sequence has the GOP structure of IPPPP......,   the value will be 0
// 2. if the sequence has the GOP structure of IPBPB......,   the percentage will be around 40%~50%. The value is 40~50.
// 3. if the sequence has the GOP structure of IPBBPBB......, the percentage will be around 50%~66%. The value is 50~60. 
// where B frames are not used for reference. 
// This is some statistic to application or pipeline whether decoder alone can have graceful degradation on quality management
// In the above example, 
// 1. Decoder alone can't have graceful quality management. Because it can only have full frame rate or 1/15 of full frame rate when GOP size is 15 frames or 1/30 when GOP size is 30 frames
// 2. Decoder alone can   have quality management. Because it can have full frame rate or 1/2 of full frame rate or 1/GOPSize
// 2. Decoder alone can   have quality management. Because it can have full frame rate,  or down to 1/3 of full frame rate or 1/GOPSize
// the attribute could be set on output sample from decoders
/// </summary>  
// {79EA74DF-A740-445B-BC98-C9ED1F260EEE}
DEFINE_GUID(MFSampleExtension_AccumulatedNonRefPicPercent,
    0x79ea74df, 0xa740, 0x445b, 0xbc, 0x98, 0xc9, 0xed, 0x1f, 0x26, 0xe, 0xee);

////////////////////////////////////////////////////////////////////////////////
// Sample extensions for SAMPLE-AES encryption

// MFSampleExtension_Encryption_ProtectionScheme {D054D096-28BB-45DA-87EC-74F351871406}
// Type: UINT32
// Specifies the cipher and mode used to encrypt the content
DEFINE_GUID(MFSampleExtension_Encryption_ProtectionScheme,
    0xd054d096, 0x28bb, 0x45da, 0x87, 0xec, 0x74, 0xf3, 0x51, 0x87, 0x14, 0x6);


typedef enum _MFSampleEncryptionProtectionScheme
{
    MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_NONE = 0,
    MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CTR = 1,
    MF_SAMPLE_ENCRYPTION_PROTECTION_SCHEME_AES_CBC = 2,
} MFSampleEncryptionProtectionScheme;



// MFSampleExtension_Encryption_CryptByteBlock {9D84289B-0C7F-4713-AB95-108AB42AD801}
// Type: UINT32
// Represents the number of encrypted blocks in the protection pattern, where each block is 16 bytes.
DEFINE_GUID(MFSampleExtension_Encryption_CryptByteBlock,
    0x9d84289b, 0xc7f, 0x4713, 0xab, 0x95, 0x10, 0x8a, 0xb4, 0x2a, 0xd8, 0x1);

// MFSampleExtension_Encryption_SkipByteBlock {0D550548-8317-4AB1-845F-D06306E293E3}
// Type: UINT32
// Represents the number of unencrypted blocks in the protection pattern, where each block is 16 bytes.
DEFINE_GUID(MFSampleExtension_Encryption_SkipByteBlock,
    0xd550548, 0x8317, 0x4ab1, 0x84, 0x5f, 0xd0, 0x63, 0x6, 0xe2, 0x93, 0xe3);

////////////////////////////////////////////////////////////////////////////////

// Attributes for HW-DRM support

//@@MFSampleExtension_Encryption_SubSample_Mapping  
/// <summary> 
/// The data blob associated with this attribute should contain an array of byte  
/// ranges as DWORDs where every two DWORDs make a set. The first DWORD in each set  
/// is the number of clear bytes and the second DWORD of the set is the number of  
/// encrypted bytes.  
/// Note that a pair of 0s is not a valid set (either value can be 0, but not both).  
/// The array of byte ranges that indicate which ranges to decrypt, including the  
/// possibility that the entire sample should NOT be decrypted.  
/// It must be set on an IMFSample using SetBlob  
/// </summary>  
DEFINE_GUID(MFSampleExtension_Encryption_SubSample_Mapping,
    0x8444F27A, 0x69A1, 0x48DA, 0xBD, 0x08, 0x11, 0xCE, 0xF3, 0x68, 0x30, 0xD2);


// MFSampleExtension_Encryption_ClearSliceHeaderData {5509A4F4-320D-4E6C-8D1A-94C66DD20CB0}
/*
The MF blob should be parsed in the way below defined in SliceHeaderSet, with proper verifications

=============================================================================================================
Note the slice header data here DO NOT have all bits for all the syntaxes.
Some bits are removed on purpose to send out a lossy compressed slice header in order to be 100% secure 
The partial slice header data here SHALL not include any bits for emulation prevention byte 0x03
=============================================================================================================

typedef struct SliceHeader_tag {
WORD dSliceHeaderLen;                   // indicate the length of the following slice header in byte, it shall not be more than 1024
BYTE SliceHeaderBytes[0];               // slice header data, the last byte might contain some bits not used, leave them random
} SliceHeader;

With dSliceHeaderLen bytes serialized after the SliceHeader struct. 
And then use an array of these serialized consecutively,

typedef struct SliceHeaderSet_tag {
WORD dNumHeaders;                       // indicate the number of slice headers in the input sample
SliceHeader rgstSliceheader[0];         // cNumHeaders slice header data
} SliceHeaderSet;
*/
// Type: BLOB
DEFINE_GUID(MFSampleExtension_Encryption_ClearSliceHeaderData,
    0x5509a4f4, 0x320d, 0x4e6c, 0x8d, 0x1a, 0x94, 0xc6, 0x6d, 0xd2, 0xc, 0xb0);


// MFSampleExtension_Encryption_HardwareProtection_KeyInfoID {8CBFCCEB-94A5-4DE1-8231-A85E47CF81E7}
// Type: GUID
// This attribute applies to media samples. The GUID associated with this 
// attribute indicates an identifier (KID/LID) for the hardware protection to be
// used for the given sample. All hardware protected samples flowing out of the 
// MFT decryptor should have this attribute set with the proper GUID.
DEFINE_GUID(MFSampleExtension_Encryption_HardwareProtection_KeyInfoID, 
0x8cbfcceb, 0x94a5, 0x4de1, 0x82, 0x31, 0xa8, 0x5e, 0x47, 0xcf, 0x81, 0xe7);


// MFSampleExtension_Encryption_HardwareProtection_KeyInfo {B2372080-455B-4DD7-9989-1A955784B754}
// Type: BLOB
// This attribute applies to media samples. The data blob associated with this 
// sample has all the information relative to the slot/ID for the hardware 
// protection to be used for the given sample. All hardware protected samples 
// flowing out of the MFT decryptor should have this attribute set with the 
// proper blob.
DEFINE_GUID(MFSampleExtension_Encryption_HardwareProtection_KeyInfo, 
0xb2372080, 0x455b, 0x4dd7, 0x99, 0x89, 0x1a, 0x95, 0x57, 0x84, 0xb7, 0x54);

// MFSampleExtension_Encryption_HardwareProtection_VideoDecryptorContext {693470C8-E837-47A0-88CB-535B905E3582}
// Data type: IUnknown * (IMFContentDecryptorContext)
// This attribute applies to media samples. It associates a sample with a 
// given IMFContentDecryptorContext which is needed to be able to to
// decrypt/decode the sample properly when using hardware protection.
DEFINE_GUID(MFSampleExtension_Encryption_HardwareProtection_VideoDecryptorContext, 
0x693470c8, 0xe837, 0x47a0, 0x88, 0xcb, 0x53, 0x5b, 0x90, 0x5e, 0x35, 0x82);


// MFSampleExtension_Encryption_Opaque_Data {224D77E5-1391-4FFB-9F41-B432F68C611D}
// Data type : BLOB
// This attribute applies to media samples.The data blob associated with this sample has some private information 
// set by OEM secure environment to be used for the given sample.The hardware protected samples flowing out of the 
// MFT decryptor might have this attribute set with the proper blob.
// When present, this attribute is set by the decryptor MFT with data that originates from the OEM secure environment.
// The host decoder may extract this and provide the data to the D3D11 device for VLD decoding through(UINT  PrivateDataSize, void* pPrivateData) 
// of D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION data structure in the DecoderBeginFrame() call, when present.
DEFINE_GUID(MFSampleExtension_Encryption_Opaque_Data,
    0x224d77e5, 0x1391, 0x4ffb, 0x9f, 0x41, 0xb4, 0x32, 0xf6, 0x8c, 0x61, 0x1d);


// MFSampleExtension_NALULengthInfo. This is an alias of  MF_NALU_LENGTH_INFORMATION
// Type: BLOB
// Set MFSampleExtension_NALULengthInfo as a BLOB on the input sample, 
// with one DWORD for each NALU including start code and NALU type in the sample. For example, if 
// there are AUD (9 bytes), SPS (25 bytes), PPS (10 bytes), IDR slice1 (50 k), IDR slice 2 (60 k), 
// then there should be 5 DWORDs with values 9, 25, 10, 50 k, 60 k in the BLOB.
//
DEFINE_GUID(MFSampleExtension_NALULengthInfo,
    0x19124E7C, 0xAD4B, 0x465F, 0xBB, 0x18, 0x20, 0x18, 0x62, 0x87, 0xB6, 0xAF);



// MFSampleExtension_Encryption_ResumeVideoOutput. {A435ABA5-AFDE-4CF5-BC1C-F6ACAF13949D}
// Type: UINT32
//
// This attribute shall be used in hardware DRM scenario only
// it is set on input compressed sample to (H.264/HEVC) video decoder
//
// when present, it indicates video output in video render should resume on the first output (uncompressed) sample 
// with the attribute MFSampleExtension_Encryption_ResumeVideoOutput set to true 
//
// note: (H.264/HEVC) video decoder should buffer the attribute when video decoder 
// detects the attribute set to true on some input sample, which might be dropped since 
// those input sample might not be decode-able because of missing references,
// and set the attribute to true on the first output sample not dropped in video decoder
// 
DEFINE_GUID(MFSampleExtension_Encryption_ResumeVideoOutput,
    0xa435aba5, 0xafde, 0x4cf5, 0xbc, 0x1c, 0xf6, 0xac, 0xaf, 0x13, 0x94, 0x9d);



// MFSampleExtension_Encryption_NALUTypes. {B0F067C7-714C-416C-8D59-5F4DDF8913B6}
// Type: BLOB
// The MF blob contains all the NALU type byte for different NALUs in the MF sample.One NALU type is one byte, including the syntaxes forbidden_zero_bit, nal_ref_idc, and nal_unit_type.
DEFINE_GUID(MFSampleExtension_Encryption_NALUTypes,
    0xb0f067c7, 0x714c, 0x416c, 0x8d, 0x59, 0x5f, 0x4d, 0xdf, 0x89, 0x13, 0xb6);


// MFSampleExtension_Encryption_SPSPPSData {AEDE0FA2-0E0C-453C-B7F3-DE8693364D11}
// Type : BLOB
// When present, the MF blob contains all SPS(s) and / or PPS(s) NALUs inside the MF sample.
// SPSs and PPSs shall be present in the same order as that in the MF sample and in the format of AvcC, 
// which is DWORD, four - byte length inforamtion for the bytes followed, and NALU data of SPS or PPS, for each NALU.
// For example, the layout could be 10 in DWORD, 10 bytes data for SPS, 5 in DWORD, and 5 bytes data for PPS.In total, it has 4 + 10 + 4 + 5 = 23 bytes.
DEFINE_GUID(MFSampleExtension_Encryption_SPSPPSData,
    0xaede0fa2, 0xe0c, 0x453c, 0xb7, 0xf3, 0xde, 0x86, 0x93, 0x36, 0x4d, 0x11);



// MFSampleExtension_Encryption_SEIData {3CF0E972-4542-4687-9999-585F565FBA7D}
// Type : BLOB
// When present, the MF blob contains all SEI NALUs inside the MF sample. (If there are multiple SEIs in the protected MF sample, all the SEIs shall be present in the blob.)
// SEIs shall be present in the same order as that in the MF sample and in the format of AvcC, 
// which is DWORD, four - byte length inforamtion for the bytes followed, and NALU data of SEI.
// For example, the layout could be 10 in DWORD, 10 bytes data for the first SEI, 5 in DWORD, and 5 bytes data for the second SEI.In total, it has 4 + 10 + 4 + 5 = 23 bytes.
//
// Some note about  how to process the SEI NALUs in the blob of MFSampleExtension_Encryption_SEIData
// Decoder should verify every byte of an SEI NALU is clear, not protected, before parsing the SEI NALU
// otherwise, decoder should treat the SEI NALU as corrupted by encryption and skip the parsing of the SEI NALU
DEFINE_GUID(MFSampleExtension_Encryption_SEIData,
    0x3cf0e972, 0x4542, 0x4687, 0x99, 0x99, 0x58, 0x5f, 0x56, 0x5f, 0xba, 0x7d);


// MFSampleExtension_Encryption_HardwareProtection {9A2B2D2B-8270-43E3-8448-994F426E8886}
// Type: UINT32
// When present, this UINT32 attribute indicates whether the sample is hardware protected.
// 0 = not hardware protected, nonzero = hardware protected
DEFINE_GUID(MFSampleExtension_Encryption_HardwareProtection,
    0x9a2b2d2b, 0x8270, 0x43e3, 0x84, 0x48, 0x99, 0x4f, 0x42, 0x6e, 0x88, 0x86);


// MFSampleExtension_CleanPoint {9cdf01d8-a0f0-43ba-b077-eaa06cbd728a}
// Type: UINT32
// If present and nonzero, indicates that the sample is a clean point (key
// frame), and decoding can begin at this sample.
DEFINE_GUID(MFSampleExtension_CleanPoint,
0x9cdf01d8, 0xa0f0, 0x43ba, 0xb0, 0x77, 0xea, 0xa0, 0x6c, 0xbd, 0x72, 0x8a);

// MFSampleExtension_Discontinuity {9cdf01d9-a0f0-43ba-b077-eaa06cbd728a}
// Type: UINT32
// If present and nonzero, indicates that the sample data represents the first
// sample following a discontinuity (gap) in the stream of samples.
// This can happen, for instance, if the previous sample was lost in
// transmission.
DEFINE_GUID(MFSampleExtension_Discontinuity,
0x9cdf01d9, 0xa0f0, 0x43ba, 0xb0, 0x77, 0xea, 0xa0, 0x6c, 0xbd, 0x72, 0x8a);

// MFSampleExtension_Token {8294da66-f328-4805-b551-00deb4c57a61}
// Type: IUNKNOWN
// When an IMFMediaStream delivers a sample via MEMediaStream, this attribute
// should be set to the IUnknown *pToken argument that was passed with the
// IMFMediaStream::RequestSample call to which this sample corresponds.
DEFINE_GUID(MFSampleExtension_Token,
0x8294da66, 0xf328, 0x4805, 0xb5, 0x51, 0x00, 0xde, 0xb4, 0xc5, 0x7a, 0x61);

// MFSampleExtension_ClosedCaption_CEA708 {26f09068-e744-47dc-aa03-dbf20403bde6}  
// Type: BLOB  
// MF sample attribute contained the closed caption data in CEA-708 format.  
DEFINE_GUID(MFSampleExtension_ClosedCaption_CEA708,  0x26f09068, 0xe744, 0x47dc, 0xaa, 0x03, 0xdb, 0xf2, 0x04, 0x03, 0xbd, 0xe6);  
#define MFSampleExtension_ClosedCaption_CEA708_MAX_SIZE    256  

// MFSampleExtension_DecodeTimestamp {73A954D4-09E2-4861-BEFC-94BD97C08E6E}
// Type : UINT64
// If present, contains the DTS (Decoding Time Stamp) of the sample.
DEFINE_GUID(MFSampleExtension_DecodeTimestamp,
0x73a954d4, 0x9e2, 0x4861, 0xbe, 0xfc, 0x94, 0xbd, 0x97, 0xc0, 0x8e, 0x6e);

// MFSampleExtension_VideoEncodeQP {B2EFE478-F979-4C66-B95E-EE2B82C82F36}
// Type: UINT64
// Used by video encoders to specify the QP used to encode the output sample.
DEFINE_GUID(MFSampleExtension_VideoEncodeQP,
0xb2efe478, 0xf979, 0x4c66, 0xb9, 0x5e, 0xee, 0x2b, 0x82, 0xc8, 0x2f, 0x36);

// MFSampleExtension_VideoEncPictureType {973704E6-CD14-483C-8F20-C9FC0928BAD5}
// Type: UINT32
// Used by video encoders to specify the output sample's picture type.
DEFINE_GUID(MFSampleExtension_VideoEncodePictureType,
0x973704e6, 0xcd14, 0x483c, 0x8f, 0x20, 0xc9, 0xfc, 0x9, 0x28, 0xba, 0xd5);

// MFSampleExtension_FrameCorruption {B4DD4A8C-0BEB-44C4-8B75-B02B913B04F0}
// Type: UINT32
// Indicates whether the frame in the sample has corruption or not
// value 0 indicates that there is no corruption, or it is unknown
// Value 1 indicates that some corruption was detected e.g, during decoding
DEFINE_GUID(MFSampleExtension_FrameCorruption,
0xb4dd4a8c, 0xbeb, 0x44c4, 0x8b, 0x75, 0xb0, 0x2b, 0x91, 0x3b, 0x4, 0xf0);

#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD)

// MFSampleExtension_DirtyRects {9BA70225-B342-4E97-9126-0B566AB7EA7E}
// Type: BLOB
// This is a blob containing information about the dirty rectangles within
// a frame. The blob is a struct of type DIRTYRECT_INFO containing an array
// of NumDirtyRects number of DirtyRects elements.
DEFINE_GUID(MFSampleExtension_DirtyRects,
0x9ba70225, 0xb342, 0x4e97, 0x91, 0x26, 0x0b, 0x56, 0x6a, 0xb7, 0xea, 0x7e);

// MFSampleExtension_MoveRegions {E2A6C693-3A8B-4B8D-95D0-F60281A12FB7}
// Type: BLOB
// This is a blob containing information about the moved regions within
// a frame. The blob is a struct of type MOVEREGION_INFO containing an array
// of NumMoveRegions number of MoveRegions elements.
DEFINE_GUID(MFSampleExtension_MoveRegions,
0xe2a6c693, 0x3a8b, 0x4b8d, 0x95, 0xd0, 0xf6, 0x02, 0x81, 0xa1, 0x2f, 0xb7);

typedef struct _MOVE_RECT
{
    POINT   SourcePoint;
    RECT    DestRect;
} MOVE_RECT;

typedef struct _DIRTYRECT_INFO
{
    UINT        FrameNumber;
    UINT        NumDirtyRects;
    RECT        DirtyRects[1];
} DIRTYRECT_INFO;

typedef struct _MOVEREGION_INFO
{
    UINT        FrameNumber;
    UINT        NumMoveRegions;
    MOVE_RECT   MoveRegions[1];
} MOVEREGION_INFO;

// MFSampleExtension_HDCP_OptionalHeader
// Type: BLOB
// This blob contains LPCM header in front of LPCM sample in a PES packet. It is
// encrypted when HDCP 2.x frames are sent, and is needed for decryption.
DEFINE_GUID(MFSampleExtension_HDCP_OptionalHeader,
            0x9a2e7390, 0x121f, 0x455f, 0x83, 0x76, 0xc9, 0x74, 0x28, 0xe0, 0xb5, 0x40);

// MFSampleExtension_HDCP_FrameCounter
// Type: BLOB
// This blob contains the PES_private_data section of a PES packet according to the
// HDCP 2.2/2.1 specification.  This blob should contain the stream counter and
// input counter.
DEFINE_GUID(MFSampleExtension_HDCP_FrameCounter, 
            0x9d389c60, 0xf507, 0x4aa6, 0xa4, 0xa, 0x71, 0x2, 0x7a, 0x2, 0xf3, 0xde); 

// MFSampleExtension_HDCP_StreamID {177E5D74-C370-4A7A-95A2-36833C01D0AF}  
// Type: UINT32
// This UINT32 value is provided to the HDCP Encryptor MFT on each input sample.  
// The Stream ID value allows the HDCP Encryptor MFT to support time-multiplexed
// encryption of multiple independent streams.  An example is using 0 for first
// display video stream, 1 for second display video stream, 2 for first display audio
// stream, 3 for second display audio stream.
// Per the HDCP 2.2 specification, this value is referred to as streamCtr.  It is called
// StreamID here to be more intuitive.
DEFINE_GUID(MFSampleExtension_HDCP_StreamID,
            0x177e5d74, 0xc370, 0x4a7a, 0x95, 0xa2, 0x36, 0x83, 0x3c, 0x01, 0xd0, 0xaf);

// MFSampleExtension_Timestamp  
// Type: int64 
// { 1e436999-69be-4c7a-9369-70068c0260cb } MFSampleExtension_Timestamp  {INT64 }
// The timestamp of a sample
// 
DEFINE_GUID(MFSampleExtension_Timestamp,
    0x1e436999, 0x69be, 0x4c7a, 0x93, 0x69, 0x70, 0x06, 0x8c, 0x02, 0x60, 0xcb);

// MFSampleExtension_RepeatFrame {88BE738F-0711-4F42-B458-344AED42EC2F}
// Type: UINT32
// This UINT32 when set to 1 indicates that the frame is a repeat of the previous frame
DEFINE_GUID(MFSampleExtension_RepeatFrame,
    0x88be738f, 0x711, 0x4f42, 0xb4, 0x58, 0x34, 0x4a, 0xed, 0x42, 0xec, 0x2f);

// MFT_ENCODER_ERROR {C8D1EDA4-98E4-41D5-9297-44F53852F90E}
// Type: GUID 
// This is the GUID of a property that caused the encoder MFT to fail initialization 
DEFINE_GUID(MFT_ENCODER_ERROR,
    0xc8d1eda4, 0x98e4, 0x41d5, 0x92, 0x97, 0x44, 0xf5, 0x38, 0x52, 0xf9, 0x0e);

// MFT_GFX_DRIVER_VERSION_ID_Attribute {F34B9093-05E0-4B16-993D-3E2A2CDE6AD3}
// Type: WSTR
// For hardware MFTs, this attribute allows the HMFT to report the graphics driver version.
DEFINE_GUID(MFT_GFX_DRIVER_VERSION_ID_Attribute,
    0xf34b9093, 0x05e0, 0x4b16, 0x99, 0x3d, 0x3e, 0x2a, 0x2c, 0xde, 0x6a, 0xd3);

#endif

/////////////////////////////////////////////////////////////////////////////
//
// The following sample attributes are used for encrypted samples
//
/////////////////////////////////////////////////////////////////////////////

// MFSampleExtension_DescrambleData {43483BE6-4903-4314-B032-2951365936FC}
// Type: UINT64
DEFINE_GUID(MFSampleExtension_DescrambleData,
0x43483be6, 0x4903, 0x4314, 0xb0, 0x32, 0x29, 0x51, 0x36, 0x59, 0x36, 0xfc);

// MFSampleExtension_SampleKeyID {9ED713C8-9B87-4B26-8297-A93B0C5A8ACC}
// Type: UINT32
DEFINE_GUID(MFSampleExtension_SampleKeyID,
0x9ed713c8, 0x9b87, 0x4b26, 0x82, 0x97, 0xa9, 0x3b, 0x0c, 0x5a, 0x8a, 0xcc);

// MFSampleExtension_GenKeyFunc {441CA1EE-6B1F-4501-903A-DE87DF42F6ED}
// Type: UINT64
DEFINE_GUID(MFSampleExtension_GenKeyFunc,
0x441ca1ee, 0x6b1f, 0x4501, 0x90, 0x3a, 0xde, 0x87, 0xdf, 0x42, 0xf6, 0xed);

// MFSampleExtension_GenKeyCtx {188120CB-D7DA-4B59-9B3E-9252FD37301C}
// Type: UINT64
DEFINE_GUID(MFSampleExtension_GenKeyCtx,
0x188120cb, 0xd7da, 0x4b59, 0x9b, 0x3e, 0x92, 0x52, 0xfd, 0x37, 0x30, 0x1c);

// MFSampleExtension_PacketCrossOffsets {2789671D-389F-40BB-90D9-C282F77F9ABD}
// Type: BLOB
DEFINE_GUID(MFSampleExtension_PacketCrossOffsets,
0x2789671d, 0x389f, 0x40bb, 0x90, 0xd9, 0xc2, 0x82, 0xf7, 0x7f, 0x9a, 0xbd);

// MFSampleExtension_Encryption_SampleID {6698B84E-0AFA-4330-AEB2-1C0A98D7A44D}
// Type: BLOB
DEFINE_GUID(MFSampleExtension_Encryption_SampleID,
0x6698b84e, 0x0afa, 0x4330, 0xae, 0xb2, 0x1c, 0x0a, 0x98, 0xd7, 0xa4, 0x4d);

// MFSampleExtension_Encryption_KeyID {76376591-795F-4DA1-86ED-9D46ECA109A9}
// Type: BLOB
DEFINE_GUID(MFSampleExtension_Encryption_KeyID,
0x76376591, 0x795f, 0x4da1, 0x86, 0xed, 0x9d, 0x46, 0xec, 0xa1, 0x09, 0xa9);

// MFSampleExtension_Content_KeyID {C6C7F5B0-ACCA-415B-87D9-10441469EFC6}
// Type: GUID
DEFINE_GUID(MFSampleExtension_Content_KeyID,
0xc6c7f5b0, 0xacca, 0x415b, 0x87, 0xd9, 0x10, 0x44, 0x14, 0x69, 0xef, 0xc6);

// MFSampleExtension_Encryption_SubSampleMappingSplit {FE0254B9-2AA5-4EDC-99F7-17E89DBF9174}
// Type: BLOB
// Specifies the regions of clear and encrypted bytes in the sample
DEFINE_GUID(MFSampleExtension_Encryption_SubSampleMappingSplit,
0xfe0254b9, 0x2aa5, 0x4edc, 0x99, 0xf7, 0x17, 0xe8, 0x9d, 0xbf, 0x91, 0x74);

/////////////////////////////////////////////////////////////////////////////
//
// MFSample STANDARD EXTENSION ATTRIBUTE GUIDs
//
/////////////////////////////////////////////////////////////////////////////

// {b1d5830a-deb8-40e3-90fa-389943716461}   MFSampleExtension_Interlaced                {UINT32 (BOOL)}
DEFINE_GUID(MFSampleExtension_Interlaced,
0xb1d5830a, 0xdeb8, 0x40e3, 0x90, 0xfa, 0x38, 0x99, 0x43, 0x71, 0x64, 0x61);

// {941ce0a3-6ae3-4dda-9a08-a64298340617}   MFSampleExtension_BottomFieldFirst          {UINT32 (BOOL)}
DEFINE_GUID(MFSampleExtension_BottomFieldFirst,
0x941ce0a3, 0x6ae3, 0x4dda, 0x9a, 0x08, 0xa6, 0x42, 0x98, 0x34, 0x06, 0x17);

// {304d257c-7493-4fbd-b149-9228de8d9a99}   MFSampleExtension_RepeatFirstField          {UINT32 (BOOL)}
DEFINE_GUID(MFSampleExtension_RepeatFirstField,
0x304d257c, 0x7493, 0x4fbd, 0xb1, 0x49, 0x92, 0x28, 0xde, 0x8d, 0x9a, 0x99);

// {9d85f816-658b-455a-bde0-9fa7e15ab8f9}   MFSampleExtension_SingleField               {UINT32 (BOOL)}
DEFINE_GUID(MFSampleExtension_SingleField,
0x9d85f816, 0x658b, 0x455a, 0xbd, 0xe0, 0x9f, 0xa7, 0xe1, 0x5a, 0xb8, 0xf9);

// {6852465a-ae1c-4553-8e9b-c3420fcb1637}   MFSampleExtension_DerivedFromTopField       {UINT32 (BOOL)}
DEFINE_GUID(MFSampleExtension_DerivedFromTopField,
0x6852465a, 0xae1c, 0x4553, 0x8e, 0x9b, 0xc3, 0x42, 0x0f, 0xcb, 0x16, 0x37);

// MFSampleExtension_MeanAbsoluteDifference {1cdbde11-08b4-4311-a6dd-0f9f371907aa}
// Type: UINT32
DEFINE_GUID(MFSampleExtension_MeanAbsoluteDifference,
0x1cdbde11, 0x08b4, 0x4311, 0xa6, 0xdd, 0x0f, 0x9f, 0x37, 0x19, 0x07, 0xaa);

// MFSampleExtension_LongTermReferenceFrameInfo {9154733f-e1bd-41bf-81d3-fcd918f71332}
// Type: UINT32
DEFINE_GUID(MFSampleExtension_LongTermReferenceFrameInfo,
0x9154733f, 0xe1bd, 0x41bf, 0x81, 0xd3, 0xfc, 0xd9, 0x18, 0xf7, 0x13, 0x32);

typedef struct _ROI_AREA {
    RECT rect;
    INT32 QPDelta;
} ROI_AREA, *PROI_AREA;

// MFSampleExtension_ROIRectangle {3414a438-4998-4d2c-be82-be3ca0b24d43}
// Type: BLOB
DEFINE_GUID(MFSampleExtension_ROIRectangle,
0x3414a438, 0x4998, 0x4d2c, 0xbe, 0x82, 0xbe, 0x3c, 0xa0, 0xb2, 0x4d, 0x43);

// MFSampleExtension_LastSlice {2b5d5457-5547-4f07-b8c8-b4a3a9a1daac}
// Type: UINT32
DEFINE_GUID(MFSampleExtension_LastSlice,
0x2b5d5457, 0x5547, 0x4f07, 0xb8, 0xc8, 0xb4, 0xa3, 0xa9, 0xa1, 0xda, 0xac);

// Indicates macroblock is not needed for output and can be skipped
#define MACROBLOCK_FLAG_SKIP 0x00000001                 
// Indicates macroblock is changed from the previous frame
#define MACROBLOCK_FLAG_DIRTY 0x00000002
// Indicates macroblock from the previous frame has moved to a new position
#define MACROBLOCK_FLAG_MOTION 0x00000004
// Indicates macroblock contains video playback or other continuous motion, rather than a slower moving screen capture
#define MACROBLOCK_FLAG_VIDEO 0x00000008
// Indicates that the motion vector values of MACROBLOCK_DATA are valid, and should be used in preference to
// the encoder's calculated motion vector values
#define MACROBLOCK_FLAG_HAS_MOTION_VECTOR 0x00000010
// Indicates that the QPDelta value of MACROBLOCK_DATA is valid, and specifies the QP of this macroblock relative
// to the rest of the frame
#define MACROBLOCK_FLAG_HAS_QP 0x00000020

typedef struct _MACROBLOCK_DATA {
    UINT32 flags;
    INT16 motionVectorX;
    INT16 motionVectorY;
    INT32 QPDelta;
} MACROBLOCK_DATA;

// MFSampleExtension_FeatureMap {a032d165-46fc-400a-b449-49de53e62a6e}
// Type: BLOB
// Blob should contain one MACROBLOCK_DATA structure for each macroblock in the
// input frame.
DEFINE_GUID(MFSampleExtension_FeatureMap,
0xa032d165, 0x46fc, 0x400a, 0xb4, 0x49, 0x49, 0xde, 0x53, 0xe6, 0x2a, 0x6e);

// MFSampleExtension_ChromaOnly {1eb9179c-a01f-4845-8c04-0e65a26eb04f}
// Type: BOOL (UINT32)
// Set to 1 if the input sample is a chroma-only frame
DEFINE_GUID(MFSampleExtension_ChromaOnly,
0x1eb9179c, 0xa01f, 0x4845, 0x8c, 0x04, 0x0e, 0x65, 0xa2, 0x6e, 0xb0, 0x4f);

// MFSampleExtension_SpatialLayerId {B7AABC7B-2396-457a-879E-623BFAB6E0AC}
// Type: UINT32
// The exact spatial layer id of a sample emitted by a decoder
DEFINE_GUID(MFSampleExtension_SpatialLayerId, 
0xb7aabc7b, 0x2396, 0x457a, 0x87, 0x9e, 0x62, 0x3b, 0xfa, 0xb6, 0xe0, 0xac);

// MFSampleExtension_TemporalLayerId {B3C1FCD2-B331-4376-B974-AD647769B2B0}
// Type: UINT32
// The exact temporal layer id of a sample emitted by a decoder
DEFINE_GUID(MFSampleExtension_TemporalLayerId, 
0xb3c1fcd2, 0xb331, 0x4376, 0xb9, 0x74, 0xad, 0x64, 0x77, 0x69, 0xb2, 0xb0);

typedef struct _MFSampleExtensionPsnrYuv { 
    FLOAT psnrY; // PSNR for Y plane
    FLOAT psnrU; // PSNR for U plane
    FLOAT psnrV; // PSNR for V plane
} MFSampleExtensionPsnrYuv;

// MFSampleExtension_FramePsnrYuv {1C633A3D-566F-4752-833B-2907DF5415E1}
// Type: IMFMediaBuffer
// A MFSampleExtensionPsnrYuv structure that specifies the PSNR data of YUV planes of an encoded video frame.
DEFINE_GUID(MFSampleExtension_FramePsnrYuv, 
0x1c633a3d, 0x566f, 0x4752, 0x83, 0x3b, 0x29, 0x07, 0xdf, 0x54, 0x15, 0xe1);

// MFSampleExtension_VideoEncodeQPMap {2C68A331-B712-49CA-860A-3A1D58237D88} 
// Type: IMFMediaBuffer
// The QP map of an encoded video frame.
DEFINE_GUID(MFSampleExtension_VideoEncodeQPMap, 
0x2c68a331, 0xb712, 0x49ca, 0x86, 0x0a, 0x3a, 0x1d, 0x58, 0x23, 0x7d, 0x88);

// MFSampleExtension_VideoEncodeBitsUsedMap {6894263D-E6E2-4BCC-849D-8570365F5114}
// Type: IMFMediaBuffer
// The bits used map of an encoded video frame.
DEFINE_GUID(MFSampleExtension_VideoEncodeBitsUsedMap, 
0x6894263d, 0xe6e2, 0x4bcc, 0x84, 0x9d, 0x85, 0x70, 0x36, 0x5f, 0x51, 0x14);

// MFSampleExtension_VideoEncodeSatdMap {ADF61D96-C2D3-4B57-A138-DDE4D351EAA9} 
// Type: IMFMediaBuffer 
// The SATD map of an encoded video frame. 
DEFINE_GUID(MFSampleExtension_VideoEncodeSatdMap,  
0xadf61d96, 0xc2d3, 0x4b57, 0xa1, 0x38, 0xdd, 0xe4, 0xd3, 0x51, 0xea, 0xa9); 


typedef enum _eAVEncVideoQPMapElementDataType
{
    // Signed types
    CODEC_API_QP_MAP_INT8   = 0x00000000, // QP map elements are of type INT8
    CODEC_API_QP_MAP_INT16  = 0x00000001, // QP map elements are of type INT16
    CODEC_API_QP_MAP_INT32  = 0x00000002, // QP map elements are of type INT32
    // Unsigned types
    CODEC_API_QP_MAP_UINT8  = 0x80000000, // QP map elements are of type UINT8
    CODEC_API_QP_MAP_UINT16 = 0x80000001, // QP map elements are of type UINT16
    CODEC_API_QP_MAP_UINT32 = 0x80000002, // QP map elements are of type UINT32
} eAVEncVideoQPMapElementDataType;

// To be used with AVEncVideoInputDeltaQPSettings and AVEncVideoInputAbsoluteQPSettings
typedef struct _inputQPSettings
{
    UINT32 minBlockSize;
    UINT32 maxBlockSize;
    UINT32 stepsBlockSize;
    eAVEncVideoQPMapElementDataType dataType;
    INT16 minValue;
    INT16 maxValue;
    UINT16 step;
} InputQPSettings;


// MFSampleExtension_VideoEncodeInputDeltaQPMap   {DAB419C3-BF21-4B46-8692-9A7BF0A71769}
// Type: IMFMediaBuffer
// MFSampleExtension_VideoEncodeInputDeltaQPMap specifies the input delta QP map of the frame.
// The delta QP map must use one of the block sizes specified by CODECAPI_AVEncVideoInputDeltaQPBlockSize.
DEFINE_GUID(MFSampleExtension_VideoEncodeInputDeltaQPMap,
0xdab419c3, 0xbf21, 0x4b46, 0x86, 0x92, 0x9a, 0x7b, 0xf0, 0xa7, 0x17, 0x69);

// MFSampleExtension_VideoEncodeInputAbsoluteQPMap {432A6E9A-F1ED-456E-8DC3-6F8985649EB9}
// Type: IMFMediaBuffer
// MFSampleExtension_VideoEncodeInputExtAbsDeltaQPMap specifies the absolute QP map of the frame.
// The absolute QP map must use one of the block sizes specified by CODECAPI_AVEncVideoInputAbsQPBlockSize.
DEFINE_GUID(MFSampleExtension_VideoEncodeInputAbsoluteQPMap,
0x432a6e9a, 0xf1ed, 0x456e, 0x8d, 0xc3, 0x6f, 0x89, 0x85, 0x64, 0x9e, 0xb9);

///////////////////////////////////////////////////////////////////////////////
/// These are the attribute GUIDs that need to be used by MFT0 to provide
/// thumbnail support.  We are declaring these in our internal idl first and
/// once we pass API spec review, we can move it to the public header.
///////////////////////////////////////////////////////////////////////////////
// MFSampleExtension_PhotoThumbnail
// {74BBC85C-C8BB-42DC-B586DA17FFD35DCC}
// Type: IUnknown
// If this attribute is set on the IMFSample provided by the MFT0, this will contain the IMFMediaBuffer which contains
// the Photo Thumbnail as configured using the KSPROPERTYSETID_ExtendedCameraControl.
DEFINE_GUID(MFSampleExtension_PhotoThumbnail, 
0x74BBC85C, 0xC8BB, 0x42DC, 0xB5, 0x86, 0xDA, 0x17, 0xFF, 0xD3, 0x5D, 0xCC);

// MFSampleExtension_PhotoThumbnailMediaType
// {61AD5420-EBF8-4143-89AF6BF25F672DEF}
// Type: IUnknown
// This attribute will contain the IMFMediaType which describes the image format type contained in the 
// MFSampleExtension_PhotoThumbnail attribute.  If the MFSampleExtension_PhotoThumbnail attribute
// is present on the photo sample, the MFSampleExtension_PhotoThumbnailMediaType is required.
DEFINE_GUID(MFSampleExtension_PhotoThumbnailMediaType, 
0x61AD5420, 0xEBF8, 0x4143, 0x89, 0xAF, 0x6B, 0xF2, 0x5F, 0x67, 0x2D, 0xEF);

// MFSampleExtension_CaptureMetadata
// Type: IUnknown (IMFAttributes)
// This is the IMFAttributes store for all the metadata related to the capture
// pipeline.  It can be potentially present on any IMFSample.
DEFINE_GUID(MFSampleExtension_CaptureMetadata,
0x2EBE23A8, 0xFAF5, 0x444A, 0xA6, 0xA2, 0xEB, 0x81, 0x08, 0x80, 0xAB, 0x5D);

// MFSampleExtension_MDLCacheCookie
// Type: IUnknown (IMFAttributes)
// This is the IMFAttributes stored in the sample if the mini driver
// desires to cache MDL's. This is used internally by the pipeline.
// {5F002AF9-D8F9-41A3-B6C3-A2AD43F647AD}

DEFINE_GUID(MFSampleExtension_MDLCacheCookie,
0x5F002AF9, 0xD8F9, 0x41A3, 0xB6, 0xC3, 0xA2, 0xAD, 0x43, 0xF6, 0x47, 0xAD);

// Put all MF_CAPTURE_METADATA_* here.
// {0F9DD6C6-6003-45D8-BD59-F1F53E3D04E8}   MF_CAPTURE_METADATA_PHOTO_FRAME_FLASH       {UINT32}
// 0 - No flash triggered on this frame.
// non-0 - Flash triggered on this frame.
// Do not explicitly check for a value of 1 here, we may overload this to
// indicate special types of flash going forward (applications should only
// check for != 0 to indicate flash took place).
DEFINE_GUID(MF_CAPTURE_METADATA_PHOTO_FRAME_FLASH,  
0x0F9DD6C6, 0x6003, 0x45D8, 0xBD, 0x59, 0xF1, 0xF5, 0x3E, 0x3D, 0x04, 0xE8);  

// The raw IUnknown corresponding to the IMFMediaBuffer that contains the metadata
// stream as written by the camera driver.  This may be a mix of pre-defined metadata
// such as photo confirmation, focus notification, or custom metadata that only
// the MFT0 can parse.
DEFINE_GUID(MF_CAPTURE_METADATA_FRAME_RAWSTREAM, 
0x9252077B, 0x2680, 0x49B9, 0xAE, 0x02, 0xB1, 0x90, 0x75, 0x97, 0x3B, 0x70);

// {A87EE154-997F-465D-B91F-29D53B982B88}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_FOCUSSTATE, 
0xa87ee154, 0x997f, 0x465d, 0xb9, 0x1f, 0x29, 0xd5, 0x3b, 0x98, 0x2b, 0x88);

// {BB3716D9-8A61-47A4-8197-459C7FF174D5}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_REQUESTED_FRAME_SETTING_ID, 
0xbb3716d9, 0x8a61, 0x47a4, 0x81, 0x97, 0x45, 0x9c, 0x7f, 0xf1, 0x74, 0xd5);

// {16B9AE99-CD84-4063-879D-A28C7633729E}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_EXPOSURE_TIME, 
0x16b9ae99, 0xcd84, 0x4063, 0x87, 0x9d, 0xa2, 0x8c, 0x76, 0x33, 0x72, 0x9e);

// {D198AA75-4B62-4345-ABF3-3C31FA12C299}
DEFINE_GUID(MF_CAPTURE_METADATA_EXPOSURE_COMPENSATION,
0xd198aa75, 0x4b62, 0x4345, 0xab, 0xf3, 0x3c, 0x31, 0xfa, 0x12, 0xc2, 0x99);

// {E528A68F-B2E3-44FE-8B65-07BF4B5A13FF}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_ISO_SPEED, 
0xe528a68f, 0xb2e3, 0x44fe, 0x8b, 0x65, 0x7, 0xbf, 0x4b, 0x5a, 0x13, 0xff);

// {B5FC8E86-11D1-4E70-819B-723A89FA4520}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_LENS_POSITION, 
0xb5fc8e86, 0x11d1, 0x4e70, 0x81, 0x9b, 0x72, 0x3a, 0x89, 0xfa, 0x45, 0x20);

// {9CC3B54D-5ED3-4BAE-B388-7670AEF59E13}
// TYPE: UINT64
DEFINE_GUID(MF_CAPTURE_METADATA_SCENE_MODE, 
0x9cc3b54d, 0x5ed3, 0x4bae, 0xb3, 0x88, 0x76, 0x70, 0xae, 0xf5, 0x9e, 0x13);

// {4A51520B-FB36-446C-9DF2-68171B9A0389}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_FLASH, 
0x4a51520b, 0xfb36, 0x446c, 0x9d, 0xf2, 0x68, 0x17, 0x1b, 0x9a, 0x3, 0x89);

// {9C0E0D49-0205-491A-BC9D-2D6E1F4D5684}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_FLASH_POWER, 
0x9c0e0d49, 0x205, 0x491a, 0xbc, 0x9d, 0x2d, 0x6e, 0x1f, 0x4d, 0x56, 0x84);

// {C736FD77-0FB9-4E2E-97A2-FCD490739EE9}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_WHITEBALANCE, 
0xc736fd77, 0xfb9, 0x4e2e, 0x97, 0xa2, 0xfc, 0xd4, 0x90, 0x73, 0x9e, 0xe9);

// {E50B0B81-E501-42C2-ABF2-857ECB13FA5C}
// TYPE: UINT32
DEFINE_GUID(MF_CAPTURE_METADATA_ZOOMFACTOR, 
0xe50b0b81, 0xe501, 0x42c2, 0xab, 0xf2, 0x85, 0x7e, 0xcb, 0x13, 0xfa, 0x5c);

// {864F25A6-349F-46B1-A30E-54CC22928A47}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_FACEROIS, 
0x864f25a6, 0x349f, 0x46b1, 0xa3, 0xe, 0x54, 0xcc, 0x22, 0x92, 0x8a, 0x47);

// {E94D50CC-3DA0-44d4-BB34-83198A741868}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_FACEROITIMESTAMPS, 
0xe94d50cc, 0x3da0, 0x44d4, 0xbb, 0x34, 0x83, 0x19, 0x8a, 0x74, 0x18, 0x68);

// {B927A1A8-18EF-46d3-B3AF-69372F94D9B2}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_FACEROICHARACTERIZATIONS, 
0xb927a1a8, 0x18ef, 0x46d3, 0xb3, 0xaf, 0x69, 0x37, 0x2f, 0x94, 0xd9, 0xb2);

// {05802AC9-0E1D-41c7-A8C8-7E7369F84E1E}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_ISO_GAINS, 
0x5802ac9, 0xe1d, 0x41c7, 0xa8, 0xc8, 0x7e, 0x73, 0x69, 0xf8, 0x4e, 0x1e);

// {DB51357E-9D3D-4962-B06D-07CE650D9A0A}
// TYPE: UINT64
DEFINE_GUID(MF_CAPTURE_METADATA_SENSORFRAMERATE, 
0xdb51357e, 0x9d3d, 0x4962, 0xb0, 0x6d, 0x7, 0xce, 0x65, 0xd, 0x9a, 0xa);

// {E7570C8F-2DCB-4c7c-AACE-22ECE7CCE647}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_WHITEBALANCE_GAINS, 
0xe7570c8f, 0x2dcb, 0x4c7c, 0xaa, 0xce, 0x22, 0xec, 0xe7, 0xcc, 0xe6, 0x47);

// {85358432-2EF6-4ba9-A3FB-06D82974B895}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_HISTOGRAM, 
0x85358432, 0x2ef6, 0x4ba9, 0xa3, 0xfb, 0x6, 0xd8, 0x29, 0x74, 0xb8, 0x95);

// {2e9575b8-8c31-4a02-8575-42b197b71592}
// TYPE: BLOB
DEFINE_GUID(MF_CAPTURE_METADATA_EXIF, 
0x2e9575b8, 0x8c31, 0x4a02, 0x85, 0x75, 0x42, 0xb1, 0x97, 0xb7, 0x15, 0x92);

// {6D688FFC-63D3-46FE-BADA-5B947DB0D080}
// TYPE: UINT64
DEFINE_GUID(MF_CAPTURE_METADATA_FRAME_ILLUMINATION,
0x6D688FFC, 0x63D3, 0x46FE, 0xBA, 0xDA, 0x5B, 0x94, 0x7D, 0xB0, 0xD0, 0x80);

// MF_CAPTURE_METADATA_UVC_PAYLOADHEADER {F9F88A87-E1DD-441E-95CB-42E21A64F1D9}
// Value type: Blob
// Stores USB Video Class Camera's payload header for user mode components to 
// get the camera timestamps and other header information.
DEFINE_GUID(MF_CAPTURE_METADATA_UVC_PAYLOADHEADER,
    0xf9f88a87, 0xe1dd, 0x441e, 0x95, 0xcb, 0x42, 0xe2, 0x1a, 0x64, 0xf1, 0xd9);

// MFSampleExtension_Depth_MinReliableDepth
// Type: UINT32, minimum reliable depth value in a D16 format depth frame.
// Default value if the attribute is absent is 1, because 0 represent invalid depth
// {5F8582B2-E36B-47C8-9B87-FEE1CA72C5B0}
DEFINE_GUID(MFSampleExtension_Depth_MinReliableDepth,
0x5f8582b2, 0xe36b, 0x47c8, 0x9b, 0x87, 0xfe, 0xe1, 0xca, 0x72, 0xc5, 0xb0);

// MFSampleExtension_Depth_MaxReliableDepth
// Type: UINT32, maximum reliable depth value in a D16 format depth frame
// Default value if the attribute is absent is 65535
// {E45545D1-1F0F-4A32-A8A7-6101A24EA8BE}
DEFINE_GUID(MFSampleExtension_Depth_MaxReliableDepth,
0xe45545d1, 0x1f0f, 0x4a32, 0xa8, 0xa7, 0x61, 0x1, 0xa2, 0x4e, 0xa8, 0xbe);

// MF_CAPTURE_METADATA_FIRST_SCANLINE_START_TIME_QPC {F9F88A87-E1DD-441E-95CB-42E21A64F1D9}
// Value type: UINT64
// Stores value of the start of scan in QPC time
DEFINE_GUID(MF_CAPTURE_METADATA_FIRST_SCANLINE_START_TIME_QPC,
    0x6a2c49f1, 0xe052, 0x46b6, 0xb2, 0xd9, 0x73, 0xc1, 0x55, 0x87, 0x09, 0xaf);

// MF_CAPTURE_METADATA_LAST_SCANLINE_END_TIME_QPC {F9F88A87-E1DD-441E-95CB-42E21A64F1D9}
// Value type: UINT64
// Stores value of the end of scan in QPC time
DEFINE_GUID(MF_CAPTURE_METADATA_LAST_SCANLINE_END_TIME_QPC,
    0xdccadecb, 0xc4d4, 0x400d, 0xb4, 0x18, 0x10, 0xe8, 0x85, 0x25, 0xe1, 0xf6);

// MF_CAPTURE_METADATA_SCANLINE_TIME_QPC_ACCURACY {F9F88A87-E1DD-441E-95CB-42E21A64F1D9}
// Value type: UINT64
// Stores value of timestamp accuracy in QPC time absolute value
DEFINE_GUID(MF_CAPTURE_METADATA_SCANLINE_TIME_QPC_ACCURACY,
    0x4cd79c51, 0xf765, 0x4b09, 0xb1, 0xe1, 0x27, 0xd1, 0xf7, 0xeb, 0xea, 0x09);

// MF_CAPTURE_METADATA_SCAN_DIRECTION {F9F88A87-E1DD-441E-95CB-42E21A64F1D9}
// Value type: UINT32
// Bitfield of the way the scan is read. If value is 0x00, scan is Left to Right, Top to Bottom
// 0x0 - Left -> Right
// 0x1 - Right -> Left
// 0x2  Bottom -> Top
// 0x0 - Horizontal Scanline
// 0x4 - Vertical Scanline
DEFINE_GUID(MF_CAPTURE_METADATA_SCANLINE_DIRECTION,
    0x6496a3ba, 0x1907, 0x49e6, 0xb0, 0xc3, 0x12, 0x37, 0x95, 0xf3, 0x80, 0xa9);

#define MFCAPTURE_METADATA_SCAN_RIGHT_LEFT         0x00000001
#define MFCAPTURE_METADATA_SCAN_BOTTOM_TOP         0x00000002
#define MFCAPTURE_METADATA_SCANLINE_VERTICAL       0x00000004

// {276F72A2-59C8-4F69-97B4-068B8C0EC044}
// Value type: BLOB
// Reports the current Digital Window as a DigitalWindowSetting structure.
DEFINE_GUID(MF_CAPTURE_METADATA_DIGITALWINDOW,
    0x276f72a2, 0x59c8, 0x4f69, 0x97, 0xb4, 0x6, 0x8b, 0x8c, 0xe, 0xc0, 0x44);

// Digital Window Region
typedef struct tagDigitalWindowSetting {
    double      OriginX;
    double      OriginY;
    double      WindowSize;
} DigitalWindowSetting;

// {03F14DD3-75DD-433A-A8E2-1E3F5F2A50A0}
// Value type: BLOB
// Reports the background segmentation mask BackgroundSegmentationMask structure.
// Refer to the KSCAMERA_METADATA_BACKGROUNDSEGMENTATIONMASK struct in ksmedia.h
DEFINE_GUID(MF_CAPTURE_METADATA_FRAME_BACKGROUND_MASK, 
0x3f14dd3, 0x75dd, 0x433a, 0xa8, 0xe2, 0x1e, 0x3f, 0x5f, 0x2a, 0x50, 0xa0);



typedef struct tagFaceRectInfoBlobHeader
{
    ULONG Size;     // Size of this header + all FaceRectInfo following
    ULONG Count;    // Number of FaceRectInfo's in the blob
} FaceRectInfoBlobHeader;

typedef struct tagFaceRectInfo
{
    RECT Region;            // Relative coordinates on the frame (Q31 format)
    LONG confidenceLevel;   // Confidence Level of the region being a face
} FaceRectInfo;


typedef struct tagFaceCharacterizationBlobHeader
{
    ULONG Size;  // Size of this header + all FaceCharacterization following
    ULONG Count; // Number of FaceCharacterization's in the blob. Must match the number of FaceRectInfo's in FaceRectInfoBlobHeader
} FaceCharacterizationBlobHeader;


typedef struct tagFaceCharacterization
{
    ULONG BlinkScoreLeft;        // [0, 100]. 0 indicates no blink for the left eye. 100 indicates definite blink for the left eye
    ULONG BlinkScoreRight;       // [0, 100]. 0 indicates no blink for the right eye. 100 indicates definite blink for the right eye
    ULONG FacialExpression;      // Any one of the MF_METADATAFACIALEXPRESSION_XXX defined 
    ULONG FacialExpressionScore; // [0, 100]. 0 indicates no such facial expression as identified. 100 indicates definite such facial expression as defined
} FaceCharacterization;
	
#define MF_METADATAFACIALEXPRESSION_SMILE       0x00000001

typedef struct tagCapturedMetadataExposureCompensation
{
    UINT64  Flags;  // KSCAMERA_EXTENDEDPROP_EVCOMP_XXX step flag
    INT32   Value;  // EV Compensation value in units of the step  
} CapturedMetadataExposureCompensation;


typedef struct tagCapturedMetadataISOGains
{
    FLOAT    AnalogGain; 	 
    FLOAT    DigitalGain;	  
} CapturedMetadataISOGains;

typedef struct tagCapturedMetadataWhiteBalanceGains
{
    FLOAT    R; 	 
    FLOAT    G;
    FLOAT    B; 	  
} CapturedMetadataWhiteBalanceGains;

typedef struct tagMetadataTimeStamps 
{
    ULONG Flags;           // Bitwise OR of MF_METADATATIMESTAMPS_XXX flags
    LONGLONG Device;       // QPC time for the sample where the metadata is derived from (in 100ns) 
    LONGLONG Presentation; // PTS for the sample where the metadata is derived from (in 100ns)
} MetadataTimeStamps;

#define MF_METADATATIMESTAMPS_DEVICE		0x00000001
#define MF_METADATATIMESTAMPS_PRESENTATION	0x00000002

typedef struct tagHistogramGrid
{
    ULONG Width;  // Width of the sensor output that histogram is collected from
    ULONG Height; // Height of the sensor output that histogram is collected from

    RECT Region;  // Absolute coordinates of the region on the sensor output that the histogram is collected for
} HistogramGrid;

typedef struct tagHistogramBlobHeader
{
    ULONG Size;        // Size of the entire histogram blob in bytes 
    ULONG Histograms;  // Number of histograms in the blob. Each histogram is identified by a HistogramHeader
} HistogramBlobHeader;

typedef struct tagHistogramHeader
{
    ULONG Size;         // Size in bytes of this header + (HistogramDataHeader + histogram data following)*number of channels available
    ULONG Bins;         // Number of bins in the histogram
    ULONG FourCC;       // Color space that the histogram is collected from
    ULONG ChannelMasks; // Masks of the color channels that the histogram is collected for
    HistogramGrid Grid; // Grid that the histogram is collected from
} HistogramHeader;

typedef struct tagHistogramDataHeader
{
    ULONG Size;        // Size in bytes of this header + histogram data following
    ULONG ChannelMask; // Mask of the color channel for the histogram data
    ULONG Linear;      // 1, if linear; 0 nonlinear
} HistogramDataHeader;

#define MF_HISTOGRAM_CHANNEL_Y      0x00000001
#define MF_HISTOGRAM_CHANNEL_R      0x00000002
#define MF_HISTOGRAM_CHANNEL_G      0x00000004
#define MF_HISTOGRAM_CHANNEL_B      0x00000008
#define MF_HISTOGRAM_CHANNEL_Cb     0x00000010
#define MF_HISTOGRAM_CHANNEL_Cr     0x00000020

///////////////////////////////////////////////////////////////////////////////////////////////////////////////  Attributes ////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////


STDAPI
MFCreateAttributes(
    _Out_   IMFAttributes** ppMFAttributes,
    _In_    UINT32          cInitialSize
    );

STDAPI
MFInitAttributesFromBlob(
    _In_                    IMFAttributes*  pAttributes,
    _In_reads_bytes_(cbBufSize)  const UINT8*    pBuf,
    _In_                    UINT            cbBufSize
    );

STDAPI
MFGetAttributesAsBlobSize(
    _In_    IMFAttributes*  pAttributes,
    _Out_   UINT32*         pcbBufSize
    );

STDAPI
MFGetAttributesAsBlob(
    _In_                    IMFAttributes*  pAttributes,
    _Out_writes_bytes_(cbBufSize) UINT8*          pBuf,
    _In_                    UINT            cbBufSize
    );


///////////////////////////////////////////////////////////////////////////////////////////////////////////////  MFT Register & Enum ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// MFT Registry categories
//

#ifdef MF_INIT_GUIDS
#include <initguid.h>
#endif

// {d6c02d4b-6833-45b4-971a-05a4b04bab91}   MFT_CATEGORY_VIDEO_DECODER
DEFINE_GUID(MFT_CATEGORY_VIDEO_DECODER,
0xd6c02d4b, 0x6833, 0x45b4, 0x97, 0x1a, 0x05, 0xa4, 0xb0, 0x4b, 0xab, 0x91);

// {f79eac7d-e545-4387-bdee-d647d7bde42a}   MFT_CATEGORY_VIDEO_ENCODER
DEFINE_GUID(MFT_CATEGORY_VIDEO_ENCODER,
0xf79eac7d, 0xe545, 0x4387, 0xbd, 0xee, 0xd6, 0x47, 0xd7, 0xbd, 0xe4, 0x2a);

// {12e17c21-532c-4a6e-8a1c-40825a736397}   MFT_CATEGORY_VIDEO_EFFECT
DEFINE_GUID(MFT_CATEGORY_VIDEO_EFFECT,
0x12e17c21, 0x532c, 0x4a6e, 0x8a, 0x1c, 0x40, 0x82, 0x5a, 0x73, 0x63, 0x97);

// {059c561e-05ae-4b61-b69d-55b61ee54a7b}   MFT_CATEGORY_MULTIPLEXER
DEFINE_GUID(MFT_CATEGORY_MULTIPLEXER,
0x059c561e, 0x05ae, 0x4b61, 0xb6, 0x9d, 0x55, 0xb6, 0x1e, 0xe5, 0x4a, 0x7b);

// {a8700a7a-939b-44c5-99d7-76226b23b3f1}   MFT_CATEGORY_DEMULTIPLEXER
DEFINE_GUID(MFT_CATEGORY_DEMULTIPLEXER,
0xa8700a7a, 0x939b, 0x44c5, 0x99, 0xd7, 0x76, 0x22, 0x6b, 0x23, 0xb3, 0xf1);

// {9ea73fb4-ef7a-4559-8d5d-719d8f0426c7}   MFT_CATEGORY_AUDIO_DECODER
DEFINE_GUID(MFT_CATEGORY_AUDIO_DECODER,
0x9ea73fb4, 0xef7a, 0x4559, 0x8d, 0x5d, 0x71, 0x9d, 0x8f, 0x04, 0x26, 0xc7);

// {91c64bd0-f91e-4d8c-9276-db248279d975}   MFT_CATEGORY_AUDIO_ENCODER
DEFINE_GUID(MFT_CATEGORY_AUDIO_ENCODER,
0x91c64bd0, 0xf91e, 0x4d8c, 0x92, 0x76, 0xdb, 0x24, 0x82, 0x79, 0xd9, 0x75);

// {11064c48-3648-4ed0-932e-05ce8ac811b7}   MFT_CATEGORY_AUDIO_EFFECT
DEFINE_GUID(MFT_CATEGORY_AUDIO_EFFECT,
0x11064c48, 0x3648, 0x4ed0, 0x93, 0x2e, 0x05, 0xce, 0x8a, 0xc8, 0x11, 0xb7);

#if (WINVER >= _WIN32_WINNT_WIN7)
// {302EA3FC-AA5F-47f9-9F7A-C2188BB163021}...MFT_CATEGORY_VIDEO_PROCESSOR
DEFINE_GUID(MFT_CATEGORY_VIDEO_PROCESSOR, 
0x302ea3fc, 0xaa5f, 0x47f9, 0x9f, 0x7a, 0xc2, 0x18, 0x8b, 0xb1, 0x63, 0x2);
#endif // (WINVER >= _WIN32_WINNT_WIN7)

// {90175d57-b7ea-4901-aeb3-933a8747756f}   MFT_CATEGORY_OTHER
DEFINE_GUID(MFT_CATEGORY_OTHER,
0x90175d57, 0xb7ea, 0x4901, 0xae, 0xb3, 0x93, 0x3a, 0x87, 0x47, 0x75, 0x6f);

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)
DEFINE_GUID(MFT_CATEGORY_ENCRYPTOR,  
0xb0c687be, 0x01cd, 0x44b5, 0xb8, 0xb2, 0x7c, 0x1d, 0x7e, 0x05, 0x8b, 0x1f);
#endif

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
// {145CD8B4-92F4-4b23-8AE7-E0DF06C2DA95}   MFT_CATEGORY_VIDEO_RENDERER_EFFECT 
DEFINE_GUID(MFT_CATEGORY_VIDEO_RENDERER_EFFECT,
0x145cd8b4, 0x92f4, 0x4b23, 0x8a, 0xe7, 0xe0, 0xdf, 0x6, 0xc2, 0xda, 0x95);
#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// "Flags" is for future expansion - for now must be 0
//
STDAPI
MFTRegister(
    _In_                            CLSID                   clsidMFT,
    _In_                            GUID                    guidCategory,
    _In_                            LPWSTR                  pszName,
    _In_                            UINT32                  Flags,
    _In_                            UINT32                  cInputTypes,
    _In_reads_opt_(cInputTypes)    MFT_REGISTER_TYPE_INFO* pInputTypes,
    _In_                            UINT32                  cOutputTypes,
    _In_reads_opt_(cOutputTypes)   MFT_REGISTER_TYPE_INFO* pOutputTypes,
    _In_opt_                        IMFAttributes*          pAttributes
    );

STDAPI
MFTUnregister(
    _In_    CLSID   clsidMFT
    );

#if (WINVER >= _WIN32_WINNT_WIN7)
//  Register an MFT class in-process
STDAPI
MFTRegisterLocal(
   _In_                        IClassFactory*          pClassFactory,
   _In_                        REFGUID                 guidCategory,
   _In_                        LPCWSTR                 pszName,
   _In_                        UINT32                  Flags,
   _In_                        UINT32                  cInputTypes,
   _In_reads_opt_(cInputTypes)const MFT_REGISTER_TYPE_INFO* pInputTypes,
   _In_                        UINT32                  cOutputTypes,
   _In_reads_opt_(cOutputTypes)const MFT_REGISTER_TYPE_INFO* pOutputTypes
    );

//  Unregister locally registered MFT
//  If pClassFactory is NULL all local MFTs are unregistered
STDAPI
MFTUnregisterLocal(
    _In_opt_    IClassFactory *   pClassFactory
    );

// Register an MFT class in-process, by CLSID
STDAPI
MFTRegisterLocalByCLSID(
   _In_                        REFCLSID                clisdMFT,
   _In_                        REFGUID                 guidCategory,
   _In_                        LPCWSTR                 pszName,
   _In_                        UINT32                  Flags,
   _In_                        UINT32                  cInputTypes,
   _In_reads_opt_(cInputTypes)const MFT_REGISTER_TYPE_INFO* pInputTypes,
   _In_                        UINT32                  cOutputTypes,
   _In_reads_opt_(cOutputTypes)const MFT_REGISTER_TYPE_INFO* pOutputTypes
    );

// Unregister locally registered MFT by CLSID
STDAPI
MFTUnregisterLocalByCLSID(
    _In_    CLSID   clsidMFT
    );
#endif // (WINVER >= _WIN32_WINNT_WIN7)

//
// result *ppclsidMFT must be freed with CoTaskMemFree.
//
STDAPI
MFTEnum(
    _In_                    GUID                    guidCategory,
    _In_                    UINT32                  Flags,
    _In_opt_                MFT_REGISTER_TYPE_INFO* pInputType,
    _In_opt_                MFT_REGISTER_TYPE_INFO* pOutputType,
    _In_opt_                IMFAttributes*          pAttributes,
    _Outptr_result_buffer_(*pcMFTs)   CLSID**           ppclsidMFT, // must be freed with CoTaskMemFree
    _Out_                   UINT32*                 pcMFTs
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

#if (WINVER >= _WIN32_WINNT_WIN7)

enum _MFT_ENUM_FLAG
{
    MFT_ENUM_FLAG_SYNCMFT                         = 0x00000001, // Enumerates V1 MFTs. This is default.
    MFT_ENUM_FLAG_ASYNCMFT                        = 0x00000002, // Enumerates only software async MFTs also known as V2 MFTs
    MFT_ENUM_FLAG_HARDWARE                        = 0x00000004, // Enumerates V2 hardware async MFTs
    MFT_ENUM_FLAG_FIELDOFUSE                      = 0x00000008, // Enumerates MFTs that require unlocking
    MFT_ENUM_FLAG_LOCALMFT                        = 0x00000010, // Enumerates Locally (in-process) registered MFTs
    MFT_ENUM_FLAG_TRANSCODE_ONLY                  = 0x00000020, // Enumerates decoder MFTs used by transcode only    
    MFT_ENUM_FLAG_SORTANDFILTER                   = 0x00000040, // Apply system local, do not use and preferred sorting and filtering
    MFT_ENUM_FLAG_SORTANDFILTER_APPROVED_ONLY     = 0x000000C0, // Similar to MFT_ENUM_FLAG_SORTANDFILTER, but apply a local policy of: MF_PLUGIN_CONTROL_POLICY_USE_APPROVED_PLUGINS
    MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY          = 0x00000140, // Similar to MFT_ENUM_FLAG_SORTANDFILTER, but apply a local policy of: MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS
    MFT_ENUM_FLAG_SORTANDFILTER_WEB_ONLY_EDGEMODE = 0x00000240, // Similar to MFT_ENUM_FLAG_SORTANDFILTER, but apply a local policy of: MF_PLUGIN_CONTROL_POLICY_USE_WEB_PLUGINS_EDGEMODE
    MFT_ENUM_FLAG_UNTRUSTED_STOREMFT              = 0x00000400, // Enumerates all untrusted store MFTs downloaded from the store
    MFT_ENUM_FLAG_ALL                             = 0x0000003F, // Enumerates all MFTs including SW and HW MFTs and applies filtering
};

//
// result *pppMFTActivate must be freed with CoTaskMemFree. Each IMFActivate pointer inside this
// buffer should be released.
//

STDAPI
MFTEnumEx(
    _In_                                GUID                            guidCategory,
    _In_                                UINT32                          Flags,
    _In_opt_                            const MFT_REGISTER_TYPE_INFO*   pInputType,
    _In_opt_                            const MFT_REGISTER_TYPE_INFO*   pOutputType,
    _Outptr_result_buffer_(*pnumMFTActivate) IMFActivate***                 pppMFTActivate,
    _Out_                               UINT32*                         pnumMFTActivate
);
#endif // (WINVER >= _WIN32_WINNT_WIN7)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

// MFT_ENUM_VIDEO_RENDERER_EXTENSION_PROFILE {62C56928-9A4E-443b-B9DC-CAC830C24100} 
// Type: VT_VECTOR | VT_LPWSTR 
// MFTEnumEx stores this on the attribute store of the IMFActivate object that  
// MFTEnumEx creates for MFTs that have an associated UWP Manifest containing the tag 
// VideoRendererExtensionProfiles.  This contains a list of all VideoRendererExtensionProfile 
// entries in the VideoRendererExtensionProfiles tag. 
DEFINE_GUID(MFT_ENUM_VIDEO_RENDERER_EXTENSION_PROFILE,
0x62c56928, 0x9a4e, 0x443b, 0xb9, 0xdc, 0xca, 0xc8, 0x30, 0xc2, 0x41, 0x0);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS1)  

// {1D39518C-E220-4DA8-A07F-BA172552D6B1}   MFT_ENUM_ADAPTER_LUID 
DEFINE_GUID( MFT_ENUM_ADAPTER_LUID,
    0x1d39518c, 0xe220, 0x4da8, 0xa0, 0x7f, 0xba, 0x17, 0x25, 0x52, 0xd6, 0xb1 );

STDAPI
MFTEnum2(
    _In_                                GUID                            guidCategory,
    _In_                                UINT32                          Flags,
    _In_opt_                            const MFT_REGISTER_TYPE_INFO*   pInputType,
    _In_opt_                            const MFT_REGISTER_TYPE_INFO*   pOutputType,
    _In_opt_                            IMFAttributes*                  pAttributes,
    _Outptr_result_buffer_( *pnumMFTActivate ) IMFActivate***           pppMFTActivate,
    _Out_                               UINT32*                         pnumMFTActivate
    );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS1)  


//
// results *pszName, *ppInputTypes, and *ppOutputTypes must be freed with CoTaskMemFree.
// *ppAttributes must be released.
//
STDAPI
MFTGetInfo(
    _In_                                   CLSID                       clsidMFT,
    _Out_opt_                              LPWSTR*                     pszName,
    _Outptr_opt_result_buffer_(*pcInputTypes)  MFT_REGISTER_TYPE_INFO**    ppInputTypes,
    _Out_opt_                              UINT32*                     pcInputTypes,
    _Outptr_opt_result_buffer_(*pcOutputTypes) MFT_REGISTER_TYPE_INFO**    ppOutputTypes,
    _Out_opt_                              UINT32*                     pcOutputTypes,
    _Outptr_opt_result_maybenull_                    IMFAttributes**             ppAttributes
    );


#if (WINVER >= _WIN32_WINNT_WIN7)

//
//  Get the plugin control API
//
STDAPI
MFGetPluginControl(
    _Out_ IMFPluginControl **ppPluginControl
    );

//
//  Get MFT's merit - checking that is has a valid certificate
//
STDAPI
MFGetMFTMerit(
    _Inout_ IUnknown *pMFT,
    _In_    UINT32   cbVerifier,
    _In_reads_bytes_(cbVerifier) const BYTE * verifier,
    _Out_   DWORD   *merit
    );

#endif // (WINVER >= _WIN32_WINNT_WIN7)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if (WINVER >= _WIN32_WINNT_WIN8)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI
MFRegisterLocalSchemeHandler(
    _In_        PCWSTR          szScheme,
    _In_        IMFActivate*    pActivate
    );

STDAPI
MFRegisterLocalByteStreamHandler(
    _In_        PCWSTR          szFileExtension,
    _In_        PCWSTR          szMimeType,
    _In_        IMFActivate*    pActivate
    );

//
// Wrap a bytestream so that calling Close() on the wrapper
// closes the wrapper but not the original bytestream. The
// original bytestream can then be passed to another
// media source for instance.
//
STDAPI
MFCreateMFByteStreamWrapper(
    _In_        IMFByteStream*  pStream,
    _Out_       IMFByteStream** ppStreamWrapper
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// Create a MF activate object that can instantiate media extension objects.
// The activate object supports both IMFActivate and IClassFactory.
//
STDAPI
MFCreateMediaExtensionActivate(
    _In_        PCWSTR          szActivatableClassId,
    _In_opt_    IUnknown*       pConfiguration,
    _In_        REFIID          riid,
    _Outptr_    LPVOID*         ppvObject
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // (WINVER >= _WIN32_WINNT_WIN8)

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


///////////////////////////////////////////////////////////////////////////////////////////////////////////////  MFT  Attributes GUIDs ////////////////////////////
// {53476A11-3F13-49fb-AC42-EE2733C96741} MFT_SUPPORT_DYNAMIC_FORMAT_CHANGE {UINT32 (BOOL)}
DEFINE_GUID(MFT_SUPPORT_DYNAMIC_FORMAT_CHANGE,
0x53476a11, 0x3f13, 0x49fb, 0xac, 0x42, 0xee, 0x27, 0x33, 0xc9, 0x67, 0x41);
///////////////////////////////////////////////////////////////////////////////////////////////////////////////  Media Type GUIDs ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// GUIDs for media types
//

//
// In MF, media types for uncompressed video formats MUST be composed from a FourCC or D3DFORMAT combined with
// the "base GUID" {00000000-0000-0010-8000-00AA00389B71} by replacing the initial 32 bits with the FourCC/D3DFORMAT
//
// Audio media types for types which already have a defined wFormatTag value can be constructed similarly, by
// putting the wFormatTag (zero-extended to 32 bits) into the first 32 bits of the base GUID.
//
// Compressed video or audio can also use any well-known GUID that exists, or can create a new GUID.
//
// GUIDs for common media types are defined below.
//

// needed for the GUID definition macros below
#ifndef FCC
#define FCC(ch4) ((((DWORD)(ch4) & 0xFF) << 24) |     \
                  (((DWORD)(ch4) & 0xFF00) << 8) |    \
                  (((DWORD)(ch4) & 0xFF0000) >> 8) |  \
                  (((DWORD)(ch4) & 0xFF000000) >> 24))
#endif




//
// this macro creates a media type GUID from a FourCC, D3DFMT, or WAVE_FORMAT
//
#ifndef DEFINE_MEDIATYPE_GUID
#define DEFINE_MEDIATYPE_GUID(name, format) \
    DEFINE_GUID(name,                       \
    format, 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71);
#endif

//
// video media types
//

//
// If no D3D headers have been included yet, define local versions of D3DFMT constants we use.
// We can't include D3D headers from this header because we need it to be compatible with all versions
// of D3D.
//
#ifndef DIRECT3D_VERSION
#define D3DFMT_R8G8B8       20
#define D3DFMT_A8R8G8B8     21
#define D3DFMT_X8R8G8B8     22
#define D3DFMT_R5G6B5       23
#define D3DFMT_X1R5G5B5     24
#define D3DFMT_A2B10G10R10  31
#define D3DFMT_P8           41
#define D3DFMT_L8           50
#define D3DFMT_D16          80
#define D3DFMT_L16          81
#define D3DFMT_A16B16G16R16F 113
#define LOCAL_D3DFMT_DEFINES 1
#endif

DEFINE_MEDIATYPE_GUID( MFVideoFormat_Base,      0x00000000 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_RGB32,     D3DFMT_X8R8G8B8 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_ARGB32,    D3DFMT_A8R8G8B8 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_RGB24,     D3DFMT_R8G8B8 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_RGB555,    D3DFMT_X1R5G5B5 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_RGB565,    D3DFMT_R5G6B5 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_RGB8,      D3DFMT_P8 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_L8,        D3DFMT_L8 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_L16,       D3DFMT_L16 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_D16,       D3DFMT_D16 );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_AI44,      FCC('AI44') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_AYUV,      FCC('AYUV') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_YUY2,      FCC('YUY2') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_YVYU,      FCC('YVYU') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_YVU9,      FCC('YVU9') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_UYVY,      FCC('UYVY') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_NV11,      FCC('NV11') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_NV12,      FCC('NV12') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_NV21,      FCC('NV21') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_YV12,      FCC('YV12') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_I420,      FCC('I420') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_I422,      FCC('I422') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_I444,      FCC('I444') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_IYUV,      FCC('IYUV') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y210,      FCC('Y210') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y216,      FCC('Y216') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y410,      FCC('Y410') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y416,      FCC('Y416') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y41P,      FCC('Y41P') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y41T,      FCC('Y41T') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_Y42T,      FCC('Y42T') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_P210,      FCC('P210') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_P216,      FCC('P216') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_P010,      FCC('P010') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_P016,      FCC('P016') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_v210,      FCC('v210') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_v216,      FCC('v216') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_v410,      FCC('v410') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MP43,      FCC('MP43') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MP4S,      FCC('MP4S') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_M4S2,      FCC('M4S2') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MP4V,      FCC('MP4V') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_WMV1,      FCC('WMV1') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_WMV2,      FCC('WMV2') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_WMV3,      FCC('WMV3') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_WVC1,      FCC('WVC1') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MSS1,      FCC('MSS1') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MSS2,      FCC('MSS2') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MPG1,      FCC('MPG1') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DVSL,      FCC('dvsl') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DVSD,      FCC('dvsd') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DVHD,      FCC('dvhd') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DV25,      FCC('dv25') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DV50,      FCC('dv50') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DVH1,      FCC('dvh1') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_DVC,       FCC('dvc ') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_H264,      FCC('H264') );  // assume MFVideoFormat_H264 is frame aligned. that is, each input sample has one complete compressed frame (one frame picture, two field pictures or a single unpaired field picture)
DEFINE_MEDIATYPE_GUID( MFVideoFormat_H265,      FCC('H265') );  
DEFINE_MEDIATYPE_GUID( MFVideoFormat_MJPG,      FCC('MJPG') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_420O,      FCC('420O') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_HEVC,      FCC('HEVC') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_HEVC_ES,   FCC('HEVS') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_VP80,      FCC('VP80') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_VP90,      FCC('VP90') );
DEFINE_MEDIATYPE_GUID( MFVideoFormat_ORAW,      FCC('ORAW') );

#if (WINVER >= _WIN32_WINNT_WIN8)
DEFINE_MEDIATYPE_GUID( MFVideoFormat_H263,      FCC('H263') );
#endif // (WINVER >= _WIN32_WINNT_WIN8)

#if (WDK_NTDDI_VERSION >= NTDDI_WIN10)
DEFINE_MEDIATYPE_GUID(MFVideoFormat_A2R10G10B10, D3DFMT_A2B10G10R10);
DEFINE_MEDIATYPE_GUID(MFVideoFormat_A16B16G16R16F, D3DFMT_A16B16G16R16F);
#endif

#if (WDK_NTDDI_VERSION >= NTDDI_WIN10_RS3)
DEFINE_MEDIATYPE_GUID(MFVideoFormat_VP10,       FCC('VP10'));
DEFINE_MEDIATYPE_GUID(MFVideoFormat_AV1,        FCC('AV01'));
#endif

DEFINE_MEDIATYPE_GUID(MFVideoFormat_Theora,     FCC('theo')); // {6F656874-0000-0010-8000-00AA00389B71}
DEFINE_MEDIATYPE_GUID(MFVideoFormat_APV,        FCC('APV '));

#if (WDK_NTDDI_VERSION >= NTDDI_WIN10)
//
// MFSample Perception Date Type-specific attribute GUIDs should be in sync with KSCameraProfileSensorType
//
typedef enum _MFFrameSourceTypes
{
    MFFrameSourceTypes_Color                = 0x0001,
    MFFrameSourceTypes_Infrared             = 0x0002,
    MFFrameSourceTypes_Depth                = 0x0004,
    MFFrameSourceTypes_Image                = 0x0008,
    MFFrameSourceTypes_Custom               = 0x0080
} MFFrameSourceTypes;

#endif // (WINVER > _WIN32_WINNT_WIN10)

//
// undef the local D3DFMT definitions to avoid later clashes with D3D headers
//
#ifdef LOCAL_D3DFMT_DEFINES
#undef D3DFMT_R8G8B8
#undef D3DFMT_A8R8G8B8
#undef D3DFMT_X8R8G8B8
#undef D3DFMT_R5G6B5
#undef D3DFMT_X1R5G5B5
#undef D3DFMT_P8
#undef D3DFMT_A2B10G10R10
#undef D3DFMT_A16B16G16R16F
#undef D3DFMT_L8
#undef D3DFMT_D16
#undef D3DFMT_L16
#undef LOCAL_D3DFMT_DEFINES
#endif


// assume MFVideoFormat_H264_ES may not be frame aligned. that is, each input sample may have one partial frame, 
// multiple frames, some frames plus some partial frame 
// or more general, N.M frames, N is the integer part and M is the fractional part.
//
// {3F40F4F0-5622-4FF8-B6D8-A17A584BEE5E}       MFVideoFormat_H264_ES
DEFINE_GUID(MFVideoFormat_H264_ES, 
0x3f40f4f0, 0x5622, 0x4ff8, 0xb6, 0xd8, 0xa1, 0x7a, 0x58, 0x4b, 0xee, 0x5e);


//
// some legacy formats that don't fit the common pattern
//

// {e06d8026-db46-11cf-b4d1-00805f6cbbea}       MFVideoFormat_MPEG2
DEFINE_GUID(MFVideoFormat_MPEG2,
0xe06d8026, 0xdb46, 0x11cf, 0xb4, 0xd1, 0x00, 0x80, 0x5f, 0x6c, 0xbb, 0xea);

#define MFVideoFormat_MPG2 MFVideoFormat_MPEG2



//
// audio media types
//
DEFINE_MEDIATYPE_GUID( MFAudioFormat_Base,              0x00000000 );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_PCM,               WAVE_FORMAT_PCM );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_Float,             WAVE_FORMAT_IEEE_FLOAT );

// MFAudioFormat_DTS is for S/PDIF-encapsulated DTS core streams. It is the same as KSDATAFORMAT_SUBTYPE_IEC61937_DTS in ksmedia.h.
// Use MEDIASUBTYPE_DTS2 (defined in wmcodecdsp.h) for raw DTS core streams.
// MFAudioFormat_DTS_RAW (same as MEDIASUBTYPE_DTS) can also be used for raw DTS core streams. While the values for MEDIASUBTYPE_DTS and
// MEDIASUBTYPE_DTS2 are different, the stream type is the same.
//
// If DTS extension substreams may be present, use MFAudioFormat_DTS_HD (same as MEDIASUBTYPE_DTS_HD) for Master Audio,
// and MEDIASUBTYPE_DTS_HD_HRA for High Resolution Audio and other extension substream variants.
// (KSDATAFORMAT_SUBTYPE_IEC61937_DTS_HD is the S/PDIF media subtype for MEDIASUBTYPE_DTS_HD and MEDIASUBTYPE_DTS_HD_HRA.)
DEFINE_MEDIATYPE_GUID( MFAudioFormat_DTS,               WAVE_FORMAT_DTS );

// MFAudioFormat_Dolby_AC3_SPDIF is for S/PDIF-encapsulated AC-3. It is the same as KSDATAFORMAT_SUBTYPE_IEC61937_DOLBY_DIGITAL in ksmedia.h.
// Use MFAudioFormat_Dolby_AC3 (MEDIASUBTYPE_DOLBY_AC3 in wmcodecdsp.h) for raw AC-3 streams.
DEFINE_MEDIATYPE_GUID( MFAudioFormat_Dolby_AC3_SPDIF,   WAVE_FORMAT_DOLBY_AC3_SPDIF );

DEFINE_MEDIATYPE_GUID( MFAudioFormat_DRM,               WAVE_FORMAT_DRM );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_WMAudioV8,         WAVE_FORMAT_WMAUDIO2 );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_WMAudioV9,         WAVE_FORMAT_WMAUDIO3 );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_WMAudio_Lossless,  WAVE_FORMAT_WMAUDIO_LOSSLESS );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_WMASPDIF,          WAVE_FORMAT_WMASPDIF );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_MSP1,              WAVE_FORMAT_WMAVOICE9 );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_MP3,               WAVE_FORMAT_MPEGLAYER3 );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_MPEG,              WAVE_FORMAT_MPEG );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_AAC,               WAVE_FORMAT_MPEG_HEAAC );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_ADTS,              WAVE_FORMAT_MPEG_ADTS_AAC );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_AMR_NB,            WAVE_FORMAT_AMR_NB );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_AMR_WB,            WAVE_FORMAT_AMR_WB );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_AMR_WP,            WAVE_FORMAT_AMR_WP );
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD)
DEFINE_MEDIATYPE_GUID( MFAudioFormat_FLAC,              WAVE_FORMAT_FLAC );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_ALAC,              WAVE_FORMAT_ALAC );
DEFINE_MEDIATYPE_GUID( MFAudioFormat_Opus,              WAVE_FORMAT_OPUS );
#endif
DEFINE_MEDIATYPE_GUID( MFAudioFormat_Dolby_AC4,         WAVE_FORMAT_DOLBY_AC4 );

// These audio types are not derived from an existing wFormatTag 
DEFINE_GUID(MFAudioFormat_Dolby_AC3, // == MEDIASUBTYPE_DOLBY_AC3 defined in ksuuids.h
0xe06d802c, 0xdb46, 0x11cf, 0xb4, 0xd1, 0x00, 0x80, 0x05f, 0x6c, 0xbb, 0xea);
DEFINE_GUID(MFAudioFormat_Dolby_DDPlus, // == MEDIASUBTYPE_DOLBY_DDPLUS defined in wmcodecdsp.h
0xa7fb87af, 0x2d02, 0x42fb, 0xa4, 0xd4, 0x5, 0xcd, 0x93, 0x84, 0x3b, 0xdd);

// {36b7927c-3d87-4a2a-9196-a21ad9e935e6}
// AC-4 bitstream versions 0 and 1.
// This audio media type is normally used as an alternate media type (the primary being MFAudioFormat_Dolby_AC4)
// to allow a MFT to register support for only version 0 and 1 of the AC-4 bistream.
DEFINE_GUID(MFAudioFormat_Dolby_AC4_V1,
0x36b7927c, 0x3d87, 0x4a2a, 0x91, 0x96, 0xa2, 0x1a, 0xd9, 0xe9, 0x35, 0xe6);

// {7998b2a0-17dd-49b6-8dfa-9b278552a2ac}
// AC-4 bitstream version 2. (Supports Immersive Stereo.)
// This audio media type is normally used as an alternate media type (the primary being MFAudioFormat_Dolby_AC4)
// to allow a MFT to register support for only version 2 of the AC-4 bistream.
DEFINE_GUID(MFAudioFormat_Dolby_AC4_V2,
0x7998b2a0, 0x17dd, 0x49b6, 0x8d, 0xfa, 0x9b, 0x27, 0x85, 0x52, 0xa2, 0xac);

// {9d8dccc6-d156-4fb8-979c-a85be7d21dfa}
// This format is used for AC-4 streams that use ac4_syncframe and the optional crc
// at the end of each frame. The frames might not be aligned with IMFSample boundaries.
DEFINE_GUID(MFAudioFormat_Dolby_AC4_V1_ES,
0x9d8dccc6, 0xd156, 0x4fb8, 0x97, 0x9c, 0xa8, 0x5b, 0xe7, 0xd2, 0x1d, 0xfa);

// {7e58c9f9-b070-45f4-8ccd-a99a0417c1ac}
// This format is used for AC-4 version 2 bit streams (may include Immersive Stereo) that use ac4_syncframe
// and the optional crc at the end of each frame. The frames might not be aligned with IMFSample boundaries.
DEFINE_GUID(MFAudioFormat_Dolby_AC4_V2_ES,
0x7e58c9f9, 0xb070, 0x45f4, 0x8c, 0xcd, 0xa9, 0x9a, 0x04, 0x17, 0xc1, 0xac);

// {7c13c441-ebf8-4931-b678-800b19242236}
// This format is used for MPEG-H MHAS bitstreams where each IMFSample is aligned with the start of a MHAS packet.
DEFINE_GUID(MFAudioFormat_MPEGH,
0x7c13c441, 0xebf8, 0x4931, 0xb6, 0x78, 0x80, 0x0b, 0x19, 0x24, 0x22, 0x36);

// {19ee97fe-1be0-4255-a876-e99f53a42ae3}
// This format is used for MPEG-H MHAS elementary bitstreams where each IMFSample may not be aligned with the start of a 
// MHAS packet.
DEFINE_GUID(MFAudioFormat_MPEGH_ES,
0x19ee97fe, 0x1be0, 0x4255, 0xa8, 0x76, 0xe9, 0x9f, 0x53, 0xa4, 0x2a, 0xe3);

DEFINE_GUID(MFAudioFormat_Vorbis,      // {8D2FD10B-5841-4a6b-8905-588FEC1ADED9}
0x8D2FD10B, 0x5841, 0x4a6b, 0x89, 0x05, 0x58, 0x8F, 0xEC, 0x1A, 0xDE, 0xD9);
DEFINE_GUID(MFAudioFormat_DTS_RAW, // == MEDIASUBTYPE_DTS defined in ksuuids.h
0xE06D8033, 0xDB46, 0x11CF, 0xB4, 0xD1, 0x00, 0x80, 0x5F, 0x6C, 0xBB, 0xEA);
DEFINE_GUID(MFAudioFormat_DTS_HD, // == MEDIASUBTYPE_DTS_HD defined in wmcodecdsp.h
0xA2E58EB7, 0x0FA9, 0x48BB, 0xA4, 0x0C, 0xFA, 0x0E, 0x15, 0x6D, 0x06, 0x45);
DEFINE_GUID(MFAudioFormat_DTS_XLL, // {45B37C1B-8C70-4E59-A7BE-A1E42C81C80D}
0x45B37C1B, 0x8C70, 0x4E59, 0xA7, 0xBE, 0xA1, 0xE4, 0x2C, 0x81, 0xC8, 0x0D);
DEFINE_GUID(MFAudioFormat_DTS_LBR, // {C2FE6F0A-4E3C-4DF1-9B60-50863091E4B9}
0xC2FE6F0A, 0x4E3C, 0x4DF1, 0x9B, 0x60, 0x50, 0x86, 0x30, 0x91, 0xE4, 0xB9);
DEFINE_GUID(MFAudioFormat_DTS_UHD, // {87020117-ACE3-42DE-B73E-C656706263F8}
0x87020117, 0xACE3, 0x42DE, 0xB7, 0x3E, 0xC6, 0x56, 0x70, 0x62, 0x63, 0xF8);
DEFINE_GUID(MFAudioFormat_DTS_UHDY, // {9B9CCA00-91B9-4CCC-883A-8F787AC3CC86}
0x9B9CCA00, 0x91B9, 0x4CCC, 0x88, 0x3A, 0x8F, 0x78, 0x7A, 0xC3, 0xCC, 0x86);

DEFINE_GUID(MFAudioFormat_IAMF, // {78a8eba0-f446-4851-a55d-5372280e6b0b}
0x78a8eba0, 0xf446, 0x4851, 0xa5, 0x5d, 0x53, 0x72, 0x28, 0x0e, 0x6b, 0x0b);

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)

DEFINE_GUID( MFAudioFormat_Float_SpatialObjects,
    0xfa39cd94, 0xbc64, 0x4ab1, 0x9b, 0x71, 0xdc, 0xd0, 0x9d, 0x5a, 0x7e, 0x7a );

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)

#if (WINVER >= _WIN32_WINNT_THRESHOLD)
// LPCM audio with headers for encapsulation in an MPEG2 bitstream
DEFINE_GUID(MFAudioFormat_LPCM, // == MEDIASUBTYPE_LPCM defined in ksmedia.h
0xe06d8032L, 0xdb46, 0x11cf, 0xb4, 0xd1, 0x00, 0x80, 0x5f, 0x6c, 0xbb, 0xea);

DEFINE_GUID(MFAudioFormat_PCM_HDCP,
0xa5e7ff01, 0x8411, 0x4acc, 0xa8, 0x65, 0x5f, 0x49, 0x41, 0x28, 0x8d, 0x80);

DEFINE_GUID(MFAudioFormat_Dolby_AC3_HDCP,
0x97663a80, 0x8ffb, 0x4445, 0xa6, 0xba, 0x79, 0x2d, 0x90, 0x8f, 0x49, 0x7f);

DEFINE_GUID(MFAudioFormat_AAC_HDCP,
0x419bce76, 0x8b72, 0x400f, 0xad, 0xeb, 0x84, 0xb5, 0x7d, 0x63, 0x48, 0x4d);

DEFINE_GUID(MFAudioFormat_ADTS_HDCP,
0xda4963a3, 0x14d8, 0x4dcf, 0x92, 0xb7, 0x19, 0x3e, 0xb8, 0x43, 0x63, 0xdb);

DEFINE_GUID(MFAudioFormat_Base_HDCP,
0x3884b5bc, 0xe277, 0x43fd, 0x98, 0x3d, 0x03, 0x8a, 0xa8, 0xd9, 0xb6, 0x05);

DEFINE_GUID(MFVideoFormat_H264_HDCP,
0x5d0ce9dd, 0x9817, 0x49da, 0xbd, 0xfd, 0xf5, 0xf5, 0xb9, 0x8f, 0x18, 0xa6);

DEFINE_GUID(MFVideoFormat_HEVC_HDCP,
0x3cfe0fe6, 0x05c4, 0x47dc, 0x9d, 0x70, 0x4b, 0xdb, 0x29, 0x59, 0x72, 0x0f);

DEFINE_GUID(MFVideoFormat_Base_HDCP,
0xeac3b9d5, 0xbd14, 0x4237, 0x8f, 0x1f, 0xba, 0xb4, 0x28, 0xe4, 0x93, 0x12);

#endif


//
// MPEG-4 media types
//

// {00000000-767a-494d-b478-f29d25dc9037}       MFMPEG4Format_Base
DEFINE_GUID(MFMPEG4Format_Base,
0x00000000, 0x767a, 0x494d, 0xb4, 0x78, 0xf2, 0x9d, 0x25, 0xdc, 0x90, 0x37);


//
// Subtitle media types
//

// {2006F94F-29CA-4195-B8DB-00DED8FF0C97}      MFSubtitleFormat_XML
DEFINE_GUID(MFSubtitleFormat_XML, 
    0x2006f94f, 0x29ca, 0x4195, 0xb8, 0xdb, 0x00, 0xde, 0xd8, 0xff, 0x0c, 0x97);

// {73E73992-9a10-4356-9557-7194E91E3E54}      MFSubtitleFormat_TTML
DEFINE_GUID(MFSubtitleFormat_TTML,
    0x73e73992, 0x9a10, 0x4356, 0x95, 0x57, 0x71, 0x94, 0xe9, 0x1e, 0x3e, 0x54);

// {7FA7FAA3-FEAE-4E16-AEDF-36B9ACFBB099}      MFSubtitleFormat_ATSC 
DEFINE_GUID(MFSubtitleFormat_ATSC,
    0x7fa7faa3, 0xfeae, 0x4e16, 0xae, 0xdf, 0x36, 0xb9, 0xac, 0xfb, 0xb0, 0x99);

// {C886D215-F485-40BB-8DB6-FADBC619A45D}      MFSubtitleFormat_WebVTT 
DEFINE_GUID(MFSubtitleFormat_WebVTT,
    0xc886d215, 0xf485, 0x40bb, 0x8d, 0xb6, 0xfa, 0xdb, 0xc6, 0x19, 0xa4, 0x5d);

// {5E467F2E-77CA-4CA5-8391-D142ED4B76C8}      MFSubtitleFormat_SRT
DEFINE_GUID(MFSubtitleFormat_SRT,
    0x5e467f2e, 0x77ca, 0x4ca5, 0x83, 0x91, 0xd1, 0x42, 0xed, 0x4b, 0x76, 0xc8);

// {57176A1B-1A9E-4EEA-ABEF-C61760198AC4}      MFSubtitleFormat_SSA
DEFINE_GUID(MFSubtitleFormat_SSA,
    0x57176a1b, 0x1a9e, 0x4eea, 0xab, 0xef, 0xc6, 0x17, 0x60, 0x19, 0x8a, 0xc4);

// {1BB3D849-6614-4D80-8882-ED24AA82DA92}      MFSubtitleFormat_CustomUserData
DEFINE_GUID(MFSubtitleFormat_CustomUserData,
    0x1bb3d849, 0x6614, 0x4d80, 0x88, 0x82, 0xed, 0x24, 0xaa, 0x82, 0xda, 0x92);

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

// {71F40E4A-1278-4442-B30D-39DD1D7722BC}      MFSubtitleFormat_PGS
DEFINE_GUID(MFSubtitleFormat_PGS,
    0x71f40e4a, 0x1278, 0x4442, 0xb3, 0x0d, 0x39, 0xdd, 0x1d, 0x77, 0x22, 0xbc);

// {6B8E40F4-8D2C-4CED-AD91-5960E45B4433}      MFSubtitleFormat_VobSub
DEFINE_GUID(MFSubtitleFormat_VobSub,
    0x6b8e40f4, 0x8d2c, 0x4ced, 0xad, 0x91, 0x59, 0x60, 0xe4, 0x5b, 0x44, 0x33);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

//
// Binary Data MediaTypes
//

#ifndef DEFINE_BINARY_MEDIATYPE_GUID
#define DEFINE_BINARY_MEDIATYPE_GUID(name, format) \
    DEFINE_GUID(name,                       \
    format, 0xbf10, 0x48b4, 0xbc, 0x18, 0x59, 0x3d, 0xc1, 0xdb, 0x95, 0xf);
#endif

DEFINE_BINARY_MEDIATYPE_GUID(MFBinaryFormat_Base, 0x00000000);
DEFINE_BINARY_MEDIATYPE_GUID(MFBinaryFormat_GPMD, 'gpmd');



///////////////////////////////////////////////////////////////////////////////////////////////////////////////  Media Type Attributes GUIDs ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// GUIDs for IMFMediaType properties - prefix 'MF_MT_' - basic prop type in {},
// with type to cast to in ().
//


//
// core info for all types
//
// {48eba18e-f8c9-4687-bf11-0a74c9f96a8f}   MF_MT_MAJOR_TYPE                {GUID}
DEFINE_GUID(MF_MT_MAJOR_TYPE,
0x48eba18e, 0xf8c9, 0x4687, 0xbf, 0x11, 0x0a, 0x74, 0xc9, 0xf9, 0x6a, 0x8f);

// {f7e34c9a-42e8-4714-b74b-cb29d72c35e5}   MF_MT_SUBTYPE                   {GUID}
DEFINE_GUID(MF_MT_SUBTYPE,
0xf7e34c9a, 0x42e8, 0x4714, 0xb7, 0x4b, 0xcb, 0x29, 0xd7, 0x2c, 0x35, 0xe5);

// {c9173739-5e56-461c-b713-46fb995cb95f}   MF_MT_ALL_SAMPLES_INDEPENDENT   {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_ALL_SAMPLES_INDEPENDENT,
0xc9173739, 0x5e56, 0x461c, 0xb7, 0x13, 0x46, 0xfb, 0x99, 0x5c, 0xb9, 0x5f);

// {b8ebefaf-b718-4e04-b0a9-116775e3321b}   MF_MT_FIXED_SIZE_SAMPLES        {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_FIXED_SIZE_SAMPLES,
0xb8ebefaf, 0xb718, 0x4e04, 0xb0, 0xa9, 0x11, 0x67, 0x75, 0xe3, 0x32, 0x1b);

// {3afd0cee-18f2-4ba5-a110-8bea502e1f92}   MF_MT_COMPRESSED                {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_COMPRESSED,
0x3afd0cee, 0x18f2, 0x4ba5, 0xa1, 0x10, 0x8b, 0xea, 0x50, 0x2e, 0x1f, 0x92);

//
// MF_MT_SAMPLE_SIZE is only valid if MF_MT_FIXED_SIZED_SAMPLES is TRUE
//
// {dad3ab78-1990-408b-bce2-eba673dacc10}   MF_MT_SAMPLE_SIZE               {UINT32}
DEFINE_GUID(MF_MT_SAMPLE_SIZE,
0xdad3ab78, 0x1990, 0x408b, 0xbc, 0xe2, 0xeb, 0xa6, 0x73, 0xda, 0xcc, 0x10);

// 4d3f7b23-d02f-4e6c-9bee-e4bf2c6c695d     MF_MT_WRAPPED_TYPE              {Blob}
DEFINE_GUID(MF_MT_WRAPPED_TYPE,
0x4d3f7b23, 0xd02f, 0x4e6c, 0x9b, 0xee, 0xe4, 0xbf, 0x2c, 0x6c, 0x69, 0x5d);

#if (WINVER >= _WIN32_WINNT_WIN8)

//
// Media Type & Sample attributes for 3D Video
//

// {CB5E88CF-7B5B-476b-85AA-1CA5AE187555}        MF_MT_VIDEO_3D                 {UINT32 (BOOL)}
DEFINE_GUID( MF_MT_VIDEO_3D, 
0xcb5e88cf, 0x7b5b, 0x476b, 0x85, 0xaa, 0x1c, 0xa5, 0xae, 0x18, 0x75, 0x55);

// Enum describing the packing for 3D video frames
typedef enum _MFVideo3DFormat {
    MFVideo3DSampleFormat_BaseView              = 0,
    MFVideo3DSampleFormat_MultiView             = 1,
    MFVideo3DSampleFormat_Packed_LeftRight      = 2,
    MFVideo3DSampleFormat_Packed_TopBottom      = 3,
} MFVideo3DFormat;

// {5315d8a0-87c5-4697-b793-666c67c49b}         MF_MT_VIDEO_3D_FORMAT           {UINT32 (anyof MFVideo3DFormat)}
DEFINE_GUID(MF_MT_VIDEO_3D_FORMAT, 
0x5315d8a0, 0x87c5, 0x4697, 0xb7, 0x93, 0x66, 0x6, 0xc6, 0x7c, 0x4, 0x9b);

// {BB077E8A-DCBF-42eb-AF60-418DF98AA495}       MF_MT_VIDEO_3D_NUM_VIEW         {UINT32}
DEFINE_GUID( MF_MT_VIDEO_3D_NUM_VIEWS, 
0xbb077e8a, 0xdcbf, 0x42eb, 0xaf, 0x60, 0x41, 0x8d, 0xf9, 0x8a, 0xa4, 0x95);

// {6D4B7BFF-5629-4404-948C-C634F4CE26D4}       MF_MT_VIDEO_3D_LEFT_IS_BASE     {UINT32}
DEFINE_GUID( MF_MT_VIDEO_3D_LEFT_IS_BASE,
0x6d4b7bff, 0x5629, 0x4404, 0x94, 0x8c, 0xc6, 0x34, 0xf4, 0xce, 0x26, 0xd4);

// {EC298493-0ADA-4ea1-A4FE-CBBD36CE9331}       MF_MT_VIDEO_3D_FIRST_IS_LEFT    {UINT32 (BOOL)}
DEFINE_GUID( MF_MT_VIDEO_3D_FIRST_IS_LEFT, 
0xec298493, 0xada, 0x4ea1, 0xa4, 0xfe, 0xcb, 0xbd, 0x36, 0xce, 0x93, 0x31);


// MFSampleExtension_3DVideo                    {F86F97A4-DD54-4e2e-9A5E-55FC2D74A005}
// Type: UINT32
// If present and nonzero, indicates that the sample contains 3D Video data
DEFINE_GUID( MFSampleExtension_3DVideo, 
0xf86f97a4, 0xdd54, 0x4e2e, 0x9a, 0x5e, 0x55, 0xfc, 0x2d, 0x74, 0xa0, 0x05);

// Enum describing the packing for 3D video frames in a sample
typedef enum _MFVideo3DSampleFormat {
    MFSampleExtension_3DVideo_MultiView         = 1,
    MFSampleExtension_3DVideo_Packed            = 0,
} MFVideo3DSampleFormat;

// MFSampleExtension_3DVideo_SampleFormat       {08671772-E36F-4cff-97B3-D72E20987A48}
// Type: UINT32
// The value of this attribute is a member of the MFVideo3DSampleFormat enumeration.
// MFVideo3DSampleFormat enumeration identifies how 3D views are stored in the sample
//      - in a packed representation, all views are stored in a single buffer
//      - in a multiview representation, each view is stored in its own buffer
DEFINE_GUID( MFSampleExtension_3DVideo_SampleFormat, 
0x8671772, 0xe36f, 0x4cff, 0x97, 0xb3, 0xd7, 0x2e, 0x20, 0x98, 0x7a, 0x48);

// Enum describing the video rotation formats
// Only the values of 0, 90, 180, and 270 are valid.
#ifndef _MFVideoRotationFormat_
#define _MFVideoRotationFormat_
typedef enum _MFVideoRotationFormat {
    MFVideoRotationFormat_0        = 0,
    MFVideoRotationFormat_90       = 90,
    MFVideoRotationFormat_180      = 180,
    MFVideoRotationFormat_270      = 270,
} MFVideoRotationFormat;
#endif

// MF_MT_VIDEO_ROTATION      {C380465D-2271-428C-9B83-ECEA3B4A85C1}
// Type: UINT32
// Description: MF_MT_VIDEO_ROTATION attribute means the degree that the content
// has already been rotated in the counter clockwise direction.
// Currently, only the values of 0, 90, 180, and 270 are valid for MF_MT_VIDEO_ROTATION.
// For convenience, these currently supported values are enumerated in MFVideoRotationFormat.
// Example: if the media type has MF_MT_VIDEO_ROTATION set as MFVideoRotationFormat_90,
// it means the content has been rotated 90 degree in the counter clockwise direction.
// If the content was actually rotated 90 degree in the clockwise direction, 90 degree in
// clockwise should be converted into 270 degree in the counter clockwise direction and set
// the attribute MF_MT_VIDEO_ROTATION as MFVideoRotationFormat_270 accordingly.
DEFINE_GUID(MF_MT_VIDEO_ROTATION,
0xc380465d, 0x2271, 0x428c, 0x9b, 0x83, 0xec, 0xea, 0x3b, 0x4a, 0x85, 0xc1);

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
DEFINE_GUID(MF_DEVICESTREAM_MULTIPLEXED_MANAGER,
0x6ea542b0, 0x281f, 0x4231, 0xa4, 0x64, 0xfe, 0x2f, 0x50, 0x22, 0x50, 0x1c);    

DEFINE_GUID(MF_MEDIATYPE_MULTIPLEXED_MANAGER,
0x13c78fb5, 0xf275, 0x4ea0, 0xbb, 0x5f, 0x2, 0x49, 0x83, 0x2b, 0xd, 0x6e);

DEFINE_GUID(MFSampleExtension_MULTIPLEXED_MANAGER,
0x8dcdee79, 0x6b5a, 0x4c45, 0x8d, 0xb9, 0x20, 0xb3, 0x95, 0xf0, 0x2f, 0xcf);


STDAPI MFCreateMuxStreamAttributes(
    _In_ IMFCollection *pAttributesToMux,
    _COM_Outptr_ IMFAttributes**ppMuxAttribs
);

STDAPI MFCreateMuxStreamMediaType(
    _In_ IMFCollection *pMediaTypesToMux,
    _COM_Outptr_ IMFMediaType**ppMuxMediaType
);

STDAPI MFCreateMuxStreamSample(
    _In_ IMFCollection *pSamplesToMux,
    _COM_Outptr_ IMFSample**ppMuxSample
);
#endif

#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD)
// MF_MT_SECURE     {c5acc4fd-0304-4ecf-809f-47bc97ff63bd }
// Type: UINT32 (BOOL)
// Description: MF_MT_SECURE attribute indicates that the content will be using
// secure D3D surfaces.  These surfaces can only be accessed by trusted hardware.
DEFINE_GUID(MF_MT_SECURE,
0xc5acc4fd, 0x0304, 0x4ecf, 0x80, 0x9f, 0x47, 0xbc, 0x97, 0xff, 0x63, 0xbd);


// MF_DEVICESTREAM_ATTRIBUTE_FRAMESOURCE_TYPES {17145FD1-1B2B-423C-8001-2B6833ED3588}
// Type: UINT32 (enum type defined in MFFrameSourceTypes)
// Description: The value of this attribute is a enum value, describing the sensor types.  
// For backward compatibility, when this attribute was not defined on in a media type, it is assumed to be MFFrameSourceTypes::Color.
DEFINE_GUID(MF_DEVICESTREAM_ATTRIBUTE_FRAMESOURCE_TYPES,
0x17145fd1, 0x1b2b, 0x423c, 0x80, 0x1, 0x2b, 0x68, 0x33, 0xed, 0x35, 0x88);

// MF_MT_ALPHA_MODE {5D959B0D-4CBF-4D04-919F-3F5F7F284211}
// Type: UINT32
// Description: To differentiate the usage of alpha channel in such video formats, a new attribute MF_MT_ALPHA_MODE is designed to describe this information.
// The value of this attribute can be cast to DXGI_ALPHA_MODE.
// If this attribute is not present, for backward compatibility, the value is DXGI_ALPHA_MODE_STRAIGHT for video format supporting alpha channel,
// such as ARGB32, or DXGI_ALPHA_MODE_IGNORE for video format without alpha channel, such as RGB32.
DEFINE_GUID(MF_MT_ALPHA_MODE,
0x5D959B0D, 0x4CBF, 0x4D04, 0x91, 0x9F, 0x3F, 0x5F, 0x7F, 0x28, 0x42, 0x11);

typedef enum _MFDepthMeasurement
{
    DistanceToFocalPlane        = 0,
    DistanceToOpticalCenter     = 1,
} MFDepthMeasurement;

// MF_MT_DEPTH_MEASUREMENT {FD5AC489-0917-4BB6-9D54-3122BF70144B}
// Type : UINT32  (MFDepthMeasurement)
// Description: If this attribute is not present, by default it is DistanceToFocalPlane, illustrated by following diagram.
DEFINE_GUID(MF_MT_DEPTH_MEASUREMENT,
0xfd5ac489, 0x917, 0x4bb6, 0x9d, 0x54, 0x31, 0x22, 0xbf, 0x70, 0x14, 0x4b);

// MF_MT_DEPTH_VALUE_UNIT    {21a800f5-3189-4797-beba-f13cd9a31a5e}
// Type : UINT64
// Description: MF_MT_DEPTH_VALUE_UNIT attribute indicates scale of the depth value in nanometers.  
// For each pixel in depth frame, the actual depth measured in nanometers is the pixel value multiplied by this attribute.
DEFINE_GUID(MF_MT_DEPTH_VALUE_UNIT,
0x21a800f5, 0x3189, 0x4797, 0xbe, 0xba, 0xf1, 0x3c, 0xd9, 0xa3, 0x1a, 0x5e);

#endif

// MF_MT_VIDEO_NO_FRAME_ORDERING {3F5B106F-6BC2-4EE3-B7ED-8902C18F5351}
// Type: UINT32
// Description: MF_MT_VIDEO_NO_FRAME_ORDERING set to non-zero (true) means external users/apps know 
// that input video bitstream has no frame rerodering,
// that is, the output and display order is the same as the input and decoding order
// it will overwrite bitstream syntaxes even if bitstream syntaxes do not indicate 
// that the output and display order is the same as the input and decoding order
//
// it is an attribute set on input media type
//
DEFINE_GUID(MF_MT_VIDEO_NO_FRAME_ORDERING,
    0x3f5b106f, 0x6bc2, 0x4ee3, 0xb7, 0xed, 0x89, 0x2, 0xc1, 0x8f, 0x53, 0x51);


// MF_MT_VIDEO_H264_NO_FMOASO {ED461CD6-EC9F-416A-A8A3-26D7D31018D7}
// Type: UINT32
// Description: MF_MT_VIDEO_H264_NO_FMOASO set to non-zero (true) means external users/apps know 
// that H.264 input video bitstream has no FMO/ASO enabled,
// that is, even if the bitstream has baseline profile and constraint_set1_flag equal to 0, 
// the bitstream shall not have FMO/ASO
// then H.264 decoder uses DXVA decoding and doesn't fall back to software decoding
// it improves power consumption, memory usage, performance and user experiences 
// (without unnecessary glitches on low end devices)
//
// it is an attribute set on input media type
//
DEFINE_GUID(MF_MT_VIDEO_H264_NO_FMOASO,
    0xed461cd6, 0xec9f, 0x416a, 0xa8, 0xa3, 0x26, 0xd7, 0xd3, 0x10, 0x18, 0xd7);


#endif // (WINVER >= _WIN32_WINNT_WIN8)

#if (NTDDI_VERSION >= NTDDI_WIN10_RS2) 

// 
// Renderer Extensions
//

// MFSampleExtension_ForwardedDecodeUnits {424C754C-97C8-48d6-8777-FC41F7B60879} 
// Type: IUnknown  
// This is an object of type IMFCollection containing IMFSample objects  
//  which contain NALU/SEI forwarded by a decoder.  
//  Contains all custom NALU/SEI since previous frame with emulation prevention bytes removed.  
// see: MF_MT_FORWARD_CUSTOM_NALU, MF_MT_FORWARD_CUSTOM_SEI 
DEFINE_GUID(MFSampleExtension_ForwardedDecodeUnits,
0x424c754c, 0x97c8, 0x48d6, 0x87, 0x77, 0xfc, 0x41, 0xf7, 0xb6, 0x8, 0x79);

// MFSampleExtension_TargetGlobalLuminance {3F60EF36-31EF-4daf-8360-940397E41EF3} 
// Type: UINT32 
// Value in Nits that specifies the targeted global backlight luminance for 
//  the associated video frame. 
DEFINE_GUID(MFSampleExtension_TargetGlobalLuminance,
0x3f60ef36, 0x31ef, 0x4daf, 0x83, 0x60, 0x94, 0x3, 0x97, 0xe4, 0x1e, 0xf3);

typedef enum _MF_CUSTOM_DECODE_UNIT_TYPE
{
    MF_DECODE_UNIT_NAL = 0,
    MF_DECODE_UNIT_SEI = 1
} MF_CUSTOM_DECODE_UNIT_TYPE;

// MFSampleExtension_ForwardedDecodeUnitType {089E57C7-47D3-4a26-BF9C-4B64FAFB5D1E} 
// Type: UINT32 (oneof MF_CUSTOM_DECODE_UNIT_TYPE) 
// Attached to IMFSample objects in MFSampleExtension_ForwardedDecodeUnits, specifies 
//  what type of unit is attached: SEI or NAL 
DEFINE_GUID(MFSampleExtension_ForwardedDecodeUnitType,
0x89e57c7, 0x47d3, 0x4a26, 0xbf, 0x9c, 0x4b, 0x64, 0xfa, 0xfb, 0x5d, 0x1e);

// MF_MT_FORWARD_CUSTOM_NALU {ED336EFD-244F-428d-9153-28F399458890} 
// Type: UINT32  
// Specifies the NAL unit type to forward on output samples of the decoder. 
// If the decoder parses the specified NALU then it will not forwarded. 
// See: MFSampleExtension_ForwardedDecodeUnits 
DEFINE_GUID(MF_MT_FORWARD_CUSTOM_NALU,
0xed336efd, 0x244f, 0x428d, 0x91, 0x53, 0x28, 0xf3, 0x99, 0x45, 0x88, 0x90);

// MF_MT_FORWARD_CUSTOM_SEI {E27362F1-B136-41d1-9594-3A7E4FEBF2D1} 
// Type: UINT32  
// Specifies the SEI type to forward on output samples of the decoder 
// If the decoder parses the specified SEI then it will not be forwarded. 
// See: MFSampleExtension_ForwardedDecodeUnits 
DEFINE_GUID(MF_MT_FORWARD_CUSTOM_SEI,
0xe27362f1, 0xb136, 0x41d1, 0x95, 0x94, 0x3a, 0x7e, 0x4f, 0xeb, 0xf2, 0xd1);

// MF_MT_VIDEO_RENDERER_EXTENSION_PROFILE {8437D4B9-D448-4fcd-9B6B-839BF96C7798} 
// Type: LPCWSTR  
// Contains a string that matches an entry in a MediaRendererEffect Manifest's  
//  VideoRendererExtensionProfiles list to select which effect to load 
DEFINE_GUID(MF_MT_VIDEO_RENDERER_EXTENSION_PROFILE,
0x8437d4b9, 0xd448, 0x4fcd, 0x9b, 0x6b, 0x83, 0x9b, 0xf9, 0x6c, 0x77, 0x98);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2) 

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4) 

// MF_DECODER_FWD_CUSTOM_SEI_DECODE_ORDER {f13bbe3c-36d4-410a-b985-7a951a1e6294}  
// Type: UINT32  
// Specifies that the SEI unit type to forward on output samples of the decoder  
// shall be sent out in decode order (i.e. ahead of time)  
// This is required for downstream apps to process the SEI in advance of receiving  
// the frame it is meant to be attached to  
DEFINE_GUID(MF_DECODER_FWD_CUSTOM_SEI_DECODE_ORDER, 0xf13bbe3c, 0x36d4, 0x410a, 0xb9, 0x85, 0x7a, 0x95, 0x1a, 0x1e, 0x62, 0x94);

#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS4) */ 

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

// {C6052A80-6D9C-40a3-9DB8-F027A25C9AB9} 
// Type: String 
// Name of the App Service, as defined in the AppX manifest of the Package that contains this Video 
// Renderer Effect. 
// This attribute is specified by the Video Renderer Effect to request that the platforms establish 
// a communication channel with the Video Renderer Effect's App Service. 
DEFINE_GUID(MF_VIDEO_RENDERER_EFFECT_APP_SERVICE_NAME,
    0xc6052a80, 0x6d9c, 0x40a3, 0x9d, 0xb8, 0xf0, 0x27, 0xa2, 0x5c, 0x9a, 0xb9);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

//
// AUDIO data
//

// {37e48bf5-645e-4c5b-89de-ada9e29b696a}   MF_MT_AUDIO_NUM_CHANNELS            {UINT32}
DEFINE_GUID(MF_MT_AUDIO_NUM_CHANNELS,
0x37e48bf5, 0x645e, 0x4c5b, 0x89, 0xde, 0xad, 0xa9, 0xe2, 0x9b, 0x69, 0x6a);

// {5faeeae7-0290-4c31-9e8a-c534f68d9dba}   MF_MT_AUDIO_SAMPLES_PER_SECOND      {UINT32}
DEFINE_GUID(MF_MT_AUDIO_SAMPLES_PER_SECOND,
0x5faeeae7, 0x0290, 0x4c31, 0x9e, 0x8a, 0xc5, 0x34, 0xf6, 0x8d, 0x9d, 0xba);

// {fb3b724a-cfb5-4319-aefe-6e42b2406132}   MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND {double}
DEFINE_GUID(MF_MT_AUDIO_FLOAT_SAMPLES_PER_SECOND,
0xfb3b724a, 0xcfb5, 0x4319, 0xae, 0xfe, 0x6e, 0x42, 0xb2, 0x40, 0x61, 0x32);

// {1aab75c8-cfef-451c-ab95-ac034b8e1731}   MF_MT_AUDIO_AVG_BYTES_PER_SECOND    {UINT32}
DEFINE_GUID(MF_MT_AUDIO_AVG_BYTES_PER_SECOND,
0x1aab75c8, 0xcfef, 0x451c, 0xab, 0x95, 0xac, 0x03, 0x4b, 0x8e, 0x17, 0x31);

// {322de230-9eeb-43bd-ab7a-ff412251541d}   MF_MT_AUDIO_BLOCK_ALIGNMENT         {UINT32}
DEFINE_GUID(MF_MT_AUDIO_BLOCK_ALIGNMENT,
0x322de230, 0x9eeb, 0x43bd, 0xab, 0x7a, 0xff, 0x41, 0x22, 0x51, 0x54, 0x1d);

// {f2deb57f-40fa-4764-aa33-ed4f2d1ff669}   MF_MT_AUDIO_BITS_PER_SAMPLE         {UINT32}
DEFINE_GUID(MF_MT_AUDIO_BITS_PER_SAMPLE,
0xf2deb57f, 0x40fa, 0x4764, 0xaa, 0x33, 0xed, 0x4f, 0x2d, 0x1f, 0xf6, 0x69);

// {d9bf8d6a-9530-4b7c-9ddf-ff6fd58bbd06}   MF_MT_AUDIO_VALID_BITS_PER_SAMPLE   {UINT32}
DEFINE_GUID(MF_MT_AUDIO_VALID_BITS_PER_SAMPLE,
0xd9bf8d6a, 0x9530, 0x4b7c, 0x9d, 0xdf, 0xff, 0x6f, 0xd5, 0x8b, 0xbd, 0x06);

// {aab15aac-e13a-4995-9222-501ea15c6877}   MF_MT_AUDIO_SAMPLES_PER_BLOCK       {UINT32}
DEFINE_GUID(MF_MT_AUDIO_SAMPLES_PER_BLOCK,
0xaab15aac, 0xe13a, 0x4995, 0x92, 0x22, 0x50, 0x1e, 0xa1, 0x5c, 0x68, 0x77);

// {55fb5765-644a-4caf-8479-938983bb1588}`  MF_MT_AUDIO_CHANNEL_MASK            {UINT32}
DEFINE_GUID(MF_MT_AUDIO_CHANNEL_MASK,
0x55fb5765, 0x644a, 0x4caf, 0x84, 0x79, 0x93, 0x89, 0x83, 0xbb, 0x15, 0x88);

//
// MF_MT_AUDIO_FOLDDOWN_MATRIX stores folddown structure from multichannel to stereo
//
typedef struct _MFFOLDDOWN_MATRIX
{
    UINT32 cbSize;
    UINT32 cSrcChannels; // number of source channels
    UINT32 cDstChannels; // number of destination channels
    UINT32 dwChannelMask; // mask
    LONG Coeff[64];
} MFFOLDDOWN_MATRIX;

// {9d62927c-36be-4cf2-b5c4-a3926e3e8711}`  MF_MT_AUDIO_FOLDDOWN_MATRIX         {BLOB, MFFOLDDOWN_MATRIX}
DEFINE_GUID(MF_MT_AUDIO_FOLDDOWN_MATRIX,
0x9d62927c, 0x36be, 0x4cf2, 0xb5, 0xc4, 0xa3, 0x92, 0x6e, 0x3e, 0x87, 0x11);

// {0x9d62927d-36be-4cf2-b5c4-a3926e3e8711}`  MF_MT_AUDIO_WMADRC_PEAKREF         {UINT32}
DEFINE_GUID(MF_MT_AUDIO_WMADRC_PEAKREF,
0x9d62927d, 0x36be, 0x4cf2, 0xb5, 0xc4, 0xa3, 0x92, 0x6e, 0x3e, 0x87, 0x11);

// {0x9d62927e-36be-4cf2-b5c4-a3926e3e8711}`  MF_MT_AUDIO_WMADRC_PEAKTARGET        {UINT32}
DEFINE_GUID(MF_MT_AUDIO_WMADRC_PEAKTARGET,
0x9d62927e, 0x36be, 0x4cf2, 0xb5, 0xc4, 0xa3, 0x92, 0x6e, 0x3e, 0x87, 0x11);


// {0x9d62927f-36be-4cf2-b5c4-a3926e3e8711}`  MF_MT_AUDIO_WMADRC_AVGREF         {UINT32}
DEFINE_GUID(MF_MT_AUDIO_WMADRC_AVGREF,
0x9d62927f, 0x36be, 0x4cf2, 0xb5, 0xc4, 0xa3, 0x92, 0x6e, 0x3e, 0x87, 0x11);

// {0x9d629280-36be-4cf2-b5c4-a3926e3e8711}`  MF_MT_AUDIO_WMADRC_AVGTARGET      {UINT32}
DEFINE_GUID(MF_MT_AUDIO_WMADRC_AVGTARGET,
0x9d629280, 0x36be, 0x4cf2, 0xb5, 0xc4, 0xa3, 0x92, 0x6e, 0x3e, 0x87, 0x11);

//
// MF_MT_AUDIO_PREFER_WAVEFORMATEX tells the converter to prefer a plain WAVEFORMATEX rather than
// a WAVEFORMATEXTENSIBLE when converting to a legacy type. It is set by the WAVEFORMATEX->IMFMediaType
// conversion routines when the original format block is a non-extensible WAVEFORMATEX.
//
// This preference can be overridden and does not guarantee that the type can be correctly expressed
// by a non-extensible type.
//
// {a901aaba-e037-458a-bdf6-545be2074042}   MF_MT_AUDIO_PREFER_WAVEFORMATEX     {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_AUDIO_PREFER_WAVEFORMATEX,
0xa901aaba, 0xe037, 0x458a, 0xbd, 0xf6, 0x54, 0x5b, 0xe2, 0x07, 0x40, 0x42);

#if (WINVER >= _WIN32_WINNT_WIN7)
//
// AUDIO - AAC extra data
//

// {BFBABE79-7434-4d1c-94F0-72A3B9E17188} MF_MT_AAC_PAYLOAD_TYPE       {UINT32}
DEFINE_GUID(MF_MT_AAC_PAYLOAD_TYPE,
0xbfbabe79, 0x7434, 0x4d1c, 0x94, 0xf0, 0x72, 0xa3, 0xb9, 0xe1, 0x71, 0x88);

// {7632F0E6-9538-4d61-ACDA-EA29C8C14456} MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION       {UINT32}
DEFINE_GUID(MF_MT_AAC_AUDIO_PROFILE_LEVEL_INDICATION,
0x7632f0e6, 0x9538, 0x4d61, 0xac, 0xda, 0xea, 0x29, 0xc8, 0xc1, 0x44, 0x56);

#endif // (WINVER >= _WIN32_WINNT_WIN7)

#if (WINVER >= _WIN32_WINNT_WIN10)
//
// AUDIO - FLAC extra data
//

// {8B81ADAE-4B5A-4D40-8022-F38D09CA3C5C} MF_MT_AUDIO_FLAC_MAX_BLOCK_SIZE       {UINT32}
DEFINE_GUID(MF_MT_AUDIO_FLAC_MAX_BLOCK_SIZE,
    0x8b81adae, 0x4b5a, 0x4d40, 0x80, 0x22, 0xf3, 0x8d, 0x9, 0xca, 0x3c, 0x5c);

#endif // (WINVER >= _WIN32_WINNT_WIN10)


#if (NTDDI_VERSION >= NTDDI_WIN10_RS2)
//
// AUDIO - Spatial Audio Sample extra data
//

// {DCFBA24A-2609-4240-A721-3FAEA76A4DF9} MF_MT_SPATIAL_AUDIO_MAX_DYNAMIC_OBJECTS     {UINT32}
DEFINE_GUID( MF_MT_SPATIAL_AUDIO_MAX_DYNAMIC_OBJECTS,
    0xdcfba24a, 0x2609, 0x4240, 0xa7, 0x21, 0x3f, 0xae, 0xa7, 0x6a, 0x4d, 0xf9 );

// {2AB71BC0-6223-4BA7-AD64-7B94B47AE792} MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_FORMAT_ID     {GUID}
DEFINE_GUID( MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_FORMAT_ID,
    0x2ab71bc0, 0x6223, 0x4ba7, 0xad, 0x64, 0x7b, 0x94, 0xb4, 0x7a, 0xe7, 0x92 );

// {094BA8BE-D723-489F-92FA-766777B34726} MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_LENGTH  {UINT32}
DEFINE_GUID( MF_MT_SPATIAL_AUDIO_OBJECT_METADATA_LENGTH,
    0x94ba8be, 0xd723, 0x489f, 0x92, 0xfa, 0x76, 0x67, 0x77, 0xb3, 0x47, 0x26 );

// {11AA80B4-E0DA-47C6-8060-96C1259AE50D} MF_MT_SPATIAL_AUDIO_MAX_METADATA_ITEMS {UINT32}
DEFINE_GUID( MF_MT_SPATIAL_AUDIO_MAX_METADATA_ITEMS,
    0x11aa80b4, 0xe0da, 0x47c6, 0x80, 0x60, 0x96, 0xc1, 0x25, 0x9a, 0xe5, 0xd );

// {83E96EC9-1184-417E-8254-9F269158FC06} MF_MT_SPATIAL_AUDIO_MIN_METADATA_ITEM_OFFSET_SPACING {UINT32}
DEFINE_GUID( MF_MT_SPATIAL_AUDIO_MIN_METADATA_ITEM_OFFSET_SPACING,
    0x83e96ec9, 0x1184, 0x417e, 0x82, 0x54, 0x9f, 0x26, 0x91, 0x58, 0xfc, 0x6 );

// {6842F6E7-D43E-4EBB-9C9C-C96F41784863} MF_MT_SPATIAL_AUDIO_DATA_PRESENT {UINT32 (BOOL)}
DEFINE_GUID( MF_MT_SPATIAL_AUDIO_DATA_PRESENT, 
    0x6842f6e7, 0xd43e, 0x4ebb, 0x9c, 0x9c, 0xc9, 0x6f, 0x41, 0x78, 0x48, 0x63 );

// {4EACAB51-FFE5-421A-A2A7-8B7409A1CAC4} MF_MT_SPATIAL_AUDIO_IS_PREVIRTUALIZED {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_SPATIAL_AUDIO_IS_PREVIRTUALIZED,
    0x4eacab51, 0xffe5, 0x421a, 0xa2, 0xa7, 0x8b, 0x74, 0x09, 0xa1, 0xca, 0xc4 );

// {51267a39-dd0c-4bb9-a7bd-9173ad4b131c}
DEFINE_GUID(MF_MT_MPEGH_AUDIO_PROFILE_LEVEL_INDICATION,
0x51267a39, 0xdd0c, 0x4bb9, 0xa7, 0xbd, 0x91, 0x73, 0xad, 0x4b, 0x13, 0x1c);

#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS2)


//
// VIDEO core data
//

// {1652c33d-d6b2-4012-b834-72030849a37d}   MF_MT_FRAME_SIZE                {UINT64 (HI32(Width),LO32(Height))}
DEFINE_GUID(MF_MT_FRAME_SIZE,
0x1652c33d, 0xd6b2, 0x4012, 0xb8, 0x34, 0x72, 0x03, 0x08, 0x49, 0xa3, 0x7d);

// {c459a2e8-3d2c-4e44-b132-fee5156c7bb0}   MF_MT_FRAME_RATE                {UINT64 (HI32(Numerator),LO32(Denominator))}
DEFINE_GUID(MF_MT_FRAME_RATE,
0xc459a2e8, 0x3d2c, 0x4e44, 0xb1, 0x32, 0xfe, 0xe5, 0x15, 0x6c, 0x7b, 0xb0);

// {c6376a1e-8d0a-4027-be45-6d9a0ad39bb6}   MF_MT_PIXEL_ASPECT_RATIO        {UINT64 (HI32(Numerator),LO32(Denominator))}
DEFINE_GUID(MF_MT_PIXEL_ASPECT_RATIO,
0xc6376a1e, 0x8d0a, 0x4027, 0xbe, 0x45, 0x6d, 0x9a, 0x0a, 0xd3, 0x9b, 0xb6);

// {8772f323-355a-4cc7-bb78-6d61a048ae82}   MF_MT_DRM_FLAGS                 {UINT32 (anyof MFVideoDRMFlags)}
DEFINE_GUID(MF_MT_DRM_FLAGS,
0x8772f323, 0x355a, 0x4cc7, 0xbb, 0x78, 0x6d, 0x61, 0xa0, 0x48, 0xae, 0x82);

#if (WINVER >= _WIN32_WINNT_WIN8)

// {24974215-1B7B-41e4-8625-AC469F2DEDAA}   MF_MT_TIMESTAMP_CAN_BE_DTS      {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_TIMESTAMP_CAN_BE_DTS, 
0x24974215, 0x1b7b, 0x41e4, 0x86, 0x25, 0xac, 0x46, 0x9f, 0x2d, 0xed, 0xaa);

#endif // (WINVER >= _WIN32_WINNT_WIN8)

typedef enum _MFVideoDRMFlags {
    MFVideoDRMFlag_None                 = 0,
    MFVideoDRMFlag_AnalogProtected      = 1,
    MFVideoDRMFlag_DigitallyProtected   = 2,
} MFVideoDRMFlags;


// {4d0e73e5-80ea-4354-a9d0-1176ceb028ea}   MF_MT_PAD_CONTROL_FLAGS         {UINT32 (oneof MFVideoPadFlags)}
DEFINE_GUID(MF_MT_PAD_CONTROL_FLAGS,
0x4d0e73e5, 0x80ea, 0x4354, 0xa9, 0xd0, 0x11, 0x76, 0xce, 0xb0, 0x28, 0xea);

typedef enum _MFVideoPadFlags {
    MFVideoPadFlag_PAD_TO_None  = 0,
    MFVideoPadFlag_PAD_TO_4x3   = 1,
    MFVideoPadFlag_PAD_TO_16x9  = 2
} MFVideoPadFlags;

// {68aca3cc-22d0-44e6-85f8-28167197fa38}   MF_MT_SOURCE_CONTENT_HINT       {UINT32 (oneof MFVideoSrcContentHintFlags)}
DEFINE_GUID(MF_MT_SOURCE_CONTENT_HINT,
0x68aca3cc, 0x22d0, 0x44e6, 0x85, 0xf8, 0x28, 0x16, 0x71, 0x97, 0xfa, 0x38);

typedef enum _MFVideoSrcContentHintFlags {
    MFVideoSrcContentHintFlag_None  = 0,
    MFVideoSrcContentHintFlag_16x9  = 1,
    MFVideoSrcContentHintFlag_235_1 = 2
} MFVideoSrcContentHintFlags;

// {65df2370-c773-4c33-aa64-843e068efb0c}   MF_MT_CHROMA_SITING             {UINT32 (anyof MFVideoChromaSubsampling)}
DEFINE_GUID(MF_MT_VIDEO_CHROMA_SITING,
0x65df2370, 0xc773, 0x4c33, 0xaa, 0x64, 0x84, 0x3e, 0x06, 0x8e, 0xfb, 0x0c);

// {e2724bb8-e676-4806-b4b2-a8d6efb44ccd}   MF_MT_INTERLACE_MODE            {UINT32 (oneof MFVideoInterlaceMode)}
DEFINE_GUID(MF_MT_INTERLACE_MODE,
0xe2724bb8, 0xe676, 0x4806, 0xb4, 0xb2, 0xa8, 0xd6, 0xef, 0xb4, 0x4c, 0xcd);

// {5fb0fce9-be5c-4935-a811-ec838f8eed93}   MF_MT_TRANSFER_FUNCTION         {UINT32 (oneof MFVideoTransferFunction)}
DEFINE_GUID(MF_MT_TRANSFER_FUNCTION,
0x5fb0fce9, 0xbe5c, 0x4935, 0xa8, 0x11, 0xec, 0x83, 0x8f, 0x8e, 0xed, 0x93);

// {dbfbe4d7-0740-4ee0-8192-850ab0e21935}   MF_MT_VIDEO_PRIMARIES           {UINT32 (oneof MFVideoPrimaries)}
DEFINE_GUID(MF_MT_VIDEO_PRIMARIES,
0xdbfbe4d7, 0x0740, 0x4ee0, 0x81, 0x92, 0x85, 0x0a, 0xb0, 0xe2, 0x19, 0x35);

#if (WINVER >= _WIN32_WINNT_WIN10)
//
// MF_MT_MAX_LUMINANCE_LEVEL specifies the maximum luminance level of the content in Nits.
// Has the same semantics as MaxCLL as defined in CEA-861.3
//
// {50253128-C110-4de4-98AE-46A324FAE6DA}   MF_MT_MAX_LUMINANCE_LEVEL   {UINT32}
DEFINE_GUID(MF_MT_MAX_LUMINANCE_LEVEL,
0x50253128, 0xc110, 0x4de4, 0x98, 0xae, 0x46, 0xa3, 0x24, 0xfa, 0xe6, 0xda);

//
// MF_MT_MAX_FRAME_AVERAGE_LUMINANCE_LEVEL specifies the maximum average per-frame
// luminance level of the content in Nits.
// Has the same semantics as MaxFALL as defined in CEA-861.3
//
// {58D4BF57-6F52-4733-A195-A9E29ECF9E27}   MF_MT_MAX_FRAME_AVERAGE_LUMINANCE_LEVEL  {UINT32}
DEFINE_GUID(MF_MT_MAX_FRAME_AVERAGE_LUMINANCE_LEVEL,
0x58d4bf57, 0x6f52, 0x4733, 0xa1, 0x95, 0xa9, 0xe2, 0x9e, 0xcf, 0x9e, 0x27);

//
// MF_MT_MAX_MASTERING_LUMINANCE specifies the maximum luminance of the display
// the content was authored on in Nits.
// Has the same semantics as max_display_mastering_luminance as defined in ST.2086
//
// {D6C6B997-272F-4ca1-8D00-8042111A0FF6} MF_MT_MAX_MASTERING_LUMINANCE {UINT32}
DEFINE_GUID(MF_MT_MAX_MASTERING_LUMINANCE,
0xd6c6b997, 0x272f, 0x4ca1, 0x8d, 0x0, 0x80, 0x42, 0x11, 0x1a, 0xf, 0xf6);

//
// MF_MT_MIN_MASTERING_LUMINANCE specifies the maximum luminance of the display
// the content was authored on in 0.0001 Nits.
// Has the same semantics as min_display_mastering_luminance as defined in ST.2086
//
// {839A4460-4E7E-4b4f-AE79-CC08905C7B27} MF_MT_MIN_MASTERING_LUMINANCE {UINT32}
DEFINE_GUID(MF_MT_MIN_MASTERING_LUMINANCE,
0x839a4460, 0x4e7e, 0x4b4f, 0xae, 0x79, 0xcc, 0x8, 0x90, 0x5c, 0x7b, 0x27);

// 
// MF_MT_DECODER_USE_MAX_RESOLUTION hints the decoder should allocate worst 
// case supported resolution whenever possible
// {4c547c24-af9a-4f38-96ad-978773cf53e7} MF_MT_DECODER_USE_MAX_RESOLUTION {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_DECODER_USE_MAX_RESOLUTION,
0x4c547c24, 0xaf9a, 0x4f38, 0x96, 0xad, 0x97, 0x87, 0x73, 0xcf, 0x53, 0xe7);

//
// MF_MT_DECODER_MAX_DPB_COUNT is a value that hints to the decoder that the current 
// decoding session will never require more than the specified number of decode surfaces
// {67BE144C-88B7-4CA9-9628-C808D5262217} MF_MT_DECODER_MAX_DPB_COUNT {UINT32}
DEFINE_GUID(MF_MT_DECODER_MAX_DPB_COUNT,
0x67be144c, 0x88b7, 0x4ca9, 0x96, 0x28, 0xc8, 0x8, 0xd5, 0x26, 0x22, 0x17);

#endif // (WINVER > _WIN32_WINNT_WIN10)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

// {47537213-8cfb-4722-aa34-fbc9e24d77b8}   MF_MT_CUSTOM_VIDEO_PRIMARIES    {BLOB (MT_CUSTOM_VIDEO_PRIMARIES)}
DEFINE_GUID(MF_MT_CUSTOM_VIDEO_PRIMARIES,
0x47537213, 0x8cfb, 0x4722, 0xaa, 0x34, 0xfb, 0xc9, 0xe2, 0x4d, 0x77, 0xb8);

typedef struct _MT_CUSTOM_VIDEO_PRIMARIES {
    float fRx;
    float fRy;
    float fGx;
    float fGy;
    float fBx;
    float fBy;
    float fWx;
    float fWy;
} MT_CUSTOM_VIDEO_PRIMARIES;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

// {3e23d450-2c75-4d25-a00e-b91670d12327}   MF_MT_YUV_MATRIX                {UINT32 (oneof MFVideoTransferMatrix)}
DEFINE_GUID(MF_MT_YUV_MATRIX,
0x3e23d450, 0x2c75, 0x4d25, 0xa0, 0x0e, 0xb9, 0x16, 0x70, 0xd1, 0x23, 0x27);

// {53a0529c-890b-4216-8bf9-599367ad6d20}   MF_MT_VIDEO_LIGHTING            {UINT32 (oneof MFVideoLighting)}
DEFINE_GUID(MF_MT_VIDEO_LIGHTING,
0x53a0529c, 0x890b, 0x4216, 0x8b, 0xf9, 0x59, 0x93, 0x67, 0xad, 0x6d, 0x20);

// {c21b8ee5-b956-4071-8daf-325edf5cab11}   MF_MT_VIDEO_NOMINAL_RANGE       {UINT32 (oneof MFNominalRange)}
DEFINE_GUID(MF_MT_VIDEO_NOMINAL_RANGE,
0xc21b8ee5, 0xb956, 0x4071, 0x8d, 0xaf, 0x32, 0x5e, 0xdf, 0x5c, 0xab, 0x11);

// {66758743-7e5f-400d-980a-aa8596c85696}   MF_MT_GEOMETRIC_APERTURE        {BLOB (MFVideoArea)}
DEFINE_GUID(MF_MT_GEOMETRIC_APERTURE,
0x66758743, 0x7e5f, 0x400d, 0x98, 0x0a, 0xaa, 0x85, 0x96, 0xc8, 0x56, 0x96);

// {d7388766-18fe-48c6-a177-ee894867c8c4}   MF_MT_MINIMUM_DISPLAY_APERTURE  {BLOB (MFVideoArea)}
DEFINE_GUID(MF_MT_MINIMUM_DISPLAY_APERTURE,
0xd7388766, 0x18fe, 0x48c6, 0xa1, 0x77, 0xee, 0x89, 0x48, 0x67, 0xc8, 0xc4);

// {79614dde-9187-48fb-b8c7-4d52689de649}   MF_MT_PAN_SCAN_APERTURE         {BLOB (MFVideoArea)}
DEFINE_GUID(MF_MT_PAN_SCAN_APERTURE,
0x79614dde, 0x9187, 0x48fb, 0xb8, 0xc7, 0x4d, 0x52, 0x68, 0x9d, 0xe6, 0x49);

// {4b7f6bc3-8b13-40b2-a993-abf630b8204e}   MF_MT_PAN_SCAN_ENABLED          {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_PAN_SCAN_ENABLED,
0x4b7f6bc3, 0x8b13, 0x40b2, 0xa9, 0x93, 0xab, 0xf6, 0x30, 0xb8, 0x20, 0x4e);

// {20332624-fb0d-4d9e-bd0d-cbf6786c102e}   MF_MT_AVG_BITRATE               {UINT32}
DEFINE_GUID(MF_MT_AVG_BITRATE,
0x20332624, 0xfb0d, 0x4d9e, 0xbd, 0x0d, 0xcb, 0xf6, 0x78, 0x6c, 0x10, 0x2e);

// {799cabd6-3508-4db4-a3c7-569cd533deb1}   MF_MT_AVG_BIT_ERROR_RATE        {UINT32}
DEFINE_GUID(MF_MT_AVG_BIT_ERROR_RATE,
0x799cabd6, 0x3508, 0x4db4, 0xa3, 0xc7, 0x56, 0x9c, 0xd5, 0x33, 0xde, 0xb1);

// {c16eb52b-73a1-476f-8d62-839d6a020652}   MF_MT_MAX_KEYFRAME_SPACING      {UINT32}
DEFINE_GUID(MF_MT_MAX_KEYFRAME_SPACING,
0xc16eb52b, 0x73a1, 0x476f, 0x8d, 0x62, 0x83, 0x9d, 0x6a, 0x02, 0x06, 0x52);

// {b6bc765f-4c3b-40a4-bd51-2535b66fe09d}   MF_MT_USER_DATA                 {BLOB}
DEFINE_GUID(MF_MT_USER_DATA,
0xb6bc765f, 0x4c3b, 0x40a4, 0xbd, 0x51, 0x25, 0x35, 0xb6, 0x6f, 0xe0, 0x9d);

// {a505d3ac-f930-436e-8ede-93a509ce23b2} MF_MT_OUTPUT_BUFFER_NUM {UINT32}
DEFINE_GUID(MF_MT_OUTPUT_BUFFER_NUM,
0xa505d3ac, 0xf930, 0x436e, 0x8e, 0xde, 0x93, 0xa5, 0x09, 0xce, 0x23, 0xb2);

#if (WINVER >= _WIN32_WINNT_WIN10)
/// {0xbb12d222,0x2bdb,0x425e,0x91,0xec,0x23,0x08,0xe1,0x89,0xa5,0x8f}   MF_MT_REALTIME_CONTENT UINT32 (0 or 1)
DEFINE_GUID(MF_MT_REALTIME_CONTENT,
0xbb12d222,0x2bdb,0x425e,0x91,0xec,0x23,0x08,0xe1,0x89,0xa5,0x8f);
#endif // (WINVER >= _WIN32_WINNT_WIN10

//
// VIDEO - uncompressed format data
//

// {644b4e48-1e02-4516-b0eb-c01ca9d49ac6}   MF_MT_DEFAULT_STRIDE            {UINT32 (INT32)} // in bytes
DEFINE_GUID(MF_MT_DEFAULT_STRIDE,
0x644b4e48, 0x1e02, 0x4516, 0xb0, 0xeb, 0xc0, 0x1c, 0xa9, 0xd4, 0x9a, 0xc6);

// {6d283f42-9846-4410-afd9-654d503b1a54}   MF_MT_PALETTE                   {BLOB (array of MFPaletteEntry - usually 256)}
DEFINE_GUID(MF_MT_PALETTE,
0x6d283f42, 0x9846, 0x4410, 0xaf, 0xd9, 0x65, 0x4d, 0x50, 0x3b, 0x1a, 0x54);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// the following is only used for legacy data that was stuck at the end of the format block when the type
// was converted from a VIDEOINFOHEADER or VIDEOINFOHEADER2 block in an AM_MEDIA_TYPE.
//

// {73d1072d-1870-4174-a063-29ff4ff6c11e}
DEFINE_GUID(MF_MT_AM_FORMAT_TYPE,
0x73d1072d, 0x1870, 0x4174, 0xa0, 0x63, 0x29, 0xff, 0x4f, 0xf6, 0xc1, 0x1e);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// VIDEO - Generic compressed video extra data
//

// {ad76a80b-2d5c-4e0b-b375-64e520137036}   MF_MT_VIDEO_PROFILE             {UINT32}    This is an alias of  MF_MT_MPEG2_PROFILE       
DEFINE_GUID(MF_MT_VIDEO_PROFILE,
0xad76a80b, 0x2d5c, 0x4e0b, 0xb3, 0x75, 0x64, 0xe5, 0x20, 0x13, 0x70, 0x36);

// {96f66574-11c5-4015-8666-bff516436da7}   MF_MT_VIDEO_LEVEL               {UINT32}    This is an alias of  MF_MT_MPEG2_LEVEL        
DEFINE_GUID(MF_MT_VIDEO_LEVEL,
0x96f66574, 0x11c5, 0x4015, 0x86, 0x66, 0xbf, 0xf5, 0x16, 0x43, 0x6d, 0xa7);


//
// VIDEO - MPEG1/2 extra data
//

// {91f67885-4333-4280-97cd-bd5a6c03a06e}   MF_MT_MPEG_START_TIME_CODE      {UINT32}
DEFINE_GUID(MF_MT_MPEG_START_TIME_CODE,
0x91f67885, 0x4333, 0x4280, 0x97, 0xcd, 0xbd, 0x5a, 0x6c, 0x03, 0xa0, 0x6e);

// {ad76a80b-2d5c-4e0b-b375-64e520137036}   MF_MT_MPEG2_PROFILE             {UINT32 (oneof AM_MPEG2Profile)}
DEFINE_GUID(MF_MT_MPEG2_PROFILE,
0xad76a80b, 0x2d5c, 0x4e0b, 0xb3, 0x75, 0x64, 0xe5, 0x20, 0x13, 0x70, 0x36);

// {96f66574-11c5-4015-8666-bff516436da7}   MF_MT_MPEG2_LEVEL               {UINT32 (oneof AM_MPEG2Level)}
DEFINE_GUID(MF_MT_MPEG2_LEVEL,
0x96f66574, 0x11c5, 0x4015, 0x86, 0x66, 0xbf, 0xf5, 0x16, 0x43, 0x6d, 0xa7);

// {31e3991d-f701-4b2f-b426-8ae3bda9e04b}   MF_MT_MPEG2_FLAGS               {UINT32 (anyof AMMPEG2_xxx flags)}
DEFINE_GUID(MF_MT_MPEG2_FLAGS,
0x31e3991d, 0xf701, 0x4b2f, 0xb4, 0x26, 0x8a, 0xe3, 0xbd, 0xa9, 0xe0, 0x4b);

// {3c036de7-3ad0-4c9e-9216-ee6d6ac21cb3}   MF_MT_MPEG_SEQUENCE_HEADER      {BLOB}
DEFINE_GUID(MF_MT_MPEG_SEQUENCE_HEADER,
0x3c036de7, 0x3ad0, 0x4c9e, 0x92, 0x16, 0xee, 0x6d, 0x6a, 0xc2, 0x1c, 0xb3);

// {A20AF9E8-928A-4B26-AAA9-F05C74CAC47C}   MF_MT_MPEG2_STANDARD            {UINT32 (0 for default MPEG2, 1  to use ATSC standard, 2 to use DVB standard, 3 to use ARIB standard)}
DEFINE_GUID(MF_MT_MPEG2_STANDARD, 
0xa20af9e8, 0x928a, 0x4b26, 0xaa, 0xa9, 0xf0, 0x5c, 0x74, 0xca, 0xc4, 0x7c);

// {5229BA10-E29D-4F80-A59C-DF4F180207D2}   MF_MT_MPEG2_TIMECODE            {UINT32 (0 for no timecode, 1 to append an 4 byte timecode to the front of each transport packet)}
DEFINE_GUID(MF_MT_MPEG2_TIMECODE, 
0x5229ba10, 0xe29d, 0x4f80, 0xa5, 0x9c, 0xdf, 0x4f, 0x18, 0x2, 0x7, 0xd2);

// {825D55E4-4F12-4197-9EB3-59B6E4710F06}   MF_MT_MPEG2_CONTENT_PACKET      {UINT32 (0 for no content packet, 1 to append a 14 byte Content Packet header according to the ARIB specification to the beginning a transport packet at 200-1000 ms intervals.)}
DEFINE_GUID(MF_MT_MPEG2_CONTENT_PACKET, 
0x825d55e4, 0x4f12, 0x4197, 0x9e, 0xb3, 0x59, 0xb6, 0xe4, 0x71, 0xf, 0x6);

// {91a49eb5-1d20-4b42-ace8-804269bf95ed}   MF_MT_MPEG2_ONE_FRAME_PER_PACKET      {UINT32 (BOOL) -- 0 for default behavior of splitting large video frames into multiple PES packets, 1 for always putting a full frame inside a PES packet, even if that requires setting the PES packet size to undefined (0)}
DEFINE_GUID(MF_MT_MPEG2_ONE_FRAME_PER_PACKET,
0x91a49eb5, 0x1d20, 0x4b42, 0xac, 0xe8, 0x80, 0x42, 0x69, 0xbf, 0x95, 0xed);

// {168f1b4a-3e91-450f-aea7-e4baeadae5ba} MF_MT_MPEG2_HDCP  {UINT32 (BOOL) -- 0 for default behavior of clear MPEG2 stream, 1 for adding the HDCP descriptor to the PMT
DEFINE_GUID(MF_MT_MPEG2_HDCP,
0x168f1b4a, 0x3e91, 0x450f, 0xae, 0xa7, 0xe4, 0xba, 0xea, 0xda, 0xe5, 0xba);

//
// VIDEO - H264 extra data
//

// {F5929986-4C45-4FBB-BB49-6CC534D05B9B}  {UINT32, UVC 1.5 H.264 format descriptor: bMaxCodecConfigDelay}
DEFINE_GUID(MF_MT_H264_MAX_CODEC_CONFIG_DELAY,
0xf5929986, 0x4c45, 0x4fbb, 0xbb, 0x49, 0x6c, 0xc5, 0x34, 0xd0, 0x5b, 0x9b);

// {C8BE1937-4D64-4549-8343-A8086C0BFDA5} {UINT32, UVC 1.5 H.264 format descriptor: bmSupportedSliceModes}
DEFINE_GUID(MF_MT_H264_SUPPORTED_SLICE_MODES,
0xc8be1937, 0x4d64, 0x4549, 0x83, 0x43, 0xa8, 0x8, 0x6c, 0xb, 0xfd, 0xa5);

// {89A52C01-F282-48D2-B522-22E6AE633199} {UINT32, UVC 1.5 H.264 format descriptor: bmSupportedSyncFrameTypes}
DEFINE_GUID(MF_MT_H264_SUPPORTED_SYNC_FRAME_TYPES,
0x89a52c01, 0xf282, 0x48d2, 0xb5, 0x22, 0x22, 0xe6, 0xae, 0x63, 0x31, 0x99);

// {E3854272-F715-4757-BA90-1B696C773457} {UINT32, UVC 1.5 H.264 format descriptor: bResolutionScaling}
DEFINE_GUID(MF_MT_H264_RESOLUTION_SCALING,
0xe3854272, 0xf715, 0x4757, 0xba, 0x90, 0x1b, 0x69, 0x6c, 0x77, 0x34, 0x57);

// {9EA2D63D-53F0-4A34-B94E-9DE49A078CB3} {UINT32, UVC 1.5 H.264 format descriptor: bSimulcastSupport}
DEFINE_GUID(MF_MT_H264_SIMULCAST_SUPPORT,
0x9ea2d63d, 0x53f0, 0x4a34, 0xb9, 0x4e, 0x9d, 0xe4, 0x9a, 0x7, 0x8c, 0xb3);

// {6A8AC47E-519C-4F18-9BB3-7EEAAEA5594D} {UINT32, UVC 1.5 H.264 format descriptor: bmSupportedRateControlModes}
DEFINE_GUID(MF_MT_H264_SUPPORTED_RATE_CONTROL_MODES,
0x6a8ac47e, 0x519c, 0x4f18, 0x9b, 0xb3, 0x7e, 0xea, 0xae, 0xa5, 0x59, 0x4d);

// {45256D30-7215-4576-9336-B0F1BCD59BB2}  {Blob of size 20 * sizeof(WORD), UVC 1.5 H.264 format descriptor: wMaxMBperSec*}
DEFINE_GUID(MF_MT_H264_MAX_MB_PER_SEC,
0x45256d30, 0x7215, 0x4576, 0x93, 0x36, 0xb0, 0xf1, 0xbc, 0xd5, 0x9b, 0xb2);

// {60B1A998-DC01-40CE-9736-ABA845A2DBDC}         {UINT32, UVC 1.5 H.264 frame descriptor: bmSupportedUsages}
DEFINE_GUID(MF_MT_H264_SUPPORTED_USAGES,
0x60b1a998, 0xdc01, 0x40ce, 0x97, 0x36, 0xab, 0xa8, 0x45, 0xa2, 0xdb, 0xdc);

// {BB3BD508-490A-11E0-99E4-1316DFD72085}         {UINT32, UVC 1.5 H.264 frame descriptor: bmCapabilities}
DEFINE_GUID(MF_MT_H264_CAPABILITIES,
0xbb3bd508, 0x490a, 0x11e0, 0x99, 0xe4, 0x13, 0x16, 0xdf, 0xd7, 0x20, 0x85);

// {F8993ABE-D937-4A8F-BBCA-6966FE9E1152}         {UINT32, UVC 1.5 H.264 frame descriptor: bmSVCCapabilities}
DEFINE_GUID(MF_MT_H264_SVC_CAPABILITIES,
0xf8993abe, 0xd937, 0x4a8f, 0xbb, 0xca, 0x69, 0x66, 0xfe, 0x9e, 0x11, 0x52);

// {359CE3A5-AF00-49CA-A2F4-2AC94CA82B61}         {UINT32, UVC 1.5 H.264 Probe/Commit Control: bUsage}
DEFINE_GUID(MF_MT_H264_USAGE,
0x359ce3a5, 0xaf00, 0x49ca, 0xa2, 0xf4, 0x2a, 0xc9, 0x4c, 0xa8, 0x2b, 0x61);

//{705177D8-45CB-11E0-AC7D-B91CE0D72085}          {UINT32, UVC 1.5 H.264 Probe/Commit Control: bmRateControlModes}
DEFINE_GUID(MF_MT_H264_RATE_CONTROL_MODES,
0x705177d8, 0x45cb, 0x11e0, 0xac, 0x7d, 0xb9, 0x1c, 0xe0, 0xd7, 0x20, 0x85);

//{85E299B2-90E3-4FE8-B2F5-C067E0BFE57A}          {UINT64, UVC 1.5 H.264 Probe/Commit Control: bmLayoutPerStream}
DEFINE_GUID(MF_MT_H264_LAYOUT_PER_STREAM,
0x85e299b2, 0x90e3, 0x4fe8, 0xb2, 0xf5, 0xc0, 0x67, 0xe0, 0xbf, 0xe5, 0x7a);

// According to Mpeg4 spec, SPS and PPS of H.264/HEVC codec could appear in sample data.
// description box. Mpeg4 sink filters out the SPS and PPS NALU and do not support in band SPS and PPS NALU.
// This attribute enables support for in band SPS and PPS to appear in the elementary stream. 
// HEVC will have in-band parameter set by default with MP4 recording for broad support.  H.264 will have out - of - band parameter set by default for historical reason.
// {75DA5090-910B-4A03-896C-7B898FEEA5AF}
DEFINE_GUID(MF_MT_IN_BAND_PARAMETER_SET,
0x75da5090, 0x910b, 0x4a03, 0x89, 0x6c, 0x7b, 0x89, 0x8f, 0xee, 0xa5, 0xaf);

//{54F486DD-9327-4F6D-80AB-6F709EBB4CCE}          {UINT32, FourCC of the track type in MPEG-4 used for binary streams}
DEFINE_GUID(MF_MT_MPEG4_TRACK_TYPE,
    0x54f486dd, 0x9327, 0x4f6d, 0x80, 0xab, 0x6f, 0x70, 0x9e, 0xbb, 0x4c, 0xce);

// The speed-up factor of playback. This is a multiplier to the frame rate, and is expressed as a ratio.
// The upper 32 bits of the attribute value contain the numerator, and the lower 32 bits contain the denominator.
// Note that this applies to the entirety of the playback.
// Use MFGetAttributeRataio/MFSetAttributeRatio to get/set value.
// {83877F5E-0444-4E28-8479-6DB0989B8C09}   {UINT64}
DEFINE_GUID(MF_MT_CONTAINER_RATE_SCALING,
    0x83877f5e, 0x444, 0x4e28, 0x84, 0x79, 0x6d, 0xb0, 0x98, 0x9b, 0x8c, 0x9);

//
// INTERLEAVED - DV extra data
//
// {84bd5d88-0fb8-4ac8-be4b-a8848bef98f3}   MF_MT_DV_AAUX_SRC_PACK_0        {UINT32}
DEFINE_GUID(MF_MT_DV_AAUX_SRC_PACK_0,
0x84bd5d88, 0x0fb8, 0x4ac8, 0xbe, 0x4b, 0xa8, 0x84, 0x8b, 0xef, 0x98, 0xf3);

// {f731004e-1dd1-4515-aabe-f0c06aa536ac}   MF_MT_DV_AAUX_CTRL_PACK_0       {UINT32}
DEFINE_GUID(MF_MT_DV_AAUX_CTRL_PACK_0,
0xf731004e, 0x1dd1, 0x4515, 0xaa, 0xbe, 0xf0, 0xc0, 0x6a, 0xa5, 0x36, 0xac);

// {720e6544-0225-4003-a651-0196563a958e}   MF_MT_DV_AAUX_SRC_PACK_1        {UINT32}
DEFINE_GUID(MF_MT_DV_AAUX_SRC_PACK_1,
0x720e6544, 0x0225, 0x4003, 0xa6, 0x51, 0x01, 0x96, 0x56, 0x3a, 0x95, 0x8e);

// {cd1f470d-1f04-4fe0-bfb9-d07ae0386ad8}   MF_MT_DV_AAUX_CTRL_PACK_1       {UINT32}
DEFINE_GUID(MF_MT_DV_AAUX_CTRL_PACK_1,
0xcd1f470d, 0x1f04, 0x4fe0, 0xbf, 0xb9, 0xd0, 0x7a, 0xe0, 0x38, 0x6a, 0xd8);

// {41402d9d-7b57-43c6-b129-2cb997f15009}   MF_MT_DV_VAUX_SRC_PACK          {UINT32}
DEFINE_GUID(MF_MT_DV_VAUX_SRC_PACK,
0x41402d9d, 0x7b57, 0x43c6, 0xb1, 0x29, 0x2c, 0xb9, 0x97, 0xf1, 0x50, 0x09);

// {2f84e1c4-0da1-4788-938e-0dfbfbb34b48}   MF_MT_DV_VAUX_CTRL_PACK         {UINT32}
DEFINE_GUID(MF_MT_DV_VAUX_CTRL_PACK,
0x2f84e1c4, 0x0da1, 0x4788, 0x93, 0x8e, 0x0d, 0xfb, 0xfb, 0xb3, 0x4b, 0x48);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if (WINVER >= _WIN32_WINNT_WIN7)

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// ARBITRARY
//

//
// MT_ARBITRARY_HEADER stores information about the format of an arbitrary media type
//
typedef struct _MT_ARBITRARY_HEADER
{
    GUID majortype;
    GUID subtype;
    BOOL bFixedSizeSamples;
    BOOL bTemporalCompression;
    ULONG lSampleSize;
    GUID formattype;
}
MT_ARBITRARY_HEADER;

// {9E6BD6F5-0109-4f95-84AC-9309153A19FC}   MF_MT_ARBITRARY_HEADER          {MT_ARBITRARY_HEADER}
DEFINE_GUID(MF_MT_ARBITRARY_HEADER,
0x9e6bd6f5, 0x109, 0x4f95, 0x84, 0xac, 0x93, 0x9, 0x15, 0x3a, 0x19, 0xfc );

// {5A75B249-0D7D-49a1-A1C3-E0D87F0CADE5}   MF_MT_ARBITRARY_FORMAT          {Blob}
DEFINE_GUID(MF_MT_ARBITRARY_FORMAT,
0x5a75b249, 0xd7d, 0x49a1, 0xa1, 0xc3, 0xe0, 0xd8, 0x7f, 0xc, 0xad, 0xe5);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// IMAGE
//
// {ED062CF4-E34E-4922-BE99-934032133D7C}   MF_MT_IMAGE_LOSS_TOLERANT       {UINT32 (BOOL)}
DEFINE_GUID(MF_MT_IMAGE_LOSS_TOLERANT, 
0xed062cf4, 0xe34e, 0x4922, 0xbe, 0x99, 0x93, 0x40, 0x32, 0x13, 0x3d, 0x7c);


//
// MPEG-4 Media Type Attributes
//
// {261E9D83-9529-4B8F-A111-8B9C950A81A9}   MF_MT_MPEG4_SAMPLE_DESCRIPTION   {BLOB}
DEFINE_GUID(MF_MT_MPEG4_SAMPLE_DESCRIPTION,
0x261e9d83, 0x9529, 0x4b8f, 0xa1, 0x11, 0x8b, 0x9c, 0x95, 0x0a, 0x81, 0xa9);

// {9aa7e155-b64a-4c1d-a500-455d600b6560}   MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY {UINT32}
DEFINE_GUID(MF_MT_MPEG4_CURRENT_SAMPLE_ENTRY,
0x9aa7e155, 0xb64a, 0x4c1d, 0xa5, 0x00, 0x45, 0x5d, 0x60, 0x0b, 0x65, 0x60);

#if (NTDDI_VERSION >= NTDDI_WIN10_RS4)
//
// Ambisonics Stream Attribute
// The value of this blob must be AMBISONICS_PARAMS structure defined in AudioClient.h
//
// {F715CF3E-A964-4C3F-94AE-9D6BA7264641}   MF_SD_AMBISONICS_SAMPLE3D_DESCRIPTION   {BLOB}
DEFINE_GUID(MF_SD_AMBISONICS_SAMPLE3D_DESCRIPTION,
0xf715cf3e, 0xa964, 0x4c3f, 0x94, 0xae, 0x9d, 0x6b, 0xa7, 0x26, 0x46, 0x41);

#endif

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// Save original format information for AVI and WAV files
//
// {d7be3fe0-2bc7-492d-b843-61a1919b70c3}   MF_MT_ORIGINAL_4CC               (UINT32)
DEFINE_GUID(MF_MT_ORIGINAL_4CC, 
0xd7be3fe0, 0x2bc7, 0x492d, 0xb8, 0x43, 0x61, 0xa1, 0x91, 0x9b, 0x70, 0xc3);

// {8cbbc843-9fd9-49c2-882f-a72586c408ad}   MF_MT_ORIGINAL_WAVE_FORMAT_TAG   (UINT32)
DEFINE_GUID(MF_MT_ORIGINAL_WAVE_FORMAT_TAG,
0x8cbbc843, 0x9fd9, 0x49c2, 0x88, 0x2f, 0xa7, 0x25, 0x86, 0xc4, 0x08, 0xad);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// Video Capture Media Type Attributes
//

// {D2E7558C-DC1F-403f-9A72-D28BB1EB3B5E}   MF_MT_FRAME_RATE_RANGE_MIN      {UINT64 (HI32(Numerator),LO32(Denominator))}
DEFINE_GUID(MF_MT_FRAME_RATE_RANGE_MIN, 
0xd2e7558c, 0xdc1f, 0x403f, 0x9a, 0x72, 0xd2, 0x8b, 0xb1, 0xeb, 0x3b, 0x5e);

// {E3371D41-B4CF-4a05-BD4E-20B88BB2C4D6}   MF_MT_FRAME_RATE_RANGE_MAX      {UINT64 (HI32(Numerator),LO32(Denominator))}
DEFINE_GUID(MF_MT_FRAME_RATE_RANGE_MAX, 
0xe3371d41, 0xb4cf, 0x4a05, 0xbd, 0x4e, 0x20, 0xb8, 0x8b, 0xb2, 0xc4, 0xd6);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // (WINVER >= _WIN32_WINNT_WIN7)

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

#if (WINVER >= _WIN32_WINNT_WIN8)
// {9C27891A-ED7A-40e1-88E8-B22727A024EE}   MF_LOW_LATENCY                  {UINT32 (BOOL)}
// Same GUID as CODECAPI_AVLowLatencyMode  
DEFINE_GUID(MF_LOW_LATENCY,
0x9c27891a, 0xed7a, 0x40e1, 0x88, 0xe8, 0xb2, 0x27, 0x27, 0xa0, 0x24, 0xee);

// {E3F2E203-D445-4B8C-9211-AE390D3BA017}  {UINT32} Maximum macroblocks per second that can be handled by MFT
DEFINE_GUID(MF_VIDEO_MAX_MB_PER_SEC,
0xe3f2e203, 0xd445, 0x4b8c, 0x92, 0x11, 0xae, 0x39, 0xd, 0x3b, 0xa0, 0x17);

// {7086E16C-49C5-4201-882A-8538F38CF13A} {UINT32 (BOOL)} Enables(0, default)/disables(1) the DXVA decode status queries in decoders. When disabled decoder won't provide MFSampleExtension_FrameCorruption
DEFINE_GUID(MF_DISABLE_FRAME_CORRUPTION_INFO,
    0x7086e16c, 0x49c5, 0x4201, 0x88, 0x2a, 0x85, 0x38, 0xf3, 0x8c, 0xf1, 0x3a);

#endif // (WINVER >= _WIN32_WINNT_WIN8)

////////////////////////////////////////////////////////////////////////////////
// Camera Extrinsics
////////////////////////////////////////////////////////////////////////////////

typedef struct _MF_FLOAT2
{
    FLOAT   x;
    FLOAT   y;
} MF_FLOAT2;

typedef struct _MF_FLOAT3
{
    FLOAT   x;
    FLOAT   y;
    FLOAT   z;
} MF_FLOAT3;

typedef struct _MF_QUATERNION
{
    FLOAT   x;
    FLOAT   y;
    FLOAT   z;
    FLOAT   w;
} MF_QUATERNION;

typedef struct _MFCameraExtrinsic_CalibratedTransform
{
    GUID            CalibrationId;
    MF_FLOAT3       Position;
    MF_QUATERNION   Orientation;
} MFCameraExtrinsic_CalibratedTransform;

typedef struct _MFCameraExtrinsics
{
    UINT32 TransformCount;
    MFCameraExtrinsic_CalibratedTransform CalibratedTransforms[1];
} MFCameraExtrinsics;

//
// MFStreamExtension_CameraExtrinsics {686196D0-13E2-41D9-9638-EF032C272A52}
// Value type: Blob (MFCameraExtrinsics)
// Stores camera extrinsics data on the stream's attribute store
//
DEFINE_GUID(MFStreamExtension_CameraExtrinsics,
    0x686196d0, 0x13e2, 0x41d9, 0x96, 0x38, 0xef, 0x3, 0x2c, 0x27, 0x2a, 0x52);

//
// MFSampleExtension_CameraExtrinsics {6B761658-B7EC-4C3B-8225-8623CABEC31D}
// Value type: Blob (MFCameraExtrinsics)
// Stores camera extrinsics data on the sample's (a.k.a frame) attribute store
//
DEFINE_GUID(MFSampleExtension_CameraExtrinsics,
    0x6b761658, 0xb7ec, 0x4c3b, 0x82, 0x25, 0x86, 0x23, 0xca, 0xbe, 0xc3, 0x1d);

////////////////////////////////////////////////////////////////////////////////
// Camera Intrinsics
////////////////////////////////////////////////////////////////////////////////

typedef struct _MFCameraIntrinsic_PinholeCameraModel
{
    MF_FLOAT2   FocalLength;
    MF_FLOAT2   PrincipalPoint;
} MFCameraIntrinsic_PinholeCameraModel;

typedef struct _MFCameraIntrinsic_DistortionModel
{
    FLOAT Radial_k1;
    FLOAT Radial_k2;
    FLOAT Radial_k3;
    FLOAT Tangential_p1;
    FLOAT Tangential_p2;
} MFCameraIntrinsic_DistortionModel;

typedef struct _MFPinholeCameraIntrinsic_IntrinsicModel
{
    UINT32 Width;
    UINT32 Height;
    MFCameraIntrinsic_PinholeCameraModel CameraModel;
    MFCameraIntrinsic_DistortionModel DistortionModel;
} MFPinholeCameraIntrinsic_IntrinsicModel;

typedef struct _MFPinholeCameraIntrinsics
{
    UINT32 IntrinsicModelCount;
    MFPinholeCameraIntrinsic_IntrinsicModel IntrinsicModels[1];
} MFPinholeCameraIntrinsics;

// MFStreamExtension_PinholeCameraIntrinsics {DBAC0455-0EC8-4AEF-9C32-7A3EE3456F53}
// Value type: Blob (MFPinholeCameraIntrinsics)
// Stores camera intrinsics data on stream attribute store
DEFINE_GUID(MFStreamExtension_PinholeCameraIntrinsics,
    0xdbac0455, 0xec8, 0x4aef, 0x9c, 0x32, 0x7a, 0x3e, 0xe3, 0x45, 0x6f, 0x53);

// MFSampleExtension_PinholeCameraIntrinsics {4EE3B6C5-6A15-4E72-9761-70C1DB8B9FE3}
// Value type: Blob (MFPinholeCameraIntrinsics)
// Stores camera intrinsics data on the sample's (a.k.a frame) attribute store
DEFINE_GUID(MFSampleExtension_PinholeCameraIntrinsics,
    0x4ee3b6c5, 0x6a15, 0x4e72, 0x97, 0x61, 0x70, 0xc1, 0xdb, 0x8b, 0x9f, 0xe3);


////////////////////////////////////////////////////////////////////////////////
///////////////////////////////  Media Type GUIDs //////////////////////////////
////////////////////////////////////////////////////////////////////////////////


//
// Major types
//
DEFINE_GUID(MFMediaType_Default,
0x81A412E6, 0x8103, 0x4B06, 0x85, 0x7F, 0x18, 0x62, 0x78, 0x10, 0x24, 0xAC);
DEFINE_GUID(MFMediaType_Audio,
0x73647561, 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71);
DEFINE_GUID(MFMediaType_Video,
0x73646976, 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71);
DEFINE_GUID(MFMediaType_Protected,
0x7b4b6fe6, 0x9d04, 0x4494, 0xbe, 0x14, 0x7e, 0x0b, 0xd0, 0x76, 0xc8, 0xe4);
DEFINE_GUID(MFMediaType_SAMI,
0xe69669a0, 0x3dcd, 0x40cb, 0x9e, 0x2e, 0x37, 0x08, 0x38, 0x7c, 0x06, 0x16);
DEFINE_GUID(MFMediaType_Script,
0x72178C22, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
DEFINE_GUID(MFMediaType_Image,
0x72178C23, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
DEFINE_GUID(MFMediaType_HTML,
0x72178C24, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
DEFINE_GUID(MFMediaType_Binary,
0x72178C25, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
DEFINE_GUID(MFMediaType_FileTransfer,
0x72178C26, 0xE45B, 0x11D5, 0xBC, 0x2A, 0x00, 0xB0, 0xD0, 0xF3, 0xF4, 0xAB);
DEFINE_GUID(MFMediaType_Stream,
0xe436eb83, 0x524f, 0x11ce, 0x9f, 0x53, 0x00, 0x20, 0xaf, 0x0b, 0xa7, 0x70);
DEFINE_GUID(MFMediaType_MultiplexedFrames,
0x6ea542b0, 0x281f, 0x4231, 0xa4, 0x64, 0xfe, 0x2f, 0x50, 0x22, 0x50, 0x1c);
DEFINE_GUID(MFMediaType_Subtitle,
0xa6d13581, 0xed50, 0x4e65, 0xae, 0x08, 0x26, 0x06, 0x55, 0x76, 0xaa, 0xcc);

#if (WINVER >= _WIN32_WINNT_WIN10)
DEFINE_GUID(MFMediaType_Perception,
0x597ff6f9, 0x6ea2, 0x4670, 0x85, 0xb4, 0xea, 0x84, 0x7, 0x3f, 0xe9, 0x40);
#endif // (WINVER >= _WIN32_WINNT_WIN10)

//
// Image subtypes (MFMediaType_Image major type)
//
// JPEG subtype: same as GUID_ContainerFormatJpeg 
DEFINE_GUID(MFImageFormat_JPEG,
0x19e4a5aa, 0x5662, 0x4fc5, 0xa0, 0xc0, 0x17, 0x58, 0x02, 0x8e, 0x10, 0x57);

// RGB32 subtype: same as MFVideoFormat_RGB32 
DEFINE_GUID(MFImageFormat_RGB32,
0x00000016, 0x0000, 0x0010, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71);

//
// MPEG2 Stream subtypes (MFMediaType_Stream major type)
//
DEFINE_GUID(MFStreamFormat_MPEG2Transport,
0xe06d8023, 0xdb46, 0x11cf, 0xb4, 0xd1, 0x00, 0x80, 0x5f, 0x6c, 0xbb, 0xea);
DEFINE_GUID(MFStreamFormat_MPEG2Program,
0x263067d1, 0xd330, 0x45dc, 0xb6, 0x69, 0x34, 0xd9, 0x86, 0xe4, 0xe3, 0xe1);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
//
// Representations
//
DEFINE_GUID(AM_MEDIA_TYPE_REPRESENTATION,
0xe2e42ad2, 0x132c, 0x491e, 0xa2, 0x68, 0x3c, 0x7c, 0x2d, 0xca, 0x18, 0x1f);
DEFINE_GUID(FORMAT_MFVideoFormat,
0xaed4ab2d, 0x7326, 0x43cb, 0x94, 0x64, 0xc8, 0x79, 0xca, 0xb9, 0xc4, 0x3d);

#if (WINVER >= _WIN32_WINNT_FE)
// {2C8FA20C-82BB-4782-90A0-98A2A5BD8EF8}
DEFINE_GUID(MFMediaType_Metadata, 
0x2c8fa20c, 0x82bb, 0x4782, 0x90, 0xa0, 0x98, 0xa2, 0xa5, 0xbd, 0x8e, 0xf8);
#endif // (WINVER >= _WIN32_WINNT_FE)



///////////////////////////////////////////////////////////////////////////////////////////////////////////////  Media Type functions //////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// Forward declaration
//
struct tagVIDEOINFOHEADER;
typedef struct tagVIDEOINFOHEADER VIDEOINFOHEADER;
struct tagVIDEOINFOHEADER2;
typedef struct tagVIDEOINFOHEADER2 VIDEOINFOHEADER2;
struct tagMPEG1VIDEOINFO;
typedef struct tagMPEG1VIDEOINFO MPEG1VIDEOINFO;
struct tagMPEG2VIDEOINFO;
typedef struct tagMPEG2VIDEOINFO MPEG2VIDEOINFO;
struct _AMMediaType;
typedef struct _AMMediaType AM_MEDIA_TYPE;

STDAPI
MFValidateMediaTypeSize(
    _In_                    GUID    FormatType,
    _In_reads_bytes_opt_(cbSize) UINT8*  pBlock,
    _In_                    UINT32  cbSize
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI
MFCreateMediaType(
    _Outptr_ IMFMediaType**  ppMFType
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI
MFCreateMFVideoFormatFromMFMediaType(
    _In_        IMFMediaType*           pMFType,
    _Out_       MFVIDEOFORMAT**         ppMFVF, // must be deleted with CoTaskMemFree
    _Out_opt_   UINT32*                 pcbSize
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

typedef enum _MFWaveFormatExConvertFlags {
    MFWaveFormatExConvertFlag_Normal            = 0,
    MFWaveFormatExConvertFlag_ForceExtensible   = 1
} MFWaveFormatExConvertFlags;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#ifdef __cplusplus

//
// declarations with default parameters
//

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
STDAPI
MFCreateWaveFormatExFromMFMediaType(
    _In_        IMFMediaType*   pMFType,
    _Out_       WAVEFORMATEX**  ppWF,
    _Out_opt_   UINT32*         pcbSize,
    _In_        UINT32          Flags = MFWaveFormatExConvertFlag_Normal
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI
MFInitMediaTypeFromVideoInfoHeader(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const VIDEOINFOHEADER*  pVIH,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype = NULL
    );

STDAPI
MFInitMediaTypeFromVideoInfoHeader2(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const VIDEOINFOHEADER2* pVIH2,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype = NULL
    );

STDAPI
MFInitMediaTypeFromMPEG1VideoInfo(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const MPEG1VIDEOINFO*   pMP1VI,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype = NULL
    );

STDAPI
MFInitMediaTypeFromMPEG2VideoInfo(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const MPEG2VIDEOINFO*   pMP2VI,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype = NULL
    );

#ifndef NOBITMAP
STDAPI
MFCalculateBitmapImageSize(
    _In_reads_bytes_(cbBufSize)  const BITMAPINFOHEADER* pBMIH,
    _In_                    UINT32                  cbBufSize,
    _Out_                   UINT32*                 pcbImageSize,
    _Out_opt_               BOOL*                   pbKnown = NULL
    );
#endif // NOBITMAP
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#else /* cplusplus */

//
// same declarations without default parameters
//

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI
MFCreateWaveFormatExFromMFMediaType(
    _In_        IMFMediaType*   pMFType,
    _Out_       WAVEFORMATEX**  ppWF,
    _Out_opt_   UINT32*         pcbSize,
    _In_        UINT32          Flags
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
STDAPI
MFInitMediaTypeFromVideoInfoHeader(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const VIDEOINFOHEADER*  pVIH,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype
    );

STDAPI
MFInitMediaTypeFromVideoInfoHeader2(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const VIDEOINFOHEADER2* pVIH2,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype
    );

STDAPI
MFInitMediaTypeFromMPEG1VideoInfo(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const MPEG1VIDEOINFO*   pMP1VI,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype
    );

STDAPI
MFInitMediaTypeFromMPEG2VideoInfo(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const MPEG2VIDEOINFO*   pMP2VI,
    _In_                    UINT32                  cbBufSize,
    _In_opt_                const GUID*             pSubtype
    );

#ifndef NOBITMAP
STDAPI
MFCalculateBitmapImageSize(
    _In_reads_bytes_(cbBufSize)  const BITMAPINFOHEADER* pBMIH,
    _In_                    UINT32                  cbBufSize,
    _Out_                   UINT32*                 pcbImageSize,
    _Out_opt_               BOOL*                   pbKnown
    );
#endif // NOBITMAP
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif /* cplusplus */

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI
MFCalculateImageSize(
    _In_                    REFGUID                 guidSubtype,
    _In_                    UINT32                  unWidth,
    _In_                    UINT32                  unHeight,
    _Out_                   UINT32*                 pcbImageSize
    );

STDAPI
MFFrameRateToAverageTimePerFrame(
    _In_                    UINT32                  unNumerator,
    _In_                    UINT32                  unDenominator,
    _Out_                   UINT64*                 punAverageTimePerFrame
    );

STDAPI
MFAverageTimePerFrameToFrameRate(
    _In_                    UINT64                  unAverageTimePerFrame,
    _Out_                   UINT32*                 punNumerator,
    _Out_                   UINT32*                 punDenominator
    );

STDAPI
MFInitMediaTypeFromWaveFormatEx(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const WAVEFORMATEX*     pWaveFormat,
    _In_                    UINT32                  cbBufSize
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
STDAPI
MFInitMediaTypeFromMFVideoFormat(
    _In_                    IMFMediaType*           pMFType,
    _In_reads_bytes_(cbBufSize)  const MFVIDEOFORMAT*    pMFVF,
    _In_                    UINT32                  cbBufSize
    );

STDAPI
MFInitMediaTypeFromAMMediaType(
    _In_    IMFMediaType*           pMFType,
    _In_    const AM_MEDIA_TYPE*    pAMType
    );

STDAPI
MFInitAMMediaTypeFromMFMediaType(
    _In_    IMFMediaType*           pMFType,
    _In_    GUID                    guidFormatBlockType,
    _Inout_ AM_MEDIA_TYPE*          pAMType
    );

STDAPI
MFCreateAMMediaTypeFromMFMediaType(
    _In_    IMFMediaType*           pMFType,
    _In_    GUID                    guidFormatBlockType,
    _Inout_ AM_MEDIA_TYPE**         ppAMType // delete with DeleteMediaType
    );


//
// This function compares a full media type to a partial media type.
//
// A "partial" media type is one that is given out by a component as a possible
// media type it could accept. Many attributes may be unset, which represents
// a "don't care" status for that attribute.
//
// For example, a video effect may report that it supports YV12,
// but not want to specify a particular size. It simply creates a media type and sets
// the major type to MFMediaType_Video and the subtype to MEDIASUBTYPE_YV12.
//
// The comparison function succeeds if the partial type contains at least a major type,
// and all of the attributes in the partial type exist in the full type and are set to
// the same value.
//
STDAPI_(BOOL)
MFCompareFullToPartialMediaType(
    _In_    IMFMediaType*   pMFTypeFull,
    _In_    IMFMediaType*   pMFTypePartial
    );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
STDAPI
MFWrapMediaType(
    _In_    IMFMediaType*    pOrig,
    _In_    REFGUID          MajorType,
    _In_    REFGUID          SubType,
    _Out_   IMFMediaType **  ppWrap
    );



STDAPI
MFUnwrapMediaType(
    _In_    IMFMediaType*    pWrap,
    _Out_   IMFMediaType **  ppOrig
    );

STDAPI MFGetStrideForBitmapInfoHeader(
    DWORD format,
    DWORD dwWidth,
    _Out_ LONG* pStride
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// MFCreateVideoMediaType
//

#ifdef _KSMEDIA_
STDAPI MFCreateVideoMediaTypeFromVideoInfoHeader(
    _In_ const KS_VIDEOINFOHEADER* pVideoInfoHeader,
    DWORD cbVideoInfoHeader,
    DWORD dwPixelAspectRatioX,
    DWORD dwPixelAspectRatioY,
    MFVideoInterlaceMode InterlaceMode,
    QWORD VideoFlags,
    _In_opt_ const GUID * pSubtype,
    _Out_ IMFVideoMediaType** ppIVideoMediaType
    );

STDAPI MFCreateVideoMediaTypeFromVideoInfoHeader2(
    _In_ const KS_VIDEOINFOHEADER2* pVideoInfoHeader,
    DWORD cbVideoInfoHeader,
    QWORD AdditionalVideoFlags,
    _In_opt_ const GUID * pSubtype,
    _Out_ IMFVideoMediaType** ppIVideoMediaType
    );

#endif

STDAPI MFCreateVideoMediaType(
    _In_ const MFVIDEOFORMAT* pVideoFormat,
    _Out_ IMFVideoMediaType** ppIVideoMediaType
    );

STDAPI MFCreateVideoMediaTypeFromSubtype(
    _In_ const GUID * pAMSubtype,
    _Out_ IMFVideoMediaType  **ppIVideoMediaType
    );

STDAPI_(BOOL)
MFIsFormatYUV(
    DWORD Format
    );

//
//  These depend on BITMAPINFOHEADER being defined
//
#ifndef NOBITMAP
STDAPI MFCreateVideoMediaTypeFromBitMapInfoHeader(
    _In_ const BITMAPINFOHEADER* pbmihBitMapInfoHeader,
    DWORD dwPixelAspectRatioX,
    DWORD dwPixelAspectRatioY,
    MFVideoInterlaceMode InterlaceMode,
    QWORD VideoFlags,
    QWORD qwFramesPerSecondNumerator,
    QWORD qwFramesPerSecondDenominator,
    DWORD dwMaxBitRate,
    _Out_ IMFVideoMediaType** ppIVideoMediaType
    );
#endif // NOBITMAP

STDAPI MFGetPlaneSize(
    DWORD format,
    DWORD dwWidth,
    DWORD dwHeight,
    _Out_ DWORD* pdwPlaneSize
    );

#if (WINVER >= _WIN32_WINNT_WIN7)
#ifndef NOBITMAP
//
// MFCreateVideoMediaTypeFromBitMapInfoHeaderEx
//

STDAPI MFCreateVideoMediaTypeFromBitMapInfoHeaderEx(
    _In_reads_bytes_(cbBitMapInfoHeader) const BITMAPINFOHEADER* pbmihBitMapInfoHeader,
    _In_    UINT32                  cbBitMapInfoHeader,
    DWORD dwPixelAspectRatioX,
    DWORD dwPixelAspectRatioY,
    MFVideoInterlaceMode InterlaceMode,
    QWORD VideoFlags,
    DWORD dwFramesPerSecondNumerator,
    DWORD dwFramesPerSecondDenominator,
    DWORD dwMaxBitRate,
    _Out_ IMFVideoMediaType** ppIVideoMediaType
    );
#endif // NOBITMAP
#endif // (WINVER >= _WIN32_WINNT_WIN7)

//
// MFCreateMediaTypeFromRepresentation
//

STDAPI MFCreateMediaTypeFromRepresentation(
    GUID guidRepresentation,
    _In_ LPVOID pvRepresentation,
    _Out_ IMFMediaType** ppIMediaType
    );


//
// MFCreateAudioMediaType
//


STDAPI
MFCreateAudioMediaType(
    _In_    const WAVEFORMATEX* pAudioFormat,
    _Out_   IMFAudioMediaType** ppIAudioMediaType
    );

DWORD
STDMETHODCALLTYPE
MFGetUncompressedVideoFormat(
    _In_    const MFVIDEOFORMAT* pVideoFormat
    );

STDAPI 
MFInitVideoFormat(
    _In_    MFVIDEOFORMAT*          pVideoFormat,
    _In_    MFStandardVideoFormat   type
    );

STDAPI
MFInitVideoFormat_RGB(
    _In_    MFVIDEOFORMAT*  pVideoFormat,
    _In_    DWORD           dwWidth,
    _In_    DWORD           dwHeight,
    _In_    DWORD           D3Dfmt /* 0 indicates sRGB */
    );

STDAPI 
MFConvertColorInfoToDXVA(
    _Out_ DWORD* pdwToDXVA,
    _In_  const MFVIDEOFORMAT* pFromFormat
    );
STDAPI
MFConvertColorInfoFromDXVA(
    _Inout_ MFVIDEOFORMAT* pToFormat,
    _In_    DWORD dwFromDXVA
    );

//
// Optimized stride copy function
//
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFCopyImage(
    _Out_writes_bytes_(_Inexpressible_(abs(lDestStride) *  dwLines)) BYTE* pDest,
    LONG    lDestStride,
    _In_reads_bytes_(_Inexpressible_(abs(lSrcStride) * dwLines)) const BYTE* pSrc,
    LONG    lSrcStride,
    _Out_range_(<=, _Inexpressible_(min(abs(lSrcStride), abs(lDestStride))))  DWORD dwWidthInBytes,
    DWORD   dwLines
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFConvertFromFP16Array(
    _Out_writes_(dwCount) float* pDest,
    _In_reads_(dwCount) const WORD* pSrc,
    DWORD dwCount
    );

STDAPI MFConvertToFP16Array(
    _Out_writes_(dwCount) WORD* pDest,
    _In_reads_(dwCount) const float* pSrc,
    DWORD dwCount
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

STDAPI MFCreate2DMediaBuffer( 
    _In_ DWORD dwWidth,
    _In_ DWORD dwHeight,
    _In_ DWORD dwFourCC,
    _In_ BOOL fBottomUp,
    _Out_ IMFMediaBuffer**    ppBuffer
    );


//
// Creates an optimal system memory media buffer from a media type
//
STDAPI MFCreateMediaBufferFromMediaType(
    _In_ IMFMediaType* pMediaType,
    _In_ LONGLONG llDuration,   // Sample Duration, needed for audio
    _In_ DWORD dwMinLength,     // 0 means optimized default 
    _In_ DWORD dwMinAlignment,  // 0 means optimized default
    _Outptr_ IMFMediaBuffer** ppBuffer 
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

///////////////////////////////////////////////////////////////////////////////////////////////////////////////  Attributes Utility functions ////////////////////////////
////////////////////////////////////////////////////////////////////////////////


#ifdef __cplusplus

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

//
// IMFAttributes inline UTILITY FUNCTIONS - used for IMFMediaType as well
//
inline
UINT32
HI32(UINT64 unPacked)
{
    return (UINT32)(unPacked >> 32);
}

inline
UINT32
LO32(UINT64 unPacked)
{
    return (UINT32)unPacked;
}

inline
UINT64
Pack2UINT32AsUINT64(UINT32 unHigh, UINT32 unLow)
{
    return ((UINT64)unHigh << 32) | unLow;
}

inline
void
Unpack2UINT32AsUINT64(UINT64 unPacked, _Out_ UINT32* punHigh, _Out_ UINT32* punLow)
{
    *punHigh = HI32(unPacked);
    *punLow = LO32(unPacked);
}

inline
UINT64
PackSize(UINT32 unWidth, UINT32 unHeight)
{
    return Pack2UINT32AsUINT64(unWidth, unHeight);
}

inline
void
UnpackSize(UINT64 unPacked, _Out_ UINT32* punWidth, _Out_ UINT32* punHeight)
{
    Unpack2UINT32AsUINT64(unPacked, punWidth, punHeight);
}

inline
UINT64
PackRatio(INT32 nNumerator, UINT32 unDenominator)
{
    return Pack2UINT32AsUINT64((UINT32)nNumerator, unDenominator);
}

inline
void
UnpackRatio(UINT64 unPacked, _Out_ INT32* pnNumerator, _Out_ UINT32* punDenominator)
{
    Unpack2UINT32AsUINT64(unPacked, (UINT32*)pnNumerator, punDenominator);
}


//
// "failsafe" inline get methods - return the stored value or return a default
//
inline
UINT32
MFGetAttributeUINT32(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    UINT32          unDefault
    )
{
    UINT32 unRet;
    if (FAILED(pAttributes->GetUINT32(guidKey, &unRet))) {
        unRet = unDefault;
    }
    return unRet;
}

inline
UINT64
MFGetAttributeUINT64(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    UINT64          unDefault
    )
{
    UINT64 unRet;
    if (FAILED(pAttributes->GetUINT64(guidKey, &unRet))) {
        unRet = unDefault;
    }
    return unRet;
}

inline
double
MFGetAttributeDouble(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    double          fDefault
    )
{
    double fRet;
    if (FAILED(pAttributes->GetDouble(guidKey, &fRet))) {
        fRet = fDefault;
    }
    return fRet;
}

//
// helpers for getting/setting ratios and sizes
//

inline
HRESULT
MFGetAttribute2UINT32asUINT64(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    _Out_ UINT32*   punHigh32,
    _Out_ UINT32*   punLow32
    )
{
    UINT64 unPacked;
    HRESULT hr = S_OK;

    hr = pAttributes->GetUINT64(guidKey, &unPacked);
    if (FAILED(hr)) {
        return hr;
    }
    Unpack2UINT32AsUINT64(unPacked, punHigh32, punLow32);

    return hr;
}

inline
HRESULT
MFSetAttribute2UINT32asUINT64(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    UINT32          unHigh32,
    UINT32          unLow32
    )
{
    return pAttributes->SetUINT64(guidKey, Pack2UINT32AsUINT64(unHigh32, unLow32));
}

inline
HRESULT
MFGetAttributeRatio(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    _Out_ UINT32*   punNumerator,
    _Out_ UINT32*   punDenominator
    )
{
    return MFGetAttribute2UINT32asUINT64(pAttributes, guidKey, punNumerator, punDenominator);
}

inline
HRESULT
MFGetAttributeSize(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    _Out_ UINT32*   punWidth,
    _Out_ UINT32*   punHeight
    )
{
    return MFGetAttribute2UINT32asUINT64(pAttributes, guidKey, punWidth, punHeight);
}

inline
HRESULT
MFSetAttributeRatio(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    UINT32          unNumerator,
    UINT32          unDenominator
    )
{
    return MFSetAttribute2UINT32asUINT64(pAttributes, guidKey, unNumerator, unDenominator);
}

inline
HRESULT
MFSetAttributeSize(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    UINT32          unWidth,
    UINT32          unHeight
    )
{
    return MFSetAttribute2UINT32asUINT64(pAttributes, guidKey, unWidth, unHeight);
}

#ifdef _INTSAFE_H_INCLUDED_
inline
HRESULT
MFGetAttributeString(
    IMFAttributes*  pAttributes,
    REFGUID         guidKey,
    _Outptr_ PWSTR *ppsz
    )
{
    UINT32 length;
    PWSTR psz = NULL;
    *ppsz = NULL;
    HRESULT hr = pAttributes->GetStringLength(guidKey, &length);
    // add NULL to length
    if (SUCCEEDED(hr)) {
        hr = UIntAdd(length, 1, &length);
    }
    if (SUCCEEDED(hr)) {
        size_t cb;
        hr = SizeTMult(length, sizeof(WCHAR), &cb);
        if( SUCCEEDED( hr ) )
        {
            psz = PWSTR( CoTaskMemAlloc( cb ) );
            if( !psz ) 
            {
                hr = E_OUTOFMEMORY;
            }
        }
    }
    if (SUCCEEDED(hr)) {
        hr = pAttributes->GetString(guidKey, psz, length, &length);
    }
    if (SUCCEEDED(hr)) {
        *ppsz = psz;
    } else {
        CoTaskMemFree(psz);
    }
    return hr;
}

#endif // _INTSAFE_H_INCLUDED_

///////////////////////////////  Collection         ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

//
// Instantiates the MF-provided IMFCollection implementation
//
STDAPI MFCreateCollection(
    _Out_ IMFCollection **ppIMFCollection );


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif

////////////////////////////////////////////////////////////////////////////////
////////////////////////////////  Memory Management ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

//
// Heap alloc/free
//
typedef enum _EAllocationType
{
    eAllocationTypeDynamic,
    eAllocationTypeRT,
    eAllocationTypePageable,
    eAllocationTypeIgnore
}   EAllocationType;

EXTERN_C void* WINAPI MFHeapAlloc( size_t nSize,
                            ULONG dwFlags,
                            _In_opt_ char *pszFile,
                            int line,
                            EAllocationType eat);
EXTERN_C void WINAPI MFHeapFree( void * pv );

//////////////////////////       SourceResolver     ////////////////////////////
////////////////////////////////////////////////////////////////////////////////
DEFINE_GUID(CLSID_MFSourceResolver,
    0x90eab60f,
    0xe43a,
    0x4188,
    0xbc, 0xc4, 0xe4, 0x7f, 0xdf, 0x04, 0x86, 0x8c);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#if (WINVER >= _WIN32_WINNT_WIN7)
//  Return (a * b + d) / c
//  Returns _I64_MAX or LLONG_MIN on failure or _I64_MAX if mplat.dll is not available

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)

LONGLONG WINAPI MFllMulDiv(LONGLONG a, LONGLONG b, LONGLONG c, LONGLONG d);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#endif // (WINVER >= _WIN32_WINNT_WIN7)


//////////////////////////    Content Protection    ////////////////////////////
////////////////////////////////////////////////////////////////////////////////

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)

STDAPI MFGetContentProtectionSystemCLSID(
    _In_ REFGUID guidProtectionSystemID,
    _Out_ CLSID *pclsid );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion

#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

// MF_DEVICESTREAM_ATTRIBUTE_FACEAUTH_CAPABILITY
// Data type: UINT64
// Represents the Capability field of the KSCAMERA_EXTENDEDPROP_HEADER corresponding to the
// KSPROPERTY_CAMERACONTROL_EXTENDED_FACEAUTH_MODE extended property control.  If this control
// is not supported, this attribute will not be present on the stream.  
// The capability advertised will only contain the bitwise OR of the available
// supported modes defined by the Face Auth DDI in ksmedia.h:
// 
//      KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_DISABLED
//      KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_ALTERNATIVE_FRAME_ILLUMINATION
//      KSCAMERA_EXTENDEDPROP_FACEAUTH_MODE_BACKGROUND_SUBTRACTION
DEFINE_GUID(MF_DEVICESTREAM_ATTRIBUTE_FACEAUTH_CAPABILITY,
0xCB6FD12A, 0x2248, 0x4E41, 0xAD, 0x46, 0xE7, 0x8B, 0xB9, 0x0A, 0xB9, 0xFC);

// MF_DEVICESTREAM_ATTRIBUTE_SECURE_CAPABILITY
// Data type: UINT64
// Represents the Capability field of the KSCAMERA_EXTENDEDPROP_HEADER corresponding to the
// KSPROPERTY_CAMERACONTROL_EXTENDED_SECURE_MODE extended property control.  If this control
// is not supported, this attribute will not be present on the stream.  
// The capability advertised will only contain the bitwise OR of the available
// supported modes defined by the Secure DDI in ksmedia.h:
// 
//      KSCAMERA_EXTENDEDPROP_SECURE_MODE_DISABLED
//      KSCAMERA_EXTENDEDPROP_SECURE_MODE_ENABLED
DEFINE_GUID(MF_DEVICESTREAM_ATTRIBUTE_SECURE_CAPABILITY,
0x940FD626, 0xEA6E, 0x4684, 0x98, 0x40, 0x36, 0xBD, 0x6E, 0xC9, 0xFB, 0xEF);

#if (NTDDI_VERSION >= NTDDI_WIN10_VB)

// MFCombineSamples
//  pSample - pointer to a sample to append/combine 'pSampleToAdd' to 
//  pSampleToAdd - the sample to append 
//  dwMaxMergedDurationInMS - indicates the maximum duration that the combined sample should be allowed to occupy 
//  pCombined - indicates that pSampleToAdd was successfully added to the base sample
STDAPI MFCombineSamples( _In_ IMFSample *pSample, _In_ IMFSample* pSampleToAdd, _In_ DWORD dwMaxMergedDurationInMS, _Out_ BOOL* pMerged );

// MFSplitSample
//  pSample - a single combined sample that should be split up 
//  pOutputSamples - output array of split samples 
//  dwOutputSampleMaxCount - maximum array size (use the BufferCount on pSample to find out an upper bound) 
//  pdwOutputSampleCount - actual number of output samples produce

STDAPI MFSplitSample( _In_ IMFSample *pSample, _Out_writes_to_(dwOutputSampleMaxCount, *pdwOutputSampleCount) IMFSample** pOutputSamples, _In_ DWORD dwOutputSampleMaxCount, _Out_ DWORD* pdwOutputSampleCount );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB)

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion


#if defined(__cplusplus)
}
#endif

#endif //#if !defined(__MFAPI_H__)

