

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

#ifndef __mfidl_h__
#define __mfidl_h__

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

#ifndef __IMFMediaSession_FWD_DEFINED__
#define __IMFMediaSession_FWD_DEFINED__
typedef interface IMFMediaSession IMFMediaSession;

#endif 	/* __IMFMediaSession_FWD_DEFINED__ */


#ifndef __IMFSourceResolver_FWD_DEFINED__
#define __IMFSourceResolver_FWD_DEFINED__
typedef interface IMFSourceResolver IMFSourceResolver;

#endif 	/* __IMFSourceResolver_FWD_DEFINED__ */


#ifndef __IMFMediaSource_FWD_DEFINED__
#define __IMFMediaSource_FWD_DEFINED__
typedef interface IMFMediaSource IMFMediaSource;

#endif 	/* __IMFMediaSource_FWD_DEFINED__ */


#ifndef __IMFMediaSourceEx_FWD_DEFINED__
#define __IMFMediaSourceEx_FWD_DEFINED__
typedef interface IMFMediaSourceEx IMFMediaSourceEx;

#endif 	/* __IMFMediaSourceEx_FWD_DEFINED__ */


#ifndef __IMFClockConsumer_FWD_DEFINED__
#define __IMFClockConsumer_FWD_DEFINED__
typedef interface IMFClockConsumer IMFClockConsumer;

#endif 	/* __IMFClockConsumer_FWD_DEFINED__ */


#ifndef __IMFMediaStream_FWD_DEFINED__
#define __IMFMediaStream_FWD_DEFINED__
typedef interface IMFMediaStream IMFMediaStream;

#endif 	/* __IMFMediaStream_FWD_DEFINED__ */


#ifndef __IMFMediaSink_FWD_DEFINED__
#define __IMFMediaSink_FWD_DEFINED__
typedef interface IMFMediaSink IMFMediaSink;

#endif 	/* __IMFMediaSink_FWD_DEFINED__ */


#ifndef __IMFStreamSink_FWD_DEFINED__
#define __IMFStreamSink_FWD_DEFINED__
typedef interface IMFStreamSink IMFStreamSink;

#endif 	/* __IMFStreamSink_FWD_DEFINED__ */


#ifndef __IMFVideoSampleAllocator_FWD_DEFINED__
#define __IMFVideoSampleAllocator_FWD_DEFINED__
typedef interface IMFVideoSampleAllocator IMFVideoSampleAllocator;

#endif 	/* __IMFVideoSampleAllocator_FWD_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorNotify_FWD_DEFINED__
#define __IMFVideoSampleAllocatorNotify_FWD_DEFINED__
typedef interface IMFVideoSampleAllocatorNotify IMFVideoSampleAllocatorNotify;

#endif 	/* __IMFVideoSampleAllocatorNotify_FWD_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorNotifyEx_FWD_DEFINED__
#define __IMFVideoSampleAllocatorNotifyEx_FWD_DEFINED__
typedef interface IMFVideoSampleAllocatorNotifyEx IMFVideoSampleAllocatorNotifyEx;

#endif 	/* __IMFVideoSampleAllocatorNotifyEx_FWD_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorCallback_FWD_DEFINED__
#define __IMFVideoSampleAllocatorCallback_FWD_DEFINED__
typedef interface IMFVideoSampleAllocatorCallback IMFVideoSampleAllocatorCallback;

#endif 	/* __IMFVideoSampleAllocatorCallback_FWD_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorEx_FWD_DEFINED__
#define __IMFVideoSampleAllocatorEx_FWD_DEFINED__
typedef interface IMFVideoSampleAllocatorEx IMFVideoSampleAllocatorEx;

#endif 	/* __IMFVideoSampleAllocatorEx_FWD_DEFINED__ */


#ifndef __IMFDXGIDeviceManagerSource_FWD_DEFINED__
#define __IMFDXGIDeviceManagerSource_FWD_DEFINED__
typedef interface IMFDXGIDeviceManagerSource IMFDXGIDeviceManagerSource;

#endif 	/* __IMFDXGIDeviceManagerSource_FWD_DEFINED__ */


#ifndef __IMFVideoProcessorControl_FWD_DEFINED__
#define __IMFVideoProcessorControl_FWD_DEFINED__
typedef interface IMFVideoProcessorControl IMFVideoProcessorControl;

#endif 	/* __IMFVideoProcessorControl_FWD_DEFINED__ */


#ifndef __IMFVideoProcessorControl2_FWD_DEFINED__
#define __IMFVideoProcessorControl2_FWD_DEFINED__
typedef interface IMFVideoProcessorControl2 IMFVideoProcessorControl2;

#endif 	/* __IMFVideoProcessorControl2_FWD_DEFINED__ */


#ifndef __IMFVideoProcessorControl3_FWD_DEFINED__
#define __IMFVideoProcessorControl3_FWD_DEFINED__
typedef interface IMFVideoProcessorControl3 IMFVideoProcessorControl3;

#endif 	/* __IMFVideoProcessorControl3_FWD_DEFINED__ */


#ifndef __IMFVideoRendererEffectControl_FWD_DEFINED__
#define __IMFVideoRendererEffectControl_FWD_DEFINED__
typedef interface IMFVideoRendererEffectControl IMFVideoRendererEffectControl;

#endif 	/* __IMFVideoRendererEffectControl_FWD_DEFINED__ */


#ifndef __IMFTopology_FWD_DEFINED__
#define __IMFTopology_FWD_DEFINED__
typedef interface IMFTopology IMFTopology;

#endif 	/* __IMFTopology_FWD_DEFINED__ */


#ifndef __IMFTopologyNode_FWD_DEFINED__
#define __IMFTopologyNode_FWD_DEFINED__
typedef interface IMFTopologyNode IMFTopologyNode;

#endif 	/* __IMFTopologyNode_FWD_DEFINED__ */


#ifndef __IMFGetService_FWD_DEFINED__
#define __IMFGetService_FWD_DEFINED__
typedef interface IMFGetService IMFGetService;

#endif 	/* __IMFGetService_FWD_DEFINED__ */


#ifndef __IMFClock_FWD_DEFINED__
#define __IMFClock_FWD_DEFINED__
typedef interface IMFClock IMFClock;

#endif 	/* __IMFClock_FWD_DEFINED__ */


#ifndef __IMFPresentationClock_FWD_DEFINED__
#define __IMFPresentationClock_FWD_DEFINED__
typedef interface IMFPresentationClock IMFPresentationClock;

#endif 	/* __IMFPresentationClock_FWD_DEFINED__ */


#ifndef __IMFPresentationTimeSource_FWD_DEFINED__
#define __IMFPresentationTimeSource_FWD_DEFINED__
typedef interface IMFPresentationTimeSource IMFPresentationTimeSource;

#endif 	/* __IMFPresentationTimeSource_FWD_DEFINED__ */


#ifndef __IMFClockStateSink_FWD_DEFINED__
#define __IMFClockStateSink_FWD_DEFINED__
typedef interface IMFClockStateSink IMFClockStateSink;

#endif 	/* __IMFClockStateSink_FWD_DEFINED__ */


#ifndef __IMFPresentationDescriptor_FWD_DEFINED__
#define __IMFPresentationDescriptor_FWD_DEFINED__
typedef interface IMFPresentationDescriptor IMFPresentationDescriptor;

#endif 	/* __IMFPresentationDescriptor_FWD_DEFINED__ */


#ifndef __IMFStreamDescriptor_FWD_DEFINED__
#define __IMFStreamDescriptor_FWD_DEFINED__
typedef interface IMFStreamDescriptor IMFStreamDescriptor;

#endif 	/* __IMFStreamDescriptor_FWD_DEFINED__ */


#ifndef __IMFMediaTypeHandler_FWD_DEFINED__
#define __IMFMediaTypeHandler_FWD_DEFINED__
typedef interface IMFMediaTypeHandler IMFMediaTypeHandler;

#endif 	/* __IMFMediaTypeHandler_FWD_DEFINED__ */


#ifndef __IMFTimer_FWD_DEFINED__
#define __IMFTimer_FWD_DEFINED__
typedef interface IMFTimer IMFTimer;

#endif 	/* __IMFTimer_FWD_DEFINED__ */


#ifndef __IMFShutdown_FWD_DEFINED__
#define __IMFShutdown_FWD_DEFINED__
typedef interface IMFShutdown IMFShutdown;

#endif 	/* __IMFShutdown_FWD_DEFINED__ */


#ifndef __IMFTopoLoader_FWD_DEFINED__
#define __IMFTopoLoader_FWD_DEFINED__
typedef interface IMFTopoLoader IMFTopoLoader;

#endif 	/* __IMFTopoLoader_FWD_DEFINED__ */


#ifndef __IMFContentProtectionManager_FWD_DEFINED__
#define __IMFContentProtectionManager_FWD_DEFINED__
typedef interface IMFContentProtectionManager IMFContentProtectionManager;

#endif 	/* __IMFContentProtectionManager_FWD_DEFINED__ */


#ifndef __IMFContentEnabler_FWD_DEFINED__
#define __IMFContentEnabler_FWD_DEFINED__
typedef interface IMFContentEnabler IMFContentEnabler;

#endif 	/* __IMFContentEnabler_FWD_DEFINED__ */


#ifndef __IMFMetadata_FWD_DEFINED__
#define __IMFMetadata_FWD_DEFINED__
typedef interface IMFMetadata IMFMetadata;

#endif 	/* __IMFMetadata_FWD_DEFINED__ */


#ifndef __IMFMetadataProvider_FWD_DEFINED__
#define __IMFMetadataProvider_FWD_DEFINED__
typedef interface IMFMetadataProvider IMFMetadataProvider;

#endif 	/* __IMFMetadataProvider_FWD_DEFINED__ */


#ifndef __IMFRateSupport_FWD_DEFINED__
#define __IMFRateSupport_FWD_DEFINED__
typedef interface IMFRateSupport IMFRateSupport;

#endif 	/* __IMFRateSupport_FWD_DEFINED__ */


#ifndef __IMFRateControl_FWD_DEFINED__
#define __IMFRateControl_FWD_DEFINED__
typedef interface IMFRateControl IMFRateControl;

#endif 	/* __IMFRateControl_FWD_DEFINED__ */


#ifndef __IMFTimecodeTranslate_FWD_DEFINED__
#define __IMFTimecodeTranslate_FWD_DEFINED__
typedef interface IMFTimecodeTranslate IMFTimecodeTranslate;

#endif 	/* __IMFTimecodeTranslate_FWD_DEFINED__ */


#ifndef __IMFSeekInfo_FWD_DEFINED__
#define __IMFSeekInfo_FWD_DEFINED__
typedef interface IMFSeekInfo IMFSeekInfo;

#endif 	/* __IMFSeekInfo_FWD_DEFINED__ */


#ifndef __IMFSimpleAudioVolume_FWD_DEFINED__
#define __IMFSimpleAudioVolume_FWD_DEFINED__
typedef interface IMFSimpleAudioVolume IMFSimpleAudioVolume;

#endif 	/* __IMFSimpleAudioVolume_FWD_DEFINED__ */


#ifndef __IMFAudioStreamVolume_FWD_DEFINED__
#define __IMFAudioStreamVolume_FWD_DEFINED__
typedef interface IMFAudioStreamVolume IMFAudioStreamVolume;

#endif 	/* __IMFAudioStreamVolume_FWD_DEFINED__ */


#ifndef __IMFAudioPolicy_FWD_DEFINED__
#define __IMFAudioPolicy_FWD_DEFINED__
typedef interface IMFAudioPolicy IMFAudioPolicy;

#endif 	/* __IMFAudioPolicy_FWD_DEFINED__ */


#ifndef __IMFSampleGrabberSinkCallback_FWD_DEFINED__
#define __IMFSampleGrabberSinkCallback_FWD_DEFINED__
typedef interface IMFSampleGrabberSinkCallback IMFSampleGrabberSinkCallback;

#endif 	/* __IMFSampleGrabberSinkCallback_FWD_DEFINED__ */


#ifndef __IMFSampleGrabberSinkCallback2_FWD_DEFINED__
#define __IMFSampleGrabberSinkCallback2_FWD_DEFINED__
typedef interface IMFSampleGrabberSinkCallback2 IMFSampleGrabberSinkCallback2;

#endif 	/* __IMFSampleGrabberSinkCallback2_FWD_DEFINED__ */


#ifndef __IMFWorkQueueServices_FWD_DEFINED__
#define __IMFWorkQueueServices_FWD_DEFINED__
typedef interface IMFWorkQueueServices IMFWorkQueueServices;

#endif 	/* __IMFWorkQueueServices_FWD_DEFINED__ */


#ifndef __IMFWorkQueueServicesEx_FWD_DEFINED__
#define __IMFWorkQueueServicesEx_FWD_DEFINED__
typedef interface IMFWorkQueueServicesEx IMFWorkQueueServicesEx;

#endif 	/* __IMFWorkQueueServicesEx_FWD_DEFINED__ */


#ifndef __IMFQualityManager_FWD_DEFINED__
#define __IMFQualityManager_FWD_DEFINED__
typedef interface IMFQualityManager IMFQualityManager;

#endif 	/* __IMFQualityManager_FWD_DEFINED__ */


#ifndef __IMFQualityAdvise_FWD_DEFINED__
#define __IMFQualityAdvise_FWD_DEFINED__
typedef interface IMFQualityAdvise IMFQualityAdvise;

#endif 	/* __IMFQualityAdvise_FWD_DEFINED__ */


#ifndef __IMFQualityAdvise2_FWD_DEFINED__
#define __IMFQualityAdvise2_FWD_DEFINED__
typedef interface IMFQualityAdvise2 IMFQualityAdvise2;

#endif 	/* __IMFQualityAdvise2_FWD_DEFINED__ */


#ifndef __IMFQualityAdviseLimits_FWD_DEFINED__
#define __IMFQualityAdviseLimits_FWD_DEFINED__
typedef interface IMFQualityAdviseLimits IMFQualityAdviseLimits;

#endif 	/* __IMFQualityAdviseLimits_FWD_DEFINED__ */


#ifndef __IMFRealTimeClient_FWD_DEFINED__
#define __IMFRealTimeClient_FWD_DEFINED__
typedef interface IMFRealTimeClient IMFRealTimeClient;

#endif 	/* __IMFRealTimeClient_FWD_DEFINED__ */


#ifndef __IMFRealTimeClientEx_FWD_DEFINED__
#define __IMFRealTimeClientEx_FWD_DEFINED__
typedef interface IMFRealTimeClientEx IMFRealTimeClientEx;

#endif 	/* __IMFRealTimeClientEx_FWD_DEFINED__ */


#ifndef __IMFSequencerSource_FWD_DEFINED__
#define __IMFSequencerSource_FWD_DEFINED__
typedef interface IMFSequencerSource IMFSequencerSource;

#endif 	/* __IMFSequencerSource_FWD_DEFINED__ */


#ifndef __IMFMediaSourceTopologyProvider_FWD_DEFINED__
#define __IMFMediaSourceTopologyProvider_FWD_DEFINED__
typedef interface IMFMediaSourceTopologyProvider IMFMediaSourceTopologyProvider;

#endif 	/* __IMFMediaSourceTopologyProvider_FWD_DEFINED__ */


#ifndef __IMFMediaSourcePresentationProvider_FWD_DEFINED__
#define __IMFMediaSourcePresentationProvider_FWD_DEFINED__
typedef interface IMFMediaSourcePresentationProvider IMFMediaSourcePresentationProvider;

#endif 	/* __IMFMediaSourcePresentationProvider_FWD_DEFINED__ */


#ifndef __IMFTopologyNodeAttributeEditor_FWD_DEFINED__
#define __IMFTopologyNodeAttributeEditor_FWD_DEFINED__
typedef interface IMFTopologyNodeAttributeEditor IMFTopologyNodeAttributeEditor;

#endif 	/* __IMFTopologyNodeAttributeEditor_FWD_DEFINED__ */


#ifndef __IMFByteStreamBuffering_FWD_DEFINED__
#define __IMFByteStreamBuffering_FWD_DEFINED__
typedef interface IMFByteStreamBuffering IMFByteStreamBuffering;

#endif 	/* __IMFByteStreamBuffering_FWD_DEFINED__ */


#ifndef __IMFByteStreamCacheControl_FWD_DEFINED__
#define __IMFByteStreamCacheControl_FWD_DEFINED__
typedef interface IMFByteStreamCacheControl IMFByteStreamCacheControl;

#endif 	/* __IMFByteStreamCacheControl_FWD_DEFINED__ */


#ifndef __IMFByteStreamTimeSeek_FWD_DEFINED__
#define __IMFByteStreamTimeSeek_FWD_DEFINED__
typedef interface IMFByteStreamTimeSeek IMFByteStreamTimeSeek;

#endif 	/* __IMFByteStreamTimeSeek_FWD_DEFINED__ */


#ifndef __IMFByteStreamCacheControl2_FWD_DEFINED__
#define __IMFByteStreamCacheControl2_FWD_DEFINED__
typedef interface IMFByteStreamCacheControl2 IMFByteStreamCacheControl2;

#endif 	/* __IMFByteStreamCacheControl2_FWD_DEFINED__ */


#ifndef __IMFNetCredential_FWD_DEFINED__
#define __IMFNetCredential_FWD_DEFINED__
typedef interface IMFNetCredential IMFNetCredential;

#endif 	/* __IMFNetCredential_FWD_DEFINED__ */


#ifndef __IMFNetCredentialManager_FWD_DEFINED__
#define __IMFNetCredentialManager_FWD_DEFINED__
typedef interface IMFNetCredentialManager IMFNetCredentialManager;

#endif 	/* __IMFNetCredentialManager_FWD_DEFINED__ */


#ifndef __IMFNetCredentialCache_FWD_DEFINED__
#define __IMFNetCredentialCache_FWD_DEFINED__
typedef interface IMFNetCredentialCache IMFNetCredentialCache;

#endif 	/* __IMFNetCredentialCache_FWD_DEFINED__ */


#ifndef __IMFSSLCertificateManager_FWD_DEFINED__
#define __IMFSSLCertificateManager_FWD_DEFINED__
typedef interface IMFSSLCertificateManager IMFSSLCertificateManager;

#endif 	/* __IMFSSLCertificateManager_FWD_DEFINED__ */


#ifndef __IMFNetResourceFilter_FWD_DEFINED__
#define __IMFNetResourceFilter_FWD_DEFINED__
typedef interface IMFNetResourceFilter IMFNetResourceFilter;

#endif 	/* __IMFNetResourceFilter_FWD_DEFINED__ */


#ifndef __IMFSourceOpenMonitor_FWD_DEFINED__
#define __IMFSourceOpenMonitor_FWD_DEFINED__
typedef interface IMFSourceOpenMonitor IMFSourceOpenMonitor;

#endif 	/* __IMFSourceOpenMonitor_FWD_DEFINED__ */


#ifndef __IMFNetProxyLocator_FWD_DEFINED__
#define __IMFNetProxyLocator_FWD_DEFINED__
typedef interface IMFNetProxyLocator IMFNetProxyLocator;

#endif 	/* __IMFNetProxyLocator_FWD_DEFINED__ */


#ifndef __IMFNetProxyLocatorFactory_FWD_DEFINED__
#define __IMFNetProxyLocatorFactory_FWD_DEFINED__
typedef interface IMFNetProxyLocatorFactory IMFNetProxyLocatorFactory;

#endif 	/* __IMFNetProxyLocatorFactory_FWD_DEFINED__ */


#ifndef __IMFSaveJob_FWD_DEFINED__
#define __IMFSaveJob_FWD_DEFINED__
typedef interface IMFSaveJob IMFSaveJob;

#endif 	/* __IMFSaveJob_FWD_DEFINED__ */


#ifndef __IMFNetSchemeHandlerConfig_FWD_DEFINED__
#define __IMFNetSchemeHandlerConfig_FWD_DEFINED__
typedef interface IMFNetSchemeHandlerConfig IMFNetSchemeHandlerConfig;

#endif 	/* __IMFNetSchemeHandlerConfig_FWD_DEFINED__ */


#ifndef __IMFSchemeHandler_FWD_DEFINED__
#define __IMFSchemeHandler_FWD_DEFINED__
typedef interface IMFSchemeHandler IMFSchemeHandler;

#endif 	/* __IMFSchemeHandler_FWD_DEFINED__ */


#ifndef __IMFByteStreamHandler_FWD_DEFINED__
#define __IMFByteStreamHandler_FWD_DEFINED__
typedef interface IMFByteStreamHandler IMFByteStreamHandler;

#endif 	/* __IMFByteStreamHandler_FWD_DEFINED__ */


#ifndef __IMFTrustedInput_FWD_DEFINED__
#define __IMFTrustedInput_FWD_DEFINED__
typedef interface IMFTrustedInput IMFTrustedInput;

#endif 	/* __IMFTrustedInput_FWD_DEFINED__ */


#ifndef __IMFInputTrustAuthority_FWD_DEFINED__
#define __IMFInputTrustAuthority_FWD_DEFINED__
typedef interface IMFInputTrustAuthority IMFInputTrustAuthority;

#endif 	/* __IMFInputTrustAuthority_FWD_DEFINED__ */


#ifndef __IMFTrustedOutput_FWD_DEFINED__
#define __IMFTrustedOutput_FWD_DEFINED__
typedef interface IMFTrustedOutput IMFTrustedOutput;

#endif 	/* __IMFTrustedOutput_FWD_DEFINED__ */


#ifndef __IMFOutputTrustAuthority_FWD_DEFINED__
#define __IMFOutputTrustAuthority_FWD_DEFINED__
typedef interface IMFOutputTrustAuthority IMFOutputTrustAuthority;

#endif 	/* __IMFOutputTrustAuthority_FWD_DEFINED__ */


#ifndef __IMFOutputPolicy_FWD_DEFINED__
#define __IMFOutputPolicy_FWD_DEFINED__
typedef interface IMFOutputPolicy IMFOutputPolicy;

#endif 	/* __IMFOutputPolicy_FWD_DEFINED__ */


#ifndef __IMFOutputSchema_FWD_DEFINED__
#define __IMFOutputSchema_FWD_DEFINED__
typedef interface IMFOutputSchema IMFOutputSchema;

#endif 	/* __IMFOutputSchema_FWD_DEFINED__ */


#ifndef __IMFSecureChannel_FWD_DEFINED__
#define __IMFSecureChannel_FWD_DEFINED__
typedef interface IMFSecureChannel IMFSecureChannel;

#endif 	/* __IMFSecureChannel_FWD_DEFINED__ */


#ifndef __IMFSampleProtection_FWD_DEFINED__
#define __IMFSampleProtection_FWD_DEFINED__
typedef interface IMFSampleProtection IMFSampleProtection;

#endif 	/* __IMFSampleProtection_FWD_DEFINED__ */


#ifndef __IMFMediaSinkPreroll_FWD_DEFINED__
#define __IMFMediaSinkPreroll_FWD_DEFINED__
typedef interface IMFMediaSinkPreroll IMFMediaSinkPreroll;

#endif 	/* __IMFMediaSinkPreroll_FWD_DEFINED__ */


#ifndef __IMFFinalizableMediaSink_FWD_DEFINED__
#define __IMFFinalizableMediaSink_FWD_DEFINED__
typedef interface IMFFinalizableMediaSink IMFFinalizableMediaSink;

#endif 	/* __IMFFinalizableMediaSink_FWD_DEFINED__ */


#ifndef __IMFStreamingSinkConfig_FWD_DEFINED__
#define __IMFStreamingSinkConfig_FWD_DEFINED__
typedef interface IMFStreamingSinkConfig IMFStreamingSinkConfig;

#endif 	/* __IMFStreamingSinkConfig_FWD_DEFINED__ */


#ifndef __IMFRemoteProxy_FWD_DEFINED__
#define __IMFRemoteProxy_FWD_DEFINED__
typedef interface IMFRemoteProxy IMFRemoteProxy;

#endif 	/* __IMFRemoteProxy_FWD_DEFINED__ */


#ifndef __IMFObjectReferenceStream_FWD_DEFINED__
#define __IMFObjectReferenceStream_FWD_DEFINED__
typedef interface IMFObjectReferenceStream IMFObjectReferenceStream;

#endif 	/* __IMFObjectReferenceStream_FWD_DEFINED__ */


#ifndef __IMFPMPHost_FWD_DEFINED__
#define __IMFPMPHost_FWD_DEFINED__
typedef interface IMFPMPHost IMFPMPHost;

#endif 	/* __IMFPMPHost_FWD_DEFINED__ */


#ifndef __IMFPMPClient_FWD_DEFINED__
#define __IMFPMPClient_FWD_DEFINED__
typedef interface IMFPMPClient IMFPMPClient;

#endif 	/* __IMFPMPClient_FWD_DEFINED__ */


#ifndef __IMFPMPServer_FWD_DEFINED__
#define __IMFPMPServer_FWD_DEFINED__
typedef interface IMFPMPServer IMFPMPServer;

#endif 	/* __IMFPMPServer_FWD_DEFINED__ */


#ifndef __IMFRemoteDesktopPlugin_FWD_DEFINED__
#define __IMFRemoteDesktopPlugin_FWD_DEFINED__
typedef interface IMFRemoteDesktopPlugin IMFRemoteDesktopPlugin;

#endif 	/* __IMFRemoteDesktopPlugin_FWD_DEFINED__ */


#ifndef __IMFSAMIStyle_FWD_DEFINED__
#define __IMFSAMIStyle_FWD_DEFINED__
typedef interface IMFSAMIStyle IMFSAMIStyle;

#endif 	/* __IMFSAMIStyle_FWD_DEFINED__ */


#ifndef __IMFTranscodeProfile_FWD_DEFINED__
#define __IMFTranscodeProfile_FWD_DEFINED__
typedef interface IMFTranscodeProfile IMFTranscodeProfile;

#endif 	/* __IMFTranscodeProfile_FWD_DEFINED__ */


#ifndef __IMFTranscodeSinkInfoProvider_FWD_DEFINED__
#define __IMFTranscodeSinkInfoProvider_FWD_DEFINED__
typedef interface IMFTranscodeSinkInfoProvider IMFTranscodeSinkInfoProvider;

#endif 	/* __IMFTranscodeSinkInfoProvider_FWD_DEFINED__ */


#ifndef __IMFFieldOfUseMFTUnlock_FWD_DEFINED__
#define __IMFFieldOfUseMFTUnlock_FWD_DEFINED__
typedef interface IMFFieldOfUseMFTUnlock IMFFieldOfUseMFTUnlock;

#endif 	/* __IMFFieldOfUseMFTUnlock_FWD_DEFINED__ */


#ifndef __IMFLocalMFTRegistration_FWD_DEFINED__
#define __IMFLocalMFTRegistration_FWD_DEFINED__
typedef interface IMFLocalMFTRegistration IMFLocalMFTRegistration;

#endif 	/* __IMFLocalMFTRegistration_FWD_DEFINED__ */


#ifndef __IMFCapturePhotoConfirmation_FWD_DEFINED__
#define __IMFCapturePhotoConfirmation_FWD_DEFINED__
typedef interface IMFCapturePhotoConfirmation IMFCapturePhotoConfirmation;

#endif 	/* __IMFCapturePhotoConfirmation_FWD_DEFINED__ */


#ifndef __IMFPMPHostApp_FWD_DEFINED__
#define __IMFPMPHostApp_FWD_DEFINED__
typedef interface IMFPMPHostApp IMFPMPHostApp;

#endif 	/* __IMFPMPHostApp_FWD_DEFINED__ */


#ifndef __IMFPMPClientApp_FWD_DEFINED__
#define __IMFPMPClientApp_FWD_DEFINED__
typedef interface IMFPMPClientApp IMFPMPClientApp;

#endif 	/* __IMFPMPClientApp_FWD_DEFINED__ */


#ifndef __IMFMediaStreamSourceSampleRequest_FWD_DEFINED__
#define __IMFMediaStreamSourceSampleRequest_FWD_DEFINED__
typedef interface IMFMediaStreamSourceSampleRequest IMFMediaStreamSourceSampleRequest;

#endif 	/* __IMFMediaStreamSourceSampleRequest_FWD_DEFINED__ */


#ifndef __IMFTrackedSample_FWD_DEFINED__
#define __IMFTrackedSample_FWD_DEFINED__
typedef interface IMFTrackedSample IMFTrackedSample;

#endif 	/* __IMFTrackedSample_FWD_DEFINED__ */


#ifndef __IMFProtectedEnvironmentAccess_FWD_DEFINED__
#define __IMFProtectedEnvironmentAccess_FWD_DEFINED__
typedef interface IMFProtectedEnvironmentAccess IMFProtectedEnvironmentAccess;

#endif 	/* __IMFProtectedEnvironmentAccess_FWD_DEFINED__ */


#ifndef __IMFSignedLibrary_FWD_DEFINED__
#define __IMFSignedLibrary_FWD_DEFINED__
typedef interface IMFSignedLibrary IMFSignedLibrary;

#endif 	/* __IMFSignedLibrary_FWD_DEFINED__ */


#ifndef __IMFSystemId_FWD_DEFINED__
#define __IMFSystemId_FWD_DEFINED__
typedef interface IMFSystemId IMFSystemId;

#endif 	/* __IMFSystemId_FWD_DEFINED__ */


#ifndef __IMFContentProtectionDevice_FWD_DEFINED__
#define __IMFContentProtectionDevice_FWD_DEFINED__
typedef interface IMFContentProtectionDevice IMFContentProtectionDevice;

#endif 	/* __IMFContentProtectionDevice_FWD_DEFINED__ */


#ifndef __IMFContentDecryptorContext_FWD_DEFINED__
#define __IMFContentDecryptorContext_FWD_DEFINED__
typedef interface IMFContentDecryptorContext IMFContentDecryptorContext;

#endif 	/* __IMFContentDecryptorContext_FWD_DEFINED__ */


#ifndef __IMFNetCrossOriginSupport_FWD_DEFINED__
#define __IMFNetCrossOriginSupport_FWD_DEFINED__
typedef interface IMFNetCrossOriginSupport IMFNetCrossOriginSupport;

#endif 	/* __IMFNetCrossOriginSupport_FWD_DEFINED__ */


#ifndef __IMFHttpDownloadRequest_FWD_DEFINED__
#define __IMFHttpDownloadRequest_FWD_DEFINED__
typedef interface IMFHttpDownloadRequest IMFHttpDownloadRequest;

#endif 	/* __IMFHttpDownloadRequest_FWD_DEFINED__ */


#ifndef __IMFHttpDownloadSession_FWD_DEFINED__
#define __IMFHttpDownloadSession_FWD_DEFINED__
typedef interface IMFHttpDownloadSession IMFHttpDownloadSession;

#endif 	/* __IMFHttpDownloadSession_FWD_DEFINED__ */


#ifndef __IMFHttpDownloadSessionProvider_FWD_DEFINED__
#define __IMFHttpDownloadSessionProvider_FWD_DEFINED__
typedef interface IMFHttpDownloadSessionProvider IMFHttpDownloadSessionProvider;

#endif 	/* __IMFHttpDownloadSessionProvider_FWD_DEFINED__ */


#ifndef __IMFMediaSource2_FWD_DEFINED__
#define __IMFMediaSource2_FWD_DEFINED__
typedef interface IMFMediaSource2 IMFMediaSource2;

#endif 	/* __IMFMediaSource2_FWD_DEFINED__ */


#ifndef __IMFMediaStream2_FWD_DEFINED__
#define __IMFMediaStream2_FWD_DEFINED__
typedef interface IMFMediaStream2 IMFMediaStream2;

#endif 	/* __IMFMediaStream2_FWD_DEFINED__ */


#ifndef __IMFSensorDevice_FWD_DEFINED__
#define __IMFSensorDevice_FWD_DEFINED__
typedef interface IMFSensorDevice IMFSensorDevice;

#endif 	/* __IMFSensorDevice_FWD_DEFINED__ */


#ifndef __IMFSensorGroup_FWD_DEFINED__
#define __IMFSensorGroup_FWD_DEFINED__
typedef interface IMFSensorGroup IMFSensorGroup;

#endif 	/* __IMFSensorGroup_FWD_DEFINED__ */


#ifndef __IMFSensorStream_FWD_DEFINED__
#define __IMFSensorStream_FWD_DEFINED__
typedef interface IMFSensorStream IMFSensorStream;

#endif 	/* __IMFSensorStream_FWD_DEFINED__ */


#ifndef __IMFSensorTransformFactory_FWD_DEFINED__
#define __IMFSensorTransformFactory_FWD_DEFINED__
typedef interface IMFSensorTransformFactory IMFSensorTransformFactory;

#endif 	/* __IMFSensorTransformFactory_FWD_DEFINED__ */


#ifndef __IMFSensorProfile_FWD_DEFINED__
#define __IMFSensorProfile_FWD_DEFINED__
typedef interface IMFSensorProfile IMFSensorProfile;

#endif 	/* __IMFSensorProfile_FWD_DEFINED__ */


#ifndef __IMFSensorProfileCollection_FWD_DEFINED__
#define __IMFSensorProfileCollection_FWD_DEFINED__
typedef interface IMFSensorProfileCollection IMFSensorProfileCollection;

#endif 	/* __IMFSensorProfileCollection_FWD_DEFINED__ */


#ifndef __IMFSensorProcessActivity_FWD_DEFINED__
#define __IMFSensorProcessActivity_FWD_DEFINED__
typedef interface IMFSensorProcessActivity IMFSensorProcessActivity;

#endif 	/* __IMFSensorProcessActivity_FWD_DEFINED__ */


#ifndef __IMFSensorActivityReport_FWD_DEFINED__
#define __IMFSensorActivityReport_FWD_DEFINED__
typedef interface IMFSensorActivityReport IMFSensorActivityReport;

#endif 	/* __IMFSensorActivityReport_FWD_DEFINED__ */


#ifndef __IMFSensorActivitiesReport_FWD_DEFINED__
#define __IMFSensorActivitiesReport_FWD_DEFINED__
typedef interface IMFSensorActivitiesReport IMFSensorActivitiesReport;

#endif 	/* __IMFSensorActivitiesReport_FWD_DEFINED__ */


#ifndef __IMFSensorActivitiesReportCallback_FWD_DEFINED__
#define __IMFSensorActivitiesReportCallback_FWD_DEFINED__
typedef interface IMFSensorActivitiesReportCallback IMFSensorActivitiesReportCallback;

#endif 	/* __IMFSensorActivitiesReportCallback_FWD_DEFINED__ */


#ifndef __IMFSensorActivityMonitor_FWD_DEFINED__
#define __IMFSensorActivityMonitor_FWD_DEFINED__
typedef interface IMFSensorActivityMonitor IMFSensorActivityMonitor;

#endif 	/* __IMFSensorActivityMonitor_FWD_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsicModel_FWD_DEFINED__
#define __IMFExtendedCameraIntrinsicModel_FWD_DEFINED__
typedef interface IMFExtendedCameraIntrinsicModel IMFExtendedCameraIntrinsicModel;

#endif 	/* __IMFExtendedCameraIntrinsicModel_FWD_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsicsDistortionModel6KT_FWD_DEFINED__
#define __IMFExtendedCameraIntrinsicsDistortionModel6KT_FWD_DEFINED__
typedef interface IMFExtendedCameraIntrinsicsDistortionModel6KT IMFExtendedCameraIntrinsicsDistortionModel6KT;

#endif 	/* __IMFExtendedCameraIntrinsicsDistortionModel6KT_FWD_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsicsDistortionModelArcTan_FWD_DEFINED__
#define __IMFExtendedCameraIntrinsicsDistortionModelArcTan_FWD_DEFINED__
typedef interface IMFExtendedCameraIntrinsicsDistortionModelArcTan IMFExtendedCameraIntrinsicsDistortionModelArcTan;

#endif 	/* __IMFExtendedCameraIntrinsicsDistortionModelArcTan_FWD_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsics_FWD_DEFINED__
#define __IMFExtendedCameraIntrinsics_FWD_DEFINED__
typedef interface IMFExtendedCameraIntrinsics IMFExtendedCameraIntrinsics;

#endif 	/* __IMFExtendedCameraIntrinsics_FWD_DEFINED__ */


#ifndef __IMFExtendedCameraControl_FWD_DEFINED__
#define __IMFExtendedCameraControl_FWD_DEFINED__
typedef interface IMFExtendedCameraControl IMFExtendedCameraControl;

#endif 	/* __IMFExtendedCameraControl_FWD_DEFINED__ */


#ifndef __IMFExtendedCameraController_FWD_DEFINED__
#define __IMFExtendedCameraController_FWD_DEFINED__
typedef interface IMFExtendedCameraController IMFExtendedCameraController;

#endif 	/* __IMFExtendedCameraController_FWD_DEFINED__ */


#ifndef __IMFRelativePanelReport_FWD_DEFINED__
#define __IMFRelativePanelReport_FWD_DEFINED__
typedef interface IMFRelativePanelReport IMFRelativePanelReport;

#endif 	/* __IMFRelativePanelReport_FWD_DEFINED__ */


#ifndef __IMFRelativePanelWatcher_FWD_DEFINED__
#define __IMFRelativePanelWatcher_FWD_DEFINED__
typedef interface IMFRelativePanelWatcher IMFRelativePanelWatcher;

#endif 	/* __IMFRelativePanelWatcher_FWD_DEFINED__ */


#ifndef __IMFVideoCaptureSampleAllocator_FWD_DEFINED__
#define __IMFVideoCaptureSampleAllocator_FWD_DEFINED__
typedef interface IMFVideoCaptureSampleAllocator IMFVideoCaptureSampleAllocator;

#endif 	/* __IMFVideoCaptureSampleAllocator_FWD_DEFINED__ */


#ifndef __IMFSampleAllocatorControl_FWD_DEFINED__
#define __IMFSampleAllocatorControl_FWD_DEFINED__
typedef interface IMFSampleAllocatorControl IMFSampleAllocatorControl;

#endif 	/* __IMFSampleAllocatorControl_FWD_DEFINED__ */


#ifndef __IMFCameraOcclusionStateReport_FWD_DEFINED__
#define __IMFCameraOcclusionStateReport_FWD_DEFINED__
typedef interface IMFCameraOcclusionStateReport IMFCameraOcclusionStateReport;

#endif 	/* __IMFCameraOcclusionStateReport_FWD_DEFINED__ */


#ifndef __IMFCameraOcclusionStateReportCallback_FWD_DEFINED__
#define __IMFCameraOcclusionStateReportCallback_FWD_DEFINED__
typedef interface IMFCameraOcclusionStateReportCallback IMFCameraOcclusionStateReportCallback;

#endif 	/* __IMFCameraOcclusionStateReportCallback_FWD_DEFINED__ */


#ifndef __IMFCameraOcclusionStateMonitor_FWD_DEFINED__
#define __IMFCameraOcclusionStateMonitor_FWD_DEFINED__
typedef interface IMFCameraOcclusionStateMonitor IMFCameraOcclusionStateMonitor;

#endif 	/* __IMFCameraOcclusionStateMonitor_FWD_DEFINED__ */


#ifndef __IMFCameraControlNotify_FWD_DEFINED__
#define __IMFCameraControlNotify_FWD_DEFINED__
typedef interface IMFCameraControlNotify IMFCameraControlNotify;

#endif 	/* __IMFCameraControlNotify_FWD_DEFINED__ */


#ifndef __IMFCameraControlMonitor_FWD_DEFINED__
#define __IMFCameraControlMonitor_FWD_DEFINED__
typedef interface IMFCameraControlMonitor IMFCameraControlMonitor;

#endif 	/* __IMFCameraControlMonitor_FWD_DEFINED__ */


#ifndef __IMFCameraControlDefaults_FWD_DEFINED__
#define __IMFCameraControlDefaults_FWD_DEFINED__
typedef interface IMFCameraControlDefaults IMFCameraControlDefaults;

#endif 	/* __IMFCameraControlDefaults_FWD_DEFINED__ */


#ifndef __IMFCameraControlDefaultsCollection_FWD_DEFINED__
#define __IMFCameraControlDefaultsCollection_FWD_DEFINED__
typedef interface IMFCameraControlDefaultsCollection IMFCameraControlDefaultsCollection;

#endif 	/* __IMFCameraControlDefaultsCollection_FWD_DEFINED__ */


#ifndef __IMFCameraConfigurationManager_FWD_DEFINED__
#define __IMFCameraConfigurationManager_FWD_DEFINED__
typedef interface IMFCameraConfigurationManager IMFCameraConfigurationManager;

#endif 	/* __IMFCameraConfigurationManager_FWD_DEFINED__ */


#ifndef __IMFFaceDetectionTransformCallback_FWD_DEFINED__
#define __IMFFaceDetectionTransformCallback_FWD_DEFINED__
typedef interface IMFFaceDetectionTransformCallback IMFFaceDetectionTransformCallback;

#endif 	/* __IMFFaceDetectionTransformCallback_FWD_DEFINED__ */


#ifndef __IMFFaceDetectionTransform_FWD_DEFINED__
#define __IMFFaceDetectionTransform_FWD_DEFINED__
typedef interface IMFFaceDetectionTransform IMFFaceDetectionTransform;

#endif 	/* __IMFFaceDetectionTransform_FWD_DEFINED__ */


/* header files for imported files */
#include "mfobjects.h"
#include "mftransform.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_mfidl_0000_0000 */
/* [local] */ 

#include <winapifamily.h>
#include <windef.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum MFSESSION_SETTOPOLOGY_FLAGS
    {
        MFSESSION_SETTOPOLOGY_IMMEDIATE	= 0x1,
        MFSESSION_SETTOPOLOGY_NORESOLUTION	= 0x2,
        MFSESSION_SETTOPOLOGY_CLEAR_CURRENT	= 0x4
    } 	MFSESSION_SETTOPOLOGY_FLAGS;

typedef 
enum MFSESSION_GETFULLTOPOLOGY_FLAGS
    {
        MFSESSION_GETFULLTOPOLOGY_CURRENT	= 0x1
    } 	MFSESSION_GETFULLTOPOLOGY_FLAGS;

typedef 
enum MFPMPSESSION_CREATION_FLAGS
    {
        MFPMPSESSION_UNPROTECTED_PROCESS	= 0x1,
        MFPMPSESSION_IN_PROCESS	= 0x2
    } 	MFPMPSESSION_CREATION_FLAGS;

typedef unsigned __int64 TOPOID;

#if (NTDDI_VERSION >= NTDDI_WIN11_ZN) 
EXTERN_GUID( MF_ACOUSTIC_ECHO_CANCELLATION_CONTROL_SERVICE, 0x7f6c3b29, 0x2d12, 0x4f6f, 0xac, 0x5, 0xc1, 0xa8, 0x9b, 0x8d, 0x52, 0x88);
EXTERN_GUID( MF_AUDIO_EFFECTS_MANAGER_SERVICE, 0x1f541943, 0xd5df, 0x455e, 0xa2, 0xe5, 0x7d, 0x64, 0xd3, 0xbb, 0xbd, 0xb5);
#endif // (NTDDI_VERSION >= NTDDI_WIN11_ZN) 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


EXTERN_GUID( MF_WVC1_PROG_SINGLE_SLICE_CONTENT, 0x67EC2559, 0x0F2F, 0x4420, 0xA4, 0xDD, 0x2F, 0x8E, 0xE7, 0xA5, 0x73, 0x8B);
EXTERN_GUID( MF_PROGRESSIVE_CODING_CONTENT, 0x8F020EEA, 0x1508, 0x471F, 0x9D, 0xA6, 0x50, 0x7D, 0x7C, 0xFA, 0x40, 0xDB);
EXTERN_GUID( MF_NALU_LENGTH_SET, 0xA7911D53, 0x12A4, 0x4965, 0xAE, 0x70, 0x6E, 0xAD, 0xD6, 0xFF, 0x05, 0x51);
EXTERN_GUID( MF_NALU_LENGTH_INFORMATION,  0x19124E7C, 0xAD4B, 0x465F, 0xBB, 0x18, 0x20, 0x18, 0x62, 0x87, 0xB6, 0xAF);
EXTERN_GUID( MF_USER_DATA_PAYLOAD,  0xd1d4985d, 0xdc92, 0x457a, 0xb3, 0xa0, 0x65, 0x1a, 0x33, 0xa3, 0x10, 0x47);
EXTERN_GUID( MF_MPEG4SINK_SPSPPS_PASSTHROUGH, 0x5601a134, 0x2005, 0x4ad2, 0xb3, 0x7d, 0x22, 0xa6, 0xc5, 0x54, 0xde, 0xb2);
EXTERN_GUID( MF_MPEG4SINK_MOOV_BEFORE_MDAT, 0xf672e3ac, 0xe1e6, 0x4f10, 0xb5, 0xec, 0x5f, 0x3b, 0x30, 0x82, 0x88, 0x16);
EXTERN_GUID( MF_MPEG4SINK_MINIMUM_PROPERTIES_SIZE, 0xdca1ed52, 0x450e, 0x4a22, 0x8c, 0x62, 0x4e, 0xd4, 0x52, 0xf7, 0xa1, 0x87);
EXTERN_GUID(MF_MPEG4SINK_MIN_FRAGMENT_DURATION, 0xa30b570c, 0x8efd, 0x45e8, 0x94, 0xfe, 0x27, 0xc8, 0x4b, 0x5b, 0xdf, 0xf6);
EXTERN_GUID(MF_MPEG4SINK_MAX_CODED_SEQUENCES_PER_FRAGMENT, 0xfc1b3bd6, 0x692d, 0x4ce5, 0x92, 0x99, 0x73, 0x8a, 0xa5, 0x46, 0x3e, 0x9a);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0000_v0_0_s_ifspec;

#ifndef __IMFMediaSession_INTERFACE_DEFINED__
#define __IMFMediaSession_INTERFACE_DEFINED__

/* interface IMFMediaSession */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("90377834-21D0-4dee-8214-BA2E3E6C1127")
    IMFMediaSession : public IMFMediaEventGenerator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetTopology( 
            /* [in] */ DWORD dwSetTopologyFlags,
            /* [in] */ __RPC__in_opt IMFTopology *pTopology) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearTopologies( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [unique][in] */ __RPC__in_opt const GUID *pguidTimeFormat,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvarStartPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClock( 
            /* [out] */ __RPC__deref_out_opt IMFClock **ppClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSessionCapabilities( 
            /* [out] */ __RPC__out DWORD *pdwCaps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFullTopology( 
            /* [in] */ DWORD dwGetFullTopologyFlags,
            /* [in] */ TOPOID TopoId,
            /* [out] */ __RPC__deref_out_opt IMFTopology **ppFullTopology) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSession * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMFMediaSession * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaSession * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaSession * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            __RPC__in IMFMediaSession * This,
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, SetTopology)
        HRESULT ( STDMETHODCALLTYPE *SetTopology )( 
            __RPC__in IMFMediaSession * This,
            /* [in] */ DWORD dwSetTopologyFlags,
            /* [in] */ __RPC__in_opt IMFTopology *pTopology);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, ClearTopologies)
        HRESULT ( STDMETHODCALLTYPE *ClearTopologies )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IMFMediaSession * This,
            /* [unique][in] */ __RPC__in_opt const GUID *pguidTimeFormat,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvarStartPosition);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IMFMediaSession * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, GetClock)
        HRESULT ( STDMETHODCALLTYPE *GetClock )( 
            __RPC__in IMFMediaSession * This,
            /* [out] */ __RPC__deref_out_opt IMFClock **ppClock);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, GetSessionCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetSessionCapabilities )( 
            __RPC__in IMFMediaSession * This,
            /* [out] */ __RPC__out DWORD *pdwCaps);
        
        DECLSPEC_XFGVIRT(IMFMediaSession, GetFullTopology)
        HRESULT ( STDMETHODCALLTYPE *GetFullTopology )( 
            __RPC__in IMFMediaSession * This,
            /* [in] */ DWORD dwGetFullTopologyFlags,
            /* [in] */ TOPOID TopoId,
            /* [out] */ __RPC__deref_out_opt IMFTopology **ppFullTopology);
        
        END_INTERFACE
    } IMFMediaSessionVtbl;

    interface IMFMediaSession
    {
        CONST_VTBL struct IMFMediaSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSession_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaSession_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaSession_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaSession_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFMediaSession_SetTopology(This,dwSetTopologyFlags,pTopology)	\
    ( (This)->lpVtbl -> SetTopology(This,dwSetTopologyFlags,pTopology) ) 

#define IMFMediaSession_ClearTopologies(This)	\
    ( (This)->lpVtbl -> ClearTopologies(This) ) 

#define IMFMediaSession_Start(This,pguidTimeFormat,pvarStartPosition)	\
    ( (This)->lpVtbl -> Start(This,pguidTimeFormat,pvarStartPosition) ) 

#define IMFMediaSession_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaSession_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFMediaSession_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#define IMFMediaSession_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFMediaSession_GetClock(This,ppClock)	\
    ( (This)->lpVtbl -> GetClock(This,ppClock) ) 

#define IMFMediaSession_GetSessionCapabilities(This,pdwCaps)	\
    ( (This)->lpVtbl -> GetSessionCapabilities(This,pdwCaps) ) 

#define IMFMediaSession_GetFullTopology(This,dwGetFullTopologyFlags,TopoId,ppFullTopology)	\
    ( (This)->lpVtbl -> GetFullTopology(This,dwGetFullTopologyFlags,TopoId,ppFullTopology) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSession_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0001 */
/* [local] */ 

EXTERN_GUID( MF_SESSION_TOPOLOADER, 0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x71);
EXTERN_GUID( MF_SESSION_GLOBAL_TIME, 0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x72);
EXTERN_GUID( MF_SESSION_QUALITY_MANAGER, 0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x73);
EXTERN_GUID( MF_SESSION_CONTENT_PROTECTION_MANAGER, 0x1e83d482, 0x1f1c, 0x4571, 0x84, 0x5, 0x88, 0xf4, 0xb2, 0x18, 0x1f, 0x74);
EXTERN_GUID( MF_SESSION_SERVER_CONTEXT, 0xafe5b291, 0x50fa, 0x46e8, 0xb9, 0xbe, 0xc, 0xc, 0x3c, 0xe4, 0xb3, 0xa5);
EXTERN_GUID( MF_SESSION_REMOTE_SOURCE_MODE, 0xf4033ef4, 0x9bb3, 0x4378, 0x94, 0x1f, 0x85, 0xa0, 0x85, 0x6b, 0xc2, 0x44);
EXTERN_GUID( MF_SESSION_APPROX_EVENT_OCCURRENCE_TIME, 0x190e852f, 0x6238, 0x42d1, 0xb5, 0xaf, 0x69, 0xea, 0x33, 0x8e, 0xf8, 0x50);
EXTERN_GUID( MF_PMP_SERVER_CONTEXT, 0x2f00c910, 0xd2cf, 0x4278, 0x8b, 0x6a, 0xd0, 0x77, 0xfa, 0xc3, 0xa2, 0x5f);
STDAPI MFCreateMediaSession(
    IMFAttributes* pConfiguration,
    _Outptr_ IMFMediaSession** ppMediaSession
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region PC Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)
STDAPI MFCreatePMPMediaSession(
    DWORD dwCreationFlags,
    IMFAttributes *pConfiguration,
    _Outptr_ IMFMediaSession** ppMediaSession,
    _Outptr_opt_ IMFActivate **ppEnablerActivate
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum MF_OBJECT_TYPE
    {
        MF_OBJECT_MEDIASOURCE	= 0,
        MF_OBJECT_BYTESTREAM	= ( MF_OBJECT_MEDIASOURCE + 1 ) ,
        MF_OBJECT_INVALID	= ( MF_OBJECT_BYTESTREAM + 1 ) 
    } 	MF_OBJECT_TYPE;


enum __MIDL___MIDL_itf_mfidl_0000_0001_0001
    {
        MF_RESOLUTION_MEDIASOURCE	= 0x1,
        MF_RESOLUTION_BYTESTREAM	= 0x2,
        MF_RESOLUTION_CONTENT_DOES_NOT_HAVE_TO_MATCH_EXTENSION_OR_MIME_TYPE	= 0x10,
        MF_RESOLUTION_KEEP_BYTE_STREAM_ALIVE_ON_FAIL	= 0x20,
        MF_RESOLUTION_DISABLE_LOCAL_PLUGINS	= 0x40,
        MF_RESOLUTION_PLUGIN_CONTROL_POLICY_APPROVED_ONLY	= 0x80,
        MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY	= 0x100,
        MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY_EDGEMODE	= 0x200,
        MF_RESOLUTION_ENABLE_STORE_PLUGINS	= 0x400,
        MF_RESOLUTION_READ	= 0x10000,
        MF_RESOLUTION_WRITE	= 0x20000
    } ;
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum _MF_CONNECT_METHOD
    {
        MF_CONNECT_DIRECT	= 0,
        MF_CONNECT_ALLOW_CONVERTER	= 0x1,
        MF_CONNECT_ALLOW_DECODER	= 0x3,
        MF_CONNECT_RESOLVE_INDEPENDENT_OUTPUTTYPES	= 0x4,
        MF_CONNECT_AS_OPTIONAL	= 0x10000,
        MF_CONNECT_AS_OPTIONAL_BRANCH	= 0x20000
    } 	MF_CONNECT_METHOD;

typedef 
enum _MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS
    {
        MF_TOPOLOGY_RESOLUTION_SUCCEEDED	= 0,
        MF_OPTIONAL_NODE_REJECTED_MEDIA_TYPE	= 0x1,
        MF_OPTIONAL_NODE_REJECTED_PROTECTED_PROCESS	= 0x2
    } 	MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS;


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0001_v0_0_s_ifspec;

#ifndef __IMFSourceResolver_INTERFACE_DEFINED__
#define __IMFSourceResolver_INTERFACE_DEFINED__

/* interface IMFSourceResolver */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFSourceResolver;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FBE5A32D-A497-4b61-BB85-97B1A848A6E3")
    IMFSourceResolver : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateObjectFromURL( 
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateObjectFromByteStream( 
            /* [in] */ IMFByteStream *pByteStream,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginCreateObjectFromURL( 
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndCreateObjectFromURL( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginCreateObjectFromByteStream( 
            /* [in] */ IMFByteStream *pByteStream,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndCreateObjectFromByteStream( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CancelObjectCreation( 
            /* [in] */ IUnknown *pIUnknownCancelCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceResolverVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFSourceResolver * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFSourceResolver * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFSourceResolver * This);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, CreateObjectFromURL)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateObjectFromURL )( 
            IMFSourceResolver * This,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, CreateObjectFromByteStream)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateObjectFromByteStream )( 
            IMFSourceResolver * This,
            /* [in] */ IMFByteStream *pByteStream,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, BeginCreateObjectFromURL)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginCreateObjectFromURL )( 
            IMFSourceResolver * This,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, EndCreateObjectFromURL)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndCreateObjectFromURL )( 
            IMFSourceResolver * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, BeginCreateObjectFromByteStream)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginCreateObjectFromByteStream )( 
            IMFSourceResolver * This,
            /* [in] */ IMFByteStream *pByteStream,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, EndCreateObjectFromByteStream)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndCreateObjectFromByteStream )( 
            IMFSourceResolver * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFSourceResolver, CancelObjectCreation)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CancelObjectCreation )( 
            IMFSourceResolver * This,
            /* [in] */ IUnknown *pIUnknownCancelCookie);
        
        END_INTERFACE
    } IMFSourceResolverVtbl;

    interface IMFSourceResolver
    {
        CONST_VTBL struct IMFSourceResolverVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceResolver_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceResolver_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceResolver_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceResolver_CreateObjectFromURL(This,pwszURL,dwFlags,pProps,pObjectType,ppObject)	\
    ( (This)->lpVtbl -> CreateObjectFromURL(This,pwszURL,dwFlags,pProps,pObjectType,ppObject) ) 

#define IMFSourceResolver_CreateObjectFromByteStream(This,pByteStream,pwszURL,dwFlags,pProps,pObjectType,ppObject)	\
    ( (This)->lpVtbl -> CreateObjectFromByteStream(This,pByteStream,pwszURL,dwFlags,pProps,pObjectType,ppObject) ) 

#define IMFSourceResolver_BeginCreateObjectFromURL(This,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginCreateObjectFromURL(This,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState) ) 

#define IMFSourceResolver_EndCreateObjectFromURL(This,pResult,pObjectType,ppObject)	\
    ( (This)->lpVtbl -> EndCreateObjectFromURL(This,pResult,pObjectType,ppObject) ) 

#define IMFSourceResolver_BeginCreateObjectFromByteStream(This,pByteStream,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginCreateObjectFromByteStream(This,pByteStream,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState) ) 

#define IMFSourceResolver_EndCreateObjectFromByteStream(This,pResult,pObjectType,ppObject)	\
    ( (This)->lpVtbl -> EndCreateObjectFromByteStream(This,pResult,pObjectType,ppObject) ) 

#define IMFSourceResolver_CancelObjectCreation(This,pIUnknownCancelCookie)	\
    ( (This)->lpVtbl -> CancelObjectCreation(This,pIUnknownCancelCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_RemoteBeginCreateObjectFromURL_Proxy( 
    __RPC__in IMFSourceResolver * This,
    /* [string][in] */ __RPC__in_string LPCWSTR pwszURL,
    /* [in] */ DWORD dwFlags,
    /* [in] */ __RPC__in_opt IPropertyStore *pProps,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFSourceResolver_RemoteBeginCreateObjectFromURL_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_RemoteEndCreateObjectFromURL_Proxy( 
    __RPC__in IMFSourceResolver * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out MF_OBJECT_TYPE *pObjectType,
    /* [out] */ __RPC__deref_out_opt IUnknown **ppObject);


void __RPC_STUB IMFSourceResolver_RemoteEndCreateObjectFromURL_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_RemoteBeginCreateObjectFromByteStream_Proxy( 
    __RPC__in IMFSourceResolver * This,
    /* [in] */ __RPC__in_opt IMFByteStream *pByteStream,
    /* [unique][in] */ __RPC__in_opt LPCWSTR pwszURL,
    /* [in] */ DWORD dwFlags,
    /* [unique][in] */ __RPC__in_opt IPropertyStore *pProps,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFSourceResolver_RemoteBeginCreateObjectFromByteStream_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_RemoteEndCreateObjectFromByteStream_Proxy( 
    __RPC__in IMFSourceResolver * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out MF_OBJECT_TYPE *pObjectType,
    /* [out] */ __RPC__deref_out_opt IUnknown **ppObject);


void __RPC_STUB IMFSourceResolver_RemoteEndCreateObjectFromByteStream_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFSourceResolver_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0002 */
/* [local] */ 

STDAPI MFCreateSourceResolver( 
        /* out */     _Outptr_ IMFSourceResolver     **ppISourceResolver);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
STDAPI CreatePropertyStore( 
        /* out */     _Outptr_ IPropertyStore        **ppStore);
STDAPI MFGetSupportedSchemes(                      
       _Out_ PROPVARIANT* pPropVarSchemeArray  );  
STDAPI MFGetSupportedMimeTypes(                      
       _Out_ PROPVARIANT* pPropVarMimeTypeArray  );  
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_SourceOpenMonitor = { { 0x074d4637, 0xb5ae, 0x465d, 0xaf, 0x17, 0x1a, 0x53, 0x8d, 0x28, 0x59, 0xdd}, 0x02 }; 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMediaSource_ApproxSeek = { { 0xb4cd270f, 0x244d, 0x4969, 0xbb, 0x92, 0x3f, 0x0f, 0xb8, 0x31, 0x6f, 0x10}, 0x01 }; 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMediaSource_IterativeSeekIfNoIndex = { { 0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c }, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMediaSource_IterativeSeek_Max_Count = { { 0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c }, 0x02 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_ASFMediaSource_IterativeSeek_Tolerance_In_MilliSecond = { { 0x170b65dc, 0x4a4e, 0x407a, 0xac, 0x22, 0x57, 0x7f, 0x50, 0xe4, 0xa3, 0x7c }, 0x03 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_Content_DLNA_Profile_ID = { { 0xcfa31b45, 0x525d, 0x4998, 0xbb, 0x44, 0x3f, 0x7d, 0x81, 0x54, 0x2f, 0xa4 }, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MediaSource_DisableReadAhead = { { 0x26366c14, 0xc5bf, 0x4c76, 0x88, 0x7b, 0x9f, 0x17, 0x54, 0xdb, 0x5f, 0x9}, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_SBESourceMode = { { 0x3fae10bb, 0xf859, 0x4192, 0xb5, 0x62, 0x18, 0x68, 0xd3, 0xda, 0x3a, 0x02}, 0x01 }; 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_PMP_Creation_Callback = { { 0x28bb4de2, 0x26a2, 0x4870, 0xb7, 0x20, 0xd2, 0x6b, 0xbe, 0xb1, 0x49, 0x42}, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Enable_Urlmon = { { 0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92}, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Urlmon_Bind_Flags = { { 0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92}, 0x02 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Urlmon_Security_Id = { { 0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92}, 0x03 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Urlmon_Window = { { 0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92}, 0x04 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Urlmon_Callback_QueryService = { { 0xeda8afdf, 0xc171, 0x417f, 0x8d, 0x17, 0x2e, 0x09, 0x18, 0x30, 0x32, 0x92}, 0x05 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MediaProtectionSystemId =  { { 0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd}, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MediaProtectionSystemContext =  { { 0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd}, 0x02 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MediaProtectionSystemIdMapping =  { { 0x636b271d, 0xddc7, 0x49e9, 0xa6, 0xc6, 0x47, 0x38, 0x59, 0x62, 0xe5, 0xbd}, 0x03 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MediaProtectionContainerGuid =  { { 0x42af3d7c, 0xcf, 0x4a0f, 0x81, 0xf0, 0xad, 0xf5, 0x24, 0xa5, 0xa5, 0xb5}, 0x1 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_MediaProtectionSystemContextsPerTrack =  { { 0x4454b092, 0xd3da, 0x49b0, 0x84, 0x52, 0x68, 0x50, 0xc7, 0xdb, 0x76, 0x4d }, 0x03 }; 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Download_Mode = { { 0x817f11b7, 0xa982, 0x46ec, 0xa4, 0x49, 0xef, 0x58, 0xae, 0xd5, 0x3c, 0xa8 }, 0x01 }; 
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Caching_Mode = { { 0x86a2403e, 0xc78b, 0x44d7, 0x8b, 0xc8, 0xff, 0x72, 0x58, 0x11, 0x75, 0x08}, 0x01 }; 
EXTERN_C const DECLSPEC_SELECTANY PROPERTYKEY MFPKEY_HTTP_ByteStream_Cache_Limit = { { 0x86a2403e, 0xc78b, 0x44d7, 0x8b, 0xc8, 0xff, 0x72, 0x58, 0x11, 0x75, 0x08}, 0x02 }; 
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
typedef 
enum _MFMEDIASOURCE_CHARACTERISTICS
    {
        MFMEDIASOURCE_IS_LIVE	= 0x1,
        MFMEDIASOURCE_CAN_SEEK	= 0x2,
        MFMEDIASOURCE_CAN_PAUSE	= 0x4,
        MFMEDIASOURCE_HAS_SLOW_SEEK	= 0x8,
        MFMEDIASOURCE_HAS_MULTIPLE_PRESENTATIONS	= 0x10,
        MFMEDIASOURCE_CAN_SKIPFORWARD	= 0x20,
        MFMEDIASOURCE_CAN_SKIPBACKWARD	= 0x40,
        MFMEDIASOURCE_DOES_NOT_USE_NETWORK	= 0x80
    } 	MFMEDIASOURCE_CHARACTERISTICS;

#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MF_TIME_FORMAT_ENTRY_RELATIVE, 0x4399f178, 0x46d3, 0x4504, 0xaf, 0xda, 0x20, 0xd3, 0x2e, 0x9b, 0xa3, 0x60 );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 




extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0002_v0_0_s_ifspec;

#ifndef __IMFMediaSource_INTERFACE_DEFINED__
#define __IMFMediaSource_INTERFACE_DEFINED__

/* interface IMFMediaSource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("279a808d-aec7-40c8-9c6b-a6b492c78a66")
    IMFMediaSource : public IMFMediaEventGenerator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCharacteristics( 
            /* [out] */ __RPC__out DWORD *pdwCharacteristics) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreatePresentationDescriptor( 
            /* [annotation][out] */ 
            _Outptr_  IMFPresentationDescriptor **ppPresentationDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [unique][in] */ __RPC__in_opt const GUID *pguidTimeFormat,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvarStartPosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSource * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMFMediaSource * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaSource * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaSource * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            __RPC__in IMFMediaSource * This,
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, GetCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetCharacteristics )( 
            __RPC__in IMFMediaSource * This,
            /* [out] */ __RPC__out DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, CreatePresentationDescriptor)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreatePresentationDescriptor )( 
            IMFMediaSource * This,
            /* [annotation][out] */ 
            _Outptr_  IMFPresentationDescriptor **ppPresentationDescriptor);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IMFMediaSource * This,
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [unique][in] */ __RPC__in_opt const GUID *pguidTimeFormat,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvarStartPosition);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IMFMediaSource * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMFMediaSource * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IMFMediaSource * This);
        
        END_INTERFACE
    } IMFMediaSourceVtbl;

    interface IMFMediaSource
    {
        CONST_VTBL struct IMFMediaSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSource_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaSource_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaSource_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaSource_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFMediaSource_GetCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetCharacteristics(This,pdwCharacteristics) ) 

#define IMFMediaSource_CreatePresentationDescriptor(This,ppPresentationDescriptor)	\
    ( (This)->lpVtbl -> CreatePresentationDescriptor(This,ppPresentationDescriptor) ) 

#define IMFMediaSource_Start(This,pPresentationDescriptor,pguidTimeFormat,pvarStartPosition)	\
    ( (This)->lpVtbl -> Start(This,pPresentationDescriptor,pguidTimeFormat,pvarStartPosition) ) 

#define IMFMediaSource_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFMediaSource_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaSource_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaSource_RemoteCreatePresentationDescriptor_Proxy( 
    __RPC__in IMFMediaSource * This,
    /* [out] */ __RPC__out DWORD *pcbPD,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPD) BYTE **pbPD,
    /* [out] */ __RPC__deref_out_opt IMFPresentationDescriptor **ppRemotePD);


void __RPC_STUB IMFMediaSource_RemoteCreatePresentationDescriptor_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFMediaSource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0003 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0003_v0_0_s_ifspec;

#ifndef __IMFMediaSourceEx_INTERFACE_DEFINED__
#define __IMFMediaSourceEx_INTERFACE_DEFINED__

/* interface IMFMediaSourceEx */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSourceEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3C9B2EB9-86D5-4514-A394-F56664F9F0D8")
    IMFMediaSourceEx : public IMFMediaSource
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSourceAttributes( 
            /* [out] */ __RPC__deref_out_opt IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamAttributes( 
            /* [in] */ DWORD dwStreamIdentifier,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetD3DManager( 
            /* [in] */ __RPC__in_opt IUnknown *pManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourceExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSourceEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSourceEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaSourceEx * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaSourceEx * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, GetCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetCharacteristics )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [out] */ __RPC__out DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, CreatePresentationDescriptor)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreatePresentationDescriptor )( 
            IMFMediaSourceEx * This,
            /* [annotation][out] */ 
            _Outptr_  IMFPresentationDescriptor **ppPresentationDescriptor);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [unique][in] */ __RPC__in_opt const GUID *pguidTimeFormat,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvarStartPosition);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IMFMediaSourceEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMFMediaSourceEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IMFMediaSourceEx * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceEx, GetSourceAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetSourceAttributes )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceEx, GetStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetStreamAttributes )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [in] */ DWORD dwStreamIdentifier,
            /* [out] */ __RPC__deref_out_opt IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceEx, SetD3DManager)
        HRESULT ( STDMETHODCALLTYPE *SetD3DManager )( 
            __RPC__in IMFMediaSourceEx * This,
            /* [in] */ __RPC__in_opt IUnknown *pManager);
        
        END_INTERFACE
    } IMFMediaSourceExVtbl;

    interface IMFMediaSourceEx
    {
        CONST_VTBL struct IMFMediaSourceExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSourceEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSourceEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSourceEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSourceEx_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaSourceEx_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaSourceEx_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaSourceEx_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFMediaSourceEx_GetCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetCharacteristics(This,pdwCharacteristics) ) 

#define IMFMediaSourceEx_CreatePresentationDescriptor(This,ppPresentationDescriptor)	\
    ( (This)->lpVtbl -> CreatePresentationDescriptor(This,ppPresentationDescriptor) ) 

#define IMFMediaSourceEx_Start(This,pPresentationDescriptor,pguidTimeFormat,pvarStartPosition)	\
    ( (This)->lpVtbl -> Start(This,pPresentationDescriptor,pguidTimeFormat,pvarStartPosition) ) 

#define IMFMediaSourceEx_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFMediaSourceEx_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaSourceEx_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 


#define IMFMediaSourceEx_GetSourceAttributes(This,ppAttributes)	\
    ( (This)->lpVtbl -> GetSourceAttributes(This,ppAttributes) ) 

#define IMFMediaSourceEx_GetStreamAttributes(This,dwStreamIdentifier,ppAttributes)	\
    ( (This)->lpVtbl -> GetStreamAttributes(This,dwStreamIdentifier,ppAttributes) ) 

#define IMFMediaSourceEx_SetD3DManager(This,pManager)	\
    ( (This)->lpVtbl -> SetD3DManager(This,pManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSourceEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0004 */
/* [local] */ 

EXTERN_GUID( MF_SOURCE_STREAM_SUPPORTS_HW_CONNECTION, 0xa38253aa, 0x6314, 0x42fd, 0xa3, 0xce, 0xbb, 0x27, 0xb6, 0x85, 0x99, 0x46);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 

#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0004_v0_0_s_ifspec;

#ifndef __IMFClockConsumer_INTERFACE_DEFINED__
#define __IMFClockConsumer_INTERFACE_DEFINED__

/* interface IMFClockConsumer */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFClockConsumer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6ef2a662-47c0-4666-b13d-cbb717f2fa2c")
    IMFClockConsumer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPresentationClock( 
            /* [in] */ __RPC__in_opt IMFPresentationClock *pPresentationClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentationClock( 
            /* [out] */ __RPC__deref_out_opt IMFPresentationClock **ppPresentationClock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFClockConsumerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFClockConsumer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFClockConsumer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFClockConsumer * This);
        
        DECLSPEC_XFGVIRT(IMFClockConsumer, SetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *SetPresentationClock )( 
            __RPC__in IMFClockConsumer * This,
            /* [in] */ __RPC__in_opt IMFPresentationClock *pPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFClockConsumer, GetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationClock )( 
            __RPC__in IMFClockConsumer * This,
            /* [out] */ __RPC__deref_out_opt IMFPresentationClock **ppPresentationClock);
        
        END_INTERFACE
    } IMFClockConsumerVtbl;

    interface IMFClockConsumer
    {
        CONST_VTBL struct IMFClockConsumerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFClockConsumer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFClockConsumer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFClockConsumer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFClockConsumer_SetPresentationClock(This,pPresentationClock)	\
    ( (This)->lpVtbl -> SetPresentationClock(This,pPresentationClock) ) 

#define IMFClockConsumer_GetPresentationClock(This,ppPresentationClock)	\
    ( (This)->lpVtbl -> GetPresentationClock(This,ppPresentationClock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFClockConsumer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0005 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0005_v0_0_s_ifspec;

#ifndef __IMFMediaStream_INTERFACE_DEFINED__
#define __IMFMediaStream_INTERFACE_DEFINED__

/* interface IMFMediaStream */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D182108F-4EC6-443f-AA42-A71106EC825F")
    IMFMediaStream : public IMFMediaEventGenerator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMediaSource( 
            /* [out] */ __RPC__deref_out_opt IMFMediaSource **ppMediaSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamDescriptor( 
            /* [out] */ __RPC__deref_out_opt IMFStreamDescriptor **ppStreamDescriptor) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RequestSample( 
            /* [in] */ IUnknown *pToken) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaStream * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaStream * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMFMediaStream * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaStream * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaStream * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            __RPC__in IMFMediaStream * This,
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaStream, GetMediaSource)
        HRESULT ( STDMETHODCALLTYPE *GetMediaSource )( 
            __RPC__in IMFMediaStream * This,
            /* [out] */ __RPC__deref_out_opt IMFMediaSource **ppMediaSource);
        
        DECLSPEC_XFGVIRT(IMFMediaStream, GetStreamDescriptor)
        HRESULT ( STDMETHODCALLTYPE *GetStreamDescriptor )( 
            __RPC__in IMFMediaStream * This,
            /* [out] */ __RPC__deref_out_opt IMFStreamDescriptor **ppStreamDescriptor);
        
        DECLSPEC_XFGVIRT(IMFMediaStream, RequestSample)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RequestSample )( 
            IMFMediaStream * This,
            /* [in] */ IUnknown *pToken);
        
        END_INTERFACE
    } IMFMediaStreamVtbl;

    interface IMFMediaStream
    {
        CONST_VTBL struct IMFMediaStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaStream_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaStream_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaStream_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaStream_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFMediaStream_GetMediaSource(This,ppMediaSource)	\
    ( (This)->lpVtbl -> GetMediaSource(This,ppMediaSource) ) 

#define IMFMediaStream_GetStreamDescriptor(This,ppStreamDescriptor)	\
    ( (This)->lpVtbl -> GetStreamDescriptor(This,ppStreamDescriptor) ) 

#define IMFMediaStream_RequestSample(This,pToken)	\
    ( (This)->lpVtbl -> RequestSample(This,pToken) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaStream_RemoteRequestSample_Proxy( 
    __RPC__in IMFMediaStream * This);


void __RPC_STUB IMFMediaStream_RemoteRequestSample_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFMediaStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0006 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MF_STREAM_SINK_SUPPORTS_HW_CONNECTION, 0x9b465cbf, 0x597, 0x4f9e, 0x9f, 0x3c, 0xb9, 0x7e, 0xee, 0xf9, 0x3, 0x59);
EXTERN_GUID( MF_STREAM_SINK_SUPPORTS_ROTATION, 0xb3e96280, 0xbd05, 0x41a5, 0x97, 0xad, 0x8a, 0x7f, 0xee, 0x24, 0xb9, 0x12);
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#define MEDIASINK_FIXED_STREAMS         0x00000001
#define MEDIASINK_CANNOT_MATCH_CLOCK    0x00000002
#define MEDIASINK_RATELESS              0x00000004
#define MEDIASINK_CLOCK_REQUIRED        0x00000008
#define MEDIASINK_CAN_PREROLL           0x00000010
#define MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE 0x00000020
typedef 
enum MF_TRANSFER_VIDEO_FRAME_FLAGS
    {
        MF_TRANSFER_VIDEO_FRAME_DEFAULT	= 0,
        MF_TRANSFER_VIDEO_FRAME_STRETCH	= 1,
        MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR	= 2
    } 	MF_TRANSFER_VIDEO_FRAME_FLAGS;

EXTERN_GUID( MF_SINK_VIDEO_PTS,  0x2162bde7, 0x421e, 0x4b90, 0x9b, 0x33, 0xe5, 0x8f, 0xbf, 0x1d, 0x58, 0xb6);
EXTERN_GUID( MF_SINK_VIDEO_NATIVE_WIDTH,  0xe6d6a707, 0x1505, 0x4747, 0x9b, 0x10, 0x72, 0xd2, 0xd1, 0x58, 0xcb, 0x3a);
EXTERN_GUID( MF_SINK_VIDEO_NATIVE_HEIGHT,  0xf0ca6705, 0x490c, 0x43e8, 0x94, 0x1c, 0xc0, 0xb3, 0x20, 0x6b, 0x9a, 0x65);
EXTERN_GUID( MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_NUMERATOR,  0xd0f33b22, 0xb78a, 0x4879, 0xb4, 0x55, 0xf0, 0x3e, 0xf3, 0xfa, 0x82, 0xcd);
EXTERN_GUID( MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_DENOMINATOR,  0x6ea1eb97, 0x1fe0, 0x4f10, 0xa6, 0xe4, 0x1f, 0x4f, 0x66, 0x15, 0x64, 0xe0);
EXTERN_GUID( MF_BD_MVC_PLANE_OFFSET_METADATA,  0x62a654e4, 0xb76c, 0x4901, 0x98, 0x23, 0x2c, 0xb6, 0x15, 0xd4, 0x73, 0x18);
EXTERN_GUID( MF_LUMA_KEY_ENABLE, 0x7369820f, 0x76de, 0x43ca, 0x92, 0x84, 0x47, 0xb8, 0xf3, 0x7e, 0x06, 0x49);
EXTERN_GUID( MF_LUMA_KEY_LOWER,  0x93d7b8d5, 0x0b81, 0x4715, 0xae, 0xa0, 0x87, 0x25, 0x87, 0x16, 0x21, 0xe9);
EXTERN_GUID( MF_LUMA_KEY_UPPER,  0xd09f39bb, 0x4602, 0x4c31, 0xa7, 0x06, 0xa1, 0x21, 0x71, 0xa5, 0x11, 0x0a);
EXTERN_GUID( MF_USER_EXTENDED_ATTRIBUTES,  0xc02abac6, 0xfeb2, 0x4541, 0x92, 0x2f, 0x92, 0x0b, 0x43, 0x70, 0x27, 0x22);
EXTERN_GUID( MF_INDEPENDENT_STILL_IMAGE, 0xea12af41, 0x0710, 0x42c9, 0xa1, 0x27, 0xda, 0xa3, 0xe7, 0x84, 0x83, 0xa5);



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0006_v0_0_s_ifspec;

#ifndef __IMFMediaSink_INTERFACE_DEFINED__
#define __IMFMediaSink_INTERFACE_DEFINED__

/* interface IMFMediaSink */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6ef2a660-47c0-4666-b13d-cbb717f2fa2c")
    IMFMediaSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCharacteristics( 
            /* [out] */ __RPC__out DWORD *pdwCharacteristics) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddStreamSink( 
            /* [in] */ DWORD dwStreamSinkIdentifier,
            /* [in] */ __RPC__in_opt IMFMediaType *pMediaType,
            /* [out] */ __RPC__deref_out_opt IMFStreamSink **ppStreamSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveStreamSink( 
            /* [in] */ DWORD dwStreamSinkIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamSinkCount( 
            /* [out] */ __RPC__out DWORD *pcStreamSinkCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamSinkByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IMFStreamSink **ppStreamSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamSinkById( 
            /* [in] */ DWORD dwStreamSinkIdentifier,
            /* [out] */ __RPC__deref_out_opt IMFStreamSink **ppStreamSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPresentationClock( 
            /* [in] */ __RPC__in_opt IMFPresentationClock *pPresentationClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentationClock( 
            /* [out] */ __RPC__deref_out_opt IMFPresentationClock **ppPresentationClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSink * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetCharacteristics )( 
            __RPC__in IMFMediaSink * This,
            /* [out] */ __RPC__out DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, AddStreamSink)
        HRESULT ( STDMETHODCALLTYPE *AddStreamSink )( 
            __RPC__in IMFMediaSink * This,
            /* [in] */ DWORD dwStreamSinkIdentifier,
            /* [in] */ __RPC__in_opt IMFMediaType *pMediaType,
            /* [out] */ __RPC__deref_out_opt IMFStreamSink **ppStreamSink);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, RemoveStreamSink)
        HRESULT ( STDMETHODCALLTYPE *RemoveStreamSink )( 
            __RPC__in IMFMediaSink * This,
            /* [in] */ DWORD dwStreamSinkIdentifier);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetStreamSinkCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSinkCount )( 
            __RPC__in IMFMediaSink * This,
            /* [out] */ __RPC__out DWORD *pcStreamSinkCount);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetStreamSinkByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSinkByIndex )( 
            __RPC__in IMFMediaSink * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__deref_out_opt IMFStreamSink **ppStreamSink);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetStreamSinkById)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSinkById )( 
            __RPC__in IMFMediaSink * This,
            /* [in] */ DWORD dwStreamSinkIdentifier,
            /* [out] */ __RPC__deref_out_opt IMFStreamSink **ppStreamSink);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, SetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *SetPresentationClock )( 
            __RPC__in IMFMediaSink * This,
            /* [in] */ __RPC__in_opt IMFPresentationClock *pPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationClock )( 
            __RPC__in IMFMediaSink * This,
            /* [out] */ __RPC__deref_out_opt IMFPresentationClock **ppPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IMFMediaSink * This);
        
        END_INTERFACE
    } IMFMediaSinkVtbl;

    interface IMFMediaSink
    {
        CONST_VTBL struct IMFMediaSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSink_GetCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetCharacteristics(This,pdwCharacteristics) ) 

#define IMFMediaSink_AddStreamSink(This,dwStreamSinkIdentifier,pMediaType,ppStreamSink)	\
    ( (This)->lpVtbl -> AddStreamSink(This,dwStreamSinkIdentifier,pMediaType,ppStreamSink) ) 

#define IMFMediaSink_RemoveStreamSink(This,dwStreamSinkIdentifier)	\
    ( (This)->lpVtbl -> RemoveStreamSink(This,dwStreamSinkIdentifier) ) 

#define IMFMediaSink_GetStreamSinkCount(This,pcStreamSinkCount)	\
    ( (This)->lpVtbl -> GetStreamSinkCount(This,pcStreamSinkCount) ) 

#define IMFMediaSink_GetStreamSinkByIndex(This,dwIndex,ppStreamSink)	\
    ( (This)->lpVtbl -> GetStreamSinkByIndex(This,dwIndex,ppStreamSink) ) 

#define IMFMediaSink_GetStreamSinkById(This,dwStreamSinkIdentifier,ppStreamSink)	\
    ( (This)->lpVtbl -> GetStreamSinkById(This,dwStreamSinkIdentifier,ppStreamSink) ) 

#define IMFMediaSink_SetPresentationClock(This,pPresentationClock)	\
    ( (This)->lpVtbl -> SetPresentationClock(This,pPresentationClock) ) 

#define IMFMediaSink_GetPresentationClock(This,ppPresentationClock)	\
    ( (This)->lpVtbl -> GetPresentationClock(This,ppPresentationClock) ) 

#define IMFMediaSink_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0007 */
/* [local] */ 

typedef 
enum _MFSTREAMSINK_MARKER_TYPE
    {
        MFSTREAMSINK_MARKER_DEFAULT	= 0,
        MFSTREAMSINK_MARKER_ENDOFSEGMENT	= ( MFSTREAMSINK_MARKER_DEFAULT + 1 ) ,
        MFSTREAMSINK_MARKER_TICK	= ( MFSTREAMSINK_MARKER_ENDOFSEGMENT + 1 ) ,
        MFSTREAMSINK_MARKER_EVENT	= ( MFSTREAMSINK_MARKER_TICK + 1 ) 
    } 	MFSTREAMSINK_MARKER_TYPE;




extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0007_v0_0_s_ifspec;

#ifndef __IMFStreamSink_INTERFACE_DEFINED__
#define __IMFStreamSink_INTERFACE_DEFINED__

/* interface IMFStreamSink */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFStreamSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0A97B3CF-8E7C-4a3d-8F8C-0C843DC247FB")
    IMFStreamSink : public IMFMediaEventGenerator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMediaSink( 
            /* [out] */ __RPC__deref_out_opt IMFMediaSink **ppMediaSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIdentifier( 
            /* [out] */ __RPC__out DWORD *pdwIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaTypeHandler( 
            /* [out] */ __RPC__deref_out_opt IMFMediaTypeHandler **ppHandler) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProcessSample( 
            /* [in] */ __RPC__in_opt IMFSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlaceMarker( 
            /* [in] */ MFSTREAMSINK_MARKER_TYPE eMarkerType,
            /* [in] */ __RPC__in const PROPVARIANT *pvarMarkerValue,
            /* [in] */ __RPC__in const PROPVARIANT *pvarContextValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Flush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFStreamSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFStreamSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFStreamSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFStreamSink * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            __RPC__in IMFStreamSink * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFStreamSink * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFStreamSink * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            __RPC__in IMFStreamSink * This,
            /* [in] */ MediaEventType met,
            /* [in] */ __RPC__in REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ __RPC__in_opt const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFStreamSink, GetMediaSink)
        HRESULT ( STDMETHODCALLTYPE *GetMediaSink )( 
            __RPC__in IMFStreamSink * This,
            /* [out] */ __RPC__deref_out_opt IMFMediaSink **ppMediaSink);
        
        DECLSPEC_XFGVIRT(IMFStreamSink, GetIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetIdentifier )( 
            __RPC__in IMFStreamSink * This,
            /* [out] */ __RPC__out DWORD *pdwIdentifier);
        
        DECLSPEC_XFGVIRT(IMFStreamSink, GetMediaTypeHandler)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTypeHandler )( 
            __RPC__in IMFStreamSink * This,
            /* [out] */ __RPC__deref_out_opt IMFMediaTypeHandler **ppHandler);
        
        DECLSPEC_XFGVIRT(IMFStreamSink, ProcessSample)
        HRESULT ( STDMETHODCALLTYPE *ProcessSample )( 
            __RPC__in IMFStreamSink * This,
            /* [in] */ __RPC__in_opt IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFStreamSink, PlaceMarker)
        HRESULT ( STDMETHODCALLTYPE *PlaceMarker )( 
            __RPC__in IMFStreamSink * This,
            /* [in] */ MFSTREAMSINK_MARKER_TYPE eMarkerType,
            /* [in] */ __RPC__in const PROPVARIANT *pvarMarkerValue,
            /* [in] */ __RPC__in const PROPVARIANT *pvarContextValue);
        
        DECLSPEC_XFGVIRT(IMFStreamSink, Flush)
        HRESULT ( STDMETHODCALLTYPE *Flush )( 
            __RPC__in IMFStreamSink * This);
        
        END_INTERFACE
    } IMFStreamSinkVtbl;

    interface IMFStreamSink
    {
        CONST_VTBL struct IMFStreamSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFStreamSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFStreamSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFStreamSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFStreamSink_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFStreamSink_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFStreamSink_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFStreamSink_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFStreamSink_GetMediaSink(This,ppMediaSink)	\
    ( (This)->lpVtbl -> GetMediaSink(This,ppMediaSink) ) 

#define IMFStreamSink_GetIdentifier(This,pdwIdentifier)	\
    ( (This)->lpVtbl -> GetIdentifier(This,pdwIdentifier) ) 

#define IMFStreamSink_GetMediaTypeHandler(This,ppHandler)	\
    ( (This)->lpVtbl -> GetMediaTypeHandler(This,ppHandler) ) 

#define IMFStreamSink_ProcessSample(This,pSample)	\
    ( (This)->lpVtbl -> ProcessSample(This,pSample) ) 

#define IMFStreamSink_PlaceMarker(This,eMarkerType,pvarMarkerValue,pvarContextValue)	\
    ( (This)->lpVtbl -> PlaceMarker(This,eMarkerType,pvarMarkerValue,pvarContextValue) ) 

#define IMFStreamSink_Flush(This)	\
    ( (This)->lpVtbl -> Flush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFStreamSink_INTERFACE_DEFINED__ */


#ifndef __IMFVideoSampleAllocator_INTERFACE_DEFINED__
#define __IMFVideoSampleAllocator_INTERFACE_DEFINED__

/* interface IMFVideoSampleAllocator */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoSampleAllocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86cbc910-e533-4751-8e3b-f19b5b806a03")
    IMFVideoSampleAllocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDirectXManager( 
            /* [unique][in] */ IUnknown *pManager) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UninitializeSampleAllocator( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeSampleAllocator( 
            /* [in] */ DWORD cRequestedFrames,
            /* [in] */ IMFMediaType *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllocateSample( 
            /* [out] */ IMFSample **ppSample) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoSampleAllocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoSampleAllocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, SetDirectXManager)
        HRESULT ( STDMETHODCALLTYPE *SetDirectXManager )( 
            IMFVideoSampleAllocator * This,
            /* [unique][in] */ IUnknown *pManager);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, UninitializeSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *UninitializeSampleAllocator )( 
            IMFVideoSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, InitializeSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *InitializeSampleAllocator )( 
            IMFVideoSampleAllocator * This,
            /* [in] */ DWORD cRequestedFrames,
            /* [in] */ IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            IMFVideoSampleAllocator * This,
            /* [out] */ IMFSample **ppSample);
        
        END_INTERFACE
    } IMFVideoSampleAllocatorVtbl;

    interface IMFVideoSampleAllocator
    {
        CONST_VTBL struct IMFVideoSampleAllocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoSampleAllocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoSampleAllocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoSampleAllocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoSampleAllocator_SetDirectXManager(This,pManager)	\
    ( (This)->lpVtbl -> SetDirectXManager(This,pManager) ) 

#define IMFVideoSampleAllocator_UninitializeSampleAllocator(This)	\
    ( (This)->lpVtbl -> UninitializeSampleAllocator(This) ) 

#define IMFVideoSampleAllocator_InitializeSampleAllocator(This,cRequestedFrames,pMediaType)	\
    ( (This)->lpVtbl -> InitializeSampleAllocator(This,cRequestedFrames,pMediaType) ) 

#define IMFVideoSampleAllocator_AllocateSample(This,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,ppSample) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoSampleAllocator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0009 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0009_v0_0_s_ifspec;

#ifndef __IMFVideoSampleAllocatorNotify_INTERFACE_DEFINED__
#define __IMFVideoSampleAllocatorNotify_INTERFACE_DEFINED__

/* interface IMFVideoSampleAllocatorNotify */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoSampleAllocatorNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A792CDBE-C374-4e89-8335-278E7B9956A4")
    IMFVideoSampleAllocatorNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyRelease( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoSampleAllocatorNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoSampleAllocatorNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoSampleAllocatorNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoSampleAllocatorNotify * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocatorNotify, NotifyRelease)
        HRESULT ( STDMETHODCALLTYPE *NotifyRelease )( 
            IMFVideoSampleAllocatorNotify * This);
        
        END_INTERFACE
    } IMFVideoSampleAllocatorNotifyVtbl;

    interface IMFVideoSampleAllocatorNotify
    {
        CONST_VTBL struct IMFVideoSampleAllocatorNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoSampleAllocatorNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoSampleAllocatorNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoSampleAllocatorNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoSampleAllocatorNotify_NotifyRelease(This)	\
    ( (This)->lpVtbl -> NotifyRelease(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoSampleAllocatorNotify_INTERFACE_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorNotifyEx_INTERFACE_DEFINED__
#define __IMFVideoSampleAllocatorNotifyEx_INTERFACE_DEFINED__

/* interface IMFVideoSampleAllocatorNotifyEx */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoSampleAllocatorNotifyEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3978AA1A-6D5B-4B7F-A340-90899189AE34")
    IMFVideoSampleAllocatorNotifyEx : public IMFVideoSampleAllocatorNotify
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyPrune( 
            IMFSample *__MIDL__IMFVideoSampleAllocatorNotifyEx0000) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoSampleAllocatorNotifyExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoSampleAllocatorNotifyEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoSampleAllocatorNotifyEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoSampleAllocatorNotifyEx * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocatorNotify, NotifyRelease)
        HRESULT ( STDMETHODCALLTYPE *NotifyRelease )( 
            IMFVideoSampleAllocatorNotifyEx * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocatorNotifyEx, NotifyPrune)
        HRESULT ( STDMETHODCALLTYPE *NotifyPrune )( 
            IMFVideoSampleAllocatorNotifyEx * This,
            IMFSample *__MIDL__IMFVideoSampleAllocatorNotifyEx0000);
        
        END_INTERFACE
    } IMFVideoSampleAllocatorNotifyExVtbl;

    interface IMFVideoSampleAllocatorNotifyEx
    {
        CONST_VTBL struct IMFVideoSampleAllocatorNotifyExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoSampleAllocatorNotifyEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoSampleAllocatorNotifyEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoSampleAllocatorNotifyEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoSampleAllocatorNotifyEx_NotifyRelease(This)	\
    ( (This)->lpVtbl -> NotifyRelease(This) ) 


#define IMFVideoSampleAllocatorNotifyEx_NotifyPrune(This,__MIDL__IMFVideoSampleAllocatorNotifyEx0000)	\
    ( (This)->lpVtbl -> NotifyPrune(This,__MIDL__IMFVideoSampleAllocatorNotifyEx0000) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoSampleAllocatorNotifyEx_INTERFACE_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorCallback_INTERFACE_DEFINED__
#define __IMFVideoSampleAllocatorCallback_INTERFACE_DEFINED__

/* interface IMFVideoSampleAllocatorCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFVideoSampleAllocatorCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("992388B4-3372-4f67-8B6F-C84C071F4751")
    IMFVideoSampleAllocatorCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetCallback( 
            /* [unique][in] */ IMFVideoSampleAllocatorNotify *pNotify) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFreeSampleCount( 
            /* [out] */ LONG *plSamples) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoSampleAllocatorCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoSampleAllocatorCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoSampleAllocatorCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoSampleAllocatorCallback * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocatorCallback, SetCallback)
        HRESULT ( STDMETHODCALLTYPE *SetCallback )( 
            IMFVideoSampleAllocatorCallback * This,
            /* [unique][in] */ IMFVideoSampleAllocatorNotify *pNotify);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocatorCallback, GetFreeSampleCount)
        HRESULT ( STDMETHODCALLTYPE *GetFreeSampleCount )( 
            IMFVideoSampleAllocatorCallback * This,
            /* [out] */ LONG *plSamples);
        
        END_INTERFACE
    } IMFVideoSampleAllocatorCallbackVtbl;

    interface IMFVideoSampleAllocatorCallback
    {
        CONST_VTBL struct IMFVideoSampleAllocatorCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoSampleAllocatorCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoSampleAllocatorCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoSampleAllocatorCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoSampleAllocatorCallback_SetCallback(This,pNotify)	\
    ( (This)->lpVtbl -> SetCallback(This,pNotify) ) 

#define IMFVideoSampleAllocatorCallback_GetFreeSampleCount(This,plSamples)	\
    ( (This)->lpVtbl -> GetFreeSampleCount(This,plSamples) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoSampleAllocatorCallback_INTERFACE_DEFINED__ */


#ifndef __IMFVideoSampleAllocatorEx_INTERFACE_DEFINED__
#define __IMFVideoSampleAllocatorEx_INTERFACE_DEFINED__

/* interface IMFVideoSampleAllocatorEx */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFVideoSampleAllocatorEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("545b3a48-3283-4f62-866f-a62d8f598f9f")
    IMFVideoSampleAllocatorEx : public IMFVideoSampleAllocator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeSampleAllocatorEx( 
            /* [annotation] */ 
            _In_  DWORD cInitialSamples,
            /* [annotation] */ 
            _In_  DWORD cMaximumSamples,
            /* [annotation] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation] */ 
            _In_  IMFMediaType *pMediaType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoSampleAllocatorExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoSampleAllocatorEx * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoSampleAllocatorEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoSampleAllocatorEx * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, SetDirectXManager)
        HRESULT ( STDMETHODCALLTYPE *SetDirectXManager )( 
            IMFVideoSampleAllocatorEx * This,
            /* [unique][in] */ IUnknown *pManager);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, UninitializeSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *UninitializeSampleAllocator )( 
            IMFVideoSampleAllocatorEx * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, InitializeSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *InitializeSampleAllocator )( 
            IMFVideoSampleAllocatorEx * This,
            /* [in] */ DWORD cRequestedFrames,
            /* [in] */ IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            IMFVideoSampleAllocatorEx * This,
            /* [out] */ IMFSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocatorEx, InitializeSampleAllocatorEx)
        HRESULT ( STDMETHODCALLTYPE *InitializeSampleAllocatorEx )( 
            IMFVideoSampleAllocatorEx * This,
            /* [annotation] */ 
            _In_  DWORD cInitialSamples,
            /* [annotation] */ 
            _In_  DWORD cMaximumSamples,
            /* [annotation] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation] */ 
            _In_  IMFMediaType *pMediaType);
        
        END_INTERFACE
    } IMFVideoSampleAllocatorExVtbl;

    interface IMFVideoSampleAllocatorEx
    {
        CONST_VTBL struct IMFVideoSampleAllocatorExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoSampleAllocatorEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoSampleAllocatorEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoSampleAllocatorEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoSampleAllocatorEx_SetDirectXManager(This,pManager)	\
    ( (This)->lpVtbl -> SetDirectXManager(This,pManager) ) 

#define IMFVideoSampleAllocatorEx_UninitializeSampleAllocator(This)	\
    ( (This)->lpVtbl -> UninitializeSampleAllocator(This) ) 

#define IMFVideoSampleAllocatorEx_InitializeSampleAllocator(This,cRequestedFrames,pMediaType)	\
    ( (This)->lpVtbl -> InitializeSampleAllocator(This,cRequestedFrames,pMediaType) ) 

#define IMFVideoSampleAllocatorEx_AllocateSample(This,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,ppSample) ) 


#define IMFVideoSampleAllocatorEx_InitializeSampleAllocatorEx(This,cInitialSamples,cMaximumSamples,pAttributes,pMediaType)	\
    ( (This)->lpVtbl -> InitializeSampleAllocatorEx(This,cInitialSamples,cMaximumSamples,pAttributes,pMediaType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoSampleAllocatorEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0013 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WINBLUE) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0013_v0_0_s_ifspec;

#ifndef __IMFDXGIDeviceManagerSource_INTERFACE_DEFINED__
#define __IMFDXGIDeviceManagerSource_INTERFACE_DEFINED__

/* interface IMFDXGIDeviceManagerSource */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFDXGIDeviceManagerSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("20bc074b-7a8d-4609-8c3b-64a0a3b5d7ce")
    IMFDXGIDeviceManagerSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetManager( 
            /* [out] */ __RPC__deref_out_opt IMFDXGIDeviceManager **ppManager) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFDXGIDeviceManagerSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFDXGIDeviceManagerSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFDXGIDeviceManagerSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFDXGIDeviceManagerSource * This);
        
        DECLSPEC_XFGVIRT(IMFDXGIDeviceManagerSource, GetManager)
        HRESULT ( STDMETHODCALLTYPE *GetManager )( 
            __RPC__in IMFDXGIDeviceManagerSource * This,
            /* [out] */ __RPC__deref_out_opt IMFDXGIDeviceManager **ppManager);
        
        END_INTERFACE
    } IMFDXGIDeviceManagerSourceVtbl;

    interface IMFDXGIDeviceManagerSource
    {
        CONST_VTBL struct IMFDXGIDeviceManagerSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFDXGIDeviceManagerSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFDXGIDeviceManagerSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFDXGIDeviceManagerSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFDXGIDeviceManagerSource_GetManager(This,ppManager)	\
    ( (This)->lpVtbl -> GetManager(This,ppManager) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFDXGIDeviceManagerSource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0014 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
typedef 
enum _MF_VIDEO_PROCESSOR_ROTATION
    {
        ROTATION_NONE	= 0,
        ROTATION_NORMAL	= 1
    } 	MF_VIDEO_PROCESSOR_ROTATION;

typedef 
enum _MF_VIDEO_PROCESSOR_MIRROR
    {
        MIRROR_NONE	= 0,
        MIRROR_HORIZONTAL	= 1,
        MIRROR_VERTICAL	= 2
    } 	MF_VIDEO_PROCESSOR_MIRROR;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0014_v0_0_s_ifspec;

#ifndef __IMFVideoProcessorControl_INTERFACE_DEFINED__
#define __IMFVideoProcessorControl_INTERFACE_DEFINED__

/* interface IMFVideoProcessorControl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFVideoProcessorControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A3F675D5-6119-4f7f-A100-1D8B280F0EFB")
    IMFVideoProcessorControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBorderColor( 
            /* [annotation][in] */ 
            _In_opt_  MFARGB *pBorderColor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSourceRectangle( 
            /* [annotation][in] */ 
            _In_opt_  RECT *pSrcRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDestinationRectangle( 
            /* [annotation][in] */ 
            _In_opt_  RECT *pDstRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMirror( 
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_MIRROR eMirror) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRotation( 
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_ROTATION eRotation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetConstrictionSize( 
            /* [annotation][in] */ 
            _In_opt_  SIZE *pConstrictionSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoProcessorControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoProcessorControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoProcessorControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoProcessorControl * This);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *SetBorderColor )( 
            IMFVideoProcessorControl * This,
            /* [annotation][in] */ 
            _In_opt_  MFARGB *pBorderColor);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetSourceRectangle)
        HRESULT ( STDMETHODCALLTYPE *SetSourceRectangle )( 
            IMFVideoProcessorControl * This,
            /* [annotation][in] */ 
            _In_opt_  RECT *pSrcRect);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetDestinationRectangle)
        HRESULT ( STDMETHODCALLTYPE *SetDestinationRectangle )( 
            IMFVideoProcessorControl * This,
            /* [annotation][in] */ 
            _In_opt_  RECT *pDstRect);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetMirror)
        HRESULT ( STDMETHODCALLTYPE *SetMirror )( 
            IMFVideoProcessorControl * This,
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_MIRROR eMirror);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetRotation)
        HRESULT ( STDMETHODCALLTYPE *SetRotation )( 
            IMFVideoProcessorControl * This,
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_ROTATION eRotation);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetConstrictionSize)
        HRESULT ( STDMETHODCALLTYPE *SetConstrictionSize )( 
            IMFVideoProcessorControl * This,
            /* [annotation][in] */ 
            _In_opt_  SIZE *pConstrictionSize);
        
        END_INTERFACE
    } IMFVideoProcessorControlVtbl;

    interface IMFVideoProcessorControl
    {
        CONST_VTBL struct IMFVideoProcessorControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoProcessorControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoProcessorControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoProcessorControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoProcessorControl_SetBorderColor(This,pBorderColor)	\
    ( (This)->lpVtbl -> SetBorderColor(This,pBorderColor) ) 

#define IMFVideoProcessorControl_SetSourceRectangle(This,pSrcRect)	\
    ( (This)->lpVtbl -> SetSourceRectangle(This,pSrcRect) ) 

#define IMFVideoProcessorControl_SetDestinationRectangle(This,pDstRect)	\
    ( (This)->lpVtbl -> SetDestinationRectangle(This,pDstRect) ) 

#define IMFVideoProcessorControl_SetMirror(This,eMirror)	\
    ( (This)->lpVtbl -> SetMirror(This,eMirror) ) 

#define IMFVideoProcessorControl_SetRotation(This,eRotation)	\
    ( (This)->lpVtbl -> SetRotation(This,eRotation) ) 

#define IMFVideoProcessorControl_SetConstrictionSize(This,pConstrictionSize)	\
    ( (This)->lpVtbl -> SetConstrictionSize(This,pConstrictionSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoProcessorControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0015 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WINBLUE) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0015_v0_0_s_ifspec;

#ifndef __IMFVideoProcessorControl2_INTERFACE_DEFINED__
#define __IMFVideoProcessorControl2_INTERFACE_DEFINED__

/* interface IMFVideoProcessorControl2 */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IMFVideoProcessorControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BDE633D3-E1DC-4a7f-A693-BBAE399C4A20")
    IMFVideoProcessorControl2 : public IMFVideoProcessorControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRotationOverride( 
            /* [annotation][in] */ 
            _In_  UINT uiRotation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableHardwareEffects( 
            /* [annotation][in] */ 
            _In_  BOOL fEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedHardwareEffects( 
            /* [annotation][retval][out] */ 
            _Out_  UINT *puiSupport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoProcessorControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoProcessorControl2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoProcessorControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoProcessorControl2 * This);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *SetBorderColor )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_opt_  MFARGB *pBorderColor);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetSourceRectangle)
        HRESULT ( STDMETHODCALLTYPE *SetSourceRectangle )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_opt_  RECT *pSrcRect);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetDestinationRectangle)
        HRESULT ( STDMETHODCALLTYPE *SetDestinationRectangle )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_opt_  RECT *pDstRect);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetMirror)
        HRESULT ( STDMETHODCALLTYPE *SetMirror )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_MIRROR eMirror);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetRotation)
        HRESULT ( STDMETHODCALLTYPE *SetRotation )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_ROTATION eRotation);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetConstrictionSize)
        HRESULT ( STDMETHODCALLTYPE *SetConstrictionSize )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_opt_  SIZE *pConstrictionSize);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl2, SetRotationOverride)
        HRESULT ( STDMETHODCALLTYPE *SetRotationOverride )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_  UINT uiRotation);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl2, EnableHardwareEffects)
        HRESULT ( STDMETHODCALLTYPE *EnableHardwareEffects )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl2, GetSupportedHardwareEffects)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedHardwareEffects )( 
            IMFVideoProcessorControl2 * This,
            /* [annotation][retval][out] */ 
            _Out_  UINT *puiSupport);
        
        END_INTERFACE
    } IMFVideoProcessorControl2Vtbl;

    interface IMFVideoProcessorControl2
    {
        CONST_VTBL struct IMFVideoProcessorControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoProcessorControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoProcessorControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoProcessorControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoProcessorControl2_SetBorderColor(This,pBorderColor)	\
    ( (This)->lpVtbl -> SetBorderColor(This,pBorderColor) ) 

#define IMFVideoProcessorControl2_SetSourceRectangle(This,pSrcRect)	\
    ( (This)->lpVtbl -> SetSourceRectangle(This,pSrcRect) ) 

#define IMFVideoProcessorControl2_SetDestinationRectangle(This,pDstRect)	\
    ( (This)->lpVtbl -> SetDestinationRectangle(This,pDstRect) ) 

#define IMFVideoProcessorControl2_SetMirror(This,eMirror)	\
    ( (This)->lpVtbl -> SetMirror(This,eMirror) ) 

#define IMFVideoProcessorControl2_SetRotation(This,eRotation)	\
    ( (This)->lpVtbl -> SetRotation(This,eRotation) ) 

#define IMFVideoProcessorControl2_SetConstrictionSize(This,pConstrictionSize)	\
    ( (This)->lpVtbl -> SetConstrictionSize(This,pConstrictionSize) ) 


#define IMFVideoProcessorControl2_SetRotationOverride(This,uiRotation)	\
    ( (This)->lpVtbl -> SetRotationOverride(This,uiRotation) ) 

#define IMFVideoProcessorControl2_EnableHardwareEffects(This,fEnabled)	\
    ( (This)->lpVtbl -> EnableHardwareEffects(This,fEnabled) ) 

#define IMFVideoProcessorControl2_GetSupportedHardwareEffects(This,puiSupport)	\
    ( (This)->lpVtbl -> GetSupportedHardwareEffects(This,puiSupport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoProcessorControl2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0016 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN10) 
typedef 
enum _MFVideoSphericalFormat
    {
        MFVideoSphericalFormat_Unsupported	= 0,
        MFVideoSphericalFormat_Equirectangular	= 1,
        MFVideoSphericalFormat_CubeMap	= 2,
        MFVideoSphericalFormat_3DMesh	= 3
    } 	MFVideoSphericalFormat;

#endif /* (WINVER >= _WIN32_WINNT_WIN10) */ 
#if (NTDDI_VERSION >= NTDDI_WIN10_RS3) 
EXTERN_GUID( MF_XVP_SAMPLE_LOCK_TIMEOUT, 0xaa4ddb29, 0x5134, 0x4363, 0xac, 0x72, 0x83, 0xec, 0x4b, 0xc1, 0x4, 0x26);
typedef 
enum MFVideoSphericalProjectionMode
    {
        MFVideoSphericalProjectionMode_Spherical	= 0,
        MFVideoSphericalProjectionMode_Flat	= ( MFVideoSphericalProjectionMode_Spherical + 1 ) 
    } 	MFVideoSphericalProjectionMode;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0016_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0016_v0_0_s_ifspec;

#ifndef __IMFVideoProcessorControl3_INTERFACE_DEFINED__
#define __IMFVideoProcessorControl3_INTERFACE_DEFINED__

/* interface IMFVideoProcessorControl3 */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IMFVideoProcessorControl3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2424B3F2-EB23-40f1-91AA-74BDDEEA0883")
    IMFVideoProcessorControl3 : public IMFVideoProcessorControl2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNaturalOutputType( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableSphericalVideoProcessing( 
            /* [annotation][in] */ 
            _In_  BOOL fEnable,
            /* [annotation][in] */ 
            _In_  MFVideoSphericalFormat eFormat,
            /* [annotation][in] */ 
            _In_  MFVideoSphericalProjectionMode eProjectionMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSphericalVideoProperties( 
            /* [annotation][in] */ 
            _In_  float X,
            /* [annotation][in] */ 
            _In_  float Y,
            /* [annotation][in] */ 
            _In_  float Z,
            /* [annotation][in] */ 
            _In_  float W,
            /* [annotation][in] */ 
            _In_  float fieldOfView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputDevice( 
            /* [annotation][in] */ 
            _In_  IUnknown *pOutputDevice) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoProcessorControl3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoProcessorControl3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoProcessorControl3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoProcessorControl3 * This);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *SetBorderColor )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_opt_  MFARGB *pBorderColor);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetSourceRectangle)
        HRESULT ( STDMETHODCALLTYPE *SetSourceRectangle )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_opt_  RECT *pSrcRect);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetDestinationRectangle)
        HRESULT ( STDMETHODCALLTYPE *SetDestinationRectangle )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_opt_  RECT *pDstRect);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetMirror)
        HRESULT ( STDMETHODCALLTYPE *SetMirror )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_MIRROR eMirror);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetRotation)
        HRESULT ( STDMETHODCALLTYPE *SetRotation )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  MF_VIDEO_PROCESSOR_ROTATION eRotation);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl, SetConstrictionSize)
        HRESULT ( STDMETHODCALLTYPE *SetConstrictionSize )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_opt_  SIZE *pConstrictionSize);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl2, SetRotationOverride)
        HRESULT ( STDMETHODCALLTYPE *SetRotationOverride )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  UINT uiRotation);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl2, EnableHardwareEffects)
        HRESULT ( STDMETHODCALLTYPE *EnableHardwareEffects )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnabled);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl2, GetSupportedHardwareEffects)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedHardwareEffects )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][retval][out] */ 
            _Out_  UINT *puiSupport);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl3, GetNaturalOutputType)
        HRESULT ( STDMETHODCALLTYPE *GetNaturalOutputType )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl3, EnableSphericalVideoProcessing)
        HRESULT ( STDMETHODCALLTYPE *EnableSphericalVideoProcessing )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  BOOL fEnable,
            /* [annotation][in] */ 
            _In_  MFVideoSphericalFormat eFormat,
            /* [annotation][in] */ 
            _In_  MFVideoSphericalProjectionMode eProjectionMode);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl3, SetSphericalVideoProperties)
        HRESULT ( STDMETHODCALLTYPE *SetSphericalVideoProperties )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  float X,
            /* [annotation][in] */ 
            _In_  float Y,
            /* [annotation][in] */ 
            _In_  float Z,
            /* [annotation][in] */ 
            _In_  float W,
            /* [annotation][in] */ 
            _In_  float fieldOfView);
        
        DECLSPEC_XFGVIRT(IMFVideoProcessorControl3, SetOutputDevice)
        HRESULT ( STDMETHODCALLTYPE *SetOutputDevice )( 
            IMFVideoProcessorControl3 * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pOutputDevice);
        
        END_INTERFACE
    } IMFVideoProcessorControl3Vtbl;

    interface IMFVideoProcessorControl3
    {
        CONST_VTBL struct IMFVideoProcessorControl3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoProcessorControl3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoProcessorControl3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoProcessorControl3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoProcessorControl3_SetBorderColor(This,pBorderColor)	\
    ( (This)->lpVtbl -> SetBorderColor(This,pBorderColor) ) 

#define IMFVideoProcessorControl3_SetSourceRectangle(This,pSrcRect)	\
    ( (This)->lpVtbl -> SetSourceRectangle(This,pSrcRect) ) 

#define IMFVideoProcessorControl3_SetDestinationRectangle(This,pDstRect)	\
    ( (This)->lpVtbl -> SetDestinationRectangle(This,pDstRect) ) 

#define IMFVideoProcessorControl3_SetMirror(This,eMirror)	\
    ( (This)->lpVtbl -> SetMirror(This,eMirror) ) 

#define IMFVideoProcessorControl3_SetRotation(This,eRotation)	\
    ( (This)->lpVtbl -> SetRotation(This,eRotation) ) 

#define IMFVideoProcessorControl3_SetConstrictionSize(This,pConstrictionSize)	\
    ( (This)->lpVtbl -> SetConstrictionSize(This,pConstrictionSize) ) 


#define IMFVideoProcessorControl3_SetRotationOverride(This,uiRotation)	\
    ( (This)->lpVtbl -> SetRotationOverride(This,uiRotation) ) 

#define IMFVideoProcessorControl3_EnableHardwareEffects(This,fEnabled)	\
    ( (This)->lpVtbl -> EnableHardwareEffects(This,fEnabled) ) 

#define IMFVideoProcessorControl3_GetSupportedHardwareEffects(This,puiSupport)	\
    ( (This)->lpVtbl -> GetSupportedHardwareEffects(This,puiSupport) ) 


#define IMFVideoProcessorControl3_GetNaturalOutputType(This,ppType)	\
    ( (This)->lpVtbl -> GetNaturalOutputType(This,ppType) ) 

#define IMFVideoProcessorControl3_EnableSphericalVideoProcessing(This,fEnable,eFormat,eProjectionMode)	\
    ( (This)->lpVtbl -> EnableSphericalVideoProcessing(This,fEnable,eFormat,eProjectionMode) ) 

#define IMFVideoProcessorControl3_SetSphericalVideoProperties(This,X,Y,Z,W,fieldOfView)	\
    ( (This)->lpVtbl -> SetSphericalVideoProperties(This,X,Y,Z,W,fieldOfView) ) 

#define IMFVideoProcessorControl3_SetOutputDevice(This,pOutputDevice)	\
    ( (This)->lpVtbl -> SetOutputDevice(This,pOutputDevice) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoProcessorControl3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0017 */
/* [local] */ 

#endif /* (NTDDI_VERSION >= NTDDI_WIN10_RS3) */ 
#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 
#if (NTDDI_VERSION >= NTDDI_WIN10_VB) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0017_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0017_v0_0_s_ifspec;

#ifndef __IMFVideoRendererEffectControl_INTERFACE_DEFINED__
#define __IMFVideoRendererEffectControl_INTERFACE_DEFINED__

/* interface IMFVideoRendererEffectControl */
/* [uuid][local][object] */ 


EXTERN_C const IID IID_IMFVideoRendererEffectControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("604D33D7-CF23-41d5-8224-5BBBB1A87475")
    IMFVideoRendererEffectControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnAppServiceConnectionEstablished( 
            /* [annotation][in] */ 
            _In_  IUnknown *pAppServiceConnection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoRendererEffectControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoRendererEffectControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoRendererEffectControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoRendererEffectControl * This);
        
        DECLSPEC_XFGVIRT(IMFVideoRendererEffectControl, OnAppServiceConnectionEstablished)
        HRESULT ( STDMETHODCALLTYPE *OnAppServiceConnectionEstablished )( 
            IMFVideoRendererEffectControl * This,
            /* [annotation][in] */ 
            _In_  IUnknown *pAppServiceConnection);
        
        END_INTERFACE
    } IMFVideoRendererEffectControlVtbl;

    interface IMFVideoRendererEffectControl
    {
        CONST_VTBL struct IMFVideoRendererEffectControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoRendererEffectControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoRendererEffectControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoRendererEffectControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoRendererEffectControl_OnAppServiceConnectionEstablished(This,pAppServiceConnection)	\
    ( (This)->lpVtbl -> OnAppServiceConnectionEstablished(This,pAppServiceConnection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoRendererEffectControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0018 */
/* [local] */ 

#endif // (WINVER >= NTDDI_WIN10_VB) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0018_v0_0_s_ifspec;

#ifndef __IMFTopology_INTERFACE_DEFINED__
#define __IMFTopology_INTERFACE_DEFINED__

/* interface IMFTopology */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFTopology;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83CF873A-F6DA-4bc8-823F-BACFD55DC433")
    IMFTopology : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTopologyID( 
            /* [out] */ __RPC__out TOPOID *pID) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE AddNode( 
            /* [in] */ IMFTopologyNode *pNode) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RemoveNode( 
            /* [in] */ IMFTopologyNode *pNode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNodeCount( 
            /* [out] */ __RPC__out WORD *pwNodes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNode( 
            /* [in] */ WORD wIndex,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppNode) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Clear( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloneFrom( 
            /* [in] */ __RPC__in_opt IMFTopology *pTopology) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNodeByID( 
            /* [in] */ TOPOID qwTopoNodeID,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppNode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceNodeCollection( 
            /* [out] */ __RPC__deref_out_opt IMFCollection **ppCollection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputNodeCollection( 
            /* [out] */ __RPC__deref_out_opt IMFCollection **ppCollection) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTopologyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFTopology * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFTopology * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFTopology * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFTopology * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFTopology * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFTopology * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFTopology * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFTopology * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFTopology * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFTopology * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFTopology * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFTopology, GetTopologyID)
        HRESULT ( STDMETHODCALLTYPE *GetTopologyID )( 
            __RPC__in IMFTopology * This,
            /* [out] */ __RPC__out TOPOID *pID);
        
        DECLSPEC_XFGVIRT(IMFTopology, AddNode)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *AddNode )( 
            IMFTopology * This,
            /* [in] */ IMFTopologyNode *pNode);
        
        DECLSPEC_XFGVIRT(IMFTopology, RemoveNode)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RemoveNode )( 
            IMFTopology * This,
            /* [in] */ IMFTopologyNode *pNode);
        
        DECLSPEC_XFGVIRT(IMFTopology, GetNodeCount)
        HRESULT ( STDMETHODCALLTYPE *GetNodeCount )( 
            __RPC__in IMFTopology * This,
            /* [out] */ __RPC__out WORD *pwNodes);
        
        DECLSPEC_XFGVIRT(IMFTopology, GetNode)
        HRESULT ( STDMETHODCALLTYPE *GetNode )( 
            __RPC__in IMFTopology * This,
            /* [in] */ WORD wIndex,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppNode);
        
        DECLSPEC_XFGVIRT(IMFTopology, Clear)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Clear )( 
            IMFTopology * This);
        
        DECLSPEC_XFGVIRT(IMFTopology, CloneFrom)
        HRESULT ( STDMETHODCALLTYPE *CloneFrom )( 
            __RPC__in IMFTopology * This,
            /* [in] */ __RPC__in_opt IMFTopology *pTopology);
        
        DECLSPEC_XFGVIRT(IMFTopology, GetNodeByID)
        HRESULT ( STDMETHODCALLTYPE *GetNodeByID )( 
            __RPC__in IMFTopology * This,
            /* [in] */ TOPOID qwTopoNodeID,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppNode);
        
        DECLSPEC_XFGVIRT(IMFTopology, GetSourceNodeCollection)
        HRESULT ( STDMETHODCALLTYPE *GetSourceNodeCollection )( 
            __RPC__in IMFTopology * This,
            /* [out] */ __RPC__deref_out_opt IMFCollection **ppCollection);
        
        DECLSPEC_XFGVIRT(IMFTopology, GetOutputNodeCollection)
        HRESULT ( STDMETHODCALLTYPE *GetOutputNodeCollection )( 
            __RPC__in IMFTopology * This,
            /* [out] */ __RPC__deref_out_opt IMFCollection **ppCollection);
        
        END_INTERFACE
    } IMFTopologyVtbl;

    interface IMFTopology
    {
        CONST_VTBL struct IMFTopologyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTopology_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTopology_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTopology_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTopology_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFTopology_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFTopology_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFTopology_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFTopology_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFTopology_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFTopology_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFTopology_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFTopology_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFTopology_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFTopology_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFTopology_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFTopology_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFTopology_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFTopology_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFTopology_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFTopology_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFTopology_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFTopology_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFTopology_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFTopology_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFTopology_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFTopology_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFTopology_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFTopology_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFTopology_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFTopology_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFTopology_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFTopology_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFTopology_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFTopology_GetTopologyID(This,pID)	\
    ( (This)->lpVtbl -> GetTopologyID(This,pID) ) 

#define IMFTopology_AddNode(This,pNode)	\
    ( (This)->lpVtbl -> AddNode(This,pNode) ) 

#define IMFTopology_RemoveNode(This,pNode)	\
    ( (This)->lpVtbl -> RemoveNode(This,pNode) ) 

#define IMFTopology_GetNodeCount(This,pwNodes)	\
    ( (This)->lpVtbl -> GetNodeCount(This,pwNodes) ) 

#define IMFTopology_GetNode(This,wIndex,ppNode)	\
    ( (This)->lpVtbl -> GetNode(This,wIndex,ppNode) ) 

#define IMFTopology_Clear(This)	\
    ( (This)->lpVtbl -> Clear(This) ) 

#define IMFTopology_CloneFrom(This,pTopology)	\
    ( (This)->lpVtbl -> CloneFrom(This,pTopology) ) 

#define IMFTopology_GetNodeByID(This,qwTopoNodeID,ppNode)	\
    ( (This)->lpVtbl -> GetNodeByID(This,qwTopoNodeID,ppNode) ) 

#define IMFTopology_GetSourceNodeCollection(This,ppCollection)	\
    ( (This)->lpVtbl -> GetSourceNodeCollection(This,ppCollection) ) 

#define IMFTopology_GetOutputNodeCollection(This,ppCollection)	\
    ( (This)->lpVtbl -> GetOutputNodeCollection(This,ppCollection) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTopology_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0019 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
EXTERN_GUID( MF_TOPOLOGY_PROJECTSTART, 0x7ed3f802, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80);
EXTERN_GUID( MF_TOPOLOGY_PROJECTSTOP, 0x7ed3f803, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80);
EXTERN_GUID( MF_TOPOLOGY_NO_MARKIN_MARKOUT, 0x7ed3f804, 0x86bb, 0x4b3f, 0xb7, 0xe4, 0x7c, 0xb4, 0x3a, 0xfd, 0x4b, 0x80);
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum MFTOPOLOGY_DXVA_MODE
    {
        MFTOPOLOGY_DXVA_DEFAULT	= 0,
        MFTOPOLOGY_DXVA_NONE	= 1,
        MFTOPOLOGY_DXVA_FULL	= 2
    } 	MFTOPOLOGY_DXVA_MODE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
EXTERN_GUID(MF_TOPOLOGY_DXVA_MODE, 0x1e8d34f6, 0xf5ab, 0x4e23, 0xbb, 0x88, 0x87, 0x4a, 0xa3, 0xa1, 0xa7, 0x4d);
EXTERN_GUID(MF_TOPOLOGY_ENABLE_XVP_FOR_PLAYBACK, 0x1967731f, 0xcd78, 0x42fc, 0xb0, 0x26, 0x9, 0x92, 0xa5, 0x6e, 0x56, 0x93);
EXTERN_GUID(MF_TOPOLOGY_STATIC_PLAYBACK_OPTIMIZATIONS, 0xb86cac42, 0x41a6, 0x4b79, 0x89, 0x7a, 0x1a, 0xb0, 0xe5, 0x2b, 0x4a, 0x1b);
EXTERN_GUID(MF_TOPOLOGY_PLAYBACK_MAX_DIMS,  0x5715cf19, 0x5768, 0x44aa, 0xad, 0x6e, 0x87, 0x21, 0xf1, 0xb0, 0xf9, 0xbb);
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum MFTOPOLOGY_HARDWARE_MODE
    {
        MFTOPOLOGY_HWMODE_SOFTWARE_ONLY	= 0,
        MFTOPOLOGY_HWMODE_USE_HARDWARE	= 1,
        MFTOPOLOGY_HWMODE_USE_ONLY_HARDWARE	= 2
    } 	MFTOPOLOGY_HARDWARE_MODE;

EXTERN_GUID(MF_TOPOLOGY_HARDWARE_MODE, 0xd2d362fd, 0x4e4f, 0x4191, 0xa5, 0x79, 0xc6, 0x18, 0xb6, 0x67, 0x6, 0xaf);
EXTERN_GUID(MF_TOPOLOGY_PLAYBACK_FRAMERATE, 0xc164737a, 0xc2b1, 0x4553, 0x83, 0xbb, 0x5a, 0x52, 0x60, 0x72, 0x44, 0x8f);
EXTERN_GUID(MF_TOPOLOGY_DYNAMIC_CHANGE_NOT_ALLOWED, 0xd529950b, 0xd484, 0x4527, 0xa9, 0xcd, 0xb1, 0x90, 0x95, 0x32, 0xb5, 0xb0);
EXTERN_GUID(MF_TOPOLOGY_ENUMERATE_SOURCE_TYPES, 0x6248c36d, 0x5d0b, 0x4f40, 0xa0, 0xbb, 0xb0, 0xb3, 0x05, 0xf7, 0x76, 0x98);
EXTERN_GUID( MF_TOPOLOGY_START_TIME_ON_PRESENTATION_SWITCH, 0xc8cc113f, 0x7951, 0x4548, 0xaa, 0xd6, 0x9e, 0xd6, 0x20, 0x2e, 0x62, 0xb3);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MF_DISABLE_LOCALLY_REGISTERED_PLUGINS, 0x66b16da9, 0xadd4, 0x47e0, 0xa1, 0x6b, 0x5a, 0xf1, 0xfb, 0x48, 0x36, 0x34);
EXTERN_GUID( MF_LOCAL_PLUGIN_CONTROL_POLICY, 0xd91b0085, 0xc86d, 0x4f81, 0x88, 0x22, 0x8c, 0x68, 0xe1, 0xd7, 0xfa, 0x04);
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region PC Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)
STDAPI MFCreateTopology(
    _Outptr_ IMFTopology ** ppTopo );
typedef 
enum MF_TOPOLOGY_TYPE
    {
        MF_TOPOLOGY_OUTPUT_NODE	= 0,
        MF_TOPOLOGY_SOURCESTREAM_NODE	= ( MF_TOPOLOGY_OUTPUT_NODE + 1 ) ,
        MF_TOPOLOGY_TRANSFORM_NODE	= ( MF_TOPOLOGY_SOURCESTREAM_NODE + 1 ) ,
        MF_TOPOLOGY_TEE_NODE	= ( MF_TOPOLOGY_TRANSFORM_NODE + 1 ) ,
        MF_TOPOLOGY_MAX	= 0xffffffff
    } 	MF_TOPOLOGY_TYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0019_v0_0_s_ifspec;

#ifndef __IMFTopologyNode_INTERFACE_DEFINED__
#define __IMFTopologyNode_INTERFACE_DEFINED__

/* interface IMFTopologyNode */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFTopologyNode;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83CF873A-F6DA-4bc8-823F-BACFD55DC430")
    IMFTopologyNode : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetObject( 
            /* [in] */ __RPC__in_opt IUnknown *pObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetObject( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNodeType( 
            /* [out] */ __RPC__out MF_TOPOLOGY_TYPE *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTopoNodeID( 
            /* [out] */ __RPC__out TOPOID *pID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTopoNodeID( 
            /* [in] */ TOPOID ullTopoID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInputCount( 
            /* [out] */ __RPC__out DWORD *pcInputs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputCount( 
            /* [out] */ __RPC__out DWORD *pcOutputs) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE ConnectOutput( 
            /* [in] */ DWORD dwOutputIndex,
            /* [in] */ IMFTopologyNode *pDownstreamNode,
            /* [in] */ DWORD dwInputIndexOnDownstreamNode) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE DisconnectOutput( 
            /* [in] */ DWORD dwOutputIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInput( 
            /* [in] */ DWORD dwInputIndex,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppUpstreamNode,
            /* [out] */ __RPC__out DWORD *pdwOutputIndexOnUpstreamNode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutput( 
            /* [in] */ DWORD dwOutputIndex,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppDownstreamNode,
            /* [out] */ __RPC__out DWORD *pdwInputIndexOnDownstreamNode) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetOutputPrefType( 
            /* [in] */ DWORD dwOutputIndex,
            /* [in] */ IMFMediaType *pType) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetOutputPrefType( 
            /* [in] */ DWORD dwOutputIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetInputPrefType( 
            /* [in] */ DWORD dwInputIndex,
            /* [in] */ IMFMediaType *pType) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetInputPrefType( 
            /* [in] */ DWORD dwInputIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloneFrom( 
            /* [in] */ __RPC__in_opt IMFTopologyNode *pNode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTopologyNodeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFTopologyNode * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFTopologyNode * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFTopologyNode * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFTopologyNode * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFTopologyNode * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFTopologyNode * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFTopologyNode * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFTopologyNode * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, SetObject)
        HRESULT ( STDMETHODCALLTYPE *SetObject )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ __RPC__in_opt IUnknown *pObject);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetObject)
        HRESULT ( STDMETHODCALLTYPE *GetObject )( 
            __RPC__in IMFTopologyNode * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetNodeType)
        HRESULT ( STDMETHODCALLTYPE *GetNodeType )( 
            __RPC__in IMFTopologyNode * This,
            /* [out] */ __RPC__out MF_TOPOLOGY_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetTopoNodeID)
        HRESULT ( STDMETHODCALLTYPE *GetTopoNodeID )( 
            __RPC__in IMFTopologyNode * This,
            /* [out] */ __RPC__out TOPOID *pID);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, SetTopoNodeID)
        HRESULT ( STDMETHODCALLTYPE *SetTopoNodeID )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ TOPOID ullTopoID);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetInputCount)
        HRESULT ( STDMETHODCALLTYPE *GetInputCount )( 
            __RPC__in IMFTopologyNode * This,
            /* [out] */ __RPC__out DWORD *pcInputs);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetOutputCount)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCount )( 
            __RPC__in IMFTopologyNode * This,
            /* [out] */ __RPC__out DWORD *pcOutputs);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, ConnectOutput)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *ConnectOutput )( 
            IMFTopologyNode * This,
            /* [in] */ DWORD dwOutputIndex,
            /* [in] */ IMFTopologyNode *pDownstreamNode,
            /* [in] */ DWORD dwInputIndexOnDownstreamNode);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, DisconnectOutput)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *DisconnectOutput )( 
            IMFTopologyNode * This,
            /* [in] */ DWORD dwOutputIndex);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetInput)
        HRESULT ( STDMETHODCALLTYPE *GetInput )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ DWORD dwInputIndex,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppUpstreamNode,
            /* [out] */ __RPC__out DWORD *pdwOutputIndexOnUpstreamNode);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetOutput)
        HRESULT ( STDMETHODCALLTYPE *GetOutput )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ DWORD dwOutputIndex,
            /* [out] */ __RPC__deref_out_opt IMFTopologyNode **ppDownstreamNode,
            /* [out] */ __RPC__out DWORD *pdwInputIndexOnDownstreamNode);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, SetOutputPrefType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetOutputPrefType )( 
            IMFTopologyNode * This,
            /* [in] */ DWORD dwOutputIndex,
            /* [in] */ IMFMediaType *pType);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetOutputPrefType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetOutputPrefType )( 
            IMFTopologyNode * This,
            /* [in] */ DWORD dwOutputIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, SetInputPrefType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetInputPrefType )( 
            IMFTopologyNode * This,
            /* [in] */ DWORD dwInputIndex,
            /* [in] */ IMFMediaType *pType);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, GetInputPrefType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetInputPrefType )( 
            IMFTopologyNode * This,
            /* [in] */ DWORD dwInputIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFTopologyNode, CloneFrom)
        HRESULT ( STDMETHODCALLTYPE *CloneFrom )( 
            __RPC__in IMFTopologyNode * This,
            /* [in] */ __RPC__in_opt IMFTopologyNode *pNode);
        
        END_INTERFACE
    } IMFTopologyNodeVtbl;

    interface IMFTopologyNode
    {
        CONST_VTBL struct IMFTopologyNodeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTopologyNode_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTopologyNode_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTopologyNode_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTopologyNode_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFTopologyNode_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFTopologyNode_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFTopologyNode_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFTopologyNode_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFTopologyNode_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFTopologyNode_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFTopologyNode_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFTopologyNode_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFTopologyNode_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFTopologyNode_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFTopologyNode_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFTopologyNode_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFTopologyNode_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFTopologyNode_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFTopologyNode_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFTopologyNode_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFTopologyNode_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFTopologyNode_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFTopologyNode_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFTopologyNode_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFTopologyNode_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFTopologyNode_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFTopologyNode_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFTopologyNode_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFTopologyNode_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFTopologyNode_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFTopologyNode_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFTopologyNode_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFTopologyNode_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFTopologyNode_SetObject(This,pObject)	\
    ( (This)->lpVtbl -> SetObject(This,pObject) ) 

#define IMFTopologyNode_GetObject(This,ppObject)	\
    ( (This)->lpVtbl -> GetObject(This,ppObject) ) 

#define IMFTopologyNode_GetNodeType(This,pType)	\
    ( (This)->lpVtbl -> GetNodeType(This,pType) ) 

#define IMFTopologyNode_GetTopoNodeID(This,pID)	\
    ( (This)->lpVtbl -> GetTopoNodeID(This,pID) ) 

#define IMFTopologyNode_SetTopoNodeID(This,ullTopoID)	\
    ( (This)->lpVtbl -> SetTopoNodeID(This,ullTopoID) ) 

#define IMFTopologyNode_GetInputCount(This,pcInputs)	\
    ( (This)->lpVtbl -> GetInputCount(This,pcInputs) ) 

#define IMFTopologyNode_GetOutputCount(This,pcOutputs)	\
    ( (This)->lpVtbl -> GetOutputCount(This,pcOutputs) ) 

#define IMFTopologyNode_ConnectOutput(This,dwOutputIndex,pDownstreamNode,dwInputIndexOnDownstreamNode)	\
    ( (This)->lpVtbl -> ConnectOutput(This,dwOutputIndex,pDownstreamNode,dwInputIndexOnDownstreamNode) ) 

#define IMFTopologyNode_DisconnectOutput(This,dwOutputIndex)	\
    ( (This)->lpVtbl -> DisconnectOutput(This,dwOutputIndex) ) 

#define IMFTopologyNode_GetInput(This,dwInputIndex,ppUpstreamNode,pdwOutputIndexOnUpstreamNode)	\
    ( (This)->lpVtbl -> GetInput(This,dwInputIndex,ppUpstreamNode,pdwOutputIndexOnUpstreamNode) ) 

#define IMFTopologyNode_GetOutput(This,dwOutputIndex,ppDownstreamNode,pdwInputIndexOnDownstreamNode)	\
    ( (This)->lpVtbl -> GetOutput(This,dwOutputIndex,ppDownstreamNode,pdwInputIndexOnDownstreamNode) ) 

#define IMFTopologyNode_SetOutputPrefType(This,dwOutputIndex,pType)	\
    ( (This)->lpVtbl -> SetOutputPrefType(This,dwOutputIndex,pType) ) 

#define IMFTopologyNode_GetOutputPrefType(This,dwOutputIndex,ppType)	\
    ( (This)->lpVtbl -> GetOutputPrefType(This,dwOutputIndex,ppType) ) 

#define IMFTopologyNode_SetInputPrefType(This,dwInputIndex,pType)	\
    ( (This)->lpVtbl -> SetInputPrefType(This,dwInputIndex,pType) ) 

#define IMFTopologyNode_GetInputPrefType(This,dwInputIndex,ppType)	\
    ( (This)->lpVtbl -> GetInputPrefType(This,dwInputIndex,ppType) ) 

#define IMFTopologyNode_CloneFrom(This,pNode)	\
    ( (This)->lpVtbl -> CloneFrom(This,pNode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFTopologyNode_RemoteGetOutputPrefType_Proxy( 
    __RPC__in IMFTopologyNode * This,
    /* [in] */ DWORD dwOutputIndex,
    /* [out] */ __RPC__out DWORD *pcbData,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData);


void __RPC_STUB IMFTopologyNode_RemoteGetOutputPrefType_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFTopologyNode_RemoteGetInputPrefType_Proxy( 
    __RPC__in IMFTopologyNode * This,
    /* [in] */ DWORD dwInputIndex,
    /* [out] */ __RPC__out DWORD *pcbData,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData);


void __RPC_STUB IMFTopologyNode_RemoteGetInputPrefType_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFTopologyNode_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0020 */
/* [local] */ 

typedef 
enum _MF_TOPONODE_FLUSH_MODE
    {
        MF_TOPONODE_FLUSH_ALWAYS	= 0,
        MF_TOPONODE_FLUSH_SEEK	= ( MF_TOPONODE_FLUSH_ALWAYS + 1 ) ,
        MF_TOPONODE_FLUSH_NEVER	= ( MF_TOPONODE_FLUSH_SEEK + 1 ) 
    } 	MF_TOPONODE_FLUSH_MODE;

EXTERN_GUID( MF_TOPONODE_FLUSH, 0x494bbce8, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
typedef 
enum _MF_TOPONODE_DRAIN_MODE
    {
        MF_TOPONODE_DRAIN_DEFAULT	= 0,
        MF_TOPONODE_DRAIN_ALWAYS	= ( MF_TOPONODE_DRAIN_DEFAULT + 1 ) ,
        MF_TOPONODE_DRAIN_NEVER	= ( MF_TOPONODE_DRAIN_ALWAYS + 1 ) 
    } 	MF_TOPONODE_DRAIN_MODE;

EXTERN_GUID( MF_TOPONODE_DRAIN, 0x494bbce9, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_D3DAWARE, 0x494bbced, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPOLOGY_RESOLUTION_STATUS, 0x494bbcde, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_ERRORCODE, 0x494bbcee, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_CONNECT_METHOD, 0x494bbcf1, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_LOCKED, 0x494bbcf7, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_WORKQUEUE_ID, 0x494bbcf8, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_WORKQUEUE_MMCSS_CLASS, 0x494bbcf9, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_DECRYPTOR, 0x494bbcfa, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_DISCARDABLE, 0x494bbcfb, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_ERROR_MAJORTYPE, 0x494bbcfd, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_ERROR_SUBTYPE, 0x494bbcfe, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_WORKQUEUE_MMCSS_TASKID, 0x494bbcff, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_WORKQUEUE_MMCSS_PRIORITY, 0x5001f840, 0x2816, 0x48f4, 0x93, 0x64, 0xad, 0x1e, 0xf6, 0x61, 0xa1, 0x23);
EXTERN_GUID( MF_TOPONODE_WORKQUEUE_ITEM_PRIORITY, 0xa1ff99be, 0x5e97, 0x4a53, 0xb4, 0x94, 0x56, 0x8c, 0x64, 0x2c, 0x0f, 0xf3);
EXTERN_GUID( MF_TOPONODE_MARKIN_HERE, 0x494bbd00, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_MARKOUT_HERE, 0x494bbd01, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_DECODER, 0x494bbd02, 0xb031,  0x4e38,  0x97, 0xc4, 0xd5, 0x42, 0x2d, 0xd6, 0x18, 0xdc);
EXTERN_GUID( MF_TOPONODE_MEDIASTART, 0x835c58ea, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6);
EXTERN_GUID( MF_TOPONODE_MEDIASTOP, 0x835c58eb, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6);
EXTERN_GUID( MF_TOPONODE_SOURCE, 0x835c58ec, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6);
EXTERN_GUID( MF_TOPONODE_PRESENTATION_DESCRIPTOR, 0x835c58ed, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6);
EXTERN_GUID( MF_TOPONODE_STREAM_DESCRIPTOR, 0x835c58ee, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6);
EXTERN_GUID( MF_TOPONODE_SEQUENCE_ELEMENTID, 0x835c58ef, 0xe075, 0x4bc7, 0xbc, 0xba, 0x4d, 0xe0, 0x00, 0xdf, 0x9a, 0xe6);
EXTERN_GUID( MF_TOPONODE_TRANSFORM_OBJECTID, 0x88dcc0c9, 0x293e, 0x4e8b, 0x9a, 0xeb, 0xa, 0xd6, 0x4c, 0xc0, 0x16, 0xb0);
EXTERN_GUID( MF_TOPONODE_STREAMID, 0x14932f9b, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04);
EXTERN_GUID( MF_TOPONODE_NOSHUTDOWN_ON_REMOVE, 0x14932f9c, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04);
EXTERN_GUID( MF_TOPONODE_RATELESS, 0x14932f9d, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04);
EXTERN_GUID( MF_TOPONODE_DISABLE_PREROLL, 0x14932f9e, 0x9087, 0x4bb4, 0x84, 0x12, 0x51, 0x67, 0x14, 0x5c, 0xbe, 0x04);
EXTERN_GUID( MF_TOPONODE_PRIMARYOUTPUT, 0x6304ef99, 0x16b2, 0x4ebe, 0x9d, 0x67, 0xe4, 0xc5, 0x39, 0xb3, 0xa2, 0x59);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region PC Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)
STDAPI MFCreateTopologyNode(
    MF_TOPOLOGY_TYPE NodeType,
    _Outptr_ IMFTopologyNode ** ppNode );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN7) 
STDAPI MFGetTopoNodeCurrentType(
    IMFTopologyNode* pNode,
    DWORD dwStreamIndex,
    BOOL fOutput,
    _Outptr_ IMFMediaType** ppType);
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0020_v0_0_s_ifspec;

#ifndef __IMFGetService_INTERFACE_DEFINED__
#define __IMFGetService_INTERFACE_DEFINED__

/* interface IMFGetService */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFGetService;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fa993888-4383-415a-a930-dd472a8cf6f7")
    IMFGetService : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetService( 
            /* [in] */ __RPC__in REFGUID guidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppvObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFGetServiceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFGetService * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFGetService * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFGetService * This);
        
        DECLSPEC_XFGVIRT(IMFGetService, GetService)
        HRESULT ( STDMETHODCALLTYPE *GetService )( 
            __RPC__in IMFGetService * This,
            /* [in] */ __RPC__in REFGUID guidService,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppvObject);
        
        END_INTERFACE
    } IMFGetServiceVtbl;

    interface IMFGetService
    {
        CONST_VTBL struct IMFGetServiceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFGetService_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFGetService_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFGetService_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFGetService_GetService(This,guidService,riid,ppvObject)	\
    ( (This)->lpVtbl -> GetService(This,guidService,riid,ppvObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFGetService_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0021 */
/* [local] */ 

STDAPI MFGetService(
    IUnknown* punkObject,
    REFGUID guidService,
    REFIID riid,
    _Outptr_ LPVOID* ppvObject
    );
typedef LONGLONG MFTIME;

typedef 
enum _MFCLOCK_CHARACTERISTICS_FLAGS
    {
        MFCLOCK_CHARACTERISTICS_FLAG_FREQUENCY_10MHZ	= 0x2,
        MFCLOCK_CHARACTERISTICS_FLAG_ALWAYS_RUNNING	= 0x4,
        MFCLOCK_CHARACTERISTICS_FLAG_IS_SYSTEM_CLOCK	= 0x8
    } 	MFCLOCK_CHARACTERISTICS_FLAGS;

typedef 
enum _MFCLOCK_STATE
    {
        MFCLOCK_STATE_INVALID	= 0,
        MFCLOCK_STATE_RUNNING	= ( MFCLOCK_STATE_INVALID + 1 ) ,
        MFCLOCK_STATE_STOPPED	= ( MFCLOCK_STATE_RUNNING + 1 ) ,
        MFCLOCK_STATE_PAUSED	= ( MFCLOCK_STATE_STOPPED + 1 ) 
    } 	MFCLOCK_STATE;

typedef 
enum _MFCLOCK_RELATIONAL_FLAGS
    {
        MFCLOCK_RELATIONAL_FLAG_JITTER_NEVER_AHEAD	= 0x1
    } 	MFCLOCK_RELATIONAL_FLAGS;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct _MFCLOCK_PROPERTIES
    {
    unsigned __int64 qwCorrelationRate;
    GUID guidClockId;
    DWORD dwClockFlags;
    unsigned __int64 qwClockFrequency;
    DWORD dwClockTolerance;
    DWORD dwClockJitter;
    } 	MFCLOCK_PROPERTIES;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
#define MFCLOCK_FREQUENCY_HNS       10000000
#define MFCLOCK_TOLERANCE_UNKNOWN   50000
#define MFCLOCK_JITTER_ISR          1000
#define MFCLOCK_JITTER_DPC          4000
#define MFCLOCK_JITTER_PASSIVE      10000


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0021_v0_0_s_ifspec;

#ifndef __IMFClock_INTERFACE_DEFINED__
#define __IMFClock_INTERFACE_DEFINED__

/* interface IMFClock */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFClock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2eb1e945-18b8-4139-9b1a-d5d584818530")
    IMFClock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClockCharacteristics( 
            /* [out] */ __RPC__out DWORD *pdwCharacteristics) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCorrelatedTime( 
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out LONGLONG *pllClockTime,
            /* [out] */ __RPC__out MFTIME *phnsSystemTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContinuityKey( 
            /* [out] */ __RPC__out DWORD *pdwContinuityKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out MFCLOCK_STATE *peClockState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [out] */ __RPC__out MFCLOCK_PROPERTIES *pClockProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFClockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFClock * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFClock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFClock * This);
        
        DECLSPEC_XFGVIRT(IMFClock, GetClockCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetClockCharacteristics )( 
            __RPC__in IMFClock * This,
            /* [out] */ __RPC__out DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFClock, GetCorrelatedTime)
        HRESULT ( STDMETHODCALLTYPE *GetCorrelatedTime )( 
            __RPC__in IMFClock * This,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out LONGLONG *pllClockTime,
            /* [out] */ __RPC__out MFTIME *phnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClock, GetContinuityKey)
        HRESULT ( STDMETHODCALLTYPE *GetContinuityKey )( 
            __RPC__in IMFClock * This,
            /* [out] */ __RPC__out DWORD *pdwContinuityKey);
        
        DECLSPEC_XFGVIRT(IMFClock, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IMFClock * This,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out MFCLOCK_STATE *peClockState);
        
        DECLSPEC_XFGVIRT(IMFClock, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IMFClock * This,
            /* [out] */ __RPC__out MFCLOCK_PROPERTIES *pClockProperties);
        
        END_INTERFACE
    } IMFClockVtbl;

    interface IMFClock
    {
        CONST_VTBL struct IMFClockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFClock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFClock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFClock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFClock_GetClockCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetClockCharacteristics(This,pdwCharacteristics) ) 

#define IMFClock_GetCorrelatedTime(This,dwReserved,pllClockTime,phnsSystemTime)	\
    ( (This)->lpVtbl -> GetCorrelatedTime(This,dwReserved,pllClockTime,phnsSystemTime) ) 

#define IMFClock_GetContinuityKey(This,pdwContinuityKey)	\
    ( (This)->lpVtbl -> GetContinuityKey(This,pdwContinuityKey) ) 

#define IMFClock_GetState(This,dwReserved,peClockState)	\
    ( (This)->lpVtbl -> GetState(This,dwReserved,peClockState) ) 

#define IMFClock_GetProperties(This,pClockProperties)	\
    ( (This)->lpVtbl -> GetProperties(This,pClockProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFClock_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0022 */
/* [local] */ 

STDAPI_(MFTIME) 
MFGetSystemTime(
    );
#define PRESENTATION_CURRENT_POSITION   0x7fffffffffffffff




extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0022_v0_0_s_ifspec;

#ifndef __IMFPresentationClock_INTERFACE_DEFINED__
#define __IMFPresentationClock_INTERFACE_DEFINED__

/* interface IMFPresentationClock */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFPresentationClock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("868CE85C-8EA9-4f55-AB82-B009A910A805")
    IMFPresentationClock : public IMFClock
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetTimeSource( 
            /* [in] */ __RPC__in_opt IMFPresentationTimeSource *pTimeSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeSource( 
            /* [out] */ __RPC__deref_out_opt IMFPresentationTimeSource **ppTimeSource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTime( 
            /* [out] */ __RPC__out MFTIME *phnsClockTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddClockStateSink( 
            /* [in] */ __RPC__in_opt IMFClockStateSink *pStateSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveClockStateSink( 
            /* [in] */ __RPC__in_opt IMFClockStateSink *pStateSink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Start( 
            /* [in] */ LONGLONG llClockStartOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPresentationClockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFPresentationClock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFPresentationClock * This);
        
        DECLSPEC_XFGVIRT(IMFClock, GetClockCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetClockCharacteristics )( 
            __RPC__in IMFPresentationClock * This,
            /* [out] */ __RPC__out DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFClock, GetCorrelatedTime)
        HRESULT ( STDMETHODCALLTYPE *GetCorrelatedTime )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out LONGLONG *pllClockTime,
            /* [out] */ __RPC__out MFTIME *phnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClock, GetContinuityKey)
        HRESULT ( STDMETHODCALLTYPE *GetContinuityKey )( 
            __RPC__in IMFPresentationClock * This,
            /* [out] */ __RPC__out DWORD *pdwContinuityKey);
        
        DECLSPEC_XFGVIRT(IMFClock, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out MFCLOCK_STATE *peClockState);
        
        DECLSPEC_XFGVIRT(IMFClock, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IMFPresentationClock * This,
            /* [out] */ __RPC__out MFCLOCK_PROPERTIES *pClockProperties);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, SetTimeSource)
        HRESULT ( STDMETHODCALLTYPE *SetTimeSource )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ __RPC__in_opt IMFPresentationTimeSource *pTimeSource);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, GetTimeSource)
        HRESULT ( STDMETHODCALLTYPE *GetTimeSource )( 
            __RPC__in IMFPresentationClock * This,
            /* [out] */ __RPC__deref_out_opt IMFPresentationTimeSource **ppTimeSource);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            __RPC__in IMFPresentationClock * This,
            /* [out] */ __RPC__out MFTIME *phnsClockTime);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, AddClockStateSink)
        HRESULT ( STDMETHODCALLTYPE *AddClockStateSink )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ __RPC__in_opt IMFClockStateSink *pStateSink);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, RemoveClockStateSink)
        HRESULT ( STDMETHODCALLTYPE *RemoveClockStateSink )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ __RPC__in_opt IMFClockStateSink *pStateSink);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            __RPC__in IMFPresentationClock * This,
            /* [in] */ LONGLONG llClockStartOffset);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            __RPC__in IMFPresentationClock * This);
        
        DECLSPEC_XFGVIRT(IMFPresentationClock, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            __RPC__in IMFPresentationClock * This);
        
        END_INTERFACE
    } IMFPresentationClockVtbl;

    interface IMFPresentationClock
    {
        CONST_VTBL struct IMFPresentationClockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPresentationClock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPresentationClock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPresentationClock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPresentationClock_GetClockCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetClockCharacteristics(This,pdwCharacteristics) ) 

#define IMFPresentationClock_GetCorrelatedTime(This,dwReserved,pllClockTime,phnsSystemTime)	\
    ( (This)->lpVtbl -> GetCorrelatedTime(This,dwReserved,pllClockTime,phnsSystemTime) ) 

#define IMFPresentationClock_GetContinuityKey(This,pdwContinuityKey)	\
    ( (This)->lpVtbl -> GetContinuityKey(This,pdwContinuityKey) ) 

#define IMFPresentationClock_GetState(This,dwReserved,peClockState)	\
    ( (This)->lpVtbl -> GetState(This,dwReserved,peClockState) ) 

#define IMFPresentationClock_GetProperties(This,pClockProperties)	\
    ( (This)->lpVtbl -> GetProperties(This,pClockProperties) ) 


#define IMFPresentationClock_SetTimeSource(This,pTimeSource)	\
    ( (This)->lpVtbl -> SetTimeSource(This,pTimeSource) ) 

#define IMFPresentationClock_GetTimeSource(This,ppTimeSource)	\
    ( (This)->lpVtbl -> GetTimeSource(This,ppTimeSource) ) 

#define IMFPresentationClock_GetTime(This,phnsClockTime)	\
    ( (This)->lpVtbl -> GetTime(This,phnsClockTime) ) 

#define IMFPresentationClock_AddClockStateSink(This,pStateSink)	\
    ( (This)->lpVtbl -> AddClockStateSink(This,pStateSink) ) 

#define IMFPresentationClock_RemoveClockStateSink(This,pStateSink)	\
    ( (This)->lpVtbl -> RemoveClockStateSink(This,pStateSink) ) 

#define IMFPresentationClock_Start(This,llClockStartOffset)	\
    ( (This)->lpVtbl -> Start(This,llClockStartOffset) ) 

#define IMFPresentationClock_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFPresentationClock_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPresentationClock_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0023 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region PC Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)
STDAPI MFCreatePresentationClock(
    _Outptr_ IMFPresentationClock** ppPresentationClock
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0023_v0_0_s_ifspec;

#ifndef __IMFPresentationTimeSource_INTERFACE_DEFINED__
#define __IMFPresentationTimeSource_INTERFACE_DEFINED__

/* interface IMFPresentationTimeSource */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFPresentationTimeSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7FF12CCE-F76F-41c2-863B-1666C8E5E139")
    IMFPresentationTimeSource : public IMFClock
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetUnderlyingClock( 
            /* [out] */ __RPC__deref_out_opt IMFClock **ppClock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPresentationTimeSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFPresentationTimeSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFPresentationTimeSource * This);
        
        DECLSPEC_XFGVIRT(IMFClock, GetClockCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetClockCharacteristics )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [out] */ __RPC__out DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFClock, GetCorrelatedTime)
        HRESULT ( STDMETHODCALLTYPE *GetCorrelatedTime )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out LONGLONG *pllClockTime,
            /* [out] */ __RPC__out MFTIME *phnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClock, GetContinuityKey)
        HRESULT ( STDMETHODCALLTYPE *GetContinuityKey )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [out] */ __RPC__out DWORD *pdwContinuityKey);
        
        DECLSPEC_XFGVIRT(IMFClock, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [in] */ DWORD dwReserved,
            /* [out] */ __RPC__out MFCLOCK_STATE *peClockState);
        
        DECLSPEC_XFGVIRT(IMFClock, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [out] */ __RPC__out MFCLOCK_PROPERTIES *pClockProperties);
        
        DECLSPEC_XFGVIRT(IMFPresentationTimeSource, GetUnderlyingClock)
        HRESULT ( STDMETHODCALLTYPE *GetUnderlyingClock )( 
            __RPC__in IMFPresentationTimeSource * This,
            /* [out] */ __RPC__deref_out_opt IMFClock **ppClock);
        
        END_INTERFACE
    } IMFPresentationTimeSourceVtbl;

    interface IMFPresentationTimeSource
    {
        CONST_VTBL struct IMFPresentationTimeSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPresentationTimeSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPresentationTimeSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPresentationTimeSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPresentationTimeSource_GetClockCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetClockCharacteristics(This,pdwCharacteristics) ) 

#define IMFPresentationTimeSource_GetCorrelatedTime(This,dwReserved,pllClockTime,phnsSystemTime)	\
    ( (This)->lpVtbl -> GetCorrelatedTime(This,dwReserved,pllClockTime,phnsSystemTime) ) 

#define IMFPresentationTimeSource_GetContinuityKey(This,pdwContinuityKey)	\
    ( (This)->lpVtbl -> GetContinuityKey(This,pdwContinuityKey) ) 

#define IMFPresentationTimeSource_GetState(This,dwReserved,peClockState)	\
    ( (This)->lpVtbl -> GetState(This,dwReserved,peClockState) ) 

#define IMFPresentationTimeSource_GetProperties(This,pClockProperties)	\
    ( (This)->lpVtbl -> GetProperties(This,pClockProperties) ) 


#define IMFPresentationTimeSource_GetUnderlyingClock(This,ppClock)	\
    ( (This)->lpVtbl -> GetUnderlyingClock(This,ppClock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPresentationTimeSource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0024 */
/* [local] */ 

STDAPI
MFCreateSystemTimeSource(
    _Outptr_ IMFPresentationTimeSource** ppSystemTimeSource
    );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0024_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0024_v0_0_s_ifspec;

#ifndef __IMFClockStateSink_INTERFACE_DEFINED__
#define __IMFClockStateSink_INTERFACE_DEFINED__

/* interface IMFClockStateSink */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFClockStateSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F6696E82-74F7-4f3d-A178-8A5E09C3659F")
    IMFClockStateSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnClockStart( 
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ LONGLONG llClockStartOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClockStop( 
            /* [in] */ MFTIME hnsSystemTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClockPause( 
            /* [in] */ MFTIME hnsSystemTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClockRestart( 
            /* [in] */ MFTIME hnsSystemTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClockSetRate( 
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ float flRate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFClockStateSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFClockStateSink * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFClockStateSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFClockStateSink * This);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStart)
        HRESULT ( STDMETHODCALLTYPE *OnClockStart )( 
            __RPC__in IMFClockStateSink * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ LONGLONG llClockStartOffset);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStop)
        HRESULT ( STDMETHODCALLTYPE *OnClockStop )( 
            __RPC__in IMFClockStateSink * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockPause)
        HRESULT ( STDMETHODCALLTYPE *OnClockPause )( 
            __RPC__in IMFClockStateSink * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockRestart)
        HRESULT ( STDMETHODCALLTYPE *OnClockRestart )( 
            __RPC__in IMFClockStateSink * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockSetRate)
        HRESULT ( STDMETHODCALLTYPE *OnClockSetRate )( 
            __RPC__in IMFClockStateSink * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ float flRate);
        
        END_INTERFACE
    } IMFClockStateSinkVtbl;

    interface IMFClockStateSink
    {
        CONST_VTBL struct IMFClockStateSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFClockStateSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFClockStateSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFClockStateSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFClockStateSink_OnClockStart(This,hnsSystemTime,llClockStartOffset)	\
    ( (This)->lpVtbl -> OnClockStart(This,hnsSystemTime,llClockStartOffset) ) 

#define IMFClockStateSink_OnClockStop(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockStop(This,hnsSystemTime) ) 

#define IMFClockStateSink_OnClockPause(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockPause(This,hnsSystemTime) ) 

#define IMFClockStateSink_OnClockRestart(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockRestart(This,hnsSystemTime) ) 

#define IMFClockStateSink_OnClockSetRate(This,hnsSystemTime,flRate)	\
    ( (This)->lpVtbl -> OnClockSetRate(This,hnsSystemTime,flRate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFClockStateSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0025 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MF_PD_PMPHOST_CONTEXT, 0x6c990d31, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_APP_CONTEXT, 0x6c990d32, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_GUID( MF_PD_DURATION, 0x6c990d33, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_TOTAL_FILE_SIZE, 0x6c990d34, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_AUDIO_ENCODING_BITRATE, 0x6c990d35, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_VIDEO_ENCODING_BITRATE, 0x6c990d36, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_MIME_TYPE, 0x6c990d37, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID(MF_PD_LAST_MODIFIED_TIME, 0x6c990d38, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID(MF_PD_PLAYBACK_ELEMENT_ID, 0x6c990d39, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_PREFERRED_LANGUAGE, 0x6c990d3A, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID(MF_PD_PLAYBACK_BOUNDARY_TIME, 0x6c990d3b, 0xbb8e, 0x477a, 0x85, 0x98, 0xd, 0x5d, 0x96, 0xfc, 0xd8, 0x8a );
EXTERN_GUID( MF_PD_AUDIO_ISVARIABLEBITRATE, 0x33026ee0, 0xe387, 0x4582, 0xae, 0x0a, 0x34, 0xa2, 0xad, 0x3b, 0xaa, 0x18 );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
DEFINE_GUID( MF_PD_ADAPTIVE_STREAMING, 0xEA0D5D97, 0x29F9, 0x488B, 0xAE, 0x6B, 0x7D, 0x6B, 0x41, 0x36, 0x11, 0x2B);
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0025_v0_0_s_ifspec;

#ifndef __IMFPresentationDescriptor_INTERFACE_DEFINED__
#define __IMFPresentationDescriptor_INTERFACE_DEFINED__

/* interface IMFPresentationDescriptor */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFPresentationDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03cb2711-24d7-4db6-a17f-f3a7a479a536")
    IMFPresentationDescriptor : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamDescriptorCount( 
            /* [out] */ __RPC__out DWORD *pdwDescriptorCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamDescriptorByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out BOOL *pfSelected,
            /* [out] */ __RPC__deref_out_opt IMFStreamDescriptor **ppDescriptor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectStream( 
            /* [in] */ DWORD dwDescriptorIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeselectStream( 
            /* [in] */ DWORD dwDescriptorIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ __RPC__deref_out_opt IMFPresentationDescriptor **ppPresentationDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPresentationDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFPresentationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFPresentationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFPresentationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFPresentationDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFPresentationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFPresentationDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFPresentationDescriptor * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFPresentationDescriptor, GetStreamDescriptorCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamDescriptorCount )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [out] */ __RPC__out DWORD *pdwDescriptorCount);
        
        DECLSPEC_XFGVIRT(IMFPresentationDescriptor, GetStreamDescriptorByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetStreamDescriptorByIndex )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ __RPC__out BOOL *pfSelected,
            /* [out] */ __RPC__deref_out_opt IMFStreamDescriptor **ppDescriptor);
        
        DECLSPEC_XFGVIRT(IMFPresentationDescriptor, SelectStream)
        HRESULT ( STDMETHODCALLTYPE *SelectStream )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [in] */ DWORD dwDescriptorIndex);
        
        DECLSPEC_XFGVIRT(IMFPresentationDescriptor, DeselectStream)
        HRESULT ( STDMETHODCALLTYPE *DeselectStream )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [in] */ DWORD dwDescriptorIndex);
        
        DECLSPEC_XFGVIRT(IMFPresentationDescriptor, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            __RPC__in IMFPresentationDescriptor * This,
            /* [out] */ __RPC__deref_out_opt IMFPresentationDescriptor **ppPresentationDescriptor);
        
        END_INTERFACE
    } IMFPresentationDescriptorVtbl;

    interface IMFPresentationDescriptor
    {
        CONST_VTBL struct IMFPresentationDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPresentationDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPresentationDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPresentationDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPresentationDescriptor_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFPresentationDescriptor_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFPresentationDescriptor_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFPresentationDescriptor_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFPresentationDescriptor_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFPresentationDescriptor_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFPresentationDescriptor_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFPresentationDescriptor_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFPresentationDescriptor_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFPresentationDescriptor_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFPresentationDescriptor_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFPresentationDescriptor_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFPresentationDescriptor_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFPresentationDescriptor_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFPresentationDescriptor_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFPresentationDescriptor_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFPresentationDescriptor_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFPresentationDescriptor_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFPresentationDescriptor_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFPresentationDescriptor_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFPresentationDescriptor_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFPresentationDescriptor_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFPresentationDescriptor_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFPresentationDescriptor_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFPresentationDescriptor_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFPresentationDescriptor_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFPresentationDescriptor_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFPresentationDescriptor_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFPresentationDescriptor_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFPresentationDescriptor_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFPresentationDescriptor_GetStreamDescriptorCount(This,pdwDescriptorCount)	\
    ( (This)->lpVtbl -> GetStreamDescriptorCount(This,pdwDescriptorCount) ) 

#define IMFPresentationDescriptor_GetStreamDescriptorByIndex(This,dwIndex,pfSelected,ppDescriptor)	\
    ( (This)->lpVtbl -> GetStreamDescriptorByIndex(This,dwIndex,pfSelected,ppDescriptor) ) 

#define IMFPresentationDescriptor_SelectStream(This,dwDescriptorIndex)	\
    ( (This)->lpVtbl -> SelectStream(This,dwDescriptorIndex) ) 

#define IMFPresentationDescriptor_DeselectStream(This,dwDescriptorIndex)	\
    ( (This)->lpVtbl -> DeselectStream(This,dwDescriptorIndex) ) 

#define IMFPresentationDescriptor_Clone(This,ppPresentationDescriptor)	\
    ( (This)->lpVtbl -> Clone(This,ppPresentationDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPresentationDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0026 */
/* [local] */ 

STDAPI MFCreatePresentationDescriptor(
    DWORD cStreamDescriptors,
    _In_reads_opt_( cStreamDescriptors ) IMFStreamDescriptor** apStreamDescriptors,
    _Outptr_ IMFPresentationDescriptor** ppPresentationDescriptor
    );

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
STDAPI MFRequireProtectedEnvironment(
     _In_ IMFPresentationDescriptor* pPresentationDescriptor
     );

STDAPI MFSerializePresentationDescriptor(
    _In_ IMFPresentationDescriptor * pPD,
    _Out_ DWORD * pcbData,
    _Outptr_result_bytebuffer_to_(*pcbData, *pcbData) BYTE ** ppbData);

STDAPI MFDeserializePresentationDescriptor(
    _In_ DWORD cbData,
    _In_reads_( cbData ) BYTE * pbData,
    _Outptr_ IMFPresentationDescriptor ** ppPD);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_GUID(MF_SD_LANGUAGE, 0xaf2180, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x3, 0x59, 0x3b, 0xc1, 0x21);
EXTERN_GUID(MF_SD_PROTECTED, 0xaf2181, 0xbdc2, 0x423c, 0xab, 0xca, 0xf5, 0x3, 0x59, 0x3b, 0xc1, 0x21);
EXTERN_GUID(MF_SD_STREAM_NAME, 0x4f1b099d, 0xd314, 0x41e5, 0xa7, 0x81, 0x7f, 0xef, 0xaa, 0x4c, 0x50, 0x1f);
EXTERN_GUID(MF_SD_MUTUALLY_EXCLUSIVE, 0x23ef79c, 0x388d, 0x487f, 0xac, 0x17, 0x69, 0x6c, 0xd6, 0xe3, 0xc6, 0xf5);
DEFINE_GUID( MF_SD_SUPPORTS_PROTECTED_CODEC_SWITCH, 0x8fb6b117, 0x862e, 0x4b31, 0x8d, 0xab, 0x5e, 0x0a, 0x43, 0x4c, 0xae, 0xf0);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0026_v0_0_s_ifspec;

#ifndef __IMFStreamDescriptor_INTERFACE_DEFINED__
#define __IMFStreamDescriptor_INTERFACE_DEFINED__

/* interface IMFStreamDescriptor */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFStreamDescriptor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56c03d9c-9dbb-45f5-ab4b-d80f47c05938")
    IMFStreamDescriptor : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStreamIdentifier( 
            /* [out] */ __RPC__out DWORD *pdwStreamIdentifier) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaTypeHandler( 
            /* [out] */ __RPC__deref_out_opt IMFMediaTypeHandler **ppMediaTypeHandler) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFStreamDescriptorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFStreamDescriptor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFStreamDescriptor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFStreamDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in_opt IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ __RPC__out BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cchBufSize) LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(( *pcchLength + 1 ) ) LPWSTR *ppwszValue,
            /* [out] */ __RPC__out UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [out] */ __RPC__out UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][out] */ __RPC__out_ecount_full(cbBufSize) UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ __RPC__inout_opt UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbSize) UINT8 **ppBuf,
            /* [out] */ __RPC__out UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            __RPC__in IMFStreamDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            __RPC__in REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [string][in] */ __RPC__in_string LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [size_is][in] */ __RPC__in_ecount_full(cbBufSize) const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            __RPC__in IMFStreamDescriptor * This,
            __RPC__in REFGUID guidKey,
            /* [in] */ __RPC__in_opt IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            __RPC__in IMFStreamDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            __RPC__in IMFStreamDescriptor * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            __RPC__in IMFStreamDescriptor * This,
            /* [out] */ __RPC__out UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            __RPC__in IMFStreamDescriptor * This,
            UINT32 unIndex,
            /* [out] */ __RPC__out GUID *pguidKey,
            /* [full][out][in] */ __RPC__inout_opt PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            __RPC__in IMFStreamDescriptor * This,
            /* [in] */ __RPC__in_opt IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFStreamDescriptor, GetStreamIdentifier)
        HRESULT ( STDMETHODCALLTYPE *GetStreamIdentifier )( 
            __RPC__in IMFStreamDescriptor * This,
            /* [out] */ __RPC__out DWORD *pdwStreamIdentifier);
        
        DECLSPEC_XFGVIRT(IMFStreamDescriptor, GetMediaTypeHandler)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTypeHandler )( 
            __RPC__in IMFStreamDescriptor * This,
            /* [out] */ __RPC__deref_out_opt IMFMediaTypeHandler **ppMediaTypeHandler);
        
        END_INTERFACE
    } IMFStreamDescriptorVtbl;

    interface IMFStreamDescriptor
    {
        CONST_VTBL struct IMFStreamDescriptorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFStreamDescriptor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFStreamDescriptor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFStreamDescriptor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFStreamDescriptor_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFStreamDescriptor_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFStreamDescriptor_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFStreamDescriptor_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFStreamDescriptor_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFStreamDescriptor_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFStreamDescriptor_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFStreamDescriptor_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFStreamDescriptor_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFStreamDescriptor_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFStreamDescriptor_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFStreamDescriptor_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFStreamDescriptor_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFStreamDescriptor_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFStreamDescriptor_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFStreamDescriptor_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFStreamDescriptor_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFStreamDescriptor_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFStreamDescriptor_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFStreamDescriptor_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFStreamDescriptor_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFStreamDescriptor_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFStreamDescriptor_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFStreamDescriptor_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFStreamDescriptor_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFStreamDescriptor_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFStreamDescriptor_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFStreamDescriptor_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFStreamDescriptor_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFStreamDescriptor_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFStreamDescriptor_GetStreamIdentifier(This,pdwStreamIdentifier)	\
    ( (This)->lpVtbl -> GetStreamIdentifier(This,pdwStreamIdentifier) ) 

#define IMFStreamDescriptor_GetMediaTypeHandler(This,ppMediaTypeHandler)	\
    ( (This)->lpVtbl -> GetMediaTypeHandler(This,ppMediaTypeHandler) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFStreamDescriptor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0027 */
/* [local] */ 

STDAPI MFCreateStreamDescriptor(
    DWORD dwStreamIdentifier,
    DWORD cMediaTypes,
    _In_reads_(cMediaTypes) IMFMediaType** apMediaTypes,
    _Outptr_ IMFStreamDescriptor** ppDescriptor
    );



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0027_v0_0_s_ifspec;

#ifndef __IMFMediaTypeHandler_INTERFACE_DEFINED__
#define __IMFMediaTypeHandler_INTERFACE_DEFINED__

/* interface IMFMediaTypeHandler */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaTypeHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e93dcf6c-4b07-4e1e-8123-aa16ed6eadf5")
    IMFMediaTypeHandler : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE IsMediaTypeSupported( 
            /* [in] */ IMFMediaType *pMediaType,
            /* [annotation][out] */ 
            _Outptr_opt_result_maybenull_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaTypeCount( 
            /* [out] */ __RPC__out DWORD *pdwTypeCount) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetMediaTypeByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE SetCurrentMediaType( 
            /* [in] */ IMFMediaType *pMediaType) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetCurrentMediaType( 
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMajorType( 
            /* [out] */ __RPC__out GUID *pguidMajorType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaTypeHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaTypeHandler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaTypeHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaTypeHandler * This);
        
        DECLSPEC_XFGVIRT(IMFMediaTypeHandler, IsMediaTypeSupported)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *IsMediaTypeSupported )( 
            IMFMediaTypeHandler * This,
            /* [in] */ IMFMediaType *pMediaType,
            /* [annotation][out] */ 
            _Outptr_opt_result_maybenull_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFMediaTypeHandler, GetMediaTypeCount)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTypeCount )( 
            __RPC__in IMFMediaTypeHandler * This,
            /* [out] */ __RPC__out DWORD *pdwTypeCount);
        
        DECLSPEC_XFGVIRT(IMFMediaTypeHandler, GetMediaTypeByIndex)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetMediaTypeByIndex )( 
            IMFMediaTypeHandler * This,
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppType);
        
        DECLSPEC_XFGVIRT(IMFMediaTypeHandler, SetCurrentMediaType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *SetCurrentMediaType )( 
            IMFMediaTypeHandler * This,
            /* [in] */ IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFMediaTypeHandler, GetCurrentMediaType)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetCurrentMediaType )( 
            IMFMediaTypeHandler * This,
            /* [annotation][out] */ 
            _Outptr_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFMediaTypeHandler, GetMajorType)
        HRESULT ( STDMETHODCALLTYPE *GetMajorType )( 
            __RPC__in IMFMediaTypeHandler * This,
            /* [out] */ __RPC__out GUID *pguidMajorType);
        
        END_INTERFACE
    } IMFMediaTypeHandlerVtbl;

    interface IMFMediaTypeHandler
    {
        CONST_VTBL struct IMFMediaTypeHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaTypeHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaTypeHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaTypeHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaTypeHandler_IsMediaTypeSupported(This,pMediaType,ppMediaType)	\
    ( (This)->lpVtbl -> IsMediaTypeSupported(This,pMediaType,ppMediaType) ) 

#define IMFMediaTypeHandler_GetMediaTypeCount(This,pdwTypeCount)	\
    ( (This)->lpVtbl -> GetMediaTypeCount(This,pdwTypeCount) ) 

#define IMFMediaTypeHandler_GetMediaTypeByIndex(This,dwIndex,ppType)	\
    ( (This)->lpVtbl -> GetMediaTypeByIndex(This,dwIndex,ppType) ) 

#define IMFMediaTypeHandler_SetCurrentMediaType(This,pMediaType)	\
    ( (This)->lpVtbl -> SetCurrentMediaType(This,pMediaType) ) 

#define IMFMediaTypeHandler_GetCurrentMediaType(This,ppMediaType)	\
    ( (This)->lpVtbl -> GetCurrentMediaType(This,ppMediaType) ) 

#define IMFMediaTypeHandler_GetMajorType(This,pguidMajorType)	\
    ( (This)->lpVtbl -> GetMajorType(This,pguidMajorType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_RemoteIsMediaTypeSupported_Proxy( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbBestMatch) BYTE **ppbBestMatch,
    /* [out] */ __RPC__out DWORD *pcbBestMatch);


void __RPC_STUB IMFMediaTypeHandler_RemoteIsMediaTypeSupported_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_RemoteGetMediaTypeByIndex_Proxy( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [in] */ DWORD dwIndex,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
    /* [out] */ __RPC__out DWORD *pcbData);


void __RPC_STUB IMFMediaTypeHandler_RemoteGetMediaTypeByIndex_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_RemoteSetCurrentMediaType_Proxy( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData);


void __RPC_STUB IMFMediaTypeHandler_RemoteSetCurrentMediaType_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_RemoteGetCurrentMediaType_Proxy( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
    /* [out] */ __RPC__out DWORD *pcbData);


void __RPC_STUB IMFMediaTypeHandler_RemoteGetCurrentMediaType_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFMediaTypeHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0028 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI MFCreateSimpleTypeHandler(
    _Outptr_ IMFMediaTypeHandler ** ppHandler );
typedef 
enum MFTIMER_FLAGS
    {
        MFTIMER_RELATIVE	= 0x1
    } 	MFTIMER_FLAGS;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0028_v0_0_s_ifspec;

#ifndef __IMFTimer_INTERFACE_DEFINED__
#define __IMFTimer_INTERFACE_DEFINED__

/* interface IMFTimer */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFTimer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e56e4cbd-8f70-49d8-a0f8-edb3d6ab9bf2")
    IMFTimer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetTimer( 
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONGLONG llClockTime,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState,
            /* [out] */ IUnknown **ppunkKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelTimer( 
            /* [in] */ IUnknown *punkKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimer * This);
        
        DECLSPEC_XFGVIRT(IMFTimer, SetTimer)
        HRESULT ( STDMETHODCALLTYPE *SetTimer )( 
            IMFTimer * This,
            /* [in] */ DWORD dwFlags,
            /* [in] */ LONGLONG llClockTime,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState,
            /* [out] */ IUnknown **ppunkKey);
        
        DECLSPEC_XFGVIRT(IMFTimer, CancelTimer)
        HRESULT ( STDMETHODCALLTYPE *CancelTimer )( 
            IMFTimer * This,
            /* [in] */ IUnknown *punkKey);
        
        END_INTERFACE
    } IMFTimerVtbl;

    interface IMFTimer
    {
        CONST_VTBL struct IMFTimerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimer_SetTimer(This,dwFlags,llClockTime,pCallback,punkState,ppunkKey)	\
    ( (This)->lpVtbl -> SetTimer(This,dwFlags,llClockTime,pCallback,punkState,ppunkKey) ) 

#define IMFTimer_CancelTimer(This,punkKey)	\
    ( (This)->lpVtbl -> CancelTimer(This,punkKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0029 */
/* [local] */ 

EXTERN_GUID( MF_ACTIVATE_CUSTOM_VIDEO_MIXER_CLSID,          0xba491360, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8 );
EXTERN_GUID( MF_ACTIVATE_CUSTOM_VIDEO_MIXER_ACTIVATE,       0xba491361, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8 );
EXTERN_GUID( MF_ACTIVATE_CUSTOM_VIDEO_MIXER_FLAGS,          0xba491362, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8 );
EXTERN_GUID( MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_CLSID,      0xba491364, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8 );
EXTERN_GUID( MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_ACTIVATE,   0xba491365, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8 );
EXTERN_GUID( MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_FLAGS,      0xba491366, 0xbe50, 0x451e, 0x95, 0xab, 0x6d, 0x4a, 0xcc, 0xc7, 0xda, 0xd8 );

enum __MIDL___MIDL_itf_mfidl_0000_0029_0001
    {
        MF_ACTIVATE_CUSTOM_MIXER_ALLOWFAIL	= 0x1
    } ;

enum __MIDL___MIDL_itf_mfidl_0000_0029_0002
    {
        MF_ACTIVATE_CUSTOM_PRESENTER_ALLOWFAIL	= 0x1
    } ;
EXTERN_GUID( MF_ACTIVATE_MFT_LOCKED,  0xc1f6093c, 0x7f65, 0x4fbd, 0x9e, 0x39, 0x5f, 0xae, 0xc3, 0xc4, 0xfb, 0xd7 );
EXTERN_GUID( MF_ACTIVATE_VIDEO_WINDOW, 0x9a2dbbdd, 0xf57e, 0x4162, 0x82, 0xb9, 0x68, 0x31, 0x37, 0x76, 0x82, 0xd3 );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum _MFSHUTDOWN_STATUS
    {
        MFSHUTDOWN_INITIATED	= 0,
        MFSHUTDOWN_COMPLETED	= ( MFSHUTDOWN_INITIATED + 1 ) 
    } 	MFSHUTDOWN_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0029_v0_0_s_ifspec;

#ifndef __IMFShutdown_INTERFACE_DEFINED__
#define __IMFShutdown_INTERFACE_DEFINED__

/* interface IMFShutdown */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFShutdown;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("97ec2ea4-0e42-4937-97ac-9d6d328824e1")
    IMFShutdown : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetShutdownStatus( 
            /* [out] */ __RPC__out MFSHUTDOWN_STATUS *pStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFShutdownVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFShutdown * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFShutdown * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFShutdown * This);
        
        DECLSPEC_XFGVIRT(IMFShutdown, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            __RPC__in IMFShutdown * This);
        
        DECLSPEC_XFGVIRT(IMFShutdown, GetShutdownStatus)
        HRESULT ( STDMETHODCALLTYPE *GetShutdownStatus )( 
            __RPC__in IMFShutdown * This,
            /* [out] */ __RPC__out MFSHUTDOWN_STATUS *pStatus);
        
        END_INTERFACE
    } IMFShutdownVtbl;

    interface IMFShutdown
    {
        CONST_VTBL struct IMFShutdownVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFShutdown_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFShutdown_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFShutdown_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFShutdown_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFShutdown_GetShutdownStatus(This,pStatus)	\
    ( (This)->lpVtbl -> GetShutdownStatus(This,pStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFShutdown_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0030 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI 
MFShutdownObject(
    IUnknown * pUnk );
STDAPI
MFCreateAudioRenderer(
    IMFAttributes* pAudioAttributes,
    _Outptr_ IMFMediaSink** ppSink
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region PC Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP)
STDAPI
MFCreateAudioRendererActivate( 
    _Outptr_ IMFActivate ** ppActivate 
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_PC_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS, 0xede4b5e0, 0xf805, 0x4d6c, 0x99, 0xb3, 0xdb, 0x01, 0xbf, 0x95, 0xdf, 0xab);
#define    MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_CROSSPROCESS          0x00000001
#define    MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_NOPERSIST          0x00000002
#if (WINVER >= _WIN32_WINNT_WIN7) 
#define    MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_DONT_ALLOW_FORMAT_CHANGES          0x00000004
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MF_AUDIO_RENDERER_ATTRIBUTE_SESSION_ID, 0xede4b5e3, 0xf805, 0x4d6c, 0x99, 0xb3, 0xdb, 0x01, 0xbf, 0x95, 0xdf, 0xab);
EXTERN_GUID( MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ID, 0xb10aaec3, 0xef71, 0x4cc3, 0xb8, 0x73, 0x5, 0xa9, 0xa0, 0x8b, 0x9f, 0x8e);
EXTERN_GUID( MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ROLE, 0x6ba644ff, 0x27c5, 0x4d02, 0x98, 0x87, 0xc2, 0x86, 0x19, 0xfd, 0xb9, 0x1b);
EXTERN_GUID( MF_AUDIO_RENDERER_ATTRIBUTE_STREAM_CATEGORY, 0xa9770471, 0x92ec, 0x4df4, 0x94, 0xfe, 0x81, 0xc3, 0x6f, 0xc, 0x3a, 0x7a);
STDAPI
MFCreateVideoRendererActivate(
    _In_ HWND hwndVideo, 
    _Outptr_ IMFActivate ** ppActivate 
    );
#if (WINVER >= _WIN32_WINNT_WIN7) 
STDAPI
MFCreateMPEG4MediaSink(
    _In_ IMFByteStream* pIByteStream, 
    _In_opt_ IMFMediaType* pVideoMediaType, 
    _In_opt_ IMFMediaType* pAudioMediaType, 
    _Outptr_ IMFMediaSink** ppIMediaSink 
    );
STDAPI
MFCreate3GPMediaSink(
    _In_ IMFByteStream* pIByteStream, 
    _In_opt_ IMFMediaType* pVideoMediaType, 
    _In_opt_ IMFMediaType* pAudioMediaType, 
    _Outptr_ IMFMediaSink** ppIMediaSink 
    );
STDAPI
MFCreateMP3MediaSink(
    _In_ IMFByteStream* pTargetByteStream, 
    _Outptr_ IMFMediaSink** ppMediaSink 
    );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
STDAPI
MFCreateAC3MediaSink(
    _In_ IMFByteStream* pTargetByteStream, 
    _In_ IMFMediaType* pAudioMediaType, 
     _Outptr_ IMFMediaSink** ppMediaSink 
    );
STDAPI
MFCreateADTSMediaSink(
    _In_ IMFByteStream* pTargetByteStream, 
    _In_ IMFMediaType* pAudioMediaType, 
     _Outptr_ IMFMediaSink** ppMediaSink 
    );
STDAPI
MFCreateMuxSink(
    _In_ GUID guidOutputSubType, 
    _In_opt_ IMFAttributes* pOutputAttributes, 
    _In_opt_ IMFByteStream* pOutputByteStream, 
    _Outptr_ IMFMediaSink** ppMuxSink 
    );
STDAPI
MFCreateFMPEG4MediaSink(
    _In_ IMFByteStream* pIByteStream, 
    _In_opt_ IMFMediaType* pVideoMediaType, 
    _In_opt_ IMFMediaType* pAudioMediaType, 
    _Outptr_ IMFMediaSink** ppIMediaSink 
    );
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#if (WINVER >= _WIN32_WINNT_WINBLUE) 
STDAPI
MFCreateAVIMediaSink(
    _In_ IMFByteStream* pIByteStream, 
    _In_ IMFMediaType* pVideoMediaType, 
    _In_opt_ IMFMediaType* pAudioMediaType, 
    _Outptr_ IMFMediaSink** ppIMediaSink 
    );
#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 
#if (WINVER >= _WIN32_WINNT_WINBLUE) 
STDAPI
MFCreateWAVEMediaSink(
    _In_ IMFByteStream* pTargetByteStream, 
    _In_ IMFMediaType* pAudioMediaType, 
     _Outptr_ IMFMediaSink** ppMediaSink 
    );
#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0030_v0_0_s_ifspec;

#ifndef __IMFTopoLoader_INTERFACE_DEFINED__
#define __IMFTopoLoader_INTERFACE_DEFINED__

/* interface IMFTopoLoader */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFTopoLoader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DE9A6157-F660-4643-B56A-DF9F7998C7CD")
    IMFTopoLoader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ IMFTopology *pInputTopo,
            /* [annotation][out] */ 
            _Outptr_  IMFTopology **ppOutputTopo,
            /* [in] */ IMFTopology *pCurrentTopo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTopoLoaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTopoLoader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTopoLoader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTopoLoader * This);
        
        DECLSPEC_XFGVIRT(IMFTopoLoader, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IMFTopoLoader * This,
            /* [in] */ IMFTopology *pInputTopo,
            /* [annotation][out] */ 
            _Outptr_  IMFTopology **ppOutputTopo,
            /* [in] */ IMFTopology *pCurrentTopo);
        
        END_INTERFACE
    } IMFTopoLoaderVtbl;

    interface IMFTopoLoader
    {
        CONST_VTBL struct IMFTopoLoaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTopoLoader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTopoLoader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTopoLoader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTopoLoader_Load(This,pInputTopo,ppOutputTopo,pCurrentTopo)	\
    ( (This)->lpVtbl -> Load(This,pInputTopo,ppOutputTopo,pCurrentTopo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTopoLoader_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0031 */
/* [local] */ 

STDAPI MFCreateTopoLoader(
    _Outptr_ IMFTopoLoader ** ppObj );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0031_v0_0_s_ifspec;

#ifndef __IMFContentProtectionManager_INTERFACE_DEFINED__
#define __IMFContentProtectionManager_INTERFACE_DEFINED__

/* interface IMFContentProtectionManager */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFContentProtectionManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ACF92459-6A61-42bd-B57C-B43E51203CB0")
    IMFContentProtectionManager : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginEnableContent( 
            /* [in] */ IMFActivate *pEnablerActivate,
            /* [in] */ IMFTopology *pTopo,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndEnableContent( 
            /* [in] */ IMFAsyncResult *pResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentProtectionManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFContentProtectionManager * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFContentProtectionManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFContentProtectionManager * This);
        
        DECLSPEC_XFGVIRT(IMFContentProtectionManager, BeginEnableContent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginEnableContent )( 
            IMFContentProtectionManager * This,
            /* [in] */ IMFActivate *pEnablerActivate,
            /* [in] */ IMFTopology *pTopo,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFContentProtectionManager, EndEnableContent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndEnableContent )( 
            IMFContentProtectionManager * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        END_INTERFACE
    } IMFContentProtectionManagerVtbl;

    interface IMFContentProtectionManager
    {
        CONST_VTBL struct IMFContentProtectionManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentProtectionManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentProtectionManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentProtectionManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentProtectionManager_BeginEnableContent(This,pEnablerActivate,pTopo,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginEnableContent(This,pEnablerActivate,pTopo,pCallback,punkState) ) 

#define IMFContentProtectionManager_EndEnableContent(This,pResult)	\
    ( (This)->lpVtbl -> EndEnableContent(This,pResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFContentProtectionManager_RemoteBeginEnableContent_Proxy( 
    __RPC__in IMFContentProtectionManager * This,
    /* [in] */ __RPC__in REFCLSID clsidType,
    /* [size_is][in] */ __RPC__in_ecount_full(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFContentProtectionManager_RemoteBeginEnableContent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFContentProtectionManager_RemoteEndEnableContent_Proxy( 
    __RPC__in IMFContentProtectionManager * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);


void __RPC_STUB IMFContentProtectionManager_RemoteEndEnableContent_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFContentProtectionManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0032 */
/* [local] */ 

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_mfidl_0000_0032_0001
    {
        MF_LICENSE_URL_UNTRUSTED	= 0,
        MF_LICENSE_URL_TRUSTED	= ( MF_LICENSE_URL_UNTRUSTED + 1 ) ,
        MF_LICENSE_URL_TAMPERED	= ( MF_LICENSE_URL_TRUSTED + 1 ) 
    } 	MF_URL_TRUST_STATUS;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0032_v0_0_s_ifspec;

#ifndef __IMFContentEnabler_INTERFACE_DEFINED__
#define __IMFContentEnabler_INTERFACE_DEFINED__

/* interface IMFContentEnabler */
/* [unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFContentEnabler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D3C4EF59-49CE-4381-9071-D5BCD044C770")
    IMFContentEnabler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetEnableType( 
            /* [out] */ __RPC__out GUID *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnableURL( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcchURL) LPWSTR *ppwszURL,
            /* [out] */ __RPC__out DWORD *pcchURL,
            /* [unique][out][in] */ __RPC__inout_opt MF_URL_TRUST_STATUS *pTrustStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEnableData( 
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
            /* [out] */ __RPC__out DWORD *pcbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAutomaticSupported( 
            /* [out] */ __RPC__out BOOL *pfAutomatic) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AutomaticEnable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MonitorEnable( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Cancel( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentEnablerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFContentEnabler * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFContentEnabler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFContentEnabler * This);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, GetEnableType)
        HRESULT ( STDMETHODCALLTYPE *GetEnableType )( 
            __RPC__in IMFContentEnabler * This,
            /* [out] */ __RPC__out GUID *pType);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, GetEnableURL)
        HRESULT ( STDMETHODCALLTYPE *GetEnableURL )( 
            __RPC__in IMFContentEnabler * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcchURL) LPWSTR *ppwszURL,
            /* [out] */ __RPC__out DWORD *pcchURL,
            /* [unique][out][in] */ __RPC__inout_opt MF_URL_TRUST_STATUS *pTrustStatus);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, GetEnableData)
        HRESULT ( STDMETHODCALLTYPE *GetEnableData )( 
            __RPC__in IMFContentEnabler * This,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
            /* [out] */ __RPC__out DWORD *pcbData);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, IsAutomaticSupported)
        HRESULT ( STDMETHODCALLTYPE *IsAutomaticSupported )( 
            __RPC__in IMFContentEnabler * This,
            /* [out] */ __RPC__out BOOL *pfAutomatic);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, AutomaticEnable)
        HRESULT ( STDMETHODCALLTYPE *AutomaticEnable )( 
            __RPC__in IMFContentEnabler * This);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, MonitorEnable)
        HRESULT ( STDMETHODCALLTYPE *MonitorEnable )( 
            __RPC__in IMFContentEnabler * This);
        
        DECLSPEC_XFGVIRT(IMFContentEnabler, Cancel)
        HRESULT ( STDMETHODCALLTYPE *Cancel )( 
            __RPC__in IMFContentEnabler * This);
        
        END_INTERFACE
    } IMFContentEnablerVtbl;

    interface IMFContentEnabler
    {
        CONST_VTBL struct IMFContentEnablerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentEnabler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentEnabler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentEnabler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentEnabler_GetEnableType(This,pType)	\
    ( (This)->lpVtbl -> GetEnableType(This,pType) ) 

#define IMFContentEnabler_GetEnableURL(This,ppwszURL,pcchURL,pTrustStatus)	\
    ( (This)->lpVtbl -> GetEnableURL(This,ppwszURL,pcchURL,pTrustStatus) ) 

#define IMFContentEnabler_GetEnableData(This,ppbData,pcbData)	\
    ( (This)->lpVtbl -> GetEnableData(This,ppbData,pcbData) ) 

#define IMFContentEnabler_IsAutomaticSupported(This,pfAutomatic)	\
    ( (This)->lpVtbl -> IsAutomaticSupported(This,pfAutomatic) ) 

#define IMFContentEnabler_AutomaticEnable(This)	\
    ( (This)->lpVtbl -> AutomaticEnable(This) ) 

#define IMFContentEnabler_MonitorEnable(This)	\
    ( (This)->lpVtbl -> MonitorEnable(This) ) 

#define IMFContentEnabler_Cancel(This)	\
    ( (This)->lpVtbl -> Cancel(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentEnabler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0033 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MFENABLETYPE_WMDRMV1_LicenseAcquisition, 0x4ff6eeaf, 0xb43, 0x4797, 0x9b, 0x85, 0xab, 0xf3, 0x18, 0x15, 0xe7, 0xb0);
EXTERN_GUID( MFENABLETYPE_WMDRMV7_LicenseAcquisition, 0x3306df, 0x4a06, 0x4884,0xa0, 0x97, 0xef, 0x6d, 0x22, 0xec, 0x84, 0xa3);
EXTERN_GUID( MFENABLETYPE_WMDRMV7_Individualization, 0xacd2c84a, 0xb303, 0x4f65, 0xbc, 0x2c, 0x2c, 0x84, 0x8d, 0x1, 0xa9, 0x89);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID( MFENABLETYPE_MF_UpdateRevocationInformation, 0xe558b0b5, 0xb3c4, 0x44a0, 0x92, 0x4c, 0x50, 0xd1, 0x78, 0x93, 0x23, 0x85);
EXTERN_GUID( MFENABLETYPE_MF_UpdateUntrustedComponent, 0x9879f3d6, 0xcee2, 0x48e6, 0xb5, 0x73, 0x97, 0x67, 0xab, 0x17, 0x2f, 0x16);
EXTERN_GUID( MFENABLETYPE_MF_RebootRequired, 0x6d4d3d4b, 0x0ece, 0x4652, 0x8b, 0x3a, 0xf2, 0xd2, 0x42, 0x60, 0xd8, 0x87);
// 
// Structs that contain information about revoked or unsigned binaries, 
// returned by the IMFContentEnabler::GetEnableData() method of  
// the Revocation content enabler 
// 
#ifndef MFRR_INFO_VERSION
#define MFRR_INFO_VERSION 0
#endif
// 
// The values for MFRR_COMPONENT_HASH_INFO.ulReason 
// 
#define MF_USER_MODE_COMPONENT_LOAD        0x00000001
#define MF_KERNEL_MODE_COMPONENT_LOAD      0x00000002
#define MF_GRL_LOAD_FAILED                 0x00000010
#define MF_INVALID_GRL_SIGNATURE           0x00000020
#define MF_GRL_ABSENT                      0x00001000
#define MF_COMPONENT_REVOKED               0x00002000
#define MF_COMPONENT_INVALID_EKU           0x00004000
#define MF_COMPONENT_CERT_REVOKED          0x00008000
#define MF_COMPONENT_INVALID_ROOT          0x00010000
#define MF_COMPONENT_HS_CERT_REVOKED       0x00020000
#define MF_COMPONENT_LS_CERT_REVOKED       0x00040000
#define MF_BOOT_DRIVER_VERIFICATION_FAILED 0x00100000
#define MF_TEST_SIGNED_COMPONENT_LOADING   0x01000000
#define MF_MINCRYPT_FAILURE                0x10000000
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
// 
// STR_HASH_LEN: Number of characters required to represent a SHA-1 hash  
// (RTL_MAX_HASH_LEN_V1) as a string of the form "0x5a3b53463b672a4f..." 
// Each byte of a SHA-1 hash takes two characters to represent, and 
// we add in two leading characters "0x" as well as the NULL terminator 
// 
#define SHA_HASH_LEN   20 
#define STR_HASH_LEN   (SHA_HASH_LEN*2 + 3) 
typedef struct _MFRR_COMPONENT_HASH_INFO 
{ 
    // Reason for failure (revoked or unsigned or badly signed).   
    DWORD ulReason; 
 
    // Header hash of the component 
    WCHAR rgHeaderHash[STR_HASH_LEN];  
 
    // Hash of public key if one of the certificates  
    // in the signing certificate chain is revoked 
    WCHAR rgPublicKeyHash[STR_HASH_LEN];  
 
    // Component name (full path name) 
    WCHAR wszName[MAX_PATH];     
 
}   MFRR_COMPONENT_HASH_INFO, *PMFRR_COMPONENT_HASH_INFO; 
typedef struct _MFRR_COMPONENTS 
{ 
 
    // Version number  
    DWORD dwRRInfoVersion; 
 
    // Number of components in list 
    DWORD dwRRComponents; 
 
    // points to the end of this structure that has  
    // allocated memory for the array of component info structures 
    PMFRR_COMPONENT_HASH_INFO pRRComponents;  
 
}   MFRR_COMPONENTS, *PMFRR_COMPONENTS; 
#pragma pack ( push ) 
#pragma pack ( 1 ) 
typedef struct _ASFFlatPicture 
{ 
    // 
    // Direct mapped fields 
    // 
    BYTE bPictureType; 
    DWORD dwDataLen; 
}   ASF_FLAT_PICTURE; 
#pragma pack ( pop ) 
#pragma pack ( push ) 
#pragma pack ( 1 ) 
typedef struct _ASFFlatSynchronisedLyrics 
{ 
    // 
    // Direct mapped fields 
    // 
    BYTE bTimeStampFormat; 
    BYTE bContentType; 
    DWORD dwLyricsLen; 
}   ASF_FLAT_SYNCHRONISED_LYRICS; 
#pragma pack ( pop ) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0033_v0_0_s_ifspec;

#ifndef __IMFMetadata_INTERFACE_DEFINED__
#define __IMFMetadata_INTERFACE_DEFINED__

/* interface IMFMetadata */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMetadata;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F88CFB8C-EF16-4991-B450-CB8C69E51704")
    IMFMetadata : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetLanguage( 
            /* [in] */ __RPC__in LPCWSTR pwszRFC1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetLanguage( 
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszRFC1766) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllLanguages( 
            /* [out] */ __RPC__out PROPVARIANT *ppvLanguages) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszName,
            /* [in] */ __RPC__in const PROPVARIANT *ppvValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszName,
            /* [out] */ __RPC__out PROPVARIANT *ppvValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteProperty( 
            /* [in] */ __RPC__in LPCWSTR pwszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllPropertyNames( 
            /* [out] */ __RPC__out PROPVARIANT *ppvNames) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMetadataVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMetadata * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMetadata * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMetadata * This);
        
        DECLSPEC_XFGVIRT(IMFMetadata, SetLanguage)
        HRESULT ( STDMETHODCALLTYPE *SetLanguage )( 
            __RPC__in IMFMetadata * This,
            /* [in] */ __RPC__in LPCWSTR pwszRFC1766);
        
        DECLSPEC_XFGVIRT(IMFMetadata, GetLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetLanguage )( 
            __RPC__in IMFMetadata * This,
            /* [out] */ __RPC__deref_out_opt LPWSTR *ppwszRFC1766);
        
        DECLSPEC_XFGVIRT(IMFMetadata, GetAllLanguages)
        HRESULT ( STDMETHODCALLTYPE *GetAllLanguages )( 
            __RPC__in IMFMetadata * This,
            /* [out] */ __RPC__out PROPVARIANT *ppvLanguages);
        
        DECLSPEC_XFGVIRT(IMFMetadata, SetProperty)
        HRESULT ( STDMETHODCALLTYPE *SetProperty )( 
            __RPC__in IMFMetadata * This,
            /* [in] */ __RPC__in LPCWSTR pwszName,
            /* [in] */ __RPC__in const PROPVARIANT *ppvValue);
        
        DECLSPEC_XFGVIRT(IMFMetadata, GetProperty)
        HRESULT ( STDMETHODCALLTYPE *GetProperty )( 
            __RPC__in IMFMetadata * This,
            /* [in] */ __RPC__in LPCWSTR pwszName,
            /* [out] */ __RPC__out PROPVARIANT *ppvValue);
        
        DECLSPEC_XFGVIRT(IMFMetadata, DeleteProperty)
        HRESULT ( STDMETHODCALLTYPE *DeleteProperty )( 
            __RPC__in IMFMetadata * This,
            /* [in] */ __RPC__in LPCWSTR pwszName);
        
        DECLSPEC_XFGVIRT(IMFMetadata, GetAllPropertyNames)
        HRESULT ( STDMETHODCALLTYPE *GetAllPropertyNames )( 
            __RPC__in IMFMetadata * This,
            /* [out] */ __RPC__out PROPVARIANT *ppvNames);
        
        END_INTERFACE
    } IMFMetadataVtbl;

    interface IMFMetadata
    {
        CONST_VTBL struct IMFMetadataVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMetadata_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMetadata_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMetadata_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMetadata_SetLanguage(This,pwszRFC1766)	\
    ( (This)->lpVtbl -> SetLanguage(This,pwszRFC1766) ) 

#define IMFMetadata_GetLanguage(This,ppwszRFC1766)	\
    ( (This)->lpVtbl -> GetLanguage(This,ppwszRFC1766) ) 

#define IMFMetadata_GetAllLanguages(This,ppvLanguages)	\
    ( (This)->lpVtbl -> GetAllLanguages(This,ppvLanguages) ) 

#define IMFMetadata_SetProperty(This,pwszName,ppvValue)	\
    ( (This)->lpVtbl -> SetProperty(This,pwszName,ppvValue) ) 

#define IMFMetadata_GetProperty(This,pwszName,ppvValue)	\
    ( (This)->lpVtbl -> GetProperty(This,pwszName,ppvValue) ) 

#define IMFMetadata_DeleteProperty(This,pwszName)	\
    ( (This)->lpVtbl -> DeleteProperty(This,pwszName) ) 

#define IMFMetadata_GetAllPropertyNames(This,ppvNames)	\
    ( (This)->lpVtbl -> GetAllPropertyNames(This,ppvNames) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMetadata_INTERFACE_DEFINED__ */


#ifndef __IMFMetadataProvider_INTERFACE_DEFINED__
#define __IMFMetadataProvider_INTERFACE_DEFINED__

/* interface IMFMetadataProvider */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMetadataProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56181D2D-E221-4adb-B1C8-3CEE6A53F76F")
    IMFMetadataProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMFMetadata( 
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [in] */ DWORD dwStreamIdentifier,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMetadata **ppMFMetadata) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMetadataProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMetadataProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMetadataProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMetadataProvider * This);
        
        DECLSPEC_XFGVIRT(IMFMetadataProvider, GetMFMetadata)
        HRESULT ( STDMETHODCALLTYPE *GetMFMetadata )( 
            __RPC__in IMFMetadataProvider * This,
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [in] */ DWORD dwStreamIdentifier,
            /* [in] */ DWORD dwFlags,
            /* [out] */ __RPC__deref_out_opt IMFMetadata **ppMFMetadata);
        
        END_INTERFACE
    } IMFMetadataProviderVtbl;

    interface IMFMetadataProvider
    {
        CONST_VTBL struct IMFMetadataProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMetadataProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMetadataProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMetadataProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMetadataProvider_GetMFMetadata(This,pPresentationDescriptor,dwStreamIdentifier,dwFlags,ppMFMetadata)	\
    ( (This)->lpVtbl -> GetMFMetadata(This,pPresentationDescriptor,dwStreamIdentifier,dwFlags,ppMFMetadata) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMetadataProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0035 */
/* [local] */ 

EXTERN_GUID( MF_METADATA_PROVIDER_SERVICE, 0xdb214084, 0x58a4, 0x4d2e, 0xb8, 0x4f, 0x6f, 0x75, 0x5b, 0x2f, 0x7a, 0xd);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MF_PROPERTY_HANDLER_SERVICE, 0xa3face02, 0x32b8, 0x41dd, 0x90, 0xe7, 0x5f, 0xef, 0x7c, 0x89, 0x91, 0xb5);
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum _MFRATE_DIRECTION
    {
        MFRATE_FORWARD	= 0,
        MFRATE_REVERSE	= ( MFRATE_FORWARD + 1 ) 
    } 	MFRATE_DIRECTION;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0035_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0035_v0_0_s_ifspec;

#ifndef __IMFRateSupport_INTERFACE_DEFINED__
#define __IMFRateSupport_INTERFACE_DEFINED__

/* interface IMFRateSupport */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFRateSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0a9ccdbc-d797-4563-9667-94ec5d79292d")
    IMFRateSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSlowestRate( 
            /* [in] */ MFRATE_DIRECTION eDirection,
            /* [in] */ BOOL fThin,
            /* [out] */ __RPC__out float *pflRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFastestRate( 
            /* [in] */ MFRATE_DIRECTION eDirection,
            /* [in] */ BOOL fThin,
            /* [out] */ __RPC__out float *pflRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsRateSupported( 
            /* [in] */ BOOL fThin,
            /* [in] */ float flRate,
            /* [unique][out][in] */ __RPC__inout_opt float *pflNearestSupportedRate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRateSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFRateSupport * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFRateSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFRateSupport * This);
        
        DECLSPEC_XFGVIRT(IMFRateSupport, GetSlowestRate)
        HRESULT ( STDMETHODCALLTYPE *GetSlowestRate )( 
            __RPC__in IMFRateSupport * This,
            /* [in] */ MFRATE_DIRECTION eDirection,
            /* [in] */ BOOL fThin,
            /* [out] */ __RPC__out float *pflRate);
        
        DECLSPEC_XFGVIRT(IMFRateSupport, GetFastestRate)
        HRESULT ( STDMETHODCALLTYPE *GetFastestRate )( 
            __RPC__in IMFRateSupport * This,
            /* [in] */ MFRATE_DIRECTION eDirection,
            /* [in] */ BOOL fThin,
            /* [out] */ __RPC__out float *pflRate);
        
        DECLSPEC_XFGVIRT(IMFRateSupport, IsRateSupported)
        HRESULT ( STDMETHODCALLTYPE *IsRateSupported )( 
            __RPC__in IMFRateSupport * This,
            /* [in] */ BOOL fThin,
            /* [in] */ float flRate,
            /* [unique][out][in] */ __RPC__inout_opt float *pflNearestSupportedRate);
        
        END_INTERFACE
    } IMFRateSupportVtbl;

    interface IMFRateSupport
    {
        CONST_VTBL struct IMFRateSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRateSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRateSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRateSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRateSupport_GetSlowestRate(This,eDirection,fThin,pflRate)	\
    ( (This)->lpVtbl -> GetSlowestRate(This,eDirection,fThin,pflRate) ) 

#define IMFRateSupport_GetFastestRate(This,eDirection,fThin,pflRate)	\
    ( (This)->lpVtbl -> GetFastestRate(This,eDirection,fThin,pflRate) ) 

#define IMFRateSupport_IsRateSupported(This,fThin,flRate,pflNearestSupportedRate)	\
    ( (This)->lpVtbl -> IsRateSupported(This,fThin,flRate,pflNearestSupportedRate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRateSupport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0036 */
/* [local] */ 

EXTERN_GUID( MF_RATE_CONTROL_SERVICE, 0x866fa297, 0xb802, 0x4bf8, 0x9d, 0xc9, 0x5e, 0x3b, 0x6a, 0x9f, 0x53, 0xc9);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0036_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0036_v0_0_s_ifspec;

#ifndef __IMFRateControl_INTERFACE_DEFINED__
#define __IMFRateControl_INTERFACE_DEFINED__

/* interface IMFRateControl */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFRateControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("88ddcd21-03c3-4275-91ed-55ee3929328f")
    IMFRateControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRate( 
            /* [in] */ BOOL fThin,
            /* [in] */ float flRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRate( 
            /* [unique][out][in] */ __RPC__inout_opt BOOL *pfThin,
            /* [unique][out][in] */ __RPC__inout_opt float *pflRate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRateControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFRateControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFRateControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFRateControl * This);
        
        DECLSPEC_XFGVIRT(IMFRateControl, SetRate)
        HRESULT ( STDMETHODCALLTYPE *SetRate )( 
            __RPC__in IMFRateControl * This,
            /* [in] */ BOOL fThin,
            /* [in] */ float flRate);
        
        DECLSPEC_XFGVIRT(IMFRateControl, GetRate)
        HRESULT ( STDMETHODCALLTYPE *GetRate )( 
            __RPC__in IMFRateControl * This,
            /* [unique][out][in] */ __RPC__inout_opt BOOL *pfThin,
            /* [unique][out][in] */ __RPC__inout_opt float *pflRate);
        
        END_INTERFACE
    } IMFRateControlVtbl;

    interface IMFRateControl
    {
        CONST_VTBL struct IMFRateControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRateControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRateControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRateControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRateControl_SetRate(This,fThin,flRate)	\
    ( (This)->lpVtbl -> SetRate(This,fThin,flRate) ) 

#define IMFRateControl_GetRate(This,pfThin,pflRate)	\
    ( (This)->lpVtbl -> GetRate(This,pfThin,pflRate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRateControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0037 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0037_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0037_v0_0_s_ifspec;

#ifndef __IMFTimecodeTranslate_INTERFACE_DEFINED__
#define __IMFTimecodeTranslate_INTERFACE_DEFINED__

/* interface IMFTimecodeTranslate */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFTimecodeTranslate;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ab9d8661-f7e8-4ef4-9861-89f334f94e74")
    IMFTimecodeTranslate : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginConvertTimecodeToHNS( 
            /* [in] */ const PROPVARIANT *pPropVarTimecode,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndConvertTimecodeToHNS( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [out] */ MFTIME *phnsTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginConvertHNSToTimecode( 
            /* [in] */ MFTIME hnsTime,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndConvertHNSToTimecode( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [out] */ PROPVARIANT *pPropVarTimecode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTimecodeTranslateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTimecodeTranslate * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTimecodeTranslate * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTimecodeTranslate * This);
        
        DECLSPEC_XFGVIRT(IMFTimecodeTranslate, BeginConvertTimecodeToHNS)
        HRESULT ( STDMETHODCALLTYPE *BeginConvertTimecodeToHNS )( 
            IMFTimecodeTranslate * This,
            /* [in] */ const PROPVARIANT *pPropVarTimecode,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFTimecodeTranslate, EndConvertTimecodeToHNS)
        HRESULT ( STDMETHODCALLTYPE *EndConvertTimecodeToHNS )( 
            IMFTimecodeTranslate * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [out] */ MFTIME *phnsTime);
        
        DECLSPEC_XFGVIRT(IMFTimecodeTranslate, BeginConvertHNSToTimecode)
        HRESULT ( STDMETHODCALLTYPE *BeginConvertHNSToTimecode )( 
            IMFTimecodeTranslate * This,
            /* [in] */ MFTIME hnsTime,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFTimecodeTranslate, EndConvertHNSToTimecode)
        HRESULT ( STDMETHODCALLTYPE *EndConvertHNSToTimecode )( 
            IMFTimecodeTranslate * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [out] */ PROPVARIANT *pPropVarTimecode);
        
        END_INTERFACE
    } IMFTimecodeTranslateVtbl;

    interface IMFTimecodeTranslate
    {
        CONST_VTBL struct IMFTimecodeTranslateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTimecodeTranslate_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTimecodeTranslate_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTimecodeTranslate_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTimecodeTranslate_BeginConvertTimecodeToHNS(This,pPropVarTimecode,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginConvertTimecodeToHNS(This,pPropVarTimecode,pCallback,punkState) ) 

#define IMFTimecodeTranslate_EndConvertTimecodeToHNS(This,pResult,phnsTime)	\
    ( (This)->lpVtbl -> EndConvertTimecodeToHNS(This,pResult,phnsTime) ) 

#define IMFTimecodeTranslate_BeginConvertHNSToTimecode(This,hnsTime,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginConvertHNSToTimecode(This,hnsTime,pCallback,punkState) ) 

#define IMFTimecodeTranslate_EndConvertHNSToTimecode(This,pResult,pPropVarTimecode)	\
    ( (This)->lpVtbl -> EndConvertHNSToTimecode(This,pResult,pPropVarTimecode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTimecodeTranslate_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0038 */
/* [local] */ 

EXTERN_GUID( MF_TIMECODE_SERVICE, 0xa0d502a7, 0x0eb3, 0x4885, 0xb1, 0xb9, 0x9f, 0xeb, 0x0d, 0x08, 0x34, 0x54 );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0038_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0038_v0_0_s_ifspec;

#ifndef __IMFSeekInfo_INTERFACE_DEFINED__
#define __IMFSeekInfo_INTERFACE_DEFINED__

/* interface IMFSeekInfo */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSeekInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("26AFEA53-D9ED-42B5-AB80-E64F9EE34779")
    IMFSeekInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNearestKeyFrames( 
            /* [annotation][in] */ 
            _In_  const GUID *pguidTimeFormat,
            /* [annotation][in] */ 
            _In_  const PROPVARIANT *pvarStartPosition,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarPreviousKeyFrame,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarNextKeyFrame) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSeekInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSeekInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSeekInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSeekInfo * This);
        
        DECLSPEC_XFGVIRT(IMFSeekInfo, GetNearestKeyFrames)
        HRESULT ( STDMETHODCALLTYPE *GetNearestKeyFrames )( 
            IMFSeekInfo * This,
            /* [annotation][in] */ 
            _In_  const GUID *pguidTimeFormat,
            /* [annotation][in] */ 
            _In_  const PROPVARIANT *pvarStartPosition,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarPreviousKeyFrame,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pvarNextKeyFrame);
        
        END_INTERFACE
    } IMFSeekInfoVtbl;

    interface IMFSeekInfo
    {
        CONST_VTBL struct IMFSeekInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSeekInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSeekInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSeekInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSeekInfo_GetNearestKeyFrames(This,pguidTimeFormat,pvarStartPosition,pvarPreviousKeyFrame,pvarNextKeyFrame)	\
    ( (This)->lpVtbl -> GetNearestKeyFrames(This,pguidTimeFormat,pvarStartPosition,pvarPreviousKeyFrame,pvarNextKeyFrame) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSeekInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0039 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_C const GUID MF_SCRUBBING_SERVICE;
#endif // (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0039_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0039_v0_0_s_ifspec;

#ifndef __IMFSimpleAudioVolume_INTERFACE_DEFINED__
#define __IMFSimpleAudioVolume_INTERFACE_DEFINED__

/* interface IMFSimpleAudioVolume */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFSimpleAudioVolume;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("089EDF13-CF71-4338-8D13-9E569DBDC319")
    IMFSimpleAudioVolume : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMasterVolume( 
            /* [in] */ float fLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMasterVolume( 
            /* [out] */ __RPC__out float *pfLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMute( 
            /* [in] */ const BOOL bMute) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMute( 
            /* [out] */ __RPC__out BOOL *pbMute) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSimpleAudioVolumeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFSimpleAudioVolume * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFSimpleAudioVolume * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFSimpleAudioVolume * This);
        
        DECLSPEC_XFGVIRT(IMFSimpleAudioVolume, SetMasterVolume)
        HRESULT ( STDMETHODCALLTYPE *SetMasterVolume )( 
            __RPC__in IMFSimpleAudioVolume * This,
            /* [in] */ float fLevel);
        
        DECLSPEC_XFGVIRT(IMFSimpleAudioVolume, GetMasterVolume)
        HRESULT ( STDMETHODCALLTYPE *GetMasterVolume )( 
            __RPC__in IMFSimpleAudioVolume * This,
            /* [out] */ __RPC__out float *pfLevel);
        
        DECLSPEC_XFGVIRT(IMFSimpleAudioVolume, SetMute)
        HRESULT ( STDMETHODCALLTYPE *SetMute )( 
            __RPC__in IMFSimpleAudioVolume * This,
            /* [in] */ const BOOL bMute);
        
        DECLSPEC_XFGVIRT(IMFSimpleAudioVolume, GetMute)
        HRESULT ( STDMETHODCALLTYPE *GetMute )( 
            __RPC__in IMFSimpleAudioVolume * This,
            /* [out] */ __RPC__out BOOL *pbMute);
        
        END_INTERFACE
    } IMFSimpleAudioVolumeVtbl;

    interface IMFSimpleAudioVolume
    {
        CONST_VTBL struct IMFSimpleAudioVolumeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSimpleAudioVolume_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSimpleAudioVolume_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSimpleAudioVolume_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSimpleAudioVolume_SetMasterVolume(This,fLevel)	\
    ( (This)->lpVtbl -> SetMasterVolume(This,fLevel) ) 

#define IMFSimpleAudioVolume_GetMasterVolume(This,pfLevel)	\
    ( (This)->lpVtbl -> GetMasterVolume(This,pfLevel) ) 

#define IMFSimpleAudioVolume_SetMute(This,bMute)	\
    ( (This)->lpVtbl -> SetMute(This,bMute) ) 

#define IMFSimpleAudioVolume_GetMute(This,pbMute)	\
    ( (This)->lpVtbl -> GetMute(This,pbMute) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSimpleAudioVolume_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0040 */
/* [local] */ 

EXTERN_GUID( MR_POLICY_VOLUME_SERVICE, 0x1abaa2ac, 0x9d3b, 0x47c6, 0xab, 0x48, 0xc5, 0x95, 0x6, 0xde, 0x78, 0x4d);
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MR_CAPTURE_POLICY_VOLUME_SERVICE, 0x24030acd, 0x107a, 0x4265, 0x97, 0x5c, 0x41, 0x4e, 0x33, 0xe6, 0x5f, 0x2a);
#endif // (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0040_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0040_v0_0_s_ifspec;

#ifndef __IMFAudioStreamVolume_INTERFACE_DEFINED__
#define __IMFAudioStreamVolume_INTERFACE_DEFINED__

/* interface IMFAudioStreamVolume */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFAudioStreamVolume;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("76B1BBDB-4EC8-4f36-B106-70A9316DF593")
    IMFAudioStreamVolume : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetChannelCount( 
            /* [out] */ __RPC__out UINT32 *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetChannelVolume( 
            /* [in] */ UINT32 dwIndex,
            /* [in] */ const float fLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChannelVolume( 
            /* [in] */ UINT32 dwIndex,
            /* [out] */ __RPC__out float *pfLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllVolumes( 
            /* [in] */ UINT32 dwCount,
            /* [size_is][in] */ __RPC__in_ecount_full(dwCount) const float *pfVolumes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllVolumes( 
            /* [in] */ UINT32 dwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(dwCount) float *pfVolumes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAudioStreamVolumeVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFAudioStreamVolume * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFAudioStreamVolume * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFAudioStreamVolume * This);
        
        DECLSPEC_XFGVIRT(IMFAudioStreamVolume, GetChannelCount)
        HRESULT ( STDMETHODCALLTYPE *GetChannelCount )( 
            __RPC__in IMFAudioStreamVolume * This,
            /* [out] */ __RPC__out UINT32 *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFAudioStreamVolume, SetChannelVolume)
        HRESULT ( STDMETHODCALLTYPE *SetChannelVolume )( 
            __RPC__in IMFAudioStreamVolume * This,
            /* [in] */ UINT32 dwIndex,
            /* [in] */ const float fLevel);
        
        DECLSPEC_XFGVIRT(IMFAudioStreamVolume, GetChannelVolume)
        HRESULT ( STDMETHODCALLTYPE *GetChannelVolume )( 
            __RPC__in IMFAudioStreamVolume * This,
            /* [in] */ UINT32 dwIndex,
            /* [out] */ __RPC__out float *pfLevel);
        
        DECLSPEC_XFGVIRT(IMFAudioStreamVolume, SetAllVolumes)
        HRESULT ( STDMETHODCALLTYPE *SetAllVolumes )( 
            __RPC__in IMFAudioStreamVolume * This,
            /* [in] */ UINT32 dwCount,
            /* [size_is][in] */ __RPC__in_ecount_full(dwCount) const float *pfVolumes);
        
        DECLSPEC_XFGVIRT(IMFAudioStreamVolume, GetAllVolumes)
        HRESULT ( STDMETHODCALLTYPE *GetAllVolumes )( 
            __RPC__in IMFAudioStreamVolume * This,
            /* [in] */ UINT32 dwCount,
            /* [size_is][out] */ __RPC__out_ecount_full(dwCount) float *pfVolumes);
        
        END_INTERFACE
    } IMFAudioStreamVolumeVtbl;

    interface IMFAudioStreamVolume
    {
        CONST_VTBL struct IMFAudioStreamVolumeVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAudioStreamVolume_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAudioStreamVolume_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAudioStreamVolume_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAudioStreamVolume_GetChannelCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetChannelCount(This,pdwCount) ) 

#define IMFAudioStreamVolume_SetChannelVolume(This,dwIndex,fLevel)	\
    ( (This)->lpVtbl -> SetChannelVolume(This,dwIndex,fLevel) ) 

#define IMFAudioStreamVolume_GetChannelVolume(This,dwIndex,pfLevel)	\
    ( (This)->lpVtbl -> GetChannelVolume(This,dwIndex,pfLevel) ) 

#define IMFAudioStreamVolume_SetAllVolumes(This,dwCount,pfVolumes)	\
    ( (This)->lpVtbl -> SetAllVolumes(This,dwCount,pfVolumes) ) 

#define IMFAudioStreamVolume_GetAllVolumes(This,dwCount,pfVolumes)	\
    ( (This)->lpVtbl -> GetAllVolumes(This,dwCount,pfVolumes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAudioStreamVolume_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0041 */
/* [local] */ 

EXTERN_GUID( MR_STREAM_VOLUME_SERVICE, 0xf8b5fa2f, 0x32ef, 0x46f5, 0xb1, 0x72, 0x13, 0x21, 0x21, 0x2f, 0xb2, 0xc4);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0041_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0041_v0_0_s_ifspec;

#ifndef __IMFAudioPolicy_INTERFACE_DEFINED__
#define __IMFAudioPolicy_INTERFACE_DEFINED__

/* interface IMFAudioPolicy */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFAudioPolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a0638c2b-6465-4395-9ae7-a321a9fd2856")
    IMFAudioPolicy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetGroupingParam( 
            /* [in] */ REFGUID rguidClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetGroupingParam( 
            /* [annotation][out] */ 
            _Out_  GUID *pguidClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDisplayName( 
            /* [in] */ LPCWSTR pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisplayName( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *pszName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetIconPath( 
            /* [in] */ LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIconPath( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *pszPath) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFAudioPolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFAudioPolicy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFAudioPolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFAudioPolicy * This);
        
        DECLSPEC_XFGVIRT(IMFAudioPolicy, SetGroupingParam)
        HRESULT ( STDMETHODCALLTYPE *SetGroupingParam )( 
            IMFAudioPolicy * This,
            /* [in] */ REFGUID rguidClass);
        
        DECLSPEC_XFGVIRT(IMFAudioPolicy, GetGroupingParam)
        HRESULT ( STDMETHODCALLTYPE *GetGroupingParam )( 
            IMFAudioPolicy * This,
            /* [annotation][out] */ 
            _Out_  GUID *pguidClass);
        
        DECLSPEC_XFGVIRT(IMFAudioPolicy, SetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *SetDisplayName )( 
            IMFAudioPolicy * This,
            /* [in] */ LPCWSTR pszName);
        
        DECLSPEC_XFGVIRT(IMFAudioPolicy, GetDisplayName)
        HRESULT ( STDMETHODCALLTYPE *GetDisplayName )( 
            IMFAudioPolicy * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *pszName);
        
        DECLSPEC_XFGVIRT(IMFAudioPolicy, SetIconPath)
        HRESULT ( STDMETHODCALLTYPE *SetIconPath )( 
            IMFAudioPolicy * This,
            /* [in] */ LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IMFAudioPolicy, GetIconPath)
        HRESULT ( STDMETHODCALLTYPE *GetIconPath )( 
            IMFAudioPolicy * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *pszPath);
        
        END_INTERFACE
    } IMFAudioPolicyVtbl;

    interface IMFAudioPolicy
    {
        CONST_VTBL struct IMFAudioPolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFAudioPolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFAudioPolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFAudioPolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFAudioPolicy_SetGroupingParam(This,rguidClass)	\
    ( (This)->lpVtbl -> SetGroupingParam(This,rguidClass) ) 

#define IMFAudioPolicy_GetGroupingParam(This,pguidClass)	\
    ( (This)->lpVtbl -> GetGroupingParam(This,pguidClass) ) 

#define IMFAudioPolicy_SetDisplayName(This,pszName)	\
    ( (This)->lpVtbl -> SetDisplayName(This,pszName) ) 

#define IMFAudioPolicy_GetDisplayName(This,pszName)	\
    ( (This)->lpVtbl -> GetDisplayName(This,pszName) ) 

#define IMFAudioPolicy_SetIconPath(This,pszPath)	\
    ( (This)->lpVtbl -> SetIconPath(This,pszPath) ) 

#define IMFAudioPolicy_GetIconPath(This,pszPath)	\
    ( (This)->lpVtbl -> GetIconPath(This,pszPath) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFAudioPolicy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0042 */
/* [local] */ 

EXTERN_GUID( MR_AUDIO_POLICY_SERVICE, 0x911fd737, 0x6775, 0x4ab0, 0xa6, 0x14, 0x29, 0x78, 0x62, 0xfd, 0xac, 0x88);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0042_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0042_v0_0_s_ifspec;

#ifndef __IMFSampleGrabberSinkCallback_INTERFACE_DEFINED__
#define __IMFSampleGrabberSinkCallback_INTERFACE_DEFINED__

/* interface IMFSampleGrabberSinkCallback */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSampleGrabberSinkCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8C7B80BF-EE42-4b59-B1DF-55668E1BDCA8")
    IMFSampleGrabberSinkCallback : public IMFClockStateSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSetPresentationClock( 
            /* [in] */ IMFPresentationClock *pPresentationClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnProcessSample( 
            /* [in] */ REFGUID guidMajorMediaType,
            /* [in] */ DWORD dwSampleFlags,
            /* [in] */ LONGLONG llSampleTime,
            /* [in] */ LONGLONG llSampleDuration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSampleSize)  const BYTE *pSampleBuffer,
            /* [in] */ DWORD dwSampleSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnShutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSampleGrabberSinkCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSampleGrabberSinkCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSampleGrabberSinkCallback * This);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStart)
        HRESULT ( STDMETHODCALLTYPE *OnClockStart )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ LONGLONG llClockStartOffset);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStop)
        HRESULT ( STDMETHODCALLTYPE *OnClockStop )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockPause)
        HRESULT ( STDMETHODCALLTYPE *OnClockPause )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockRestart)
        HRESULT ( STDMETHODCALLTYPE *OnClockRestart )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockSetRate)
        HRESULT ( STDMETHODCALLTYPE *OnClockSetRate )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ float flRate);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback, OnSetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *OnSetPresentationClock )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ IMFPresentationClock *pPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback, OnProcessSample)
        HRESULT ( STDMETHODCALLTYPE *OnProcessSample )( 
            IMFSampleGrabberSinkCallback * This,
            /* [in] */ REFGUID guidMajorMediaType,
            /* [in] */ DWORD dwSampleFlags,
            /* [in] */ LONGLONG llSampleTime,
            /* [in] */ LONGLONG llSampleDuration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSampleSize)  const BYTE *pSampleBuffer,
            /* [in] */ DWORD dwSampleSize);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback, OnShutdown)
        HRESULT ( STDMETHODCALLTYPE *OnShutdown )( 
            IMFSampleGrabberSinkCallback * This);
        
        END_INTERFACE
    } IMFSampleGrabberSinkCallbackVtbl;

    interface IMFSampleGrabberSinkCallback
    {
        CONST_VTBL struct IMFSampleGrabberSinkCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSampleGrabberSinkCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSampleGrabberSinkCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSampleGrabberSinkCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSampleGrabberSinkCallback_OnClockStart(This,hnsSystemTime,llClockStartOffset)	\
    ( (This)->lpVtbl -> OnClockStart(This,hnsSystemTime,llClockStartOffset) ) 

#define IMFSampleGrabberSinkCallback_OnClockStop(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockStop(This,hnsSystemTime) ) 

#define IMFSampleGrabberSinkCallback_OnClockPause(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockPause(This,hnsSystemTime) ) 

#define IMFSampleGrabberSinkCallback_OnClockRestart(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockRestart(This,hnsSystemTime) ) 

#define IMFSampleGrabberSinkCallback_OnClockSetRate(This,hnsSystemTime,flRate)	\
    ( (This)->lpVtbl -> OnClockSetRate(This,hnsSystemTime,flRate) ) 


#define IMFSampleGrabberSinkCallback_OnSetPresentationClock(This,pPresentationClock)	\
    ( (This)->lpVtbl -> OnSetPresentationClock(This,pPresentationClock) ) 

#define IMFSampleGrabberSinkCallback_OnProcessSample(This,guidMajorMediaType,dwSampleFlags,llSampleTime,llSampleDuration,pSampleBuffer,dwSampleSize)	\
    ( (This)->lpVtbl -> OnProcessSample(This,guidMajorMediaType,dwSampleFlags,llSampleTime,llSampleDuration,pSampleBuffer,dwSampleSize) ) 

#define IMFSampleGrabberSinkCallback_OnShutdown(This)	\
    ( (This)->lpVtbl -> OnShutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSampleGrabberSinkCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0043 */
/* [local] */ 

STDAPI
MFCreateSampleGrabberSinkActivate(
    IMFMediaType *pIMFMediaType,
    IMFSampleGrabberSinkCallback* pIMFSampleGrabberSinkCallback,
    _Outptr_ IMFActivate** ppIActivate
    );
EXTERN_GUID( MF_SAMPLEGRABBERSINK_SAMPLE_TIME_OFFSET, 0x62e3d776, 0x8100, 0x4e03, 0xa6, 0xe8, 0xbd, 0x38, 0x57, 0xac, 0x9c, 0x47);
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MF_SAMPLEGRABBERSINK_IGNORE_CLOCK, 0x0efda2c0, 0x2b69, 0x4e2e, 0xab, 0x8d, 0x46, 0xdc, 0xbf, 0xf7, 0xd2, 0x5d);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0043_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0043_v0_0_s_ifspec;

#ifndef __IMFSampleGrabberSinkCallback2_INTERFACE_DEFINED__
#define __IMFSampleGrabberSinkCallback2_INTERFACE_DEFINED__

/* interface IMFSampleGrabberSinkCallback2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSampleGrabberSinkCallback2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ca86aa50-c46e-429e-ab27-16d6ac6844cb")
    IMFSampleGrabberSinkCallback2 : public IMFSampleGrabberSinkCallback
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnProcessSampleEx( 
            /* [in] */ REFGUID guidMajorMediaType,
            /* [in] */ DWORD dwSampleFlags,
            /* [in] */ LONGLONG llSampleTime,
            /* [in] */ LONGLONG llSampleDuration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSampleSize)  const BYTE *pSampleBuffer,
            /* [in] */ DWORD dwSampleSize,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttributes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSampleGrabberSinkCallback2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSampleGrabberSinkCallback2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSampleGrabberSinkCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStart)
        HRESULT ( STDMETHODCALLTYPE *OnClockStart )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ LONGLONG llClockStartOffset);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockStop)
        HRESULT ( STDMETHODCALLTYPE *OnClockStop )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockPause)
        HRESULT ( STDMETHODCALLTYPE *OnClockPause )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockRestart)
        HRESULT ( STDMETHODCALLTYPE *OnClockRestart )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ MFTIME hnsSystemTime);
        
        DECLSPEC_XFGVIRT(IMFClockStateSink, OnClockSetRate)
        HRESULT ( STDMETHODCALLTYPE *OnClockSetRate )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ MFTIME hnsSystemTime,
            /* [in] */ float flRate);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback, OnSetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *OnSetPresentationClock )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ IMFPresentationClock *pPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback, OnProcessSample)
        HRESULT ( STDMETHODCALLTYPE *OnProcessSample )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ REFGUID guidMajorMediaType,
            /* [in] */ DWORD dwSampleFlags,
            /* [in] */ LONGLONG llSampleTime,
            /* [in] */ LONGLONG llSampleDuration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSampleSize)  const BYTE *pSampleBuffer,
            /* [in] */ DWORD dwSampleSize);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback, OnShutdown)
        HRESULT ( STDMETHODCALLTYPE *OnShutdown )( 
            IMFSampleGrabberSinkCallback2 * This);
        
        DECLSPEC_XFGVIRT(IMFSampleGrabberSinkCallback2, OnProcessSampleEx)
        HRESULT ( STDMETHODCALLTYPE *OnProcessSampleEx )( 
            IMFSampleGrabberSinkCallback2 * This,
            /* [in] */ REFGUID guidMajorMediaType,
            /* [in] */ DWORD dwSampleFlags,
            /* [in] */ LONGLONG llSampleTime,
            /* [in] */ LONGLONG llSampleDuration,
            /* [annotation][in] */ 
            _In_reads_bytes_(dwSampleSize)  const BYTE *pSampleBuffer,
            /* [in] */ DWORD dwSampleSize,
            /* [annotation][in] */ 
            _In_  IMFAttributes *pAttributes);
        
        END_INTERFACE
    } IMFSampleGrabberSinkCallback2Vtbl;

    interface IMFSampleGrabberSinkCallback2
    {
        CONST_VTBL struct IMFSampleGrabberSinkCallback2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSampleGrabberSinkCallback2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSampleGrabberSinkCallback2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSampleGrabberSinkCallback2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSampleGrabberSinkCallback2_OnClockStart(This,hnsSystemTime,llClockStartOffset)	\
    ( (This)->lpVtbl -> OnClockStart(This,hnsSystemTime,llClockStartOffset) ) 

#define IMFSampleGrabberSinkCallback2_OnClockStop(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockStop(This,hnsSystemTime) ) 

#define IMFSampleGrabberSinkCallback2_OnClockPause(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockPause(This,hnsSystemTime) ) 

#define IMFSampleGrabberSinkCallback2_OnClockRestart(This,hnsSystemTime)	\
    ( (This)->lpVtbl -> OnClockRestart(This,hnsSystemTime) ) 

#define IMFSampleGrabberSinkCallback2_OnClockSetRate(This,hnsSystemTime,flRate)	\
    ( (This)->lpVtbl -> OnClockSetRate(This,hnsSystemTime,flRate) ) 


#define IMFSampleGrabberSinkCallback2_OnSetPresentationClock(This,pPresentationClock)	\
    ( (This)->lpVtbl -> OnSetPresentationClock(This,pPresentationClock) ) 

#define IMFSampleGrabberSinkCallback2_OnProcessSample(This,guidMajorMediaType,dwSampleFlags,llSampleTime,llSampleDuration,pSampleBuffer,dwSampleSize)	\
    ( (This)->lpVtbl -> OnProcessSample(This,guidMajorMediaType,dwSampleFlags,llSampleTime,llSampleDuration,pSampleBuffer,dwSampleSize) ) 

#define IMFSampleGrabberSinkCallback2_OnShutdown(This)	\
    ( (This)->lpVtbl -> OnShutdown(This) ) 


#define IMFSampleGrabberSinkCallback2_OnProcessSampleEx(This,guidMajorMediaType,dwSampleFlags,llSampleTime,llSampleDuration,pSampleBuffer,dwSampleSize,pAttributes)	\
    ( (This)->lpVtbl -> OnProcessSampleEx(This,guidMajorMediaType,dwSampleFlags,llSampleTime,llSampleDuration,pSampleBuffer,dwSampleSize,pAttributes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSampleGrabberSinkCallback2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0044 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID( MF_QUALITY_SERVICES, 0xb7e2be11, 0x2f96, 0x4640, 0xb5, 0x2c, 0x28, 0x23, 0x65, 0xbd, 0xf1, 0x6c);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0044_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0044_v0_0_s_ifspec;

#ifndef __IMFWorkQueueServices_INTERFACE_DEFINED__
#define __IMFWorkQueueServices_INTERFACE_DEFINED__

/* interface IMFWorkQueueServices */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFWorkQueueServices;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("35FE1BB8-A3A9-40fe-BBEC-EB569C9CCCA3")
    IMFWorkQueueServices : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginRegisterTopologyWorkQueuesWithMMCSS( 
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndRegisterTopologyWorkQueuesWithMMCSS( 
            /* [in] */ IMFAsyncResult *pResult) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginUnregisterTopologyWorkQueuesWithMMCSS( 
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndUnregisterTopologyWorkQueuesWithMMCSS( 
            /* [in] */ IMFAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTopologyWorkQueueMMCSSClass( 
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcchClass) LPWSTR pwszClass,
            /* [out][in] */ __RPC__inout DWORD *pcchClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTopologyWorkQueueMMCSSTaskId( 
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [out] */ __RPC__out DWORD *pdwTaskId) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginRegisterPlatformWorkQueueWithMMCSS( 
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ LPCWSTR wszClass,
            /* [in] */ DWORD dwTaskId,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndRegisterPlatformWorkQueueWithMMCSS( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTaskId) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginUnregisterPlatformWorkQueueWithMMCSS( 
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE EndUnregisterPlatformWorkQueueWithMMCSS( 
            /* [in] */ IMFAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlaftormWorkQueueMMCSSClass( 
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcchClass) LPWSTR pwszClass,
            /* [out][in] */ __RPC__inout DWORD *pcchClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlatformWorkQueueMMCSSTaskId( 
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [out] */ __RPC__out DWORD *pdwTaskId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFWorkQueueServicesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFWorkQueueServices * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFWorkQueueServices * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFWorkQueueServices * This);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginRegisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginRegisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndRegisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndRegisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginUnregisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginUnregisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndUnregisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndUnregisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetTopologyWorkQueueMMCSSClass)
        HRESULT ( STDMETHODCALLTYPE *GetTopologyWorkQueueMMCSSClass )( 
            __RPC__in IMFWorkQueueServices * This,
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcchClass) LPWSTR pwszClass,
            /* [out][in] */ __RPC__inout DWORD *pcchClass);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetTopologyWorkQueueMMCSSTaskId)
        HRESULT ( STDMETHODCALLTYPE *GetTopologyWorkQueueMMCSSTaskId )( 
            __RPC__in IMFWorkQueueServices * This,
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [out] */ __RPC__out DWORD *pdwTaskId);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginRegisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginRegisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ LPCWSTR wszClass,
            /* [in] */ DWORD dwTaskId,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndRegisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndRegisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTaskId);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginUnregisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginUnregisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndUnregisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndUnregisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServices * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetPlaftormWorkQueueMMCSSClass)
        HRESULT ( STDMETHODCALLTYPE *GetPlaftormWorkQueueMMCSSClass )( 
            __RPC__in IMFWorkQueueServices * This,
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcchClass) LPWSTR pwszClass,
            /* [out][in] */ __RPC__inout DWORD *pcchClass);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetPlatformWorkQueueMMCSSTaskId)
        HRESULT ( STDMETHODCALLTYPE *GetPlatformWorkQueueMMCSSTaskId )( 
            __RPC__in IMFWorkQueueServices * This,
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [out] */ __RPC__out DWORD *pdwTaskId);
        
        END_INTERFACE
    } IMFWorkQueueServicesVtbl;

    interface IMFWorkQueueServices
    {
        CONST_VTBL struct IMFWorkQueueServicesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFWorkQueueServices_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFWorkQueueServices_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFWorkQueueServices_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFWorkQueueServices_BeginRegisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginRegisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState) ) 

#define IMFWorkQueueServices_EndRegisterTopologyWorkQueuesWithMMCSS(This,pResult)	\
    ( (This)->lpVtbl -> EndRegisterTopologyWorkQueuesWithMMCSS(This,pResult) ) 

#define IMFWorkQueueServices_BeginUnregisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginUnregisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState) ) 

#define IMFWorkQueueServices_EndUnregisterTopologyWorkQueuesWithMMCSS(This,pResult)	\
    ( (This)->lpVtbl -> EndUnregisterTopologyWorkQueuesWithMMCSS(This,pResult) ) 

#define IMFWorkQueueServices_GetTopologyWorkQueueMMCSSClass(This,dwTopologyWorkQueueId,pwszClass,pcchClass)	\
    ( (This)->lpVtbl -> GetTopologyWorkQueueMMCSSClass(This,dwTopologyWorkQueueId,pwszClass,pcchClass) ) 

#define IMFWorkQueueServices_GetTopologyWorkQueueMMCSSTaskId(This,dwTopologyWorkQueueId,pdwTaskId)	\
    ( (This)->lpVtbl -> GetTopologyWorkQueueMMCSSTaskId(This,dwTopologyWorkQueueId,pdwTaskId) ) 

#define IMFWorkQueueServices_BeginRegisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,wszClass,dwTaskId,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginRegisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,wszClass,dwTaskId,pCallback,pState) ) 

#define IMFWorkQueueServices_EndRegisterPlatformWorkQueueWithMMCSS(This,pResult,pdwTaskId)	\
    ( (This)->lpVtbl -> EndRegisterPlatformWorkQueueWithMMCSS(This,pResult,pdwTaskId) ) 

#define IMFWorkQueueServices_BeginUnregisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginUnregisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,pCallback,pState) ) 

#define IMFWorkQueueServices_EndUnregisterPlatformWorkQueueWithMMCSS(This,pResult)	\
    ( (This)->lpVtbl -> EndUnregisterPlatformWorkQueueWithMMCSS(This,pResult) ) 

#define IMFWorkQueueServices_GetPlaftormWorkQueueMMCSSClass(This,dwPlatformWorkQueueId,pwszClass,pcchClass)	\
    ( (This)->lpVtbl -> GetPlaftormWorkQueueMMCSSClass(This,dwPlatformWorkQueueId,pwszClass,pcchClass) ) 

#define IMFWorkQueueServices_GetPlatformWorkQueueMMCSSTaskId(This,dwPlatformWorkQueueId,pdwTaskId)	\
    ( (This)->lpVtbl -> GetPlatformWorkQueueMMCSSTaskId(This,dwPlatformWorkQueueId,pdwTaskId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteBeginRegisterTopologyWorkQueuesWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFWorkQueueServices_RemoteBeginRegisterTopologyWorkQueuesWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteEndRegisterTopologyWorkQueuesWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);


void __RPC_STUB IMFWorkQueueServices_RemoteEndRegisterTopologyWorkQueuesWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteBeginUnregisterTopologyWorkQueuesWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFWorkQueueServices_RemoteBeginUnregisterTopologyWorkQueuesWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteEndUnregisterTopologyWorkQueuesWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);


void __RPC_STUB IMFWorkQueueServices_RemoteEndUnregisterTopologyWorkQueuesWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteBeginRegisterPlatformWorkQueueWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ __RPC__in LPCWSTR wszClass,
    /* [in] */ DWORD dwTaskId,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFWorkQueueServices_RemoteBeginRegisterPlatformWorkQueueWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteEndRegisterPlatformWorkQueueWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out DWORD *pdwTaskId);


void __RPC_STUB IMFWorkQueueServices_RemoteEndRegisterPlatformWorkQueueWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteBeginUnregisterPlatformWorkQueueWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFWorkQueueServices_RemoteBeginUnregisterPlatformWorkQueueWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_RemoteEndUnregisterPlatformWorkQueueWithMMCSS_Proxy( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);


void __RPC_STUB IMFWorkQueueServices_RemoteEndUnregisterPlatformWorkQueueWithMMCSS_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFWorkQueueServices_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0045 */
/* [local] */ 

EXTERN_GUID( MF_WORKQUEUE_SERVICES, 0x8e37d489, 0x41e0, 0x413a, 0x90, 0x68, 0x28, 0x7c, 0x88, 0x6d, 0x8d, 0xda);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0045_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0045_v0_0_s_ifspec;

#ifndef __IMFWorkQueueServicesEx_INTERFACE_DEFINED__
#define __IMFWorkQueueServicesEx_INTERFACE_DEFINED__

/* interface IMFWorkQueueServicesEx */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFWorkQueueServicesEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("96bf961b-40fe-42f1-ba9d-320238b49700")
    IMFWorkQueueServicesEx : public IMFWorkQueueServices
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTopologyWorkQueueMMCSSPriority( 
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [out] */ __RPC__out LONG *plPriority) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BeginRegisterPlatformWorkQueueWithMMCSSEx( 
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ LPCWSTR wszClass,
            /* [in] */ DWORD dwTaskId,
            /* [in] */ LONG lPriority,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlatformWorkQueueMMCSSPriority( 
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [out] */ __RPC__out LONG *plPriority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFWorkQueueServicesExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFWorkQueueServicesEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFWorkQueueServicesEx * This);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginRegisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginRegisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndRegisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndRegisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginUnregisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginUnregisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndUnregisterTopologyWorkQueuesWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndUnregisterTopologyWorkQueuesWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetTopologyWorkQueueMMCSSClass)
        HRESULT ( STDMETHODCALLTYPE *GetTopologyWorkQueueMMCSSClass )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcchClass) LPWSTR pwszClass,
            /* [out][in] */ __RPC__inout DWORD *pcchClass);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetTopologyWorkQueueMMCSSTaskId)
        HRESULT ( STDMETHODCALLTYPE *GetTopologyWorkQueueMMCSSTaskId )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [out] */ __RPC__out DWORD *pdwTaskId);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginRegisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginRegisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ LPCWSTR wszClass,
            /* [in] */ DWORD dwTaskId,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndRegisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndRegisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTaskId);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, BeginUnregisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginUnregisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, EndUnregisterPlatformWorkQueueWithMMCSS)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndUnregisterPlatformWorkQueueWithMMCSS )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetPlaftormWorkQueueMMCSSClass)
        HRESULT ( STDMETHODCALLTYPE *GetPlaftormWorkQueueMMCSSClass )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [size_is][out] */ __RPC__out_ecount_full(*pcchClass) LPWSTR pwszClass,
            /* [out][in] */ __RPC__inout DWORD *pcchClass);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServices, GetPlatformWorkQueueMMCSSTaskId)
        HRESULT ( STDMETHODCALLTYPE *GetPlatformWorkQueueMMCSSTaskId )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [out] */ __RPC__out DWORD *pdwTaskId);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServicesEx, GetTopologyWorkQueueMMCSSPriority)
        HRESULT ( STDMETHODCALLTYPE *GetTopologyWorkQueueMMCSSPriority )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwTopologyWorkQueueId,
            /* [out] */ __RPC__out LONG *plPriority);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServicesEx, BeginRegisterPlatformWorkQueueWithMMCSSEx)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginRegisterPlatformWorkQueueWithMMCSSEx )( 
            IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwPlatformWorkQueue,
            /* [in] */ LPCWSTR wszClass,
            /* [in] */ DWORD dwTaskId,
            /* [in] */ LONG lPriority,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFWorkQueueServicesEx, GetPlatformWorkQueueMMCSSPriority)
        HRESULT ( STDMETHODCALLTYPE *GetPlatformWorkQueueMMCSSPriority )( 
            __RPC__in IMFWorkQueueServicesEx * This,
            /* [in] */ DWORD dwPlatformWorkQueueId,
            /* [out] */ __RPC__out LONG *plPriority);
        
        END_INTERFACE
    } IMFWorkQueueServicesExVtbl;

    interface IMFWorkQueueServicesEx
    {
        CONST_VTBL struct IMFWorkQueueServicesExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFWorkQueueServicesEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFWorkQueueServicesEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFWorkQueueServicesEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFWorkQueueServicesEx_BeginRegisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginRegisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState) ) 

#define IMFWorkQueueServicesEx_EndRegisterTopologyWorkQueuesWithMMCSS(This,pResult)	\
    ( (This)->lpVtbl -> EndRegisterTopologyWorkQueuesWithMMCSS(This,pResult) ) 

#define IMFWorkQueueServicesEx_BeginUnregisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginUnregisterTopologyWorkQueuesWithMMCSS(This,pCallback,pState) ) 

#define IMFWorkQueueServicesEx_EndUnregisterTopologyWorkQueuesWithMMCSS(This,pResult)	\
    ( (This)->lpVtbl -> EndUnregisterTopologyWorkQueuesWithMMCSS(This,pResult) ) 

#define IMFWorkQueueServicesEx_GetTopologyWorkQueueMMCSSClass(This,dwTopologyWorkQueueId,pwszClass,pcchClass)	\
    ( (This)->lpVtbl -> GetTopologyWorkQueueMMCSSClass(This,dwTopologyWorkQueueId,pwszClass,pcchClass) ) 

#define IMFWorkQueueServicesEx_GetTopologyWorkQueueMMCSSTaskId(This,dwTopologyWorkQueueId,pdwTaskId)	\
    ( (This)->lpVtbl -> GetTopologyWorkQueueMMCSSTaskId(This,dwTopologyWorkQueueId,pdwTaskId) ) 

#define IMFWorkQueueServicesEx_BeginRegisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,wszClass,dwTaskId,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginRegisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,wszClass,dwTaskId,pCallback,pState) ) 

#define IMFWorkQueueServicesEx_EndRegisterPlatformWorkQueueWithMMCSS(This,pResult,pdwTaskId)	\
    ( (This)->lpVtbl -> EndRegisterPlatformWorkQueueWithMMCSS(This,pResult,pdwTaskId) ) 

#define IMFWorkQueueServicesEx_BeginUnregisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginUnregisterPlatformWorkQueueWithMMCSS(This,dwPlatformWorkQueue,pCallback,pState) ) 

#define IMFWorkQueueServicesEx_EndUnregisterPlatformWorkQueueWithMMCSS(This,pResult)	\
    ( (This)->lpVtbl -> EndUnregisterPlatformWorkQueueWithMMCSS(This,pResult) ) 

#define IMFWorkQueueServicesEx_GetPlaftormWorkQueueMMCSSClass(This,dwPlatformWorkQueueId,pwszClass,pcchClass)	\
    ( (This)->lpVtbl -> GetPlaftormWorkQueueMMCSSClass(This,dwPlatformWorkQueueId,pwszClass,pcchClass) ) 

#define IMFWorkQueueServicesEx_GetPlatformWorkQueueMMCSSTaskId(This,dwPlatformWorkQueueId,pdwTaskId)	\
    ( (This)->lpVtbl -> GetPlatformWorkQueueMMCSSTaskId(This,dwPlatformWorkQueueId,pdwTaskId) ) 


#define IMFWorkQueueServicesEx_GetTopologyWorkQueueMMCSSPriority(This,dwTopologyWorkQueueId,plPriority)	\
    ( (This)->lpVtbl -> GetTopologyWorkQueueMMCSSPriority(This,dwTopologyWorkQueueId,plPriority) ) 

#define IMFWorkQueueServicesEx_BeginRegisterPlatformWorkQueueWithMMCSSEx(This,dwPlatformWorkQueue,wszClass,dwTaskId,lPriority,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginRegisterPlatformWorkQueueWithMMCSSEx(This,dwPlatformWorkQueue,wszClass,dwTaskId,lPriority,pCallback,pState) ) 

#define IMFWorkQueueServicesEx_GetPlatformWorkQueueMMCSSPriority(This,dwPlatformWorkQueueId,plPriority)	\
    ( (This)->lpVtbl -> GetPlatformWorkQueueMMCSSPriority(This,dwPlatformWorkQueueId,plPriority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServicesEx_RemoteBeginRegisterPlatformWorkQueueWithMMCSSEx_Proxy( 
    __RPC__in IMFWorkQueueServicesEx * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ __RPC__in LPCWSTR wszClass,
    /* [in] */ DWORD dwTaskId,
    /* [in] */ LONG lPriority,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);


void __RPC_STUB IMFWorkQueueServicesEx_RemoteBeginRegisterPlatformWorkQueueWithMMCSSEx_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFWorkQueueServicesEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0046 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum _MF_QUALITY_DROP_MODE
    {
        MF_DROP_MODE_NONE	= 0,
        MF_DROP_MODE_1	= 0x1,
        MF_DROP_MODE_2	= 0x2,
        MF_DROP_MODE_3	= 0x3,
        MF_DROP_MODE_4	= 0x4,
        MF_DROP_MODE_5	= 0x5,
        MF_NUM_DROP_MODES	= 0x6
    } 	MF_QUALITY_DROP_MODE;

typedef 
enum _MF_QUALITY_LEVEL
    {
        MF_QUALITY_NORMAL	= 0,
        MF_QUALITY_NORMAL_MINUS_1	= 0x1,
        MF_QUALITY_NORMAL_MINUS_2	= 0x2,
        MF_QUALITY_NORMAL_MINUS_3	= 0x3,
        MF_QUALITY_NORMAL_MINUS_4	= 0x4,
        MF_QUALITY_NORMAL_MINUS_5	= 0x5,
        MF_NUM_QUALITY_LEVELS	= 0x6
    } 	MF_QUALITY_LEVEL;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum _MF_QUALITY_ADVISE_FLAGS
    {
        MF_QUALITY_CANNOT_KEEP_UP	= 0x1
    } 	MF_QUALITY_ADVISE_FLAGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0046_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0046_v0_0_s_ifspec;

#ifndef __IMFQualityManager_INTERFACE_DEFINED__
#define __IMFQualityManager_INTERFACE_DEFINED__

/* interface IMFQualityManager */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFQualityManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8D009D86-5B9F-4115-B1FC-9F80D52AB8AB")
    IMFQualityManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyTopology( 
            /* [in] */ IMFTopology *pTopology) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyPresentationClock( 
            /* [in] */ IMFPresentationClock *pClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyProcessInput( 
            /* [in] */ IMFTopologyNode *pNode,
            /* [in] */ long lInputIndex,
            /* [in] */ IMFSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyProcessOutput( 
            /* [in] */ IMFTopologyNode *pNode,
            /* [in] */ long lOutputIndex,
            /* [in] */ IMFSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyQualityEvent( 
            /* [in] */ IUnknown *pObject,
            /* [in] */ IMFMediaEvent *pEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFQualityManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFQualityManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFQualityManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFQualityManager * This);
        
        DECLSPEC_XFGVIRT(IMFQualityManager, NotifyTopology)
        HRESULT ( STDMETHODCALLTYPE *NotifyTopology )( 
            IMFQualityManager * This,
            /* [in] */ IMFTopology *pTopology);
        
        DECLSPEC_XFGVIRT(IMFQualityManager, NotifyPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *NotifyPresentationClock )( 
            IMFQualityManager * This,
            /* [in] */ IMFPresentationClock *pClock);
        
        DECLSPEC_XFGVIRT(IMFQualityManager, NotifyProcessInput)
        HRESULT ( STDMETHODCALLTYPE *NotifyProcessInput )( 
            IMFQualityManager * This,
            /* [in] */ IMFTopologyNode *pNode,
            /* [in] */ long lInputIndex,
            /* [in] */ IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFQualityManager, NotifyProcessOutput)
        HRESULT ( STDMETHODCALLTYPE *NotifyProcessOutput )( 
            IMFQualityManager * This,
            /* [in] */ IMFTopologyNode *pNode,
            /* [in] */ long lOutputIndex,
            /* [in] */ IMFSample *pSample);
        
        DECLSPEC_XFGVIRT(IMFQualityManager, NotifyQualityEvent)
        HRESULT ( STDMETHODCALLTYPE *NotifyQualityEvent )( 
            IMFQualityManager * This,
            /* [in] */ IUnknown *pObject,
            /* [in] */ IMFMediaEvent *pEvent);
        
        DECLSPEC_XFGVIRT(IMFQualityManager, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFQualityManager * This);
        
        END_INTERFACE
    } IMFQualityManagerVtbl;

    interface IMFQualityManager
    {
        CONST_VTBL struct IMFQualityManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFQualityManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFQualityManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFQualityManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFQualityManager_NotifyTopology(This,pTopology)	\
    ( (This)->lpVtbl -> NotifyTopology(This,pTopology) ) 

#define IMFQualityManager_NotifyPresentationClock(This,pClock)	\
    ( (This)->lpVtbl -> NotifyPresentationClock(This,pClock) ) 

#define IMFQualityManager_NotifyProcessInput(This,pNode,lInputIndex,pSample)	\
    ( (This)->lpVtbl -> NotifyProcessInput(This,pNode,lInputIndex,pSample) ) 

#define IMFQualityManager_NotifyProcessOutput(This,pNode,lOutputIndex,pSample)	\
    ( (This)->lpVtbl -> NotifyProcessOutput(This,pNode,lOutputIndex,pSample) ) 

#define IMFQualityManager_NotifyQualityEvent(This,pObject,pEvent)	\
    ( (This)->lpVtbl -> NotifyQualityEvent(This,pObject,pEvent) ) 

#define IMFQualityManager_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFQualityManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0047 */
/* [local] */ 

STDAPI MFCreateStandardQualityManager(
    _Outptr_ IMFQualityManager **ppQualityManager );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID( MF_QUALITY_NOTIFY_PROCESSING_LATENCY, 0xf6b44af8, 0x604d, 0x46fe, 0xa9, 0x5d, 0x45, 0x47, 0x9b, 0x10, 0xc9, 0xbc );
EXTERN_GUID( MF_QUALITY_NOTIFY_SAMPLE_LAG, 0x30d15206, 0xed2a, 0x4760, 0xbe, 0x17, 0xeb, 0x4a, 0x9f, 0x12, 0x29, 0x5c );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0047_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0047_v0_0_s_ifspec;

#ifndef __IMFQualityAdvise_INTERFACE_DEFINED__
#define __IMFQualityAdvise_INTERFACE_DEFINED__

/* interface IMFQualityAdvise */
/* [uuid][object] */ 






EXTERN_C const IID IID_IMFQualityAdvise;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EC15E2E9-E36B-4f7c-8758-77D452EF4CE7")
    IMFQualityAdvise : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDropMode( 
            /* [in] */ MF_QUALITY_DROP_MODE eDropMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetQualityLevel( 
            /* [in] */ MF_QUALITY_LEVEL eQualityLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDropMode( 
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_DROP_MODE *peDropMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetQualityLevel( 
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_LEVEL *peQualityLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DropTime( 
            /* [in] */ LONGLONG hnsAmountToDrop) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFQualityAdviseVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFQualityAdvise * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFQualityAdvise * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFQualityAdvise * This);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, SetDropMode)
        HRESULT ( STDMETHODCALLTYPE *SetDropMode )( 
            __RPC__in IMFQualityAdvise * This,
            /* [in] */ MF_QUALITY_DROP_MODE eDropMode);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, SetQualityLevel)
        HRESULT ( STDMETHODCALLTYPE *SetQualityLevel )( 
            __RPC__in IMFQualityAdvise * This,
            /* [in] */ MF_QUALITY_LEVEL eQualityLevel);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, GetDropMode)
        HRESULT ( STDMETHODCALLTYPE *GetDropMode )( 
            __RPC__in IMFQualityAdvise * This,
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_DROP_MODE *peDropMode);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, GetQualityLevel)
        HRESULT ( STDMETHODCALLTYPE *GetQualityLevel )( 
            __RPC__in IMFQualityAdvise * This,
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_LEVEL *peQualityLevel);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, DropTime)
        HRESULT ( STDMETHODCALLTYPE *DropTime )( 
            __RPC__in IMFQualityAdvise * This,
            /* [in] */ LONGLONG hnsAmountToDrop);
        
        END_INTERFACE
    } IMFQualityAdviseVtbl;

    interface IMFQualityAdvise
    {
        CONST_VTBL struct IMFQualityAdviseVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFQualityAdvise_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFQualityAdvise_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFQualityAdvise_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFQualityAdvise_SetDropMode(This,eDropMode)	\
    ( (This)->lpVtbl -> SetDropMode(This,eDropMode) ) 

#define IMFQualityAdvise_SetQualityLevel(This,eQualityLevel)	\
    ( (This)->lpVtbl -> SetQualityLevel(This,eQualityLevel) ) 

#define IMFQualityAdvise_GetDropMode(This,peDropMode)	\
    ( (This)->lpVtbl -> GetDropMode(This,peDropMode) ) 

#define IMFQualityAdvise_GetQualityLevel(This,peQualityLevel)	\
    ( (This)->lpVtbl -> GetQualityLevel(This,peQualityLevel) ) 

#define IMFQualityAdvise_DropTime(This,hnsAmountToDrop)	\
    ( (This)->lpVtbl -> DropTime(This,hnsAmountToDrop) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFQualityAdvise_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0048 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0048_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0048_v0_0_s_ifspec;

#ifndef __IMFQualityAdvise2_INTERFACE_DEFINED__
#define __IMFQualityAdvise2_INTERFACE_DEFINED__

/* interface IMFQualityAdvise2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFQualityAdvise2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F3706F0D-8EA2-4886-8000-7155E9EC2EAE")
    IMFQualityAdvise2 : public IMFQualityAdvise
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyQualityEvent( 
            /* [in] */ __RPC__in_opt IMFMediaEvent *pEvent,
            /* [out] */ __RPC__out DWORD *pdwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFQualityAdvise2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFQualityAdvise2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFQualityAdvise2 * This);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, SetDropMode)
        HRESULT ( STDMETHODCALLTYPE *SetDropMode )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [in] */ MF_QUALITY_DROP_MODE eDropMode);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, SetQualityLevel)
        HRESULT ( STDMETHODCALLTYPE *SetQualityLevel )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [in] */ MF_QUALITY_LEVEL eQualityLevel);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, GetDropMode)
        HRESULT ( STDMETHODCALLTYPE *GetDropMode )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_DROP_MODE *peDropMode);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, GetQualityLevel)
        HRESULT ( STDMETHODCALLTYPE *GetQualityLevel )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_LEVEL *peQualityLevel);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise, DropTime)
        HRESULT ( STDMETHODCALLTYPE *DropTime )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [in] */ LONGLONG hnsAmountToDrop);
        
        DECLSPEC_XFGVIRT(IMFQualityAdvise2, NotifyQualityEvent)
        HRESULT ( STDMETHODCALLTYPE *NotifyQualityEvent )( 
            __RPC__in IMFQualityAdvise2 * This,
            /* [in] */ __RPC__in_opt IMFMediaEvent *pEvent,
            /* [out] */ __RPC__out DWORD *pdwFlags);
        
        END_INTERFACE
    } IMFQualityAdvise2Vtbl;

    interface IMFQualityAdvise2
    {
        CONST_VTBL struct IMFQualityAdvise2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFQualityAdvise2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFQualityAdvise2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFQualityAdvise2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFQualityAdvise2_SetDropMode(This,eDropMode)	\
    ( (This)->lpVtbl -> SetDropMode(This,eDropMode) ) 

#define IMFQualityAdvise2_SetQualityLevel(This,eQualityLevel)	\
    ( (This)->lpVtbl -> SetQualityLevel(This,eQualityLevel) ) 

#define IMFQualityAdvise2_GetDropMode(This,peDropMode)	\
    ( (This)->lpVtbl -> GetDropMode(This,peDropMode) ) 

#define IMFQualityAdvise2_GetQualityLevel(This,peQualityLevel)	\
    ( (This)->lpVtbl -> GetQualityLevel(This,peQualityLevel) ) 

#define IMFQualityAdvise2_DropTime(This,hnsAmountToDrop)	\
    ( (This)->lpVtbl -> DropTime(This,hnsAmountToDrop) ) 


#define IMFQualityAdvise2_NotifyQualityEvent(This,pEvent,pdwFlags)	\
    ( (This)->lpVtbl -> NotifyQualityEvent(This,pEvent,pdwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFQualityAdvise2_INTERFACE_DEFINED__ */


#ifndef __IMFQualityAdviseLimits_INTERFACE_DEFINED__
#define __IMFQualityAdviseLimits_INTERFACE_DEFINED__

/* interface IMFQualityAdviseLimits */
/* [uuid][object] */ 






EXTERN_C const IID IID_IMFQualityAdviseLimits;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("dfcd8e4d-30b5-4567-acaa-8eb5b7853dc9")
    IMFQualityAdviseLimits : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMaximumDropMode( 
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_DROP_MODE *peDropMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMinimumQualityLevel( 
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_LEVEL *peQualityLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFQualityAdviseLimitsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFQualityAdviseLimits * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFQualityAdviseLimits * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFQualityAdviseLimits * This);
        
        DECLSPEC_XFGVIRT(IMFQualityAdviseLimits, GetMaximumDropMode)
        HRESULT ( STDMETHODCALLTYPE *GetMaximumDropMode )( 
            __RPC__in IMFQualityAdviseLimits * This,
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_DROP_MODE *peDropMode);
        
        DECLSPEC_XFGVIRT(IMFQualityAdviseLimits, GetMinimumQualityLevel)
        HRESULT ( STDMETHODCALLTYPE *GetMinimumQualityLevel )( 
            __RPC__in IMFQualityAdviseLimits * This,
            /* [annotation][out] */ 
            _Out_  MF_QUALITY_LEVEL *peQualityLevel);
        
        END_INTERFACE
    } IMFQualityAdviseLimitsVtbl;

    interface IMFQualityAdviseLimits
    {
        CONST_VTBL struct IMFQualityAdviseLimitsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFQualityAdviseLimits_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFQualityAdviseLimits_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFQualityAdviseLimits_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFQualityAdviseLimits_GetMaximumDropMode(This,peDropMode)	\
    ( (This)->lpVtbl -> GetMaximumDropMode(This,peDropMode) ) 

#define IMFQualityAdviseLimits_GetMinimumQualityLevel(This,peQualityLevel)	\
    ( (This)->lpVtbl -> GetMinimumQualityLevel(This,peQualityLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFQualityAdviseLimits_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0050 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0050_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0050_v0_0_s_ifspec;

#ifndef __IMFRealTimeClient_INTERFACE_DEFINED__
#define __IMFRealTimeClient_INTERFACE_DEFINED__

/* interface IMFRealTimeClient */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFRealTimeClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2347D60B-3FB5-480c-8803-8DF3ADCD3EF0")
    IMFRealTimeClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterThreads( 
            /* [in] */ DWORD dwTaskIndex,
            /* [in] */ LPCWSTR wszClass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterThreads( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWorkQueue( 
            /* [in] */ DWORD dwWorkQueueId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRealTimeClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFRealTimeClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFRealTimeClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFRealTimeClient * This);
        
        DECLSPEC_XFGVIRT(IMFRealTimeClient, RegisterThreads)
        HRESULT ( STDMETHODCALLTYPE *RegisterThreads )( 
            IMFRealTimeClient * This,
            /* [in] */ DWORD dwTaskIndex,
            /* [in] */ LPCWSTR wszClass);
        
        DECLSPEC_XFGVIRT(IMFRealTimeClient, UnregisterThreads)
        HRESULT ( STDMETHODCALLTYPE *UnregisterThreads )( 
            IMFRealTimeClient * This);
        
        DECLSPEC_XFGVIRT(IMFRealTimeClient, SetWorkQueue)
        HRESULT ( STDMETHODCALLTYPE *SetWorkQueue )( 
            IMFRealTimeClient * This,
            /* [in] */ DWORD dwWorkQueueId);
        
        END_INTERFACE
    } IMFRealTimeClientVtbl;

    interface IMFRealTimeClient
    {
        CONST_VTBL struct IMFRealTimeClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRealTimeClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRealTimeClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRealTimeClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRealTimeClient_RegisterThreads(This,dwTaskIndex,wszClass)	\
    ( (This)->lpVtbl -> RegisterThreads(This,dwTaskIndex,wszClass) ) 

#define IMFRealTimeClient_UnregisterThreads(This)	\
    ( (This)->lpVtbl -> UnregisterThreads(This) ) 

#define IMFRealTimeClient_SetWorkQueue(This,dwWorkQueueId)	\
    ( (This)->lpVtbl -> SetWorkQueue(This,dwWorkQueueId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRealTimeClient_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0051 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0051_v0_0_s_ifspec;

#ifndef __IMFRealTimeClientEx_INTERFACE_DEFINED__
#define __IMFRealTimeClientEx_INTERFACE_DEFINED__

/* interface IMFRealTimeClientEx */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFRealTimeClientEx;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03910848-AB16-4611-B100-17B88AE2F248")
    IMFRealTimeClientEx : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterThreadsEx( 
            /* [out][in] */ __RPC__inout DWORD *pdwTaskIndex,
            /* [in] */ __RPC__in LPCWSTR wszClassName,
            /* [in] */ LONG lBasePriority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterThreads( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetWorkQueueEx( 
            /* [in] */ DWORD dwMultithreadedWorkQueueId,
            /* [in] */ LONG lWorkItemBasePriority) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRealTimeClientExVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFRealTimeClientEx * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFRealTimeClientEx * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFRealTimeClientEx * This);
        
        DECLSPEC_XFGVIRT(IMFRealTimeClientEx, RegisterThreadsEx)
        HRESULT ( STDMETHODCALLTYPE *RegisterThreadsEx )( 
            __RPC__in IMFRealTimeClientEx * This,
            /* [out][in] */ __RPC__inout DWORD *pdwTaskIndex,
            /* [in] */ __RPC__in LPCWSTR wszClassName,
            /* [in] */ LONG lBasePriority);
        
        DECLSPEC_XFGVIRT(IMFRealTimeClientEx, UnregisterThreads)
        HRESULT ( STDMETHODCALLTYPE *UnregisterThreads )( 
            __RPC__in IMFRealTimeClientEx * This);
        
        DECLSPEC_XFGVIRT(IMFRealTimeClientEx, SetWorkQueueEx)
        HRESULT ( STDMETHODCALLTYPE *SetWorkQueueEx )( 
            __RPC__in IMFRealTimeClientEx * This,
            /* [in] */ DWORD dwMultithreadedWorkQueueId,
            /* [in] */ LONG lWorkItemBasePriority);
        
        END_INTERFACE
    } IMFRealTimeClientExVtbl;

    interface IMFRealTimeClientEx
    {
        CONST_VTBL struct IMFRealTimeClientExVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRealTimeClientEx_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRealTimeClientEx_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRealTimeClientEx_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRealTimeClientEx_RegisterThreadsEx(This,pdwTaskIndex,wszClassName,lBasePriority)	\
    ( (This)->lpVtbl -> RegisterThreadsEx(This,pdwTaskIndex,wszClassName,lBasePriority) ) 

#define IMFRealTimeClientEx_UnregisterThreads(This)	\
    ( (This)->lpVtbl -> UnregisterThreads(This) ) 

#define IMFRealTimeClientEx_SetWorkQueueEx(This,dwMultithreadedWorkQueueId,lWorkItemBasePriority)	\
    ( (This)->lpVtbl -> SetWorkQueueEx(This,dwMultithreadedWorkQueueId,lWorkItemBasePriority) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRealTimeClientEx_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0052 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef DWORD MFSequencerElementId;

#define	MFSEQUENCER_INVALID_ELEMENT_ID	( 0xffffffff )

typedef 
enum _MFSequencerTopologyFlags
    {
        SequencerTopologyFlags_Last	= 0x1
    } 	MFSequencerTopologyFlags;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0052_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0052_v0_0_s_ifspec;

#ifndef __IMFSequencerSource_INTERFACE_DEFINED__
#define __IMFSequencerSource_INTERFACE_DEFINED__

/* interface IMFSequencerSource */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSequencerSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("197CD219-19CB-4de1-A64C-ACF2EDCBE59E")
    IMFSequencerSource : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AppendTopology( 
            /* [in] */ IMFTopology *pTopology,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  MFSequencerElementId *pdwId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteTopology( 
            /* [in] */ MFSequencerElementId dwId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPresentationContext( 
            /* [in] */ IMFPresentationDescriptor *pPD,
            /* [annotation][optional][out] */ 
            _Out_opt_  MFSequencerElementId *pId,
            /* [annotation][optional][out] */ 
            _Out_opt_  IMFTopology **ppTopology) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateTopology( 
            /* [in] */ MFSequencerElementId dwId,
            /* [in] */ IMFTopology *pTopology) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateTopologyFlags( 
            /* [in] */ MFSequencerElementId dwId,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSequencerSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSequencerSource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSequencerSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSequencerSource * This);
        
        DECLSPEC_XFGVIRT(IMFSequencerSource, AppendTopology)
        HRESULT ( STDMETHODCALLTYPE *AppendTopology )( 
            IMFSequencerSource * This,
            /* [in] */ IMFTopology *pTopology,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  MFSequencerElementId *pdwId);
        
        DECLSPEC_XFGVIRT(IMFSequencerSource, DeleteTopology)
        HRESULT ( STDMETHODCALLTYPE *DeleteTopology )( 
            IMFSequencerSource * This,
            /* [in] */ MFSequencerElementId dwId);
        
        DECLSPEC_XFGVIRT(IMFSequencerSource, GetPresentationContext)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationContext )( 
            IMFSequencerSource * This,
            /* [in] */ IMFPresentationDescriptor *pPD,
            /* [annotation][optional][out] */ 
            _Out_opt_  MFSequencerElementId *pId,
            /* [annotation][optional][out] */ 
            _Out_opt_  IMFTopology **ppTopology);
        
        DECLSPEC_XFGVIRT(IMFSequencerSource, UpdateTopology)
        HRESULT ( STDMETHODCALLTYPE *UpdateTopology )( 
            IMFSequencerSource * This,
            /* [in] */ MFSequencerElementId dwId,
            /* [in] */ IMFTopology *pTopology);
        
        DECLSPEC_XFGVIRT(IMFSequencerSource, UpdateTopologyFlags)
        HRESULT ( STDMETHODCALLTYPE *UpdateTopologyFlags )( 
            IMFSequencerSource * This,
            /* [in] */ MFSequencerElementId dwId,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IMFSequencerSourceVtbl;

    interface IMFSequencerSource
    {
        CONST_VTBL struct IMFSequencerSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSequencerSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSequencerSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSequencerSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSequencerSource_AppendTopology(This,pTopology,dwFlags,pdwId)	\
    ( (This)->lpVtbl -> AppendTopology(This,pTopology,dwFlags,pdwId) ) 

#define IMFSequencerSource_DeleteTopology(This,dwId)	\
    ( (This)->lpVtbl -> DeleteTopology(This,dwId) ) 

#define IMFSequencerSource_GetPresentationContext(This,pPD,pId,ppTopology)	\
    ( (This)->lpVtbl -> GetPresentationContext(This,pPD,pId,ppTopology) ) 

#define IMFSequencerSource_UpdateTopology(This,dwId,pTopology)	\
    ( (This)->lpVtbl -> UpdateTopology(This,dwId,pTopology) ) 

#define IMFSequencerSource_UpdateTopologyFlags(This,dwId,dwFlags)	\
    ( (This)->lpVtbl -> UpdateTopologyFlags(This,dwId,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSequencerSource_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0053 */
/* [local] */ 

EXTERN_GUID( MF_TIME_FORMAT_SEGMENT_OFFSET, 0xc8b8be77, 0x869c, 0x431d, 0x81, 0x2e, 0x16, 0x96, 0x93, 0xf6, 0x5a, 0x39 );
STDAPI MFCreateSequencerSource(
    IUnknown *pReserved,
    _Outptr_ IMFSequencerSource **ppSequencerSource
    );
STDAPI MFCreateSequencerSegmentOffset(
    MFSequencerElementId dwId,
    MFTIME hnsOffset,
    _Out_ PROPVARIANT *pvarSegmentOffset
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
#if (WINVER >= _WIN32_WINNT_WIN7) 
STDAPI MFCreateAggregateSource(
    _In_ IMFCollection *pSourceCollection,
    _Outptr_ IMFMediaSource **ppAggSource
    );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0053_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0053_v0_0_s_ifspec;

#ifndef __IMFMediaSourceTopologyProvider_INTERFACE_DEFINED__
#define __IMFMediaSourceTopologyProvider_INTERFACE_DEFINED__

/* interface IMFMediaSourceTopologyProvider */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSourceTopologyProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0E1D6009-C9F3-442d-8C51-A42D2D49452F")
    IMFMediaSourceTopologyProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMediaSourceTopology( 
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [out] */ __RPC__deref_out_opt IMFTopology **ppTopology) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourceTopologyProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSourceTopologyProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSourceTopologyProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSourceTopologyProvider * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceTopologyProvider, GetMediaSourceTopology)
        HRESULT ( STDMETHODCALLTYPE *GetMediaSourceTopology )( 
            __RPC__in IMFMediaSourceTopologyProvider * This,
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor,
            /* [out] */ __RPC__deref_out_opt IMFTopology **ppTopology);
        
        END_INTERFACE
    } IMFMediaSourceTopologyProviderVtbl;

    interface IMFMediaSourceTopologyProvider
    {
        CONST_VTBL struct IMFMediaSourceTopologyProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSourceTopologyProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSourceTopologyProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSourceTopologyProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSourceTopologyProvider_GetMediaSourceTopology(This,pPresentationDescriptor,ppTopology)	\
    ( (This)->lpVtbl -> GetMediaSourceTopology(This,pPresentationDescriptor,ppTopology) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSourceTopologyProvider_INTERFACE_DEFINED__ */


#ifndef __IMFMediaSourcePresentationProvider_INTERFACE_DEFINED__
#define __IMFMediaSourcePresentationProvider_INTERFACE_DEFINED__

/* interface IMFMediaSourcePresentationProvider */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSourcePresentationProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0E1D600a-C9F3-442d-8C51-A42D2D49452F")
    IMFMediaSourcePresentationProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ForceEndOfPresentation( 
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSourcePresentationProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSourcePresentationProvider * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSourcePresentationProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSourcePresentationProvider * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourcePresentationProvider, ForceEndOfPresentation)
        HRESULT ( STDMETHODCALLTYPE *ForceEndOfPresentation )( 
            __RPC__in IMFMediaSourcePresentationProvider * This,
            /* [in] */ __RPC__in_opt IMFPresentationDescriptor *pPresentationDescriptor);
        
        END_INTERFACE
    } IMFMediaSourcePresentationProviderVtbl;

    interface IMFMediaSourcePresentationProvider
    {
        CONST_VTBL struct IMFMediaSourcePresentationProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSourcePresentationProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSourcePresentationProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSourcePresentationProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSourcePresentationProvider_ForceEndOfPresentation(This,pPresentationDescriptor)	\
    ( (This)->lpVtbl -> ForceEndOfPresentation(This,pPresentationDescriptor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSourcePresentationProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0055 */
/* [local] */ 

EXTERN_GUID( MF_SOURCE_PRESENTATION_PROVIDER_SERVICE, 0xe002aadc, 0xf4af, 0x4ee5, 0x98, 0x47, 0x05, 0x3e, 0xdf, 0x84, 0x04, 0x26 );
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820 4201) // Disable C4820: padding after data member, C4201: nonstandard extension used: nameless struct/union
#endif
typedef struct _MFTOPONODE_ATTRIBUTE_UPDATE
    {
    TOPOID NodeId;
    GUID guidAttributeKey;
    MF_ATTRIBUTE_TYPE attrType;
    /* [switch_type][switch_is] */ union 
        {
        /* [case()] */ UINT32 u32;
        /* [case()] */ UINT64 u64;
        /* [case()] */ double d;
        /* [default] */  /* Empty union arm */ 
        } 	;
    } 	MFTOPONODE_ATTRIBUTE_UPDATE;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0055_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0055_v0_0_s_ifspec;

#ifndef __IMFTopologyNodeAttributeEditor_INTERFACE_DEFINED__
#define __IMFTopologyNodeAttributeEditor_INTERFACE_DEFINED__

/* interface IMFTopologyNodeAttributeEditor */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFTopologyNodeAttributeEditor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("676aa6dd-238a-410d-bb99-65668d01605a")
    IMFTopologyNodeAttributeEditor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UpdateNodeAttributes( 
            /* [in] */ TOPOID TopoId,
            /* [in] */ DWORD cUpdates,
            /* [size_is][in] */ __RPC__in_ecount_full(cUpdates) MFTOPONODE_ATTRIBUTE_UPDATE *pUpdates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTopologyNodeAttributeEditorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFTopologyNodeAttributeEditor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFTopologyNodeAttributeEditor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFTopologyNodeAttributeEditor * This);
        
        DECLSPEC_XFGVIRT(IMFTopologyNodeAttributeEditor, UpdateNodeAttributes)
        HRESULT ( STDMETHODCALLTYPE *UpdateNodeAttributes )( 
            __RPC__in IMFTopologyNodeAttributeEditor * This,
            /* [in] */ TOPOID TopoId,
            /* [in] */ DWORD cUpdates,
            /* [size_is][in] */ __RPC__in_ecount_full(cUpdates) MFTOPONODE_ATTRIBUTE_UPDATE *pUpdates);
        
        END_INTERFACE
    } IMFTopologyNodeAttributeEditorVtbl;

    interface IMFTopologyNodeAttributeEditor
    {
        CONST_VTBL struct IMFTopologyNodeAttributeEditorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTopologyNodeAttributeEditor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTopologyNodeAttributeEditor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTopologyNodeAttributeEditor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTopologyNodeAttributeEditor_UpdateNodeAttributes(This,TopoId,cUpdates,pUpdates)	\
    ( (This)->lpVtbl -> UpdateNodeAttributes(This,TopoId,cUpdates,pUpdates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTopologyNodeAttributeEditor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0056 */
/* [local] */ 

EXTERN_GUID( MF_TOPONODE_ATTRIBUTE_EDITOR_SERVICE, 0x65656e1a, 0x077f, 0x4472, 0x83, 0xef, 0x31, 0x6f, 0x11, 0xd5, 0x08, 0x7a );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [public] */ struct _MF_LEAKY_BUCKET_PAIR
    {
    DWORD dwBitrate;
    DWORD msBufferWindow;
    } 	MF_LEAKY_BUCKET_PAIR;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef /* [public] */ struct _MFBYTESTREAM_BUFFERING_PARAMS
    {
    QWORD cbTotalFileSize;
    QWORD cbPlayableDataSize;
    MF_LEAKY_BUCKET_PAIR *prgBuckets;
    DWORD cBuckets;
    QWORD qwNetBufferingTime;
    QWORD qwExtraBufferingTimeDuringSeek;
    QWORD qwPlayDuration;
    float dRate;
    } 	MFBYTESTREAM_BUFFERING_PARAMS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0056_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0056_v0_0_s_ifspec;

#ifndef __IMFByteStreamBuffering_INTERFACE_DEFINED__
#define __IMFByteStreamBuffering_INTERFACE_DEFINED__

/* interface IMFByteStreamBuffering */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFByteStreamBuffering;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6d66d782-1d4f-4db7-8c63-cb8c77f1ef5e")
    IMFByteStreamBuffering : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetBufferingParams( 
            /* [in] */ __RPC__in MFBYTESTREAM_BUFFERING_PARAMS *pParams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnableBuffering( 
            /* [in] */ BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopBuffering( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamBufferingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFByteStreamBuffering * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFByteStreamBuffering * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFByteStreamBuffering * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamBuffering, SetBufferingParams)
        HRESULT ( STDMETHODCALLTYPE *SetBufferingParams )( 
            __RPC__in IMFByteStreamBuffering * This,
            /* [in] */ __RPC__in MFBYTESTREAM_BUFFERING_PARAMS *pParams);
        
        DECLSPEC_XFGVIRT(IMFByteStreamBuffering, EnableBuffering)
        HRESULT ( STDMETHODCALLTYPE *EnableBuffering )( 
            __RPC__in IMFByteStreamBuffering * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IMFByteStreamBuffering, StopBuffering)
        HRESULT ( STDMETHODCALLTYPE *StopBuffering )( 
            __RPC__in IMFByteStreamBuffering * This);
        
        END_INTERFACE
    } IMFByteStreamBufferingVtbl;

    interface IMFByteStreamBuffering
    {
        CONST_VTBL struct IMFByteStreamBufferingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStreamBuffering_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStreamBuffering_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStreamBuffering_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStreamBuffering_SetBufferingParams(This,pParams)	\
    ( (This)->lpVtbl -> SetBufferingParams(This,pParams) ) 

#define IMFByteStreamBuffering_EnableBuffering(This,fEnable)	\
    ( (This)->lpVtbl -> EnableBuffering(This,fEnable) ) 

#define IMFByteStreamBuffering_StopBuffering(This)	\
    ( (This)->lpVtbl -> StopBuffering(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFByteStreamBuffering_INTERFACE_DEFINED__ */


#ifndef __IMFByteStreamCacheControl_INTERFACE_DEFINED__
#define __IMFByteStreamCacheControl_INTERFACE_DEFINED__

/* interface IMFByteStreamCacheControl */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFByteStreamCacheControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F5042EA4-7A96-4a75-AA7B-2BE1EF7F88D5")
    IMFByteStreamCacheControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StopBackgroundTransfer( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamCacheControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFByteStreamCacheControl * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFByteStreamCacheControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFByteStreamCacheControl * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamCacheControl, StopBackgroundTransfer)
        HRESULT ( STDMETHODCALLTYPE *StopBackgroundTransfer )( 
            __RPC__in IMFByteStreamCacheControl * This);
        
        END_INTERFACE
    } IMFByteStreamCacheControlVtbl;

    interface IMFByteStreamCacheControl
    {
        CONST_VTBL struct IMFByteStreamCacheControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStreamCacheControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStreamCacheControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStreamCacheControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStreamCacheControl_StopBackgroundTransfer(This)	\
    ( (This)->lpVtbl -> StopBackgroundTransfer(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFByteStreamCacheControl_INTERFACE_DEFINED__ */


#ifndef __IMFByteStreamTimeSeek_INTERFACE_DEFINED__
#define __IMFByteStreamTimeSeek_INTERFACE_DEFINED__

/* interface IMFByteStreamTimeSeek */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFByteStreamTimeSeek;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("64976BFA-FB61-4041-9069-8C9A5F659BEB")
    IMFByteStreamTimeSeek : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsTimeSeekSupported( 
            /* [out] */ __RPC__out BOOL *pfTimeSeekIsSupported) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TimeSeek( 
            /* [in] */ QWORD qwTimePosition) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeSeekResult( 
            /* [out] */ __RPC__out QWORD *pqwStartTime,
            /* [out] */ __RPC__out QWORD *pqwStopTime,
            /* [out] */ __RPC__out QWORD *pqwDuration) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamTimeSeekVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFByteStreamTimeSeek * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFByteStreamTimeSeek * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFByteStreamTimeSeek * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamTimeSeek, IsTimeSeekSupported)
        HRESULT ( STDMETHODCALLTYPE *IsTimeSeekSupported )( 
            __RPC__in IMFByteStreamTimeSeek * This,
            /* [out] */ __RPC__out BOOL *pfTimeSeekIsSupported);
        
        DECLSPEC_XFGVIRT(IMFByteStreamTimeSeek, TimeSeek)
        HRESULT ( STDMETHODCALLTYPE *TimeSeek )( 
            __RPC__in IMFByteStreamTimeSeek * This,
            /* [in] */ QWORD qwTimePosition);
        
        DECLSPEC_XFGVIRT(IMFByteStreamTimeSeek, GetTimeSeekResult)
        HRESULT ( STDMETHODCALLTYPE *GetTimeSeekResult )( 
            __RPC__in IMFByteStreamTimeSeek * This,
            /* [out] */ __RPC__out QWORD *pqwStartTime,
            /* [out] */ __RPC__out QWORD *pqwStopTime,
            /* [out] */ __RPC__out QWORD *pqwDuration);
        
        END_INTERFACE
    } IMFByteStreamTimeSeekVtbl;

    interface IMFByteStreamTimeSeek
    {
        CONST_VTBL struct IMFByteStreamTimeSeekVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStreamTimeSeek_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStreamTimeSeek_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStreamTimeSeek_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStreamTimeSeek_IsTimeSeekSupported(This,pfTimeSeekIsSupported)	\
    ( (This)->lpVtbl -> IsTimeSeekSupported(This,pfTimeSeekIsSupported) ) 

#define IMFByteStreamTimeSeek_TimeSeek(This,qwTimePosition)	\
    ( (This)->lpVtbl -> TimeSeek(This,qwTimePosition) ) 

#define IMFByteStreamTimeSeek_GetTimeSeekResult(This,pqwStartTime,pqwStopTime,pqwDuration)	\
    ( (This)->lpVtbl -> GetTimeSeekResult(This,pqwStartTime,pqwStopTime,pqwDuration) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFByteStreamTimeSeek_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0059 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN8) 
typedef /* [public][public] */ struct __MIDL___MIDL_itf_mfidl_0000_0059_0001
    {
    QWORD qwStartOffset;
    QWORD qwEndOffset;
    } 	MF_BYTE_STREAM_CACHE_RANGE;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0059_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0059_v0_0_s_ifspec;

#ifndef __IMFByteStreamCacheControl2_INTERFACE_DEFINED__
#define __IMFByteStreamCacheControl2_INTERFACE_DEFINED__

/* interface IMFByteStreamCacheControl2 */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFByteStreamCacheControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71CE469C-F34B-49EA-A56B-2D2A10E51149")
    IMFByteStreamCacheControl2 : public IMFByteStreamCacheControl
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetByteRanges( 
            /* [out] */ __RPC__out DWORD *pcRanges,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcRanges) MF_BYTE_STREAM_CACHE_RANGE **ppRanges) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCacheLimit( 
            /* [in] */ QWORD qwBytes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsBackgroundTransferActive( 
            /* [out] */ __RPC__out BOOL *pfActive) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamCacheControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFByteStreamCacheControl2 * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFByteStreamCacheControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFByteStreamCacheControl2 * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamCacheControl, StopBackgroundTransfer)
        HRESULT ( STDMETHODCALLTYPE *StopBackgroundTransfer )( 
            __RPC__in IMFByteStreamCacheControl2 * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamCacheControl2, GetByteRanges)
        HRESULT ( STDMETHODCALLTYPE *GetByteRanges )( 
            __RPC__in IMFByteStreamCacheControl2 * This,
            /* [out] */ __RPC__out DWORD *pcRanges,
            /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcRanges) MF_BYTE_STREAM_CACHE_RANGE **ppRanges);
        
        DECLSPEC_XFGVIRT(IMFByteStreamCacheControl2, SetCacheLimit)
        HRESULT ( STDMETHODCALLTYPE *SetCacheLimit )( 
            __RPC__in IMFByteStreamCacheControl2 * This,
            /* [in] */ QWORD qwBytes);
        
        DECLSPEC_XFGVIRT(IMFByteStreamCacheControl2, IsBackgroundTransferActive)
        HRESULT ( STDMETHODCALLTYPE *IsBackgroundTransferActive )( 
            __RPC__in IMFByteStreamCacheControl2 * This,
            /* [out] */ __RPC__out BOOL *pfActive);
        
        END_INTERFACE
    } IMFByteStreamCacheControl2Vtbl;

    interface IMFByteStreamCacheControl2
    {
        CONST_VTBL struct IMFByteStreamCacheControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStreamCacheControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStreamCacheControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStreamCacheControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStreamCacheControl2_StopBackgroundTransfer(This)	\
    ( (This)->lpVtbl -> StopBackgroundTransfer(This) ) 


#define IMFByteStreamCacheControl2_GetByteRanges(This,pcRanges,ppRanges)	\
    ( (This)->lpVtbl -> GetByteRanges(This,pcRanges,ppRanges) ) 

#define IMFByteStreamCacheControl2_SetCacheLimit(This,qwBytes)	\
    ( (This)->lpVtbl -> SetCacheLimit(This,qwBytes) ) 

#define IMFByteStreamCacheControl2_IsBackgroundTransferActive(This,pfActive)	\
    ( (This)->lpVtbl -> IsBackgroundTransferActive(This,pfActive) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFByteStreamCacheControl2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0060 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0060_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0060_v0_0_s_ifspec;

#ifndef __IMFNetCredential_INTERFACE_DEFINED__
#define __IMFNetCredential_INTERFACE_DEFINED__

/* interface IMFNetCredential */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFNetCredential;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5b87ef6a-7ed8-434f-ba0e-184fac1628d1")
    IMFNetCredential : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetUser( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbData)  BYTE *pbData,
            /* [in] */ DWORD cbData,
            /* [in] */ BOOL fDataIsEncrypted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPassword( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbData)  BYTE *pbData,
            /* [in] */ DWORD cbData,
            /* [in] */ BOOL fDataIsEncrypted) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetUser( 
            /* [annotation][size_is][out] */ 
            _Out_writes_to_opt_(*pcbData,*pcbData)  BYTE *pbData,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pcbData,
            /* [in] */ BOOL fEncryptData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPassword( 
            /* [annotation][size_is][out] */ 
            _Out_writes_to_opt_(*pcbData,*pcbData)  BYTE *pbData,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pcbData,
            /* [in] */ BOOL fEncryptData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoggedOnUser( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfLoggedOnUser) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetCredentialVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetCredential * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetCredential * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetCredential * This);
        
        DECLSPEC_XFGVIRT(IMFNetCredential, SetUser)
        HRESULT ( STDMETHODCALLTYPE *SetUser )( 
            IMFNetCredential * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbData)  BYTE *pbData,
            /* [in] */ DWORD cbData,
            /* [in] */ BOOL fDataIsEncrypted);
        
        DECLSPEC_XFGVIRT(IMFNetCredential, SetPassword)
        HRESULT ( STDMETHODCALLTYPE *SetPassword )( 
            IMFNetCredential * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbData)  BYTE *pbData,
            /* [in] */ DWORD cbData,
            /* [in] */ BOOL fDataIsEncrypted);
        
        DECLSPEC_XFGVIRT(IMFNetCredential, GetUser)
        HRESULT ( STDMETHODCALLTYPE *GetUser )( 
            IMFNetCredential * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_opt_(*pcbData,*pcbData)  BYTE *pbData,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pcbData,
            /* [in] */ BOOL fEncryptData);
        
        DECLSPEC_XFGVIRT(IMFNetCredential, GetPassword)
        HRESULT ( STDMETHODCALLTYPE *GetPassword )( 
            IMFNetCredential * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_opt_(*pcbData,*pcbData)  BYTE *pbData,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pcbData,
            /* [in] */ BOOL fEncryptData);
        
        DECLSPEC_XFGVIRT(IMFNetCredential, LoggedOnUser)
        HRESULT ( STDMETHODCALLTYPE *LoggedOnUser )( 
            IMFNetCredential * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfLoggedOnUser);
        
        END_INTERFACE
    } IMFNetCredentialVtbl;

    interface IMFNetCredential
    {
        CONST_VTBL struct IMFNetCredentialVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetCredential_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetCredential_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetCredential_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetCredential_SetUser(This,pbData,cbData,fDataIsEncrypted)	\
    ( (This)->lpVtbl -> SetUser(This,pbData,cbData,fDataIsEncrypted) ) 

#define IMFNetCredential_SetPassword(This,pbData,cbData,fDataIsEncrypted)	\
    ( (This)->lpVtbl -> SetPassword(This,pbData,cbData,fDataIsEncrypted) ) 

#define IMFNetCredential_GetUser(This,pbData,pcbData,fEncryptData)	\
    ( (This)->lpVtbl -> GetUser(This,pbData,pcbData,fEncryptData) ) 

#define IMFNetCredential_GetPassword(This,pbData,pcbData,fEncryptData)	\
    ( (This)->lpVtbl -> GetPassword(This,pbData,pcbData,fEncryptData) ) 

#define IMFNetCredential_LoggedOnUser(This,pfLoggedOnUser)	\
    ( (This)->lpVtbl -> LoggedOnUser(This,pfLoggedOnUser) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetCredential_INTERFACE_DEFINED__ */


#ifndef __IMFNetCredentialManager_INTERFACE_DEFINED__
#define __IMFNetCredentialManager_INTERFACE_DEFINED__

/* interface IMFNetCredentialManager */
/* [local][uuid][object] */ 

typedef struct _MFNetCredentialManagerGetParam
    {
    HRESULT hrOp;
    BOOL fAllowLoggedOnUser;
    BOOL fClearTextPackage;
    LPCWSTR pszUrl;
    LPCWSTR pszSite;
    LPCWSTR pszRealm;
    LPCWSTR pszPackage;
    LONG nRetries;
    } 	MFNetCredentialManagerGetParam;


EXTERN_C const IID IID_IMFNetCredentialManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5b87ef6b-7ed8-434f-ba0e-184fac1628d1")
    IMFNetCredentialManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginGetCredentials( 
            /* [in] */ MFNetCredentialManagerGetParam *pParam,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetCredentials( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  IMFNetCredential **ppCred) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGood( 
            /* [in] */ IMFNetCredential *pCred,
            /* [in] */ BOOL fGood) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetCredentialManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetCredentialManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetCredentialManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetCredentialManager * This);
        
        DECLSPEC_XFGVIRT(IMFNetCredentialManager, BeginGetCredentials)
        HRESULT ( STDMETHODCALLTYPE *BeginGetCredentials )( 
            IMFNetCredentialManager * This,
            /* [in] */ MFNetCredentialManagerGetParam *pParam,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFNetCredentialManager, EndGetCredentials)
        HRESULT ( STDMETHODCALLTYPE *EndGetCredentials )( 
            IMFNetCredentialManager * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Outptr_  IMFNetCredential **ppCred);
        
        DECLSPEC_XFGVIRT(IMFNetCredentialManager, SetGood)
        HRESULT ( STDMETHODCALLTYPE *SetGood )( 
            IMFNetCredentialManager * This,
            /* [in] */ IMFNetCredential *pCred,
            /* [in] */ BOOL fGood);
        
        END_INTERFACE
    } IMFNetCredentialManagerVtbl;

    interface IMFNetCredentialManager
    {
        CONST_VTBL struct IMFNetCredentialManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetCredentialManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetCredentialManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetCredentialManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetCredentialManager_BeginGetCredentials(This,pParam,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginGetCredentials(This,pParam,pCallback,pState) ) 

#define IMFNetCredentialManager_EndGetCredentials(This,pResult,ppCred)	\
    ( (This)->lpVtbl -> EndGetCredentials(This,pResult,ppCred) ) 

#define IMFNetCredentialManager_SetGood(This,pCred,fGood)	\
    ( (This)->lpVtbl -> SetGood(This,pCred,fGood) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetCredentialManager_INTERFACE_DEFINED__ */


#ifndef __IMFNetCredentialCache_INTERFACE_DEFINED__
#define __IMFNetCredentialCache_INTERFACE_DEFINED__

/* interface IMFNetCredentialCache */
/* [local][uuid][object] */ 

typedef 
enum _MFNetCredentialRequirements
    {
        REQUIRE_PROMPT	= 0x1,
        REQUIRE_SAVE_SELECTED	= 0x2
    } 	MFNetCredentialRequirements;

typedef 
enum _MFNetCredentialOptions
    {
        MFNET_CREDENTIAL_SAVE	= 0x1,
        MFNET_CREDENTIAL_DONT_CACHE	= 0x2,
        MFNET_CREDENTIAL_ALLOW_CLEAR_TEXT	= 0x4
    } 	MFNetCredentialOptions;

typedef 
enum _MFNetAuthenticationFlags
    {
        MFNET_AUTHENTICATION_PROXY	= 0x1,
        MFNET_AUTHENTICATION_CLEAR_TEXT	= 0x2,
        MFNET_AUTHENTICATION_LOGGED_ON_USER	= 0x4
    } 	MFNetAuthenticationFlags;


EXTERN_C const IID IID_IMFNetCredentialCache;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5b87ef6c-7ed8-434f-ba0e-184fac1628d1")
    IMFNetCredentialCache : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCredential( 
            /* [in] */ LPCWSTR pszUrl,
            /* [in] */ LPCWSTR pszRealm,
            /* [in] */ DWORD dwAuthenticationFlags,
            /* [annotation][out] */ 
            _Outptr_  IMFNetCredential **ppCred,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwRequirementsFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGood( 
            /* [in] */ IMFNetCredential *pCred,
            /* [in] */ BOOL fGood) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetUserOptions( 
            /* [in] */ IMFNetCredential *pCred,
            /* [in] */ DWORD dwOptionsFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetCredentialCacheVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetCredentialCache * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetCredentialCache * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetCredentialCache * This);
        
        DECLSPEC_XFGVIRT(IMFNetCredentialCache, GetCredential)
        HRESULT ( STDMETHODCALLTYPE *GetCredential )( 
            IMFNetCredentialCache * This,
            /* [in] */ LPCWSTR pszUrl,
            /* [in] */ LPCWSTR pszRealm,
            /* [in] */ DWORD dwAuthenticationFlags,
            /* [annotation][out] */ 
            _Outptr_  IMFNetCredential **ppCred,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwRequirementsFlags);
        
        DECLSPEC_XFGVIRT(IMFNetCredentialCache, SetGood)
        HRESULT ( STDMETHODCALLTYPE *SetGood )( 
            IMFNetCredentialCache * This,
            /* [in] */ IMFNetCredential *pCred,
            /* [in] */ BOOL fGood);
        
        DECLSPEC_XFGVIRT(IMFNetCredentialCache, SetUserOptions)
        HRESULT ( STDMETHODCALLTYPE *SetUserOptions )( 
            IMFNetCredentialCache * This,
            /* [in] */ IMFNetCredential *pCred,
            /* [in] */ DWORD dwOptionsFlags);
        
        END_INTERFACE
    } IMFNetCredentialCacheVtbl;

    interface IMFNetCredentialCache
    {
        CONST_VTBL struct IMFNetCredentialCacheVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetCredentialCache_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetCredentialCache_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetCredentialCache_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetCredentialCache_GetCredential(This,pszUrl,pszRealm,dwAuthenticationFlags,ppCred,pdwRequirementsFlags)	\
    ( (This)->lpVtbl -> GetCredential(This,pszUrl,pszRealm,dwAuthenticationFlags,ppCred,pdwRequirementsFlags) ) 

#define IMFNetCredentialCache_SetGood(This,pCred,fGood)	\
    ( (This)->lpVtbl -> SetGood(This,pCred,fGood) ) 

#define IMFNetCredentialCache_SetUserOptions(This,pCred,dwOptionsFlags)	\
    ( (This)->lpVtbl -> SetUserOptions(This,pCred,dwOptionsFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetCredentialCache_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0063 */
/* [local] */ 

STDAPI 
MFCreateCredentialCache(
    _Outptr_ IMFNetCredentialCache ** ppCache);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0063_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0063_v0_0_s_ifspec;

#ifndef __IMFSSLCertificateManager_INTERFACE_DEFINED__
#define __IMFSSLCertificateManager_INTERFACE_DEFINED__

/* interface IMFSSLCertificateManager */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSSLCertificateManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("61f7d887-1230-4a8b-aeba-8ad434d1a64d")
    IMFSSLCertificateManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetClientCertificate( 
            /* [in] */ LPCWSTR pszURL,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*pcbData)  BYTE **ppbData,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginGetClientCertificate( 
            /* [in] */ LPCWSTR pszURL,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetClientCertificate( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*pcbData)  BYTE **ppbData,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCertificatePolicy( 
            /* [in] */ LPCWSTR pszURL,
            /* [out] */ BOOL *pfOverrideAutomaticCheck,
            /* [out] */ BOOL *pfClientCertificateAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnServerCertificate( 
            /* [in] */ LPCWSTR pszURL,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbData)  BYTE *pbData,
            /* [in] */ DWORD cbData,
            /* [out] */ BOOL *pfIsGood) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSSLCertificateManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSSLCertificateManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSSLCertificateManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSSLCertificateManager * This);
        
        DECLSPEC_XFGVIRT(IMFSSLCertificateManager, GetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetClientCertificate )( 
            IMFSSLCertificateManager * This,
            /* [in] */ LPCWSTR pszURL,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*pcbData)  BYTE **ppbData,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbData);
        
        DECLSPEC_XFGVIRT(IMFSSLCertificateManager, BeginGetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *BeginGetClientCertificate )( 
            IMFSSLCertificateManager * This,
            /* [in] */ LPCWSTR pszURL,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFSSLCertificateManager, EndGetClientCertificate)
        HRESULT ( STDMETHODCALLTYPE *EndGetClientCertificate )( 
            IMFSSLCertificateManager * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_bytebuffer_(*pcbData)  BYTE **ppbData,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbData);
        
        DECLSPEC_XFGVIRT(IMFSSLCertificateManager, GetCertificatePolicy)
        HRESULT ( STDMETHODCALLTYPE *GetCertificatePolicy )( 
            IMFSSLCertificateManager * This,
            /* [in] */ LPCWSTR pszURL,
            /* [out] */ BOOL *pfOverrideAutomaticCheck,
            /* [out] */ BOOL *pfClientCertificateAvailable);
        
        DECLSPEC_XFGVIRT(IMFSSLCertificateManager, OnServerCertificate)
        HRESULT ( STDMETHODCALLTYPE *OnServerCertificate )( 
            IMFSSLCertificateManager * This,
            /* [in] */ LPCWSTR pszURL,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbData)  BYTE *pbData,
            /* [in] */ DWORD cbData,
            /* [out] */ BOOL *pfIsGood);
        
        END_INTERFACE
    } IMFSSLCertificateManagerVtbl;

    interface IMFSSLCertificateManager
    {
        CONST_VTBL struct IMFSSLCertificateManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSSLCertificateManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSSLCertificateManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSSLCertificateManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSSLCertificateManager_GetClientCertificate(This,pszURL,ppbData,pcbData)	\
    ( (This)->lpVtbl -> GetClientCertificate(This,pszURL,ppbData,pcbData) ) 

#define IMFSSLCertificateManager_BeginGetClientCertificate(This,pszURL,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginGetClientCertificate(This,pszURL,pCallback,pState) ) 

#define IMFSSLCertificateManager_EndGetClientCertificate(This,pResult,ppbData,pcbData)	\
    ( (This)->lpVtbl -> EndGetClientCertificate(This,pResult,ppbData,pcbData) ) 

#define IMFSSLCertificateManager_GetCertificatePolicy(This,pszURL,pfOverrideAutomaticCheck,pfClientCertificateAvailable)	\
    ( (This)->lpVtbl -> GetCertificatePolicy(This,pszURL,pfOverrideAutomaticCheck,pfClientCertificateAvailable) ) 

#define IMFSSLCertificateManager_OnServerCertificate(This,pszURL,pbData,cbData,pfIsGood)	\
    ( (This)->lpVtbl -> OnServerCertificate(This,pszURL,pbData,cbData,pfIsGood) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSSLCertificateManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0064 */
/* [local] */ 

EXTERN_GUID( MFNETSOURCE_SSLCERTIFICATE_MANAGER, 0x55e6cb27, 0xe69b, 0x4267, 0x94, 0x0c, 0x2d, 0x7e, 0xc5, 0xbb, 0x8a, 0x0f );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0064_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0064_v0_0_s_ifspec;

#ifndef __IMFNetResourceFilter_INTERFACE_DEFINED__
#define __IMFNetResourceFilter_INTERFACE_DEFINED__

/* interface IMFNetResourceFilter */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFNetResourceFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("091878a3-bf11-4a5c-bc9f-33995b06ef2d")
    IMFNetResourceFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnRedirect( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUrl,
            /* [annotation][out] */ 
            _Out_  VARIANT_BOOL *pvbCancel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnSendingRequest( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUrl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetResourceFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetResourceFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetResourceFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetResourceFilter * This);
        
        DECLSPEC_XFGVIRT(IMFNetResourceFilter, OnRedirect)
        HRESULT ( STDMETHODCALLTYPE *OnRedirect )( 
            IMFNetResourceFilter * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUrl,
            /* [annotation][out] */ 
            _Out_  VARIANT_BOOL *pvbCancel);
        
        DECLSPEC_XFGVIRT(IMFNetResourceFilter, OnSendingRequest)
        HRESULT ( STDMETHODCALLTYPE *OnSendingRequest )( 
            IMFNetResourceFilter * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pszUrl);
        
        END_INTERFACE
    } IMFNetResourceFilterVtbl;

    interface IMFNetResourceFilter
    {
        CONST_VTBL struct IMFNetResourceFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetResourceFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetResourceFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetResourceFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetResourceFilter_OnRedirect(This,pszUrl,pvbCancel)	\
    ( (This)->lpVtbl -> OnRedirect(This,pszUrl,pvbCancel) ) 

#define IMFNetResourceFilter_OnSendingRequest(This,pszUrl)	\
    ( (This)->lpVtbl -> OnSendingRequest(This,pszUrl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetResourceFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0065 */
/* [local] */ 

EXTERN_GUID( MFNETSOURCE_RESOURCE_FILTER, 0x815d0ff6, 0x265a, 0x4477, 0x9e, 0x46, 0x7b, 0x80, 0xad, 0x80, 0xb5, 0xfb);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0065_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0065_v0_0_s_ifspec;

#ifndef __IMFSourceOpenMonitor_INTERFACE_DEFINED__
#define __IMFSourceOpenMonitor_INTERFACE_DEFINED__

/* interface IMFSourceOpenMonitor */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFSourceOpenMonitor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("059054B3-027C-494C-A27D-9113291CF87F")
    IMFSourceOpenMonitor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnSourceEvent( 
            /* [in] */ __RPC__in_opt IMFMediaEvent *pEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSourceOpenMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFSourceOpenMonitor * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFSourceOpenMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFSourceOpenMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFSourceOpenMonitor, OnSourceEvent)
        HRESULT ( STDMETHODCALLTYPE *OnSourceEvent )( 
            __RPC__in IMFSourceOpenMonitor * This,
            /* [in] */ __RPC__in_opt IMFMediaEvent *pEvent);
        
        END_INTERFACE
    } IMFSourceOpenMonitorVtbl;

    interface IMFSourceOpenMonitor
    {
        CONST_VTBL struct IMFSourceOpenMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSourceOpenMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSourceOpenMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSourceOpenMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSourceOpenMonitor_OnSourceEvent(This,pEvent)	\
    ( (This)->lpVtbl -> OnSourceEvent(This,pEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSourceOpenMonitor_INTERFACE_DEFINED__ */


#ifndef __IMFNetProxyLocator_INTERFACE_DEFINED__
#define __IMFNetProxyLocator_INTERFACE_DEFINED__

/* interface IMFNetProxyLocator */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFNetProxyLocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e9cd0383-a268-4bb4-82de-658d53574d41")
    IMFNetProxyLocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindFirstProxy( 
            /* [in] */ LPCWSTR pszHost,
            /* [in] */ LPCWSTR pszUrl,
            /* [in] */ BOOL fReserved) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindNextProxy( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterProxyResult( 
            /* [in] */ HRESULT hrOp) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentProxy( 
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(*pcchStr)  LPWSTR pszStr,
            /* [out][in] */ DWORD *pcchStr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [out] */ IMFNetProxyLocator **ppProxyLocator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetProxyLocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetProxyLocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetProxyLocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetProxyLocator * This);
        
        DECLSPEC_XFGVIRT(IMFNetProxyLocator, FindFirstProxy)
        HRESULT ( STDMETHODCALLTYPE *FindFirstProxy )( 
            IMFNetProxyLocator * This,
            /* [in] */ LPCWSTR pszHost,
            /* [in] */ LPCWSTR pszUrl,
            /* [in] */ BOOL fReserved);
        
        DECLSPEC_XFGVIRT(IMFNetProxyLocator, FindNextProxy)
        HRESULT ( STDMETHODCALLTYPE *FindNextProxy )( 
            IMFNetProxyLocator * This);
        
        DECLSPEC_XFGVIRT(IMFNetProxyLocator, RegisterProxyResult)
        HRESULT ( STDMETHODCALLTYPE *RegisterProxyResult )( 
            IMFNetProxyLocator * This,
            /* [in] */ HRESULT hrOp);
        
        DECLSPEC_XFGVIRT(IMFNetProxyLocator, GetCurrentProxy)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentProxy )( 
            IMFNetProxyLocator * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_opt_(*pcchStr)  LPWSTR pszStr,
            /* [out][in] */ DWORD *pcchStr);
        
        DECLSPEC_XFGVIRT(IMFNetProxyLocator, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IMFNetProxyLocator * This,
            /* [out] */ IMFNetProxyLocator **ppProxyLocator);
        
        END_INTERFACE
    } IMFNetProxyLocatorVtbl;

    interface IMFNetProxyLocator
    {
        CONST_VTBL struct IMFNetProxyLocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetProxyLocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetProxyLocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetProxyLocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetProxyLocator_FindFirstProxy(This,pszHost,pszUrl,fReserved)	\
    ( (This)->lpVtbl -> FindFirstProxy(This,pszHost,pszUrl,fReserved) ) 

#define IMFNetProxyLocator_FindNextProxy(This)	\
    ( (This)->lpVtbl -> FindNextProxy(This) ) 

#define IMFNetProxyLocator_RegisterProxyResult(This,hrOp)	\
    ( (This)->lpVtbl -> RegisterProxyResult(This,hrOp) ) 

#define IMFNetProxyLocator_GetCurrentProxy(This,pszStr,pcchStr)	\
    ( (This)->lpVtbl -> GetCurrentProxy(This,pszStr,pcchStr) ) 

#define IMFNetProxyLocator_Clone(This,ppProxyLocator)	\
    ( (This)->lpVtbl -> Clone(This,ppProxyLocator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetProxyLocator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0067 */
/* [local] */ 

STDAPI MFCreateProxyLocator(
    LPCWSTR pszProtocol,
    IPropertyStore* pProxyConfig, 
    _Outptr_ IMFNetProxyLocator** ppProxyLocator );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0067_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0067_v0_0_s_ifspec;

#ifndef __IMFNetProxyLocatorFactory_INTERFACE_DEFINED__
#define __IMFNetProxyLocatorFactory_INTERFACE_DEFINED__

/* interface IMFNetProxyLocatorFactory */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFNetProxyLocatorFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e9cd0384-a268-4bb4-82de-658d53574d41")
    IMFNetProxyLocatorFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateProxyLocator( 
            /* [in] */ LPCWSTR pszProtocol,
            /* [annotation][out] */ 
            _Outptr_  IMFNetProxyLocator **ppProxyLocator) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetProxyLocatorFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetProxyLocatorFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetProxyLocatorFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetProxyLocatorFactory * This);
        
        DECLSPEC_XFGVIRT(IMFNetProxyLocatorFactory, CreateProxyLocator)
        HRESULT ( STDMETHODCALLTYPE *CreateProxyLocator )( 
            IMFNetProxyLocatorFactory * This,
            /* [in] */ LPCWSTR pszProtocol,
            /* [annotation][out] */ 
            _Outptr_  IMFNetProxyLocator **ppProxyLocator);
        
        END_INTERFACE
    } IMFNetProxyLocatorFactoryVtbl;

    interface IMFNetProxyLocatorFactory
    {
        CONST_VTBL struct IMFNetProxyLocatorFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetProxyLocatorFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetProxyLocatorFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetProxyLocatorFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetProxyLocatorFactory_CreateProxyLocator(This,pszProtocol,ppProxyLocator)	\
    ( (This)->lpVtbl -> CreateProxyLocator(This,pszProtocol,ppProxyLocator) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetProxyLocatorFactory_INTERFACE_DEFINED__ */


#ifndef __IMFSaveJob_INTERFACE_DEFINED__
#define __IMFSaveJob_INTERFACE_DEFINED__

/* interface IMFSaveJob */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSaveJob;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e9931663-80bf-4c6e-98af-5dcf58747d1f")
    IMFSaveJob : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginSave( 
            /* [in] */ IMFByteStream *pStream,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndSave( 
            /* [in] */ IMFAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelSave( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProgress( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPercentComplete) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSaveJobVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSaveJob * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSaveJob * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSaveJob * This);
        
        DECLSPEC_XFGVIRT(IMFSaveJob, BeginSave)
        HRESULT ( STDMETHODCALLTYPE *BeginSave )( 
            IMFSaveJob * This,
            /* [in] */ IMFByteStream *pStream,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFSaveJob, EndSave)
        HRESULT ( STDMETHODCALLTYPE *EndSave )( 
            IMFSaveJob * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFSaveJob, CancelSave)
        HRESULT ( STDMETHODCALLTYPE *CancelSave )( 
            IMFSaveJob * This);
        
        DECLSPEC_XFGVIRT(IMFSaveJob, GetProgress)
        HRESULT ( STDMETHODCALLTYPE *GetProgress )( 
            IMFSaveJob * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPercentComplete);
        
        END_INTERFACE
    } IMFSaveJobVtbl;

    interface IMFSaveJob
    {
        CONST_VTBL struct IMFSaveJobVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSaveJob_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSaveJob_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSaveJob_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSaveJob_BeginSave(This,pStream,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginSave(This,pStream,pCallback,pState) ) 

#define IMFSaveJob_EndSave(This,pResult)	\
    ( (This)->lpVtbl -> EndSave(This,pResult) ) 

#define IMFSaveJob_CancelSave(This)	\
    ( (This)->lpVtbl -> CancelSave(This) ) 

#define IMFSaveJob_GetProgress(This,pdwPercentComplete)	\
    ( (This)->lpVtbl -> GetProgress(This,pdwPercentComplete) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSaveJob_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0069 */
/* [local] */ 

EXTERN_GUID( MFNET_SAVEJOB_SERVICE, 0xb85a587f, 0x3d02, 0x4e52, 0x95, 0x65, 0x55, 0xd3, 0xec, 0x1e, 0x7f, 0xf7 );
typedef 
enum _MFNETSOURCE_PROTOCOL_TYPE
    {
        MFNETSOURCE_UNDEFINED	= 0,
        MFNETSOURCE_HTTP	= 0x1,
        MFNETSOURCE_RTSP	= 0x2,
        MFNETSOURCE_FILE	= 0x3,
        MFNETSOURCE_MULTICAST	= 0x4
    } 	MFNETSOURCE_PROTOCOL_TYPE;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0069_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0069_v0_0_s_ifspec;

#ifndef __IMFNetSchemeHandlerConfig_INTERFACE_DEFINED__
#define __IMFNetSchemeHandlerConfig_INTERFACE_DEFINED__

/* interface IMFNetSchemeHandlerConfig */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFNetSchemeHandlerConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7BE19E73-C9BF-468a-AC5A-A5E8653BEC87")
    IMFNetSchemeHandlerConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfSupportedProtocols( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcProtocols) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSupportedProtocolType( 
            /* [in] */ ULONG nProtocolIndex,
            /* [annotation][out] */ 
            _Out_  MFNETSOURCE_PROTOCOL_TYPE *pnProtocolType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetProtocolRolloverSettings( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetSchemeHandlerConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetSchemeHandlerConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetSchemeHandlerConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetSchemeHandlerConfig * This);
        
        DECLSPEC_XFGVIRT(IMFNetSchemeHandlerConfig, GetNumberOfSupportedProtocols)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfSupportedProtocols )( 
            IMFNetSchemeHandlerConfig * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcProtocols);
        
        DECLSPEC_XFGVIRT(IMFNetSchemeHandlerConfig, GetSupportedProtocolType)
        HRESULT ( STDMETHODCALLTYPE *GetSupportedProtocolType )( 
            IMFNetSchemeHandlerConfig * This,
            /* [in] */ ULONG nProtocolIndex,
            /* [annotation][out] */ 
            _Out_  MFNETSOURCE_PROTOCOL_TYPE *pnProtocolType);
        
        DECLSPEC_XFGVIRT(IMFNetSchemeHandlerConfig, ResetProtocolRolloverSettings)
        HRESULT ( STDMETHODCALLTYPE *ResetProtocolRolloverSettings )( 
            IMFNetSchemeHandlerConfig * This);
        
        END_INTERFACE
    } IMFNetSchemeHandlerConfigVtbl;

    interface IMFNetSchemeHandlerConfig
    {
        CONST_VTBL struct IMFNetSchemeHandlerConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetSchemeHandlerConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetSchemeHandlerConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetSchemeHandlerConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetSchemeHandlerConfig_GetNumberOfSupportedProtocols(This,pcProtocols)	\
    ( (This)->lpVtbl -> GetNumberOfSupportedProtocols(This,pcProtocols) ) 

#define IMFNetSchemeHandlerConfig_GetSupportedProtocolType(This,nProtocolIndex,pnProtocolType)	\
    ( (This)->lpVtbl -> GetSupportedProtocolType(This,nProtocolIndex,pnProtocolType) ) 

#define IMFNetSchemeHandlerConfig_ResetProtocolRolloverSettings(This)	\
    ( (This)->lpVtbl -> ResetProtocolRolloverSettings(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetSchemeHandlerConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0070 */
/* [local] */ 

STDAPI MFCreateNetSchemePlugin(
    REFIID riid, 
    LPVOID *ppvHandler );
typedef 
enum _MFNETSOURCE_TRANSPORT_TYPE
    {
        MFNETSOURCE_UDP	= 0,
        MFNETSOURCE_TCP	= ( MFNETSOURCE_UDP + 1 ) 
    } 	MFNETSOURCE_TRANSPORT_TYPE;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum _MFNETSOURCE_CACHE_STATE
    {
        MFNETSOURCE_CACHE_UNAVAILABLE	= 0,
        MFNETSOURCE_CACHE_ACTIVE_WRITING	= ( MFNETSOURCE_CACHE_UNAVAILABLE + 1 ) ,
        MFNETSOURCE_CACHE_ACTIVE_COMPLETE	= ( MFNETSOURCE_CACHE_ACTIVE_WRITING + 1 ) 
    } 	MFNETSOURCE_CACHE_STATE;

typedef 
enum _MFNETSOURCE_STATISTICS_IDS
    {
        MFNETSOURCE_RECVPACKETS_ID	= 0,
        MFNETSOURCE_LOSTPACKETS_ID	= ( MFNETSOURCE_RECVPACKETS_ID + 1 ) ,
        MFNETSOURCE_RESENDSREQUESTED_ID	= ( MFNETSOURCE_LOSTPACKETS_ID + 1 ) ,
        MFNETSOURCE_RESENDSRECEIVED_ID	= ( MFNETSOURCE_RESENDSREQUESTED_ID + 1 ) ,
        MFNETSOURCE_RECOVEREDBYECCPACKETS_ID	= ( MFNETSOURCE_RESENDSRECEIVED_ID + 1 ) ,
        MFNETSOURCE_RECOVEREDBYRTXPACKETS_ID	= ( MFNETSOURCE_RECOVEREDBYECCPACKETS_ID + 1 ) ,
        MFNETSOURCE_OUTPACKETS_ID	= ( MFNETSOURCE_RECOVEREDBYRTXPACKETS_ID + 1 ) ,
        MFNETSOURCE_RECVRATE_ID	= ( MFNETSOURCE_OUTPACKETS_ID + 1 ) ,
        MFNETSOURCE_AVGBANDWIDTHBPS_ID	= ( MFNETSOURCE_RECVRATE_ID + 1 ) ,
        MFNETSOURCE_BYTESRECEIVED_ID	= ( MFNETSOURCE_AVGBANDWIDTHBPS_ID + 1 ) ,
        MFNETSOURCE_PROTOCOL_ID	= ( MFNETSOURCE_BYTESRECEIVED_ID + 1 ) ,
        MFNETSOURCE_TRANSPORT_ID	= ( MFNETSOURCE_PROTOCOL_ID + 1 ) ,
        MFNETSOURCE_CACHE_STATE_ID	= ( MFNETSOURCE_TRANSPORT_ID + 1 ) ,
        MFNETSOURCE_LINKBANDWIDTH_ID	= ( MFNETSOURCE_CACHE_STATE_ID + 1 ) ,
        MFNETSOURCE_CONTENTBITRATE_ID	= ( MFNETSOURCE_LINKBANDWIDTH_ID + 1 ) ,
        MFNETSOURCE_SPEEDFACTOR_ID	= ( MFNETSOURCE_CONTENTBITRATE_ID + 1 ) ,
        MFNETSOURCE_BUFFERSIZE_ID	= ( MFNETSOURCE_SPEEDFACTOR_ID + 1 ) ,
        MFNETSOURCE_BUFFERPROGRESS_ID	= ( MFNETSOURCE_BUFFERSIZE_ID + 1 ) ,
        MFNETSOURCE_LASTBWSWITCHTS_ID	= ( MFNETSOURCE_BUFFERPROGRESS_ID + 1 ) ,
        MFNETSOURCE_SEEKRANGESTART_ID	= ( MFNETSOURCE_LASTBWSWITCHTS_ID + 1 ) ,
        MFNETSOURCE_SEEKRANGEEND_ID	= ( MFNETSOURCE_SEEKRANGESTART_ID + 1 ) ,
        MFNETSOURCE_BUFFERINGCOUNT_ID	= ( MFNETSOURCE_SEEKRANGEEND_ID + 1 ) ,
        MFNETSOURCE_INCORRECTLYSIGNEDPACKETS_ID	= ( MFNETSOURCE_BUFFERINGCOUNT_ID + 1 ) ,
        MFNETSOURCE_SIGNEDSESSION_ID	= ( MFNETSOURCE_INCORRECTLYSIGNEDPACKETS_ID + 1 ) ,
        MFNETSOURCE_MAXBITRATE_ID	= ( MFNETSOURCE_SIGNEDSESSION_ID + 1 ) ,
        MFNETSOURCE_RECEPTION_QUALITY_ID	= ( MFNETSOURCE_MAXBITRATE_ID + 1 ) ,
        MFNETSOURCE_RECOVEREDPACKETS_ID	= ( MFNETSOURCE_RECEPTION_QUALITY_ID + 1 ) ,
        MFNETSOURCE_VBR_ID	= ( MFNETSOURCE_RECOVEREDPACKETS_ID + 1 ) ,
        MFNETSOURCE_DOWNLOADPROGRESS_ID	= ( MFNETSOURCE_VBR_ID + 1 ) ,
        MFNETSOURCE_UNPREDEFINEDPROTOCOLNAME_ID	= ( MFNETSOURCE_DOWNLOADPROGRESS_ID + 1 ) 
    } 	MFNETSOURCE_STATISTICS_IDS;

EXTERN_GUID( MFNETSOURCE_STATISTICS_SERVICE, 0x3cb1f275, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_STATISTICS, 0x3cb1f274, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MFNETSOURCE_BUFFERINGTIME, 0x3cb1f276, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ACCELERATEDSTREAMINGDURATION, 0x3cb1f277, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_MAXUDPACCELERATEDSTREAMINGDURATION, 0x4aab2879, 0xbbe1, 0x4994, 0x9f, 0xf0, 0x54, 0x95, 0xbd, 0x25, 0x1, 0x29 );
EXTERN_GUID( MFNETSOURCE_MAXBUFFERTIMEMS, 0x408b24e6, 0x4038, 0x4401, 0xb5, 0xb2, 0xfe, 0x70, 0x1a, 0x9e, 0xbf, 0x10 );
EXTERN_GUID( MFNETSOURCE_CONNECTIONBANDWIDTH, 0x3cb1f278, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_CACHEENABLED, 0x3cb1f279, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_AUTORECONNECTLIMIT, 0x3cb1f27a, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_RESENDSENABLED, 0x3cb1f27b, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_THINNINGENABLED, 0x3cb1f27c, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROTOCOL, 0x3cb1f27d, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_TRANSPORT, 0x3cb1f27e, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MFNETSOURCE_PREVIEWMODEENABLED, 0x3cb1f27f, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MFNETSOURCE_CREDENTIAL_MANAGER, 0x3cb1f280, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PPBANDWIDTH, 0x3cb1f281, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_AUTORECONNECTPROGRESS, 0x3cb1f282, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYLOCATORFACTORY, 0x3cb1f283, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_BROWSERUSERAGENT, 0x3cb1f28b, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_BROWSERWEBPAGE, 0x3cb1f28c, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PLAYERVERSION, 0x3cb1f28d, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PLAYERID, 0x3cb1f28e, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_HOSTEXE, 0x3cb1f28f, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_HOSTVERSION, 0x3cb1f291, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PLAYERUSERAGENT, 0x3cb1f292, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MFNETSOURCE_CLIENTGUID, 0x60a2c4a6, 0xf197, 0x4c14, 0xa5, 0xbf, 0x88, 0x83, 0xd, 0x24, 0x58, 0xaf );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MFNETSOURCE_LOGURL, 0x3cb1f293, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_UDP, 0x3cb1f294, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_TCP, 0x3cb1f295, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_MSB, 0x3cb1f296, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_RTSP, 0x3cb1f298, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_HTTP, 0x3cb1f299, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_STREAMING, 0x3cb1f29c, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_DOWNLOAD, 0x3cb1f29d, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_ENABLE_PRIVATEMODE, 0x824779d8, 0xf18b, 0x4405, 0x8c, 0xf1, 0x46, 0x4f, 0xb5, 0xaa, 0x8f, 0x71 );
EXTERN_GUID( MFNETSOURCE_UDP_PORT_RANGE, 0x3cb1f29a, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYINFO, 0x3cb1f29b, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_DRMNET_LICENSE_REPRESENTATION, 0x47eae1bd, 0xbdfe, 0x42e2, 0x82, 0xf3, 0x54, 0xa4, 0x8c, 0x17, 0x96, 0x2d );
EXTERN_GUID( MFNETSOURCE_PROXYSETTINGS, 0x3cb1f287, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYHOSTNAME, 0x3cb1f284, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYPORT, 0x3cb1f288, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYEXCEPTIONLIST, 0x3cb1f285, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYBYPASSFORLOCAL, 0x3cb1f286, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
EXTERN_GUID( MFNETSOURCE_PROXYRERUNAUTODETECTION, 0x3cb1f289, 0x0505, 0x4c5d, 0xae, 0x71, 0x0a, 0x55, 0x63, 0x44, 0xef, 0xa1 );
#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID( MFNETSOURCE_STREAM_LANGUAGE, 0x9ab44318, 0xf7cd, 0x4f2d, 0x8d, 0x6d, 0xfa, 0x35, 0xb4, 0x92, 0xce, 0xcb );
EXTERN_GUID( MFNETSOURCE_LOGPARAMS, 0x64936ae8, 0x9418, 0x453a, 0x8c, 0xda, 0x3e, 0xa, 0x66, 0x8b, 0x35, 0x3b );
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MFNETSOURCE_PEERMANAGER, 0x48b29adb, 0xfebf, 0x45ee, 0xa9, 0xbf, 0xef, 0xb8, 0x1c, 0x49, 0x2e, 0xfc );
EXTERN_GUID( MFNETSOURCE_FRIENDLYNAME, 0x5b2a7757, 0xbc6b, 0x447e, 0xaa, 0x06, 0x0d, 0xda, 0x1c, 0x64, 0x6e, 0x2f );
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
typedef 
enum _MFNET_PROXYSETTINGS
    {
        MFNET_PROXYSETTING_NONE	= 0,
        MFNET_PROXYSETTING_MANUAL	= 1,
        MFNET_PROXYSETTING_AUTO	= 2,
        MFNET_PROXYSETTING_BROWSER	= 3
    } 	MFNET_PROXYSETTINGS;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0070_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0070_v0_0_s_ifspec;

#ifndef __IMFSchemeHandler_INTERFACE_DEFINED__
#define __IMFSchemeHandler_INTERFACE_DEFINED__

/* interface IMFSchemeHandler */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSchemeHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6D4C7B74-52A0-4bb7-B0DB-55F29F47A668")
    IMFSchemeHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginCreateObject( 
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndCreateObject( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelObjectCreation( 
            /* [in] */ IUnknown *pIUnknownCancelCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSchemeHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSchemeHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSchemeHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSchemeHandler * This);
        
        DECLSPEC_XFGVIRT(IMFSchemeHandler, BeginCreateObject)
        HRESULT ( STDMETHODCALLTYPE *BeginCreateObject )( 
            IMFSchemeHandler * This,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFSchemeHandler, EndCreateObject)
        HRESULT ( STDMETHODCALLTYPE *EndCreateObject )( 
            IMFSchemeHandler * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFSchemeHandler, CancelObjectCreation)
        HRESULT ( STDMETHODCALLTYPE *CancelObjectCreation )( 
            IMFSchemeHandler * This,
            /* [in] */ IUnknown *pIUnknownCancelCookie);
        
        END_INTERFACE
    } IMFSchemeHandlerVtbl;

    interface IMFSchemeHandler
    {
        CONST_VTBL struct IMFSchemeHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSchemeHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSchemeHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSchemeHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSchemeHandler_BeginCreateObject(This,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginCreateObject(This,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState) ) 

#define IMFSchemeHandler_EndCreateObject(This,pResult,pObjectType,ppObject)	\
    ( (This)->lpVtbl -> EndCreateObject(This,pResult,pObjectType,ppObject) ) 

#define IMFSchemeHandler_CancelObjectCreation(This,pIUnknownCancelCookie)	\
    ( (This)->lpVtbl -> CancelObjectCreation(This,pIUnknownCancelCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSchemeHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0071 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_GUID(MF_BYTESTREAMHANDLER_ACCEPTS_SHARE_WRITE, 0xa6e1f733, 0x3001, 0x4915, 0x81, 0x50, 0x15, 0x58, 0xa2, 0x18, 0xe, 0xc8);
#endif // (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0071_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0071_v0_0_s_ifspec;

#ifndef __IMFByteStreamHandler_INTERFACE_DEFINED__
#define __IMFByteStreamHandler_INTERFACE_DEFINED__

/* interface IMFByteStreamHandler */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFByteStreamHandler;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BB420AA4-765B-4a1f-91FE-D6A8A143924C")
    IMFByteStreamHandler : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginCreateObject( 
            /* [in] */ IMFByteStream *pByteStream,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndCreateObject( 
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelObjectCreation( 
            /* [in] */ IUnknown *pIUnknownCancelCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxNumberOfBytesRequiredForResolution( 
            /* [annotation][out] */ 
            _Out_  QWORD *pqwBytes) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFByteStreamHandlerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFByteStreamHandler * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFByteStreamHandler * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFByteStreamHandler * This);
        
        DECLSPEC_XFGVIRT(IMFByteStreamHandler, BeginCreateObject)
        HRESULT ( STDMETHODCALLTYPE *BeginCreateObject )( 
            IMFByteStreamHandler * This,
            /* [in] */ IMFByteStream *pByteStream,
            /* [in] */ LPCWSTR pwszURL,
            /* [in] */ DWORD dwFlags,
            /* [in] */ IPropertyStore *pProps,
            /* [annotation][out] */ 
            _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFByteStreamHandler, EndCreateObject)
        HRESULT ( STDMETHODCALLTYPE *EndCreateObject )( 
            IMFByteStreamHandler * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  MF_OBJECT_TYPE *pObjectType,
            /* [annotation][out] */ 
            _Outptr_  IUnknown **ppObject);
        
        DECLSPEC_XFGVIRT(IMFByteStreamHandler, CancelObjectCreation)
        HRESULT ( STDMETHODCALLTYPE *CancelObjectCreation )( 
            IMFByteStreamHandler * This,
            /* [in] */ IUnknown *pIUnknownCancelCookie);
        
        DECLSPEC_XFGVIRT(IMFByteStreamHandler, GetMaxNumberOfBytesRequiredForResolution)
        HRESULT ( STDMETHODCALLTYPE *GetMaxNumberOfBytesRequiredForResolution )( 
            IMFByteStreamHandler * This,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwBytes);
        
        END_INTERFACE
    } IMFByteStreamHandlerVtbl;

    interface IMFByteStreamHandler
    {
        CONST_VTBL struct IMFByteStreamHandlerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFByteStreamHandler_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFByteStreamHandler_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFByteStreamHandler_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFByteStreamHandler_BeginCreateObject(This,pByteStream,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginCreateObject(This,pByteStream,pwszURL,dwFlags,pProps,ppIUnknownCancelCookie,pCallback,punkState) ) 

#define IMFByteStreamHandler_EndCreateObject(This,pResult,pObjectType,ppObject)	\
    ( (This)->lpVtbl -> EndCreateObject(This,pResult,pObjectType,ppObject) ) 

#define IMFByteStreamHandler_CancelObjectCreation(This,pIUnknownCancelCookie)	\
    ( (This)->lpVtbl -> CancelObjectCreation(This,pIUnknownCancelCookie) ) 

#define IMFByteStreamHandler_GetMaxNumberOfBytesRequiredForResolution(This,pqwBytes)	\
    ( (This)->lpVtbl -> GetMaxNumberOfBytesRequiredForResolution(This,pqwBytes) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFByteStreamHandler_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0072 */
/* [local] */ 

EXTERN_GUID( MF_BYTESTREAM_SERVICE, 0xab025e2b, 0x16d9, 0x4180, 0xa1, 0x27, 0xba, 0x6c, 0x70, 0x15, 0x61, 0x61 );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0072_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0072_v0_0_s_ifspec;

#ifndef __IMFTrustedInput_INTERFACE_DEFINED__
#define __IMFTrustedInput_INTERFACE_DEFINED__

/* interface IMFTrustedInput */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFTrustedInput;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("542612C4-A1B8-4632-B521-DE11EA64A0B0")
    IMFTrustedInput : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInputTrustAuthority( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunkObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTrustedInputVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFTrustedInput * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFTrustedInput * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFTrustedInput * This);
        
        DECLSPEC_XFGVIRT(IMFTrustedInput, GetInputTrustAuthority)
        HRESULT ( STDMETHODCALLTYPE *GetInputTrustAuthority )( 
            __RPC__in IMFTrustedInput * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt IUnknown **ppunkObject);
        
        END_INTERFACE
    } IMFTrustedInputVtbl;

    interface IMFTrustedInput
    {
        CONST_VTBL struct IMFTrustedInputVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTrustedInput_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTrustedInput_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTrustedInput_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTrustedInput_GetInputTrustAuthority(This,dwStreamID,riid,ppunkObject)	\
    ( (This)->lpVtbl -> GetInputTrustAuthority(This,dwStreamID,riid,ppunkObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTrustedInput_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0073 */
/* [local] */ 

typedef 
enum _MFPOLICYMANAGER_ACTION
    {
        PEACTION_NO	= 0,
        PEACTION_PLAY	= 1,
        PEACTION_COPY	= 2,
        PEACTION_EXPORT	= 3,
        PEACTION_EXTRACT	= 4,
        PEACTION_RESERVED1	= 5,
        PEACTION_RESERVED2	= 6,
        PEACTION_RESERVED3	= 7,
        PEACTION_LAST	= 7
    } 	MFPOLICYMANAGER_ACTION;

typedef struct _MFINPUTTRUSTAUTHORITY_ACTION
    {
    MFPOLICYMANAGER_ACTION Action;
    BYTE *pbTicket;
    DWORD cbTicket;
    } 	MFINPUTTRUSTAUTHORITY_ACCESS_ACTION;

typedef struct _MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS
    {
    DWORD dwSize;
    DWORD dwVer;
    DWORD cbSignatureOffset;
    DWORD cbSignatureSize;
    DWORD cbExtensionOffset;
    DWORD cbExtensionSize;
    DWORD cActions;
    MFINPUTTRUSTAUTHORITY_ACCESS_ACTION rgOutputActions[ 1 ];
    } 	MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS;


EXTERN_GUID( MF_MEDIA_PROTECTION_MANAGER_PROPERTIES, 0x38BD81A9, 0xACEA, 0x4C73, 0x89, 0xB2, 0x55, 0x32, 0xC0, 0xAE, 0xCA, 0x79 );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0073_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0073_v0_0_s_ifspec;

#ifndef __IMFInputTrustAuthority_INTERFACE_DEFINED__
#define __IMFInputTrustAuthority_INTERFACE_DEFINED__

/* interface IMFInputTrustAuthority */
/* [helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFInputTrustAuthority;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D19F8E98-B126-4446-890C-5DCB7AD71453")
    IMFInputTrustAuthority : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetDecrypter( 
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE RequestAccess( 
            /* [in] */ MFPOLICYMANAGER_ACTION Action,
            /* [annotation][out] */ 
            _Outptr_  IMFActivate **ppContentEnablerActivate) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetPolicy( 
            /* [in] */ MFPOLICYMANAGER_ACTION Action,
            /* [annotation][out] */ 
            _Outptr_  IMFOutputPolicy **ppPolicy) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE BindAccess( 
            /* [annotation][in] */ 
            _In_  MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS *pParam) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE UpdateAccess( 
            /* [annotation][in] */ 
            _In_  MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS *pParam) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFInputTrustAuthorityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFInputTrustAuthority * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFInputTrustAuthority * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFInputTrustAuthority * This);
        
        DECLSPEC_XFGVIRT(IMFInputTrustAuthority, GetDecrypter)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetDecrypter )( 
            IMFInputTrustAuthority * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IMFInputTrustAuthority, RequestAccess)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RequestAccess )( 
            IMFInputTrustAuthority * This,
            /* [in] */ MFPOLICYMANAGER_ACTION Action,
            /* [annotation][out] */ 
            _Outptr_  IMFActivate **ppContentEnablerActivate);
        
        DECLSPEC_XFGVIRT(IMFInputTrustAuthority, GetPolicy)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetPolicy )( 
            IMFInputTrustAuthority * This,
            /* [in] */ MFPOLICYMANAGER_ACTION Action,
            /* [annotation][out] */ 
            _Outptr_  IMFOutputPolicy **ppPolicy);
        
        DECLSPEC_XFGVIRT(IMFInputTrustAuthority, BindAccess)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BindAccess )( 
            IMFInputTrustAuthority * This,
            /* [annotation][in] */ 
            _In_  MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS *pParam);
        
        DECLSPEC_XFGVIRT(IMFInputTrustAuthority, UpdateAccess)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *UpdateAccess )( 
            IMFInputTrustAuthority * This,
            /* [annotation][in] */ 
            _In_  MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS *pParam);
        
        DECLSPEC_XFGVIRT(IMFInputTrustAuthority, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            __RPC__in IMFInputTrustAuthority * This);
        
        END_INTERFACE
    } IMFInputTrustAuthorityVtbl;

    interface IMFInputTrustAuthority
    {
        CONST_VTBL struct IMFInputTrustAuthorityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFInputTrustAuthority_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFInputTrustAuthority_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFInputTrustAuthority_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFInputTrustAuthority_GetDecrypter(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetDecrypter(This,riid,ppv) ) 

#define IMFInputTrustAuthority_RequestAccess(This,Action,ppContentEnablerActivate)	\
    ( (This)->lpVtbl -> RequestAccess(This,Action,ppContentEnablerActivate) ) 

#define IMFInputTrustAuthority_GetPolicy(This,Action,ppPolicy)	\
    ( (This)->lpVtbl -> GetPolicy(This,Action,ppPolicy) ) 

#define IMFInputTrustAuthority_BindAccess(This,pParam)	\
    ( (This)->lpVtbl -> BindAccess(This,pParam) ) 

#define IMFInputTrustAuthority_UpdateAccess(This,pParam)	\
    ( (This)->lpVtbl -> UpdateAccess(This,pParam) ) 

#define IMFInputTrustAuthority_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFInputTrustAuthority_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0074 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0074_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0074_v0_0_s_ifspec;

#ifndef __IMFTrustedOutput_INTERFACE_DEFINED__
#define __IMFTrustedOutput_INTERFACE_DEFINED__

/* interface IMFTrustedOutput */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFTrustedOutput;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D19F8E95-B126-4446-890C-5DCB7AD71453")
    IMFTrustedOutput : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOutputTrustAuthorityCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputTrustAuthorities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputTrustAuthorityByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFOutputTrustAuthority **ppauthority) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsFinal( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsFinal) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTrustedOutputVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTrustedOutput * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTrustedOutput * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTrustedOutput * This);
        
        DECLSPEC_XFGVIRT(IMFTrustedOutput, GetOutputTrustAuthorityCount)
        HRESULT ( STDMETHODCALLTYPE *GetOutputTrustAuthorityCount )( 
            IMFTrustedOutput * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcOutputTrustAuthorities);
        
        DECLSPEC_XFGVIRT(IMFTrustedOutput, GetOutputTrustAuthorityByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetOutputTrustAuthorityByIndex )( 
            IMFTrustedOutput * This,
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_  IMFOutputTrustAuthority **ppauthority);
        
        DECLSPEC_XFGVIRT(IMFTrustedOutput, IsFinal)
        HRESULT ( STDMETHODCALLTYPE *IsFinal )( 
            IMFTrustedOutput * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsFinal);
        
        END_INTERFACE
    } IMFTrustedOutputVtbl;

    interface IMFTrustedOutput
    {
        CONST_VTBL struct IMFTrustedOutputVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTrustedOutput_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTrustedOutput_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTrustedOutput_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTrustedOutput_GetOutputTrustAuthorityCount(This,pcOutputTrustAuthorities)	\
    ( (This)->lpVtbl -> GetOutputTrustAuthorityCount(This,pcOutputTrustAuthorities) ) 

#define IMFTrustedOutput_GetOutputTrustAuthorityByIndex(This,dwIndex,ppauthority)	\
    ( (This)->lpVtbl -> GetOutputTrustAuthorityByIndex(This,dwIndex,ppauthority) ) 

#define IMFTrustedOutput_IsFinal(This,pfIsFinal)	\
    ( (This)->lpVtbl -> IsFinal(This,pfIsFinal) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTrustedOutput_INTERFACE_DEFINED__ */


#ifndef __IMFOutputTrustAuthority_INTERFACE_DEFINED__
#define __IMFOutputTrustAuthority_INTERFACE_DEFINED__

/* interface IMFOutputTrustAuthority */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFOutputTrustAuthority;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D19F8E94-B126-4446-890C-5DCB7AD71453")
    IMFOutputTrustAuthority : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAction( 
            /* [annotation][out] */ 
            _Out_  MFPOLICYMANAGER_ACTION *pAction) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPolicy( 
            /* [annotation][in] */ 
            _In_reads_opt_(nPolicy)  IMFOutputPolicy **ppPolicy,
            /* [in] */ DWORD nPolicy,
            /* [annotation][size_is][size_is][unique][out] */ 
            _Outptr_opt_result_bytebuffer_(*pcbTicket)  BYTE **ppbTicket,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbTicket) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFOutputTrustAuthorityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFOutputTrustAuthority * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFOutputTrustAuthority * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFOutputTrustAuthority * This);
        
        DECLSPEC_XFGVIRT(IMFOutputTrustAuthority, GetAction)
        HRESULT ( STDMETHODCALLTYPE *GetAction )( 
            IMFOutputTrustAuthority * This,
            /* [annotation][out] */ 
            _Out_  MFPOLICYMANAGER_ACTION *pAction);
        
        DECLSPEC_XFGVIRT(IMFOutputTrustAuthority, SetPolicy)
        HRESULT ( STDMETHODCALLTYPE *SetPolicy )( 
            IMFOutputTrustAuthority * This,
            /* [annotation][in] */ 
            _In_reads_opt_(nPolicy)  IMFOutputPolicy **ppPolicy,
            /* [in] */ DWORD nPolicy,
            /* [annotation][size_is][size_is][unique][out] */ 
            _Outptr_opt_result_bytebuffer_(*pcbTicket)  BYTE **ppbTicket,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pcbTicket);
        
        END_INTERFACE
    } IMFOutputTrustAuthorityVtbl;

    interface IMFOutputTrustAuthority
    {
        CONST_VTBL struct IMFOutputTrustAuthorityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFOutputTrustAuthority_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFOutputTrustAuthority_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFOutputTrustAuthority_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFOutputTrustAuthority_GetAction(This,pAction)	\
    ( (This)->lpVtbl -> GetAction(This,pAction) ) 

#define IMFOutputTrustAuthority_SetPolicy(This,ppPolicy,nPolicy,ppbTicket,pcbTicket)	\
    ( (This)->lpVtbl -> SetPolicy(This,ppPolicy,nPolicy,ppbTicket,pcbTicket) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFOutputTrustAuthority_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0076 */
/* [local] */ 




extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0076_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0076_v0_0_s_ifspec;

#ifndef __IMFOutputPolicy_INTERFACE_DEFINED__
#define __IMFOutputPolicy_INTERFACE_DEFINED__

/* interface IMFOutputPolicy */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFOutputPolicy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7F00F10A-DAED-41AF-AB26-5FDFA4DFBA3C")
    IMFOutputPolicy : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GenerateRequiredSchemas( 
            /* [in] */ DWORD dwAttributes,
            /* [in] */ GUID guidOutputSubType,
            /* [in] */ GUID *rgGuidProtectionSchemasSupported,
            /* [in] */ DWORD cProtectionSchemasSupported,
            /* [annotation][out] */ 
            _Outptr_  IMFCollection **ppRequiredProtectionSchemas) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginatorID( 
            /* [annotation][out] */ 
            _Out_  GUID *pguidOriginatorID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMinimumGRLVersion( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwMinimumGRLVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFOutputPolicyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFOutputPolicy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFOutputPolicy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFOutputPolicy * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFOutputPolicy * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFOutputPolicy * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFOutputPolicy * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFOutputPolicy * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFOutputPolicy * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFOutputPolicy * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFOutputPolicy * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFOutputPolicy * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFOutputPolicy * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFOutputPolicy, GenerateRequiredSchemas)
        HRESULT ( STDMETHODCALLTYPE *GenerateRequiredSchemas )( 
            IMFOutputPolicy * This,
            /* [in] */ DWORD dwAttributes,
            /* [in] */ GUID guidOutputSubType,
            /* [in] */ GUID *rgGuidProtectionSchemasSupported,
            /* [in] */ DWORD cProtectionSchemasSupported,
            /* [annotation][out] */ 
            _Outptr_  IMFCollection **ppRequiredProtectionSchemas);
        
        DECLSPEC_XFGVIRT(IMFOutputPolicy, GetOriginatorID)
        HRESULT ( STDMETHODCALLTYPE *GetOriginatorID )( 
            IMFOutputPolicy * This,
            /* [annotation][out] */ 
            _Out_  GUID *pguidOriginatorID);
        
        DECLSPEC_XFGVIRT(IMFOutputPolicy, GetMinimumGRLVersion)
        HRESULT ( STDMETHODCALLTYPE *GetMinimumGRLVersion )( 
            IMFOutputPolicy * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwMinimumGRLVersion);
        
        END_INTERFACE
    } IMFOutputPolicyVtbl;

    interface IMFOutputPolicy
    {
        CONST_VTBL struct IMFOutputPolicyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFOutputPolicy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFOutputPolicy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFOutputPolicy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFOutputPolicy_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFOutputPolicy_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFOutputPolicy_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFOutputPolicy_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFOutputPolicy_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFOutputPolicy_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFOutputPolicy_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFOutputPolicy_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFOutputPolicy_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFOutputPolicy_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFOutputPolicy_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFOutputPolicy_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFOutputPolicy_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFOutputPolicy_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFOutputPolicy_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFOutputPolicy_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFOutputPolicy_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFOutputPolicy_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFOutputPolicy_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFOutputPolicy_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFOutputPolicy_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFOutputPolicy_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFOutputPolicy_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFOutputPolicy_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFOutputPolicy_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFOutputPolicy_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFOutputPolicy_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFOutputPolicy_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFOutputPolicy_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFOutputPolicy_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFOutputPolicy_GenerateRequiredSchemas(This,dwAttributes,guidOutputSubType,rgGuidProtectionSchemasSupported,cProtectionSchemasSupported,ppRequiredProtectionSchemas)	\
    ( (This)->lpVtbl -> GenerateRequiredSchemas(This,dwAttributes,guidOutputSubType,rgGuidProtectionSchemasSupported,cProtectionSchemasSupported,ppRequiredProtectionSchemas) ) 

#define IMFOutputPolicy_GetOriginatorID(This,pguidOriginatorID)	\
    ( (This)->lpVtbl -> GetOriginatorID(This,pguidOriginatorID) ) 

#define IMFOutputPolicy_GetMinimumGRLVersion(This,pdwMinimumGRLVersion)	\
    ( (This)->lpVtbl -> GetMinimumGRLVersion(This,pdwMinimumGRLVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFOutputPolicy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0077 */
/* [local] */ 

#define MFOUTPUTATTRIBUTE_DIGITAL               ((DWORD) 0x00000001)
#define MFOUTPUTATTRIBUTE_NONSTANDARDIMPLEMENTATION ((DWORD) 0x00000002)
#define MFOUTPUTATTRIBUTE_VIDEO                 ((DWORD) 0x00000004)
#define MFOUTPUTATTRIBUTE_COMPRESSED            ((DWORD) 0x00000008)
#define MFOUTPUTATTRIBUTE_SOFTWARE              ((DWORD) 0x00000010)
#define MFOUTPUTATTRIBUTE_BUS                   ((DWORD) 0x00000020)
#define MFOUTPUTATTRIBUTE_BUSIMPLEMENTATION     ((DWORD) 0x0000FF00)
EXTERN_GUID( MFCONNECTOR_SPDIF, 0xb94a712, 0xad3e, 0x4cee, 0x83, 0xce, 0xce, 0x32, 0xe3, 0xdb, 0x65, 0x22);

EXTERN_GUID( MFCONNECTOR_UNKNOWN, 0xac3aef5c, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_PCI,  0xac3aef5d, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_PCIX, 0xac3aef5e, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_PCI_Express, 0xac3aef5f, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_AGP, 0xac3aef60, 0xce43, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);


EXTERN_GUID( MFCONNECTOR_VGA, 0x57cd5968, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_SVIDEO, 0x57cd5969, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_COMPOSITE, 0x57cd596a, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_COMPONENT, 0x57cd596b, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_DVI, 0x57cd596c, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_HDMI, 0x57cd596d, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_LVDS, 0x57cd596e, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_D_JPN, 0x57cd5970, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_SDI, 0x57cd5971, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_DISPLAYPORT_EXTERNAL, 0x57cd5972, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_DISPLAYPORT_EMBEDDED, 0x57cd5973, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_UDI_EXTERNAL, 0x57cd5974, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_UDI_EMBEDDED, 0x57cd5975, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_MIRACAST, 0x57cd5977, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98);
EXTERN_GUID( MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A, 0x57cd5978, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98); 
EXTERN_GUID( MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B, 0x57cd5979, 0xce47, 0x11d9, 0x92, 0xdb, 0x00, 0x0b, 0xdb, 0x28, 0xff, 0x98); 

#if (NTDDI_VERSION >= NTDDI_WIN10_VB) 
EXTERN_C const DECLSPEC_SELECTANY GUID MF_POLICY_ID = { 0xb160c24d, 0xc059, 0x48f1, { 0xa9, 0x1, 0x9e, 0xe2, 0x98, 0xa9, 0xa8, 0xc3 } };
#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0077_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0077_v0_0_s_ifspec;

#ifndef __IMFOutputSchema_INTERFACE_DEFINED__
#define __IMFOutputSchema_INTERFACE_DEFINED__

/* interface IMFOutputSchema */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFOutputSchema;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7BE0FC5B-ABD9-44FB-A5C8-F50136E71599")
    IMFOutputSchema : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSchemaType( 
            /* [annotation][out] */ 
            _Out_  GUID *pguidSchemaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetConfigurationData( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOriginatorID( 
            /* [annotation][out] */ 
            _Out_  GUID *pguidOriginatorID) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFOutputSchemaVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFOutputSchema * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFOutputSchema * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFOutputSchema * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFOutputSchema * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFOutputSchema * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFOutputSchema * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFOutputSchema * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFOutputSchema * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFOutputSchema * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFOutputSchema * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFOutputSchema * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFOutputSchema * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFOutputSchema, GetSchemaType)
        HRESULT ( STDMETHODCALLTYPE *GetSchemaType )( 
            IMFOutputSchema * This,
            /* [annotation][out] */ 
            _Out_  GUID *pguidSchemaType);
        
        DECLSPEC_XFGVIRT(IMFOutputSchema, GetConfigurationData)
        HRESULT ( STDMETHODCALLTYPE *GetConfigurationData )( 
            IMFOutputSchema * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVal);
        
        DECLSPEC_XFGVIRT(IMFOutputSchema, GetOriginatorID)
        HRESULT ( STDMETHODCALLTYPE *GetOriginatorID )( 
            IMFOutputSchema * This,
            /* [annotation][out] */ 
            _Out_  GUID *pguidOriginatorID);
        
        END_INTERFACE
    } IMFOutputSchemaVtbl;

    interface IMFOutputSchema
    {
        CONST_VTBL struct IMFOutputSchemaVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFOutputSchema_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFOutputSchema_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFOutputSchema_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFOutputSchema_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFOutputSchema_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFOutputSchema_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFOutputSchema_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFOutputSchema_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFOutputSchema_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFOutputSchema_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFOutputSchema_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFOutputSchema_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFOutputSchema_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFOutputSchema_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFOutputSchema_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFOutputSchema_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFOutputSchema_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFOutputSchema_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFOutputSchema_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFOutputSchema_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFOutputSchema_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFOutputSchema_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFOutputSchema_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFOutputSchema_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFOutputSchema_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFOutputSchema_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFOutputSchema_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFOutputSchema_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFOutputSchema_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFOutputSchema_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFOutputSchema_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFOutputSchema_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFOutputSchema_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFOutputSchema_GetSchemaType(This,pguidSchemaType)	\
    ( (This)->lpVtbl -> GetSchemaType(This,pguidSchemaType) ) 

#define IMFOutputSchema_GetConfigurationData(This,pdwVal)	\
    ( (This)->lpVtbl -> GetConfigurationData(This,pdwVal) ) 

#define IMFOutputSchema_GetOriginatorID(This,pguidOriginatorID)	\
    ( (This)->lpVtbl -> GetOriginatorID(This,pguidOriginatorID) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFOutputSchema_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0078 */
/* [local] */ 


EXTERN_GUID( MFPROTECTION_DISABLE, 0x8cc6d81b, 0xfec6, 0x4d8f, 0x96, 0x4b, 0xcf, 0xba, 0x0b, 0x0d, 0xad, 0x0d);
EXTERN_GUID( MFPROTECTION_CONSTRICTVIDEO, 0x193370ce, 0xc5e4, 0x4c3a, 0x8a, 0x66, 0x69, 0x59, 0xb4, 0xda, 0x44, 0x42);
EXTERN_GUID( MFPROTECTION_CONSTRICTVIDEO_NOOPM, 0xa580e8cd, 0xc247, 0x4957, 0xb9, 0x83, 0x3c, 0x2e, 0xeb, 0xd1, 0xff, 0x59);
EXTERN_GUID( MFPROTECTION_CONSTRICTAUDIO, 0xffc99b44, 0xdf48, 0x4e16, 0x8e, 0x66, 0x09, 0x68, 0x92, 0xc1, 0x57, 0x8a);
EXTERN_GUID( MFPROTECTION_TRUSTEDAUDIODRIVERS, 0x65bdf3d2, 0x0168, 0x4816, 0xa5, 0x33, 0x55, 0xd4, 0x7b, 0x02, 0x71, 0x01);
EXTERN_GUID( MFPROTECTION_HDCP, 0xAE7CC03D, 0xC828, 0x4021, 0xac, 0xb7, 0xd5, 0x78, 0xd2, 0x7a, 0xaf, 0x13);
EXTERN_GUID( MFPROTECTION_CGMSA, 0xE57E69E9, 0x226B, 0x4d31, 0xB4, 0xE3, 0xD3, 0xDB, 0x00, 0x87, 0x36, 0xDD);
EXTERN_GUID( MFPROTECTION_ACP, 0xc3fd11c6, 0xf8b7, 0x4d20, 0xb0, 0x08, 0x1d, 0xb1, 0x7d, 0x61, 0xf2, 0xda);
EXTERN_GUID( MFPROTECTION_WMDRMOTA, 0xa267a6a1, 0x362e, 0x47d0, 0x88, 0x05, 0x46, 0x28, 0x59, 0x8a, 0x23, 0xe4);
EXTERN_GUID( MFPROTECTION_FFT, 0x462a56b2, 0x2866, 0x4bb6, 0x98, 0x0d, 0x6d, 0x8d, 0x9e, 0xdb, 0x1a, 0x8c);
EXTERN_GUID( MFPROTECTION_PROTECTED_SURFACE, 0x4f5d9566, 0xe742, 0x4a25, 0x8d, 0x1f, 0xd2, 0x87, 0xb5, 0xfa, 0x0a, 0xde);
EXTERN_GUID( MFPROTECTION_DISABLE_SCREEN_SCRAPE, 0xa21179a4, 0xb7cd, 0x40d8, 0x96, 0x14, 0x8e, 0xf2, 0x37, 0x1b, 0xa7, 0x8d);
EXTERN_GUID( MFPROTECTION_VIDEO_FRAMES, 0x36a59cbc, 0x7401, 0x4a8c, 0xbc, 0x20, 0x46, 0xa7, 0xc9, 0xe5, 0x97, 0xf0);
EXTERN_GUID(MFPROTECTION_HARDWARE, 0x4ee7f0c1, 0x9ed7, 0x424f, 0xb6, 0xbe, 0x99, 0x6b, 0x33, 0x52, 0x88, 0x56);
EXTERN_GUID( MFPROTECTION_HDCP_WITH_TYPE_ENFORCEMENT, 0xa4a585e8, 0xed60, 0x442d, 0x81, 0x4d, 0xdb, 0x4d, 0x42, 0x20, 0xa0, 0x6d);
typedef 
enum _MF_OPM_CGMSA_PROTECTION_LEVEL
    {
        MF_OPM_CGMSA_OFF	= 0,
        MF_OPM_CGMSA_COPY_FREELY	= 0x1,
        MF_OPM_CGMSA_COPY_NO_MORE	= 0x2,
        MF_OPM_CGMSA_COPY_ONE_GENERATION	= 0x3,
        MF_OPM_CGMSA_COPY_NEVER	= 0x4,
        MF_OPM_CGMSA_REDISTRIBUTION_CONTROL_REQUIRED	= 0x8
    } 	MF_OPM_CGMSA_PROTECTION_LEVEL;

typedef 
enum _MF_OPM_ACP_PROTECTION_LEVEL
    {
        MF_OPM_ACP_OFF	= 0,
        MF_OPM_ACP_LEVEL_ONE	= 1,
        MF_OPM_ACP_LEVEL_TWO	= 2,
        MF_OPM_ACP_LEVEL_THREE	= 3,
        MF_OPM_ACP_FORCE_ULONG	= 0x7fffffff
    } 	MF_OPM_ACP_PROTECTION_LEVEL;

EXTERN_GUID( MFPROTECTIONATTRIBUTE_BEST_EFFORT, 0xc8e06331, 0x75f0, 0x4ec1, 0x8e, 0x77, 0x17, 0x57, 0x8f, 0x77, 0x3b, 0x46);

EXTERN_GUID( MFPROTECTIONATTRIBUTE_FAIL_OVER, 0x8536abc5, 0x38f1, 0x4151, 0x9c, 0xce, 0xf5, 0x5d, 0x94, 0x12, 0x29, 0xac);

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID(MFPROTECTION_GRAPHICS_TRANSFER_AES_ENCRYPTION, 0xc873de64, 0xd8a5, 0x49e6, 0x88, 0xbb, 0xfb, 0x96, 0x3f, 0xd3, 0xd4, 0xce);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)

EXTERN_GUID( MFPROTECTIONATTRIBUTE_CONSTRICTVIDEO_IMAGESIZE, 0x8476fc, 0x4b58, 0x4d80, 0xa7, 0x90, 0xe7, 0x29, 0x76, 0x73, 0x16, 0x1d);
EXTERN_GUID( MFPROTECTIONATTRIBUTE_HDCP_SRM, 0x6f302107, 0x3477, 0x4468, 0x8a, 0x8, 0xee, 0xf9, 0xdb, 0x10, 0xe2, 0xf);


#define MAKE_MFPROTECTIONDATA_DISABLE(Disable)  \
    ((DWORD)(Disable ? 0x00000001 : 0))

#define EXTRACT_MFPROTECTIONDATA_DISABLE_ON(Data) \
    (0 != ((Data) & 0x00000001))

#define EXTRACT_MFPROTECTIONDATA_DISABLE_RESERVED(Data) \
    (((DWORD)((Data) & 0xFFFFFFFE)) >> 1)


#define MAKE_MFPROTECTIONDATA_CONSTRICTAUDIO(Level) \
    ((DWORD)(Level))

#define EXTRACT_MFPROTECTIONDATA_CONSTRICTAUDIO_LEVEL(Data) \
    ((DWORD)((Data) & 0x000000FF))

#define EXTRACT_MFPROTECTIONDATA_CONSTRICTAUDIO_RESERVED(Data) \
    (((DWORD)((Data) & 0xFFFFFF00)) >> 8)


typedef enum _MFAudioConstriction                                  
{                                                                  
    MFaudioConstrictionOff   = 0,                                  
    MFaudioConstriction48_16 = ( MFaudioConstrictionOff + 1 ) ,    
    MFaudioConstriction44_16 = ( MFaudioConstriction48_16 + 1 ) ,  
    MFaudioConstriction14_14 = ( MFaudioConstriction44_16 + 1 ) ,  
    MFaudioConstrictionMute  = ( MFaudioConstriction14_14 + 1 )    
} MFAudioConstriction;                                             


#define MAKE_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS(TestCertificateEnable, DigitalOutputDisable, DrmLevel) \
    (((DWORD)((TestCertificateEnable) ? 0x00020000 : 0)) | \
     ((DWORD)((DigitalOutputDisable) ? 0x00010000 : 0)) | \
     ((DWORD)(DrmLevel)))


#if (WINVER >= _WIN32_WINNT_WIN7) 
#define MAKE_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS2(TestCertificateEnable, DigitalOutputDisable, CopyOK, DrmLevel) \
    (((DWORD)((TestCertificateEnable) ? 0x00020000 : 0)) | \
     ((DWORD)((DigitalOutputDisable) ? 0x00010000 : 0)) | \
     ((DWORD)((CopyOK) ? 0x00040000 : 0)) | \
     ((DWORD)(DrmLevel)))

#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#define EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_DRMLEVEL(Data) \
    ((DWORD)((Data) & 0x0000FFFF))

#define EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_DIGITALOUTPUTDISABLE(Data) \
    (0 != ((Data) & 0x00010000))

#define EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_TESTCERTIFICATEENABLE(Data) \
    (0 != ((Data) & 0x00020000))

#if (WINVER >= _WIN32_WINNT_WIN7) 
#define EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_COPYOK(Data) \
    (0 != ((Data) & 0x00040000))

#define EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_RESERVED(Data) \
    (((DWORD)((Data) & 0xFFF80000)) >> 19)

#else 
#define EXTRACT_MFPROTECTIONDATA_TRUSTEDAUDIODRIVERS_RESERVED(Data) \
    (((DWORD)((Data) & 0xFFF80000)) >> 18)

#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0078_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0078_v0_0_s_ifspec;

#ifndef __IMFSecureChannel_INTERFACE_DEFINED__
#define __IMFSecureChannel_INTERFACE_DEFINED__

/* interface IMFSecureChannel */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSecureChannel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d0ae555d-3b12-4d97-b060-0990bc5aeb67")
    IMFSecureChannel : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCertificate( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcbCert)  BYTE **ppCert,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbCert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetupSession( 
            /* [annotation][in] */ 
            _In_reads_bytes_(cbSessionKey)  BYTE *pbEncryptedSessionKey,
            /* [in] */ DWORD cbSessionKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSecureChannelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSecureChannel * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSecureChannel * This);
        
        DECLSPEC_XFGVIRT(IMFSecureChannel, GetCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetCertificate )( 
            IMFSecureChannel * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcbCert)  BYTE **ppCert,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbCert);
        
        DECLSPEC_XFGVIRT(IMFSecureChannel, SetupSession)
        HRESULT ( STDMETHODCALLTYPE *SetupSession )( 
            IMFSecureChannel * This,
            /* [annotation][in] */ 
            _In_reads_bytes_(cbSessionKey)  BYTE *pbEncryptedSessionKey,
            /* [in] */ DWORD cbSessionKey);
        
        END_INTERFACE
    } IMFSecureChannelVtbl;

    interface IMFSecureChannel
    {
        CONST_VTBL struct IMFSecureChannelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSecureChannel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSecureChannel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSecureChannel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSecureChannel_GetCertificate(This,ppCert,pcbCert)	\
    ( (This)->lpVtbl -> GetCertificate(This,ppCert,pcbCert) ) 

#define IMFSecureChannel_SetupSession(This,pbEncryptedSessionKey,cbSessionKey)	\
    ( (This)->lpVtbl -> SetupSession(This,pbEncryptedSessionKey,cbSessionKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSecureChannel_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0079 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum SAMPLE_PROTECTION_VERSION
    {
        SAMPLE_PROTECTION_VERSION_NO	= 0,
        SAMPLE_PROTECTION_VERSION_BASIC_LOKI	= 1,
        SAMPLE_PROTECTION_VERSION_SCATTER	= 2,
        SAMPLE_PROTECTION_VERSION_RC4	= 3,
        SAMPLE_PROTECTION_VERSION_AES128CTR	= 4
    } 	SAMPLE_PROTECTION_VERSION;

EXTERN_GUID( MF_SampleProtectionSalt, 0x5403deee, 0xb9ee, 0x438f, 0xaa, 0x83, 0x38, 0x4, 0x99, 0x7e, 0x56, 0x9d);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0079_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0079_v0_0_s_ifspec;

#ifndef __IMFSampleProtection_INTERFACE_DEFINED__
#define __IMFSampleProtection_INTERFACE_DEFINED__

/* interface IMFSampleProtection */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSampleProtection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8e36395f-c7b9-43c4-a54d-512b4af63c95")
    IMFSampleProtection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInputProtectionVersion( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputProtectionVersion( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProtectionCertificate( 
            /* [in] */ DWORD dwVersion,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcbCert)  BYTE **ppCert,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbCert) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitOutputProtection( 
            /* [in] */ DWORD dwVersion,
            /* [in] */ DWORD dwOutputId,
            /* [in] */ BYTE *pbCert,
            /* [in] */ DWORD cbCert,
            /* [out] */ BYTE **ppbSeed,
            /* [out] */ DWORD *pcbSeed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitInputProtection( 
            /* [in] */ DWORD dwVersion,
            /* [in] */ DWORD dwInputId,
            /* [in] */ BYTE *pbSeed,
            /* [in] */ DWORD cbSeed) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSampleProtectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSampleProtection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSampleProtection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSampleProtection * This);
        
        DECLSPEC_XFGVIRT(IMFSampleProtection, GetInputProtectionVersion)
        HRESULT ( STDMETHODCALLTYPE *GetInputProtectionVersion )( 
            IMFSampleProtection * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IMFSampleProtection, GetOutputProtectionVersion)
        HRESULT ( STDMETHODCALLTYPE *GetOutputProtectionVersion )( 
            IMFSampleProtection * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVersion);
        
        DECLSPEC_XFGVIRT(IMFSampleProtection, GetProtectionCertificate)
        HRESULT ( STDMETHODCALLTYPE *GetProtectionCertificate )( 
            IMFSampleProtection * This,
            /* [in] */ DWORD dwVersion,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pcbCert)  BYTE **ppCert,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbCert);
        
        DECLSPEC_XFGVIRT(IMFSampleProtection, InitOutputProtection)
        HRESULT ( STDMETHODCALLTYPE *InitOutputProtection )( 
            IMFSampleProtection * This,
            /* [in] */ DWORD dwVersion,
            /* [in] */ DWORD dwOutputId,
            /* [in] */ BYTE *pbCert,
            /* [in] */ DWORD cbCert,
            /* [out] */ BYTE **ppbSeed,
            /* [out] */ DWORD *pcbSeed);
        
        DECLSPEC_XFGVIRT(IMFSampleProtection, InitInputProtection)
        HRESULT ( STDMETHODCALLTYPE *InitInputProtection )( 
            IMFSampleProtection * This,
            /* [in] */ DWORD dwVersion,
            /* [in] */ DWORD dwInputId,
            /* [in] */ BYTE *pbSeed,
            /* [in] */ DWORD cbSeed);
        
        END_INTERFACE
    } IMFSampleProtectionVtbl;

    interface IMFSampleProtection
    {
        CONST_VTBL struct IMFSampleProtectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSampleProtection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSampleProtection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSampleProtection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSampleProtection_GetInputProtectionVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetInputProtectionVersion(This,pdwVersion) ) 

#define IMFSampleProtection_GetOutputProtectionVersion(This,pdwVersion)	\
    ( (This)->lpVtbl -> GetOutputProtectionVersion(This,pdwVersion) ) 

#define IMFSampleProtection_GetProtectionCertificate(This,dwVersion,ppCert,pcbCert)	\
    ( (This)->lpVtbl -> GetProtectionCertificate(This,dwVersion,ppCert,pcbCert) ) 

#define IMFSampleProtection_InitOutputProtection(This,dwVersion,dwOutputId,pbCert,cbCert,ppbSeed,pcbSeed)	\
    ( (This)->lpVtbl -> InitOutputProtection(This,dwVersion,dwOutputId,pbCert,cbCert,ppbSeed,pcbSeed) ) 

#define IMFSampleProtection_InitInputProtection(This,dwVersion,dwInputId,pbSeed,cbSeed)	\
    ( (This)->lpVtbl -> InitInputProtection(This,dwVersion,dwInputId,pbSeed,cbSeed) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSampleProtection_INTERFACE_DEFINED__ */


#ifndef __IMFMediaSinkPreroll_INTERFACE_DEFINED__
#define __IMFMediaSinkPreroll_INTERFACE_DEFINED__

/* interface IMFMediaSinkPreroll */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSinkPreroll;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5dfd4b2a-7674-4110-a4e6-8a68fd5f3688")
    IMFMediaSinkPreroll : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyPreroll( 
            /* [in] */ MFTIME hnsUpcomingStartTime) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSinkPrerollVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaSinkPreroll * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaSinkPreroll * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaSinkPreroll * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSinkPreroll, NotifyPreroll)
        HRESULT ( STDMETHODCALLTYPE *NotifyPreroll )( 
            __RPC__in IMFMediaSinkPreroll * This,
            /* [in] */ MFTIME hnsUpcomingStartTime);
        
        END_INTERFACE
    } IMFMediaSinkPrerollVtbl;

    interface IMFMediaSinkPreroll
    {
        CONST_VTBL struct IMFMediaSinkPrerollVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSinkPreroll_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSinkPreroll_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSinkPreroll_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSinkPreroll_NotifyPreroll(This,hnsUpcomingStartTime)	\
    ( (This)->lpVtbl -> NotifyPreroll(This,hnsUpcomingStartTime) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSinkPreroll_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0081 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0081_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0081_v0_0_s_ifspec;

#ifndef __IMFFinalizableMediaSink_INTERFACE_DEFINED__
#define __IMFFinalizableMediaSink_INTERFACE_DEFINED__

/* interface IMFFinalizableMediaSink */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFFinalizableMediaSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EAECB74A-9A50-42ce-9541-6A7F57AA4AD7")
    IMFFinalizableMediaSink : public IMFMediaSink
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginFinalize( 
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndFinalize( 
            /* [in] */ IMFAsyncResult *pResult) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFFinalizableMediaSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFFinalizableMediaSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFFinalizableMediaSink * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetCharacteristics )( 
            IMFFinalizableMediaSink * This,
            /* [out] */ DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, AddStreamSink)
        HRESULT ( STDMETHODCALLTYPE *AddStreamSink )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ DWORD dwStreamSinkIdentifier,
            /* [in] */ IMFMediaType *pMediaType,
            /* [out] */ IMFStreamSink **ppStreamSink);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, RemoveStreamSink)
        HRESULT ( STDMETHODCALLTYPE *RemoveStreamSink )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ DWORD dwStreamSinkIdentifier);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetStreamSinkCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSinkCount )( 
            IMFFinalizableMediaSink * This,
            /* [out] */ DWORD *pcStreamSinkCount);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetStreamSinkByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSinkByIndex )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ DWORD dwIndex,
            /* [out] */ IMFStreamSink **ppStreamSink);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetStreamSinkById)
        HRESULT ( STDMETHODCALLTYPE *GetStreamSinkById )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ DWORD dwStreamSinkIdentifier,
            /* [out] */ IMFStreamSink **ppStreamSink);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, SetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *SetPresentationClock )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ IMFPresentationClock *pPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, GetPresentationClock)
        HRESULT ( STDMETHODCALLTYPE *GetPresentationClock )( 
            IMFFinalizableMediaSink * This,
            /* [out] */ IMFPresentationClock **ppPresentationClock);
        
        DECLSPEC_XFGVIRT(IMFMediaSink, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFFinalizableMediaSink * This);
        
        DECLSPEC_XFGVIRT(IMFFinalizableMediaSink, BeginFinalize)
        HRESULT ( STDMETHODCALLTYPE *BeginFinalize )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFFinalizableMediaSink, EndFinalize)
        HRESULT ( STDMETHODCALLTYPE *EndFinalize )( 
            IMFFinalizableMediaSink * This,
            /* [in] */ IMFAsyncResult *pResult);
        
        END_INTERFACE
    } IMFFinalizableMediaSinkVtbl;

    interface IMFFinalizableMediaSink
    {
        CONST_VTBL struct IMFFinalizableMediaSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFFinalizableMediaSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFFinalizableMediaSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFFinalizableMediaSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFFinalizableMediaSink_GetCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetCharacteristics(This,pdwCharacteristics) ) 

#define IMFFinalizableMediaSink_AddStreamSink(This,dwStreamSinkIdentifier,pMediaType,ppStreamSink)	\
    ( (This)->lpVtbl -> AddStreamSink(This,dwStreamSinkIdentifier,pMediaType,ppStreamSink) ) 

#define IMFFinalizableMediaSink_RemoveStreamSink(This,dwStreamSinkIdentifier)	\
    ( (This)->lpVtbl -> RemoveStreamSink(This,dwStreamSinkIdentifier) ) 

#define IMFFinalizableMediaSink_GetStreamSinkCount(This,pcStreamSinkCount)	\
    ( (This)->lpVtbl -> GetStreamSinkCount(This,pcStreamSinkCount) ) 

#define IMFFinalizableMediaSink_GetStreamSinkByIndex(This,dwIndex,ppStreamSink)	\
    ( (This)->lpVtbl -> GetStreamSinkByIndex(This,dwIndex,ppStreamSink) ) 

#define IMFFinalizableMediaSink_GetStreamSinkById(This,dwStreamSinkIdentifier,ppStreamSink)	\
    ( (This)->lpVtbl -> GetStreamSinkById(This,dwStreamSinkIdentifier,ppStreamSink) ) 

#define IMFFinalizableMediaSink_SetPresentationClock(This,pPresentationClock)	\
    ( (This)->lpVtbl -> SetPresentationClock(This,pPresentationClock) ) 

#define IMFFinalizableMediaSink_GetPresentationClock(This,ppPresentationClock)	\
    ( (This)->lpVtbl -> GetPresentationClock(This,ppPresentationClock) ) 

#define IMFFinalizableMediaSink_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 


#define IMFFinalizableMediaSink_BeginFinalize(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginFinalize(This,pCallback,punkState) ) 

#define IMFFinalizableMediaSink_EndFinalize(This,pResult)	\
    ( (This)->lpVtbl -> EndFinalize(This,pResult) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFFinalizableMediaSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0082 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0082_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0082_v0_0_s_ifspec;

#ifndef __IMFStreamingSinkConfig_INTERFACE_DEFINED__
#define __IMFStreamingSinkConfig_INTERFACE_DEFINED__

/* interface IMFStreamingSinkConfig */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFStreamingSinkConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9db7aa41-3cc5-40d4-8509-555804ad34cc")
    IMFStreamingSinkConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartStreaming( 
            /* [in] */ BOOL fSeekOffsetIsByteOffset,
            /* [in] */ QWORD qwSeekOffset) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFStreamingSinkConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFStreamingSinkConfig * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFStreamingSinkConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFStreamingSinkConfig * This);
        
        DECLSPEC_XFGVIRT(IMFStreamingSinkConfig, StartStreaming)
        HRESULT ( STDMETHODCALLTYPE *StartStreaming )( 
            __RPC__in IMFStreamingSinkConfig * This,
            /* [in] */ BOOL fSeekOffsetIsByteOffset,
            /* [in] */ QWORD qwSeekOffset);
        
        END_INTERFACE
    } IMFStreamingSinkConfigVtbl;

    interface IMFStreamingSinkConfig
    {
        CONST_VTBL struct IMFStreamingSinkConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFStreamingSinkConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFStreamingSinkConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFStreamingSinkConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFStreamingSinkConfig_StartStreaming(This,fSeekOffsetIsByteOffset,qwSeekOffset)	\
    ( (This)->lpVtbl -> StartStreaming(This,fSeekOffsetIsByteOffset,qwSeekOffset) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFStreamingSinkConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0083 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0083_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0083_v0_0_s_ifspec;

#ifndef __IMFRemoteProxy_INTERFACE_DEFINED__
#define __IMFRemoteProxy_INTERFACE_DEFINED__

/* interface IMFRemoteProxy */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFRemoteProxy;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("994e23ad-1cc2-493c-b9fa-46f1cb040fa4")
    IMFRemoteProxy : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRemoteObject( 
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRemoteHost( 
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRemoteProxyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFRemoteProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFRemoteProxy * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFRemoteProxy * This);
        
        DECLSPEC_XFGVIRT(IMFRemoteProxy, GetRemoteObject)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteObject )( 
            IMFRemoteProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        DECLSPEC_XFGVIRT(IMFRemoteProxy, GetRemoteHost)
        HRESULT ( STDMETHODCALLTYPE *GetRemoteHost )( 
            IMFRemoteProxy * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Outptr_  void **ppv);
        
        END_INTERFACE
    } IMFRemoteProxyVtbl;

    interface IMFRemoteProxy
    {
        CONST_VTBL struct IMFRemoteProxyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRemoteProxy_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRemoteProxy_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRemoteProxy_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRemoteProxy_GetRemoteObject(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetRemoteObject(This,riid,ppv) ) 

#define IMFRemoteProxy_GetRemoteHost(This,riid,ppv)	\
    ( (This)->lpVtbl -> GetRemoteHost(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRemoteProxy_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0084 */
/* [local] */ 

EXTERN_GUID( MF_REMOTE_PROXY, 0x2f00c90e, 0xd2cf, 0x4278, 0x8b, 0x6a, 0xd0, 0x77, 0xfa, 0xc3, 0xa2, 0x5f);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0084_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0084_v0_0_s_ifspec;

#ifndef __IMFObjectReferenceStream_INTERFACE_DEFINED__
#define __IMFObjectReferenceStream_INTERFACE_DEFINED__

/* interface IMFObjectReferenceStream */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFObjectReferenceStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("09EF5BE3-C8A7-469e-8B70-73BF25BB193F")
    IMFObjectReferenceStream : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SaveReference( 
            /* [in] */ REFIID riid,
            /* [in] */ IUnknown *pUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LoadReference( 
            /* [in] */ REFIID riid,
            /* [out] */ void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFObjectReferenceStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFObjectReferenceStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFObjectReferenceStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFObjectReferenceStream * This);
        
        DECLSPEC_XFGVIRT(IMFObjectReferenceStream, SaveReference)
        HRESULT ( STDMETHODCALLTYPE *SaveReference )( 
            IMFObjectReferenceStream * This,
            /* [in] */ REFIID riid,
            /* [in] */ IUnknown *pUnk);
        
        DECLSPEC_XFGVIRT(IMFObjectReferenceStream, LoadReference)
        HRESULT ( STDMETHODCALLTYPE *LoadReference )( 
            IMFObjectReferenceStream * This,
            /* [in] */ REFIID riid,
            /* [out] */ void **ppv);
        
        END_INTERFACE
    } IMFObjectReferenceStreamVtbl;

    interface IMFObjectReferenceStream
    {
        CONST_VTBL struct IMFObjectReferenceStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFObjectReferenceStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFObjectReferenceStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFObjectReferenceStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFObjectReferenceStream_SaveReference(This,riid,pUnk)	\
    ( (This)->lpVtbl -> SaveReference(This,riid,pUnk) ) 

#define IMFObjectReferenceStream_LoadReference(This,riid,ppv)	\
    ( (This)->lpVtbl -> LoadReference(This,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFObjectReferenceStream_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0085 */
/* [local] */ 

EXTERN_GUID( CLSID_CreateMediaExtensionObject, 0xef65a54d, 0x0788, 0x45b8, 0x8b, 0x14, 0xbc, 0x0f, 0x6a, 0x6b, 0x51, 0x37);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0085_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0085_v0_0_s_ifspec;

#ifndef __IMFPMPHost_INTERFACE_DEFINED__
#define __IMFPMPHost_INTERFACE_DEFINED__

/* interface IMFPMPHost */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFPMPHost;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F70CA1A9-FDC7-4782-B994-ADFFB1C98606")
    IMFPMPHost : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LockProcess( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockProcess( void) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE CreateObjectByCLSID( 
            /* [in] */ REFCLSID clsid,
            /* [unique][in] */ IStream *pStream,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPMPHostVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFPMPHost * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFPMPHost * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFPMPHost * This);
        
        DECLSPEC_XFGVIRT(IMFPMPHost, LockProcess)
        HRESULT ( STDMETHODCALLTYPE *LockProcess )( 
            __RPC__in IMFPMPHost * This);
        
        DECLSPEC_XFGVIRT(IMFPMPHost, UnlockProcess)
        HRESULT ( STDMETHODCALLTYPE *UnlockProcess )( 
            __RPC__in IMFPMPHost * This);
        
        DECLSPEC_XFGVIRT(IMFPMPHost, CreateObjectByCLSID)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreateObjectByCLSID )( 
            IMFPMPHost * This,
            /* [in] */ REFCLSID clsid,
            /* [unique][in] */ IStream *pStream,
            /* [in] */ REFIID riid,
            /* [iid_is][out] */ void **ppv);
        
        END_INTERFACE
    } IMFPMPHostVtbl;

    interface IMFPMPHost
    {
        CONST_VTBL struct IMFPMPHostVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPMPHost_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPMPHost_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPMPHost_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPMPHost_LockProcess(This)	\
    ( (This)->lpVtbl -> LockProcess(This) ) 

#define IMFPMPHost_UnlockProcess(This)	\
    ( (This)->lpVtbl -> UnlockProcess(This) ) 

#define IMFPMPHost_CreateObjectByCLSID(This,clsid,pStream,riid,ppv)	\
    ( (This)->lpVtbl -> CreateObjectByCLSID(This,clsid,pStream,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFPMPHost_RemoteCreateObjectByCLSID_Proxy( 
    __RPC__in IMFPMPHost * This,
    /* [in] */ __RPC__in REFCLSID clsid,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);


void __RPC_STUB IMFPMPHost_RemoteCreateObjectByCLSID_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IMFPMPHost_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0086 */
/* [local] */ 

#if (WINVER >= _WIN32_WINNT_WIN7) 
EXTERN_C const GUID MF_PMP_SERVICE;
#endif // (WINVER >= _WIN32_WINNT_WIN7) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0086_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0086_v0_0_s_ifspec;

#ifndef __IMFPMPClient_INTERFACE_DEFINED__
#define __IMFPMPClient_INTERFACE_DEFINED__

/* interface IMFPMPClient */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFPMPClient;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6C4E655D-EAD8-4421-B6B9-54DCDBBDF820")
    IMFPMPClient : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPMPHost( 
            /* [in] */ IMFPMPHost *pPMPHost) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPMPClientVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFPMPClient * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFPMPClient * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFPMPClient * This);
        
        DECLSPEC_XFGVIRT(IMFPMPClient, SetPMPHost)
        HRESULT ( STDMETHODCALLTYPE *SetPMPHost )( 
            IMFPMPClient * This,
            /* [in] */ IMFPMPHost *pPMPHost);
        
        END_INTERFACE
    } IMFPMPClientVtbl;

    interface IMFPMPClient
    {
        CONST_VTBL struct IMFPMPClientVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPMPClient_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPMPClient_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPMPClient_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPMPClient_SetPMPHost(This,pPMPHost)	\
    ( (This)->lpVtbl -> SetPMPHost(This,pPMPHost) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPMPClient_INTERFACE_DEFINED__ */


#ifndef __IMFPMPServer_INTERFACE_DEFINED__
#define __IMFPMPServer_INTERFACE_DEFINED__

/* interface IMFPMPServer */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFPMPServer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("994e23af-1cc2-493c-b9fa-46f1cb040fa4")
    IMFPMPServer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LockProcess( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockProcess( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateObjectByCLSID( 
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPMPServerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFPMPServer * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFPMPServer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFPMPServer * This);
        
        DECLSPEC_XFGVIRT(IMFPMPServer, LockProcess)
        HRESULT ( STDMETHODCALLTYPE *LockProcess )( 
            __RPC__in IMFPMPServer * This);
        
        DECLSPEC_XFGVIRT(IMFPMPServer, UnlockProcess)
        HRESULT ( STDMETHODCALLTYPE *UnlockProcess )( 
            __RPC__in IMFPMPServer * This);
        
        DECLSPEC_XFGVIRT(IMFPMPServer, CreateObjectByCLSID)
        HRESULT ( STDMETHODCALLTYPE *CreateObjectByCLSID )( 
            __RPC__in IMFPMPServer * This,
            /* [in] */ __RPC__in REFCLSID clsid,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppObject);
        
        END_INTERFACE
    } IMFPMPServerVtbl;

    interface IMFPMPServer
    {
        CONST_VTBL struct IMFPMPServerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPMPServer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPMPServer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPMPServer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPMPServer_LockProcess(This)	\
    ( (This)->lpVtbl -> LockProcess(This) ) 

#define IMFPMPServer_UnlockProcess(This)	\
    ( (This)->lpVtbl -> UnlockProcess(This) ) 

#define IMFPMPServer_CreateObjectByCLSID(This,clsid,riid,ppObject)	\
    ( (This)->lpVtbl -> CreateObjectByCLSID(This,clsid,riid,ppObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPMPServer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0088 */
/* [local] */ 

STDAPI MFCreatePMPServer(
    DWORD dwCreationFlags,
    _Outptr_ IMFPMPServer** ppPMPServer
    );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0088_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0088_v0_0_s_ifspec;

#ifndef __IMFRemoteDesktopPlugin_INTERFACE_DEFINED__
#define __IMFRemoteDesktopPlugin_INTERFACE_DEFINED__

/* interface IMFRemoteDesktopPlugin */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFRemoteDesktopPlugin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1cde6309-cae0-4940-907e-c1ec9c3d1d4a")
    IMFRemoteDesktopPlugin : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE UpdateTopology( 
            /* [out][in] */ IMFTopology *pTopology) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRemoteDesktopPluginVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFRemoteDesktopPlugin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFRemoteDesktopPlugin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFRemoteDesktopPlugin * This);
        
        DECLSPEC_XFGVIRT(IMFRemoteDesktopPlugin, UpdateTopology)
        HRESULT ( STDMETHODCALLTYPE *UpdateTopology )( 
            IMFRemoteDesktopPlugin * This,
            /* [out][in] */ IMFTopology *pTopology);
        
        END_INTERFACE
    } IMFRemoteDesktopPluginVtbl;

    interface IMFRemoteDesktopPlugin
    {
        CONST_VTBL struct IMFRemoteDesktopPluginVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRemoteDesktopPlugin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRemoteDesktopPlugin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRemoteDesktopPlugin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRemoteDesktopPlugin_UpdateTopology(This,pTopology)	\
    ( (This)->lpVtbl -> UpdateTopology(This,pTopology) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRemoteDesktopPlugin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0089 */
/* [local] */ 

STDAPI MFCreateRemoteDesktopPlugin(
    _Outptr_ IMFRemoteDesktopPlugin** ppPlugin );
EXTERN_C HRESULT STDAPICALLTYPE CreateNamedPropertyStore(
        _Outptr_ INamedPropertyStore **ppStore
        );



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0089_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0089_v0_0_s_ifspec;

#ifndef __IMFSAMIStyle_INTERFACE_DEFINED__
#define __IMFSAMIStyle_INTERFACE_DEFINED__

/* interface IMFSAMIStyle */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSAMIStyle;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A7E025DD-5303-4a62-89D6-E747E1EFAC73")
    IMFSAMIStyle : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStyleCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStyles( 
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pPropVarStyleArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSelectedStyle( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszStyle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSelectedStyle( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppwszStyle) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSAMIStyleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSAMIStyle * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSAMIStyle * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSAMIStyle * This);
        
        DECLSPEC_XFGVIRT(IMFSAMIStyle, GetStyleCount)
        HRESULT ( STDMETHODCALLTYPE *GetStyleCount )( 
            IMFSAMIStyle * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFSAMIStyle, GetStyles)
        HRESULT ( STDMETHODCALLTYPE *GetStyles )( 
            IMFSAMIStyle * This,
            /* [annotation][out] */ 
            _Out_  PROPVARIANT *pPropVarStyleArray);
        
        DECLSPEC_XFGVIRT(IMFSAMIStyle, SetSelectedStyle)
        HRESULT ( STDMETHODCALLTYPE *SetSelectedStyle )( 
            IMFSAMIStyle * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszStyle);
        
        DECLSPEC_XFGVIRT(IMFSAMIStyle, GetSelectedStyle)
        HRESULT ( STDMETHODCALLTYPE *GetSelectedStyle )( 
            IMFSAMIStyle * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppwszStyle);
        
        END_INTERFACE
    } IMFSAMIStyleVtbl;

    interface IMFSAMIStyle
    {
        CONST_VTBL struct IMFSAMIStyleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSAMIStyle_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSAMIStyle_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSAMIStyle_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSAMIStyle_GetStyleCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetStyleCount(This,pdwCount) ) 

#define IMFSAMIStyle_GetStyles(This,pPropVarStyleArray)	\
    ( (This)->lpVtbl -> GetStyles(This,pPropVarStyleArray) ) 

#define IMFSAMIStyle_SetSelectedStyle(This,pwszStyle)	\
    ( (This)->lpVtbl -> SetSelectedStyle(This,pwszStyle) ) 

#define IMFSAMIStyle_GetSelectedStyle(This,ppwszStyle)	\
    ( (This)->lpVtbl -> GetSelectedStyle(This,ppwszStyle) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSAMIStyle_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0090 */
/* [local] */ 

EXTERN_GUID( MF_SAMI_SERVICE, 0x49a89ae7, 0xb4d9, 0x4ef2, 0xaa, 0x5c, 0xf6, 0x5a, 0x3e, 0x5, 0xae, 0x4e );
EXTERN_GUID( MF_PD_SAMI_STYLELIST, 0xe0b73c7f, 0x486d, 0x484e, 0x98, 0x72, 0x4d, 0xe5, 0x19, 0x2a, 0x7b, 0xf8 );
EXTERN_GUID( MF_SD_SAMI_LANGUAGE, 0x36fcb98a, 0x6cd0, 0x44cb, 0xac, 0xb9, 0xa8, 0xf5, 0x60, 0xd, 0xd0, 0xbb );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI MFCreateSampleCopierMFT(_Outptr_ IMFTransform** ppCopierMFT);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0090_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0090_v0_0_s_ifspec;

#ifndef __IMFTranscodeProfile_INTERFACE_DEFINED__
#define __IMFTranscodeProfile_INTERFACE_DEFINED__

/* interface IMFTranscodeProfile */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFTranscodeProfile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4ADFDBA3-7AB0-4953-A62B-461E7FF3DA1E")
    IMFTranscodeProfile : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAudioAttributes( 
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioAttributes( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IMFAttributes **ppAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVideoAttributes( 
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoAttributes( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IMFAttributes **ppAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetContainerAttributes( 
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttrs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetContainerAttributes( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IMFAttributes **ppAttrs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTranscodeProfileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTranscodeProfile * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTranscodeProfile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTranscodeProfile * This);
        
        DECLSPEC_XFGVIRT(IMFTranscodeProfile, SetAudioAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetAudioAttributes )( 
            IMFTranscodeProfile * This,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttrs);
        
        DECLSPEC_XFGVIRT(IMFTranscodeProfile, GetAudioAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAudioAttributes )( 
            IMFTranscodeProfile * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IMFAttributes **ppAttrs);
        
        DECLSPEC_XFGVIRT(IMFTranscodeProfile, SetVideoAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetVideoAttributes )( 
            IMFTranscodeProfile * This,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttrs);
        
        DECLSPEC_XFGVIRT(IMFTranscodeProfile, GetVideoAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetVideoAttributes )( 
            IMFTranscodeProfile * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IMFAttributes **ppAttrs);
        
        DECLSPEC_XFGVIRT(IMFTranscodeProfile, SetContainerAttributes)
        HRESULT ( STDMETHODCALLTYPE *SetContainerAttributes )( 
            IMFTranscodeProfile * This,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttrs);
        
        DECLSPEC_XFGVIRT(IMFTranscodeProfile, GetContainerAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetContainerAttributes )( 
            IMFTranscodeProfile * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IMFAttributes **ppAttrs);
        
        END_INTERFACE
    } IMFTranscodeProfileVtbl;

    interface IMFTranscodeProfile
    {
        CONST_VTBL struct IMFTranscodeProfileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTranscodeProfile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTranscodeProfile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTranscodeProfile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTranscodeProfile_SetAudioAttributes(This,pAttrs)	\
    ( (This)->lpVtbl -> SetAudioAttributes(This,pAttrs) ) 

#define IMFTranscodeProfile_GetAudioAttributes(This,ppAttrs)	\
    ( (This)->lpVtbl -> GetAudioAttributes(This,ppAttrs) ) 

#define IMFTranscodeProfile_SetVideoAttributes(This,pAttrs)	\
    ( (This)->lpVtbl -> SetVideoAttributes(This,pAttrs) ) 

#define IMFTranscodeProfile_GetVideoAttributes(This,ppAttrs)	\
    ( (This)->lpVtbl -> GetVideoAttributes(This,ppAttrs) ) 

#define IMFTranscodeProfile_SetContainerAttributes(This,pAttrs)	\
    ( (This)->lpVtbl -> SetContainerAttributes(This,pAttrs) ) 

#define IMFTranscodeProfile_GetContainerAttributes(This,ppAttrs)	\
    ( (This)->lpVtbl -> GetContainerAttributes(This,ppAttrs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTranscodeProfile_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0091 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID( MF_TRANSCODE_CONTAINERTYPE, 0x150ff23f, 0x4abc, 0x478b, 0xac, 0x4f, 0xe1, 0x91, 0x6f, 0xba, 0x1c, 0xca );
EXTERN_GUID( MFTranscodeContainerType_ASF, 0x430f6f6e, 0xb6bf, 0x4fc1, 0xa0, 0xbd, 0x9e, 0xe4, 0x6e, 0xee, 0x2a, 0xfb );
EXTERN_GUID( MFTranscodeContainerType_MPEG4, 0xdc6cd05d, 0xb9d0, 0x40ef, 0xbd, 0x35, 0xfa, 0x62, 0x2c, 0x1a, 0xb2, 0x8a );
EXTERN_GUID( MFTranscodeContainerType_MP3, 0xe438b912, 0x83f1, 0x4de6, 0x9e, 0x3a, 0x9f, 0xfb, 0xc6, 0xdd, 0x24, 0xd1 );
EXTERN_GUID( MFTranscodeContainerType_FLAC, 0x31344aa3, 0x05a9, 0x42b5, 0x90, 0x1b, 0x8e, 0x9d, 0x42, 0x57, 0xf7, 0x5e );
EXTERN_GUID( MFTranscodeContainerType_3GP, 0x34c50167, 0x4472, 0x4f34, 0x9e, 0xa0, 0xc4, 0x9f, 0xba, 0xcf, 0x03, 0x7d );
EXTERN_GUID( MFTranscodeContainerType_AC3, 0x6d8d91c3, 0x8c91, 0x4ed1, 0x87, 0x42, 0x8c, 0x34, 0x7d, 0x5b, 0x44, 0xd0 );
EXTERN_GUID( MFTranscodeContainerType_ADTS, 0x132fd27d, 0x0f02, 0x43de, 0xa3, 0x01, 0x38, 0xfb, 0xbb, 0xb3, 0x83, 0x4e );
EXTERN_GUID( MFTranscodeContainerType_MPEG2, 0xbfc2dbf9, 0x7bb4, 0x4f8f, 0xaf, 0xde, 0xe1, 0x12, 0xc4, 0x4b, 0xa8, 0x82 );
EXTERN_GUID( MFTranscodeContainerType_WAVE, 0x64c3453c, 0x0f26, 0x4741, 0xbe, 0x63, 0x87, 0xbd, 0xf8, 0xbb, 0x93, 0x5b );
EXTERN_GUID( MFTranscodeContainerType_AVI, 0x7edfe8af, 0x402f, 0x4d76, 0xa3, 0x3c, 0x61, 0x9f, 0xd1, 0x57, 0xd0, 0xf1 );
#if (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MFTranscodeContainerType_FMPEG4, 0x9ba876f1, 0x419f, 0x4b77, 0xa1, 0xe0, 0x35, 0x95, 0x9d, 0x9d, 0x40, 0x4 );
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
EXTERN_GUID( MFTranscodeContainerType_AMR, 0x25d5ad3, 0x621a, 0x475b, 0x96, 0x4d, 0x66, 0xb1, 0xc8, 0x24, 0xf0, 0x79 );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MF_TRANSCODE_SKIP_METADATA_TRANSFER, 0x4e4469ef, 0xb571, 0x4959, 0x8f, 0x83, 0x3d, 0xcf, 0xba, 0x33, 0xa3, 0x93 );
EXTERN_GUID( MF_TRANSCODE_TOPOLOGYMODE, 0x3e3df610, 0x394a, 0x40b2, 0x9d, 0xea, 0x3b, 0xab, 0x65, 0xb, 0xeb, 0xf2 );
typedef 
enum _MF_TRANSCODE_TOPOLOGYMODE_FLAGS
    {
        MF_TRANSCODE_TOPOLOGYMODE_SOFTWARE_ONLY	= 0,
        MF_TRANSCODE_TOPOLOGYMODE_HARDWARE_ALLOWED	= 1
    } 	MF_TRANSCODE_TOPOLOGYMODE_FLAGS;

EXTERN_GUID( MF_TRANSCODE_ADJUST_PROFILE, 0x9c37c21b, 0x60f, 0x487c, 0xa6, 0x90, 0x80, 0xd7, 0xf5, 0xd, 0x1c, 0x72 );
typedef 
enum _MF_TRANSCODE_ADJUST_PROFILE_FLAGS
    {
        MF_TRANSCODE_ADJUST_PROFILE_DEFAULT	= 0,
        MF_TRANSCODE_ADJUST_PROFILE_USE_SOURCE_ATTRIBUTES	= 1
    } 	MF_TRANSCODE_ADJUST_PROFILE_FLAGS;

EXTERN_GUID( MF_TRANSCODE_ENCODINGPROFILE, 0x6947787c, 0xf508, 0x4ea9, 0xb1, 0xe9, 0xa1, 0xfe, 0x3a, 0x49, 0xfb, 0xc9 );
EXTERN_GUID( MF_TRANSCODE_QUALITYVSSPEED, 0x98332df8, 0x03cd, 0x476b, 0x89, 0xfa, 0x3f, 0x9e, 0x44, 0x2d, 0xec, 0x9f );
EXTERN_GUID( MF_TRANSCODE_DONOT_INSERT_ENCODER, 0xf45aa7ce, 0xab24, 0x4012, 0xa1, 0x1b, 0xdc, 0x82, 0x20, 0x20, 0x14, 0x10 );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum _MF_VIDEO_PROCESSOR_ALGORITHM_TYPE
    {
        MF_VIDEO_PROCESSOR_ALGORITHM_DEFAULT	= 0,
        MF_VIDEO_PROCESSOR_ALGORITHM_MRF_CRF_444	= 1
    } 	MF_VIDEO_PROCESSOR_ALGORITHM_TYPE;

EXTERN_GUID( MF_VIDEO_PROCESSOR_ALGORITHM, 0x4a0a1e1f, 0x272c, 0x4fb6, 0x9e, 0xb1, 0xdb, 0x33, 0xc, 0xbc, 0x97, 0xca);
EXTERN_GUID( MF_XVP_DISABLE_FRC, 0x2c0afa19, 0x7a97, 0x4d5a, 0x9e, 0xe8, 0x16, 0xd4, 0xfc, 0x51, 0x8d, 0x8c );
#if (WINVER >= _WIN32_WINNT_WINBLUE) 
EXTERN_GUID( MF_XVP_CALLER_ALLOCATES_OUTPUT, 0x4a2cabc, 0xcab, 0x40b1, 0xa1, 0xb9, 0x75, 0xbc, 0x36, 0x58, 0xf0, 0x0 );
#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if (WINVER >= _WIN32_WINNT_WINBLUE) 
#if (WINVER < _WIN32_WINNT_WINTHRESHOLD) 
EXTERN_GUID(CLSID_VideoProcessorMFT, 0x88753b26, 0x5b24, 0x49bd, 0xb2, 0xe7, 0xc, 0x44, 0x5c, 0x78, 0xc9, 0x82);
#endif // (WINVER < _WIN32_WINNT_WINTHRESHOLD) 
#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 
STDAPI MFCreateTranscodeProfile(
    _Outptr_ IMFTranscodeProfile** ppTranscodeProfile
    );
STDAPI MFCreateTranscodeTopology(
    _In_ IMFMediaSource* pSrc,
    _In_ LPCWSTR pwszOutputFilePath,
    _In_ IMFTranscodeProfile* pProfile,
    _Outptr_ IMFTopology** ppTranscodeTopo
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI MFCreateTranscodeTopologyFromByteStream(
    _In_ IMFMediaSource* pSrc,
    _In_ IMFByteStream* pOutputStream,
    _In_ IMFTranscodeProfile* pProfile,
    _Out_ IMFTopology** ppTranscodeTopo
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN8) 
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI MFTranscodeGetAudioOutputAvailableTypes(
 _In_ REFGUID guidSubType,
 _In_ DWORD dwMFTFlags,
 _In_opt_ IMFAttributes* pCodecConfig, 
 _Outptr_ IMFCollection** ppAvailableTypes );
typedef struct _MF_TRANSCODE_SINK_INFO
    {
    DWORD dwVideoStreamID;
    IMFMediaType *pVideoMediaType;
    DWORD dwAudioStreamID;
    IMFMediaType *pAudioMediaType;
    } 	MF_TRANSCODE_SINK_INFO;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0091_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0091_v0_0_s_ifspec;

#ifndef __IMFTranscodeSinkInfoProvider_INTERFACE_DEFINED__
#define __IMFTranscodeSinkInfoProvider_INTERFACE_DEFINED__

/* interface IMFTranscodeSinkInfoProvider */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFTranscodeSinkInfoProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8CFFCD2E-5A03-4a3a-AFF7-EDCD107C620E")
    IMFTranscodeSinkInfoProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetOutputFile( 
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszFileName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputByteStream( 
            /* [annotation][in] */ 
            _In_  IMFActivate *pByteStreamActivate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProfile( 
            /* [annotation][in] */ 
            _In_  IMFTranscodeProfile *pProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSinkInfo( 
            /* [annotation][out] */ 
            _Out_  MF_TRANSCODE_SINK_INFO *pSinkInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTranscodeSinkInfoProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTranscodeSinkInfoProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTranscodeSinkInfoProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTranscodeSinkInfoProvider * This);
        
        DECLSPEC_XFGVIRT(IMFTranscodeSinkInfoProvider, SetOutputFile)
        HRESULT ( STDMETHODCALLTYPE *SetOutputFile )( 
            IMFTranscodeSinkInfoProvider * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR pwszFileName);
        
        DECLSPEC_XFGVIRT(IMFTranscodeSinkInfoProvider, SetOutputByteStream)
        HRESULT ( STDMETHODCALLTYPE *SetOutputByteStream )( 
            IMFTranscodeSinkInfoProvider * This,
            /* [annotation][in] */ 
            _In_  IMFActivate *pByteStreamActivate);
        
        DECLSPEC_XFGVIRT(IMFTranscodeSinkInfoProvider, SetProfile)
        HRESULT ( STDMETHODCALLTYPE *SetProfile )( 
            IMFTranscodeSinkInfoProvider * This,
            /* [annotation][in] */ 
            _In_  IMFTranscodeProfile *pProfile);
        
        DECLSPEC_XFGVIRT(IMFTranscodeSinkInfoProvider, GetSinkInfo)
        HRESULT ( STDMETHODCALLTYPE *GetSinkInfo )( 
            IMFTranscodeSinkInfoProvider * This,
            /* [annotation][out] */ 
            _Out_  MF_TRANSCODE_SINK_INFO *pSinkInfo);
        
        END_INTERFACE
    } IMFTranscodeSinkInfoProviderVtbl;

    interface IMFTranscodeSinkInfoProvider
    {
        CONST_VTBL struct IMFTranscodeSinkInfoProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTranscodeSinkInfoProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTranscodeSinkInfoProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTranscodeSinkInfoProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTranscodeSinkInfoProvider_SetOutputFile(This,pwszFileName)	\
    ( (This)->lpVtbl -> SetOutputFile(This,pwszFileName) ) 

#define IMFTranscodeSinkInfoProvider_SetOutputByteStream(This,pByteStreamActivate)	\
    ( (This)->lpVtbl -> SetOutputByteStream(This,pByteStreamActivate) ) 

#define IMFTranscodeSinkInfoProvider_SetProfile(This,pProfile)	\
    ( (This)->lpVtbl -> SetProfile(This,pProfile) ) 

#define IMFTranscodeSinkInfoProvider_GetSinkInfo(This,pSinkInfo)	\
    ( (This)->lpVtbl -> GetSinkInfo(This,pSinkInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTranscodeSinkInfoProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0092 */
/* [local] */ 

 STDAPI MFCreateTranscodeSinkActivate( 
 _Outptr_ IMFActivate** ppActivate ); 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0092_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0092_v0_0_s_ifspec;

#ifndef __IMFFieldOfUseMFTUnlock_INTERFACE_DEFINED__
#define __IMFFieldOfUseMFTUnlock_INTERFACE_DEFINED__

/* interface IMFFieldOfUseMFTUnlock */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFFieldOfUseMFTUnlock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("508E71D3-EC66-4fc3-8775-B4B9ED6BA847")
    IMFFieldOfUseMFTUnlock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Unlock( 
            /* [in] */ __RPC__in_opt IUnknown *pUnkMFT) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFFieldOfUseMFTUnlockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFFieldOfUseMFTUnlock * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFFieldOfUseMFTUnlock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFFieldOfUseMFTUnlock * This);
        
        DECLSPEC_XFGVIRT(IMFFieldOfUseMFTUnlock, Unlock)
        HRESULT ( STDMETHODCALLTYPE *Unlock )( 
            __RPC__in IMFFieldOfUseMFTUnlock * This,
            /* [in] */ __RPC__in_opt IUnknown *pUnkMFT);
        
        END_INTERFACE
    } IMFFieldOfUseMFTUnlockVtbl;

    interface IMFFieldOfUseMFTUnlock
    {
        CONST_VTBL struct IMFFieldOfUseMFTUnlockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFFieldOfUseMFTUnlock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFFieldOfUseMFTUnlock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFFieldOfUseMFTUnlock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFFieldOfUseMFTUnlock_Unlock(This,pUnkMFT)	\
    ( (This)->lpVtbl -> Unlock(This,pUnkMFT) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFFieldOfUseMFTUnlock_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0093 */
/* [local] */ 

typedef struct _MFT_REGISTRATION_INFO
    {
    CLSID clsid;
    GUID guidCategory;
    UINT32 uiFlags;
    LPCWSTR pszName;
    DWORD cInTypes;
    /* [size_is] */ MFT_REGISTER_TYPE_INFO *pInTypes;
    DWORD cOutTypes;
    /* [size_is] */ MFT_REGISTER_TYPE_INFO *pOutTypes;
    } 	MFT_REGISTRATION_INFO;

EXTERN_GUID(MF_LOCAL_MFT_REGISTRATION_SERVICE, 0xddf5cf9c, 0x4506, 0x45aa, 0xab, 0xf0, 0x6d, 0x5d, 0x94, 0xdd, 0x1b, 0x4a);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0093_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0093_v0_0_s_ifspec;

#ifndef __IMFLocalMFTRegistration_INTERFACE_DEFINED__
#define __IMFLocalMFTRegistration_INTERFACE_DEFINED__

/* interface IMFLocalMFTRegistration */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFLocalMFTRegistration;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("149c4d73-b4be-4f8d-8b87-079e926b6add")
    IMFLocalMFTRegistration : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterMFTs( 
            /* [size_is][in] */ __RPC__in_ecount_full(cMFTs) MFT_REGISTRATION_INFO *pMFTs,
            DWORD cMFTs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFLocalMFTRegistrationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFLocalMFTRegistration * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFLocalMFTRegistration * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFLocalMFTRegistration * This);
        
        DECLSPEC_XFGVIRT(IMFLocalMFTRegistration, RegisterMFTs)
        HRESULT ( STDMETHODCALLTYPE *RegisterMFTs )( 
            __RPC__in IMFLocalMFTRegistration * This,
            /* [size_is][in] */ __RPC__in_ecount_full(cMFTs) MFT_REGISTRATION_INFO *pMFTs,
            DWORD cMFTs);
        
        END_INTERFACE
    } IMFLocalMFTRegistrationVtbl;

    interface IMFLocalMFTRegistration
    {
        CONST_VTBL struct IMFLocalMFTRegistrationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFLocalMFTRegistration_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFLocalMFTRegistration_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFLocalMFTRegistration_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFLocalMFTRegistration_RegisterMFTs(This,pMFTs,cMFTs)	\
    ( (This)->lpVtbl -> RegisterMFTs(This,pMFTs,cMFTs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFLocalMFTRegistration_INTERFACE_DEFINED__ */


#ifndef __IMFCapturePhotoConfirmation_INTERFACE_DEFINED__
#define __IMFCapturePhotoConfirmation_INTERFACE_DEFINED__

/* interface IMFCapturePhotoConfirmation */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFCapturePhotoConfirmation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("19f68549-ca8a-4706-a4ef-481dbc95e12c")
    IMFCapturePhotoConfirmation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPhotoConfirmationCallback( 
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pNotificationCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPixelFormat( 
            /* [annotation][in] */ 
            _In_  GUID subtype) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPixelFormat( 
            /* [annotation][out] */ 
            _Out_  GUID *subtype) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCapturePhotoConfirmationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCapturePhotoConfirmation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCapturePhotoConfirmation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCapturePhotoConfirmation * This);
        
        DECLSPEC_XFGVIRT(IMFCapturePhotoConfirmation, SetPhotoConfirmationCallback)
        HRESULT ( STDMETHODCALLTYPE *SetPhotoConfirmationCallback )( 
            IMFCapturePhotoConfirmation * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pNotificationCallback);
        
        DECLSPEC_XFGVIRT(IMFCapturePhotoConfirmation, SetPixelFormat)
        HRESULT ( STDMETHODCALLTYPE *SetPixelFormat )( 
            IMFCapturePhotoConfirmation * This,
            /* [annotation][in] */ 
            _In_  GUID subtype);
        
        DECLSPEC_XFGVIRT(IMFCapturePhotoConfirmation, GetPixelFormat)
        HRESULT ( STDMETHODCALLTYPE *GetPixelFormat )( 
            IMFCapturePhotoConfirmation * This,
            /* [annotation][out] */ 
            _Out_  GUID *subtype);
        
        END_INTERFACE
    } IMFCapturePhotoConfirmationVtbl;

    interface IMFCapturePhotoConfirmation
    {
        CONST_VTBL struct IMFCapturePhotoConfirmationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCapturePhotoConfirmation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCapturePhotoConfirmation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCapturePhotoConfirmation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCapturePhotoConfirmation_SetPhotoConfirmationCallback(This,pNotificationCallback)	\
    ( (This)->lpVtbl -> SetPhotoConfirmationCallback(This,pNotificationCallback) ) 

#define IMFCapturePhotoConfirmation_SetPixelFormat(This,subtype)	\
    ( (This)->lpVtbl -> SetPixelFormat(This,subtype) ) 

#define IMFCapturePhotoConfirmation_GetPixelFormat(This,subtype)	\
    ( (This)->lpVtbl -> GetPixelFormat(This,subtype) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCapturePhotoConfirmation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0095 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
#if (WINVER >= _WIN32_WINNT_WIN8) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0095_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0095_v0_0_s_ifspec;

#ifndef __IMFPMPHostApp_INTERFACE_DEFINED__
#define __IMFPMPHostApp_INTERFACE_DEFINED__

/* interface IMFPMPHostApp */
/* [uuid][object] */ 


EXTERN_C const IID IID_IMFPMPHostApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("84d2054a-3aa1-4728-a3b0-440a418cf49c")
    IMFPMPHostApp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LockProcess( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockProcess( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateClassById( 
            /* [in] */ __RPC__in LPCWSTR id,
            /* [unique][in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPMPHostAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFPMPHostApp * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFPMPHostApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFPMPHostApp * This);
        
        DECLSPEC_XFGVIRT(IMFPMPHostApp, LockProcess)
        HRESULT ( STDMETHODCALLTYPE *LockProcess )( 
            __RPC__in IMFPMPHostApp * This);
        
        DECLSPEC_XFGVIRT(IMFPMPHostApp, UnlockProcess)
        HRESULT ( STDMETHODCALLTYPE *UnlockProcess )( 
            __RPC__in IMFPMPHostApp * This);
        
        DECLSPEC_XFGVIRT(IMFPMPHostApp, ActivateClassById)
        HRESULT ( STDMETHODCALLTYPE *ActivateClassById )( 
            __RPC__in IMFPMPHostApp * This,
            /* [in] */ __RPC__in LPCWSTR id,
            /* [unique][in] */ __RPC__in_opt IStream *pStream,
            /* [in] */ __RPC__in REFIID riid,
            /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);
        
        END_INTERFACE
    } IMFPMPHostAppVtbl;

    interface IMFPMPHostApp
    {
        CONST_VTBL struct IMFPMPHostAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPMPHostApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPMPHostApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPMPHostApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPMPHostApp_LockProcess(This)	\
    ( (This)->lpVtbl -> LockProcess(This) ) 

#define IMFPMPHostApp_UnlockProcess(This)	\
    ( (This)->lpVtbl -> UnlockProcess(This) ) 

#define IMFPMPHostApp_ActivateClassById(This,id,pStream,riid,ppv)	\
    ( (This)->lpVtbl -> ActivateClassById(This,id,pStream,riid,ppv) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPMPHostApp_INTERFACE_DEFINED__ */


#ifndef __IMFPMPClientApp_INTERFACE_DEFINED__
#define __IMFPMPClientApp_INTERFACE_DEFINED__

/* interface IMFPMPClientApp */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFPMPClientApp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c004f646-be2c-48f3-93a2-a0983eba1108")
    IMFPMPClientApp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetPMPHost( 
            /* [in] */ IMFPMPHostApp *pPMPHost) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFPMPClientAppVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFPMPClientApp * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFPMPClientApp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFPMPClientApp * This);
        
        DECLSPEC_XFGVIRT(IMFPMPClientApp, SetPMPHost)
        HRESULT ( STDMETHODCALLTYPE *SetPMPHost )( 
            IMFPMPClientApp * This,
            /* [in] */ IMFPMPHostApp *pPMPHost);
        
        END_INTERFACE
    } IMFPMPClientAppVtbl;

    interface IMFPMPClientApp
    {
        CONST_VTBL struct IMFPMPClientAppVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFPMPClientApp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFPMPClientApp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFPMPClientApp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFPMPClientApp_SetPMPHost(This,pPMPHost)	\
    ( (This)->lpVtbl -> SetPMPHost(This,pPMPHost) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFPMPClientApp_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0097 */
/* [local] */ 

#endif 
#if (WINVER >= _WIN32_WINNT_WINBLUE) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0097_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0097_v0_0_s_ifspec;

#ifndef __IMFMediaStreamSourceSampleRequest_INTERFACE_DEFINED__
#define __IMFMediaStreamSourceSampleRequest_INTERFACE_DEFINED__

/* interface IMFMediaStreamSourceSampleRequest */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaStreamSourceSampleRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("380b9af9-a85b-4e78-a2af-ea5ce645c6b4")
    IMFMediaStreamSourceSampleRequest : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSample( 
            /* [in] */ __RPC__in_opt IMFSample *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaStreamSourceSampleRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMFMediaStreamSourceSampleRequest * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMFMediaStreamSourceSampleRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMFMediaStreamSourceSampleRequest * This);
        
        DECLSPEC_XFGVIRT(IMFMediaStreamSourceSampleRequest, SetSample)
        HRESULT ( STDMETHODCALLTYPE *SetSample )( 
            __RPC__in IMFMediaStreamSourceSampleRequest * This,
            /* [in] */ __RPC__in_opt IMFSample *value);
        
        END_INTERFACE
    } IMFMediaStreamSourceSampleRequestVtbl;

    interface IMFMediaStreamSourceSampleRequest
    {
        CONST_VTBL struct IMFMediaStreamSourceSampleRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaStreamSourceSampleRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaStreamSourceSampleRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaStreamSourceSampleRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaStreamSourceSampleRequest_SetSample(This,value)	\
    ( (This)->lpVtbl -> SetSample(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaStreamSourceSampleRequest_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0098 */
/* [local] */ 

#endif 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0098_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0098_v0_0_s_ifspec;

#ifndef __IMFTrackedSample_INTERFACE_DEFINED__
#define __IMFTrackedSample_INTERFACE_DEFINED__

/* interface IMFTrackedSample */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFTrackedSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("245BF8E9-0755-40f7-88A5-AE0F18D55E17")
    IMFTrackedSample : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAllocator( 
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pSampleAllocator,
            /* [unique][in] */ IUnknown *pUnkState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFTrackedSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFTrackedSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFTrackedSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFTrackedSample * This);
        
        DECLSPEC_XFGVIRT(IMFTrackedSample, SetAllocator)
        HRESULT ( STDMETHODCALLTYPE *SetAllocator )( 
            IMFTrackedSample * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pSampleAllocator,
            /* [unique][in] */ IUnknown *pUnkState);
        
        END_INTERFACE
    } IMFTrackedSampleVtbl;

    interface IMFTrackedSample
    {
        CONST_VTBL struct IMFTrackedSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFTrackedSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFTrackedSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFTrackedSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFTrackedSample_SetAllocator(This,pSampleAllocator,pUnkState)	\
    ( (This)->lpVtbl -> SetAllocator(This,pSampleAllocator,pUnkState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFTrackedSample_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0099 */
/* [local] */ 

STDAPI MFCreateTrackedSample(
    _Outptr_ IMFTrackedSample** ppMFSample);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Desktop or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES)
STDAPI MFCreateMFByteStreamOnStream(
    IStream*        pStream,
    _Outptr_ IMFByteStream** ppByteStream);
STDAPI MFCreateStreamOnMFByteStream(
    _In_ IMFByteStream* pByteStream,
    _Outptr_ IStream** ppStream);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_GAMES) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
STDAPI MFCreateMFByteStreamOnStreamEx(
    _In_ IUnknown* punkStream,
    _Outptr_ IMFByteStream** ppByteStream );
STDAPI MFCreateStreamOnMFByteStreamEx(
    _In_ IMFByteStream* pByteStream,
    _In_ REFIID riid,
    _Outptr_ void **ppv );
STDAPI MFCreateMediaTypeFromProperties(
    _In_ IUnknown* punkStream,
    _Outptr_ IMFMediaType** ppMediaType );
STDAPI MFCreatePropertiesFromMediaType(
    _In_ IMFMediaType* pMediaType,
    _In_ REFIID riid,
    _Outptr_ void **ppv );
#if (WINVER >= _WIN32_WINNT_WINBLUE) 
DEFINE_GUID(MF_WRAPPED_BUFFER_SERVICE, 0xab544072, 0xc269, 0x4ebc, 0xa5, 0x52, 0x1c, 0x3b, 0x32, 0xbe, 0xd5, 0xca);
EXTERN_GUID(MF_WRAPPED_SAMPLE_SERVICE, 0x31f52bf2, 0xd03e, 0x4048, 0x80, 0xd0, 0x9c, 0x10, 0x46, 0xd8, 0x7c, 0x61);
#endif // (WINVER >= _WIN32_WINNT_WINBLUE) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MF_WRAPPED_OBJECT, 0x2b182c4c, 0xd6ac, 0x49f4, 0x89, 0x15, 0xf7, 0x18, 0x87, 0xdb, 0x70, 0xcd);
EXTERN_GUID(CLSID_HttpSchemePlugin, 0x44cb442b, 0x9da9, 0x49df, 0xb3, 0xfd, 0x2, 0x37, 0x77, 0xb1, 0x6e, 0x50);
EXTERN_GUID(CLSID_UrlmonSchemePlugin, 0x9ec4b4f9, 0x3029, 0x45ad, 0x94, 0x7b, 0x34, 0x4d, 0xe2, 0xa2, 0x49, 0xe2);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID(CLSID_NetSchemePlugin, 0xe9f4ebab, 0xd97b, 0x463e, 0xa2, 0xb1, 0xc5, 0x4e, 0xe3, 0xf9, 0x41, 0x4d);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
STDAPI MFEnumDeviceSources(
    _In_                IMFAttributes*                      pAttributes,
    _Outptr_result_buffer_(*pcSourceActivate) IMFActivate***    pppSourceActivate,
    _Out_               UINT32*                             pcSourceActivate
);
STDAPI MFCreateDeviceSource(
    _In_  IMFAttributes*   pAttributes,
    _Outptr_ IMFMediaSource** ppSource
);
STDAPI MFCreateDeviceSourceActivate( 
    _In_  IMFAttributes*   pAttributes,
    _Outptr_ IMFActivate**   ppActivate
);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE, 0xc60ac5fe, 0x252a, 0x478f, 0xa0, 0xef, 0xbc, 0x8f, 0xa5, 0xf7, 0xca, 0xd3);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_HW_SOURCE, 0xde7046ba, 0x54d6, 0x4487, 0xa2, 0xa4, 0xec, 0x7c, 0xd, 0x1b, 0xd1, 0x63);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME, 0x60d0e559, 0x52f8, 0x4fa2, 0xbb, 0xce, 0xac, 0xdb, 0x34, 0xa8, 0xec, 0x1);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_MEDIA_TYPE, 0x56a819ca, 0xc78, 0x4de4, 0xa0, 0xa7, 0x3d, 0xda, 0xba, 0xf, 0x24, 0xd4);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_CATEGORY, 0x77f0ae69, 0xc3bd, 0x4509, 0x94, 0x1d, 0x46, 0x7e, 0x4d, 0x24, 0x89, 0x9e);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK, 0x58f0aad8, 0x22bf, 0x4f8a, 0xbb, 0x3d, 0xd2, 0xc4, 0x97, 0x8c, 0x6e, 0x2f);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_SYMBOLIC_LINK, 0x98d24b5e, 0x5930, 0x4614, 0xb5, 0xa1, 0xf6, 0x0, 0xf9, 0x35, 0x5a, 0x78);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_MAX_BUFFERS, 0x7dd9b730, 0x4f2d, 0x41d5, 0x8f, 0x95, 0xc, 0xc9, 0xa9, 0x12, 0xba, 0x26);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ENDPOINT_ID, 0x30da9258, 0xfeb9, 0x47a7, 0xa4, 0x53, 0x76, 0x3a, 0x7a, 0x8e, 0x1c, 0x5f);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ROLE, 0xbc9d118e, 0x8c67, 0x4a18, 0x85, 0xd4, 0x12, 0xd3, 0x0, 0x40, 0x5, 0x52);
EXTERN_GUID( MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_PROVIDER_DEVICE_ID, 0x36689d42, 0xa06c, 0x40ae, 0x84, 0xcf, 0xf5, 0xa0, 0x34, 0x6, 0x7c, 0xc4);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_SOURCE_XADDRESS, 0xbca0be52, 0xc327, 0x44c7, 0x9b, 0x7d, 0x7f, 0xa8, 0xd9, 0xb5, 0xbc, 0xda);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_SOURCE_STREAM_URL, 0x9d7b40d2, 0x3617, 0x4043, 0x93, 0xe3, 0x8d, 0x6d, 0xa9, 0xbb, 0x34, 0x92);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_SOURCE_USERNAME,0x5d01add, 0x949f, 0x46eb, 0xbc, 0x8e, 0x8b, 0xd, 0x2b, 0x32, 0xd7, 0x9d);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_SOURCE_PASSWORD, 0xa0fd7e16, 0x42d9, 0x49df, 0x84, 0xc0, 0xe8, 0x2c, 0x5e, 0xab, 0x88, 0x74);
EXTERN_GUID(CLSID_FrameServerNetworkCameraSource, 0x7a213aa7, 0x866f, 0x414a, 0x8c, 0x1a, 0x27, 0x5c, 0x72, 0x83, 0xa3, 0x95);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_GUID, 0x14dd9a1c, 0x7cff, 0x41be, 0xb1, 0xb9, 0xba, 0x1a, 0xc6, 0xec, 0xb5, 0x71);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID, 0x8ac3587a, 0x4ae7, 0x42d8, 0x99, 0xe0, 0x0a, 0x60, 0x13, 0xee, 0xf9, 0x0f);
EXTERN_GUID( MF_DEVICESTREAM_IMAGE_STREAM, 0xa7ffb865, 0xe7b2, 0x43b0, 0x9f, 0x6f, 0x9a, 0xf2, 0xa0, 0xe5, 0xf, 0xc0);
EXTERN_GUID( MF_DEVICESTREAM_INDEPENDENT_IMAGE_STREAM, 0x3eeec7e, 0xd605, 0x4576, 0x8b, 0x29, 0x65, 0x80, 0xb4, 0x90, 0xd7, 0xd3);
EXTERN_GUID( MF_DEVICESTREAM_STREAM_ID, 0x11bd5120, 0xd124, 0x446b, 0x88, 0xe6, 0x17, 0x6, 0x2, 0x57, 0xff, 0xf9);
EXTERN_GUID( MF_DEVICESTREAM_STREAM_CATEGORY, 0x2939e7b8, 0xa62e, 0x4579, 0xb6, 0x74, 0xd4, 0x7, 0x3d, 0xfa, 0xbb, 0xba);
EXTERN_GUID( MF_DEVICESTREAM_FRAMESERVER_SHARED, 0x1CB378E9, 0xB279, 0x41D4, 0xAF, 0x97, 0x34, 0xA2, 0x43, 0xE6, 0x83, 0x20);
EXTERN_GUID( MF_DEVICESTREAM_TRANSFORM_STREAM_ID,  0xe63937b7, 0xdaaf, 0x4d49, 0x81, 0x5f, 0xd8, 0x26, 0xf8, 0xad, 0x31, 0xe7);
EXTERN_GUID( MF_DEVICESTREAM_EXTENSION_PLUGIN_CLSID, 0x048e6558, 0x60c4, 0x4173, 0xbd, 0x5b, 0x6a, 0x3c, 0xa2, 0x89, 0x6a, 0xee);
EXTERN_GUID( MF_DEVICEMFT_EXTENSION_PLUGIN_CLSID, 0x844dbae, 0x34fa, 0x48a0, 0xa7, 0x83, 0x8e, 0x69, 0x6f, 0xb1, 0xc9, 0xa8);
EXTERN_GUID( MF_DEVICESTREAM_EXTENSION_PLUGIN_CONNECTION_POINT,  0x37f9375c, 0xe664, 0x4ea4, 0xaa, 0xe4, 0xcb, 0x6d, 0x1d, 0xac, 0xa1, 0xf4);
EXTERN_GUID( MF_DEVICESTREAM_TAKEPHOTO_TRIGGER, 0x1d180e34, 0x538c, 0x4fbb, 0xa7, 0x5a, 0x85, 0x9a, 0xf7, 0xd2, 0x61, 0xa6 );
EXTERN_GUID( MF_DEVICESTREAM_MAX_FRAME_BUFFERS, 0x1684cebe, 0x3175, 0x4985, 0x88, 0x2c, 0x0e, 0xfd, 0x3e, 0x8a, 0xc1, 0x1e );
EXTERN_GUID( MF_DEVICEMFT_CONNECTED_FILTER_KSCONTROL, 0x6a2c4fa6, 0xd179, 0x41cd, 0x95, 0x23, 0x82, 0x23, 0x71, 0xea, 0x40, 0xe5);
EXTERN_GUID( MF_DEVICEMFT_CONNECTED_PIN_KSCONTROL, 0xe63310f7, 0xb244, 0x4ef8, 0x9a, 0x7d, 0x24, 0xc7, 0x4e, 0x32, 0xeb, 0xd0);
EXTERN_GUID( MF_DEVICE_THERMAL_STATE_CHANGED, 0x70ccd0af, 0xfc9f, 0x4deb, 0xa8, 0x75, 0x9f, 0xec, 0xd1, 0x6c, 0x5b, 0xd4 );
EXTERN_GUID(MFSampleExtension_DeviceTimestamp, 0x8f3e35e7, 0x2dcd, 0x4887, 0x86, 0x22, 0x2a, 0x58, 0xba, 0xa6, 0x52, 0xb0);
EXTERN_GUID(MFSampleExtension_Spatial_CameraViewTransform , 0x4e251fa4, 0x830f, 0x4770, 0x85, 0x9a, 0x4b, 0x8d, 0x99, 0xaa, 0x80, 0x9b);
EXTERN_GUID(MFSampleExtension_Spatial_CameraCoordinateSystem , 0x9d13c82f, 0x2199, 0x4e67, 0x91, 0xcd, 0xd1, 0xa4, 0x18, 0x1f, 0x25, 0x34);
EXTERN_GUID(MFSampleExtension_Spatial_CameraProjectionTransform , 0x47f9fcb5, 0x2a02, 0x4f26, 0xa4, 0x77, 0x79, 0x2f, 0xdf, 0x95, 0x88, 0x6a);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif // (WINVER >= _WIN32_WINNT_WIN7) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0099_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0099_v0_0_s_ifspec;

#ifndef __IMFProtectedEnvironmentAccess_INTERFACE_DEFINED__
#define __IMFProtectedEnvironmentAccess_INTERFACE_DEFINED__

/* interface IMFProtectedEnvironmentAccess */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFProtectedEnvironmentAccess;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ef5dc845-f0d9-4ec9-b00c-cb5183d38434")
    IMFProtectedEnvironmentAccess : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Call( 
            /* [annotation] */ 
            _In_  UINT32 inputLength,
            /* [annotation] */ 
            _In_reads_bytes_(inputLength)  const BYTE *input,
            /* [annotation] */ 
            _In_  UINT32 outputLength,
            /* [annotation] */ 
            _Out_writes_bytes_(outputLength)  BYTE *output) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReadGRL( 
            /* [annotation] */ 
            _Out_  UINT32 *outputLength,
            /* [annotation] */ 
            _Outptr_result_bytebuffer_(*outputLength)  BYTE **output) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFProtectedEnvironmentAccessVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFProtectedEnvironmentAccess * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFProtectedEnvironmentAccess * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFProtectedEnvironmentAccess * This);
        
        DECLSPEC_XFGVIRT(IMFProtectedEnvironmentAccess, Call)
        HRESULT ( STDMETHODCALLTYPE *Call )( 
            IMFProtectedEnvironmentAccess * This,
            /* [annotation] */ 
            _In_  UINT32 inputLength,
            /* [annotation] */ 
            _In_reads_bytes_(inputLength)  const BYTE *input,
            /* [annotation] */ 
            _In_  UINT32 outputLength,
            /* [annotation] */ 
            _Out_writes_bytes_(outputLength)  BYTE *output);
        
        DECLSPEC_XFGVIRT(IMFProtectedEnvironmentAccess, ReadGRL)
        HRESULT ( STDMETHODCALLTYPE *ReadGRL )( 
            IMFProtectedEnvironmentAccess * This,
            /* [annotation] */ 
            _Out_  UINT32 *outputLength,
            /* [annotation] */ 
            _Outptr_result_bytebuffer_(*outputLength)  BYTE **output);
        
        END_INTERFACE
    } IMFProtectedEnvironmentAccessVtbl;

    interface IMFProtectedEnvironmentAccess
    {
        CONST_VTBL struct IMFProtectedEnvironmentAccessVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFProtectedEnvironmentAccess_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFProtectedEnvironmentAccess_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFProtectedEnvironmentAccess_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFProtectedEnvironmentAccess_Call(This,inputLength,input,outputLength,output)	\
    ( (This)->lpVtbl -> Call(This,inputLength,input,outputLength,output) ) 

#define IMFProtectedEnvironmentAccess_ReadGRL(This,outputLength,output)	\
    ( (This)->lpVtbl -> ReadGRL(This,outputLength,output) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFProtectedEnvironmentAccess_INTERFACE_DEFINED__ */


#ifndef __IMFSignedLibrary_INTERFACE_DEFINED__
#define __IMFSignedLibrary_INTERFACE_DEFINED__

/* interface IMFSignedLibrary */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSignedLibrary;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4a724bca-ff6a-4c07-8e0d-7a358421cf06")
    IMFSignedLibrary : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProcedureAddress( 
            /* [annotation] */ 
            _In_  LPCSTR name,
            /* [annotation] */ 
            _Outptr_  PVOID *address) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSignedLibraryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSignedLibrary * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSignedLibrary * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSignedLibrary * This);
        
        DECLSPEC_XFGVIRT(IMFSignedLibrary, GetProcedureAddress)
        HRESULT ( STDMETHODCALLTYPE *GetProcedureAddress )( 
            IMFSignedLibrary * This,
            /* [annotation] */ 
            _In_  LPCSTR name,
            /* [annotation] */ 
            _Outptr_  PVOID *address);
        
        END_INTERFACE
    } IMFSignedLibraryVtbl;

    interface IMFSignedLibrary
    {
        CONST_VTBL struct IMFSignedLibraryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSignedLibrary_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSignedLibrary_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSignedLibrary_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSignedLibrary_GetProcedureAddress(This,name,address)	\
    ( (This)->lpVtbl -> GetProcedureAddress(This,name,address) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSignedLibrary_INTERFACE_DEFINED__ */


#ifndef __IMFSystemId_INTERFACE_DEFINED__
#define __IMFSystemId_INTERFACE_DEFINED__

/* interface IMFSystemId */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFSystemId;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("fff4af3a-1fc1-4ef9-a29b-d26c49e2f31a")
    IMFSystemId : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetData( 
            /* [annotation] */ 
            _Out_  UINT32 *size,
            /* [annotation] */ 
            _Outptr_result_bytebuffer_(*size)  BYTE **data) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Setup( 
            UINT32 stage,
            UINT32 cbIn,
            /* [annotation] */ 
            _In_reads_bytes_(cbIn)  const BYTE *pbIn,
            /* [annotation] */ 
            _Out_  UINT32 *pcbOut,
            /* [annotation] */ 
            _Outptr_result_bytebuffer_(*pcbOut)  BYTE **ppbOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSystemIdVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSystemId * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSystemId * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSystemId * This);
        
        DECLSPEC_XFGVIRT(IMFSystemId, GetData)
        HRESULT ( STDMETHODCALLTYPE *GetData )( 
            IMFSystemId * This,
            /* [annotation] */ 
            _Out_  UINT32 *size,
            /* [annotation] */ 
            _Outptr_result_bytebuffer_(*size)  BYTE **data);
        
        DECLSPEC_XFGVIRT(IMFSystemId, Setup)
        HRESULT ( STDMETHODCALLTYPE *Setup )( 
            IMFSystemId * This,
            UINT32 stage,
            UINT32 cbIn,
            /* [annotation] */ 
            _In_reads_bytes_(cbIn)  const BYTE *pbIn,
            /* [annotation] */ 
            _Out_  UINT32 *pcbOut,
            /* [annotation] */ 
            _Outptr_result_bytebuffer_(*pcbOut)  BYTE **ppbOut);
        
        END_INTERFACE
    } IMFSystemIdVtbl;

    interface IMFSystemId
    {
        CONST_VTBL struct IMFSystemIdVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSystemId_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSystemId_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSystemId_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSystemId_GetData(This,size,data)	\
    ( (This)->lpVtbl -> GetData(This,size,data) ) 

#define IMFSystemId_Setup(This,stage,cbIn,pbIn,pcbOut,ppbOut)	\
    ( (This)->lpVtbl -> Setup(This,stage,cbIn,pbIn,pcbOut,ppbOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSystemId_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0102 */
/* [local] */ 

STDAPI MFCreateProtectedEnvironmentAccess( 
    _Outptr_ IMFProtectedEnvironmentAccess**   ppAccess
);
STDAPI MFLoadSignedLibrary( 
    _In_ LPCWSTR pszName,
    _Outptr_ IMFSignedLibrary**   ppLib
);
STDAPI MFGetSystemId( 
    _Outptr_ IMFSystemId** ppId
);
STDAPI MFGetLocalId( 
    _In_reads_bytes_(size) const BYTE *verifier,
    _In_ UINT32 size,
    _Outptr_ LPWSTR *id
);
// {40871C59-AB40-471F-8DC3-1F259D862479}
DEFINE_GUID(CLSID_MPEG2ByteStreamPlugin, 
0x40871c59, 0xab40, 0x471f, 0x8d, 0xc3, 0x1f, 0x25, 0x9d, 0x86, 0x24, 0x79);
// {f09992f7-9fba-4c4a-a37f-8c47b4e1dfe7}
EXTERN_GUID( MF_MEDIASOURCE_SERVICE, 0xf09992f7, 0x9fba, 0x4c4a, 0xa3, 0x7f, 0x8c, 0x47, 0xb4, 0xe1, 0xdf, 0xe7 );
// {014A5031-2F05-4C6A-9F9C-7D0DC4EDA5F4}
EXTERN_GUID( MF_ACCESS_CONTROLLED_MEDIASOURCE_SERVICE, 0x14a5031, 0x2f05, 0x4c6a, 0x9f, 0x9c, 0x7d, 0xd, 0xc4, 0xed, 0xa5, 0xf4 );
typedef struct _MFCONTENTPROTECTIONDEVICE_INPUT_DATA
    {
    DWORD HWProtectionFunctionID;
    DWORD PrivateDataByteCount;
    DWORD HWProtectionDataByteCount;
    DWORD Reserved;
    BYTE InputData[ 4 ];
    } 	MFCONTENTPROTECTIONDEVICE_INPUT_DATA;

typedef struct _MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA
    {
    DWORD PrivateDataByteCount;
    DWORD MaxHWProtectionDataByteCount;
    DWORD HWProtectionDataByteCount;
    HRESULT Status;
    LONGLONG TransportTimeInHundredsOfNanoseconds;
    LONGLONG ExecutionTimeInHundredsOfNanoseconds;
    BYTE OutputData[ 4 ];
    } 	MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA;

#define MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START 0x04000000
#define MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA_FUNCTIONID MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START
typedef struct _MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    DWORD TaskIndex;
    WCHAR ClassName[MAX_PATH];
    LONG BasePriority;
} MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA;


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0102_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0102_v0_0_s_ifspec;

#ifndef __IMFContentProtectionDevice_INTERFACE_DEFINED__
#define __IMFContentProtectionDevice_INTERFACE_DEFINED__

/* interface IMFContentProtectionDevice */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFContentProtectionDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E6257174-A060-4C9A-A088-3B1B471CAD28")
    IMFContentProtectionDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InvokeFunction( 
            /* [annotation][in] */ 
            _In_  DWORD FunctionId,
            /* [annotation][in] */ 
            _In_  DWORD InputBufferByteCount,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(InputBufferByteCount)  const BYTE *InputBuffer,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *OutputBufferByteCount,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(*OutputBufferByteCount)  BYTE *OutputBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateDataByteCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *PrivateInputByteCount,
            /* [annotation][out] */ 
            _Out_  DWORD *PrivateOutputByteCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentProtectionDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFContentProtectionDevice * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFContentProtectionDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFContentProtectionDevice * This);
        
        DECLSPEC_XFGVIRT(IMFContentProtectionDevice, InvokeFunction)
        HRESULT ( STDMETHODCALLTYPE *InvokeFunction )( 
            IMFContentProtectionDevice * This,
            /* [annotation][in] */ 
            _In_  DWORD FunctionId,
            /* [annotation][in] */ 
            _In_  DWORD InputBufferByteCount,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(InputBufferByteCount)  const BYTE *InputBuffer,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *OutputBufferByteCount,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(*OutputBufferByteCount)  BYTE *OutputBuffer);
        
        DECLSPEC_XFGVIRT(IMFContentProtectionDevice, GetPrivateDataByteCount)
        HRESULT ( STDMETHODCALLTYPE *GetPrivateDataByteCount )( 
            IMFContentProtectionDevice * This,
            /* [annotation][out] */ 
            _Out_  DWORD *PrivateInputByteCount,
            /* [annotation][out] */ 
            _Out_  DWORD *PrivateOutputByteCount);
        
        END_INTERFACE
    } IMFContentProtectionDeviceVtbl;

    interface IMFContentProtectionDevice
    {
        CONST_VTBL struct IMFContentProtectionDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentProtectionDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentProtectionDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentProtectionDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentProtectionDevice_InvokeFunction(This,FunctionId,InputBufferByteCount,InputBuffer,OutputBufferByteCount,OutputBuffer)	\
    ( (This)->lpVtbl -> InvokeFunction(This,FunctionId,InputBufferByteCount,InputBuffer,OutputBufferByteCount,OutputBuffer) ) 

#define IMFContentProtectionDevice_GetPrivateDataByteCount(This,PrivateInputByteCount,PrivateOutputByteCount)	\
    ( (This)->lpVtbl -> GetPrivateDataByteCount(This,PrivateInputByteCount,PrivateOutputByteCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentProtectionDevice_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0103 */
/* [local] */ 

STDAPI MFCreateContentProtectionDevice( 
   _In_ REFGUID ProtectionSystemId, 
   _Outptr_ IMFContentProtectionDevice **ContentProtectionDevice); 
STDAPI MFIsContentProtectionDeviceSupported( 
   _In_ REFGUID ProtectionSystemId, 
   _Out_ BOOL *isSupported); 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0103_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0103_v0_0_s_ifspec;

#ifndef __IMFContentDecryptorContext_INTERFACE_DEFINED__
#define __IMFContentDecryptorContext_INTERFACE_DEFINED__

/* interface IMFContentDecryptorContext */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFContentDecryptorContext;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7EC4B1BD-43FB-4763-85D2-64FCB5C5F4CB")
    IMFContentDecryptorContext : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeHardwareKey( 
            /* [annotation][in] */ 
            _In_  UINT InputPrivateDataByteCount,
            /* [annotation][in] */ 
            _In_reads_opt_(InputPrivateDataByteCount)  const void *InputPrivateData,
            /* [annotation][out] */ 
            _Out_  UINT64 *OutputPrivateData) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFContentDecryptorContextVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFContentDecryptorContext * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFContentDecryptorContext * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFContentDecryptorContext * This);
        
        DECLSPEC_XFGVIRT(IMFContentDecryptorContext, InitializeHardwareKey)
        HRESULT ( STDMETHODCALLTYPE *InitializeHardwareKey )( 
            IMFContentDecryptorContext * This,
            /* [annotation][in] */ 
            _In_  UINT InputPrivateDataByteCount,
            /* [annotation][in] */ 
            _In_reads_opt_(InputPrivateDataByteCount)  const void *InputPrivateData,
            /* [annotation][out] */ 
            _Out_  UINT64 *OutputPrivateData);
        
        END_INTERFACE
    } IMFContentDecryptorContextVtbl;

    interface IMFContentDecryptorContext
    {
        CONST_VTBL struct IMFContentDecryptorContextVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFContentDecryptorContext_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFContentDecryptorContext_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFContentDecryptorContext_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFContentDecryptorContext_InitializeHardwareKey(This,InputPrivateDataByteCount,InputPrivateData,OutputPrivateData)	\
    ( (This)->lpVtbl -> InitializeHardwareKey(This,InputPrivateDataByteCount,InputPrivateData,OutputPrivateData) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFContentDecryptorContext_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0104 */
/* [local] */ 

EXTERN_GUID( MF_CONTENT_DECRYPTOR_SERVICE, 0x68a72927, 0xfc7b, 0x44ee, 0x85, 0xf4, 0x7c, 0x51, 0xbd, 0x55, 0xa6, 0x59);
EXTERN_GUID( MF_CONTENT_PROTECTION_DEVICE_SERVICE, 0xff58436f, 0x76a0, 0x41fe, 0xb5, 0x66, 0x10, 0xcc, 0x53, 0x96, 0x2e, 0xdd);
STDAPI MFCreateContentDecryptorContext( 
   _In_ REFGUID guidMediaProtectionSystemId, 
   _In_opt_ IMFDXGIDeviceManager *pD3DManager, 
   _In_ IMFContentProtectionDevice *pContentProtectionDevice, 
   _Outptr_ IMFContentDecryptorContext **ppContentDecryptorContext); 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Application Family
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
EXTERN_GUID( MF_SD_AUDIO_ENCODER_DELAY,   0x8e85422c, 0x73de, 0x403f, 0x9a, 0x35, 0x55, 0x0a, 0xd6, 0xe8, 0xb9, 0x51 );
EXTERN_GUID( MF_SD_AUDIO_ENCODER_PADDING, 0x529c7f2c, 0xac4b, 0x4e3f, 0xbf, 0xc3, 0x09, 0x02, 0x19, 0x49, 0x82, 0xcb );
EXTERN_GUID(CLSID_MSH264DecoderMFT, 0x62CE7E72, 0x4C71, 0x4d20, 0xB1, 0x5D, 0x45, 0x28, 0x31, 0xA8, 0x7D, 0x9D);
EXTERN_GUID(CLSID_MSH264EncoderMFT, 0x6ca50344, 0x051a, 0x4ded, 0x97, 0x79, 0xa4, 0x33, 0x05, 0x16, 0x5e, 0x35);
EXTERN_GUID(CLSID_MSDDPlusDecMFT, 0x177C0AFE, 0x900B, 0x48d4, 0x9E, 0x4C, 0x57, 0xAD, 0xD2, 0x50, 0xB3, 0xD4);
EXTERN_GUID(CLSID_MP3DecMediaObject, 0xbbeea841, 0x0a63, 0x4f52, 0xa7, 0xab, 0xa9, 0xb3, 0xa8, 0x4e, 0xd3, 0x8a);
EXTERN_GUID(CLSID_MSAACDecMFT, 0x32d186a7, 0x218f, 0x4c75, 0x88, 0x76, 0xdd, 0x77, 0x27, 0x3a, 0x89, 0x99);
EXTERN_GUID(CLSID_MSH265DecoderMFT, 0x420A51A3, 0xD605, 0x430C, 0xB4, 0xFC, 0x45, 0x27, 0x4F, 0xA6, 0xC5, 0x62);
EXTERN_GUID(CLSID_WMVDecoderMFT, 0x82d353df, 0x90bd, 0x4382, 0x8b, 0xc2, 0x3f, 0x61, 0x92, 0xb7, 0x6e, 0x34);
EXTERN_GUID(CLSID_WMADecMediaObject, 0x2eeb4adf, 0x4578, 0x4d10, 0xbc, 0xa7, 0xbb, 0x95, 0x5f, 0x56, 0x32, 0x0a);
EXTERN_GUID(CLSID_MSMPEGAudDecMFT, 0x70707B39, 0xB2CA, 0x4015, 0xAB, 0xEA, 0xF8, 0x44, 0x7D, 0x22, 0xD8, 0x8B);
EXTERN_GUID(CLSID_MSMPEGDecoderMFT, 0x2D709E52, 0x123F, 0x49b5, 0x9C, 0xBC, 0x9A, 0xF5, 0xCD, 0xE2, 0x8F, 0xB9);
EXTERN_GUID(CLSID_AudioResamplerMediaObject, 0xf447b69e, 0x1884, 0x4a7e, 0x80, 0x55, 0x34, 0x6f, 0x74, 0xd6, 0xed, 0xb3);
EXTERN_GUID(CLSID_MSVPxDecoder, 0xE3AAF548, 0xC9A4, 0x4C6E, 0x23, 0x4D, 0x5A, 0xDA, 0x37, 0x4B, 0x00, 0x00);
EXTERN_GUID(CLSID_MSOpusDecoder, 0x63e17c10, 0x2d43, 0x4c42, 0x8f, 0xe3, 0x8d, 0x8b, 0x63, 0xe4, 0x6a, 0x6a);
EXTERN_GUID(CLSID_VideoProcessorMFT, 0x88753b26, 0x5b24, 0x49bd, 0xb2, 0xe7, 0xc, 0x44, 0x5c, 0x78, 0xc9, 0x82);
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma endregion
#pragma region EME2 helper types
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
typedef 
enum MF_MEDIAKEYSESSION_TYPE
    {
        MF_MEDIAKEYSESSION_TYPE_TEMPORARY	= 0,
        MF_MEDIAKEYSESSION_TYPE_PERSISTENT_LICENSE	= ( MF_MEDIAKEYSESSION_TYPE_TEMPORARY + 1 ) ,
        MF_MEDIAKEYSESSION_TYPE_PERSISTENT_RELEASE_MESSAGE	= ( MF_MEDIAKEYSESSION_TYPE_PERSISTENT_LICENSE + 1 ) ,
        MF_MEDIAKEYSESSION_TYPE_PERSISTENT_USAGE_RECORD	= ( MF_MEDIAKEYSESSION_TYPE_PERSISTENT_RELEASE_MESSAGE + 1 ) 
    } 	MF_MEDIAKEYSESSION_TYPE;

typedef 
enum MF_MEDIAKEY_STATUS
    {
        MF_MEDIAKEY_STATUS_USABLE	= 0,
        MF_MEDIAKEY_STATUS_EXPIRED	= ( MF_MEDIAKEY_STATUS_USABLE + 1 ) ,
        MF_MEDIAKEY_STATUS_OUTPUT_DOWNSCALED	= ( MF_MEDIAKEY_STATUS_EXPIRED + 1 ) ,
        MF_MEDIAKEY_STATUS_OUTPUT_NOT_ALLOWED	= ( MF_MEDIAKEY_STATUS_OUTPUT_DOWNSCALED + 1 ) ,
        MF_MEDIAKEY_STATUS_STATUS_PENDING	= ( MF_MEDIAKEY_STATUS_OUTPUT_NOT_ALLOWED + 1 ) ,
        MF_MEDIAKEY_STATUS_INTERNAL_ERROR	= ( MF_MEDIAKEY_STATUS_STATUS_PENDING + 1 ) ,
        MF_MEDIAKEY_STATUS_RELEASED	= ( MF_MEDIAKEY_STATUS_INTERNAL_ERROR + 1 ) ,
        MF_MEDIAKEY_STATUS_OUTPUT_RESTRICTED	= ( MF_MEDIAKEY_STATUS_RELEASED + 1 ) 
    } 	MF_MEDIAKEY_STATUS;

typedef struct MFMediaKeyStatus
    {
    BYTE *pbKeyId;
    UINT cbKeyId;
    MF_MEDIAKEY_STATUS eMediaKeyStatus;
    } 	MFMediaKeyStatus;

typedef 
enum MF_MEDIAKEYSESSION_MESSAGETYPE
    {
        MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_REQUEST	= 0,
        MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RENEWAL	= 1,
        MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RELEASE	= 2,
        MF_MEDIAKEYSESSION_MESSAGETYPE_INDIVIDUALIZATION_REQUEST	= 3
    } 	MF_MEDIAKEYSESSION_MESSAGETYPE;

typedef 
enum _MF_CROSS_ORIGIN_POLICY
    {
        MF_CROSS_ORIGIN_POLICY_NONE	= 0,
        MF_CROSS_ORIGIN_POLICY_ANONYMOUS	= 1,
        MF_CROSS_ORIGIN_POLICY_USE_CREDENTIALS	= 2
    } 	MF_CROSS_ORIGIN_POLICY;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0104_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0104_v0_0_s_ifspec;

#ifndef __IMFNetCrossOriginSupport_INTERFACE_DEFINED__
#define __IMFNetCrossOriginSupport_INTERFACE_DEFINED__

/* interface IMFNetCrossOriginSupport */
/* [local][unique][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFNetCrossOriginSupport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bc2b7d44-a72d-49d5-8376-1480dee58b22")
    IMFNetCrossOriginSupport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCrossOriginPolicy( 
            /* [annotation][out] */ 
            _Out_  MF_CROSS_ORIGIN_POLICY *pPolicy) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSourceOrigin( 
            /* [annotation][out] */ 
            _Out_  LPWSTR *wszSourceOrigin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSameOrigin( 
            /* [in] */ LPCWSTR wszURL,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsSameOrigin) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFNetCrossOriginSupportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFNetCrossOriginSupport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFNetCrossOriginSupport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFNetCrossOriginSupport * This);
        
        DECLSPEC_XFGVIRT(IMFNetCrossOriginSupport, GetCrossOriginPolicy)
        HRESULT ( STDMETHODCALLTYPE *GetCrossOriginPolicy )( 
            IMFNetCrossOriginSupport * This,
            /* [annotation][out] */ 
            _Out_  MF_CROSS_ORIGIN_POLICY *pPolicy);
        
        DECLSPEC_XFGVIRT(IMFNetCrossOriginSupport, GetSourceOrigin)
        HRESULT ( STDMETHODCALLTYPE *GetSourceOrigin )( 
            IMFNetCrossOriginSupport * This,
            /* [annotation][out] */ 
            _Out_  LPWSTR *wszSourceOrigin);
        
        DECLSPEC_XFGVIRT(IMFNetCrossOriginSupport, IsSameOrigin)
        HRESULT ( STDMETHODCALLTYPE *IsSameOrigin )( 
            IMFNetCrossOriginSupport * This,
            /* [in] */ LPCWSTR wszURL,
            /* [annotation][out] */ 
            _Out_  BOOL *pfIsSameOrigin);
        
        END_INTERFACE
    } IMFNetCrossOriginSupportVtbl;

    interface IMFNetCrossOriginSupport
    {
        CONST_VTBL struct IMFNetCrossOriginSupportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFNetCrossOriginSupport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFNetCrossOriginSupport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFNetCrossOriginSupport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFNetCrossOriginSupport_GetCrossOriginPolicy(This,pPolicy)	\
    ( (This)->lpVtbl -> GetCrossOriginPolicy(This,pPolicy) ) 

#define IMFNetCrossOriginSupport_GetSourceOrigin(This,wszSourceOrigin)	\
    ( (This)->lpVtbl -> GetSourceOrigin(This,wszSourceOrigin) ) 

#define IMFNetCrossOriginSupport_IsSameOrigin(This,wszURL,pfIsSameOrigin)	\
    ( (This)->lpVtbl -> IsSameOrigin(This,wszURL,pfIsSameOrigin) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFNetCrossOriginSupport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0105 */
/* [local] */ 

EXTERN_GUID(MFNETSOURCE_CROSS_ORIGIN_SUPPORT, 0x9842207c, 0xb02c, 0x4271, 0xa2, 0xfc, 0x72, 0xe4, 0x93, 0x8, 0xe5, 0xc2);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0105_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0105_v0_0_s_ifspec;

#ifndef __IMFHttpDownloadRequest_INTERFACE_DEFINED__
#define __IMFHttpDownloadRequest_INTERFACE_DEFINED__

/* interface IMFHttpDownloadRequest */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFHttpDownloadRequest;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F779FDDF-26E7-4270-8A8B-B983D1859DE0")
    IMFHttpDownloadRequest : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddHeader( 
            /* [annotation][in] */ 
            _In_  LPCWSTR szHeader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginSendRequest( 
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(cbPayload)  const BYTE *pbPayload,
            /* [annotation][in] */ 
            _In_  ULONG cbPayload,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndSendRequest( 
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginReceiveResponse( 
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndReceiveResponse( 
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginReadPayload( 
            /* [annotation][size_is][out] */ 
            _Out_writes_(cb)  BYTE *pb,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndReadPayload( 
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwOffset,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryHeader( 
            /* [annotation][in] */ 
            _In_  LPCWSTR szHeaderName,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppszHeaderValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetURL( 
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppszURL) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE HasNullSourceOrigin( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfNullSourceOrigin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeSeekResult( 
            /* [annotation][out] */ 
            _Out_  QWORD *pqwStartTime,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwStopTime,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetHttpStatus( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHttpStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAtEndOfPayload( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfAtEndOfPayload) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalLength( 
            /* [annotation][out] */ 
            _Out_  QWORD *pqwTotalLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRangeEndOffset( 
            /* [annotation][out] */ 
            _Out_  QWORD *pqwRangeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFHttpDownloadRequestVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFHttpDownloadRequest * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFHttpDownloadRequest * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFHttpDownloadRequest * This);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, AddHeader)
        HRESULT ( STDMETHODCALLTYPE *AddHeader )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR szHeader);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, BeginSendRequest)
        HRESULT ( STDMETHODCALLTYPE *BeginSendRequest )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][size_is][in] */ 
            _In_reads_opt_(cbPayload)  const BYTE *pbPayload,
            /* [annotation][in] */ 
            _In_  ULONG cbPayload,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, EndSendRequest)
        HRESULT ( STDMETHODCALLTYPE *EndSendRequest )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, BeginReceiveResponse)
        HRESULT ( STDMETHODCALLTYPE *BeginReceiveResponse )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, EndReceiveResponse)
        HRESULT ( STDMETHODCALLTYPE *EndReceiveResponse )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, BeginReadPayload)
        HRESULT ( STDMETHODCALLTYPE *BeginReadPayload )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_(cb)  BYTE *pb,
            /* [annotation][in] */ 
            _In_  ULONG cb,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, EndReadPayload)
        HRESULT ( STDMETHODCALLTYPE *EndReadPayload )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwOffset,
            /* [annotation][out] */ 
            _Out_  ULONG *pcbRead);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, QueryHeader)
        HRESULT ( STDMETHODCALLTYPE *QueryHeader )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR szHeaderName,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppszHeaderValue);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, GetURL)
        HRESULT ( STDMETHODCALLTYPE *GetURL )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Outptr_  LPWSTR *ppszURL);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, HasNullSourceOrigin)
        HRESULT ( STDMETHODCALLTYPE *HasNullSourceOrigin )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfNullSourceOrigin);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, GetTimeSeekResult)
        HRESULT ( STDMETHODCALLTYPE *GetTimeSeekResult )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwStartTime,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwStopTime,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwDuration);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, GetHttpStatus)
        HRESULT ( STDMETHODCALLTYPE *GetHttpStatus )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwHttpStatus);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, GetAtEndOfPayload)
        HRESULT ( STDMETHODCALLTYPE *GetAtEndOfPayload )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfAtEndOfPayload);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, GetTotalLength)
        HRESULT ( STDMETHODCALLTYPE *GetTotalLength )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwTotalLength);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, GetRangeEndOffset)
        HRESULT ( STDMETHODCALLTYPE *GetRangeEndOffset )( 
            IMFHttpDownloadRequest * This,
            /* [annotation][out] */ 
            _Out_  QWORD *pqwRangeEnd);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadRequest, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IMFHttpDownloadRequest * This);
        
        END_INTERFACE
    } IMFHttpDownloadRequestVtbl;

    interface IMFHttpDownloadRequest
    {
        CONST_VTBL struct IMFHttpDownloadRequestVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFHttpDownloadRequest_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFHttpDownloadRequest_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFHttpDownloadRequest_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFHttpDownloadRequest_AddHeader(This,szHeader)	\
    ( (This)->lpVtbl -> AddHeader(This,szHeader) ) 

#define IMFHttpDownloadRequest_BeginSendRequest(This,pbPayload,cbPayload,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginSendRequest(This,pbPayload,cbPayload,pCallback,punkState) ) 

#define IMFHttpDownloadRequest_EndSendRequest(This,pResult)	\
    ( (This)->lpVtbl -> EndSendRequest(This,pResult) ) 

#define IMFHttpDownloadRequest_BeginReceiveResponse(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginReceiveResponse(This,pCallback,punkState) ) 

#define IMFHttpDownloadRequest_EndReceiveResponse(This,pResult)	\
    ( (This)->lpVtbl -> EndReceiveResponse(This,pResult) ) 

#define IMFHttpDownloadRequest_BeginReadPayload(This,pb,cb,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginReadPayload(This,pb,cb,pCallback,punkState) ) 

#define IMFHttpDownloadRequest_EndReadPayload(This,pResult,pqwOffset,pcbRead)	\
    ( (This)->lpVtbl -> EndReadPayload(This,pResult,pqwOffset,pcbRead) ) 

#define IMFHttpDownloadRequest_QueryHeader(This,szHeaderName,dwIndex,ppszHeaderValue)	\
    ( (This)->lpVtbl -> QueryHeader(This,szHeaderName,dwIndex,ppszHeaderValue) ) 

#define IMFHttpDownloadRequest_GetURL(This,ppszURL)	\
    ( (This)->lpVtbl -> GetURL(This,ppszURL) ) 

#define IMFHttpDownloadRequest_HasNullSourceOrigin(This,pfNullSourceOrigin)	\
    ( (This)->lpVtbl -> HasNullSourceOrigin(This,pfNullSourceOrigin) ) 

#define IMFHttpDownloadRequest_GetTimeSeekResult(This,pqwStartTime,pqwStopTime,pqwDuration)	\
    ( (This)->lpVtbl -> GetTimeSeekResult(This,pqwStartTime,pqwStopTime,pqwDuration) ) 

#define IMFHttpDownloadRequest_GetHttpStatus(This,pdwHttpStatus)	\
    ( (This)->lpVtbl -> GetHttpStatus(This,pdwHttpStatus) ) 

#define IMFHttpDownloadRequest_GetAtEndOfPayload(This,pfAtEndOfPayload)	\
    ( (This)->lpVtbl -> GetAtEndOfPayload(This,pfAtEndOfPayload) ) 

#define IMFHttpDownloadRequest_GetTotalLength(This,pqwTotalLength)	\
    ( (This)->lpVtbl -> GetTotalLength(This,pqwTotalLength) ) 

#define IMFHttpDownloadRequest_GetRangeEndOffset(This,pqwRangeEnd)	\
    ( (This)->lpVtbl -> GetRangeEndOffset(This,pqwRangeEnd) ) 

#define IMFHttpDownloadRequest_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFHttpDownloadRequest_INTERFACE_DEFINED__ */


#ifndef __IMFHttpDownloadSession_INTERFACE_DEFINED__
#define __IMFHttpDownloadSession_INTERFACE_DEFINED__

/* interface IMFHttpDownloadSession */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFHttpDownloadSession;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("71FA9A2C-53CE-4662-A132-1A7E8CBF62DB")
    IMFHttpDownloadSession : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetServer( 
            /* [annotation][in] */ 
            _In_  LPCWSTR szServerName,
            /* [annotation][in] */ 
            _In_  DWORD nPort) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRequest( 
            /* [annotation][in] */ 
            _In_  LPCWSTR szObjectName,
            /* [annotation][in] */ 
            _In_  BOOL fBypassProxyCache,
            /* [annotation][in] */ 
            _In_  BOOL fSecure,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR szVerb,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR szReferrer,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFHttpDownloadRequest **ppRequest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Close( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFHttpDownloadSessionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFHttpDownloadSession * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFHttpDownloadSession * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFHttpDownloadSession * This);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadSession, SetServer)
        HRESULT ( STDMETHODCALLTYPE *SetServer )( 
            IMFHttpDownloadSession * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR szServerName,
            /* [annotation][in] */ 
            _In_  DWORD nPort);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadSession, CreateRequest)
        HRESULT ( STDMETHODCALLTYPE *CreateRequest )( 
            IMFHttpDownloadSession * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR szObjectName,
            /* [annotation][in] */ 
            _In_  BOOL fBypassProxyCache,
            /* [annotation][in] */ 
            _In_  BOOL fSecure,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR szVerb,
            /* [annotation][in] */ 
            _In_opt_  LPCWSTR szReferrer,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFHttpDownloadRequest **ppRequest);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadSession, Close)
        HRESULT ( STDMETHODCALLTYPE *Close )( 
            IMFHttpDownloadSession * This);
        
        END_INTERFACE
    } IMFHttpDownloadSessionVtbl;

    interface IMFHttpDownloadSession
    {
        CONST_VTBL struct IMFHttpDownloadSessionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFHttpDownloadSession_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFHttpDownloadSession_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFHttpDownloadSession_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFHttpDownloadSession_SetServer(This,szServerName,nPort)	\
    ( (This)->lpVtbl -> SetServer(This,szServerName,nPort) ) 

#define IMFHttpDownloadSession_CreateRequest(This,szObjectName,fBypassProxyCache,fSecure,szVerb,szReferrer,ppRequest)	\
    ( (This)->lpVtbl -> CreateRequest(This,szObjectName,fBypassProxyCache,fSecure,szVerb,szReferrer,ppRequest) ) 

#define IMFHttpDownloadSession_Close(This)	\
    ( (This)->lpVtbl -> Close(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFHttpDownloadSession_INTERFACE_DEFINED__ */


#ifndef __IMFHttpDownloadSessionProvider_INTERFACE_DEFINED__
#define __IMFHttpDownloadSessionProvider_INTERFACE_DEFINED__

/* interface IMFHttpDownloadSessionProvider */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFHttpDownloadSessionProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1B4CF4B9-3A16-4115-839D-03CC5C99DF01")
    IMFHttpDownloadSessionProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateHttpDownloadSession( 
            /* [annotation][in] */ 
            _In_  LPCWSTR wszScheme,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFHttpDownloadSession **ppDownloadSession) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFHttpDownloadSessionProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFHttpDownloadSessionProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFHttpDownloadSessionProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFHttpDownloadSessionProvider * This);
        
        DECLSPEC_XFGVIRT(IMFHttpDownloadSessionProvider, CreateHttpDownloadSession)
        HRESULT ( STDMETHODCALLTYPE *CreateHttpDownloadSession )( 
            IMFHttpDownloadSessionProvider * This,
            /* [annotation][in] */ 
            _In_  LPCWSTR wszScheme,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFHttpDownloadSession **ppDownloadSession);
        
        END_INTERFACE
    } IMFHttpDownloadSessionProviderVtbl;

    interface IMFHttpDownloadSessionProvider
    {
        CONST_VTBL struct IMFHttpDownloadSessionProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFHttpDownloadSessionProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFHttpDownloadSessionProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFHttpDownloadSessionProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFHttpDownloadSessionProvider_CreateHttpDownloadSession(This,wszScheme,ppDownloadSession)	\
    ( (This)->lpVtbl -> CreateHttpDownloadSession(This,wszScheme,ppDownloadSession) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFHttpDownloadSessionProvider_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0108 */
/* [local] */ 

EXTERN_GUID(MFNETSOURCE_HTTP_DOWNLOAD_SESSION_PROVIDER, 0x7d55081e, 0x307d, 0x4d6d, 0xa6, 0x63, 0xa9, 0x3b, 0xe9, 0x7c, 0x4b, 0x5c);
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma endregion
#if (WINVER >= _WIN32_WINNT_WIN10) 
typedef 
enum MF_MEDIASOURCE_STATUS_INFO
    {
        MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED	= 0,
        MF_MEDIASOURCE_STATUS_INFO_UNKNOWN	= 1
    } 	MF_MEDIASOURCE_STATUS_INFO;

EXTERN_GUID(MF_SD_MEDIASOURCE_STATUS, 0x1913678b, 0xfc0f, 0x44da, 0x8f, 0x43, 0x1b, 0xa3, 0xb5, 0x26, 0xf4, 0xae);
typedef struct _MF_VIDEO_SPHERICAL_VIEWDIRECTION
    {
    int iHeading;
    int iPitch;
    int iRoll;
    } 	MF_VIDEO_SPHERICAL_VIEWDIRECTION;

#define MF_UNKNOWN_DURATION 0
EXTERN_GUID(MF_SD_VIDEO_SPHERICAL, 0xa51da449, 0x3fdc, 0x478c, 0xbc, 0xb5, 0x30, 0xbe, 0x76, 0x59, 0x5f, 0x55);
EXTERN_GUID(MF_SD_VIDEO_SPHERICAL_FORMAT, 0x4a8fc407, 0x6ea1, 0x46c8, 0xb5, 0x67, 0x69, 0x71, 0xd4, 0xa1, 0x39, 0xc3);
EXTERN_GUID(MF_SD_VIDEO_SPHERICAL_INITIAL_VIEWDIRECTION, 0x11d25a49, 0xbb62, 0x467f, 0x9d, 0xb1, 0xc1, 0x71, 0x65, 0x71, 0x6c, 0x49);
EXTERN_GUID(MF_MEDIASOURCE_EXPOSE_ALL_STREAMS, 0xe7f250b8, 0x8fd9, 0x4a09, 0xb6, 0xc1, 0x6a, 0x31, 0x5c, 0x7c, 0x72, 0xe);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0108_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0108_v0_0_s_ifspec;

#ifndef __IMFMediaSource2_INTERFACE_DEFINED__
#define __IMFMediaSource2_INTERFACE_DEFINED__

/* interface IMFMediaSource2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaSource2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FBB03414-D13B-4786-8319-5AC51FC0A136")
    IMFMediaSource2 : public IMFMediaSourceEx
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaSource2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaSource2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaSource2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaSource2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            IMFMediaSource2 * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaSource2 * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaSource2 * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            IMFMediaSource2 * This,
            /* [in] */ MediaEventType met,
            /* [in] */ REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, GetCharacteristics)
        HRESULT ( STDMETHODCALLTYPE *GetCharacteristics )( 
            IMFMediaSource2 * This,
            /* [out] */ DWORD *pdwCharacteristics);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, CreatePresentationDescriptor)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *CreatePresentationDescriptor )( 
            IMFMediaSource2 * This,
            /* [annotation][out] */ 
            _Outptr_  IMFPresentationDescriptor **ppPresentationDescriptor);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IMFMediaSource2 * This,
            /* [in] */ IMFPresentationDescriptor *pPresentationDescriptor,
            /* [unique][in] */ const GUID *pguidTimeFormat,
            /* [unique][in] */ const PROPVARIANT *pvarStartPosition);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMFMediaSource2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IMFMediaSource2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSource, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFMediaSource2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceEx, GetSourceAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetSourceAttributes )( 
            IMFMediaSource2 * This,
            /* [out] */ IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceEx, GetStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetStreamAttributes )( 
            IMFMediaSource2 * This,
            /* [in] */ DWORD dwStreamIdentifier,
            /* [out] */ IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFMediaSourceEx, SetD3DManager)
        HRESULT ( STDMETHODCALLTYPE *SetD3DManager )( 
            IMFMediaSource2 * This,
            /* [in] */ IUnknown *pManager);
        
        DECLSPEC_XFGVIRT(IMFMediaSource2, SetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            IMFMediaSource2 * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamID,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType);
        
        END_INTERFACE
    } IMFMediaSource2Vtbl;

    interface IMFMediaSource2
    {
        CONST_VTBL struct IMFMediaSource2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaSource2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaSource2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaSource2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaSource2_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaSource2_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaSource2_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaSource2_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFMediaSource2_GetCharacteristics(This,pdwCharacteristics)	\
    ( (This)->lpVtbl -> GetCharacteristics(This,pdwCharacteristics) ) 

#define IMFMediaSource2_CreatePresentationDescriptor(This,ppPresentationDescriptor)	\
    ( (This)->lpVtbl -> CreatePresentationDescriptor(This,ppPresentationDescriptor) ) 

#define IMFMediaSource2_Start(This,pPresentationDescriptor,pguidTimeFormat,pvarStartPosition)	\
    ( (This)->lpVtbl -> Start(This,pPresentationDescriptor,pguidTimeFormat,pvarStartPosition) ) 

#define IMFMediaSource2_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFMediaSource2_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMFMediaSource2_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 


#define IMFMediaSource2_GetSourceAttributes(This,ppAttributes)	\
    ( (This)->lpVtbl -> GetSourceAttributes(This,ppAttributes) ) 

#define IMFMediaSource2_GetStreamAttributes(This,dwStreamIdentifier,ppAttributes)	\
    ( (This)->lpVtbl -> GetStreamAttributes(This,dwStreamIdentifier,ppAttributes) ) 

#define IMFMediaSource2_SetD3DManager(This,pManager)	\
    ( (This)->lpVtbl -> SetD3DManager(This,pManager) ) 


#define IMFMediaSource2_SetMediaType(This,dwStreamID,pMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,dwStreamID,pMediaType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaSource2_INTERFACE_DEFINED__ */


#ifndef __IMFMediaStream2_INTERFACE_DEFINED__
#define __IMFMediaStream2_INTERFACE_DEFINED__

/* interface IMFMediaStream2 */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFMediaStream2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C5BC37D6-75C7-46A1-A132-81B5F723C20F")
    IMFMediaStream2 : public IMFMediaStream
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetStreamState( 
            /* [annotation][in] */ 
            _In_  MF_STREAM_STATE value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamState( 
            /* [annotation][out] */ 
            _Out_  MF_STREAM_STATE *value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFMediaStream2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFMediaStream2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFMediaStream2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFMediaStream2 * This);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, GetEvent)
        HRESULT ( STDMETHODCALLTYPE *GetEvent )( 
            IMFMediaStream2 * This,
            /* [in] */ DWORD dwFlags,
            /* [out] */ IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, BeginGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *BeginGetEvent )( 
            IMFMediaStream2 * This,
            /* [in] */ IMFAsyncCallback *pCallback,
            /* [in] */ IUnknown *punkState);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, EndGetEvent)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *EndGetEvent )( 
            IMFMediaStream2 * This,
            /* [in] */ IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _Out_  IMFMediaEvent **ppEvent);
        
        DECLSPEC_XFGVIRT(IMFMediaEventGenerator, QueueEvent)
        HRESULT ( STDMETHODCALLTYPE *QueueEvent )( 
            IMFMediaStream2 * This,
            /* [in] */ MediaEventType met,
            /* [in] */ REFGUID guidExtendedType,
            /* [in] */ HRESULT hrStatus,
            /* [unique][in] */ const PROPVARIANT *pvValue);
        
        DECLSPEC_XFGVIRT(IMFMediaStream, GetMediaSource)
        HRESULT ( STDMETHODCALLTYPE *GetMediaSource )( 
            IMFMediaStream2 * This,
            /* [out] */ IMFMediaSource **ppMediaSource);
        
        DECLSPEC_XFGVIRT(IMFMediaStream, GetStreamDescriptor)
        HRESULT ( STDMETHODCALLTYPE *GetStreamDescriptor )( 
            IMFMediaStream2 * This,
            /* [out] */ IMFStreamDescriptor **ppStreamDescriptor);
        
        DECLSPEC_XFGVIRT(IMFMediaStream, RequestSample)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *RequestSample )( 
            IMFMediaStream2 * This,
            /* [in] */ IUnknown *pToken);
        
        DECLSPEC_XFGVIRT(IMFMediaStream2, SetStreamState)
        HRESULT ( STDMETHODCALLTYPE *SetStreamState )( 
            IMFMediaStream2 * This,
            /* [annotation][in] */ 
            _In_  MF_STREAM_STATE value);
        
        DECLSPEC_XFGVIRT(IMFMediaStream2, GetStreamState)
        HRESULT ( STDMETHODCALLTYPE *GetStreamState )( 
            IMFMediaStream2 * This,
            /* [annotation][out] */ 
            _Out_  MF_STREAM_STATE *value);
        
        END_INTERFACE
    } IMFMediaStream2Vtbl;

    interface IMFMediaStream2
    {
        CONST_VTBL struct IMFMediaStream2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFMediaStream2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFMediaStream2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFMediaStream2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFMediaStream2_GetEvent(This,dwFlags,ppEvent)	\
    ( (This)->lpVtbl -> GetEvent(This,dwFlags,ppEvent) ) 

#define IMFMediaStream2_BeginGetEvent(This,pCallback,punkState)	\
    ( (This)->lpVtbl -> BeginGetEvent(This,pCallback,punkState) ) 

#define IMFMediaStream2_EndGetEvent(This,pResult,ppEvent)	\
    ( (This)->lpVtbl -> EndGetEvent(This,pResult,ppEvent) ) 

#define IMFMediaStream2_QueueEvent(This,met,guidExtendedType,hrStatus,pvValue)	\
    ( (This)->lpVtbl -> QueueEvent(This,met,guidExtendedType,hrStatus,pvValue) ) 


#define IMFMediaStream2_GetMediaSource(This,ppMediaSource)	\
    ( (This)->lpVtbl -> GetMediaSource(This,ppMediaSource) ) 

#define IMFMediaStream2_GetStreamDescriptor(This,ppStreamDescriptor)	\
    ( (This)->lpVtbl -> GetStreamDescriptor(This,ppStreamDescriptor) ) 

#define IMFMediaStream2_RequestSample(This,pToken)	\
    ( (This)->lpVtbl -> RequestSample(This,pToken) ) 


#define IMFMediaStream2_SetStreamState(This,value)	\
    ( (This)->lpVtbl -> SetStreamState(This,value) ) 

#define IMFMediaStream2_GetStreamState(This,value)	\
    ( (This)->lpVtbl -> GetStreamState(This,value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFMediaStream2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0110 */
/* [local] */ 

#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#if (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region Application or Games Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES)
EXTERN_GUID(MF_ST_MEDIASOURCE_COLLECTION, 0x616DE972, 0x83AD, 0x4950, 0x81, 0x70, 0x63, 0x0D, 0x19, 0xCB, 0xE3, 0x07);
EXTERN_GUID(MF_DEVICESTREAM_FILTER_KSCONTROL, 0x46783CCA, 0x3DF5, 0x4923, 0xA9, 0xEF, 0x36, 0xB7, 0x22, 0x3E, 0xDD, 0xE0);
EXTERN_GUID(MF_DEVICESTREAM_PIN_KSCONTROL, 0xEF3EF9A7, 0x87F2, 0x48CA, 0xBE, 0x02, 0x67, 0x48, 0x78, 0x91, 0x8E, 0x98);
EXTERN_GUID(MF_DEVICESTREAM_SOURCE_ATTRIBUTES, 0x2F8CB617, 0x361B, 0x434F, 0x85, 0xEA, 0x99, 0xA0, 0x3E, 0x1C, 0xE4, 0xE0);
EXTERN_GUID( MF_DEVICESTREAM_FRAMESERVER_HIDDEN, 0xF402567B, 0x4D91, 0x4179, 0x96, 0xD1, 0x74, 0xC8, 0x48, 0x0C, 0x20, 0x34);
EXTERN_GUID( MF_STF_VERSION_INFO, 0x6770BD39, 0xEF82, 0x44EE, 0xA4, 0x9B, 0x93, 0x4B, 0xEB, 0x24, 0xAE, 0xF7);
EXTERN_GUID( MF_STF_VERSION_DATE, 0x31A165D5, 0xDF67, 0x4095, 0x8E, 0x44, 0x88, 0x68, 0xFC, 0x20, 0xDB, 0xFD);
EXTERN_GUID( MF_DEVICESTREAM_REQUIRED_CAPABILITIES, 0x6D8B957E, 0x7CF6, 0x43F4, 0xAF, 0x56, 0x9C, 0x0E, 0x1E, 0x4F, 0xCB, 0xE1);
EXTERN_GUID( MF_DEVICESTREAM_REQUIRED_SDDL, 0x331AE85D, 0xC0D3, 0x49BA, 0x83, 0xBA, 0x82, 0xA1, 0x2D, 0x63, 0xCD, 0xD6);
EXTERN_GUID(MF_DEVICEMFT_SENSORPROFILE_COLLECTION, 0x36EBDC44, 0xB12C, 0x441B, 0x89, 0xF4, 0x08, 0xB2, 0xF4, 0x1A, 0x9C, 0xFC );
EXTERN_GUID(MF_DEVICESTREAM_SENSORSTREAM_ID, 0xE35B9FE4, 0x0659, 0x4CAD, 0xBB, 0x51, 0x33, 0x16, 0x0B, 0xE7, 0xE4, 0x13 );
typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_mfidl_0000_0110_0001
    {
        MFSensorDeviceType_Unknown	= 0,
        MFSensorDeviceType_Device	= ( MFSensorDeviceType_Unknown + 1 ) ,
        MFSensorDeviceType_MediaSource	= ( MFSensorDeviceType_Device + 1 ) ,
        MFSensorDeviceType_FrameProvider	= ( MFSensorDeviceType_MediaSource + 1 ) ,
        MFSensorDeviceType_SensorTransform	= ( MFSensorDeviceType_FrameProvider + 1 ) 
    } 	MFSensorDeviceType;

typedef /* [public][public][public] */ 
enum __MIDL___MIDL_itf_mfidl_0000_0110_0002
    {
        MFSensorStreamType_Unknown	= 0,
        MFSensorStreamType_Input	= ( MFSensorStreamType_Unknown + 1 ) ,
        MFSensorStreamType_Output	= ( MFSensorStreamType_Input + 1 ) 
    } 	MFSensorStreamType;

typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_mfidl_0000_0110_0003
    {
        MFSensorDeviceMode_Controller	= 0,
        MFSensorDeviceMode_Shared	= ( MFSensorDeviceMode_Controller + 1 ) 
    } 	MFSensorDeviceMode;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0110_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0110_v0_0_s_ifspec;

#ifndef __IMFSensorDevice_INTERFACE_DEFINED__
#define __IMFSensorDevice_INTERFACE_DEFINED__

/* interface IMFSensorDevice */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FB9F48F2-2A18-4E28-9730-786F30F04DC4")
    IMFSensorDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDeviceId( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pDeviceId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceType( 
            /* [annotation][out] */ 
            _Out_  MFSensorDeviceType *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSymbolicLink( 
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchSymbolicLink)  LPWSTR SymbolicLink,
            /* [annotation][in] */ 
            _In_  LONG cchSymbolicLink,
            /* [annotation][out] */ 
            _Out_  LONG *pcchWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceAttributes( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamAttributesCount( 
            /* [annotation][in] */ 
            _In_  MFSensorStreamType eType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamAttributes( 
            /* [annotation][in] */ 
            _In_  MFSensorStreamType eType,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSensorDeviceMode( 
            /* [annotation][in] */ 
            _In_  MFSensorDeviceMode eMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSensorDeviceMode( 
            /* [annotation][out] */ 
            _Out_  MFSensorDeviceMode *peMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorDevice * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorDevice * This);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetDeviceId)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceId )( 
            IMFSensorDevice * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pDeviceId);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetDeviceType)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceType )( 
            IMFSensorDevice * This,
            /* [annotation][out] */ 
            _Out_  MFSensorDeviceType *pType);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IMFSensorDevice * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pFlags);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetSymbolicLink)
        HRESULT ( STDMETHODCALLTYPE *GetSymbolicLink )( 
            IMFSensorDevice * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchSymbolicLink)  LPWSTR SymbolicLink,
            /* [annotation][in] */ 
            _In_  LONG cchSymbolicLink,
            /* [annotation][out] */ 
            _Out_  LONG *pcchWritten);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetDeviceAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetDeviceAttributes )( 
            IMFSensorDevice * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetStreamAttributesCount)
        HRESULT ( STDMETHODCALLTYPE *GetStreamAttributesCount )( 
            IMFSensorDevice * This,
            /* [annotation][in] */ 
            _In_  MFSensorStreamType eType,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetStreamAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetStreamAttributes )( 
            IMFSensorDevice * This,
            /* [annotation][in] */ 
            _In_  MFSensorStreamType eType,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, SetSensorDeviceMode)
        HRESULT ( STDMETHODCALLTYPE *SetSensorDeviceMode )( 
            IMFSensorDevice * This,
            /* [annotation][in] */ 
            _In_  MFSensorDeviceMode eMode);
        
        DECLSPEC_XFGVIRT(IMFSensorDevice, GetSensorDeviceMode)
        HRESULT ( STDMETHODCALLTYPE *GetSensorDeviceMode )( 
            IMFSensorDevice * This,
            /* [annotation][out] */ 
            _Out_  MFSensorDeviceMode *peMode);
        
        END_INTERFACE
    } IMFSensorDeviceVtbl;

    interface IMFSensorDevice
    {
        CONST_VTBL struct IMFSensorDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorDevice_GetDeviceId(This,pDeviceId)	\
    ( (This)->lpVtbl -> GetDeviceId(This,pDeviceId) ) 

#define IMFSensorDevice_GetDeviceType(This,pType)	\
    ( (This)->lpVtbl -> GetDeviceType(This,pType) ) 

#define IMFSensorDevice_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define IMFSensorDevice_GetSymbolicLink(This,SymbolicLink,cchSymbolicLink,pcchWritten)	\
    ( (This)->lpVtbl -> GetSymbolicLink(This,SymbolicLink,cchSymbolicLink,pcchWritten) ) 

#define IMFSensorDevice_GetDeviceAttributes(This,ppAttributes)	\
    ( (This)->lpVtbl -> GetDeviceAttributes(This,ppAttributes) ) 

#define IMFSensorDevice_GetStreamAttributesCount(This,eType,pdwCount)	\
    ( (This)->lpVtbl -> GetStreamAttributesCount(This,eType,pdwCount) ) 

#define IMFSensorDevice_GetStreamAttributes(This,eType,dwIndex,ppAttributes)	\
    ( (This)->lpVtbl -> GetStreamAttributes(This,eType,dwIndex,ppAttributes) ) 

#define IMFSensorDevice_SetSensorDeviceMode(This,eMode)	\
    ( (This)->lpVtbl -> SetSensorDeviceMode(This,eMode) ) 

#define IMFSensorDevice_GetSensorDeviceMode(This,peMode)	\
    ( (This)->lpVtbl -> GetSensorDeviceMode(This,peMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorDevice_INTERFACE_DEFINED__ */


#ifndef __IMFSensorGroup_INTERFACE_DEFINED__
#define __IMFSensorGroup_INTERFACE_DEFINED__

/* interface IMFSensorGroup */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorGroup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4110243A-9757-461F-89F1-F22345BCAB4E")
    IMFSensorGroup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSymbolicLink( 
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchSymbolicLink)  LPWSTR SymbolicLink,
            /* [annotation][in] */ 
            _In_  LONG cchSymbolicLink,
            /* [annotation][out] */ 
            _Out_  LONG *pcchWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFlags( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSensorGroupAttributes( 
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSensorDeviceCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSensorDevice( 
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorDevice **ppDevice) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultSensorDeviceIndex( 
            /* [annotation][in] */ 
            _In_  DWORD dwIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultSensorDeviceIndex( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateMediaSource( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSource **ppSource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorGroupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorGroup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorGroup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorGroup * This);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, GetSymbolicLink)
        HRESULT ( STDMETHODCALLTYPE *GetSymbolicLink )( 
            IMFSensorGroup * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchSymbolicLink)  LPWSTR SymbolicLink,
            /* [annotation][in] */ 
            _In_  LONG cchSymbolicLink,
            /* [annotation][out] */ 
            _Out_  LONG *pcchWritten);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, GetFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFlags )( 
            IMFSensorGroup * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pFlags);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, GetSensorGroupAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetSensorGroupAttributes )( 
            IMFSensorGroup * This,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, GetSensorDeviceCount)
        HRESULT ( STDMETHODCALLTYPE *GetSensorDeviceCount )( 
            IMFSensorGroup * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, GetSensorDevice)
        HRESULT ( STDMETHODCALLTYPE *GetSensorDevice )( 
            IMFSensorGroup * This,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorDevice **ppDevice);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, SetDefaultSensorDeviceIndex)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSensorDeviceIndex )( 
            IMFSensorGroup * This,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, GetDefaultSensorDeviceIndex)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultSensorDeviceIndex )( 
            IMFSensorGroup * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwIndex);
        
        DECLSPEC_XFGVIRT(IMFSensorGroup, CreateMediaSource)
        HRESULT ( STDMETHODCALLTYPE *CreateMediaSource )( 
            IMFSensorGroup * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaSource **ppSource);
        
        END_INTERFACE
    } IMFSensorGroupVtbl;

    interface IMFSensorGroup
    {
        CONST_VTBL struct IMFSensorGroupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorGroup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorGroup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorGroup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorGroup_GetSymbolicLink(This,SymbolicLink,cchSymbolicLink,pcchWritten)	\
    ( (This)->lpVtbl -> GetSymbolicLink(This,SymbolicLink,cchSymbolicLink,pcchWritten) ) 

#define IMFSensorGroup_GetFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetFlags(This,pFlags) ) 

#define IMFSensorGroup_GetSensorGroupAttributes(This,ppAttributes)	\
    ( (This)->lpVtbl -> GetSensorGroupAttributes(This,ppAttributes) ) 

#define IMFSensorGroup_GetSensorDeviceCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetSensorDeviceCount(This,pdwCount) ) 

#define IMFSensorGroup_GetSensorDevice(This,dwIndex,ppDevice)	\
    ( (This)->lpVtbl -> GetSensorDevice(This,dwIndex,ppDevice) ) 

#define IMFSensorGroup_SetDefaultSensorDeviceIndex(This,dwIndex)	\
    ( (This)->lpVtbl -> SetDefaultSensorDeviceIndex(This,dwIndex) ) 

#define IMFSensorGroup_GetDefaultSensorDeviceIndex(This,pdwIndex)	\
    ( (This)->lpVtbl -> GetDefaultSensorDeviceIndex(This,pdwIndex) ) 

#define IMFSensorGroup_CreateMediaSource(This,ppSource)	\
    ( (This)->lpVtbl -> CreateMediaSource(This,ppSource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorGroup_INTERFACE_DEFINED__ */


#ifndef __IMFSensorStream_INTERFACE_DEFINED__
#define __IMFSensorStream_INTERFACE_DEFINED__

/* interface IMFSensorStream */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorStream;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E9A42171-C56E-498A-8B39-EDA5A070B7FC")
    IMFSensorStream : public IMFAttributes
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetMediaTypeCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaType( 
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CloneSensorStream( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorStream **ppStream) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorStreamVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorStream * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorStream * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorStream * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFSensorStream * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFSensorStream * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFSensorStream * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFSensorStream * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFSensorStream * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFSensorStream * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFSensorStream * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFSensorStream * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFSensorStream * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFSensorStream, GetMediaTypeCount)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTypeCount )( 
            IMFSensorStream * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFSensorStream, GetMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetMediaType )( 
            IMFSensorStream * This,
            /* [annotation][in] */ 
            _In_  DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFMediaType **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMFSensorStream, CloneSensorStream)
        HRESULT ( STDMETHODCALLTYPE *CloneSensorStream )( 
            IMFSensorStream * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorStream **ppStream);
        
        END_INTERFACE
    } IMFSensorStreamVtbl;

    interface IMFSensorStream
    {
        CONST_VTBL struct IMFSensorStreamVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorStream_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorStream_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorStream_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorStream_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFSensorStream_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFSensorStream_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFSensorStream_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFSensorStream_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFSensorStream_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFSensorStream_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFSensorStream_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFSensorStream_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFSensorStream_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFSensorStream_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFSensorStream_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFSensorStream_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFSensorStream_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFSensorStream_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFSensorStream_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFSensorStream_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFSensorStream_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFSensorStream_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFSensorStream_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFSensorStream_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFSensorStream_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFSensorStream_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFSensorStream_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFSensorStream_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFSensorStream_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFSensorStream_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFSensorStream_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFSensorStream_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFSensorStream_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFSensorStream_GetMediaTypeCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetMediaTypeCount(This,pdwCount) ) 

#define IMFSensorStream_GetMediaType(This,dwIndex,ppMediaType)	\
    ( (This)->lpVtbl -> GetMediaType(This,dwIndex,ppMediaType) ) 

#define IMFSensorStream_CloneSensorStream(This,ppStream)	\
    ( (This)->lpVtbl -> CloneSensorStream(This,ppStream) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorStream_INTERFACE_DEFINED__ */


#ifndef __IMFSensorTransformFactory_INTERFACE_DEFINED__
#define __IMFSensorTransformFactory_INTERFACE_DEFINED__

/* interface IMFSensorTransformFactory */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorTransformFactory;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("EED9C2EE-66B4-4F18-A697-AC7D3960215C")
    IMFSensorTransformFactory : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFactoryAttributes( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE InitializeFactory( 
            /* [annotation][in] */ 
            _In_  DWORD dwMaxTransformCount,
            /* [annotation][in] */ 
            _In_  IMFCollection *pSensorDevices,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransformInformation( 
            /* [annotation][in] */ 
            _In_  DWORD TransformIndex,
            /* [annotation][out] */ 
            _Out_  GUID *pguidTransformId,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFAttributes **ppAttributes,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCollection **ppStreamInformation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTransform( 
            /* [annotation][in] */ 
            _In_  REFGUID guidSensorTransformID,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFDeviceTransform **ppDeviceMFT) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorTransformFactoryVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorTransformFactory * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorTransformFactory * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorTransformFactory * This);
        
        DECLSPEC_XFGVIRT(IMFSensorTransformFactory, GetFactoryAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetFactoryAttributes )( 
            IMFSensorTransformFactory * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFAttributes **ppAttributes);
        
        DECLSPEC_XFGVIRT(IMFSensorTransformFactory, InitializeFactory)
        HRESULT ( STDMETHODCALLTYPE *InitializeFactory )( 
            IMFSensorTransformFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD dwMaxTransformCount,
            /* [annotation][in] */ 
            _In_  IMFCollection *pSensorDevices,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes);
        
        DECLSPEC_XFGVIRT(IMFSensorTransformFactory, GetTransformCount)
        HRESULT ( STDMETHODCALLTYPE *GetTransformCount )( 
            IMFSensorTransformFactory * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFSensorTransformFactory, GetTransformInformation)
        HRESULT ( STDMETHODCALLTYPE *GetTransformInformation )( 
            IMFSensorTransformFactory * This,
            /* [annotation][in] */ 
            _In_  DWORD TransformIndex,
            /* [annotation][out] */ 
            _Out_  GUID *pguidTransformId,
            /* [annotation][out] */ 
            _COM_Outptr_result_maybenull_  IMFAttributes **ppAttributes,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCollection **ppStreamInformation);
        
        DECLSPEC_XFGVIRT(IMFSensorTransformFactory, CreateTransform)
        HRESULT ( STDMETHODCALLTYPE *CreateTransform )( 
            IMFSensorTransformFactory * This,
            /* [annotation][in] */ 
            _In_  REFGUID guidSensorTransformID,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFDeviceTransform **ppDeviceMFT);
        
        END_INTERFACE
    } IMFSensorTransformFactoryVtbl;

    interface IMFSensorTransformFactory
    {
        CONST_VTBL struct IMFSensorTransformFactoryVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorTransformFactory_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorTransformFactory_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorTransformFactory_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorTransformFactory_GetFactoryAttributes(This,ppAttributes)	\
    ( (This)->lpVtbl -> GetFactoryAttributes(This,ppAttributes) ) 

#define IMFSensorTransformFactory_InitializeFactory(This,dwMaxTransformCount,pSensorDevices,pAttributes)	\
    ( (This)->lpVtbl -> InitializeFactory(This,dwMaxTransformCount,pSensorDevices,pAttributes) ) 

#define IMFSensorTransformFactory_GetTransformCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetTransformCount(This,pdwCount) ) 

#define IMFSensorTransformFactory_GetTransformInformation(This,TransformIndex,pguidTransformId,ppAttributes,ppStreamInformation)	\
    ( (This)->lpVtbl -> GetTransformInformation(This,TransformIndex,pguidTransformId,ppAttributes,ppStreamInformation) ) 

#define IMFSensorTransformFactory_CreateTransform(This,guidSensorTransformID,pAttributes,ppDeviceMFT)	\
    ( (This)->lpVtbl -> CreateTransform(This,guidSensorTransformID,pAttributes,ppDeviceMFT) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorTransformFactory_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0114 */
/* [local] */ 

STDAPI
MFCreateSensorGroup(
    _In_z_ LPCWSTR SensorGroupSymbolicLink,
    _COM_Outptr_ IMFSensorGroup** ppSensorGroup
    );

STDAPI
MFCreateSensorStream(
    _In_ DWORD StreamId,
    _In_opt_ IMFAttributes* pAttributes,
    _In_ IMFCollection* pMediaTypeCollection,
    _COM_Outptr_ IMFSensorStream** ppStream
    );

typedef /* [public][public][public][public] */ struct __MIDL___MIDL_itf_mfidl_0000_0114_0001
    {
    GUID Type;
    UINT32 Index;
    UINT32 Unused;
    } 	SENSORPROFILEID;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0114_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0114_v0_0_s_ifspec;

#ifndef __IMFSensorProfile_INTERFACE_DEFINED__
#define __IMFSensorProfile_INTERFACE_DEFINED__

/* interface IMFSensorProfile */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorProfile;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22F765D1-8DAB-4107-846D-56BAF72215E7")
    IMFSensorProfile : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProfileId( 
            /* [annotation][out] */ 
            _Out_  SENSORPROFILEID *pId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddProfileFilter( 
            /* [annotation][in] */ 
            _In_  UINT32 StreamId,
            /* [annotation][in] */ 
            _In_z_  LPCWSTR wzFilterSetString) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsMediaTypeSupported( 
            /* [annotation][in] */ 
            _In_  UINT32 StreamId,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][out] */ 
            _Out_  BOOL *pfSupported) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddBlockedControl( 
            /* [annotation][in] */ 
            _In_z_  LPCWSTR wzBlockedControl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorProfileVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorProfile * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorProfile * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorProfile * This);
        
        DECLSPEC_XFGVIRT(IMFSensorProfile, GetProfileId)
        HRESULT ( STDMETHODCALLTYPE *GetProfileId )( 
            IMFSensorProfile * This,
            /* [annotation][out] */ 
            _Out_  SENSORPROFILEID *pId);
        
        DECLSPEC_XFGVIRT(IMFSensorProfile, AddProfileFilter)
        HRESULT ( STDMETHODCALLTYPE *AddProfileFilter )( 
            IMFSensorProfile * This,
            /* [annotation][in] */ 
            _In_  UINT32 StreamId,
            /* [annotation][in] */ 
            _In_z_  LPCWSTR wzFilterSetString);
        
        DECLSPEC_XFGVIRT(IMFSensorProfile, IsMediaTypeSupported)
        HRESULT ( STDMETHODCALLTYPE *IsMediaTypeSupported )( 
            IMFSensorProfile * This,
            /* [annotation][in] */ 
            _In_  UINT32 StreamId,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType,
            /* [annotation][out] */ 
            _Out_  BOOL *pfSupported);
        
        DECLSPEC_XFGVIRT(IMFSensorProfile, AddBlockedControl)
        HRESULT ( STDMETHODCALLTYPE *AddBlockedControl )( 
            IMFSensorProfile * This,
            /* [annotation][in] */ 
            _In_z_  LPCWSTR wzBlockedControl);
        
        END_INTERFACE
    } IMFSensorProfileVtbl;

    interface IMFSensorProfile
    {
        CONST_VTBL struct IMFSensorProfileVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorProfile_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorProfile_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorProfile_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorProfile_GetProfileId(This,pId)	\
    ( (This)->lpVtbl -> GetProfileId(This,pId) ) 

#define IMFSensorProfile_AddProfileFilter(This,StreamId,wzFilterSetString)	\
    ( (This)->lpVtbl -> AddProfileFilter(This,StreamId,wzFilterSetString) ) 

#define IMFSensorProfile_IsMediaTypeSupported(This,StreamId,pMediaType,pfSupported)	\
    ( (This)->lpVtbl -> IsMediaTypeSupported(This,StreamId,pMediaType,pfSupported) ) 

#define IMFSensorProfile_AddBlockedControl(This,wzBlockedControl)	\
    ( (This)->lpVtbl -> AddBlockedControl(This,wzBlockedControl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorProfile_INTERFACE_DEFINED__ */


#ifndef __IMFSensorProfileCollection_INTERFACE_DEFINED__
#define __IMFSensorProfileCollection_INTERFACE_DEFINED__

/* interface IMFSensorProfileCollection */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorProfileCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C95EA55B-0187-48BE-9353-8D2507662351")
    IMFSensorProfileCollection : public IUnknown
    {
    public:
        virtual DWORD STDMETHODCALLTYPE GetProfileCount( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProfile( 
            /* [annotation][in] */ 
            _In_  DWORD Index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorProfile **ppProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddProfile( 
            /* [annotation][in] */ 
            _In_  IMFSensorProfile *pProfile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindProfile( 
            /* [annotation][in] */ 
            _In_  SENSORPROFILEID *ProfileId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorProfile **ppProfile) = 0;
        
        virtual void STDMETHODCALLTYPE RemoveProfileByIndex( 
            /* [annotation][in] */ 
            _In_  DWORD Index) = 0;
        
        virtual void STDMETHODCALLTYPE RemoveProfile( 
            /* [annotation][in] */ 
            _In_  SENSORPROFILEID *ProfileId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorProfileCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorProfileCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorProfileCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorProfileCollection * This);
        
        DECLSPEC_XFGVIRT(IMFSensorProfileCollection, GetProfileCount)
        DWORD ( STDMETHODCALLTYPE *GetProfileCount )( 
            IMFSensorProfileCollection * This);
        
        DECLSPEC_XFGVIRT(IMFSensorProfileCollection, GetProfile)
        HRESULT ( STDMETHODCALLTYPE *GetProfile )( 
            IMFSensorProfileCollection * This,
            /* [annotation][in] */ 
            _In_  DWORD Index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IMFSensorProfileCollection, AddProfile)
        HRESULT ( STDMETHODCALLTYPE *AddProfile )( 
            IMFSensorProfileCollection * This,
            /* [annotation][in] */ 
            _In_  IMFSensorProfile *pProfile);
        
        DECLSPEC_XFGVIRT(IMFSensorProfileCollection, FindProfile)
        HRESULT ( STDMETHODCALLTYPE *FindProfile )( 
            IMFSensorProfileCollection * This,
            /* [annotation][in] */ 
            _In_  SENSORPROFILEID *ProfileId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorProfile **ppProfile);
        
        DECLSPEC_XFGVIRT(IMFSensorProfileCollection, RemoveProfileByIndex)
        void ( STDMETHODCALLTYPE *RemoveProfileByIndex )( 
            IMFSensorProfileCollection * This,
            /* [annotation][in] */ 
            _In_  DWORD Index);
        
        DECLSPEC_XFGVIRT(IMFSensorProfileCollection, RemoveProfile)
        void ( STDMETHODCALLTYPE *RemoveProfile )( 
            IMFSensorProfileCollection * This,
            /* [annotation][in] */ 
            _In_  SENSORPROFILEID *ProfileId);
        
        END_INTERFACE
    } IMFSensorProfileCollectionVtbl;

    interface IMFSensorProfileCollection
    {
        CONST_VTBL struct IMFSensorProfileCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorProfileCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorProfileCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorProfileCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorProfileCollection_GetProfileCount(This)	\
    ( (This)->lpVtbl -> GetProfileCount(This) ) 

#define IMFSensorProfileCollection_GetProfile(This,Index,ppProfile)	\
    ( (This)->lpVtbl -> GetProfile(This,Index,ppProfile) ) 

#define IMFSensorProfileCollection_AddProfile(This,pProfile)	\
    ( (This)->lpVtbl -> AddProfile(This,pProfile) ) 

#define IMFSensorProfileCollection_FindProfile(This,ProfileId,ppProfile)	\
    ( (This)->lpVtbl -> FindProfile(This,ProfileId,ppProfile) ) 

#define IMFSensorProfileCollection_RemoveProfileByIndex(This,Index)	\
    ( (This)->lpVtbl -> RemoveProfileByIndex(This,Index) ) 

#define IMFSensorProfileCollection_RemoveProfile(This,ProfileId)	\
    ( (This)->lpVtbl -> RemoveProfile(This,ProfileId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorProfileCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0116 */
/* [local] */ 

STDAPI
MFCreateSensorProfile(
    _In_ REFGUID ProfileType,
    _In_ UINT32 ProfileIndex,
    _In_opt_z_ LPCWSTR Constraints,
    _COM_Outptr_ IMFSensorProfile** ppProfile
    );
STDAPI
MFCreateSensorProfileCollection(
    _COM_Outptr_ IMFSensorProfileCollection** ppSensorProfile
    );
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP | WINAPI_PARTITION_GAMES) */
#pragma endregion


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0116_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0116_v0_0_s_ifspec;

#ifndef __IMFSensorProcessActivity_INTERFACE_DEFINED__
#define __IMFSensorProcessActivity_INTERFACE_DEFINED__

/* interface IMFSensorProcessActivity */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorProcessActivity;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("39DC7F4A-B141-4719-813C-A7F46162A2B8")
    IMFSensorProcessActivity : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProcessId( 
            /* [annotation][out] */ 
            _Out_  ULONG *pPID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamingState( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfStreaming) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamingMode( 
            /* [annotation][out] */ 
            _Out_  MFSensorDeviceMode *pMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReportTime( 
            /* [annotation][out] */ 
            _Out_  FILETIME *pft) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorProcessActivityVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorProcessActivity * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorProcessActivity * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorProcessActivity * This);
        
        DECLSPEC_XFGVIRT(IMFSensorProcessActivity, GetProcessId)
        HRESULT ( STDMETHODCALLTYPE *GetProcessId )( 
            IMFSensorProcessActivity * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pPID);
        
        DECLSPEC_XFGVIRT(IMFSensorProcessActivity, GetStreamingState)
        HRESULT ( STDMETHODCALLTYPE *GetStreamingState )( 
            IMFSensorProcessActivity * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfStreaming);
        
        DECLSPEC_XFGVIRT(IMFSensorProcessActivity, GetStreamingMode)
        HRESULT ( STDMETHODCALLTYPE *GetStreamingMode )( 
            IMFSensorProcessActivity * This,
            /* [annotation][out] */ 
            _Out_  MFSensorDeviceMode *pMode);
        
        DECLSPEC_XFGVIRT(IMFSensorProcessActivity, GetReportTime)
        HRESULT ( STDMETHODCALLTYPE *GetReportTime )( 
            IMFSensorProcessActivity * This,
            /* [annotation][out] */ 
            _Out_  FILETIME *pft);
        
        END_INTERFACE
    } IMFSensorProcessActivityVtbl;

    interface IMFSensorProcessActivity
    {
        CONST_VTBL struct IMFSensorProcessActivityVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorProcessActivity_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorProcessActivity_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorProcessActivity_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorProcessActivity_GetProcessId(This,pPID)	\
    ( (This)->lpVtbl -> GetProcessId(This,pPID) ) 

#define IMFSensorProcessActivity_GetStreamingState(This,pfStreaming)	\
    ( (This)->lpVtbl -> GetStreamingState(This,pfStreaming) ) 

#define IMFSensorProcessActivity_GetStreamingMode(This,pMode)	\
    ( (This)->lpVtbl -> GetStreamingMode(This,pMode) ) 

#define IMFSensorProcessActivity_GetReportTime(This,pft)	\
    ( (This)->lpVtbl -> GetReportTime(This,pft) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorProcessActivity_INTERFACE_DEFINED__ */


#ifndef __IMFSensorActivityReport_INTERFACE_DEFINED__
#define __IMFSensorActivityReport_INTERFACE_DEFINED__

/* interface IMFSensorActivityReport */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorActivityReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("3E8C4BE1-A8C2-4528-90DE-2851BDE5FEAD")
    IMFSensorActivityReport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFriendlyName( 
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchFriendlyName)  LPWSTR FriendlyName,
            /* [annotation][in] */ 
            _In_  ULONG cchFriendlyName,
            /* [annotation][out] */ 
            _Out_  ULONG *pcchWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSymbolicLink( 
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchSymbolicLink)  LPWSTR SymbolicLink,
            /* [annotation][in] */ 
            _In_  ULONG cchSymbolicLink,
            /* [annotation][out] */ 
            _Out_  ULONG *pcchWritten) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProcessCount( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProcessActivity( 
            /* [annotation][in] */ 
            _In_  ULONG Index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorProcessActivity **ppProcessActivity) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorActivityReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorActivityReport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorActivityReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorActivityReport * This);
        
        DECLSPEC_XFGVIRT(IMFSensorActivityReport, GetFriendlyName)
        HRESULT ( STDMETHODCALLTYPE *GetFriendlyName )( 
            IMFSensorActivityReport * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchFriendlyName)  LPWSTR FriendlyName,
            /* [annotation][in] */ 
            _In_  ULONG cchFriendlyName,
            /* [annotation][out] */ 
            _Out_  ULONG *pcchWritten);
        
        DECLSPEC_XFGVIRT(IMFSensorActivityReport, GetSymbolicLink)
        HRESULT ( STDMETHODCALLTYPE *GetSymbolicLink )( 
            IMFSensorActivityReport * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_z_(cchSymbolicLink)  LPWSTR SymbolicLink,
            /* [annotation][in] */ 
            _In_  ULONG cchSymbolicLink,
            /* [annotation][out] */ 
            _Out_  ULONG *pcchWritten);
        
        DECLSPEC_XFGVIRT(IMFSensorActivityReport, GetProcessCount)
        HRESULT ( STDMETHODCALLTYPE *GetProcessCount )( 
            IMFSensorActivityReport * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcCount);
        
        DECLSPEC_XFGVIRT(IMFSensorActivityReport, GetProcessActivity)
        HRESULT ( STDMETHODCALLTYPE *GetProcessActivity )( 
            IMFSensorActivityReport * This,
            /* [annotation][in] */ 
            _In_  ULONG Index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorProcessActivity **ppProcessActivity);
        
        END_INTERFACE
    } IMFSensorActivityReportVtbl;

    interface IMFSensorActivityReport
    {
        CONST_VTBL struct IMFSensorActivityReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorActivityReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorActivityReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorActivityReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorActivityReport_GetFriendlyName(This,FriendlyName,cchFriendlyName,pcchWritten)	\
    ( (This)->lpVtbl -> GetFriendlyName(This,FriendlyName,cchFriendlyName,pcchWritten) ) 

#define IMFSensorActivityReport_GetSymbolicLink(This,SymbolicLink,cchSymbolicLink,pcchWritten)	\
    ( (This)->lpVtbl -> GetSymbolicLink(This,SymbolicLink,cchSymbolicLink,pcchWritten) ) 

#define IMFSensorActivityReport_GetProcessCount(This,pcCount)	\
    ( (This)->lpVtbl -> GetProcessCount(This,pcCount) ) 

#define IMFSensorActivityReport_GetProcessActivity(This,Index,ppProcessActivity)	\
    ( (This)->lpVtbl -> GetProcessActivity(This,Index,ppProcessActivity) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorActivityReport_INTERFACE_DEFINED__ */


#ifndef __IMFSensorActivitiesReport_INTERFACE_DEFINED__
#define __IMFSensorActivitiesReport_INTERFACE_DEFINED__

/* interface IMFSensorActivitiesReport */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorActivitiesReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("683F7A5E-4A19-43CD-B1A9-DBF4AB3F7777")
    IMFSensorActivitiesReport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCount( 
            /* [annotation][out] */ 
            _Out_  ULONG *pcCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityReport( 
            /* [annotation][in] */ 
            _In_  ULONG Index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorActivityReport **sensorActivityReport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActivityReportByDeviceName( 
            /* [annotation][in] */ 
            _In_z_  LPCWSTR SymbolicName,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorActivityReport **sensorActivityReport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorActivitiesReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorActivitiesReport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorActivitiesReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorActivitiesReport * This);
        
        DECLSPEC_XFGVIRT(IMFSensorActivitiesReport, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFSensorActivitiesReport * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pcCount);
        
        DECLSPEC_XFGVIRT(IMFSensorActivitiesReport, GetActivityReport)
        HRESULT ( STDMETHODCALLTYPE *GetActivityReport )( 
            IMFSensorActivitiesReport * This,
            /* [annotation][in] */ 
            _In_  ULONG Index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorActivityReport **sensorActivityReport);
        
        DECLSPEC_XFGVIRT(IMFSensorActivitiesReport, GetActivityReportByDeviceName)
        HRESULT ( STDMETHODCALLTYPE *GetActivityReportByDeviceName )( 
            IMFSensorActivitiesReport * This,
            /* [annotation][in] */ 
            _In_z_  LPCWSTR SymbolicName,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFSensorActivityReport **sensorActivityReport);
        
        END_INTERFACE
    } IMFSensorActivitiesReportVtbl;

    interface IMFSensorActivitiesReport
    {
        CONST_VTBL struct IMFSensorActivitiesReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorActivitiesReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorActivitiesReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorActivitiesReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorActivitiesReport_GetCount(This,pcCount)	\
    ( (This)->lpVtbl -> GetCount(This,pcCount) ) 

#define IMFSensorActivitiesReport_GetActivityReport(This,Index,sensorActivityReport)	\
    ( (This)->lpVtbl -> GetActivityReport(This,Index,sensorActivityReport) ) 

#define IMFSensorActivitiesReport_GetActivityReportByDeviceName(This,SymbolicName,sensorActivityReport)	\
    ( (This)->lpVtbl -> GetActivityReportByDeviceName(This,SymbolicName,sensorActivityReport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorActivitiesReport_INTERFACE_DEFINED__ */


#ifndef __IMFSensorActivitiesReportCallback_INTERFACE_DEFINED__
#define __IMFSensorActivitiesReportCallback_INTERFACE_DEFINED__

/* interface IMFSensorActivitiesReportCallback */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorActivitiesReportCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DE5072EE-DBE3-46DC-8A87-B6F631194751")
    IMFSensorActivitiesReportCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnActivitiesReport( 
            /* [annotation][in] */ 
            _In_  IMFSensorActivitiesReport *sensorActivitiesReport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorActivitiesReportCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorActivitiesReportCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorActivitiesReportCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorActivitiesReportCallback * This);
        
        DECLSPEC_XFGVIRT(IMFSensorActivitiesReportCallback, OnActivitiesReport)
        HRESULT ( STDMETHODCALLTYPE *OnActivitiesReport )( 
            IMFSensorActivitiesReportCallback * This,
            /* [annotation][in] */ 
            _In_  IMFSensorActivitiesReport *sensorActivitiesReport);
        
        END_INTERFACE
    } IMFSensorActivitiesReportCallbackVtbl;

    interface IMFSensorActivitiesReportCallback
    {
        CONST_VTBL struct IMFSensorActivitiesReportCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorActivitiesReportCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorActivitiesReportCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorActivitiesReportCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorActivitiesReportCallback_OnActivitiesReport(This,sensorActivitiesReport)	\
    ( (This)->lpVtbl -> OnActivitiesReport(This,sensorActivitiesReport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorActivitiesReportCallback_INTERFACE_DEFINED__ */


#ifndef __IMFSensorActivityMonitor_INTERFACE_DEFINED__
#define __IMFSensorActivityMonitor_INTERFACE_DEFINED__

/* interface IMFSensorActivityMonitor */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSensorActivityMonitor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D0CEF145-B3F4-4340-A2E5-7A5080CA05CB")
    IMFSensorActivityMonitor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSensorActivityMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSensorActivityMonitor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSensorActivityMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSensorActivityMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFSensorActivityMonitor, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IMFSensorActivityMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFSensorActivityMonitor, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMFSensorActivityMonitor * This);
        
        END_INTERFACE
    } IMFSensorActivityMonitorVtbl;

    interface IMFSensorActivityMonitor
    {
        CONST_VTBL struct IMFSensorActivityMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSensorActivityMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSensorActivityMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSensorActivityMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSensorActivityMonitor_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IMFSensorActivityMonitor_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSensorActivityMonitor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0121 */
/* [local] */ 


STDAPI
MFCreateSensorActivityMonitor(
    _In_ IMFSensorActivitiesReportCallback* pCallback,
    _COM_Outptr_ IMFSensorActivityMonitor** ppActivityMonitor
    );
#endif // (WINVER >= _WIN32_WINNT_WINTHRESHOLD) 
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef struct _MFCameraIntrinsic_CameraModel
    {
    FLOAT FocalLength_x;
    FLOAT FocalLength_y;
    FLOAT PrincipalPoint_x;
    FLOAT PrincipalPoint_y;
    } 	MFCameraIntrinsic_CameraModel;

typedef struct _MFCameraIntrinsic_DistortionModel6KT
    {
    FLOAT Radial_k1;
    FLOAT Radial_k2;
    FLOAT Radial_k3;
    FLOAT Radial_k4;
    FLOAT Radial_k5;
    FLOAT Radial_k6;
    FLOAT Tangential_p1;
    FLOAT Tangential_p2;
    } 	MFCameraIntrinsic_DistortionModel6KT;

typedef struct _MFCameraIntrinsic_DistortionModelArcTan
    {
    FLOAT Radial_k0;
    FLOAT DistortionCenter_x;
    FLOAT DistortionCenter_y;
    FLOAT Tangential_x;
    FLOAT Tangential_y;
    } 	MFCameraIntrinsic_DistortionModelArcTan;

typedef 
enum _MFCameraIntrinsic_DistortionModelType
    {
        MFCameraIntrinsic_DistortionModelType_6KT	= 0,
        MFCameraIntrinsic_DistortionModelType_ArcTan	= ( MFCameraIntrinsic_DistortionModelType_6KT + 1 ) 
    } 	MFCameraIntrinsic_DistortionModelType;

typedef struct _MFExtendedCameraIntrinsic_IntrinsicModel
    {
    UINT32 Width;
    UINT32 Height;
    UINT32 SplitFrameId;
    MFCameraIntrinsic_CameraModel CameraModel;
    } 	MFExtendedCameraIntrinsic_IntrinsicModel;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0121_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0121_v0_0_s_ifspec;

#ifndef __IMFExtendedCameraIntrinsicModel_INTERFACE_DEFINED__
#define __IMFExtendedCameraIntrinsicModel_INTERFACE_DEFINED__

/* interface IMFExtendedCameraIntrinsicModel */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedCameraIntrinsicModel;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5C595E64-4630-4231-855A-12842F733245")
    IMFExtendedCameraIntrinsicModel : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetModel( 
            /* [annotation][out] */ 
            _Out_  MFExtendedCameraIntrinsic_IntrinsicModel *pIntrinsicModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetModel( 
            /* [annotation][in] */ 
            _In_  const MFExtendedCameraIntrinsic_IntrinsicModel *pIntrinsicModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDistortionModelType( 
            /* [annotation][out] */ 
            _Out_  MFCameraIntrinsic_DistortionModelType *pDistortionModelType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedCameraIntrinsicModelVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedCameraIntrinsicModel * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedCameraIntrinsicModel * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedCameraIntrinsicModel * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicModel, GetModel)
        HRESULT ( STDMETHODCALLTYPE *GetModel )( 
            IMFExtendedCameraIntrinsicModel * This,
            /* [annotation][out] */ 
            _Out_  MFExtendedCameraIntrinsic_IntrinsicModel *pIntrinsicModel);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicModel, SetModel)
        HRESULT ( STDMETHODCALLTYPE *SetModel )( 
            IMFExtendedCameraIntrinsicModel * This,
            /* [annotation][in] */ 
            _In_  const MFExtendedCameraIntrinsic_IntrinsicModel *pIntrinsicModel);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicModel, GetDistortionModelType)
        HRESULT ( STDMETHODCALLTYPE *GetDistortionModelType )( 
            IMFExtendedCameraIntrinsicModel * This,
            /* [annotation][out] */ 
            _Out_  MFCameraIntrinsic_DistortionModelType *pDistortionModelType);
        
        END_INTERFACE
    } IMFExtendedCameraIntrinsicModelVtbl;

    interface IMFExtendedCameraIntrinsicModel
    {
        CONST_VTBL struct IMFExtendedCameraIntrinsicModelVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedCameraIntrinsicModel_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedCameraIntrinsicModel_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedCameraIntrinsicModel_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedCameraIntrinsicModel_GetModel(This,pIntrinsicModel)	\
    ( (This)->lpVtbl -> GetModel(This,pIntrinsicModel) ) 

#define IMFExtendedCameraIntrinsicModel_SetModel(This,pIntrinsicModel)	\
    ( (This)->lpVtbl -> SetModel(This,pIntrinsicModel) ) 

#define IMFExtendedCameraIntrinsicModel_GetDistortionModelType(This,pDistortionModelType)	\
    ( (This)->lpVtbl -> GetDistortionModelType(This,pDistortionModelType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedCameraIntrinsicModel_INTERFACE_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsicsDistortionModel6KT_INTERFACE_DEFINED__
#define __IMFExtendedCameraIntrinsicsDistortionModel6KT_INTERFACE_DEFINED__

/* interface IMFExtendedCameraIntrinsicsDistortionModel6KT */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedCameraIntrinsicsDistortionModel6KT;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("74C2653B-5F55-4EB1-9F0F-18B8F68B7D3D")
    IMFExtendedCameraIntrinsicsDistortionModel6KT : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDistortionModel( 
            /* [annotation][out] */ 
            _Out_  MFCameraIntrinsic_DistortionModel6KT *pDistortionModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDistortionModel( 
            /* [annotation][in] */ 
            _In_  const MFCameraIntrinsic_DistortionModel6KT *pDistortionModel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedCameraIntrinsicsDistortionModel6KTVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedCameraIntrinsicsDistortionModel6KT * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedCameraIntrinsicsDistortionModel6KT * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedCameraIntrinsicsDistortionModel6KT * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicsDistortionModel6KT, GetDistortionModel)
        HRESULT ( STDMETHODCALLTYPE *GetDistortionModel )( 
            IMFExtendedCameraIntrinsicsDistortionModel6KT * This,
            /* [annotation][out] */ 
            _Out_  MFCameraIntrinsic_DistortionModel6KT *pDistortionModel);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicsDistortionModel6KT, SetDistortionModel)
        HRESULT ( STDMETHODCALLTYPE *SetDistortionModel )( 
            IMFExtendedCameraIntrinsicsDistortionModel6KT * This,
            /* [annotation][in] */ 
            _In_  const MFCameraIntrinsic_DistortionModel6KT *pDistortionModel);
        
        END_INTERFACE
    } IMFExtendedCameraIntrinsicsDistortionModel6KTVtbl;

    interface IMFExtendedCameraIntrinsicsDistortionModel6KT
    {
        CONST_VTBL struct IMFExtendedCameraIntrinsicsDistortionModel6KTVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedCameraIntrinsicsDistortionModel6KT_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedCameraIntrinsicsDistortionModel6KT_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedCameraIntrinsicsDistortionModel6KT_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedCameraIntrinsicsDistortionModel6KT_GetDistortionModel(This,pDistortionModel)	\
    ( (This)->lpVtbl -> GetDistortionModel(This,pDistortionModel) ) 

#define IMFExtendedCameraIntrinsicsDistortionModel6KT_SetDistortionModel(This,pDistortionModel)	\
    ( (This)->lpVtbl -> SetDistortionModel(This,pDistortionModel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedCameraIntrinsicsDistortionModel6KT_INTERFACE_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsicsDistortionModelArcTan_INTERFACE_DEFINED__
#define __IMFExtendedCameraIntrinsicsDistortionModelArcTan_INTERFACE_DEFINED__

/* interface IMFExtendedCameraIntrinsicsDistortionModelArcTan */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedCameraIntrinsicsDistortionModelArcTan;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("812D5F95-B572-45DC-BAFC-AE24199DDDA8")
    IMFExtendedCameraIntrinsicsDistortionModelArcTan : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDistortionModel( 
            /* [annotation][out] */ 
            _Out_  MFCameraIntrinsic_DistortionModelArcTan *pDistortionModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDistortionModel( 
            /* [annotation][in] */ 
            _In_  const MFCameraIntrinsic_DistortionModelArcTan *pDistortionModel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedCameraIntrinsicsDistortionModelArcTanVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedCameraIntrinsicsDistortionModelArcTan * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedCameraIntrinsicsDistortionModelArcTan * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedCameraIntrinsicsDistortionModelArcTan * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicsDistortionModelArcTan, GetDistortionModel)
        HRESULT ( STDMETHODCALLTYPE *GetDistortionModel )( 
            IMFExtendedCameraIntrinsicsDistortionModelArcTan * This,
            /* [annotation][out] */ 
            _Out_  MFCameraIntrinsic_DistortionModelArcTan *pDistortionModel);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsicsDistortionModelArcTan, SetDistortionModel)
        HRESULT ( STDMETHODCALLTYPE *SetDistortionModel )( 
            IMFExtendedCameraIntrinsicsDistortionModelArcTan * This,
            /* [annotation][in] */ 
            _In_  const MFCameraIntrinsic_DistortionModelArcTan *pDistortionModel);
        
        END_INTERFACE
    } IMFExtendedCameraIntrinsicsDistortionModelArcTanVtbl;

    interface IMFExtendedCameraIntrinsicsDistortionModelArcTan
    {
        CONST_VTBL struct IMFExtendedCameraIntrinsicsDistortionModelArcTanVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedCameraIntrinsicsDistortionModelArcTan_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedCameraIntrinsicsDistortionModelArcTan_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedCameraIntrinsicsDistortionModelArcTan_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedCameraIntrinsicsDistortionModelArcTan_GetDistortionModel(This,pDistortionModel)	\
    ( (This)->lpVtbl -> GetDistortionModel(This,pDistortionModel) ) 

#define IMFExtendedCameraIntrinsicsDistortionModelArcTan_SetDistortionModel(This,pDistortionModel)	\
    ( (This)->lpVtbl -> SetDistortionModel(This,pDistortionModel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedCameraIntrinsicsDistortionModelArcTan_INTERFACE_DEFINED__ */


#ifndef __IMFExtendedCameraIntrinsics_INTERFACE_DEFINED__
#define __IMFExtendedCameraIntrinsics_INTERFACE_DEFINED__

/* interface IMFExtendedCameraIntrinsics */
/* [local][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedCameraIntrinsics;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("687F6DAC-6987-4750-A16A-734D1E7A10FE")
    IMFExtendedCameraIntrinsics : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeFromBuffer( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(dwBufferSize)  BYTE *pbBuffer,
            /* [in] */ DWORD dwBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBufferSize( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SerializeToBuffer( 
            /* [annotation][out] */ 
            _Out_writes_bytes_to_(*pdwBufferSize, *pdwBufferSize)  BYTE *pbBuffer,
            /* [annotation][out] */ 
            _Inout_  DWORD *pdwBufferSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIntrinsicModelCount( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetIntrinsicModelByIndex( 
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFExtendedCameraIntrinsicModel **ppIntrinsicModel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddIntrinsicModel( 
            /* [annotation][in] */ 
            _In_  IMFExtendedCameraIntrinsicModel *pIntrinsicModel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedCameraIntrinsicsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedCameraIntrinsics * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedCameraIntrinsics * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedCameraIntrinsics * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsics, InitializeFromBuffer)
        HRESULT ( STDMETHODCALLTYPE *InitializeFromBuffer )( 
            IMFExtendedCameraIntrinsics * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(dwBufferSize)  BYTE *pbBuffer,
            /* [in] */ DWORD dwBufferSize);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsics, GetBufferSize)
        HRESULT ( STDMETHODCALLTYPE *GetBufferSize )( 
            IMFExtendedCameraIntrinsics * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwBufferSize);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsics, SerializeToBuffer)
        HRESULT ( STDMETHODCALLTYPE *SerializeToBuffer )( 
            IMFExtendedCameraIntrinsics * This,
            /* [annotation][out] */ 
            _Out_writes_bytes_to_(*pdwBufferSize, *pdwBufferSize)  BYTE *pbBuffer,
            /* [annotation][out] */ 
            _Inout_  DWORD *pdwBufferSize);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsics, GetIntrinsicModelCount)
        HRESULT ( STDMETHODCALLTYPE *GetIntrinsicModelCount )( 
            IMFExtendedCameraIntrinsics * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwCount);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsics, GetIntrinsicModelByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetIntrinsicModelByIndex )( 
            IMFExtendedCameraIntrinsics * This,
            /* [in] */ DWORD dwIndex,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFExtendedCameraIntrinsicModel **ppIntrinsicModel);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraIntrinsics, AddIntrinsicModel)
        HRESULT ( STDMETHODCALLTYPE *AddIntrinsicModel )( 
            IMFExtendedCameraIntrinsics * This,
            /* [annotation][in] */ 
            _In_  IMFExtendedCameraIntrinsicModel *pIntrinsicModel);
        
        END_INTERFACE
    } IMFExtendedCameraIntrinsicsVtbl;

    interface IMFExtendedCameraIntrinsics
    {
        CONST_VTBL struct IMFExtendedCameraIntrinsicsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedCameraIntrinsics_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedCameraIntrinsics_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedCameraIntrinsics_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedCameraIntrinsics_InitializeFromBuffer(This,pbBuffer,dwBufferSize)	\
    ( (This)->lpVtbl -> InitializeFromBuffer(This,pbBuffer,dwBufferSize) ) 

#define IMFExtendedCameraIntrinsics_GetBufferSize(This,pdwBufferSize)	\
    ( (This)->lpVtbl -> GetBufferSize(This,pdwBufferSize) ) 

#define IMFExtendedCameraIntrinsics_SerializeToBuffer(This,pbBuffer,pdwBufferSize)	\
    ( (This)->lpVtbl -> SerializeToBuffer(This,pbBuffer,pdwBufferSize) ) 

#define IMFExtendedCameraIntrinsics_GetIntrinsicModelCount(This,pdwCount)	\
    ( (This)->lpVtbl -> GetIntrinsicModelCount(This,pdwCount) ) 

#define IMFExtendedCameraIntrinsics_GetIntrinsicModelByIndex(This,dwIndex,ppIntrinsicModel)	\
    ( (This)->lpVtbl -> GetIntrinsicModelByIndex(This,dwIndex,ppIntrinsicModel) ) 

#define IMFExtendedCameraIntrinsics_AddIntrinsicModel(This,pIntrinsicModel)	\
    ( (This)->lpVtbl -> AddIntrinsicModel(This,pIntrinsicModel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedCameraIntrinsics_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0125 */
/* [local] */ 

DEFINE_GUID(MFStreamExtension_ExtendedCameraIntrinsics,
    0xaa74b3df, 0x9a2c, 0x48d6, 0x83, 0x93, 0x5b, 0xd1, 0xc1, 0xa8, 0x1e, 0x6e);
DEFINE_GUID(MFSampleExtension_ExtendedCameraIntrinsics,
    0x560bc4a5, 0x4de0, 0x4113, 0x9c, 0xdc, 0x83, 0x2d, 0xb9, 0x74, 0xf, 0x3d);
#if (NTDDI_VERSION >= NTDDI_WIN10_RS5) 
STDAPI
MFCreateExtendedCameraIntrinsics(
    _COM_Outptr_ IMFExtendedCameraIntrinsics** ppExtendedCameraIntrinsics
    );
STDAPI
MFCreateExtendedCameraIntrinsicModel(
    const MFCameraIntrinsic_DistortionModelType distortionModelType,
    _COM_Outptr_ IMFExtendedCameraIntrinsicModel** ppExtendedCameraIntrinsicModel
    );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_RS5) 
#if (NTDDI_VERSION >= NTDDI_WIN10_VB) 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0125_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0125_v0_0_s_ifspec;

#ifndef __IMFExtendedCameraControl_INTERFACE_DEFINED__
#define __IMFExtendedCameraControl_INTERFACE_DEFINED__

/* interface IMFExtendedCameraControl */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedCameraControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("38E33520-FCA1-4845-A27A-68B7C6AB3789")
    IMFExtendedCameraControl : public IUnknown
    {
    public:
        virtual ULONGLONG STDMETHODCALLTYPE GetCapabilities( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFlags( 
            /* [annotation][in] */ 
            _In_  ULONGLONG ulFlags) = 0;
        
        virtual ULONGLONG STDMETHODCALLTYPE GetFlags( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockPayload( 
            /* [annotation][out] */ 
            _Outptr_result_buffer_(*pulPayload)  BYTE **ppPayload,
            /* [annotation][out] */ 
            _Out_  ULONG *pulPayload) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockPayload( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CommitSettings( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedCameraControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedCameraControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedCameraControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedCameraControl * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraControl, GetCapabilities)
        ULONGLONG ( STDMETHODCALLTYPE *GetCapabilities )( 
            IMFExtendedCameraControl * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraControl, SetFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFlags )( 
            IMFExtendedCameraControl * This,
            /* [annotation][in] */ 
            _In_  ULONGLONG ulFlags);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraControl, GetFlags)
        ULONGLONG ( STDMETHODCALLTYPE *GetFlags )( 
            IMFExtendedCameraControl * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraControl, LockPayload)
        HRESULT ( STDMETHODCALLTYPE *LockPayload )( 
            IMFExtendedCameraControl * This,
            /* [annotation][out] */ 
            _Outptr_result_buffer_(*pulPayload)  BYTE **ppPayload,
            /* [annotation][out] */ 
            _Out_  ULONG *pulPayload);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraControl, UnlockPayload)
        HRESULT ( STDMETHODCALLTYPE *UnlockPayload )( 
            IMFExtendedCameraControl * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraControl, CommitSettings)
        HRESULT ( STDMETHODCALLTYPE *CommitSettings )( 
            IMFExtendedCameraControl * This);
        
        END_INTERFACE
    } IMFExtendedCameraControlVtbl;

    interface IMFExtendedCameraControl
    {
        CONST_VTBL struct IMFExtendedCameraControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedCameraControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedCameraControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedCameraControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedCameraControl_GetCapabilities(This)	\
    ( (This)->lpVtbl -> GetCapabilities(This) ) 

#define IMFExtendedCameraControl_SetFlags(This,ulFlags)	\
    ( (This)->lpVtbl -> SetFlags(This,ulFlags) ) 

#define IMFExtendedCameraControl_GetFlags(This)	\
    ( (This)->lpVtbl -> GetFlags(This) ) 

#define IMFExtendedCameraControl_LockPayload(This,ppPayload,pulPayload)	\
    ( (This)->lpVtbl -> LockPayload(This,ppPayload,pulPayload) ) 

#define IMFExtendedCameraControl_UnlockPayload(This)	\
    ( (This)->lpVtbl -> UnlockPayload(This) ) 

#define IMFExtendedCameraControl_CommitSettings(This)	\
    ( (This)->lpVtbl -> CommitSettings(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedCameraControl_INTERFACE_DEFINED__ */


#ifndef __IMFExtendedCameraController_INTERFACE_DEFINED__
#define __IMFExtendedCameraController_INTERFACE_DEFINED__

/* interface IMFExtendedCameraController */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFExtendedCameraController;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B91EBFEE-CA03-4AF4-8A82-A31752F4A0FC")
    IMFExtendedCameraController : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetExtendedCameraControl( 
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  ULONG ulPropertyId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFExtendedCameraControl **ppControl) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFExtendedCameraControllerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFExtendedCameraController * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFExtendedCameraController * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFExtendedCameraController * This);
        
        DECLSPEC_XFGVIRT(IMFExtendedCameraController, GetExtendedCameraControl)
        HRESULT ( STDMETHODCALLTYPE *GetExtendedCameraControl )( 
            IMFExtendedCameraController * This,
            /* [annotation][in] */ 
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */ 
            _In_  ULONG ulPropertyId,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFExtendedCameraControl **ppControl);
        
        END_INTERFACE
    } IMFExtendedCameraControllerVtbl;

    interface IMFExtendedCameraController
    {
        CONST_VTBL struct IMFExtendedCameraControllerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFExtendedCameraController_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFExtendedCameraController_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFExtendedCameraController_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFExtendedCameraController_GetExtendedCameraControl(This,dwStreamIndex,ulPropertyId,ppControl)	\
    ( (This)->lpVtbl -> GetExtendedCameraControl(This,dwStreamIndex,ulPropertyId,ppControl) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFExtendedCameraController_INTERFACE_DEFINED__ */


#ifndef __IMFRelativePanelReport_INTERFACE_DEFINED__
#define __IMFRelativePanelReport_INTERFACE_DEFINED__

/* interface IMFRelativePanelReport */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFRelativePanelReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F25362EA-2C0E-447F-81E2-755914CDC0C3")
    IMFRelativePanelReport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRelativePanel( 
            /* [annotation][out] */ 
            _Out_  ULONG *panel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRelativePanelReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFRelativePanelReport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFRelativePanelReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFRelativePanelReport * This);
        
        DECLSPEC_XFGVIRT(IMFRelativePanelReport, GetRelativePanel)
        HRESULT ( STDMETHODCALLTYPE *GetRelativePanel )( 
            IMFRelativePanelReport * This,
            /* [annotation][out] */ 
            _Out_  ULONG *panel);
        
        END_INTERFACE
    } IMFRelativePanelReportVtbl;

    interface IMFRelativePanelReport
    {
        CONST_VTBL struct IMFRelativePanelReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRelativePanelReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRelativePanelReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRelativePanelReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRelativePanelReport_GetRelativePanel(This,panel)	\
    ( (This)->lpVtbl -> GetRelativePanel(This,panel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRelativePanelReport_INTERFACE_DEFINED__ */


#ifndef __IMFRelativePanelWatcher_INTERFACE_DEFINED__
#define __IMFRelativePanelWatcher_INTERFACE_DEFINED__

/* interface IMFRelativePanelWatcher */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFRelativePanelWatcher;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("421AF7F6-573E-4AD0-8FDA-2E57CEDB18C6")
    IMFRelativePanelWatcher : public IMFShutdown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE BeginGetReport( 
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndGetReport( 
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFRelativePanelReport **ppRelativePanelReport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetReport( 
            /* [annotation][out] */ 
            _COM_Outptr_  IMFRelativePanelReport **ppRelativePanelReport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFRelativePanelWatcherVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFRelativePanelWatcher * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFRelativePanelWatcher * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFRelativePanelWatcher * This);
        
        DECLSPEC_XFGVIRT(IMFShutdown, Shutdown)
        HRESULT ( STDMETHODCALLTYPE *Shutdown )( 
            IMFRelativePanelWatcher * This);
        
        DECLSPEC_XFGVIRT(IMFShutdown, GetShutdownStatus)
        HRESULT ( STDMETHODCALLTYPE *GetShutdownStatus )( 
            IMFRelativePanelWatcher * This,
            /* [out] */ MFSHUTDOWN_STATUS *pStatus);
        
        DECLSPEC_XFGVIRT(IMFRelativePanelWatcher, BeginGetReport)
        HRESULT ( STDMETHODCALLTYPE *BeginGetReport )( 
            IMFRelativePanelWatcher * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncCallback *pCallback,
            /* [annotation][in] */ 
            _In_opt_  IUnknown *pState);
        
        DECLSPEC_XFGVIRT(IMFRelativePanelWatcher, EndGetReport)
        HRESULT ( STDMETHODCALLTYPE *EndGetReport )( 
            IMFRelativePanelWatcher * This,
            /* [annotation][in] */ 
            _In_  IMFAsyncResult *pResult,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFRelativePanelReport **ppRelativePanelReport);
        
        DECLSPEC_XFGVIRT(IMFRelativePanelWatcher, GetReport)
        HRESULT ( STDMETHODCALLTYPE *GetReport )( 
            IMFRelativePanelWatcher * This,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFRelativePanelReport **ppRelativePanelReport);
        
        END_INTERFACE
    } IMFRelativePanelWatcherVtbl;

    interface IMFRelativePanelWatcher
    {
        CONST_VTBL struct IMFRelativePanelWatcherVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFRelativePanelWatcher_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFRelativePanelWatcher_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFRelativePanelWatcher_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFRelativePanelWatcher_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#define IMFRelativePanelWatcher_GetShutdownStatus(This,pStatus)	\
    ( (This)->lpVtbl -> GetShutdownStatus(This,pStatus) ) 


#define IMFRelativePanelWatcher_BeginGetReport(This,pCallback,pState)	\
    ( (This)->lpVtbl -> BeginGetReport(This,pCallback,pState) ) 

#define IMFRelativePanelWatcher_EndGetReport(This,pResult,ppRelativePanelReport)	\
    ( (This)->lpVtbl -> EndGetReport(This,pResult,ppRelativePanelReport) ) 

#define IMFRelativePanelWatcher_GetReport(This,ppRelativePanelReport)	\
    ( (This)->lpVtbl -> GetReport(This,ppRelativePanelReport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFRelativePanelWatcher_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0129 */
/* [local] */ 


STDAPI
MFCreateRelativePanelWatcher(
    _In_ PCWSTR videoDeviceId,
    _In_ PCWSTR displayMonitorDeviceId,
    _COM_Outptr_ IMFRelativePanelWatcher** ppRelativePanelWatcher
    );


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0129_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0129_v0_0_s_ifspec;

#ifndef __IMFVideoCaptureSampleAllocator_INTERFACE_DEFINED__
#define __IMFVideoCaptureSampleAllocator_INTERFACE_DEFINED__

/* interface IMFVideoCaptureSampleAllocator */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IMFVideoCaptureSampleAllocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("725B77C7-CA9F-4FE5-9D72-9946BF9B3C70")
    IMFVideoCaptureSampleAllocator : public IMFVideoSampleAllocator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitializeCaptureSampleAllocator( 
            /* [annotation][in] */ 
            _In_  DWORD cbSampleSize,
            /* [annotation][in] */ 
            _In_  DWORD cbCaptureMetadataSize,
            /* [annotation][in] */ 
            _In_  DWORD cbAlignment,
            /* [annotation][in] */ 
            _In_  DWORD cMinimumSamples,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFVideoCaptureSampleAllocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFVideoCaptureSampleAllocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFVideoCaptureSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFVideoCaptureSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, SetDirectXManager)
        HRESULT ( STDMETHODCALLTYPE *SetDirectXManager )( 
            IMFVideoCaptureSampleAllocator * This,
            /* [unique][in] */ IUnknown *pManager);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, UninitializeSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *UninitializeSampleAllocator )( 
            IMFVideoCaptureSampleAllocator * This);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, InitializeSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *InitializeSampleAllocator )( 
            IMFVideoCaptureSampleAllocator * This,
            /* [in] */ DWORD cRequestedFrames,
            /* [in] */ IMFMediaType *pMediaType);
        
        DECLSPEC_XFGVIRT(IMFVideoSampleAllocator, AllocateSample)
        HRESULT ( STDMETHODCALLTYPE *AllocateSample )( 
            IMFVideoCaptureSampleAllocator * This,
            /* [out] */ IMFSample **ppSample);
        
        DECLSPEC_XFGVIRT(IMFVideoCaptureSampleAllocator, InitializeCaptureSampleAllocator)
        HRESULT ( STDMETHODCALLTYPE *InitializeCaptureSampleAllocator )( 
            IMFVideoCaptureSampleAllocator * This,
            /* [annotation][in] */ 
            _In_  DWORD cbSampleSize,
            /* [annotation][in] */ 
            _In_  DWORD cbCaptureMetadataSize,
            /* [annotation][in] */ 
            _In_  DWORD cbAlignment,
            /* [annotation][in] */ 
            _In_  DWORD cMinimumSamples,
            /* [annotation][in] */ 
            _In_opt_  IMFAttributes *pAttributes,
            /* [annotation][in] */ 
            _In_  IMFMediaType *pMediaType);
        
        END_INTERFACE
    } IMFVideoCaptureSampleAllocatorVtbl;

    interface IMFVideoCaptureSampleAllocator
    {
        CONST_VTBL struct IMFVideoCaptureSampleAllocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFVideoCaptureSampleAllocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFVideoCaptureSampleAllocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFVideoCaptureSampleAllocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFVideoCaptureSampleAllocator_SetDirectXManager(This,pManager)	\
    ( (This)->lpVtbl -> SetDirectXManager(This,pManager) ) 

#define IMFVideoCaptureSampleAllocator_UninitializeSampleAllocator(This)	\
    ( (This)->lpVtbl -> UninitializeSampleAllocator(This) ) 

#define IMFVideoCaptureSampleAllocator_InitializeSampleAllocator(This,cRequestedFrames,pMediaType)	\
    ( (This)->lpVtbl -> InitializeSampleAllocator(This,cRequestedFrames,pMediaType) ) 

#define IMFVideoCaptureSampleAllocator_AllocateSample(This,ppSample)	\
    ( (This)->lpVtbl -> AllocateSample(This,ppSample) ) 


#define IMFVideoCaptureSampleAllocator_InitializeCaptureSampleAllocator(This,cbSampleSize,cbCaptureMetadataSize,cbAlignment,cMinimumSamples,pAttributes,pMediaType)	\
    ( (This)->lpVtbl -> InitializeCaptureSampleAllocator(This,cbSampleSize,cbCaptureMetadataSize,cbAlignment,cMinimumSamples,pAttributes,pMediaType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFVideoCaptureSampleAllocator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0130 */
/* [local] */ 

typedef 
enum MFSampleAllocatorUsage
    {
        MFSampleAllocatorUsage_UsesProvidedAllocator	= 0,
        MFSampleAllocatorUsage_UsesCustomAllocator	= ( MFSampleAllocatorUsage_UsesProvidedAllocator + 1 ) ,
        MFSampleAllocatorUsage_DoesNotAllocate	= ( MFSampleAllocatorUsage_UsesCustomAllocator + 1 ) 
    } 	MFSampleAllocatorUsage;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0130_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0130_v0_0_s_ifspec;

#ifndef __IMFSampleAllocatorControl_INTERFACE_DEFINED__
#define __IMFSampleAllocatorControl_INTERFACE_DEFINED__

/* interface IMFSampleAllocatorControl */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFSampleAllocatorControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DA62B958-3A38-4A97-BD27-149C640C0771")
    IMFSampleAllocatorControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDefaultAllocator( 
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][in] */ 
            _In_  IUnknown *pAllocator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllocatorUsage( 
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwInputStreamID,
            /* [annotation][out] */ 
            _Out_  MFSampleAllocatorUsage *peUsage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFSampleAllocatorControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFSampleAllocatorControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFSampleAllocatorControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFSampleAllocatorControl * This);
        
        DECLSPEC_XFGVIRT(IMFSampleAllocatorControl, SetDefaultAllocator)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultAllocator )( 
            IMFSampleAllocatorControl * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][in] */ 
            _In_  IUnknown *pAllocator);
        
        DECLSPEC_XFGVIRT(IMFSampleAllocatorControl, GetAllocatorUsage)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatorUsage )( 
            IMFSampleAllocatorControl * This,
            /* [annotation][in] */ 
            _In_  DWORD dwOutputStreamID,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwInputStreamID,
            /* [annotation][out] */ 
            _Out_  MFSampleAllocatorUsage *peUsage);
        
        END_INTERFACE
    } IMFSampleAllocatorControlVtbl;

    interface IMFSampleAllocatorControl
    {
        CONST_VTBL struct IMFSampleAllocatorControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFSampleAllocatorControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFSampleAllocatorControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFSampleAllocatorControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFSampleAllocatorControl_SetDefaultAllocator(This,dwOutputStreamID,pAllocator)	\
    ( (This)->lpVtbl -> SetDefaultAllocator(This,dwOutputStreamID,pAllocator) ) 

#define IMFSampleAllocatorControl_GetAllocatorUsage(This,dwOutputStreamID,pdwInputStreamID,peUsage)	\
    ( (This)->lpVtbl -> GetAllocatorUsage(This,dwOutputStreamID,pdwInputStreamID,peUsage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFSampleAllocatorControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0131 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN10_VB) 
#if (NTDDI_VERSION >= NTDDI_WIN10_CO) 
typedef /* [v1_enum] */ 
enum MFCameraOcclusionState
    {
        MFCameraOcclusionState_Open	= 0,
        MFCameraOcclusionState_OccludedByLid	= 0x1,
        MFCameraOcclusionState_OccludedByCameraHardware	= 0x2
    } 	MFCameraOcclusionState;

DEFINE_ENUM_FLAG_OPERATORS(MFCameraOcclusionState)


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0131_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0131_v0_0_s_ifspec;

#ifndef __IMFCameraOcclusionStateReport_INTERFACE_DEFINED__
#define __IMFCameraOcclusionStateReport_INTERFACE_DEFINED__

/* interface IMFCameraOcclusionStateReport */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraOcclusionStateReport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1640B2CF-74DA-4462-A43B-B76D3BDC1434")
    IMFCameraOcclusionStateReport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetOcclusionState( 
            /* [annotation][out] */ 
            _Out_   DWORD *occlusionState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraOcclusionStateReportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraOcclusionStateReport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraOcclusionStateReport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraOcclusionStateReport * This);
        
        DECLSPEC_XFGVIRT(IMFCameraOcclusionStateReport, GetOcclusionState)
        HRESULT ( STDMETHODCALLTYPE *GetOcclusionState )( 
            IMFCameraOcclusionStateReport * This,
            /* [annotation][out] */ 
            _Out_   DWORD *occlusionState);
        
        END_INTERFACE
    } IMFCameraOcclusionStateReportVtbl;

    interface IMFCameraOcclusionStateReport
    {
        CONST_VTBL struct IMFCameraOcclusionStateReportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraOcclusionStateReport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraOcclusionStateReport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraOcclusionStateReport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraOcclusionStateReport_GetOcclusionState(This,occlusionState)	\
    ( (This)->lpVtbl -> GetOcclusionState(This,occlusionState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraOcclusionStateReport_INTERFACE_DEFINED__ */


#ifndef __IMFCameraOcclusionStateReportCallback_INTERFACE_DEFINED__
#define __IMFCameraOcclusionStateReportCallback_INTERFACE_DEFINED__

/* interface IMFCameraOcclusionStateReportCallback */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraOcclusionStateReportCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6E5841C7-3889-4019-9035-783FB19B5948")
    IMFCameraOcclusionStateReportCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnOcclusionStateReport( 
            /* [annotation][in] */ 
            _In_  IMFCameraOcclusionStateReport *occlusionStateReport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraOcclusionStateReportCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraOcclusionStateReportCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraOcclusionStateReportCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraOcclusionStateReportCallback * This);
        
        DECLSPEC_XFGVIRT(IMFCameraOcclusionStateReportCallback, OnOcclusionStateReport)
        HRESULT ( STDMETHODCALLTYPE *OnOcclusionStateReport )( 
            IMFCameraOcclusionStateReportCallback * This,
            /* [annotation][in] */ 
            _In_  IMFCameraOcclusionStateReport *occlusionStateReport);
        
        END_INTERFACE
    } IMFCameraOcclusionStateReportCallbackVtbl;

    interface IMFCameraOcclusionStateReportCallback
    {
        CONST_VTBL struct IMFCameraOcclusionStateReportCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraOcclusionStateReportCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraOcclusionStateReportCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraOcclusionStateReportCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraOcclusionStateReportCallback_OnOcclusionStateReport(This,occlusionStateReport)	\
    ( (This)->lpVtbl -> OnOcclusionStateReport(This,occlusionStateReport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraOcclusionStateReportCallback_INTERFACE_DEFINED__ */


#ifndef __IMFCameraOcclusionStateMonitor_INTERFACE_DEFINED__
#define __IMFCameraOcclusionStateMonitor_INTERFACE_DEFINED__

/* interface IMFCameraOcclusionStateMonitor */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraOcclusionStateMonitor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CC692F46-C697-47E2-A72D-7B064617749B")
    IMFCameraOcclusionStateMonitor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual DWORD STDMETHODCALLTYPE GetSupportedStates( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraOcclusionStateMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraOcclusionStateMonitor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraOcclusionStateMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraOcclusionStateMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFCameraOcclusionStateMonitor, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IMFCameraOcclusionStateMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFCameraOcclusionStateMonitor, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMFCameraOcclusionStateMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFCameraOcclusionStateMonitor, GetSupportedStates)
        DWORD ( STDMETHODCALLTYPE *GetSupportedStates )( 
            IMFCameraOcclusionStateMonitor * This);
        
        END_INTERFACE
    } IMFCameraOcclusionStateMonitorVtbl;

    interface IMFCameraOcclusionStateMonitor
    {
        CONST_VTBL struct IMFCameraOcclusionStateMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraOcclusionStateMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraOcclusionStateMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraOcclusionStateMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraOcclusionStateMonitor_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IMFCameraOcclusionStateMonitor_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFCameraOcclusionStateMonitor_GetSupportedStates(This)	\
    ( (This)->lpVtbl -> GetSupportedStates(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraOcclusionStateMonitor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0134 */
/* [local] */ 


STDAPI
MFCreateCameraOcclusionStateMonitor(
    _In_z_ LPCWSTR symbolicLink,
    _In_ IMFCameraOcclusionStateReportCallback* callback,
    _COM_Outptr_ IMFCameraOcclusionStateMonitor** occlusionStateMonitor
    );
#endif // (NTDDI_VERSION >= NTDDI_WIN10_CO) 
#if (NTDDI_VERSION >= NTDDI_WIN10_NI) 
EXTERN_GUID(KSPROPERTYSETID_ANYCAMERACONTROL, 0x94dd0c30, 0x28c7, 0x4efb, 0x9d, 0x6b, 0x81, 0x23, 0x0, 0xfb, 0xc, 0x7f);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0134_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0134_v0_0_s_ifspec;

#ifndef __IMFCameraControlNotify_INTERFACE_DEFINED__
#define __IMFCameraControlNotify_INTERFACE_DEFINED__

/* interface IMFCameraControlNotify */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraControlNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("E8F2540D-558A-4449-8B64-4863467A9FE8")
    IMFCameraControlNotify : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnChange( 
            /* [annotation][in] */ 
            _In_  REFGUID controlSet,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual void STDMETHODCALLTYPE OnError( 
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraControlNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraControlNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraControlNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraControlNotify * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlNotify, OnChange)
        void ( STDMETHODCALLTYPE *OnChange )( 
            IMFCameraControlNotify * This,
            /* [annotation][in] */ 
            _In_  REFGUID controlSet,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IMFCameraControlNotify, OnError)
        void ( STDMETHODCALLTYPE *OnError )( 
            IMFCameraControlNotify * This,
            /* [annotation][in] */ 
            _In_  HRESULT hrStatus);
        
        END_INTERFACE
    } IMFCameraControlNotifyVtbl;

    interface IMFCameraControlNotify
    {
        CONST_VTBL struct IMFCameraControlNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraControlNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraControlNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraControlNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraControlNotify_OnChange(This,controlSet,id)	\
    ( (This)->lpVtbl -> OnChange(This,controlSet,id) ) 

#define IMFCameraControlNotify_OnError(This,hrStatus)	\
    ( (This)->lpVtbl -> OnError(This,hrStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraControlNotify_INTERFACE_DEFINED__ */


#ifndef __IMFCameraControlMonitor_INTERFACE_DEFINED__
#define __IMFCameraControlMonitor_INTERFACE_DEFINED__

/* interface IMFCameraControlMonitor */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraControlMonitor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4D46F2C9-28BA-4970-8C7B-1F0C9D80AF69")
    IMFCameraControlMonitor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Start( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddControlSubscription( 
            /* [annotation][in] */ 
            _In_  GUID controlSet,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveControlSubscription( 
            /* [annotation][in] */ 
            _In_  GUID controlSet,
            /* [annotation][in] */ 
            _In_  UINT32 id) = 0;
        
        virtual void STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraControlMonitorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraControlMonitor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraControlMonitor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraControlMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlMonitor, Start)
        HRESULT ( STDMETHODCALLTYPE *Start )( 
            IMFCameraControlMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlMonitor, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMFCameraControlMonitor * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlMonitor, AddControlSubscription)
        HRESULT ( STDMETHODCALLTYPE *AddControlSubscription )( 
            IMFCameraControlMonitor * This,
            /* [annotation][in] */ 
            _In_  GUID controlSet,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IMFCameraControlMonitor, RemoveControlSubscription)
        HRESULT ( STDMETHODCALLTYPE *RemoveControlSubscription )( 
            IMFCameraControlMonitor * This,
            /* [annotation][in] */ 
            _In_  GUID controlSet,
            /* [annotation][in] */ 
            _In_  UINT32 id);
        
        DECLSPEC_XFGVIRT(IMFCameraControlMonitor, Shutdown)
        void ( STDMETHODCALLTYPE *Shutdown )( 
            IMFCameraControlMonitor * This);
        
        END_INTERFACE
    } IMFCameraControlMonitorVtbl;

    interface IMFCameraControlMonitor
    {
        CONST_VTBL struct IMFCameraControlMonitorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraControlMonitor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraControlMonitor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraControlMonitor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraControlMonitor_Start(This)	\
    ( (This)->lpVtbl -> Start(This) ) 

#define IMFCameraControlMonitor_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMFCameraControlMonitor_AddControlSubscription(This,controlSet,id)	\
    ( (This)->lpVtbl -> AddControlSubscription(This,controlSet,id) ) 

#define IMFCameraControlMonitor_RemoveControlSubscription(This,controlSet,id)	\
    ( (This)->lpVtbl -> RemoveControlSubscription(This,controlSet,id) ) 

#define IMFCameraControlMonitor_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraControlMonitor_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0136 */
/* [local] */ 


STDAPI
MFCreateCameraControlMonitor(
    _In_z_ LPCWSTR symbolicLink,
    _In_ IMFCameraControlNotify* callback,
    _COM_Outptr_ IMFCameraControlMonitor ** ppCameraControlMonitor
    );
typedef /* [public][public][public][public] */ 
enum __MIDL___MIDL_itf_mfidl_0000_0136_0001
    {
        MF_CAMERA_CONTROL_CONFIGURATION_TYPE_PRESTART	= 0,
        MF_CAMERA_CONTROL_CONFIGURATION_TYPE_POSTSTART	= ( MF_CAMERA_CONTROL_CONFIGURATION_TYPE_PRESTART + 1 ) 
    } 	MF_CAMERA_CONTROL_CONFIGURATION_TYPE;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_mfidl_0000_0136_0002
    {
    LONG minValue;
    LONG maxValue;
    LONG stepValue;
    LONG defaultValue;
    } 	MF_CAMERA_CONTROL_RANGE_INFO;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0136_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0136_v0_0_s_ifspec;

#ifndef __IMFCameraControlDefaults_INTERFACE_DEFINED__
#define __IMFCameraControlDefaults_INTERFACE_DEFINED__

/* interface IMFCameraControlDefaults */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraControlDefaults;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("75510662-B034-48F4-88A7-8DE61DAA4AF9")
    IMFCameraControlDefaults : public IUnknown
    {
    public:
        virtual MF_CAMERA_CONTROL_CONFIGURATION_TYPE STDMETHODCALLTYPE GetType( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRangeInfo( 
            /* [annotation][out] */ 
            _Out_  MF_CAMERA_CONTROL_RANGE_INFO *rangeInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockControlData( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*controlSize)  void **control,
            /* [annotation][out] */ 
            _Out_  ULONG *controlSize,
            /* [annotation][out] */ 
            _Outptr_opt_result_bytebuffer_(*dataSize)  void **data,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *dataSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockControlData( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraControlDefaultsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraControlDefaults * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraControlDefaults * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraControlDefaults * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaults, GetType)
        MF_CAMERA_CONTROL_CONFIGURATION_TYPE ( STDMETHODCALLTYPE *GetType )( 
            IMFCameraControlDefaults * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaults, GetRangeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetRangeInfo )( 
            IMFCameraControlDefaults * This,
            /* [annotation][out] */ 
            _Out_  MF_CAMERA_CONTROL_RANGE_INFO *rangeInfo);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaults, LockControlData)
        HRESULT ( STDMETHODCALLTYPE *LockControlData )( 
            IMFCameraControlDefaults * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*controlSize)  void **control,
            /* [annotation][out] */ 
            _Out_  ULONG *controlSize,
            /* [annotation][out] */ 
            _Outptr_opt_result_bytebuffer_(*dataSize)  void **data,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *dataSize);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaults, UnlockControlData)
        HRESULT ( STDMETHODCALLTYPE *UnlockControlData )( 
            IMFCameraControlDefaults * This);
        
        END_INTERFACE
    } IMFCameraControlDefaultsVtbl;

    interface IMFCameraControlDefaults
    {
        CONST_VTBL struct IMFCameraControlDefaultsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraControlDefaults_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraControlDefaults_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraControlDefaults_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraControlDefaults_GetType(This)	\
    ( (This)->lpVtbl -> GetType(This) ) 

#define IMFCameraControlDefaults_GetRangeInfo(This,rangeInfo)	\
    ( (This)->lpVtbl -> GetRangeInfo(This,rangeInfo) ) 

#define IMFCameraControlDefaults_LockControlData(This,control,controlSize,data,dataSize)	\
    ( (This)->lpVtbl -> LockControlData(This,control,controlSize,data,dataSize) ) 

#define IMFCameraControlDefaults_UnlockControlData(This)	\
    ( (This)->lpVtbl -> UnlockControlData(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraControlDefaults_INTERFACE_DEFINED__ */


#ifndef __IMFCameraControlDefaultsCollection_INTERFACE_DEFINED__
#define __IMFCameraControlDefaultsCollection_INTERFACE_DEFINED__

/* interface IMFCameraControlDefaultsCollection */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraControlDefaultsCollection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92D43D0F-54A8-4BAE-96DA-356D259A5C26")
    IMFCameraControlDefaultsCollection : public IMFAttributes
    {
    public:
        virtual ULONG STDMETHODCALLTYPE GetControlCount( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetControl( 
            /* [annotation][in] */ 
            _In_  ULONG index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaults **configuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOrAddExtendedControl( 
            /* [annotation][in] */ 
            _In_  MF_CAMERA_CONTROL_CONFIGURATION_TYPE configType,
            /* [annotation][in] */ 
            _In_  ULONG constrolId,
            /* [annotation][in] */ 
            _In_  DWORD streamId,
            /* [annotation][in] */ 
            _In_  ULONG dataSize,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaults **defaults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOrAddControl( 
            /* [annotation][in] */ 
            _In_  MF_CAMERA_CONTROL_CONFIGURATION_TYPE configType,
            /* [annotation][in] */ 
            _In_  REFGUID controlSet,
            /* [annotation][in] */ 
            _In_  ULONG constrolId,
            /* [annotation][in] */ 
            _In_  ULONG controlSize,
            /* [annotation][in] */ 
            _In_  ULONG dataSize,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaults **defaults) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveControl( 
            /* [annotation][in] */ 
            _In_  REFGUID controlSet,
            /* [annotation][in] */ 
            _In_  ULONG constrolId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveAllControls( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraControlDefaultsCollectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraControlDefaultsCollection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraControlDefaultsCollection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraControlDefaultsCollection * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItem)
        HRESULT ( STDMETHODCALLTYPE *GetItem )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemType)
        HRESULT ( STDMETHODCALLTYPE *GetItemType )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ MF_ATTRIBUTE_TYPE *pType);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CompareItem)
        HRESULT ( STDMETHODCALLTYPE *CompareItem )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            REFPROPVARIANT Value,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, Compare)
        HRESULT ( STDMETHODCALLTYPE *Compare )( 
            IMFCameraControlDefaultsCollection * This,
            IMFAttributes *pTheirs,
            MF_ATTRIBUTES_MATCH_TYPE MatchType,
            /* [out] */ BOOL *pbResult);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT32)
        HRESULT ( STDMETHODCALLTYPE *GetUINT32 )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUINT64)
        HRESULT ( STDMETHODCALLTYPE *GetUINT64 )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ UINT64 *punValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetDouble)
        HRESULT ( STDMETHODCALLTYPE *GetDouble )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ double *pfValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetGUID)
        HRESULT ( STDMETHODCALLTYPE *GetGUID )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ GUID *pguidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetStringLength)
        HRESULT ( STDMETHODCALLTYPE *GetStringLength )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetString)
        HRESULT ( STDMETHODCALLTYPE *GetString )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [size_is][out] */ LPWSTR pwszValue,
            UINT32 cchBufSize,
            /* [full][out][in] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedString)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedString )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ LPWSTR *ppwszValue,
            /* [out] */ UINT32 *pcchLength);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlobSize)
        HRESULT ( STDMETHODCALLTYPE *GetBlobSize )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [out] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetBlob)
        HRESULT ( STDMETHODCALLTYPE *GetBlob )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [size_is][out] */ UINT8 *pBuf,
            UINT32 cbBufSize,
            /* [full][out][in] */ UINT32 *pcbBlobSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetAllocatedBlob)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatedBlob )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [size_is][size_is][out] */ UINT8 **ppBuf,
            /* [out] */ UINT32 *pcbSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetUnknown)
        HRESULT ( STDMETHODCALLTYPE *GetUnknown )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            REFIID riid,
            /* [iid_is][out] */ LPVOID *ppv);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetItem)
        HRESULT ( STDMETHODCALLTYPE *SetItem )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            REFPROPVARIANT Value);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteItem)
        HRESULT ( STDMETHODCALLTYPE *DeleteItem )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey);
        
        DECLSPEC_XFGVIRT(IMFAttributes, DeleteAllItems)
        HRESULT ( STDMETHODCALLTYPE *DeleteAllItems )( 
            IMFCameraControlDefaultsCollection * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT32)
        HRESULT ( STDMETHODCALLTYPE *SetUINT32 )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            UINT32 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUINT64)
        HRESULT ( STDMETHODCALLTYPE *SetUINT64 )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            UINT64 unValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetDouble)
        HRESULT ( STDMETHODCALLTYPE *SetDouble )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            double fValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetGUID)
        HRESULT ( STDMETHODCALLTYPE *SetGUID )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            REFGUID guidValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetString)
        HRESULT ( STDMETHODCALLTYPE *SetString )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [string][in] */ LPCWSTR wszValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetBlob)
        HRESULT ( STDMETHODCALLTYPE *SetBlob )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [size_is][in] */ const UINT8 *pBuf,
            UINT32 cbBufSize);
        
        DECLSPEC_XFGVIRT(IMFAttributes, SetUnknown)
        HRESULT ( STDMETHODCALLTYPE *SetUnknown )( 
            IMFCameraControlDefaultsCollection * This,
            REFGUID guidKey,
            /* [in] */ IUnknown *pUnknown);
        
        DECLSPEC_XFGVIRT(IMFAttributes, LockStore)
        HRESULT ( STDMETHODCALLTYPE *LockStore )( 
            IMFCameraControlDefaultsCollection * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, UnlockStore)
        HRESULT ( STDMETHODCALLTYPE *UnlockStore )( 
            IMFCameraControlDefaultsCollection * This);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetCount)
        HRESULT ( STDMETHODCALLTYPE *GetCount )( 
            IMFCameraControlDefaultsCollection * This,
            /* [out] */ UINT32 *pcItems);
        
        DECLSPEC_XFGVIRT(IMFAttributes, GetItemByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetItemByIndex )( 
            IMFCameraControlDefaultsCollection * This,
            UINT32 unIndex,
            /* [out] */ GUID *pguidKey,
            /* [full][out][in] */ PROPVARIANT *pValue);
        
        DECLSPEC_XFGVIRT(IMFAttributes, CopyAllItems)
        HRESULT ( STDMETHODCALLTYPE *CopyAllItems )( 
            IMFCameraControlDefaultsCollection * This,
            /* [in] */ IMFAttributes *pDest);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaultsCollection, GetControlCount)
        ULONG ( STDMETHODCALLTYPE *GetControlCount )( 
            IMFCameraControlDefaultsCollection * This);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaultsCollection, GetControl)
        HRESULT ( STDMETHODCALLTYPE *GetControl )( 
            IMFCameraControlDefaultsCollection * This,
            /* [annotation][in] */ 
            _In_  ULONG index,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaults **configuration);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaultsCollection, GetOrAddExtendedControl)
        HRESULT ( STDMETHODCALLTYPE *GetOrAddExtendedControl )( 
            IMFCameraControlDefaultsCollection * This,
            /* [annotation][in] */ 
            _In_  MF_CAMERA_CONTROL_CONFIGURATION_TYPE configType,
            /* [annotation][in] */ 
            _In_  ULONG constrolId,
            /* [annotation][in] */ 
            _In_  DWORD streamId,
            /* [annotation][in] */ 
            _In_  ULONG dataSize,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaults **defaults);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaultsCollection, GetOrAddControl)
        HRESULT ( STDMETHODCALLTYPE *GetOrAddControl )( 
            IMFCameraControlDefaultsCollection * This,
            /* [annotation][in] */ 
            _In_  MF_CAMERA_CONTROL_CONFIGURATION_TYPE configType,
            /* [annotation][in] */ 
            _In_  REFGUID controlSet,
            /* [annotation][in] */ 
            _In_  ULONG constrolId,
            /* [annotation][in] */ 
            _In_  ULONG controlSize,
            /* [annotation][in] */ 
            _In_  ULONG dataSize,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaults **defaults);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaultsCollection, RemoveControl)
        HRESULT ( STDMETHODCALLTYPE *RemoveControl )( 
            IMFCameraControlDefaultsCollection * This,
            /* [annotation][in] */ 
            _In_  REFGUID controlSet,
            /* [annotation][in] */ 
            _In_  ULONG constrolId);
        
        DECLSPEC_XFGVIRT(IMFCameraControlDefaultsCollection, RemoveAllControls)
        HRESULT ( STDMETHODCALLTYPE *RemoveAllControls )( 
            IMFCameraControlDefaultsCollection * This);
        
        END_INTERFACE
    } IMFCameraControlDefaultsCollectionVtbl;

    interface IMFCameraControlDefaultsCollection
    {
        CONST_VTBL struct IMFCameraControlDefaultsCollectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraControlDefaultsCollection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraControlDefaultsCollection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraControlDefaultsCollection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraControlDefaultsCollection_GetItem(This,guidKey,pValue)	\
    ( (This)->lpVtbl -> GetItem(This,guidKey,pValue) ) 

#define IMFCameraControlDefaultsCollection_GetItemType(This,guidKey,pType)	\
    ( (This)->lpVtbl -> GetItemType(This,guidKey,pType) ) 

#define IMFCameraControlDefaultsCollection_CompareItem(This,guidKey,Value,pbResult)	\
    ( (This)->lpVtbl -> CompareItem(This,guidKey,Value,pbResult) ) 

#define IMFCameraControlDefaultsCollection_Compare(This,pTheirs,MatchType,pbResult)	\
    ( (This)->lpVtbl -> Compare(This,pTheirs,MatchType,pbResult) ) 

#define IMFCameraControlDefaultsCollection_GetUINT32(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT32(This,guidKey,punValue) ) 

#define IMFCameraControlDefaultsCollection_GetUINT64(This,guidKey,punValue)	\
    ( (This)->lpVtbl -> GetUINT64(This,guidKey,punValue) ) 

#define IMFCameraControlDefaultsCollection_GetDouble(This,guidKey,pfValue)	\
    ( (This)->lpVtbl -> GetDouble(This,guidKey,pfValue) ) 

#define IMFCameraControlDefaultsCollection_GetGUID(This,guidKey,pguidValue)	\
    ( (This)->lpVtbl -> GetGUID(This,guidKey,pguidValue) ) 

#define IMFCameraControlDefaultsCollection_GetStringLength(This,guidKey,pcchLength)	\
    ( (This)->lpVtbl -> GetStringLength(This,guidKey,pcchLength) ) 

#define IMFCameraControlDefaultsCollection_GetString(This,guidKey,pwszValue,cchBufSize,pcchLength)	\
    ( (This)->lpVtbl -> GetString(This,guidKey,pwszValue,cchBufSize,pcchLength) ) 

#define IMFCameraControlDefaultsCollection_GetAllocatedString(This,guidKey,ppwszValue,pcchLength)	\
    ( (This)->lpVtbl -> GetAllocatedString(This,guidKey,ppwszValue,pcchLength) ) 

#define IMFCameraControlDefaultsCollection_GetBlobSize(This,guidKey,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlobSize(This,guidKey,pcbBlobSize) ) 

#define IMFCameraControlDefaultsCollection_GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize)	\
    ( (This)->lpVtbl -> GetBlob(This,guidKey,pBuf,cbBufSize,pcbBlobSize) ) 

#define IMFCameraControlDefaultsCollection_GetAllocatedBlob(This,guidKey,ppBuf,pcbSize)	\
    ( (This)->lpVtbl -> GetAllocatedBlob(This,guidKey,ppBuf,pcbSize) ) 

#define IMFCameraControlDefaultsCollection_GetUnknown(This,guidKey,riid,ppv)	\
    ( (This)->lpVtbl -> GetUnknown(This,guidKey,riid,ppv) ) 

#define IMFCameraControlDefaultsCollection_SetItem(This,guidKey,Value)	\
    ( (This)->lpVtbl -> SetItem(This,guidKey,Value) ) 

#define IMFCameraControlDefaultsCollection_DeleteItem(This,guidKey)	\
    ( (This)->lpVtbl -> DeleteItem(This,guidKey) ) 

#define IMFCameraControlDefaultsCollection_DeleteAllItems(This)	\
    ( (This)->lpVtbl -> DeleteAllItems(This) ) 

#define IMFCameraControlDefaultsCollection_SetUINT32(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT32(This,guidKey,unValue) ) 

#define IMFCameraControlDefaultsCollection_SetUINT64(This,guidKey,unValue)	\
    ( (This)->lpVtbl -> SetUINT64(This,guidKey,unValue) ) 

#define IMFCameraControlDefaultsCollection_SetDouble(This,guidKey,fValue)	\
    ( (This)->lpVtbl -> SetDouble(This,guidKey,fValue) ) 

#define IMFCameraControlDefaultsCollection_SetGUID(This,guidKey,guidValue)	\
    ( (This)->lpVtbl -> SetGUID(This,guidKey,guidValue) ) 

#define IMFCameraControlDefaultsCollection_SetString(This,guidKey,wszValue)	\
    ( (This)->lpVtbl -> SetString(This,guidKey,wszValue) ) 

#define IMFCameraControlDefaultsCollection_SetBlob(This,guidKey,pBuf,cbBufSize)	\
    ( (This)->lpVtbl -> SetBlob(This,guidKey,pBuf,cbBufSize) ) 

#define IMFCameraControlDefaultsCollection_SetUnknown(This,guidKey,pUnknown)	\
    ( (This)->lpVtbl -> SetUnknown(This,guidKey,pUnknown) ) 

#define IMFCameraControlDefaultsCollection_LockStore(This)	\
    ( (This)->lpVtbl -> LockStore(This) ) 

#define IMFCameraControlDefaultsCollection_UnlockStore(This)	\
    ( (This)->lpVtbl -> UnlockStore(This) ) 

#define IMFCameraControlDefaultsCollection_GetCount(This,pcItems)	\
    ( (This)->lpVtbl -> GetCount(This,pcItems) ) 

#define IMFCameraControlDefaultsCollection_GetItemByIndex(This,unIndex,pguidKey,pValue)	\
    ( (This)->lpVtbl -> GetItemByIndex(This,unIndex,pguidKey,pValue) ) 

#define IMFCameraControlDefaultsCollection_CopyAllItems(This,pDest)	\
    ( (This)->lpVtbl -> CopyAllItems(This,pDest) ) 


#define IMFCameraControlDefaultsCollection_GetControlCount(This)	\
    ( (This)->lpVtbl -> GetControlCount(This) ) 

#define IMFCameraControlDefaultsCollection_GetControl(This,index,configuration)	\
    ( (This)->lpVtbl -> GetControl(This,index,configuration) ) 

#define IMFCameraControlDefaultsCollection_GetOrAddExtendedControl(This,configType,constrolId,streamId,dataSize,defaults)	\
    ( (This)->lpVtbl -> GetOrAddExtendedControl(This,configType,constrolId,streamId,dataSize,defaults) ) 

#define IMFCameraControlDefaultsCollection_GetOrAddControl(This,configType,controlSet,constrolId,controlSize,dataSize,defaults)	\
    ( (This)->lpVtbl -> GetOrAddControl(This,configType,controlSet,constrolId,controlSize,dataSize,defaults) ) 

#define IMFCameraControlDefaultsCollection_RemoveControl(This,controlSet,constrolId)	\
    ( (This)->lpVtbl -> RemoveControl(This,controlSet,constrolId) ) 

#define IMFCameraControlDefaultsCollection_RemoveAllControls(This)	\
    ( (This)->lpVtbl -> RemoveAllControls(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraControlDefaultsCollection_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0138 */
/* [local] */ 

EXTERN_GUID(CLSID_CameraConfigurationManager, 0x6C92B540, 0x5854, 0x4A17,0x92, 0xB6, 0xAC, 0x89, 0xC9, 0x6E, 0x96, 0x83);


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0138_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0138_v0_0_s_ifspec;

#ifndef __IMFCameraConfigurationManager_INTERFACE_DEFINED__
#define __IMFCameraConfigurationManager_INTERFACE_DEFINED__

/* interface IMFCameraConfigurationManager */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFCameraConfigurationManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A624F617-4704-4206-8A6D-EBDA4A093985")
    IMFCameraConfigurationManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE LoadDefaults( 
            /* [annotation][in] */ 
            _In_  IMFAttributes *cameraAttributes,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaultsCollection **configurations) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SaveDefaults( 
            /* [annotation][in] */ 
            _In_  IMFCameraControlDefaultsCollection *configurations) = 0;
        
        virtual void STDMETHODCALLTYPE Shutdown( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFCameraConfigurationManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFCameraConfigurationManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFCameraConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFCameraConfigurationManager * This);
        
        DECLSPEC_XFGVIRT(IMFCameraConfigurationManager, LoadDefaults)
        HRESULT ( STDMETHODCALLTYPE *LoadDefaults )( 
            IMFCameraConfigurationManager * This,
            /* [annotation][in] */ 
            _In_  IMFAttributes *cameraAttributes,
            /* [annotation][out] */ 
            _COM_Outptr_  IMFCameraControlDefaultsCollection **configurations);
        
        DECLSPEC_XFGVIRT(IMFCameraConfigurationManager, SaveDefaults)
        HRESULT ( STDMETHODCALLTYPE *SaveDefaults )( 
            IMFCameraConfigurationManager * This,
            /* [annotation][in] */ 
            _In_  IMFCameraControlDefaultsCollection *configurations);
        
        DECLSPEC_XFGVIRT(IMFCameraConfigurationManager, Shutdown)
        void ( STDMETHODCALLTYPE *Shutdown )( 
            IMFCameraConfigurationManager * This);
        
        END_INTERFACE
    } IMFCameraConfigurationManagerVtbl;

    interface IMFCameraConfigurationManager
    {
        CONST_VTBL struct IMFCameraConfigurationManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFCameraConfigurationManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFCameraConfigurationManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFCameraConfigurationManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFCameraConfigurationManager_LoadDefaults(This,cameraAttributes,configurations)	\
    ( (This)->lpVtbl -> LoadDefaults(This,cameraAttributes,configurations) ) 

#define IMFCameraConfigurationManager_SaveDefaults(This,configurations)	\
    ( (This)->lpVtbl -> SaveDefaults(This,configurations) ) 

#define IMFCameraConfigurationManager_Shutdown(This)	\
    ( (This)->lpVtbl -> Shutdown(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFCameraConfigurationManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0139 */
/* [local] */ 

#endif // (NTDDI_VERSION >= NTDDI_WIN10_NI) 
#if (NTDDI_VERSION >= NTDDI_WIN11_ZN) 
typedef /* [public][public] */ struct __MIDL___MIDL_itf_mfidl_0000_0139_0001
    {
    DWORD sizeInBytes;
    float normalizedXPosition;
    float normalizedYPosition;
    float normalizedWidth;
    float normalizedHeight;
    LONG confidenceValue;
    ULONGLONG flags;
    } 	DetectedFaceBound;



extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0139_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0139_v0_0_s_ifspec;

#ifndef __IMFFaceDetectionTransformCallback_INTERFACE_DEFINED__
#define __IMFFaceDetectionTransformCallback_INTERFACE_DEFINED__

/* interface IMFFaceDetectionTransformCallback */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFFaceDetectionTransformCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0BFD1ADE-0421-4909-ACB7-7A7125416881")
    IMFFaceDetectionTransformCallback : public IUnknown
    {
    public:
        virtual void STDMETHODCALLTYPE OnFaceDetectionResult( 
            /* [annotation][in] */ 
            _In_  ULONG countOfBounds,
            /* [annotation][in] */ 
            _In_reads_(countOfBounds)  DetectedFaceBound *detectedFaceBounds) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFFaceDetectionTransformCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFFaceDetectionTransformCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFFaceDetectionTransformCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFFaceDetectionTransformCallback * This);
        
        DECLSPEC_XFGVIRT(IMFFaceDetectionTransformCallback, OnFaceDetectionResult)
        void ( STDMETHODCALLTYPE *OnFaceDetectionResult )( 
            IMFFaceDetectionTransformCallback * This,
            /* [annotation][in] */ 
            _In_  ULONG countOfBounds,
            /* [annotation][in] */ 
            _In_reads_(countOfBounds)  DetectedFaceBound *detectedFaceBounds);
        
        END_INTERFACE
    } IMFFaceDetectionTransformCallbackVtbl;

    interface IMFFaceDetectionTransformCallback
    {
        CONST_VTBL struct IMFFaceDetectionTransformCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFFaceDetectionTransformCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFFaceDetectionTransformCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFFaceDetectionTransformCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFFaceDetectionTransformCallback_OnFaceDetectionResult(This,countOfBounds,detectedFaceBounds)	\
    ( (This)->lpVtbl -> OnFaceDetectionResult(This,countOfBounds,detectedFaceBounds) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFFaceDetectionTransformCallback_INTERFACE_DEFINED__ */


#ifndef __IMFFaceDetectionTransform_INTERFACE_DEFINED__
#define __IMFFaceDetectionTransform_INTERFACE_DEFINED__

/* interface IMFFaceDetectionTransform */
/* [local][helpstring][uuid][object] */ 


EXTERN_C const IID IID_IMFFaceDetectionTransform;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DDD59578-D0E7-46E2-BE8C-1CE76AD147C0")
    IMFFaceDetectionTransform : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDetectionCallback( 
            /* [annotation][in] */ 
            _In_  IMFFaceDetectionTransformCallback *callback,
            /* [annotation][out] */ 
            _Out_  void **callbackToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ClearDetectionCallback( 
            /* [annotation][in] */ 
            _In_  void *callbackToken) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMFFaceDetectionTransformVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMFFaceDetectionTransform * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMFFaceDetectionTransform * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMFFaceDetectionTransform * This);
        
        DECLSPEC_XFGVIRT(IMFFaceDetectionTransform, SetDetectionCallback)
        HRESULT ( STDMETHODCALLTYPE *SetDetectionCallback )( 
            IMFFaceDetectionTransform * This,
            /* [annotation][in] */ 
            _In_  IMFFaceDetectionTransformCallback *callback,
            /* [annotation][out] */ 
            _Out_  void **callbackToken);
        
        DECLSPEC_XFGVIRT(IMFFaceDetectionTransform, ClearDetectionCallback)
        HRESULT ( STDMETHODCALLTYPE *ClearDetectionCallback )( 
            IMFFaceDetectionTransform * This,
            /* [annotation][in] */ 
            _In_  void *callbackToken);
        
        END_INTERFACE
    } IMFFaceDetectionTransformVtbl;

    interface IMFFaceDetectionTransform
    {
        CONST_VTBL struct IMFFaceDetectionTransformVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMFFaceDetectionTransform_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMFFaceDetectionTransform_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMFFaceDetectionTransform_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMFFaceDetectionTransform_SetDetectionCallback(This,callback,callbackToken)	\
    ( (This)->lpVtbl -> SetDetectionCallback(This,callback,callbackToken) ) 

#define IMFFaceDetectionTransform_ClearDetectionCallback(This,callbackToken)	\
    ( (This)->lpVtbl -> ClearDetectionCallback(This,callbackToken) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMFFaceDetectionTransform_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_mfidl_0000_0141 */
/* [local] */ 

EXTERN_GUID(CLSID_FaceDetectionMFT,  0xc1e565e2, 0xf2de, 0x4537, 0x96, 0x12, 0x2f, 0x30, 0xa1, 0x60, 0xeb, 0x5c);
EXTERN_GUID(CLSID_FrameServerClassFactory, 0x9A93092C, 0x9CDC, 0x49B8, 0x83, 0x49, 0xCB, 0xCF, 0x31, 0x45, 0xFE, 0x0A);
EXTERN_GUID(MF_CAMERASOURCE_PROVIDE_SELECTED_PROFILE_ON_START, 0xA9B46058, 0x82F2, 0x4E5C, 0xBF, 0x6E, 0x25, 0xB4, 0xB0, 0x9F, 0x22, 0xED);
EXTERN_GUID(MF_DEVSOURCE_ATTRIBUTE_FRAMESERVER_SHARE_MODE, 0x44d1a9bc, 0x2999, 0x4238, 0xae, 0x43, 0x07, 0x30, 0xce, 0xb2, 0xab, 0x1b);
#endif // (NTDDI_VERSION >= NTDDI_WIN11_ZN) 
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion 


extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0141_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_mfidl_0000_0141_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

unsigned long             __RPC_USER  BSTR_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

unsigned long             __RPC_USER  BSTR_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in BSTR * ); 
unsigned char * __RPC_USER  BSTR_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out BSTR * ); 
void                      __RPC_USER  BSTR_UserFree64(     __RPC__in unsigned long *, __RPC__in BSTR * ); 

unsigned long             __RPC_USER  LPSAFEARRAY_UserSize64(     __RPC__in unsigned long *, unsigned long            , __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserMarshal64(  __RPC__in unsigned long *, __RPC__inout_xcount(0) unsigned char *, __RPC__in LPSAFEARRAY * ); 
unsigned char * __RPC_USER  LPSAFEARRAY_UserUnmarshal64(__RPC__in unsigned long *, __RPC__in_xcount(0) unsigned char *, __RPC__out LPSAFEARRAY * ); 
void                      __RPC_USER  LPSAFEARRAY_UserFree64(     __RPC__in unsigned long *, __RPC__in LPSAFEARRAY * ); 

/* [local] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_BeginCreateObjectFromURL_Proxy( 
    IMFSourceResolver * This,
    /* [in] */ LPCWSTR pwszURL,
    /* [in] */ DWORD dwFlags,
    /* [in] */ IPropertyStore *pProps,
    /* [annotation][out] */ 
    _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *punkState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_BeginCreateObjectFromURL_Stub( 
    __RPC__in IMFSourceResolver * This,
    /* [string][in] */ __RPC__in_string LPCWSTR pwszURL,
    /* [in] */ DWORD dwFlags,
    /* [in] */ __RPC__in_opt IPropertyStore *pProps,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_EndCreateObjectFromURL_Proxy( 
    IMFSourceResolver * This,
    /* [in] */ IMFAsyncResult *pResult,
    /* [annotation][out] */ 
    _Out_  MF_OBJECT_TYPE *pObjectType,
    /* [annotation][out] */ 
    _Outptr_  IUnknown **ppObject);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_EndCreateObjectFromURL_Stub( 
    __RPC__in IMFSourceResolver * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out MF_OBJECT_TYPE *pObjectType,
    /* [out] */ __RPC__deref_out_opt IUnknown **ppObject);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_BeginCreateObjectFromByteStream_Proxy( 
    IMFSourceResolver * This,
    /* [in] */ IMFByteStream *pByteStream,
    /* [in] */ LPCWSTR pwszURL,
    /* [in] */ DWORD dwFlags,
    /* [in] */ IPropertyStore *pProps,
    /* [annotation][out] */ 
    _Outptr_opt_  IUnknown **ppIUnknownCancelCookie,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *punkState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_BeginCreateObjectFromByteStream_Stub( 
    __RPC__in IMFSourceResolver * This,
    /* [in] */ __RPC__in_opt IMFByteStream *pByteStream,
    /* [unique][in] */ __RPC__in_opt LPCWSTR pwszURL,
    /* [in] */ DWORD dwFlags,
    /* [unique][in] */ __RPC__in_opt IPropertyStore *pProps,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_EndCreateObjectFromByteStream_Proxy( 
    IMFSourceResolver * This,
    /* [in] */ IMFAsyncResult *pResult,
    /* [annotation][out] */ 
    _Out_  MF_OBJECT_TYPE *pObjectType,
    /* [annotation][out] */ 
    _Outptr_  IUnknown **ppObject);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFSourceResolver_EndCreateObjectFromByteStream_Stub( 
    __RPC__in IMFSourceResolver * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out MF_OBJECT_TYPE *pObjectType,
    /* [out] */ __RPC__deref_out_opt IUnknown **ppObject);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaSource_CreatePresentationDescriptor_Proxy( 
    IMFMediaSource * This,
    /* [annotation][out] */ 
    _Outptr_  IMFPresentationDescriptor **ppPresentationDescriptor);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaSource_CreatePresentationDescriptor_Stub( 
    __RPC__in IMFMediaSource * This,
    /* [out] */ __RPC__out DWORD *pcbPD,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbPD) BYTE **pbPD,
    /* [out] */ __RPC__deref_out_opt IMFPresentationDescriptor **ppRemotePD);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaStream_RequestSample_Proxy( 
    IMFMediaStream * This,
    /* [in] */ IUnknown *pToken);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaStream_RequestSample_Stub( 
    __RPC__in IMFMediaStream * This);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFTopologyNode_GetOutputPrefType_Proxy( 
    IMFTopologyNode * This,
    /* [in] */ DWORD dwOutputIndex,
    /* [annotation][out] */ 
    _Outptr_  IMFMediaType **ppType);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFTopologyNode_GetOutputPrefType_Stub( 
    __RPC__in IMFTopologyNode * This,
    /* [in] */ DWORD dwOutputIndex,
    /* [out] */ __RPC__out DWORD *pcbData,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFTopologyNode_GetInputPrefType_Proxy( 
    IMFTopologyNode * This,
    /* [in] */ DWORD dwInputIndex,
    /* [annotation][out] */ 
    _Outptr_  IMFMediaType **ppType);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFTopologyNode_GetInputPrefType_Stub( 
    __RPC__in IMFTopologyNode * This,
    /* [in] */ DWORD dwInputIndex,
    /* [out] */ __RPC__out DWORD *pcbData,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_IsMediaTypeSupported_Proxy( 
    IMFMediaTypeHandler * This,
    /* [in] */ IMFMediaType *pMediaType,
    /* [annotation][out] */ 
    _Outptr_opt_result_maybenull_  IMFMediaType **ppMediaType);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_IsMediaTypeSupported_Stub( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbBestMatch) BYTE **ppbBestMatch,
    /* [out] */ __RPC__out DWORD *pcbBestMatch);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_GetMediaTypeByIndex_Proxy( 
    IMFMediaTypeHandler * This,
    /* [in] */ DWORD dwIndex,
    /* [annotation][out] */ 
    _Outptr_  IMFMediaType **ppType);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_GetMediaTypeByIndex_Stub( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [in] */ DWORD dwIndex,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
    /* [out] */ __RPC__out DWORD *pcbData);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_SetCurrentMediaType_Proxy( 
    IMFMediaTypeHandler * This,
    /* [in] */ IMFMediaType *pMediaType);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_SetCurrentMediaType_Stub( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [size_is][in] */ __RPC__in_ecount_full(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_GetCurrentMediaType_Proxy( 
    IMFMediaTypeHandler * This,
    /* [annotation][out] */ 
    _Outptr_  IMFMediaType **ppMediaType);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFMediaTypeHandler_GetCurrentMediaType_Stub( 
    __RPC__in IMFMediaTypeHandler * This,
    /* [size_is][size_is][out] */ __RPC__deref_out_ecount_full_opt(*pcbData) BYTE **ppbData,
    /* [out] */ __RPC__out DWORD *pcbData);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFContentProtectionManager_BeginEnableContent_Proxy( 
    IMFContentProtectionManager * This,
    /* [in] */ IMFActivate *pEnablerActivate,
    /* [in] */ IMFTopology *pTopo,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *punkState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFContentProtectionManager_BeginEnableContent_Stub( 
    __RPC__in IMFContentProtectionManager * This,
    /* [in] */ __RPC__in REFCLSID clsidType,
    /* [size_is][in] */ __RPC__in_ecount_full(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFContentProtectionManager_EndEnableContent_Proxy( 
    IMFContentProtectionManager * This,
    /* [in] */ IMFAsyncResult *pResult);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFContentProtectionManager_EndEnableContent_Stub( 
    __RPC__in IMFContentProtectionManager * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginRegisterTopologyWorkQueuesWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *pState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginRegisterTopologyWorkQueuesWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndRegisterTopologyWorkQueuesWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ IMFAsyncResult *pResult);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndRegisterTopologyWorkQueuesWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginUnregisterTopologyWorkQueuesWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *pState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginUnregisterTopologyWorkQueuesWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndUnregisterTopologyWorkQueuesWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ IMFAsyncResult *pResult);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndUnregisterTopologyWorkQueuesWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginRegisterPlatformWorkQueueWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ LPCWSTR wszClass,
    /* [in] */ DWORD dwTaskId,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *pState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginRegisterPlatformWorkQueueWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ __RPC__in LPCWSTR wszClass,
    /* [in] */ DWORD dwTaskId,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndRegisterPlatformWorkQueueWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ IMFAsyncResult *pResult,
    /* [annotation][out] */ 
    _Out_  DWORD *pdwTaskId);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndRegisterPlatformWorkQueueWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult,
    /* [out] */ __RPC__out DWORD *pdwTaskId);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginUnregisterPlatformWorkQueueWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *pState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_BeginUnregisterPlatformWorkQueueWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndUnregisterPlatformWorkQueueWithMMCSS_Proxy( 
    IMFWorkQueueServices * This,
    /* [in] */ IMFAsyncResult *pResult);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServices_EndUnregisterPlatformWorkQueueWithMMCSS_Stub( 
    __RPC__in IMFWorkQueueServices * This,
    /* [in] */ __RPC__in_opt IUnknown *pResult);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServicesEx_BeginRegisterPlatformWorkQueueWithMMCSSEx_Proxy( 
    IMFWorkQueueServicesEx * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ LPCWSTR wszClass,
    /* [in] */ DWORD dwTaskId,
    /* [in] */ LONG lPriority,
    /* [in] */ IMFAsyncCallback *pCallback,
    /* [in] */ IUnknown *pState);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFWorkQueueServicesEx_BeginRegisterPlatformWorkQueueWithMMCSSEx_Stub( 
    __RPC__in IMFWorkQueueServicesEx * This,
    /* [in] */ DWORD dwPlatformWorkQueue,
    /* [in] */ __RPC__in LPCWSTR wszClass,
    /* [in] */ DWORD dwTaskId,
    /* [in] */ LONG lPriority,
    /* [in] */ __RPC__in_opt IMFRemoteAsyncCallback *pCallback);

/* [local] */ HRESULT STDMETHODCALLTYPE IMFPMPHost_CreateObjectByCLSID_Proxy( 
    IMFPMPHost * This,
    /* [in] */ REFCLSID clsid,
    /* [unique][in] */ IStream *pStream,
    /* [in] */ REFIID riid,
    /* [iid_is][out] */ void **ppv);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IMFPMPHost_CreateObjectByCLSID_Stub( 
    __RPC__in IMFPMPHost * This,
    /* [in] */ __RPC__in REFCLSID clsid,
    /* [size_is][unique][in] */ __RPC__in_ecount_full_opt(cbData) BYTE *pbData,
    /* [in] */ DWORD cbData,
    /* [in] */ __RPC__in REFIID riid,
    /* [iid_is][out] */ __RPC__deref_out_opt void **ppv);



/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


