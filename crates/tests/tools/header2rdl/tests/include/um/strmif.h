

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

#ifndef __strmif_h__
#define __strmif_h__

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

#ifndef __ICreateDevEnum_FWD_DEFINED__
#define __ICreateDevEnum_FWD_DEFINED__
typedef interface ICreateDevEnum ICreateDevEnum;

#endif 	/* __ICreateDevEnum_FWD_DEFINED__ */


#ifndef __IPin_FWD_DEFINED__
#define __IPin_FWD_DEFINED__
typedef interface IPin IPin;

#endif 	/* __IPin_FWD_DEFINED__ */


#ifndef __IEnumPins_FWD_DEFINED__
#define __IEnumPins_FWD_DEFINED__
typedef interface IEnumPins IEnumPins;

#endif 	/* __IEnumPins_FWD_DEFINED__ */


#ifndef __IEnumMediaTypes_FWD_DEFINED__
#define __IEnumMediaTypes_FWD_DEFINED__
typedef interface IEnumMediaTypes IEnumMediaTypes;

#endif 	/* __IEnumMediaTypes_FWD_DEFINED__ */


#ifndef __IFilterGraph_FWD_DEFINED__
#define __IFilterGraph_FWD_DEFINED__
typedef interface IFilterGraph IFilterGraph;

#endif 	/* __IFilterGraph_FWD_DEFINED__ */


#ifndef __IEnumFilters_FWD_DEFINED__
#define __IEnumFilters_FWD_DEFINED__
typedef interface IEnumFilters IEnumFilters;

#endif 	/* __IEnumFilters_FWD_DEFINED__ */


#ifndef __IMediaFilter_FWD_DEFINED__
#define __IMediaFilter_FWD_DEFINED__
typedef interface IMediaFilter IMediaFilter;

#endif 	/* __IMediaFilter_FWD_DEFINED__ */


#ifndef __IBaseFilter_FWD_DEFINED__
#define __IBaseFilter_FWD_DEFINED__
typedef interface IBaseFilter IBaseFilter;

#endif 	/* __IBaseFilter_FWD_DEFINED__ */


#ifndef __IReferenceClock_FWD_DEFINED__
#define __IReferenceClock_FWD_DEFINED__
typedef interface IReferenceClock IReferenceClock;

#endif 	/* __IReferenceClock_FWD_DEFINED__ */


#ifndef __IReferenceClockTimerControl_FWD_DEFINED__
#define __IReferenceClockTimerControl_FWD_DEFINED__
typedef interface IReferenceClockTimerControl IReferenceClockTimerControl;

#endif 	/* __IReferenceClockTimerControl_FWD_DEFINED__ */


#ifndef __IReferenceClock2_FWD_DEFINED__
#define __IReferenceClock2_FWD_DEFINED__
typedef interface IReferenceClock2 IReferenceClock2;

#endif 	/* __IReferenceClock2_FWD_DEFINED__ */


#ifndef __IMediaSample_FWD_DEFINED__
#define __IMediaSample_FWD_DEFINED__
typedef interface IMediaSample IMediaSample;

#endif 	/* __IMediaSample_FWD_DEFINED__ */


#ifndef __IMediaSample2_FWD_DEFINED__
#define __IMediaSample2_FWD_DEFINED__
typedef interface IMediaSample2 IMediaSample2;

#endif 	/* __IMediaSample2_FWD_DEFINED__ */


#ifndef __IMediaSample2Config_FWD_DEFINED__
#define __IMediaSample2Config_FWD_DEFINED__
typedef interface IMediaSample2Config IMediaSample2Config;

#endif 	/* __IMediaSample2Config_FWD_DEFINED__ */


#ifndef __IMemAllocator_FWD_DEFINED__
#define __IMemAllocator_FWD_DEFINED__
typedef interface IMemAllocator IMemAllocator;

#endif 	/* __IMemAllocator_FWD_DEFINED__ */


#ifndef __IMemAllocatorCallbackTemp_FWD_DEFINED__
#define __IMemAllocatorCallbackTemp_FWD_DEFINED__
typedef interface IMemAllocatorCallbackTemp IMemAllocatorCallbackTemp;

#endif 	/* __IMemAllocatorCallbackTemp_FWD_DEFINED__ */


#ifndef __IMemAllocatorNotifyCallbackTemp_FWD_DEFINED__
#define __IMemAllocatorNotifyCallbackTemp_FWD_DEFINED__
typedef interface IMemAllocatorNotifyCallbackTemp IMemAllocatorNotifyCallbackTemp;

#endif 	/* __IMemAllocatorNotifyCallbackTemp_FWD_DEFINED__ */


#ifndef __IMemInputPin_FWD_DEFINED__
#define __IMemInputPin_FWD_DEFINED__
typedef interface IMemInputPin IMemInputPin;

#endif 	/* __IMemInputPin_FWD_DEFINED__ */


#ifndef __IAMovieSetup_FWD_DEFINED__
#define __IAMovieSetup_FWD_DEFINED__
typedef interface IAMovieSetup IAMovieSetup;

#endif 	/* __IAMovieSetup_FWD_DEFINED__ */


#ifndef __IMediaSeeking_FWD_DEFINED__
#define __IMediaSeeking_FWD_DEFINED__
typedef interface IMediaSeeking IMediaSeeking;

#endif 	/* __IMediaSeeking_FWD_DEFINED__ */


#ifndef __ICodecAPI_FWD_DEFINED__
#define __ICodecAPI_FWD_DEFINED__
typedef interface ICodecAPI ICodecAPI;

#endif 	/* __ICodecAPI_FWD_DEFINED__ */


#ifndef __IEnumRegFilters_FWD_DEFINED__
#define __IEnumRegFilters_FWD_DEFINED__
typedef interface IEnumRegFilters IEnumRegFilters;

#endif 	/* __IEnumRegFilters_FWD_DEFINED__ */


#ifndef __IFilterMapper_FWD_DEFINED__
#define __IFilterMapper_FWD_DEFINED__
typedef interface IFilterMapper IFilterMapper;

#endif 	/* __IFilterMapper_FWD_DEFINED__ */


#ifndef __IFilterMapper2_FWD_DEFINED__
#define __IFilterMapper2_FWD_DEFINED__
typedef interface IFilterMapper2 IFilterMapper2;

#endif 	/* __IFilterMapper2_FWD_DEFINED__ */


#ifndef __IFilterMapper3_FWD_DEFINED__
#define __IFilterMapper3_FWD_DEFINED__
typedef interface IFilterMapper3 IFilterMapper3;

#endif 	/* __IFilterMapper3_FWD_DEFINED__ */


#ifndef __IQualityControl_FWD_DEFINED__
#define __IQualityControl_FWD_DEFINED__
typedef interface IQualityControl IQualityControl;

#endif 	/* __IQualityControl_FWD_DEFINED__ */


#ifndef __IOverlayNotify_FWD_DEFINED__
#define __IOverlayNotify_FWD_DEFINED__
typedef interface IOverlayNotify IOverlayNotify;

#endif 	/* __IOverlayNotify_FWD_DEFINED__ */


#ifndef __IOverlayNotify2_FWD_DEFINED__
#define __IOverlayNotify2_FWD_DEFINED__
typedef interface IOverlayNotify2 IOverlayNotify2;

#endif 	/* __IOverlayNotify2_FWD_DEFINED__ */


#ifndef __IOverlay_FWD_DEFINED__
#define __IOverlay_FWD_DEFINED__
typedef interface IOverlay IOverlay;

#endif 	/* __IOverlay_FWD_DEFINED__ */


#ifndef __IMediaEventSink_FWD_DEFINED__
#define __IMediaEventSink_FWD_DEFINED__
typedef interface IMediaEventSink IMediaEventSink;

#endif 	/* __IMediaEventSink_FWD_DEFINED__ */


#ifndef __IFileSourceFilter_FWD_DEFINED__
#define __IFileSourceFilter_FWD_DEFINED__
typedef interface IFileSourceFilter IFileSourceFilter;

#endif 	/* __IFileSourceFilter_FWD_DEFINED__ */


#ifndef __IFileSinkFilter_FWD_DEFINED__
#define __IFileSinkFilter_FWD_DEFINED__
typedef interface IFileSinkFilter IFileSinkFilter;

#endif 	/* __IFileSinkFilter_FWD_DEFINED__ */


#ifndef __IFileSinkFilter2_FWD_DEFINED__
#define __IFileSinkFilter2_FWD_DEFINED__
typedef interface IFileSinkFilter2 IFileSinkFilter2;

#endif 	/* __IFileSinkFilter2_FWD_DEFINED__ */


#ifndef __IGraphBuilder_FWD_DEFINED__
#define __IGraphBuilder_FWD_DEFINED__
typedef interface IGraphBuilder IGraphBuilder;

#endif 	/* __IGraphBuilder_FWD_DEFINED__ */


#ifndef __ICaptureGraphBuilder_FWD_DEFINED__
#define __ICaptureGraphBuilder_FWD_DEFINED__
typedef interface ICaptureGraphBuilder ICaptureGraphBuilder;

#endif 	/* __ICaptureGraphBuilder_FWD_DEFINED__ */


#ifndef __IAMCopyCaptureFileProgress_FWD_DEFINED__
#define __IAMCopyCaptureFileProgress_FWD_DEFINED__
typedef interface IAMCopyCaptureFileProgress IAMCopyCaptureFileProgress;

#endif 	/* __IAMCopyCaptureFileProgress_FWD_DEFINED__ */


#ifndef __ICaptureGraphBuilder2_FWD_DEFINED__
#define __ICaptureGraphBuilder2_FWD_DEFINED__
typedef interface ICaptureGraphBuilder2 ICaptureGraphBuilder2;

#endif 	/* __ICaptureGraphBuilder2_FWD_DEFINED__ */


#ifndef __IFilterGraph2_FWD_DEFINED__
#define __IFilterGraph2_FWD_DEFINED__
typedef interface IFilterGraph2 IFilterGraph2;

#endif 	/* __IFilterGraph2_FWD_DEFINED__ */


#ifndef __IFilterGraph3_FWD_DEFINED__
#define __IFilterGraph3_FWD_DEFINED__
typedef interface IFilterGraph3 IFilterGraph3;

#endif 	/* __IFilterGraph3_FWD_DEFINED__ */


#ifndef __IStreamBuilder_FWD_DEFINED__
#define __IStreamBuilder_FWD_DEFINED__
typedef interface IStreamBuilder IStreamBuilder;

#endif 	/* __IStreamBuilder_FWD_DEFINED__ */


#ifndef __IAsyncReader_FWD_DEFINED__
#define __IAsyncReader_FWD_DEFINED__
typedef interface IAsyncReader IAsyncReader;

#endif 	/* __IAsyncReader_FWD_DEFINED__ */


#ifndef __IGraphVersion_FWD_DEFINED__
#define __IGraphVersion_FWD_DEFINED__
typedef interface IGraphVersion IGraphVersion;

#endif 	/* __IGraphVersion_FWD_DEFINED__ */


#ifndef __IResourceConsumer_FWD_DEFINED__
#define __IResourceConsumer_FWD_DEFINED__
typedef interface IResourceConsumer IResourceConsumer;

#endif 	/* __IResourceConsumer_FWD_DEFINED__ */


#ifndef __IResourceManager_FWD_DEFINED__
#define __IResourceManager_FWD_DEFINED__
typedef interface IResourceManager IResourceManager;

#endif 	/* __IResourceManager_FWD_DEFINED__ */


#ifndef __IDistributorNotify_FWD_DEFINED__
#define __IDistributorNotify_FWD_DEFINED__
typedef interface IDistributorNotify IDistributorNotify;

#endif 	/* __IDistributorNotify_FWD_DEFINED__ */


#ifndef __IAMStreamControl_FWD_DEFINED__
#define __IAMStreamControl_FWD_DEFINED__
typedef interface IAMStreamControl IAMStreamControl;

#endif 	/* __IAMStreamControl_FWD_DEFINED__ */


#ifndef __ISeekingPassThru_FWD_DEFINED__
#define __ISeekingPassThru_FWD_DEFINED__
typedef interface ISeekingPassThru ISeekingPassThru;

#endif 	/* __ISeekingPassThru_FWD_DEFINED__ */


#ifndef __IAMStreamConfig_FWD_DEFINED__
#define __IAMStreamConfig_FWD_DEFINED__
typedef interface IAMStreamConfig IAMStreamConfig;

#endif 	/* __IAMStreamConfig_FWD_DEFINED__ */


#ifndef __IConfigInterleaving_FWD_DEFINED__
#define __IConfigInterleaving_FWD_DEFINED__
typedef interface IConfigInterleaving IConfigInterleaving;

#endif 	/* __IConfigInterleaving_FWD_DEFINED__ */


#ifndef __IConfigAviMux_FWD_DEFINED__
#define __IConfigAviMux_FWD_DEFINED__
typedef interface IConfigAviMux IConfigAviMux;

#endif 	/* __IConfigAviMux_FWD_DEFINED__ */


#ifndef __IAMVideoCompression_FWD_DEFINED__
#define __IAMVideoCompression_FWD_DEFINED__
typedef interface IAMVideoCompression IAMVideoCompression;

#endif 	/* __IAMVideoCompression_FWD_DEFINED__ */


#ifndef __IAMVfwCaptureDialogs_FWD_DEFINED__
#define __IAMVfwCaptureDialogs_FWD_DEFINED__
typedef interface IAMVfwCaptureDialogs IAMVfwCaptureDialogs;

#endif 	/* __IAMVfwCaptureDialogs_FWD_DEFINED__ */


#ifndef __IAMVfwCompressDialogs_FWD_DEFINED__
#define __IAMVfwCompressDialogs_FWD_DEFINED__
typedef interface IAMVfwCompressDialogs IAMVfwCompressDialogs;

#endif 	/* __IAMVfwCompressDialogs_FWD_DEFINED__ */


#ifndef __IAMDroppedFrames_FWD_DEFINED__
#define __IAMDroppedFrames_FWD_DEFINED__
typedef interface IAMDroppedFrames IAMDroppedFrames;

#endif 	/* __IAMDroppedFrames_FWD_DEFINED__ */


#ifndef __IAMAudioInputMixer_FWD_DEFINED__
#define __IAMAudioInputMixer_FWD_DEFINED__
typedef interface IAMAudioInputMixer IAMAudioInputMixer;

#endif 	/* __IAMAudioInputMixer_FWD_DEFINED__ */


#ifndef __IAMBufferNegotiation_FWD_DEFINED__
#define __IAMBufferNegotiation_FWD_DEFINED__
typedef interface IAMBufferNegotiation IAMBufferNegotiation;

#endif 	/* __IAMBufferNegotiation_FWD_DEFINED__ */


#ifndef __IAMAnalogVideoDecoder_FWD_DEFINED__
#define __IAMAnalogVideoDecoder_FWD_DEFINED__
typedef interface IAMAnalogVideoDecoder IAMAnalogVideoDecoder;

#endif 	/* __IAMAnalogVideoDecoder_FWD_DEFINED__ */


#ifndef __IAMVideoProcAmp_FWD_DEFINED__
#define __IAMVideoProcAmp_FWD_DEFINED__
typedef interface IAMVideoProcAmp IAMVideoProcAmp;

#endif 	/* __IAMVideoProcAmp_FWD_DEFINED__ */


#ifndef __IAMCameraControl_FWD_DEFINED__
#define __IAMCameraControl_FWD_DEFINED__
typedef interface IAMCameraControl IAMCameraControl;

#endif 	/* __IAMCameraControl_FWD_DEFINED__ */


#ifndef __IAMVideoControl_FWD_DEFINED__
#define __IAMVideoControl_FWD_DEFINED__
typedef interface IAMVideoControl IAMVideoControl;

#endif 	/* __IAMVideoControl_FWD_DEFINED__ */


#ifndef __IAMCrossbar_FWD_DEFINED__
#define __IAMCrossbar_FWD_DEFINED__
typedef interface IAMCrossbar IAMCrossbar;

#endif 	/* __IAMCrossbar_FWD_DEFINED__ */


#ifndef __IAMTuner_FWD_DEFINED__
#define __IAMTuner_FWD_DEFINED__
typedef interface IAMTuner IAMTuner;

#endif 	/* __IAMTuner_FWD_DEFINED__ */


#ifndef __IAMTunerNotification_FWD_DEFINED__
#define __IAMTunerNotification_FWD_DEFINED__
typedef interface IAMTunerNotification IAMTunerNotification;

#endif 	/* __IAMTunerNotification_FWD_DEFINED__ */


#ifndef __IAMTVTuner_FWD_DEFINED__
#define __IAMTVTuner_FWD_DEFINED__
typedef interface IAMTVTuner IAMTVTuner;

#endif 	/* __IAMTVTuner_FWD_DEFINED__ */


#ifndef __IBPCSatelliteTuner_FWD_DEFINED__
#define __IBPCSatelliteTuner_FWD_DEFINED__
typedef interface IBPCSatelliteTuner IBPCSatelliteTuner;

#endif 	/* __IBPCSatelliteTuner_FWD_DEFINED__ */


#ifndef __IAMTVAudio_FWD_DEFINED__
#define __IAMTVAudio_FWD_DEFINED__
typedef interface IAMTVAudio IAMTVAudio;

#endif 	/* __IAMTVAudio_FWD_DEFINED__ */


#ifndef __IAMTVAudioNotification_FWD_DEFINED__
#define __IAMTVAudioNotification_FWD_DEFINED__
typedef interface IAMTVAudioNotification IAMTVAudioNotification;

#endif 	/* __IAMTVAudioNotification_FWD_DEFINED__ */


#ifndef __IAMAnalogVideoEncoder_FWD_DEFINED__
#define __IAMAnalogVideoEncoder_FWD_DEFINED__
typedef interface IAMAnalogVideoEncoder IAMAnalogVideoEncoder;

#endif 	/* __IAMAnalogVideoEncoder_FWD_DEFINED__ */


#ifndef __IKsPropertySet_FWD_DEFINED__
#define __IKsPropertySet_FWD_DEFINED__
typedef interface IKsPropertySet IKsPropertySet;

#endif 	/* __IKsPropertySet_FWD_DEFINED__ */


#ifndef __IMediaPropertyBag_FWD_DEFINED__
#define __IMediaPropertyBag_FWD_DEFINED__
typedef interface IMediaPropertyBag IMediaPropertyBag;

#endif 	/* __IMediaPropertyBag_FWD_DEFINED__ */


#ifndef __IPersistMediaPropertyBag_FWD_DEFINED__
#define __IPersistMediaPropertyBag_FWD_DEFINED__
typedef interface IPersistMediaPropertyBag IPersistMediaPropertyBag;

#endif 	/* __IPersistMediaPropertyBag_FWD_DEFINED__ */


#ifndef __IAMPhysicalPinInfo_FWD_DEFINED__
#define __IAMPhysicalPinInfo_FWD_DEFINED__
typedef interface IAMPhysicalPinInfo IAMPhysicalPinInfo;

#endif 	/* __IAMPhysicalPinInfo_FWD_DEFINED__ */


#ifndef __IAMExtDevice_FWD_DEFINED__
#define __IAMExtDevice_FWD_DEFINED__
typedef interface IAMExtDevice IAMExtDevice;

#endif 	/* __IAMExtDevice_FWD_DEFINED__ */


#ifndef __IAMExtTransport_FWD_DEFINED__
#define __IAMExtTransport_FWD_DEFINED__
typedef interface IAMExtTransport IAMExtTransport;

#endif 	/* __IAMExtTransport_FWD_DEFINED__ */


#ifndef __IAMTimecodeReader_FWD_DEFINED__
#define __IAMTimecodeReader_FWD_DEFINED__
typedef interface IAMTimecodeReader IAMTimecodeReader;

#endif 	/* __IAMTimecodeReader_FWD_DEFINED__ */


#ifndef __IAMTimecodeGenerator_FWD_DEFINED__
#define __IAMTimecodeGenerator_FWD_DEFINED__
typedef interface IAMTimecodeGenerator IAMTimecodeGenerator;

#endif 	/* __IAMTimecodeGenerator_FWD_DEFINED__ */


#ifndef __IAMTimecodeDisplay_FWD_DEFINED__
#define __IAMTimecodeDisplay_FWD_DEFINED__
typedef interface IAMTimecodeDisplay IAMTimecodeDisplay;

#endif 	/* __IAMTimecodeDisplay_FWD_DEFINED__ */


#ifndef __IAMDevMemoryAllocator_FWD_DEFINED__
#define __IAMDevMemoryAllocator_FWD_DEFINED__
typedef interface IAMDevMemoryAllocator IAMDevMemoryAllocator;

#endif 	/* __IAMDevMemoryAllocator_FWD_DEFINED__ */


#ifndef __IAMDevMemoryControl_FWD_DEFINED__
#define __IAMDevMemoryControl_FWD_DEFINED__
typedef interface IAMDevMemoryControl IAMDevMemoryControl;

#endif 	/* __IAMDevMemoryControl_FWD_DEFINED__ */


#ifndef __IAMStreamSelect_FWD_DEFINED__
#define __IAMStreamSelect_FWD_DEFINED__
typedef interface IAMStreamSelect IAMStreamSelect;

#endif 	/* __IAMStreamSelect_FWD_DEFINED__ */


#ifndef __IAMResourceControl_FWD_DEFINED__
#define __IAMResourceControl_FWD_DEFINED__
typedef interface IAMResourceControl IAMResourceControl;

#endif 	/* __IAMResourceControl_FWD_DEFINED__ */


#ifndef __IAMClockAdjust_FWD_DEFINED__
#define __IAMClockAdjust_FWD_DEFINED__
typedef interface IAMClockAdjust IAMClockAdjust;

#endif 	/* __IAMClockAdjust_FWD_DEFINED__ */


#ifndef __IAMFilterMiscFlags_FWD_DEFINED__
#define __IAMFilterMiscFlags_FWD_DEFINED__
typedef interface IAMFilterMiscFlags IAMFilterMiscFlags;

#endif 	/* __IAMFilterMiscFlags_FWD_DEFINED__ */


#ifndef __IDrawVideoImage_FWD_DEFINED__
#define __IDrawVideoImage_FWD_DEFINED__
typedef interface IDrawVideoImage IDrawVideoImage;

#endif 	/* __IDrawVideoImage_FWD_DEFINED__ */


#ifndef __IDecimateVideoImage_FWD_DEFINED__
#define __IDecimateVideoImage_FWD_DEFINED__
typedef interface IDecimateVideoImage IDecimateVideoImage;

#endif 	/* __IDecimateVideoImage_FWD_DEFINED__ */


#ifndef __IAMVideoDecimationProperties_FWD_DEFINED__
#define __IAMVideoDecimationProperties_FWD_DEFINED__
typedef interface IAMVideoDecimationProperties IAMVideoDecimationProperties;

#endif 	/* __IAMVideoDecimationProperties_FWD_DEFINED__ */


#ifndef __IVideoFrameStep_FWD_DEFINED__
#define __IVideoFrameStep_FWD_DEFINED__
typedef interface IVideoFrameStep IVideoFrameStep;

#endif 	/* __IVideoFrameStep_FWD_DEFINED__ */


#ifndef __IAMLatency_FWD_DEFINED__
#define __IAMLatency_FWD_DEFINED__
typedef interface IAMLatency IAMLatency;

#endif 	/* __IAMLatency_FWD_DEFINED__ */


#ifndef __IAMPushSource_FWD_DEFINED__
#define __IAMPushSource_FWD_DEFINED__
typedef interface IAMPushSource IAMPushSource;

#endif 	/* __IAMPushSource_FWD_DEFINED__ */


#ifndef __IAMDeviceRemoval_FWD_DEFINED__
#define __IAMDeviceRemoval_FWD_DEFINED__
typedef interface IAMDeviceRemoval IAMDeviceRemoval;

#endif 	/* __IAMDeviceRemoval_FWD_DEFINED__ */


#ifndef __IDVEnc_FWD_DEFINED__
#define __IDVEnc_FWD_DEFINED__
typedef interface IDVEnc IDVEnc;

#endif 	/* __IDVEnc_FWD_DEFINED__ */


#ifndef __IIPDVDec_FWD_DEFINED__
#define __IIPDVDec_FWD_DEFINED__
typedef interface IIPDVDec IIPDVDec;

#endif 	/* __IIPDVDec_FWD_DEFINED__ */


#ifndef __IDVRGB219_FWD_DEFINED__
#define __IDVRGB219_FWD_DEFINED__
typedef interface IDVRGB219 IDVRGB219;

#endif 	/* __IDVRGB219_FWD_DEFINED__ */


#ifndef __IDVSplitter_FWD_DEFINED__
#define __IDVSplitter_FWD_DEFINED__
typedef interface IDVSplitter IDVSplitter;

#endif 	/* __IDVSplitter_FWD_DEFINED__ */


#ifndef __IAMAudioRendererStats_FWD_DEFINED__
#define __IAMAudioRendererStats_FWD_DEFINED__
typedef interface IAMAudioRendererStats IAMAudioRendererStats;

#endif 	/* __IAMAudioRendererStats_FWD_DEFINED__ */


#ifndef __IAMGraphStreams_FWD_DEFINED__
#define __IAMGraphStreams_FWD_DEFINED__
typedef interface IAMGraphStreams IAMGraphStreams;

#endif 	/* __IAMGraphStreams_FWD_DEFINED__ */


#ifndef __IAMOverlayFX_FWD_DEFINED__
#define __IAMOverlayFX_FWD_DEFINED__
typedef interface IAMOverlayFX IAMOverlayFX;

#endif 	/* __IAMOverlayFX_FWD_DEFINED__ */


#ifndef __IAMOpenProgress_FWD_DEFINED__
#define __IAMOpenProgress_FWD_DEFINED__
typedef interface IAMOpenProgress IAMOpenProgress;

#endif 	/* __IAMOpenProgress_FWD_DEFINED__ */


#ifndef __IMpeg2Demultiplexer_FWD_DEFINED__
#define __IMpeg2Demultiplexer_FWD_DEFINED__
typedef interface IMpeg2Demultiplexer IMpeg2Demultiplexer;

#endif 	/* __IMpeg2Demultiplexer_FWD_DEFINED__ */


#ifndef __IEnumStreamIdMap_FWD_DEFINED__
#define __IEnumStreamIdMap_FWD_DEFINED__
typedef interface IEnumStreamIdMap IEnumStreamIdMap;

#endif 	/* __IEnumStreamIdMap_FWD_DEFINED__ */


#ifndef __IMPEG2StreamIdMap_FWD_DEFINED__
#define __IMPEG2StreamIdMap_FWD_DEFINED__
typedef interface IMPEG2StreamIdMap IMPEG2StreamIdMap;

#endif 	/* __IMPEG2StreamIdMap_FWD_DEFINED__ */


#ifndef __IRegisterServiceProvider_FWD_DEFINED__
#define __IRegisterServiceProvider_FWD_DEFINED__
typedef interface IRegisterServiceProvider IRegisterServiceProvider;

#endif 	/* __IRegisterServiceProvider_FWD_DEFINED__ */


#ifndef __IAMClockSlave_FWD_DEFINED__
#define __IAMClockSlave_FWD_DEFINED__
typedef interface IAMClockSlave IAMClockSlave;

#endif 	/* __IAMClockSlave_FWD_DEFINED__ */


#ifndef __IAMGraphBuilderCallback_FWD_DEFINED__
#define __IAMGraphBuilderCallback_FWD_DEFINED__
typedef interface IAMGraphBuilderCallback IAMGraphBuilderCallback;

#endif 	/* __IAMGraphBuilderCallback_FWD_DEFINED__ */


#ifndef __IGetCapabilitiesKey_FWD_DEFINED__
#define __IGetCapabilitiesKey_FWD_DEFINED__
typedef interface IGetCapabilitiesKey IGetCapabilitiesKey;

#endif 	/* __IGetCapabilitiesKey_FWD_DEFINED__ */


#ifndef __IEncoderAPI_FWD_DEFINED__
#define __IEncoderAPI_FWD_DEFINED__
typedef interface IEncoderAPI IEncoderAPI;

#endif 	/* __IEncoderAPI_FWD_DEFINED__ */


#ifndef __IVideoEncoder_FWD_DEFINED__
#define __IVideoEncoder_FWD_DEFINED__
typedef interface IVideoEncoder IVideoEncoder;

#endif 	/* __IVideoEncoder_FWD_DEFINED__ */


#ifndef __IAMDecoderCaps_FWD_DEFINED__
#define __IAMDecoderCaps_FWD_DEFINED__
typedef interface IAMDecoderCaps IAMDecoderCaps;

#endif 	/* __IAMDecoderCaps_FWD_DEFINED__ */


#ifndef __IAMCertifiedOutputProtection_FWD_DEFINED__
#define __IAMCertifiedOutputProtection_FWD_DEFINED__
typedef interface IAMCertifiedOutputProtection IAMCertifiedOutputProtection;

#endif 	/* __IAMCertifiedOutputProtection_FWD_DEFINED__ */


#ifndef __IAMAsyncReaderTimestampScaling_FWD_DEFINED__
#define __IAMAsyncReaderTimestampScaling_FWD_DEFINED__
typedef interface IAMAsyncReaderTimestampScaling IAMAsyncReaderTimestampScaling;

#endif 	/* __IAMAsyncReaderTimestampScaling_FWD_DEFINED__ */


#ifndef __IAMPluginControl_FWD_DEFINED__
#define __IAMPluginControl_FWD_DEFINED__
typedef interface IAMPluginControl IAMPluginControl;

#endif 	/* __IAMPluginControl_FWD_DEFINED__ */


#ifndef __IPinConnection_FWD_DEFINED__
#define __IPinConnection_FWD_DEFINED__
typedef interface IPinConnection IPinConnection;

#endif 	/* __IPinConnection_FWD_DEFINED__ */


#ifndef __IPinFlowControl_FWD_DEFINED__
#define __IPinFlowControl_FWD_DEFINED__
typedef interface IPinFlowControl IPinFlowControl;

#endif 	/* __IPinFlowControl_FWD_DEFINED__ */


#ifndef __IGraphConfig_FWD_DEFINED__
#define __IGraphConfig_FWD_DEFINED__
typedef interface IGraphConfig IGraphConfig;

#endif 	/* __IGraphConfig_FWD_DEFINED__ */


#ifndef __IGraphConfigCallback_FWD_DEFINED__
#define __IGraphConfigCallback_FWD_DEFINED__
typedef interface IGraphConfigCallback IGraphConfigCallback;

#endif 	/* __IGraphConfigCallback_FWD_DEFINED__ */


#ifndef __IFilterChain_FWD_DEFINED__
#define __IFilterChain_FWD_DEFINED__
typedef interface IFilterChain IFilterChain;

#endif 	/* __IFilterChain_FWD_DEFINED__ */


#ifndef __IVMRImagePresenter_FWD_DEFINED__
#define __IVMRImagePresenter_FWD_DEFINED__
typedef interface IVMRImagePresenter IVMRImagePresenter;

#endif 	/* __IVMRImagePresenter_FWD_DEFINED__ */


#ifndef __IVMRSurfaceAllocator_FWD_DEFINED__
#define __IVMRSurfaceAllocator_FWD_DEFINED__
typedef interface IVMRSurfaceAllocator IVMRSurfaceAllocator;

#endif 	/* __IVMRSurfaceAllocator_FWD_DEFINED__ */


#ifndef __IVMRSurfaceAllocatorNotify_FWD_DEFINED__
#define __IVMRSurfaceAllocatorNotify_FWD_DEFINED__
typedef interface IVMRSurfaceAllocatorNotify IVMRSurfaceAllocatorNotify;

#endif 	/* __IVMRSurfaceAllocatorNotify_FWD_DEFINED__ */


#ifndef __IVMRWindowlessControl_FWD_DEFINED__
#define __IVMRWindowlessControl_FWD_DEFINED__
typedef interface IVMRWindowlessControl IVMRWindowlessControl;

#endif 	/* __IVMRWindowlessControl_FWD_DEFINED__ */


#ifndef __IVMRMixerControl_FWD_DEFINED__
#define __IVMRMixerControl_FWD_DEFINED__
typedef interface IVMRMixerControl IVMRMixerControl;

#endif 	/* __IVMRMixerControl_FWD_DEFINED__ */


#ifndef __IVMRMonitorConfig_FWD_DEFINED__
#define __IVMRMonitorConfig_FWD_DEFINED__
typedef interface IVMRMonitorConfig IVMRMonitorConfig;

#endif 	/* __IVMRMonitorConfig_FWD_DEFINED__ */


#ifndef __IVMRFilterConfig_FWD_DEFINED__
#define __IVMRFilterConfig_FWD_DEFINED__
typedef interface IVMRFilterConfig IVMRFilterConfig;

#endif 	/* __IVMRFilterConfig_FWD_DEFINED__ */


#ifndef __IVMRAspectRatioControl_FWD_DEFINED__
#define __IVMRAspectRatioControl_FWD_DEFINED__
typedef interface IVMRAspectRatioControl IVMRAspectRatioControl;

#endif 	/* __IVMRAspectRatioControl_FWD_DEFINED__ */


#ifndef __IVMRDeinterlaceControl_FWD_DEFINED__
#define __IVMRDeinterlaceControl_FWD_DEFINED__
typedef interface IVMRDeinterlaceControl IVMRDeinterlaceControl;

#endif 	/* __IVMRDeinterlaceControl_FWD_DEFINED__ */


#ifndef __IVMRMixerBitmap_FWD_DEFINED__
#define __IVMRMixerBitmap_FWD_DEFINED__
typedef interface IVMRMixerBitmap IVMRMixerBitmap;

#endif 	/* __IVMRMixerBitmap_FWD_DEFINED__ */


#ifndef __IVMRImageCompositor_FWD_DEFINED__
#define __IVMRImageCompositor_FWD_DEFINED__
typedef interface IVMRImageCompositor IVMRImageCompositor;

#endif 	/* __IVMRImageCompositor_FWD_DEFINED__ */


#ifndef __IVMRVideoStreamControl_FWD_DEFINED__
#define __IVMRVideoStreamControl_FWD_DEFINED__
typedef interface IVMRVideoStreamControl IVMRVideoStreamControl;

#endif 	/* __IVMRVideoStreamControl_FWD_DEFINED__ */


#ifndef __IVMRSurface_FWD_DEFINED__
#define __IVMRSurface_FWD_DEFINED__
typedef interface IVMRSurface IVMRSurface;

#endif 	/* __IVMRSurface_FWD_DEFINED__ */


#ifndef __IVMRImagePresenterConfig_FWD_DEFINED__
#define __IVMRImagePresenterConfig_FWD_DEFINED__
typedef interface IVMRImagePresenterConfig IVMRImagePresenterConfig;

#endif 	/* __IVMRImagePresenterConfig_FWD_DEFINED__ */


#ifndef __IVMRImagePresenterExclModeConfig_FWD_DEFINED__
#define __IVMRImagePresenterExclModeConfig_FWD_DEFINED__
typedef interface IVMRImagePresenterExclModeConfig IVMRImagePresenterExclModeConfig;

#endif 	/* __IVMRImagePresenterExclModeConfig_FWD_DEFINED__ */


#ifndef __IVPManager_FWD_DEFINED__
#define __IVPManager_FWD_DEFINED__
typedef interface IVPManager IVPManager;

#endif 	/* __IVPManager_FWD_DEFINED__ */


#ifndef __IDvdControl_FWD_DEFINED__
#define __IDvdControl_FWD_DEFINED__
typedef interface IDvdControl IDvdControl;

#endif 	/* __IDvdControl_FWD_DEFINED__ */


#ifndef __IDvdInfo_FWD_DEFINED__
#define __IDvdInfo_FWD_DEFINED__
typedef interface IDvdInfo IDvdInfo;

#endif 	/* __IDvdInfo_FWD_DEFINED__ */


#ifndef __IDvdCmd_FWD_DEFINED__
#define __IDvdCmd_FWD_DEFINED__
typedef interface IDvdCmd IDvdCmd;

#endif 	/* __IDvdCmd_FWD_DEFINED__ */


#ifndef __IDvdState_FWD_DEFINED__
#define __IDvdState_FWD_DEFINED__
typedef interface IDvdState IDvdState;

#endif 	/* __IDvdState_FWD_DEFINED__ */


#ifndef __IDvdControl2_FWD_DEFINED__
#define __IDvdControl2_FWD_DEFINED__
typedef interface IDvdControl2 IDvdControl2;

#endif 	/* __IDvdControl2_FWD_DEFINED__ */


#ifndef __IDvdInfo2_FWD_DEFINED__
#define __IDvdInfo2_FWD_DEFINED__
typedef interface IDvdInfo2 IDvdInfo2;

#endif 	/* __IDvdInfo2_FWD_DEFINED__ */


#ifndef __IDvdGraphBuilder_FWD_DEFINED__
#define __IDvdGraphBuilder_FWD_DEFINED__
typedef interface IDvdGraphBuilder IDvdGraphBuilder;

#endif 	/* __IDvdGraphBuilder_FWD_DEFINED__ */


#ifndef __IDDrawExclModeVideo_FWD_DEFINED__
#define __IDDrawExclModeVideo_FWD_DEFINED__
typedef interface IDDrawExclModeVideo IDDrawExclModeVideo;

#endif 	/* __IDDrawExclModeVideo_FWD_DEFINED__ */


#ifndef __IDDrawExclModeVideoCallback_FWD_DEFINED__
#define __IDDrawExclModeVideoCallback_FWD_DEFINED__
typedef interface IDDrawExclModeVideoCallback IDDrawExclModeVideoCallback;

#endif 	/* __IDDrawExclModeVideoCallback_FWD_DEFINED__ */


/* header files for imported files */
#include "unknwn.h"
#include "objidl.h"
#include "oaidl.h"
#include "ocidl.h"

#ifdef __cplusplus
extern "C"{
#endif 


/* interface __MIDL_itf_strmif_0000_0000 */
/* [local] */ 

//+-------------------------------------------------------------------------
//
//  Copyright (C) Microsoft Corporation, 1999-2002.
//
//--------------------------------------------------------------------------
#include <winapifamily.h>
// Disable /W4 compiler warning C4201: nameless struct/union
#pragma warning(push)
#pragma warning(disable:4201)  // Disable C4201: nameless struct/union
  
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define CDEF_CLASS_DEFAULT      0x0001
#define CDEF_BYPASS_CLASS_MANAGER   0x0002
#define CDEF_MERIT_ABOVE_DO_NOT_USE  0x0008
#define CDEF_DEVMON_CMGR_DEVICE  0x0010
#define CDEF_DEVMON_DMO  0x0020
#define CDEF_DEVMON_PNP_DEVICE  0x0040
#define CDEF_DEVMON_FILTER  0x0080
#define CDEF_DEVMON_SELECTIVE_MASK  0x00f0


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0000_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0000_v0_0_s_ifspec;

#ifndef __ICreateDevEnum_INTERFACE_DEFINED__
#define __ICreateDevEnum_INTERFACE_DEFINED__

/* interface ICreateDevEnum */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ICreateDevEnum;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("29840822-5B84-11D0-BD3B-00A0C911CE86")
    ICreateDevEnum : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateClassEnumerator( 
            /* [in] */ REFCLSID clsidDeviceClass,
            /* [annotation][out] */ 
            _Out_  IEnumMoniker **ppEnumMoniker,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICreateDevEnumVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICreateDevEnum * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICreateDevEnum * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICreateDevEnum * This);
        
        DECLSPEC_XFGVIRT(ICreateDevEnum, CreateClassEnumerator)
        HRESULT ( STDMETHODCALLTYPE *CreateClassEnumerator )( 
            ICreateDevEnum * This,
            /* [in] */ REFCLSID clsidDeviceClass,
            /* [annotation][out] */ 
            _Out_  IEnumMoniker **ppEnumMoniker,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } ICreateDevEnumVtbl;

    interface ICreateDevEnum
    {
        CONST_VTBL struct ICreateDevEnumVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICreateDevEnum_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICreateDevEnum_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICreateDevEnum_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICreateDevEnum_CreateClassEnumerator(This,clsidDeviceClass,ppEnumMoniker,dwFlags)	\
    ( (This)->lpVtbl -> CreateClassEnumerator(This,clsidDeviceClass,ppEnumMoniker,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICreateDevEnum_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0001 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define CHARS_IN_GUID     39
typedef struct _AMMediaType
    {
    GUID majortype;
    GUID subtype;
    BOOL bFixedSizeSamples;
    BOOL bTemporalCompression;
    ULONG lSampleSize;
    GUID formattype;
    IUnknown *pUnk;
    ULONG cbFormat;
    /* [size_is] */ BYTE *pbFormat;
    } 	AM_MEDIA_TYPE;

typedef 
enum _PinDirection
    {
        PINDIR_INPUT	= 0,
        PINDIR_OUTPUT	= ( PINDIR_INPUT + 1 ) 
    } 	PIN_DIRECTION;

#define MAX_PIN_NAME     128
#define MAX_FILTER_NAME  128
typedef LONGLONG REFERENCE_TIME;

typedef double REFTIME;

typedef DWORD_PTR HSEMAPHORE;

typedef DWORD_PTR HEVENT;

typedef struct _AllocatorProperties
    {
    long cBuffers;
    long cbBuffer;
    long cbAlign;
    long cbPrefix;
    } 	ALLOCATOR_PROPERTIES;

















extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0001_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0001_v0_0_s_ifspec;

#ifndef __IPin_INTERFACE_DEFINED__
#define __IPin_INTERFACE_DEFINED__

/* interface IPin */
/* [unique][uuid][object][local] */ 

typedef struct _PinInfo
    {
    IBaseFilter *pFilter;
    PIN_DIRECTION dir;
    WCHAR achName[ 128 ];
    } 	PIN_INFO;


EXTERN_C const IID IID_IPin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a86891-0ad4-11ce-b03a-0020af0ba770")
    IPin : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ IPin *pReceivePin,
            /* [annotation][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReceiveConnection( 
            /* [in] */ IPin *pConnector,
            /* [in] */ const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectedTo( 
            /* [annotation][out] */ 
            _Out_  IPin **pPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectionMediaType( 
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryPinInfo( 
            /* [annotation][out] */ 
            _Out_  PIN_INFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryDirection( 
            /* [annotation][out] */ 
            _Out_  PIN_DIRECTION *pPinDir) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryId( 
            /* [annotation][out] */ 
            _Out_  LPWSTR *Id) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryAccept( 
            /* [in] */ const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMediaTypes( 
            /* [annotation][out] */ 
            _Out_  IEnumMediaTypes **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryInternalConnections( 
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*nPin, *nPin)  IPin **apPin,
            /* [out][in] */ ULONG *nPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndOfStream( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginFlush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndFlush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NewSegment( 
            /* [in] */ REFERENCE_TIME tStart,
            /* [in] */ REFERENCE_TIME tStop,
            /* [in] */ double dRate) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPinVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPin * This);
        
        DECLSPEC_XFGVIRT(IPin, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IPin * This,
            /* [in] */ IPin *pReceivePin,
            /* [annotation][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IPin, ReceiveConnection)
        HRESULT ( STDMETHODCALLTYPE *ReceiveConnection )( 
            IPin * This,
            /* [in] */ IPin *pConnector,
            /* [in] */ const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IPin, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IPin * This);
        
        DECLSPEC_XFGVIRT(IPin, ConnectedTo)
        HRESULT ( STDMETHODCALLTYPE *ConnectedTo )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_  IPin **pPin);
        
        DECLSPEC_XFGVIRT(IPin, ConnectionMediaType)
        HRESULT ( STDMETHODCALLTYPE *ConnectionMediaType )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IPin, QueryPinInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryPinInfo )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_  PIN_INFO *pInfo);
        
        DECLSPEC_XFGVIRT(IPin, QueryDirection)
        HRESULT ( STDMETHODCALLTYPE *QueryDirection )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_  PIN_DIRECTION *pPinDir);
        
        DECLSPEC_XFGVIRT(IPin, QueryId)
        HRESULT ( STDMETHODCALLTYPE *QueryId )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_  LPWSTR *Id);
        
        DECLSPEC_XFGVIRT(IPin, QueryAccept)
        HRESULT ( STDMETHODCALLTYPE *QueryAccept )( 
            IPin * This,
            /* [in] */ const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IPin, EnumMediaTypes)
        HRESULT ( STDMETHODCALLTYPE *EnumMediaTypes )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_  IEnumMediaTypes **ppEnum);
        
        DECLSPEC_XFGVIRT(IPin, QueryInternalConnections)
        HRESULT ( STDMETHODCALLTYPE *QueryInternalConnections )( 
            IPin * This,
            /* [annotation][out] */ 
            _Out_writes_to_opt_(*nPin, *nPin)  IPin **apPin,
            /* [out][in] */ ULONG *nPin);
        
        DECLSPEC_XFGVIRT(IPin, EndOfStream)
        HRESULT ( STDMETHODCALLTYPE *EndOfStream )( 
            IPin * This);
        
        DECLSPEC_XFGVIRT(IPin, BeginFlush)
        HRESULT ( STDMETHODCALLTYPE *BeginFlush )( 
            IPin * This);
        
        DECLSPEC_XFGVIRT(IPin, EndFlush)
        HRESULT ( STDMETHODCALLTYPE *EndFlush )( 
            IPin * This);
        
        DECLSPEC_XFGVIRT(IPin, NewSegment)
        HRESULT ( STDMETHODCALLTYPE *NewSegment )( 
            IPin * This,
            /* [in] */ REFERENCE_TIME tStart,
            /* [in] */ REFERENCE_TIME tStop,
            /* [in] */ double dRate);
        
        END_INTERFACE
    } IPinVtbl;

    interface IPin
    {
        CONST_VTBL struct IPinVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPin_Connect(This,pReceivePin,pmt)	\
    ( (This)->lpVtbl -> Connect(This,pReceivePin,pmt) ) 

#define IPin_ReceiveConnection(This,pConnector,pmt)	\
    ( (This)->lpVtbl -> ReceiveConnection(This,pConnector,pmt) ) 

#define IPin_Disconnect(This)	\
    ( (This)->lpVtbl -> Disconnect(This) ) 

#define IPin_ConnectedTo(This,pPin)	\
    ( (This)->lpVtbl -> ConnectedTo(This,pPin) ) 

#define IPin_ConnectionMediaType(This,pmt)	\
    ( (This)->lpVtbl -> ConnectionMediaType(This,pmt) ) 

#define IPin_QueryPinInfo(This,pInfo)	\
    ( (This)->lpVtbl -> QueryPinInfo(This,pInfo) ) 

#define IPin_QueryDirection(This,pPinDir)	\
    ( (This)->lpVtbl -> QueryDirection(This,pPinDir) ) 

#define IPin_QueryId(This,Id)	\
    ( (This)->lpVtbl -> QueryId(This,Id) ) 

#define IPin_QueryAccept(This,pmt)	\
    ( (This)->lpVtbl -> QueryAccept(This,pmt) ) 

#define IPin_EnumMediaTypes(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumMediaTypes(This,ppEnum) ) 

#define IPin_QueryInternalConnections(This,apPin,nPin)	\
    ( (This)->lpVtbl -> QueryInternalConnections(This,apPin,nPin) ) 

#define IPin_EndOfStream(This)	\
    ( (This)->lpVtbl -> EndOfStream(This) ) 

#define IPin_BeginFlush(This)	\
    ( (This)->lpVtbl -> BeginFlush(This) ) 

#define IPin_EndFlush(This)	\
    ( (This)->lpVtbl -> EndFlush(This) ) 

#define IPin_NewSegment(This,tStart,tStop,dRate)	\
    ( (This)->lpVtbl -> NewSegment(This,tStart,tStop,dRate) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0002 */
/* [local] */ 

typedef IPin *PPIN;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0002_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0002_v0_0_s_ifspec;

#ifndef __IEnumPins_INTERFACE_DEFINED__
#define __IEnumPins_INTERFACE_DEFINED__

/* interface IEnumPins */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumPins;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a86892-0ad4-11ce-b03a-0020af0ba770")
    IEnumPins : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cPins,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cPins, *pcFetched)  IPin **ppPins,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cPins) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IEnumPins **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumPinsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumPins * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumPins * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumPins * This);
        
        DECLSPEC_XFGVIRT(IEnumPins, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumPins * This,
            /* [in] */ ULONG cPins,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cPins, *pcFetched)  IPin **ppPins,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumPins, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumPins * This,
            /* [in] */ ULONG cPins);
        
        DECLSPEC_XFGVIRT(IEnumPins, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumPins * This);
        
        DECLSPEC_XFGVIRT(IEnumPins, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumPins * This,
            /* [annotation][out] */ 
            _Out_  IEnumPins **ppEnum);
        
        END_INTERFACE
    } IEnumPinsVtbl;

    interface IEnumPins
    {
        CONST_VTBL struct IEnumPinsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumPins_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumPins_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumPins_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumPins_Next(This,cPins,ppPins,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cPins,ppPins,pcFetched) ) 

#define IEnumPins_Skip(This,cPins)	\
    ( (This)->lpVtbl -> Skip(This,cPins) ) 

#define IEnumPins_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumPins_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumPins_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0003 */
/* [local] */ 

typedef IEnumPins *PENUMPINS;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0003_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0003_v0_0_s_ifspec;

#ifndef __IEnumMediaTypes_INTERFACE_DEFINED__
#define __IEnumMediaTypes_INTERFACE_DEFINED__

/* interface IEnumMediaTypes */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumMediaTypes;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("89c31040-846b-11ce-97d3-00aa0055595a")
    IEnumMediaTypes : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cMediaTypes,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cMediaTypes, *pcFetched)  AM_MEDIA_TYPE **ppMediaTypes,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cMediaTypes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IEnumMediaTypes **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumMediaTypesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumMediaTypes * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumMediaTypes * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumMediaTypes * This);
        
        DECLSPEC_XFGVIRT(IEnumMediaTypes, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumMediaTypes * This,
            /* [in] */ ULONG cMediaTypes,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(cMediaTypes, *pcFetched)  AM_MEDIA_TYPE **ppMediaTypes,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumMediaTypes, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumMediaTypes * This,
            /* [in] */ ULONG cMediaTypes);
        
        DECLSPEC_XFGVIRT(IEnumMediaTypes, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumMediaTypes * This);
        
        DECLSPEC_XFGVIRT(IEnumMediaTypes, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumMediaTypes * This,
            /* [annotation][out] */ 
            _Out_  IEnumMediaTypes **ppEnum);
        
        END_INTERFACE
    } IEnumMediaTypesVtbl;

    interface IEnumMediaTypes
    {
        CONST_VTBL struct IEnumMediaTypesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumMediaTypes_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumMediaTypes_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumMediaTypes_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumMediaTypes_Next(This,cMediaTypes,ppMediaTypes,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cMediaTypes,ppMediaTypes,pcFetched) ) 

#define IEnumMediaTypes_Skip(This,cMediaTypes)	\
    ( (This)->lpVtbl -> Skip(This,cMediaTypes) ) 

#define IEnumMediaTypes_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumMediaTypes_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumMediaTypes_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0004 */
/* [local] */ 

typedef IEnumMediaTypes *PENUMMEDIATYPES;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0004_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0004_v0_0_s_ifspec;

#ifndef __IFilterGraph_INTERFACE_DEFINED__
#define __IFilterGraph_INTERFACE_DEFINED__

/* interface IFilterGraph */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFilterGraph;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a8689f-0ad4-11ce-b03a-0020af0ba770")
    IFilterGraph : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddFilter( 
            /* [in] */ IBaseFilter *pFilter,
            /* [string][in] */ LPCWSTR pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveFilter( 
            /* [in] */ IBaseFilter *pFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumFilters( 
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindFilterByName( 
            /* [string][in] */ LPCWSTR pName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConnectDirect( 
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reconnect( 
            /* [in] */ IPin *ppin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disconnect( 
            /* [in] */ IPin *ppin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultSyncSource( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterGraphVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterGraph * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterGraph * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterGraph * This);
        
        DECLSPEC_XFGVIRT(IFilterGraph, AddFilter)
        HRESULT ( STDMETHODCALLTYPE *AddFilter )( 
            IFilterGraph * This,
            /* [in] */ IBaseFilter *pFilter,
            /* [string][in] */ LPCWSTR pName);
        
        DECLSPEC_XFGVIRT(IFilterGraph, RemoveFilter)
        HRESULT ( STDMETHODCALLTYPE *RemoveFilter )( 
            IFilterGraph * This,
            /* [in] */ IBaseFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, EnumFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumFilters )( 
            IFilterGraph * This,
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum);
        
        DECLSPEC_XFGVIRT(IFilterGraph, FindFilterByName)
        HRESULT ( STDMETHODCALLTYPE *FindFilterByName )( 
            IFilterGraph * This,
            /* [string][in] */ LPCWSTR pName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, ConnectDirect)
        HRESULT ( STDMETHODCALLTYPE *ConnectDirect )( 
            IFilterGraph * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Reconnect)
        HRESULT ( STDMETHODCALLTYPE *Reconnect )( 
            IFilterGraph * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IFilterGraph * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, SetDefaultSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSyncSource )( 
            IFilterGraph * This);
        
        END_INTERFACE
    } IFilterGraphVtbl;

    interface IFilterGraph
    {
        CONST_VTBL struct IFilterGraphVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterGraph_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterGraph_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterGraph_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterGraph_AddFilter(This,pFilter,pName)	\
    ( (This)->lpVtbl -> AddFilter(This,pFilter,pName) ) 

#define IFilterGraph_RemoveFilter(This,pFilter)	\
    ( (This)->lpVtbl -> RemoveFilter(This,pFilter) ) 

#define IFilterGraph_EnumFilters(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFilters(This,ppEnum) ) 

#define IFilterGraph_FindFilterByName(This,pName,ppFilter)	\
    ( (This)->lpVtbl -> FindFilterByName(This,pName,ppFilter) ) 

#define IFilterGraph_ConnectDirect(This,ppinOut,ppinIn,pmt)	\
    ( (This)->lpVtbl -> ConnectDirect(This,ppinOut,ppinIn,pmt) ) 

#define IFilterGraph_Reconnect(This,ppin)	\
    ( (This)->lpVtbl -> Reconnect(This,ppin) ) 

#define IFilterGraph_Disconnect(This,ppin)	\
    ( (This)->lpVtbl -> Disconnect(This,ppin) ) 

#define IFilterGraph_SetDefaultSyncSource(This)	\
    ( (This)->lpVtbl -> SetDefaultSyncSource(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterGraph_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0005 */
/* [local] */ 

typedef IFilterGraph *PFILTERGRAPH;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0005_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0005_v0_0_s_ifspec;

#ifndef __IEnumFilters_INTERFACE_DEFINED__
#define __IEnumFilters_INTERFACE_DEFINED__

/* interface IEnumFilters */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumFilters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a86893-0ad4-11ce-b03a-0020af0ba770")
    IEnumFilters : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cFilters,
            /* [annotation][out] */ 
            _Out_writes_to_(cFilters, *pcFetched)  IBaseFilter **ppFilter,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cFilters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumFiltersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumFilters * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumFilters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumFilters * This);
        
        DECLSPEC_XFGVIRT(IEnumFilters, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumFilters * This,
            /* [in] */ ULONG cFilters,
            /* [annotation][out] */ 
            _Out_writes_to_(cFilters, *pcFetched)  IBaseFilter **ppFilter,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumFilters, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumFilters * This,
            /* [in] */ ULONG cFilters);
        
        DECLSPEC_XFGVIRT(IEnumFilters, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumFilters * This);
        
        DECLSPEC_XFGVIRT(IEnumFilters, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumFilters * This,
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum);
        
        END_INTERFACE
    } IEnumFiltersVtbl;

    interface IEnumFilters
    {
        CONST_VTBL struct IEnumFiltersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumFilters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumFilters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumFilters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumFilters_Next(This,cFilters,ppFilter,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cFilters,ppFilter,pcFetched) ) 

#define IEnumFilters_Skip(This,cFilters)	\
    ( (This)->lpVtbl -> Skip(This,cFilters) ) 

#define IEnumFilters_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumFilters_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumFilters_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0006 */
/* [local] */ 

typedef IEnumFilters *PENUMFILTERS;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0006_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0006_v0_0_s_ifspec;

#ifndef __IMediaFilter_INTERFACE_DEFINED__
#define __IMediaFilter_INTERFACE_DEFINED__

/* interface IMediaFilter */
/* [unique][uuid][object][local] */ 

typedef 
enum _FilterState
    {
        State_Stopped	= 0,
        State_Paused	= ( State_Stopped + 1 ) ,
        State_Running	= ( State_Paused + 1 ) 
    } 	FILTER_STATE;


EXTERN_C const IID IID_IMediaFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a86899-0ad4-11ce-b03a-0020af0ba770")
    IMediaFilter : public IPersist
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Run( 
            REFERENCE_TIME tStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [in] */ DWORD dwMilliSecsTimeout,
            /* [annotation][out] */ 
            _Out_  FILTER_STATE *State) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncSource( 
            /* [annotation][in] */ 
            _In_opt_  IReferenceClock *pClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSyncSource( 
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IReferenceClock **pClock) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaFilter * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            IMediaFilter * This,
            /* [out] */ CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IMediaFilter * This);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IMediaFilter * This);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IMediaFilter * This,
            REFERENCE_TIME tStart);
        
        DECLSPEC_XFGVIRT(IMediaFilter, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IMediaFilter * This,
            /* [in] */ DWORD dwMilliSecsTimeout,
            /* [annotation][out] */ 
            _Out_  FILTER_STATE *State);
        
        DECLSPEC_XFGVIRT(IMediaFilter, SetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSource )( 
            IMediaFilter * This,
            /* [annotation][in] */ 
            _In_opt_  IReferenceClock *pClock);
        
        DECLSPEC_XFGVIRT(IMediaFilter, GetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *GetSyncSource )( 
            IMediaFilter * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IReferenceClock **pClock);
        
        END_INTERFACE
    } IMediaFilterVtbl;

    interface IMediaFilter
    {
        CONST_VTBL struct IMediaFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaFilter_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IMediaFilter_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IMediaFilter_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IMediaFilter_Run(This,tStart)	\
    ( (This)->lpVtbl -> Run(This,tStart) ) 

#define IMediaFilter_GetState(This,dwMilliSecsTimeout,State)	\
    ( (This)->lpVtbl -> GetState(This,dwMilliSecsTimeout,State) ) 

#define IMediaFilter_SetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> SetSyncSource(This,pClock) ) 

#define IMediaFilter_GetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> GetSyncSource(This,pClock) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0007 */
/* [local] */ 

typedef IMediaFilter *PMEDIAFILTER;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0007_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0007_v0_0_s_ifspec;

#ifndef __IBaseFilter_INTERFACE_DEFINED__
#define __IBaseFilter_INTERFACE_DEFINED__

/* interface IBaseFilter */
/* [unique][uuid][object][local] */ 

typedef struct _FilterInfo
    {
    WCHAR achName[ 128 ];
    IFilterGraph *pGraph;
    } 	FILTER_INFO;


EXTERN_C const IID IID_IBaseFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a86895-0ad4-11ce-b03a-0020af0ba770")
    IBaseFilter : public IMediaFilter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumPins( 
            /* [annotation][out] */ 
            _Out_  IEnumPins **ppEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindPin( 
            /* [string][in] */ LPCWSTR Id,
            /* [annotation][out] */ 
            _Out_  IPin **ppPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryFilterInfo( 
            /* [annotation][out] */ 
            _Out_  FILTER_INFO *pInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE JoinFilterGraph( 
            /* [annotation][in] */ 
            _In_opt_  IFilterGraph *pGraph,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pName) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryVendorInfo( 
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pVendorInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBaseFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBaseFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBaseFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBaseFilter * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            IBaseFilter * This,
            /* [out] */ CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IBaseFilter * This);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IBaseFilter * This);
        
        DECLSPEC_XFGVIRT(IMediaFilter, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IBaseFilter * This,
            REFERENCE_TIME tStart);
        
        DECLSPEC_XFGVIRT(IMediaFilter, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IBaseFilter * This,
            /* [in] */ DWORD dwMilliSecsTimeout,
            /* [annotation][out] */ 
            _Out_  FILTER_STATE *State);
        
        DECLSPEC_XFGVIRT(IMediaFilter, SetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSource )( 
            IBaseFilter * This,
            /* [annotation][in] */ 
            _In_opt_  IReferenceClock *pClock);
        
        DECLSPEC_XFGVIRT(IMediaFilter, GetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *GetSyncSource )( 
            IBaseFilter * This,
            /* [annotation][out] */ 
            _Outptr_result_maybenull_  IReferenceClock **pClock);
        
        DECLSPEC_XFGVIRT(IBaseFilter, EnumPins)
        HRESULT ( STDMETHODCALLTYPE *EnumPins )( 
            IBaseFilter * This,
            /* [annotation][out] */ 
            _Out_  IEnumPins **ppEnum);
        
        DECLSPEC_XFGVIRT(IBaseFilter, FindPin)
        HRESULT ( STDMETHODCALLTYPE *FindPin )( 
            IBaseFilter * This,
            /* [string][in] */ LPCWSTR Id,
            /* [annotation][out] */ 
            _Out_  IPin **ppPin);
        
        DECLSPEC_XFGVIRT(IBaseFilter, QueryFilterInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryFilterInfo )( 
            IBaseFilter * This,
            /* [annotation][out] */ 
            _Out_  FILTER_INFO *pInfo);
        
        DECLSPEC_XFGVIRT(IBaseFilter, JoinFilterGraph)
        HRESULT ( STDMETHODCALLTYPE *JoinFilterGraph )( 
            IBaseFilter * This,
            /* [annotation][in] */ 
            _In_opt_  IFilterGraph *pGraph,
            /* [annotation][string][in] */ 
            _In_opt_  LPCWSTR pName);
        
        DECLSPEC_XFGVIRT(IBaseFilter, QueryVendorInfo)
        HRESULT ( STDMETHODCALLTYPE *QueryVendorInfo )( 
            IBaseFilter * This,
            /* [annotation][string][out] */ 
            _Out_  LPWSTR *pVendorInfo);
        
        END_INTERFACE
    } IBaseFilterVtbl;

    interface IBaseFilter
    {
        CONST_VTBL struct IBaseFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBaseFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBaseFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBaseFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBaseFilter_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IBaseFilter_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IBaseFilter_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IBaseFilter_Run(This,tStart)	\
    ( (This)->lpVtbl -> Run(This,tStart) ) 

#define IBaseFilter_GetState(This,dwMilliSecsTimeout,State)	\
    ( (This)->lpVtbl -> GetState(This,dwMilliSecsTimeout,State) ) 

#define IBaseFilter_SetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> SetSyncSource(This,pClock) ) 

#define IBaseFilter_GetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> GetSyncSource(This,pClock) ) 


#define IBaseFilter_EnumPins(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumPins(This,ppEnum) ) 

#define IBaseFilter_FindPin(This,Id,ppPin)	\
    ( (This)->lpVtbl -> FindPin(This,Id,ppPin) ) 

#define IBaseFilter_QueryFilterInfo(This,pInfo)	\
    ( (This)->lpVtbl -> QueryFilterInfo(This,pInfo) ) 

#define IBaseFilter_JoinFilterGraph(This,pGraph,pName)	\
    ( (This)->lpVtbl -> JoinFilterGraph(This,pGraph,pName) ) 

#define IBaseFilter_QueryVendorInfo(This,pVendorInfo)	\
    ( (This)->lpVtbl -> QueryVendorInfo(This,pVendorInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBaseFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0008 */
/* [local] */ 

typedef IBaseFilter *PFILTER;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0008_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0008_v0_0_s_ifspec;

#ifndef __IReferenceClock_INTERFACE_DEFINED__
#define __IReferenceClock_INTERFACE_DEFINED__

/* interface IReferenceClock */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IReferenceClock;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a86897-0ad4-11ce-b03a-0020af0ba770")
    IReferenceClock : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTime( 
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AdviseTime( 
            /* [in] */ REFERENCE_TIME baseTime,
            /* [in] */ REFERENCE_TIME streamTime,
            /* [in] */ HEVENT hEvent,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwAdviseCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AdvisePeriodic( 
            /* [in] */ REFERENCE_TIME startTime,
            /* [in] */ REFERENCE_TIME periodTime,
            /* [in] */ HSEMAPHORE hSemaphore,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwAdviseCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( 
            /* [in] */ DWORD_PTR dwAdviseCookie) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IReferenceClockVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IReferenceClock * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IReferenceClock * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IReferenceClock * This);
        
        DECLSPEC_XFGVIRT(IReferenceClock, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IReferenceClock * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTime);
        
        DECLSPEC_XFGVIRT(IReferenceClock, AdviseTime)
        HRESULT ( STDMETHODCALLTYPE *AdviseTime )( 
            IReferenceClock * This,
            /* [in] */ REFERENCE_TIME baseTime,
            /* [in] */ REFERENCE_TIME streamTime,
            /* [in] */ HEVENT hEvent,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwAdviseCookie);
        
        DECLSPEC_XFGVIRT(IReferenceClock, AdvisePeriodic)
        HRESULT ( STDMETHODCALLTYPE *AdvisePeriodic )( 
            IReferenceClock * This,
            /* [in] */ REFERENCE_TIME startTime,
            /* [in] */ REFERENCE_TIME periodTime,
            /* [in] */ HSEMAPHORE hSemaphore,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwAdviseCookie);
        
        DECLSPEC_XFGVIRT(IReferenceClock, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            IReferenceClock * This,
            /* [in] */ DWORD_PTR dwAdviseCookie);
        
        END_INTERFACE
    } IReferenceClockVtbl;

    interface IReferenceClock
    {
        CONST_VTBL struct IReferenceClockVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReferenceClock_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReferenceClock_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReferenceClock_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReferenceClock_GetTime(This,pTime)	\
    ( (This)->lpVtbl -> GetTime(This,pTime) ) 

#define IReferenceClock_AdviseTime(This,baseTime,streamTime,hEvent,pdwAdviseCookie)	\
    ( (This)->lpVtbl -> AdviseTime(This,baseTime,streamTime,hEvent,pdwAdviseCookie) ) 

#define IReferenceClock_AdvisePeriodic(This,startTime,periodTime,hSemaphore,pdwAdviseCookie)	\
    ( (This)->lpVtbl -> AdvisePeriodic(This,startTime,periodTime,hSemaphore,pdwAdviseCookie) ) 

#define IReferenceClock_Unadvise(This,dwAdviseCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,dwAdviseCookie) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReferenceClock_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0009 */
/* [local] */ 

typedef IReferenceClock *PREFERENCECLOCK;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0009_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0009_v0_0_s_ifspec;

#ifndef __IReferenceClockTimerControl_INTERFACE_DEFINED__
#define __IReferenceClockTimerControl_INTERFACE_DEFINED__

/* interface IReferenceClockTimerControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IReferenceClockTimerControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ebec459c-2eca-4d42-a8af-30df557614b8")
    IReferenceClockTimerControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDefaultTimerResolution( 
            REFERENCE_TIME timerResolution) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultTimerResolution( 
            /* [annotation] */ 
            _Out_  REFERENCE_TIME *pTimerResolution) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IReferenceClockTimerControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IReferenceClockTimerControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IReferenceClockTimerControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IReferenceClockTimerControl * This);
        
        DECLSPEC_XFGVIRT(IReferenceClockTimerControl, SetDefaultTimerResolution)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultTimerResolution )( 
            IReferenceClockTimerControl * This,
            REFERENCE_TIME timerResolution);
        
        DECLSPEC_XFGVIRT(IReferenceClockTimerControl, GetDefaultTimerResolution)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultTimerResolution )( 
            IReferenceClockTimerControl * This,
            /* [annotation] */ 
            _Out_  REFERENCE_TIME *pTimerResolution);
        
        END_INTERFACE
    } IReferenceClockTimerControlVtbl;

    interface IReferenceClockTimerControl
    {
        CONST_VTBL struct IReferenceClockTimerControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReferenceClockTimerControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReferenceClockTimerControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReferenceClockTimerControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReferenceClockTimerControl_SetDefaultTimerResolution(This,timerResolution)	\
    ( (This)->lpVtbl -> SetDefaultTimerResolution(This,timerResolution) ) 

#define IReferenceClockTimerControl_GetDefaultTimerResolution(This,pTimerResolution)	\
    ( (This)->lpVtbl -> GetDefaultTimerResolution(This,pTimerResolution) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReferenceClockTimerControl_INTERFACE_DEFINED__ */


#ifndef __IReferenceClock2_INTERFACE_DEFINED__
#define __IReferenceClock2_INTERFACE_DEFINED__

/* interface IReferenceClock2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IReferenceClock2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36b73885-c2c8-11cf-8b46-00805f6cef60")
    IReferenceClock2 : public IReferenceClock
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IReferenceClock2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IReferenceClock2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IReferenceClock2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IReferenceClock2 * This);
        
        DECLSPEC_XFGVIRT(IReferenceClock, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IReferenceClock2 * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTime);
        
        DECLSPEC_XFGVIRT(IReferenceClock, AdviseTime)
        HRESULT ( STDMETHODCALLTYPE *AdviseTime )( 
            IReferenceClock2 * This,
            /* [in] */ REFERENCE_TIME baseTime,
            /* [in] */ REFERENCE_TIME streamTime,
            /* [in] */ HEVENT hEvent,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwAdviseCookie);
        
        DECLSPEC_XFGVIRT(IReferenceClock, AdvisePeriodic)
        HRESULT ( STDMETHODCALLTYPE *AdvisePeriodic )( 
            IReferenceClock2 * This,
            /* [in] */ REFERENCE_TIME startTime,
            /* [in] */ REFERENCE_TIME periodTime,
            /* [in] */ HSEMAPHORE hSemaphore,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwAdviseCookie);
        
        DECLSPEC_XFGVIRT(IReferenceClock, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            IReferenceClock2 * This,
            /* [in] */ DWORD_PTR dwAdviseCookie);
        
        END_INTERFACE
    } IReferenceClock2Vtbl;

    interface IReferenceClock2
    {
        CONST_VTBL struct IReferenceClock2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IReferenceClock2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IReferenceClock2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IReferenceClock2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IReferenceClock2_GetTime(This,pTime)	\
    ( (This)->lpVtbl -> GetTime(This,pTime) ) 

#define IReferenceClock2_AdviseTime(This,baseTime,streamTime,hEvent,pdwAdviseCookie)	\
    ( (This)->lpVtbl -> AdviseTime(This,baseTime,streamTime,hEvent,pdwAdviseCookie) ) 

#define IReferenceClock2_AdvisePeriodic(This,startTime,periodTime,hSemaphore,pdwAdviseCookie)	\
    ( (This)->lpVtbl -> AdvisePeriodic(This,startTime,periodTime,hSemaphore,pdwAdviseCookie) ) 

#define IReferenceClock2_Unadvise(This,dwAdviseCookie)	\
    ( (This)->lpVtbl -> Unadvise(This,dwAdviseCookie) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IReferenceClock2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0011 */
/* [local] */ 

typedef IReferenceClock2 *PREFERENCECLOCK2;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0011_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0011_v0_0_s_ifspec;

#ifndef __IMediaSample_INTERFACE_DEFINED__
#define __IMediaSample_INTERFACE_DEFINED__

/* interface IMediaSample */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMediaSample;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a8689a-0ad4-11ce-b03a-0020af0ba770")
    IMediaSample : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPointer( 
            /* [annotation][out] */ 
            _Outptr_result_buffer_to_(_Inexpressible_(this->GetSize()), _Inexpressible_(this->GetActualDataLength()))  BYTE **ppBuffer) = 0;
        
        virtual long STDMETHODCALLTYPE GetSize( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTime( 
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTimeStart,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTime( 
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pTimeStart,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSyncPoint( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncPoint( 
            BOOL bIsSyncPoint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsPreroll( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreroll( 
            BOOL bIsPreroll) = 0;
        
        virtual long STDMETHODCALLTYPE GetActualDataLength( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetActualDataLength( 
            long __MIDL__IMediaSample0000) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaType( 
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaType( 
            /* [annotation][in] */ 
            _In_  AM_MEDIA_TYPE *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDiscontinuity( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDiscontinuity( 
            BOOL bDiscontinuity) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMediaTime( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTimeStart,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTimeEnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMediaTime( 
            /* [annotation][in] */ 
            _In_opt_  LONGLONG *pTimeStart,
            /* [annotation][in] */ 
            _In_opt_  LONGLONG *pTimeEnd) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaSampleVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaSample * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetPointer)
        HRESULT ( STDMETHODCALLTYPE *GetPointer )( 
            IMediaSample * This,
            /* [annotation][out] */ 
            _Outptr_result_buffer_to_(_Inexpressible_(this->GetSize()), _Inexpressible_(this->GetActualDataLength()))  BYTE **ppBuffer);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetSize)
        long ( STDMETHODCALLTYPE *GetSize )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IMediaSample * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTimeStart,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetTime)
        HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            IMediaSample * This,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pTimeStart,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample, IsSyncPoint)
        HRESULT ( STDMETHODCALLTYPE *IsSyncPoint )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetSyncPoint)
        HRESULT ( STDMETHODCALLTYPE *SetSyncPoint )( 
            IMediaSample * This,
            BOOL bIsSyncPoint);
        
        DECLSPEC_XFGVIRT(IMediaSample, IsPreroll)
        HRESULT ( STDMETHODCALLTYPE *IsPreroll )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetPreroll)
        HRESULT ( STDMETHODCALLTYPE *SetPreroll )( 
            IMediaSample * This,
            BOOL bIsPreroll);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetActualDataLength)
        long ( STDMETHODCALLTYPE *GetActualDataLength )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetActualDataLength)
        HRESULT ( STDMETHODCALLTYPE *SetActualDataLength )( 
            IMediaSample * This,
            long __MIDL__IMediaSample0000);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetMediaType )( 
            IMediaSample * This,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            IMediaSample * This,
            /* [annotation][in] */ 
            _In_  AM_MEDIA_TYPE *pMediaType);
        
        DECLSPEC_XFGVIRT(IMediaSample, IsDiscontinuity)
        HRESULT ( STDMETHODCALLTYPE *IsDiscontinuity )( 
            IMediaSample * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetDiscontinuity)
        HRESULT ( STDMETHODCALLTYPE *SetDiscontinuity )( 
            IMediaSample * This,
            BOOL bDiscontinuity);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetMediaTime)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTime )( 
            IMediaSample * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTimeStart,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetMediaTime)
        HRESULT ( STDMETHODCALLTYPE *SetMediaTime )( 
            IMediaSample * This,
            /* [annotation][in] */ 
            _In_opt_  LONGLONG *pTimeStart,
            /* [annotation][in] */ 
            _In_opt_  LONGLONG *pTimeEnd);
        
        END_INTERFACE
    } IMediaSampleVtbl;

    interface IMediaSample
    {
        CONST_VTBL struct IMediaSampleVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaSample_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaSample_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaSample_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaSample_GetPointer(This,ppBuffer)	\
    ( (This)->lpVtbl -> GetPointer(This,ppBuffer) ) 

#define IMediaSample_GetSize(This)	\
    ( (This)->lpVtbl -> GetSize(This) ) 

#define IMediaSample_GetTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetTime(This,pTimeStart,pTimeEnd) ) 

#define IMediaSample_SetTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> SetTime(This,pTimeStart,pTimeEnd) ) 

#define IMediaSample_IsSyncPoint(This)	\
    ( (This)->lpVtbl -> IsSyncPoint(This) ) 

#define IMediaSample_SetSyncPoint(This,bIsSyncPoint)	\
    ( (This)->lpVtbl -> SetSyncPoint(This,bIsSyncPoint) ) 

#define IMediaSample_IsPreroll(This)	\
    ( (This)->lpVtbl -> IsPreroll(This) ) 

#define IMediaSample_SetPreroll(This,bIsPreroll)	\
    ( (This)->lpVtbl -> SetPreroll(This,bIsPreroll) ) 

#define IMediaSample_GetActualDataLength(This)	\
    ( (This)->lpVtbl -> GetActualDataLength(This) ) 

#define IMediaSample_SetActualDataLength(This,__MIDL__IMediaSample0000)	\
    ( (This)->lpVtbl -> SetActualDataLength(This,__MIDL__IMediaSample0000) ) 

#define IMediaSample_GetMediaType(This,ppMediaType)	\
    ( (This)->lpVtbl -> GetMediaType(This,ppMediaType) ) 

#define IMediaSample_SetMediaType(This,pMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,pMediaType) ) 

#define IMediaSample_IsDiscontinuity(This)	\
    ( (This)->lpVtbl -> IsDiscontinuity(This) ) 

#define IMediaSample_SetDiscontinuity(This,bDiscontinuity)	\
    ( (This)->lpVtbl -> SetDiscontinuity(This,bDiscontinuity) ) 

#define IMediaSample_GetMediaTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetMediaTime(This,pTimeStart,pTimeEnd) ) 

#define IMediaSample_SetMediaTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> SetMediaTime(This,pTimeStart,pTimeEnd) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaSample_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0012 */
/* [local] */ 

typedef IMediaSample *PMEDIASAMPLE;


enum tagAM_SAMPLE_PROPERTY_FLAGS
    {
        AM_SAMPLE_SPLICEPOINT	= 0x1,
        AM_SAMPLE_PREROLL	= 0x2,
        AM_SAMPLE_DATADISCONTINUITY	= 0x4,
        AM_SAMPLE_TYPECHANGED	= 0x8,
        AM_SAMPLE_TIMEVALID	= 0x10,
        AM_SAMPLE_TIMEDISCONTINUITY	= 0x40,
        AM_SAMPLE_FLUSH_ON_PAUSE	= 0x80,
        AM_SAMPLE_STOPVALID	= 0x100,
        AM_SAMPLE_ENDOFSTREAM	= 0x200,
        AM_STREAM_MEDIA	= 0,
        AM_STREAM_CONTROL	= 1
    } ;
typedef struct tagAM_SAMPLE2_PROPERTIES
    {
    DWORD cbData;
    DWORD dwTypeSpecificFlags;
    DWORD dwSampleFlags;
    LONG lActual;
    REFERENCE_TIME tStart;
    REFERENCE_TIME tStop;
    DWORD dwStreamId;
    AM_MEDIA_TYPE *pMediaType;
    BYTE *pbBuffer;
    LONG cbBuffer;
    } 	AM_SAMPLE2_PROPERTIES;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0012_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0012_v0_0_s_ifspec;

#ifndef __IMediaSample2_INTERFACE_DEFINED__
#define __IMediaSample2_INTERFACE_DEFINED__

/* interface IMediaSample2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMediaSample2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36b73884-c2c8-11cf-8b46-00805f6cef60")
    IMediaSample2 : public IMediaSample
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [in] */ DWORD cbProperties,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(cbProperties)  BYTE *pbProperties) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetProperties( 
            /* [in] */ DWORD cbProperties,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbProperties)  const BYTE *pbProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaSample2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaSample2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetPointer)
        HRESULT ( STDMETHODCALLTYPE *GetPointer )( 
            IMediaSample2 * This,
            /* [annotation][out] */ 
            _Outptr_result_buffer_to_(_Inexpressible_(this->GetSize()), _Inexpressible_(this->GetActualDataLength()))  BYTE **ppBuffer);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetSize)
        long ( STDMETHODCALLTYPE *GetSize )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetTime)
        HRESULT ( STDMETHODCALLTYPE *GetTime )( 
            IMediaSample2 * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTimeStart,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetTime)
        HRESULT ( STDMETHODCALLTYPE *SetTime )( 
            IMediaSample2 * This,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pTimeStart,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample, IsSyncPoint)
        HRESULT ( STDMETHODCALLTYPE *IsSyncPoint )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetSyncPoint)
        HRESULT ( STDMETHODCALLTYPE *SetSyncPoint )( 
            IMediaSample2 * This,
            BOOL bIsSyncPoint);
        
        DECLSPEC_XFGVIRT(IMediaSample, IsPreroll)
        HRESULT ( STDMETHODCALLTYPE *IsPreroll )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetPreroll)
        HRESULT ( STDMETHODCALLTYPE *SetPreroll )( 
            IMediaSample2 * This,
            BOOL bIsPreroll);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetActualDataLength)
        long ( STDMETHODCALLTYPE *GetActualDataLength )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetActualDataLength)
        HRESULT ( STDMETHODCALLTYPE *SetActualDataLength )( 
            IMediaSample2 * This,
            long __MIDL__IMediaSample0000);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetMediaType)
        HRESULT ( STDMETHODCALLTYPE *GetMediaType )( 
            IMediaSample2 * This,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppMediaType);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetMediaType )( 
            IMediaSample2 * This,
            /* [annotation][in] */ 
            _In_  AM_MEDIA_TYPE *pMediaType);
        
        DECLSPEC_XFGVIRT(IMediaSample, IsDiscontinuity)
        HRESULT ( STDMETHODCALLTYPE *IsDiscontinuity )( 
            IMediaSample2 * This);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetDiscontinuity)
        HRESULT ( STDMETHODCALLTYPE *SetDiscontinuity )( 
            IMediaSample2 * This,
            BOOL bDiscontinuity);
        
        DECLSPEC_XFGVIRT(IMediaSample, GetMediaTime)
        HRESULT ( STDMETHODCALLTYPE *GetMediaTime )( 
            IMediaSample2 * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTimeStart,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample, SetMediaTime)
        HRESULT ( STDMETHODCALLTYPE *SetMediaTime )( 
            IMediaSample2 * This,
            /* [annotation][in] */ 
            _In_opt_  LONGLONG *pTimeStart,
            /* [annotation][in] */ 
            _In_opt_  LONGLONG *pTimeEnd);
        
        DECLSPEC_XFGVIRT(IMediaSample2, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            IMediaSample2 * This,
            /* [in] */ DWORD cbProperties,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(cbProperties)  BYTE *pbProperties);
        
        DECLSPEC_XFGVIRT(IMediaSample2, SetProperties)
        HRESULT ( STDMETHODCALLTYPE *SetProperties )( 
            IMediaSample2 * This,
            /* [in] */ DWORD cbProperties,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbProperties)  const BYTE *pbProperties);
        
        END_INTERFACE
    } IMediaSample2Vtbl;

    interface IMediaSample2
    {
        CONST_VTBL struct IMediaSample2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaSample2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaSample2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaSample2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaSample2_GetPointer(This,ppBuffer)	\
    ( (This)->lpVtbl -> GetPointer(This,ppBuffer) ) 

#define IMediaSample2_GetSize(This)	\
    ( (This)->lpVtbl -> GetSize(This) ) 

#define IMediaSample2_GetTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetTime(This,pTimeStart,pTimeEnd) ) 

#define IMediaSample2_SetTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> SetTime(This,pTimeStart,pTimeEnd) ) 

#define IMediaSample2_IsSyncPoint(This)	\
    ( (This)->lpVtbl -> IsSyncPoint(This) ) 

#define IMediaSample2_SetSyncPoint(This,bIsSyncPoint)	\
    ( (This)->lpVtbl -> SetSyncPoint(This,bIsSyncPoint) ) 

#define IMediaSample2_IsPreroll(This)	\
    ( (This)->lpVtbl -> IsPreroll(This) ) 

#define IMediaSample2_SetPreroll(This,bIsPreroll)	\
    ( (This)->lpVtbl -> SetPreroll(This,bIsPreroll) ) 

#define IMediaSample2_GetActualDataLength(This)	\
    ( (This)->lpVtbl -> GetActualDataLength(This) ) 

#define IMediaSample2_SetActualDataLength(This,__MIDL__IMediaSample0000)	\
    ( (This)->lpVtbl -> SetActualDataLength(This,__MIDL__IMediaSample0000) ) 

#define IMediaSample2_GetMediaType(This,ppMediaType)	\
    ( (This)->lpVtbl -> GetMediaType(This,ppMediaType) ) 

#define IMediaSample2_SetMediaType(This,pMediaType)	\
    ( (This)->lpVtbl -> SetMediaType(This,pMediaType) ) 

#define IMediaSample2_IsDiscontinuity(This)	\
    ( (This)->lpVtbl -> IsDiscontinuity(This) ) 

#define IMediaSample2_SetDiscontinuity(This,bDiscontinuity)	\
    ( (This)->lpVtbl -> SetDiscontinuity(This,bDiscontinuity) ) 

#define IMediaSample2_GetMediaTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> GetMediaTime(This,pTimeStart,pTimeEnd) ) 

#define IMediaSample2_SetMediaTime(This,pTimeStart,pTimeEnd)	\
    ( (This)->lpVtbl -> SetMediaTime(This,pTimeStart,pTimeEnd) ) 


#define IMediaSample2_GetProperties(This,cbProperties,pbProperties)	\
    ( (This)->lpVtbl -> GetProperties(This,cbProperties,pbProperties) ) 

#define IMediaSample2_SetProperties(This,cbProperties,pbProperties)	\
    ( (This)->lpVtbl -> SetProperties(This,cbProperties,pbProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaSample2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0013 */
/* [local] */ 

typedef IMediaSample2 *PMEDIASAMPLE2;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0013_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0013_v0_0_s_ifspec;

#ifndef __IMediaSample2Config_INTERFACE_DEFINED__
#define __IMediaSample2Config_INTERFACE_DEFINED__

/* interface IMediaSample2Config */
/* [unique][uuid][object] */ 


EXTERN_C const IID IID_IMediaSample2Config;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("68961E68-832B-41ea-BC91-63593F3E70E3")
    IMediaSample2Config : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetSurface( 
            /* [out] */ __RPC__deref_out_opt IUnknown **ppDirect3DSurface9) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaSample2ConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            __RPC__in IMediaSample2Config * This,
            /* [in] */ __RPC__in REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            __RPC__in IMediaSample2Config * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            __RPC__in IMediaSample2Config * This);
        
        DECLSPEC_XFGVIRT(IMediaSample2Config, GetSurface)
        HRESULT ( STDMETHODCALLTYPE *GetSurface )( 
            __RPC__in IMediaSample2Config * This,
            /* [out] */ __RPC__deref_out_opt IUnknown **ppDirect3DSurface9);
        
        END_INTERFACE
    } IMediaSample2ConfigVtbl;

    interface IMediaSample2Config
    {
        CONST_VTBL struct IMediaSample2ConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaSample2Config_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaSample2Config_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaSample2Config_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaSample2Config_GetSurface(This,ppDirect3DSurface9)	\
    ( (This)->lpVtbl -> GetSurface(This,ppDirect3DSurface9) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaSample2Config_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0014 */
/* [local] */ 

#define AM_GBF_PREVFRAMESKIPPED 1
#define AM_GBF_NOTASYNCPOINT 2
#define AM_GBF_NOWAIT 4
#define AM_GBF_NODDSURFACELOCK 8


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0014_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0014_v0_0_s_ifspec;

#ifndef __IMemAllocator_INTERFACE_DEFINED__
#define __IMemAllocator_INTERFACE_DEFINED__

/* interface IMemAllocator */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMemAllocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a8689c-0ad4-11ce-b03a-0020af0ba770")
    IMemAllocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetProperties( 
            /* [annotation][in] */ 
            _In_  ALLOCATOR_PROPERTIES *pRequest,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pActual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetProperties( 
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Commit( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Decommit( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBuffer( 
            /* [annotation][out] */ 
            _Out_  IMediaSample **ppBuffer,
            /* [annotation][unique][in] */ 
            _In_opt_  REFERENCE_TIME *pStartTime,
            /* [annotation][unique][in] */ 
            _In_opt_  REFERENCE_TIME *pEndTime,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseBuffer( 
            /* [in] */ IMediaSample *pBuffer) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMemAllocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMemAllocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMemAllocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMemAllocator * This);
        
        DECLSPEC_XFGVIRT(IMemAllocator, SetProperties)
        HRESULT ( STDMETHODCALLTYPE *SetProperties )( 
            IMemAllocator * This,
            /* [annotation][in] */ 
            _In_  ALLOCATOR_PROPERTIES *pRequest,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pActual);
        
        DECLSPEC_XFGVIRT(IMemAllocator, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            IMemAllocator * This,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pProps);
        
        DECLSPEC_XFGVIRT(IMemAllocator, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            IMemAllocator * This);
        
        DECLSPEC_XFGVIRT(IMemAllocator, Decommit)
        HRESULT ( STDMETHODCALLTYPE *Decommit )( 
            IMemAllocator * This);
        
        DECLSPEC_XFGVIRT(IMemAllocator, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IMemAllocator * This,
            /* [annotation][out] */ 
            _Out_  IMediaSample **ppBuffer,
            /* [annotation][unique][in] */ 
            _In_opt_  REFERENCE_TIME *pStartTime,
            /* [annotation][unique][in] */ 
            _In_opt_  REFERENCE_TIME *pEndTime,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMemAllocator, ReleaseBuffer)
        HRESULT ( STDMETHODCALLTYPE *ReleaseBuffer )( 
            IMemAllocator * This,
            /* [in] */ IMediaSample *pBuffer);
        
        END_INTERFACE
    } IMemAllocatorVtbl;

    interface IMemAllocator
    {
        CONST_VTBL struct IMemAllocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMemAllocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMemAllocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMemAllocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMemAllocator_SetProperties(This,pRequest,pActual)	\
    ( (This)->lpVtbl -> SetProperties(This,pRequest,pActual) ) 

#define IMemAllocator_GetProperties(This,pProps)	\
    ( (This)->lpVtbl -> GetProperties(This,pProps) ) 

#define IMemAllocator_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#define IMemAllocator_Decommit(This)	\
    ( (This)->lpVtbl -> Decommit(This) ) 

#define IMemAllocator_GetBuffer(This,ppBuffer,pStartTime,pEndTime,dwFlags)	\
    ( (This)->lpVtbl -> GetBuffer(This,ppBuffer,pStartTime,pEndTime,dwFlags) ) 

#define IMemAllocator_ReleaseBuffer(This,pBuffer)	\
    ( (This)->lpVtbl -> ReleaseBuffer(This,pBuffer) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMemAllocator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0015 */
/* [local] */ 

typedef IMemAllocator *PMEMALLOCATOR;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0015_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0015_v0_0_s_ifspec;

#ifndef __IMemAllocatorCallbackTemp_INTERFACE_DEFINED__
#define __IMemAllocatorCallbackTemp_INTERFACE_DEFINED__

/* interface IMemAllocatorCallbackTemp */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMemAllocatorCallbackTemp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("379a0cf0-c1de-11d2-abf5-00a0c905f375")
    IMemAllocatorCallbackTemp : public IMemAllocator
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetNotify( 
            /* [in] */ IMemAllocatorNotifyCallbackTemp *pNotify) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFreeCount( 
            /* [annotation][out] */ 
            _Out_  LONG *plBuffersFree) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMemAllocatorCallbackTempVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMemAllocatorCallbackTemp * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMemAllocatorCallbackTemp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMemAllocatorCallbackTemp * This);
        
        DECLSPEC_XFGVIRT(IMemAllocator, SetProperties)
        HRESULT ( STDMETHODCALLTYPE *SetProperties )( 
            IMemAllocatorCallbackTemp * This,
            /* [annotation][in] */ 
            _In_  ALLOCATOR_PROPERTIES *pRequest,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pActual);
        
        DECLSPEC_XFGVIRT(IMemAllocator, GetProperties)
        HRESULT ( STDMETHODCALLTYPE *GetProperties )( 
            IMemAllocatorCallbackTemp * This,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pProps);
        
        DECLSPEC_XFGVIRT(IMemAllocator, Commit)
        HRESULT ( STDMETHODCALLTYPE *Commit )( 
            IMemAllocatorCallbackTemp * This);
        
        DECLSPEC_XFGVIRT(IMemAllocator, Decommit)
        HRESULT ( STDMETHODCALLTYPE *Decommit )( 
            IMemAllocatorCallbackTemp * This);
        
        DECLSPEC_XFGVIRT(IMemAllocator, GetBuffer)
        HRESULT ( STDMETHODCALLTYPE *GetBuffer )( 
            IMemAllocatorCallbackTemp * This,
            /* [annotation][out] */ 
            _Out_  IMediaSample **ppBuffer,
            /* [annotation][unique][in] */ 
            _In_opt_  REFERENCE_TIME *pStartTime,
            /* [annotation][unique][in] */ 
            _In_opt_  REFERENCE_TIME *pEndTime,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IMemAllocator, ReleaseBuffer)
        HRESULT ( STDMETHODCALLTYPE *ReleaseBuffer )( 
            IMemAllocatorCallbackTemp * This,
            /* [in] */ IMediaSample *pBuffer);
        
        DECLSPEC_XFGVIRT(IMemAllocatorCallbackTemp, SetNotify)
        HRESULT ( STDMETHODCALLTYPE *SetNotify )( 
            IMemAllocatorCallbackTemp * This,
            /* [in] */ IMemAllocatorNotifyCallbackTemp *pNotify);
        
        DECLSPEC_XFGVIRT(IMemAllocatorCallbackTemp, GetFreeCount)
        HRESULT ( STDMETHODCALLTYPE *GetFreeCount )( 
            IMemAllocatorCallbackTemp * This,
            /* [annotation][out] */ 
            _Out_  LONG *plBuffersFree);
        
        END_INTERFACE
    } IMemAllocatorCallbackTempVtbl;

    interface IMemAllocatorCallbackTemp
    {
        CONST_VTBL struct IMemAllocatorCallbackTempVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMemAllocatorCallbackTemp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMemAllocatorCallbackTemp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMemAllocatorCallbackTemp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMemAllocatorCallbackTemp_SetProperties(This,pRequest,pActual)	\
    ( (This)->lpVtbl -> SetProperties(This,pRequest,pActual) ) 

#define IMemAllocatorCallbackTemp_GetProperties(This,pProps)	\
    ( (This)->lpVtbl -> GetProperties(This,pProps) ) 

#define IMemAllocatorCallbackTemp_Commit(This)	\
    ( (This)->lpVtbl -> Commit(This) ) 

#define IMemAllocatorCallbackTemp_Decommit(This)	\
    ( (This)->lpVtbl -> Decommit(This) ) 

#define IMemAllocatorCallbackTemp_GetBuffer(This,ppBuffer,pStartTime,pEndTime,dwFlags)	\
    ( (This)->lpVtbl -> GetBuffer(This,ppBuffer,pStartTime,pEndTime,dwFlags) ) 

#define IMemAllocatorCallbackTemp_ReleaseBuffer(This,pBuffer)	\
    ( (This)->lpVtbl -> ReleaseBuffer(This,pBuffer) ) 


#define IMemAllocatorCallbackTemp_SetNotify(This,pNotify)	\
    ( (This)->lpVtbl -> SetNotify(This,pNotify) ) 

#define IMemAllocatorCallbackTemp_GetFreeCount(This,plBuffersFree)	\
    ( (This)->lpVtbl -> GetFreeCount(This,plBuffersFree) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMemAllocatorCallbackTemp_INTERFACE_DEFINED__ */


#ifndef __IMemAllocatorNotifyCallbackTemp_INTERFACE_DEFINED__
#define __IMemAllocatorNotifyCallbackTemp_INTERFACE_DEFINED__

/* interface IMemAllocatorNotifyCallbackTemp */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMemAllocatorNotifyCallbackTemp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92980b30-c1de-11d2-abf5-00a0c905f375")
    IMemAllocatorNotifyCallbackTemp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE NotifyRelease( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMemAllocatorNotifyCallbackTempVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMemAllocatorNotifyCallbackTemp * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMemAllocatorNotifyCallbackTemp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMemAllocatorNotifyCallbackTemp * This);
        
        DECLSPEC_XFGVIRT(IMemAllocatorNotifyCallbackTemp, NotifyRelease)
        HRESULT ( STDMETHODCALLTYPE *NotifyRelease )( 
            IMemAllocatorNotifyCallbackTemp * This);
        
        END_INTERFACE
    } IMemAllocatorNotifyCallbackTempVtbl;

    interface IMemAllocatorNotifyCallbackTemp
    {
        CONST_VTBL struct IMemAllocatorNotifyCallbackTempVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMemAllocatorNotifyCallbackTemp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMemAllocatorNotifyCallbackTemp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMemAllocatorNotifyCallbackTemp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMemAllocatorNotifyCallbackTemp_NotifyRelease(This)	\
    ( (This)->lpVtbl -> NotifyRelease(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMemAllocatorNotifyCallbackTemp_INTERFACE_DEFINED__ */


#ifndef __IMemInputPin_INTERFACE_DEFINED__
#define __IMemInputPin_INTERFACE_DEFINED__

/* interface IMemInputPin */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMemInputPin;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a8689d-0ad4-11ce-b03a-0020af0ba770")
    IMemInputPin : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAllocator( 
            /* [annotation][out] */ 
            _Out_  IMemAllocator **ppAllocator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyAllocator( 
            /* [in] */ IMemAllocator *pAllocator,
            /* [in] */ BOOL bReadOnly) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllocatorRequirements( 
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pProps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Receive( 
            /* [in] */ IMediaSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReceiveMultiple( 
            /* [annotation][size_is][in] */ 
            _In_reads_(nSamples)  IMediaSample **pSamples,
            /* [in] */ long nSamples,
            /* [annotation][out] */ 
            _Out_  long *nSamplesProcessed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReceiveCanBlock( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMemInputPinVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMemInputPin * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMemInputPin * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMemInputPin * This);
        
        DECLSPEC_XFGVIRT(IMemInputPin, GetAllocator)
        HRESULT ( STDMETHODCALLTYPE *GetAllocator )( 
            IMemInputPin * This,
            /* [annotation][out] */ 
            _Out_  IMemAllocator **ppAllocator);
        
        DECLSPEC_XFGVIRT(IMemInputPin, NotifyAllocator)
        HRESULT ( STDMETHODCALLTYPE *NotifyAllocator )( 
            IMemInputPin * This,
            /* [in] */ IMemAllocator *pAllocator,
            /* [in] */ BOOL bReadOnly);
        
        DECLSPEC_XFGVIRT(IMemInputPin, GetAllocatorRequirements)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatorRequirements )( 
            IMemInputPin * This,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pProps);
        
        DECLSPEC_XFGVIRT(IMemInputPin, Receive)
        HRESULT ( STDMETHODCALLTYPE *Receive )( 
            IMemInputPin * This,
            /* [in] */ IMediaSample *pSample);
        
        DECLSPEC_XFGVIRT(IMemInputPin, ReceiveMultiple)
        HRESULT ( STDMETHODCALLTYPE *ReceiveMultiple )( 
            IMemInputPin * This,
            /* [annotation][size_is][in] */ 
            _In_reads_(nSamples)  IMediaSample **pSamples,
            /* [in] */ long nSamples,
            /* [annotation][out] */ 
            _Out_  long *nSamplesProcessed);
        
        DECLSPEC_XFGVIRT(IMemInputPin, ReceiveCanBlock)
        HRESULT ( STDMETHODCALLTYPE *ReceiveCanBlock )( 
            IMemInputPin * This);
        
        END_INTERFACE
    } IMemInputPinVtbl;

    interface IMemInputPin
    {
        CONST_VTBL struct IMemInputPinVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMemInputPin_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMemInputPin_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMemInputPin_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMemInputPin_GetAllocator(This,ppAllocator)	\
    ( (This)->lpVtbl -> GetAllocator(This,ppAllocator) ) 

#define IMemInputPin_NotifyAllocator(This,pAllocator,bReadOnly)	\
    ( (This)->lpVtbl -> NotifyAllocator(This,pAllocator,bReadOnly) ) 

#define IMemInputPin_GetAllocatorRequirements(This,pProps)	\
    ( (This)->lpVtbl -> GetAllocatorRequirements(This,pProps) ) 

#define IMemInputPin_Receive(This,pSample)	\
    ( (This)->lpVtbl -> Receive(This,pSample) ) 

#define IMemInputPin_ReceiveMultiple(This,pSamples,nSamples,nSamplesProcessed)	\
    ( (This)->lpVtbl -> ReceiveMultiple(This,pSamples,nSamples,nSamplesProcessed) ) 

#define IMemInputPin_ReceiveCanBlock(This)	\
    ( (This)->lpVtbl -> ReceiveCanBlock(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMemInputPin_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0018 */
/* [local] */ 

typedef IMemInputPin *PMEMINPUTPIN;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0018_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0018_v0_0_s_ifspec;

#ifndef __IAMovieSetup_INTERFACE_DEFINED__
#define __IAMovieSetup_INTERFACE_DEFINED__

/* interface IAMovieSetup */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMovieSetup;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a3d8cec0-7e5a-11cf-bbc5-00805f6cef20")
    IAMovieSetup : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Register( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unregister( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMovieSetupVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMovieSetup * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMovieSetup * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMovieSetup * This);
        
        DECLSPEC_XFGVIRT(IAMovieSetup, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            IAMovieSetup * This);
        
        DECLSPEC_XFGVIRT(IAMovieSetup, Unregister)
        HRESULT ( STDMETHODCALLTYPE *Unregister )( 
            IAMovieSetup * This);
        
        END_INTERFACE
    } IAMovieSetupVtbl;

    interface IAMovieSetup
    {
        CONST_VTBL struct IAMovieSetupVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMovieSetup_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMovieSetup_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMovieSetup_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMovieSetup_Register(This)	\
    ( (This)->lpVtbl -> Register(This) ) 

#define IAMovieSetup_Unregister(This)	\
    ( (This)->lpVtbl -> Unregister(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMovieSetup_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0019 */
/* [local] */ 

typedef IAMovieSetup *PAMOVIESETUP;

typedef 
enum AM_SEEKING_SeekingFlags
    {
        AM_SEEKING_NoPositioning	= 0,
        AM_SEEKING_AbsolutePositioning	= 0x1,
        AM_SEEKING_RelativePositioning	= 0x2,
        AM_SEEKING_IncrementalPositioning	= 0x3,
        AM_SEEKING_PositioningBitsMask	= 0x3,
        AM_SEEKING_SeekToKeyFrame	= 0x4,
        AM_SEEKING_ReturnTime	= 0x8,
        AM_SEEKING_Segment	= 0x10,
        AM_SEEKING_NoFlush	= 0x20
    } 	AM_SEEKING_SEEKING_FLAGS;

typedef 
enum AM_SEEKING_SeekingCapabilities
    {
        AM_SEEKING_CanSeekAbsolute	= 0x1,
        AM_SEEKING_CanSeekForwards	= 0x2,
        AM_SEEKING_CanSeekBackwards	= 0x4,
        AM_SEEKING_CanGetCurrentPos	= 0x8,
        AM_SEEKING_CanGetStopPos	= 0x10,
        AM_SEEKING_CanGetDuration	= 0x20,
        AM_SEEKING_CanPlayBackwards	= 0x40,
        AM_SEEKING_CanDoSegments	= 0x80,
        AM_SEEKING_Source	= 0x100
    } 	AM_SEEKING_SEEKING_CAPABILITIES;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0019_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0019_v0_0_s_ifspec;

#ifndef __IMediaSeeking_INTERFACE_DEFINED__
#define __IMediaSeeking_INTERFACE_DEFINED__

/* interface IMediaSeeking */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMediaSeeking;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36b73880-c2c8-11cf-8b46-00805f6cef60")
    IMediaSeeking : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilities( 
            /* [annotation][out] */ 
            _Out_  DWORD *pCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckCapabilities( 
            /* [out][in] */ DWORD *pCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsFormatSupported( 
            /* [in] */ const GUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QueryPreferredFormat( 
            /* [annotation][out] */ 
            _Out_  GUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimeFormat( 
            /* [annotation][out] */ 
            _Out_  GUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsUsingTimeFormat( 
            /* [in] */ const GUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimeFormat( 
            /* [in] */ const GUID *pFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDuration( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStopPosition( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pStop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentPosition( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ConvertTimeFormat( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTarget,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pTargetFormat,
            /* [in] */ LONGLONG Source,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pSourceFormat) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPositions( 
            /* [annotation][out][in] */ 
            _Inout_opt_  LONGLONG *pCurrent,
            /* [in] */ DWORD dwCurrentFlags,
            /* [annotation][out][in] */ 
            _Inout_opt_  LONGLONG *pStop,
            /* [in] */ DWORD dwStopFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPositions( 
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pCurrent,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pStop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAvailable( 
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pEarliest,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pLatest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRate( 
            /* [in] */ double dRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRate( 
            /* [annotation][out] */ 
            _Out_  double *pdRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPreroll( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pllPreroll) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaSeekingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaSeeking * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaSeeking * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaSeeking * This);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilities )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pCapabilities);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, CheckCapabilities)
        HRESULT ( STDMETHODCALLTYPE *CheckCapabilities )( 
            IMediaSeeking * This,
            /* [out][in] */ DWORD *pCapabilities);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, IsFormatSupported)
        HRESULT ( STDMETHODCALLTYPE *IsFormatSupported )( 
            IMediaSeeking * This,
            /* [in] */ const GUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, QueryPreferredFormat)
        HRESULT ( STDMETHODCALLTYPE *QueryPreferredFormat )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  GUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *GetTimeFormat )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  GUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, IsUsingTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *IsUsingTimeFormat )( 
            IMediaSeeking * This,
            /* [in] */ const GUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, SetTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *SetTimeFormat )( 
            IMediaSeeking * This,
            /* [in] */ const GUID *pFormat);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetDuration)
        HRESULT ( STDMETHODCALLTYPE *GetDuration )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pDuration);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetStopPosition)
        HRESULT ( STDMETHODCALLTYPE *GetStopPosition )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pStop);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetCurrentPosition)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentPosition )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pCurrent);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, ConvertTimeFormat)
        HRESULT ( STDMETHODCALLTYPE *ConvertTimeFormat )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTarget,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pTargetFormat,
            /* [in] */ LONGLONG Source,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pSourceFormat);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, SetPositions)
        HRESULT ( STDMETHODCALLTYPE *SetPositions )( 
            IMediaSeeking * This,
            /* [annotation][out][in] */ 
            _Inout_opt_  LONGLONG *pCurrent,
            /* [in] */ DWORD dwCurrentFlags,
            /* [annotation][out][in] */ 
            _Inout_opt_  LONGLONG *pStop,
            /* [in] */ DWORD dwStopFlags);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetPositions)
        HRESULT ( STDMETHODCALLTYPE *GetPositions )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pCurrent,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pStop);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetAvailable)
        HRESULT ( STDMETHODCALLTYPE *GetAvailable )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pEarliest,
            /* [annotation][out] */ 
            _Out_opt_  LONGLONG *pLatest);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, SetRate)
        HRESULT ( STDMETHODCALLTYPE *SetRate )( 
            IMediaSeeking * This,
            /* [in] */ double dRate);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetRate)
        HRESULT ( STDMETHODCALLTYPE *GetRate )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  double *pdRate);
        
        DECLSPEC_XFGVIRT(IMediaSeeking, GetPreroll)
        HRESULT ( STDMETHODCALLTYPE *GetPreroll )( 
            IMediaSeeking * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pllPreroll);
        
        END_INTERFACE
    } IMediaSeekingVtbl;

    interface IMediaSeeking
    {
        CONST_VTBL struct IMediaSeekingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaSeeking_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaSeeking_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaSeeking_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaSeeking_GetCapabilities(This,pCapabilities)	\
    ( (This)->lpVtbl -> GetCapabilities(This,pCapabilities) ) 

#define IMediaSeeking_CheckCapabilities(This,pCapabilities)	\
    ( (This)->lpVtbl -> CheckCapabilities(This,pCapabilities) ) 

#define IMediaSeeking_IsFormatSupported(This,pFormat)	\
    ( (This)->lpVtbl -> IsFormatSupported(This,pFormat) ) 

#define IMediaSeeking_QueryPreferredFormat(This,pFormat)	\
    ( (This)->lpVtbl -> QueryPreferredFormat(This,pFormat) ) 

#define IMediaSeeking_GetTimeFormat(This,pFormat)	\
    ( (This)->lpVtbl -> GetTimeFormat(This,pFormat) ) 

#define IMediaSeeking_IsUsingTimeFormat(This,pFormat)	\
    ( (This)->lpVtbl -> IsUsingTimeFormat(This,pFormat) ) 

#define IMediaSeeking_SetTimeFormat(This,pFormat)	\
    ( (This)->lpVtbl -> SetTimeFormat(This,pFormat) ) 

#define IMediaSeeking_GetDuration(This,pDuration)	\
    ( (This)->lpVtbl -> GetDuration(This,pDuration) ) 

#define IMediaSeeking_GetStopPosition(This,pStop)	\
    ( (This)->lpVtbl -> GetStopPosition(This,pStop) ) 

#define IMediaSeeking_GetCurrentPosition(This,pCurrent)	\
    ( (This)->lpVtbl -> GetCurrentPosition(This,pCurrent) ) 

#define IMediaSeeking_ConvertTimeFormat(This,pTarget,pTargetFormat,Source,pSourceFormat)	\
    ( (This)->lpVtbl -> ConvertTimeFormat(This,pTarget,pTargetFormat,Source,pSourceFormat) ) 

#define IMediaSeeking_SetPositions(This,pCurrent,dwCurrentFlags,pStop,dwStopFlags)	\
    ( (This)->lpVtbl -> SetPositions(This,pCurrent,dwCurrentFlags,pStop,dwStopFlags) ) 

#define IMediaSeeking_GetPositions(This,pCurrent,pStop)	\
    ( (This)->lpVtbl -> GetPositions(This,pCurrent,pStop) ) 

#define IMediaSeeking_GetAvailable(This,pEarliest,pLatest)	\
    ( (This)->lpVtbl -> GetAvailable(This,pEarliest,pLatest) ) 

#define IMediaSeeking_SetRate(This,dRate)	\
    ( (This)->lpVtbl -> SetRate(This,dRate) ) 

#define IMediaSeeking_GetRate(This,pdRate)	\
    ( (This)->lpVtbl -> GetRate(This,pdRate) ) 

#define IMediaSeeking_GetPreroll(This,pllPreroll)	\
    ( (This)->lpVtbl -> GetPreroll(This,pllPreroll) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaSeeking_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0020 */
/* [local] */ 

typedef IMediaSeeking *PMEDIASEEKING;

enum tagAM_MEDIAEVENT_FLAGS
{
    AM_MEDIAEVENT_NONOTIFY = 0x01
};
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <winapifamily.h>
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
struct CodecAPIEventData
    {
    GUID guid;
    DWORD dataLength;
    DWORD reserved[ 3 ];
    } ;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0020_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0020_v0_0_s_ifspec;

#ifndef __ICodecAPI_INTERFACE_DEFINED__
#define __ICodecAPI_INTERFACE_DEFINED__

/* interface ICodecAPI */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ICodecAPI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("901db4c7-31ce-41a2-85dc-8fa0bf41b8da")
    ICodecAPI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsSupported( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsModifiable( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterRange( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterValues( 
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterForEvent( 
            /* [in] */ const GUID *Api,
            /* [in] */ LONG_PTR userData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterForEvent( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllDefaults( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValueWithNotify( 
            /* [in] */ const GUID *Api,
            /* [in] */ VARIANT *Value,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllDefaultsWithNotify( 
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllSettings( 
            /* [in] */ IStream *__MIDL__ICodecAPI0000) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllSettings( 
            /* [in] */ IStream *__MIDL__ICodecAPI0001) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAllSettingsWithNotify( 
            IStream *__MIDL__ICodecAPI0002,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICodecAPIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICodecAPI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICodecAPI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICodecAPI * This);
        
        DECLSPEC_XFGVIRT(ICodecAPI, IsSupported)
        HRESULT ( STDMETHODCALLTYPE *IsSupported )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(ICodecAPI, IsModifiable)
        HRESULT ( STDMETHODCALLTYPE *IsModifiable )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetParameterRange)
        HRESULT ( STDMETHODCALLTYPE *GetParameterRange )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetParameterValues)
        HRESULT ( STDMETHODCALLTYPE *GetParameterValues )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetDefaultValue)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultValue )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(ICodecAPI, RegisterForEvent)
        HRESULT ( STDMETHODCALLTYPE *RegisterForEvent )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [in] */ LONG_PTR userData);
        
        DECLSPEC_XFGVIRT(ICodecAPI, UnregisterForEvent)
        HRESULT ( STDMETHODCALLTYPE *UnregisterForEvent )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllDefaults)
        HRESULT ( STDMETHODCALLTYPE *SetAllDefaults )( 
            ICodecAPI * This);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetValueWithNotify)
        HRESULT ( STDMETHODCALLTYPE *SetValueWithNotify )( 
            ICodecAPI * This,
            /* [in] */ const GUID *Api,
            /* [in] */ VARIANT *Value,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllDefaultsWithNotify)
        HRESULT ( STDMETHODCALLTYPE *SetAllDefaultsWithNotify )( 
            ICodecAPI * This,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount);
        
        DECLSPEC_XFGVIRT(ICodecAPI, GetAllSettings)
        HRESULT ( STDMETHODCALLTYPE *GetAllSettings )( 
            ICodecAPI * This,
            /* [in] */ IStream *__MIDL__ICodecAPI0000);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllSettings)
        HRESULT ( STDMETHODCALLTYPE *SetAllSettings )( 
            ICodecAPI * This,
            /* [in] */ IStream *__MIDL__ICodecAPI0001);
        
        DECLSPEC_XFGVIRT(ICodecAPI, SetAllSettingsWithNotify)
        HRESULT ( STDMETHODCALLTYPE *SetAllSettingsWithNotify )( 
            ICodecAPI * This,
            IStream *__MIDL__ICodecAPI0002,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ChangedParamCount)  GUID **ChangedParam,
            /* [annotation][out] */ 
            _Out_  ULONG *ChangedParamCount);
        
        END_INTERFACE
    } ICodecAPIVtbl;

    interface ICodecAPI
    {
        CONST_VTBL struct ICodecAPIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICodecAPI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICodecAPI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICodecAPI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICodecAPI_IsSupported(This,Api)	\
    ( (This)->lpVtbl -> IsSupported(This,Api) ) 

#define ICodecAPI_IsModifiable(This,Api)	\
    ( (This)->lpVtbl -> IsModifiable(This,Api) ) 

#define ICodecAPI_GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta)	\
    ( (This)->lpVtbl -> GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta) ) 

#define ICodecAPI_GetParameterValues(This,Api,Values,ValuesCount)	\
    ( (This)->lpVtbl -> GetParameterValues(This,Api,Values,ValuesCount) ) 

#define ICodecAPI_GetDefaultValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetDefaultValue(This,Api,Value) ) 

#define ICodecAPI_GetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetValue(This,Api,Value) ) 

#define ICodecAPI_SetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Api,Value) ) 

#define ICodecAPI_RegisterForEvent(This,Api,userData)	\
    ( (This)->lpVtbl -> RegisterForEvent(This,Api,userData) ) 

#define ICodecAPI_UnregisterForEvent(This,Api)	\
    ( (This)->lpVtbl -> UnregisterForEvent(This,Api) ) 

#define ICodecAPI_SetAllDefaults(This)	\
    ( (This)->lpVtbl -> SetAllDefaults(This) ) 

#define ICodecAPI_SetValueWithNotify(This,Api,Value,ChangedParam,ChangedParamCount)	\
    ( (This)->lpVtbl -> SetValueWithNotify(This,Api,Value,ChangedParam,ChangedParamCount) ) 

#define ICodecAPI_SetAllDefaultsWithNotify(This,ChangedParam,ChangedParamCount)	\
    ( (This)->lpVtbl -> SetAllDefaultsWithNotify(This,ChangedParam,ChangedParamCount) ) 

#define ICodecAPI_GetAllSettings(This,__MIDL__ICodecAPI0000)	\
    ( (This)->lpVtbl -> GetAllSettings(This,__MIDL__ICodecAPI0000) ) 

#define ICodecAPI_SetAllSettings(This,__MIDL__ICodecAPI0001)	\
    ( (This)->lpVtbl -> SetAllSettings(This,__MIDL__ICodecAPI0001) ) 

#define ICodecAPI_SetAllSettingsWithNotify(This,__MIDL__ICodecAPI0002,ChangedParam,ChangedParamCount)	\
    ( (This)->lpVtbl -> SetAllSettingsWithNotify(This,__MIDL__ICodecAPI0002,ChangedParam,ChangedParamCount) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ICodecAPI_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0021 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion

























































typedef struct REGFILTER
    {
    CLSID Clsid;
    LPWSTR Name;
    } 	REGFILTER;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0021_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0021_v0_0_s_ifspec;

#ifndef __IEnumRegFilters_INTERFACE_DEFINED__
#define __IEnumRegFilters_INTERFACE_DEFINED__

/* interface IEnumRegFilters */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEnumRegFilters;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a4-0ad4-11ce-b03a-0020af0ba770")
    IEnumRegFilters : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cFilters,
            /* [annotation][out] */ 
            _Out_writes_to_(cFilters, *pcFetched)  REGFILTER **apRegFilter,
            /* [annotation][out] */ 
            _Inout_opt_  ULONG *pcFetched) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cFilters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IEnumRegFilters **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumRegFiltersVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumRegFilters * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumRegFilters * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumRegFilters * This);
        
        DECLSPEC_XFGVIRT(IEnumRegFilters, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumRegFilters * This,
            /* [in] */ ULONG cFilters,
            /* [annotation][out] */ 
            _Out_writes_to_(cFilters, *pcFetched)  REGFILTER **apRegFilter,
            /* [annotation][out] */ 
            _Inout_opt_  ULONG *pcFetched);
        
        DECLSPEC_XFGVIRT(IEnumRegFilters, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumRegFilters * This,
            /* [in] */ ULONG cFilters);
        
        DECLSPEC_XFGVIRT(IEnumRegFilters, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumRegFilters * This);
        
        DECLSPEC_XFGVIRT(IEnumRegFilters, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumRegFilters * This,
            /* [annotation][out] */ 
            _Out_  IEnumRegFilters **ppEnum);
        
        END_INTERFACE
    } IEnumRegFiltersVtbl;

    interface IEnumRegFilters
    {
        CONST_VTBL struct IEnumRegFiltersVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumRegFilters_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumRegFilters_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumRegFilters_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumRegFilters_Next(This,cFilters,apRegFilter,pcFetched)	\
    ( (This)->lpVtbl -> Next(This,cFilters,apRegFilter,pcFetched) ) 

#define IEnumRegFilters_Skip(This,cFilters)	\
    ( (This)->lpVtbl -> Skip(This,cFilters) ) 

#define IEnumRegFilters_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumRegFilters_Clone(This,ppEnum)	\
    ( (This)->lpVtbl -> Clone(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumRegFilters_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0022 */
/* [local] */ 

typedef IEnumRegFilters *PENUMREGFILTERS;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0022_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0022_v0_0_s_ifspec;

#ifndef __IFilterMapper_INTERFACE_DEFINED__
#define __IFilterMapper_INTERFACE_DEFINED__

/* interface IFilterMapper */
/* [unique][uuid][object][local] */ 


enum __MIDL_IFilterMapper_0001
    {
        MERIT_PREFERRED	= 0x800000,
        MERIT_NORMAL	= 0x600000,
        MERIT_UNLIKELY	= 0x400000,
        MERIT_DO_NOT_USE	= 0x200000,
        MERIT_SW_COMPRESSOR	= 0x100000,
        MERIT_HW_COMPRESSOR	= 0x100050
    } ;

EXTERN_C const IID IID_IFilterMapper;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a3-0ad4-11ce-b03a-0020af0ba770")
    IFilterMapper : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterFilter( 
            /* [in] */ CLSID clsid,
            /* [in] */ LPCWSTR Name,
            /* [in] */ DWORD dwMerit) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterFilterInstance( 
            /* [in] */ CLSID clsid,
            /* [in] */ LPCWSTR Name,
            /* [annotation][out] */ 
            _Out_  CLSID *MRId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterPin( 
            /* [in] */ CLSID Filter,
            /* [in] */ LPCWSTR Name,
            /* [in] */ BOOL bRendered,
            /* [in] */ BOOL bOutput,
            /* [in] */ BOOL bZero,
            /* [in] */ BOOL bMany,
            /* [in] */ CLSID ConnectsToFilter,
            /* [in] */ LPCWSTR ConnectsToPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterPinType( 
            /* [in] */ CLSID clsFilter,
            /* [in] */ LPCWSTR strName,
            /* [in] */ CLSID clsMajorType,
            /* [in] */ CLSID clsSubType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterFilter( 
            /* [in] */ CLSID Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterFilterInstance( 
            /* [in] */ CLSID MRId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterPin( 
            /* [in] */ CLSID Filter,
            /* [in] */ LPCWSTR Name) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMatchingFilters( 
            /* [annotation][out] */ 
            _Out_  IEnumRegFilters **ppEnum,
            /* [in] */ DWORD dwMerit,
            /* [in] */ BOOL bInputNeeded,
            /* [in] */ CLSID clsInMaj,
            /* [in] */ CLSID clsInSub,
            /* [in] */ BOOL bRender,
            /* [in] */ BOOL bOututNeeded,
            /* [in] */ CLSID clsOutMaj,
            /* [in] */ CLSID clsOutSub) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterMapperVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterMapper * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterMapper * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterMapper * This);
        
        DECLSPEC_XFGVIRT(IFilterMapper, RegisterFilter)
        HRESULT ( STDMETHODCALLTYPE *RegisterFilter )( 
            IFilterMapper * This,
            /* [in] */ CLSID clsid,
            /* [in] */ LPCWSTR Name,
            /* [in] */ DWORD dwMerit);
        
        DECLSPEC_XFGVIRT(IFilterMapper, RegisterFilterInstance)
        HRESULT ( STDMETHODCALLTYPE *RegisterFilterInstance )( 
            IFilterMapper * This,
            /* [in] */ CLSID clsid,
            /* [in] */ LPCWSTR Name,
            /* [annotation][out] */ 
            _Out_  CLSID *MRId);
        
        DECLSPEC_XFGVIRT(IFilterMapper, RegisterPin)
        HRESULT ( STDMETHODCALLTYPE *RegisterPin )( 
            IFilterMapper * This,
            /* [in] */ CLSID Filter,
            /* [in] */ LPCWSTR Name,
            /* [in] */ BOOL bRendered,
            /* [in] */ BOOL bOutput,
            /* [in] */ BOOL bZero,
            /* [in] */ BOOL bMany,
            /* [in] */ CLSID ConnectsToFilter,
            /* [in] */ LPCWSTR ConnectsToPin);
        
        DECLSPEC_XFGVIRT(IFilterMapper, RegisterPinType)
        HRESULT ( STDMETHODCALLTYPE *RegisterPinType )( 
            IFilterMapper * This,
            /* [in] */ CLSID clsFilter,
            /* [in] */ LPCWSTR strName,
            /* [in] */ CLSID clsMajorType,
            /* [in] */ CLSID clsSubType);
        
        DECLSPEC_XFGVIRT(IFilterMapper, UnregisterFilter)
        HRESULT ( STDMETHODCALLTYPE *UnregisterFilter )( 
            IFilterMapper * This,
            /* [in] */ CLSID Filter);
        
        DECLSPEC_XFGVIRT(IFilterMapper, UnregisterFilterInstance)
        HRESULT ( STDMETHODCALLTYPE *UnregisterFilterInstance )( 
            IFilterMapper * This,
            /* [in] */ CLSID MRId);
        
        DECLSPEC_XFGVIRT(IFilterMapper, UnregisterPin)
        HRESULT ( STDMETHODCALLTYPE *UnregisterPin )( 
            IFilterMapper * This,
            /* [in] */ CLSID Filter,
            /* [in] */ LPCWSTR Name);
        
        DECLSPEC_XFGVIRT(IFilterMapper, EnumMatchingFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumMatchingFilters )( 
            IFilterMapper * This,
            /* [annotation][out] */ 
            _Out_  IEnumRegFilters **ppEnum,
            /* [in] */ DWORD dwMerit,
            /* [in] */ BOOL bInputNeeded,
            /* [in] */ CLSID clsInMaj,
            /* [in] */ CLSID clsInSub,
            /* [in] */ BOOL bRender,
            /* [in] */ BOOL bOututNeeded,
            /* [in] */ CLSID clsOutMaj,
            /* [in] */ CLSID clsOutSub);
        
        END_INTERFACE
    } IFilterMapperVtbl;

    interface IFilterMapper
    {
        CONST_VTBL struct IFilterMapperVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterMapper_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterMapper_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterMapper_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterMapper_RegisterFilter(This,clsid,Name,dwMerit)	\
    ( (This)->lpVtbl -> RegisterFilter(This,clsid,Name,dwMerit) ) 

#define IFilterMapper_RegisterFilterInstance(This,clsid,Name,MRId)	\
    ( (This)->lpVtbl -> RegisterFilterInstance(This,clsid,Name,MRId) ) 

#define IFilterMapper_RegisterPin(This,Filter,Name,bRendered,bOutput,bZero,bMany,ConnectsToFilter,ConnectsToPin)	\
    ( (This)->lpVtbl -> RegisterPin(This,Filter,Name,bRendered,bOutput,bZero,bMany,ConnectsToFilter,ConnectsToPin) ) 

#define IFilterMapper_RegisterPinType(This,clsFilter,strName,clsMajorType,clsSubType)	\
    ( (This)->lpVtbl -> RegisterPinType(This,clsFilter,strName,clsMajorType,clsSubType) ) 

#define IFilterMapper_UnregisterFilter(This,Filter)	\
    ( (This)->lpVtbl -> UnregisterFilter(This,Filter) ) 

#define IFilterMapper_UnregisterFilterInstance(This,MRId)	\
    ( (This)->lpVtbl -> UnregisterFilterInstance(This,MRId) ) 

#define IFilterMapper_UnregisterPin(This,Filter,Name)	\
    ( (This)->lpVtbl -> UnregisterPin(This,Filter,Name) ) 

#define IFilterMapper_EnumMatchingFilters(This,ppEnum,dwMerit,bInputNeeded,clsInMaj,clsInSub,bRender,bOututNeeded,clsOutMaj,clsOutSub)	\
    ( (This)->lpVtbl -> EnumMatchingFilters(This,ppEnum,dwMerit,bInputNeeded,clsInMaj,clsInSub,bRender,bOututNeeded,clsOutMaj,clsOutSub) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterMapper_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0023 */
/* [local] */ 

typedef struct REGPINTYPES
    {
    const CLSID *clsMajorType;
    const CLSID *clsMinorType;
    } 	REGPINTYPES;

typedef struct REGFILTERPINS
    {
    LPWSTR strName;
    BOOL bRendered;
    BOOL bOutput;
    BOOL bZero;
    BOOL bMany;
    const CLSID *clsConnectsToFilter;
    const WCHAR *strConnectsToPin;
    UINT nMediaTypes;
    const REGPINTYPES *lpMediaType;
    } 	REGFILTERPINS;

typedef struct REGPINMEDIUM
    {
    CLSID clsMedium;
    DWORD dw1;
    DWORD dw2;
    } 	REGPINMEDIUM;


enum __MIDL___MIDL_itf_strmif_0000_0023_0001
    {
        REG_PINFLAG_B_ZERO	= 0x1,
        REG_PINFLAG_B_RENDERER	= 0x2,
        REG_PINFLAG_B_MANY	= 0x4,
        REG_PINFLAG_B_OUTPUT	= 0x8
    } ;
typedef struct REGFILTERPINS2
    {
    DWORD dwFlags;
    UINT cInstances;
    UINT nMediaTypes;
    /* [size_is] */ const REGPINTYPES *lpMediaType;
    UINT nMediums;
    /* [size_is] */ const REGPINMEDIUM *lpMedium;
    const CLSID *clsPinCategory;
    } 	REGFILTERPINS2;

typedef struct REGFILTER2
    {
    DWORD dwVersion;
    DWORD dwMerit;
    /* [switch_type][switch_is] */ union 
        {
        /* [case()] */ struct 
            {
            ULONG cPins;
            /* [size_is] */ const REGFILTERPINS *rgPins;
            } 	DUMMYSTRUCTNAME;
        /* [case()] */ struct 
            {
            ULONG cPins2;
            /* [size_is] */ const REGFILTERPINS2 *rgPins2;
            } 	DUMMYSTRUCTNAME2;
        } 	DUMMYUNIONNAME;
    } 	REGFILTER2;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0023_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0023_v0_0_s_ifspec;

#ifndef __IFilterMapper2_INTERFACE_DEFINED__
#define __IFilterMapper2_INTERFACE_DEFINED__

/* interface IFilterMapper2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFilterMapper2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b79bb0b0-33c1-11d1-abe1-00a0c905f375")
    IFilterMapper2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateCategory( 
            /* [in] */ REFCLSID clsidCategory,
            /* [in] */ DWORD dwCategoryMerit,
            /* [in] */ LPCWSTR Description) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnregisterFilter( 
            /* [in] */ const CLSID *pclsidCategory,
            /* [in] */ LPCOLESTR szInstance,
            /* [in] */ REFCLSID Filter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterFilter( 
            /* [in] */ REFCLSID clsidFilter,
            /* [in] */ LPCWSTR Name,
            /* [annotation][out][in] */ 
            _Inout_opt_  IMoniker **ppMoniker,
            /* [in] */ const CLSID *pclsidCategory,
            /* [annotation][in] */ 
            _In_  LPCOLESTR szInstance,
            /* [in] */ const REGFILTER2 *prf2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumMatchingFilters( 
            /* [annotation][out] */ 
            _Out_  IEnumMoniker **ppEnum,
            /* [in] */ DWORD dwFlags,
            /* [in] */ BOOL bExactMatch,
            /* [in] */ DWORD dwMerit,
            /* [in] */ BOOL bInputNeeded,
            /* [in] */ DWORD cInputTypes,
            /* [annotation][size_is] */ 
            _In_reads_opt_(cInputTypes * 2)  const GUID *pInputTypes,
            /* [annotation][in] */ 
            _In_opt_  const REGPINMEDIUM *pMedIn,
            /* [annotation][in] */ 
            _In_opt_  const CLSID *pPinCategoryIn,
            /* [in] */ BOOL bRender,
            /* [in] */ BOOL bOutputNeeded,
            /* [in] */ DWORD cOutputTypes,
            /* [annotation][size_is] */ 
            _In_reads_opt_(cOutputTypes * 2)  const GUID *pOutputTypes,
            /* [annotation][in] */ 
            _In_opt_  const REGPINMEDIUM *pMedOut,
            /* [annotation][in] */ 
            _In_opt_  const CLSID *pPinCategoryOut) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterMapper2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterMapper2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterMapper2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterMapper2 * This);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, CreateCategory)
        HRESULT ( STDMETHODCALLTYPE *CreateCategory )( 
            IFilterMapper2 * This,
            /* [in] */ REFCLSID clsidCategory,
            /* [in] */ DWORD dwCategoryMerit,
            /* [in] */ LPCWSTR Description);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, UnregisterFilter)
        HRESULT ( STDMETHODCALLTYPE *UnregisterFilter )( 
            IFilterMapper2 * This,
            /* [in] */ const CLSID *pclsidCategory,
            /* [in] */ LPCOLESTR szInstance,
            /* [in] */ REFCLSID Filter);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, RegisterFilter)
        HRESULT ( STDMETHODCALLTYPE *RegisterFilter )( 
            IFilterMapper2 * This,
            /* [in] */ REFCLSID clsidFilter,
            /* [in] */ LPCWSTR Name,
            /* [annotation][out][in] */ 
            _Inout_opt_  IMoniker **ppMoniker,
            /* [in] */ const CLSID *pclsidCategory,
            /* [annotation][in] */ 
            _In_  LPCOLESTR szInstance,
            /* [in] */ const REGFILTER2 *prf2);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, EnumMatchingFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumMatchingFilters )( 
            IFilterMapper2 * This,
            /* [annotation][out] */ 
            _Out_  IEnumMoniker **ppEnum,
            /* [in] */ DWORD dwFlags,
            /* [in] */ BOOL bExactMatch,
            /* [in] */ DWORD dwMerit,
            /* [in] */ BOOL bInputNeeded,
            /* [in] */ DWORD cInputTypes,
            /* [annotation][size_is] */ 
            _In_reads_opt_(cInputTypes * 2)  const GUID *pInputTypes,
            /* [annotation][in] */ 
            _In_opt_  const REGPINMEDIUM *pMedIn,
            /* [annotation][in] */ 
            _In_opt_  const CLSID *pPinCategoryIn,
            /* [in] */ BOOL bRender,
            /* [in] */ BOOL bOutputNeeded,
            /* [in] */ DWORD cOutputTypes,
            /* [annotation][size_is] */ 
            _In_reads_opt_(cOutputTypes * 2)  const GUID *pOutputTypes,
            /* [annotation][in] */ 
            _In_opt_  const REGPINMEDIUM *pMedOut,
            /* [annotation][in] */ 
            _In_opt_  const CLSID *pPinCategoryOut);
        
        END_INTERFACE
    } IFilterMapper2Vtbl;

    interface IFilterMapper2
    {
        CONST_VTBL struct IFilterMapper2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterMapper2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterMapper2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterMapper2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterMapper2_CreateCategory(This,clsidCategory,dwCategoryMerit,Description)	\
    ( (This)->lpVtbl -> CreateCategory(This,clsidCategory,dwCategoryMerit,Description) ) 

#define IFilterMapper2_UnregisterFilter(This,pclsidCategory,szInstance,Filter)	\
    ( (This)->lpVtbl -> UnregisterFilter(This,pclsidCategory,szInstance,Filter) ) 

#define IFilterMapper2_RegisterFilter(This,clsidFilter,Name,ppMoniker,pclsidCategory,szInstance,prf2)	\
    ( (This)->lpVtbl -> RegisterFilter(This,clsidFilter,Name,ppMoniker,pclsidCategory,szInstance,prf2) ) 

#define IFilterMapper2_EnumMatchingFilters(This,ppEnum,dwFlags,bExactMatch,dwMerit,bInputNeeded,cInputTypes,pInputTypes,pMedIn,pPinCategoryIn,bRender,bOutputNeeded,cOutputTypes,pOutputTypes,pMedOut,pPinCategoryOut)	\
    ( (This)->lpVtbl -> EnumMatchingFilters(This,ppEnum,dwFlags,bExactMatch,dwMerit,bInputNeeded,cInputTypes,pInputTypes,pMedIn,pPinCategoryIn,bRender,bOutputNeeded,cOutputTypes,pOutputTypes,pMedOut,pPinCategoryOut) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterMapper2_INTERFACE_DEFINED__ */


#ifndef __IFilterMapper3_INTERFACE_DEFINED__
#define __IFilterMapper3_INTERFACE_DEFINED__

/* interface IFilterMapper3 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFilterMapper3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b79bb0b1-33c1-11d1-abe1-00a0c905f375")
    IFilterMapper3 : public IFilterMapper2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetICreateDevEnum( 
            /* [annotation][out] */ 
            _Out_  ICreateDevEnum **ppEnum) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterMapper3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterMapper3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterMapper3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterMapper3 * This);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, CreateCategory)
        HRESULT ( STDMETHODCALLTYPE *CreateCategory )( 
            IFilterMapper3 * This,
            /* [in] */ REFCLSID clsidCategory,
            /* [in] */ DWORD dwCategoryMerit,
            /* [in] */ LPCWSTR Description);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, UnregisterFilter)
        HRESULT ( STDMETHODCALLTYPE *UnregisterFilter )( 
            IFilterMapper3 * This,
            /* [in] */ const CLSID *pclsidCategory,
            /* [in] */ LPCOLESTR szInstance,
            /* [in] */ REFCLSID Filter);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, RegisterFilter)
        HRESULT ( STDMETHODCALLTYPE *RegisterFilter )( 
            IFilterMapper3 * This,
            /* [in] */ REFCLSID clsidFilter,
            /* [in] */ LPCWSTR Name,
            /* [annotation][out][in] */ 
            _Inout_opt_  IMoniker **ppMoniker,
            /* [in] */ const CLSID *pclsidCategory,
            /* [annotation][in] */ 
            _In_  LPCOLESTR szInstance,
            /* [in] */ const REGFILTER2 *prf2);
        
        DECLSPEC_XFGVIRT(IFilterMapper2, EnumMatchingFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumMatchingFilters )( 
            IFilterMapper3 * This,
            /* [annotation][out] */ 
            _Out_  IEnumMoniker **ppEnum,
            /* [in] */ DWORD dwFlags,
            /* [in] */ BOOL bExactMatch,
            /* [in] */ DWORD dwMerit,
            /* [in] */ BOOL bInputNeeded,
            /* [in] */ DWORD cInputTypes,
            /* [annotation][size_is] */ 
            _In_reads_opt_(cInputTypes * 2)  const GUID *pInputTypes,
            /* [annotation][in] */ 
            _In_opt_  const REGPINMEDIUM *pMedIn,
            /* [annotation][in] */ 
            _In_opt_  const CLSID *pPinCategoryIn,
            /* [in] */ BOOL bRender,
            /* [in] */ BOOL bOutputNeeded,
            /* [in] */ DWORD cOutputTypes,
            /* [annotation][size_is] */ 
            _In_reads_opt_(cOutputTypes * 2)  const GUID *pOutputTypes,
            /* [annotation][in] */ 
            _In_opt_  const REGPINMEDIUM *pMedOut,
            /* [annotation][in] */ 
            _In_opt_  const CLSID *pPinCategoryOut);
        
        DECLSPEC_XFGVIRT(IFilterMapper3, GetICreateDevEnum)
        HRESULT ( STDMETHODCALLTYPE *GetICreateDevEnum )( 
            IFilterMapper3 * This,
            /* [annotation][out] */ 
            _Out_  ICreateDevEnum **ppEnum);
        
        END_INTERFACE
    } IFilterMapper3Vtbl;

    interface IFilterMapper3
    {
        CONST_VTBL struct IFilterMapper3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterMapper3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterMapper3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterMapper3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterMapper3_CreateCategory(This,clsidCategory,dwCategoryMerit,Description)	\
    ( (This)->lpVtbl -> CreateCategory(This,clsidCategory,dwCategoryMerit,Description) ) 

#define IFilterMapper3_UnregisterFilter(This,pclsidCategory,szInstance,Filter)	\
    ( (This)->lpVtbl -> UnregisterFilter(This,pclsidCategory,szInstance,Filter) ) 

#define IFilterMapper3_RegisterFilter(This,clsidFilter,Name,ppMoniker,pclsidCategory,szInstance,prf2)	\
    ( (This)->lpVtbl -> RegisterFilter(This,clsidFilter,Name,ppMoniker,pclsidCategory,szInstance,prf2) ) 

#define IFilterMapper3_EnumMatchingFilters(This,ppEnum,dwFlags,bExactMatch,dwMerit,bInputNeeded,cInputTypes,pInputTypes,pMedIn,pPinCategoryIn,bRender,bOutputNeeded,cOutputTypes,pOutputTypes,pMedOut,pPinCategoryOut)	\
    ( (This)->lpVtbl -> EnumMatchingFilters(This,ppEnum,dwFlags,bExactMatch,dwMerit,bInputNeeded,cInputTypes,pInputTypes,pMedIn,pPinCategoryIn,bRender,bOutputNeeded,cOutputTypes,pOutputTypes,pMedOut,pPinCategoryOut) ) 


#define IFilterMapper3_GetICreateDevEnum(This,ppEnum)	\
    ( (This)->lpVtbl -> GetICreateDevEnum(This,ppEnum) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterMapper3_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0025 */
/* [local] */ 

typedef 
enum tagQualityMessageType
    {
        Famine	= 0,
        Flood	= ( Famine + 1 ) 
    } 	QualityMessageType;

typedef struct tagQuality
    {
    QualityMessageType Type;
    long Proportion;
    REFERENCE_TIME Late;
    REFERENCE_TIME TimeStamp;
    } 	Quality;

typedef IQualityControl *PQUALITYCONTROL;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0025_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0025_v0_0_s_ifspec;

#ifndef __IQualityControl_INTERFACE_DEFINED__
#define __IQualityControl_INTERFACE_DEFINED__

/* interface IQualityControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IQualityControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a5-0ad4-11ce-b03a-0020af0ba770")
    IQualityControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ IBaseFilter *pSelf,
            /* [in] */ Quality q) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSink( 
            /* [in] */ IQualityControl *piqc) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IQualityControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IQualityControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IQualityControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IQualityControl * This);
        
        DECLSPEC_XFGVIRT(IQualityControl, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            IQualityControl * This,
            /* [in] */ IBaseFilter *pSelf,
            /* [in] */ Quality q);
        
        DECLSPEC_XFGVIRT(IQualityControl, SetSink)
        HRESULT ( STDMETHODCALLTYPE *SetSink )( 
            IQualityControl * This,
            /* [in] */ IQualityControl *piqc);
        
        END_INTERFACE
    } IQualityControlVtbl;

    interface IQualityControl
    {
        CONST_VTBL struct IQualityControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IQualityControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IQualityControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IQualityControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IQualityControl_Notify(This,pSelf,q)	\
    ( (This)->lpVtbl -> Notify(This,pSelf,q) ) 

#define IQualityControl_SetSink(This,piqc)	\
    ( (This)->lpVtbl -> SetSink(This,piqc) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IQualityControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0026 */
/* [local] */ 


enum __MIDL___MIDL_itf_strmif_0000_0026_0001
    {
        CK_NOCOLORKEY	= 0,
        CK_INDEX	= 0x1,
        CK_RGB	= 0x2
    } ;
typedef struct tagCOLORKEY
    {
    DWORD KeyType;
    DWORD PaletteIndex;
    COLORREF LowColorValue;
    COLORREF HighColorValue;
    } 	COLORKEY;


enum __MIDL___MIDL_itf_strmif_0000_0026_0002
    {
        ADVISE_NONE	= 0,
        ADVISE_CLIPPING	= 0x1,
        ADVISE_PALETTE	= 0x2,
        ADVISE_COLORKEY	= 0x4,
        ADVISE_POSITION	= 0x8,
        ADVISE_DISPLAY_CHANGE	= 0x10
    } ;
#define	ADVISE_ALL	( ( ( ( ADVISE_CLIPPING | ADVISE_PALETTE )  | ADVISE_COLORKEY )  | ADVISE_POSITION )  )

#define	ADVISE_ALL2	( ( ADVISE_ALL | ADVISE_DISPLAY_CHANGE )  )

#ifndef _WINGDI_
typedef struct _RGNDATAHEADER
    {
    DWORD dwSize;
    DWORD iType;
    DWORD nCount;
    DWORD nRgnSize;
    RECT rcBound;
    } 	RGNDATAHEADER;

typedef struct _RGNDATA
    {
    RGNDATAHEADER rdh;
    char Buffer[ 1 ];
    } 	RGNDATA;

#endif


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0026_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0026_v0_0_s_ifspec;

#ifndef __IOverlayNotify_INTERFACE_DEFINED__
#define __IOverlayNotify_INTERFACE_DEFINED__

/* interface IOverlayNotify */
/* [unique][uuid][local][object][local] */ 


EXTERN_C const IID IID_IOverlayNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a0-0ad4-11ce-b03a-0020af0ba770")
    IOverlayNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnPaletteChange( 
            /* [in] */ DWORD dwColors,
            /* [in] */ const PALETTEENTRY *pPalette) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnClipChange( 
            /* [in] */ const RECT *pSourceRect,
            /* [in] */ const RECT *pDestinationRect,
            /* [in] */ const RGNDATA *pRgnData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnColorKeyChange( 
            /* [in] */ const COLORKEY *pColorKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnPositionChange( 
            /* [in] */ const RECT *pSourceRect,
            /* [in] */ const RECT *pDestinationRect) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOverlayNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOverlayNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOverlayNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOverlayNotify * This);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnPaletteChange)
        HRESULT ( STDMETHODCALLTYPE *OnPaletteChange )( 
            IOverlayNotify * This,
            /* [in] */ DWORD dwColors,
            /* [in] */ const PALETTEENTRY *pPalette);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnClipChange)
        HRESULT ( STDMETHODCALLTYPE *OnClipChange )( 
            IOverlayNotify * This,
            /* [in] */ const RECT *pSourceRect,
            /* [in] */ const RECT *pDestinationRect,
            /* [in] */ const RGNDATA *pRgnData);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnColorKeyChange)
        HRESULT ( STDMETHODCALLTYPE *OnColorKeyChange )( 
            IOverlayNotify * This,
            /* [in] */ const COLORKEY *pColorKey);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnPositionChange)
        HRESULT ( STDMETHODCALLTYPE *OnPositionChange )( 
            IOverlayNotify * This,
            /* [in] */ const RECT *pSourceRect,
            /* [in] */ const RECT *pDestinationRect);
        
        END_INTERFACE
    } IOverlayNotifyVtbl;

    interface IOverlayNotify
    {
        CONST_VTBL struct IOverlayNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOverlayNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOverlayNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOverlayNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOverlayNotify_OnPaletteChange(This,dwColors,pPalette)	\
    ( (This)->lpVtbl -> OnPaletteChange(This,dwColors,pPalette) ) 

#define IOverlayNotify_OnClipChange(This,pSourceRect,pDestinationRect,pRgnData)	\
    ( (This)->lpVtbl -> OnClipChange(This,pSourceRect,pDestinationRect,pRgnData) ) 

#define IOverlayNotify_OnColorKeyChange(This,pColorKey)	\
    ( (This)->lpVtbl -> OnColorKeyChange(This,pColorKey) ) 

#define IOverlayNotify_OnPositionChange(This,pSourceRect,pDestinationRect)	\
    ( (This)->lpVtbl -> OnPositionChange(This,pSourceRect,pDestinationRect) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOverlayNotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0027 */
/* [local] */ 

typedef IOverlayNotify *POVERLAYNOTIFY;

#if !defined(HMONITOR_DECLARED) && !defined(HMONITOR) && (WINVER < 0x0500)
#define HMONITOR_DECLARED
#if 0
typedef HANDLE HMONITOR;

#endif
DECLARE_HANDLE(HMONITOR);
#endif


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0027_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0027_v0_0_s_ifspec;

#ifndef __IOverlayNotify2_INTERFACE_DEFINED__
#define __IOverlayNotify2_INTERFACE_DEFINED__

/* interface IOverlayNotify2 */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IOverlayNotify2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("680EFA10-D535-11D1-87C8-00A0C9223196")
    IOverlayNotify2 : public IOverlayNotify
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnDisplayChange( 
            HMONITOR hMonitor) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOverlayNotify2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOverlayNotify2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOverlayNotify2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOverlayNotify2 * This);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnPaletteChange)
        HRESULT ( STDMETHODCALLTYPE *OnPaletteChange )( 
            IOverlayNotify2 * This,
            /* [in] */ DWORD dwColors,
            /* [in] */ const PALETTEENTRY *pPalette);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnClipChange)
        HRESULT ( STDMETHODCALLTYPE *OnClipChange )( 
            IOverlayNotify2 * This,
            /* [in] */ const RECT *pSourceRect,
            /* [in] */ const RECT *pDestinationRect,
            /* [in] */ const RGNDATA *pRgnData);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnColorKeyChange)
        HRESULT ( STDMETHODCALLTYPE *OnColorKeyChange )( 
            IOverlayNotify2 * This,
            /* [in] */ const COLORKEY *pColorKey);
        
        DECLSPEC_XFGVIRT(IOverlayNotify, OnPositionChange)
        HRESULT ( STDMETHODCALLTYPE *OnPositionChange )( 
            IOverlayNotify2 * This,
            /* [in] */ const RECT *pSourceRect,
            /* [in] */ const RECT *pDestinationRect);
        
        DECLSPEC_XFGVIRT(IOverlayNotify2, OnDisplayChange)
        HRESULT ( STDMETHODCALLTYPE *OnDisplayChange )( 
            IOverlayNotify2 * This,
            HMONITOR hMonitor);
        
        END_INTERFACE
    } IOverlayNotify2Vtbl;

    interface IOverlayNotify2
    {
        CONST_VTBL struct IOverlayNotify2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOverlayNotify2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOverlayNotify2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOverlayNotify2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOverlayNotify2_OnPaletteChange(This,dwColors,pPalette)	\
    ( (This)->lpVtbl -> OnPaletteChange(This,dwColors,pPalette) ) 

#define IOverlayNotify2_OnClipChange(This,pSourceRect,pDestinationRect,pRgnData)	\
    ( (This)->lpVtbl -> OnClipChange(This,pSourceRect,pDestinationRect,pRgnData) ) 

#define IOverlayNotify2_OnColorKeyChange(This,pColorKey)	\
    ( (This)->lpVtbl -> OnColorKeyChange(This,pColorKey) ) 

#define IOverlayNotify2_OnPositionChange(This,pSourceRect,pDestinationRect)	\
    ( (This)->lpVtbl -> OnPositionChange(This,pSourceRect,pDestinationRect) ) 


#define IOverlayNotify2_OnDisplayChange(This,hMonitor)	\
    ( (This)->lpVtbl -> OnDisplayChange(This,hMonitor) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOverlayNotify2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0028 */
/* [local] */ 

typedef IOverlayNotify2 *POVERLAYNOTIFY2;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0028_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0028_v0_0_s_ifspec;

#ifndef __IOverlay_INTERFACE_DEFINED__
#define __IOverlay_INTERFACE_DEFINED__

/* interface IOverlay */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IOverlay;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a1-0ad4-11ce-b03a-0020af0ba770")
    IOverlay : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPalette( 
            /* [annotation][out] */ 
            _Inout_  DWORD *pdwColors,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_to_(*pdwColors, *pdwColors)  PALETTEENTRY **ppPalette) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPalette( 
            /* [in] */ DWORD dwColors,
            /* [annotation][size_is][in] */ 
            _In_reads_(dwColors)  PALETTEENTRY *pPalette) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultColorKey( 
            /* [annotation][out] */ 
            _Out_  COLORKEY *pColorKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColorKey( 
            /* [annotation][out] */ 
            _Out_  COLORKEY *pColorKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColorKey( 
            /* [out][in] */ COLORKEY *pColorKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetWindowHandle( 
            /* [annotation][out] */ 
            _Out_  HWND *pHwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetClipList( 
            /* [annotation][out] */ 
            _Out_  RECT *pSourceRect,
            /* [annotation][out] */ 
            _Out_  RECT *pDestinationRect,
            /* [annotation][out] */ 
            _Out_  RGNDATA **ppRgnData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoPosition( 
            /* [annotation][out] */ 
            _Out_  RECT *pSourceRect,
            /* [annotation][out] */ 
            _Out_  RECT *pDestinationRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Advise( 
            /* [in] */ IOverlayNotify *pOverlayNotify,
            /* [in] */ DWORD dwInterests) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Unadvise( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IOverlayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IOverlay * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IOverlay * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IOverlay * This);
        
        DECLSPEC_XFGVIRT(IOverlay, GetPalette)
        HRESULT ( STDMETHODCALLTYPE *GetPalette )( 
            IOverlay * This,
            /* [annotation][out] */ 
            _Inout_  DWORD *pdwColors,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_to_(*pdwColors, *pdwColors)  PALETTEENTRY **ppPalette);
        
        DECLSPEC_XFGVIRT(IOverlay, SetPalette)
        HRESULT ( STDMETHODCALLTYPE *SetPalette )( 
            IOverlay * This,
            /* [in] */ DWORD dwColors,
            /* [annotation][size_is][in] */ 
            _In_reads_(dwColors)  PALETTEENTRY *pPalette);
        
        DECLSPEC_XFGVIRT(IOverlay, GetDefaultColorKey)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultColorKey )( 
            IOverlay * This,
            /* [annotation][out] */ 
            _Out_  COLORKEY *pColorKey);
        
        DECLSPEC_XFGVIRT(IOverlay, GetColorKey)
        HRESULT ( STDMETHODCALLTYPE *GetColorKey )( 
            IOverlay * This,
            /* [annotation][out] */ 
            _Out_  COLORKEY *pColorKey);
        
        DECLSPEC_XFGVIRT(IOverlay, SetColorKey)
        HRESULT ( STDMETHODCALLTYPE *SetColorKey )( 
            IOverlay * This,
            /* [out][in] */ COLORKEY *pColorKey);
        
        DECLSPEC_XFGVIRT(IOverlay, GetWindowHandle)
        HRESULT ( STDMETHODCALLTYPE *GetWindowHandle )( 
            IOverlay * This,
            /* [annotation][out] */ 
            _Out_  HWND *pHwnd);
        
        DECLSPEC_XFGVIRT(IOverlay, GetClipList)
        HRESULT ( STDMETHODCALLTYPE *GetClipList )( 
            IOverlay * This,
            /* [annotation][out] */ 
            _Out_  RECT *pSourceRect,
            /* [annotation][out] */ 
            _Out_  RECT *pDestinationRect,
            /* [annotation][out] */ 
            _Out_  RGNDATA **ppRgnData);
        
        DECLSPEC_XFGVIRT(IOverlay, GetVideoPosition)
        HRESULT ( STDMETHODCALLTYPE *GetVideoPosition )( 
            IOverlay * This,
            /* [annotation][out] */ 
            _Out_  RECT *pSourceRect,
            /* [annotation][out] */ 
            _Out_  RECT *pDestinationRect);
        
        DECLSPEC_XFGVIRT(IOverlay, Advise)
        HRESULT ( STDMETHODCALLTYPE *Advise )( 
            IOverlay * This,
            /* [in] */ IOverlayNotify *pOverlayNotify,
            /* [in] */ DWORD dwInterests);
        
        DECLSPEC_XFGVIRT(IOverlay, Unadvise)
        HRESULT ( STDMETHODCALLTYPE *Unadvise )( 
            IOverlay * This);
        
        END_INTERFACE
    } IOverlayVtbl;

    interface IOverlay
    {
        CONST_VTBL struct IOverlayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IOverlay_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IOverlay_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IOverlay_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IOverlay_GetPalette(This,pdwColors,ppPalette)	\
    ( (This)->lpVtbl -> GetPalette(This,pdwColors,ppPalette) ) 

#define IOverlay_SetPalette(This,dwColors,pPalette)	\
    ( (This)->lpVtbl -> SetPalette(This,dwColors,pPalette) ) 

#define IOverlay_GetDefaultColorKey(This,pColorKey)	\
    ( (This)->lpVtbl -> GetDefaultColorKey(This,pColorKey) ) 

#define IOverlay_GetColorKey(This,pColorKey)	\
    ( (This)->lpVtbl -> GetColorKey(This,pColorKey) ) 

#define IOverlay_SetColorKey(This,pColorKey)	\
    ( (This)->lpVtbl -> SetColorKey(This,pColorKey) ) 

#define IOverlay_GetWindowHandle(This,pHwnd)	\
    ( (This)->lpVtbl -> GetWindowHandle(This,pHwnd) ) 

#define IOverlay_GetClipList(This,pSourceRect,pDestinationRect,ppRgnData)	\
    ( (This)->lpVtbl -> GetClipList(This,pSourceRect,pDestinationRect,ppRgnData) ) 

#define IOverlay_GetVideoPosition(This,pSourceRect,pDestinationRect)	\
    ( (This)->lpVtbl -> GetVideoPosition(This,pSourceRect,pDestinationRect) ) 

#define IOverlay_Advise(This,pOverlayNotify,dwInterests)	\
    ( (This)->lpVtbl -> Advise(This,pOverlayNotify,dwInterests) ) 

#define IOverlay_Unadvise(This)	\
    ( (This)->lpVtbl -> Unadvise(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IOverlay_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0029 */
/* [local] */ 

typedef IOverlay *POVERLAY;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0029_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0029_v0_0_s_ifspec;

#ifndef __IMediaEventSink_INTERFACE_DEFINED__
#define __IMediaEventSink_INTERFACE_DEFINED__

/* interface IMediaEventSink */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IMediaEventSink;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a2-0ad4-11ce-b03a-0020af0ba770")
    IMediaEventSink : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Notify( 
            /* [in] */ long EventCode,
            /* [in] */ LONG_PTR EventParam1,
            /* [in] */ LONG_PTR EventParam2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaEventSinkVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaEventSink * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaEventSink * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaEventSink * This);
        
        DECLSPEC_XFGVIRT(IMediaEventSink, Notify)
        HRESULT ( STDMETHODCALLTYPE *Notify )( 
            IMediaEventSink * This,
            /* [in] */ long EventCode,
            /* [in] */ LONG_PTR EventParam1,
            /* [in] */ LONG_PTR EventParam2);
        
        END_INTERFACE
    } IMediaEventSinkVtbl;

    interface IMediaEventSink
    {
        CONST_VTBL struct IMediaEventSinkVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaEventSink_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaEventSink_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaEventSink_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaEventSink_Notify(This,EventCode,EventParam1,EventParam2)	\
    ( (This)->lpVtbl -> Notify(This,EventCode,EventParam1,EventParam2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaEventSink_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0030 */
/* [local] */ 

typedef IMediaEventSink *PMEDIAEVENTSINK;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0030_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0030_v0_0_s_ifspec;

#ifndef __IFileSourceFilter_INTERFACE_DEFINED__
#define __IFileSourceFilter_INTERFACE_DEFINED__

/* interface IFileSourceFilter */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFileSourceFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a6-0ad4-11ce-b03a-0020af0ba770")
    IFileSourceFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ LPCOLESTR pszFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurFile( 
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszFileName,
            /* [annotation][out] */ 
            _Out_opt_  AM_MEDIA_TYPE *pmt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSourceFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFileSourceFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFileSourceFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFileSourceFilter * This);
        
        DECLSPEC_XFGVIRT(IFileSourceFilter, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IFileSourceFilter * This,
            /* [in] */ LPCOLESTR pszFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFileSourceFilter, GetCurFile)
        HRESULT ( STDMETHODCALLTYPE *GetCurFile )( 
            IFileSourceFilter * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszFileName,
            /* [annotation][out] */ 
            _Out_opt_  AM_MEDIA_TYPE *pmt);
        
        END_INTERFACE
    } IFileSourceFilterVtbl;

    interface IFileSourceFilter
    {
        CONST_VTBL struct IFileSourceFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSourceFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSourceFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSourceFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSourceFilter_Load(This,pszFileName,pmt)	\
    ( (This)->lpVtbl -> Load(This,pszFileName,pmt) ) 

#define IFileSourceFilter_GetCurFile(This,ppszFileName,pmt)	\
    ( (This)->lpVtbl -> GetCurFile(This,ppszFileName,pmt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSourceFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0031 */
/* [local] */ 

typedef IFileSourceFilter *PFILTERFILESOURCE;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0031_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0031_v0_0_s_ifspec;

#ifndef __IFileSinkFilter_INTERFACE_DEFINED__
#define __IFileSinkFilter_INTERFACE_DEFINED__

/* interface IFileSinkFilter */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFileSinkFilter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a2104830-7c70-11cf-8bce-00aa00a3f1a6")
    IFileSinkFilter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFileName( 
            /* [in] */ LPCOLESTR pszFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurFile( 
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszFileName,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE *pmt) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSinkFilterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFileSinkFilter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFileSinkFilter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFileSinkFilter * This);
        
        DECLSPEC_XFGVIRT(IFileSinkFilter, SetFileName)
        HRESULT ( STDMETHODCALLTYPE *SetFileName )( 
            IFileSinkFilter * This,
            /* [in] */ LPCOLESTR pszFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFileSinkFilter, GetCurFile)
        HRESULT ( STDMETHODCALLTYPE *GetCurFile )( 
            IFileSinkFilter * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszFileName,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE *pmt);
        
        END_INTERFACE
    } IFileSinkFilterVtbl;

    interface IFileSinkFilter
    {
        CONST_VTBL struct IFileSinkFilterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSinkFilter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSinkFilter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSinkFilter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSinkFilter_SetFileName(This,pszFileName,pmt)	\
    ( (This)->lpVtbl -> SetFileName(This,pszFileName,pmt) ) 

#define IFileSinkFilter_GetCurFile(This,ppszFileName,pmt)	\
    ( (This)->lpVtbl -> GetCurFile(This,ppszFileName,pmt) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSinkFilter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0032 */
/* [local] */ 

typedef IFileSinkFilter *PFILTERFILESINK;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0032_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0032_v0_0_s_ifspec;

#ifndef __IFileSinkFilter2_INTERFACE_DEFINED__
#define __IFileSinkFilter2_INTERFACE_DEFINED__

/* interface IFileSinkFilter2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFileSinkFilter2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("00855B90-CE1B-11d0-BD4F-00A0C911CE86")
    IFileSinkFilter2 : public IFileSinkFilter
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMode( 
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMode( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFileSinkFilter2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFileSinkFilter2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFileSinkFilter2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFileSinkFilter2 * This);
        
        DECLSPEC_XFGVIRT(IFileSinkFilter, SetFileName)
        HRESULT ( STDMETHODCALLTYPE *SetFileName )( 
            IFileSinkFilter2 * This,
            /* [in] */ LPCOLESTR pszFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFileSinkFilter, GetCurFile)
        HRESULT ( STDMETHODCALLTYPE *GetCurFile )( 
            IFileSinkFilter2 * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszFileName,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFileSinkFilter2, SetMode)
        HRESULT ( STDMETHODCALLTYPE *SetMode )( 
            IFileSinkFilter2 * This,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IFileSinkFilter2, GetMode)
        HRESULT ( STDMETHODCALLTYPE *GetMode )( 
            IFileSinkFilter2 * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwFlags);
        
        END_INTERFACE
    } IFileSinkFilter2Vtbl;

    interface IFileSinkFilter2
    {
        CONST_VTBL struct IFileSinkFilter2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFileSinkFilter2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFileSinkFilter2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFileSinkFilter2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFileSinkFilter2_SetFileName(This,pszFileName,pmt)	\
    ( (This)->lpVtbl -> SetFileName(This,pszFileName,pmt) ) 

#define IFileSinkFilter2_GetCurFile(This,ppszFileName,pmt)	\
    ( (This)->lpVtbl -> GetCurFile(This,ppszFileName,pmt) ) 


#define IFileSinkFilter2_SetMode(This,dwFlags)	\
    ( (This)->lpVtbl -> SetMode(This,dwFlags) ) 

#define IFileSinkFilter2_GetMode(This,pdwFlags)	\
    ( (This)->lpVtbl -> GetMode(This,pdwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFileSinkFilter2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0033 */
/* [local] */ 

typedef IFileSinkFilter2 *PFILESINKFILTER2;

typedef 
enum AM_FILESINK_FLAGS
    {
        AM_FILE_OVERWRITE	= 0x1
    } 	AM_FILESINK_FLAGS;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0033_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0033_v0_0_s_ifspec;

#ifndef __IGraphBuilder_INTERFACE_DEFINED__
#define __IGraphBuilder_INTERFACE_DEFINED__

/* interface IGraphBuilder */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IGraphBuilder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868a9-0ad4-11ce-b03a-0020af0ba770")
    IGraphBuilder : public IFilterGraph
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Connect( 
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Render( 
            /* [in] */ IPin *ppinOut) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderFile( 
            /* [in] */ LPCWSTR lpcwstrFile,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrPlayList) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddSourceFilter( 
            /* [in] */ LPCWSTR lpcwstrFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetLogFile( 
            /* [in] */ DWORD_PTR hFile) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Abort( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShouldOperationContinue( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGraphBuilderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGraphBuilder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IFilterGraph, AddFilter)
        HRESULT ( STDMETHODCALLTYPE *AddFilter )( 
            IGraphBuilder * This,
            /* [in] */ IBaseFilter *pFilter,
            /* [string][in] */ LPCWSTR pName);
        
        DECLSPEC_XFGVIRT(IFilterGraph, RemoveFilter)
        HRESULT ( STDMETHODCALLTYPE *RemoveFilter )( 
            IGraphBuilder * This,
            /* [in] */ IBaseFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, EnumFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumFilters )( 
            IGraphBuilder * This,
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum);
        
        DECLSPEC_XFGVIRT(IFilterGraph, FindFilterByName)
        HRESULT ( STDMETHODCALLTYPE *FindFilterByName )( 
            IGraphBuilder * This,
            /* [string][in] */ LPCWSTR pName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, ConnectDirect)
        HRESULT ( STDMETHODCALLTYPE *ConnectDirect )( 
            IGraphBuilder * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Reconnect)
        HRESULT ( STDMETHODCALLTYPE *Reconnect )( 
            IGraphBuilder * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IGraphBuilder * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, SetDefaultSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSyncSource )( 
            IGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IGraphBuilder * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Render)
        HRESULT ( STDMETHODCALLTYPE *Render )( 
            IGraphBuilder * This,
            /* [in] */ IPin *ppinOut);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, RenderFile)
        HRESULT ( STDMETHODCALLTYPE *RenderFile )( 
            IGraphBuilder * This,
            /* [in] */ LPCWSTR lpcwstrFile,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrPlayList);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, AddSourceFilter)
        HRESULT ( STDMETHODCALLTYPE *AddSourceFilter )( 
            IGraphBuilder * This,
            /* [in] */ LPCWSTR lpcwstrFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, SetLogFile)
        HRESULT ( STDMETHODCALLTYPE *SetLogFile )( 
            IGraphBuilder * This,
            /* [in] */ DWORD_PTR hFile);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, ShouldOperationContinue)
        HRESULT ( STDMETHODCALLTYPE *ShouldOperationContinue )( 
            IGraphBuilder * This);
        
        END_INTERFACE
    } IGraphBuilderVtbl;

    interface IGraphBuilder
    {
        CONST_VTBL struct IGraphBuilderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGraphBuilder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGraphBuilder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGraphBuilder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGraphBuilder_AddFilter(This,pFilter,pName)	\
    ( (This)->lpVtbl -> AddFilter(This,pFilter,pName) ) 

#define IGraphBuilder_RemoveFilter(This,pFilter)	\
    ( (This)->lpVtbl -> RemoveFilter(This,pFilter) ) 

#define IGraphBuilder_EnumFilters(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFilters(This,ppEnum) ) 

#define IGraphBuilder_FindFilterByName(This,pName,ppFilter)	\
    ( (This)->lpVtbl -> FindFilterByName(This,pName,ppFilter) ) 

#define IGraphBuilder_ConnectDirect(This,ppinOut,ppinIn,pmt)	\
    ( (This)->lpVtbl -> ConnectDirect(This,ppinOut,ppinIn,pmt) ) 

#define IGraphBuilder_Reconnect(This,ppin)	\
    ( (This)->lpVtbl -> Reconnect(This,ppin) ) 

#define IGraphBuilder_Disconnect(This,ppin)	\
    ( (This)->lpVtbl -> Disconnect(This,ppin) ) 

#define IGraphBuilder_SetDefaultSyncSource(This)	\
    ( (This)->lpVtbl -> SetDefaultSyncSource(This) ) 


#define IGraphBuilder_Connect(This,ppinOut,ppinIn)	\
    ( (This)->lpVtbl -> Connect(This,ppinOut,ppinIn) ) 

#define IGraphBuilder_Render(This,ppinOut)	\
    ( (This)->lpVtbl -> Render(This,ppinOut) ) 

#define IGraphBuilder_RenderFile(This,lpcwstrFile,lpcwstrPlayList)	\
    ( (This)->lpVtbl -> RenderFile(This,lpcwstrFile,lpcwstrPlayList) ) 

#define IGraphBuilder_AddSourceFilter(This,lpcwstrFileName,lpcwstrFilterName,ppFilter)	\
    ( (This)->lpVtbl -> AddSourceFilter(This,lpcwstrFileName,lpcwstrFilterName,ppFilter) ) 

#define IGraphBuilder_SetLogFile(This,hFile)	\
    ( (This)->lpVtbl -> SetLogFile(This,hFile) ) 

#define IGraphBuilder_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IGraphBuilder_ShouldOperationContinue(This)	\
    ( (This)->lpVtbl -> ShouldOperationContinue(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGraphBuilder_INTERFACE_DEFINED__ */


#ifndef __ICaptureGraphBuilder_INTERFACE_DEFINED__
#define __ICaptureGraphBuilder_INTERFACE_DEFINED__

/* interface ICaptureGraphBuilder */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ICaptureGraphBuilder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bf87b6e0-8c27-11d0-b3f0-00aa003761c5")
    ICaptureGraphBuilder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFiltergraph( 
            /* [in] */ IGraphBuilder *pfg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFiltergraph( 
            /* [annotation][out] */ 
            _Out_  IGraphBuilder **ppfg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputFileName( 
            /* [in] */ const GUID *pType,
            /* [in] */ LPCOLESTR lpstrFile,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppf,
            /* [annotation][out] */ 
            _Out_  IFileSinkFilter **ppSink) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE FindInterface( 
            /* [annotation][unique][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ IBaseFilter *pf,
            /* [in] */ REFIID riid,
            /* [annotation][out] */ 
            _Out_  void **ppint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderStream( 
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ IUnknown *pSource,
            /* [in] */ IBaseFilter *pfCompressor,
            /* [in] */ IBaseFilter *pfRenderer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ControlStream( 
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ IBaseFilter *pFilter,
            /* [in] */ REFERENCE_TIME *pstart,
            /* [in] */ REFERENCE_TIME *pstop,
            /* [in] */ WORD wStartCookie,
            /* [in] */ WORD wStopCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllocCapFile( 
            /* [in] */ LPCOLESTR lpstr,
            /* [in] */ DWORDLONG dwlSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyCaptureFile( 
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrOld,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrNew,
            /* [in] */ int fAllowEscAbort,
            /* [in] */ IAMCopyCaptureFileProgress *pCallback) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICaptureGraphBuilderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICaptureGraphBuilder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICaptureGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICaptureGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, SetFiltergraph)
        HRESULT ( STDMETHODCALLTYPE *SetFiltergraph )( 
            ICaptureGraphBuilder * This,
            /* [in] */ IGraphBuilder *pfg);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, GetFiltergraph)
        HRESULT ( STDMETHODCALLTYPE *GetFiltergraph )( 
            ICaptureGraphBuilder * This,
            /* [annotation][out] */ 
            _Out_  IGraphBuilder **ppfg);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, SetOutputFileName)
        HRESULT ( STDMETHODCALLTYPE *SetOutputFileName )( 
            ICaptureGraphBuilder * This,
            /* [in] */ const GUID *pType,
            /* [in] */ LPCOLESTR lpstrFile,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppf,
            /* [annotation][out] */ 
            _Out_  IFileSinkFilter **ppSink);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, FindInterface)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FindInterface )( 
            ICaptureGraphBuilder * This,
            /* [annotation][unique][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ IBaseFilter *pf,
            /* [in] */ REFIID riid,
            /* [annotation][out] */ 
            _Out_  void **ppint);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, RenderStream)
        HRESULT ( STDMETHODCALLTYPE *RenderStream )( 
            ICaptureGraphBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ IUnknown *pSource,
            /* [in] */ IBaseFilter *pfCompressor,
            /* [in] */ IBaseFilter *pfRenderer);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, ControlStream)
        HRESULT ( STDMETHODCALLTYPE *ControlStream )( 
            ICaptureGraphBuilder * This,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ IBaseFilter *pFilter,
            /* [in] */ REFERENCE_TIME *pstart,
            /* [in] */ REFERENCE_TIME *pstop,
            /* [in] */ WORD wStartCookie,
            /* [in] */ WORD wStopCookie);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, AllocCapFile)
        HRESULT ( STDMETHODCALLTYPE *AllocCapFile )( 
            ICaptureGraphBuilder * This,
            /* [in] */ LPCOLESTR lpstr,
            /* [in] */ DWORDLONG dwlSize);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder, CopyCaptureFile)
        HRESULT ( STDMETHODCALLTYPE *CopyCaptureFile )( 
            ICaptureGraphBuilder * This,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrOld,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrNew,
            /* [in] */ int fAllowEscAbort,
            /* [in] */ IAMCopyCaptureFileProgress *pCallback);
        
        END_INTERFACE
    } ICaptureGraphBuilderVtbl;

    interface ICaptureGraphBuilder
    {
        CONST_VTBL struct ICaptureGraphBuilderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICaptureGraphBuilder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICaptureGraphBuilder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICaptureGraphBuilder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICaptureGraphBuilder_SetFiltergraph(This,pfg)	\
    ( (This)->lpVtbl -> SetFiltergraph(This,pfg) ) 

#define ICaptureGraphBuilder_GetFiltergraph(This,ppfg)	\
    ( (This)->lpVtbl -> GetFiltergraph(This,ppfg) ) 

#define ICaptureGraphBuilder_SetOutputFileName(This,pType,lpstrFile,ppf,ppSink)	\
    ( (This)->lpVtbl -> SetOutputFileName(This,pType,lpstrFile,ppf,ppSink) ) 

#define ICaptureGraphBuilder_FindInterface(This,pCategory,pf,riid,ppint)	\
    ( (This)->lpVtbl -> FindInterface(This,pCategory,pf,riid,ppint) ) 

#define ICaptureGraphBuilder_RenderStream(This,pCategory,pSource,pfCompressor,pfRenderer)	\
    ( (This)->lpVtbl -> RenderStream(This,pCategory,pSource,pfCompressor,pfRenderer) ) 

#define ICaptureGraphBuilder_ControlStream(This,pCategory,pFilter,pstart,pstop,wStartCookie,wStopCookie)	\
    ( (This)->lpVtbl -> ControlStream(This,pCategory,pFilter,pstart,pstop,wStartCookie,wStopCookie) ) 

#define ICaptureGraphBuilder_AllocCapFile(This,lpstr,dwlSize)	\
    ( (This)->lpVtbl -> AllocCapFile(This,lpstr,dwlSize) ) 

#define ICaptureGraphBuilder_CopyCaptureFile(This,lpwstrOld,lpwstrNew,fAllowEscAbort,pCallback)	\
    ( (This)->lpVtbl -> CopyCaptureFile(This,lpwstrOld,lpwstrNew,fAllowEscAbort,pCallback) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ICaptureGraphBuilder_RemoteFindInterface_Proxy( 
    ICaptureGraphBuilder * This,
    /* [annotation][unique][in] */ 
    _In_opt_  const GUID *pCategory,
    /* [in] */ IBaseFilter *pf,
    /* [in] */ REFIID riid,
    /* [annotation][out] */ 
    _Out_  IUnknown **ppint);


void __RPC_STUB ICaptureGraphBuilder_RemoteFindInterface_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ICaptureGraphBuilder_INTERFACE_DEFINED__ */


#ifndef __IAMCopyCaptureFileProgress_INTERFACE_DEFINED__
#define __IAMCopyCaptureFileProgress_INTERFACE_DEFINED__

/* interface IAMCopyCaptureFileProgress */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMCopyCaptureFileProgress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("670d1d20-a068-11d0-b3f0-00aa003761c5")
    IAMCopyCaptureFileProgress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Progress( 
            /* [in] */ int iProgress) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMCopyCaptureFileProgressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMCopyCaptureFileProgress * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMCopyCaptureFileProgress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMCopyCaptureFileProgress * This);
        
        DECLSPEC_XFGVIRT(IAMCopyCaptureFileProgress, Progress)
        HRESULT ( STDMETHODCALLTYPE *Progress )( 
            IAMCopyCaptureFileProgress * This,
            /* [in] */ int iProgress);
        
        END_INTERFACE
    } IAMCopyCaptureFileProgressVtbl;

    interface IAMCopyCaptureFileProgress
    {
        CONST_VTBL struct IAMCopyCaptureFileProgressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMCopyCaptureFileProgress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMCopyCaptureFileProgress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMCopyCaptureFileProgress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMCopyCaptureFileProgress_Progress(This,iProgress)	\
    ( (This)->lpVtbl -> Progress(This,iProgress) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMCopyCaptureFileProgress_INTERFACE_DEFINED__ */


#ifndef __ICaptureGraphBuilder2_INTERFACE_DEFINED__
#define __ICaptureGraphBuilder2_INTERFACE_DEFINED__

/* interface ICaptureGraphBuilder2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ICaptureGraphBuilder2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("93E5A4E0-2D50-11d2-ABFA-00A0C9C6E38D")
    ICaptureGraphBuilder2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFiltergraph( 
            /* [in] */ IGraphBuilder *pfg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFiltergraph( 
            /* [annotation][out] */ 
            _Out_  IGraphBuilder **ppfg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputFileName( 
            /* [in] */ const GUID *pType,
            /* [in] */ LPCOLESTR lpstrFile,
            /* [annotation][out] */ 
            _Outptr_  IBaseFilter **ppf,
            /* [annotation][out] */ 
            _Outptr_opt_  IFileSinkFilter **ppSink) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE FindInterface( 
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pType,
            /* [in] */ IBaseFilter *pf,
            /* [in] */ REFIID riid,
            /* [annotation][out] */ 
            _Out_  void **ppint) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderStream( 
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ const GUID *pType,
            /* [in] */ IUnknown *pSource,
            /* [in] */ IBaseFilter *pfCompressor,
            /* [in] */ IBaseFilter *pfRenderer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ControlStream( 
            /* [in] */ const GUID *pCategory,
            /* [in] */ const GUID *pType,
            /* [in] */ IBaseFilter *pFilter,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pstart,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pstop,
            /* [in] */ WORD wStartCookie,
            /* [in] */ WORD wStopCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AllocCapFile( 
            /* [in] */ LPCOLESTR lpstr,
            /* [in] */ DWORDLONG dwlSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CopyCaptureFile( 
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrOld,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrNew,
            /* [in] */ int fAllowEscAbort,
            /* [in] */ IAMCopyCaptureFileProgress *pCallback) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FindPin( 
            /* [in] */ IUnknown *pSource,
            /* [in] */ PIN_DIRECTION pindir,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pType,
            /* [in] */ BOOL fUnconnected,
            /* [in] */ int num,
            /* [annotation][out] */ 
            _Out_  IPin **ppPin) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ICaptureGraphBuilder2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ICaptureGraphBuilder2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ICaptureGraphBuilder2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ICaptureGraphBuilder2 * This);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, SetFiltergraph)
        HRESULT ( STDMETHODCALLTYPE *SetFiltergraph )( 
            ICaptureGraphBuilder2 * This,
            /* [in] */ IGraphBuilder *pfg);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, GetFiltergraph)
        HRESULT ( STDMETHODCALLTYPE *GetFiltergraph )( 
            ICaptureGraphBuilder2 * This,
            /* [annotation][out] */ 
            _Out_  IGraphBuilder **ppfg);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, SetOutputFileName)
        HRESULT ( STDMETHODCALLTYPE *SetOutputFileName )( 
            ICaptureGraphBuilder2 * This,
            /* [in] */ const GUID *pType,
            /* [in] */ LPCOLESTR lpstrFile,
            /* [annotation][out] */ 
            _Outptr_  IBaseFilter **ppf,
            /* [annotation][out] */ 
            _Outptr_opt_  IFileSinkFilter **ppSink);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, FindInterface)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *FindInterface )( 
            ICaptureGraphBuilder2 * This,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pType,
            /* [in] */ IBaseFilter *pf,
            /* [in] */ REFIID riid,
            /* [annotation][out] */ 
            _Out_  void **ppint);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, RenderStream)
        HRESULT ( STDMETHODCALLTYPE *RenderStream )( 
            ICaptureGraphBuilder2 * This,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [in] */ const GUID *pType,
            /* [in] */ IUnknown *pSource,
            /* [in] */ IBaseFilter *pfCompressor,
            /* [in] */ IBaseFilter *pfRenderer);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, ControlStream)
        HRESULT ( STDMETHODCALLTYPE *ControlStream )( 
            ICaptureGraphBuilder2 * This,
            /* [in] */ const GUID *pCategory,
            /* [in] */ const GUID *pType,
            /* [in] */ IBaseFilter *pFilter,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pstart,
            /* [annotation][in] */ 
            _In_opt_  REFERENCE_TIME *pstop,
            /* [in] */ WORD wStartCookie,
            /* [in] */ WORD wStopCookie);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, AllocCapFile)
        HRESULT ( STDMETHODCALLTYPE *AllocCapFile )( 
            ICaptureGraphBuilder2 * This,
            /* [in] */ LPCOLESTR lpstr,
            /* [in] */ DWORDLONG dwlSize);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, CopyCaptureFile)
        HRESULT ( STDMETHODCALLTYPE *CopyCaptureFile )( 
            ICaptureGraphBuilder2 * This,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrOld,
            /* [annotation][in] */ 
            _In_  LPOLESTR lpwstrNew,
            /* [in] */ int fAllowEscAbort,
            /* [in] */ IAMCopyCaptureFileProgress *pCallback);
        
        DECLSPEC_XFGVIRT(ICaptureGraphBuilder2, FindPin)
        HRESULT ( STDMETHODCALLTYPE *FindPin )( 
            ICaptureGraphBuilder2 * This,
            /* [in] */ IUnknown *pSource,
            /* [in] */ PIN_DIRECTION pindir,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pCategory,
            /* [annotation][in] */ 
            _In_opt_  const GUID *pType,
            /* [in] */ BOOL fUnconnected,
            /* [in] */ int num,
            /* [annotation][out] */ 
            _Out_  IPin **ppPin);
        
        END_INTERFACE
    } ICaptureGraphBuilder2Vtbl;

    interface ICaptureGraphBuilder2
    {
        CONST_VTBL struct ICaptureGraphBuilder2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ICaptureGraphBuilder2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ICaptureGraphBuilder2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ICaptureGraphBuilder2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ICaptureGraphBuilder2_SetFiltergraph(This,pfg)	\
    ( (This)->lpVtbl -> SetFiltergraph(This,pfg) ) 

#define ICaptureGraphBuilder2_GetFiltergraph(This,ppfg)	\
    ( (This)->lpVtbl -> GetFiltergraph(This,ppfg) ) 

#define ICaptureGraphBuilder2_SetOutputFileName(This,pType,lpstrFile,ppf,ppSink)	\
    ( (This)->lpVtbl -> SetOutputFileName(This,pType,lpstrFile,ppf,ppSink) ) 

#define ICaptureGraphBuilder2_FindInterface(This,pCategory,pType,pf,riid,ppint)	\
    ( (This)->lpVtbl -> FindInterface(This,pCategory,pType,pf,riid,ppint) ) 

#define ICaptureGraphBuilder2_RenderStream(This,pCategory,pType,pSource,pfCompressor,pfRenderer)	\
    ( (This)->lpVtbl -> RenderStream(This,pCategory,pType,pSource,pfCompressor,pfRenderer) ) 

#define ICaptureGraphBuilder2_ControlStream(This,pCategory,pType,pFilter,pstart,pstop,wStartCookie,wStopCookie)	\
    ( (This)->lpVtbl -> ControlStream(This,pCategory,pType,pFilter,pstart,pstop,wStartCookie,wStopCookie) ) 

#define ICaptureGraphBuilder2_AllocCapFile(This,lpstr,dwlSize)	\
    ( (This)->lpVtbl -> AllocCapFile(This,lpstr,dwlSize) ) 

#define ICaptureGraphBuilder2_CopyCaptureFile(This,lpwstrOld,lpwstrNew,fAllowEscAbort,pCallback)	\
    ( (This)->lpVtbl -> CopyCaptureFile(This,lpwstrOld,lpwstrNew,fAllowEscAbort,pCallback) ) 

#define ICaptureGraphBuilder2_FindPin(This,pSource,pindir,pCategory,pType,fUnconnected,num,ppPin)	\
    ( (This)->lpVtbl -> FindPin(This,pSource,pindir,pCategory,pType,fUnconnected,num,ppPin) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE ICaptureGraphBuilder2_RemoteFindInterface_Proxy( 
    ICaptureGraphBuilder2 * This,
    /* [annotation][in] */ 
    _In_opt_  const GUID *pCategory,
    /* [annotation][in] */ 
    _In_opt_  const GUID *pType,
    /* [in] */ IBaseFilter *pf,
    /* [in] */ REFIID riid,
    /* [annotation][out] */ 
    _Out_  IUnknown **ppint);


void __RPC_STUB ICaptureGraphBuilder2_RemoteFindInterface_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __ICaptureGraphBuilder2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0037 */
/* [local] */ 


enum _AM_RENSDEREXFLAGS
    {
        AM_RENDEREX_RENDERTOEXISTINGRENDERERS	= 0x1
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0037_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0037_v0_0_s_ifspec;

#ifndef __IFilterGraph2_INTERFACE_DEFINED__
#define __IFilterGraph2_INTERFACE_DEFINED__

/* interface IFilterGraph2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFilterGraph2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36b73882-c2c8-11cf-8b46-00805f6cef60")
    IFilterGraph2 : public IGraphBuilder
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AddSourceFilterForMoniker( 
            /* [in] */ IMoniker *pMoniker,
            /* [in] */ IBindCtx *pCtx,
            /* [unique][in] */ LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReconnectEx( 
            /* [in] */ IPin *ppin,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderEx( 
            /* [in] */ IPin *pPinOut,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out][in] */ 
            _Reserved_  DWORD *pvContext) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterGraph2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterGraph2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterGraph2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterGraph2 * This);
        
        DECLSPEC_XFGVIRT(IFilterGraph, AddFilter)
        HRESULT ( STDMETHODCALLTYPE *AddFilter )( 
            IFilterGraph2 * This,
            /* [in] */ IBaseFilter *pFilter,
            /* [string][in] */ LPCWSTR pName);
        
        DECLSPEC_XFGVIRT(IFilterGraph, RemoveFilter)
        HRESULT ( STDMETHODCALLTYPE *RemoveFilter )( 
            IFilterGraph2 * This,
            /* [in] */ IBaseFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, EnumFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumFilters )( 
            IFilterGraph2 * This,
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum);
        
        DECLSPEC_XFGVIRT(IFilterGraph, FindFilterByName)
        HRESULT ( STDMETHODCALLTYPE *FindFilterByName )( 
            IFilterGraph2 * This,
            /* [string][in] */ LPCWSTR pName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, ConnectDirect)
        HRESULT ( STDMETHODCALLTYPE *ConnectDirect )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Reconnect)
        HRESULT ( STDMETHODCALLTYPE *Reconnect )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, SetDefaultSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSyncSource )( 
            IFilterGraph2 * This);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Render)
        HRESULT ( STDMETHODCALLTYPE *Render )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *ppinOut);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, RenderFile)
        HRESULT ( STDMETHODCALLTYPE *RenderFile )( 
            IFilterGraph2 * This,
            /* [in] */ LPCWSTR lpcwstrFile,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrPlayList);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, AddSourceFilter)
        HRESULT ( STDMETHODCALLTYPE *AddSourceFilter )( 
            IFilterGraph2 * This,
            /* [in] */ LPCWSTR lpcwstrFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, SetLogFile)
        HRESULT ( STDMETHODCALLTYPE *SetLogFile )( 
            IFilterGraph2 * This,
            /* [in] */ DWORD_PTR hFile);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IFilterGraph2 * This);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, ShouldOperationContinue)
        HRESULT ( STDMETHODCALLTYPE *ShouldOperationContinue )( 
            IFilterGraph2 * This);
        
        DECLSPEC_XFGVIRT(IFilterGraph2, AddSourceFilterForMoniker)
        HRESULT ( STDMETHODCALLTYPE *AddSourceFilterForMoniker )( 
            IFilterGraph2 * This,
            /* [in] */ IMoniker *pMoniker,
            /* [in] */ IBindCtx *pCtx,
            /* [unique][in] */ LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph2, ReconnectEx)
        HRESULT ( STDMETHODCALLTYPE *ReconnectEx )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *ppin,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFilterGraph2, RenderEx)
        HRESULT ( STDMETHODCALLTYPE *RenderEx )( 
            IFilterGraph2 * This,
            /* [in] */ IPin *pPinOut,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out][in] */ 
            _Reserved_  DWORD *pvContext);
        
        END_INTERFACE
    } IFilterGraph2Vtbl;

    interface IFilterGraph2
    {
        CONST_VTBL struct IFilterGraph2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterGraph2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterGraph2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterGraph2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterGraph2_AddFilter(This,pFilter,pName)	\
    ( (This)->lpVtbl -> AddFilter(This,pFilter,pName) ) 

#define IFilterGraph2_RemoveFilter(This,pFilter)	\
    ( (This)->lpVtbl -> RemoveFilter(This,pFilter) ) 

#define IFilterGraph2_EnumFilters(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFilters(This,ppEnum) ) 

#define IFilterGraph2_FindFilterByName(This,pName,ppFilter)	\
    ( (This)->lpVtbl -> FindFilterByName(This,pName,ppFilter) ) 

#define IFilterGraph2_ConnectDirect(This,ppinOut,ppinIn,pmt)	\
    ( (This)->lpVtbl -> ConnectDirect(This,ppinOut,ppinIn,pmt) ) 

#define IFilterGraph2_Reconnect(This,ppin)	\
    ( (This)->lpVtbl -> Reconnect(This,ppin) ) 

#define IFilterGraph2_Disconnect(This,ppin)	\
    ( (This)->lpVtbl -> Disconnect(This,ppin) ) 

#define IFilterGraph2_SetDefaultSyncSource(This)	\
    ( (This)->lpVtbl -> SetDefaultSyncSource(This) ) 


#define IFilterGraph2_Connect(This,ppinOut,ppinIn)	\
    ( (This)->lpVtbl -> Connect(This,ppinOut,ppinIn) ) 

#define IFilterGraph2_Render(This,ppinOut)	\
    ( (This)->lpVtbl -> Render(This,ppinOut) ) 

#define IFilterGraph2_RenderFile(This,lpcwstrFile,lpcwstrPlayList)	\
    ( (This)->lpVtbl -> RenderFile(This,lpcwstrFile,lpcwstrPlayList) ) 

#define IFilterGraph2_AddSourceFilter(This,lpcwstrFileName,lpcwstrFilterName,ppFilter)	\
    ( (This)->lpVtbl -> AddSourceFilter(This,lpcwstrFileName,lpcwstrFilterName,ppFilter) ) 

#define IFilterGraph2_SetLogFile(This,hFile)	\
    ( (This)->lpVtbl -> SetLogFile(This,hFile) ) 

#define IFilterGraph2_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IFilterGraph2_ShouldOperationContinue(This)	\
    ( (This)->lpVtbl -> ShouldOperationContinue(This) ) 


#define IFilterGraph2_AddSourceFilterForMoniker(This,pMoniker,pCtx,lpcwstrFilterName,ppFilter)	\
    ( (This)->lpVtbl -> AddSourceFilterForMoniker(This,pMoniker,pCtx,lpcwstrFilterName,ppFilter) ) 

#define IFilterGraph2_ReconnectEx(This,ppin,pmt)	\
    ( (This)->lpVtbl -> ReconnectEx(This,ppin,pmt) ) 

#define IFilterGraph2_RenderEx(This,pPinOut,dwFlags,pvContext)	\
    ( (This)->lpVtbl -> RenderEx(This,pPinOut,dwFlags,pvContext) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterGraph2_INTERFACE_DEFINED__ */


#ifndef __IFilterGraph3_INTERFACE_DEFINED__
#define __IFilterGraph3_INTERFACE_DEFINED__

/* interface IFilterGraph3 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFilterGraph3;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aaf38154-b80b-422f-91e6-b66467509a07")
    IFilterGraph3 : public IFilterGraph2
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetSyncSourceEx( 
            /* [annotation][in] */ 
            _In_  IReferenceClock *pClockForMostOfFilterGraph,
            /* [annotation][in] */ 
            _In_  IReferenceClock *pClockForFilter,
            /* [annotation][in] */ 
            _In_  IBaseFilter *pFilter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterGraph3Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterGraph3 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterGraph3 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterGraph3 * This);
        
        DECLSPEC_XFGVIRT(IFilterGraph, AddFilter)
        HRESULT ( STDMETHODCALLTYPE *AddFilter )( 
            IFilterGraph3 * This,
            /* [in] */ IBaseFilter *pFilter,
            /* [string][in] */ LPCWSTR pName);
        
        DECLSPEC_XFGVIRT(IFilterGraph, RemoveFilter)
        HRESULT ( STDMETHODCALLTYPE *RemoveFilter )( 
            IFilterGraph3 * This,
            /* [in] */ IBaseFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, EnumFilters)
        HRESULT ( STDMETHODCALLTYPE *EnumFilters )( 
            IFilterGraph3 * This,
            /* [annotation][out] */ 
            _Out_  IEnumFilters **ppEnum);
        
        DECLSPEC_XFGVIRT(IFilterGraph, FindFilterByName)
        HRESULT ( STDMETHODCALLTYPE *FindFilterByName )( 
            IFilterGraph3 * This,
            /* [string][in] */ LPCWSTR pName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph, ConnectDirect)
        HRESULT ( STDMETHODCALLTYPE *ConnectDirect )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Reconnect)
        HRESULT ( STDMETHODCALLTYPE *Reconnect )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, Disconnect)
        HRESULT ( STDMETHODCALLTYPE *Disconnect )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *ppin);
        
        DECLSPEC_XFGVIRT(IFilterGraph, SetDefaultSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultSyncSource )( 
            IFilterGraph3 * This);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Connect)
        HRESULT ( STDMETHODCALLTYPE *Connect )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IPin *ppinIn);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Render)
        HRESULT ( STDMETHODCALLTYPE *Render )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *ppinOut);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, RenderFile)
        HRESULT ( STDMETHODCALLTYPE *RenderFile )( 
            IFilterGraph3 * This,
            /* [in] */ LPCWSTR lpcwstrFile,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrPlayList);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, AddSourceFilter)
        HRESULT ( STDMETHODCALLTYPE *AddSourceFilter )( 
            IFilterGraph3 * This,
            /* [in] */ LPCWSTR lpcwstrFileName,
            /* [annotation][unique][in] */ 
            _In_opt_  LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, SetLogFile)
        HRESULT ( STDMETHODCALLTYPE *SetLogFile )( 
            IFilterGraph3 * This,
            /* [in] */ DWORD_PTR hFile);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, Abort)
        HRESULT ( STDMETHODCALLTYPE *Abort )( 
            IFilterGraph3 * This);
        
        DECLSPEC_XFGVIRT(IGraphBuilder, ShouldOperationContinue)
        HRESULT ( STDMETHODCALLTYPE *ShouldOperationContinue )( 
            IFilterGraph3 * This);
        
        DECLSPEC_XFGVIRT(IFilterGraph2, AddSourceFilterForMoniker)
        HRESULT ( STDMETHODCALLTYPE *AddSourceFilterForMoniker )( 
            IFilterGraph3 * This,
            /* [in] */ IMoniker *pMoniker,
            /* [in] */ IBindCtx *pCtx,
            /* [unique][in] */ LPCWSTR lpcwstrFilterName,
            /* [annotation][out] */ 
            _Out_  IBaseFilter **ppFilter);
        
        DECLSPEC_XFGVIRT(IFilterGraph2, ReconnectEx)
        HRESULT ( STDMETHODCALLTYPE *ReconnectEx )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *ppin,
            /* [annotation][unique][in] */ 
            _In_opt_  const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IFilterGraph2, RenderEx)
        HRESULT ( STDMETHODCALLTYPE *RenderEx )( 
            IFilterGraph3 * This,
            /* [in] */ IPin *pPinOut,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out][in] */ 
            _Reserved_  DWORD *pvContext);
        
        DECLSPEC_XFGVIRT(IFilterGraph3, SetSyncSourceEx)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSourceEx )( 
            IFilterGraph3 * This,
            /* [annotation][in] */ 
            _In_  IReferenceClock *pClockForMostOfFilterGraph,
            /* [annotation][in] */ 
            _In_  IReferenceClock *pClockForFilter,
            /* [annotation][in] */ 
            _In_  IBaseFilter *pFilter);
        
        END_INTERFACE
    } IFilterGraph3Vtbl;

    interface IFilterGraph3
    {
        CONST_VTBL struct IFilterGraph3Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterGraph3_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterGraph3_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterGraph3_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterGraph3_AddFilter(This,pFilter,pName)	\
    ( (This)->lpVtbl -> AddFilter(This,pFilter,pName) ) 

#define IFilterGraph3_RemoveFilter(This,pFilter)	\
    ( (This)->lpVtbl -> RemoveFilter(This,pFilter) ) 

#define IFilterGraph3_EnumFilters(This,ppEnum)	\
    ( (This)->lpVtbl -> EnumFilters(This,ppEnum) ) 

#define IFilterGraph3_FindFilterByName(This,pName,ppFilter)	\
    ( (This)->lpVtbl -> FindFilterByName(This,pName,ppFilter) ) 

#define IFilterGraph3_ConnectDirect(This,ppinOut,ppinIn,pmt)	\
    ( (This)->lpVtbl -> ConnectDirect(This,ppinOut,ppinIn,pmt) ) 

#define IFilterGraph3_Reconnect(This,ppin)	\
    ( (This)->lpVtbl -> Reconnect(This,ppin) ) 

#define IFilterGraph3_Disconnect(This,ppin)	\
    ( (This)->lpVtbl -> Disconnect(This,ppin) ) 

#define IFilterGraph3_SetDefaultSyncSource(This)	\
    ( (This)->lpVtbl -> SetDefaultSyncSource(This) ) 


#define IFilterGraph3_Connect(This,ppinOut,ppinIn)	\
    ( (This)->lpVtbl -> Connect(This,ppinOut,ppinIn) ) 

#define IFilterGraph3_Render(This,ppinOut)	\
    ( (This)->lpVtbl -> Render(This,ppinOut) ) 

#define IFilterGraph3_RenderFile(This,lpcwstrFile,lpcwstrPlayList)	\
    ( (This)->lpVtbl -> RenderFile(This,lpcwstrFile,lpcwstrPlayList) ) 

#define IFilterGraph3_AddSourceFilter(This,lpcwstrFileName,lpcwstrFilterName,ppFilter)	\
    ( (This)->lpVtbl -> AddSourceFilter(This,lpcwstrFileName,lpcwstrFilterName,ppFilter) ) 

#define IFilterGraph3_SetLogFile(This,hFile)	\
    ( (This)->lpVtbl -> SetLogFile(This,hFile) ) 

#define IFilterGraph3_Abort(This)	\
    ( (This)->lpVtbl -> Abort(This) ) 

#define IFilterGraph3_ShouldOperationContinue(This)	\
    ( (This)->lpVtbl -> ShouldOperationContinue(This) ) 


#define IFilterGraph3_AddSourceFilterForMoniker(This,pMoniker,pCtx,lpcwstrFilterName,ppFilter)	\
    ( (This)->lpVtbl -> AddSourceFilterForMoniker(This,pMoniker,pCtx,lpcwstrFilterName,ppFilter) ) 

#define IFilterGraph3_ReconnectEx(This,ppin,pmt)	\
    ( (This)->lpVtbl -> ReconnectEx(This,ppin,pmt) ) 

#define IFilterGraph3_RenderEx(This,pPinOut,dwFlags,pvContext)	\
    ( (This)->lpVtbl -> RenderEx(This,pPinOut,dwFlags,pvContext) ) 


#define IFilterGraph3_SetSyncSourceEx(This,pClockForMostOfFilterGraph,pClockForFilter,pFilter)	\
    ( (This)->lpVtbl -> SetSyncSourceEx(This,pClockForMostOfFilterGraph,pClockForFilter,pFilter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterGraph3_INTERFACE_DEFINED__ */


#ifndef __IStreamBuilder_INTERFACE_DEFINED__
#define __IStreamBuilder_INTERFACE_DEFINED__

/* interface IStreamBuilder */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IStreamBuilder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868bf-0ad4-11ce-b03a-0020af0ba770")
    IStreamBuilder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Render( 
            /* [in] */ IPin *ppinOut,
            /* [in] */ IGraphBuilder *pGraph) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Backout( 
            /* [in] */ IPin *ppinOut,
            /* [in] */ IGraphBuilder *pGraph) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IStreamBuilderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IStreamBuilder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IStreamBuilder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IStreamBuilder * This);
        
        DECLSPEC_XFGVIRT(IStreamBuilder, Render)
        HRESULT ( STDMETHODCALLTYPE *Render )( 
            IStreamBuilder * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IGraphBuilder *pGraph);
        
        DECLSPEC_XFGVIRT(IStreamBuilder, Backout)
        HRESULT ( STDMETHODCALLTYPE *Backout )( 
            IStreamBuilder * This,
            /* [in] */ IPin *ppinOut,
            /* [in] */ IGraphBuilder *pGraph);
        
        END_INTERFACE
    } IStreamBuilderVtbl;

    interface IStreamBuilder
    {
        CONST_VTBL struct IStreamBuilderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IStreamBuilder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IStreamBuilder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IStreamBuilder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IStreamBuilder_Render(This,ppinOut,pGraph)	\
    ( (This)->lpVtbl -> Render(This,ppinOut,pGraph) ) 

#define IStreamBuilder_Backout(This,ppinOut,pGraph)	\
    ( (This)->lpVtbl -> Backout(This,ppinOut,pGraph) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IStreamBuilder_INTERFACE_DEFINED__ */


#ifndef __IAsyncReader_INTERFACE_DEFINED__
#define __IAsyncReader_INTERFACE_DEFINED__

/* interface IAsyncReader */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAsyncReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868aa-0ad4-11ce-b03a-0020af0ba770")
    IAsyncReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RequestAllocator( 
            /* [in] */ IMemAllocator *pPreferred,
            /* [annotation][in] */ 
            _In_  ALLOCATOR_PROPERTIES *pProps,
            /* [annotation][out] */ 
            _Out_  IMemAllocator **ppActual) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Request( 
            /* [in] */ IMediaSample *pSample,
            /* [in] */ DWORD_PTR dwUser) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitForNext( 
            /* [in] */ DWORD dwTimeout,
            /* [annotation][out] */ 
            _Out_opt_  IMediaSample **ppSample,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwUser) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SyncReadAligned( 
            /* [in] */ IMediaSample *pSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SyncRead( 
            /* [in] */ LONGLONG llPosition,
            /* [in] */ LONG lLength,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(lLength)  BYTE *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Length( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTotal,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pAvailable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BeginFlush( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EndFlush( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAsyncReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAsyncReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAsyncReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAsyncReader * This);
        
        DECLSPEC_XFGVIRT(IAsyncReader, RequestAllocator)
        HRESULT ( STDMETHODCALLTYPE *RequestAllocator )( 
            IAsyncReader * This,
            /* [in] */ IMemAllocator *pPreferred,
            /* [annotation][in] */ 
            _In_  ALLOCATOR_PROPERTIES *pProps,
            /* [annotation][out] */ 
            _Out_  IMemAllocator **ppActual);
        
        DECLSPEC_XFGVIRT(IAsyncReader, Request)
        HRESULT ( STDMETHODCALLTYPE *Request )( 
            IAsyncReader * This,
            /* [in] */ IMediaSample *pSample,
            /* [in] */ DWORD_PTR dwUser);
        
        DECLSPEC_XFGVIRT(IAsyncReader, WaitForNext)
        HRESULT ( STDMETHODCALLTYPE *WaitForNext )( 
            IAsyncReader * This,
            /* [in] */ DWORD dwTimeout,
            /* [annotation][out] */ 
            _Out_opt_  IMediaSample **ppSample,
            /* [annotation][out] */ 
            _Out_  DWORD_PTR *pdwUser);
        
        DECLSPEC_XFGVIRT(IAsyncReader, SyncReadAligned)
        HRESULT ( STDMETHODCALLTYPE *SyncReadAligned )( 
            IAsyncReader * This,
            /* [in] */ IMediaSample *pSample);
        
        DECLSPEC_XFGVIRT(IAsyncReader, SyncRead)
        HRESULT ( STDMETHODCALLTYPE *SyncRead )( 
            IAsyncReader * This,
            /* [in] */ LONGLONG llPosition,
            /* [in] */ LONG lLength,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_(lLength)  BYTE *pBuffer);
        
        DECLSPEC_XFGVIRT(IAsyncReader, Length)
        HRESULT ( STDMETHODCALLTYPE *Length )( 
            IAsyncReader * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pTotal,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pAvailable);
        
        DECLSPEC_XFGVIRT(IAsyncReader, BeginFlush)
        HRESULT ( STDMETHODCALLTYPE *BeginFlush )( 
            IAsyncReader * This);
        
        DECLSPEC_XFGVIRT(IAsyncReader, EndFlush)
        HRESULT ( STDMETHODCALLTYPE *EndFlush )( 
            IAsyncReader * This);
        
        END_INTERFACE
    } IAsyncReaderVtbl;

    interface IAsyncReader
    {
        CONST_VTBL struct IAsyncReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAsyncReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAsyncReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAsyncReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAsyncReader_RequestAllocator(This,pPreferred,pProps,ppActual)	\
    ( (This)->lpVtbl -> RequestAllocator(This,pPreferred,pProps,ppActual) ) 

#define IAsyncReader_Request(This,pSample,dwUser)	\
    ( (This)->lpVtbl -> Request(This,pSample,dwUser) ) 

#define IAsyncReader_WaitForNext(This,dwTimeout,ppSample,pdwUser)	\
    ( (This)->lpVtbl -> WaitForNext(This,dwTimeout,ppSample,pdwUser) ) 

#define IAsyncReader_SyncReadAligned(This,pSample)	\
    ( (This)->lpVtbl -> SyncReadAligned(This,pSample) ) 

#define IAsyncReader_SyncRead(This,llPosition,lLength,pBuffer)	\
    ( (This)->lpVtbl -> SyncRead(This,llPosition,lLength,pBuffer) ) 

#define IAsyncReader_Length(This,pTotal,pAvailable)	\
    ( (This)->lpVtbl -> Length(This,pTotal,pAvailable) ) 

#define IAsyncReader_BeginFlush(This)	\
    ( (This)->lpVtbl -> BeginFlush(This) ) 

#define IAsyncReader_EndFlush(This)	\
    ( (This)->lpVtbl -> EndFlush(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAsyncReader_INTERFACE_DEFINED__ */


#ifndef __IGraphVersion_INTERFACE_DEFINED__
#define __IGraphVersion_INTERFACE_DEFINED__

/* interface IGraphVersion */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IGraphVersion;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868ab-0ad4-11ce-b03a-0020af0ba770")
    IGraphVersion : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryVersion( 
            /* [annotation] */ 
            _Out_  LONG *pVersion) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGraphVersionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGraphVersion * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGraphVersion * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGraphVersion * This);
        
        DECLSPEC_XFGVIRT(IGraphVersion, QueryVersion)
        HRESULT ( STDMETHODCALLTYPE *QueryVersion )( 
            IGraphVersion * This,
            /* [annotation] */ 
            _Out_  LONG *pVersion);
        
        END_INTERFACE
    } IGraphVersionVtbl;

    interface IGraphVersion
    {
        CONST_VTBL struct IGraphVersionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGraphVersion_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGraphVersion_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGraphVersion_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGraphVersion_QueryVersion(This,pVersion)	\
    ( (This)->lpVtbl -> QueryVersion(This,pVersion) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGraphVersion_INTERFACE_DEFINED__ */


#ifndef __IResourceConsumer_INTERFACE_DEFINED__
#define __IResourceConsumer_INTERFACE_DEFINED__

/* interface IResourceConsumer */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IResourceConsumer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868ad-0ad4-11ce-b03a-0020af0ba770")
    IResourceConsumer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AcquireResource( 
            /* [in] */ LONG idResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseResource( 
            /* [in] */ LONG idResource) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceConsumerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IResourceConsumer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IResourceConsumer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IResourceConsumer * This);
        
        DECLSPEC_XFGVIRT(IResourceConsumer, AcquireResource)
        HRESULT ( STDMETHODCALLTYPE *AcquireResource )( 
            IResourceConsumer * This,
            /* [in] */ LONG idResource);
        
        DECLSPEC_XFGVIRT(IResourceConsumer, ReleaseResource)
        HRESULT ( STDMETHODCALLTYPE *ReleaseResource )( 
            IResourceConsumer * This,
            /* [in] */ LONG idResource);
        
        END_INTERFACE
    } IResourceConsumerVtbl;

    interface IResourceConsumer
    {
        CONST_VTBL struct IResourceConsumerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceConsumer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceConsumer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceConsumer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceConsumer_AcquireResource(This,idResource)	\
    ( (This)->lpVtbl -> AcquireResource(This,idResource) ) 

#define IResourceConsumer_ReleaseResource(This,idResource)	\
    ( (This)->lpVtbl -> ReleaseResource(This,idResource) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceConsumer_INTERFACE_DEFINED__ */


#ifndef __IResourceManager_INTERFACE_DEFINED__
#define __IResourceManager_INTERFACE_DEFINED__

/* interface IResourceManager */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IResourceManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868ac-0ad4-11ce-b03a-0020af0ba770")
    IResourceManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Register( 
            /* [in] */ LPCWSTR pName,
            /* [in] */ LONG cResource,
            /* [annotation][out] */ 
            _Out_  LONG *plToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterGroup( 
            /* [in] */ LPCWSTR pName,
            /* [in] */ LONG cResource,
            /* [annotation][size_is][in] */ 
            _In_reads_(cResource)  LONG *palTokens,
            /* [annotation][out] */ 
            _Out_  LONG *plToken) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RequestResource( 
            /* [in] */ LONG idResource,
            /* [in] */ IUnknown *pFocusObject,
            /* [in] */ IResourceConsumer *pConsumer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyAcquire( 
            /* [in] */ LONG idResource,
            /* [in] */ IResourceConsumer *pConsumer,
            /* [in] */ HRESULT hr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyRelease( 
            /* [in] */ LONG idResource,
            /* [in] */ IResourceConsumer *pConsumer,
            /* [in] */ BOOL bStillWant) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelRequest( 
            /* [in] */ LONG idResource,
            /* [in] */ IResourceConsumer *pConsumer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFocus( 
            /* [in] */ IUnknown *pFocusObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReleaseFocus( 
            /* [in] */ IUnknown *pFocusObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IResourceManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IResourceManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IResourceManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IResourceManager * This);
        
        DECLSPEC_XFGVIRT(IResourceManager, Register)
        HRESULT ( STDMETHODCALLTYPE *Register )( 
            IResourceManager * This,
            /* [in] */ LPCWSTR pName,
            /* [in] */ LONG cResource,
            /* [annotation][out] */ 
            _Out_  LONG *plToken);
        
        DECLSPEC_XFGVIRT(IResourceManager, RegisterGroup)
        HRESULT ( STDMETHODCALLTYPE *RegisterGroup )( 
            IResourceManager * This,
            /* [in] */ LPCWSTR pName,
            /* [in] */ LONG cResource,
            /* [annotation][size_is][in] */ 
            _In_reads_(cResource)  LONG *palTokens,
            /* [annotation][out] */ 
            _Out_  LONG *plToken);
        
        DECLSPEC_XFGVIRT(IResourceManager, RequestResource)
        HRESULT ( STDMETHODCALLTYPE *RequestResource )( 
            IResourceManager * This,
            /* [in] */ LONG idResource,
            /* [in] */ IUnknown *pFocusObject,
            /* [in] */ IResourceConsumer *pConsumer);
        
        DECLSPEC_XFGVIRT(IResourceManager, NotifyAcquire)
        HRESULT ( STDMETHODCALLTYPE *NotifyAcquire )( 
            IResourceManager * This,
            /* [in] */ LONG idResource,
            /* [in] */ IResourceConsumer *pConsumer,
            /* [in] */ HRESULT hr);
        
        DECLSPEC_XFGVIRT(IResourceManager, NotifyRelease)
        HRESULT ( STDMETHODCALLTYPE *NotifyRelease )( 
            IResourceManager * This,
            /* [in] */ LONG idResource,
            /* [in] */ IResourceConsumer *pConsumer,
            /* [in] */ BOOL bStillWant);
        
        DECLSPEC_XFGVIRT(IResourceManager, CancelRequest)
        HRESULT ( STDMETHODCALLTYPE *CancelRequest )( 
            IResourceManager * This,
            /* [in] */ LONG idResource,
            /* [in] */ IResourceConsumer *pConsumer);
        
        DECLSPEC_XFGVIRT(IResourceManager, SetFocus)
        HRESULT ( STDMETHODCALLTYPE *SetFocus )( 
            IResourceManager * This,
            /* [in] */ IUnknown *pFocusObject);
        
        DECLSPEC_XFGVIRT(IResourceManager, ReleaseFocus)
        HRESULT ( STDMETHODCALLTYPE *ReleaseFocus )( 
            IResourceManager * This,
            /* [in] */ IUnknown *pFocusObject);
        
        END_INTERFACE
    } IResourceManagerVtbl;

    interface IResourceManager
    {
        CONST_VTBL struct IResourceManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IResourceManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IResourceManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IResourceManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IResourceManager_Register(This,pName,cResource,plToken)	\
    ( (This)->lpVtbl -> Register(This,pName,cResource,plToken) ) 

#define IResourceManager_RegisterGroup(This,pName,cResource,palTokens,plToken)	\
    ( (This)->lpVtbl -> RegisterGroup(This,pName,cResource,palTokens,plToken) ) 

#define IResourceManager_RequestResource(This,idResource,pFocusObject,pConsumer)	\
    ( (This)->lpVtbl -> RequestResource(This,idResource,pFocusObject,pConsumer) ) 

#define IResourceManager_NotifyAcquire(This,idResource,pConsumer,hr)	\
    ( (This)->lpVtbl -> NotifyAcquire(This,idResource,pConsumer,hr) ) 

#define IResourceManager_NotifyRelease(This,idResource,pConsumer,bStillWant)	\
    ( (This)->lpVtbl -> NotifyRelease(This,idResource,pConsumer,bStillWant) ) 

#define IResourceManager_CancelRequest(This,idResource,pConsumer)	\
    ( (This)->lpVtbl -> CancelRequest(This,idResource,pConsumer) ) 

#define IResourceManager_SetFocus(This,pFocusObject)	\
    ( (This)->lpVtbl -> SetFocus(This,pFocusObject) ) 

#define IResourceManager_ReleaseFocus(This,pFocusObject)	\
    ( (This)->lpVtbl -> ReleaseFocus(This,pFocusObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IResourceManager_INTERFACE_DEFINED__ */


#ifndef __IDistributorNotify_INTERFACE_DEFINED__
#define __IDistributorNotify_INTERFACE_DEFINED__

/* interface IDistributorNotify */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDistributorNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56a868af-0ad4-11ce-b03a-0020af0ba770")
    IDistributorNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Run( 
            REFERENCE_TIME tStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSyncSource( 
            /* [in] */ IReferenceClock *pClock) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyGraphChange( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDistributorNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDistributorNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDistributorNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDistributorNotify * This);
        
        DECLSPEC_XFGVIRT(IDistributorNotify, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IDistributorNotify * This);
        
        DECLSPEC_XFGVIRT(IDistributorNotify, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IDistributorNotify * This);
        
        DECLSPEC_XFGVIRT(IDistributorNotify, Run)
        HRESULT ( STDMETHODCALLTYPE *Run )( 
            IDistributorNotify * This,
            REFERENCE_TIME tStart);
        
        DECLSPEC_XFGVIRT(IDistributorNotify, SetSyncSource)
        HRESULT ( STDMETHODCALLTYPE *SetSyncSource )( 
            IDistributorNotify * This,
            /* [in] */ IReferenceClock *pClock);
        
        DECLSPEC_XFGVIRT(IDistributorNotify, NotifyGraphChange)
        HRESULT ( STDMETHODCALLTYPE *NotifyGraphChange )( 
            IDistributorNotify * This);
        
        END_INTERFACE
    } IDistributorNotifyVtbl;

    interface IDistributorNotify
    {
        CONST_VTBL struct IDistributorNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDistributorNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDistributorNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDistributorNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDistributorNotify_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IDistributorNotify_Pause(This)	\
    ( (This)->lpVtbl -> Pause(This) ) 

#define IDistributorNotify_Run(This,tStart)	\
    ( (This)->lpVtbl -> Run(This,tStart) ) 

#define IDistributorNotify_SetSyncSource(This,pClock)	\
    ( (This)->lpVtbl -> SetSyncSource(This,pClock) ) 

#define IDistributorNotify_NotifyGraphChange(This)	\
    ( (This)->lpVtbl -> NotifyGraphChange(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDistributorNotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0045 */
/* [local] */ 

typedef 
enum AM_STREAM_INFO_FLAGS
    {
        AM_STREAM_INFO_START_DEFINED	= 0x1,
        AM_STREAM_INFO_STOP_DEFINED	= 0x2,
        AM_STREAM_INFO_DISCARDING	= 0x4,
        AM_STREAM_INFO_STOP_SEND_EXTRA	= 0x10
    } 	AM_STREAM_INFO_FLAGS;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct AM_STREAM_INFO
    {
    REFERENCE_TIME tStart;
    REFERENCE_TIME tStop;
    DWORD dwStartCookie;
    DWORD dwStopCookie;
    DWORD dwFlags;
    } 	AM_STREAM_INFO;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0045_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0045_v0_0_s_ifspec;

#ifndef __IAMStreamControl_INTERFACE_DEFINED__
#define __IAMStreamControl_INTERFACE_DEFINED__

/* interface IAMStreamControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMStreamControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36b73881-c2c8-11cf-8b46-00805f6cef60")
    IAMStreamControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartAt( 
            /* [annotation][in] */ 
            _In_opt_  const REFERENCE_TIME *ptStart,
            /* [in] */ DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopAt( 
            /* [annotation][in] */ 
            _In_opt_  const REFERENCE_TIME *ptStop,
            /* [in] */ BOOL bSendExtra,
            /* [in] */ DWORD dwCookie) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [annotation][out] */ 
            _Out_  AM_STREAM_INFO *pInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMStreamControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMStreamControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMStreamControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMStreamControl * This);
        
        DECLSPEC_XFGVIRT(IAMStreamControl, StartAt)
        HRESULT ( STDMETHODCALLTYPE *StartAt )( 
            IAMStreamControl * This,
            /* [annotation][in] */ 
            _In_opt_  const REFERENCE_TIME *ptStart,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(IAMStreamControl, StopAt)
        HRESULT ( STDMETHODCALLTYPE *StopAt )( 
            IAMStreamControl * This,
            /* [annotation][in] */ 
            _In_opt_  const REFERENCE_TIME *ptStop,
            /* [in] */ BOOL bSendExtra,
            /* [in] */ DWORD dwCookie);
        
        DECLSPEC_XFGVIRT(IAMStreamControl, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IAMStreamControl * This,
            /* [annotation][out] */ 
            _Out_  AM_STREAM_INFO *pInfo);
        
        END_INTERFACE
    } IAMStreamControlVtbl;

    interface IAMStreamControl
    {
        CONST_VTBL struct IAMStreamControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMStreamControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMStreamControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMStreamControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMStreamControl_StartAt(This,ptStart,dwCookie)	\
    ( (This)->lpVtbl -> StartAt(This,ptStart,dwCookie) ) 

#define IAMStreamControl_StopAt(This,ptStop,bSendExtra,dwCookie)	\
    ( (This)->lpVtbl -> StopAt(This,ptStop,bSendExtra,dwCookie) ) 

#define IAMStreamControl_GetInfo(This,pInfo)	\
    ( (This)->lpVtbl -> GetInfo(This,pInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMStreamControl_INTERFACE_DEFINED__ */


#ifndef __ISeekingPassThru_INTERFACE_DEFINED__
#define __ISeekingPassThru_INTERFACE_DEFINED__

/* interface ISeekingPassThru */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_ISeekingPassThru;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("36b73883-c2c8-11cf-8b46-00805f6cef60")
    ISeekingPassThru : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Init( 
            /* [in] */ BOOL bSupportRendering,
            /* [in] */ IPin *pPin) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct ISeekingPassThruVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            ISeekingPassThru * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            ISeekingPassThru * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            ISeekingPassThru * This);
        
        DECLSPEC_XFGVIRT(ISeekingPassThru, Init)
        HRESULT ( STDMETHODCALLTYPE *Init )( 
            ISeekingPassThru * This,
            /* [in] */ BOOL bSupportRendering,
            /* [in] */ IPin *pPin);
        
        END_INTERFACE
    } ISeekingPassThruVtbl;

    interface ISeekingPassThru
    {
        CONST_VTBL struct ISeekingPassThruVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define ISeekingPassThru_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define ISeekingPassThru_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define ISeekingPassThru_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define ISeekingPassThru_Init(This,bSupportRendering,pPin)	\
    ( (This)->lpVtbl -> Init(This,bSupportRendering,pPin) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __ISeekingPassThru_INTERFACE_DEFINED__ */


#ifndef __IAMStreamConfig_INTERFACE_DEFINED__
#define __IAMStreamConfig_INTERFACE_DEFINED__

/* interface IAMStreamConfig */
/* [unique][uuid][object][local] */ 

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct _VIDEO_STREAM_CONFIG_CAPS
    {
    GUID guid;
    ULONG VideoStandard;
    SIZE InputSize;
    SIZE MinCroppingSize;
    SIZE MaxCroppingSize;
    int CropGranularityX;
    int CropGranularityY;
    int CropAlignX;
    int CropAlignY;
    SIZE MinOutputSize;
    SIZE MaxOutputSize;
    int OutputGranularityX;
    int OutputGranularityY;
    int StretchTapsX;
    int StretchTapsY;
    int ShrinkTapsX;
    int ShrinkTapsY;
    LONGLONG MinFrameInterval;
    LONGLONG MaxFrameInterval;
    LONG MinBitsPerSecond;
    LONG MaxBitsPerSecond;
    } 	VIDEO_STREAM_CONFIG_CAPS;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
typedef struct _AUDIO_STREAM_CONFIG_CAPS
    {
    GUID guid;
    ULONG MinimumChannels;
    ULONG MaximumChannels;
    ULONG ChannelsGranularity;
    ULONG MinimumBitsPerSample;
    ULONG MaximumBitsPerSample;
    ULONG BitsPerSampleGranularity;
    ULONG MinimumSampleFrequency;
    ULONG MaximumSampleFrequency;
    ULONG SampleFrequencyGranularity;
    } 	AUDIO_STREAM_CONFIG_CAPS;


EXTERN_C const IID IID_IAMStreamConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13340-30AC-11d0-A18C-00A0C9118956")
    IAMStreamConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetFormat( 
            /* [in] */ AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFormat( 
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfCapabilities( 
            /* [annotation][out] */ 
            _Out_  int *piCount,
            /* [annotation][out] */ 
            _Out_  int *piSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamCaps( 
            /* [in] */ int iIndex,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppmt,
            /* [annotation][out] */ 
            _Out_  BYTE *pSCC) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMStreamConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMStreamConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMStreamConfig * This);
        
        DECLSPEC_XFGVIRT(IAMStreamConfig, SetFormat)
        HRESULT ( STDMETHODCALLTYPE *SetFormat )( 
            IAMStreamConfig * This,
            /* [in] */ AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IAMStreamConfig, GetFormat)
        HRESULT ( STDMETHODCALLTYPE *GetFormat )( 
            IAMStreamConfig * This,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppmt);
        
        DECLSPEC_XFGVIRT(IAMStreamConfig, GetNumberOfCapabilities)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfCapabilities )( 
            IAMStreamConfig * This,
            /* [annotation][out] */ 
            _Out_  int *piCount,
            /* [annotation][out] */ 
            _Out_  int *piSize);
        
        DECLSPEC_XFGVIRT(IAMStreamConfig, GetStreamCaps)
        HRESULT ( STDMETHODCALLTYPE *GetStreamCaps )( 
            IAMStreamConfig * This,
            /* [in] */ int iIndex,
            /* [annotation][out] */ 
            _Out_  AM_MEDIA_TYPE **ppmt,
            /* [annotation][out] */ 
            _Out_  BYTE *pSCC);
        
        END_INTERFACE
    } IAMStreamConfigVtbl;

    interface IAMStreamConfig
    {
        CONST_VTBL struct IAMStreamConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMStreamConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMStreamConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMStreamConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMStreamConfig_SetFormat(This,pmt)	\
    ( (This)->lpVtbl -> SetFormat(This,pmt) ) 

#define IAMStreamConfig_GetFormat(This,ppmt)	\
    ( (This)->lpVtbl -> GetFormat(This,ppmt) ) 

#define IAMStreamConfig_GetNumberOfCapabilities(This,piCount,piSize)	\
    ( (This)->lpVtbl -> GetNumberOfCapabilities(This,piCount,piSize) ) 

#define IAMStreamConfig_GetStreamCaps(This,iIndex,ppmt,pSCC)	\
    ( (This)->lpVtbl -> GetStreamCaps(This,iIndex,ppmt,pSCC) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMStreamConfig_INTERFACE_DEFINED__ */


#ifndef __IConfigInterleaving_INTERFACE_DEFINED__
#define __IConfigInterleaving_INTERFACE_DEFINED__

/* interface IConfigInterleaving */
/* [unique][uuid][object][local] */ 

typedef 
enum InterleavingMode
    {
        INTERLEAVE_NONE	= 0,
        INTERLEAVE_CAPTURE	= ( INTERLEAVE_NONE + 1 ) ,
        INTERLEAVE_FULL	= ( INTERLEAVE_CAPTURE + 1 ) ,
        INTERLEAVE_NONE_BUFFERED	= ( INTERLEAVE_FULL + 1 ) 
    } 	InterleavingMode;


EXTERN_C const IID IID_IConfigInterleaving;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("BEE3D220-157B-11d0-BD23-00A0C911CE86")
    IConfigInterleaving : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ InterleavingMode mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [annotation][out] */ 
            _Out_  InterleavingMode *pMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Interleaving( 
            /* [in] */ const REFERENCE_TIME *prtInterleave,
            /* [in] */ const REFERENCE_TIME *prtPreroll) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Interleaving( 
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtInterleave,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtPreroll) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConfigInterleavingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IConfigInterleaving * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IConfigInterleaving * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IConfigInterleaving * This);
        
        DECLSPEC_XFGVIRT(IConfigInterleaving, put_Mode)
        HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            IConfigInterleaving * This,
            /* [in] */ InterleavingMode mode);
        
        DECLSPEC_XFGVIRT(IConfigInterleaving, get_Mode)
        HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            IConfigInterleaving * This,
            /* [annotation][out] */ 
            _Out_  InterleavingMode *pMode);
        
        DECLSPEC_XFGVIRT(IConfigInterleaving, put_Interleaving)
        HRESULT ( STDMETHODCALLTYPE *put_Interleaving )( 
            IConfigInterleaving * This,
            /* [in] */ const REFERENCE_TIME *prtInterleave,
            /* [in] */ const REFERENCE_TIME *prtPreroll);
        
        DECLSPEC_XFGVIRT(IConfigInterleaving, get_Interleaving)
        HRESULT ( STDMETHODCALLTYPE *get_Interleaving )( 
            IConfigInterleaving * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtInterleave,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtPreroll);
        
        END_INTERFACE
    } IConfigInterleavingVtbl;

    interface IConfigInterleaving
    {
        CONST_VTBL struct IConfigInterleavingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConfigInterleaving_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConfigInterleaving_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConfigInterleaving_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConfigInterleaving_put_Mode(This,mode)	\
    ( (This)->lpVtbl -> put_Mode(This,mode) ) 

#define IConfigInterleaving_get_Mode(This,pMode)	\
    ( (This)->lpVtbl -> get_Mode(This,pMode) ) 

#define IConfigInterleaving_put_Interleaving(This,prtInterleave,prtPreroll)	\
    ( (This)->lpVtbl -> put_Interleaving(This,prtInterleave,prtPreroll) ) 

#define IConfigInterleaving_get_Interleaving(This,prtInterleave,prtPreroll)	\
    ( (This)->lpVtbl -> get_Interleaving(This,prtInterleave,prtPreroll) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConfigInterleaving_INTERFACE_DEFINED__ */


#ifndef __IConfigAviMux_INTERFACE_DEFINED__
#define __IConfigAviMux_INTERFACE_DEFINED__

/* interface IConfigAviMux */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IConfigAviMux;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5ACD6AA0-F482-11ce-8B67-00AA00A3F1A6")
    IConfigAviMux : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMasterStream( 
            /* [in] */ LONG iStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMasterStream( 
            /* [annotation][out] */ 
            _Out_  LONG *pStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputCompatibilityIndex( 
            /* [in] */ BOOL fOldIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputCompatibilityIndex( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfOldIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IConfigAviMuxVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IConfigAviMux * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IConfigAviMux * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IConfigAviMux * This);
        
        DECLSPEC_XFGVIRT(IConfigAviMux, SetMasterStream)
        HRESULT ( STDMETHODCALLTYPE *SetMasterStream )( 
            IConfigAviMux * This,
            /* [in] */ LONG iStream);
        
        DECLSPEC_XFGVIRT(IConfigAviMux, GetMasterStream)
        HRESULT ( STDMETHODCALLTYPE *GetMasterStream )( 
            IConfigAviMux * This,
            /* [annotation][out] */ 
            _Out_  LONG *pStream);
        
        DECLSPEC_XFGVIRT(IConfigAviMux, SetOutputCompatibilityIndex)
        HRESULT ( STDMETHODCALLTYPE *SetOutputCompatibilityIndex )( 
            IConfigAviMux * This,
            /* [in] */ BOOL fOldIndex);
        
        DECLSPEC_XFGVIRT(IConfigAviMux, GetOutputCompatibilityIndex)
        HRESULT ( STDMETHODCALLTYPE *GetOutputCompatibilityIndex )( 
            IConfigAviMux * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfOldIndex);
        
        END_INTERFACE
    } IConfigAviMuxVtbl;

    interface IConfigAviMux
    {
        CONST_VTBL struct IConfigAviMuxVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IConfigAviMux_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IConfigAviMux_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IConfigAviMux_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IConfigAviMux_SetMasterStream(This,iStream)	\
    ( (This)->lpVtbl -> SetMasterStream(This,iStream) ) 

#define IConfigAviMux_GetMasterStream(This,pStream)	\
    ( (This)->lpVtbl -> GetMasterStream(This,pStream) ) 

#define IConfigAviMux_SetOutputCompatibilityIndex(This,fOldIndex)	\
    ( (This)->lpVtbl -> SetOutputCompatibilityIndex(This,fOldIndex) ) 

#define IConfigAviMux_GetOutputCompatibilityIndex(This,pfOldIndex)	\
    ( (This)->lpVtbl -> GetOutputCompatibilityIndex(This,pfOldIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IConfigAviMux_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0050 */
/* [local] */ 

typedef 
enum CompressionCaps
    {
        CompressionCaps_CanQuality	= 0x1,
        CompressionCaps_CanCrunch	= 0x2,
        CompressionCaps_CanKeyFrame	= 0x4,
        CompressionCaps_CanBFrame	= 0x8,
        CompressionCaps_CanWindow	= 0x10
    } 	CompressionCaps;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0050_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0050_v0_0_s_ifspec;

#ifndef __IAMVideoCompression_INTERFACE_DEFINED__
#define __IAMVideoCompression_INTERFACE_DEFINED__

/* interface IAMVideoCompression */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMVideoCompression;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13343-30AC-11d0-A18C-00A0C9118956")
    IAMVideoCompression : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_KeyFrameRate( 
            /* [in] */ long KeyFrameRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_KeyFrameRate( 
            /* [annotation][out] */ 
            _Out_  long *pKeyFrameRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_PFramesPerKeyFrame( 
            /* [in] */ long PFramesPerKeyFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_PFramesPerKeyFrame( 
            /* [annotation][out] */ 
            _Out_  long *pPFramesPerKeyFrame) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Quality( 
            /* [in] */ double Quality) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Quality( 
            /* [annotation][out] */ 
            _Out_  double *pQuality) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_WindowSize( 
            /* [in] */ DWORDLONG WindowSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_WindowSize( 
            /* [annotation][out] */ 
            _Out_  DWORDLONG *pWindowSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_opt_(*pcbVersion)  LPWSTR pszVersion,
            /* [annotation][out][in] */ 
            _Inout_opt_  int *pcbVersion,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_opt_(*pcbDescription)  LPWSTR pszDescription,
            /* [annotation][out][in] */ 
            _Inout_opt_  int *pcbDescription,
            /* [annotation][out] */ 
            _Out_opt_  long *pDefaultKeyFrameRate,
            /* [annotation][out] */ 
            _Out_opt_  long *pDefaultPFramesPerKey,
            /* [annotation][out] */ 
            _Out_opt_  double *pDefaultQuality,
            /* [annotation][out] */ 
            _Out_opt_  long *pCapabilities) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OverrideKeyFrame( 
            /* [in] */ long FrameNumber) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OverrideFrameSize( 
            /* [in] */ long FrameNumber,
            /* [in] */ long Size) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVideoCompressionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVideoCompression * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVideoCompression * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVideoCompression * This);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, put_KeyFrameRate)
        HRESULT ( STDMETHODCALLTYPE *put_KeyFrameRate )( 
            IAMVideoCompression * This,
            /* [in] */ long KeyFrameRate);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, get_KeyFrameRate)
        HRESULT ( STDMETHODCALLTYPE *get_KeyFrameRate )( 
            IAMVideoCompression * This,
            /* [annotation][out] */ 
            _Out_  long *pKeyFrameRate);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, put_PFramesPerKeyFrame)
        HRESULT ( STDMETHODCALLTYPE *put_PFramesPerKeyFrame )( 
            IAMVideoCompression * This,
            /* [in] */ long PFramesPerKeyFrame);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, get_PFramesPerKeyFrame)
        HRESULT ( STDMETHODCALLTYPE *get_PFramesPerKeyFrame )( 
            IAMVideoCompression * This,
            /* [annotation][out] */ 
            _Out_  long *pPFramesPerKeyFrame);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, put_Quality)
        HRESULT ( STDMETHODCALLTYPE *put_Quality )( 
            IAMVideoCompression * This,
            /* [in] */ double Quality);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, get_Quality)
        HRESULT ( STDMETHODCALLTYPE *get_Quality )( 
            IAMVideoCompression * This,
            /* [annotation][out] */ 
            _Out_  double *pQuality);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, put_WindowSize)
        HRESULT ( STDMETHODCALLTYPE *put_WindowSize )( 
            IAMVideoCompression * This,
            /* [in] */ DWORDLONG WindowSize);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, get_WindowSize)
        HRESULT ( STDMETHODCALLTYPE *get_WindowSize )( 
            IAMVideoCompression * This,
            /* [annotation][out] */ 
            _Out_  DWORDLONG *pWindowSize);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IAMVideoCompression * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_opt_(*pcbVersion)  LPWSTR pszVersion,
            /* [annotation][out][in] */ 
            _Inout_opt_  int *pcbVersion,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_opt_(*pcbDescription)  LPWSTR pszDescription,
            /* [annotation][out][in] */ 
            _Inout_opt_  int *pcbDescription,
            /* [annotation][out] */ 
            _Out_opt_  long *pDefaultKeyFrameRate,
            /* [annotation][out] */ 
            _Out_opt_  long *pDefaultPFramesPerKey,
            /* [annotation][out] */ 
            _Out_opt_  double *pDefaultQuality,
            /* [annotation][out] */ 
            _Out_opt_  long *pCapabilities);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, OverrideKeyFrame)
        HRESULT ( STDMETHODCALLTYPE *OverrideKeyFrame )( 
            IAMVideoCompression * This,
            /* [in] */ long FrameNumber);
        
        DECLSPEC_XFGVIRT(IAMVideoCompression, OverrideFrameSize)
        HRESULT ( STDMETHODCALLTYPE *OverrideFrameSize )( 
            IAMVideoCompression * This,
            /* [in] */ long FrameNumber,
            /* [in] */ long Size);
        
        END_INTERFACE
    } IAMVideoCompressionVtbl;

    interface IAMVideoCompression
    {
        CONST_VTBL struct IAMVideoCompressionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVideoCompression_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVideoCompression_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVideoCompression_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVideoCompression_put_KeyFrameRate(This,KeyFrameRate)	\
    ( (This)->lpVtbl -> put_KeyFrameRate(This,KeyFrameRate) ) 

#define IAMVideoCompression_get_KeyFrameRate(This,pKeyFrameRate)	\
    ( (This)->lpVtbl -> get_KeyFrameRate(This,pKeyFrameRate) ) 

#define IAMVideoCompression_put_PFramesPerKeyFrame(This,PFramesPerKeyFrame)	\
    ( (This)->lpVtbl -> put_PFramesPerKeyFrame(This,PFramesPerKeyFrame) ) 

#define IAMVideoCompression_get_PFramesPerKeyFrame(This,pPFramesPerKeyFrame)	\
    ( (This)->lpVtbl -> get_PFramesPerKeyFrame(This,pPFramesPerKeyFrame) ) 

#define IAMVideoCompression_put_Quality(This,Quality)	\
    ( (This)->lpVtbl -> put_Quality(This,Quality) ) 

#define IAMVideoCompression_get_Quality(This,pQuality)	\
    ( (This)->lpVtbl -> get_Quality(This,pQuality) ) 

#define IAMVideoCompression_put_WindowSize(This,WindowSize)	\
    ( (This)->lpVtbl -> put_WindowSize(This,WindowSize) ) 

#define IAMVideoCompression_get_WindowSize(This,pWindowSize)	\
    ( (This)->lpVtbl -> get_WindowSize(This,pWindowSize) ) 

#define IAMVideoCompression_GetInfo(This,pszVersion,pcbVersion,pszDescription,pcbDescription,pDefaultKeyFrameRate,pDefaultPFramesPerKey,pDefaultQuality,pCapabilities)	\
    ( (This)->lpVtbl -> GetInfo(This,pszVersion,pcbVersion,pszDescription,pcbDescription,pDefaultKeyFrameRate,pDefaultPFramesPerKey,pDefaultQuality,pCapabilities) ) 

#define IAMVideoCompression_OverrideKeyFrame(This,FrameNumber)	\
    ( (This)->lpVtbl -> OverrideKeyFrame(This,FrameNumber) ) 

#define IAMVideoCompression_OverrideFrameSize(This,FrameNumber,Size)	\
    ( (This)->lpVtbl -> OverrideFrameSize(This,FrameNumber,Size) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVideoCompression_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0051 */
/* [local] */ 

typedef 
enum VfwCaptureDialogs
    {
        VfwCaptureDialog_Source	= 0x1,
        VfwCaptureDialog_Format	= 0x2,
        VfwCaptureDialog_Display	= 0x4
    } 	VfwCaptureDialogs;

typedef 
enum VfwCompressDialogs
    {
        VfwCompressDialog_Config	= 0x1,
        VfwCompressDialog_About	= 0x2,
        VfwCompressDialog_QueryConfig	= 0x4,
        VfwCompressDialog_QueryAbout	= 0x8
    } 	VfwCompressDialogs;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0051_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0051_v0_0_s_ifspec;

#ifndef __IAMVfwCaptureDialogs_INTERFACE_DEFINED__
#define __IAMVfwCaptureDialogs_INTERFACE_DEFINED__

/* interface IAMVfwCaptureDialogs */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMVfwCaptureDialogs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8D715A0-6E5E-11D0-B3F0-00AA003761C5")
    IAMVfwCaptureDialogs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE HasDialog( 
            /* [in] */ int iDialog) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowDialog( 
            /* [in] */ int iDialog,
            /* [in] */ HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendDriverMessage( 
            /* [in] */ int iDialog,
            /* [in] */ int uMsg,
            /* [in] */ long dw1,
            /* [in] */ long dw2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVfwCaptureDialogsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVfwCaptureDialogs * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVfwCaptureDialogs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVfwCaptureDialogs * This);
        
        DECLSPEC_XFGVIRT(IAMVfwCaptureDialogs, HasDialog)
        HRESULT ( STDMETHODCALLTYPE *HasDialog )( 
            IAMVfwCaptureDialogs * This,
            /* [in] */ int iDialog);
        
        DECLSPEC_XFGVIRT(IAMVfwCaptureDialogs, ShowDialog)
        HRESULT ( STDMETHODCALLTYPE *ShowDialog )( 
            IAMVfwCaptureDialogs * This,
            /* [in] */ int iDialog,
            /* [in] */ HWND hwnd);
        
        DECLSPEC_XFGVIRT(IAMVfwCaptureDialogs, SendDriverMessage)
        HRESULT ( STDMETHODCALLTYPE *SendDriverMessage )( 
            IAMVfwCaptureDialogs * This,
            /* [in] */ int iDialog,
            /* [in] */ int uMsg,
            /* [in] */ long dw1,
            /* [in] */ long dw2);
        
        END_INTERFACE
    } IAMVfwCaptureDialogsVtbl;

    interface IAMVfwCaptureDialogs
    {
        CONST_VTBL struct IAMVfwCaptureDialogsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVfwCaptureDialogs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVfwCaptureDialogs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVfwCaptureDialogs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVfwCaptureDialogs_HasDialog(This,iDialog)	\
    ( (This)->lpVtbl -> HasDialog(This,iDialog) ) 

#define IAMVfwCaptureDialogs_ShowDialog(This,iDialog,hwnd)	\
    ( (This)->lpVtbl -> ShowDialog(This,iDialog,hwnd) ) 

#define IAMVfwCaptureDialogs_SendDriverMessage(This,iDialog,uMsg,dw1,dw2)	\
    ( (This)->lpVtbl -> SendDriverMessage(This,iDialog,uMsg,dw1,dw2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVfwCaptureDialogs_INTERFACE_DEFINED__ */


#ifndef __IAMVfwCompressDialogs_INTERFACE_DEFINED__
#define __IAMVfwCompressDialogs_INTERFACE_DEFINED__

/* interface IAMVfwCompressDialogs */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMVfwCompressDialogs;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D8D715A3-6E5E-11D0-B3F0-00AA003761C5")
    IAMVfwCompressDialogs : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE ShowDialog( 
            /* [in] */ int iDialog,
            /* [in] */ HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(*pcbState, *pcbState)  LPVOID pState,
            /* [annotation][out][in] */ 
            _Inout_  int *pcbState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetState( 
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbState)  LPVOID pState,
            /* [in] */ int cbState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SendDriverMessage( 
            /* [in] */ int uMsg,
            /* [in] */ long dw1,
            /* [in] */ long dw2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVfwCompressDialogsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVfwCompressDialogs * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVfwCompressDialogs * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVfwCompressDialogs * This);
        
        DECLSPEC_XFGVIRT(IAMVfwCompressDialogs, ShowDialog)
        HRESULT ( STDMETHODCALLTYPE *ShowDialog )( 
            IAMVfwCompressDialogs * This,
            /* [in] */ int iDialog,
            /* [in] */ HWND hwnd);
        
        DECLSPEC_XFGVIRT(IAMVfwCompressDialogs, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IAMVfwCompressDialogs * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(*pcbState, *pcbState)  LPVOID pState,
            /* [annotation][out][in] */ 
            _Inout_  int *pcbState);
        
        DECLSPEC_XFGVIRT(IAMVfwCompressDialogs, SetState)
        HRESULT ( STDMETHODCALLTYPE *SetState )( 
            IAMVfwCompressDialogs * This,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbState)  LPVOID pState,
            /* [in] */ int cbState);
        
        DECLSPEC_XFGVIRT(IAMVfwCompressDialogs, SendDriverMessage)
        HRESULT ( STDMETHODCALLTYPE *SendDriverMessage )( 
            IAMVfwCompressDialogs * This,
            /* [in] */ int uMsg,
            /* [in] */ long dw1,
            /* [in] */ long dw2);
        
        END_INTERFACE
    } IAMVfwCompressDialogsVtbl;

    interface IAMVfwCompressDialogs
    {
        CONST_VTBL struct IAMVfwCompressDialogsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVfwCompressDialogs_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVfwCompressDialogs_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVfwCompressDialogs_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVfwCompressDialogs_ShowDialog(This,iDialog,hwnd)	\
    ( (This)->lpVtbl -> ShowDialog(This,iDialog,hwnd) ) 

#define IAMVfwCompressDialogs_GetState(This,pState,pcbState)	\
    ( (This)->lpVtbl -> GetState(This,pState,pcbState) ) 

#define IAMVfwCompressDialogs_SetState(This,pState,cbState)	\
    ( (This)->lpVtbl -> SetState(This,pState,cbState) ) 

#define IAMVfwCompressDialogs_SendDriverMessage(This,uMsg,dw1,dw2)	\
    ( (This)->lpVtbl -> SendDriverMessage(This,uMsg,dw1,dw2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVfwCompressDialogs_INTERFACE_DEFINED__ */


#ifndef __IAMDroppedFrames_INTERFACE_DEFINED__
#define __IAMDroppedFrames_INTERFACE_DEFINED__

/* interface IAMDroppedFrames */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMDroppedFrames;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13344-30AC-11d0-A18C-00A0C9118956")
    IAMDroppedFrames : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumDropped( 
            /* [annotation][out] */ 
            _Out_  long *plDropped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumNotDropped( 
            /* [annotation][out] */ 
            _Out_  long *plNotDropped) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDroppedInfo( 
            /* [in] */ long lSize,
            /* [annotation][out] */ 
            _Out_  long *plArray,
            /* [annotation][out] */ 
            _Out_  long *plNumCopied) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAverageFrameSize( 
            /* [annotation][out] */ 
            _Out_  long *plAverageSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMDroppedFramesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMDroppedFrames * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMDroppedFrames * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMDroppedFrames * This);
        
        DECLSPEC_XFGVIRT(IAMDroppedFrames, GetNumDropped)
        HRESULT ( STDMETHODCALLTYPE *GetNumDropped )( 
            IAMDroppedFrames * This,
            /* [annotation][out] */ 
            _Out_  long *plDropped);
        
        DECLSPEC_XFGVIRT(IAMDroppedFrames, GetNumNotDropped)
        HRESULT ( STDMETHODCALLTYPE *GetNumNotDropped )( 
            IAMDroppedFrames * This,
            /* [annotation][out] */ 
            _Out_  long *plNotDropped);
        
        DECLSPEC_XFGVIRT(IAMDroppedFrames, GetDroppedInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDroppedInfo )( 
            IAMDroppedFrames * This,
            /* [in] */ long lSize,
            /* [annotation][out] */ 
            _Out_  long *plArray,
            /* [annotation][out] */ 
            _Out_  long *plNumCopied);
        
        DECLSPEC_XFGVIRT(IAMDroppedFrames, GetAverageFrameSize)
        HRESULT ( STDMETHODCALLTYPE *GetAverageFrameSize )( 
            IAMDroppedFrames * This,
            /* [annotation][out] */ 
            _Out_  long *plAverageSize);
        
        END_INTERFACE
    } IAMDroppedFramesVtbl;

    interface IAMDroppedFrames
    {
        CONST_VTBL struct IAMDroppedFramesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMDroppedFrames_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMDroppedFrames_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMDroppedFrames_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMDroppedFrames_GetNumDropped(This,plDropped)	\
    ( (This)->lpVtbl -> GetNumDropped(This,plDropped) ) 

#define IAMDroppedFrames_GetNumNotDropped(This,plNotDropped)	\
    ( (This)->lpVtbl -> GetNumNotDropped(This,plNotDropped) ) 

#define IAMDroppedFrames_GetDroppedInfo(This,lSize,plArray,plNumCopied)	\
    ( (This)->lpVtbl -> GetDroppedInfo(This,lSize,plArray,plNumCopied) ) 

#define IAMDroppedFrames_GetAverageFrameSize(This,plAverageSize)	\
    ( (This)->lpVtbl -> GetAverageFrameSize(This,plAverageSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMDroppedFrames_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0054 */
/* [local] */ 

#define AMF_AUTOMATICGAIN -1.0


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0054_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0054_v0_0_s_ifspec;

#ifndef __IAMAudioInputMixer_INTERFACE_DEFINED__
#define __IAMAudioInputMixer_INTERFACE_DEFINED__

/* interface IAMAudioInputMixer */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMAudioInputMixer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("54C39221-8380-11d0-B3F0-00AA003761C5")
    IAMAudioInputMixer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_Enable( 
            /* [in] */ BOOL fEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Enable( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Mono( 
            /* [in] */ BOOL fMono) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Mono( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfMono) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_MixLevel( 
            /* [in] */ double Level) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_MixLevel( 
            /* [annotation][out] */ 
            _Out_  double *pLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Pan( 
            /* [in] */ double Pan) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Pan( 
            /* [annotation][out] */ 
            _Out_  double *pPan) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Loudness( 
            /* [in] */ BOOL fLoudness) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Loudness( 
            /* [annotation][out] */ 
            _Out_  BOOL *pfLoudness) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Treble( 
            /* [in] */ double Treble) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Treble( 
            /* [annotation][out] */ 
            _Out_  double *pTreble) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TrebleRange( 
            /* [annotation][out] */ 
            _Out_  double *pRange) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Bass( 
            /* [in] */ double Bass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Bass( 
            /* [annotation][out] */ 
            _Out_  double *pBass) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_BassRange( 
            /* [annotation][out] */ 
            _Out_  double *pRange) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMAudioInputMixerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMAudioInputMixer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMAudioInputMixer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMAudioInputMixer * This);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_Enable)
        HRESULT ( STDMETHODCALLTYPE *put_Enable )( 
            IAMAudioInputMixer * This,
            /* [in] */ BOOL fEnable);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_Enable)
        HRESULT ( STDMETHODCALLTYPE *get_Enable )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfEnable);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_Mono)
        HRESULT ( STDMETHODCALLTYPE *put_Mono )( 
            IAMAudioInputMixer * This,
            /* [in] */ BOOL fMono);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_Mono)
        HRESULT ( STDMETHODCALLTYPE *get_Mono )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfMono);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_MixLevel)
        HRESULT ( STDMETHODCALLTYPE *put_MixLevel )( 
            IAMAudioInputMixer * This,
            /* [in] */ double Level);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_MixLevel)
        HRESULT ( STDMETHODCALLTYPE *get_MixLevel )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  double *pLevel);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_Pan)
        HRESULT ( STDMETHODCALLTYPE *put_Pan )( 
            IAMAudioInputMixer * This,
            /* [in] */ double Pan);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_Pan)
        HRESULT ( STDMETHODCALLTYPE *get_Pan )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  double *pPan);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_Loudness)
        HRESULT ( STDMETHODCALLTYPE *put_Loudness )( 
            IAMAudioInputMixer * This,
            /* [in] */ BOOL fLoudness);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_Loudness)
        HRESULT ( STDMETHODCALLTYPE *get_Loudness )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  BOOL *pfLoudness);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_Treble)
        HRESULT ( STDMETHODCALLTYPE *put_Treble )( 
            IAMAudioInputMixer * This,
            /* [in] */ double Treble);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_Treble)
        HRESULT ( STDMETHODCALLTYPE *get_Treble )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  double *pTreble);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_TrebleRange)
        HRESULT ( STDMETHODCALLTYPE *get_TrebleRange )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  double *pRange);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, put_Bass)
        HRESULT ( STDMETHODCALLTYPE *put_Bass )( 
            IAMAudioInputMixer * This,
            /* [in] */ double Bass);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_Bass)
        HRESULT ( STDMETHODCALLTYPE *get_Bass )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  double *pBass);
        
        DECLSPEC_XFGVIRT(IAMAudioInputMixer, get_BassRange)
        HRESULT ( STDMETHODCALLTYPE *get_BassRange )( 
            IAMAudioInputMixer * This,
            /* [annotation][out] */ 
            _Out_  double *pRange);
        
        END_INTERFACE
    } IAMAudioInputMixerVtbl;

    interface IAMAudioInputMixer
    {
        CONST_VTBL struct IAMAudioInputMixerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMAudioInputMixer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMAudioInputMixer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMAudioInputMixer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMAudioInputMixer_put_Enable(This,fEnable)	\
    ( (This)->lpVtbl -> put_Enable(This,fEnable) ) 

#define IAMAudioInputMixer_get_Enable(This,pfEnable)	\
    ( (This)->lpVtbl -> get_Enable(This,pfEnable) ) 

#define IAMAudioInputMixer_put_Mono(This,fMono)	\
    ( (This)->lpVtbl -> put_Mono(This,fMono) ) 

#define IAMAudioInputMixer_get_Mono(This,pfMono)	\
    ( (This)->lpVtbl -> get_Mono(This,pfMono) ) 

#define IAMAudioInputMixer_put_MixLevel(This,Level)	\
    ( (This)->lpVtbl -> put_MixLevel(This,Level) ) 

#define IAMAudioInputMixer_get_MixLevel(This,pLevel)	\
    ( (This)->lpVtbl -> get_MixLevel(This,pLevel) ) 

#define IAMAudioInputMixer_put_Pan(This,Pan)	\
    ( (This)->lpVtbl -> put_Pan(This,Pan) ) 

#define IAMAudioInputMixer_get_Pan(This,pPan)	\
    ( (This)->lpVtbl -> get_Pan(This,pPan) ) 

#define IAMAudioInputMixer_put_Loudness(This,fLoudness)	\
    ( (This)->lpVtbl -> put_Loudness(This,fLoudness) ) 

#define IAMAudioInputMixer_get_Loudness(This,pfLoudness)	\
    ( (This)->lpVtbl -> get_Loudness(This,pfLoudness) ) 

#define IAMAudioInputMixer_put_Treble(This,Treble)	\
    ( (This)->lpVtbl -> put_Treble(This,Treble) ) 

#define IAMAudioInputMixer_get_Treble(This,pTreble)	\
    ( (This)->lpVtbl -> get_Treble(This,pTreble) ) 

#define IAMAudioInputMixer_get_TrebleRange(This,pRange)	\
    ( (This)->lpVtbl -> get_TrebleRange(This,pRange) ) 

#define IAMAudioInputMixer_put_Bass(This,Bass)	\
    ( (This)->lpVtbl -> put_Bass(This,Bass) ) 

#define IAMAudioInputMixer_get_Bass(This,pBass)	\
    ( (This)->lpVtbl -> get_Bass(This,pBass) ) 

#define IAMAudioInputMixer_get_BassRange(This,pRange)	\
    ( (This)->lpVtbl -> get_BassRange(This,pRange) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMAudioInputMixer_INTERFACE_DEFINED__ */


#ifndef __IAMBufferNegotiation_INTERFACE_DEFINED__
#define __IAMBufferNegotiation_INTERFACE_DEFINED__

/* interface IAMBufferNegotiation */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMBufferNegotiation;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("56ED71A0-AF5F-11D0-B3F0-00AA003761C5")
    IAMBufferNegotiation : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SuggestAllocatorProperties( 
            /* [in] */ const ALLOCATOR_PROPERTIES *pprop) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllocatorProperties( 
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pprop) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMBufferNegotiationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMBufferNegotiation * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMBufferNegotiation * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMBufferNegotiation * This);
        
        DECLSPEC_XFGVIRT(IAMBufferNegotiation, SuggestAllocatorProperties)
        HRESULT ( STDMETHODCALLTYPE *SuggestAllocatorProperties )( 
            IAMBufferNegotiation * This,
            /* [in] */ const ALLOCATOR_PROPERTIES *pprop);
        
        DECLSPEC_XFGVIRT(IAMBufferNegotiation, GetAllocatorProperties)
        HRESULT ( STDMETHODCALLTYPE *GetAllocatorProperties )( 
            IAMBufferNegotiation * This,
            /* [annotation][out] */ 
            _Out_  ALLOCATOR_PROPERTIES *pprop);
        
        END_INTERFACE
    } IAMBufferNegotiationVtbl;

    interface IAMBufferNegotiation
    {
        CONST_VTBL struct IAMBufferNegotiationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMBufferNegotiation_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMBufferNegotiation_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMBufferNegotiation_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMBufferNegotiation_SuggestAllocatorProperties(This,pprop)	\
    ( (This)->lpVtbl -> SuggestAllocatorProperties(This,pprop) ) 

#define IAMBufferNegotiation_GetAllocatorProperties(This,pprop)	\
    ( (This)->lpVtbl -> GetAllocatorProperties(This,pprop) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMBufferNegotiation_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0056 */
/* [local] */ 

#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(push)
#pragma warning(disable:4001) 
#pragma once
#pragma warning(pop)
#pragma warning(pop)
#pragma region Desktop Family
#pragma region Desktop Family
#pragma endregion
typedef /* [v1_enum] */ 
enum tagAnalogVideoStandard
    {
        AnalogVideo_None	= 0,
        AnalogVideo_NTSC_M	= 0x1,
        AnalogVideo_NTSC_M_J	= 0x2,
        AnalogVideo_NTSC_433	= 0x4,
        AnalogVideo_PAL_B	= 0x10,
        AnalogVideo_PAL_D	= 0x20,
        AnalogVideo_PAL_G	= 0x40,
        AnalogVideo_PAL_H	= 0x80,
        AnalogVideo_PAL_I	= 0x100,
        AnalogVideo_PAL_M	= 0x200,
        AnalogVideo_PAL_N	= 0x400,
        AnalogVideo_PAL_60	= 0x800,
        AnalogVideo_SECAM_B	= 0x1000,
        AnalogVideo_SECAM_D	= 0x2000,
        AnalogVideo_SECAM_G	= 0x4000,
        AnalogVideo_SECAM_H	= 0x8000,
        AnalogVideo_SECAM_K	= 0x10000,
        AnalogVideo_SECAM_K1	= 0x20000,
        AnalogVideo_SECAM_L	= 0x40000,
        AnalogVideo_SECAM_L1	= 0x80000,
        AnalogVideo_PAL_N_COMBO	= 0x100000,
        AnalogVideoMask_MCE_NTSC	= ( ( ( ( ( ( AnalogVideo_NTSC_M | AnalogVideo_NTSC_M_J )  | AnalogVideo_NTSC_433 )  | AnalogVideo_PAL_M )  | AnalogVideo_PAL_N )  | AnalogVideo_PAL_60 )  | AnalogVideo_PAL_N_COMBO ) ,
        AnalogVideoMask_MCE_PAL	= ( ( ( ( AnalogVideo_PAL_B | AnalogVideo_PAL_D )  | AnalogVideo_PAL_G )  | AnalogVideo_PAL_H )  | AnalogVideo_PAL_I ) ,
        AnalogVideoMask_MCE_SECAM	= ( ( ( ( ( ( ( AnalogVideo_SECAM_B | AnalogVideo_SECAM_D )  | AnalogVideo_SECAM_G )  | AnalogVideo_SECAM_H )  | AnalogVideo_SECAM_K )  | AnalogVideo_SECAM_K1 )  | AnalogVideo_SECAM_L )  | AnalogVideo_SECAM_L1 ) 
    } 	AnalogVideoStandard;

typedef 
enum tagTunerInputType
    {
        TunerInputCable	= 0,
        TunerInputAntenna	= ( TunerInputCable + 1 ) 
    } 	TunerInputType;

#pragma region Desktop Family
#pragma endregion
#pragma endregion
#define AnalogVideo_NTSC_Mask  0x00000007
#define AnalogVideo_PAL_Mask   0x00100FF0
#define AnalogVideo_SECAM_Mask 0x000FF000
typedef 
enum VideoCopyProtectionType
    {
        VideoCopyProtectionMacrovisionBasic	= 0,
        VideoCopyProtectionMacrovisionCBI	= ( VideoCopyProtectionMacrovisionBasic + 1 ) 
    } 	VideoCopyProtectionType;

typedef 
enum tagPhysicalConnectorType
    {
        PhysConn_Video_Tuner	= 1,
        PhysConn_Video_Composite	= ( PhysConn_Video_Tuner + 1 ) ,
        PhysConn_Video_SVideo	= ( PhysConn_Video_Composite + 1 ) ,
        PhysConn_Video_RGB	= ( PhysConn_Video_SVideo + 1 ) ,
        PhysConn_Video_YRYBY	= ( PhysConn_Video_RGB + 1 ) ,
        PhysConn_Video_SerialDigital	= ( PhysConn_Video_YRYBY + 1 ) ,
        PhysConn_Video_ParallelDigital	= ( PhysConn_Video_SerialDigital + 1 ) ,
        PhysConn_Video_SCSI	= ( PhysConn_Video_ParallelDigital + 1 ) ,
        PhysConn_Video_AUX	= ( PhysConn_Video_SCSI + 1 ) ,
        PhysConn_Video_1394	= ( PhysConn_Video_AUX + 1 ) ,
        PhysConn_Video_USB	= ( PhysConn_Video_1394 + 1 ) ,
        PhysConn_Video_VideoDecoder	= ( PhysConn_Video_USB + 1 ) ,
        PhysConn_Video_VideoEncoder	= ( PhysConn_Video_VideoDecoder + 1 ) ,
        PhysConn_Video_SCART	= ( PhysConn_Video_VideoEncoder + 1 ) ,
        PhysConn_Video_Black	= ( PhysConn_Video_SCART + 1 ) ,
        PhysConn_Audio_Tuner	= 0x1000,
        PhysConn_Audio_Line	= ( PhysConn_Audio_Tuner + 1 ) ,
        PhysConn_Audio_Mic	= ( PhysConn_Audio_Line + 1 ) ,
        PhysConn_Audio_AESDigital	= ( PhysConn_Audio_Mic + 1 ) ,
        PhysConn_Audio_SPDIFDigital	= ( PhysConn_Audio_AESDigital + 1 ) ,
        PhysConn_Audio_SCSI	= ( PhysConn_Audio_SPDIFDigital + 1 ) ,
        PhysConn_Audio_AUX	= ( PhysConn_Audio_SCSI + 1 ) ,
        PhysConn_Audio_1394	= ( PhysConn_Audio_AUX + 1 ) ,
        PhysConn_Audio_USB	= ( PhysConn_Audio_1394 + 1 ) ,
        PhysConn_Audio_AudioDecoder	= ( PhysConn_Audio_USB + 1 ) 
    } 	PhysicalConnectorType;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0056_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0056_v0_0_s_ifspec;

#ifndef __IAMAnalogVideoDecoder_INTERFACE_DEFINED__
#define __IAMAnalogVideoDecoder_INTERFACE_DEFINED__

/* interface IAMAnalogVideoDecoder */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMAnalogVideoDecoder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13350-30AC-11d0-A18C-00A0C9118956")
    IAMAnalogVideoDecoder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AvailableTVFormats( 
            /* [annotation][out] */ 
            _Out_  long *lAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_TVFormat( 
            /* [in] */ long lAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TVFormat( 
            /* [annotation][out] */ 
            _Out_  long *plAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_HorizontalLocked( 
            /* [annotation][out] */ 
            _Out_  long *plLocked) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_VCRHorizontalLocking( 
            /* [in] */ long lVCRHorizontalLocking) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_VCRHorizontalLocking( 
            /* [annotation][out] */ 
            _Out_  long *plVCRHorizontalLocking) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NumberOfLines( 
            /* [annotation][out] */ 
            _Out_  long *plNumberOfLines) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_OutputEnable( 
            /* [in] */ long lOutputEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_OutputEnable( 
            /* [annotation][out] */ 
            _Out_  long *plOutputEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMAnalogVideoDecoderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMAnalogVideoDecoder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMAnalogVideoDecoder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMAnalogVideoDecoder * This);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, get_AvailableTVFormats)
        HRESULT ( STDMETHODCALLTYPE *get_AvailableTVFormats )( 
            IAMAnalogVideoDecoder * This,
            /* [annotation][out] */ 
            _Out_  long *lAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, put_TVFormat)
        HRESULT ( STDMETHODCALLTYPE *put_TVFormat )( 
            IAMAnalogVideoDecoder * This,
            /* [in] */ long lAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, get_TVFormat)
        HRESULT ( STDMETHODCALLTYPE *get_TVFormat )( 
            IAMAnalogVideoDecoder * This,
            /* [annotation][out] */ 
            _Out_  long *plAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, get_HorizontalLocked)
        HRESULT ( STDMETHODCALLTYPE *get_HorizontalLocked )( 
            IAMAnalogVideoDecoder * This,
            /* [annotation][out] */ 
            _Out_  long *plLocked);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, put_VCRHorizontalLocking)
        HRESULT ( STDMETHODCALLTYPE *put_VCRHorizontalLocking )( 
            IAMAnalogVideoDecoder * This,
            /* [in] */ long lVCRHorizontalLocking);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, get_VCRHorizontalLocking)
        HRESULT ( STDMETHODCALLTYPE *get_VCRHorizontalLocking )( 
            IAMAnalogVideoDecoder * This,
            /* [annotation][out] */ 
            _Out_  long *plVCRHorizontalLocking);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, get_NumberOfLines)
        HRESULT ( STDMETHODCALLTYPE *get_NumberOfLines )( 
            IAMAnalogVideoDecoder * This,
            /* [annotation][out] */ 
            _Out_  long *plNumberOfLines);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, put_OutputEnable)
        HRESULT ( STDMETHODCALLTYPE *put_OutputEnable )( 
            IAMAnalogVideoDecoder * This,
            /* [in] */ long lOutputEnable);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoDecoder, get_OutputEnable)
        HRESULT ( STDMETHODCALLTYPE *get_OutputEnable )( 
            IAMAnalogVideoDecoder * This,
            /* [annotation][out] */ 
            _Out_  long *plOutputEnable);
        
        END_INTERFACE
    } IAMAnalogVideoDecoderVtbl;

    interface IAMAnalogVideoDecoder
    {
        CONST_VTBL struct IAMAnalogVideoDecoderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMAnalogVideoDecoder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMAnalogVideoDecoder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMAnalogVideoDecoder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMAnalogVideoDecoder_get_AvailableTVFormats(This,lAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_AvailableTVFormats(This,lAnalogVideoStandard) ) 

#define IAMAnalogVideoDecoder_put_TVFormat(This,lAnalogVideoStandard)	\
    ( (This)->lpVtbl -> put_TVFormat(This,lAnalogVideoStandard) ) 

#define IAMAnalogVideoDecoder_get_TVFormat(This,plAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_TVFormat(This,plAnalogVideoStandard) ) 

#define IAMAnalogVideoDecoder_get_HorizontalLocked(This,plLocked)	\
    ( (This)->lpVtbl -> get_HorizontalLocked(This,plLocked) ) 

#define IAMAnalogVideoDecoder_put_VCRHorizontalLocking(This,lVCRHorizontalLocking)	\
    ( (This)->lpVtbl -> put_VCRHorizontalLocking(This,lVCRHorizontalLocking) ) 

#define IAMAnalogVideoDecoder_get_VCRHorizontalLocking(This,plVCRHorizontalLocking)	\
    ( (This)->lpVtbl -> get_VCRHorizontalLocking(This,plVCRHorizontalLocking) ) 

#define IAMAnalogVideoDecoder_get_NumberOfLines(This,plNumberOfLines)	\
    ( (This)->lpVtbl -> get_NumberOfLines(This,plNumberOfLines) ) 

#define IAMAnalogVideoDecoder_put_OutputEnable(This,lOutputEnable)	\
    ( (This)->lpVtbl -> put_OutputEnable(This,lOutputEnable) ) 

#define IAMAnalogVideoDecoder_get_OutputEnable(This,plOutputEnable)	\
    ( (This)->lpVtbl -> get_OutputEnable(This,plOutputEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMAnalogVideoDecoder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0057 */
/* [local] */ 

typedef 
enum tagVideoProcAmpProperty
    {
        VideoProcAmp_Brightness	= 0,
        VideoProcAmp_Contrast	= ( VideoProcAmp_Brightness + 1 ) ,
        VideoProcAmp_Hue	= ( VideoProcAmp_Contrast + 1 ) ,
        VideoProcAmp_Saturation	= ( VideoProcAmp_Hue + 1 ) ,
        VideoProcAmp_Sharpness	= ( VideoProcAmp_Saturation + 1 ) ,
        VideoProcAmp_Gamma	= ( VideoProcAmp_Sharpness + 1 ) ,
        VideoProcAmp_ColorEnable	= ( VideoProcAmp_Gamma + 1 ) ,
        VideoProcAmp_WhiteBalance	= ( VideoProcAmp_ColorEnable + 1 ) ,
        VideoProcAmp_BacklightCompensation	= ( VideoProcAmp_WhiteBalance + 1 ) ,
        VideoProcAmp_Gain	= ( VideoProcAmp_BacklightCompensation + 1 ) 
    } 	VideoProcAmpProperty;

typedef 
enum tagVideoProcAmpFlags
    {
        VideoProcAmp_Flags_Auto	= 0x1,
        VideoProcAmp_Flags_Manual	= 0x2
    } 	VideoProcAmpFlags;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0057_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0057_v0_0_s_ifspec;

#ifndef __IAMVideoProcAmp_INTERFACE_DEFINED__
#define __IAMVideoProcAmp_INTERFACE_DEFINED__

/* interface IAMVideoProcAmp */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMVideoProcAmp;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13360-30AC-11d0-A18C-00A0C9118956")
    IAMVideoProcAmp : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRange( 
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *pMin,
            /* [annotation][out] */ 
            _Out_  long *pMax,
            /* [annotation][out] */ 
            _Out_  long *pSteppingDelta,
            /* [annotation][out] */ 
            _Out_  long *pDefault,
            /* [annotation][out] */ 
            _Out_  long *pCapsFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Set( 
            /* [in] */ long Property,
            /* [in] */ long lValue,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *lValue,
            /* [annotation][out] */ 
            _Out_  long *Flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVideoProcAmpVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVideoProcAmp * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVideoProcAmp * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVideoProcAmp * This);
        
        DECLSPEC_XFGVIRT(IAMVideoProcAmp, GetRange)
        HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            IAMVideoProcAmp * This,
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *pMin,
            /* [annotation][out] */ 
            _Out_  long *pMax,
            /* [annotation][out] */ 
            _Out_  long *pSteppingDelta,
            /* [annotation][out] */ 
            _Out_  long *pDefault,
            /* [annotation][out] */ 
            _Out_  long *pCapsFlags);
        
        DECLSPEC_XFGVIRT(IAMVideoProcAmp, Set)
        HRESULT ( STDMETHODCALLTYPE *Set )( 
            IAMVideoProcAmp * This,
            /* [in] */ long Property,
            /* [in] */ long lValue,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IAMVideoProcAmp, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            IAMVideoProcAmp * This,
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *lValue,
            /* [annotation][out] */ 
            _Out_  long *Flags);
        
        END_INTERFACE
    } IAMVideoProcAmpVtbl;

    interface IAMVideoProcAmp
    {
        CONST_VTBL struct IAMVideoProcAmpVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVideoProcAmp_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVideoProcAmp_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVideoProcAmp_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVideoProcAmp_GetRange(This,Property,pMin,pMax,pSteppingDelta,pDefault,pCapsFlags)	\
    ( (This)->lpVtbl -> GetRange(This,Property,pMin,pMax,pSteppingDelta,pDefault,pCapsFlags) ) 

#define IAMVideoProcAmp_Set(This,Property,lValue,Flags)	\
    ( (This)->lpVtbl -> Set(This,Property,lValue,Flags) ) 

#define IAMVideoProcAmp_Get(This,Property,lValue,Flags)	\
    ( (This)->lpVtbl -> Get(This,Property,lValue,Flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVideoProcAmp_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0058 */
/* [local] */ 

typedef 
enum tagCameraControlProperty
    {
        CameraControl_Pan	= 0,
        CameraControl_Tilt	= ( CameraControl_Pan + 1 ) ,
        CameraControl_Roll	= ( CameraControl_Tilt + 1 ) ,
        CameraControl_Zoom	= ( CameraControl_Roll + 1 ) ,
        CameraControl_Exposure	= ( CameraControl_Zoom + 1 ) ,
        CameraControl_Iris	= ( CameraControl_Exposure + 1 ) ,
        CameraControl_Focus	= ( CameraControl_Iris + 1 ) 
    } 	CameraControlProperty;

typedef 
enum tagCameraControlFlags
    {
        CameraControl_Flags_Auto	= 0x1,
        CameraControl_Flags_Manual	= 0x2
    } 	CameraControlFlags;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0058_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0058_v0_0_s_ifspec;

#ifndef __IAMCameraControl_INTERFACE_DEFINED__
#define __IAMCameraControl_INTERFACE_DEFINED__

/* interface IAMCameraControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMCameraControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13370-30AC-11d0-A18C-00A0C9118956")
    IAMCameraControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetRange( 
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *pMin,
            /* [annotation][out] */ 
            _Out_  long *pMax,
            /* [annotation][out] */ 
            _Out_  long *pSteppingDelta,
            /* [annotation][out] */ 
            _Out_  long *pDefault,
            /* [annotation][out] */ 
            _Out_  long *pCapsFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Set( 
            /* [in] */ long Property,
            /* [in] */ long lValue,
            /* [in] */ long Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *lValue,
            /* [annotation][out] */ 
            _Out_  long *Flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMCameraControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMCameraControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMCameraControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMCameraControl * This);
        
        DECLSPEC_XFGVIRT(IAMCameraControl, GetRange)
        HRESULT ( STDMETHODCALLTYPE *GetRange )( 
            IAMCameraControl * This,
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *pMin,
            /* [annotation][out] */ 
            _Out_  long *pMax,
            /* [annotation][out] */ 
            _Out_  long *pSteppingDelta,
            /* [annotation][out] */ 
            _Out_  long *pDefault,
            /* [annotation][out] */ 
            _Out_  long *pCapsFlags);
        
        DECLSPEC_XFGVIRT(IAMCameraControl, Set)
        HRESULT ( STDMETHODCALLTYPE *Set )( 
            IAMCameraControl * This,
            /* [in] */ long Property,
            /* [in] */ long lValue,
            /* [in] */ long Flags);
        
        DECLSPEC_XFGVIRT(IAMCameraControl, Get)
        HRESULT ( STDMETHODCALLTYPE *Get )( 
            IAMCameraControl * This,
            /* [in] */ long Property,
            /* [annotation][out] */ 
            _Out_  long *lValue,
            /* [annotation][out] */ 
            _Out_  long *Flags);
        
        END_INTERFACE
    } IAMCameraControlVtbl;

    interface IAMCameraControl
    {
        CONST_VTBL struct IAMCameraControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMCameraControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMCameraControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMCameraControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMCameraControl_GetRange(This,Property,pMin,pMax,pSteppingDelta,pDefault,pCapsFlags)	\
    ( (This)->lpVtbl -> GetRange(This,Property,pMin,pMax,pSteppingDelta,pDefault,pCapsFlags) ) 

#define IAMCameraControl_Set(This,Property,lValue,Flags)	\
    ( (This)->lpVtbl -> Set(This,Property,lValue,Flags) ) 

#define IAMCameraControl_Get(This,Property,lValue,Flags)	\
    ( (This)->lpVtbl -> Get(This,Property,lValue,Flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMCameraControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0059 */
/* [local] */ 

typedef 
enum tagVideoControlFlags
    {
        VideoControlFlag_FlipHorizontal	= 0x1,
        VideoControlFlag_FlipVertical	= 0x2,
        VideoControlFlag_ExternalTriggerEnable	= 0x4,
        VideoControlFlag_Trigger	= 0x8
    } 	VideoControlFlags;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0059_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0059_v0_0_s_ifspec;

#ifndef __IAMVideoControl_INTERFACE_DEFINED__
#define __IAMVideoControl_INTERFACE_DEFINED__

/* interface IAMVideoControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMVideoControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6a2e0670-28e4-11d0-a18c-00a0c9118956")
    IAMVideoControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCaps( 
            /* [in] */ IPin *pPin,
            /* [annotation][out] */ 
            _Out_  long *pCapsFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMode( 
            /* [in] */ IPin *pPin,
            /* [in] */ long Mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMode( 
            /* [in] */ IPin *pPin,
            /* [annotation][out] */ 
            _Out_  long *Mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentActualFrameRate( 
            /* [in] */ IPin *pPin,
            /* [annotation][out] */ 
            _Out_  LONGLONG *ActualFrameRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxAvailableFrameRate( 
            /* [in] */ IPin *pPin,
            /* [in] */ long iIndex,
            /* [in] */ SIZE Dimensions,
            /* [annotation][out] */ 
            _Out_  LONGLONG *MaxAvailableFrameRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFrameRateList( 
            /* [in] */ IPin *pPin,
            /* [in] */ long iIndex,
            /* [in] */ SIZE Dimensions,
            /* [annotation][out] */ 
            _Out_  long *ListSize,
            /* [annotation][out] */ 
            _Out_  LONGLONG **FrameRates) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVideoControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVideoControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVideoControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVideoControl * This);
        
        DECLSPEC_XFGVIRT(IAMVideoControl, GetCaps)
        HRESULT ( STDMETHODCALLTYPE *GetCaps )( 
            IAMVideoControl * This,
            /* [in] */ IPin *pPin,
            /* [annotation][out] */ 
            _Out_  long *pCapsFlags);
        
        DECLSPEC_XFGVIRT(IAMVideoControl, SetMode)
        HRESULT ( STDMETHODCALLTYPE *SetMode )( 
            IAMVideoControl * This,
            /* [in] */ IPin *pPin,
            /* [in] */ long Mode);
        
        DECLSPEC_XFGVIRT(IAMVideoControl, GetMode)
        HRESULT ( STDMETHODCALLTYPE *GetMode )( 
            IAMVideoControl * This,
            /* [in] */ IPin *pPin,
            /* [annotation][out] */ 
            _Out_  long *Mode);
        
        DECLSPEC_XFGVIRT(IAMVideoControl, GetCurrentActualFrameRate)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentActualFrameRate )( 
            IAMVideoControl * This,
            /* [in] */ IPin *pPin,
            /* [annotation][out] */ 
            _Out_  LONGLONG *ActualFrameRate);
        
        DECLSPEC_XFGVIRT(IAMVideoControl, GetMaxAvailableFrameRate)
        HRESULT ( STDMETHODCALLTYPE *GetMaxAvailableFrameRate )( 
            IAMVideoControl * This,
            /* [in] */ IPin *pPin,
            /* [in] */ long iIndex,
            /* [in] */ SIZE Dimensions,
            /* [annotation][out] */ 
            _Out_  LONGLONG *MaxAvailableFrameRate);
        
        DECLSPEC_XFGVIRT(IAMVideoControl, GetFrameRateList)
        HRESULT ( STDMETHODCALLTYPE *GetFrameRateList )( 
            IAMVideoControl * This,
            /* [in] */ IPin *pPin,
            /* [in] */ long iIndex,
            /* [in] */ SIZE Dimensions,
            /* [annotation][out] */ 
            _Out_  long *ListSize,
            /* [annotation][out] */ 
            _Out_  LONGLONG **FrameRates);
        
        END_INTERFACE
    } IAMVideoControlVtbl;

    interface IAMVideoControl
    {
        CONST_VTBL struct IAMVideoControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVideoControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVideoControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVideoControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVideoControl_GetCaps(This,pPin,pCapsFlags)	\
    ( (This)->lpVtbl -> GetCaps(This,pPin,pCapsFlags) ) 

#define IAMVideoControl_SetMode(This,pPin,Mode)	\
    ( (This)->lpVtbl -> SetMode(This,pPin,Mode) ) 

#define IAMVideoControl_GetMode(This,pPin,Mode)	\
    ( (This)->lpVtbl -> GetMode(This,pPin,Mode) ) 

#define IAMVideoControl_GetCurrentActualFrameRate(This,pPin,ActualFrameRate)	\
    ( (This)->lpVtbl -> GetCurrentActualFrameRate(This,pPin,ActualFrameRate) ) 

#define IAMVideoControl_GetMaxAvailableFrameRate(This,pPin,iIndex,Dimensions,MaxAvailableFrameRate)	\
    ( (This)->lpVtbl -> GetMaxAvailableFrameRate(This,pPin,iIndex,Dimensions,MaxAvailableFrameRate) ) 

#define IAMVideoControl_GetFrameRateList(This,pPin,iIndex,Dimensions,ListSize,FrameRates)	\
    ( (This)->lpVtbl -> GetFrameRateList(This,pPin,iIndex,Dimensions,ListSize,FrameRates) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVideoControl_INTERFACE_DEFINED__ */


#ifndef __IAMCrossbar_INTERFACE_DEFINED__
#define __IAMCrossbar_INTERFACE_DEFINED__

/* interface IAMCrossbar */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMCrossbar;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E13380-30AC-11d0-A18C-00A0C9118956")
    IAMCrossbar : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_PinCounts( 
            /* [annotation][out] */ 
            _Out_  long *OutputPinCount,
            /* [annotation][out] */ 
            _Out_  long *InputPinCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanRoute( 
            /* [in] */ long OutputPinIndex,
            /* [in] */ long InputPinIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Route( 
            /* [in] */ long OutputPinIndex,
            /* [in] */ long InputPinIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_IsRoutedTo( 
            /* [in] */ long OutputPinIndex,
            /* [annotation][out] */ 
            _Out_  long *InputPinIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_CrossbarPinInfo( 
            /* [in] */ BOOL IsInputPin,
            /* [in] */ long PinIndex,
            /* [annotation][out] */ 
            _Out_  long *PinIndexRelated,
            /* [annotation][out] */ 
            _Out_  long *PhysicalType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMCrossbarVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMCrossbar * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMCrossbar * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMCrossbar * This);
        
        DECLSPEC_XFGVIRT(IAMCrossbar, get_PinCounts)
        HRESULT ( STDMETHODCALLTYPE *get_PinCounts )( 
            IAMCrossbar * This,
            /* [annotation][out] */ 
            _Out_  long *OutputPinCount,
            /* [annotation][out] */ 
            _Out_  long *InputPinCount);
        
        DECLSPEC_XFGVIRT(IAMCrossbar, CanRoute)
        HRESULT ( STDMETHODCALLTYPE *CanRoute )( 
            IAMCrossbar * This,
            /* [in] */ long OutputPinIndex,
            /* [in] */ long InputPinIndex);
        
        DECLSPEC_XFGVIRT(IAMCrossbar, Route)
        HRESULT ( STDMETHODCALLTYPE *Route )( 
            IAMCrossbar * This,
            /* [in] */ long OutputPinIndex,
            /* [in] */ long InputPinIndex);
        
        DECLSPEC_XFGVIRT(IAMCrossbar, get_IsRoutedTo)
        HRESULT ( STDMETHODCALLTYPE *get_IsRoutedTo )( 
            IAMCrossbar * This,
            /* [in] */ long OutputPinIndex,
            /* [annotation][out] */ 
            _Out_  long *InputPinIndex);
        
        DECLSPEC_XFGVIRT(IAMCrossbar, get_CrossbarPinInfo)
        HRESULT ( STDMETHODCALLTYPE *get_CrossbarPinInfo )( 
            IAMCrossbar * This,
            /* [in] */ BOOL IsInputPin,
            /* [in] */ long PinIndex,
            /* [annotation][out] */ 
            _Out_  long *PinIndexRelated,
            /* [annotation][out] */ 
            _Out_  long *PhysicalType);
        
        END_INTERFACE
    } IAMCrossbarVtbl;

    interface IAMCrossbar
    {
        CONST_VTBL struct IAMCrossbarVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMCrossbar_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMCrossbar_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMCrossbar_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMCrossbar_get_PinCounts(This,OutputPinCount,InputPinCount)	\
    ( (This)->lpVtbl -> get_PinCounts(This,OutputPinCount,InputPinCount) ) 

#define IAMCrossbar_CanRoute(This,OutputPinIndex,InputPinIndex)	\
    ( (This)->lpVtbl -> CanRoute(This,OutputPinIndex,InputPinIndex) ) 

#define IAMCrossbar_Route(This,OutputPinIndex,InputPinIndex)	\
    ( (This)->lpVtbl -> Route(This,OutputPinIndex,InputPinIndex) ) 

#define IAMCrossbar_get_IsRoutedTo(This,OutputPinIndex,InputPinIndex)	\
    ( (This)->lpVtbl -> get_IsRoutedTo(This,OutputPinIndex,InputPinIndex) ) 

#define IAMCrossbar_get_CrossbarPinInfo(This,IsInputPin,PinIndex,PinIndexRelated,PhysicalType)	\
    ( (This)->lpVtbl -> get_CrossbarPinInfo(This,IsInputPin,PinIndex,PinIndexRelated,PhysicalType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMCrossbar_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0061 */
/* [local] */ 

typedef 
enum tagAMTunerSubChannel
    {
        AMTUNER_SUBCHAN_NO_TUNE	= -2,
        AMTUNER_SUBCHAN_DEFAULT	= -1
    } 	AMTunerSubChannel;

typedef 
enum tagAMTunerSignalStrength
    {
        AMTUNER_HASNOSIGNALSTRENGTH	= -1,
        AMTUNER_NOSIGNAL	= 0,
        AMTUNER_SIGNALPRESENT	= 1
    } 	AMTunerSignalStrength;

typedef 
enum tagAMTunerModeType
    {
        AMTUNER_MODE_DEFAULT	= 0,
        AMTUNER_MODE_TV	= 0x1,
        AMTUNER_MODE_FM_RADIO	= 0x2,
        AMTUNER_MODE_AM_RADIO	= 0x4,
        AMTUNER_MODE_DSS	= 0x8
    } 	AMTunerModeType;

typedef 
enum tagAMTunerEventType
    {
        AMTUNER_EVENT_CHANGED	= 0x1
    } 	AMTunerEventType;




extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0061_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0061_v0_0_s_ifspec;

#ifndef __IAMTuner_INTERFACE_DEFINED__
#define __IAMTuner_INTERFACE_DEFINED__

/* interface IAMTuner */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMTuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("211A8761-03AC-11d1-8D13-00AA00BD8339")
    IAMTuner : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE put_Channel( 
            /* [in] */ long lChannel,
            /* [in] */ long lVideoSubChannel,
            /* [in] */ long lAudioSubChannel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Channel( 
            /* [annotation][out] */ 
            _Out_  long *plChannel,
            /* [annotation][out] */ 
            _Out_  long *plVideoSubChannel,
            /* [annotation][out] */ 
            _Out_  long *plAudioSubChannel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChannelMinMax( 
            /* [annotation][out] */ 
            _Out_  long *lChannelMin,
            /* [annotation][out] */ 
            _Out_  long *lChannelMax) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_CountryCode( 
            /* [in] */ long lCountryCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_CountryCode( 
            /* [annotation][out] */ 
            _Out_  long *plCountryCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_TuningSpace( 
            /* [in] */ long lTuningSpace) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TuningSpace( 
            /* [annotation][out] */ 
            _Out_  long *plTuningSpace) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Logon( 
            /* [in] */ HANDLE hCurrentUser) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Logout( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SignalPresent( 
            /* [annotation][out] */ 
            _Out_  long *plSignalStrength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ AMTunerModeType lMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [annotation][out] */ 
            _Out_  AMTunerModeType *plMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAvailableModes( 
            /* [annotation][out] */ 
            _Out_  long *plModes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterNotificationCallBack( 
            /* [in] */ IAMTunerNotification *pNotify,
            /* [in] */ long lEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterNotificationCallBack( 
            /* [in] */ IAMTunerNotification *pNotify) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTuner * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_Channel)
        HRESULT ( STDMETHODCALLTYPE *put_Channel )( 
            IAMTuner * This,
            /* [in] */ long lChannel,
            /* [in] */ long lVideoSubChannel,
            /* [in] */ long lAudioSubChannel);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_Channel)
        HRESULT ( STDMETHODCALLTYPE *get_Channel )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plChannel,
            /* [annotation][out] */ 
            _Out_  long *plVideoSubChannel,
            /* [annotation][out] */ 
            _Out_  long *plAudioSubChannel);
        
        DECLSPEC_XFGVIRT(IAMTuner, ChannelMinMax)
        HRESULT ( STDMETHODCALLTYPE *ChannelMinMax )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  long *lChannelMin,
            /* [annotation][out] */ 
            _Out_  long *lChannelMax);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_CountryCode)
        HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            IAMTuner * This,
            /* [in] */ long lCountryCode);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_CountryCode)
        HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plCountryCode);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_TuningSpace)
        HRESULT ( STDMETHODCALLTYPE *put_TuningSpace )( 
            IAMTuner * This,
            /* [in] */ long lTuningSpace);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_TuningSpace)
        HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plTuningSpace);
        
        DECLSPEC_XFGVIRT(IAMTuner, Logon)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Logon )( 
            IAMTuner * This,
            /* [in] */ HANDLE hCurrentUser);
        
        DECLSPEC_XFGVIRT(IAMTuner, Logout)
        HRESULT ( STDMETHODCALLTYPE *Logout )( 
            IAMTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTuner, SignalPresent)
        HRESULT ( STDMETHODCALLTYPE *SignalPresent )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plSignalStrength);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_Mode)
        HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            IAMTuner * This,
            /* [in] */ AMTunerModeType lMode);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_Mode)
        HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  AMTunerModeType *plMode);
        
        DECLSPEC_XFGVIRT(IAMTuner, GetAvailableModes)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableModes )( 
            IAMTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plModes);
        
        DECLSPEC_XFGVIRT(IAMTuner, RegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotificationCallBack )( 
            IAMTuner * This,
            /* [in] */ IAMTunerNotification *pNotify,
            /* [in] */ long lEvents);
        
        DECLSPEC_XFGVIRT(IAMTuner, UnRegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterNotificationCallBack )( 
            IAMTuner * This,
            /* [in] */ IAMTunerNotification *pNotify);
        
        END_INTERFACE
    } IAMTunerVtbl;

    interface IAMTuner
    {
        CONST_VTBL struct IAMTunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTuner_put_Channel(This,lChannel,lVideoSubChannel,lAudioSubChannel)	\
    ( (This)->lpVtbl -> put_Channel(This,lChannel,lVideoSubChannel,lAudioSubChannel) ) 

#define IAMTuner_get_Channel(This,plChannel,plVideoSubChannel,plAudioSubChannel)	\
    ( (This)->lpVtbl -> get_Channel(This,plChannel,plVideoSubChannel,plAudioSubChannel) ) 

#define IAMTuner_ChannelMinMax(This,lChannelMin,lChannelMax)	\
    ( (This)->lpVtbl -> ChannelMinMax(This,lChannelMin,lChannelMax) ) 

#define IAMTuner_put_CountryCode(This,lCountryCode)	\
    ( (This)->lpVtbl -> put_CountryCode(This,lCountryCode) ) 

#define IAMTuner_get_CountryCode(This,plCountryCode)	\
    ( (This)->lpVtbl -> get_CountryCode(This,plCountryCode) ) 

#define IAMTuner_put_TuningSpace(This,lTuningSpace)	\
    ( (This)->lpVtbl -> put_TuningSpace(This,lTuningSpace) ) 

#define IAMTuner_get_TuningSpace(This,plTuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,plTuningSpace) ) 

#define IAMTuner_Logon(This,hCurrentUser)	\
    ( (This)->lpVtbl -> Logon(This,hCurrentUser) ) 

#define IAMTuner_Logout(This)	\
    ( (This)->lpVtbl -> Logout(This) ) 

#define IAMTuner_SignalPresent(This,plSignalStrength)	\
    ( (This)->lpVtbl -> SignalPresent(This,plSignalStrength) ) 

#define IAMTuner_put_Mode(This,lMode)	\
    ( (This)->lpVtbl -> put_Mode(This,lMode) ) 

#define IAMTuner_get_Mode(This,plMode)	\
    ( (This)->lpVtbl -> get_Mode(This,plMode) ) 

#define IAMTuner_GetAvailableModes(This,plModes)	\
    ( (This)->lpVtbl -> GetAvailableModes(This,plModes) ) 

#define IAMTuner_RegisterNotificationCallBack(This,pNotify,lEvents)	\
    ( (This)->lpVtbl -> RegisterNotificationCallBack(This,pNotify,lEvents) ) 

#define IAMTuner_UnRegisterNotificationCallBack(This,pNotify)	\
    ( (This)->lpVtbl -> UnRegisterNotificationCallBack(This,pNotify) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTuner_INTERFACE_DEFINED__ */


#ifndef __IAMTunerNotification_INTERFACE_DEFINED__
#define __IAMTunerNotification_INTERFACE_DEFINED__

/* interface IAMTunerNotification */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMTunerNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("211A8760-03AC-11d1-8D13-00AA00BD8339")
    IAMTunerNotification : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [in] */ AMTunerEventType Event) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTunerNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTunerNotification * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTunerNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTunerNotification * This);
        
        DECLSPEC_XFGVIRT(IAMTunerNotification, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            IAMTunerNotification * This,
            /* [in] */ AMTunerEventType Event);
        
        END_INTERFACE
    } IAMTunerNotificationVtbl;

    interface IAMTunerNotification
    {
        CONST_VTBL struct IAMTunerNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTunerNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTunerNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTunerNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTunerNotification_OnEvent(This,Event)	\
    ( (This)->lpVtbl -> OnEvent(This,Event) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTunerNotification_INTERFACE_DEFINED__ */


#ifndef __IAMTVTuner_INTERFACE_DEFINED__
#define __IAMTVTuner_INTERFACE_DEFINED__

/* interface IAMTVTuner */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMTVTuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("211A8766-03AC-11d1-8D13-00AA00BD8339")
    IAMTVTuner : public IAMTuner
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AvailableTVFormats( 
            /* [annotation][out] */ 
            _Out_  long *lAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TVFormat( 
            /* [annotation][out] */ 
            _Out_  long *plAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AutoTune( 
            /* [in] */ long lChannel,
            /* [annotation][out] */ 
            _Out_  long *plFoundSignal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StoreAutoTune( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_NumInputConnections( 
            /* [annotation][out] */ 
            _Out_  long *plNumInputConnections) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_InputType( 
            /* [in] */ long lIndex,
            /* [in] */ TunerInputType InputType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_InputType( 
            /* [in] */ long lIndex,
            /* [annotation][out] */ 
            _Out_  TunerInputType *pInputType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_ConnectInput( 
            /* [in] */ long lIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ConnectInput( 
            /* [annotation][out] */ 
            _Out_  long *plIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_VideoFrequency( 
            /* [annotation][out] */ 
            _Out_  long *lFreq) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AudioFrequency( 
            /* [annotation][out] */ 
            _Out_  long *lFreq) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTVTunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTVTuner * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTVTuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTVTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_Channel)
        HRESULT ( STDMETHODCALLTYPE *put_Channel )( 
            IAMTVTuner * This,
            /* [in] */ long lChannel,
            /* [in] */ long lVideoSubChannel,
            /* [in] */ long lAudioSubChannel);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_Channel)
        HRESULT ( STDMETHODCALLTYPE *get_Channel )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plChannel,
            /* [annotation][out] */ 
            _Out_  long *plVideoSubChannel,
            /* [annotation][out] */ 
            _Out_  long *plAudioSubChannel);
        
        DECLSPEC_XFGVIRT(IAMTuner, ChannelMinMax)
        HRESULT ( STDMETHODCALLTYPE *ChannelMinMax )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *lChannelMin,
            /* [annotation][out] */ 
            _Out_  long *lChannelMax);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_CountryCode)
        HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            IAMTVTuner * This,
            /* [in] */ long lCountryCode);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_CountryCode)
        HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plCountryCode);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_TuningSpace)
        HRESULT ( STDMETHODCALLTYPE *put_TuningSpace )( 
            IAMTVTuner * This,
            /* [in] */ long lTuningSpace);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_TuningSpace)
        HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plTuningSpace);
        
        DECLSPEC_XFGVIRT(IAMTuner, Logon)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Logon )( 
            IAMTVTuner * This,
            /* [in] */ HANDLE hCurrentUser);
        
        DECLSPEC_XFGVIRT(IAMTuner, Logout)
        HRESULT ( STDMETHODCALLTYPE *Logout )( 
            IAMTVTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTuner, SignalPresent)
        HRESULT ( STDMETHODCALLTYPE *SignalPresent )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plSignalStrength);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_Mode)
        HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            IAMTVTuner * This,
            /* [in] */ AMTunerModeType lMode);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_Mode)
        HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  AMTunerModeType *plMode);
        
        DECLSPEC_XFGVIRT(IAMTuner, GetAvailableModes)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableModes )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plModes);
        
        DECLSPEC_XFGVIRT(IAMTuner, RegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotificationCallBack )( 
            IAMTVTuner * This,
            /* [in] */ IAMTunerNotification *pNotify,
            /* [in] */ long lEvents);
        
        DECLSPEC_XFGVIRT(IAMTuner, UnRegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterNotificationCallBack )( 
            IAMTVTuner * This,
            /* [in] */ IAMTunerNotification *pNotify);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_AvailableTVFormats)
        HRESULT ( STDMETHODCALLTYPE *get_AvailableTVFormats )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *lAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_TVFormat)
        HRESULT ( STDMETHODCALLTYPE *get_TVFormat )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, AutoTune)
        HRESULT ( STDMETHODCALLTYPE *AutoTune )( 
            IAMTVTuner * This,
            /* [in] */ long lChannel,
            /* [annotation][out] */ 
            _Out_  long *plFoundSignal);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, StoreAutoTune)
        HRESULT ( STDMETHODCALLTYPE *StoreAutoTune )( 
            IAMTVTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_NumInputConnections)
        HRESULT ( STDMETHODCALLTYPE *get_NumInputConnections )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plNumInputConnections);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, put_InputType)
        HRESULT ( STDMETHODCALLTYPE *put_InputType )( 
            IAMTVTuner * This,
            /* [in] */ long lIndex,
            /* [in] */ TunerInputType InputType);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_InputType)
        HRESULT ( STDMETHODCALLTYPE *get_InputType )( 
            IAMTVTuner * This,
            /* [in] */ long lIndex,
            /* [annotation][out] */ 
            _Out_  TunerInputType *pInputType);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, put_ConnectInput)
        HRESULT ( STDMETHODCALLTYPE *put_ConnectInput )( 
            IAMTVTuner * This,
            /* [in] */ long lIndex);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_ConnectInput)
        HRESULT ( STDMETHODCALLTYPE *get_ConnectInput )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plIndex);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_VideoFrequency)
        HRESULT ( STDMETHODCALLTYPE *get_VideoFrequency )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *lFreq);
        
        DECLSPEC_XFGVIRT(IAMTVTuner, get_AudioFrequency)
        HRESULT ( STDMETHODCALLTYPE *get_AudioFrequency )( 
            IAMTVTuner * This,
            /* [annotation][out] */ 
            _Out_  long *lFreq);
        
        END_INTERFACE
    } IAMTVTunerVtbl;

    interface IAMTVTuner
    {
        CONST_VTBL struct IAMTVTunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTVTuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTVTuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTVTuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTVTuner_put_Channel(This,lChannel,lVideoSubChannel,lAudioSubChannel)	\
    ( (This)->lpVtbl -> put_Channel(This,lChannel,lVideoSubChannel,lAudioSubChannel) ) 

#define IAMTVTuner_get_Channel(This,plChannel,plVideoSubChannel,plAudioSubChannel)	\
    ( (This)->lpVtbl -> get_Channel(This,plChannel,plVideoSubChannel,plAudioSubChannel) ) 

#define IAMTVTuner_ChannelMinMax(This,lChannelMin,lChannelMax)	\
    ( (This)->lpVtbl -> ChannelMinMax(This,lChannelMin,lChannelMax) ) 

#define IAMTVTuner_put_CountryCode(This,lCountryCode)	\
    ( (This)->lpVtbl -> put_CountryCode(This,lCountryCode) ) 

#define IAMTVTuner_get_CountryCode(This,plCountryCode)	\
    ( (This)->lpVtbl -> get_CountryCode(This,plCountryCode) ) 

#define IAMTVTuner_put_TuningSpace(This,lTuningSpace)	\
    ( (This)->lpVtbl -> put_TuningSpace(This,lTuningSpace) ) 

#define IAMTVTuner_get_TuningSpace(This,plTuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,plTuningSpace) ) 

#define IAMTVTuner_Logon(This,hCurrentUser)	\
    ( (This)->lpVtbl -> Logon(This,hCurrentUser) ) 

#define IAMTVTuner_Logout(This)	\
    ( (This)->lpVtbl -> Logout(This) ) 

#define IAMTVTuner_SignalPresent(This,plSignalStrength)	\
    ( (This)->lpVtbl -> SignalPresent(This,plSignalStrength) ) 

#define IAMTVTuner_put_Mode(This,lMode)	\
    ( (This)->lpVtbl -> put_Mode(This,lMode) ) 

#define IAMTVTuner_get_Mode(This,plMode)	\
    ( (This)->lpVtbl -> get_Mode(This,plMode) ) 

#define IAMTVTuner_GetAvailableModes(This,plModes)	\
    ( (This)->lpVtbl -> GetAvailableModes(This,plModes) ) 

#define IAMTVTuner_RegisterNotificationCallBack(This,pNotify,lEvents)	\
    ( (This)->lpVtbl -> RegisterNotificationCallBack(This,pNotify,lEvents) ) 

#define IAMTVTuner_UnRegisterNotificationCallBack(This,pNotify)	\
    ( (This)->lpVtbl -> UnRegisterNotificationCallBack(This,pNotify) ) 


#define IAMTVTuner_get_AvailableTVFormats(This,lAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_AvailableTVFormats(This,lAnalogVideoStandard) ) 

#define IAMTVTuner_get_TVFormat(This,plAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_TVFormat(This,plAnalogVideoStandard) ) 

#define IAMTVTuner_AutoTune(This,lChannel,plFoundSignal)	\
    ( (This)->lpVtbl -> AutoTune(This,lChannel,plFoundSignal) ) 

#define IAMTVTuner_StoreAutoTune(This)	\
    ( (This)->lpVtbl -> StoreAutoTune(This) ) 

#define IAMTVTuner_get_NumInputConnections(This,plNumInputConnections)	\
    ( (This)->lpVtbl -> get_NumInputConnections(This,plNumInputConnections) ) 

#define IAMTVTuner_put_InputType(This,lIndex,InputType)	\
    ( (This)->lpVtbl -> put_InputType(This,lIndex,InputType) ) 

#define IAMTVTuner_get_InputType(This,lIndex,pInputType)	\
    ( (This)->lpVtbl -> get_InputType(This,lIndex,pInputType) ) 

#define IAMTVTuner_put_ConnectInput(This,lIndex)	\
    ( (This)->lpVtbl -> put_ConnectInput(This,lIndex) ) 

#define IAMTVTuner_get_ConnectInput(This,plIndex)	\
    ( (This)->lpVtbl -> get_ConnectInput(This,plIndex) ) 

#define IAMTVTuner_get_VideoFrequency(This,lFreq)	\
    ( (This)->lpVtbl -> get_VideoFrequency(This,lFreq) ) 

#define IAMTVTuner_get_AudioFrequency(This,lFreq)	\
    ( (This)->lpVtbl -> get_AudioFrequency(This,lFreq) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTVTuner_INTERFACE_DEFINED__ */


#ifndef __IBPCSatelliteTuner_INTERFACE_DEFINED__
#define __IBPCSatelliteTuner_INTERFACE_DEFINED__

/* interface IBPCSatelliteTuner */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IBPCSatelliteTuner;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("211A8765-03AC-11d1-8D13-00AA00BD8339")
    IBPCSatelliteTuner : public IAMTuner
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_DefaultSubChannelTypes( 
            /* [annotation][out] */ 
            _Out_  long *plDefaultVideoType,
            /* [annotation][out] */ 
            _Out_  long *plDefaultAudioType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DefaultSubChannelTypes( 
            /* [in] */ long lDefaultVideoType,
            /* [in] */ long lDefaultAudioType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsTapingPermitted( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IBPCSatelliteTunerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IBPCSatelliteTuner * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IBPCSatelliteTuner * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IBPCSatelliteTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_Channel)
        HRESULT ( STDMETHODCALLTYPE *put_Channel )( 
            IBPCSatelliteTuner * This,
            /* [in] */ long lChannel,
            /* [in] */ long lVideoSubChannel,
            /* [in] */ long lAudioSubChannel);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_Channel)
        HRESULT ( STDMETHODCALLTYPE *get_Channel )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plChannel,
            /* [annotation][out] */ 
            _Out_  long *plVideoSubChannel,
            /* [annotation][out] */ 
            _Out_  long *plAudioSubChannel);
        
        DECLSPEC_XFGVIRT(IAMTuner, ChannelMinMax)
        HRESULT ( STDMETHODCALLTYPE *ChannelMinMax )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *lChannelMin,
            /* [annotation][out] */ 
            _Out_  long *lChannelMax);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_CountryCode)
        HRESULT ( STDMETHODCALLTYPE *put_CountryCode )( 
            IBPCSatelliteTuner * This,
            /* [in] */ long lCountryCode);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_CountryCode)
        HRESULT ( STDMETHODCALLTYPE *get_CountryCode )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plCountryCode);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_TuningSpace)
        HRESULT ( STDMETHODCALLTYPE *put_TuningSpace )( 
            IBPCSatelliteTuner * This,
            /* [in] */ long lTuningSpace);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_TuningSpace)
        HRESULT ( STDMETHODCALLTYPE *get_TuningSpace )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plTuningSpace);
        
        DECLSPEC_XFGVIRT(IAMTuner, Logon)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Logon )( 
            IBPCSatelliteTuner * This,
            /* [in] */ HANDLE hCurrentUser);
        
        DECLSPEC_XFGVIRT(IAMTuner, Logout)
        HRESULT ( STDMETHODCALLTYPE *Logout )( 
            IBPCSatelliteTuner * This);
        
        DECLSPEC_XFGVIRT(IAMTuner, SignalPresent)
        HRESULT ( STDMETHODCALLTYPE *SignalPresent )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plSignalStrength);
        
        DECLSPEC_XFGVIRT(IAMTuner, put_Mode)
        HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            IBPCSatelliteTuner * This,
            /* [in] */ AMTunerModeType lMode);
        
        DECLSPEC_XFGVIRT(IAMTuner, get_Mode)
        HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  AMTunerModeType *plMode);
        
        DECLSPEC_XFGVIRT(IAMTuner, GetAvailableModes)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableModes )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plModes);
        
        DECLSPEC_XFGVIRT(IAMTuner, RegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotificationCallBack )( 
            IBPCSatelliteTuner * This,
            /* [in] */ IAMTunerNotification *pNotify,
            /* [in] */ long lEvents);
        
        DECLSPEC_XFGVIRT(IAMTuner, UnRegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterNotificationCallBack )( 
            IBPCSatelliteTuner * This,
            /* [in] */ IAMTunerNotification *pNotify);
        
        DECLSPEC_XFGVIRT(IBPCSatelliteTuner, get_DefaultSubChannelTypes)
        HRESULT ( STDMETHODCALLTYPE *get_DefaultSubChannelTypes )( 
            IBPCSatelliteTuner * This,
            /* [annotation][out] */ 
            _Out_  long *plDefaultVideoType,
            /* [annotation][out] */ 
            _Out_  long *plDefaultAudioType);
        
        DECLSPEC_XFGVIRT(IBPCSatelliteTuner, put_DefaultSubChannelTypes)
        HRESULT ( STDMETHODCALLTYPE *put_DefaultSubChannelTypes )( 
            IBPCSatelliteTuner * This,
            /* [in] */ long lDefaultVideoType,
            /* [in] */ long lDefaultAudioType);
        
        DECLSPEC_XFGVIRT(IBPCSatelliteTuner, IsTapingPermitted)
        HRESULT ( STDMETHODCALLTYPE *IsTapingPermitted )( 
            IBPCSatelliteTuner * This);
        
        END_INTERFACE
    } IBPCSatelliteTunerVtbl;

    interface IBPCSatelliteTuner
    {
        CONST_VTBL struct IBPCSatelliteTunerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IBPCSatelliteTuner_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IBPCSatelliteTuner_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IBPCSatelliteTuner_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IBPCSatelliteTuner_put_Channel(This,lChannel,lVideoSubChannel,lAudioSubChannel)	\
    ( (This)->lpVtbl -> put_Channel(This,lChannel,lVideoSubChannel,lAudioSubChannel) ) 

#define IBPCSatelliteTuner_get_Channel(This,plChannel,plVideoSubChannel,plAudioSubChannel)	\
    ( (This)->lpVtbl -> get_Channel(This,plChannel,plVideoSubChannel,plAudioSubChannel) ) 

#define IBPCSatelliteTuner_ChannelMinMax(This,lChannelMin,lChannelMax)	\
    ( (This)->lpVtbl -> ChannelMinMax(This,lChannelMin,lChannelMax) ) 

#define IBPCSatelliteTuner_put_CountryCode(This,lCountryCode)	\
    ( (This)->lpVtbl -> put_CountryCode(This,lCountryCode) ) 

#define IBPCSatelliteTuner_get_CountryCode(This,plCountryCode)	\
    ( (This)->lpVtbl -> get_CountryCode(This,plCountryCode) ) 

#define IBPCSatelliteTuner_put_TuningSpace(This,lTuningSpace)	\
    ( (This)->lpVtbl -> put_TuningSpace(This,lTuningSpace) ) 

#define IBPCSatelliteTuner_get_TuningSpace(This,plTuningSpace)	\
    ( (This)->lpVtbl -> get_TuningSpace(This,plTuningSpace) ) 

#define IBPCSatelliteTuner_Logon(This,hCurrentUser)	\
    ( (This)->lpVtbl -> Logon(This,hCurrentUser) ) 

#define IBPCSatelliteTuner_Logout(This)	\
    ( (This)->lpVtbl -> Logout(This) ) 

#define IBPCSatelliteTuner_SignalPresent(This,plSignalStrength)	\
    ( (This)->lpVtbl -> SignalPresent(This,plSignalStrength) ) 

#define IBPCSatelliteTuner_put_Mode(This,lMode)	\
    ( (This)->lpVtbl -> put_Mode(This,lMode) ) 

#define IBPCSatelliteTuner_get_Mode(This,plMode)	\
    ( (This)->lpVtbl -> get_Mode(This,plMode) ) 

#define IBPCSatelliteTuner_GetAvailableModes(This,plModes)	\
    ( (This)->lpVtbl -> GetAvailableModes(This,plModes) ) 

#define IBPCSatelliteTuner_RegisterNotificationCallBack(This,pNotify,lEvents)	\
    ( (This)->lpVtbl -> RegisterNotificationCallBack(This,pNotify,lEvents) ) 

#define IBPCSatelliteTuner_UnRegisterNotificationCallBack(This,pNotify)	\
    ( (This)->lpVtbl -> UnRegisterNotificationCallBack(This,pNotify) ) 


#define IBPCSatelliteTuner_get_DefaultSubChannelTypes(This,plDefaultVideoType,plDefaultAudioType)	\
    ( (This)->lpVtbl -> get_DefaultSubChannelTypes(This,plDefaultVideoType,plDefaultAudioType) ) 

#define IBPCSatelliteTuner_put_DefaultSubChannelTypes(This,lDefaultVideoType,lDefaultAudioType)	\
    ( (This)->lpVtbl -> put_DefaultSubChannelTypes(This,lDefaultVideoType,lDefaultAudioType) ) 

#define IBPCSatelliteTuner_IsTapingPermitted(This)	\
    ( (This)->lpVtbl -> IsTapingPermitted(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IBPCSatelliteTuner_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0065 */
/* [local] */ 

typedef 
enum tagTVAudioMode
    {
        AMTVAUDIO_MODE_MONO	= 0x1,
        AMTVAUDIO_MODE_STEREO	= 0x2,
        AMTVAUDIO_MODE_LANG_A	= 0x10,
        AMTVAUDIO_MODE_LANG_B	= 0x20,
        AMTVAUDIO_MODE_LANG_C	= 0x40,
        AMTVAUDIO_PRESET_STEREO	= 0x200,
        AMTVAUDIO_PRESET_LANG_A	= 0x1000,
        AMTVAUDIO_PRESET_LANG_B	= 0x2000,
        AMTVAUDIO_PRESET_LANG_C	= 0x4000
    } 	TVAudioMode;

typedef 
enum tagAMTVAudioEventType
    {
        AMTVAUDIO_EVENT_CHANGED	= 0x1
    } 	AMTVAudioEventType;




extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0065_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0065_v0_0_s_ifspec;

#ifndef __IAMTVAudio_INTERFACE_DEFINED__
#define __IAMTVAudio_INTERFACE_DEFINED__

/* interface IAMTVAudio */
/* [unique][uuid][local][object][local] */ 


EXTERN_C const IID IID_IAMTVAudio;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83EC1C30-23D1-11d1-99E6-00A0C9560266")
    IAMTVAudio : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetHardwareSupportedTVAudioModes( 
            /* [annotation][out] */ 
            _Out_  long *plModes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAvailableTVAudioModes( 
            /* [annotation][out] */ 
            _Out_  long *plModes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TVAudioMode( 
            /* [annotation][out] */ 
            _Out_  long *plMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_TVAudioMode( 
            /* [in] */ long lMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RegisterNotificationCallBack( 
            /* [in] */ IAMTunerNotification *pNotify,
            /* [in] */ long lEvents) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnRegisterNotificationCallBack( 
            IAMTunerNotification *pNotify) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTVAudioVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTVAudio * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTVAudio * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTVAudio * This);
        
        DECLSPEC_XFGVIRT(IAMTVAudio, GetHardwareSupportedTVAudioModes)
        HRESULT ( STDMETHODCALLTYPE *GetHardwareSupportedTVAudioModes )( 
            IAMTVAudio * This,
            /* [annotation][out] */ 
            _Out_  long *plModes);
        
        DECLSPEC_XFGVIRT(IAMTVAudio, GetAvailableTVAudioModes)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableTVAudioModes )( 
            IAMTVAudio * This,
            /* [annotation][out] */ 
            _Out_  long *plModes);
        
        DECLSPEC_XFGVIRT(IAMTVAudio, get_TVAudioMode)
        HRESULT ( STDMETHODCALLTYPE *get_TVAudioMode )( 
            IAMTVAudio * This,
            /* [annotation][out] */ 
            _Out_  long *plMode);
        
        DECLSPEC_XFGVIRT(IAMTVAudio, put_TVAudioMode)
        HRESULT ( STDMETHODCALLTYPE *put_TVAudioMode )( 
            IAMTVAudio * This,
            /* [in] */ long lMode);
        
        DECLSPEC_XFGVIRT(IAMTVAudio, RegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *RegisterNotificationCallBack )( 
            IAMTVAudio * This,
            /* [in] */ IAMTunerNotification *pNotify,
            /* [in] */ long lEvents);
        
        DECLSPEC_XFGVIRT(IAMTVAudio, UnRegisterNotificationCallBack)
        HRESULT ( STDMETHODCALLTYPE *UnRegisterNotificationCallBack )( 
            IAMTVAudio * This,
            IAMTunerNotification *pNotify);
        
        END_INTERFACE
    } IAMTVAudioVtbl;

    interface IAMTVAudio
    {
        CONST_VTBL struct IAMTVAudioVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTVAudio_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTVAudio_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTVAudio_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTVAudio_GetHardwareSupportedTVAudioModes(This,plModes)	\
    ( (This)->lpVtbl -> GetHardwareSupportedTVAudioModes(This,plModes) ) 

#define IAMTVAudio_GetAvailableTVAudioModes(This,plModes)	\
    ( (This)->lpVtbl -> GetAvailableTVAudioModes(This,plModes) ) 

#define IAMTVAudio_get_TVAudioMode(This,plMode)	\
    ( (This)->lpVtbl -> get_TVAudioMode(This,plMode) ) 

#define IAMTVAudio_put_TVAudioMode(This,lMode)	\
    ( (This)->lpVtbl -> put_TVAudioMode(This,lMode) ) 

#define IAMTVAudio_RegisterNotificationCallBack(This,pNotify,lEvents)	\
    ( (This)->lpVtbl -> RegisterNotificationCallBack(This,pNotify,lEvents) ) 

#define IAMTVAudio_UnRegisterNotificationCallBack(This,pNotify)	\
    ( (This)->lpVtbl -> UnRegisterNotificationCallBack(This,pNotify) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTVAudio_INTERFACE_DEFINED__ */


#ifndef __IAMTVAudioNotification_INTERFACE_DEFINED__
#define __IAMTVAudioNotification_INTERFACE_DEFINED__

/* interface IAMTVAudioNotification */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMTVAudioNotification;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("83EC1C33-23D1-11d1-99E6-00A0C9560266")
    IAMTVAudioNotification : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnEvent( 
            /* [in] */ AMTVAudioEventType Event) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTVAudioNotificationVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTVAudioNotification * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTVAudioNotification * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTVAudioNotification * This);
        
        DECLSPEC_XFGVIRT(IAMTVAudioNotification, OnEvent)
        HRESULT ( STDMETHODCALLTYPE *OnEvent )( 
            IAMTVAudioNotification * This,
            /* [in] */ AMTVAudioEventType Event);
        
        END_INTERFACE
    } IAMTVAudioNotificationVtbl;

    interface IAMTVAudioNotification
    {
        CONST_VTBL struct IAMTVAudioNotificationVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTVAudioNotification_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTVAudioNotification_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTVAudioNotification_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTVAudioNotification_OnEvent(This,Event)	\
    ( (This)->lpVtbl -> OnEvent(This,Event) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTVAudioNotification_INTERFACE_DEFINED__ */


#ifndef __IAMAnalogVideoEncoder_INTERFACE_DEFINED__
#define __IAMAnalogVideoEncoder_INTERFACE_DEFINED__

/* interface IAMAnalogVideoEncoder */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMAnalogVideoEncoder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("C6E133B0-30AC-11d0-A18C-00A0C9118956")
    IAMAnalogVideoEncoder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_AvailableTVFormats( 
            /* [annotation][out] */ 
            _Out_  long *lAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_TVFormat( 
            /* [in] */ long lAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_TVFormat( 
            /* [annotation][out] */ 
            _Out_  long *plAnalogVideoStandard) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_CopyProtection( 
            /* [in] */ long lVideoCopyProtection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_CopyProtection( 
            /* [annotation][out] */ 
            _Out_  long *lVideoCopyProtection) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_CCEnable( 
            /* [in] */ long lCCEnable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_CCEnable( 
            /* [annotation][out] */ 
            _Out_  long *lCCEnable) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMAnalogVideoEncoderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMAnalogVideoEncoder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMAnalogVideoEncoder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMAnalogVideoEncoder * This);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, get_AvailableTVFormats)
        HRESULT ( STDMETHODCALLTYPE *get_AvailableTVFormats )( 
            IAMAnalogVideoEncoder * This,
            /* [annotation][out] */ 
            _Out_  long *lAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, put_TVFormat)
        HRESULT ( STDMETHODCALLTYPE *put_TVFormat )( 
            IAMAnalogVideoEncoder * This,
            /* [in] */ long lAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, get_TVFormat)
        HRESULT ( STDMETHODCALLTYPE *get_TVFormat )( 
            IAMAnalogVideoEncoder * This,
            /* [annotation][out] */ 
            _Out_  long *plAnalogVideoStandard);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, put_CopyProtection)
        HRESULT ( STDMETHODCALLTYPE *put_CopyProtection )( 
            IAMAnalogVideoEncoder * This,
            /* [in] */ long lVideoCopyProtection);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, get_CopyProtection)
        HRESULT ( STDMETHODCALLTYPE *get_CopyProtection )( 
            IAMAnalogVideoEncoder * This,
            /* [annotation][out] */ 
            _Out_  long *lVideoCopyProtection);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, put_CCEnable)
        HRESULT ( STDMETHODCALLTYPE *put_CCEnable )( 
            IAMAnalogVideoEncoder * This,
            /* [in] */ long lCCEnable);
        
        DECLSPEC_XFGVIRT(IAMAnalogVideoEncoder, get_CCEnable)
        HRESULT ( STDMETHODCALLTYPE *get_CCEnable )( 
            IAMAnalogVideoEncoder * This,
            /* [annotation][out] */ 
            _Out_  long *lCCEnable);
        
        END_INTERFACE
    } IAMAnalogVideoEncoderVtbl;

    interface IAMAnalogVideoEncoder
    {
        CONST_VTBL struct IAMAnalogVideoEncoderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMAnalogVideoEncoder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMAnalogVideoEncoder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMAnalogVideoEncoder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMAnalogVideoEncoder_get_AvailableTVFormats(This,lAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_AvailableTVFormats(This,lAnalogVideoStandard) ) 

#define IAMAnalogVideoEncoder_put_TVFormat(This,lAnalogVideoStandard)	\
    ( (This)->lpVtbl -> put_TVFormat(This,lAnalogVideoStandard) ) 

#define IAMAnalogVideoEncoder_get_TVFormat(This,plAnalogVideoStandard)	\
    ( (This)->lpVtbl -> get_TVFormat(This,plAnalogVideoStandard) ) 

#define IAMAnalogVideoEncoder_put_CopyProtection(This,lVideoCopyProtection)	\
    ( (This)->lpVtbl -> put_CopyProtection(This,lVideoCopyProtection) ) 

#define IAMAnalogVideoEncoder_get_CopyProtection(This,lVideoCopyProtection)	\
    ( (This)->lpVtbl -> get_CopyProtection(This,lVideoCopyProtection) ) 

#define IAMAnalogVideoEncoder_put_CCEnable(This,lCCEnable)	\
    ( (This)->lpVtbl -> put_CCEnable(This,lCCEnable) ) 

#define IAMAnalogVideoEncoder_get_CCEnable(This,lCCEnable)	\
    ( (This)->lpVtbl -> get_CCEnable(This,lCCEnable) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMAnalogVideoEncoder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0068 */
/* [local] */ 

typedef 
enum AMPROPERTY_PIN
    {
        AMPROPERTY_PIN_CATEGORY	= 0,
        AMPROPERTY_PIN_MEDIUM	= ( AMPROPERTY_PIN_CATEGORY + 1 ) 
    } 	AMPROPERTY_PIN;

#ifndef _IKsPropertySet_
#define _IKsPropertySet_
#define KSPROPERTY_SUPPORT_GET  1
#define KSPROPERTY_SUPPORT_SET  2


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0068_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0068_v0_0_s_ifspec;

#ifndef __IKsPropertySet_INTERFACE_DEFINED__
#define __IKsPropertySet_INTERFACE_DEFINED__

/* interface IKsPropertySet */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IKsPropertySet;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31EFAC30-515C-11d0-A9AA-00AA0061BE93")
    IKsPropertySet : public IUnknown
    {
    public:
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Set( 
            /* [in] */ REFGUID guidPropSet,
            /* [in] */ DWORD dwPropID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbInstanceData)  LPVOID pInstanceData,
            /* [in] */ DWORD cbInstanceData,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbPropData)  LPVOID pPropData,
            /* [in] */ DWORD cbPropData) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE Get( 
            /* [in] */ REFGUID guidPropSet,
            /* [in] */ DWORD dwPropID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbInstanceData)  LPVOID pInstanceData,
            /* [in] */ DWORD cbInstanceData,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(cbPropData, *pcbReturned)  LPVOID pPropData,
            /* [in] */ DWORD cbPropData,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbReturned) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE QuerySupported( 
            /* [in] */ REFGUID guidPropSet,
            /* [in] */ DWORD dwPropID,
            /* [annotation][out] */ 
            _Out_  DWORD *pTypeSupport) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IKsPropertySetVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IKsPropertySet * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IKsPropertySet * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IKsPropertySet * This);
        
        DECLSPEC_XFGVIRT(IKsPropertySet, Set)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Set )( 
            IKsPropertySet * This,
            /* [in] */ REFGUID guidPropSet,
            /* [in] */ DWORD dwPropID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbInstanceData)  LPVOID pInstanceData,
            /* [in] */ DWORD cbInstanceData,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbPropData)  LPVOID pPropData,
            /* [in] */ DWORD cbPropData);
        
        DECLSPEC_XFGVIRT(IKsPropertySet, Get)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Get )( 
            IKsPropertySet * This,
            /* [in] */ REFGUID guidPropSet,
            /* [in] */ DWORD dwPropID,
            /* [annotation][size_is][in] */ 
            _In_reads_bytes_(cbInstanceData)  LPVOID pInstanceData,
            /* [in] */ DWORD cbInstanceData,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(cbPropData, *pcbReturned)  LPVOID pPropData,
            /* [in] */ DWORD cbPropData,
            /* [annotation][out] */ 
            _Out_  DWORD *pcbReturned);
        
        DECLSPEC_XFGVIRT(IKsPropertySet, QuerySupported)
        HRESULT ( STDMETHODCALLTYPE *QuerySupported )( 
            IKsPropertySet * This,
            /* [in] */ REFGUID guidPropSet,
            /* [in] */ DWORD dwPropID,
            /* [annotation][out] */ 
            _Out_  DWORD *pTypeSupport);
        
        END_INTERFACE
    } IKsPropertySetVtbl;

    interface IKsPropertySet
    {
        CONST_VTBL struct IKsPropertySetVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IKsPropertySet_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IKsPropertySet_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IKsPropertySet_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IKsPropertySet_Set(This,guidPropSet,dwPropID,pInstanceData,cbInstanceData,pPropData,cbPropData)	\
    ( (This)->lpVtbl -> Set(This,guidPropSet,dwPropID,pInstanceData,cbInstanceData,pPropData,cbPropData) ) 

#define IKsPropertySet_Get(This,guidPropSet,dwPropID,pInstanceData,cbInstanceData,pPropData,cbPropData,pcbReturned)	\
    ( (This)->lpVtbl -> Get(This,guidPropSet,dwPropID,pInstanceData,cbInstanceData,pPropData,cbPropData,pcbReturned) ) 

#define IKsPropertySet_QuerySupported(This,guidPropSet,dwPropID,pTypeSupport)	\
    ( (This)->lpVtbl -> QuerySupported(This,guidPropSet,dwPropID,pTypeSupport) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */



/* [call_as] */ HRESULT STDMETHODCALLTYPE IKsPropertySet_RemoteSet_Proxy( 
    IKsPropertySet * This,
    /* [in] */ REFGUID guidPropSet,
    /* [in] */ DWORD dwPropID,
    /* [size_is][in] */ byte *pInstanceData,
    /* [in] */ DWORD cbInstanceData,
    /* [size_is][in] */ byte *pPropData,
    /* [in] */ DWORD cbPropData);


void __RPC_STUB IKsPropertySet_RemoteSet_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);


/* [call_as] */ HRESULT STDMETHODCALLTYPE IKsPropertySet_RemoteGet_Proxy( 
    IKsPropertySet * This,
    /* [in] */ REFGUID guidPropSet,
    /* [in] */ DWORD dwPropID,
    /* [size_is][in] */ byte *pInstanceData,
    /* [in] */ DWORD cbInstanceData,
    /* [size_is][out] */ byte *pPropData,
    /* [in] */ DWORD cbPropData,
    /* [annotation][out] */ 
    _Out_  DWORD *pcbReturned);


void __RPC_STUB IKsPropertySet_RemoteGet_Stub(
    IRpcStubBuffer *This,
    IRpcChannelBuffer *_pRpcChannelBuffer,
    PRPC_MESSAGE _pRpcMessage,
    DWORD *_pdwStubPhase);



#endif 	/* __IKsPropertySet_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0069 */
/* [local] */ 

#endif // _IKsPropertySet_


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0069_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0069_v0_0_s_ifspec;

#ifndef __IMediaPropertyBag_INTERFACE_DEFINED__
#define __IMediaPropertyBag_INTERFACE_DEFINED__

/* interface IMediaPropertyBag */
/* [unique][uuid][object][local] */ 

typedef IMediaPropertyBag *LPMEDIAPROPERTYBAG;


EXTERN_C const IID IID_IMediaPropertyBag;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6025A880-C0D5-11d0-BD4E-00A0C911CE86")
    IMediaPropertyBag : public IPropertyBag
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE EnumProperty( 
            /* [in] */ ULONG iProperty,
            /* [out][in] */ VARIANT *pvarPropertyName,
            /* [out][in] */ VARIANT *pvarPropertyValue) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMediaPropertyBagVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMediaPropertyBag * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMediaPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMediaPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Read)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *Read )( 
            IMediaPropertyBag * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [out][in] */ VARIANT *pVar,
            /* [unique][in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPropertyBag, Write)
        HRESULT ( STDMETHODCALLTYPE *Write )( 
            IMediaPropertyBag * This,
            /* [in] */ LPCOLESTR pszPropName,
            /* [in] */ VARIANT *pVar);
        
        DECLSPEC_XFGVIRT(IMediaPropertyBag, EnumProperty)
        HRESULT ( STDMETHODCALLTYPE *EnumProperty )( 
            IMediaPropertyBag * This,
            /* [in] */ ULONG iProperty,
            /* [out][in] */ VARIANT *pvarPropertyName,
            /* [out][in] */ VARIANT *pvarPropertyValue);
        
        END_INTERFACE
    } IMediaPropertyBagVtbl;

    interface IMediaPropertyBag
    {
        CONST_VTBL struct IMediaPropertyBagVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMediaPropertyBag_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMediaPropertyBag_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMediaPropertyBag_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMediaPropertyBag_Read(This,pszPropName,pVar,pErrorLog)	\
    ( (This)->lpVtbl -> Read(This,pszPropName,pVar,pErrorLog) ) 

#define IMediaPropertyBag_Write(This,pszPropName,pVar)	\
    ( (This)->lpVtbl -> Write(This,pszPropName,pVar) ) 


#define IMediaPropertyBag_EnumProperty(This,iProperty,pvarPropertyName,pvarPropertyValue)	\
    ( (This)->lpVtbl -> EnumProperty(This,iProperty,pvarPropertyName,pvarPropertyValue) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMediaPropertyBag_INTERFACE_DEFINED__ */


#ifndef __IPersistMediaPropertyBag_INTERFACE_DEFINED__
#define __IPersistMediaPropertyBag_INTERFACE_DEFINED__

/* interface IPersistMediaPropertyBag */
/* [unique][uuid][object][local] */ 

typedef IPersistMediaPropertyBag *LPPERSISTMEDIAPROPERTYBAG;


EXTERN_C const IID IID_IPersistMediaPropertyBag;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5738E040-B67F-11d0-BD4D-00A0C911CE86")
    IPersistMediaPropertyBag : public IPersist
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitNew( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Load( 
            /* [in] */ IMediaPropertyBag *pPropBag,
            /* [in] */ IErrorLog *pErrorLog) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Save( 
            /* [in] */ IMediaPropertyBag *pPropBag,
            /* [in] */ BOOL fClearDirty,
            /* [in] */ BOOL fSaveAllProperties) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPersistMediaPropertyBagVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPersistMediaPropertyBag * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPersistMediaPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPersistMediaPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IPersist, GetClassID)
        HRESULT ( STDMETHODCALLTYPE *GetClassID )( 
            IPersistMediaPropertyBag * This,
            /* [out] */ CLSID *pClassID);
        
        DECLSPEC_XFGVIRT(IPersistMediaPropertyBag, InitNew)
        HRESULT ( STDMETHODCALLTYPE *InitNew )( 
            IPersistMediaPropertyBag * This);
        
        DECLSPEC_XFGVIRT(IPersistMediaPropertyBag, Load)
        HRESULT ( STDMETHODCALLTYPE *Load )( 
            IPersistMediaPropertyBag * This,
            /* [in] */ IMediaPropertyBag *pPropBag,
            /* [in] */ IErrorLog *pErrorLog);
        
        DECLSPEC_XFGVIRT(IPersistMediaPropertyBag, Save)
        HRESULT ( STDMETHODCALLTYPE *Save )( 
            IPersistMediaPropertyBag * This,
            /* [in] */ IMediaPropertyBag *pPropBag,
            /* [in] */ BOOL fClearDirty,
            /* [in] */ BOOL fSaveAllProperties);
        
        END_INTERFACE
    } IPersistMediaPropertyBagVtbl;

    interface IPersistMediaPropertyBag
    {
        CONST_VTBL struct IPersistMediaPropertyBagVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPersistMediaPropertyBag_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPersistMediaPropertyBag_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPersistMediaPropertyBag_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPersistMediaPropertyBag_GetClassID(This,pClassID)	\
    ( (This)->lpVtbl -> GetClassID(This,pClassID) ) 


#define IPersistMediaPropertyBag_InitNew(This)	\
    ( (This)->lpVtbl -> InitNew(This) ) 

#define IPersistMediaPropertyBag_Load(This,pPropBag,pErrorLog)	\
    ( (This)->lpVtbl -> Load(This,pPropBag,pErrorLog) ) 

#define IPersistMediaPropertyBag_Save(This,pPropBag,fClearDirty,fSaveAllProperties)	\
    ( (This)->lpVtbl -> Save(This,pPropBag,fClearDirty,fSaveAllProperties) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPersistMediaPropertyBag_INTERFACE_DEFINED__ */


#ifndef __IAMPhysicalPinInfo_INTERFACE_DEFINED__
#define __IAMPhysicalPinInfo_INTERFACE_DEFINED__

/* interface IAMPhysicalPinInfo */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMPhysicalPinInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F938C991-3029-11cf-8C44-00AA006B6814")
    IAMPhysicalPinInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPhysicalType( 
            /* [annotation][out] */ 
            _Out_  long *pType,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszType) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMPhysicalPinInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMPhysicalPinInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMPhysicalPinInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMPhysicalPinInfo * This);
        
        DECLSPEC_XFGVIRT(IAMPhysicalPinInfo, GetPhysicalType)
        HRESULT ( STDMETHODCALLTYPE *GetPhysicalType )( 
            IAMPhysicalPinInfo * This,
            /* [annotation][out] */ 
            _Out_  long *pType,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszType);
        
        END_INTERFACE
    } IAMPhysicalPinInfoVtbl;

    interface IAMPhysicalPinInfo
    {
        CONST_VTBL struct IAMPhysicalPinInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMPhysicalPinInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMPhysicalPinInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMPhysicalPinInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMPhysicalPinInfo_GetPhysicalType(This,pType,ppszType)	\
    ( (This)->lpVtbl -> GetPhysicalType(This,pType,ppszType) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMPhysicalPinInfo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0072 */
/* [local] */ 

typedef IAMPhysicalPinInfo *PAMPHYSICALPININFO;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0072_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0072_v0_0_s_ifspec;

#ifndef __IAMExtDevice_INTERFACE_DEFINED__
#define __IAMExtDevice_INTERFACE_DEFINED__

/* interface IAMExtDevice */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMExtDevice;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("B5730A90-1A2C-11cf-8C23-00AA006B6814")
    IAMExtDevice : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapability( 
            /* [in] */ long Capability,
            /* [annotation][out] */ 
            _Out_  long *pValue,
            /* [annotation][out] */ 
            _Out_  double *pdblValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ExternalDeviceID( 
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_ExternalDeviceVersion( 
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DevicePower( 
            /* [in] */ long PowerMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DevicePower( 
            /* [annotation][out] */ 
            _Out_  long *pPowerMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Calibrate( 
            /* [in] */ HEVENT hEvent,
            /* [in] */ long Mode,
            /* [annotation][out] */ 
            _Out_  long *pStatus) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_DevicePort( 
            /* [in] */ long DevicePort) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_DevicePort( 
            /* [annotation][out] */ 
            _Out_  long *pDevicePort) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMExtDeviceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMExtDevice * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMExtDevice * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMExtDevice * This);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, GetCapability)
        HRESULT ( STDMETHODCALLTYPE *GetCapability )( 
            IAMExtDevice * This,
            /* [in] */ long Capability,
            /* [annotation][out] */ 
            _Out_  long *pValue,
            /* [annotation][out] */ 
            _Out_  double *pdblValue);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, get_ExternalDeviceID)
        HRESULT ( STDMETHODCALLTYPE *get_ExternalDeviceID )( 
            IAMExtDevice * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszData);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, get_ExternalDeviceVersion)
        HRESULT ( STDMETHODCALLTYPE *get_ExternalDeviceVersion )( 
            IAMExtDevice * This,
            /* [annotation][out] */ 
            _Out_  LPOLESTR *ppszData);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, put_DevicePower)
        HRESULT ( STDMETHODCALLTYPE *put_DevicePower )( 
            IAMExtDevice * This,
            /* [in] */ long PowerMode);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, get_DevicePower)
        HRESULT ( STDMETHODCALLTYPE *get_DevicePower )( 
            IAMExtDevice * This,
            /* [annotation][out] */ 
            _Out_  long *pPowerMode);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, Calibrate)
        HRESULT ( STDMETHODCALLTYPE *Calibrate )( 
            IAMExtDevice * This,
            /* [in] */ HEVENT hEvent,
            /* [in] */ long Mode,
            /* [annotation][out] */ 
            _Out_  long *pStatus);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, put_DevicePort)
        HRESULT ( STDMETHODCALLTYPE *put_DevicePort )( 
            IAMExtDevice * This,
            /* [in] */ long DevicePort);
        
        DECLSPEC_XFGVIRT(IAMExtDevice, get_DevicePort)
        HRESULT ( STDMETHODCALLTYPE *get_DevicePort )( 
            IAMExtDevice * This,
            /* [annotation][out] */ 
            _Out_  long *pDevicePort);
        
        END_INTERFACE
    } IAMExtDeviceVtbl;

    interface IAMExtDevice
    {
        CONST_VTBL struct IAMExtDeviceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMExtDevice_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMExtDevice_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMExtDevice_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMExtDevice_GetCapability(This,Capability,pValue,pdblValue)	\
    ( (This)->lpVtbl -> GetCapability(This,Capability,pValue,pdblValue) ) 

#define IAMExtDevice_get_ExternalDeviceID(This,ppszData)	\
    ( (This)->lpVtbl -> get_ExternalDeviceID(This,ppszData) ) 

#define IAMExtDevice_get_ExternalDeviceVersion(This,ppszData)	\
    ( (This)->lpVtbl -> get_ExternalDeviceVersion(This,ppszData) ) 

#define IAMExtDevice_put_DevicePower(This,PowerMode)	\
    ( (This)->lpVtbl -> put_DevicePower(This,PowerMode) ) 

#define IAMExtDevice_get_DevicePower(This,pPowerMode)	\
    ( (This)->lpVtbl -> get_DevicePower(This,pPowerMode) ) 

#define IAMExtDevice_Calibrate(This,hEvent,Mode,pStatus)	\
    ( (This)->lpVtbl -> Calibrate(This,hEvent,Mode,pStatus) ) 

#define IAMExtDevice_put_DevicePort(This,DevicePort)	\
    ( (This)->lpVtbl -> put_DevicePort(This,DevicePort) ) 

#define IAMExtDevice_get_DevicePort(This,pDevicePort)	\
    ( (This)->lpVtbl -> get_DevicePort(This,pDevicePort) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMExtDevice_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0073 */
/* [local] */ 

typedef IAMExtDevice *PEXTDEVICE;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0073_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0073_v0_0_s_ifspec;

#ifndef __IAMExtTransport_INTERFACE_DEFINED__
#define __IAMExtTransport_INTERFACE_DEFINED__

/* interface IAMExtTransport */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMExtTransport;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A03CD5F0-3045-11cf-8C44-00AA006B6814")
    IAMExtTransport : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapability( 
            /* [in] */ long Capability,
            /* [annotation][out] */ 
            _Out_  long *pValue,
            /* [annotation][out] */ 
            _Out_  double *pdblValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_MediaState( 
            /* [in] */ long State) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_MediaState( 
            /* [annotation][out] */ 
            _Out_  long *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_LocalControl( 
            /* [in] */ long State) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_LocalControl( 
            /* [annotation][out] */ 
            _Out_  long *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStatus( 
            /* [in] */ long StatusItem,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual /* [local] */ HRESULT STDMETHODCALLTYPE GetTransportBasicParameters( 
            /* [in] */ long Param,
            /* [annotation][out][in] */ 
            _Inout_  long *pValue,
            /* [annotation][out][in] */ 
            _Inout_  LPOLESTR *ppszData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransportBasicParameters( 
            /* [in] */ long Param,
            /* [in] */ long Value,
            /* [in] */ LPCOLESTR pszData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportVideoParameters( 
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransportVideoParameters( 
            /* [in] */ long Param,
            /* [in] */ long Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTransportAudioParameters( 
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTransportAudioParameters( 
            /* [in] */ long Param,
            /* [in] */ long Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Mode( 
            /* [in] */ long Mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Mode( 
            /* [annotation][out] */ 
            _Out_  long *pMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_Rate( 
            /* [in] */ double dblRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_Rate( 
            /* [annotation][out] */ 
            _Out_  double *pdblRate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetChase( 
            /* [annotation][out] */ 
            _Out_  long *pEnabled,
            /* [annotation][out] */ 
            _Out_  long *pOffset,
            /* [annotation][out] */ 
            _Out_  HEVENT *phEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetChase( 
            /* [in] */ long Enable,
            /* [in] */ long Offset,
            /* [in] */ HEVENT hEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBump( 
            /* [annotation][out] */ 
            _Out_  long *pSpeed,
            /* [annotation][out] */ 
            _Out_  long *pDuration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBump( 
            /* [in] */ long Speed,
            /* [in] */ long Duration) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_AntiClogControl( 
            /* [annotation][out] */ 
            _Out_  long *pEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_AntiClogControl( 
            /* [in] */ long Enable) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEditPropertySet( 
            /* [in] */ long EditID,
            /* [annotation][out] */ 
            _Out_  long *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEditPropertySet( 
            /* [out][in] */ long *pEditID,
            /* [in] */ long State) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetEditProperty( 
            /* [in] */ long EditID,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetEditProperty( 
            /* [in] */ long EditID,
            /* [in] */ long Param,
            /* [in] */ long Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_EditStart( 
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_EditStart( 
            /* [in] */ long Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMExtTransportVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMExtTransport * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMExtTransport * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMExtTransport * This);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetCapability)
        HRESULT ( STDMETHODCALLTYPE *GetCapability )( 
            IAMExtTransport * This,
            /* [in] */ long Capability,
            /* [annotation][out] */ 
            _Out_  long *pValue,
            /* [annotation][out] */ 
            _Out_  double *pdblValue);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, put_MediaState)
        HRESULT ( STDMETHODCALLTYPE *put_MediaState )( 
            IAMExtTransport * This,
            /* [in] */ long State);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, get_MediaState)
        HRESULT ( STDMETHODCALLTYPE *get_MediaState )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pState);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, put_LocalControl)
        HRESULT ( STDMETHODCALLTYPE *put_LocalControl )( 
            IAMExtTransport * This,
            /* [in] */ long State);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, get_LocalControl)
        HRESULT ( STDMETHODCALLTYPE *get_LocalControl )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pState);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetStatus)
        HRESULT ( STDMETHODCALLTYPE *GetStatus )( 
            IAMExtTransport * This,
            /* [in] */ long StatusItem,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetTransportBasicParameters)
        /* [local] */ HRESULT ( STDMETHODCALLTYPE *GetTransportBasicParameters )( 
            IAMExtTransport * This,
            /* [in] */ long Param,
            /* [annotation][out][in] */ 
            _Inout_  long *pValue,
            /* [annotation][out][in] */ 
            _Inout_  LPOLESTR *ppszData);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetTransportBasicParameters)
        HRESULT ( STDMETHODCALLTYPE *SetTransportBasicParameters )( 
            IAMExtTransport * This,
            /* [in] */ long Param,
            /* [in] */ long Value,
            /* [in] */ LPCOLESTR pszData);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetTransportVideoParameters)
        HRESULT ( STDMETHODCALLTYPE *GetTransportVideoParameters )( 
            IAMExtTransport * This,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetTransportVideoParameters)
        HRESULT ( STDMETHODCALLTYPE *SetTransportVideoParameters )( 
            IAMExtTransport * This,
            /* [in] */ long Param,
            /* [in] */ long Value);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetTransportAudioParameters)
        HRESULT ( STDMETHODCALLTYPE *GetTransportAudioParameters )( 
            IAMExtTransport * This,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetTransportAudioParameters)
        HRESULT ( STDMETHODCALLTYPE *SetTransportAudioParameters )( 
            IAMExtTransport * This,
            /* [in] */ long Param,
            /* [in] */ long Value);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, put_Mode)
        HRESULT ( STDMETHODCALLTYPE *put_Mode )( 
            IAMExtTransport * This,
            /* [in] */ long Mode);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, get_Mode)
        HRESULT ( STDMETHODCALLTYPE *get_Mode )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pMode);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, put_Rate)
        HRESULT ( STDMETHODCALLTYPE *put_Rate )( 
            IAMExtTransport * This,
            /* [in] */ double dblRate);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, get_Rate)
        HRESULT ( STDMETHODCALLTYPE *get_Rate )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  double *pdblRate);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetChase)
        HRESULT ( STDMETHODCALLTYPE *GetChase )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pEnabled,
            /* [annotation][out] */ 
            _Out_  long *pOffset,
            /* [annotation][out] */ 
            _Out_  HEVENT *phEvent);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetChase)
        HRESULT ( STDMETHODCALLTYPE *SetChase )( 
            IAMExtTransport * This,
            /* [in] */ long Enable,
            /* [in] */ long Offset,
            /* [in] */ HEVENT hEvent);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetBump)
        HRESULT ( STDMETHODCALLTYPE *GetBump )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pSpeed,
            /* [annotation][out] */ 
            _Out_  long *pDuration);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetBump)
        HRESULT ( STDMETHODCALLTYPE *SetBump )( 
            IAMExtTransport * This,
            /* [in] */ long Speed,
            /* [in] */ long Duration);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, get_AntiClogControl)
        HRESULT ( STDMETHODCALLTYPE *get_AntiClogControl )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pEnabled);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, put_AntiClogControl)
        HRESULT ( STDMETHODCALLTYPE *put_AntiClogControl )( 
            IAMExtTransport * This,
            /* [in] */ long Enable);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetEditPropertySet)
        HRESULT ( STDMETHODCALLTYPE *GetEditPropertySet )( 
            IAMExtTransport * This,
            /* [in] */ long EditID,
            /* [annotation][out] */ 
            _Out_  long *pState);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetEditPropertySet)
        HRESULT ( STDMETHODCALLTYPE *SetEditPropertySet )( 
            IAMExtTransport * This,
            /* [out][in] */ long *pEditID,
            /* [in] */ long State);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, GetEditProperty)
        HRESULT ( STDMETHODCALLTYPE *GetEditProperty )( 
            IAMExtTransport * This,
            /* [in] */ long EditID,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, SetEditProperty)
        HRESULT ( STDMETHODCALLTYPE *SetEditProperty )( 
            IAMExtTransport * This,
            /* [in] */ long EditID,
            /* [in] */ long Param,
            /* [in] */ long Value);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, get_EditStart)
        HRESULT ( STDMETHODCALLTYPE *get_EditStart )( 
            IAMExtTransport * This,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMExtTransport, put_EditStart)
        HRESULT ( STDMETHODCALLTYPE *put_EditStart )( 
            IAMExtTransport * This,
            /* [in] */ long Value);
        
        END_INTERFACE
    } IAMExtTransportVtbl;

    interface IAMExtTransport
    {
        CONST_VTBL struct IAMExtTransportVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMExtTransport_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMExtTransport_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMExtTransport_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMExtTransport_GetCapability(This,Capability,pValue,pdblValue)	\
    ( (This)->lpVtbl -> GetCapability(This,Capability,pValue,pdblValue) ) 

#define IAMExtTransport_put_MediaState(This,State)	\
    ( (This)->lpVtbl -> put_MediaState(This,State) ) 

#define IAMExtTransport_get_MediaState(This,pState)	\
    ( (This)->lpVtbl -> get_MediaState(This,pState) ) 

#define IAMExtTransport_put_LocalControl(This,State)	\
    ( (This)->lpVtbl -> put_LocalControl(This,State) ) 

#define IAMExtTransport_get_LocalControl(This,pState)	\
    ( (This)->lpVtbl -> get_LocalControl(This,pState) ) 

#define IAMExtTransport_GetStatus(This,StatusItem,pValue)	\
    ( (This)->lpVtbl -> GetStatus(This,StatusItem,pValue) ) 

#define IAMExtTransport_GetTransportBasicParameters(This,Param,pValue,ppszData)	\
    ( (This)->lpVtbl -> GetTransportBasicParameters(This,Param,pValue,ppszData) ) 

#define IAMExtTransport_SetTransportBasicParameters(This,Param,Value,pszData)	\
    ( (This)->lpVtbl -> SetTransportBasicParameters(This,Param,Value,pszData) ) 

#define IAMExtTransport_GetTransportVideoParameters(This,Param,pValue)	\
    ( (This)->lpVtbl -> GetTransportVideoParameters(This,Param,pValue) ) 

#define IAMExtTransport_SetTransportVideoParameters(This,Param,Value)	\
    ( (This)->lpVtbl -> SetTransportVideoParameters(This,Param,Value) ) 

#define IAMExtTransport_GetTransportAudioParameters(This,Param,pValue)	\
    ( (This)->lpVtbl -> GetTransportAudioParameters(This,Param,pValue) ) 

#define IAMExtTransport_SetTransportAudioParameters(This,Param,Value)	\
    ( (This)->lpVtbl -> SetTransportAudioParameters(This,Param,Value) ) 

#define IAMExtTransport_put_Mode(This,Mode)	\
    ( (This)->lpVtbl -> put_Mode(This,Mode) ) 

#define IAMExtTransport_get_Mode(This,pMode)	\
    ( (This)->lpVtbl -> get_Mode(This,pMode) ) 

#define IAMExtTransport_put_Rate(This,dblRate)	\
    ( (This)->lpVtbl -> put_Rate(This,dblRate) ) 

#define IAMExtTransport_get_Rate(This,pdblRate)	\
    ( (This)->lpVtbl -> get_Rate(This,pdblRate) ) 

#define IAMExtTransport_GetChase(This,pEnabled,pOffset,phEvent)	\
    ( (This)->lpVtbl -> GetChase(This,pEnabled,pOffset,phEvent) ) 

#define IAMExtTransport_SetChase(This,Enable,Offset,hEvent)	\
    ( (This)->lpVtbl -> SetChase(This,Enable,Offset,hEvent) ) 

#define IAMExtTransport_GetBump(This,pSpeed,pDuration)	\
    ( (This)->lpVtbl -> GetBump(This,pSpeed,pDuration) ) 

#define IAMExtTransport_SetBump(This,Speed,Duration)	\
    ( (This)->lpVtbl -> SetBump(This,Speed,Duration) ) 

#define IAMExtTransport_get_AntiClogControl(This,pEnabled)	\
    ( (This)->lpVtbl -> get_AntiClogControl(This,pEnabled) ) 

#define IAMExtTransport_put_AntiClogControl(This,Enable)	\
    ( (This)->lpVtbl -> put_AntiClogControl(This,Enable) ) 

#define IAMExtTransport_GetEditPropertySet(This,EditID,pState)	\
    ( (This)->lpVtbl -> GetEditPropertySet(This,EditID,pState) ) 

#define IAMExtTransport_SetEditPropertySet(This,pEditID,State)	\
    ( (This)->lpVtbl -> SetEditPropertySet(This,pEditID,State) ) 

#define IAMExtTransport_GetEditProperty(This,EditID,Param,pValue)	\
    ( (This)->lpVtbl -> GetEditProperty(This,EditID,Param,pValue) ) 

#define IAMExtTransport_SetEditProperty(This,EditID,Param,Value)	\
    ( (This)->lpVtbl -> SetEditProperty(This,EditID,Param,Value) ) 

#define IAMExtTransport_get_EditStart(This,pValue)	\
    ( (This)->lpVtbl -> get_EditStart(This,pValue) ) 

#define IAMExtTransport_put_EditStart(This,Value)	\
    ( (This)->lpVtbl -> put_EditStart(This,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMExtTransport_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0074 */
/* [local] */ 

typedef IAMExtTransport *PIAMEXTTRANSPORT;

#if 0
/* the following is what MIDL knows how to remote */
typedef struct tagTIMECODE
    {
    WORD wFrameRate;
    WORD wFrameFract;
    DWORD dwFrames;
    } 	TIMECODE;

#else /* 0 */
#ifndef TIMECODE_DEFINED
#define TIMECODE_DEFINED
typedef union _timecode {
   struct {
  WORD   wFrameRate;
  WORD   wFrameFract;
  DWORD  dwFrames;
  } DUMMYSTRUCTNAME;
   DWORDLONG  qw;
   } TIMECODE;

typedef struct tagTIMECODE_SAMPLE
    {
    LONGLONG qwTick;
    TIMECODE timecode;
    DWORD dwUser;
    DWORD dwFlags;
    } 	TIMECODE_SAMPLE;

#endif /* TIMECODE_DEFINED */
#endif /* 0 */
typedef TIMECODE *PTIMECODE;

typedef TIMECODE_SAMPLE *PTIMECODE_SAMPLE;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0074_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0074_v0_0_s_ifspec;

#ifndef __IAMTimecodeReader_INTERFACE_DEFINED__
#define __IAMTimecodeReader_INTERFACE_DEFINED__

/* interface IAMTimecodeReader */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMTimecodeReader;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B496CE1-811B-11cf-8C77-00AA006B6814")
    IAMTimecodeReader : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTCRMode( 
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTCRMode( 
            /* [in] */ long Param,
            /* [in] */ long Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_VITCLine( 
            /* [in] */ long Line) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_VITCLine( 
            /* [annotation][out] */ 
            _Out_  long *pLine) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimecode( 
            /* [annotation][out] */ 
            _Out_  PTIMECODE_SAMPLE pTimecodeSample) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTimecodeReaderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTimecodeReader * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTimecodeReader * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTimecodeReader * This);
        
        DECLSPEC_XFGVIRT(IAMTimecodeReader, GetTCRMode)
        HRESULT ( STDMETHODCALLTYPE *GetTCRMode )( 
            IAMTimecodeReader * This,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMTimecodeReader, SetTCRMode)
        HRESULT ( STDMETHODCALLTYPE *SetTCRMode )( 
            IAMTimecodeReader * This,
            /* [in] */ long Param,
            /* [in] */ long Value);
        
        DECLSPEC_XFGVIRT(IAMTimecodeReader, put_VITCLine)
        HRESULT ( STDMETHODCALLTYPE *put_VITCLine )( 
            IAMTimecodeReader * This,
            /* [in] */ long Line);
        
        DECLSPEC_XFGVIRT(IAMTimecodeReader, get_VITCLine)
        HRESULT ( STDMETHODCALLTYPE *get_VITCLine )( 
            IAMTimecodeReader * This,
            /* [annotation][out] */ 
            _Out_  long *pLine);
        
        DECLSPEC_XFGVIRT(IAMTimecodeReader, GetTimecode)
        HRESULT ( STDMETHODCALLTYPE *GetTimecode )( 
            IAMTimecodeReader * This,
            /* [annotation][out] */ 
            _Out_  PTIMECODE_SAMPLE pTimecodeSample);
        
        END_INTERFACE
    } IAMTimecodeReaderVtbl;

    interface IAMTimecodeReader
    {
        CONST_VTBL struct IAMTimecodeReaderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTimecodeReader_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTimecodeReader_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTimecodeReader_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTimecodeReader_GetTCRMode(This,Param,pValue)	\
    ( (This)->lpVtbl -> GetTCRMode(This,Param,pValue) ) 

#define IAMTimecodeReader_SetTCRMode(This,Param,Value)	\
    ( (This)->lpVtbl -> SetTCRMode(This,Param,Value) ) 

#define IAMTimecodeReader_put_VITCLine(This,Line)	\
    ( (This)->lpVtbl -> put_VITCLine(This,Line) ) 

#define IAMTimecodeReader_get_VITCLine(This,pLine)	\
    ( (This)->lpVtbl -> get_VITCLine(This,pLine) ) 

#define IAMTimecodeReader_GetTimecode(This,pTimecodeSample)	\
    ( (This)->lpVtbl -> GetTimecode(This,pTimecodeSample) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTimecodeReader_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0075 */
/* [local] */ 

typedef IAMTimecodeReader *PIAMTIMECODEREADER;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0075_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0075_v0_0_s_ifspec;

#ifndef __IAMTimecodeGenerator_INTERFACE_DEFINED__
#define __IAMTimecodeGenerator_INTERFACE_DEFINED__

/* interface IAMTimecodeGenerator */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMTimecodeGenerator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B496CE0-811B-11cf-8C77-00AA006B6814")
    IAMTimecodeGenerator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTCGMode( 
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTCGMode( 
            /* [in] */ long Param,
            /* [in] */ long Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_VITCLine( 
            /* [in] */ long Line) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE get_VITCLine( 
            /* [annotation][out] */ 
            _Out_  long *pLine) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimecode( 
            /* [in] */ PTIMECODE_SAMPLE pTimecodeSample) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTimecode( 
            /* [annotation][out] */ 
            _Out_  PTIMECODE_SAMPLE pTimecodeSample) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTimecodeGeneratorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTimecodeGenerator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTimecodeGenerator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTimecodeGenerator * This);
        
        DECLSPEC_XFGVIRT(IAMTimecodeGenerator, GetTCGMode)
        HRESULT ( STDMETHODCALLTYPE *GetTCGMode )( 
            IAMTimecodeGenerator * This,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMTimecodeGenerator, SetTCGMode)
        HRESULT ( STDMETHODCALLTYPE *SetTCGMode )( 
            IAMTimecodeGenerator * This,
            /* [in] */ long Param,
            /* [in] */ long Value);
        
        DECLSPEC_XFGVIRT(IAMTimecodeGenerator, put_VITCLine)
        HRESULT ( STDMETHODCALLTYPE *put_VITCLine )( 
            IAMTimecodeGenerator * This,
            /* [in] */ long Line);
        
        DECLSPEC_XFGVIRT(IAMTimecodeGenerator, get_VITCLine)
        HRESULT ( STDMETHODCALLTYPE *get_VITCLine )( 
            IAMTimecodeGenerator * This,
            /* [annotation][out] */ 
            _Out_  long *pLine);
        
        DECLSPEC_XFGVIRT(IAMTimecodeGenerator, SetTimecode)
        HRESULT ( STDMETHODCALLTYPE *SetTimecode )( 
            IAMTimecodeGenerator * This,
            /* [in] */ PTIMECODE_SAMPLE pTimecodeSample);
        
        DECLSPEC_XFGVIRT(IAMTimecodeGenerator, GetTimecode)
        HRESULT ( STDMETHODCALLTYPE *GetTimecode )( 
            IAMTimecodeGenerator * This,
            /* [annotation][out] */ 
            _Out_  PTIMECODE_SAMPLE pTimecodeSample);
        
        END_INTERFACE
    } IAMTimecodeGeneratorVtbl;

    interface IAMTimecodeGenerator
    {
        CONST_VTBL struct IAMTimecodeGeneratorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTimecodeGenerator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTimecodeGenerator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTimecodeGenerator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTimecodeGenerator_GetTCGMode(This,Param,pValue)	\
    ( (This)->lpVtbl -> GetTCGMode(This,Param,pValue) ) 

#define IAMTimecodeGenerator_SetTCGMode(This,Param,Value)	\
    ( (This)->lpVtbl -> SetTCGMode(This,Param,Value) ) 

#define IAMTimecodeGenerator_put_VITCLine(This,Line)	\
    ( (This)->lpVtbl -> put_VITCLine(This,Line) ) 

#define IAMTimecodeGenerator_get_VITCLine(This,pLine)	\
    ( (This)->lpVtbl -> get_VITCLine(This,pLine) ) 

#define IAMTimecodeGenerator_SetTimecode(This,pTimecodeSample)	\
    ( (This)->lpVtbl -> SetTimecode(This,pTimecodeSample) ) 

#define IAMTimecodeGenerator_GetTimecode(This,pTimecodeSample)	\
    ( (This)->lpVtbl -> GetTimecode(This,pTimecodeSample) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTimecodeGenerator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0076 */
/* [local] */ 

typedef IAMTimecodeGenerator *PIAMTIMECODEGENERATOR;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0076_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0076_v0_0_s_ifspec;

#ifndef __IAMTimecodeDisplay_INTERFACE_DEFINED__
#define __IAMTimecodeDisplay_INTERFACE_DEFINED__

/* interface IAMTimecodeDisplay */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMTimecodeDisplay;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9B496CE2-811B-11cf-8C77-00AA006B6814")
    IAMTimecodeDisplay : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTCDisplayEnable( 
            /* [annotation][out] */ 
            _Out_  long *pState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTCDisplayEnable( 
            /* [in] */ long State) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTCDisplay( 
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTCDisplay( 
            /* [in] */ long Param,
            /* [in] */ long Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMTimecodeDisplayVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMTimecodeDisplay * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMTimecodeDisplay * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMTimecodeDisplay * This);
        
        DECLSPEC_XFGVIRT(IAMTimecodeDisplay, GetTCDisplayEnable)
        HRESULT ( STDMETHODCALLTYPE *GetTCDisplayEnable )( 
            IAMTimecodeDisplay * This,
            /* [annotation][out] */ 
            _Out_  long *pState);
        
        DECLSPEC_XFGVIRT(IAMTimecodeDisplay, SetTCDisplayEnable)
        HRESULT ( STDMETHODCALLTYPE *SetTCDisplayEnable )( 
            IAMTimecodeDisplay * This,
            /* [in] */ long State);
        
        DECLSPEC_XFGVIRT(IAMTimecodeDisplay, GetTCDisplay)
        HRESULT ( STDMETHODCALLTYPE *GetTCDisplay )( 
            IAMTimecodeDisplay * This,
            /* [in] */ long Param,
            /* [annotation][out] */ 
            _Out_  long *pValue);
        
        DECLSPEC_XFGVIRT(IAMTimecodeDisplay, SetTCDisplay)
        HRESULT ( STDMETHODCALLTYPE *SetTCDisplay )( 
            IAMTimecodeDisplay * This,
            /* [in] */ long Param,
            /* [in] */ long Value);
        
        END_INTERFACE
    } IAMTimecodeDisplayVtbl;

    interface IAMTimecodeDisplay
    {
        CONST_VTBL struct IAMTimecodeDisplayVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMTimecodeDisplay_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMTimecodeDisplay_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMTimecodeDisplay_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMTimecodeDisplay_GetTCDisplayEnable(This,pState)	\
    ( (This)->lpVtbl -> GetTCDisplayEnable(This,pState) ) 

#define IAMTimecodeDisplay_SetTCDisplayEnable(This,State)	\
    ( (This)->lpVtbl -> SetTCDisplayEnable(This,State) ) 

#define IAMTimecodeDisplay_GetTCDisplay(This,Param,pValue)	\
    ( (This)->lpVtbl -> GetTCDisplay(This,Param,pValue) ) 

#define IAMTimecodeDisplay_SetTCDisplay(This,Param,Value)	\
    ( (This)->lpVtbl -> SetTCDisplay(This,Param,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMTimecodeDisplay_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0077 */
/* [local] */ 

typedef IAMTimecodeDisplay *PIAMTIMECODEDISPLAY;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0077_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0077_v0_0_s_ifspec;

#ifndef __IAMDevMemoryAllocator_INTERFACE_DEFINED__
#define __IAMDevMemoryAllocator_INTERFACE_DEFINED__

/* interface IAMDevMemoryAllocator */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMDevMemoryAllocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c6545bf0-e76b-11d0-bd52-00a0c911ce86")
    IAMDevMemoryAllocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetInfo( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbTotalFree,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbLargestFree,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbTotalMemory,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbMinimumChunk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckMemory( 
            /* [in] */ const BYTE *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Alloc( 
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pdwcbBuffer)  BYTE **ppBuffer,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwcbBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Free( 
            /* [in] */ BYTE *pBuffer) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDevMemoryObject( 
            /* [annotation][out] */ 
            _Out_  IUnknown **ppUnkInnner,
            /* [in] */ IUnknown *pUnkOuter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMDevMemoryAllocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMDevMemoryAllocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMDevMemoryAllocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMDevMemoryAllocator * This);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryAllocator, GetInfo)
        HRESULT ( STDMETHODCALLTYPE *GetInfo )( 
            IAMDevMemoryAllocator * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbTotalFree,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbLargestFree,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbTotalMemory,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwcbMinimumChunk);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryAllocator, CheckMemory)
        HRESULT ( STDMETHODCALLTYPE *CheckMemory )( 
            IAMDevMemoryAllocator * This,
            /* [in] */ const BYTE *pBuffer);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryAllocator, Alloc)
        HRESULT ( STDMETHODCALLTYPE *Alloc )( 
            IAMDevMemoryAllocator * This,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pdwcbBuffer)  BYTE **ppBuffer,
            /* [annotation][out][in] */ 
            _Inout_  DWORD *pdwcbBuffer);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryAllocator, Free)
        HRESULT ( STDMETHODCALLTYPE *Free )( 
            IAMDevMemoryAllocator * This,
            /* [in] */ BYTE *pBuffer);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryAllocator, GetDevMemoryObject)
        HRESULT ( STDMETHODCALLTYPE *GetDevMemoryObject )( 
            IAMDevMemoryAllocator * This,
            /* [annotation][out] */ 
            _Out_  IUnknown **ppUnkInnner,
            /* [in] */ IUnknown *pUnkOuter);
        
        END_INTERFACE
    } IAMDevMemoryAllocatorVtbl;

    interface IAMDevMemoryAllocator
    {
        CONST_VTBL struct IAMDevMemoryAllocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMDevMemoryAllocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMDevMemoryAllocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMDevMemoryAllocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMDevMemoryAllocator_GetInfo(This,pdwcbTotalFree,pdwcbLargestFree,pdwcbTotalMemory,pdwcbMinimumChunk)	\
    ( (This)->lpVtbl -> GetInfo(This,pdwcbTotalFree,pdwcbLargestFree,pdwcbTotalMemory,pdwcbMinimumChunk) ) 

#define IAMDevMemoryAllocator_CheckMemory(This,pBuffer)	\
    ( (This)->lpVtbl -> CheckMemory(This,pBuffer) ) 

#define IAMDevMemoryAllocator_Alloc(This,ppBuffer,pdwcbBuffer)	\
    ( (This)->lpVtbl -> Alloc(This,ppBuffer,pdwcbBuffer) ) 

#define IAMDevMemoryAllocator_Free(This,pBuffer)	\
    ( (This)->lpVtbl -> Free(This,pBuffer) ) 

#define IAMDevMemoryAllocator_GetDevMemoryObject(This,ppUnkInnner,pUnkOuter)	\
    ( (This)->lpVtbl -> GetDevMemoryObject(This,ppUnkInnner,pUnkOuter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMDevMemoryAllocator_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0078 */
/* [local] */ 

typedef IAMDevMemoryAllocator *PAMDEVMEMORYALLOCATOR;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0078_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0078_v0_0_s_ifspec;

#ifndef __IAMDevMemoryControl_INTERFACE_DEFINED__
#define __IAMDevMemoryControl_INTERFACE_DEFINED__

/* interface IAMDevMemoryControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMDevMemoryControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c6545bf1-e76b-11d0-bd52-00a0c911ce86")
    IAMDevMemoryControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryWriteSync( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WriteSync( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDevId( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDevId) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMDevMemoryControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMDevMemoryControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMDevMemoryControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMDevMemoryControl * This);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryControl, QueryWriteSync)
        HRESULT ( STDMETHODCALLTYPE *QueryWriteSync )( 
            IAMDevMemoryControl * This);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryControl, WriteSync)
        HRESULT ( STDMETHODCALLTYPE *WriteSync )( 
            IAMDevMemoryControl * This);
        
        DECLSPEC_XFGVIRT(IAMDevMemoryControl, GetDevId)
        HRESULT ( STDMETHODCALLTYPE *GetDevId )( 
            IAMDevMemoryControl * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwDevId);
        
        END_INTERFACE
    } IAMDevMemoryControlVtbl;

    interface IAMDevMemoryControl
    {
        CONST_VTBL struct IAMDevMemoryControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMDevMemoryControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMDevMemoryControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMDevMemoryControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMDevMemoryControl_QueryWriteSync(This)	\
    ( (This)->lpVtbl -> QueryWriteSync(This) ) 

#define IAMDevMemoryControl_WriteSync(This)	\
    ( (This)->lpVtbl -> WriteSync(This) ) 

#define IAMDevMemoryControl_GetDevId(This,pdwDevId)	\
    ( (This)->lpVtbl -> GetDevId(This,pdwDevId) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMDevMemoryControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0079 */
/* [local] */ 

typedef IAMDevMemoryControl *PAMDEVMEMORYCONTROL;


enum _AMSTREAMSELECTINFOFLAGS
    {
        AMSTREAMSELECTINFO_ENABLED	= 0x1,
        AMSTREAMSELECTINFO_EXCLUSIVE	= 0x2
    } ;

enum _AMSTREAMSELECTENABLEFLAGS
    {
        AMSTREAMSELECTENABLE_ENABLE	= 0x1,
        AMSTREAMSELECTENABLE_ENABLEALL	= 0x2
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0079_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0079_v0_0_s_ifspec;

#ifndef __IAMStreamSelect_INTERFACE_DEFINED__
#define __IAMStreamSelect_INTERFACE_DEFINED__

/* interface IAMStreamSelect */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMStreamSelect;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c1960960-17f5-11d1-abe1-00a0c905f375")
    IAMStreamSelect : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Count( 
            /* [annotation][out] */ 
            _Out_  DWORD *pcStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Info( 
            /* [in] */ long lIndex,
            /* [annotation][out] */ 
            _Out_opt_  AM_MEDIA_TYPE **ppmt,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwFlags,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plcid,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwGroup,
            /* [annotation][out] */ 
            _Out_opt_  LPWSTR *ppszName,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppObject,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnk) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Enable( 
            /* [in] */ long lIndex,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMStreamSelectVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMStreamSelect * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMStreamSelect * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMStreamSelect * This);
        
        DECLSPEC_XFGVIRT(IAMStreamSelect, Count)
        HRESULT ( STDMETHODCALLTYPE *Count )( 
            IAMStreamSelect * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pcStreams);
        
        DECLSPEC_XFGVIRT(IAMStreamSelect, Info)
        HRESULT ( STDMETHODCALLTYPE *Info )( 
            IAMStreamSelect * This,
            /* [in] */ long lIndex,
            /* [annotation][out] */ 
            _Out_opt_  AM_MEDIA_TYPE **ppmt,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwFlags,
            /* [annotation][out] */ 
            _Out_opt_  LCID *plcid,
            /* [annotation][out] */ 
            _Out_opt_  DWORD *pdwGroup,
            /* [annotation][out] */ 
            _Out_opt_  LPWSTR *ppszName,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppObject,
            /* [annotation][out] */ 
            _Out_opt_  IUnknown **ppUnk);
        
        DECLSPEC_XFGVIRT(IAMStreamSelect, Enable)
        HRESULT ( STDMETHODCALLTYPE *Enable )( 
            IAMStreamSelect * This,
            /* [in] */ long lIndex,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IAMStreamSelectVtbl;

    interface IAMStreamSelect
    {
        CONST_VTBL struct IAMStreamSelectVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMStreamSelect_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMStreamSelect_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMStreamSelect_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMStreamSelect_Count(This,pcStreams)	\
    ( (This)->lpVtbl -> Count(This,pcStreams) ) 

#define IAMStreamSelect_Info(This,lIndex,ppmt,pdwFlags,plcid,pdwGroup,ppszName,ppObject,ppUnk)	\
    ( (This)->lpVtbl -> Info(This,lIndex,ppmt,pdwFlags,plcid,pdwGroup,ppszName,ppObject,ppUnk) ) 

#define IAMStreamSelect_Enable(This,lIndex,dwFlags)	\
    ( (This)->lpVtbl -> Enable(This,lIndex,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMStreamSelect_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0080 */
/* [local] */ 

typedef IAMStreamSelect *PAMSTREAMSELECT;


enum _AMRESCTL_RESERVEFLAGS
    {
        AMRESCTL_RESERVEFLAGS_RESERVE	= 0,
        AMRESCTL_RESERVEFLAGS_UNRESERVE	= 0x1
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0080_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0080_v0_0_s_ifspec;

#ifndef __IAMResourceControl_INTERFACE_DEFINED__
#define __IAMResourceControl_INTERFACE_DEFINED__

/* interface IAMResourceControl */
/* [local][unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMResourceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8389d2d0-77d7-11d1-abe6-00a0c905f375")
    IAMResourceControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reserve( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][in] */ 
            _Reserved_  PVOID pvReserved) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMResourceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMResourceControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMResourceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMResourceControl * This);
        
        DECLSPEC_XFGVIRT(IAMResourceControl, Reserve)
        HRESULT ( STDMETHODCALLTYPE *Reserve )( 
            IAMResourceControl * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][in] */ 
            _Reserved_  PVOID pvReserved);
        
        END_INTERFACE
    } IAMResourceControlVtbl;

    interface IAMResourceControl
    {
        CONST_VTBL struct IAMResourceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMResourceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMResourceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMResourceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMResourceControl_Reserve(This,dwFlags,pvReserved)	\
    ( (This)->lpVtbl -> Reserve(This,dwFlags,pvReserved) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMResourceControl_INTERFACE_DEFINED__ */


#ifndef __IAMClockAdjust_INTERFACE_DEFINED__
#define __IAMClockAdjust_INTERFACE_DEFINED__

/* interface IAMClockAdjust */
/* [local][unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMClockAdjust;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4d5466b0-a49c-11d1-abe8-00a0c905f375")
    IAMClockAdjust : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetClockDelta( 
            /* [in] */ REFERENCE_TIME rtDelta) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMClockAdjustVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMClockAdjust * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMClockAdjust * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMClockAdjust * This);
        
        DECLSPEC_XFGVIRT(IAMClockAdjust, SetClockDelta)
        HRESULT ( STDMETHODCALLTYPE *SetClockDelta )( 
            IAMClockAdjust * This,
            /* [in] */ REFERENCE_TIME rtDelta);
        
        END_INTERFACE
    } IAMClockAdjustVtbl;

    interface IAMClockAdjust
    {
        CONST_VTBL struct IAMClockAdjustVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMClockAdjust_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMClockAdjust_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMClockAdjust_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMClockAdjust_SetClockDelta(This,rtDelta)	\
    ( (This)->lpVtbl -> SetClockDelta(This,rtDelta) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMClockAdjust_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0082 */
/* [local] */ 


enum _AM_FILTER_MISC_FLAGS
    {
        AM_FILTER_MISC_FLAGS_IS_RENDERER	= 0x1,
        AM_FILTER_MISC_FLAGS_IS_SOURCE	= 0x2
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0082_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0082_v0_0_s_ifspec;

#ifndef __IAMFilterMiscFlags_INTERFACE_DEFINED__
#define __IAMFilterMiscFlags_INTERFACE_DEFINED__

/* interface IAMFilterMiscFlags */
/* [local][unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMFilterMiscFlags;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2dd74950-a890-11d1-abe8-00a0c905f375")
    IAMFilterMiscFlags : public IUnknown
    {
    public:
        virtual ULONG STDMETHODCALLTYPE GetMiscFlags( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMFilterMiscFlagsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMFilterMiscFlags * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMFilterMiscFlags * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMFilterMiscFlags * This);
        
        DECLSPEC_XFGVIRT(IAMFilterMiscFlags, GetMiscFlags)
        ULONG ( STDMETHODCALLTYPE *GetMiscFlags )( 
            IAMFilterMiscFlags * This);
        
        END_INTERFACE
    } IAMFilterMiscFlagsVtbl;

    interface IAMFilterMiscFlags
    {
        CONST_VTBL struct IAMFilterMiscFlagsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMFilterMiscFlags_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMFilterMiscFlags_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMFilterMiscFlags_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMFilterMiscFlags_GetMiscFlags(This)	\
    ( (This)->lpVtbl -> GetMiscFlags(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMFilterMiscFlags_INTERFACE_DEFINED__ */


#ifndef __IDrawVideoImage_INTERFACE_DEFINED__
#define __IDrawVideoImage_INTERFACE_DEFINED__

/* interface IDrawVideoImage */
/* [unique][uuid][local][object][local] */ 


EXTERN_C const IID IID_IDrawVideoImage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("48efb120-ab49-11d2-aed2-00a0c995e8d5")
    IDrawVideoImage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DrawVideoImageBegin( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DrawVideoImageEnd( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DrawVideoImageDraw( 
            /* [in] */ HDC hdc,
            /* [annotation][in] */ 
            _In_  LPRECT lprcSrc,
            /* [annotation][in] */ 
            _In_  LPRECT lprcDst) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDrawVideoImageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDrawVideoImage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDrawVideoImage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDrawVideoImage * This);
        
        DECLSPEC_XFGVIRT(IDrawVideoImage, DrawVideoImageBegin)
        HRESULT ( STDMETHODCALLTYPE *DrawVideoImageBegin )( 
            IDrawVideoImage * This);
        
        DECLSPEC_XFGVIRT(IDrawVideoImage, DrawVideoImageEnd)
        HRESULT ( STDMETHODCALLTYPE *DrawVideoImageEnd )( 
            IDrawVideoImage * This);
        
        DECLSPEC_XFGVIRT(IDrawVideoImage, DrawVideoImageDraw)
        HRESULT ( STDMETHODCALLTYPE *DrawVideoImageDraw )( 
            IDrawVideoImage * This,
            /* [in] */ HDC hdc,
            /* [annotation][in] */ 
            _In_  LPRECT lprcSrc,
            /* [annotation][in] */ 
            _In_  LPRECT lprcDst);
        
        END_INTERFACE
    } IDrawVideoImageVtbl;

    interface IDrawVideoImage
    {
        CONST_VTBL struct IDrawVideoImageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDrawVideoImage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDrawVideoImage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDrawVideoImage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDrawVideoImage_DrawVideoImageBegin(This)	\
    ( (This)->lpVtbl -> DrawVideoImageBegin(This) ) 

#define IDrawVideoImage_DrawVideoImageEnd(This)	\
    ( (This)->lpVtbl -> DrawVideoImageEnd(This) ) 

#define IDrawVideoImage_DrawVideoImageDraw(This,hdc,lprcSrc,lprcDst)	\
    ( (This)->lpVtbl -> DrawVideoImageDraw(This,hdc,lprcSrc,lprcDst) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDrawVideoImage_INTERFACE_DEFINED__ */


#ifndef __IDecimateVideoImage_INTERFACE_DEFINED__
#define __IDecimateVideoImage_INTERFACE_DEFINED__

/* interface IDecimateVideoImage */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDecimateVideoImage;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("2e5ea3e0-e924-11d2-b6da-00a0c995e8df")
    IDecimateVideoImage : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDecimationImageSize( 
            /* [in] */ long lWidth,
            /* [in] */ long lHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ResetDecimationImageSize( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDecimateVideoImageVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDecimateVideoImage * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDecimateVideoImage * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDecimateVideoImage * This);
        
        DECLSPEC_XFGVIRT(IDecimateVideoImage, SetDecimationImageSize)
        HRESULT ( STDMETHODCALLTYPE *SetDecimationImageSize )( 
            IDecimateVideoImage * This,
            /* [in] */ long lWidth,
            /* [in] */ long lHeight);
        
        DECLSPEC_XFGVIRT(IDecimateVideoImage, ResetDecimationImageSize)
        HRESULT ( STDMETHODCALLTYPE *ResetDecimationImageSize )( 
            IDecimateVideoImage * This);
        
        END_INTERFACE
    } IDecimateVideoImageVtbl;

    interface IDecimateVideoImage
    {
        CONST_VTBL struct IDecimateVideoImageVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDecimateVideoImage_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDecimateVideoImage_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDecimateVideoImage_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDecimateVideoImage_SetDecimationImageSize(This,lWidth,lHeight)	\
    ( (This)->lpVtbl -> SetDecimationImageSize(This,lWidth,lHeight) ) 

#define IDecimateVideoImage_ResetDecimationImageSize(This)	\
    ( (This)->lpVtbl -> ResetDecimationImageSize(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDecimateVideoImage_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0085 */
/* [local] */ 

typedef 
enum _DECIMATION_USAGE
    {
        DECIMATION_LEGACY	= 0,
        DECIMATION_USE_DECODER_ONLY	= ( DECIMATION_LEGACY + 1 ) ,
        DECIMATION_USE_VIDEOPORT_ONLY	= ( DECIMATION_USE_DECODER_ONLY + 1 ) ,
        DECIMATION_USE_OVERLAY_ONLY	= ( DECIMATION_USE_VIDEOPORT_ONLY + 1 ) ,
        DECIMATION_DEFAULT	= ( DECIMATION_USE_OVERLAY_ONLY + 1 ) 
    } 	DECIMATION_USAGE;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0085_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0085_v0_0_s_ifspec;

#ifndef __IAMVideoDecimationProperties_INTERFACE_DEFINED__
#define __IAMVideoDecimationProperties_INTERFACE_DEFINED__

/* interface IAMVideoDecimationProperties */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMVideoDecimationProperties;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("60d32930-13da-11d3-9ec6-c4fcaef5c7be")
    IAMVideoDecimationProperties : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryDecimationUsage( 
            /* [annotation][out] */ 
            _Out_  DECIMATION_USAGE *lpUsage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDecimationUsage( 
            /* [in] */ DECIMATION_USAGE Usage) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMVideoDecimationPropertiesVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMVideoDecimationProperties * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMVideoDecimationProperties * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMVideoDecimationProperties * This);
        
        DECLSPEC_XFGVIRT(IAMVideoDecimationProperties, QueryDecimationUsage)
        HRESULT ( STDMETHODCALLTYPE *QueryDecimationUsage )( 
            IAMVideoDecimationProperties * This,
            /* [annotation][out] */ 
            _Out_  DECIMATION_USAGE *lpUsage);
        
        DECLSPEC_XFGVIRT(IAMVideoDecimationProperties, SetDecimationUsage)
        HRESULT ( STDMETHODCALLTYPE *SetDecimationUsage )( 
            IAMVideoDecimationProperties * This,
            /* [in] */ DECIMATION_USAGE Usage);
        
        END_INTERFACE
    } IAMVideoDecimationPropertiesVtbl;

    interface IAMVideoDecimationProperties
    {
        CONST_VTBL struct IAMVideoDecimationPropertiesVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMVideoDecimationProperties_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMVideoDecimationProperties_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMVideoDecimationProperties_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMVideoDecimationProperties_QueryDecimationUsage(This,lpUsage)	\
    ( (This)->lpVtbl -> QueryDecimationUsage(This,lpUsage) ) 

#define IAMVideoDecimationProperties_SetDecimationUsage(This,Usage)	\
    ( (This)->lpVtbl -> SetDecimationUsage(This,Usage) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMVideoDecimationProperties_INTERFACE_DEFINED__ */


#ifndef __IVideoFrameStep_INTERFACE_DEFINED__
#define __IVideoFrameStep_INTERFACE_DEFINED__

/* interface IVideoFrameStep */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IVideoFrameStep;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e46a9787-2b71-444d-a4b5-1fab7b708d6a")
    IVideoFrameStep : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Step( 
            DWORD dwFrames,
            /* [annotation][unique] */ 
            _In_opt_  IUnknown *pStepObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CanStep( 
            long bMultiple,
            /* [annotation][unique] */ 
            _In_opt_  IUnknown *pStepObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CancelStep( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVideoFrameStepVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVideoFrameStep * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVideoFrameStep * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVideoFrameStep * This);
        
        DECLSPEC_XFGVIRT(IVideoFrameStep, Step)
        HRESULT ( STDMETHODCALLTYPE *Step )( 
            IVideoFrameStep * This,
            DWORD dwFrames,
            /* [annotation][unique] */ 
            _In_opt_  IUnknown *pStepObject);
        
        DECLSPEC_XFGVIRT(IVideoFrameStep, CanStep)
        HRESULT ( STDMETHODCALLTYPE *CanStep )( 
            IVideoFrameStep * This,
            long bMultiple,
            /* [annotation][unique] */ 
            _In_opt_  IUnknown *pStepObject);
        
        DECLSPEC_XFGVIRT(IVideoFrameStep, CancelStep)
        HRESULT ( STDMETHODCALLTYPE *CancelStep )( 
            IVideoFrameStep * This);
        
        END_INTERFACE
    } IVideoFrameStepVtbl;

    interface IVideoFrameStep
    {
        CONST_VTBL struct IVideoFrameStepVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVideoFrameStep_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVideoFrameStep_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVideoFrameStep_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVideoFrameStep_Step(This,dwFrames,pStepObject)	\
    ( (This)->lpVtbl -> Step(This,dwFrames,pStepObject) ) 

#define IVideoFrameStep_CanStep(This,bMultiple,pStepObject)	\
    ( (This)->lpVtbl -> CanStep(This,bMultiple,pStepObject) ) 

#define IVideoFrameStep_CancelStep(This)	\
    ( (This)->lpVtbl -> CancelStep(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVideoFrameStep_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0087 */
/* [local] */ 


enum _AM_PUSHSOURCE_FLAGS
    {
        AM_PUSHSOURCECAPS_INTERNAL_RM	= 0x1,
        AM_PUSHSOURCECAPS_NOT_LIVE	= 0x2,
        AM_PUSHSOURCECAPS_PRIVATE_CLOCK	= 0x4,
        AM_PUSHSOURCEREQS_USE_STREAM_CLOCK	= 0x10000,
        AM_PUSHSOURCEREQS_USE_CLOCK_CHAIN	= 0x20000
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0087_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0087_v0_0_s_ifspec;

#ifndef __IAMLatency_INTERFACE_DEFINED__
#define __IAMLatency_INTERFACE_DEFINED__

/* interface IAMLatency */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMLatency;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("62EA93BA-EC62-11d2-B770-00C04FB6BD3D")
    IAMLatency : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetLatency( 
            /* [annotation][in] */ 
            _Out_  REFERENCE_TIME *prtLatency) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMLatencyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMLatency * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMLatency * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMLatency * This);
        
        DECLSPEC_XFGVIRT(IAMLatency, GetLatency)
        HRESULT ( STDMETHODCALLTYPE *GetLatency )( 
            IAMLatency * This,
            /* [annotation][in] */ 
            _Out_  REFERENCE_TIME *prtLatency);
        
        END_INTERFACE
    } IAMLatencyVtbl;

    interface IAMLatency
    {
        CONST_VTBL struct IAMLatencyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMLatency_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMLatency_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMLatency_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMLatency_GetLatency(This,prtLatency)	\
    ( (This)->lpVtbl -> GetLatency(This,prtLatency) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMLatency_INTERFACE_DEFINED__ */


#ifndef __IAMPushSource_INTERFACE_DEFINED__
#define __IAMPushSource_INTERFACE_DEFINED__

/* interface IAMPushSource */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMPushSource;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("F185FE76-E64E-11d2-B76E-00C04FB6BD3D")
    IAMPushSource : public IAMLatency
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPushSourceFlags( 
            /* [annotation][out] */ 
            _Out_  ULONG *pFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPushSourceFlags( 
            /* [in] */ ULONG Flags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamOffset( 
            /* [in] */ REFERENCE_TIME rtOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamOffset( 
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxStreamOffset( 
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtMaxOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxStreamOffset( 
            /* [in] */ REFERENCE_TIME rtMaxOffset) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMPushSourceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMPushSource * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMPushSource * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMPushSource * This);
        
        DECLSPEC_XFGVIRT(IAMLatency, GetLatency)
        HRESULT ( STDMETHODCALLTYPE *GetLatency )( 
            IAMPushSource * This,
            /* [annotation][in] */ 
            _Out_  REFERENCE_TIME *prtLatency);
        
        DECLSPEC_XFGVIRT(IAMPushSource, GetPushSourceFlags)
        HRESULT ( STDMETHODCALLTYPE *GetPushSourceFlags )( 
            IAMPushSource * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pFlags);
        
        DECLSPEC_XFGVIRT(IAMPushSource, SetPushSourceFlags)
        HRESULT ( STDMETHODCALLTYPE *SetPushSourceFlags )( 
            IAMPushSource * This,
            /* [in] */ ULONG Flags);
        
        DECLSPEC_XFGVIRT(IAMPushSource, SetStreamOffset)
        HRESULT ( STDMETHODCALLTYPE *SetStreamOffset )( 
            IAMPushSource * This,
            /* [in] */ REFERENCE_TIME rtOffset);
        
        DECLSPEC_XFGVIRT(IAMPushSource, GetStreamOffset)
        HRESULT ( STDMETHODCALLTYPE *GetStreamOffset )( 
            IAMPushSource * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtOffset);
        
        DECLSPEC_XFGVIRT(IAMPushSource, GetMaxStreamOffset)
        HRESULT ( STDMETHODCALLTYPE *GetMaxStreamOffset )( 
            IAMPushSource * This,
            /* [annotation][out] */ 
            _Out_  REFERENCE_TIME *prtMaxOffset);
        
        DECLSPEC_XFGVIRT(IAMPushSource, SetMaxStreamOffset)
        HRESULT ( STDMETHODCALLTYPE *SetMaxStreamOffset )( 
            IAMPushSource * This,
            /* [in] */ REFERENCE_TIME rtMaxOffset);
        
        END_INTERFACE
    } IAMPushSourceVtbl;

    interface IAMPushSource
    {
        CONST_VTBL struct IAMPushSourceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMPushSource_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMPushSource_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMPushSource_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMPushSource_GetLatency(This,prtLatency)	\
    ( (This)->lpVtbl -> GetLatency(This,prtLatency) ) 


#define IAMPushSource_GetPushSourceFlags(This,pFlags)	\
    ( (This)->lpVtbl -> GetPushSourceFlags(This,pFlags) ) 

#define IAMPushSource_SetPushSourceFlags(This,Flags)	\
    ( (This)->lpVtbl -> SetPushSourceFlags(This,Flags) ) 

#define IAMPushSource_SetStreamOffset(This,rtOffset)	\
    ( (This)->lpVtbl -> SetStreamOffset(This,rtOffset) ) 

#define IAMPushSource_GetStreamOffset(This,prtOffset)	\
    ( (This)->lpVtbl -> GetStreamOffset(This,prtOffset) ) 

#define IAMPushSource_GetMaxStreamOffset(This,prtMaxOffset)	\
    ( (This)->lpVtbl -> GetMaxStreamOffset(This,prtMaxOffset) ) 

#define IAMPushSource_SetMaxStreamOffset(This,rtMaxOffset)	\
    ( (This)->lpVtbl -> SetMaxStreamOffset(This,rtMaxOffset) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMPushSource_INTERFACE_DEFINED__ */


#ifndef __IAMDeviceRemoval_INTERFACE_DEFINED__
#define __IAMDeviceRemoval_INTERFACE_DEFINED__

/* interface IAMDeviceRemoval */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMDeviceRemoval;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("f90a6130-b658-11d2-ae49-0000f8754b99")
    IAMDeviceRemoval : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DeviceInfo( 
            /* [annotation][out] */ 
            _Out_  CLSID *pclsidInterfaceClass,
            /* [annotation][out] */ 
            _Out_  LPWSTR *pwszSymbolicLink) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reassociate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Disassociate( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMDeviceRemovalVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMDeviceRemoval * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMDeviceRemoval * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMDeviceRemoval * This);
        
        DECLSPEC_XFGVIRT(IAMDeviceRemoval, DeviceInfo)
        HRESULT ( STDMETHODCALLTYPE *DeviceInfo )( 
            IAMDeviceRemoval * This,
            /* [annotation][out] */ 
            _Out_  CLSID *pclsidInterfaceClass,
            /* [annotation][out] */ 
            _Out_  LPWSTR *pwszSymbolicLink);
        
        DECLSPEC_XFGVIRT(IAMDeviceRemoval, Reassociate)
        HRESULT ( STDMETHODCALLTYPE *Reassociate )( 
            IAMDeviceRemoval * This);
        
        DECLSPEC_XFGVIRT(IAMDeviceRemoval, Disassociate)
        HRESULT ( STDMETHODCALLTYPE *Disassociate )( 
            IAMDeviceRemoval * This);
        
        END_INTERFACE
    } IAMDeviceRemovalVtbl;

    interface IAMDeviceRemoval
    {
        CONST_VTBL struct IAMDeviceRemovalVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMDeviceRemoval_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMDeviceRemoval_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMDeviceRemoval_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMDeviceRemoval_DeviceInfo(This,pclsidInterfaceClass,pwszSymbolicLink)	\
    ( (This)->lpVtbl -> DeviceInfo(This,pclsidInterfaceClass,pwszSymbolicLink) ) 

#define IAMDeviceRemoval_Reassociate(This)	\
    ( (This)->lpVtbl -> Reassociate(This) ) 

#define IAMDeviceRemoval_Disassociate(This)	\
    ( (This)->lpVtbl -> Disassociate(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMDeviceRemoval_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0089 */
/* [local] */ 

typedef struct DVINFO
    {
    DWORD dwDVAAuxSrc;
    DWORD dwDVAAuxCtl;
    DWORD dwDVAAuxSrc1;
    DWORD dwDVAAuxCtl1;
    DWORD dwDVVAuxSrc;
    DWORD dwDVVAuxCtl;
    DWORD dwDVReserved[ 2 ];
    } 	DVINFO;

typedef struct DVINFO *PDVINFO;


enum _DVENCODERRESOLUTION
    {
        DVENCODERRESOLUTION_720x480	= 2012,
        DVENCODERRESOLUTION_360x240	= 2013,
        DVENCODERRESOLUTION_180x120	= 2014,
        DVENCODERRESOLUTION_88x60	= 2015
    } ;

enum _DVENCODERVIDEOFORMAT
    {
        DVENCODERVIDEOFORMAT_NTSC	= 2000,
        DVENCODERVIDEOFORMAT_PAL	= 2001
    } ;

enum _DVENCODERFORMAT
    {
        DVENCODERFORMAT_DVSD	= 2007,
        DVENCODERFORMAT_DVHD	= 2008,
        DVENCODERFORMAT_DVSL	= 2009
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0089_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0089_v0_0_s_ifspec;

#ifndef __IDVEnc_INTERFACE_DEFINED__
#define __IDVEnc_INTERFACE_DEFINED__

/* interface IDVEnc */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVEnc;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("d18e17a0-aacb-11d0-afb0-00aa00b67a42")
    IDVEnc : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_IFormatResolution( 
            /* [annotation][out] */ 
            _Out_  int *VideoFormat,
            /* [annotation][out] */ 
            _Out_  int *DVFormat,
            /* [annotation][out] */ 
            _Out_  int *Resolution,
            /* [in] */ BYTE fDVInfo,
            /* [annotation][out] */ 
            _Out_  DVINFO *sDVInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_IFormatResolution( 
            /* [in] */ int VideoFormat,
            /* [in] */ int DVFormat,
            /* [in] */ int Resolution,
            /* [in] */ BYTE fDVInfo,
            /* [annotation][in] */ 
            _In_  DVINFO *sDVInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVEncVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVEnc * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVEnc * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVEnc * This);
        
        DECLSPEC_XFGVIRT(IDVEnc, get_IFormatResolution)
        HRESULT ( STDMETHODCALLTYPE *get_IFormatResolution )( 
            IDVEnc * This,
            /* [annotation][out] */ 
            _Out_  int *VideoFormat,
            /* [annotation][out] */ 
            _Out_  int *DVFormat,
            /* [annotation][out] */ 
            _Out_  int *Resolution,
            /* [in] */ BYTE fDVInfo,
            /* [annotation][out] */ 
            _Out_  DVINFO *sDVInfo);
        
        DECLSPEC_XFGVIRT(IDVEnc, put_IFormatResolution)
        HRESULT ( STDMETHODCALLTYPE *put_IFormatResolution )( 
            IDVEnc * This,
            /* [in] */ int VideoFormat,
            /* [in] */ int DVFormat,
            /* [in] */ int Resolution,
            /* [in] */ BYTE fDVInfo,
            /* [annotation][in] */ 
            _In_  DVINFO *sDVInfo);
        
        END_INTERFACE
    } IDVEncVtbl;

    interface IDVEnc
    {
        CONST_VTBL struct IDVEncVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVEnc_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVEnc_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVEnc_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVEnc_get_IFormatResolution(This,VideoFormat,DVFormat,Resolution,fDVInfo,sDVInfo)	\
    ( (This)->lpVtbl -> get_IFormatResolution(This,VideoFormat,DVFormat,Resolution,fDVInfo,sDVInfo) ) 

#define IDVEnc_put_IFormatResolution(This,VideoFormat,DVFormat,Resolution,fDVInfo,sDVInfo)	\
    ( (This)->lpVtbl -> put_IFormatResolution(This,VideoFormat,DVFormat,Resolution,fDVInfo,sDVInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVEnc_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0090 */
/* [local] */ 


enum _DVDECODERRESOLUTION
    {
        DVDECODERRESOLUTION_720x480	= 1000,
        DVDECODERRESOLUTION_360x240	= 1001,
        DVDECODERRESOLUTION_180x120	= 1002,
        DVDECODERRESOLUTION_88x60	= 1003
    } ;

enum _DVRESOLUTION
    {
        DVRESOLUTION_FULL	= 1000,
        DVRESOLUTION_HALF	= 1001,
        DVRESOLUTION_QUARTER	= 1002,
        DVRESOLUTION_DC	= 1003
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0090_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0090_v0_0_s_ifspec;

#ifndef __IIPDVDec_INTERFACE_DEFINED__
#define __IIPDVDec_INTERFACE_DEFINED__

/* interface IIPDVDec */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IIPDVDec;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("b8e8bd60-0bfe-11d0-af91-00aa00b67a42")
    IIPDVDec : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE get_IPDisplay( 
            /* [annotation][out] */ 
            _Out_  int *displayPix) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE put_IPDisplay( 
            /* [in] */ int displayPix) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IIPDVDecVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IIPDVDec * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IIPDVDec * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IIPDVDec * This);
        
        DECLSPEC_XFGVIRT(IIPDVDec, get_IPDisplay)
        HRESULT ( STDMETHODCALLTYPE *get_IPDisplay )( 
            IIPDVDec * This,
            /* [annotation][out] */ 
            _Out_  int *displayPix);
        
        DECLSPEC_XFGVIRT(IIPDVDec, put_IPDisplay)
        HRESULT ( STDMETHODCALLTYPE *put_IPDisplay )( 
            IIPDVDec * This,
            /* [in] */ int displayPix);
        
        END_INTERFACE
    } IIPDVDecVtbl;

    interface IIPDVDec
    {
        CONST_VTBL struct IIPDVDecVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IIPDVDec_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IIPDVDec_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IIPDVDec_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IIPDVDec_get_IPDisplay(This,displayPix)	\
    ( (This)->lpVtbl -> get_IPDisplay(This,displayPix) ) 

#define IIPDVDec_put_IPDisplay(This,displayPix)	\
    ( (This)->lpVtbl -> put_IPDisplay(This,displayPix) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IIPDVDec_INTERFACE_DEFINED__ */


#ifndef __IDVRGB219_INTERFACE_DEFINED__
#define __IDVRGB219_INTERFACE_DEFINED__

/* interface IDVRGB219 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDVRGB219;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("58473A19-2BC8-4663-8012-25F81BABDDD1")
    IDVRGB219 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRGB219( 
            /* [in] */ BOOL bState) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVRGB219Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVRGB219 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVRGB219 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVRGB219 * This);
        
        DECLSPEC_XFGVIRT(IDVRGB219, SetRGB219)
        HRESULT ( STDMETHODCALLTYPE *SetRGB219 )( 
            IDVRGB219 * This,
            /* [in] */ BOOL bState);
        
        END_INTERFACE
    } IDVRGB219Vtbl;

    interface IDVRGB219
    {
        CONST_VTBL struct IDVRGB219Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVRGB219_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVRGB219_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVRGB219_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVRGB219_SetRGB219(This,bState)	\
    ( (This)->lpVtbl -> SetRGB219(This,bState) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVRGB219_INTERFACE_DEFINED__ */


#ifndef __IDVSplitter_INTERFACE_DEFINED__
#define __IDVSplitter_INTERFACE_DEFINED__

/* interface IDVSplitter */
/* [uuid][object][local] */ 


EXTERN_C const IID IID_IDVSplitter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("92a3a302-da7c-4a1f-ba7e-1802bb5d2d02")
    IDVSplitter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DiscardAlternateVideoFrames( 
            /* [in] */ int nDiscard) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDVSplitterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDVSplitter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDVSplitter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDVSplitter * This);
        
        DECLSPEC_XFGVIRT(IDVSplitter, DiscardAlternateVideoFrames)
        HRESULT ( STDMETHODCALLTYPE *DiscardAlternateVideoFrames )( 
            IDVSplitter * This,
            /* [in] */ int nDiscard);
        
        END_INTERFACE
    } IDVSplitterVtbl;

    interface IDVSplitter
    {
        CONST_VTBL struct IDVSplitterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDVSplitter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDVSplitter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDVSplitter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDVSplitter_DiscardAlternateVideoFrames(This,nDiscard)	\
    ( (This)->lpVtbl -> DiscardAlternateVideoFrames(This,nDiscard) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDVSplitter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0093 */
/* [local] */ 


enum _AM_AUDIO_RENDERER_STAT_PARAM
    {
        AM_AUDREND_STAT_PARAM_BREAK_COUNT	= 1,
        AM_AUDREND_STAT_PARAM_SLAVE_MODE	= ( AM_AUDREND_STAT_PARAM_BREAK_COUNT + 1 ) ,
        AM_AUDREND_STAT_PARAM_SILENCE_DUR	= ( AM_AUDREND_STAT_PARAM_SLAVE_MODE + 1 ) ,
        AM_AUDREND_STAT_PARAM_LAST_BUFFER_DUR	= ( AM_AUDREND_STAT_PARAM_SILENCE_DUR + 1 ) ,
        AM_AUDREND_STAT_PARAM_DISCONTINUITIES	= ( AM_AUDREND_STAT_PARAM_LAST_BUFFER_DUR + 1 ) ,
        AM_AUDREND_STAT_PARAM_SLAVE_RATE	= ( AM_AUDREND_STAT_PARAM_DISCONTINUITIES + 1 ) ,
        AM_AUDREND_STAT_PARAM_SLAVE_DROPWRITE_DUR	= ( AM_AUDREND_STAT_PARAM_SLAVE_RATE + 1 ) ,
        AM_AUDREND_STAT_PARAM_SLAVE_HIGHLOWERROR	= ( AM_AUDREND_STAT_PARAM_SLAVE_DROPWRITE_DUR + 1 ) ,
        AM_AUDREND_STAT_PARAM_SLAVE_LASTHIGHLOWERROR	= ( AM_AUDREND_STAT_PARAM_SLAVE_HIGHLOWERROR + 1 ) ,
        AM_AUDREND_STAT_PARAM_SLAVE_ACCUMERROR	= ( AM_AUDREND_STAT_PARAM_SLAVE_LASTHIGHLOWERROR + 1 ) ,
        AM_AUDREND_STAT_PARAM_BUFFERFULLNESS	= ( AM_AUDREND_STAT_PARAM_SLAVE_ACCUMERROR + 1 ) ,
        AM_AUDREND_STAT_PARAM_JITTER	= ( AM_AUDREND_STAT_PARAM_BUFFERFULLNESS + 1 ) 
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0093_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0093_v0_0_s_ifspec;

#ifndef __IAMAudioRendererStats_INTERFACE_DEFINED__
#define __IAMAudioRendererStats_INTERFACE_DEFINED__

/* interface IAMAudioRendererStats */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMAudioRendererStats;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("22320CB2-D41A-11d2-BF7C-D7CB9DF0BF93")
    IAMAudioRendererStats : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetStatParam( 
            /* [in] */ DWORD dwParam,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwParam1,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwParam2) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMAudioRendererStatsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMAudioRendererStats * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMAudioRendererStats * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMAudioRendererStats * This);
        
        DECLSPEC_XFGVIRT(IAMAudioRendererStats, GetStatParam)
        HRESULT ( STDMETHODCALLTYPE *GetStatParam )( 
            IAMAudioRendererStats * This,
            /* [in] */ DWORD dwParam,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwParam1,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwParam2);
        
        END_INTERFACE
    } IAMAudioRendererStatsVtbl;

    interface IAMAudioRendererStats
    {
        CONST_VTBL struct IAMAudioRendererStatsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMAudioRendererStats_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMAudioRendererStats_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMAudioRendererStats_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMAudioRendererStats_GetStatParam(This,dwParam,pdwParam1,pdwParam2)	\
    ( (This)->lpVtbl -> GetStatParam(This,dwParam,pdwParam1,pdwParam2) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMAudioRendererStats_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0095 */
/* [local] */ 


enum _AM_INTF_SEARCH_FLAGS
    {
        AM_INTF_SEARCH_INPUT_PIN	= 0x1,
        AM_INTF_SEARCH_OUTPUT_PIN	= 0x2,
        AM_INTF_SEARCH_FILTER	= 0x4
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0095_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0095_v0_0_s_ifspec;

#ifndef __IAMGraphStreams_INTERFACE_DEFINED__
#define __IAMGraphStreams_INTERFACE_DEFINED__

/* interface IAMGraphStreams */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMGraphStreams;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("632105FA-072E-11d3-8AF9-00C04FB6BD3D")
    IAMGraphStreams : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE FindUpstreamInterface( 
            /* [in] */ IPin *pPin,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  void **ppvInterface,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SyncUsingStreamOffset( 
            /* [in] */ BOOL bUseStreamOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMaxGraphLatency( 
            /* [in] */ REFERENCE_TIME rtMaxGraphLatency) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMGraphStreamsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMGraphStreams * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMGraphStreams * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMGraphStreams * This);
        
        DECLSPEC_XFGVIRT(IAMGraphStreams, FindUpstreamInterface)
        HRESULT ( STDMETHODCALLTYPE *FindUpstreamInterface )( 
            IAMGraphStreams * This,
            /* [in] */ IPin *pPin,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _Out_  void **ppvInterface,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IAMGraphStreams, SyncUsingStreamOffset)
        HRESULT ( STDMETHODCALLTYPE *SyncUsingStreamOffset )( 
            IAMGraphStreams * This,
            /* [in] */ BOOL bUseStreamOffset);
        
        DECLSPEC_XFGVIRT(IAMGraphStreams, SetMaxGraphLatency)
        HRESULT ( STDMETHODCALLTYPE *SetMaxGraphLatency )( 
            IAMGraphStreams * This,
            /* [in] */ REFERENCE_TIME rtMaxGraphLatency);
        
        END_INTERFACE
    } IAMGraphStreamsVtbl;

    interface IAMGraphStreams
    {
        CONST_VTBL struct IAMGraphStreamsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMGraphStreams_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMGraphStreams_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMGraphStreams_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMGraphStreams_FindUpstreamInterface(This,pPin,riid,ppvInterface,dwFlags)	\
    ( (This)->lpVtbl -> FindUpstreamInterface(This,pPin,riid,ppvInterface,dwFlags) ) 

#define IAMGraphStreams_SyncUsingStreamOffset(This,bUseStreamOffset)	\
    ( (This)->lpVtbl -> SyncUsingStreamOffset(This,bUseStreamOffset) ) 

#define IAMGraphStreams_SetMaxGraphLatency(This,rtMaxGraphLatency)	\
    ( (This)->lpVtbl -> SetMaxGraphLatency(This,rtMaxGraphLatency) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMGraphStreams_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0096 */
/* [local] */ 


enum AMOVERLAYFX
    {
        AMOVERFX_NOFX	= 0,
        AMOVERFX_MIRRORLEFTRIGHT	= 0x2,
        AMOVERFX_MIRRORUPDOWN	= 0x4,
        AMOVERFX_DEINTERLACE	= 0x8
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0096_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0096_v0_0_s_ifspec;

#ifndef __IAMOverlayFX_INTERFACE_DEFINED__
#define __IAMOverlayFX_INTERFACE_DEFINED__

/* interface IAMOverlayFX */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMOverlayFX;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("62fae250-7e65-4460-bfc9-6398b322073c")
    IAMOverlayFX : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryOverlayFXCaps( 
            /* [annotation][out] */ 
            _Out_  DWORD *lpdwOverlayFXCaps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOverlayFX( 
            /* [in] */ DWORD dwOverlayFX) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOverlayFX( 
            /* [annotation][out] */ 
            _Out_  DWORD *lpdwOverlayFX) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMOverlayFXVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMOverlayFX * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMOverlayFX * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMOverlayFX * This);
        
        DECLSPEC_XFGVIRT(IAMOverlayFX, QueryOverlayFXCaps)
        HRESULT ( STDMETHODCALLTYPE *QueryOverlayFXCaps )( 
            IAMOverlayFX * This,
            /* [annotation][out] */ 
            _Out_  DWORD *lpdwOverlayFXCaps);
        
        DECLSPEC_XFGVIRT(IAMOverlayFX, SetOverlayFX)
        HRESULT ( STDMETHODCALLTYPE *SetOverlayFX )( 
            IAMOverlayFX * This,
            /* [in] */ DWORD dwOverlayFX);
        
        DECLSPEC_XFGVIRT(IAMOverlayFX, GetOverlayFX)
        HRESULT ( STDMETHODCALLTYPE *GetOverlayFX )( 
            IAMOverlayFX * This,
            /* [annotation][out] */ 
            _Out_  DWORD *lpdwOverlayFX);
        
        END_INTERFACE
    } IAMOverlayFXVtbl;

    interface IAMOverlayFX
    {
        CONST_VTBL struct IAMOverlayFXVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMOverlayFX_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMOverlayFX_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMOverlayFX_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMOverlayFX_QueryOverlayFXCaps(This,lpdwOverlayFXCaps)	\
    ( (This)->lpVtbl -> QueryOverlayFXCaps(This,lpdwOverlayFXCaps) ) 

#define IAMOverlayFX_SetOverlayFX(This,dwOverlayFX)	\
    ( (This)->lpVtbl -> SetOverlayFX(This,dwOverlayFX) ) 

#define IAMOverlayFX_GetOverlayFX(This,lpdwOverlayFX)	\
    ( (This)->lpVtbl -> GetOverlayFX(This,lpdwOverlayFX) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMOverlayFX_INTERFACE_DEFINED__ */


#ifndef __IAMOpenProgress_INTERFACE_DEFINED__
#define __IAMOpenProgress_INTERFACE_DEFINED__

/* interface IAMOpenProgress */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMOpenProgress;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("8E1C39A1-DE53-11cf-AA63-0080C744528D")
    IAMOpenProgress : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE QueryProgress( 
            /* [annotation][out] */ 
            _Out_  LONGLONG *pllTotal,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pllCurrent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AbortOperation( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMOpenProgressVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMOpenProgress * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMOpenProgress * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMOpenProgress * This);
        
        DECLSPEC_XFGVIRT(IAMOpenProgress, QueryProgress)
        HRESULT ( STDMETHODCALLTYPE *QueryProgress )( 
            IAMOpenProgress * This,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pllTotal,
            /* [annotation][out] */ 
            _Out_  LONGLONG *pllCurrent);
        
        DECLSPEC_XFGVIRT(IAMOpenProgress, AbortOperation)
        HRESULT ( STDMETHODCALLTYPE *AbortOperation )( 
            IAMOpenProgress * This);
        
        END_INTERFACE
    } IAMOpenProgressVtbl;

    interface IAMOpenProgress
    {
        CONST_VTBL struct IAMOpenProgressVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMOpenProgress_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMOpenProgress_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMOpenProgress_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMOpenProgress_QueryProgress(This,pllTotal,pllCurrent)	\
    ( (This)->lpVtbl -> QueryProgress(This,pllTotal,pllCurrent) ) 

#define IAMOpenProgress_AbortOperation(This)	\
    ( (This)->lpVtbl -> AbortOperation(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMOpenProgress_INTERFACE_DEFINED__ */


#ifndef __IMpeg2Demultiplexer_INTERFACE_DEFINED__
#define __IMpeg2Demultiplexer_INTERFACE_DEFINED__

/* interface IMpeg2Demultiplexer */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IMpeg2Demultiplexer;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("436eee9c-264f-4242-90e1-4e330c107512")
    IMpeg2Demultiplexer : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE CreateOutputPin( 
            /* [in] */ AM_MEDIA_TYPE *pMediaType,
            /* [annotation][in] */ 
            _In_  LPWSTR pszPinName,
            /* [annotation][out] */ 
            _Out_  IPin **ppIPin) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputPinMediaType( 
            /* [annotation][in] */ 
            _In_  LPWSTR pszPinName,
            /* [annotation][in] */ 
            _In_  AM_MEDIA_TYPE *pMediaType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DeleteOutputPin( 
            /* [annotation][in] */ 
            _In_  LPWSTR pszPinName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMpeg2DemultiplexerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMpeg2Demultiplexer * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMpeg2Demultiplexer * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMpeg2Demultiplexer * This);
        
        DECLSPEC_XFGVIRT(IMpeg2Demultiplexer, CreateOutputPin)
        HRESULT ( STDMETHODCALLTYPE *CreateOutputPin )( 
            IMpeg2Demultiplexer * This,
            /* [in] */ AM_MEDIA_TYPE *pMediaType,
            /* [annotation][in] */ 
            _In_  LPWSTR pszPinName,
            /* [annotation][out] */ 
            _Out_  IPin **ppIPin);
        
        DECLSPEC_XFGVIRT(IMpeg2Demultiplexer, SetOutputPinMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetOutputPinMediaType )( 
            IMpeg2Demultiplexer * This,
            /* [annotation][in] */ 
            _In_  LPWSTR pszPinName,
            /* [annotation][in] */ 
            _In_  AM_MEDIA_TYPE *pMediaType);
        
        DECLSPEC_XFGVIRT(IMpeg2Demultiplexer, DeleteOutputPin)
        HRESULT ( STDMETHODCALLTYPE *DeleteOutputPin )( 
            IMpeg2Demultiplexer * This,
            /* [annotation][in] */ 
            _In_  LPWSTR pszPinName);
        
        END_INTERFACE
    } IMpeg2DemultiplexerVtbl;

    interface IMpeg2Demultiplexer
    {
        CONST_VTBL struct IMpeg2DemultiplexerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMpeg2Demultiplexer_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMpeg2Demultiplexer_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMpeg2Demultiplexer_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMpeg2Demultiplexer_CreateOutputPin(This,pMediaType,pszPinName,ppIPin)	\
    ( (This)->lpVtbl -> CreateOutputPin(This,pMediaType,pszPinName,ppIPin) ) 

#define IMpeg2Demultiplexer_SetOutputPinMediaType(This,pszPinName,pMediaType)	\
    ( (This)->lpVtbl -> SetOutputPinMediaType(This,pszPinName,pMediaType) ) 

#define IMpeg2Demultiplexer_DeleteOutputPin(This,pszPinName)	\
    ( (This)->lpVtbl -> DeleteOutputPin(This,pszPinName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMpeg2Demultiplexer_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0099 */
/* [local] */ 

#define MPEG2_PROGRAM_STREAM_MAP                 0x00000000
#define MPEG2_PROGRAM_ELEMENTARY_STREAM          0x00000001
#define MPEG2_PROGRAM_DIRECTORY_PES_PACKET       0x00000002
#define MPEG2_PROGRAM_PACK_HEADER                0x00000003
#define MPEG2_PROGRAM_PES_STREAM                 0x00000004
#define MPEG2_PROGRAM_SYSTEM_HEADER              0x00000005
#define SUBSTREAM_FILTER_VAL_NONE                0x10000000
typedef struct STREAM_ID_MAP
    {
    ULONG stream_id;
    DWORD dwMediaSampleContent;
    ULONG ulSubstreamFilterValue;
    int iDataOffset;
    } 	STREAM_ID_MAP;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0099_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0099_v0_0_s_ifspec;

#ifndef __IEnumStreamIdMap_INTERFACE_DEFINED__
#define __IEnumStreamIdMap_INTERFACE_DEFINED__

/* interface IEnumStreamIdMap */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IEnumStreamIdMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("945C1566-6202-46fc-96C7-D87F289C6534")
    IEnumStreamIdMap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Next( 
            /* [in] */ ULONG cRequest,
            /* [annotation][size_is][out][in] */ 
            _Out_writes_to_(cRequest, *pcReceived)  STREAM_ID_MAP *pStreamIdMap,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcReceived) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Skip( 
            /* [in] */ ULONG cRecords) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reset( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Clone( 
            /* [annotation][out] */ 
            _Out_  IEnumStreamIdMap **ppIEnumStreamIdMap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEnumStreamIdMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEnumStreamIdMap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEnumStreamIdMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEnumStreamIdMap * This);
        
        DECLSPEC_XFGVIRT(IEnumStreamIdMap, Next)
        HRESULT ( STDMETHODCALLTYPE *Next )( 
            IEnumStreamIdMap * This,
            /* [in] */ ULONG cRequest,
            /* [annotation][size_is][out][in] */ 
            _Out_writes_to_(cRequest, *pcReceived)  STREAM_ID_MAP *pStreamIdMap,
            /* [annotation][out] */ 
            _Out_opt_  ULONG *pcReceived);
        
        DECLSPEC_XFGVIRT(IEnumStreamIdMap, Skip)
        HRESULT ( STDMETHODCALLTYPE *Skip )( 
            IEnumStreamIdMap * This,
            /* [in] */ ULONG cRecords);
        
        DECLSPEC_XFGVIRT(IEnumStreamIdMap, Reset)
        HRESULT ( STDMETHODCALLTYPE *Reset )( 
            IEnumStreamIdMap * This);
        
        DECLSPEC_XFGVIRT(IEnumStreamIdMap, Clone)
        HRESULT ( STDMETHODCALLTYPE *Clone )( 
            IEnumStreamIdMap * This,
            /* [annotation][out] */ 
            _Out_  IEnumStreamIdMap **ppIEnumStreamIdMap);
        
        END_INTERFACE
    } IEnumStreamIdMapVtbl;

    interface IEnumStreamIdMap
    {
        CONST_VTBL struct IEnumStreamIdMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEnumStreamIdMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEnumStreamIdMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEnumStreamIdMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEnumStreamIdMap_Next(This,cRequest,pStreamIdMap,pcReceived)	\
    ( (This)->lpVtbl -> Next(This,cRequest,pStreamIdMap,pcReceived) ) 

#define IEnumStreamIdMap_Skip(This,cRecords)	\
    ( (This)->lpVtbl -> Skip(This,cRecords) ) 

#define IEnumStreamIdMap_Reset(This)	\
    ( (This)->lpVtbl -> Reset(This) ) 

#define IEnumStreamIdMap_Clone(This,ppIEnumStreamIdMap)	\
    ( (This)->lpVtbl -> Clone(This,ppIEnumStreamIdMap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEnumStreamIdMap_INTERFACE_DEFINED__ */


#ifndef __IMPEG2StreamIdMap_INTERFACE_DEFINED__
#define __IMPEG2StreamIdMap_INTERFACE_DEFINED__

/* interface IMPEG2StreamIdMap */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IMPEG2StreamIdMap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("D0E04C47-25B8-4369-925A-362A01D95444")
    IMPEG2StreamIdMap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE MapStreamId( 
            /* [in] */ ULONG ulStreamId,
            /* [in] */ DWORD MediaSampleContent,
            /* [in] */ ULONG ulSubstreamFilterValue,
            /* [in] */ int iDataOffset) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnmapStreamId( 
            /* [in] */ ULONG culStreamId,
            /* [annotation][in] */ 
            _In_reads_(culStreamId)  ULONG *pulStreamId) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumStreamIdMap( 
            /* [annotation][out] */ 
            _Out_  IEnumStreamIdMap **ppIEnumStreamIdMap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IMPEG2StreamIdMapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IMPEG2StreamIdMap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IMPEG2StreamIdMap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IMPEG2StreamIdMap * This);
        
        DECLSPEC_XFGVIRT(IMPEG2StreamIdMap, MapStreamId)
        HRESULT ( STDMETHODCALLTYPE *MapStreamId )( 
            IMPEG2StreamIdMap * This,
            /* [in] */ ULONG ulStreamId,
            /* [in] */ DWORD MediaSampleContent,
            /* [in] */ ULONG ulSubstreamFilterValue,
            /* [in] */ int iDataOffset);
        
        DECLSPEC_XFGVIRT(IMPEG2StreamIdMap, UnmapStreamId)
        HRESULT ( STDMETHODCALLTYPE *UnmapStreamId )( 
            IMPEG2StreamIdMap * This,
            /* [in] */ ULONG culStreamId,
            /* [annotation][in] */ 
            _In_reads_(culStreamId)  ULONG *pulStreamId);
        
        DECLSPEC_XFGVIRT(IMPEG2StreamIdMap, EnumStreamIdMap)
        HRESULT ( STDMETHODCALLTYPE *EnumStreamIdMap )( 
            IMPEG2StreamIdMap * This,
            /* [annotation][out] */ 
            _Out_  IEnumStreamIdMap **ppIEnumStreamIdMap);
        
        END_INTERFACE
    } IMPEG2StreamIdMapVtbl;

    interface IMPEG2StreamIdMap
    {
        CONST_VTBL struct IMPEG2StreamIdMapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IMPEG2StreamIdMap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IMPEG2StreamIdMap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IMPEG2StreamIdMap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IMPEG2StreamIdMap_MapStreamId(This,ulStreamId,MediaSampleContent,ulSubstreamFilterValue,iDataOffset)	\
    ( (This)->lpVtbl -> MapStreamId(This,ulStreamId,MediaSampleContent,ulSubstreamFilterValue,iDataOffset) ) 

#define IMPEG2StreamIdMap_UnmapStreamId(This,culStreamId,pulStreamId)	\
    ( (This)->lpVtbl -> UnmapStreamId(This,culStreamId,pulStreamId) ) 

#define IMPEG2StreamIdMap_EnumStreamIdMap(This,ppIEnumStreamIdMap)	\
    ( (This)->lpVtbl -> EnumStreamIdMap(This,ppIEnumStreamIdMap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IMPEG2StreamIdMap_INTERFACE_DEFINED__ */


#ifndef __IRegisterServiceProvider_INTERFACE_DEFINED__
#define __IRegisterServiceProvider_INTERFACE_DEFINED__

/* interface IRegisterServiceProvider */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IRegisterServiceProvider;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7B3A2F01-0751-48DD-B556-004785171C54")
    IRegisterServiceProvider : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE RegisterService( 
            /* [in] */ REFGUID guidService,
            /* [in] */ IUnknown *pUnkObject) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IRegisterServiceProviderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IRegisterServiceProvider * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IRegisterServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IRegisterServiceProvider * This);
        
        DECLSPEC_XFGVIRT(IRegisterServiceProvider, RegisterService)
        HRESULT ( STDMETHODCALLTYPE *RegisterService )( 
            IRegisterServiceProvider * This,
            /* [in] */ REFGUID guidService,
            /* [in] */ IUnknown *pUnkObject);
        
        END_INTERFACE
    } IRegisterServiceProviderVtbl;

    interface IRegisterServiceProvider
    {
        CONST_VTBL struct IRegisterServiceProviderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IRegisterServiceProvider_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IRegisterServiceProvider_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IRegisterServiceProvider_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IRegisterServiceProvider_RegisterService(This,guidService,pUnkObject)	\
    ( (This)->lpVtbl -> RegisterService(This,guidService,pUnkObject) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IRegisterServiceProvider_INTERFACE_DEFINED__ */


#ifndef __IAMClockSlave_INTERFACE_DEFINED__
#define __IAMClockSlave_INTERFACE_DEFINED__

/* interface IAMClockSlave */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IAMClockSlave;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9FD52741-176D-4b36-8F51-CA8F933223BE")
    IAMClockSlave : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetErrorTolerance( 
            /* [in] */ DWORD dwTolerance) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetErrorTolerance( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTolerance) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMClockSlaveVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMClockSlave * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMClockSlave * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMClockSlave * This);
        
        DECLSPEC_XFGVIRT(IAMClockSlave, SetErrorTolerance)
        HRESULT ( STDMETHODCALLTYPE *SetErrorTolerance )( 
            IAMClockSlave * This,
            /* [in] */ DWORD dwTolerance);
        
        DECLSPEC_XFGVIRT(IAMClockSlave, GetErrorTolerance)
        HRESULT ( STDMETHODCALLTYPE *GetErrorTolerance )( 
            IAMClockSlave * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwTolerance);
        
        END_INTERFACE
    } IAMClockSlaveVtbl;

    interface IAMClockSlave
    {
        CONST_VTBL struct IAMClockSlaveVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMClockSlave_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMClockSlave_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMClockSlave_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMClockSlave_SetErrorTolerance(This,dwTolerance)	\
    ( (This)->lpVtbl -> SetErrorTolerance(This,dwTolerance) ) 

#define IAMClockSlave_GetErrorTolerance(This,pdwTolerance)	\
    ( (This)->lpVtbl -> GetErrorTolerance(This,pdwTolerance) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMClockSlave_INTERFACE_DEFINED__ */


#ifndef __IAMGraphBuilderCallback_INTERFACE_DEFINED__
#define __IAMGraphBuilderCallback_INTERFACE_DEFINED__

/* interface IAMGraphBuilderCallback */
/* [unique][local][uuid][object] */ 


EXTERN_C const IID IID_IAMGraphBuilderCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4995f511-9ddb-4f12-bd3b-f04611807b79")
    IAMGraphBuilderCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SelectedFilter( 
            /* [in] */ IMoniker *pMon) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatedFilter( 
            /* [in] */ IBaseFilter *pFil) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMGraphBuilderCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMGraphBuilderCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMGraphBuilderCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMGraphBuilderCallback * This);
        
        DECLSPEC_XFGVIRT(IAMGraphBuilderCallback, SelectedFilter)
        HRESULT ( STDMETHODCALLTYPE *SelectedFilter )( 
            IAMGraphBuilderCallback * This,
            /* [in] */ IMoniker *pMon);
        
        DECLSPEC_XFGVIRT(IAMGraphBuilderCallback, CreatedFilter)
        HRESULT ( STDMETHODCALLTYPE *CreatedFilter )( 
            IAMGraphBuilderCallback * This,
            /* [in] */ IBaseFilter *pFil);
        
        END_INTERFACE
    } IAMGraphBuilderCallbackVtbl;

    interface IAMGraphBuilderCallback
    {
        CONST_VTBL struct IAMGraphBuilderCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMGraphBuilderCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMGraphBuilderCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMGraphBuilderCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMGraphBuilderCallback_SelectedFilter(This,pMon)	\
    ( (This)->lpVtbl -> SelectedFilter(This,pMon) ) 

#define IAMGraphBuilderCallback_CreatedFilter(This,pFil)	\
    ( (This)->lpVtbl -> CreatedFilter(This,pFil) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMGraphBuilderCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0104 */
/* [local] */ 

#ifdef __cplusplus
#ifndef _IAMFilterGraphCallback_
#define _IAMFilterGraphCallback_
// Note: Because this interface was not defined as a proper interface it is
//       supported under C++ only. Methods aren't stdcall.
EXTERN_GUID(IID_IAMFilterGraphCallback,0x56a868fd,0x0ad4,0x11ce,0xb0,0xa3,0x0,0x20,0xaf,0x0b,0xa7,0x70);
interface IAMFilterGraphCallback : public IUnknown
{
    // S_OK means rendering complete, S_FALSE means retry now.
    virtual HRESULT UnableToRender(IPin *pPin) = 0;
 
};
#endif // _IAMFilterGraphCallback_
#endif


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0104_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0104_v0_0_s_ifspec;

#ifndef __IGetCapabilitiesKey_INTERFACE_DEFINED__
#define __IGetCapabilitiesKey_INTERFACE_DEFINED__

/* interface IGetCapabilitiesKey */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IGetCapabilitiesKey;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a8809222-07bb-48ea-951c-33158100625b")
    IGetCapabilitiesKey : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCapabilitiesKey( 
            /* [annotation][out] */ 
            _Out_  HKEY *pHKey) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGetCapabilitiesKeyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGetCapabilitiesKey * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGetCapabilitiesKey * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGetCapabilitiesKey * This);
        
        DECLSPEC_XFGVIRT(IGetCapabilitiesKey, GetCapabilitiesKey)
        HRESULT ( STDMETHODCALLTYPE *GetCapabilitiesKey )( 
            IGetCapabilitiesKey * This,
            /* [annotation][out] */ 
            _Out_  HKEY *pHKey);
        
        END_INTERFACE
    } IGetCapabilitiesKeyVtbl;

    interface IGetCapabilitiesKey
    {
        CONST_VTBL struct IGetCapabilitiesKeyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGetCapabilitiesKey_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGetCapabilitiesKey_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGetCapabilitiesKey_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGetCapabilitiesKey_GetCapabilitiesKey(This,pHKey)	\
    ( (This)->lpVtbl -> GetCapabilitiesKey(This,pHKey) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGetCapabilitiesKey_INTERFACE_DEFINED__ */


#ifndef __IEncoderAPI_INTERFACE_DEFINED__
#define __IEncoderAPI_INTERFACE_DEFINED__

/* interface IEncoderAPI */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IEncoderAPI;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("70423839-6ACC-4b23-B079-21DBF08156A5")
    IEncoderAPI : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsSupported( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAvailable( 
            /* [in] */ const GUID *Api) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterRange( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParameterValues( 
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetValue( 
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IEncoderAPIVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IEncoderAPI * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IEncoderAPI * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IEncoderAPI * This);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, IsSupported)
        HRESULT ( STDMETHODCALLTYPE *IsSupported )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, IsAvailable)
        HRESULT ( STDMETHODCALLTYPE *IsAvailable )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetParameterRange)
        HRESULT ( STDMETHODCALLTYPE *GetParameterRange )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetParameterValues)
        HRESULT ( STDMETHODCALLTYPE *GetParameterValues )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetDefaultValue)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultValue )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            IEncoderAPI * This,
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value);
        
        END_INTERFACE
    } IEncoderAPIVtbl;

    interface IEncoderAPI
    {
        CONST_VTBL struct IEncoderAPIVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IEncoderAPI_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IEncoderAPI_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IEncoderAPI_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IEncoderAPI_IsSupported(This,Api)	\
    ( (This)->lpVtbl -> IsSupported(This,Api) ) 

#define IEncoderAPI_IsAvailable(This,Api)	\
    ( (This)->lpVtbl -> IsAvailable(This,Api) ) 

#define IEncoderAPI_GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta)	\
    ( (This)->lpVtbl -> GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta) ) 

#define IEncoderAPI_GetParameterValues(This,Api,Values,ValuesCount)	\
    ( (This)->lpVtbl -> GetParameterValues(This,Api,Values,ValuesCount) ) 

#define IEncoderAPI_GetDefaultValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetDefaultValue(This,Api,Value) ) 

#define IEncoderAPI_GetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetValue(This,Api,Value) ) 

#define IEncoderAPI_SetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Api,Value) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IEncoderAPI_INTERFACE_DEFINED__ */


#ifndef __IVideoEncoder_INTERFACE_DEFINED__
#define __IVideoEncoder_INTERFACE_DEFINED__

/* interface IVideoEncoder */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IVideoEncoder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("02997C3B-8E1B-460e-9270-545E0DE9563E")
    IVideoEncoder : public IEncoderAPI
    {
    public:
    };
    
    
#else 	/* C style interface */

    typedef struct IVideoEncoderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVideoEncoder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVideoEncoder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVideoEncoder * This);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, IsSupported)
        HRESULT ( STDMETHODCALLTYPE *IsSupported )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, IsAvailable)
        HRESULT ( STDMETHODCALLTYPE *IsAvailable )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetParameterRange)
        HRESULT ( STDMETHODCALLTYPE *GetParameterRange )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMin,
            /* [annotation][out] */ 
            _Out_  VARIANT *ValueMax,
            /* [annotation][out] */ 
            _Out_  VARIANT *SteppingDelta);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetParameterValues)
        HRESULT ( STDMETHODCALLTYPE *GetParameterValues )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api,
            /* [annotation][size_is][size_is][out] */ 
            _Outptr_result_buffer_all_(*ValuesCount)  VARIANT **Values,
            /* [annotation][out] */ 
            _Out_  ULONG *ValuesCount);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetDefaultValue)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultValue )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, GetValue)
        HRESULT ( STDMETHODCALLTYPE *GetValue )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api,
            /* [annotation][out] */ 
            _Out_  VARIANT *Value);
        
        DECLSPEC_XFGVIRT(IEncoderAPI, SetValue)
        HRESULT ( STDMETHODCALLTYPE *SetValue )( 
            IVideoEncoder * This,
            /* [in] */ const GUID *Api,
            /* [annotation][in] */ 
            _In_  VARIANT *Value);
        
        END_INTERFACE
    } IVideoEncoderVtbl;

    interface IVideoEncoder
    {
        CONST_VTBL struct IVideoEncoderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVideoEncoder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVideoEncoder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVideoEncoder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVideoEncoder_IsSupported(This,Api)	\
    ( (This)->lpVtbl -> IsSupported(This,Api) ) 

#define IVideoEncoder_IsAvailable(This,Api)	\
    ( (This)->lpVtbl -> IsAvailable(This,Api) ) 

#define IVideoEncoder_GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta)	\
    ( (This)->lpVtbl -> GetParameterRange(This,Api,ValueMin,ValueMax,SteppingDelta) ) 

#define IVideoEncoder_GetParameterValues(This,Api,Values,ValuesCount)	\
    ( (This)->lpVtbl -> GetParameterValues(This,Api,Values,ValuesCount) ) 

#define IVideoEncoder_GetDefaultValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetDefaultValue(This,Api,Value) ) 

#define IVideoEncoder_GetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> GetValue(This,Api,Value) ) 

#define IVideoEncoder_SetValue(This,Api,Value)	\
    ( (This)->lpVtbl -> SetValue(This,Api,Value) ) 


#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVideoEncoder_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0107 */
/* [local] */ 

#ifndef __ENCODER_API_DEFINES__
#define __ENCODER_API_DEFINES__
typedef 
enum VIDEOENCODER_BITRATE_MODE
    {
        ConstantBitRate	= 0,
        VariableBitRateAverage	= ( ConstantBitRate + 1 ) ,
        VariableBitRatePeak	= ( VariableBitRateAverage + 1 ) 
    } 	VIDEOENCODER_BITRATE_MODE;

#endif // __ENCODER_API_DEFINES__
#define AM_GETDECODERCAP_QUERY_VMR_SUPPORT   0x00000001
#define      VMR_NOTSUPPORTED                0x00000000
#define      VMR_SUPPORTED                   0x00000001
#define AM_QUERY_DECODER_VMR_SUPPORT         0x00000001
#define AM_QUERY_DECODER_DXVA_1_SUPPORT      0x00000002
#define AM_QUERY_DECODER_DVD_SUPPORT         0x00000003
#define AM_QUERY_DECODER_ATSC_SD_SUPPORT     0x00000004
#define AM_QUERY_DECODER_ATSC_HD_SUPPORT     0x00000005
#define AM_GETDECODERCAP_QUERY_VMR9_SUPPORT  0x00000006
#define AM_GETDECODERCAP_QUERY_EVR_SUPPORT   0x00000007
#define      DECODER_CAP_NOTSUPPORTED        0x00000000
#define      DECODER_CAP_SUPPORTED           0x00000001


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0107_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0107_v0_0_s_ifspec;

#ifndef __IAMDecoderCaps_INTERFACE_DEFINED__
#define __IAMDecoderCaps_INTERFACE_DEFINED__

/* interface IAMDecoderCaps */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMDecoderCaps;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c0dff467-d499-4986-972b-e1d9090fa941")
    IAMDecoderCaps : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDecoderCaps( 
            /* [in] */ DWORD dwCapIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *lpdwCap) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMDecoderCapsVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMDecoderCaps * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMDecoderCaps * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMDecoderCaps * This);
        
        DECLSPEC_XFGVIRT(IAMDecoderCaps, GetDecoderCaps)
        HRESULT ( STDMETHODCALLTYPE *GetDecoderCaps )( 
            IAMDecoderCaps * This,
            /* [in] */ DWORD dwCapIndex,
            /* [annotation][out] */ 
            _Out_  DWORD *lpdwCap);
        
        END_INTERFACE
    } IAMDecoderCapsVtbl;

    interface IAMDecoderCaps
    {
        CONST_VTBL struct IAMDecoderCapsVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMDecoderCaps_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMDecoderCaps_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMDecoderCaps_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMDecoderCaps_GetDecoderCaps(This,dwCapIndex,lpdwCap)	\
    ( (This)->lpVtbl -> GetDecoderCaps(This,dwCapIndex,lpdwCap) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMDecoderCaps_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0108 */
/* [local] */ 

typedef struct _AMCOPPSignature
    {
    BYTE Signature[ 256 ];
    } 	AMCOPPSignature;

typedef struct _AMCOPPCommand
    {
    GUID macKDI;
    GUID guidCommandID;
    DWORD dwSequence;
    DWORD cbSizeData;
    BYTE CommandData[ 4056 ];
    } 	AMCOPPCommand;

typedef struct _AMCOPPCommand *LPAMCOPPCommand;

typedef struct _AMCOPPStatusInput
    {
    GUID rApp;
    GUID guidStatusRequestID;
    DWORD dwSequence;
    DWORD cbSizeData;
    BYTE StatusData[ 4056 ];
    } 	AMCOPPStatusInput;

typedef struct _AMCOPPStatusInput *LPAMCOPPStatusInput;

typedef struct _AMCOPPStatusOutput
    {
    GUID macKDI;
    DWORD cbSizeData;
    BYTE COPPStatus[ 4076 ];
    } 	AMCOPPStatusOutput;

typedef struct _AMCOPPStatusOutput *LPAMCOPPStatusOutput;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0108_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0108_v0_0_s_ifspec;

#ifndef __IAMCertifiedOutputProtection_INTERFACE_DEFINED__
#define __IAMCertifiedOutputProtection_INTERFACE_DEFINED__

/* interface IAMCertifiedOutputProtection */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMCertifiedOutputProtection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("6feded3e-0ff1-4901-a2f1-43f7012c8515")
    IAMCertifiedOutputProtection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE KeyExchange( 
            /* [annotation][out] */ 
            _Out_  GUID *pRandom,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pdwLengthCertGH)  BYTE **VarLenCertGH,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwLengthCertGH) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SessionSequenceStart( 
            /* [in] */ AMCOPPSignature *pSig) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProtectionCommand( 
            /* [in] */ const AMCOPPCommand *cmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ProtectionStatus( 
            /* [in] */ const AMCOPPStatusInput *pStatusInput,
            /* [annotation][out] */ 
            _Out_  AMCOPPStatusOutput *pStatusOutput) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMCertifiedOutputProtectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMCertifiedOutputProtection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMCertifiedOutputProtection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMCertifiedOutputProtection * This);
        
        DECLSPEC_XFGVIRT(IAMCertifiedOutputProtection, KeyExchange)
        HRESULT ( STDMETHODCALLTYPE *KeyExchange )( 
            IAMCertifiedOutputProtection * This,
            /* [annotation][out] */ 
            _Out_  GUID *pRandom,
            /* [annotation][out] */ 
            _Outptr_result_bytebuffer_(*pdwLengthCertGH)  BYTE **VarLenCertGH,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwLengthCertGH);
        
        DECLSPEC_XFGVIRT(IAMCertifiedOutputProtection, SessionSequenceStart)
        HRESULT ( STDMETHODCALLTYPE *SessionSequenceStart )( 
            IAMCertifiedOutputProtection * This,
            /* [in] */ AMCOPPSignature *pSig);
        
        DECLSPEC_XFGVIRT(IAMCertifiedOutputProtection, ProtectionCommand)
        HRESULT ( STDMETHODCALLTYPE *ProtectionCommand )( 
            IAMCertifiedOutputProtection * This,
            /* [in] */ const AMCOPPCommand *cmd);
        
        DECLSPEC_XFGVIRT(IAMCertifiedOutputProtection, ProtectionStatus)
        HRESULT ( STDMETHODCALLTYPE *ProtectionStatus )( 
            IAMCertifiedOutputProtection * This,
            /* [in] */ const AMCOPPStatusInput *pStatusInput,
            /* [annotation][out] */ 
            _Out_  AMCOPPStatusOutput *pStatusOutput);
        
        END_INTERFACE
    } IAMCertifiedOutputProtectionVtbl;

    interface IAMCertifiedOutputProtection
    {
        CONST_VTBL struct IAMCertifiedOutputProtectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMCertifiedOutputProtection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMCertifiedOutputProtection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMCertifiedOutputProtection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMCertifiedOutputProtection_KeyExchange(This,pRandom,VarLenCertGH,pdwLengthCertGH)	\
    ( (This)->lpVtbl -> KeyExchange(This,pRandom,VarLenCertGH,pdwLengthCertGH) ) 

#define IAMCertifiedOutputProtection_SessionSequenceStart(This,pSig)	\
    ( (This)->lpVtbl -> SessionSequenceStart(This,pSig) ) 

#define IAMCertifiedOutputProtection_ProtectionCommand(This,cmd)	\
    ( (This)->lpVtbl -> ProtectionCommand(This,cmd) ) 

#define IAMCertifiedOutputProtection_ProtectionStatus(This,pStatusInput,pStatusOutput)	\
    ( (This)->lpVtbl -> ProtectionStatus(This,pStatusInput,pStatusOutput) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMCertifiedOutputProtection_INTERFACE_DEFINED__ */


#ifndef __IAMAsyncReaderTimestampScaling_INTERFACE_DEFINED__
#define __IAMAsyncReaderTimestampScaling_INTERFACE_DEFINED__

/* interface IAMAsyncReaderTimestampScaling */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMAsyncReaderTimestampScaling;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("cf7b26fc-9a00-485b-8147-3e789d5e8f67")
    IAMAsyncReaderTimestampScaling : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetTimestampMode( 
            /* [annotation] */ 
            _Out_  BOOL *pfRaw) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetTimestampMode( 
            BOOL fRaw) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMAsyncReaderTimestampScalingVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMAsyncReaderTimestampScaling * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMAsyncReaderTimestampScaling * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMAsyncReaderTimestampScaling * This);
        
        DECLSPEC_XFGVIRT(IAMAsyncReaderTimestampScaling, GetTimestampMode)
        HRESULT ( STDMETHODCALLTYPE *GetTimestampMode )( 
            IAMAsyncReaderTimestampScaling * This,
            /* [annotation] */ 
            _Out_  BOOL *pfRaw);
        
        DECLSPEC_XFGVIRT(IAMAsyncReaderTimestampScaling, SetTimestampMode)
        HRESULT ( STDMETHODCALLTYPE *SetTimestampMode )( 
            IAMAsyncReaderTimestampScaling * This,
            BOOL fRaw);
        
        END_INTERFACE
    } IAMAsyncReaderTimestampScalingVtbl;

    interface IAMAsyncReaderTimestampScaling
    {
        CONST_VTBL struct IAMAsyncReaderTimestampScalingVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMAsyncReaderTimestampScaling_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMAsyncReaderTimestampScaling_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMAsyncReaderTimestampScaling_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMAsyncReaderTimestampScaling_GetTimestampMode(This,pfRaw)	\
    ( (This)->lpVtbl -> GetTimestampMode(This,pfRaw) ) 

#define IAMAsyncReaderTimestampScaling_SetTimestampMode(This,fRaw)	\
    ( (This)->lpVtbl -> SetTimestampMode(This,fRaw) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMAsyncReaderTimestampScaling_INTERFACE_DEFINED__ */


#ifndef __IAMPluginControl_INTERFACE_DEFINED__
#define __IAMPluginControl_INTERFACE_DEFINED__

/* interface IAMPluginControl */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IAMPluginControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0e26a181-f40c-4635-8786-976284b52981")
    IAMPluginControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetPreferredClsid( 
            /* [annotation] */ 
            _In_  REFGUID subType,
            /* [annotation] */ 
            _Out_  CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPreferredClsidByIndex( 
            DWORD index,
            /* [annotation] */ 
            _Out_  GUID *subType,
            /* [annotation] */ 
            _Out_  CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPreferredClsid( 
            /* [annotation] */ 
            _In_  REFGUID subType,
            /* [annotation] */ 
            _In_opt_  const CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsDisabled( 
            REFCLSID clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDisabledByIndex( 
            DWORD index,
            /* [annotation] */ 
            _Out_  CLSID *clsid) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDisabled( 
            REFCLSID clsid,
            BOOL disabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsLegacyDisabled( 
            LPCWSTR dllName) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IAMPluginControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IAMPluginControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IAMPluginControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IAMPluginControl * This);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, GetPreferredClsid)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredClsid )( 
            IAMPluginControl * This,
            /* [annotation] */ 
            _In_  REFGUID subType,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, GetPreferredClsidByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetPreferredClsidByIndex )( 
            IAMPluginControl * This,
            DWORD index,
            /* [annotation] */ 
            _Out_  GUID *subType,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, SetPreferredClsid)
        HRESULT ( STDMETHODCALLTYPE *SetPreferredClsid )( 
            IAMPluginControl * This,
            /* [annotation] */ 
            _In_  REFGUID subType,
            /* [annotation] */ 
            _In_opt_  const CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, IsDisabled)
        HRESULT ( STDMETHODCALLTYPE *IsDisabled )( 
            IAMPluginControl * This,
            REFCLSID clsid);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, GetDisabledByIndex)
        HRESULT ( STDMETHODCALLTYPE *GetDisabledByIndex )( 
            IAMPluginControl * This,
            DWORD index,
            /* [annotation] */ 
            _Out_  CLSID *clsid);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, SetDisabled)
        HRESULT ( STDMETHODCALLTYPE *SetDisabled )( 
            IAMPluginControl * This,
            REFCLSID clsid,
            BOOL disabled);
        
        DECLSPEC_XFGVIRT(IAMPluginControl, IsLegacyDisabled)
        HRESULT ( STDMETHODCALLTYPE *IsLegacyDisabled )( 
            IAMPluginControl * This,
            LPCWSTR dllName);
        
        END_INTERFACE
    } IAMPluginControlVtbl;

    interface IAMPluginControl
    {
        CONST_VTBL struct IAMPluginControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IAMPluginControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IAMPluginControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IAMPluginControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IAMPluginControl_GetPreferredClsid(This,subType,clsid)	\
    ( (This)->lpVtbl -> GetPreferredClsid(This,subType,clsid) ) 

#define IAMPluginControl_GetPreferredClsidByIndex(This,index,subType,clsid)	\
    ( (This)->lpVtbl -> GetPreferredClsidByIndex(This,index,subType,clsid) ) 

#define IAMPluginControl_SetPreferredClsid(This,subType,clsid)	\
    ( (This)->lpVtbl -> SetPreferredClsid(This,subType,clsid) ) 

#define IAMPluginControl_IsDisabled(This,clsid)	\
    ( (This)->lpVtbl -> IsDisabled(This,clsid) ) 

#define IAMPluginControl_GetDisabledByIndex(This,index,clsid)	\
    ( (This)->lpVtbl -> GetDisabledByIndex(This,index,clsid) ) 

#define IAMPluginControl_SetDisabled(This,clsid,disabled)	\
    ( (This)->lpVtbl -> SetDisabled(This,clsid,disabled) ) 

#define IAMPluginControl_IsLegacyDisabled(This,dllName)	\
    ( (This)->lpVtbl -> IsLegacyDisabled(This,dllName) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IAMPluginControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0111 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)






extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0111_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0111_v0_0_s_ifspec;

#ifndef __IPinConnection_INTERFACE_DEFINED__
#define __IPinConnection_INTERFACE_DEFINED__

/* interface IPinConnection */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IPinConnection;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("4a9a62d3-27d4-403d-91e9-89f540e55534")
    IPinConnection : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE DynamicQueryAccept( 
            /* [in] */ const AM_MEDIA_TYPE *pmt) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyEndOfStream( 
            /* [in] */ HANDLE hNotifyEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsEndPin( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DynamicDisconnect( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPinConnectionVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPinConnection * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPinConnection * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPinConnection * This);
        
        DECLSPEC_XFGVIRT(IPinConnection, DynamicQueryAccept)
        HRESULT ( STDMETHODCALLTYPE *DynamicQueryAccept )( 
            IPinConnection * This,
            /* [in] */ const AM_MEDIA_TYPE *pmt);
        
        DECLSPEC_XFGVIRT(IPinConnection, NotifyEndOfStream)
        HRESULT ( STDMETHODCALLTYPE *NotifyEndOfStream )( 
            IPinConnection * This,
            /* [in] */ HANDLE hNotifyEvent);
        
        DECLSPEC_XFGVIRT(IPinConnection, IsEndPin)
        HRESULT ( STDMETHODCALLTYPE *IsEndPin )( 
            IPinConnection * This);
        
        DECLSPEC_XFGVIRT(IPinConnection, DynamicDisconnect)
        HRESULT ( STDMETHODCALLTYPE *DynamicDisconnect )( 
            IPinConnection * This);
        
        END_INTERFACE
    } IPinConnectionVtbl;

    interface IPinConnection
    {
        CONST_VTBL struct IPinConnectionVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPinConnection_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPinConnection_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPinConnection_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPinConnection_DynamicQueryAccept(This,pmt)	\
    ( (This)->lpVtbl -> DynamicQueryAccept(This,pmt) ) 

#define IPinConnection_NotifyEndOfStream(This,hNotifyEvent)	\
    ( (This)->lpVtbl -> NotifyEndOfStream(This,hNotifyEvent) ) 

#define IPinConnection_IsEndPin(This)	\
    ( (This)->lpVtbl -> IsEndPin(This) ) 

#define IPinConnection_DynamicDisconnect(This)	\
    ( (This)->lpVtbl -> DynamicDisconnect(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPinConnection_INTERFACE_DEFINED__ */


#ifndef __IPinFlowControl_INTERFACE_DEFINED__
#define __IPinFlowControl_INTERFACE_DEFINED__

/* interface IPinFlowControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IPinFlowControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("c56e9858-dbf3-4f6b-8119-384af2060deb")
    IPinFlowControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Block( 
            /* [in] */ DWORD dwBlockFlags,
            /* [in] */ HANDLE hEvent) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IPinFlowControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IPinFlowControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IPinFlowControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IPinFlowControl * This);
        
        DECLSPEC_XFGVIRT(IPinFlowControl, Block)
        HRESULT ( STDMETHODCALLTYPE *Block )( 
            IPinFlowControl * This,
            /* [in] */ DWORD dwBlockFlags,
            /* [in] */ HANDLE hEvent);
        
        END_INTERFACE
    } IPinFlowControlVtbl;

    interface IPinFlowControl
    {
        CONST_VTBL struct IPinFlowControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IPinFlowControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IPinFlowControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IPinFlowControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IPinFlowControl_Block(This,dwBlockFlags,hEvent)	\
    ( (This)->lpVtbl -> Block(This,dwBlockFlags,hEvent) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IPinFlowControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0113 */
/* [local] */ 


enum _AM_PIN_FLOW_CONTROL_BLOCK_FLAGS
    {
        AM_PIN_FLOW_CONTROL_BLOCK	= 0x1
    } ;
typedef 
enum _AM_GRAPH_CONFIG_RECONNECT_FLAGS
    {
        AM_GRAPH_CONFIG_RECONNECT_DIRECTCONNECT	= 0x1,
        AM_GRAPH_CONFIG_RECONNECT_CACHE_REMOVED_FILTERS	= 0x2,
        AM_GRAPH_CONFIG_RECONNECT_USE_ONLY_CACHED_FILTERS	= 0x4
    } 	AM_GRAPH_CONFIG_RECONNECT_FLAGS;


enum _REM_FILTER_FLAGS
    {
        REMFILTERF_LEAVECONNECTED	= 0x1
    } ;
typedef 
enum _AM_FILTER_FLAGS
    {
        AM_FILTER_FLAGS_REMOVABLE	= 0x1
    } 	AM_FILTER_FLAGS;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0113_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0113_v0_0_s_ifspec;

#ifndef __IGraphConfig_INTERFACE_DEFINED__
#define __IGraphConfig_INTERFACE_DEFINED__

/* interface IGraphConfig */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IGraphConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("03A1EB8E-32BF-4245-8502-114D08A9CB88")
    IGraphConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reconnect( 
            /* [in] */ IPin *pOutputPin,
            /* [in] */ IPin *pInputPin,
            /* [in] */ const AM_MEDIA_TYPE *pmtFirstConnection,
            /* [in] */ IBaseFilter *pUsingFilter,
            /* [in] */ HANDLE hAbortEvent,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Reconfigure( 
            /* [in] */ IGraphConfigCallback *pCallback,
            /* [in] */ PVOID pvContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hAbortEvent) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AddFilterToCache( 
            /* [in] */ IBaseFilter *pFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE EnumCacheFilter( 
            /* [out] */ IEnumFilters **pEnum) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveFilterFromCache( 
            /* [in] */ IBaseFilter *pFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStartTime( 
            /* [out] */ REFERENCE_TIME *prtStart) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PushThroughData( 
            /* [in] */ IPin *pOutputPin,
            /* [in] */ IPinConnection *pConnection,
            /* [in] */ HANDLE hEventAbort) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetFilterFlags( 
            /* [in] */ IBaseFilter *pFilter,
            /* [in] */ DWORD dwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetFilterFlags( 
            /* [in] */ IBaseFilter *pFilter,
            /* [out] */ DWORD *pdwFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveFilterEx( 
            /* [in] */ IBaseFilter *pFilter,
            DWORD Flags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGraphConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGraphConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGraphConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGraphConfig * This);
        
        DECLSPEC_XFGVIRT(IGraphConfig, Reconnect)
        HRESULT ( STDMETHODCALLTYPE *Reconnect )( 
            IGraphConfig * This,
            /* [in] */ IPin *pOutputPin,
            /* [in] */ IPin *pInputPin,
            /* [in] */ const AM_MEDIA_TYPE *pmtFirstConnection,
            /* [in] */ IBaseFilter *pUsingFilter,
            /* [in] */ HANDLE hAbortEvent,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IGraphConfig, Reconfigure)
        HRESULT ( STDMETHODCALLTYPE *Reconfigure )( 
            IGraphConfig * This,
            /* [in] */ IGraphConfigCallback *pCallback,
            /* [in] */ PVOID pvContext,
            /* [in] */ DWORD dwFlags,
            /* [in] */ HANDLE hAbortEvent);
        
        DECLSPEC_XFGVIRT(IGraphConfig, AddFilterToCache)
        HRESULT ( STDMETHODCALLTYPE *AddFilterToCache )( 
            IGraphConfig * This,
            /* [in] */ IBaseFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IGraphConfig, EnumCacheFilter)
        HRESULT ( STDMETHODCALLTYPE *EnumCacheFilter )( 
            IGraphConfig * This,
            /* [out] */ IEnumFilters **pEnum);
        
        DECLSPEC_XFGVIRT(IGraphConfig, RemoveFilterFromCache)
        HRESULT ( STDMETHODCALLTYPE *RemoveFilterFromCache )( 
            IGraphConfig * This,
            /* [in] */ IBaseFilter *pFilter);
        
        DECLSPEC_XFGVIRT(IGraphConfig, GetStartTime)
        HRESULT ( STDMETHODCALLTYPE *GetStartTime )( 
            IGraphConfig * This,
            /* [out] */ REFERENCE_TIME *prtStart);
        
        DECLSPEC_XFGVIRT(IGraphConfig, PushThroughData)
        HRESULT ( STDMETHODCALLTYPE *PushThroughData )( 
            IGraphConfig * This,
            /* [in] */ IPin *pOutputPin,
            /* [in] */ IPinConnection *pConnection,
            /* [in] */ HANDLE hEventAbort);
        
        DECLSPEC_XFGVIRT(IGraphConfig, SetFilterFlags)
        HRESULT ( STDMETHODCALLTYPE *SetFilterFlags )( 
            IGraphConfig * This,
            /* [in] */ IBaseFilter *pFilter,
            /* [in] */ DWORD dwFlags);
        
        DECLSPEC_XFGVIRT(IGraphConfig, GetFilterFlags)
        HRESULT ( STDMETHODCALLTYPE *GetFilterFlags )( 
            IGraphConfig * This,
            /* [in] */ IBaseFilter *pFilter,
            /* [out] */ DWORD *pdwFlags);
        
        DECLSPEC_XFGVIRT(IGraphConfig, RemoveFilterEx)
        HRESULT ( STDMETHODCALLTYPE *RemoveFilterEx )( 
            IGraphConfig * This,
            /* [in] */ IBaseFilter *pFilter,
            DWORD Flags);
        
        END_INTERFACE
    } IGraphConfigVtbl;

    interface IGraphConfig
    {
        CONST_VTBL struct IGraphConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGraphConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGraphConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGraphConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGraphConfig_Reconnect(This,pOutputPin,pInputPin,pmtFirstConnection,pUsingFilter,hAbortEvent,dwFlags)	\
    ( (This)->lpVtbl -> Reconnect(This,pOutputPin,pInputPin,pmtFirstConnection,pUsingFilter,hAbortEvent,dwFlags) ) 

#define IGraphConfig_Reconfigure(This,pCallback,pvContext,dwFlags,hAbortEvent)	\
    ( (This)->lpVtbl -> Reconfigure(This,pCallback,pvContext,dwFlags,hAbortEvent) ) 

#define IGraphConfig_AddFilterToCache(This,pFilter)	\
    ( (This)->lpVtbl -> AddFilterToCache(This,pFilter) ) 

#define IGraphConfig_EnumCacheFilter(This,pEnum)	\
    ( (This)->lpVtbl -> EnumCacheFilter(This,pEnum) ) 

#define IGraphConfig_RemoveFilterFromCache(This,pFilter)	\
    ( (This)->lpVtbl -> RemoveFilterFromCache(This,pFilter) ) 

#define IGraphConfig_GetStartTime(This,prtStart)	\
    ( (This)->lpVtbl -> GetStartTime(This,prtStart) ) 

#define IGraphConfig_PushThroughData(This,pOutputPin,pConnection,hEventAbort)	\
    ( (This)->lpVtbl -> PushThroughData(This,pOutputPin,pConnection,hEventAbort) ) 

#define IGraphConfig_SetFilterFlags(This,pFilter,dwFlags)	\
    ( (This)->lpVtbl -> SetFilterFlags(This,pFilter,dwFlags) ) 

#define IGraphConfig_GetFilterFlags(This,pFilter,pdwFlags)	\
    ( (This)->lpVtbl -> GetFilterFlags(This,pFilter,pdwFlags) ) 

#define IGraphConfig_RemoveFilterEx(This,pFilter,Flags)	\
    ( (This)->lpVtbl -> RemoveFilterEx(This,pFilter,Flags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGraphConfig_INTERFACE_DEFINED__ */


#ifndef __IGraphConfigCallback_INTERFACE_DEFINED__
#define __IGraphConfigCallback_INTERFACE_DEFINED__

/* interface IGraphConfigCallback */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IGraphConfigCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ade0fd60-d19d-11d2-abf6-00a0c905f375")
    IGraphConfigCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE Reconfigure( 
            PVOID pvContext,
            DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IGraphConfigCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IGraphConfigCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IGraphConfigCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IGraphConfigCallback * This);
        
        DECLSPEC_XFGVIRT(IGraphConfigCallback, Reconfigure)
        HRESULT ( STDMETHODCALLTYPE *Reconfigure )( 
            IGraphConfigCallback * This,
            PVOID pvContext,
            DWORD dwFlags);
        
        END_INTERFACE
    } IGraphConfigCallbackVtbl;

    interface IGraphConfigCallback
    {
        CONST_VTBL struct IGraphConfigCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IGraphConfigCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IGraphConfigCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IGraphConfigCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IGraphConfigCallback_Reconfigure(This,pvContext,dwFlags)	\
    ( (This)->lpVtbl -> Reconfigure(This,pvContext,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IGraphConfigCallback_INTERFACE_DEFINED__ */


#ifndef __IFilterChain_INTERFACE_DEFINED__
#define __IFilterChain_INTERFACE_DEFINED__

/* interface IFilterChain */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IFilterChain;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("DCFBDCF6-0DC2-45f5-9AB2-7C330EA09C29")
    IFilterChain : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartChain( 
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PauseChain( 
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopChain( 
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RemoveChain( 
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IFilterChainVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IFilterChain * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IFilterChain * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IFilterChain * This);
        
        DECLSPEC_XFGVIRT(IFilterChain, StartChain)
        HRESULT ( STDMETHODCALLTYPE *StartChain )( 
            IFilterChain * This,
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter);
        
        DECLSPEC_XFGVIRT(IFilterChain, PauseChain)
        HRESULT ( STDMETHODCALLTYPE *PauseChain )( 
            IFilterChain * This,
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter);
        
        DECLSPEC_XFGVIRT(IFilterChain, StopChain)
        HRESULT ( STDMETHODCALLTYPE *StopChain )( 
            IFilterChain * This,
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter);
        
        DECLSPEC_XFGVIRT(IFilterChain, RemoveChain)
        HRESULT ( STDMETHODCALLTYPE *RemoveChain )( 
            IFilterChain * This,
            /* [in] */ IBaseFilter *pStartFilter,
            /* [in] */ IBaseFilter *pEndFilter);
        
        END_INTERFACE
    } IFilterChainVtbl;

    interface IFilterChain
    {
        CONST_VTBL struct IFilterChainVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IFilterChain_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IFilterChain_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IFilterChain_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IFilterChain_StartChain(This,pStartFilter,pEndFilter)	\
    ( (This)->lpVtbl -> StartChain(This,pStartFilter,pEndFilter) ) 

#define IFilterChain_PauseChain(This,pStartFilter,pEndFilter)	\
    ( (This)->lpVtbl -> PauseChain(This,pStartFilter,pEndFilter) ) 

#define IFilterChain_StopChain(This,pStartFilter,pEndFilter)	\
    ( (This)->lpVtbl -> StopChain(This,pStartFilter,pEndFilter) ) 

#define IFilterChain_RemoveChain(This,pStartFilter,pEndFilter)	\
    ( (This)->lpVtbl -> RemoveChain(This,pStartFilter,pEndFilter) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IFilterChain_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0116 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#if 0
typedef DWORD *LPDIRECTDRAW7;

typedef DWORD *LPDIRECTDRAWSURFACE7;

typedef DWORD *LPDDPIXELFORMAT;

typedef DWORD *LPBITMAPINFOHEADER;

typedef struct DDCOLORKEY
    {
    DWORD dw1;
    DWORD dw2;
    } 	DDCOLORKEY;

typedef DDCOLORKEY *LPDDCOLORKEY;

#endif
#include <ddraw.h>














typedef 
enum VMRPresentationFlags
    {
        VMRSample_SyncPoint	= 0x1,
        VMRSample_Preroll	= 0x2,
        VMRSample_Discontinuity	= 0x4,
        VMRSample_TimeValid	= 0x8,
        VMRSample_SrcDstRectsValid	= 0x10
    } 	VMRPresentationFlags;

typedef struct tagVMRPRESENTATIONINFO
    {
    DWORD dwFlags;
    LPDIRECTDRAWSURFACE7 lpSurf;
    REFERENCE_TIME rtStart;
    REFERENCE_TIME rtEnd;
    SIZE szAspectRatio;
    RECT rcSrc;
    RECT rcDst;
    DWORD dwTypeSpecificFlags;
    DWORD dwInterlaceFlags;
    } 	VMRPRESENTATIONINFO;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0116_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0116_v0_0_s_ifspec;

#ifndef __IVMRImagePresenter_INTERFACE_DEFINED__
#define __IVMRImagePresenter_INTERFACE_DEFINED__

/* interface IVMRImagePresenter */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRImagePresenter;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("CE704FE7-E71E-41fb-BAA2-C4403E1182F5")
    IVMRImagePresenter : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE StartPresenting( 
            /* [in] */ DWORD_PTR dwUserID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopPresenting( 
            /* [in] */ DWORD_PTR dwUserID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PresentImage( 
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ VMRPRESENTATIONINFO *lpPresInfo) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRImagePresenterVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRImagePresenter * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRImagePresenter * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRImagePresenter * This);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenter, StartPresenting)
        HRESULT ( STDMETHODCALLTYPE *StartPresenting )( 
            IVMRImagePresenter * This,
            /* [in] */ DWORD_PTR dwUserID);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenter, StopPresenting)
        HRESULT ( STDMETHODCALLTYPE *StopPresenting )( 
            IVMRImagePresenter * This,
            /* [in] */ DWORD_PTR dwUserID);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenter, PresentImage)
        HRESULT ( STDMETHODCALLTYPE *PresentImage )( 
            IVMRImagePresenter * This,
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ VMRPRESENTATIONINFO *lpPresInfo);
        
        END_INTERFACE
    } IVMRImagePresenterVtbl;

    interface IVMRImagePresenter
    {
        CONST_VTBL struct IVMRImagePresenterVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRImagePresenter_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRImagePresenter_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRImagePresenter_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRImagePresenter_StartPresenting(This,dwUserID)	\
    ( (This)->lpVtbl -> StartPresenting(This,dwUserID) ) 

#define IVMRImagePresenter_StopPresenting(This,dwUserID)	\
    ( (This)->lpVtbl -> StopPresenting(This,dwUserID) ) 

#define IVMRImagePresenter_PresentImage(This,dwUserID,lpPresInfo)	\
    ( (This)->lpVtbl -> PresentImage(This,dwUserID,lpPresInfo) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRImagePresenter_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0117 */
/* [local] */ 

typedef 
enum VMRSurfaceAllocationFlags
    {
        AMAP_PIXELFORMAT_VALID	= 0x1,
        AMAP_3D_TARGET	= 0x2,
        AMAP_ALLOW_SYSMEM	= 0x4,
        AMAP_FORCE_SYSMEM	= 0x8,
        AMAP_DIRECTED_FLIP	= 0x10,
        AMAP_DXVA_TARGET	= 0x20
    } 	VMRSurfaceAllocationFlags;

typedef struct tagVMRALLOCATIONINFO
    {
    DWORD dwFlags;
    LPBITMAPINFOHEADER lpHdr;
    LPDDPIXELFORMAT lpPixFmt;
    SIZE szAspectRatio;
    DWORD dwMinBuffers;
    DWORD dwMaxBuffers;
    DWORD dwInterlaceFlags;
    SIZE szNativeSize;
    } 	VMRALLOCATIONINFO;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0117_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0117_v0_0_s_ifspec;

#ifndef __IVMRSurfaceAllocator_INTERFACE_DEFINED__
#define __IVMRSurfaceAllocator_INTERFACE_DEFINED__

/* interface IVMRSurfaceAllocator */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRSurfaceAllocator;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("31ce832e-4484-458b-8cca-f4d7e3db0b52")
    IVMRSurfaceAllocator : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AllocateSurface( 
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ VMRALLOCATIONINFO *lpAllocInfo,
            /* [out][in] */ DWORD *lpdwActualBuffers,
            /* [out] */ LPDIRECTDRAWSURFACE7 *lplpSurface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE FreeSurface( 
            /* [in] */ DWORD_PTR dwID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrepareSurface( 
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ LPDIRECTDRAWSURFACE7 lpSurface,
            /* [in] */ DWORD dwSurfaceFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AdviseNotify( 
            /* [in] */ IVMRSurfaceAllocatorNotify *lpIVMRSurfAllocNotify) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRSurfaceAllocatorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRSurfaceAllocator * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRSurfaceAllocator * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRSurfaceAllocator * This);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocator, AllocateSurface)
        HRESULT ( STDMETHODCALLTYPE *AllocateSurface )( 
            IVMRSurfaceAllocator * This,
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ VMRALLOCATIONINFO *lpAllocInfo,
            /* [out][in] */ DWORD *lpdwActualBuffers,
            /* [out] */ LPDIRECTDRAWSURFACE7 *lplpSurface);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocator, FreeSurface)
        HRESULT ( STDMETHODCALLTYPE *FreeSurface )( 
            IVMRSurfaceAllocator * This,
            /* [in] */ DWORD_PTR dwID);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocator, PrepareSurface)
        HRESULT ( STDMETHODCALLTYPE *PrepareSurface )( 
            IVMRSurfaceAllocator * This,
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ LPDIRECTDRAWSURFACE7 lpSurface,
            /* [in] */ DWORD dwSurfaceFlags);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocator, AdviseNotify)
        HRESULT ( STDMETHODCALLTYPE *AdviseNotify )( 
            IVMRSurfaceAllocator * This,
            /* [in] */ IVMRSurfaceAllocatorNotify *lpIVMRSurfAllocNotify);
        
        END_INTERFACE
    } IVMRSurfaceAllocatorVtbl;

    interface IVMRSurfaceAllocator
    {
        CONST_VTBL struct IVMRSurfaceAllocatorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRSurfaceAllocator_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRSurfaceAllocator_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRSurfaceAllocator_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRSurfaceAllocator_AllocateSurface(This,dwUserID,lpAllocInfo,lpdwActualBuffers,lplpSurface)	\
    ( (This)->lpVtbl -> AllocateSurface(This,dwUserID,lpAllocInfo,lpdwActualBuffers,lplpSurface) ) 

#define IVMRSurfaceAllocator_FreeSurface(This,dwID)	\
    ( (This)->lpVtbl -> FreeSurface(This,dwID) ) 

#define IVMRSurfaceAllocator_PrepareSurface(This,dwUserID,lpSurface,dwSurfaceFlags)	\
    ( (This)->lpVtbl -> PrepareSurface(This,dwUserID,lpSurface,dwSurfaceFlags) ) 

#define IVMRSurfaceAllocator_AdviseNotify(This,lpIVMRSurfAllocNotify)	\
    ( (This)->lpVtbl -> AdviseNotify(This,lpIVMRSurfAllocNotify) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRSurfaceAllocator_INTERFACE_DEFINED__ */


#ifndef __IVMRSurfaceAllocatorNotify_INTERFACE_DEFINED__
#define __IVMRSurfaceAllocatorNotify_INTERFACE_DEFINED__

/* interface IVMRSurfaceAllocatorNotify */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRSurfaceAllocatorNotify;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aada05a8-5a4e-4729-af0b-cea27aed51e2")
    IVMRSurfaceAllocatorNotify : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE AdviseSurfaceAllocator( 
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ IVMRSurfaceAllocator *lpIVRMSurfaceAllocator) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDDrawDevice( 
            /* [in] */ LPDIRECTDRAW7 lpDDrawDevice,
            /* [in] */ HMONITOR hMonitor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChangeDDrawDevice( 
            /* [in] */ LPDIRECTDRAW7 lpDDrawDevice,
            /* [in] */ HMONITOR hMonitor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RestoreDDrawSurfaces( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NotifyEvent( 
            /* [in] */ LONG EventCode,
            /* [in] */ LONG_PTR Param1,
            /* [in] */ LONG_PTR Param2) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBorderColor( 
            /* [in] */ COLORREF clrBorder) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRSurfaceAllocatorNotifyVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRSurfaceAllocatorNotify * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRSurfaceAllocatorNotify * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRSurfaceAllocatorNotify * This);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocatorNotify, AdviseSurfaceAllocator)
        HRESULT ( STDMETHODCALLTYPE *AdviseSurfaceAllocator )( 
            IVMRSurfaceAllocatorNotify * This,
            /* [in] */ DWORD_PTR dwUserID,
            /* [in] */ IVMRSurfaceAllocator *lpIVRMSurfaceAllocator);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocatorNotify, SetDDrawDevice)
        HRESULT ( STDMETHODCALLTYPE *SetDDrawDevice )( 
            IVMRSurfaceAllocatorNotify * This,
            /* [in] */ LPDIRECTDRAW7 lpDDrawDevice,
            /* [in] */ HMONITOR hMonitor);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocatorNotify, ChangeDDrawDevice)
        HRESULT ( STDMETHODCALLTYPE *ChangeDDrawDevice )( 
            IVMRSurfaceAllocatorNotify * This,
            /* [in] */ LPDIRECTDRAW7 lpDDrawDevice,
            /* [in] */ HMONITOR hMonitor);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocatorNotify, RestoreDDrawSurfaces)
        HRESULT ( STDMETHODCALLTYPE *RestoreDDrawSurfaces )( 
            IVMRSurfaceAllocatorNotify * This);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocatorNotify, NotifyEvent)
        HRESULT ( STDMETHODCALLTYPE *NotifyEvent )( 
            IVMRSurfaceAllocatorNotify * This,
            /* [in] */ LONG EventCode,
            /* [in] */ LONG_PTR Param1,
            /* [in] */ LONG_PTR Param2);
        
        DECLSPEC_XFGVIRT(IVMRSurfaceAllocatorNotify, SetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *SetBorderColor )( 
            IVMRSurfaceAllocatorNotify * This,
            /* [in] */ COLORREF clrBorder);
        
        END_INTERFACE
    } IVMRSurfaceAllocatorNotifyVtbl;

    interface IVMRSurfaceAllocatorNotify
    {
        CONST_VTBL struct IVMRSurfaceAllocatorNotifyVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRSurfaceAllocatorNotify_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRSurfaceAllocatorNotify_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRSurfaceAllocatorNotify_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRSurfaceAllocatorNotify_AdviseSurfaceAllocator(This,dwUserID,lpIVRMSurfaceAllocator)	\
    ( (This)->lpVtbl -> AdviseSurfaceAllocator(This,dwUserID,lpIVRMSurfaceAllocator) ) 

#define IVMRSurfaceAllocatorNotify_SetDDrawDevice(This,lpDDrawDevice,hMonitor)	\
    ( (This)->lpVtbl -> SetDDrawDevice(This,lpDDrawDevice,hMonitor) ) 

#define IVMRSurfaceAllocatorNotify_ChangeDDrawDevice(This,lpDDrawDevice,hMonitor)	\
    ( (This)->lpVtbl -> ChangeDDrawDevice(This,lpDDrawDevice,hMonitor) ) 

#define IVMRSurfaceAllocatorNotify_RestoreDDrawSurfaces(This)	\
    ( (This)->lpVtbl -> RestoreDDrawSurfaces(This) ) 

#define IVMRSurfaceAllocatorNotify_NotifyEvent(This,EventCode,Param1,Param2)	\
    ( (This)->lpVtbl -> NotifyEvent(This,EventCode,Param1,Param2) ) 

#define IVMRSurfaceAllocatorNotify_SetBorderColor(This,clrBorder)	\
    ( (This)->lpVtbl -> SetBorderColor(This,clrBorder) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRSurfaceAllocatorNotify_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0119 */
/* [local] */ 

typedef 
enum VMR_ASPECT_RATIO_MODE
    {
        VMR_ARMODE_NONE	= 0,
        VMR_ARMODE_LETTER_BOX	= ( VMR_ARMODE_NONE + 1 ) 
    } 	VMR_ASPECT_RATIO_MODE;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0119_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0119_v0_0_s_ifspec;

#ifndef __IVMRWindowlessControl_INTERFACE_DEFINED__
#define __IVMRWindowlessControl_INTERFACE_DEFINED__

/* interface IVMRWindowlessControl */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRWindowlessControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("0eb1088c-4dcd-46f0-878f-39dae86a51b7")
    IVMRWindowlessControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNativeVideoSize( 
            /* [out] */ LONG *lpWidth,
            /* [out] */ LONG *lpHeight,
            /* [out] */ LONG *lpARWidth,
            /* [out] */ LONG *lpARHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMinIdealVideoSize( 
            /* [out] */ LONG *lpWidth,
            /* [out] */ LONG *lpHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMaxIdealVideoSize( 
            /* [out] */ LONG *lpWidth,
            /* [out] */ LONG *lpHeight) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVideoPosition( 
            /* [in] */ const LPRECT lpSRCRect,
            /* [in] */ const LPRECT lpDSTRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoPosition( 
            /* [out] */ LPRECT lpSRCRect,
            /* [out] */ LPRECT lpDSTRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAspectRatioMode( 
            /* [out] */ DWORD *lpAspectRatioMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAspectRatioMode( 
            /* [in] */ DWORD AspectRatioMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetVideoClippingWindow( 
            /* [in] */ HWND hwnd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RepaintVideo( 
            /* [in] */ HWND hwnd,
            /* [in] */ HDC hdc) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE DisplayModeChanged( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentImage( 
            /* [out] */ BYTE **lpDib) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBorderColor( 
            /* [in] */ COLORREF Clr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBorderColor( 
            /* [out] */ COLORREF *lpClr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetColorKey( 
            /* [in] */ COLORREF Clr) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColorKey( 
            /* [out] */ COLORREF *lpClr) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRWindowlessControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRWindowlessControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRWindowlessControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRWindowlessControl * This);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetNativeVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetNativeVideoSize )( 
            IVMRWindowlessControl * This,
            /* [out] */ LONG *lpWidth,
            /* [out] */ LONG *lpHeight,
            /* [out] */ LONG *lpARWidth,
            /* [out] */ LONG *lpARHeight);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetMinIdealVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetMinIdealVideoSize )( 
            IVMRWindowlessControl * This,
            /* [out] */ LONG *lpWidth,
            /* [out] */ LONG *lpHeight);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetMaxIdealVideoSize)
        HRESULT ( STDMETHODCALLTYPE *GetMaxIdealVideoSize )( 
            IVMRWindowlessControl * This,
            /* [out] */ LONG *lpWidth,
            /* [out] */ LONG *lpHeight);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, SetVideoPosition)
        HRESULT ( STDMETHODCALLTYPE *SetVideoPosition )( 
            IVMRWindowlessControl * This,
            /* [in] */ const LPRECT lpSRCRect,
            /* [in] */ const LPRECT lpDSTRect);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetVideoPosition)
        HRESULT ( STDMETHODCALLTYPE *GetVideoPosition )( 
            IVMRWindowlessControl * This,
            /* [out] */ LPRECT lpSRCRect,
            /* [out] */ LPRECT lpDSTRect);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetAspectRatioMode)
        HRESULT ( STDMETHODCALLTYPE *GetAspectRatioMode )( 
            IVMRWindowlessControl * This,
            /* [out] */ DWORD *lpAspectRatioMode);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, SetAspectRatioMode)
        HRESULT ( STDMETHODCALLTYPE *SetAspectRatioMode )( 
            IVMRWindowlessControl * This,
            /* [in] */ DWORD AspectRatioMode);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, SetVideoClippingWindow)
        HRESULT ( STDMETHODCALLTYPE *SetVideoClippingWindow )( 
            IVMRWindowlessControl * This,
            /* [in] */ HWND hwnd);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, RepaintVideo)
        HRESULT ( STDMETHODCALLTYPE *RepaintVideo )( 
            IVMRWindowlessControl * This,
            /* [in] */ HWND hwnd,
            /* [in] */ HDC hdc);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, DisplayModeChanged)
        HRESULT ( STDMETHODCALLTYPE *DisplayModeChanged )( 
            IVMRWindowlessControl * This);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetCurrentImage)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentImage )( 
            IVMRWindowlessControl * This,
            /* [out] */ BYTE **lpDib);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, SetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *SetBorderColor )( 
            IVMRWindowlessControl * This,
            /* [in] */ COLORREF Clr);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetBorderColor)
        HRESULT ( STDMETHODCALLTYPE *GetBorderColor )( 
            IVMRWindowlessControl * This,
            /* [out] */ COLORREF *lpClr);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, SetColorKey)
        HRESULT ( STDMETHODCALLTYPE *SetColorKey )( 
            IVMRWindowlessControl * This,
            /* [in] */ COLORREF Clr);
        
        DECLSPEC_XFGVIRT(IVMRWindowlessControl, GetColorKey)
        HRESULT ( STDMETHODCALLTYPE *GetColorKey )( 
            IVMRWindowlessControl * This,
            /* [out] */ COLORREF *lpClr);
        
        END_INTERFACE
    } IVMRWindowlessControlVtbl;

    interface IVMRWindowlessControl
    {
        CONST_VTBL struct IVMRWindowlessControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRWindowlessControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRWindowlessControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRWindowlessControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRWindowlessControl_GetNativeVideoSize(This,lpWidth,lpHeight,lpARWidth,lpARHeight)	\
    ( (This)->lpVtbl -> GetNativeVideoSize(This,lpWidth,lpHeight,lpARWidth,lpARHeight) ) 

#define IVMRWindowlessControl_GetMinIdealVideoSize(This,lpWidth,lpHeight)	\
    ( (This)->lpVtbl -> GetMinIdealVideoSize(This,lpWidth,lpHeight) ) 

#define IVMRWindowlessControl_GetMaxIdealVideoSize(This,lpWidth,lpHeight)	\
    ( (This)->lpVtbl -> GetMaxIdealVideoSize(This,lpWidth,lpHeight) ) 

#define IVMRWindowlessControl_SetVideoPosition(This,lpSRCRect,lpDSTRect)	\
    ( (This)->lpVtbl -> SetVideoPosition(This,lpSRCRect,lpDSTRect) ) 

#define IVMRWindowlessControl_GetVideoPosition(This,lpSRCRect,lpDSTRect)	\
    ( (This)->lpVtbl -> GetVideoPosition(This,lpSRCRect,lpDSTRect) ) 

#define IVMRWindowlessControl_GetAspectRatioMode(This,lpAspectRatioMode)	\
    ( (This)->lpVtbl -> GetAspectRatioMode(This,lpAspectRatioMode) ) 

#define IVMRWindowlessControl_SetAspectRatioMode(This,AspectRatioMode)	\
    ( (This)->lpVtbl -> SetAspectRatioMode(This,AspectRatioMode) ) 

#define IVMRWindowlessControl_SetVideoClippingWindow(This,hwnd)	\
    ( (This)->lpVtbl -> SetVideoClippingWindow(This,hwnd) ) 

#define IVMRWindowlessControl_RepaintVideo(This,hwnd,hdc)	\
    ( (This)->lpVtbl -> RepaintVideo(This,hwnd,hdc) ) 

#define IVMRWindowlessControl_DisplayModeChanged(This)	\
    ( (This)->lpVtbl -> DisplayModeChanged(This) ) 

#define IVMRWindowlessControl_GetCurrentImage(This,lpDib)	\
    ( (This)->lpVtbl -> GetCurrentImage(This,lpDib) ) 

#define IVMRWindowlessControl_SetBorderColor(This,Clr)	\
    ( (This)->lpVtbl -> SetBorderColor(This,Clr) ) 

#define IVMRWindowlessControl_GetBorderColor(This,lpClr)	\
    ( (This)->lpVtbl -> GetBorderColor(This,lpClr) ) 

#define IVMRWindowlessControl_SetColorKey(This,Clr)	\
    ( (This)->lpVtbl -> SetColorKey(This,Clr) ) 

#define IVMRWindowlessControl_GetColorKey(This,lpClr)	\
    ( (This)->lpVtbl -> GetColorKey(This,lpClr) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRWindowlessControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0120 */
/* [local] */ 

typedef 
enum VMRMixerPrefs
    {
        MixerPref_NoDecimation	= 0x1,
        MixerPref_DecimateOutput	= 0x2,
        MixerPref_ARAdjustXorY	= 0x4,
        MixerPref_DecimationReserved	= 0x8,
        MixerPref_DecimateMask	= 0xf,
        MixerPref_BiLinearFiltering	= 0x10,
        MixerPref_PointFiltering	= 0x20,
        MixerPref_FilteringMask	= 0xf0,
        MixerPref_RenderTargetRGB	= 0x100,
        MixerPref_RenderTargetYUV	= 0x1000,
        MixerPref_RenderTargetYUV420	= 0x200,
        MixerPref_RenderTargetYUV422	= 0x400,
        MixerPref_RenderTargetYUV444	= 0x800,
        MixerPref_RenderTargetReserved	= 0xe000,
        MixerPref_RenderTargetMask	= 0xff00,
        MixerPref_DynamicSwitchToBOB	= 0x10000,
        MixerPref_DynamicDecimateBy2	= 0x20000,
        MixerPref_DynamicReserved	= 0xc0000,
        MixerPref_DynamicMask	= 0xf0000
    } 	VMRMixerPrefs;

typedef struct _NORMALIZEDRECT
    {
    float left;
    float top;
    float right;
    float bottom;
    } 	NORMALIZEDRECT;

typedef struct _NORMALIZEDRECT *PNORMALIZEDRECT;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0120_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0120_v0_0_s_ifspec;

#ifndef __IVMRMixerControl_INTERFACE_DEFINED__
#define __IVMRMixerControl_INTERFACE_DEFINED__

/* interface IVMRMixerControl */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRMixerControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1c1a17b0-bed0-415d-974b-dc6696131599")
    IVMRMixerControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAlpha( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ float Alpha) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlpha( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ float *pAlpha) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetZOrder( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ DWORD dwZ) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetZOrder( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ DWORD *pZ) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOutputRect( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ const NORMALIZEDRECT *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetOutputRect( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ NORMALIZEDRECT *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetBackgroundClr( 
            /* [in] */ COLORREF ClrBkg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetBackgroundClr( 
            /* [in] */ COLORREF *lpClrBkg) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetMixingPrefs( 
            /* [in] */ DWORD dwMixerPrefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMixingPrefs( 
            /* [out] */ DWORD *pdwMixerPrefs) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRMixerControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRMixerControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRMixerControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRMixerControl * This);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, SetAlpha)
        HRESULT ( STDMETHODCALLTYPE *SetAlpha )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ float Alpha);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, GetAlpha)
        HRESULT ( STDMETHODCALLTYPE *GetAlpha )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ float *pAlpha);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, SetZOrder)
        HRESULT ( STDMETHODCALLTYPE *SetZOrder )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ DWORD dwZ);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, GetZOrder)
        HRESULT ( STDMETHODCALLTYPE *GetZOrder )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ DWORD *pZ);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, SetOutputRect)
        HRESULT ( STDMETHODCALLTYPE *SetOutputRect )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ const NORMALIZEDRECT *pRect);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, GetOutputRect)
        HRESULT ( STDMETHODCALLTYPE *GetOutputRect )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ NORMALIZEDRECT *pRect);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, SetBackgroundClr)
        HRESULT ( STDMETHODCALLTYPE *SetBackgroundClr )( 
            IVMRMixerControl * This,
            /* [in] */ COLORREF ClrBkg);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, GetBackgroundClr)
        HRESULT ( STDMETHODCALLTYPE *GetBackgroundClr )( 
            IVMRMixerControl * This,
            /* [in] */ COLORREF *lpClrBkg);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, SetMixingPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetMixingPrefs )( 
            IVMRMixerControl * This,
            /* [in] */ DWORD dwMixerPrefs);
        
        DECLSPEC_XFGVIRT(IVMRMixerControl, GetMixingPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetMixingPrefs )( 
            IVMRMixerControl * This,
            /* [out] */ DWORD *pdwMixerPrefs);
        
        END_INTERFACE
    } IVMRMixerControlVtbl;

    interface IVMRMixerControl
    {
        CONST_VTBL struct IVMRMixerControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRMixerControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRMixerControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRMixerControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRMixerControl_SetAlpha(This,dwStreamID,Alpha)	\
    ( (This)->lpVtbl -> SetAlpha(This,dwStreamID,Alpha) ) 

#define IVMRMixerControl_GetAlpha(This,dwStreamID,pAlpha)	\
    ( (This)->lpVtbl -> GetAlpha(This,dwStreamID,pAlpha) ) 

#define IVMRMixerControl_SetZOrder(This,dwStreamID,dwZ)	\
    ( (This)->lpVtbl -> SetZOrder(This,dwStreamID,dwZ) ) 

#define IVMRMixerControl_GetZOrder(This,dwStreamID,pZ)	\
    ( (This)->lpVtbl -> GetZOrder(This,dwStreamID,pZ) ) 

#define IVMRMixerControl_SetOutputRect(This,dwStreamID,pRect)	\
    ( (This)->lpVtbl -> SetOutputRect(This,dwStreamID,pRect) ) 

#define IVMRMixerControl_GetOutputRect(This,dwStreamID,pRect)	\
    ( (This)->lpVtbl -> GetOutputRect(This,dwStreamID,pRect) ) 

#define IVMRMixerControl_SetBackgroundClr(This,ClrBkg)	\
    ( (This)->lpVtbl -> SetBackgroundClr(This,ClrBkg) ) 

#define IVMRMixerControl_GetBackgroundClr(This,lpClrBkg)	\
    ( (This)->lpVtbl -> GetBackgroundClr(This,lpClrBkg) ) 

#define IVMRMixerControl_SetMixingPrefs(This,dwMixerPrefs)	\
    ( (This)->lpVtbl -> SetMixingPrefs(This,dwMixerPrefs) ) 

#define IVMRMixerControl_GetMixingPrefs(This,pdwMixerPrefs)	\
    ( (This)->lpVtbl -> GetMixingPrefs(This,pdwMixerPrefs) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRMixerControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0121 */
/* [local] */ 

typedef struct tagVMRGUID
    {
    GUID *pGUID;
    GUID GUID;
    } 	VMRGUID;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct tagVMRMONITORINFO
    {
    VMRGUID guid;
    RECT rcMonitor;
    HMONITOR hMon;
    DWORD dwFlags;
    wchar_t szDevice[ 32 ];
    wchar_t szDescription[ 256 ];
    LARGE_INTEGER liDriverVersion;
    DWORD dwVendorId;
    DWORD dwDeviceId;
    DWORD dwSubSysId;
    DWORD dwRevision;
    } 	VMRMONITORINFO;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0121_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0121_v0_0_s_ifspec;

#ifndef __IVMRMonitorConfig_INTERFACE_DEFINED__
#define __IVMRMonitorConfig_INTERFACE_DEFINED__

/* interface IVMRMonitorConfig */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IVMRMonitorConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9cf0b1b6-fbaa-4b7f-88cf-cf1f130a0dce")
    IVMRMonitorConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetMonitor( 
            /* [in] */ const VMRGUID *pGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMonitor( 
            /* [out] */ VMRGUID *pGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDefaultMonitor( 
            /* [in] */ const VMRGUID *pGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultMonitor( 
            /* [out] */ VMRGUID *pGUID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAvailableMonitors( 
            /* [size_is][out] */ VMRMONITORINFO *pInfo,
            /* [in] */ DWORD dwMaxInfoArraySize,
            /* [out] */ DWORD *pdwNumDevices) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRMonitorConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRMonitorConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRMonitorConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRMonitorConfig * This);
        
        DECLSPEC_XFGVIRT(IVMRMonitorConfig, SetMonitor)
        HRESULT ( STDMETHODCALLTYPE *SetMonitor )( 
            IVMRMonitorConfig * This,
            /* [in] */ const VMRGUID *pGUID);
        
        DECLSPEC_XFGVIRT(IVMRMonitorConfig, GetMonitor)
        HRESULT ( STDMETHODCALLTYPE *GetMonitor )( 
            IVMRMonitorConfig * This,
            /* [out] */ VMRGUID *pGUID);
        
        DECLSPEC_XFGVIRT(IVMRMonitorConfig, SetDefaultMonitor)
        HRESULT ( STDMETHODCALLTYPE *SetDefaultMonitor )( 
            IVMRMonitorConfig * This,
            /* [in] */ const VMRGUID *pGUID);
        
        DECLSPEC_XFGVIRT(IVMRMonitorConfig, GetDefaultMonitor)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultMonitor )( 
            IVMRMonitorConfig * This,
            /* [out] */ VMRGUID *pGUID);
        
        DECLSPEC_XFGVIRT(IVMRMonitorConfig, GetAvailableMonitors)
        HRESULT ( STDMETHODCALLTYPE *GetAvailableMonitors )( 
            IVMRMonitorConfig * This,
            /* [size_is][out] */ VMRMONITORINFO *pInfo,
            /* [in] */ DWORD dwMaxInfoArraySize,
            /* [out] */ DWORD *pdwNumDevices);
        
        END_INTERFACE
    } IVMRMonitorConfigVtbl;

    interface IVMRMonitorConfig
    {
        CONST_VTBL struct IVMRMonitorConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRMonitorConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRMonitorConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRMonitorConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRMonitorConfig_SetMonitor(This,pGUID)	\
    ( (This)->lpVtbl -> SetMonitor(This,pGUID) ) 

#define IVMRMonitorConfig_GetMonitor(This,pGUID)	\
    ( (This)->lpVtbl -> GetMonitor(This,pGUID) ) 

#define IVMRMonitorConfig_SetDefaultMonitor(This,pGUID)	\
    ( (This)->lpVtbl -> SetDefaultMonitor(This,pGUID) ) 

#define IVMRMonitorConfig_GetDefaultMonitor(This,pGUID)	\
    ( (This)->lpVtbl -> GetDefaultMonitor(This,pGUID) ) 

#define IVMRMonitorConfig_GetAvailableMonitors(This,pInfo,dwMaxInfoArraySize,pdwNumDevices)	\
    ( (This)->lpVtbl -> GetAvailableMonitors(This,pInfo,dwMaxInfoArraySize,pdwNumDevices) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRMonitorConfig_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0122 */
/* [local] */ 

typedef 
enum VMRRenderPrefs
    {
        RenderPrefs_RestrictToInitialMonitor	= 0,
        RenderPrefs_ForceOffscreen	= 0x1,
        RenderPrefs_ForceOverlays	= 0x2,
        RenderPrefs_AllowOverlays	= 0,
        RenderPrefs_AllowOffscreen	= 0,
        RenderPrefs_DoNotRenderColorKeyAndBorder	= 0x8,
        RenderPrefs_Reserved	= 0x10,
        RenderPrefs_PreferAGPMemWhenMixing	= 0x20,
        RenderPrefs_Mask	= 0x3f
    } 	VMRRenderPrefs;

typedef 
enum VMRMode
    {
        VMRMode_Windowed	= 0x1,
        VMRMode_Windowless	= 0x2,
        VMRMode_Renderless	= 0x4,
        VMRMode_Mask	= 0x7
    } 	VMRMode;


enum __MIDL___MIDL_itf_strmif_0000_0122_0001
    {
        MAX_NUMBER_OF_STREAMS	= 16
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0122_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0122_v0_0_s_ifspec;

#ifndef __IVMRFilterConfig_INTERFACE_DEFINED__
#define __IVMRFilterConfig_INTERFACE_DEFINED__

/* interface IVMRFilterConfig */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IVMRFilterConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9e5530c5-7034-48b4-bb46-0b8a6efc8e36")
    IVMRFilterConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetImageCompositor( 
            /* [in] */ IVMRImageCompositor *lpVMRImgCompositor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetNumberOfStreams( 
            /* [in] */ DWORD dwMaxStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfStreams( 
            /* [out] */ DWORD *pdwMaxStreams) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRenderingPrefs( 
            /* [in] */ DWORD dwRenderFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRenderingPrefs( 
            /* [out] */ DWORD *pdwRenderFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRenderingMode( 
            /* [in] */ DWORD Mode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRenderingMode( 
            /* [out] */ DWORD *pMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRFilterConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRFilterConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRFilterConfig * This);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, SetImageCompositor)
        HRESULT ( STDMETHODCALLTYPE *SetImageCompositor )( 
            IVMRFilterConfig * This,
            /* [in] */ IVMRImageCompositor *lpVMRImgCompositor);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, SetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *SetNumberOfStreams )( 
            IVMRFilterConfig * This,
            /* [in] */ DWORD dwMaxStreams);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, GetNumberOfStreams)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfStreams )( 
            IVMRFilterConfig * This,
            /* [out] */ DWORD *pdwMaxStreams);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, SetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetRenderingPrefs )( 
            IVMRFilterConfig * This,
            /* [in] */ DWORD dwRenderFlags);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, GetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetRenderingPrefs )( 
            IVMRFilterConfig * This,
            /* [out] */ DWORD *pdwRenderFlags);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, SetRenderingMode)
        HRESULT ( STDMETHODCALLTYPE *SetRenderingMode )( 
            IVMRFilterConfig * This,
            /* [in] */ DWORD Mode);
        
        DECLSPEC_XFGVIRT(IVMRFilterConfig, GetRenderingMode)
        HRESULT ( STDMETHODCALLTYPE *GetRenderingMode )( 
            IVMRFilterConfig * This,
            /* [out] */ DWORD *pMode);
        
        END_INTERFACE
    } IVMRFilterConfigVtbl;

    interface IVMRFilterConfig
    {
        CONST_VTBL struct IVMRFilterConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRFilterConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRFilterConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRFilterConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRFilterConfig_SetImageCompositor(This,lpVMRImgCompositor)	\
    ( (This)->lpVtbl -> SetImageCompositor(This,lpVMRImgCompositor) ) 

#define IVMRFilterConfig_SetNumberOfStreams(This,dwMaxStreams)	\
    ( (This)->lpVtbl -> SetNumberOfStreams(This,dwMaxStreams) ) 

#define IVMRFilterConfig_GetNumberOfStreams(This,pdwMaxStreams)	\
    ( (This)->lpVtbl -> GetNumberOfStreams(This,pdwMaxStreams) ) 

#define IVMRFilterConfig_SetRenderingPrefs(This,dwRenderFlags)	\
    ( (This)->lpVtbl -> SetRenderingPrefs(This,dwRenderFlags) ) 

#define IVMRFilterConfig_GetRenderingPrefs(This,pdwRenderFlags)	\
    ( (This)->lpVtbl -> GetRenderingPrefs(This,pdwRenderFlags) ) 

#define IVMRFilterConfig_SetRenderingMode(This,Mode)	\
    ( (This)->lpVtbl -> SetRenderingMode(This,Mode) ) 

#define IVMRFilterConfig_GetRenderingMode(This,pMode)	\
    ( (This)->lpVtbl -> GetRenderingMode(This,pMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRFilterConfig_INTERFACE_DEFINED__ */


#ifndef __IVMRAspectRatioControl_INTERFACE_DEFINED__
#define __IVMRAspectRatioControl_INTERFACE_DEFINED__

/* interface IVMRAspectRatioControl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IVMRAspectRatioControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("ede80b5c-bad6-4623-b537-65586c9f8dfd")
    IVMRAspectRatioControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetAspectRatioMode( 
            /* [out] */ LPDWORD lpdwARMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetAspectRatioMode( 
            /* [in] */ DWORD dwARMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRAspectRatioControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRAspectRatioControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRAspectRatioControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRAspectRatioControl * This);
        
        DECLSPEC_XFGVIRT(IVMRAspectRatioControl, GetAspectRatioMode)
        HRESULT ( STDMETHODCALLTYPE *GetAspectRatioMode )( 
            IVMRAspectRatioControl * This,
            /* [out] */ LPDWORD lpdwARMode);
        
        DECLSPEC_XFGVIRT(IVMRAspectRatioControl, SetAspectRatioMode)
        HRESULT ( STDMETHODCALLTYPE *SetAspectRatioMode )( 
            IVMRAspectRatioControl * This,
            /* [in] */ DWORD dwARMode);
        
        END_INTERFACE
    } IVMRAspectRatioControlVtbl;

    interface IVMRAspectRatioControl
    {
        CONST_VTBL struct IVMRAspectRatioControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRAspectRatioControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRAspectRatioControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRAspectRatioControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRAspectRatioControl_GetAspectRatioMode(This,lpdwARMode)	\
    ( (This)->lpVtbl -> GetAspectRatioMode(This,lpdwARMode) ) 

#define IVMRAspectRatioControl_SetAspectRatioMode(This,dwARMode)	\
    ( (This)->lpVtbl -> SetAspectRatioMode(This,dwARMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRAspectRatioControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0124 */
/* [local] */ 

typedef 
enum VMRDeinterlacePrefs
    {
        DeinterlacePref_NextBest	= 0x1,
        DeinterlacePref_BOB	= 0x2,
        DeinterlacePref_Weave	= 0x4,
        DeinterlacePref_Mask	= 0x7
    } 	VMRDeinterlacePrefs;

typedef 
enum VMRDeinterlaceTech
    {
        DeinterlaceTech_Unknown	= 0,
        DeinterlaceTech_BOBLineReplicate	= 0x1,
        DeinterlaceTech_BOBVerticalStretch	= 0x2,
        DeinterlaceTech_MedianFiltering	= 0x4,
        DeinterlaceTech_EdgeFiltering	= 0x10,
        DeinterlaceTech_FieldAdaptive	= 0x20,
        DeinterlaceTech_PixelAdaptive	= 0x40,
        DeinterlaceTech_MotionVectorSteered	= 0x80
    } 	VMRDeinterlaceTech;

typedef struct _VMRFrequency
    {
    DWORD dwNumerator;
    DWORD dwDenominator;
    } 	VMRFrequency;

typedef struct _VMRVideoDesc
    {
    DWORD dwSize;
    DWORD dwSampleWidth;
    DWORD dwSampleHeight;
    BOOL SingleFieldPerSample;
    DWORD dwFourCC;
    VMRFrequency InputSampleFreq;
    VMRFrequency OutputFrameFreq;
    } 	VMRVideoDesc;

typedef struct _VMRDeinterlaceCaps
    {
    DWORD dwSize;
    DWORD dwNumPreviousOutputFrames;
    DWORD dwNumForwardRefSamples;
    DWORD dwNumBackwardRefSamples;
    VMRDeinterlaceTech DeinterlaceTechnology;
    } 	VMRDeinterlaceCaps;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0124_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0124_v0_0_s_ifspec;

#ifndef __IVMRDeinterlaceControl_INTERFACE_DEFINED__
#define __IVMRDeinterlaceControl_INTERFACE_DEFINED__

/* interface IVMRDeinterlaceControl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IVMRDeinterlaceControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("bb057577-0db8-4e6a-87a7-1a8c9a505a0f")
    IVMRDeinterlaceControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfDeinterlaceModes( 
            /* [in] */ VMRVideoDesc *lpVideoDescription,
            /* [out][in] */ LPDWORD lpdwNumDeinterlaceModes,
            /* [out] */ LPGUID lpDeinterlaceModes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeinterlaceModeCaps( 
            /* [in] */ LPGUID lpDeinterlaceMode,
            /* [in] */ VMRVideoDesc *lpVideoDescription,
            /* [out][in] */ VMRDeinterlaceCaps *lpDeinterlaceCaps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeinterlaceMode( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ LPGUID lpDeinterlaceMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDeinterlaceMode( 
            /* [in] */ DWORD dwStreamID,
            /* [in] */ LPGUID lpDeinterlaceMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeinterlacePrefs( 
            /* [out] */ LPDWORD lpdwDeinterlacePrefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDeinterlacePrefs( 
            /* [in] */ DWORD dwDeinterlacePrefs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetActualDeinterlaceMode( 
            /* [in] */ DWORD dwStreamID,
            /* [out] */ LPGUID lpDeinterlaceMode) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRDeinterlaceControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRDeinterlaceControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRDeinterlaceControl * This);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, GetNumberOfDeinterlaceModes)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfDeinterlaceModes )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ VMRVideoDesc *lpVideoDescription,
            /* [out][in] */ LPDWORD lpdwNumDeinterlaceModes,
            /* [out] */ LPGUID lpDeinterlaceModes);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, GetDeinterlaceModeCaps)
        HRESULT ( STDMETHODCALLTYPE *GetDeinterlaceModeCaps )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ LPGUID lpDeinterlaceMode,
            /* [in] */ VMRVideoDesc *lpVideoDescription,
            /* [out][in] */ VMRDeinterlaceCaps *lpDeinterlaceCaps);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, GetDeinterlaceMode)
        HRESULT ( STDMETHODCALLTYPE *GetDeinterlaceMode )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ LPGUID lpDeinterlaceMode);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, SetDeinterlaceMode)
        HRESULT ( STDMETHODCALLTYPE *SetDeinterlaceMode )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [in] */ LPGUID lpDeinterlaceMode);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, GetDeinterlacePrefs)
        HRESULT ( STDMETHODCALLTYPE *GetDeinterlacePrefs )( 
            IVMRDeinterlaceControl * This,
            /* [out] */ LPDWORD lpdwDeinterlacePrefs);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, SetDeinterlacePrefs)
        HRESULT ( STDMETHODCALLTYPE *SetDeinterlacePrefs )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ DWORD dwDeinterlacePrefs);
        
        DECLSPEC_XFGVIRT(IVMRDeinterlaceControl, GetActualDeinterlaceMode)
        HRESULT ( STDMETHODCALLTYPE *GetActualDeinterlaceMode )( 
            IVMRDeinterlaceControl * This,
            /* [in] */ DWORD dwStreamID,
            /* [out] */ LPGUID lpDeinterlaceMode);
        
        END_INTERFACE
    } IVMRDeinterlaceControlVtbl;

    interface IVMRDeinterlaceControl
    {
        CONST_VTBL struct IVMRDeinterlaceControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRDeinterlaceControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRDeinterlaceControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRDeinterlaceControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRDeinterlaceControl_GetNumberOfDeinterlaceModes(This,lpVideoDescription,lpdwNumDeinterlaceModes,lpDeinterlaceModes)	\
    ( (This)->lpVtbl -> GetNumberOfDeinterlaceModes(This,lpVideoDescription,lpdwNumDeinterlaceModes,lpDeinterlaceModes) ) 

#define IVMRDeinterlaceControl_GetDeinterlaceModeCaps(This,lpDeinterlaceMode,lpVideoDescription,lpDeinterlaceCaps)	\
    ( (This)->lpVtbl -> GetDeinterlaceModeCaps(This,lpDeinterlaceMode,lpVideoDescription,lpDeinterlaceCaps) ) 

#define IVMRDeinterlaceControl_GetDeinterlaceMode(This,dwStreamID,lpDeinterlaceMode)	\
    ( (This)->lpVtbl -> GetDeinterlaceMode(This,dwStreamID,lpDeinterlaceMode) ) 

#define IVMRDeinterlaceControl_SetDeinterlaceMode(This,dwStreamID,lpDeinterlaceMode)	\
    ( (This)->lpVtbl -> SetDeinterlaceMode(This,dwStreamID,lpDeinterlaceMode) ) 

#define IVMRDeinterlaceControl_GetDeinterlacePrefs(This,lpdwDeinterlacePrefs)	\
    ( (This)->lpVtbl -> GetDeinterlacePrefs(This,lpdwDeinterlacePrefs) ) 

#define IVMRDeinterlaceControl_SetDeinterlacePrefs(This,dwDeinterlacePrefs)	\
    ( (This)->lpVtbl -> SetDeinterlacePrefs(This,dwDeinterlacePrefs) ) 

#define IVMRDeinterlaceControl_GetActualDeinterlaceMode(This,dwStreamID,lpDeinterlaceMode)	\
    ( (This)->lpVtbl -> GetActualDeinterlaceMode(This,dwStreamID,lpDeinterlaceMode) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRDeinterlaceControl_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0125 */
/* [local] */ 

typedef struct _VMRALPHABITMAP
    {
    DWORD dwFlags;
    HDC hdc;
    LPDIRECTDRAWSURFACE7 pDDS;
    RECT rSrc;
    NORMALIZEDRECT rDest;
    FLOAT fAlpha;
    COLORREF clrSrcKey;
    } 	VMRALPHABITMAP;

typedef struct _VMRALPHABITMAP *PVMRALPHABITMAP;

#define VMRBITMAP_DISABLE            0x00000001
#define VMRBITMAP_HDC                0x00000002
#define VMRBITMAP_ENTIREDDS          0x00000004
#define VMRBITMAP_SRCCOLORKEY        0x00000008
#define VMRBITMAP_SRCRECT            0x00000010


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0125_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0125_v0_0_s_ifspec;

#ifndef __IVMRMixerBitmap_INTERFACE_DEFINED__
#define __IVMRMixerBitmap_INTERFACE_DEFINED__

/* interface IVMRMixerBitmap */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IVMRMixerBitmap;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("1E673275-0257-40aa-AF20-7C608D4A0428")
    IVMRMixerBitmap : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetAlphaBitmap( 
            /* [in] */ const VMRALPHABITMAP *pBmpParms) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpdateAlphaBitmapParameters( 
            /* [in] */ PVMRALPHABITMAP pBmpParms) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAlphaBitmapParameters( 
            /* [out] */ PVMRALPHABITMAP pBmpParms) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRMixerBitmapVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRMixerBitmap * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRMixerBitmap * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRMixerBitmap * This);
        
        DECLSPEC_XFGVIRT(IVMRMixerBitmap, SetAlphaBitmap)
        HRESULT ( STDMETHODCALLTYPE *SetAlphaBitmap )( 
            IVMRMixerBitmap * This,
            /* [in] */ const VMRALPHABITMAP *pBmpParms);
        
        DECLSPEC_XFGVIRT(IVMRMixerBitmap, UpdateAlphaBitmapParameters)
        HRESULT ( STDMETHODCALLTYPE *UpdateAlphaBitmapParameters )( 
            IVMRMixerBitmap * This,
            /* [in] */ PVMRALPHABITMAP pBmpParms);
        
        DECLSPEC_XFGVIRT(IVMRMixerBitmap, GetAlphaBitmapParameters)
        HRESULT ( STDMETHODCALLTYPE *GetAlphaBitmapParameters )( 
            IVMRMixerBitmap * This,
            /* [out] */ PVMRALPHABITMAP pBmpParms);
        
        END_INTERFACE
    } IVMRMixerBitmapVtbl;

    interface IVMRMixerBitmap
    {
        CONST_VTBL struct IVMRMixerBitmapVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRMixerBitmap_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRMixerBitmap_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRMixerBitmap_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRMixerBitmap_SetAlphaBitmap(This,pBmpParms)	\
    ( (This)->lpVtbl -> SetAlphaBitmap(This,pBmpParms) ) 

#define IVMRMixerBitmap_UpdateAlphaBitmapParameters(This,pBmpParms)	\
    ( (This)->lpVtbl -> UpdateAlphaBitmapParameters(This,pBmpParms) ) 

#define IVMRMixerBitmap_GetAlphaBitmapParameters(This,pBmpParms)	\
    ( (This)->lpVtbl -> GetAlphaBitmapParameters(This,pBmpParms) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRMixerBitmap_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0126 */
/* [local] */ 

typedef struct _VMRVIDEOSTREAMINFO
    {
    LPDIRECTDRAWSURFACE7 pddsVideoSurface;
    DWORD dwWidth;
    DWORD dwHeight;
    DWORD dwStrmID;
    FLOAT fAlpha;
    DDCOLORKEY ddClrKey;
    NORMALIZEDRECT rNormal;
    } 	VMRVIDEOSTREAMINFO;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0126_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0126_v0_0_s_ifspec;

#ifndef __IVMRImageCompositor_INTERFACE_DEFINED__
#define __IVMRImageCompositor_INTERFACE_DEFINED__

/* interface IVMRImageCompositor */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRImageCompositor;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("7a4fb5af-479f-4074-bb40-ce6722e43c82")
    IVMRImageCompositor : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE InitCompositionTarget( 
            /* [in] */ IUnknown *pD3DDevice,
            /* [in] */ LPDIRECTDRAWSURFACE7 pddsRenderTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TermCompositionTarget( 
            /* [in] */ IUnknown *pD3DDevice,
            /* [in] */ LPDIRECTDRAWSURFACE7 pddsRenderTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamMediaType( 
            /* [in] */ DWORD dwStrmID,
            /* [in] */ AM_MEDIA_TYPE *pmt,
            /* [in] */ BOOL fTexture) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CompositeImage( 
            /* [in] */ IUnknown *pD3DDevice,
            /* [in] */ LPDIRECTDRAWSURFACE7 pddsRenderTarget,
            /* [in] */ AM_MEDIA_TYPE *pmtRenderTarget,
            /* [in] */ REFERENCE_TIME rtStart,
            /* [in] */ REFERENCE_TIME rtEnd,
            /* [in] */ DWORD dwClrBkGnd,
            /* [in] */ VMRVIDEOSTREAMINFO *pVideoStreamInfo,
            /* [in] */ UINT cStreams) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRImageCompositorVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRImageCompositor * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRImageCompositor * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRImageCompositor * This);
        
        DECLSPEC_XFGVIRT(IVMRImageCompositor, InitCompositionTarget)
        HRESULT ( STDMETHODCALLTYPE *InitCompositionTarget )( 
            IVMRImageCompositor * This,
            /* [in] */ IUnknown *pD3DDevice,
            /* [in] */ LPDIRECTDRAWSURFACE7 pddsRenderTarget);
        
        DECLSPEC_XFGVIRT(IVMRImageCompositor, TermCompositionTarget)
        HRESULT ( STDMETHODCALLTYPE *TermCompositionTarget )( 
            IVMRImageCompositor * This,
            /* [in] */ IUnknown *pD3DDevice,
            /* [in] */ LPDIRECTDRAWSURFACE7 pddsRenderTarget);
        
        DECLSPEC_XFGVIRT(IVMRImageCompositor, SetStreamMediaType)
        HRESULT ( STDMETHODCALLTYPE *SetStreamMediaType )( 
            IVMRImageCompositor * This,
            /* [in] */ DWORD dwStrmID,
            /* [in] */ AM_MEDIA_TYPE *pmt,
            /* [in] */ BOOL fTexture);
        
        DECLSPEC_XFGVIRT(IVMRImageCompositor, CompositeImage)
        HRESULT ( STDMETHODCALLTYPE *CompositeImage )( 
            IVMRImageCompositor * This,
            /* [in] */ IUnknown *pD3DDevice,
            /* [in] */ LPDIRECTDRAWSURFACE7 pddsRenderTarget,
            /* [in] */ AM_MEDIA_TYPE *pmtRenderTarget,
            /* [in] */ REFERENCE_TIME rtStart,
            /* [in] */ REFERENCE_TIME rtEnd,
            /* [in] */ DWORD dwClrBkGnd,
            /* [in] */ VMRVIDEOSTREAMINFO *pVideoStreamInfo,
            /* [in] */ UINT cStreams);
        
        END_INTERFACE
    } IVMRImageCompositorVtbl;

    interface IVMRImageCompositor
    {
        CONST_VTBL struct IVMRImageCompositorVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRImageCompositor_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRImageCompositor_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRImageCompositor_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRImageCompositor_InitCompositionTarget(This,pD3DDevice,pddsRenderTarget)	\
    ( (This)->lpVtbl -> InitCompositionTarget(This,pD3DDevice,pddsRenderTarget) ) 

#define IVMRImageCompositor_TermCompositionTarget(This,pD3DDevice,pddsRenderTarget)	\
    ( (This)->lpVtbl -> TermCompositionTarget(This,pD3DDevice,pddsRenderTarget) ) 

#define IVMRImageCompositor_SetStreamMediaType(This,dwStrmID,pmt,fTexture)	\
    ( (This)->lpVtbl -> SetStreamMediaType(This,dwStrmID,pmt,fTexture) ) 

#define IVMRImageCompositor_CompositeImage(This,pD3DDevice,pddsRenderTarget,pmtRenderTarget,rtStart,rtEnd,dwClrBkGnd,pVideoStreamInfo,cStreams)	\
    ( (This)->lpVtbl -> CompositeImage(This,pD3DDevice,pddsRenderTarget,pmtRenderTarget,rtStart,rtEnd,dwClrBkGnd,pVideoStreamInfo,cStreams) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRImageCompositor_INTERFACE_DEFINED__ */


#ifndef __IVMRVideoStreamControl_INTERFACE_DEFINED__
#define __IVMRVideoStreamControl_INTERFACE_DEFINED__

/* interface IVMRVideoStreamControl */
/* [unique][helpstring][uuid][local][object] */ 


EXTERN_C const IID IID_IVMRVideoStreamControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("058d1f11-2a54-4bef-bd54-df706626b727")
    IVMRVideoStreamControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetColorKey( 
            /* [in] */ LPDDCOLORKEY lpClrKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetColorKey( 
            /* [out] */ LPDDCOLORKEY lpClrKey) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetStreamActiveState( 
            /* [in] */ BOOL fActive) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetStreamActiveState( 
            /* [out] */ BOOL *lpfActive) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRVideoStreamControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRVideoStreamControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRVideoStreamControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRVideoStreamControl * This);
        
        DECLSPEC_XFGVIRT(IVMRVideoStreamControl, SetColorKey)
        HRESULT ( STDMETHODCALLTYPE *SetColorKey )( 
            IVMRVideoStreamControl * This,
            /* [in] */ LPDDCOLORKEY lpClrKey);
        
        DECLSPEC_XFGVIRT(IVMRVideoStreamControl, GetColorKey)
        HRESULT ( STDMETHODCALLTYPE *GetColorKey )( 
            IVMRVideoStreamControl * This,
            /* [out] */ LPDDCOLORKEY lpClrKey);
        
        DECLSPEC_XFGVIRT(IVMRVideoStreamControl, SetStreamActiveState)
        HRESULT ( STDMETHODCALLTYPE *SetStreamActiveState )( 
            IVMRVideoStreamControl * This,
            /* [in] */ BOOL fActive);
        
        DECLSPEC_XFGVIRT(IVMRVideoStreamControl, GetStreamActiveState)
        HRESULT ( STDMETHODCALLTYPE *GetStreamActiveState )( 
            IVMRVideoStreamControl * This,
            /* [out] */ BOOL *lpfActive);
        
        END_INTERFACE
    } IVMRVideoStreamControlVtbl;

    interface IVMRVideoStreamControl
    {
        CONST_VTBL struct IVMRVideoStreamControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRVideoStreamControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRVideoStreamControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRVideoStreamControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRVideoStreamControl_SetColorKey(This,lpClrKey)	\
    ( (This)->lpVtbl -> SetColorKey(This,lpClrKey) ) 

#define IVMRVideoStreamControl_GetColorKey(This,lpClrKey)	\
    ( (This)->lpVtbl -> GetColorKey(This,lpClrKey) ) 

#define IVMRVideoStreamControl_SetStreamActiveState(This,fActive)	\
    ( (This)->lpVtbl -> SetStreamActiveState(This,fActive) ) 

#define IVMRVideoStreamControl_GetStreamActiveState(This,lpfActive)	\
    ( (This)->lpVtbl -> GetStreamActiveState(This,lpfActive) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRVideoStreamControl_INTERFACE_DEFINED__ */


#ifndef __IVMRSurface_INTERFACE_DEFINED__
#define __IVMRSurface_INTERFACE_DEFINED__

/* interface IVMRSurface */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRSurface;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("a9849bbe-9ec8-4263-b764-62730f0d15d0")
    IVMRSurface : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE IsSurfaceLocked( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LockSurface( 
            /* [out] */ BYTE **lpSurface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UnlockSurface( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSurface( 
            /* [out] */ LPDIRECTDRAWSURFACE7 *lplpSurface) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRSurfaceVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRSurface * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRSurface * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRSurface * This);
        
        DECLSPEC_XFGVIRT(IVMRSurface, IsSurfaceLocked)
        HRESULT ( STDMETHODCALLTYPE *IsSurfaceLocked )( 
            IVMRSurface * This);
        
        DECLSPEC_XFGVIRT(IVMRSurface, LockSurface)
        HRESULT ( STDMETHODCALLTYPE *LockSurface )( 
            IVMRSurface * This,
            /* [out] */ BYTE **lpSurface);
        
        DECLSPEC_XFGVIRT(IVMRSurface, UnlockSurface)
        HRESULT ( STDMETHODCALLTYPE *UnlockSurface )( 
            IVMRSurface * This);
        
        DECLSPEC_XFGVIRT(IVMRSurface, GetSurface)
        HRESULT ( STDMETHODCALLTYPE *GetSurface )( 
            IVMRSurface * This,
            /* [out] */ LPDIRECTDRAWSURFACE7 *lplpSurface);
        
        END_INTERFACE
    } IVMRSurfaceVtbl;

    interface IVMRSurface
    {
        CONST_VTBL struct IVMRSurfaceVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRSurface_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRSurface_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRSurface_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRSurface_IsSurfaceLocked(This)	\
    ( (This)->lpVtbl -> IsSurfaceLocked(This) ) 

#define IVMRSurface_LockSurface(This,lpSurface)	\
    ( (This)->lpVtbl -> LockSurface(This,lpSurface) ) 

#define IVMRSurface_UnlockSurface(This)	\
    ( (This)->lpVtbl -> UnlockSurface(This) ) 

#define IVMRSurface_GetSurface(This,lplpSurface)	\
    ( (This)->lpVtbl -> GetSurface(This,lplpSurface) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRSurface_INTERFACE_DEFINED__ */


#ifndef __IVMRImagePresenterConfig_INTERFACE_DEFINED__
#define __IVMRImagePresenterConfig_INTERFACE_DEFINED__

/* interface IVMRImagePresenterConfig */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRImagePresenterConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("9f3a1c85-8555-49ba-935f-be5b5b29d178")
    IVMRImagePresenterConfig : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetRenderingPrefs( 
            /* [in] */ DWORD dwRenderFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRenderingPrefs( 
            /* [out] */ DWORD *dwRenderFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRImagePresenterConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRImagePresenterConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRImagePresenterConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRImagePresenterConfig * This);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenterConfig, SetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetRenderingPrefs )( 
            IVMRImagePresenterConfig * This,
            /* [in] */ DWORD dwRenderFlags);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenterConfig, GetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetRenderingPrefs )( 
            IVMRImagePresenterConfig * This,
            /* [out] */ DWORD *dwRenderFlags);
        
        END_INTERFACE
    } IVMRImagePresenterConfigVtbl;

    interface IVMRImagePresenterConfig
    {
        CONST_VTBL struct IVMRImagePresenterConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRImagePresenterConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRImagePresenterConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRImagePresenterConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRImagePresenterConfig_SetRenderingPrefs(This,dwRenderFlags)	\
    ( (This)->lpVtbl -> SetRenderingPrefs(This,dwRenderFlags) ) 

#define IVMRImagePresenterConfig_GetRenderingPrefs(This,dwRenderFlags)	\
    ( (This)->lpVtbl -> GetRenderingPrefs(This,dwRenderFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRImagePresenterConfig_INTERFACE_DEFINED__ */


#ifndef __IVMRImagePresenterExclModeConfig_INTERFACE_DEFINED__
#define __IVMRImagePresenterExclModeConfig_INTERFACE_DEFINED__

/* interface IVMRImagePresenterExclModeConfig */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVMRImagePresenterExclModeConfig;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("e6f7ce40-4673-44f1-8f77-5499d68cb4ea")
    IVMRImagePresenterExclModeConfig : public IVMRImagePresenterConfig
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetXlcModeDDObjAndPrimarySurface( 
            /* [in] */ LPDIRECTDRAW7 lpDDObj,
            /* [in] */ LPDIRECTDRAWSURFACE7 lpPrimarySurf) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetXlcModeDDObjAndPrimarySurface( 
            /* [out] */ LPDIRECTDRAW7 *lpDDObj,
            /* [out] */ LPDIRECTDRAWSURFACE7 *lpPrimarySurf) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVMRImagePresenterExclModeConfigVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVMRImagePresenterExclModeConfig * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVMRImagePresenterExclModeConfig * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVMRImagePresenterExclModeConfig * This);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenterConfig, SetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *SetRenderingPrefs )( 
            IVMRImagePresenterExclModeConfig * This,
            /* [in] */ DWORD dwRenderFlags);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenterConfig, GetRenderingPrefs)
        HRESULT ( STDMETHODCALLTYPE *GetRenderingPrefs )( 
            IVMRImagePresenterExclModeConfig * This,
            /* [out] */ DWORD *dwRenderFlags);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenterExclModeConfig, SetXlcModeDDObjAndPrimarySurface)
        HRESULT ( STDMETHODCALLTYPE *SetXlcModeDDObjAndPrimarySurface )( 
            IVMRImagePresenterExclModeConfig * This,
            /* [in] */ LPDIRECTDRAW7 lpDDObj,
            /* [in] */ LPDIRECTDRAWSURFACE7 lpPrimarySurf);
        
        DECLSPEC_XFGVIRT(IVMRImagePresenterExclModeConfig, GetXlcModeDDObjAndPrimarySurface)
        HRESULT ( STDMETHODCALLTYPE *GetXlcModeDDObjAndPrimarySurface )( 
            IVMRImagePresenterExclModeConfig * This,
            /* [out] */ LPDIRECTDRAW7 *lpDDObj,
            /* [out] */ LPDIRECTDRAWSURFACE7 *lpPrimarySurf);
        
        END_INTERFACE
    } IVMRImagePresenterExclModeConfigVtbl;

    interface IVMRImagePresenterExclModeConfig
    {
        CONST_VTBL struct IVMRImagePresenterExclModeConfigVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVMRImagePresenterExclModeConfig_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVMRImagePresenterExclModeConfig_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVMRImagePresenterExclModeConfig_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVMRImagePresenterExclModeConfig_SetRenderingPrefs(This,dwRenderFlags)	\
    ( (This)->lpVtbl -> SetRenderingPrefs(This,dwRenderFlags) ) 

#define IVMRImagePresenterExclModeConfig_GetRenderingPrefs(This,dwRenderFlags)	\
    ( (This)->lpVtbl -> GetRenderingPrefs(This,dwRenderFlags) ) 


#define IVMRImagePresenterExclModeConfig_SetXlcModeDDObjAndPrimarySurface(This,lpDDObj,lpPrimarySurf)	\
    ( (This)->lpVtbl -> SetXlcModeDDObjAndPrimarySurface(This,lpDDObj,lpPrimarySurf) ) 

#define IVMRImagePresenterExclModeConfig_GetXlcModeDDObjAndPrimarySurface(This,lpDDObj,lpPrimarySurf)	\
    ( (This)->lpVtbl -> GetXlcModeDDObjAndPrimarySurface(This,lpDDObj,lpPrimarySurf) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVMRImagePresenterExclModeConfig_INTERFACE_DEFINED__ */


#ifndef __IVPManager_INTERFACE_DEFINED__
#define __IVPManager_INTERFACE_DEFINED__

/* interface IVPManager */
/* [unique][helpstring][uuid][local][object][local] */ 


EXTERN_C const IID IID_IVPManager;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("aac18c18-e186-46d2-825d-a1f8dc8e395a")
    IVPManager : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetVideoPortIndex( 
            /* [in] */ DWORD dwVideoPortIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVideoPortIndex( 
            /* [out] */ DWORD *pdwVideoPortIndex) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IVPManagerVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IVPManager * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IVPManager * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IVPManager * This);
        
        DECLSPEC_XFGVIRT(IVPManager, SetVideoPortIndex)
        HRESULT ( STDMETHODCALLTYPE *SetVideoPortIndex )( 
            IVPManager * This,
            /* [in] */ DWORD dwVideoPortIndex);
        
        DECLSPEC_XFGVIRT(IVPManager, GetVideoPortIndex)
        HRESULT ( STDMETHODCALLTYPE *GetVideoPortIndex )( 
            IVPManager * This,
            /* [out] */ DWORD *pdwVideoPortIndex);
        
        END_INTERFACE
    } IVPManagerVtbl;

    interface IVPManager
    {
        CONST_VTBL struct IVPManagerVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IVPManager_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IVPManager_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IVPManager_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IVPManager_SetVideoPortIndex(This,dwVideoPortIndex)	\
    ( (This)->lpVtbl -> SetVideoPortIndex(This,dwVideoPortIndex) ) 

#define IVPManager_GetVideoPortIndex(This,pdwVideoPortIndex)	\
    ( (This)->lpVtbl -> GetVideoPortIndex(This,pdwVideoPortIndex) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IVPManager_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0132 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#include <winapifamily.h>
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#include <ddraw.h>









#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)
typedef 
enum tagDVD_DOMAIN
    {
        DVD_DOMAIN_FirstPlay	= 1,
        DVD_DOMAIN_VideoManagerMenu	= ( DVD_DOMAIN_FirstPlay + 1 ) ,
        DVD_DOMAIN_VideoTitleSetMenu	= ( DVD_DOMAIN_VideoManagerMenu + 1 ) ,
        DVD_DOMAIN_Title	= ( DVD_DOMAIN_VideoTitleSetMenu + 1 ) ,
        DVD_DOMAIN_Stop	= ( DVD_DOMAIN_Title + 1 ) 
    } 	DVD_DOMAIN;

typedef 
enum tagDVD_MENU_ID
    {
        DVD_MENU_Title	= 2,
        DVD_MENU_Root	= 3,
        DVD_MENU_Subpicture	= 4,
        DVD_MENU_Audio	= 5,
        DVD_MENU_Angle	= 6,
        DVD_MENU_Chapter	= 7
    } 	DVD_MENU_ID;

typedef 
enum tagDVD_DISC_SIDE
    {
        DVD_SIDE_A	= 1,
        DVD_SIDE_B	= 2
    } 	DVD_DISC_SIDE;

typedef 
enum tagDVD_PREFERRED_DISPLAY_MODE
    {
        DISPLAY_CONTENT_DEFAULT	= 0,
        DISPLAY_16x9	= 1,
        DISPLAY_4x3_PANSCAN_PREFERRED	= 2,
        DISPLAY_4x3_LETTERBOX_PREFERRED	= 3
    } 	DVD_PREFERRED_DISPLAY_MODE;

typedef WORD DVD_REGISTER;

typedef DVD_REGISTER GPRMARRAY[ 16 ];

typedef DVD_REGISTER SPRMARRAY[ 24 ];

typedef struct tagDVD_ATR
    {
    ULONG ulCAT;
    BYTE pbATRI[ 768 ];
    } 	DVD_ATR;

typedef BYTE DVD_VideoATR[ 2 ];

typedef BYTE DVD_AudioATR[ 8 ];

typedef BYTE DVD_SubpictureATR[ 6 ];

typedef 
enum tagDVD_FRAMERATE
    {
        DVD_FPS_25	= 1,
        DVD_FPS_30NonDrop	= 3
    } 	DVD_FRAMERATE;

typedef struct tagDVD_TIMECODE
{
   ULONG Hours1    :4; // Hours
   ULONG Hours10  :4; // Tens of Hours 

   ULONG Minutes1  :4; // Minutes 
   ULONG Minutes10:4; // Tens of Minutes 

   ULONG Seconds1  :4; // Seconds 
   ULONG Seconds10:4; // Tens of Seconds 

   ULONG Frames1   :4; // Frames 
   ULONG Frames10 :2; // Tens of Frames 

   ULONG FrameRateCode: 2; // use DVD_FRAMERATE to indicate frames/sec and drop/non-drop
} DVD_TIMECODE;
typedef 
enum tagDVD_NavCmdType
    {
        DVD_NavCmdType_Pre	= 1,
        DVD_NavCmdType_Post	= 2,
        DVD_NavCmdType_Cell	= 3,
        DVD_NavCmdType_Button	= 4
    } 	DVD_NavCmdType;

typedef 
enum tagDVD_TIMECODE_FLAGS
    {
        DVD_TC_FLAG_25fps	= 0x1,
        DVD_TC_FLAG_30fps	= 0x2,
        DVD_TC_FLAG_DropFrame	= 0x4,
        DVD_TC_FLAG_Interpolated	= 0x8
    } 	DVD_TIMECODE_FLAGS;

typedef struct tagDVD_HMSF_TIMECODE
    {
    BYTE bHours;
    BYTE bMinutes;
    BYTE bSeconds;
    BYTE bFrames;
    } 	DVD_HMSF_TIMECODE;

typedef struct tagDVD_PLAYBACK_LOCATION2
    {
    ULONG TitleNum;
    ULONG ChapterNum;
    DVD_HMSF_TIMECODE TimeCode;
    ULONG TimeCodeFlags;
    } 	DVD_PLAYBACK_LOCATION2;

typedef struct tagDVD_PLAYBACK_LOCATION
    {
    ULONG TitleNum;
    ULONG ChapterNum;
    ULONG TimeCode;
    } 	DVD_PLAYBACK_LOCATION;

typedef DWORD VALID_UOP_SOMTHING_OR_OTHER;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_strmif_0000_0132_0001
    {
        UOP_FLAG_Play_Title_Or_AtTime	= 0x1,
        UOP_FLAG_Play_Chapter	= 0x2,
        UOP_FLAG_Play_Title	= 0x4,
        UOP_FLAG_Stop	= 0x8,
        UOP_FLAG_ReturnFromSubMenu	= 0x10,
        UOP_FLAG_Play_Chapter_Or_AtTime	= 0x20,
        UOP_FLAG_PlayPrev_Or_Replay_Chapter	= 0x40,
        UOP_FLAG_PlayNext_Chapter	= 0x80,
        UOP_FLAG_Play_Forwards	= 0x100,
        UOP_FLAG_Play_Backwards	= 0x200,
        UOP_FLAG_ShowMenu_Title	= 0x400,
        UOP_FLAG_ShowMenu_Root	= 0x800,
        UOP_FLAG_ShowMenu_SubPic	= 0x1000,
        UOP_FLAG_ShowMenu_Audio	= 0x2000,
        UOP_FLAG_ShowMenu_Angle	= 0x4000,
        UOP_FLAG_ShowMenu_Chapter	= 0x8000,
        UOP_FLAG_Resume	= 0x10000,
        UOP_FLAG_Select_Or_Activate_Button	= 0x20000,
        UOP_FLAG_Still_Off	= 0x40000,
        UOP_FLAG_Pause_On	= 0x80000,
        UOP_FLAG_Select_Audio_Stream	= 0x100000,
        UOP_FLAG_Select_SubPic_Stream	= 0x200000,
        UOP_FLAG_Select_Angle	= 0x400000,
        UOP_FLAG_Select_Karaoke_Audio_Presentation_Mode	= 0x800000,
        UOP_FLAG_Select_Video_Mode_Preference	= 0x1000000
    } 	VALID_UOP_FLAG;

typedef /* [public] */ 
enum __MIDL___MIDL_itf_strmif_0000_0132_0002
    {
        DVD_CMD_FLAG_None	= 0,
        DVD_CMD_FLAG_Flush	= 0x1,
        DVD_CMD_FLAG_SendEvents	= 0x2,
        DVD_CMD_FLAG_Block	= 0x4,
        DVD_CMD_FLAG_StartWhenRendered	= 0x8,
        DVD_CMD_FLAG_EndAfterRendered	= 0x10
    } 	DVD_CMD_FLAGS;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_strmif_0000_0132_0003
    {
        DVD_ResetOnStop	= 1,
        DVD_NotifyParentalLevelChange	= 2,
        DVD_HMSF_TimeCodeEvents	= 3,
        DVD_AudioDuringFFwdRew	= 4,
        DVD_EnableNonblockingAPIs	= 5,
        DVD_CacheSizeInMB	= 6,
        DVD_EnablePortableBookmarks	= 7,
        DVD_EnableExtendedCopyProtectErrors	= 8,
        DVD_NotifyPositionChange	= 9,
        DVD_IncreaseOutputControl	= 10,
        DVD_EnableStreaming	= 11,
        DVD_EnableESOutput	= 12,
        DVD_EnableTitleLength	= 13,
        DVD_DisableStillThrottle	= 14,
        DVD_EnableLoggingEvents	= 15,
        DVD_MaxReadBurstInKB	= 16,
        DVD_ReadBurstPeriodInMS	= 17,
        DVD_RestartDisc	= 18,
        DVD_EnableCC	= 19
    } 	DVD_OPTION_FLAG;

typedef /* [public][public] */ 
enum __MIDL___MIDL_itf_strmif_0000_0132_0004
    {
        DVD_Relative_Upper	= 1,
        DVD_Relative_Lower	= 2,
        DVD_Relative_Left	= 3,
        DVD_Relative_Right	= 4
    } 	DVD_RELATIVE_BUTTON;

typedef 
enum tagDVD_PARENTAL_LEVEL
    {
        DVD_PARENTAL_LEVEL_8	= 0x8000,
        DVD_PARENTAL_LEVEL_7	= 0x4000,
        DVD_PARENTAL_LEVEL_6	= 0x2000,
        DVD_PARENTAL_LEVEL_5	= 0x1000,
        DVD_PARENTAL_LEVEL_4	= 0x800,
        DVD_PARENTAL_LEVEL_3	= 0x400,
        DVD_PARENTAL_LEVEL_2	= 0x200,
        DVD_PARENTAL_LEVEL_1	= 0x100
    } 	DVD_PARENTAL_LEVEL;

typedef 
enum tagDVD_AUDIO_LANG_EXT
    {
        DVD_AUD_EXT_NotSpecified	= 0,
        DVD_AUD_EXT_Captions	= 1,
        DVD_AUD_EXT_VisuallyImpaired	= 2,
        DVD_AUD_EXT_DirectorComments1	= 3,
        DVD_AUD_EXT_DirectorComments2	= 4
    } 	DVD_AUDIO_LANG_EXT;

typedef 
enum tagDVD_SUBPICTURE_LANG_EXT
    {
        DVD_SP_EXT_NotSpecified	= 0,
        DVD_SP_EXT_Caption_Normal	= 1,
        DVD_SP_EXT_Caption_Big	= 2,
        DVD_SP_EXT_Caption_Children	= 3,
        DVD_SP_EXT_CC_Normal	= 5,
        DVD_SP_EXT_CC_Big	= 6,
        DVD_SP_EXT_CC_Children	= 7,
        DVD_SP_EXT_Forced	= 9,
        DVD_SP_EXT_DirectorComments_Normal	= 13,
        DVD_SP_EXT_DirectorComments_Big	= 14,
        DVD_SP_EXT_DirectorComments_Children	= 15
    } 	DVD_SUBPICTURE_LANG_EXT;

typedef 
enum tagDVD_AUDIO_APPMODE
    {
        DVD_AudioMode_None	= 0,
        DVD_AudioMode_Karaoke	= 1,
        DVD_AudioMode_Surround	= 2,
        DVD_AudioMode_Other	= 3
    } 	DVD_AUDIO_APPMODE;

typedef 
enum tagDVD_AUDIO_FORMAT
    {
        DVD_AudioFormat_AC3	= 0,
        DVD_AudioFormat_MPEG1	= 1,
        DVD_AudioFormat_MPEG1_DRC	= 2,
        DVD_AudioFormat_MPEG2	= 3,
        DVD_AudioFormat_MPEG2_DRC	= 4,
        DVD_AudioFormat_LPCM	= 5,
        DVD_AudioFormat_DTS	= 6,
        DVD_AudioFormat_SDDS	= 7,
        DVD_AudioFormat_Other	= 8
    } 	DVD_AUDIO_FORMAT;

typedef 
enum tagDVD_KARAOKE_DOWNMIX
    {
        DVD_Mix_0to0	= 0x1,
        DVD_Mix_1to0	= 0x2,
        DVD_Mix_2to0	= 0x4,
        DVD_Mix_3to0	= 0x8,
        DVD_Mix_4to0	= 0x10,
        DVD_Mix_Lto0	= 0x20,
        DVD_Mix_Rto0	= 0x40,
        DVD_Mix_0to1	= 0x100,
        DVD_Mix_1to1	= 0x200,
        DVD_Mix_2to1	= 0x400,
        DVD_Mix_3to1	= 0x800,
        DVD_Mix_4to1	= 0x1000,
        DVD_Mix_Lto1	= 0x2000,
        DVD_Mix_Rto1	= 0x4000
    } 	DVD_KARAOKE_DOWNMIX;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct tagDVD_AudioAttributes
    {
    DVD_AUDIO_APPMODE AppMode;
    BYTE AppModeData;
    DVD_AUDIO_FORMAT AudioFormat;
    LCID Language;
    DVD_AUDIO_LANG_EXT LanguageExtension;
    BOOL fHasMultichannelInfo;
    DWORD dwFrequency;
    BYTE bQuantization;
    BYTE bNumberOfChannels;
    DWORD dwReserved[ 2 ];
    } 	DVD_AudioAttributes;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
typedef struct tagDVD_MUA_MixingInfo
    {
    BOOL fMixTo0;
    BOOL fMixTo1;
    BOOL fMix0InPhase;
    BOOL fMix1InPhase;
    DWORD dwSpeakerPosition;
    } 	DVD_MUA_MixingInfo;

typedef struct tagDVD_MUA_Coeff
    {
    double log2_alpha;
    double log2_beta;
    } 	DVD_MUA_Coeff;

typedef struct tagDVD_MultichannelAudioAttributes
    {
    DVD_MUA_MixingInfo Info[ 8 ];
    DVD_MUA_Coeff Coeff[ 8 ];
    } 	DVD_MultichannelAudioAttributes;

typedef 
enum tagDVD_KARAOKE_CONTENTS
    {
        DVD_Karaoke_GuideVocal1	= 0x1,
        DVD_Karaoke_GuideVocal2	= 0x2,
        DVD_Karaoke_GuideMelody1	= 0x4,
        DVD_Karaoke_GuideMelody2	= 0x8,
        DVD_Karaoke_GuideMelodyA	= 0x10,
        DVD_Karaoke_GuideMelodyB	= 0x20,
        DVD_Karaoke_SoundEffectA	= 0x40,
        DVD_Karaoke_SoundEffectB	= 0x80
    } 	DVD_KARAOKE_CONTENTS;

typedef 
enum tagDVD_KARAOKE_ASSIGNMENT
    {
        DVD_Assignment_reserved0	= 0,
        DVD_Assignment_reserved1	= 1,
        DVD_Assignment_LR	= 2,
        DVD_Assignment_LRM	= 3,
        DVD_Assignment_LR1	= 4,
        DVD_Assignment_LRM1	= 5,
        DVD_Assignment_LR12	= 6,
        DVD_Assignment_LRM12	= 7
    } 	DVD_KARAOKE_ASSIGNMENT;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct tagDVD_KaraokeAttributes
    {
    BYTE bVersion;
    BOOL fMasterOfCeremoniesInGuideVocal1;
    BOOL fDuet;
    DVD_KARAOKE_ASSIGNMENT ChannelAssignment;
    WORD wChannelContents[ 8 ];
    } 	DVD_KaraokeAttributes;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
typedef 
enum tagDVD_VIDEO_COMPRESSION
    {
        DVD_VideoCompression_Other	= 0,
        DVD_VideoCompression_MPEG1	= 1,
        DVD_VideoCompression_MPEG2	= 2
    } 	DVD_VIDEO_COMPRESSION;

typedef struct tagDVD_VideoAttributes
    {
    BOOL fPanscanPermitted;
    BOOL fLetterboxPermitted;
    ULONG ulAspectX;
    ULONG ulAspectY;
    ULONG ulFrameRate;
    ULONG ulFrameHeight;
    DVD_VIDEO_COMPRESSION Compression;
    BOOL fLine21Field1InGOP;
    BOOL fLine21Field2InGOP;
    ULONG ulSourceResolutionX;
    ULONG ulSourceResolutionY;
    BOOL fIsSourceLetterboxed;
    BOOL fIsFilmMode;
    } 	DVD_VideoAttributes;

typedef 
enum tagDVD_SUBPICTURE_TYPE
    {
        DVD_SPType_NotSpecified	= 0,
        DVD_SPType_Language	= 1,
        DVD_SPType_Other	= 2
    } 	DVD_SUBPICTURE_TYPE;

typedef 
enum tagDVD_SUBPICTURE_CODING
    {
        DVD_SPCoding_RunLength	= 0,
        DVD_SPCoding_Extended	= 1,
        DVD_SPCoding_Other	= 2
    } 	DVD_SUBPICTURE_CODING;

typedef struct tagDVD_SubpictureAttributes
    {
    DVD_SUBPICTURE_TYPE Type;
    DVD_SUBPICTURE_CODING CodingMode;
    LCID Language;
    DVD_SUBPICTURE_LANG_EXT LanguageExtension;
    } 	DVD_SubpictureAttributes;

typedef 
enum tagDVD_TITLE_APPMODE
    {
        DVD_AppMode_Not_Specified	= 0,
        DVD_AppMode_Karaoke	= 1,
        DVD_AppMode_Other	= 3
    } 	DVD_TITLE_APPMODE;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(push)
#pragma warning(disable:4820) // Disable C4820: padding after data member
#endif
typedef struct tagDVD_TitleMainAttributes
    {
    union 
        {
        DVD_TITLE_APPMODE AppMode;
        DVD_HMSF_TIMECODE TitleLength;
        } 	;
    DVD_VideoAttributes VideoAttributes;
    ULONG ulNumberOfAudioStreams;
    DVD_AudioAttributes AudioAttributes[ 8 ];
    DVD_MultichannelAudioAttributes MultichannelAudioAttributes[ 8 ];
    ULONG ulNumberOfSubpictureStreams;
    DVD_SubpictureAttributes SubpictureAttributes[ 32 ];
    } 	DVD_TitleAttributes;

#if defined(_MSC_VER) && (_MSC_VER >= 1600)
#pragma warning(pop)
#endif
typedef struct tagDVD_MenuAttributes
    {
    BOOL fCompatibleRegion[ 8 ];
    DVD_VideoAttributes VideoAttributes;
    BOOL fAudioPresent;
    DVD_AudioAttributes AudioAttributes;
    BOOL fSubpicturePresent;
    DVD_SubpictureAttributes SubpictureAttributes;
    } 	DVD_MenuAttributes;

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0132_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0132_v0_0_s_ifspec;

#ifndef __IDvdControl_INTERFACE_DEFINED__
#define __IDvdControl_INTERFACE_DEFINED__

/* interface IDvdControl */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvdControl;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A70EFE61-E2A3-11d0-A9BE-00AA0061BE93")
    IDvdControl : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE TitlePlay( 
            /* [in] */ ULONG ulTitle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChapterPlay( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TimePlay( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG bcdTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StopForResume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GoUp( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TimeSearch( 
            /* [in] */ ULONG bcdTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChapterSearch( 
            /* [in] */ ULONG ulChapter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PrevPGSearch( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE TopPGSearch( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE NextPGSearch( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ForwardScan( 
            /* [in] */ double dwSpeed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE BackwardScan( 
            /* [in] */ double dwSpeed) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MenuCall( 
            /* [in] */ DVD_MENU_ID MenuID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE UpperButtonSelect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LowerButtonSelect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE LeftButtonSelect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RightButtonSelect( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ButtonActivate( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ButtonSelectAndActivate( 
            /* [in] */ ULONG ulButton) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StillOff( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PauseOn( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PauseOff( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MenuLanguageSelect( 
            /* [in] */ LCID Language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AudioStreamChange( 
            /* [in] */ ULONG ulAudio) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SubpictureStreamChange( 
            /* [in] */ ULONG ulSubPicture,
            /* [in] */ BOOL bDisplay) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AngleChange( 
            /* [in] */ ULONG ulAngle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParentalLevelSelect( 
            /* [in] */ ULONG ulParentalLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ParentalCountrySelect( 
            /* [in] */ WORD wCountry) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE KaraokeAudioPresentationModeChange( 
            /* [in] */ ULONG ulMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE VideoModePreferrence( 
            /* [in] */ ULONG ulPreferredDisplayMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetRoot( 
            /* [in] */ LPCWSTR pszPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MouseActivate( 
            /* [in] */ POINT point) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE MouseSelect( 
            /* [in] */ POINT point) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ChapterPlayAutoStop( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter,
            /* [in] */ ULONG ulChaptersToPlay) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdControlVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdControl * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, TitlePlay)
        HRESULT ( STDMETHODCALLTYPE *TitlePlay )( 
            IDvdControl * This,
            /* [in] */ ULONG ulTitle);
        
        DECLSPEC_XFGVIRT(IDvdControl, ChapterPlay)
        HRESULT ( STDMETHODCALLTYPE *ChapterPlay )( 
            IDvdControl * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter);
        
        DECLSPEC_XFGVIRT(IDvdControl, TimePlay)
        HRESULT ( STDMETHODCALLTYPE *TimePlay )( 
            IDvdControl * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG bcdTime);
        
        DECLSPEC_XFGVIRT(IDvdControl, StopForResume)
        HRESULT ( STDMETHODCALLTYPE *StopForResume )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, GoUp)
        HRESULT ( STDMETHODCALLTYPE *GoUp )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, TimeSearch)
        HRESULT ( STDMETHODCALLTYPE *TimeSearch )( 
            IDvdControl * This,
            /* [in] */ ULONG bcdTime);
        
        DECLSPEC_XFGVIRT(IDvdControl, ChapterSearch)
        HRESULT ( STDMETHODCALLTYPE *ChapterSearch )( 
            IDvdControl * This,
            /* [in] */ ULONG ulChapter);
        
        DECLSPEC_XFGVIRT(IDvdControl, PrevPGSearch)
        HRESULT ( STDMETHODCALLTYPE *PrevPGSearch )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, TopPGSearch)
        HRESULT ( STDMETHODCALLTYPE *TopPGSearch )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, NextPGSearch)
        HRESULT ( STDMETHODCALLTYPE *NextPGSearch )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, ForwardScan)
        HRESULT ( STDMETHODCALLTYPE *ForwardScan )( 
            IDvdControl * This,
            /* [in] */ double dwSpeed);
        
        DECLSPEC_XFGVIRT(IDvdControl, BackwardScan)
        HRESULT ( STDMETHODCALLTYPE *BackwardScan )( 
            IDvdControl * This,
            /* [in] */ double dwSpeed);
        
        DECLSPEC_XFGVIRT(IDvdControl, MenuCall)
        HRESULT ( STDMETHODCALLTYPE *MenuCall )( 
            IDvdControl * This,
            /* [in] */ DVD_MENU_ID MenuID);
        
        DECLSPEC_XFGVIRT(IDvdControl, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, UpperButtonSelect)
        HRESULT ( STDMETHODCALLTYPE *UpperButtonSelect )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, LowerButtonSelect)
        HRESULT ( STDMETHODCALLTYPE *LowerButtonSelect )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, LeftButtonSelect)
        HRESULT ( STDMETHODCALLTYPE *LeftButtonSelect )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, RightButtonSelect)
        HRESULT ( STDMETHODCALLTYPE *RightButtonSelect )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, ButtonActivate)
        HRESULT ( STDMETHODCALLTYPE *ButtonActivate )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, ButtonSelectAndActivate)
        HRESULT ( STDMETHODCALLTYPE *ButtonSelectAndActivate )( 
            IDvdControl * This,
            /* [in] */ ULONG ulButton);
        
        DECLSPEC_XFGVIRT(IDvdControl, StillOff)
        HRESULT ( STDMETHODCALLTYPE *StillOff )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, PauseOn)
        HRESULT ( STDMETHODCALLTYPE *PauseOn )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, PauseOff)
        HRESULT ( STDMETHODCALLTYPE *PauseOff )( 
            IDvdControl * This);
        
        DECLSPEC_XFGVIRT(IDvdControl, MenuLanguageSelect)
        HRESULT ( STDMETHODCALLTYPE *MenuLanguageSelect )( 
            IDvdControl * This,
            /* [in] */ LCID Language);
        
        DECLSPEC_XFGVIRT(IDvdControl, AudioStreamChange)
        HRESULT ( STDMETHODCALLTYPE *AudioStreamChange )( 
            IDvdControl * This,
            /* [in] */ ULONG ulAudio);
        
        DECLSPEC_XFGVIRT(IDvdControl, SubpictureStreamChange)
        HRESULT ( STDMETHODCALLTYPE *SubpictureStreamChange )( 
            IDvdControl * This,
            /* [in] */ ULONG ulSubPicture,
            /* [in] */ BOOL bDisplay);
        
        DECLSPEC_XFGVIRT(IDvdControl, AngleChange)
        HRESULT ( STDMETHODCALLTYPE *AngleChange )( 
            IDvdControl * This,
            /* [in] */ ULONG ulAngle);
        
        DECLSPEC_XFGVIRT(IDvdControl, ParentalLevelSelect)
        HRESULT ( STDMETHODCALLTYPE *ParentalLevelSelect )( 
            IDvdControl * This,
            /* [in] */ ULONG ulParentalLevel);
        
        DECLSPEC_XFGVIRT(IDvdControl, ParentalCountrySelect)
        HRESULT ( STDMETHODCALLTYPE *ParentalCountrySelect )( 
            IDvdControl * This,
            /* [in] */ WORD wCountry);
        
        DECLSPEC_XFGVIRT(IDvdControl, KaraokeAudioPresentationModeChange)
        HRESULT ( STDMETHODCALLTYPE *KaraokeAudioPresentationModeChange )( 
            IDvdControl * This,
            /* [in] */ ULONG ulMode);
        
        DECLSPEC_XFGVIRT(IDvdControl, VideoModePreferrence)
        HRESULT ( STDMETHODCALLTYPE *VideoModePreferrence )( 
            IDvdControl * This,
            /* [in] */ ULONG ulPreferredDisplayMode);
        
        DECLSPEC_XFGVIRT(IDvdControl, SetRoot)
        HRESULT ( STDMETHODCALLTYPE *SetRoot )( 
            IDvdControl * This,
            /* [in] */ LPCWSTR pszPath);
        
        DECLSPEC_XFGVIRT(IDvdControl, MouseActivate)
        HRESULT ( STDMETHODCALLTYPE *MouseActivate )( 
            IDvdControl * This,
            /* [in] */ POINT point);
        
        DECLSPEC_XFGVIRT(IDvdControl, MouseSelect)
        HRESULT ( STDMETHODCALLTYPE *MouseSelect )( 
            IDvdControl * This,
            /* [in] */ POINT point);
        
        DECLSPEC_XFGVIRT(IDvdControl, ChapterPlayAutoStop)
        HRESULT ( STDMETHODCALLTYPE *ChapterPlayAutoStop )( 
            IDvdControl * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter,
            /* [in] */ ULONG ulChaptersToPlay);
        
        END_INTERFACE
    } IDvdControlVtbl;

    interface IDvdControl
    {
        CONST_VTBL struct IDvdControlVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdControl_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdControl_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdControl_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdControl_TitlePlay(This,ulTitle)	\
    ( (This)->lpVtbl -> TitlePlay(This,ulTitle) ) 

#define IDvdControl_ChapterPlay(This,ulTitle,ulChapter)	\
    ( (This)->lpVtbl -> ChapterPlay(This,ulTitle,ulChapter) ) 

#define IDvdControl_TimePlay(This,ulTitle,bcdTime)	\
    ( (This)->lpVtbl -> TimePlay(This,ulTitle,bcdTime) ) 

#define IDvdControl_StopForResume(This)	\
    ( (This)->lpVtbl -> StopForResume(This) ) 

#define IDvdControl_GoUp(This)	\
    ( (This)->lpVtbl -> GoUp(This) ) 

#define IDvdControl_TimeSearch(This,bcdTime)	\
    ( (This)->lpVtbl -> TimeSearch(This,bcdTime) ) 

#define IDvdControl_ChapterSearch(This,ulChapter)	\
    ( (This)->lpVtbl -> ChapterSearch(This,ulChapter) ) 

#define IDvdControl_PrevPGSearch(This)	\
    ( (This)->lpVtbl -> PrevPGSearch(This) ) 

#define IDvdControl_TopPGSearch(This)	\
    ( (This)->lpVtbl -> TopPGSearch(This) ) 

#define IDvdControl_NextPGSearch(This)	\
    ( (This)->lpVtbl -> NextPGSearch(This) ) 

#define IDvdControl_ForwardScan(This,dwSpeed)	\
    ( (This)->lpVtbl -> ForwardScan(This,dwSpeed) ) 

#define IDvdControl_BackwardScan(This,dwSpeed)	\
    ( (This)->lpVtbl -> BackwardScan(This,dwSpeed) ) 

#define IDvdControl_MenuCall(This,MenuID)	\
    ( (This)->lpVtbl -> MenuCall(This,MenuID) ) 

#define IDvdControl_Resume(This)	\
    ( (This)->lpVtbl -> Resume(This) ) 

#define IDvdControl_UpperButtonSelect(This)	\
    ( (This)->lpVtbl -> UpperButtonSelect(This) ) 

#define IDvdControl_LowerButtonSelect(This)	\
    ( (This)->lpVtbl -> LowerButtonSelect(This) ) 

#define IDvdControl_LeftButtonSelect(This)	\
    ( (This)->lpVtbl -> LeftButtonSelect(This) ) 

#define IDvdControl_RightButtonSelect(This)	\
    ( (This)->lpVtbl -> RightButtonSelect(This) ) 

#define IDvdControl_ButtonActivate(This)	\
    ( (This)->lpVtbl -> ButtonActivate(This) ) 

#define IDvdControl_ButtonSelectAndActivate(This,ulButton)	\
    ( (This)->lpVtbl -> ButtonSelectAndActivate(This,ulButton) ) 

#define IDvdControl_StillOff(This)	\
    ( (This)->lpVtbl -> StillOff(This) ) 

#define IDvdControl_PauseOn(This)	\
    ( (This)->lpVtbl -> PauseOn(This) ) 

#define IDvdControl_PauseOff(This)	\
    ( (This)->lpVtbl -> PauseOff(This) ) 

#define IDvdControl_MenuLanguageSelect(This,Language)	\
    ( (This)->lpVtbl -> MenuLanguageSelect(This,Language) ) 

#define IDvdControl_AudioStreamChange(This,ulAudio)	\
    ( (This)->lpVtbl -> AudioStreamChange(This,ulAudio) ) 

#define IDvdControl_SubpictureStreamChange(This,ulSubPicture,bDisplay)	\
    ( (This)->lpVtbl -> SubpictureStreamChange(This,ulSubPicture,bDisplay) ) 

#define IDvdControl_AngleChange(This,ulAngle)	\
    ( (This)->lpVtbl -> AngleChange(This,ulAngle) ) 

#define IDvdControl_ParentalLevelSelect(This,ulParentalLevel)	\
    ( (This)->lpVtbl -> ParentalLevelSelect(This,ulParentalLevel) ) 

#define IDvdControl_ParentalCountrySelect(This,wCountry)	\
    ( (This)->lpVtbl -> ParentalCountrySelect(This,wCountry) ) 

#define IDvdControl_KaraokeAudioPresentationModeChange(This,ulMode)	\
    ( (This)->lpVtbl -> KaraokeAudioPresentationModeChange(This,ulMode) ) 

#define IDvdControl_VideoModePreferrence(This,ulPreferredDisplayMode)	\
    ( (This)->lpVtbl -> VideoModePreferrence(This,ulPreferredDisplayMode) ) 

#define IDvdControl_SetRoot(This,pszPath)	\
    ( (This)->lpVtbl -> SetRoot(This,pszPath) ) 

#define IDvdControl_MouseActivate(This,point)	\
    ( (This)->lpVtbl -> MouseActivate(This,point) ) 

#define IDvdControl_MouseSelect(This,point)	\
    ( (This)->lpVtbl -> MouseSelect(This,point) ) 

#define IDvdControl_ChapterPlayAutoStop(This,ulTitle,ulChapter,ulChaptersToPlay)	\
    ( (This)->lpVtbl -> ChapterPlayAutoStop(This,ulTitle,ulChapter,ulChaptersToPlay) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdControl_INTERFACE_DEFINED__ */


#ifndef __IDvdInfo_INTERFACE_DEFINED__
#define __IDvdInfo_INTERFACE_DEFINED__

/* interface IDvdInfo */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvdInfo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("A70EFE60-E2A3-11d0-A9BE-00AA0061BE93")
    IDvdInfo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCurrentDomain( 
            /* [out] */ DVD_DOMAIN *pDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentLocation( 
            /* [annotation][out] */ 
            _Out_  DVD_PLAYBACK_LOCATION *pLocation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalTitleTime( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulTotalTime) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentButton( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulButtonsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentButton) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentAngle( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulAnglesAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentAngle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentAudio( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentSubpicture( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream,
            /* [annotation][out] */ 
            _Out_  BOOL *pIsDisabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentUOPS( 
            /* [annotation][out] */ 
            _Out_  VALID_UOP_SOMTHING_OR_OTHER *pUOP) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllSPRMs( 
            /* [annotation][out] */ 
            _Out_  SPRMARRAY *pRegisterArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllGPRMs( 
            /* [annotation][out] */ 
            _Out_  GPRMARRAY *pRegisterArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioLanguage( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubpictureLanguage( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTitleAttributes( 
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  DVD_ATR *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVMGAttributes( 
            /* [annotation][out] */ 
            _Out_  DVD_ATR *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentVideoAttributes( 
            /* [annotation][out] */ 
            _Out_  DVD_VideoATR *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentAudioAttributes( 
            /* [annotation][out] */ 
            _Out_  DVD_AudioATR *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentSubpictureAttributes( 
            /* [annotation][out] */ 
            _Out_  DVD_SubpictureATR *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentVolumeInfo( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfVol,
            /* [annotation][out] */ 
            _Out_  ULONG *pulThisVolNum,
            /* [annotation][out] */ 
            _Out_  DVD_DISC_SIDE *pSide,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfTitles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDTextInfo( 
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(ulBufSize, *pulActualSize)  BYTE *pTextManager,
            /* [in] */ ULONG ulBufSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlayerParentalLevel( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevel,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCountryCode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfChapters( 
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumberOfChapters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTitleParentalLevels( 
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetRoot( 
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(ulBufSize, *pulActualSize)  LPSTR pRoot,
            /* [in] */ ULONG ulBufSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdInfoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdInfo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdInfo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdInfo * This);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentDomain)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentDomain )( 
            IDvdInfo * This,
            /* [out] */ DVD_DOMAIN *pDomain);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentLocation)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentLocation )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  DVD_PLAYBACK_LOCATION *pLocation);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetTotalTitleTime)
        HRESULT ( STDMETHODCALLTYPE *GetTotalTitleTime )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulTotalTime);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentButton)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentButton )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulButtonsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentButton);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentAngle)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentAngle )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulAnglesAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentAngle);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentAudio)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentAudio )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentSubpicture)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSubpicture )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream,
            /* [annotation][out] */ 
            _Out_  BOOL *pIsDisabled);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentUOPS)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentUOPS )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  VALID_UOP_SOMTHING_OR_OTHER *pUOP);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetAllSPRMs)
        HRESULT ( STDMETHODCALLTYPE *GetAllSPRMs )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  SPRMARRAY *pRegisterArray);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetAllGPRMs)
        HRESULT ( STDMETHODCALLTYPE *GetAllGPRMs )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  GPRMARRAY *pRegisterArray);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetAudioLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetAudioLanguage )( 
            IDvdInfo * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetSubpictureLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetSubpictureLanguage )( 
            IDvdInfo * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetTitleAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetTitleAttributes )( 
            IDvdInfo * This,
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  DVD_ATR *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetVMGAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetVMGAttributes )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  DVD_ATR *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentVideoAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentVideoAttributes )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  DVD_VideoATR *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentAudioAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentAudioAttributes )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  DVD_AudioATR *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentSubpictureAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSubpictureAttributes )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  DVD_SubpictureATR *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetCurrentVolumeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentVolumeInfo )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfVol,
            /* [annotation][out] */ 
            _Out_  ULONG *pulThisVolNum,
            /* [annotation][out] */ 
            _Out_  DVD_DISC_SIDE *pSide,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfTitles);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetDVDTextInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDVDTextInfo )( 
            IDvdInfo * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_bytes_to_(ulBufSize, *pulActualSize)  BYTE *pTextManager,
            /* [in] */ ULONG ulBufSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetPlayerParentalLevel)
        HRESULT ( STDMETHODCALLTYPE *GetPlayerParentalLevel )( 
            IDvdInfo * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevel,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCountryCode);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetNumberOfChapters)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfChapters )( 
            IDvdInfo * This,
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumberOfChapters);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetTitleParentalLevels)
        HRESULT ( STDMETHODCALLTYPE *GetTitleParentalLevels )( 
            IDvdInfo * This,
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevels);
        
        DECLSPEC_XFGVIRT(IDvdInfo, GetRoot)
        HRESULT ( STDMETHODCALLTYPE *GetRoot )( 
            IDvdInfo * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(ulBufSize, *pulActualSize)  LPSTR pRoot,
            /* [in] */ ULONG ulBufSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize);
        
        END_INTERFACE
    } IDvdInfoVtbl;

    interface IDvdInfo
    {
        CONST_VTBL struct IDvdInfoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdInfo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdInfo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdInfo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdInfo_GetCurrentDomain(This,pDomain)	\
    ( (This)->lpVtbl -> GetCurrentDomain(This,pDomain) ) 

#define IDvdInfo_GetCurrentLocation(This,pLocation)	\
    ( (This)->lpVtbl -> GetCurrentLocation(This,pLocation) ) 

#define IDvdInfo_GetTotalTitleTime(This,pulTotalTime)	\
    ( (This)->lpVtbl -> GetTotalTitleTime(This,pulTotalTime) ) 

#define IDvdInfo_GetCurrentButton(This,pulButtonsAvailable,pulCurrentButton)	\
    ( (This)->lpVtbl -> GetCurrentButton(This,pulButtonsAvailable,pulCurrentButton) ) 

#define IDvdInfo_GetCurrentAngle(This,pulAnglesAvailable,pulCurrentAngle)	\
    ( (This)->lpVtbl -> GetCurrentAngle(This,pulAnglesAvailable,pulCurrentAngle) ) 

#define IDvdInfo_GetCurrentAudio(This,pulStreamsAvailable,pulCurrentStream)	\
    ( (This)->lpVtbl -> GetCurrentAudio(This,pulStreamsAvailable,pulCurrentStream) ) 

#define IDvdInfo_GetCurrentSubpicture(This,pulStreamsAvailable,pulCurrentStream,pIsDisabled)	\
    ( (This)->lpVtbl -> GetCurrentSubpicture(This,pulStreamsAvailable,pulCurrentStream,pIsDisabled) ) 

#define IDvdInfo_GetCurrentUOPS(This,pUOP)	\
    ( (This)->lpVtbl -> GetCurrentUOPS(This,pUOP) ) 

#define IDvdInfo_GetAllSPRMs(This,pRegisterArray)	\
    ( (This)->lpVtbl -> GetAllSPRMs(This,pRegisterArray) ) 

#define IDvdInfo_GetAllGPRMs(This,pRegisterArray)	\
    ( (This)->lpVtbl -> GetAllGPRMs(This,pRegisterArray) ) 

#define IDvdInfo_GetAudioLanguage(This,ulStream,pLanguage)	\
    ( (This)->lpVtbl -> GetAudioLanguage(This,ulStream,pLanguage) ) 

#define IDvdInfo_GetSubpictureLanguage(This,ulStream,pLanguage)	\
    ( (This)->lpVtbl -> GetSubpictureLanguage(This,ulStream,pLanguage) ) 

#define IDvdInfo_GetTitleAttributes(This,ulTitle,pATR)	\
    ( (This)->lpVtbl -> GetTitleAttributes(This,ulTitle,pATR) ) 

#define IDvdInfo_GetVMGAttributes(This,pATR)	\
    ( (This)->lpVtbl -> GetVMGAttributes(This,pATR) ) 

#define IDvdInfo_GetCurrentVideoAttributes(This,pATR)	\
    ( (This)->lpVtbl -> GetCurrentVideoAttributes(This,pATR) ) 

#define IDvdInfo_GetCurrentAudioAttributes(This,pATR)	\
    ( (This)->lpVtbl -> GetCurrentAudioAttributes(This,pATR) ) 

#define IDvdInfo_GetCurrentSubpictureAttributes(This,pATR)	\
    ( (This)->lpVtbl -> GetCurrentSubpictureAttributes(This,pATR) ) 

#define IDvdInfo_GetCurrentVolumeInfo(This,pulNumOfVol,pulThisVolNum,pSide,pulNumOfTitles)	\
    ( (This)->lpVtbl -> GetCurrentVolumeInfo(This,pulNumOfVol,pulThisVolNum,pSide,pulNumOfTitles) ) 

#define IDvdInfo_GetDVDTextInfo(This,pTextManager,ulBufSize,pulActualSize)	\
    ( (This)->lpVtbl -> GetDVDTextInfo(This,pTextManager,ulBufSize,pulActualSize) ) 

#define IDvdInfo_GetPlayerParentalLevel(This,pulParentalLevel,pulCountryCode)	\
    ( (This)->lpVtbl -> GetPlayerParentalLevel(This,pulParentalLevel,pulCountryCode) ) 

#define IDvdInfo_GetNumberOfChapters(This,ulTitle,pulNumberOfChapters)	\
    ( (This)->lpVtbl -> GetNumberOfChapters(This,ulTitle,pulNumberOfChapters) ) 

#define IDvdInfo_GetTitleParentalLevels(This,ulTitle,pulParentalLevels)	\
    ( (This)->lpVtbl -> GetTitleParentalLevels(This,ulTitle,pulParentalLevels) ) 

#define IDvdInfo_GetRoot(This,pRoot,ulBufSize,pulActualSize)	\
    ( (This)->lpVtbl -> GetRoot(This,pRoot,ulBufSize,pulActualSize) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdInfo_INTERFACE_DEFINED__ */


#ifndef __IDvdCmd_INTERFACE_DEFINED__
#define __IDvdCmd_INTERFACE_DEFINED__

/* interface IDvdCmd */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvdCmd;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("5a4a97e4-94ee-4a55-9751-74b5643aa27d")
    IDvdCmd : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE WaitForStart( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE WaitForEnd( void) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdCmdVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdCmd * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdCmd * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdCmd * This);
        
        DECLSPEC_XFGVIRT(IDvdCmd, WaitForStart)
        HRESULT ( STDMETHODCALLTYPE *WaitForStart )( 
            IDvdCmd * This);
        
        DECLSPEC_XFGVIRT(IDvdCmd, WaitForEnd)
        HRESULT ( STDMETHODCALLTYPE *WaitForEnd )( 
            IDvdCmd * This);
        
        END_INTERFACE
    } IDvdCmdVtbl;

    interface IDvdCmd
    {
        CONST_VTBL struct IDvdCmdVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdCmd_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdCmd_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdCmd_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdCmd_WaitForStart(This)	\
    ( (This)->lpVtbl -> WaitForStart(This) ) 

#define IDvdCmd_WaitForEnd(This)	\
    ( (This)->lpVtbl -> WaitForEnd(This) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdCmd_INTERFACE_DEFINED__ */


#ifndef __IDvdState_INTERFACE_DEFINED__
#define __IDvdState_INTERFACE_DEFINED__

/* interface IDvdState */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvdState;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("86303d6d-1c4a-4087-ab42-f711167048ef")
    IDvdState : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetDiscID( 
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullUniqueID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetParentalLevel( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevel) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdStateVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdState * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdState * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdState * This);
        
        DECLSPEC_XFGVIRT(IDvdState, GetDiscID)
        HRESULT ( STDMETHODCALLTYPE *GetDiscID )( 
            IDvdState * This,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullUniqueID);
        
        DECLSPEC_XFGVIRT(IDvdState, GetParentalLevel)
        HRESULT ( STDMETHODCALLTYPE *GetParentalLevel )( 
            IDvdState * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevel);
        
        END_INTERFACE
    } IDvdStateVtbl;

    interface IDvdState
    {
        CONST_VTBL struct IDvdStateVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdState_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdState_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdState_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdState_GetDiscID(This,pullUniqueID)	\
    ( (This)->lpVtbl -> GetDiscID(This,pullUniqueID) ) 

#define IDvdState_GetParentalLevel(This,pulParentalLevel)	\
    ( (This)->lpVtbl -> GetParentalLevel(This,pulParentalLevel) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdState_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0136 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
#pragma region Application Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP)


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0136_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0136_v0_0_s_ifspec;

#ifndef __IDvdControl2_INTERFACE_DEFINED__
#define __IDvdControl2_INTERFACE_DEFINED__

/* interface IDvdControl2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvdControl2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("33BC7430-EEC0-11D2-8201-00A0C9D74842")
    IDvdControl2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE PlayTitle( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayChapterInTitle( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayAtTimeInTitle( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ DVD_HMSF_TIMECODE *pStartTime,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Stop( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReturnFromSubmenu( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayAtTime( 
            /* [in] */ DVD_HMSF_TIMECODE *pTime,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayChapter( 
            /* [in] */ ULONG ulChapter,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayPrevChapter( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ReplayChapter( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayNextChapter( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayForwards( 
            /* [in] */ double dSpeed,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayBackwards( 
            /* [in] */ double dSpeed,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ShowMenu( 
            /* [in] */ DVD_MENU_ID MenuID,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Resume( 
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectRelativeButton( 
            DVD_RELATIVE_BUTTON buttonDir) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateButton( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectButton( 
            /* [in] */ ULONG ulButton) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectAndActivateButton( 
            /* [in] */ ULONG ulButton) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE StillOff( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE Pause( 
            /* [in] */ BOOL bState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectAudioStream( 
            /* [in] */ ULONG ulAudio,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectSubpictureStream( 
            /* [in] */ ULONG ulSubPicture,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetSubpictureState( 
            /* [in] */ BOOL bState,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectAngle( 
            /* [in] */ ULONG ulAngle,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectParentalLevel( 
            /* [in] */ ULONG ulParentalLevel) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectParentalCountry( 
            /* [in] */ BYTE bCountry[ 2 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectKaraokeAudioPresentationMode( 
            /* [in] */ ULONG ulMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectVideoModePreference( 
            /* [in] */ ULONG ulPreferredDisplayMode) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDVDDirectory( 
            /* [in] */ LPCWSTR pszwPath) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE ActivateAtPosition( 
            /* [in] */ POINT point) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectAtPosition( 
            /* [in] */ POINT point) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayChaptersAutoStop( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter,
            /* [in] */ ULONG ulChaptersToPlay,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE AcceptParentalLevelChange( 
            /* [in] */ BOOL bAccept) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetOption( 
            /* [in] */ DVD_OPTION_FLAG flag,
            /* [in] */ BOOL fState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetState( 
            /* [in] */ IDvdState *pState,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE PlayPeriodInTitleAutoStop( 
            /* [in] */ ULONG ulTitle,
            /* [in] */ DVD_HMSF_TIMECODE *pStartTime,
            /* [in] */ DVD_HMSF_TIMECODE *pEndTime,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetGPRM( 
            /* [in] */ ULONG ulIndex,
            /* [in] */ WORD wValue,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectDefaultMenuLanguage( 
            /* [in] */ LCID Language) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectDefaultAudioLanguage( 
            /* [in] */ LCID Language,
            /* [in] */ DVD_AUDIO_LANG_EXT audioExtension) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SelectDefaultSubpictureLanguage( 
            /* [in] */ LCID Language,
            /* [in] */ DVD_SUBPICTURE_LANG_EXT subpictureExtension) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdControl2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdControl2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdControl2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdControl2 * This);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayTitle)
        HRESULT ( STDMETHODCALLTYPE *PlayTitle )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayChapterInTitle)
        HRESULT ( STDMETHODCALLTYPE *PlayChapterInTitle )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayAtTimeInTitle)
        HRESULT ( STDMETHODCALLTYPE *PlayAtTimeInTitle )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ DVD_HMSF_TIMECODE *pStartTime,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, Stop)
        HRESULT ( STDMETHODCALLTYPE *Stop )( 
            IDvdControl2 * This);
        
        DECLSPEC_XFGVIRT(IDvdControl2, ReturnFromSubmenu)
        HRESULT ( STDMETHODCALLTYPE *ReturnFromSubmenu )( 
            IDvdControl2 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayAtTime)
        HRESULT ( STDMETHODCALLTYPE *PlayAtTime )( 
            IDvdControl2 * This,
            /* [in] */ DVD_HMSF_TIMECODE *pTime,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayChapter)
        HRESULT ( STDMETHODCALLTYPE *PlayChapter )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulChapter,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayPrevChapter)
        HRESULT ( STDMETHODCALLTYPE *PlayPrevChapter )( 
            IDvdControl2 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, ReplayChapter)
        HRESULT ( STDMETHODCALLTYPE *ReplayChapter )( 
            IDvdControl2 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayNextChapter)
        HRESULT ( STDMETHODCALLTYPE *PlayNextChapter )( 
            IDvdControl2 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayForwards)
        HRESULT ( STDMETHODCALLTYPE *PlayForwards )( 
            IDvdControl2 * This,
            /* [in] */ double dSpeed,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayBackwards)
        HRESULT ( STDMETHODCALLTYPE *PlayBackwards )( 
            IDvdControl2 * This,
            /* [in] */ double dSpeed,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, ShowMenu)
        HRESULT ( STDMETHODCALLTYPE *ShowMenu )( 
            IDvdControl2 * This,
            /* [in] */ DVD_MENU_ID MenuID,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, Resume)
        HRESULT ( STDMETHODCALLTYPE *Resume )( 
            IDvdControl2 * This,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectRelativeButton)
        HRESULT ( STDMETHODCALLTYPE *SelectRelativeButton )( 
            IDvdControl2 * This,
            DVD_RELATIVE_BUTTON buttonDir);
        
        DECLSPEC_XFGVIRT(IDvdControl2, ActivateButton)
        HRESULT ( STDMETHODCALLTYPE *ActivateButton )( 
            IDvdControl2 * This);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectButton)
        HRESULT ( STDMETHODCALLTYPE *SelectButton )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulButton);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectAndActivateButton)
        HRESULT ( STDMETHODCALLTYPE *SelectAndActivateButton )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulButton);
        
        DECLSPEC_XFGVIRT(IDvdControl2, StillOff)
        HRESULT ( STDMETHODCALLTYPE *StillOff )( 
            IDvdControl2 * This);
        
        DECLSPEC_XFGVIRT(IDvdControl2, Pause)
        HRESULT ( STDMETHODCALLTYPE *Pause )( 
            IDvdControl2 * This,
            /* [in] */ BOOL bState);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectAudioStream)
        HRESULT ( STDMETHODCALLTYPE *SelectAudioStream )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulAudio,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectSubpictureStream)
        HRESULT ( STDMETHODCALLTYPE *SelectSubpictureStream )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulSubPicture,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SetSubpictureState)
        HRESULT ( STDMETHODCALLTYPE *SetSubpictureState )( 
            IDvdControl2 * This,
            /* [in] */ BOOL bState,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectAngle)
        HRESULT ( STDMETHODCALLTYPE *SelectAngle )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulAngle,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectParentalLevel)
        HRESULT ( STDMETHODCALLTYPE *SelectParentalLevel )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulParentalLevel);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectParentalCountry)
        HRESULT ( STDMETHODCALLTYPE *SelectParentalCountry )( 
            IDvdControl2 * This,
            /* [in] */ BYTE bCountry[ 2 ]);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectKaraokeAudioPresentationMode)
        HRESULT ( STDMETHODCALLTYPE *SelectKaraokeAudioPresentationMode )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulMode);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectVideoModePreference)
        HRESULT ( STDMETHODCALLTYPE *SelectVideoModePreference )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulPreferredDisplayMode);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SetDVDDirectory)
        HRESULT ( STDMETHODCALLTYPE *SetDVDDirectory )( 
            IDvdControl2 * This,
            /* [in] */ LPCWSTR pszwPath);
        
        DECLSPEC_XFGVIRT(IDvdControl2, ActivateAtPosition)
        HRESULT ( STDMETHODCALLTYPE *ActivateAtPosition )( 
            IDvdControl2 * This,
            /* [in] */ POINT point);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectAtPosition)
        HRESULT ( STDMETHODCALLTYPE *SelectAtPosition )( 
            IDvdControl2 * This,
            /* [in] */ POINT point);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayChaptersAutoStop)
        HRESULT ( STDMETHODCALLTYPE *PlayChaptersAutoStop )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ ULONG ulChapter,
            /* [in] */ ULONG ulChaptersToPlay,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, AcceptParentalLevelChange)
        HRESULT ( STDMETHODCALLTYPE *AcceptParentalLevelChange )( 
            IDvdControl2 * This,
            /* [in] */ BOOL bAccept);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SetOption)
        HRESULT ( STDMETHODCALLTYPE *SetOption )( 
            IDvdControl2 * This,
            /* [in] */ DVD_OPTION_FLAG flag,
            /* [in] */ BOOL fState);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SetState)
        HRESULT ( STDMETHODCALLTYPE *SetState )( 
            IDvdControl2 * This,
            /* [in] */ IDvdState *pState,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, PlayPeriodInTitleAutoStop)
        HRESULT ( STDMETHODCALLTYPE *PlayPeriodInTitleAutoStop )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulTitle,
            /* [in] */ DVD_HMSF_TIMECODE *pStartTime,
            /* [in] */ DVD_HMSF_TIMECODE *pEndTime,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SetGPRM)
        HRESULT ( STDMETHODCALLTYPE *SetGPRM )( 
            IDvdControl2 * This,
            /* [in] */ ULONG ulIndex,
            /* [in] */ WORD wValue,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **ppCmd);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectDefaultMenuLanguage)
        HRESULT ( STDMETHODCALLTYPE *SelectDefaultMenuLanguage )( 
            IDvdControl2 * This,
            /* [in] */ LCID Language);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectDefaultAudioLanguage)
        HRESULT ( STDMETHODCALLTYPE *SelectDefaultAudioLanguage )( 
            IDvdControl2 * This,
            /* [in] */ LCID Language,
            /* [in] */ DVD_AUDIO_LANG_EXT audioExtension);
        
        DECLSPEC_XFGVIRT(IDvdControl2, SelectDefaultSubpictureLanguage)
        HRESULT ( STDMETHODCALLTYPE *SelectDefaultSubpictureLanguage )( 
            IDvdControl2 * This,
            /* [in] */ LCID Language,
            /* [in] */ DVD_SUBPICTURE_LANG_EXT subpictureExtension);
        
        END_INTERFACE
    } IDvdControl2Vtbl;

    interface IDvdControl2
    {
        CONST_VTBL struct IDvdControl2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdControl2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdControl2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdControl2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdControl2_PlayTitle(This,ulTitle,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayTitle(This,ulTitle,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayChapterInTitle(This,ulTitle,ulChapter,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayChapterInTitle(This,ulTitle,ulChapter,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayAtTimeInTitle(This,ulTitle,pStartTime,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayAtTimeInTitle(This,ulTitle,pStartTime,dwFlags,ppCmd) ) 

#define IDvdControl2_Stop(This)	\
    ( (This)->lpVtbl -> Stop(This) ) 

#define IDvdControl2_ReturnFromSubmenu(This,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> ReturnFromSubmenu(This,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayAtTime(This,pTime,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayAtTime(This,pTime,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayChapter(This,ulChapter,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayChapter(This,ulChapter,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayPrevChapter(This,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayPrevChapter(This,dwFlags,ppCmd) ) 

#define IDvdControl2_ReplayChapter(This,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> ReplayChapter(This,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayNextChapter(This,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayNextChapter(This,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayForwards(This,dSpeed,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayForwards(This,dSpeed,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayBackwards(This,dSpeed,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayBackwards(This,dSpeed,dwFlags,ppCmd) ) 

#define IDvdControl2_ShowMenu(This,MenuID,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> ShowMenu(This,MenuID,dwFlags,ppCmd) ) 

#define IDvdControl2_Resume(This,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> Resume(This,dwFlags,ppCmd) ) 

#define IDvdControl2_SelectRelativeButton(This,buttonDir)	\
    ( (This)->lpVtbl -> SelectRelativeButton(This,buttonDir) ) 

#define IDvdControl2_ActivateButton(This)	\
    ( (This)->lpVtbl -> ActivateButton(This) ) 

#define IDvdControl2_SelectButton(This,ulButton)	\
    ( (This)->lpVtbl -> SelectButton(This,ulButton) ) 

#define IDvdControl2_SelectAndActivateButton(This,ulButton)	\
    ( (This)->lpVtbl -> SelectAndActivateButton(This,ulButton) ) 

#define IDvdControl2_StillOff(This)	\
    ( (This)->lpVtbl -> StillOff(This) ) 

#define IDvdControl2_Pause(This,bState)	\
    ( (This)->lpVtbl -> Pause(This,bState) ) 

#define IDvdControl2_SelectAudioStream(This,ulAudio,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> SelectAudioStream(This,ulAudio,dwFlags,ppCmd) ) 

#define IDvdControl2_SelectSubpictureStream(This,ulSubPicture,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> SelectSubpictureStream(This,ulSubPicture,dwFlags,ppCmd) ) 

#define IDvdControl2_SetSubpictureState(This,bState,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> SetSubpictureState(This,bState,dwFlags,ppCmd) ) 

#define IDvdControl2_SelectAngle(This,ulAngle,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> SelectAngle(This,ulAngle,dwFlags,ppCmd) ) 

#define IDvdControl2_SelectParentalLevel(This,ulParentalLevel)	\
    ( (This)->lpVtbl -> SelectParentalLevel(This,ulParentalLevel) ) 

#define IDvdControl2_SelectParentalCountry(This,bCountry)	\
    ( (This)->lpVtbl -> SelectParentalCountry(This,bCountry) ) 

#define IDvdControl2_SelectKaraokeAudioPresentationMode(This,ulMode)	\
    ( (This)->lpVtbl -> SelectKaraokeAudioPresentationMode(This,ulMode) ) 

#define IDvdControl2_SelectVideoModePreference(This,ulPreferredDisplayMode)	\
    ( (This)->lpVtbl -> SelectVideoModePreference(This,ulPreferredDisplayMode) ) 

#define IDvdControl2_SetDVDDirectory(This,pszwPath)	\
    ( (This)->lpVtbl -> SetDVDDirectory(This,pszwPath) ) 

#define IDvdControl2_ActivateAtPosition(This,point)	\
    ( (This)->lpVtbl -> ActivateAtPosition(This,point) ) 

#define IDvdControl2_SelectAtPosition(This,point)	\
    ( (This)->lpVtbl -> SelectAtPosition(This,point) ) 

#define IDvdControl2_PlayChaptersAutoStop(This,ulTitle,ulChapter,ulChaptersToPlay,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayChaptersAutoStop(This,ulTitle,ulChapter,ulChaptersToPlay,dwFlags,ppCmd) ) 

#define IDvdControl2_AcceptParentalLevelChange(This,bAccept)	\
    ( (This)->lpVtbl -> AcceptParentalLevelChange(This,bAccept) ) 

#define IDvdControl2_SetOption(This,flag,fState)	\
    ( (This)->lpVtbl -> SetOption(This,flag,fState) ) 

#define IDvdControl2_SetState(This,pState,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> SetState(This,pState,dwFlags,ppCmd) ) 

#define IDvdControl2_PlayPeriodInTitleAutoStop(This,ulTitle,pStartTime,pEndTime,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> PlayPeriodInTitleAutoStop(This,ulTitle,pStartTime,pEndTime,dwFlags,ppCmd) ) 

#define IDvdControl2_SetGPRM(This,ulIndex,wValue,dwFlags,ppCmd)	\
    ( (This)->lpVtbl -> SetGPRM(This,ulIndex,wValue,dwFlags,ppCmd) ) 

#define IDvdControl2_SelectDefaultMenuLanguage(This,Language)	\
    ( (This)->lpVtbl -> SelectDefaultMenuLanguage(This,Language) ) 

#define IDvdControl2_SelectDefaultAudioLanguage(This,Language,audioExtension)	\
    ( (This)->lpVtbl -> SelectDefaultAudioLanguage(This,Language,audioExtension) ) 

#define IDvdControl2_SelectDefaultSubpictureLanguage(This,Language,subpictureExtension)	\
    ( (This)->lpVtbl -> SelectDefaultSubpictureLanguage(This,Language,subpictureExtension) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdControl2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0137 */
/* [local] */ 


enum DVD_TextStringType
    {
        DVD_Struct_Volume	= 0x1,
        DVD_Struct_Title	= 0x2,
        DVD_Struct_ParentalID	= 0x3,
        DVD_Struct_PartOfTitle	= 0x4,
        DVD_Struct_Cell	= 0x5,
        DVD_Stream_Audio	= 0x10,
        DVD_Stream_Subpicture	= 0x11,
        DVD_Stream_Angle	= 0x12,
        DVD_Channel_Audio	= 0x20,
        DVD_General_Name	= 0x30,
        DVD_General_Comments	= 0x31,
        DVD_Title_Series	= 0x38,
        DVD_Title_Movie	= 0x39,
        DVD_Title_Video	= 0x3a,
        DVD_Title_Album	= 0x3b,
        DVD_Title_Song	= 0x3c,
        DVD_Title_Other	= 0x3f,
        DVD_Title_Sub_Series	= 0x40,
        DVD_Title_Sub_Movie	= 0x41,
        DVD_Title_Sub_Video	= 0x42,
        DVD_Title_Sub_Album	= 0x43,
        DVD_Title_Sub_Song	= 0x44,
        DVD_Title_Sub_Other	= 0x47,
        DVD_Title_Orig_Series	= 0x48,
        DVD_Title_Orig_Movie	= 0x49,
        DVD_Title_Orig_Video	= 0x4a,
        DVD_Title_Orig_Album	= 0x4b,
        DVD_Title_Orig_Song	= 0x4c,
        DVD_Title_Orig_Other	= 0x4f,
        DVD_Other_Scene	= 0x50,
        DVD_Other_Cut	= 0x51,
        DVD_Other_Take	= 0x52
    } ;

enum DVD_TextCharSet
    {
        DVD_CharSet_Unicode	= 0,
        DVD_CharSet_ISO646	= 1,
        DVD_CharSet_JIS_Roman_Kanji	= 2,
        DVD_CharSet_ISO8859_1	= 3,
        DVD_CharSet_ShiftJIS_Kanji_Roman_Katakana	= 4
    } ;
#define DVD_TITLE_MENU				0x000
#define DVD_STREAM_DATA_CURRENT     0x800
#define DVD_STREAM_DATA_VMGM        0x400
#define DVD_STREAM_DATA_VTSM        0x401
#define DVD_DEFAULT_AUDIO_STREAM	0x0f
typedef struct tagDVD_DECODER_CAPS
    {
    DWORD dwSize;
    DWORD dwAudioCaps;
    double dFwdMaxRateVideo;
    double dFwdMaxRateAudio;
    double dFwdMaxRateSP;
    double dBwdMaxRateVideo;
    double dBwdMaxRateAudio;
    double dBwdMaxRateSP;
    DWORD dwRes1;
    DWORD dwRes2;
    DWORD dwRes3;
    DWORD dwRes4;
    } 	DVD_DECODER_CAPS;

#define DVD_AUDIO_CAPS_AC3		0x00000001
#define DVD_AUDIO_CAPS_MPEG2	0x00000002
#define DVD_AUDIO_CAPS_LPCM		0x00000004
#define DVD_AUDIO_CAPS_DTS		0x00000008
#define DVD_AUDIO_CAPS_SDDS		0x00000010


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0137_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0137_v0_0_s_ifspec;

#ifndef __IDvdInfo2_INTERFACE_DEFINED__
#define __IDvdInfo2_INTERFACE_DEFINED__

/* interface IDvdInfo2 */
/* [unique][uuid][object][local] */ 


EXTERN_C const IID IID_IDvdInfo2;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("34151510-EEC0-11D2-8201-00A0C9D74842")
    IDvdInfo2 : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetCurrentDomain( 
            /* [annotation][out] */ 
            _Out_  DVD_DOMAIN *pDomain) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentLocation( 
            /* [annotation][out] */ 
            _Out_  DVD_PLAYBACK_LOCATION2 *pLocation) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTotalTitleTime( 
            /* [annotation][out] */ 
            _Out_  DVD_HMSF_TIMECODE *pTotalTime,
            /* [annotation][out] */ 
            _Out_  ULONG *ulTimeCodeFlags) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentButton( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulButtonsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentButton) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentAngle( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulAnglesAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentAngle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentAudio( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentSubpicture( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream,
            /* [annotation][out] */ 
            _Out_  BOOL *pbIsDisabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentUOPS( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulUOPs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllSPRMs( 
            /* [annotation][out] */ 
            _Out_  SPRMARRAY *pRegisterArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAllGPRMs( 
            /* [annotation][out] */ 
            _Out_  GPRMARRAY *pRegisterArray) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioLanguage( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubpictureLanguage( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTitleAttributes( 
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  DVD_MenuAttributes *pMenu,
            /* [annotation][out] */ 
            _Out_  DVD_TitleAttributes *pTitle) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetVMGAttributes( 
            /* [annotation][out] */ 
            _Out_  DVD_MenuAttributes *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCurrentVideoAttributes( 
            /* [annotation][out] */ 
            _Out_  DVD_VideoAttributes *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetAudioAttributes( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  DVD_AudioAttributes *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetKaraokeAttributes( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  DVD_KaraokeAttributes *pAttributes) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetSubpictureAttributes( 
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  DVD_SubpictureAttributes *pATR) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDVolumeInfo( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfVolumes,
            /* [annotation][out] */ 
            _Out_  ULONG *pulVolume,
            /* [annotation][out] */ 
            _Out_  DVD_DISC_SIDE *pSide,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfTitles) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDTextNumberOfLanguages( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfLangs) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDTextLanguageInfo( 
            /* [in] */ ULONG ulLangIndex,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfStrings,
            /* [annotation][out] */ 
            _Out_  LCID *pLangCode,
            /* [annotation][out] */ 
            _Out_  enum DVD_TextCharSet *pbCharacterSet) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDTextStringAsNative( 
            /* [in] */ ULONG ulLangIndex,
            /* [in] */ ULONG ulStringIndex,
            /* [annotation][out] */ 
            _Out_  BYTE *pbBuffer,
            /* [in] */ ULONG ulMaxBufferSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize,
            /* [annotation][out] */ 
            _Out_  enum DVD_TextStringType *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDTextStringAsUnicode( 
            /* [in] */ ULONG ulLangIndex,
            /* [in] */ ULONG ulStringIndex,
            /* [annotation][out] */ 
            _Out_  WCHAR *pchwBuffer,
            /* [in] */ ULONG ulMaxBufferSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize,
            /* [annotation][out] */ 
            _Out_  enum DVD_TextStringType *pType) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPlayerParentalLevel( 
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevel,
            /* [annotation][out] */ 
            _Out_  BYTE pbCountryCode[ 2 ]) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNumberOfChapters( 
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfChapters) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetTitleParentalLevels( 
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevels) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDVDDirectory( 
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(ulMaxSize, *pulActualSize)  LPWSTR pszwPath,
            /* [in] */ ULONG ulMaxSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsAudioStreamEnabled( 
            /* [in] */ ULONG ulStreamNum,
            /* [annotation][out] */ 
            _Out_  BOOL *pbEnabled) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDiscID( 
            /* [in] */ LPCWSTR pszwPath,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullDiscID) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetState( 
            /* [annotation][out] */ 
            _Out_  IDvdState **pStateData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetMenuLanguages( 
            /* [annotation][out] */ 
            _Out_  LCID *pLanguages,
            /* [in] */ ULONG ulMaxLanguages,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualLanguages) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetButtonAtPosition( 
            /* [in] */ POINT point,
            /* [annotation][out] */ 
            _Out_  ULONG *pulButtonIndex) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetCmdFromEvent( 
            /* [in] */ LONG_PTR lParam1,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **pCmdObj) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultMenuLanguage( 
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultAudioLanguage( 
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage,
            /* [annotation][out] */ 
            _Out_  DVD_AUDIO_LANG_EXT *pAudioExtension) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDefaultSubpictureLanguage( 
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage,
            /* [annotation][out] */ 
            _Out_  DVD_SUBPICTURE_LANG_EXT *pSubpictureExtension) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDecoderCaps( 
            /* [annotation][out] */ 
            _Out_  DVD_DECODER_CAPS *pCaps) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetButtonRect( 
            /* [in] */ ULONG ulButton,
            /* [annotation][out] */ 
            _Out_  RECT *pRect) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE IsSubpictureStreamEnabled( 
            /* [in] */ ULONG ulStreamNum,
            /* [annotation][out] */ 
            _Out_  BOOL *pbEnabled) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdInfo2Vtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdInfo2 * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdInfo2 * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdInfo2 * This);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentDomain)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentDomain )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  DVD_DOMAIN *pDomain);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentLocation)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentLocation )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  DVD_PLAYBACK_LOCATION2 *pLocation);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetTotalTitleTime)
        HRESULT ( STDMETHODCALLTYPE *GetTotalTitleTime )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  DVD_HMSF_TIMECODE *pTotalTime,
            /* [annotation][out] */ 
            _Out_  ULONG *ulTimeCodeFlags);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentButton)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentButton )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulButtonsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentButton);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentAngle)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentAngle )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulAnglesAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentAngle);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentAudio)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentAudio )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentSubpicture)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentSubpicture )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulStreamsAvailable,
            /* [annotation][out] */ 
            _Out_  ULONG *pulCurrentStream,
            /* [annotation][out] */ 
            _Out_  BOOL *pbIsDisabled);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentUOPS)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentUOPS )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulUOPs);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetAllSPRMs)
        HRESULT ( STDMETHODCALLTYPE *GetAllSPRMs )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  SPRMARRAY *pRegisterArray);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetAllGPRMs)
        HRESULT ( STDMETHODCALLTYPE *GetAllGPRMs )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  GPRMARRAY *pRegisterArray);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetAudioLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetAudioLanguage )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetSubpictureLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetSubpictureLanguage )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetTitleAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetTitleAttributes )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  DVD_MenuAttributes *pMenu,
            /* [annotation][out] */ 
            _Out_  DVD_TitleAttributes *pTitle);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetVMGAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetVMGAttributes )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  DVD_MenuAttributes *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCurrentVideoAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetCurrentVideoAttributes )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  DVD_VideoAttributes *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetAudioAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetAudioAttributes )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  DVD_AudioAttributes *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetKaraokeAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetKaraokeAttributes )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  DVD_KaraokeAttributes *pAttributes);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetSubpictureAttributes)
        HRESULT ( STDMETHODCALLTYPE *GetSubpictureAttributes )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStream,
            /* [annotation][out] */ 
            _Out_  DVD_SubpictureAttributes *pATR);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDVDVolumeInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDVDVolumeInfo )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfVolumes,
            /* [annotation][out] */ 
            _Out_  ULONG *pulVolume,
            /* [annotation][out] */ 
            _Out_  DVD_DISC_SIDE *pSide,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfTitles);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDVDTextNumberOfLanguages)
        HRESULT ( STDMETHODCALLTYPE *GetDVDTextNumberOfLanguages )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfLangs);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDVDTextLanguageInfo)
        HRESULT ( STDMETHODCALLTYPE *GetDVDTextLanguageInfo )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulLangIndex,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfStrings,
            /* [annotation][out] */ 
            _Out_  LCID *pLangCode,
            /* [annotation][out] */ 
            _Out_  enum DVD_TextCharSet *pbCharacterSet);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDVDTextStringAsNative)
        HRESULT ( STDMETHODCALLTYPE *GetDVDTextStringAsNative )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulLangIndex,
            /* [in] */ ULONG ulStringIndex,
            /* [annotation][out] */ 
            _Out_  BYTE *pbBuffer,
            /* [in] */ ULONG ulMaxBufferSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize,
            /* [annotation][out] */ 
            _Out_  enum DVD_TextStringType *pType);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDVDTextStringAsUnicode)
        HRESULT ( STDMETHODCALLTYPE *GetDVDTextStringAsUnicode )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulLangIndex,
            /* [in] */ ULONG ulStringIndex,
            /* [annotation][out] */ 
            _Out_  WCHAR *pchwBuffer,
            /* [in] */ ULONG ulMaxBufferSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize,
            /* [annotation][out] */ 
            _Out_  enum DVD_TextStringType *pType);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetPlayerParentalLevel)
        HRESULT ( STDMETHODCALLTYPE *GetPlayerParentalLevel )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevel,
            /* [annotation][out] */ 
            _Out_  BYTE pbCountryCode[ 2 ]);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetNumberOfChapters)
        HRESULT ( STDMETHODCALLTYPE *GetNumberOfChapters )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulNumOfChapters);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetTitleParentalLevels)
        HRESULT ( STDMETHODCALLTYPE *GetTitleParentalLevels )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulTitle,
            /* [annotation][out] */ 
            _Out_  ULONG *pulParentalLevels);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDVDDirectory)
        HRESULT ( STDMETHODCALLTYPE *GetDVDDirectory )( 
            IDvdInfo2 * This,
            /* [annotation][size_is][out] */ 
            _Out_writes_to_(ulMaxSize, *pulActualSize)  LPWSTR pszwPath,
            /* [in] */ ULONG ulMaxSize,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualSize);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, IsAudioStreamEnabled)
        HRESULT ( STDMETHODCALLTYPE *IsAudioStreamEnabled )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStreamNum,
            /* [annotation][out] */ 
            _Out_  BOOL *pbEnabled);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDiscID)
        HRESULT ( STDMETHODCALLTYPE *GetDiscID )( 
            IDvdInfo2 * This,
            /* [in] */ LPCWSTR pszwPath,
            /* [annotation][out] */ 
            _Out_  ULONGLONG *pullDiscID);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetState)
        HRESULT ( STDMETHODCALLTYPE *GetState )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  IDvdState **pStateData);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetMenuLanguages)
        HRESULT ( STDMETHODCALLTYPE *GetMenuLanguages )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguages,
            /* [in] */ ULONG ulMaxLanguages,
            /* [annotation][out] */ 
            _Out_  ULONG *pulActualLanguages);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetButtonAtPosition)
        HRESULT ( STDMETHODCALLTYPE *GetButtonAtPosition )( 
            IDvdInfo2 * This,
            /* [in] */ POINT point,
            /* [annotation][out] */ 
            _Out_  ULONG *pulButtonIndex);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetCmdFromEvent)
        HRESULT ( STDMETHODCALLTYPE *GetCmdFromEvent )( 
            IDvdInfo2 * This,
            /* [in] */ LONG_PTR lParam1,
            /* [annotation][out] */ 
            _Out_  IDvdCmd **pCmdObj);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDefaultMenuLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultMenuLanguage )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDefaultAudioLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultAudioLanguage )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage,
            /* [annotation][out] */ 
            _Out_  DVD_AUDIO_LANG_EXT *pAudioExtension);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDefaultSubpictureLanguage)
        HRESULT ( STDMETHODCALLTYPE *GetDefaultSubpictureLanguage )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  LCID *pLanguage,
            /* [annotation][out] */ 
            _Out_  DVD_SUBPICTURE_LANG_EXT *pSubpictureExtension);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetDecoderCaps)
        HRESULT ( STDMETHODCALLTYPE *GetDecoderCaps )( 
            IDvdInfo2 * This,
            /* [annotation][out] */ 
            _Out_  DVD_DECODER_CAPS *pCaps);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, GetButtonRect)
        HRESULT ( STDMETHODCALLTYPE *GetButtonRect )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulButton,
            /* [annotation][out] */ 
            _Out_  RECT *pRect);
        
        DECLSPEC_XFGVIRT(IDvdInfo2, IsSubpictureStreamEnabled)
        HRESULT ( STDMETHODCALLTYPE *IsSubpictureStreamEnabled )( 
            IDvdInfo2 * This,
            /* [in] */ ULONG ulStreamNum,
            /* [annotation][out] */ 
            _Out_  BOOL *pbEnabled);
        
        END_INTERFACE
    } IDvdInfo2Vtbl;

    interface IDvdInfo2
    {
        CONST_VTBL struct IDvdInfo2Vtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdInfo2_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdInfo2_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdInfo2_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdInfo2_GetCurrentDomain(This,pDomain)	\
    ( (This)->lpVtbl -> GetCurrentDomain(This,pDomain) ) 

#define IDvdInfo2_GetCurrentLocation(This,pLocation)	\
    ( (This)->lpVtbl -> GetCurrentLocation(This,pLocation) ) 

#define IDvdInfo2_GetTotalTitleTime(This,pTotalTime,ulTimeCodeFlags)	\
    ( (This)->lpVtbl -> GetTotalTitleTime(This,pTotalTime,ulTimeCodeFlags) ) 

#define IDvdInfo2_GetCurrentButton(This,pulButtonsAvailable,pulCurrentButton)	\
    ( (This)->lpVtbl -> GetCurrentButton(This,pulButtonsAvailable,pulCurrentButton) ) 

#define IDvdInfo2_GetCurrentAngle(This,pulAnglesAvailable,pulCurrentAngle)	\
    ( (This)->lpVtbl -> GetCurrentAngle(This,pulAnglesAvailable,pulCurrentAngle) ) 

#define IDvdInfo2_GetCurrentAudio(This,pulStreamsAvailable,pulCurrentStream)	\
    ( (This)->lpVtbl -> GetCurrentAudio(This,pulStreamsAvailable,pulCurrentStream) ) 

#define IDvdInfo2_GetCurrentSubpicture(This,pulStreamsAvailable,pulCurrentStream,pbIsDisabled)	\
    ( (This)->lpVtbl -> GetCurrentSubpicture(This,pulStreamsAvailable,pulCurrentStream,pbIsDisabled) ) 

#define IDvdInfo2_GetCurrentUOPS(This,pulUOPs)	\
    ( (This)->lpVtbl -> GetCurrentUOPS(This,pulUOPs) ) 

#define IDvdInfo2_GetAllSPRMs(This,pRegisterArray)	\
    ( (This)->lpVtbl -> GetAllSPRMs(This,pRegisterArray) ) 

#define IDvdInfo2_GetAllGPRMs(This,pRegisterArray)	\
    ( (This)->lpVtbl -> GetAllGPRMs(This,pRegisterArray) ) 

#define IDvdInfo2_GetAudioLanguage(This,ulStream,pLanguage)	\
    ( (This)->lpVtbl -> GetAudioLanguage(This,ulStream,pLanguage) ) 

#define IDvdInfo2_GetSubpictureLanguage(This,ulStream,pLanguage)	\
    ( (This)->lpVtbl -> GetSubpictureLanguage(This,ulStream,pLanguage) ) 

#define IDvdInfo2_GetTitleAttributes(This,ulTitle,pMenu,pTitle)	\
    ( (This)->lpVtbl -> GetTitleAttributes(This,ulTitle,pMenu,pTitle) ) 

#define IDvdInfo2_GetVMGAttributes(This,pATR)	\
    ( (This)->lpVtbl -> GetVMGAttributes(This,pATR) ) 

#define IDvdInfo2_GetCurrentVideoAttributes(This,pATR)	\
    ( (This)->lpVtbl -> GetCurrentVideoAttributes(This,pATR) ) 

#define IDvdInfo2_GetAudioAttributes(This,ulStream,pATR)	\
    ( (This)->lpVtbl -> GetAudioAttributes(This,ulStream,pATR) ) 

#define IDvdInfo2_GetKaraokeAttributes(This,ulStream,pAttributes)	\
    ( (This)->lpVtbl -> GetKaraokeAttributes(This,ulStream,pAttributes) ) 

#define IDvdInfo2_GetSubpictureAttributes(This,ulStream,pATR)	\
    ( (This)->lpVtbl -> GetSubpictureAttributes(This,ulStream,pATR) ) 

#define IDvdInfo2_GetDVDVolumeInfo(This,pulNumOfVolumes,pulVolume,pSide,pulNumOfTitles)	\
    ( (This)->lpVtbl -> GetDVDVolumeInfo(This,pulNumOfVolumes,pulVolume,pSide,pulNumOfTitles) ) 

#define IDvdInfo2_GetDVDTextNumberOfLanguages(This,pulNumOfLangs)	\
    ( (This)->lpVtbl -> GetDVDTextNumberOfLanguages(This,pulNumOfLangs) ) 

#define IDvdInfo2_GetDVDTextLanguageInfo(This,ulLangIndex,pulNumOfStrings,pLangCode,pbCharacterSet)	\
    ( (This)->lpVtbl -> GetDVDTextLanguageInfo(This,ulLangIndex,pulNumOfStrings,pLangCode,pbCharacterSet) ) 

#define IDvdInfo2_GetDVDTextStringAsNative(This,ulLangIndex,ulStringIndex,pbBuffer,ulMaxBufferSize,pulActualSize,pType)	\
    ( (This)->lpVtbl -> GetDVDTextStringAsNative(This,ulLangIndex,ulStringIndex,pbBuffer,ulMaxBufferSize,pulActualSize,pType) ) 

#define IDvdInfo2_GetDVDTextStringAsUnicode(This,ulLangIndex,ulStringIndex,pchwBuffer,ulMaxBufferSize,pulActualSize,pType)	\
    ( (This)->lpVtbl -> GetDVDTextStringAsUnicode(This,ulLangIndex,ulStringIndex,pchwBuffer,ulMaxBufferSize,pulActualSize,pType) ) 

#define IDvdInfo2_GetPlayerParentalLevel(This,pulParentalLevel,pbCountryCode)	\
    ( (This)->lpVtbl -> GetPlayerParentalLevel(This,pulParentalLevel,pbCountryCode) ) 

#define IDvdInfo2_GetNumberOfChapters(This,ulTitle,pulNumOfChapters)	\
    ( (This)->lpVtbl -> GetNumberOfChapters(This,ulTitle,pulNumOfChapters) ) 

#define IDvdInfo2_GetTitleParentalLevels(This,ulTitle,pulParentalLevels)	\
    ( (This)->lpVtbl -> GetTitleParentalLevels(This,ulTitle,pulParentalLevels) ) 

#define IDvdInfo2_GetDVDDirectory(This,pszwPath,ulMaxSize,pulActualSize)	\
    ( (This)->lpVtbl -> GetDVDDirectory(This,pszwPath,ulMaxSize,pulActualSize) ) 

#define IDvdInfo2_IsAudioStreamEnabled(This,ulStreamNum,pbEnabled)	\
    ( (This)->lpVtbl -> IsAudioStreamEnabled(This,ulStreamNum,pbEnabled) ) 

#define IDvdInfo2_GetDiscID(This,pszwPath,pullDiscID)	\
    ( (This)->lpVtbl -> GetDiscID(This,pszwPath,pullDiscID) ) 

#define IDvdInfo2_GetState(This,pStateData)	\
    ( (This)->lpVtbl -> GetState(This,pStateData) ) 

#define IDvdInfo2_GetMenuLanguages(This,pLanguages,ulMaxLanguages,pulActualLanguages)	\
    ( (This)->lpVtbl -> GetMenuLanguages(This,pLanguages,ulMaxLanguages,pulActualLanguages) ) 

#define IDvdInfo2_GetButtonAtPosition(This,point,pulButtonIndex)	\
    ( (This)->lpVtbl -> GetButtonAtPosition(This,point,pulButtonIndex) ) 

#define IDvdInfo2_GetCmdFromEvent(This,lParam1,pCmdObj)	\
    ( (This)->lpVtbl -> GetCmdFromEvent(This,lParam1,pCmdObj) ) 

#define IDvdInfo2_GetDefaultMenuLanguage(This,pLanguage)	\
    ( (This)->lpVtbl -> GetDefaultMenuLanguage(This,pLanguage) ) 

#define IDvdInfo2_GetDefaultAudioLanguage(This,pLanguage,pAudioExtension)	\
    ( (This)->lpVtbl -> GetDefaultAudioLanguage(This,pLanguage,pAudioExtension) ) 

#define IDvdInfo2_GetDefaultSubpictureLanguage(This,pLanguage,pSubpictureExtension)	\
    ( (This)->lpVtbl -> GetDefaultSubpictureLanguage(This,pLanguage,pSubpictureExtension) ) 

#define IDvdInfo2_GetDecoderCaps(This,pCaps)	\
    ( (This)->lpVtbl -> GetDecoderCaps(This,pCaps) ) 

#define IDvdInfo2_GetButtonRect(This,ulButton,pRect)	\
    ( (This)->lpVtbl -> GetButtonRect(This,ulButton,pRect) ) 

#define IDvdInfo2_IsSubpictureStreamEnabled(This,ulStreamNum,pbEnabled)	\
    ( (This)->lpVtbl -> IsSubpictureStreamEnabled(This,ulStreamNum,pbEnabled) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdInfo2_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0138 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_APP) */
#pragma endregion
#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
typedef 
enum _AM_DVD_GRAPH_FLAGS
    {
        AM_DVD_HWDEC_PREFER	= 0x1,
        AM_DVD_HWDEC_ONLY	= 0x2,
        AM_DVD_SWDEC_PREFER	= 0x4,
        AM_DVD_SWDEC_ONLY	= 0x8,
        AM_DVD_NOVPE	= 0x100,
        AM_DVD_DO_NOT_CLEAR	= 0x200,
        AM_DVD_VMR9_ONLY	= 0x800,
        AM_DVD_EVR_ONLY	= 0x1000,
        AM_DVD_EVR_QOS	= 0x2000,
        AM_DVD_ADAPT_GRAPH	= 0x4000,
        AM_DVD_MASK	= 0xffff
    } 	AM_DVD_GRAPH_FLAGS;

typedef 
enum _AM_DVD_STREAM_FLAGS
    {
        AM_DVD_STREAM_VIDEO	= 0x1,
        AM_DVD_STREAM_AUDIO	= 0x2,
        AM_DVD_STREAM_SUBPIC	= 0x4
    } 	AM_DVD_STREAM_FLAGS;

typedef /* [public][public] */ struct __MIDL___MIDL_itf_strmif_0000_0138_0001
    {
    HRESULT hrVPEStatus;
    BOOL bDvdVolInvalid;
    BOOL bDvdVolUnknown;
    BOOL bNoLine21In;
    BOOL bNoLine21Out;
    int iNumStreams;
    int iNumStreamsFailed;
    DWORD dwFailedStreamsFlag;
    } 	AM_DVD_RENDERSTATUS;



extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0138_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0138_v0_0_s_ifspec;

#ifndef __IDvdGraphBuilder_INTERFACE_DEFINED__
#define __IDvdGraphBuilder_INTERFACE_DEFINED__

/* interface IDvdGraphBuilder */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDvdGraphBuilder;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("FCC152B6-F372-11d0-8E00-00C04FD7C08B")
    IDvdGraphBuilder : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE GetFiltergraph( 
            /* [annotation][out] */ 
            _Out_  IGraphBuilder **ppGB) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDvdInterface( 
            /* [in] */ REFIID riid,
            /* [annotation][out] */ 
            _Out_  void **ppvIF) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE RenderDvdVideoVolume( 
            /* [in] */ LPCWSTR lpcwszPathName,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  AM_DVD_RENDERSTATUS *pStatus) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDvdGraphBuilderVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDvdGraphBuilder * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDvdGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDvdGraphBuilder * This);
        
        DECLSPEC_XFGVIRT(IDvdGraphBuilder, GetFiltergraph)
        HRESULT ( STDMETHODCALLTYPE *GetFiltergraph )( 
            IDvdGraphBuilder * This,
            /* [annotation][out] */ 
            _Out_  IGraphBuilder **ppGB);
        
        DECLSPEC_XFGVIRT(IDvdGraphBuilder, GetDvdInterface)
        HRESULT ( STDMETHODCALLTYPE *GetDvdInterface )( 
            IDvdGraphBuilder * This,
            /* [in] */ REFIID riid,
            /* [annotation][out] */ 
            _Out_  void **ppvIF);
        
        DECLSPEC_XFGVIRT(IDvdGraphBuilder, RenderDvdVideoVolume)
        HRESULT ( STDMETHODCALLTYPE *RenderDvdVideoVolume )( 
            IDvdGraphBuilder * This,
            /* [in] */ LPCWSTR lpcwszPathName,
            /* [in] */ DWORD dwFlags,
            /* [annotation][out] */ 
            _Out_  AM_DVD_RENDERSTATUS *pStatus);
        
        END_INTERFACE
    } IDvdGraphBuilderVtbl;

    interface IDvdGraphBuilder
    {
        CONST_VTBL struct IDvdGraphBuilderVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDvdGraphBuilder_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDvdGraphBuilder_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDvdGraphBuilder_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDvdGraphBuilder_GetFiltergraph(This,ppGB)	\
    ( (This)->lpVtbl -> GetFiltergraph(This,ppGB) ) 

#define IDvdGraphBuilder_GetDvdInterface(This,riid,ppvIF)	\
    ( (This)->lpVtbl -> GetDvdInterface(This,riid,ppvIF) ) 

#define IDvdGraphBuilder_RenderDvdVideoVolume(This,lpcwszPathName,dwFlags,pStatus)	\
    ( (This)->lpVtbl -> RenderDvdVideoVolume(This,lpcwszPathName,dwFlags,pStatus) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDvdGraphBuilder_INTERFACE_DEFINED__ */


#ifndef __IDDrawExclModeVideo_INTERFACE_DEFINED__
#define __IDDrawExclModeVideo_INTERFACE_DEFINED__

/* interface IDDrawExclModeVideo */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDDrawExclModeVideo;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("153ACC21-D83B-11d1-82BF-00A0C9696C8F")
    IDDrawExclModeVideo : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE SetDDrawObject( 
            /* [in] */ IDirectDraw *pDDrawObject) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDDrawObject( 
            /* [annotation][out] */ 
            _Out_  IDirectDraw **ppDDrawObject,
            /* [annotation][out] */ 
            _Out_  BOOL *pbUsingExternal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDDrawSurface( 
            /* [in] */ IDirectDrawSurface *pDDrawSurface) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDDrawSurface( 
            /* [annotation][out] */ 
            _Out_  IDirectDrawSurface **ppDDrawSurface,
            /* [annotation][out] */ 
            _Out_  BOOL *pbUsingExternal) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetDrawParameters( 
            /* [in] */ const RECT *prcSource,
            /* [in] */ const RECT *prcTarget) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetNativeVideoProps( 
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVideoWidth,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVideoHeight,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPictAspectRatioX,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPictAspectRatioY) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetCallbackInterface( 
            /* [in] */ IDDrawExclModeVideoCallback *pCallback,
            /* [in] */ DWORD dwFlags) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDDrawExclModeVideoVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDDrawExclModeVideo * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDDrawExclModeVideo * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDDrawExclModeVideo * This);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, SetDDrawObject)
        HRESULT ( STDMETHODCALLTYPE *SetDDrawObject )( 
            IDDrawExclModeVideo * This,
            /* [in] */ IDirectDraw *pDDrawObject);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, GetDDrawObject)
        HRESULT ( STDMETHODCALLTYPE *GetDDrawObject )( 
            IDDrawExclModeVideo * This,
            /* [annotation][out] */ 
            _Out_  IDirectDraw **ppDDrawObject,
            /* [annotation][out] */ 
            _Out_  BOOL *pbUsingExternal);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, SetDDrawSurface)
        HRESULT ( STDMETHODCALLTYPE *SetDDrawSurface )( 
            IDDrawExclModeVideo * This,
            /* [in] */ IDirectDrawSurface *pDDrawSurface);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, GetDDrawSurface)
        HRESULT ( STDMETHODCALLTYPE *GetDDrawSurface )( 
            IDDrawExclModeVideo * This,
            /* [annotation][out] */ 
            _Out_  IDirectDrawSurface **ppDDrawSurface,
            /* [annotation][out] */ 
            _Out_  BOOL *pbUsingExternal);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, SetDrawParameters)
        HRESULT ( STDMETHODCALLTYPE *SetDrawParameters )( 
            IDDrawExclModeVideo * This,
            /* [in] */ const RECT *prcSource,
            /* [in] */ const RECT *prcTarget);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, GetNativeVideoProps)
        HRESULT ( STDMETHODCALLTYPE *GetNativeVideoProps )( 
            IDDrawExclModeVideo * This,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVideoWidth,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwVideoHeight,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPictAspectRatioX,
            /* [annotation][out] */ 
            _Out_  DWORD *pdwPictAspectRatioY);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideo, SetCallbackInterface)
        HRESULT ( STDMETHODCALLTYPE *SetCallbackInterface )( 
            IDDrawExclModeVideo * This,
            /* [in] */ IDDrawExclModeVideoCallback *pCallback,
            /* [in] */ DWORD dwFlags);
        
        END_INTERFACE
    } IDDrawExclModeVideoVtbl;

    interface IDDrawExclModeVideo
    {
        CONST_VTBL struct IDDrawExclModeVideoVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDDrawExclModeVideo_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDDrawExclModeVideo_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDDrawExclModeVideo_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDDrawExclModeVideo_SetDDrawObject(This,pDDrawObject)	\
    ( (This)->lpVtbl -> SetDDrawObject(This,pDDrawObject) ) 

#define IDDrawExclModeVideo_GetDDrawObject(This,ppDDrawObject,pbUsingExternal)	\
    ( (This)->lpVtbl -> GetDDrawObject(This,ppDDrawObject,pbUsingExternal) ) 

#define IDDrawExclModeVideo_SetDDrawSurface(This,pDDrawSurface)	\
    ( (This)->lpVtbl -> SetDDrawSurface(This,pDDrawSurface) ) 

#define IDDrawExclModeVideo_GetDDrawSurface(This,ppDDrawSurface,pbUsingExternal)	\
    ( (This)->lpVtbl -> GetDDrawSurface(This,ppDDrawSurface,pbUsingExternal) ) 

#define IDDrawExclModeVideo_SetDrawParameters(This,prcSource,prcTarget)	\
    ( (This)->lpVtbl -> SetDrawParameters(This,prcSource,prcTarget) ) 

#define IDDrawExclModeVideo_GetNativeVideoProps(This,pdwVideoWidth,pdwVideoHeight,pdwPictAspectRatioX,pdwPictAspectRatioY)	\
    ( (This)->lpVtbl -> GetNativeVideoProps(This,pdwVideoWidth,pdwVideoHeight,pdwPictAspectRatioX,pdwPictAspectRatioY) ) 

#define IDDrawExclModeVideo_SetCallbackInterface(This,pCallback,dwFlags)	\
    ( (This)->lpVtbl -> SetCallbackInterface(This,pCallback,dwFlags) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDDrawExclModeVideo_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0140 */
/* [local] */ 


enum _AM_OVERLAY_NOTIFY_FLAGS
    {
        AM_OVERLAY_NOTIFY_VISIBLE_CHANGE	= 0x1,
        AM_OVERLAY_NOTIFY_SOURCE_CHANGE	= 0x2,
        AM_OVERLAY_NOTIFY_DEST_CHANGE	= 0x4
    } ;


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0140_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0140_v0_0_s_ifspec;

#ifndef __IDDrawExclModeVideoCallback_INTERFACE_DEFINED__
#define __IDDrawExclModeVideoCallback_INTERFACE_DEFINED__

/* interface IDDrawExclModeVideoCallback */
/* [unique][uuid][local][object] */ 


EXTERN_C const IID IID_IDDrawExclModeVideoCallback;

#if defined(__cplusplus) && !defined(CINTERFACE)
    
    MIDL_INTERFACE("913c24a0-20ab-11d2-9038-00a0c9697298")
    IDDrawExclModeVideoCallback : public IUnknown
    {
    public:
        virtual HRESULT STDMETHODCALLTYPE OnUpdateOverlay( 
            /* [in] */ BOOL bBefore,
            /* [in] */ DWORD dwFlags,
            /* [in] */ BOOL bOldVisible,
            /* [in] */ const RECT *prcOldSrc,
            /* [in] */ const RECT *prcOldDest,
            /* [in] */ BOOL bNewVisible,
            /* [in] */ const RECT *prcNewSrc,
            /* [in] */ const RECT *prcNewDest) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnUpdateColorKey( 
            /* [in] */ const COLORKEY *pKey,
            /* [in] */ DWORD dwColor) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OnUpdateSize( 
            /* [in] */ DWORD dwWidth,
            /* [in] */ DWORD dwHeight,
            /* [in] */ DWORD dwARWidth,
            /* [in] */ DWORD dwARHeight) = 0;
        
    };
    
    
#else 	/* C style interface */

    typedef struct IDDrawExclModeVideoCallbackVtbl
    {
        BEGIN_INTERFACE
        
        DECLSPEC_XFGVIRT(IUnknown, QueryInterface)
        HRESULT ( STDMETHODCALLTYPE *QueryInterface )( 
            IDDrawExclModeVideoCallback * This,
            /* [in] */ REFIID riid,
            /* [annotation][iid_is][out] */ 
            _COM_Outptr_  void **ppvObject);
        
        DECLSPEC_XFGVIRT(IUnknown, AddRef)
        ULONG ( STDMETHODCALLTYPE *AddRef )( 
            IDDrawExclModeVideoCallback * This);
        
        DECLSPEC_XFGVIRT(IUnknown, Release)
        ULONG ( STDMETHODCALLTYPE *Release )( 
            IDDrawExclModeVideoCallback * This);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideoCallback, OnUpdateOverlay)
        HRESULT ( STDMETHODCALLTYPE *OnUpdateOverlay )( 
            IDDrawExclModeVideoCallback * This,
            /* [in] */ BOOL bBefore,
            /* [in] */ DWORD dwFlags,
            /* [in] */ BOOL bOldVisible,
            /* [in] */ const RECT *prcOldSrc,
            /* [in] */ const RECT *prcOldDest,
            /* [in] */ BOOL bNewVisible,
            /* [in] */ const RECT *prcNewSrc,
            /* [in] */ const RECT *prcNewDest);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideoCallback, OnUpdateColorKey)
        HRESULT ( STDMETHODCALLTYPE *OnUpdateColorKey )( 
            IDDrawExclModeVideoCallback * This,
            /* [in] */ const COLORKEY *pKey,
            /* [in] */ DWORD dwColor);
        
        DECLSPEC_XFGVIRT(IDDrawExclModeVideoCallback, OnUpdateSize)
        HRESULT ( STDMETHODCALLTYPE *OnUpdateSize )( 
            IDDrawExclModeVideoCallback * This,
            /* [in] */ DWORD dwWidth,
            /* [in] */ DWORD dwHeight,
            /* [in] */ DWORD dwARWidth,
            /* [in] */ DWORD dwARHeight);
        
        END_INTERFACE
    } IDDrawExclModeVideoCallbackVtbl;

    interface IDDrawExclModeVideoCallback
    {
        CONST_VTBL struct IDDrawExclModeVideoCallbackVtbl *lpVtbl;
    };

    

#ifdef COBJMACROS


#define IDDrawExclModeVideoCallback_QueryInterface(This,riid,ppvObject)	\
    ( (This)->lpVtbl -> QueryInterface(This,riid,ppvObject) ) 

#define IDDrawExclModeVideoCallback_AddRef(This)	\
    ( (This)->lpVtbl -> AddRef(This) ) 

#define IDDrawExclModeVideoCallback_Release(This)	\
    ( (This)->lpVtbl -> Release(This) ) 


#define IDDrawExclModeVideoCallback_OnUpdateOverlay(This,bBefore,dwFlags,bOldVisible,prcOldSrc,prcOldDest,bNewVisible,prcNewSrc,prcNewDest)	\
    ( (This)->lpVtbl -> OnUpdateOverlay(This,bBefore,dwFlags,bOldVisible,prcOldSrc,prcOldDest,bNewVisible,prcNewSrc,prcNewDest) ) 

#define IDDrawExclModeVideoCallback_OnUpdateColorKey(This,pKey,dwColor)	\
    ( (This)->lpVtbl -> OnUpdateColorKey(This,pKey,dwColor) ) 

#define IDDrawExclModeVideoCallback_OnUpdateSize(This,dwWidth,dwHeight,dwARWidth,dwARHeight)	\
    ( (This)->lpVtbl -> OnUpdateSize(This,dwWidth,dwHeight,dwARWidth,dwARHeight) ) 

#endif /* COBJMACROS */


#endif 	/* C style interface */




#endif 	/* __IDDrawExclModeVideoCallback_INTERFACE_DEFINED__ */


/* interface __MIDL_itf_strmif_0000_0141 */
/* [local] */ 

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion
// Restore the previous setting for C4201 compiler warning
#pragma warning(pop)


extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0141_v0_0_c_ifspec;
extern RPC_IF_HANDLE __MIDL_itf_strmif_0000_0141_v0_0_s_ifspec;

/* Additional Prototypes for ALL interfaces */

/* end of Additional Prototypes */

#ifdef __cplusplus
}
#endif

#endif


