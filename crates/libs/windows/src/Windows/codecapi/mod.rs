pub const AVENC_H263V_LEVELCOUNT: u32 = 8;
pub const AVENC_H264V_LEVELCOUNT: u32 = 16;
pub const AVENC_H264V_MAX_MBBITS: u32 = 3200;
pub const AVEncAudioInputContent_Music: eAVEncAudioInputContent = 2;
pub const AVEncAudioInputContent_Unknown: eAVEncAudioInputContent = 0;
pub const AVEncAudioInputContent_Voice: eAVEncAudioInputContent = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVAudioChannelConfig(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVAudioChannelCount(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVAudioSampleRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDDSurroundMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDSPLoudnessEqualization(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDSPSpeakerFill(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecAACDownmixMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecAudioDualMono(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecAudioDualMonoReproMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecCommonInputFormat(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecCommonMeanBitRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecCommonMeanBitRateInterval(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecCommonOutputFormat(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecDDDynamicRangeScaleHigh(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecDDDynamicRangeScaleLow(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecDDMatrixDecodingMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecDDOperationalMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecDDStereoDownMixMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecDisableVideoPostProcessing(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecHEAACDynamicRangeControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecNumWorkerThreads(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecSoftwareDynamicFormatChange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoAcceleration_H264(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoAcceleration_MPEG2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoAcceleration_VC1(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoCodecType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoDXVABusEncryption(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoDXVAMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoDropPicWithMissingRef(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoFastDecodeMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoH264ErrorConcealment(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoImageSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoInputScanType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoMPEG2ErrorConcealment(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoMaxCodedHeight(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoMaxCodedWidth(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoPixelAspectRatio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoProcDeinterlaceCSC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoSWPowerLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoSoftwareDeinterlaceMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVDecVideoThumbnailGenerationMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEnableInLoopDeblockFilter(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAACEnableVBR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAdaptiveMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioDualMono(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioInputContent(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioIntervalToEncode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioIntervalToSkip(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel0(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel1(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel10(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel11(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel12(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel13(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel14(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel15(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel3(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel4(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel5(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel6(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel7(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel8(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMapDestChannel9(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncAudioMeanBitRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncChromaEncodeMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncChromaUpdateTime(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCodecType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonAllowFrameDrops(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonBufferInLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonBufferOutLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonBufferSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonFormatConstraint(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonLowLatency(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonMaxBitRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonMeanBitRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonMeanBitRateInterval(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonMinBitRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonMultipassMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonPassEnd(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonPassStart(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonQuality(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonQualityVsSpeed(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonRateControlMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonRealTime(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonStreamEndHandling(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncCommonTranscodeEncodingProfile(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDAtoDConverterType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDCentreDownMixLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDChannelBWLowPassFilter(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDCopyright(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDDCHighPassFilter(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDDialogNormalization(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDDigitalDeemphasis(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDDynamicRangeCompressionControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDHeadphoneMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDLFELowPassFilter(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDLoRoCenterMixLvl_x10(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDLoRoSurroundMixLvl_x10(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDLtRtCenterMixLvl_x10(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDLtRtSurroundMixLvl_x10(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDOriginalBitstream(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDPreferredStereoDownMixMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDProductionInfoExists(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDProductionMixLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDProductionRoomType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDRFPreEmphasisFilter(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDService(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDSurround3dBAttenuation(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDSurround90DegreeePhaseShift(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDSurroundDownMixLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncDDSurroundExMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncEnableVideoProcessing(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncH264CABACEnable(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncH264PPSID(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncH264SPSID(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncInputVideoSystem(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncLowPowerEncoder(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxDVDNavPacks(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxEarliestPTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxInitialSCR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxLargestPacketSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxMuxRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxNumStreams(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxPackSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxPacketOverhead(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxSysAudioLock(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxSysCSPS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxSysFixed(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxSysRateBound(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxSysSTDBufferBound(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxSysVideoLock(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12MuxTargetPacketizer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12PktzCopyright(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12PktzInitialPTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12PktzOriginal(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12PktzPacketSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12PktzSTDBuffer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMP12PktzStreamID(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPACodingMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPACopyright(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPAEmphasisType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPAEnableRedundancyProtection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPALayer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPAOriginalBitstream(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPAPrivateUserBit(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVAddSeqEndCode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVDefaultBPictureCount(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVFrameFieldMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGOPOpen(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGOPSInSeq(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGOPSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGOPSizeMax(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGOPSizeMin(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGenerateHeaderPicDispExt(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGenerateHeaderPicExt(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGenerateHeaderSeqDispExt(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGenerateHeaderSeqExt(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVGenerateHeaderSeqScaleExt(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVIntraDCPrecision(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVIntraVLCTable(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVLevel(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVProfile(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVQScaleType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVQuantMatrixChromaIntra(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVQuantMatrixChromaNonIntra(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVQuantMatrixIntra(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVQuantMatrixNonIntra(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVScanPattern(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVSceneDetection(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMPVUseConcealmentMotionVectors(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMaxFrameRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncMuxOutputStreamType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncNoInputCopy(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncNumWorkerThreads(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncProgressiveUpdateTime(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncSliceControlMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncSliceControlSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncSliceGenerationMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatAudioAverageBPS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatAudioAveragePCMValue(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatAudioPeakPCMValue(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatAverageBPS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatCommonCompletedPasses(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatHardwareBandwidthUtilitization(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatHardwareProcessorUtilitization(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatMPVSkippedEmptyFrames(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatVideoCodedFrames(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatVideoOutputFrameRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatVideoTotalFrames(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatWMVCBAvg(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatWMVCBMax(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncStatWMVDecoderComplexityProfile(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncTileColumns(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncTileRows(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoCBRMotionTradeoff(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoCTBSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoCodedVideoAccessUnitSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoConsecutiveFramesForLayer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoContentType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoD3D12ReconstructedPictureOutputMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoDefaultUpperFieldDominant(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoDirtyRectEnabled(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoDisplayDimension(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoEnableFramePsnrYuv(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoEnableSpatialAdaptiveQuantization(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoEncodeDimension(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoEncodeFrameTypeQP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoEncodeOffsetOrigin(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoEncodeQP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoFieldSwap(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoForceKeyFrame(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoForceSourceScanType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoGradualIntraRefresh(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoHeaderDropFrame(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoHeaderFrames(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoHeaderHours(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoHeaderMinutes(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoHeaderSeconds(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputAbsoluteQPBlockSettings(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputChromaResolution(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputChromaSubsampling(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputColorLighting(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputColorNominalRange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputColorPrimaries(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputColorTransferFunction(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputColorTransferMatrix(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInputDeltaQPBlockSettings(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInstantTemporalUpSwitching(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoIntraLayerPrediction(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInverseTelecineEnable(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoInverseTelecineThreshold(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoLTRBufferControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMarkLTRFrame(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMaxCTBSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMaxKeyframeDistance(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMaxNumRefFrame(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMaxNumRefFrameForLayer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMaxQP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMaxTemporalLayers(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMeanAbsoluteDifference(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoMinQP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoNoOfFieldsToEncode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoNoOfFieldsToSkip(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoNumGOPsPerIDR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputBitsUsedMapBlockSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputChromaResolution(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputChromaSubsampling(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputColorLighting(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputColorNominalRange(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputColorPrimaries(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputColorTransferFunction(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputColorTransferMatrix(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputFrameRate(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputFrameRateConversion(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputQPMapBlockSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoOutputScanType(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoPixelAspectRatio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoROIEnabled(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoRateControlParams(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoSatdMapBlockSize(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoSelectLayer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoSourceFilmContent(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoSourceIsBW(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoSupportedControls(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoTemporalLayerCount(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoUsage(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncVideoUseLTRFrame(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncWMVDecoderComplexity(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncWMVInterlacedEncoding(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncWMVKeyFrameBufferLevelMarker(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncWMVKeyFrameDistance(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVEncWMVProduceDummyFrames(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVLowLatencyMode(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVPriorityControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVRealtimeControl(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_AVScenarioInfo(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_FeatureMapFlagsUsed(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputAAC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputDTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputDolby(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputDolbyDigitalPlus(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputHEAAC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputMPEG(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputPCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputWMA(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioInputWMAPro(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioOutputFormat_PCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioOutputFormat_PCM_Headphones(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioOutputFormat_PCM_Stereo_Auto(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioOutputFormat_PCM_Stereo_MatrixEncoded(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioOutputFormat_SPDIF_Bitstream(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVDecAudioOutputFormat_SPDIF_PCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatATSC(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatDVB(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatDVD_DashVR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatDVD_PlusVR(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatDVD_V(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatHighMAT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatHighMPV(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatMP3(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatSVCD(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatUnSpecified(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncCommonFormatVCD(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncDTS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncDTSHD(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncDV(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncDolbyDigitalConsumer(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncDolbyDigitalPlus(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncDolbyDigitalPro(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncH264Video(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncMLP(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncMPEG1Audio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncMPEG1Video(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncMPEG2Audio(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncMPEG2Video(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncPCM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncSDDS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncWMALossless(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncWMAPro(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncWMAVoice(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEncWMV(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GUID_AVEndMPEG4Video(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_GetOPMContext(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_SetHDCPManagerContext(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODECAPI_VideoEncoderDisplayContentType(pub u8);
pub type eAVAudioChannelConfig = i32;
pub const eAVAudioChannelConfig_BACK_CENTER: eAVAudioChannelConfig = 256;
pub const eAVAudioChannelConfig_BACK_LEFT: eAVAudioChannelConfig = 16;
pub const eAVAudioChannelConfig_BACK_RIGHT: eAVAudioChannelConfig = 32;
pub const eAVAudioChannelConfig_FRONT_CENTER: eAVAudioChannelConfig = 4;
pub const eAVAudioChannelConfig_FRONT_LEFT: eAVAudioChannelConfig = 1;
pub const eAVAudioChannelConfig_FRONT_LEFT_OF_CENTER: eAVAudioChannelConfig = 64;
pub const eAVAudioChannelConfig_FRONT_RIGHT: eAVAudioChannelConfig = 2;
pub const eAVAudioChannelConfig_FRONT_RIGHT_OF_CENTER: eAVAudioChannelConfig = 128;
pub const eAVAudioChannelConfig_LOW_FREQUENCY: eAVAudioChannelConfig = 8;
pub const eAVAudioChannelConfig_SIDE_LEFT: eAVAudioChannelConfig = 512;
pub const eAVAudioChannelConfig_SIDE_RIGHT: eAVAudioChannelConfig = 1024;
pub const eAVAudioChannelConfig_TOP_BACK_CENTER: eAVAudioChannelConfig = 65536;
pub const eAVAudioChannelConfig_TOP_BACK_LEFT: eAVAudioChannelConfig = 32768;
pub const eAVAudioChannelConfig_TOP_BACK_RIGHT: eAVAudioChannelConfig = 131072;
pub const eAVAudioChannelConfig_TOP_CENTER: eAVAudioChannelConfig = 2048;
pub const eAVAudioChannelConfig_TOP_FRONT_CENTER: eAVAudioChannelConfig = 8192;
pub const eAVAudioChannelConfig_TOP_FRONT_LEFT: eAVAudioChannelConfig = 4096;
pub const eAVAudioChannelConfig_TOP_FRONT_RIGHT: eAVAudioChannelConfig = 16384;
pub type eAVDDSurroundMode = i32;
pub const eAVDDSurroundMode_No: eAVDDSurroundMode = 1;
pub const eAVDDSurroundMode_NotIndicated: eAVDDSurroundMode = 0;
pub const eAVDDSurroundMode_Yes: eAVDDSurroundMode = 2;
pub type eAVDSPLoudnessEqualization = i32;
pub const eAVDSPLoudnessEqualization_AUTO: eAVDSPLoudnessEqualization = 2;
pub const eAVDSPLoudnessEqualization_OFF: eAVDSPLoudnessEqualization = 0;
pub const eAVDSPLoudnessEqualization_ON: eAVDSPLoudnessEqualization = 1;
pub type eAVDSPSpeakerFill = i32;
pub const eAVDSPSpeakerFill_AUTO: eAVDSPSpeakerFill = 2;
pub const eAVDSPSpeakerFill_OFF: eAVDSPSpeakerFill = 0;
pub const eAVDSPSpeakerFill_ON: eAVDSPSpeakerFill = 1;
pub type eAVDecAACDownmixMode = i32;
pub const eAVDecAACUseARIBDownmix: eAVDecAACDownmixMode = 1;
pub const eAVDecAACUseISODownmix: eAVDecAACDownmixMode = 0;
pub type eAVDecAudioDualMono = i32;
pub type eAVDecAudioDualMonoReproMode = i32;
pub const eAVDecAudioDualMonoReproMode_LEFT_MONO: eAVDecAudioDualMonoReproMode = 1;
pub const eAVDecAudioDualMonoReproMode_MIX_MONO: eAVDecAudioDualMonoReproMode = 3;
pub const eAVDecAudioDualMonoReproMode_RIGHT_MONO: eAVDecAudioDualMonoReproMode = 2;
pub const eAVDecAudioDualMonoReproMode_STEREO: eAVDecAudioDualMonoReproMode = 0;
pub const eAVDecAudioDualMono_IsDualMono: eAVDecAudioDualMono = 1;
pub const eAVDecAudioDualMono_IsNotDualMono: eAVDecAudioDualMono = 0;
pub const eAVDecAudioDualMono_UnSpecified: eAVDecAudioDualMono = 2;
pub type eAVDecDDMatrixDecodingMode = i32;
pub const eAVDecDDMatrixDecodingMode_AUTO: eAVDecDDMatrixDecodingMode = 2;
pub const eAVDecDDMatrixDecodingMode_OFF: eAVDecDDMatrixDecodingMode = 0;
pub const eAVDecDDMatrixDecodingMode_ON: eAVDecDDMatrixDecodingMode = 1;
pub type eAVDecDDOperationalMode = i32;
pub const eAVDecDDOperationalMode_CUSTOM0: eAVDecDDOperationalMode = 3;
pub const eAVDecDDOperationalMode_CUSTOM1: eAVDecDDOperationalMode = 4;
pub const eAVDecDDOperationalMode_LINE: eAVDecDDOperationalMode = 1;
pub const eAVDecDDOperationalMode_NONE: eAVDecDDOperationalMode = 0;
pub const eAVDecDDOperationalMode_PORTABLE11: eAVDecDDOperationalMode = 6;
pub const eAVDecDDOperationalMode_PORTABLE14: eAVDecDDOperationalMode = 7;
pub const eAVDecDDOperationalMode_PORTABLE16: eAVDecDDOperationalMode = 8;
pub const eAVDecDDOperationalMode_PORTABLE8: eAVDecDDOperationalMode = 5;
pub const eAVDecDDOperationalMode_RF: eAVDecDDOperationalMode = 2;
pub type eAVDecDDStereoDownMixMode = i32;
pub const eAVDecDDStereoDownMixMode_Auto: eAVDecDDStereoDownMixMode = 0;
pub const eAVDecDDStereoDownMixMode_LoRo: eAVDecDDStereoDownMixMode = 2;
pub const eAVDecDDStereoDownMixMode_LtRt: eAVDecDDStereoDownMixMode = 1;
pub type eAVDecHEAACDynamicRangeControl = i32;
pub const eAVDecHEAACDynamicRangeControl_OFF: eAVDecHEAACDynamicRangeControl = 0;
pub const eAVDecHEAACDynamicRangeControl_ON: eAVDecHEAACDynamicRangeControl = 1;
pub type eAVDecVideoCodecType = i32;
pub const eAVDecVideoCodecType_H264: eAVDecVideoCodecType = 2;
pub const eAVDecVideoCodecType_MPEG2: eAVDecVideoCodecType = 1;
pub const eAVDecVideoCodecType_NOTPLAYING: eAVDecVideoCodecType = 0;
pub type eAVDecVideoDXVABusEncryption = i32;
pub const eAVDecVideoDXVABusEncryption_AES: eAVDecVideoDXVABusEncryption = 2;
pub const eAVDecVideoDXVABusEncryption_NONE: eAVDecVideoDXVABusEncryption = 0;
pub const eAVDecVideoDXVABusEncryption_PRIVATE: eAVDecVideoDXVABusEncryption = 1;
pub type eAVDecVideoDXVAMode = i32;
pub const eAVDecVideoDXVAMode_IDCT: eAVDecVideoDXVAMode = 3;
pub const eAVDecVideoDXVAMode_MC: eAVDecVideoDXVAMode = 2;
pub const eAVDecVideoDXVAMode_NOTPLAYING: eAVDecVideoDXVAMode = 0;
pub const eAVDecVideoDXVAMode_SW: eAVDecVideoDXVAMode = 1;
pub const eAVDecVideoDXVAMode_VLD: eAVDecVideoDXVAMode = 4;
pub type eAVDecVideoH264ErrorConcealment = i32;
pub type eAVDecVideoInputScanType = i32;
pub const eAVDecVideoInputScan_Interlaced_LowerFieldFirst: eAVDecVideoInputScanType = 3;
pub const eAVDecVideoInputScan_Interlaced_UpperFieldFirst: eAVDecVideoInputScanType = 2;
pub const eAVDecVideoInputScan_Progressive: eAVDecVideoInputScanType = 1;
pub const eAVDecVideoInputScan_Unknown: eAVDecVideoInputScanType = 0;
pub type eAVDecVideoMPEG2ErrorConcealment = i32;
pub type eAVDecVideoSWPowerLevel = i32;
pub const eAVDecVideoSWPowerLevel_Balanced: eAVDecVideoSWPowerLevel = 50;
pub const eAVDecVideoSWPowerLevel_BatteryLife: eAVDecVideoSWPowerLevel = 0;
pub const eAVDecVideoSWPowerLevel_VideoQuality: eAVDecVideoSWPowerLevel = 100;
pub type eAVDecVideoSoftwareDeinterlaceMode = i32;
pub const eAVDecVideoSoftwareDeinterlaceMode_BOBDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 2;
pub const eAVDecVideoSoftwareDeinterlaceMode_NoDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 0;
pub const eAVDecVideoSoftwareDeinterlaceMode_ProgressiveDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 1;
pub const eAVDecVideoSoftwareDeinterlaceMode_SmartBOBDeinterlacing: eAVDecVideoSoftwareDeinterlaceMode = 3;
pub type eAVEncAV1PictureType = i32;
pub const eAVEncAV1PictureType_Inter: eAVEncAV1PictureType = 2;
pub const eAVEncAV1PictureType_Intra_Only: eAVEncAV1PictureType = 1;
pub const eAVEncAV1PictureType_Key: eAVEncAV1PictureType = 0;
pub const eAVEncAV1PictureType_Switch: eAVEncAV1PictureType = 3;
pub type eAVEncAV1VLevel = i32;
pub const eAVEncAV1VLevel2: eAVEncAV1VLevel = 0;
pub const eAVEncAV1VLevel2_1: eAVEncAV1VLevel = 1;
pub const eAVEncAV1VLevel3: eAVEncAV1VLevel = 4;
pub const eAVEncAV1VLevel3_1: eAVEncAV1VLevel = 5;
pub const eAVEncAV1VLevel4: eAVEncAV1VLevel = 8;
pub const eAVEncAV1VLevel4_1: eAVEncAV1VLevel = 9;
pub const eAVEncAV1VLevel5: eAVEncAV1VLevel = 12;
pub const eAVEncAV1VLevel5_1: eAVEncAV1VLevel = 13;
pub const eAVEncAV1VLevel5_2: eAVEncAV1VLevel = 14;
pub const eAVEncAV1VLevel5_3: eAVEncAV1VLevel = 15;
pub const eAVEncAV1VLevel6: eAVEncAV1VLevel = 16;
pub const eAVEncAV1VLevel6_1: eAVEncAV1VLevel = 17;
pub const eAVEncAV1VLevel6_2: eAVEncAV1VLevel = 18;
pub const eAVEncAV1VLevel6_3: eAVEncAV1VLevel = 19;
pub type eAVEncAV1VProfile = i32;
pub const eAVEncAV1VProfile_High_444_10: eAVEncAV1VProfile = 6;
pub const eAVEncAV1VProfile_High_444_8: eAVEncAV1VProfile = 5;
pub const eAVEncAV1VProfile_Main_400_10: eAVEncAV1VProfile = 4;
pub const eAVEncAV1VProfile_Main_400_8: eAVEncAV1VProfile = 3;
pub const eAVEncAV1VProfile_Main_420_10: eAVEncAV1VProfile = 2;
pub const eAVEncAV1VProfile_Main_420_8: eAVEncAV1VProfile = 1;
pub const eAVEncAV1VProfile_Professional_400_12: eAVEncAV1VProfile = 8;
pub const eAVEncAV1VProfile_Professional_420_12: eAVEncAV1VProfile = 7;
pub const eAVEncAV1VProfile_Professional_422_10: eAVEncAV1VProfile = 11;
pub const eAVEncAV1VProfile_Professional_422_12: eAVEncAV1VProfile = 12;
pub const eAVEncAV1VProfile_Professional_422_8: eAVEncAV1VProfile = 10;
pub const eAVEncAV1VProfile_Professional_444_12: eAVEncAV1VProfile = 9;
pub const eAVEncAV1VProfile_unknown: eAVEncAV1VProfile = 0;
pub type eAVEncAdaptiveMode = i32;
pub const eAVEncAdaptiveMode_FrameRate: eAVEncAdaptiveMode = 2;
pub const eAVEncAdaptiveMode_None: eAVEncAdaptiveMode = 0;
pub const eAVEncAdaptiveMode_Resolution: eAVEncAdaptiveMode = 1;
pub type eAVEncAudioDualMono = i32;
pub const eAVEncAudioDualMono_Off: eAVEncAudioDualMono = 1;
pub const eAVEncAudioDualMono_On: eAVEncAudioDualMono = 2;
pub const eAVEncAudioDualMono_SameAsInput: eAVEncAudioDualMono = 0;
pub type eAVEncAudioInputContent = i32;
pub type eAVEncChromaEncodeMode = i32;
pub const eAVEncChromaEncodeMode_420: eAVEncChromaEncodeMode = 0;
pub const eAVEncChromaEncodeMode_444: eAVEncChromaEncodeMode = 1;
pub const eAVEncChromaEncodeMode_444_v2: eAVEncChromaEncodeMode = 2;
pub type eAVEncCommonRateControlMode = i32;
pub const eAVEncCommonRateControlMode_CBR: eAVEncCommonRateControlMode = 0;
pub const eAVEncCommonRateControlMode_GlobalLowDelayVBR: eAVEncCommonRateControlMode = 6;
pub const eAVEncCommonRateControlMode_GlobalVBR: eAVEncCommonRateControlMode = 5;
pub const eAVEncCommonRateControlMode_LowDelayVBR: eAVEncCommonRateControlMode = 4;
pub const eAVEncCommonRateControlMode_PeakConstrainedVBR: eAVEncCommonRateControlMode = 1;
pub const eAVEncCommonRateControlMode_Quality: eAVEncCommonRateControlMode = 3;
pub const eAVEncCommonRateControlMode_UnconstrainedVBR: eAVEncCommonRateControlMode = 2;
pub type eAVEncCommonStreamEndHandling = i32;
pub const eAVEncCommonStreamEndHandling_DiscardPartial: eAVEncCommonStreamEndHandling = 0;
pub const eAVEncCommonStreamEndHandling_EnsureComplete: eAVEncCommonStreamEndHandling = 1;
pub type eAVEncDDAtoDConverterType = i32;
pub const eAVEncDDAtoDConverterType_HDCD: eAVEncDDAtoDConverterType = 1;
pub const eAVEncDDAtoDConverterType_Standard: eAVEncDDAtoDConverterType = 0;
pub type eAVEncDDDynamicRangeCompressionControl = i32;
pub const eAVEncDDDynamicRangeCompressionControl_FilmLight: eAVEncDDDynamicRangeCompressionControl = 2;
pub const eAVEncDDDynamicRangeCompressionControl_FilmStandard: eAVEncDDDynamicRangeCompressionControl = 1;
pub const eAVEncDDDynamicRangeCompressionControl_MusicLight: eAVEncDDDynamicRangeCompressionControl = 4;
pub const eAVEncDDDynamicRangeCompressionControl_MusicStandard: eAVEncDDDynamicRangeCompressionControl = 3;
pub const eAVEncDDDynamicRangeCompressionControl_None: eAVEncDDDynamicRangeCompressionControl = 0;
pub const eAVEncDDDynamicRangeCompressionControl_Speech: eAVEncDDDynamicRangeCompressionControl = 5;
pub type eAVEncDDHeadphoneMode = i32;
pub const eAVEncDDHeadphoneMode_Encoded: eAVEncDDHeadphoneMode = 2;
pub const eAVEncDDHeadphoneMode_NotEncoded: eAVEncDDHeadphoneMode = 1;
pub const eAVEncDDHeadphoneMode_NotIndicated: eAVEncDDHeadphoneMode = 0;
pub type eAVEncDDPreferredStereoDownMixMode = i32;
pub const eAVEncDDPreferredStereoDownMixMode_LoRo: eAVEncDDPreferredStereoDownMixMode = 1;
pub const eAVEncDDPreferredStereoDownMixMode_LtRt: eAVEncDDPreferredStereoDownMixMode = 0;
pub type eAVEncDDProductionRoomType = i32;
pub const eAVEncDDProductionRoomType_Large: eAVEncDDProductionRoomType = 1;
pub const eAVEncDDProductionRoomType_NotIndicated: eAVEncDDProductionRoomType = 0;
pub const eAVEncDDProductionRoomType_Small: eAVEncDDProductionRoomType = 2;
pub type eAVEncDDService = i32;
pub const eAVEncDDService_C: eAVEncDDService = 5;
pub const eAVEncDDService_CM: eAVEncDDService = 0;
pub const eAVEncDDService_D: eAVEncDDService = 4;
pub const eAVEncDDService_E: eAVEncDDService = 6;
pub const eAVEncDDService_HI: eAVEncDDService = 3;
pub const eAVEncDDService_ME: eAVEncDDService = 1;
pub const eAVEncDDService_VI: eAVEncDDService = 2;
pub const eAVEncDDService_VO: eAVEncDDService = 7;
pub type eAVEncDDSurroundExMode = i32;
pub const eAVEncDDSurroundExMode_No: eAVEncDDSurroundExMode = 1;
pub const eAVEncDDSurroundExMode_NotIndicated: eAVEncDDSurroundExMode = 0;
pub const eAVEncDDSurroundExMode_Yes: eAVEncDDSurroundExMode = 2;
pub type eAVEncH263PictureType = i32;
pub const eAVEncH263PictureType_B: eAVEncH263PictureType = 2;
pub const eAVEncH263PictureType_I: eAVEncH263PictureType = 0;
pub const eAVEncH263PictureType_P: eAVEncH263PictureType = 1;
pub type eAVEncH263VLevel = i32;
pub const eAVEncH263VLevel1: eAVEncH263VLevel = 10;
pub const eAVEncH263VLevel2: eAVEncH263VLevel = 20;
pub const eAVEncH263VLevel3: eAVEncH263VLevel = 30;
pub const eAVEncH263VLevel4: eAVEncH263VLevel = 40;
pub const eAVEncH263VLevel4_5: eAVEncH263VLevel = 45;
pub const eAVEncH263VLevel5: eAVEncH263VLevel = 50;
pub const eAVEncH263VLevel6: eAVEncH263VLevel = 60;
pub const eAVEncH263VLevel7: eAVEncH263VLevel = 70;
pub type eAVEncH263VProfile = i32;
pub const eAVEncH263VProfile_Base: eAVEncH263VProfile = 0;
pub const eAVEncH263VProfile_CompatibilityV1: eAVEncH263VProfile = 2;
pub const eAVEncH263VProfile_CompatibilityV2: eAVEncH263VProfile = 1;
pub const eAVEncH263VProfile_HighCompression: eAVEncH263VProfile = 5;
pub const eAVEncH263VProfile_HighLatency: eAVEncH263VProfile = 8;
pub const eAVEncH263VProfile_Interlace: eAVEncH263VProfile = 7;
pub const eAVEncH263VProfile_Internet: eAVEncH263VProfile = 6;
pub const eAVEncH263VProfile_WirelessV2: eAVEncH263VProfile = 3;
pub const eAVEncH263VProfile_WirelessV3: eAVEncH263VProfile = 4;
pub type eAVEncH264PictureType = i32;
pub const eAVEncH264PictureType_B: eAVEncH264PictureType = 2;
pub const eAVEncH264PictureType_IDR: eAVEncH264PictureType = 0;
pub const eAVEncH264PictureType_P: eAVEncH264PictureType = 1;
pub type eAVEncH264VLevel = i32;
pub const eAVEncH264VLevel1: eAVEncH264VLevel = 10;
pub const eAVEncH264VLevel1_1: eAVEncH264VLevel = 11;
pub const eAVEncH264VLevel1_2: eAVEncH264VLevel = 12;
pub const eAVEncH264VLevel1_3: eAVEncH264VLevel = 13;
pub const eAVEncH264VLevel1_b: eAVEncH264VLevel = 11;
pub const eAVEncH264VLevel2: eAVEncH264VLevel = 20;
pub const eAVEncH264VLevel2_1: eAVEncH264VLevel = 21;
pub const eAVEncH264VLevel2_2: eAVEncH264VLevel = 22;
pub const eAVEncH264VLevel3: eAVEncH264VLevel = 30;
pub const eAVEncH264VLevel3_1: eAVEncH264VLevel = 31;
pub const eAVEncH264VLevel3_2: eAVEncH264VLevel = 32;
pub const eAVEncH264VLevel4: eAVEncH264VLevel = 40;
pub const eAVEncH264VLevel4_1: eAVEncH264VLevel = 41;
pub const eAVEncH264VLevel4_2: eAVEncH264VLevel = 42;
pub const eAVEncH264VLevel5: eAVEncH264VLevel = 50;
pub const eAVEncH264VLevel5_1: eAVEncH264VLevel = 51;
pub const eAVEncH264VLevel5_2: eAVEncH264VLevel = 52;
pub const eAVEncH264VLevel6: eAVEncH264VLevel = 60;
pub const eAVEncH264VLevel6_1: eAVEncH264VLevel = 61;
pub const eAVEncH264VLevel6_2: eAVEncH264VLevel = 62;
pub type eAVEncH264VProfile = i32;
pub const eAVEncH264VProfile_422: eAVEncH264VProfile = 122;
pub const eAVEncH264VProfile_444: eAVEncH264VProfile = 244;
pub const eAVEncH264VProfile_Base: eAVEncH264VProfile = 66;
pub const eAVEncH264VProfile_ConstrainedBase: eAVEncH264VProfile = 256;
pub const eAVEncH264VProfile_ConstrainedHigh: u32 = 257;
pub const eAVEncH264VProfile_Extended: eAVEncH264VProfile = 88;
pub const eAVEncH264VProfile_High: eAVEncH264VProfile = 100;
pub const eAVEncH264VProfile_High10: eAVEncH264VProfile = 110;
pub const eAVEncH264VProfile_Main: eAVEncH264VProfile = 77;
pub const eAVEncH264VProfile_MultiviewHigh: eAVEncH264VProfile = 118;
pub const eAVEncH264VProfile_ScalableBase: eAVEncH264VProfile = 83;
pub const eAVEncH264VProfile_ScalableHigh: eAVEncH264VProfile = 86;
pub const eAVEncH264VProfile_Simple: eAVEncH264VProfile = 66;
pub const eAVEncH264VProfile_StereoHigh: eAVEncH264VProfile = 128;
pub const eAVEncH264VProfile_UCConstrainedHigh: eAVEncH264VProfile = 257;
pub const eAVEncH264VProfile_UCScalableConstrainedBase: eAVEncH264VProfile = 258;
pub const eAVEncH264VProfile_UCScalableConstrainedHigh: eAVEncH264VProfile = 259;
pub const eAVEncH264VProfile_unknown: eAVEncH264VProfile = 0;
pub type eAVEncH265VLevel = i32;
pub const eAVEncH265VLevel1: eAVEncH265VLevel = 30;
pub const eAVEncH265VLevel2: eAVEncH265VLevel = 60;
pub const eAVEncH265VLevel2_1: eAVEncH265VLevel = 63;
pub const eAVEncH265VLevel3: eAVEncH265VLevel = 90;
pub const eAVEncH265VLevel3_1: eAVEncH265VLevel = 93;
pub const eAVEncH265VLevel4: eAVEncH265VLevel = 120;
pub const eAVEncH265VLevel4_1: eAVEncH265VLevel = 123;
pub const eAVEncH265VLevel5: eAVEncH265VLevel = 150;
pub const eAVEncH265VLevel5_1: eAVEncH265VLevel = 153;
pub const eAVEncH265VLevel5_2: eAVEncH265VLevel = 156;
pub const eAVEncH265VLevel6: eAVEncH265VLevel = 180;
pub const eAVEncH265VLevel6_1: eAVEncH265VLevel = 183;
pub const eAVEncH265VLevel6_2: eAVEncH265VLevel = 186;
pub type eAVEncH265VProfile = i32;
pub const eAVEncH265VProfile_MainIntra_420_10: eAVEncH265VProfile = 12;
pub const eAVEncH265VProfile_MainIntra_420_12: eAVEncH265VProfile = 13;
pub const eAVEncH265VProfile_MainIntra_420_8: eAVEncH265VProfile = 11;
pub const eAVEncH265VProfile_MainIntra_422_10: eAVEncH265VProfile = 14;
pub const eAVEncH265VProfile_MainIntra_422_12: eAVEncH265VProfile = 15;
pub const eAVEncH265VProfile_MainIntra_444_10: eAVEncH265VProfile = 17;
pub const eAVEncH265VProfile_MainIntra_444_12: eAVEncH265VProfile = 18;
pub const eAVEncH265VProfile_MainIntra_444_16: eAVEncH265VProfile = 19;
pub const eAVEncH265VProfile_MainIntra_444_8: eAVEncH265VProfile = 16;
pub const eAVEncH265VProfile_MainStill_420_8: eAVEncH265VProfile = 20;
pub const eAVEncH265VProfile_MainStill_444_16: eAVEncH265VProfile = 22;
pub const eAVEncH265VProfile_MainStill_444_8: eAVEncH265VProfile = 21;
pub const eAVEncH265VProfile_Main_420_10: eAVEncH265VProfile = 2;
pub const eAVEncH265VProfile_Main_420_12: eAVEncH265VProfile = 3;
pub const eAVEncH265VProfile_Main_420_8: eAVEncH265VProfile = 1;
pub const eAVEncH265VProfile_Main_422_10: eAVEncH265VProfile = 4;
pub const eAVEncH265VProfile_Main_422_12: eAVEncH265VProfile = 5;
pub const eAVEncH265VProfile_Main_444_10: eAVEncH265VProfile = 7;
pub const eAVEncH265VProfile_Main_444_12: eAVEncH265VProfile = 8;
pub const eAVEncH265VProfile_Main_444_8: eAVEncH265VProfile = 6;
pub const eAVEncH265VProfile_Monochrome_12: eAVEncH265VProfile = 9;
pub const eAVEncH265VProfile_Monochrome_16: eAVEncH265VProfile = 10;
pub const eAVEncH265VProfile_unknown: eAVEncH265VProfile = 0;
pub type eAVEncInputVideoSystem = i32;
pub const eAVEncInputVideoSystem_Component: eAVEncInputVideoSystem = 6;
pub const eAVEncInputVideoSystem_HDV: eAVEncInputVideoSystem = 5;
pub const eAVEncInputVideoSystem_MAC: eAVEncInputVideoSystem = 4;
pub const eAVEncInputVideoSystem_NTSC: eAVEncInputVideoSystem = 2;
pub const eAVEncInputVideoSystem_PAL: eAVEncInputVideoSystem = 1;
pub const eAVEncInputVideoSystem_SECAM: eAVEncInputVideoSystem = 3;
pub const eAVEncInputVideoSystem_Unspecified: eAVEncInputVideoSystem = 0;
pub type eAVEncMPACodingMode = i32;
pub const eAVEncMPACodingMode_DualChannel: eAVEncMPACodingMode = 2;
pub const eAVEncMPACodingMode_JointStereo: eAVEncMPACodingMode = 3;
pub const eAVEncMPACodingMode_Mono: eAVEncMPACodingMode = 0;
pub const eAVEncMPACodingMode_Stereo: eAVEncMPACodingMode = 1;
pub const eAVEncMPACodingMode_Surround: eAVEncMPACodingMode = 4;
pub type eAVEncMPAEmphasisType = i32;
pub const eAVEncMPAEmphasisType_50_15: eAVEncMPAEmphasisType = 1;
pub const eAVEncMPAEmphasisType_CCITT_J17: eAVEncMPAEmphasisType = 3;
pub const eAVEncMPAEmphasisType_None: eAVEncMPAEmphasisType = 0;
pub const eAVEncMPAEmphasisType_Reserved: eAVEncMPAEmphasisType = 2;
pub type eAVEncMPALayer = i32;
pub const eAVEncMPALayer_1: eAVEncMPALayer = 1;
pub const eAVEncMPALayer_2: eAVEncMPALayer = 2;
pub const eAVEncMPALayer_3: eAVEncMPALayer = 3;
pub type eAVEncMPVFrameFieldMode = i32;
pub const eAVEncMPVFrameFieldMode_FieldMode: eAVEncMPVFrameFieldMode = 0;
pub const eAVEncMPVFrameFieldMode_FrameMode: eAVEncMPVFrameFieldMode = 1;
pub type eAVEncMPVIntraVLCTable = i32;
pub const eAVEncMPVIntraVLCTable_Alternate: eAVEncMPVIntraVLCTable = 2;
pub const eAVEncMPVIntraVLCTable_Auto: eAVEncMPVIntraVLCTable = 0;
pub const eAVEncMPVIntraVLCTable_MPEG1: eAVEncMPVIntraVLCTable = 1;
pub type eAVEncMPVLevel = i32;
pub const eAVEncMPVLevel_High: eAVEncMPVLevel = 4;
pub const eAVEncMPVLevel_High1440: eAVEncMPVLevel = 3;
pub const eAVEncMPVLevel_Low: eAVEncMPVLevel = 1;
pub const eAVEncMPVLevel_Main: eAVEncMPVLevel = 2;
pub type eAVEncMPVProfile = i32;
pub const eAVEncMPVProfile_422: eAVEncMPVProfile = 4;
pub const eAVEncMPVProfile_High: eAVEncMPVProfile = 3;
pub const eAVEncMPVProfile_Main: eAVEncMPVProfile = 2;
pub const eAVEncMPVProfile_Simple: eAVEncMPVProfile = 1;
pub const eAVEncMPVProfile_unknown: eAVEncMPVProfile = 0;
pub type eAVEncMPVQScaleType = i32;
pub const eAVEncMPVQScaleType_Auto: eAVEncMPVQScaleType = 0;
pub const eAVEncMPVQScaleType_Linear: eAVEncMPVQScaleType = 1;
pub const eAVEncMPVQScaleType_NonLinear: eAVEncMPVQScaleType = 2;
pub type eAVEncMPVScanPattern = i32;
pub const eAVEncMPVScanPattern_AlternateScan: eAVEncMPVScanPattern = 2;
pub const eAVEncMPVScanPattern_Auto: eAVEncMPVScanPattern = 0;
pub const eAVEncMPVScanPattern_ZigZagScan: eAVEncMPVScanPattern = 1;
pub type eAVEncMPVSceneDetection = i32;
pub const eAVEncMPVSceneDetection_InsertIPicture: eAVEncMPVSceneDetection = 1;
pub const eAVEncMPVSceneDetection_None: eAVEncMPVSceneDetection = 0;
pub const eAVEncMPVSceneDetection_StartNewGOP: eAVEncMPVSceneDetection = 2;
pub const eAVEncMPVSceneDetection_StartNewLocatableGOP: eAVEncMPVSceneDetection = 3;
pub type eAVEncMuxOutput = i32;
pub const eAVEncMuxOutputAuto: eAVEncMuxOutput = 0;
pub const eAVEncMuxOutputPS: eAVEncMuxOutput = 1;
pub const eAVEncMuxOutputTS: eAVEncMuxOutput = 2;
pub type eAVEncVP9VProfile = i32;
pub const eAVEncVP9VProfile_420_10: eAVEncVP9VProfile = 2;
pub const eAVEncVP9VProfile_420_12: eAVEncVP9VProfile = 3;
pub const eAVEncVP9VProfile_420_8: eAVEncVP9VProfile = 1;
pub const eAVEncVP9VProfile_unknown: eAVEncVP9VProfile = 0;
pub type eAVEncVideoChromaResolution = i32;
pub const eAVEncVideoChromaResolution_411: eAVEncVideoChromaResolution = 4;
pub const eAVEncVideoChromaResolution_420: eAVEncVideoChromaResolution = 3;
pub const eAVEncVideoChromaResolution_422: eAVEncVideoChromaResolution = 2;
pub const eAVEncVideoChromaResolution_444: eAVEncVideoChromaResolution = 1;
pub const eAVEncVideoChromaResolution_SameAsSource: eAVEncVideoChromaResolution = 0;
pub type eAVEncVideoChromaSubsampling = i32;
pub const eAVEncVideoChromaSubsamplingFormat_Horizontally_Cosited: eAVEncVideoChromaSubsampling = 4;
pub const eAVEncVideoChromaSubsamplingFormat_ProgressiveChroma: eAVEncVideoChromaSubsampling = 8;
pub const eAVEncVideoChromaSubsamplingFormat_SameAsSource: eAVEncVideoChromaSubsampling = 0;
pub const eAVEncVideoChromaSubsamplingFormat_Vertically_AlignedChromaPlanes: eAVEncVideoChromaSubsampling = 1;
pub const eAVEncVideoChromaSubsamplingFormat_Vertically_Cosited: eAVEncVideoChromaSubsampling = 2;
pub type eAVEncVideoColorLighting = i32;
pub const eAVEncVideoColorLighting_Bright: eAVEncVideoColorLighting = 2;
pub const eAVEncVideoColorLighting_Dark: eAVEncVideoColorLighting = 5;
pub const eAVEncVideoColorLighting_Dim: eAVEncVideoColorLighting = 4;
pub const eAVEncVideoColorLighting_Office: eAVEncVideoColorLighting = 3;
pub const eAVEncVideoColorLighting_SameAsSource: eAVEncVideoColorLighting = 0;
pub const eAVEncVideoColorLighting_Unknown: eAVEncVideoColorLighting = 1;
pub type eAVEncVideoColorNominalRange = i32;
pub const eAVEncVideoColorNominalRange_0_255: eAVEncVideoColorNominalRange = 1;
pub const eAVEncVideoColorNominalRange_16_235: eAVEncVideoColorNominalRange = 2;
pub const eAVEncVideoColorNominalRange_48_208: eAVEncVideoColorNominalRange = 3;
pub const eAVEncVideoColorNominalRange_SameAsSource: eAVEncVideoColorNominalRange = 0;
pub type eAVEncVideoColorPrimaries = i32;
pub const eAVEncVideoColorPrimaries_BT470_2_SysBG: eAVEncVideoColorPrimaries = 4;
pub const eAVEncVideoColorPrimaries_BT470_2_SysM: eAVEncVideoColorPrimaries = 3;
pub const eAVEncVideoColorPrimaries_BT709: eAVEncVideoColorPrimaries = 2;
pub const eAVEncVideoColorPrimaries_EBU3231: eAVEncVideoColorPrimaries = 7;
pub const eAVEncVideoColorPrimaries_Reserved: eAVEncVideoColorPrimaries = 1;
pub const eAVEncVideoColorPrimaries_SMPTE170M: eAVEncVideoColorPrimaries = 5;
pub const eAVEncVideoColorPrimaries_SMPTE240M: eAVEncVideoColorPrimaries = 6;
pub const eAVEncVideoColorPrimaries_SMPTE_C: eAVEncVideoColorPrimaries = 8;
pub const eAVEncVideoColorPrimaries_SameAsSource: eAVEncVideoColorPrimaries = 0;
pub type eAVEncVideoColorTransferFunction = i32;
pub const eAVEncVideoColorTransferFunction_10: eAVEncVideoColorTransferFunction = 1;
pub const eAVEncVideoColorTransferFunction_18: eAVEncVideoColorTransferFunction = 2;
pub const eAVEncVideoColorTransferFunction_20: eAVEncVideoColorTransferFunction = 3;
pub const eAVEncVideoColorTransferFunction_22: eAVEncVideoColorTransferFunction = 4;
pub const eAVEncVideoColorTransferFunction_22_240M: eAVEncVideoColorTransferFunction = 6;
pub const eAVEncVideoColorTransferFunction_22_709: eAVEncVideoColorTransferFunction = 5;
pub const eAVEncVideoColorTransferFunction_22_8bit_sRGB: eAVEncVideoColorTransferFunction = 7;
pub const eAVEncVideoColorTransferFunction_28: eAVEncVideoColorTransferFunction = 8;
pub const eAVEncVideoColorTransferFunction_SameAsSource: eAVEncVideoColorTransferFunction = 0;
pub type eAVEncVideoColorTransferMatrix = i32;
pub const eAVEncVideoColorTransferMatrix_BT601: eAVEncVideoColorTransferMatrix = 2;
pub const eAVEncVideoColorTransferMatrix_BT709: eAVEncVideoColorTransferMatrix = 1;
pub const eAVEncVideoColorTransferMatrix_SMPTE240M: eAVEncVideoColorTransferMatrix = 3;
pub const eAVEncVideoColorTransferMatrix_SameAsSource: eAVEncVideoColorTransferMatrix = 0;
pub type eAVEncVideoContentType = i32;
pub const eAVEncVideoContentType_FixedCameraAngle: eAVEncVideoContentType = 1;
pub const eAVEncVideoContentType_Unknown: eAVEncVideoContentType = 0;
pub type eAVEncVideoD3D12ReconstructedPictureOutputMode = i32;
pub const eAVEncVideoEncodeD3D12ReconstructedPictureMode_Copy: eAVEncVideoD3D12ReconstructedPictureOutputMode = 1;
pub const eAVEncVideoEncodeD3D12ReconstructedPictureMode_None: eAVEncVideoD3D12ReconstructedPictureOutputMode = 0;
pub const eAVEncVideoEncodeD3D12ReconstructedPictureMode_Shared: eAVEncVideoD3D12ReconstructedPictureOutputMode = 2;
pub type eAVEncVideoFilmContent = i32;
pub const eAVEncVideoFilmContent_FilmOnly: eAVEncVideoFilmContent = 1;
pub const eAVEncVideoFilmContent_Mixed: eAVEncVideoFilmContent = 2;
pub const eAVEncVideoFilmContent_VideoOnly: eAVEncVideoFilmContent = 0;
pub type eAVEncVideoOutputFrameRateConversion = i32;
pub const eAVEncVideoOutputFrameRateConversion_Alias: eAVEncVideoOutputFrameRateConversion = 2;
pub const eAVEncVideoOutputFrameRateConversion_Disable: eAVEncVideoOutputFrameRateConversion = 0;
pub const eAVEncVideoOutputFrameRateConversion_Enable: eAVEncVideoOutputFrameRateConversion = 1;
pub type eAVEncVideoOutputScanType = i32;
pub const eAVEncVideoOutputScan_Automatic: eAVEncVideoOutputScanType = 3;
pub const eAVEncVideoOutputScan_Interlaced: eAVEncVideoOutputScanType = 1;
pub const eAVEncVideoOutputScan_Progressive: eAVEncVideoOutputScanType = 0;
pub const eAVEncVideoOutputScan_SameAsInput: eAVEncVideoOutputScanType = 2;
pub type eAVEncVideoSourceScanType = i32;
pub const eAVEncVideoSourceScan_Automatic: eAVEncVideoSourceScanType = 0;
pub const eAVEncVideoSourceScan_Interlaced: eAVEncVideoSourceScanType = 1;
pub const eAVEncVideoSourceScan_Progressive: eAVEncVideoSourceScanType = 2;
pub type eAVFastDecodeMode = i32;
pub type eAVScenarioInfo = i32;
pub const eAVScenarioInfo_Archive: eAVScenarioInfo = 3;
pub const eAVScenarioInfo_CameraRecord: eAVScenarioInfo = 5;
pub const eAVScenarioInfo_DisplayRemoting: eAVScenarioInfo = 1;
pub const eAVScenarioInfo_DisplayRemotingWithFeatureMap: eAVScenarioInfo = 6;
pub const eAVScenarioInfo_LiveStreaming: eAVScenarioInfo = 4;
pub const eAVScenarioInfo_Unknown: eAVScenarioInfo = 0;
pub const eAVScenarioInfo_VideoConference: eAVScenarioInfo = 2;
pub const eErrorConcealmentOff: eAVDecVideoMPEG2ErrorConcealment = 0;
pub const eErrorConcealmentOn: eAVDecVideoMPEG2ErrorConcealment = 1;
pub const eErrorConcealmentTypeAdvanced: eAVDecVideoH264ErrorConcealment = 2;
pub const eErrorConcealmentTypeBasic: eAVDecVideoH264ErrorConcealment = 1;
pub const eErrorConcealmentTypeDXVASetBlack: eAVDecVideoH264ErrorConcealment = 3;
pub const eErrorConcealmentTypeDrop: eAVDecVideoH264ErrorConcealment = 0;
pub const eVideoDecodeCompliant: eAVFastDecodeMode = 0;
pub const eVideoDecodeDisableLF: eAVFastDecodeMode = 2;
pub const eVideoDecodeFastest: eAVFastDecodeMode = 32;
pub const eVideoDecodeOptimalLF: eAVFastDecodeMode = 1;
pub type eVideoEncoderDisplayContentType = i32;
pub const eVideoEncoderDisplayContent_FullScreenVideo: eVideoEncoderDisplayContentType = 1;
pub const eVideoEncoderDisplayContent_Unknown: eVideoEncoderDisplayContentType = 0;
